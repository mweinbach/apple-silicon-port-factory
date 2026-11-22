//! Windows Spatial Audio (`<spatialaudioclient.h>`) on CoreAudio:
//! `ISpatialAudioClient`, `IAudioFormatEnumerator`,
//! `ISpatialAudioObjectRenderStream` (inheriting
//! `ISpatialAudioObjectRenderStreamBase`), and `ISpatialAudioObject`
//! (inheriting `ISpatialAudioObjectBase`).
//!
//! Supports both static 7.1.4/stereo bed channels (`FrontLeft`, `FrontRight`,
//! `FrontCenter`, `LowFrequency`, `SideLeft`, `SideRight`, `BackLeft`,
//! `BackRight`, `Top*`, `Bottom*`, `BackCenter`, `StereoLeft`, `StereoRight`)
//! and up to 128 concurrent dynamic 3D objects (`AudioObjectType_Dynamic`).
//!
//! Dynamic objects are spatialized from Windows right-handed 3D coordinates
//! (`+X` right, `+Y` up, `-Z` front, meters relative to the listener origin)
//! into stereo `f32` via constant-power azimuth panning, subtle interaural time
//! difference (ITD, fractional delay line up to ~0.65 ms), inverse-distance
//! attenuation beyond 1 m, and subtle rear attenuation, then pushed into a live
//! [`super::backend::OutputStream`] (auto-falling back to [`Backend::Null`]
//! when headless or `AW2_AUDIO_BACKEND=null`) while signaling the caller's
//! Win32 `EventHandle` on each 480-frame (10 ms @ 48 kHz) audio quantum.

use super::backend::{self, Backend, OutputStream, StreamControl, StreamShared};
use super::com::*;
use super::format::{SampleType, WaveFormat, WAVEFORMATEX, WAVE_FORMAT_IEEE_FLOAT};
use super::mmdevapi::Endpoint;
use super::ring::Ring;
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, OnceLock, Weak};

// ---------------------------------------------------------------------------
// Spatial Audio IIDs (<spatialaudioclient.h>)
// ---------------------------------------------------------------------------

pub const IID_IAudioFormatEnumerator: GUID =
    GUID::new(0xDCDAA858, 0x895A, 0x4A22, [0xA5, 0xEB, 0x67, 0xBD, 0xA5, 0x06, 0x09, 0x6D]);
pub const IID_ISpatialAudioObjectBase: GUID =
    GUID::new(0xCCE0B8F2, 0x8D4D, 0x4EFB, [0xA8, 0xCF, 0x3D, 0x6E, 0xCF, 0x1C, 0x30, 0xE0]);
pub const IID_ISpatialAudioObject: GUID =
    GUID::new(0xDDE28967, 0x521B, 0x46E5, [0x8F, 0x00, 0xBD, 0x6F, 0x2B, 0xC8, 0xAB, 0x1D]);
pub const IID_ISpatialAudioObjectRenderStreamBase: GUID =
    GUID::new(0xFEAAF403, 0xC1D8, 0x450D, [0xAA, 0x05, 0xE0, 0xCC, 0xEE, 0x75, 0x02, 0xA8]);
pub const IID_ISpatialAudioObjectRenderStream: GUID =
    GUID::new(0xBAB5F473, 0xB423, 0x477B, [0x85, 0xF5, 0xB5, 0xA3, 0x32, 0xA0, 0x41, 0x53]);
pub const IID_ISpatialAudioObjectRenderStreamNotify: GUID =
    GUID::new(0xDDDF83E6, 0x68D7, 0x4C70, [0x88, 0x3F, 0xA1, 0x83, 0x6A, 0xFB, 0x4A, 0x50]);

// ---------------------------------------------------------------------------
// AudioObjectType bitflags (<spatialaudioclient.h>)
// ---------------------------------------------------------------------------

pub const AudioObjectType_None: u32 = 0;
pub const AudioObjectType_Dynamic: u32 = 1 << 0;
pub const AudioObjectType_FrontLeft: u32 = 1 << 1;
pub const AudioObjectType_FrontRight: u32 = 1 << 2;
pub const AudioObjectType_FrontCenter: u32 = 1 << 3;
pub const AudioObjectType_LowFrequency: u32 = 1 << 4;
pub const AudioObjectType_SideLeft: u32 = 1 << 5;
pub const AudioObjectType_SideRight: u32 = 1 << 6;
pub const AudioObjectType_BackLeft: u32 = 1 << 7;
pub const AudioObjectType_BackRight: u32 = 1 << 8;
pub const AudioObjectType_TopFrontLeft: u32 = 1 << 9;
pub const AudioObjectType_TopFrontRight: u32 = 1 << 10;
pub const AudioObjectType_TopBackLeft: u32 = 1 << 11;
pub const AudioObjectType_TopBackRight: u32 = 1 << 12;
pub const AudioObjectType_BottomFrontLeft: u32 = 1 << 13;
pub const AudioObjectType_BottomFrontRight: u32 = 1 << 14;
pub const AudioObjectType_BottomBackLeft: u32 = 1 << 15;
pub const AudioObjectType_BottomBackRight: u32 = 1 << 16;
pub const AudioObjectType_BackCenter: u32 = 1 << 17;
pub const AudioObjectType_StereoLeft: u32 = 1 << 18;
pub const AudioObjectType_StereoRight: u32 = 1 << 19;

/// Native 7.1.4 static speaker bed reported by `GetNativeStaticObjectTypeMask`.
pub const NATIVE_STATIC_MASK: u32 = AudioObjectType_FrontLeft
    | AudioObjectType_FrontRight
    | AudioObjectType_FrontCenter
    | AudioObjectType_LowFrequency
    | AudioObjectType_SideLeft
    | AudioObjectType_SideRight
    | AudioObjectType_BackLeft
    | AudioObjectType_BackRight
    | AudioObjectType_TopFrontLeft
    | AudioObjectType_TopFrontRight
    | AudioObjectType_TopBackLeft
    | AudioObjectType_TopBackRight;

/// All valid static `AudioObjectType` bits (bits 1..=19).
pub const ALL_STATIC_MASK: u32 = NATIVE_STATIC_MASK
    | AudioObjectType_BottomFrontLeft
    | AudioObjectType_BottomFrontRight
    | AudioObjectType_BottomBackLeft
    | AudioObjectType_BottomBackRight
    | AudioObjectType_BackCenter
    | AudioObjectType_StereoLeft
    | AudioObjectType_StereoRight;

// ---------------------------------------------------------------------------
// Spatial Audio HRESULTs (<spatialaudioclient.h>, AUDCLNT_ERR(0x0100 + n))
// ---------------------------------------------------------------------------

