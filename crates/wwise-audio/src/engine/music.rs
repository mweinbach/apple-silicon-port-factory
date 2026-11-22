//! Interactive Music Engine (Milestone A6, docs/AUDIO.md §7 item 9 & §8 A6).
//!
//! Runtime scheduler and helper logic for Wwise's 4 music HIRC types:
//! - [`MusicTrack`] (`0x0b`): sub-track selection (`Normal`, `Random`, `Sequence`, `Switch`),
//!   `TrackSrcInfo` clip timing/trimming (`play_at`, `begin_trim_offset`, `end_trim_offset`,
//!   `src_duration` in ms -> 48 kHz samples), and `ClipAutomation` envelopes (`Volume`, `LPF`,
//!   `HPF`, `FadeIn`, `FadeOut`).
//! - [`MusicSegment`] (`0x0a`): entry/exit markers (`Entry` cue `43573010` / `1408807229` / `0`,
//!   `Exit` cue `1539036744` / `1652161569` / `1`, or first/last marker fallback) dividing the
//!   segment into pre-entry (`0..entry`), active body (`entry..exit`), and post-exit (`exit..duration`),
//!   plus meter grid/beat/bar sync points (`MeterInfo`).
//! - [`MusicRanSeqCntr`] (`0x0d`): hierarchical playlist traversal (`ContinuousSequence`,
//!   `StepSequence`, `ContinuousRandom`, `StepRandom`, and leaf `Segment`) with loop counts and
//!   avoid-repeat, chaining segments across entry/exit markers using `TransitionRule`s.
//! - [`MusicSwitchCntr`] (`0x0c`): `DecisionTree` evaluation (depth up to 6, `BestMatch` vs
//!   `Weighted` modes, wildcard `0` fallback), dynamic transition scheduling on `set_state` /
//!   `set_switch` at `AkSyncType` boundaries (`Immediate`, `NextGrid`, `NextBar`, `NextBeat`,
//!   `NextMarker`, `NextUserMarker`, `EntryMarker`, `ExitMarker`, `ExitNever`), source fade-out +
//!   destination fade-in + optional transition segment (`TransitionObject`), and `Stinger` triggers.

use super::math::{eval_graph, RATE};
use super::params::{GameState, Rng};
use super::project::Project;
use crate::bnk::hirc::{
    ClipAutomation, DecisionTree, HircBody, MeterInfo, MusicFade, MusicPlaylistItem, MusicSegment,
    MusicTrack, Stinger, TrackSrcInfo, TransitionRule,
};
use std::collections::HashMap;

/// Known Wwise Entry marker IDs (`FNV1("entry cue")`, prompt's `1408807229`, legacy `0`).
pub const MARKER_ENTRY_IDS: [u32; 4] = [43_573_010, 1_408_807_229, 2_184_290_995, 0];
/// Known Wwise Exit marker IDs (`FNV1("exit cue")`, prompt's `1652161569`, legacy `1`).
pub const MARKER_EXIT_IDS: [u32; 4] = [1_539_036_744, 1_652_161_569, 26_822_469, 1];

/// `AkSyncType` synchronization points for music transitions and stingers.
pub mod sync_type {
    pub const IMMEDIATE: u32 = 0;
    pub const NEXT_GRID: u32 = 1;
    pub const NEXT_BAR: u32 = 2;
    pub const NEXT_BEAT: u32 = 3;
    pub const NEXT_MARKER: u32 = 4;
    pub const NEXT_USER_MARKER: u32 = 5;
    pub const ENTRY_MARKER: u32 = 6;
    pub const EXIT_MARKER: u32 = 7;
    pub const EXIT_NEVER: u32 = 8;
    pub const LAST_EXIT_POSITION: u32 = 9;
}

/// `AkClipAutomationType` for bank v112+ (including v145).
pub mod clip_auto {
    pub const VOLUME: u32 = 0;
    pub const LPF: u32 = 1;
    pub const HPF: u32 = 2;
    pub const FADE_IN: u32 = 3;
    pub const FADE_OUT: u32 = 4;
}

/// Convert milliseconds (`f64`) to engine samples at 48 kHz (`i64`, preserving sign).
#[inline]
pub fn ms_to_samples_i64(ms: f64) -> i64 {
    (ms * (RATE as f64) / 1000.0).round() as i64
}

/// Convert non-negative milliseconds (`f64`) to engine samples at 48 kHz (`u64`).
#[inline]
pub fn ms_to_samples_u64(ms: f64) -> u64 {
    ms_to_samples_i64(ms.max(0.0)).max(0) as u64
}

/// Entry and exit marker positions (in ms and 48 kHz samples) for a [`MusicSegment`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegmentBounds {
    pub entry_ms: f64,
    pub exit_ms: f64,
    pub duration_ms: f64,
    pub entry_samples: u64,
    pub exit_samples: u64,
    pub duration_samples: u64,
}

