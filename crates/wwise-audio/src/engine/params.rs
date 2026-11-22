//! Static parameter resolution for one voice: actor-mixer property accumulation, RTPCs at their
//! current game-parameter values, output-bus routing and the bus chain.
//!
//! Semantics (wwiser `doc/EDITOR.md` + AK SDK documentation; [A] until checked against the
//! oracle): Volume, Pitch, LPF, HPF, MakeUpGain, OutputBusVolume and InitialDelay are *relative*
//! properties, summed from the sound up the `DirectParentID` chain; a ranged modifier adds a
//! uniform random offset drawn at play time. The output bus is the first non-zero
//! `OverrideBusId` walking up. Bus "voice" properties (Volume/Pitch/LPF/HPF on a bus) are added
//! to every voice routed through that bus or its descendants; BusVolume is applied to the bus mix.
//! Positioning and effects are *absolute*: taken from the first node (walking up) that overrides
//! its parent, or the top-most node.

use super::project::Project;
use super::spatial::atten::WorldTransform;
use crate::bnk::hirc::{prop, FxChain, HircBody, NodeBase, Positioning, Props, RangedProps, Rtpc};
use std::collections::{BTreeSet, HashMap};

/// AkRTPC_ParameterID (bank v135+, wwiser `AkRTPC_ParameterID_135`).
pub mod rtpc_param {
    pub const VOLUME: u32 = 0x00;
    pub const LFE: u32 = 0x01;
    pub const PITCH: u32 = 0x02;
    pub const LPF: u32 = 0x03;
    pub const HPF: u32 = 0x04;
    pub const BUS_VOLUME: u32 = 0x05;
    pub const INITIAL_DELAY: u32 = 0x06;
    pub const MAKE_UP_GAIN: u32 = 0x07;
    pub const PLAYBACK_SPEED: u32 = 0x0D;
    pub const MUTE_RATIO: u32 = 0x0E;
    pub const PLAY_MECHANISM_SPECIAL_TRANSITIONS_VALUE: u32 = 0x0F;
    pub const MAX_NUM_INSTANCES: u32 = 0x10;
    pub const PRIORITY: u32 = 0x11;
    pub const POSITION_PAN_X_2D: u32 = 0x12;
    pub const POSITION_PAN_Y_2D: u32 = 0x13;
    pub const POSITION_PAN_X_3D: u32 = 0x14;
    pub const POSITION_PAN_Y_3D: u32 = 0x15;
    pub const POSITION_PAN_Z_3D: u32 = 0x16;
    pub const POSITIONING_TYPE_BLEND: u32 = 0x17;
    pub const POSITIONING_DIVERGENCE_CENTER_PCT: u32 = 0x18;
    pub const CONE_ATTENUATION_ON_OFF: u32 = 0x19;
    pub const CONE_ATTENUATION: u32 = 0x1A;
    pub const CONE_LPF: u32 = 0x1B;
    pub const CONE_HPF: u32 = 0x1C;
    pub const BYPASS_FX0: u32 = 0x1D;
    pub const BYPASS_FX1: u32 = 0x1E;
    pub const BYPASS_FX2: u32 = 0x1F;
    pub const BYPASS_FX3: u32 = 0x20;
    pub const BYPASS_ALL_FX: u32 = 0x21;
    pub const GAME_AUX_SEND_VOLUME: u32 = 0x26;
    pub const USER_AUX_SEND_VOLUME0: u32 = 0x27;
    pub const USER_AUX_SEND_VOLUME1: u32 = 0x28;
    pub const USER_AUX_SEND_VOLUME2: u32 = 0x29;
    pub const USER_AUX_SEND_VOLUME3: u32 = 0x2A;
    pub const OUTPUT_BUS_VOLUME: u32 = 0x2B;
    pub const OUTPUT_BUS_HPF: u32 = 0x2C;
    pub const OUTPUT_BUS_LPF: u32 = 0x2D;
    pub const POSITIONING_ENABLE_ATTENUATION: u32 = 0x2E;
    pub const REFLECTIONS_VOLUME: u32 = 0x2F;
    pub const USER_AUX_SEND_LPF0: u32 = 0x30;
    pub const USER_AUX_SEND_LPF1: u32 = 0x31;
    pub const USER_AUX_SEND_LPF2: u32 = 0x32;
    pub const USER_AUX_SEND_LPF3: u32 = 0x33;
    pub const USER_AUX_SEND_HPF0: u32 = 0x34;
    pub const USER_AUX_SEND_HPF1: u32 = 0x35;
    pub const USER_AUX_SEND_HPF2: u32 = 0x36;
    pub const USER_AUX_SEND_HPF3: u32 = 0x37;
    pub const GAME_AUX_SEND_LPF: u32 = 0x38;
    pub const GAME_AUX_SEND_HPF: u32 = 0x39;
    pub const POSITION_PAN_Z_2D: u32 = 0x3A;
}

/// AkRtpcAccum (bank v128+, wwiser `AkRtpcAccum_128`).
pub mod rtpc_accum {
    pub const NONE: u8 = 0;
    pub const EXCLUSIVE: u8 = 1;
    pub const ADDITIVE: u8 = 2;
    pub const MULTIPLY: u8 = 3;
    pub const BOOLEAN: u8 = 4;
    pub const MAXIMUM: u8 = 5;
    pub const FILTER: u8 = 6;
}

/// Apply an RTPC or State value `y` to an accumulator `dst` according to `accum` (`AkRtpcAccum`).
/// For linear/multiplicative curves (`MULTIPLY`), `dst` is in dB when `dst_is_db` is true, so a
/// linear multiplier `y` adds `20 * log10(y)` (`fast_pow_to_db(y * y)`) in dB; otherwise `dst *= y`.
#[inline]
pub fn accum_value(dst: &mut f32, y: f32, accum: u8, dst_is_db: bool) {
    match accum {
        rtpc_accum::EXCLUSIVE => *dst = y,
        rtpc_accum::MULTIPLY => {
            if dst_is_db {
                if y <= 0.0 {
                    *dst = -96.3;
                } else {
                    *dst += super::math::fast_pow_to_db(y * y);
                }
            } else {
                *dst *= y;
            }
        }
        rtpc_accum::MAXIMUM => *dst = dst.max(y),
        rtpc_accum::BOOLEAN => *dst = if y != 0.0 { 1.0 } else { 0.0 },
        _ => *dst += y, // ADDITIVE (2), FILTER (6), default
    }
}

