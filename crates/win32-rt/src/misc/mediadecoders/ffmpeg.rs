//! Dynamic FFmpeg 9.0.1 loader (`libavutil 61`, `libavcodec 63`, `libavformat 63`,
//! optional `libswscale 10`, optional `libswresample 7`) with exact compiler-verified
//! struct field offsets for `aarch64-apple-darwin`.

use std::ffi::{c_char, c_void, CString};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Expected FFmpeg major ABI versions (FFmpeg 9.0.1)
// ---------------------------------------------------------------------------

pub const EXPECTED_AVUTIL_MAJOR: u32 = 61;
pub const EXPECTED_AVCODEC_MAJOR: u32 = 63;
pub const EXPECTED_AVFORMAT_MAJOR: u32 = 63;
pub const EXPECTED_SWSCALE_MAJOR: u32 = 10;
pub const EXPECTED_SWRESAMPLE_MAJOR: u32 = 7;

// ---------------------------------------------------------------------------
// FFmpeg ABI constants
// ---------------------------------------------------------------------------

pub const AV_CODEC_ID_H264: i32 = 27;
pub const AV_CODEC_ID_VP8: i32 = 139;
pub const AV_CODEC_ID_VP9: i32 = 166;
pub const AV_CODEC_ID_MP3: i32 = 86017;
pub const AV_CODEC_ID_AAC: i32 = 86018;
pub const AV_CODEC_ID_VORBIS: i32 = 86021;
pub const AV_CODEC_ID_OPUS: i32 = 86076;

pub const AVMEDIA_TYPE_VIDEO: i32 = 0;
pub const AVMEDIA_TYPE_AUDIO: i32 = 1;

pub const AV_PIX_FMT_YUV420P: i32 = 0;
pub const AV_PIX_FMT_YUVJ420P: i32 = 12;
pub const AV_PIX_FMT_YUVA420P: i32 = 33;

pub const AV_SAMPLE_FMT_U8: i32 = 0;
pub const AV_SAMPLE_FMT_S16: i32 = 1;
pub const AV_SAMPLE_FMT_S32: i32 = 2;
pub const AV_SAMPLE_FMT_FLT: i32 = 3;
pub const AV_SAMPLE_FMT_DBL: i32 = 4;
pub const AV_SAMPLE_FMT_U8P: i32 = 5;
pub const AV_SAMPLE_FMT_S16P: i32 = 6;
pub const AV_SAMPLE_FMT_S32P: i32 = 7;
pub const AV_SAMPLE_FMT_FLTP: i32 = 8;
pub const AV_SAMPLE_FMT_DBLP: i32 = 9;

pub const AV_PKT_FLAG_KEY: i32 = 1;
pub const AV_PKT_FLAG_CORRUPT: i32 = 2;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: i32 = 15;

pub const SWS_BILINEAR: i32 = 2;

pub const AVERROR_EOF: i32 = -541_478_725;
pub const AVERROR_EAGAIN: i32 = -(libc::EAGAIN as i32);
pub const AV_NOPTS_VALUE: i64 = i64::MIN;
pub const AV_TIME_BASE: i64 = 1_000_000;
pub const AVSEEK_SIZE: i32 = 65536;
pub const AV_INPUT_BUFFER_PADDING_SIZE: usize = 64;

// ---------------------------------------------------------------------------
// Compiler-verified struct offsets (FFmpeg 9.0.1 on aarch64-apple-darwin)
// ---------------------------------------------------------------------------

// AVFrame (size 424)
pub const AVFRAME_DATA_OFF: usize = 0; // [*mut u8; 8]
pub const AVFRAME_LINESIZE_OFF: usize = 64; // [i32; 8]
pub const AVFRAME_EXTENDED_DATA_OFF: usize = 96; // *mut *mut u8
pub const AVFRAME_WIDTH_OFF: usize = 104; // i32
pub const AVFRAME_HEIGHT_OFF: usize = 108; // i32
pub const AVFRAME_NB_SAMPLES_OFF: usize = 112; // i32
pub const AVFRAME_FORMAT_OFF: usize = 116; // i32
pub const AVFRAME_PTS_OFF: usize = 136; // i64
pub const AVFRAME_SAMPLE_RATE_OFF: usize = 180; // i32
pub const AVFRAME_CH_LAYOUT_OFF: usize = 384; // AvChannelLayout (24 bytes)

