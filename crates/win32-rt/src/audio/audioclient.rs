//! audioclient: `IAudioClient`/`IAudioClient2`/`IAudioClient3` (shared mode),
//! `IAudioRenderClient`, `IAudioCaptureClient`, `IAudioClock`.
//!
//! Exactly the methods Wwise 2022.1.4's `AkSinkCoreAudio` uses are on the hot
//! path (decompiled at 0x141652ce0 Init, 0x1416538d0 stream setup, 0x141653fc0
//! format choice, 0x141653de0 IsDataNeeded, 0x141654310/0x141654b40 submit):
//! GetMixFormat, IsFormatSupported, QI(IAudioClient2), IsOffloadCapable,
//! SetClientProperties (failure is fatal for Wwise), GetDevicePeriod,
//! Initialize(SHARED, EVENTCALLBACK [| AUTOCONVERTPCM | SRC_DEFAULT_QUALITY]),
//! SetEventHandle, GetBufferSize, GetService(IAudioRenderClient), Start,
//! GetCurrentPadding, GetBuffer/ReleaseBuffer (incl. AUDCLNT_BUFFERFLAGS_SILENT
//! prefill; AUDCLNT_E_BUFFER_TOO_LARGE is tolerated by Wwise, any other
//! failure flags the device as lost), Stop on teardown.
//!
//! Capture endpoints (`eCapture`) expose honest silent PCM packets
//! (`AUDCLNT_BUFFERFLAGS_SILENT`) through `IAudioCaptureClient` paced at real
//! wall-clock cadence without opening any macOS microphone input stream.
//!
//! Timing model: the WASAPI buffer is a ring of `buffer_frames` frames;
//! padding = frames queued and not yet pulled by the device (render) or frames
//! waiting to be read (capture). The host device (AUHAL render callback, or
//! the null clock) pulls whatever it needs and signals the client event once
//! per IO cycle (~one 10 ms period).

use super::backend::{self, LoopbackTap, OutputStream, StreamShared};
use super::com::*;
use super::format::{apply_matrix, channel_matrix, to_f32, FormatError, SampleType, WaveFormat, WAVEFORMATEX};
use super::mmdevapi::{new_session_control, new_simple_audio_volume, Endpoint, SessionState};
use super::ring::Ring;
use super::tap::{self, Tap};
use crate::types::*;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::{Duration, Instant};

pub const AUDCLNT_SHAREMODE_SHARED: u32 = 0;
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: u32 = 1;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 0x0001_0000;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 0x0002_0000;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 0x0004_0000;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 0x0008_0000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 0x0010_0000;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 0x0800_0000;
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 0x8000_0000;
pub const AUDCLNT_SESSIONFLAGS_MASK: u32 = 0x7000_0000;
const KNOWN_FLAGS: u32 = AUDCLNT_STREAMFLAGS_CROSSPROCESS
    | AUDCLNT_STREAMFLAGS_LOOPBACK
    | AUDCLNT_STREAMFLAGS_EVENTCALLBACK
    | AUDCLNT_STREAMFLAGS_NOPERSIST
    | AUDCLNT_STREAMFLAGS_RATEADJUST
    | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY
    | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
    | AUDCLNT_SESSIONFLAGS_MASK;
pub const AUDCLNT_BUFFERFLAGS_SILENT: u32 = 0x2;

/// Device period reported by GetDevicePeriod (typical Windows values).
pub const DEFAULT_PERIOD_HNS: i64 = 100_000; // 10 ms
pub const MIN_PERIOD_HNS: i64 = 30_000; // 3 ms

fn frames_for(hns: i64, rate: u32) -> u32 {
    ((hns.max(0) as u128 * rate as u128).div_ceil(10_000_000)) as u32
}

