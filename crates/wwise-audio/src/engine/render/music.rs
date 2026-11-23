//! Interactive music hierarchy playback (`MusicTrack`, `MusicSegment`, `MusicRanSeqCntr`,
//! `MusicSwitchCntr`, transition rules, and stinger triggers).

use super::voice::{start_fade_out, stop_voice};
use super::{voice_gain, ActiveMusicSegment, ActiveMusicSwitchCntr, Fade, Renderer, VState, Voice};
use crate::bnk::hirc::HircBody;
use crate::engine::filter::ConnectionFilter;
use crate::engine::fx::FxInstance;
use crate::engine::math::RATE;
use crate::engine::music::{
    compute_sync_point, find_stingers, match_transition_rule, ms_to_samples_u64, music_fade_window,
    resolve_decision_tree, schedule_track_clip, select_sub_track, sync_type, SegmentBounds,
};
use crate::engine::params::resolve;
use crate::engine::resample::Resampler;
use std::collections::{BTreeSet, HashMap};

impl<'p> Renderer<'p> {
    pub(super) fn post_trigger_at(&mut self, trigger_id: u32, at: u64) -> Result<(), String> {
        // Find active music segment(s) playing at `at`
        let active_segs: Vec<ActiveMusicSegment> = self
            .state
            .active_music_segments
            .iter()
            .filter(|s| s.origin_sample <= at && at < s.end_sample)
            .cloned()
            .collect();
        let fallback_segs = if active_segs.is_empty() {
            self.state
                .active_music_segments
                .last()
                .cloned()
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            active_segs
        };

        let mut fired_owners = BTreeSet::new();
        for aseg in fallback_segs {
            let Some(seg_obj) = self.project.node(aseg.segment_id) else { continue };
            let HircBody::MusicSegment(seg) = &seg_obj.body else { continue };
            let seg = seg.clone();
            // Search stingers on the segment and its owner_chain
            let mut candidates = find_stingers(self.project, aseg.segment_id, trigger_id);
            for &owner in &aseg.owner_chain {
                for st in find_stingers(self.project, owner, trigger_id) {
                    if !candidates.iter().any(|(o, s)| *o == st.0 && s.segment_id == st.1.segment_id) {
                        candidates.push(st);
                    }
                }
            }
            for (owner_id, stinger) in candidates {
                if fired_owners.contains(&(owner_id, stinger.segment_id)) {
                    continue;
                }
                let dont_repeat_samples = ms_to_samples_u64(stinger.dont_repeat_time.max(0) as f64);
                if let Some(&last_at) = self.state.stinger_last_fired.get(&(owner_id, trigger_id)) {
                    if at < last_at + dont_repeat_samples {
                        continue;
                    }
                }
                let Some(sync_at) = compute_sync_point(
                    &seg,
                    None,
                    aseg.origin_sample,
                    at,
                    stinger.sync_play_at,
                    stinger.cue_filter_hash,
                ) else {
                    continue;
                };
                self.state.stinger_last_fired.insert((owner_id, trigger_id), at);
                fired_owners.insert((owner_id, stinger.segment_id));
                // Play stinger segment with its pre-entry aligned so its entry marker lands on `sync_at`
                let stinger_entry_samples = self
                    .project
                    .node(stinger.segment_id)
                    .and_then(|o| match &o.body {
                        HircBody::MusicSegment(ms) => Some(SegmentBounds::from_segment(ms).entry_samples),
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
    pub(super) fn play_music_track(
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
        let Some(sub_track) = select_sub_track(
            track_id,
            &track,
            &self.state.gs,
            &mut self.state.rng,
            &mut self.state.music_track_seq,
        ) else {
            return Ok(Some(seg_origin_sample));
        };

        let params = resolve(self.project, &self.state.gs, track_id, &mut self.state.rng)?;
        for n in &params.notes {
            self.state.notes.insert(n.clone());
        }
        let mut base_chain = Vec::new();
        let mut nid = track_id;
        while nid != 0 && base_chain.len() < 64 {
            base_chain.push(nid);
            nid = self
                .project
                .node(nid)
                .and_then(|o| o.node_base())
                .map_or(0, |n| n.direct_parent_id);
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
                let ev_at = seg_origin_sample + ms_to_samples_u64(clip.play_at.max(0.0));
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
            let Some(sched) = schedule_track_clip(
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
            if self.state.voices.len() >= 20_000 {
                return Err("more than 20000 voices scheduled".into());
            }
            let ch = pcm.data.len();
            let mut res = Resampler::new(ch, pcm.rate, self.state.opts.resampler_fma);
            res.set_pitch(params.pitch_cents, false);
            let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &params);
            let (folded_out_vol, folded_out_lpf, folded_out_hpf) =
                params.fold_passthrough_buses(self.state.opts.bus_fx);
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
            self.state.voices.push(Voice {
                sound_id: track_id,
                playing_id: self.state.cur_playing_id,
                event_id: self.state.cur_event_id,
                game_object_id: self.state.cur_game_obj,
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
    pub(super) fn play_music_segment(
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
        let bounds = SegmentBounds::from_segment(&seg);
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
        let end_sample = seg_origin_sample + ms_to_samples_u64(win_end_ms);
        self.state.active_music_segments.push(ActiveMusicSegment {
            owner_chain: chain.clone(),
            segment_id,
            playing_id: self.state.cur_playing_id,
            event_id: self.state.cur_event_id,
            game_object_id: self.state.cur_game_obj,
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
    pub(super) fn play_music_ranseq(
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
        let mut runner = self.state.music_playlists.remove(&ranseq_id).unwrap_or_default();
        let seg_ids = runner.next_segments(&cntr.playlist, &mut self.state.rng, 32);
        self.state.music_playlists.insert(ranseq_id, runner);

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
            let bounds = SegmentBounds::from_segment(seg);

            let rule = match_transition_rule(self.project, &cntr.trans.rules, prev_seg_id, seg_id);
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
                    rule.and_then(|r| music_fade_window(&r.dst_fade, seg_origin))
                        .map(|(start, len, curve)| Fade { start, len, curve })
                })
            } else {
                rule.and_then(|r| music_fade_window(&r.dst_fade, cur_entry_sample))
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
    pub(super) fn play_music_switch(
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
        let resolved = resolve_decision_tree(&cntr.tree, &self.state.gs, &mut self.state.rng).unwrap_or(0);

        if let Some(ex) = self
            .state
            .active_music_switches
            .iter_mut()
            .find(|a| a.cntr_id == switch_cntr_id)
        {
            ex.active_child = resolved;
            ex.playing_id = self.state.cur_playing_id;
        } else {
            self.state.active_music_switches.push(ActiveMusicSwitchCntr {
                cntr_id: switch_cntr_id,
                active_child: resolved,
                depth,
                playing_id: self.state.cur_playing_id,
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

        let rule = match_transition_rule(self.project, &cntr.trans.rules, 0, resolved);
        let init_fade = fade.or_else(|| {
            rule.and_then(|r| music_fade_window(&r.dst_fade, at))
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
    pub(super) fn update_music_switch_cntrs(&mut self, at: u64) -> Result<(), String> {
        let count = self.state.active_music_switches.len();
        for idx in 0..count {
            let cntr_id = self.state.active_music_switches[idx].cntr_id;
            let old_child = self.state.active_music_switches[idx].active_child;
            let depth = self.state.active_music_switches[idx].depth;
            let Some(o) = self.project.node(cntr_id) else { continue };
            let HircBody::MusicSwitchCntr(cntr) = &o.body else { continue };
            let cntr = cntr.clone();
            let new_child = resolve_decision_tree(&cntr.tree, &self.state.gs, &mut self.state.rng).unwrap_or(0);
            if new_child == old_child && cntr.continue_playback {
                continue;
            }
            self.state.active_music_switches[idx].active_child = new_child;

            let rule = match_transition_rule(self.project, &cntr.trans.rules, old_child, new_child).cloned();

            let sync_type = rule.as_ref().map_or(sync_type::IMMEDIATE, |r| r.src_sync_type);
            let cue_hash = rule.as_ref().map_or(0, |r| r.src_cue_filter_hash);

            // Find currently active segment under `cntr_id` at `at`
            let cur_seg = self
                .state
                .active_music_segments
                .iter()
                .filter(|s| s.owner_chain.contains(&cntr_id) && s.origin_sample <= at && at <= s.end_sample)
                .min_by_key(|s| s.origin_sample)
                .cloned()
                .or_else(|| {
                    self.state
                        .active_music_segments
                        .iter()
                        .rev()
                        .find(|s| s.owner_chain.contains(&cntr_id))
                        .cloned()
                });

            let sync_sample = if let Some(ref aseg) = cur_seg {
                if let Some(so) = self.project.node(aseg.segment_id) {
                    if let HircBody::MusicSegment(ms) = &so.body {
                        let meter_ovr = (cntr.trans.music.meter.override_flag != 0).then_some(&cntr.trans.music.meter);
                        compute_sync_point(ms, meter_ovr, aseg.origin_sample, at, sync_type, cue_hash)
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
            let src_fade = rule.as_ref().and_then(|r| music_fade_window(&r.src_fade, sync_sample));
            for v in self.state.voices.iter_mut() {
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
                    let f = Fade {
                        start: f_start,
                        len: f_len,
                        curve: f_curve,
                    };
                    start_fade_out(v, f);
                    v.stop_at = Some(f.end());
                } else {
                    stop_voice(v, sync_sample, 0, 4);
                }
            }
            // Trim or remove queued ActiveMusicSegments that haven't started yet
            self.state
                .active_music_segments
                .retain(|s| !(s.owner_chain.contains(&cntr_id) && s.origin_sample >= sync_sample));

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
                            let ts_bounds = SegmentBounds::from_segment(ts);
                            let play_pre = trans_obj.play_pre_entry;
                            let play_post = trans_obj.play_post_exit;
                            let ts_origin = if play_pre && sync_sample >= ts_bounds.entry_samples {
                                sync_sample - ts_bounds.entry_samples
                            } else {
                                sync_sample
                            };
                            let ts_fade = music_fade_window(&trans_obj.fade_in, sync_sample)
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
                .and_then(|r| music_fade_window(&r.dst_fade, dst_anchor))
                .map(|(start, len, curve)| Fade { start, len, curve });

            self.play_music_child(new_child, dst_anchor, dst_fade, depth + 1, &chain)?;
        }
        Ok(())
    }
}
