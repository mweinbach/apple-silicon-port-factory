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

mod api;
mod callbacks;
mod commands;
#[cfg(test)]
mod tests;
mod trace;

pub use callbacks::{
    AkDurationCallbackInfo, AkEventCallbackInfo, AkMusicSyncCallbackInfo, AkSegmentInfo, CallbackEvent,
    CallbackPayload, EventCallbackTarget, GlobalCallbackTarget, Win64EventCallback, Win64GlobalCallback,
};
pub use commands::parse_ak_world_transform;
pub use trace::TraceReplayReport;

use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::engine::math::{FRAME, RATE};
use crate::engine::params::GameState;
use crate::engine::project::Project;
use crate::engine::render::{RenderOptions, RendererState};
use crate::fnv1_32_lower;
use callbacks::{
    collect_duration_callbacks, collect_end_of_event_callbacks, collect_music_sync_callbacks, GlobalCallbackEntry,
    PlayingTracker,
};
use commands::{drain_commands, step_rtpc_ramps, Command, RtpcRamp, RtpcTarget};

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
// Audio Sink Trait
// ---------------------------------------------------------------------------

/// Consumer of interleaved float32 output frames produced by [`NativeSoundEngine::render_audio`].
pub trait AudioSink: Send + Sync {
    fn push_samples(&self, samples: &[f32], channels: usize, sample_rate: u32);
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
    #[allow(dead_code)]
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

    pub fn unregister_global_callback_win64(&self, cb: Win64GlobalCallback, location_mask: u32) -> i32 {
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