/// Background period timer used only for `EVENTCALLBACK` capture streams so
/// event-driven capture loops wake at real wall-clock stream cadence without
/// opening a macOS microphone stream.
struct CaptureTimer {
    quit: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl CaptureTimer {
    fn spawn(shared: Arc<StreamShared>, period: Duration) -> Result<Self, String> {
        let quit = Arc::new(AtomicBool::new(false));
        let q2 = quit.clone();
        let thread = std::thread::Builder::new()
            .name("wasapi-capture-timer".into())
            .spawn(move || {
                let mut next = Instant::now();
                while !q2.load(Ordering::Acquire) {
                    next += period;
                    let now = Instant::now();
                    if next > now {
                        std::thread::sleep(next - now);
                    } else if now - next > period * 8 {
                        next = now;
                    }
                    if shared.started.load(Ordering::Acquire) && !shared.invalid.load(Ordering::Acquire) {
                        shared.signal_event();
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self { quit, thread: Some(thread) })
    }
}

impl Drop for CaptureTimer {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

struct RenderState {
    ep_id: String,
    source_id: u64,
    out_channels: u16,
    matrix: Option<Vec<f32>>,
    out: OutputStream,
    /// Client-visible buffer handed out by GetBuffer (f32-backed for alignment).
    staging: Vec<f32>,
    tap: Option<Tap>,
    scratch: Vec<f32>,
    scratch2: Vec<f32>,
    /// Queue depth (frames) we want in steady state: 2 device slices + one
    /// Wwise frame, from the granted IO size; grows with observed slices.
    target_base: u32,
    /// Adaptive headroom on (see [`Stream::padding`]); `AW2_AUDIO_LOW_LATENCY=0` disables.
    low_latency: bool,
}

impl Drop for RenderState {
    fn drop(&mut self) {
        unregister_loopback_source(&self.ep_id, self.source_id);
    }
}

/// Maximum shared-mode buffer duration advertised by `IAudioClient2::GetBufferSizeLimits` (2 s).
pub const MAX_BUFFER_DURATION_HNS: i64 = 20_000_000;

static NEXT_LOOPBACK_SOURCE_ID: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(1);

pub(super) fn alloc_loopback_source_id() -> u64 {
    NEXT_LOOPBACK_SOURCE_ID.fetch_add(1, Ordering::Relaxed)
}

struct LoopbackSourceQueue {
    converter: Option<super::backend::TapFormatConverter>,
    /// Converted interleaved float32 samples at `(dst_rate, dst_channels)`.
    buf: VecDeque<f32>,
    /// Set when the owning render stream drops; entry is pruned once `buf` drains.
    retired: bool,
}

struct LoopbackMixer {
    dst_rate: u32,
    dst_channels: usize,
    cap_samples: usize,
    sources: std::collections::HashMap<u64, LoopbackSourceQueue>,
}

impl LoopbackMixer {
    fn new(cap_frames: usize, dst_rate: u32, dst_channels: u16) -> Self {
        let ch = dst_channels.max(1) as usize;
        Self {
            dst_rate: dst_rate.max(1),
            dst_channels: ch,
            cap_samples: cap_frames.max(4096) * ch,
            sources: std::collections::HashMap::new(),
        }
    }

    fn push_source(&mut self, source_id: u64, samples_f32: &[f32], src_rate: u32, src_channels: u16) {
        let src_ch = src_channels.max(1) as usize;
        let src_rate = src_rate.max(1);
        let frames = samples_f32.len() / src_ch;
        if frames == 0 {
            return;
        }
        let input = &samples_f32[..frames * src_ch];
        let dst_rate = self.dst_rate;
        let dst_ch = self.dst_channels;
        let cap_samples = self.cap_samples;

        let entry = self.sources.entry(source_id).or_insert_with(|| LoopbackSourceQueue {
            converter: None,
            buf: VecDeque::new(),
            retired: false,
        });
        entry.retired = false;

        let converted: &[f32] = if src_rate == dst_rate && src_ch == dst_ch {
            input
        } else {
            let needs_new = match entry.converter.as_ref() {
                Some(c) => !c.matches_src(src_rate, src_ch as u32),
                None => true,
            };
            if needs_new {
                entry.converter = Some(super::backend::TapFormatConverter::new(
                    src_rate,
                    src_ch as u32,
                    dst_rate,
                    dst_ch as u32,
                ));
            }
            entry.converter.as_mut().unwrap().convert(input)
        };
        if converted.is_empty() {
            return;
        }
        entry.buf.extend(converted.iter().copied());
        if entry.buf.len() > cap_samples {
            let drop_samples = ((entry.buf.len() - cap_samples) / dst_ch) * dst_ch;
            entry.buf.drain(..drop_samples);
        }
    }

    fn clear(&mut self) {
        self.sources.clear();
    }

    fn retire_source(&mut self, source_id: u64) {
        if let Some(q) = self.sources.get_mut(&source_id) {
            q.retired = true;
            if q.buf.is_empty() {
                self.sources.remove(&source_id);
            }
        }
    }

    /// Mix up to `max_frames` frames across all buffered render sources on a shared timeline
    /// into `out` (interleaved float32 at `(dst_rate, dst_channels)`), returning frames written.
    fn drain_mixed(&mut self, max_frames: usize, out: &mut Vec<f32>) -> usize {
        if max_frames == 0 {
            return 0;
        }
        let ch = self.dst_channels;
        let max_avail_frames = self
            .sources
            .values()
            .map(|q| q.buf.len() / ch)
            .max()
            .unwrap_or(0);
        let take = max_avail_frames.min(max_frames);
        if take == 0 {
            self.sources.retain(|_, q| !q.retired || !q.buf.is_empty());
            return 0;
        }
        let n_samples = take * ch;
        out.clear();
        out.resize(n_samples, 0.0);
        let mut active_count = 0usize;
        for q in self.sources.values_mut() {
            let src_frames = (q.buf.len() / ch).min(take);
            if src_frames == 0 {
                continue;
            }
            active_count += 1;
            let src_samples = src_frames * ch;
            for dst in &mut out[..src_samples] {
                if let Some(s) = q.buf.pop_front() {
                    *dst += s;
                }
            }
        }
        if active_count > 1 {
            for s in &mut out[..n_samples] {
                *s = s.clamp(-1.0, 1.0);
            }
        }
        self.sources.retain(|_, q| !q.retired || !q.buf.is_empty());
        take
    }
}

/// Per-endpoint in-process render loopback subscriber (for `AUDCLNT_STREAMFLAGS_LOOPBACK`
/// fallback when CoreAudio Process Tap is unavailable).
struct LoopbackSub {
    /// True only while the owning loopback capture client is started (`Start`..`Stop`).
    active: AtomicBool,
    /// Serializes all render producers and mixes simultaneous streams on a shared timeline.
    mixer: Mutex<LoopbackMixer>,
}

static LOOPBACK_SUB_COUNT: AtomicUsize = AtomicUsize::new(0);
static LOOPBACK_BUS: Mutex<Vec<(String, Weak<LoopbackSub>)>> = Mutex::new(Vec::new());

impl Drop for LoopbackSub {
    fn drop(&mut self) {
        LOOPBACK_SUB_COUNT.fetch_sub(1, Ordering::Relaxed);
    }
}

pub(super) fn has_loopback_subscribers() -> bool {
    LOOPBACK_SUB_COUNT.load(Ordering::Relaxed) > 0
}

fn register_loopback_sub(ep_id: &str, cap_frames: usize, rate: u32, channels: u16) -> Arc<LoopbackSub> {
    let sub = Arc::new(LoopbackSub {
        active: AtomicBool::new(false),
        mixer: Mutex::new(LoopbackMixer::new(cap_frames, rate, channels)),
    });
    LOOPBACK_SUB_COUNT.fetch_add(1, Ordering::Relaxed);
    let mut bus = LOOPBACK_BUS.lock().unwrap();
    bus.retain(|(_, w)| w.strong_count() > 0);
    bus.push((ep_id.to_string(), Arc::downgrade(&sub)));
    sub
}

pub(super) fn unregister_loopback_source(ep_id: &str, source_id: u64) {
    if LOOPBACK_SUB_COUNT.load(Ordering::Relaxed) == 0 {
        return;
    }
    let subs: Vec<Arc<LoopbackSub>> = {
        let mut bus = LOOPBACK_BUS.lock().unwrap();
        bus.retain(|(_, w)| w.strong_count() > 0);
        bus.iter()
            .filter(|(id, _)| id == ep_id)
            .filter_map(|(_, w)| w.upgrade())
            .collect()
    };
    for sub in subs {
        sub.mixer.lock().unwrap().retire_source(source_id);
    }
}

/// Fast-path render tee: zero lock overhead when no in-process loopback capture client is active.
/// Serializes multi-thread render producers, normalizes sample rate + channel layout per source,
/// and stages frames per `source_id` so simultaneous streams are mixed on a shared timeline.
pub(super) fn push_loopback_frames(
    ep_id: &str,
    source_id: u64,
    samples_f32: &[f32],
    src_rate: u32,
    src_channels: u16,
) {
    if LOOPBACK_SUB_COUNT.load(Ordering::Relaxed) == 0 || samples_f32.is_empty() || src_channels == 0 {
        return;
    }
    let subs: Vec<Arc<LoopbackSub>> = {
        let mut bus = LOOPBACK_BUS.lock().unwrap();
        bus.retain(|(_, w)| w.strong_count() > 0);
        bus.iter()
            .filter(|(id, _)| id == ep_id)
            .filter_map(|(_, w)| w.upgrade())
            .collect()
    };
    for sub in subs {
        if !sub.active.load(Ordering::Acquire) {
            continue;
        }
        sub.mixer
            .lock()
            .unwrap()
            .push_source(source_id, samples_f32, src_rate, src_channels);
    }
}

/// Encode interleaved float32 samples into client PCM format bytes.
fn encode_from_f32(sample: SampleType, src: &[f32], out: &mut Vec<u8>) {
    out.clear();
    match sample {
        SampleType::F32 => {
            out.reserve(src.len() * 4);
            for &v in src {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        SampleType::F64 => {
            out.reserve(src.len() * 8);
            for &v in src {
                out.extend_from_slice(&(v as f64).to_le_bytes());
            }
        }
        SampleType::I16 => {
            out.reserve(src.len() * 2);
            for &v in src {
                let s = (v.clamp(-1.0, 1.0) * 32767.0).round() as i16;
                out.extend_from_slice(&s.to_le_bytes());
            }
        }
        SampleType::I24 => {
            out.reserve(src.len() * 3);
            for &v in src {
                let s = (v.clamp(-1.0, 1.0) * 8_388_607.0).round() as i32;
                let b = s.to_le_bytes();
                out.extend_from_slice(&b[0..3]);
            }
        }
        SampleType::I32 => {
            out.reserve(src.len() * 4);
            for &v in src {
                let s = (v.clamp(-1.0, 1.0) as f64 * 2_147_483_647.0).round() as i32;
                out.extend_from_slice(&s.to_le_bytes());
            }
        }
        SampleType::U8 => {
            out.reserve(src.len());
            for &v in src {
                let s = ((v.clamp(-1.0, 1.0) * 127.5) + 128.0).round().clamp(0.0, 255.0) as u8;
                out.push(s);
            }
        }
    }
}

/// Single selected loopback audio source for a loopback capture stream.
/// Chosen once at `IAudioClient::Initialize` time so a stream never switches
/// dynamically between CoreAudio tap and in-process fallback samples.
enum LoopbackSource {
    CoreAudioTap {
        _tap: LoopbackTap,
        ring: Arc<Ring>,
    },
    InProcessBus(Arc<LoopbackSub>),
}

struct LoopbackCapture {
    source: LoopbackSource,
    scratch_f32: Vec<f32>,
    packets: VecDeque<(Vec<u8>, u32, bool)>,
}

struct CaptureState {
    /// Zero-filled or PCM-populated buffer handed out by `IAudioCaptureClient::GetBuffer`.
    staging: Vec<u8>,
    packet_frames: u32,
    queued_packets: VecDeque<u32>,
    queued_frames: u32,
    last_tick: Option<Instant>,
    frame_accum: f64,
    device_pos: u64,
    loopback: Option<LoopbackCapture>,
    _timer: Option<CaptureTimer>,
}

impl CaptureState {
    /// Advance capture arrival up to `Instant::now()`: for loopback streams,
    /// drains live frames from the selected loopback source (CoreAudio Process Tap
    /// or in-process render loopback mixer); for silent capture or idle loopback,
    /// paces silent packets.
    fn advance(&mut self, fmt: &WaveFormat, buffer_frames: u32) {
        // Gate all capture arrival (including loopback) on the started state (`last_tick.is_some()`).
        // Polling a stopped client must never enqueue new frames or re-arm `last_tick`.
        let Some(prev) = self.last_tick else {
            return;
        };
        let rate = fmt.rate;
        let ch = fmt.channels.max(1) as usize;
        let pkt = self.packet_frames.max(1);

        if let Some(lb) = &mut self.loopback {
            if self.queued_frames < buffer_frames {
                let max_take = (buffer_frames - self.queued_frames) as usize;
                let got: u32 = match &lb.source {
                    LoopbackSource::CoreAudioTap { ring, .. } => {
                        let avail = ring.len().min(max_take);
                        if avail > 0 {
                            lb.scratch_f32.resize(avail * ch, 0.0);
                            ring.pop_into(&mut lb.scratch_f32) as u32
                        } else {
                            0
                        }
                    }
                    LoopbackSource::InProcessBus(sub) => {
                        sub.mixer.lock().unwrap().drain_mixed(max_take, &mut lb.scratch_f32) as u32
                    }
                };
                if got > 0 {
                    let used = got as usize * ch;
                    let silent = lb.scratch_f32[..used].iter().all(|&x| x == 0.0);
                    let mut encoded = Vec::new();
                    encode_from_f32(fmt.sample, &lb.scratch_f32[..used], &mut encoded);
                    lb.packets.push_back((encoded, got, silent));
                    self.queued_packets.push_back(got);
                    self.queued_frames += got;
                    self.last_tick = Some(Instant::now());
                    self.frame_accum = 0.0;
                    return;
                }
            }
        }

        let now = Instant::now();
        let dt = now.saturating_duration_since(prev).as_secs_f64();
        self.last_tick = Some(now);
        self.frame_accum += dt * rate as f64;
        while self.frame_accum >= pkt as f64 {
            self.frame_accum -= pkt as f64;
            if self.queued_frames + pkt <= buffer_frames {
                if let Some(lb) = &mut self.loopback {
                    let byte_len = pkt as usize * fmt.block_align as usize;
                    lb.packets.push_back((vec![0u8; byte_len], pkt, true));
                }
                self.queued_packets.push_back(pkt);
                self.queued_frames += pkt;
            } else {
                // Capture buffer full; stop accumulating until the client drains.
                self.frame_accum = 0.0;
                break;
            }
        }
    }
}

enum StreamIo {
    Render(RenderState),
    Capture(CaptureState),
}

/// An initialized stream (render or capture).
pub struct Stream {
    pub fmt: WaveFormat,
    pub flags: u32,
    pub buffer_frames: u32,
    pub period_frames: u32,
    pub shared: Arc<StreamShared>,
    pub(super) session: Arc<SessionState>,
    pending: Option<u32>,
    started: bool,
    pub frames_written: u64,
    /// Silent frames accepted but not queued (count as played immediately).
    pub dropped_silence: u64,
    io: StreamIo,
}

impl Stream {
    fn is_capture(&self) -> bool {
        matches!(self.io, StreamIo::Capture(_))
    }

    /// Reported padding. For render streams: frames queued and not yet pulled
    /// (with adaptive headroom unless `AW2_AUDIO_LOW_LATENCY=0`). For capture
    /// streams: frames currently available to read in the capture buffer.
    fn padding(&mut self) -> u32 {
        let fmt = self.fmt;
        let buffer_frames = self.buffer_frames;
        let seen = self.shared.max_slice.load(Ordering::Relaxed) as u32;
        match &mut self.io {
            StreamIo::Render(r) => {
                if OFFLINE_DISCARD.load(Ordering::Relaxed) {
                    return 0;
                }
                let len = (self.shared.ring.len() as u32).min(buffer_frames);
                if !r.low_latency || len == 0 {
                    return len;
                }
                let target = r.target_base.max(2 * (seen + 16) + 512).min(buffer_frames);
                (len + buffer_frames - target).min(buffer_frames)
            }
            StreamIo::Capture(c) => {
                c.advance(&fmt, buffer_frames);
                c.queued_frames
            }
        }
    }
}

/// Offline rendering (A2 oracle, `akoracle`): Wwise renders one frame per
/// `RenderAudio` as fast as the caller wants, and only runs its output stage
/// (sink + `StartOutputCapture` + capture callbacks) when the sink reports
/// room. With this set every stream reports padding 0 and submitted frames go
/// to the tap only (not to the device), so every offline frame is output.
static OFFLINE_DISCARD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_offline_discard(on: bool) {
    OFFLINE_DISCARD.store(on, std::sync::atomic::Ordering::Relaxed);
    log::info!(target: "audio", "WASAPI streams: offline discard {}", if on { "ON (padding 0, tap only)" } else { "off" });
}

static STREAMS_INITIALIZED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Number of successful `IAudioClient::Initialize` calls so far.
pub fn streams_initialized() -> u32 {
    STREAMS_INITIALIZED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Initialized streams by endpoint id, for device-loss invalidation.
static LIVE: Mutex<Vec<(String, std::sync::Weak<StreamShared>)>> = Mutex::new(Vec::new());

/// Invalidate every stream on endpoint `id` (device removed / dead / mix
/// format changed): from now on their client calls return
/// AUDCLNT_E_DEVICE_INVALIDATED, which is what makes Wwise flag the device as
/// lost and rebuild its sink. The client event is set once so a thread
/// blocked on it wakes up and sees the error. Returns the number of streams.
pub fn invalidate_endpoint(id: &str, why: &str) -> usize {
    let hit: Vec<Arc<StreamShared>> = {
        let mut g = LIVE.lock().unwrap();
        g.retain(|(_, w)| w.strong_count() > 0);
        g.iter().filter(|(e, _)| e == id).filter_map(|(_, w)| w.upgrade()).collect()
    };
    let mut n = 0;
    for s in hit {
        if !s.invalid.swap(true, std::sync::atomic::Ordering::AcqRel) {
            n += 1;
            s.signal_event();
        }
    }
    if n > 0 {
        log::warn!(target: "audio", "{n} WASAPI stream(s) on {id} invalidated ({why}) -> AUDCLNT_E_DEVICE_INVALIDATED");
    }
    n
}

fn is_invalid(s: &Stream) -> bool {
    s.shared.invalid.load(std::sync::atomic::Ordering::Acquire)
}

/// Promote the thread that services the render client (Wwise's audio
/// thread; it waits on the client event and refills the CoreAudio ring)
/// to a Mach time-constraint thread. On macOS the period matches one
/// 512-frame Wwise tick (`10.67 ms`, `5.33 ms` computation); on iOS
/// RemoteIO delivers 1024-frame slices (`21.33 ms`), so we match the
/// actual IO slice with a generous 75% computation budget so XNU does not
/// demote the audio thread out of realtime during multi-block mixes.
/// Once per thread; not in offline (oracle) rendering.
/// `AW2_AUDIO_RT=0` disables.
fn promote_render_thread(rate: u32, io_slice_frames: u32) {
    thread_local!(static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) });
    if DONE.with(|d| d.get()) || OFFLINE_DISCARD.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    DONE.with(|d| d.set(true));
    if std::env::var("AW2_AUDIO_RT").is_ok_and(|v| v == "0") {
        log::info!(target: "audio", "AW2_AUDIO_RT=0: render thread keeps its normal scheduling");
        return;
    }
    use crate::thread::sched;
    let period_frames = if cfg!(target_os = "ios") {
        io_slice_frames.clamp(512, 2048) as u64
    } else {
        512
    };
    let period_ns = period_frames * 1_000_000_000u64 / rate.max(1) as u64;
    let comp_ns = if cfg!(target_os = "ios") {
        (period_ns * 3) / 4
    } else {
        period_ns / 2
    };
    let tc = sched::time_constraint_for(period_ns, comp_ns, period_ns);
    let (tid, name) = crate::thread::current_thread_object()
        .map(|t| (t.tid.load(std::sync::atomic::Ordering::Relaxed), t.name.lock().unwrap().clone()))
        .unwrap_or_default();
    match sched::set_time_constraint(sched::current_port(), &tc) {
        Ok(()) => log::info!(
            target: "audio",
            "render thread (tid 0x{tid:x} {name:?}) -> Mach time-constraint: period {:.2} ms, computation {:.2} ms, constraint {:.2} ms (abs {:?})",
            period_ns as f64 / 1e6, comp_ns as f64 / 1e6, period_ns as f64 / 1e6, tc
        ),
        Err(kr) => log::warn!(target: "audio", "render thread (tid 0x{tid:x} {name:?}): thread_policy_set(TIME_CONSTRAINT) failed: kern_return {kr}"),
    }
}

#[derive(Default)]
pub struct ClientState {
    pub category: u32,
    pub initializing: bool,
    pub stream: Option<Stream>,
}

pub struct ClientCore {
    pub ep: Arc<Endpoint>,
    pub(super) channel_volumes: Arc<backend::StreamChannelGain>,
    pub st: Mutex<ClientState>,
}

impl Drop for ClientCore {
    fn drop(&mut self) {
        // `get_mut()` requires exclusive ownership: no mutex is held while
        // `Stream` (and its `OutputStream` / `CaptureTimer` / `Tap`) drops.
        if let Some(s) = self.st.get_mut().unwrap().stream.take() {
            match &s.io {
                StreamIo::Render(_) => log::info!(
                    target: "audio",
                    "IAudioClient released: {} frames submitted ({} silent frames not queued), {} underrun frames, {} device callbacks, max slice {} frames",
                    s.frames_written,
                    s.dropped_silence,
                    s.shared.underrun_frames.load(Ordering::Relaxed),
                    s.shared.callbacks.load(Ordering::Relaxed),
                    s.shared.max_slice.load(Ordering::Relaxed)
                ),
                StreamIo::Capture(c) => log::info!(
                    target: "audio",
                    "IAudioClient (capture) released: {} silent frames delivered",
                    c.device_pos
                ),
            }
        }
    }
}

/// Clears `ClientState::initializing` on early return so failed `Initialize`
/// calls never leave the client stuck in an initializing state.
struct InitGuard<'a> {
    core: &'a ClientCore,
    active: bool,
}

impl<'a> Drop for InitGuard<'a> {
    fn drop(&mut self) {
        if self.active {
            self.core.st.lock().unwrap().initializing = false;
        }
    }
}

// ---------------------------------------------------------------------------
// IAudioClient3 (superset vtable of IAudioClient / IAudioClient2)
// ---------------------------------------------------------------------------

type C = ClientObj;

#[repr(C)]
pub struct IAudioClient3Vtbl {
    qi: unsafe extern "win64" fn(*mut C, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut C) -> u32,
    release: unsafe extern "win64" fn(*mut C) -> u32,
    // IAudioClient
    initialize: unsafe extern "win64" fn(*mut C, u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT,
    get_buffer_size: unsafe extern "win64" fn(*mut C, *mut u32) -> HRESULT,
    get_stream_latency: unsafe extern "win64" fn(*mut C, *mut i64) -> HRESULT,
    get_current_padding: unsafe extern "win64" fn(*mut C, *mut u32) -> HRESULT,
    is_format_supported: unsafe extern "win64" fn(*mut C, u32, *const WAVEFORMATEX, *mut *mut WAVEFORMATEX) -> HRESULT,
    get_mix_format: unsafe extern "win64" fn(*mut C, *mut *mut WAVEFORMATEX) -> HRESULT,
    get_device_period: unsafe extern "win64" fn(*mut C, *mut i64, *mut i64) -> HRESULT,
    start: unsafe extern "win64" fn(*mut C) -> HRESULT,
    stop: unsafe extern "win64" fn(*mut C) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut C) -> HRESULT,
    set_event_handle: unsafe extern "win64" fn(*mut C, HANDLE) -> HRESULT,
    get_service: unsafe extern "win64" fn(*mut C, *const GUID, *mut *mut c_void) -> HRESULT,
    // IAudioClient2
    is_offload_capable: unsafe extern "win64" fn(*mut C, u32, *mut BOOL) -> HRESULT,
    set_client_properties: unsafe extern "win64" fn(*mut C, *const u8) -> HRESULT,
    get_buffer_size_limits: unsafe extern "win64" fn(*mut C, *const WAVEFORMATEX, BOOL, *mut i64, *mut i64) -> HRESULT,
    // IAudioClient3
    get_shared_mode_engine_period: unsafe extern "win64" fn(*mut C, *const WAVEFORMATEX, *mut u32, *mut u32, *mut u32, *mut u32) -> HRESULT,
    get_current_shared_mode_engine_period: unsafe extern "win64" fn(*mut C, *mut *mut WAVEFORMATEX, *mut u32) -> HRESULT,
    initialize_shared_audio_stream: unsafe extern "win64" fn(*mut C, u32, u32, *const WAVEFORMATEX, *const GUID) -> HRESULT,
}

pub type ClientObj = ComObj<IAudioClient3Vtbl, Arc<ClientCore>>;

pub fn new_client(ep: Arc<Endpoint>) -> *mut ClientObj {
    let ch = ep.mix_format().channels.max(1) as usize;
    ClientObj::create(
        &CLIENT_VTBL,
        Arc::new(ClientCore {
            ep,
            channel_volumes: backend::StreamChannelGain::new(ch),
            st: Mutex::new(ClientState::default()),
        }),
    )
}

unsafe fn alloc_format(f: &WaveFormat) -> *mut WAVEFORMATEX {
    let b = f.extensible_bytes();
    let p = co_alloc(b.len());
    if !p.is_null() {
        std::ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
    }
    p as *mut WAVEFORMATEX
}

/// Shared-mode Initialize (also used by IAudioClient3::InitializeSharedAudioStream).
/// Never holds `core.st` across `OutputStream::open` / CoreAudio calls / `LIVE`.
pub fn initialize(
    core: &ClientCore,
    mode: u32,
    flags: u32,
    dur: i64,
    periodicity: i64,
    fmt: *const WAVEFORMATEX,
    session_guid: Option<GUID>,
) -> HRESULT {
    {
        let mut st = core.st.lock().unwrap();
        if st.stream.is_some() || st.initializing {
            return AUDCLNT_E_ALREADY_INITIALIZED;
        }
        st.initializing = true;
    }
    let mut guard = InitGuard { core, active: true };

    if !super::devnotify::endpoint_present(&core.ep.id) {
        log::info!(target: "audio", "IAudioClient::Initialize on removed endpoint {} -> AUDCLNT_E_DEVICE_INVALIDATED", core.ep.id);
        return AUDCLNT_E_DEVICE_INVALIDATED;
    }
    if mode == AUDCLNT_SHAREMODE_EXCLUSIVE {
        // Native-unsupported: macOS shared audio endpoints (AirPods/Bluetooth/virtual) and iOS
        // AVAudioSession do not allow exclusive hardware hogging without silencing the entire OS.
        // Matches Windows WASAPI when exclusive-mode access is disallowed on an endpoint
        // (Microsoft Learn: IAudioClient::Initialize -> AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED).
        log::debug!(target: "audio", "IAudioClient::Initialize(EXCLUSIVE) on '{}' -> AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED", core.ep.info.name);
        return AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED;
    }
    if mode != AUDCLNT_SHAREMODE_SHARED || dur < 0 {
        return E_INVALIDARG;
    }
    if flags & !KNOWN_FLAGS != 0 {
        return AUDCLNT_E_INVALID_STREAM_FLAG;
    }
    let is_loopback = flags & AUDCLNT_STREAMFLAGS_LOOPBACK != 0;
    if is_loopback && core.ep.is_capture() {
        return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
    }
    let f = match unsafe { WaveFormat::from_ptr(fmt) } {
        Ok(f) => f,
        Err(FormatError::Null) => return E_POINTER,
        Err(FormatError::Unsupported(why)) => {
            log::warn!(target: "audio", "IAudioClient::Initialize: unsupported format ({why}) -> AUDCLNT_E_UNSUPPORTED_FORMAT");
            return AUDCLNT_E_UNSUPPORTED_FORMAT;
        }
    };
    let mix = core.ep.mix_format();
    let autoconv = flags & AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM != 0;
    if !autoconv && (f.rate != mix.rate || f.channels != mix.channels) {
        log::info!(
            target: "audio",
            "IAudioClient::Initialize: {} Hz/{} ch differs from mix format {} Hz/{} ch without AUTOCONVERTPCM -> AUDCLNT_E_UNSUPPORTED_FORMAT",
            f.rate, f.channels, mix.rate, mix.channels
        );
        return AUDCLNT_E_UNSUPPORTED_FORMAT;
    }
    if periodicity != 0 {
        log::debug!(target: "audio", "IAudioClient::Initialize: shared-mode periodicity {periodicity} ignored");
    }
    let period_frames = frames_for(DEFAULT_PERIOD_HNS, f.rate).max(1);
    let requested = frames_for(dur, f.rate);

    let session = SessionState::get_or_create(core.ep.clone(), session_guid);
    let (shared, stream) = if core.ep.is_capture() || is_loopback {
        let buffer_frames = requested.max(period_frames * 4);
        let shared = StreamShared::new(Ring::new(1, 1));
        let timer = if flags & AUDCLNT_STREAMFLAGS_EVENTCALLBACK != 0 {
            let period_dur = Duration::from_secs_f64(period_frames as f64 / f.rate.max(1) as f64);
            match CaptureTimer::spawn(shared.clone(), period_dur) {
                Ok(t) => Some(t),
                Err(e) => {
                    log::error!(target: "audio", "capture event timer failed to spawn: {e}");
                    return AUDCLNT_E_ENDPOINT_CREATE_FAILED;
                }
            }
        } else {
            None
        };
        let loopback = if is_loopback {
            let ca_ring = Arc::new(Ring::new(buffer_frames as usize * 2, f.channels.max(1) as usize));
            let source = match LoopbackTap::try_open(&core.ep.info, ca_ring.clone(), f.rate, f.channels as u32) {
                Some(tap) => LoopbackSource::CoreAudioTap { _tap: tap, ring: ca_ring },
                None => LoopbackSource::InProcessBus(register_loopback_sub(
                    &core.ep.id,
                    buffer_frames as usize * 2,
                    f.rate,
                    f.channels,
                )),
            };
            Some(LoopbackCapture {
                source,
                scratch_f32: Vec::new(),
                packets: VecDeque::new(),
            })
        } else {
            None
        };
        let staging_bytes = buffer_frames as usize * f.block_align as usize;
        log::info!(
            target: "audio",
            "IAudioClient::Initialize ({}) on '{}': {} Hz {} ch {:?}, flags {flags:#x}, buffer {} frames, packet {} frames",
            if is_loopback { "loopback" } else { "capture" },
            core.ep.info.name, f.rate, f.channels, f.sample, buffer_frames, period_frames
        );
        let stream = Stream {
            fmt: f,
            flags,
            buffer_frames,
            period_frames,
            shared: shared.clone(),
            session,
            pending: None,
            started: false,
            frames_written: 0,
            dropped_silence: 0,
            io: StreamIo::Capture(CaptureState {
                staging: vec![0u8; staging_bytes],
                packet_frames: period_frames,
                queued_packets: VecDeque::new(),
                queued_frames: 0,
                last_tick: None,
                frame_accum: 0.0,
                device_pos: 0,
                loopback,
                _timer: timer,
            }),
        };
        (shared, stream)
    } else {
        let out_channels = if f.channels == mix.channels { f.channels } else { mix.channels };
        let matrix = (out_channels != f.channels).then(|| channel_matrix(f.channels, f.mask, out_channels, mix.mask));
        // The ring is allocated for the worst case (a 4096-frame device slice at
        // display sleep); the client-visible buffer size is decided below.
        let ring_cap = requested.max(2 * backend::MAX_SLICE * f.rate.div_ceil(core.ep.info.rate.max(1)) + 1024);
        let source_id = alloc_loopback_source_id();
        let gain = backend::PlaybackGain::new(
            session.clone(),
            core.channel_volumes.clone(),
            matrix.is_none(),
            super::mmdevapi::endpoint_software_gain_handle(&core.ep.id),
        );
        let shared = StreamShared::with_loopback(
            Ring::new(ring_cap as usize, out_channels as usize),
            core.ep.id.clone(),
            source_id,
            f.rate,
            out_channels,
            Some(gain),
        );
        let io_request = 512; // device frames; ~one Wwise frame
        let out = match OutputStream::open(&core.ep.info, f.rate, out_channels, shared.clone(), io_request) {
            Ok(o) => o,
            Err(e) => {
                log::error!(target: "audio", "audio device '{}' failed to open: {e}; falling back to the silent null device", core.ep.info.name);
                match OutputStream::open(&backend::null_device(), f.rate, out_channels, shared.clone(), period_frames) {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!(target: "audio", "null audio device failed too: {e}");
                        return AUDCLNT_E_ENDPOINT_CREATE_FAILED;
                    }
                }
            }
        };
        // GetBufferSize >= 2 worst-case device slices (MaximumFramesPerSlice =
        // 4096, in client frames + SRC slack) + one Wwise frame, so a 4096-frame
        // slice (display sleep) can always be served while the client refills
        // (docs/design/ADVICE_M2_A1.md §2.1). Steady-state latency is governed by
        // `target` (2 granted slices + 512), see `Stream::padding`.
        let to_client = |dev_frames: u32| (dev_frames as u64 * f.rate as u64).div_ceil(core.ep.info.rate.max(1) as u64) as u32 + 16;
        let slice = to_client(out.io_frames);
        let worst = to_client(backend::MAX_SLICE);
        let floor = std::env::var("AW2_AUDIO_MIN_BUFFER_FRAMES").ok().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let buffer_frames = requested.max(2 * worst + 512).max(floor).min(ring_cap);
        let slice_mult = if cfg!(target_os = "ios") { 4 } else { 2 };
        let target_base = requested.max(slice_mult * slice + 512).min(buffer_frames);
        let low_latency = std::env::var("AW2_AUDIO_LOW_LATENCY").map(|v| v != "0").unwrap_or(true);
        let tap = tap::next_path().and_then(|p| match Tap::create(p.clone(), f) {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!(target: "audio", "audio tap {}: {e}", p.display());
                None
            }
        });
        log::info!(
            target: "audio",
            "IAudioClient::Initialize on '{}': {} Hz {} ch {:?} (mix {} Hz {} ch), flags {flags:#x}, buffer {} frames ({} hns requested), steady-state target {} frames{}, device slice {} frames, backend {:?}{}{}",
            core.ep.info.name, f.rate, f.channels, f.sample, mix.rate, mix.channels, buffer_frames, dur, target_base,
            if low_latency { "" } else { " (AW2_AUDIO_LOW_LATENCY=0: full buffer)" },
            out.io_frames, out.kind,
            if f.rate != core.ep.info.rate { " [host SRC]" } else { "" },
            if matrix.is_some() { " [channel matrix]" } else { "" },
        );
        let staging_len = (buffer_frames as usize * f.block_align as usize).div_ceil(4);
        let stream = Stream {
            fmt: f,
            flags,
            buffer_frames,
            period_frames,
            shared: shared.clone(),
            session,
            pending: None,
            started: false,
            frames_written: 0,
            dropped_silence: 0,
            io: StreamIo::Render(RenderState {
                ep_id: core.ep.id.clone(),
                source_id,
                out_channels,
                matrix,
                out,
                staging: vec![0.0; staging_len],
                tap,
                scratch: Vec::new(),
                scratch2: Vec::new(),
                target_base,
                low_latency,
            }),
        };
        (shared, stream)
    };

    core.channel_volumes.resize(f.channels.max(1) as usize);
    STREAMS_INITIALIZED.fetch_add(1, Ordering::Relaxed);
    LIVE.lock().unwrap().push((core.ep.id.clone(), Arc::downgrade(&shared)));
    {
        let mut st = core.st.lock().unwrap();
        st.stream = Some(stream);
        st.initializing = false;
    }
    guard.active = false;
    S_OK
}

mod client {
    use super::*;
    crate::com_iunknown!(ClientObj, [IID_IAudioClient, IID_IAudioClient2, IID_IAudioClient3]);

    fn core<'a>(this: *mut ClientObj) -> &'a ClientCore {
        unsafe { ClientObj::inner(this) }
    }

    macro_rules! with_stream {
        ($this:expr, |$s:ident| $body:expr) => {{
            let mut st = core($this).st.lock().unwrap();
            match st.stream.as_mut() {
                None => AUDCLNT_E_NOT_INITIALIZED,
                Some(s) if is_invalid(s) => AUDCLNT_E_DEVICE_INVALIDATED,
                Some($s) => $body,
            }
        }};
    }

    pub unsafe extern "win64" fn initialize(this: *mut C, mode: u32, flags: u32, dur: i64, per: i64, fmt: *const WAVEFORMATEX, session: *const GUID) -> HRESULT {
        super::initialize(core(this), mode, flags, dur, per, fmt, GUID::read(session))
    }

    pub unsafe extern "win64" fn get_buffer_size(this: *mut C, out: *mut u32) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        with_stream!(this, |s| write_out(out, s.buffer_frames))
    }

    pub unsafe extern "win64" fn get_stream_latency(this: *mut C, out: *mut i64) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let lat_frames = core(this).ep.info.latency_frames as i64;
        let rate = core(this).ep.info.rate.max(1) as i64;
        with_stream!(this, |_s| write_out(out, DEFAULT_PERIOD_HNS + lat_frames * 10_000_000 / rate))
    }

    pub unsafe extern "win64" fn get_current_padding(this: *mut C, out: *mut u32) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        with_stream!(this, |s| {
            let pad = s.padding();
            write_out(out, pad)
        })
    }

