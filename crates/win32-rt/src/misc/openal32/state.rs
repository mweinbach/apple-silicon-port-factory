//! OpenAL 1.1 runtime state, object tables, C-string interning, and property helpers.

use super::mixer::MixerRuntime;
use crate::audio::backend::DeviceInfo;
use crate::strings::rd;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};

// ---------------------------------------------------------------------------
// OpenAL 1.1 / ALC 1.1 / AL_EXT_FLOAT32 Constants
// ---------------------------------------------------------------------------

pub(super) const AL_NONE: i32 = 0;
pub(super) const AL_FALSE: u8 = 0;
pub(super) const AL_TRUE: u8 = 1;

pub(super) const ALC_FALSE: u8 = 0;
pub(super) const ALC_TRUE: u8 = 1;

// ALC error codes
pub(super) const ALC_NO_ERROR: i32 = 0;
pub(super) const ALC_INVALID_DEVICE: i32 = 0xA001;
pub(super) const ALC_INVALID_CONTEXT: i32 = 0xA002;
pub(super) const ALC_INVALID_ENUM: i32 = 0xA003;
pub(super) const ALC_INVALID_VALUE: i32 = 0xA004;
pub(super) const ALC_OUT_OF_MEMORY: i32 = 0xA005;

// ALC queries & attributes
pub(super) const ALC_MAJOR_VERSION: i32 = 0x1000;
pub(super) const ALC_MINOR_VERSION: i32 = 0x1001;
pub(super) const ALC_ATTRIBUTES_SIZE: i32 = 0x1002;
pub(super) const ALC_ALL_ATTRIBUTES: i32 = 0x1003;
pub(super) const ALC_DEFAULT_DEVICE_SPECIFIER: i32 = 0x1004;
pub(super) const ALC_DEVICE_SPECIFIER: i32 = 0x1005;
pub(super) const ALC_EXTENSIONS: i32 = 0x1006;
pub(super) const ALC_FREQUENCY: i32 = 0x1007;
pub(super) const ALC_REFRESH: i32 = 0x1008;
pub(super) const ALC_SYNC: i32 = 0x1009;
pub(super) const ALC_MONO_SOURCES: i32 = 0x1010;
pub(super) const ALC_STEREO_SOURCES: i32 = 0x1011;
pub(super) const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: i32 = 0x1012;
pub(super) const ALC_ALL_DEVICES_SPECIFIER: i32 = 0x1013;
pub(super) const ALC_CAPTURE_DEVICE_SPECIFIER: i32 = 0x0310;
pub(super) const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: i32 = 0x0311;
pub(super) const ALC_CAPTURE_SAMPLES: i32 = 0x0312;

// AL error codes
pub(super) const AL_NO_ERROR: i32 = 0;
pub(super) const AL_INVALID_NAME: i32 = 0xA001;
pub(super) const AL_INVALID_ENUM: i32 = 0xA002;
pub(super) const AL_INVALID_VALUE: i32 = 0xA003;
pub(super) const AL_INVALID_OPERATION: i32 = 0xA004;
pub(super) const AL_OUT_OF_MEMORY: i32 = 0xA005;

// AL string queries
pub(super) const AL_VENDOR: i32 = 0xB001;
pub(super) const AL_VERSION: i32 = 0xB002;
pub(super) const AL_RENDERER: i32 = 0xB003;
pub(super) const AL_EXTENSIONS: i32 = 0xB004;

// AL global state & distance models
pub(super) const AL_DOPPLER_FACTOR: i32 = 0xC000;
pub(super) const AL_DOPPLER_VELOCITY: i32 = 0xC001;
pub(super) const AL_SPEED_OF_SOUND: i32 = 0xC003;
pub(super) const AL_DISTANCE_MODEL: i32 = 0xD000;
pub(super) const AL_INVERSE_DISTANCE: i32 = 0xD001;
pub(super) const AL_INVERSE_DISTANCE_CLAMPED: i32 = 0xD002;
pub(super) const AL_LINEAR_DISTANCE: i32 = 0xD003;
pub(super) const AL_LINEAR_DISTANCE_CLAMPED: i32 = 0xD004;
pub(super) const AL_EXPONENT_DISTANCE: i32 = 0xD005;
pub(super) const AL_EXPONENT_DISTANCE_CLAMPED: i32 = 0xD006;

// AL capabilities (optional extension enum)
pub(super) const AL_SOURCE_DISTANCE_MODEL: i32 = 0x0200;

