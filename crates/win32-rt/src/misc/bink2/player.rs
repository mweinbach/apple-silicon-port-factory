//! `BinkInstance` state, container opening/closing, stream/packet I/O,
//! frame buffer management, and Bink 2 / VideoToolbox HEVC decode pipeline.

use super::async_threads::{
    set_bink_error, spawn_async_job, AsyncDecodeGuard, MAX_ASYNC_THREADS, OS_CLOSE, OS_OPEN,
    OS_READ, OS_SEEK, PREOPEN_FILE_OFFSET, PREOPEN_FRAME_RATE_DEN, PREOPEN_FRAME_RATE_NUM,
    PREOPEN_IO_SIZE, PREOPEN_SIMULATE, PREOPEN_USER_IO, SOUND_DISABLED, SOUND_TRACK_SELECTION,
};
use super::audio as bink2_audio;
use super::blit::fill_plane_set_black;
use super::subtitles::SubtitleEntry;
use super::types::{
    BinkFrameBuffers, BinkFramePlaneSet, BinkOpenOptions, BinkPlane, FrameTimingSample,
    DEFAULT_BINK_COLORSPACE,
};
use super::vt as bink2_vt;
use crate::strings::{read_ansi_bytes_z, DEFAULT_MAX_STR_LEN};
use bink2_codec::codec::{compute_slices, decode_frame, FramePlanes, SliceInfo};
use bink2_codec::hevc_container::{is_kb2v_magic, Kb2vMeta, KB2V_META_SIZE};
use bink2_codec::stream::{expand_frame_offsets, has_extended_header, is_bink2_magic, Bk2FileHeader};
use bink2_codec::tables::FULLRANGE_BINK_COLORSPACE;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Completion payload produced by an asynchronous worker decode and committed back onto
/// `BinkInstance` on the calling thread when `BinkDoFrameAsyncWait` (or a synchronizing API)
/// observes completion.
struct AsyncDecodeResult {
    video_us: u64,
    video_buf: Vec<u8>,
    hevc_decoder: Option<bink2_vt::VtHevcDecoder>,
}

struct AsyncJobInner {
    done: bool,
    result: Option<AsyncDecodeResult>,
}

/// Per-handle in-flight asynchronous video decode state.
pub(crate) struct PendingAsyncDecode {
    sync: Arc<(Mutex<AsyncJobInner>, Condvar)>,
    cur_frame: u32,
    fb_ptr: usize,
    dst_idx: usize,
    pub(crate) same_buf: bool,
    read_us: u64,
    audio_us: u64,
    packet_bytes: u64,
}

/// Snapshotted destination/reference plane addresses handed to the background worker.
///
/// Safety invariant:
/// - `dst_*` points to the back-buffer slot `dst_idx` exclusively owned by this in-flight job.
/// - `ref_*` points to the read-only front-buffer slot `prev_idx` (or `0` when `ref_owned` is used
///   for single-frame `same_buf` streams).
/// - Every handle-mutating API (`BinkClose`, `BinkNextFrame`, `BinkGoto`,
///   `BinkRegisterFrameBuffers`, `BinkDoFrame*`) drains this job before freeing or reassigning
///   buffers.
struct WorkerPlaneJob {
    cur_frame: u32,
    fourcc: u32,
    video_flags: u32,
    slices: SliceInfo,
    is_key: bool,
    has_video_payload: bool,
    ya_w: usize,
    ya_h: usize,
    ya_pitch: usize,
    crcb_pitch: usize,
    ya_bytes: usize,
    crcb_bytes: usize,
    dst_y: usize,
    dst_cr: usize,
    dst_cb: usize,
    dst_a: usize,
    dst_h: usize,
    dst_y_alloc: bool,
    dst_cr_alloc: bool,
    dst_cb_alloc: bool,
    dst_a_alloc: bool,
    dst_h_alloc: bool,
    ref_y: usize,
    ref_cr: usize,
    ref_cb: usize,
    ref_a: usize,
    ref_h: usize,
    ref_owned: Option<OwnedRefPlanes>,
    video_buf: Vec<u8>,
    hevc_decoder: Option<bink2_vt::VtHevcDecoder>,
}

