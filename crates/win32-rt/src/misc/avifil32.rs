//! avifil32.dll: pure-Rust Video for Windows RIFF `AVI ` writer and reader
//! producing standard `RIFF`/`AVI ` (`hdrl`/`avih`/`strl`/`strh`/`strf`/`movi`/`idx1`)
//! files through `crate::path::to_host`.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const AVIERR_OK: i32 = 0;
const AVIERR_BADFORMAT: i32 = 0x8004_4066u32 as i32;
const AVIERR_FILEOPEN: i32 = 0x8004_406Cu32 as i32;
const AVIERR_FILEREAD: i32 = 0x8004_406Du32 as i32;
const AVIERR_FILEWRITE: i32 = 0x8004_406Eu32 as i32;
const AVIERR_NOCOMPRESSOR: i32 = 0x8004_4071u32 as i32;
const AVIERR_READONLY: i32 = 0x8004_4073u32 as i32;
const AVIERR_BADHANDLE: i32 = 0x8004_4074u32 as i32;
const AVIERR_BADPARAM: i32 = 0x8004_406Au32 as i32;

const OF_WRITE: u32 = 0x0001;
const OF_READWRITE: u32 = 0x0002;
const OF_CREATE: u32 = 0x1000;

const AVIF_HASINDEX: u32 = 0x0000_0010;
const AVIF_ISINTERLEAVED: u32 = 0x0000_0100;
const AVIIF_KEYFRAME: u32 = 0x0000_0010;
const AVICOMPRESSF_VALID: u32 = 0x0000_0008;

const FCC_VIDS: u32 = u32::from_le_bytes(*b"vids");
const FCC_AUDS: u32 = u32::from_le_bytes(*b"auds");
const FCC_DIB: u32 = u32::from_le_bytes(*b"DIB ");
const FCC_RGB: u32 = u32::from_le_bytes(*b"RGB ");
const FCC_RAW: u32 = u32::from_le_bytes(*b"RAW ");

static AVI_INIT_REFS: AtomicU32 = AtomicU32::new(0);
static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(0x4156_0001);

fn alloc_handle() -> usize {
    NEXT_HANDLE.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug)]
struct AviStreamData {
    fcc_type: u32,
    fcc_handler: u32,
    dw_flags: u32,
    w_priority: u16,
    w_language: u16,
    dw_scale: u32,
    dw_rate: u32,
    dw_start: u32,
    dw_length: u32,
    dw_initial_frames: u32,
    dw_suggested_buffer_size: u32,
    dw_quality: u32,
    dw_sample_size: u32,
    rc_frame: [i16; 4],
    format: Vec<u8>,
    width: u32,
    height: u32,
}

impl Default for AviStreamData {
    fn default() -> Self {
        Self {
            fcc_type: FCC_VIDS,
            fcc_handler: FCC_DIB,
            dw_flags: 0,
            w_priority: 0,
            w_language: 0,
            dw_scale: 1,
            dw_rate: 30,
            dw_start: 0,
            dw_length: 0,
            dw_initial_frames: 0,
            dw_suggested_buffer_size: 0,
            dw_quality: 0,
            dw_sample_size: 0,
            rc_frame: [0; 4],
            format: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug)]
struct AviChunk {
    stream_idx: usize,
    flags: u32,
    data: Vec<u8>,
}

struct AviFileState {
    host_path: PathBuf,
    writable: bool,
    dirty: bool,
    streams: Vec<AviStreamData>,
    chunks: Vec<AviChunk>,
}

impl AviFileState {
    fn flush_to_disk(&mut self) -> Result<(), i32> {
        if !self.writable || !self.dirty {
            return Ok(());
        }
        let bytes = serialize_riff_avi(&self.streams, &self.chunks);
        std::fs::write(&self.host_path, bytes).map_err(|_| AVIERR_FILEWRITE)?;
        self.dirty = false;
        Ok(())
    }
}

impl Drop for AviFileState {
    fn drop(&mut self) {
        let _ = self.flush_to_disk();
    }
}

struct AviFileEntry {
    refs: u32,
    state: Arc<Mutex<AviFileState>>,
}

struct AviStreamEntry {
    refs: u32,
    file: Arc<Mutex<AviFileState>>,
    stream_idx: usize,
}

static AVI_FILES: LazyLock<Mutex<HashMap<usize, AviFileEntry>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static AVI_STREAMS: LazyLock<Mutex<HashMap<usize, AviStreamEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn push_chunk(dst: &mut Vec<u8>, fourcc: [u8; 4], payload: &[u8]) {
    dst.extend_from_slice(&fourcc);
    dst.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    dst.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        dst.push(0);
    }
}

fn push_list(dst: &mut Vec<u8>, list_type: [u8; 4], body: &[u8]) {
    dst.extend_from_slice(b"LIST");
    dst.extend_from_slice(&((4 + body.len()) as u32).to_le_bytes());
    dst.extend_from_slice(&list_type);
    dst.extend_from_slice(body);
    if body.len() % 2 == 1 {
        dst.push(0);
    }
}