/// Game-side state that drives the graph (switches, states, game parameters, 3D listener/emitter).
#[derive(Debug, Clone)]
pub struct GameState {
    /// Global RTPC values (`SetRTPCValue(rtpc_id, val, AK_INVALID_GAME_OBJECT)`).
    pub params: HashMap<u32, f32>,
    /// Per-game-object RTPC values (`SetRTPCValue(rtpc_id, val, game_obj_id)`): `(game_obj_id, rtpc_id) -> val`.
    pub obj_params: HashMap<(u64, u32), f32>,
    /// Per-playing-ID RTPC values (`SetRTPCValueByPlayingID`): `(playing_id, rtpc_id) -> val`.
    pub playing_id_params: HashMap<(u32, u32), f32>,
    /// Active game object ID for current evaluation (`0` = default / global).
    pub game_object_id: u64,
    /// Active playing ID for current evaluation (`0` = none).
    pub active_playing_id: u32,
    pub switches: HashMap<u32, u32>,
    /// Per-game-object switches: `(game_obj_id, group_id) -> switch_id`.
    pub obj_switches: HashMap<(u64, u32), u32>,
    pub states: HashMap<u32, u32>,
    /// Game-defined auxiliary sends (`SetGameObjectAuxSendValues`): `(aux_bus_id, send_volume_db)`.
    pub game_aux_sends: Vec<(u32, f32)>,
    pub listener: WorldTransform,
    pub listener_scaling: f32,
    pub emitter: Option<WorldTransform>,
    pub emitter_scaling: f32,
    pub occlusion: f32,
    pub obstruction: f32,
    pub diffraction: f32,
    pub transmission: f32,
    /// Dynamic FX bypass bitmask overrides per node or bus ID (`ActionKind::BypassFx`).
    pub fx_bypass_overrides: HashMap<u32, u8>,
    /// Registered game objects (`RegisterGameObj`): `obj_id -> debug_name`.
    pub registered_objects: HashMap<u64, String>,
    /// Per-game-object 3D transform (`SetPosition` / `SetMultiplePositions`).
    pub obj_transforms: HashMap<u64, WorldTransform>,
    /// Per-game-object attenuation scaling factor (`SetScalingFactor`).
    pub obj_scaling: HashMap<u64, f32>,
    /// Per-game-object obstruction and occlusion (`SetObjectObstructionAndOcclusion`): `(emitter, listener) -> (obs, occ)`.
    pub obj_obs_occ: HashMap<(u64, u64), (f32, f32)>,
    /// Per-game-object auxiliary send values (`SetGameObjectAuxSendValues`): `obj_id -> Vec<(aux_bus_id, send_db)>`.
    pub obj_aux_sends: HashMap<u64, Vec<(u32, f32)>>,
    /// Per-(emitter, listener) output bus volume in dB (`SetGameObjectOutputBusVolume`).
    pub obj_output_bus_volume_db: HashMap<(u64, u64), f32>,
    /// Default listener game object IDs (`SetDefaultListeners`).
    pub default_listeners: Vec<u64>,
    /// Per-emitter listener override list (`SetListeners`).
    pub obj_listeners: HashMap<u64, Vec<u64>>,
    /// Listener -> distance probe object mapping (`SetDistanceProbe`).
    pub distance_probes: HashMap<u64, u64>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            params: HashMap::new(),
            obj_params: HashMap::new(),
            playing_id_params: HashMap::new(),
            game_object_id: 0,
            active_playing_id: 0,
            switches: HashMap::new(),
            obj_switches: HashMap::new(),
            states: HashMap::new(),
            game_aux_sends: Vec::new(),
            listener: WorldTransform::default(),
            listener_scaling: 1.0,
            emitter: None,
            emitter_scaling: 1.0,
            occlusion: 0.0,
            obstruction: 0.0,
            diffraction: 0.0,
            transmission: 0.0,
            fx_bypass_overrides: HashMap::new(),
            registered_objects: HashMap::new(),
            obj_transforms: HashMap::new(),
            obj_scaling: HashMap::new(),
            obj_obs_occ: HashMap::new(),
            obj_aux_sends: HashMap::new(),
            obj_output_bus_volume_db: HashMap::new(),
            default_listeners: Vec::new(),
            obj_listeners: HashMap::new(),
            distance_probes: HashMap::new(),
        }
    }
}

impl GameState {
    /// Synchronize active `listener`, `emitter`, `obstruction`, `occlusion`, and `game_aux_sends`
    /// fields from the per-object tables for `obj_id` (and `playing_id`).
    pub fn sync_for_object(&mut self, obj_id: u64, playing_id: u32) {
        self.game_object_id = obj_id;
        self.active_playing_id = playing_id;
        if obj_id == 0 || obj_id == u64::MAX {
            return;
        }
        let listener_id = self
            .obj_listeners
            .get(&obj_id)
            .and_then(|v| v.first().copied())
            .or_else(|| self.default_listeners.first().copied());
        if let Some(lid) = listener_id {
            if let Some(&lt) = self.obj_transforms.get(&lid) {
                self.listener = lt;
            }
            if let Some(&ls) = self.obj_scaling.get(&lid) {
                self.listener_scaling = ls;
            }
        }
        if let Some(&et) = self.obj_transforms.get(&obj_id) {
            self.emitter = Some(et);
        }
        if let Some(&es) = self.obj_scaling.get(&obj_id) {
            self.emitter_scaling = es;
        }
        let obs_occ = listener_id
            .and_then(|lid| self.obj_obs_occ.get(&(obj_id, lid)).copied())
            .or_else(|| self.obj_obs_occ.get(&(obj_id, 0)).copied());
        if let Some((obs, occ)) = obs_occ {
            self.obstruction = obs;
            self.occlusion = occ;
        }
        if let Some(sends) = self.obj_aux_sends.get(&obj_id) {
            self.game_aux_sends = sends.clone();
        }
    }

    /// Set a global RTPC value (`AK_INVALID_GAME_OBJECT`).
    pub fn set_rtpc(&mut self, rtpc_id: u32, val: f32) {
        self.params.insert(rtpc_id, val);
    }

    /// Set a per-game-object RTPC value.
    pub fn set_rtpc_for_object(&mut self, game_obj: u64, rtpc_id: u32, val: f32) {
        self.obj_params.insert((game_obj, rtpc_id), val);
    }

    /// Look up an RTPC value for `self.active_playing_id` / `self.game_object_id`, falling back
    /// to global `params`, then built-in spatial parameters (if `builtin != 0`), then
    /// `project.param_default(rtpc_id)`.
    pub fn rtpc_value(&self, project: &Project, rtpc_id: u32) -> f32 {
        if self.active_playing_id != 0 {
            if let Some(&v) = self.playing_id_params.get(&(self.active_playing_id, rtpc_id)) {
                return v;
            }
        }
        if let Some(&v) = self.obj_params.get(&(self.game_object_id, rtpc_id)) {
            return v;
        }
        if let Some(&v) = self.params.get(&rtpc_id) {
            return v;
        }
        let builtin = project
            .banks
            .iter()
            .filter(|b| b.active)
            .filter_map(|b| b.bank.global_settings.as_ref())
            .flat_map(|g| g.rtpc_ramping.iter())
            .find(|x| x.rtpc_id == rtpc_id)
            .map_or(0, |x| x.bind_to_builtin);
        if builtin != 0 {
            if let Some(em) = &self.emitter {
                let ray = super::spatial::atten::ray_geometry(&self.listener, self.listener_scaling, em, self.emitter_scaling);
                match builtin {
                    1 => return ray.distance,
                    2 => return ray.pos_l[0].atan2(ray.pos_l[2]).to_degrees(),
                    3 => {
                        let h = (ray.pos_l[0] * ray.pos_l[0] + ray.pos_l[2] * ray.pos_l[2]).sqrt();
                        return ray.pos_l[1].atan2(h).to_degrees();
                    }
                    4 => return ray.emitter_angle.to_degrees(),
                    5 => return self.obstruction * 100.0,
                    6 => return self.occlusion * 100.0,
                    7 => return ray.listener_angle.to_degrees(),
                    8 => return self.diffraction * 100.0,
                    9 => return self.transmission * 100.0,
                    _ => {}
                }
            }
        }
        project.param_default(rtpc_id)
    }

    /// Set a state value for `group_id`.
    pub fn set_state(&mut self, group_id: u32, state_id: u32) {
        self.states.insert(group_id, state_id);
    }

    /// Set a global switch value for `group_id`.
    pub fn set_switch(&mut self, group_id: u32, switch_id: u32) {
        self.switches.insert(group_id, switch_id);
    }

    /// Set a per-game-object switch value for `group_id`.
    pub fn set_switch_for_object(&mut self, game_obj: u64, group_id: u32, switch_id: u32) {
        self.obj_switches.insert((game_obj, group_id), switch_id);
    }

    /// Look up a switch value for `group_id` (per-game-object first, then global).
    pub fn switch_value(&self, group_id: u32) -> Option<u32> {
        self.obj_switches
            .get(&(self.game_object_id, group_id))
            .or_else(|| self.switches.get(&group_id))
            .copied()
    }

    /// Set game-defined auxiliary send targets `(aux_bus_id, send_volume_db)`.
    pub fn set_game_aux_sends(&mut self, sends: Vec<(u32, f32)>) {
        self.game_aux_sends = sends;
    }
}

/// AK's random generator (one global stream for every draw site) [V].
pub use super::rng::AkRng as Rng;

