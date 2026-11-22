//! Oracle harness (milestone A2, docs/AUDIO.md §8): drive the ORIGINAL
//! statically linked Wwise offline and deterministically.
//!
//! Both modes boot Wwise like the A1 harness ([`super::harness::boot`]) and
//! load the Init bank while still rendering in REALTIME: the System sink (our
//! IAudioClient) is only created by the audio thread after Init.bnk (which
//! defines the AudioDevice sharesets) is loaded, and never while offline;
//! without a sink there is no output stage, so neither StartOutputCapture nor
//! capture callbacks get data. Once `audioclient::streams_initialized() > 0`
//! and 8 more frames began, [`go_offline`] registers the capture callback
//! (it binds to the devices that exist at call time), then
//! `SetOfflineRendering(true)`, `SetOfflineRenderingFrameTime(spf / rate)`
//! (one audio frame per `RenderAudio`) and `SetRandomSeed(seed)`, committed
//! with one `RenderAudio`; it waits until the audio thread stops beginning
//! frames on its own (BeginRender counter, [`super::akrec::frames`]) and turns
//! the shim's offline discard on. Every later frame is rendered
//! synchronously by our `RenderAudio(true)` calls.
//!
//! Outputs per render: Wwise's own `StartOutputCapture` WAV (16-bit) and a
//! float WAV of the same frames taken with `RegisterCaptureCallback` (the
//! device's final mix, interleaved, before Wwise's float→i16 conversion);
//! both runs cross-check `i16 == trunc(clamp(f) * 32767)` per sample.
//!
//! * **replay** (`AW2_AUDIO_HARNESS_MODE=replay`): play an `AW2_AK_RECORD`
//!   trace. Records up to and including the first LoadBank of a bank with an
//!   `INIT` chunk are applied in realtime (pre-roll, above); the offline
//!   timeline starts at that record's frame. Calls recorded between two
//!   `RenderAudio`s form a batch; batch *k* is applied after rendering up to
//!   the frame stamped on `RenderAudio` *k* (frames counted from
//!   `SoundEngine::Init` in both runs), so the batch is committed to the same
//!   audio frame as in the recording (±1: the realtime audio thread may have
//!   been mid-frame). Init/Term/callback registrations are not replayed;
//!   playing IDs and output device IDs are remapped; banks come from
//!   `<trace>.banks/`.
//! * **events** (`AW2_AUDIO_HARNESS_MODE=events`): load banks, go offline,
//!   post discarded warm-up event(s), then for each event ShortID: reload the
//!   non-Init banks (fresh container state), render until silent, reseed,
//!   start both captures, `PostEvent(id, emitter, AK_EndOfEvent)` (optionally
//!   `StopPlayingID` after N frames), render until the EndOfEvent callback or
//!   a frame cap (+ optional tail), stop captures, `StopAll`, drain. Results
//!   go to `<out>/results.tsv`. Renders are independent of list order.
//!
//! Optional FX bypass (`AW2_AUDIO_HARNESS_BYPASS_FX=1`):
//! `SetBusEffect(bus, slot, 0)` for slots 0..3 of every Bus/AuxBus (HIRC
//! types 0x08/0x12) in every loaded bank, and `SetOutputDeviceEffect` on the
//! main output (Init.bnk's System device has a Mastering Suite). Object-level
//! FX, mixer plugins and actor-mixer FX are untouched.

use super::aktrace::{self, Op, Record};
use super::akrec;
use super::harness::{self, akr, env_or, wide_z, Ak, Boot, AK_SUCCESS};
use std::collections::HashMap;
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

type F<A> = A;

/// AK exports used by the oracle beyond the A1 harness set.
struct Api {
    set_offline: F<unsafe extern "win64" fn(bool) -> i32>,
    set_offline_frame_time: F<unsafe extern "win64" fn(f32) -> i32>,
    set_random_seed: F<unsafe extern "win64" fn(u32)>,
    register_global_cb: akrec::RegisterGlobalCallbackFn,
    register_capture_cb: F<unsafe extern "win64" fn(CaptureCallback, u64, *mut c_void) -> i32>,
    music_defaults: F<unsafe extern "win64" fn(*mut u8)>,
    music_init: F<unsafe extern "win64" fn(*mut u8) -> i32>,
    load_bank_view: F<unsafe extern "win64" fn(*const u8, u32, *mut u32) -> i32>,
    load_bank_copy: F<unsafe extern "win64" fn(*const u8, u32, *mut u32) -> i32>,
    unload_bank: F<unsafe extern "win64" fn(u32, *const u8, u32) -> i32>,
    register_obj: F<unsafe extern "win64" fn(u64, *const u8) -> i32>,
    unregister_obj: F<unsafe extern "win64" fn(u64) -> i32>,
    unregister_all: F<unsafe extern "win64" fn() -> i32>,
    set_position: F<unsafe extern "win64" fn(u64, *const u8, i32) -> i32>,
    set_multi_pos: F<unsafe extern "win64" fn(u64, *const u8, u16, i32, i32) -> i32>,
    set_default_listeners: F<unsafe extern "win64" fn(*const u64, u32) -> i32>,
    set_distance_probe: F<unsafe extern "win64" fn(u64, u64) -> i32>,
    set_scaling: F<unsafe extern "win64" fn(u64, f32) -> i32>,
    set_obs_occ: F<unsafe extern "win64" fn(u64, u64, f32, f32) -> i32>,
    set_aux: F<unsafe extern "win64" fn(u64, *mut u8, u32) -> i32>,
    set_out_bus_vol: F<unsafe extern "win64" fn(u64, u64, f32) -> i32>,
    set_rtpc: F<unsafe extern "win64" fn(u32, f32, u64, i32, i32, bool) -> i32>,
    set_rtpc_pid: F<unsafe extern "win64" fn(u32, f32, u32, i32, i32, bool) -> i32>,
    reset_rtpc: F<unsafe extern "win64" fn(u32, u64, i32, i32, bool) -> i32>,
    set_switch: F<unsafe extern "win64" fn(u32, u32, u64) -> i32>,
    set_state: F<unsafe extern "win64" fn(u32, u32) -> i32>,
    post_trigger: F<unsafe extern "win64" fn(u32, u64) -> i32>,
    post_event: F<unsafe extern "win64" fn(u32, u64, u32, *const c_void, *mut c_void, u32, *const u8, u32) -> u32>,
    exec_action: F<unsafe extern "win64" fn(u32, i32, u64, i32, i32, u32) -> i32>,
    exec_action_name: F<unsafe extern "win64" fn(*const u8, i32, u64, i32, i32, u32) -> i32>,
    seek: F<unsafe extern "win64" fn(u32, u64, i32, bool, u32) -> i32>,
    seek_pct: F<unsafe extern "win64" fn(u32, u64, f32, bool, u32) -> i32>,
    stop_all: F<unsafe extern "win64" fn(u64)>,
    stop_pid: F<unsafe extern "win64" fn(u32, i32, i32)>,
    pin: F<unsafe extern "win64" fn(u32, i8, i8) -> i32>,
    unpin: F<unsafe extern "win64" fn(u32) -> i32>,
    add_output: F<unsafe extern "win64" fn(*const u8, *mut u64, *const u64, u32) -> i32>,
    remove_output: F<unsafe extern "win64" fn(u64) -> i32>,
    set_speaker_angles: F<unsafe extern "win64" fn(*const f32, u32, f32, u64) -> i32>,
    set_bus_effect: F<unsafe extern "win64" fn(u32, u32, u32) -> i32>,
    set_output_device_effect: F<unsafe extern "win64" fn(u64, u32, u32) -> i32>,
    get_output_id: F<unsafe extern "win64" fn(u32, u32) -> u64>,
}