pub const SPTLAUDCLNT_E_DESTROYED: HRESULT = 0x8889_0100u32 as i32;
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: HRESULT = 0x8889_0101u32 as i32;
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: HRESULT = 0x8889_0102u32 as i32;
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: HRESULT = 0x8889_0103u32 as i32;
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: HRESULT = 0x8889_0104u32 as i32;
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: HRESULT = 0x8889_0107u32 as i32;
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: HRESULT = 0x8889_010Au32 as i32;
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: HRESULT = 0x8889_010Bu32 as i32;
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: HRESULT = 0x8889_010Cu32 as i32;
pub const SPTLAUDCLNT_E_INTERNAL: HRESULT = 0x8889_010Du32 as i32;

// ---------------------------------------------------------------------------
// Stream parameters & constants
// ---------------------------------------------------------------------------

pub const SPATIAL_SAMPLE_RATE: u32 = 48_000;
/// 10 ms audio quantum at 48 kHz (standard Windows Spatial Audio pass size).
pub const QUANTUM_FRAMES: u32 = 480;
/// Maximum concurrent dynamic 3D audio objects supported per stream.
pub const MAX_DYNAMIC_OBJECTS: u32 = 128;

const VT_BLOB: u16 = 65;
const ITD_RING_LEN: usize = 64;
const ITD_RING_MASK: usize = ITD_RING_LEN - 1;
/// Woodworth max interaural time difference (~0.65 ms @ 48 kHz).
const MAX_ITD_SAMPLES: f32 = 31.0;
/// Per-object gain smoothing ramp length (samples) when moving across quanta.
const GAIN_RAMP_SAMPLES: usize = 64;

/// Packed `<spatialaudioclient.h>` activation payload (`#pragma pack(push, 1)`).
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub object_format: *const WAVEFORMATEX,
    pub static_object_type_mask: u32,
    pub min_dynamic_object_count: u32,
    pub max_dynamic_object_count: u32,
    pub category: u32,
    pub event_handle: HANDLE,
    pub notify_object: *mut c_void,
}

const _: () = assert!(std::mem::size_of::<SpatialAudioObjectRenderStreamActivationParams>() == 40);

/// Fixed-size COM vtable registered with `VA_DISPATCH` on creation.
#[repr(C)]
pub struct ComVtbl<const N: usize>(pub [usize; N]);

fn register_vtbl<const N: usize>(slots: [usize; N]) -> ComVtbl<N> {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    ComVtbl(slots)
}

/// 48 kHz mono 32-bit IEEE float `WAVEFORMATEX`.
pub fn preferred_object_format() -> WAVEFORMATEX {
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_IEEE_FLOAT,
        nChannels: 1,
        nSamplesPerSec: SPATIAL_SAMPLE_RATE,
        nAvgBytesPerSec: SPATIAL_SAMPLE_RATE * 4,
        nBlockAlign: 4,
        wBitsPerSample: 32,
        cbSize: 0,
    }
}

unsafe fn is_supported_object_format(p: *const WAVEFORMATEX) -> Result<WaveFormat, HRESULT> {
    if p.is_null() {
        return Err(E_POINTER);
    }
    let wf = match WaveFormat::from_ptr(p) {
        Ok(f) => f,
        Err(_) => return Err(AUDCLNT_E_UNSUPPORTED_FORMAT),
    };
    if wf.rate == SPATIAL_SAMPLE_RATE && wf.channels == 1 && wf.sample == SampleType::F32 {
        Ok(wf)
    } else {
        Err(AUDCLNT_E_UNSUPPORTED_FORMAT)
    }
}

// ---------------------------------------------------------------------------
// 3D Geometry, Static Beds & Stereo Spatializer DSP
// ---------------------------------------------------------------------------

/// Right-handed 3D speaker position in meters (`+X` right, `+Y` up, `-Z` front)
/// for a single static `AudioObjectType` bit.
pub fn static_object_position(obj_type: u32) -> Option<[f32; 3]> {
    const SIN30: f32 = 0.5;
    const COS30: f32 = 0.866_025_4;
    const D45: f32 = std::f32::consts::FRAC_1_SQRT_2;
    match obj_type {
        AudioObjectType_FrontLeft => Some([-SIN30, 0.0, -COS30]),
        AudioObjectType_FrontRight => Some([SIN30, 0.0, -COS30]),
        AudioObjectType_FrontCenter => Some([0.0, 0.0, -1.0]),
        AudioObjectType_LowFrequency => Some([0.0, -0.2, -1.0]),
        AudioObjectType_SideLeft | AudioObjectType_StereoLeft => Some([-1.0, 0.0, 0.0]),
        AudioObjectType_SideRight | AudioObjectType_StereoRight => Some([1.0, 0.0, 0.0]),
        AudioObjectType_BackLeft => Some([-D45, 0.0, D45]),
        AudioObjectType_BackRight => Some([D45, 0.0, D45]),
        AudioObjectType_TopFrontLeft => Some([-0.5, D45, -0.5]),
        AudioObjectType_TopFrontRight => Some([0.5, D45, -0.5]),
        AudioObjectType_TopBackLeft => Some([-0.5, D45, 0.5]),
        AudioObjectType_TopBackRight => Some([0.5, D45, 0.5]),
        AudioObjectType_BottomFrontLeft => Some([-0.5, -D45, -0.5]),
        AudioObjectType_BottomFrontRight => Some([0.5, -D45, -0.5]),
        AudioObjectType_BottomBackLeft => Some([-0.5, -D45, 0.5]),
        AudioObjectType_BottomBackRight => Some([0.5, -D45, 0.5]),
        AudioObjectType_BackCenter => Some([0.0, 0.0, 1.0]),
        _ => None,
    }
}

/// Stereo `(L, R)` downmix gains for a single static bed channel.
fn static_bed_gains(obj_type: u32) -> (f32, f32) {
    const M3DB: f32 = std::f32::consts::FRAC_1_SQRT_2;
    match obj_type {
        AudioObjectType_FrontLeft | AudioObjectType_StereoLeft => (1.0, 0.0),
        AudioObjectType_FrontRight | AudioObjectType_StereoRight => (0.0, 1.0),
        AudioObjectType_FrontCenter | AudioObjectType_LowFrequency => (M3DB, M3DB),
        AudioObjectType_SideLeft
        | AudioObjectType_BackLeft
        | AudioObjectType_TopFrontLeft
        | AudioObjectType_BottomFrontLeft => (M3DB, 0.0),
        AudioObjectType_SideRight
        | AudioObjectType_BackRight
        | AudioObjectType_TopFrontRight
        | AudioObjectType_BottomFrontRight => (0.0, M3DB),
        AudioObjectType_TopBackLeft | AudioObjectType_BottomBackLeft => (0.5, 0.0),
        AudioObjectType_TopBackRight | AudioObjectType_BottomBackRight => (0.0, 0.5),
        AudioObjectType_BackCenter => (0.5, 0.5),
        _ => (0.0, 0.0),
    }
}

