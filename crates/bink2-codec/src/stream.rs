//! Pure-Rust streaming Bink 2 (`KB2f`..`KB2n`) container parser and sequential YUV420p decoder.
//!
//! Used by offline asset optimizers to decode any standalone `.bk2` or embedded `KB2*` stream
//! into planar YUV420p frames without relying on external Bink binaries.

use crate::codec::{compute_slices, decode_frame, FramePlanes, SliceInfo};

/// Standard 44-byte Bink 2 / Bink 1 (`Bk2FileHeader`) file header at offset `0x00`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bk2FileHeader {
    pub magic: [u8; 4],          // +0x00: b"KB2*" or b"BIK*" or b"KB2v"
    pub file_size_minus_8: u32,  // +0x04
    pub frames: u32,             // +0x08
    pub largest_frame_size: u32, // +0x0c
    pub frames_dup: u32,         // +0x10
    pub width: u32,              // +0x14
    pub height: u32,             // +0x18
    pub fps_num: u32,            // +0x1c
    pub fps_den: u32,            // +0x20
    pub video_flags: u32,        // +0x24
    pub num_tracks: u32,         // +0x28
}

const _: () = assert!(std::mem::size_of::<Bk2FileHeader>() == 0x2c);

impl Bk2FileHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 0x2c {
            return None;
        }
        let u = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        Some(Self {
            magic: bytes[0..4].try_into().unwrap(),
            file_size_minus_8: u(0x04),
            frames: u(0x08),
            largest_frame_size: u(0x0c),
            frames_dup: u(0x10),
            width: u(0x14),
            height: u(0x18),
            fps_num: u(0x1c),
            fps_den: u(0x20),
            video_flags: u(0x24),
            num_tracks: u(0x28),
        })
    }

    pub fn to_bytes(&self) -> [u8; 0x2c] {
        let mut out = [0u8; 0x2c];
        out[0x00..0x04].copy_from_slice(&self.magic);
        out[0x04..0x08].copy_from_slice(&self.file_size_minus_8.to_le_bytes());
        out[0x08..0x0c].copy_from_slice(&self.frames.to_le_bytes());
        out[0x0c..0x10].copy_from_slice(&self.largest_frame_size.to_le_bytes());
        out[0x10..0x14].copy_from_slice(&self.frames_dup.to_le_bytes());
        out[0x14..0x18].copy_from_slice(&self.width.to_le_bytes());
        out[0x18..0x1c].copy_from_slice(&self.height.to_le_bytes());
        out[0x1c..0x20].copy_from_slice(&self.fps_num.to_le_bytes());
        out[0x20..0x24].copy_from_slice(&self.fps_den.to_le_bytes());
        out[0x24..0x28].copy_from_slice(&self.video_flags.to_le_bytes());
        out[0x28..0x2c].copy_from_slice(&self.num_tracks.to_le_bytes());
        out
    }

    #[inline]
    pub fn fourcc(&self) -> u32 {
        u32::from_le_bytes(self.magic)
    }

    #[inline]
    pub fn display_width(&self) -> u32 {
        (self.width & 0x7fff).max(16)
    }

    #[inline]
    pub fn display_height(&self) -> u32 {
        (self.height & 0x7fff).max(16)
    }

    #[inline]
    pub fn has_alpha(&self) -> bool {
        (self.video_flags & 0x0010_0000) != 0
    }

    #[inline]
    pub fn has_hdr(&self) -> bool {
        (self.video_flags & 0x0000_0004) != 0
    }

    #[inline]
    pub fn is_full_range(&self) -> bool {
        (self.video_flags & 0x0000_0010) != 0
    }
}

#[inline]
pub fn is_bink2_magic(fourcc: u32) -> bool {
    matches!(
        &fourcc.to_le_bytes(),
        b"KB2f" | b"KB2g" | b"KB2h" | b"KB2i" | b"KB2j" | b"KB2k" | b"KB2m" | b"KB2n"
    )
}