impl SegmentBounds {
    /// Extract entry, exit, and total duration from a [`MusicSegment`].
    /// Looks up known Entry/Exit marker IDs first, falling back to the first/last marker or
    /// `0.0 .. duration_ms`.
    pub fn from_segment(seg: &MusicSegment) -> Self {
        let dur_ms = seg.duration_ms.max(0.0);
        let entry_ms = seg
            .markers
            .iter()
            .find(|m| MARKER_ENTRY_IDS.contains(&m.id))
            .or_else(|| seg.markers.first())
            .map_or(0.0, |m| m.position_ms.clamp(0.0, dur_ms));
        let exit_ms = seg
            .markers
            .iter()
            .rev()
            .find(|m| MARKER_EXIT_IDS.contains(&m.id))
            .or_else(|| seg.markers.last())
            .map_or(dur_ms, |m| m.position_ms.clamp(entry_ms, dur_ms));
        let entry_samples = ms_to_samples_u64(entry_ms);
        let exit_samples = ms_to_samples_u64(exit_ms).max(entry_samples);
        let duration_samples = ms_to_samples_u64(dur_ms).max(exit_samples);
        SegmentBounds {
            entry_ms,
            exit_ms,
            duration_ms: dur_ms,
            entry_samples,
            exit_samples,
            duration_samples,
        }
    }

    /// Length of the active segment body (`exit - entry`) in samples.
    pub fn active_len_samples(&self) -> u64 {
        self.exit_samples.saturating_sub(self.entry_samples)
    }
}

/// Compute the next synchronization sample (`>= now`) for a currently playing segment whose
/// timeline origin (`t = 0`, pre-entry start) is anchored at `seg_origin_sample`.
/// Returns `None` if `sync_play_at == EXIT_NEVER` or no valid marker exists.
pub fn compute_sync_point(
    seg: &MusicSegment,
    meter_override: Option<&MeterInfo>,
    seg_origin_sample: u64,
    now: u64,
    sync_play_at: u32,
    cue_filter_hash: u32,
) -> Option<u64> {
    let bounds = SegmentBounds::from_segment(seg);
    let entry_abs = seg_origin_sample + bounds.entry_samples;
    let exit_abs = seg_origin_sample + bounds.exit_samples;
    let meter = meter_override.unwrap_or(&seg.music.meter);

    match sync_play_at {
        sync_type::IMMEDIATE => Some(now),
        sync_type::ENTRY_MARKER => Some(if now <= entry_abs { entry_abs } else { exit_abs }),
        sync_type::EXIT_MARKER | sync_type::LAST_EXIT_POSITION => Some(exit_abs.max(now)),
        sync_type::EXIT_NEVER => None,
        sync_type::NEXT_GRID | sync_type::NEXT_BEAT | sync_type::NEXT_BAR => {
            let tempo = if meter.tempo > 0.0 { meter.tempo as f64 } else { 120.0 };
            let beat_val = if meter.time_sig_beat_value > 0 { meter.time_sig_beat_value as f64 } else { 4.0 };
            let num_beats = if meter.time_sig_num_beats > 0 { meter.time_sig_num_beats as f64 } else { 4.0 };
            let beat_ms = (60_000.0 / tempo) * (4.0 / beat_val);
            let bar_ms = beat_ms * num_beats;
            let grid_ms = if meter.grid_period > 0.0 { meter.grid_period } else { beat_ms };
            let step_ms = match sync_play_at {
                sync_type::NEXT_GRID => grid_ms,
                sync_type::NEXT_BAR => bar_ms,
                _ => beat_ms,
            };
            let step_samples = ms_to_samples_u64(step_ms).max(1);
            let offset_samples = ms_to_samples_u64(meter.grid_offset.max(0.0));
            let anchor = entry_abs + offset_samples;
            if now <= anchor {
                Some(anchor)
            } else {
                let elapsed = now - anchor;
                let k = (elapsed + step_samples - 1) / step_samples;
                Some(anchor + k * step_samples)
            }
        }
        sync_type::NEXT_MARKER | sync_type::NEXT_USER_MARKER => {
            let user_only = sync_play_at == sync_type::NEXT_USER_MARKER;
            let mut best: Option<u64> = None;
            for m in &seg.markers {
                if user_only && (MARKER_ENTRY_IDS.contains(&m.id) || MARKER_EXIT_IDS.contains(&m.id)) {
                    continue;
                }
                if cue_filter_hash != 0 && m.id != cue_filter_hash {
                    continue;
                }
                let m_abs = seg_origin_sample + ms_to_samples_u64(m.position_ms);
                if m_abs >= now && best.map_or(true, |b| m_abs < b) {
                    best = Some(m_abs);
                }
            }
            best.or(Some(exit_abs.max(now)))
        }
        _ => Some(now),
    }
}