// AL source & listener properties
pub(super) const AL_SOURCE_RELATIVE: i32 = 0x0202;
pub(super) const AL_CONE_INNER_ANGLE: i32 = 0x1001;
pub(super) const AL_CONE_OUTER_ANGLE: i32 = 0x1002;
pub(super) const AL_PITCH: i32 = 0x1003;
pub(super) const AL_POSITION: i32 = 0x1004;
pub(super) const AL_DIRECTION: i32 = 0x1005;
pub(super) const AL_VELOCITY: i32 = 0x1006;
pub(super) const AL_LOOPING: i32 = 0x1007;
pub(super) const AL_BUFFER: i32 = 0x1009;
pub(super) const AL_GAIN: i32 = 0x100A;
pub(super) const AL_MIN_GAIN: i32 = 0x100D;
pub(super) const AL_MAX_GAIN: i32 = 0x100E;
pub(super) const AL_ORIENTATION: i32 = 0x100F;
pub(super) const AL_SOURCE_STATE: i32 = 0x1010;
pub(super) const AL_INITIAL: i32 = 0x1011;
pub(super) const AL_PLAYING: i32 = 0x1012;
pub(super) const AL_PAUSED: i32 = 0x1013;
pub(super) const AL_STOPPED: i32 = 0x1014;
pub(super) const AL_BUFFERS_QUEUED: i32 = 0x1015;
pub(super) const AL_BUFFERS_PROCESSED: i32 = 0x1016;
pub(super) const AL_REFERENCE_DISTANCE: i32 = 0x1020;
pub(super) const AL_ROLLOFF_FACTOR: i32 = 0x1021;
pub(super) const AL_CONE_OUTER_GAIN: i32 = 0x1022;
pub(super) const AL_MAX_DISTANCE: i32 = 0x1023;
pub(super) const AL_SEC_OFFSET: i32 = 0x1024;
pub(super) const AL_SAMPLE_OFFSET: i32 = 0x1025;
pub(super) const AL_BYTE_OFFSET: i32 = 0x1026;
pub(super) const AL_SOURCE_TYPE: i32 = 0x1027;
pub(super) const AL_STATIC: i32 = 0x1028;
pub(super) const AL_STREAMING: i32 = 0x1029;
pub(super) const AL_UNDETERMINED: i32 = 0x1030;

// AL buffer formats & properties
pub(super) const AL_FORMAT_MONO8: i32 = 0x1100;
pub(super) const AL_FORMAT_MONO16: i32 = 0x1101;
pub(super) const AL_FORMAT_STEREO8: i32 = 0x1102;
pub(super) const AL_FORMAT_STEREO16: i32 = 0x1103;
pub(super) const AL_FORMAT_MONO_FLOAT32: i32 = 0x10010;
pub(super) const AL_FORMAT_STEREO_FLOAT32: i32 = 0x10011;
pub(super) const AL_FREQUENCY: i32 = 0x2001;
pub(super) const AL_BITS: i32 = 0x2002;
pub(super) const AL_CHANNELS: i32 = 0x2003;
pub(super) const AL_SIZE: i32 = 0x2004;

pub(super) const DEFAULT_OUTPUT_RATE: u32 = 48_000;
pub(super) const DEFAULT_REFRESH: i32 = 60;
pub(super) const DEFAULT_MONO_SOURCES: i32 = 256;
pub(super) const DEFAULT_STEREO_SOURCES: i32 = 64;
pub(super) const MIX_CHUNK_FRAMES: usize = 256;
pub(super) const RING_CAPACITY_FRAMES: usize = 4096;

// ---------------------------------------------------------------------------
// Scalar Clamping & C-String Interning Helpers
// ---------------------------------------------------------------------------

#[inline]
pub(super) fn clamp_f32(v: f32, lo: f32, hi: f32) -> f32 {
    if !v.is_finite() {
        return lo;
    }
    let (a, b) = if lo <= hi { (lo, hi) } else { (lo, lo) };
    if v < a {
        a
    } else if v > b {
        b
    } else {
        v
    }
}

/// Intern raw NUL-terminated (or double-NUL-terminated) bytes so pointers
/// returned by `alcGetString` stay permanently valid without leaking on repeat calls.
fn intern_bytes(bytes: Vec<u8>) -> *const u8 {
    static CACHE: OnceLock<Mutex<HashMap<Vec<u8>, &'static [u8]>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = cache.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = guard.get(&bytes) {
        return existing.as_ptr();
    }
    let leaked: &'static [u8] = Box::leak(bytes.clone().into_boxed_slice());
    let ptr = leaked.as_ptr();
    guard.insert(bytes, leaked);
    ptr
}

pub(super) fn intern_cstr(s: &str) -> *const u8 {
    let mut v = Vec::with_capacity(s.len() + 1);
    for &b in s.as_bytes() {
        if b != 0 {
            v.push(b);
        }
    }
    v.push(0);
    intern_bytes(v)
}

pub(super) fn intern_cstr_list(items: &[DeviceInfo]) -> *const u8 {
    let mut v = Vec::new();
    for d in items {
        for &b in d.name.as_bytes() {
            if b != 0 {
                v.push(b);
            }
        }
        v.push(0);
    }
    if v.is_empty() {
        v.push(0);
    }
    v.push(0);
    intern_bytes(v)
}

