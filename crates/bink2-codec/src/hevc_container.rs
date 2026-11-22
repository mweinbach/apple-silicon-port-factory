//! Universal `KB2v` hardware-HEVC video container format & offline `hevc_videotoolbox` transcoder.
//!
//! Layout:
//! - `[0x00..0x2c]`: standard 44-byte [`Bk2FileHeader`] with `magic = *b"KB2v"`.
//!   Keeping the exact 44-byte prefix + `"KB2"` 3-byte magic ensures game engines that sniff
//!   `KB2*` headers or read `width`/`height`/`frames` directly before calling `bink2w64.dll`
//!   accept `KB2v` streams transparently.
//! - `[0x2c..0x44]`: 24-byte [`Kb2vMeta`] (`tag = *b"HEVC"`, `version = 1`, `vps_len`, `sps_len`,
//!   `pps_len`, `reserved = 0`).
//! - `[0x44..0x44 + align4(vps_len + sps_len + pps_len)]`: raw VPS, SPS, PPS NAL units (no start
//!   codes) padded with zeroes to a 4-byte boundary.
//! - `[tables_off..tables_off + (frames + 1) * 4]`: standard Bink 2 32-bit frame offset table
//!   (`(byte_offset & !3) | keyframe_bit`), compatible with `expand_frame_offsets`.
//! - `[data_off..]`: 4-byte-aligned HVCC access units (`[u32_be(nal_len)][nal_bytes]...`), directly
//!   decodable frame-by-frame via Apple `VideoToolbox` (`VTDecompressionSession`).

use crate::stream::{is_bink2_magic, Bink2StreamDecoder, Bk2FileHeader};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub const KB2V_MAGIC: [u8; 4] = *b"KB2v";
pub const KB2V_HEVC_TAG: [u8; 4] = *b"HEVC";
pub const KB2V_VERSION: u32 = 1;
pub const KB2V_META_SIZE: usize = 24;

#[inline]
pub fn is_kb2v_magic(fourcc: u32) -> bool {
    fourcc.to_le_bytes() == KB2V_MAGIC
}

#[inline]
pub fn align4(n: usize) -> usize {
    (n + 3) & !3
}

/// 24-byte secondary header immediately following [`Bk2FileHeader`] at offset `0x2c`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kb2vMeta {
    pub tag: [u8; 4],  // b"HEVC"
    pub version: u32,  // 1
    pub vps_len: u32,
    pub sps_len: u32,
    pub pps_len: u32,
    pub reserved: u32,
}

const _: () = assert!(std::mem::size_of::<Kb2vMeta>() == KB2V_META_SIZE);

impl Kb2vMeta {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < KB2V_META_SIZE {
            return None;
        }
        let u = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        let meta = Self {
            tag: bytes[0..4].try_into().unwrap(),
            version: u(4),
            vps_len: u(8),
            sps_len: u(12),
            pps_len: u(16),
            reserved: u(20),
        };
        if meta.tag != KB2V_HEVC_TAG || meta.version != KB2V_VERSION {
            return None;
        }
        if meta.vps_len == 0 || meta.sps_len == 0 || meta.pps_len == 0 {
            return None;
        }
        if meta.vps_len > 65536 || meta.sps_len > 65536 || meta.pps_len > 65536 {
            return None;
        }
        Some(meta)
    }

    pub fn to_bytes(&self) -> [u8; KB2V_META_SIZE] {
        let mut out = [0u8; KB2V_META_SIZE];
        out[0..4].copy_from_slice(&self.tag);
        out[4..8].copy_from_slice(&self.version.to_le_bytes());
        out[8..12].copy_from_slice(&self.vps_len.to_le_bytes());
        out[12..16].copy_from_slice(&self.sps_len.to_le_bytes());
        out[16..20].copy_from_slice(&self.pps_len.to_le_bytes());
        out[20..24].copy_from_slice(&self.reserved.to_le_bytes());
        out
    }

    pub fn param_sets_padded_len(&self) -> usize {
        align4(self.vps_len as usize + self.sps_len as usize + self.pps_len as usize)
    }
}