macro_rules! ak {
    ($m:literal) => {
        std::mem::transmute(harness::resolve($m)?)
    };
}

impl Api {
    unsafe fn load() -> Result<Api, String> {
        Ok(Api {
            set_offline: ak!("?SetOfflineRendering@SoundEngine@AK@@YA?AW4AKRESULT@@_N@Z"),
            set_offline_frame_time: ak!("?SetOfflineRenderingFrameTime@SoundEngine@AK@@YA?AW4AKRESULT@@M@Z"),
            set_random_seed: ak!("?SetRandomSeed@SoundEngine@AK@@YAXI@Z"),
            register_global_cb: ak!("?RegisterGlobalCallback@SoundEngine@AK@@YA?AW4AKRESULT@@P6AXPEAVIAkGlobalPluginContext@2@W4AkGlobalCallbackLocation@@PEAX@ZI2W4AkPluginType@@II@Z"),
            register_capture_cb: ak!("?RegisterCaptureCallback@SoundEngine@AK@@YA?AW4AKRESULT@@P6AXAEAVAkAudioBuffer@@_KPEAX@Z12@Z"),
            music_defaults: ak!("?GetDefaultInitSettings@MusicEngine@AK@@YAXAEAUAkMusicSettings@@@Z"),
            music_init: ak!("?Init@MusicEngine@AK@@YA?AW4AKRESULT@@PEAUAkMusicSettings@@@Z"),
            load_bank_view: ak!("?LoadBankMemoryView@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI@Z"),
            load_bank_copy: ak!("?LoadBankMemoryCopy@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI@Z"),
            unload_bank: ak!("?UnloadBank@SoundEngine@AK@@YA?AW4AKRESULT@@IPEBXI@Z"),
            register_obj: ak!("?RegisterGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEBD@Z"),
            unregister_obj: ak!("?UnregisterGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@_K@Z"),
            unregister_all: ak!("?UnregisterAllGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
            set_position: ak!("?SetPosition@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEBVAkWorldTransform@@W4AkSetPositionFlags@@@Z"),
            set_multi_pos: ak!("?SetMultiplePositions@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEBVAkWorldTransform@@GW4MultiPositionType@12@W4AkSetPositionFlags@@@Z"),
            set_default_listeners: ak!("?SetDefaultListeners@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_KI@Z"),
            set_distance_probe: ak!("?SetDistanceProbe@SoundEngine@AK@@YA?AW4AKRESULT@@_K0@Z"),
            set_scaling: ak!("?SetScalingFactor@SoundEngine@AK@@YA?AW4AKRESULT@@_KM@Z"),
            set_obs_occ: ak!("?SetObjectObstructionAndOcclusion@SoundEngine@AK@@YA?AW4AKRESULT@@_K0MM@Z"),
            set_aux: ak!("?SetGameObjectAuxSendValues@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEAUAkAuxSendValue@@I@Z"),
            set_out_bus_vol: ak!("?SetGameObjectOutputBusVolume@SoundEngine@AK@@YA?AW4AKRESULT@@_K0M@Z"),
            set_rtpc: ak!("?SetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@IM_KHW4AkCurveInterpolation@@_N@Z"),
            set_rtpc_pid: ak!("?SetRTPCValueByPlayingID@SoundEngine@AK@@YA?AW4AKRESULT@@IMIHW4AkCurveInterpolation@@_N@Z"),
            reset_rtpc: ak!("?ResetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@I_KHW4AkCurveInterpolation@@_N@Z"),
            set_switch: ak!("?SetSwitch@SoundEngine@AK@@YA?AW4AKRESULT@@II_K@Z"),
            set_state: ak!("?SetState@SoundEngine@AK@@YA?AW4AKRESULT@@II@Z"),
            post_trigger: ak!("?PostTrigger@SoundEngine@AK@@YA?AW4AKRESULT@@I_K@Z"),
            post_event: ak!("?PostEvent@SoundEngine@AK@@YAII_KIP6AXW4AkCallbackType@@PEAUAkCallbackInfo@@@ZPEAXIPEAUAkExternalSourceInfo@@I@Z"),
            exec_action: ak!("?ExecuteActionOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@IW4AkActionOnEventType@12@_KHW4AkCurveInterpolation@@I@Z"),
            exec_action_name: ak!("?ExecuteActionOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDW4AkActionOnEventType@12@_KHW4AkCurveInterpolation@@I@Z"),
            seek: ak!("?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@I_KH_NI@Z"),
            seek_pct: ak!("?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@I_KM_NI@Z"),
            stop_all: ak!("?StopAll@SoundEngine@AK@@YAX_K@Z"),
            stop_pid: ak!("?StopPlayingID@SoundEngine@AK@@YAXIHW4AkCurveInterpolation@@@Z"),
            pin: ak!("?PinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@ICC@Z"),
            unpin: ak!("?UnpinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@I@Z"),
            add_output: ak!("?AddOutput@SoundEngine@AK@@YA?AW4AKRESULT@@AEBUAkOutputSettings@@PEA_KPEB_KI@Z"),
            remove_output: ak!("?RemoveOutput@SoundEngine@AK@@YA?AW4AKRESULT@@_K@Z"),
            set_speaker_angles: ak!("?SetSpeakerAngles@SoundEngine@AK@@YA?AW4AKRESULT@@PEBMIM_K@Z"),
            set_bus_effect: ak!("?SetBusEffect@SoundEngine@AK@@YA?AW4AKRESULT@@III@Z"),
            set_output_device_effect: ak!("?SetOutputDeviceEffect@SoundEngine@AK@@YA?AW4AKRESULT@@_KII@Z"),
            get_output_id: ak!("?GetOutputID@SoundEngine@AK@@YA_KII@Z"),
        })
    }
}

// ---------------------------------------------------------------------------
// Float capture (RegisterCaptureCallback)
// ---------------------------------------------------------------------------

type CaptureCallback = unsafe extern "win64" fn(*const u8, u64, *mut c_void);

struct FloatCap {
    recording: bool,
    channels: u16,
    samples: Vec<f32>,
    callbacks: u64,
    logged: bool,
}

static FLOAT: Mutex<FloatCap> = Mutex::new(FloatCap { recording: false, channels: 0, samples: Vec::new(), callbacks: 0, logged: false });
/// Peak |x| of the last captured buffer (f32 bits), recorded or not.
static LAST_PEAK: AtomicU32 = AtomicU32::new(0);

/// `AkAudioBuffer` (2022.1): pData @0, AkChannelConfig u32 @8 (bits 0..7 =
/// channel count), eState @0xc, uMaxFrames u16 @0x10, uValidFrames u16 @0x12.
/// The buffer handed to capture callbacks is the device's final mix already
/// INTERLEAVED (`data[frame * ch + c]`), unlike the planar buffers effects
/// see: measured against StartOutputCapture, whose i16 samples are exactly
/// `trunc(x * 32767)` of these floats in this order.
unsafe extern "win64" fn on_capture(buf: *const u8, _out: u64, _cookie: *mut c_void) {
    if buf.is_null() {
        return;
    }
    let data = (buf as *const *const f32).read_unaligned();
    let cfg = (buf.add(8) as *const u32).read_unaligned();
    let max = (buf.add(0x10) as *const u16).read_unaligned() as usize;
    let valid = (buf.add(0x12) as *const u16).read_unaligned() as usize;
    let ch = (cfg & 0xff) as usize;
    let mut g = FLOAT.lock().unwrap();
    g.callbacks += 1;
    if !g.logged {
        g.logged = true;
        log::info!(target: "audio", "oracle: capture callback AkAudioBuffer: data {data:p} channelConfig {cfg:#x} ({ch} ch) maxFrames {max} validFrames {valid}");
    }
    if data.is_null() || ch == 0 || ch > 16 || valid > max {
        return;
    }
    let s = std::slice::from_raw_parts(data, valid * ch);
    let peak = s.iter().fold(0f32, |m, v| m.max(v.abs()));
    LAST_PEAK.store(peak.to_bits(), Ordering::Relaxed);
    if g.recording {
        g.channels = ch as u16;
        g.samples.extend_from_slice(s);
    }
}

fn float_start() {
    let mut g = FLOAT.lock().unwrap();
    g.samples.clear();
    g.recording = true;
}

fn float_stop() -> (u16, Vec<f32>) {
    let mut g = FLOAT.lock().unwrap();
    g.recording = false;
    (g.channels.max(1), std::mem::take(&mut g.samples))
}

fn last_peak() -> f32 {
    f32::from_bits(LAST_PEAK.load(Ordering::Relaxed))
}

/// IEEE-float WAV (format tag 3).
fn write_float_wav(path: &Path, channels: u16, rate: u32, samples: &[f32]) -> Result<(), String> {
    let data_len = (samples.len() * 4) as u32;
    let mut b = Vec::with_capacity(44 + data_len as usize);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data_len).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&3u16.to_le_bytes());
    b.extend_from_slice(&channels.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&(rate * channels as u32 * 4).to_le_bytes());
    b.extend_from_slice(&(channels * 4).to_le_bytes());
    b.extend_from_slice(&32u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        b.extend_from_slice(&s.to_le_bytes());
    }
    std::fs::write(path, b).map_err(|e| format!("{}: {e}", path.display()))
}