#[derive(Debug, Clone, Default)]
pub struct BusLink {
    pub id: u32,
    pub bus_volume_db: f32,
    /// OutputBusVolume (prop 0x18 / RTPC 0x2B) applied on this bus's connection into its parent
    /// mixing bus (`FUN_141645fc0`), or folded into a child's connection when pass-through.
    pub output_bus_volume_db: f32,
    /// Active (not rendered, not bypassed) effect slots: (slot, fx object id, plugin id).
    pub fx: Vec<(u8, u32, u32)>,
    pub channel_config: u32,
    /// The bus is mixed as its own node (see [`BusLink::mix_node`] and [`bus_is_mix_node_static`]);
    /// without bus FX this is decided by `mix_node_static` alone.
    pub mix_node_static: bool,
    /// Bus positioning used when this bus is mixed into its parent: Center % and panner type.
    pub center_pct: f32,
    pub panner_type: u8,
    pub pan: (f32, f32),
    /// OutputBusLPF / OutputBusHPF (props 0x1A / 0x19, 0..100) of this bus: filters its
    /// connection into the parent bus (`FUN_14163dbc0`) or folds into its child when pass-through
    /// (`sub_1415ca360` / `sub_1415c9610`).
    pub out_lpf: f32,
    pub out_hpf: f32,
    /// Auto-ducking targets (`DuckInfo`) configured on this bus.
    pub ducks: Vec<crate::bnk::hirc::DuckInfo>,
    pub recovery_time_ms: i32,
    pub max_duck_volume: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceLimit {
    /// Node or bus ID that owns this limit.
    pub scope_id: u32,
    pub is_bus: bool,
    pub max_instances: u16,
    pub kill_newest: bool,
    pub use_virtual_behavior: bool,
}

#[derive(Debug, Clone, Default)]
pub struct VoiceParams {
    /// Sum of relative voice Volume terms (node chain, bus voice volumes, RTPC/State Volume), dB.
    /// Applied in Stage 1 (`FUN_141600880`: `FastPow10(volume_db)`).
    pub volume_db: f32,
    /// Sum of MakeUpGain terms (props, ranged, RTPC 7), dB. Applied in Stage 2 (`FUN_141603110`:
    /// `FastPow10(makeup_db) * akd_gain`).
    pub makeup_db: f32,
    /// Sum of OutputBusVolume terms on the actor-mixer chain (prop 0x18, RTPC 0x2B), dB. Applied
    /// together with any folded pass-through bus `BusVolume`/`OutputBusVolume` in Stage 3
    /// (`FUN_141645fc0`: `FastPow10(output_bus_volume_db)`).
    pub output_bus_volume_db: f32,
    pub pitch_cents: f32,
    pub lpf: f32,
    pub hpf: f32,
    /// Actor-mixer OutputBusLPF / OutputBusHPF (props 0x1A / 0x19), applied on the voice -> mixing
    /// bus connection alongside any folded pass-through bus filters.
    pub output_bus_lpf: f32,
    pub output_bus_hpf: f32,
    /// Output bus first, master last.
    pub buses: Vec<BusLink>,
    pub positioning_node: u32,
    pub positioning: Positioning,
    pub attenuation_id: u32,
    pub spat_mix: f32,
    pub pan: (f32, f32, f32),
    pub cone_volume: Option<f32>,
    pub cone_lpf: Option<f32>,
    pub cone_hpf: Option<f32>,
    /// User-defined auxiliary sends `(aux_bus_id, send_volume_db, send_lpf, send_hpf)`.
    pub user_aux_sends: Vec<(u32, f32, f32, f32)>,
    /// Game-defined auxiliary send volume offset (dB), LPF, HPF.
    pub use_game_aux: bool,
    pub game_aux_volume_db: f32,
    pub game_aux_lpf: f32,
    pub game_aux_hpf: f32,
    pub priority: f32,
    pub priority_dist_offset: f32,
    /// Active instance limits along the actor-mixer and bus chains (`max_num_instance > 0`).
    pub instance_limits: Vec<InstanceLimit>,
    /// AkBelowThresholdBehavior (`0` ContinueToPlay, `1` KillVoice, `2` SetAsVirtualVoice, `3` KillIfOneShotElseVirtual).
    pub below_threshold_behavior: u8,
    /// AkVirtualQueueBehavior (`0` FromBeginning, `1` FromElapsedTime, `2` Resume).
    pub virtual_queue_behavior: u8,
    /// Active node FX (not rendered/bypassed): (slot, fx id, plugin id).
    pub node_fx: Vec<(u8, u32, u32)>,
    /// Feature tags that the offline renderer does not (yet) model faithfully.
    pub notes: BTreeSet<String>,
}

impl BusLink {
    /// Whether the bus gets its own mix stage (`CAkBus::IsMixingBus`, `sub_1415ca5a0`).
    /// With bus effects rendered, any active effect slot also makes the bus a mixing node; with
    /// bus FX bypassed (the A3 exit-test setup) only the static conditions apply.
    pub fn mix_node(&self, bus_fx: bool) -> bool {
        self.mix_node_static || (bus_fx && !self.fx.is_empty())
    }
}

/// Static half of `CAkBus::IsMixingBus` (`sub_1415ca5a0` [V: decompile]).
///
/// A bus creates its own mixing node (`CAkVPLMixBusNode`) when it is an AuxBus, top-level master
/// bus (`override_bus_id == 0`), declares an explicit `channel_config`, is HDR (`hdr_bits & 1`),
/// has a mixer plugin (`mixer_fx_id != 0`), has 3D/non-direct positioning, has non-zero user/game
/// aux sends, or has an RTPC on `BusVolume` / `OutputBusVolume` / `OutputBusHPF` / `OutputBusLPF`
/// (plus active FX slots handled by [`BusLink::mix_node`]).
///
/// Crucially, static `BusVolume` (`prop 0x05`) and static `OutputBusLPF`/`OutputBusHPF`
/// (`prop 0x1A`/`0x19`) do **not** make a bus a mixing bus: when `IsMixingBus()` is false, Wwise
/// folds them into the child voice's or child mixing bus's output-bus connection
/// (`sub_1415ca360` + `sub_1415c9610`), avoiding two spurious `FastPow10(0)` stages per bus.
pub fn bus_is_mix_node_static(is_aux: bool, b: &crate::bnk::hirc::Bus) -> bool {
    is_aux
        || b.override_bus_id == 0
        || b.channel_config != 0
        || (b.hdr_bits & 1 != 0)
        || b.mixer_fx_id != 0
        || b.positioning.has_listener_relative_routing()
        || (b.positioning.override_parent() && b.positioning.panner_type() != 0)
        || b.aux.aux_ids.is_some_and(|ids| ids.iter().any(|&a| a != 0))
        || (b.aux.bits & 0x02 != 0)
        || b.rtpcs.iter().any(|r| {
            matches!(
                r.param_id,
                rtpc_param::BUS_VOLUME
                    | rtpc_param::OUTPUT_BUS_VOLUME
                    | rtpc_param::OUTPUT_BUS_HPF
                    | rtpc_param::OUTPUT_BUS_LPF
            )
        })
}

impl VoiceParams {
    pub fn is_3d(&self) -> bool {
        self.positioning.has_listener_relative_routing()
    }

    /// Fold `BusVolume`, `OutputBusVolume`, `OutputBusLPF`, and `OutputBusHPF` of any leading
    /// pass-through buses (`!b.mix_node(bus_fx)`) into the voice's direct connection to its first
    /// mixing bus (`sub_1415ca360` + `sub_1415c9610` [V: decompile]).
    pub fn fold_passthrough_buses(&self, bus_fx: bool) -> (f32, f32, f32) {
        let mut out_vol = self.output_bus_volume_db;
        let mut out_lpf = self.output_bus_lpf;
        let mut out_hpf = self.output_bus_hpf;
        for b in &self.buses {
            if b.mix_node(bus_fx) {
                break;
            }
            out_vol += b.bus_volume_db + b.output_bus_volume_db;
            out_lpf += b.out_lpf;
            out_hpf += b.out_hpf;
        }
        (out_vol, out_lpf, out_hpf)
    }