#[inline]
pub fn has_extended_header(fourcc: u32) -> bool {
    matches!(
        &fourcc.to_le_bytes(),
        b"KB2i" | b"KB2j" | b"KB2k" | b"KB2m" | b"KB2n" | b"BIKk"
    )
}

/// Convert 32-bit Bink frame offset table into wrap-aware 64-bit offsets.
/// Bit 0 of each entry indicates a keyframe; upper bits are the 4-byte aligned offset.
pub fn expand_frame_offsets(
    frames: usize,
    raw_offs: &[u32],
    fps_rounded: u32,
) -> Option<(Vec<u64>, u64, u32)> {
    let mut out = vec![0u64; frames + 1];
    for i in 0..=frames {
        out[i] = raw_offs.get(i).copied().unwrap_or(0) as u64;
    }
    let mut wrap = 0u64;
    for i in 0..frames {
        let cur = out[i];
        out[i] = cur + wrap;
        let next = out[i + 1];
        let diff = (next & !3) as i64 - (cur & !3) as i64;
        if diff > 0x1000_0000 {
            return None;
        }
        if diff < 1 {
            wrap += 0x1_0000_0000;
        }
    }
    out[frames] += wrap;

    let span = fps_rounded.max(1) as usize;
    let mut highest_rate = 0u64;
    let mut highest_frame = 0u32;
    if frames > span {
        for i in 0..(frames - span) {
            let bytes = (out[i + span] & !3).saturating_sub(out[i] & !3);
            if bytes > highest_rate {
                highest_rate = bytes;
                highest_frame = i as u32;
            }
        }
    }
    if highest_rate == 0 && frames > 0 {
        let total = (out[frames] & !3).saturating_sub(out[0] & !3);
        highest_rate = total.saturating_mul(span as u64) / (frames as u64);
    }
    Some((out, highest_rate, highest_frame))
}

/// Sequential in-memory Bink 2 decoder producing tight even-dimension planar YUV420p frames.
pub struct Bink2StreamDecoder<'a> {
    data: &'a [u8],
    pub header: Bk2FileHeader,
    pub width: u32,
    pub height: u32,
    /// Even-aligned output dimensions suitable for `yuv420p` encoders.
    pub yuv_width: u32,
    pub yuv_height: u32,
    ya_width: usize,
    ya_height: usize,
    ya_pitch: usize,
    crcb_pitch: usize,
    slices: SliceInfo,
    frame_offsets: Vec<u64>,
    num_tracks: usize,
    has_alpha: bool,
    has_hdr: bool,
    // Ping-pong planar buffers (`[0]` and `[1]`).
    y_bufs: [Vec<u8>; 2],
    cr_bufs: [Vec<u8>; 2],
    cb_bufs: [Vec<u8>; 2],
    a_bufs: Option<[Vec<u8>; 2]>,
    h_bufs: Option<[Vec<u8>; 2]>,
    cur_buf: usize,
    next_frame: usize,
}

