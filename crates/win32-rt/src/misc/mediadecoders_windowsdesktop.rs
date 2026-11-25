//! Builtin `MediaDecoders.WindowsDesktop.dll` — native ARM64 clean-room HTML5 media
//! demuxer and codec plugin (`cohtml::media::*`).
//!
//! Exports `InitializeAllocatorAndLogger` plus the factory entry points
//! (`CreateWebMDemuxer`, `CreateMP4Demuxer`, `CreateVPXDecoder`, `CreateVorbisDecoder`,
//! `CreateH264Decoder`, `CreateAACDecoder`, `CreateMP3Decoder`) and the 25 C++ base
//! class symbols, backed natively by FFmpeg (`libavformat`, `libavcodec`, `libavutil`,
//! `libswscale`, `libswresample`) loaded dynamically via `dlopen`.

use super::mediadecoders::{
    decoder::{create_decoder, CodecKind},
    demuxer::{create_demuxer, ContainerKind},
    ffmpeg::{self, AV_CODEC_ID_H264},
    store_allocator_and_logger, AllocatorAndLoggerConfig, VpxInfo,
};
use std::ffi::c_void;

pub static ORDINALS: &[(u16, &str)] = &[];

static BASE_VTBL: [crate::Ptr; 16] = [
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
    crate::Ptr(dtor_noop as *const ()),
];

pub(crate) fn register_static_vtables() {
    let dtor_addr = dtor_noop as *const () as usize;
    let ctor_addr = ctor_self as *const () as usize;
    crate::register_native_va(dtor_addr as u64, dtor_addr);
    crate::register_native_va(ctor_addr as u64, ctor_addr);
}

fn ensure_base_vtbl_registered() {
    static REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if REGISTERED.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }
    if crate::has_native_va_registrar() {
        register_static_vtables();
        REGISTERED.store(true, std::sync::atomic::Ordering::Release);
    }
}

pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "??0IDecoder@media@cohtml@@QEAA@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDecoder@media@cohtml@@QEAA@XZ",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDecoderCallbacks@media@cohtml@@QEAA@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDecoderCallbacks@media@cohtml@@QEAA@XZ",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxer@media@cohtml@@QEAA@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxer@media@cohtml@@QEAA@XZ",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxerPlaybackCallbacks@media@cohtml@@QEAA@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxerPlaybackCallbacks@media@cohtml@@QEAA@XZ",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxerSeekCallbacks@media@cohtml@@QEAA@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??0IDemuxerSeekCallbacks@media@cohtml@@QEAA@XZ",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??1IDecoder@media@cohtml@@UEAA@XZ",
        ptr: crate::Ptr(dtor_noop as *const ()),
    },
    crate::Export {
        name: "??1IDecoderCallbacks@media@cohtml@@UEAA@XZ",
        ptr: crate::Ptr(dtor_noop as *const ()),
    },
    crate::Export {
        name: "??1IDemuxer@media@cohtml@@UEAA@XZ",
        ptr: crate::Ptr(dtor_noop as *const ()),
    },
    crate::Export {
        name: "??1IDemuxerPlaybackCallbacks@media@cohtml@@UEAA@XZ",
        ptr: crate::Ptr(dtor_noop as *const ()),
    },
    crate::Export {
        name: "??1IDemuxerSeekCallbacks@media@cohtml@@UEAA@XZ",
        ptr: crate::Ptr(dtor_noop as *const ()),
    },
    crate::Export {
        name: "??4IDecoder@media@cohtml@@QEAAAEAV012@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??4IDecoderCallbacks@media@cohtml@@QEAAAEAV012@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??4IDemuxer@media@cohtml@@QEAAAEAV012@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??4IDemuxerPlaybackCallbacks@media@cohtml@@QEAAAEAV012@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??4IDemuxerSeekCallbacks@media@cohtml@@QEAAAEAV012@AEBV012@@Z",
        ptr: crate::Ptr(ctor_self as *const ()),
    },
    crate::Export {
        name: "??_7IDecoder@media@cohtml@@6B@",
        ptr: crate::Ptr(BASE_VTBL.as_ptr() as *const ()),
    },
    crate::Export {
        name: "??_7IDecoderCallbacks@media@cohtml@@6B@",
        ptr: crate::Ptr(BASE_VTBL.as_ptr() as *const ()),
    },
    crate::Export {
        name: "??_7IDemuxer@media@cohtml@@6B@",
        ptr: crate::Ptr(BASE_VTBL.as_ptr() as *const ()),
    },
    crate::Export {
        name: "??_7IDemuxerPlaybackCallbacks@media@cohtml@@6B@",
        ptr: crate::Ptr(BASE_VTBL.as_ptr() as *const ()),
    },
    crate::Export {
        name: "??_7IDemuxerSeekCallbacks@media@cohtml@@6B@",
        ptr: crate::Ptr(BASE_VTBL.as_ptr() as *const ()),
    },
];

unsafe extern "win64" fn ctor_self(this: *mut usize) -> *mut usize {
    ensure_base_vtbl_registered();
    this
}

unsafe extern "win64" fn dtor_noop(_this: *mut usize) {}