fn stream_chunk_fourcc(stream_idx: usize, stream: &AviStreamData) -> [u8; 4] {
    let d0 = b'0' + ((stream_idx / 10) % 10) as u8;
    let d1 = b'0' + (stream_idx % 10) as u8;
    let suffix = if stream.fcc_type == FCC_AUDS {
        *b"wb"
    } else if stream.fcc_type == FCC_VIDS {
        if stream.fcc_handler == 0
            || stream.fcc_handler == FCC_DIB
            || stream.fcc_handler == FCC_RGB
            || stream.fcc_handler == FCC_RAW
        {
            *b"db"
        } else {
            *b"dc"
        }
    } else {
        *b"tx"
    };
    [d0, d1, suffix[0], suffix[1]]
}

/// Serialize streams and sample chunks into a valid RIFF `AVI ` container
/// containing `LIST hdrl` (`avih` + `LIST strl` (`strh` + `strf`)), `LIST movi`,
/// and `idx1`.
fn serialize_riff_avi(streams: &[AviStreamData], chunks: &[AviChunk]) -> Vec<u8> {
    let primary_video = streams.iter().find(|s| s.fcc_type == FCC_VIDS).or_else(|| streams.first());

    let microsec_per_frame = primary_video
        .map(|s| {
            if s.dw_rate > 0 && s.dw_scale > 0 {
                ((s.dw_scale as u64).saturating_mul(1_000_000) / (s.dw_rate as u64)).max(1) as u32
            } else {
                33_333
            }
        })
        .unwrap_or(33_333);

    let total_frames = primary_video.map(|s| s.dw_length).unwrap_or(0);
    let max_chunk_size = chunks
        .iter()
        .map(|c| c.data.len() as u32)
        .max()
        .or_else(|| streams.iter().map(|s| s.dw_suggested_buffer_size).max())
        .unwrap_or(0);

    let (width, height) = primary_video
        .map(|s| {
            let w = if s.width > 0 {
                s.width
            } else {
                (s.rc_frame[2] as i32 - s.rc_frame[0] as i32).max(0) as u32
            };
            let h = if s.height > 0 {
                s.height
            } else {
                (s.rc_frame[3] as i32 - s.rc_frame[1] as i32).max(0) as u32
            };
            (w, h)
        })
        .unwrap_or((0, 0));

    let max_bytes_per_sec = if microsec_per_frame > 0 {
        ((max_chunk_size as u64).saturating_mul(1_000_000) / (microsec_per_frame as u64))
            .min(u32::MAX as u64) as u32
    } else {
        0
    };

    // MainAVIHeader (`avih`, 56 bytes)
    let mut avih = Vec::with_capacity(56);
    avih.extend_from_slice(&microsec_per_frame.to_le_bytes());
    avih.extend_from_slice(&max_bytes_per_sec.to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes()); // dwPaddingGranularity
    avih.extend_from_slice(&(AVIF_HASINDEX | AVIF_ISINTERLEAVED).to_le_bytes());
    avih.extend_from_slice(&total_frames.to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes()); // dwInitialFrames
    avih.extend_from_slice(&(streams.len() as u32).to_le_bytes());
    avih.extend_from_slice(&max_chunk_size.to_le_bytes());
    avih.extend_from_slice(&width.to_le_bytes());
    avih.extend_from_slice(&height.to_le_bytes());
    avih.extend_from_slice(&[0u8; 16]); // dwReserved[4]
    debug_assert_eq!(avih.len(), 56);

    // `hdrl` body
    let mut hdrl_body = Vec::new();
    push_chunk(&mut hdrl_body, *b"avih", &avih);

    for s in streams {
        let mut strl_body = Vec::new();
        // AVIStreamHeader (`strh`, 56 bytes)
        let mut strh = Vec::with_capacity(56);
        strh.extend_from_slice(&s.fcc_type.to_le_bytes());
        strh.extend_from_slice(&s.fcc_handler.to_le_bytes());
        strh.extend_from_slice(&s.dw_flags.to_le_bytes());
        strh.extend_from_slice(&s.w_priority.to_le_bytes());
        strh.extend_from_slice(&s.w_language.to_le_bytes());
        strh.extend_from_slice(&s.dw_initial_frames.to_le_bytes());
        strh.extend_from_slice(&s.dw_scale.to_le_bytes());
        strh.extend_from_slice(&s.dw_rate.to_le_bytes());
        strh.extend_from_slice(&s.dw_start.to_le_bytes());
        strh.extend_from_slice(&s.dw_length.to_le_bytes());
        strh.extend_from_slice(&s.dw_suggested_buffer_size.to_le_bytes());
        strh.extend_from_slice(&s.dw_quality.to_le_bytes());
        strh.extend_from_slice(&s.dw_sample_size.to_le_bytes());
        for coord in s.rc_frame {
            strh.extend_from_slice(&coord.to_le_bytes());
        }
        debug_assert_eq!(strh.len(), 56);
        push_chunk(&mut strl_body, *b"strh", &strh);
        push_chunk(&mut strl_body, *b"strf", &s.format);
        push_list(&mut hdrl_body, *b"strl", &strl_body);
    }

    // `movi` body + `idx1` index
    let mut movi_body = Vec::new();
    let mut idx1_body = Vec::with_capacity(chunks.len() * 16);
    for c in chunks {
        let Some(stream) = streams.get(c.stream_idx) else {
            continue;
        };
        let ckid = stream_chunk_fourcc(c.stream_idx, stream);
        // Offset relative to the start of the "movi" FourCC (4 bytes before movi_body).
        let chunk_offset = (4 + movi_body.len()) as u32;
        push_chunk(&mut movi_body, ckid, &c.data);

        let idx_flags = if (c.flags & AVIIF_KEYFRAME) != 0 {
            AVIIF_KEYFRAME
        } else {
            0
        };
        idx1_body.extend_from_slice(&ckid);
        idx1_body.extend_from_slice(&idx_flags.to_le_bytes());
        idx1_body.extend_from_slice(&chunk_offset.to_le_bytes());
        idx1_body.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
    }

    let mut avi_body = Vec::new();
    push_list(&mut avi_body, *b"hdrl", &hdrl_body);
    push_list(&mut avi_body, *b"movi", &movi_body);
    push_chunk(&mut avi_body, *b"idx1", &idx1_body);

    let mut out = Vec::with_capacity(12 + avi_body.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&((4 + avi_body.len()) as u32).to_le_bytes());
    out.extend_from_slice(b"AVI ");
    out.extend_from_slice(&avi_body);
    out
}

