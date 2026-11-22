//! Native `IDecoder` C++ objects (H.264, VP8, VP9 + optional alpha, Vorbis, MP3, AAC)
//! backed directly by native ARM64 FFmpeg `libavcodec` + `libswscale` + `libswresample`.

use super::ffmpeg::{
    self, field_ptr, field_ptr_mut, rd, wr, AvChannelLayout, FfmpegApi, AVCODECCTX_CH_LAYOUT_OFF,
    AVCODECCTX_EXTRADATA_OFF, AVCODECCTX_EXTRADATA_SIZE_OFF, AVCODECCTX_SAMPLE_RATE_OFF,
    AVCODECCTX_THREAD_COUNT_OFF, AVERROR_EAGAIN, AVERROR_EOF, AVFRAME_CH_LAYOUT_OFF,
    AVFRAME_DATA_OFF, AVFRAME_EXTENDED_DATA_OFF, AVFRAME_FORMAT_OFF, AVFRAME_HEIGHT_OFF,
    AVFRAME_LINESIZE_OFF, AVFRAME_NB_SAMPLES_OFF, AVFRAME_PTS_OFF, AVFRAME_SAMPLE_RATE_OFF,
    AVFRAME_WIDTH_OFF, AVPACKET_DATA_OFF, AVPACKET_DTS_OFF, AVPACKET_PTS_OFF, AV_CODEC_ID_AAC,
    AV_CODEC_ID_H264, AV_CODEC_ID_MP3, AV_CODEC_ID_VORBIS, AV_CODEC_ID_VP8, AV_CODEC_ID_VP9,
    AV_INPUT_BUFFER_PADDING_SIZE, AV_NOPTS_VALUE, AV_PIX_FMT_YUV420P, AV_PIX_FMT_YUVA420P,
    AV_PIX_FMT_YUVJ420P, AV_SAMPLE_FMT_DBL, AV_SAMPLE_FMT_DBLP, AV_SAMPLE_FMT_FLT,
    AV_SAMPLE_FMT_FLTP, AV_SAMPLE_FMT_S16, AV_SAMPLE_FMT_S16P, AV_SAMPLE_FMT_S32,
    AV_SAMPLE_FMT_S32P, AV_SAMPLE_FMT_U8, AV_SAMPLE_FMT_U8P, SWS_BILINEAR,
};
use super::{read_guest_vfunc, AudioFrameData, AudioStreamInfo, VideoFrameData};
use crate::Ptr;
use std::ffi::{c_void, CString};
use std::sync::Once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecKind {
    H264,
    Vp8,
    Vp9,
    Vorbis,
    Mp3,
    Aac,
}

impl CodecKind {
    pub const fn av_codec_id(self) -> i32 {
        match self {
            Self::H264 => AV_CODEC_ID_H264,
            Self::Vp8 => AV_CODEC_ID_VP8,
            Self::Vp9 => AV_CODEC_ID_VP9,
            Self::Vorbis => AV_CODEC_ID_VORBIS,
            Self::Mp3 => AV_CODEC_ID_MP3,
            Self::Aac => AV_CODEC_ID_AAC,
        }
    }

    pub const fn is_audio(self) -> bool {
        matches!(self, Self::Vorbis | Self::Mp3 | Self::Aac)
    }

    pub const fn supports_side_alpha(self) -> bool {
        matches!(self, Self::Vp8 | Self::Vp9)
    }
}

/// C++ `IDecoder` object exposed across the Win64 ABI boundary.
///
/// `+0x00` holds the 6-slot vtable pointer `[dtor, Destroy, SetCallbacks, Initialize, DecodeFrame, IsAsync]`.
#[repr(C)]
pub struct DecoderObject {
    pub vtbl: *const [Ptr; 6],
    pub inner: DecoderInner,
}

pub struct DecoderInner {
    kind: CodecKind,
    threads: u32,
    callbacks: *mut c_void,

    // Primary FFmpeg decoder state
    codec_ctx: *mut c_void,
    pkt: *mut c_void,
    frame: *mut c_void,

    // Optional secondary VPX alpha decoder state
    alpha_ctx: *mut c_void,
    alpha_pkt: *mut c_void,
    alpha_frame: *mut c_void,

    // Cached libswscale converter (for non-YUV420P video frames)
    sws_ctx: *mut c_void,
    sws_src_w: i32,
    sws_src_h: i32,
    sws_src_fmt: i32,
    yuv_y: Vec<u8>,
    yuv_u: Vec<u8>,
    yuv_v: Vec<u8>,

    // Cached libswresample converter + planar f32 output storage
    swr_ctx: *mut c_void,
    swr_in_fmt: i32,
    swr_in_rate: i32,
    swr_in_channels: i32,
    audio_planes: Vec<Vec<f32>>,
    audio_plane_ptrs: Vec<*const f32>,

