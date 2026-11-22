//! Native `IDemuxer` C++ objects (WebM/Matroska and MP4) backed by FFmpeg `libavformat`
//! with a custom streaming/buffered `AVIOContext`.

use super::ffmpeg::{
    self, rd, wr, AvChannelLayout, AvRational, FfmpegApi, AVCODECPAR_BITS_PER_CODED_SAMPLE_OFF,
    AVCODECPAR_BITS_PER_RAW_SAMPLE_OFF, AVCODECPAR_CH_LAYOUT_OFF, AVCODECPAR_CODEC_ID_OFF,
    AVCODECPAR_CODEC_TYPE_OFF, AVCODECPAR_EXTRADATA_OFF, AVCODECPAR_EXTRADATA_SIZE_OFF,
    AVCODECPAR_HEIGHT_OFF, AVCODECPAR_SAMPLE_RATE_OFF, AVCODECPAR_WIDTH_OFF,
    AVFORMATCTX_DURATION_OFF, AVFORMATCTX_NB_STREAMS_OFF, AVFORMATCTX_PB_OFF,
    AVFORMATCTX_STREAMS_OFF, AVERROR_EOF, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_VIDEO,
    AVPACKET_DATA_OFF, AVPACKET_DTS_OFF, AVPACKET_FLAGS_OFF, AVPACKET_PTS_OFF,
    AVPACKET_SIDEDATA_DATA_OFF, AVPACKET_SIDEDATA_LEN_OFF, AVPACKET_SIDEDATA_SIZE,
    AVPACKET_SIDEDATA_TYPE_OFF, AVPACKET_SIDE_DATA_ELEMS_OFF, AVPACKET_SIDE_DATA_OFF,
    AVPACKET_SIZE_OFF, AVPACKET_STREAM_INDEX_OFF, AVSEEK_SIZE, AVSTREAM_CODECPAR_OFF,
    AVSTREAM_DURATION_OFF, AVSTREAM_TIME_BASE_OFF, AV_CODEC_ID_AAC, AV_CODEC_ID_H264,
    AV_CODEC_ID_MP3, AV_CODEC_ID_OPUS, AV_CODEC_ID_VORBIS, AV_CODEC_ID_VP8, AV_CODEC_ID_VP9,
    AV_NOPTS_VALUE, AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL, AV_PKT_FLAG_CORRUPT, AV_PKT_FLAG_KEY,
    AV_TIME_BASE,
};
use super::{read_guest_vfunc, DemuxerFrame, TrackInfo};
use crate::Ptr;
use std::collections::HashMap;
use std::ffi::{c_char, c_void};
use std::sync::Once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerKind {
    WebM,
    Mp4,
}

impl ContainerKind {
    const fn av_format_names(self) -> &'static [&'static [u8]] {
        match self {
            Self::WebM => &[b"matroska\0", b"webm\0"],
            Self::Mp4 => &[b"mp4\0", b"mov\0"],
        }
    }
}

/// C++ `IDemuxer` object exposed across the Win64 ABI boundary.
///
/// `+0x00` holds the 7-slot vtable pointer:
/// `[dtor, Destroy, SetPlaybackCallbacks, SetSeekCallbacks, Demux, DemuxMoov, Reset]`.
#[repr(C)]
pub struct DemuxerObject {
    pub vtbl: *const [Ptr; 7],
    pub inner: DemuxerInner,
}

struct OwnedPacket {
    data: Vec<u8>,
    alpha: Vec<u8>,
    track_id: u64,
    timestamp_ms: f32,
    is_keyframe: u8,
}

#[derive(Clone, Copy)]
struct TrackMeta {
    track_id: u64,
    time_base: AvRational,
}

/// In-memory virtual file view passed as `opaque` to FFmpeg `AVIOContext`.
struct MemIoView {
    buf_ptr: *const u8,
    buf_len: usize,
    moov_ptr: *const u8,
    moov_len: usize,
    moov_offset: usize,
    pos: usize,
    hit_gap: bool,
}

