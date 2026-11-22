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
//! Every behaviour that is not decompiled is listed in [`Renderer::notes`] with an `[A]` tag in
//! the code; the per-event notes let callers pick events whose whole path is modelled.

use super::filter::ConnectionFilter;
use super::fx::FxInstance;
use super::math::{db_to_lin, transition_ticks, transition_value, FRAME, RATE};
use super::mixer::mix_channel;
use super::params::{node_initial_delay, resolve, GameState, Rng, VoiceParams};
use super::project::Project;
use super::ranseq::{single_item_next, LoopInfo, RandomInfo, SeqInfo};
use super::resample::{Mode, Resampler};
use super::rng::INV_2_31_F32;
use super::spatial::{
    atten::{ray_gains, ray_geometry, slot as atten_slot, AttenModel, ConeOverride, ProjectCurves},
    positioning_matrix, Cfg, PanParams, Ray3d, SpeakerSetup,
};
use super::speakers::{direct_matrix, Layout};
use crate::bnk::hirc::{prop, ActionKind, ActionParams, HircBody};
use crate::codec::decode_wem;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc as Rc;

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

pub struct Renderer<'p> {
    pub project: &'p Project,
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

impl<'p> Renderer<'p> {
    pub fn new(project: &'p Project, gs: GameState, opts: RenderOptions) -> Self {
        let rng = Rng::new(opts.seed);
        let cur_game_obj = gs.game_object_id;
        Renderer {
            project,
            gs,
            opts,
            rng,
            now: 0,
            pending: vec![],
            voices: vec![],
            pcm_cache: HashMap::new(),
            cntr: HashMap::new(),
            switch_played_once: BTreeSet::new(),
            active_switch_cntrs: vec![],
            music_track_seq: HashMap::new(),
            music_playlists: HashMap::new(),
            active_music_switches: vec![],
            active_music_segments: vec![],
            stinger_last_fired: HashMap::new(),
            notes: BTreeSet::new(),
            log: vec![],
            bus_order: vec![],
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

    /// Pre-populate decoded PCM for `source_id` (used by unit tests with synthetic waveforms).
    pub fn insert_pcm(&mut self, source_id: u32, pcm: Pcm) {
        self.pcm_cache.insert(source_id, Rc::new(pcm));
    }

    /// Start over for another post on the same nodes: voices, buses, time and notes reset, RNG
    /// reseeded (`SetRandomSeed(opts.seed)`), RanSeq container state kept. Models a node that
    /// survives between posts (the A2 oracle's warm-up post when the bank reload does not free
    /// the node, or a container shared by two banks of one oracle group).
    pub fn next_event(&mut self) {
        let cntr = std::mem::take(&mut self.cntr);
        let music_track_seq = std::mem::take(&mut self.music_track_seq);
        let music_playlists = std::mem::take(&mut self.music_playlists);
        *self = Renderer::new(self.project, self.gs.clone(), self.opts.clone());
        self.cntr = cntr;
        self.music_track_seq = music_track_seq;
        self.music_playlists = music_playlists;
    }

    fn note(&mut self, s: impl Into<String>) {
        self.notes.insert(s.into());
    }

    fn horizon(&self) -> u64 {
        (self.opts.max_seconds as f64 * RATE as f64) as u64
    }

    /// Post an event at absolute sample `at` (actions run in bank order).
    pub fn post_event(&mut self, event_id: u32, at: u64) -> Result<(), String> {
        let pid = self.cur_playing_id;
        let obj = self.gs.game_object_id;
        let ext = self.cur_ext_sources.clone();
        self.post_event_ex(event_id, at, pid, obj, ext)
    }

    /// Post an event with explicit `playing_id`, `game_object_id`, and `ext_sources` at sample `at`.
    pub fn post_event_ex(
        &mut self,
        event_id: u32,
        at: u64,
        playing_id: u32,
        game_object_id: u64,
        ext_sources: Vec<ExternalSourceDesc>,
    ) -> Result<(), String> {
        let o = self.project.event(event_id).ok_or_else(|| format!("event {} not loaded", event_id))?;
        let HircBody::Event(ev) = &o.body else { return Err(format!("object {} is not an event", event_id)) };
        for &a in &ev.actions {
            let Some(ao) = self.project.action(a) else {
                self.note(format!("missing_action_{}", a));
                continue;
            };
            let HircBody::Action(act) = &ao.body else { continue };
            let delay_ms = act.props.get_raw(prop::DELAY_TIME).unwrap_or(0) as i32;
            if act.ranged.0.iter().any(|(id, lo, hi)| *id == prop::DELAY_TIME && (*lo != 0 || *hi != 0)) {
                self.note("random_action_delay");
            }
            let at = at + ms_to_samples(delay_ms.max(0) as f32);
            self.pending.push(Pending {
                at,
                action: a,
                playing_id,
                event_id,
                game_object_id,
                ext_sources: ext_sources.clone(),
            });
        }
        self.pending.sort_by_key(|p| p.at);
        Ok(())
    }

    fn run_action(&mut self, p: &Pending) -> Result<(), String> {
        let prev_pid = self.cur_playing_id;
        let prev_ev = self.cur_event_id;
        let prev_obj = self.cur_game_obj;
        let prev_ext = std::mem::replace(&mut self.cur_ext_sources, p.ext_sources.clone());
        self.cur_playing_id = p.playing_id;
        self.cur_event_id = p.event_id;
        self.cur_game_obj = p.game_object_id;
        self.gs.sync_for_object(p.game_object_id, p.playing_id);

        let res = self.run_action_inner(p);

        self.cur_playing_id = prev_pid;
        self.cur_event_id = prev_ev;
        self.cur_game_obj = prev_obj;
        self.cur_ext_sources = prev_ext;
        res
    }

    fn run_action_inner(&mut self, p: &Pending) -> Result<(), String> {
        let o = self.project.action(p.action).unwrap();
        let HircBody::Action(act) = &o.body else { return Ok(()) };
        let fade_ms = act.props.get_raw(prop::TRANSITION_TIME).unwrap_or(0) as i32 as f32;
        let at = p.at;
        self.log.push(format!(
            "t={} action {} {:?} ({:#06x}) target {}{} fade {} ms curve {:?}",
            at,
            p.action,
            act.kind,
            act.action_type,
            act.target_id,
            if act.target_is_bus { " (bus)" } else { "" },
            fade_ms,
            match &act.params {
                ActionParams::Play { fade_curve, .. } | ActionParams::Active { fade_curve, .. } => Some(*fade_curve),
                _ => None,
            }
        ));
        if let ActionParams::Seek { relative_to_duration, value, snap_to_marker, .. } = &act.params {
            self.log.push(format!("  seek relative={} value={:?} snap={}", relative_to_duration, value, snap_to_marker));
        }
        match (act.kind, &act.params) {
            (ActionKind::Play, ActionParams::Play { fade_curve, .. }) => {
                if act.props.get_raw(prop::PROBABILITY).is_some() {
                    self.note("play_probability");
                }
                let fade = if fade_ms > 0.0 { Some(Fade { start: at, len: ms_to_samples(fade_ms), curve: *fade_curve as u32 }) } else { None };
                self.play_node(act.target_id, at, fade, 0)?;
            }
            (ActionKind::Stop | ActionKind::Pause | ActionKind::Resume, ActionParams::Active { fade_curve, exceptions, .. }) => {
                let scope = act.action_type & 0xff;
                let all = scope & 0x0c != 0; // 0x04 ALL, 0x08 ALL-EXCEPT
                let scope_obj = scope & 0x02 != 0 && p.game_object_id != 0 && p.game_object_id != u64::MAX;
                let exc: Vec<u32> = if scope & 0x08 != 0 { exceptions.iter().map(|e| e.0).collect() } else { vec![] };
                let target = act.target_id;
                let target_is_bus = act.target_is_bus;
                let len = ms_to_samples(fade_ms);
                let curve = *fade_curve as u32;
                for v in self.voices.iter_mut() {
                    if v.state == VState::Done {
                        continue;
                    }
                    if scope_obj && v.game_object_id != 0 && v.game_object_id != p.game_object_id {
                        continue;
                    }
                    let hit = if all {
                        !exc.iter().any(|e| v.chain.contains(e))
                    } else if target_is_bus {
                        v.params.buses.iter().any(|b| b.id == target)
                    } else {
                        v.chain.contains(&target)
                    };
                    if !hit {
                        continue;
                    }
                    match act.kind {
                        ActionKind::Stop => stop_voice(v, at, len, curve),
                        ActionKind::Pause => {
                            if v.state == VState::Playing {
                                pause_voice(v, at, len, curve);
                            } else if v.state == VState::Scheduled {
                                self.notes.insert("pause_scheduled_voice".into());
                            }
                        }
                        _ => {
                            resume_voice(v, at, len, curve);
                        }
                    }
                }
            }
            (ActionKind::Seek, ActionParams::Seek { relative_to_duration, value, .. }) => {
                self.note("seek");
                if value.1 != value.2 {
                    self.note("random_seek");
                }
                // value = (base, modifier min, modifier max). Relative seeks are a fraction of
                // the duration (0..1), not percent [V data: the oracle's seek of 1316883725
                // lands at exactly 0 + draw 4 * 0.5 of the source]. The modifier is drawn like
                // a ranged property (`draw + min`, no draw for an empty range) when the action
                // runs, after the plays of the same event [V: draw index; A: the AK VA is not
                // located yet].
                let v_seek = value.0 + self.rng.ranged_offset(value.1, value.2) + value.1;
                let target = act.target_id;
                for v in self.voices.iter_mut() {
                    if v.state == VState::Done || !v.chain.contains(&target) {
                        continue;
                    }
                    let n = v.pcm.frames() as f32;
                    // [A] truncation of the source position
                    let pos = if *relative_to_duration { v_seek.clamp(0.0, 1.0) * n } else { v_seek * v.pcm.rate as f32 / 1000.0 };
                    seek_voice(v, pos.max(0.0) as usize);
                }
            }
            (ActionKind::SetGameParameter, ActionParams::SetGameParameter { value, .. }) => {
                let rtpc_id = act.target_id;
                let off = if value.min != 0.0 || value.max != 0.0 {
                    self.rng.ranged_offset(value.min, value.max) + value.min
                } else {
                    0.0
                };
                let v = value.base + off;
                let scope_obj = act.action_type & 0x01 != 0;
                let cur = self.gs.rtpc_value(self.project, rtpc_id);
                let next = match (act.action_type & 0xFF00, value.meaning) {
                    (0x1400, _) | (_, 0) => self.project.param_default(rtpc_id) + off,
                    (_, 2) => cur + v,
                    _ => v,
                };
                if scope_obj {
                    self.gs.set_rtpc_for_object(self.gs.game_object_id, rtpc_id, next);
                } else {
                    self.gs.set_rtpc(rtpc_id, next);
                }
                self.refresh_voices_rtpc();
            }
            (ActionKind::SetState, ActionParams::SetState { group_id, state_id }) => {
                self.set_state_at(*group_id, *state_id, at)?;
            }
            (ActionKind::SetSwitch, ActionParams::SetSwitch { group_id, switch_id }) => {
                let scope_obj = act.action_type & 0x01 != 0;
                let obj = scope_obj.then_some(self.gs.game_object_id);
                self.set_switch_at(*group_id, *switch_id, obj, at)?;
            }
            (ActionKind::Trigger, _) => {
                self.post_trigger_at(act.target_id, at)?;
            }
            (ActionKind::BypassFx, ActionParams::BypassFx { bypass, target_mask, .. }) => {
                let target = act.target_id;
                let reset = matches!(act.action_type & 0xFF00, 0x1B00 | 0x3500..=0x3700);
                if reset {
                    self.gs.fx_bypass_overrides.remove(&target);
                } else {
                    let base_bits = if act.target_is_bus {
                        self.project.bus(target).and_then(|o| match &o.body {
                            HircBody::Bus(b) | HircBody::AuxBus(b) => Some(b.fx.bypass_bits),
                            _ => None,
                        })
                    } else {
                        self.project.node(target).and_then(|o| o.node_base()).map(|n| n.fx.bypass_bits)
                    }
                    .unwrap_or(0);
                    let cur = self.gs.fx_bypass_overrides.get(&target).copied().unwrap_or(base_bits);
                    let next = if *bypass { cur | *target_mask } else { cur & !*target_mask };
                    self.gs.fx_bypass_overrides.insert(target, next);
                }
                self.refresh_voices_rtpc();
            }
            (k, _) => {
                self.note(format!("action_{:?}", k));
            }
        }
        Ok(())
    }

    /// Play a node at `at`; returns the end sample of everything it started (None = infinite).
    /// Like AK's generic node play (`FUN_1415faca0`), the node's initial delay is computed (and
    /// its ranged modifier drawn) first, then the node plays after that delay.
    fn play_node(&mut self, id: u32, at: u64, fade: Option<Fade>, depth: usize) -> Result<Option<u64>, String> {
        if depth > 32 {
            return Err("container nesting too deep".into());
        }
        let o = self.project.node(id).ok_or_else(|| format!("node {} not loaded", id))?;
        let mut fade = fade;
        let at = match o.node_base() {
            Some(n) => {
                let d = node_initial_delay(self.project, &self.gs, n, &mut self.rng, &mut self.notes);
                if d > 0 {
                    // the delayed play re-enters with the same transition, which starts one frame
                    // before the frame holding the delayed start [V: event 553378243, a voice
                    // delayed to sample 48000 (frame 93) fades in from frame 92: -60 -> -132
                    // dBFS; starting in frame 93 or 94 is worse] [A: frame-aligned delays]
                    if let Some(f) = fade.as_mut() {
                        f.start = (f.start + d).saturating_sub(FRAME as u64).max(f.start);
                    }
                }
                at + d
            }
            None => at,
        };
        match &o.body {
            HircBody::Sound(_) => self.start_sound(id, at, fade),
            HircBody::RanSeqCntr(c) => {
                let c = c.clone();
                if c.playlist.is_empty() {
                    return Ok(Some(at));
                }
                let continuous = c.bits & 0x08 != 0;
                if !continuous {
                    let idx = self.step_select(id, &c);
                    let child = c.playlist[idx].0;
                    return self.play_node(child, at, fade, depth + 1);
                }
                self.note(format!("continuous_ranseq_transition_{}", c.transition_mode));
                // FUN_1415f9720: loop counter (and its modifier draw) when the play starts
                let mut li = LoopInfo::new(&c, &mut self.rng);
                let mut st = CntrState::default();
                if c.loop_mod_min != 0 || c.loop_mod_max != 0 {
                    self.note("random_loop_count");
                }
                if c.transition_time_mod_min != 0.0 || c.transition_time_mod_max != 0.0 {
                    self.note("random_transition_time");
                }
                if li.infinite {
                    self.note("infinite_ranseq");
                }
                let xfade_curve = if c.transition_mode == 2 { 1 } else { 4 };
                let mut t = at;
                let mut end_max = Some(at);
                let mut first = true;
                let mut cur = self.cont_select(&c, &mut st, &mut li);
                let mut items = 0usize;
                let mut last_progress = at;
                let mut xfade_len = 0u64;
                while let Some(idx) = cur {
                    if t >= self.horizon() {
                        end_max = None;
                        break;
                    }
                    items += 1;
                    if items % c.playlist.len().max(1) == 0 {
                        if t <= last_progress && items > c.playlist.len() {
                            // a whole pass produced no audio time (e.g. unsupported sources): AK
                            // would spin through it, we stop instead of looping forever
                            self.note("zero_length_ranseq_pass");
                            break;
                        }
                        last_progress = t;
                    }
                    let child = c.playlist[idx].0;
                    if c.transition_mode == 5 {
                        // trigger rate (FUN_1415fc410): select the next item before playing this
                        // one, then draw the trigger interval
                        let next = self.cont_select(&c, &mut st, &mut li);
                        let end = self.play_node(child, t, if first { fade } else { None }, depth + 1)?;
                        first = false;
                        end_max = match (end_max, end) {
                            (Some(a), Some(b)) => Some(a.max(b)),
                            _ => None,
                        };
                        if next.is_none() {
                            break;
                        }
                        let tt = self.transition_time_ms(&c);
                        t += (RATE as f32 as f64 * (tt / 1000.0) as f64 + 0.5) as i64 as u64;
                        cur = next;
                        continue;
                    }
                    let item_fade = match c.transition_mode {
                        1 | 2 if !first => Some(Fade { start: t, len: xfade_len, curve: xfade_curve }),
                        _ if first => fade,
                        _ => None,
                    };
                    let first_voice = self.voices.len();
                    let end = self.play_node(child, t, item_fade, depth + 1)?;
                    first = false;
                    let Some(end) = end else {
                        end_max = None;
                        break;
                    };
                    end_max = end_max.map(|e| e.max(end));
                    // FUN_1415c11f0: select the next item, then draw its transition time
                    let next = self.cont_select(&c, &mut st, &mut li);
                    if next.is_none() {
                        break;
                    }
                    let tt_ms = self.transition_time_ms(&c);
                    let tt = ms_to_samples(tt_ms);
                    t = match c.transition_mode {
                        1 | 2 => {
                            // crossfade: next item starts tt before this one ends; fade this one
                            // out and the next one in over the overlap [A]
                            let s = end.saturating_sub(tt).max(t);
                            for v in self.voices[first_voice..].iter_mut() {
                                start_fade_out(v, Fade { start: s, len: end - s, curve: xfade_curve });
                            }
                            s
                        }
                        3 => end + tt,
                        _ => end, // 0 disabled, 4 sample accurate [A]
                    };
                    // the next item's crossfade-in spans the overlap
                    xfade_len = end.saturating_sub(t);
                    cur = next;
                }
                Ok(end_max)
            }
            HircBody::SwitchCntr(c) => {
                let c = c.clone();
                let val = if c.group_type == 1 {
                    self.gs.states.get(&c.group_id).copied()
                } else {
                    self.gs.switch_value(c.group_id)
                }
                .unwrap_or(c.default_switch);
                let nodes = c
                    .switches
                    .iter()
                    .find(|(s, _)| *s == val)
                    .or_else(|| c.switches.iter().find(|(s, _)| *s == c.default_switch))
                    .map(|(_, n)| n.clone())
                    .unwrap_or_default();
                if c.continuous_validation {
                    if let Some(ex) = self.active_switch_cntrs.iter_mut().find(|a| a.cntr_id == id) {
                        ex.active_switch = val;
                        ex.active_children = nodes.clone();
                        ex.playing_id = self.cur_playing_id;
                    } else {
                        self.active_switch_cntrs.push(ActiveSwitchCntr {
                            cntr_id: id,
                            group_type: c.group_type,
                            group_id: c.group_id,
                            active_switch: val,
                            active_children: nodes.clone(),
                            depth,
                            playing_id: self.cur_playing_id,
                        });
                    }
                }
                let mut end = Some(at);
                for n in nodes {
                    let sp = c.params.iter().find(|p| p.node_id == n);
                    let is_first_only = sp.is_some_and(|p| p.bits & 1 != 0);
                    if is_first_only && self.switch_played_once.contains(&(id, n)) {
                        continue;
                    }
                    self.switch_played_once.insert((id, n));
                    let child_fade = fade.or_else(|| {
                        let fi_ms = sp.map_or(0, |p| p.fade_in_ms);
                        (fi_ms > 0).then(|| Fade { start: at, len: ms_to_samples(fi_ms as f32), curve: 4 })
                    });
                    let e = self.play_node(n, at, child_fade, depth + 1)?;
                    end = match (end, e) {
                        (Some(a), Some(b)) => Some(a.max(b)),
                        _ => None,
                    };
                }
                Ok(end)
            }
            HircBody::LayerCntr(c) => {
                let c = c.clone();
                if c.layers.iter().any(|l| l.crossfade_rtpc_id != 0) {
                    self.note("layer_crossfade_rtpc");
                }
                let mut end = Some(at);
                for &ch in &c.children {
                    let e = self.play_node(ch, at, fade, depth + 1)?;
                    end = match (end, e) {
                        (Some(a), Some(b)) => Some(a.max(b)),
                        _ => None,
                    };
                }
                Ok(end)
            }
            HircBody::MusicTrack(_) => self.play_music_track(id, at, 0.0, f64::INFINITY, fade, &[]),
            HircBody::MusicSegment(_) => self.play_music_segment(id, at, true, true, fade, &[]),
            HircBody::MusicRanSeqCntr(_) => self.play_music_ranseq(id, at, fade, &[]),
            HircBody::MusicSwitchCntr(_) => self.play_music_switch(id, at, fade, depth, &[]),
            _ => {
                self.note(format!("play_target_type_{:#x}", o.hirc_type));
                Ok(Some(at))
            }
        }
    }

    /// Step-mode RanSeq selection (`FUN_1415fa360`): per-(container, game object) state; a
    /// single-item playlist is picked without a draw.
    fn step_select(&mut self, id: u32, c: &crate::bnk::hirc::RanSeqCntr) -> usize {
        let n = c.playlist.len();
        if n == 1 {
            return 0;
        }
        let mut st = self.cntr.remove(&id).unwrap_or_default();
        let idx = if c.mode == 1 {
            st.seq.next(c, None).unwrap_or(0)
        } else {
            self.notes.insert("random_container".into());
            st.random.get_or_insert_with(|| RandomInfo::new(c)).select(c, &mut self.rng, None).unwrap_or(0)
        };
        self.cntr.insert(id, st);
        idx.min(n - 1)
    }

    /// Continuous-mode selection (`FUN_1415fa780`): state private to this play.
    fn cont_select(&mut self, c: &crate::bnk::hirc::RanSeqCntr, st: &mut CntrState, li: &mut LoopInfo) -> Option<usize> {
        let n = c.playlist.len();
        let r = if n == 1 {
            single_item_next(li)
        } else if c.mode == 1 {
            st.seq.next(c, Some(li))
        } else {
            self.notes.insert("random_container".into());
            st.random.get_or_insert_with(|| RandomInfo::new(c)).select(c, &mut self.rng, Some(li))
        };
        r.map(|i| i.min(n - 1))
    }

    /// Transition time of the next continuous item, ms (`FUN_1415fbfc0`): `draw + min + base`,
    /// plus RTPC 0xF (`PlayMechanismSpecialTransitionsValue`, seconds x 1000), clamped at 0.
    fn transition_time_ms(&mut self, c: &crate::bnk::hirc::RanSeqCntr) -> f32 {
        let d = c.transition_time_mod_max - c.transition_time_mod_min;
        let r = if d != 0.0 { self.rng.next() as f32 * INV_2_31_F32 * d } else { 0.0 };
        let mut rt = 0.0f32;
        for rtpc in &c.node.rtpcs {
            if rtpc.rtpc_type == 0 && rtpc.param_id == super::params::rtpc_param::PLAY_MECHANISM_SPECIAL_TRANSITIONS_VALUE {
                let x = self.gs.rtpc_value(self.project, rtpc.rtpc_id);
                let y = super::math::eval_graph(&rtpc.points, rtpc.scaling, x) * 1000.0;
                super::params::accum_value(&mut rt, y, rtpc.accum, false);
            }
        }
        let v = r + c.transition_time_mod_min + c.transition_time + rt;
        v.max(0.0)
    }

    /// Update an RTPC value (global when `game_object_id.is_none()`, else per-game-object) and
    /// refresh active voices.
    pub fn set_rtpc_value(&mut self, rtpc_id: u32, value: f32, game_object_id: Option<u64>) {
        match game_object_id {
            Some(obj) => self.gs.set_rtpc_for_object(obj, rtpc_id, value),
            None => self.gs.set_rtpc(rtpc_id, value),
        }
        self.refresh_voices_rtpc();
    }

    /// Set a global State (`AK::SoundEngine::SetState`) at the current render sample, refreshing
    /// StateChunk overrides on active voices/buses and re-validating continuous state-driven
    /// `SwitchCntr` and `MusicSwitchCntr` containers.
    pub fn set_state(&mut self, group_id: u32, state_id: u32) -> Result<(), String> {
        self.set_state_at(group_id, state_id, self.now)
    }

    fn set_state_at(&mut self, group_id: u32, state_id: u32, at: u64) -> Result<(), String> {
        self.gs.set_state(group_id, state_id);
        self.refresh_voices_rtpc();
        self.update_continuous_switch_cntrs(1, group_id, at)?;
        self.update_music_switch_cntrs(at)
    }

    /// Set a Switch (`AK::SoundEngine::SetSwitch`) at the current render sample (global when
    /// `game_object_id.is_none()`, else per-game-object) and re-validate continuous switch-driven
    /// `SwitchCntr` and `MusicSwitchCntr` containers.
    pub fn set_switch(&mut self, group_id: u32, switch_id: u32, game_object_id: Option<u64>) -> Result<(), String> {
        self.set_switch_at(group_id, switch_id, game_object_id, self.now)
    }

    fn set_switch_at(&mut self, group_id: u32, switch_id: u32, game_object_id: Option<u64>, at: u64) -> Result<(), String> {
        match game_object_id {
            Some(obj) => self.gs.set_switch_for_object(obj, group_id, switch_id),
            None => self.gs.set_switch(group_id, switch_id),
        }
        self.update_continuous_switch_cntrs(0, group_id, at)?;
        self.update_music_switch_cntrs(at)
    }

    /// Post a Wwise Music Trigger (`AK::SoundEngine::PostTrigger`) at the current render sample,
    /// scheduling any matching `Stinger` segments on active music hierarchies at their sync point.
    pub fn post_trigger(&mut self, trigger_id: u32) -> Result<(), String> {
        self.post_trigger_at(trigger_id, self.now)
    }

    fn post_trigger_at(&mut self, trigger_id: u32, at: u64) -> Result<(), String> {
        // Find active music segment(s) playing at `at`
        let active_segs: Vec<ActiveMusicSegment> = self
            .active_music_segments
            .iter()
            .filter(|s| s.origin_sample <= at && at < s.end_sample)
            .cloned()
            .collect();
        let fallback_segs = if active_segs.is_empty() {
            self.active_music_segments.last().cloned().into_iter().collect::<Vec<_>>()
        } else {
            active_segs
        };

        let mut fired_owners = BTreeSet::new();
        for aseg in fallback_segs {
            let Some(seg_obj) = self.project.node(aseg.segment_id) else { continue };
            let HircBody::MusicSegment(seg) = &seg_obj.body else { continue };
            let seg = seg.clone();
            // Search stingers on the segment and its owner_chain
            let mut candidates = super::music::find_stingers(self.project, aseg.segment_id, trigger_id);
            for &owner in &aseg.owner_chain {
                for st in super::music::find_stingers(self.project, owner, trigger_id) {
                    if !candidates.iter().any(|(o, s)| *o == st.0 && s.segment_id == st.1.segment_id) {
                        candidates.push(st);
                    }
                }
            }
            for (owner_id, stinger) in candidates {
                if fired_owners.contains(&(owner_id, stinger.segment_id)) {
                    continue;
                }
                let dont_repeat_samples = super::music::ms_to_samples_u64(stinger.dont_repeat_time.max(0) as f64);
                if let Some(&last_at) = self.stinger_last_fired.get(&(owner_id, trigger_id)) {
                    if at < last_at + dont_repeat_samples {
                        continue;
                    }
                }
                let Some(sync_at) = super::music::compute_sync_point(
                    &seg,
                    None,
                    aseg.origin_sample,
                    at,
                    stinger.sync_play_at,
                    stinger.cue_filter_hash,
                ) else {
                    continue;
                };
                self.stinger_last_fired.insert((owner_id, trigger_id), at);
                fired_owners.insert((owner_id, stinger.segment_id));
                // Play stinger segment with its pre-entry aligned so its entry marker lands on `sync_at`
                let stinger_entry_samples = self
                    .project
                    .node(stinger.segment_id)
                    .and_then(|o| match &o.body {
                        HircBody::MusicSegment(ms) => Some(super::music::SegmentBounds::from_segment(ms).entry_samples),
                        _ => None,
                    })
                    .unwrap_or(0);
                let stinger_start = sync_at.saturating_sub(stinger_entry_samples);
                let play_pre_entry = sync_at >= stinger_entry_samples;
                self.play_music_segment(
                    stinger.segment_id,
                    stinger_start,
                    play_pre_entry,
                    true,
                    None,
                    &[owner_id],
                )?;
            }
        }
        Ok(())
    }

    /// Play a [`MusicTrack`] (`0x0b`) within a segment timeline starting at `seg_origin_sample`
    /// and audible across `[window_start_ms, window_end_ms]`.
    fn play_music_track(
        &mut self,
        track_id: u32,
        seg_origin_sample: u64,
        window_start_ms: f64,
        window_end_ms: f64,
        fade: Option<Fade>,
        extra_chain: &[u32],
    ) -> Result<Option<u64>, String> {
        let Some(o) = self.project.node(track_id) else {
            return Err(format!("music track {} not loaded", track_id));
        };
        let HircBody::MusicTrack(track) = &o.body else {
            return Err(format!("node {} is not a MusicTrack", track_id));
        };
        let track = track.clone();
        let Some(sub_track) = super::music::select_sub_track(
            track_id,
            &track,
            &self.gs,
            &mut self.rng,
            &mut self.music_track_seq,
        ) else {
            return Ok(Some(seg_origin_sample));
        };

        let params = resolve(self.project, &self.gs, track_id, &mut self.rng)?;
        for n in &params.notes {
            self.notes.insert(n.clone());
        }
        let mut base_chain = Vec::new();
        let mut nid = track_id;
        while nid != 0 && base_chain.len() < 64 {
            base_chain.push(nid);
            nid = self.project.node(nid).and_then(|o| o.node_base()).map_or(0, |n| n.direct_parent_id);
        }
        for &ec in extra_chain {
            if !base_chain.contains(&ec) {
                base_chain.push(ec);
            }
        }

        let mut max_end = Some(seg_origin_sample);
        for (clip_idx, clip) in track.playlist.iter().enumerate() {
            if clip.track_id != sub_track {
                continue;
            }
            if clip.event_id != 0 {
                let ev_at = seg_origin_sample + super::music::ms_to_samples_u64(clip.play_at.max(0.0));
                let _ = self.post_event(clip.event_id, ev_at);
            }
            let Some(src) = track.sources.iter().find(|s| s.source_id == clip.source_id).cloned() else {
                continue;
            };
            let pcm = match self.load_source_pcm(&src) {
                Ok(p) => p,
                Err(e) => {
                    self.note(format!("unsupported_source: {}", e));
                    continue;
                }
            };
            let Some(sched) = super::music::schedule_track_clip(
                clip_idx,
                clip,
                pcm.frames(),
                pcm.rate,
                seg_origin_sample,
                window_start_ms,
                window_end_ms,
                &track.clip_automation,
            ) else {
                continue;
            };
            if self.voices.len() >= 20_000 {
                return Err("more than 20000 voices scheduled".into());
            }
            let ch = pcm.data.len();
            let mut res = Resampler::new(ch, pcm.rate, self.opts.resampler_fma);
            res.set_pitch(params.pitch_cents, false);
            let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &params);
            let (folded_out_vol, folded_out_lpf, folded_out_hpf) = params.fold_passthrough_buses(self.opts.bus_fx);
            let base_lin = voice_gain(params.volume_db, params.makeup_db, folded_out_vol) * pcm.akd_gain * ray_dry;
            let conn_lpf = (params.lpf + folded_out_lpf).max(ray_lpf).clamp(0.0, 100.0);
            let conn_hpf = (params.hpf + folded_out_hpf).max(ray_hpf).clamp(0.0, 100.0);
            let filter = (conn_lpf > 0.0 || conn_hpf > 0.0)
                .then(|| ConnectionFilter::new(conn_lpf, conn_hpf, ch, RATE));
            let mut f_in = fade;
            if let Some(f) = f_in.as_mut() {
                if f.len == 0 {
                    f_in = None;
                }
            }
            let end = Some(sched.end_sample);
            max_end = match (max_end, end) {
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            };
            let node_fx: Vec<(u8, FxInstance)> = params
                .node_fx
                .iter()
                .filter_map(|&(slot, fx_id, _)| FxInstance::from_project(self.project, fx_id).map(|inst| (slot, inst)))
                .collect();
            self.voices.push(Voice {
                sound_id: track_id,
                playing_id: self.cur_playing_id,
                event_id: self.cur_event_id,
                game_object_id: self.cur_game_obj,
                chain: base_chain.clone(),
                params: params.clone(),
                pcm,
                start: sched.start_sample,
                state: VState::Scheduled,
                evicted_by_limit: false,
                segments: sched.pcm_segments,
                seg: 0,
                seg_pos: 0,
                res,
                base_lin,
                ray_user_aux,
                ray_game_aux,
                ray_lpf,
                ray_hpf,
                prev_gain: None,
                aux_prev_gain: HashMap::new(),
                aux_filters: HashMap::new(),
                node_fx,
                fade_in: f_in,
                fade_out: None,
                stop_at: None,
                end,
                matrix,
                filter,
                pause_fade: None,
                clip_origin_sample: sched.clip_origin_sample,
                clip_automations: sched.automations,
            });
        }
        Ok(max_end)
    }

    /// Play a [`MusicSegment`] (`0x0a`) whose timeline `0..duration` is anchored at `seg_origin_sample`.
    /// When `play_pre_entry` is false, clips are trimmed at `entry_ms`; when `play_post_exit` is
    /// false, clips are trimmed at `exit_ms`.
    fn play_music_segment(
        &mut self,
        segment_id: u32,
        seg_origin_sample: u64,
        play_pre_entry: bool,
        play_post_exit: bool,
        fade: Option<Fade>,
        extra_chain: &[u32],
    ) -> Result<Option<u64>, String> {
        let Some(o) = self.project.node(segment_id) else {
            return Err(format!("music segment {} not loaded", segment_id));
        };
        let HircBody::MusicSegment(seg) = &o.body else {
            return Err(format!("node {} is not a MusicSegment", segment_id));
        };
        let seg = seg.clone();
        let bounds = super::music::SegmentBounds::from_segment(&seg);
        let win_start_ms = if play_pre_entry { 0.0 } else { bounds.entry_ms };
        let win_end_ms = if play_post_exit { bounds.duration_ms } else { bounds.exit_ms };

        let mut chain = vec![segment_id];
        for &ec in extra_chain {
            if !chain.contains(&ec) {
                chain.push(ec);
            }
        }
        let entry_sample = seg_origin_sample + bounds.entry_samples;
        let exit_sample = seg_origin_sample + bounds.exit_samples;
        let end_sample = seg_origin_sample + super::music::ms_to_samples_u64(win_end_ms);
        self.active_music_segments.push(ActiveMusicSegment {
            owner_chain: chain.clone(),
            segment_id,
            playing_id: self.cur_playing_id,
            event_id: self.cur_event_id,
            game_object_id: self.cur_game_obj,
            origin_sample: seg_origin_sample,
            entry_sample,
            exit_sample,
            end_sample,
        });

        let mut max_end = Some(end_sample);
        for &child_id in &seg.music.children {
            let e = self.play_music_track(
                child_id,
                seg_origin_sample,
                win_start_ms,
                win_end_ms,
                fade,
                &chain,
            )?;
            max_end = match (max_end, e) {
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            };
        }
        Ok(max_end)
    }

    /// Play a [`MusicRanSeqCntr`] (`0x0d`), traversing its playlist tree and chaining segments
    /// sample-accurately across entry/exit markers up to `horizon()`.
    fn play_music_ranseq(
        &mut self,
        ranseq_id: u32,
        at: u64,
        fade: Option<Fade>,
        extra_chain: &[u32],
    ) -> Result<Option<u64>, String> {
        let Some(o) = self.project.node(ranseq_id) else {
            return Err(format!("music ranseq {} not loaded", ranseq_id));
        };
        let HircBody::MusicRanSeqCntr(cntr) = &o.body else {
            return Err(format!("node {} is not a MusicRanSeqCntr", ranseq_id));
        };
        let cntr = cntr.clone();
        let mut runner = self.music_playlists.remove(&ranseq_id).unwrap_or_default();
        let seg_ids = runner.next_segments(&cntr.playlist, &mut self.rng, 32);
        self.music_playlists.insert(ranseq_id, runner);

        if seg_ids.is_empty() {
            return Ok(Some(at));
        }

        let mut chain = vec![ranseq_id];
        for &ec in extra_chain {
            if !chain.contains(&ec) {
                chain.push(ec);
            }
        }

        let horizon = self.horizon();
        // `cur_entry_sample` is where the current segment's Entry marker is aligned.
        // For the first segment, if it has a pre-entry and rules allow it, we start at `at` so
        // pre-entry plays from `at` (or if `at >= entry_samples` when chained, pre-entry overlaps
        // before `cur_entry_sample`).
        let mut cur_entry_sample = at;
        let mut max_end = Some(at);
        let mut prev_seg_id: u32 = 0;

        for (i, &seg_id) in seg_ids.iter().enumerate() {
            let Some(so) = self.project.node(seg_id) else { continue };
            let HircBody::MusicSegment(seg) = &so.body else { continue };
            let bounds = super::music::SegmentBounds::from_segment(seg);

            let rule = super::music::match_transition_rule(
                self.project,
                &cntr.trans.rules,
                prev_seg_id,
                seg_id,
            );
            let play_pre_entry = rule.map_or(true, |r| r.play_pre_entry);
            let play_post_exit = rule.map_or(true, |r| r.play_post_exit);

            let seg_origin = if i == 0 {
                if play_pre_entry && cur_entry_sample >= bounds.entry_samples {
                    cur_entry_sample - bounds.entry_samples
                } else {
                    cur_entry_sample
                }
            } else {
                cur_entry_sample.saturating_sub(bounds.entry_samples)
            };

            if seg_origin >= horizon && i > 0 {
                break;
            }

            let seg_fade = if i == 0 {
                fade.or_else(|| {
                    rule.and_then(|r| super::music::music_fade_window(&r.dst_fade, seg_origin))
                        .map(|(start, len, curve)| Fade { start, len, curve })
                })
            } else {
                rule.and_then(|r| super::music::music_fade_window(&r.dst_fade, cur_entry_sample))
                    .map(|(start, len, curve)| Fade { start, len, curve })
            };

            let end = self.play_music_segment(
                seg_id,
                seg_origin,
                play_pre_entry || i == 0,
                play_post_exit,
                seg_fade,
                &chain,
            )?;
            max_end = match (max_end, end) {
                (Some(a), Some(b)) => Some(a.max(b)),
                _ => None,
            };

            let next_exit = seg_origin + bounds.exit_samples;
            if next_exit <= cur_entry_sample && bounds.duration_samples == 0 {
                break;
            }
            cur_entry_sample = next_exit.max(cur_entry_sample + 1);
            prev_seg_id = seg_id;
            if cur_entry_sample >= horizon {
                break;
            }
        }
        Ok(max_end)
    }

    /// Play a [`MusicSwitchCntr`] (`0x0c`), resolving its `DecisionTree` against current State /
    /// Switch values and registering it for dynamic transition rule evaluation on state/switch changes.
    fn play_music_switch(
        &mut self,
        switch_cntr_id: u32,
        at: u64,
        fade: Option<Fade>,
        depth: usize,
        extra_chain: &[u32],
    ) -> Result<Option<u64>, String> {
        if depth > 32 {
            return Err("music switch container nesting too deep".into());
        }
        let Some(o) = self.project.node(switch_cntr_id) else {
            return Err(format!("music switch cntr {} not loaded", switch_cntr_id));
        };
        let HircBody::MusicSwitchCntr(cntr) = &o.body else {
            return Err(format!("node {} is not a MusicSwitchCntr", switch_cntr_id));
        };
        let cntr = cntr.clone();
        let resolved = super::music::resolve_decision_tree(&cntr.tree, &self.gs, &mut self.rng)
            .unwrap_or(0);

        if let Some(ex) = self.active_music_switches.iter_mut().find(|a| a.cntr_id == switch_cntr_id) {
            ex.active_child = resolved;
            ex.playing_id = self.cur_playing_id;
        } else {
            self.active_music_switches.push(ActiveMusicSwitchCntr {
                cntr_id: switch_cntr_id,
                active_child: resolved,
                depth,
                playing_id: self.cur_playing_id,
            });
        }

        if resolved == 0 {
            return Ok(Some(at));
        }

        let mut chain = vec![switch_cntr_id];
        for &ec in extra_chain {
            if !chain.contains(&ec) {
                chain.push(ec);
            }
        }

        let rule = super::music::match_transition_rule(self.project, &cntr.trans.rules, 0, resolved);
        let init_fade = fade.or_else(|| {
            rule.and_then(|r| super::music::music_fade_window(&r.dst_fade, at))
                .map(|(start, len, curve)| Fade { start, len, curve })
        });

        self.play_music_child(resolved, at, init_fade, depth + 1, &chain)
    }

    fn play_music_child(
        &mut self,
        node_id: u32,
        at: u64,
        fade: Option<Fade>,
        depth: usize,
        extra_chain: &[u32],
    ) -> Result<Option<u64>, String> {
        let Some(o) = self.project.node(node_id) else {
            return Ok(Some(at));
        };
        match &o.body {
            HircBody::MusicSegment(_) => self.play_music_segment(node_id, at, true, true, fade, extra_chain),
            HircBody::MusicRanSeqCntr(_) => self.play_music_ranseq(node_id, at, fade, extra_chain),
            HircBody::MusicSwitchCntr(_) => self.play_music_switch(node_id, at, fade, depth, extra_chain),
            HircBody::MusicTrack(_) => self.play_music_track(node_id, at, 0.0, f64::INFINITY, fade, extra_chain),
            _ => self.play_node(node_id, at, fade, depth),
        }
    }

    /// Re-evaluate active [`MusicSwitchCntr`] decision trees at sample `at` after a `set_state` or
    /// `set_switch` call, scheduling transitions according to the matched [`TransitionRule`].
    fn update_music_switch_cntrs(&mut self, at: u64) -> Result<(), String> {
        let count = self.active_music_switches.len();
        for idx in 0..count {
            let cntr_id = self.active_music_switches[idx].cntr_id;
            let old_child = self.active_music_switches[idx].active_child;
            let depth = self.active_music_switches[idx].depth;
            let Some(o) = self.project.node(cntr_id) else { continue };
            let HircBody::MusicSwitchCntr(cntr) = &o.body else { continue };
            let cntr = cntr.clone();
            let new_child = super::music::resolve_decision_tree(&cntr.tree, &self.gs, &mut self.rng)
                .unwrap_or(0);
            if new_child == old_child && cntr.continue_playback {
                continue;
            }
            self.active_music_switches[idx].active_child = new_child;

            let rule = super::music::match_transition_rule(
                self.project,
                &cntr.trans.rules,
                old_child,
                new_child,
            )
            .cloned();

            let sync_type = rule.as_ref().map_or(super::music::sync_type::IMMEDIATE, |r| r.src_sync_type);
            let cue_hash = rule.as_ref().map_or(0, |r| r.src_cue_filter_hash);

            // Find currently active segment under `cntr_id` at `at`
            let cur_seg = self
                .active_music_segments
                .iter()
                .filter(|s| s.owner_chain.contains(&cntr_id) && s.origin_sample <= at && at <= s.end_sample)
                .min_by_key(|s| s.origin_sample)
                .cloned()
                .or_else(|| {
                    self.active_music_segments
                        .iter()
                        .rev()
                        .find(|s| s.owner_chain.contains(&cntr_id))
                        .cloned()
                });

            let sync_sample = if let Some(ref aseg) = cur_seg {
                if let Some(so) = self.project.node(aseg.segment_id) {
                    if let HircBody::MusicSegment(ms) = &so.body {
                        let meter_ovr = (cntr.trans.music.meter.override_flag != 0)
                            .then_some(&cntr.trans.music.meter);
                        super::music::compute_sync_point(ms, meter_ovr, aseg.origin_sample, at, sync_type, cue_hash)
                    } else {
                        Some(at)
                    }
                } else {
                    Some(at)
                }
            } else {
                Some(at)
            };

            let Some(sync_sample) = sync_sample else {
                // EXIT_NEVER: do not transition
                continue;
            };

            // Stop/fade-out voices belonging to `old_child` (and any future queued segments under `cntr_id`)
            let src_fade = rule
                .as_ref()
                .and_then(|r| super::music::music_fade_window(&r.src_fade, sync_sample));
            for v in self.voices.iter_mut() {
                if v.state == VState::Done || !v.chain.contains(&cntr_id) {
                    continue;
                }
                if old_child != 0 && !v.chain.contains(&old_child) {
                    continue;
                }
                if v.start >= sync_sample {
                    v.state = VState::Done;
                    continue;
                }
                if let Some((f_start, f_len, f_curve)) = src_fade {
                    let f = Fade { start: f_start, len: f_len, curve: f_curve };
                    start_fade_out(v, f);
                    v.stop_at = Some(f.end());
                } else {
                    stop_voice(v, sync_sample, 0, 4);
                }
            }
            // Trim or remove queued ActiveMusicSegments that haven't started yet
            self.active_music_segments.retain(|s| {
                !(s.owner_chain.contains(&cntr_id) && s.origin_sample >= sync_sample)
            });

            if new_child == 0 {
                continue;
            }

            let chain = vec![cntr_id];
            let mut dst_anchor = sync_sample;

            // Optional transition segment (`TransitionObject`)
            if let Some(trans_obj) = rule.as_ref().and_then(|r| r.transition_object.as_ref()) {
                if trans_obj.segment_id != 0 {
                    if let Some(tso) = self.project.node(trans_obj.segment_id) {
                        if let HircBody::MusicSegment(ts) = &tso.body {
                            let ts_bounds = super::music::SegmentBounds::from_segment(ts);
                            let play_pre = trans_obj.play_pre_entry;
                            let play_post = trans_obj.play_post_exit;
                            let ts_origin = if play_pre && sync_sample >= ts_bounds.entry_samples {
                                sync_sample - ts_bounds.entry_samples
                            } else {
                                sync_sample
                            };
                            let ts_fade = super::music::music_fade_window(&trans_obj.fade_in, sync_sample)
                                .map(|(start, len, curve)| Fade { start, len, curve });
                            self.play_music_segment(
                                trans_obj.segment_id,
                                ts_origin,
                                play_pre,
                                play_post,
                                ts_fade,
                                &chain,
                            )?;
                            dst_anchor = ts_origin + ts_bounds.exit_samples;
                        }
                    }
                }
            }

            let dst_fade = rule
                .as_ref()
                .and_then(|r| super::music::music_fade_window(&r.dst_fade, dst_anchor))
                .map(|(start, len, curve)| Fade { start, len, curve });

            self.play_music_child(new_child, dst_anchor, dst_fade, depth + 1, &chain)?;
        }
        Ok(())
    }

    fn update_continuous_switch_cntrs(&mut self, group_type: u8, group_id: u32, at: u64) -> Result<(), String> {
        let count = self.active_switch_cntrs.len();
        for idx in 0..count {
            if self.active_switch_cntrs[idx].group_type != group_type || self.active_switch_cntrs[idx].group_id != group_id {
                continue;
            }
            let cntr_id = self.active_switch_cntrs[idx].cntr_id;
            let old_switch = self.active_switch_cntrs[idx].active_switch;
            let old_children = self.active_switch_cntrs[idx].active_children.clone();
            let depth = self.active_switch_cntrs[idx].depth;
            let Some(o) = self.project.node(cntr_id) else { continue };
            let HircBody::SwitchCntr(c) = &o.body else { continue };
            let c = c.clone();
            let new_val = if c.group_type == 1 {
                self.gs.states.get(&c.group_id).copied()
            } else {
                self.gs.switch_value(c.group_id)
            }
            .unwrap_or(c.default_switch);
            if new_val == old_switch {
                continue;
            }
            let new_nodes = c
                .switches
                .iter()
                .find(|(s, _)| *s == new_val)
                .or_else(|| c.switches.iter().find(|(s, _)| *s == c.default_switch))
                .map(|(_, n)| n.clone())
                .unwrap_or_default();
            self.active_switch_cntrs[idx].active_switch = new_val;
            self.active_switch_cntrs[idx].active_children = new_nodes.clone();

            // Stop old children when not continuing playback and on_switch_mode == 1 (Stop)
            for &old_ch in &old_children {
                let sp = c.params.iter().find(|p| p.node_id == old_ch);
                let continue_playback = sp.is_some_and(|p| p.bits & 2 != 0);
                let on_switch_mode = sp.map_or(0, |p| p.on_switch_mode);
                let fade_out_ms = sp.map_or(0, |p| p.fade_out_ms);
                if continue_playback && new_nodes.contains(&old_ch) {
                    continue;
                }
                if on_switch_mode == 1 {
                    let len = ms_to_samples(fade_out_ms.max(0) as f32);
                    for v in self.voices.iter_mut() {
                        if v.state != VState::Done && v.chain.contains(&cntr_id) && v.chain.contains(&old_ch) {
                            stop_voice(v, at, len, 4);
                        }
                    }
                }
            }

            // Start new children (skipping those that continue playback and are still active)
            for &new_ch in &new_nodes {
                let sp = c.params.iter().find(|p| p.node_id == new_ch);
                let is_first_only = sp.is_some_and(|p| p.bits & 1 != 0);
                let continue_playback = sp.is_some_and(|p| p.bits & 2 != 0);
                let fade_in_ms = sp.map_or(0, |p| p.fade_in_ms);
                if continue_playback && old_children.contains(&new_ch) {
                    let still_playing = self.voices.iter().any(|v| {
                        v.state != VState::Done && v.stop_at.is_none() && v.chain.contains(&cntr_id) && v.chain.contains(&new_ch)
                    });
                    if still_playing {
                        continue;
                    }
                }
                if is_first_only && self.switch_played_once.contains(&(cntr_id, new_ch)) {
                    continue;
                }
                self.switch_played_once.insert((cntr_id, new_ch));
                let fade_in = (fade_in_ms > 0).then(|| Fade { start: at, len: ms_to_samples(fade_in_ms as f32), curve: 4 });
                self.play_node(new_ch, at, fade_in, depth + 1)?;
            }
        }
        Ok(())
    }

    pub fn refresh_voices_rtpc(&mut self) {
        let saved_obj = self.gs.game_object_id;
        let saved_pid = self.gs.active_playing_id;
        let mut dummy_rng = Rng::new(0x1234);
        for i in 0..self.voices.len() {
            if self.voices[i].state == VState::Done {
                continue;
            }
            let sid = self.voices[i].sound_id;
            let v_obj = self.voices[i].game_object_id;
            let v_pid = self.voices[i].playing_id;
            self.gs.sync_for_object(v_obj, v_pid);
            if let Ok(vp) = resolve(self.project, &self.gs, sid, &mut dummy_rng) {
                let pcm = self.voices[i].pcm.clone();
                let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &vp);
                let out_bus_vol_db = self
                    .gs
                    .obj_output_bus_volume_db
                    .get(&(v_obj, 0))
                    .copied()
                    .unwrap_or(0.0);
                let (folded_out_vol, folded_out_lpf, folded_out_hpf) = vp.fold_passthrough_buses(self.opts.bus_fx);
                let v = &mut self.voices[i];
                v.base_lin = voice_gain(vp.volume_db, vp.makeup_db, folded_out_vol + out_bus_vol_db) * v.pcm.akd_gain * ray_dry;
                v.ray_user_aux = ray_user_aux;
                v.ray_game_aux = ray_game_aux;
                v.ray_lpf = ray_lpf;
                v.ray_hpf = ray_hpf;
                v.res.set_pitch(vp.pitch_cents, true);
                v.matrix = matrix;
                let conn_lpf = (vp.lpf + folded_out_lpf).max(ray_lpf).clamp(0.0, 100.0);
                let conn_hpf = (vp.hpf + folded_out_hpf).max(ray_hpf).clamp(0.0, 100.0);
                v.filter = (conn_lpf > 0.0 || conn_hpf > 0.0).then(|| ConnectionFilter::new(conn_lpf, conn_hpf, v.pcm.data.len(), RATE));
                let mut old_fx = std::mem::take(&mut v.node_fx);
                let mut new_fx = Vec::with_capacity(vp.node_fx.len());
                for &(slot, fx_id, _) in &vp.node_fx {
                    if let Some(pos) = old_fx.iter().position(|(s, inst)| *s == slot && inst.fx_id == fx_id) {
                        new_fx.push(old_fx.remove(pos));
                    } else if let Some(inst) = FxInstance::from_project(self.project, fx_id) {
                        new_fx.push((slot, inst));
                    }
                }
                v.node_fx = new_fx;
                v.params = vp;
            }
        }
        self.gs.game_object_id = saved_obj;
        self.gs.active_playing_id = saved_pid;
        self.bus_voices_seen = usize::MAX;
    }