#[derive(Debug)]
struct ParsedAvi {
    streams: Vec<AviStreamData>,
    chunks: Vec<AviChunk>,
    width: u32,
    height: u32,
    total_frames: u32,
    microsec_per_frame: u32,
    idx1_count: usize,
}

fn read_le_u32(data: &[u8], off: usize) -> Option<u32> {
    let slice = data.get(off..off + 4)?;
    Some(u32::from_le_bytes(slice.try_into().ok()?))
}

fn read_le_u16(data: &[u8], off: usize) -> Option<u16> {
    let slice = data.get(off..off + 2)?;
    Some(u16::from_le_bytes(slice.try_into().ok()?))
}

fn read_le_i16(data: &[u8], off: usize) -> Option<i16> {
    let slice = data.get(off..off + 2)?;
    Some(i16::from_le_bytes(slice.try_into().ok()?))
}

fn read_le_i32(data: &[u8], off: usize) -> Option<i32> {
    let slice = data.get(off..off + 4)?;
    Some(i32::from_le_bytes(slice.try_into().ok()?))
}

fn parse_strl_list(body: &[u8]) -> Option<AviStreamData> {
    let mut stream = AviStreamData::default();
    let mut got_strh = false;
    let mut pos = 0usize;
    while pos + 8 <= body.len() {
        let id = &body[pos..pos + 4];
        let sz = read_le_u32(body, pos + 4)? as usize;
        let payload_start = pos + 8;
        let payload_end = payload_start.checked_add(sz)?;
        if payload_end > body.len() {
            return None;
        }
        let payload = &body[payload_start..payload_end];
        if id == b"strh" && payload.len() >= 56 {
            stream.fcc_type = read_le_u32(payload, 0)?;
            stream.fcc_handler = read_le_u32(payload, 4)?;
            stream.dw_flags = read_le_u32(payload, 8)?;
            stream.w_priority = read_le_u16(payload, 12)?;
            stream.w_language = read_le_u16(payload, 14)?;
            stream.dw_initial_frames = read_le_u32(payload, 16)?;
            stream.dw_scale = read_le_u32(payload, 20)?;
            stream.dw_rate = read_le_u32(payload, 24)?;
            stream.dw_start = read_le_u32(payload, 28)?;
            stream.dw_length = read_le_u32(payload, 32)?;
            stream.dw_suggested_buffer_size = read_le_u32(payload, 36)?;
            stream.dw_quality = read_le_u32(payload, 40)?;
            stream.dw_sample_size = read_le_u32(payload, 44)?;
            stream.rc_frame = [
                read_le_i16(payload, 48)?,
                read_le_i16(payload, 50)?,
                read_le_i16(payload, 52)?,
                read_le_i16(payload, 54)?,
            ];
            got_strh = true;
        } else if id == b"strf" {
            stream.format = payload.to_vec();
            if stream.fcc_type == FCC_VIDS && payload.len() >= 12 {
                stream.width = read_le_i32(payload, 4).unwrap_or(0).unsigned_abs();
                stream.height = read_le_i32(payload, 8).unwrap_or(0).unsigned_abs();
            }
        }
        pos = payload_end + (sz & 1);
    }
    got_strh.then_some(stream)
}