    // Stored configuration for lazy/re-open
    extradata: Vec<u8>,
    hint_channels: u16,
    hint_sample_rate: u32,
}

impl Drop for DecoderInner {
    fn drop(&mut self) {
        unsafe {
            self.cleanup();
        }
    }
}

impl DecoderInner {
    fn new(kind: CodecKind, threads: u32) -> Self {
        Self {
            kind,
            threads,
            callbacks: std::ptr::null_mut(),
            codec_ctx: std::ptr::null_mut(),
            pkt: std::ptr::null_mut(),
            frame: std::ptr::null_mut(),
            alpha_ctx: std::ptr::null_mut(),
            alpha_pkt: std::ptr::null_mut(),
            alpha_frame: std::ptr::null_mut(),
            sws_ctx: std::ptr::null_mut(),
            sws_src_w: 0,
            sws_src_h: 0,
            sws_src_fmt: -1,
            yuv_y: Vec::new(),
            yuv_u: Vec::new(),
            yuv_v: Vec::new(),
            swr_ctx: std::ptr::null_mut(),
            swr_in_fmt: -1,
            swr_in_rate: 0,
            swr_in_channels: 0,
            audio_planes: Vec::new(),
            audio_plane_ptrs: Vec::new(),
            extradata: Vec::new(),
            hint_channels: 0,
            hint_sample_rate: 0,
        }
    }

    pub(crate) unsafe fn cleanup(&mut self) {
        let Some(ff) = ffmpeg::ffmpeg() else {
            return;
        };
        if !self.sws_ctx.is_null() {
            if let Some(sws) = &ff.swscale {
                (sws.sws_freeContext)(self.sws_ctx);
            }
            self.sws_ctx = std::ptr::null_mut();
        }
        if !self.swr_ctx.is_null() {
            if let Some(swr) = &ff.swresample {
                (swr.swr_free)(&mut self.swr_ctx);
            }
            self.swr_ctx = std::ptr::null_mut();
        }
        if !self.alpha_frame.is_null() {
            (ff.av_frame_free)(&mut self.alpha_frame);
        }
        if !self.alpha_pkt.is_null() {
            (ff.av_packet_free)(&mut self.alpha_pkt);
        }
        if !self.alpha_ctx.is_null() {
            (ff.avcodec_free_context)(&mut self.alpha_ctx);
        }
        if !self.frame.is_null() {
            (ff.av_frame_free)(&mut self.frame);
        }
        if !self.pkt.is_null() {
            (ff.av_packet_free)(&mut self.pkt);
        }
        if !self.codec_ctx.is_null() {
            (ff.avcodec_free_context)(&mut self.codec_ctx);
        }
    }

    unsafe fn free_primary_codec(&mut self, ff: &FfmpegApi) {
        if !self.codec_ctx.is_null() {
            (ff.avcodec_free_context)(&mut self.codec_ctx);
        }
    }

    unsafe fn open_codec_ctx(
        ff: &FfmpegApi,
        kind: CodecKind,
        threads: u32,
        extradata: &[u8],
        hint_channels: u16,
        hint_sample_rate: u32,
    ) -> Result<*mut c_void, String> {
        let codec = (ff.avcodec_find_decoder)(kind.av_codec_id());
        if codec.is_null() {
            return Err(format!("FFmpeg decoder not found for {:?}", kind));
        }
        let mut ctx = (ff.avcodec_alloc_context3)(codec);
        if ctx.is_null() {
            return Err("avcodec_alloc_context3 returned null".into());
        }

        if !kind.is_audio() && threads > 0 {
            wr::<i32>(ctx, AVCODECCTX_THREAD_COUNT_OFF, threads.min(16) as i32);
        }

        if kind.is_audio() {
            if hint_sample_rate > 0 {
                wr::<i32>(ctx, AVCODECCTX_SAMPLE_RATE_OFF, hint_sample_rate as i32);
            }
            if hint_channels > 0 {
                let ch_ptr = field_ptr_mut::<AvChannelLayout>(ctx, AVCODECCTX_CH_LAYOUT_OFF);
                (ff.av_channel_layout_default)(ch_ptr, hint_channels as i32);
            }
        }

        if !extradata.is_empty() {
            let alloc_len = extradata.len() + AV_INPUT_BUFFER_PADDING_SIZE;
            let buf = (ff.av_mallocz)(alloc_len) as *mut u8;
            if buf.is_null() {
                (ff.avcodec_free_context)(&mut ctx);
                return Err("av_mallocz failed for codec extradata".into());
            }
            std::ptr::copy_nonoverlapping(extradata.as_ptr(), buf, extradata.len());
            wr::<*mut u8>(ctx, AVCODECCTX_EXTRADATA_OFF, buf);
            wr::<i32>(ctx, AVCODECCTX_EXTRADATA_SIZE_OFF, extradata.len() as i32);
        }

        let rc = (ff.avcodec_open2)(ctx, codec, std::ptr::null_mut());
        if rc < 0 {
            let msg = format!("avcodec_open2({:?}) failed: {}", kind, ff.err_string(rc));
            (ff.avcodec_free_context)(&mut ctx);
            return Err(msg);
        }

        Ok(ctx)
    }

