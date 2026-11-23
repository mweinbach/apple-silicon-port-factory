//! Game-object, spatial, game-sync, event, action, and query APIs on [`NativeSoundEngine`].

use super::callbacks::{AkSegmentInfo, CallbackEvent, EventCallbackTarget};
use super::commands::{parse_ak_world_transform, Command};
use super::{
    EngineInner, NativeSoundEngine, AK_ENABLE_GET_SOURCE_PLAY_POSITION, AK_FAIL, AK_INVALID_PARAMETER, AK_SUCCESS,
};
use crate::bnk::hirc::HircBody;
use crate::engine::math::{fast_pow_to_db, RATE};
use crate::engine::music::SegmentBounds;
use crate::engine::render::ExternalSourceDesc;
use crate::engine::spatial::atten::WorldTransform;
use crate::fnv1_32_lower;
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl NativeSoundEngine {
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
        self.set_world_transform(obj_id, WorldTransform { front, top, pos })
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
    #[allow(clippy::too_many_arguments)]
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

    // --- Queries ---

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
            .unwrap_or_default();
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
}
