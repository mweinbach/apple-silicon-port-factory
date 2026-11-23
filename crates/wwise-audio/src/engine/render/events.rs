//! Event posting, action execution, and actor-mixer container expansion (`RanSeq`, `Switch`, `Layer`).

use super::voice::{ms_to_samples, pause_voice, resume_voice, seek_voice, start_fade_out, stop_voice};
use super::{ActiveSwitchCntr, CntrState, ExternalSourceDesc, Fade, Pending, Renderer, VState};
use crate::bnk::hirc::{prop, ActionKind, ActionParams, HircBody};
use crate::engine::math::{FRAME, RATE};
use crate::engine::params::node_initial_delay;
use crate::engine::ranseq::{single_item_next, LoopInfo, RandomInfo};
use crate::engine::rng::INV_2_31_F32;

impl<'p> Renderer<'p> {
    /// Post an event at absolute sample `at` (actions run in bank order).
    pub fn post_event(&mut self, event_id: u32, at: u64) -> Result<(), String> {
        let pid = self.state.cur_playing_id;
        let obj = self.state.gs.game_object_id;
        let ext = self.state.cur_ext_sources.clone();
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
        let o = self
            .project
            .event(event_id)
            .ok_or_else(|| format!("event {} not loaded", event_id))?;
        let HircBody::Event(ev) = &o.body else {
            return Err(format!("object {} is not an event", event_id));
        };
        for &a in &ev.actions {
            let Some(ao) = self.project.action(a) else {
                self.note(format!("missing_action_{}", a));
                continue;
            };
            let HircBody::Action(act) = &ao.body else { continue };
            let delay_ms = act.props.get_raw(prop::DELAY_TIME).unwrap_or(0) as i32;
            if act
                .ranged
                .0
                .iter()
                .any(|(id, lo, hi)| *id == prop::DELAY_TIME && (*lo != 0 || *hi != 0))
            {
                self.note("random_action_delay");
            }
            let at = at + ms_to_samples(delay_ms.max(0) as f32);
            self.state.pending.push(Pending {
                at,
                action: a,
                playing_id,
                event_id,
                game_object_id,
                ext_sources: ext_sources.clone(),
            });
        }
        self.state.pending.sort_by_key(|p| p.at);
        Ok(())
    }

    pub(super) fn run_action(&mut self, p: &Pending) -> Result<(), String> {
        let prev_pid = self.state.cur_playing_id;
        let prev_ev = self.state.cur_event_id;
        let prev_obj = self.state.cur_game_obj;
        let prev_ext = std::mem::replace(&mut self.state.cur_ext_sources, p.ext_sources.clone());
        self.state.cur_playing_id = p.playing_id;
        self.state.cur_event_id = p.event_id;
        self.state.cur_game_obj = p.game_object_id;
        self.state.gs.sync_for_object(p.game_object_id, p.playing_id);

        let res = self.run_action_inner(p);

        self.state.cur_playing_id = prev_pid;
        self.state.cur_event_id = prev_ev;
        self.state.cur_game_obj = prev_obj;
        self.state.cur_ext_sources = prev_ext;
        res
    }

