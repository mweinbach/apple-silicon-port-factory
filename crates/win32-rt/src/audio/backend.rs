//! Host audio backends behind the WASAPI objects.
//!
//! * **CoreAudio** (macOS): endpoints = HAL output devices
//!   (`AudioObjectGetPropertyData`). Each initialized `IAudioClient` gets one
//!   output unit — `kAudioUnitSubType_DefaultOutput` for the default endpoint
//!   (follows system default-device switches by itself) or
//!   `kAudioUnitSubType_HALOutput` bound to a specific device — pulling
//!   interleaved float32 from the client's lock-free [`Ring`] in its render
//!   callback. The unit's input-scope format is the *client* format; AUHAL's
//!   converter resamples / maps channels to whatever the device runs.
//! * **Null**: a virtual 48 kHz stereo endpoint consumed in real time by a
//!   clock thread. Used when there is no output device, in tests, or with
//!   `AW2_AUDIO_BACKEND=null`.
//!
//! Threading rules (docs/design/ADVICE_M2_A1.md §2.1, Track 1 §1.1 rule 5):
//! * every CoreAudio/AudioToolbox/CoreFoundation call runs on one host
//!   "aw2-coreaudio" control thread ([`ca_sync`]), never on a Win32 thread —
//!   Win32 threads alias pthread TSD slots 6/11/12 with TEB/TLS/PEB;
//! * the render callback (CoreAudio IO thread) only touches atomics and the
//!   ring and signals a mach semaphore; a per-stream host "pump" thread turns
//!   that into `Event::set` on the client's Win32 event (`Event::set` takes the
//!   global wait lock, which must never be taken on the IO thread, and needs no
//!   TEB);
//! * no guest code ever runs on CoreAudio or helper threads;
//! * device-change listeners (device list, default output, per-device alive)
//!   run on the HAL's notification thread and only call the registered `fn()`
//!   (which posts to [`super::devnotify`]'s Win32 dispatcher thread).

use super::ring::Ring;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex, OnceLock};

/// One host output device as seen by the endpoint layer.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    /// `AudioDeviceID` (0 = null device).
    pub id: u32,
    /// Stable unique id (CoreAudio device UID or `"aw2-null"`).
    pub uid: String,
    pub name: String,
    /// Nominal device rate.
    pub rate: u32,
    /// Output channels (sum over output streams).
    pub channels: u32,
    /// Current HAL IO buffer size, frames.
    pub io_frames: u32,
    /// Device latency + safety offset + IO buffer, frames.
    pub latency_frames: u32,
    /// Headphones / Bluetooth (PKEY_AudioEndpoint_FormFactor).
    pub headphones: bool,
    /// System default output at enumeration time.
    pub is_default: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    CoreAudio,
    Null,
}

static FORCED: Mutex<Option<Backend>> = Mutex::new(None);
#[cfg(test)]
static FORCED_OUTPUT_DEVICES: Mutex<Option<Vec<DeviceInfo>>> = Mutex::new(None);
#[cfg(test)]
static FORCED_INPUT_DEVICES: Mutex<Option<Vec<DeviceInfo>>> = Mutex::new(None);

/// Force a backend (tests). `None` = automatic.
pub fn force_backend(b: Option<Backend>) {
    *FORCED.lock().unwrap() = b;
}

/// Override the enumerated output device list (tests only). `None` = automatic.
#[cfg(test)]
pub fn force_output_devices(devs: Option<Vec<DeviceInfo>>) {
    *FORCED_OUTPUT_DEVICES.lock().unwrap() = devs;
}

/// Override the enumerated capture/input device list (tests only). `None` = automatic.
#[cfg(test)]
pub fn force_input_devices(devs: Option<Vec<DeviceInfo>>) {
    *FORCED_INPUT_DEVICES.lock().unwrap() = devs;
}

pub fn backend() -> Backend {
    if let Some(b) = *FORCED.lock().unwrap() {
        return b;
    }
    match std::env::var("AW2_AUDIO_BACKEND").as_deref() {
        Ok("null") => Backend::Null,
        _ if cfg!(any(target_os = "macos", target_os = "ios")) => Backend::CoreAudio,
        _ => Backend::Null,
    }
}

pub fn null_device() -> DeviceInfo {
    DeviceInfo {
        id: 0,
        uid: "aw2-null".into(),
        name: "AW2 Null Output".into(),
        rate: 48000,
        channels: 2,
        io_frames: 512,
        latency_frames: 0,
        headphones: false,
        is_default: true,
    }
}

/// Fallback capture device when running headless, on the null backend, or when
/// no host CoreAudio input device is available.
pub fn null_capture_device() -> DeviceInfo {
    DeviceInfo {
        id: 0,
        uid: "port-null-capture".into(),
        name: "Default Microphone".into(),
        rate: 48000,
        channels: 2,
        io_frames: 512,
        latency_frames: 0,
        headphones: false,
        is_default: true,
    }
}

/// Synthetic 4-channel USB DualSense audio-haptics endpoint (`slot` = 0-based
/// USB DualSense index) used for tests and deterministic container pairing.
pub fn dualsense_usb_device(slot: u32) -> DeviceInfo {
    DeviceInfo {
        id: 0,
        uid: format!("dualsense-usb-audio-{slot}"),
        name: "Wireless Controller (DualSense USB)".into(),
        rate: 48000,
        channels: 4,
        io_frames: 512,
        latency_frames: 0,
        headphones: false,
        is_default: false,
    }
}

/// Output devices (default first). Falls back to the null device if the host
/// has none (or the backend is forced to null).
///
/// `AW2_AUDIO_DEVICE=<name substring | UID>` makes the first matching device
/// the default endpoint (bound explicitly via HALOutput, so it does not follow
/// system default-device changes). No match: logged, system default kept.
pub fn output_devices() -> Vec<DeviceInfo> {
    #[cfg(test)]
    if let Some(devs) = FORCED_OUTPUT_DEVICES.lock().unwrap().clone() {
        return devs;
    }
    let mut v = match backend() {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        Backend::CoreAudio => ca_sync(ca::output_devices),
        _ => Vec::new(),
    };
    if let Ok(want) = std::env::var("AW2_AUDIO_DEVICE") {
        if !want.is_empty() && !v.is_empty() {
            select_device(&mut v, &want);
        }
    }
    if v.is_empty() {
        v.push(null_device());
    }
    v
}

/// Capture / input devices (default first). Queries CoreAudio default input
/// metadata read-only via `AudioObjectGetPropertyData` (never opens a capture
/// stream or triggers macOS microphone TCC prompts), falling back cleanly to
/// `"Default Microphone"` when headless, forced null, or unavailable.
pub fn input_devices() -> Vec<DeviceInfo> {
    #[cfg(test)]
    if let Some(devs) = FORCED_INPUT_DEVICES.lock().unwrap().clone() {
        return devs;
    }
    let headless = std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
        || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1");
    let mut v = if headless {
        Vec::new()
    } else {
        match backend() {
            #[cfg(target_os = "macos")]
            Backend::CoreAudio => ca_sync(ca::input_devices),
            _ => Vec::new(),
        }
    };
    if v.is_empty() {
        v.push(null_capture_device());
    }
    v
}

/// Move the device matching `want` (exact UID, else case-insensitive name
/// substring) to the front as the only default.
pub fn select_device(v: &mut Vec<DeviceInfo>, want: &str) -> bool {
    let lw = want.to_lowercase();
    let pos = v.iter().position(|d| d.uid == want).or_else(|| v.iter().position(|d| d.name.to_lowercase().contains(&lw)));
    match pos {
        Some(i) => {
            let mut d = v.remove(i);
            let was_default = d.is_default;
            for o in v.iter_mut() {
                o.is_default = false;
            }
            // Keep DefaultOutput (follows switches) only if it already was the system default.
            d.is_default = was_default;
            log::info!(target: "audio", "AW2_AUDIO_DEVICE={want:?}: using '{}' ({})", d.name, d.uid);
            v.insert(0, d);
            true
        }
        None => {
            let names: Vec<&str> = v.iter().map(|d| d.name.as_str()).collect();
            log::warn!(target: "audio", "AW2_AUDIO_DEVICE={want:?} matches no output device (have {names:?}); keeping the system default");
            false
        }
    }
}

// ---------------------------------------------------------------------------
// CoreAudio control thread
// ---------------------------------------------------------------------------

type Job = Box<dyn FnOnce() + Send>;

thread_local! {
    static IN_CA_THREAD: Cell<bool> = const { Cell::new(false) };
}

/// Query CoreAudio HAL device master scalar volume `[0.0, 1.0]` when using the
/// CoreAudio backend on macOS (returns `None` on null/virtual/iOS devices).
pub fn device_volume_scalar(dev: &DeviceInfo, is_capture: bool) -> Option<f32> {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::device_volume_scalar(id, is_capture))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = is_capture;
        None
    }
}

pub fn set_device_volume_scalar(dev: &DeviceInfo, is_capture: bool, scalar: f32) -> bool {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return false;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::set_device_volume_scalar(id, is_capture, scalar))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (is_capture, scalar);
        false
    }
}

pub fn device_channel_volume_scalar(dev: &DeviceInfo, is_capture: bool, channel: u32) -> Option<f32> {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::device_channel_volume_scalar(id, is_capture, channel))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (is_capture, channel);
        None
    }
}

pub fn set_device_channel_volume_scalar(dev: &DeviceInfo, is_capture: bool, channel: u32, scalar: f32) -> bool {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return false;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::set_device_channel_volume_scalar(id, is_capture, channel, scalar))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (is_capture, channel, scalar);
        false
    }
}

pub fn device_mute(dev: &DeviceInfo, is_capture: bool) -> Option<bool> {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::device_mute(id, is_capture))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = is_capture;
        None
    }
}

pub fn set_device_mute(dev: &DeviceInfo, is_capture: bool, mute: bool) -> bool {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return false;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::set_device_mute(id, is_capture, mute))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (is_capture, mute);
        false
    }
}

pub fn device_volume_range_db(dev: &DeviceInfo, is_capture: bool) -> Option<(f32, f32)> {
    if dev.id == 0 || backend() != Backend::CoreAudio {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        let id = dev.id;
        ca_sync(move || ca::device_volume_range_db(id, is_capture))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = is_capture;
        None
    }
}

/// Real-time safe channel-layout converter + linear resampler for loopback audio.
/// Converts interleaved float32 input `(src_rate, src_channels)` into `(dst_rate, dst_channels)`
/// using preallocated buffers so the CoreAudio IOProc never allocates on the realtime thread.
pub(crate) struct TapFormatConverter {
    src_rate: u32,
    src_channels: usize,
    dst_rate: u32,
    dst_channels: usize,
    /// Preallocated scratch buffer for channel-converted frames (`src_rate`, `dst_channels`).
    ch_buf: Vec<f32>,
    /// Preallocated scratch buffer for resampled output frames (`dst_rate`, `dst_channels`).
    out_buf: Vec<f32>,
    /// Exact rational input-frame cursor numerator in units of `1 / dst_rate` input frames
    /// (zero floating-point drift across arbitrary stream durations).
    phase_num: u64,
    /// Last channel-converted frame from the previous callback (for interpolation across boundaries).
    prev_frame: [f32; 8],
    has_prev: bool,
}

