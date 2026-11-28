//! Copy, blit, mipmap generation, multisample resolve, query timestamps,
//! occlusion queries, and counter resolution commands.

use super::opts::{barrier_masks, legacy_barriers, timestamp_granularity};
use super::state::{note_frag_resolve_dst, Encoder, FullWhy};
use super::CommandList;
use crate::metal::encstats as stats;
use crate::metal::passtiming as pt;
use crate::metal::resource::{MtlBuffer, MtlTexture, Resource};
use objc2::rc::Retained;
use objc2::Message;
use objc2_metal::*;

/// One side of `CopyTextureRegion` (`D3D12_TEXTURE_COPY_LOCATION`).
#[derive(Clone, Copy)]
pub enum CopyLoc<'a> {
    Subresource(&'a Resource, u32),
    Footprint(&'a Resource, crate::layout::PlacedFootprint),
}

/// `D3D12_BOX`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CopyBox {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}

/// (mip, slice, plane) of a subresource index.
pub fn split_subresource(r: &Resource, s: u32) -> (u32, u32, u32) {
    let mips = r.desc.mip_levels();
    let arr = r.desc.array_size();
    (s % mips, (s / mips) % arr, s / (mips * arr))
}

fn plane_option(tex: &MtlTexture, plane: u32) -> MTLBlitOption {
    match (tex.pixelFormat(), plane) {
        (MTLPixelFormat::Depth32Float_Stencil8, 0) => MTLBlitOption::DepthFromDepthStencil,
        (MTLPixelFormat::Depth32Float_Stencil8, _) => MTLBlitOption::StencilFromDepthStencil,
        _ => MTLBlitOption::None,
    }
}

/// Return `(src_tex, dst_tex)` with matching `MTLPixelFormat`, creating a view
/// on whichever side supports the other's format when they differ (e.g. typeless
/// or sRGB/linear casts).
fn matching_copy_textures(
    sr: &Resource,
    dr: &Resource,
) -> Result<(Retained<MtlTexture>, Retained<MtlTexture>), String> {
    let st = sr.texture().ok_or("copy source is not a texture")?;
    let dt = dr.texture().ok_or("copy destination is not a texture")?;
    if st.pixelFormat() == dt.pixelFormat() {
        return Ok((st.retain(), dt.retain()));
    }
    if let Some(dv) = dr.texture_view(
        st.pixelFormat(),
        dt.textureType(),
        (0, dt.mipmapLevelCount() as u32),
        (0, dr.desc.array_size()),
    ) {
        return Ok((st.retain(), dv));
    }
    if let Some(sv) = sr.texture_view(
        dt.pixelFormat(),
        st.textureType(),
        (0, st.mipmapLevelCount() as u32),
        (0, sr.desc.array_size()),
    ) {
        return Ok((sv, dt.retain()));
    }
    Err(format!(
        "incompatible texture formats {:?} -> {:?}",
        st.pixelFormat(),
        dt.pixelFormat()
    ))
}

impl CommandList {
    /// `CopyBufferRegion`.
    pub fn copy_buffer(
        &mut self,
        dst: &Resource,
        dst_off: u64,
        src: &Resource,
        src_off: u64,
        n: u64,
    ) -> Result<(), String> {
        let (Some(d), Some(s)) = (dst.buffer(), src.buffer()) else {
            return Err("CopyBufferRegion on a non-buffer".into());
        };
        if n == 0 {
            return Ok(());
        }
        dst.register_ptr();
        src.register_ptr();
        if d.storageMode() != MTLStorageMode::Shared && dst.cpu_ptr().is_none() {
            if let Some(bytes) = src.cpu_bytes(src_off, n as usize) {
                dst.record_cpu_copy(dst_off, &bytes);
            }
        }
        let touch = [dst as *const Resource as usize, src as *const Resource as usize];
        self.flush_clears_for(&touch)?;
        let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        unsafe {
            e.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                s,
                src_off as usize,
                d,
                dst_off as usize,
                n as usize,
            )
        };
        self.note_stage_ran(MTLStages::Blit);
        Ok(())
    }

    /// Write a single 32-bit value into `dst` at 4-byte-aligned `dst_off` on the
    /// GPU timeline using the command allocator's transient upload ring.
    pub fn write_buffer_u32(
        &mut self,
        dst: &Resource,
        dst_off: u64,
        value: u32,
    ) -> Result<(), String> {
        let Some(d) = dst.buffer() else {
            return Err("write_buffer_u32 on a non-buffer".into());
        };
        if dst_off & 3 != 0 || dst_off.saturating_add(4) > dst.desc.Width {
            return Err(format!(
                "write_buffer_u32: offset {dst_off}+4 out of bounds ({})",
                dst.desc.Width
            ));
        }
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let (src_buf, src_off) = {
            let mut r = ring.lock().unwrap();
            let (b, off) = r.alloc_buffer(4, 4)?;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    value.to_le_bytes().as_ptr(),
                    (b.contents().as_ptr() as *mut u8).add(off),
                    4,
                );
            }
            (b, off)
        };
        dst.register_ptr();
        dst.mark_gpu_used();
        let touch = [dst as *const Resource as usize];
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;
        let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        unsafe {
            e.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                &src_buf,
                src_off,
                d,
                dst_off as usize,
                4,
            )
        };
        self.note_stage_ran(MTLStages::Blit);
        self.produced(MTLStages::Blit, MTLStages::All);
        Ok(())
    }

    /// `CopyTextureRegion`: footprint↔texture and texture↔texture copies.
    pub fn copy_texture_region(
        &mut self,
        dst: CopyLoc,
        dst_xyz: [u32; 3],
        src: CopyLoc,
        bx: Option<CopyBox>,
    ) -> Result<(), String> {
        let (CopyLoc::Subresource(d, _) | CopyLoc::Footprint(d, _)) = dst;
        let (CopyLoc::Subresource(s, _) | CopyLoc::Footprint(s, _)) = src;
        d.register_ptr();
        s.register_ptr();
        let touch = [d as *const Resource as usize, s as *const Resource as usize];
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;
        let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        let need_produced = !self.ib_tight_active()
            || (d.desc.Flags & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS != 0);
        let o = |x: u32, y: u32, z: u32| MTLOrigin { x: x as usize, y: y as usize, z: z as usize };
        match (src, dst) {
            (CopyLoc::Footprint(b, fp), CopyLoc::Subresource(t, s)) => {
                let buf = b.buffer().ok_or("footprint source is not a buffer")?;
                let tex = t.texture().ok_or("copy destination is not a texture")?;
                let (mip, slice, plane) = split_subresource(t, s);
                let fi = crate::format::info(fp.format);
                let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
                let bpe = crate::format::info(crate::layout::plane_format(t.desc.Format, plane))
                    .bytes
                    .max(fi.bytes)
                    .max(1) as u64;
                let b0 = bx.unwrap_or(CopyBox {
                    left: 0,
                    top: 0,
                    front: 0,
                    right: fp.width,
                    bottom: fp.height,
                    back: fp.depth,
                });
                if b0.right <= b0.left || b0.bottom <= b0.top || b0.back <= b0.front {
                    return Ok(());
                }
                let rows = fp.height.div_ceil(bh) as u64;
                let image = fp.row_pitch as u64 * rows;
                let off = fp.offset
                    + b0.front as u64 * image
                    + (b0.top / bh) as u64 * fp.row_pitch as u64
                    + (b0.left / bw) as u64 * bpe;
                let mw = (tex.width() >> mip).max(1) as u32;
                let mh = (tex.height() >> mip).max(1) as u32;
                let md = if tex.textureType() == MTLTextureType::Type3D {
                    (tex.depth() >> mip).max(1) as u32
                } else {
                    1
                };
                let w = (b0.right - b0.left).min(mw.saturating_sub(dst_xyz[0]));
                let h = (b0.bottom - b0.top).min(mh.saturating_sub(dst_xyz[1]));
                let d = (b0.back - b0.front).min(md.saturating_sub(dst_xyz[2]));
                if w == 0 || h == 0 || d == 0 {
                    return Ok(());
                }
                let copy_rows = h.div_ceil(bh) as u64;
                let src_image = if d <= 1 { fp.row_pitch as u64 * copy_rows } else { image };
                unsafe {
                    e.copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
                        buf,
                        off as usize,
                        fp.row_pitch as usize,
                        src_image as usize,
                        MTLSize { width: w as usize, height: h as usize, depth: d as usize },
                        tex,
                        slice as usize,
                        mip as usize,
                        o(dst_xyz[0], dst_xyz[1], dst_xyz[2]),
                        plane_option(tex, plane),
                    )
                };
            }
            (CopyLoc::Subresource(t, s), CopyLoc::Footprint(b, fp)) => {
                let buf = b.buffer().ok_or("footprint destination is not a buffer")?;
                let tex = t.texture().ok_or("copy source is not a texture")?;
                let (mip, slice, plane) = split_subresource(t, s);
                let fi = crate::format::info(fp.format);
                let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
                let bpe = crate::format::info(crate::layout::plane_format(t.desc.Format, plane))
                    .bytes
                    .max(fi.bytes)
                    .max(1) as u64;
                let mw = (tex.width() >> mip).max(1) as u32;
                let mh = (tex.height() >> mip).max(1) as u32;
                let md = if tex.textureType() == MTLTextureType::Type3D {
                    (tex.depth() >> mip).max(1) as u32
                } else {
                    1
                };
                let b0 = bx.unwrap_or(CopyBox {
                    left: 0,
                    top: 0,
                    front: 0,
                    right: mw,
                    bottom: mh,
                    back: md,
                });
                if b0.right <= b0.left || b0.bottom <= b0.top || b0.back <= b0.front {
                    return Ok(());
                }
                let rows = fp.height.div_ceil(bh) as u64;
                let image = fp.row_pitch as u64 * rows;
                let off = fp.offset
                    + dst_xyz[2] as u64 * image
                    + (dst_xyz[1] / bh) as u64 * fp.row_pitch as u64
                    + (dst_xyz[0] / bw) as u64 * bpe;
                let w = b0.right.min(mw).saturating_sub(b0.left);
                let h = b0.bottom.min(mh).saturating_sub(b0.top);
                let d = b0.back.min(md).saturating_sub(b0.front);
                if w == 0 || h == 0 || d == 0 {
                    return Ok(());
                }
                let copy_rows = h.div_ceil(bh) as u64;
                let dst_image = if d <= 1 { fp.row_pitch as u64 * copy_rows } else { image };
                unsafe {
                    e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
                        tex,
                        slice as usize,
                        mip as usize,
                        o(b0.left, b0.top, b0.front),
                        MTLSize { width: w as usize, height: h as usize, depth: d as usize },
                        buf,
                        off as usize,
                        fp.row_pitch as usize,
                        dst_image as usize,
                        plane_option(tex, plane),
                    )
                };
            }
            (CopyLoc::Subresource(st, ss), CopyLoc::Subresource(dt, ds)) => {
                let stex = st.texture().ok_or("copy source is not a texture")?;
                let dtex = dt.texture().ok_or("copy destination is not a texture")?;
                let (smip, sslice, splane) = split_subresource(st, ss);
                let (dmip, dslice, dplane) = split_subresource(dt, ds);
                let mw = (stex.width() >> smip).max(1) as u32;
                let mh = (stex.height() >> smip).max(1) as u32;
                let md = if stex.textureType() == MTLTextureType::Type3D {
                    (stex.depth() >> smip).max(1) as u32
                } else {
                    1
                };
                let dmw = (dtex.width() >> dmip).max(1) as u32;
                let dmh = (dtex.height() >> dmip).max(1) as u32;
                let dmd = if dtex.textureType() == MTLTextureType::Type3D {
                    (dtex.depth() >> dmip).max(1) as u32
                } else {
                    1
                };
                let b0 = bx.unwrap_or(CopyBox {
                    left: 0,
                    top: 0,
                    front: 0,
                    right: mw,
                    bottom: mh,
                    back: md,
                });
                if b0.right <= b0.left || b0.bottom <= b0.top || b0.back <= b0.front {
                    return Ok(());
                }
                let w = b0
                    .right
                    .min(mw)
                    .saturating_sub(b0.left)
                    .min(dmw.saturating_sub(dst_xyz[0]));
                let h = b0
                    .bottom
                    .min(mh)
                    .saturating_sub(b0.top)
                    .min(dmh.saturating_sub(dst_xyz[1]));
                let d = b0
                    .back
                    .min(md)
                    .saturating_sub(b0.front)
                    .min(dmd.saturating_sub(dst_xyz[2]));
                if w == 0 || h == 0 || d == 0 {
                    return Ok(());
                }
                if stex.pixelFormat() == MTLPixelFormat::Depth32Float_Stencil8
                    || dtex.pixelFormat() == MTLPixelFormat::Depth32Float_Stencil8
                {
                    let bpe = if stex.pixelFormat() == MTLPixelFormat::Depth32Float_Stencil8 {
                        if splane == 0 { 4usize } else { 1usize }
                    } else {
                        crate::format::info(crate::layout::plane_format(st.desc.Format, splane))
                            .bytes
                            .max(1) as usize
                    };
                    let row_pitch = (w as usize * bpe).next_multiple_of(256);
                    let img_pitch = row_pitch * h as usize;
                    let ring = self.ring.clone().ok_or("command list is not open")?;
                    let (tmp_buf, tmp_off) =
                        ring.lock().unwrap().alloc_buffer(img_pitch * d as usize, 256)?;
                    unsafe {
                        e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
                            stex,
                            sslice as usize,
                            smip as usize,
                            o(b0.left, b0.top, b0.front),
                            MTLSize { width: w as usize, height: h as usize, depth: d as usize },
                            &tmp_buf,
                            tmp_off,
                            row_pitch,
                            img_pitch,
                            plane_option(stex, splane),
                        );
                        e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                            MTLStages::Blit,
                            MTLStages::Blit,
                            MTL4VisibilityOptions::Device,
                        );
                        e.copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
                            &tmp_buf,
                            tmp_off,
                            row_pitch,
                            img_pitch,
                            MTLSize { width: w as usize, height: h as usize, depth: d as usize },
                            dtex,
                            dslice as usize,
                            dmip as usize,
                            o(dst_xyz[0], dst_xyz[1], dst_xyz[2]),
                            plane_option(dtex, dplane),
                        );
                    }
                    self.note_stage_ran(MTLStages::Blit);
                    if need_produced {
                        self.produced(MTLStages::Blit, MTLStages::All);
                    } else if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
                        self.ib.ib_blit_dst.push(touch[0]);
                    }
                    return Ok(());
                }
                let (sv, dv) = matching_copy_textures(st, dt)?;
                unsafe {
                    e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                        &sv,
                        sslice as usize,
                        smip as usize,
                        o(b0.left, b0.top, b0.front),
                        MTLSize { width: w as usize, height: h as usize, depth: d as usize },
                        &dv,
                        dslice as usize,
                        dmip as usize,
                        o(dst_xyz[0], dst_xyz[1], dst_xyz[2]),
                    )
                };
            }
            (CopyLoc::Footprint(..), CopyLoc::Footprint(..)) => {
                return Err("footprint to footprint copy".into());
            }
        }
        self.note_stage_ran(MTLStages::Blit);
        if need_produced {
            self.produced(MTLStages::Blit, MTLStages::All);
        } else if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
            self.ib.ib_blit_dst.push(touch[0]);
        }
        Ok(())
    }

    /// `CopyResource`: whole-resource copy (buffers or all subresources).
    pub fn copy_resource(&mut self, dst: &Resource, src: &Resource) -> Result<(), String> {
        dst.register_ptr();
        src.register_ptr();
        let touch = [dst as *const Resource as usize, src as *const Resource as usize];
        self.disc_unmark(touch[0]);
        self.flush_clears_for(&touch)?;
        let need_produced = !self.ib_tight_active()
            || (dst.desc.Flags & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS != 0);
        if let (Some(d), Some(s)) = (dst.buffer(), src.buffer()) {
            let n = d.length().min(s.length());
            if dst.cpu_ptr().is_none() {
                if let Some(bytes) = src.cpu_bytes(0, n) {
                    dst.record_cpu_copy(0, &bytes);
                }
            }
            let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
            self.pt_note_util(pt::synth::COPY_BLIT, 0);
            unsafe {
                e.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(s, 0, d, 0, n)
            };
            self.note_stage_ran(MTLStages::Blit);
            if need_produced {
                self.produced(MTLStages::Blit, MTLStages::All);
            } else if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
                self.ib.ib_blit_dst.push(touch[0]);
            }
            return Ok(());
        }
        if dst.texture().is_none() || src.texture().is_none() {
            return Err("CopyResource between a buffer and a texture".into());
        }
        let (sv, dv) = matching_copy_textures(src, dst)?;
        let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        unsafe { e.copyFromTexture_toTexture(&sv, &dv) };
        self.note_stage_ran(MTLStages::Blit);
        if need_produced {
            self.produced(MTLStages::Blit, MTLStages::All);
        } else if touch[0] != 0 && !self.ib.ib_blit_dst.contains(&touch[0]) {
            self.ib.ib_blit_dst.push(touch[0]);
        }
        Ok(())
    }

    /// Generate a mipmap chain on the GPU via Metal 4 `generateMipmapsForTexture:`.
    pub fn generate_mips(
        &mut self,
        r: &Resource,
        view_dxgi: u32,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_slice: u32,
        slice_count: u32,
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        r.register_ptr();
        r.mark_gpu_used();
        let rk = r as *const Resource as usize;
        self.disc_unmark(rk);
        self.flush_clears_for(&[rk])?;

        let t = r.texture().ok_or("generate_mips: resource is not a texture")?;
        if t.sampleCount() > 1 {
            return Err("generate_mips: multisampled texture not supported".into());
        }
        let total_mips = r.desc.mip_levels();
        if most_detailed_mip >= total_mips {
            return Ok(());
        }
        let mips = if mip_levels == 0 || mip_levels == u32::MAX {
            total_mips - most_detailed_mip
        } else {
            mip_levels.min(total_mips - most_detailed_mip)
        };
        if mips <= 1 {
            return Ok(());
        }

        let eff_dxgi = if view_dxgi != 0 { view_dxgi } else { r.desc.Format };
        let fi = crate::format::info(eff_dxgi);
        if fi.is_depth_stencil()
            || fi.block_w > 1
            || fi.block_h > 1
            || matches!(fi.num, crate::format::Num::Uint | crate::format::Num::Sint)
        {
            return Err(format!("generate_mips: unsupported format {}", fi.name));
        }
        let fmt = crate::format::view_format(eff_dxgi, r.mtl_format);
        if fmt == MTLPixelFormat::Invalid {
            return Err(format!("generate_mips: invalid Metal format for {}", fi.name));
        }

        let ty = t.textureType();
        let slices = match ty {
            MTLTextureType::Type1DArray
            | MTLTextureType::Type2DArray
            | MTLTextureType::TypeCube
            | MTLTextureType::TypeCubeArray => {
                let total_slices = r.desc.array_size();
                if first_slice >= total_slices {
                    return Ok(());
                }
                let sc = if slice_count == 0 || slice_count == u32::MAX {
                    total_slices - first_slice
                } else {
                    slice_count.min(total_slices - first_slice)
                };
                if sc == 0 {
                    return Ok(());
                }
                (first_slice, sc)
            }
            MTLTextureType::Type1D | MTLTextureType::Type2D | MTLTextureType::Type3D => (0, 1),
            other => {
                return Err(format!("generate_mips: unsupported Metal texture type {other:?}"));
            }
        };

        let view = r
            .texture_view(fmt, ty, (most_detailed_mip, mips), slices)
            .ok_or("generate_mips: texture view creation failed")?;
        if view.mipmapLevelCount() <= 1 {
            return Ok(());
        }

        let touch = [rk];
        let e = self.compute_blit_touch(&touch).ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        unsafe { e.generateMipmapsForTexture(&view) };
        self.note_stage_ran(MTLStages::Blit);
        self.produced(MTLStages::Blit, MTLStages::All);
        Ok(())
    }

    /// `ResolveSubresource(dst, dst_sub, src, src_sub, format)`: an empty
    /// render pass that loads the multisampled `src` and stores with
    /// `MultisampleResolve` (average) into `dst`, both viewed as `format`.
    pub fn resolve_subresource(
        &mut self,
        dst: &Resource,
        dst_sub: u32,
        src: &Resource,
        src_sub: u32,
        dxgi: u32,
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        dst.register_ptr();
        src.register_ptr();
        dst.mark_gpu_used();
        src.mark_gpu_used();
        self.disc_unmark(dst as *const Resource as usize);
        note_frag_resolve_dst(dst as *const Resource as usize);
        let (st, dt) = (
            src.texture().ok_or("resolve source is not a texture")?,
            dst.texture().ok_or("resolve destination is not a texture")?,
        );
        if st.sampleCount() <= 1 {
            return Err("resolve source is not multisampled".into());
        }
        let fi = crate::format::info(dxgi);
        if matches!(fi.num, crate::format::Num::Uint | crate::format::Num::Sint)
            || fi.planes > 1
            || crate::format::info(src.desc.Format).is_depth_stencil()
        {
            return Err(format!(
                "resolve of format {} (D3D12 requires a float/norm color format)",
                fi.name
            ));
        }
        let (sm, ss, _) = split_subresource(src, src_sub);
        let (dm, ds, _) = split_subresource(dst, dst_sub);
        let sv = src
            .texture_view(
                crate::format::view_format(dxgi, src.mtl_format),
                st.textureType(),
                (0, st.mipmapLevelCount() as u32),
                (0, src.desc.array_size()),
            )
            .ok_or("resolve source view")?;
        let dv = dst
            .texture_view(
                crate::format::view_format(dxgi, dst.mtl_format),
                dt.textureType(),
                (0, dt.mipmapLevelCount() as u32),
                (0, dst.desc.array_size()),
            )
            .ok_or("resolve destination view")?;
        let rp = MTL4RenderPassDescriptor::new();
        let ca = unsafe { rp.colorAttachments().objectAtIndexedSubscript(0) };
        ca.setTexture(Some(&sv));
        ca.setLevel(sm as usize);
        ca.setSlice(ss as usize);
        ca.setLoadAction(MTLLoadAction::Load);
        ca.setStoreAction(MTLStoreAction::MultisampleResolve);
        ca.setResolveTexture(Some(&dv));
        ca.setResolveLevel(dm as usize);
        if dv.textureType() == MTLTextureType::Type3D {
            ca.setResolveDepthPlane(ds as usize);
        } else {
            ca.setResolveSlice(ds as usize);
        }
        let targets = [src as *const Resource as usize, dst as *const Resource as usize];
        self.flush_clears(&stats::c::cf_resolve, |p| targets.contains(&p.view.resource))?;
        stats::inc(&stats::c::util_pass);
        self.util_render_pass(&rp, &targets, pt::kind::UTIL)
            .ok_or("render encoder")?;
        if let Some(m) = self.pt_cur() {
            (m.w, m.h) = ((dt.width() >> dm).max(1) as u32, (dt.height() >> dm).max(1) as u32);
            m.shader = pt::synth::RESOLVE;
        }
        self.end_encoder();
        Ok(())
    }

    /// Write a GPU timestamp at the current point of the list.
    pub(crate) fn timestamp_into(&mut self, h: &crate::metal::query::MtlCounterHeap, index: usize) {
        self.bar.has_gpu_work = true;
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        // Apple AGX silently drops the 5th+ `writeTimestampWithGranularity` call inside a
        // single encoder (and drops timestamps inside an empty compute encoder) to 0.
        // Reserve 2 slots for `passtiming` (open + close) when active; overflow safely
        // closes the encoder and uses command-buffer-level `writeTimestampIntoHeap`, which
        // has no per-encoder cap.
        let max_enc_ts: u8 = if self.pt.is_some() { 2 } else { 4 };
        let must_close = match &self.enc {
            Encoder::Render(_) => self.enc_ts_count >= max_enc_ts,
            Encoder::Compute(_) => self.enc_ts_count >= max_enc_ts || self.bar.enc_ran_stages.is_empty(),
            Encoder::None => false,
        };
        if must_close {
            self.end_encoder();
        }
        unsafe {
            match &self.enc {
                Encoder::Render(e) => {
                    e.writeTimestampWithGranularity_afterStage_intoHeap_atIndex(
                        timestamp_granularity(),
                        MTLRenderStages::Fragment,
                        h,
                        index,
                    );
                    self.enc_ts_count = self.enc_ts_count.saturating_add(1);
                }
                Encoder::Compute(e) => {
                    e.writeTimestampWithGranularity_intoHeap_atIndex(timestamp_granularity(), h, index);
                    self.enc_ts_count = self.enc_ts_count.saturating_add(1);
                }
                Encoder::None => self.cb.writeTimestampIntoHeap_atIndex(h, index),
            }
        }
    }

    /// Resolve counter heap entries into GPU memory (outside any encoder);
    /// later work sees the data after a barrier.
    pub(crate) fn resolve_counters(
        &mut self,
        h: &crate::metal::query::MtlCounterHeap,
        range: objc2_foundation::NSRange,
        dst: MTL4BufferRange,
    ) {
        self.bar.has_gpu_work = true;
        stats::inc(&stats::c::q_resolve);
        self.end_encoder();
        unsafe {
            self.cb
                .resolveCounterHeap_withRange_intoBuffer_waitFence_updateFence(
                    h, range, dst, None, None,
                )
        };
        self.need_full(FullWhy::Internal);
    }

    /// Flush all pending staged GPU timestamps from the shared scratch
    /// `MTL4CounterHeap` ring into their `QueryHeap` backing `MTLBuffer`s.
    pub(crate) fn flush_staged_timestamps(&mut self) {
        if self.pending_staged_ts.is_empty() {
            return;
        }
        let Ok(pool) = crate::metal::query::timestamp_scratch_pool() else {
            self.pending_staged_ts.clear();
            return;
        };
        // Ensure any in-flight encoder work writing timestamps (or reading a timestamp
        // backing buffer in Blit) completes before `resolveCounterHeap` (MTLStages::Blit).
        match &self.enc {
            Encoder::Render(e) => {
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Blit,
                    MTL4VisibilityOptions::Device,
                );
            }
            Encoder::Compute(e) => {
                self.bar.ts_buf_blit_read = false;
                e.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Blit,
                    MTL4VisibilityOptions::Device,
                );
            }
            Encoder::None => {}
        }
        self.end_encoder();
        self.bar.has_gpu_work = true;
        let pending = std::mem::take(&mut self.pending_staged_ts);
        let mut i = 0;
        while i < pending.len() {
            let first = &pending[i];
            let mut run_len = 1usize;
            while i + run_len < pending.len() {
                let next = &pending[i + run_len];
                if next.heap_idx == first.heap_idx
                    && next.scratch_slot as usize == first.scratch_slot as usize + run_len
                    && next.dst_gpu_va == first.dst_gpu_va + (run_len as u64) * 8
                {
                    run_len += 1;
                } else {
                    break;
                }
            }
            let h = pool.heap(first.heap_idx as usize);
            let range = objc2_foundation::NSRange::new(first.scratch_slot as usize, run_len);
            let dst = MTL4BufferRange {
                bufferAddress: first.dst_gpu_va,
                length: (run_len as u64) * 8,
            };
            stats::inc(&stats::c::q_resolve);
            unsafe {
                self.cb
                    .resolveCounterHeap_withRange_intoBuffer_waitFence_updateFence(
                        h, range, dst, None, None,
                    );
            }
            i += run_len;
        }
        self.need_full(FullWhy::Internal);
    }

    /// Arm (`Some`, resetting its slot) or disarm (`None`) the occlusion query that
    /// render passes count into.
    ///
    /// Under stage-masked barriers, consecutive occlusion queries on the same
    /// `visibilityResultBuffer` reuse the open `MTLVisibilityResultType::Reset`
    /// render pass without closing the encoder, running a `fillBuffer` compute pass,
    /// or emitting a `FullWhy::Internal` (`All -> All`) barrier. If a query closes
    /// with zero draws and no render pass opened for it, a Blit-scoped `fillBuffer`
    /// zeroes that single 8-byte slot with stage-masked `Blit -> Fragment | Blit`.
    pub(crate) fn set_occlusion(
        &mut self,
        o: Option<crate::metal::query::ActiveOcclusion>,
    ) -> Result<(), String> {
        let masked = !legacy_barriers() && barrier_masks();
        match o {
            Some(q) => {
                stats::inc(&stats::c::occl_begin);
                crate::metal::pipeline::note_frag_buf_write();
                if masked {
                    if !self.icb.icb_segs.is_empty() {
                        self.flush_draw_batch(true);
                    }
                    let q_buf_ptr = &*q.buffer as *const MtlBuffer as usize;
                    let mode = if q.binary {
                        MTLVisibilityResultMode::Boolean
                    } else {
                        MTLVisibilityResultMode::Counting
                    };
                    let had_blit_read = self.bar.vis_buf_blit_read.contains(&q_buf_ptr);
                    let had_cs_read = self.bar.vis_buf_cs_read.contains(&q_buf_ptr);
                    let slot_previously_written =
                        self.bar.vis_buf_written.contains(&(q_buf_ptr, q.index));
                    // Reuse the open `Reset` render pass when it already has this visibility
                    // buffer attached, targets have not changed, no resolve read intervened, and
                    // this query slot has neither been written in an earlier pass nor used in
                    // the current pass (reusing a slot written in an earlier pass on another
                    // target requires closing and emitting `Fragment -> Fragment` sync).
                    if !had_blit_read
                        && !had_cs_read
                        && !slot_previously_written
                        && self.enc_vis_buf == Some(q_buf_ptr)
                        && self.enc_vis_reset
                        && !self.gfx.had_target_rebind
                        && !self.enc_vis_used_indices.contains(&q.index)
                    {
                        if let Encoder::Render(e) = &self.enc {
                            unsafe { e.setVisibilityResultMode_offset(mode, q.index as usize * 8) };
                            self.enc_vis_used_indices.push(q.index);
                            if !self.bar.vis_buf_written.contains(&(q_buf_ptr, q.index)) {
                                self.bar.vis_buf_written.push((q_buf_ptr, q.index));
                            }
                            self.occl_drew = true;
                            self.occlusion = Some(q);
                            return Ok(());
                        }
                    }
                    // Otherwise open a new render pass on the next draw with `Reset`.
                    if matches!(self.enc, Encoder::Render(_)) {
                        self.end_encoder();
                    }
                    // Preserve `vis_buf_{blit,cs}_read` until either a render pass actually opens
                    // and consumes `Blit|Dispatch -> PRE_RASTER | Fragment`, or a 0-draw `EndQuery`
                    // consumes it with a `Blit|Dispatch -> Blit` barrier before zero-filling the slot.
                    // Wait before `PRE_RASTER | Fragment` (not `Fragment` alone) so the next
                    // render pass's `MTLVisibilityResultType::Reset` prologue cannot race an
                    // earlier pass's `Fragment` write or `ResolveQueryData`'s read.
                    if had_blit_read {
                        self.add_pend(
                            MTLStages::Blit,
                            super::state::PRE_RASTER | MTLStages::Fragment,
                        );
                    }
                    if had_cs_read {
                        self.add_pend(
                            MTLStages::Dispatch,
                            super::state::PRE_RASTER | MTLStages::Fragment,
                        );
                    }
                    if slot_previously_written {
                        self.add_pend(
                            MTLStages::Fragment,
                            super::state::PRE_RASTER | MTLStages::Fragment,
                        );
                    }
                    self.occl_drew = false;
                    self.occlusion = Some(q);
                    return Ok(());
                }
                // Legacy barrier mode: close render pass, zero via Blit, arm full barrier.
                if !self.icb.icb_segs.is_empty() {
                    self.flush_draw_batch(false);
                }
                if let Encoder::Render(_) = self.enc {
                    self.end_encoder();
                }
                let e = self.compute().ok_or("compute encoder")?;
                self.pt_note_util(pt::synth::FILL_OCCL, 0);
                unsafe {
                    e.fillBuffer_range_value(
                        &q.buffer,
                        objc2_foundation::NSRange::new(q.index as usize * 8, 8),
                        0,
                    )
                };
                self.note_stage_ran(MTLStages::Blit);
                self.need_full(FullWhy::Internal);
                self.occl_drew = true;
                self.occlusion = Some(q);
            }
            None => {
                if !self.icb.icb_segs.is_empty() {
                    self.flush_draw_batch(true);
                }
                let prev = self.occlusion.take();
                let drew = std::mem::replace(&mut self.occl_drew, false);
                if self.enc_vis_buf.is_some() {
                    if let Encoder::Render(e) = &self.enc {
                        // Pass the ended query's own offset rather than 0: in a
                        // `MTLVisibilityResultType::Reset` pass, AGX zeroes whatever offset
                        // is touched by `setVisibilityResultMode:offset:`, so passing 0 would
                        // clobber slot 0 whenever any non-zero slot ends.
                        let off = prev.as_ref().map_or(0, |q| q.index as usize * 8);
                        unsafe {
                            e.setVisibilityResultMode_offset(MTLVisibilityResultMode::Disabled, off)
                        };
                    }
                } else if masked && !drew {
                    // `BeginQuery..EndQuery` completed with zero draws and no render pass ever
                    // opened for it: zero the 8-byte slot in `Blit` with stage-masked sync.
                    if let Some(prev_q) = prev {
                        let q_buf_ptr = &*prev_q.buffer as *const MtlBuffer as usize;
                        if self.bar.vis_buf_written.contains(&(q_buf_ptr, prev_q.index)) {
                            self.add_pend(MTLStages::Fragment, MTLStages::Blit);
                        }
                        if self.bar.vis_buf_blit_read.contains(&q_buf_ptr) {
                            self.bar.vis_buf_blit_read.retain(|&p| p != q_buf_ptr);
                            self.add_pend(MTLStages::Blit, MTLStages::Blit);
                            if matches!(self.enc, Encoder::Compute(_)) {
                                if self.ib_tight_active() {
                                    self.ib_add_obl_tight(0, MTLStages::Blit, MTLStages::Blit);
                                } else {
                                    self.ib_note(0);
                                }
                            }
                        }
                        if self.bar.vis_buf_cs_read.contains(&q_buf_ptr) {
                            self.bar.vis_buf_cs_read.retain(|&p| p != q_buf_ptr);
                            self.add_pend(MTLStages::Dispatch, MTLStages::Blit);
                            if matches!(self.enc, Encoder::Compute(_)) {
                                if self.ib_tight_active() {
                                    self.ib_add_obl_tight(0, MTLStages::Dispatch, MTLStages::Blit);
                                } else {
                                    self.ib_note(0);
                                }
                            }
                        }
                        let e = self.compute_blit_touch(&[]).ok_or("compute encoder")?;
                        self.pt_note_util(pt::synth::FILL_OCCL, 0);
                        unsafe {
                            e.fillBuffer_range_value(
                                &prev_q.buffer,
                                objc2_foundation::NSRange::new(prev_q.index as usize * 8, 8),
                                0,
                            )
                        };
                        self.note_stage_ran(MTLStages::Blit);
                        if self.ib_tight_active() {
                            self.ib_add_obl_tight(0, MTLStages::Blit, MTLStages::Blit);
                        } else {
                            self.ib_note(0);
                        }
                        self.add_pend(MTLStages::Blit, MTLStages::Fragment | MTLStages::Blit);
                    }
                }
            }
        }
        Ok(())
    }

    fn copy_query_buffer(
        &mut self,
        src: &MtlBuffer,
        src_off: u64,
        dst: &Resource,
        dst_off: u64,
        n: u64,
        is_vis: bool,
    ) -> Result<(), String> {
        if n == 0 {
            return Ok(());
        }
        let d = dst.buffer().ok_or("resolve destination is not a buffer")?;
        dst.register_ptr();
        let dst_id = dst as *const Resource as usize;
        let src_ptr = src as *const MtlBuffer as usize;
        if !legacy_barriers() && barrier_masks() {
            self.flush_clears_for(&[dst_id])?;
            if matches!(self.enc, Encoder::Render(_)) {
                self.end_encoder();
            }
            self.add_pend(MTLStages::Fragment | MTLStages::Blit, MTLStages::Blit);
            // Include `0` alongside `dst_id` because `src` is a query backing `MtlBuffer`
            // (not a tracked `Resource`) written by `Fragment` or `Blit`, potentially in an
            // earlier list; this prevents destination-only list-start `READBACK` narrowing from
            // dropping cross-list synchronization.
            let e = self.compute_blit_touch(&[dst_id, 0]).ok_or("compute encoder")?;
            self.pt_note_util(pt::synth::COPY_BLIT, 0);
            unsafe {
                e.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                    src,
                    src_off as usize,
                    d,
                    dst_off as usize,
                    n as usize,
                )
            };
            self.note_stage_ran(MTLStages::Blit);
            if is_vis {
                if !self.bar.vis_buf_blit_read.contains(&src_ptr) {
                    self.bar.vis_buf_blit_read.push(src_ptr);
                }
            } else {
                self.bar.ts_buf_blit_read = true;
            }
            return Ok(());
        }
        self.end_encoder();
        self.need_full(FullWhy::Internal);
        let e = self.compute().ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::COPY_BLIT, 0);
        unsafe {
            e.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                src,
                src_off as usize,
                d,
                dst_off as usize,
                n as usize,
            )
        };
        self.note_stage_ran(MTLStages::Blit);
        if !is_vis {
            self.bar.ts_buf_blit_read = true;
        }
        Ok(())
    }

    /// Buffer copy ordered after render-pass visibility results (`ResolveQueryData(OCCLUSION)`).
    pub(crate) fn copy_after_render(
        &mut self,
        src: &MtlBuffer,
        src_off: u64,
        dst: &Resource,
        dst_off: u64,
        n: u64,
    ) -> Result<(), String> {
        self.copy_query_buffer(src, src_off, dst, dst_off, n, true)
    }

    /// Buffer copy from a staged timestamp backing buffer (`ResolveQueryData(TIMESTAMP)`).
    pub(crate) fn copy_timestamp_buffer(
        &mut self,
        src: &MtlBuffer,
        src_off: u64,
        dst: &Resource,
        dst_off: u64,
        n: u64,
    ) -> Result<(), String> {
        self.copy_query_buffer(src, src_off, dst, dst_off, n, false)
    }

    /// Resolve binary occlusion slots (`ResolveQueryData(BINARY_OCCLUSION)`), normalizing
    /// each 64-bit visibility slot on the GPU to strictly `0u64` or `1u64`.
    pub(crate) fn resolve_binary_after_render(
        &mut self,
        src: &MtlBuffer,
        src_off: u64,
        dst: &Resource,
        dst_off: u64,
        count: u32,
    ) -> Result<(), String> {
        if count == 0 {
            return Ok(());
        }
        if dst_off % 4 != 0 {
            return Err(format!(
                "binary occlusion resolve destination offset {dst_off} must be at least 4-byte aligned"
            ));
        }
        let d = dst.buffer().ok_or("resolve destination is not a buffer")?;
        dst.register_ptr();
        dst.mark_gpu_used();
        let dst_id = dst as *const Resource as usize;
        let src_ptr = src as *const MtlBuffer as usize;
        self.flush_clears_for(&[dst_id])?;
        let p = crate::metal::clear::BinaryOcclResolveParams {
            src: src.gpuAddress() + src_off,
            dst: d.gpuAddress() + dst_off,
            count,
            _pad: 0,
        };
        // Include `0` alongside `dst_id` because `src` is a visibility-result `MtlBuffer`
        // (not a tracked `Resource`) written by `Fragment`/`Blit`, potentially in an earlier list.
        self.util_dispatch_with(
            "port_resolve_binary_occl",
            None,
            None,
            crate::metal::clear::bytes_of(&p),
            [count, 1, 1],
            &[dst_id, 0],
            true,
        )?;
        if !legacy_barriers() && barrier_masks() {
            // Ensure subsequent render-pass `Reset` or 0-draw `fillBuffer` on `src` waits on this
            // compute read, even when `opt_cs_to_ps_overlap` narrows `util_dispatch_with`'s post-sync.
            self.add_pend(
                MTLStages::Dispatch,
                super::state::PRE_RASTER | MTLStages::Fragment | MTLStages::Blit,
            );
            if !self.bar.vis_buf_cs_read.contains(&src_ptr) {
                self.bar.vis_buf_cs_read.push(src_ptr);
            }
        }
        Ok(())
    }
}