impl MemIoView {
    fn virt_len(&self) -> usize {
        if self.moov_len > 0 && self.moov_offset != usize::MAX {
            self.moov_offset.saturating_add(self.moov_len).max(self.buf_len)
        } else {
            self.buf_len
        }
    }
}

unsafe extern "C" fn mem_avio_read(opaque: *mut c_void, dst: *mut u8, buf_size: i32) -> i32 {
    if opaque.is_null() || dst.is_null() || buf_size <= 0 {
        return AVERROR_EOF;
    }
    let st = &mut *(opaque as *mut MemIoView);
    let want = buf_size as usize;
    let virt_len = st.virt_len();

    if st.pos < st.buf_len {
        let avail = st.buf_len - st.pos;
        let n = avail.min(want);
        std::ptr::copy_nonoverlapping(st.buf_ptr.add(st.pos), dst, n);
        st.pos += n;
        return n as i32;
    }

    if st.moov_len > 0
        && st.moov_offset != usize::MAX
        && st.pos >= st.moov_offset
        && st.pos < st.moov_offset.saturating_add(st.moov_len)
    {
        let rel = st.pos - st.moov_offset;
        let avail = st.moov_len - rel;
        let n = avail.min(want);
        std::ptr::copy_nonoverlapping(st.moov_ptr.add(rel), dst, n);
        st.pos += n;
        return n as i32;
    }

    if st.pos < virt_len {
        st.hit_gap = true;
    }
    AVERROR_EOF
}

unsafe extern "C" fn mem_avio_seek(opaque: *mut c_void, offset: i64, whence: i32) -> i64 {
    if opaque.is_null() {
        return -(libc::EINVAL as i64);
    }
    let st = &mut *(opaque as *mut MemIoView);
    let virt_len = st.virt_len();

    if (whence & AVSEEK_SIZE) != 0 {
        return virt_len as i64;
    }

    let base_whence = whence & 0xFFFF;
    let target: i64 = match base_whence {
        0 => offset,                       // SEEK_SET
        1 => st.pos as i64 + offset,       // SEEK_CUR
        2 => virt_len as i64 + offset,     // SEEK_END
        _ => return -(libc::EINVAL as i64),
    };

    if target < 0 || (target as usize) > virt_len {
        return -(libc::EINVAL as i64);
    }
    st.pos = target as usize;
    target
}

/// Inspect top-level ISO BMFF (`mp4`) boxes in `buf` to determine where an out-of-band
/// `moov` atom belongs in the virtual file stream.
///
/// Returns `None` if `buf` already contains a `moov` box; otherwise returns the byte offset
/// immediately following `mdat` (or `buf.len()` if `mdat` hasn't appeared yet).
fn compute_mp4_moov_offset(buf: &[u8]) -> Option<usize> {
    let mut pos = 0usize;
    while pos + 8 <= buf.len() {
        let size32 = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as u64;
        let box_type = &buf[pos + 4..pos + 8];
        if box_type == b"moov" {
            return None;
        }
        let box_size: u64 = if size32 == 1 {
            if pos + 16 > buf.len() {
                break;
            }
            u64::from_be_bytes([
                buf[pos + 8],
                buf[pos + 9],
                buf[pos + 10],
                buf[pos + 11],
                buf[pos + 12],
                buf[pos + 13],
                buf[pos + 14],
                buf[pos + 15],
            ])
        } else if size32 == 0 {
            break;
        } else {
            size32
        };
        if box_size < 8 {
            break;
        }
        let next = (pos as u64).saturating_add(box_size);
        if box_type == b"mdat" && next >= buf.len() as u64 {
            return usize::try_from(next).ok();
        }
        if next > buf.len() as u64 {
            break;
        }
        pos = next as usize;
    }
    Some(buf.len())
}

