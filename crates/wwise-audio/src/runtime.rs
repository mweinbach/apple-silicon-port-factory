//! Realtime Wwise 2022.1 SoundEngine runtime (`docs/AUDIO.md` §7 & §8 Milestone A8).
//!
//! [`NativeSoundEngine`] wraps [`Project`] + [`RendererState`] behind a thread-safe
//! command queue, bank manager, external source resolver, callback dispatcher,
//! output sink, and `AW2AKT01` trace replayer:
//!
//! * **Command Queue**: game threads call `post_event`, `set_rtpc_value`,
//!   `set_position`, `set_state`, `set_switch`, `execute_action_on_event`,
//!   `seek_on_event`, `stop_playing_id`, etc. without blocking on DSP; commands
//!   are drained in FIFO order at the start of [`NativeSoundEngine::render_audio`].
//! * **RTPC Ramping**: `SetRTPCValue(..., transition_ms, curve)` interpolates
//!   smoothly across audio frames using Wwise's exact curve evaluator
//!   (`engine::math::eval_curve_segment`).
//! * **Callbacks**: `AK_EndOfEvent` (`0x0001`), `AK_Duration` (`0x0008`), and
//!   `AK_MusicSync*` (`0x0100..=0x4000`) are synthesized with the exact Win64
//!   `AkEventCallbackInfo` (`0x18` bytes), `AkDurationCallbackInfo` (`0x30` bytes),
//!   and `AkMusicSyncCallbackInfo` (`0x48` bytes) layouts verified from Remedy's
//!   callback handlers (`0x1424c1620`, `0x1424c1500`), and can be delivered to
//!   either Win64 guest function pointers or native Rust closures.
//! * **External Sources**: `AkExternalSourceInfo` entries passed to `PostEvent`
//!   resolve dialogue `.wem` media by memory pointer, host file path, VFS path
//!   (`data_pc/audio/dialogue/script/locale/en/...`), or file ID.

use std::collections::{HashMap, VecDeque};
use std::ffi::{c_void, CString};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::bnk::hirc::HircBody;
use crate::engine::math::{fast_pow_to_db, interp_shape, FRAME, RATE};
use crate::engine::music::{SegmentBounds, MARKER_ENTRY_IDS, MARKER_EXIT_IDS};
use crate::engine::params::GameState;
use crate::engine::project::Project;
use crate::engine::render::{ExternalSourceDesc, RenderOptions, Renderer, RendererState};
use crate::engine::spatial::atten::WorldTransform;
use crate::fnv1_32_lower;

// ---------------------------------------------------------------------------
// AKRESULT & AkCallbackType Constants
// ---------------------------------------------------------------------------

pub const AK_NOT_IMPLEMENTED: i32 = 0;
pub const AK_SUCCESS: i32 = 1;
pub const AK_FAIL: i32 = 2;
pub const AK_INVALID_PARAMETER: i32 = 31;
pub const AK_UNKNOWN_BANK_ID: i32 = 54;
pub const AK_BANK_READ_ERROR: i32 = 56;
pub const AK_FILE_NOT_FOUND: i32 = 66;
pub const AK_NOT_INITIALIZED: i32 = 102;

pub const AK_END_OF_EVENT: u32 = 0x0001;
pub const AK_END_OF_DYNAMIC_SEQUENCE_ITEM: u32 = 0x0002;
pub const AK_MARKER: u32 = 0x0004;
pub const AK_DURATION: u32 = 0x0008;
pub const AK_SPEAKER_VOLUME_MATRIX: u32 = 0x0010;
pub const AK_STARVATION: u32 = 0x0020;
pub const AK_MUSIC_PLAYLIST_SELECT: u32 = 0x0040;
pub const AK_MUSIC_PLAY_STARTED: u32 = 0x0080;
pub const AK_MUSIC_SYNC_BEAT: u32 = 0x0100;
pub const AK_MUSIC_SYNC_BAR: u32 = 0x0200;
pub const AK_MUSIC_SYNC_ENTRY: u32 = 0x0400;
pub const AK_MUSIC_SYNC_EXIT: u32 = 0x0800;
pub const AK_MUSIC_SYNC_GRID: u32 = 0x1000;
pub const AK_MUSIC_SYNC_USER_CUE: u32 = 0x2000;
pub const AK_MUSIC_SYNC_POINT: u32 = 0x4000;
pub const AK_MUSIC_SYNC_ALL: u32 = 0x7f00;
pub const AK_MIDI_EVENT: u32 = 0x10000;
pub const AK_CALLBACK_BITS: u32 = 0xfffff;
pub const AK_ENABLE_GET_SOURCE_PLAY_POSITION: u32 = 0x100000;
pub const AK_ENABLE_GET_MUSIC_PLAY_POSITION: u32 = 0x200000;
pub const AK_ENABLE_GET_SOURCE_STREAM_BUFFERING: u32 = 0x400000;

/// `AkGlobalCallbackLocation_BeginRender` (`1 << 4`).
pub const LOC_BEGIN_RENDER: u32 = 0x10;
/// `AkGlobalCallbackLocation_EndRender` (`1 << 5`).
pub const LOC_END_RENDER: u32 = 0x20;

// ---------------------------------------------------------------------------
// Exact Win64 Callback Structs (verified at 0x1424c1620 & 0x1424c1500)
// ---------------------------------------------------------------------------

/// `AkEventCallbackInfo` (`0x18` bytes on Win64).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AkEventCallbackInfo {
    pub p_cookie: u64,
    pub game_obj_id: u64,
    pub playing_id: u32,
    pub event_id: u32,
}
const _: () = assert!(std::mem::size_of::<AkEventCallbackInfo>() == 0x18);

/// `AkDurationCallbackInfo` (`0x30` bytes on Win64).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AkDurationCallbackInfo {
    pub event: AkEventCallbackInfo,
    pub f_duration: f32,
    pub f_estimated_duration: f32,
    pub audio_node_id: u32,
    pub media_id: u32,
    pub b_streaming: u8,
    pub _pad: [u8; 7],
}
const _: () = assert!(std::mem::size_of::<AkDurationCallbackInfo>() == 0x30);

/// `AkMusicSyncCallbackInfo` (`0x48` bytes on Win64, verified from `0x1424c1500`).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AkMusicSyncCallbackInfo {
    pub p_cookie: u64,
    pub game_obj_id: u64,
    pub playing_id: u32,
    pub i_current_position: i32,
    pub i_pre_entry_duration: i32,
    pub i_active_duration: i32,
    pub i_post_exit_duration: i32,
    pub i_remaining_look_ahead_time: i32,
    pub f_beat_duration: f32,
    pub f_bar_duration: f32,
    pub f_grid_duration: f32,
    pub f_grid_offset: f32,
    pub music_sync_type: u32,
    pub _pad: u32,
    pub psz_user_cue_name: u64,
}
const _: () = assert!(std::mem::size_of::<AkMusicSyncCallbackInfo>() == 0x48);

/// `AkSegmentInfo` (`0x24` bytes on Win64, verified from `AK::MusicEngine::GetPlayingSegmentInfo` at `0x1415373a0`).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AkSegmentInfo {
    pub i_current_position: i32,
    pub i_pre_entry_duration: i32,
    pub i_active_duration: i32,
    pub i_post_exit_duration: i32,
    pub i_remaining_look_ahead_time: i32,
    pub f_beat_duration: f32,
    pub f_bar_duration: f32,
    pub f_grid_duration: f32,
    pub f_grid_offset: f32,
}
const _: () = assert!(std::mem::size_of::<AkSegmentInfo>() == 0x24);


/// Guest Win64 event callback signature: `void AkCallbackFunc(AkCallbackType, AkCallbackInfo*)`.
#[cfg(target_arch = "x86_64")]
pub type Win64EventCallback = unsafe extern "win64" fn(u32, *mut c_void);
#[cfg(not(target_arch = "x86_64"))]
pub type Win64EventCallback = unsafe extern "C" fn(u32, *mut c_void);

/// Guest Win64 global callback signature: `void AkGlobalCallbackFunc(IAkGlobalPluginContext*, AkGlobalCallbackLocation, void*)`.
#[cfg(target_arch = "x86_64")]
pub type Win64GlobalCallback = unsafe extern "win64" fn(*mut c_void, u32, *mut c_void);
#[cfg(not(target_arch = "x86_64"))]
pub type Win64GlobalCallback = unsafe extern "C" fn(*mut c_void, u32, *mut c_void);

/// High-level Rust representation of a fired event callback.
#[derive(Debug, Clone)]
pub struct CallbackEvent {
    pub callback_type: u32,
    pub cookie: u64,
    pub game_obj_id: u64,
    pub playing_id: u32,
    pub event_id: u32,
    pub payload: CallbackPayload,
}

#[derive(Debug, Clone)]
pub enum CallbackPayload {
    EndOfEvent,
    Duration {
        duration_ms: f32,
        estimated_duration_ms: f32,
        audio_node_id: u32,
        media_id: u32,
        streaming: bool,
    },
    MusicSync {
        current_position_ms: i32,
        pre_entry_duration_ms: i32,
        active_duration_ms: i32,
        post_exit_duration_ms: i32,
        beat_duration_s: f32,
        bar_duration_s: f32,
        grid_duration_s: f32,
        grid_offset_s: f32,
        user_cue_name: Option<String>,
    },
}

#[derive(Clone)]
pub enum EventCallbackTarget {
    Rust(Arc<dyn Fn(&CallbackEvent) + Send + Sync>),
    Win64(Win64EventCallback),
}

impl EventCallbackTarget {
    fn invoke(&self, ev: &CallbackEvent) {
        match self {
            EventCallbackTarget::Rust(f) => f(ev),
            EventCallbackTarget::Win64(cb) => unsafe {
                match &ev.payload {
                    CallbackPayload::EndOfEvent => {
                        let mut info = AkEventCallbackInfo {
                            p_cookie: ev.cookie,
                            game_obj_id: ev.game_obj_id,
                            playing_id: ev.playing_id,
                            event_id: ev.event_id,
                        };
                        cb(ev.callback_type, &mut info as *mut _ as *mut c_void);
                    }
                    CallbackPayload::Duration {
                        duration_ms,
                        estimated_duration_ms,
                        audio_node_id,
                        media_id,
                        streaming,
                    } => {
                        let mut info = AkDurationCallbackInfo {
                            event: AkEventCallbackInfo {
                                p_cookie: ev.cookie,
                                game_obj_id: ev.game_obj_id,
                                playing_id: ev.playing_id,
                                event_id: ev.event_id,
                            },
                            f_duration: *duration_ms,
                            f_estimated_duration: *estimated_duration_ms,
                            audio_node_id: *audio_node_id,
                            media_id: *media_id,
                            b_streaming: u8::from(*streaming),
                            _pad: [0; 7],
                        };
                        cb(ev.callback_type, &mut info as *mut _ as *mut c_void);
                    }
                    CallbackPayload::MusicSync {
                        current_position_ms,
                        pre_entry_duration_ms,
                        active_duration_ms,
                        post_exit_duration_ms,
                        beat_duration_s,
                        bar_duration_s,
                        grid_duration_s,
                        grid_offset_s,
                        user_cue_name,
                    } => {
                        let c_cue = user_cue_name
                            .as_deref()
                            .and_then(|s| CString::new(s).ok());
                        let cue_ptr = c_cue.as_ref().map_or(0u64, |c| c.as_ptr() as u64);
                        let mut info = AkMusicSyncCallbackInfo {
                            p_cookie: ev.cookie,
                            game_obj_id: ev.game_obj_id,
                            playing_id: ev.playing_id,
                            i_current_position: *current_position_ms,
                            i_pre_entry_duration: *pre_entry_duration_ms,
                            i_active_duration: *active_duration_ms,
                            i_post_exit_duration: *post_exit_duration_ms,
                            i_remaining_look_ahead_time: 0,
                            f_beat_duration: *beat_duration_s,
                            f_bar_duration: *bar_duration_s,
                            f_grid_duration: *grid_duration_s,
                            f_grid_offset: *grid_offset_s,
                            music_sync_type: ev.callback_type,
                            _pad: 0,
                            psz_user_cue_name: cue_ptr,
                        };
                        cb(ev.callback_type, &mut info as *mut _ as *mut c_void);
                    }
                }
            },
        }
    }
}

#[derive(Clone)]
pub enum GlobalCallbackTarget {
    Rust(Arc<dyn Fn(u32, u64) + Send + Sync>),
    Win64(Win64GlobalCallback),
}

