//! Clean-room native ARM64 implementation backing `MediaDecoders.WindowsDesktop.dll`.
//!
//! Organized into three submodules:
//! - [`ffmpeg`]: Dynamic `dlopen` bindings and compiler-verified FFmpeg 9.0.1 ABI accessors.
//! - [`decoder`]: `IDecoder` C++ objects (H.264, VP8/VP9 + alpha, Vorbis, MP3, AAC).
//! - [`demuxer`]: `IDemuxer` C++ objects (streaming/buffered WebM/Matroska and MP4).

pub mod decoder;
pub mod demuxer;
pub mod ffmpeg;

use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Host-supplied allocator / logger / task callbacks (`InitializeAllocatorAndLogger`)
// ---------------------------------------------------------------------------

static ALLOC_MALLOC: AtomicUsize = AtomicUsize::new(0);
static ALLOC_REALLOC: AtomicUsize = AtomicUsize::new(0);
static ALLOC_FREE: AtomicUsize = AtomicUsize::new(0);
static LOG_FN: AtomicUsize = AtomicUsize::new(0);
static CREATE_TASK_FN: AtomicUsize = AtomicUsize::new(0);
static USER_DATA: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocatorAndLoggerConfig {
    pub malloc_fn: usize,
    pub realloc_fn: usize,
    pub free_fn: usize,
    pub log_fn: usize,
    pub create_task_fn: usize,
    pub user_data: usize,
}

pub fn store_allocator_and_logger(cfg: AllocatorAndLoggerConfig) {
    ALLOC_MALLOC.store(cfg.malloc_fn, Ordering::Release);
    ALLOC_REALLOC.store(cfg.realloc_fn, Ordering::Release);
    ALLOC_FREE.store(cfg.free_fn, Ordering::Release);
    LOG_FN.store(cfg.log_fn, Ordering::Release);
    CREATE_TASK_FN.store(cfg.create_task_fn, Ordering::Release);
    USER_DATA.store(cfg.user_data, Ordering::Release);
}

#[allow(dead_code)]
pub fn load_allocator_and_logger() -> AllocatorAndLoggerConfig {
    AllocatorAndLoggerConfig {
        malloc_fn: ALLOC_MALLOC.load(Ordering::Acquire),
        realloc_fn: ALLOC_REALLOC.load(Ordering::Acquire),
        free_fn: ALLOC_FREE.load(Ordering::Acquire),
        log_fn: LOG_FN.load(Ordering::Acquire),
        create_task_fn: CREATE_TASK_FN.load(Ordering::Acquire),
        user_data: USER_DATA.load(Ordering::Acquire),
    }
}

// ---------------------------------------------------------------------------
// Win64 ABI structures shared between `MediaDecoders.WindowsDesktop.dll` and caller
// ---------------------------------------------------------------------------

/// Parameter block passed to `CreateVPXDecoder`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VpxInfo {
    pub version: i32, // +0x00: 8 = VP8, 9 = VP9
    pub threads: u32, // +0x04
}
const _: () = assert!(std::mem::size_of::<VpxInfo>() == 8);

/// Audio stream metadata optionally passed to `IDecoder::Initialize` for MP3/AAC.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamInfo {
    pub channels: u16,    // +0x00
    pub _pad: u16,        // +0x02
    pub sample_rate: u32, // +0x04
}
const _: () = assert!(std::mem::size_of::<AudioStreamInfo>() == 8);

/// Planar YUV(A) 4:2:0 video frame passed to `IDecoderCallbacks::OnVideoFrame`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VideoFrameData {
    pub width: u32,          // +0x00
    pub height: u32,         // +0x04
    pub display_width: u32,  // +0x08
    pub display_height: u32, // +0x0c
    pub y_plane: *const u8,  // +0x10
    pub u_plane: *const u8,  // +0x18
    pub v_plane: *const u8,  // +0x20
    pub a_plane: *const u8,  // +0x28 (null if no alpha)
    pub y_stride: u32,       // +0x30
    pub u_stride: u32,       // +0x34
    pub v_stride: u32,       // +0x38
    pub a_stride: u32,       // +0x3c (0 if no alpha)
    pub timestamp_ms: u32,   // +0x40
    pub _pad: u32,           // +0x44
}
const _: () = assert!(std::mem::size_of::<VideoFrameData>() == 0x48);

