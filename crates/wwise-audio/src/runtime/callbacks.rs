//! Win64 ABI callback structs (`AkEventCallbackInfo`, `AkDurationCallbackInfo`,
//! `AkMusicSyncCallbackInfo`, `AkSegmentInfo`), Rust/Win64 callback targets, and per-frame
//! callback collectors.

use super::{
    AK_DURATION, AK_END_OF_EVENT, AK_MUSIC_SYNC_ALL, AK_MUSIC_SYNC_BAR, AK_MUSIC_SYNC_BEAT, AK_MUSIC_SYNC_ENTRY,
    AK_MUSIC_SYNC_EXIT, AK_MUSIC_SYNC_GRID, AK_MUSIC_SYNC_USER_CUE,
};
use crate::bnk::hirc::HircBody;
use crate::engine::math::{FRAME, RATE};
use crate::engine::music::{SegmentBounds, MARKER_ENTRY_IDS, MARKER_EXIT_IDS};
use crate::engine::render::Renderer;
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::sync::Arc;

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
    pub(super) fn invoke(&self, ev: &CallbackEvent) {
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
                        let c_cue = user_cue_name.as_deref().and_then(|s| CString::new(s).ok());
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

#[derive(Clone)]
pub(super) struct PlayingTracker {
    pub(super) playing_id: u32,
    pub(super) event_id: u32,
    pub(super) game_obj_id: u64,
    pub(super) flags: u32,
    pub(super) callback: Option<EventCallbackTarget>,
    pub(super) cookie: u64,
    pub(super) duration_sent: bool,
    /// Per-(segment_id) last cursor in seconds so we can detect music sync crossings.
    pub(super) music_seg_prev_sec: HashMap<u32, f64>,
}

pub(super) struct GlobalCallbackEntry {
    pub(super) callback: GlobalCallbackTarget,
    pub(super) location_mask: u32,
    pub(super) cookie: u64,
}

pub(super) fn collect_duration_callbacks(
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

pub(super) fn collect_music_sync_callbacks(
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

pub(super) fn collect_end_of_event_callbacks(
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
