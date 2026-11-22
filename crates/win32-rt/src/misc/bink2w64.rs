//! `bink2w64.dll` — native Rust replacement for RAD Game Tools Bink 2 (`1.999a` / `2022.05`).
//!
//! Supports full native ARM64 decoding of Bink 2 (`KB2f`..`KB2n`, integer path) frames
//! AND hardware Apple `VideoToolbox` HEVC (`KB2v`) frames into caller-registered planar
//! YCrCb (+ optional Alpha / HDR) buffers via the standard `BinkOpenWithOptions` /
//! `BinkGetFrameBuffersInfo` / `BinkRegisterFrameBuffers` / `BinkDoFrame*` / `BinkNextFrame` /
//! `BinkWait` / `BinkGoto` / `BinkGetKeyFrame` API.

#[path = "bink2/mod.rs"]
mod bink2;
use bink2::audio as bink2_audio;
use bink2::vt as bink2_vt;

use crate::Export;
use bink2_codec::codec::{compute_slices, decode_frame, FramePlanes, SliceInfo};
use bink2_codec::hevc_container::{is_kb2v_magic, Kb2vMeta, KB2V_META_SIZE};
use bink2_codec::stream::{expand_frame_offsets, has_extended_header, is_bink2_magic, Bk2FileHeader};
use bink2_codec::tables::FULLRANGE_BINK_COLORSPACE;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::{c_void, CStr};
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

static MEM_ALLOC: AtomicUsize = AtomicUsize::new(0);
static MEM_FREE: AtomicUsize = AtomicUsize::new(0);

static OS_OPEN: AtomicUsize = AtomicUsize::new(0);
static OS_READ: AtomicUsize = AtomicUsize::new(0);
static OS_SEEK: AtomicUsize = AtomicUsize::new(0);
static OS_CLOSE: AtomicUsize = AtomicUsize::new(0);

static SOUND_OPEN: AtomicUsize = AtomicUsize::new(0);
static SOUND_PARAM1: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM2: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM3: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM4: AtomicU64 = AtomicU64::new(0);
static SOUND_DISABLED: AtomicBool = AtomicBool::new(false);
static SOUND_TRACK_SELECTION: Mutex<Option<Vec<u32>>> = Mutex::new(None);

// Pre-open global options set via `BinkSetFileOffset`, `BinkSetFrameRate`, `BinkSetIO`,
// `BinkSetIOSize`, and `BinkSetSimulate` (honored by `BinkOpen` / `BinkOpenWithOptions`).
static PREOPEN_FILE_OFFSET: AtomicU64 = AtomicU64::new(0);
static PREOPEN_FRAME_RATE_NUM: AtomicU32 = AtomicU32::new(0);
static PREOPEN_FRAME_RATE_DEN: AtomicU32 = AtomicU32::new(0);
static PREOPEN_USER_IO: AtomicUsize = AtomicUsize::new(0);
static PREOPEN_IO_SIZE: AtomicU32 = AtomicU32::new(0);
static PREOPEN_SIMULATE: AtomicU32 = AtomicU32::new(0);

// Static NUL-terminated error message buffer for `BinkGetError` / `BinkSetError`.
static LAST_ERROR_LOCK: Mutex<()> = Mutex::new(());
static LAST_ERROR: [AtomicU8; 512] = [const { AtomicU8::new(0) }; 512];

// Monotonic millisecond clock epoch for `RADTimerRead`.
static RAD_TIMER_EPOCH: OnceLock<Instant> = OnceLock::new();

// Track host-allocated `BinkUtilMalloc` blocks so `BinkUtilFree` always matches the allocator
// even if `BinkSetMemory` is changed between alloc and free.
static HOST_UTIL_ALLOCS: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();

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
static BINK_LOGO_DATA: [u8; 64] = [
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

const MAX_ASYNC_THREADS: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum AsyncThreadState {
    Stopped,
    Running,
    StopRequested,
}

struct AsyncThreadPool {
    states: [AsyncThreadState; MAX_ASYNC_THREADS],
    in_flight: [u32; MAX_ASYNC_THREADS],
}

impl AsyncThreadPool {
    const fn new() -> Self {
        Self {
            states: [AsyncThreadState::Stopped; MAX_ASYNC_THREADS],
            in_flight: [0; MAX_ASYNC_THREADS],
        }
    }
}

static ASYNC_POOL: OnceLock<(Mutex<AsyncThreadPool>, Condvar)> = OnceLock::new();

fn async_pool() -> &'static (Mutex<AsyncThreadPool>, Condvar) {
    ASYNC_POOL.get_or_init(|| (Mutex::new(AsyncThreadPool::new()), Condvar::new()))
}

struct AsyncDecodeGuard {
    slots: Vec<usize>,
}

impl AsyncDecodeGuard {
    fn acquire(thread_ids: &[u32]) -> Self {
        let mut slots = Vec::with_capacity(thread_ids.len());
        let (lock, _) = async_pool();
        let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
        for &tid in thread_ids {
            let idx = tid as usize;
            if idx < MAX_ASYNC_THREADS && !slots.contains(&idx) {
                pool.in_flight[idx] = pool.in_flight[idx].saturating_add(1);
                slots.push(idx);
            }
        }
        Self { slots }
    }
}

impl Drop for AsyncDecodeGuard {
    fn drop(&mut self) {
        if self.slots.is_empty() {
            return;
        }
        let (lock, cvar) = async_pool();
        let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
        for &idx in &self.slots {
            pool.in_flight[idx] = pool.in_flight[idx].saturating_sub(1);
        }
        cvar.notify_all();
    }
}

struct BinkMutexState {
    owner: Option<std::thread::ThreadId>,
    recursion: u32,
}

struct BinkUtilMutex {
    state: Mutex<BinkMutexState>,
    cvar: Condvar,
}

impl BinkUtilMutex {
    fn new() -> Self {
        Self {
            state: Mutex::new(BinkMutexState {
                owner: None,
                recursion: 0,
            }),
            cvar: Condvar::new(),
        }
    }

    fn lock(&self) {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.owner == Some(me) {
            st.recursion = st.recursion.saturating_add(1);
            return;
        }
        while st.owner.is_some() {
            st = self.cvar.wait(st).unwrap_or_else(|e| e.into_inner());
        }
        st.owner = Some(me);
        st.recursion = 1;
    }

    fn lock_timeout(&self, timeout_ms: i32) -> i32 {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.owner == Some(me) {
            st.recursion = st.recursion.saturating_add(1);
            return 1;
        }
        if st.owner.is_none() {
            st.owner = Some(me);
            st.recursion = 1;
            return 1;
        }
        if timeout_ms == 0 {
            return 0;
        }
        if timeout_ms < 0 {
            while st.owner.is_some() {
                st = self.cvar.wait(st).unwrap_or_else(|e| e.into_inner());
            }
            st.owner = Some(me);
            st.recursion = 1;
            return 1;
        }
        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        while st.owner.is_some() {
            let Some(rem) = deadline.checked_duration_since(Instant::now()) else {
                return 0;
            };
            let (next_st, res) = self
                .cvar
                .wait_timeout(st, rem)
                .unwrap_or_else(|e| e.into_inner());
            st = next_st;
            if st.owner.is_none() {
                break;
            }
            if res.timed_out() {
                return 0;
            }
        }
        st.owner = Some(me);
        st.recursion = 1;
        1
    }

    fn unlock(&self) {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.owner == Some(me) {
            if st.recursion > 1 {
                st.recursion -= 1;
            } else {
                st.owner = None;
                st.recursion = 0;
                self.cvar.notify_one();
            }
        }
    }
}

static UTIL_MUTEX_TABLE: OnceLock<Mutex<HashMap<usize, Arc<BinkUtilMutex>>>> = OnceLock::new();
static SOUND_GLOBAL_MUTEX: OnceLock<BinkUtilMutex> = OnceLock::new();

fn util_mutex_table() -> &'static Mutex<HashMap<usize, Arc<BinkUtilMutex>>> {
    UTIL_MUTEX_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn host_util_allocs() -> &'static Mutex<HashSet<usize>> {
    HOST_UTIL_ALLOCS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn set_bink_error(msg: &str) {
    let _guard = LAST_ERROR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let bytes = msg.as_bytes();
    let max_copy = bytes.len().min(LAST_ERROR.len() - 1);
    for (i, &b) in bytes[..max_copy].iter().enumerate() {
        LAST_ERROR[i].store(if b == 0 { b' ' } else { b }, Ordering::Release);
    }
    LAST_ERROR[max_copy].store(0, Ordering::Release);
}

#[derive(Clone, Copy, Default)]
struct FrameTimingSample {
    video_us: u64,
    audio_us: u64,
    read_us: u64,
    blit_us: u64,
    packet_bytes: u64,
}

struct SubtitleEntry {
    start_ms: u64,
    end_ms: u64,
    text_nul: Vec<u8>,
}

fn parse_srt_timestamp_ms(s: &str) -> Option<u64> {
    let s = s.trim();
    let mut parts = s.split(':');
    let h: u64 = parts.next()?.trim().parse().ok()?;
    let m: u64 = parts.next()?.trim().parse().ok()?;
    let sec_ms = parts.next()?.trim();
    let (sec_str, ms_str) = sec_ms
        .split_once(',')
        .or_else(|| sec_ms.split_once('.'))
        .unwrap_or((sec_ms, "0"));
    let sec: u64 = sec_str.trim().parse().ok()?;
    let ms_digits: String = ms_str.chars().take_while(|c| c.is_ascii_digit()).collect();
    let ms: u64 = if ms_digits.is_empty() {
        0
    } else {
        ms_digits.parse().ok()?
    };
    Some(
        h.saturating_mul(3_600_000)
            .saturating_add(m.saturating_mul(60_000))
            .saturating_add(sec.saturating_mul(1_000))
            .saturating_add(ms.min(999)),
    )
}

fn parse_srt_subtitles(content: &str) -> Vec<SubtitleEntry> {
    let mut entries = Vec::new();
    let clean = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut lines = clean.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let time_line = if trimmed.contains("-->") {
            trimmed
        } else if let Some(next_line) = lines.peek() {
            if next_line.contains("-->") {
                lines.next().unwrap().trim()
            } else {
                continue;
            }
        } else {
            continue;
        };
        let Some((start_s, end_s)) = time_line.split_once("-->") else {
            continue;
        };
        let (Some(start_ms), Some(end_ms)) = (
            parse_srt_timestamp_ms(start_s),
            parse_srt_timestamp_ms(end_s.split_whitespace().next().unwrap_or("")),
        ) else {
            continue;
        };
        let mut text = String::new();
        for text_line in lines.by_ref() {
            if text_line.trim().is_empty() {
                break;
            }
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str(text_line.trim_end_matches('\r'));
        }
        if !text.is_empty() {
            let mut text_nul = text.into_bytes();
            text_nul.push(0);
            entries.push(SubtitleEntry {
                start_ms,
                end_ms,
                text_nul,
            });
        }
    }
    entries
}

/// BT.709 limited-range YCrCb -> RGB 4x4 shader constants (`hbink + 0x60..0xa0`).
pub const DEFAULT_BINK_COLORSPACE: [f32; 16] = [
    1.59579468, -0.81347656, 0.0, 0.0,
    0.0, -0.39144897, 2.01782227, 0.0,
    -0.87406909, 0.53178209, -1.08591092, 1.0,
    1.16412354, 1.16412354, 1.16412354, 0.0,
];

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
    _pad: u32,             // +0x24
    decoder: bink2_audio::BinkAudioDecoder,
    last_frame_epoch: u64,
    seek_epoch: u64,
    pcm_scratch: Vec<i16>,
}

/// Heap-allocated Bink handle whose prefix `[0..0x800]` matches the binary layout
/// of `BINK` in `bink2w64.dll` (`0x6e8` bytes).
#[repr(C, align(64))]
pub struct BinkInstance {
    pub raw: [u8; 0x800],
    is_user_io: bool,
    from_memory_ptr: *const u8,
    from_memory_len: usize,
    file_offset: u64,
    user_file_slot: u64,
    host_file: Option<std::fs::File>,
    file_num_tracks: usize,
    slices: SliceInfo,
    pub played_frames: u32,
    last_decoded_frame: u32,
    pub default_frame_buffers: BinkFrameBuffers,
    owned_planes: Vec<Vec<u8>>,
    playback_start: Option<Instant>,
    playback_base_frame: u32,
    track_max_sizes: Vec<i32>,
    track_flags: Vec<u32>,
    track_ids: Vec<i32>,
    open_track_indices: Vec<i32>,
    audio_tracks: Vec<bink2_audio::AudioTrackHeader>,
    audio_player: bink2_audio::BinkAudioPlayer,
    frame_offsets: Vec<u64>,
    io_buffer: Vec<u8>,
    frame_packet_buf: Vec<u8>,
    cached_packet_frame: u32,
    cached_packet_len: usize,
    cached_packet_is_key: bool,
    cached_video_offset: usize,
    cached_track_spans: Vec<(usize, usize)>,
    frame_advance_epoch: u64,
    seek_epoch: u64,
    total_audio_decomp_us: u64,
    total_video_decomp_us: u64,
    total_read_us: u64,
    total_blit_us: u64,
    total_bytes_read: u64,
    video_on: bool,
    bg_io_suspended: i32,
    simulate_rate: u32,
    frame_timings: VecDeque<FrameTimingSample>,
    subtitles: Vec<SubtitleEntry>,
    hevc_decoder: Option<bink2_vt::VtHevcDecoder>,
}

impl BinkInstance {
    #[inline]
    unsafe fn rd_u32(&self, off: usize) -> u32 {
        std::ptr::read_unaligned(self.raw.as_ptr().add(off) as *const u32)
    }
    #[inline]
    unsafe fn wr_u32(&mut self, off: usize, v: u32) {
        std::ptr::write_unaligned(self.raw.as_mut_ptr().add(off) as *mut u32, v);
    }
    #[inline]
    unsafe fn rd_u64(&self, off: usize) -> u64 {
        std::ptr::read_unaligned(self.raw.as_ptr().add(off) as *const u64)
    }
    #[inline]
    unsafe fn wr_u64(&mut self, off: usize, v: u64) {
        std::ptr::write_unaligned(self.raw.as_mut_ptr().add(off) as *mut u64, v);
    }
    #[inline]
    unsafe fn bio_ptr(&mut self) -> *mut u8 {
        self.raw.as_mut_ptr().add(0x210)
    }
}

/// Sequential read helper during `BinkOpenWithOptions` across memory, user I/O, and OS callbacks.
unsafe fn read_open_stream(
    inst: &mut BinkInstance,
    is_from_memory: bool,
    mem_pos: &mut usize,
    dst: &mut [u8],
) -> bool {
    if dst.is_empty() {
        return true;
    }
    if is_from_memory {
        if *mem_pos + dst.len() > inst.from_memory_len {
            return false;
        }
        std::ptr::copy_nonoverlapping(inst.from_memory_ptr.add(*mem_pos), dst.as_mut_ptr(), dst.len());
        *mem_pos += dst.len();
        true
    } else if inst.is_user_io {
        let bio = inst.bio_ptr();
        let read_header_fn = inst.rd_u64(0x210) as usize;
        if read_header_fn == 0 {
            return false;
        }
        let n = crate::call_guest8(
            read_header_fn,
            bio as u64,
            u64::MAX,
            dst.as_mut_ptr() as u64,
            dst.len() as u64,
            0,
            0,
            0,
            0,
        );
        (n as usize) >= dst.len()
    } else if let Some(ref mut f) = inst.host_file {
        f.read_exact(dst).is_ok()
    } else {
        let os_read = OS_READ.load(Ordering::Acquire);
        if os_read == 0 || inst.user_file_slot == 0 {
            return false;
        }
        let slot_ptr = &mut inst.user_file_slot as *mut u64;
        let n = crate::call_guest8(
            os_read,
            slot_ptr as u64,
            dst.as_mut_ptr() as u64,
            dst.len() as u64,
            0,
            0,
            0,
            0,
            0,
        );
        (n as usize) >= dst.len()
    }
}