/// Computed schedule and source slice segments for a single [`TrackSrcInfo`] clip inside a
/// [`MusicSegment`] window `[window_start_samples, window_end_samples)`.
#[derive(Debug, Clone)]
pub struct ScheduledClip {
    pub clip_index: usize,
    pub track_id: u32,
    pub source_id: u32,
    pub event_id: u32,
    /// Absolute engine sample when this clip begins playing.
    pub start_sample: u64,
    /// Absolute engine sample when this clip stops playing.
    pub end_sample: u64,
    /// Segment-relative sample where the clip's own timeline (`t_clip = 0`, after `play_at + begin_trim`)
    /// starts, used to evaluate `ClipAutomation` curves (`from` in seconds relative to clip start).
    pub clip_origin_sample: u64,
    /// Source frame ranges `(start_frame, end_frame)` at `src_rate` to feed into the resampler.
    pub pcm_segments: Vec<(usize, usize)>,
    /// Clip automation curves attached to this `clip_index`.
    pub automations: Vec<ClipAutomation>,
}

/// Schedule a [`TrackSrcInfo`] clip at `src_rate` / `pcm_frames` within a segment whose timeline
/// `0..duration` starts at `seg_origin_sample` and is audible over `[window_start_ms, window_end_ms]`.
pub fn schedule_track_clip(
    clip_index: usize,
    clip: &TrackSrcInfo,
    pcm_frames: usize,
    src_rate: u32,
    seg_origin_sample: u64,
    window_start_ms: f64,
    window_end_ms: f64,
    all_automations: &[ClipAutomation],
) -> Option<ScheduledClip> {
    if pcm_frames == 0 || src_rate == 0 || clip.src_duration <= 0.0 {
        return None;
    }

    // Wwise clip timing (in ms relative to segment timeline 0):
    // - clip_start_ms = fPlayAt + fBeginTrimOffset
    // - clip_end_ms   = fPlayAt + fSrcDuration + fEndTrimOffset
    // When fBeginTrimOffset >= 0, we skip `fBeginTrimOffset` into the source.
    // When fBeginTrimOffset < 0, the source repeats before its start by `|fBeginTrimOffset|`.
    // When fEndTrimOffset > 0, the source repeats past its end by `fEndTrimOffset`.
    let threshold = 1.0 / (RATE as f64);
    let mut fbt = clip.begin_trim_offset;
    let mut fet = clip.end_trim_offset;
    if fbt < 0.0 && fbt > -threshold {
        fbt = 0.0;
        if fet > 0.0 && fet < threshold {
            fet = 0.0;
        }
    }

    let clip_start_ms = (clip.play_at + fbt).max(0.0);
    let clip_end_ms = clip.play_at + clip.src_duration + fet;
    if clip_end_ms <= clip_start_ms {
        return None;
    }

    let eff_start_ms = clip_start_ms.max(window_start_ms);
    let eff_end_ms = clip_end_ms.min(window_end_ms);
    if eff_end_ms <= eff_start_ms {
        return None;
    }

    // Source offset at `clip_start_ms`:
    let base_src_offset_ms = if fbt >= 0.0 {
        fbt
    } else {
        let rem = fbt % clip.src_duration;
        if rem < 0.0 { rem + clip.src_duration } else { rem }
    };
    let window_skip_ms = eff_start_ms - clip_start_ms;
    let play_dur_ms = eff_end_ms - eff_start_ms;

    let ms_to_src_frames = |ms: f64| -> usize {
        ((ms.max(0.0) * (src_rate as f64) / 1000.0).round() as usize).min(pcm_frames)
    };
    let total_src_needed = ((play_dur_ms * (src_rate as f64) / 1000.0).round() as usize).max(1);
    let mut cur_src_pos = ms_to_src_frames((base_src_offset_ms + window_skip_ms) % clip.src_duration);
    if cur_src_pos >= pcm_frames {
        cur_src_pos = 0;
    }

    let mut remaining = total_src_needed;
    let mut pcm_segments = Vec::new();
    while remaining > 0 {
        let avail = pcm_frames.saturating_sub(cur_src_pos);
        if avail == 0 {
            cur_src_pos = 0;
            continue;
        }
        let take = remaining.min(avail);
        pcm_segments.push((cur_src_pos, cur_src_pos + take));
        remaining -= take;
        cur_src_pos = 0;
    }

    let clip_origin_sample = seg_origin_sample + ms_to_samples_u64(clip_start_ms);
    let start_sample = seg_origin_sample + ms_to_samples_u64(eff_start_ms);
    let end_sample = seg_origin_sample + ms_to_samples_u64(eff_end_ms);
    if end_sample <= start_sample {
        return None;
    }

    let automations: Vec<ClipAutomation> = all_automations
        .iter()
        .filter(|a| a.clip_index as usize == clip_index)
        .cloned()
        .collect();

    Some(ScheduledClip {
        clip_index,
        track_id: clip.track_id,
        source_id: clip.source_id,
        event_id: clip.event_id,
        start_sample,
        end_sample,
        clip_origin_sample,
        pcm_segments,
        automations,
    })
}

