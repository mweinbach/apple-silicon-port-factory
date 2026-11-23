//! FIFO command queue, RTPC ramping, and binary `AkWorldTransform` parser.

use super::callbacks::{EventCallbackTarget, PlayingTracker};
use crate::engine::math::{fast_pow_to_db, interp_shape, FRAME, RATE};
use crate::engine::params::GameState;
use crate::engine::render::{ExternalSourceDesc, Renderer};
use crate::engine::spatial::atten::WorldTransform;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub(super) enum Command {
    RegisterGameObj {
        obj_id: u64,
        name: Option<String>,
    },
    UnregisterGameObj {
        obj_id: u64,
    },
    UnregisterAllGameObj,
    SetPosition {
        obj_id: u64,
        transform: WorldTransform,
    },
    SetDefaultListeners {
        listeners: Vec<u64>,
    },
    SetListeners {
        emitter: u64,
        listeners: Vec<u64>,
    },
    SetDistanceProbe {
        listener: u64,
        probe: u64,
    },
    SetScalingFactor {
        obj_id: u64,
        factor: f32,
    },
    SetObjectObstructionAndOcclusion {
        emitter: u64,
        listener: u64,
        obs: f32,
        occ: f32,
    },
    SetGameObjectAuxSendValues {
        obj_id: u64,
        sends: Vec<(u64, u32, f32)>,
    },
    SetGameObjectOutputBusVolume {
        emitter: u64,
        listener: u64,
        volume_linear: f32,
    },
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
    SetSwitch {
        group_id: u32,
        switch_id: u32,
        obj_id: u64,
    },
    SetState {
        group_id: u32,
        state_id: u32,
    },
    PostTrigger {
        trigger_id: u32,
    },
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
    StopAll {
        game_obj_id: u64,
    },
    StopPlayingId {
        playing_id: u32,
        transition_ms: i32,
        curve: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum RtpcTarget {
    Global,
    GameObject(u64),
    PlayingId(u32),
}

#[derive(Debug, Clone)]
pub(super) struct RtpcRamp {
    rtpc_id: u32,
    target: RtpcTarget,
    start_val: f32,
    end_val: f32,
    start_sample: u64,
    duration_samples: u64,
    curve: u8,
}

pub(super) fn drain_commands(
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
                r.gs.registered_objects.insert(obj_id, name.unwrap_or_default());
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
                schedule_rtpc_change(
                    r,
                    rtpc_ramps,
                    rtpc_id,
                    target,
                    value,
                    transition_ms,
                    curve,
                    bypass_interp,
                    now,
                    sr,
                );
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
                schedule_rtpc_change(
                    r,
                    rtpc_ramps,
                    rtpc_id,
                    target,
                    value,
                    transition_ms,
                    curve,
                    bypass_interp,
                    now,
                    sr,
                );
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

pub(super) fn step_rtpc_ramps(r: &mut Renderer<'_>, rtpc_ramps: &mut HashMap<(u32, RtpcTarget), RtpcRamp>) {
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