fn parse_riff_avi(bytes: &[u8]) -> Result<ParsedAvi, i32> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"AVI " {
        return Err(AVIERR_BADFORMAT);
    }
    let riff_size = read_le_u32(bytes, 4).ok_or(AVIERR_BADFORMAT)? as usize;
    let end = (8usize).saturating_add(riff_size).min(bytes.len());

    let mut streams = Vec::new();
    let mut chunks = Vec::new();
    let mut width = 0u32;
    let mut height = 0u32;
    let mut total_frames = 0u32;
    let mut microsec_per_frame = 0u32;
    let mut idx1_count = 0usize;
    let mut got_avih = false;

    let mut pos = 12usize;
    while pos + 8 <= end {
        let id = &bytes[pos..pos + 4];
        let sz = read_le_u32(bytes, pos + 4).ok_or(AVIERR_BADFORMAT)? as usize;
        let payload_start = pos + 8;
        let payload_end = payload_start.checked_add(sz).ok_or(AVIERR_BADFORMAT)?;
        if payload_end > end {
            return Err(AVIERR_BADFORMAT);
        }
        let payload = &bytes[payload_start..payload_end];

        if id == b"LIST" && payload.len() >= 4 {
            let list_type = &payload[0..4];
            let list_body = &payload[4..];
            if list_type == b"hdrl" {
                let mut hpos = 0usize;
                while hpos + 8 <= list_body.len() {
                    let hid = &list_body[hpos..hpos + 4];
                    let hsz = read_le_u32(list_body, hpos + 4).ok_or(AVIERR_BADFORMAT)? as usize;
                    let hs = hpos + 8;
                    let he = hs.checked_add(hsz).ok_or(AVIERR_BADFORMAT)?;
                    if he > list_body.len() {
                        return Err(AVIERR_BADFORMAT);
                    }
                    let hp = &list_body[hs..he];
                    if hid == b"avih" && hp.len() >= 56 {
                        microsec_per_frame = read_le_u32(hp, 0).unwrap_or(0);
                        total_frames = read_le_u32(hp, 16).unwrap_or(0);
                        width = read_le_u32(hp, 32).unwrap_or(0);
                        height = read_le_u32(hp, 36).unwrap_or(0);
                        got_avih = true;
                    } else if hid == b"LIST" && hp.len() >= 4 && &hp[0..4] == b"strl" {
                        if let Some(s) = parse_strl_list(&hp[4..]) {
                            streams.push(s);
                        }
                    }
                    hpos = he + (hsz & 1);
                }
            } else if list_type == b"movi" {
                let mut mpos = 0usize;
                while mpos + 8 <= list_body.len() {
                    let mid = &list_body[mpos..mpos + 4];
                    let msz = read_le_u32(list_body, mpos + 4).ok_or(AVIERR_BADFORMAT)? as usize;
                    let ms = mpos + 8;
                    let me = ms.checked_add(msz).ok_or(AVIERR_BADFORMAT)?;
                    if me > list_body.len() {
                        return Err(AVIERR_BADFORMAT);
                    }
                    if mid[0].is_ascii_digit() && mid[1].is_ascii_digit() {
                        let sidx = ((mid[0] - b'0') as usize) * 10 + ((mid[1] - b'0') as usize);
                        chunks.push(AviChunk {
                            stream_idx: sidx,
                            flags: 0,
                            data: list_body[ms..me].to_vec(),
                        });
                    }
                    mpos = me + (msz & 1);
                }
            }
        } else if id == b"idx1" {
            idx1_count = payload.len() / 16;
            for (i, entry) in payload.chunks_exact(16).enumerate() {
                if let Some(c) = chunks.get_mut(i) {
                    c.flags = u32::from_le_bytes(entry[4..8].try_into().unwrap());
                }
            }
        }

        pos = payload_end + (sz & 1);
    }

    if !got_avih {
        return Err(AVIERR_BADFORMAT);
    }
    Ok(ParsedAvi {
        streams,
        chunks,
        width,
        height,
        total_frames,
        microsec_per_frame,
        idx1_count,
    })
}

fn open_avi_file(win_path: &str, mode: u32) -> Result<usize, i32> {
    let host_path = crate::path::to_host(win_path).ok_or(AVIERR_FILEOPEN)?;
    let writable = (mode & (OF_CREATE | OF_WRITE | OF_READWRITE)) != 0;
    let create = (mode & OF_CREATE) != 0;

    let (streams, chunks, dirty) = if create {
        if let Some(parent) = host_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|_| AVIERR_FILEOPEN)?;
            }
        }
        std::fs::File::create(&host_path).map_err(|_| AVIERR_FILEOPEN)?;
        (Vec::new(), Vec::new(), true)
    } else if host_path.exists() {
        let bytes = std::fs::read(&host_path).map_err(|_| AVIERR_FILEREAD)?;
        if bytes.is_empty() && writable {
            (Vec::new(), Vec::new(), true)
        } else {
            let parsed = parse_riff_avi(&bytes)?;
            log::debug!(
                "avifil32: parsed {:?} ({}x{}, {} frames, {} us/frame, {} idx1 entries)",
                host_path,
                parsed.width,
                parsed.height,
                parsed.total_frames,
                parsed.microsec_per_frame,
                parsed.idx1_count,
            );
            (parsed.streams, parsed.chunks, false)
        }
    } else if writable {
        if let Some(parent) = host_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|_| AVIERR_FILEOPEN)?;
            }
        }
        std::fs::File::create(&host_path).map_err(|_| AVIERR_FILEOPEN)?;
        (Vec::new(), Vec::new(), true)
    } else {
        return Err(AVIERR_FILEOPEN);
    };

    let state = Arc::new(Mutex::new(AviFileState {
        host_path,
        writable,
        dirty,
        streams,
        chunks,
    }));
    let handle = alloc_handle();
    AVI_FILES
        .lock()
        .unwrap()
        .insert(handle, AviFileEntry { refs: 1, state });
    Ok(handle)
}