pub unsafe extern "win64" fn InitializeAllocatorAndLogger(
    malloc_fn: usize,
    realloc_fn: usize,
    free_fn: usize,
    log_fn: usize,
    create_task_fn: usize,
    user_data: usize,
) {
    ensure_base_vtbl_registered();
    store_allocator_and_logger(AllocatorAndLoggerConfig {
        malloc_fn,
        realloc_fn,
        free_fn,
        log_fn,
        create_task_fn,
        user_data,
    });
    log::info!(
        "MediaDecoders (builtin): InitializeAllocatorAndLogger(malloc={:#x}, log={:#x}, create_task={:#x})",
        malloc_fn,
        log_fn,
        create_task_fn
    );
}

pub unsafe extern "win64" fn CreateWebMDemuxer(_params: *const c_void) -> *mut c_void {
    create_demuxer(ContainerKind::WebM)
}

pub unsafe extern "win64" fn CreateMP4Demuxer(_params: *const c_void) -> *mut c_void {
    create_demuxer(ContainerKind::Mp4)
}

pub unsafe extern "win64" fn CreateVPXDecoder(params: *const c_void) -> *mut c_void {
    let (kind, threads) = if params.is_null() {
        (CodecKind::Vp9, 0)
    } else {
        let info = std::ptr::read_unaligned(params as *const VpxInfo);
        let kind = if info.version == 8 {
            CodecKind::Vp8
        } else {
            CodecKind::Vp9
        };
        (kind, info.threads)
    };
    create_decoder(kind, threads)
}

pub unsafe extern "win64" fn CreateVorbisDecoder(_params: *const c_void) -> *mut c_void {
    create_decoder(CodecKind::Vorbis, 0)
}

pub unsafe extern "win64" fn CreateH264Decoder(params: *const c_void) -> *mut c_void {
    if !params.is_null() {
        let probe_only = std::ptr::read_unaligned((params as *const u8).add(8) as *const u64);
        if probe_only == 1 {
            return match ffmpeg::ffmpeg() {
                Some(ff) if ff.has_decoder(AV_CODEC_ID_H264) => 1usize as *mut c_void,
                _ => std::ptr::null_mut(),
            };
        }
    }
    create_decoder(CodecKind::H264, 0)
}

pub unsafe extern "win64" fn CreateAACDecoder(_params: *const c_void) -> *mut c_void {
    create_decoder(CodecKind::Aac, 0)
}