// ---------------------------------------------------------------------------
// Core Runtime Data Structures
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(super) struct BufferData {
    /// Interleaved float32 PCM samples (`frames * channels`).
    pub(super) samples: Vec<f32>,
    pub(super) freq: u32,
    pub(super) channels: u16,
    pub(super) bits: u16,
    pub(super) size_bytes: i32,
}

impl Default for BufferData {
    fn default() -> Self {
        Self {
            samples: Vec::new(),
            freq: DEFAULT_OUTPUT_RATE,
            channels: 1,
            bits: 16,
            size_bytes: 0,
        }
    }
}

impl BufferData {
    #[inline]
    pub(super) fn frames(&self) -> usize {
        let ch = (self.channels as usize).max(1);
        self.samples.len() / ch
    }

    #[inline]
    pub(super) fn frame_bytes(&self) -> usize {
        (self.channels as usize).max(1) * ((self.bits as usize) / 8).max(1)
    }

    #[inline]
    pub(super) fn sample_mono(&self, frame: usize) -> f32 {
        let ch = (self.channels as usize).max(1);
        let base = frame * ch;
        if ch == 1 {
            self.samples.get(base).copied().unwrap_or(0.0)
        } else {
            let l = self.samples.get(base).copied().unwrap_or(0.0);
            let r = self.samples.get(base + 1).copied().unwrap_or(l);
            0.5 * (l + r)
        }
    }

    #[inline]
    pub(super) fn sample_stereo(&self, frame: usize) -> (f32, f32) {
        let ch = (self.channels as usize).max(1);
        let base = frame * ch;
        if ch == 1 {
            let s = self.samples.get(base).copied().unwrap_or(0.0);
            (s, s)
        } else {
            let l = self.samples.get(base).copied().unwrap_or(0.0);
            let r = self.samples.get(base + 1).copied().unwrap_or(l);
            (l, r)
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Listener {
    pub(super) gain: f32,
    pub(super) position: [f32; 3],
    pub(super) velocity: [f32; 3],
    /// `[at_x, at_y, at_z, up_x, up_y, up_z]`
    pub(super) orientation: [f32; 6],
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            gain: 1.0,
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            orientation: [0.0, 0.0, -1.0, 0.0, 1.0, 0.0],
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Source {
    pub(super) state: i32,
    pub(super) source_type: i32,
    pub(super) queue: VecDeque<u32>,
    pub(super) processed: usize,
    pub(super) frame_pos: f64,
    pub(super) pending_offset: bool,

    pub(super) looping: bool,
    pub(super) gain: f32,
    pub(super) min_gain: f32,
    pub(super) max_gain: f32,
    pub(super) pitch: f32,
    pub(super) position: [f32; 3],
    pub(super) velocity: [f32; 3],
    pub(super) direction: [f32; 3],
    pub(super) source_relative: bool,
    pub(super) reference_distance: f32,
    pub(super) rolloff_factor: f32,
    pub(super) max_distance: f32,
    pub(super) cone_inner_angle: f32,
    pub(super) cone_outer_angle: f32,
    pub(super) cone_outer_gain: f32,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            state: AL_INITIAL,
            source_type: AL_UNDETERMINED,
            queue: VecDeque::new(),
            processed: 0,
            frame_pos: 0.0,
            pending_offset: false,

            looping: false,
            gain: 1.0,
            min_gain: 0.0,
            max_gain: 1.0,
            pitch: 1.0,
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 0.0],
            source_relative: false,
            reference_distance: 1.0,
            rolloff_factor: 1.0,
            max_distance: f32::MAX,
            cone_inner_angle: 360.0,
            cone_outer_angle: 360.0,
            cone_outer_gain: 0.0,
        }
    }
}

impl Source {
    pub(super) fn current_buffer_id(&self) -> u32 {
        if self.queue.is_empty() {
            return 0;
        }
        match self.source_type {
            AL_STATIC => self.queue.front().copied().unwrap_or(0),
            AL_STREAMING => {
                if self.state == AL_STOPPED {
                    0
                } else {
                    self.queue.get(self.processed).copied().unwrap_or(0)
                }
            }
            _ => 0,
        }
    }

    pub(super) fn buffers_processed(&self) -> i32 {
        if self.state == AL_INITIAL {
            return 0;
        }
        match self.source_type {
            AL_STATIC => {
                if self.state == AL_STOPPED && !self.queue.is_empty() {
                    1
                } else {
                    0
                }
            }
            AL_STREAMING => self.processed.min(self.queue.len()) as i32,
            _ => 0,
        }
    }