unsafe fn parse_stream_info_header(psi: *const u8) -> AviStreamData {
    let fcc_type = std::ptr::read_unaligned(psi.add(0) as *const u32);
    let fcc_handler = std::ptr::read_unaligned(psi.add(4) as *const u32);
    let dw_flags = std::ptr::read_unaligned(psi.add(8) as *const u32);
    let w_priority = std::ptr::read_unaligned(psi.add(16) as *const u16);
    let w_language = std::ptr::read_unaligned(psi.add(18) as *const u16);
    let dw_scale = std::ptr::read_unaligned(psi.add(20) as *const u32);
    let dw_rate = std::ptr::read_unaligned(psi.add(24) as *const u32);
    let dw_start = std::ptr::read_unaligned(psi.add(28) as *const u32);
    let dw_length = std::ptr::read_unaligned(psi.add(32) as *const u32);
    let dw_initial_frames = std::ptr::read_unaligned(psi.add(36) as *const u32);
    let dw_suggested_buffer_size = std::ptr::read_unaligned(psi.add(40) as *const u32);
    let dw_quality = std::ptr::read_unaligned(psi.add(44) as *const u32);
    let dw_sample_size = std::ptr::read_unaligned(psi.add(48) as *const u32);
    let left = std::ptr::read_unaligned(psi.add(52) as *const i32);
    let top = std::ptr::read_unaligned(psi.add(56) as *const i32);
    let right = std::ptr::read_unaligned(psi.add(60) as *const i32);
    let bottom = std::ptr::read_unaligned(psi.add(64) as *const i32);

    AviStreamData {
        fcc_type: if fcc_type == 0 { FCC_VIDS } else { fcc_type },
        fcc_handler,
        dw_flags,
        w_priority,
        w_language,
        dw_scale: if dw_scale == 0 { 1 } else { dw_scale },
        dw_rate: if dw_rate == 0 { 30 } else { dw_rate },
        dw_start,
        dw_length,
        dw_initial_frames,
        dw_suggested_buffer_size,
        dw_quality,
        dw_sample_size,
        rc_frame: [
            left.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            top.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            right.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            bottom.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
        ],
        format: Vec::new(),
        width: (right - left).max(0) as u32,
        height: (bottom - top).max(0) as u32,
    }
}

unsafe fn create_stream_common(pfile: usize, ppavi: *mut usize, psi: *const u8) -> i32 {
    if ppavi.is_null() || psi.is_null() {
        return AVIERR_BADPARAM;
    }
    std::ptr::write_unaligned(ppavi, 0);
    let file_state = {
        let files = AVI_FILES.lock().unwrap();
        let Some(entry) = files.get(&pfile) else {
            return AVIERR_BADHANDLE;
        };
        Arc::clone(&entry.state)
    };

    let info = parse_stream_info_header(psi);
    let stream_idx = {
        let mut st = file_state.lock().unwrap();
        if !st.writable {
            return AVIERR_READONLY;
        }
        let idx = st.streams.len();
        st.streams.push(info);
        st.dirty = true;
        idx
    };

    let shandle = alloc_handle();
    AVI_STREAMS.lock().unwrap().insert(
        shandle,
        AviStreamEntry {
            refs: 1,
            file: file_state,
            stream_idx,
        },
    );
    std::ptr::write_unaligned(ppavi, shandle);
    AVIERR_OK
}

pub unsafe extern "win64" fn AVIFileInit() {
    AVI_INIT_REFS.fetch_add(1, Ordering::Relaxed);
}

pub unsafe extern "win64" fn AVIFileExit() {
    let _ = AVI_INIT_REFS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
        Some(v.saturating_sub(1))
    });
}