struct OwnedRefPlanes {
    y: Vec<u8>,
    cr: Vec<u8>,
    cb: Vec<u8>,
    a: Option<Vec<u8>>,
    h: Option<Vec<u8>>,
}

/// Ensures the waiting caller is always unblocked even if a decode task unexpectedly unwinds.
struct AsyncCompletionSignal {
    sync: Arc<(Mutex<AsyncJobInner>, Condvar)>,
    result: Option<AsyncDecodeResult>,
    guard: Option<AsyncDecodeGuard>,
}

impl Drop for AsyncCompletionSignal {
    fn drop(&mut self) {
        // Release the thread-pool slot before waking `BinkDoFrameAsyncWait` so callers observing
        // completion also observe `in_flight == 0` with zero lock nesting.
        drop(self.guard.take());
        let (lock, cvar) = &*self.sync;
        let mut inner = lock.lock().unwrap_or_else(|e| e.into_inner());
        inner.result = self.result.take();
        inner.done = true;
        drop(inner);
        cvar.notify_all();
    }
}

/// Heap-allocated Bink handle whose prefix `[0..0x800]` matches the binary layout
/// of `BINK` in `bink2w64.dll` (`0x6e8` bytes).
#[repr(C, align(64))]
pub struct BinkInstance {
    pub raw: [u8; 0x800],
    pub(crate) is_user_io: bool,
    pub(crate) from_memory_ptr: *const u8,
    pub(crate) from_memory_len: usize,
    pub(crate) file_offset: u64,
    pub(crate) user_file_slot: u64,
    pub(crate) host_file: Option<std::fs::File>,
    pub(crate) file_num_tracks: usize,
    pub(crate) slices: SliceInfo,
    pub played_frames: u32,
    pub(crate) last_decoded_frame: u32,
    pub default_frame_buffers: BinkFrameBuffers,
    pub(crate) owned_planes: Vec<Vec<u8>>,
    pub(crate) playback_start: Option<Instant>,
    pub(crate) playback_base_frame: u32,
    pub(crate) track_max_sizes: Vec<i32>,
    pub(crate) track_flags: Vec<u32>,
    pub(crate) track_ids: Vec<i32>,
    pub(crate) open_track_indices: Vec<i32>,
    pub(crate) audio_tracks: Vec<bink2_audio::AudioTrackHeader>,
    pub(crate) audio_player: bink2_audio::BinkAudioPlayer,
    pub(crate) frame_offsets: Vec<u64>,
    pub(crate) io_buffer: Vec<u8>,
    pub(crate) frame_packet_buf: Vec<u8>,
    pub(crate) cached_packet_frame: u32,
    pub(crate) cached_packet_len: usize,
    pub(crate) cached_packet_is_key: bool,
    pub(crate) cached_video_offset: usize,
    pub(crate) cached_track_spans: Vec<(usize, usize)>,
    pub(crate) frame_advance_epoch: u64,
    pub(crate) seek_epoch: u64,
    pub(crate) total_audio_decomp_us: u64,
    pub(crate) total_video_decomp_us: u64,
    pub(crate) total_read_us: u64,
    pub(crate) total_blit_us: u64,
    pub(crate) total_bytes_read: u64,
    pub(crate) video_on: bool,
    pub(crate) bg_io_suspended: i32,
    pub(crate) simulate_rate: u32,
    pub(crate) frame_timings: VecDeque<FrameTimingSample>,
    pub(crate) subtitles: Vec<SubtitleEntry>,
    pub(crate) hevc_decoder: Option<bink2_vt::VtHevcDecoder>,
    pub(crate) pending_async: Option<PendingAsyncDecode>,
    pub(crate) async_video_buf: Vec<u8>,
}

