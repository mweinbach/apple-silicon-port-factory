//! MetalFX spatial & temporal upscaling and DXR raytracing commands.

use super::opts::{barrier_masks, barrier_tight_stages, legacy_barriers};
use super::state::{Encoder, FullWhy, StageSync, COMPUTE_STAGES, PRE_RASTER, STAGE_BITS};
use super::CommandList;
use crate::metal::binding::MtlArgumentTable;
use crate::metal::encstats as stats;
use crate::metal::passtiming as pt;
use crate::metal::resource::{MtlTexture, Resource};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::Message;
use objc2_metal::*;
use std::sync::Arc;

pub(super) const UTIL_STAGES: MTLStages = PRE_RASTER
    .union(MTLStages::Fragment)
    .union(MTLStages::Dispatch)
    .union(MTLStages::Blit)
    .union(MTLStages::AccelerationStructure);

/// Compute/ML stages executed internally by an opaque `MTL4FX*Scaler` pass.
pub(crate) const MFX_COMPUTE_ML_STAGES: MTLStages = MTLStages::Dispatch
    .union(MTLStages::Blit)
    .union(MTLStages::MachineLearning);

/// Producer stages of an opaque `MTL4FX*Scaler` pass keyed off the hardware
/// output texture usage requirements (never off pass indices).
#[inline]
pub(crate) fn metalfx_producer_stages(output_usage: MTLTextureUsage) -> MTLStages {
    if output_usage.contains(MTLTextureUsage::RenderTarget) {
        MFX_COMPUTE_ML_STAGES | MTLStages::Fragment
    } else {
        MFX_COMPUTE_ML_STAGES
    }
}

/// Consumer stages of an opaque `MTL4FX*Scaler` pass keyed off its output usage
/// (when a scaler renders into an RT output, its internal fragment pass samples
/// the input textures in `Fragment`; otherwise inputs are read in compute/ML/blit).
#[inline]
pub(crate) fn metalfx_consumer_stages(output_usage: MTLTextureUsage) -> MTLStages {
    metalfx_producer_stages(output_usage)
}

/// Pre-MetalFX producer barrier `(after_stages, before_queue_stages)` emitted
/// on the fence-updating encoder before an opaque `MTL4FX*Scaler` pass.
#[inline]
pub(crate) fn metalfx_pre_producer_barrier(
    fence_stage: MTLStages,
    output_usage: MTLTextureUsage,
    has_fence: bool,
    tight_stages: bool,
    legacy: bool,
    masks: bool,
) -> (MTLStages, MTLStages) {
    if tight_stages && !legacy && masks && has_fence {
        (fence_stage, metalfx_consumer_stages(output_usage))
    } else {
        (MTLStages::All, MTLStages::All)
    }
}

/// Post-MetalFX synchronization policy for the encoder(s) following an opaque
/// `MTL4FX*Scaler` pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MetalFxPostSync {
    /// Stage-scoped synchronization: wait on `MTLFence` per consumer stage plus
    /// `after -> hard_before` queue barriers for device cache visibility (and an
    /// optional per-resource soft `PRE_RASTER` obligation on the upscaled output).
    Scoped {
        after: MTLStages,
        hard_before: MTLStages,
        soft_pre_raster: bool,
    },
    /// Conservative full barrier (`FullWhy::ClearUav` -> `All -> All`) when
    /// legacy barriers are enabled, masks are disabled, or the scaler does not
    /// expose `setFence:`.
    FullFallback,
}

/// Compute the post-MetalFX sync policy from scaler capabilities and barrier mode.
#[inline]
pub(crate) fn metalfx_post_sync_policy(
    output_usage: MTLTextureUsage,
    has_fence: bool,
    has_scratch_blit: bool,
    tight_stages: bool,
    legacy: bool,
    masks: bool,
) -> MetalFxPostSync {
    if !tight_stages || legacy || !masks || !has_fence {
        return MetalFxPostSync::FullFallback;
    }
    let after = metalfx_producer_stages(output_usage);
    if has_scratch_blit {
        MetalFxPostSync::Scoped {
            after,
            hard_before: MTLStages::Blit,
            soft_pre_raster: false,
        }
    } else {
        MetalFxPostSync::Scoped {
            after,
            hard_before: MTLStages::Fragment | COMPUTE_STAGES,
            soft_pre_raster: true,
        }
    }
}

/// Compute which encoder stages should wait on `pending_mfx_wait_fence` when
/// opening a render or compute encoder.
#[inline]
pub(crate) fn metalfx_enc_fence_wait_stages(
    pending_stages: MTLStages,
    render: bool,
    full: bool,
    opening_pre_raster_reads: u8,
    split_compute_exact: bool,
    opening_compute_stage: MTLStages,
) -> MTLStages {
    if render {
        let want = if full || opening_pre_raster_reads != 0 {
            PRE_RASTER | MTLStages::Fragment
        } else {
            MTLStages::Fragment
        };
        if full {
            want
        } else {
            pending_stages & want
        }
    } else {
        let want = if !full
            && split_compute_exact
            && !(opening_compute_stage & COMPUTE_STAGES).is_empty()
        {
            opening_compute_stage & COMPUTE_STAGES
        } else {
            COMPUTE_STAGES
        };
        if full {
            want
        } else {
            pending_stages & want
        }
    }
}

impl CommandList {
    /// 2D temporal upscale using Apple MetalFX (`MTL4FXTemporalScaler`) when
    /// available and compatible, falling back gracefully to spatial MetalFX
    /// (`MTL4FXSpatialScaler`) and then the bilinear compute shader
    /// (`port_upscale_2d_f`).
    pub fn upscale_temporal(
        &mut self,
        params: &crate::metal::metalfx::TemporalUpscaleParams<'_>,
    ) -> Result<(), String> {
        self.upscale_temporal_with_backend(params).map(|_| ())
    }

    /// Like [`Self::upscale_temporal`], returning which upscale backend ran.
    pub fn upscale_temporal_with_backend(
        &mut self,
        params: &crate::metal::metalfx::TemporalUpscaleParams<'_>,
    ) -> Result<crate::metal::metalfx::UpscaleBackend, String> {
        self.upscale_temporal_with(params, barrier_tight_stages())
    }