/// One HEVC access unit in HVCC 4-byte-length-prefixed format.
#[derive(Clone, Debug)]
pub struct HevcAccessUnit {
    pub is_keyframe: bool,
    pub hvcc_bytes: Vec<u8>,
}

/// Parsed HEVC elementary stream split into parameter sets + per-frame HVCC packets.
#[derive(Clone, Debug)]
pub struct ParsedHevcStream {
    pub vps: Vec<u8>,
    pub sps: Vec<u8>,
    pub pps: Vec<u8>,
    pub frames: Vec<HevcAccessUnit>,
}

/// Parse an Annex-B HEVC elementary stream (e.g. from `ffmpeg -f hevc`, encoded with `-bf 0`)
/// into VPS/SPS/PPS parameter sets and per-frame HVCC 4-byte-length-prefixed access units.
pub fn parse_annex_b_hevc(data: &[u8]) -> Result<ParsedHevcStream, String> {
    // Locate all Annex-B start codes (`00 00 00 01` or `00 00 01`).
    let mut nal_spans: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    let n = data.len();
    let mut cur_start: Option<usize> = None;

    while i + 2 < n {
        let sc_len = if i + 3 < n
            && data[i] == 0
            && data[i + 1] == 0
            && data[i + 2] == 0
            && data[i + 3] == 1
        {
            4
        } else if data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 1 {
            3
        } else {
            0
        };
        if sc_len > 0 {
            if let Some(s) = cur_start {
                let mut e = i;
                while e > s && data[e - 1] == 0 {
                    e -= 1;
                }
                if e > s {
                    nal_spans.push((s, e));
                }
            }
            cur_start = Some(i + sc_len);
            i += sc_len;
        } else {
            i += 1;
        }
    }
    if let Some(s) = cur_start {
        if n > s {
            nal_spans.push((s, n));
        }
    }

    let mut vps: Option<Vec<u8>> = None;
    let mut sps: Option<Vec<u8>> = None;
    let mut pps: Option<Vec<u8>> = None;
    let mut frames: Vec<HevcAccessUnit> = Vec::new();

    for (s, e) in nal_spans {
        let nal = &data[s..e];
        if nal.len() < 2 {
            continue;
        }
        let nal_type = (nal[0] >> 1) & 0x3f;
        match nal_type {
            32 => {
                if vps.is_none() {
                    vps = Some(nal.to_vec());
                }
            }
            33 => {
                if sps.is_none() {
                    sps = Some(nal.to_vec());
                }
            }
            34 => {
                if pps.is_none() {
                    pps = Some(nal.to_vec());
                }
            }
            // AUD (35), EOS (36), EOB (37), FD (38), SEI (39, 40) — omit from VCL frame payload.
            35..=63 => {}
            // VCL slice NAL units (0..=31).
            0..=31 => {
                let first_slice = nal.len() >= 3 && (nal[2] & 0x80) != 0;
                let is_irap = (16..=21).contains(&nal_type);
                if first_slice || frames.is_empty() {
                    let mut hvcc = Vec::with_capacity(4 + nal.len());
                    hvcc.extend_from_slice(&(nal.len() as u32).to_be_bytes());
                    hvcc.extend_from_slice(nal);
                    frames.push(HevcAccessUnit {
                        is_keyframe: is_irap,
                        hvcc_bytes: hvcc,
                    });
                } else if let Some(last) = frames.last_mut() {
                    if is_irap {
                        last.is_keyframe = true;
                    }
                    last.hvcc_bytes
                        .extend_from_slice(&(nal.len() as u32).to_be_bytes());
                    last.hvcc_bytes.extend_from_slice(nal);
                }
            }
            _ => {}
        }
    }

    let vps = vps.ok_or("HEVC stream missing VPS (NAL type 32)")?;
    let sps = sps.ok_or("HEVC stream missing SPS (NAL type 33)")?;
    let pps = pps.ok_or("HEVC stream missing PPS (NAL type 34)")?;
    if frames.is_empty() {
        return Err("HEVC stream contained zero VCL video frames".to_string());
    }

    Ok(ParsedHevcStream {
        vps,
        sps,
        pps,
        frames,
    })
}