    pub(super) fn play(&mut self) {
        if self.queue.is_empty() {
            self.state = AL_STOPPED;
            self.processed = 0;
            self.frame_pos = 0.0;
            self.pending_offset = false;
            return;
        }
        match self.state {
            AL_PAUSED => {
                self.state = AL_PLAYING;
            }
            AL_PLAYING => {
                self.processed = 0;
                self.frame_pos = 0.0;
                self.pending_offset = false;
            }
            // AL_INITIAL | AL_STOPPED
            _ => {
                if !self.pending_offset {
                    self.processed = 0;
                    self.frame_pos = 0.0;
                }
                self.pending_offset = false;
                if self.processed >= self.queue.len() {
                    self.processed = 0;
                    self.frame_pos = 0.0;
                }
                self.state = AL_PLAYING;
            }
        }
    }

    pub(super) fn pause(&mut self) {
        if self.state == AL_PLAYING {
            self.state = AL_PAUSED;
        }
    }

    pub(super) fn stop(&mut self) {
        if self.state != AL_STOPPED {
            self.state = AL_STOPPED;
            self.processed = self.queue.len();
            self.frame_pos = 0.0;
            self.pending_offset = false;
        }
    }

    pub(super) fn rewind(&mut self) {
        self.state = AL_INITIAL;
        self.processed = 0;
        self.frame_pos = 0.0;
        self.pending_offset = false;
    }

    pub(super) fn offsets(&self, buffers: &HashMap<u32, Arc<BufferData>>) -> (f64, f64, f64) {
        if self.state == AL_STOPPED || self.queue.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        let mut secs = 0.0f64;
        let mut frames = 0.0f64;
        let mut bytes = 0.0f64;

        let done = self.processed.min(self.queue.len());
        for i in 0..done {
            if let Some(buf) = buffers.get(&self.queue[i]) {
                let f = buf.frames() as f64;
                frames += f;
                bytes += buf.size_bytes.max(0) as f64;
                secs += f / buf.freq.max(1) as f64;
            }
        }
        if done < self.queue.len() {
            if let Some(cur) = buffers.get(&self.queue[done]) {
                let fp = self.frame_pos.clamp(0.0, cur.frames() as f64);
                frames += fp;
                bytes += fp.floor() * cur.frame_bytes() as f64;
                secs += fp / cur.freq.max(1) as f64;
            }
        }
        (secs, frames, bytes)
    }

    pub(super) fn set_offset(
        &mut self,
        buffers: &HashMap<u32, Arc<BufferData>>,
        param: i32,
        value: f64,
    ) -> Result<(), i32> {
        if !value.is_finite() || value < 0.0 {
            return Err(AL_INVALID_VALUE);
        }
        if self.queue.is_empty() {
            return if value == 0.0 {
                self.processed = 0;
                self.frame_pos = 0.0;
                Ok(())
            } else {
                Err(AL_INVALID_VALUE)
            };
        }
        let mut rem = value;
        for (idx, bid) in self.queue.iter().enumerate() {
            let Some(buf) = buffers.get(bid) else {
                continue;
            };
            let b_frames = buf.frames() as f64;
            let b_freq = buf.freq.max(1) as f64;
            let b_fbytes = buf.frame_bytes().max(1) as f64;
            let span = match param {
                AL_SEC_OFFSET => b_frames / b_freq,
                AL_SAMPLE_OFFSET => b_frames,
                AL_BYTE_OFFSET => b_frames * b_fbytes,
                _ => return Err(AL_INVALID_ENUM),
            };
            if rem < span || (rem == 0.0 && span == 0.0) {
                let local_frame = match param {
                    AL_SEC_OFFSET => rem * b_freq,
                    AL_SAMPLE_OFFSET => rem,
                    AL_BYTE_OFFSET => (rem / b_fbytes).floor(),
                    _ => 0.0,
                };
                self.processed = idx;
                self.frame_pos = local_frame.clamp(0.0, b_frames);
                if self.state == AL_INITIAL || self.state == AL_STOPPED {
                    self.pending_offset = true;
                }
                return Ok(());
            }
            rem -= span;
        }
        Err(AL_INVALID_VALUE)
    }
}

pub(super) struct Device {
    pub(super) info: DeviceInfo,
    pub(super) last_error: i32,
    pub(super) buffers: HashMap<u32, Arc<BufferData>>,
    pub(super) contexts: Vec<usize>,
    pub(super) output_rate: u32,
    pub(super) refresh: i32,
    pub(super) sync: bool,
    pub(super) mono_sources: i32,
    pub(super) stereo_sources: i32,
}

impl Device {
    pub(super) fn new(info: DeviceInfo) -> Self {
        let rate = if info.rate >= 8_000 && info.rate <= 192_000 {
            info.rate
        } else {
            DEFAULT_OUTPUT_RATE
        };
        Self {
            info,
            last_error: ALC_NO_ERROR,
            buffers: HashMap::new(),
            contexts: Vec::new(),
            output_rate: rate,
            refresh: DEFAULT_REFRESH,
            sync: false,
            mono_sources: DEFAULT_MONO_SOURCES,
            stereo_sources: DEFAULT_STEREO_SOURCES,
        }
    }

