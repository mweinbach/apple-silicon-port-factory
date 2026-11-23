//! ABI structures, surface pixel format constants, background I/O flags,
//! default colorspace matrix, and embedded logo stream for `bink2w64.dll`.

use super::audio as bink2_audio;
use std::ffi::c_void;

// Surface pixel format constants for `BinkCopyToBuffer` / `BinkCopyToBufferRect`.
pub const BINKSURFACE8P: u32 = 0;
pub const BINKSURFACE24: u32 = 1;
pub const BINKSURFACE24R: u32 = 2;
pub const BINKSURFACE32: u32 = 3;
pub const BINKSURFACE32R: u32 = 4;
pub const BINKSURFACE32A: u32 = 5;
pub const BINKSURFACE32RA: u32 = 6;
pub const BINKSURFACE4444: u32 = 7;
pub const BINKSURFACE5551: u32 = 8;
pub const BINKSURFACE555: u32 = 9;
pub const BINKSURFACE565: u32 = 10;
pub const BINKSURFACE655: u32 = 11;
pub const BINKSURFACE664: u32 = 12;
pub const BINKSURFACEYUY2: u32 = 13;
pub const BINKSURFACEUYVY: u32 = 14;
pub const BINKSURFACEYV12: u32 = 15;
pub const BINKSURFACE555R: u32 = 16;
pub const BINKSURFACE565R: u32 = 17;
pub const BINKSURFACEMASK: u32 = 0x1f;

// Background I/O control flags for `BinkControlBackgroundIO`.
pub const BINKBGIOSUSPEND: u32 = 1;
pub const BINKBGIORESUME: u32 = 2;
pub const BINKBGIOWAIT: u32 = 0x8000_0000;
pub const BINKBGIOTRYWAIT: u32 = 0x4000_0000;

/// Minimal valid 1-frame 320x180 30fps `KB2i` in-memory stream returned by `BinkLogoAddress()`.
/// Allows callers to pass `BinkOpen(BinkLogoAddress(), BINKFROMMEMORY)` cleanly.
pub(crate) static BINK_LOGO_DATA: [u8; 64] = [
    b'K', b'B', b'2', b'i', // +0x00 magic
    0x38, 0x00, 0x00, 0x00, // +0x04 file_size_minus_8 = 56 (64 - 8)
    0x01, 0x00, 0x00, 0x00, // +0x08 frames = 1
    0x08, 0x00, 0x00, 0x00, // +0x0c largest_frame_size = 8
    0x01, 0x00, 0x00, 0x00, // +0x10 frames_dup = 1
    0x40, 0x01, 0x00, 0x00, // +0x14 width = 320
    0xb4, 0x00, 0x00, 0x00, // +0x18 height = 180
    0x1e, 0x00, 0x00, 0x00, // +0x1c fps_num = 30
    0x01, 0x00, 0x00, 0x00, // +0x20 fps_den = 1
    0x00, 0x00, 0x00, 0x00, // +0x24 video_flags = 0
    0x00, 0x00, 0x00, 0x00, // +0x28 num_tracks = 0
    0x00, 0x00, 0x00, 0x00, // +0x2c extended header (4 bytes for KB2i)
    0x39, 0x00, 0x00, 0x00, // +0x30 frame_offsets[0] = 0x38 | 1 (keyframe)
    0x40, 0x00, 0x00, 0x00, // +0x34 frame_offsets[1] = 0x40
    0x00, 0x00, 0x00, 0x00, // +0x38 frame 1 payload (8 zero bytes)
    0x00, 0x00, 0x00, 0x00, // +0x3c
];

/// BT.709 limited-range YCrCb -> RGB 4x4 shader constants (`hbink + 0x60..0xa0`).
pub const DEFAULT_BINK_COLORSPACE: [f32; 16] = [
    1.59579468, -0.81347656, 0.0, 0.0,
    0.0, -0.39144897, 2.01782227, 0.0,
    -0.87406909, 0.53178209, -1.08591092, 1.0,
    1.16412354, 1.16412354, 1.16412354, 0.0,
];