fn f32_hash(s: &[f32]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for v in s {
        for x in v.to_bits().to_le_bytes() {
            h ^= x as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// FNV-1a 64 of a WAV file's data chunk (0 if unreadable).
fn wav_data_hash(path: &Path) -> (u64, usize) {
    let Ok(b) = std::fs::read(path) else { return (0, 0) };
    let mut pos = 12;
    while pos + 8 <= b.len() {
        let size = u32::from_le_bytes(b[pos + 4..pos + 8].try_into().unwrap()) as usize;
        if &b[pos..pos + 4] == b"data" {
            let d = &b[pos + 8..(pos + 8 + size).min(b.len())];
            return (aktrace::fnv1a64(d), d.len());
        }
        pos += 8 + size + (size & 1);
    }
    (0, 0)
}

/// Cross-check the float capture against the i16 StartOutputCapture file:
/// (samples compared, samples where i16 != trunc(clamp(f) * 32767), length
/// difference in samples).
fn i16_vs_f32(path: &Path, fl: &[f32]) -> (usize, usize, i64) {
    let Ok(b) = std::fs::read(path) else { return (0, 0, -(fl.len() as i64)) };
    let Ok(w) = super::wavcmp::read_wav(&b) else { return (0, 0, -(fl.len() as i64)) };
    // read_wav normalizes i16 to s / 32768.
    let n = w.samples.len().min(fl.len());
    let bad = (0..n).filter(|&k| (w.samples[k] * 32768.0).round() as i32 != (fl[k].clamp(-1.0, 1.0) * 32767.0) as i32).count();
    (n, bad, w.samples.len() as i64 - fl.len() as i64)
}

/// Whether a bank (top-level chunk list) contains chunk `tag`.
pub fn bank_chunk(bank: &[u8], tag: &[u8; 4]) -> bool {
    let mut pos = 0;
    while pos + 8 <= bank.len() {
        if &bank[pos..pos + 4] == tag {
            return true;
        }
        pos += 8 + u32::from_le_bytes(bank[pos + 4..pos + 8].try_into().unwrap()) as usize;
    }
    false
}

/// [`bank_chunk`] on the saved bytes of a recorded LoadBank* record.
fn bank_has_chunk(banks: &Path, r: &Record<'_>, tag: &[u8; 4]) -> bool {
    let mut d = r.dec();
    let (Ok(_ptr), Ok(size), Ok(id), Ok(hash)) = (d.u64(), d.u32(), d.u32(), d.u64()) else { return false };
    std::fs::read(banks.join(aktrace::bank_file_name(id, size, hash))).is_ok_and(|b| bank_chunk(&b, tag))
}

// ---------------------------------------------------------------------------
// Offline session
// ---------------------------------------------------------------------------

/// Register the frame counter (before any realtime pre-roll so it sees
/// every frame).
unsafe fn prepare(api: &Api) -> Result<(), String> {
    let r = akrec::register_frame_counter(api.register_global_cb);
    if r != AK_SUCCESS {
        return Err(format!("RegisterGlobalCallback(frame counter) -> {}", akr(r)));
    }
    Ok(())
}

/// Switch to offline rendering: frame time, seed. Call after the Init bank
/// is loaded (see below).
unsafe fn go_offline(ak: &Ak, api: &Api, b: &Boot, seed: u32) -> Result<(), String> {
    let ft = b.spf as f32 / b.rate as f32;
    // The System sink (our IAudioClient) is created by the audio thread only
    // once Init.bnk (which defines the AudioDevice sharesets) is loaded, and
    // only while rendering in realtime; offline mode never Activates it, and
    // without a sink Wwise skips its output stage (no StartOutputCapture data,
    // no capture callbacks). Run realtime until the stream is initialized and
    // the audio thread has begun a few more frames, then switch.
    (ak.render_audio)(true);
    let t0 = Instant::now();
    let mut f_init = None;
    let mut n = 0u32;
    loop {
        if f_init.is_none() && super::audioclient::streams_initialized() > 0 {
            f_init = Some(akrec::frames());
        }
        if f_init.is_some_and(|f| akrec::frames() >= f + 8) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        n += 1;
        if n % 4 == 0 {
            (ak.render_audio)(true);
        }
        if t0.elapsed() > Duration::from_secs(10) {
            return Err(format!(
                "no output sink after 10 s ({} frames, {} IAudioClient streams initialized; was an Init bank loaded?)",
                akrec::frames(),
                super::audioclient::streams_initialized()
            ));
        }
    }
    // RegisterCaptureCallback binds synchronously to the output device that
    // exists at call time; register only now that the real sink is up.
    let r = (api.register_capture_cb)(on_capture, 0, std::ptr::null_mut());
    if r != AK_SUCCESS {
        return Err(format!("RegisterCaptureCallback -> {}", akr(r)));
    }
    for (what, r) in [("SetOfflineRendering(true)", (api.set_offline)(true)), ("SetOfflineRenderingFrameTime", (api.set_offline_frame_time)(ft))] {
        if r != AK_SUCCESS {
            return Err(format!("{what} -> {}", akr(r)));
        }
    }
    (api.set_random_seed)(seed);
    (ak.render_audio)(true);
    // Wait for the audio thread to process the switch: it stops beginning frames.
    let t0 = Instant::now();
    let mut last = akrec::frames();
    let mut stable = 0;
    while stable < 6 {
        std::thread::sleep(Duration::from_millis(20));
        let f = akrec::frames();
        if f == last {
            stable += 1;
        } else {
            stable = 0;
            last = f;
        }
        if t0.elapsed() > Duration::from_secs(5) {
            return Err("the audio thread kept rendering after SetOfflineRendering(true)".into());
        }
    }
    super::audioclient::set_offline_discard(true);
    let f0 = akrec::frames();
    (ak.render_audio)(true);
    let f1 = akrec::frames();
    log::info!(
        target: "audio",
        "oracle: offline rendering on after {} realtime frames (frame time {ft} s = {} samples, seed {seed:#x}); one RenderAudio -> {} frame(s)",
        f0,
        b.spf,
        f1 - f0
    );
    if f1 != f0 + 1 {
        return Err(format!("offline RenderAudio rendered {} frames, expected 1", f1 - f0));
    }
    Ok(())
}

/// Render exactly one offline frame.
unsafe fn render(ak: &Ak) {
    (ak.render_audio)(true);
}

/// Render until 8 consecutive silent output frames (or `max` frames);
/// returns whether silence was reached.
unsafe fn drain(ak: &Ak, max: u64) -> bool {
    let mut quiet = 0;
    for _ in 0..max.max(8) {
        render(ak);
        quiet = if last_peak() == 0.0 { quiet + 1 } else { 0 };
        if quiet >= 8 {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Bus FX bypass
// ---------------------------------------------------------------------------

/// IDs of Bus (0x08) and AuxBus (0x12) objects in a bank's HIRC chunk.
pub fn hirc_bus_ids(bank: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos + 8 <= bank.len() {
        let size = u32::from_le_bytes(bank[pos + 4..pos + 8].try_into().unwrap()) as usize;
        let body = pos + 8;
        if &bank[pos..pos + 4] == b"HIRC" && body + 4 <= bank.len() {
            let n = u32::from_le_bytes(bank[body..body + 4].try_into().unwrap()) as usize;
            let mut p = body + 4;
            for _ in 0..n {
                if p + 9 > bank.len() {
                    break;
                }
                let ty = bank[p];
                let sz = u32::from_le_bytes(bank[p + 1..p + 5].try_into().unwrap()) as usize;
                let id = u32::from_le_bytes(bank[p + 5..p + 9].try_into().unwrap());
                if ty == 0x08 || ty == 0x12 {
                    out.push(id);
                }
                p += 5 + sz;
            }
        }
        pos = body + size;
    }
    out
}

/// `AK_NUM_EFFECTS_PER_OBJ`.
const FX_SLOTS: u32 = 4;

unsafe fn bypass_bus_fx(api: &Api, bank: &[u8]) -> (usize, usize) {
    let ids = hirc_bus_ids(bank);
    let mut failed = 0;
    for &id in &ids {
        for slot in 0..FX_SLOTS {
            if (api.set_bus_effect)(id, slot, 0) != AK_SUCCESS {
                failed += 1;
            }
        }
    }
    (ids.len(), failed)
}

/// Remove the main output device's FX (Init.bnk's System AudioDevice carries
/// a Mastering Suite). `GetOutputID(0, 0)` resolves the live main device's
/// id (wildcard lookup); SetOutputDeviceEffect is a queued message that
/// always returns success. Call once the sink exists.
unsafe fn bypass_device_fx(api: &Api) {
    let out = (api.get_output_id)(0, 0);
    for s in 0..FX_SLOTS {
        (api.set_output_device_effect)(out, s, 0);
    }
    log::info!(target: "audio", "oracle: SetOutputDeviceEffect(output {out:#x}, slots 0..{FX_SLOTS}, none)");
}

fn bypass_enabled() -> bool {
    env_or("AW2_AUDIO_HARNESS_BYPASS_FX", "0") == "1"
}

fn seed() -> u32 {
    harness::parse_id(&env_or("AW2_AK_SEED", "0x5eed")).unwrap_or(0x5eed)
}

unsafe fn music_init(api: &Api) {
    let mut m = [0u8; 64];
    (api.music_defaults)(m.as_mut_ptr());
    let r = (api.music_init)(m.as_mut_ptr());
    log::info!(target: "audio", "oracle: MusicEngine::Init -> {}", akr(r));
}

/// 16-byte aligned copy of a bank (LoadBankMemoryView requires
/// AK_BANK_PLATFORM_DATA_ALIGNMENT = 16 and the memory to outlive the bank).
struct Aligned(Vec<u128>, usize);

impl Aligned {
    fn new(b: &[u8]) -> Aligned {
        let mut v = vec![0u128; b.len().div_ceil(16)];
        unsafe { std::ptr::copy_nonoverlapping(b.as_ptr(), v.as_mut_ptr() as *mut u8, b.len()) };
        Aligned(v, b.len())
    }
    fn ptr(&self) -> *const u8 {
        self.0.as_ptr() as *const u8
    }
    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.1) }
    }
}

unsafe extern "win64" fn noop_callback(_ty: u32, _info: *mut c_void) {}

static ENDED: AtomicBool = AtomicBool::new(false);
static ENDED_PID: AtomicU32 = AtomicU32::new(0);

/// `AkCallbackFunc`: AK_EndOfEvent = 1; `AkEventCallbackInfo.playingID` @0x10.
unsafe extern "win64" fn end_callback(ty: u32, info: *mut c_void) {
    if ty == 1 && !info.is_null() {
        let pid = ((info as *const u8).add(0x10) as *const u32).read_unaligned();
        if pid == ENDED_PID.load(Ordering::Relaxed) {
            ENDED.store(true, Ordering::Relaxed);
        }
    }
}

// ---------------------------------------------------------------------------
// Replay
// ---------------------------------------------------------------------------

#[derive(Default)]
struct ReplayState {
    pids: HashMap<u32, u32>,
    outputs: HashMap<u64, u64>,
    views: HashMap<u64, Aligned>,
    applied: usize,
    mismatches: usize,
    errors: usize,
    skipped: HashMap<Op, usize>,
}

impl ReplayState {
    fn pid(&self, p: u32) -> u32 {
        if p == 0 {
            0
        } else {
            self.pids.get(&p).copied().unwrap_or(p)
        }
    }
    fn out(&self, d: u64) -> u64 {
        if d == 0 {
            0
        } else {
            self.outputs.get(&d).copied().unwrap_or(d)
        }
    }
}

unsafe fn apply(api: &Api, st: &mut ReplayState, banks: &Path, bypass: bool, r: &Record<'_>) -> Result<(), String> {
    let mut d = r.dec();
    let res: u32 = match r.op {
        Op::Init | Op::Term | Op::MusicInit | Op::MusicTerm | Op::RegisterGlobalCallback | Op::RegisterBusVolumeCallback | Op::RenderAudio => {
            *st.skipped.entry(r.op).or_default() += 1;
            return Ok(());
        }
        Op::RegisterGameObj => (api.register_obj)(d.u64()?, std::ptr::null()) as u32,
        Op::UnregisterGameObj => (api.unregister_obj)(d.u64()?) as u32,
        Op::UnregisterAllGameObj => (api.unregister_all)() as u32,
        Op::SetPosition => {
            let obj = d.u64()?;
            let t = d.take(aktrace::WORLD_TRANSFORM_LEN)?;
            (api.set_position)(obj, t.as_ptr(), d.i32()?) as u32
        }
        Op::SetMultiplePositions => {
            let (obj, n, ty, flags) = (d.u64()?, d.u16()?, d.i32()?, d.i32()?);
            let t = d.take(n as usize * aktrace::WORLD_TRANSFORM_LEN)?.to_vec();
            (api.set_multi_pos)(obj, if n == 0 { std::ptr::null() } else { t.as_ptr() }, n, ty, flags) as u32
        }
        Op::SetDefaultListeners => {
            let n = d.u32()?;
            let l: Vec<u64> = (0..n).map(|_| d.u64()).collect::<Result<_, _>>()?;
            (api.set_default_listeners)(l.as_ptr(), n) as u32
        }
        Op::SetDistanceProbe => (api.set_distance_probe)(d.u64()?, d.u64()?) as u32,
        Op::SetScalingFactor => (api.set_scaling)(d.u64()?, d.f32()?) as u32,
        Op::SetObjectObstructionAndOcclusion => (api.set_obs_occ)(d.u64()?, d.u64()?, d.f32()?, d.f32()?) as u32,
        Op::SetGameObjectAuxSendValues => {
            let (obj, n) = (d.u64()?, d.u32()?);
            let mut v = d.take(n as usize * aktrace::AUX_SEND_LEN)?.to_vec();
            (api.set_aux)(obj, v.as_mut_ptr(), n) as u32
        }
        Op::SetGameObjectOutputBusVolume => (api.set_out_bus_vol)(d.u64()?, d.u64()?, d.f32()?) as u32,
        Op::SetRTPCValue => (api.set_rtpc)(d.u32()?, d.f32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?) as u32,
        Op::SetRTPCValueByPlayingID => {
            let (id, v, pid) = (d.u32()?, d.f32()?, d.u32()?);
            (api.set_rtpc_pid)(id, v, st.pid(pid), d.i32()?, d.i32()?, d.bool()?) as u32
        }
        Op::ResetRTPCValue => (api.reset_rtpc)(d.u32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?) as u32,
        Op::SetSwitch => (api.set_switch)(d.u32()?, d.u32()?, d.u64()?) as u32,
        Op::SetState => (api.set_state)(d.u32()?, d.u32()?) as u32,
        Op::PostTrigger => (api.post_trigger)(d.u32()?, d.u64()?) as u32,
        Op::PostEvent => {
            let (ev, obj, flags, cb, _cookie, pid, n) = (d.u32()?, d.u64()?, d.u32()?, d.u64()?, d.u64()?, d.u32()?, d.u32()?);
            let mut ext = vec![0u8; n as usize * 0x20];
            let mut paths: Vec<Vec<u16>> = Vec::new();
            for i in 0..n as usize {
                let x = d.ext()?;
                let mut w = x.path.clone();
                w.push(0);
                paths.push(w);
                let p = &mut ext[i * 0x20..];
                p[0..4].copy_from_slice(&x.cookie.to_le_bytes());
                p[4..8].copy_from_slice(&x.codec.to_le_bytes());
                // pInMemory is not recorded: in-memory external sources replay as files.
                p[0x18..0x1c].copy_from_slice(&x.mem_size.to_le_bytes());
                p[0x1c..0x20].copy_from_slice(&x.file_id.to_le_bytes());
            }
            for (i, w) in paths.iter().enumerate() {
                let ptr = if w.len() > 1 { w.as_ptr() as u64 } else { 0 };
                ext[i * 0x20 + 8..i * 0x20 + 0x10].copy_from_slice(&ptr.to_le_bytes());
            }
            let cbp = if cb != 0 { noop_callback as *const c_void } else { std::ptr::null() };
            let new = (api.post_event)(ev, obj, flags, cbp, std::ptr::null_mut(), n, if n == 0 { std::ptr::null() } else { ext.as_ptr() }, st.pid(pid));
            if r.ret != 0 && new != 0 {
                st.pids.insert(r.ret, new);
            }
            if (r.ret == 0) != (new == 0) {
                st.mismatches += 1;
                log::warn!(target: "audio", "replay: PostEvent({ev:#x}) -> playing id {new} (recorded {})", r.ret);
            }
            st.applied += 1;
            return Ok(());
        }
        Op::ExecuteActionOnEvent => {
            let (ev, a, obj, ms, c, pid) = (d.u32()?, d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
            (api.exec_action)(ev, a, obj, ms, c, st.pid(pid)) as u32
        }
        Op::ExecuteActionOnEventName => {
            let name = std::ffi::CString::new(d.blob()?.to_vec()).map_err(|e| e.to_string())?;
            let (a, obj, ms, c, pid) = (d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
            (api.exec_action_name)(name.as_ptr() as *const u8, a, obj, ms, c, st.pid(pid)) as u32
        }
        Op::SeekOnEvent => {
            let (ev, obj, ms, near, pid) = (d.u32()?, d.u64()?, d.i32()?, d.bool()?, d.u32()?);
            (api.seek)(ev, obj, ms, near, st.pid(pid)) as u32
        }
        Op::SeekOnEventPercent => {
            let (ev, obj, pct, near, pid) = (d.u32()?, d.u64()?, d.f32()?, d.bool()?, d.u32()?);
            (api.seek_pct)(ev, obj, pct, near, st.pid(pid)) as u32
        }
        Op::StopAll => {
            (api.stop_all)(d.u64()?);
            0
        }
        Op::StopPlayingID => {
            let pid = d.u32()?;
            (api.stop_pid)(st.pid(pid), d.i32()?, d.i32()?);
            0
        }
        Op::PinEventInStreamCache => (api.pin)(d.u32()?, d.i8()?, d.i8()?) as u32,
        Op::UnpinEventInStreamCache => (api.unpin)(d.u32()?) as u32,
        Op::LoadBankMemoryCopy | Op::LoadBankMemoryView => {
            let (ptr, size, id, hash) = (d.u64()?, d.u32()?, d.u32()?, d.u64()?);
            let p = banks.join(aktrace::bank_file_name(id, size, hash));
            let bytes = std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))?;
            let a = Aligned::new(&bytes);
            let mut new_id = 0u32;
            let res = if r.op == Op::LoadBankMemoryCopy {
                (api.load_bank_copy)(a.ptr(), size, &mut new_id)
            } else {
                (api.load_bank_view)(a.ptr(), size, &mut new_id)
            };
            if bypass && res == AK_SUCCESS {
                let (n, f) = bypass_bus_fx(api, a.bytes());
                log::info!(target: "audio", "replay: bus FX bypass on {n} buses of bank {new_id:#x} ({f} failed)");
            }
            if r.op == Op::LoadBankMemoryView && res == AK_SUCCESS {
                st.views.insert(ptr, a);
            }
            res as u32
        }
        Op::UnloadBank => {
            let (id, ptr, ty) = (d.u32()?, d.u64()?, d.u32()?);
            let mem = st.views.get(&ptr).map_or(std::ptr::null(), |a| a.ptr());
            let res = (api.unload_bank)(id, mem, ty);
            if res == AK_SUCCESS {
                st.views.remove(&ptr);
            }
            res as u32
        }
        Op::AddOutput => {
            let s = d.take(aktrace::OUTPUT_SETTINGS_LEN)?.to_vec();
            let n = d.u32()?;
            let l: Vec<u64> = (0..n).map(|_| d.u64()).collect::<Result<_, _>>()?;
            let rec_id = d.u64()?;
            let mut id = 0u64;
            let res = (api.add_output)(s.as_ptr(), &mut id, if n == 0 { std::ptr::null() } else { l.as_ptr() }, n);
            if res == AK_SUCCESS {
                st.outputs.insert(rec_id, id);
            }
            res as u32
        }
        Op::RemoveOutput => {
            let id = d.u64()?;
            (api.remove_output)(st.out(id)) as u32
        }
        Op::SetSpeakerAngles => {
            let n = d.u32()?;
            let a: Vec<f32> = (0..n).map(|_| d.f32()).collect::<Result<_, _>>()?;
            let h = d.f32()?;
            let dev = d.u64()?;
            (api.set_speaker_angles)(a.as_ptr(), n, h, st.out(dev)) as u32
        }
    };
    st.applied += 1;
    if res != r.ret {
        st.mismatches += 1;
        if st.mismatches <= 20 {
            log::warn!(target: "audio", "replay: {:?} -> {} (recorded {})", r.op, akr(res as i32), akr(r.ret as i32));
        }
    }
    Ok(())
}

/// `AW2_AUDIO_HARNESS_MODE=replay`.
pub unsafe fn replay() -> Result<i32, String> {
    let dir = harness::base_dir().clone();
    let trace_path = PathBuf::from(env_or("AW2_AK_TRACE", &dir.join("trace.akt").to_string_lossy()));
    let out = PathBuf::from(env_or("AW2_AK_REPLAY_OUT", &dir.join("replay.wav").to_string_lossy()));
    let float_out = out.with_extension("f32.wav");
    let tail: u64 = env_or("AW2_AK_REPLAY_TAIL_FRAMES", "94").parse().unwrap_or(94);
    let data = std::fs::read(&trace_path).map_err(|e| format!("{}: {e}", trace_path.display()))?;
    let (recs, truncated) = aktrace::parse(&data)?;
    log::info!(target: "audio", "replay: {} ({truncated} truncated records dropped): {}", trace_path.display(), aktrace::summarize(&recs));
    let banks = aktrace::banks_dir(&trace_path);
    let _ = std::fs::remove_file(&out);

    let ak = Ak::load()?;
    let api = Api::load()?;
    let b = harness::boot(&ak, std::env::var_os("AW2_AK_REPLAY_TAP").map(PathBuf::from))?;
    if recs.iter().any(|r| r.op == Op::MusicInit) {
        music_init(&api);
    }
    let seed = seed();
    let bypass = bypass_enabled();
    prepare(&api)?;
    // Realtime pre-roll: everything up to and including the Init bank load
    // (the sink only exists after it; see go_offline). The offline timeline
    // starts at that record's frame.
    let is_load = |r: &&Record<'_>| matches!(r.op, Op::LoadBankMemoryCopy | Op::LoadBankMemoryView);
    let k = recs
        .iter()
        .position(|r| is_load(&r) && bank_has_chunk(&banks, r, b"INIT"))
        .or_else(|| recs.iter().position(|r| is_load(&r)))
        .ok_or("the trace loads no bank")?;
    let mut st = ReplayState::default();
    for p in recs[..=k].iter().filter(|r| r.op != Op::RenderAudio) {
        if let Err(e) = apply(&api, &mut st, &banks, bypass, p) {
            st.errors += 1;
            log::error!(target: "audio", "replay (pre-roll): {:?}: {e}", p.op);
        }
    }
    let base = recs[k].frame as u64;
    log::info!(target: "audio", "replay: realtime pre-roll applied records 0..={k} ({} calls); offline timeline starts at recorded frame {base}", st.applied);
    go_offline(&ak, &api, &b, seed)?;
    if bypass {
        bypass_device_fx(&api);
    }

    let cap_w = wide_z(&out.to_string_lossy());
    let r = (ak.start_capture)(cap_w.as_ptr());
    if r != AK_SUCCESS {
        return Err(format!("StartOutputCapture -> {}", akr(r)));
    }
    float_start();
    let t0 = Instant::now();
    let mut pending: Vec<&Record<'_>> = Vec::new();
    let mut rendered: u64 = 0;
    let mut batches = 0usize;
    for r in &recs[k + 1..] {
        if r.op == Op::RenderAudio {
            while rendered + base < r.frame as u64 {
                render(&ak);
                rendered += 1;
            }
            for p in pending.drain(..) {
                if let Err(e) = apply(&api, &mut st, &banks, bypass, p) {
                    st.errors += 1;
                    log::error!(target: "audio", "replay: {:?}: {e}", p.op);
                }
            }
            batches += 1;
        } else {
            pending.push(r);
        }
    }
    for p in pending.drain(..) {
        if let Err(e) = apply(&api, &mut st, &banks, bypass, p) {
            st.errors += 1;
            log::error!(target: "audio", "replay: {:?}: {e}", p.op);
        }
    }
    for _ in 0..tail {
        render(&ak);
        rendered += 1;
    }
    let r = (ak.stop_capture)();
    let (ch, fl) = float_stop();
    render(&ak); // processes StopOutputCapture (not captured by either writer)
    (ak.stop_all)(u64::MAX);
    render(&ak);
    if env_or("AW2_AUDIO_HARNESS_TERM", "1") != "0" {
        (ak.se_term)();
    }
    write_float_wav(&float_out, ch, b.rate, &fl)?;
    let (h16, n16) = wav_data_hash(&out);
    let (xn, xbad, xlen) = i16_vs_f32(&out, &fl);
    let text = format!(
        "trace    : {} ({})\nseed     : {seed:#x}; bus FX bypass {}; mixer SIMD select {} -> {}\nrendered : {rendered} frames ({:.2} s audio) in {:.2} s; {batches} batches\napplied  : {} calls; {} result mismatches vs recording; {} errors; skipped {:?}\ni16 wav  : {} ({n16} data bytes, fnv64 {h16:016x}; StopOutputCapture -> {})\nf32 wav  : {} ({ch} ch, {} frames, fnv64 {:016x})\ni16==f32 : {xbad} of {xn} samples differ from trunc(clamp(f)*32767); length diff {xlen}\nsimd     : {}\n",
        trace_path.display(),
        aktrace::summarize(&recs),
        if bypass { "ON" } else { "off" },
        b.mixer_simd.0,
        b.mixer_simd.1,
        rendered as f64 * b.spf as f64 / b.rate as f64,
        t0.elapsed().as_secs_f64(),
        st.applied,
        st.mismatches,
        st.errors,
        st.skipped,
        out.display(),
        akr(r),
        float_out.display(),
        fl.len() / ch as usize,
        f32_hash(&fl),
        b.simd.map_or("?".into(), harness::simd_desc),
    );
    log::info!(target: "audio", "replay report:\n{text}");
    let _ = std::fs::write(out.with_extension("txt"), &text);
    Ok(if st.errors == 0 && n16 > 0 && xn > 0 && xbad == 0 && xlen == 0 { 0 } else { 2 })
}

// ---------------------------------------------------------------------------
// Events (single post / batch)
// ---------------------------------------------------------------------------

/// `AW2_AUDIO_HARNESS_EVENT_IDS`: comma/space/newline separated decimal or 0x
/// IDs, or `@file` with the same syntax (`#` starts a comment).
fn event_ids_from_env() -> Result<Option<Vec<u32>>, String> {
    let Ok(spec) = std::env::var("AW2_AUDIO_HARNESS_EVENT_IDS") else { return Ok(None) };
    let text = if let Some(f) = spec.strip_prefix('@') { std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))? } else { spec };
    let mut ids = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("");
        for tok in line.split(|c: char| c == ',' || c.is_whitespace()).filter(|t| !t.is_empty()) {
            ids.push(harness::parse_id(tok).ok_or_else(|| format!("bad event id '{tok}'"))?);
        }
    }
    Ok(Some(ids))
}

