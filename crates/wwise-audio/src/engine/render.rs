//! Offline renderer: event/action engine, containers, voices and the bus mix (A3 subset).
//!
//! Model. A posted event's actions are scheduled on a sample timeline. Playing a node expands it
//! immediately into *scheduled voices* (start sample, parameters, envelopes); continuous
//! containers pre-compute item boundaries from the exact resampled source lengths. Stop / Pause /
//! Resume / Seek act on the voices (scheduled or running) whose actor-mixer chain contains the
//! action target. Audio is produced in AK frames of [`FRAME`] samples: per voice decode ->
//! [`Resampler`] -> 2D matrix x gain via the AK mixer ramp -> output bus buffer; buses are mixed
//! child-to-parent with their bus volume; the master bus is the output.
//!
//! Every behaviour that is not decompiled is listed in [`RendererState::notes`] with an `[A]` tag
//! in the code; the per-event notes let callers pick events whose whole path is modelled.

mod events;
mod mix;
mod music;
#[cfg(test)]
mod tests;
mod voice;

use super::filter::ConnectionFilter;
use super::fx::FxInstance;
use super::math::{db_to_lin, transition_ticks, transition_value, FRAME, RATE};
use super::params::{GameState, Rng, VoiceParams};
use super::project::Project;
use super::ranseq::{RandomInfo, SeqInfo};
use super::resample::Resampler;
use super::speakers::Layout;
use crate::bnk::hirc::HircBody;
use std::collections::{BTreeSet, HashMap};
use std::ops::{Deref, DerefMut};
use std::sync::Arc as Rc;
use voice::{ms_to_samples, pause_voice, resume_voice, seek_voice, stop_voice};

/// External source descriptor (`AkExternalSourceInfo`) supplied to `PostEvent`.
#[derive(Debug, Clone, Default)]
pub struct ExternalSourceDesc {
    pub cookie: u32,
    pub codec: u32,
    pub file_id: u32,
    pub path: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

/// Linear gain of a voice's dry mix connection:
/// `FastPow10(volume_db) * FastPow10(makeup_db) * FastPow10(output_bus_volume_db)`, three
/// separate `FastPow10` stages matching Wwise 2022.1 [V: `FUN_141600880` (`Volume`),
/// `FUN_141603110` (`MakeUpGain`), `FUN_141645fc0` (`OutputBusVolume`, including folded
/// `BusVolume`/`OutputBusVolume` from leading pass-through buses via `sub_1415ca360`)].
pub fn voice_gain(volume_db: f32, makeup_db: f32, output_bus_volume_db: f32) -> f32 {
    db_to_lin(volume_db) * db_to_lin(makeup_db) * db_to_lin(output_bus_volume_db)
}

/// Linear gain of a bus -> parent mix [V: `FUN_1415ee330`, `FUN_1415f2db0`, `FUN_141645fc0`]:
/// - Pass-through bus (`!mix_node`): `1.0` (0 stages; its `BusVolume`/`OutputBusVolume` were
///   folded into the child voice or child mixing bus per `sub_1415ca360`/`sub_1415c9610`).
/// - Master / top-level bus (`mix_node && is_master`): `FastPow10(bus_volume_db)` (1 stage;
///   no parent connection in `FUN_1415ee330` so `FUN_141645fc0` is skipped).
/// - Non-master mixing bus (`mix_node && !is_master`):
///   `FastPow10(bus_volume_db) * FastPow10(output_bus_volume_db)` (2 stages).
pub fn bus_gain(bus_volume_db: f32, output_bus_volume_db: f32, mix_node: bool, is_master: bool) -> f32 {
    if !mix_node {
        1.0
    } else if is_master {
        db_to_lin(bus_volume_db)
    } else {
        db_to_lin(bus_volume_db) * db_to_lin(output_bus_volume_db)
    }
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// FMA lerp in the resampler's fixed-step routine. AK fills its resampler dispatch table
    /// (`0x143931f90`) from the CPUID AVX2 bit at init, independently of the mixer selector
    /// (`0x143931f80`); under Rosetta AVX2 is advertised, so the oracle runs the AVX2/FMA variant.
    pub resampler_fma: bool,
    /// FMA in the mixer ramp kernel (the oracle forces the SSE kernel `0x141658fc0`: no FMA).
    pub mixer_fma: bool,
    /// Apply bus effects (`Bus::fx` / `AuxBus::fx`).
    pub bus_fx: bool,
    /// `SetRandomSeed` value (the A2 oracle reseeds with 0x5eed before every event).
    pub seed: u32,
    /// Hard cap on rendered length (infinite loops, long sequences).
    pub max_seconds: f32,
    /// Gain on a voice's first frame ramps from 0 instead of starting at the target [A].
    pub start_ramp: bool,
    /// Ramp the voice's gain to 0 over the frame in which its media ends. Off by default: with
    /// the device Mastering Suite bypassed the oracle shows no ramp (docs/AUDIO.md "A3 status";
    /// the A2 "natural end ramp" was the Mastering Suite's response to the step).
    pub end_ramp: bool,
    /// Output channel count (2 = stereo endpoint as in the A1 harness).
    pub out_channels: usize,
    /// Extra silent frames rendered after the last voice ends.
    pub tail_frames: usize,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            resampler_fma: true,
            mixer_fma: false,
            bus_fx: false,
            seed: 0x5eed,
            max_seconds: 60.0,
            start_ramp: false,
            end_ramp: false,
            out_channels: 2,
            tail_frames: 0,
        }
    }
}