/// Compute `(gain_l, gain_r, delay_l_samples, delay_r_samples)` for one spatial
/// audio object at right-handed 3D `position` (`+X` right, `+Y` up, `-Z` front)
/// and linear `volume` in `[0.0, 1.0]`.
pub fn spatial_gains_and_itd(object_type: u32, position: [f32; 3], volume: f32) -> (f32, f32, f32, f32) {
    let vol = if volume.is_finite() { volume.clamp(0.0, 1.0) } else { 0.0 };
    if object_type != AudioObjectType_Dynamic {
        let (l, r) = static_bed_gains(object_type);
        return (l * vol, r * vol, 0.0, 0.0);
    }
    let [x, y, z] = position;
    let dist_sq = x * x + y * y + z * z;
    if !dist_sq.is_finite() || dist_sq < 1e-12 {
        let c = std::f32::consts::FRAC_1_SQRT_2 * vol;
        return (c, c, 0.0, 0.0);
    }
    let dist = dist_sq.sqrt();
    // Lateral projection onto X axis handles elevation smoothly (overhead -> pan 0).
    let pan = (x / dist).clamp(-1.0, 1.0);
    // Constant-power azimuth pan: L = cos((pan + 1) * pi/4), R = sin((pan + 1) * pi/4).
    let theta = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
    let pan_l = theta.cos();
    let pan_r = theta.sin();
    // Unity inside reference radius (1.0 m), inverse-distance rolloff beyond 1.0 m.
    let atten = 1.0 / dist.max(1.0);
    // Subtle front/back cue (+Z is behind listener; max ~-1.1 dB directly behind).
    let rear = 1.0 - 0.12 * (z / dist).max(0.0);
    let scale = vol * atten * rear;
    let (delay_l, delay_r) = if pan >= 0.0 {
        // Source on right: left ear delayed.
        (pan * MAX_ITD_SAMPLES, 0.0)
    } else {
        // Source on left: right ear delayed.
        (0.0, (-pan) * MAX_ITD_SAMPLES)
    };
    (pan_l * scale, pan_r * scale, delay_l, delay_r)
}

#[inline]
fn read_delay(ring: &[f32; ITD_RING_LEN], write_pos: usize, delay: f32) -> f32 {
    if delay <= 0.0 {
        return ring[write_pos];
    }
    let d = delay.clamp(0.0, (ITD_RING_LEN - 2) as f32);
    let d0 = d.floor() as usize;
    let frac = d - d0 as f32;
    let idx0 = write_pos.wrapping_sub(d0) & ITD_RING_MASK;
    let idx1 = write_pos.wrapping_sub(d0 + 1) & ITD_RING_MASK;
    ring[idx0] * (1.0 - frac) + ring[idx1] * frac
}

/// One active (or end-of-stream draining) audio object slot owned by a stream.
struct ObjectSlot {
    generation: u64,
    object_type: u32,
    position: [f32; 3],
    volume: f32,
    active: bool,
    written_this_pass: bool,
    eos_frames: Option<u32>,
    released_pending: bool,
    /// Stable heap allocation handed out via `ISpatialAudioObjectBase::GetBuffer`.
    pcm: Box<[f32]>,
    delay_line: [f32; ITD_RING_LEN],
    delay_pos: usize,
    prev_gain_l: f32,
    prev_gain_r: f32,
    has_prev_gains: bool,
}

impl ObjectSlot {
    fn new(generation: u64, object_type: u32, frames: usize) -> Self {
        Self {
            generation,
            object_type,
            position: [0.0, 0.0, 0.0],
            volume: 1.0,
            active: true,
            written_this_pass: false,
            eos_frames: None,
            released_pending: false,
            pcm: vec![0.0f32; frames].into_boxed_slice(),
            delay_line: [0.0f32; ITD_RING_LEN],
            delay_pos: 0,
            prev_gain_l: 0.0,
            prev_gain_r: 0.0,
            has_prev_gains: false,
        }
    }

    fn mix_into(&mut self, frames: usize, out_stereo: &mut [f32]) {
        let (target_l, target_r, delay_l, delay_r) =
            spatial_gains_and_itd(self.object_type, self.position, self.volume);
        let (start_l, start_r) = if self.has_prev_gains {
            (self.prev_gain_l, self.prev_gain_r)
        } else {
            (target_l, target_r)
        };
        let ramp = if self.has_prev_gains { GAIN_RAMP_SAMPLES.min(frames) } else { 0 };
        let inv_ramp = if ramp > 0 { 1.0 / ramp as f32 } else { 0.0 };
        let valid_frames = self
            .eos_frames
            .map(|e| (e as usize).min(frames))
            .unwrap_or(frames)
            .min(self.pcm.len());
        let has_itd = delay_l > 0.0 || delay_r > 0.0;

        for i in 0..frames {
            let raw = if self.written_this_pass && i < valid_frames {
                self.pcm[i]
            } else {
                0.0
            };
            let sample = if raw.is_finite() { raw } else { 0.0 };
            let (sl, sr) = if has_itd {
                self.delay_line[self.delay_pos] = sample;
                let l = read_delay(&self.delay_line, self.delay_pos, delay_l);
                let r = read_delay(&self.delay_line, self.delay_pos, delay_r);
                self.delay_pos = (self.delay_pos + 1) & ITD_RING_MASK;
                (l, r)
            } else {
                // Keep delay line zeroed if object later moves off-center.
                self.delay_line[self.delay_pos] = sample;
                self.delay_pos = (self.delay_pos + 1) & ITD_RING_MASK;
                (sample, sample)
            };
            let (gl, gr) = if i < ramp {
                let t = (i + 1) as f32 * inv_ramp;
                (start_l + (target_l - start_l) * t, start_r + (target_r - start_r) * t)
            } else {
                (target_l, target_r)
            };
            out_stereo[i * 2] += sl * gl;
            out_stereo[i * 2 + 1] += sr * gr;
        }
        self.prev_gain_l = target_l;
        self.prev_gain_r = target_r;
        self.has_prev_gains = true;
    }
}

/// Source description for deterministic offline [`mix_quantum`] rendering.
#[derive(Clone, Debug)]
pub struct SpatialSource {
    pub object_type: u32,
    pub position: [f32; 3],
    pub volume: f32,
    pub samples: Vec<f32>,
}