    pub unsafe extern "win64" fn is_format_supported(this: *mut C, mode: u32, fmt: *const WAVEFORMATEX, closest: *mut *mut WAVEFORMATEX) -> HRESULT {
        if fmt.is_null() {
            return E_POINTER;
        }
        if !closest.is_null() {
            std::ptr::write_unaligned(closest, std::ptr::null_mut());
        }
        if mode == AUDCLNT_SHAREMODE_EXCLUSIVE {
            return AUDCLNT_E_UNSUPPORTED_FORMAT; // exclusive mode not offered
        }
        if mode != AUDCLNT_SHAREMODE_SHARED {
            return E_INVALIDARG;
        }
        if closest.is_null() {
            return E_POINTER;
        }
        let f = match WaveFormat::from_ptr(fmt) {
            Ok(f) => f,
            Err(_) => return AUDCLNT_E_UNSUPPORTED_FORMAT,
        };
        let mix = core(this).ep.mix_format();
        if f.rate == mix.rate && f.channels == mix.channels {
            return S_OK;
        }
        let p = alloc_format(&mix);
        if p.is_null() {
            return E_OUTOFMEMORY;
        }
        std::ptr::write_unaligned(closest, p);
        log::debug!(target: "audio", "IsFormatSupported({} Hz {} ch) -> S_FALSE, closest = mix {} Hz {} ch", f.rate, f.channels, mix.rate, mix.channels);
        S_FALSE
    }