    /// Effective priority of the voice (`priority + distance_ratio * priority_dist_offset` when 3D).
    pub fn effective_priority(&self, project: &Project, gs: &GameState) -> f32 {
        let mut p = self.priority;
        if self.priority_dist_offset != 0.0 && self.is_3d() {
            if let Some(em) = &gs.emitter {
                let ray = super::spatial::atten::ray_geometry(&gs.listener, gs.listener_scaling, em, gs.emitter_scaling);
                let max_d = project
                    .attenuation(self.attenuation_id)
                    .and_then(|o| match &o.body {
                        HircBody::Attenuation(a) => a.curves.first().and_then(|c| c.points.last()).map(|pt| pt.from),
                        _ => None,
                    })
                    .unwrap_or(100.0);
                if max_d > 0.0 {
                    p += (ray.distance / max_d).clamp(0.0, 1.0) * self.priority_dist_offset;
                }
            }
        }
        p.clamp(0.0, 100.0)
    }
}

/// Ranged properties drawn while gathering a voice's parameters, in draw order
/// (`FUN_1415c0040`: prop 0, 6, 2, 3, 4 per node; the node's draws happen before it recurses into
/// its parent, so draws run child -> parent) [V].
const RANGED_DRAW_ORDER: [u8; 5] = [prop::VOLUME, prop::MAKE_UP_GAIN, prop::PITCH, prop::LPF, prop::HPF];

fn add_props(p: &Props, r: &RangedProps, rng: &mut Rng, vp: &mut VoiceParams, _is_bus: bool) {
    let acc = |id: u8, v: f32, vp: &mut VoiceParams| match id {
        prop::VOLUME => vp.volume_db += v,
        prop::OUTPUT_BUS_VOLUME => vp.output_bus_volume_db += v,
        prop::MAKE_UP_GAIN => vp.makeup_db += v,
        prop::PITCH => vp.pitch_cents += v,
        prop::LPF => vp.lpf += v,
        prop::OUTPUT_BUS_LPF => vp.output_bus_lpf += v,
        prop::HPF => vp.hpf += v,
        prop::OUTPUT_BUS_HPF => vp.output_bus_hpf += v,
        prop::GAME_AUX_SEND_VOLUME => vp.game_aux_volume_db += v,
        0x44 => vp.game_aux_lpf += v, // GameAuxSendLPF
        0x45 => vp.game_aux_hpf += v, // GameAuxSendHPF
        0x13..=0x16 => {
            let idx = (id - 0x13) as usize;
            if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                s.1 += v;
            }
        }
        0x3C..=0x3F => {
            let idx = (id - 0x3C) as usize;
            if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                s.2 += v;
            }
        }
        0x40..=0x43 => {
            let idx = (id - 0x40) as usize;
            if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                s.3 += v;
            }
        }
        _ => {}
    };
    for (id, raw) in &p.0 {
        let v = f32::from_bits(*raw);
        acc(*id, v, vp);
        if v != 0.0 && (*id == prop::OUTPUT_BUS_LPF || *id == prop::OUTPUT_BUS_HPF) {
            vp.notes.insert(if *id == prop::OUTPUT_BUS_LPF { "output_bus_lpf".into() } else { "output_bus_hpf".into() });
        }
    }
    for want in RANGED_DRAW_ORDER {
        if let Some((_, lo, hi)) = r.0.iter().find(|(id, _, _)| *id == want) {
            let (lo, hi) = (f32::from_bits(*lo), f32::from_bits(*hi));
            if lo != hi {
                vp.notes.insert(format!("random_{}", prop::name(want)));
            }
            // AK: `draw + min`, then added to the accumulator
            let v = rng.ranged_offset(lo, hi) + lo;
            acc(want, v, vp);
        }
    }
    for (id, lo, hi) in &r.0 {
        if !RANGED_DRAW_ORDER.contains(id) && *id != prop::INITIAL_DELAY && (*lo != 0 || *hi != 0) {
            vp.notes.insert(format!("random_unmodelled_{}", prop::name(*id)));
        }
    }
}

/// Initial delay contributed by one node when the play path passes through it
/// (`FUN_1415c0d60`, called from the generic node play `FUN_1415faca0` and the per-type play
/// functions): prop 0x3B + RTPC 6 (InitialDelay) + ranged 0x3B (`draw + min`), seconds. Only the
/// nodes actually played (action target and its descendants down to the sound) contribute;
/// ancestors above the action target do not [V: call graph]. Returns samples, rounded half away
/// from zero in double precision like `FUN_1415bf880` / `FUN_1415c1600`.
pub fn node_initial_delay(project: &Project, gs: &GameState, node: &NodeBase, rng: &mut Rng, notes: &mut BTreeSet<String>) -> u64 {
    let mut d = node.props.get_f32(prop::INITIAL_DELAY).unwrap_or(0.0);
    let mut rt = 0.0f32;
    for r in &node.rtpcs {
        if r.rtpc_type == 0 && r.param_id == rtpc_param::INITIAL_DELAY {
            let x = gs.rtpc_value(project, r.rtpc_id);
            let y = super::math::eval_graph(&r.points, r.scaling, x);
            accum_value(&mut rt, y, r.accum, false);
            notes.insert("initial_delay_rtpc".into());
        }
    }
    d += rt;
    if let Some((_, lo, hi)) = node.ranged.0.iter().find(|(id, _, _)| *id == prop::INITIAL_DELAY) {
        let (lo, hi) = (f32::from_bits(*lo), f32::from_bits(*hi));
        if lo != hi {
            notes.insert("random_InitialDelay".into());
        }
        let off = rng.ranged_offset(lo, hi);
        d = d + off + lo;
    }
    if d > 0.0 {
        notes.insert("initial_delay".into());
        (super::math::RATE as f32 as f64 * d as f64 + 0.5) as i64 as u64
    } else {
        0
    }
}

fn active_fx(project: &Project, gs: &GameState, owner_id: u32, fx: &FxChain, rtpcs: &[Rtpc]) -> Vec<(u8, u32, u32)> {
    let mut bypass = gs.fx_bypass_overrides.get(&owner_id).copied().unwrap_or(fx.bypass_bits);
    for r in rtpcs {
        if r.rtpc_type != 0 {
            continue;
        }
        if matches!(r.param_id, rtpc_param::BYPASS_FX0..=rtpc_param::BYPASS_ALL_FX) {
            let x = gs.rtpc_value(project, r.rtpc_id);
            let y = super::math::eval_graph(&r.points, r.scaling, x);
            let bit = match r.param_id {
                rtpc_param::BYPASS_FX0 => 1u8 << 0,
                rtpc_param::BYPASS_FX1 => 1u8 << 1,
                rtpc_param::BYPASS_FX2 => 1u8 << 2,
                rtpc_param::BYPASS_FX3 => 1u8 << 3,
                _ => 0x10u8,
            };
            if y >= 0.5 {
                bypass |= bit;
            } else {
                bypass &= !bit;
            }
        }
    }
    if bypass & 0x10 != 0 {
        return vec![];
    }
    fx.slots
        .iter()
        .filter(|s| !s.is_rendered && bypass & (1 << s.index) == 0)
        .map(|s| {
            let plugin = match project.fx(s.fx_id).map(|o| &o.body) {
                Some(HircBody::FxShareSet(f)) | Some(HircBody::FxCustom(f)) => f.plugin_id,
                _ => 0,
            };
            (s.index, s.fx_id, plugin)
        })
        .collect()
}