/// Planar float32 audio frame passed to `IDecoderCallbacks::OnAudioFrame`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioFrameData {
    pub samples_per_channel: i32,        // +0x00
    pub channels: u32,                   // +0x04
    pub channel_ptrs: *const *const f32, // +0x08
    pub timestamp_ms: u32,               // +0x10
    pub _pad: u32,                       // +0x14
}
const _: () = assert!(std::mem::size_of::<AudioFrameData>() == 0x18);

/// Track descriptor passed to `IDemuxerPlaybackCallbacks::OnTrackInfo`.
///
/// Layout of `+0x40..+0x50` union:
/// - Video (`track_type == 1`): `width: u64` (`+0x40`), `height: u64` (`+0x48`).
/// - Audio (`track_type == 0`): `bits_per_sample: u32` (`+0x40`), `channels: u32` (`+0x44`),
///   `sample_rate: f32` (`+0x48`), `_pad: u32` (`+0x4c`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrackInfo {
    pub track_type: u32,          // +0x00: 0 = audio, 1 = video
    pub _pad0: u32,               // +0x04
    pub track_id: u64,            // +0x08
    pub codec_name: [u8; 32],     // +0x10: NUL-terminated ASCII
    pub codec_private: *const u8, // +0x30
    pub codec_private_len: usize, // +0x38
    pub field0: u64,              // +0x40
    pub field1: u64,              // +0x48
}
const _: () = assert!(std::mem::size_of::<TrackInfo>() == 0x50);

impl TrackInfo {
    pub fn video(
        track_id: u64,
        codec: &str,
        codec_private: *const u8,
        codec_private_len: usize,
        width: u64,
        height: u64,
    ) -> Self {
        Self {
            track_type: 1,
            _pad0: 0,
            track_id,
            codec_name: encode_codec_name(codec),
            codec_private,
            codec_private_len,
            field0: width,
            field1: height,
        }
    }

    pub fn audio(
        track_id: u64,
        codec: &str,
        codec_private: *const u8,
        codec_private_len: usize,
        bits_per_sample: u32,
        channels: u32,
        sample_rate: f32,
    ) -> Self {
        Self {
            track_type: 0,
            _pad0: 0,
            track_id,
            codec_name: encode_codec_name(codec),
            codec_private,
            codec_private_len,
            field0: (bits_per_sample as u64) | ((channels as u64) << 32),
            field1: sample_rate.to_bits() as u64,
        }
    }
}

/// Demuxed elementary packet passed in a slice to `IDemuxerPlaybackCallbacks::OnFrames`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DemuxerFrame {
    pub data: *const u8,       // +0x00
    pub size: usize,           // +0x08
    pub alpha_data: *const u8, // +0x10 (null if none)
    pub alpha_size: usize,     // +0x18 (0 if none)
    pub track_id: u64,         // +0x20
    pub timestamp_ms: f32,     // +0x28
    pub is_keyframe: u8,       // +0x2c
    pub _pad: [u8; 3],         // +0x2d
}
const _: () = assert!(std::mem::size_of::<DemuxerFrame>() == 0x30);

fn encode_codec_name(name: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    let bytes = name.as_bytes();
    let n = bytes.len().min(31);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

/// Safely read slot `index` from a guest C++ object's vtable (`*(obj + 0)`).
pub(crate) unsafe fn read_guest_vfunc(obj: *mut std::ffi::c_void, slot: usize) -> usize {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const usize);
    if vtbl == 0 {
        return 0;
    }
    std::ptr::read_unaligned((vtbl + slot * std::mem::size_of::<usize>()) as *const usize)
}