// AVPacket (size 104)
pub const AVPACKET_PTS_OFF: usize = 8; // i64
pub const AVPACKET_DTS_OFF: usize = 16; // i64
pub const AVPACKET_DATA_OFF: usize = 24; // *mut u8
pub const AVPACKET_SIZE_OFF: usize = 32; // i32
pub const AVPACKET_STREAM_INDEX_OFF: usize = 36; // i32
pub const AVPACKET_FLAGS_OFF: usize = 40; // i32
pub const AVPACKET_SIDE_DATA_OFF: usize = 48; // *mut AvPacketSideData
pub const AVPACKET_SIDE_DATA_ELEMS_OFF: usize = 56; // i32

// AVPacketSideData (size 24)
pub const AVPACKET_SIDEDATA_SIZE: usize = 24;
pub const AVPACKET_SIDEDATA_DATA_OFF: usize = 0; // *mut u8
pub const AVPACKET_SIDEDATA_LEN_OFF: usize = 8; // usize
pub const AVPACKET_SIDEDATA_TYPE_OFF: usize = 16; // i32

// AVCodecContext (size 864)
pub const AVCODECCTX_EXTRADATA_OFF: usize = 72; // *mut u8
pub const AVCODECCTX_EXTRADATA_SIZE_OFF: usize = 80; // i32
pub const AVCODECCTX_SAMPLE_RATE_OFF: usize = 344; // i32
pub const AVCODECCTX_CH_LAYOUT_OFF: usize = 352; // AvChannelLayout (24 bytes)
pub const AVCODECCTX_THREAD_COUNT_OFF: usize = 656; // i32

// AVFormatContext (size 480)
pub const AVFORMATCTX_PB_OFF: usize = 32; // *mut c_void
pub const AVFORMATCTX_NB_STREAMS_OFF: usize = 44; // u32
pub const AVFORMATCTX_STREAMS_OFF: usize = 48; // *mut *mut c_void
pub const AVFORMATCTX_DURATION_OFF: usize = 104; // i64

// AVStream (size 216)
pub const AVSTREAM_CODECPAR_OFF: usize = 16; // *mut c_void
pub const AVSTREAM_TIME_BASE_OFF: usize = 32; // AvRational (8 bytes)
pub const AVSTREAM_DURATION_OFF: usize = 48; // i64

// AVCodecParameters (size 184)
pub const AVCODECPAR_CODEC_TYPE_OFF: usize = 0; // i32
pub const AVCODECPAR_CODEC_ID_OFF: usize = 4; // i32
pub const AVCODECPAR_EXTRADATA_OFF: usize = 16; // *mut u8
pub const AVCODECPAR_EXTRADATA_SIZE_OFF: usize = 24; // i32
pub const AVCODECPAR_BITS_PER_CODED_SAMPLE_OFF: usize = 56; // i32
pub const AVCODECPAR_BITS_PER_RAW_SAMPLE_OFF: usize = 60; // i32
pub const AVCODECPAR_WIDTH_OFF: usize = 72; // i32
pub const AVCODECPAR_HEIGHT_OFF: usize = 76; // i32
pub const AVCODECPAR_CH_LAYOUT_OFF: usize = 128; // AvChannelLayout (24 bytes)
pub const AVCODECPAR_SAMPLE_RATE_OFF: usize = 152; // i32

/// FFmpeg `AVChannelLayout` (24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AvChannelLayout {
    pub order: i32,             // +0x00
    pub nb_channels: i32,       // +0x04
    pub u_and_opaque: [u64; 2], // +0x08..+0x18
}
const _: () = assert!(std::mem::size_of::<AvChannelLayout>() == 24);

/// FFmpeg `AVRational` (8 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AvRational {
    pub num: i32,
    pub den: i32,
}
const _: () = assert!(std::mem::size_of::<AvRational>() == 8);

// ---------------------------------------------------------------------------
// Function pointer types (native C ABI)
// ---------------------------------------------------------------------------

pub type AvioReadPacketFn = unsafe extern "C" fn(*mut c_void, *mut u8, i32) -> i32;
pub type AvioWritePacketFn = unsafe extern "C" fn(*mut c_void, *const u8, i32) -> i32;
pub type AvioSeekFn = unsafe extern "C" fn(*mut c_void, i64, i32) -> i64;

pub struct SwScaleApi {
    _handle: usize,
    pub sws_getContext: unsafe extern "C" fn(
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        *mut c_void,
        *mut c_void,
        *const f64,
    ) -> *mut c_void,
    pub sws_scale: unsafe extern "C" fn(
        *mut c_void,
        *const *const u8,
        *const i32,
        i32,
        i32,
        *const *mut u8,
        *const i32,
    ) -> i32,
    pub sws_freeContext: unsafe extern "C" fn(*mut c_void),
}