    pub unsafe extern "win64" fn get_mix_format(this: *mut C, out: *mut *mut WAVEFORMATEX) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let p = alloc_format(&core(this).ep.mix_format());
        std::ptr::write_unaligned(out, p);
        if p.is_null() {
            E_OUTOFMEMORY
        } else {
            S_OK
        }
    }

    pub unsafe extern "win64" fn get_device_period(_this: *mut C, def: *mut i64, min: *mut i64) -> HRESULT {
        if def.is_null() && min.is_null() {
            return E_POINTER;
        }
        if !def.is_null() {
            std::ptr::write_unaligned(def, DEFAULT_PERIOD_HNS);
        }
        if !min.is_null() {
            std::ptr::write_unaligned(min, MIN_PERIOD_HNS);
        }
        S_OK
    }

    pub unsafe extern "win64" fn start(this: *mut C) -> HRESULT {
        let render_ctrl = {
            let mut st = core(this).st.lock().unwrap();
            let Some(s) = st.stream.as_mut() else {
                return AUDCLNT_E_NOT_INITIALIZED;
            };
            if is_invalid(s) {
                return AUDCLNT_E_DEVICE_INVALIDATED;
            }
            if s.started {
                return AUDCLNT_E_NOT_STOPPED;
            }
            if s.flags & AUDCLNT_STREAMFLAGS_EVENTCALLBACK != 0 && !s.shared.has_event.load(Ordering::Acquire) {
                return AUDCLNT_E_EVENTHANDLE_NOT_SET;
            }
            s.started = true;
            match &mut s.io {
                StreamIo::Render(r) => Some((r.out.control(), s.padding())),
                StreamIo::Capture(c) => {
                    // Discard any loopback audio accumulated while stopped, then enable live capture.
                    if let Some(lb) = &mut c.loopback {
                        match &lb.source {
                            LoopbackSource::CoreAudioTap { ring, .. } => ring.clear(),
                            LoopbackSource::InProcessBus(sub) => {
                                sub.mixer.lock().unwrap().clear();
                                sub.active.store(true, Ordering::Release);
                            }
                        }
                    }
                    c.last_tick = Some(Instant::now());
                    s.shared.started.store(true, Ordering::Release);
                    log::debug!(target: "audio", "IAudioClient::Start (capture)");
                    None
                }
            }
        };
        // Never hold `core.st` across `AudioOutputUnitStart`.
        if let Some((ctrl, pad)) = render_ctrl {
            match ctrl.start() {
                Ok(()) => {
                    log::debug!(target: "audio", "IAudioClient::Start (padding {pad})");
                    S_OK
                }
                Err(e) => {
                    log::error!(target: "audio", "IAudioClient::Start: {e}");
                    if let Some(s) = core(this).st.lock().unwrap().stream.as_mut() {
                        s.started = false;
                    }
                    AUDCLNT_E_DEVICE_INVALIDATED
                }
            }
        } else {
            S_OK
        }
    }

    pub unsafe extern "win64" fn stop(this: *mut C) -> HRESULT {
        let (was, invalid, render_ctrl) = {
            let mut st = core(this).st.lock().unwrap();
            let Some(s) = st.stream.as_mut() else {
                return AUDCLNT_E_NOT_INITIALIZED;
            };
            let was = s.started;
            let invalid = is_invalid(s);
            let ctrl = if was {
                s.started = false;
                let fmt = s.fmt;
                let buf = s.buffer_frames;
                match &mut s.io {
                    StreamIo::Render(r) => Some(r.out.control()),
                    StreamIo::Capture(c) => {
                        c.advance(&fmt, buf);
                        if let Some(lb) = &mut c.loopback {
                            if let LoopbackSource::InProcessBus(sub) = &lb.source {
                                sub.active.store(false, Ordering::Release);
                            }
                        }
                        c.last_tick = None;
                        s.shared.started.store(false, Ordering::Release);
                        None
                    }
                }
            } else {
                None
            };
            (was, invalid, ctrl)
        };
        // Never hold `core.st` across `AudioOutputUnitStop`.
        if let Some(ctrl) = render_ctrl {
            ctrl.stop();
        }
        if invalid {
            AUDCLNT_E_DEVICE_INVALIDATED
        } else if was {
            S_OK
        } else {
            S_FALSE
        }
    }

    pub unsafe extern "win64" fn reset(this: *mut C) -> HRESULT {
        with_stream!(this, |s| {
            if s.started {
                return AUDCLNT_E_NOT_STOPPED;
            }
            if s.pending.is_some() {
                return AUDCLNT_E_BUFFER_OPERATION_PENDING;
            }
            match &mut s.io {
                StreamIo::Render(_) => {
                    if s.shared.ring.is_empty() {
                        return S_FALSE;
                    }
                    s.shared.ring.clear();
                    S_OK
                }
                StreamIo::Capture(c) => {
                    if c.queued_packets.is_empty() && c.queued_frames == 0 && c.frame_accum == 0.0 {
                        return S_FALSE;
                    }
                    c.queued_packets.clear();
                    if let Some(lb) = &mut c.loopback {
                        lb.packets.clear();
                        match &lb.source {
                            LoopbackSource::CoreAudioTap { ring, .. } => ring.clear(),
                            LoopbackSource::InProcessBus(sub) => sub.mixer.lock().unwrap().clear(),
                        }
                    }
                    c.queued_frames = 0;
                    c.frame_accum = 0.0;
                    S_OK
                }
            }
        })
    }

    pub unsafe extern "win64" fn set_event_handle(this: *mut C, h: HANDLE) -> HRESULT {
        let shared = {
            let st = core(this).st.lock().unwrap();
            let Some(s) = st.stream.as_ref() else {
                return AUDCLNT_E_NOT_INITIALIZED;
            };
            if is_invalid(s) {
                return AUDCLNT_E_DEVICE_INVALIDATED;
            }
            if s.flags & AUDCLNT_STREAMFLAGS_EVENTCALLBACK == 0 {
                return AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED;
            }
            s.shared.clone()
        };
        match crate::handles::get_as::<crate::handles::Event>(h) {
            Some(e) => {
                shared.set_event(e);
                S_OK
            }
            None => E_INVALIDARG,
        }
    }

    pub unsafe extern "win64" fn get_service(this: *mut C, riid: *const GUID, out: *mut *mut c_void) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        let Some(iid) = GUID::read(riid) else { return E_POINTER };
        let c = ClientObj::inner(this).clone();
        let (is_capture, session) = {
            let st = c.st.lock().unwrap();
            let Some(s) = st.stream.as_ref() else {
                return AUDCLNT_E_NOT_INITIALIZED;
            };
            (s.is_capture(), s.session.clone())
        };
        if iid == IID_IAudioRenderClient {
            if is_capture {
                AUDCLNT_E_WRONG_ENDPOINT_TYPE
            } else {
                std::ptr::write_unaligned(out, RenderObj::create(&RENDER_VTBL, c) as *mut c_void);
                S_OK
            }
        } else if iid == IID_IAudioCaptureClient {
            if is_capture {
                std::ptr::write_unaligned(out, CaptureObj::create(&CAPTURE_VTBL, c) as *mut c_void);
                S_OK
            } else {
                AUDCLNT_E_WRONG_ENDPOINT_TYPE
            }
        } else if iid == IID_IAudioClock {
            std::ptr::write_unaligned(out, ClockObj::create(&CLOCK_VTBL, c) as *mut c_void);
            S_OK
        } else if iid == IID_IAudioClock2 {
            std::ptr::write_unaligned(out, Clock2Obj::create(&CLOCK2_VTBL, c) as *mut c_void);
            S_OK
        } else if iid == IID_ISimpleAudioVolume {
            std::ptr::write_unaligned(out, new_simple_audio_volume(session));
            S_OK
        } else if iid == IID_IAudioStreamVolume {
            std::ptr::write_unaligned(out, StreamVolumeObj::create(stream_vol::vtbl(), c) as *mut c_void);
            S_OK
        } else if iid == IID_IAudioSessionControl || iid == IID_IAudioSessionControl2 {
            std::ptr::write_unaligned(out, new_session_control(session));
            S_OK
        } else {
            // Standard COM behavior: IAudioClient::GetService returns E_NOINTERFACE for
            // unrecognized service IIDs (Microsoft Learn: IAudioClient::GetService).
            crate::com::warn_qi_failed_once("IAudioClient::GetService", &crate::com::Guid::from_win_guid(iid));
            E_NOINTERFACE
        }
    }

    pub unsafe extern "win64" fn is_offload_capable(_this: *mut C, _cat: u32, out: *mut BOOL) -> HRESULT {
        write_out(out, FALSE)
    }

    /// `AudioClientProperties { UINT32 cbSize; BOOL bIsOffload; AUDIO_STREAM_CATEGORY eCategory; [AUDCLNT_STREAMOPTIONS Options;] }`
    pub unsafe extern "win64" fn set_client_properties(this: *mut C, props: *const u8) -> HRESULT {
        if props.is_null() {
            return E_POINTER;
        }
        let cb = std::ptr::read_unaligned(props as *const u32);
        if cb < 12 {
            return E_INVALIDARG;
        }
        let offload = std::ptr::read_unaligned(props.add(4) as *const i32);
        let cat = std::ptr::read_unaligned(props.add(8) as *const u32);
        let mut st = core(this).st.lock().unwrap();
        if st.stream.is_some() || st.initializing {
            return AUDCLNT_E_ALREADY_INITIALIZED;
        }
        if offload != 0 {
            return AUDCLNT_E_ENDPOINT_CREATE_FAILED; // offload is not capable (IsOffloadCapable says FALSE)
        }
        st.category = cat;
        log::debug!(target: "audio", "IAudioClient2::SetClientProperties(category {cat})");
        S_OK
    }

    pub unsafe extern "win64" fn get_buffer_size_limits(
        _this: *mut C,
        fmt: *const WAVEFORMATEX,
        event_driven: BOOL,
        min_dur: *mut i64,
        max_dur: *mut i64,
    ) -> HRESULT {
        if min_dur.is_null() || max_dur.is_null() {
            return E_POINTER;
        }
        match WaveFormat::from_ptr(fmt) {
            Ok(_) => {}
            Err(FormatError::Null) => return E_POINTER,
            Err(_) => return AUDCLNT_E_UNSUPPORTED_FORMAT,
        }
        let min_hns = if event_driven != 0 { MIN_PERIOD_HNS } else { DEFAULT_PERIOD_HNS };
        std::ptr::write_unaligned(min_dur, min_hns);
        std::ptr::write_unaligned(max_dur, MAX_BUFFER_DURATION_HNS);
        S_OK
    }

    pub unsafe extern "win64" fn get_shared_mode_engine_period(
        this: *mut C,
        fmt: *const WAVEFORMATEX,
        def: *mut u32,
        fund: *mut u32,
        min: *mut u32,
        max: *mut u32,
    ) -> HRESULT {
        if def.is_null() || fund.is_null() || min.is_null() || max.is_null() {
            return E_POINTER;
        }
        let f = match WaveFormat::from_ptr(fmt) {
            Ok(f) => f,
            Err(FormatError::Null) => return E_POINTER,
            Err(_) => return AUDCLNT_E_UNSUPPORTED_FORMAT,
        };
        let _ = this;
        // No low-latency engine periods: the only period is the default one.
        let p = frames_for(DEFAULT_PERIOD_HNS, f.rate);
        std::ptr::write_unaligned(def, p);
        std::ptr::write_unaligned(fund, p);
        std::ptr::write_unaligned(min, p);
        std::ptr::write_unaligned(max, p);
        S_OK
    }

    pub unsafe extern "win64" fn get_current_shared_mode_engine_period(this: *mut C, fmt: *mut *mut WAVEFORMATEX, period: *mut u32) -> HRESULT {
        if fmt.is_null() || period.is_null() {
            return E_POINTER;
        }
        let mix = core(this).ep.mix_format();
        let p = alloc_format(&mix);
        if p.is_null() {
            return E_OUTOFMEMORY;
        }
        std::ptr::write_unaligned(fmt, p);
        std::ptr::write_unaligned(period, frames_for(DEFAULT_PERIOD_HNS, mix.rate));
        S_OK
    }

    pub unsafe extern "win64" fn initialize_shared_audio_stream(this: *mut C, flags: u32, period_frames: u32, fmt: *const WAVEFORMATEX, session: *const GUID) -> HRESULT {
        if let Ok(f) = WaveFormat::from_ptr(fmt) {
            if period_frames != frames_for(DEFAULT_PERIOD_HNS, f.rate) {
                return E_INVALIDARG; // must be one of GetSharedModeEnginePeriod's values
            }
        }
        super::initialize(core(this), AUDCLNT_SHAREMODE_SHARED, flags, 0, 0, fmt, GUID::read(session))
    }
}