/// Evaluate [`ClipAutomation`] curves at absolute engine sample `sample_at` for a clip whose
/// un-clamped start is `clip_origin_sample`.
/// Returns `(linear_gain_multiplier, lpf_add, hpf_add)`.
pub fn eval_clip_automations(
    automations: &[ClipAutomation],
    clip_origin_sample: u64,
    sample_at: u64,
) -> (f32, f32, f32) {
    if automations.is_empty() {
        return (1.0, 0.0, 0.0);
    }
    let t_sec = (sample_at.saturating_sub(clip_origin_sample) as f64 / RATE as f64) as f32;
    let mut gain = 1.0f32;
    let mut lpf = 0.0f32;
    let mut hpf = 0.0f32;
    for auto in automations {
        if auto.points.is_empty() {
            continue;
        }
        match auto.auto_type {
            clip_auto::VOLUME => {
                // In Wwise v70+ (including v145), Volume clip automation stores normalized dB-scaled
                // values in `[-1.0 .. 0.0]` where `0.0` is unity (`1.0`) and `-1.0` is silence (`0.0`);
                // `eval_graph` with `scaling = 2` (dBToLin on `y`) or linear `1.0 + y` maps this cleanly.
                // If any point is `< -1.0`, treat as raw dB.
                let is_raw_db = auto.points.iter().any(|p| p.to < -1.001);
                if is_raw_db {
                    let db = eval_graph(&auto.points, 0, t_sec);
                    gain *= super::math::db_to_lin(db);
                } else {
                    // `scaling = 2` in `eval_graph` converts `[-1..0]` via `db_to_lin` / `(y + 1.0)`
                    let y = eval_graph(&auto.points, 0, t_sec);
                    gain *= (1.0 + y).clamp(0.0, 4.0);
                }
            }
            clip_auto::FADE_IN | clip_auto::FADE_OUT => {
                let y = eval_graph(&auto.points, 0, t_sec);
                gain *= y.clamp(0.0, 1.0);
            }
            clip_auto::LPF => {
                let y = eval_graph(&auto.points, 0, t_sec);
                lpf += y;
            }
            clip_auto::HPF => {
                let y = eval_graph(&auto.points, 0, t_sec);
                hpf += y;
            }
            _ => {}
        }
    }
    (gain, lpf.clamp(0.0, 100.0), hpf.clamp(0.0, 100.0))
}

/// Select the active sub-track index (`0..num_sub_track`) for a [`MusicTrack`].
/// Returns `None` if `num_sub_track == 0` or a Switch track maps to no subtrack.
pub fn select_sub_track(
    track_id: u32,
    track: &MusicTrack,
    gs: &GameState,
    rng: &mut Rng,
    seq_state: &mut HashMap<u32, u32>,
) -> Option<u32> {
    let n = track.num_sub_track;
    if n == 0 {
        return None;
    }
    match track.track_type {
        0 => Some(0), // Normal
        1 => {
            // RandomStep
            if n == 1 {
                Some(0)
            } else {
                let r = (rng.next() as u64 * n as u64) >> 31;
                Some((r as u32).min(n - 1))
            }
        }
        2 => {
            // SequenceStep
            let cur = seq_state.entry(track_id).or_insert(0);
            let picked = *cur % n;
            *cur = (*cur + 1) % n;
            Some(picked)
        }
        3 => {
            // Switch
            let Some(sw) = &track.switch else { return Some(0) };
            let val = if sw.group_type == 1 {
                gs.states.get(&sw.group_id).copied()
            } else {
                gs.switch_value(sw.group_id)
            }
            .unwrap_or(sw.default_switch);
            sw.switch_assoc
                .iter()
                .position(|&assoc| assoc == val)
                .or_else(|| sw.switch_assoc.iter().position(|&assoc| assoc == sw.default_switch))
                .map(|idx| idx as u32)
        }
        _ => Some(0),
    }
}