pub unsafe extern "win64" fn CreateMP3Decoder(_params: *const c_void) -> *mut c_void {
    create_decoder(CodecKind::Mp3, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::mediadecoders::{
        decoder::{DecoderObject, DECODER_VTBL},
        demuxer::{DemuxerObject, DEMUXER_VTBL},
        load_allocator_and_logger, AudioFrameData, DemuxerFrame, TrackInfo, VideoFrameData,
    };
    use std::ffi::CStr;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    // -----------------------------------------------------------------------
    // Mock C++ callback objects invoked through `crate::call_guest8`
    // -----------------------------------------------------------------------

    #[derive(Clone, Debug)]
    struct RecordedTrack {
        track_type: u32,
        track_id: u64,
        codec_name: String,
        codec_private: Vec<u8>,
        width: u64,
        height: u64,
        bits_per_sample: u32,
        channels: u32,
        sample_rate: f32,
    }

    #[derive(Clone, Debug)]
    struct RecordedPacket {
        data: Vec<u8>,
        alpha: Vec<u8>,
        track_id: u64,
        timestamp_ms: f32,
        is_keyframe: u8,
    }

    #[repr(C)]
    struct MockDemuxCallbacks {
        vtbl: *const [crate::Ptr; 5],
        durations: Vec<f64>,
        tracks: Vec<RecordedTrack>,
        packets: Vec<RecordedPacket>,
        errors: usize,
    }

    unsafe extern "C" fn mock_demux_dtor(
        this: u64,
        _a1: u64,
        _a2: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        this
    }

    unsafe extern "C" fn mock_on_file_info(
        this: u64,
        _demuxer: u64,
        dur_ptr: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDemuxCallbacks);
        let dur = std::ptr::read_unaligned(dur_ptr as *const f64);
        cb.durations.push(dur);
        0
    }

    unsafe extern "C" fn mock_on_track_info(
        this: u64,
        _demuxer: u64,
        track_ptr: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDemuxCallbacks);
        let ti = std::ptr::read_unaligned(track_ptr as *const TrackInfo);
        let end = ti.codec_name.iter().position(|&b| b == 0).unwrap_or(32);
        let codec_name = String::from_utf8_lossy(&ti.codec_name[..end]).into_owned();
        let codec_private = if !ti.codec_private.is_null() && ti.codec_private_len > 0 {
            std::slice::from_raw_parts(ti.codec_private, ti.codec_private_len).to_vec()
        } else {
            Vec::new()
        };
        let (width, height, bits_per_sample, channels, sample_rate) = if ti.track_type == 1 {
            (ti.field0, ti.field1, 0, 0, 0.0)
        } else {
            let bits = (ti.field0 & 0xFFFF_FFFF) as u32;
            let ch = (ti.field0 >> 32) as u32;
            let sr = f32::from_bits((ti.field1 & 0xFFFF_FFFF) as u32);
            (0, 0, bits, ch, sr)
        };
        cb.tracks.push(RecordedTrack {
            track_type: ti.track_type,
            track_id: ti.track_id,
            codec_name,
            codec_private,
            width,
            height,
            bits_per_sample,
            channels,
            sample_rate,
        });
        0
    }

    unsafe extern "C" fn mock_on_frames(
        this: u64,
        _demuxer: u64,
        frames_ptr: u64,
        count: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDemuxCallbacks);
        let slice = std::slice::from_raw_parts(frames_ptr as *const DemuxerFrame, count as usize);
        for f in slice {
            let data = if !f.data.is_null() && f.size > 0 {
                std::slice::from_raw_parts(f.data, f.size).to_vec()
            } else {
                Vec::new()
            };
            let alpha = if !f.alpha_data.is_null() && f.alpha_size > 0 {
                std::slice::from_raw_parts(f.alpha_data, f.alpha_size).to_vec()
            } else {
                Vec::new()
            };
            cb.packets.push(RecordedPacket {
                data,
                alpha,
                track_id: f.track_id,
                timestamp_ms: f.timestamp_ms,
                is_keyframe: f.is_keyframe,
            });
        }
        0
    }

    unsafe extern "C" fn mock_demux_on_error(
        this: u64,
        _demuxer: u64,
        _a2: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDemuxCallbacks);
        cb.errors += 1;
        0
    }

    static MOCK_DEMUX_VTBL: [crate::Ptr; 5] = [
        crate::Ptr(mock_demux_dtor as *const ()),
        crate::Ptr(mock_on_file_info as *const ()),
        crate::Ptr(mock_on_track_info as *const ()),
        crate::Ptr(mock_on_frames as *const ()),
        crate::Ptr(mock_demux_on_error as *const ()),
    ];

    #[repr(C)]
    struct MockSeekCallbacks {
        vtbl: *const [crate::Ptr; 4],
        no_seek_count: usize,
    }

    unsafe extern "C" fn mock_on_no_seek(
        this: u64,
        _demuxer: u64,
        _a2: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockSeekCallbacks);
        cb.no_seek_count += 1;
        0
    }

    static MOCK_SEEK_VTBL: [crate::Ptr; 4] = [
        crate::Ptr(mock_demux_dtor as *const ()),
        crate::Ptr(mock_demux_dtor as *const ()),
        crate::Ptr(mock_demux_dtor as *const ()),
        crate::Ptr(mock_on_no_seek as *const ()),
    ];

    #[derive(Clone, Debug)]
    struct RecordedVideoFrame {
        width: u32,
        height: u32,
        avg_y: f32,
        avg_u: f32,
        avg_v: f32,
        has_alpha: bool,
        timestamp_ms: u32,
    }

    #[derive(Clone, Debug)]
    struct RecordedAudioFrame {
        samples_per_channel: i32,
        channels: u32,
        max_abs: f32,
        all_finite: bool,
        timestamp_ms: u32,
    }

    #[repr(C)]
    struct MockDecoderCallbacks {
        vtbl: *const [crate::Ptr; 4],
        video_frames: Vec<RecordedVideoFrame>,
        audio_frames: Vec<RecordedAudioFrame>,
        errors: Vec<String>,
    }

    unsafe extern "C" fn mock_on_video_frame(
        this: u64,
        _decoder: u64,
        frame_ptr: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDecoderCallbacks);
        let vf = std::ptr::read_unaligned(frame_ptr as *const VideoFrameData);
        assert!(!vf.y_plane.is_null());
        assert!(!vf.u_plane.is_null());
        assert!(!vf.v_plane.is_null());
        assert!(vf.y_stride >= vf.width);
        let uv_w = (vf.width + 1) / 2;
        let uv_h = (vf.height + 1) / 2;
        assert!(vf.u_stride >= uv_w);
        assert!(vf.v_stride >= uv_w);

        let mut sum_y = 0u64;
        for row in 0..vf.height as usize {
            let line = std::slice::from_raw_parts(
                vf.y_plane.add(row * vf.y_stride as usize),
                vf.width as usize,
            );
            sum_y += line.iter().map(|&b| b as u64).sum::<u64>();
        }
        let mut sum_u = 0u64;
        let mut sum_v = 0u64;
        for row in 0..uv_h as usize {
            let ul = std::slice::from_raw_parts(
                vf.u_plane.add(row * vf.u_stride as usize),
                uv_w as usize,
            );
            let vl = std::slice::from_raw_parts(
                vf.v_plane.add(row * vf.v_stride as usize),
                uv_w as usize,
            );
            sum_u += ul.iter().map(|&b| b as u64).sum::<u64>();
            sum_v += vl.iter().map(|&b| b as u64).sum::<u64>();
        }
        let y_pixels = (vf.width as usize * vf.height as usize).max(1) as f32;
        let uv_pixels = (uv_w as usize * uv_h as usize).max(1) as f32;

        cb.video_frames.push(RecordedVideoFrame {
            width: vf.width,
            height: vf.height,
            avg_y: sum_y as f32 / y_pixels,
            avg_u: sum_u as f32 / uv_pixels,
            avg_v: sum_v as f32 / uv_pixels,
            has_alpha: !vf.a_plane.is_null() && vf.a_stride >= vf.width,
            timestamp_ms: vf.timestamp_ms,
        });
        0
    }

    unsafe extern "C" fn mock_on_audio_frame(
        this: u64,
        _decoder: u64,
        frame_ptr: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDecoderCallbacks);
        let af = std::ptr::read_unaligned(frame_ptr as *const AudioFrameData);
        assert!(af.samples_per_channel > 0);
        assert!(af.channels > 0);
        assert!(!af.channel_ptrs.is_null());

        let ptrs = std::slice::from_raw_parts(af.channel_ptrs, af.channels as usize);
        let mut max_abs = 0.0f32;
        let mut all_finite = true;
        for &p in ptrs {
            assert!(!p.is_null());
            let samples = std::slice::from_raw_parts(p, af.samples_per_channel as usize);
            for &s in samples {
                if !s.is_finite() {
                    all_finite = false;
                } else {
                    max_abs = max_abs.max(s.abs());
                }
            }
        }
        cb.audio_frames.push(RecordedAudioFrame {
            samples_per_channel: af.samples_per_channel,
            channels: af.channels,
            max_abs,
            all_finite,
            timestamp_ms: af.timestamp_ms,
        });
        0
    }

    unsafe extern "C" fn mock_decoder_on_error(
        this: u64,
        _decoder: u64,
        msg_ptr: u64,
        _a3: u64,
        _a4: u64,
        _a5: u64,
        _a6: u64,
        _a7: u64,
    ) -> u64 {
        let cb = &mut *(this as *mut MockDecoderCallbacks);
        let msg = if msg_ptr != 0 {
            CStr::from_ptr(msg_ptr as *const libc::c_char)
                .to_string_lossy()
                .into_owned()
        } else {
            String::new()
        };
        cb.errors.push(msg);
        0
    }

    static MOCK_DECODER_VTBL: [crate::Ptr; 4] = [
        crate::Ptr(mock_demux_dtor as *const ()),
        crate::Ptr(mock_on_video_frame as *const ()),
        crate::Ptr(mock_on_audio_frame as *const ()),
        crate::Ptr(mock_decoder_on_error as *const ()),
    ];

    // Helpers to call through actual C++ vtable slots (verifying exact slot offsets!).
    unsafe fn vcall_demuxer_set_playback(obj: *mut c_void, cb: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) = std::mem::transmute(*vtbl.add(2));
        f(obj, cb);
    }
    unsafe fn vcall_demuxer_set_seek(obj: *mut c_void, cb: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) = std::mem::transmute(*vtbl.add(3));
        f(obj, cb);
    }
    unsafe fn vcall_demuxer_demux(obj: *mut c_void, data: &[u8]) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *const u8, usize) =
            std::mem::transmute(*vtbl.add(4));
        f(obj, data.as_ptr(), data.len());
    }
    unsafe fn vcall_demuxer_demux_moov(obj: *mut c_void, data: &[u8]) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *const u8, usize) =
            std::mem::transmute(*vtbl.add(5));
        f(obj, data.as_ptr(), data.len());
    }
    unsafe fn vcall_demuxer_reset(obj: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(*vtbl.add(6));
        f(obj);
    }
    unsafe fn vcall_demuxer_destroy(obj: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(*vtbl.add(1));
        f(obj);
    }

    unsafe fn vcall_decoder_set_callbacks(obj: *mut c_void, cb: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) = std::mem::transmute(*vtbl.add(2));
        f(obj, cb);
    }
    unsafe fn vcall_decoder_init(obj: *mut c_void, cfg: &[u8]) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *const u8, usize) =
            std::mem::transmute(*vtbl.add(3));
        let ptr = if cfg.is_empty() {
            std::ptr::null()
        } else {
            cfg.as_ptr()
        };
        f(obj, ptr, cfg.len());
    }
    unsafe fn vcall_decoder_decode(
        obj: *mut c_void,
        data: &[u8],
        ts_ms: u32,
        alpha_len: usize,
    ) -> u8 {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, *const u8, usize, u32, usize) -> u8 =
            std::mem::transmute(*vtbl.add(4));
        let ptr = if data.is_empty() {
            std::ptr::null()
        } else {
            data.as_ptr()
        };
        f(obj, ptr, data.len().saturating_sub(alpha_len), ts_ms, alpha_len)
    }
    unsafe fn vcall_decoder_is_async(obj: *mut c_void) -> u8 {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void) -> u8 = std::mem::transmute(*vtbl.add(5));
        f(obj)
    }
    unsafe fn vcall_decoder_destroy(obj: *mut c_void) {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(*vtbl.add(1));
        f(obj);
    }
    unsafe fn vcall_scalar_dtor(obj: *mut c_void, flags: u32) -> *mut c_void {
        let vtbl = *(obj as *const *const usize);
        let f: unsafe extern "C" fn(*mut c_void, u32) -> *mut c_void =
            std::mem::transmute(*vtbl.add(0));
        f(obj, flags)
    }

    fn find_ffmpeg_cli() -> Option<PathBuf> {
        for cand in [
            "/opt/homebrew/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "ffmpeg",
        ] {
            if Command::new(cand).arg("-version").output().is_ok_and(|o| o.status.success()) {
                return Some(PathBuf::from(cand));
            }
        }
        None
    }

    fn run_ffmpeg_to_bytes(ext: &str, args: &[&str]) -> Option<Vec<u8>> {
        static SEQ: AtomicU64 = AtomicU64::new(1);
        let bin = find_ffmpeg_cli()?;
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "port_mediadecoders_test_{}_{}.{ext}",
            std::process::id(),
            id
        ));
        let status = Command::new(bin)
            .arg("-y")
            .arg("-loglevel")
            .arg("error")
            .args(args)
            .arg(&path)
            .status()
            .ok()?;
        let bytes = if status.success() {
            std::fs::read(&path).ok()
        } else {
            None
        };
        let _ = std::fs::remove_file(&path);
        bytes
    }

    #[test]
    fn exports_allocator_and_struct_layouts() {
        assert_eq!(EXTRA_EXPORTS.len(), 25);
        for sym in [
            "InitializeAllocatorAndLogger",
            "CreateWebMDemuxer",
            "CreateMP4Demuxer",
            "CreateVPXDecoder",
            "CreateVorbisDecoder",
            "CreateH264Decoder",
            "CreateAACDecoder",
            "CreateMP3Decoder",
        ] {
            assert!(
                crate::lookup("MediaDecoders.WindowsDesktop.dll", sym).is_some(),
                "missing export {sym}"
            );
        }
        for e in EXTRA_EXPORTS {
            assert!(
                crate::lookup("MediaDecoders.WindowsDesktop.dll", e.name).is_some(),
                "missing extra export {}",
                e.name
            );
        }

        unsafe {
            InitializeAllocatorAndLogger(0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666);
        }
        let cfg = load_allocator_and_logger();
        assert_eq!(cfg.malloc_fn, 0x1111);
        assert_eq!(cfg.realloc_fn, 0x2222);
        assert_eq!(cfg.free_fn, 0x3333);
        assert_eq!(cfg.log_fn, 0x4444);
        assert_eq!(cfg.create_task_fn, 0x5555);
        assert_eq!(cfg.user_data, 0x6666);

        // Verify audio TrackInfo union byte offsets (+0x40 bits, +0x44 channels, +0x48 sample_rate).
        let ti = TrackInfo::audio(7, "vorbis", std::ptr::null(), 0, 16, 2, 48_000.0);
        unsafe {
            let base = (&ti as *const TrackInfo) as *const u8;
            assert_eq!(std::ptr::read_unaligned(base.add(0x00) as *const u32), 0);
            assert_eq!(std::ptr::read_unaligned(base.add(0x08) as *const u64), 7);
            assert_eq!(std::ptr::read_unaligned(base.add(0x40) as *const u32), 16);
            assert_eq!(std::ptr::read_unaligned(base.add(0x44) as *const u32), 2);
            assert_eq!(
                std::ptr::read_unaligned(base.add(0x48) as *const f32),
                48_000.0
            );
        }
    }

    #[test]
    fn h264_probe_and_object_lifecycle() {
        let has_ff = ffmpeg::ffmpeg().is_some();
        unsafe {
            // Probe with [0u64, 1u64] -> ProbeOnly at +0x08 == 1.
            let probe_info: [u64; 2] = [0, 1];
            let probe_res = CreateH264Decoder(probe_info.as_ptr() as *const c_void);
            if has_ff && ffmpeg::ffmpeg().unwrap().has_decoder(AV_CODEC_ID_H264) {
                assert_eq!(probe_res as usize, 1);
            } else {
                assert!(probe_res.is_null());
            }

            if !has_ff {
                return;
            }

            // Full decoder creation and 6-slot vtable verification across all codecs.
            let non_probe: [u64; 2] = [0, 0];
            let vp8_info = VpxInfo {
                version: 8,
                threads: 2,
            };
            let vp9_info = VpxInfo {
                version: 9,
                threads: 2,
            };
            let decoders = [
                CreateH264Decoder(non_probe.as_ptr() as *const c_void),
                CreateVPXDecoder(&vp8_info as *const VpxInfo as *const c_void),
                CreateVPXDecoder(&vp9_info as *const VpxInfo as *const c_void),
                CreateVorbisDecoder(std::ptr::null()),
                CreateMP3Decoder(std::ptr::null()),
                CreateAACDecoder(std::ptr::null()),
            ];
            for (idx, dec) in decoders.into_iter().enumerate() {
                assert!(!dec.is_null(), "decoder {idx} failed to create");
                let obj = dec as *const DecoderObject;
                assert_eq!((*obj).vtbl, &DECODER_VTBL as *const _);
                assert_eq!(vcall_decoder_is_async(dec), 0);
                if idx % 2 == 0 {
                    vcall_decoder_destroy(dec);
                } else {
                    // Non-deleting dtor (flags=0) followed by deleting dtor (flags=1).
                    assert_eq!(vcall_scalar_dtor(dec, 0), dec);
                    assert_eq!(vcall_scalar_dtor(dec, 1), dec);
                }
            }

            // Full demuxer creation and 7-slot vtable verification.
            let webm = CreateWebMDemuxer(std::ptr::null());
            let mp4 = CreateMP4Demuxer(std::ptr::null());
            assert!(!webm.is_null());
            assert!(!mp4.is_null());
            assert_eq!((*(webm as *const DemuxerObject)).vtbl, &DEMUXER_VTBL as *const _);
            assert_eq!((*(mp4 as *const DemuxerObject)).vtbl, &DEMUXER_VTBL as *const _);
            vcall_demuxer_reset(webm);
            vcall_demuxer_destroy(webm);
            assert_eq!(vcall_scalar_dtor(mp4, 1), mp4);
        }
    }

    #[test]
    fn end_to_end_webm_vp8_vorbis_and_vp9() {
        if ffmpeg::ffmpeg().is_none() || find_ffmpeg_cli().is_none() {
            return;
        }

        // 1) WebM containing VP8 (red 64x64) + Vorbis (440 Hz sine), fed in 1024-byte chunks!
        let webm_bytes = run_ffmpeg_to_bytes(
            "webm",
            &[
                "-f", "lavfi", "-i", "color=c=red:s=64x64:d=0.25:r=10",
                "-f", "lavfi", "-i", "sine=frequency=440:sample_rate=48000:duration=0.25",
                "-c:v", "libvpx", "-pix_fmt", "yuv420p",
                "-c:a", "libvorbis",
            ],
        )
        .or_else(|| {
            run_ffmpeg_to_bytes(
                "webm",
                &[
                    "-f", "lavfi", "-i", "color=c=red:s=64x64:d=0.25:r=10",
                    "-f", "lavfi", "-i", "sine=frequency=440:sample_rate=48000:duration=0.25",
                    "-c:v", "libvpx", "-pix_fmt", "yuv420p",
                    "-ac", "2", "-strict", "-2", "-c:a", "vorbis",
                ],
            )
        });
        if let Some(webm_bytes) = webm_bytes {
            unsafe {
                let demuxer = CreateWebMDemuxer(std::ptr::null());
                assert!(!demuxer.is_null());

                let mut dcb = MockDemuxCallbacks {
                    vtbl: &MOCK_DEMUX_VTBL,
                    durations: Vec::new(),
                    tracks: Vec::new(),
                    packets: Vec::new(),
                    errors: 0,
                };
                let mut scb = MockSeekCallbacks {
                    vtbl: &MOCK_SEEK_VTBL,
                    no_seek_count: 0,
                };
                vcall_demuxer_set_playback(demuxer, (&mut dcb as *mut MockDemuxCallbacks) as *mut c_void);
                vcall_demuxer_set_seek(demuxer, (&mut scb as *mut MockSeekCallbacks) as *mut c_void);

                // Stream in incremental chunks to exercise streaming WebM demuxing.
                for chunk in webm_bytes.chunks(1024) {
                    vcall_demuxer_demux(demuxer, chunk);
                }

                assert_eq!(dcb.errors, 0);
                assert_eq!(dcb.durations.len(), 1);
                assert_eq!(scb.no_seek_count, 1);

                let v_track = dcb
                    .tracks
                    .iter()
                    .find(|t| t.track_type == 1)
                    .expect("missing WebM video track");
                assert_eq!(v_track.codec_name, "vp8");
                assert_eq!(v_track.width, 64);
                assert_eq!(v_track.height, 64);

                let a_track = dcb
                    .tracks
                    .iter()
                    .find(|t| t.track_type == 0)
                    .expect("missing WebM audio track");
                assert_eq!(a_track.codec_name, "vorbis");
                assert!(!a_track.codec_private.is_empty());
                assert!(a_track.channels >= 1);
                assert!((a_track.sample_rate - 48_000.0).abs() < 1.0);
                let _ = a_track.bits_per_sample;

                // Decode VP8 packets.
                let vp8_info = VpxInfo { version: 8, threads: 1 };
                let vp8_dec = CreateVPXDecoder(&vp8_info as *const VpxInfo as *const c_void);
                assert!(!vp8_dec.is_null());
                let mut vcb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(vp8_dec, (&mut vcb as *mut MockDecoderCallbacks) as *mut c_void);
                vcall_decoder_init(vp8_dec, &v_track.codec_private);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == v_track.track_id) {
                    let _ = (pkt.alpha.len(), pkt.is_keyframe);
                    assert_eq!(vcall_decoder_decode(vp8_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert!(vcb.errors.is_empty(), "VP8 decode errors: {:?}", vcb.errors);
                assert!(!vcb.video_frames.is_empty());
                let vf = &vcb.video_frames[0];
                assert_eq!((vf.width, vf.height), (64, 64));
                assert!(!vf.has_alpha);
                let _ = vf.timestamp_ms;
                // Red in YUV has high V (~240) and lower U (~90).
                assert!(
                    vf.avg_v > 180.0 && vf.avg_u < 120.0,
                    "expected red YUV chroma, got Y={} U={} V={}",
                    vf.avg_y,
                    vf.avg_u,
                    vf.avg_v
                );
                vcall_decoder_destroy(vp8_dec);

                // Decode Vorbis packets.
                let vorbis_dec = CreateVorbisDecoder(std::ptr::null());
                assert!(!vorbis_dec.is_null());
                let mut acb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(vorbis_dec, (&mut acb as *mut MockDecoderCallbacks) as *mut c_void);
                vcall_decoder_init(vorbis_dec, &a_track.codec_private);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == a_track.track_id) {
                    assert_eq!(vcall_decoder_decode(vorbis_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert!(acb.errors.is_empty(), "Vorbis decode errors: {:?}", acb.errors);
                assert!(!acb.audio_frames.is_empty());
                assert!(acb.audio_frames.iter().all(|f| f.all_finite && f.channels >= 1 && f.samples_per_channel > 0));
                let _ = acb.audio_frames[0].timestamp_ms;
                let peak = acb.audio_frames.iter().map(|f| f.max_abs).fold(0.0f32, f32::max);
                assert!(peak > 0.05 && peak <= 1.5, "unexpected Vorbis peak amplitude {peak}");
                vcall_decoder_destroy(vorbis_dec);

                vcall_demuxer_destroy(demuxer);
            }
        }

        // 2) WebM containing VP9 (blue 64x64).
        if let Some(vp9_webm) = run_ffmpeg_to_bytes(
            "webm",
            &[
                "-f", "lavfi", "-i", "color=c=blue:s=64x64:d=0.2:r=10",
                "-c:v", "libvpx-vp9", "-pix_fmt", "yuv420p",
            ],
        ) {
            unsafe {
                let demuxer = CreateWebMDemuxer(std::ptr::null());
                let mut dcb = MockDemuxCallbacks {
                    vtbl: &MOCK_DEMUX_VTBL,
                    durations: Vec::new(),
                    tracks: Vec::new(),
                    packets: Vec::new(),
                    errors: 0,
                };
                vcall_demuxer_set_playback(demuxer, (&mut dcb as *mut MockDemuxCallbacks) as *mut c_void);
                vcall_demuxer_demux(demuxer, &vp9_webm);

                let v_track = dcb.tracks.iter().find(|t| t.track_type == 1).expect("missing VP9 track");
                assert_eq!(v_track.codec_name, "vp9");

                let vp9_info = VpxInfo { version: 9, threads: 1 };
                let vp9_dec = CreateVPXDecoder(&vp9_info as *const VpxInfo as *const c_void);
                let mut vcb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(vp9_dec, (&mut vcb as *mut MockDecoderCallbacks) as *mut c_void);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == v_track.track_id) {
                    assert_eq!(vcall_decoder_decode(vp9_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert!(vcb.errors.is_empty(), "VP9 decode errors: {:?}", vcb.errors);
                assert!(!vcb.video_frames.is_empty());
                let vf = &vcb.video_frames[0];
                // Blue in YUV has high U (~240) and lower V (~110).
                assert!(
                    vf.avg_u > 180.0 && vf.avg_v < 135.0,
                    "expected blue YUV chroma, got Y={} U={} V={}",
                    vf.avg_y,
                    vf.avg_u,
                    vf.avg_v
                );
                vcall_decoder_destroy(vp9_dec);
                vcall_demuxer_destroy(demuxer);
            }
        }
    }

    #[test]
    fn end_to_end_mp4_h264_aac_mp3_and_demux_moov() {
        if ffmpeg::ffmpeg().is_none() || find_ffmpeg_cli().is_none() {
            return;
        }

        // 1) Fast-start MP4 with H.264 + AAC.
        let mp4_h264_aac = run_ffmpeg_to_bytes(
            "mp4",
            &[
                "-f", "lavfi", "-i", "color=c=white:s=64x64:d=0.25:r=10",
                "-f", "lavfi", "-i", "sine=frequency=880:sample_rate=48000:duration=0.25",
                "-c:v", "libx264", "-profile:v", "baseline", "-pix_fmt", "yuv420p",
                "-c:a", "aac", "-movflags", "+faststart",
            ],
        )
        .or_else(|| {
            run_ffmpeg_to_bytes(
                "mp4",
                &[
                    "-f", "lavfi", "-i", "color=c=white:s=64x64:d=0.25:r=10",
                    "-f", "lavfi", "-i", "sine=frequency=880:sample_rate=48000:duration=0.25",
                    "-c:v", "h264_videotoolbox", "-pix_fmt", "yuv420p",
                    "-c:a", "aac", "-movflags", "+faststart",
                ],
            )
        });

        if let Some(mp4_bytes) = mp4_h264_aac {
            unsafe {
                let demuxer = CreateMP4Demuxer(std::ptr::null());
                assert!(!demuxer.is_null());
                let mut dcb = MockDemuxCallbacks {
                    vtbl: &MOCK_DEMUX_VTBL,
                    durations: Vec::new(),
                    tracks: Vec::new(),
                    packets: Vec::new(),
                    errors: 0,
                };
                vcall_demuxer_set_playback(demuxer, (&mut dcb as *mut MockDemuxCallbacks) as *mut c_void);
                vcall_demuxer_demux(demuxer, &mp4_bytes);

                assert_eq!(dcb.errors, 0);
                assert_eq!(dcb.durations.len(), 1);
                assert!(dcb.durations[0] > 0.0);

                let v_track = dcb.tracks.iter().find(|t| t.track_type == 1).expect("missing H264 track");
                assert_eq!(v_track.codec_name, "avc1");
                assert_eq!((v_track.width, v_track.height), (64, 64));
                assert!(!v_track.codec_private.is_empty());

                let a_track = dcb.tracks.iter().find(|t| t.track_type == 0).expect("missing AAC track");
                assert_eq!(a_track.codec_name, "aac");
                assert!(!a_track.codec_private.is_empty());

                // Decode H.264 packets + drain.
                let h264_dec = CreateH264Decoder(std::ptr::null());
                assert!(!h264_dec.is_null());
                let mut vcb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(h264_dec, (&mut vcb as *mut MockDecoderCallbacks) as *mut c_void);
                vcall_decoder_init(h264_dec, &v_track.codec_private);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == v_track.track_id) {
                    assert_eq!(vcall_decoder_decode(h264_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert_eq!(vcall_decoder_decode(h264_dec, &[], 0, 0), 1);
                assert!(vcb.errors.is_empty(), "H264 decode errors: {:?}", vcb.errors);
                assert!(!vcb.video_frames.is_empty());
                let vf = &vcb.video_frames[0];
                assert_eq!((vf.width, vf.height), (64, 64));
                // White has bright Y (~235) and neutral U/V (~128).
                assert!(
                    vf.avg_y > 200.0 && (vf.avg_u - 128.0).abs() < 20.0 && (vf.avg_v - 128.0).abs() < 20.0,
                    "expected white YUV, got Y={} U={} V={}",
                    vf.avg_y,
                    vf.avg_u,
                    vf.avg_v
                );
                vcall_decoder_destroy(h264_dec);

                // Decode AAC packets.
                let aac_dec = CreateAACDecoder(std::ptr::null());
                assert!(!aac_dec.is_null());
                let mut acb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(aac_dec, (&mut acb as *mut MockDecoderCallbacks) as *mut c_void);
                vcall_decoder_init(aac_dec, &a_track.codec_private);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == a_track.track_id) {
                    assert_eq!(vcall_decoder_decode(aac_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert!(acb.errors.is_empty(), "AAC decode errors: {:?}", acb.errors);
                assert!(!acb.audio_frames.is_empty());
                assert!(acb.audio_frames.iter().all(|f| f.all_finite && f.channels >= 1));
                let peak = acb.audio_frames.iter().map(|f| f.max_abs).fold(0.0f32, f32::max);
                assert!(peak > 0.05 && peak <= 1.5, "unexpected AAC peak amplitude {peak}");
                vcall_decoder_destroy(aac_dec);

                vcall_demuxer_destroy(demuxer);
            }
        }

        // 2) Trailing-moov MP4 with MP3 audio, split into `Demux(ftyp+mdat)` + `DemuxMoov(moov)`.
        if let Some(mp4_mp3) = run_ffmpeg_to_bytes(
            "mp4",
            &[
                "-f", "lavfi", "-i", "sine=frequency=660:sample_rate=44100:duration=0.25",
                "-c:a", "libmp3lame",
            ],
        ) {
            // Find the trailing `moov` box at top level so we can supply it via `DemuxMoov`.
            let mut pos = 0usize;
            let mut moov_pos = None;
            while pos + 8 <= mp4_mp3.len() {
                let sz = u32::from_be_bytes([
                    mp4_mp3[pos],
                    mp4_mp3[pos + 1],
                    mp4_mp3[pos + 2],
                    mp4_mp3[pos + 3],
                ]) as usize;
                if &mp4_mp3[pos + 4..pos + 8] == b"moov" {
                    moov_pos = Some((pos, sz));
                    break;
                }
                if sz < 8 {
                    break;
                }
                pos += sz;
            }

            unsafe {
                let demuxer = CreateMP4Demuxer(std::ptr::null());
                let mut dcb = MockDemuxCallbacks {
                    vtbl: &MOCK_DEMUX_VTBL,
                    durations: Vec::new(),
                    tracks: Vec::new(),
                    packets: Vec::new(),
                    errors: 0,
                };
                vcall_demuxer_set_playback(demuxer, (&mut dcb as *mut MockDemuxCallbacks) as *mut c_void);

                if let Some((mp, msz)) = moov_pos {
                    let prefix = &mp4_mp3[..mp];
                    let moov = &mp4_mp3[mp..mp + msz];
                    let half = prefix.len() / 2;
                    vcall_demuxer_demux(demuxer, &prefix[..half]);
                    vcall_demuxer_demux_moov(demuxer, moov);
                    vcall_demuxer_demux(demuxer, &prefix[half..]);
                } else {
                    vcall_demuxer_demux(demuxer, &mp4_mp3);
                }

                let a_track = dcb.tracks.iter().find(|t| t.track_type == 0).expect("missing MP3 track");
                assert_eq!(a_track.codec_name, "mp3");
                assert!(!dcb.packets.is_empty());

                let mp3_dec = CreateMP3Decoder(std::ptr::null());
                assert!(!mp3_dec.is_null());
                let mut acb = MockDecoderCallbacks {
                    vtbl: &MOCK_DECODER_VTBL,
                    video_frames: Vec::new(),
                    audio_frames: Vec::new(),
                    errors: Vec::new(),
                };
                vcall_decoder_set_callbacks(mp3_dec, (&mut acb as *mut MockDecoderCallbacks) as *mut c_void);
                let info = crate::misc::mediadecoders::AudioStreamInfo {
                    channels: a_track.channels as u16,
                    _pad: 0,
                    sample_rate: a_track.sample_rate as u32,
                };
                let info_bytes = std::slice::from_raw_parts(
                    (&info as *const crate::misc::mediadecoders::AudioStreamInfo) as *const u8,
                    std::mem::size_of::<crate::misc::mediadecoders::AudioStreamInfo>(),
                );
                vcall_decoder_init(mp3_dec, info_bytes);
                for pkt in dcb.packets.iter().filter(|p| p.track_id == a_track.track_id) {
                    assert_eq!(vcall_decoder_decode(mp3_dec, &pkt.data, pkt.timestamp_ms as u32, 0), 1);
                }
                assert!(acb.errors.is_empty(), "MP3 decode errors: {:?}", acb.errors);
                assert!(!acb.audio_frames.is_empty());
                assert!(acb.audio_frames.iter().all(|f| f.all_finite && f.channels >= 1));
                let peak = acb.audio_frames.iter().map(|f| f.max_abs).fold(0.0f32, f32::max);
                assert!(peak > 0.05 && peak <= 1.5, "unexpected MP3 peak amplitude {peak}");
                vcall_decoder_destroy(mp3_dec);

                vcall_demuxer_destroy(demuxer);
            }
        }
    }
}