pub struct SwResampleApi {
    _handle: usize,
    pub swr_alloc_set_opts2: unsafe extern "C" fn(
        *mut *mut c_void,
        *const AvChannelLayout,
        i32,
        i32,
        *const AvChannelLayout,
        i32,
        i32,
        i32,
        *mut c_void,
    ) -> i32,
    pub swr_init: unsafe extern "C" fn(*mut c_void) -> i32,
    pub swr_convert: unsafe extern "C" fn(
        *mut c_void,
        *mut *mut u8,
        i32,
        *const *const u8,
        i32,
    ) -> i32,
    pub swr_free: unsafe extern "C" fn(*mut *mut c_void),
}

pub struct FfmpegApi {
    _avutil_handle: usize,
    _avcodec_handle: usize,
    _avformat_handle: usize,

    // libavutil
    pub av_malloc: unsafe extern "C" fn(usize) -> *mut c_void,
    pub av_mallocz: unsafe extern "C" fn(usize) -> *mut c_void,
    pub av_free: unsafe extern "C" fn(*mut c_void),
    pub av_frame_alloc: unsafe extern "C" fn() -> *mut c_void,
    pub av_frame_free: unsafe extern "C" fn(*mut *mut c_void),
    pub av_frame_unref: unsafe extern "C" fn(*mut c_void),
    pub av_channel_layout_default: unsafe extern "C" fn(*mut AvChannelLayout, i32),
    pub av_channel_layout_uninit: unsafe extern "C" fn(*mut AvChannelLayout),
    pub av_strerror: unsafe extern "C" fn(i32, *mut c_char, usize) -> i32,

    // libavcodec
    pub avcodec_find_decoder: unsafe extern "C" fn(i32) -> *const c_void,
    pub avcodec_alloc_context3: unsafe extern "C" fn(*const c_void) -> *mut c_void,
    pub avcodec_free_context: unsafe extern "C" fn(*mut *mut c_void),
    pub avcodec_open2: unsafe extern "C" fn(*mut c_void, *const c_void, *mut *mut c_void) -> i32,
    pub avcodec_send_packet: unsafe extern "C" fn(*mut c_void, *const c_void) -> i32,
    pub avcodec_receive_frame: unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32,
    pub avcodec_flush_buffers: unsafe extern "C" fn(*mut c_void),
    pub av_packet_alloc: unsafe extern "C" fn() -> *mut c_void,
    pub av_packet_free: unsafe extern "C" fn(*mut *mut c_void),
    pub av_packet_unref: unsafe extern "C" fn(*mut c_void),
    pub av_new_packet: unsafe extern "C" fn(*mut c_void, i32) -> i32,

    // libavformat
    pub avformat_alloc_context: unsafe extern "C" fn() -> *mut c_void,
    pub avformat_free_context: unsafe extern "C" fn(*mut c_void),
    pub avformat_open_input: unsafe extern "C" fn(
        *mut *mut c_void,
        *const c_char,
        *const c_void,
        *mut *mut c_void,
    ) -> i32,
    pub avformat_find_stream_info: unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> i32,
    pub avformat_close_input: unsafe extern "C" fn(*mut *mut c_void),
    pub av_read_frame: unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32,
    pub av_find_input_format: unsafe extern "C" fn(*const c_char) -> *const c_void,
    pub avio_alloc_context: unsafe extern "C" fn(
        *mut u8,
        i32,
        i32,
        *mut c_void,
        Option<AvioReadPacketFn>,
        Option<AvioWritePacketFn>,
        Option<AvioSeekFn>,
    ) -> *mut c_void,
    pub avio_context_free: unsafe extern "C" fn(*mut *mut c_void),

    // Optional converters
    pub swscale: Option<SwScaleApi>,
    pub swresample: Option<SwResampleApi>,
}

unsafe impl Send for FfmpegApi {}
unsafe impl Sync for FfmpegApi {}

impl FfmpegApi {
    pub fn err_string(&self, code: i32) -> String {
        let mut buf = [0u8; 128];
        unsafe {
            (self.av_strerror)(code, buf.as_mut_ptr() as *mut c_char, buf.len());
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..end]).into_owned()
    }

    pub fn has_decoder(&self, codec_id: i32) -> bool {
        unsafe { !(self.avcodec_find_decoder)(codec_id).is_null() }
    }
}