/// Deterministically mix a set of static and/or dynamic spatial sources for one
/// quantum of `frames` frames into interleaved stereo `f32` (`[L0, R0, ...]`).
/// Uses the exact same DSP path as `ISpatialAudioObjectRenderStream::EndUpdatingAudioObjects`.
pub fn mix_quantum(sources: &[SpatialSource], frames: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; frames * 2];
    for src in sources {
        let mut slot = ObjectSlot::new(1, src.object_type, frames);
        slot.position = src.position;
        slot.volume = src.volume;
        slot.written_this_pass = true;
        let n = src.samples.len().min(frames);
        slot.pcm[..n].copy_from_slice(&src.samples[..n]);
        slot.mix_into(frames, &mut out);
    }
    for s in &mut out {
        *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
    }
    out
}

// ---------------------------------------------------------------------------
// Stream Core & Object Lifecycle
// ---------------------------------------------------------------------------

struct StreamState {
    static_mask: u32,
    max_dynamic: u32,
    quantum_frames: u32,
    started: bool,
    in_update: bool,
    next_gen: u64,
    slots: Vec<Option<ObjectSlot>>,
    mix_buf: Vec<f32>,
    quanta_mixed: u64,
    last_mixed: Vec<f32>,
}

impl StreamState {
    fn new(static_mask: u32, max_dynamic: u32, quantum_frames: u32) -> Self {
        let cap = (static_mask.count_ones() + max_dynamic) as usize;
        let mut slots = Vec::with_capacity(cap);
        slots.resize_with(cap, || None);
        Self {
            static_mask,
            max_dynamic,
            quantum_frames,
            started: false,
            in_update: false,
            next_gen: 1,
            slots,
            mix_buf: vec![0.0f32; quantum_frames as usize * 2],
            quanta_mixed: 0,
            last_mixed: Vec::new(),
        }
    }

    fn active_dynamic_count(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .filter(|o| o.active && o.object_type == AudioObjectType_Dynamic)
            .count() as u32
    }

    fn available_dynamic_count(&self) -> u32 {
        self.max_dynamic.saturating_sub(self.active_dynamic_count())
    }

    fn static_is_active(&self, obj_type: u32) -> bool {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .any(|o| o.active && o.object_type == obj_type)
    }

    fn alloc_slot(&mut self, obj_type: u32) -> Option<(usize, u64)> {
        let generation = self.next_gen;
        self.next_gen += 1;
        let slot_obj = ObjectSlot::new(generation, obj_type, self.quantum_frames as usize);
        if let Some((idx, cell)) = self.slots.iter_mut().enumerate().find(|(_, s)| s.is_none()) {
            *cell = Some(slot_obj);
            return Some((idx, generation));
        }
        let idx = self.slots.len();
        self.slots.push(Some(slot_obj));
        Some((idx, generation))
    }

    fn slot_mut(&mut self, idx: usize, generation: u64) -> Option<&mut ObjectSlot> {
        self.slots
            .get_mut(idx)
            .and_then(|o| o.as_mut())
            .filter(|o| o.generation == generation)
    }

    fn slot_ref(&self, idx: usize, generation: u64) -> Option<&ObjectSlot> {
        self.slots
            .get(idx)
            .and_then(|o| o.as_ref())
            .filter(|o| o.generation == generation)
    }

    fn reset_objects(&mut self) {
        for cell in &mut self.slots {
            *cell = None;
        }
    }
}

pub struct SpatialStreamCore {
    ep: Arc<Endpoint>,
    shared: Arc<StreamShared>,
    /// Live CoreAudio/Null output stream. Lives outside `st` mutex so dropping
    /// it never deadlocks with audio pump/clock/CoreAudio control threads.
    _out: OutputStream,
    ctrl: StreamControl,
    out_io_frames: u32,
    notify_object: usize,
    st: Mutex<StreamState>,
}

/// Invoke `IUnknown::AddRef` (slot 1) or `Release` (slot 2) on a guest COM object.
unsafe fn guest_unknown_call(obj: *mut c_void, slot: usize) -> u32 {
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

impl Drop for SpatialStreamCore {
    fn drop(&mut self) {
        self.ctrl.stop();
        if self.notify_object != 0 {
            unsafe {
                guest_unknown_call(self.notify_object as *mut c_void, 2);
            }
        }
        let quanta = self.st.get_mut().map(|s| s.quanta_mixed).unwrap_or(0);
        log::info!(
            target: "audio",
            "ISpatialAudioObjectRenderStream released on '{}': {} quanta mixed, {} callbacks",
            self.ep.info.name,
            quanta,
            self.shared.callbacks.load(Ordering::Relaxed)
        );
    }
}

impl SpatialStreamCore {
    fn is_invalid(&self) -> bool {
        self.shared.invalid.load(Ordering::Acquire) || !super::devnotify::endpoint_present(&self.ep.id)
    }

    /// Snapshot of the most recently mixed stereo quantum (`[L0, R0, ...]`).
    pub fn last_mixed_quantum(&self) -> Vec<f32> {
        self.st.lock().unwrap().last_mixed.clone()
    }

    /// Underlying `StreamShared` diagnostics (callbacks, underruns, ring len).
    pub fn shared(&self) -> &Arc<StreamShared> {
        &self.shared
    }
}

// ---------------------------------------------------------------------------
// IAudioFormatEnumerator (5 vtable slots: 0..=4)
// ---------------------------------------------------------------------------

pub type FormatEnumObj = ComObj<ComVtbl<5>, ()>;

mod format_enum {
    use super::*;
    crate::com_iunknown!(FormatEnumObj, [IID_IAudioFormatEnumerator]);

    pub unsafe extern "win64" fn get_count(_this: *mut FormatEnumObj, count: *mut u32) -> HRESULT {
        write_out(count, 1)
    }

    pub unsafe extern "win64" fn get_format(
        _this: *mut FormatEnumObj,
        index: u32,
        format: *mut *mut WAVEFORMATEX,
    ) -> HRESULT {
        if format.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(format, std::ptr::null_mut());
        if index != 0 {
            return E_INVALIDARG;
        }
        let wf = preferred_object_format();
        let sz = std::mem::size_of::<WAVEFORMATEX>();
        let p = co_alloc(sz) as *mut WAVEFORMATEX;
        if p.is_null() {
            return E_OUTOFMEMORY;
        }
        std::ptr::write_unaligned(p, wf);
        std::ptr::write_unaligned(format, p);
        S_OK
    }

    pub fn vtbl() -> &'static ComVtbl<5> {
        static V: OnceLock<ComVtbl<5>> = OnceLock::new();
        V.get_or_init(|| {
            register_vtbl([
                qi as *const () as usize,
                add_ref as *const () as usize,
                release as *const () as usize,
                get_count as *const () as usize,
                get_format as *const () as usize,
            ])
        })
    }
}