impl TapFormatConverter {
    pub(crate) fn new(src_rate: u32, src_channels: u32, dst_rate: u32, dst_channels: u32) -> Self {
        let sc = src_channels.clamp(1, 8) as usize;
        let dc = dst_channels.clamp(1, 8) as usize;
        Self {
            src_rate: src_rate.max(1),
            src_channels: sc,
            dst_rate: dst_rate.max(1),
            dst_channels: dc,
            ch_buf: Vec::with_capacity(4096 * dc),
            out_buf: Vec::with_capacity(8192 * dc),
            phase_num: 0,
            prev_frame: [0.0; 8],
            has_prev: false,
        }
    }

    #[inline]
    pub(crate) fn matches_src(&self, src_rate: u32, src_channels: u32) -> bool {
        self.src_rate == src_rate.max(1) && self.src_channels == src_channels.clamp(1, 8) as usize
    }

    #[inline]
    pub(crate) fn is_passthrough(&self) -> bool {
        self.src_rate == self.dst_rate && self.src_channels == self.dst_channels
    }

    /// Convert interleaved float32 input samples and return a slice of interleaved float32 output
    /// samples in `(dst_rate, dst_channels)`.
    pub(crate) fn convert<'a>(&'a mut self, input: &'a [f32]) -> &'a [f32] {
        if self.is_passthrough() {
            return input;
        }
        let sc = self.src_channels;
        let dc = self.dst_channels;
        let in_frames = input.len() / sc;
        if in_frames == 0 {
            return &[];
        }

        // Step 1: Channel conversion (src_channels -> dst_channels).
        let ch_src: &[f32] = if sc == dc {
            &input[..in_frames * sc]
        } else {
            self.ch_buf.clear();
            let cap_frames = self.ch_buf.capacity() / dc;
            let frames = in_frames.min(cap_frames.max(in_frames));
            if self.ch_buf.capacity() < frames * dc {
                self.ch_buf.reserve(frames * dc - self.ch_buf.capacity());
            }
            for f in 0..frames {
                let base = f * sc;
                if dc == 1 {
                    let mut sum = 0.0f32;
                    for c in 0..sc {
                        sum += input[base + c];
                    }
                    self.ch_buf.push(sum / sc as f32);
                } else if sc == 1 {
                    let s = input[base];
                    for c in 0..dc {
                        self.ch_buf.push(if c < 2 { s } else { 0.0 });
                    }
                } else {
                    for c in 0..dc {
                        self.ch_buf.push(if c < sc { input[base + c] } else { 0.0 });
                    }
                }
            }
            &self.ch_buf
        };

        if self.src_rate == self.dst_rate {
            return ch_src;
        }

        // Step 2: Exact-ratio linear resampling (src_rate -> dst_rate).
        let src_frames = ch_src.len() / dc;
        let sr = self.src_rate as u64;
        let dr = self.dst_rate as u64;
        let inv_dr = 1.0f32 / self.dst_rate as f32;
        self.out_buf.clear();
        if !self.has_prev {
            for c in 0..dc {
                self.prev_frame[c] = ch_src[c];
            }
            self.has_prev = true;
        }

        let mut pos_num = self.phase_num;
        let limit_num = (src_frames as u64) * dr;
        while pos_num < limit_num {
            let idx = (pos_num / dr) as usize;
            let frac = (pos_num % dr) as f32 * inv_dr;
            for c in 0..dc {
                let s0 = if idx == 0 {
                    self.prev_frame[c]
                } else {
                    ch_src[(idx - 1) * dc + c]
                };
                let s1 = ch_src[idx.min(src_frames - 1) * dc + c];
                self.out_buf.push(s0 + (s1 - s0) * frac);
            }
            pos_num += sr;
        }
        self.phase_num = pos_num - limit_num;
        let last_base = (src_frames - 1) * dc;
        for c in 0..dc {
            self.prev_frame[c] = ch_src[last_base + c];
        }
        &self.out_buf
    }
}

/// Optional live macOS 14.2+ CoreAudio Process Tap for system loopback capture.
pub struct LoopbackTap {
    #[cfg(target_os = "macos")]
    tap: Option<ca::ProcessTap>,
}

impl LoopbackTap {
    /// Attempt to open a macOS 14.2+ CoreAudio process tap feeding `ring` in `(target_rate, target_channels)`.
    /// Returns `None` when headless, on the Null backend, on iOS, on macOS < 14.2,
    /// or when audio-capture TCC permission is not granted (caller falls back to
    /// in-process WASAPI render loopback).
    pub fn try_open(dev: &DeviceInfo, ring: Arc<Ring>, target_rate: u32, target_channels: u32) -> Option<Self> {
        let headless = std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
            || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1");
        if headless || dev.id == 0 || backend() != Backend::CoreAudio {
            return None;
        }
        #[cfg(target_os = "macos")]
        {
            let uid = dev.uid.clone();
            match ca_sync(move || ca::ProcessTap::open(ring, Some(&uid), target_rate, target_channels)) {
                Ok(tap) => {
                    log::info!(target: "audio", "CoreAudio ProcessTap opened for loopback capture on '{}'", dev.name);
                    Some(Self { tap: Some(tap) })
                }
                Err(e) => {
                    log::debug!(target: "audio", "CoreAudio ProcessTap unavailable ({e}); using in-process WASAPI render loopback");
                    None
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (ring, target_rate, target_channels);
            None
        }
    }
}

impl Drop for LoopbackTap {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        if let Some(t) = self.tap.take() {
            ca_sync(move || drop(t));
        }
    }
}

/// Run `f` on the dedicated host thread that owns all CoreAudio calls and wait
/// for its result. If already running on the CoreAudio control thread, runs
/// inline to prevent self-deadlock.
pub fn ca_sync<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    if IN_CA_THREAD.with(|c| c.get()) {
        return f();
    }
    // Shadow probe children: the worker thread is gone after fork (g3br: Wwise device
    // enumeration in a run crashed in libdispatch).
    crate::shadow::guard_host("coreaudio");
    static TX: OnceLock<Mutex<mpsc::Sender<Job>>> = OnceLock::new();
    let tx = TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Job>();
        std::thread::Builder::new()
            .name("aw2-coreaudio".into())
            .spawn(move || {
                IN_CA_THREAD.with(|c| c.set(true));
                while let Ok(job) = rx.recv() {
                    job();
                }
            })
            .expect("spawn aw2-coreaudio thread");
        Mutex::new(tx)
    });
    let (rtx, rrx) = mpsc::sync_channel::<R>(1);
    let job: Job = Box::new(move || {
        let _ = rtx.send(f());
    });
    tx.lock().unwrap().send(job).expect("aw2-coreaudio thread is gone");
    rrx.recv().expect("aw2-coreaudio job panicked")
}

// ---------------------------------------------------------------------------
// Streams
// ---------------------------------------------------------------------------

/// Playback-side loopback feed attached to render/spatial `StreamShared` instances.
///
/// The playback consumer (CoreAudio RT callback or Null clock thread) feeds samples
/// strictly as playback consumes them from `StreamShared::ring`, ensuring unstarted
/// prefill or reset audio never leaks into fallback loopback capture.
pub struct LoopbackFeed {
    pub ep_id: String,
    pub source_id: u64,
    pub rate: u32,
    pub channels: u16,
    /// Lock-free SPSC ring filled by the realtime CoreAudio IO thread and drained by
    /// the non-RT `wwise-audio-pump` thread into `push_loopback_frames`.
    pub ring: Ring,
}

/// State shared between the client, the IO callback and the pump thread.
pub struct StreamShared {
    pub ring: Ring,
    pub loopback: Option<LoopbackFeed>,
    pub started: AtomicBool,
    pub underrun_frames: AtomicU64,
    pub callbacks: AtomicU64,
    /// Largest slice the device pulled in one callback.
    pub max_slice: AtomicU64,
    /// Event signalled once per device IO cycle while started.
    pub event: Mutex<Option<Arc<crate::handles::Event>>>,
    /// Fast lock-free check for whether `SetEventHandle` has installed an event.
    pub has_event: AtomicBool,
    /// The endpoint went away (device removed / dead / format changed):
    /// every client call fails with AUDCLNT_E_DEVICE_INVALIDATED from now on.
    pub invalid: AtomicBool,
    sema: Semaphore,
    quit: AtomicBool,
}

static GLOBAL_UNDERRUN_FRAMES: AtomicU64 = AtomicU64::new(0);

/// Return and reset total CoreAudio underrun frames across all output streams
/// since the previous call (zero overhead when no underruns occur).
pub fn take_underrun_frames() -> u64 {
    GLOBAL_UNDERRUN_FRAMES.swap(0, Ordering::Relaxed)
}

impl StreamShared {
    pub fn new(ring: Ring) -> Arc<StreamShared> {
        Arc::new(StreamShared {
            ring,
            loopback: None,
            started: AtomicBool::new(false),
            underrun_frames: AtomicU64::new(0),
            callbacks: AtomicU64::new(0),
            max_slice: AtomicU64::new(0),
            event: Mutex::new(None),
            has_event: AtomicBool::new(false),
            invalid: AtomicBool::new(false),
            sema: Semaphore::new(),
            quit: AtomicBool::new(false),
        })
    }

    pub fn with_loopback(
        ring: Ring,
        ep_id: String,
        source_id: u64,
        rate: u32,
        channels: u16,
    ) -> Arc<StreamShared> {
        let lb_cap = ring.capacity().max(8192);
        let lb_ring = Ring::new(lb_cap, channels.max(1) as usize);
        Arc::new(StreamShared {
            ring,
            loopback: Some(LoopbackFeed {
                ep_id,
                source_id,
                rate,
                channels,
                ring: lb_ring,
            }),
            started: AtomicBool::new(false),
            underrun_frames: AtomicU64::new(0),
            callbacks: AtomicU64::new(0),
            max_slice: AtomicU64::new(0),
            event: Mutex::new(None),
            has_event: AtomicBool::new(false),
            invalid: AtomicBool::new(false),
            sema: Semaphore::new(),
            quit: AtomicBool::new(false),
        })
    }

    pub fn set_event(&self, ev: Arc<crate::handles::Event>) {
        *self.event.lock().unwrap() = Some(ev);
        self.has_event.store(true, Ordering::Release);
    }

    /// Realtime-safe: the device consumed `frames` (got `got` of them).
    #[inline]
    pub fn consumed(&self, frames: usize, got: usize) {
        if got < frames {
            let miss = (frames - got) as u64;
            self.underrun_frames.fetch_add(miss, Ordering::Relaxed);
            GLOBAL_UNDERRUN_FRAMES.fetch_add(miss, Ordering::Relaxed);
        }
        self.max_slice.fetch_max(frames as u64, Ordering::Relaxed);
        self.callbacks.fetch_add(1, Ordering::Relaxed);
        self.sema.signal();
    }

    pub fn signal_event(&self) {
        let ev = self.event.lock().unwrap().clone();
        if let Some(e) = ev {
            e.set();
        }
    }
}