// ---------------------------------------------------------------------------
// Audio Sink Trait
// ---------------------------------------------------------------------------

/// Consumer of interleaved float32 output frames produced by [`NativeSoundEngine::render_audio`].
pub trait AudioSink: Send + Sync {
    fn push_samples(&self, samples: &[f32], channels: usize, sample_rate: u32);
}

// ---------------------------------------------------------------------------
// Queued Commands & RTPC Ramps
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum Command {
    RegisterGameObj { obj_id: u64, name: Option<String> },
    UnregisterGameObj { obj_id: u64 },
    UnregisterAllGameObj,
    SetPosition { obj_id: u64, transform: WorldTransform },
    SetDefaultListeners { listeners: Vec<u64> },
    SetListeners { emitter: u64, listeners: Vec<u64> },
    SetDistanceProbe { listener: u64, probe: u64 },
    SetScalingFactor { obj_id: u64, factor: f32 },
    SetObjectObstructionAndOcclusion { emitter: u64, listener: u64, obs: f32, occ: f32 },
    SetGameObjectAuxSendValues { obj_id: u64, sends: Vec<(u64, u32, f32)> },
    SetGameObjectOutputBusVolume { emitter: u64, listener: u64, volume_linear: f32 },
    SetRtpcValue {
        rtpc_id: u32,
        value: f32,
        obj_id: u64,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    },
    SetRtpcValueByPlayingId {
        rtpc_id: u32,
        value: f32,
        playing_id: u32,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    },
    ResetRtpcValue {
        rtpc_id: u32,
        obj_id: u64,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    },
    SetSwitch { group_id: u32, switch_id: u32, obj_id: u64 },
    SetState { group_id: u32, state_id: u32 },
    PostTrigger { trigger_id: u32 },
    PostEvent {
        playing_id: u32,
        event_id: u32,
        game_obj_id: u64,
        flags: u32,
        callback: Option<EventCallbackTarget>,
        cookie: u64,
        external_sources: Vec<ExternalSourceDesc>,
    },
    ExecuteActionOnEvent {
        event_id: u32,
        action_type: i32,
        game_obj_id: u64,
        transition_ms: i32,
        curve: i32,
        playing_id: u32,
    },
    SeekOnEvent {
        event_id: u32,
        game_obj_id: u64,
        position_ms: Option<i32>,
        percent: Option<f32>,
        playing_id: u32,
    },
    StopAll { game_obj_id: u64 },
    StopPlayingId { playing_id: u32, transition_ms: i32, curve: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RtpcTarget {
    Global,
    GameObject(u64),
    PlayingId(u32),
}

#[derive(Debug, Clone)]
struct RtpcRamp {
    rtpc_id: u32,
    target: RtpcTarget,
    start_val: f32,
    end_val: f32,
    start_sample: u64,
    duration_samples: u64,
    curve: u8,
}

#[derive(Clone)]
struct PlayingTracker {
    playing_id: u32,
    event_id: u32,
    game_obj_id: u64,
    flags: u32,
    callback: Option<EventCallbackTarget>,
    cookie: u64,
    duration_sent: bool,
    /// Per-(segment_id) last cursor in seconds so we can detect music sync crossings.
    music_seg_prev_sec: HashMap<u32, f64>,
}

struct GlobalCallbackEntry {
    callback: GlobalCallbackTarget,
    location_mask: u32,
    cookie: u64,
}

struct CaptureState {
    path: PathBuf,
    samples: Vec<f32>,
}

struct EngineInner {
    project: Project,
    renderer: RendererState,
    commands: VecDeque<Command>,
    rtpc_ramps: HashMap<(u32, RtpcTarget), RtpcRamp>,
    playing: HashMap<u32, PlayingTracker>,
    global_callbacks: Vec<GlobalCallbackEntry>,
    callback_history: Vec<CallbackEvent>,
    sink: Option<Arc<dyn AudioSink>>,
    capture: Option<CaptureState>,
    language: String,
    panning_rule: i32,
    output_volume: f32,
}

// ---------------------------------------------------------------------------
// NativeSoundEngine
// ---------------------------------------------------------------------------

/// Thread-safe Wwise 2022.1 SoundEngine replacement.
pub struct NativeSoundEngine {
    inner: Mutex<EngineInner>,
    initialized: AtomicBool,
    music_initialized: AtomicBool,
    next_playing_id: AtomicU32,
    frames_rendered: AtomicU32,
}

impl Default for NativeSoundEngine {
    fn default() -> Self {
        Self::new(RenderOptions {
            bus_fx: true,
            ..RenderOptions::default()
        })
    }
}

impl NativeSoundEngine {
    /// Construct a new `NativeSoundEngine` with the given [`RenderOptions`], attaching
    /// the default audio data source if one is enabled/available.
    pub fn new(opts: RenderOptions) -> Self {
        Self::with_source(opts, crate::data::default_source())
    }

    /// Construct a new `NativeSoundEngine` backed by an explicit [`crate::data::AudioDataSource`].
    pub fn with_source(opts: RenderOptions, game: Option<Arc<dyn crate::data::AudioDataSource>>) -> Self {
        let project = Project::new(game);
        let gs = GameState::default();
        let renderer = RendererState::new(gs, opts);
        Self {
            inner: Mutex::new(EngineInner {
                project,
                renderer,
                commands: VecDeque::new(),
                rtpc_ramps: HashMap::new(),
                playing: HashMap::new(),
                global_callbacks: Vec::new(),
                callback_history: Vec::new(),
                sink: None,
                capture: None,
                language: "en(US)".to_string(),
                panning_rule: 0,
                output_volume: 1.0,
            }),
            initialized: AtomicBool::new(true),
            music_initialized: AtomicBool::new(false),
            next_playing_id: AtomicU32::new(1),
            frames_rendered: AtomicU32::new(0),
        }
    }

    // --- Lifecycle ---

    pub fn init(&self) -> i32 {
        self.initialized.store(true, Ordering::Release);
        AK_SUCCESS
    }

    pub fn term(&self) {
        let _ = self.stop_output_capture();
        let mut g = self.inner.lock().unwrap();
        g.commands.clear();
        g.playing.clear();
        g.rtpc_ramps.clear();
        self.initialized.store(false, Ordering::Release);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    pub fn music_init(&self) -> i32 {
        self.music_initialized.store(true, Ordering::Release);
        AK_SUCCESS
    }

    pub fn music_term(&self) {
        self.music_initialized.store(false, Ordering::Release);
    }

    pub fn frames_rendered(&self) -> u32 {
        self.frames_rendered.load(Ordering::Relaxed)
    }

    pub fn sample_tick(&self) -> u64 {
        self.frames_rendered() as u64 * FRAME as u64
    }

    pub fn set_random_seed(&self, seed: u32) {
        let mut g = self.inner.lock().unwrap();
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| r.set_random_seed(seed));
    }

    pub fn set_output_sink(&self, sink: Option<Arc<dyn AudioSink>>) {
        self.inner.lock().unwrap().sink = sink;
    }

    pub fn has_output_sink(&self) -> bool {
        self.inner.lock().unwrap().sink.is_some()
    }

    pub fn get_id_from_string(&self, name: &str) -> u32 {
        fnv1_32_lower(name)
    }

    pub fn set_current_language(&self, lang: &str) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.language = lang.to_string();
        AK_SUCCESS
    }

    pub fn current_language(&self) -> String {
        self.inner.lock().unwrap().language.clone()
    }

    pub fn set_panning_rule(&self, rule: i32, _output_id: u64) -> i32 {
        self.inner.lock().unwrap().panning_rule = rule;
        AK_SUCCESS
    }

    pub fn panning_rule(&self, _output_id: u64) -> i32 {
        self.inner.lock().unwrap().panning_rule
    }

    pub fn set_output_volume(&self, _output_id: u64, volume: f32) -> i32 {
        self.inner.lock().unwrap().output_volume = volume;
        AK_SUCCESS
    }

    // --- Bank Management ---

    /// Load a SoundBank from raw bytes (`LoadBankMemoryCopy` / `LoadBankMemoryView`).
    /// Returns the bank's `dwSoundBankID` from its `BKHD` header.
    pub fn load_bank_bytes(&self, label: impl Into<String>, bytes: Vec<u8>) -> Result<u32, String> {
        let mut g = self.inner.lock().unwrap();
        let bi = g.project.load_bank_bytes(label.into(), bytes)?;
        let bank_id = g.project.banks[bi].id;
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| {
            r.invalidate_bus_order();
        });
        Ok(bank_id)
    }

    /// Load a SoundBank by name or host/VFS path.
    pub fn load_bank_name(&self, spec: &str) -> Result<u32, String> {
        let mut g = self.inner.lock().unwrap();
        let id = g.project.load_bank_with_id(spec)?;
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| {
            r.invalidate_bus_order();
        });
        Ok(id)
    }

    /// Unload a previously loaded SoundBank by its `dwSoundBankID`.
    pub fn unload_bank(&self, bank_id: u32) -> bool {
        let mut g = self.inner.lock().unwrap();
        let unloaded = g.project.unload_bank(bank_id);
        if unloaded {
            let EngineInner { project, renderer, .. } = &mut *g;
            renderer.with_project(project, |r| {
                r.invalidate_bus_order();
            });
        }
        unloaded
    }

    /// Unload all banks (`AK::SoundEngine::ClearBanks`).
    pub fn clear_banks(&self) -> i32 {
        let mut g = self.inner.lock().unwrap();
        let bank_ids: Vec<u32> = g.project.banks.iter().map(|b| b.id).collect();
        for id in bank_ids {
            let _ = g.project.unload_bank(id);
        }
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| {
            r.invalidate_bus_order();
        });
        AK_SUCCESS
    }

    // --- Output Capture ---

    pub fn start_output_capture(&self, path: impl AsRef<Path>) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.capture = Some(CaptureState {
            path: path.as_ref().to_path_buf(),
            samples: Vec::new(),
        });
        AK_SUCCESS
    }

    pub fn stop_output_capture(&self) -> Result<(), String> {
        let (cap, channels, sample_rate) = {
            let mut g = self.inner.lock().unwrap();
            let ch = g.renderer.opts.out_channels as u16;
            let sr = RATE;
            (g.capture.take(), ch, sr)
        };
        let Some(cap) = cap else {
            return Ok(());
        };
        if let Some(parent) = cap.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let f16 = File::create(&cap.path).map_err(|e| format!("{}: {e}", cap.path.display()))?;
        crate::wav::write_wav(BufWriter::new(f16), channels, sample_rate, &cap.samples, false)
            .map_err(|e| format!("{}: {e}", cap.path.display()))?;
        let f32_path = cap.path.with_extension("f32.wav");
        let ff32 = File::create(&f32_path).map_err(|e| format!("{}: {e}", f32_path.display()))?;
        crate::wav::write_wav(BufWriter::new(ff32), channels, sample_rate, &cap.samples, true)
            .map_err(|e| format!("{}: {e}", f32_path.display()))?;
        Ok(())
    }

    // --- Global Callbacks ---

    pub fn register_global_callback_rust(
        &self,
        location_mask: u32,
        cookie: u64,
        cb: Arc<dyn Fn(u32, u64) + Send + Sync>,
    ) -> i32 {
        self.inner.lock().unwrap().global_callbacks.push(GlobalCallbackEntry {
            callback: GlobalCallbackTarget::Rust(cb),
            location_mask,
            cookie,
        });
        AK_SUCCESS
    }

    pub fn register_global_callback_win64(
        &self,
        cb: Win64GlobalCallback,
        location_mask: u32,
        cookie: u64,
    ) -> i32 {
        self.inner.lock().unwrap().global_callbacks.push(GlobalCallbackEntry {
            callback: GlobalCallbackTarget::Win64(cb),
            location_mask,
            cookie,
        });
        AK_SUCCESS
    }

    pub fn unregister_global_callback_win64(
        &self,
        cb: Win64GlobalCallback,
        location_mask: u32,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        let target = cb as usize;
        for entry in &mut g.global_callbacks {
            if let GlobalCallbackTarget::Win64(f) = entry.callback {
                if f as usize == target {
                    entry.location_mask &= !location_mask;
                }
            }
        }
        g.global_callbacks.retain(|e| e.location_mask != 0);
        AK_SUCCESS
    }

    pub fn cancel_event_callback(&self, playing_id: u32) {
        let mut g = self.inner.lock().unwrap();
        if let Some(t) = g.playing.get_mut(&playing_id) {
            t.callback = None;
            t.flags &= !AK_CALLBACK_BITS;
        }
        for cmd in &mut g.commands {
            if let Command::PostEvent {
                playing_id: pid,
                callback,
                flags,
                ..
            } = cmd
            {
                if *pid == playing_id {
                    *callback = None;
                    *flags &= !AK_CALLBACK_BITS;
                }
            }
        }
    }

    pub fn cancel_event_callback_cookie(&self, cookie: u64) {
        let mut g = self.inner.lock().unwrap();
        for t in g.playing.values_mut() {
            if t.cookie == cookie {
                t.callback = None;
                t.flags &= !AK_CALLBACK_BITS;
            }
        }
        for cmd in &mut g.commands {
            if let Command::PostEvent {
                cookie: c,
                callback,
                flags,
                ..
            } = cmd
            {
                if *c == cookie {
                    *callback = None;
                    *flags &= !AK_CALLBACK_BITS;
                }
            }
        }
    }

    pub fn cancel_event_callback_game_object(&self, game_obj_id: u64) {
        let mut g = self.inner.lock().unwrap();
        for t in g.playing.values_mut() {
            if t.game_obj_id == game_obj_id {
                t.callback = None;
                t.flags &= !AK_CALLBACK_BITS;
            }
        }
        for cmd in &mut g.commands {
            if let Command::PostEvent {
                game_obj_id: obj,
                callback,
                flags,
                ..
            } = cmd
            {
                if *obj == game_obj_id {
                    *callback = None;
                    *flags &= !AK_CALLBACK_BITS;
                }
            }
        }
    }

    // --- Game Objects & Spatial ---

    pub fn register_game_obj(&self, obj_id: u64, name: Option<&str>) -> i32 {
        let mut g = self.inner.lock().unwrap();
        let name_str = name.unwrap_or("").to_string();
        g.renderer.gs.registered_objects.insert(obj_id, name_str.clone());
        g.commands.push_back(Command::RegisterGameObj {
            obj_id,
            name: Some(name_str),
        });
        AK_SUCCESS
    }

    pub fn unregister_game_obj(&self, obj_id: u64) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::UnregisterGameObj { obj_id });
        AK_SUCCESS
    }

    pub fn unregister_all_game_obj(&self) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::UnregisterAllGameObj);
        AK_SUCCESS
    }

    pub fn set_world_transform(&self, obj_id: u64, transform: WorldTransform) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.obj_transforms.insert(obj_id, transform);
        g.commands.push_back(Command::SetPosition { obj_id, transform });
        AK_SUCCESS
    }

    pub fn set_position(&self, obj_id: u64, pos: [f64; 3], front: [f32; 3], top: [f32; 3]) -> i32 {
        self.set_world_transform(
            obj_id,
            WorldTransform {
                front,
                top,
                pos,
            },
        )
    }

    /// Parse a `0x30`-byte `AkWorldTransform` (`front: [f32; 3]` @ `+0x00`,
    /// `top: [f32; 3]` @ `+0x0c`, `pos: [f64; 3]` @ `+0x18`) and queue `SetPosition`.
    pub fn set_position_bytes(&self, obj_id: u64, raw: &[u8]) -> i32 {
        let Some(transform) = parse_ak_world_transform(raw) else {
            return AK_INVALID_PARAMETER;
        };
        self.set_world_transform(obj_id, transform)
    }

    pub fn set_multiple_positions_bytes(&self, obj_id: u64, raw: &[u8], count: usize, _multi_type: i32) -> i32 {
        if count == 0 {
            return AK_SUCCESS;
        }
        if raw.len() < count * 0x30 {
            return AK_INVALID_PARAMETER;
        }
        // Use the primary (first) transform for emitter placement.
        self.set_position_bytes(obj_id, &raw[..0x30])
    }

    pub fn set_default_listeners(&self, listeners: &[u64]) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.default_listeners = listeners.to_vec();
        g.commands.push_back(Command::SetDefaultListeners {
            listeners: listeners.to_vec(),
        });
        AK_SUCCESS
    }

    pub fn set_listeners(&self, emitter: u64, listeners: &[u64]) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.obj_listeners.insert(emitter, listeners.to_vec());
        g.commands.push_back(Command::SetListeners {
            emitter,
            listeners: listeners.to_vec(),
        });
        AK_SUCCESS
    }

    pub fn set_distance_probe(&self, listener: u64, probe: u64) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.distance_probes.insert(listener, probe);
        g.commands.push_back(Command::SetDistanceProbe { listener, probe });
        AK_SUCCESS
    }

    pub fn set_scaling_factor(&self, obj_id: u64, factor: f32) -> i32 {
        if !factor.is_finite() || factor <= 0.0 {
            return AK_INVALID_PARAMETER;
        }
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.obj_scaling.insert(obj_id, factor);
        g.commands.push_back(Command::SetScalingFactor { obj_id, factor });
        AK_SUCCESS
    }

    pub fn set_object_obstruction_and_occlusion(&self, emitter: u64, listener: u64, obs: f32, occ: f32) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.obj_obs_occ.insert((emitter, listener), (obs, occ));
        g.commands.push_back(Command::SetObjectObstructionAndOcclusion {
            emitter,
            listener,
            obs,
            occ,
        });
        AK_SUCCESS
    }

    pub fn set_game_object_aux_send_values(&self, obj_id: u64, sends: Vec<(u64, u32, f32)>) -> i32 {
        let mut g = self.inner.lock().unwrap();
        let send_dbs: Vec<(u32, f32)> = sends
            .iter()
            .map(|&(_listener, bus, ctrl)| {
                let db = if ctrl <= 0.0 {
                    -96.3
                } else {
                    fast_pow_to_db(ctrl * ctrl)
                };
                (bus, db)
            })
            .collect();
        g.renderer.gs.obj_aux_sends.insert(obj_id, send_dbs);
        g.commands.push_back(Command::SetGameObjectAuxSendValues { obj_id, sends });
        AK_SUCCESS
    }

    /// Parse `count` `AkAuxSendValue` structs (`0x10` bytes each: `listener: u64` @0,
    /// `aux_bus_id: u32` @8, `control_value: f32` @12).
    pub fn set_game_object_aux_send_bytes(&self, obj_id: u64, raw: &[u8], count: usize) -> i32 {
        if raw.len() < count * 0x10 {
            return AK_INVALID_PARAMETER;
        }
        let mut sends = Vec::with_capacity(count);
        for i in 0..count {
            let s = &raw[i * 0x10..(i + 1) * 0x10];
            let listener = u64::from_le_bytes(s[0..8].try_into().unwrap());
            let bus = u32::from_le_bytes(s[8..12].try_into().unwrap());
            let val = f32::from_bits(u32::from_le_bytes(s[12..16].try_into().unwrap()));
            sends.push((listener, bus, val));
        }
        self.set_game_object_aux_send_values(obj_id, sends)
    }

    pub fn set_game_object_output_bus_volume(&self, emitter: u64, listener: u64, volume_linear: f32) -> i32 {
        let mut g = self.inner.lock().unwrap();
        let db = if volume_linear <= 0.0 {
            -96.3
        } else {
            fast_pow_to_db(volume_linear * volume_linear)
        };
        g.renderer.gs.obj_output_bus_volume_db.insert((emitter, listener), db);
        if listener != 0 {
            g.renderer.gs.obj_output_bus_volume_db.insert((emitter, 0), db);
        }
        g.commands.push_back(Command::SetGameObjectOutputBusVolume {
            emitter,
            listener,
            volume_linear,
        });
        AK_SUCCESS
    }

    // --- Game Syncs (RTPC, State, Switch, Trigger) ---

    pub fn set_rtpc_value(
        &self,
        rtpc_id: u32,
        value: f32,
        obj_id: u64,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        if transition_ms <= 0 || bypass_interp {
            if obj_id == u64::MAX || obj_id == 0 {
                g.renderer.gs.params.insert(rtpc_id, value);
            } else {
                g.renderer.gs.obj_params.insert((obj_id, rtpc_id), value);
            }
        }
        g.commands.push_back(Command::SetRtpcValue {
            rtpc_id,
            value,
            obj_id,
            transition_ms,
            curve,
            bypass_interp,
        });
        AK_SUCCESS
    }

    pub fn set_rtpc_value_by_playing_id(
        &self,
        rtpc_id: u32,
        value: f32,
        playing_id: u32,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        if transition_ms <= 0 || bypass_interp {
            g.renderer.gs.playing_id_params.insert((playing_id, rtpc_id), value);
        }
        g.commands.push_back(Command::SetRtpcValueByPlayingId {
            rtpc_id,
            value,
            playing_id,
            transition_ms,
            curve,
            bypass_interp,
        });
        AK_SUCCESS
    }

    pub fn reset_rtpc_value(
        &self,
        rtpc_id: u32,
        obj_id: u64,
        transition_ms: i32,
        curve: i32,
        bypass_interp: bool,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::ResetRtpcValue {
            rtpc_id,
            obj_id,
            transition_ms,
            curve,
            bypass_interp,
        });
        AK_SUCCESS
    }

    /// Query the current value of an RTPC (`AK::SoundEngine::Query::GetRTPCValue`).
    /// Returns `(value, value_type)` where `value_type` is 1 (Global), 2 (GameObject),
    /// 3 (PlayingID), or 4 (Unavailable).
    pub fn get_rtpc_value(&self, rtpc_id: u32, obj_id: u64, playing_id: u32) -> (f32, i32) {
        let g = self.inner.lock().unwrap();
        let gs = &g.renderer.gs;
        if playing_id != 0 {
            if let Some(&v) = gs.playing_id_params.get(&(playing_id, rtpc_id)) {
                return (v, 3);
            }
        }
        if obj_id != 0 && obj_id != u64::MAX {
            if let Some(&v) = gs.obj_params.get(&(obj_id, rtpc_id)) {
                return (v, 2);
            }
        }
        if let Some(&v) = gs.params.get(&rtpc_id) {
            return (v, 1);
        }
        let def = g.project.param_defaults.get(&rtpc_id).copied().unwrap_or(0.0);
        (def, 1)
    }

    pub fn set_switch(&self, group_id: u32, switch_id: u32, obj_id: u64) -> i32 {
        let mut g = self.inner.lock().unwrap();
        if obj_id == u64::MAX || obj_id == 0 {
            g.renderer.gs.switches.insert(group_id, switch_id);
        } else {
            g.renderer.gs.obj_switches.insert((obj_id, group_id), switch_id);
        }
        g.commands.push_back(Command::SetSwitch {
            group_id,
            switch_id,
            obj_id,
        });
        AK_SUCCESS
    }

    pub fn set_state(&self, group_id: u32, state_id: u32) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.renderer.gs.states.insert(group_id, state_id);
        g.commands.push_back(Command::SetState { group_id, state_id });
        AK_SUCCESS
    }

    pub fn post_trigger(&self, trigger_id: u32, _obj_id: u64) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::PostTrigger { trigger_id });
        AK_SUCCESS
    }

    // --- Events & Actions ---

    fn alloc_playing_id(&self, requested: u32) -> u32 {
        if requested != 0 {
            let mut cur = self.next_playing_id.load(Ordering::Relaxed);
            while cur <= requested {
                match self.next_playing_id.compare_exchange_weak(
                    cur,
                    requested.wrapping_add(1).max(1),
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(actual) => cur = actual,
                }
            }
            requested
        } else {
            let id = self.next_playing_id.fetch_add(1, Ordering::Relaxed);
            if id == 0 {
                self.next_playing_id.fetch_add(1, Ordering::Relaxed)
            } else {
                id
            }
        }
    }

    /// Post an event (`AK::SoundEngine::PostEvent`), returning the assigned `AkPlayingID`.
    pub fn post_event(
        &self,
        event_id: u32,
        game_obj_id: u64,
        flags: u32,
        callback: Option<EventCallbackTarget>,
        cookie: u64,
        external_sources: Vec<ExternalSourceDesc>,
        custom_playing_id: u32,
    ) -> u32 {
        if event_id == 0 {
            return 0;
        }
        let playing_id = self.alloc_playing_id(custom_playing_id);
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::PostEvent {
            playing_id,
            event_id,
            game_obj_id,
            flags,
            callback,
            cookie,
            external_sources,
        });
        playing_id
    }

    /// Convenience helper for posting an event with a Rust closure callback.
    pub fn post_event_rust<F>(
        &self,
        event_id: u32,
        game_obj_id: u64,
        flags: u32,
        cookie: u64,
        external_sources: Vec<ExternalSourceDesc>,
        callback: F,
    ) -> u32
    where
        F: Fn(&CallbackEvent) + Send + Sync + 'static,
    {
        self.post_event(
            event_id,
            game_obj_id,
            flags,
            Some(EventCallbackTarget::Rust(Arc::new(callback))),
            cookie,
            external_sources,
            0,
        )
    }

    pub fn execute_action_on_event(
        &self,
        event_id: u32,
        action_type: i32,
        game_obj_id: u64,
        transition_ms: i32,
        curve: i32,
        playing_id: u32,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::ExecuteActionOnEvent {
            event_id,
            action_type,
            game_obj_id,
            transition_ms,
            curve,
            playing_id,
        });
        AK_SUCCESS
    }

    pub fn execute_action_on_event_name(
        &self,
        event_name: &str,
        action_type: i32,
        game_obj_id: u64,
        transition_ms: i32,
        curve: i32,
        playing_id: u32,
    ) -> i32 {
        let event_id = fnv1_32_lower(event_name);
        self.execute_action_on_event(event_id, action_type, game_obj_id, transition_ms, curve, playing_id)
    }

    pub fn seek_on_event(
        &self,
        event_id: u32,
        game_obj_id: u64,
        position_ms: i32,
        _to_nearest_marker: bool,
        playing_id: u32,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::SeekOnEvent {
            event_id,
            game_obj_id,
            position_ms: Some(position_ms),
            percent: None,
            playing_id,
        });
        AK_SUCCESS
    }

    pub fn seek_on_event_percent(
        &self,
        event_id: u32,
        game_obj_id: u64,
        percent: f32,
        _to_nearest_marker: bool,
        playing_id: u32,
    ) -> i32 {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::SeekOnEvent {
            event_id,
            game_obj_id,
            position_ms: None,
            percent: Some(percent),
            playing_id,
        });
        AK_SUCCESS
    }

    pub fn stop_all(&self, game_obj_id: u64) {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::StopAll { game_obj_id });
    }

    pub fn stop_playing_id(&self, playing_id: u32, transition_ms: i32, curve: i32) {
        let mut g = self.inner.lock().unwrap();
        g.commands.push_back(Command::StopPlayingId {
            playing_id,
            transition_ms,
            curve,
        });
    }

    /// `AK::SoundEngine::GetSourcePlayPosition`.
    pub fn get_source_play_position(&self, playing_id: u32) -> Result<i32, i32> {
        let mut g = self.inner.lock().unwrap();
        let Some(tracker) = g.playing.get(&playing_id) else {
            return Err(AK_FAIL);
        };
        if (tracker.flags & AK_ENABLE_GET_SOURCE_PLAY_POSITION) == 0 {
            return Err(AK_FAIL);
        }
        let EngineInner { project, renderer, .. } = &mut *g;
        let pos = renderer.with_project(project, |r| r.source_play_position_ms(playing_id));
        pos.ok_or(AK_FAIL)
    }

    /// `AK::SoundEngine::Query::GetPosition`: writes `AkWorldTransform` (`0x30` bytes) for `obj_id`.
    pub fn get_position_bytes(&self, obj_id: u64, out: &mut [u8; 0x30]) -> i32 {
        let g = self.inner.lock().unwrap();
        let wt = g
            .renderer
            .gs
            .obj_transforms
            .get(&obj_id)
            .copied()
            .unwrap_or_else(|| WorldTransform::default());
        for (i, f) in wt.front.iter().enumerate() {
            out[i * 4..(i + 1) * 4].copy_from_slice(&f.to_le_bytes());
        }
        for (i, t) in wt.top.iter().enumerate() {
            out[12 + i * 4..12 + (i + 1) * 4].copy_from_slice(&t.to_le_bytes());
        }
        for (i, p) in wt.pos.iter().enumerate() {
            out[24 + i * 8..24 + (i + 1) * 8].copy_from_slice(&p.to_le_bytes());
        }
        AK_SUCCESS
    }

    /// `AK::SoundEngine::Query::GetPlayingIDsFromGameObject`.
    pub fn get_playing_ids_from_game_object(&self, obj_id: u64) -> Vec<u32> {
        let mut g = self.inner.lock().unwrap();
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| r.playing_ids_for_game_object(obj_id))
    }

    /// `AK::SoundEngine::Query::GetActiveGameObjects`.
    pub fn active_game_objects(&self) -> Vec<u64> {
        let g = self.inner.lock().unwrap();
        let mut out: Vec<u64> = g.renderer.gs.registered_objects.keys().copied().collect();
        out.sort_unstable();
        out
    }

    /// `AK::SoundEngine::Query::GetState`.
    pub fn get_state(&self, group_id: u32) -> u32 {
        let g = self.inner.lock().unwrap();
        g.renderer.gs.states.get(&group_id).copied().unwrap_or(0)
    }

    /// `AK::SoundEngine::Query::GetSwitch`.
    pub fn get_switch(&self, group_id: u32, obj_id: u64) -> u32 {
        let g = self.inner.lock().unwrap();
        if obj_id != 0 && obj_id != u64::MAX {
            if let Some(&s) = g.renderer.gs.obj_switches.get(&(obj_id, group_id)) {
                return s;
            }
        }
        g.renderer.gs.switches.get(&group_id).copied().unwrap_or(0)
    }

    /// `AK::SoundEngine::Query::GetEventIDFromPlayingID`.
    pub fn get_event_id_from_playing_id(&self, playing_id: u32) -> u32 {
        let g = self.inner.lock().unwrap();
        g.playing.get(&playing_id).map_or(0, |t| t.event_id)
    }

    /// `AK::SoundEngine::Query::GetGameObjectFromPlayingID`.
    pub fn get_game_object_from_playing_id(&self, playing_id: u32) -> u64 {
        let g = self.inner.lock().unwrap();
        g.playing.get(&playing_id).map_or(u64::MAX, |t| t.game_obj_id)
    }

    /// `AK::SoundEngine::Query::GetIsGameObjectActive`.
    pub fn is_game_object_active(&self, obj_id: u64) -> bool {
        !self.get_playing_ids_from_game_object(obj_id).is_empty()
    }

    /// `AK::MusicEngine::GetPlayingSegmentInfo`.
    pub fn get_playing_segment_info(&self, playing_id: u32) -> Option<AkSegmentInfo> {
        let mut g = self.inner.lock().unwrap();
        let EngineInner { project, renderer, .. } = &mut *g;
        renderer.with_project(project, |r| {
            let sr = RATE as f64;
            let now_sample = r.now();
            for seg in r.active_music_segments_for_playing_id(playing_id) {
                let Some(seg_obj) = r.project.node(seg.segment_id) else {
                    continue;
                };
                let HircBody::MusicSegment(m) = &seg_obj.body else {
                    continue;
                };
                let bounds = SegmentBounds::from_segment(m);
                let cur_sec = now_sample.saturating_sub(seg.origin_sample) as f64 / sr;
                let entry_sec = bounds.entry_ms * 0.001;
                let total_sec = bounds.duration_ms * 0.001;
                let pre_entry_ms = bounds.entry_ms.round() as i32;
                let active_ms = (bounds.exit_ms - bounds.entry_ms).max(0.0).round() as i32;
                let post_exit_ms = (total_sec * 1000.0 - bounds.exit_ms).max(0.0).round() as i32;
                let cur_pos_ms = ((cur_sec - entry_sec) * 1000.0).round() as i32;

                let tempo = if m.music.meter.tempo > 0.0 {
                    m.music.meter.tempo as f64
                } else {
                    120.0
                };
                let beat_val = if m.music.meter.time_sig_beat_value > 0 {
                    m.music.meter.time_sig_beat_value as f64
                } else {
                    4.0
                };
                let beats_per_bar = if m.music.meter.time_sig_num_beats > 0 {
                    m.music.meter.time_sig_num_beats as f64
                } else {
                    4.0
                };
                let beat_sec_f64 = (60.0 / tempo) * (4.0 / beat_val);
                let bar_sec_f64 = beat_sec_f64 * beats_per_bar;
                let grid_sec_f64 = if m.music.meter.grid_period > 0.0 {
                    (m.music.meter.grid_period * 0.001).max(0.001)
                } else {
                    beat_sec_f64.max(0.001)
                };
                return Some(AkSegmentInfo {
                    i_current_position: cur_pos_ms,
                    i_pre_entry_duration: pre_entry_ms,
                    i_active_duration: active_ms,
                    i_post_exit_duration: post_exit_ms,
                    i_remaining_look_ahead_time: 0,
                    f_beat_duration: beat_sec_f64 as f32,
                    f_bar_duration: bar_sec_f64 as f32,
                    f_grid_duration: grid_sec_f64 as f32,
                    f_grid_offset: (m.music.meter.grid_offset * 0.001) as f32,
                });
            }
            None
        })
    }

    pub fn pin_event(&self, _event_id: u32, _priority: i8, _active: i8) -> i32 {
        AK_SUCCESS
    }

    pub fn unpin_event(&self, _event_id: u32) -> i32 {
        AK_SUCCESS
    }

    pub fn add_output(&self, _settings: &[u8], _listeners: &[u64]) -> (i32, u64) {
        (AK_SUCCESS, 1)
    }

    pub fn remove_output(&self, _output_id: u64) -> i32 {
        AK_SUCCESS
    }

    pub fn set_speaker_angles(&self, _angles: &[f32], _height: f32, _dev: u64) -> i32 {
        AK_SUCCESS
    }

    /// Snapshot of all callbacks fired so far (useful for tests & diagnostics).
    pub fn callback_history(&self) -> Vec<CallbackEvent> {
        self.inner.lock().unwrap().callback_history.clone()
    }

    // --- Frame Rendering, Command Drain, RTPC Ramping & Callback Dispatch ---

    /// Drain queued commands, step RTPC ramps, render one audio frame (`FRAME` = 512
    /// samples per channel at 48 kHz), push to output sink / capture WAV, and dispatch
    /// global + event callbacks.
    pub fn render_audio(&self, _allow_sync: bool) -> Result<Vec<f32>, String> {
        let mut pending_global_begin: Vec<(GlobalCallbackTarget, u64)> = Vec::new();
        let mut pending_global_end: Vec<(GlobalCallbackTarget, u64)> = Vec::new();
        let mut pending_events: Vec<(Option<EventCallbackTarget>, CallbackEvent)> = Vec::new();
        let mut sink_out: Option<(Arc<dyn AudioSink>, usize, u32)> = None;

        let frame_samples = {
            let mut g = self.inner.lock().unwrap();
            for gc in &g.global_callbacks {
                if (gc.location_mask & LOC_BEGIN_RENDER) != 0 {
                    pending_global_begin.push((gc.callback.clone(), gc.cookie));
                }
                if (gc.location_mask & LOC_END_RENDER) != 0 {
                    pending_global_end.push((gc.callback.clone(), gc.cookie));
                }
            }

            let EngineInner {
                project,
                renderer,
                commands,
                rtpc_ramps,
                playing,
                callback_history,
                sink,
                capture,
                ..
            } = &mut *g;

            let samples = renderer.with_project(project, |r| -> Result<Vec<f32>, String> {
                // 1. Drain queued commands in FIFO order.
                drain_commands(r, commands, rtpc_ramps, playing)?;

                // 2. Step active RTPC ramps for the current sample timestamp.
                step_rtpc_ramps(r, rtpc_ramps);

                // 3. Render one quantum frame (which executes due Pending actions and starts voices).
                let mut out = Vec::with_capacity(FRAME * r.opts.out_channels);
                r.render_frame(&mut out)?;

                // 4. Fire AK_Duration callbacks for voices that became active this frame.
                collect_duration_callbacks(r, playing, &mut pending_events);

                // 5. Check music sync crossings and end-of-event transitions.
                collect_music_sync_callbacks(r, playing, &mut pending_events);
                collect_end_of_event_callbacks(r, playing, &mut pending_events);

                Ok(out)
            })?;

            for (_, ev) in &pending_events {
                callback_history.push(ev.clone());
            }

            if let Some(cap) = capture.as_mut() {
                cap.samples.extend_from_slice(&samples);
            }
            if let Some(s) = sink.as_ref() {
                sink_out = Some((s.clone(), renderer.opts.out_channels, RATE));
            }
            samples
        };

        self.frames_rendered.fetch_add(1, Ordering::Relaxed);

        // Fire BeginRender callbacks outside the engine mutex so guest callbacks
        // can re-enter `AK::SoundEngine::*` without deadlocking!
        for (cb, cookie) in pending_global_begin {
            match cb {
                GlobalCallbackTarget::Rust(f) => f(LOC_BEGIN_RENDER, cookie),
                GlobalCallbackTarget::Win64(f) => unsafe {
                    f(std::ptr::null_mut(), LOC_BEGIN_RENDER, cookie as *mut c_void);
                },
            }
        }

        // Push rendered audio to output sink.
        if let Some((sink, ch, sr)) = sink_out {
            sink.push_samples(&frame_samples, ch, sr);
        }

        // Fire event callbacks outside the lock.
        for (target, ev) in pending_events {
            if let Some(cb) = target {
                cb.invoke(&ev);
            }
        }

        // Fire EndRender callbacks outside the lock.
        for (cb, cookie) in pending_global_end {
            match cb {
                GlobalCallbackTarget::Rust(f) => f(LOC_END_RENDER, cookie),
                GlobalCallbackTarget::Win64(f) => unsafe {
                    f(std::ptr::null_mut(), LOC_END_RENDER, cookie as *mut c_void);
                },
            }
        }

        Ok(frame_samples)
    }

    /// Render `count` consecutive audio frames and concatenate their interleaved samples.
    pub fn render_frames(&self, count: usize) -> Result<Vec<f32>, String> {
        let mut all = Vec::new();
        for _ in 0..count {
            let f = self.render_audio(true)?;
            all.extend_from_slice(&f);
        }
        Ok(all)
    }

    /// Return all Event ShortIDs currently registered across loaded SoundBanks.
    pub fn loaded_event_ids(&self) -> Vec<u32> {
        self.inner.lock().unwrap().project.events()
    }

    /// Mix 512-sample stereo frames (`FRAME * 2` floats per quantum) into `out` via [`Self::render_audio`],
    /// returning the number of stereo frames (samples per channel) written.
    pub fn mix_stereo_interleaved(&self, out: &mut [f32]) -> Result<usize, String> {
        let mut written = 0usize;
        while written + FRAME * 2 <= out.len() {
            let frame = self.render_audio(true)?;
            let n = frame.len().min(FRAME * 2);
            out[written..written + n].copy_from_slice(&frame[..n]);
            for s in &mut out[written + n..written + FRAME * 2] {
                *s = 0.0;
            }
            written += FRAME * 2;
        }
        Ok(written / 2)
    }
}