pub unsafe extern "win64" fn AVIFileOpenA(
    ppfile: *mut usize,
    sz_file: LPCSTR,
    mode: u32,
    _pclsid_handler: *const u8,
) -> i32 {
    if ppfile.is_null() || sz_file.is_null() {
        return AVIERR_BADPARAM;
    }
    std::ptr::write_unaligned(ppfile, 0);
    let win_path = crate::strings::from_ansi(sz_file);
    match open_avi_file(&win_path, mode) {
        Ok(h) => {
            std::ptr::write_unaligned(ppfile, h);
            AVIERR_OK
        }
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn AVIFileOpenW(
    ppfile: *mut usize,
    sz_file: LPCWSTR,
    mode: u32,
    _pclsid_handler: *const u8,
) -> i32 {
    if ppfile.is_null() || sz_file.is_null() {
        return AVIERR_BADPARAM;
    }
    std::ptr::write_unaligned(ppfile, 0);
    let win_path = crate::strings::from_wide(sz_file);
    match open_avi_file(&win_path, mode) {
        Ok(h) => {
            std::ptr::write_unaligned(ppfile, h);
            AVIERR_OK
        }
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn AVIFileCreateStreamA(
    pfile: usize,
    ppavi: *mut usize,
    psi: *const u8,
) -> i32 {
    create_stream_common(pfile, ppavi, psi)
}

pub unsafe extern "win64" fn AVIFileCreateStreamW(
    pfile: usize,
    ppavi: *mut usize,
    psi: *const u8,
) -> i32 {
    create_stream_common(pfile, ppavi, psi)
}

pub unsafe extern "win64" fn AVIMakeCompressedStream(
    pps_compressed: *mut usize,
    ps_source: usize,
    lp_options: *const u8,
    _pclsid_handler: *const u8,
) -> i32 {
    if pps_compressed.is_null() {
        return AVIERR_BADPARAM;
    }
    std::ptr::write_unaligned(pps_compressed, 0);

    let (file, stream_idx) = {
        let streams = AVI_STREAMS.lock().unwrap();
        let Some(entry) = streams.get(&ps_source) else {
            return AVIERR_BADHANDLE;
        };
        (Arc::clone(&entry.file), entry.stream_idx)
    };

    let handler = if lp_options.is_null() {
        FCC_DIB
    } else {
        std::ptr::read_unaligned(lp_options.add(4) as *const u32)
    };

    // Uncompressed RGB/DIB passthrough is supported directly; external binary codecs are unavailable.
    let uncompressed = handler == 0 || handler == FCC_DIB || handler == FCC_RGB || handler == FCC_RAW;
    if !uncompressed {
        return AVIERR_NOCOMPRESSOR;
    }

    {
        let mut st = file.lock().unwrap();
        if let Some(s) = st.streams.get_mut(stream_idx) {
            if s.fcc_type == FCC_VIDS {
                s.fcc_handler = if handler == 0 { FCC_DIB } else { handler };
            }
        }
    }

    let out_handle = alloc_handle();
    AVI_STREAMS.lock().unwrap().insert(
        out_handle,
        AviStreamEntry {
            refs: 1,
            file,
            stream_idx,
        },
    );
    std::ptr::write_unaligned(pps_compressed, out_handle);
    AVIERR_OK
}

pub unsafe extern "win64" fn AVIStreamSetFormat(
    pavi: usize,
    _lpos: i32,
    lp_format: *const u8,
    cb_format: i32,
) -> i32 {
    if lp_format.is_null() || cb_format <= 0 {
        return AVIERR_BADPARAM;
    }
    let (file, stream_idx) = {
        let streams = AVI_STREAMS.lock().unwrap();
        let Some(entry) = streams.get(&pavi) else {
            return AVIERR_BADHANDLE;
        };
        (Arc::clone(&entry.file), entry.stream_idx)
    };

    let fmt = std::slice::from_raw_parts(lp_format, cb_format as usize).to_vec();
    let mut st = file.lock().unwrap();
    if !st.writable {
        return AVIERR_READONLY;
    }
    let Some(stream) = st.streams.get_mut(stream_idx) else {
        return AVIERR_BADHANDLE;
    };
    if stream.fcc_type == FCC_VIDS && fmt.len() >= 12 {
        let w = read_le_i32(&fmt, 4).unwrap_or(0).unsigned_abs();
        let h = read_le_i32(&fmt, 8).unwrap_or(0).unsigned_abs();
        if w > 0 {
            stream.width = w;
            stream.rc_frame[2] = w.min(i16::MAX as u32) as i16;
        }
        if h > 0 {
            stream.height = h;
            stream.rc_frame[3] = h.min(i16::MAX as u32) as i16;
        }
    }
    stream.format = fmt;
    st.dirty = true;
    AVIERR_OK
}

pub unsafe extern "win64" fn AVIStreamWrite(
    pavi: usize,
    _lstart: i32,
    lsamples: i32,
    lp_buffer: *const u8,
    cb_buffer: i32,
    dw_flags: u32,
    pl_samp_written: *mut i32,
    pl_bytes_written: *mut i32,
) -> i32 {
    if !pl_samp_written.is_null() {
        std::ptr::write_unaligned(pl_samp_written, 0);
    }
    if !pl_bytes_written.is_null() {
        std::ptr::write_unaligned(pl_bytes_written, 0);
    }
    if lsamples < 0 || cb_buffer < 0 || (cb_buffer > 0 && lp_buffer.is_null()) {
        return AVIERR_BADPARAM;
    }
    let (file, stream_idx) = {
        let streams = AVI_STREAMS.lock().unwrap();
        let Some(entry) = streams.get(&pavi) else {
            return AVIERR_BADHANDLE;
        };
        (Arc::clone(&entry.file), entry.stream_idx)
    };

    let data = if cb_buffer > 0 {
        std::slice::from_raw_parts(lp_buffer, cb_buffer as usize).to_vec()
    } else {
        Vec::new()
    };

    let mut st = file.lock().unwrap();
    if !st.writable {
        return AVIERR_READONLY;
    }
    let Some(stream) = st.streams.get_mut(stream_idx) else {
        return AVIERR_BADHANDLE;
    };
    let n_samples = if lsamples == 0 && !data.is_empty() { 1 } else { lsamples as u32 };
    stream.dw_length = stream.dw_length.saturating_add(n_samples);
    if (cb_buffer as u32) > stream.dw_suggested_buffer_size {
        stream.dw_suggested_buffer_size = cb_buffer as u32;
    }
    st.chunks.push(AviChunk {
        stream_idx,
        flags: dw_flags,
        data,
    });
    st.dirty = true;

    if !pl_samp_written.is_null() {
        std::ptr::write_unaligned(pl_samp_written, n_samples as i32);
    }
    if !pl_bytes_written.is_null() {
        std::ptr::write_unaligned(pl_bytes_written, cb_buffer);
    }
    AVIERR_OK
}

pub unsafe extern "win64" fn AVIFileRelease(pfile: usize) -> u32 {
    let removed = {
        let mut files = AVI_FILES.lock().unwrap();
        let Some(entry) = files.get_mut(&pfile) else {
            return 0;
        };
        entry.refs = entry.refs.saturating_sub(1);
        if entry.refs == 0 {
            files.remove(&pfile)
        } else {
            return entry.refs;
        }
    };
    if let Some(entry) = removed {
        let _ = entry.state.lock().unwrap().flush_to_disk();
    }
    0
}

pub unsafe extern "win64" fn AVIStreamRelease(pavi: usize) -> u32 {
    let removed = {
        let mut streams = AVI_STREAMS.lock().unwrap();
        let Some(entry) = streams.get_mut(&pavi) else {
            return 0;
        };
        entry.refs = entry.refs.saturating_sub(1);
        if entry.refs == 0 {
            streams.remove(&pavi)
        } else {
            return entry.refs;
        }
    };
    if let Some(entry) = removed {
        let _ = entry.file.lock().unwrap().flush_to_disk();
    }
    0
}

pub unsafe extern "win64" fn AVISaveOptions(
    _hwnd: HWND,
    _ui_flags: u32,
    n_streams: i32,
    ppavi: *const usize,
    plp_options: *const *mut u8,
) -> BOOL {
    if n_streams <= 0 || plp_options.is_null() {
        return FALSE;
    }
    for i in 0..n_streams as usize {
        let opts = std::ptr::read_unaligned(plp_options.add(i));
        if opts.is_null() {
            continue;
        }
        let mut fcc_type = FCC_VIDS;
        if !ppavi.is_null() {
            let s = std::ptr::read_unaligned(ppavi.add(i));
            let streams = AVI_STREAMS.lock().unwrap();
            if let Some(entry) = streams.get(&s) {
                let st = entry.file.lock().unwrap();
                if let Some(sd) = st.streams.get(entry.stream_idx) {
                    fcc_type = sd.fcc_type;
                }
            }
        }
        std::ptr::write_unaligned(opts.add(0) as *mut u32, fcc_type);
        std::ptr::write_unaligned(
            opts.add(4) as *mut u32,
            if fcc_type == FCC_VIDS { FCC_DIB } else { 0 },
        );
        let flags = std::ptr::read_unaligned(opts.add(20) as *const u32);
        std::ptr::write_unaligned(opts.add(20) as *mut u32, flags | AVICOMPRESSF_VALID);
    }
    TRUE
}

pub unsafe extern "win64" fn AVISaveOptionsFree(n_streams: i32, plp_options: *const *mut u8) -> i32 {
    if n_streams <= 0 || plp_options.is_null() {
        return AVIERR_OK;
    }
    for i in 0..n_streams as usize {
        let opts = std::ptr::read_unaligned(plp_options.add(i));
        if opts.is_null() {
            continue;
        }
        let lp_format = std::ptr::read_unaligned(opts.add(24) as *const *mut c_void);
        if !lp_format.is_null() && crate::heap::is_ours(lp_format) {
            crate::heap::free(lp_format);
            std::ptr::write_unaligned(opts.add(24) as *mut usize, 0);
            std::ptr::write_unaligned(opts.add(32) as *mut u32, 0);
        }
        let lp_parms = std::ptr::read_unaligned(opts.add(40) as *const *mut c_void);
        if !lp_parms.is_null() && crate::heap::is_ours(lp_parms) {
            crate::heap::free(lp_parms);
            std::ptr::write_unaligned(opts.add(40) as *mut usize, 0);
            std::ptr::write_unaligned(opts.add(48) as *mut u32, 0);
        }
    }
    AVIERR_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_bitmap_info_header(width: i32, height: i32, bpp: u16) -> [u8; 40] {
        let mut hdr = [0u8; 40];
        let img_size = ((width.unsigned_abs() * (bpp as u32 / 8) + 3) & !3) * height.unsigned_abs();
        hdr[0..4].copy_from_slice(&40u32.to_le_bytes()); // biSize
        hdr[4..8].copy_from_slice(&width.to_le_bytes()); // biWidth
        hdr[8..12].copy_from_slice(&height.to_le_bytes()); // biHeight
        hdr[12..14].copy_from_slice(&1u16.to_le_bytes()); // biPlanes
        hdr[14..16].copy_from_slice(&bpp.to_le_bytes()); // biBitCount
        hdr[16..20].copy_from_slice(&0u32.to_le_bytes()); // BI_RGB
        hdr[20..24].copy_from_slice(&img_size.to_le_bytes());
        hdr
    }

    #[test]
    fn riff_avi_write_and_read_validation() {
        let root = std::env::temp_dir().join(format!("win32_avifil32_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        crate::path::add_mount("V:\\AviTest", &root);

        unsafe {
            AVIFileInit();

            let mut pfile: usize = 0;
            let path_w = crate::strings::to_wide_z("V:\\AviTest\\capture.avi");
            assert_eq!(
                AVIFileOpenW(&mut pfile, path_w.as_ptr(), OF_CREATE | OF_WRITE, std::ptr::null()),
                AVIERR_OK
            );
            assert_ne!(pfile, 0);

            // Create a 4x4 24-bit RGB video stream at 60 fps
            let mut stream_info = [0u8; 204];
            stream_info[0..4].copy_from_slice(b"vids");
            stream_info[4..8].copy_from_slice(b"DIB ");
            stream_info[20..24].copy_from_slice(&1u32.to_le_bytes()); // dwScale
            stream_info[24..28].copy_from_slice(&60u32.to_le_bytes()); // dwRate
            stream_info[60..64].copy_from_slice(&4i32.to_le_bytes()); // rcFrame.right
            stream_info[64..68].copy_from_slice(&4i32.to_le_bytes()); // rcFrame.bottom

            let mut pstream: usize = 0;
            assert_eq!(AVIFileCreateStreamW(pfile, &mut pstream, stream_info.as_ptr()), AVIERR_OK);
            assert_ne!(pstream, 0);

            // Wrap with AVIMakeCompressedStream for uncompressed DIB
            let mut opts = [0u8; 56];
            opts[0..4].copy_from_slice(b"vids");
            opts[4..8].copy_from_slice(b"DIB ");
            let mut pcomp: usize = 0;
            assert_eq!(
                AVIMakeCompressedStream(&mut pcomp, pstream, opts.as_ptr(), std::ptr::null()),
                AVIERR_OK
            );

            // Unsupported codec returns AVIERR_NOCOMPRESSOR
            let mut bad_opts = [0u8; 56];
            bad_opts[4..8].copy_from_slice(b"XVID");
            let mut bad_comp: usize = 0;
            assert_eq!(
                AVIMakeCompressedStream(&mut bad_comp, pstream, bad_opts.as_ptr(), std::ptr::null()),
                AVIERR_NOCOMPRESSOR
            );

            // Set BITMAPINFOHEADER format
            let bih = make_bitmap_info_header(4, 4, 24);
            assert_eq!(AVIStreamSetFormat(pcomp, 0, bih.as_ptr(), bih.len() as i32), AVIERR_OK);

            // Write 2 RGB frames (4x4x3 = 48 bytes) plus an odd-length 47-byte chunk to test RIFF word padding
            let frame0 = [0x11u8; 48];
            let frame1 = [0x22u8; 47];
            let (mut sw, mut bw) = (0i32, 0i32);
            assert_eq!(
                AVIStreamWrite(
                    pcomp,
                    0,
                    1,
                    frame0.as_ptr(),
                    frame0.len() as i32,
                    AVIIF_KEYFRAME,
                    &mut sw,
                    &mut bw
                ),
                AVIERR_OK
            );
            assert_eq!((sw, bw), (1, 48));
            assert_eq!(
                AVIStreamWrite(
                    pcomp,
                    1,
                    1,
                    frame1.as_ptr(),
                    frame1.len() as i32,
                    AVIIF_KEYFRAME,
                    &mut sw,
                    &mut bw
                ),
                AVIERR_OK
            );

            assert_eq!(AVIStreamRelease(pcomp), 0);
            assert_eq!(AVIStreamRelease(pstream), 0);
            assert_eq!(AVIFileRelease(pfile), 0);

            // Validate raw RIFF AVI binary structure on disk
            let raw = std::fs::read(root.join("capture.avi")).unwrap();
            assert_eq!(&raw[0..4], b"RIFF");
            assert_eq!(&raw[8..12], b"AVI ");
            assert_eq!(read_le_u32(&raw, 4).unwrap() as usize + 8, raw.len());
            assert!(raw.windows(4).any(|w| w == b"hdrl"));
            assert!(raw.windows(4).any(|w| w == b"avih"));
            assert!(raw.windows(4).any(|w| w == b"strl"));
            assert!(raw.windows(4).any(|w| w == b"strh"));
            assert!(raw.windows(4).any(|w| w == b"strf"));
            assert!(raw.windows(4).any(|w| w == b"movi"));
            assert!(raw.windows(4).any(|w| w == b"idx1"));

            // Parse back through our RIFF reader and verify every header & frame
            let parsed = parse_riff_avi(&raw).unwrap();
            assert_eq!(parsed.width, 4);
            assert_eq!(parsed.height, 4);
            assert_eq!(parsed.total_frames, 2);
            assert_eq!(parsed.microsec_per_frame, 16_666);
            assert_eq!(parsed.streams.len(), 1);
            assert_eq!(parsed.streams[0].format, bih);
            assert_eq!(parsed.chunks.len(), 2);
            assert_eq!(parsed.idx1_count, 2);
            assert_eq!(parsed.chunks[0].data, frame0);
            assert_eq!(parsed.chunks[1].data, frame1);
            assert_eq!(parsed.chunks[0].flags, AVIIF_KEYFRAME);

            // Re-open in read-only mode via AVIFileOpenA
            let mut rfile: usize = 0;
            assert_eq!(
                AVIFileOpenA(&mut rfile, b"V:\\AviTest\\capture.avi\0".as_ptr(), 0, std::ptr::null()),
                AVIERR_OK
            );
            assert_ne!(rfile, 0);
            assert_eq!(AVIFileRelease(rfile), 0);

            AVIFileExit();
        }

        let _ = std::fs::remove_dir_all(&root);
    }
}