// ---------------------------------------------------------------------------
// ISpatialAudioObject (9 vtable slots: 0..=8)
// ---------------------------------------------------------------------------

pub struct SpatialObjInner {
    stream: Arc<SpatialStreamCore>,
    slot_idx: usize,
    generation: u64,
    object_type: u32,
}

impl Drop for SpatialObjInner {
    fn drop(&mut self) {
        let mut st = self.stream.st.lock().unwrap();
        let in_update = st.in_update;
        let should_keep_until_end = match st.slot_mut(self.slot_idx, self.generation) {
            Some(o) if in_update && (o.written_this_pass || o.eos_frames.is_some()) => {
                // Caller called SetEndOfStream + Release before EndUpdatingAudioObjects:
                // keep the buffer alive until EndUpdatingAudioObjects mixes it, then free.
                o.released_pending = true;
                true
            }
            Some(_) => false,
            None => return,
        };
        if !should_keep_until_end {
            if let Some(cell) = st.slots.get_mut(self.slot_idx) {
                if cell.as_ref().is_some_and(|o| o.generation == self.generation) {
                    *cell = None;
                }
            }
        }
    }
}

pub type SpatialObj = ComObj<ComVtbl<9>, SpatialObjInner>;

mod spatial_object {
    use super::*;
    crate::com_iunknown!(SpatialObj, [IID_ISpatialAudioObjectBase, IID_ISpatialAudioObject]);

    pub unsafe extern "win64" fn get_buffer(
        this: *mut SpatialObj,
        buffer: *mut *mut u8,
        buffer_length: *mut u32,
    ) -> HRESULT {
        if buffer.is_null() || buffer_length.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(buffer, std::ptr::null_mut());
        std::ptr::write_unaligned(buffer_length, 0);

        let inner = SpatialObj::inner(this);
        if inner.stream.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let mut st = inner.stream.st.lock().unwrap();
        if !st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        let bytes = st.quantum_frames * 4;
        let Some(slot) = st.slot_mut(inner.slot_idx, inner.generation) else {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        };
        if !slot.active {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        if !slot.written_this_pass {
            slot.pcm.fill(0.0);
            slot.written_this_pass = true;
        }
        std::ptr::write_unaligned(buffer, slot.pcm.as_mut_ptr() as *mut u8);
        std::ptr::write_unaligned(buffer_length, bytes);
        S_OK
    }

    pub unsafe extern "win64" fn set_end_of_stream(this: *mut SpatialObj, frame_count: u32) -> HRESULT {
        let inner = SpatialObj::inner(this);
        if inner.stream.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let mut st = inner.stream.st.lock().unwrap();
        if !st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        if frame_count > st.quantum_frames {
            return E_INVALIDARG;
        }
        let Some(slot) = st.slot_mut(inner.slot_idx, inner.generation) else {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        };
        if !slot.active {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        slot.eos_frames = Some(frame_count);
        S_OK
    }

    pub unsafe extern "win64" fn is_active(this: *mut SpatialObj, out: *mut BOOL) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let inner = SpatialObj::inner(this);
        if inner.stream.is_invalid() {
            return write_out(out, FALSE);
        }
        let st = inner.stream.st.lock().unwrap();
        let active = st
            .slot_ref(inner.slot_idx, inner.generation)
            .is_some_and(|o| o.active);
        write_out(out, if active { TRUE } else { FALSE })
    }

    pub unsafe extern "win64" fn get_audio_object_type(this: *mut SpatialObj, out: *mut u32) -> HRESULT {
        let inner = SpatialObj::inner(this);
        write_out(out, inner.object_type)
    }

    pub unsafe extern "win64" fn set_position(this: *mut SpatialObj, x: f32, y: f32, z: f32) -> HRESULT {
        if this.is_null() {
            return E_POINTER;
        }
        let inner = SpatialObj::inner(this);
        if inner.object_type != AudioObjectType_Dynamic {
            return SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED;
        }
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return E_INVALIDARG;
        }
        if inner.stream.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let mut st = inner.stream.st.lock().unwrap();
        if !st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        let Some(slot) = st.slot_mut(inner.slot_idx, inner.generation) else {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        };
        if !slot.active {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        slot.position = [x, y, z];
        S_OK
    }
    crate::win64_bridge!(fn set_position(*mut SpatialObj, f32, f32, f32) -> HRESULT);

    pub unsafe extern "win64" fn set_volume(this: *mut SpatialObj, volume: f32) -> HRESULT {
        if this.is_null() {
            return E_POINTER;
        }
        if !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
            return E_INVALIDARG;
        }
        let inner = SpatialObj::inner(this);
        if inner.stream.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let mut st = inner.stream.st.lock().unwrap();
        if !st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        let Some(slot) = st.slot_mut(inner.slot_idx, inner.generation) else {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        };
        if !slot.active {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        slot.volume = volume;
        S_OK
    }
    crate::win64_bridge!(fn set_volume(*mut SpatialObj, f32) -> HRESULT);

    pub fn vtbl() -> &'static ComVtbl<9> {
        static V: OnceLock<ComVtbl<9>> = OnceLock::new();
        V.get_or_init(|| {
            register_vtbl([
                qi as *const () as usize,                         // 0: IUnknown::QueryInterface
                add_ref as *const () as usize,                    // 1: IUnknown::AddRef
                release as *const () as usize,                    // 2: IUnknown::Release
                get_buffer as *const () as usize,                 // 3: ISpatialAudioObjectBase::GetBuffer
                set_end_of_stream as *const () as usize,          // 4: ISpatialAudioObjectBase::SetEndOfStream
                is_active as *const () as usize,                  // 5: ISpatialAudioObjectBase::IsActive
                get_audio_object_type as *const () as usize,      // 6: ISpatialAudioObjectBase::GetAudioObjectType
                crate::bridge_entry!(set_position) as usize,      // 7: ISpatialAudioObject::SetPosition
                crate::bridge_entry!(set_volume) as usize,        // 8: ISpatialAudioObject::SetVolume
            ])
        })
    }
}

// ---------------------------------------------------------------------------
// ISpatialAudioObjectRenderStream (11 vtable slots: 0..=10)
//   0..=2  IUnknown
//   3..=9  ISpatialAudioObjectRenderStreamBase
//   10     ISpatialAudioObjectRenderStream::ActivateSpatialAudioObject
// ---------------------------------------------------------------------------

pub type SpatialStreamObj = ComObj<ComVtbl<11>, Arc<SpatialStreamCore>>;

mod render_stream {
    use super::*;
    crate::com_iunknown!(
        SpatialStreamObj,
        [IID_ISpatialAudioObjectRenderStreamBase, IID_ISpatialAudioObjectRenderStream]
    );