/// Resolve an [`DecisionTree`] (`CAkMusicSwitchCntr` or `CAkDialogueEvent`) to a leaf audio node ID.
///
/// The flat `nodes` slice stores the root node at index 0 (`key = 0`), whose `child_or_audio` packs
/// `uIdx = low u16`, `uCount = high u16` for depth 0 children. At `depth == arguments.len()`,
/// `child_or_audio` is the leaf `audioNodeId`.
pub fn resolve_decision_tree(
    tree: &DecisionTree,
    gs: &GameState,
    rng: &mut Rng,
) -> Option<u32> {
    if tree.nodes.is_empty() {
        return None;
    }
    let max_depth = tree.arguments.len();
    if max_depth == 0 {
        return Some(tree.nodes[0].child_or_audio);
    }

    // Look up argument values for each depth level (`0` = unset / wildcard).
    let mut arg_vals = Vec::with_capacity(max_depth);
    for &(group_id, group_type) in &tree.arguments {
        let val = if group_type == 1 {
            gs.states.get(&group_id).copied().unwrap_or(0)
        } else {
            gs.switch_value(group_id).unwrap_or(0)
        };
        arg_vals.push(val);
    }

    // Some single-depth trees store their depth-1 children directly in `tree.nodes[0..]` without a
    // synthetic root header node when `nodes[0].child_or_audio` is already an audio node ID (> nodes.len()).
    let root_packed = tree.nodes[0].child_or_audio;
    let root_idx = (root_packed & 0xFFFF) as usize;
    let root_cnt = (root_packed >> 16) as usize;
    let (start_idx, start_cnt) = if max_depth == 1
        && (root_idx == 0 || root_cnt == 0 || root_idx + root_cnt > tree.nodes.len())
    {
        (0usize, tree.nodes.len())
    } else {
        (root_idx, root_cnt)
    };

    // Collect candidate matches: `(audio_node_id, wildcards_used, weight, probability)`
    let mut candidates: Vec<(u32, usize, u32)> = Vec::new();
    collect_tree_candidates(
        tree,
        &arg_vals,
        0,
        start_idx,
        start_cnt,
        0,
        50,
        &mut candidates,
    );

    if candidates.is_empty() {
        return None;
    }

    if tree.mode == 1 && candidates.len() > 1 {
        // Weighted mode: weight-proportional selection among matching candidates
        let total_weight: u32 = candidates.iter().map(|c| c.2).sum();
        if total_weight > 0 {
            let draw = ((rng.next() as u64 * total_weight as u64) >> 31) as u32;
            let mut acc = 0u32;
            for &(id, _, w) in &candidates {
                acc += w;
                if draw < acc {
                    return Some(id);
                }
            }
        }
    }

    // BestMatch mode (0) or fallback: pick candidate with fewest wildcards (preserving tree order on ties)
    candidates.sort_by_key(|&(_, wildcards, _)| wildcards);
    Some(candidates[0].0)
}

#[allow(clippy::too_many_arguments)]
fn collect_tree_candidates(
    tree: &DecisionTree,
    arg_vals: &[u32],
    depth: usize,
    idx: usize,
    count: usize,
    wildcards: usize,
    parent_weight: u32,
    out: &mut Vec<(u32, usize, u32)>,
) {
    let end = (idx + count).min(tree.nodes.len());
    if idx >= end || depth >= arg_vals.len() {
        return;
    }
    let target_key = arg_vals[depth];
    let is_last = depth + 1 == arg_vals.len();

    for node in &tree.nodes[idx..end] {
        let matches_exact = target_key != 0 && node.key == target_key;
        let matches_wildcard = node.key == 0;
        if !matches_exact && !matches_wildcard {
            continue;
        }
        if node.weight == 0 || node.probability == 0 {
            continue;
        }
        let next_wildcards = wildcards + usize::from(!matches_exact);
        let eff_weight = (parent_weight * node.weight as u32) / 50.max(1);
        let packed = node.child_or_audio;
        let ch_idx = (packed & 0xFFFF) as usize;
        let ch_cnt = (packed >> 16) as usize;
        let is_leaf = is_last || ch_cnt == 0 || ch_idx >= tree.nodes.len() || ch_idx + ch_cnt > tree.nodes.len();
        if is_leaf {
            out.push((packed, next_wildcards, eff_weight.max(1)));
        } else {
            collect_tree_candidates(
                tree,
                arg_vals,
                depth + 1,
                ch_idx,
                ch_cnt,
                next_wildcards,
                eff_weight.max(1),
                out,
            );
        }
    }
}

/// Match a [`TransitionRule`] from `rules` for a transition from `src_id` to `dst_id`.
/// In Wwise (`CAkMusicTransAware`), rules are evaluated in order (or last-specific match);
/// `-1` (`0xFFFF_FFFF`) and `0` match any source or destination node, while specific IDs match
/// either the node itself or any ancestor in its music hierarchy.
pub fn match_transition_rule<'a>(
    project: &Project,
    rules: &'a [TransitionRule],
    src_id: u32,
    dst_id: u32,
) -> Option<&'a TransitionRule> {
    let matches_node = |rule_ids: &[u32], target: u32| -> (bool, bool) {
        if rule_ids.is_empty() {
            return (true, false);
        }
        for &rid in rule_ids {
            if rid == target && rid != 0 && rid != 0xFFFF_FFFF {
                return (true, true);
            }
            // Check ancestor chain
            let mut cur = target;
            let mut depth = 0;
            while cur != 0 && depth < 16 {
                if cur == rid && rid != 0xFFFF_FFFF {
                    return (true, true);
                }
                cur = project
                    .node(cur)
                    .and_then(|o| o.node_base())
                    .map_or(0, |n| n.direct_parent_id);
                depth += 1;
            }
        }
        let any = rule_ids.iter().any(|&rid| rid == 0xFFFF_FFFF || rid == 0);
        (any, false)
    };

    let mut best: Option<(&'a TransitionRule, u8)> = None;
    for rule in rules {
        let (src_ok, src_exact) = matches_node(&rule.src_ids, src_id);
        let (dst_ok, dst_exact) = matches_node(&rule.dst_ids, dst_id);
        if !src_ok || !dst_ok {
            continue;
        }
        let specificity = (u8::from(src_exact) << 1) | u8::from(dst_exact);
        // Prefer more specific rules; among equal specificity, later rules in Wwise override earlier wildcard rules
        if best.map_or(true, |(_, s)| specificity >= s) {
            best = Some((rule, specificity));
        }
    }
    best.map(|(r, _)| r)
}