impl<'a> Bink2StreamDecoder<'a> {
    pub fn open(data: &'a [u8]) -> Result<Self, String> {
        let header = Bk2FileHeader::parse(data).ok_or("input shorter than 44-byte Bink header")?;
        let fourcc = header.fourcc();
        if !is_bink2_magic(fourcc) {
            return Err(format!(
                "unsupported Bink magic {:?}",
                std::str::from_utf8(&header.magic).unwrap_or("????")
            ));
        }
        if header.frames == 0 || header.width == 0 || header.height == 0 {
            return Err("invalid Bink2 header dimensions or frame count".to_string());
        }

        let width = header.display_width();
        let height = header.display_height();
        let yuv_width = (width + 1) & !1;
        let yuv_height = (height + 1) & !1;
        let frames = header.frames as usize;
        let num_tracks = (header.num_tracks & 0x3fff).min(256) as usize;
        let has_alpha = header.has_alpha();
        let has_hdr = header.has_hdr();

        let extra_hdr_bytes = if has_extended_header(fourcc) {
            let base = ((header.video_flags >> 24) as usize) * 4 + 4;
            if has_hdr {
                base + 12
            } else if (header.video_flags & 0x10000) != 0 {
                base + 24
            } else {
                base
            }
        } else {
            0
        };

        let tables_off = 0x2c + extra_hdr_bytes;
        let tables_dwords = num_tracks * 3 + frames + 1;
        let tables_bytes = tables_dwords
            .checked_mul(4)
            .ok_or("Bink2 tables size overflow")?;
        if tables_off + tables_bytes > data.len() {
            return Err("Bink2 offset table truncated".to_string());
        }

        let fo_base = tables_off + num_tracks * 3 * 4;
        let mut raw_fo = Vec::with_capacity(frames + 1);
        for i in 0..=frames {
            let p = fo_base + i * 4;
            raw_fo.push(u32::from_le_bytes(data[p..p + 4].try_into().unwrap()));
        }

        let fps_num = header.fps_num.max(1);
        let fps_den = header.fps_den.max(1);
        let fps_rounded = ((fps_den >> 1).saturating_add(fps_num) / fps_den).max(1);
        let (frame_offsets, _, _) = expand_frame_offsets(frames, &raw_fo, fps_rounded)
            .ok_or("invalid Bink2 frame offset table")?;

        let open_flags = header.video_flags & 0x0006_0017;
        let slices = compute_slices(fourcc, open_flags, width, height);

        let ya_width = ((width + 31) & !31) as usize;
        let ya_height = ((height + 15) & !15) as usize;
        let crcb_w = ya_width >> 1;
        let crcb_h = ya_height >> 1;
        let ya_pitch = (ya_width + 15) & !15;
        let crcb_pitch = (crcb_w + 15) & !15;
        let ya_bytes = ya_pitch * ya_height;
        let crcb_bytes = crcb_pitch * crcb_h;

        let y_bufs = [vec![0x10u8; ya_bytes], vec![0x10u8; ya_bytes]];
        let cr_bufs = [vec![0x80u8; crcb_bytes], vec![0x80u8; crcb_bytes]];
        let cb_bufs = [vec![0x80u8; crcb_bytes], vec![0x80u8; crcb_bytes]];
        let a_bufs = has_alpha.then(|| [vec![0xFFu8; ya_bytes], vec![0xFFu8; ya_bytes]]);
        let h_bufs = has_hdr.then(|| [vec![0x00u8; ya_bytes], vec![0x00u8; ya_bytes]]);

        Ok(Self {
            data,
            header,
            width,
            height,
            yuv_width,
            yuv_height,
            ya_width,
            ya_height,
            ya_pitch,
            crcb_pitch,
            slices,
            frame_offsets,
            num_tracks,
            has_alpha,
            has_hdr,
            y_bufs,
            cr_bufs,
            cb_bufs,
            a_bufs,
            h_bufs,
            cur_buf: 0,
            next_frame: 0,
        })
    }

    pub fn total_frames(&self) -> usize {
        self.header.frames as usize
    }