    unsafe fn ensure_open(&mut self) -> Result<&'static FfmpegApi, String> {
        let ff = ffmpeg::ffmpeg().ok_or_else(|| "FFmpeg runtime unavailable".to_string())?;
        if self.pkt.is_null() {
            self.pkt = (ff.av_packet_alloc)();
            if self.pkt.is_null() {
                return Err("av_packet_alloc failed".into());
            }
        }
        if self.frame.is_null() {
            self.frame = (ff.av_frame_alloc)();
            if self.frame.is_null() {
                return Err("av_frame_alloc failed".into());
            }
        }
        if self.codec_ctx.is_null() {
            self.codec_ctx = Self::open_codec_ctx(
                ff,
                self.kind,
                self.threads,
                &self.extradata,
                self.hint_channels,
                self.hint_sample_rate,
            )?;
        }
        Ok(ff)
    }

    unsafe fn ensure_alpha_open(&mut self, ff: &FfmpegApi) -> Result<(), String> {
        if self.alpha_pkt.is_null() {
            self.alpha_pkt = (ff.av_packet_alloc)();
            if self.alpha_pkt.is_null() {
                return Err("av_packet_alloc (alpha) failed".into());
            }
        }
        if self.alpha_frame.is_null() {
            self.alpha_frame = (ff.av_frame_alloc)();
            if self.alpha_frame.is_null() {
                return Err("av_frame_alloc (alpha) failed".into());
            }
        }
        if self.alpha_ctx.is_null() {
            self.alpha_ctx = Self::open_codec_ctx(ff, self.kind, 1, &[], 0, 0)?;
        }
        Ok(())
    }

    unsafe fn emit_error(&self, self_obj: *mut c_void, msg: &str) {
        log::warn!("mediadecoders ({:?}): {msg}", self.kind);
        let on_error = read_guest_vfunc(self.callbacks, 3);
        if on_error != 0 {
            let cmsg = CString::new(msg).unwrap_or_else(|_| CString::new("decoder error").unwrap());
            let _ = crate::call_guest8(
                on_error,
                self.callbacks as u64,
                self_obj as u64,
                cmsg.as_ptr() as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn emit_video_frame(&self, self_obj: *mut c_void, frame: &VideoFrameData) {
        let on_video = read_guest_vfunc(self.callbacks, 1);
        if on_video != 0 {
            let _ = crate::call_guest8(
                on_video,
                self.callbacks as u64,
                self_obj as u64,
                (frame as *const VideoFrameData) as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    unsafe fn emit_audio_frame(&self, self_obj: *mut c_void, frame: &AudioFrameData) {
        let on_audio = read_guest_vfunc(self.callbacks, 2);
        if on_audio != 0 {
            let _ = crate::call_guest8(
                on_audio,
                self.callbacks as u64,
                self_obj as u64,
                (frame as *const AudioFrameData) as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }

    pub(crate) unsafe fn initialize(
        &mut self,
        self_obj: *mut c_void,
        config_ptr: *const u8,
        config_len: usize,
    ) {
        self.extradata.clear();
        self.hint_channels = 0;
        self.hint_sample_rate = 0;

        if !config_ptr.is_null() && config_len > 0 {
            let slice = std::slice::from_raw_parts(config_ptr, config_len);
            let parsed_stream_info = if matches!(self.kind, CodecKind::Mp3 | CodecKind::Aac)
                && config_len == std::mem::size_of::<AudioStreamInfo>()
            {
                let info = std::ptr::read_unaligned(config_ptr as *const AudioStreamInfo);
                if info._pad == 0
                    && (1..=8).contains(&info.channels)
                    && (8_000..=192_000).contains(&info.sample_rate)
                {
                    self.hint_channels = info.channels;
                    self.hint_sample_rate = info.sample_rate;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if !parsed_stream_info {
                self.extradata.extend_from_slice(slice);
            }
        } else if !self.kind.is_audio() {
            // H.264 / VPX with empty config is a no-op until first DecodeFrame.
            return;
        }

        if let Some(ff) = ffmpeg::ffmpeg() {
            self.free_primary_codec(ff);
            if let Err(e) = self.ensure_open() {
                self.emit_error(self_obj, &e);
            }
        }
    }

    unsafe fn decode_alpha_packet(
        &mut self,
        ff: &FfmpegApi,
        alpha_bytes: &[u8],
        timestamp_ms: u32,
    ) -> bool {
        if self.ensure_alpha_open(ff).is_err() {
            return false;
        }
        (ff.av_packet_unref)(self.alpha_pkt);
        if (ff.av_new_packet)(self.alpha_pkt, alpha_bytes.len() as i32) < 0 {
            return false;
        }
        let dst = rd::<*mut u8>(self.alpha_pkt, AVPACKET_DATA_OFF);
        if dst.is_null() {
            return false;
        }
        std::ptr::copy_nonoverlapping(alpha_bytes.as_ptr(), dst, alpha_bytes.len());
        wr::<i64>(self.alpha_pkt, AVPACKET_PTS_OFF, timestamp_ms as i64);
        wr::<i64>(self.alpha_pkt, AVPACKET_DTS_OFF, timestamp_ms as i64);

        let send_rc = (ff.avcodec_send_packet)(self.alpha_ctx, self.alpha_pkt);
        (ff.av_packet_unref)(self.alpha_pkt);
        if send_rc < 0 {
            return false;
        }
        (ff.av_frame_unref)(self.alpha_frame);
        (ff.avcodec_receive_frame)(self.alpha_ctx, self.alpha_frame) == 0
    }

    unsafe fn deliver_video_frame(
        &mut self,
        ff: &FfmpegApi,
        self_obj: *mut c_void,
        fallback_ts_ms: u32,
        has_side_alpha: bool,
    ) -> Result<(), String> {
        let width = rd::<i32>(self.frame, AVFRAME_WIDTH_OFF);
        let height = rd::<i32>(self.frame, AVFRAME_HEIGHT_OFF);
        let format = rd::<i32>(self.frame, AVFRAME_FORMAT_OFF);
        let pts = rd::<i64>(self.frame, AVFRAME_PTS_OFF);
        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let data: [*mut u8; 8] = rd(self.frame, AVFRAME_DATA_OFF);
        let linesize: [i32; 8] = rd(self.frame, AVFRAME_LINESIZE_OFF);

        let timestamp_ms = if pts != AV_NOPTS_VALUE && pts >= 0 {
            pts as u32
        } else {
            fallback_ts_ms
        };

        let mut a_plane = std::ptr::null();
        let mut a_stride = 0u32;

        if format == AV_PIX_FMT_YUVA420P && !data[3].is_null() && linesize[3] > 0 {
            a_plane = data[3] as *const u8;
            a_stride = linesize[3] as u32;
        } else if has_side_alpha && !self.alpha_frame.is_null() {
            let aw = rd::<i32>(self.alpha_frame, AVFRAME_WIDTH_OFF);
            let ah = rd::<i32>(self.alpha_frame, AVFRAME_HEIGHT_OFF);
            let adata: [*mut u8; 8] = rd(self.alpha_frame, AVFRAME_DATA_OFF);
            let alines: [i32; 8] = rd(self.alpha_frame, AVFRAME_LINESIZE_OFF);
            if aw == width && ah == height && !adata[0].is_null() && alines[0] > 0 {
                a_plane = adata[0] as *const u8;
                a_stride = alines[0] as u32;
            }
        }

        let direct_planar_yuv420 = (format == AV_PIX_FMT_YUV420P
            || format == AV_PIX_FMT_YUVJ420P
            || format == AV_PIX_FMT_YUVA420P)
            && !data[0].is_null()
            && !data[1].is_null()
            && !data[2].is_null()
            && linesize[0] > 0
            && linesize[1] > 0
            && linesize[2] > 0;

        if direct_planar_yuv420 {
            let vf = VideoFrameData {
                width: width as u32,
                height: height as u32,
                display_width: width as u32,
                display_height: height as u32,
                y_plane: data[0] as *const u8,
                u_plane: data[1] as *const u8,
                v_plane: data[2] as *const u8,
                a_plane,
                y_stride: linesize[0] as u32,
                u_stride: linesize[1] as u32,
                v_stride: linesize[2] as u32,
                a_stride,
                timestamp_ms,
                _pad: 0,
            };
            self.emit_video_frame(self_obj, &vf);
            return Ok(());
        }

        // Convert arbitrary pixel formats (NV12, 10-bit, YUV422P, RGB, etc.) to YUV420P via libswscale.
        let sws = ff
            .swscale
            .as_ref()
            .ok_or_else(|| format!("libswscale unavailable to convert pix_fmt {format}"))?;

        if self.sws_ctx.is_null()
            || self.sws_src_w != width
            || self.sws_src_h != height
            || self.sws_src_fmt != format
        {
            if !self.sws_ctx.is_null() {
                (sws.sws_freeContext)(self.sws_ctx);
                self.sws_ctx = std::ptr::null_mut();
            }
            self.sws_ctx = (sws.sws_getContext)(
                width,
                height,
                format,
                width,
                height,
                AV_PIX_FMT_YUV420P,
                SWS_BILINEAR,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            );
            if self.sws_ctx.is_null() {
                return Err(format!("sws_getContext failed for {width}x{height} fmt={format}"));
            }
            self.sws_src_w = width;
            self.sws_src_h = height;
            self.sws_src_fmt = format;
        }

        let w = width as usize;
        let h = height as usize;
        let uv_w = (w + 1) / 2;
        let uv_h = (h + 1) / 2;
        self.yuv_y.resize(w * h, 0);
        self.yuv_u.resize(uv_w * uv_h, 0x80);
        self.yuv_v.resize(uv_w * uv_h, 0x80);

        let src_ptrs: [*const u8; 4] = [
            data[0] as *const u8,
            data[1] as *const u8,
            data[2] as *const u8,
            data[3] as *const u8,
        ];
        let src_strides: [i32; 4] = [linesize[0], linesize[1], linesize[2], linesize[3]];
        let dst_ptrs: [*mut u8; 4] = [
            self.yuv_y.as_mut_ptr(),
            self.yuv_u.as_mut_ptr(),
            self.yuv_v.as_mut_ptr(),
            std::ptr::null_mut(),
        ];
        let dst_strides: [i32; 4] = [w as i32, uv_w as i32, uv_w as i32, 0];

        let scaled_h = (sws.sws_scale)(
            self.sws_ctx,
            src_ptrs.as_ptr(),
            src_strides.as_ptr(),
            0,
            height,
            dst_ptrs.as_ptr(),
            dst_strides.as_ptr(),
        );
        if scaled_h <= 0 {
            return Err(format!("sws_scale failed (ret={scaled_h})"));
        }

        let vf = VideoFrameData {
            width: width as u32,
            height: height as u32,
            display_width: width as u32,
            display_height: height as u32,
            y_plane: self.yuv_y.as_ptr(),
            u_plane: self.yuv_u.as_ptr(),
            v_plane: self.yuv_v.as_ptr(),
            a_plane,
            y_stride: w as u32,
            u_stride: uv_w as u32,
            v_stride: uv_w as u32,
            a_stride,
            timestamp_ms,
            _pad: 0,
        };
        self.emit_video_frame(self_obj, &vf);
        Ok(())
    }

    unsafe fn deliver_audio_frame(
        &mut self,
        ff: &FfmpegApi,
        self_obj: *mut c_void,
        fallback_ts_ms: u32,
    ) -> Result<(), String> {
        let nb_samples = rd::<i32>(self.frame, AVFRAME_NB_SAMPLES_OFF);
        let format = rd::<i32>(self.frame, AVFRAME_FORMAT_OFF);
        let pts = rd::<i64>(self.frame, AVFRAME_PTS_OFF);
        let sample_rate = rd::<i32>(self.frame, AVFRAME_SAMPLE_RATE_OFF);
        let ch_layout: AvChannelLayout = rd(self.frame, AVFRAME_CH_LAYOUT_OFF);
        let channels = ch_layout.nb_channels;

        if nb_samples <= 0 || channels <= 0 {
            return Ok(());
        }

        let timestamp_ms = if pts != AV_NOPTS_VALUE && pts >= 0 {
            pts as u32
        } else {
            fallback_ts_ms
        };

        let ch_count = channels as usize;
        let sample_count = nb_samples as usize;
        let ext_data = rd::<*mut *mut u8>(self.frame, AVFRAME_EXTENDED_DATA_OFF);
        let data: [*mut u8; 8] = rd(self.frame, AVFRAME_DATA_OFF);

        let plane_ptr = |c: usize| -> *const u8 {
            if !ext_data.is_null() {
                *ext_data.add(c) as *const u8
            } else if c < 8 {
                data[c] as *const u8
            } else {
                std::ptr::null()
            }
        };

        // Fast path: already planar f32 (AV_SAMPLE_FMT_FLTP).
        if format == AV_SAMPLE_FMT_FLTP {
            self.audio_plane_ptrs.clear();
            self.audio_plane_ptrs.reserve(ch_count);
            for c in 0..ch_count {
                let p = plane_ptr(c);
                if p.is_null() {
                    return Err("null audio plane pointer in FLTP frame".into());
                }
                self.audio_plane_ptrs.push(p as *const f32);
            }
            let af = AudioFrameData {
                samples_per_channel: nb_samples,
                channels: channels as u32,
                channel_ptrs: self.audio_plane_ptrs.as_ptr(),
                timestamp_ms,
                _pad: 0,
            };
            self.emit_audio_frame(self_obj, &af);
            return Ok(());
        }

        // Prepare owned planar f32 output buffers.
        if self.audio_planes.len() < ch_count {
            self.audio_planes.resize_with(ch_count, Vec::new);
        }
        for plane in self.audio_planes.iter_mut().take(ch_count) {
            plane.resize(sample_count, 0.0);
        }

        let mut converted_direct = true;
        match format {
            AV_SAMPLE_FMT_FLT => {
                let src = plane_ptr(0) as *const f32;
                if src.is_null() {
                    return Err("null audio buffer in FLT frame".into());
                }
                for i in 0..sample_count {
                    for c in 0..ch_count {
                        self.audio_planes[c][i] =
                            std::ptr::read_unaligned(src.add(i * ch_count + c));
                    }
                }
            }
            AV_SAMPLE_FMT_S16 => {
                let src = plane_ptr(0) as *const i16;
                if src.is_null() {
                    return Err("null audio buffer in S16 frame".into());
                }
                const SCALE: f32 = 1.0 / 32768.0;
                for i in 0..sample_count {
                    for c in 0..ch_count {
                        let s = std::ptr::read_unaligned(src.add(i * ch_count + c));
                        self.audio_planes[c][i] = (s as f32) * SCALE;
                    }
                }
            }
            AV_SAMPLE_FMT_S16P => {
                const SCALE: f32 = 1.0 / 32768.0;
                for c in 0..ch_count {
                    let src = plane_ptr(c) as *const i16;
                    if src.is_null() {
                        return Err("null audio plane in S16P frame".into());
                    }
                    let dst = &mut self.audio_planes[c];
                    for i in 0..sample_count {
                        dst[i] = (std::ptr::read_unaligned(src.add(i)) as f32) * SCALE;
                    }
                }
            }
            AV_SAMPLE_FMT_S32 => {
                let src = plane_ptr(0) as *const i32;
                if src.is_null() {
                    return Err("null audio buffer in S32 frame".into());
                }
                const SCALE: f32 = 1.0 / 2_147_483_648.0;
                for i in 0..sample_count {
                    for c in 0..ch_count {
                        let s = std::ptr::read_unaligned(src.add(i * ch_count + c));
                        self.audio_planes[c][i] = (s as f32) * SCALE;
                    }
                }
            }
            AV_SAMPLE_FMT_S32P => {
                const SCALE: f32 = 1.0 / 2_147_483_648.0;
                for c in 0..ch_count {
                    let src = plane_ptr(c) as *const i32;
                    if src.is_null() {
                        return Err("null audio plane in S32P frame".into());
                    }
                    let dst = &mut self.audio_planes[c];
                    for i in 0..sample_count {
                        dst[i] = (std::ptr::read_unaligned(src.add(i)) as f32) * SCALE;
                    }
                }
            }
            AV_SAMPLE_FMT_U8 => {
                let src = plane_ptr(0);
                if src.is_null() {
                    return Err("null audio buffer in U8 frame".into());
                }
                for i in 0..sample_count {
                    for c in 0..ch_count {
                        let s = *src.add(i * ch_count + c);
                        self.audio_planes[c][i] = (s as f32 - 128.0) / 128.0;
                    }
                }
            }
            AV_SAMPLE_FMT_U8P => {
                for c in 0..ch_count {
                    let src = plane_ptr(c);
                    if src.is_null() {
                        return Err("null audio plane in U8P frame".into());
                    }
                    let dst = &mut self.audio_planes[c];
                    for i in 0..sample_count {
                        dst[i] = (*src.add(i) as f32 - 128.0) / 128.0;
                    }
                }
            }
            AV_SAMPLE_FMT_DBL => {
                let src = plane_ptr(0) as *const f64;
                if src.is_null() {
                    return Err("null audio buffer in DBL frame".into());
                }
                for i in 0..sample_count {
                    for c in 0..ch_count {
                        self.audio_planes[c][i] =
                            std::ptr::read_unaligned(src.add(i * ch_count + c)) as f32;
                    }
                }
            }
            AV_SAMPLE_FMT_DBLP => {
                for c in 0..ch_count {
                    let src = plane_ptr(c) as *const f64;
                    if src.is_null() {
                        return Err("null audio plane in DBLP frame".into());
                    }
                    let dst = &mut self.audio_planes[c];
                    for i in 0..sample_count {
                        dst[i] = std::ptr::read_unaligned(src.add(i)) as f32;
                    }
                }
            }
            _ => {
                converted_direct = false;
            }
        }

        let mut out_samples = nb_samples;
        if !converted_direct {
            let swr = ff
                .swresample
                .as_ref()
                .ok_or_else(|| format!("libswresample unavailable for sample_fmt {format}"))?;
            let rate = if sample_rate > 0 { sample_rate } else { 48_000 };
            if self.swr_ctx.is_null()
                || self.swr_in_fmt != format
                || self.swr_in_rate != rate
                || self.swr_in_channels != channels
            {
                if !self.swr_ctx.is_null() {
                    (swr.swr_free)(&mut self.swr_ctx);
                }
                let in_layout_ptr = field_ptr::<AvChannelLayout>(self.frame, AVFRAME_CH_LAYOUT_OFF);
                let mut out_layout = AvChannelLayout::default();
                (ff.av_channel_layout_default)(&mut out_layout, channels);
                let rc = (swr.swr_alloc_set_opts2)(
                    &mut self.swr_ctx,
                    &out_layout,
                    AV_SAMPLE_FMT_FLTP,
                    rate,
                    in_layout_ptr,
                    format,
                    rate,
                    0,
                    std::ptr::null_mut(),
                );
                (ff.av_channel_layout_uninit)(&mut out_layout);
                if rc < 0 || self.swr_ctx.is_null() {
                    return Err(format!("swr_alloc_set_opts2 failed: {}", ff.err_string(rc)));
                }
                let init_rc = (swr.swr_init)(self.swr_ctx);
                if init_rc < 0 {
                    (swr.swr_free)(&mut self.swr_ctx);
                    return Err(format!("swr_init failed: {}", ff.err_string(init_rc)));
                }
                self.swr_in_fmt = format;
                self.swr_in_rate = rate;
                self.swr_in_channels = channels;
            }

            let mut out_ptrs: Vec<*mut u8> = self
                .audio_planes
                .iter_mut()
                .take(ch_count)
                .map(|v| v.as_mut_ptr() as *mut u8)
                .collect();
            let in_ptrs = if !ext_data.is_null() {
                ext_data as *const *const u8
            } else {
                data.as_ptr() as *const *const u8
            };
            let converted = (swr.swr_convert)(
                self.swr_ctx,
                out_ptrs.as_mut_ptr(),
                nb_samples,
                in_ptrs,
                nb_samples,
            );
            if converted < 0 {
                return Err(format!("swr_convert failed: {}", ff.err_string(converted)));
            }
            out_samples = converted;
        }

        if out_samples <= 0 {
            return Ok(());
        }

        self.audio_plane_ptrs.clear();
        self.audio_plane_ptrs.reserve(ch_count);
        for plane in self.audio_planes.iter().take(ch_count) {
            self.audio_plane_ptrs.push(plane.as_ptr());
        }

        let af = AudioFrameData {
            samples_per_channel: out_samples,
            channels: channels as u32,
            channel_ptrs: self.audio_plane_ptrs.as_ptr(),
            timestamp_ms,
            _pad: 0,
        };
        self.emit_audio_frame(self_obj, &af);
        Ok(())
    }

    pub(crate) unsafe fn decode_frame(
        &mut self,
        self_obj: *mut c_void,
        data: *const u8,
        len: usize,
        timestamp_ms: u32,
        alpha_len: usize,
    ) -> u8 {
        if data.is_null() && len > 0 {
            self.emit_error(self_obj, "null packet pointer with non-zero length");
            return 0;
        }

        let ff = match self.ensure_open() {
            Ok(ff) => ff,
            Err(e) => {
                self.emit_error(self_obj, &e);
                return 0;
            }
        };

        let is_drain = len == 0;
        if is_drain {
            let rc = (ff.avcodec_send_packet)(self.codec_ctx, std::ptr::null());
            if rc < 0 && rc != AVERROR_EOF {
                self.emit_error(
                    self_obj,
                    &format!("avcodec_send_packet(drain) failed: {}", ff.err_string(rc)),
                );
                return 0;
            }
        } else {
            let pkt_size = match i32::try_from(len) {
                Ok(n) => n,
                Err(_) => {
                    self.emit_error(self_obj, "packet length exceeds i32::MAX");
                    return 0;
                }
            };
            (ff.av_packet_unref)(self.pkt);
            let alloc_rc = (ff.av_new_packet)(self.pkt, pkt_size);
            if alloc_rc < 0 {
                self.emit_error(
                    self_obj,
                    &format!("av_new_packet failed: {}", ff.err_string(alloc_rc)),
                );
                return 0;
            }
            let pkt_dst = rd::<*mut u8>(self.pkt, AVPACKET_DATA_OFF);
            if pkt_dst.is_null() {
                (ff.av_packet_unref)(self.pkt);
                self.emit_error(self_obj, "av_new_packet produced null buffer");
                return 0;
            }
            std::ptr::copy_nonoverlapping(data, pkt_dst, len);
            wr::<i64>(self.pkt, AVPACKET_PTS_OFF, timestamp_ms as i64);
            wr::<i64>(self.pkt, AVPACKET_DTS_OFF, timestamp_ms as i64);

            let send_rc = (ff.avcodec_send_packet)(self.codec_ctx, self.pkt);
            (ff.av_packet_unref)(self.pkt);
            if send_rc < 0 && send_rc != AVERROR_EAGAIN {
                self.emit_error(
                    self_obj,
                    &format!("avcodec_send_packet failed: {}", ff.err_string(send_rc)),
                );
                return 0;
            }
        }

        // Decode optional VPX side alpha packet (stored immediately after `data[..len]`).
        let has_side_alpha = if !is_drain && alpha_len > 0 && self.kind.supports_side_alpha() {
            let alpha_slice = std::slice::from_raw_parts(data.add(len), alpha_len);
            self.decode_alpha_packet(ff, alpha_slice, timestamp_ms)
        } else {
            false
        };

        loop {
            (ff.av_frame_unref)(self.frame);
            let rc = (ff.avcodec_receive_frame)(self.codec_ctx, self.frame);
            if rc == AVERROR_EAGAIN || rc == AVERROR_EOF {
                break;
            }
            if rc < 0 {
                self.emit_error(
                    self_obj,
                    &format!("avcodec_receive_frame failed: {}", ff.err_string(rc)),
                );
                return 0;
            }

            let res = if self.kind.is_audio() {
                self.deliver_audio_frame(ff, self_obj, timestamp_ms)
            } else {
                self.deliver_video_frame(ff, self_obj, timestamp_ms, has_side_alpha)
            };
            if let Err(e) = res {
                self.emit_error(self_obj, &e);
                return 0;
            }
        }

        if is_drain && !self.codec_ctx.is_null() {
            (ff.avcodec_flush_buffers)(self.codec_ctx);
        }

        1
    }
}

// ---------------------------------------------------------------------------
// IDecoder C++ vtable (6 slots: [0..6])
// ---------------------------------------------------------------------------

unsafe extern "win64" fn decoder_scalar_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    if !this.is_null() {
        let obj = this as *mut DecoderObject;
        if (flags & 1) != 0 {
            let _ = Box::from_raw(obj);
        } else {
            (*obj).inner.cleanup();
        }
    }
    this
}

unsafe extern "win64" fn decoder_destroy(this: *mut c_void) {
    if !this.is_null() {
        let _ = Box::from_raw(this as *mut DecoderObject);
    }
}

unsafe extern "win64" fn decoder_set_callbacks(this: *mut c_void, cb: *mut c_void) {
    if !this.is_null() {
        (*(this as *mut DecoderObject)).inner.callbacks = cb;
    }
}

unsafe extern "win64" fn decoder_initialize(
    this: *mut c_void,
    config_ptr: *const u8,
    config_len: usize,
) {
    if !this.is_null() {
        (*(this as *mut DecoderObject))
            .inner
            .initialize(this, config_ptr, config_len);
    }
}

unsafe extern "win64" fn decoder_decode_frame(
    this: *mut c_void,
    data: *const u8,
    len: usize,
    timestamp_ms: u32,
    alpha_len: usize,
) -> u8 {
    if this.is_null() {
        return 0;
    }
    (*(this as *mut DecoderObject))
        .inner
        .decode_frame(this, data, len, timestamp_ms, alpha_len)
}

unsafe extern "win64" fn decoder_is_async(_this: *mut c_void) -> u8 {
    0
}

pub static DECODER_VTBL: [Ptr; 6] = [
    Ptr(decoder_scalar_dtor as *const ()),   // 0 (+0x00): scalar deleting dtor
    Ptr(decoder_destroy as *const ()),       // 1 (+0x08): Destroy
    Ptr(decoder_set_callbacks as *const ()), // 2 (+0x10): SetCallbacks
    Ptr(decoder_initialize as *const ()),    // 3 (+0x18): Initialize
    Ptr(decoder_decode_frame as *const ()),  // 4 (+0x20): DecodeFrame
    Ptr(decoder_is_async as *const ()),      // 5 (+0x28): IsAsync
];

static REGISTER_DECODER_VTBL: Once = Once::new();

fn ensure_vtable_registered() {
    REGISTER_DECODER_VTBL.call_once(|| {
        for p in &DECODER_VTBL {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

/// Allocate a new native `IDecoder` object for `kind`, or return null if FFmpeg / codec is unavailable.
pub fn create_decoder(kind: CodecKind, threads: u32) -> *mut c_void {
    let Some(ff) = ffmpeg::ffmpeg() else {
        log::warn!("mediadecoders: cannot create {:?} decoder (FFmpeg unavailable)", kind);
        return std::ptr::null_mut();
    };
    if !ff.has_decoder(kind.av_codec_id()) {
        log::warn!("mediadecoders: FFmpeg has no decoder for {:?}", kind);
        return std::ptr::null_mut();
    }
    ensure_vtable_registered();
    let obj = Box::new(DecoderObject {
        vtbl: &DECODER_VTBL,
        inner: DecoderInner::new(kind, threads),
    });
    Box::into_raw(obj) as *mut c_void
}