    fn load_pcm(&mut self, sound_id: u32) -> Result<Rc<Pcm>, String> {
        let o = self.project.node(sound_id).unwrap();
        let HircBody::Sound(s) = &o.body else { unreachable!() };
        let src = s.source.clone();
        self.load_source_pcm(&src)
    }

    fn decode_wem_to_pcm(buf: &[u8]) -> Result<Rc<Pcm>, String> {
        let wem = crate::wem::Wem::parse(buf).map_err(|e| e.to_string())?;
        let d = decode_wem(buf)?;
        let ch = d.channels as usize;
        let frames = d.samples.len() / ch.max(1);
        let mut data = vec![Vec::with_capacity(frames); ch];
        for f in 0..frames {
            for c in 0..ch {
                data[c].push(d.samples[f * ch + c]);
            }
        }
        let layout = Layout::from_wem(wem.channels, wem.channel_config_type, wem.channel_mask);
        Ok(Rc::new(Pcm {
            rate: d.sample_rate,
            layout,
            data,
            loop_points: wem.loop_points,
            akd_gain: wem.akd_gain(),
        }))
    }

    fn resolve_external_wem_bytes(&self, ext: &ExternalSourceDesc) -> Result<Vec<u8>, String> {
        if let Some(b) = &ext.bytes {
            return Ok(b.clone());
        }
        if let Some(raw_path) = &ext.path {
            let trimmed = raw_path.trim().trim_matches('\0');
            if !trimmed.is_empty() {
                if std::path::Path::new(trimmed).is_file() {
                    if let Ok(b) = std::fs::read(trimmed) {
                        return Ok(b);
                    }
                }
                let mut norm = trimmed.replace('\\', "/").to_ascii_lowercase();
                while norm.starts_with('/') {
                    norm.remove(0);
                }
                if !norm.ends_with(".wem") {
                    norm.push_str(".wem");
                }
                if std::path::Path::new(&norm).is_file() {
                    if let Ok(b) = std::fs::read(&norm) {
                        return Ok(b);
                    }
                }
                if let Some(ga) = self.project.game.as_ref() {
                    for vpath in ga.external_candidates(&norm) {
                        if let Ok(b) = ga.read(&vpath) {
                            return Ok(b);
                        }
                    }
                }
            }
        }
        if ext.file_id != 0 {
            if let Some(ga) = self.project.game.as_ref() {
                let vpath = ga.media_vpath(ext.file_id);
                if let Ok(b) = ga.read(&vpath) {
                    return Ok(b);
                }
            }
        }
        Err(format!(
            "external_source cookie={:#x} path={:?} file_id={} not found",
            ext.cookie, ext.path, ext.file_id
        ))
    }