    pub unsafe extern "win64" fn get_available_dynamic_object_count(
        this: *mut SpatialStreamObj,
        value: *mut u32,
    ) -> HRESULT {
        if value.is_null() {
            return E_POINTER;
        }
        let core = SpatialStreamObj::inner(this);
        if core.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let avail = core.st.lock().unwrap().available_dynamic_count();
        write_out(value, avail)
    }

    pub unsafe extern "win64" fn get_service(
        this: *mut SpatialStreamObj,
        riid: *const GUID,
        service: *mut *mut c_void,
    ) -> HRESULT {
        if service.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(service, std::ptr::null_mut());
        let Some(iid) = GUID::read(riid) else {
            return E_POINTER;
        };
        if iid == IID_IUnknown
            || iid == IID_ISpatialAudioObjectRenderStreamBase
            || iid == IID_ISpatialAudioObjectRenderStream
        {
            SpatialStreamObj::add_ref(this);
            std::ptr::write_unaligned(service, this as *mut c_void);
            return S_OK;
        }
        stub!("ISpatialAudioObjectRenderStream::GetService({iid:?}) -> E_NOINTERFACE");
        E_NOINTERFACE
    }

    pub unsafe extern "win64" fn start(this: *mut SpatialStreamObj) -> HRESULT {
        let core = SpatialStreamObj::inner(this);
        if core.is_invalid() {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        let ctrl = {
            let mut st = core.st.lock().unwrap();
            if st.started {
                return SPTLAUDCLNT_E_STREAM_NOT_STOPPED;
            }
            st.started = true;
            core.ctrl.clone()
        };
        // Never hold `core.st` across `AudioOutputUnitStart` or `Event::set`.
        if let Err(e) = ctrl.start() {
            log::error!(target: "audio", "ISpatialAudioObjectRenderStream::Start: {e}");
            core.st.lock().unwrap().started = false;
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        // Signal the caller's EventHandle immediately so the first quantum is requested.
        core.shared.signal_event();
        log::debug!(target: "audio", "ISpatialAudioObjectRenderStream::Start on '{}'", core.ep.info.name);
        S_OK
    }

    pub unsafe extern "win64" fn stop(this: *mut SpatialStreamObj) -> HRESULT {
        let core = SpatialStreamObj::inner(this);
        let (was, ctrl) = {
            let mut st = core.st.lock().unwrap();
            let was = st.started;
            st.started = false;
            (was, if was { Some(core.ctrl.clone()) } else { None })
        };
        if let Some(ctrl) = ctrl {
            ctrl.stop();
        }
        if core.is_invalid() {
            AUDCLNT_E_DEVICE_INVALIDATED
        } else if was {
            S_OK
        } else {
            S_FALSE
        }
    }

    pub unsafe extern "win64" fn reset(this: *mut SpatialStreamObj) -> HRESULT {
        let core = SpatialStreamObj::inner(this);
        if core.is_invalid() {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        let mut st = core.st.lock().unwrap();
        if st.started {
            return SPTLAUDCLNT_E_STREAM_NOT_STOPPED;
        }
        if st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        st.reset_objects();
        core.shared.ring.clear();
        S_OK
    }

    pub unsafe extern "win64" fn begin_updating_audio_objects(
        this: *mut SpatialStreamObj,
        available_dynamic_object_count: *mut u32,
        frame_count_per_buffer: *mut u32,
    ) -> HRESULT {
        if available_dynamic_object_count.is_null() || frame_count_per_buffer.is_null() {
            return E_POINTER;
        }
        let core = SpatialStreamObj::inner(this);
        if core.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let mut st = core.st.lock().unwrap();
        if st.in_update {
            return SPTLAUDCLNT_E_OUT_OF_ORDER;
        }
        st.in_update = true;
        for slot in st.slots.iter_mut().flatten() {
            slot.written_this_pass = false;
            slot.eos_frames = None;
        }
        let avail = st.available_dynamic_count();
        let frames = st.quantum_frames;
        std::ptr::write_unaligned(available_dynamic_object_count, avail);
        std::ptr::write_unaligned(frame_count_per_buffer, frames);
        S_OK
    }

    pub unsafe extern "win64" fn end_updating_audio_objects(this: *mut SpatialStreamObj) -> HRESULT {
        let core = SpatialStreamObj::inner(this);
        if core.is_invalid() {
            core.st.lock().unwrap().in_update = false;
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let need_catchup_signal = {
            let mut st = core.st.lock().unwrap();
            if !st.in_update {
                return SPTLAUDCLNT_E_OUT_OF_ORDER;
            }
            let frames = st.quantum_frames as usize;
            let StreamState {
                slots,
                mix_buf,
                last_mixed,
                quanta_mixed,
                in_update,
                started,
                ..
            } = &mut *st;
            mix_buf.resize(frames * 2, 0.0);
            mix_buf.fill(0.0);

            for cell in slots.iter_mut() {
                let mut clear_slot = false;
                if let Some(slot) = cell.as_mut() {
                    if slot.active {
                        slot.mix_into(frames, mix_buf);
                        if slot.eos_frames.is_some() || slot.released_pending {
                            slot.active = false;
                            clear_slot = true;
                        }
                    } else if slot.released_pending {
                        clear_slot = true;
                    }
                }
                if clear_slot {
                    *cell = None;
                }
            }

            for s in mix_buf.iter_mut() {
                *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
            }
            core.shared.ring.push(mix_buf);
            last_mixed.clear();
            last_mixed.extend_from_slice(mix_buf);
            *quanta_mixed += 1;
            *in_update = false;
            // When CoreAudio's granted IO slice is larger than one 480-frame quantum
            // (e.g. 512 on macOS or 1024 on iOS RemoteIO), immediately request a
            // catch-up quantum if the ring still holds less than one IO slice.
            *started
                && core.out_io_frames > QUANTUM_FRAMES
                && (core.shared.ring.len() as u32) < core.out_io_frames
        };
        if need_catchup_signal {
            core.shared.signal_event();
        }
        S_OK
    }

    pub unsafe extern "win64" fn activate_spatial_audio_object(
        this: *mut SpatialStreamObj,
        obj_type: u32,
        audio_object: *mut *mut c_void,
    ) -> HRESULT {
        if audio_object.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(audio_object, std::ptr::null_mut());
        if obj_type == AudioObjectType_None || obj_type.count_ones() != 1 {
            return E_INVALIDARG;
        }
        let core = SpatialStreamObj::inner(this).clone();
        if core.is_invalid() {
            return SPTLAUDCLNT_E_RESOURCES_INVALIDATED;
        }
        let (slot_idx, generation) = {
            let mut st = core.st.lock().unwrap();
            if !st.in_update {
                return SPTLAUDCLNT_E_OUT_OF_ORDER;
            }
            if obj_type == AudioObjectType_Dynamic {
                if st.available_dynamic_count() == 0 {
                    return SPTLAUDCLNT_E_NO_MORE_OBJECTS;
                }
            } else {
                if obj_type & ALL_STATIC_MASK == 0 {
                    return E_INVALIDARG;
                }
                if obj_type & st.static_mask == 0 {
                    return SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE;
                }
                if st.static_is_active(obj_type) {
                    return SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE;
                }
            }
            match st.alloc_slot(obj_type) {
                Some(pair) => pair,
                None => return SPTLAUDCLNT_E_NO_MORE_OBJECTS,
            }
        };
        let obj = SpatialObj::create(
            spatial_object::vtbl(),
            SpatialObjInner {
                stream: core,
                slot_idx,
                generation,
                object_type: obj_type,
            },
        );
        std::ptr::write_unaligned(audio_object, obj as *mut c_void);
        S_OK
    }

    pub fn vtbl() -> &'static ComVtbl<11> {
        static V: OnceLock<ComVtbl<11>> = OnceLock::new();
        V.get_or_init(|| {
            register_vtbl([
                qi as *const () as usize,                                 // 0: IUnknown::QueryInterface
                add_ref as *const () as usize,                            // 1: IUnknown::AddRef
                release as *const () as usize,                            // 2: IUnknown::Release
                get_available_dynamic_object_count as *const () as usize, // 3: Base::GetAvailableDynamicObjectCount
                get_service as *const () as usize,                        // 4: Base::GetService
                start as *const () as usize,                              // 5: Base::Start
                stop as *const () as usize,                               // 6: Base::Stop
                reset as *const () as usize,                              // 7: Base::Reset
                begin_updating_audio_objects as *const () as usize,       // 8: Base::BeginUpdatingAudioObjects
                end_updating_audio_objects as *const () as usize,         // 9: Base::EndUpdatingAudioObjects
                activate_spatial_audio_object as *const () as usize,      // 10: RenderStream::ActivateSpatialAudioObject
            ])
        })
    }
}

// ---------------------------------------------------------------------------
// ISpatialAudioClient (11 vtable slots: 0..=10)
// ---------------------------------------------------------------------------

pub struct SpatialClientInner {
    ep: Arc<Endpoint>,
    active_stream: Mutex<Weak<SpatialStreamCore>>,
}

pub type SpatialClientObj = ComObj<ComVtbl<11>, SpatialClientInner>;

mod client {
    use super::*;
    crate::com_iunknown!(SpatialClientObj, [IID_ISpatialAudioClient]);

    pub unsafe extern "win64" fn get_static_object_position(
        _this: *mut SpatialClientObj,
        obj_type: u32,
        x: *mut f32,
        y: *mut f32,
        z: *mut f32,
    ) -> HRESULT {
        if x.is_null() || y.is_null() || z.is_null() {
            return E_POINTER;
        }
        let Some([px, py, pz]) = static_object_position(obj_type) else {
            return E_INVALIDARG;
        };
        std::ptr::write_unaligned(x, px);
        std::ptr::write_unaligned(y, py);
        std::ptr::write_unaligned(z, pz);
        S_OK
    }

    pub unsafe extern "win64" fn get_native_static_object_type_mask(
        _this: *mut SpatialClientObj,
        mask: *mut u32,
    ) -> HRESULT {
        write_out(mask, NATIVE_STATIC_MASK)
    }

    pub unsafe extern "win64" fn get_max_dynamic_object_count(
        _this: *mut SpatialClientObj,
        value: *mut u32,
    ) -> HRESULT {
        write_out(value, MAX_DYNAMIC_OBJECTS)
    }

    pub unsafe extern "win64" fn get_supported_audio_object_format_enumerator(
        _this: *mut SpatialClientObj,
        enumerator: *mut *mut c_void,
    ) -> HRESULT {
        if enumerator.is_null() {
            return E_POINTER;
        }
        let obj = FormatEnumObj::create(format_enum::vtbl(), ());
        std::ptr::write_unaligned(enumerator, obj as *mut c_void);
        S_OK
    }

    pub unsafe extern "win64" fn get_max_frame_count(
        _this: *mut SpatialClientObj,
        object_format: *const WAVEFORMATEX,
        frame_count_per_buffer: *mut u32,
    ) -> HRESULT {
        if frame_count_per_buffer.is_null() {
            return E_POINTER;
        }
        if let Err(hr) = is_supported_object_format(object_format) {
            return hr;
        }
        write_out(frame_count_per_buffer, QUANTUM_FRAMES)
    }

    pub unsafe extern "win64" fn is_audio_object_format_supported(
        _this: *mut SpatialClientObj,
        object_format: *const WAVEFORMATEX,
    ) -> HRESULT {
        match is_supported_object_format(object_format) {
            Ok(_) => S_OK,
            Err(hr) => hr,
        }
    }

    pub unsafe extern "win64" fn is_spatial_audio_stream_available(
        this: *mut SpatialClientObj,
        stream_uuid: *const GUID,
        _auxiliary_info: *const u8,
    ) -> HRESULT {
        let Some(iid) = GUID::read(stream_uuid) else {
            return E_POINTER;
        };
        let ep = &SpatialClientObj::inner(this).ep;
        if ep.is_capture() {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        }
        if !super::super::devnotify::endpoint_present(&ep.id) {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        if iid == IID_ISpatialAudioObjectRenderStream || iid == IID_ISpatialAudioObjectRenderStreamBase {
            S_OK
        } else {
            SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE
        }
    }

    pub unsafe extern "win64" fn activate_spatial_audio_stream(
        this: *mut SpatialClientObj,
        activation_params: *const u8,
        riid: *const GUID,
        stream_out: *mut *mut c_void,
    ) -> HRESULT {
        if stream_out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(stream_out, std::ptr::null_mut());
        let Some(iid) = GUID::read(riid) else {
            return E_POINTER;
        };
        if iid != IID_ISpatialAudioObjectRenderStream && iid != IID_ISpatialAudioObjectRenderStreamBase {
            return E_NOINTERFACE;
        }
        if activation_params.is_null() {
            return E_POINTER;
        }

        let inner = SpatialClientObj::inner(this);
        if inner.ep.is_capture() {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        }
        if !super::super::devnotify::endpoint_present(&inner.ep.id) {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        {
            let active = inner.active_stream.lock().unwrap();
            if active.strong_count() > 0 {
                return AUDCLNT_E_ALREADY_INITIALIZED;
            }
        }

        // Parse PROPVARIANT (24 bytes): vt @ +0, blob.cbSize @ +8, blob.pBlobData @ +16.
        let vt = std::ptr::read_unaligned(activation_params as *const u16);
        if vt != VT_BLOB {
            return E_INVALIDARG;
        }
        let cb_size = std::ptr::read_unaligned(activation_params.add(8) as *const u32);
        let blob_ptr = std::ptr::read_unaligned(activation_params.add(16) as *const *const u8);
        if (cb_size as usize) < std::mem::size_of::<SpatialAudioObjectRenderStreamActivationParams>()
            || blob_ptr.is_null()
        {
            return E_INVALIDARG;
        }

        let params = std::ptr::read_unaligned(
            blob_ptr as *const SpatialAudioObjectRenderStreamActivationParams,
        );
        let obj_fmt_ptr = params.object_format;
        let static_mask = params.static_object_type_mask;
        let min_dyn = params.min_dynamic_object_count;
        let max_dyn = params.max_dynamic_object_count;
        let category = params.category;
        let event_handle = params.event_handle;
        let notify_object = params.notify_object;

        if obj_fmt_ptr.is_null() {
            return E_INVALIDARG;
        }
        if let Err(hr) = is_supported_object_format(obj_fmt_ptr) {
            return hr;
        }
        if static_mask & !ALL_STATIC_MASK != 0 {
            return E_INVALIDARG;
        }
        if min_dyn > max_dyn {
            return E_INVALIDARG;
        }
        if min_dyn > MAX_DYNAMIC_OBJECTS {
            return SPTLAUDCLNT_E_NO_MORE_OBJECTS;
        }
        if static_mask == 0 && max_dyn == 0 {
            return E_INVALIDARG;
        }
        if event_handle.is_null() {
            return E_INVALIDARG;
        }
        let Some(event) = crate::handles::get_as::<crate::handles::Event>(event_handle) else {
            return E_INVALIDARG;
        };

        let granted_dynamic = max_dyn.min(MAX_DYNAMIC_OBJECTS);
        let ring_cap = (2 * backend::MAX_SLICE + QUANTUM_FRAMES * 4) as usize;
        let shared = StreamShared::new(Ring::new(ring_cap, 2));
        shared.set_event(event);

        // Open CoreAudio output stream, auto-falling back to Backend::Null when
        // headless, AW2_AUDIO_BACKEND=null, or hardware open fails.
        let headless = std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
            || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1");
        let use_null = headless || backend::backend() == Backend::Null || inner.ep.info.id == 0;
        let out = if use_null {
            match OutputStream::open(
                &backend::null_device(),
                SPATIAL_SAMPLE_RATE,
                2,
                shared.clone(),
                QUANTUM_FRAMES,
            ) {
                Ok(o) => o,
                Err(e) => {
                    log::error!(target: "audio", "spatial audio null output failed: {e}");
                    return AUDCLNT_E_ENDPOINT_CREATE_FAILED;
                }
            }
        } else {
            match OutputStream::open(
                &inner.ep.info,
                SPATIAL_SAMPLE_RATE,
                2,
                shared.clone(),
                QUANTUM_FRAMES,
            ) {
                Ok(o) => o,
                Err(e) => {
                    log::error!(
                        target: "audio",
                        "spatial audio device '{}' failed to open: {e}; falling back to Backend::Null",
                        inner.ep.info.name
                    );
                    match OutputStream::open(
                        &backend::null_device(),
                        SPATIAL_SAMPLE_RATE,
                        2,
                        shared.clone(),
                        QUANTUM_FRAMES,
                    ) {
                        Ok(o) => o,
                        Err(e) => {
                            log::error!(target: "audio", "spatial audio null fallback failed: {e}");
                            return AUDCLNT_E_ENDPOINT_CREATE_FAILED;
                        }
                    }
                }
            }
        };

        if !notify_object.is_null() {
            guest_unknown_call(notify_object, 1);
        }

        let ctrl = out.control();
        let out_io_frames = out.io_frames;
        let out_kind = out.kind;
        let core = Arc::new(SpatialStreamCore {
            ep: inner.ep.clone(),
            shared,
            _out: out,
            ctrl,
            out_io_frames,
            notify_object: notify_object as usize,
            st: Mutex::new(StreamState::new(static_mask, granted_dynamic, QUANTUM_FRAMES)),
        });

        {
            let mut active = inner.active_stream.lock().unwrap();
            if active.strong_count() > 0 {
                return AUDCLNT_E_ALREADY_INITIALIZED;
            }
            *active = Arc::downgrade(&core);
        }

        log::info!(
            target: "audio",
            "ISpatialAudioClient::ActivateSpatialAudioStream on '{}': static_mask={static_mask:#x}, dynamic={min_dyn}..={granted_dynamic}, cat={category}, quantum={} frames @ {} Hz, backend={out_kind:?}, io_slice={out_io_frames}",
            inner.ep.info.name,
            QUANTUM_FRAMES,
            SPATIAL_SAMPLE_RATE
        );

        let obj = SpatialStreamObj::create(render_stream::vtbl(), core);
        std::ptr::write_unaligned(stream_out, obj as *mut c_void);
        S_OK
    }

    pub fn vtbl() -> &'static ComVtbl<11> {
        static V: OnceLock<ComVtbl<11>> = OnceLock::new();
        V.get_or_init(|| {
            register_vtbl([
                qi as *const () as usize,                                        // 0: IUnknown::QueryInterface
                add_ref as *const () as usize,                                   // 1: IUnknown::AddRef
                release as *const () as usize,                                   // 2: IUnknown::Release
                get_static_object_position as *const () as usize,                // 3: GetStaticObjectPosition
                get_native_static_object_type_mask as *const () as usize,        // 4: GetNativeStaticObjectTypeMask
                get_max_dynamic_object_count as *const () as usize,              // 5: GetMaxDynamicObjectCount
                get_supported_audio_object_format_enumerator as *const () as usize, // 6: GetSupportedAudioObjectFormatEnumerator
                get_max_frame_count as *const () as usize,                       // 7: GetMaxFrameCount
                is_audio_object_format_supported as *const () as usize,          // 8: IsAudioObjectFormatSupported
                is_spatial_audio_stream_available as *const () as usize,         // 9: IsSpatialAudioStreamAvailable
                activate_spatial_audio_stream as *const () as usize,             // 10: ActivateSpatialAudioStream
            ])
        })
    }
}

/// Entry point called by `IMMDevice::Activate(IID_ISpatialAudioClient, ...)`.
pub unsafe fn create_spatial_audio_client(ep: Arc<Endpoint>, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if ep.is_capture() {
        return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
    }
    let obj = SpatialClientObj::create(
        client::vtbl(),
        SpatialClientInner {
            ep,
            active_stream: Mutex::new(Weak::new()),
        },
    );
    std::ptr::write_unaligned(out, obj as *mut c_void);
    S_OK
}