fn drain_commands(
    r: &mut Renderer<'_>,
    commands: &mut VecDeque<Command>,
    rtpc_ramps: &mut HashMap<(u32, RtpcTarget), RtpcRamp>,
    playing: &mut HashMap<u32, PlayingTracker>,
) -> Result<(), String> {
    let sr = RATE as f64;
    let now = r.now();
    let mut rtpc_dirty = false;

    while let Some(cmd) = commands.pop_front() {
        match cmd {
            Command::RegisterGameObj { obj_id, name } => {
                r.gs.registered_objects
                    .insert(obj_id, name.unwrap_or_default());
            }
            Command::UnregisterGameObj { obj_id } => {
                r.gs.registered_objects.remove(&obj_id);
                r.gs.obj_transforms.remove(&obj_id);
                r.gs.obj_scaling.remove(&obj_id);
                r.gs.obj_aux_sends.remove(&obj_id);
                r.gs.obj_listeners.remove(&obj_id);
                r.gs.obj_params.retain(|&(o, _), _| o != obj_id);
                r.gs.obj_switches.retain(|&(o, _), _| o != obj_id);
            }
            Command::UnregisterAllGameObj => {
                r.gs.registered_objects.clear();
                r.gs.obj_transforms.clear();
                r.gs.obj_scaling.clear();
                r.gs.obj_aux_sends.clear();
                r.gs.obj_listeners.clear();
                r.gs.obj_params.clear();
                r.gs.obj_switches.clear();
            }
            Command::SetPosition { obj_id, transform } => {
                r.gs.obj_transforms.insert(obj_id, transform);
                if r.gs.default_listeners.contains(&obj_id) || r.gs.default_listeners.is_empty() {
                    r.gs.listener = transform;
                }
                rtpc_dirty = true;
            }
            Command::SetDefaultListeners { listeners } => {
                if let Some(&first) = listeners.first() {
                    if let Some(&t) = r.gs.obj_transforms.get(&first) {
                        r.gs.listener = t;
                    }
                }
                r.gs.default_listeners = listeners;
                rtpc_dirty = true;
            }
            Command::SetListeners { emitter, listeners } => {
                r.gs.obj_listeners.insert(emitter, listeners);
                rtpc_dirty = true;
            }
            Command::SetDistanceProbe { listener, probe } => {
                r.gs.distance_probes.insert(listener, probe);
            }
            Command::SetScalingFactor { obj_id, factor } => {
                r.gs.obj_scaling.insert(obj_id, factor);
                rtpc_dirty = true;
            }
            Command::SetObjectObstructionAndOcclusion {
                emitter,
                listener,
                obs,
                occ,
            } => {
                r.gs.obj_obs_occ.insert((emitter, listener), (obs, occ));
                rtpc_dirty = true;
            }
            Command::SetGameObjectAuxSendValues { obj_id, sends } => {
                let send_dbs: Vec<(u32, f32)> = sends
                    .iter()
                    .map(|&(_listener, bus, ctrl)| {
                        let db = if ctrl <= 0.0 {
                            -96.3
                        } else {
                            fast_pow_to_db(ctrl * ctrl)
                        };
                        (bus, db)
                    })
                    .collect();
                r.gs.obj_aux_sends.insert(obj_id, send_dbs);
                rtpc_dirty = true;
            }
            Command::SetGameObjectOutputBusVolume {
                emitter,
                listener,
                volume_linear,
            } => {
                let db = if volume_linear <= 0.0 {
                    -96.3
                } else {
                    fast_pow_to_db(volume_linear * volume_linear)
                };
                r.gs.obj_output_bus_volume_db.insert((emitter, listener), db);
                if listener != 0 {
                    r.gs.obj_output_bus_volume_db.insert((emitter, 0), db);
                }
                rtpc_dirty = true;
            }
            Command::SetRtpcValue {
                rtpc_id,
                value,
                obj_id,
                transition_ms,
                curve,
                bypass_interp,
            } => {
                let target = if obj_id == u64::MAX || obj_id == 0 {
                    RtpcTarget::Global
                } else {
                    RtpcTarget::GameObject(obj_id)
                };
                schedule_rtpc_change(r, rtpc_ramps, rtpc_id, target, value, transition_ms, curve, bypass_interp, now, sr);
                rtpc_dirty = true;
            }
            Command::SetRtpcValueByPlayingId {
                rtpc_id,
                value,
                playing_id,
                transition_ms,
                curve,
                bypass_interp,
            } => {
                let target = RtpcTarget::PlayingId(playing_id);
                schedule_rtpc_change(r, rtpc_ramps, rtpc_id, target, value, transition_ms, curve, bypass_interp, now, sr);
                rtpc_dirty = true;
            }
            Command::ResetRtpcValue {
                rtpc_id,
                obj_id,
                transition_ms,
                curve,
                bypass_interp,
            } => {
                let default_val = r.project.param_defaults.get(&rtpc_id).copied().unwrap_or(0.0);
                let target = if obj_id == u64::MAX || obj_id == 0 {
                    RtpcTarget::Global
                } else {
                    RtpcTarget::GameObject(obj_id)
                };
                schedule_rtpc_change(
                    r,
                    rtpc_ramps,
                    rtpc_id,
                    target,
                    default_val,
                    transition_ms,
                    curve,
                    bypass_interp,
                    now,
                    sr,
                );
                rtpc_dirty = true;
            }
            Command::SetSwitch {
                group_id,
                switch_id,
                obj_id,
            } => {
                let obj_opt = (obj_id != 0 && obj_id != u64::MAX).then_some(obj_id);
                let _ = r.set_switch(group_id, switch_id, obj_opt);
            }
            Command::SetState { group_id, state_id } => {
                let _ = r.set_state(group_id, state_id);
            }
            Command::PostTrigger { trigger_id } => {
                let _ = r.post_trigger(trigger_id);
            }
            Command::PostEvent {
                playing_id,
                event_id,
                game_obj_id,
                flags,
                callback,
                cookie,
                external_sources,
            } => {
                let _ = r.post_event_ex(event_id, now, playing_id, game_obj_id, external_sources);
                playing.insert(
                    playing_id,
                    PlayingTracker {
                        playing_id,
                        event_id,
                        game_obj_id,
                        flags,
                        callback,
                        cookie,
                        duration_sent: false,
                        music_seg_prev_sec: HashMap::new(),
                    },
                );
            }
            Command::ExecuteActionOnEvent {
                event_id,
                action_type,
                game_obj_id,
                transition_ms,
                curve,
                playing_id,
            } => {
                let obj_opt = (game_obj_id != 0 && game_obj_id != u64::MAX).then_some(game_obj_id);
                r.execute_action_on_event(
                    event_id,
                    action_type,
                    obj_opt,
                    transition_ms,
                    curve.clamp(0, 9) as u32,
                    playing_id,
                );
            }
            Command::SeekOnEvent {
                event_id,
                game_obj_id,
                position_ms,
                percent,
                playing_id,
            } => {
                let obj_opt = (game_obj_id != 0 && game_obj_id != u64::MAX).then_some(game_obj_id);
                if let Some(pct) = percent {
                    r.seek_on_event(event_id, obj_opt, pct, true, playing_id);
                } else if let Some(ms) = position_ms {
                    r.seek_on_event(event_id, obj_opt, ms as f32, false, playing_id);
                }
            }
            Command::StopAll { game_obj_id } => {
                let obj_opt = (game_obj_id != 0 && game_obj_id != u64::MAX).then_some(game_obj_id);
                r.stop_all(obj_opt);
            }
            Command::StopPlayingId {
                playing_id,
                transition_ms,
                curve,
            } => {
                r.stop_playing_id(playing_id, transition_ms, curve.clamp(0, 9) as u32);
            }
        }
    }

    if rtpc_dirty {
        r.refresh_voices_rtpc();
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn schedule_rtpc_change(
    r: &mut Renderer<'_>,
    rtpc_ramps: &mut HashMap<(u32, RtpcTarget), RtpcRamp>,
    rtpc_id: u32,
    target: RtpcTarget,
    value: f32,
    transition_ms: i32,
    curve: i32,
    bypass_interp: bool,
    now: u64,
    sr: f64,
) {
    let cur = match target {
        RtpcTarget::Global => r
            .gs
            .params
            .get(&rtpc_id)
            .copied()
            .or_else(|| r.project.param_defaults.get(&rtpc_id).copied())
            .unwrap_or(0.0),
        RtpcTarget::GameObject(obj) => r
            .gs
            .obj_params
            .get(&(obj, rtpc_id))
            .copied()
            .or_else(|| r.gs.params.get(&rtpc_id).copied())
            .or_else(|| r.project.param_defaults.get(&rtpc_id).copied())
            .unwrap_or(0.0),
        RtpcTarget::PlayingId(pid) => r
            .gs
            .playing_id_params
            .get(&(pid, rtpc_id))
            .copied()
            .or_else(|| r.gs.params.get(&rtpc_id).copied())
            .or_else(|| r.project.param_defaults.get(&rtpc_id).copied())
            .unwrap_or(0.0),
    };
    if transition_ms <= 0 || bypass_interp || (value - cur).abs() <= 1e-7 {
        rtpc_ramps.remove(&(rtpc_id, target));
        apply_rtpc_target(&mut r.gs, rtpc_id, target, value);
    } else {
        let dur_samples = ((transition_ms as f64 * 0.001) * sr).round().max(1.0) as u64;
        rtpc_ramps.insert(
            (rtpc_id, target),
            RtpcRamp {
                rtpc_id,
                target,
                start_val: cur,
                end_val: value,
                start_sample: now,
                duration_samples: dur_samples,
                curve: curve.clamp(0, 9) as u8,
            },
        );
    }
}

fn apply_rtpc_target(gs: &mut GameState, rtpc_id: u32, target: RtpcTarget, val: f32) {
    match target {
        RtpcTarget::Global => {
            gs.params.insert(rtpc_id, val);
        }
        RtpcTarget::GameObject(obj) => {
            gs.obj_params.insert((obj, rtpc_id), val);
        }
        RtpcTarget::PlayingId(pid) => {
            gs.playing_id_params.insert((pid, rtpc_id), val);
        }
    }
}

fn step_rtpc_ramps(r: &mut Renderer<'_>, rtpc_ramps: &mut HashMap<(u32, RtpcTarget), RtpcRamp>) {
    if rtpc_ramps.is_empty() {
        return;
    }
    let now = r.now().saturating_add(FRAME as u64);
    let mut finished = Vec::new();
    for (&key, ramp) in rtpc_ramps.iter() {
        let elapsed = now.saturating_sub(ramp.start_sample);
        if elapsed >= ramp.duration_samples {
            apply_rtpc_target(&mut r.gs, ramp.rtpc_id, ramp.target, ramp.end_val);
            finished.push(key);
        } else {
            let t = (elapsed as f32 / ramp.duration_samples.max(1) as f32).clamp(0.0, 1.0);
            let k = interp_shape(ramp.curve as u32, t);
            let val = ramp.start_val + (ramp.end_val - ramp.start_val) * k;
            apply_rtpc_target(&mut r.gs, ramp.rtpc_id, ramp.target, val);
        }
    }
    for k in finished {
        rtpc_ramps.remove(&k);
    }
    r.refresh_voices_rtpc();
}

fn collect_duration_callbacks(
    r: &Renderer<'_>,
    playing: &mut HashMap<u32, PlayingTracker>,
    out: &mut Vec<(Option<EventCallbackTarget>, CallbackEvent)>,
) {
    for tracker in playing.values_mut() {
        if tracker.duration_sent {
            continue;
        }
        if (tracker.flags & AK_DURATION) != 0 {
            if let Some((dur_ms, node_id, media_id)) = r.playing_id_duration_info(tracker.playing_id) {
                tracker.duration_sent = true;
                out.push((
                    tracker.callback.clone(),
                    CallbackEvent {
                        callback_type: AK_DURATION,
                        cookie: tracker.cookie,
                        game_obj_id: tracker.game_obj_id,
                        playing_id: tracker.playing_id,
                        event_id: tracker.event_id,
                        payload: CallbackPayload::Duration {
                            duration_ms: dur_ms,
                            estimated_duration_ms: dur_ms,
                            audio_node_id: node_id,
                            media_id,
                            streaming: false,
                        },
                    },
                ));
            }
        } else {
            tracker.duration_sent = true;
        }
    }
}

fn collect_music_sync_callbacks(
    r: &Renderer<'_>,
    playing: &mut HashMap<u32, PlayingTracker>,
    out: &mut Vec<(Option<EventCallbackTarget>, CallbackEvent)>,
) {
    let sr = RATE as f64;
    let dt_sec = FRAME as f64 / sr;
    let now_sample = r.now();
    for tracker in playing.values_mut() {
        let sync_mask = tracker.flags & AK_MUSIC_SYNC_ALL;
        if sync_mask == 0 {
            continue;
        }
        for seg in r.active_music_segments_for_playing_id(tracker.playing_id) {
            if now_sample < seg.origin_sample || now_sample > seg.end_sample + FRAME as u64 {
                continue;
            }
            let Some(seg_obj) = r.project.node(seg.segment_id) else {
                continue;
            };
            let HircBody::MusicSegment(m) = &seg_obj.body else {
                continue;
            };
            let bounds = SegmentBounds::from_segment(m);
            let cur_sec = now_sample.saturating_sub(seg.origin_sample) as f64 / sr;
            let prev_sec = tracker
                .music_seg_prev_sec
                .insert(seg.segment_id, cur_sec)
                .unwrap_or((cur_sec - dt_sec).max(0.0));

            let entry_sec = bounds.entry_ms * 0.001;
            let exit_sec = bounds.exit_ms * 0.001;
            let total_sec = bounds.duration_ms * 0.001;
            let pre_entry_ms = bounds.entry_ms.round() as i32;
            let active_ms = (bounds.exit_ms - bounds.entry_ms).max(0.0).round() as i32;
            let post_exit_ms = (total_sec * 1000.0 - bounds.exit_ms).max(0.0).round() as i32;
            let cur_pos_ms = ((cur_sec - entry_sec) * 1000.0).round() as i32;

            let tempo = if m.music.meter.tempo > 0.0 {
                m.music.meter.tempo as f64
            } else {
                120.0
            };
            let beat_val = if m.music.meter.time_sig_beat_value > 0 {
                m.music.meter.time_sig_beat_value as f64
            } else {
                4.0
            };
            let beats_per_bar = if m.music.meter.time_sig_num_beats > 0 {
                m.music.meter.time_sig_num_beats as f64
            } else {
                4.0
            };
            let beat_sec_f64 = (60.0 / tempo) * (4.0 / beat_val);
            let bar_sec_f64 = beat_sec_f64 * beats_per_bar;
            let grid_sec_f64 = if m.music.meter.grid_period > 0.0 {
                (m.music.meter.grid_period * 0.001).max(0.001)
            } else {
                beat_sec_f64.max(0.001)
            };

            let mut fire = |sync_type: u32, cue_name: Option<String>| {
                out.push((
                    tracker.callback.clone(),
                    CallbackEvent {
                        callback_type: sync_type,
                        cookie: tracker.cookie,
                        game_obj_id: tracker.game_obj_id,
                        playing_id: tracker.playing_id,
                        event_id: tracker.event_id,
                        payload: CallbackPayload::MusicSync {
                            current_position_ms: cur_pos_ms,
                            pre_entry_duration_ms: pre_entry_ms,
                            active_duration_ms: active_ms,
                            post_exit_duration_ms: post_exit_ms,
                            beat_duration_s: beat_sec_f64 as f32,
                            bar_duration_s: bar_sec_f64 as f32,
                            grid_duration_s: grid_sec_f64 as f32,
                            grid_offset_s: (m.music.meter.grid_offset * 0.001) as f32,
                            user_cue_name: cue_name,
                        },
                    },
                ));
            };

            if (sync_mask & AK_MUSIC_SYNC_ENTRY) != 0 && prev_sec <= entry_sec && cur_sec >= entry_sec {
                fire(AK_MUSIC_SYNC_ENTRY, None);
            }
            if (sync_mask & AK_MUSIC_SYNC_BEAT) != 0 {
                let b0 = ((prev_sec - entry_sec) / beat_sec_f64).floor() as i64;
                let b1 = ((cur_sec - entry_sec) / beat_sec_f64).floor() as i64;
                if b1 > b0 && cur_sec >= entry_sec {
                    fire(AK_MUSIC_SYNC_BEAT, None);
                }
            }
            if (sync_mask & AK_MUSIC_SYNC_BAR) != 0 {
                let b0 = ((prev_sec - entry_sec) / bar_sec_f64).floor() as i64;
                let b1 = ((cur_sec - entry_sec) / bar_sec_f64).floor() as i64;
                if b1 > b0 && cur_sec >= entry_sec {
                    fire(AK_MUSIC_SYNC_BAR, None);
                }
            }
            if (sync_mask & AK_MUSIC_SYNC_GRID) != 0 {
                let b0 = ((prev_sec - entry_sec) / grid_sec_f64).floor() as i64;
                let b1 = ((cur_sec - entry_sec) / grid_sec_f64).floor() as i64;
                if b1 > b0 && cur_sec >= entry_sec {
                    fire(AK_MUSIC_SYNC_GRID, None);
                }
            }
            if (sync_mask & AK_MUSIC_SYNC_USER_CUE) != 0 {
                for marker in &m.markers {
                    if !MARKER_ENTRY_IDS.contains(&marker.id) && !MARKER_EXIT_IDS.contains(&marker.id) {
                        let m_sec = marker.position_ms * 0.001;
                        if prev_sec < m_sec && cur_sec >= m_sec {
                            fire(AK_MUSIC_SYNC_USER_CUE, Some(marker.name.clone()));
                        }
                    }
                }
            }
            if (sync_mask & AK_MUSIC_SYNC_EXIT) != 0 && prev_sec < exit_sec && cur_sec >= exit_sec {
                fire(AK_MUSIC_SYNC_EXIT, None);
            }
        }
    }
}

fn collect_end_of_event_callbacks(
    r: &Renderer<'_>,
    playing: &mut HashMap<u32, PlayingTracker>,
    out: &mut Vec<(Option<EventCallbackTarget>, CallbackEvent)>,
) {
    let mut finished = Vec::new();
    for (&pid, tracker) in playing.iter() {
        if !r.is_playing_id_active(pid) {
            finished.push(pid);
            if (tracker.flags & AK_END_OF_EVENT) != 0 {
                out.push((
                    tracker.callback.clone(),
                    CallbackEvent {
                        callback_type: AK_END_OF_EVENT,
                        cookie: tracker.cookie,
                        game_obj_id: tracker.game_obj_id,
                        playing_id: tracker.playing_id,
                        event_id: tracker.event_id,
                        payload: CallbackPayload::EndOfEvent,
                    },
                ));
            }
        }
    }
    for pid in finished {
        playing.remove(&pid);
    }
}

/// Parse a `0x30`-byte `AkWorldTransform`:
/// `orientationFront` (3×f32 @ `+0x00`), `orientationTop` (3×f32 @ `+0x0c`), `position` (3×f64 @ `+0x18`).
pub fn parse_ak_world_transform(raw: &[u8]) -> Option<WorldTransform> {
    if raw.len() < 0x30 {
        return None;
    }
    let f32_at = |o: usize| f32::from_bits(u32::from_le_bytes(raw[o..o + 4].try_into().unwrap()));
    let f64_at = |o: usize| f64::from_bits(u64::from_le_bytes(raw[o..o + 8].try_into().unwrap()));
    Some(WorldTransform {
        front: [f32_at(0x00), f32_at(0x04), f32_at(0x08)],
        top: [f32_at(0x0c), f32_at(0x10), f32_at(0x14)],
        pos: [f64_at(0x18), f64_at(0x20), f64_at(0x28)],
    })
}

// ---------------------------------------------------------------------------
// AW2AKT01 Trace Replay Driver
// ---------------------------------------------------------------------------

const TRACE_MAGIC: &[u8; 8] = b"AW2AKT01";
const TRACE_HEADER_LEN: usize = 14;

#[derive(Debug, Clone)]
pub struct TraceReplayReport {
    pub records_total: usize,
    pub records_applied: usize,
    pub frames_rendered: u32,
    pub banks_loaded: usize,
    pub events_posted: usize,
    pub callbacks_fired: usize,
    pub samples: Vec<f32>,
}

struct TraceDec<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> TraceDec<'a> {
    fn new(b: &'a [u8]) -> Self {
        Self { b, p: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        if self.p + n > self.b.len() {
            return Err(format!("trace payload truncated at {} (need {n})", self.p));
        }
        let s = &self.b[self.p..self.p + n];
        self.p += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Result<bool, String> {
        Ok(self.u8()? != 0)
    }
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(self.u32()? as i32)
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn blob(&mut self) -> Result<&'a [u8], String> {
        let n = self.u32()? as usize;
        self.take(n)
    }
    fn wstr(&mut self) -> Result<String, String> {
        let n = self.u32()? as usize;
        let b = self.take(n * 2)?;
        let u16s: Vec<u16> = b.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        Ok(String::from_utf16_lossy(&u16s))
    }
    fn ext(&mut self) -> Result<ExternalSourceDesc, String> {
        let cookie = self.u32()?;
        let codec = self.u32()?;
        let file_id = self.u32()?;
        let _mem_size = self.u32()?;
        let _has_mem = self.bool()?;
        let path_s = self.wstr()?;
        Ok(ExternalSourceDesc {
            cookie,
            codec,
            file_id,
            path: (!path_s.is_empty()).then_some(path_s),
            bytes: None,
        })
    }
}

impl NativeSoundEngine {
    /// Replay a compact `AW2AKT01` binary trace against this `NativeSoundEngine`.
    pub fn replay_trace(
        &self,
        trace_bytes: &[u8],
        banks_dir: &Path,
        tail_frames: usize,
    ) -> Result<TraceReplayReport, String> {
        if trace_bytes.len() < TRACE_MAGIC.len() || &trace_bytes[..TRACE_MAGIC.len()] != TRACE_MAGIC {
            return Err("not an AW2AKT01 trace (bad magic)".into());
        }
        let mut p = TRACE_MAGIC.len();
        let mut records_total = 0usize;
        let mut records_applied = 0usize;
        let mut banks_loaded = 0usize;
        let mut events_posted = 0usize;
        let mut samples = Vec::new();
        let mut pid_map: HashMap<u32, u32> = HashMap::new();

        while p + TRACE_HEADER_LEN <= trace_bytes.len() {
            let h = &trace_bytes[p..p + TRACE_HEADER_LEN];
            let op = h[0];
            let ret = u32::from_le_bytes(h[6..10].try_into().unwrap());
            let len = u32::from_le_bytes(h[10..14].try_into().unwrap()) as usize;
            if p + TRACE_HEADER_LEN + len > trace_bytes.len() {
                break;
            }
            let payload = &trace_bytes[p + TRACE_HEADER_LEN..p + TRACE_HEADER_LEN + len];
            p += TRACE_HEADER_LEN + len;
            records_total += 1;

            let mut d = TraceDec::new(payload);
            match op {
                // Init, Term, MusicInit, MusicTerm, RegisterGlobalCallback, RegisterBusVolumeCallback
                1..=6 | 47 | 48 | 60..=62 => {}
                10 => {
                    // RenderAudio
                    let allow_sync = d.bool().unwrap_or(true);
                    let f = self.render_audio(allow_sync)?;
                    samples.extend_from_slice(&f);
                    records_applied += 1;
                }
                11 => {
                    self.register_game_obj(d.u64()?, None);
                    records_applied += 1;
                }
                12 => {
                    self.unregister_game_obj(d.u64()?);
                    records_applied += 1;
                }
                13 => {
                    self.unregister_all_game_obj();
                    records_applied += 1;
                }
                14 => {
                    let obj = d.u64()?;
                    let raw = d.take(0x30)?;
                    let _flags = d.i32()?;
                    self.set_position_bytes(obj, raw);
                    records_applied += 1;
                }
                15 => {
                    let obj = d.u64()?;
                    let n = d.u16()? as usize;
                    let ty = d.i32()?;
                    let _flags = d.i32()?;
                    let raw = d.take(n * 0x30)?;
                    self.set_multiple_positions_bytes(obj, raw, n, ty);
                    records_applied += 1;
                }
                16 => {
                    let n = d.u32()? as usize;
                    let mut l = Vec::with_capacity(n);
                    for _ in 0..n {
                        l.push(d.u64()?);
                    }
                    self.set_default_listeners(&l);
                    records_applied += 1;
                }
                17 => {
                    self.set_distance_probe(d.u64()?, d.u64()?);
                    records_applied += 1;
                }
                18 => {
                    self.set_scaling_factor(d.u64()?, d.f32()?);
                    records_applied += 1;
                }
                19 => {
                    self.set_object_obstruction_and_occlusion(d.u64()?, d.u64()?, d.f32()?, d.f32()?);
                    records_applied += 1;
                }
                20 => {
                    let obj = d.u64()?;
                    let n = d.u32()? as usize;
                    let raw = d.take(n * 0x10)?;
                    self.set_game_object_aux_send_bytes(obj, raw, n);
                    records_applied += 1;
                }
                21 => {
                    self.set_game_object_output_bus_volume(d.u64()?, d.u64()?, d.f32()?);
                    records_applied += 1;
                }
                30 => {
                    self.set_rtpc_value(d.u32()?, d.f32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?);
                    records_applied += 1;
                }
                31 => {
                    let (id, val, pid, ms, curve, bypass) =
                        (d.u32()?, d.f32()?, d.u32()?, d.i32()?, d.i32()?, d.bool()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.set_rtpc_value_by_playing_id(id, val, mapped_pid, ms, curve, bypass);
                    records_applied += 1;
                }
                32 => {
                    self.reset_rtpc_value(d.u32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?);
                    records_applied += 1;
                }
                33 => {
                    self.set_switch(d.u32()?, d.u32()?, d.u64()?);
                    records_applied += 1;
                }
                34 => {
                    self.set_state(d.u32()?, d.u32()?);
                    records_applied += 1;
                }
                35 => {
                    self.post_trigger(d.u32()?, d.u64()?);
                    records_applied += 1;
                }
                40 => {
                    let (ev, obj, flags, _cb, cookie, custom_pid, n) = (
                        d.u32()?,
                        d.u64()?,
                        d.u32()?,
                        d.u64()?,
                        d.u64()?,
                        d.u32()?,
                        d.u32()? as usize,
                    );
                    let mut exts = Vec::with_capacity(n);
                    for _ in 0..n {
                        exts.push(d.ext()?);
                    }
                    let assigned = self.post_event(ev, obj, flags, None, cookie, exts, if ret != 0 { ret } else { custom_pid });
                    if ret != 0 && assigned != 0 {
                        pid_map.insert(ret, assigned);
                    }
                    events_posted += 1;
                    records_applied += 1;
                }
                41 => {
                    let (ev, act, obj, ms, curve, pid) = (d.u32()?, d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.execute_action_on_event(ev, act, obj, ms, curve, mapped_pid);
                    records_applied += 1;
                }
                42 => {
                    let name = String::from_utf8_lossy(d.blob()?).into_owned();
                    let (act, obj, ms, curve, pid) = (d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.execute_action_on_event_name(&name, act, obj, ms, curve, mapped_pid);
                    records_applied += 1;
                }
                43 => {
                    let (ev, obj, ms, near, pid) = (d.u32()?, d.u64()?, d.i32()?, d.bool()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.seek_on_event(ev, obj, ms, near, mapped_pid);
                    records_applied += 1;
                }
                44 => {
                    let (ev, obj, pct, near, pid) = (d.u32()?, d.u64()?, d.f32()?, d.bool()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.seek_on_event_percent(ev, obj, pct, near, mapped_pid);
                    records_applied += 1;
                }
                45 => {
                    self.stop_all(d.u64()?);
                    records_applied += 1;
                }
                46 => {
                    let (pid, ms, curve) = (d.u32()?, d.i32()?, d.i32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.stop_playing_id(mapped_pid, ms, curve);
                    records_applied += 1;
                }
                50 | 51 => {
                    let (_ptr, size, id, hash) = (d.u64()?, d.u32()?, d.u32()?, d.u64()?);
                    let fname = format!("bank_{id:08x}_{size}_{hash:016x}.bnk");
                    let bank_path = banks_dir.join(&fname);
                    let bytes = std::fs::read(&bank_path)
                        .map_err(|e| format!("failed to read trace bank {}: {e}", bank_path.display()))?;
                    self.load_bank_bytes(fname, bytes)?;
                    banks_loaded += 1;
                    records_applied += 1;
                }
                52 => {
                    let (id, _ptr, _ty) = (d.u32()?, d.u64()?, d.u32()?);
                    self.unload_bank(id);
                    records_applied += 1;
                }
                _ => {}
            }
        }

        if tail_frames > 0 {
            let tail = self.render_frames(tail_frames)?;
            samples.extend_from_slice(&tail);
        }

        Ok(TraceReplayReport {
            records_total,
            records_applied,
            frames_rendered: self.frames_rendered(),
            banks_loaded,
            events_posted,
            callbacks_fired: self.callback_history().len(),
            samples,
        })
    }
}

// ---------------------------------------------------------------------------
// Unit & Integration Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    /// Build a minimal valid v145 SoundBank with `BKHD` + `DIDX` + `DATA` + `HIRC`
    /// containing a PCM16 `.wem` Sound, a Play Action, and an Event.
    fn build_synthetic_bank(
        bank_id: u32,
        event_id: u32,
        action_id: u32,
        sound_id: u32,
        media_id: u32,
        plugin_id: u32,
        cookie: u32,
    ) -> Vec<u8> {
        let wem = build_pcm16_wem(48000, 1, 2048, 0.5);

        let mut bkhd_body = Vec::new();
        bkhd_body.extend_from_slice(&145u32.to_le_bytes()); // version 145
        bkhd_body.extend_from_slice(&bank_id.to_le_bytes());
        bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // language_id
        bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // alt_values
        bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // project_id
        bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // bank_type
        bkhd_body.extend_from_slice(&[0u8; 16]); // hash

        let mut didx_body = Vec::new();
        if plugin_id != 0x0008_0001 {
            didx_body.extend_from_slice(&media_id.to_le_bytes());
            didx_body.extend_from_slice(&0u32.to_le_bytes()); // offset 0
            didx_body.extend_from_slice(&(wem.len() as u32).to_le_bytes());
        }

        let bus_id = 0x8888u32;

        // HIRC objects:
        // 0. Master Bus (type 0x08)
        let mut bus_body = Vec::new();
        bus_body.extend_from_slice(&bus_id.to_le_bytes());
        bus_body.extend_from_slice(&0u32.to_le_bytes()); // override_bus_id = 0 (master)
        bus_body.extend_from_slice(&0u32.to_le_bytes()); // device_shareset_id
        bus_body.push(0); // props
        bus_body.push(0); // positioning
        bus_body.push(0); // aux_bits
        bus_body.extend_from_slice(&0u32.to_le_bytes()); // reflections_aux_bus
        bus_body.push(0); // bits
        bus_body.extend_from_slice(&0u16.to_le_bytes()); // max_num_instance
        bus_body.extend_from_slice(&0x0000_3102u32.to_le_bytes()); // channel_config (stereo: n=2, kind=1, mask=0x3)
        bus_body.push(0); // hdr_bits
        bus_body.extend_from_slice(&0i32.to_le_bytes()); // recovery_time_ms
        bus_body.extend_from_slice(&0.0f32.to_le_bytes()); // max_duck_volume
        bus_body.extend_from_slice(&0u32.to_le_bytes()); // ducks count = 0
        bus_body.push(0); // fx count = 0
        bus_body.extend_from_slice(&0u32.to_le_bytes()); // mixer_fx_id
        bus_body.push(0); // mixer_is_shareset
        bus_body.push(0); // override_attachment_params
        bus_body.push(0); // metadata count = 0
        bus_body.extend_from_slice(&0u16.to_le_bytes()); // rtpcs count = 0
        bus_body.push(0); // state_props count = 0
        bus_body.push(0); // state_groups count = 0

        // 1. Sound (type 0x02)
        let mut sound_body = Vec::new();
        sound_body.extend_from_slice(&sound_id.to_le_bytes());
        sound_body.extend_from_slice(&plugin_id.to_le_bytes());
        sound_body.push(0); // stream_type = Data (in-bank)
        let src_id = if plugin_id == 0x0008_0001 { cookie } else { media_id };
        sound_body.extend_from_slice(&src_id.to_le_bytes());
        sound_body.extend_from_slice(&(wem.len() as u32).to_le_bytes()); // in_memory_media_size
        sound_body.push(0); // source_bits
        if (plugin_id & 0x0f) == 2 {
            sound_body.extend_from_slice(&0u32.to_le_bytes()); // plugin_param_size = 0
        }
        // NodeBaseParams (v145)
        sound_body.push(0); // fx override_parent
        sound_body.push(0); // num_fx
        sound_body.push(0); // metadata override_parent
        sound_body.push(0); // num_metadata
        sound_body.push(0); // override_attachment_params
        sound_body.extend_from_slice(&bus_id.to_le_bytes()); // override_bus_id
        sound_body.extend_from_slice(&0u32.to_le_bytes()); // direct_parent_id
        sound_body.push(0); // priority_bits
        sound_body.push(0); // num_props
        sound_body.push(0); // num_ranged
        sound_body.push(0); // positioning_bits
        sound_body.push(0); // aux_bits
        sound_body.extend_from_slice(&0u32.to_le_bytes()); // reflections_aux_bus
        sound_body.push(0); // adv_bits
        sound_body.push(0); // virtual_queue_behavior
        sound_body.extend_from_slice(&0u16.to_le_bytes()); // max_num_instance
        sound_body.push(0); // below_threshold_behavior
        sound_body.push(0); // hdr_bits
        sound_body.push(0); // num_state_props
        sound_body.push(0); // num_state_groups
        sound_body.extend_from_slice(&0u16.to_le_bytes()); // num_rtpc

        // 2. Action Play (type 0x03)
        let mut action_body = Vec::new();
        action_body.extend_from_slice(&action_id.to_le_bytes());
        action_body.extend_from_slice(&0x0403u16.to_le_bytes()); // Play action
        action_body.extend_from_slice(&sound_id.to_le_bytes()); // id_ext = sound_id
        action_body.push(0); // is_bus
        action_body.push(0); // num_props
        action_body.push(0); // num_ranged
        action_body.push(4); // fade_curve
        action_body.extend_from_slice(&bank_id.to_le_bytes()); // bank_id
        action_body.extend_from_slice(&0u32.to_le_bytes()); // bank_type

        // 3. Event (type 0x04)
        let mut event_body = Vec::new();
        event_body.extend_from_slice(&event_id.to_le_bytes());
        event_body.push(1); // action count = 1
        event_body.extend_from_slice(&action_id.to_le_bytes());

        let mut hirc_body = Vec::new();
        hirc_body.extend_from_slice(&4u32.to_le_bytes()); // 4 objects
        for (ty, body) in [(8u8, &bus_body), (2u8, &sound_body), (3u8, &action_body), (4u8, &event_body)] {
            hirc_body.push(ty);
            hirc_body.extend_from_slice(&(body.len() as u32).to_le_bytes());
            hirc_body.extend_from_slice(body);
        }

        let mut bank = Vec::new();
        let write_chunk = |out: &mut Vec<u8>, tag: &[u8; 4], body: &[u8]| {
            out.extend_from_slice(tag);
            out.extend_from_slice(&(body.len() as u32).to_le_bytes());
            out.extend_from_slice(body);
        };
        write_chunk(&mut bank, b"BKHD", &bkhd_body);
        if !didx_body.is_empty() {
            write_chunk(&mut bank, b"DIDX", &didx_body);
            write_chunk(&mut bank, b"DATA", &wem);
        }
        write_chunk(&mut bank, b"HIRC", &hirc_body);
        bank
    }

    fn build_pcm16_wem(sample_rate: u32, channels: u16, frames: usize, amp: f32) -> Vec<u8> {
        let block_align = channels * 2;
        let data_len = (frames * block_align as usize) as u32;
        let mut w = Vec::with_capacity(44 + data_len as usize);
        w.extend_from_slice(b"RIFF");
        w.extend_from_slice(&(36 + data_len).to_le_bytes());
        w.extend_from_slice(b"WAVEfmt ");
        w.extend_from_slice(&18u32.to_le_bytes());
        w.extend_from_slice(&1u16.to_le_bytes()); // PCM
        w.extend_from_slice(&channels.to_le_bytes());
        w.extend_from_slice(&sample_rate.to_le_bytes());
        w.extend_from_slice(&(sample_rate * block_align as u32).to_le_bytes());
        w.extend_from_slice(&block_align.to_le_bytes());
        w.extend_from_slice(&16u16.to_le_bytes());
        w.extend_from_slice(&0u16.to_le_bytes());
        w.extend_from_slice(b"data");
        w.extend_from_slice(&data_len.to_le_bytes());
        let s = (amp.clamp(-1.0, 1.0) * 32767.0) as i16;
        for _ in 0..(frames * channels as usize) {
            w.extend_from_slice(&s.to_le_bytes());
        }
        w
    }

    #[test]
    fn bank_load_post_event_and_end_of_event_callback() {
        let engine = NativeSoundEngine::default();

        let bank_bytes = build_synthetic_bank(0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x0001_0001, 0);
        let bank_id = engine.load_bank_bytes("test.bnk", bank_bytes).unwrap();
        assert_eq!(bank_id, 0x1111);

        engine.register_game_obj(100, Some("Emitter"));
        engine.set_default_listeners(&[100]);

        let duration_count = Arc::new(AtomicUsize::new(0));
        let end_count = Arc::new(AtomicUsize::new(0));
        let d_clone = duration_count.clone();
        let e_clone = end_count.clone();

        let pid = engine.post_event_rust(
            0x2222,
            100,
            AK_END_OF_EVENT | AK_DURATION | AK_ENABLE_GET_SOURCE_PLAY_POSITION,
            0xdead_beef,
            Vec::new(),
            move |ev| {
                assert_eq!(ev.cookie, 0xdead_beef);
                assert_eq!(ev.event_id, 0x2222);
                assert_eq!(ev.game_obj_id, 100);
                if ev.callback_type == AK_DURATION {
                    d_clone.fetch_add(1, Ordering::SeqCst);
                } else if ev.callback_type == AK_END_OF_EVENT {
                    e_clone.fetch_add(1, Ordering::SeqCst);
                }
            },
        );
        assert!(pid > 0);

        // 2048 samples = 4 frames of 512 samples.
        let mut total_energy = 0.0f64;
        for i in 0..6 {
            let samples = engine.render_audio(true).unwrap();
            total_energy += samples.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>();
            if i == 0 {
                // During playback, source play position should be queryable.
                assert!(engine.get_source_play_position(pid).is_ok());
            }
        }
        assert!(total_energy > 1.0, "expected non-silent rendered audio, got energy={total_energy}");
        assert_eq!(duration_count.load(Ordering::SeqCst), 1, "AK_Duration callback should fire once");
        assert_eq!(end_count.load(Ordering::SeqCst), 1, "AK_EndOfEvent callback should fire once");

        // Unloading the bank succeeds and removes the event.
        assert!(engine.unload_bank(bank_id));
    }

    #[test]
    fn dialogue_external_source_resolves_and_plays() {
        let engine = NativeSoundEngine::default();

        // External source plugin_id = 0x0008_0001, cookie = 0x1234_5678
        let cookie = 0x1234_5678u32;
        let bank_bytes = build_synthetic_bank(0xaaaa, 0xbbbb, 0xcccc, 0xdddd, 0, 0x0008_0001, cookie);
        engine.load_bank_bytes("dialogue.bnk", bank_bytes).unwrap();

        let ext_wem = build_pcm16_wem(48000, 1, 1024, 0.75);
        let ended = Arc::new(AtomicBool::new(false));
        let ended_clone = ended.clone();

        let pid = engine.post_event_rust(
            0xbbbb,
            1,
            AK_END_OF_EVENT,
            42,
            vec![ExternalSourceDesc {
                cookie,
                codec: 1,
                file_id: 0,
                path: None,
                bytes: Some(ext_wem),
            }],
            move |ev| {
                if ev.callback_type == AK_END_OF_EVENT {
                    ended_clone.store(true, Ordering::SeqCst);
                }
            },
        );
        assert!(pid > 0);

        let samples = engine.render_frames(4).unwrap();
        let peak = samples.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
        assert!(peak > 0.1, "external source audio should play (peak={peak})");
        assert!(ended.load(Ordering::SeqCst), "external source event should fire AK_EndOfEvent");
    }

    #[test]
    fn rtpc_ramping_and_command_queue_ordering() {
        let engine = NativeSoundEngine::default();

        let rtpc_id = 0x9999u32;
        engine.set_rtpc_value(rtpc_id, 0.0, 0, 0, 4, true);
        assert_eq!(engine.get_rtpc_value(rtpc_id, 0, 0), (0.0, 1));

        // 4 frames of 512 samples at 48 kHz = 2048 samples = 42.6667 ms.
        // Use a 40 ms linear ramp (1920 samples) so frames 1, 2, 3, 4 step smoothly to 100.0.
        engine.set_rtpc_value(rtpc_id, 100.0, 0, 40, 4, false);

        engine.render_audio(true).unwrap();
        let (v1, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
        // 512 / 1920 * 100.0 = 26.6667
        assert!((v1 - 26.6667).abs() < 1.0, "after 1 frame of 40ms ramp, expected ~26.7, got {v1}");

        engine.render_audio(true).unwrap();
        let (v2, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
        // 1024 / 1920 * 100.0 = 53.3333
        assert!((v2 - 53.3333).abs() < 1.0, "after 2 frames of 40ms ramp, expected ~53.3, got {v2}");

        engine.render_frames(2).unwrap();
        let (v4, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
        assert!((v4 - 100.0).abs() < 1e-3, "after 4 frames of 40ms ramp, expected 100.0, got {v4}");
    }

    #[test]
    fn trace_replay_on_native_sound_engine() {
        let dir = std::env::temp_dir().join(format!("aw2_native_trace_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let bank_id = 0x11223344u32;
        let event_id = 0x55667788u32;
        let bank_bytes = build_synthetic_bank(bank_id, event_id, 0x3333, 0x4444, 0x5555, 0x0001_0001, 0);
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for &b in &bank_bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        let bank_fname = format!("bank_{bank_id:08x}_{}_{h:016x}.bnk", bank_bytes.len());
        std::fs::write(dir.join(&bank_fname), &bank_bytes).unwrap();

        // Build an AW2AKT01 trace: LoadBankMemoryCopy (50) -> RegisterGameObj (11) -> PostEvent (40) -> RenderAudio (10) x 3
        let mut trace = TRACE_MAGIC.to_vec();
        let push_rec = |buf: &mut Vec<u8>, op: u8, ret: u32, payload: &[u8]| {
            buf.push(op);
            buf.push(0); // thread
            buf.extend_from_slice(&0u32.to_le_bytes()); // frame
            buf.extend_from_slice(&ret.to_le_bytes());
            buf.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            buf.extend_from_slice(payload);
        };

        // Op 50: LoadBankMemoryCopy (ptr u64, size u32, bank_id u32, hash u64)
        let mut p_load = Vec::new();
        p_load.extend_from_slice(&0x1000u64.to_le_bytes());
        p_load.extend_from_slice(&(bank_bytes.len() as u32).to_le_bytes());
        p_load.extend_from_slice(&bank_id.to_le_bytes());
        p_load.extend_from_slice(&h.to_le_bytes());
        push_rec(&mut trace, 50, 1, &p_load);

        // Op 11: RegisterGameObj (obj u64)
        push_rec(&mut trace, 11, 1, &100u64.to_le_bytes());

        // Op 40: PostEvent (ev u32, obj u64, flags u32, cb u64, cookie u64, pid u32, n u32)
        let mut p_post = Vec::new();
        p_post.extend_from_slice(&event_id.to_le_bytes());
        p_post.extend_from_slice(&100u64.to_le_bytes());
        p_post.extend_from_slice(&AK_END_OF_EVENT.to_le_bytes());
        p_post.extend_from_slice(&0u64.to_le_bytes());
        p_post.extend_from_slice(&0u64.to_le_bytes());
        p_post.extend_from_slice(&0u32.to_le_bytes());
        p_post.extend_from_slice(&0u32.to_le_bytes());
        push_rec(&mut trace, 40, 77, &p_post);

        // Op 10: RenderAudio x 3
        for _ in 0..3 {
            push_rec(&mut trace, 10, 1, &[1u8]);
        }

        let engine = NativeSoundEngine::default();
        let report = engine.replay_trace(&trace, &dir, 3).unwrap();
        assert_eq!(report.banks_loaded, 1);
        assert_eq!(report.events_posted, 1);
        assert_eq!(report.frames_rendered, 6);
        assert_eq!(report.callbacks_fired, 1);
        let peak = report.samples.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
        assert!(peak > 0.1, "replayed trace should produce non-silent audio (peak={peak})");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cancel_callbacks_queries_and_clear_banks() {
        let engine = NativeSoundEngine::default();
        assert_eq!(engine.current_language(), "en(US)");
        assert_eq!(engine.set_current_language("fr(FR)"), AK_SUCCESS);
        assert_eq!(engine.current_language(), "fr(FR)");

        let bank_bytes = build_synthetic_bank(0x7001, 0x7002, 0x7003, 0x7004, 0x7005, 0x0001_0001, 0);
        engine.load_bank_bytes("test_cancel.bnk", bank_bytes).unwrap();
        engine.register_game_obj(500, Some("CancelEmitter"));
        engine.set_default_listeners(&[500]);
        engine.set_state(0x100, 0x200);
        engine.set_switch(0x300, 0x400, 500);
        assert_eq!(engine.get_state(0x100), 0x200);
        assert_eq!(engine.get_switch(0x300, 500), 0x400);
        assert_eq!(engine.active_game_objects(), vec![500]);

        let cb_count = Arc::new(AtomicUsize::new(0));
        let cb1 = cb_count.clone();
        let pid1 = engine.post_event_rust(
            0x7002,
            500,
            AK_END_OF_EVENT | AK_DURATION,
            0xaaaa,
            Vec::new(),
            move |_| {
                cb1.fetch_add(1, Ordering::SeqCst);
            },
        );
        // Cancel before command queue is drained
        engine.cancel_event_callback(pid1);

        let cb2 = cb_count.clone();
        let _pid2 = engine.post_event_rust(
            0x7002,
            500,
            AK_END_OF_EVENT | AK_DURATION,
            0xbbbb,
            Vec::new(),
            move |_| {
                cb2.fetch_add(1, Ordering::SeqCst);
            },
        );
        engine.cancel_event_callback_cookie(0xbbbb);

        engine.render_frames(6).unwrap();
        assert_eq!(cb_count.load(Ordering::SeqCst), 0, "cancelled callbacks must not fire");
        assert_eq!(engine.clear_banks(), AK_SUCCESS);
    }
}