unsafe fn fill_plane_set_black(fb: *const BinkFrameBuffers, idx: usize) {
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

unsafe fn read_frame_packet(inst: &mut BinkInstance, frame_1based: u32) -> Option<(usize, bool)> {
    let idx = frame_1based.saturating_sub(1) as usize;
    if idx + 1 >= inst.frame_offsets.len() {
        return None;
    }
    let raw_start = inst.frame_offsets[idx];
    let raw_end = inst.frame_offsets[idx + 1];
    let is_key = (raw_start & 1) != 0;
    let start_off = raw_start & !3;
    let end_off = raw_end & !3;
    if end_off <= start_off {
        return None;
    }
    let size = (end_off - start_off) as usize;
    if size > 64 * 1024 * 1024 {
        return None;
    }

    if !inst.from_memory_ptr.is_null() {
        let s = start_off as usize;
        if s + size > inst.from_memory_len {
            return None;
        }
        inst.frame_packet_buf.resize(size + 8, 0);
        std::ptr::copy_nonoverlapping(
            inst.from_memory_ptr.add(s),
            inst.frame_packet_buf.as_mut_ptr(),
            size,
        );
        return Some((size, is_key));
    }

    inst.frame_packet_buf.resize(size + 8, 0);
    if inst.is_user_io {
        let read_frame_fn = inst.rd_u64(0x218) as usize;
        let read_header_fn = inst.rd_u64(0x210) as usize;
        let bio = inst.bio_ptr();
        let buf_ptr = inst.frame_packet_buf.as_mut_ptr() as u64;
        if read_frame_fn != 0 {
            crate::call_guest8(
                read_frame_fn,
                bio as u64,
                frame_1based as u64,
                start_off,
                buf_ptr,
                size as u64,
                0,
                0,
                0,
            );
            if inst.rd_u32(0x2d8) != 0 {
                inst.wr_u32(0x01c, 1);
                return None;
            }
            return Some((size, is_key));
        } else if read_header_fn != 0 {
            let n = crate::call_guest8(
                read_header_fn,
                bio as u64,
                inst.file_offset.wrapping_add(start_off),
                buf_ptr,
                size as u64,
                0,
                0,
                0,
                0,
            );
            if (n as usize) >= size {
                return Some((size, is_key));
            }
            return None;
        }
        return None;
    }

    if let Some(ref mut f) = inst.host_file {
        if f.seek(SeekFrom::Start(inst.file_offset.wrapping_add(start_off)))
            .is_ok()
            && f.read_exact(&mut inst.frame_packet_buf[..size]).is_ok()
        {
            return Some((size, is_key));
        }
        return None;
    }

    if inst.user_file_slot != 0 {
        let os_seek = OS_SEEK.load(Ordering::Acquire);
        let os_read = OS_READ.load(Ordering::Acquire);
        if os_read != 0 {
            let slot_ptr = &mut inst.user_file_slot as *mut u64;
            if os_seek != 0 {
                crate::call_guest8(
                    os_seek,
                    slot_ptr as u64,
                    inst.file_offset.wrapping_add(start_off),
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                );
            }
            let n = crate::call_guest8(
                os_read,
                slot_ptr as u64,
                inst.frame_packet_buf.as_mut_ptr() as u64,
                size as u64,
                0,
                0,
                0,
                0,
                0,
            );
            if (n as usize) >= size {
                return Some((size, is_key));
            }
        }
    }

    None
}

/// Ensure frame `cur_frame` is loaded into `inst.frame_packet_buf` and its leading
/// `file_num_tracks` audio chunk spans (`[u32_le alen][alen bytes]`) are indexed.
unsafe fn ensure_frame_packet_loaded(inst: &mut BinkInstance, cur_frame: u32) -> bool {
    if inst.cached_packet_frame == cur_frame && inst.cached_packet_len > 0 {
        return true;
    }
    let Some((pkt_len, is_key)) = read_frame_packet(inst, cur_frame) else {
        inst.cached_packet_frame = 0;
        inst.cached_packet_len = 0;
        inst.cached_packet_is_key = false;
        inst.cached_video_offset = 0;
        inst.cached_track_spans.clear();
        return false;
    };

    inst.cached_track_spans.clear();
    let mut pos = 0usize;
    for _ in 0..inst.file_num_tracks {
        if pos + 4 > pkt_len {
            pos = pkt_len;
            break;
        }
        let alen = u32::from_le_bytes([
            inst.frame_packet_buf[pos],
            inst.frame_packet_buf[pos + 1],
            inst.frame_packet_buf[pos + 2],
            inst.frame_packet_buf[pos + 3],
        ]) as usize;
        let start = pos + 4;
        let end = start.saturating_add(alen).min(pkt_len);
        inst.cached_track_spans.push((start, end));
        pos = end;
    }
    inst.cached_packet_frame = cur_frame;
    inst.cached_packet_len = pkt_len;
    inst.cached_packet_is_key = is_key;
    inst.cached_video_offset = pos;
    true
}

pub unsafe extern "win64" fn BinkSetMemory(alloc_fn: usize, free_fn: usize) {
    MEM_ALLOC.store(alloc_fn, Ordering::Release);
    MEM_FREE.store(free_fn, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetOSFileCallbacks(
    open_fn: usize,
    read_fn: usize,
    seek_fn: usize,
    close_fn: usize,
) {
    OS_OPEN.store(open_fn, Ordering::Release);
    OS_READ.store(read_fn, Ordering::Release);
    OS_SEEK.store(seek_fn, Ordering::Release);
    OS_CLOSE.store(close_fn, Ordering::Release);
}

pub unsafe extern "win64" fn BinkOpenXAudio2(_param1: u64, _param2: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio27(_param1: u64, _param2: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio28(_param1: u64, _param2: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio29(_param1: u64, _param2: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenDirectSound(_param: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenWaveOut(_param: u64) -> *mut c_void {
    SOUND_DISABLED.store(false, Ordering::Release);
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkFindXAudio2WinDevice(
    param1: u64,
    param2: u64,
) -> *mut c_void {
    log::debug!(
        "bink2w64: BinkFindXAudio2WinDevice({param1:#x}, {param2:#x}) -> NULL (default device)"
    );
    // NULL directs XAudio2 / Bink to use the default system audio endpoint.
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn BinkSetSoundSystem(open_fn: usize, param: u64) -> i32 {
    BinkSetSoundSystem4(open_fn, param, 0, 0, 0)
}

pub unsafe extern "win64" fn BinkSetSoundSystem2(open_fn: usize, param1: u64, param2: u64) -> i32 {
    BinkSetSoundSystem4(open_fn, param1, param2, 0, 0)
}

pub unsafe extern "win64" fn BinkSetSoundSystem4(
    open_fn: usize,
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> i32 {
    SOUND_OPEN.store(open_fn, Ordering::Release);
    SOUND_PARAM1.store(param1, Ordering::Release);
    SOUND_PARAM2.store(param2, Ordering::Release);
    SOUND_PARAM3.store(param3, Ordering::Release);
    SOUND_PARAM4.store(param4, Ordering::Release);
    SOUND_DISABLED.store(open_fn == 0, Ordering::Release);
    1
}

pub unsafe extern "win64" fn BinkSetSoundTrack(total_tracks: u32, tracks: *const u32) {
    let mut ids = Vec::new();
    if total_tracks > 0 && !tracks.is_null() {
        let count = (total_tracks as usize).min(32);
        ids.reserve(count);
        for i in 0..count {
            ids.push(std::ptr::read_unaligned(tracks.add(i)));
        }
    }
    *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = Some(ids);
}

pub unsafe extern "win64" fn BinkSetFileOffset(offset: u64) {
    PREOPEN_FILE_OFFSET.store(offset, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetFrameRate(forcerate: u32, forceratediv: u32) {
    PREOPEN_FRAME_RATE_NUM.store(forcerate, Ordering::Release);
    PREOPEN_FRAME_RATE_DEN.store(forceratediv, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetIO(io_open_fn: usize) {
    PREOPEN_USER_IO.store(io_open_fn, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetIOSize(iosize: u32) {
    PREOPEN_IO_SIZE.store(iosize, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetSimulate(sim: u32) {
    PREOPEN_SIMULATE.store(sim, Ordering::Release);
}

pub unsafe extern "win64" fn BinkGetError() -> *const u8 {
    LAST_ERROR.as_ptr() as *const u8
}

pub unsafe extern "win64" fn BinkSetError(err: *const u8) {
    let _guard = LAST_ERROR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if err.is_null() {
        LAST_ERROR[0].store(0, Ordering::Release);
        return;
    }
    let max_copy = LAST_ERROR.len() - 1;
    let mut i = 0usize;
    while i < max_copy {
        let b = std::ptr::read_unaligned(err.add(i));
        if b == 0 {
            break;
        }
        LAST_ERROR[i].store(b, Ordering::Release);
        i += 1;
    }
    LAST_ERROR[i].store(0, Ordering::Release);
}

pub unsafe extern "win64" fn RADTimerRead() -> u32 {
    let epoch = RAD_TIMER_EPOCH.get_or_init(Instant::now);
    epoch.elapsed().as_millis() as u32
}

pub unsafe extern "win64" fn BinkLogoAddress() -> *mut c_void {
    BINK_LOGO_DATA.as_ptr() as *mut c_void
}

pub unsafe extern "win64" fn BinkGetPlatformInfo(
    bink_platform_enum: i32,
    output: *mut c_void,
) -> i32 {
    log::debug!(
        "bink2w64: BinkGetPlatformInfo(enum={bink_platform_enum}, output={output:?}) -> 0"
    );
    0
}

pub unsafe extern "win64" fn BinkFreeGlobals() {
    let _ = BinkWaitStopAsyncThreadsMulti(std::ptr::null(), 0);
    PREOPEN_FILE_OFFSET.store(0, Ordering::Release);
    PREOPEN_FRAME_RATE_NUM.store(0, Ordering::Release);
    PREOPEN_FRAME_RATE_DEN.store(0, Ordering::Release);
    PREOPEN_USER_IO.store(0, Ordering::Release);
    PREOPEN_IO_SIZE.store(0, Ordering::Release);
    PREOPEN_SIMULATE.store(0, Ordering::Release);
    *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = None;
    set_bink_error("");
}

pub unsafe extern "win64" fn BinkUtilCPUs() -> i32 {
    std::thread::available_parallelism()
        .map(|n| n.get().clamp(1, 256) as i32)
        .unwrap_or(4)
}

pub unsafe extern "win64" fn BinkUtilMalloc(bytes: u64) -> *mut c_void {
    let alloc_fn = MEM_ALLOC.load(Ordering::Acquire);
    if alloc_fn != 0 {
        return crate::call_guest8(alloc_fn, bytes, 0, 0, 0, 0, 0, 0, 0) as *mut c_void;
    }
    let size = (bytes as usize).max(1);
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let rc = libc::posix_memalign(&mut ptr, 32, size);
    if rc != 0 || ptr.is_null() {
        return std::ptr::null_mut();
    }
    host_util_allocs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(ptr as usize);
    ptr
}

pub unsafe extern "win64" fn BinkUtilFree(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let is_host = host_util_allocs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&(ptr as usize));
    if is_host {
        libc::free(ptr);
        return;
    }
    let free_fn = MEM_FREE.load(Ordering::Acquire);
    if free_fn != 0 {
        crate::call_guest8(free_fn, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    } else {
        libc::free(ptr);
    }
}

pub unsafe extern "win64" fn BinkUtilMutexCreate() -> *mut c_void {
    let m = Arc::new(BinkUtilMutex::new());
    let id = Arc::as_ptr(&m) as usize;
    util_mutex_table()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(id, m);
    id as *mut c_void
}

pub unsafe extern "win64" fn BinkUtilMutexDestroy(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    util_mutex_table()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&(mutex as usize));
}

pub unsafe extern "win64" fn BinkUtilMutexLock(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    let m = util_mutex_table()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&(mutex as usize))
        .cloned();
    if let Some(m) = m {
        m.lock();
    }
}

pub unsafe extern "win64" fn BinkUtilMutexLockTimeOut(mutex: *mut c_void, ms: i32) -> i32 {
    if mutex.is_null() {
        return 0;
    }
    let m = util_mutex_table()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&(mutex as usize))
        .cloned();
    match m {
        Some(m) => m.lock_timeout(ms),
        None => 0,
    }
}

pub unsafe extern "win64" fn BinkUtilMutexUnlock(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    let m = util_mutex_table()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&(mutex as usize))
        .cloned();
    if let Some(m) = m {
        m.unlock();
    }
}

pub unsafe extern "win64" fn BinkUtilSoundGlobalLock() {
    SOUND_GLOBAL_MUTEX.get_or_init(BinkUtilMutex::new).lock();
}

pub unsafe extern "win64" fn BinkUtilSoundGlobalUnlock() {
    SOUND_GLOBAL_MUTEX.get_or_init(BinkUtilMutex::new).unlock();
}

pub unsafe extern "win64" fn BinkStartAsyncThread(thread_num: i32, _param: *const c_void) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    pool.states[thread_num as usize] = AsyncThreadState::Running;
    1
}

pub unsafe extern "win64" fn BinkRequestStopAsyncThread(thread_num: i32) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    let slot = &mut pool.states[thread_num as usize];
    if *slot == AsyncThreadState::Running {
        *slot = AsyncThreadState::StopRequested;
    }
    1
}

pub unsafe extern "win64" fn BinkWaitStopAsyncThread(thread_num: i32) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let idx = thread_num as usize;
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    while pool.in_flight[idx] > 0 {
        pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
    }
    pool.states[idx] = AsyncThreadState::Stopped;
    1
}

pub unsafe extern "win64" fn BinkRequestStopAsyncThreadsMulti(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if threads.is_null() {
        if thread_count > 0 {
            return 0;
        }
        for slot in &mut pool.states {
            if *slot == AsyncThreadState::Running {
                *slot = AsyncThreadState::StopRequested;
            }
        }
        return 1;
    }
    if thread_count <= 0 {
        return 1;
    }
    let count = (thread_count as usize).min(MAX_ASYNC_THREADS);
    for i in 0..count {
        let tid = std::ptr::read_unaligned(threads.add(i)) as usize;
        if tid < MAX_ASYNC_THREADS && pool.states[tid] == AsyncThreadState::Running {
            pool.states[tid] = AsyncThreadState::StopRequested;
        }
    }
    1
}

pub unsafe extern "win64" fn BinkWaitStopAsyncThreadsMulti(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if threads.is_null() {
        if thread_count > 0 {
            return 0;
        }
        while pool.in_flight.iter().any(|&n| n > 0) {
            pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
        }
        for slot in &mut pool.states {
            *slot = AsyncThreadState::Stopped;
        }
        return 1;
    }
    if thread_count <= 0 {
        return 1;
    }
    let count = (thread_count as usize).min(MAX_ASYNC_THREADS);
    let mut targets = Vec::with_capacity(count);
    for i in 0..count {
        let tid = std::ptr::read_unaligned(threads.add(i)) as usize;
        if tid < MAX_ASYNC_THREADS && !targets.contains(&tid) {
            targets.push(tid);
        }
    }
    while targets.iter().any(|&tid| pool.in_flight[tid] > 0) {
        pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
    }
    for tid in targets {
        pool.states[tid] = AsyncThreadState::Stopped;
    }
    1
}

pub unsafe extern "win64" fn BinkOpen(name: *const u8, flags: u32) -> *mut c_void {
    BinkOpenWithOptions(name, std::ptr::null(), flags)
}

pub unsafe extern "win64" fn BinkOpenWithOptions(
    name: *const u8,
    boo: *const BinkOpenOptions,
    flags: u32,
) -> *mut c_void {
    if name.is_null() {
        set_bink_error("Invalid Bink file name.");
        return std::ptr::null_mut();
    }

    let boo_val = (!boo.is_null()).then(|| std::ptr::read_unaligned(boo));
    let file_offset = if (flags & 0x20) != 0 {
        if let Some(b) = boo_val {
            b.file_offset
        } else {
            PREOPEN_FILE_OFFSET.load(Ordering::Acquire)
        }
    } else {
        0
    };
    let simulate_rate = if (flags & 0x0040_0000) != 0 {
        if let Some(b) = boo_val {
            b.simulate.max(0) as u32
        } else {
            PREOPEN_SIMULATE.load(Ordering::Acquire)
        }
    } else {
        0
    };
    let requested_io_size: u64 = if (flags & 0x0100_0000) != 0 {
        if let Some(b) = boo_val {
            b.io_buffer_size.max(0) as u64
        } else {
            PREOPEN_IO_SIZE.load(Ordering::Acquire) as u64
        }
    } else {
        0
    };

    let mut inst = Box::new(BinkInstance {
        raw: [0u8; 0x800],
        is_user_io: false,
        from_memory_ptr: std::ptr::null(),
        from_memory_len: 0,
        file_offset,
        user_file_slot: 0,
        host_file: None,
        file_num_tracks: 0,
        slices: SliceInfo {
            num_slices: 1,
            slice_ends: [0; 8],
        },
        played_frames: 0,
        last_decoded_frame: 0,
        default_frame_buffers: BinkFrameBuffers {
            total_frames: 2,
            ya_buffer_width: 0,
            ya_buffer_height: 0,
            crcb_buffer_width: 0,
            crcb_buffer_height: 0,
            frame_num: 0,
            frames: [BinkFramePlaneSet::default(); 3],
        },
        owned_planes: Vec::new(),
        playback_start: None,
        playback_base_frame: 0,
        track_max_sizes: Vec::new(),
        track_flags: Vec::new(),
        track_ids: Vec::new(),
        open_track_indices: Vec::new(),
        audio_tracks: Vec::new(),
        audio_player: bink2_audio::BinkAudioPlayer::new(Vec::new(), false),
        frame_offsets: Vec::new(),
        io_buffer: Vec::new(),
        frame_packet_buf: Vec::new(),
        cached_packet_frame: 0,
        cached_packet_len: 0,
        cached_packet_is_key: false,
        cached_video_offset: 0,
        cached_track_spans: Vec::new(),
        frame_advance_epoch: 0,
        seek_epoch: 0,
        total_audio_decomp_us: 0,
        total_video_decomp_us: 0,
        total_read_us: 0,
        total_blit_us: 0,
        total_bytes_read: 0,
        video_on: true,
        bg_io_suspended: 0,
        simulate_rate,
        frame_timings: VecDeque::new(),
        subtitles: Vec::new(),
        hevc_decoder: None,
    });

    let hbink_ptr = inst.raw.as_mut_ptr();
    // Set bio.bink (`hbink + 0x248` = `bio + 0x38`) to `hbink`.
    inst.wr_u64(0x248, hbink_ptr as u64);
    inst.wr_u64(0x0a0, file_offset);

    let mut hdr = Bk2FileHeader::default();
    let mut header_ok = false;

    let user_open = if (flags & 0x0200_0000) != 0 {
        if let Some(b) = boo_val {
            b.user_open
        } else {
            PREOPEN_USER_IO.load(Ordering::Acquire)
        }
    } else {
        0
    };

    let is_from_memory = (flags & 0x0400_0000) != 0;
    if is_from_memory {
        std::ptr::copy_nonoverlapping(name, &mut hdr as *mut Bk2FileHeader as *mut u8, 0x2c);
        inst.from_memory_ptr = name;
        inst.from_memory_len = hdr.file_size_minus_8.saturating_add(8) as usize;
        header_ok = true;
    } else if user_open != 0 {
        let bio = inst.bio_ptr();
        let opened = crate::call_guest8(
            user_open,
            bio as u64,
            name as u64,
            flags as u64,
            0,
            0,
            0,
            0,
            0,
        ) as i32;
        if opened == 0 {
            return std::ptr::null_mut();
        }
        inst.is_user_io = true;
        let read_header_fn = inst.rd_u64(0x210) as usize;
        if read_header_fn != 0 {
            let n = crate::call_guest8(
                read_header_fn,
                bio as u64,
                0,
                (&mut hdr as *mut Bk2FileHeader) as u64,
                0x2c,
                0,
                0,
                0,
                0,
            );
            header_ok = n >= 0x2c;
        }
    } else {
        let os_open = OS_OPEN.load(Ordering::Acquire);
        let os_read = OS_READ.load(Ordering::Acquire);
        let os_seek = OS_SEEK.load(Ordering::Acquire);
        if os_open != 0 && os_read != 0 {
            let slot_ptr = &mut inst.user_file_slot as *mut u64;
            let opened =
                crate::call_guest8(os_open, slot_ptr as u64, name as u64, 0, 0, 0, 0, 0, 0) as i32;
            if opened == 0 {
                return std::ptr::null_mut();
            }
            if file_offset != 0 && os_seek != 0 {
                crate::call_guest8(os_seek, slot_ptr as u64, file_offset, 0, 0, 0, 0, 0, 0);
            }
            let n = crate::call_guest8(
                os_read,
                slot_ptr as u64,
                (&mut hdr as *mut Bk2FileHeader) as u64,
                0x2c,
                0,
                0,
                0,
                0,
                0,
            );
            header_ok = n >= 0x2c;
        } else if let Ok(path_str) = CStr::from_ptr(name as *const std::ffi::c_char).to_str() {
            let opened_file = crate::path::to_host(path_str)
                .and_then(|p| std::fs::File::open(p).ok())
                .or_else(|| std::fs::File::open(path_str).ok());
            if let Some(mut f) = opened_file {
                let seek_ok = file_offset == 0 || f.seek(SeekFrom::Start(file_offset)).is_ok();
                let mut raw_hdr = [0u8; 0x2c];
                if seek_ok && f.read_exact(&mut raw_hdr).is_ok() {
                    if let Some(parsed) = Bk2FileHeader::parse(&raw_hdr) {
                        hdr = parsed;
                        header_ok = true;
                        inst.host_file = Some(f);
                    }
                }
            }
        }
    }

    let valid_magic = header_ok
        && (&hdr.magic[..3] == b"KB2" || &hdr.magic[..3] == b"BIK")
        && hdr.frames > 0
        && hdr.width > 0
        && hdr.height > 0;

    if !valid_magic {
        set_bink_error("Error opening Bink file.");
        BinkClose(Box::into_raw(inst) as *mut c_void);
        return std::ptr::null_mut();
    }

    let fourcc = u32::from_le_bytes(hdr.magic);
    let width = (hdr.width & 0x7fff).max(16);
    let height = (hdr.height & 0x7fff).max(16);
    let frames = hdr.frames.max(1);
    let mut fps_num = hdr.fps_num.max(1);
    let mut fps_den = hdr.fps_den.max(1);
    if (flags & 0x1000) != 0 {
        if let Some(b) = boo_val {
            if b.force_rate > 0 {
                fps_num = b.force_rate as u32;
            }
            if b.force_rate_div > 0 {
                fps_den = b.force_rate_div as u32;
            }
        } else {
            let pre_num = PREOPEN_FRAME_RATE_NUM.load(Ordering::Acquire);
            let pre_den = PREOPEN_FRAME_RATE_DEN.load(Ordering::Acquire);
            if pre_num > 0 {
                fps_num = pre_num;
            }
            if pre_den > 0 {
                fps_den = pre_den;
            }
        }
    }
    let file_size = hdr.file_size_minus_8.saturating_add(8);
    let file_num_tracks = (hdr.num_tracks & 0x3fff).min(256) as usize;
    inst.file_num_tracks = file_num_tracks;

    // Preserve exact open_flags semantics (`bink_rt2.c` lines 459-471), including HDR bit 2!
    let mut base_open_flags = flags;
    if (base_open_flags & 0x100) != 0 {
        base_open_flags = (base_open_flags & !0x100) | 0x400;
    }
    if (hdr.video_flags & 0x0010_0000) == 0 {
        base_open_flags &= !0x0010_0000;
    }
    let open_flags = if (hdr.video_flags & 0x04) == 0 {
        (hdr.video_flags & 0x0006_0013) | (base_open_flags & !0x04)
    } else {
        (hdr.video_flags & 0x0006_0013) | base_open_flags | 0x04
    };

    let is_kb2v = is_kb2v_magic(fourcc);

    // Exact extended header size (`bink_rt2.c` lines 473-490)
    let extra_hdr_bytes: usize = if is_kb2v {
        0
    } else if has_extended_header(fourcc) {
        let base = ((hdr.video_flags >> 24) as usize) * 4 + 4;
        if (hdr.video_flags & 0x04) != 0 {
            base + 12
        } else if (hdr.video_flags & 0x10000) != 0 {
            base + 24
        } else {
            base
        }
    } else {
        0
    };

    let mut extra_hdr_buf = vec![0u8; extra_hdr_bytes];
    let tables_dwords = file_num_tracks * 3 + (frames as usize) + 1;
    let tables_bytes = tables_dwords * 4;
    let mut tables_buf = vec![0u32; tables_dwords];

    let mut mem_pos = 0x2cusize;
    if is_kb2v {
        let mut meta_raw = [0u8; KB2V_META_SIZE];
        if !read_open_stream(&mut inst, is_from_memory, &mut mem_pos, &mut meta_raw) {
            BinkClose(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        }
        let Some(meta) = Kb2vMeta::parse(&meta_raw) else {
            log::error!("bink2w64: invalid KB2v HEVC secondary header");
            set_bink_error("Invalid KB2v HEVC header.");
            BinkClose(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        };
        let ps_len = meta.param_sets_padded_len();
        let mut ps_buf = vec![0u8; ps_len];
        if !read_open_stream(&mut inst, is_from_memory, &mut mem_pos, &mut ps_buf) {
            set_bink_error("Error reading KB2v parameter sets.");
            BinkClose(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        }
        let vps_end = meta.vps_len as usize;
        let sps_end = vps_end + meta.sps_len as usize;
        let pps_end = sps_end + meta.pps_len as usize;
        let full_range = (hdr.video_flags & 0x10) != 0;
        match bink2_vt::VtHevcDecoder::new(
            &ps_buf[..vps_end],
            &ps_buf[vps_end..sps_end],
            &ps_buf[sps_end..pps_end],
            full_range,
        ) {
            Ok(dec) => inst.hevc_decoder = Some(dec),
            Err(e) => {
                log::error!("bink2w64: VideoToolbox HEVC init failed: {e}");
                set_bink_error("VideoToolbox HEVC init failed.");
                BinkClose(Box::into_raw(inst) as *mut c_void);
                return std::ptr::null_mut();
            }
        }
        let tbl_bytes_slice = std::slice::from_raw_parts_mut(
            tables_buf.as_mut_ptr() as *mut u8,
            tables_bytes,
        );
        if !read_open_stream(&mut inst, is_from_memory, &mut mem_pos, tbl_bytes_slice) {
            BinkClose(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        }
    } else {
        if extra_hdr_bytes > 0 {
            let _ = read_open_stream(&mut inst, is_from_memory, &mut mem_pos, &mut extra_hdr_buf);
        }
        let tbl_bytes_slice = std::slice::from_raw_parts_mut(
            tables_buf.as_mut_ptr() as *mut u8,
            tables_bytes,
        );
        let _ = read_open_stream(&mut inst, is_from_memory, &mut mem_pos, tbl_bytes_slice);
    }

    // Parse colorspace (`hbink + 0x60..0xa0`, `bink_rt2.c` lines 602-722)
    let mut colorspace = [0.0f32; 16];
    if (hdr.video_flags & 0x04) != 0 && extra_hdr_buf.len() >= 12 {
        // HDR: 6 i16 values (first 5 scaled by 1/32767.0, 6th as integer float)
        let scale = 1.0f32 / 32767.0f32;
        for i in 0..5 {
            let v = i16::from_le_bytes([extra_hdr_buf[i * 2], extra_hdr_buf[i * 2 + 1]]);
            colorspace[i] = (v as f32) * scale;
        }
        let v5 = i16::from_le_bytes([extra_hdr_buf[10], extra_hdr_buf[11]]);
        colorspace[5] = v5 as f32;
    } else if (hdr.video_flags & 0x10000) != 0 && extra_hdr_buf.len() >= 24 {
        // Custom 4x3 matrix (12 i16 values scaled by 1/4096.0, 4th column = 0.0)
        let scale = 1.0f32 / 4096.0f32;
        let mut s_idx = 0usize;
        for row in 0..4 {
            for col in 0..3 {
                let v = i16::from_le_bytes([
                    extra_hdr_buf[s_idx * 2],
                    extra_hdr_buf[s_idx * 2 + 1],
                ]);
                colorspace[row * 4 + col] = (v as f32) * scale;
                s_idx += 1;
            }
            colorspace[row * 4 + 3] = 0.0;
        }
    } else if (open_flags & 0x10) != 0 {
        colorspace = FULLRANGE_BINK_COLORSPACE;
    } else {
        colorspace = DEFAULT_BINK_COLORSPACE;
    }

    inst.audio_tracks = bink2_audio::parse_audio_track_headers(file_num_tracks, &tables_buf);
    inst.track_max_sizes = Vec::with_capacity(file_num_tracks.max(1));
    inst.track_flags = Vec::with_capacity(file_num_tracks.max(1));
    inst.track_ids = Vec::with_capacity(file_num_tracks.max(1));
    for info in &inst.audio_tracks {
        inst.track_max_sizes.push(info.max_size as i32);
        inst.track_flags.push(info.raw_type);
        inst.track_ids.push(info.track_id as i32);
    }

    let fps_rounded = ((fps_den >> 1).saturating_add(fps_num) / fps_den).max(1);
    let fo_base = file_num_tracks * 3;
    let raw_fo = &tables_buf[fo_base..fo_base + (frames as usize) + 1];
    let (expanded_offs, highest_rate, _highest_frame) =
        expand_frame_offsets(frames as usize, raw_fo, fps_rounded)
            .unwrap_or_else(|| (raw_fo.iter().map(|&x| x as u64).collect(), 0, 0));
    inst.frame_offsets = expanded_offs;

    // Match requested tracks (`BINKSNDTRACK` = 0x4000) from `boo.track_nums` or `BinkSetSoundTrack`.
    if (flags & 0x4000) != 0 {
        let mut req_ids: Vec<u32> = Vec::new();
        if let Some(b) = boo_val {
            if b.tot_tracks > 0 && !b.track_nums.is_null() {
                let req_count = (b.tot_tracks as usize).min(32);
                req_ids.reserve(req_count);
                for idx in 0..req_count {
                    req_ids.push(std::ptr::read_unaligned(b.track_nums.add(idx)) as u32);
                }
            }
        }
        if req_ids.is_empty() {
            if let Some(ref sel) = *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) {
                req_ids.extend_from_slice(sel);
            }
        }
        for req_id in req_ids {
            if let Some(pos) = inst.audio_tracks.iter().position(|t| t.track_id == req_id) {
                if !inst.open_track_indices.contains(&(pos as i32)) {
                    inst.open_track_indices.push(pos as i32);
                }
            }
        }
    } else if file_num_tracks > 0 {
        inst.open_track_indices.push(0);
    }
    let open_track_indices_len = inst.open_track_indices.len() as u32;

    // Construct real Bink Audio decoders for each selected playback track.
    let version_b = hdr.magic == *b"BIKb";
    let mut active_tracks = Vec::with_capacity(open_track_indices_len as usize);
    for &track_idx_i32 in &inst.open_track_indices {
        let track_idx = track_idx_i32 as usize;
        if let Some(&info) = inst.audio_tracks.get(track_idx) {
            let sr = if info.sample_rate >= 300 {
                info.sample_rate
            } else {
                44_100
            };
            let ch = info.channels();
            log::debug!(
                "bink2w64: audio track[{track_idx}] id={} rate={}Hz ch={} 16bit={} codec={} max_size={}",
                info.track_id,
                sr,
                ch,
                info.is_16bit(),
                if info.use_dct() { "DCT" } else { "RDFT" },
                info.max_size
            );
            active_tracks.push(bink2_audio::ActiveAudioTrack {
                file_track_index: track_idx,
                track_id: info.track_id,
                sample_rate: sr,
                channels: ch,
                volume: bink2_audio::BINK_UNITY_VOLUME,
                pan: bink2_audio::BINK_CENTER_PAN,
                decoder: bink2_audio::BinkAudioDecoder::new(sr, ch, info.use_dct(), version_b),
            });
        }
    }
    let sound_enabled = !SOUND_DISABLED.load(Ordering::Acquire);
    inst.audio_player = bink2_audio::BinkAudioPlayer::new(active_tracks, sound_enabled);

    if inst.track_max_sizes.is_empty() {
        inst.track_max_sizes.push(0);
    }
    if inst.track_flags.is_empty() {
        inst.track_flags.push(0);
    }
    if inst.track_ids.is_empty() {
        inst.track_ids.push(0);
    }
    if inst.open_track_indices.is_empty() {
        inst.open_track_indices.push(0);
    }

    let slices = compute_slices(fourcc, open_flags, width, height);
    inst.slices = slices;

    let highest_1sec_rate = highest_rate.max((hdr.largest_frame_size.max(65536) as u64) * 2);

    let (ya_w, ya_h, crcb_w, crcb_h) = if is_bink2_magic(fourcc) || is_kb2v_magic(fourcc) {
        let yw = (width + 31) & !31;
        let yh = (height + 15) & !15;
        (yw, yh, yw >> 1, yh >> 1)
    } else {
        let yw = (width + 7) & !7;
        let yh = (height + 7) & !7;
        let cw = ((width + 1) >> 1).wrapping_add(7) & !7;
        let ch = ((height + 1) >> 1).wrapping_add(7) & !7;
        (yw, yh, cw, ch)
    };
    let total_fb = if frames == 1 {
        1
    } else {
        (((open_flags & 8) | 0x10) >> 3) as i32
    };
    let ya_pitch = (ya_w + 15) & !15;
    let crcb_pitch = (crcb_w + 15) & !15;
    let has_alpha = (open_flags & 0x0010_0000) != 0;
    let has_hdr = (open_flags & 0x0000_0004) != 0;

    inst.default_frame_buffers.total_frames = total_fb;
    inst.default_frame_buffers.ya_buffer_width = ya_w;
    inst.default_frame_buffers.ya_buffer_height = ya_h;
    inst.default_frame_buffers.crcb_buffer_width = crcb_w;
    inst.default_frame_buffers.crcb_buffer_height = crcb_h;
    inst.default_frame_buffers.frame_num = 0;

    let auto_alloc_fb = (flags & 0x400) == 0;
    for i in 0..(total_fb as usize).min(3) {
        inst.default_frame_buffers.frames[i].y_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: ya_pitch,
        };
        inst.default_frame_buffers.frames[i].cr_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: crcb_pitch,
        };
        inst.default_frame_buffers.frames[i].cb_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: crcb_pitch,
        };
        if has_alpha {
            inst.default_frame_buffers.frames[i].a_plane = BinkPlane {
                buffer: std::ptr::null_mut(),
                allocate: 1,
                buffer_pitch: ya_pitch,
            };
        }
        if has_hdr {
            inst.default_frame_buffers.frames[i].h_plane = BinkPlane {
                buffer: std::ptr::null_mut(),
                allocate: 1,
                buffer_pitch: ya_pitch,
            };
        }
    }

    if auto_alloc_fb {
        let ya_bytes = (ya_pitch as usize) * (ya_h as usize);
        let crcb_bytes = (crcb_pitch as usize) * (crcb_h as usize);
        for i in 0..(total_fb as usize).min(3) {
            let mut y_buf = vec![0x10u8; ya_bytes];
            let mut cr_buf = vec![0x80u8; crcb_bytes];
            let mut cb_buf = vec![0x80u8; crcb_bytes];
            inst.default_frame_buffers.frames[i].y_plane.buffer = y_buf.as_mut_ptr() as *mut c_void;
            inst.default_frame_buffers.frames[i].cr_plane.buffer =
                cr_buf.as_mut_ptr() as *mut c_void;
            inst.default_frame_buffers.frames[i].cb_plane.buffer =
                cb_buf.as_mut_ptr() as *mut c_void;
            inst.owned_planes.push(y_buf);
            inst.owned_planes.push(cr_buf);
            inst.owned_planes.push(cb_buf);
            if has_alpha {
                let mut a_buf = vec![0xFFu8; ya_bytes];
                inst.default_frame_buffers.frames[i].a_plane.buffer =
                    a_buf.as_mut_ptr() as *mut c_void;
                inst.owned_planes.push(a_buf);
            }
            if has_hdr {
                let mut h_buf = vec![0x00u8; ya_bytes];
                inst.default_frame_buffers.frames[i].h_plane.buffer =
                    h_buf.as_mut_ptr() as *mut c_void;
                inst.owned_planes.push(h_buf);
            }
        }
    }

    let default_fb_ptr = (&raw mut inst.default_frame_buffers) as u64;

    inst.wr_u32(0x000, width);
    inst.wr_u32(0x004, height);
    inst.wr_u32(0x008, frames);
    inst.wr_u32(0x00c, 1); // FrameNum (1-based)
    inst.wr_u32(0x010, 1); // LastFrameNum
    inst.wr_u32(0x014, fps_num);
    inst.wr_u32(0x018, fps_den);
    inst.wr_u32(0x01c, 0); // ReadError
    inst.wr_u32(0x020, open_flags);
    inst.wr_u32(0x024, hdr.video_flags);
    inst.wr_u32(0x028, file_size);
    inst.wr_u32(0x02c, hdr.largest_frame_size);
    inst.wr_u32(0x030, 0);
    inst.wr_u32(0x034, 0);
    inst.wr_u32(0x038, file_num_tracks as u32);
    inst.wr_u32(0x040, 1);
    inst.wr_u32(0x044, 1);
    inst.wr_u64(0x050, default_fb_ptr);
    inst.wr_u32(0x058, file_num_tracks as u32);

    for (i, &c) in colorspace.iter().enumerate() {
        inst.wr_u32(0x060 + i * 4, c.to_bits());
    }

    let open_track_indices_ptr = inst.open_track_indices.as_ptr() as u64;
    let frame_offsets_ptr = inst.frame_offsets.as_ptr() as u64;
    let track_max_sizes_ptr = inst.track_max_sizes.as_ptr() as u64;
    let track_flags_ptr = inst.track_flags.as_ptr() as u64;
    let track_ids_ptr = inst.track_ids.as_ptr() as u64;

    inst.wr_u64(0x0a0, file_offset);
    inst.wr_u64(0x0a8, highest_1sec_rate);
    inst.wr_u64(0x0b0, if auto_alloc_fb { default_fb_ptr } else { 0 });
    inst.wr_u64(0x0c8, open_track_indices_ptr);
    inst.wr_u64(0x0d0, file_size as u64);
    let effective_io_size = if requested_io_size > 0 {
        requested_io_size
    } else {
        highest_1sec_rate
    };
    inst.wr_u64(0x0d8, effective_io_size);
    inst.wr_u64(0x0e0, 0x10000);
    inst.wr_u64(0x110, 0xf000_0000);
    inst.wr_u32(0x118, fourcc);
    inst.wr_u32(0x11c, 0); // Paused = 0
    inst.wr_u32(0x12c, open_track_indices_len);
    inst.wr_u32(0x134, slices.num_slices);
    for i in 0..8 {
        inst.wr_u32(0x13c + i * 4, slices.slice_ends[i]);
    }
    inst.wr_u64(0x1c8, frame_offsets_ptr);
    inst.wr_u64(0x1e0, track_max_sizes_ptr);
    inst.wr_u64(0x1e8, track_flags_ptr);
    inst.wr_u64(0x1f0, track_ids_ptr);
    inst.wr_u32(0x3b8, if (open_flags & 0x80) != 0 { 1 } else { 0 });

    if inst.is_user_io {
        let bio = inst.bio_ptr();
        let get_buf_size_fn = inst.rd_u64(0x220) as usize;
        let mut io_buf_size = effective_io_size.max((hdr.largest_frame_size as u64).saturating_add(4096));
        if get_buf_size_fn != 0 {
            let s = crate::call_guest8(get_buf_size_fn, bio as u64, io_buf_size, 0, 0, 0, 0, 0, 0);
            if s > 0 {
                io_buf_size = s;
            }
        }
        let min_sz = (hdr.largest_frame_size as usize).saturating_add(4096).max(4096);
        let alloc_sz = (io_buf_size as usize).clamp(min_sz, 32 * 1024 * 1024);
        inst.io_buffer = vec![0u8; alloc_sz];
        let io_buf_ptr = inst.io_buffer.as_mut_ptr() as u64;
        inst.wr_u64(0x0d8, alloc_sz as u64);
        let set_info_fn = inst.rd_u64(0x228) as usize;
        if set_info_fn != 0 {
            crate::call_guest8(
                set_info_fn,
                bio as u64,
                io_buf_ptr,
                alloc_sz as u64,
                (file_size as u64).saturating_add(8),
                0,
                0,
                0,
                0,
            );
        }
    }

    set_bink_error("");

    let fourcc_bytes = fourcc.to_le_bytes();
    log::info!(
        "bink2w64 (builtin): opened {} {}x{} frames={} fps={}/{} vflags={:#x} slices={} tracks={} sim={} io={}",
        std::str::from_utf8(&fourcc_bytes).unwrap_or("????"),
        width,
        height,
        frames,
        fps_num,
        fps_den,
        hdr.video_flags,
        slices.num_slices,
        file_num_tracks,
        inst.simulate_rate,
        if is_from_memory {
            "memory"
        } else if inst.is_user_io {
            "user_io"
        } else if inst.host_file.is_some() {
            "file"
        } else {
            "os"
        }
    );

    Box::into_raw(inst) as *mut c_void
}

pub unsafe extern "win64" fn BinkClose(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let mut inst = Box::from_raw(bink as *mut BinkInstance);
    if inst.is_user_io {
        let close_fn = inst.rd_u64(0x238) as usize;
        if close_fn != 0 {
            let bio = inst.bio_ptr();
            crate::call_guest8(close_fn, bio as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    } else if inst.user_file_slot != 0 {
        let os_close = OS_CLOSE.load(Ordering::Acquire);
        if os_close != 0 {
            let slot_ptr = &mut inst.user_file_slot as *mut u64;
            crate::call_guest8(os_close, slot_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
        }
        inst.user_file_slot = 0;
    }
}

pub unsafe extern "win64" fn BinkGetSummary(bink: *mut c_void, sum: *mut BinkSummary) {
    if bink.is_null() || sum.is_null() {
        return;
    }
    let inst = &*(bink as *const BinkInstance);
    let width = inst.rd_u32(0x000);
    let height = inst.rd_u32(0x004);
    let frames = inst.rd_u32(0x008).max(1);
    let frame_num = inst.rd_u32(0x00c);
    let fps_num = inst.rd_u32(0x014).max(1);
    let fps_den = inst.rd_u32(0x018).max(1);
    let total_time = ((frames as u64) * (fps_den as u64) * 1000 / (fps_num as u64)) as u32;
    let total_played_frames = inst.played_frames.max(frame_num);
    let avg_data_rate = if total_time > 0 {
        let total_bytes = inst.total_bytes_read.max(inst.rd_u64(0x0d0));
        ((total_bytes.saturating_mul(1000)) / (total_time as u64)).min(u32::MAX as u64) as u32
    } else {
        0
    };
    let io_mem = inst
        .io_buffer
        .len()
        .max(inst.rd_u64(0x0d8) as usize)
        .min(u32::MAX as usize) as u32;

    let summary = BinkSummary {
        width,
        height,
        total_time,
        file_frame_rate: fps_num,
        file_frame_rate_div: fps_den,
        frame_rate: fps_num,
        frame_rate_div: fps_den,
        total_open_time: 1,
        total_frames: frames,
        total_played_frames,
        total_blit_time: (inst.total_blit_us / 1000) as u32,
        total_read_time: (inst.total_read_us / 1000) as u32,
        total_video_decomp_time: (inst.total_video_decomp_us / 1000) as u32,
        total_audio_decomp_time: (inst.total_audio_decomp_us / 1000) as u32,
        average_data_rate: avg_data_rate,
        total_io_memory: io_mem,
        highest_1sec_rate: inst.rd_u64(0x0a8) as u32,
        ..BinkSummary::default()
    };
    std::ptr::write_unaligned(sum, summary);
}

pub unsafe extern "win64" fn BinkGetRealtime(
    bink: *mut c_void,
    run: *mut BinkRealtime,
    frame_window: u32,
) {
    if bink.is_null() || run.is_null() {
        return;
    }
    let inst = &*(bink as *const BinkInstance);
    let frame_num = inst.rd_u32(0x00c);
    let fps_num = inst.rd_u32(0x014).max(1);
    let fps_den = inst.rd_u32(0x018).max(1);
    let want = frame_window.max(1) as usize;
    let count = inst.frame_timings.len().min(want);

    let mut video_us = 0u64;
    let mut audio_us = 0u64;
    let mut read_us = 0u64;
    let mut blit_us = 0u64;
    let mut bytes = 0u64;
    if count > 0 {
        let start = inst.frame_timings.len() - count;
        for sample in inst.frame_timings.iter().skip(start) {
            video_us = video_us.saturating_add(sample.video_us);
            audio_us = audio_us.saturating_add(sample.audio_us);
            read_us = read_us.saturating_add(sample.read_us);
            blit_us = blit_us.saturating_add(sample.blit_us);
            bytes = bytes.saturating_add(sample.packet_bytes);
        }
    }

    let nominal_ms = (count as u64)
        .saturating_mul(fps_den as u64)
        .saturating_mul(1000)
        / (fps_num as u64);
    let work_ms = video_us
        .saturating_add(audio_us)
        .saturating_add(read_us)
        .saturating_add(blit_us)
        / 1000;
    let frames_time = nominal_ms.max(work_ms).min(u32::MAX as u64) as u32;
    let data_rate = if frames_time > 0 {
        (bytes.saturating_mul(1000) / (frames_time as u64)).min(u32::MAX as u64) as u32
    } else {
        0
    };
    let read_buf_size = inst
        .io_buffer
        .len()
        .max(inst.rd_u64(0x0d8) as usize)
        .min(u32::MAX as usize) as u32;
    let read_buf_used = inst.cached_packet_len.min(read_buf_size as usize) as u32;

    let rt = BinkRealtime {
        frame_num,
        frame_rate: fps_num,
        frame_rate_div: fps_den,
        frames: count as u32,
        frames_time,
        frames_video_decomp_time: (video_us / 1000).min(u32::MAX as u64) as u32,
        frames_audio_decomp_time: (audio_us / 1000).min(u32::MAX as u64) as u32,
        frames_read_time: (read_us / 1000).min(u32::MAX as u64) as u32,
        frames_idle_read_time: 0,
        frames_thread_read_time: 0,
        frames_blit_time: (blit_us / 1000).min(u32::MAX as u64) as u32,
        read_buffer_size: read_buf_size,
        read_buffer_used: read_buf_used,
        frames_data_rate: data_rate,
    };
    std::ptr::write_unaligned(run, rt);
}

pub unsafe extern "win64" fn BinkGetRects(bink: *mut c_void, _flags: u32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    if !inst.video_on {
        return 0;
    }
    1
}

pub unsafe extern "win64" fn BinkGetGPUDataBuffersInfo(
    _bink: *mut c_void,
    gpu: *mut c_void,
) -> i32 {
    if !gpu.is_null() {
        std::ptr::write_bytes(gpu as *mut u8, 0, 0xe0);
    }
    // Returning 0 selects CPU standard decompressor (`BinkGetFrameBuffersInfo` /
    // `BinkRegisterFrameBuffers`) so our native ARM64 Bink 2 decoder writes planar YCrCb(A/H).
    0
}

pub unsafe extern "win64" fn BinkRegisterGPUDataBuffers(bink: *mut c_void, gpu: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.wr_u64(0x1c0, gpu as u64);
}

pub unsafe extern "win64" fn BinkGetFrameBuffersInfo(
    bink: *mut c_void,
    fbset: *mut BinkFrameBuffers,
) {
    if bink.is_null() || fbset.is_null() {
        return;
    }
    let inst = &*(bink as *const BinkInstance);
    let width = inst.rd_u32(0x000);
    let height = inst.rd_u32(0x004);
    let frames = inst.rd_u32(0x008).max(1);
    let open_flags = inst.rd_u32(0x020);
    let fourcc = inst.rd_u32(0x118);

    let (ya_w, ya_h, crcb_w, crcb_h) = if is_bink2_magic(fourcc) || is_kb2v_magic(fourcc) {
        let yw = (width + 31) & !31;
        let yh = (height + 15) & !15;
        (yw, yh, yw >> 1, yh >> 1)
    } else {
        let yw = (width + 7) & !7;
        let yh = (height + 7) & !7;
        let cw = ((width + 1) >> 1).wrapping_add(7) & !7;
        let ch = ((height + 1) >> 1).wrapping_add(7) & !7;
        (yw, yh, cw, ch)
    };
    let total_frames = if frames == 1 {
        1
    } else {
        (((open_flags & 8) | 0x10) >> 3) as i32
    };

    let mut fb = BinkFrameBuffers {
        total_frames,
        ya_buffer_width: ya_w,
        ya_buffer_height: ya_h,
        crcb_buffer_width: crcb_w,
        crcb_buffer_height: crcb_h,
        frame_num: 0,
        frames: [BinkFramePlaneSet::default(); 3],
    };

    let existing_fb = inst.rd_u64(0x0b0) as *const BinkFrameBuffers;
    if !existing_fb.is_null() {
        fb.frames = std::ptr::read_unaligned(existing_fb).frames;
        std::ptr::write_unaligned(fbset, fb);
        return;
    }

    let ya_pitch = (ya_w + 15) & !15;
    let crcb_pitch = (crcb_w + 15) & !15;
    let has_alpha = (open_flags & 0x0010_0000) != 0;
    let has_hdr = (open_flags & 0x0000_0004) != 0;

    for i in 0..(total_frames as usize).min(3) {
        fb.frames[i].y_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: ya_pitch,
        };
        fb.frames[i].cr_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: crcb_pitch,
        };
        fb.frames[i].cb_plane = BinkPlane {
            buffer: std::ptr::null_mut(),
            allocate: 1,
            buffer_pitch: crcb_pitch,
        };
        if has_alpha {
            fb.frames[i].a_plane = BinkPlane {
                buffer: std::ptr::null_mut(),
                allocate: 1,
                buffer_pitch: ya_pitch,
            };
        }
        if has_hdr {
            fb.frames[i].h_plane = BinkPlane {
                buffer: std::ptr::null_mut(),
                allocate: 1,
                buffer_pitch: ya_pitch,
            };
        }
    }

    std::ptr::write_unaligned(fbset, fb);
}

pub unsafe extern "win64" fn BinkRegisterFrameBuffers(
    bink: *mut c_void,
    fbset: *mut BinkFrameBuffers,
) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let ptr = if fbset.is_null() {
        (&raw mut inst.default_frame_buffers) as u64
    } else {
        fbset as u64
    };
    let open_flags = inst.rd_u32(0x020) & !0x100;
    inst.wr_u32(0x020, open_flags);
    inst.wr_u64(0x050, ptr);
    inst.wr_u64(0x0b0, ptr);
    if !fbset.is_null() {
        let fb_val = std::ptr::read_unaligned(fbset);
        let total = (fb_val.total_frames.max(0) as usize).min(3);
        for i in 0..total {
            fill_plane_set_black(fbset, i);
        }
    }
}

pub unsafe extern "win64" fn BinkAllocateFrameBuffers(
    bink: *mut c_void,
    set: *mut BinkFrameBuffers,
    minimum_alignment: u32,
) -> i32 {
    if bink.is_null() || set.is_null() {
        return 0;
    }
    let mut fb = std::ptr::read_unaligned(set);
    if fb.total_frames <= 0 || fb.ya_buffer_width == 0 || fb.ya_buffer_height == 0 {
        BinkGetFrameBuffersInfo(bink, set);
        fb = std::ptr::read_unaligned(set);
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let align = if minimum_alignment == 0 {
        16usize
    } else {
        (minimum_alignment as usize)
            .next_power_of_two()
            .clamp(16, 65536)
    };
    let ya_w = fb.ya_buffer_width;
    let ya_h = fb.ya_buffer_height;
    let crcb_w = fb.crcb_buffer_width;
    let crcb_h = fb.crcb_buffer_height;

    let mut alloc_plane = |plane: &mut BinkPlane, w: u32, h: u32, fill: u8| {
        if plane.allocate == 0 || !plane.buffer.is_null() || w == 0 || h == 0 {
            return;
        }
        if plane.buffer_pitch == 0 {
            plane.buffer_pitch = (w.saturating_add(15)) & !15;
        }
        let bytes = (plane.buffer_pitch as usize).saturating_mul(h as usize);
        if bytes == 0 {
            return;
        }
        let buf = vec![fill; bytes.saturating_add(align)];
        inst.owned_planes.push(buf);
        let raw = inst.owned_planes.last_mut().unwrap().as_mut_ptr() as usize;
        let aligned = (raw.saturating_add(align - 1)) & !(align - 1);
        plane.buffer = aligned as *mut c_void;
    };

    let total = (fb.total_frames.max(0) as usize).min(3);
    for i in 0..total {
        let ps = &mut fb.frames[i];
        alloc_plane(&mut ps.y_plane, ya_w, ya_h, 0x10);
        alloc_plane(&mut ps.cr_plane, crcb_w, crcb_h, 0x80);
        alloc_plane(&mut ps.cb_plane, crcb_w, crcb_h, 0x80);
        alloc_plane(&mut ps.a_plane, ya_w, ya_h, 0xFF);
        alloc_plane(&mut ps.h_plane, ya_w, ya_h, 0x00);
    }

    std::ptr::write_unaligned(set, fb);
    1
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

unsafe fn decode_frame_into_buffers(inst: &mut BinkInstance, decode_audio: bool) {
    let cur_frame = inst.rd_u32(0x00c).max(1);
    if inst.last_decoded_frame == cur_frame {
        return;
    }
    let fb = inst.rd_u64(0x0b0) as *mut BinkFrameBuffers;
    let video_flags = inst.rd_u32(0x024);
    let fourcc = inst.rd_u32(0x118);

    let t_read = Instant::now();
    let pkt_loaded = ensure_frame_packet_loaded(inst, cur_frame);
    let read_us = t_read.elapsed().as_micros() as u64;
    let packet_bytes = if pkt_loaded {
        inst.cached_packet_len as u64
    } else {
        0
    };
    inst.total_read_us = inst.total_read_us.saturating_add(read_us);
    inst.total_bytes_read = inst.total_bytes_read.saturating_add(packet_bytes);

    let mut audio_us = 0u64;
    if pkt_loaded && decode_audio && inst.file_num_tracks > 0 {
        let t0 = Instant::now();
        let pkt_len = inst.cached_packet_len;
        inst.audio_player.submit_frame_packets(
            &inst.frame_packet_buf[..pkt_len],
            &inst.cached_track_spans,
        );
        audio_us = t0.elapsed().as_micros() as u64;
        inst.total_audio_decomp_us = inst.total_audio_decomp_us.saturating_add(audio_us);
    }

    let mut video_us = 0u64;
    if inst.video_on && !fb.is_null() {
        let t_vid = Instant::now();
        let fb_val = std::ptr::read_unaligned(fb);
        let total = (fb_val.total_frames.max(1) as u32).min(3);
        let prev_idx = (fb_val.frame_num % total) as usize;
        let dst_idx = ((fb_val.frame_num + 1) % total) as usize;

        let mut decoded = false;
        if pkt_loaded {
            let pkt_len = inst.cached_packet_len;
            let video_off = inst.cached_video_offset;
            let is_key = inst.cached_packet_is_key;

            if is_kb2v_magic(fourcc) && video_off < pkt_len {
                let ya_w = fb_val.ya_buffer_width as usize;
                let ya_h = fb_val.ya_buffer_height as usize;
                let crcb_h = fb_val.crcb_buffer_height as usize;
                let dst_ps = fb_val.frames[dst_idx];
                let ya_pitch = dst_ps.y_plane.buffer_pitch as usize;
                let crcb_pitch = dst_ps.cr_plane.buffer_pitch as usize;
                let ya_bytes = ya_pitch.saturating_mul(ya_h);
                let crcb_bytes = crcb_pitch.saturating_mul(crcb_h);

                if !dst_ps.y_plane.buffer.is_null()
                    && !dst_ps.cr_plane.buffer.is_null()
                    && !dst_ps.cb_plane.buffer.is_null()
                    && ya_bytes > 0
                    && crcb_bytes > 0
                {
                    let dst_y = std::slice::from_raw_parts_mut(
                        dst_ps.y_plane.buffer as *mut u8,
                        ya_bytes,
                    );
                    let dst_cr = std::slice::from_raw_parts_mut(
                        dst_ps.cr_plane.buffer as *mut u8,
                        crcb_bytes,
                    );
                    let dst_cb = std::slice::from_raw_parts_mut(
                        dst_ps.cb_plane.buffer as *mut u8,
                        crcb_bytes,
                    );
                    if let Some(ref mut hevc) = inst.hevc_decoder {
                        match hevc.decode_hvcc_frame(
                            &inst.frame_packet_buf[video_off..pkt_len],
                            ya_w,
                            ya_h,
                            ya_pitch,
                            crcb_pitch,
                            dst_y,
                            dst_cr,
                            dst_cb,
                        ) {
                            Ok(()) => decoded = true,
                            Err(e) => {
                                log::warn!("bink2w64 (KB2v HEVC): frame {cur_frame} decode failed: {e}");
                            }
                        }
                    }
                }
            } else if is_bink2_magic(fourcc) && video_off + 8 <= pkt_len {
                let ya_w = fb_val.ya_buffer_width as usize;
                let ya_h = fb_val.ya_buffer_height as usize;
                let crcb_h = fb_val.crcb_buffer_height as usize;
                let dst_ps = fb_val.frames[dst_idx];
                let ref_ps = fb_val.frames[prev_idx];

                let ya_pitch = dst_ps.y_plane.buffer_pitch as usize;
                let crcb_pitch = dst_ps.cr_plane.buffer_pitch as usize;
                let ya_bytes = ya_pitch.saturating_mul(ya_h);
                let crcb_bytes = crcb_pitch.saturating_mul(crcb_h);

                if !dst_ps.y_plane.buffer.is_null()
                    && !dst_ps.cr_plane.buffer.is_null()
                    && !dst_ps.cb_plane.buffer.is_null()
                    && !ref_ps.y_plane.buffer.is_null()
                    && !ref_ps.cr_plane.buffer.is_null()
                    && !ref_ps.cb_plane.buffer.is_null()
                    && ya_bytes > 0
                    && crcb_bytes > 0
                {
                    let has_alpha = (video_flags & 0x0010_0000) != 0;
                    let has_hdr = (video_flags & 0x0000_0004) != 0;

                    // Handle single-frame buffers (`dst_idx == prev_idx`) without aliasing &mut/&
                    let same_buf = dst_idx == prev_idx
                        || dst_ps.y_plane.buffer == ref_ps.y_plane.buffer;

                    let ref_y_owned: Vec<u8>;
                    let ref_cr_owned: Vec<u8>;
                    let ref_cb_owned: Vec<u8>;
                    let ref_a_owned: Option<Vec<u8>>;
                    let ref_h_owned: Option<Vec<u8>>;

                    let (ref_y, ref_cr, ref_cb, ref_a, ref_h) = if same_buf {
                        ref_y_owned = std::slice::from_raw_parts(
                            ref_ps.y_plane.buffer as *const u8,
                            ya_bytes,
                        )
                        .to_vec();
                        ref_cr_owned = std::slice::from_raw_parts(
                            ref_ps.cr_plane.buffer as *const u8,
                            crcb_bytes,
                        )
                        .to_vec();
                        ref_cb_owned = std::slice::from_raw_parts(
                            ref_ps.cb_plane.buffer as *const u8,
                            crcb_bytes,
                        )
                        .to_vec();
                        ref_a_owned = if has_alpha && !ref_ps.a_plane.buffer.is_null() {
                            Some(
                                std::slice::from_raw_parts(
                                    ref_ps.a_plane.buffer as *const u8,
                                    ya_bytes,
                                )
                                .to_vec(),
                            )
                        } else {
                            None
                        };
                        ref_h_owned = if has_hdr && !ref_ps.h_plane.buffer.is_null() {
                            Some(
                                std::slice::from_raw_parts(
                                    ref_ps.h_plane.buffer as *const u8,
                                    ya_bytes,
                                )
                                .to_vec(),
                            )
                        } else {
                            None
                        };
                        (
                            ref_y_owned.as_slice(),
                            ref_cr_owned.as_slice(),
                            ref_cb_owned.as_slice(),
                            ref_a_owned.as_deref(),
                            ref_h_owned.as_deref(),
                        )
                    } else {
                        let ra = if has_alpha && !ref_ps.a_plane.buffer.is_null() {
                            Some(std::slice::from_raw_parts(
                                ref_ps.a_plane.buffer as *const u8,
                                ya_bytes,
                            ))
                        } else {
                            None
                        };
                        let rh = if has_hdr && !ref_ps.h_plane.buffer.is_null() {
                            Some(std::slice::from_raw_parts(
                                ref_ps.h_plane.buffer as *const u8,
                                ya_bytes,
                            ))
                        } else {
                            None
                        };
                        (
                            std::slice::from_raw_parts(
                                ref_ps.y_plane.buffer as *const u8,
                                ya_bytes,
                            ),
                            std::slice::from_raw_parts(
                                ref_ps.cr_plane.buffer as *const u8,
                                crcb_bytes,
                            ),
                            std::slice::from_raw_parts(
                                ref_ps.cb_plane.buffer as *const u8,
                                crcb_bytes,
                            ),
                            ra,
                            rh,
                        )
                    };

                    let dst_y = std::slice::from_raw_parts_mut(
                        dst_ps.y_plane.buffer as *mut u8,
                        ya_bytes,
                    );
                    let dst_cr = std::slice::from_raw_parts_mut(
                        dst_ps.cr_plane.buffer as *mut u8,
                        crcb_bytes,
                    );
                    let dst_cb = std::slice::from_raw_parts_mut(
                        dst_ps.cb_plane.buffer as *mut u8,
                        crcb_bytes,
                    );
                    let dst_a = if has_alpha && !dst_ps.a_plane.buffer.is_null() {
                        Some(std::slice::from_raw_parts_mut(
                            dst_ps.a_plane.buffer as *mut u8,
                            ya_bytes,
                        ))
                    } else {
                        None
                    };
                    let dst_h = if has_hdr && !dst_ps.h_plane.buffer.is_null() {
                        Some(std::slice::from_raw_parts_mut(
                            dst_ps.h_plane.buffer as *mut u8,
                            ya_bytes,
                        ))
                    } else {
                        None
                    };

                    let mut planes = FramePlanes {
                        ya_width: ya_w,
                        ya_height: ya_h,
                        ya_pitch,
                        crcb_pitch,
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
                    match decode_frame(
                        &inst.frame_packet_buf[video_off..pkt_len],
                        &inst.slices,
                        is_key,
                        has_alpha,
                        has_hdr,
                        &mut planes,
                    ) {
                        Ok(()) => decoded = true,
                        Err(e) => {
                            log::warn!("bink2w64: frame {cur_frame} decode failed: {e}");
                        }
                    }
                }
            }
        }

        if !decoded {
            fill_plane_set_black(fb, dst_idx);
        }
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*fb).frame_num), dst_idx as u32);
        video_us = t_vid.elapsed().as_micros() as u64;
        inst.total_video_decomp_us = inst.total_video_decomp_us.saturating_add(video_us);
    }

    if inst.frame_timings.len() >= 120 {
        inst.frame_timings.pop_front();
    }
    inst.frame_timings.push_back(FrameTimingSample {
        video_us,
        audio_us,
        read_us,
        blit_us: 0,
        packet_bytes,
    });

    inst.last_decoded_frame = cur_frame;
    inst.wr_u32(0x010, cur_frame);
    inst.played_frames = inst.played_frames.saturating_add(1);
    if inst.playback_start.is_none() {
        inst.playback_start = Some(Instant::now());
        inst.playback_base_frame = inst.played_frames.saturating_sub(1);
    }
}

pub unsafe extern "win64" fn BinkDoFrame(bink: *mut c_void) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    decode_frame_into_buffers(inst, true);
    0
}

pub unsafe extern "win64" fn BinkDoFramePlane(bink: *mut c_void, which_planes: u32) -> i32 {
    if bink.is_null() || which_planes == 0 {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    decode_frame_into_buffers(inst, true);
    0
}

pub unsafe extern "win64" fn BinkDoFrameAsync(
    bink: *mut c_void,
    thread_one: u32,
    thread_two: u32,
) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let _guard = AsyncDecodeGuard::acquire(&[thread_one, thread_two]);
    let inst = &mut *(bink as *mut BinkInstance);
    decode_frame_into_buffers(inst, true);
    1
}

pub unsafe extern "win64" fn BinkDoFrameAsyncMulti(
    bink: *mut c_void,
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let mut tids = Vec::new();
    if !threads.is_null() && thread_count > 0 {
        let count = (thread_count as usize).min(MAX_ASYNC_THREADS);
        tids.reserve(count);
        for i in 0..count {
            tids.push(std::ptr::read_unaligned(threads.add(i)));
        }
    }
    let _guard = AsyncDecodeGuard::acquire(&tids);
    let inst = &mut *(bink as *mut BinkInstance);
    decode_frame_into_buffers(inst, true);
    1
}

pub unsafe extern "win64" fn BinkDoFrameAsyncWait(bink: *mut c_void, us: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if !pool.in_flight.iter().any(|&n| n > 0) {
        return 1;
    }
    if us == 0 {
        return 0;
    }
    if us < 0 {
        while pool.in_flight.iter().any(|&n| n > 0) {
            pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
        }
        return 1;
    }
    let deadline = Instant::now() + Duration::from_micros(us as u64);
    while pool.in_flight.iter().any(|&n| n > 0) {
        let Some(rem) = deadline.checked_duration_since(Instant::now()) else {
            return 0;
        };
        let (next_pool, res) = cvar
            .wait_timeout(pool, rem)
            .unwrap_or_else(|e| e.into_inner());
        pool = next_pool;
        if !pool.in_flight.iter().any(|&n| n > 0) {
            return 1;
        }
        if res.timed_out() {
            return 0;
        }
    }
    1
}

pub unsafe extern "win64" fn BinkCopyToBuffer(
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
    BinkCopyToBufferRect(
        bink, dest, destpitch, destheight, destx, desty, 0, 0, width, height, flags,
    )
}

pub unsafe extern "win64" fn BinkCopyToBufferRect(
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

pub unsafe extern "win64" fn BinkService(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    // Keep audio stream paused state consistent with the BINK header.
    let paused = inst.rd_u32(0x11c) != 0;
    inst.audio_player.set_paused(paused);
}

pub unsafe extern "win64" fn BinkControlBackgroundIO(bink: *mut c_void, control: u32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    match control & 0xff {
        BINKBGIOSUSPEND => {
            inst.bg_io_suspended = inst.bg_io_suspended.saturating_add(1);
        }
        BINKBGIORESUME => {
            inst.bg_io_suspended = inst.bg_io_suspended.saturating_sub(1);
        }
        _ => {}
    }
    i32::from(inst.bg_io_suspended > 0)
}

pub unsafe extern "win64" fn BinkSetVideoOnOff(bink: *mut c_void, onoff: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.video_on = onoff != 0;
    i32::from(inst.video_on)
}

pub unsafe extern "win64" fn BinkNextFrame(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let cur = inst.rd_u32(0x00c).max(1);
    let will_loop = inst.rd_u32(0x3b8) != 0 || (inst.rd_u32(0x020) & 0x80) != 0;

    inst.wr_u32(0x010, cur); // LastFrameNum = FrameNum
    if cur < frames {
        inst.wr_u32(0x00c, cur + 1);
    } else if will_loop {
        inst.wr_u32(0x00c, 1);
    } else {
        inst.wr_u32(0x00c, frames);
    }
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.frame_advance_epoch = inst.frame_advance_epoch.wrapping_add(1);
}

pub unsafe extern "win64" fn BinkWait(bink: *mut c_void) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    if inst.rd_u32(0x01c) != 0 {
        return 0;
    }
    if inst.rd_u32(0x11c) != 0 {
        // Paused
        return 1;
    }
    if inst.played_frames == 0 {
        return 0;
    }
    let Some(start) = inst.playback_start else {
        return 0;
    };
    let fps_num = inst.rd_u32(0x014);
    let fps_den = inst.rd_u32(0x018).max(1);
    if fps_num == 0 {
        return 0;
    }
    let elapsed_frames = inst.played_frames.saturating_sub(inst.playback_base_frame) as u64;
    let due_us = elapsed_frames
        .saturating_mul(fps_den as u64)
        .saturating_mul(1_000_000)
        / (fps_num as u64);
    let elapsed = start.elapsed();
    let due = Duration::from_micros(due_us);
    if elapsed >= due {
        let frame_dur_us = (fps_den as u64).saturating_mul(1_000_000) / (fps_num as u64);
        if elapsed > due + Duration::from_micros(frame_dur_us.saturating_mul(4)) {
            // Re-anchor clock after a long stall so we don't burst frames
            inst.playback_start = Some(Instant::now());
            inst.playback_base_frame = inst.played_frames;
        }
        0
    } else {
        1
    }
}

pub unsafe extern "win64" fn BinkShouldSkip(_bink: *mut c_void) -> i32 {
    0
}

pub unsafe extern "win64" fn BinkPause(bink: *mut c_void, pause: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let paused = if pause != 0 { 1u32 } else { 0u32 };
    inst.wr_u32(0x11c, paused);
    inst.playback_start = None;
    inst.playback_base_frame = inst.played_frames;
    inst.audio_player.set_paused(paused != 0);
    paused as i32
}

pub unsafe extern "win64" fn BinkSetWillLoop(bink: *mut c_void, onoff: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let open_flags = inst.rd_u32(0x020);
    if onoff != 0 {
        inst.wr_u32(0x020, open_flags | 0x80);
        inst.wr_u32(0x3b8, 1);
    } else {
        inst.wr_u32(0x020, open_flags & !0x80);
        inst.wr_u32(0x3b8, 0);
    }
}

pub unsafe extern "win64" fn BinkGetKeyFrame(bink: *mut c_void, frame: u32, flags: i32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let f = frame.clamp(1, frames);
    let exclude_exact = (flags as u8 & 0x80) != 0;
    let is_key = |idx_1: u32| -> bool {
        let i = idx_1.saturating_sub(1) as usize;
        inst.frame_offsets
            .get(i)
            .map(|&v| (v & 1) != 0)
            .unwrap_or(idx_1 == 1)
    };
    if !exclude_exact && is_key(f) {
        return f;
    }
    match flags & 0x7f {
        0 => {
            // Previous keyframe
            let mut k = f.saturating_sub(1);
            while k > 1 && !is_key(k) {
                k -= 1;
            }
            k.max(1)
        }
        1 => {
            // Next keyframe
            let mut k = f + 1;
            while k <= frames {
                if is_key(k) {
                    return k;
                }
                k += 1;
            }
            0
        }
        2 => {
            // Nearest keyframe
            let mut prev = f as i64 - 1;
            let mut next = f as i64 + 1;
            loop {
                if prev >= 1 && is_key(prev as u32) {
                    return prev as u32;
                }
                if next <= frames as i64 && is_key(next as u32) {
                    return next as u32;
                }
                if prev < 1 && next > frames as i64 {
                    return 0;
                }
                prev -= 1;
                next += 1;
            }
        }
        _ => 0,
    }
}

pub unsafe extern "win64" fn BinkGoto(bink: *mut c_void, frame: u32, flags: i32) {
    if bink.is_null() {
        return;
    }
    let quick = (flags & 1) != 0;
    let kf = if quick {
        1
    } else {
        BinkGetKeyFrame(bink, frame, 0).max(1)
    };
    let inst = &mut *(bink as *mut BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let cur = inst.rd_u32(0x00c).max(1);
    let target = frame.clamp(1, frames);

    inst.wr_u32(0x010, cur);
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.audio_player.flush_and_reset();

    if !quick {
        let has_io = !inst.from_memory_ptr.is_null()
            || inst.is_user_io
            || inst.user_file_slot != 0
            || inst.host_file.is_some();
        if kf < target && inst.rd_u64(0x0b0) != 0 && has_io {
            let start = if cur >= kf && cur < target { cur } else { kf };
            for f in start..target {
                inst.wr_u32(0x00c, f);
                inst.last_decoded_frame = 0;
                inst.cached_packet_frame = 0;
                decode_frame_into_buffers(inst, false);
            }
        }
    }

    inst.wr_u32(0x00c, target);
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.seek_epoch = inst.seek_epoch.wrapping_add(1);
    inst.frame_advance_epoch = inst.frame_advance_epoch.wrapping_add(1);
    inst.playback_start = None;
    inst.playback_base_frame = inst.played_frames;
}

pub unsafe extern "win64" fn BinkSetVolume(bink: *mut c_void, track_id: u32, volume: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_volume(track_id, volume);
}

pub unsafe extern "win64" fn BinkSetPan(bink: *mut c_void, track_id: u32, pan: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_pan(track_id, pan);
}

pub unsafe extern "win64" fn BinkSetSpeakerVolumes(
    bink: *mut c_void,
    track_id: u32,
    speaker_indexes: *const u32,
    volumes: *const i32,
    total: u32,
) {
    if bink.is_null() || volumes.is_null() || total == 0 {
        return;
    }
    let count = (total as usize).min(16);
    let mut left_max = 0i32;
    let mut right_max = 0i32;
    for i in 0..count {
        let vol =
            std::ptr::read_unaligned(volumes.add(i)).clamp(0, bink2_audio::BINK_MAX_VOLUME);
        let spk = if !speaker_indexes.is_null() {
            std::ptr::read_unaligned(speaker_indexes.add(i))
        } else {
            i as u32
        };
        if count == 1 {
            left_max = left_max.max(vol);
            right_max = right_max.max(vol);
        } else {
            match spk {
                // FL, BL, SL
                0 | 4 | 6 => left_max = left_max.max(vol),
                // FR, BR, SR
                1 | 5 | 7 => right_max = right_max.max(vol),
                // FC, LFE, or higher center/mono speakers
                _ => {
                    left_max = left_max.max(vol);
                    right_max = right_max.max(vol);
                }
            }
        }
    }
    let eff_vol = left_max.max(right_max);
    let eff_pan = if eff_vol == 0 {
        bink2_audio::BINK_CENTER_PAN
    } else if left_max >= right_max {
        (((right_max as i64) * (bink2_audio::BINK_CENTER_PAN as i64)) / (left_max as i64)) as i32
    } else {
        bink2_audio::BINK_MAX_PAN
            - ((((left_max as i64) * (bink2_audio::BINK_CENTER_PAN as i64)) / (right_max as i64))
                as i32)
    };
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_volume(track_id, eff_vol);
    inst.audio_player.set_pan(track_id, eff_pan);
}

pub unsafe extern "win64" fn BinkSetSoundOnOff(bink: *mut c_void, onoff: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let _ = inst.audio_player.set_sound_on(onoff != 0);
    i32::from(inst.audio_player.is_sound_on())
}

pub unsafe extern "win64" fn BinkGetTrackMaxSize(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.max_size)
}

pub unsafe extern "win64" fn BinkGetTrackID(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.track_id)
}

pub unsafe extern "win64" fn BinkGetTrackType(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.raw_type)
}

pub unsafe extern "win64" fn BinkOpenTrack(bink: *mut c_void, track_index: u32) -> *mut BinkTrack {
    if bink.is_null() {
        return std::ptr::null_mut();
    }
    let inst = &*(bink as *const BinkInstance);
    let Some(&info) = inst.audio_tracks.get(track_index as usize) else {
        return std::ptr::null_mut();
    };
    let fourcc = inst.rd_u32(0x118);
    let version_b = fourcc.to_le_bytes() == *b"BIKb";
    let sample_rate = if info.sample_rate >= 300 {
        info.sample_rate
    } else {
        44_100
    };
    let channels = info.channels();
    let decoder = bink2_audio::BinkAudioDecoder::new(sample_rate, channels, info.use_dct(), version_b);
    let default_max = (decoder.output_frames_per_block() * (channels as usize) * 2) as u32;
    let max_size = if info.max_size > 0 {
        info.max_size
    } else {
        default_max
    };
    let track = Box::new(BinkTrack {
        frequency: sample_rate,
        bits: 16,
        channels: channels as u32,
        max_size,
        bink,
        sndcomp: 0,
        track_index: track_index as i32,
        _pad: 0,
        decoder,
        last_frame_epoch: u64::MAX,
        seek_epoch: inst.seek_epoch,
        pcm_scratch: Vec::new(),
    });
    Box::into_raw(track)
}

pub unsafe extern "win64" fn BinkCloseTrack(track: *mut BinkTrack) {
    if track.is_null() {
        return;
    }
    drop(Box::from_raw(track));
}

pub unsafe extern "win64" fn BinkGetTrackData(track: *mut BinkTrack, dest: *mut c_void) -> u32 {
    if track.is_null() || dest.is_null() {
        return 0;
    }
    let tr = &mut *track;
    if tr.bink.is_null() || tr.track_index < 0 {
        return 0;
    }
    let inst = &mut *(tr.bink as *mut BinkInstance);

    if tr.seek_epoch != inst.seek_epoch {
        tr.decoder.reset();
        tr.last_frame_epoch = u64::MAX;
        tr.seek_epoch = inst.seek_epoch;
    }
    if tr.last_frame_epoch == inst.frame_advance_epoch {
        return 0;
    }

    let cur_frame = inst.rd_u32(0x00c).max(1);
    if !ensure_frame_packet_loaded(inst, cur_frame) {
        return 0;
    }
    let track_idx = tr.track_index as usize;
    let Some(&(start, end)) = inst.cached_track_spans.get(track_idx) else {
        return 0;
    };
    tr.last_frame_epoch = inst.frame_advance_epoch;

    if end <= start || end > inst.cached_packet_len {
        return 0;
    }
    let pkt = &inst.frame_packet_buf[start..end];
    let n_samples = tr.decoder.decode_packet_i16(pkt, &mut tr.pcm_scratch);
    if n_samples == 0 {
        return 0;
    }

    let max_samples = if tr.max_size > 0 {
        (tr.max_size as usize) / 2
    } else {
        n_samples
    };
    let copy_samples = n_samples.min(max_samples);
    let dst_bytes = dest as *mut u8;
    for (i, &sample) in tr.pcm_scratch[..copy_samples].iter().enumerate() {
        let le = sample.to_le_bytes();
        std::ptr::copy_nonoverlapping(le.as_ptr(), dst_bytes.add(i * 2), 2);
    }
    (copy_samples * 2) as u32
}

pub unsafe extern "win64" fn BinkLoadSubtitles(bink: *mut c_void, name: *const u8) -> i32 {
    if bink.is_null() || name.is_null() {
        return 0;
    }
    let Ok(path_str) = CStr::from_ptr(name as *const std::ffi::c_char).to_str() else {
        return 0;
    };
    let content = crate::path::to_host(path_str)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .or_else(|| std::fs::read_to_string(path_str).ok());
    let Some(text) = content else {
        return 0;
    };
    let parsed = parse_srt_subtitles(&text);
    let count = parsed.len() as i32;
    let inst = &mut *(bink as *mut BinkInstance);
    inst.subtitles = parsed;
    count
}

pub unsafe extern "win64" fn BinkCurrentSubtitle(bink: *mut c_void) -> *const u8 {
    if bink.is_null() {
        return std::ptr::null();
    }
    let inst = &*(bink as *const BinkInstance);
    if inst.subtitles.is_empty() {
        return std::ptr::null();
    }
    let cur_frame = inst.rd_u32(0x00c).saturating_sub(1) as u64;
    let fps_num = inst.rd_u32(0x014).max(1) as u64;
    let fps_den = inst.rd_u32(0x018).max(1) as u64;
    let time_ms = cur_frame
        .saturating_mul(fps_den)
        .saturating_mul(1000)
        / fps_num;
    for entry in &inst.subtitles {
        if time_ms >= entry.start_ms && time_ms <= entry.end_ms {
            return entry.text_nul.as_ptr();
        }
    }
    std::ptr::null()
}

pub unsafe extern "win64" fn BinkGetSubtitleByIndex(bink: *mut c_void, index: i32) -> *const u8 {
    if bink.is_null() || index < 0 {
        return std::ptr::null();
    }
    let inst = &*(bink as *const BinkInstance);
    inst.subtitles
        .get(index as usize)
        .map_or(std::ptr::null(), |e| e.text_nul.as_ptr())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bink_lifecycle_and_offsets() {
        unsafe {
            let mut bk2 = [0u8; 1024];
            let hdr = Bk2FileHeader {
                magic: *b"KB2n",
                file_size_minus_8: 100_000 - 8,
                frames: 120,
                largest_frame_size: 8192,
                frames_dup: 120,
                width: 1920,
                height: 1080,
                fps_num: 30,
                fps_den: 1,
                video_flags: 0,
                num_tracks: 0,
            };
            std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

            // Open from memory (BINKFROMMEMORY = 0x0400_0000, BINKNOFRAMEBUFFERS = 0x0400)
            let hbink = BinkOpenWithOptions(bk2.as_ptr(), std::ptr::null(), 0x0400_0400);
            assert!(!hbink.is_null());
            let base = hbink as *const u8;
            assert_eq!(std::ptr::read_unaligned(base.add(0x00) as *const u32), 1920);
            assert_eq!(std::ptr::read_unaligned(base.add(0x04) as *const u32), 1080);
            assert_eq!(std::ptr::read_unaligned(base.add(0x08) as *const u32), 120);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x14) as *const u32), 30);
            assert_eq!(std::ptr::read_unaligned(base.add(0x18) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x134) as *const u32), 2);

            // GPU buffers info returns 0 to select StandardDecompressor
            let mut gpu = [0u8; 0xe0];
            assert_eq!(
                BinkGetGPUDataBuffersInfo(hbink, gpu.as_mut_ptr() as *mut c_void),
                0
            );

            // FrameBuffers info & registration
            let mut fb: BinkFrameBuffers = std::mem::zeroed();
            BinkGetFrameBuffersInfo(hbink, &mut fb);
            assert_eq!(fb.total_frames, 2);
            assert_eq!(fb.ya_buffer_width, 1920);
            assert_eq!(fb.ya_buffer_height, 1088);
            assert_eq!(fb.crcb_buffer_width, 960);
            assert_eq!(fb.crcb_buffer_height, 544);
            assert_eq!(fb.frames[0].y_plane.allocate, 1);

            BinkRegisterFrameBuffers(hbink, &mut fb);
            assert_eq!(
                std::ptr::read_unaligned(base.add(0x50) as *const usize),
                (&mut fb as *mut BinkFrameBuffers) as usize
            );
            assert_eq!(
                std::ptr::read_unaligned(base.add(0xb0) as *const usize),
                (&mut fb as *mut BinkFrameBuffers) as usize
            );

            assert_eq!(BinkWait(hbink), 0);
            assert_eq!(BinkDoFrame(hbink), 0);
            let threads = [2u32, 3u32];
            assert_eq!(BinkDoFrameAsyncMulti(hbink, threads.as_ptr(), 2), 1);
            assert_eq!(BinkDoFrameAsync(hbink, 2, 3), 1);
            assert_eq!(BinkDoFrameAsyncWait(hbink, -1), 1);
            BinkNextFrame(hbink);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 2);
            assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 1);

            let mut sum = BinkSummary::default();
            BinkGetSummary(hbink, &mut sum);
            assert_eq!(sum.total_frames, 120);
            assert!(sum.total_played_frames >= 2);

            // Non-looping video reaches FrameNum == Frames cleanly without stalling
            BinkSetWillLoop(hbink, 0);
            BinkGoto(hbink, 119, 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 119);
            BinkNextFrame(hbink);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 120);
            assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 119);
            BinkNextFrame(hbink);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 120);
            assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 120);

            BinkPause(hbink, 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x11c) as *const u32), 1);
            assert_eq!(BinkWait(hbink), 1);
            BinkPause(hbink, 0);
            assert_eq!(BinkWait(hbink), 0);
            BinkSetWillLoop(hbink, 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x3b8) as *const u32), 1);
            BinkNextFrame(hbink);
            assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 120);

            BinkClose(hbink);
        }
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[test]
    fn kb2v_videotoolbox_hevc_end_to_end_pixel_accuracy() {
        use bink2_codec::hevc_container::{build_kb2v, parse_annex_b_hevc};
        use std::io::Write;
        use std::process::{Command, Stdio};

        let w = 320usize;
        let h = 192usize;
        // Distinct (Y, Cb, Cr) frames so any Y/Cb/Cr plane swap or range bug fails immediately.
        let colors: [(u8, u8, u8); 4] = [
            (81, 90, 240),   // red
            (41, 240, 110),  // blue
            (145, 54, 34),   // green
            (220, 128, 128), // near-white
        ];
        let mut raw_yuv = Vec::with_capacity(colors.len() * (w * h * 3 / 2));
        for &(y, cb, cr) in &colors {
            raw_yuv.extend(std::iter::repeat_n(y, w * h));
            raw_yuv.extend(std::iter::repeat_n(cb, (w / 2) * (h / 2)));
            raw_yuv.extend(std::iter::repeat_n(cr, (w / 2) * (h / 2)));
        }

        let Ok(mut child) = Command::new("ffmpeg")
            .args([
                "-y", "-v", "error",
                "-f", "rawvideo", "-pix_fmt", "yuv420p", "-s", "320x192", "-r", "30",
                "-color_range", "pc", "-i", "pipe:0",
                "-c:v", "hevc_videotoolbox", "-b:v", "1500k", "-bf", "0", "-g", "30",
                "-allow_sw", "1", "-color_range", "pc", "-f", "hevc", "pipe:1",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        else {
            // Skip gracefully if ffmpeg is absent in a minimal CI environment.
            return;
        };
        child.stdin.take().unwrap().write_all(&raw_yuv).unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "ffmpeg hevc_videotoolbox failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );

        let parsed = parse_annex_b_hevc(&out.stdout).expect("parse annex-b HEVC");
        assert_eq!(parsed.frames.len(), colors.len());
        let kb2v = build_kb2v(w as u32, h as u32, 30, 1, true, &parsed).expect("build KB2v");

        unsafe {
            // Open from memory with auto-allocated frame buffers (BINKFROMMEMORY = 0x0400_0000).
            let hbink = BinkOpenWithOptions(kb2v.as_ptr(), std::ptr::null(), 0x0400_0000);
            assert!(!hbink.is_null(), "BinkOpenWithOptions failed on KB2v");

            let mut fb: BinkFrameBuffers = std::mem::zeroed();
            BinkGetFrameBuffersInfo(hbink, &mut fb);
            assert_eq!(fb.ya_buffer_width, 320);
            assert_eq!(fb.ya_buffer_height, 192);
            assert_eq!(fb.crcb_buffer_width, 160);
            assert_eq!(fb.crcb_buffer_height, 96);

            let inst = &*(hbink as *const BinkInstance);
            for (idx, &(exp_y, exp_cb, exp_cr)) in colors.iter().enumerate() {
                assert_eq!(BinkDoFrame(hbink), 0);
                let live_fb = &*(inst.rd_u64(0x0b0) as *const BinkFrameBuffers);
                let ps = &live_fb.frames[live_fb.frame_num as usize];
                let y_pitch = ps.y_plane.buffer_pitch as usize;
                let c_pitch = ps.cb_plane.buffer_pitch as usize;
                let got_y = *(ps.y_plane.buffer as *const u8).add((h / 2) * y_pitch + (w / 2));
                let got_cb = *(ps.cb_plane.buffer as *const u8).add((h / 4) * c_pitch + (w / 4));
                let got_cr = *(ps.cr_plane.buffer as *const u8).add((h / 4) * c_pitch + (w / 4));
                assert!(
                    got_y.abs_diff(exp_y) <= 14
                        && got_cb.abs_diff(exp_cb) <= 14
                        && got_cr.abs_diff(exp_cr) <= 14,
                    "frame {} mismatch: got YCbCr=({},{},{}) expected ({},{},{})",
                    idx + 1,
                    got_y,
                    got_cb,
                    got_cr,
                    exp_y,
                    exp_cb,
                    exp_cr
                );
                BinkNextFrame(hbink);
            }

            // Seek back to frame 1 (IDR) and verify decode still succeeds cleanly.
            BinkGoto(hbink, 1, 0);
            assert_eq!(BinkDoFrame(hbink), 0);
            let live_fb = &*(inst.rd_u64(0x0b0) as *const BinkFrameBuffers);
            let ps = &live_fb.frames[live_fb.frame_num as usize];
            let got_cr = *(ps.cr_plane.buffer as *const u8)
                .add((h / 4) * (ps.cr_plane.buffer_pitch as usize) + (w / 4));
            assert!(got_cr.abs_diff(colors[0].2) <= 14, "seeked frame 1 Cr={got_cr}");

            BinkClose(hbink);
        }
    }

    fn build_synthetic_bink_with_audio(
        magic: [u8; 4],
        tracks: &[(u32, u32, u16, u32)], // (max_size, sample_rate, flags, track_id)
        frame_track_packets: &[Vec<Vec<u8>>],
    ) -> Vec<u8> {
        let num_frames = frame_track_packets.len() as u32;
        let num_tracks = tracks.len();
        let fourcc = u32::from_le_bytes(magic);
        let extra_hdr_bytes = if has_extended_header(fourcc) { 4 } else { 0 };
        let tables_off = 0x2c + extra_hdr_bytes;
        let tables_bytes = (num_tracks * 3 + (num_frames as usize) + 1) * 4;
        let data_start = (tables_off + tables_bytes + 3) & !3;

        let mut frame_blobs: Vec<Vec<u8>> = Vec::with_capacity(frame_track_packets.len());
        let mut frame_offsets: Vec<u32> = Vec::with_capacity(frame_track_packets.len() + 1);
        let mut cur_off = data_start;
        let mut largest_frame = 0u32;

        for (f_idx, track_pkts) in frame_track_packets.iter().enumerate() {
            let mut blob = Vec::new();
            for pkt in track_pkts {
                blob.extend_from_slice(&(pkt.len() as u32).to_le_bytes());
                blob.extend_from_slice(pkt);
            }
            // Pad frame blob to 4-byte boundary.
            while (blob.len() & 3) != 0 {
                blob.push(0);
            }
            largest_frame = largest_frame.max(blob.len() as u32);
            let is_key = if f_idx == 0 { 1u32 } else { 0u32 };
            frame_offsets.push(((cur_off as u32) & !3) | is_key);
            cur_off += blob.len();
            frame_blobs.push(blob);
        }
        frame_offsets.push((cur_off as u32) & !3);

        let total_len = cur_off;
        let hdr = Bk2FileHeader {
            magic,
            file_size_minus_8: (total_len.saturating_sub(8)) as u32,
            frames: num_frames,
            largest_frame_size: largest_frame.max(64),
            frames_dup: num_frames,
            width: 320,
            height: 180,
            fps_num: 30,
            fps_den: 1,
            video_flags: 0,
            num_tracks: num_tracks as u32,
        };

        let mut out = Vec::with_capacity(total_len);
        out.extend_from_slice(&hdr.to_bytes());
        out.resize(tables_off, 0);
        for &(max_size, _, _, _) in tracks {
            out.extend_from_slice(&max_size.to_le_bytes());
        }
        for &(_, sr, fl, _) in tracks {
            let raw_type = (sr & 0xFFFF) | ((fl as u32) << 16);
            out.extend_from_slice(&raw_type.to_le_bytes());
        }
        for &(_, _, _, tid) in tracks {
            out.extend_from_slice(&tid.to_le_bytes());
        }
        for &fo in &frame_offsets {
            out.extend_from_slice(&fo.to_le_bytes());
        }
        while out.len() < data_start {
            out.push(0);
        }
        for blob in frame_blobs {
            out.extend_from_slice(&blob);
        }
        assert_eq!(out.len(), total_len);
        out
    }

    #[test]
    fn failed_open_returns_null_without_synthetic_header() {
        unsafe {
            let missing = b"/nonexistent_dir_bink_test/missing_movie.bk2\0";
            let h = BinkOpenWithOptions(missing.as_ptr(), std::ptr::null(), 0);
            assert!(h.is_null(), "missing file must return NULL, not a fake header");

            let bad_magic = [0xDEu8; 256];
            let h2 = BinkOpenWithOptions(bad_magic.as_ptr(), std::ptr::null(), 0x0400_0000);
            assert!(h2.is_null(), "invalid magic in memory buffer must return NULL");
        }
    }

    #[test]
    fn bink_audio_tracks_playback_and_track_data_api() {
        use bink2_audio::test_util::{encode_simple_block, wrap_audio_packet, BitWriter};
        use bink2_audio::{BINK_AUD_16BITS, BINK_AUD_STEREO, BINK_AUD_USEDCT};

        let _audio_guard = crate::audio::audio_test_lock();
        crate::audio::backend::force_backend(Some(crate::audio::backend::Backend::Null));

        // Track 0: 44100 Hz stereo 16-bit DCT, id=10
        // Track 1: 22050 Hz mono 16-bit RDFT, id=20
        let t0_flags = BINK_AUD_16BITS | BINK_AUD_STEREO | BINK_AUD_USEDCT;
        let t1_flags = BINK_AUD_16BITS;

        // Frame 1 audio packets (non-zero DCT tone on track 0, non-zero RDFT sine on track 1)
        let mut bw_t0_f1 = BitWriter::new();
        encode_simple_block(
            &mut bw_t0_f1,
            44100,
            2,
            true,
            (0, 0, false),
            (0, 0, false),
            65,
            Some((10, 12, false)),
        );
        let pkt_t0_f1 = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_t0_f1.into_bytes());

        let mut bw_t1_f1 = BitWriter::new();
        encode_simple_block(
            &mut bw_t1_f1,
            22050,
            1,
            false,
            (0, 0, false),
            (0, 0, false),
            62,
            Some((9, 14, false)),
        );
        let pkt_t1_f1 = wrap_audio_packet((960 * 2) as u32, &bw_t1_f1.into_bytes());

        // Frame 2 audio packets (silence on track 0, truncated corrupt packet on track 1)
        let mut bw_t0_f2 = BitWriter::new();
        encode_simple_block(
            &mut bw_t0_f2,
            44100,
            2,
            true,
            (0, 0, false),
            (0, 0, false),
            0,
            None,
        );
        let pkt_t0_f2 = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_t0_f2.into_bytes());
        let pkt_t1_f2_truncated = vec![0x80, 0x07, 0x00, 0x00, 0x12, 0x34];

        let tracks = [
            ((1920 * 2 * 2) as u32, 44100u32, t0_flags, 10u32),
            ((960 * 2) as u32, 22050u32, t1_flags, 20u32),
        ];

        // Test BOTH Bink 2 (`KB2i`) and Bink 1 (`BIKi`) headers.
        for magic in [*b"KB2i", *b"BIKi"] {
            let bink_bytes = build_synthetic_bink_with_audio(
                magic,
                &tracks,
                &[
                    vec![pkt_t0_f1.clone(), pkt_t1_f1.clone()],
                    vec![pkt_t0_f2.clone(), pkt_t1_f2_truncated.clone()],
                ],
            );

            unsafe {
                assert!(!BinkOpenXAudio2(0, 0).is_null());
                assert_eq!(BinkSetSoundSystem2(BinkOpenXAudio2 as *const () as usize, 0, 0), 1);

                // Select track_id = 20 via BinkSetSoundTrack + BINKSNDTRACK (0x4000)
                let sel_id = 20u32;
                BinkSetSoundTrack(1, &sel_id);
                let hbink = BinkOpenWithOptions(
                    bink_bytes.as_ptr(),
                    std::ptr::null(),
                    0x0400_0400 | 0x4000, // BINKFROMMEMORY | BINKNOFRAMEBUFFERS | BINKSNDTRACK
                );
                assert!(!hbink.is_null());

                // Verify track header metadata queries.
                assert_eq!(BinkGetTrackID(hbink, 0), 10);
                assert_eq!(BinkGetTrackID(hbink, 1), 20);
                assert_eq!(BinkGetTrackID(hbink, 2), 0);
                assert_eq!(BinkGetTrackMaxSize(hbink, 0), 1920 * 4);
                assert_eq!(BinkGetTrackMaxSize(hbink, 1), 960 * 2);
                assert_eq!(
                    BinkGetTrackType(hbink, 0),
                    44100u32 | ((t0_flags as u32) << 16)
                );
                assert_eq!(
                    BinkGetTrackType(hbink, 1),
                    22050u32 | ((t1_flags as u32) << 16)
                );

                // Verify selected playback track and volume/pan/pause/sound-on-off controls.
                assert_eq!(
                    (*(hbink as *const BinkInstance)).audio_player.track_count(),
                    1
                );
                BinkSetVolume(hbink, 20, 16384);
                BinkSetPan(hbink, 20, 49152);
                assert_eq!(
                    (*(hbink as *const BinkInstance)).audio_player.track_volume_pan(0),
                    Some((16384, 49152))
                );

                // Pause before decoding frame 1 so the null clock does not drain the ring immediately.
                assert_eq!(BinkPause(hbink, 1), 1);
                assert_eq!(BinkDoFrame(hbink), 0);
                assert_eq!(
                    (*(hbink as *const BinkInstance))
                        .audio_player
                        .ring_queued_frames(),
                    960
                );

                // BinkGoto must flush queued audio frames and reset decoder overlap state.
                BinkGoto(hbink, 1, 1);
                assert_eq!(
                    (*(hbink as *const BinkInstance))
                        .audio_player
                        .ring_queued_frames(),
                    0
                );
                assert_eq!(BinkPause(hbink, 0), 0);

                // Turning sound off mutes and clears; turning back on re-enables.
                assert_eq!(BinkSetSoundOnOff(hbink, 0), 0);
                assert_eq!(BinkSetSoundOnOff(hbink, 1), 1);

                // Pull decoded 16-bit PCM directly via BinkOpenTrack / BinkGetTrackData.
                let htr0 = BinkOpenTrack(hbink, 0);
                let htr1 = BinkOpenTrack(hbink, 1);
                assert!(!htr0.is_null() && !htr1.is_null());
                assert!(BinkOpenTrack(hbink, 2).is_null());
                assert_eq!((*htr0).frequency, 44100);
                assert_eq!((*htr0).bits, 16);
                assert_eq!((*htr0).channels, 2);
                assert_eq!((*htr0).max_size, 1920 * 4);
                assert_eq!((*htr1).frequency, 22050);
                assert_eq!((*htr1).channels, 1);

                let mut pcm0 = vec![0u8; 1920 * 4];
                let got0 = BinkGetTrackData(htr0, pcm0.as_mut_ptr() as *mut c_void);
                assert_eq!(got0, 1920 * 4);
                let samples0: Vec<i16> = pcm0
                    .chunks_exact(2)
                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                    .collect();
                let peak0 = samples0.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
                assert!(peak0 > 1000, "DCT track 0 should have non-zero PCM, peak={peak0}");
                // Second read on same frame without BinkNextFrame returns 0.
                assert_eq!(BinkGetTrackData(htr0, pcm0.as_mut_ptr() as *mut c_void), 0);

                let mut pcm1 = vec![0u8; 960 * 2];
                let got1 = BinkGetTrackData(htr1, pcm1.as_mut_ptr() as *mut c_void);
                assert_eq!(got1, 960 * 2);
                let samples1: Vec<i16> = pcm1
                    .chunks_exact(2)
                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                    .collect();
                let peak1 = samples1.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
                assert!(peak1 > 1000, "RDFT track 1 should have non-zero PCM, peak={peak1}");

                // Advance to frame 2: track 1 has truncated packet — must fail gracefully (0 bytes, no panic).
                BinkNextFrame(hbink);
                assert_eq!(BinkDoFrame(hbink), 0);
                let got1_f2 = BinkGetTrackData(htr1, pcm1.as_mut_ptr() as *mut c_void);
                assert_eq!(got1_f2, 0);

                BinkCloseTrack(htr0);
                BinkCloseTrack(htr1);
                BinkClose(hbink);

                // BinkSetSoundTrack(0, NULL) + BINKSNDTRACK opens 0 playback tracks cleanly.
                BinkSetSoundTrack(0, std::ptr::null());
                let hbink_muted = BinkOpenWithOptions(
                    bink_bytes.as_ptr(),
                    std::ptr::null(),
                    0x0400_0400 | 0x4000,
                );
                assert!(!hbink_muted.is_null());
                assert_eq!((*(hbink_muted as *const BinkInstance)).audio_player.track_count(), 0);
                BinkClose(hbink_muted);
            }
        }

        // Restore global track selection state for subsequent tests.
        *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = None;
        SOUND_DISABLED.store(false, Ordering::Release);
    }

    #[test]
    fn bink_copy_to_buffer_and_rect_formats() {
        unsafe {
            // Minimal 32x16 full-range KB2n movie with alpha flag (0x0010_0000 | 0x10)
            let mut bk2 = [0u8; 256];
            let hdr = Bk2FileHeader {
                magic: *b"KB2n",
                file_size_minus_8: 256 - 8,
                frames: 2,
                largest_frame_size: 64,
                frames_dup: 2,
                width: 32,
                height: 16,
                fps_num: 30,
                fps_den: 1,
                video_flags: 0x0010_0010, // alpha + full range
                num_tracks: 0,
            };
            std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

            // BINKFROMMEMORY (0x0400_0000) + auto-allocated frame buffers + alpha
            let hbink = BinkOpenWithOptions(
                bk2.as_ptr(),
                std::ptr::null(),
                0x0400_0000 | 0x0010_0000,
            );
            assert!(!hbink.is_null());

            // Populate the active decoded YCrCbA planes with known full-range values:
            // Left half (x < 16): pure red (Y=76, Cr=255, Cb=85, A=0x80)
            // Right half (x >= 16): pure white (Y=255, Cr=128, Cb=128, A=0xCC)
            let inst = &*(hbink as *const BinkInstance);
            let fb_ptr = inst.rd_u64(0x0b0) as *const BinkFrameBuffers;
            let fb = std::ptr::read_unaligned(fb_ptr);
            let ps = fb.frames[fb.frame_num as usize];
            let y_pitch = ps.y_plane.buffer_pitch as usize;
            let c_pitch = ps.cr_plane.buffer_pitch as usize;
            let a_pitch = ps.a_plane.buffer_pitch as usize;
            for y in 0..16usize {
                for x in 0..32usize {
                    let (yv, av) = if x < 16 { (76u8, 0x80u8) } else { (255u8, 0xCCu8) };
                    *(ps.y_plane.buffer as *mut u8).add(y * y_pitch + x) = yv;
                    *(ps.a_plane.buffer as *mut u8).add(y * a_pitch + x) = av;
                }
            }
            for cy in 0..8usize {
                for cx in 0..16usize {
                    let (cr, cb) = if cx < 8 { (255u8, 85u8) } else { (128u8, 128u8) };
                    *(ps.cr_plane.buffer as *mut u8).add(cy * c_pitch + cx) = cr;
                    *(ps.cb_plane.buffer as *mut u8).add(cy * c_pitch + cx) = cb;
                }
            }

            // 1. BINKSURFACE32 (BGRA opaque: [B, G, R, 0xFF])
            let mut bgra = vec![0u8; 32 * 16 * 4];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    bgra.as_mut_ptr() as *mut c_void,
                    32 * 4,
                    16,
                    0,
                    0,
                    BINKSURFACE32
                ),
                0
            );
            // Red pixel at (0, 0)
            assert!(bgra[0] <= 4 && bgra[1] <= 4 && bgra[2] >= 250 && bgra[3] == 0xFF);
            // White pixel at (20, 0)
            let woff = 20 * 4;
            assert!(bgra[woff] >= 252 && bgra[woff + 1] >= 252 && bgra[woff + 2] >= 252 && bgra[woff + 3] == 0xFF);

            // 2. BINKSURFACE32RA (RGBA with alpha plane: [R, G, B, A])
            let mut rgba = vec![0u8; 32 * 16 * 4];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    rgba.as_mut_ptr() as *mut c_void,
                    32 * 4,
                    16,
                    0,
                    0,
                    BINKSURFACE32RA
                ),
                0
            );
            assert!(rgba[0] >= 250 && rgba[1] <= 4 && rgba[2] <= 4 && rgba[3] == 0x80);
            assert!(rgba[woff] >= 252 && rgba[woff + 1] >= 252 && rgba[woff + 2] >= 252 && rgba[woff + 3] == 0xCC);

            // 3. BINKSURFACE24R (RGB24: [R, G, B])
            let mut rgb24 = vec![0u8; 32 * 16 * 3];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    rgb24.as_mut_ptr() as *mut c_void,
                    32 * 3,
                    16,
                    0,
                    0,
                    BINKSURFACE24R
                ),
                0
            );
            assert!(rgb24[0] >= 250 && rgb24[1] <= 4 && rgb24[2] <= 4);

            // 4. BINKSURFACE565 (16-bit RGB565)
            let mut rgb565 = vec![0u8; 32 * 16 * 2];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    rgb565.as_mut_ptr() as *mut c_void,
                    32 * 2,
                    16,
                    0,
                    0,
                    BINKSURFACE565
                ),
                0
            );
            let px_red = u16::from_le_bytes([rgb565[0], rgb565[1]]);
            assert_eq!(px_red >> 11, 31); // full red
            assert_eq!((px_red >> 5) & 0x3f, 0);
            assert_eq!(px_red & 0x1f, 0);
            let px_white = u16::from_le_bytes([rgb565[20 * 2], rgb565[20 * 2 + 1]]);
            assert_eq!(px_white, 0xFFFF);

            // 5. BinkCopyToBufferRect sub-rectangle into offset destination
            let mut rect_dst = vec![0x11u8; 32 * 16 * 4];
            assert_eq!(
                BinkCopyToBufferRect(
                    hbink,
                    rect_dst.as_mut_ptr() as *mut c_void,
                    32 * 4,
                    16,
                    4,
                    2,
                    16,
                    0,
                    8,
                    4,
                    BINKSURFACE32A
                ),
                0
            );
            // Untouched pixel at (0, 0) stays 0x11
            assert_eq!(&rect_dst[0..4], &[0x11, 0x11, 0x11, 0x11]);
            // Copied white pixel with alpha 0xCC at (4, 2)
            let roff = (2 * 32 + 4) * 4;
            assert!(
                rect_dst[roff] >= 252
                    && rect_dst[roff + 1] >= 252
                    && rect_dst[roff + 2] >= 252
                    && rect_dst[roff + 3] == 0xCC
            );

            // 6. BinkCopyToBuffer with YUY2 & YV12
            let mut yuy2 = vec![0u8; 32 * 16 * 2];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    yuy2.as_mut_ptr() as *mut c_void,
                    32 * 2,
                    16,
                    0,
                    0,
                    BINKSURFACEYUY2
                ),
                0
            );
            assert_eq!(&yuy2[0..4], &[76, 85, 76, 255]);

            let mut yv12 = vec![0u8; 32 * 16 + 16 * 8 * 2];
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    yv12.as_mut_ptr() as *mut c_void,
                    32,
                    16,
                    0,
                    0,
                    BINKSURFACEYV12
                ),
                0
            );
            assert_eq!(yv12[0], 76);
            assert_eq!(yv12[20], 255);
            assert_eq!(yv12[32 * 16], 255); // V = Cr
            assert_eq!(yv12[32 * 16 + 16 * 8], 85); // U = Cb

            // 7. Turning video off makes BinkCopyToBuffer return 1 (skipped) and BinkGetRects return 0
            assert_eq!(BinkGetRects(hbink, 0), 1);
            assert_eq!(BinkSetVideoOnOff(hbink, 0), 0);
            assert_eq!(BinkGetRects(hbink, 0), 0);
            assert_eq!(
                BinkCopyToBuffer(
                    hbink,
                    bgra.as_mut_ptr() as *mut c_void,
                    32 * 4,
                    16,
                    0,
                    0,
                    BINKSURFACE32
                ),
                1
            );
            assert_eq!(BinkSetVideoOnOff(hbink, 1), 1);
            assert_eq!(BinkGetRects(hbink, 0), 1);

            BinkClose(hbink);
        }
    }

    #[test]
    fn bink_async_threads_multi_coordination() {
        unsafe {
            assert_eq!(BinkStartAsyncThread(0, std::ptr::null()), 1);
            assert_eq!(BinkStartAsyncThread(1, std::ptr::null()), 1);
            assert_eq!(BinkStartAsyncThread(2, std::ptr::null()), 1);
            assert_eq!(BinkStartAsyncThread(-1, std::ptr::null()), 0);
            assert_eq!(BinkStartAsyncThread(MAX_ASYNC_THREADS as i32, std::ptr::null()), 0);

            let hbink = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0000);
            assert!(!hbink.is_null());

            let tids = [0u32, 1u32, 2u32];
            assert_eq!(BinkDoFrameAsyncMulti(hbink, tids.as_ptr(), 3), 1);
            assert_eq!(BinkDoFrameAsyncWait(hbink, -1), 1);

            // Simulate an in-flight decode on thread 1 and verify BinkWaitStopAsyncThreadsMulti
            // blocks until the in-flight decode guard drops.
            let guard = AsyncDecodeGuard::acquire(&[1]);
            assert_eq!(BinkRequestStopAsyncThreadsMulti(tids.as_ptr(), 3), 1);
            assert_eq!(BinkDoFrameAsyncWait(hbink, 0), 0);

            let finished = Arc::new(AtomicBool::new(false));
            let finished_bg = finished.clone();
            let waiter = std::thread::spawn(move || {
                let ids = [0u32, 1u32, 2u32];
                let rc = BinkWaitStopAsyncThreadsMulti(ids.as_ptr(), 3);
                finished_bg.store(true, Ordering::Release);
                rc
            });

            std::thread::sleep(Duration::from_millis(15));
            assert!(
                !finished.load(Ordering::Acquire),
                "wait must not return while decode is in flight"
            );
            drop(guard);
            assert_eq!(waiter.join().unwrap(), 1);
            assert!(finished.load(Ordering::Acquire));

            {
                let (lock, _) = async_pool();
                let pool = lock.lock().unwrap_or_else(|e| e.into_inner());
                assert_eq!(pool.states[0], AsyncThreadState::Stopped);
                assert_eq!(pool.states[1], AsyncThreadState::Stopped);
                assert_eq!(pool.states[2], AsyncThreadState::Stopped);
            }

            // Null pointer with positive count fails; null pointer with 0 stops all cleanly.
            assert_eq!(BinkRequestStopAsyncThreadsMulti(std::ptr::null(), 2), 0);
            assert_eq!(BinkWaitStopAsyncThreadsMulti(std::ptr::null(), 2), 0);
            assert_eq!(BinkRequestStopAsyncThreadsMulti(std::ptr::null(), 0), 1);
            assert_eq!(BinkWaitStopAsyncThreadsMulti(std::ptr::null(), 0), 1);

            BinkClose(hbink);
        }
    }

    #[test]
    fn bink_error_get_set_and_open_failure() {
        unsafe {
            BinkSetError(b"Custom Bink error 42\0".as_ptr());
            let got = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
                .to_str()
                .unwrap();
            assert_eq!(got, "Custom Bink error 42");

            // NULL clears error string to empty.
            BinkSetError(std::ptr::null());
            let cleared = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
                .to_str()
                .unwrap();
            assert_eq!(cleared, "");

            // Failed BinkOpen populates a descriptive error.
            let h = BinkOpen(b"/nonexistent_bink_err_test/nope.bk2\0".as_ptr(), 0);
            assert!(h.is_null());
            let open_err = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
                .to_str()
                .unwrap();
            assert!(!open_err.is_empty());

            // Successful open clears error.
            let ok = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0000);
            assert!(!ok.is_null());
            let ok_err = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
                .to_str()
                .unwrap();
            assert_eq!(ok_err, "");
            BinkClose(ok);
        }
    }

    #[test]
    fn bink_util_mutex_and_helpers() {
        unsafe {
            assert!(BinkUtilCPUs() >= 1);

            // BinkUtilMalloc / BinkUtilFree (32-byte aligned)
            let p = BinkUtilMalloc(128);
            assert!(!p.is_null());
            assert_eq!((p as usize) % 32, 0);
            std::ptr::write_bytes(p as *mut u8, 0xA5, 128);
            BinkUtilFree(p);
            BinkUtilFree(std::ptr::null_mut());

            // BinkUtilMutex* with cross-thread timeout and unlock wakeup
            let m = BinkUtilMutexCreate();
            assert!(!m.is_null());
            BinkUtilMutexLock(m);
            // Re-entrant lock on same thread succeeds immediately
            assert_eq!(BinkUtilMutexLockTimeOut(m, 0), 1);
            BinkUtilMutexUnlock(m);

            // Another thread attempting lock with short timeout must time out (return 0)
            let m_addr = m as usize;
            let t1 = std::thread::spawn(move || {
                BinkUtilMutexLockTimeOut(m_addr as *mut c_void, 20)
            });
            assert_eq!(t1.join().unwrap(), 0);

            // Release lock while another thread waits with generous timeout -> succeeds (1)
            let t2 = std::thread::spawn(move || {
                let got = BinkUtilMutexLockTimeOut(m_addr as *mut c_void, 500);
                if got != 0 {
                    BinkUtilMutexUnlock(m_addr as *mut c_void);
                }
                got
            });
            std::thread::sleep(Duration::from_millis(15));
            BinkUtilMutexUnlock(m);
            assert_eq!(t2.join().unwrap(), 1);

            BinkUtilMutexDestroy(m);
            // Operations on destroyed/null mutex are safe no-ops
            assert_eq!(BinkUtilMutexLockTimeOut(m, 0), 0);
            assert_eq!(BinkUtilMutexLockTimeOut(std::ptr::null_mut(), 0), 0);
            BinkUtilMutexDestroy(std::ptr::null_mut());

            // Global sound lock/unlock
            BinkUtilSoundGlobalLock();
            BinkUtilSoundGlobalLock(); // re-entrant safe
            BinkUtilSoundGlobalUnlock();
            BinkUtilSoundGlobalUnlock();
        }
    }

    #[test]
    fn bink_realtime_allocate_plane_bgio_and_timer() {
        unsafe {
            let t0 = RADTimerRead();
            std::thread::sleep(Duration::from_millis(12));
            let t1 = RADTimerRead();
            assert!(
                t1.wrapping_sub(t0) >= 5,
                "RADTimerRead should advance monotonically: t0={t0}, t1={t1}"
            );

            let hbink = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0400);
            assert!(!hbink.is_null());

            // BinkAllocateFrameBuffers with 256-byte minimum alignment
            let mut fb: BinkFrameBuffers = std::mem::zeroed();
            assert_eq!(BinkAllocateFrameBuffers(hbink, &mut fb, 256), 1);
            assert!(!fb.frames[0].y_plane.buffer.is_null());
            assert_eq!((fb.frames[0].y_plane.buffer as usize) % 256, 0);
            assert!(!fb.frames[0].cr_plane.buffer.is_null());
            assert_eq!((fb.frames[0].cr_plane.buffer as usize) % 256, 0);
            BinkRegisterFrameBuffers(hbink, &mut fb);

            // BinkDoFramePlane + BinkService + BinkControlBackgroundIO
            assert_eq!(BinkDoFramePlane(hbink, 0x1f), 0);
            BinkService(hbink);
            assert_eq!(BinkControlBackgroundIO(hbink, BINKBGIOSUSPEND | BINKBGIOWAIT), 1);
            assert_eq!(BinkControlBackgroundIO(hbink, BINKBGIORESUME), 0);

            // BinkGetRealtime
            let mut rt = BinkRealtime::default();
            BinkGetRealtime(hbink, &mut rt, 30);
            assert_eq!(rt.frame_num, 1);
            assert_eq!(rt.frame_rate, 30);
            assert_eq!(rt.frame_rate_div, 1);
            assert_eq!(rt.frames, 1);
            assert!(rt.frames_time >= 33);
            assert!(rt.read_buffer_size > 0);

            // BinkGetPlatformInfo returns 0 cleanly
            assert_eq!(BinkGetPlatformInfo(0, std::ptr::null_mut()), 0);

            BinkClose(hbink);
            BinkFreeGlobals();
        }
    }

    #[test]
    fn bink_sound_open_variants_sound_system4_and_speaker_volumes() {
        let _audio_guard = crate::audio::audio_test_lock();
        crate::audio::backend::force_backend(Some(crate::audio::backend::Backend::Null));

        unsafe {
            assert!(!BinkOpenXAudio27(0, 0).is_null());
            assert!(!BinkOpenXAudio28(0, 0).is_null());
            assert!(!BinkOpenXAudio29(0, 0).is_null());
            assert!(!BinkOpenDirectSound(0).is_null());
            assert!(!BinkOpenWaveOut(0).is_null());
            assert!(BinkFindXAudio2WinDevice(0, 0).is_null());

            assert_eq!(
                BinkSetSoundSystem4(BinkOpenXAudio29 as *const () as usize, 1, 2, 3, 4),
                1
            );
            assert!(!SOUND_DISABLED.load(Ordering::Acquire));

            // Open a synthetic 1-track video and verify BinkSetSpeakerVolumes updates volume & pan
            let tracks = [(1920u32 * 4, 44100u32, 0x7000u16, 7u32)];
            let bink_bytes = build_synthetic_bink_with_audio(*b"KB2i", &tracks, &[vec![vec![]]]);
            let hbink = BinkOpen(bink_bytes.as_ptr(), 0x0400_0400);
            assert!(!hbink.is_null());

            let spk_idx = [0u32, 1u32]; // FL, FR
            let vols = [16384i32, 8192i32]; // Left 0.5, Right 0.25 -> vol=16384, pan=16384
            BinkSetSpeakerVolumes(hbink, 7, spk_idx.as_ptr(), vols.as_ptr(), 2);
            assert_eq!(
                (*(hbink as *const BinkInstance)).audio_player.track_volume_pan(0),
                Some((16384, 16384))
            );

            // Right-only speaker volume -> pan = 65536
            let vols_r = [0i32, 32768i32];
            BinkSetSpeakerVolumes(hbink, 7, spk_idx.as_ptr(), vols_r.as_ptr(), 2);
            assert_eq!(
                (*(hbink as *const BinkInstance)).audio_player.track_volume_pan(0),
                Some((32768, 65536))
            );

            BinkClose(hbink);

            // Null sound system disables audio cleanly
            assert_eq!(BinkSetSoundSystem4(0, 0, 0, 0, 0), 1);
            assert!(SOUND_DISABLED.load(Ordering::Acquire));
            SOUND_DISABLED.store(false, Ordering::Release);
        }
    }

    #[test]
    fn bink_subtitles_and_preopen_setters() {
        unsafe {
            // Pre-open setters: BinkSetFrameRate + BINKFRAMERATE (0x1000)
            BinkSetFrameRate(60, 1);
            BinkSetIOSize(131_072);
            BinkSetSimulate(500_000);
            BinkSetFileOffset(0);
            BinkSetIO(0);

            let mut bk2 = [0u8; 256];
            let hdr = Bk2FileHeader {
                magic: *b"KB2n",
                file_size_minus_8: 256 - 8,
                frames: 300,
                largest_frame_size: 64,
                frames_dup: 300,
                width: 320,
                height: 180,
                fps_num: 30,
                fps_den: 1,
                video_flags: 0,
                num_tracks: 0,
            };
            std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

            let hbink = BinkOpen(
                bk2.as_ptr(),
                0x0400_0400 | 0x0000_1000 | 0x0100_0000 | 0x0040_0000,
            );
            assert!(!hbink.is_null());
            let base = hbink as *const u8;
            assert_eq!(std::ptr::read_unaligned(base.add(0x14) as *const u32), 60);
            assert_eq!(std::ptr::read_unaligned(base.add(0x18) as *const u32), 1);

            // Write a temp .srt file and test BinkLoadSubtitles / BinkCurrentSubtitle / BinkGetSubtitleByIndex
            let tmp_srt = std::env::temp_dir().join(format!(
                "bink2w64_sub_test_{}.srt",
                std::process::id()
            ));
            let srt_body = "1\n00:00:00,000 --> 00:00:01,000\nHello Overwatch\n\n2\n00:00:02,000 --> 00:00:03,000\nSecond Line\n";
            std::fs::write(&tmp_srt, srt_body).unwrap();
            let mut path_nul = tmp_srt.to_string_lossy().as_bytes().to_vec();
            path_nul.push(0);

            assert_eq!(BinkLoadSubtitles(hbink, path_nul.as_ptr()), 2);
            let sub0 = BinkGetSubtitleByIndex(hbink, 0);
            assert!(!sub0.is_null());
            assert_eq!(
                CStr::from_ptr(sub0 as *const std::ffi::c_char).to_str().unwrap(),
                "Hello Overwatch"
            );
            let sub1 = BinkGetSubtitleByIndex(hbink, 1);
            assert!(!sub1.is_null());
            assert_eq!(
                CStr::from_ptr(sub1 as *const std::ffi::c_char).to_str().unwrap(),
                "Second Line"
            );
            assert!(BinkGetSubtitleByIndex(hbink, 2).is_null());
            assert!(BinkGetSubtitleByIndex(hbink, -1).is_null());

            // At frame 1 (t = 0 ms), subtitle 0 is active
            let cur0 = BinkCurrentSubtitle(hbink);
            assert!(!cur0.is_null());
            assert_eq!(
                CStr::from_ptr(cur0 as *const std::ffi::c_char).to_str().unwrap(),
                "Hello Overwatch"
            );

            // At frame 151 at 60fps (t = 2500 ms), subtitle 1 is active
            BinkGoto(hbink, 151, 1);
            let cur1 = BinkCurrentSubtitle(hbink);
            assert!(!cur1.is_null());
            assert_eq!(
                CStr::from_ptr(cur1 as *const std::ffi::c_char).to_str().unwrap(),
                "Second Line"
            );

            // At frame 91 at 60fps (t = 1500 ms), no subtitle is active
            BinkGoto(hbink, 91, 1);
            assert!(BinkCurrentSubtitle(hbink).is_null());

            let _ = std::fs::remove_file(&tmp_srt);
            BinkClose(hbink);
            BinkFreeGlobals();
        }
    }
}