static CLIENT_VTBL: IAudioClient3Vtbl = IAudioClient3Vtbl {
    qi: client::qi,
    add_ref: client::add_ref,
    release: client::release,
    initialize: client::initialize,
    get_buffer_size: client::get_buffer_size,
    get_stream_latency: client::get_stream_latency,
    get_current_padding: client::get_current_padding,
    is_format_supported: client::is_format_supported,
    get_mix_format: client::get_mix_format,
    get_device_period: client::get_device_period,
    start: client::start,
    stop: client::stop,
    reset: client::reset,
    set_event_handle: client::set_event_handle,
    get_service: client::get_service,
    is_offload_capable: client::is_offload_capable,
    set_client_properties: client::set_client_properties,
    get_buffer_size_limits: client::get_buffer_size_limits,
    get_shared_mode_engine_period: client::get_shared_mode_engine_period,
    get_current_shared_mode_engine_period: client::get_current_shared_mode_engine_period,
    initialize_shared_audio_stream: client::initialize_shared_audio_stream,
};

// ---------------------------------------------------------------------------
// IAudioRenderClient
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IAudioRenderClientVtbl {
    qi: unsafe extern "win64" fn(*mut RenderObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut RenderObj) -> u32,
    release: unsafe extern "win64" fn(*mut RenderObj) -> u32,
    get_buffer: unsafe extern "win64" fn(*mut RenderObj, u32, *mut *mut u8) -> HRESULT,
    release_buffer: unsafe extern "win64" fn(*mut RenderObj, u32, u32) -> HRESULT,
}