fn search_dirs() -> Vec<String> {
    let mut dirs = Vec::with_capacity(6);
    if let Ok(env_dir) = std::env::var("PORT_FFMPEG_LIB_DIR") {
        let trimmed = env_dir.trim();
        if !trimmed.is_empty() {
            dirs.push(trimmed.to_string());
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            dirs.push(dir.join("../Frameworks").to_string_lossy().into_owned());
        }
    }
    dirs.push("/opt/homebrew/opt/ffmpeg/lib".to_string());
    dirs.push("/opt/homebrew/lib".to_string());
    dirs.push("/usr/local/opt/ffmpeg/lib".to_string());
    dirs.push("/usr/local/lib".to_string());
    dirs
}

unsafe fn dlopen_first(names: &[&str]) -> Option<*mut c_void> {
    let dirs = search_dirs();
    for dir in &dirs {
        for name in names {
            let full = format!("{dir}/{name}");
            if let Ok(cpath) = CString::new(full) {
                let h = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if !h.is_null() {
                    return Some(h);
                }
            }
        }
    }
    for name in names {
        if let Ok(cpath) = CString::new(*name) {
            let h = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
            if !h.is_null() {
                return Some(h);
            }
        }
    }
    None
}

unsafe fn load_sym<T: Copy>(handle: *mut c_void, name: &str) -> Option<T> {
    let cname = CString::new(name).ok()?;
    let ptr = libc::dlsym(handle, cname.as_ptr());
    if ptr.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy(&ptr))
    }
}

unsafe fn load_swscale() -> Option<SwScaleApi> {
    let h = dlopen_first(&["libswscale.10.dylib", "libswscale.dylib"])?;
    let swscale_version: unsafe extern "C" fn() -> u32 = load_sym(h, "swscale_version")?;
    let major = swscale_version() >> 16;
    if major != EXPECTED_SWSCALE_MAJOR {
        log::warn!(
            "mediadecoders: libswscale major version mismatch (got {major}, expected {EXPECTED_SWSCALE_MAJOR})"
        );
        return None;
    }
    Some(SwScaleApi {
        _handle: h as usize,
        sws_getContext: load_sym(h, "sws_getContext")?,
        sws_scale: load_sym(h, "sws_scale")?,
        sws_freeContext: load_sym(h, "sws_freeContext")?,
    })
}

unsafe fn load_swresample() -> Option<SwResampleApi> {
    let h = dlopen_first(&["libswresample.7.dylib", "libswresample.dylib"])?;
    let swresample_version: unsafe extern "C" fn() -> u32 = load_sym(h, "swresample_version")?;
    let major = swresample_version() >> 16;
    if major != EXPECTED_SWRESAMPLE_MAJOR {
        log::warn!(
            "mediadecoders: libswresample major version mismatch (got {major}, expected {EXPECTED_SWRESAMPLE_MAJOR})"
        );
        return None;
    }
    Some(SwResampleApi {
        _handle: h as usize,
        swr_alloc_set_opts2: load_sym(h, "swr_alloc_set_opts2")?,
        swr_init: load_sym(h, "swr_init")?,
        swr_convert: load_sym(h, "swr_convert")?,
        swr_free: load_sym(h, "swr_free")?,
    })
}