    /// Decode the next frame and write tight planar `YUV420p` (`Y`, then `U=Cb`, then `V=Cr`)
    /// of size `yuv_width * yuv_height * 3 / 2` into `out_yuv420p`.
    /// Returns `Ok(false)` once all frames have been decoded.
    pub fn decode_next_yuv420p(&mut self, out_yuv420p: &mut Vec<u8>) -> Result<bool, String> {
        let idx = self.next_frame;
        if idx >= self.total_frames() {
            return Ok(false);
        }

        let raw_start = self.frame_offsets[idx];
        let raw_end = self.frame_offsets[idx + 1];
        let is_key = (raw_start & 1) != 0;
        let start_off = (raw_start & !3) as usize;
        let end_off = (raw_end & !3) as usize;
        if end_off <= start_off || end_off > self.data.len() {
            return Err(format!(
                "frame {} out of bounds ({}..{}, len {})",
                idx + 1,
                start_off,
                end_off,
                self.data.len()
            ));
        }

        let pkt = &self.data[start_off..end_off];
        let mut pos = 0usize;
        for _ in 0..self.num_tracks {
            if pos + 4 > pkt.len() {
                pos = pkt.len();
                break;
            }
            let alen = u32::from_le_bytes(pkt[pos..pos + 4].try_into().unwrap()) as usize;
            pos = pos.saturating_add(4).saturating_add(alen);
        }
        if pos + 8 > pkt.len() {
            return Err(format!("frame {} video payload truncated", idx + 1));
        }

        let ref_idx = self.cur_buf;
        let dst_idx = 1 - self.cur_buf;

        let (y0, y1) = self.y_bufs.split_at_mut(1);
        let (ref_y, dst_y) = if ref_idx == 0 {
            (y0[0].as_slice(), y1[0].as_mut_slice())
        } else {
            (y1[0].as_slice(), y0[0].as_mut_slice())
        };
        let (cr0, cr1) = self.cr_bufs.split_at_mut(1);
        let (ref_cr, dst_cr) = if ref_idx == 0 {
            (cr0[0].as_slice(), cr1[0].as_mut_slice())
        } else {
            (cr1[0].as_slice(), cr0[0].as_mut_slice())
        };
        let (cb0, cb1) = self.cb_bufs.split_at_mut(1);
        let (ref_cb, dst_cb) = if ref_idx == 0 {
            (cb0[0].as_slice(), cb1[0].as_mut_slice())
        } else {
            (cb1[0].as_slice(), cb0[0].as_mut_slice())
        };
        let (ref_a, dst_a) = match self.a_bufs.as_mut() {
            Some(bufs) => {
                let (a0, a1) = bufs.split_at_mut(1);
                if ref_idx == 0 {
                    (Some(a0[0].as_slice()), Some(a1[0].as_mut_slice()))
                } else {
                    (Some(a1[0].as_slice()), Some(a0[0].as_mut_slice()))
                }
            }
            None => (None, None),
        };
        let (ref_h, dst_h) = match self.h_bufs.as_mut() {
            Some(bufs) => {
                let (h0, h1) = bufs.split_at_mut(1);
                if ref_idx == 0 {
                    (Some(h0[0].as_slice()), Some(h1[0].as_mut_slice()))
                } else {
                    (Some(h1[0].as_slice()), Some(h0[0].as_mut_slice()))
                }
            }
            None => (None, None),
        };

        let mut planes = FramePlanes {
            ya_width: self.ya_width,
            ya_height: self.ya_height,
            ya_pitch: self.ya_pitch,
            crcb_pitch: self.crcb_pitch,
            dst_y,
            dst_cr,
            dst_cb,
            dst_a,
            dst_h,
            ref_y,
            ref_cr,
            ref_cb,
            ref_a,
            ref_h,
        };
        decode_frame(
            &pkt[pos..],
            &self.slices,
            is_key,
            self.has_alpha,
            self.has_hdr,
            &mut planes,
        )
        .map_err(|e| format!("frame {} decode error: {e}", idx + 1))?;

        let yw = self.yuv_width as usize;
        let yh = self.yuv_height as usize;
        let cw = yw >> 1;
        let ch = yh >> 1;
        let frame_bytes = yw * yh + cw * ch * 2;
        out_yuv420p.clear();
        out_yuv420p.reserve(frame_bytes);

        let dst_y_ro = &self.y_bufs[dst_idx];
        let dst_cb_ro = &self.cb_bufs[dst_idx];
        let dst_cr_ro = &self.cr_bufs[dst_idx];

        // Y plane
        for row in 0..yh {
            let off = row * self.ya_pitch;
            out_yuv420p.extend_from_slice(&dst_y_ro[off..off + yw]);
        }
        // U (Cb) plane — ffmpeg yuv420p expects Y, U (Cb), V (Cr)
        for row in 0..ch {
            let off = row * self.crcb_pitch;
            out_yuv420p.extend_from_slice(&dst_cb_ro[off..off + cw]);
        }
        // V (Cr) plane
        for row in 0..ch {
            let off = row * self.crcb_pitch;
            out_yuv420p.extend_from_slice(&dst_cr_ro[off..off + cw]);
        }

        self.cur_buf = dst_idx;
        self.next_frame += 1;
        Ok(true)
    }
}
