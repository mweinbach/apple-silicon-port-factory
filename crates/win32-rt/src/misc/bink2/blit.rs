//! Planar YCrCb(A/H) buffer clearing and CPU pixel-format conversion blitter
//! (`BinkCopyToBuffer` / `BinkCopyToBufferRect`) for `bink2w64.dll`.

use super::player::BinkInstance;
use super::types::{
    BinkFrameBuffers, BINKSURFACE24, BINKSURFACE24R, BINKSURFACE32, BINKSURFACE32A, BINKSURFACE32R,
    BINKSURFACE32RA, BINKSURFACE4444, BINKSURFACE555, BINKSURFACE5551, BINKSURFACE555R,
    BINKSURFACE565, BINKSURFACE565R, BINKSURFACE655, BINKSURFACE664, BINKSURFACE8P,
    BINKSURFACEMASK, BINKSURFACEUYVY, BINKSURFACEYUY2, BINKSURFACEYV12, DEFAULT_BINK_COLORSPACE,
};
use bink2_codec::tables::FULLRANGE_BINK_COLORSPACE;
use std::ffi::c_void;
use std::time::Instant;

pub(crate) unsafe fn fill_plane_set_black(fb: *const BinkFrameBuffers, idx: usize) {
    if fb.is_null() || idx >= 3 {
        return;
    }
    let fb_ref = std::ptr::read_unaligned(fb);
    let ya_h = fb_ref.ya_buffer_height as usize;
    let crcb_h = fb_ref.crcb_buffer_height as usize;
    let ps = fb_ref.frames[idx];

    if ps.y_plane.allocate != 0 && !ps.y_plane.buffer.is_null() && ps.y_plane.buffer_pitch > 0 {
        std::ptr::write_bytes(
            ps.y_plane.buffer as *mut u8,
            0x10,
            (ps.y_plane.buffer_pitch as usize).saturating_mul(ya_h),
        );
    }
    if ps.cr_plane.allocate != 0 && !ps.cr_plane.buffer.is_null() && ps.cr_plane.buffer_pitch > 0 {
        std::ptr::write_bytes(
            ps.cr_plane.buffer as *mut u8,
            0x80,
            (ps.cr_plane.buffer_pitch as usize).saturating_mul(crcb_h),
        );
    }
    if ps.cb_plane.allocate != 0 && !ps.cb_plane.buffer.is_null() && ps.cb_plane.buffer_pitch > 0 {
        std::ptr::write_bytes(
            ps.cb_plane.buffer as *mut u8,
            0x80,
            (ps.cb_plane.buffer_pitch as usize).saturating_mul(crcb_h),
        );
    }
    if ps.a_plane.allocate != 0 && !ps.a_plane.buffer.is_null() && ps.a_plane.buffer_pitch > 0 {
        std::ptr::write_bytes(
            ps.a_plane.buffer as *mut u8,
            0xFF,
            (ps.a_plane.buffer_pitch as usize).saturating_mul(ya_h),
        );
    }
    if ps.h_plane.allocate != 0 && !ps.h_plane.buffer.is_null() && ps.h_plane.buffer_pitch > 0 {
        std::ptr::write_bytes(
            ps.h_plane.buffer as *mut u8,
            0x00,
            (ps.h_plane.buffer_pitch as usize).saturating_mul(ya_h),
        );
    }
}