unsafe fn load_ffmpeg_inner() -> Option<FfmpegApi> {
    let h_util = dlopen_first(&["libavutil.61.dylib", "libavutil.dylib"])?;
    let h_codec = dlopen_first(&["libavcodec.63.dylib", "libavcodec.dylib"])?;
    let h_fmt = dlopen_first(&["libavformat.63.dylib", "libavformat.dylib"])?;

    let avutil_version: unsafe extern "C" fn() -> u32 = load_sym(h_util, "avutil_version")?;
    let avcodec_version: unsafe extern "C" fn() -> u32 = load_sym(h_codec, "avcodec_version")?;
    let avformat_version: unsafe extern "C" fn() -> u32 = load_sym(h_fmt, "avformat_version")?;

    let util_maj = avutil_version() >> 16;
    let codec_maj = avcodec_version() >> 16;
    let fmt_maj = avformat_version() >> 16;

    if util_maj != EXPECTED_AVUTIL_MAJOR
        || codec_maj != EXPECTED_AVCODEC_MAJOR
        || fmt_maj != EXPECTED_AVFORMAT_MAJOR
    {
        log::warn!(
            "mediadecoders: FFmpeg ABI major version mismatch: avutil={util_maj} (want {EXPECTED_AVUTIL_MAJOR}), \
             avcodec={codec_maj} (want {EXPECTED_AVCODEC_MAJOR}), avformat={fmt_maj} (want {EXPECTED_AVFORMAT_MAJOR})"
        );
        return None;
    }

    let api = FfmpegApi {
        _avutil_handle: h_util as usize,
        _avcodec_handle: h_codec as usize,
        _avformat_handle: h_fmt as usize,

        av_malloc: load_sym(h_util, "av_malloc")?,
        av_mallocz: load_sym(h_util, "av_mallocz")?,
        av_free: load_sym(h_util, "av_free")?,
        av_frame_alloc: load_sym(h_util, "av_frame_alloc")?,
        av_frame_free: load_sym(h_util, "av_frame_free")?,
        av_frame_unref: load_sym(h_util, "av_frame_unref")?,
        av_channel_layout_default: load_sym(h_util, "av_channel_layout_default")?,
        av_channel_layout_uninit: load_sym(h_util, "av_channel_layout_uninit")?,
        av_strerror: load_sym(h_util, "av_strerror")?,

        avcodec_find_decoder: load_sym(h_codec, "avcodec_find_decoder")?,
        avcodec_alloc_context3: load_sym(h_codec, "avcodec_alloc_context3")?,
        avcodec_free_context: load_sym(h_codec, "avcodec_free_context")?,
        avcodec_open2: load_sym(h_codec, "avcodec_open2")?,
        avcodec_send_packet: load_sym(h_codec, "avcodec_send_packet")?,
        avcodec_receive_frame: load_sym(h_codec, "avcodec_receive_frame")?,
        avcodec_flush_buffers: load_sym(h_codec, "avcodec_flush_buffers")?,
        av_packet_alloc: load_sym(h_codec, "av_packet_alloc")?,
        av_packet_free: load_sym(h_codec, "av_packet_free")?,
        av_packet_unref: load_sym(h_codec, "av_packet_unref")?,
        av_new_packet: load_sym(h_codec, "av_new_packet")?,

        avformat_alloc_context: load_sym(h_fmt, "avformat_alloc_context")?,
        avformat_free_context: load_sym(h_fmt, "avformat_free_context")?,
        avformat_open_input: load_sym(h_fmt, "avformat_open_input")?,
        avformat_find_stream_info: load_sym(h_fmt, "avformat_find_stream_info")?,
        avformat_close_input: load_sym(h_fmt, "avformat_close_input")?,
        av_read_frame: load_sym(h_fmt, "av_read_frame")?,
        av_find_input_format: load_sym(h_fmt, "av_find_input_format")?,
        avio_alloc_context: load_sym(h_fmt, "avio_alloc_context")?,
        avio_context_free: load_sym(h_fmt, "avio_context_free")?,

        swscale: load_swscale(),
        swresample: load_swresample(),
    };

    log::info!(
        "mediadecoders: loaded native FFmpeg (avutil={}, avcodec={}, avformat={}, swscale={}, swresample={})",
        util_maj,
        codec_maj,
        fmt_maj,
        api.swscale.is_some(),
        api.swresample.is_some(),
    );
    Some(api)
}

/// Returns the loaded FFmpeg API table if a compatible native FFmpeg 9.0.1 runtime is present.
pub fn ffmpeg() -> Option<&'static FfmpegApi> {
    static API: OnceLock<Option<FfmpegApi>> = OnceLock::new();
    API.get_or_init(|| unsafe { load_ffmpeg_inner() }).as_ref()
}

// ---------------------------------------------------------------------------
// Typed unaligned field accessors using compiler-verified offsets
// ---------------------------------------------------------------------------

#[inline]
pub unsafe fn rd<T: Copy>(base: *const c_void, off: usize) -> T {
    std::ptr::read_unaligned((base as *const u8).add(off) as *const T)
}

#[inline]
pub unsafe fn wr<T: Copy>(base: *mut c_void, off: usize, val: T) {
    std::ptr::write_unaligned((base as *mut u8).add(off) as *mut T, val)
}

#[inline]
pub unsafe fn field_ptr_mut<T>(base: *mut c_void, off: usize) -> *mut T {
    (base as *mut u8).add(off) as *mut T
}

#[inline]
pub unsafe fn field_ptr<T>(base: *const c_void, off: usize) -> *const T {
    (base as *const u8).add(off) as *const T
}