    pub(super) fn set_error(&mut self, err: i32) {
        if self.last_error == ALC_NO_ERROR {
            self.last_error = err;
        }
    }
}

pub(super) struct Context {
    pub(super) device: usize,
    pub(super) output_rate: u32,
    pub(super) refresh: i32,
    pub(super) sync: bool,
    pub(super) mono_sources: i32,
    pub(super) stereo_sources: i32,
    pub(super) suspended: bool,
    pub(super) last_error: i32,
    pub(super) listener: Listener,
    pub(super) distance_model: i32,
    pub(super) source_distance_model_enabled: bool,
    pub(super) doppler_factor: f32,
    pub(super) doppler_velocity: f32,
    pub(super) speed_of_sound: f32,
    pub(super) sources: HashMap<u32, Source>,
    pub(super) mixer: Option<MixerRuntime>,
}

impl Context {
    pub(super) fn new(
        device: usize,
        output_rate: u32,
        refresh: i32,
        sync: bool,
        mono_sources: i32,
        stereo_sources: i32,
    ) -> Self {
        Self {
            device,
            output_rate,
            refresh,
            sync,
            mono_sources,
            stereo_sources,
            suspended: false,
            last_error: AL_NO_ERROR,
            listener: Listener::default(),
            distance_model: AL_INVERSE_DISTANCE_CLAMPED,
            source_distance_model_enabled: false,
            doppler_factor: 1.0,
            doppler_velocity: 1.0,
            speed_of_sound: 343.3,
            sources: HashMap::new(),
            mixer: None,
        }
    }

    pub(super) fn set_error(&mut self, err: i32) {
        if self.last_error == AL_NO_ERROR {
            self.last_error = err;
        }
    }

    pub(super) fn has_playing_source(&self) -> bool {
        self.sources.values().any(|s| s.state == AL_PLAYING)
    }
}

pub(super) struct OpenAlState {
    next_handle: usize,
    next_al_id: u32,
    pub(super) devices: HashMap<usize, Device>,
    pub(super) contexts: HashMap<usize, Context>,
    pub(super) current_context: Option<usize>,
    pub(super) global_alc_error: i32,
    pub(super) global_al_error: i32,
}

impl OpenAlState {
    fn new() -> Self {
        Self {
            next_handle: 0xA100_0001,
            next_al_id: 1,
            devices: HashMap::new(),
            contexts: HashMap::new(),
            current_context: None,
            global_alc_error: ALC_NO_ERROR,
            global_al_error: AL_NO_ERROR,
        }
    }

    pub(super) fn alloc_handle(&mut self) -> usize {
        let h = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1).max(0xA100_0001);
        h
    }

    pub(super) fn alloc_al_id(&mut self) -> u32 {
        let id = self.next_al_id;
        self.next_al_id = self.next_al_id.wrapping_add(1);
        if self.next_al_id == 0 {
            self.next_al_id = 1;
        }
        id
    }

    pub(super) fn set_alc_error(&mut self, device: usize, err: i32) {
        if let Some(dev) = self.devices.get_mut(&device) {
            dev.set_error(err);
        } else if self.global_alc_error == ALC_NO_ERROR {
            self.global_alc_error = err;
        }
    }

    pub(super) fn set_al_error(&mut self, err: i32) {
        if let Some(cid) = self.current_context {
            if let Some(ctx) = self.contexts.get_mut(&cid) {
                ctx.set_error(err);
                return;
            }
        }
        if self.global_al_error == AL_NO_ERROR {
            self.global_al_error = err;
        }
    }

    pub(super) fn current_ctx_mut(&mut self) -> Option<&mut Context> {
        let cid = self.current_context?;
        self.contexts.get_mut(&cid)
    }