/// CoreAudio output unit wrapped so its disposal (`AudioOutputUnitStop` /
/// `AudioUnitUninitialize` / `AudioComponentInstanceDispose`) always runs on
/// the `aw2-coreaudio` control thread outside any client or backend mutex.
#[cfg(any(target_os = "macos", target_os = "ios"))]
struct ManagedUnit {
    handle: ca::Handle,
    unit: Option<ca::Unit>,
}

// All CoreAudio operations on `handle` / `unit` are serialized onto the single
// `aw2-coreaudio` thread via `ca_sync`, and `unit` is only touched on `Drop`.
#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe impl Send for ManagedUnit {}
#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe impl Sync for ManagedUnit {}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl ManagedUnit {
    fn new(unit: ca::Unit) -> Arc<Self> {
        let handle = unit.handle();
        Arc::new(Self { handle, unit: Some(unit) })
    }

    fn handle(&self) -> ca::Handle {
        self.handle
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Drop for ManagedUnit {
    fn drop(&mut self) {
        if let Some(u) = self.unit.take() {
            ca_sync(move || drop(u));
        }
    }
}

/// Outer-mutex-free start/stop handle for an [`OutputStream`]. Cloning this
/// before dropping `ClientCore::st` lets `Start`/`Stop` call
/// `AudioOutputUnitStart`/`AudioOutputUnitStop` without holding any mutex
/// while keeping the underlying `AudioUnit` alive until the call returns.
#[derive(Clone)]
pub struct StreamControl {
    shared: Arc<StreamShared>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    unit: Option<Arc<ManagedUnit>>,
}

impl StreamControl {
    pub fn start(&self) -> Result<(), String> {
        self.shared.started.store(true, Ordering::Release);
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let Some(u) = &self.unit {
            let au = u.handle();
            if let Err(e) = ca_sync(move || ca::start(au)) {
                self.shared.started.store(false, Ordering::Release);
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn stop(&self) {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let Some(u) = &self.unit {
            let au = u.handle();
            ca_sync(move || ca::stop(au));
        }
        self.shared.started.store(false, Ordering::Release);
    }
}

/// A host stream. Dropping it stops and tears down the device unit and the
/// helper threads (no callbacks run afterwards).
pub struct OutputStream {
    shared: Arc<StreamShared>,
    pump: Option<std::thread::JoinHandle<()>>,
    clock: Option<std::thread::JoinHandle<()>>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    unit: Option<Arc<ManagedUnit>>,
    /// IO slice granted by the device (frames at the device rate).
    pub io_frames: u32,
    pub kind: Backend,
}

impl OutputStream {
    /// Open (but don't start) a stream of interleaved f32 at `rate`/`channels`.
    /// `io_request` = desired IO cycle in device frames.
    pub fn open(dev: &DeviceInfo, rate: u32, channels: u16, shared: Arc<StreamShared>, io_request: u32) -> Result<OutputStream, String> {
        let kind = if dev.id == 0 { Backend::Null } else { backend() };
        let pump = {
            let s = shared.clone();
            std::thread::Builder::new()
                .name("wwise-audio-pump".into())
                .spawn(move || {
                    let mut lb_buf = Vec::<f32>::new();
                    while !s.quit.load(Ordering::Acquire) {
                        if s.sema.wait() && !s.quit.load(Ordering::Acquire) {
                            if let Some(lb) = &s.loopback {
                                let ch = lb.ring.channels();
                                loop {
                                    let avail = lb.ring.len();
                                    if avail == 0 {
                                        break;
                                    }
                                    let take = avail.min(4096);
                                    lb_buf.resize(take * ch, 0.0);
                                    let got = lb.ring.pop_into(&mut lb_buf[..take * ch]);
                                    if got == 0 {
                                        break;
                                    }
                                    super::audioclient::push_loopback_frames(
                                        &lb.ep_id,
                                        lb.source_id,
                                        &lb_buf[..got * ch],
                                        lb.rate,
                                        lb.channels,
                                    );
                                }
                            }
                            if s.started.load(Ordering::Acquire) {
                                s.signal_event();
                            }
                        }
                    }
                })
                .map_err(|e| e.to_string())?
        };
        let mut st = OutputStream {
            shared: shared.clone(),
            pump: Some(pump),
            clock: None,
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            unit: None,
            io_frames: io_request,
            kind,
        };
        match kind {
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            Backend::CoreAudio => {
                let (id, def) = (dev.id, dev.is_default);
                let (unit, granted) = ca_sync(move || ca::Unit::open(id, def, rate, channels, shared, io_request))?;
                st.unit = Some(ManagedUnit::new(unit));
                st.io_frames = granted;
            }
            _ => {
                let s = shared;
                let period = io_request.max(1) as usize;
                let dur = std::time::Duration::from_secs_f64(period as f64 / rate as f64);
                st.clock = Some(
                    std::thread::Builder::new()
                        .name("wwise-audio-null".into())
                        .spawn(move || {
                            let ch = s.ring.channels();
                            let mut lb_buf = vec![0.0f32; period * ch];
                            let mut next = std::time::Instant::now();
                            while !s.quit.load(Ordering::Acquire) {
                                next += dur;
                                let now = std::time::Instant::now();
                                if next > now {
                                    std::thread::sleep(next - now);
                                } else if now - next > dur * 8 {
                                    next = now; // don't burst after a long stall
                                }
                                if s.started.load(Ordering::Acquire) {
                                    let got = if let Some(lb) = &s.loopback {
                                        if super::audioclient::has_loopback_subscribers() {
                                            let got = s.ring.pop_into(&mut lb_buf);
                                            if got > 0 {
                                                super::audioclient::push_loopback_frames(
                                                    &lb.ep_id,
                                                    lb.source_id,
                                                    &lb_buf[..got * ch],
                                                    lb.rate,
                                                    lb.channels,
                                                );
                                            }
                                            got
                                        } else {
                                            s.ring.discard(period)
                                        }
                                    } else {
                                        s.ring.discard(period)
                                    };
                                    s.consumed(period, got);
                                }
                            }
                        })
                        .map_err(|e| e.to_string())?,
                );
            }
        }
        Ok(st)
    }

    pub fn control(&self) -> StreamControl {
        StreamControl {
            shared: self.shared.clone(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            unit: self.unit.clone(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.control().start()
    }

    pub fn stop(&mut self) {
        self.control().stop()
    }
}

impl Drop for OutputStream {
    fn drop(&mut self) {
        self.stop();
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        drop(self.unit.take()); // disposes the AU on aw2-coreaudio once the last Arc drops
        self.shared.quit.store(true, Ordering::Release);
        self.shared.sema.signal();
        if let Some(t) = self.clock.take() {
            let _ = t.join();
        }
        if let Some(t) = self.pump.take() {
            let _ = t.join();
        }
    }
}

// ---------------------------------------------------------------------------
// Mach semaphore (semaphore_signal is safe on realtime threads; no
// CoreFoundation / libdispatch involved).
// ---------------------------------------------------------------------------

extern "C" {
    static mach_task_self_: u32;
    fn semaphore_create(task: u32, sem: *mut u32, policy: i32, value: i32) -> i32;
    fn semaphore_destroy(task: u32, sem: u32) -> i32;
    fn semaphore_signal(sem: u32) -> i32;
    fn semaphore_wait(sem: u32) -> i32;
}

struct Semaphore(u32);

impl Semaphore {
    fn new() -> Semaphore {
        let mut s = 0u32;
        let kr = unsafe { semaphore_create(mach_task_self_, &mut s, 0 /* SYNC_POLICY_FIFO */, 0) };
        assert_eq!(kr, 0, "semaphore_create failed: {kr}");
        Semaphore(s)
    }
    #[inline]
    fn signal(&self) {
        unsafe { semaphore_signal(self.0) };
    }
    /// Block until signalled (returns false on interruption/abort).
    fn wait(&self) -> bool {
        unsafe { semaphore_wait(self.0) == 0 }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { semaphore_destroy(mach_task_self_, self.0) };
    }
}

// ---------------------------------------------------------------------------
// CoreAudio FFI (subset). Constants are FourCCs from CoreAudio/AudioHardware*.h
// and AudioUnit/AudioUnitProperties.h. Everything in here runs on the
// aw2-coreaudio control thread, except `render` (CoreAudio IO thread).
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod ca {
    use super::*;
    use std::ffi::c_void;

    type OSStatus = i32;
    type AudioObjectID = u32;
    type CFStringRef = *const c_void;
    pub type AudioUnit = *mut c_void;
    type AudioComponent = *mut c_void;

    const fn fourcc(s: &[u8; 4]) -> u32 {
        ((s[0] as u32) << 24) | ((s[1] as u32) << 16) | ((s[2] as u32) << 8) | s[3] as u32
    }

    const kAudioObjectSystemObject: AudioObjectID = 1;
    const kAudioObjectPropertyScopeGlobal: u32 = fourcc(b"glob");
    const kAudioDevicePropertyScopeInput: u32 = fourcc(b"inpt");
    const kAudioDevicePropertyScopeOutput: u32 = fourcc(b"outp");
    const kAudioObjectPropertyElementMain: u32 = 0;
    const kAudioHardwarePropertyDevices: u32 = fourcc(b"dev#");
    const kAudioHardwarePropertyDefaultInputDevice: u32 = fourcc(b"dIn ");
    const kAudioHardwarePropertyDefaultOutputDevice: u32 = fourcc(b"dOut");
    const kAudioHardwarePropertyRunLoop: u32 = fourcc(b"rnlp");
    const kAudioDevicePropertyDeviceIsAlive: u32 = fourcc(b"livn");
    const kAudioDevicePropertyStreamConfiguration: u32 = fourcc(b"slay");
    const kAudioDevicePropertyNominalSampleRate: u32 = fourcc(b"nsrt");
    const kAudioDevicePropertyDeviceUID: u32 = fourcc(b"uid ");
    const kAudioObjectPropertyName: u32 = fourcc(b"lnam");
    const kAudioDevicePropertyBufferFrameSize: u32 = fourcc(b"fsiz");
    const kAudioDevicePropertyLatency: u32 = fourcc(b"ltnc");
    const kAudioDevicePropertySafetyOffset: u32 = fourcc(b"saft");
    const kAudioDevicePropertyDataSource: u32 = fourcc(b"ssrc");
    const kAudioDevicePropertyTransportType: u32 = fourcc(b"tran");
    const kAudioDeviceTransportTypeBluetooth: u32 = fourcc(b"blue");
    const kIOAudioOutputPortSubTypeHeadphones: u32 = fourcc(b"hdpn");
    const kAudioHardwareServiceDeviceProperty_VirtualMainVolume: u32 = fourcc(b"vmvc");
    const kAudioDevicePropertyVolumeScalar: u32 = fourcc(b"volm");
    const kAudioDevicePropertyMute: u32 = fourcc(b"mute");
    const kAudioDevicePropertyVolumeRangeDecibels: u32 = fourcc(b"vdb#");
    const kAudioDevicePropertyStreamFormat: u32 = fourcc(b"sfmt");
    const kAudioTapPropertyFormat: u32 = fourcc(b"tfmt");

    const kAudioUnitType_Output: u32 = fourcc(b"auou");
    const kAudioUnitSubType_HALOutput: u32 = fourcc(b"ahal");
    const kAudioUnitSubType_DefaultOutput: u32 = fourcc(b"def ");
    const kAudioUnitManufacturer_Apple: u32 = fourcc(b"appl");
    const kAudioOutputUnitProperty_CurrentDevice: u32 = 2000;
    const kAudioUnitProperty_StreamFormat: u32 = 8;
    const kAudioUnitProperty_SetRenderCallback: u32 = 23;
    const kAudioUnitProperty_MaximumFramesPerSlice: u32 = 14;
    const kAudioUnitScope_Global: u32 = 0;
    const kAudioUnitScope_Input: u32 = 1;
    const kAudioFormatLinearPCM: u32 = fourcc(b"lpcm");
    const kAudioFormatFlagIsFloat: u32 = 1;
    const kAudioFormatFlagIsPacked: u32 = 8;
    const kAudioFormatFlagIsNonInterleaved: u32 = 1 << 5;
    const kAudioUnitRenderAction_OutputIsSilence: u32 = 1 << 4;
    const kCFStringEncodingUTF8: u32 = 0x0800_0100;

    /// Slice limit (CoreAudio raises the slice to 4096 when the display sleeps).
    pub const MAX_SLICE: u32 = 4096;

    #[repr(C)]
    struct AudioObjectPropertyAddress {
        mSelector: u32,
        mScope: u32,
        mElement: u32,
    }

    #[repr(C)]
    struct AudioComponentDescription {
        componentType: u32,
        componentSubType: u32,
        componentManufacturer: u32,
        componentFlags: u32,
        componentFlagsMask: u32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct AudioStreamBasicDescription {
        mSampleRate: f64,
        mFormatID: u32,
        mFormatFlags: u32,
        mBytesPerPacket: u32,
        mFramesPerPacket: u32,
        mBytesPerFrame: u32,
        mChannelsPerFrame: u32,
        mBitsPerChannel: u32,
        mReserved: u32,
    }

    #[repr(C)]
    struct AudioBuffer {
        mNumberChannels: u32,
        mDataByteSize: u32,
        mData: *mut c_void,
    }

    #[repr(C)]
    struct AudioBufferList {
        mNumberBuffers: u32,
        mBuffers: [AudioBuffer; 1],
    }

    type AURenderCallback =
        unsafe extern "C" fn(refcon: *mut c_void, flags: *mut u32, ts: *const c_void, bus: u32, frames: u32, data: *mut AudioBufferList) -> OSStatus;

    #[repr(C)]
    struct AURenderCallbackStruct {
        inputProc: AURenderCallback,
        inputProcRefCon: *mut c_void,
    }

    type ListenerProc = unsafe extern "C" fn(id: AudioObjectID, n: u32, addrs: *const AudioObjectPropertyAddress, client: *mut c_void) -> OSStatus;

    #[link(name = "CoreAudio", kind = "framework")]
    extern "C" {
        fn AudioObjectAddPropertyListener(id: AudioObjectID, addr: *const AudioObjectPropertyAddress, proc_: ListenerProc, client: *mut c_void) -> OSStatus;
        fn AudioObjectRemovePropertyListener(id: AudioObjectID, addr: *const AudioObjectPropertyAddress, proc_: ListenerProc, client: *mut c_void) -> OSStatus;
        fn AudioObjectSetPropertyData(
            id: AudioObjectID,
            addr: *const AudioObjectPropertyAddress,
            qsize: u32,
            qdata: *const c_void,
            size: u32,
            data: *const c_void,
        ) -> OSStatus;
        fn AudioObjectGetPropertyDataSize(id: AudioObjectID, addr: *const AudioObjectPropertyAddress, qsize: u32, qdata: *const c_void, size: *mut u32) -> OSStatus;
        fn AudioObjectGetPropertyData(
            id: AudioObjectID,
            addr: *const AudioObjectPropertyAddress,
            qsize: u32,
            qdata: *const c_void,
            size: *mut u32,
            data: *mut c_void,
        ) -> OSStatus;
    }

    #[link(name = "AudioToolbox", kind = "framework")]
    extern "C" {
        fn AudioComponentFindNext(c: AudioComponent, desc: *const AudioComponentDescription) -> AudioComponent;
        fn AudioComponentInstanceNew(c: AudioComponent, out: *mut AudioUnit) -> OSStatus;
        fn AudioComponentInstanceDispose(u: AudioUnit) -> OSStatus;
        fn AudioUnitSetProperty(u: AudioUnit, id: u32, scope: u32, elem: u32, data: *const c_void, size: u32) -> OSStatus;
        fn AudioUnitGetProperty(u: AudioUnit, id: u32, scope: u32, elem: u32, data: *mut c_void, size: *mut u32) -> OSStatus;
        fn AudioUnitInitialize(u: AudioUnit) -> OSStatus;
        fn AudioUnitUninitialize(u: AudioUnit) -> OSStatus;
        fn AudioOutputUnitStart(u: AudioUnit) -> OSStatus;
        fn AudioOutputUnitStop(u: AudioUnit) -> OSStatus;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringGetCString(s: CFStringRef, buf: *mut u8, size: isize, enc: u32) -> u8;
        fn CFRelease(o: *const c_void);
    }

    fn addr(sel: u32, scope: u32) -> AudioObjectPropertyAddress {
        AudioObjectPropertyAddress { mSelector: sel, mScope: scope, mElement: kAudioObjectPropertyElementMain }
    }

    unsafe fn get<T: Default>(id: AudioObjectID, sel: u32, scope: u32) -> Option<T> {
        let a = addr(sel, scope);
        let mut v = T::default();
        let mut size = std::mem::size_of::<T>() as u32;
        let st = AudioObjectGetPropertyData(id, &a, 0, std::ptr::null(), &mut size, &mut v as *mut T as *mut c_void);
        (st == 0).then_some(v)
    }

    unsafe fn get_bytes(id: AudioObjectID, sel: u32, scope: u32) -> Option<Vec<u8>> {
        let a = addr(sel, scope);
        let mut size = 0u32;
        if AudioObjectGetPropertyDataSize(id, &a, 0, std::ptr::null(), &mut size) != 0 {
            return None;
        }
        // u64-backed for AudioBufferList alignment.
        let mut buf = vec![0u64; (size as usize).div_ceil(8)];
        if AudioObjectGetPropertyData(id, &a, 0, std::ptr::null(), &mut size, buf.as_mut_ptr() as *mut c_void) != 0 {
            return None;
        }
        let bytes = std::slice::from_raw_parts(buf.as_ptr() as *const u8, size as usize);
        Some(bytes.to_vec())
    }

    unsafe fn get_string(id: AudioObjectID, sel: u32) -> Option<String> {
        let a = addr(sel, kAudioObjectPropertyScopeGlobal);
        let mut s: CFStringRef = std::ptr::null();
        let mut size = std::mem::size_of::<CFStringRef>() as u32;
        if AudioObjectGetPropertyData(id, &a, 0, std::ptr::null(), &mut size, &mut s as *mut _ as *mut c_void) != 0 || s.is_null() {
            return None;
        }
        let mut buf = vec![0u8; 1024];
        let ok = CFStringGetCString(s, buf.as_mut_ptr(), buf.len() as isize, kCFStringEncodingUTF8) != 0;
        CFRelease(s);
        if !ok {
            return None;
        }
        let n = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        Some(String::from_utf8_lossy(&buf[..n]).into_owned())
    }

    unsafe fn stream_channels(id: AudioObjectID, scope: u32) -> u32 {
        let Some(b) = get_bytes(id, kAudioDevicePropertyStreamConfiguration, scope) else { return 0 };
        if b.len() < 4 {
            return 0;
        }
        let n = u32::from_ne_bytes(b[0..4].try_into().unwrap()) as usize;
        // AudioBufferList: u32 count, pad to 8, then {u32 ch, u32 size, ptr} x n.
        (0..n)
            .filter_map(|i| b.get(8 + i * 16..12 + i * 16))
            .map(|c| u32::from_ne_bytes(c.try_into().unwrap()))
            .sum()
    }

    pub fn default_output() -> Option<u32> {
        unsafe { get::<u32>(kAudioObjectSystemObject, kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyScopeGlobal) }.filter(|&d| d != 0)
    }

    pub fn default_input() -> Option<u32> {
        unsafe { get::<u32>(kAudioObjectSystemObject, kAudioHardwarePropertyDefaultInputDevice, kAudioObjectPropertyScopeGlobal) }.filter(|&d| d != 0)
    }

    /// Read-only metadata query for the host CoreAudio default input device.
    /// Never opens an input AudioUnit or IO proc, so macOS TCC microphone
    /// permission prompts are never triggered.
    pub fn input_devices() -> Vec<DeviceInfo> {
        let mut v = Vec::new();
        unsafe {
            let Some(id) = default_input() else {
                return v;
            };
            if get::<u32>(id, kAudioDevicePropertyDeviceIsAlive, kAudioObjectPropertyScopeGlobal) == Some(0) {
                return v;
            }
            let ch = stream_channels(id, kAudioDevicePropertyScopeInput);
            if ch == 0 {
                return v;
            }
            let rate = get::<f64>(id, kAudioDevicePropertyNominalSampleRate, kAudioObjectPropertyScopeGlobal).unwrap_or(48000.0);
            let io = get::<u32>(id, kAudioDevicePropertyBufferFrameSize, kAudioObjectPropertyScopeGlobal).unwrap_or(512);
            let lat = get::<u32>(id, kAudioDevicePropertyLatency, kAudioDevicePropertyScopeInput).unwrap_or(0)
                + get::<u32>(id, kAudioDevicePropertySafetyOffset, kAudioDevicePropertyScopeInput).unwrap_or(0)
                + io;
            v.push(DeviceInfo {
                id,
                uid: get_string(id, kAudioDevicePropertyDeviceUID).unwrap_or_else(|| format!("coreaudio-in-{id}")),
                name: get_string(id, kAudioObjectPropertyName).unwrap_or_else(|| "Default Microphone".into()),
                rate: rate.round() as u32,
                channels: ch,
                io_frames: io,
                latency_frames: lat,
                headphones: false,
                is_default: true,
            });
        }
        v
    }

    pub fn output_devices() -> Vec<DeviceInfo> {
        let mut v = Vec::new();
        unsafe {
            let Some(b) = get_bytes(kAudioObjectSystemObject, kAudioHardwarePropertyDevices, kAudioObjectPropertyScopeGlobal) else {
                log::warn!(target: "audio", "CoreAudio: cannot list devices");
                return v;
            };
            let def = default_output();
            let mut ids = Vec::new();
            for c in b.chunks_exact(4) {
                let id = u32::from_ne_bytes(c.try_into().unwrap());
                let ch = stream_channels(id, kAudioDevicePropertyScopeOutput);
                if ch == 0 {
                    continue;
                }
                ids.push(id);
                if get::<u32>(id, kAudioDevicePropertyDeviceIsAlive, kAudioObjectPropertyScopeGlobal) == Some(0) {
                    continue; // going away; its 'dev#' notification follows
                }
                let rate = get::<f64>(id, kAudioDevicePropertyNominalSampleRate, kAudioObjectPropertyScopeGlobal).unwrap_or(48000.0);
                let io = get::<u32>(id, kAudioDevicePropertyBufferFrameSize, kAudioObjectPropertyScopeGlobal).unwrap_or(512);
                let lat = get::<u32>(id, kAudioDevicePropertyLatency, kAudioDevicePropertyScopeOutput).unwrap_or(0)
                    + get::<u32>(id, kAudioDevicePropertySafetyOffset, kAudioDevicePropertyScopeOutput).unwrap_or(0)
                    + io;
                let transport = get::<u32>(id, kAudioDevicePropertyTransportType, kAudioObjectPropertyScopeGlobal).unwrap_or(0);
                let source = get::<u32>(id, kAudioDevicePropertyDataSource, kAudioDevicePropertyScopeOutput).unwrap_or(0);
                let info = DeviceInfo {
                    id,
                    uid: get_string(id, kAudioDevicePropertyDeviceUID).unwrap_or_else(|| format!("coreaudio-{id}")),
                    name: get_string(id, kAudioObjectPropertyName).unwrap_or_else(|| format!("Audio device {id}")),
                    rate: rate.round() as u32,
                    channels: ch,
                    io_frames: io,
                    latency_frames: lat,
                    headphones: source == kIOAudioOutputPortSubTypeHeadphones || transport == kAudioDeviceTransportTypeBluetooth,
                    is_default: Some(id) == def,
                };
                if info.is_default {
                    v.insert(0, info);
                } else {
                    v.push(info);
                }
            }
            if WATCHING.load(Ordering::Relaxed) {
                sync_alive_listeners(&ids);
            }
        }
        v
    }

    // -- device-change listeners -------------------------------------------

    static ON_CHANGE: OnceLock<fn()> = OnceLock::new();
    static WATCHING: AtomicBool = AtomicBool::new(false);
    /// Devices with a 'livn' listener (control thread only).
    static ALIVE_WATCHED: Mutex<Vec<AudioObjectID>> = Mutex::new(Vec::new());

    /// HAL notification thread: only forwards to the registered hook.
    unsafe extern "C" fn listener(_id: AudioObjectID, _n: u32, _addrs: *const AudioObjectPropertyAddress, _client: *mut c_void) -> OSStatus {
        if let Some(f) = ON_CHANGE.get() {
            f();
        }
        0
    }

    fn sync_alive_listeners(ids: &[AudioObjectID]) {
        let a = addr(kAudioDevicePropertyDeviceIsAlive, kAudioObjectPropertyScopeGlobal);
        let (to_remove, to_add): (Vec<AudioObjectID>, Vec<AudioObjectID>) = {
            let w = ALIVE_WATCHED.lock().unwrap();
            let rem = w.iter().copied().filter(|id| !ids.contains(id)).collect();
            let add = ids.iter().copied().filter(|id| !w.contains(id)).collect();
            (rem, add)
        };
        // Never hold ALIVE_WATCHED across CoreAudio HAL listener add/remove calls.
        for id in &to_remove {
            unsafe { AudioObjectRemovePropertyListener(*id, &a, listener, std::ptr::null_mut()) }; // fails harmlessly if the device is gone
        }
        let mut added = Vec::new();
        for id in to_add {
            if unsafe { AudioObjectAddPropertyListener(id, &a, listener, std::ptr::null_mut()) } == 0 {
                added.push(id);
            }
        }
        let mut w = ALIVE_WATCHED.lock().unwrap();
        w.retain(|id| !to_remove.contains(id));
        for id in added {
            if !w.contains(&id) {
                w.push(id);
            }
        }
    }

    /// Install the device-list, default-output and per-device alive listeners
    /// (once); `f` runs on the HAL notification thread for every change.
    pub fn watch(f: fn()) -> Result<(), String> {
        if ON_CHANGE.set(f).is_err() {
            return Ok(());
        }
        unsafe {
            // Deliver on the HAL's own thread instead of the main run loop
            // (which nobody runs in the loader).
            let rl: *const c_void = std::ptr::null();
            let a = addr(kAudioHardwarePropertyRunLoop, kAudioObjectPropertyScopeGlobal);
            let st = AudioObjectSetPropertyData(kAudioObjectSystemObject, &a, 0, std::ptr::null(), std::mem::size_of_val(&rl) as u32, &rl as *const _ as *const c_void);
            if st != 0 {
                log::debug!(target: "audio", "CoreAudio: set kAudioHardwarePropertyRunLoop = NULL failed ({st}); notifications may need a run loop");
            }
            for sel in [kAudioHardwarePropertyDevices, kAudioHardwarePropertyDefaultOutputDevice] {
                check(AudioObjectAddPropertyListener(kAudioObjectSystemObject, &addr(sel, kAudioObjectPropertyScopeGlobal), listener, std::ptr::null_mut()), "AudioObjectAddPropertyListener")?;
            }
        }
        WATCHING.store(true, Ordering::Relaxed);
        let _ = output_devices(); // installs the alive listeners
        Ok(())
    }

    /// Make `id` the system default output device.
    pub fn set_default_output(id: AudioObjectID) -> Result<(), String> {
        let a = addr(kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyScopeGlobal);
        check(unsafe { AudioObjectSetPropertyData(kAudioObjectSystemObject, &a, 0, std::ptr::null(), 4, &id as *const u32 as *const c_void) }, "set DefaultOutputDevice")
    }

    /// Query CoreAudio HAL device master scalar volume `[0.0, 1.0]`.
    pub fn device_volume_scalar(id: AudioObjectID, is_capture: bool) -> Option<f32> {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        unsafe {
            get::<f32>(id, kAudioHardwareServiceDeviceProperty_VirtualMainVolume, scope)
                .or_else(|| get::<f32>(id, kAudioDevicePropertyVolumeScalar, scope))
                .map(|v| v.clamp(0.0, 1.0))
        }
    }

    /// Set CoreAudio HAL device master scalar volume `[0.0, 1.0]`.
    pub fn set_device_volume_scalar(id: AudioObjectID, is_capture: bool, scalar: f32) -> bool {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        let v = scalar.clamp(0.0, 1.0);
        unsafe {
            let a_vm = addr(kAudioHardwareServiceDeviceProperty_VirtualMainVolume, scope);
            if AudioObjectSetPropertyData(id, &a_vm, 0, std::ptr::null(), 4, &v as *const f32 as *const c_void) == 0 {
                return true;
            }
            let a_vs = addr(kAudioDevicePropertyVolumeScalar, scope);
            AudioObjectSetPropertyData(id, &a_vs, 0, std::ptr::null(), 4, &v as *const f32 as *const c_void) == 0
        }
    }

    /// Query CoreAudio HAL per-channel scalar volume `[0.0, 1.0]` (`channel` is 0-based -> CoreAudio element `channel + 1`).
    pub fn device_channel_volume_scalar(id: AudioObjectID, is_capture: bool, channel: u32) -> Option<f32> {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        let a = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyVolumeScalar,
            mScope: scope,
            mElement: channel.saturating_add(1),
        };
        let mut v: f32 = 0.0;
        let mut n = std::mem::size_of::<f32>() as u32;
        if unsafe { AudioObjectGetPropertyData(id, &a, 0, std::ptr::null(), &mut n, &mut v as *mut f32 as *mut c_void) } == 0
            && n == 4
        {
            Some(v.clamp(0.0, 1.0))
        } else {
            None
        }
    }

    /// Set CoreAudio HAL per-channel scalar volume `[0.0, 1.0]` (`channel` is 0-based -> CoreAudio element `channel + 1`).
    pub fn set_device_channel_volume_scalar(id: AudioObjectID, is_capture: bool, channel: u32, scalar: f32) -> bool {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        let a = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyVolumeScalar,
            mScope: scope,
            mElement: channel.saturating_add(1),
        };
        let v = scalar.clamp(0.0, 1.0);
        unsafe { AudioObjectSetPropertyData(id, &a, 0, std::ptr::null(), 4, &v as *const f32 as *const c_void) == 0 }
    }

    /// Query CoreAudio HAL device mute flag.
    pub fn device_mute(id: AudioObjectID, is_capture: bool) -> Option<bool> {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        unsafe { get::<u32>(id, kAudioDevicePropertyMute, scope).map(|m| m != 0) }
    }

    /// Set CoreAudio HAL device mute flag.
    pub fn set_device_mute(id: AudioObjectID, is_capture: bool, mute: bool) -> bool {
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        let m: u32 = u32::from(mute);
        let a = addr(kAudioDevicePropertyMute, scope);
        unsafe { AudioObjectSetPropertyData(id, &a, 0, std::ptr::null(), 4, &m as *const u32 as *const c_void) == 0 }
    }

    /// Query CoreAudio HAL device decibel range `(min_db, max_db)`.
    pub fn device_volume_range_db(id: AudioObjectID, is_capture: bool) -> Option<(f32, f32)> {
        #[repr(C)]
        #[derive(Default)]
        struct AudioValueRange {
            min: f64,
            max: f64,
        }
        let scope = if is_capture { kAudioDevicePropertyScopeInput } else { kAudioDevicePropertyScopeOutput };
        unsafe {
            let r = get::<AudioValueRange>(id, kAudioDevicePropertyVolumeRangeDecibels, scope)?;
            if r.min.is_finite() && r.max.is_finite() && r.max > r.min {
                Some((r.min as f32, r.max as f32))
            } else {
                None
            }
        }
    }

    struct TapIoCtx {
        ring: Arc<Ring>,
        converter: TapFormatConverter,
        non_interleaved: bool,
        src_channels: usize,
        interleave_buf: Vec<f32>,
    }

    /// macOS 14.2+ CoreAudio Process Tap (`AudioHardwareCreateProcessTap` + private
    /// aggregate device + IOProc) for system/endpoint loopback capture.
    /// Dynamically resolved via `dlsym` so older macOS versions degrade cleanly.
    pub struct ProcessTap {
        tap_id: AudioObjectID,
        agg_id: AudioObjectID,
        io_proc_id: *mut c_void,
        /// Heap-allocated context owned by `ProcessTap` and freed strictly AFTER
        /// `AudioDeviceStop` and `AudioDeviceDestroyIOProcID` in `Drop`.
        ctx: *mut TapIoCtx,
        destroy_tap_fn: unsafe extern "C" fn(AudioObjectID) -> OSStatus,
        destroy_agg_fn: unsafe extern "C" fn(AudioObjectID) -> OSStatus,
        stop_fn: unsafe extern "C" fn(AudioObjectID, *mut c_void) -> OSStatus,
        destroy_ioproc_fn: unsafe extern "C" fn(AudioObjectID, *mut c_void) -> OSStatus,
    }
    unsafe impl Send for ProcessTap {}

    impl ProcessTap {
        pub fn open(
            ring: Arc<Ring>,
            device_uid: Option<&str>,
            target_rate: u32,
            target_channels: u32,
        ) -> Result<Self, String> {
            use objc2::msg_send;
            use objc2::runtime::{AnyClass, AnyObject};
            use objc2_foundation::{NSArray, NSString};

            type CreateTapFn = unsafe extern "C" fn(*const AnyObject, *mut AudioObjectID) -> OSStatus;
            type DestroyTapFn = unsafe extern "C" fn(AudioObjectID) -> OSStatus;
            type CreateAggFn = unsafe extern "C" fn(*const c_void, *mut AudioObjectID) -> OSStatus;
            type DestroyAggFn = unsafe extern "C" fn(AudioObjectID) -> OSStatus;
            type IOProc = unsafe extern "C" fn(
                AudioObjectID,
                *const c_void,
                *const AudioBufferList,
                *const c_void,
                *mut AudioBufferList,
                *const c_void,
                *mut c_void,
            ) -> OSStatus;
            type CreateIOProcFn = unsafe extern "C" fn(AudioObjectID, IOProc, *mut c_void, *mut *mut c_void) -> OSStatus;
            type DestroyIOProcFn = unsafe extern "C" fn(AudioObjectID, *mut c_void) -> OSStatus;
            type StartStopFn = unsafe extern "C" fn(AudioObjectID, *mut c_void) -> OSStatus;

            unsafe extern "C" fn tap_ioproc(
                _dev: AudioObjectID,
                _now: *const c_void,
                in_data: *const AudioBufferList,
                _in_time: *const c_void,
                _out_data: *mut AudioBufferList,
                _out_time: *const c_void,
                client_data: *mut c_void,
            ) -> OSStatus {
                if in_data.is_null() || client_data.is_null() {
                    return 0;
                }
                let ctx = &mut *(client_data as *mut TapIoCtx);
                let nbufs = (*in_data).mNumberBuffers as usize;
                if nbufs == 0 {
                    return 0;
                }
                if ctx.non_interleaved {
                    let ch = ctx.src_channels.min(nbufs);
                    let bufs = std::slice::from_raw_parts((*in_data).mBuffers.as_ptr(), ch);
                    let frames = bufs
                        .iter()
                        .map(|b| if b.mData.is_null() { 0 } else { (b.mDataByteSize / 4) as usize })
                        .min()
                        .unwrap_or(0);
                    let cap_frames = ctx.interleave_buf.capacity() / ch.max(1);
                    let frames = frames.min(cap_frames);
                    if frames == 0 {
                        return 0;
                    }
                    ctx.interleave_buf.clear();
                    for f in 0..frames {
                        for b in bufs.iter() {
                            let s = *(b.mData as *const f32).add(f);
                            ctx.interleave_buf.push(s);
                        }
                    }
                    let converted = ctx.converter.convert(&ctx.interleave_buf);
                    if !converted.is_empty() {
                        ctx.ring.push(converted);
                    }
                } else {
                    let b0 = &(*in_data).mBuffers[0];
                    if !b0.mData.is_null() && b0.mDataByteSize >= 4 {
                        let samples = std::slice::from_raw_parts(
                            b0.mData as *const f32,
                            (b0.mDataByteSize / 4) as usize,
                        );
                        let converted = ctx.converter.convert(samples);
                        if !converted.is_empty() {
                            ctx.ring.push(converted);
                        }
                    }
                }
                0
            }

            unsafe {
                let sym = |name: &std::ffi::CStr| -> Result<*mut c_void, String> {
                    let p = libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr());
                    if p.is_null() {
                        Err(format!("CoreAudio symbol {:?} unavailable", name))
                    } else {
                        Ok(p)
                    }
                };
                let create_tap: CreateTapFn = std::mem::transmute(sym(c"AudioHardwareCreateProcessTap")?);
                let destroy_tap: DestroyTapFn = std::mem::transmute(sym(c"AudioHardwareDestroyProcessTap")?);
                let create_agg: CreateAggFn = std::mem::transmute(sym(c"AudioHardwareCreateAggregateDevice")?);
                let destroy_agg: DestroyAggFn = std::mem::transmute(sym(c"AudioHardwareDestroyAggregateDevice")?);
                let create_ioproc: CreateIOProcFn = std::mem::transmute(sym(c"AudioDeviceCreateIOProcID")?);
                let destroy_ioproc: DestroyIOProcFn = std::mem::transmute(sym(c"AudioDeviceDestroyIOProcID")?);
                let start_dev: StartStopFn = std::mem::transmute(sym(c"AudioDeviceStart")?);
                let stop_dev: StartStopFn = std::mem::transmute(sym(c"AudioDeviceStop")?);

                let Some(cls) = AnyClass::get(c"CATapDescription") else {
                    return Err("CATapDescription class unavailable (requires macOS 14.2+)".into());
                };
                let alloc_obj: *mut AnyObject = msg_send![cls, alloc];
                let empty_arr = NSArray::<AnyObject>::new();
                let tap_desc: *mut AnyObject = msg_send![alloc_obj, initStereoGlobalTapButExcludeProcesses: &*empty_arr];
                if tap_desc.is_null() {
                    return Err("CATapDescription init returned null".into());
                }
                let tap_uuid: *mut AnyObject = msg_send![tap_desc, UUID];
                let tap_uid_ns: *mut NSString = msg_send![tap_uuid, UUIDString];
                let tap_uid = if !tap_uid_ns.is_null() {
                    (*tap_uid_ns).to_string()
                } else {
                    String::new()
                };

                let mut tap_id: AudioObjectID = 0;
                let st = create_tap(tap_desc, &mut tap_id);
                let _: () = msg_send![tap_desc, release];
                check(st, "AudioHardwareCreateProcessTap")?;
                if tap_uid.is_empty() {
                    let _ = destroy_tap(tap_id);
                    return Err("CATapDescription UUID empty".into());
                }

                // Create a private aggregate device hosting the process tap.
                #[link(name = "CoreFoundation", kind = "framework")]
                extern "C" {
                    static kCFTypeDictionaryKeyCallBacks: u8;
                    static kCFTypeDictionaryValueCallBacks: u8;
                    static kCFTypeArrayCallBacks: u8;
                    fn CFStringCreateWithCString(alloc: *const c_void, s: *const std::ffi::c_char, enc: u32) -> CFStringRef;
                    fn CFNumberCreate(alloc: *const c_void, ty: isize, v: *const c_void) -> *const c_void;
                    fn CFDictionaryCreate(alloc: *const c_void, k: *const *const c_void, v: *const *const c_void, n: isize, kcb: *const u8, vcb: *const u8) -> *const c_void;
                    fn CFArrayCreate(alloc: *const c_void, v: *const *const c_void, n: isize, cb: *const u8) -> *const c_void;
                }
                let cfstr = |s: &str| -> CFStringRef {
                    let c = std::ffi::CString::new(s).unwrap();
                    CFStringCreateWithCString(std::ptr::null(), c.as_ptr(), kCFStringEncodingUTF8)
                };
                let make_dict = |kv: &[(&str, *const c_void)]| -> *const c_void {
                    let k: Vec<*const c_void> = kv.iter().map(|(k, _)| cfstr(k)).collect();
                    let v: Vec<*const c_void> = kv.iter().map(|(_, v)| *v).collect();
                    let d = CFDictionaryCreate(
                        std::ptr::null(),
                        k.as_ptr(),
                        v.as_ptr(),
                        kv.len() as isize,
                        &kCFTypeDictionaryKeyCallBacks,
                        &kCFTypeDictionaryValueCallBacks,
                    );
                    for p in k.into_iter().chain(v) {
                        CFRelease(p);
                    }
                    d
                };

                let one = 1i32;
                let tap_entry = make_dict(&[("uid", cfstr(&tap_uid))]);
                let tap_list = CFArrayCreate(std::ptr::null(), &tap_entry, 1, &kCFTypeArrayCallBacks);
                CFRelease(tap_entry);
                let agg_uid = format!("port-factory-tap-{}-{}", std::process::id(), tap_id);
                let mut kvs: Vec<(&str, *const c_void)> = vec![
                    ("uid", cfstr(&agg_uid)),
                    ("name", cfstr("Port Factory Loopback Tap")),
                    ("private", CFNumberCreate(std::ptr::null(), 3, &one as *const i32 as *const c_void)),
                    ("tapautostart", CFNumberCreate(std::ptr::null(), 3, &one as *const i32 as *const c_void)),
                    ("taps", tap_list),
                ];
                if let Some(uid) = device_uid {
                    if !uid.is_empty() && uid != "aw2-null" {
                        kvs.push(("master", cfstr(uid)));
                    }
                }
                let desc = make_dict(&kvs);
                let mut agg_id: AudioObjectID = 0;
                let st2 = create_agg(desc, &mut agg_id);
                CFRelease(desc);
                if st2 != 0 {
                    let _ = destroy_tap(tap_id);
                    return Err(format!("AudioHardwareCreateAggregateDevice for tap failed: {st2}"));
                }

                // Query actual ProcessTap stream format (rate, channels, interleaved vs planar, float32)
                // so the IOProc converts channel layout and resamples into the client's capture format.
                let tap_asbd = get::<AudioStreamBasicDescription>(tap_id, kAudioTapPropertyFormat, kAudioObjectPropertyScopeGlobal)
                    .or_else(|| get::<AudioStreamBasicDescription>(agg_id, kAudioDevicePropertyStreamFormat, kAudioDevicePropertyScopeInput));
                let (src_rate, src_channels, non_interleaved) = match tap_asbd {
                    Some(asbd) => {
                        let is_f32 = asbd.mFormatID == kAudioFormatLinearPCM
                            && (asbd.mFormatFlags & kAudioFormatFlagIsFloat) != 0
                            && asbd.mBitsPerChannel == 32;
                        if !is_f32 || asbd.mChannelsPerFrame == 0 || asbd.mSampleRate <= 0.0 {
                            let _ = destroy_agg(agg_id);
                            let _ = destroy_tap(tap_id);
                            return Err(format!(
                                "ProcessTap unsupported stream format (fmt={:#x}, bits={}, ch={}, rate={})",
                                asbd.mFormatID, asbd.mBitsPerChannel, asbd.mChannelsPerFrame, asbd.mSampleRate
                            ));
                        }
                        (
                            asbd.mSampleRate.round() as u32,
                            asbd.mChannelsPerFrame,
                            (asbd.mFormatFlags & kAudioFormatFlagIsNonInterleaved) != 0,
                        )
                    }
                    None => (48_000, 2, false),
                };

                let ctx = Box::into_raw(Box::new(TapIoCtx {
                    ring,
                    converter: TapFormatConverter::new(src_rate, src_channels, target_rate, target_channels),
                    non_interleaved,
                    src_channels: src_channels.clamp(1, 8) as usize,
                    interleave_buf: Vec::with_capacity(4096 * src_channels.clamp(1, 8) as usize),
                }));

                let mut io_proc_id: *mut c_void = std::ptr::null_mut();
                let st3 = create_ioproc(agg_id, tap_ioproc, ctx as *mut c_void, &mut io_proc_id);
                if st3 != 0 {
                    drop(Box::from_raw(ctx));
                    let _ = destroy_agg(agg_id);
                    let _ = destroy_tap(tap_id);
                    return Err(format!("AudioDeviceCreateIOProcID failed: {st3}"));
                }
                let st4 = start_dev(agg_id, io_proc_id);
                if st4 != 0 {
                    let _ = destroy_ioproc(agg_id, io_proc_id);
                    drop(Box::from_raw(ctx));
                    let _ = destroy_agg(agg_id);
                    let _ = destroy_tap(tap_id);
                    return Err(format!("AudioDeviceStart(tap) failed: {st4}"));
                }
                Ok(Self {
                    tap_id,
                    agg_id,
                    io_proc_id,
                    ctx,
                    destroy_tap_fn: destroy_tap,
                    destroy_agg_fn: destroy_agg,
                    stop_fn: stop_dev,
                    destroy_ioproc_fn: destroy_ioproc,
                })
            }
        }
    }

    impl Drop for ProcessTap {
        fn drop(&mut self) {
            unsafe {
                let _ = (self.stop_fn)(self.agg_id, self.io_proc_id);
                let _ = (self.destroy_ioproc_fn)(self.agg_id, self.io_proc_id);
                let _ = (self.destroy_agg_fn)(self.agg_id);
                let _ = (self.destroy_tap_fn)(self.tap_id);
                if !self.ctx.is_null() {
                    drop(Box::from_raw(self.ctx));
                    self.ctx = std::ptr::null_mut();
                }
            }
        }
    }

    /// Process-private aggregate devices (tests: a hot-pluggable device that
    /// exists only for this process and disappears with it).
    #[cfg(test)]
    pub mod aggregate {
        use super::*;

        #[link(name = "CoreAudio", kind = "framework")]
        extern "C" {
            fn AudioHardwareCreateAggregateDevice(desc: *const c_void, out: *mut AudioObjectID) -> OSStatus;
            fn AudioHardwareDestroyAggregateDevice(id: AudioObjectID) -> OSStatus;
        }
        #[link(name = "CoreFoundation", kind = "framework")]
        extern "C" {
            static kCFTypeDictionaryKeyCallBacks: u8;
            static kCFTypeDictionaryValueCallBacks: u8;
            static kCFTypeArrayCallBacks: u8;
            fn CFStringCreateWithCString(alloc: *const c_void, s: *const std::ffi::c_char, enc: u32) -> CFStringRef;
            fn CFNumberCreate(alloc: *const c_void, ty: isize, v: *const c_void) -> *const c_void;
            fn CFDictionaryCreate(alloc: *const c_void, k: *const *const c_void, v: *const *const c_void, n: isize, kcb: *const u8, vcb: *const u8) -> *const c_void;
            fn CFArrayCreate(alloc: *const c_void, v: *const *const c_void, n: isize, cb: *const u8) -> *const c_void;
        }

        unsafe fn cfstr(s: &str) -> CFStringRef {
            let c = std::ffi::CString::new(s).unwrap();
            CFStringCreateWithCString(std::ptr::null(), c.as_ptr(), kCFStringEncodingUTF8)
        }

        unsafe fn dict(kv: &[(&str, *const c_void)]) -> *const c_void {
            let k: Vec<*const c_void> = kv.iter().map(|(k, _)| cfstr(k)).collect();
            let v: Vec<*const c_void> = kv.iter().map(|(_, v)| *v).collect();
            let d = CFDictionaryCreate(std::ptr::null(), k.as_ptr(), v.as_ptr(), kv.len() as isize, &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
            for p in k.into_iter().chain(v) {
                CFRelease(p);
            }
            d
        }

        /// Create a private aggregate device wrapping the device with UID `sub_uid`.
        pub fn create(uid: &str, name: &str, sub_uid: &str) -> Result<AudioObjectID, String> {
            unsafe {
                let one = 1i32;
                let sub = dict(&[("uid", cfstr(sub_uid))]);
                let subs = CFArrayCreate(std::ptr::null(), &sub, 1, &kCFTypeArrayCallBacks);
                CFRelease(sub);
                let desc = dict(&[
                    ("uid", cfstr(uid)),
                    ("name", cfstr(name)),
                    ("private", CFNumberCreate(std::ptr::null(), 3 /* kCFNumberSInt32Type */, &one as *const i32 as *const c_void)),
                    ("subdevices", subs),
                    ("master", cfstr(sub_uid)),
                ]);
                let mut id = 0;
                let st = AudioHardwareCreateAggregateDevice(desc, &mut id);
                CFRelease(desc);
                check(st, "AudioHardwareCreateAggregateDevice")?;
                Ok(id)
            }
        }

        pub fn destroy(id: AudioObjectID) -> Result<(), String> {
            check(unsafe { AudioHardwareDestroyAggregateDevice(id) }, "AudioHardwareDestroyAggregateDevice")
        }
    }

    /// An output unit. Created, started, stopped and disposed on the control thread.
    pub struct Unit {
        au: AudioUnit,
        _shared: Arc<StreamShared>,
    }
    unsafe impl Send for Unit {}

    #[derive(Clone, Copy)]
    pub struct Handle(AudioUnit);
    unsafe impl Send for Handle {}

    unsafe extern "C" fn render(refcon: *mut c_void, flags: *mut u32, _ts: *const c_void, _bus: u32, _frames: u32, data: *mut AudioBufferList) -> OSStatus {
        let s = &*(refcon as *const StreamShared);
        let b = &mut (*data).mBuffers[0];
        if b.mData.is_null() {
            return 0;
        }
        let out = std::slice::from_raw_parts_mut(b.mData as *mut f32, (b.mDataByteSize / 4) as usize);
        if !s.started.load(Ordering::Acquire) {
            out.fill(0.0);
            *flags |= kAudioUnitRenderAction_OutputIsSilence;
            return 0;
        }
        let want = out.len() / s.ring.channels();
        let got = s.ring.pop_into(out);
        if got > 0 {
            if let Some(lb) = &s.loopback {
                if super::super::audioclient::has_loopback_subscribers() {
                    lb.ring.push(&out[..got * s.ring.channels()]);
                }
            }
        }
        s.consumed(want, got);
        0
    }

    fn check(st: OSStatus, what: &str) -> Result<(), String> {
        if st == 0 {
            return Ok(());
        }
        let b = (st as u32).to_be_bytes();
        let cc = if b.iter().all(|c| c.is_ascii_graphic()) { format!(" '{}'", String::from_utf8_lossy(&b)) } else { String::new() };
        Err(format!("{what} failed: OSStatus {st}{cc}"))
    }

    impl Unit {
        /// Returns the unit and the IO slice granted by the device.
        pub fn open(dev: u32, use_default: bool, rate: u32, channels: u16, shared: Arc<StreamShared>, io_request: u32) -> Result<(Unit, u32), String> {
            unsafe {
                let desc = AudioComponentDescription {
                    componentType: kAudioUnitType_Output,
                    componentSubType: if use_default { kAudioUnitSubType_DefaultOutput } else { kAudioUnitSubType_HALOutput },
                    componentManufacturer: kAudioUnitManufacturer_Apple,
                    componentFlags: 0,
                    componentFlagsMask: 0,
                };
                let comp = AudioComponentFindNext(std::ptr::null_mut(), &desc);
                if comp.is_null() {
                    return Err("output AudioComponent not found".into());
                }
                let mut au: AudioUnit = std::ptr::null_mut();
                check(AudioComponentInstanceNew(comp, &mut au), "AudioComponentInstanceNew")?;
                let refcon = Arc::as_ptr(&shared) as *mut c_void;
                let u = Unit { au, _shared: shared }; // disposes on early return
                let set = |id: u32, scope: u32, data: *const c_void, size: usize, what: &str| check(AudioUnitSetProperty(au, id, scope, 0, data, size as u32), what);
                if !use_default {
                    set(kAudioOutputUnitProperty_CurrentDevice, kAudioUnitScope_Global, &dev as *const u32 as *const c_void, 4, "set CurrentDevice")?;
                }
                let asbd = AudioStreamBasicDescription {
                    mSampleRate: rate as f64,
                    mFormatID: kAudioFormatLinearPCM,
                    mFormatFlags: kAudioFormatFlagIsFloat | kAudioFormatFlagIsPacked,
                    mBytesPerPacket: 4 * channels as u32,
                    mFramesPerPacket: 1,
                    mBytesPerFrame: 4 * channels as u32,
                    mChannelsPerFrame: channels as u32,
                    mBitsPerChannel: 32,
                    mReserved: 0,
                };
                set(kAudioUnitProperty_StreamFormat, kAudioUnitScope_Input, &asbd as *const _ as *const c_void, std::mem::size_of_val(&asbd), "set StreamFormat")?;
                // Best effort: the HAL clamps to the device's range.
                if let Err(e) = set(kAudioDevicePropertyBufferFrameSize, kAudioUnitScope_Global, &io_request as *const u32 as *const c_void, 4, "set BufferFrameSize") {
                    log::debug!(target: "audio", "{e} (ignored)");
                }
                let mut granted = io_request;
                let mut sz = 4u32;
                if AudioUnitGetProperty(au, kAudioDevicePropertyBufferFrameSize, kAudioUnitScope_Global, 0, &mut granted as *mut u32 as *mut c_void, &mut sz) != 0 {
                    granted = io_request;
                }
                let maxf = MAX_SLICE;
                if let Err(e) = set(kAudioUnitProperty_MaximumFramesPerSlice, kAudioUnitScope_Global, &maxf as *const u32 as *const c_void, 4, "set MaximumFramesPerSlice") {
                    log::debug!(target: "audio", "{e} (ignored)");
                }
                let cb = AURenderCallbackStruct { inputProc: render, inputProcRefCon: refcon };
                set(kAudioUnitProperty_SetRenderCallback, kAudioUnitScope_Input, &cb as *const _ as *const c_void, std::mem::size_of_val(&cb), "set RenderCallback")?;
                check(AudioUnitInitialize(au), "AudioUnitInitialize")?;
                log::info!(
                    target: "audio",
                    "CoreAudio: {} unit{}: client {rate} Hz {channels} ch float32, IO slice {granted} frames (requested {io_request})",
                    if use_default { "DefaultOutput" } else { "HALOutput" },
                    if use_default { String::new() } else { format!(" on device {dev}") },
                );
                Ok((u, granted))
            }
        }

        pub fn handle(&self) -> Handle {
            Handle(self.au)
        }
    }

    pub fn start(h: Handle) -> Result<(), String> {
        check(unsafe { AudioOutputUnitStart(h.0) }, "AudioOutputUnitStart")
    }

    pub fn stop(h: Handle) {
        unsafe { AudioOutputUnitStop(h.0) };
    }

    impl Drop for Unit {
        fn drop(&mut self) {
            unsafe {
                AudioOutputUnitStop(self.au);
                AudioUnitUninitialize(self.au);
                AudioComponentInstanceDispose(self.au);
            }
        }
    }
}

#[cfg(target_os = "ios")]
mod ca {
    use super::*;
    use std::ffi::c_void;

    type OSStatus = i32;
    pub type AudioUnit = *mut c_void;
    type AudioComponent = *mut c_void;

    const fn fourcc(s: &[u8; 4]) -> u32 {
        ((s[0] as u32) << 24) | ((s[1] as u32) << 16) | ((s[2] as u32) << 8) | s[3] as u32
    }

    const kAudioUnitType_Output: u32 = fourcc(b"auou");
    const kAudioUnitSubType_RemoteIO: u32 = fourcc(b"rioc");
    const kAudioUnitManufacturer_Apple: u32 = fourcc(b"appl");
    const kAudioUnitProperty_StreamFormat: u32 = 8;
    const kAudioUnitProperty_SetRenderCallback: u32 = 23;
    const kAudioUnitProperty_MaximumFramesPerSlice: u32 = 14;
    const kAudioOutputUnitProperty_EnableIO: u32 = 2003;
    const kAudioUnitScope_Global: u32 = 0;
    const kAudioUnitScope_Input: u32 = 1;
    const kAudioUnitScope_Output: u32 = 2;
    const kAudioFormatLinearPCM: u32 = fourcc(b"lpcm");
    const kAudioFormatFlagIsFloat: u32 = 1;
    const kAudioFormatFlagIsPacked: u32 = 8;
    const kAudioUnitRenderAction_OutputIsSilence: u32 = 1 << 4;

    pub const MAX_SLICE: u32 = 4096;

    #[repr(C)]
    struct AudioComponentDescription {
        componentType: u32,
        componentSubType: u32,
        componentManufacturer: u32,
        componentFlags: u32,
        componentFlagsMask: u32,
    }

    #[repr(C)]
    struct AudioStreamBasicDescription {
        mSampleRate: f64,
        mFormatID: u32,
        mFormatFlags: u32,
        mBytesPerPacket: u32,
        mFramesPerPacket: u32,
        mBytesPerFrame: u32,
        mChannelsPerFrame: u32,
        mBitsPerChannel: u32,
        mReserved: u32,
    }

    #[repr(C)]
    struct AudioBuffer {
        mNumberChannels: u32,
        mDataByteSize: u32,
        mData: *mut c_void,
    }

    #[repr(C)]
    struct AudioBufferList {
        mNumberBuffers: u32,
        mBuffers: [AudioBuffer; 1],
    }

    type AURenderCallback =
        unsafe extern "C" fn(refcon: *mut c_void, flags: *mut u32, ts: *const c_void, bus: u32, frames: u32, data: *mut AudioBufferList) -> OSStatus;

    #[repr(C)]
    struct AURenderCallbackStruct {
        inputProc: AURenderCallback,
        inputProcRefCon: *mut c_void,
    }

    #[link(name = "AudioToolbox", kind = "framework")]
    extern "C" {
        fn AudioComponentFindNext(c: AudioComponent, desc: *const AudioComponentDescription) -> AudioComponent;
        fn AudioComponentInstanceNew(c: AudioComponent, out: *mut AudioUnit) -> OSStatus;
        fn AudioComponentInstanceDispose(u: AudioUnit) -> OSStatus;
        fn AudioUnitSetProperty(u: AudioUnit, id: u32, scope: u32, elem: u32, data: *const c_void, size: u32) -> OSStatus;
        fn AudioUnitInitialize(u: AudioUnit) -> OSStatus;
        fn AudioUnitUninitialize(u: AudioUnit) -> OSStatus;
        fn AudioOutputUnitStart(u: AudioUnit) -> OSStatus;
        fn AudioOutputUnitStop(u: AudioUnit) -> OSStatus;
    }

    #[link(name = "AVFAudio", kind = "framework")]
    extern "C" {}

    fn configure_audio_session() -> (u32, u32) {
        use objc2::msg_send;
        use objc2::runtime::{AnyClass, AnyObject, Bool};
        use objc2_foundation::NSString;

        unsafe {
            let Some(cls) = AnyClass::get(c"AVAudioSession") else {
                return (48000, 512);
            };
            let session: *mut AnyObject = msg_send![cls, sharedInstance];
            if session.is_null() {
                return (48000, 512);
            }
            let cat = NSString::from_str("AVAudioSessionCategoryPlayback");
            let mut err: *mut AnyObject = std::ptr::null_mut();
            let _: Bool = msg_send![session, setCategory: &*cat, error: &mut err];
            let mut err2: *mut AnyObject = std::ptr::null_mut();
            let _: Bool = msg_send![session, setActive: Bool::YES, error: &mut err2];
            let sr: f64 = msg_send![session, sampleRate];
            let io_dur: f64 = msg_send![session, IOBufferDuration];
            let rate = if sr.is_finite() && sr >= 8000.0 { sr.round() as u32 } else { 48000 };
            let io_frames = if io_dur.is_finite() && io_dur > 0.0 {
                (io_dur * rate as f64).round().clamp(128.0, 2048.0) as u32
            } else {
                512
            };
            (rate, io_frames)
        }
    }

    pub fn output_devices() -> Vec<DeviceInfo> {
        let (rate, io_frames) = configure_audio_session();
        vec![DeviceInfo {
            id: 1,
            uid: "ios-remoteio-default".into(),
            name: "iOS Audio Output".into(),
            rate,
            channels: 2,
            io_frames,
            latency_frames: io_frames,
            headphones: false,
            is_default: true,
        }]
    }

    pub struct Unit {
        au: AudioUnit,
        _shared: Arc<StreamShared>,
    }
    unsafe impl Send for Unit {}

    #[derive(Clone, Copy)]
    pub struct Handle(AudioUnit);
    unsafe impl Send for Handle {}

    unsafe extern "C" fn render(refcon: *mut c_void, flags: *mut u32, _ts: *const c_void, _bus: u32, _frames: u32, data: *mut AudioBufferList) -> OSStatus {
        let s = &*(refcon as *const StreamShared);
        let b = &mut (*data).mBuffers[0];
        if b.mData.is_null() {
            return 0;
        }
        let out = std::slice::from_raw_parts_mut(b.mData as *mut f32, (b.mDataByteSize / 4) as usize);
        if !s.started.load(Ordering::Acquire) {
            out.fill(0.0);
            *flags |= kAudioUnitRenderAction_OutputIsSilence;
            return 0;
        }
        let want = out.len() / s.ring.channels();
        let got = s.ring.pop_into(out);
        if got > 0 {
            if let Some(lb) = &s.loopback {
                if super::super::audioclient::has_loopback_subscribers() {
                    lb.ring.push(&out[..got * s.ring.channels()]);
                }
            }
        }
        s.consumed(want, got);
        0
    }

    fn check(st: OSStatus, what: &str) -> Result<(), String> {
        if st == 0 {
            return Ok(());
        }
        Err(format!("{what} failed: OSStatus {st}"))
    }

    impl Unit {
        pub fn open(_dev: u32, _use_default: bool, rate: u32, channels: u16, shared: Arc<StreamShared>, io_request: u32) -> Result<(Unit, u32), String> {
            let (_session_rate, granted) = configure_audio_session();
            unsafe {
                let desc = AudioComponentDescription {
                    componentType: kAudioUnitType_Output,
                    componentSubType: kAudioUnitSubType_RemoteIO,
                    componentManufacturer: kAudioUnitManufacturer_Apple,
                    componentFlags: 0,
                    componentFlagsMask: 0,
                };
                let comp = AudioComponentFindNext(std::ptr::null_mut(), &desc);
                if comp.is_null() {
                    return Err("RemoteIO AudioComponent not found".into());
                }
                let mut au: AudioUnit = std::ptr::null_mut();
                check(AudioComponentInstanceNew(comp, &mut au), "AudioComponentInstanceNew")?;
                let refcon = Arc::as_ptr(&shared) as *mut c_void;
                let u = Unit { au, _shared: shared };
                let enable: u32 = 1;
                let _ = AudioUnitSetProperty(au, kAudioOutputUnitProperty_EnableIO, kAudioUnitScope_Output, 0, &enable as *const u32 as *const c_void, 4);
                let asbd = AudioStreamBasicDescription {
                    mSampleRate: rate as f64,
                    mFormatID: kAudioFormatLinearPCM,
                    mFormatFlags: kAudioFormatFlagIsFloat | kAudioFormatFlagIsPacked,
                    mBytesPerPacket: 4 * channels as u32,
                    mFramesPerPacket: 1,
                    mBytesPerFrame: 4 * channels as u32,
                    mChannelsPerFrame: channels as u32,
                    mBitsPerChannel: 32,
                    mReserved: 0,
                };
                check(
                    AudioUnitSetProperty(au, kAudioUnitProperty_StreamFormat, kAudioUnitScope_Input, 0, &asbd as *const _ as *const c_void, std::mem::size_of_val(&asbd) as u32),
                    "set StreamFormat",
                )?;
                let maxf = MAX_SLICE;
                let _ = AudioUnitSetProperty(au, kAudioUnitProperty_MaximumFramesPerSlice, kAudioUnitScope_Global, 0, &maxf as *const u32 as *const c_void, 4);
                let cb = AURenderCallbackStruct { inputProc: render, inputProcRefCon: refcon };
                check(
                    AudioUnitSetProperty(au, kAudioUnitProperty_SetRenderCallback, kAudioUnitScope_Input, 0, &cb as *const _ as *const c_void, std::mem::size_of_val(&cb) as u32),
                    "set RenderCallback",
                )?;
                check(AudioUnitInitialize(au), "AudioUnitInitialize")?;
                let slice = if granted > 0 { granted } else { io_request.max(128) };
                log::info!(target: "audio", "CoreAudio(iOS RemoteIO): client {rate} Hz {channels} ch float32, IO slice {slice} frames");
                Ok((u, slice))
            }
        }

        pub fn handle(&self) -> Handle {
            Handle(self.au)
        }
    }

    pub fn start(h: Handle) -> Result<(), String> {
        check(unsafe { AudioOutputUnitStart(h.0) }, "AudioOutputUnitStart")
    }

    pub fn stop(h: Handle) {
        unsafe { AudioOutputUnitStop(h.0) };
    }

    impl Drop for Unit {
        fn drop(&mut self) {
            unsafe {
                AudioOutputUnitStop(self.au);
                AudioUnitUninitialize(self.au);
                AudioComponentInstanceDispose(self.au);
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use ca::MAX_SLICE;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub const MAX_SLICE: u32 = 4096;

#[cfg(target_os = "macos")]
pub fn default_output_device() -> Option<u32> {
    ca_sync(ca::default_output)
}

/// Call `f` (on a CoreAudio notification thread) whenever the device list,
/// the default output device or a device's alive state changes. CoreAudio
/// backend only; the hook can be installed once per process.
pub fn watch_device_changes(f: fn()) -> Result<(), String> {
    match backend() {
        #[cfg(target_os = "macos")]
        Backend::CoreAudio => ca_sync(move || ca::watch(f)),
        _ => {
            let _ = f;
            Err("no device-change events on this audio backend".into())
        }
    }
}

/// Make the CoreAudio device `id` the system default output (tests / tools).
#[cfg(target_os = "macos")]
pub fn set_default_output_device(id: u32) -> Result<(), String> {
    ca_sync(move || ca::set_default_output(id))
}

#[cfg(all(test, target_os = "macos"))]
pub fn create_private_aggregate(uid: &str, name: &str, sub_uid: &str) -> Result<u32, String> {
    let (uid, name, sub) = (uid.to_string(), name.to_string(), sub_uid.to_string());
    ca_sync(move || ca::aggregate::create(&uid, &name, &sub))
}

#[cfg(all(test, target_os = "macos"))]
pub fn destroy_private_aggregate(id: u32) -> Result<(), String> {
    ca_sync(move || ca::aggregate::destroy(id))
}