    fn load_source_pcm(&mut self, src: &crate::bnk::hirc::Source) -> Result<Rc<Pcm>, String> {
        if src.plugin_id == 0x0008_0001 {
            let ext_opt = self
                .cur_ext_sources
                .iter()
                .find(|e| e.cookie == src.source_id || e.cookie == 0)
                .or_else(|| self.cur_ext_sources.first())
                .cloned();
            if let Some(ext) = ext_opt {
                let buf = self.resolve_external_wem_bytes(&ext)?;
                return Self::decode_wem_to_pcm(&buf);
            }
            if let Some(p) = self.pcm_cache.get(&src.source_id) {
                return Ok(p.clone());
            }
            return Err("external_source (dialogue, path supplied by the game at PostEvent)".into());
        }
        if let Some(p) = self.pcm_cache.get(&src.source_id) {
            return Ok(p.clone());
        }
        if src.plugin_type() != 1 {
            return Err(format!("source plugin {} ({:#x})", crate::bnk::plugin_name(src.plugin_id), src.plugin_id));
        }
        let loc = self.project.locate_media(src).ok_or_else(|| format!("media {} not found", src.source_id))?;
        let buf = self.project.read_media(&loc)?;
        let p = Self::decode_wem_to_pcm(&buf)?;
        self.pcm_cache.insert(src.source_id, p.clone());
        Ok(p)
    }