#[derive(Clone, Copy, Default)]
pub(crate) struct FrameTimingSample {
    pub(crate) video_us: u64,
    pub(crate) audio_us: u64,
    pub(crate) read_us: u64,
    pub(crate) blit_us: u64,
    pub(crate) packet_bytes: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BinkPlane {
    pub buffer: *mut c_void, // +0x00
    pub allocate: i32,       // +0x08
    pub buffer_pitch: u32,   // +0x0c
}
const _: () = assert!(std::mem::size_of::<BinkPlane>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BinkFramePlaneSet {
    pub y_plane: BinkPlane,  // +0x00 (fb + 0x18)
    pub cr_plane: BinkPlane, // +0x10 (fb + 0x28)
    pub cb_plane: BinkPlane, // +0x20 (fb + 0x38)
    pub a_plane: BinkPlane,  // +0x30 (fb + 0x48)
    pub h_plane: BinkPlane,  // +0x40 (fb + 0x58)
}
const _: () = assert!(std::mem::size_of::<BinkFramePlaneSet>() == 0x50);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinkFrameBuffers {
    pub total_frames: i32,              // +0x00
    pub ya_buffer_width: u32,           // +0x04
    pub ya_buffer_height: u32,          // +0x08
    pub crcb_buffer_width: u32,         // +0x0c
    pub crcb_buffer_height: u32,        // +0x10
    pub frame_num: u32,                 // +0x14
    pub frames: [BinkFramePlaneSet; 3], // +0x18..+0x108
}
const _: () = assert!(std::mem::size_of::<BinkFrameBuffers>() == 0x108);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinkOpenOptions {
    pub file_offset: u64,       // +0x00
    pub force_rate: i32,        // +0x08
    pub force_rate_div: i32,    // +0x0c
    pub io_buffer_size: i32,    // +0x10
    pub simulate: i32,          // +0x14
    pub reserved: u32,          // +0x18
    pub tot_tracks: u32,        // +0x1c
    pub track_nums: *const i32, // +0x20
    pub user_open: usize,       // +0x28
}
const _: () = assert!(std::mem::size_of::<BinkOpenOptions>() == 0x30);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BinkSummary {
    pub width: u32,                   // +0x00
    pub height: u32,                  // +0x04
    pub total_time: u32,              // +0x08
    pub file_frame_rate: u32,         // +0x0c
    pub file_frame_rate_div: u32,     // +0x10
    pub frame_rate: u32,              // +0x14
    pub frame_rate_div: u32,          // +0x18
    pub total_open_time: u32,         // +0x1c
    pub total_frames: u32,            // +0x20
    pub total_played_frames: u32,     // +0x24
    pub skipped_frames: u32,          // +0x28
    pub skipped_blits: u32,           // +0x2c
    pub sound_skips: u32,             // +0x30
    pub total_blit_time: u32,         // +0x34
    pub total_read_time: u32,         // +0x38
    pub total_video_decomp_time: u32, // +0x3c
    pub total_audio_decomp_time: u32, // +0x40
    pub total_idle_read_time: u32,    // +0x44
    pub total_back_read_time: u32,    // +0x48
    pub total_read_speed: u32,        // +0x4c
    pub slowest_frame_time: u32,      // +0x50
    pub slowest2_frame_time: u32,     // +0x54
    pub slowest_frame_num: u32,       // +0x58
    pub slowest2_frame_num: u32,      // +0x5c
    pub average_data_rate: u32,       // +0x60
    pub lowest_frame_size: u32,       // +0x64
    pub highest_mem_amount: u32,      // +0x68
    pub total_io_memory: u32,         // +0x6c
    pub highest_io_used: u32,         // +0x70
    pub highest_1sec_rate: u32,       // +0x74
    pub highest_1sec_frame: u32,      // +0x78
    pub _extra: [u32; 5],             // +0x7c..+0x90
}
const _: () = assert!(std::mem::size_of::<BinkSummary>() == 0x90);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BinkRealtime {
    pub frame_num: u32,                // +0x00
    pub frame_rate: u32,               // +0x04
    pub frame_rate_div: u32,           // +0x08
    pub frames: u32,                   // +0x0c
    pub frames_time: u32,              // +0x10
    pub frames_video_decomp_time: u32, // +0x14
    pub frames_audio_decomp_time: u32, // +0x18
    pub frames_read_time: u32,         // +0x1c
    pub frames_idle_read_time: u32,    // +0x20
    pub frames_thread_read_time: u32,  // +0x24
    pub frames_blit_time: u32,         // +0x28
    pub read_buffer_size: u32,         // +0x2c
    pub read_buffer_used: u32,         // +0x30
    pub frames_data_rate: u32,         // +0x34
}
const _: () = assert!(std::mem::size_of::<BinkRealtime>() == 0x38);

/// Heap-allocated Bink audio track handle (`HBINKTRACK`) whose prefix matches
/// `BINKTRACK` in `bink2w64.dll` (`bink.h`).
#[repr(C)]
pub struct BinkTrack {
    pub frequency: u32,    // +0x00
    pub bits: u32,         // +0x04
    pub channels: u32,     // +0x08
    pub max_size: u32,     // +0x0c
    pub bink: *mut c_void, // +0x10
    pub sndcomp: usize,    // +0x18
    pub track_index: i32,  // +0x20
    pub(crate) _pad: u32,  // +0x24
    pub(crate) decoder: bink2_audio::BinkAudioDecoder,
    pub(crate) last_frame_epoch: u64,
    pub(crate) seek_epoch: u64,
    pub(crate) pcm_scratch: Vec<i16>,
}