/// Mux a [`ParsedHevcStream`] into a self-contained `KB2v` byte buffer.
pub fn build_kb2v(
    width: u32,
    height: u32,
    fps_num: u32,
    fps_den: u32,
    full_range: bool,
    stream: &ParsedHevcStream,
) -> Result<Vec<u8>, String> {
    if stream.frames.is_empty() {
        return Err("cannot build KB2v with 0 frames".to_string());
    }
    let frame_count = stream.frames.len() as u32;
    let meta = Kb2vMeta {
        tag: KB2V_HEVC_TAG,
        version: KB2V_VERSION,
        vps_len: stream.vps.len() as u32,
        sps_len: stream.sps.len() as u32,
        pps_len: stream.pps.len() as u32,
        reserved: 0,
    };

    let ps_padded = meta.param_sets_padded_len();
    let tables_off = 0x2c + KB2V_META_SIZE + ps_padded;
    let tables_bytes = (frame_count as usize + 1) * 4;
    let data_start = tables_off + tables_bytes;

    let mut offsets: Vec<u32> = Vec::with_capacity(frame_count as usize + 1);
    let mut cur_off = data_start as u64;
    let mut largest_frame = 0u32;

    for au in &stream.frames {
        let aligned_len = align4(au.hvcc_bytes.len());
        largest_frame = largest_frame.max(aligned_len as u32);
        let entry = ((cur_off as u32) & !3) | if au.is_keyframe { 1 } else { 0 };
        offsets.push(entry);
        cur_off += aligned_len as u64;
    }
    offsets.push((cur_off as u32) & !3);

    let total_len = cur_off as usize;
    let mut out = Vec::with_capacity(total_len);

    let hdr = Bk2FileHeader {
        magic: KB2V_MAGIC,
        file_size_minus_8: (total_len.saturating_sub(8)) as u32,
        frames: frame_count,
        largest_frame_size: largest_frame,
        frames_dup: frame_count,
        width,
        height,
        fps_num: fps_num.max(1),
        fps_den: fps_den.max(1),
        video_flags: if full_range { 0x10 } else { 0x00 },
        num_tracks: 0,
    };
    out.extend_from_slice(&hdr.to_bytes());
    out.extend_from_slice(&meta.to_bytes());
    out.extend_from_slice(&stream.vps);
    out.extend_from_slice(&stream.sps);
    out.extend_from_slice(&stream.pps);
    while out.len() < tables_off {
        out.push(0);
    }
    for off in &offsets {
        out.extend_from_slice(&off.to_le_bytes());
    }
    for au in &stream.frames {
        out.extend_from_slice(&au.hvcc_bytes);
        let pad = align4(au.hvcc_bytes.len()) - au.hvcc_bytes.len();
        for _ in 0..pad {
            out.push(0);
        }
    }

    debug_assert_eq!(out.len(), total_len);
    Ok(out)
}

/// Compute aspect-preserving, even target dimensions capped to `max_height` (0 = keep original).
pub fn compute_target_video_dims(src_w: u32, src_h: u32, max_height: u32) -> (u32, u32) {
    let even_w = ((src_w + 1) & !1).max(16);
    let even_h = ((src_h + 1) & !1).max(16);
    if max_height == 0 || even_h <= max_height {
        return (even_w, even_h);
    }
    let dst_h = (max_height & !1).max(16);
    let scaled_w = ((even_w as u64 * dst_h as u64) + (even_h as u64 / 2)) / (even_h as u64);
    let dst_w = (((scaled_w as u32) + 1) & !1).max(16);
    (dst_w, dst_h)
}