    fn start_sound(&mut self, id: u32, at: u64, fade: Option<Fade>) -> Result<Option<u64>, String> {
        if self.voices.len() >= 20_000 {
            return Err("more than 20000 voices scheduled".into());
        }
        let params = resolve(self.project, &self.gs, id, &mut self.rng)?;
        for n in &params.notes {
            self.notes.insert(n.clone());
        }
        let pcm = match self.load_pcm(id) {
            Ok(p) => p,
            Err(e) => {
                self.note(format!("unsupported_source: {}", e));
                return Ok(Some(at));
            }
        };
        let o = self.project.node(id).unwrap();
        let HircBody::Sound(s) = &o.body else { unreachable!() };
        // Loop prop (int): absent = play once, 0 = infinite, N = N plays [V: data scan].
        let loops = s.node.props.get_raw(prop::LOOP).unwrap_or(1);
        if s.node.ranged.0.iter().any(|(i, _, _)| *i == prop::LOOP) {
            self.note("random_loop");
        }
        let n = pcm.frames();
        let (ls, le) = match pcm.loop_points {
            Some((a, b)) if loops != 1 => (a as usize, (b as usize).min(n)),
            _ => (0, n),
        };
        let mut segments = vec![(0usize, le)];
        if loops == 0 {
            segments.push((ls, le)); // repeated forever
            self.note("infinite_loop");
        } else {
            for _ in 1..loops {
                segments.push((ls, le));
            }
            if le < n {
                segments.push((le, n));
            }
        }
        if pcm.loop_points.is_some() && loops != 1 {
            self.note("smpl_loop");
        }
        let ch = pcm.data.len();
        let mut res = Resampler::new(ch, pcm.rate, self.opts.resampler_fma);
        res.set_pitch(params.pitch_cents, false);
        if params.pitch_cents != 0.0 {
            self.note("pitch");
        }
        let total_in: u64 = segments.iter().map(|(a, b)| (b - a) as u64).sum();
        // initial delays were applied per played node in play_node
        let start = at;
        let out_len = if res.mode == Mode::Bypass { total_in } else { Resampler::output_len(total_in, res.step) };
        let end = if loops == 0 { None } else { Some(start + out_len) };
        let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &params);
        let out_bus_vol_db = self
            .gs
            .obj_output_bus_volume_db
            .get(&(self.cur_game_obj, 0))
            .copied()
            .unwrap_or(0.0);
        let (folded_out_vol, folded_out_lpf, folded_out_hpf) = params.fold_passthrough_buses(self.opts.bus_fx);
        // the media's `akd ` gain and 3D ray dry attenuation multiply the voice gain (Wem::akd_gain)
        let base_lin = voice_gain(params.volume_db, params.makeup_db, folded_out_vol + out_bus_vol_db) * pcm.akd_gain * ray_dry;
        if pcm.akd_gain != 1.0 {
            self.note("akd_gain");
        }
        let conn_lpf = (params.lpf + folded_out_lpf).max(ray_lpf).clamp(0.0, 100.0);
        let conn_hpf = (params.hpf + folded_out_hpf).max(ray_hpf).clamp(0.0, 100.0);
        let filter = (conn_lpf > 0.0 || conn_hpf > 0.0).then(|| ConnectionFilter::new(conn_lpf, conn_hpf, ch, RATE));
        let mut chain = vec![];
        let mut nid = id;
        while nid != 0 && chain.len() < 64 {
            chain.push(nid);
            nid = self.project.node(nid).and_then(|o| o.node_base()).map_or(0, |n| n.direct_parent_id);
        }
        let mut f_in = fade;
        if let Some(f) = f_in.as_mut() {
            if f.len == 0 {
                f_in = None;
            }
        }
        self.log.push(format!(
            "t={} sound {} src {} {}ch {}Hz {} frames loops={} vol={:.3}dB makeup={:.3}dB pitch={:.1}c step={} -> end {:?} bus {:?}",
            start,
            id,
            s.source.source_id,
            ch,
            pcm.rate,
            n,
            loops,
            params.volume_db,
            params.makeup_db,
            params.pitch_cents,
            res.step,
            end,
            params.buses.iter().map(|b| (b.id, b.bus_volume_db)).collect::<Vec<_>>()
        ));
        let node_fx: Vec<(u8, FxInstance)> = params
            .node_fx
            .iter()
            .filter_map(|&(slot, fx_id, _)| FxInstance::from_project(self.project, fx_id).map(|inst| (slot, inst)))
            .collect();
        self.voices.push(Voice {
            sound_id: id,
            playing_id: self.cur_playing_id,
            event_id: self.cur_event_id,
            game_object_id: self.cur_game_obj,
            chain,
            params,
            pcm,
            start,
            state: VState::Scheduled,
            evicted_by_limit: false,
            segments,
            seg: 0,
            seg_pos: 0,
            res,
            base_lin,
            ray_user_aux,
            ray_game_aux,
            ray_lpf,
            ray_hpf,
            prev_gain: None,
            aux_prev_gain: HashMap::new(),
            aux_filters: HashMap::new(),
            node_fx,
            fade_in: f_in,
            fade_out: None,
            stop_at: None,
            end,
            matrix,
            filter,
            pause_fade: None,
            clip_origin_sample: 0,
            clip_automations: Vec::new(),
        });
        Ok(end)
    }

    /// Layout of a bus: its own channel config, else its parent's ("same as parent"), the
    /// master bus falling back to the device layout (`out_channels`, stereo) [A].
    fn bus_layout(&self, buses: &[super::params::BusLink], i: usize) -> Layout {
        for b in &buses[i..] {
            if let Some(l) = Layout::from_config(b.channel_config) {
                return l;
            }
        }
        self.device_layout()
    }

    fn device_layout(&self) -> Layout {
        match self.opts.out_channels {
            1 => Layout::MONO,
            _ => Layout::STEREO,
        }
    }

    /// Voice -> output bus matrix [in][out], plus 3D ray dry/user_aux/game_aux gains and LPF/HPF.
    fn spatial_eval(&mut self, pcm: &Pcm, vp: &VoiceParams) -> (Vec<Vec<f32>>, f32, f32, f32, f32, f32) {
        if vp.positioning.panner_type() != 0 {
            // balance-fade at centre equals direct assignment only for stereo outputs [V §1.2]
            self.note(format!("panner_type_{}", vp.positioning.panner_type()));
        }
        if self.opts.out_channels != 2 {
            self.note("non_stereo_output");
        }
        let out = self.bus_layout(&vp.buses, 0);
        if pcm.layout.channels > 2 {
            self.note(format!("multichannel_{}ch_{:#x}", pcm.layout.channels, pcm.layout.mask));
        }
        let (m2d, notes) = direct_matrix(pcm.layout, out, vp.pan.2 / 100.0);
        for n in notes {
            self.note(n);
        }
        let Some(emitter) = self.gs.emitter.as_ref() else {
            return (m2d, 1.0, 1.0, 1.0, 0.0, 0.0);
        };
        if !vp.is_3d() {
            return (m2d, 1.0, 1.0, 1.0, 0.0, 0.0);
        }
        let mut ray = ray_geometry(&self.gs.listener, self.gs.listener_scaling, emitter, self.gs.emitter_scaling);
        ray.occlusion = self.gs.occlusion;
        ray.obstruction = self.gs.obstruction;
        ray.diffraction = self.gs.diffraction;
        ray.transmission = self.gs.transmission;
        let att = if vp.positioning.enable_attenuation() && vp.attenuation_id != 0 {
            self.project.attenuation(vp.attenuation_id).and_then(|o| match &o.body {
                HircBody::Attenuation(a) => Some(AttenModel::new(a)),
                _ => None,
            })
        } else {
            None
        };
        let proj = ProjectCurves::from_env(self.project.env_curves());
        let cone_ovr = ConeOverride { volume: vp.cone_volume, lpf: vp.cone_lpf, hpf: vp.cone_hpf };
        ray_gains(&mut ray, att.as_ref(), &proj, cone_ovr, true);
        let spread = att.as_ref().and_then(|a| a.eval_slot(atten_slot::SPREAD, ray.distance)).unwrap_or(0.0) * 0.01;
        let focus = att.as_ref().and_then(|a| a.eval_slot(atten_slot::FOCUS, ray.distance)).unwrap_or(0.0) * 0.01;
        let ray3d = Ray3d {
            front: ray.front_l,
            top: ray.top_l,
            pos: ray.pos_l,
            spread: spread.clamp(0.0, 1.0),
            focus: focus.clamp(0.0, 1.0),
            mask: !0,
        };
        let pp = PanParams {
            center: vp.pan.2 * 0.01,
            pan_lr: vp.pan.0,
            pan_bf: vp.pan.1,
            pan_du: 0.0,
            spat_mix: (vp.spat_mix * 0.01).clamp(0.0, 1.0),
            spat_mode: vp.positioning.spatialization_mode(),
            panner_type: vp.positioning.panner_type(),
            height_spread: att.as_ref().is_some_and(|a| a.height_spread),
        };
        let setup = SpeakerSetup::aw2();
        let in_cfg = Cfg { n: pcm.layout.channels as u32, kind: pcm.layout.kind as u32, mask: pcm.layout.mask };
        let out_cfg = Cfg { n: out.channels as u32, kind: out.kind as u32, mask: out.mask };
        let m3d = positioning_matrix(&setup, &ray3d, &pp, in_cfg, out_cfg, Some(&m2d));
        (m3d, ray.dry, ray.user_aux, ray.game_aux, ray.lpf, ray.hpf)
    }

    /// Transition fade ratio (0..1) for the frame ending at `t`, modeling Wwise's single active
    /// `CAkTransition` handoff (`FUN_1415ada30` / `CAkTransitionManager::ChangeParameter`) [V].
    ///
    /// Precedence when a newer fade begins while an earlier fade is still running:
    /// - Once `fade_out` starts (`fo.elapsed(t) > 0`), it drives the envelope from its captured
    ///   starting ratio to `0.0` (superseding `fade_in` and `pause_fade`).
    /// - Otherwise, once `pause_fade` starts (`pf.elapsed(t) > 0`), it drives the envelope from
    ///   its captured starting ratio to `0.0` (pause) or `1.0` (resume), superseding `fade_in`.
    /// - Otherwise, `fade_in` drives `0.0 -> 1.0`.
    fn fade_ratio(v: &Voice, t: u64) -> f32 {
        if let Some((fo, from)) = v.fade_out {
            if fo.elapsed(t) > 0 {
                return fo.value(from, 0.0, t);
            }
        }
        if let Some((pf, from, is_pause)) = v.pause_fade {
            if pf.elapsed(t) > 0 {
                return pf.value(from, if is_pause { 0.0 } else { 1.0 }, t);
            }
        }
        if let Some(fi) = v.fade_in {
            return fi.value(0.0, 1.0, t);
        }
        1.0
    }

    /// Play/stop/pause fade ratio plus MusicTrack `ClipAutomation` gain for the frame ending at `t`.
    fn envelope(v: &Voice, t: u64) -> f32 {
        let mut g = Self::fade_ratio(v, t);
        if !v.clip_automations.is_empty() {
            let (auto_gain, _, _) = super::music::eval_clip_automations(
                &v.clip_automations,
                v.clip_origin_sample,
                t,
            );
            g *= auto_gain;
        }
        if let Some(s) = v.stop_at {
            if t >= s {
                g = 0.0;
            }
        }
        g
    }

    fn compute_bus_order(&mut self) {
        if self.voices.len() == self.bus_voices_seen {
            return;
        }
        self.bus_voices_seen = self.voices.len();
        let mut seen: HashMap<u32, BusNode> = HashMap::new();
        let device = self.device_layout();
        let mut notes = vec![];
        let mut chains: Vec<Vec<super::params::BusLink>> = Vec::new();
        for v in &self.voices {
            chains.push(v.params.buses.clone());
            for &(aux_id, _, _, _) in &v.params.user_aux_sends {
                if let Ok(ch) = super::params::resolve_bus_chain(self.project, &self.gs, aux_id) {
                    chains.push(ch);
                }
            }
            if v.params.use_game_aux {
                for &(aux_id, _) in &self.gs.game_aux_sends {
                    if let Ok(ch) = super::params::resolve_bus_chain(self.project, &self.gs, aux_id) {
                        chains.push(ch);
                    }
                }
            }
        }
        for buses in &chains {
            let n = buses.len();
            for (i, b) in buses.iter().enumerate() {
                if seen.contains_key(&b.id) {
                    continue;
                }
                let parent = buses.get(i + 1).map_or(0, |p| p.id);
                let layout = self.bus_layout(buses, i);
                let parent_layout = if parent == 0 { device } else { self.bus_layout(buses, i + 1) };
                if b.panner_type != 0 && (b.pan.0 != 0.0 || b.pan.1 != 0.0) {
                    notes.push(format!("bus_panner_type_{}", b.panner_type));
                }
                let (m, ns) = direct_matrix(layout, parent_layout, b.center_pct / 100.0);
                notes.extend(ns.iter().map(|s| format!("bus_{}", s)));
                let mix_node = b.mix_node(self.opts.bus_fx);
                // Per CAkBus::IsMixingBus + sub_1415ca360 / sub_1415c9610:
                // - Mixing nodes absorb `BusVolume`, `OutputBusVolume`, and `OutputBusLPF`/`HPF`
                //   of any immediately following pass-through ancestor buses up to the next mixing bus.
                // - Pass-through buses themselves do NOT apply gain or connection filters (their
                //   params were folded into their child voice or child mixing bus).
                let (out_vol_db, out_lpf, out_hpf) = if mix_node {
                    let mut ov = b.output_bus_volume_db;
                    let mut ol = b.out_lpf;
                    let mut oh = b.out_hpf;
                    for anc in &buses[i + 1..] {
                        if anc.mix_node(self.opts.bus_fx) {
                            break;
                        }
                        ov += anc.bus_volume_db + anc.output_bus_volume_db;
                        ol += anc.out_lpf;
                        oh += anc.out_hpf;
                    }
                    (ov, ol, oh)
                } else {
                    (0.0, 0.0, 0.0)
                };
                seen.insert(
                    b.id,
                    BusNode {
                        id: b.id,
                        parent,
                        depth: n - 1 - i,
                        layout,
                        to_parent: m,
                        gain_db: b.bus_volume_db,
                        out_vol_db,
                        mix_node,
                        out_lpf,
                        out_hpf,
                        fx: b.fx.clone(),
                        ducks: b.ducks.clone(),
                        recovery_time_ms: b.recovery_time_ms,
                        max_duck_volume: b.max_duck_volume,
                    },
                );
            }
        }
        for n in notes {
            self.note(n);
        }
        let mut order: Vec<BusNode> = seen.into_values().collect();
        order.sort_by(|a, b| b.depth.cmp(&a.depth).then(a.id.cmp(&b.id)));
        self.bus_order = order;
    }

    fn active(&self) -> bool {
        self.pending.iter().any(|p| self.pending_keeps_alive(p)) || self.voices.iter().any(|v| v.state != VState::Done)
    }

    /// AK launches a delayed action one buffer tick before the frame that contains its due
    /// sample (the same one-frame lead as the delayed play fade in `play_node`), and an event
    /// whose last voice has ended completes in the frame that runs its last pending action.
    /// So a pending Stop/Pause/Seek/... due in frame `k` keeps the render alive only through
    /// frame `k - 1` [V: oracle lengths of 1716566570 (Stop at 168000 -> 328 frames),
    /// 786641639 (67200 -> 131) and 3084055277 (110400 -> 215)]. Plays always keep it alive
    /// (they start voices). Their effect on playing voices is unchanged: the transition math
    /// (`Fade`) already gives 0 elapsed ticks in that lead frame.
    fn pending_keeps_alive(&self, p: &Pending) -> bool {
        let is_play = self.project.action(p.action).is_some_and(|o| matches!(&o.body, HircBody::Action(a) if a.kind == ActionKind::Play));
        is_play || p.at / FRAME as u64 > self.now / FRAME as u64
    }

    /// Enforce `project.volume_threshold_db` (`BelowThresholdBehavior`) and `MaxNumInstance`
    /// voice limiting (`AdvSettings` / `Bus::max_num_instance`) for voices active in `[f0, f1)`.
    fn enforce_voice_limits_and_thresholds(&mut self, f0: u64, f1: u64) {
        let thresh_db = self.project.volume_threshold_db;
        // 1. Activate scheduled voices starting in this frame and check volume threshold
        for v in self.voices.iter_mut() {
            if v.state == VState::Done || v.state == VState::Paused || v.start >= f1 {
                continue;
            }
            if let Some(s) = v.stop_at {
                if s <= f0 {
                    v.state = VState::Done;
                    continue;
                }
            }
            if v.state == VState::Scheduled {
                v.state = VState::Playing;
            }
            let btb = v.params.below_threshold_behavior;
            if btb != 0 {
                let eff_db = if v.base_lin <= 0.0 {
                    -96.3
                } else {
                    super::math::fast_pow_to_db(v.base_lin * v.base_lin)
                };
                if eff_db < thresh_db {
                    match btb {
                        1 => v.state = VState::Done, // KillVoice
                        2 => {
                            // SetAsVirtualVoice
                            if v.state != VState::Virtual {
                                v.state = VState::Virtual;
                                v.prev_gain = Some(0.0);
                                if v.params.virtual_queue_behavior == 0 {
                                    v.seg = 0;
                                    v.seg_pos = 0;
                                    v.res.reset();
                                }
                            }
                        }
                        3 => {
                            // KillIfOneShotElseVirtual
                            if v.end.is_some() {
                                v.state = VState::Done;
                            } else if v.state != VState::Virtual {
                                v.state = VState::Virtual;
                                v.prev_gain = Some(0.0);
                                if v.params.virtual_queue_behavior == 0 {
                                    v.seg = 0;
                                    v.seg_pos = 0;
                                    v.res.reset();
                                }
                            }
                        }
                        _ => {}
                    }
                } else if v.state == VState::Virtual && !v.evicted_by_limit {
                    // Volume rose back above threshold: wake from virtual
                    v.state = VState::Playing;
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                }
            }
        }

        // 2. Collect unique InstanceLimits across active voices
        let mut limits: Vec<super::params::InstanceLimit> = Vec::new();
        for v in &self.voices {
            if v.state == VState::Done || v.state == VState::Paused || v.start >= f1 {
                continue;
            }
            for &lim in &v.params.instance_limits {
                if !limits.iter().any(|l| l.scope_id == lim.scope_id && l.is_bus == lim.is_bus) {
                    limits.push(lim);
                }
            }
        }
        if limits.is_empty() {
            return;
        }

        // 3. Enforce each InstanceLimit (evicting lowest-priority or oldest/newest voices, or waking limit-evicted virtual voices)
        for lim in limits {
            let max_inst = lim.max_instances as usize;
            let in_scope = |v: &Voice| -> bool {
                if lim.is_bus {
                    v.params.buses.iter().any(|b| b.id == lim.scope_id)
                } else {
                    v.chain.contains(&lim.scope_id)
                }
            };
            loop {
                let playing_indices: Vec<usize> = self
                    .voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.state == VState::Playing && v.start < f1 && in_scope(v))
                    .map(|(i, _)| i)
                    .collect();
                if playing_indices.len() <= max_inst {
                    break;
                }
                // Pick victim: lowest effective priority, breaking ties by kill_newest (newest vs oldest)
                let mut victim = playing_indices[0];
                let mut victim_prio = self.voices[victim].params.effective_priority(self.project, &self.gs);
                for &idx in &playing_indices[1..] {
                    let prio = self.voices[idx].params.effective_priority(self.project, &self.gs);
                    let is_worse = if (prio - victim_prio).abs() > 1e-4 {
                        prio < victim_prio
                    } else if lim.kill_newest {
                        (self.voices[idx].start, idx) > (self.voices[victim].start, victim)
                    } else {
                        (self.voices[idx].start, idx) < (self.voices[victim].start, victim)
                    };
                    if is_worse {
                        victim = idx;
                        victim_prio = prio;
                    }
                }
                let v = &mut self.voices[victim];
                let btb = v.params.below_threshold_behavior;
                if lim.use_virtual_behavior && (btb == 0 || btb == 2 || (btb == 3 && v.end.is_none())) {
                    v.state = VState::Virtual;
                    v.evicted_by_limit = true;
                    v.prev_gain = Some(0.0);
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                } else {
                    v.state = VState::Done;
                }
            }
            // Wake limit-evicted virtual voices if a slot opened up
            loop {
                let playing_count = self
                    .voices
                    .iter()
                    .filter(|v| v.state == VState::Playing && v.start < f1 && in_scope(v))
                    .count();
                if playing_count >= max_inst {
                    break;
                }
                let candidate = self
                    .voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.state == VState::Virtual && v.evicted_by_limit && v.start < f1 && in_scope(v))
                    .max_by(|(ia, va), (ib, vb)| {
                        let pa = va.params.effective_priority(self.project, &self.gs);
                        let pb = vb.params.effective_priority(self.project, &self.gs);
                        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal).then_with(|| ib.cmp(ia))
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = candidate {
                    let v = &mut self.voices[idx];
                    v.state = VState::Playing;
                    v.evicted_by_limit = false;
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Render one frame into `out` (interleaved, `out_channels`).
    pub fn render_frame(&mut self, out: &mut Vec<f32>) -> Result<(), String> {
        let f0 = self.now;
        let f1 = f0 + FRAME as u64;
        // actions due in this frame (sample-accurate scheduling inside the frame [A])
        while let Some(p) = self.pending.first().cloned() {
            if p.at >= f1 {
                break;
            }
            self.pending.remove(0);
            self.run_action(&p)?;
        }
        self.enforce_voice_limits_and_thresholds(f0, f1);
        self.compute_bus_order();
        let oc = self.opts.out_channels;
        let mut bus_buf: HashMap<u32, Vec<Vec<f32>>> = HashMap::new();
        for bn in &self.bus_order {
            bus_buf.insert(bn.id, vec![vec![0.0; FRAME]; bn.layout.channels]);
        }
        let mut active_buses: BTreeSet<u32> = BTreeSet::new();
        let fma = self.opts.mixer_fma;
        let start_ramp = self.opts.start_ramp;
        let mut meter_updates: Vec<(u32, f32)> = Vec::new();
        for v in self.voices.iter_mut() {
            if v.state == VState::Done || v.state == VState::Paused || v.start >= f1 {
                continue;
            }
            if let Some(s) = v.stop_at {
                if s <= f0 {
                    v.state = VState::Done;
                    continue;
                }
            }
            let offset = v.start.saturating_sub(f0) as usize;
            let ch = v.pcm.data.len();
            if v.state == VState::Virtual {
                match v.params.virtual_queue_behavior {
                    1 => {
                        // FromElapsedTime: advance cursor silently without mixing
                        let mut scratch = vec![vec![0.0f32; FRAME]; ch];
                        let produced = pull(v, &mut scratch, offset, FRAME - offset);
                        if produced < FRAME - offset || v.end.is_some_and(|e| e <= f1) {
                            v.state = VState::Done;
                        }
                    }
                    2 => {
                        // Resume: freeze cursor while virtual and extend planned end by elapsed virtual frame
                        if let Some(e) = v.end.as_mut() {
                            *e += (FRAME - offset) as u64;
                        }
                    }
                    _ => {
                        // FromBeginning: cursor will rewind to 0 on wake; extend planned end while virtual
                        if let Some(e) = v.end.as_mut() {
                            *e += (FRAME - offset) as u64;
                        }
                    }
                }
                continue;
            }
            v.state = VState::Playing;
            for b in &v.params.buses {
                active_buses.insert(b.id);
            }
            let offset = v.start.saturating_sub(f0) as usize;
            let ch = v.pcm.data.len();
            let mut vbuf_raw = vec![vec![0.0f32; FRAME]; ch];
            let produced = pull(v, &mut vbuf_raw, offset, FRAME - offset);
            if !v.node_fx.is_empty() {
                let lfe_ch = (v.pcm.layout.mask & super::speakers::LFE != 0)
                    .then(|| ch.saturating_sub(1));
                for (_, fx_inst) in v.node_fx.iter_mut() {
                    if let Some(upd) = fx_inst.process(self.project, &self.gs, &mut vbuf_raw, lfe_ch) {
                        meter_updates.push(upd);
                    }
                }
            }
            if !v.clip_automations.is_empty() {
                let (_, auto_lpf, auto_hpf) = super::music::eval_clip_automations(
                    &v.clip_automations,
                    v.clip_origin_sample,
                    f1,
                );
                if auto_lpf > 0.0 || auto_hpf > 0.0 {
                    let eff_lpf = (v.params.lpf + auto_lpf).max(v.ray_lpf).clamp(0.0, 100.0);
                    let eff_hpf = (v.params.hpf + auto_hpf).max(v.ray_hpf).clamp(0.0, 100.0);
                    v.filter = Some(ConnectionFilter::new(eff_lpf, eff_hpf, ch, RATE));
                }
            }
            let has_aux = !v.params.user_aux_sends.is_empty() || (v.params.use_game_aux && !self.gs.game_aux_sends.is_empty());
            let mut vbuf = if has_aux && v.filter.is_some() { vbuf_raw.clone() } else { std::mem::take(&mut vbuf_raw) };
            if let Some(flt) = v.filter.as_mut() {
                for (c, ch_buf) in vbuf.iter_mut().enumerate() {
                    flt.process(c, ch_buf);
                }
            }
            if has_aux && vbuf_raw.is_empty() {
                vbuf_raw = vbuf.clone();
            }
            // the source also ends when its last sample lands exactly on the frame boundary: AK
            // flags NoMoreData with that full frame and releases the voice (no extra frame) [V:
            // oracle lengths, e.g. event 1664992110 ends at 2525*512]
            let finished = produced < FRAME - offset || v.end.is_some_and(|e| e <= f1);
            let env_f0 = Renderer::envelope(v, f0);
            let env_f1 = Renderer::envelope(v, f1);
            // natural end inside the frame: no ramp in the oracle (see `RenderOptions::end_ramp`)
            let nominal = v.base_lin * env_f1;
            let target = if finished && self.opts.end_ramp { 0.0 } else { nominal };
            let prev = match v.prev_gain {
                Some(p) => p,
                None if start_ramp => 0.0,
                // first frame: the fade ratio's previous value (0 for a fade-in) [V: 3586369275
                // frame 0 ramps 0 -> interp(1/N)]
                None => v.base_lin * env_f0,
            };
            // pause: once the pause fade completed, hold
            if let Some((f, _, true)) = v.pause_fade {
                if f1 >= f.end() {
                    v.state = VState::Paused;
                    v.prev_gain = Some(0.0);
                }
            }
            let bus = v.params.buses.first().map_or(0, |b| b.id);
            if let Some(bb) = bus_buf.get_mut(&bus) {
                // AkMixer: per (in, out) pair, gain ramps from prevMx*prevGain to nextMx*nextGain;
                // the matrix itself does not ramp (prev == next, ADVICE_A3 §2.1)
                for (ic, row) in v.matrix.iter().enumerate() {
                    for (o, &m) in row.iter().enumerate() {
                        if o < bb.len() {
                            mix_channel(&vbuf[ic], &mut bb[o], m * prev, m * target, fma);
                        }
                    }
                }
            }
            // Route auxiliary sends (user-defined 0..3 and game-defined) to their AuxBus buffers
            if has_aux {
                let voice_pre_dry = db_to_lin(v.params.volume_db) * db_to_lin(v.params.makeup_db) * v.pcm.akd_gain;
                let mut sends: Vec<(u32, f32, f32, f32, bool)> = v
                    .params
                    .user_aux_sends
                    .iter()
                    .map(|&(aid, vol, lpf, hpf)| (aid, vol, lpf, hpf, false))
                    .collect();
                if v.params.use_game_aux {
                    for &(aid, env_vol) in &self.gs.game_aux_sends {
                        sends.push((
                            aid,
                            env_vol + v.params.game_aux_volume_db,
                            v.params.game_aux_lpf,
                            v.params.game_aux_hpf,
                            true,
                        ));
                    }
                }
                for (aux_id, send_vol_db, send_lpf, send_hpf, is_game) in sends {
                    let Some(abuf) = bus_buf.get_mut(&aux_id) else { continue };
                    active_buses.insert(aux_id);
                    let ray_aux = if is_game { v.ray_game_aux } else { v.ray_user_aux };
                    let send_base = voice_pre_dry * db_to_lin(send_vol_db) * ray_aux;
                    let send_nom = send_base * env_f1;
                    let send_tgt = if finished && self.opts.end_ramp { 0.0 } else { send_nom };
                    let send_prv = match v.aux_prev_gain.get(&aux_id).copied() {
                        Some(p) => p,
                        None if start_ramp => 0.0,
                        None => send_base * env_f0,
                    };
                    let eff_lpf = send_lpf.max(v.ray_lpf).clamp(0.0, 100.0);
                    let eff_hpf = send_hpf.max(v.ray_hpf).clamp(0.0, 100.0);
                    let mut sbuf;
                    let src_buf = if eff_lpf > 0.0 || eff_hpf > 0.0 {
                        sbuf = vbuf_raw.clone();
                        let flt = v
                            .aux_filters
                            .entry(aux_id)
                            .or_insert_with(|| ConnectionFilter::new(eff_lpf, eff_hpf, ch, RATE));
                        for (c, ch_buf) in sbuf.iter_mut().enumerate() {
                            flt.process(c, ch_buf);
                        }
                        &sbuf
                    } else {
                        &vbuf_raw
                    };
                    for (ic, row) in v.matrix.iter().enumerate() {
                        for (o, &m) in row.iter().enumerate() {
                            if o < abuf.len() {
                                mix_channel(&src_buf[ic], &mut abuf[o], m * send_prv, m * send_tgt, fma);
                            }
                        }
                    }
                    if v.state != VState::Paused {
                        v.aux_prev_gain.insert(aux_id, send_tgt);
                    }
                }
            }
            if v.state != VState::Paused {
                v.prev_gain = Some(target);
            }
            if finished || target == 0.0 && v.stop_at.is_some_and(|s| s <= f1) {
                v.state = VState::Done;
            }
            if v.fade_out.is_some_and(|(f, _)| f1 >= f.end()) {
                v.state = VState::Done;
            }
        }
        // Update bus auto-ducking states (`DuckInfo` on buses)
        for bn in &self.bus_order {
            if bn.ducks.is_empty() {
                continue;
            }
            let is_active = active_buses.contains(&bn.id);
            for d in &bn.ducks {
                let st = self.duck_states.entry((bn.id, d.bus_id)).or_insert(DuckState {
                    active: false,
                    current_db: 0.0,
                    from_db: 0.0,
                    to_db: 0.0,
                    trans_start: 0,
                    trans_len: 0,
                    curve: d.fade_curve as u32,
                });
                if is_active && !st.active {
                    st.active = true;
                    st.from_db = st.current_db;
                    st.to_db = d.duck_volume;
                    st.trans_start = f0;
                    st.trans_len = ms_to_samples(d.fade_out_ms.max(0) as f32);
                    st.curve = d.fade_curve as u32;
                } else if !is_active && st.active {
                    st.active = false;
                    st.from_db = st.current_db;
                    st.to_db = 0.0;
                    st.trans_start = f0 + ms_to_samples(bn.recovery_time_ms.max(0) as f32);
                    st.trans_len = ms_to_samples(d.fade_in_ms.max(0) as f32);
                    st.curve = d.fade_curve as u32;
                }
                if f1 <= st.trans_start {
                    st.current_db = st.from_db;
                } else if st.trans_len == 0 {
                    st.current_db = st.to_db;
                } else {
                    let fade = Fade { start: st.trans_start, len: st.trans_len, curve: st.curve };
                    st.current_db = fade.value(st.from_db, st.to_db, f1);
                }
            }
        }
        // bus mix, children before parents: bus -> parent with direct-assignment matrix x bus volume
        let mut master = vec![vec![0.0f32; FRAME]; oc];
        for bn in &self.bus_order {
            let mut buf = bus_buf.remove(&bn.id).unwrap();
            if self.opts.bus_fx && !bn.fx.is_empty() {
                let lfe_ch = (bn.layout.mask & super::speakers::LFE != 0)
                    .then(|| buf.len().saturating_sub(1));
                for &(slot, fx_id, _) in &bn.fx {
                    let key = (bn.id, slot);
                    let needs_init = self
                        .bus_fx_instances
                        .get(&key)
                        .map_or(true, |inst| inst.fx_id != fx_id);
                    if needs_init {
                        if let Some(inst) = FxInstance::from_project(self.project, fx_id) {
                            self.bus_fx_instances.insert(key, inst);
                        }
                    }
                    if let Some(inst) = self.bus_fx_instances.get_mut(&key) {
                        if let Some(upd) = inst.process(self.project, &self.gs, &mut buf, lfe_ch) {
                            meter_updates.push(upd);
                        }
                    }
                }
            }
            if bn.out_lpf != 0.0 || bn.out_hpf != 0.0 {
                // the connection filters each bus channel before the gain/matrix mix
                // (`FUN_14163dbc0`: LPF, HPF, then MixOneInNChannels) [V]
                let f = self.bus_filters.entry(bn.id).or_insert_with(|| ConnectionFilter::new(bn.out_lpf, bn.out_hpf, buf.len(), RATE));
                for (c, ch) in buf.iter_mut().enumerate() {
                    f.process(c, ch);
                }
            }
            let mut duck_db: f32 = self
                .duck_states
                .iter()
                .filter(|((_, target_id), _)| *target_id == bn.id)
                .map(|(_, st)| st.current_db)
                .sum();
            if duck_db != 0.0 && bn.max_duck_volume < 0.0 {
                duck_db = duck_db.max(bn.max_duck_volume);
            }
            let mut g = bus_gain(bn.gain_db, bn.out_vol_db, bn.mix_node, bn.parent == 0);
            if duck_db != 0.0 {
                g *= db_to_lin(duck_db);
            }
            let gp = *self.bus_prev_gain.entry(bn.id).or_insert(g);
            self.bus_prev_gain.insert(bn.id, g);
            let dst = if bn.parent == 0 { &mut master } else { bus_buf.get_mut(&bn.parent).unwrap() };
            for (ic, row) in bn.to_parent.iter().enumerate() {
                for (o, &m) in row.iter().enumerate() {
                    if o < dst.len() {
                        mix_channel(&buf[ic], &mut dst[o], m * gp, m * g, fma);
                    }
                }
            }
        }
        if !meter_updates.is_empty() {
            let mut changed = false;
            for (param_id, val) in meter_updates {
                let prev = self.gs.params.get(&param_id).copied();
                if prev.map_or(true, |p| (p - val).abs() > 1e-4) {
                    self.gs.set_rtpc(param_id, val);
                    changed = true;
                }
            }
            if changed {
                self.refresh_voices_rtpc();
            }
        }
        for n in 0..FRAME {
            for c in 0..oc {
                out.push(master[c][n]);
            }
        }
        self.now = f1;
        Ok(())
    }

    /// Render until every voice has finished (or `max_seconds`).
    pub fn render(&mut self) -> Result<Vec<f32>, String> {
        let mut out = vec![];
        let horizon = self.horizon();
        while self.active() {
            if self.now >= horizon {
                self.note("truncated_at_max_seconds");
                break;
            }
            self.render_frame(&mut out)?;
        }
        for _ in 0..self.opts.tail_frames {
            self.render_frame(&mut out)?;
        }
        Ok(out)
    }

    /// Sample at which the last voice ends (for length checks); None if any voice is infinite.
    pub fn planned_end(&self) -> Option<u64> {
        let mut e = 0;
        for v in &self.voices {
            e = e.max(v.end?);
        }
        Some(e)
    }

    pub fn voice_count(&self) -> usize {
        self.voices.len()
    }

    /// Current rendered sample position (`now`).
    pub fn now(&self) -> u64 {
        self.now
    }

    /// Whether any voices or pending actions are active.
    pub fn is_active(&self) -> bool {
        self.active()
    }

    /// Reseed the internal Wwise PRNG (`AK::SoundEngine::SetRandomSeed`).
    pub fn set_random_seed(&mut self, seed: u32) {
        self.opts.seed = seed;
        self.rng = Rng::new(seed);
    }

    /// Mark bus order dirty after bank loads/unloads or bus changes.
    pub fn invalidate_bus_order(&mut self) {
        self.bus_voices_seen = usize::MAX;
    }

    /// Whether a specific `playing_id` still has active voices, pending actions, or active music segments.
    pub fn is_playing_id_active(&self, playing_id: u32) -> bool {
        if playing_id == 0 {
            return false;
        }
        if self.pending.iter().any(|p| p.playing_id == playing_id && self.pending_keeps_alive(p)) {
            return true;
        }
        if self.voices.iter().any(|v| v.playing_id == playing_id && v.state != VState::Done) {
            return true;
        }
        if self.active_music_segments.iter().any(|s| s.playing_id == playing_id && self.now < s.end_sample) {
            return true;
        }
        false
    }

    /// Stop all voices and pending actions for `playing_id` (`AK::SoundEngine::StopPlayingID`).
    pub fn stop_playing_id(&mut self, playing_id: u32, transition_ms: i32, curve: u32) {
        if playing_id == 0 {
            return;
        }
        let at = self.now;
        let len = ms_to_samples(transition_ms.max(0) as f32);
        self.pending.retain(|p| p.playing_id != playing_id);
        for v in self.voices.iter_mut() {
            if v.state != VState::Done && v.playing_id == playing_id {
                stop_voice(v, at, len, curve);
            }
        }
        if len == 0 {
            self.active_music_segments.retain(|s| s.playing_id != playing_id);
            self.active_music_switches.retain(|s| s.playing_id != playing_id);
            self.active_switch_cntrs.retain(|s| s.playing_id != playing_id);
        }
    }

    /// Stop all voices and pending actions globally (`game_object_id.is_none()`) or for a single
    /// game object (`AK::SoundEngine::StopAll`).
    pub fn stop_all(&mut self, game_object_id: Option<u64>) {
        let match_obj = |obj: u64| match game_object_id {
            None | Some(u64::MAX) => true,
            Some(target) => obj == target,
        };
        self.pending.retain(|p| !match_obj(p.game_object_id));
        for v in self.voices.iter_mut() {
            if v.state != VState::Done && match_obj(v.game_object_id) {
                v.state = VState::Done;
            }
        }
        self.active_music_segments.retain(|s| !match_obj(s.game_object_id));
        if game_object_id.is_none() || game_object_id == Some(u64::MAX) {
            self.active_music_switches.clear();
            self.active_switch_cntrs.clear();
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
        let at = self.now;
        let len = ms_to_samples(transition_ms.max(0) as f32);
        let match_obj = |obj: u64| match game_object_id {
            None | Some(u64::MAX) | Some(0) => true,
            Some(target) => obj == target || obj == 0,
        };
        let match_pid = |pid: u32| playing_id == 0 || pid == playing_id;
        if action_type == 0 {
            self.pending.retain(|p| !(p.event_id == event_id && match_obj(p.game_object_id) && match_pid(p.playing_id)));
        }
        for v in self.voices.iter_mut() {
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
        for v in self.voices.iter_mut() {
            if v.state == VState::Done || v.event_id != event_id || !match_obj(v.game_object_id) || !match_pid(v.playing_id) {
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
        for v in &self.voices {
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
        let v = self.voices.iter().find(|v| v.playing_id == playing_id)?;
        let dur_samples = v.end.map(|e| e.saturating_sub(v.start)).unwrap_or_else(|| v.pcm.frames() as u64);
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
        self.active_music_segments
            .iter()
            .filter(|s| s.playing_id == playing_id)
            .cloned()
            .collect()
    }

    /// Active playing IDs on `game_object_id` (`AK::SoundEngine::Query::GetPlayingIDsFromGameObject`).
    pub fn playing_ids_for_game_object(&self, game_object_id: u64) -> Vec<u32> {
        let mut pids = BTreeSet::new();
        for p in &self.pending {
            if p.game_object_id == game_object_id && p.playing_id != 0 && self.pending_keeps_alive(p) {
                pids.insert(p.playing_id);
            }
        }
        for v in &self.voices {
            if v.game_object_id == game_object_id && v.playing_id != 0 && v.state != VState::Done {
                pids.insert(v.playing_id);
            }
        }
        for s in &self.active_music_segments {
            if s.game_object_id == game_object_id && s.playing_id != 0 && self.now < s.end_sample {
                pids.insert(s.playing_id);
            }
        }
        pids.into_iter().collect()
    }
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
    stinger_last_fired: HashMap<(u32, u32), u64>,
    pub notes: BTreeSet<String>,
    pub log: Vec<String>,
    bus_order: Vec<BusNode>,
    bus_voices_seen: usize,
    bus_prev_gain: HashMap<u32, f32>,
    bus_filters: HashMap<u32, ConnectionFilter>,
    bus_fx_instances: HashMap<(u32, u8), FxInstance>,
    duck_states: HashMap<(u32, u32), DuckState>,
    cur_playing_id: u32,
    cur_event_id: u32,
    cur_game_obj: u64,
    cur_ext_sources: Vec<ExternalSourceDesc>,
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
            gs: std::mem::take(&mut self.gs),
            opts: self.opts.clone(),
            rng: std::mem::replace(&mut self.rng, Rng::new(0)),
            now: self.now,
            pending: std::mem::take(&mut self.pending),
            voices: std::mem::take(&mut self.voices),
            pcm_cache: std::mem::take(&mut self.pcm_cache),
            cntr: std::mem::take(&mut self.cntr),
            switch_played_once: std::mem::take(&mut self.switch_played_once),
            active_switch_cntrs: std::mem::take(&mut self.active_switch_cntrs),
            music_track_seq: std::mem::take(&mut self.music_track_seq),
            music_playlists: std::mem::take(&mut self.music_playlists),
            active_music_switches: std::mem::take(&mut self.active_music_switches),
            active_music_segments: std::mem::take(&mut self.active_music_segments),
            stinger_last_fired: std::mem::take(&mut self.stinger_last_fired),
            notes: std::mem::take(&mut self.notes),
            log: std::mem::take(&mut self.log),
            bus_order: std::mem::take(&mut self.bus_order),
            bus_voices_seen: self.bus_voices_seen,
            bus_prev_gain: std::mem::take(&mut self.bus_prev_gain),
            bus_filters: std::mem::take(&mut self.bus_filters),
            bus_fx_instances: std::mem::take(&mut self.bus_fx_instances),
            duck_states: std::mem::take(&mut self.duck_states),
            cur_playing_id: self.cur_playing_id,
            cur_event_id: self.cur_event_id,
            cur_game_obj: self.cur_game_obj,
            cur_ext_sources: std::mem::take(&mut self.cur_ext_sources),
        };
        let out = f(&mut r);
        self.gs = r.gs;
        self.opts = r.opts;
        self.rng = r.rng;
        self.now = r.now;
        self.pending = r.pending;
        self.voices = r.voices;
        self.pcm_cache = r.pcm_cache;
        self.cntr = r.cntr;
        self.switch_played_once = r.switch_played_once;
        self.active_switch_cntrs = r.active_switch_cntrs;
        self.music_track_seq = r.music_track_seq;
        self.music_playlists = r.music_playlists;
        self.active_music_switches = r.active_music_switches;
        self.active_music_segments = r.active_music_segments;
        self.stinger_last_fired = r.stinger_last_fired;
        self.notes = r.notes;
        self.log = r.log;
        self.bus_order = r.bus_order;
        self.bus_voices_seen = r.bus_voices_seen;
        self.bus_prev_gain = r.bus_prev_gain;
        self.bus_filters = r.bus_filters;
        self.bus_fx_instances = r.bus_fx_instances;
        self.duck_states = r.duck_states;
        self.cur_playing_id = r.cur_playing_id;
        self.cur_event_id = r.cur_event_id;
        self.cur_game_obj = r.cur_game_obj;
        self.cur_ext_sources = r.cur_ext_sources;
        out
    }
}

fn ms_to_samples(ms: f32) -> u64 {
    // AK converts transition/delay times to samples at the engine rate [A: rounding]
    ((ms.max(0.0) as f64) * RATE as f64 / 1000.0).round() as u64
}

/// Schedule a fade-out on `v`, capturing the voice's active fade ratio at the frame boundary
/// where `f` starts so mid-fade-in, mid-pause, or retargeted fade-out stops hand off continuously
/// (`CAkTransitionManager::ChangeParameter`, `FUN_1415ada30` [V]).
fn start_fade_out(v: &mut Voice, f: Fade) {
    let prev_end = (f.start / FRAME as u64) * FRAME as u64;
    let from = if v.state == VState::Paused {
        0.0
    } else {
        Renderer::fade_ratio(v, prev_end)
    };
    v.fade_out = Some((f, from.clamp(0.0, 1.0)));
}

fn stop_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    if (v.state == VState::Scheduled && v.start >= at) || v.state == VState::Paused {
        v.state = VState::Done;
        return;
    }
    // 0 ms stop: the transition (0 ticks) is done at its first update, so the voice ramps to 0
    // over the action's frame via the mixer [V: FUN_1415ad0e0; ADVICE_A3 §2.2]
    let f = Fade { start: at, len, curve };
    start_fade_out(v, f);
    v.stop_at = Some(f.end());
}

/// Schedule a pause fade-out on `v`, capturing its active fade ratio at the action's frame
/// boundary so pausing mid-fade-in or mid-resume hands off smoothly without jumping to 1.0.
fn pause_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    let prev_end = (at / FRAME as u64) * FRAME as u64;
    let from = Renderer::fade_ratio(v, prev_end).clamp(0.0, 1.0);
    v.pause_fade = Some((Fade { start: at, len: len.max(FRAME as u64), curve }, from, true));
}

/// Schedule a resume fade-in on `v`, handling both fully paused voices (`from = 0.0`) and voices
/// resumed while a pause fade-out is still running (`from = current pause fade ratio`).
fn resume_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    if v.state == VState::Paused {
        v.state = VState::Playing;
        v.pause_fade = Some((Fade { start: at, len, curve }, 0.0, false));
    } else if v.state == VState::Playing && v.pause_fade.is_some_and(|(_, _, is_pause)| is_pause) {
        let prev_end = (at / FRAME as u64) * FRAME as u64;
        let from = Renderer::fade_ratio(v, prev_end).clamp(0.0, 1.0);
        v.pause_fade = Some((Fade { start: at, len, curve }, from, false));
    }
}

fn seek_voice(v: &mut Voice, pos: usize) {
    // place the cursor in the first segment that contains `pos` (source frames from the start)
    let mut acc = 0usize;
    for (i, (a, b)) in v.segments.iter().enumerate() {
        let len = b - a;
        if pos < acc + len {
            v.seg = i;
            v.seg_pos = pos - acc;
            return;
        }
        acc += len;
    }
    v.seg = v.segments.len();
}

/// Pull `n` output frames of the voice into `buf[c][offset..]`, returns frames produced.
fn pull(v: &mut Voice, buf: &mut [Vec<f32>], offset: usize, n: usize) -> usize {
    let mut produced = 0;
    let infinite_last = v.end.is_none();
    while produced < n {
        if v.seg >= v.segments.len() {
            if infinite_last && !v.segments.is_empty() {
                v.seg = v.segments.len() - 1; // loop the last (loop) segment forever
                v.seg_pos = 0;
            } else {
                break;
            }
        }
        let (a, b) = v.segments[v.seg];
        let s = a + v.seg_pos;
        if s >= b {
            v.seg += 1;
            v.seg_pos = 0;
            continue;
        }
        let input: Vec<&[f32]> = v.pcm.data.iter().map(|c| &c[s..b]).collect();
        let (consumed, p) = v.res.process(&input, buf, offset + produced, n - produced);
        v.seg_pos += consumed;
        produced += p;
        if consumed == 0 && p == 0 {
            break;
        }
    }
    produced
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnk::hirc::{Action, Bus, Event, HircObject, NodeBase, Props, RangedProps, Sound, Source, SwitchCntr, SwitchNodeParams};

    fn make_sound(id: u32, parent: u32, bus: u32, src_id: u32, vol_db: f32) -> HircObject {
        HircObject {
            hirc_type: 0x02,
            id,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: src_id,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: bus,
                    direct_parent_id: parent,
                    props: Props(vec![(prop::VOLUME, vol_db.to_bits())]),
                    ..Default::default()
                },
            }),
        }
    }

    #[test]
    fn switch_cntr_continuous_validation_and_set_switch() {
        let mut p = Project::new(None);
        // Master bus 1
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });
        // Sounds 101 (switch A), 102 (switch B), 103 (shared with bContinuePlayback)
        p.insert_hirc(make_sound(101, 100, 0, 1001, 0.0));
        p.insert_hirc(make_sound(102, 100, 0, 1002, 0.0));
        p.insert_hirc(make_sound(103, 100, 0, 1003, 0.0));
        // SwitchCntr 100: continuous validation, switch group 50, default switch 1
        p.insert_hirc(HircObject {
            hirc_type: 0x06,
            id: 100,
            offset: 0,
            size: 0,
            body: HircBody::SwitchCntr(SwitchCntr {
                node: NodeBase { override_bus_id: 1, ..Default::default() },
                group_type: 0,
                group_id: 50,
                default_switch: 1,
                continuous_validation: true,
                children: vec![101, 102, 103],
                switches: vec![(1, vec![101, 103]), (2, vec![102, 103])],
                params: vec![
                    // 101: stop on switch (on_switch_mode = 1), bIsFirstOnly = 1 (bits = 1)
                    SwitchNodeParams { node_id: 101, bits: 1, on_switch_mode: 1, fade_out_ms: 0, fade_in_ms: 0 },
                    // 102: stop on switch (on_switch_mode = 1)
                    SwitchNodeParams { node_id: 102, bits: 0, on_switch_mode: 1, fade_out_ms: 0, fade_in_ms: 0 },
                    // 103: bContinuePlayback = 1 (bits = 2), on_switch_mode = 1
                    SwitchNodeParams { node_id: 103, bits: 2, on_switch_mode: 1, fade_out_ms: 0, fade_in_ms: 0 },
                ],
            }),
        });
        // Action 200: Play SwitchCntr 100
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 200,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 100,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        // Event 300
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 300,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![200] }),
        });

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions::default());
        for (sid, amp) in [(1001, 0.25f32), (1002, 0.5f32), (1003, 0.125f32)] {
            r.insert_pcm(
                sid,
                Pcm {
                    rate: RATE,
                    layout: Layout::MONO,
                    data: vec![vec![amp; FRAME * 4]],
                    loop_points: None,
                    akd_gain: 1.0,
                },
            );
        }

        r.post_event(300, 0).unwrap();
        let mut out = vec![];
        r.render_frame(&mut out).unwrap();
        // Switch 1 active: sounds 101 and 103 started (2 voices)
        assert_eq!(r.voice_count(), 2);

        // Switch to 2: sound 101 stops, sound 103 continues without restarting, sound 102 starts
        r.set_switch(50, 2, None).unwrap();
        assert_eq!(r.voice_count(), 3);
        assert!(r.voices[0].stop_at.is_some(), "sound 101 should be stopping");
        assert!(r.voices[1].stop_at.is_none(), "sound 103 (bContinuePlayback) should keep playing");
        assert_eq!(r.voices[2].sound_id, 102, "sound 102 should have started");

        // Switch back to 1: sound 101 has bIsFirstOnly=1 so it does NOT restart; 103 continues
        r.set_switch(50, 1, None).unwrap();
        assert_eq!(r.voice_count(), 3, "sound 101 must not restart due to bIsFirstOnly");
    }

    #[test]
    fn bus_ducking_and_recovery() {
        use crate::bnk::hirc::DuckInfo;

        let mut p = Project::new(None);
        // Master bus 1
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });
        // Music bus 2 (child of Master 1), max_duck_volume = -12 dB
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 2,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                override_bus_id: 1,
                max_duck_volume: -12.0,
                ..Default::default()
            }),
        });
        // VO bus 3 (child of Master 1): ducks Music bus 2 by -6.0206 dB (~0.5x), 0ms fade, 0ms recovery
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 3,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                override_bus_id: 1,
                recovery_time_ms: 0,
                ducks: vec![DuckInfo {
                    bus_id: 2,
                    duck_volume: -6.0206,
                    fade_out_ms: 0,
                    fade_in_ms: 0,
                    fade_curve: 4,
                    target_prop: prop::BUS_VOLUME,
                }],
                ..Default::default()
            }),
        });

        // Sound 101 on Music bus 2 (4 frames long, amplitude 1.0)
        p.insert_hirc(make_sound(101, 0, 2, 1001, 0.0));
        // Sound 102 on VO bus 3 (1 frame long, amplitude 0.0 so we measure Music bus 2's output directly)
        p.insert_hirc(make_sound(102, 0, 3, 1002, 0.0));

        for (aid, tid) in [(201, 101), (202, 102)] {
            p.insert_hirc(HircObject {
                hirc_type: 0x03,
                id: aid,
                offset: 0,
                size: 0,
                body: HircBody::Action(Action {
                    action_type: 0x0403,
                    kind: ActionKind::Play,
                    target_id: tid,
                    target_is_bus: false,
                    props: Props::default(),
                    ranged: RangedProps::default(),
                    params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
                }),
            });
        }
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 301,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![201, 202] }),
        });

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![1.0; FRAME * 3]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
        r.insert_pcm(
            1002,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.0; FRAME]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );

        r.post_event(301, 0).unwrap();
        let mut f0_out = vec![];
        r.render_frame(&mut f0_out).unwrap();
        // Frame 0: VO bus 3 is active, so Music bus 2 is ducked by -6.0206 dB (~0.5)
        let f0_end = *f0_out.last().unwrap();
        assert!((f0_end - 0.5).abs() < 0.02, "expected ducked ~0.5, got {}", f0_end);

        // Frame 1: VO bus 3 finished at end of frame 0; Music bus 2 recovers to 1.0
        let mut f1_out = vec![];
        r.render_frame(&mut f1_out).unwrap();
        let f1_end = *f1_out.last().unwrap();
        assert!((f1_end - 1.0).abs() < 0.02, "expected recovered ~1.0, got {}", f1_end);
    }

    #[test]
    fn aux_bus_user_and_game_sends() {
        use crate::bnk::hirc::AuxParams;

        let mut p = Project::new(None);
        // Master bus 1
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });
        // AuxBus 10 -> Master bus 1 (0 dB bus volume)
        p.insert_hirc(HircObject {
            hirc_type: 0x12,
            id: 10,
            offset: 0,
            size: 0,
            body: HircBody::AuxBus(Bus {
                override_bus_id: 1,
                ..Default::default()
            }),
        });
        // AuxBus 11 -> Master bus 1 (game aux target)
        p.insert_hirc(HircObject {
            hirc_type: 0x12,
            id: 11,
            offset: 0,
            size: 0,
            body: HircBody::AuxBus(Bus {
                override_bus_id: 1,
                ..Default::default()
            }),
        });

        // Sound 101 -> Master bus 1, with UserAuxSend0 -> AuxBus 10 at -6.0206 dB (~0.5)
        // and bUseGameAuxSends (0x20 | 0x10 | 0x08 | 0x04 = 0x3C)
        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 101,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 1,
                    props: Props(vec![(prop::USER_AUX_SEND_VOLUME0, (-6.0206f32).to_bits())]),
                    aux: AuxParams {
                        bits: 0x3C,
                        aux_ids: Some([10, 0, 0, 0]),
                        reflections_aux_bus: 0,
                        span: (0, 0),
                    },
                    ..Default::default()
                },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 301,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![201] }),
        });

        let mut gs = GameState::default();
        // GameAuxSend to AuxBus 11 at -12.0412 dB (~0.25)
        gs.set_game_aux_sends(vec![(11, -12.0412)]);

        let mut r = Renderer::new(&p, gs, RenderOptions { out_channels: 1, ..Default::default() });
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.4; FRAME]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
        r.post_event(301, 0).unwrap();
        let out = r.render().unwrap();
        // Dry (1.0) + UserAux0 (0.5) + GameAux (0.25) = 1.75 * 0.4 = 0.70
        let sample = out[FRAME / 2];
        assert!((sample - 0.70).abs() < 0.02, "expected dry+user_aux+game_aux ~0.70, got {}", sample);
    }

    #[test]
    fn max_num_instance_limiting_and_virtual_voices() {
        use crate::bnk::hirc::AdvSettings;

        let mut p = Project::new(None);
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });

        // Sound 101: max_num_instance = 1, bUseVirtualBehavior = 1 (bit 1 = 0x02),
        // virtual_queue_behavior = 2 (Resume), bIsVVoicesOptOverrideParent = 1 (bit 4 = 0x10)
        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 101,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 1,
                    props: Props(vec![(prop::PRIORITY, 50.0f32.to_bits())]),
                    adv: AdvSettings {
                        bits: 0x02 | 0x10, // bUseVirtualBehavior | bIsVVoicesOptOverrideParent
                        virtual_queue_behavior: 2, // Resume
                        max_num_instance: 1,
                        below_threshold_behavior: 2, // SetAsVirtualVoice
                        hdr_bits: 0,
                    },
                    ..Default::default()
                },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 301,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![201] }),
        });

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.5; FRAME * 2]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );

        // Post two instances at sample 0: max_num_instance = 1 means oldest (voice 0) becomes Virtual
        // while newest (voice 1) plays physically.
        r.post_event(301, 0).unwrap();
        r.post_event(301, 0).unwrap();
        let mut f0 = vec![];
        r.render_frame(&mut f0).unwrap();
        assert_eq!(r.voices.len(), 2);
        assert_eq!(r.voices[0].state, VState::Virtual, "oldest voice should be virtualized by max_num_instance=1");
        assert_eq!(r.voices[1].state, VState::Playing, "newest voice should play physically");
        // Output amplitude should be 0.5 (single voice playing), not 1.0 (two voices)
        assert!((f0[FRAME - 1] - 0.5).abs() < 0.02);

        // Render frame 1: voice 1 finishes at end of frame 1 (2 frames long)
        let mut f1 = vec![];
        r.render_frame(&mut f1).unwrap();
        assert_eq!(r.voices[1].state, VState::Done);

        // Render frame 2: slot is now free, so virtual voice 0 wakes up with Resume behavior and plays!
        let mut f2 = vec![];
        r.render_frame(&mut f2).unwrap();
        assert_eq!(r.voices[0].state, VState::Playing, "virtual voice should wake up when slot frees");
        assert!((f2[FRAME - 1] - 0.5).abs() < 0.02);
    }

    #[test]
    fn below_threshold_virtual_voice_behaviors() {
        use crate::bnk::hirc::AdvSettings;

        let mut p = Project::new(None);
        p.volume_threshold_db = -20.0;
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });

        // Sound 101: volume = -30 dB (below -20 dB threshold), SetAsVirtualVoice (2), FromElapsedTime (1)
        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 101,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 1,
                    props: Props(vec![(prop::VOLUME, (-30.0f32).to_bits())]),
                    adv: AdvSettings {
                        bits: 0x10, // bIsVVoicesOptOverrideParent
                        virtual_queue_behavior: 1, // FromElapsedTime
                        max_num_instance: 0,
                        below_threshold_behavior: 2, // SetAsVirtualVoice
                        hdr_bits: 0,
                    },
                    ..Default::default()
                },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 301,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![201] }),
        });

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        // Frame 0 has amplitude 0.25, Frame 1 has amplitude 0.75
        let mut samples = vec![0.25f32; FRAME];
        samples.extend(vec![0.75f32; FRAME]);
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![samples],
                loop_points: None,
                akd_gain: 1.0,
            },
        );

        r.post_event(301, 0).unwrap();
        let mut f0 = vec![];
        r.render_frame(&mut f0).unwrap();
        // Frame 0: voice is below threshold (-30 dB < -20 dB), so it is Virtual and produces silence,
        // while its cursor advances through Frame 0 (FromElapsedTime).
        assert_eq!(r.voices[0].state, VState::Virtual);
        assert!(f0.iter().all(|&x| x == 0.0), "virtual voice must not mix into output");
        assert_eq!(r.voices[0].seg_pos, FRAME, "FromElapsedTime must advance cursor while virtual");

        // Raise voice volume above threshold for Frame 1
        r.voices[0].params.volume_db = 0.0;
        r.voices[0].base_lin = 1.0;
        let mut f1 = vec![];
        r.render_frame(&mut f1).unwrap();
        assert_eq!(r.voices[0].state, VState::Done, "voice finishes at end of Frame 1");
        assert!((f1[FRAME - 1] - 0.75).abs() < 0.02, "woken FromElapsedTime voice should play Frame 1 samples (0.75)");
    }

    #[test]
    fn music_ranseq_chaining_pre_entry_and_post_exit_overlap() {
        use crate::bnk::hirc::{
            MeterInfo, MusicMarker, MusicNode, MusicPlaylistItem, MusicRanSeqCntr, MusicSegment,
            MusicTrack, MusicTransNode, TrackSrcInfo,
        };

        let mut p = Project::new(None);
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });

        // Segment 1 (id 10): duration 30 ms (1440 samples), Entry at 0 ms, Exit at 20 ms (960 samples).
        // Post-exit tail is [20..30 ms] = [960..1440 samples].
        // Track 11 has source 1001 (amplitude 0.4).
        p.insert_hirc(HircObject {
            hirc_type: 0x0b,
            id: 11,
            offset: 0,
            size: 0,
            body: HircBody::MusicTrack(MusicTrack {
                flags: 0,
                sources: vec![Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                }],
                playlist: vec![TrackSrcInfo {
                    track_id: 0,
                    source_id: 1001,
                    event_id: 0,
                    play_at: 0.0,
                    begin_trim_offset: 0.0,
                    end_trim_offset: 0.0,
                    src_duration: 30.0,
                }],
                num_sub_track: 1,
                clip_automation: vec![],
                node: NodeBase { override_bus_id: 1, direct_parent_id: 10, ..Default::default() },
                track_type: 0,
                switch: None,
                look_ahead_time: 0,
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x0a,
            id: 10,
            offset: 0,
            size: 0,
            body: HircBody::MusicSegment(MusicSegment {
                music: MusicNode {
                    flags: 0,
                    node: NodeBase { override_bus_id: 1, direct_parent_id: 100, ..Default::default() },
                    children: vec![11],
                    meter: MeterInfo::default(),
                    stingers: vec![],
                },
                duration_ms: 30.0,
                markers: vec![
                    MusicMarker { id: 43_573_010, position_ms: 0.0, name: String::new() },
                    MusicMarker { id: 1_539_036_744, position_ms: 20.0, name: String::new() },
                ],
            }),
        });

        // Segment 2 (id 20): duration 30 ms (1440 samples), Entry at 10 ms (480 samples), Exit at 30 ms.
        // Pre-entry is [0..10 ms] (480 samples). When chained at Segment 1's Exit (960 samples),
        // Segment 2's Entry lands at 960, so its Pre-entry starts at 960 - 480 = 480 samples,
        // and its post-entry overlaps Segment 1's post-exit across [960..1440 samples]!
        // Track 21 has source 1002 (amplitude 0.3).
        p.insert_hirc(HircObject {
            hirc_type: 0x0b,
            id: 21,
            offset: 0,
            size: 0,
            body: HircBody::MusicTrack(MusicTrack {
                flags: 0,
                sources: vec![Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1002,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                }],
                playlist: vec![TrackSrcInfo {
                    track_id: 0,
                    source_id: 1002,
                    event_id: 0,
                    play_at: 0.0,
                    begin_trim_offset: 0.0,
                    end_trim_offset: 0.0,
                    src_duration: 30.0,
                }],
                num_sub_track: 1,
                clip_automation: vec![],
                node: NodeBase { override_bus_id: 1, direct_parent_id: 20, ..Default::default() },
                track_type: 0,
                switch: None,
                look_ahead_time: 0,
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x0a,
            id: 20,
            offset: 0,
            size: 0,
            body: HircBody::MusicSegment(MusicSegment {
                music: MusicNode {
                    flags: 0,
                    node: NodeBase { override_bus_id: 1, direct_parent_id: 100, ..Default::default() },
                    children: vec![21],
                    meter: MeterInfo::default(),
                    stingers: vec![],
                },
                duration_ms: 30.0,
                markers: vec![
                    MusicMarker { id: 43_573_010, position_ms: 10.0, name: String::new() },
                    MusicMarker { id: 1_539_036_744, position_ms: 30.0, name: String::new() },
                ],
            }),
        });

        // MusicRanSeqCntr 100: ContinuousSequence (rs_type = 0, loop_count = 1) with children [Seg 10, Seg 20]
        p.insert_hirc(HircObject {
            hirc_type: 0x0d,
            id: 100,
            offset: 0,
            size: 0,
            body: HircBody::MusicRanSeqCntr(MusicRanSeqCntr {
                trans: MusicTransNode {
                    music: MusicNode {
                        flags: 0,
                        node: NodeBase { override_bus_id: 1, ..Default::default() },
                        children: vec![10, 20],
                        meter: MeterInfo::default(),
                        stingers: vec![],
                    },
                    rules: vec![],
                },
                num_playlist_items: 3,
                playlist: vec![MusicPlaylistItem {
                    segment_id: 0,
                    playlist_item_id: 1,
                    rs_type: 0, // ContinuousSequence
                    loop_count: 1,
                    loop_min: 0,
                    loop_max: 0,
                    weight: 50,
                    avoid_repeat_count: 0,
                    is_using_weight: false,
                    is_shuffle: false,
                    children: vec![
                        MusicPlaylistItem {
                            segment_id: 10,
                            playlist_item_id: 2,
                            rs_type: 0xFFFF_FFFF,
                            loop_count: 1,
                            loop_min: 0,
                            loop_max: 0,
                            weight: 50,
                            avoid_repeat_count: 0,
                            is_using_weight: false,
                            is_shuffle: false,
                            children: vec![],
                        },
                        MusicPlaylistItem {
                            segment_id: 20,
                            playlist_item_id: 3,
                            rs_type: 0xFFFF_FFFF,
                            loop_count: 1,
                            loop_min: 0,
                            loop_max: 0,
                            weight: 50,
                            avoid_repeat_count: 0,
                            is_using_weight: false,
                            is_shuffle: false,
                            children: vec![],
                        },
                    ],
                }],
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 200,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 100,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 300,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![200] }),
        });

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.4; 1440]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
        r.insert_pcm(
            1002,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.3; 1440]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );

        r.post_event(300, 0).unwrap();
        let out = r.render().unwrap();
        assert_eq!(r.voices.len(), 2);
        // Voice 0 (Seg 10): [0 .. 1440]
        assert_eq!(r.voices[0].start, 0);
        assert_eq!(r.voices[0].end, Some(1440));
        // Voice 1 (Seg 20): Entry at 960 minus pre-entry 480 -> starts at 480, ends at 480 + 1440 = 1920!
        assert_eq!(r.voices[1].start, 480);
        assert_eq!(r.voices[1].end, Some(1920));
        // During overlap [480 .. 1440], both voices play (0.4 + 0.3 = 0.7)
        assert!((out[1024] - 0.7).abs() < 0.02, "expected overlap 0.7, got {}", out[1024]);
    }

    #[test]
    fn music_switch_cntr_decision_tree_transition_and_stinger() {
        use crate::bnk::hirc::{
            DecisionNode, DecisionTree, MeterInfo, MusicFade, MusicMarker, MusicNode, MusicSegment,
            MusicSwitchCntr, MusicTrack, MusicTransNode, Stinger, TrackSrcInfo, TransitionRule,
        };
        use crate::engine::music::sync_type;

        let mut p = Project::new(None);
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });

        // Helper to insert a MusicSegment + MusicTrack with a given constant PCM source
        let mut add_seg = |seg_id: u32, track_id: u32, src_id: u32, dur_ms: f64| {
            p.insert_hirc(HircObject {
                hirc_type: 0x0b,
                id: track_id,
                offset: 0,
                size: 0,
                body: HircBody::MusicTrack(MusicTrack {
                    flags: 0,
                    sources: vec![Source {
                        plugin_id: 0x0001_0001,
                        stream_type: 0,
                        source_id: src_id,
                        in_memory_media_size: 0,
                        source_bits: 0,
                        plugin_params: None,
                    }],
                    playlist: vec![TrackSrcInfo {
                        track_id: 0,
                        source_id: src_id,
                        event_id: 0,
                        play_at: 0.0,
                        begin_trim_offset: 0.0,
                        end_trim_offset: 0.0,
                        src_duration: dur_ms,
                    }],
                    num_sub_track: 1,
                    clip_automation: vec![],
                    node: NodeBase { override_bus_id: 1, direct_parent_id: seg_id, ..Default::default() },
                    track_type: 0,
                    switch: None,
                    look_ahead_time: 0,
                }),
            });
            p.insert_hirc(HircObject {
                hirc_type: 0x0a,
                id: seg_id,
                offset: 0,
                size: 0,
                body: HircBody::MusicSegment(MusicSegment {
                    music: MusicNode {
                        flags: 0,
                        node: NodeBase { override_bus_id: 1, direct_parent_id: 500, ..Default::default() },
                        children: vec![track_id],
                        meter: MeterInfo {
                            grid_period: 10.0, // 480 samples
                            grid_offset: 0.0,
                            tempo: 120.0,
                            time_sig_num_beats: 4,
                            time_sig_beat_value: 4,
                            override_flag: 1,
                        },
                        stingers: vec![],
                    },
                    duration_ms: dur_ms,
                    markers: vec![
                        MusicMarker { id: 43_573_010, position_ms: 0.0, name: String::new() },
                        MusicMarker { id: 1_539_036_744, position_ms: dur_ms, name: String::new() },
                    ],
                }),
            });
        };

        // Explore segment 10 (src 1001 = 0.25), Combat segment 20 (src 1002 = 0.6), Stinger segment 30 (src 1003 = 0.2)
        add_seg(10, 11, 1001, 40.0);
        add_seg(20, 21, 1002, 40.0);
        add_seg(30, 31, 1003, 20.0);

        // MusicSwitchCntr 500: driven by StateGroup 90 (state 1 = Explore -> 10, state 2 = Combat -> 20, wildcard 0 -> 10),
        // TransitionRule: Any -> Any on NextGrid (sync_type = 1, 10 ms = 480 samples),
        // Stinger on Trigger 777 -> Segment 30 Immediate (sync_play_at = 0).
        p.insert_hirc(HircObject {
            hirc_type: 0x0c,
            id: 500,
            offset: 0,
            size: 0,
            body: HircBody::MusicSwitchCntr(MusicSwitchCntr {
                trans: MusicTransNode {
                    music: MusicNode {
                        flags: 0,
                        node: NodeBase { override_bus_id: 1, ..Default::default() },
                        children: vec![10, 20],
                        meter: MeterInfo {
                            grid_period: 10.0,
                            grid_offset: 0.0,
                            tempo: 120.0,
                            time_sig_num_beats: 4,
                            time_sig_beat_value: 4,
                            override_flag: 1,
                        },
                        stingers: vec![Stinger {
                            trigger_id: 777,
                            segment_id: 30,
                            sync_play_at: sync_type::IMMEDIATE,
                            cue_filter_hash: 0,
                            dont_repeat_time: 100,
                            num_segment_look_ahead: 1,
                        }],
                    },
                    rules: vec![TransitionRule {
                        src_ids: vec![0xFFFF_FFFF],
                        dst_ids: vec![0xFFFF_FFFF],
                        src_fade: MusicFade { transition_time: 0, fade_curve: 4, fade_offset: 0 },
                        src_sync_type: sync_type::NEXT_GRID,
                        src_cue_filter_hash: 0,
                        play_post_exit: false,
                        dst_fade: MusicFade { transition_time: 0, fade_curve: 4, fade_offset: 0 },
                        dst_cue_filter_hash: 0,
                        jump_to_id: 0,
                        jump_to_type: 0,
                        entry_type: 0,
                        play_pre_entry: true,
                        dest_match_source_cue_name: false,
                        transition_object: None,
                    }],
                },
                continue_playback: true,
                tree: DecisionTree {
                    mode: 0,
                    arguments: vec![(90, 1)], // StateGroup 90
                    nodes: vec![
                        DecisionNode { key: 0, child_or_audio: 10, weight: 50, probability: 100 },
                        DecisionNode { key: 1, child_or_audio: 10, weight: 50, probability: 100 },
                        DecisionNode { key: 2, child_or_audio: 20, weight: 50, probability: 100 },
                    ],
                },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 200,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 500,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 300,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![200] }),
        });

        let mut gs = GameState::default();
        gs.set_state(90, 1); // Start in Explore (segment 10)
        let mut r = Renderer::new(&p, gs, RenderOptions { out_channels: 1, ..Default::default() });
        for (sid, amp) in [(1001, 0.25f32), (1002, 0.6f32), (1003, 0.2f32)] {
            r.insert_pcm(
                sid,
                Pcm {
                    rate: RATE,
                    layout: Layout::MONO,
                    data: vec![vec![amp; 1920]],
                    loop_points: None,
                    akd_gain: 1.0,
                },
            );
        }

        r.post_event(300, 0).unwrap();
        let mut f0 = vec![];
        r.render_frame(&mut f0).unwrap();
        // Segment 10 (Track 11) is playing
        assert_eq!(r.voices.len(), 1);
        assert_eq!(r.voices[0].sound_id, 11);

        // At sample 512 (end of Frame 0), switch StateGroup 90 to 2 (Combat -> Segment 20).
        // NextGrid (10 ms = 480 samples) after 512 is 2 * 480 = 960!
        r.set_state(90, 2).unwrap();
        assert_eq!(r.voices.len(), 2);
        assert_eq!(r.voices[0].fade_out.map(|(f, _)| f.start), Some(960), "Explore track fade-out should anchor at NextGrid (960)");
        assert_eq!(r.voices[0].stop_at, Some(1024), "Explore track 0-tick fade-out completes at end of frame containing 960 (1024)");
        assert_eq!(r.voices[1].sound_id, 21, "Combat track 21 should be scheduled");
        assert_eq!(r.voices[1].start, 960, "Combat track should start at NextGrid (960)");

        // Fire Trigger 777: schedules Stinger segment 30 (Track 31) immediately at sample 512
        r.post_trigger(777).unwrap();
        assert_eq!(r.voices.len(), 3);
        assert_eq!(r.voices[2].sound_id, 31, "Stinger track 31 should start on trigger 777");
        assert_eq!(r.voices[2].start, 512);
    }

    #[test]
    fn real_global_bnk_music_switch_cntr_resolves_and_schedules() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/audiobanks/pc");
        let init_path = dir.join("Init.bnk");
        let global_path = dir.join("Global.bnk");
        if !init_path.exists() || !global_path.exists() {
            return;
        }
        let mut p = Project::new(None);
        p.load_bank_bytes("Init.bnk".into(), std::fs::read(&init_path).unwrap()).unwrap();
        p.load_bank_bytes("Global.bnk".into(), std::fs::read(&global_path).unwrap()).unwrap();

        // Global.bnk contains MusicSwitchCntr 18728212 -> MusicRanSeqCntr 98248585 -> MusicSegment 540312475 -> MusicTrack 912726085
        assert!(matches!(p.node(18_728_212).map(|o| &o.body), Some(HircBody::MusicSwitchCntr(_))));
        assert!(matches!(p.node(98_248_585).map(|o| &o.body), Some(HircBody::MusicRanSeqCntr(_))));
        assert!(matches!(p.node(540_312_475).map(|o| &o.body), Some(HircBody::MusicSegment(_))));
        assert!(matches!(p.node(912_726_085).map(|o| &o.body), Some(HircBody::MusicTrack(_))));

        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { max_seconds: 1.0, ..Default::default() });
        // Play MusicSwitchCntr 18728212 directly: it should resolve its DecisionTree to MusicRanSeqCntr 98248585,
        // traverse its playlist to MusicSegment 540312475, and schedule active MusicSegment instances!
        let _ = r.play_node(18_728_212, 0, None, 0).unwrap();
        assert!(!r.active_music_switches.is_empty(), "MusicSwitchCntr should be registered active");
        assert_eq!(r.active_music_switches[0].active_child, 98_248_585);
        assert!(!r.active_music_segments.is_empty(), "MusicRanSeqCntr should have scheduled MusicSegment 540312475");
        assert_eq!(r.active_music_segments[0].segment_id, 540_312_475);
    }

    #[test]
    fn node_and_bus_fx_chain_meter_writeback_and_bypass_action() {
        use crate::bnk::hirc::{FxBase, FxChain, FxSlot};
        use crate::engine::fx::plugin_id;

        let mut p = Project::new(None);

        // FX 501: Wwise Gain (+6.0206 dB -> ~2.0x) on Sound 101 slot 0
        let mut gain_bytes = Vec::new();
        gain_bytes.extend_from_slice(&6.0206f32.to_le_bytes());
        gain_bytes.extend_from_slice(&0.0f32.to_le_bytes());
        p.insert_hirc(HircObject {
            hirc_type: 0x11,
            id: 501,
            offset: 0,
            size: 0,
            body: HircBody::FxCustom(FxBase {
                plugin_id: plugin_id::GAIN,
                params: Some(gain_bytes),
                media: Vec::new(),
                rtpcs: Vec::new(),
                states: Default::default(),
                property_values: Vec::new(),
            }),
        });

        // FX 502: Wwise Meter on Bus 1 slot 0 writing Peak dB to GameParameter 888
        let mut meter_bytes = Vec::new();
        meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // attack = 0
        meter_bytes.extend_from_slice(&0.1f32.to_le_bytes()); // release = 0.1
        meter_bytes.extend_from_slice(&(-48.0f32).to_le_bytes()); // min = -48
        meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // max = 0
        meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // hold = 0
        meter_bytes.extend_from_slice(&[0, 0, 0, 0]);
        meter_bytes.extend_from_slice(&888u32.to_le_bytes()); // uGameParamID = 888
        p.insert_hirc(HircObject {
            hirc_type: 0x10,
            id: 502,
            offset: 0,
            size: 0,
            body: HircBody::FxShareSet(FxBase {
                plugin_id: plugin_id::METER,
                params: Some(meter_bytes),
                media: Vec::new(),
                rtpcs: Vec::new(),
                states: Default::default(),
                property_values: Vec::new(),
            }),
        });

        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                fx: FxChain {
                    override_parent: false,
                    bypass_bits: 0,
                    bypass_pos: None,
                    slots: vec![FxSlot { index: 0, fx_id: 502, is_shareset: true, is_rendered: false }],
                },
                ..Default::default()
            }),
        });

        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 101,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 1,
                    fx: FxChain {
                        override_parent: true,
                        bypass_bits: 0,
                        bypass_pos: None,
                        slots: vec![FxSlot { index: 0, fx_id: 501, is_shareset: false, is_rendered: false }],
                    },
                    ..Default::default()
                },
            }),
        });

        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        // Action 202: BypassFx on Sound 101 slot 0 (target_mask = 1 << 0)
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 202,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x1A02,
                kind: ActionKind::BypassFx,
                target_id: 101,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::BypassFx { bypass: true, target_mask: 0x01, exceptions: Vec::new() },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 301,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![201] }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: 302,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![202] }),
        });

        let mut r = Renderer::new(
            &p,
            GameState::default(),
            RenderOptions {
                out_channels: 1,
                bus_fx: true,
                ..Default::default()
            },
        );
        r.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![0.25; FRAME * 2]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );

        // Frame 0: Gain FX (+6 dB) doubles 0.25 -> ~0.50; Bus Meter writes ~-6.02 dB to GameParameter 888!
        r.post_event(301, 0).unwrap();
        let mut f0 = vec![];
        r.render_frame(&mut f0).unwrap();
        assert!((f0[FRAME / 2] - 0.50).abs() < 0.02, "expected ~0.50 with +6 dB Gain FX, got {}", f0[FRAME / 2]);
        let metered = r.gs.params.get(&888).copied().expect("Meter FX should write GameParam 888");
        assert!((metered - (-6.02)).abs() < 0.25, "expected ~-6.02 dB meter RTPC, got {metered}");

        // Frame 1: Post BypassFx event 302 at sample 512 -> Gain FX is bypassed, output drops back to 0.25!
        r.post_event(302, 512).unwrap();
        let mut f1 = vec![];
        r.render_frame(&mut f1).unwrap();
        assert!((f1[FRAME / 2] - 0.25).abs() < 0.02, "expected 0.25 after BypassFx, got {}", f1[FRAME / 2]);
    }

    #[test]
    fn fade_in_to_fade_out_handoff_starts_from_current_ratio() {
        // 10-tick linear fade-in (100 ms -> transition_ticks(4800) == 10), interrupted at
        // frame 4 (t = 4 * FRAME) by a 4-tick linear fade-out (40 ms -> 4 ticks).
        let fi = Fade { start: 0, len: ms_to_samples(100.0), curve: 4 /* Linear */ };
        assert_eq!(fi.ticks(), 10);
        let mut v = Voice {
            sound_id: 1,
            playing_id: 1,
            event_id: 1,
            game_object_id: 0,
            chain: vec![1],
            params: super::super::params::VoiceParams::default(),
            pcm: Rc::new(Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![1.0; FRAME * 20]],
                loop_points: None,
                akd_gain: 1.0,
            }),
            start: 0,
            state: VState::Playing,
            evicted_by_limit: false,
            segments: vec![(0, FRAME * 20)],
            seg: 0,
            seg_pos: 0,
            res: Resampler::new(1, RATE, false),
            base_lin: 1.0,
            ray_user_aux: 1.0,
            ray_game_aux: 1.0,
            ray_lpf: 0.0,
            ray_hpf: 0.0,
            prev_gain: Some(0.0),
            aux_prev_gain: HashMap::new(),
            aux_filters: HashMap::new(),
            node_fx: vec![],
            fade_in: Some(fi),
            fade_out: None,
            stop_at: None,
            end: Some((FRAME * 20) as u64),
            matrix: vec![],
            filter: None,
            pause_fade: None,
            clip_origin_sample: 0,
            clip_automations: vec![],
        };

        // Frames 0..4 ramp linearly: 0.1, 0.2, 0.3, 0.4.
        for k in 1..=4u64 {
            let g = Renderer::envelope(&v, k * FRAME as u64);
            let want = k as f32 * 0.1;
            assert!((g - want).abs() < 1e-5, "fade-in frame {}: got {g}, want {want}", k - 1);
        }

        // Stop at sample 4 * FRAME with 40 ms (4-tick) linear fade-out: must hand off from 0.4 -> 0.0.
        stop_voice(&mut v, 4 * FRAME as u64, ms_to_samples(40.0), 4 /* Linear */);
        let (_, captured_from) = v.fade_out.expect("fade_out scheduled");
        assert!((captured_from - 0.4).abs() < 1e-5, "expected captured handoff 0.4, got {captured_from}");

        // Frames 4..8 must descend smoothly from 0.4 -> 0.3 -> 0.2 -> 0.1 -> 0.0 (zero jump to 1.0).
        let expected_out = [0.3f32, 0.2, 0.1, 0.0];
        for (i, &want) in expected_out.iter().enumerate() {
            let t = (5 + i as u64) * FRAME as u64;
            let g = Renderer::envelope(&v, t);
            assert!((g - want).abs() < 1e-5, "fade-out step {i} (t={t}): got {g}, want {want}");
        }

        // Even before `fade_out.start`, scheduled fade-out must not prematurely suppress fade-in.
        assert!((Renderer::envelope(&v, 2 * FRAME as u64) - 0.2).abs() < 1e-5);
    }

    #[test]
    fn fade_handoff_sample_accurate_continuity_across_render_frames() {
        let mut p = Project::new(None);
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });
        p.insert_hirc(make_sound(101, 0, 1, 1001, 0.0));
        // Action 201: Play 101 with 100 ms (10-tick) Linear fade-in
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 100u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
            }),
        });
        // Action 202: Stop 101 with 40 ms (4-tick) Linear fade-out
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 202,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0102,
                kind: ActionKind::Stop,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Active { fade_curve: 4, specific: None, exceptions: vec![] },
            }),
        });
        // Action 203: Pause 101 with 40 ms (4-tick) Linear fade-out
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 203,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0202,
                kind: ActionKind::Pause,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Active { fade_curve: 4, specific: None, exceptions: vec![] },
            }),
        });
        // Action 204: Resume 101 with 40 ms (4-tick) Linear fade-in
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 204,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0302,
                kind: ActionKind::Resume,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Active { fade_curve: 4, specific: None, exceptions: vec![] },
            }),
        });
        // Action 205: Play 101 with 100 ms SCurve (5) fade-in, Stop 206 with 40 ms SCurve (5) fade-out
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 205,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 100u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Play { fade_curve: 5, bank_id: 0, bank_type: 0 },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: 206,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0102,
                kind: ActionKind::Stop,
                target_id: 101,
                target_is_bus: false,
                props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
                ranged: RangedProps::default(),
                params: ActionParams::Active { fade_curve: 5, specific: None, exceptions: vec![] },
            }),
        });
        for (eid, aid) in [(301, 201), (302, 202), (303, 203), (304, 204), (305, 205), (306, 206)] {
            p.insert_hirc(HircObject {
                hirc_type: 0x04,
                id: eid,
                offset: 0,
                size: 0,
                body: HircBody::Event(Event { actions: vec![aid] }),
            });
        }

        let dc_pcm = || Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![1.0; FRAME * 24]],
            loop_points: None,
            akd_gain: 1.0,
        };
        let unity_scale = db_to_lin(0.0).powi(4); // voice (3) + master (1)

        // --- Case A: Linear fade-in (10 ticks) -> Stop fade-out (4 ticks) at frame 4 ---
        let mut r = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r.insert_pcm(1001, dc_pcm());
        r.post_event(301, 0).unwrap();
        r.post_event(302, 4 * FRAME as u64).unwrap();
        let out = r.render().unwrap();
        assert_eq!(out.len(), 8 * FRAME, "4 fade-in frames + 4 fade-out frames");
        let max_step = out.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0f32, f32::max);
        assert!(max_step < 0.0005, "expected sample-continuous ramp across handoff, max step = {max_step}");
        let peak_idx = 4 * FRAME - 1;
        assert!((out[peak_idx] - 0.4 * unity_scale).abs() < 1e-4, "peak at handoff should be 0.4, got {}", out[peak_idx]);
        assert!(out[4 * FRAME] < out[peak_idx], "fade-out must descend immediately at handoff sample");
        assert!(out.last().unwrap().abs() < 1e-5, "fade-out must reach 0.0 cleanly");

        // --- Case B: Curved (SCurve) fade-in -> curved fade-out at frame 5 ---
        let mut r_sc = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r_sc.insert_pcm(1001, dc_pcm());
        r_sc.post_event(305, 0).unwrap();
        r_sc.post_event(306, 5 * FRAME as u64).unwrap();
        let out_sc = r_sc.render().unwrap();
        let handoff_jump = (out_sc[5 * FRAME] - out_sc[5 * FRAME - 1]).abs();
        assert!(handoff_jump < 0.001, "SCurve handoff discontinuity: {handoff_jump}");
        assert!(out_sc[5 * FRAME] < out_sc[5 * FRAME - 1], "SCurve fade-out must descend from handoff");
        assert!(out_sc.last().unwrap().abs() < 1e-5);

        // --- Case C: fade-in -> pause mid-fade-in (frame 4, ratio 0.4) -> resume mid-pause-fade-out (frame 6, ratio 0.2) ---
        let mut r_pr = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 1, ..Default::default() });
        r_pr.insert_pcm(1001, dc_pcm());
        r_pr.post_event(301, 0).unwrap();
        r_pr.post_event(303, 4 * FRAME as u64).unwrap(); // Pause at 0.4 over 4 ticks -> 0.3, 0.2, ...
        r_pr.post_event(304, 6 * FRAME as u64).unwrap(); // Resume mid-pause-fade at 0.2 over 4 ticks -> 0.4, 0.6, 0.8, 1.0
        let mut out_pr = Vec::new();
        for _ in 0..12 {
            r_pr.render_frame(&mut out_pr).unwrap();
        }
        let max_step_pr = out_pr.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0f32, f32::max);
        assert!(max_step_pr < 0.001, "pause/resume mid-fade handoff discontinuity: max step = {max_step_pr}");
        // At end of frame 3 (handoff to Pause): 0.4
        assert!((out_pr[4 * FRAME - 1] - 0.4 * unity_scale).abs() < 1e-4);
        // At end of frame 5 (handoff to Resume mid-pause-fade): 0.2
        assert!((out_pr[6 * FRAME - 1] - 0.2 * unity_scale).abs() < 1e-4);
        // After 4 resume ticks (frame 10): smoothly back to full unity 1.0!
        assert!((out_pr[10 * FRAME - 1] - 1.0 * unity_scale).abs() < 1e-4);
    }

    #[test]
    fn bus_1134869537_passthrough_gain_staging_eliminates_fast_pow10_offset() {
        use crate::bnk::hirc::{FxChain, FxSlot};

        let mut p = Project::new(None);
        // Master bus 1 (override_bus_id = 0 -> 1 FastPow10 stage)
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus::default()),
        });
        // Non-master mixing bus 2 (explicit channel_config -> 2 FastPow10 stages)
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 2,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                override_bus_id: 1,
                channel_config: 0x3102,
                ..Default::default()
            }),
        });
        // Filtered bus 1134869537: BusVolume = -3 dB, OutputBusLPF = 17, channel_config = 0,
        // active FX slot bypassed in bus_fx=false mode -> pass-through (0 stages of its own)!
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 1134869537,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                override_bus_id: 2,
                props: Props(vec![
                    (prop::BUS_VOLUME, (-3.0f32).to_bits()),
                    (prop::OUTPUT_BUS_LPF, 17.0f32.to_bits()),
                ]),
                fx: FxChain {
                    override_parent: false,
                    bypass_bits: 0,
                    bypass_pos: None,
                    slots: vec![FxSlot { index: 3, fx_id: 9999, is_shareset: false, is_rendered: false }],
                },
                ..Default::default()
            }),
        });
        // Standard 0 dB pass-through bus 3 -> mixing bus 2 -> master bus 1 (like event 4152114336)
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 3,
            offset: 0,
            size: 0,
            body: HircBody::Bus(Bus {
                override_bus_id: 2,
                ..Default::default()
            }),
        });
        // Sound 101 (vol = -5, makeup = +3) -> bus 1134869537 (matches event 1318950597!)
        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 101,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1001,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 1134869537,
                    props: Props(vec![
                        (prop::VOLUME, (-5.0f32).to_bits()),
                        (prop::MAKE_UP_GAIN, 3.0f32.to_bits()),
                    ]),
                    ..Default::default()
                },
            }),
        });
        // Sound 102 (vol = -2, makeup = +6) -> bus 3 (matches event 4152114336!)
        p.insert_hirc(HircObject {
            hirc_type: 0x02,
            id: 102,
            offset: 0,
            size: 0,
            body: HircBody::Sound(Sound {
                source: Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: 1002,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                },
                node: NodeBase {
                    override_bus_id: 3,
                    props: Props(vec![
                        (prop::VOLUME, (-2.0f32).to_bits()),
                        (prop::MAKE_UP_GAIN, 6.0f32.to_bits()),
                    ]),
                    ..Default::default()
                },
            }),
        });
        for (aid, eid, tid) in [(201, 301, 101), (202, 302, 102)] {
            p.insert_hirc(HircObject {
                hirc_type: 0x03,
                id: aid,
                offset: 0,
                size: 0,
                body: HircBody::Action(Action {
                    action_type: 0x0403,
                    kind: ActionKind::Play,
                    target_id: tid,
                    target_is_bus: false,
                    props: Props::default(),
                    ranged: RangedProps::default(),
                    params: ActionParams::Play { fade_curve: 4, bank_id: 0, bank_type: 0 },
                }),
            });
            p.insert_hirc(HircObject {
                hirc_type: 0x04,
                id: eid,
                offset: 0,
                size: 0,
                body: HircBody::Event(Event { actions: vec![aid] }),
            });
        }

        // 1. Filtered bus 1134869537 with bus_fx = false:
        //    Exact Wwise 6-stage product: db(-5) * db(+3) * db(-3) * db(0)^3
        //    (eliminates the 2 extra FastPow10(0) stages -> exact 1 / FastPow10(0)^2 fix!).
        let mut r1 = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 2, bus_fx: false, ..Default::default() });
        r1.insert_pcm(
            1001,
            Pcm {
                rate: RATE,
                layout: Layout::STEREO,
                data: vec![vec![1.0; FRAME * 2], vec![1.0; FRAME * 2]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
        r1.post_event(301, 0).unwrap();
        let out1 = r1.render().unwrap();
        // DC steady state after LPF=17 settles (last sample of frame 1)
        let got1 = out1[out1.len() - 2];
        let want1 = db_to_lin(-5.0) * db_to_lin(3.0) * db_to_lin(-3.0) * db_to_lin(0.0).powi(3);
        let old_buggy1 = want1 * db_to_lin(0.0).powi(2);
        assert!(
            (got1 - want1).abs() < 1e-6,
            "bus 1134869537 gain mismatch: got {got1:.8}, want {want1:.8} (old buggy was {old_buggy1:.8})"
        );
        // Verify LPF=17 filter was applied on the voice connection (and NOT double-applied on bus 1134869537)
        assert!(r1.voices[0].filter.is_some(), "folded OutputBusLPF=17 must live on voice connection");
        assert!(r1.bus_filters.get(&1134869537).is_none(), "pass-through bus must not double-filter");

        // 2. Standard 0 dB pass-through bus 3: exact 6-stage product db(-2) * db(+6) * db(0)^4 preserved!
        let mut r2 = Renderer::new(&p, GameState::default(), RenderOptions { out_channels: 2, bus_fx: false, ..Default::default() });
        r2.insert_pcm(
            1002,
            Pcm {
                rate: RATE,
                layout: Layout::STEREO,
                data: vec![vec![1.0; FRAME], vec![1.0; FRAME]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
        r2.post_event(302, 0).unwrap();
        let out2 = r2.render().unwrap();
        let got2 = out2[out2.len() - 2];
        let want2 = db_to_lin(-2.0) * db_to_lin(6.0) * db_to_lin(0.0).powi(4);
        assert!(
            (got2 - want2).abs() < 1e-6,
            "standard pass-through bus gain mismatch: got {got2:.8}, want {want2:.8}"
        );
    }
}