pub fn apply_rtpcs(project: &Project, gs: &GameState, rtpcs: &[Rtpc], vp: &mut VoiceParams, mut bus: Option<&mut BusLink>) {
    for r in rtpcs {
        if r.rtpc_type != 0 {
            vp.notes.insert(if r.rtpc_type == 2 { "modulator_rtpc".into() } else { "midi_rtpc".into() });
            continue;
        }
        let builtin = project
            .banks
            .iter()
            .filter_map(|b| b.bank.global_settings.as_ref())
            .flat_map(|g| g.rtpc_ramping.iter())
            .find(|x| x.rtpc_id == r.rtpc_id)
            .map_or(0, |x| x.bind_to_builtin);
        if builtin != 0 && gs.emitter.is_none() {
            vp.notes.insert(format!("builtin_rtpc_{}", builtin));
        }
        let x = gs.rtpc_value(project, r.rtpc_id);
        let y = super::math::eval_graph(&r.points, r.scaling, x);
        // When scaling is dBToLin (4) or Log (3) and accum is Additive (2) on a dB property, or
        // when accum is Multiply (3), convert linear gain to dB if needed.
        let eff_accum = if (r.scaling == 4 || r.scaling == 3) && r.accum == rtpc_accum::ADDITIVE {
            rtpc_accum::MULTIPLY
        } else {
            r.accum
        };
        match r.param_id {
            rtpc_param::VOLUME => accum_value(&mut vp.volume_db, y, eff_accum, true),
            rtpc_param::OUTPUT_BUS_VOLUME => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.output_bus_volume_db, y, eff_accum, true),
                None => accum_value(&mut vp.output_bus_volume_db, y, eff_accum, true),
            },
            rtpc_param::MAKE_UP_GAIN => accum_value(&mut vp.makeup_db, y, eff_accum, true),
            rtpc_param::PITCH => accum_value(&mut vp.pitch_cents, y, r.accum, false),
            rtpc_param::LPF => accum_value(&mut vp.lpf, y, r.accum, false),
            rtpc_param::HPF => accum_value(&mut vp.hpf, y, r.accum, false),
            rtpc_param::OUTPUT_BUS_LPF => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.out_lpf, y, r.accum, false),
                None => accum_value(&mut vp.output_bus_lpf, y, r.accum, false),
            },
            rtpc_param::OUTPUT_BUS_HPF => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.out_hpf, y, r.accum, false),
                None => accum_value(&mut vp.output_bus_hpf, y, r.accum, false),
            },
            rtpc_param::INITIAL_DELAY => {} // per played node, see node_initial_delay
            rtpc_param::BUS_VOLUME => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.bus_volume_db, y, eff_accum, true),
                None => accum_value(&mut vp.volume_db, y, eff_accum, true),
            },
            rtpc_param::POSITIONING_TYPE_BLEND => accum_value(&mut vp.spat_mix, y, r.accum, false),
            rtpc_param::POSITIONING_DIVERGENCE_CENTER_PCT => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.center_pct, y, r.accum, false),
                None => accum_value(&mut vp.pan.2, y, r.accum, false),
            },
            rtpc_param::POSITION_PAN_X_2D | rtpc_param::POSITION_PAN_X_3D => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.pan.0, y, r.accum, false),
                None => accum_value(&mut vp.pan.0, y, r.accum, false),
            },
            rtpc_param::POSITION_PAN_Y_2D | rtpc_param::POSITION_PAN_Y_3D => match bus.as_deref_mut() {
                Some(b) => accum_value(&mut b.pan.1, y, r.accum, false),
                None => accum_value(&mut vp.pan.1, y, r.accum, false),
            },
            rtpc_param::PRIORITY => accum_value(&mut vp.priority, y, r.accum, false),
            rtpc_param::CONE_ATTENUATION => {
                let c = vp.cone_volume.get_or_insert(0.0);
                accum_value(c, y, eff_accum, true);
            }
            rtpc_param::CONE_LPF => {
                let c = vp.cone_lpf.get_or_insert(0.0);
                accum_value(c, y, r.accum, false);
            }
            rtpc_param::CONE_HPF => {
                let c = vp.cone_hpf.get_or_insert(0.0);
                accum_value(c, y, r.accum, false);
            }
            rtpc_param::GAME_AUX_SEND_VOLUME => accum_value(&mut vp.game_aux_volume_db, y, eff_accum, true),
            rtpc_param::GAME_AUX_SEND_LPF => accum_value(&mut vp.game_aux_lpf, y, r.accum, false),
            rtpc_param::GAME_AUX_SEND_HPF => accum_value(&mut vp.game_aux_hpf, y, r.accum, false),
            rtpc_param::USER_AUX_SEND_VOLUME0..=rtpc_param::USER_AUX_SEND_VOLUME3 => {
                let idx = (r.param_id - rtpc_param::USER_AUX_SEND_VOLUME0) as usize;
                if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                    accum_value(&mut s.1, y, eff_accum, true);
                }
            }
            rtpc_param::USER_AUX_SEND_LPF0..=rtpc_param::USER_AUX_SEND_LPF3 => {
                let idx = (r.param_id - rtpc_param::USER_AUX_SEND_LPF0) as usize;
                if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                    accum_value(&mut s.2, y, r.accum, false);
                }
            }
            rtpc_param::USER_AUX_SEND_HPF0..=rtpc_param::USER_AUX_SEND_HPF3 => {
                let idx = (r.param_id - rtpc_param::USER_AUX_SEND_HPF0) as usize;
                if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                    accum_value(&mut s.3, y, r.accum, false);
                }
            }
            rtpc_param::BYPASS_FX0..=rtpc_param::BYPASS_ALL_FX => {} // handled in active_fx
            rtpc_param::PLAY_MECHANISM_SPECIAL_TRANSITIONS_VALUE => {} // handled in RanSeq transition_time_ms
            p => {
                vp.notes.insert(format!("rtpc_param_{:#x}", p));
            }
        }
    }
}

/// Evaluate `StateChunk` overrides for a node or bus against `gs.states` and loaded `HircBody::State`
/// objects (`0x01`).
pub fn apply_states(project: &Project, gs: &GameState, chunk: &crate::bnk::hirc::StateChunk, vp: &mut VoiceParams, mut bus: Option<&mut BusLink>) {
    if chunk.groups.is_empty() {
        return;
    }
    for grp in &chunk.groups {
        let Some(&active_state_id) = gs.states.get(&grp.group_id) else {
            continue;
        };
        let Some(&(_, state_obj_id)) = grp.states.iter().find(|(sid, _)| *sid == active_state_id) else {
            continue;
        };
        let Some(so) = project.state(state_obj_id) else {
            continue;
        };
        let HircBody::State(st) = &so.body else {
            continue;
        };
        for &(prop_id_u16, val) in &st.props {
            let pid = prop_id_u16 as u32;
            let sp = chunk.props.iter().find(|p| p.property_id == pid);
            let accum = sp.map_or(rtpc_accum::ADDITIVE, |p| p.accum);
            let in_db = sp.map_or(true, |p| p.in_db != 0);
            let eff_accum = if !in_db && accum == rtpc_accum::ADDITIVE && matches!(pid, rtpc_param::VOLUME | rtpc_param::BUS_VOLUME | rtpc_param::OUTPUT_BUS_VOLUME | rtpc_param::MAKE_UP_GAIN) {
                rtpc_accum::MULTIPLY
            } else {
                accum
            };
            match pid {
                rtpc_param::VOLUME => accum_value(&mut vp.volume_db, val, eff_accum, true),
                rtpc_param::OUTPUT_BUS_VOLUME => match bus.as_deref_mut() {
                    Some(b) => accum_value(&mut b.output_bus_volume_db, val, eff_accum, true),
                    None => accum_value(&mut vp.output_bus_volume_db, val, eff_accum, true),
                },
                rtpc_param::MAKE_UP_GAIN => accum_value(&mut vp.makeup_db, val, eff_accum, true),
                rtpc_param::PITCH => accum_value(&mut vp.pitch_cents, val, accum, false),
                rtpc_param::LPF => accum_value(&mut vp.lpf, val, accum, false),
                rtpc_param::HPF => accum_value(&mut vp.hpf, val, accum, false),
                rtpc_param::OUTPUT_BUS_LPF => match bus.as_deref_mut() {
                    Some(b) => accum_value(&mut b.out_lpf, val, accum, false),
                    None => accum_value(&mut vp.output_bus_lpf, val, accum, false),
                },
                rtpc_param::OUTPUT_BUS_HPF => match bus.as_deref_mut() {
                    Some(b) => accum_value(&mut b.out_hpf, val, accum, false),
                    None => accum_value(&mut vp.output_bus_hpf, val, accum, false),
                },
                rtpc_param::BUS_VOLUME => match bus.as_deref_mut() {
                    Some(b) => accum_value(&mut b.bus_volume_db, val, eff_accum, true),
                    None => accum_value(&mut vp.volume_db, val, eff_accum, true),
                },
                rtpc_param::GAME_AUX_SEND_VOLUME => accum_value(&mut vp.game_aux_volume_db, val, eff_accum, true),
                rtpc_param::GAME_AUX_SEND_LPF => accum_value(&mut vp.game_aux_lpf, val, accum, false),
                rtpc_param::GAME_AUX_SEND_HPF => accum_value(&mut vp.game_aux_hpf, val, accum, false),
                rtpc_param::USER_AUX_SEND_VOLUME0..=rtpc_param::USER_AUX_SEND_VOLUME3 => {
                    let idx = (pid - rtpc_param::USER_AUX_SEND_VOLUME0) as usize;
                    if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                        accum_value(&mut s.1, val, eff_accum, true);
                    }
                }
                rtpc_param::USER_AUX_SEND_LPF0..=rtpc_param::USER_AUX_SEND_LPF3 => {
                    let idx = (pid - rtpc_param::USER_AUX_SEND_LPF0) as usize;
                    if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                        accum_value(&mut s.2, val, accum, false);
                    }
                }
                rtpc_param::USER_AUX_SEND_HPF0..=rtpc_param::USER_AUX_SEND_HPF3 => {
                    let idx = (pid - rtpc_param::USER_AUX_SEND_HPF0) as usize;
                    if let Some(s) = vp.user_aux_sends.get_mut(idx) {
                        accum_value(&mut s.3, val, accum, false);
                    }
                }
                _ => {}
            }
        }
    }
}