/// Decoded source media, planar.
pub struct Pcm {
    pub rate: u32,
    pub layout: Layout,
    pub data: Vec<Vec<f32>>,
    /// `smpl` loop (start, end exclusive) in frames.
    pub loop_points: Option<(u32, u32)>,
    /// Linear gain from the media's `akd ` chunk (1.0 without one).
    pub akd_gain: f32,
}

impl Pcm {
    pub fn frames(&self) -> usize {
        self.data.first().map_or(0, |c| c.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VState {
    Scheduled,
    Playing,
    Virtual,
    Paused,
    Done,
}

#[derive(Debug, Clone, Copy)]
struct Fade {
    start: u64,
    /// nominal length in samples (ms * 48)
    len: u64,
    curve: u32,
}

impl Fade {
    /// Duration in buffer ticks (frames), `ceil(ms / 10)` [V: `FUN_1415ada30`].
    fn ticks(&self) -> u64 {
        transition_ticks(self.len)
    }

    /// Ticks elapsed when the frame ending at `f1` renders: the transition starts in the frame
    /// containing `start` and that frame already sees one tick of progress [V: fade-in of event
    /// 3586369275 fits `interp((k+1)/N)` per frame k to -141 dB].
    fn elapsed(&self, f1: u64) -> u64 {
        (f1 / FRAME as u64).saturating_sub(self.start / FRAME as u64)
    }

    /// Value of the `from -> to` transition for the frame ending at `f1`. Before its first
    /// update (elapsed 0) the target still holds `from`; AK never evaluates the curve at t = 0
    /// here (SCurve's `interp(0)` is 0.000697) [V: event 3644236398 frame 0 ramps from 0].
    fn value(&self, from: f32, to: f32, f1: u64) -> f32 {
        match self.elapsed(f1) {
            0 => from,
            e => transition_value(from, to, self.curve, e, self.ticks()),
        }
    }

    /// First sample after the frame in which the transition reaches its target (at least the
    /// action's own frame: a 0 ms transition completes there).
    fn end(&self) -> u64 {
        (self.start / FRAME as u64 + self.ticks().max(1)) * FRAME as u64
    }
}

struct Voice {
    sound_id: u32,
    playing_id: u32,
    event_id: u32,
    game_object_id: u64,
    /// Sound + DirectParentID chain (for action targeting).
    chain: Vec<u32>,
    params: VoiceParams,
    pcm: Rc<Pcm>,
    start: u64,
    state: VState,
    evicted_by_limit: bool,
    /// Input stream position: segment index and offset (see `segments`).
    segments: Vec<(usize, usize)>,
    seg: usize,
    seg_pos: usize,
    res: Resampler,
    base_lin: f32,
    ray_user_aux: f32,
    ray_game_aux: f32,
    ray_lpf: f32,
    ray_hpf: f32,
    prev_gain: Option<f32>,
    aux_prev_gain: HashMap<u32, f32>,
    aux_filters: HashMap<u32, ConnectionFilter>,
    node_fx: Vec<(u8, FxInstance)>,
    fade_in: Option<Fade>,
    /// Active fade-out and the starting amplitude captured when the fade-out was scheduled
    /// (equals the in-progress fade-in ratio if a fade-in was still running at handoff).
    fade_out: Option<(Fade, f32)>,
    /// Hard stop sample (after a fade-out / stop request).
    stop_at: Option<u64>,
    /// Expected end (output samples, absolute) or None for infinite.
    pub end: Option<u64>,
    matrix: Vec<Vec<f32>>,
    filter: Option<ConnectionFilter>,
    /// Active pause/resume fade: `(fade, captured_start_ratio, is_pause)`.
    pause_fade: Option<(Fade, f32, bool)>,
    /// For MusicTrack clips: segment-relative sample where clip timeline `t = 0` starts, and
    /// `ClipAutomation` envelopes (`Volume`, `LPF`, `HPF`, `FadeIn`, `FadeOut`).
    clip_origin_sample: u64,
    clip_automations: Vec<crate::bnk::hirc::ClipAutomation>,
}

#[derive(Debug, Clone)]
struct DuckState {
    active: bool,
    current_db: f32,
    from_db: f32,
    to_db: f32,
    trans_start: u64,
    trans_len: u64,
    curve: u32,
}

/// Pending action instance.
#[derive(Debug, Clone)]
struct Pending {
    at: u64,
    action: u32,
    playing_id: u32,
    event_id: u32,
    game_object_id: u64,
    ext_sources: Vec<ExternalSourceDesc>,
}

#[derive(Default)]
struct CntrState {
    random: Option<RandomInfo>,
    seq: SeqInfo,
}

#[derive(Debug, Clone)]
struct ActiveSwitchCntr {
    cntr_id: u32,
    group_type: u8,
    group_id: u32,
    active_switch: u32,
    active_children: Vec<u32>,
    depth: usize,
    playing_id: u32,
}

#[derive(Debug, Clone)]
struct ActiveMusicSwitchCntr {
    cntr_id: u32,
    active_child: u32,
    depth: usize,
    playing_id: u32,
}

#[derive(Debug, Clone)]
pub struct ActiveMusicSegment {
    /// Hierarchy node IDs owning this segment (e.g. `[seg_id, ranseq_id, switch_cntr_id]`).
    pub owner_chain: Vec<u32>,
    pub segment_id: u32,
    pub playing_id: u32,
    pub event_id: u32,
    pub game_object_id: u64,
    /// Sample where segment timeline `t = 0` (pre-entry start) is anchored.
    pub origin_sample: u64,
    pub entry_sample: u64,
    pub exit_sample: u64,
    pub end_sample: u64,
}

#[derive(Debug, Clone)]
struct BusNode {
    id: u32,
    parent: u32,
    depth: usize,
    layout: Layout,
    /// Matrix [this bus channel][parent channel] (direct assignment / standard downmix).
    to_parent: Vec<Vec<f32>>,
    gain_db: f32,
    /// Connection OutputBusVolume dB (plus folded `BusVolume`/`OutputBusVolume` of any
    /// immediately following pass-through ancestor buses) for non-master mixing buses.
    out_vol_db: f32,
    mix_node: bool,
    /// OutputBusLPF / OutputBusHPF of this bus (its connection into the parent). Zero on
    /// pass-through buses whose filter was already folded into their child.
    out_lpf: f32,
    out_hpf: f32,
    fx: Vec<(u8, u32, u32)>,
    ducks: Vec<crate::bnk::hirc::DuckInfo>,
    recovery_time_ms: i32,
    max_duck_volume: f32,
}

/// Owned renderer state without a borrowed `&Project` reference, allowing [`Project`] and
/// [`RendererState`] to live together in [`crate::runtime::NativeSoundEngine`] while banks are
/// loaded and unloaded dynamically.
pub struct RendererState {
    pub gs: GameState,
    pub opts: RenderOptions,
    rng: Rng,
    now: u64,
    pending: Vec<Pending>,
    voices: Vec<Voice>,
    pcm_cache: HashMap<u32, Rc<Pcm>>,
    cntr: HashMap<u32, CntrState>,
    switch_played_once: BTreeSet<(u32, u32)>,
    active_switch_cntrs: Vec<ActiveSwitchCntr>,
    music_track_seq: HashMap<u32, u32>,
    music_playlists: HashMap<u32, super::music::PlaylistRunnerState>,
    active_music_switches: Vec<ActiveMusicSwitchCntr>,
    active_music_segments: Vec<ActiveMusicSegment>,
    /// Last sample at which `(owner_node_id, trigger_id)` fired a stinger (for `dont_repeat_time`).
    stinger_last_fired: HashMap<(u32, u32), u64>,
    /// Features met during this render that are not modelled exactly (see module docs).
    pub notes: BTreeSet<String>,
    pub log: Vec<String>,
    bus_order: Vec<BusNode>,
    bus_voices_seen: usize,
    bus_prev_gain: HashMap<u32, f32>,
    /// Filter state of each filtered bus -> parent connection.
    bus_filters: HashMap<u32, ConnectionFilter>,
    /// Stateful bus FX instances keyed by `(bus_id, slot_index)`.
    bus_fx_instances: HashMap<(u32, u8), FxInstance>,
    /// Auto-ducking state per `(source_bus_id, target_bus_id)`.
    duck_states: HashMap<(u32, u32), DuckState>,
    cur_playing_id: u32,
    cur_event_id: u32,
    cur_game_obj: u64,
    cur_ext_sources: Vec<ExternalSourceDesc>,
}

impl Default for RendererState {
    fn default() -> Self {
        Self::new(GameState::default(), RenderOptions::default())
    }
}

impl RendererState {
    pub fn new(gs: GameState, opts: RenderOptions) -> Self {
        let rng = Rng::new(opts.seed);
        let cur_game_obj = gs.game_object_id;
        RendererState {
            gs,
            opts,
            rng,
            now: 0,
            pending: Vec::new(),
            voices: Vec::new(),
            pcm_cache: HashMap::new(),
            cntr: HashMap::new(),
            switch_played_once: BTreeSet::new(),
            active_switch_cntrs: Vec::new(),
            music_track_seq: HashMap::new(),
            music_playlists: HashMap::new(),
            active_music_switches: Vec::new(),
            active_music_segments: Vec::new(),
            stinger_last_fired: HashMap::new(),
            notes: BTreeSet::new(),
            log: Vec::new(),
            bus_order: Vec::new(),
            bus_voices_seen: usize::MAX,
            bus_prev_gain: HashMap::new(),
            bus_filters: HashMap::new(),
            bus_fx_instances: HashMap::new(),
            duck_states: HashMap::new(),
            cur_playing_id: 0,
            cur_event_id: 0,
            cur_game_obj,
            cur_ext_sources: Vec::new(),
        }
    }

    /// Borrow `project` and execute `f` against a temporary [`Renderer`] view over `self`.
    pub fn with_project<'p, R>(&mut self, project: &'p Project, f: impl FnOnce(&mut Renderer<'p>) -> R) -> R {
        let mut r = Renderer {
            project,
            state: std::mem::take(self),
        };
        let out = f(&mut r);
        *self = r.state;
        out
    }
}

pub struct Renderer<'p> {
    pub project: &'p Project,
    pub state: RendererState,
}

impl Deref for Renderer<'_> {
    type Target = RendererState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Renderer<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<'p> Renderer<'p> {
    pub fn new(project: &'p Project, gs: GameState, opts: RenderOptions) -> Self {
        Renderer {
            project,
            state: RendererState::new(gs, opts),
        }
    }

    /// Pre-populate decoded PCM for `source_id` (used by unit tests with synthetic waveforms).
    pub fn insert_pcm(&mut self, source_id: u32, pcm: Pcm) {
        self.state.pcm_cache.insert(source_id, Rc::new(pcm));
    }

    /// Start over for another post on the same nodes: voices, buses, time and notes reset, RNG
    /// reseeded (`SetRandomSeed(opts.seed)`), RanSeq container state kept. Models a node that
    /// survives between posts (the A2 oracle's warm-up post when the bank reload does not free
    /// the node, or a container shared by two banks of one oracle group).
    pub fn next_event(&mut self) {
        let cntr = std::mem::take(&mut self.state.cntr);
        let music_track_seq = std::mem::take(&mut self.state.music_track_seq);
        let music_playlists = std::mem::take(&mut self.state.music_playlists);
        self.state = RendererState::new(self.state.gs.clone(), self.state.opts.clone());
        self.state.cntr = cntr;
        self.state.music_track_seq = music_track_seq;
        self.state.music_playlists = music_playlists;
    }

    pub(super) fn note(&mut self, s: impl Into<String>) {
        self.state.notes.insert(s.into());
    }

    pub(super) fn horizon(&self) -> u64 {
        (self.state.opts.max_seconds as f64 * RATE as f64) as u64
    }

    /// Update an RTPC value (global when `game_object_id.is_none()`, else per-game-object) and
    /// refresh active voices.
    pub fn set_rtpc_value(&mut self, rtpc_id: u32, value: f32, game_object_id: Option<u64>) {
        match game_object_id {
            Some(obj) => self.state.gs.set_rtpc_for_object(obj, rtpc_id, value),
            None => self.state.gs.set_rtpc(rtpc_id, value),
        }
        self.refresh_voices_rtpc();
    }

    /// Set a global State (`AK::SoundEngine::SetState`) at the current render sample, refreshing
    /// StateChunk overrides on active voices/buses and re-validating continuous state-driven
    /// `SwitchCntr` and `MusicSwitchCntr` containers.
    pub fn set_state(&mut self, group_id: u32, state_id: u32) -> Result<(), String> {
        self.set_state_at(group_id, state_id, self.state.now)
    }

    /// Set a Switch (`AK::SoundEngine::SetSwitch`) at the current render sample (global when
    /// `game_object_id.is_none()`, else per-game-object) and re-validate continuous switch-driven
    /// `SwitchCntr` and `MusicSwitchCntr` containers.
    pub fn set_switch(&mut self, group_id: u32, switch_id: u32, game_object_id: Option<u64>) -> Result<(), String> {
        self.set_switch_at(group_id, switch_id, game_object_id, self.state.now)
    }

    /// Post a Wwise Music Trigger (`AK::SoundEngine::PostTrigger`) at the current render sample,
    /// scheduling any matching `Stinger` segments on active music hierarchies at their sync point.
    pub fn post_trigger(&mut self, trigger_id: u32) -> Result<(), String> {
        self.post_trigger_at(trigger_id, self.state.now)
    }

    /// Sample at which the last voice ends (for length checks); None if any voice is infinite.
    pub fn planned_end(&self) -> Option<u64> {
        let mut e = 0;
        for v in &self.state.voices {
            e = e.max(v.end?);
        }
        Some(e)
    }

    pub fn voice_count(&self) -> usize {
        self.state.voices.len()
    }

    /// Current rendered sample position (`now`).
    pub fn now(&self) -> u64 {
        self.state.now
    }

    /// Whether any voices or pending actions are active.
    pub fn is_active(&self) -> bool {
        self.active()
    }

    /// Reseed the internal Wwise PRNG (`AK::SoundEngine::SetRandomSeed`).
    pub fn set_random_seed(&mut self, seed: u32) {
        self.state.opts.seed = seed;
        self.state.rng = Rng::new(seed);
    }

    /// Mark bus order dirty after bank loads/unloads or bus changes.
    pub fn invalidate_bus_order(&mut self) {
        self.state.bus_voices_seen = usize::MAX;
    }

    /// Whether a specific `playing_id` still has active voices, pending actions, or active music segments.
    pub fn is_playing_id_active(&self, playing_id: u32) -> bool {
        if playing_id == 0 {
            return false;
        }
        if self
            .state
            .pending
            .iter()
            .any(|p| p.playing_id == playing_id && self.pending_keeps_alive(p))
        {
            return true;
        }
        if self
            .state
            .voices
            .iter()
            .any(|v| v.playing_id == playing_id && v.state != VState::Done)
        {
            return true;
        }
        if self
            .state
            .active_music_segments
            .iter()
            .any(|s| s.playing_id == playing_id && self.state.now < s.end_sample)
        {
            return true;
        }
        false
    }

    /// Stop all voices and pending actions for `playing_id` (`AK::SoundEngine::StopPlayingID`).
    pub fn stop_playing_id(&mut self, playing_id: u32, transition_ms: i32, curve: u32) {
        if playing_id == 0 {
            return;
        }
        let at = self.state.now;
        let len = ms_to_samples(transition_ms.max(0) as f32);
        self.state.pending.retain(|p| p.playing_id != playing_id);
        for v in self.state.voices.iter_mut() {
            if v.state != VState::Done && v.playing_id == playing_id {
                stop_voice(v, at, len, curve);
            }
        }
        if len == 0 {
            self.state.active_music_segments.retain(|s| s.playing_id != playing_id);
            self.state.active_music_switches.retain(|s| s.playing_id != playing_id);
            self.state.active_switch_cntrs.retain(|s| s.playing_id != playing_id);
        }
    }

    /// Stop all voices and pending actions globally (`game_object_id.is_none()`) or for a single
    /// game object (`AK::SoundEngine::StopAll`).
    pub fn stop_all(&mut self, game_object_id: Option<u64>) {
        let match_obj = |obj: u64| match game_object_id {
            None | Some(u64::MAX) => true,
            Some(target) => obj == target,
        };
        self.state.pending.retain(|p| !match_obj(p.game_object_id));
        for v in self.state.voices.iter_mut() {
            if v.state != VState::Done && match_obj(v.game_object_id) {
                v.state = VState::Done;
            }
        }
        self.state
            .active_music_segments
            .retain(|s| !match_obj(s.game_object_id));
        if game_object_id.is_none() || game_object_id == Some(u64::MAX) {
            self.state.active_music_switches.clear();
            self.state.active_switch_cntrs.clear();
        }
    }

    /// Execute an action on all active voices spawned by `event_id` (`AK::SoundEngine::ExecuteActionOnEvent`).
    /// `action_type`: `0 = Stop`, `1 = Pause`, `2 = Resume`, `3 = Break`, `4 = ReleaseEnvelope`.
    pub fn execute_action_on_event(
        &mut self,
        event_id: u32,
        action_type: i32,
        game_object_id: Option<u64>,
        transition_ms: i32,
        curve: u32,
        playing_id: u32,
    ) {
        let at = self.state.now;
        let len = ms_to_samples(transition_ms.max(0) as f32);
        let match_obj = |obj: u64| match game_object_id {
            None | Some(u64::MAX) | Some(0) => true,
            Some(target) => obj == target || obj == 0,
        };
        let match_pid = |pid: u32| playing_id == 0 || pid == playing_id;
        if action_type == 0 {
            self.state
                .pending
                .retain(|p| !(p.event_id == event_id && match_obj(p.game_object_id) && match_pid(p.playing_id)));
        }
        for v in self.state.voices.iter_mut() {
            if v.state == VState::Done {
                continue;
            }
            let event_matches = v.event_id == event_id
                || self
                    .project
                    .event(event_id)
                    .and_then(|o| match &o.body {
                        HircBody::Event(ev) => Some(ev.actions.iter().any(|&aid| {
                            self.project.action(aid).is_some_and(|ao| match &ao.body {
                                HircBody::Action(act) => v.chain.contains(&act.target_id),
                                _ => false,
                            })
                        })),
                        _ => None,
                    })
                    .unwrap_or(false);
            if !event_matches || !match_obj(v.game_object_id) || !match_pid(v.playing_id) {
                continue;
            }
            match action_type {
                0 | 3 | 4 => stop_voice(v, at, len, curve),
                1 => {
                    if v.state == VState::Playing {
                        pause_voice(v, at, len, curve);
                    }
                }
                2 => {
                    resume_voice(v, at, len, curve);
                }
                _ => {}
            }
        }
    }

    /// Seek active voices belonging to `event_id` (`AK::SoundEngine::SeekOnEvent`).
    pub fn seek_on_event(
        &mut self,
        event_id: u32,
        game_object_id: Option<u64>,
        value: f32,
        relative_to_duration: bool,
        playing_id: u32,
    ) {
        let match_obj = |obj: u64| match game_object_id {
            None | Some(u64::MAX) | Some(0) => true,
            Some(target) => obj == target || obj == 0,
        };
        let match_pid = |pid: u32| playing_id == 0 || pid == playing_id;
        for v in self.state.voices.iter_mut() {
            if v.state == VState::Done
                || v.event_id != event_id
                || !match_obj(v.game_object_id)
                || !match_pid(v.playing_id)
            {
                continue;
            }
            let n = v.pcm.frames() as f32;
            let pos = if relative_to_duration {
                value.clamp(0.0, 1.0) * n
            } else {
                value * v.pcm.rate as f32 / 1000.0
            };
            seek_voice(v, pos.max(0.0) as usize);
        }
    }

    /// Current source playback position in milliseconds for `playing_id` (`AK::SoundEngine::GetSourcePlayPosition`).
    pub fn source_play_position_ms(&self, playing_id: u32) -> Option<i32> {
        if playing_id == 0 {
            return None;
        }
        for v in &self.state.voices {
            if v.playing_id == playing_id && v.state != VState::Done {
                let sample_in_seg = v
                    .segments
                    .get(v.seg)
                    .map_or(0, |(a, _)| a.saturating_add(v.seg_pos));
                let ms = (sample_in_seg as f64 * 1000.0 / v.pcm.rate.max(1) as f64).round() as i32;
                return Some(ms);
            }
        }
        None
    }

    /// Duration info `(duration_ms, audio_node_id, media_id)` for the first voice of `playing_id`
    /// (`AK_Duration` callback).
    pub fn playing_id_duration_info(&self, playing_id: u32) -> Option<(f32, u32, u32)> {
        if playing_id == 0 {
            return None;
        }
        let v = self.state.voices.iter().find(|v| v.playing_id == playing_id)?;
        let dur_samples = v
            .end
            .map(|e| e.saturating_sub(v.start))
            .unwrap_or_else(|| v.pcm.frames() as u64);
        let dur_ms = (dur_samples as f64 * 1000.0 / RATE as f64) as f32;
        let media_id = self
            .project
            .node(v.sound_id)
            .and_then(|o| match &o.body {
                HircBody::Sound(s) => Some(s.source.source_id),
                _ => None,
            })
            .unwrap_or(0);
        Some((dur_ms, v.sound_id, media_id))
    }

    /// Active music segments belonging to `playing_id`.
    pub fn active_music_segments_for_playing_id(&self, playing_id: u32) -> Vec<ActiveMusicSegment> {
        self.state
            .active_music_segments
            .iter()
            .filter(|s| s.playing_id == playing_id)
            .cloned()
            .collect()
    }

    /// Active playing IDs on `game_object_id` (`AK::SoundEngine::Query::GetPlayingIDsFromGameObject`).
    pub fn playing_ids_for_game_object(&self, game_object_id: u64) -> Vec<u32> {
        let mut pids = BTreeSet::new();
        for p in &self.state.pending {
            if p.game_object_id == game_object_id && p.playing_id != 0 && self.pending_keeps_alive(p) {
                pids.insert(p.playing_id);
            }
        }
        for v in &self.state.voices {
            if v.game_object_id == game_object_id && v.playing_id != 0 && v.state != VState::Done {
                pids.insert(v.playing_id);
            }
        }
        for s in &self.state.active_music_segments {
            if s.game_object_id == game_object_id && s.playing_id != 0 && self.state.now < s.end_sample {
                pids.insert(s.playing_id);
            }
        }
        pids.into_iter().collect()
    }
}