    pub(super) fn buffer_in_use_on_device(
        &self,
        dev_handle: usize,
        bid: u32,
        active_only: bool,
    ) -> bool {
        let Some(dev) = self.devices.get(&dev_handle) else {
            return false;
        };
        for cid in &dev.contexts {
            if let Some(ctx) = self.contexts.get(cid) {
                for src in ctx.sources.values() {
                    if active_only && src.state != AL_PLAYING && src.state != AL_PAUSED {
                        continue;
                    }
                    if src.queue.contains(&bid) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn state() -> &'static Mutex<OpenAlState> {
    static STATE: OnceLock<Mutex<OpenAlState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(OpenAlState::new()))
}

pub(super) fn lock_state() -> std::sync::MutexGuard<'static, OpenAlState> {
    state().lock().unwrap_or_else(|e| e.into_inner())
}

/// Lazily open & start the host audio mixer for `ctx_handle` if needed.
/// Must be called WITHOUT holding `STATE`.
pub(super) fn ensure_mixer_running(ctx_handle: usize) {
    let start_params: Option<(DeviceInfo, u32)> = {
        let st = lock_state();
        let Some(ctx) = st.contexts.get(&ctx_handle) else {
            return;
        };
        if ctx.sync || ctx.suspended || ctx.mixer.is_some() || !ctx.has_playing_source() {
            return;
        }
        let Some(dev) = st.devices.get(&ctx.device) else {
            return;
        };
        Some((dev.info.clone(), ctx.output_rate))
    };

    let Some((dev_info, output_rate)) = start_params else {
        return;
    };

    let mixer = match MixerRuntime::start(ctx_handle, &dev_info, output_rate) {
        Ok(m) => m,
        Err(e) => {
            log::error!(target: "audio", "openal32: failed to start host mixer: {e}");
            return;
        }
    };

    let unwanted = {
        let mut st = lock_state();
        match st.contexts.get_mut(&ctx_handle) {
            Some(ctx) if ctx.mixer.is_none() && !ctx.sync => {
                ctx.mixer = Some(mixer);
                None
            }
            _ => Some(mixer),
        }
    };
    if let Some(m) = unwanted {
        m.stop();
    }
}

// ---------------------------------------------------------------------------
// Property & Batch Operation Helpers
// ---------------------------------------------------------------------------

pub(super) fn buffer_set_invalid(buffer: u32, _param: i32, ptr_null: bool) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if buffer == 0
        || !st
            .devices
            .get(&dev_handle)
            .is_some_and(|d| d.buffers.contains_key(&buffer))
    {
        st.set_al_error(AL_INVALID_NAME);
        return;
    }
    if ptr_null {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    st.set_al_error(AL_INVALID_ENUM);
}

pub(super) fn get_buffer_prop(buffer: u32, param: i32) -> Result<i32, i32> {
    let st = lock_state();
    let Some(cid) = st.current_context else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        return Err(AL_INVALID_OPERATION);
    };
    if buffer == 0 {
        return Err(AL_INVALID_NAME);
    }
    let Some(buf) = st
        .devices
        .get(&dev_handle)
        .and_then(|d| d.buffers.get(&buffer))
    else {
        return Err(AL_INVALID_NAME);
    };
    match param {
        AL_FREQUENCY => Ok(buf.freq as i32),
        AL_BITS => Ok(buf.bits as i32),
        AL_CHANNELS => Ok(buf.channels as i32),
        AL_SIZE => Ok(buf.size_bytes),
        _ => Err(AL_INVALID_ENUM),
    }
}

pub(super) unsafe fn source_batch_op(
    n: i32,
    sources: *const u32,
    op: fn(&mut Source),
) -> Option<usize> {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return None;
    };
    if n == 0 {
        return None;
    }
    if n < 0 || sources.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return None;
    }
    let mut ids = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        ids.push(rd(sources.add(i)));
    }
    let Some(ctx) = st.contexts.get_mut(&cid) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return None;
    };
    for &sid in &ids {
        if !ctx.sources.contains_key(&sid) {
            ctx.set_error(AL_INVALID_NAME);
            return None;
        }
    }
    for sid in ids {
        if let Some(src) = ctx.sources.get_mut(&sid) {
            op(src);
        }
    }
    if !ctx.sync && !ctx.suspended && ctx.mixer.is_none() && ctx.has_playing_source() {
        Some(cid)
    } else {
        None
    }
}

