//! CoreAudio-backed OpenAL 1.1 runtime (`openal32.dll`).
//!
//! Implements the OpenAL 1.1 ALC (device/context management) and AL (buffers,
//! sources, streaming queues, 3D listener, distance attenuation models, stereo
//! panning, linear-interpolation resampling, and error reporting) APIs on top
//! of [`crate::audio::backend`] (CoreAudio AUHAL on macOS, RemoteIO on iOS,
//! and the real-time Null backend when `AW2_AUDIO_BACKEND=null` or headless).

#![allow(non_snake_case)]

use crate::audio::backend::{self, DeviceInfo, OutputStream, StreamShared};
use crate::audio::ring::Ring;
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

// ---------------------------------------------------------------------------
// OpenAL 1.1 / ALC 1.1 / AL_EXT_FLOAT32 Constants
// ---------------------------------------------------------------------------

const AL_NONE: i32 = 0;
const AL_FALSE: u8 = 0;
const AL_TRUE: u8 = 1;

const ALC_FALSE: u8 = 0;
const ALC_TRUE: u8 = 1;

// ALC error codes
const ALC_NO_ERROR: i32 = 0;
const ALC_INVALID_DEVICE: i32 = 0xA001;
const ALC_INVALID_CONTEXT: i32 = 0xA002;
const ALC_INVALID_ENUM: i32 = 0xA003;
const ALC_INVALID_VALUE: i32 = 0xA004;
const ALC_OUT_OF_MEMORY: i32 = 0xA005;

// ALC queries & attributes
const ALC_MAJOR_VERSION: i32 = 0x1000;
const ALC_MINOR_VERSION: i32 = 0x1001;
const ALC_ATTRIBUTES_SIZE: i32 = 0x1002;
const ALC_ALL_ATTRIBUTES: i32 = 0x1003;
const ALC_DEFAULT_DEVICE_SPECIFIER: i32 = 0x1004;
const ALC_DEVICE_SPECIFIER: i32 = 0x1005;
const ALC_EXTENSIONS: i32 = 0x1006;
const ALC_FREQUENCY: i32 = 0x1007;
const ALC_REFRESH: i32 = 0x1008;
const ALC_SYNC: i32 = 0x1009;
const ALC_MONO_SOURCES: i32 = 0x1010;
const ALC_STEREO_SOURCES: i32 = 0x1011;
const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: i32 = 0x1012;
const ALC_ALL_DEVICES_SPECIFIER: i32 = 0x1013;
const ALC_CAPTURE_DEVICE_SPECIFIER: i32 = 0x0310;
const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: i32 = 0x0311;
const ALC_CAPTURE_SAMPLES: i32 = 0x0312;

// AL error codes
const AL_NO_ERROR: i32 = 0;
const AL_INVALID_NAME: i32 = 0xA001;
const AL_INVALID_ENUM: i32 = 0xA002;
const AL_INVALID_VALUE: i32 = 0xA003;
const AL_INVALID_OPERATION: i32 = 0xA004;
const AL_OUT_OF_MEMORY: i32 = 0xA005;

// AL string queries
const AL_VENDOR: i32 = 0xB001;
const AL_VERSION: i32 = 0xB002;
const AL_RENDERER: i32 = 0xB003;
const AL_EXTENSIONS: i32 = 0xB004;

// AL global state & distance models
const AL_DOPPLER_FACTOR: i32 = 0xC000;
const AL_DOPPLER_VELOCITY: i32 = 0xC001;
const AL_SPEED_OF_SOUND: i32 = 0xC003;
const AL_DISTANCE_MODEL: i32 = 0xD000;
const AL_INVERSE_DISTANCE: i32 = 0xD001;
const AL_INVERSE_DISTANCE_CLAMPED: i32 = 0xD002;
const AL_LINEAR_DISTANCE: i32 = 0xD003;
const AL_LINEAR_DISTANCE_CLAMPED: i32 = 0xD004;
const AL_EXPONENT_DISTANCE: i32 = 0xD005;
const AL_EXPONENT_DISTANCE_CLAMPED: i32 = 0xD006;

// AL capabilities (optional extension enum)
const AL_SOURCE_DISTANCE_MODEL: i32 = 0x0200;

// AL source & listener properties
const AL_SOURCE_RELATIVE: i32 = 0x0202;
const AL_CONE_INNER_ANGLE: i32 = 0x1001;
const AL_CONE_OUTER_ANGLE: i32 = 0x1002;
const AL_PITCH: i32 = 0x1003;
const AL_POSITION: i32 = 0x1004;
const AL_DIRECTION: i32 = 0x1005;
const AL_VELOCITY: i32 = 0x1006;
const AL_LOOPING: i32 = 0x1007;
const AL_BUFFER: i32 = 0x1009;
const AL_GAIN: i32 = 0x100A;
const AL_MIN_GAIN: i32 = 0x100D;
const AL_MAX_GAIN: i32 = 0x100E;
const AL_ORIENTATION: i32 = 0x100F;
const AL_SOURCE_STATE: i32 = 0x1010;
const AL_INITIAL: i32 = 0x1011;
const AL_PLAYING: i32 = 0x1012;
const AL_PAUSED: i32 = 0x1013;
const AL_STOPPED: i32 = 0x1014;
const AL_BUFFERS_QUEUED: i32 = 0x1015;
const AL_BUFFERS_PROCESSED: i32 = 0x1016;
const AL_REFERENCE_DISTANCE: i32 = 0x1020;
const AL_ROLLOFF_FACTOR: i32 = 0x1021;
const AL_CONE_OUTER_GAIN: i32 = 0x1022;
const AL_MAX_DISTANCE: i32 = 0x1023;
const AL_SEC_OFFSET: i32 = 0x1024;
const AL_SAMPLE_OFFSET: i32 = 0x1025;
const AL_BYTE_OFFSET: i32 = 0x1026;
const AL_SOURCE_TYPE: i32 = 0x1027;
const AL_STATIC: i32 = 0x1028;
const AL_STREAMING: i32 = 0x1029;
const AL_UNDETERMINED: i32 = 0x1030;

// AL buffer formats & properties
const AL_FORMAT_MONO8: i32 = 0x1100;
const AL_FORMAT_MONO16: i32 = 0x1101;
const AL_FORMAT_STEREO8: i32 = 0x1102;
const AL_FORMAT_STEREO16: i32 = 0x1103;
const AL_FORMAT_MONO_FLOAT32: i32 = 0x10010;
const AL_FORMAT_STEREO_FLOAT32: i32 = 0x10011;
const AL_FREQUENCY: i32 = 0x2001;
const AL_BITS: i32 = 0x2002;
const AL_CHANNELS: i32 = 0x2003;
const AL_SIZE: i32 = 0x2004;

const DEFAULT_OUTPUT_RATE: u32 = 48_000;
const DEFAULT_REFRESH: i32 = 60;
const DEFAULT_MONO_SOURCES: i32 = 256;
const DEFAULT_STEREO_SOURCES: i32 = 64;
const MIX_CHUNK_FRAMES: usize = 256;
const RING_CAPACITY_FRAMES: usize = 4096;

// ---------------------------------------------------------------------------
// Export tables (mixed int+float Win64 ABI entries bridged for aarch64)
// ---------------------------------------------------------------------------

pub static ORDINALS: &[(u16, &str)] = &[];

pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "alSourcef",
        ptr: crate::Ptr(crate::bridge_entry!(al_source_f)),
    },
    crate::Export {
        name: "alSource3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_source_3f)),
    },
    crate::Export {
        name: "alListenerf",
        ptr: crate::Ptr(crate::bridge_entry!(al_listener_f)),
    },
    crate::Export {
        name: "alListener3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_listener_3f)),
    },
    crate::Export {
        name: "alBufferf",
        ptr: crate::Ptr(crate::bridge_entry!(al_buffer_f)),
    },
    crate::Export {
        name: "alBuffer3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_buffer_3f)),
    },
];

// ---------------------------------------------------------------------------
// Unaligned guest memory & C-string interning helpers
// ---------------------------------------------------------------------------

#[inline]
unsafe fn rd<T: Copy>(p: *const T) -> T {
    std::ptr::read_unaligned(p)
}

#[inline]
unsafe fn wr<T>(p: *mut T, v: T) {
    std::ptr::write_unaligned(p, v);
}