/// `AW2_AUDIO_HARNESS_MODE=events`.
pub unsafe fn events() -> Result<i32, String> {
    let dir = harness::base_dir().clone();
    let default_bank = env_or("AW2_AUDIO_HARNESS_BANK", "play_gui_caseboard_hand_up");
    let bank_names: Vec<String> = env_or("AW2_AUDIO_HARNESS_BANKS", &format!("Init,{default_bank}")).split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let out_dir = PathBuf::from(env_or("AW2_AUDIO_HARNESS_OUT", &dir.join("events").to_string_lossy()));
    let max_frames: u64 = env_or("AW2_AUDIO_HARNESS_MAX_FRAMES", "2813").parse().unwrap_or(2813);
    let tail: u64 = env_or("AW2_AUDIO_HARNESS_TAIL_FRAMES", "0").parse().unwrap_or(0);
    let drain_max: u64 = env_or("AW2_AUDIO_HARNESS_DRAIN_FRAMES", "938").parse().unwrap_or(938);
    // StopPlayingID(pid, STOP_MS, STOP_CURVE) issued before rendering frame
    // STOP_AFTER of each event (capture continues until EndOfEvent).
    let stop_after: Option<u64> = std::env::var("AW2_AUDIO_HARNESS_STOP_AFTER").ok().and_then(|s| s.parse().ok());
    let stop_ms: i32 = env_or("AW2_AUDIO_HARNESS_STOP_MS", "0").parse().unwrap_or(0);
    let stop_curve: i32 = env_or("AW2_AUDIO_HARNESS_STOP_CURVE", "4").parse().unwrap_or(4); // AkCurveInterpolation_Linear
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let banks: Vec<(String, Vec<u8>)> = bank_names.iter().map(|n| harness::read_bank(&dir, n).map(|b| (n.clone(), b))).collect::<Result<_, _>>()?;

    let ak = Ak::load()?;
    let api = Api::load()?;
    let ids = match event_ids_from_env()? {
        Some(v) => v,
        None => vec![harness::event_id_from_env(&ak, &default_bank).0],
    };
    let b = harness::boot(&ak, None)?;
    music_init(&api);
    let seed = seed();
    let bypass = bypass_enabled();
    prepare(&api)?;
    let load = |name: &str, bytes: &[u8]| -> Result<u32, String> {
        let mut id = 0u32;
        let r = (api.load_bank_copy)(bytes.as_ptr(), bytes.len() as u32, &mut id);
        if r != AK_SUCCESS {
            return Err(format!("LoadBankMemoryCopy({name}.bnk) -> {}", akr(r)));
        }
        if bypass {
            let (n, f) = bypass_bus_fx(&api, bytes);
            log::debug!(target: "audio", "events: bus FX bypass on {n} buses of {name}.bnk ({f} failed)");
        }
        Ok(id)
    };
    // (index into `banks`, bank id) of the reloadable (non-Init) banks.
    let mut sound_banks: Vec<(usize, u32)> = Vec::new();
    for (k, (name, bytes)) in banks.iter().enumerate() {
        let id = load(name, bytes)?;
        log::info!(target: "audio", "events: loaded {name}.bnk ({} bytes) -> bank id {id:#x}", bytes.len());
        if !bank_chunk(bytes, b"INIT") {
            sound_banks.push((k, id));
        }
    }
    let reload = env_or("AW2_AUDIO_HARNESS_RELOAD", "1") != "0";
    const LISTENER: u64 = 1;
    const EMITTER: u64 = 2;
    for o in [LISTENER, EMITTER] {
        (api.register_obj)(o, std::ptr::null());
    }
    (api.set_default_listeners)(&LISTENER, 1);
    go_offline(&ak, &api, &b, seed)?;
    if bypass {
        bypass_device_fx(&api);
    }
    render(&ak);

    let mut tsv = String::from("event_id\tplaying_id\tframes\tended\tpeak\trms\ti16_fnv64\tf32_fnv64\ti16_f32_mismatch\ti16_f32_len_diff\ti16_wav\tf32_wav\n");
    let (mut ok, mut failed, mut capped) = (0, 0, 0);
    let t0 = Instant::now();
    // Warm-up: the first voice(s) a process renders differ from later
    // identical posts by ~1e-6 (some engine/bus state is created on first
    // use and then persists; measured: hand_up after hand_down == hand_up
    // after hand_up != first hand_up). Post discarded warm-up events first so
    // every captured render is independent of list order. "first" (default)
    // warms with the first ID, "all" with every unique ID, "0" disables.
    let warm_mode = env_or("AW2_AUDIO_HARNESS_WARMUP", "first");
    let warm: Vec<u32> = match warm_mode.as_str() {
        "0" | "none" => vec![],
        "all" => {
            let mut v = ids.clone();
            v.sort_unstable();
            v.dedup();
            v
        }
        _ => ids.first().copied().into_iter().collect(),
    };
    for &id in &warm {
        drain(&ak, drain_max);
        ENDED.store(false, Ordering::Relaxed);
        let pid = (api.post_event)(id, EMITTER, 1, end_callback as *const c_void, std::ptr::null_mut(), 0, std::ptr::null(), 0);
        ENDED_PID.store(pid, Ordering::Relaxed);
        let mut frames = 0;
        while pid != 0 && !ENDED.load(Ordering::Relaxed) && frames < max_frames {
            render(&ak);
            frames += 1;
        }
        (api.stop_all)(u64::MAX);
    }
    let mut seen: HashMap<u32, usize> = HashMap::new();
    for (n, &id) in ids.iter().enumerate() {
        // Identical preamble for every event: fresh sound-bank objects
        // (random/sequence container history), then silence.
        if reload && (n > 0 || !warm.is_empty()) {
            for (k, bank_id) in sound_banks.iter_mut() {
                let r = (api.unload_bank)(*bank_id, std::ptr::null(), 0);
                if r != AK_SUCCESS {
                    log::warn!(target: "audio", "events: UnloadBank({:#x}) -> {}", *bank_id, akr(r));
                }
                *bank_id = load(&banks[*k].0, &banks[*k].1)?;
            }
        }
        drain(&ak, drain_max);
        let dup = seen.entry(id).or_insert(0);
        let stem = if *dup == 0 { format!("{id}") } else { format!("{id}_{dup}") };
        *dup += 1;
        let wav = out_dir.join(format!("{stem}.wav"));
        let fwav = out_dir.join(format!("{stem}.f32.wav"));
        let _ = std::fs::remove_file(&wav);
        (api.set_random_seed)(seed);
        let w = wide_z(&wav.to_string_lossy());
        (ak.start_capture)(w.as_ptr());
        float_start();
        ENDED.store(false, Ordering::Relaxed);
        // The callback may fire inside a later RenderAudio only, so the ID
        // can be stored after PostEvent returns.
        let pid = (api.post_event)(id, EMITTER, 1, end_callback as *const c_void, std::ptr::null_mut(), 0, std::ptr::null(), 0);
        ENDED_PID.store(pid, Ordering::Relaxed);
        let mut frames = 0u64;
        if pid != 0 {
            while !ENDED.load(Ordering::Relaxed) && frames < max_frames {
                if stop_after.is_some_and(|n| n == frames) {
                    (api.stop_pid)(pid, stop_ms, stop_curve);
                }
                render(&ak);
                frames += 1;
            }
            for _ in 0..tail {
                render(&ak);
                frames += 1;
            }
        }
        (ak.stop_capture)();
        let (ch, fl) = float_stop();
        render(&ak);
        let ended = ENDED.load(Ordering::Relaxed);
        write_float_wav(&fwav, ch, b.rate, &fl)?;
        (api.stop_all)(u64::MAX);
        // Drain voices / bus tails so the next event starts from silence.
        let quiet = drain(&ak, drain_max);
        let peak = fl.iter().fold(0f32, |m, v| m.max(v.abs()));
        let rms = if fl.is_empty() { 0.0 } else { (fl.iter().map(|v| (*v as f64) * (*v as f64)).sum::<f64>() / fl.len() as f64).sqrt() };
        let (h16, _) = wav_data_hash(&wav);
        let (_, xbad, xlen) = i16_vs_f32(&wav, &fl);
        if pid == 0 {
            failed += 1;
        } else if ended {
            ok += 1;
        } else {
            capped += 1;
        }
        tsv.push_str(&format!(
            "{id}\t{pid}\t{frames}\t{}\t{peak:.6}\t{rms:.6}\t{h16:016x}\t{:016x}\t{xbad}\t{xlen}\t{}\t{}\n",
            ended as u8,
            f32_hash(&fl),
            wav.display(),
            fwav.display()
        ));
        log::info!(target: "audio", "events: {id} pid {pid}: {frames} frames, ended={ended}, peak {peak:.4}, i16/f32 mismatches {xbad} (len diff {xlen}), drain quiet={quiet}");
    }
    if env_or("AW2_AUDIO_HARNESS_TERM", "1") != "0" {
        (ak.se_term)();
    }
    std::fs::write(out_dir.join("results.tsv"), &tsv).map_err(|e| e.to_string())?;
    log::info!(
        target: "audio",
        "events: {} events in {:.1} s: {ok} ended, {capped} hit the {max_frames}-frame cap, {failed} failed to post; seed {seed:#x}; bus FX bypass {}; results {}",
        ids.len(),
        t0.elapsed().as_secs_f64(),
        if bypass { "ON" } else { "off" },
        out_dir.join("results.tsv").display()
    );
    Ok(if failed == 0 { 0 } else { 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_chunk_scan() {
        let mut b = b"BKHD".to_vec();
        b.extend_from_slice(&4u32.to_le_bytes());
        b.extend_from_slice(&[1, 2, 3, 4]);
        b.extend_from_slice(b"INIT");
        b.extend_from_slice(&0u32.to_le_bytes());
        assert!(bank_chunk(&b, b"INIT"));
        assert!(bank_chunk(&b, b"BKHD"));
        assert!(!bank_chunk(&b, b"HIRC"));
        assert!(!bank_chunk(&b[..10], b"INIT"));
    }

    #[test]
    fn hirc_bus_scan() {
        // BKHD (empty) + HIRC with Sound(0x02), Bus(0x08), AuxBus(0x12).
        let mut b = b"BKHD".to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        let mut h = 3u32.to_le_bytes().to_vec();
        for (ty, id) in [(2u8, 10u32), (8, 20), (0x12, 30)] {
            h.push(ty);
            h.extend_from_slice(&6u32.to_le_bytes());
            h.extend_from_slice(&id.to_le_bytes());
            h.extend_from_slice(&[0, 0]);
        }
        b.extend_from_slice(b"HIRC");
        b.extend_from_slice(&(h.len() as u32).to_le_bytes());
        b.extend_from_slice(&h);
        assert_eq!(hirc_bus_ids(&b), vec![20, 30]);
    }

    #[test]
    fn float_wav_reads_back() {
        let p = std::env::temp_dir().join(format!("aw2_f32_{}.wav", std::process::id()));
        write_float_wav(&p, 2, 48000, &[0.5, -0.25, 1.0, 0.0]).unwrap();
        let w = super::super::wavcmp::read_wav(&std::fs::read(&p).unwrap()).unwrap();
        let _ = std::fs::remove_file(&p);
        assert!(w.float);
        assert_eq!((w.channels, w.rate, w.samples.clone()), (2, 48000, vec![0.5, -0.25, 1.0, 0.0]));
    }
}