fn codec_name_for_id(codec_id: i32) -> Option<&'static str> {
    match codec_id {
        AV_CODEC_ID_VP8 => Some("vp8"),
        AV_CODEC_ID_VP9 => Some("vp9"),
        AV_CODEC_ID_VORBIS => Some("vorbis"),
        AV_CODEC_ID_H264 => Some("avc1"),
        AV_CODEC_ID_MP3 => Some("mp3"),
        AV_CODEC_ID_AAC => Some("aac"),
        AV_CODEC_ID_OPUS => Some("opus"),
        _ => None,
    }
}

pub struct DemuxerInner {
    kind: ContainerKind,
    playback_cb: *mut c_void,
    seek_cb: *mut c_void,
    buf: Vec<u8>,
    moov_buf: Vec<u8>,
    emitted_headers: bool,
    emitted_no_seek: bool,
    emitted_packets: usize,
}

impl DemuxerInner {
    fn new(kind: ContainerKind) -> Self {
        Self {
            kind,
            playback_cb: std::ptr::null_mut(),
            seek_cb: std::ptr::null_mut(),
            buf: Vec::new(),
            moov_buf: Vec::new(),
            emitted_headers: false,
            emitted_no_seek: false,
            emitted_packets: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.buf.clear();
        self.moov_buf.clear();
        self.emitted_headers = false;
        self.emitted_no_seek = false;
        self.emitted_packets = 0;
    }

    unsafe fn emit_file_info(&self, self_obj: *mut c_void, duration_secs: f64) {
        let on_file_info = read_guest_vfunc(self.playback_cb, 1);
        if on_file_info != 0 {
            let dur = duration_secs;
            let _ = crate::call_guest8(
                on_file_info,
                self.playback_cb as u64,
                self_obj as u64,
                (&dur as *const f64) as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn emit_track_info(&self, self_obj: *mut c_void, track: &TrackInfo) {
        let on_track = read_guest_vfunc(self.playback_cb, 2);
        if on_track != 0 {
            let _ = crate::call_guest8(
                on_track,
                self.playback_cb as u64,
                self_obj as u64,
                (track as *const TrackInfo) as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn emit_frames(&self, self_obj: *mut c_void, batch: &[OwnedPacket]) {
        if batch.is_empty() {
            return;
        }
        let on_frames = read_guest_vfunc(self.playback_cb, 3);
        if on_frames == 0 {
            return;
        }
        let mut frames = Vec::with_capacity(batch.len());
        for p in batch {
            frames.push(DemuxerFrame {
                data: p.data.as_ptr(),
                size: p.data.len(),
                alpha_data: if p.alpha.is_empty() {
                    std::ptr::null()
                } else {
                    p.alpha.as_ptr()
                },
                alpha_size: p.alpha.len(),
                track_id: p.track_id,
                timestamp_ms: p.timestamp_ms,
                is_keyframe: p.is_keyframe,
                _pad: [0; 3],
            });
        }
        let _ = crate::call_guest8(
            on_frames,
            self.playback_cb as u64,
            self_obj as u64,
            frames.as_ptr() as u64,
            frames.len() as u64,
            0,
            0,
            0,
            0,
        );
    }

    unsafe fn emit_error(&self, self_obj: *mut c_void) {
        let on_error = read_guest_vfunc(self.playback_cb, 4);
        if on_error != 0 {
            let _ = crate::call_guest8(
                on_error,
                self.playback_cb as u64,
                self_obj as u64,
                0,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn emit_no_seek_head_once(&mut self, self_obj: *mut c_void) {
        if self.emitted_no_seek || self.seek_cb.is_null() {
            return;
        }
        self.emitted_no_seek = true;
        let on_no_seek = read_guest_vfunc(self.seek_cb, 3);
        if on_no_seek != 0 {
            let _ = crate::call_guest8(
                on_no_seek,
                self.seek_cb as u64,
                self_obj as u64,
                0,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn find_input_format(ff: &FfmpegApi, kind: ContainerKind) -> *const c_void {
        for &name in kind.av_format_names() {
            let fmt = (ff.av_find_input_format)(name.as_ptr() as *const c_char);
            if !fmt.is_null() {
                return fmt;
            }
        }
        std::ptr::null()
    }

    pub(crate) unsafe fn demux_chunk(&mut self, self_obj: *mut c_void, data: *const u8, len: usize) {
        if !data.is_null() && len > 0 {
            let slice = std::slice::from_raw_parts(data, len);
            self.buf.extend_from_slice(slice);
        }
        self.process_buffered(self_obj);
    }

    pub(crate) unsafe fn demux_moov(&mut self, self_obj: *mut c_void, data: *const u8, len: usize) {
        if self.kind != ContainerKind::Mp4 {
            return;
        }
        if !data.is_null() && len > 0 {
            let slice = std::slice::from_raw_parts(data, len);
            self.moov_buf.extend_from_slice(slice);
        }
        self.process_buffered(self_obj);
    }

    unsafe fn process_buffered(&mut self, self_obj: *mut c_void) {
        if self.buf.is_empty() && self.moov_buf.is_empty() {
            return;
        }
        let Some(ff) = ffmpeg::ffmpeg() else {
            self.emit_error(self_obj);
            return;
        };

        let moov_offset = if self.kind == ContainerKind::Mp4 && !self.moov_buf.is_empty() {
            compute_mp4_moov_offset(&self.buf).unwrap_or(usize::MAX)
        } else {
            usize::MAX
        };

        let mut io_view = Box::new(MemIoView {
            buf_ptr: self.buf.as_ptr(),
            buf_len: self.buf.len(),
            moov_ptr: self.moov_buf.as_ptr(),
            moov_len: self.moov_buf.len(),
            moov_offset,
            pos: 0,
            hit_gap: false,
        });

        const AVIO_BUF_SIZE: usize = 8192;
        let avio_buf = (ff.av_malloc)(AVIO_BUF_SIZE) as *mut u8;
        if avio_buf.is_null() {
            self.emit_error(self_obj);
            return;
        }

        let io_ptr = (&mut *io_view as *mut MemIoView) as *mut c_void;
        let mut avio_ctx = (ff.avio_alloc_context)(
            avio_buf,
            AVIO_BUF_SIZE as i32,
            0,
            io_ptr,
            Some(mem_avio_read),
            None,
            Some(mem_avio_seek),
        );
        if avio_ctx.is_null() {
            (ff.av_free)(avio_buf as *mut c_void);
            self.emit_error(self_obj);
            return;
        }

        let mut fmt_ctx = (ff.avformat_alloc_context)();
        if fmt_ctx.is_null() {
            (ff.avio_context_free)(&mut avio_ctx);
            self.emit_error(self_obj);
            return;
        }
        wr::<*mut c_void>(fmt_ctx, AVFORMATCTX_PB_OFF, avio_ctx);

        let input_fmt = Self::find_input_format(ff, self.kind);
        let open_rc = (ff.avformat_open_input)(
            &mut fmt_ctx,
            std::ptr::null(),
            input_fmt,
            std::ptr::null_mut(),
        );
        if open_rc < 0 {
            // `avformat_open_input` frees `fmt_ctx` on error; free custom `avio_ctx`.
            // Do not fire `OnError` here because callers stream headers incrementally.
            if !fmt_ctx.is_null() {
                (ff.avformat_free_context)(fmt_ctx);
            }
            (ff.avio_context_free)(&mut avio_ctx);
            return;
        }

        if !self.emitted_headers && !self.buf.is_empty() {
            let _ = (ff.avformat_find_stream_info)(fmt_ctx, std::ptr::null_mut());
        }

        let nb_streams = rd::<u32>(fmt_ctx, AVFORMATCTX_NB_STREAMS_OFF) as usize;
        let streams_ptr = rd::<*mut *mut c_void>(fmt_ctx, AVFORMATCTX_STREAMS_OFF);
        if nb_streams == 0 || streams_ptr.is_null() {
            (ff.avformat_close_input)(&mut fmt_ctx);
            (ff.avio_context_free)(&mut avio_ctx);
            return;
        }

        let mut track_map: HashMap<i32, TrackMeta> = HashMap::new();
        let mut tracks_to_emit: Vec<(TrackInfo, Vec<u8>)> = Vec::new();
        let mut fallback_duration_secs = 0.0f64;

        for idx in 0..nb_streams {
            let stream = *streams_ptr.add(idx);
            if stream.is_null() {
                continue;
            }
            let codecpar = rd::<*mut c_void>(stream, AVSTREAM_CODECPAR_OFF);
            if codecpar.is_null() {
                continue;
            }
            let codec_type = rd::<i32>(codecpar, AVCODECPAR_CODEC_TYPE_OFF);
            let codec_id = rd::<i32>(codecpar, AVCODECPAR_CODEC_ID_OFF);
            let Some(codec_name) = codec_name_for_id(codec_id) else {
                continue;
            };

            let time_base: AvRational = rd(stream, AVSTREAM_TIME_BASE_OFF);
            let stream_dur = rd::<i64>(stream, AVSTREAM_DURATION_OFF);
            if stream_dur != AV_NOPTS_VALUE && stream_dur > 0 && time_base.den > 0 {
                let s = (stream_dur as f64) * (time_base.num as f64) / (time_base.den as f64);
                if s > fallback_duration_secs {
                    fallback_duration_secs = s;
                }
            }

            let track_id = (idx + 1) as u64;
            track_map.insert(idx as i32, TrackMeta { track_id, time_base });

            if !self.emitted_headers {
                let extra_ptr = rd::<*mut u8>(codecpar, AVCODECPAR_EXTRADATA_OFF);
                let extra_sz = rd::<i32>(codecpar, AVCODECPAR_EXTRADATA_SIZE_OFF);
                let extra_vec = if !extra_ptr.is_null() && extra_sz > 0 {
                    std::slice::from_raw_parts(extra_ptr, extra_sz as usize).to_vec()
                } else {
                    Vec::new()
                };

                if codec_type == AVMEDIA_TYPE_VIDEO {
                    let w = rd::<i32>(codecpar, AVCODECPAR_WIDTH_OFF).max(0) as u64;
                    let h = rd::<i32>(codecpar, AVCODECPAR_HEIGHT_OFF).max(0) as u64;
                    let info = TrackInfo::video(track_id, codec_name, std::ptr::null(), 0, w, h);
                    tracks_to_emit.push((info, extra_vec));
                } else if codec_type == AVMEDIA_TYPE_AUDIO {
                    let bits_raw = rd::<i32>(codecpar, AVCODECPAR_BITS_PER_RAW_SAMPLE_OFF);
                    let bits_coded = rd::<i32>(codecpar, AVCODECPAR_BITS_PER_CODED_SAMPLE_OFF);
                    let bits = if bits_raw > 0 {
                        bits_raw as u32
                    } else if bits_coded > 0 {
                        bits_coded as u32
                    } else {
                        16
                    };
                    let ch_layout: AvChannelLayout = rd(codecpar, AVCODECPAR_CH_LAYOUT_OFF);
                    let channels = ch_layout.nb_channels.max(0) as u32;
                    let sample_rate = rd::<i32>(codecpar, AVCODECPAR_SAMPLE_RATE_OFF).max(0) as f32;
                    let info = TrackInfo::audio(
                        track_id,
                        codec_name,
                        std::ptr::null(),
                        0,
                        bits,
                        channels,
                        sample_rate,
                    );
                    tracks_to_emit.push((info, extra_vec));
                }
            }
        }

        if !self.emitted_headers && !tracks_to_emit.is_empty() {
            let raw_dur = rd::<i64>(fmt_ctx, AVFORMATCTX_DURATION_OFF);
            let duration_secs = if raw_dur != AV_NOPTS_VALUE && raw_dur > 0 {
                (raw_dur as f64) / (AV_TIME_BASE as f64)
            } else {
                fallback_duration_secs
            };
            self.emitted_headers = true;
            self.emit_file_info(self_obj, duration_secs);
            for (mut info, extra_vec) in tracks_to_emit {
                if !extra_vec.is_empty() {
                    info.codec_private = extra_vec.as_ptr();
                    info.codec_private_len = extra_vec.len();
                }
                self.emit_track_info(self_obj, &info);
            }
            self.emit_no_seek_head_once(self_obj);
        }

        // Only demux packets once actual media bytes exist in `self.buf`.
        if !self.buf.is_empty() && !track_map.is_empty() {
            let mut pkt = (ff.av_packet_alloc)();
            if !pkt.is_null() {
                let mut seen_packets = 0usize;
                let mut batch: Vec<OwnedPacket> = Vec::with_capacity(64);

                loop {
                    (ff.av_packet_unref)(pkt);
                    let rc = (ff.av_read_frame)(fmt_ctx, pkt);
                    if rc < 0 {
                        break;
                    }

                    let stream_idx = rd::<i32>(pkt, AVPACKET_STREAM_INDEX_OFF);
                    let Some(&meta) = track_map.get(&stream_idx) else {
                        continue;
                    };

                    let pkt_data = rd::<*mut u8>(pkt, AVPACKET_DATA_OFF);
                    let pkt_size = rd::<i32>(pkt, AVPACKET_SIZE_OFF);
                    let pkt_flags = rd::<i32>(pkt, AVPACKET_FLAGS_OFF);

                    if pkt_data.is_null() || pkt_size <= 0 {
                        continue;
                    }

                    // If streaming hit the end of buffered bytes mid-packet, stop and wait for more data.
                    if (pkt_flags & AV_PKT_FLAG_CORRUPT) != 0
                        && (io_view.hit_gap || io_view.pos >= io_view.buf_len)
                    {
                        break;
                    }

                    if seen_packets < self.emitted_packets {
                        seen_packets += 1;
                        continue;
                    }

                    let pts = rd::<i64>(pkt, AVPACKET_PTS_OFF);
                    let dts = rd::<i64>(pkt, AVPACKET_DTS_OFF);
                    let ts = if pts != AV_NOPTS_VALUE {
                        pts
                    } else if dts != AV_NOPTS_VALUE {
                        dts
                    } else {
                        0
                    };
                    let timestamp_ms = if meta.time_base.den > 0 {
                        ((ts as f64) * (meta.time_base.num as f64) * 1000.0
                            / (meta.time_base.den as f64)) as f32
                    } else {
                        0.0
                    };

                    let data_vec =
                        std::slice::from_raw_parts(pkt_data as *const u8, pkt_size as usize)
                            .to_vec();

                    // Extract optional WebM Matroska BlockAdditional (BlockAddID == 1) alpha payload.
                    let mut alpha_vec = Vec::new();
                    let sd_ptr = rd::<*mut c_void>(pkt, AVPACKET_SIDE_DATA_OFF);
                    let sd_elems = rd::<i32>(pkt, AVPACKET_SIDE_DATA_ELEMS_OFF);
                    if !sd_ptr.is_null() && sd_elems > 0 {
                        for j in 0..(sd_elems as usize) {
                            let entry =
                                (sd_ptr as *const u8).add(j * AVPACKET_SIDEDATA_SIZE) as *const c_void;
                            let sd_type = rd::<i32>(entry, AVPACKET_SIDEDATA_TYPE_OFF);
                            if sd_type == AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL {
                                let sd_data = rd::<*mut u8>(entry, AVPACKET_SIDEDATA_DATA_OFF);
                                let sd_len = rd::<usize>(entry, AVPACKET_SIDEDATA_LEN_OFF);
                                if !sd_data.is_null() && sd_len > 8 {
                                    let raw = std::slice::from_raw_parts(sd_data, sd_len);
                                    let add_id = u64::from_be_bytes([
                                        raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6],
                                        raw[7],
                                    ]);
                                    if add_id == 1 {
                                        alpha_vec.extend_from_slice(&raw[8..]);
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    seen_packets += 1;
                    self.emitted_packets += 1;
                    batch.push(OwnedPacket {
                        data: data_vec,
                        alpha: alpha_vec,
                        track_id: meta.track_id,
                        timestamp_ms,
                        is_keyframe: if (pkt_flags & AV_PKT_FLAG_KEY) != 0 { 1 } else { 0 },
                    });

                    if batch.len() >= 64 {
                        self.emit_frames(self_obj, &batch);
                        batch.clear();
                    }
                }

                if !batch.is_empty() {
                    self.emit_frames(self_obj, &batch);
                }
                (ff.av_packet_free)(&mut pkt);
            }
        }

        (ff.avformat_close_input)(&mut fmt_ctx);
        (ff.avio_context_free)(&mut avio_ctx);
        drop(io_view);
    }
}

// ---------------------------------------------------------------------------
// IDemuxer C++ vtable (7 slots: [0..7])
// ---------------------------------------------------------------------------

unsafe extern "win64" fn demuxer_scalar_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    if !this.is_null() {
        let obj = this as *mut DemuxerObject;
        if (flags & 1) != 0 {
            let _ = Box::from_raw(obj);
        } else {
            (*obj).inner.reset();
        }
    }
    this
}

unsafe extern "win64" fn demuxer_destroy(this: *mut c_void) {
    if !this.is_null() {
        let _ = Box::from_raw(this as *mut DemuxerObject);
    }
}

unsafe extern "win64" fn demuxer_set_playback_callbacks(this: *mut c_void, cb: *mut c_void) {
    if !this.is_null() {
        (*(this as *mut DemuxerObject)).inner.playback_cb = cb;
    }
}

unsafe extern "win64" fn demuxer_set_seek_callbacks(this: *mut c_void, cb: *mut c_void) {
    if !this.is_null() {
        (*(this as *mut DemuxerObject)).inner.seek_cb = cb;
    }
}

unsafe extern "win64" fn demuxer_demux(this: *mut c_void, data: *const u8, len: usize) {
    if !this.is_null() {
        (*(this as *mut DemuxerObject))
            .inner
            .demux_chunk(this, data, len);
    }
}

unsafe extern "win64" fn demuxer_demux_moov(this: *mut c_void, data: *const u8, len: usize) {
    if !this.is_null() {
        (*(this as *mut DemuxerObject))
            .inner
            .demux_moov(this, data, len);
    }
}

unsafe extern "win64" fn demuxer_reset(this: *mut c_void) {
    if !this.is_null() {
        (*(this as *mut DemuxerObject)).inner.reset();
    }
}

pub static DEMUXER_VTBL: [Ptr; 7] = [
    Ptr(demuxer_scalar_dtor as *const ()),            // 0 (+0x00): scalar deleting dtor
    Ptr(demuxer_destroy as *const ()),                // 1 (+0x08): Destroy
    Ptr(demuxer_set_playback_callbacks as *const ()), // 2 (+0x10): SetPlaybackCallbacks
    Ptr(demuxer_set_seek_callbacks as *const ()),     // 3 (+0x18): SetSeekCallbacks
    Ptr(demuxer_demux as *const ()),                  // 4 (+0x20): Demux
    Ptr(demuxer_demux_moov as *const ()),             // 5 (+0x28): DemuxMoov
    Ptr(demuxer_reset as *const ()),                  // 6 (+0x30): Reset
];

static REGISTER_DEMUXER_VTBL: Once = Once::new();

fn ensure_vtable_registered() {
    REGISTER_DEMUXER_VTBL.call_once(|| {
        for p in &DEMUXER_VTBL {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

/// Allocate a new native `IDemuxer` object for `kind`, or return null if FFmpeg is unavailable.
pub fn create_demuxer(kind: ContainerKind) -> *mut c_void {
    if ffmpeg::ffmpeg().is_none() {
        log::warn!("mediadecoders: cannot create {:?} demuxer (FFmpeg unavailable)", kind);
        return std::ptr::null_mut();
    }
    ensure_vtable_registered();
    let obj = Box::new(DemuxerObject {
        vtbl: &DEMUXER_VTBL,
        inner: DemuxerInner::new(kind),
    });
    Box::into_raw(obj) as *mut c_void
}