/// Resolve a bus chain (`[start_bus, parent, ..., master]`) for an `AuxBus` or `Bus`.
pub fn resolve_bus_chain(project: &Project, gs: &GameState, start_bus_id: u32) -> Result<Vec<BusLink>, String> {
    let mut chain = Vec::new();
    let mut dummy_vp = VoiceParams::default();
    let mut bus_id = start_bus_id;
    let mut depth = 0;
    while bus_id != 0 {
        depth += 1;
        if depth > 32 {
            return Err("bus chain too deep".into());
        }
        let o = project.bus(bus_id).ok_or_else(|| format!("bus {} not loaded", bus_id))?;
        let b = match &o.body {
            HircBody::Bus(b) | HircBody::AuxBus(b) => b,
            _ => return Err(format!("object {} is not a bus", bus_id)),
        };
        let pf = |id| b.props.get_f32(id).unwrap_or(0.0);
        let mut link = BusLink {
            id: bus_id,
            channel_config: b.channel_config,
            center_pct: pf(prop::CENTER_PCT),
            panner_type: b.positioning.panner_type(),
            pan: (pf(prop::PAN_LR), pf(prop::PAN_FR)),
            ducks: b.ducks.clone(),
            recovery_time_ms: b.recovery_time_ms,
            max_duck_volume: b.max_duck_volume,
            ..Default::default()
        };
        for (id, raw) in &b.props.0 {
            match *id {
                prop::BUS_VOLUME | prop::VOLUME => link.bus_volume_db += f32::from_bits(*raw),
                prop::OUTPUT_BUS_VOLUME => link.output_bus_volume_db += f32::from_bits(*raw),
                prop::OUTPUT_BUS_LPF | prop::LPF => link.out_lpf += f32::from_bits(*raw),
                prop::OUTPUT_BUS_HPF | prop::HPF => link.out_hpf += f32::from_bits(*raw),
                _ => {}
            }
        }
        apply_rtpcs(project, gs, &b.rtpcs, &mut dummy_vp, Some(&mut link));
        apply_states(project, gs, &b.states, &mut dummy_vp, Some(&mut link));
        link.fx = active_fx(project, gs, bus_id, &b.fx, &b.rtpcs);
        link.mix_node_static = bus_is_mix_node_static(matches!(o.body, HircBody::AuxBus(_)), b);
        chain.push(link);
        bus_id = b.override_bus_id;
    }
    Ok(chain)
}