impl BinkInstance {
    #[inline]
    pub(crate) unsafe fn rd_u32(&self, off: usize) -> u32 {
        std::ptr::read_unaligned(self.raw.as_ptr().add(off) as *const u32)
    }
    #[inline]
    pub(crate) unsafe fn wr_u32(&mut self, off: usize, v: u32) {
        std::ptr::write_unaligned(self.raw.as_mut_ptr().add(off) as *mut u32, v);
    }
    #[inline]
    pub(crate) unsafe fn rd_u64(&self, off: usize) -> u64 {
        std::ptr::read_unaligned(self.raw.as_ptr().add(off) as *const u64)
    }
    #[inline]
    pub(crate) unsafe fn wr_u64(&mut self, off: usize, v: u64) {
        std::ptr::write_unaligned(self.raw.as_mut_ptr().add(off) as *mut u64, v);
    }
    #[inline]
    pub(crate) unsafe fn bio_ptr(&mut self) -> *mut u8 {
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
        std::ptr::copy_nonoverlapping(
            inst.from_memory_ptr.add(*mem_pos),
            dst.as_mut_ptr(),
            dst.len(),
        );
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
pub(crate) unsafe fn ensure_frame_packet_loaded(
    inst: &mut BinkInstance,
    cur_frame: u32,
) -> bool {
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

// ============================================================================
// Lifecycle (Open / Close)
// ============================================================================

pub(crate) unsafe fn open_with_options(
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
        pending_async: None,
        async_video_buf: Vec::new(),
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
        } else {
            let raw_name = read_ansi_bytes_z(name, DEFAULT_MAX_STR_LEN);
            if let Ok(path_str) = std::str::from_utf8(&raw_name) {
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
    }

    let valid_magic = header_ok
        && (&hdr.magic[..3] == b"KB2" || &hdr.magic[..3] == b"BIK")
        && hdr.frames > 0
        && hdr.width > 0
        && hdr.height > 0;

    if !valid_magic {
        set_bink_error("Error opening Bink file.");
        close(Box::into_raw(inst) as *mut c_void);
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
            close(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        }
        let Some(meta) = Kb2vMeta::parse(&meta_raw) else {
            log::error!("bink2w64: invalid KB2v HEVC secondary header");
            set_bink_error("Invalid KB2v HEVC header.");
            close(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        };
        let ps_len = meta.param_sets_padded_len();
        let mut ps_buf = vec![0u8; ps_len];
        if !read_open_stream(&mut inst, is_from_memory, &mut mem_pos, &mut ps_buf) {
            set_bink_error("Error reading KB2v parameter sets.");
            close(Box::into_raw(inst) as *mut c_void);
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
                close(Box::into_raw(inst) as *mut c_void);
                return std::ptr::null_mut();
            }
        }
        let tbl_bytes_slice =
            std::slice::from_raw_parts_mut(tables_buf.as_mut_ptr() as *mut u8, tables_bytes);
        if !read_open_stream(&mut inst, is_from_memory, &mut mem_pos, tbl_bytes_slice) {
            close(Box::into_raw(inst) as *mut c_void);
            return std::ptr::null_mut();
        }
    } else {
        if extra_hdr_bytes > 0 {
            let _ = read_open_stream(&mut inst, is_from_memory, &mut mem_pos, &mut extra_hdr_buf);
        }
        let tbl_bytes_slice =
            std::slice::from_raw_parts_mut(tables_buf.as_mut_ptr() as *mut u8, tables_bytes);
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
            if let Some(ref sel) = *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner())
            {
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
        let mut io_buf_size =
            effective_io_size.max((hdr.largest_frame_size as u64).saturating_add(4096));
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

pub(crate) unsafe fn close(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let mut inst = Box::from_raw(bink as *mut BinkInstance);
    // Ensure any in-flight worker decode finishes writing plane buffers before releasing I/O,
    // owned planes, or the BinkInstance itself.
    drain_async_decode(&mut inst);
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

// ============================================================================
// Frame Buffer Registration & Allocation
// ============================================================================

pub(crate) unsafe fn get_gpu_data_buffers_info(gpu: *mut c_void) -> i32 {
    if !gpu.is_null() {
        std::ptr::write_bytes(gpu as *mut u8, 0, 0xe0);
    }
    // Returning 0 selects CPU standard decompressor (`BinkGetFrameBuffersInfo` /
    // `BinkRegisterFrameBuffers`) so our native ARM64 Bink 2 decoder writes planar YCrCb(A/H).
    0
}

pub(crate) unsafe fn register_gpu_data_buffers(bink: *mut c_void, gpu: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.wr_u64(0x1c0, gpu as u64);
}

pub(crate) unsafe fn get_frame_buffers_info(bink: *mut c_void, fbset: *mut BinkFrameBuffers) {
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

pub(crate) unsafe fn register_frame_buffers(bink: *mut c_void, fbset: *mut BinkFrameBuffers) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    drain_async_decode(inst);
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

pub(crate) unsafe fn allocate_frame_buffers(
    bink: *mut c_void,
    set: *mut BinkFrameBuffers,
    minimum_alignment: u32,
) -> i32 {
    if bink.is_null() || set.is_null() {
        return 0;
    }
    let mut fb = std::ptr::read_unaligned(set);
    if fb.total_frames <= 0 || fb.ya_buffer_width == 0 || fb.ya_buffer_height == 0 {
        get_frame_buffers_info(bink, set);
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

// ============================================================================
// Frame Decode Pipeline (Bink 2 + VideoToolbox HEVC + Bink Audio)
// ============================================================================

pub(crate) unsafe fn decode_frame_into_buffers(inst: &mut BinkInstance, decode_audio: bool) {
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
                                log::warn!(
                                    "bink2w64 (KB2v HEVC): frame {cur_frame} decode failed: {e}"
                                );
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

unsafe fn execute_worker_plane_job(mut job: WorkerPlaneJob) -> AsyncDecodeResult {
    let t_vid = Instant::now();
    let mut decoded = false;

    if job.has_video_payload {
        if is_kb2v_magic(job.fourcc) && !job.video_buf.is_empty() {
            if job.dst_y != 0
                && job.dst_cr != 0
                && job.dst_cb != 0
                && job.ya_bytes > 0
                && job.crcb_bytes > 0
            {
                let dst_y = std::slice::from_raw_parts_mut(job.dst_y as *mut u8, job.ya_bytes);
                let dst_cr = std::slice::from_raw_parts_mut(job.dst_cr as *mut u8, job.crcb_bytes);
                let dst_cb = std::slice::from_raw_parts_mut(job.dst_cb as *mut u8, job.crcb_bytes);
                if let Some(ref mut hevc) = job.hevc_decoder {
                    match hevc.decode_hvcc_frame(
                        &job.video_buf,
                        job.ya_w,
                        job.ya_h,
                        job.ya_pitch,
                        job.crcb_pitch,
                        dst_y,
                        dst_cr,
                        dst_cb,
                    ) {
                        Ok(()) => decoded = true,
                        Err(e) => {
                            log::warn!(
                                "bink2w64 (KB2v HEVC): frame {} decode failed: {e}",
                                job.cur_frame
                            );
                        }
                    }
                }
            }
        } else if is_bink2_magic(job.fourcc)
            && job.video_buf.len() >= 8
            && job.dst_y != 0
            && job.dst_cr != 0
            && job.dst_cb != 0
            && job.ya_bytes > 0
            && job.crcb_bytes > 0
        {
            let has_alpha = (job.video_flags & 0x0010_0000) != 0;
            let has_hdr = (job.video_flags & 0x0000_0004) != 0;

            let refs_valid = job.ref_owned.is_some()
                || (job.ref_y != 0 && job.ref_cr != 0 && job.ref_cb != 0);
            if refs_valid {
                let (ref_y, ref_cr, ref_cb, ref_a, ref_h) = if let Some(ref owned) = job.ref_owned {
                    (
                        owned.y.as_slice(),
                        owned.cr.as_slice(),
                        owned.cb.as_slice(),
                        owned.a.as_deref(),
                        owned.h.as_deref(),
                    )
                } else {
                    let ra = if has_alpha && job.ref_a != 0 {
                        Some(std::slice::from_raw_parts(
                            job.ref_a as *const u8,
                            job.ya_bytes,
                        ))
                    } else {
                        None
                    };
                    let rh = if has_hdr && job.ref_h != 0 {
                        Some(std::slice::from_raw_parts(
                            job.ref_h as *const u8,
                            job.ya_bytes,
                        ))
                    } else {
                        None
                    };
                    (
                        std::slice::from_raw_parts(job.ref_y as *const u8, job.ya_bytes),
                        std::slice::from_raw_parts(job.ref_cr as *const u8, job.crcb_bytes),
                        std::slice::from_raw_parts(job.ref_cb as *const u8, job.crcb_bytes),
                        ra,
                        rh,
                    )
                };

                let dst_y = std::slice::from_raw_parts_mut(job.dst_y as *mut u8, job.ya_bytes);
                let dst_cr = std::slice::from_raw_parts_mut(job.dst_cr as *mut u8, job.crcb_bytes);
                let dst_cb = std::slice::from_raw_parts_mut(job.dst_cb as *mut u8, job.crcb_bytes);
                let dst_a = if has_alpha && job.dst_a != 0 {
                    Some(std::slice::from_raw_parts_mut(
                        job.dst_a as *mut u8,
                        job.ya_bytes,
                    ))
                } else {
                    None
                };
                let dst_h = if has_hdr && job.dst_h != 0 {
                    Some(std::slice::from_raw_parts_mut(
                        job.dst_h as *mut u8,
                        job.ya_bytes,
                    ))
                } else {
                    None
                };

                let mut planes = FramePlanes {
                    ya_width: job.ya_w,
                    ya_height: job.ya_h,
                    ya_pitch: job.ya_pitch,
                    crcb_pitch: job.crcb_pitch,
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
                    &job.video_buf,
                    &job.slices,
                    job.is_key,
                    has_alpha,
                    has_hdr,
                    &mut planes,
                ) {
                    Ok(()) => decoded = true,
                    Err(e) => {
                        log::warn!("bink2w64: frame {} decode failed: {e}", job.cur_frame);
                    }
                }
            }
        }
    }

    if !decoded {
        if job.dst_y_alloc && job.dst_y != 0 && job.ya_bytes > 0 {
            std::ptr::write_bytes(job.dst_y as *mut u8, 0x10, job.ya_bytes);
        }
        if job.dst_cr_alloc && job.dst_cr != 0 && job.crcb_bytes > 0 {
            std::ptr::write_bytes(job.dst_cr as *mut u8, 0x80, job.crcb_bytes);
        }
        if job.dst_cb_alloc && job.dst_cb != 0 && job.crcb_bytes > 0 {
            std::ptr::write_bytes(job.dst_cb as *mut u8, 0x80, job.crcb_bytes);
        }
        if job.dst_a_alloc && job.dst_a != 0 && job.ya_bytes > 0 {
            std::ptr::write_bytes(job.dst_a as *mut u8, 0xFF, job.ya_bytes);
        }
        if job.dst_h_alloc && job.dst_h != 0 && job.ya_bytes > 0 {
            std::ptr::write_bytes(job.dst_h as *mut u8, 0x00, job.ya_bytes);
        }
    }

    AsyncDecodeResult {
        video_us: t_vid.elapsed().as_micros() as u64,
        video_buf: job.video_buf,
        hevc_decoder: job.hevc_decoder,
    }
}

unsafe fn commit_async_decode(
    inst: &mut BinkInstance,
    pending: PendingAsyncDecode,
    result: Option<AsyncDecodeResult>,
) {
    let video_us = if let Some(res) = result {
        inst.async_video_buf = res.video_buf;
        if res.hevc_decoder.is_some() {
            inst.hevc_decoder = res.hevc_decoder;
        }
        res.video_us
    } else {
        0
    };

    if pending.fb_ptr != 0 {
        let fb = pending.fb_ptr as *mut BinkFrameBuffers;
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*fb).frame_num),
            pending.dst_idx as u32,
        );
    }

    inst.total_video_decomp_us = inst.total_video_decomp_us.saturating_add(video_us);
    if inst.frame_timings.len() >= 120 {
        inst.frame_timings.pop_front();
    }
    inst.frame_timings.push_back(FrameTimingSample {
        video_us,
        audio_us: pending.audio_us,
        read_us: pending.read_us,
        blit_us: 0,
        packet_bytes: pending.packet_bytes,
    });

    inst.last_decoded_frame = pending.cur_frame;
    inst.wr_u32(0x010, pending.cur_frame);
    inst.played_frames = inst.played_frames.saturating_add(1);
    if inst.playback_start.is_none() {
        inst.playback_start = Some(Instant::now());
        inst.playback_base_frame = inst.played_frames.saturating_sub(1);
    }
}

/// Wait up to `us` microseconds (`< 0` = infinite, `0` = non-blocking poll) for this handle's
/// in-flight async decode to finish. Commits frame-buffer publication & stats on completion.
/// Returns `1` if complete (or no async decode is in flight), `0` if still in flight.
pub(crate) unsafe fn wait_and_commit_async_decode(inst: &mut BinkInstance, us: i32) -> i32 {
    let Some(ref pending) = inst.pending_async else {
        return 1;
    };
    let (lock, cvar) = &*pending.sync;
    let mut inner = lock.lock().unwrap_or_else(|e| e.into_inner());
    if !inner.done {
        if us == 0 {
            return 0;
        }
        if us < 0 {
            while !inner.done {
                inner = cvar.wait(inner).unwrap_or_else(|e| e.into_inner());
            }
        } else {
            let deadline = Instant::now() + Duration::from_micros(us as u64);
            while !inner.done {
                let Some(rem) = deadline.checked_duration_since(Instant::now()) else {
                    return 0;
                };
                let (next_inner, wait_res) = cvar
                    .wait_timeout(inner, rem)
                    .unwrap_or_else(|e| e.into_inner());
                inner = next_inner;
                if inner.done {
                    break;
                }
                if wait_res.timed_out() {
                    return 0;
                }
            }
        }
    }
    let result = inner.result.take();
    drop(inner);

    let pending = inst.pending_async.take().unwrap();
    commit_async_decode(inst, pending, result);
    1
}

/// Synchronously wait for and commit any in-flight async decode on `inst`.
#[inline]
pub(crate) unsafe fn drain_async_decode(inst: &mut BinkInstance) {
    if inst.pending_async.is_some() {
        let _ = wait_and_commit_async_decode(inst, -1);
    }
}

/// Non-blocking commit if an in-flight async decode on `inst` has already completed.
#[inline]
pub(crate) unsafe fn try_commit_async_decode(inst: &mut BinkInstance) {
    if inst.pending_async.is_some() {
        let _ = wait_and_commit_async_decode(inst, 0);
    }
}

unsafe fn dispatch_frame_async(inst: &mut BinkInstance, thread_ids: &[u32]) -> i32 {
    // If an earlier async decode is still pending on this handle, synchronize it first so
    // back-to-back calls without an intervening wait never race on frame buffers.
    drain_async_decode(inst);

    let cur_frame = inst.rd_u32(0x00c).max(1);
    if inst.last_decoded_frame == cur_frame {
        return 1;
    }

    // Phase 1: Packet I/O runs on the caller's Win32 thread so guest custom I/O callbacks
    // (`user_io`, `OS_READ`, `OS_SEEK`) always execute with valid guest TLS/TEB state.
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

    // Phase 2: Submit lightweight Bink audio chunks on the calling thread.
    let mut audio_us = 0u64;
    if pkt_loaded && inst.file_num_tracks > 0 {
        let t0 = Instant::now();
        let pkt_len = inst.cached_packet_len;
        inst.audio_player.submit_frame_packets(
            &inst.frame_packet_buf[..pkt_len],
            &inst.cached_track_spans,
        );
        audio_us = t0.elapsed().as_micros() as u64;
        inst.total_audio_decomp_us = inst.total_audio_decomp_us.saturating_add(audio_us);
    }

    let fb = inst.rd_u64(0x0b0) as *mut BinkFrameBuffers;
    if !inst.video_on || fb.is_null() {
        if inst.frame_timings.len() >= 120 {
            inst.frame_timings.pop_front();
        }
        inst.frame_timings.push_back(FrameTimingSample {
            video_us: 0,
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
        return 1;
    }

    let fb_val = std::ptr::read_unaligned(fb);
    let total = (fb_val.total_frames.max(1) as u32).min(3);
    let prev_idx = (fb_val.frame_num % total) as usize;
    let dst_idx = ((fb_val.frame_num + 1) % total) as usize;
    let dst_ps = fb_val.frames[dst_idx];
    let ref_ps = fb_val.frames[prev_idx];

    let ya_w = fb_val.ya_buffer_width as usize;
    let ya_h = fb_val.ya_buffer_height as usize;
    let crcb_h = fb_val.crcb_buffer_height as usize;
    let ya_pitch = dst_ps.y_plane.buffer_pitch as usize;
    let crcb_pitch = dst_ps.cr_plane.buffer_pitch as usize;
    let ya_bytes = ya_pitch.saturating_mul(ya_h);
    let crcb_bytes = crcb_pitch.saturating_mul(crcb_h);

    let video_flags = inst.rd_u32(0x024);
    let fourcc = inst.rd_u32(0x118);
    let has_alpha = (video_flags & 0x0010_0000) != 0;
    let has_hdr = (video_flags & 0x0000_0004) != 0;
    let same_buf = dst_idx == prev_idx || dst_ps.y_plane.buffer == ref_ps.y_plane.buffer;

    let mut video_buf = std::mem::take(&mut inst.async_video_buf);
    video_buf.clear();
    let has_video_payload = pkt_loaded && inst.cached_video_offset < inst.cached_packet_len;
    if has_video_payload {
        video_buf.extend_from_slice(
            &inst.frame_packet_buf[inst.cached_video_offset..inst.cached_packet_len],
        );
    }

    let ref_owned = if same_buf
        && has_video_payload
        && is_bink2_magic(fourcc)
        && !ref_ps.y_plane.buffer.is_null()
        && !ref_ps.cr_plane.buffer.is_null()
        && !ref_ps.cb_plane.buffer.is_null()
        && ya_bytes > 0
        && crcb_bytes > 0
    {
        Some(OwnedRefPlanes {
            y: std::slice::from_raw_parts(ref_ps.y_plane.buffer as *const u8, ya_bytes).to_vec(),
            cr: std::slice::from_raw_parts(ref_ps.cr_plane.buffer as *const u8, crcb_bytes)
                .to_vec(),
            cb: std::slice::from_raw_parts(ref_ps.cb_plane.buffer as *const u8, crcb_bytes)
                .to_vec(),
            a: if has_alpha && !ref_ps.a_plane.buffer.is_null() {
                Some(
                    std::slice::from_raw_parts(ref_ps.a_plane.buffer as *const u8, ya_bytes)
                        .to_vec(),
                )
            } else {
                None
            },
            h: if has_hdr && !ref_ps.h_plane.buffer.is_null() {
                Some(
                    std::slice::from_raw_parts(ref_ps.h_plane.buffer as *const u8, ya_bytes)
                        .to_vec(),
                )
            } else {
                None
            },
        })
    } else {
        None
    };

    let hevc_decoder = if is_kb2v_magic(fourcc) {
        inst.hevc_decoder.take()
    } else {
        None
    };

    let job = WorkerPlaneJob {
        cur_frame,
        fourcc,
        video_flags,
        slices: inst.slices,
        is_key: inst.cached_packet_is_key,
        has_video_payload,
        ya_w,
        ya_h,
        ya_pitch,
        crcb_pitch,
        ya_bytes,
        crcb_bytes,
        dst_y: dst_ps.y_plane.buffer as usize,
        dst_cr: dst_ps.cr_plane.buffer as usize,
        dst_cb: dst_ps.cb_plane.buffer as usize,
        dst_a: dst_ps.a_plane.buffer as usize,
        dst_h: dst_ps.h_plane.buffer as usize,
        dst_y_alloc: dst_ps.y_plane.allocate != 0,
        dst_cr_alloc: dst_ps.cr_plane.allocate != 0,
        dst_cb_alloc: dst_ps.cb_plane.allocate != 0,
        dst_a_alloc: dst_ps.a_plane.allocate != 0,
        dst_h_alloc: dst_ps.h_plane.allocate != 0,
        ref_y: ref_ps.y_plane.buffer as usize,
        ref_cr: ref_ps.cr_plane.buffer as usize,
        ref_cb: ref_ps.cb_plane.buffer as usize,
        ref_a: ref_ps.a_plane.buffer as usize,
        ref_h: ref_ps.h_plane.buffer as usize,
        ref_owned,
        video_buf,
        hevc_decoder,
    };

    let guard = AsyncDecodeGuard::acquire(thread_ids);
    let sync = Arc::new((
        Mutex::new(AsyncJobInner {
            done: false,
            result: None,
        }),
        Condvar::new(),
    ));

    inst.pending_async = Some(PendingAsyncDecode {
        sync: sync.clone(),
        cur_frame,
        fb_ptr: fb as usize,
        dst_idx,
        same_buf,
        read_us,
        audio_us,
        packet_bytes,
    });

    spawn_async_job(move || {
        let mut signal = AsyncCompletionSignal {
            sync,
            result: None,
            guard: Some(guard),
        };
        signal.result = Some(execute_worker_plane_job(job));
    });

    1
}

pub(crate) unsafe fn do_frame(bink: *mut c_void) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    drain_async_decode(inst);
    decode_frame_into_buffers(inst, true);
    0
}

pub(crate) unsafe fn do_frame_plane(bink: *mut c_void, which_planes: u32) -> i32 {
    if bink.is_null() || which_planes == 0 {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    drain_async_decode(inst);
    decode_frame_into_buffers(inst, true);
    0
}

pub(crate) unsafe fn do_frame_async(bink: *mut c_void, thread_one: u32, thread_two: u32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    dispatch_frame_async(inst, &[thread_one, thread_two])
}

pub(crate) unsafe fn do_frame_async_multi(
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
    let inst = &mut *(bink as *mut BinkInstance);
    dispatch_frame_async(inst, &tids)
}

/// Test-only helper to deterministically stage an in-flight async worker job that blocks on a
/// caller-controlled gate, enabling race-free tests of wait timeouts, multi-handle independence,
/// `BinkClose`-while-in-flight, and `BinkNextFrame`/`BinkGoto` synchronization.
#[cfg(test)]
pub(crate) unsafe fn start_gated_async_job_for_test(
    bink: *mut c_void,
    thread_ids: &[u32],
    gate: Arc<(Mutex<bool>, Condvar)>,
    worker_finished: Arc<std::sync::atomic::AtomicBool>,
) {
    let inst = &mut *(bink as *mut BinkInstance);
    drain_async_decode(inst);

    let cur_frame = inst.rd_u32(0x00c).max(1);
    let fb = inst.rd_u64(0x0b0) as *mut BinkFrameBuffers;
    let (fb_ptr, dst_idx, same_buf) = if !fb.is_null() {
        let fb_val = std::ptr::read_unaligned(fb);
        let total = (fb_val.total_frames.max(1) as u32).min(3);
        let prev_idx = (fb_val.frame_num % total) as usize;
        let next_idx = ((fb_val.frame_num + 1) % total) as usize;
        (fb as usize, next_idx, next_idx == prev_idx)
    } else {
        (0, 0, false)
    };

    let guard = AsyncDecodeGuard::acquire(thread_ids);
    let sync = Arc::new((
        Mutex::new(AsyncJobInner {
            done: false,
            result: None,
        }),
        Condvar::new(),
    ));
    inst.pending_async = Some(PendingAsyncDecode {
        sync: sync.clone(),
        cur_frame,
        fb_ptr,
        dst_idx,
        same_buf,
        read_us: 0,
        audio_us: 0,
        packet_bytes: 0,
    });

    spawn_async_job(move || {
        let mut signal = AsyncCompletionSignal {
            sync,
            result: None,
            guard: Some(guard),
        };
        let (glock, gcvar) = &*gate;
        let mut released = glock.lock().unwrap_or_else(|e| e.into_inner());
        while !*released {
            released = gcvar.wait(released).unwrap_or_else(|e| e.into_inner());
        }
        drop(released);
        worker_finished.store(true, Ordering::Release);
        signal.result = Some(AsyncDecodeResult {
            video_us: 100,
            video_buf: Vec::new(),
            hevc_decoder: None,
        });
    });
}