#[inline]
fn clamp_f32(v: f32, lo: f32, hi: f32) -> f32 {
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

fn intern_cstr(s: &str) -> *const u8 {
    let mut v = Vec::with_capacity(s.len() + 1);
    for &b in s.as_bytes() {
        if b != 0 {
            v.push(b);
        }
    }
    v.push(0);
    intern_bytes(v)
}

fn intern_cstr_list(items: &[DeviceInfo]) -> *const u8 {
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
struct BufferData {
    /// Interleaved float32 PCM samples (`frames * channels`).
    samples: Vec<f32>,
    freq: u32,
    channels: u16,
    bits: u16,
    size_bytes: i32,
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
    fn frames(&self) -> usize {
        let ch = (self.channels as usize).max(1);
        self.samples.len() / ch
    }

    #[inline]
    fn frame_bytes(&self) -> usize {
        (self.channels as usize).max(1) * ((self.bits as usize) / 8).max(1)
    }

    #[inline]
    fn sample_mono(&self, frame: usize) -> f32 {
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
    fn sample_stereo(&self, frame: usize) -> (f32, f32) {
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
struct Listener {
    gain: f32,
    position: [f32; 3],
    velocity: [f32; 3],
    /// `[at_x, at_y, at_z, up_x, up_y, up_z]`
    orientation: [f32; 6],
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
struct Source {
    state: i32,
    source_type: i32,
    queue: VecDeque<u32>,
    processed: usize,
    frame_pos: f64,
    pending_offset: bool,

    looping: bool,
    gain: f32,
    min_gain: f32,
    max_gain: f32,
    pitch: f32,
    position: [f32; 3],
    velocity: [f32; 3],
    direction: [f32; 3],
    source_relative: bool,
    reference_distance: f32,
    rolloff_factor: f32,
    max_distance: f32,
    cone_inner_angle: f32,
    cone_outer_angle: f32,
    cone_outer_gain: f32,
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
    fn current_buffer_id(&self) -> u32 {
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

    fn buffers_processed(&self) -> i32 {
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

    fn play(&mut self) {
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

    fn pause(&mut self) {
        if self.state == AL_PLAYING {
            self.state = AL_PAUSED;
        }
    }

    fn stop(&mut self) {
        if self.state != AL_STOPPED {
            self.state = AL_STOPPED;
            self.processed = self.queue.len();
            self.frame_pos = 0.0;
            self.pending_offset = false;
        }
    }

    fn rewind(&mut self) {
        self.state = AL_INITIAL;
        self.processed = 0;
        self.frame_pos = 0.0;
        self.pending_offset = false;
    }

    fn offsets(&self, buffers: &HashMap<u32, Arc<BufferData>>) -> (f64, f64, f64) {
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

    fn set_offset(
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

/// Host mixer thread + [`OutputStream`] bound to a single OpenAL context.
struct MixerRuntime {
    quit: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    stream: Option<OutputStream>,
}

impl MixerRuntime {
    fn start(ctx_handle: usize, dev_info: &DeviceInfo, output_rate: u32) -> Result<Self, String> {
        let rate = output_rate.clamp(8_000, 192_000);
        let ring = Ring::new(RING_CAPACITY_FRAMES, 2);
        let shared = StreamShared::new(ring);
        let mut stream = match OutputStream::open(dev_info, rate, 2, shared.clone(), 512) {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    target: "audio",
                    "openal32: OutputStream::open failed ({e}); falling back to null output"
                );
                let null_dev = backend::null_device();
                OutputStream::open(&null_dev, rate, 2, shared.clone(), 512)?
            }
        };
        if let Err(e) = stream.start() {
            log::warn!(
                target: "audio",
                "openal32: OutputStream::start failed ({e}); falling back to null output"
            );
            drop(stream);
            let ring2 = Ring::new(RING_CAPACITY_FRAMES, 2);
            let shared2 = StreamShared::new(ring2);
            let null_dev = backend::null_device();
            let mut null_stream = OutputStream::open(&null_dev, rate, 2, shared2.clone(), 512)?;
            null_stream.start()?;
            return Self::spawn_thread(ctx_handle, null_stream, shared2);
        }
        Self::spawn_thread(ctx_handle, stream, shared)
    }

    fn spawn_thread(
        ctx_handle: usize,
        stream: OutputStream,
        shared: Arc<StreamShared>,
    ) -> Result<Self, String> {
        let io_frames = (stream.io_frames as usize).max(256);
        let target_frames = (io_frames * 3).clamp(1024, RING_CAPACITY_FRAMES - MIX_CHUNK_FRAMES);
        let quit = Arc::new(AtomicBool::new(false));
        let quit_thread = quit.clone();
        let thread = std::thread::Builder::new()
            .name("openal-mixer".into())
            .spawn(move || {
                let mut mix_buf = [0.0f32; MIX_CHUNK_FRAMES * 2];
                while !quit_thread.load(Ordering::Acquire) {
                    let mut pumped = false;
                    while !quit_thread.load(Ordering::Acquire)
                        && shared.ring.len() + MIX_CHUNK_FRAMES <= target_frames
                    {
                        render_context_frames(ctx_handle, &mut mix_buf);
                        let pushed = shared.ring.push(&mix_buf);
                        if pushed == 0 {
                            break;
                        }
                        pumped = true;
                    }
                    if !pumped || shared.ring.len() >= target_frames / 2 {
                        std::thread::sleep(Duration::from_millis(4));
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            quit,
            thread: Some(thread),
            stream: Some(stream),
        })
    }

    /// Tear down the mixer thread and CoreAudio/Null output stream.
    /// Must NEVER be called while holding `STATE`.
    fn stop(mut self) {
        self.quit.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        drop(self.stream.take());
    }
}

impl Drop for MixerRuntime {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        drop(self.stream.take());
    }
}

struct Device {
    info: DeviceInfo,
    last_error: i32,
    buffers: HashMap<u32, Arc<BufferData>>,
    contexts: Vec<usize>,
    output_rate: u32,
    refresh: i32,
    sync: bool,
    mono_sources: i32,
    stereo_sources: i32,
}

impl Device {
    fn new(info: DeviceInfo) -> Self {
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

    fn set_error(&mut self, err: i32) {
        if self.last_error == ALC_NO_ERROR {
            self.last_error = err;
        }
    }
}

struct Context {
    device: usize,
    output_rate: u32,
    refresh: i32,
    sync: bool,
    mono_sources: i32,
    stereo_sources: i32,
    suspended: bool,
    last_error: i32,
    listener: Listener,
    distance_model: i32,
    source_distance_model_enabled: bool,
    doppler_factor: f32,
    doppler_velocity: f32,
    speed_of_sound: f32,
    sources: HashMap<u32, Source>,
    mixer: Option<MixerRuntime>,
}

impl Context {
    fn new(
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

    fn set_error(&mut self, err: i32) {
        if self.last_error == AL_NO_ERROR {
            self.last_error = err;
        }
    }

    fn has_playing_source(&self) -> bool {
        self.sources.values().any(|s| s.state == AL_PLAYING)
    }
}

struct OpenAlState {
    next_handle: usize,
    next_al_id: u32,
    devices: HashMap<usize, Device>,
    contexts: HashMap<usize, Context>,
    current_context: Option<usize>,
    global_alc_error: i32,
    global_al_error: i32,
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

    fn alloc_handle(&mut self) -> usize {
        let h = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1).max(0xA100_0001);
        h
    }

    fn alloc_al_id(&mut self) -> u32 {
        let id = self.next_al_id;
        self.next_al_id = self.next_al_id.wrapping_add(1);
        if self.next_al_id == 0 {
            self.next_al_id = 1;
        }
        id
    }

    fn set_alc_error(&mut self, device: usize, err: i32) {
        if let Some(dev) = self.devices.get_mut(&device) {
            dev.set_error(err);
        } else if self.global_alc_error == ALC_NO_ERROR {
            self.global_alc_error = err;
        }
    }

    fn set_al_error(&mut self, err: i32) {
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

    fn current_ctx_mut(&mut self) -> Option<&mut Context> {
        let cid = self.current_context?;
        self.contexts.get_mut(&cid)
    }

    fn buffer_in_use_on_device(&self, dev_handle: usize, bid: u32, active_only: bool) -> bool {
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

fn lock_state() -> std::sync::MutexGuard<'static, OpenAlState> {
    state().lock().unwrap_or_else(|e| e.into_inner())
}

/// Lazily open & start the host audio mixer for `ctx_handle` if needed.
/// Must be called WITHOUT holding `STATE`.
fn ensure_mixer_running(ctx_handle: usize) {
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
// 3D Spatialization, Distance Attenuation & Stereo Software Mixer
// ---------------------------------------------------------------------------

#[inline]
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn len3(v: [f32; 3]) -> f32 {
    dot3(v, v).sqrt()
}

fn normalize3(v: [f32; 3], fallback: [f32; 3]) -> [f32; 3] {
    let l = len3(v);
    if l > 1e-6 && l.is_finite() {
        [v[0] / l, v[1] / l, v[2] / l]
    } else {
        fallback
    }
}

fn distance_attenuation(model: i32, distance: f32, ref_dist: f32, max_dist: f32, rolloff: f32) -> f32 {
    let ref_d = ref_dist.max(0.0);
    let max_d = max_dist.max(ref_d);
    let ro = rolloff.max(0.0);
    match model {
        AL_NONE => 1.0,
        AL_INVERSE_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let denom = ref_d + ro * (distance - ref_d);
                if denom <= 1e-6 {
                    1e6
                } else {
                    ref_d / denom
                }
            }
        }
        AL_INVERSE_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                let denom = ref_d + ro * (d - ref_d);
                if denom <= 1e-6 {
                    1.0
                } else {
                    ref_d / denom
                }
            }
        }
        AL_LINEAR_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else {
                let d = distance.min(max_d);
                let span = max_d - ref_d;
                if span <= 1e-6 {
                    if d <= ref_d { 1.0 } else { 0.0 }
                } else {
                    (1.0 - ro * (d - ref_d) / span).max(0.0)
                }
            }
        }
        AL_LINEAR_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                let span = max_d - ref_d;
                if span <= 1e-6 {
                    1.0
                } else {
                    (1.0 - ro * (d - ref_d) / span).max(0.0)
                }
            }
        }
        AL_EXPONENT_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else if distance <= 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                (distance / ref_d).powf(-ro)
            }
        }
        AL_EXPONENT_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                (d / ref_d).powf(-ro)
            }
        }
        _ => 1.0,
    }
}

fn cone_attenuation(src: &Source, rel: [f32; 3], distance: f32) -> f32 {
    if src.cone_inner_angle >= 360.0 || distance <= 1e-6 {
        return 1.0;
    }
    let dir_len = len3(src.direction);
    if dir_len <= 1e-6 {
        return 1.0;
    }
    let dir = [
        src.direction[0] / dir_len,
        src.direction[1] / dir_len,
        src.direction[2] / dir_len,
    ];
    // Vector from source to listener = -rel / distance.
    let s_to_l = [-rel[0] / distance, -rel[1] / distance, -rel[2] / distance];
    let angle_deg = clamp_f32(dot3(s_to_l, dir), -1.0, 1.0).acos().to_degrees() * 2.0;
    if angle_deg <= src.cone_inner_angle {
        1.0
    } else if angle_deg >= src.cone_outer_angle {
        src.cone_outer_gain
    } else {
        let span = (src.cone_outer_angle - src.cone_inner_angle).max(1e-6);
        let t = (angle_deg - src.cone_inner_angle) / span;
        1.0 + t * (src.cone_outer_gain - 1.0)
    }
}

fn compute_doppler_pitch(
    listener: &Listener,
    src: &Source,
    rel: [f32; 3],
    distance: f32,
    doppler_factor: f32,
    speed_of_sound: f32,
) -> f64 {
    if doppler_factor <= 0.0 || speed_of_sound <= 0.0 || distance <= 1e-6 {
        return 1.0;
    }
    if len3(src.velocity) <= 1e-6 && len3(listener.velocity) <= 1e-6 {
        return 1.0;
    }
    // SL = unit vector from source to listener = -rel / distance.
    let sl = [-rel[0] / distance, -rel[1] / distance, -rel[2] / distance];
    let max_v = speed_of_sound / doppler_factor;
    let vls = dot3(sl, listener.velocity).clamp(-max_v * 0.95, max_v * 0.95);
    let vss = dot3(sl, src.velocity).clamp(-max_v * 0.95, max_v * 0.95);
    let num = speed_of_sound - doppler_factor * vls;
    let den = speed_of_sound - doppler_factor * vss;
    if den.abs() <= 1e-6 {
        return 1.0;
    }
    ((num / den) as f64).clamp(0.25, 4.0)
}

/// Compute `(mono_left_gain, mono_right_gain, stereo_gain, doppler_pitch)` for a source.
fn compute_spatial_params(
    listener: &Listener,
    distance_model: i32,
    doppler_factor: f32,
    speed_of_sound: f32,
    src: &Source,
) -> (f32, f32, f32, f64) {
    let min_g = src.min_gain.min(src.max_gain);
    let max_g = src.max_gain;
    let stereo_gain = clamp_f32(src.gain, min_g, max_g) * listener.gain;

    let (rel, right) = if src.source_relative {
        (src.position, [1.0f32, 0.0, 0.0])
    } else {
        let r = [
            src.position[0] - listener.position[0],
            src.position[1] - listener.position[1],
            src.position[2] - listener.position[2],
        ];
        let fwd = normalize3(
            [
                listener.orientation[0],
                listener.orientation[1],
                listener.orientation[2],
            ],
            [0.0, 0.0, -1.0],
        );
        let up = normalize3(
            [
                listener.orientation[3],
                listener.orientation[4],
                listener.orientation[5],
            ],
            [0.0, 1.0, 0.0],
        );
        let right = normalize3(cross3(fwd, up), [1.0, 0.0, 0.0]);
        (r, right)
    };

    let dist = len3(rel);
    let att = distance_attenuation(
        distance_model,
        dist,
        src.reference_distance,
        src.max_distance,
        src.rolloff_factor,
    );
    let cone = cone_attenuation(src, rel, dist);
    let mono_gain = clamp_f32(src.gain * att * cone, min_g, max_g) * listener.gain;

    let pan = if dist > 1e-6 {
        let dir = [rel[0] / dist, rel[1] / dist, rel[2] / dist];
        clamp_f32(dot3(dir, right), -1.0, 1.0)
    } else {
        0.0
    };
    // Constant-power stereo pan: pan=-1 -> (1, 0), pan=0 -> (0.707, 0.707), pan=+1 -> (0, 1).
    let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
    let left_gain = mono_gain * angle.cos();
    let right_gain = mono_gain * angle.sin();

    let doppler = if src.source_relative {
        1.0
    } else {
        compute_doppler_pitch(listener, src, rel, dist, doppler_factor, speed_of_sound)
    };

    (left_gain, right_gain, stereo_gain, doppler)
}

/// Mix `out_interleaved.len() / 2` stereo float32 frames for `ctx_handle`.
/// Called both by the host mixer thread and synchronously in deterministic tests.
fn render_context_frames(ctx_handle: usize, out_interleaved: &mut [f32]) {
    out_interleaved.fill(0.0);
    let num_frames = out_interleaved.len() / 2;
    if num_frames == 0 {
        return;
    }

    let mut st = lock_state();
    let Some(ctx_ref) = st.contexts.get(&ctx_handle) else {
        return;
    };
    if ctx_ref.suspended {
        return;
    }
    let dev_handle = ctx_ref.device;
    let output_rate = ctx_ref.output_rate.max(1) as f64;
    let listener = ctx_ref.listener.clone();
    let distance_model = ctx_ref.distance_model;
    let doppler_factor = ctx_ref.doppler_factor;
    let speed_of_sound = ctx_ref.speed_of_sound;

    // Clone Arc<BufferData> table references so we can mutably borrow `st.contexts`.
    let buffers: HashMap<u32, Arc<BufferData>> = match st.devices.get(&dev_handle) {
        Some(dev) => dev.buffers.clone(),
        None => return,
    };

    let Some(ctx) = st.contexts.get_mut(&ctx_handle) else {
        return;
    };

    for src in ctx.sources.values_mut() {
        if src.state != AL_PLAYING {
            continue;
        }
        if src.queue.is_empty() {
            src.state = AL_STOPPED;
            src.processed = 0;
            src.frame_pos = 0.0;
            continue;
        }

        let (mono_l, mono_r, stereo_gain, doppler) =
            compute_spatial_params(&listener, distance_model, doppler_factor, speed_of_sound, src);

        for frame_idx in 0..num_frames {
            // Skip any 0-frame or missing buffers safely (bounded to avoid infinite loop).
            let mut skips = 0usize;
            let cur_buf = loop {
                if src.processed >= src.queue.len() {
                    if src.looping && !src.queue.is_empty() && skips <= src.queue.len() {
                        src.processed = 0;
                    } else {
                        src.state = AL_STOPPED;
                        src.frame_pos = 0.0;
                        break None;
                    }
                }
                let bid = src.queue[src.processed];
                if let Some(b) = buffers.get(&bid) {
                    if b.frames() > 0 {
                        break Some(b);
                    }
                }
                skips += 1;
                if skips > src.queue.len() {
                    src.state = AL_STOPPED;
                    src.processed = src.queue.len();
                    src.frame_pos = 0.0;
                    break None;
                }
                src.processed += 1;
                src.frame_pos = 0.0;
            };

            let Some(buf) = cur_buf else {
                break;
            };

            let total_frames = buf.frames();
            let pos = src.frame_pos.max(0.0);
            let idx0 = (pos.floor() as usize).min(total_frames - 1);
            let frac = (pos - idx0 as f64).clamp(0.0, 1.0) as f32;

            let next_sample = |ch_stereo: bool| -> (f32, f32) {
                if idx0 + 1 < total_frames {
                    if ch_stereo {
                        buf.sample_stereo(idx0 + 1)
                    } else {
                        let s = buf.sample_mono(idx0 + 1);
                        (s, s)
                    }
                } else if src.source_type == AL_STATIC && src.looping {
                    if ch_stereo {
                        buf.sample_stereo(0)
                    } else {
                        let s = buf.sample_mono(0);
                        (s, s)
                    }
                } else if src.source_type == AL_STREAMING {
                    let next_idx = if src.processed + 1 < src.queue.len() {
                        Some(src.processed + 1)
                    } else if src.looping && !src.queue.is_empty() {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(ni) = next_idx {
                        if let Some(nb) = buffers.get(&src.queue[ni]) {
                            if nb.frames() > 0 {
                                return if ch_stereo {
                                    nb.sample_stereo(0)
                                } else {
                                    let s = nb.sample_mono(0);
                                    (s, s)
                                };
                            }
                        }
                    }
                    if ch_stereo {
                        buf.sample_stereo(idx0)
                    } else {
                        let s = buf.sample_mono(idx0);
                        (s, s)
                    }
                } else if ch_stereo {
                    buf.sample_stereo(idx0)
                } else {
                    let s = buf.sample_mono(idx0);
                    (s, s)
                }
            };

            if buf.channels == 1 {
                let s0 = buf.sample_mono(idx0);
                let (s1, _) = next_sample(false);
                let s = s0 + (s1 - s0) * frac;
                out_interleaved[frame_idx * 2] += s * mono_l;
                out_interleaved[frame_idx * 2 + 1] += s * mono_r;
            } else {
                let (l0, r0) = buf.sample_stereo(idx0);
                let (l1, r1) = next_sample(true);
                let sl = l0 + (l1 - l0) * frac;
                let sr = r0 + (r1 - r0) * frac;
                out_interleaved[frame_idx * 2] += sl * stereo_gain;
                out_interleaved[frame_idx * 2 + 1] += sr * stereo_gain;
            }

            let eff_doppler = if buf.channels == 1 { doppler } else { 1.0 };
            let step = ((buf.freq as f64 / output_rate) * (src.pitch as f64) * eff_doppler)
                .max(1e-6);
            src.frame_pos += step;

            // Advance across buffer boundary (handles pitch > 1 skipping multiple frames).
            let mut cur_total = total_frames as f64;
            let mut adv_guard = 0usize;
            while src.frame_pos >= cur_total {
                if src.source_type == AL_STATIC {
                    if src.looping {
                        src.frame_pos %= cur_total;
                        break;
                    } else {
                        src.state = AL_STOPPED;
                        src.processed = src.queue.len();
                        src.frame_pos = 0.0;
                        break;
                    }
                } else {
                    src.frame_pos -= cur_total;
                    src.processed += 1;
                    if src.processed >= src.queue.len() {
                        if src.looping && !src.queue.is_empty() {
                            src.processed = 0;
                        } else {
                            src.state = AL_STOPPED;
                            src.frame_pos = 0.0;
                            break;
                        }
                    }
                    adv_guard += 1;
                    if adv_guard > src.queue.len() + 1 {
                        src.state = AL_STOPPED;
                        src.frame_pos = 0.0;
                        break;
                    }
                    let next_bid = src.queue[src.processed];
                    cur_total = buffers.get(&next_bid).map(|b| b.frames()).unwrap_or(0) as f64;
                    if cur_total <= 0.0 {
                        cur_total = 1.0;
                        src.frame_pos = 1.0;
                    }
                }
            }
            if src.state != AL_PLAYING {
                break;
            }
        }
    }

    for s in out_interleaved.iter_mut() {
        *s = clamp_f32(*s, -1.0, 1.0);
    }
}

// ---------------------------------------------------------------------------
// ALC 1.1 API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alcOpenDevice(devicename: *const u8) -> usize {
    let devs = backend::output_devices();
    let chosen = match crate::strings::from_ansi_opt(devicename) {
        None => devs.into_iter().next(),
        Some(ref s) if s.is_empty() => devs.into_iter().next(),
        Some(want) => {
            let lw = want.to_lowercase();
            let exact = devs
                .iter()
                .position(|d| d.name == want || d.uid == want)
                .or_else(|| devs.iter().position(|d| d.name.to_lowercase() == lw))
                .or_else(|| devs.iter().position(|d| d.name.to_lowercase().contains(&lw)));
            if let Some(idx) = exact {
                Some(devs.into_iter().nth(idx).unwrap())
            } else if matches!(
                lw.as_str(),
                "generic software"
                    | "generic hardware"
                    | "directsound"
                    | "directsound3d"
                    | "mmsystem"
                    | "openal soft"
                    | "openal soft on default"
            ) {
                devs.into_iter().next()
            } else {
                None
            }
        }
    };

    let Some(info) = chosen else {
        lock_state().set_alc_error(0, ALC_INVALID_VALUE);
        return 0;
    };

    log::info!(
        target: "audio",
        "openal32: alcOpenDevice -> '{}' ({} Hz, {} ch)",
        info.name,
        info.rate,
        info.channels
    );
    let mut st = lock_state();
    let handle = st.alloc_handle();
    st.devices.insert(handle, Device::new(info));
    handle
}

pub unsafe extern "win64" fn alcCloseDevice(device: usize) -> u8 {
    let mixers_to_stop: Vec<MixerRuntime> = {
        let mut st = lock_state();
        let Some(dev) = st.devices.remove(&device) else {
            st.set_alc_error(0, ALC_INVALID_DEVICE);
            return ALC_FALSE;
        };
        let mut mixers = Vec::new();
        for cid in dev.contexts {
            if st.current_context == Some(cid) {
                st.current_context = None;
            }
            if let Some(mut ctx) = st.contexts.remove(&cid) {
                if let Some(m) = ctx.mixer.take() {
                    mixers.push(m);
                }
            }
        }
        mixers
    };
    for m in mixers_to_stop {
        m.stop();
    }
    ALC_TRUE
}

pub unsafe extern "win64" fn alcCreateContext(device: usize, attrlist: *const i32) -> usize {
    let mut st = lock_state();
    let Some(dev) = st.devices.get_mut(&device) else {
        st.set_alc_error(0, ALC_INVALID_DEVICE);
        return 0;
    };

    let mut output_rate = DEFAULT_OUTPUT_RATE;
    let mut refresh = dev.refresh;
    let mut sync = false;
    let mut mono_sources = dev.mono_sources;
    let mut stereo_sources = dev.stereo_sources;

    if !attrlist.is_null() {
        let mut idx = 0usize;
        while idx < 128 {
            let key = rd(attrlist.add(idx));
            if key == 0 {
                break;
            }
            let val = rd(attrlist.add(idx + 1));
            idx += 2;
            match key {
                ALC_FREQUENCY => {
                    if val <= 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    output_rate = (val as u32).clamp(8_000, 192_000);
                }
                ALC_REFRESH => {
                    if val <= 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    refresh = val;
                }
                ALC_SYNC => {
                    sync = val != 0;
                }
                ALC_MONO_SOURCES => {
                    if val < 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    mono_sources = val;
                }
                ALC_STEREO_SOURCES => {
                    if val < 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    stereo_sources = val;
                }
                _ => {}
            }
        }
    }

    dev.output_rate = output_rate;
    dev.refresh = refresh;
    dev.sync = sync;
    dev.mono_sources = mono_sources;
    dev.stereo_sources = stereo_sources;

    let ctx_handle = st.alloc_handle();
    if let Some(dev) = st.devices.get_mut(&device) {
        dev.contexts.push(ctx_handle);
    }
    st.contexts.insert(
        ctx_handle,
        Context::new(
            device,
            output_rate,
            refresh,
            sync,
            mono_sources,
            stereo_sources,
        ),
    );
    ctx_handle
}

pub unsafe extern "win64" fn alcMakeContextCurrent(context: usize) -> u8 {
    let mut st = lock_state();
    if context == 0 {
        st.current_context = None;
        return ALC_TRUE;
    }
    if st.contexts.contains_key(&context) {
        st.current_context = Some(context);
        ALC_TRUE
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
        ALC_FALSE
    }
}

pub unsafe extern "win64" fn alcProcessContext(context: usize) {
    let should_start = {
        let mut st = lock_state();
        let Some(ctx) = st.contexts.get_mut(&context) else {
            st.set_alc_error(0, ALC_INVALID_CONTEXT);
            return;
        };
        ctx.suspended = false;
        !ctx.sync && ctx.has_playing_source() && ctx.mixer.is_none()
    };
    if should_start {
        ensure_mixer_running(context);
    }
}

pub unsafe extern "win64" fn alcSuspendContext(context: usize) {
    let mut st = lock_state();
    if let Some(ctx) = st.contexts.get_mut(&context) {
        ctx.suspended = true;
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
    }
}

pub unsafe extern "win64" fn alcDestroyContext(context: usize) {
    let mixer_to_stop = {
        let mut st = lock_state();
        let Some(mut ctx) = st.contexts.remove(&context) else {
            st.set_alc_error(0, ALC_INVALID_CONTEXT);
            return;
        };
        if st.current_context == Some(context) {
            st.current_context = None;
        }
        if let Some(dev) = st.devices.get_mut(&ctx.device) {
            dev.contexts.retain(|&c| c != context);
        }
        ctx.mixer.take()
    };
    if let Some(m) = mixer_to_stop {
        m.stop();
    }
}

pub unsafe extern "win64" fn alcGetCurrentContext() -> usize {
    lock_state().current_context.unwrap_or(0)
}

pub unsafe extern "win64" fn alcGetContextsDevice(context: usize) -> usize {
    let mut st = lock_state();
    if let Some(ctx) = st.contexts.get(&context) {
        ctx.device
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
        0
    }
}

pub unsafe extern "win64" fn alcGetError(device: usize) -> i32 {
    let mut st = lock_state();
    if device == 0 {
        let e = st.global_alc_error;
        st.global_alc_error = ALC_NO_ERROR;
        return e;
    }
    if let Some(dev) = st.devices.get_mut(&device) {
        let e = dev.last_error;
        dev.last_error = ALC_NO_ERROR;
        e
    } else {
        ALC_INVALID_DEVICE
    }
}

pub unsafe extern "win64" fn alcGetString(device: usize, param: i32) -> *const u8 {
    match param {
        ALC_NO_ERROR => b"No Error\0".as_ptr(),
        ALC_INVALID_DEVICE => b"Invalid Device\0".as_ptr(),
        ALC_INVALID_CONTEXT => b"Invalid Context\0".as_ptr(),
        ALC_INVALID_ENUM => b"Invalid Enum\0".as_ptr(),
        ALC_INVALID_VALUE => b"Invalid Value\0".as_ptr(),
        ALC_OUT_OF_MEMORY => b"Out of Memory\0".as_ptr(),
        ALC_DEFAULT_DEVICE_SPECIFIER | ALC_DEFAULT_ALL_DEVICES_SPECIFIER => {
            let devs = backend::output_devices();
            let name = devs
                .first()
                .map(|d| d.name.as_str())
                .unwrap_or("CoreAudio Default");
            intern_cstr(name)
        }
        ALC_DEVICE_SPECIFIER | ALC_ALL_DEVICES_SPECIFIER => {
            if device == 0 {
                let devs = backend::output_devices();
                intern_cstr_list(&devs)
            } else {
                let mut st = lock_state();
                if let Some(dev) = st.devices.get(&device) {
                    let name = dev.info.name.clone();
                    drop(st);
                    intern_cstr(&name)
                } else {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    std::ptr::null()
                }
            }
        }
        ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER => {
            let devs = backend::input_devices();
            let name = devs
                .first()
                .map(|d| d.name.as_str())
                .unwrap_or("Default Microphone");
            intern_cstr(name)
        }
        ALC_CAPTURE_DEVICE_SPECIFIER => {
            if device == 0 {
                let devs = backend::input_devices();
                intern_cstr_list(&devs)
            } else {
                let mut st = lock_state();
                if st.devices.contains_key(&device) {
                    drop(st);
                    let devs = backend::input_devices();
                    let name = devs
                        .first()
                        .map(|d| d.name.as_str())
                        .unwrap_or("Default Microphone");
                    intern_cstr(name)
                } else {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    std::ptr::null()
                }
            }
        }
        ALC_EXTENSIONS => {
            if device != 0 {
                let mut st = lock_state();
                if !st.devices.contains_key(&device) {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    return std::ptr::null();
                }
            }
            b"ALC_ENUMERATION_EXT ALC_ENUMERATE_ALL_EXT\0".as_ptr()
        }
        _ => {
            lock_state().set_alc_error(device, ALC_INVALID_ENUM);
            std::ptr::null()
        }
    }
}

pub unsafe extern "win64" fn alcGetIntegerv(
    device: usize,
    param: i32,
    size: i32,
    values: *mut i32,
) {
    if values.is_null() || size <= 0 {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return;
    }

    match param {
        ALC_MAJOR_VERSION => {
            wr(values, 1);
            return;
        }
        ALC_MINOR_VERSION => {
            wr(values, 1);
            return;
        }
        _ => {}
    }

    let mut st = lock_state();
    if device != 0 && !st.devices.contains_key(&device) {
        st.set_alc_error(0, ALC_INVALID_DEVICE);
        return;
    }

    // Resolve attribute source: current context on device -> last context on device -> device defaults.
    let (freq, refresh, sync, mono, stereo) = if let Some(dev) = st.devices.get(&device) {
        let active_ctx = st
            .current_context
            .filter(|cid| dev.contexts.contains(cid))
            .or_else(|| dev.contexts.last().copied());
        if let Some(ctx) = active_ctx.and_then(|cid| st.contexts.get(&cid)) {
            (
                ctx.output_rate as i32,
                ctx.refresh,
                if ctx.sync { 1 } else { 0 },
                ctx.mono_sources,
                ctx.stereo_sources,
            )
        } else {
            (
                dev.output_rate as i32,
                dev.refresh,
                if dev.sync { 1 } else { 0 },
                dev.mono_sources,
                dev.stereo_sources,
            )
        }
    } else {
        (
            DEFAULT_OUTPUT_RATE as i32,
            DEFAULT_REFRESH,
            0,
            DEFAULT_MONO_SOURCES,
            DEFAULT_STEREO_SOURCES,
        )
    };

    match param {
        ALC_FREQUENCY => wr(values, freq),
        ALC_REFRESH => wr(values, refresh),
        ALC_SYNC => wr(values, sync),
        ALC_MONO_SOURCES => wr(values, mono),
        ALC_STEREO_SOURCES => wr(values, stereo),
        ALC_CAPTURE_SAMPLES => wr(values, 0),
        ALC_ATTRIBUTES_SIZE => {
            if device == 0 {
                st.set_alc_error(0, ALC_INVALID_DEVICE);
                return;
            }
            wr(values, 11);
        }
        ALC_ALL_ATTRIBUTES => {
            if device == 0 {
                st.set_alc_error(0, ALC_INVALID_DEVICE);
                return;
            }
            let attrs: [i32; 11] = [
                ALC_FREQUENCY,
                freq,
                ALC_REFRESH,
                refresh,
                ALC_SYNC,
                sync,
                ALC_MONO_SOURCES,
                mono,
                ALC_STEREO_SOURCES,
                stereo,
                0,
            ];
            if (size as usize) < attrs.len() {
                st.set_alc_error(device, ALC_INVALID_VALUE);
                return;
            }
            for (i, &v) in attrs.iter().enumerate() {
                wr(values.add(i), v);
            }
        }
        _ => st.set_alc_error(device, ALC_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alcIsExtensionPresent(device: usize, extname: *const u8) -> u8 {
    let Some(name) = crate::strings::from_ansi_opt(extname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return ALC_FALSE;
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return ALC_FALSE;
    }
    if name.eq_ignore_ascii_case("ALC_ENUMERATION_EXT")
        || name.eq_ignore_ascii_case("ALC_ENUMERATE_ALL_EXT")
    {
        ALC_TRUE
    } else {
        ALC_FALSE
    }
}

pub unsafe extern "win64" fn alcGetProcAddress(
    device: usize,
    funcname: *const u8,
) -> *const c_void {
    let Some(name) = crate::strings::from_ansi_opt(funcname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return std::ptr::null();
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return std::ptr::null();
    }
    if let Some(ptr) = crate::lookup("openal32.dll", &name) {
        let addr = ptr as usize;
        crate::register_native_va(addr as u64, addr);
        ptr as *const c_void
    } else {
        std::ptr::null()
    }
}

pub unsafe extern "win64" fn alcGetEnumValue(device: usize, enumname: *const u8) -> i32 {
    let Some(name) = crate::strings::from_ansi_opt(enumname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return 0;
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return 0;
    }
    match name.as_str() {
        "ALC_FALSE" => ALC_FALSE as i32,
        "ALC_TRUE" => ALC_TRUE as i32,
        "ALC_NO_ERROR" => ALC_NO_ERROR,
        "ALC_INVALID_DEVICE" => ALC_INVALID_DEVICE,
        "ALC_INVALID_CONTEXT" => ALC_INVALID_CONTEXT,
        "ALC_INVALID_ENUM" => ALC_INVALID_ENUM,
        "ALC_INVALID_VALUE" => ALC_INVALID_VALUE,
        "ALC_OUT_OF_MEMORY" => ALC_OUT_OF_MEMORY,
        "ALC_MAJOR_VERSION" => ALC_MAJOR_VERSION,
        "ALC_MINOR_VERSION" => ALC_MINOR_VERSION,
        "ALC_ATTRIBUTES_SIZE" => ALC_ATTRIBUTES_SIZE,
        "ALC_ALL_ATTRIBUTES" => ALC_ALL_ATTRIBUTES,
        "ALC_DEFAULT_DEVICE_SPECIFIER" => ALC_DEFAULT_DEVICE_SPECIFIER,
        "ALC_DEVICE_SPECIFIER" => ALC_DEVICE_SPECIFIER,
        "ALC_EXTENSIONS" => ALC_EXTENSIONS,
        "ALC_FREQUENCY" => ALC_FREQUENCY,
        "ALC_REFRESH" => ALC_REFRESH,
        "ALC_SYNC" => ALC_SYNC,
        "ALC_MONO_SOURCES" => ALC_MONO_SOURCES,
        "ALC_STEREO_SOURCES" => ALC_STEREO_SOURCES,
        "ALC_DEFAULT_ALL_DEVICES_SPECIFIER" => ALC_DEFAULT_ALL_DEVICES_SPECIFIER,
        "ALC_ALL_DEVICES_SPECIFIER" => ALC_ALL_DEVICES_SPECIFIER,
        "ALC_CAPTURE_DEVICE_SPECIFIER" => ALC_CAPTURE_DEVICE_SPECIFIER,
        "ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER" => ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER,
        "ALC_CAPTURE_SAMPLES" => ALC_CAPTURE_SAMPLES,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Buffer API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alGenBuffers(n: i32, buffers: *mut u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let mut ids = Vec::with_capacity(n as usize);
    for _ in 0..(n as usize) {
        ids.push(st.alloc_al_id());
    }
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        for (i, id) in ids.into_iter().enumerate() {
            dev.buffers.insert(id, Arc::new(BufferData::default()));
            wr(buffers.add(i), id);
        }
    }
}

pub unsafe extern "win64" fn alDeleteBuffers(n: i32, buffers: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let mut ids = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        ids.push(rd(buffers.add(i)));
    }
    // Atomic validation pass (OpenAL 1.1 spec): verify every ID before removing any.
    for &bid in &ids {
        if bid == 0 {
            continue;
        }
        let exists = st
            .devices
            .get(&dev_handle)
            .is_some_and(|d| d.buffers.contains_key(&bid));
        if !exists {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
        if st.buffer_in_use_on_device(dev_handle, bid, false) {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        }
    }
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        for bid in ids {
            if bid != 0 {
                dev.buffers.remove(&bid);
            }
        }
    }
}

pub unsafe extern "win64" fn alIsBuffer(buffer: u32) -> u8 {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    if buffer == 0 {
        // OpenAL 1.1 spec: alIsBuffer(0) returns AL_TRUE (NULL buffer).
        return AL_TRUE;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        return AL_FALSE;
    };
    if st
        .devices
        .get(&dev_handle)
        .is_some_and(|d| d.buffers.contains_key(&buffer))
    {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alBufferData(
    buffer: u32,
    format: i32,
    data: *const u8,
    size: i32,
    freq: i32,
) {
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
    if st.buffer_in_use_on_device(dev_handle, buffer, true) {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    }
    let (channels, bits, frame_bytes): (u16, u16, usize) = match format {
        AL_FORMAT_MONO8 => (1, 8, 1),
        AL_FORMAT_MONO16 => (1, 16, 2),
        AL_FORMAT_STEREO8 => (2, 8, 2),
        AL_FORMAT_STEREO16 => (2, 16, 4),
        AL_FORMAT_MONO_FLOAT32 => (1, 32, 4),
        AL_FORMAT_STEREO_FLOAT32 => (2, 32, 8),
        _ => {
            st.set_al_error(AL_INVALID_ENUM);
            return;
        }
    };
    if size < 0 || freq <= 0 || (size > 0 && data.is_null()) || (size as usize) % frame_bytes != 0 {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }

    let byte_len = size as usize;
    let raw: &[u8] = if byte_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(data, byte_len)
    };

    let samples: Vec<f32> = match format {
        AL_FORMAT_MONO8 | AL_FORMAT_STEREO8 => {
            raw.iter().map(|&b| (b as f32 - 128.0) / 128.0).collect()
        }
        AL_FORMAT_MONO16 | AL_FORMAT_STEREO16 => raw
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0)
            .collect(),
        AL_FORMAT_MONO_FLOAT32 | AL_FORMAT_STEREO_FLOAT32 => raw
            .chunks_exact(4)
            .map(|c| {
                let v = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                if v.is_finite() { v } else { 0.0 }
            })
            .collect(),
        _ => unreachable!(),
    };

    let buf = Arc::new(BufferData {
        samples,
        freq: freq as u32,
        channels,
        bits,
        size_bytes: size,
    });
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        dev.buffers.insert(buffer, buf);
    }
}

fn buffer_set_invalid(buffer: u32, _param: i32, ptr_null: bool) {
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

pub unsafe extern "win64" fn alBufferi(buffer: u32, param: i32, _value: i32) {
    buffer_set_invalid(buffer, param, false);
}

pub unsafe extern "win64" fn alBuffer3i(
    buffer: u32,
    param: i32,
    _v1: i32,
    _v2: i32,
    _v3: i32,
) {
    buffer_set_invalid(buffer, param, false);
}

pub unsafe extern "win64" fn alBufferiv(buffer: u32, param: i32, values: *const i32) {
    buffer_set_invalid(buffer, param, values.is_null());
}

pub unsafe extern "win64" fn alBufferfv(buffer: u32, param: i32, values: *const f32) {
    buffer_set_invalid(buffer, param, values.is_null());
}

unsafe extern "win64" fn al_buffer_f(buffer: u32, param: i32, _value: f32) {
    buffer_set_invalid(buffer, param, false);
}
crate::win64_bridge!(fn al_buffer_f(u32, i32, f32) -> ());

unsafe extern "win64" fn al_buffer_3f(
    buffer: u32,
    param: i32,
    _v1: f32,
    _v2: f32,
    _v3: f32,
) {
    buffer_set_invalid(buffer, param, false);
}
crate::win64_bridge!(fn al_buffer_3f(u32, i32, f32, f32, f32) -> ());

fn get_buffer_prop(buffer: u32, param: i32) -> Result<i32, i32> {
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

pub unsafe extern "win64" fn alGetBufferi(buffer: u32, param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_buffer_prop(buffer, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetBufferiv(buffer: u32, param: i32, values: *mut i32) {
    alGetBufferi(buffer, param, values);
}

pub unsafe extern "win64" fn alGetBuffer3i(
    buffer: u32,
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    buffer_set_invalid(buffer, param, v1.is_null() || v2.is_null() || v3.is_null());
}

pub unsafe extern "win64" fn alGetBufferf(buffer: u32, param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_buffer_prop(buffer, param) {
        Ok(v) => wr(value, v as f32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetBufferfv(buffer: u32, param: i32, values: *mut f32) {
    alGetBufferf(buffer, param, values);
}

pub unsafe extern "win64" fn alGetBuffer3f(
    buffer: u32,
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    buffer_set_invalid(buffer, param, v1.is_null() || v2.is_null() || v3.is_null());
}

// ---------------------------------------------------------------------------
// AL 1.1 Source API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alGenSources(n: i32, sources: *mut u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || sources.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut ids = Vec::with_capacity(n as usize);
    for _ in 0..(n as usize) {
        ids.push(st.alloc_al_id());
    }
    if let Some(ctx) = st.contexts.get_mut(&cid) {
        for (i, id) in ids.into_iter().enumerate() {
            ctx.sources.insert(id, Source::default());
            wr(sources.add(i), id);
        }
    }
}

pub unsafe extern "win64" fn alDeleteSources(n: i32, sources: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || sources.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut ids = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        ids.push(rd(sources.add(i)));
    }
    let Some(ctx) = st.contexts.get_mut(&cid) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    for &sid in &ids {
        if !ctx.sources.contains_key(&sid) {
            ctx.set_error(AL_INVALID_NAME);
            return;
        }
    }
    for sid in ids {
        ctx.sources.remove(&sid);
    }
}

pub unsafe extern "win64" fn alIsSource(source: u32) -> u8 {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    if st
        .contexts
        .get(&cid)
        .is_some_and(|c| c.sources.contains_key(&source))
    {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alSourcePlay(source: u32) {
    let start_ctx = {
        let mut st = lock_state();
        let Some(cid) = st.current_context else {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        };
        let Some(ctx) = st.contexts.get_mut(&cid) else {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        };
        let Some(src) = ctx.sources.get_mut(&source) else {
            ctx.set_error(AL_INVALID_NAME);
            return;
        };
        src.play();
        if src.state == AL_PLAYING && !ctx.sync && !ctx.suspended && ctx.mixer.is_none() {
            Some(cid)
        } else {
            None
        }
    };
    if let Some(cid) = start_ctx {
        ensure_mixer_running(cid);
    }
}

pub unsafe extern "win64" fn alSourcePause(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.pause();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

pub unsafe extern "win64" fn alSourceStop(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.stop();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

pub unsafe extern "win64" fn alSourceRewind(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.rewind();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

unsafe fn source_batch_op(n: i32, sources: *const u32, op: fn(&mut Source)) -> Option<usize> {
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

pub unsafe extern "win64" fn alSourcePlayv(n: i32, sources: *const u32) {
    if let Some(cid) = source_batch_op(n, sources, Source::play) {
        ensure_mixer_running(cid);
    }
}

pub unsafe extern "win64" fn alSourcePausev(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::pause);
}

pub unsafe extern "win64" fn alSourceStopv(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::stop);
}

pub unsafe extern "win64" fn alSourceRewindv(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::rewind);
}

pub unsafe extern "win64" fn alSourceQueueBuffers(source: u32, nb: i32, buffers: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if nb == 0 {
        return;
    }
    if nb < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };

    let mut bids = Vec::with_capacity(nb as usize);
    for i in 0..(nb as usize) {
        bids.push(rd(buffers.add(i)));
    }

    // Validate source exists and is not AL_STATIC.
    let src_type = match st.contexts.get(&cid).and_then(|c| c.sources.get(&source)) {
        Some(s) => s.source_type,
        None => {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
    };
    if src_type == AL_STATIC {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    }

    // Validate every non-zero buffer ID exists on this device.
    for &bid in &bids {
        if bid != 0
            && !st
                .devices
                .get(&dev_handle)
                .is_some_and(|d| d.buffers.contains_key(&bid))
        {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
    }

    if let Some(src) = st
        .contexts
        .get_mut(&cid)
        .and_then(|c| c.sources.get_mut(&source))
    {
        src.source_type = AL_STREAMING;
        for bid in bids {
            src.queue.push_back(bid);
        }
    }
}

pub unsafe extern "win64" fn alSourceUnqueueBuffers(source: u32, nb: i32, buffers: *mut u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if nb == 0 {
        return;
    }
    if nb < 0 || buffers.is_null() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    let Some(src) = ctx.sources.get_mut(&source) else {
        ctx.set_error(AL_INVALID_NAME);
        return;
    };
    if src.source_type != AL_STREAMING {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    let count = nb as usize;
    let available = src.buffers_processed() as usize;
    if count > available || count > src.queue.len() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    for i in 0..count {
        let bid = src.queue.pop_front().unwrap_or(0);
        wr(buffers.add(i), bid);
    }
    src.processed = src.processed.saturating_sub(count);
}

fn source_set_int_prop(source: u32, param: i32, value: i32) {
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

fn source_set_float_prop(source: u32, param: i32, value: f32) {
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

fn source_set_vec3_prop(source: u32, param: i32, v1: f32, v2: f32, v3: f32) {
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

pub unsafe extern "win64" fn alSourcei(source: u32, param: i32, value: i32) {
    source_set_int_prop(source, param, value);
}

pub unsafe extern "win64" fn alSource3i(source: u32, param: i32, v1: i32, v2: i32, v3: i32) {
    source_set_vec3_prop(source, param, v1 as f32, v2 as f32, v3 as f32);
}

pub unsafe extern "win64" fn alSourceiv(source: u32, param: i32, values: *const i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => {
            let v1 = rd(values) as f32;
            let v2 = rd(values.add(1)) as f32;
            let v3 = rd(values.add(2)) as f32;
            source_set_vec3_prop(source, param, v1, v2, v3);
        }
        _ => source_set_int_prop(source, param, rd(values)),
    }
}

pub unsafe extern "win64" fn alSourcefv(source: u32, param: i32, values: *const f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => {
            let v1 = rd(values);
            let v2 = rd(values.add(1));
            let v3 = rd(values.add(2));
            source_set_vec3_prop(source, param, v1, v2, v3);
        }
        _ => source_set_float_prop(source, param, rd(values)),
    }
}

unsafe extern "win64" fn al_source_f(source: u32, param: i32, value: f32) {
    source_set_float_prop(source, param, value);
}
crate::win64_bridge!(fn al_source_f(u32, i32, f32) -> ());

unsafe extern "win64" fn al_source_3f(source: u32, param: i32, v1: f32, v2: f32, v3: f32) {
    source_set_vec3_prop(source, param, v1, v2, v3);
}
crate::win64_bridge!(fn al_source_3f(u32, i32, f32, f32, f32) -> ());

fn get_source_int(source: u32, param: i32) -> Result<i32, i32> {
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

fn get_source_float(source: u32, param: i32) -> Result<f32, i32> {
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

fn get_source_vec3(source: u32, param: i32) -> Result<[f32; 3], i32> {
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

pub unsafe extern "win64" fn alGetSourcei(source: u32, param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_int(source, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSource3i(
    source: u32,
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_vec3(source, param) {
        Ok(v) => {
            wr(v1, v[0].round() as i32);
            wr(v2, v[1].round() as i32);
            wr(v3, v[2].round() as i32);
        }
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSourceiv(source: u32, param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => match get_source_vec3(source, param) {
            Ok(v) => {
                wr(values, v[0].round() as i32);
                wr(values.add(1), v[1].round() as i32);
                wr(values.add(2), v[2].round() as i32);
            }
            Err(e) => lock_state().set_al_error(e),
        },
        _ => alGetSourcei(source, param, values),
    }
}

pub unsafe extern "win64" fn alGetSourcef(source: u32, param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_float(source, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSource3f(
    source: u32,
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_vec3(source, param) {
        Ok(v) => {
            wr(v1, v[0]);
            wr(v2, v[1]);
            wr(v3, v[2]);
        }
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSourcefv(source: u32, param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => match get_source_vec3(source, param) {
            Ok(v) => {
                wr(values, v[0]);
                wr(values.add(1), v[1]);
                wr(values.add(2), v[2]);
            }
            Err(e) => lock_state().set_al_error(e),
        },
        _ => alGetSourcef(source, param, values),
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Listener API Exports
// ---------------------------------------------------------------------------

fn listener_set_float(param: i32, value: f32) {
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

fn listener_set_vec3(param: i32, v1: f32, v2: f32, v3: f32) {
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

fn listener_set_orientation(vals: [f32; 6]) {
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

pub unsafe extern "win64" fn alListeneri(param: i32, value: i32) {
    listener_set_float(param, value as f32);
}

pub unsafe extern "win64" fn alListener3i(param: i32, v1: i32, v2: i32, v3: i32) {
    listener_set_vec3(param, v1 as f32, v2 as f32, v3 as f32);
}

pub unsafe extern "win64" fn alListeneriv(param: i32, values: *const i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY => {
            listener_set_vec3(
                param,
                rd(values) as f32,
                rd(values.add(1)) as f32,
                rd(values.add(2)) as f32,
            );
        }
        AL_ORIENTATION => {
            listener_set_orientation([
                rd(values) as f32,
                rd(values.add(1)) as f32,
                rd(values.add(2)) as f32,
                rd(values.add(3)) as f32,
                rd(values.add(4)) as f32,
                rd(values.add(5)) as f32,
            ]);
        }
        _ => listener_set_float(param, rd(values) as f32),
    }
}

pub unsafe extern "win64" fn alListenerfv(param: i32, values: *const f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_GAIN => listener_set_float(param, rd(values)),
        AL_POSITION | AL_VELOCITY => {
            listener_set_vec3(param, rd(values), rd(values.add(1)), rd(values.add(2)));
        }
        AL_ORIENTATION => {
            listener_set_orientation([
                rd(values),
                rd(values.add(1)),
                rd(values.add(2)),
                rd(values.add(3)),
                rd(values.add(4)),
                rd(values.add(5)),
            ]);
        }
        _ => {
            let mut st = lock_state();
            if let Some(ctx) = st.current_ctx_mut() {
                ctx.set_error(AL_INVALID_ENUM);
            } else {
                st.set_al_error(AL_INVALID_OPERATION);
            }
        }
    }
}

unsafe extern "win64" fn al_listener_f(param: i32, value: f32) {
    listener_set_float(param, value);
}
crate::win64_bridge!(fn al_listener_f(i32, f32) -> ());

unsafe extern "win64" fn al_listener_3f(param: i32, v1: f32, v2: f32, v3: f32) {
    listener_set_vec3(param, v1, v2, v3);
}
crate::win64_bridge!(fn al_listener_3f(i32, f32, f32, f32) -> ());

pub unsafe extern "win64" fn alGetListenerf(param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(value, ctx.listener.gain),
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListener3f(
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let v = match param {
        AL_POSITION => ctx.listener.position,
        AL_VELOCITY => ctx.listener.velocity,
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            return;
        }
    };
    wr(v1, v[0]);
    wr(v2, v[1]);
    wr(v3, v[2]);
}

pub unsafe extern "win64" fn alGetListenerfv(param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(values, ctx.listener.gain),
        AL_POSITION => {
            for (i, &x) in ctx.listener.position.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        AL_VELOCITY => {
            for (i, &x) in ctx.listener.velocity.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        AL_ORIENTATION => {
            for (i, &x) in ctx.listener.orientation.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListeneri(param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(value, ctx.listener.gain.round() as i32),
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListener3i(
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let v = match param {
        AL_POSITION => ctx.listener.position,
        AL_VELOCITY => ctx.listener.velocity,
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            return;
        }
    };
    wr(v1, v[0].round() as i32);
    wr(v2, v[1].round() as i32);
    wr(v3, v[2].round() as i32);
}

pub unsafe extern "win64" fn alGetListeneriv(param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(values, ctx.listener.gain.round() as i32),
        AL_POSITION => {
            for (i, &x) in ctx.listener.position.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        AL_VELOCITY => {
            for (i, &x) in ctx.listener.velocity.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        AL_ORIENTATION => {
            for (i, &x) in ctx.listener.orientation.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Global State, Distance Model, Errors & Introspection Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alDistanceModel(value: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match value {
        AL_NONE
        | AL_INVERSE_DISTANCE
        | AL_INVERSE_DISTANCE_CLAMPED
        | AL_LINEAR_DISTANCE
        | AL_LINEAR_DISTANCE_CLAMPED
        | AL_EXPONENT_DISTANCE
        | AL_EXPONENT_DISTANCE_CLAMPED => ctx.distance_model = value,
        _ => ctx.set_error(AL_INVALID_VALUE),
    }
}

pub unsafe extern "win64" fn alDopplerFactor(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value < 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.doppler_factor = value;
    }
}

pub unsafe extern "win64" fn alDopplerVelocity(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value <= 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.doppler_velocity = value;
    }
}

pub unsafe extern "win64" fn alSpeedOfSound(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value <= 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.speed_of_sound = value;
    }
}

pub unsafe extern "win64" fn alGetError() -> i32 {
    let mut st = lock_state();
    if let Some(cid) = st.current_context {
        if let Some(ctx) = st.contexts.get_mut(&cid) {
            let e = ctx.last_error;
            ctx.last_error = AL_NO_ERROR;
            return e;
        }
    }
    let e = st.global_al_error;
    st.global_al_error = AL_NO_ERROR;
    e
}

pub unsafe extern "win64" fn alGetString(param: i32) -> *const u8 {
    match param {
        AL_NO_ERROR => b"No Error\0".as_ptr(),
        AL_INVALID_NAME => b"Invalid Name\0".as_ptr(),
        AL_INVALID_ENUM => b"Invalid Enum\0".as_ptr(),
        AL_INVALID_VALUE => b"Invalid Value\0".as_ptr(),
        AL_INVALID_OPERATION => b"Invalid Operation\0".as_ptr(),
        AL_OUT_OF_MEMORY => b"Out of Memory\0".as_ptr(),
        AL_VENDOR => b"PortFactory OpenAL\0".as_ptr(),
        AL_VERSION => b"1.1 CoreAudio\0".as_ptr(),
        AL_RENDERER => b"CoreAudio Software Mixer\0".as_ptr(),
        AL_EXTENSIONS => b"AL_EXT_FLOAT32\0".as_ptr(),
        _ => {
            lock_state().set_al_error(AL_INVALID_ENUM);
            std::ptr::null()
        }
    }
}

fn get_global_param_f64(param: i32) -> Result<f64, i32> {
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

pub unsafe extern "win64" fn alGetBoolean(param: i32) -> u8 {
    match get_global_param_f64(param) {
        Ok(v) => {
            if v != 0.0 {
                AL_TRUE
            } else {
                AL_FALSE
            }
        }
        Err(e) => {
            lock_state().set_al_error(e);
            AL_FALSE
        }
    }
}

pub unsafe extern "win64" fn alGetInteger(param: i32) -> i32 {
    match get_global_param_f64(param) {
        Ok(v) => v as i32,
        Err(e) => {
            lock_state().set_al_error(e);
            0
        }
    }
}

pub unsafe extern "win64" fn alGetFloat(param: i32) -> f32 {
    match get_global_param_f64(param) {
        Ok(v) => v as f32,
        Err(e) => {
            lock_state().set_al_error(e);
            0.0
        }
    }
}

pub unsafe extern "win64" fn alGetDouble(param: i32) -> f64 {
    match get_global_param_f64(param) {
        Ok(v) => v,
        Err(e) => {
            lock_state().set_al_error(e);
            0.0
        }
    }
}

pub unsafe extern "win64" fn alGetBooleanv(param: i32, values: *mut u8) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, if v != 0.0 { AL_TRUE } else { AL_FALSE }),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetIntegerv(param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v as i32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetFloatv(param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v as f32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetDoublev(param: i32, values: *mut f64) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alEnable(capability: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => ctx.source_distance_model_enabled = true,
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alDisable(capability: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => ctx.source_distance_model_enabled = false,
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alIsEnabled(capability: i32) -> u8 {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => {
            if ctx.source_distance_model_enabled {
                AL_TRUE
            } else {
                AL_FALSE
            }
        }
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            AL_FALSE
        }
    }
}

pub unsafe extern "win64" fn alIsExtensionPresent(extname: *const u8) -> u8 {
    let Some(name) = crate::strings::from_ansi_opt(extname) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return AL_FALSE;
    };
    if name.eq_ignore_ascii_case("AL_EXT_FLOAT32") {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alGetProcAddress(fname: *const u8) -> *const c_void {
    let Some(name) = crate::strings::from_ansi_opt(fname) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return std::ptr::null();
    };
    if let Some(ptr) = crate::lookup("openal32.dll", &name) {
        let addr = ptr as usize;
        crate::register_native_va(addr as u64, addr);
        ptr as *const c_void
    } else {
        std::ptr::null()
    }
}

pub unsafe extern "win64" fn alGetEnumValue(ename: *const u8) -> i32 {
    let Some(name) = crate::strings::from_ansi_opt(ename) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return AL_NONE;
    };
    match name.as_str() {
        "AL_NONE" => AL_NONE,
        "AL_FALSE" => AL_FALSE as i32,
        "AL_TRUE" => AL_TRUE as i32,
        "AL_SOURCE_RELATIVE" => AL_SOURCE_RELATIVE,
        "AL_CONE_INNER_ANGLE" => AL_CONE_INNER_ANGLE,
        "AL_CONE_OUTER_ANGLE" => AL_CONE_OUTER_ANGLE,
        "AL_PITCH" => AL_PITCH,
        "AL_POSITION" => AL_POSITION,
        "AL_DIRECTION" => AL_DIRECTION,
        "AL_VELOCITY" => AL_VELOCITY,
        "AL_LOOPING" => AL_LOOPING,
        "AL_BUFFER" => AL_BUFFER,
        "AL_GAIN" => AL_GAIN,
        "AL_MIN_GAIN" => AL_MIN_GAIN,
        "AL_MAX_GAIN" => AL_MAX_GAIN,
        "AL_ORIENTATION" => AL_ORIENTATION,
        "AL_SOURCE_STATE" => AL_SOURCE_STATE,
        "AL_INITIAL" => AL_INITIAL,
        "AL_PLAYING" => AL_PLAYING,
        "AL_PAUSED" => AL_PAUSED,
        "AL_STOPPED" => AL_STOPPED,
        "AL_BUFFERS_QUEUED" => AL_BUFFERS_QUEUED,
        "AL_BUFFERS_PROCESSED" => AL_BUFFERS_PROCESSED,
        "AL_REFERENCE_DISTANCE" => AL_REFERENCE_DISTANCE,
        "AL_ROLLOFF_FACTOR" => AL_ROLLOFF_FACTOR,
        "AL_CONE_OUTER_GAIN" => AL_CONE_OUTER_GAIN,
        "AL_MAX_DISTANCE" => AL_MAX_DISTANCE,
        "AL_SEC_OFFSET" => AL_SEC_OFFSET,
        "AL_SAMPLE_OFFSET" => AL_SAMPLE_OFFSET,
        "AL_BYTE_OFFSET" => AL_BYTE_OFFSET,
        "AL_SOURCE_TYPE" => AL_SOURCE_TYPE,
        "AL_STATIC" => AL_STATIC,
        "AL_STREAMING" => AL_STREAMING,
        "AL_UNDETERMINED" => AL_UNDETERMINED,
        "AL_FORMAT_MONO8" => AL_FORMAT_MONO8,
        "AL_FORMAT_MONO16" => AL_FORMAT_MONO16,
        "AL_FORMAT_STEREO8" => AL_FORMAT_STEREO8,
        "AL_FORMAT_STEREO16" => AL_FORMAT_STEREO16,
        "AL_FORMAT_MONO_FLOAT32" => AL_FORMAT_MONO_FLOAT32,
        "AL_FORMAT_STEREO_FLOAT32" => AL_FORMAT_STEREO_FLOAT32,
        "AL_FREQUENCY" => AL_FREQUENCY,
        "AL_BITS" => AL_BITS,
        "AL_CHANNELS" => AL_CHANNELS,
        "AL_SIZE" => AL_SIZE,
        "AL_NO_ERROR" => AL_NO_ERROR,
        "AL_INVALID_NAME" => AL_INVALID_NAME,
        "AL_INVALID_ENUM" => AL_INVALID_ENUM,
        "AL_INVALID_VALUE" => AL_INVALID_VALUE,
        "AL_INVALID_OPERATION" => AL_INVALID_OPERATION,
        "AL_OUT_OF_MEMORY" => AL_OUT_OF_MEMORY,
        "AL_VENDOR" => AL_VENDOR,
        "AL_VERSION" => AL_VERSION,
        "AL_RENDERER" => AL_RENDERER,
        "AL_EXTENSIONS" => AL_EXTENSIONS,
        "AL_DOPPLER_FACTOR" => AL_DOPPLER_FACTOR,
        "AL_DOPPLER_VELOCITY" => AL_DOPPLER_VELOCITY,
        "AL_SPEED_OF_SOUND" => AL_SPEED_OF_SOUND,
        "AL_DISTANCE_MODEL" => AL_DISTANCE_MODEL,
        "AL_INVERSE_DISTANCE" => AL_INVERSE_DISTANCE,
        "AL_INVERSE_DISTANCE_CLAMPED" => AL_INVERSE_DISTANCE_CLAMPED,
        "AL_LINEAR_DISTANCE" => AL_LINEAR_DISTANCE,
        "AL_LINEAR_DISTANCE_CLAMPED" => AL_LINEAR_DISTANCE_CLAMPED,
        "AL_EXPONENT_DISTANCE" => AL_EXPONENT_DISTANCE,
        "AL_EXPONENT_DISTANCE_CLAMPED" => AL_EXPONENT_DISTANCE_CLAMPED,
        "AL_SOURCE_DISTANCE_MODEL" => AL_SOURCE_DISTANCE_MODEL,
        _ => AL_NONE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::backend::Backend;
    use std::time::Instant;

    /// Guard that serializes OpenAL unit tests and forces the Null audio backend
    /// so tests never touch physical CoreAudio hardware or race on `current_context`.
    struct TestEnv {
        _audio_guard: crate::audio::AudioTestGuard,
        dev: usize,
        ctx: usize,
    }

    impl TestEnv {
        fn new(sync: bool) -> Self {
            let audio_guard = crate::audio::audio_test_lock();
            backend::force_backend(Some(Backend::Null));
            unsafe {
                let dev = alcOpenDevice(std::ptr::null());
                assert_ne!(dev, 0, "alcOpenDevice failed");
                let attrs = [
                    ALC_FREQUENCY,
                    48_000,
                    ALC_SYNC,
                    if sync { 1 } else { 0 },
                    0,
                ];
                let ctx = alcCreateContext(dev, attrs.as_ptr());
                assert_ne!(ctx, 0, "alcCreateContext failed");
                assert_eq!(alcMakeContextCurrent(ctx), ALC_TRUE);
                // Clear any prior error state.
                let _ = alGetError();
                let _ = alcGetError(dev);
                Self {
                    _audio_guard: audio_guard,
                    dev,
                    ctx,
                }
            }
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            unsafe {
                alcMakeContextCurrent(0);
                alcDestroyContext(self.ctx);
                alcCloseDevice(self.dev);
            }
        }
    }

    fn read_cstr_lossy(p: *const u8) -> String {
        assert!(!p.is_null());
        unsafe { crate::strings::from_ansi(p) }
    }

    #[test]
    fn openal_device_context_lifecycle_and_queries() {
        let env = TestEnv::new(true);
        unsafe {
            assert_eq!(alcGetCurrentContext(), env.ctx);
            assert_eq!(alcGetContextsDevice(env.ctx), env.dev);

            // Real device name from crate::audio::backend (Null backend in test).
            let def_name = read_cstr_lossy(alcGetString(0, ALC_DEFAULT_DEVICE_SPECIFIER));
            assert!(!def_name.is_empty());
            let dev_name = read_cstr_lossy(alcGetString(env.dev, ALC_DEVICE_SPECIFIER));
            assert_eq!(dev_name, def_name);

            // Double-NUL-terminated device list when device == 0.
            let list_ptr = alcGetString(0, ALC_DEVICE_SPECIFIER);
            assert!(!list_ptr.is_null());
            let first = read_cstr_lossy(list_ptr);
            assert_eq!(first, def_name);
            let mut off = 0usize;
            while *list_ptr.add(off) != 0 {
                let s = read_cstr_lossy(list_ptr.add(off));
                off += s.len() + 1;
            }
            assert_eq!(*list_ptr.add(off), 0);

            // ALC integer queries.
            let mut major = 0;
            let mut minor = 0;
            let mut freq = 0;
            let mut mono = 0;
            let mut stereo = 0;
            let mut sync = 0;
            let mut attr_sz = 0;
            alcGetIntegerv(env.dev, ALC_MAJOR_VERSION, 1, &mut major);
            alcGetIntegerv(env.dev, ALC_MINOR_VERSION, 1, &mut minor);
            alcGetIntegerv(env.dev, ALC_FREQUENCY, 1, &mut freq);
            alcGetIntegerv(env.dev, ALC_MONO_SOURCES, 1, &mut mono);
            alcGetIntegerv(env.dev, ALC_STEREO_SOURCES, 1, &mut stereo);
            alcGetIntegerv(env.dev, ALC_SYNC, 1, &mut sync);
            alcGetIntegerv(env.dev, ALC_ATTRIBUTES_SIZE, 1, &mut attr_sz);
            assert_eq!(major, 1);
            assert_eq!(minor, 1);
            assert_eq!(freq, 48_000);
            assert!(mono >= 64);
            assert!(stereo >= 16);
            assert_eq!(sync, 1);
            assert_eq!(attr_sz, 11);

            let mut all_attrs = [0i32; 11];
            alcGetIntegerv(env.dev, ALC_ALL_ATTRIBUTES, 11, all_attrs.as_mut_ptr());
            assert_eq!(all_attrs[0], ALC_FREQUENCY);
            assert_eq!(all_attrs[1], 48_000);
            assert_eq!(all_attrs[10], 0);

            // Extensions, strings, proc addresses, enum values.
            assert_eq!(
                alcIsExtensionPresent(env.dev, b"ALC_ENUMERATION_EXT\0".as_ptr()),
                ALC_TRUE
            );
            assert_eq!(
                alcIsExtensionPresent(env.dev, b"ALC_ENUMERATE_ALL_EXT\0".as_ptr()),
                ALC_TRUE
            );
            assert_eq!(
                alcIsExtensionPresent(env.dev, b"ALC_NONEXISTENT\0".as_ptr()),
                ALC_FALSE
            );
            assert_eq!(
                alIsExtensionPresent(b"AL_EXT_FLOAT32\0".as_ptr()),
                AL_TRUE
            );
            assert!(!alcGetProcAddress(env.dev, b"alcOpenDevice\0".as_ptr()).is_null());
            assert!(!alGetProcAddress(b"alSource3f\0".as_ptr()).is_null());
            assert!(!alGetProcAddress(b"alSourcePlay\0".as_ptr()).is_null());
            assert!(alGetProcAddress(b"alDoesNotExist\0".as_ptr()).is_null());
            assert_eq!(
                alGetEnumValue(b"AL_FORMAT_STEREO_FLOAT32\0".as_ptr()),
                AL_FORMAT_STEREO_FLOAT32
            );
            assert!(read_cstr_lossy(alGetString(AL_VERSION)).contains("1.1"));
            assert_eq!(alcGetError(env.dev), ALC_NO_ERROR);
            assert_eq!(alGetError(), AL_NO_ERROR);
        }
    }

    #[test]
    fn openal_buffer_upload_formats_and_queries() {
        let env = TestEnv::new(true);
        unsafe {
            let mut bufs = [0u32; 4];
            alGenBuffers(4, bufs.as_mut_ptr());
            assert_eq!(alGetError(), AL_NO_ERROR);
            for &b in &bufs {
                assert_ne!(b, 0);
                assert_eq!(alIsBuffer(b), AL_TRUE);
            }

            // 1. MONO8
            let mono8: [u8; 4] = [0, 128, 192, 255];
            alBufferData(bufs[0], AL_FORMAT_MONO8, mono8.as_ptr(), 4, 22_050);

            // 2. MONO16
            let mono16_vals: [i16; 3] = [-32768, 0, 16384];
            let mut mono16_bytes = Vec::new();
            for s in mono16_vals {
                mono16_bytes.extend_from_slice(&s.to_le_bytes());
            }
            alBufferData(
                bufs[1],
                AL_FORMAT_MONO16,
                mono16_bytes.as_ptr(),
                mono16_bytes.len() as i32,
                44_100,
            );

            // 3. STEREO16
            let stereo16_vals: [i16; 4] = [16384, -16384, 8192, -8192];
            let mut stereo16_bytes = Vec::new();
            for s in stereo16_vals {
                stereo16_bytes.extend_from_slice(&s.to_le_bytes());
            }
            alBufferData(
                bufs[2],
                AL_FORMAT_STEREO16,
                stereo16_bytes.as_ptr(),
                stereo16_bytes.len() as i32,
                48_000,
            );

            // 4. STEREO_FLOAT32
            let stereo_f32: [f32; 4] = [0.25, -0.75, 0.5, -0.5];
            let mut stereo_f32_bytes = Vec::new();
            for s in stereo_f32 {
                stereo_f32_bytes.extend_from_slice(&s.to_le_bytes());
            }
            alBufferData(
                bufs[3],
                AL_FORMAT_STEREO_FLOAT32,
                stereo_f32_bytes.as_ptr(),
                stereo_f32_bytes.len() as i32,
                48_000,
            );
            assert_eq!(alGetError(), AL_NO_ERROR);

            // Query properties via i, iv, f, fv.
            let (mut f_i, mut bits, mut ch, mut sz) = (0, 0, 0, 0);
            alGetBufferi(bufs[1], AL_FREQUENCY, &mut f_i);
            alGetBufferiv(bufs[1], AL_BITS, &mut bits);
            alGetBufferi(bufs[1], AL_CHANNELS, &mut ch);
            alGetBufferi(bufs[1], AL_SIZE, &mut sz);
            assert_eq!((f_i, bits, ch, sz), (44_100, 16, 1, 6));

            let mut f_f = 0.0f32;
            let mut sz_f = 0.0f32;
            alGetBufferf(bufs[3], AL_FREQUENCY, &mut f_f);
            alGetBufferfv(bufs[3], AL_SIZE, &mut sz_f);
            assert_eq!(f_f, 48_000.0);
            assert_eq!(sz_f, 16.0);

            // Verify decoded float32 samples directly in device buffer store.
            {
                let st = lock_state();
                let dev = st.devices.get(&env.dev).unwrap();
                let b0 = dev.buffers.get(&bufs[0]).unwrap();
                assert!((b0.samples[0] - (-1.0)).abs() < 1e-5);
                assert!((b0.samples[1] - 0.0).abs() < 1e-5);
                assert!((b0.samples[2] - 0.5).abs() < 1e-5);

                let b1 = dev.buffers.get(&bufs[1]).unwrap();
                assert!((b1.samples[0] - (-1.0)).abs() < 1e-5);
                assert!((b1.samples[1] - 0.0).abs() < 1e-5);
                assert!((b1.samples[2] - 0.5).abs() < 1e-5);

                let b2 = dev.buffers.get(&bufs[2]).unwrap();
                assert_eq!(b2.channels, 2);
                assert!((b2.samples[0] - 0.5).abs() < 1e-5);
                assert!((b2.samples[1] - (-0.5)).abs() < 1e-5);

                let b3 = dev.buffers.get(&bufs[3]).unwrap();
                assert_eq!(b3.samples, stereo_f32);
            }

            alDeleteBuffers(4, bufs.as_ptr());
            assert_eq!(alGetError(), AL_NO_ERROR);
            assert_eq!(alIsBuffer(bufs[0]), AL_FALSE);
        }
    }

    #[test]
    fn openal_mixer_play_pause_stop_rewind_looping_and_pitch() {
        let env = TestEnv::new(true);
        unsafe {
            let mut buf = 0u32;
            let mut src = 0u32;
            alGenBuffers(1, &mut buf);
            alGenSources(1, &mut src);
            assert_eq!(alIsSource(src), AL_TRUE);

            // 8 stereo float32 frames: ramp 0.1, 0.2, ..., 0.8 on L, -0.1, ..., -0.8 on R.
            let mut pcm = Vec::new();
            for i in 1..=8 {
                let v = i as f32 * 0.1;
                pcm.extend_from_slice(&v.to_le_bytes());
                pcm.extend_from_slice(&(-v).to_le_bytes());
            }
            alBufferData(
                buf,
                AL_FORMAT_STEREO_FLOAT32,
                pcm.as_ptr(),
                pcm.len() as i32,
                48_000,
            );
            alSourcei(src, AL_BUFFER, buf as i32);

            let mut state_val = 0;
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_INITIAL);

            // Play first 4 frames.
            alSourcePlay(src);
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_PLAYING);

            let mut out = [0.0f32; 8]; // 4 stereo frames
            render_context_frames(env.ctx, &mut out);
            assert!((out[0] - 0.1).abs() < 1e-5);
            assert!((out[1] - (-0.1)).abs() < 1e-5);
            assert!((out[6] - 0.4).abs() < 1e-5);
            assert!((out[7] - (-0.4)).abs() < 1e-5);

            // Pause: must output silence and preserve sample offset = 4.
            alSourcePause(src);
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_PAUSED);
            let mut offset = 0;
            let mut byte_off = 0;
            let mut sec_off = 0.0f32;
            alGetSourcei(src, AL_SAMPLE_OFFSET, &mut offset);
            alGetSourcei(src, AL_BYTE_OFFSET, &mut byte_off);
            alGetSourcef(src, AL_SEC_OFFSET, &mut sec_off);
            assert_eq!(offset, 4);
            assert_eq!(byte_off, 4 * 8); // 4 stereo float32 frames = 32 bytes
            assert!((sec_off - (4.0 / 48_000.0)).abs() < 1e-6);

            render_context_frames(env.ctx, &mut out);
            assert!(out.iter().all(|&x| x == 0.0));

            // Suspend / process context gates mixer output cleanly.
            alSourcePlay(src);
            alcSuspendContext(env.ctx);
            render_context_frames(env.ctx, &mut out);
            assert!(out.iter().all(|&x| x == 0.0));
            alcProcessContext(env.ctx);

            // Finish remaining 4 frames + 2 trailing silent frames.
            let mut out6 = [0.0f32; 12]; // 6 stereo frames
            render_context_frames(env.ctx, &mut out6);
            assert!((out6[0] - 0.5).abs() < 1e-5);
            assert!((out6[6] - 0.8).abs() < 1e-5);
            assert_eq!(out6[8], 0.0);
            assert_eq!(out6[10], 0.0);
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_STOPPED);

            // Rewind -> AL_INITIAL, then enable looping and render 12 frames (wraps past 8).
            alSourceRewind(src);
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_INITIAL);
            alSourcei(src, AL_LOOPING, 1);
            alSourcePlayv(1, &src);

            let mut loop_out = [0.0f32; 24]; // 12 stereo frames
            render_context_frames(env.ctx, &mut loop_out);
            // Frame 8 wraps back to sample 0 (0.1, -0.1).
            assert!((loop_out[16] - 0.1).abs() < 1e-5);
            assert!((loop_out[17] - (-0.1)).abs() < 1e-5);
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_PLAYING);

            // Pitch = 2.0: samples advance 2 frames per output frame.
            alSourceStopv(1, &src);
            alSourcei(src, AL_LOOPING, 0);
            al_source_f(src, AL_PITCH, 2.0);
            alSourcePlay(src);
            let mut pitch_out = [0.0f32; 10]; // 5 stereo frames (buffer finishes after 4)
            render_context_frames(env.ctx, &mut pitch_out);
            assert!((pitch_out[0] - 0.1).abs() < 1e-5); // frame 0
            assert!((pitch_out[2] - 0.3).abs() < 1e-5); // frame 2
            assert!((pitch_out[4] - 0.5).abs() < 1e-5); // frame 4
            assert!((pitch_out[6] - 0.7).abs() < 1e-5); // frame 6
            assert_eq!(pitch_out[8], 0.0); // finished
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            assert_eq!(state_val, AL_STOPPED);
            assert_eq!(alGetError(), AL_NO_ERROR);
        }
    }

    #[test]
    fn openal_streaming_queue_and_unqueue() {
        let env = TestEnv::new(true);
        unsafe {
            let mut bufs = [0u32; 3];
            let mut src = 0u32;
            alGenBuffers(3, bufs.as_mut_ptr());
            alGenSources(1, &mut src);

            // Three 4-frame stereo float32 buffers with distinct DC levels 0.25, 0.50, 0.75.
            for (i, &val) in [0.25f32, 0.50, 0.75].iter().enumerate() {
                let mut bytes = Vec::new();
                for _ in 0..4 {
                    bytes.extend_from_slice(&val.to_le_bytes());
                    bytes.extend_from_slice(&val.to_le_bytes());
                }
                alBufferData(
                    bufs[i],
                    AL_FORMAT_STEREO_FLOAT32,
                    bytes.as_ptr(),
                    bytes.len() as i32,
                    48_000,
                );
            }

            alSourceQueueBuffers(src, 3, bufs.as_ptr());
            assert_eq!(alGetError(), AL_NO_ERROR);

            let (mut stype, mut queued, mut processed) = (0, 0, 0);
            alGetSourcei(src, AL_SOURCE_TYPE, &mut stype);
            alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
            alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
            assert_eq!(stype, AL_STREAMING);
            assert_eq!(queued, 3);
            assert_eq!(processed, 0);

            // Play and consume exactly buffer 0 (4 frames).
            alSourcePlay(src);
            let mut out4 = [0.0f32; 8];
            render_context_frames(env.ctx, &mut out4);
            assert!(out4.iter().all(|&x| (x - 0.25).abs() < 1e-5));

            alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
            assert_eq!(processed, 1);

            // Unqueuing 2 when only 1 is processed must fail with AL_INVALID_VALUE atomically.
            let mut unq2 = [0u32; 2];
            alSourceUnqueueBuffers(src, 2, unq2.as_mut_ptr());
            assert_eq!(alGetError(), AL_INVALID_VALUE);
            alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
            assert_eq!(queued, 3);

            // Unqueue 1 processed buffer -> must return bufs[0].
            let mut unq1 = 0u32;
            alSourceUnqueueBuffers(src, 1, &mut unq1);
            assert_eq!(alGetError(), AL_NO_ERROR);
            assert_eq!(unq1, bufs[0]);
            alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
            alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
            assert_eq!((queued, processed), (2, 0));

            // Render 10 frames: 4 frames of 0.50, 4 frames of 0.75, 2 frames of silence.
            let mut out10 = [0.0f32; 20];
            render_context_frames(env.ctx, &mut out10);
            for i in 0..8 {
                assert!((out10[i] - 0.50).abs() < 1e-5);
            }
            for i in 8..16 {
                assert!((out10[i] - 0.75).abs() < 1e-5);
            }
            for i in 16..20 {
                assert_eq!(out10[i], 0.0);
            }

            let mut state_val = 0;
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
            assert_eq!(state_val, AL_STOPPED);
            assert_eq!(processed, 2);

            let mut rest = [0u32; 2];
            alSourceUnqueueBuffers(src, 2, rest.as_mut_ptr());
            assert_eq!(alGetError(), AL_NO_ERROR);
            assert_eq!(rest, [bufs[1], bufs[2]]);
            alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
            assert_eq!(queued, 0);
        }
    }

    #[test]
    fn openal_3d_panning_listener_and_distance_models() {
        let env = TestEnv::new(true);
        unsafe {
            let mut buf = 0u32;
            let mut src = 0u32;
            alGenBuffers(1, &mut buf);
            alGenSources(1, &mut src);

            // Constant 1.0 mono float32 buffer (looping).
            let mut bytes = Vec::new();
            for _ in 0..16 {
                bytes.extend_from_slice(&1.0f32.to_le_bytes());
            }
            alBufferData(
                buf,
                AL_FORMAT_MONO_FLOAT32,
                bytes.as_ptr(),
                bytes.len() as i32,
                48_000,
            );
            alSourcei(src, AL_BUFFER, buf as i32);
            alSourcei(src, AL_LOOPING, 1);

            // Disable distance attenuation first to verify pure 3D stereo panning.
            alDistanceModel(AL_NONE);
            assert_eq!(alGetInteger(AL_DISTANCE_MODEL), AL_NONE);

            // Listener at origin looking -Z with up +Y => right = +X.
            al_listener_3f(AL_POSITION, 0.0, 0.0, 0.0);
            let orient = [0.0f32, 0.0, -1.0, 0.0, 1.0, 0.0];
            alListenerfv(AL_ORIENTATION, orient.as_ptr());

            let mut got_orient = [0.0f32; 6];
            alGetListenerfv(AL_ORIENTATION, got_orient.as_mut_ptr());
            assert_eq!(got_orient, orient);

            // Source hard right (+10, 0, 0): L ~ 0, R ~ 1.
            al_source_3f(src, AL_POSITION, 10.0, 0.0, 0.0);
            alSourcePlay(src);
            let mut frame = [0.0f32; 2];
            render_context_frames(env.ctx, &mut frame);
            assert!(frame[0].abs() < 1e-4, "expected L~0, got {}", frame[0]);
            assert!((frame[1] - 1.0).abs() < 1e-4, "expected R~1, got {}", frame[1]);

            // Source hard left (-10, 0, 0): L ~ 1, R ~ 0.
            al_source_3f(src, AL_POSITION, -10.0, 0.0, 0.0);
            render_context_frames(env.ctx, &mut frame);
            assert!((frame[0] - 1.0).abs() < 1e-4, "expected L~1, got {}", frame[0]);
            assert!(frame[1].abs() < 1e-4, "expected R~0, got {}", frame[1]);

            // Rotate listener to face +X, up +Y => right = cross((1,0,0), (0,1,0)) = (0,0,+1).
            // Source at (0, 0, +10) should now pan hard right!
            let orient_x = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0];
            alListenerfv(AL_ORIENTATION, orient_x.as_ptr());
            al_source_3f(src, AL_POSITION, 0.0, 0.0, 10.0);
            render_context_frames(env.ctx, &mut frame);
            assert!(frame[0].abs() < 1e-4);
            assert!((frame[1] - 1.0).abs() < 1e-4);

            // Distance attenuation: center source ahead along +X at distance = 4.0.
            // Reset orientation to default (-Z forward, +Y up), place source at (0, 0, -4).
            alListenerfv(AL_ORIENTATION, orient.as_ptr());
            al_source_3f(src, AL_POSITION, 0.0, 0.0, -4.0);
            al_source_f(src, AL_REFERENCE_DISTANCE, 1.0);
            al_source_f(src, AL_ROLLOFF_FACTOR, 1.0);
            al_source_f(src, AL_MAX_DISTANCE, 10.0);

            // Inverse distance clamped at d=4, ref=1, rolloff=1 => gain = 1 / (1 + 1*(4-1)) = 0.25.
            // Centered pan multiplies by cos(pi/4) = FRAC_1_SQRT_2.
            alDistanceModel(AL_INVERSE_DISTANCE_CLAMPED);
            render_context_frames(env.ctx, &mut frame);
            let exp_inv = 0.25 * std::f32::consts::FRAC_1_SQRT_2;
            assert!((frame[0] - exp_inv).abs() < 1e-4);
            assert!((frame[1] - exp_inv).abs() < 1e-4);

            // Linear distance clamped at d=4, ref=1, max=10, rolloff=1 => 1 - (3/9) = 2/3.
            alDistanceModel(AL_LINEAR_DISTANCE_CLAMPED);
            render_context_frames(env.ctx, &mut frame);
            let exp_lin = (2.0 / 3.0) * std::f32::consts::FRAC_1_SQRT_2;
            assert!((frame[0] - exp_lin).abs() < 1e-4);

            // Exponent distance clamped at d=4, ref=1, rolloff=2 => (4/1)^(-2) = 1/16 = 0.0625.
            al_source_f(src, AL_ROLLOFF_FACTOR, 2.0);
            alDistanceModel(AL_EXPONENT_DISTANCE_CLAMPED);
            render_context_frames(env.ctx, &mut frame);
            let exp_pow = 0.0625 * std::f32::consts::FRAC_1_SQRT_2;
            assert!((frame[0] - exp_pow).abs() < 1e-4);

            // Listener gain halves output.
            al_listener_f(AL_GAIN, 0.5);
            render_context_frames(env.ctx, &mut frame);
            assert!((frame[0] - exp_pow * 0.5).abs() < 1e-4);

            // AL_MIN_GAIN / AL_MAX_GAIN clamping.
            al_source_f(src, AL_MIN_GAIN, 0.5);
            render_context_frames(env.ctx, &mut frame);
            let exp_min = 0.5 * 0.5 * std::f32::consts::FRAC_1_SQRT_2;
            assert!((frame[0] - exp_min).abs() < 1e-4);

            // AL_SOURCE_RELATIVE ignores listener offset/orientation.
            al_listener_f(AL_GAIN, 1.0);
            al_listener_3f(AL_POSITION, 100.0, 0.0, 0.0);
            al_source_f(src, AL_MIN_GAIN, 0.0);
            alDistanceModel(AL_NONE);
            alSourcei(src, AL_SOURCE_RELATIVE, 1);
            al_source_3f(src, AL_POSITION, 10.0, 0.0, 0.0);
            render_context_frames(env.ctx, &mut frame);
            assert!(frame[0].abs() < 1e-4);
            assert!((frame[1] - 1.0).abs() < 1e-4);

            // Global Doppler / speed of sound / enable-disable queries.
            alDopplerFactor(2.0);
            alSpeedOfSound(340.0);
            assert_eq!(alGetFloat(AL_DOPPLER_FACTOR), 2.0);
            assert_eq!(alGetFloat(AL_SPEED_OF_SOUND), 340.0);
            assert_eq!(alGetBoolean(AL_DOPPLER_FACTOR), AL_TRUE);
            alEnable(AL_SOURCE_DISTANCE_MODEL);
            assert_eq!(alIsEnabled(AL_SOURCE_DISTANCE_MODEL), AL_TRUE);
            alDisable(AL_SOURCE_DISTANCE_MODEL);
            assert_eq!(alIsEnabled(AL_SOURCE_DISTANCE_MODEL), AL_FALSE);

            assert_eq!(alGetError(), AL_NO_ERROR);
        }
    }

    #[test]
    fn openal_error_codes_and_validation() {
        let env = TestEnv::new(true);
        unsafe {
            // AL_INVALID_NAME on bad source/buffer ID.
            alSourcePlay(999_999);
            assert_eq!(alGetError(), AL_INVALID_NAME);
            assert_eq!(alGetError(), AL_NO_ERROR); // cleared on read

            // Sticky first error: only first error is kept until alGetError().
            alSourcePlay(999_999); // AL_INVALID_NAME
            alDistanceModel(0x1234_5678); // AL_INVALID_VALUE (ignored while error pending)
            assert_eq!(alGetError(), AL_INVALID_NAME);
            assert_eq!(alGetError(), AL_NO_ERROR);

            // AL_INVALID_ENUM on unknown format.
            let mut buf = 0u32;
            let mut src = 0u32;
            alGenBuffers(1, &mut buf);
            alGenSources(1, &mut src);
            let sample = [0u8; 4];
            alBufferData(buf, 0x9999, sample.as_ptr(), 4, 48_000);
            assert_eq!(alGetError(), AL_INVALID_ENUM);

            // AL_INVALID_VALUE on misaligned size for STEREO16 (must be multiple of 4).
            alBufferData(buf, AL_FORMAT_STEREO16, sample.as_ptr(), 3, 48_000);
            assert_eq!(alGetError(), AL_INVALID_VALUE);

            // AL_INVALID_VALUE on invalid pitch / negative gain / null output ptr.
            al_source_f(src, AL_PITCH, 0.0);
            assert_eq!(alGetError(), AL_INVALID_VALUE);
            al_source_f(src, AL_GAIN, -0.5);
            assert_eq!(alGetError(), AL_INVALID_VALUE);
            alGetSourcei(src, AL_SOURCE_STATE, std::ptr::null_mut());
            assert_eq!(alGetError(), AL_INVALID_VALUE);

            // AL_INVALID_OPERATION: deleting an attached buffer, or queueing on static source.
            alBufferData(buf, AL_FORMAT_MONO16, sample.as_ptr(), 4, 48_000);
            alSourcei(src, AL_BUFFER, buf as i32);
            assert_eq!(alGetError(), AL_NO_ERROR);

            alDeleteBuffers(1, &buf);
            assert_eq!(alGetError(), AL_INVALID_OPERATION);
            assert_eq!(alIsBuffer(buf), AL_TRUE);

            alSourceQueueBuffers(src, 1, &buf);
            assert_eq!(alGetError(), AL_INVALID_OPERATION);

            // ALC errors.
            assert_eq!(alcMakeContextCurrent(0xDEAD_BEEF), ALC_FALSE);
            assert_eq!(alcGetError(0), ALC_INVALID_CONTEXT);
            assert_eq!(alcGetError(0xDEAD_BEEF), ALC_INVALID_DEVICE);
            let mut v = 0;
            alcGetIntegerv(env.dev, 0x7777, 1, &mut v);
            assert_eq!(alcGetError(env.dev), ALC_INVALID_ENUM);
        }
    }

    #[test]
    fn openal_async_host_mixer_thread_pumps_and_tears_down() {
        let _env = TestEnv::new(false);
        unsafe {
            let mut buf = 0u32;
            let mut src = 0u32;
            alGenBuffers(1, &mut buf);
            alGenSources(1, &mut src);

            // ~10 ms mono16 buffer at 48 kHz (480 samples).
            let pcm = vec![0u8; 480 * 2];
            alBufferData(buf, AL_FORMAT_MONO16, pcm.as_ptr(), pcm.len() as i32, 48_000);
            alSourcei(src, AL_BUFFER, buf as i32);
            alSourcePlay(src);

            // Host mixer thread + Null OutputStream must consume the buffer and stop it.
            let deadline = Instant::now() + Duration::from_secs(2);
            let mut state_val = AL_PLAYING;
            while Instant::now() < deadline {
                alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
                if state_val == AL_STOPPED {
                    break;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            assert_eq!(
                state_val, AL_STOPPED,
                "async host mixer thread did not finish short buffer in time"
            );
            assert_eq!(alGetError(), AL_NO_ERROR);
        }
        // Drop of TestEnv exercises alcDestroyContext + alcCloseDevice teardown.
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn openal_win64_bridge_aarch64_mixed_float_slots() {
        let _env = TestEnv::new(true);
        unsafe {
            let mut src = 0u32;
            alGenSources(1, &mut src);

            // Win64 ABI for alSource3f(src, AL_POSITION, 1.5, -2.25, 3.75):
            //   slot 1 (x0) = src
            //   slot 2 (x1) = AL_POSITION
            //   slot 3 (v2/xmm2) = 1.5f32
            //   slot 4 (v3/xmm3) = -2.25f32
            //   slot 5 (x4/stack slot 5) = 3.75f32.to_bits()
            let x2 = f64::from_bits(1.5f32.to_bits() as u64);
            let x3 = f64::from_bits((-2.25f32).to_bits() as u64);
            al_source_3f::bridge(
                src as u64,
                AL_POSITION as u32 as u64,
                0xDEAD,
                0xBEEF,
                3.75f32.to_bits() as u64,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0.0,
                0.0,
                x2,
                x3,
            );

            let (mut px, mut py, mut pz) = (0.0f32, 0.0, 0.0);
            alGetSource3f(src, AL_POSITION, &mut px, &mut py, &mut pz);
            assert_eq!((px, py, pz), (1.5, -2.25, 3.75));
            assert_eq!(alGetError(), AL_NO_ERROR);
        }
    }
}