pub type RenderObj = ComObj<IAudioRenderClientVtbl, Arc<ClientCore>>;

/// Push `frames` frames from the staging buffer (or silence) into the ring.
/// Session (`ISimpleAudioVolume`), 1:1 stream-channel (`IAudioStreamVolume`), and
/// software endpoint (`IAudioEndpointVolume`) gains are applied lock-free at playback
/// consumption so volume/mute changes affect already-queued audio immediately.
fn submit(s: &mut Stream, ch_gains: &[f32; 8], frames: u32, flags: u32) {
    let n = frames as usize;
    let bytes = n * s.fmt.block_align as usize;
    let padding_before = s.padding();
    let seen = s.shared.max_slice.load(Ordering::Relaxed) as u32;
    let buffer_frames = s.buffer_frames;
    let fmt = s.fmt;
    let StreamIo::Render(r) = &mut s.io else {
        return;
    };
    if OFFLINE_DISCARD.load(Ordering::Relaxed) {
        if let Some(t) = &mut r.tap {
            let raw: &[u8] = unsafe { std::slice::from_raw_parts(r.staging.as_ptr() as *const u8, bytes) };
            let silent = flags & AUDCLNT_BUFFERFLAGS_SILENT != 0;
            t.submit(frames, flags, padding_before, (!silent).then_some(raw), if silent { bytes } else { 0 });
        }
        s.dropped_silence += n as u64;
        s.frames_written += frames as u64;
        return;
    }
    if flags & AUDCLNT_BUFFERFLAGS_SILENT != 0 {
        if let Some(t) = &mut r.tap {
            t.submit(frames, flags, padding_before, None, bytes);
        }
        // Silence beyond the steady-state target (Wwise's full-buffer silent
        // prefill) is not queued: it counts as played immediately (IAudioClock
        // includes it), so it adds no latency.
        let target = r.target_base.max(2 * (seen + 16) + 512).min(buffer_frames);
        let keep = if r.low_latency { n.min((target as usize).saturating_sub(s.shared.ring.len())) } else { n };
        s.shared.ring.push_silence(keep);
        s.dropped_silence += (n - keep) as u64;
    } else {
        let raw: &[u8] = unsafe { std::slice::from_raw_parts(r.staging.as_ptr() as *const u8, bytes) };
        if let Some(t) = &mut r.tap {
            t.submit(frames, flags, padding_before, Some(raw), 0);
        }
        let pushed: &[f32] = if fmt.sample == SampleType::F32 && r.matrix.is_none() {
            &r.staging[..n * fmt.channels as usize]
        } else {
            if fmt.sample == SampleType::F32 {
                let count = n * fmt.channels as usize;
                r.scratch.clear();
                r.scratch.extend_from_slice(&r.staging[..count]);
            } else {
                to_f32(&fmt, raw, n, &mut r.scratch);
            }
            if let Some(m) = &r.matrix {
                // Pre-downmix per-client-channel gains when channel counts differ; session and
                // endpoint gains still apply at playback consumption.
                let ch = fmt.channels.max(1) as usize;
                if ch_gains.iter().take(ch.min(8)).any(|&g| (g - 1.0).abs() > 1e-6) {
                    for frame in r.scratch.chunks_exact_mut(ch) {
                        for (c, sample) in frame.iter_mut().enumerate() {
                            if c < 8 {
                                *sample *= ch_gains[c];
                            }
                        }
                    }
                }
                apply_matrix(m, ch, r.out_channels as usize, &r.scratch, &mut r.scratch2);
                &r.scratch2
            } else {
                &r.scratch
            }
        };
        s.shared.ring.push(pushed);
    }
    s.frames_written += frames as u64;
}

