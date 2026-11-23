//! MetalFX spatial & temporal upscaling and DXR raytracing commands.

use super::opts::{barrier_masks, barrier_tight_stages, legacy_barriers};
use super::state::{Encoder, FullWhy, StageSync, COMPUTE_STAGES, PRE_RASTER};
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

const UTIL_STAGES: MTLStages = PRE_RASTER
    .union(MTLStages::Fragment)
    .union(MTLStages::Dispatch)
    .union(MTLStages::Blit)
    .union(MTLStages::AccelerationStructure);

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
        let x0 = if params.render_size[0] > 0 {
            params.render_size[0].min(st.width() as u32)
        } else {
            st.width() as u32
        };
        let y0 = if params.render_size[1] > 0 {
            params.render_size[1].min(st.height() as u32)
        } else {
            st.height() as u32
        };
        let w = if params.upscale_size[0] > 0 {
            params.upscale_size[0].min(dt.width() as u32)
        } else {
            dt.width() as u32
        };
        let h = if params.upscale_size[1] > 0 {
            params.upscale_size[1].min(dt.height() as u32)
        } else {
            dt.height() as u32
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
        if x0 == w && y0 == h {
            if touch[0] == touch[1] && s_fmt == d_fmt {
                return Ok(UpscaleBackend::BilinearCompute);
            }
            if s_fmt == d_fmt {
                self.copy_upscale_1to1(&sv, &dv, w, h, &fallback_touch, tight_stages)?;
                return Ok(UpscaleBackend::BilinearCompute);
            }
            let p = TexParams { u: [0; 4], f: [0.0; 4], x0, y0, z0: 0, w, h, d: 1, _pad: [0; 2] };
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
        let p = TexParams { u: [0; 4], f: [0.0; 4], x0, y0, z0: 0, w, h, d: 1, _pad: [0; 2] };
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

    /// Shared post-encode scratch-blit and barrier epilogue for both temporal
    /// and spatial MetalFX passes.
    #[allow(clippy::too_many_arguments)]
    fn finish_metalfx_output_blit(
        &mut self,
        scratch: Option<Retained<MtlTexture>>,
        dv: &MtlTexture,
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
                    MTLOrigin { x: 0, y: 0, z: 0 },
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
            // Keep `self.bar.needs_barrier == true` (armed by `finish_opaque_metalfx_pass`)
            // so the next encoder unconditionally waits on `MTLStages::All`.
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

        let in_w = sv.width() as u32;
        let in_h = sv.height() as u32;
        let exp_mv_w = if params.output_resolution_motion_vectors { w } else { in_w };
        let exp_mv_h = if params.output_resolution_motion_vectors { h } else { in_h };
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
        let direct_ok = dv.storageMode() == MTLStorageMode::Private
            && dv.usage().contains(scaler.output_usage)
            && w == dv.width() as u32
            && h == dv.height() as u32;
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
        let mfx_fence = self.sync_queue_before_metalfx();
        let used_fence = params.fence.or(mfx_fence.as_deref());

        let mfx_dst: &MtlTexture = scratch.as_deref().unwrap_or(dv);
        let inputs = crate::metal::metalfx::TemporalEncodeInputs {
            color: sv,
            depth: &depth_v,
            motion: &motion_v,
            output: mfx_dst,
            exposure: exp_v.as_deref(),
            reactive_mask: reactive_v.as_deref(),
            content_w: x0,
            content_h: y0,
            jitter_offset: params.jitter_offset,
            motion_vector_scale: params.motion_vector_scale,
            reset: params.reset,
            depth_reversed: params.depth_reversed,
            pre_exposure: params.pre_exposure,
            fence: used_fence,
        };
        scaler.encode(&self.cb, &inputs);
        drop(scaler);
        self.finish_opaque_metalfx_pass(used_fence);

        self.finish_metalfx_output_blit(
            scratch,
            dv,
            w,
            h,
            touch,
            tight_stages,
            "metalfx temporal scratch blit encoder",
        )?;
        Ok(true)
    }

    /// Emit a full Metal 4 queue consumer barrier + `MTLFence` update + producer barrier
    /// before an opaque `MTL4FX*Scaler` pass.
    fn sync_queue_before_metalfx(&mut self) -> Option<Retained<ProtocolObject<dyn MTLFence>>> {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(false);
        }
        if self.mfx_fence.is_none() {
            self.mfx_fence = crate::metal::device().and_then(|d| d.dev.newFence());
        }
        let pending_wait = self.pending_mfx_wait_fence.take();
        match &self.enc {
            Encoder::Compute(e) => {
                let e = e.clone();
                if let Some(wf) = pending_wait {
                    e.waitForFence_beforeEncoderStages(&wf, MTLStages::Dispatch);
                }
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Dispatch,
                    MTL4VisibilityOptions::Device,
                );
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    COMPUTE_STAGES,
                    MTLStages::Dispatch,
                    MTL4VisibilityOptions::Device,
                );
                if let Some(f) = &self.mfx_fence {
                    e.updateFence_afterEncoderStages(f, MTLStages::Dispatch);
                }
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::All,
                    MTL4VisibilityOptions::Device,
                );
                self.end_encoder();
            }
            Encoder::Render(e) => {
                let e = e.clone();
                if let Some(wf) = pending_wait {
                    e.waitForFence_beforeEncoderStages(&wf, MTLStages::Fragment);
                }
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
                if let Some(f) = &self.mfx_fence {
                    e.updateFence_afterEncoderStages(f, MTLStages::Fragment);
                }
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::All,
                    MTL4VisibilityOptions::Device,
                );
                self.end_encoder();
            }
            Encoder::None => {
                if let Some(e) = self.cb.computeCommandEncoder() {
                    if let Some(wf) = pending_wait {
                        e.waitForFence_beforeEncoderStages(&wf, MTLStages::Dispatch);
                    }
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        MTLStages::All,
                        MTLStages::Dispatch,
                        MTL4VisibilityOptions::Device,
                    );
                    if let Some(f) = &self.mfx_fence {
                        e.updateFence_afterEncoderStages(f, MTLStages::Dispatch);
                    }
                    e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                        MTLStages::All,
                        MTLStages::All,
                        MTL4VisibilityOptions::Device,
                    );
                    e.endEncoding();
                }
            }
        }
        self.mfx_fence.clone()
    }

    /// Flush any deferred post-MetalFX fence wait into a minimal fallback compute encoder.
    pub(super) fn flush_pending_mfx_wait_fence(&mut self) {
        if let Some(f) = self.pending_mfx_wait_fence.take() {
            if let Some(e) = self.cb.computeCommandEncoder() {
                e.waitForFence_beforeEncoderStages(&f, MTLStages::Dispatch);
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::Dispatch,
                    MTLStages::All,
                    MTL4VisibilityOptions::Device,
                );
                e.endEncoding();
            }
        }
    }

    /// Record that an opaque `MTL4FX*Scaler` pass ran directly on `self.cb`.
    fn finish_opaque_metalfx_pass(&mut self, fence: Option<&ProtocolObject<dyn MTLFence>>) {
        self.pending_mfx_wait_fence = fence.map(|f| f.retain());
        self.bar.has_gpu_work = true;
        self.bar.stage_sync = [StageSync::default(); 7];
        for g in &mut self.bar.stage_gen {
            *g = g.wrapping_add(1);
        }
        self.bar.uav_passthrough.clear();
        self.bar.cs_disp_passthrough.clear();
        self.need_full(FullWhy::ClearUav);
    }

    #[cfg(test)]
    pub(crate) fn metalfx_post_sync_clean(&self) -> bool {
        self.bar.needs_barrier
            && self.bar.full_why == FullWhy::ClearUav
            && self
                .bar
                .stage_sync
                .iter()
                .all(|s| !s.pre_list_all && s.gen == [0; 7])
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
        self.upscale_resource_with(dst, src, render_size, upscale_size, barrier_tight_stages())
    }

    pub(super) fn upscale_resource_with(
        &mut self,
        dst: &Resource,
        src: &Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
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
        let x0 = if render_size[0] > 0 {
            render_size[0].min(st.width() as u32)
        } else {
            st.width() as u32
        };
        let y0 = if render_size[1] > 0 {
            render_size[1].min(st.height() as u32)
        } else {
            st.height() as u32
        };
        let w = if upscale_size[0] > 0 {
            upscale_size[0].min(dt.width() as u32)
        } else {
            dt.width() as u32
        };
        let h = if upscale_size[1] > 0 {
            upscale_size[1].min(dt.height() as u32)
        } else {
            dt.height() as u32
        };
        if w == 0 || h == 0 {
            return Ok(());
        }
        if x0 == w && y0 == h {
            if touch[0] == touch[1] && s_fmt == d_fmt {
                return Ok(());
            }
            if s_fmt == d_fmt {
                return self.copy_upscale_1to1(&sv, &dv, w, h, &touch, tight_stages);
            }
            let p = TexParams { u: [0; 4], f: [0.0; 4], x0, y0, z0: 0, w, h, d: 1, _pad: [0; 2] };
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
        if self.try_upscale_metalfx(&sv, &dv, x0, y0, w, h, s_fmt, d_fmt, &touch, tight_stages)? {
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
        let p = TexParams { u: [0; 4], f: [0.0; 4], x0, y0, z0: 0, w, h, d: 1, _pad: [0; 2] };
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

    fn copy_upscale_1to1(
        &mut self,
        sv: &MtlTexture,
        dv: &MtlTexture,
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
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize { width: w as usize, height: h as usize, depth: 1 },
                dv,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
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
        let Some(scaler_arc) = crate::metal::metalfx::get_or_create_scaler(
            sv.width() as u32,
            sv.height() as u32,
            s_fmt,
            w,
            h,
            d_fmt,
        ) else {
            return Ok(false);
        };
        let mut scaler = scaler_arc.lock().unwrap();
        if !sv.usage().contains(scaler.color_usage) {
            return Ok(false);
        }
        let direct_ok = dv.usage().contains(scaler.output_usage)
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
        let mfx_fence = self.sync_queue_before_metalfx();

        let mfx_dst: &MtlTexture = scratch.as_deref().unwrap_or(dv);
        scaler.encode(&self.cb, sv, mfx_dst, x0, y0, mfx_fence.as_deref());
        drop(scaler);
        self.finish_opaque_metalfx_pass(mfx_fence.as_deref());

        self.finish_metalfx_output_blit(
            scratch,
            dv,
            w,
            h,
            touch,
            tight_stages,
            "metalfx scratch blit encoder",
        )?;
        Ok(true)
    }

    /// The utility-kernel argument table (one buffer, two textures).
    pub(super) fn ensure_util_table(&mut self) -> Result<Retained<MtlArgumentTable>, String> {
        if self.util_table.is_none() {
            let d = crate::metal::device().ok_or("no device")?;
            let desc = MTL4ArgumentTableDescriptor::new();
            desc.setMaxBufferBindCount(1);
            desc.setMaxTextureBindCount(2);
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