#[inline]
fn clamp_u8(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

struct YuvToRgbCoeffs {
    r_cr: i32,
    r_cb: i32,
    r_off: i32,
    r_y: i32,
    g_cr: i32,
    g_cb: i32,
    g_off: i32,
    g_y: i32,
    b_cr: i32,
    b_cb: i32,
    b_off: i32,
    b_y: i32,
}

impl YuvToRgbCoeffs {
    fn from_matrix(m: &[f32; 16]) -> Self {
        let s = |v: f32| (v * 1024.0).round() as i32;
        let off = |v: f32| (v * 255.0 * 1024.0 + 512.0).round() as i32;
        Self {
            r_cr: s(m[0]),
            g_cr: s(m[1]),
            b_cr: s(m[2]),
            r_cb: s(m[4]),
            g_cb: s(m[5]),
            b_cb: s(m[6]),
            r_off: off(m[8]),
            g_off: off(m[9]),
            b_off: off(m[10]),
            r_y: s(m[12]),
            g_y: s(m[13]),
            b_y: s(m[14]),
        }
    }

    #[inline]
    fn convert(&self, y: u8, cr: u8, cb: u8) -> (u8, u8, u8) {
        let yi = y as i32;
        let cri = cr as i32;
        let cbi = cb as i32;
        let r = (self.r_y * yi + self.r_cr * cri + self.r_cb * cbi + self.r_off) >> 10;
        let g = (self.g_y * yi + self.g_cr * cri + self.g_cb * cbi + self.g_off) >> 10;
        let b = (self.b_y * yi + self.b_cr * cri + self.b_cb * cbi + self.b_off) >> 10;
        (clamp_u8(r), clamp_u8(g), clamp_u8(b))
    }
}

pub(crate) unsafe fn copy_to_buffer(
    bink: *mut c_void,
    dest: *mut c_void,
    destpitch: i32,
    destheight: u32,
    destx: u32,
    desty: u32,
    flags: u32,
) -> i32 {
    if bink.is_null() {
        return 1;
    }
    let inst = &*(bink as *const BinkInstance);
    let width = inst.rd_u32(0x000);
    let height = inst.rd_u32(0x004);
    copy_to_buffer_rect(
        bink, dest, destpitch, destheight, destx, desty, 0, 0, width, height, flags,
    )
}

pub(crate) unsafe fn copy_to_buffer_rect(
    bink: *mut c_void,
    dest: *mut c_void,
    destpitch: i32,
    destheight: u32,
    destx: u32,
    desty: u32,
    srcx: u32,
    srcy: u32,
    srcw: u32,
    srch: u32,
    flags: u32,
) -> i32 {
    if bink.is_null() || dest.is_null() || destpitch == 0 || destheight == 0 {
        return 1;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    if !inst.video_on {
        return 1;
    }

    let fb_raw = inst.rd_u64(0x0b0) as *const BinkFrameBuffers;
    let fb_ptr = if !fb_raw.is_null() {
        fb_raw
    } else {
        &raw const inst.default_frame_buffers
    };
    let fb = std::ptr::read_unaligned(fb_ptr);
    let total = (fb.total_frames.max(1) as usize).min(3);
    let idx = (fb.frame_num as usize) % total;
    let ps = fb.frames[idx];
    if ps.y_plane.buffer.is_null()
        || ps.cr_plane.buffer.is_null()
        || ps.cb_plane.buffer.is_null()
        || ps.y_plane.buffer_pitch == 0
    {
        return 1;
    }

    let video_w = inst.rd_u32(0x000);
    let video_h = inst.rd_u32(0x004);
    let ya_w = fb.ya_buffer_width as usize;
    let ya_h = fb.ya_buffer_height as usize;
    let crcb_w = fb.crcb_buffer_width as usize;
    let crcb_h = fb.crcb_buffer_height as usize;
    if ya_w == 0 || ya_h == 0 || srcx >= video_w || srcy >= video_h || desty >= destheight {
        return 0;
    }

    let max_src_w = video_w
        .saturating_sub(srcx)
        .min((ya_w as u32).saturating_sub(srcx));
    let max_src_h = video_h
        .saturating_sub(srcy)
        .min((ya_h as u32).saturating_sub(srcy))
        .min(destheight.saturating_sub(desty));
    let mut copy_w = srcw.min(max_src_w) as usize;
    let copy_h = srch.min(max_src_h) as usize;
    if copy_w == 0 || copy_h == 0 {
        return 0;
    }

    let fmt = flags & BINKSURFACEMASK;
    let row_stride_abs = destpitch.unsigned_abs() as usize;
    let bpp: usize = match fmt {
        BINKSURFACE8P | BINKSURFACEYV12 => 1,
        BINKSURFACE4444
        | BINKSURFACE5551
        | BINKSURFACE555
        | BINKSURFACE565
        | BINKSURFACE655
        | BINKSURFACE664
        | BINKSURFACE555R
        | BINKSURFACE565R
        | BINKSURFACEYUY2
        | BINKSURFACEUYVY => 2,
        BINKSURFACE24 | BINKSURFACE24R => 3,
        _ => 4,
    };
    let max_dst_px = row_stride_abs / bpp;
    if (destx as usize) >= max_dst_px {
        return 0;
    }
    copy_w = copy_w.min(max_dst_px - (destx as usize));
    if copy_w == 0 {
        return 0;
    }

    let t_blit = Instant::now();
    let video_flags = inst.rd_u32(0x024);
    let open_flags = inst.rd_u32(0x020);
    let cs_matrix = if (video_flags & 0x04) != 0 {
        if (open_flags & 0x10) != 0 {
            FULLRANGE_BINK_COLORSPACE
        } else {
            DEFAULT_BINK_COLORSPACE
        }
    } else {
        let mut m = [0.0f32; 16];
        for (i, slot) in m.iter_mut().enumerate() {
            *slot = f32::from_bits(inst.rd_u32(0x060 + i * 4));
        }
        if m[12] == 0.0 && m[13] == 0.0 && m[14] == 0.0 {
            DEFAULT_BINK_COLORSPACE
        } else {
            m
        }
    };
    let coeffs = YuvToRgbCoeffs::from_matrix(&cs_matrix);

    let y_ptr = ps.y_plane.buffer as *const u8;
    let cr_ptr = ps.cr_plane.buffer as *const u8;
    let cb_ptr = ps.cb_plane.buffer as *const u8;
    let y_pitch = ps.y_plane.buffer_pitch as usize;
    let cr_pitch = ps.cr_plane.buffer_pitch as usize;
    let cb_pitch = ps.cb_plane.buffer_pitch as usize;
    let has_alpha_plane = !ps.a_plane.buffer.is_null() && ps.a_plane.buffer_pitch > 0;
    let a_ptr = ps.a_plane.buffer as *const u8;
    let a_pitch = ps.a_plane.buffer_pitch as usize;

    let dest_base = dest as *mut u8;
    let pitch = destpitch as isize;
    let dx_bytes = (destx as usize) * bpp;

    if fmt == BINKSURFACEYV12 {
        for row in 0..copy_h {
            let sy = (srcy as usize) + row;
            let dst_row = dest_base
                .offset(((desty as isize) + (row as isize)) * pitch)
                .add(dx_bytes);
            let src_row = y_ptr.add(sy * y_pitch + (srcx as usize));
            std::ptr::copy_nonoverlapping(src_row, dst_row, copy_w);
        }
        if destpitch > 0 && crcb_w > 0 && crcb_h > 0 && cr_pitch > 0 && cb_pitch > 0 {
            let uv_pitch = row_stride_abs / 2;
            let uv_h = (destheight as usize) / 2;
            let v_base = dest_base.add(row_stride_abs * (destheight as usize));
            let u_base = v_base.add(uv_pitch * uv_h);
            let copy_uv_w = ((copy_w + 1) / 2).min(uv_pitch.saturating_sub((destx as usize) / 2));
            let copy_uv_h = ((copy_h + 1) / 2).min(uv_h.saturating_sub((desty as usize) / 2));
            for row in 0..copy_uv_h {
                let cy = (((srcy as usize) >> 1) + row).min(crcb_h - 1);
                let cx0 = ((srcx as usize) >> 1).min(crcb_w - 1);
                let safe_w = copy_uv_w.min(crcb_w - cx0);
                let dy = ((desty as usize) / 2) + row;
                let dx = (destx as usize) / 2;
                std::ptr::copy_nonoverlapping(
                    cr_ptr.add(cy * cr_pitch + cx0),
                    v_base.add(dy * uv_pitch + dx),
                    safe_w,
                );
                std::ptr::copy_nonoverlapping(
                    cb_ptr.add(cy * cb_pitch + cx0),
                    u_base.add(dy * uv_pitch + dx),
                    safe_w,
                );
            }
        }
    } else if fmt == BINKSURFACEYUY2 || fmt == BINKSURFACEUYVY {
        let pairs = copy_w / 2;
        for row in 0..copy_h {
            let sy = (srcy as usize) + row;
            let cy = if crcb_h > 0 {
                (sy >> 1).min(crcb_h - 1)
            } else {
                0
            };
            let dst_row = dest_base
                .offset(((desty as isize) + (row as isize)) * pitch)
                .add(dx_bytes);
            for p in 0..pairs {
                let sx0 = (srcx as usize) + p * 2;
                let sx1 = sx0 + 1;
                let y0 = *y_ptr.add(sy * y_pitch + sx0);
                let y1 = *y_ptr.add(sy * y_pitch + sx1);
                let (cr, cb) = if crcb_w > 0 && crcb_h > 0 && cr_pitch > 0 && cb_pitch > 0 {
                    let cx = (sx0 >> 1).min(crcb_w - 1);
                    (*cr_ptr.add(cy * cr_pitch + cx), *cb_ptr.add(cy * cb_pitch + cx))
                } else {
                    (0x80, 0x80)
                };
                let quad = if fmt == BINKSURFACEYUY2 {
                    [y0, cb, y1, cr]
                } else {
                    [cb, y0, cr, y1]
                };
                std::ptr::copy_nonoverlapping(quad.as_ptr(), dst_row.add(p * 4), 4);
            }
        }
    } else {
        for row in 0..copy_h {
            let sy = (srcy as usize) + row;
            let cy = if crcb_h > 0 {
                (sy >> 1).min(crcb_h - 1)
            } else {
                0
            };
            let dst_row = dest_base
                .offset(((desty as isize) + (row as isize)) * pitch)
                .add(dx_bytes);
            let y_row = y_ptr.add(sy * y_pitch);
            let cr_row = if crcb_h > 0 && cr_pitch > 0 {
                cr_ptr.add(cy * cr_pitch)
            } else {
                std::ptr::null()
            };
            let cb_row = if crcb_h > 0 && cb_pitch > 0 {
                cb_ptr.add(cy * cb_pitch)
            } else {
                std::ptr::null()
            };
            let a_row = if has_alpha_plane {
                a_ptr.add(sy * a_pitch)
            } else {
                std::ptr::null()
            };

            for col in 0..copy_w {
                let sx = (srcx as usize) + col;
                let y = *y_row.add(sx);
                if fmt == BINKSURFACE8P {
                    *dst_row.add(col) = y;
                    continue;
                }
                let (cr, cb) = if !cr_row.is_null() && !cb_row.is_null() && crcb_w > 0 {
                    let cx = (sx >> 1).min(crcb_w - 1);
                    (*cr_row.add(cx), *cb_row.add(cx))
                } else {
                    (0x80, 0x80)
                };
                let (r, g, b) = coeffs.convert(y, cr, cb);
                let a = if !a_row.is_null() {
                    *a_row.add(sx)
                } else {
                    0xFF
                };
                match fmt {
                    BINKSURFACE24 => {
                        let px = [b, g, r];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 3), 3);
                    }
                    BINKSURFACE24R => {
                        let px = [r, g, b];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 3), 3);
                    }
                    BINKSURFACE32 => {
                        let px = [b, g, r, 0xFF];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 4), 4);
                    }
                    BINKSURFACE32R => {
                        let px = [r, g, b, 0xFF];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 4), 4);
                    }
                    BINKSURFACE32A => {
                        let px = [b, g, r, a];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 4), 4);
                    }
                    BINKSURFACE32RA => {
                        let px = [r, g, b, a];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 4), 4);
                    }
                    BINKSURFACE4444 => {
                        let v: u16 = (((a as u16) >> 4) << 12)
                            | (((r as u16) >> 4) << 8)
                            | (((g as u16) >> 4) << 4)
                            | ((b as u16) >> 4);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE5551 => {
                        let v: u16 = (((a as u16) >> 7) << 15)
                            | (((r as u16) >> 3) << 10)
                            | (((g as u16) >> 3) << 5)
                            | ((b as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE555 => {
                        let v: u16 = (((r as u16) >> 3) << 10)
                            | (((g as u16) >> 3) << 5)
                            | ((b as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE565 => {
                        let v: u16 = (((r as u16) >> 3) << 11)
                            | (((g as u16) >> 2) << 5)
                            | ((b as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE655 => {
                        let v: u16 = (((r as u16) >> 2) << 10)
                            | (((g as u16) >> 3) << 5)
                            | ((b as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE664 => {
                        let v: u16 = (((r as u16) >> 2) << 10)
                            | (((g as u16) >> 2) << 4)
                            | ((b as u16) >> 4);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE555R => {
                        let v: u16 = (((b as u16) >> 3) << 10)
                            | (((g as u16) >> 3) << 5)
                            | ((r as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    BINKSURFACE565R => {
                        let v: u16 = (((b as u16) >> 3) << 11)
                            | (((g as u16) >> 2) << 5)
                            | ((r as u16) >> 3);
                        std::ptr::copy_nonoverlapping(
                            v.to_le_bytes().as_ptr(),
                            dst_row.add(col * 2),
                            2,
                        );
                    }
                    _ => {
                        let px = [b, g, r, 0xFF];
                        std::ptr::copy_nonoverlapping(px.as_ptr(), dst_row.add(col * 4), 4);
                    }
                }
            }
        }
    }

    let blit_us = t_blit.elapsed().as_micros() as u64;
    inst.total_blit_us = inst.total_blit_us.saturating_add(blit_us);
    if let Some(last) = inst.frame_timings.back_mut() {
        last.blit_us = last.blit_us.saturating_add(blit_us);
    }
    0
}