mod render {
    use super::*;
    crate::com_iunknown!(RenderObj, [IID_IAudioRenderClient]);

    pub unsafe extern "win64" fn get_buffer(this: *mut RenderObj, frames: u32, out: *mut *mut u8) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        let core = RenderObj::inner(this);
        let mut st = core.st.lock().unwrap();
        let Some(s) = st.stream.as_mut() else { return AUDCLNT_E_NOT_INITIALIZED };
        if is_invalid(s) {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        if s.is_capture() {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        }
        let io_slice = match &s.io {
            StreamIo::Render(r) => r.out.io_frames,
            StreamIo::Capture(_) => 512,
        };
        promote_render_thread(s.fmt.rate, io_slice);
        if s.pending.is_some() {
            return AUDCLNT_E_OUT_OF_ORDER;
        }
        let pad = s.padding();
        if frames > s.buffer_frames - pad {
            return AUDCLNT_E_BUFFER_TOO_LARGE;
        }
        s.pending = Some(frames);
        if frames > 0 {
            if let StreamIo::Render(r) = &mut s.io {
                std::ptr::write_unaligned(out, r.staging.as_mut_ptr() as *mut u8);
            }
        }
        S_OK
    }

    pub unsafe extern "win64" fn release_buffer(this: *mut RenderObj, frames: u32, flags: u32) -> HRESULT {
        let core = RenderObj::inner(this);
        let mut st = core.st.lock().unwrap();
        let Some(s) = st.stream.as_mut() else { return AUDCLNT_E_NOT_INITIALIZED };
        if is_invalid(s) {
            s.pending = None;
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        if s.is_capture() {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        }
        let Some(req) = s.pending else { return AUDCLNT_E_OUT_OF_ORDER };
        if frames > req {
            return AUDCLNT_E_INVALID_SIZE;
        }
        if flags & !AUDCLNT_BUFFERFLAGS_SILENT != 0 {
            return E_INVALIDARG;
        }
        s.pending = None;
        if frames > 0 {
            let ch_gains = core.channel_volumes.snapshot();
            submit(s, &ch_gains, frames, flags);
        }
        S_OK
    }
}

static RENDER_VTBL: IAudioRenderClientVtbl = IAudioRenderClientVtbl {
    qi: render::qi,
    add_ref: render::add_ref,
    release: render::release,
    get_buffer: render::get_buffer,
    release_buffer: render::release_buffer,
};

// ---------------------------------------------------------------------------
// IAudioCaptureClient
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IAudioCaptureClientVtbl {
    qi: unsafe extern "win64" fn(*mut CaptureObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut CaptureObj) -> u32,
    release: unsafe extern "win64" fn(*mut CaptureObj) -> u32,
    get_buffer: unsafe extern "win64" fn(*mut CaptureObj, *mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT,
    release_buffer: unsafe extern "win64" fn(*mut CaptureObj, u32) -> HRESULT,
    get_next_packet_size: unsafe extern "win64" fn(*mut CaptureObj, *mut u32) -> HRESULT,
}

pub type CaptureObj = ComObj<IAudioCaptureClientVtbl, Arc<ClientCore>>;

mod capture {
    use super::*;
    crate::com_iunknown!(CaptureObj, [IID_IAudioCaptureClient]);

    pub unsafe extern "win64" fn get_buffer(
        this: *mut CaptureObj,
        pp_data: *mut *mut u8,
        num_frames_out: *mut u32,
        flags_out: *mut u32,
        dev_pos_out: *mut u64,
        qpc_pos_out: *mut u64,
    ) -> HRESULT {
        if pp_data.is_null() || num_frames_out.is_null() || flags_out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(pp_data, std::ptr::null_mut());
        std::ptr::write_unaligned(num_frames_out, 0);
        std::ptr::write_unaligned(flags_out, 0);

        let core = CaptureObj::inner(this);
        let mut st = core.st.lock().unwrap();
        let Some(s) = st.stream.as_mut() else {
            return AUDCLNT_E_NOT_INITIALIZED;
        };
        if is_invalid(s) {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        if s.pending.is_some() {
            return AUDCLNT_E_OUT_OF_ORDER;
        }
        let fmt = s.fmt;
        let buffer_frames = s.buffer_frames;
        let StreamIo::Capture(c) = &mut s.io else {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        };
        c.advance(&fmt, buffer_frames);
        let Some(&pkt_frames) = c.queued_packets.front() else {
            return AUDCLNT_S_BUFFER_EMPTY;
        };
        let mut buf_flags = AUDCLNT_BUFFERFLAGS_SILENT;
        if let Some(lb) = &c.loopback {
            if let Some((bytes, _nf, silent)) = lb.packets.front() {
                if c.staging.len() < bytes.len() {
                    c.staging.resize(bytes.len(), 0);
                }
                c.staging[..bytes.len()].copy_from_slice(bytes);
                if !silent {
                    buf_flags = 0;
                }
            }
        }
        s.pending = Some(pkt_frames);
        std::ptr::write_unaligned(pp_data, c.staging.as_mut_ptr());
        std::ptr::write_unaligned(num_frames_out, pkt_frames);
        std::ptr::write_unaligned(flags_out, buf_flags);
        if !dev_pos_out.is_null() {
            std::ptr::write_unaligned(dev_pos_out, c.device_pos);
        }
        if !qpc_pos_out.is_null() {
            std::ptr::write_unaligned(qpc_pos_out, crate::kernel32::time::interrupt_time_100ns());
        }
        S_OK
    }

    pub unsafe extern "win64" fn release_buffer(this: *mut CaptureObj, num_frames_read: u32) -> HRESULT {
        let core = CaptureObj::inner(this);
        let mut st = core.st.lock().unwrap();
        let Some(s) = st.stream.as_mut() else {
            return AUDCLNT_E_NOT_INITIALIZED;
        };
        if is_invalid(s) {
            s.pending = None;
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        let Some(req) = s.pending else {
            return AUDCLNT_E_OUT_OF_ORDER;
        };
        if num_frames_read != 0 && num_frames_read != req {
            return AUDCLNT_E_INVALID_SIZE;
        }
        let StreamIo::Capture(c) = &mut s.io else {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        };
        s.pending = None;
        if num_frames_read == req {
            if let Some(pkt) = c.queued_packets.pop_front() {
                if let Some(lb) = &mut c.loopback {
                    let _ = lb.packets.pop_front();
                }
                c.queued_frames = c.queued_frames.saturating_sub(pkt);
                c.device_pos += pkt as u64;
            }
        }
        S_OK
    }

    pub unsafe extern "win64" fn get_next_packet_size(this: *mut CaptureObj, out: *mut u32) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let core = CaptureObj::inner(this);
        let mut st = core.st.lock().unwrap();
        let Some(s) = st.stream.as_mut() else {
            return AUDCLNT_E_NOT_INITIALIZED;
        };
        if is_invalid(s) {
            return AUDCLNT_E_DEVICE_INVALIDATED;
        }
        let fmt = s.fmt;
        let buffer_frames = s.buffer_frames;
        let StreamIo::Capture(c) = &mut s.io else {
            return AUDCLNT_E_WRONG_ENDPOINT_TYPE;
        };
        c.advance(&fmt, buffer_frames);
        let next = c.queued_packets.front().copied().unwrap_or(0);
        write_out(out, next)
    }
}

static CAPTURE_VTBL: IAudioCaptureClientVtbl = IAudioCaptureClientVtbl {
    qi: capture::qi,
    add_ref: capture::add_ref,
    release: capture::release,
    get_buffer: capture::get_buffer,
    release_buffer: capture::release_buffer,
    get_next_packet_size: capture::get_next_packet_size,
};

// ---------------------------------------------------------------------------
// IAudioClock
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IAudioClockVtbl {
    qi: unsafe extern "win64" fn(*mut ClockObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut ClockObj) -> u32,
    release: unsafe extern "win64" fn(*mut ClockObj) -> u32,
    get_frequency: unsafe extern "win64" fn(*mut ClockObj, *mut u64) -> HRESULT,
    get_position: unsafe extern "win64" fn(*mut ClockObj, *mut u64, *mut u64) -> HRESULT,
    get_characteristics: unsafe extern "win64" fn(*mut ClockObj, *mut u32) -> HRESULT,
}

pub type ClockObj = ComObj<IAudioClockVtbl, Arc<ClientCore>>;

mod clock {
    use super::*;
    crate::com_iunknown!(ClockObj, [IID_IAudioClock]);

    /// Shared-mode streams report the position in bytes; frequency = byte rate.
    pub unsafe extern "win64" fn get_frequency(this: *mut ClockObj, out: *mut u64) -> HRESULT {
        let st = ClockObj::inner(this).st.lock().unwrap();
        match st.stream.as_ref() {
            None => AUDCLNT_E_NOT_INITIALIZED,
            Some(s) => write_out(out, s.fmt.rate as u64 * s.fmt.block_align as u64),
        }
    }

    pub unsafe extern "win64" fn get_position(this: *mut ClockObj, pos: *mut u64, qpc: *mut u64) -> HRESULT {
        if pos.is_null() {
            return E_POINTER;
        }
        let st = ClockObj::inner(this).st.lock().unwrap();
        match st.stream.as_ref() {
            None => AUDCLNT_E_NOT_INITIALIZED,
            Some(s) if is_invalid(s) => AUDCLNT_E_DEVICE_INVALIDATED,
            Some(s) => {
                let frames = match &s.io {
                    StreamIo::Render(_) => s.shared.ring.consumed() as u64 + s.dropped_silence,
                    StreamIo::Capture(c) => c.device_pos,
                };
                std::ptr::write_unaligned(pos, frames * s.fmt.block_align as u64);
                if !qpc.is_null() {
                    std::ptr::write_unaligned(qpc, crate::kernel32::time::interrupt_time_100ns());
                }
                S_OK
            }
        }
    }

    pub unsafe extern "win64" fn get_characteristics(_this: *mut ClockObj, out: *mut u32) -> HRESULT {
        write_out(out, 0)
    }
}

static CLOCK_VTBL: IAudioClockVtbl = IAudioClockVtbl {
    qi: clock::qi,
    add_ref: clock::add_ref,
    release: clock::release,
    get_frequency: clock::get_frequency,
    get_position: clock::get_position,
    get_characteristics: clock::get_characteristics,
};

// ---------------------------------------------------------------------------
// IAudioClock2
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IAudioClock2Vtbl {
    qi: unsafe extern "win64" fn(*mut Clock2Obj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut Clock2Obj) -> u32,
    release: unsafe extern "win64" fn(*mut Clock2Obj) -> u32,
    get_device_position: unsafe extern "win64" fn(*mut Clock2Obj, *mut u64, *mut u64) -> HRESULT,
}

pub type Clock2Obj = ComObj<IAudioClock2Vtbl, Arc<ClientCore>>;

mod clock2 {
    use super::*;
    crate::com_iunknown!(Clock2Obj, [IID_IAudioClock2]);

    pub unsafe extern "win64" fn get_device_position(
        this: *mut Clock2Obj,
        dev_pos: *mut u64,
        qpc_pos: *mut u64,
    ) -> HRESULT {
        if dev_pos.is_null() {
            return E_POINTER;
        }
        let st = Clock2Obj::inner(this).st.lock().unwrap();
        match st.stream.as_ref() {
            None => AUDCLNT_E_NOT_INITIALIZED,
            Some(s) if is_invalid(s) => AUDCLNT_E_DEVICE_INVALIDATED,
            Some(s) => {
                let frames = match &s.io {
                    StreamIo::Render(_) => s.shared.ring.consumed() as u64 + s.dropped_silence,
                    StreamIo::Capture(c) => c.device_pos,
                };
                std::ptr::write_unaligned(dev_pos, frames);
                if !qpc_pos.is_null() {
                    std::ptr::write_unaligned(qpc_pos, crate::kernel32::time::interrupt_time_100ns());
                }
                S_OK
            }
        }
    }
}

static CLOCK2_VTBL: IAudioClock2Vtbl = IAudioClock2Vtbl {
    qi: clock2::qi,
    add_ref: clock2::add_ref,
    release: clock2::release,
    get_device_position: clock2::get_device_position,
};

// ---------------------------------------------------------------------------
// IAudioStreamVolume
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IAudioStreamVolumeVtbl([usize; 8]);

pub type StreamVolumeObj = ComObj<IAudioStreamVolumeVtbl, Arc<ClientCore>>;

mod stream_vol {
    use super::*;
    crate::com_iunknown!(StreamVolumeObj, [IID_IAudioStreamVolume]);

    pub unsafe extern "win64" fn get_channel_count(this: *mut StreamVolumeObj, out: *mut u32) -> HRESULT {
        let n = StreamVolumeObj::inner(this).channel_volumes.len() as u32;
        write_out(out, n)
    }

    pub unsafe extern "win64" fn set_channel_volume(this: *mut StreamVolumeObj, idx: u32, level: f32) -> HRESULT {
        if !level.is_finite() || !(0.0..=1.0).contains(&level) {
            return E_INVALIDARG;
        }
        if !StreamVolumeObj::inner(this).channel_volumes.set(idx as usize, level) {
            return E_INVALIDARG;
        }
        S_OK
    }
    crate::win64_bridge!(fn set_channel_volume(*mut StreamVolumeObj, u32, f32) -> HRESULT);

    pub unsafe extern "win64" fn get_channel_volume(this: *mut StreamVolumeObj, idx: u32, out: *mut f32) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let Some(v) = StreamVolumeObj::inner(this).channel_volumes.get(idx as usize) else {
            return E_INVALIDARG;
        };
        write_out(out, v)
    }

    pub unsafe extern "win64" fn set_all_volumes(this: *mut StreamVolumeObj, count: u32, levels: *const f32) -> HRESULT {
        if levels.is_null() {
            return E_POINTER;
        }
        let vols = &StreamVolumeObj::inner(this).channel_volumes;
        let n = vols.len();
        if count as usize != n {
            return E_INVALIDARG;
        }
        let mut next = [1.0f32; 8];
        for (i, slot) in next.iter_mut().enumerate().take(n) {
            let v = std::ptr::read_unaligned(levels.add(i));
            if !v.is_finite() || !(0.0..=1.0).contains(&v) {
                return E_INVALIDARG;
            }
            *slot = v;
        }
        vols.set_all(&next[..n]);
        S_OK
    }

    pub unsafe extern "win64" fn get_all_volumes(this: *mut StreamVolumeObj, count: u32, levels: *mut f32) -> HRESULT {
        if levels.is_null() {
            return E_POINTER;
        }
        let vols = &StreamVolumeObj::inner(this).channel_volumes;
        let n = vols.len();
        if count as usize != n {
            return E_INVALIDARG;
        }
        let snap = vols.snapshot();
        for (i, &v) in snap.iter().enumerate().take(n) {
            std::ptr::write_unaligned(levels.add(i), v);
        }
        S_OK
    }

    pub fn vtbl() -> &'static IAudioStreamVolumeVtbl {
        static V: OnceLock<IAudioStreamVolumeVtbl> = OnceLock::new();
        V.get_or_init(|| {
            let slots = [
                qi as *const () as usize,
                add_ref as *const () as usize,
                release as *const () as usize,
                get_channel_count as *const () as usize,
                crate::bridge_entry!(set_channel_volume) as usize,
                get_channel_volume as *const () as usize,
                set_all_volumes as *const () as usize,
                get_all_volumes as *const () as usize,
            ];
            for &a in &slots {
                crate::register_native_va(a as u64, a);
            }
            IAudioStreamVolumeVtbl(slots)
        })
    }
}