/// Options for offline `KB2* -> KB2v` hardware HEVC transcoding.
#[derive(Clone, Debug)]
pub struct HevcTranscodeOptions {
    /// Maximum output vertical resolution (e.g. `1080` or `720`; `0` = preserve original height).
    pub max_height: u32,
    /// Target video bitrate in kbps (e.g. `5000` for 1080p High, `3000` for 720p Compact).
    pub bitrate_kbps: u32,
    /// Keyframe interval in frames (default `60`, i.e. 2 s at 30 fps for fast seeking).
    pub gop_size: u32,
    /// Minimum input size in bytes to consider transcoding (default `512 KiB`).
    pub min_input_bytes: usize,
    /// Path to `ffmpeg` binary (default `"ffmpeg"`).
    pub ffmpeg_bin: String,
}

impl Default for HevcTranscodeOptions {
    fn default() -> Self {
        Self {
            max_height: 1080,
            bitrate_kbps: 5000,
            gop_size: 60,
            min_input_bytes: 512 * 1024,
            ffmpeg_bin: "ffmpeg".to_string(),
        }
    }
}

/// Result of attempting to transcode a Bink 2 byte buffer to `KB2v` hardware HEVC.
#[derive(Clone, Debug)]
pub enum TranscodeOutcome {
    Transcoded {
        bytes: Vec<u8>,
        orig_w: u32,
        orig_h: u32,
        new_w: u32,
        new_h: u32,
        frames: u32,
    },
    Skipped(&'static str),
}

/// Transcode a raw `KB2f..KB2n` stream into `KB2v` hardware HEVC using our native Rust Bink 2
/// decoder piped directly into `ffmpeg -c:v hevc_videotoolbox -bf 0` (zero temp files).
pub fn transcode_bink2_to_kb2v(
    input_kb2: &[u8],
    opts: &HevcTranscodeOptions,
) -> Result<TranscodeOutcome, String> {
    if input_kb2.len() < opts.min_input_bytes {
        return Ok(TranscodeOutcome::Skipped("below min_input_bytes"));
    }
    let Some(hdr) = Bk2FileHeader::parse(input_kb2) else {
        return Ok(TranscodeOutcome::Skipped("not a Bink file"));
    };
    if !is_bink2_magic(hdr.fourcc()) {
        return Ok(TranscodeOutcome::Skipped("not KB2f..KB2n"));
    }
    if hdr.frames < 2 {
        return Ok(TranscodeOutcome::Skipped("single-frame video"));
    }
    if hdr.num_tracks != 0 {
        return Ok(TranscodeOutcome::Skipped("has embedded audio tracks"));
    }
    if hdr.has_alpha() {
        return Ok(TranscodeOutcome::Skipped("has alpha plane"));
    }
    if hdr.has_hdr() {
        return Ok(TranscodeOutcome::Skipped("has HDR plane"));
    }

    let mut dec = Bink2StreamDecoder::open(input_kb2)?;
    let orig_w = dec.width;
    let orig_h = dec.height;
    let src_yuv_w = dec.yuv_width;
    let src_yuv_h = dec.yuv_height;
    let (dst_w, dst_h) = compute_target_video_dims(orig_w, orig_h, opts.max_height);
    let fps_num = hdr.fps_num.max(1);
    let fps_den = hdr.fps_den.max(1);
    let full_range = hdr.is_full_range();
    let total_frames = dec.total_frames();

    // Scale bitrate softly down for smaller resolutions (e.g. small UI/effect videos).
    let pixel_ratio = ((dst_w as f64) * (dst_h as f64)) / (1920.0 * 1080.0);
    let effective_kbps = ((opts.bitrate_kbps as f64) * pixel_ratio.clamp(0.15, 1.5))
        .round()
        .max(400.0) as u32;

    let mut cmd = Command::new(&opts.ffmpeg_bin);
    cmd.args([
        "-y",
        "-v",
        "error",
        "-f",
        "rawvideo",
        "-pix_fmt",
        "yuv420p",
        "-s",
        &format!("{src_yuv_w}x{src_yuv_h}"),
        "-r",
        &format!("{fps_num}/{fps_den}"),
        "-color_range",
        if full_range { "pc" } else { "tv" },
        "-i",
        "pipe:0",
    ]);
    if dst_w != src_yuv_w || dst_h != src_yuv_h {
        cmd.args(["-vf", &format!("scale={dst_w}:{dst_h}:flags=lanczos")]);
    }
    cmd.args([
        "-c:v",
        "hevc_videotoolbox",
        "-b:v",
        &format!("{effective_kbps}k"),
        "-bf",
        "0",
        "-g",
        &opts.gop_size.max(1).to_string(),
        "-allow_sw",
        "1",
        "-color_range",
        if full_range { "pc" } else { "tv" },
        "-f",
        "hevc",
        "pipe:1",
    ]);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("failed to spawn {}: {e}", opts.ffmpeg_bin))?;
    let mut stdin = child.stdin.take().ok_or("ffmpeg stdin missing")?;
    let mut stdout = child.stdout.take().ok_or("ffmpeg stdout missing")?;
    let mut stderr = child.stderr.take().ok_or("ffmpeg stderr missing")?;