pub(super) fn source_set_int_prop(source: u32, param: i32, value: i32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };

    if param == AL_BUFFER {
        let bid = value as u32;
        if bid != 0
            && !st
                .devices
                .get(&dev_handle)
                .is_some_and(|d| d.buffers.contains_key(&bid))
        {
            st.set_al_error(AL_INVALID_VALUE);
            return;
        }
        let Some(ctx) = st.contexts.get_mut(&cid) else {
            return;
        };
        let Some(src) = ctx.sources.get_mut(&source) else {
            ctx.set_error(AL_INVALID_NAME);
            return;
        };
        if src.state == AL_PLAYING || src.state == AL_PAUSED {
            ctx.set_error(AL_INVALID_OPERATION);
            return;
        }
        src.queue.clear();
        src.processed = 0;
        src.frame_pos = 0.0;
        src.pending_offset = false;
        if bid == 0 {
            src.source_type = AL_UNDETERMINED;
        } else {
            src.queue.push_back(bid);
            src.source_type = AL_STATIC;
        }
        return;
    }

    if matches!(param, AL_SEC_OFFSET | AL_SAMPLE_OFFSET | AL_BYTE_OFFSET) {
        let buffers = match st.devices.get(&dev_handle) {
            Some(d) => d.buffers.clone(),
            None => return,
        };
        let Some(ctx) = st.contexts.get_mut(&cid) else {
            return;
        };
        let Some(src) = ctx.sources.get_mut(&source) else {
            ctx.set_error(AL_INVALID_NAME);
            return;
        };
        if let Err(e) = src.set_offset(&buffers, param, value as f64) {
            ctx.set_error(e);
        }
        return;
    }

    let Some(ctx) = st.contexts.get_mut(&cid) else {
        return;
    };
    let Some(src) = ctx.sources.get_mut(&source) else {
        ctx.set_error(AL_INVALID_NAME);
        return;
    };

    match param {
        AL_LOOPING => {
            if value != 0 && value != 1 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.looping = value != 0;
            }
        }
        AL_SOURCE_RELATIVE => {
            if value != 0 && value != 1 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.source_relative = value != 0;
            }
        }
        AL_REFERENCE_DISTANCE => {
            if value < 0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.reference_distance = value as f32;
            }
        }
        AL_ROLLOFF_FACTOR => {
            if value < 0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.rolloff_factor = value as f32;
            }
        }
        AL_MAX_DISTANCE => {
            if value < 0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.max_distance = value as f32;
            }
        }
        AL_CONE_INNER_ANGLE => {
            if !(0..=360).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.cone_inner_angle = value as f32;
            }
        }
        AL_CONE_OUTER_ANGLE => {
            if !(0..=360).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.cone_outer_angle = value as f32;
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub(super) fn source_set_float_prop(source: u32, param: i32, value: f32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };

    if matches!(param, AL_SEC_OFFSET | AL_SAMPLE_OFFSET | AL_BYTE_OFFSET) {
        let buffers = match st.devices.get(&dev_handle) {
            Some(d) => d.buffers.clone(),
            None => return,
        };
        let Some(ctx) = st.contexts.get_mut(&cid) else {
            return;
        };
        let Some(src) = ctx.sources.get_mut(&source) else {
            ctx.set_error(AL_INVALID_NAME);
            return;
        };
        if let Err(e) = src.set_offset(&buffers, param, value as f64) {
            ctx.set_error(e);
        }
        return;
    }

    let Some(ctx) = st.contexts.get_mut(&cid) else {
        return;
    };
    let Some(src) = ctx.sources.get_mut(&source) else {
        ctx.set_error(AL_INVALID_NAME);
        return;
    };

    match param {
        AL_GAIN => {
            if !value.is_finite() || value < 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.gain = value;
            }
        }
        AL_MIN_GAIN => {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.min_gain = value;
            }
        }
        AL_MAX_GAIN => {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.max_gain = value;
            }
        }
        AL_PITCH => {
            if !value.is_finite() || value <= 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.pitch = value;
            }
        }
        AL_REFERENCE_DISTANCE => {
            if !value.is_finite() || value < 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.reference_distance = value;
            }
        }
        AL_ROLLOFF_FACTOR => {
            if !value.is_finite() || value < 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.rolloff_factor = value;
            }
        }
        AL_MAX_DISTANCE => {
            if !value.is_finite() || value < 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.max_distance = value;
            }
        }
        AL_CONE_INNER_ANGLE => {
            if !value.is_finite() || !(0.0..=360.0).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.cone_inner_angle = value;
            }
        }
        AL_CONE_OUTER_ANGLE => {
            if !value.is_finite() || !(0.0..=360.0).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.cone_outer_angle = value;
            }
        }
        AL_CONE_OUTER_GAIN => {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                src.cone_outer_gain = value;
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub(super) fn source_set_vec3_prop(source: u32, param: i32, v1: f32, v2: f32, v3: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let Some(src) = ctx.sources.get_mut(&source) else {
        ctx.set_error(AL_INVALID_NAME);
        return;
    };
    if !v1.is_finite() || !v2.is_finite() || !v3.is_finite() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION => src.position = [v1, v2, v3],
        AL_VELOCITY => src.velocity = [v1, v2, v3],
        AL_DIRECTION => src.direction = [v1, v2, v3],
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub(super) fn get_source_int(source: u32, param: i32) -> Result<i32, i32> {
    let st = lock_state();
    let Some(cid) = st.current_context else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(ctx) = st.contexts.get(&cid) else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(src) = ctx.sources.get(&source) else {
        return Err(AL_INVALID_NAME);
    };
    match param {
        AL_BUFFER => Ok(src.current_buffer_id() as i32),
        AL_SOURCE_STATE => Ok(src.state),
        AL_SOURCE_TYPE => Ok(src.source_type),
        AL_LOOPING => Ok(if src.looping { 1 } else { 0 }),
        AL_SOURCE_RELATIVE => Ok(if src.source_relative { 1 } else { 0 }),
        AL_BUFFERS_QUEUED => Ok(src.queue.len() as i32),
        AL_BUFFERS_PROCESSED => Ok(src.buffers_processed()),
        AL_REFERENCE_DISTANCE => Ok(src.reference_distance.round() as i32),
        AL_ROLLOFF_FACTOR => Ok(src.rolloff_factor.round() as i32),
        AL_MAX_DISTANCE => Ok(src.max_distance.min(i32::MAX as f32).round() as i32),
        AL_CONE_INNER_ANGLE => Ok(src.cone_inner_angle.round() as i32),
        AL_CONE_OUTER_ANGLE => Ok(src.cone_outer_angle.round() as i32),
        AL_SEC_OFFSET | AL_SAMPLE_OFFSET | AL_BYTE_OFFSET => {
            let empty = HashMap::new();
            let bufs = st
                .devices
                .get(&ctx.device)
                .map(|d| &d.buffers)
                .unwrap_or(&empty);
            let (secs, frames, bytes) = src.offsets(bufs);
            Ok(match param {
                AL_SEC_OFFSET => secs as i32,
                AL_SAMPLE_OFFSET => frames as i32,
                AL_BYTE_OFFSET => bytes as i32,
                _ => 0,
            })
        }
        _ => Err(AL_INVALID_ENUM),
    }
}

pub(super) fn get_source_float(source: u32, param: i32) -> Result<f32, i32> {
    let st = lock_state();
    let Some(cid) = st.current_context else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(ctx) = st.contexts.get(&cid) else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(src) = ctx.sources.get(&source) else {
        return Err(AL_INVALID_NAME);
    };
    match param {
        AL_GAIN => Ok(src.gain),
        AL_MIN_GAIN => Ok(src.min_gain),
        AL_MAX_GAIN => Ok(src.max_gain),
        AL_PITCH => Ok(src.pitch),
        AL_REFERENCE_DISTANCE => Ok(src.reference_distance),
        AL_ROLLOFF_FACTOR => Ok(src.rolloff_factor),
        AL_MAX_DISTANCE => Ok(src.max_distance),
        AL_CONE_INNER_ANGLE => Ok(src.cone_inner_angle),
        AL_CONE_OUTER_ANGLE => Ok(src.cone_outer_angle),
        AL_CONE_OUTER_GAIN => Ok(src.cone_outer_gain),
        AL_SEC_OFFSET | AL_SAMPLE_OFFSET | AL_BYTE_OFFSET => {
            let empty = HashMap::new();
            let bufs = st
                .devices
                .get(&ctx.device)
                .map(|d| &d.buffers)
                .unwrap_or(&empty);
            let (secs, frames, bytes) = src.offsets(bufs);
            Ok(match param {
                AL_SEC_OFFSET => secs as f32,
                AL_SAMPLE_OFFSET => frames as f32,
                AL_BYTE_OFFSET => bytes as f32,
                _ => 0.0,
            })
        }
        _ => Err(AL_INVALID_ENUM),
    }
}

pub(super) fn get_source_vec3(source: u32, param: i32) -> Result<[f32; 3], i32> {
    let st = lock_state();
    let Some(cid) = st.current_context else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(ctx) = st.contexts.get(&cid) else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(src) = ctx.sources.get(&source) else {
        return Err(AL_INVALID_NAME);
    };
    match param {
        AL_POSITION => Ok(src.position),
        AL_VELOCITY => Ok(src.velocity),
        AL_DIRECTION => Ok(src.direction),
        _ => Err(AL_INVALID_ENUM),
    }
}

pub(super) fn listener_set_float(param: i32, value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => {
            if !value.is_finite() || value < 0.0 {
                ctx.set_error(AL_INVALID_VALUE);
            } else {
                ctx.listener.gain = value;
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub(super) fn listener_set_vec3(param: i32, v1: f32, v2: f32, v3: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !v1.is_finite() || !v2.is_finite() || !v3.is_finite() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION => ctx.listener.position = [v1, v2, v3],
        AL_VELOCITY => ctx.listener.velocity = [v1, v2, v3],
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub(super) fn listener_set_orientation(vals: [f32; 6]) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if vals.iter().any(|v| !v.is_finite()) {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    ctx.listener.orientation = vals;
}

pub(super) fn get_global_param_f64(param: i32) -> Result<f64, i32> {
    let st = lock_state();
    let Some(cid) = st.current_context else {
        return Err(AL_INVALID_OPERATION);
    };
    let Some(ctx) = st.contexts.get(&cid) else {
        return Err(AL_INVALID_OPERATION);
    };
    match param {
        AL_DOPPLER_FACTOR => Ok(ctx.doppler_factor as f64),
        AL_DOPPLER_VELOCITY => Ok(ctx.doppler_velocity as f64),
        AL_SPEED_OF_SOUND => Ok(ctx.speed_of_sound as f64),
        AL_DISTANCE_MODEL => Ok(ctx.distance_model as f64),
        _ => Err(AL_INVALID_ENUM),
    }
}