    pub(super) fn upscale_temporal_with(
        &mut self,
        params: &crate::metal::metalfx::TemporalUpscaleParams<'_>,
        tight_stages: bool,
    ) -> Result<crate::metal::metalfx::UpscaleBackend, String> {
        use crate::metal::clear::{self, TexParams};
        use crate::metal::metalfx::UpscaleBackend;

        self.flush_icb_prepass_if_pending();
        let mut touch = Vec::with_capacity(6);
        for r in [params.output, params.color, params.depth, params.motion] {
            r.register_ptr();
            r.mark_gpu_used();
            touch.push(r as *const Resource as usize);
        }
        if let Some(exp) = params.exposure {
            exp.register_ptr();
            exp.mark_gpu_used();
            touch.push(exp as *const Resource as usize);
        }
        if let Some(rm) = params.reactive_mask {
            rm.register_ptr();
            rm.mark_gpu_used();
            touch.push(rm as *const Resource as usize);
        }
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;

        let st = params
            .color
            .texture()
            .ok_or("upscale_temporal: color has no texture")?;
        let dt = params
            .output
            .texture()
            .ok_or("upscale_temporal: output has no texture")?;
        let s_fmt = crate::format::view_format(params.color.desc.Format, params.color.mtl_format);
        let d_fmt = crate::format::view_format(params.output.desc.Format, params.output.mtl_format);
        let sv = params
            .color
            .texture_view(s_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("upscale_temporal: color view")?;
        let dv = params
            .output
            .texture_view(d_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("upscale_temporal: output view")?;
        let st_w = st.width() as u32;
        let st_h = st.height() as u32;
        let dt_w = dt.width() as u32;
        let dt_h = dt.height() as u32;
        if params.color_origin[0] >= st_w
            || params.color_origin[1] >= st_h
            || params.output_origin[0] >= dt_w
            || params.output_origin[1] >= dt_h
        {
            return Err(format!(
                "upscale_temporal: subrect origin out of bounds (color_origin={:?} in {}x{}, output_origin={:?} in {}x{})",
                params.color_origin, st_w, st_h, params.output_origin, dt_w, dt_h
            ));
        }
        let avail_src_w = st_w - params.color_origin[0];
        let avail_src_h = st_h - params.color_origin[1];
        let avail_dst_w = dt_w - params.output_origin[0];
        let avail_dst_h = dt_h - params.output_origin[1];
        let x0 = if params.render_size[0] > 0 {
            params.render_size[0].min(avail_src_w)
        } else {
            avail_src_w
        };
        let y0 = if params.render_size[1] > 0 {
            params.render_size[1].min(avail_src_h)
        } else {
            avail_src_h
        };
        let w = if params.upscale_size[0] > 0 {
            params.upscale_size[0].min(avail_dst_w)
        } else {
            avail_dst_w
        };
        let h = if params.upscale_size[1] > 0 {
            params.upscale_size[1].min(avail_dst_h)
        } else {
            avail_dst_h
        };
        if w == 0 || h == 0 {
            return Ok(UpscaleBackend::BilinearCompute);
        }

        if crate::metal::gpunan::is_capturing() && self.open {
            self.flush_pending_mfx_wait_fence();
            self.end_encoder();
            let _ = crate::metal::gpunan::encode_scan_resource(
                &self.cb,
                params.color,
                "upscale_in_color",
                "upscale",
                0,
                0,
                0,
                0,
            );
            let _ = crate::metal::gpunan::encode_scan_resource(
                &self.cb,
                params.motion,
                "upscale_in_mv",
                "upscale",
                0,
                0,
                0,
                0,
            );
            if let Some(exp) = params.exposure {
                let _ = crate::metal::gpunan::encode_scan_resource(
                    &self.cb,
                    exp,
                    "upscale_in_exp",
                    "upscale",
                    0,
                    0,
                    0,
                    0,
                );
            }
            if let Some(rm) = params.reactive_mask {
                let _ = crate::metal::gpunan::encode_scan_resource(
                    &self.cb,
                    rm,
                    "upscale_in_react",
                    "upscale",
                    0,
                    0,
                    0,
                    0,
                );
            }
            self.need_full(FullWhy::ClearUav);
        }
        if self.try_upscale_temporal_metalfx(
            params,
            &sv,
            &dv,
            x0,
            y0,
            w,
            h,
            s_fmt,
            d_fmt,
            &touch,
            tight_stages,
        )? {
            if crate::metal::gpunan::is_capturing() && self.open {
                self.flush_pending_mfx_wait_fence();
                self.end_encoder();
                let _ = crate::metal::gpunan::encode_scan_resource(
                    &self.cb,
                    params.output,
                    "upscale_out_tmfx",
                    "upscale",
                    0,
                    0,
                    0,
                    0,
                );
                self.need_full(FullWhy::ClearUav);
            }
            return Ok(UpscaleBackend::TemporalMetalFx);
        }

        if crate::metal::metalfx::is_temporal_enabled() {
            log::warn!(
                target: "d3d",
                "metalfx: temporal upscale unavailable for {}x{} (color={:?}, depth={:?}, motion={:?}) -> {}x{} ({:?}); falling back to spatial/bilinear",
                x0,
                y0,
                s_fmt,
                params.depth.mtl_format,
                params.motion.mtl_format,
                w,
                h,
                d_fmt,
            );
        }

        let fallback_touch = [touch[0], touch[1]];
        let src_origin = params.color_origin;
        let dst_origin = params.output_origin;
        if x0 == w && y0 == h {
            if touch[0] == touch[1] && s_fmt == d_fmt && src_origin == dst_origin {
                return Ok(UpscaleBackend::BilinearCompute);
            }
            if s_fmt == d_fmt {
                self.copy_upscale_1to1(
                    &sv,
                    &dv,
                    src_origin,
                    dst_origin,
                    w,
                    h,
                    &fallback_touch,
                    tight_stages,
                )?;
                return Ok(UpscaleBackend::BilinearCompute);
            }
            let p = TexParams {
                u: [src_origin[0], src_origin[1], dst_origin[0], dst_origin[1]],
                f: [0.0; 4],
                x0,
                y0,
                z0: 0,
                w,
                h,
                d: 1,
                _pad: [0; 2],
            };
            self.util_dispatch_with(
                "port_upscale_2d_f",
                Some(&sv),
                Some(&dv),
                clear::bytes_of(&p),
                [w, h, 1],
                &fallback_touch,
                tight_stages,
            )?;
            return Ok(UpscaleBackend::BilinearCompute);
        }
        if self.try_upscale_metalfx(
            &sv,
            &dv,
            src_origin,
            dst_origin,
            x0,
            y0,
            w,
            h,
            s_fmt,
            d_fmt,
            &fallback_touch,
            tight_stages,
        )? {
            return Ok(UpscaleBackend::SpatialMetalFx);
        }
        let p = TexParams {
            u: [src_origin[0], src_origin[1], dst_origin[0], dst_origin[1]],
            f: [0.0; 4],
            x0,
            y0,
            z0: 0,
            w,
            h,
            d: 1,
            _pad: [0; 2],
        };
        self.util_dispatch_with(
            "port_upscale_2d_f",
            Some(&sv),
            Some(&dv),
            clear::bytes_of(&p),
            [w, h, 1],
            &fallback_touch,
            tight_stages,
        )?;
        Ok(UpscaleBackend::BilinearCompute)
    }

    /// Blit a `[origin.x .. origin.x + w, origin.y .. origin.y + h]` subrect
    /// from `src` into a dedicated MetalFX staging texture (`slot`) at `(0, 0)`.
    #[allow(clippy::too_many_arguments)]
    fn blit_subrect_to_scratch(
        &mut self,
        src: &MtlTexture,
        origin: [u32; 2],
        w: u32,
        h: u32,
        slot: u8,
        fmt: MTLPixelFormat,
        usage: MTLTextureUsage,
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<Option<Retained<MtlTexture>>, String> {
        let sw = src.width() as u32;
        let sh = src.height() as u32;
        if w == 0 || h == 0 || origin[0].saturating_add(w) > sw || origin[1].saturating_add(h) > sh
        {
            return Ok(None);
        }
        let Some(dst) =
            crate::metal::metalfx::scratch_texture_slot(slot, w, h, fmt, usage)
        else {
            return Ok(None);
        };
        let masked = tight_stages && !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        if masked {
            self.add_pend(UTIL_STAGES, MTLStages::Blit);
            if ib_tight {
                for &r in touch {
                    self.ib_note_touch_before_tight(r, MTLStages::Blit);
                }
            } else {
                for &r in touch {
                    self.ib_note(r);
                }
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        let e = self
            .compute_blit_touch(touch)
            .ok_or("metalfx subrect input blit encoder")?;
        unsafe {
            e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                src,
                0,
                0,
                MTLOrigin { x: origin[0] as usize, y: origin[1] as usize, z: 0 },
                MTLSize { width: w as usize, height: h as usize, depth: 1 },
                &dst,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
            );
        }
        self.note_stage_ran(MTLStages::Blit);
        if masked {
            self.add_pend(MTLStages::Blit, UTIL_STAGES);
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        Ok(Some(dst))
    }

    /// Shared post-encode scratch-blit and barrier epilogue for both temporal
    /// and spatial MetalFX passes.
    #[allow(clippy::too_many_arguments)]
    fn finish_metalfx_output_blit(
        &mut self,
        scratch: Option<Retained<MtlTexture>>,
        dv: &MtlTexture,
        dst_origin: [u32; 2],
        w: u32,
        h: u32,
        touch: &[usize],
        tight_stages: bool,
        enc_err: &'static str,
    ) -> Result<(), String> {
        let masked = tight_stages && !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        if let Some(scratch_tex) = scratch {
            let e = self.compute_blit_touch(touch).ok_or(enc_err)?;
            self.pt_note_util(pt::synth::UPSCALE, 1);
            unsafe {
                e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                    &scratch_tex,
                    0,
                    0,
                    MTLOrigin { x: 0, y: 0, z: 0 },
                    MTLSize { width: w as usize, height: h as usize, depth: 1 },
                    dv,
                    0,
                    0,
                    MTLOrigin { x: dst_origin[0] as usize, y: dst_origin[1] as usize, z: 0 },
                );
            }
            self.note_stage_ran(MTLStages::Blit);
            if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
                self.ib.ib_blit_dst.push(touch[0]);
            }
            if masked {
                self.add_pend(MTLStages::Blit, UTIL_STAGES);
                if ib_tight {
                    for &r in touch {
                        self.ib_add_obl_tight(r, MTLStages::Blit, COMPUTE_STAGES);
                    }
                } else {
                    for &r in touch {
                        self.ib_note(r);
                    }
                }
                stats::inc(&stats::c::qb_upscale_masked);
            } else {
                self.need_full(FullWhy::ClearUav);
            }
        } else {
            if touch[0] != 0 && !self.bar.written.contains(&touch[0]) {
                self.bar.written.push(touch[0]);
            }
            // Post-pass synchronization (stage-scoped fence + queue barrier, or
            // conservative `FullWhy::ClearUav` fallback) was armed by
            // `finish_opaque_metalfx_pass`.
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn try_upscale_temporal_metalfx(
        &mut self,
        params: &crate::metal::metalfx::TemporalUpscaleParams<'_>,
        sv: &MtlTexture,
        dv: &MtlTexture,
        x0: u32,
        y0: u32,
        w: u32,
        h: u32,
        s_fmt: MTLPixelFormat,
        d_fmt: MTLPixelFormat,
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<bool, String> {
        if !self.open || !crate::metal::metalfx::is_temporal_enabled() {
            return Ok(false);
        }
        if x0 == 0
            || y0 == 0
            || x0 > w
            || y0 > h
            || sv.textureType() != MTLTextureType::Type2D
            || dv.textureType() != MTLTextureType::Type2D
        {
            return Ok(false);
        }
        let Some(_dt_tex) = params.depth.texture() else {
            return Ok(false);
        };
        let Some(_mt_tex) = params.motion.texture() else {
            return Ok(false);
        };
        let depth_fmt = crate::format::view_format(params.depth.desc.Format, params.depth.mtl_format);
        let motion_fmt =
            crate::format::view_format(params.motion.desc.Format, params.motion.mtl_format);
        let Some(depth_v) = params
            .depth
            .texture_view(depth_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
        else {
            return Ok(false);
        };
        let Some(motion_v) = params
            .motion
            .texture_view(motion_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
        else {
            return Ok(false);
        };
        let mut exp_v = if !params.auto_exposure {
            if let Some(exp) = params.exposure {
                let ef = crate::format::view_format(exp.desc.Format, exp.mtl_format);
                exp.texture_view(ef, MTLTextureType::Type2D, (0, 1), (0, 1))
            } else {
                None
            }
        } else {
            None
        };
        let mut reactive_v_and_fmt = if let Some(rm) = params.reactive_mask {
            let rf = crate::format::view_format(rm.desc.Format, rm.mtl_format);
            if crate::metal::metalfx::temporal_reactive_format_supported(rf) {
                rm.texture_view(rf, MTLTextureType::Type2D, (0, 1), (0, 1))
                    .map(|v| (v, rf))
            } else {
                log::warn!(
                    target: "d3d",
                    "metalfx: ignoring unsupported temporal reactive mask format {:?}",
                    rf,
                );
                None
            }
        } else {
            None
        };

        let has_input_subrect = params.color_origin != [0, 0]
            || params.depth_origin != [0, 0]
            || params.motion_origin != [0, 0]
            || (reactive_v_and_fmt.is_some() && params.reactive_origin != [0, 0]);

        let (in_w, in_h) = if has_input_subrect {
            (x0, y0)
        } else {
            (sv.width() as u32, sv.height() as u32)
        };
        let exp_mv_w = if params.output_resolution_motion_vectors { w } else { in_w };
        let exp_mv_h = if params.output_resolution_motion_vectors { h } else { in_h };
        if has_input_subrect {
            if params.color_origin[0].saturating_add(in_w) > sv.width() as u32
                || params.color_origin[1].saturating_add(in_h) > sv.height() as u32
                || params.depth_origin[0].saturating_add(in_w) > depth_v.width() as u32
                || params.depth_origin[1].saturating_add(in_h) > depth_v.height() as u32
                || params.motion_origin[0].saturating_add(exp_mv_w) > motion_v.width() as u32
                || params.motion_origin[1].saturating_add(exp_mv_h) > motion_v.height() as u32
            {
                return Ok(false);
            }
            if let Some((rv, _)) = &reactive_v_and_fmt {
                if params.reactive_origin[0].saturating_add(in_w) > rv.width() as u32
                    || params.reactive_origin[1].saturating_add(in_h) > rv.height() as u32
                {
                    reactive_v_and_fmt = None;
                }
            }
        } else {
            if depth_v.width() as u32 != in_w
                || depth_v.height() as u32 != in_h
                || motion_v.width() as u32 != exp_mv_w
                || motion_v.height() as u32 != exp_mv_h
            {
                log::warn!(
                    target: "d3d",
                    "metalfx: temporal input size mismatch (color={}x{}, depth={}x{}, motion={}x{}, out_res_mv={})",
                    in_w,
                    in_h,
                    depth_v.width(),
                    depth_v.height(),
                    motion_v.width(),
                    motion_v.height(),
                    params.output_resolution_motion_vectors,
                );
                return Ok(false);
            }
            if let Some((rv, _)) = &reactive_v_and_fmt {
                if rv.width() as u32 != in_w || rv.height() as u32 != in_h {
                    log::warn!(
                        target: "d3d",
                        "metalfx: ignoring temporal reactive mask size mismatch (color={}x{}, reactive={}x{})",
                        in_w,
                        in_h,
                        rv.width(),
                        rv.height(),
                    );
                    reactive_v_and_fmt = None;
                }
            }
        }
        let (reactive_v, reactive_fmt) = match reactive_v_and_fmt {
            Some((v, rf)) => (Some(v), Some(rf)),
            None => (None, None),
        };
        let mut exp_convert_src: Option<(Retained<MtlTexture>, usize)> = None;
        if let Some(ev) = &exp_v {
            if ev.pixelFormat() != MTLPixelFormat::R16Float {
                if matches!(
                    ev.pixelFormat(),
                    MTLPixelFormat::R32Float
                        | MTLPixelFormat::RG16Float
                        | MTLPixelFormat::RG32Float
                        | MTLPixelFormat::RGBA16Float
                        | MTLPixelFormat::RGBA32Float
                        | MTLPixelFormat::RG11B10Float
                ) {
                    if let Some(scratch_exp) =
                        crate::metal::metalfx::scratch_output_texture_with_usage(
                            1,
                            1,
                            MTLPixelFormat::R16Float,
                            MTLTextureUsage::ShaderWrite,
                        )
                    {
                        let exp_ptr = params.exposure.map_or(0, |r| r as *const Resource as usize);
                        exp_convert_src = Some((ev.clone(), exp_ptr));
                        exp_v = Some(scratch_exp);
                    } else {
                        log::warn!(
                            target: "d3d",
                            "metalfx: failed to allocate 1x1 R16Float scratch for exposure {:?}",
                            ev.pixelFormat(),
                        );
                        return Ok(false);
                    }
                } else {
                    log::warn!(
                        target: "d3d",
                        "metalfx: temporal exposure texture must be float (got {:?})",
                        ev.pixelFormat(),
                    );
                    return Ok(false);
                }
            }
        }
        let dynamic_resolution = params.dynamic_resolution || x0 != in_w || y0 != in_h;
        let key = crate::metal::metalfx::TemporalScalerKey::new(
            in_w, in_h, w, h, s_fmt, depth_fmt, motion_fmt, d_fmt,
        )
        .with_reactive_format(reactive_fmt)
        .with_auto_exposure(params.auto_exposure)
        .with_dynamic_resolution(dynamic_resolution)
        .with_output_resolution_motion_vectors(params.output_resolution_motion_vectors)
        .with_jittered_motion_vectors(params.jittered_motion_vectors);

        let Some(scaler_arc) = crate::metal::metalfx::get_or_create_temporal_scaler(key) else {
            return Ok(false);
        };
        let mut scaler = scaler_arc.lock().unwrap();
        let scale_x = w as f32 / x0 as f32;
        let scale_y = h as f32 / y0 as f32;
        if scale_x < scaler.min_scale - 1e-4
            || scale_y < scaler.min_scale - 1e-4
            || scale_x > scaler.max_scale + 1e-4
            || scale_y > scaler.max_scale + 1e-4
        {
            log::warn!(
                target: "d3d",
                "metalfx: temporal scale ({scale_x:.2}x, {scale_y:.2}x) outside [{:.2}, {:.2}]",
                scaler.min_scale,
                scaler.max_scale,
            );
            return Ok(false);
        }
        if !has_input_subrect {
            if !sv.usage().contains(scaler.color_usage)
                || !depth_v.usage().contains(scaler.depth_usage)
                || !motion_v.usage().contains(scaler.motion_usage)
            {
                log::warn!(
                    target: "d3d",
                    "metalfx: temporal input usage mismatch (color={:?}/{:?}, depth={:?}/{:?}, motion={:?}/{:?})",
                    sv.usage(),
                    scaler.color_usage,
                    depth_v.usage(),
                    scaler.depth_usage,
                    motion_v.usage(),
                    scaler.motion_usage,
                );
                return Ok(false);
            }
            if let (Some(rv), Some(req)) = (&reactive_v, scaler.reactive_usage) {
                if !rv.usage().contains(req) {
                    log::warn!(
                        target: "d3d",
                        "metalfx: temporal reactive mask usage mismatch ({:?}/{:?})",
                        rv.usage(),
                        req,
                    );
                    return Ok(false);
                }
            }
        }
        let color_usage = scaler.color_usage;
        let depth_usage = scaler.depth_usage;
        let motion_usage = scaler.motion_usage;
        let reactive_usage = scaler.reactive_usage.unwrap_or(MTLTextureUsage::ShaderRead);
        let use_native_offsets =
            crate::metal::m41::metalfx_offsets_enabled() && scaler.has_content_offsets();
        let inputs_usage_ok = sv.usage().contains(color_usage)
            && depth_v.usage().contains(depth_usage)
            && motion_v.usage().contains(motion_usage)
            && reactive_v
                .as_ref()
                .zip(scaler.reactive_usage)
                .is_none_or(|(rv, req)| rv.usage().contains(req));
        let use_native_input_offsets = has_input_subrect && use_native_offsets && inputs_usage_ok;
        let direct_ok = dv.storageMode() == MTLStorageMode::Private
            && dv.usage().contains(scaler.output_usage)
            && ((params.output_origin == [0, 0] && w == dv.width() as u32 && h == dv.height() as u32)
                || (use_native_offsets
                    && params.output_origin[0].saturating_add(w) <= dv.width() as u32
                    && params.output_origin[1].saturating_add(h) <= dv.height() as u32));
        let scratch = if direct_ok {
            None
        } else {
            let Some(st) = crate::metal::metalfx::scratch_output_texture_with_usage(
                w,
                h,
                d_fmt,
                scaler.output_usage,
            ) else {
                return Ok(false);
            };
            Some(st)
        };

        self.flush_clear_tex_cs_touch(touch);
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(false);
        }
        let (staged_color, staged_depth, staged_motion, staged_reactive) = if has_input_subrect
            && !use_native_input_offsets
        {
            let Some(sc) = self.blit_subrect_to_scratch(
                sv,
                params.color_origin,
                in_w,
                in_h,
                1,
                s_fmt,
                color_usage,
                touch,
                tight_stages,
            )?
            else {
                return Ok(false);
            };
            let Some(sd) = self.blit_subrect_to_scratch(
                &depth_v,
                params.depth_origin,
                in_w,
                in_h,
                2,
                depth_fmt,
                depth_usage,
                touch,
                tight_stages,
            )?
            else {
                return Ok(false);
            };
            let Some(sm) = self.blit_subrect_to_scratch(
                &motion_v,
                params.motion_origin,
                exp_mv_w,
                exp_mv_h,
                3,
                motion_fmt,
                motion_usage,
                touch,
                tight_stages,
            )?
            else {
                return Ok(false);
            };
            let sr = if let (Some(rv), Some(rf)) = (&reactive_v, reactive_fmt) {
                self.blit_subrect_to_scratch(
                    rv,
                    params.reactive_origin,
                    in_w,
                    in_h,
                    4,
                    rf,
                    reactive_usage,
                    touch,
                    tight_stages,
                )?
            } else {
                None
            };
            (Some(sc), Some(sd), Some(sm), sr)
        } else {
            (None, None, None, None)
        };
        if let (Some((src_exp, exp_ptr)), Some(dst_exp)) = (&exp_convert_src, &exp_v) {
            let exp_touch = [0usize, *exp_ptr];
            self.util_dispatch_with(
                "port_present_copy_2d",
                Some(src_exp),
                Some(dst_exp),
                &[0u8; 4],
                [1, 1, 1],
                &exp_touch,
                tight_stages,
            )?;
        }
        let output_usage = scaler.output_usage;
        let has_fence = scaler.has_fence();
        let has_scratch_blit = scratch.is_some();
        let (mfx_fence, pre_sync, was_full) = self.sync_queue_before_metalfx(
            output_usage,
            has_fence,
            tight_stages,
            pt::synth::METALFX_TEMPORAL,
            w,
            h,
        );
        let used_fence = params.fence.or(mfx_fence.as_deref());
        let effective_has_fence = has_fence && used_fence.is_some();

        let mfx_dst: &MtlTexture = scratch.as_deref().unwrap_or(dv);
        let inputs = crate::metal::metalfx::TemporalEncodeInputs {
            color: staged_color.as_deref().unwrap_or(sv),
            depth: staged_depth.as_deref().unwrap_or(&depth_v),
            motion: staged_motion.as_deref().unwrap_or(&motion_v),
            output: mfx_dst,
            exposure: exp_v.as_deref(),
            reactive_mask: staged_reactive.as_deref().or(reactive_v.as_deref()),
            content_w: x0,
            content_h: y0,
            color_offset: if use_native_input_offsets { params.color_origin } else { [0, 0] },
            depth_offset: if use_native_input_offsets { params.depth_origin } else { [0, 0] },
            motion_offset: if use_native_input_offsets { params.motion_origin } else { [0, 0] },
            reactive_offset: if use_native_input_offsets { params.reactive_origin } else { [0, 0] },
            output_offset: if direct_ok && use_native_offsets { params.output_origin } else { [0, 0] },
            jitter_offset: params.jitter_offset,
            motion_vector_scale: params.motion_vector_scale,
            reset: params.reset,
            depth_reversed: params.depth_reversed,
            pre_exposure: params.pre_exposure,
            fence: used_fence,
        };
        scaler.encode(&self.cb, &inputs);
        self.has_ml_work = true;
        drop(scaler);
        self.finish_opaque_metalfx_pass(
            used_fence,
            output_usage,
            effective_has_fence,
            has_scratch_blit,
            touch[0],
            pre_sync,
            was_full,
            tight_stages,
        );

        self.finish_metalfx_output_blit(
            scratch,
            dv,
            params.output_origin,
            w,
            h,
            touch,
            tight_stages,
            "metalfx temporal scratch blit encoder",
        )?;
        Ok(true)
    }

    /// Emit Metal 4 queue consumer barrier + `MTLFence` update + stage-scoped
    /// producer barrier before an opaque `MTL4FX*Scaler` pass.
    fn sync_queue_before_metalfx(
        &mut self,
        output_usage: MTLTextureUsage,
        has_fence: bool,
        tight_stages: bool,
        pt_shader: u64,
        pt_w: u32,
        pt_h: u32,
    ) -> (Option<Retained<ProtocolObject<dyn MTLFence>>>, StageSync, bool) {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(false);
        }
        // Acquire a fence from the bound `CommandAllocator`'s pool so two in-flight
        // recordings reusing the same `CommandList` on different allocators never
        // alias a live `MTLFence` in `updateFence` / `scaler.setFence` / `waitForFence`.
        let cur_fence = self
            .mfx_fences
            .as_ref()
            .and_then(|pool| pool.lock().unwrap().acquire())
            .or_else(|| {
                if self.mfx_fence.is_none() {
                    self.mfx_fence = crate::metal::device().and_then(|d| d.dev.newFence());
                }
                self.mfx_fence.clone()
            });
        let legacy = legacy_barriers();
        let masks = barrier_masks();
        let masked = tight_stages && !legacy && masks;
        let effective_has_fence = has_fence && cur_fence.is_some();
        let was_full = self.bar.needs_barrier;
        let need_all_before =
            !masked || was_full || self.bar.list_unwaited_cs || self.bar.list_unwaited_gfx;
        let ml_sync: u8 = if need_all_before { 2 } else { 1 };
        let pending_wait = self.pending_mfx_wait_fence.take();
        self.pending_mfx_wait_stages = MTLStages::empty();

        match &self.enc {
            Encoder::Compute(e) => {
                let e = e.clone();
                if let Some(wf) = pending_wait {
                    e.waitForFence_beforeEncoderStages(&wf, MTLStages::Dispatch);
                }
                let pre_after = if need_all_before {
                    MTLStages::All
                } else {
                    self.unsatisfied_enc_after_for(UTIL_STAGES, MTLStages::Dispatch, true)
                };
                if !pre_after.is_empty() {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        pre_after,
                        MTLStages::Dispatch,
                        MTL4VisibilityOptions::Device,
                    );
                    self.record_enc_queue_sync(pre_after, MTLStages::Dispatch);
                }
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    COMPUTE_STAGES,
                    MTLStages::Dispatch,
                    MTL4VisibilityOptions::Device,
                );
                if let Some(f) = &cur_fence {
                    e.updateFence_afterEncoderStages(f, MTLStages::Dispatch);
                }
                self.note_stage_ran(MTLStages::Dispatch);
                let (prod_after, prod_before) = metalfx_pre_producer_barrier(
                    MTLStages::Dispatch,
                    output_usage,
                    effective_has_fence,
                    tight_stages,
                    legacy,
                    masks,
                );
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    prod_after,
                    prod_before,
                    MTL4VisibilityOptions::Device,
                );
                if let Some(i) = self.pt.as_mut().and_then(|p| p.close()) {
                    self.pt_stamp(i, true);
                }
                if let Some(i) = self
                    .pt
                    .as_mut()
                    .and_then(|p| p.open_ml(pt_shader, pt_w, pt_h, ml_sync))
                {
                    self.pt_stamp(i, true);
                }
                self.end_encoder();
            }
            Encoder::Render(e) => {
                let e = e.clone();
                if let Some(wf) = pending_wait {
                    e.waitForFence_beforeEncoderStages(&wf, MTLStages::Fragment);
                }
                let pre_after = if need_all_before {
                    MTLStages::All
                } else {
                    self.unsatisfied_enc_after_for(UTIL_STAGES, MTLStages::Fragment, true)
                };
                if !pre_after.is_empty() {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        pre_after,
                        MTLStages::Fragment,
                        MTL4VisibilityOptions::Device,
                    );
                    self.record_enc_queue_sync(pre_after, MTLStages::Fragment);
                }
                if self.gfx.pass_uav {
                    e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                        PRE_RASTER,
                        MTLStages::Fragment,
                        MTL4VisibilityOptions::Device,
                    );
                }
                if let Some(f) = &cur_fence {
                    e.updateFence_afterEncoderStages(f, MTLStages::Fragment);
                }
                self.note_stage_ran(MTLStages::Fragment);
                let (prod_after, prod_before) = metalfx_pre_producer_barrier(
                    MTLStages::Fragment,
                    output_usage,
                    effective_has_fence,
                    tight_stages,
                    legacy,
                    masks,
                );
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    prod_after,
                    prod_before,
                    MTL4VisibilityOptions::Device,
                );
                if let Some(i) = self.pt.as_mut().and_then(|p| p.close()) {
                    self.pt_stamp(i, true);
                }
                if let Some(i) = self
                    .pt
                    .as_mut()
                    .and_then(|p| p.open_ml(pt_shader, pt_w, pt_h, ml_sync))
                {
                    self.pt_stamp(i, true);
                }
                self.end_encoder();
            }
            Encoder::None => {
                if let Some(e) = self.cb.computeCommandEncoder() {
                    if let Some(wf) = pending_wait {
                        e.waitForFence_beforeEncoderStages(&wf, MTLStages::Dispatch);
                    }
                    let pre_after = if need_all_before {
                        MTLStages::All
                    } else {
                        UTIL_STAGES
                    };
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        pre_after,
                        MTLStages::Dispatch,
                        MTL4VisibilityOptions::Device,
                    );
                    if let Some(f) = &cur_fence {
                        e.updateFence_afterEncoderStages(f, MTLStages::Dispatch);
                    }
                    let (prod_after, prod_before) = metalfx_pre_producer_barrier(
                        MTLStages::Dispatch,
                        output_usage,
                        effective_has_fence,
                        tight_stages,
                        legacy,
                        masks,
                    );
                    e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                        prod_after,
                        prod_before,
                        MTL4VisibilityOptions::Device,
                    );
                    if self.pt.is_none() && self.pt_armed {
                        self.pt = pt::PassLog::acquire();
                        self.pt_armed = false;
                    }
                    if let Some(p) = self.pt.as_mut() {
                        if p.has_pending_ml() {
                            // Two consecutive direct-output MetalFX scalers separated only by
                            // a 0-dispatch compute encoder have no valid AGX boundary timestamp
                            // between them; discard the pending pass (which also suppresses
                            // starting the chained pass so neither is misattributed).
                            p.discard_ml();
                        } else if let Some(i) = p.open_ml(pt_shader, pt_w, pt_h, ml_sync) {
                            unsafe {
                                e.writeTimestampWithGranularity_intoHeap_atIndex(
                                    MTL4TimestampGranularity::Relaxed,
                                    p.heap(),
                                    i,
                                );
                            }
                        }
                    }
                    e.endEncoding();
                } else if let Some(p) = self.pt.as_mut() {
                    p.discard_ml();
                }
            }
        }
        let pre_sync = self.sync_from_after(if need_all_before {
            MTLStages::All
        } else {
            UTIL_STAGES
        });
        (cur_fence, pre_sync, was_full)
    }

    /// Wait on `pending_mfx_wait_fence` for the subset of `stages` that has not
    /// yet synchronized with the post-MetalFX fence, clearing those stage bits.
    #[inline]
    pub(super) fn wait_pending_mfx_fence(
        &mut self,
        enc: &ProtocolObject<dyn MTL4CommandEncoder>,
        stages: MTLStages,
    ) -> MTLStages {
        let wait = self.pending_mfx_wait_stages & stages;
        if wait.is_empty() {
            return MTLStages::empty();
        }
        if let Some(f) = self.pending_mfx_wait_fence.clone() {
            enc.waitForFence_beforeEncoderStages(&f, wait);
            self.pending_mfx_wait_stages -= wait;
            if self.pending_mfx_wait_stages.is_empty() {
                self.pending_mfx_wait_fence = None;
                self.pending_mfx_after = MTLStages::empty();
            }
            wait
        } else {
            self.pending_mfx_wait_stages = MTLStages::empty();
            self.pending_mfx_after = MTLStages::empty();
            MTLStages::empty()
        }
    }

    /// Consume `pending_mfx_wait_fence` at the start of the first post-MetalFX
    /// encoder (`begin_barrier`).
    ///
    /// In stage-scoped mode (`!full_mfx`), the encoder waits on the fence only
    /// for the stages that actually read MetalFX output on opening (e.g.
    /// `Fragment` on a render pass without pre-raster reads, letting `PRE_RASTER`
    /// overlap with MetalFX), then emits a single stage-scoped producer barrier
    /// `wait_stages -> UTIL_STAGES` so every subsequent encoder (and subsequent
    /// command buffer) is transitively ordered after MetalFX without extra
    /// per-encoder fence waits or close-time barriers. Any unwaited intra-encoder
    /// stages (such as `PRE_RASTER` on the same render encoder) keep the fence
    /// only until `end_encoder()`.
    pub(super) fn consume_pending_mfx_fence_on_enc_open(
        &mut self,
        enc: &ProtocolObject<dyn MTL4CommandEncoder>,
        render: bool,
        full_mfx: bool,
        opening_pre_raster_reads: u8,
    ) {
        let wait_stages = metalfx_enc_fence_wait_stages(
            self.pending_mfx_wait_stages,
            render,
            full_mfx,
            opening_pre_raster_reads,
            self.split_compute_exact_active(),
            self.bar.opening_compute_stage,
        );
        if full_mfx {
            if let Some(f) = self.pending_mfx_wait_fence.take() {
                self.pending_mfx_wait_stages = MTLStages::empty();
                self.pending_mfx_after = MTLStages::empty();
                self.pending_mfx_cross_enc_released = false;
                enc.waitForFence_beforeEncoderStages(&f, wait_stages);
                enc.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    wait_stages,
                    MTLStages::All,
                    MTL4VisibilityOptions::Device,
                );
            }
            return;
        }
        if wait_stages.is_empty() {
            return;
        }
        if let Some(f) = self.pending_mfx_wait_fence.clone() {
            enc.waitForFence_beforeEncoderStages(&f, wait_stages);
            enc.barrierAfterStages_beforeQueueStages_visibilityOptions(
                wait_stages,
                UTIL_STAGES,
                MTL4VisibilityOptions::Device,
            );
            self.pending_mfx_cross_enc_released = true;
            let same_enc_stages = if render {
                PRE_RASTER | MTLStages::Fragment
            } else {
                COMPUTE_STAGES
            };
            self.pending_mfx_wait_stages = (self.pending_mfx_wait_stages - wait_stages) & same_enc_stages;
            if self.pending_mfx_wait_stages.is_empty() {
                self.pending_mfx_wait_fence = None;
                self.pending_mfx_after = MTLStages::empty();
            }
        }
    }

    /// Once the first post-MetalFX encoder closes, its `barrierAfterStages`
    /// producer barrier (`wait_stages -> UTIL_STAGES`) has transitively ordered
    /// and made visible MetalFX's writes for all subsequent encoders. Clear any
    /// remaining intra-encoder fence state and strip `MTLStages::MachineLearning`
    /// (which is outside 7-bit `STAGE_BITS` tracking) so it does not linger into
    /// unrelated later passes.
    pub(super) fn finish_mfx_fence_on_encoder_end(&mut self) {
        if !self.pending_mfx_cross_enc_released {
            return;
        }
        self.pending_mfx_cross_enc_released = false;
        self.pending_mfx_wait_fence = None;
        self.pending_mfx_wait_stages = MTLStages::empty();
        self.pending_mfx_after = MTLStages::empty();
        let ml = MTLStages::MachineLearning;
        self.bar.pend_after -= ml;
        self.bar.split_pend.vtx_after -= ml;
        self.bar.split_pend.frag_after -= ml;
        self.bar.split_pend.disp_after -= ml;
        self.bar.split_pend.blit_after -= ml;
        self.bar.split_pend.as_after -= ml;
        for i in 0..2 {
            self.bar.split_pend.soft_after[i] -= ml;
            if let Some((a, b)) = self.bar.vtx_carry[i] {
                let na = a - ml;
                self.bar.vtx_carry[i] = if na.is_empty() { None } else { Some((na, b)) };
            }
            self.bar.vtx_carry_items[i].retain_mut(|it| {
                it.after -= ml;
                !it.after.is_empty()
            });
        }
    }

    /// Flush any deferred post-MetalFX fence wait into a minimal fallback compute encoder.
    pub(super) fn flush_pending_mfx_wait_fence(&mut self) {
        if let Some(f) = self.pending_mfx_wait_fence.take() {
            let after = std::mem::replace(&mut self.pending_mfx_after, MTLStages::empty());
            self.pending_mfx_wait_stages = MTLStages::empty();
            self.pending_mfx_cross_enc_released = false;
            let scoped = !after.is_empty()
                && after != MTLStages::All
                && !legacy_barriers()
                && barrier_masks();
            let before_q = if scoped { UTIL_STAGES } else { MTLStages::All };
            if let Some(p) = self.pt.as_mut() {
                // This fallback fence-flush compute encoder has 0 dispatches (its relaxed
                // timestamp resolves to 0 on AGX), so discard any trailing pending ML pass
                // rather than stamping an invalid endpoint.
                p.discard_ml();
            }
            if let Some(e) = self.cb.computeCommandEncoder() {
                e.waitForFence_beforeEncoderStages(&f, MTLStages::Dispatch);
                if scoped {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        after,
                        MTLStages::Dispatch,
                        MTL4VisibilityOptions::Device,
                    );
                }
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::Dispatch,
                    before_q,
                    MTL4VisibilityOptions::Device,
                );
                e.endEncoding();
            }
        }
    }

    /// Flush any unwaited post-MetalFX fence on `CommandList::close()`.
    /// When an encoder already opened after MetalFX and emitted its stage-scoped
    /// `barrierAfterStages(wait_stages, UTIL_STAGES, Device)` producer barrier,
    /// no extra close-time barrier or fallback compute encoder is needed.
    pub(super) fn flush_pending_mfx_wait_fence_on_close(&mut self) {
        if self.pending_mfx_wait_fence.is_none() || self.pending_mfx_cross_enc_released {
            return;
        }
        self.flush_pending_mfx_wait_fence();
    }

    /// Record that an opaque `MTL4FX*Scaler` pass ran directly on `self.cb`,
    /// arming stage-scoped post-pass synchronization (or a conservative full
    /// barrier fallback when legacy barriers / no fence apply).
    #[allow(clippy::too_many_arguments)]
    fn finish_opaque_metalfx_pass(
        &mut self,
        fence: Option<&ProtocolObject<dyn MTLFence>>,
        output_usage: MTLTextureUsage,
        has_fence: bool,
        has_scratch_blit: bool,
        dst_res: usize,
        pre_sync: StageSync,
        was_full: bool,
        tight_stages: bool,
    ) {
        self.bar.has_gpu_work = true;
        self.bar.uav_passthrough.clear();
        self.bar.cs_disp_passthrough.clear();
        let policy = metalfx_post_sync_policy(
            output_usage,
            has_fence,
            has_scratch_blit,
            tight_stages,
            legacy_barriers(),
            barrier_masks(),
        );
        match policy {
            MetalFxPostSync::Scoped {
                after,
                hard_before,
                soft_pre_raster,
            } => {
                self.pending_mfx_wait_fence = fence.map(|f| f.retain());
                self.pending_mfx_wait_stages = if has_scratch_blit {
                    MTLStages::Blit
                } else {
                    UTIL_STAGES
                };
                self.pending_mfx_after = after;
                self.pending_mfx_cross_enc_released = false;
                for (i, &bit) in STAGE_BITS.iter().enumerate() {
                    if after.contains(bit) {
                        self.bar.stage_sync[i] = pre_sync;
                        self.bar.stage_gen[i] = self.bar.stage_gen[i].wrapping_add(1);
                    }
                }
                if after.contains(MTLStages::Dispatch) {
                    self.bar.cs_disp_gen = self.bar.cs_disp_gen.wrapping_add(1);
                }
                if was_full {
                    self.bar.needs_barrier = false;
                    self.bar.list_unwaited_cs = false;
                    self.bar.list_unwaited_gfx = false;
                }
                if soft_pre_raster && !was_full && dst_res != 0 {
                    self.add_pend_with_soft_res(after, hard_before, &[dst_res]);
                } else {
                    let before = if soft_pre_raster {
                        hard_before | PRE_RASTER
                    } else {
                        hard_before
                    };
                    self.add_pend(after, before);
                }
                if !has_scratch_blit {
                    stats::inc(&stats::c::qb_upscale_masked);
                }
            }
            MetalFxPostSync::FullFallback => {
                self.pending_mfx_wait_fence = fence.map(|f| f.retain());
                self.pending_mfx_wait_stages = if fence.is_some() {
                    UTIL_STAGES
                } else {
                    MTLStages::empty()
                };
                self.pending_mfx_after = MTLStages::All;
                self.pending_mfx_cross_enc_released = false;
                self.bar.stage_sync = [StageSync::default(); 7];
                for g in &mut self.bar.stage_gen {
                    *g = g.wrapping_add(1);
                }
                self.need_full(FullWhy::ClearUav);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn metalfx_post_sync_clean(&self) -> bool {
        if self.bar.needs_barrier {
            // Conservative fallback mode (`FullWhy::ClearUav`).
            self.bar.full_why == FullWhy::ClearUav
                && self
                    .bar
                    .stage_sync
                    .iter()
                    .all(|s| !s.pre_list_all && s.gen == [0; 7])
        } else {
            // Stage-scoped mode: fence armed for consumer stages and `MachineLearning`
            // producer stage queued in `split_pend` without escalating to `FullWhy::ClearUav`.
            self.pending_mfx_wait_fence.is_some()
                && !self.pending_mfx_wait_stages.is_empty()
                && self.bar.split_pend.frag_after.contains(MTLStages::MachineLearning)
                && self.bar.split_pend.disp_after.contains(MTLStages::MachineLearning)
                && self.bar.split_pend.blit_after.contains(MTLStages::MachineLearning)
        }
    }

    /// 2D upscale from `src` (`render_size`) into `dst` (`upscale_size`), using
    /// Apple MetalFX (`MTL4FXSpatialScaler`) when available and compatible, with
    /// automatic fallback to the bilinear compute shader (`port_upscale_2d_f`).
    pub fn upscale_resource(
        &mut self,
        dst: &Resource,
        src: &Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
    ) -> Result<(), String> {
        self.upscale_resource_with_origins(
            dst,
            src,
            render_size,
            upscale_size,
            [0, 0],
            [0, 0],
            barrier_tight_stages(),
        )
    }

    /// 2D upscale between arbitrary source/destination subrects (`src_origin` -> `dst_origin`).
    pub fn upscale_resource_subrect(
        &mut self,
        dst: &Resource,
        src: &Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
        src_origin: [u32; 2],
        dst_origin: [u32; 2],
    ) -> Result<(), String> {
        self.upscale_resource_with_origins(
            dst,
            src,
            render_size,
            upscale_size,
            src_origin,
            dst_origin,
            barrier_tight_stages(),
        )
    }

    #[cfg(test)]
    pub(super) fn upscale_resource_with(
        &mut self,
        dst: &Resource,
        src: &Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
        tight_stages: bool,
    ) -> Result<(), String> {
        self.upscale_resource_with_origins(
            dst,
            src,
            render_size,
            upscale_size,
            [0, 0],
            [0, 0],
            tight_stages,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn upscale_resource_with_origins(
        &mut self,
        dst: &Resource,
        src: &Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
        src_origin: [u32; 2],
        dst_origin: [u32; 2],
        tight_stages: bool,
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        dst.register_ptr();
        src.register_ptr();
        dst.mark_gpu_used();
        src.mark_gpu_used();
        let touch = [dst as *const Resource as usize, src as *const Resource as usize];
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;
        use crate::metal::clear::{self, TexParams};
        let st = src.texture().ok_or("upscale_resource: src has no texture")?;
        let dt = dst.texture().ok_or("upscale_resource: dst has no texture")?;
        let s_fmt = crate::format::view_format(src.desc.Format, src.mtl_format);
        let d_fmt = crate::format::view_format(dst.desc.Format, dst.mtl_format);
        let sv = src
            .texture_view(s_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("upscale_resource: src view")?;
        let dv = dst
            .texture_view(d_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("upscale_resource: dst view")?;
        let st_w = st.width() as u32;
        let st_h = st.height() as u32;
        let dt_w = dt.width() as u32;
        let dt_h = dt.height() as u32;
        if src_origin[0] >= st_w
            || src_origin[1] >= st_h
            || dst_origin[0] >= dt_w
            || dst_origin[1] >= dt_h
        {
            return Err(format!(
                "upscale_resource: subrect origin out of bounds (src_origin={src_origin:?} in {st_w}x{st_h}, dst_origin={dst_origin:?} in {dt_w}x{dt_h})"
            ));
        }
        let avail_src_w = st_w - src_origin[0];
        let avail_src_h = st_h - src_origin[1];
        let avail_dst_w = dt_w - dst_origin[0];
        let avail_dst_h = dt_h - dst_origin[1];
        let x0 = if render_size[0] > 0 {
            render_size[0].min(avail_src_w)
        } else {
            avail_src_w
        };
        let y0 = if render_size[1] > 0 {
            render_size[1].min(avail_src_h)
        } else {
            avail_src_h
        };
        let w = if upscale_size[0] > 0 {
            upscale_size[0].min(avail_dst_w)
        } else {
            avail_dst_w
        };
        let h = if upscale_size[1] > 0 {
            upscale_size[1].min(avail_dst_h)
        } else {
            avail_dst_h
        };
        if w == 0 || h == 0 {
            return Ok(());
        }
        if x0 == w && y0 == h {
            if touch[0] == touch[1] && s_fmt == d_fmt && src_origin == dst_origin {
                return Ok(());
            }
            if s_fmt == d_fmt {
                return self.copy_upscale_1to1(
                    &sv,
                    &dv,
                    src_origin,
                    dst_origin,
                    w,
                    h,
                    &touch,
                    tight_stages,
                );
            }
            let p = TexParams {
                u: [src_origin[0], src_origin[1], dst_origin[0], dst_origin[1]],
                f: [0.0; 4],
                x0,
                y0,
                z0: 0,
                w,
                h,
                d: 1,
                _pad: [0; 2],
            };
            return self.util_dispatch_with(
                "port_upscale_2d_f",
                Some(&sv),
                Some(&dv),
                clear::bytes_of(&p),
                [w, h, 1],
                &touch,
                tight_stages,
            );
        }
        if crate::metal::gpunan::is_capturing() && self.open {
            self.flush_pending_mfx_wait_fence();
            self.end_encoder();
            let _ = crate::metal::gpunan::encode_scan_resource(
                &self.cb,
                src,
                "upscale_res_in",
                "upscale",
                0,
                0,
                0,
                0,
            );
            self.need_full(FullWhy::ClearUav);
        }
        if self.try_upscale_metalfx(
            &sv,
            &dv,
            src_origin,
            dst_origin,
            x0,
            y0,
            w,
            h,
            s_fmt,
            d_fmt,
            &touch,
            tight_stages,
        )? {
            if crate::metal::gpunan::is_capturing() && self.open {
                self.flush_pending_mfx_wait_fence();
                self.end_encoder();
                let _ = crate::metal::gpunan::encode_scan_resource(
                    &self.cb,
                    dst,
                    "upscale_res_out_smfx",
                    "upscale",
                    0,
                    0,
                    0,
                    0,
                );
                self.need_full(FullWhy::ClearUav);
            }
            return Ok(());
        }
        let p = TexParams {
            u: [src_origin[0], src_origin[1], dst_origin[0], dst_origin[1]],
            f: [0.0; 4],
            x0,
            y0,
            z0: 0,
            w,
            h,
            d: 1,
            _pad: [0; 2],
        };
        self.util_dispatch_with(
            "port_upscale_2d_f",
            Some(&sv),
            Some(&dv),
            clear::bytes_of(&p),
            [w, h, 1],
            &touch,
            tight_stages,
        )?;
        if crate::metal::gpunan::is_capturing() && self.open {
            self.end_encoder();
            let _ = crate::metal::gpunan::encode_scan_resource(
                &self.cb,
                dst,
                "upscale_res_out_bilin",
                "upscale",
                0,
                0,
                0,
                0,
            );
            self.need_full(FullWhy::ClearUav);
        }
        Ok(())
    }

    /// FidelityFX FSR3 autogen reactive mask pass (`port_ffx_gen_reactive_2d`):
    /// computes `out_reactive` from `opaque` and `pre_upscale` color textures
    /// honoring `APPLY_TONEMAP` (1), `APPLY_INVERSETONEMAP` (2),
    /// `APPLY_THRESHOLD` (4), and `USE_COMPONENTS_MAX` (8).
    #[allow(clippy::too_many_arguments)]
    pub fn generate_reactive_mask(
        &mut self,
        opaque: &Resource,
        pre_upscale: &Resource,
        out_reactive: &Resource,
        render_size: [u32; 2],
        scale: f32,
        cutoff_threshold: f32,
        binary_value: f32,
        flags: u32,
    ) -> Result<(), String> {
        use crate::metal::clear::{self, FfxReactiveParams};

        self.flush_icb_prepass_if_pending();
        out_reactive.register_ptr();
        opaque.register_ptr();
        pre_upscale.register_ptr();
        out_reactive.mark_gpu_used();
        opaque.mark_gpu_used();
        pre_upscale.mark_gpu_used();
        let touch = [
            out_reactive as *const Resource as usize,
            opaque as *const Resource as usize,
            pre_upscale as *const Resource as usize,
        ];
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;

        let ot = opaque.texture().ok_or("generate_reactive_mask: opaque has no texture")?;
        let pt = pre_upscale
            .texture()
            .ok_or("generate_reactive_mask: pre_upscale has no texture")?;
        let dt = out_reactive
            .texture()
            .ok_or("generate_reactive_mask: out_reactive has no texture")?;
        let o_fmt = crate::format::view_format(opaque.desc.Format, opaque.mtl_format);
        let p_fmt = crate::format::view_format(pre_upscale.desc.Format, pre_upscale.mtl_format);
        let d_fmt = crate::format::view_format(out_reactive.desc.Format, out_reactive.mtl_format);
        let ov = opaque
            .texture_view(o_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("generate_reactive_mask: opaque view")?;
        let pv = pre_upscale
            .texture_view(p_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("generate_reactive_mask: pre_upscale view")?;
        let dv = out_reactive
            .texture_view(d_fmt, MTLTextureType::Type2D, (0, 1), (0, 1))
            .ok_or("generate_reactive_mask: out_reactive view")?;

        let max_w = (ot.width() as u32).min(pt.width() as u32).min(dt.width() as u32);
        let max_h = (ot.height() as u32).min(pt.height() as u32).min(dt.height() as u32);
        let w = if render_size[0] > 0 { render_size[0].min(max_w) } else { max_w };
        let h = if render_size[1] > 0 { render_size[1].min(max_h) } else { max_h };
        if w == 0 || h == 0 {
            return Ok(());
        }
        let params = FfxReactiveParams {
            width: w,
            height: h,
            flags,
            _pad0: 0,
            scale: if scale.is_finite() { scale } else { 1.0 },
            cutoff_threshold: if cutoff_threshold.is_finite() { cutoff_threshold } else { 0.0 },
            binary_value: if binary_value.is_finite() { binary_value } else { 0.0 },
            _pad1: 0.0,
        };
        let (kernel_name, grid) = if crate::metal::m41::block_read_kernels_enabled() {
            ("port_ffx_gen_reactive_2d_b2", [w.div_ceil(2), h, 1])
        } else {
            ("port_ffx_gen_reactive_2d", [w, h, 1])
        };
        self.util_dispatch_3tex_with(
            kernel_name,
            Some(&ov),
            Some(&pv),
            Some(&dv),
            clear::bytes_of(&params),
            grid,
            &touch,
            barrier_tight_stages(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn copy_upscale_1to1(
        &mut self,
        sv: &MtlTexture,
        dv: &MtlTexture,
        src_origin: [u32; 2],
        dst_origin: [u32; 2],
        w: u32,
        h: u32,
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<(), String> {
        let masked = tight_stages && !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        if masked {
            self.add_pend(UTIL_STAGES, MTLStages::Blit);
            if ib_tight {
                for &r in touch {
                    self.ib_note_touch_before_tight(r, MTLStages::Blit);
                }
            } else {
                for &r in touch {
                    self.ib_note(r);
                }
            }
            stats::inc(&stats::c::qb_upscale_masked);
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        let e = self
            .compute_blit_touch(touch)
            .ok_or("upscale_resource 1:1 blit encoder")?;
        self.pt_note_util(pt::synth::UPSCALE, 0);
        unsafe {
            e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                sv,
                0,
                0,
                MTLOrigin { x: src_origin[0] as usize, y: src_origin[1] as usize, z: 0 },
                MTLSize { width: w as usize, height: h as usize, depth: 1 },
                dv,
                0,
                0,
                MTLOrigin { x: dst_origin[0] as usize, y: dst_origin[1] as usize, z: 0 },
            );
        }
        self.note_stage_ran(MTLStages::Blit);
        if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
            self.ib.ib_blit_dst.push(touch[0]);
        }
        if masked {
            self.add_pend(MTLStages::Blit, UTIL_STAGES);
            if ib_tight {
                for &r in touch {
                    self.ib_add_obl_tight(r, MTLStages::Blit, COMPUTE_STAGES);
                }
            } else {
                for &r in touch {
                    self.ib_note(r);
                }
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn try_upscale_metalfx(
        &mut self,
        sv: &MtlTexture,
        dv: &MtlTexture,
        src_origin: [u32; 2],
        dst_origin: [u32; 2],
        x0: u32,
        y0: u32,
        w: u32,
        h: u32,
        s_fmt: MTLPixelFormat,
        d_fmt: MTLPixelFormat,
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<bool, String> {
        if !self.open || !crate::metal::metalfx::is_enabled() {
            return Ok(false);
        }
        if x0 == 0
            || y0 == 0
            || x0 > w
            || y0 > h
            || (x0 == w && y0 == h)
            || sv.textureType() != MTLTextureType::Type2D
            || dv.textureType() != MTLTextureType::Type2D
            || dv.storageMode() != MTLStorageMode::Private
        {
            return Ok(false);
        }
        let has_src_subrect = src_origin != [0, 0];
        let (in_w, in_h) = if has_src_subrect {
            (x0, y0)
        } else {
            (sv.width() as u32, sv.height() as u32)
        };
        let Some(scaler_arc) = crate::metal::metalfx::get_or_create_scaler(
            in_w,
            in_h,
            s_fmt,
            w,
            h,
            d_fmt,
        ) else {
            return Ok(false);
        };
        let mut scaler = scaler_arc.lock().unwrap();
        if !has_src_subrect && !sv.usage().contains(scaler.color_usage) {
            return Ok(false);
        }
        let color_usage = scaler.color_usage;
        let direct_ok = dv.usage().contains(scaler.output_usage)
            && dst_origin == [0, 0]
            && w == dv.width() as u32
            && h == dv.height() as u32;
        let scratch = if direct_ok {
            None
        } else if w >= 64 && h >= 64 {
            let Some(st) = crate::metal::metalfx::scratch_output_texture(w, h, d_fmt) else {
                return Ok(false);
            };
            Some(st)
        } else {
            return Ok(false);
        };

        self.flush_clear_tex_cs_touch(touch);
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(false);
        }
        let staged_src = if has_src_subrect {
            let Some(sc) = self.blit_subrect_to_scratch(
                sv,
                src_origin,
                in_w,
                in_h,
                1,
                s_fmt,
                color_usage,
                touch,
                tight_stages,
            )?
            else {
                return Ok(false);
            };
            Some(sc)
        } else {
            None
        };
        let output_usage = scaler.output_usage;
        let has_fence = scaler.has_fence();
        let has_scratch_blit = scratch.is_some();
        let (mfx_fence, pre_sync, was_full) = self.sync_queue_before_metalfx(
            output_usage,
            has_fence,
            tight_stages,
            pt::synth::METALFX_SPATIAL,
            w,
            h,
        );
        let effective_has_fence = has_fence && mfx_fence.is_some();

        let mfx_src: &MtlTexture = staged_src.as_deref().unwrap_or(sv);
        let mfx_dst: &MtlTexture = scratch.as_deref().unwrap_or(dv);
        scaler.encode(&self.cb, mfx_src, mfx_dst, x0, y0, mfx_fence.as_deref());
        drop(scaler);
        self.finish_opaque_metalfx_pass(
            mfx_fence.as_deref(),
            output_usage,
            effective_has_fence,
            has_scratch_blit,
            touch[0],
            pre_sync,
            was_full,
            tight_stages,
        );

        self.finish_metalfx_output_blit(
            scratch,
            dv,
            dst_origin,
            w,
            h,
            touch,
            tight_stages,
            "metalfx scratch blit encoder",
        )?;
        Ok(true)
    }

    /// The utility-kernel argument table (one buffer, three textures).
    pub(super) fn ensure_util_table(&mut self) -> Result<Retained<MtlArgumentTable>, String> {
        if self.util_table.is_none() {
            let d = crate::metal::device().ok_or("no device")?;
            let desc = MTL4ArgumentTableDescriptor::new();
            desc.setMaxBufferBindCount(1);
            desc.setMaxTextureBindCount(3);
            desc.setInitializeBindings(true);
            desc.setLabel(Some(&objc2_foundation::NSString::from_str(
                "aw2 utility kernels",
            )));
            self.util_table = Some(
                d.dev
                    .newArgumentTableWithDescriptor_error(&desc)
                    .map_err(|e| format!("newArgumentTable: {e}"))?,
            );
        }
        Ok(self.util_table.as_ref().unwrap().clone())
    }

    fn with_rt_ctx(
        &mut self,
        f: impl FnOnce(&mut crate::rt::accel::Ctx) -> Result<(), String>,
    ) -> Result<(), String> {
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let keep = self.rt_keep.clone().ok_or("command list is not open")?;
        let table = self.ensure_util_table()?;
        self.flush_enc_cs_soft_disp_all();
        self.flush_cs_uav_deferred();
        self.flush_cs_end_after();
        self.flush_cs_mid_res_all();
        let enc = self.compute().ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::RT_ACCEL, 0);
        let res = {
            let mut r = ring.lock().unwrap();
            let mut k = keep.lock().unwrap();
            let mut ctx = crate::rt::accel::Ctx {
                enc: &enc,
                table: &table,
                ring: &mut r,
                keep: &mut k,
            };
            f(&mut ctx)
        };
        self.note_stage_ran(COMPUTE_STAGES);
        self.bar.cs_as_gen = self.bar.cs_as_gen.wrapping_add(1);
        self.bar.cs_disp_gen = self.bar.cs_disp_gen.wrapping_add(1);
        enc.setArgumentTable(Some(&self.table));
        self.compute_pso_bound = false;
        res
    }

    /// `BuildRaytracingAccelerationStructure`.
    pub fn build_accel(
        &mut self,
        args: &crate::rt::accel::BuildArgs,
        postbuild: &[crate::rt::d3d::PostbuildInfoDesc],
    ) -> Result<(), String> {
        self.with_rt_ctx(|ctx| crate::rt::accel::encode_build(ctx, args, postbuild))
    }

    /// `EmitRaytracingAccelerationStructurePostbuildInfo`.
    pub fn emit_accel_postbuild(
        &mut self,
        desc: &crate::rt::d3d::PostbuildInfoDesc,
        sources: &[u64],
    ) -> Result<(), String> {
        self.with_rt_ctx(|ctx| crate::rt::accel::encode_postbuild(ctx, desc, sources))
    }

    /// `CopyRaytracingAccelerationStructure`.
    pub fn copy_accel(
        &mut self,
        dest: u64,
        source: u64,
        mode: u32,
        owner: u64,
    ) -> Result<(), String> {
        self.with_rt_ctx(|ctx| crate::rt::accel::encode_copy(ctx, dest, source, mode, owner))
    }

    /// `SetPipelineState1`: bind an executable raytracing state object.
    pub fn set_raytracing_pipeline(&mut self, so: Arc<crate::rt::pipeline::StateObject>) {
        if self.rt_pso.as_ref().is_some_and(|c| Arc::ptr_eq(c, &so)) {
            return;
        }
        self.rt_pso = Some(so);
    }

    /// `DispatchRays(desc)`.
    pub fn dispatch_rays(
        &mut self,
        desc: &crate::rt::pipeline::DispatchRaysDesc,
    ) -> Result<(), String> {
        self.disc_unmark(0);
        if desc.Width == 0 || desc.Height == 0 || desc.Depth == 0 {
            return Ok(());
        }
        let so = self
            .rt_pso
            .clone()
            .ok_or("DispatchRays without a raytracing pipeline")?;
        let rt = so
            .pipeline
            .as_ref()
            .ok_or("bound state object is not an executable raytracing pipeline")?;
        let grs = self.flush_root(true)?;
        let arg = crate::rt::pipeline::DispatchRaysArgument {
            dispatch_rays_desc: *desc,
            grs,
            res_desc_heap: self.heaps[0],
            smp_desc_heap: self.heaps[1],
            visible_function_table: rt.vft_gpu_id,
            intersection_function_table: rt.ift_gpu_id,
            intersection_function_tables: 0,
        };
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let arg_va = ring.lock().unwrap().upload(arg.as_bytes(), 256)?;
        let e = self.compute().ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::RT_RAYS, 1);
        e.setComputePipelineState(&rt.pso);
        self.compute_pso_bound = false;
        self.bind_table(grs);
        self.set_table_addr(arg_va, crate::rt::pipeline::RAY_DISPATCH_ARGUMENTS_BIND_POINT);
        e.dispatchThreads_threadsPerThreadgroup(
            MTLSize {
                width: desc.Width as usize,
                height: desc.Height as usize,
                depth: desc.Depth as usize,
            },
            MTLSize {
                width: (desc.Width as usize).clamp(1, 8),
                height: (desc.Height as usize).clamp(1, 8),
                depth: 1,
            },
        );
        self.note_stage_ran(MTLStages::Dispatch);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::PlacedFootprint;
    use crate::metal::cmdlist::{CommandAllocator, CopyLoc, TargetView};
    use crate::metal::metalfx::{is_temporal_enabled, TemporalUpscaleParams, UpscaleBackend};
    use crate::metal::queue::{ListType, Queue};
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
    use std::sync::atomic::Ordering::Relaxed;

    const RGBA16_FLOAT: u32 = 10;
    const RG16_FLOAT: u32 = 34;
    const D32_FLOAT: u32 = 40;

    fn make_tex(w: u32, h: u32, fmt: u32, flags: u32, label: &str) -> Resource {
        let desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: w as u64,
            Height: h,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: fmt,
            SampleCount: 1,
            Flags: flags,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        Resource::new_committed_default(&desc, &hp, label).unwrap()
    }

    fn make_readback(bytes: u64) -> Resource {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: bytes,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
        Resource::new_committed_default(&desc, &hp, "metalfx barrier readback").unwrap()
    }

    fn f16_to_f32(h: u16) -> f32 {
        let s = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
        let e = ((h >> 10) & 0x1f) as i32;
        let m = (h & 0x3ff) as f32;
        s * match e {
            0 => m * 2f32.powi(-24),
            31 => {
                if m == 0.0 {
                    f32::INFINITY
                } else {
                    f32::NAN
                }
            }
            _ => (1.0 + m / 1024.0) * 2f32.powi(e - 15),
        }
    }

    #[test]
    fn metalfx_barrier_translation_stage_scoping_and_fallbacks() {
        // 1) Pure compute/ML MetalFX output (no RenderTarget) scopes to Dispatch|Blit|MachineLearning.
        let cs_only_usage = MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite;
        assert_eq!(metalfx_producer_stages(cs_only_usage), MFX_COMPUTE_ML_STAGES);
        assert_eq!(metalfx_consumer_stages(cs_only_usage), MFX_COMPUTE_ML_STAGES);
        assert!(!metalfx_producer_stages(cs_only_usage).contains(MTLStages::Fragment));
        assert!(metalfx_producer_stages(cs_only_usage).contains(MTLStages::MachineLearning));

        // 2) RenderTarget-backed MetalFX output includes Fragment alongside Dispatch|Blit|MachineLearning,
        //    never escalating to MTLStages::All or PRE_RASTER.
        let rt_usage = MTLTextureUsage::ShaderRead
            | MTLTextureUsage::ShaderWrite
            | MTLTextureUsage::RenderTarget;
        let rt_stages = metalfx_producer_stages(rt_usage);
        assert_eq!(rt_stages, MFX_COMPUTE_ML_STAGES | MTLStages::Fragment);
        assert_ne!(rt_stages, MTLStages::All);
        assert!(!rt_stages.intersects(PRE_RASTER));

        // 3) Pre-MetalFX producer barrier narrows from All->All to fence_stage->mfx_consumer_stages
        //    when tight_stages + masks + fence are available, and falls back to All->All otherwise.
        assert_eq!(
            metalfx_pre_producer_barrier(MTLStages::Fragment, rt_usage, true, true, false, true),
            (MTLStages::Fragment, MFX_COMPUTE_ML_STAGES | MTLStages::Fragment),
        );
        assert_eq!(
            metalfx_pre_producer_barrier(MTLStages::Dispatch, cs_only_usage, true, true, false, true),
            (MTLStages::Dispatch, MFX_COMPUTE_ML_STAGES),
        );
        for (has_fence, tight, legacy, masks) in [
            (false, true, false, true),
            (true, false, false, true),
            (true, true, true, true),
            (true, true, false, false),
        ] {
            assert_eq!(
                metalfx_pre_producer_barrier(
                    MTLStages::Fragment,
                    rt_usage,
                    has_fence,
                    tight,
                    legacy,
                    masks,
                ),
                (MTLStages::All, MTLStages::All),
                "fallback pre-barrier for has_fence={has_fence} tight={tight} legacy={legacy} masks={masks}"
            );
        }

        // 4) Post-MetalFX policy: direct output uses stage-scoped hard Fragment|COMPUTE_STAGES
        //    plus deferred soft PRE_RASTER; scratch-blit narrows strictly to Blit; missing fence or
        //    legacy/unmasked flags fall back conservatively to FullFallback.
        assert_eq!(
            metalfx_post_sync_policy(rt_usage, true, false, true, false, true),
            MetalFxPostSync::Scoped {
                after: MFX_COMPUTE_ML_STAGES | MTLStages::Fragment,
                hard_before: MTLStages::Fragment | COMPUTE_STAGES,
                soft_pre_raster: true,
            },
        );
        assert_eq!(
            metalfx_post_sync_policy(cs_only_usage, true, false, true, false, true),
            MetalFxPostSync::Scoped {
                after: MFX_COMPUTE_ML_STAGES,
                hard_before: MTLStages::Fragment | COMPUTE_STAGES,
                soft_pre_raster: true,
            },
        );
        assert_eq!(
            metalfx_post_sync_policy(rt_usage, true, true, true, false, true),
            MetalFxPostSync::Scoped {
                after: MFX_COMPUTE_ML_STAGES | MTLStages::Fragment,
                hard_before: MTLStages::Blit,
                soft_pre_raster: false,
            },
        );
        for (has_fence, tight, legacy, masks) in [
            (false, true, false, true),
            (true, false, false, true),
            (true, true, true, true),
            (true, true, false, false),
        ] {
            assert_eq!(
                metalfx_post_sync_policy(rt_usage, has_fence, false, tight, legacy, masks),
                MetalFxPostSync::FullFallback,
                "fallback post-sync for has_fence={has_fence} tight={tight} legacy={legacy} masks={masks}"
            );
        }

        // 5) Per-encoder fence wait stage selection:
        //    - Render pass with no pre-raster reads waits only on Fragment (not PRE_RASTER).
        assert_eq!(
            metalfx_enc_fence_wait_stages(UTIL_STAGES, true, false, 0, true, MTLStages::empty()),
            MTLStages::Fragment,
        );
        //    - Render pass with pre-raster reads waits on PRE_RASTER | Fragment.
        assert_eq!(
            metalfx_enc_fence_wait_stages(UTIL_STAGES, true, false, 1, true, MTLStages::empty()),
            PRE_RASTER | MTLStages::Fragment,
        );
        //    - Subsequent render pass after Fragment already waited only waits if pre-raster reads occur.
        let after_frag = UTIL_STAGES - MTLStages::Fragment;
        assert_eq!(
            metalfx_enc_fence_wait_stages(after_frag, true, false, 0, true, MTLStages::empty()),
            MTLStages::empty(),
        );
        assert_eq!(
            metalfx_enc_fence_wait_stages(after_frag, true, false, 1, true, MTLStages::empty()),
            PRE_RASTER,
        );
        //    - Compute pass with split_compute_exact scopes fence wait to the opening compute stage.
        assert_eq!(
            metalfx_enc_fence_wait_stages(after_frag, false, false, 0, true, MTLStages::Blit),
            MTLStages::Blit,
        );
        assert_eq!(
            metalfx_enc_fence_wait_stages(after_frag, false, false, 0, true, MTLStages::Dispatch),
            MTLStages::Dispatch,
        );
        assert_eq!(
            metalfx_enc_fence_wait_stages(after_frag, false, false, 0, false, MTLStages::Blit),
            COMPUTE_STAGES,
        );
        //    - Full fallback waits on all encoder stages.
        assert_eq!(
            metalfx_enc_fence_wait_stages(
                MTLStages::empty(),
                true,
                true,
                0,
                true,
                MTLStages::empty(),
            ),
            PRE_RASTER | MTLStages::Fragment,
        );
        assert_eq!(
            metalfx_enc_fence_wait_stages(
                MTLStages::empty(),
                false,
                true,
                0,
                true,
                MTLStages::Blit,
            ),
            COMPUTE_STAGES,
        );
    }

    #[test]
    fn metalfx_post_pass_avoids_full_queue_barrier_and_preserves_multi_encoder_coherence_on_gpu() {
        if crate::metal::device().is_none() || !is_temporal_enabled() {
            return;
        }
        let q = Queue::new(ListType::Direct, "mfx narrow barrier gpu test").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        let color = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "nb c");
        let depth = make_tex(64, 64, D32_FLOAT, resource_flags::ALLOW_DEPTH_STENCIL, "nb d");
        let motion = make_tex(64, 64, RG16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "nb m");
        let out = make_tex(
            128,
            128,
            RGBA16_FLOAT,
            resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS,
            "nb out",
        );
        let aux = make_tex(128, 128, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "nb aux");
        let rb = make_readback(128 * 8 * 128);
        let rtv_c = TargetView::rtv(&color, None).unwrap();
        let dsv_d = TargetView::dsv(&depth, None).unwrap();
        let rtv_m = TargetView::rtv(&motion, None).unwrap();
        let rtv_aux = TargetView::rtv(&aux, None).unwrap();
        let fp = PlacedFootprint {
            offset: 0,
            format: RGBA16_FLOAT,
            width: 128,
            height: 128,
            depth: 1,
            row_pitch: 1024,
            _pad: 0,
        };

        // 1) Direct temporal MetalFX -> Render pass (Fragment wait) -> Blit copy (Blit wait)
        //    must emit ZERO `qb_clear_uav` / `q_barrier_all` after MetalFX while keeping GPU
        //    output coherent and free of Metal validation errors.
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_c, [0.9, 0.4, 0.2, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
            l.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        // Flush list-start clears before sampling q_barrier_all so only post-MetalFX barriers are counted.
        l.flush_clears_for(&[
            &color as *const Resource as usize,
            &depth as *const Resource as usize,
            &motion as *const Resource as usize,
        ])
        .unwrap();

        let qba0 = stats::c::q_barrier_all.load(Relaxed);
        let qbc0 = stats::c::qb_clear_uav.load(Relaxed);
        let qbf0 = stats::c::q_barrier_frag.load(Relaxed);

        let mut p = TemporalUpscaleParams::new(&out, &color, &depth, &motion, [64, 64], [128, 128]);
        p.reset = true;
        p.auto_exposure = true;
        assert_eq!(l.upscale_temporal_with_backend(&p).unwrap(), UpscaleBackend::TemporalMetalFx);
        assert!(l.metalfx_post_sync_clean());
        assert!(
            !l.bar.needs_barrier,
            "scoped temporal MetalFX must not arm a full-stage queue barrier"
        );

        // Intervening render pass consumes Fragment wait and releases the fence to subsequent
        // encoders; closing the render pass must clear pending_mfx_wait_fence and strip
        // MachineLearning from split_pend so it does not linger, while the subsequent Blit copy
        // reads back exact upscaled pixels.
        unsafe {
            l.clear_rtv(
                &rtv_aux,
                [0.1, 0.1, 0.1, 1.0],
                &[crate::types::Rect { left: 0, top: 0, right: 32, bottom: 32 }],
            )
            .unwrap();
        }
        assert!(
            l.pending_mfx_wait_fence.is_none() && !l.pending_mfx_cross_enc_released,
            "closing the first post-MetalFX render pass must finalize fence release"
        );
        assert!(!l.bar.split_pend.disp_after.contains(MTLStages::MachineLearning));
        assert!(!l.bar.split_pend.blit_after.contains(MTLStages::MachineLearning));
        l.copy_texture_region(
            CopyLoc::Footprint(&rb, fp),
            [0, 0, 0],
            CopyLoc::Subresource(&out, 0),
            None,
        )
        .unwrap();
        l.close().unwrap();

        assert_eq!(
            stats::c::q_barrier_all.load(Relaxed) - qba0,
            0,
            "post-MetalFX render + blit sequence must emit 0 q_barrier_all"
        );
        assert_eq!(
            stats::c::qb_clear_uav.load(Relaxed) - qbc0,
            0,
            "post-MetalFX render + blit sequence must emit 0 qb_clear_uav"
        );
        assert!(
            stats::c::q_barrier_frag.load(Relaxed) - qbf0 >= 1,
            "post-MetalFX consumers must emit stage-masked queue barriers"
        );

        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        let px = unsafe { std::slice::from_raw_parts(rb.cpu_ptr().unwrap() as *const u16, 128 * 512) };
        let base = (64 * 128 + 64) * 4;
        let r = f16_to_f32(px[base]);
        let g = f16_to_f32(px[base + 1]);
        let b = f16_to_f32(px[base + 2]);
        assert!(
            (r - 0.9).abs() < 0.15 && (g - 0.4).abs() < 0.15 && (b - 0.2).abs() < 0.15,
            "multi-encoder post-MetalFX readback corrupted: ({r}, {g}, {b})"
        );

        // 1b) Closing the command list while a post-MetalFX render encoder is still open must
        //     not emit invalid compute/ML stages on the render encoder or spawn a fallback encoder.
        let mut l_close_render = CommandList::new(ListType::Direct).unwrap();
        l_close_render.begin(&alloc).unwrap();
        unsafe {
            l_close_render.clear_rtv(&rtv_c, [0.3, 0.8, 0.1, 1.0], &[]).unwrap();
            l_close_render.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
            l_close_render.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        l_close_render
            .flush_clears_for(&[
                &color as *const Resource as usize,
                &depth as *const Resource as usize,
                &motion as *const Resource as usize,
            ])
            .unwrap();
        let qba_r = stats::c::q_barrier_all.load(Relaxed);
        assert_eq!(
            l_close_render.upscale_temporal_with_backend(&p).unwrap(),
            UpscaleBackend::TemporalMetalFx
        );
        let rp = MTL4RenderPassDescriptor::new();
        let ca = unsafe { rp.colorAttachments().objectAtIndexedSubscript(0) };
        ca.setTexture(Some(aux.texture().unwrap()));
        ca.setLoadAction(MTLLoadAction::Clear);
        ca.setClearColor(MTLClearColor { red: 0.2, green: 0.2, blue: 0.2, alpha: 1.0 });
        ca.setStoreAction(MTLStoreAction::Store);
        let aux_id = &aux as *const Resource as usize;
        let _e = l_close_render.util_render_pass(&rp, &[aux_id], pt::kind::DRAW).unwrap();
        assert!(matches!(l_close_render.enc, Encoder::Render(_)));
        assert!(l_close_render.pending_mfx_cross_enc_released);
        l_close_render.close().unwrap();
        assert_eq!(stats::c::q_barrier_all.load(Relaxed) - qba_r, 0);
        assert!(l_close_render.pending_mfx_wait_fence.is_none());
        assert!(!l_close_render.pending_mfx_cross_enc_released);
        q.execute_command_lists(&mut [&mut l_close_render]);
        q.wait_idle();

        // 2) Conservative fallback (`tight_stages = false`) must still arm FullWhy::ClearUav
        //    and emit a full queue barrier on the following encoder.
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_c, [0.2, 0.7, 0.5, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
            l.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        l.flush_clears_for(&[
            &color as *const Resource as usize,
            &depth as *const Resource as usize,
            &motion as *const Resource as usize,
        ])
        .unwrap();
        let qba1 = stats::c::q_barrier_all.load(Relaxed);
        let qbc1 = stats::c::qb_clear_uav.load(Relaxed);
        let mut p2 = TemporalUpscaleParams::new(&out, &color, &depth, &motion, [64, 64], [128, 128]);
        p2.reset = true;
        p2.auto_exposure = true;
        assert_eq!(l.upscale_temporal_with(&p2, false).unwrap(), UpscaleBackend::TemporalMetalFx);
        assert!(l.bar.needs_barrier, "tight_stages=false fallback must arm FullWhy::ClearUav");
        l.copy_texture_region(
            CopyLoc::Footprint(&rb, fp),
            [0, 0, 0],
            CopyLoc::Subresource(&out, 0),
            None,
        )
        .unwrap();
        l.close().unwrap();
        assert_eq!(stats::c::qb_clear_uav.load(Relaxed) - qbc1, 1);
        assert_eq!(stats::c::q_barrier_all.load(Relaxed) - qba1, 1);
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        let px2 = unsafe { std::slice::from_raw_parts(rb.cpu_ptr().unwrap() as *const u16, 128 * 512) };
        let r2 = f16_to_f32(px2[base]);
        let g2 = f16_to_f32(px2[base + 1]);
        let b2 = f16_to_f32(px2[base + 2]);
        assert!(
            (r2 - 0.2).abs() < 0.15 && (g2 - 0.7).abs() < 0.15 && (b2 - 0.5).abs() < 0.15,
            "fallback post-MetalFX readback corrupted: ({r2}, {g2}, {b2})"
        );
    }

    #[test]
    fn allocator_mfx_fence_pool_isolates_reused_cmdlist_across_allocators() {
        if crate::metal::device().is_none() {
            return;
        }
        use crate::metal::cmdlist::CommandAllocator;
        use crate::metal::queue::ListType;

        let alloc0 = CommandAllocator::new(ListType::Direct).unwrap();
        let alloc1 = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        // First recording on `alloc0`: acquires fence A from `alloc0`'s pool.
        l.begin(&alloc0).unwrap();
        let (f0_a, _, _) = l.sync_queue_before_metalfx(
            MTLTextureUsage::ShaderWrite,
            true,
            true,
            pt::synth::METALFX_TEMPORAL,
            64,
            64,
        );
        let (f0_b, _, _) = l.sync_queue_before_metalfx(
            MTLTextureUsage::ShaderWrite,
            true,
            true,
            pt::synth::METALFX_TEMPORAL,
            64,
            64,
        );
        l.close().unwrap();

        // Second recording of the SAME `CommandList` on `alloc1` while `alloc0` is still
        // in flight: must acquire a distinct `MTLFence` from `alloc1`'s pool so the two
        // in-flight recordings never alias the same live `MTLFence`.
        l.begin(&alloc1).unwrap();
        let (f1_a, _, _) = l.sync_queue_before_metalfx(
            MTLTextureUsage::ShaderWrite,
            true,
            true,
            pt::synth::METALFX_TEMPORAL,
            64,
            64,
        );
        l.close().unwrap();

        let p0_a = Retained::as_ptr(f0_a.as_ref().expect("f0_a")) as usize;
        let p0_b = Retained::as_ptr(f0_b.as_ref().expect("f0_b")) as usize;
        let p1_a = Retained::as_ptr(f1_a.as_ref().expect("f1_a")) as usize;
        assert_ne!(p0_a, p0_b, "two MetalFX passes in one recording must use distinct fences");
        assert_ne!(p0_a, p1_a, "reusing CommandList across allocators must not alias MTLFence");
        assert_ne!(p0_b, p1_a, "reusing CommandList across allocators must not alias MTLFence");

        // After `alloc0.reset()`, its pool cursor rewinds and reuses `p0_a` without leaking.
        alloc0.reset();
        l.begin(&alloc0).unwrap();
        let (f0_reused, _, _) = l.sync_queue_before_metalfx(
            MTLTextureUsage::ShaderWrite,
            true,
            true,
            pt::synth::METALFX_TEMPORAL,
            64,
            64,
        );
        l.close().unwrap();
        let p0_reused = Retained::as_ptr(f0_reused.as_ref().expect("f0_reused")) as usize;
        assert_eq!(p0_reused, p0_a, "CommandAllocator::reset must rewind and reuse pooled MTLFence");
    }
}