    fn run_action_inner(&mut self, p: &Pending) -> Result<(), String> {
        let o = self.project.action(p.action).unwrap();
        let HircBody::Action(act) = &o.body else { return Ok(()) };
        let fade_ms = act.props.get_raw(prop::TRANSITION_TIME).unwrap_or(0) as i32 as f32;
        let at = p.at;
        self.state.log.push(format!(
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
        if let ActionParams::Seek {
            relative_to_duration,
            value,
            snap_to_marker,
            ..
        } = &act.params
        {
            self.state.log.push(format!(
                "  seek relative={} value={:?} snap={}",
                relative_to_duration, value, snap_to_marker
            ));
        }
        match (act.kind, &act.params) {
            (ActionKind::Play, ActionParams::Play { fade_curve, .. }) => {
                if act.props.get_raw(prop::PROBABILITY).is_some() {
                    self.note("play_probability");
                }
                let fade = if fade_ms > 0.0 {
                    Some(Fade {
                        start: at,
                        len: ms_to_samples(fade_ms),
                        curve: *fade_curve as u32,
                    })
                } else {
                    None
                };
                self.play_node(act.target_id, at, fade, 0)?;
            }
            (
                ActionKind::Stop | ActionKind::Pause | ActionKind::Resume,
                ActionParams::Active {
                    fade_curve,
                    exceptions,
                    ..
                },
            ) => {
                let scope = act.action_type & 0xff;
                let all = scope & 0x0c != 0; // 0x04 ALL, 0x08 ALL-EXCEPT
                let scope_obj = scope & 0x02 != 0 && p.game_object_id != 0 && p.game_object_id != u64::MAX;
                let exc: Vec<u32> = if scope & 0x08 != 0 {
                    exceptions.iter().map(|e| e.0).collect()
                } else {
                    vec![]
                };
                let target = act.target_id;
                let target_is_bus = act.target_is_bus;
                let len = ms_to_samples(fade_ms);
                let curve = *fade_curve as u32;
                for v in self.state.voices.iter_mut() {
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
                                self.state.notes.insert("pause_scheduled_voice".into());
                            }
                        }
                        _ => {
                            resume_voice(v, at, len, curve);
                        }
                    }
                }
            }
            (
                ActionKind::Seek,
                ActionParams::Seek {
                    relative_to_duration,
                    value,
                    ..
                },
            ) => {
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
                let v_seek = value.0 + self.state.rng.ranged_offset(value.1, value.2) + value.1;
                let target = act.target_id;
                for v in self.state.voices.iter_mut() {
                    if v.state == VState::Done || !v.chain.contains(&target) {
                        continue;
                    }
                    let n = v.pcm.frames() as f32;
                    // [A] truncation of the source position
                    let pos = if *relative_to_duration {
                        v_seek.clamp(0.0, 1.0) * n
                    } else {
                        v_seek * v.pcm.rate as f32 / 1000.0
                    };
                    seek_voice(v, pos.max(0.0) as usize);
                }
            }
            (ActionKind::SetGameParameter, ActionParams::SetGameParameter { value, .. }) => {
                let rtpc_id = act.target_id;
                let off = if value.min != 0.0 || value.max != 0.0 {
                    self.state.rng.ranged_offset(value.min, value.max) + value.min
                } else {
                    0.0
                };
                let v = value.base + off;
                let scope_obj = act.action_type & 0x01 != 0;
                let cur = self.state.gs.rtpc_value(self.project, rtpc_id);
                let next = match (act.action_type & 0xFF00, value.meaning) {
                    (0x1400, _) | (_, 0) => self.project.param_default(rtpc_id) + off,
                    (_, 2) => cur + v,
                    _ => v,
                };
                if scope_obj {
                    let obj = self.state.gs.game_object_id;
                    self.state.gs.set_rtpc_for_object(obj, rtpc_id, next);
                } else {
                    self.state.gs.set_rtpc(rtpc_id, next);
                }
                self.refresh_voices_rtpc();
            }
            (ActionKind::SetState, ActionParams::SetState { group_id, state_id }) => {
                self.set_state_at(*group_id, *state_id, at)?;
            }
            (ActionKind::SetSwitch, ActionParams::SetSwitch { group_id, switch_id }) => {
                let scope_obj = act.action_type & 0x01 != 0;
                let obj = scope_obj.then_some(self.state.gs.game_object_id);
                self.set_switch_at(*group_id, *switch_id, obj, at)?;
            }
            (ActionKind::Trigger, _) => {
                self.post_trigger_at(act.target_id, at)?;
            }
            (ActionKind::BypassFx, ActionParams::BypassFx { bypass, target_mask, .. }) => {
                let target = act.target_id;
                let reset = matches!(act.action_type & 0xFF00, 0x1B00 | 0x3500..=0x3700);
                if reset {
                    self.state.gs.fx_bypass_overrides.remove(&target);
                } else {
                    let base_bits = if act.target_is_bus {
                        self.project.bus(target).and_then(|o| match &o.body {
                            HircBody::Bus(b) | HircBody::AuxBus(b) => Some(b.fx.bypass_bits),
                            _ => None,
                        })
                    } else {
                        self.project
                            .node(target)
                            .and_then(|o| o.node_base())
                            .map(|n| n.fx.bypass_bits)
                    }
                    .unwrap_or(0);
                    let cur = self
                        .state
                        .gs
                        .fx_bypass_overrides
                        .get(&target)
                        .copied()
                        .unwrap_or(base_bits);
                    let next = if *bypass { cur | *target_mask } else { cur & !*target_mask };
                    self.state.gs.fx_bypass_overrides.insert(target, next);
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
    pub(super) fn play_node(&mut self, id: u32, at: u64, fade: Option<Fade>, depth: usize) -> Result<Option<u64>, String> {
        if depth > 32 {
            return Err("container nesting too deep".into());
        }
        let o = self
            .project
            .node(id)
            .ok_or_else(|| format!("node {} not loaded", id))?;
        let mut fade = fade;
        let at = match o.node_base() {
            Some(n) => {
                let d = node_initial_delay(
                    self.project,
                    &self.state.gs,
                    n,
                    &mut self.state.rng,
                    &mut self.state.notes,
                );
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
                let mut li = LoopInfo::new(&c, &mut self.state.rng);
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
                        1 | 2 if !first => Some(Fade {
                            start: t,
                            len: xfade_len,
                            curve: xfade_curve,
                        }),
                        _ if first => fade,
                        _ => None,
                    };
                    let first_voice = self.state.voices.len();
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
                            for v in self.state.voices[first_voice..].iter_mut() {
                                start_fade_out(
                                    v,
                                    Fade {
                                        start: s,
                                        len: end - s,
                                        curve: xfade_curve,
                                    },
                                );
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
                    self.state.gs.states.get(&c.group_id).copied()
                } else {
                    self.state.gs.switch_value(c.group_id)
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
                    if let Some(ex) = self.state.active_switch_cntrs.iter_mut().find(|a| a.cntr_id == id) {
                        ex.active_switch = val;
                        ex.active_children = nodes.clone();
                        ex.playing_id = self.state.cur_playing_id;
                    } else {
                        self.state.active_switch_cntrs.push(ActiveSwitchCntr {
                            cntr_id: id,
                            group_type: c.group_type,
                            group_id: c.group_id,
                            active_switch: val,
                            active_children: nodes.clone(),
                            depth,
                            playing_id: self.state.cur_playing_id,
                        });
                    }
                }
                let mut end = Some(at);
                for n in nodes {
                    let sp = c.params.iter().find(|p| p.node_id == n);
                    let is_first_only = sp.is_some_and(|p| p.bits & 1 != 0);
                    if is_first_only && self.state.switch_played_once.contains(&(id, n)) {
                        continue;
                    }
                    self.state.switch_played_once.insert((id, n));
                    let child_fade = fade.or_else(|| {
                        let fi_ms = sp.map_or(0, |p| p.fade_in_ms);
                        (fi_ms > 0).then(|| Fade {
                            start: at,
                            len: ms_to_samples(fi_ms as f32),
                            curve: 4,
                        })
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
        let mut st = self.state.cntr.remove(&id).unwrap_or_default();
        let idx = if c.mode == 1 {
            st.seq.next(c, None).unwrap_or(0)
        } else {
            self.state.notes.insert("random_container".into());
            st.random
                .get_or_insert_with(|| RandomInfo::new(c))
                .select(c, &mut self.state.rng, None)
                .unwrap_or(0)
        };
        self.state.cntr.insert(id, st);
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
            self.state.notes.insert("random_container".into());
            st.random
                .get_or_insert_with(|| RandomInfo::new(c))
                .select(c, &mut self.state.rng, Some(li))
        };
        r.map(|i| i.min(n - 1))
    }

    /// Transition time of the next continuous item, ms (`FUN_1415fbfc0`): `draw + min + base`,
    /// plus RTPC 0xF (`PlayMechanismSpecialTransitionsValue`, seconds x 1000), clamped at 0.
    fn transition_time_ms(&mut self, c: &crate::bnk::hirc::RanSeqCntr) -> f32 {
        let d = c.transition_time_mod_max - c.transition_time_mod_min;
        let r = if d != 0.0 {
            self.state.rng.next() as f32 * INV_2_31_F32 * d
        } else {
            0.0
        };
        let mut rt = 0.0f32;
        for rtpc in &c.node.rtpcs {
            if rtpc.rtpc_type == 0
                && rtpc.param_id == crate::engine::params::rtpc_param::PLAY_MECHANISM_SPECIAL_TRANSITIONS_VALUE
            {
                let x = self.state.gs.rtpc_value(self.project, rtpc.rtpc_id);
                let y = crate::engine::math::eval_graph(&rtpc.points, rtpc.scaling, x) * 1000.0;
                crate::engine::params::accum_value(&mut rt, y, rtpc.accum, false);
            }
        }
        let v = r + c.transition_time_mod_min + c.transition_time + rt;
        v.max(0.0)
    }

    pub(super) fn set_state_at(&mut self, group_id: u32, state_id: u32, at: u64) -> Result<(), String> {
        self.state.gs.set_state(group_id, state_id);
        self.refresh_voices_rtpc();
        self.update_continuous_switch_cntrs(1, group_id, at)?;
        self.update_music_switch_cntrs(at)
    }

    pub(super) fn set_switch_at(
        &mut self,
        group_id: u32,
        switch_id: u32,
        game_object_id: Option<u64>,
        at: u64,
    ) -> Result<(), String> {
        match game_object_id {
            Some(obj) => self.state.gs.set_switch_for_object(obj, group_id, switch_id),
            None => self.state.gs.set_switch(group_id, switch_id),
        }
        self.update_continuous_switch_cntrs(0, group_id, at)?;
        self.update_music_switch_cntrs(at)
    }

    fn update_continuous_switch_cntrs(&mut self, group_type: u8, group_id: u32, at: u64) -> Result<(), String> {
        let count = self.state.active_switch_cntrs.len();
        for idx in 0..count {
            if self.state.active_switch_cntrs[idx].group_type != group_type
                || self.state.active_switch_cntrs[idx].group_id != group_id
            {
                continue;
            }
            let cntr_id = self.state.active_switch_cntrs[idx].cntr_id;
            let old_switch = self.state.active_switch_cntrs[idx].active_switch;
            let old_children = self.state.active_switch_cntrs[idx].active_children.clone();
            let depth = self.state.active_switch_cntrs[idx].depth;
            let Some(o) = self.project.node(cntr_id) else { continue };
            let HircBody::SwitchCntr(c) = &o.body else { continue };
            let c = c.clone();
            let new_val = if c.group_type == 1 {
                self.state.gs.states.get(&c.group_id).copied()
            } else {
                self.state.gs.switch_value(c.group_id)
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
            self.state.active_switch_cntrs[idx].active_switch = new_val;
            self.state.active_switch_cntrs[idx].active_children = new_nodes.clone();

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
                    for v in self.state.voices.iter_mut() {
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
                    let still_playing = self.state.voices.iter().any(|v| {
                        v.state != VState::Done
                            && v.stop_at.is_none()
                            && v.chain.contains(&cntr_id)
                            && v.chain.contains(&new_ch)
                    });
                    if still_playing {
                        continue;
                    }
                }
                if is_first_only && self.state.switch_played_once.contains(&(cntr_id, new_ch)) {
                    continue;
                }
                self.state.switch_played_once.insert((cntr_id, new_ch));
                let fade_in = (fade_in_ms > 0).then(|| Fade {
                    start: at,
                    len: ms_to_samples(fade_in_ms as f32),
                    curve: 4,
                });
                self.play_node(new_ch, at, fade_in, depth + 1)?;
            }
        }
        Ok(())
    }
}