    // Drain stdout and stderr concurrently so ffmpeg never deadlocks on full pipes.
    let reader_handle = std::thread::spawn(move || -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();
        stdout
            .read_to_end(&mut buf)
            .map_err(|e| format!("reading ffmpeg stdout: {e}"))?;
        Ok(buf)
    });
    let err_handle = std::thread::spawn(move || -> String {
        let mut s = String::new();
        let _ = stderr.read_to_string(&mut s);
        s
    });

    let mut frame_buf = Vec::new();
    let mut decode_err: Option<String> = None;
    loop {
        match dec.decode_next_yuv420p(&mut frame_buf) {
            Ok(true) => {
                if let Err(e) = stdin.write_all(&frame_buf) {
                    decode_err = Some(format!("writing raw YUV to ffmpeg: {e}"));
                    break;
                }
            }
            Ok(false) => break,
            Err(e) => {
                decode_err = Some(e);
                break;
            }
        }
    }
    drop(stdin);

    let status = child
        .wait()
        .map_err(|e| format!("waiting for ffmpeg: {e}"))?;
    let annex_b = reader_handle
        .join()
        .map_err(|_| "ffmpeg stdout reader panicked".to_string())??;
    let err_text = err_handle.join().unwrap_or_default();

    if let Some(e) = decode_err {
        return Err(format!("{e} (ffmpeg stderr: {})", err_text.trim()));
    }
    if !status.success() {
        return Err(format!(
            "ffmpeg exited with {status}: {}",
            err_text.trim()
        ));
    }

    let parsed = parse_annex_b_hevc(&annex_b)?;
    if parsed.frames.len() != total_frames {
        return Err(format!(
            "HEVC frame count mismatch: decoded {} frames from KB2, ffmpeg produced {}",
            total_frames,
            parsed.frames.len()
        ));
    }

    let kb2v = build_kb2v(dst_w, dst_h, fps_num, fps_den, full_range, &parsed)?;
    if kb2v.len() >= input_kb2.len() {
        return Ok(TranscodeOutcome::Skipped("HEVC output not smaller than input"));
    }

    Ok(TranscodeOutcome::Transcoded {
        bytes: kb2v,
        orig_w,
        orig_h,
        new_w: dst_w,
        new_h: dst_h,
        frames: total_frames as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::expand_frame_offsets;

    #[test]
    fn annex_b_parse_and_kb2v_roundtrip() {
        // Synthetic Annex-B stream: VPS(32), SPS(33), PPS(34), SEI(39), IDR(20) w/ 2 slices, TRAIL(1)
        let mut annex = Vec::new();
        let push_nal = |dst: &mut Vec<u8>, bytes: &[u8]| {
            dst.extend_from_slice(&[0, 0, 0, 1]);
            dst.extend_from_slice(bytes);
        };
        push_nal(&mut annex, &[32 << 1, 1, 0xAA, 0xBB]); // VPS
        push_nal(&mut annex, &[33 << 1, 1, 0xCC, 0xDD, 0xEE]); // SPS
        push_nal(&mut annex, &[34 << 1, 1, 0x11]); // PPS
        push_nal(&mut annex, &[39 << 1, 1, 0x99]); // Prefix SEI (skipped)
        push_nal(&mut annex, &[20 << 1, 1, 0x80, 0x01, 0x02]); // IDR slice 0 (first_slice=1)
        push_nal(&mut annex, &[20 << 1, 1, 0x00, 0x03]); // IDR slice 1 (continuation)
        push_nal(&mut annex, &[1 << 1, 1, 0x80, 0x04, 0x05, 0x06]); // P-frame

        let parsed = parse_annex_b_hevc(&annex).unwrap();
        assert_eq!(parsed.vps, &[32 << 1, 1, 0xAA, 0xBB]);
        assert_eq!(parsed.sps, &[33 << 1, 1, 0xCC, 0xDD, 0xEE]);
        assert_eq!(parsed.pps, &[34 << 1, 1, 0x11]);
        assert_eq!(parsed.frames.len(), 2);
        assert!(parsed.frames[0].is_keyframe);
        assert!(!parsed.frames[1].is_keyframe);
        // Frame 0 holds two length-prefixed NALs: (4+5) + (4+4) = 17 bytes.
        assert_eq!(parsed.frames[0].hvcc_bytes.len(), 17);

        let kb2v = build_kb2v(1920, 1080, 30, 1, true, &parsed).unwrap();
        let hdr = Bk2FileHeader::parse(&kb2v).unwrap();
        assert_eq!(hdr.magic, KB2V_MAGIC);
        assert_eq!(hdr.width, 1920);
        assert_eq!(hdr.height, 1080);
        assert_eq!(hdr.frames, 2);
        assert_eq!(hdr.video_flags, 0x10);
        assert_eq!(hdr.file_size_minus_8 as usize + 8, kb2v.len());

        let meta = Kb2vMeta::parse(&kb2v[0x2c..]).unwrap();
        assert_eq!(meta.vps_len, 4);
        assert_eq!(meta.sps_len, 5);
        assert_eq!(meta.pps_len, 3);

        let tables_off = 0x2c + KB2V_META_SIZE + meta.param_sets_padded_len();
        let raw_offs: Vec<u32> = (0..=2)
            .map(|i| {
                u32::from_le_bytes(kb2v[tables_off + i * 4..tables_off + i * 4 + 4].try_into().unwrap())
            })
            .collect();
        let (expanded, _, _) = expand_frame_offsets(2, &raw_offs, 30).unwrap();
        assert_eq!(expanded[0] & 1, 1); // keyframe
        assert_eq!(expanded[1] & 1, 0); // P-frame
        assert_eq!(expanded[2] as usize, kb2v.len());
    }

    #[test]
    fn target_video_dims_preserve_aspect_and_even() {
        assert_eq!(compute_target_video_dims(2560, 1440, 1080), (1920, 1080));
        assert_eq!(compute_target_video_dims(2560, 1440, 720), (1280, 720));
        assert_eq!(compute_target_video_dims(1920, 1080, 1080), (1920, 1080));
        assert_eq!(compute_target_video_dims(1563, 1080, 1080), (1564, 1080));
        assert_eq!(compute_target_video_dims(2084, 1440, 1080), (1564, 1080));
    }
}