/// Runtime state for a [`MusicRanSeqCntr`] playlist node (`SequenceContinuous`, `SequenceStep`,
/// `RandomContinuous`, `RandomStep`).
#[derive(Debug, Default, Clone)]
pub struct PlaylistRunnerState {
    /// Per `playlist_item_id`: next child index for sequence nodes.
    pub seq_index: HashMap<u32, usize>,
    /// Per `playlist_item_id`: recently played child indices for avoid-repeat in random nodes.
    pub recent: HashMap<u32, Vec<usize>>,
}

impl PlaylistRunnerState {
    /// Expand a [`MusicRanSeqCntr`] playlist into a sequence of `segment_id`s up to `max_segments`
    /// (for continuous playlists) or 1 segment (for step playlists).
    pub fn next_segments(
        &mut self,
        root: &[MusicPlaylistItem],
        rng: &mut Rng,
        max_segments: usize,
    ) -> Vec<u32> {
        let mut out = Vec::new();
        for item in root {
            self.collect_item(item, rng, max_segments, &mut out);
            if out.len() >= max_segments {
                break;
            }
        }
        out
    }

    fn collect_item(
        &mut self,
        item: &MusicPlaylistItem,
        rng: &mut Rng,
        max_segments: usize,
        out: &mut Vec<u32>,
    ) {
        if out.len() >= max_segments {
            return;
        }
        // Loop count: 0 = infinite (bounded by max_segments), > 0 = N iterations
        let loops: usize = if item.loop_count <= 0 {
            max_segments.max(1)
        } else {
            item.loop_count as usize
        };

        let is_leaf = item.rs_type == 0xFFFF_FFFF || item.children.is_empty();
        if is_leaf {
            if item.segment_id != 0 {
                for _ in 0..loops {
                    if out.len() >= max_segments {
                        break;
                    }
                    out.push(item.segment_id);
                }
            }
            return;
        }

        let n = item.children.len();
        for _ in 0..loops {
            if out.len() >= max_segments {
                break;
            }
            match item.rs_type {
                0 => {
                    // ContinuousSequence: play all children in order
                    for ch in &item.children {
                        self.collect_item(ch, rng, max_segments, out);
                        if out.len() >= max_segments {
                            break;
                        }
                    }
                }
                1 => {
                    // StepSequence: pick one child per play, advancing index
                    let idx_entry = self.seq_index.entry(item.playlist_item_id).or_insert(0);
                    let idx = *idx_entry % n;
                    *idx_entry = (*idx_entry + 1) % n;
                    self.collect_item(&item.children[idx], rng, max_segments, out);
                    break;
                }
                2 => {
                    // ContinuousRandom: play `n` randomly selected children per pass
                    for _ in 0..n {
                        if out.len() >= max_segments {
                            break;
                        }
                        let idx = self.pick_random_child(item, rng);
                        self.collect_item(&item.children[idx], rng, max_segments, out);
                    }
                }
                3 => {
                    // StepRandom: pick one random child per play
                    let idx = self.pick_random_child(item, rng);
                    self.collect_item(&item.children[idx], rng, max_segments, out);
                    break;
                }
                _ => {
                    for ch in &item.children {
                        self.collect_item(ch, rng, max_segments, out);
                        if out.len() >= max_segments {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn pick_random_child(&mut self, item: &MusicPlaylistItem, rng: &mut Rng) -> usize {
        let n = item.children.len();
        if n <= 1 {
            return 0;
        }
        let avoid = (item.avoid_repeat_count as usize).min(n - 1);
        let history = self.recent.entry(item.playlist_item_id).or_default();
        let candidates: Vec<usize> = (0..n).filter(|i| !history.contains(i)).collect();
        let pool = if candidates.is_empty() { (0..n).collect::<Vec<_>>() } else { candidates };

        let chosen = if item.is_using_weight {
            let total_w: u64 = pool.iter().map(|&i| item.children[i].weight.max(1) as u64).sum();
            let draw = ((rng.next() as u64) * total_w) >> 31;
            let mut acc = 0u64;
            let mut pick = pool[0];
            for &i in &pool {
                acc += item.children[i].weight.max(1) as u64;
                if draw < acc {
                    pick = i;
                    break;
                }
            }
            pick
        } else {
            let k = ((rng.next() as u64 * pool.len() as u64) >> 31) as usize;
            pool[k.min(pool.len() - 1)]
        };

        if avoid > 0 {
            history.push(chosen);
            if history.len() > avoid {
                history.remove(0);
            }
        }
        chosen
    }
}

/// Look up stingers attached to `node_id` or its music hierarchy ancestors matching `trigger_id`.
pub fn find_stingers(project: &Project, node_id: u32, trigger_id: u32) -> Vec<(u32, Stinger)> {
    let mut out = Vec::new();
    let mut cur = node_id;
    let mut depth = 0;
    while cur != 0 && depth < 16 {
        let Some(o) = project.node(cur) else { break };
        let stingers = match &o.body {
            HircBody::MusicSegment(s) => &s.music.stingers[..],
            HircBody::MusicRanSeqCntr(r) => &r.trans.music.stingers[..],
            HircBody::MusicSwitchCntr(s) => &s.trans.music.stingers[..],
            _ => &[],
        };
        for st in stingers {
            if st.trigger_id == trigger_id && st.segment_id != 0 {
                out.push((cur, st.clone()));
            }
        }
        cur = o.node_base().map_or(0, |n| n.direct_parent_id);
        depth += 1;
    }
    out
}

/// Compute fade-in / fade-out parameters in samples from a [`MusicFade`] and anchor sample.
pub fn music_fade_window(fade: &MusicFade, anchor_sample: u64) -> Option<(u64, u64, u32)> {
    if fade.transition_time <= 0 {
        return None;
    }
    let len = ms_to_samples_u64(fade.transition_time as f64);
    if len == 0 {
        return None;
    }
    let off = ms_to_samples_i64(fade.fade_offset as f64);
    let start = (anchor_sample as i64 + off).max(0) as u64;
    Some((start, len, fade.fade_curve))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnk::hirc::{DecisionNode, GraphPoint, MusicMarker, MusicNode, NodeBase, TrackSwitch};

    #[test]
    fn segment_bounds_and_sync_points() {
        let seg = MusicSegment {
            music: MusicNode {
                flags: 0,
                node: NodeBase::default(),
                children: vec![],
                meter: MeterInfo {
                    grid_period: 250.0,
                    grid_offset: 0.0,
                    tempo: 120.0, // 1 beat = 500 ms (24000 samples), 1 bar (4/4) = 2000 ms (96000 samples)
                    time_sig_num_beats: 4,
                    time_sig_beat_value: 4,
                    override_flag: 0,
                },
                stingers: vec![],
            },
            duration_ms: 3000.0,
            markers: vec![
                MusicMarker { id: 43_573_010, position_ms: 500.0, name: String::new() }, // entry at 24000
                MusicMarker { id: 9999, position_ms: 1250.0, name: String::new() },       // user marker at 60000
                MusicMarker { id: 1_539_036_744, position_ms: 2500.0, name: String::new() }, // exit at 120000
            ],
        };
        let b = SegmentBounds::from_segment(&seg);
        assert_eq!(b.entry_samples, 24_000);
        assert_eq!(b.exit_samples, 120_000);
        assert_eq!(b.duration_samples, 144_000);
        assert_eq!(b.active_len_samples(), 96_000);

        // At sample 30000 (inside first beat after entry 24000):
        // Next beat is 24000 + 24000 = 48000; Next bar is 24000 + 96000 = 120000;
        // Next grid (250 ms = 12000 samples) is 24000 + 12000 = 36000;
        // Next user marker is 60000; Exit marker is 120000; ExitNever is None.
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::IMMEDIATE, 0), Some(30_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::NEXT_GRID, 0), Some(36_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::NEXT_BEAT, 0), Some(48_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::NEXT_BAR, 0), Some(120_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::NEXT_USER_MARKER, 0), Some(60_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::EXIT_MARKER, 0), Some(120_000));
        assert_eq!(compute_sync_point(&seg, None, 0, 30_000, sync_type::EXIT_NEVER, 0), None);
    }

    #[test]
    fn track_clip_trimming_loop_and_automation() {
        // 1000 ms source (48000 frames at 48 kHz), scheduled at play_at = 100 ms,
        // begin_trim_offset = 200 ms, end_trim_offset = -300 ms -> active [300 ms .. 800 ms] (500 ms = 24000 samples),
        // reading source frames [9600 .. 33600].
        let clip = TrackSrcInfo {
            track_id: 0,
            source_id: 10,
            event_id: 0,
            play_at: 100.0,
            begin_trim_offset: 200.0,
            end_trim_offset: -300.0,
            src_duration: 1000.0,
        };
        let autos = vec![
            ClipAutomation {
                clip_index: 0,
                auto_type: clip_auto::FADE_IN,
                points: vec![
                    GraphPoint { from: 0.0, to: 0.0, interp: 4 },
                    GraphPoint { from: 0.25, to: 1.0, interp: 4 },
                ],
            },
            ClipAutomation {
                clip_index: 0,
                auto_type: clip_auto::VOLUME,
                points: vec![
                    GraphPoint { from: 0.0, to: -0.5, interp: 4 },
                    GraphPoint { from: 0.5, to: -0.5, interp: 4 },
                ],
            },
        ];
        let sched = schedule_track_clip(0, &clip, 48_000, 48_000, 1_000, 0.0, 2000.0, &autos).unwrap();
        assert_eq!(sched.start_sample, 1_000 + 14_400); // 300 ms
        assert_eq!(sched.end_sample, 1_000 + 38_400);   // 800 ms
        assert_eq!(sched.pcm_segments, vec![(9_600, 33_600)]);

        // At midpoint of fade-in (t = 0.125 s = 6000 samples after clip_origin_sample):
        // FadeIn = 0.5, Volume (-0.5 normalized) = 0.5 -> combined gain = 0.25
        let (g_mid, _, _) = eval_clip_automations(&sched.automations, sched.clip_origin_sample, sched.clip_origin_sample + 6_000);
        assert!((g_mid - 0.25).abs() < 1e-3, "expected 0.25, got {}", g_mid);

        // After fade-in completes (t = 0.30 s = 14400 samples):
        // FadeIn = 1.0, Volume = 0.5 -> combined gain = 0.5
        let (g_full, _, _) = eval_clip_automations(&sched.automations, sched.clip_origin_sample, sched.clip_origin_sample + 14_400);
        assert!((g_full - 0.5).abs() < 1e-3, "expected 0.5, got {}", g_full);
    }

    #[test]
    fn sub_track_selection_modes() {
        let mut gs = GameState::default();
        gs.set_switch(77, 20);
        let mut rng = Rng::new(42);
        let mut seq = HashMap::new();

        // SequenceStep (track_type = 2) cycles 0, 1, 2, 0
        let seq_track = MusicTrack {
            flags: 0,
            sources: vec![],
            playlist: vec![],
            num_sub_track: 3,
            clip_automation: vec![],
            node: NodeBase::default(),
            track_type: 2,
            switch: None,
            look_ahead_time: 0,
        };
        assert_eq!(select_sub_track(500, &seq_track, &gs, &mut rng, &mut seq), Some(0));
        assert_eq!(select_sub_track(500, &seq_track, &gs, &mut rng, &mut seq), Some(1));
        assert_eq!(select_sub_track(500, &seq_track, &gs, &mut rng, &mut seq), Some(2));
        assert_eq!(select_sub_track(500, &seq_track, &gs, &mut rng, &mut seq), Some(0));

        // Switch (track_type = 3) picks sub-track matching switch_assoc
        let sw_track = MusicTrack {
            track_type: 3,
            num_sub_track: 2,
            switch: Some(TrackSwitch {
                group_type: 0,
                group_id: 77,
                default_switch: 10,
                switch_assoc: vec![10, 20],
                src_fade: MusicFade::default(),
                sync_type: 0,
                cue_filter_hash: 0,
                dest_fade: MusicFade::default(),
            }),
            ..seq_track
        };
        assert_eq!(select_sub_track(501, &sw_track, &gs, &mut rng, &mut seq), Some(1));
    }

    #[test]
    fn decision_tree_depth_and_wildcard_resolution() {
        // Depth-2 DecisionTree:
        // Arg 0: StateGroup 10, Arg 1: SwitchGroup 20
        // Root (node 0): children at idx 1, count 2 (nodes 1 and 2)
        // Node 1 (key = 0 wildcard): children at idx 3, count 1 -> Node 3 (key = 0 wildcard -> audio 1000)
        // Node 2 (key = 101 exact): children at idx 4, count 2 -> Node 4 (key = 0 -> audio 2000), Node 5 (key = 202 -> audio 2002)
        let tree = DecisionTree {
            mode: 0, // BestMatch
            arguments: vec![(10, 1), (20, 0)],
            nodes: vec![
                DecisionNode { key: 0, child_or_audio: 1 | (2 << 16), weight: 50, probability: 100 },
                DecisionNode { key: 0, child_or_audio: 3 | (1 << 16), weight: 50, probability: 100 },
                DecisionNode { key: 101, child_or_audio: 4 | (2 << 16), weight: 50, probability: 100 },
                DecisionNode { key: 0, child_or_audio: 1000, weight: 50, probability: 100 },
                DecisionNode { key: 0, child_or_audio: 2000, weight: 50, probability: 100 },
                DecisionNode { key: 202, child_or_audio: 2002, weight: 50, probability: 100 },
            ],
        };
        let mut gs = GameState::default();
        let mut rng = Rng::new(1);

        // Both unset -> falls back to (0, 0) wildcard -> 1000
        assert_eq!(resolve_decision_tree(&tree, &gs, &mut rng), Some(1000));

        // State 10 = 101, Switch 20 unset -> (101, 0) -> 2000 (1 wildcard beats 2 wildcards)
        gs.set_state(10, 101);
        assert_eq!(resolve_decision_tree(&tree, &gs, &mut rng), Some(2000));

        // State 10 = 101, Switch 20 = 202 -> exact match (0 wildcards) -> 2002
        gs.set_switch(20, 202);
        assert_eq!(resolve_decision_tree(&tree, &gs, &mut rng), Some(2002));
    }
}