/// Resolve the parameters of a voice playing `sound_id`. `rng` supplies ranged-modifier draws.
pub fn resolve(project: &Project, gs: &GameState, sound_id: u32, rng: &mut Rng) -> Result<VoiceParams, String> {
    let mut vp = VoiceParams {
        spat_mix: 100.0,
        priority: 50.0,
        user_aux_sends: vec![(0, 0.0, 0.0, 0.0); 4],
        ..Default::default()
    };
    let mut node_id = sound_id;
    let mut prev_child_id = 0u32;
    let mut output_bus = 0u32;
    let mut have_pos = false;
    let mut have_fx = false;
    let mut have_prio = false;
    let mut have_user_aux = false;
    let mut have_game_aux = false;
    let mut have_vvoice_opts = false;
    let mut stop_max_inst = false;
    let mut depth = 0;
    let mut last: Option<&NodeBase> = None;
    let mut last_id = 0;
    while node_id != 0 {
        depth += 1;
        if depth > 64 {
            return Err("actor-mixer chain too deep".into());
        }
        let o = project.node(node_id).ok_or_else(|| format!("node {} not loaded", node_id))?;
        let n = o.node_base().ok_or_else(|| format!("object {} (type {:#x}) is not a parameter node", node_id, o.hirc_type))?;
        if !have_user_aux && (n.aux.bits & 0x04 != 0 || n.aux.aux_ids.is_some() || n.direct_parent_id == 0) {
            if n.aux.bits & 0x04 != 0 || n.direct_parent_id == 0 {
                have_user_aux = true;
            }
            if let Some(ids) = n.aux.aux_ids {
                for (i, &aid) in ids.iter().enumerate() {
                    if vp.user_aux_sends[i].0 == 0 {
                        vp.user_aux_sends[i].0 = aid;
                    }
                }
            }
        }
        if !have_game_aux && (n.aux.bits & 0x10 != 0 || n.direct_parent_id == 0) {
            have_game_aux = true;
            vp.use_game_aux = n.aux.bits & 0x20 != 0;
        }
        if !have_vvoice_opts && (n.adv.bits & 0x10 != 0 || n.direct_parent_id == 0) {
            have_vvoice_opts = true;
            vp.below_threshold_behavior = n.adv.below_threshold_behavior;
            vp.virtual_queue_behavior = n.adv.virtual_queue_behavior;
        }
        if !stop_max_inst {
            let mut max_inst = n.adv.max_num_instance as f32;
            for r in &n.rtpcs {
                if r.rtpc_type == 0 && r.param_id == rtpc_param::MAX_NUM_INSTANCES {
                    let x = gs.rtpc_value(project, r.rtpc_id);
                    let y = super::math::eval_graph(&r.points, r.scaling, x);
                    accum_value(&mut max_inst, y, r.accum, false);
                }
            }
            let max_u16 = max_inst.round().max(0.0) as u16;
            if max_u16 > 0 {
                vp.instance_limits.push(InstanceLimit {
                    scope_id: node_id,
                    is_bus: false,
                    max_instances: max_u16,
                    kill_newest: n.adv.bits & 0x01 != 0,
                    use_virtual_behavior: n.adv.bits & 0x02 != 0,
                });
            }
            if n.adv.bits & 0x08 != 0 {
                stop_max_inst = true;
            }
        }
        add_props(&n.props, &n.ranged, rng, &mut vp, false);
        apply_rtpcs(project, gs, &n.rtpcs, &mut vp, None);
        apply_states(project, gs, &n.states, &mut vp, None);
        if let HircBody::LayerCntr(lc) = &o.body {
            for layer in &lc.layers {
                let matches_child = layer.assocs.is_empty() || layer.assocs.iter().any(|(cid, _)| *cid == prev_child_id);
                if !matches_child {
                    continue;
                }
                if layer.crossfade_rtpc_id != 0 {
                    let x = gs.rtpc_value(project, layer.crossfade_rtpc_id);
                    for (cid, pts) in &layer.assocs {
                        if *cid == prev_child_id && !pts.is_empty() {
                            // Layer crossfade curve (linear amplitude 0..1 or dB scaling 2)
                            let y = super::math::eval_graph(pts, 2, x);
                            vp.volume_db += y;
                        }
                    }
                }
                apply_rtpcs(project, gs, &layer.rtpcs, &mut vp, None);
            }
        }
        if output_bus == 0 && n.override_bus_id != 0 {
            output_bus = n.override_bus_id;
        }
        if !have_prio && (n.priority_bits & 1 != 0 || n.direct_parent_id == 0) {
            have_prio = true;
            if let Some(p) = n.props.get_f32(prop::PRIORITY) {
                vp.priority = p;
            }
            if n.priority_bits & 2 != 0 {
                vp.priority_dist_offset = n.props.get_f32(prop::PRIORITY_DISTANCE_OFFSET).unwrap_or(0.0);
            }
        }
        if !have_pos && n.positioning.override_parent() {
            have_pos = true;
            vp.positioning = n.positioning.clone();
            vp.positioning_node = node_id;
            let pf = |id| n.props.get_f32(id).unwrap_or(0.0);
            vp.pan = (pf(prop::PAN_LR), pf(prop::PAN_FR), pf(prop::CENTER_PCT));
            vp.attenuation_id = n.props.get_raw(prop::ATTENUATION_ID).unwrap_or(0);
            vp.spat_mix = n.props.get_f32(prop::POSITIONING_TYPE_BLEND).unwrap_or(100.0);
        }
        if !have_fx && (n.fx.override_parent || n.direct_parent_id == 0) {
            have_fx = true;
            vp.node_fx = active_fx(project, gs, node_id, &n.fx, &n.rtpcs);
        }
        if let Some(ids) = n.aux.aux_ids {
            if ids.iter().any(|&a| a != 0) {
                vp.notes.insert("user_aux_send".into());
            }
        }
        last = Some(n);
        last_id = node_id;
        prev_child_id = node_id;
        node_id = n.direct_parent_id;
    }
    if let Some(n) = last {
        if !have_pos {
            vp.positioning = n.positioning.clone();
            vp.positioning_node = last_id;
            let pf = |id| n.props.get_f32(id).unwrap_or(0.0);
            vp.pan = (pf(prop::PAN_LR), pf(prop::PAN_FR), pf(prop::CENTER_PCT));
            vp.attenuation_id = n.props.get_raw(prop::ATTENUATION_ID).unwrap_or(0);
            vp.spat_mix = n.props.get_f32(prop::POSITIONING_TYPE_BLEND).unwrap_or(100.0);
        }
        if !have_fx {
            vp.node_fx = active_fx(project, gs, last_id, &n.fx, &n.rtpcs);
        }
    }
    if vp.attenuation_id != 0 {
        if let Some(ao) = project.attenuation(vp.attenuation_id) {
            if let HircBody::Attenuation(a) = &ao.body {
                if let Some(c) = &a.cone {
                    vp.cone_volume.get_or_insert(c.outside_volume);
                    vp.cone_lpf.get_or_insert(c.lo_pass);
                    vp.cone_hpf.get_or_insert(c.hi_pass);
                }
                apply_rtpcs(project, gs, &a.rtpcs, &mut vp, None);
            }
        }
    }
    // With direct speaker assignment (panner type 0) the 2D pan position is ignored.
    if vp.positioning.panner_type() != 0 && (vp.pan.0 != 0.0 || vp.pan.1 != 0.0) {
        vp.notes.insert("pan".into());
    }
    if vp.is_3d() {
        vp.notes.insert("3d".into());
    }
    if !vp.node_fx.is_empty() {
        vp.notes.insert("node_fx".into());
    }
    if output_bus == 0 {
        return Err(format!("sound {}: no output bus in its actor-mixer chain", sound_id));
    }
    // bus chain
    let mut bus_id = output_bus;
    let mut stop_bus_max_inst = false;
    let mut depth = 0;
    while bus_id != 0 {
        depth += 1;
        if depth > 32 {
            return Err("bus chain too deep".into());
        }
        let o = project.bus(bus_id).ok_or_else(|| format!("bus {} not loaded (load Init.bnk)", bus_id))?;
        let b = match &o.body {
            HircBody::Bus(b) | HircBody::AuxBus(b) => b,
            _ => return Err(format!("object {} is not a bus", bus_id)),
        };
        if !stop_bus_max_inst {
            let mut max_inst = b.max_num_instance as f32;
            for r in &b.rtpcs {
                if r.rtpc_type == 0 && r.param_id == rtpc_param::MAX_NUM_INSTANCES {
                    let x = gs.rtpc_value(project, r.rtpc_id);
                    let y = super::math::eval_graph(&r.points, r.scaling, x);
                    accum_value(&mut max_inst, y, r.accum, false);
                }
            }
            let max_u16 = max_inst.round().max(0.0) as u16;
            if max_u16 > 0 {
                vp.instance_limits.push(InstanceLimit {
                    scope_id: bus_id,
                    is_bus: true,
                    max_instances: max_u16,
                    kill_newest: b.bits & 0x01 != 0,
                    use_virtual_behavior: b.bits & 0x02 != 0,
                });
            }
            if b.bits & 0x04 != 0 {
                stop_bus_max_inst = true;
            }
        }
        let pf = |id| b.props.get_f32(id).unwrap_or(0.0);
        let mut link = BusLink {
            id: bus_id,
            channel_config: b.channel_config,
            center_pct: pf(prop::CENTER_PCT),
            panner_type: b.positioning.panner_type(),
            pan: (pf(prop::PAN_LR), pf(prop::PAN_FR)),
            ducks: b.ducks.clone(),
            recovery_time_ms: b.recovery_time_ms,
            max_duck_volume: b.max_duck_volume,
            ..Default::default()
        };
        // bus "voice" props are relative and summed into the voice; BusVolume and OutputBus* stay on the bus.
        let mut voice_props = Props::default();
        for (id, raw) in &b.props.0 {
            match *id {
                prop::BUS_VOLUME => link.bus_volume_db += f32::from_bits(*raw),
                prop::OUTPUT_BUS_VOLUME => link.output_bus_volume_db += f32::from_bits(*raw),
                prop::CENTER_PCT | prop::PAN_LR | prop::PAN_FR | prop::PAN_UD => {}
                prop::OUTPUT_BUS_LPF => link.out_lpf += f32::from_bits(*raw),
                prop::OUTPUT_BUS_HPF => link.out_hpf += f32::from_bits(*raw),
                _ => voice_props.0.push((*id, *raw)),
            }
        }
        add_props(&voice_props, &RangedProps::default(), rng, &mut vp, true);
        apply_rtpcs(project, gs, &b.rtpcs, &mut vp, Some(&mut link));
        apply_states(project, gs, &b.states, &mut vp, Some(&mut link));
        link.fx = active_fx(project, gs, bus_id, &b.fx, &b.rtpcs);
        link.mix_node_static = bus_is_mix_node_static(matches!(o.body, HircBody::AuxBus(_)), b);
        if b.hdr_bits & 1 != 0 {
            vp.notes.insert("hdr_bus".into());
        }
        if !b.ducks.is_empty() {
            vp.notes.insert("ducking_bus".into());
        }
        if b.mixer_fx_id != 0 {
            vp.notes.insert("bus_mixer_plugin".into());
        }
        vp.buses.push(link);
        bus_id = b.override_bus_id;
    }
    vp.user_aux_sends.retain(|s| s.0 != 0);
    if vp.lpf > 0.0 {
        vp.notes.insert("lpf".into());
    }
    if vp.hpf > 0.0 {
        vp.notes.insert("hpf".into());
    }
    Ok(vp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnk::hirc::GraphPoint;

    #[test]
    fn rtpc_curve_shapes_and_accum() {
        let mut p = Project::new(None);
        p.param_defaults.insert(100, 50.0);

        let rtpcs = vec![
            // Linear Volume RTPC (0..100 -> -12..0 dB, Additive)
            Rtpc {
                rtpc_id: 100,
                rtpc_type: 0,
                accum: rtpc_accum::ADDITIVE,
                param_id: rtpc_param::VOLUME,
                curve_id: 1,
                scaling: 0,
                points: vec![
                    GraphPoint { from: 0.0, to: -12.0, interp: 4 },
                    GraphPoint { from: 100.0, to: 0.0, interp: 4 },
                ],
            },
            // SCurve Pitch RTPC (0..100 -> -200..200 cents, Additive)
            Rtpc {
                rtpc_id: 100,
                rtpc_type: 0,
                accum: rtpc_accum::ADDITIVE,
                param_id: rtpc_param::PITCH,
                curve_id: 2,
                scaling: 0,
                points: vec![
                    GraphPoint { from: 0.0, to: -200.0, interp: 5 },
                    GraphPoint { from: 100.0, to: 200.0, interp: 5 },
                ],
            },
            // Exp1 LPF RTPC (0..100 -> 0..60, Maximum)
            Rtpc {
                rtpc_id: 100,
                rtpc_type: 0,
                accum: rtpc_accum::MAXIMUM,
                param_id: rtpc_param::LPF,
                curve_id: 3,
                scaling: 0,
                points: vec![
                    GraphPoint { from: 0.0, to: 0.0, interp: 6 },
                    GraphPoint { from: 100.0, to: 60.0, interp: 6 },
                ],
            },
            // Log1 HPF RTPC (0..100 -> 0..40, Additive)
            Rtpc {
                rtpc_id: 100,
                rtpc_type: 0,
                accum: rtpc_accum::ADDITIVE,
                param_id: rtpc_param::HPF,
                curve_id: 4,
                scaling: 0,
                points: vec![
                    GraphPoint { from: 0.0, to: 0.0, interp: 2 },
                    GraphPoint { from: 100.0, to: 40.0, interp: 2 },
                ],
            },
        ];

        // Default RTPC value (50.0): Volume = -6 dB, Pitch = 0 cents (midpoint of SCurve)
        let mut gs = GameState::default();
        let mut vp = VoiceParams::default();
        apply_rtpcs(&p, &gs, &rtpcs, &mut vp, None);
        assert!((vp.volume_db - -6.0).abs() < 1e-4);
        assert!(vp.pitch_cents.abs() < 1.0);
        assert!(vp.lpf > 10.0 && vp.lpf < 50.0);
        assert!(vp.hpf > 10.0 && vp.hpf < 40.0);

        // Global SetRTPCValue = 100.0
        gs.set_rtpc(100, 100.0);
        let mut vp = VoiceParams::default();
        apply_rtpcs(&p, &gs, &rtpcs, &mut vp, None);
        assert!((vp.volume_db - 0.0).abs() < 1e-4);
        assert!((vp.pitch_cents - 200.0).abs() < 1e-4);
        assert!((vp.lpf - 60.0).abs() < 1e-4);
        assert!((vp.hpf - 40.0).abs() < 1e-4);

        // Per-game-object SetRTPCValue overrides global for object 42
        gs.set_rtpc_for_object(42, 100, 0.0);
        gs.game_object_id = 42;
        let mut vp_obj = VoiceParams::default();
        apply_rtpcs(&p, &gs, &rtpcs, &mut vp_obj, None);
        assert!((vp_obj.volume_db - -12.0).abs() < 1e-4);
        assert!((vp_obj.pitch_cents - -200.0).abs() < 1.0);

        //Multiplicative RTPC accumulation on Volume (y = 0.5 -> ~ -6.02 dB)
        let mul_rtpc = vec![Rtpc {
            rtpc_id: 100,
            rtpc_type: 0,
            accum: rtpc_accum::MULTIPLY,
            param_id: rtpc_param::VOLUME,
            curve_id: 5,
            scaling: 0,
            points: vec![
                GraphPoint { from: 0.0, to: 0.5, interp: 4 },
                GraphPoint { from: 100.0, to: 1.0, interp: 4 },
            ],
        }];
        let mut vp_mul = VoiceParams { volume_db: -3.0, ..Default::default() };
        apply_rtpcs(&p, &gs, &mul_rtpc, &mut vp_mul, None);
        assert!((vp_mul.volume_db - -9.0206).abs() < 0.05, "got {}", vp_mul.volume_db);
    }

    #[test]
    fn state_chunk_overrides() {
        use crate::bnk::hirc::{HircObject, State, StateChunk, StateGroupChunk, StateProp};

        let mut p = Project::new(None);
        // State object 500: Volume -6 dB, LPF +25, BusVolume -3 dB
        p.insert_hirc(HircObject {
            hirc_type: 0x01,
            id: 500,
            offset: 0,
            size: 0,
            body: HircBody::State(State {
                props: vec![
                    (rtpc_param::VOLUME as u16, -6.0),
                    (rtpc_param::LPF as u16, 25.0),
                    (rtpc_param::BUS_VOLUME as u16, -3.0),
                ],
            }),
        });

        let chunk = StateChunk {
            props: vec![
                StateProp { property_id: rtpc_param::VOLUME, accum: rtpc_accum::ADDITIVE, in_db: 1 },
                StateProp { property_id: rtpc_param::LPF, accum: rtpc_accum::ADDITIVE, in_db: 0 },
                StateProp { property_id: rtpc_param::BUS_VOLUME, accum: rtpc_accum::ADDITIVE, in_db: 1 },
            ],
            groups: vec![StateGroupChunk {
                group_id: 10,
                sync_type: 0,
                states: vec![(1, 500)],
            }],
        };

        let mut gs = GameState::default();
        let mut vp = VoiceParams { volume_db: -2.0, lpf: 10.0, ..Default::default() };
        let mut bus = BusLink { id: 99, bus_volume_db: -1.0, ..Default::default() };

        // Inactive state: no change
        apply_states(&p, &gs, &chunk, &mut vp, Some(&mut bus));
        assert!((vp.volume_db - -2.0).abs() < 1e-5);
        assert!((vp.lpf - 10.0).abs() < 1e-5);
        assert!((bus.bus_volume_db - -1.0).abs() < 1e-5);

        // Activate state (group 10 -> state 1)
        gs.set_state(10, 1);
        apply_states(&p, &gs, &chunk, &mut vp, Some(&mut bus));
        assert!((vp.volume_db - -8.0).abs() < 1e-5);
        assert!((vp.lpf - 35.0).abs() < 1e-5);
        assert!((bus.bus_volume_db - -4.0).abs() < 1e-5);
    }

    #[test]
    fn passthrough_bus_static_volume_and_filter_fold_without_allocating_mix_node() {
        use crate::bnk::hirc::{Bus, FxChain, FxSlot, HircObject, Props, StateChunk};

        let make_bus = |override_bus_id: u32, channel_config: u32, props: Props, fx: FxChain| Bus {
            override_bus_id,
            device_shareset_id: None,
            props,
            positioning: Default::default(),
            aux: Default::default(),
            bits: 0,
            max_num_instance: 0,
            channel_config,
            hdr_bits: 0,
            recovery_time_ms: 0,
            max_duck_volume: 0.0,
            ducks: vec![],
            fx,
            mixer_fx_id: 0,
            mixer_is_shareset: false,
            override_attachment_params: 0,
            metadata: vec![],
            rtpcs: vec![],
            states: StateChunk::default(),
        };

        let mut p = Project::new(None);
        // Master bus (override_bus_id = 0)
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 100,
            offset: 0,
            size: 0,
            body: HircBody::Bus(make_bus(0, 0, Props::default(), FxChain::default())),
        });
        // Non-master mixing bus with explicit channel_config
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 150,
            offset: 0,
            size: 0,
            body: HircBody::Bus(make_bus(100, 0x3102, Props::default(), FxChain::default())),
        });
        // Bus with static BusVolume = -3 dB, OutputBusLPF = 17, channel_config = 0, and a bus FX
        // slot (same topology as filtered bus 1134869537): pass-through when bus_fx = false,
        // mixing node when bus_fx = true.
        let fx_chain = FxChain {
            override_parent: false,
            bypass_bits: 0,
            bypass_pos: None,
            slots: vec![FxSlot { index: 3, fx_id: 999, is_shareset: false, is_rendered: false }],
        };
        p.insert_hirc(HircObject {
            hirc_type: 0x08,
            id: 201,
            offset: 0,
            size: 0,
            body: HircBody::Bus(make_bus(
                150,
                0,
                Props(vec![
                    (prop::BUS_VOLUME, (-3.0f32).to_bits()),
                    (prop::OUTPUT_BUS_LPF, 17.0f32.to_bits()),
                ]),
                fx_chain,
            )),
        });

        let gs = GameState::default();
        let chain = resolve_bus_chain(&p, &gs, 201).unwrap();
        assert_eq!(chain.len(), 3);
        // Per CAkBus::IsMixingBus (sub_1415ca5a0), static BusVolume and OutputBusLPF do NOT
        // allocate a mix node on their own:
        assert!(!chain[0].mix_node_static, "static BusVolume + OutputBusLPF stays pass-through");
        assert!(!chain[0].mix_node(false), "without bus FX, bus 201 is pass-through");
        assert!(chain[0].mix_node(true), "with bus FX enabled, active FX slot makes bus 201 a mix node");
        assert!(chain[1].mix_node_static, "explicit channel_config allocates a mix node");
        assert!(chain[2].mix_node_static, "root/master bus always allocates a mix node");

        // Folding pass-through buses into a voice folds -3 dB and LPF 17 cleanly when bus_fx = false,
        // and leaves them on bus 201 when bus_fx = true.
        let vp = VoiceParams { buses: chain, ..Default::default() };
        let (vol_no_fx, lpf_no_fx, hpf_no_fx) = vp.fold_passthrough_buses(false);
        assert!((vol_no_fx - -3.0).abs() < 1e-5);
        assert!((lpf_no_fx - 17.0).abs() < 1e-5);
        assert_eq!(hpf_no_fx, 0.0);

        let (vol_with_fx, lpf_with_fx, _) = vp.fold_passthrough_buses(true);
        assert_eq!(vol_with_fx, 0.0);
        assert_eq!(lpf_with_fx, 0.0);
    }
}
