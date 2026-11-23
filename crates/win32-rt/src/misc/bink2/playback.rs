//! Playback transport, frame pacing, keyframe seeking, summary/realtime
//! telemetry, and per-track audio controls/PCM extraction for `bink2w64.dll`.

use super::audio as bink2_audio;
use super::player::{decode_frame_into_buffers, ensure_frame_packet_loaded, BinkInstance};
use super::types::{
    BinkRealtime, BinkSummary, BinkTrack, BINKBGIORESUME, BINKBGIOSUSPEND,
};
use std::ffi::c_void;
use std::time::{Duration, Instant};

// ============================================================================
// Telemetry & Service
// ============================================================================

pub(crate) unsafe fn get_summary(bink: *mut c_void, sum: *mut BinkSummary) {
    if bink.is_null() || sum.is_null() {
        return;
    }
    let inst = &*(bink as *const BinkInstance);
    let width = inst.rd_u32(0x000);
    let height = inst.rd_u32(0x004);
    let frames = inst.rd_u32(0x008).max(1);
    let frame_num = inst.rd_u32(0x00c);
    let fps_num = inst.rd_u32(0x014).max(1);
    let fps_den = inst.rd_u32(0x018).max(1);
    let total_time = ((frames as u64) * (fps_den as u64) * 1000 / (fps_num as u64)) as u32;
    let total_played_frames = inst.played_frames.max(frame_num);
    let avg_data_rate = if total_time > 0 {
        let total_bytes = inst.total_bytes_read.max(inst.rd_u64(0x0d0));
        ((total_bytes.saturating_mul(1000)) / (total_time as u64)).min(u32::MAX as u64) as u32
    } else {
        0
    };
    let io_mem = inst
        .io_buffer
        .len()
        .max(inst.rd_u64(0x0d8) as usize)
        .min(u32::MAX as usize) as u32;

    let summary = BinkSummary {
        width,
        height,
        total_time,
        file_frame_rate: fps_num,
        file_frame_rate_div: fps_den,
        frame_rate: fps_num,
        frame_rate_div: fps_den,
        total_open_time: 1,
        total_frames: frames,
        total_played_frames,
        total_blit_time: (inst.total_blit_us / 1000) as u32,
        total_read_time: (inst.total_read_us / 1000) as u32,
        total_video_decomp_time: (inst.total_video_decomp_us / 1000) as u32,
        total_audio_decomp_time: (inst.total_audio_decomp_us / 1000) as u32,
        average_data_rate: avg_data_rate,
        total_io_memory: io_mem,
        highest_1sec_rate: inst.rd_u64(0x0a8) as u32,
        ..BinkSummary::default()
    };
    std::ptr::write_unaligned(sum, summary);
}

pub(crate) unsafe fn get_realtime(bink: *mut c_void, run: *mut BinkRealtime, frame_window: u32) {
    if bink.is_null() || run.is_null() {
        return;
    }
    let inst = &*(bink as *const BinkInstance);
    let frame_num = inst.rd_u32(0x00c);
    let fps_num = inst.rd_u32(0x014).max(1);
    let fps_den = inst.rd_u32(0x018).max(1);
    let want = frame_window.max(1) as usize;
    let count = inst.frame_timings.len().min(want);

    let mut video_us = 0u64;
    let mut audio_us = 0u64;
    let mut read_us = 0u64;
    let mut blit_us = 0u64;
    let mut bytes = 0u64;
    if count > 0 {
        let start = inst.frame_timings.len() - count;
        for sample in inst.frame_timings.iter().skip(start) {
            video_us = video_us.saturating_add(sample.video_us);
            audio_us = audio_us.saturating_add(sample.audio_us);
            read_us = read_us.saturating_add(sample.read_us);
            blit_us = blit_us.saturating_add(sample.blit_us);
            bytes = bytes.saturating_add(sample.packet_bytes);
        }
    }

    let nominal_ms = (count as u64)
        .saturating_mul(fps_den as u64)
        .saturating_mul(1000)
        / (fps_num as u64);
    let work_ms = video_us
        .saturating_add(audio_us)
        .saturating_add(read_us)
        .saturating_add(blit_us)
        / 1000;
    let frames_time = nominal_ms.max(work_ms).min(u32::MAX as u64) as u32;
    let data_rate = if frames_time > 0 {
        (bytes.saturating_mul(1000) / (frames_time as u64)).min(u32::MAX as u64) as u32
    } else {
        0
    };
    let read_buf_size = inst
        .io_buffer
        .len()
        .max(inst.rd_u64(0x0d8) as usize)
        .min(u32::MAX as usize) as u32;
    let read_buf_used = inst.cached_packet_len.min(read_buf_size as usize) as u32;

    let rt = BinkRealtime {
        frame_num,
        frame_rate: fps_num,
        frame_rate_div: fps_den,
        frames: count as u32,
        frames_time,
        frames_video_decomp_time: (video_us / 1000).min(u32::MAX as u64) as u32,
        frames_audio_decomp_time: (audio_us / 1000).min(u32::MAX as u64) as u32,
        frames_read_time: (read_us / 1000).min(u32::MAX as u64) as u32,
        frames_idle_read_time: 0,
        frames_thread_read_time: 0,
        frames_blit_time: (blit_us / 1000).min(u32::MAX as u64) as u32,
        read_buffer_size: read_buf_size,
        read_buffer_used: read_buf_used,
        frames_data_rate: data_rate,
    };
    std::ptr::write_unaligned(run, rt);
}

pub(crate) unsafe fn get_rects(bink: *mut c_void) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    if !inst.video_on {
        return 0;
    }
    1
}

pub(crate) unsafe fn service(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    // Keep audio stream paused state consistent with the BINK header.
    let paused = inst.rd_u32(0x11c) != 0;
    inst.audio_player.set_paused(paused);
}

pub(crate) unsafe fn control_background_io(bink: *mut c_void, control: u32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    match control & 0xff {
        BINKBGIOSUSPEND => {
            inst.bg_io_suspended = inst.bg_io_suspended.saturating_add(1);
        }
        BINKBGIORESUME => {
            inst.bg_io_suspended = inst.bg_io_suspended.saturating_sub(1);
        }
        _ => {}
    }
    i32::from(inst.bg_io_suspended > 0)
}

pub(crate) unsafe fn set_video_on_off(bink: *mut c_void, onoff: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.video_on = onoff != 0;
    i32::from(inst.video_on)
}

// ============================================================================
// Transport, Pacing & Seeking
// ============================================================================

pub(crate) unsafe fn next_frame(bink: *mut c_void) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let cur = inst.rd_u32(0x00c).max(1);
    let will_loop = inst.rd_u32(0x3b8) != 0 || (inst.rd_u32(0x020) & 0x80) != 0;

    inst.wr_u32(0x010, cur); // LastFrameNum = FrameNum
    if cur < frames {
        inst.wr_u32(0x00c, cur + 1);
    } else if will_loop {
        inst.wr_u32(0x00c, 1);
    } else {
        inst.wr_u32(0x00c, frames);
    }
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.frame_advance_epoch = inst.frame_advance_epoch.wrapping_add(1);
}

pub(crate) unsafe fn wait(bink: *mut c_void) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    if inst.rd_u32(0x01c) != 0 {
        return 0;
    }
    if inst.rd_u32(0x11c) != 0 {
        // Paused
        return 1;
    }
    if inst.played_frames == 0 {
        return 0;
    }
    let Some(start) = inst.playback_start else {
        return 0;
    };
    let fps_num = inst.rd_u32(0x014);
    let fps_den = inst.rd_u32(0x018).max(1);
    if fps_num == 0 {
        return 0;
    }
    let elapsed_frames = inst.played_frames.saturating_sub(inst.playback_base_frame) as u64;
    let due_us = elapsed_frames
        .saturating_mul(fps_den as u64)
        .saturating_mul(1_000_000)
        / (fps_num as u64);
    let elapsed = start.elapsed();
    let due = Duration::from_micros(due_us);
    if elapsed >= due {
        let frame_dur_us = (fps_den as u64).saturating_mul(1_000_000) / (fps_num as u64);
        if elapsed > due + Duration::from_micros(frame_dur_us.saturating_mul(4)) {
            // Re-anchor clock after a long stall so we don't burst frames
            inst.playback_start = Some(Instant::now());
            inst.playback_base_frame = inst.played_frames;
        }
        0
    } else {
        1
    }
}

pub(crate) unsafe fn pause(bink: *mut c_void, pause: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let paused = if pause != 0 { 1u32 } else { 0u32 };
    inst.wr_u32(0x11c, paused);
    inst.playback_start = None;
    inst.playback_base_frame = inst.played_frames;
    inst.audio_player.set_paused(paused != 0);
    paused as i32
}

pub(crate) unsafe fn set_will_loop(bink: *mut c_void, onoff: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let open_flags = inst.rd_u32(0x020);
    if onoff != 0 {
        inst.wr_u32(0x020, open_flags | 0x80);
        inst.wr_u32(0x3b8, 1);
    } else {
        inst.wr_u32(0x020, open_flags & !0x80);
        inst.wr_u32(0x3b8, 0);
    }
}

pub(crate) unsafe fn get_key_frame(bink: *mut c_void, frame: u32, flags: i32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let f = frame.clamp(1, frames);
    let exclude_exact = (flags as u8 & 0x80) != 0;
    let is_key = |idx_1: u32| -> bool {
        let i = idx_1.saturating_sub(1) as usize;
        inst.frame_offsets
            .get(i)
            .map(|&v| (v & 1) != 0)
            .unwrap_or(idx_1 == 1)
    };
    if !exclude_exact && is_key(f) {
        return f;
    }
    match flags & 0x7f {
        0 => {
            // Previous keyframe
            let mut k = f.saturating_sub(1);
            while k > 1 && !is_key(k) {
                k -= 1;
            }
            k.max(1)
        }
        1 => {
            // Next keyframe
            let mut k = f + 1;
            while k <= frames {
                if is_key(k) {
                    return k;
                }
                k += 1;
            }
            0
        }
        2 => {
            // Nearest keyframe
            let mut prev = f as i64 - 1;
            let mut next = f as i64 + 1;
            loop {
                if prev >= 1 && is_key(prev as u32) {
                    return prev as u32;
                }
                if next <= frames as i64 && is_key(next as u32) {
                    return next as u32;
                }
                if prev < 1 && next > frames as i64 {
                    return 0;
                }
                prev -= 1;
                next += 1;
            }
        }
        _ => 0,
    }
}

pub(crate) unsafe fn goto(bink: *mut c_void, frame: u32, flags: i32) {
    if bink.is_null() {
        return;
    }
    let quick = (flags & 1) != 0;
    let kf = if quick {
        1
    } else {
        get_key_frame(bink, frame, 0).max(1)
    };
    let inst = &mut *(bink as *mut BinkInstance);
    let frames = inst.rd_u32(0x008).max(1);
    let cur = inst.rd_u32(0x00c).max(1);
    let target = frame.clamp(1, frames);

    inst.wr_u32(0x010, cur);
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.audio_player.flush_and_reset();

    if !quick {
        let has_io = !inst.from_memory_ptr.is_null()
            || inst.is_user_io
            || inst.user_file_slot != 0
            || inst.host_file.is_some();
        if kf < target && inst.rd_u64(0x0b0) != 0 && has_io {
            let start = if cur >= kf && cur < target { cur } else { kf };
            for f in start..target {
                inst.wr_u32(0x00c, f);
                inst.last_decoded_frame = 0;
                inst.cached_packet_frame = 0;
                decode_frame_into_buffers(inst, false);
            }
        }
    }

    inst.wr_u32(0x00c, target);
    inst.last_decoded_frame = 0;
    inst.cached_packet_frame = 0;
    inst.seek_epoch = inst.seek_epoch.wrapping_add(1);
    inst.frame_advance_epoch = inst.frame_advance_epoch.wrapping_add(1);
    inst.playback_start = None;
    inst.playback_base_frame = inst.played_frames;
}

// ============================================================================
// Audio Track Controls & PCM Extraction
// ============================================================================

pub(crate) unsafe fn set_volume(bink: *mut c_void, track_id: u32, volume: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_volume(track_id, volume);
}

pub(crate) unsafe fn set_pan(bink: *mut c_void, track_id: u32, pan: i32) {
    if bink.is_null() {
        return;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_pan(track_id, pan);
}

pub(crate) unsafe fn set_speaker_volumes(
    bink: *mut c_void,
    track_id: u32,
    speaker_indexes: *const u32,
    volumes: *const i32,
    total: u32,
) {
    if bink.is_null() || volumes.is_null() || total == 0 {
        return;
    }
    let count = (total as usize).min(16);
    let mut left_max = 0i32;
    let mut right_max = 0i32;
    for i in 0..count {
        let vol =
            std::ptr::read_unaligned(volumes.add(i)).clamp(0, bink2_audio::BINK_MAX_VOLUME);
        let spk = if !speaker_indexes.is_null() {
            std::ptr::read_unaligned(speaker_indexes.add(i))
        } else {
            i as u32
        };
        if count == 1 {
            left_max = left_max.max(vol);
            right_max = right_max.max(vol);
        } else {
            match spk {
                // FL, BL, SL
                0 | 4 | 6 => left_max = left_max.max(vol),
                // FR, BR, SR
                1 | 5 | 7 => right_max = right_max.max(vol),
                // FC, LFE, or higher center/mono speakers
                _ => {
                    left_max = left_max.max(vol);
                    right_max = right_max.max(vol);
                }
            }
        }
    }
    let eff_vol = left_max.max(right_max);
    let eff_pan = if eff_vol == 0 {
        bink2_audio::BINK_CENTER_PAN
    } else if left_max >= right_max {
        (((right_max as i64) * (bink2_audio::BINK_CENTER_PAN as i64)) / (left_max as i64)) as i32
    } else {
        bink2_audio::BINK_MAX_PAN
            - ((((left_max as i64) * (bink2_audio::BINK_CENTER_PAN as i64)) / (right_max as i64))
                as i32)
    };
    let inst = &mut *(bink as *mut BinkInstance);
    inst.audio_player.set_volume(track_id, eff_vol);
    inst.audio_player.set_pan(track_id, eff_pan);
}

pub(crate) unsafe fn set_sound_on_off(bink: *mut c_void, onoff: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &mut *(bink as *mut BinkInstance);
    let _ = inst.audio_player.set_sound_on(onoff != 0);
    i32::from(inst.audio_player.is_sound_on())
}

pub(crate) unsafe fn get_track_max_size(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.max_size)
}

pub(crate) unsafe fn get_track_id(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.track_id)
}

pub(crate) unsafe fn get_track_type(bink: *mut c_void, track_index: u32) -> u32 {
    if bink.is_null() {
        return 0;
    }
    let inst = &*(bink as *const BinkInstance);
    inst.audio_tracks
        .get(track_index as usize)
        .map_or(0, |t| t.raw_type)
}

pub(crate) unsafe fn open_track(bink: *mut c_void, track_index: u32) -> *mut BinkTrack {
    if bink.is_null() {
        return std::ptr::null_mut();
    }
    let inst = &*(bink as *const BinkInstance);
    let Some(&info) = inst.audio_tracks.get(track_index as usize) else {
        return std::ptr::null_mut();
    };
    let fourcc = inst.rd_u32(0x118);
    let version_b = fourcc.to_le_bytes() == *b"BIKb";
    let sample_rate = if info.sample_rate >= 300 {
        info.sample_rate
    } else {
        44_100
    };
    let channels = info.channels();
    let decoder =
        bink2_audio::BinkAudioDecoder::new(sample_rate, channels, info.use_dct(), version_b);
    let default_max = (decoder.output_frames_per_block() * (channels as usize) * 2) as u32;
    let max_size = if info.max_size > 0 {
        info.max_size
    } else {
        default_max
    };
    let track = Box::new(BinkTrack {
        frequency: sample_rate,
        bits: 16,
        channels: channels as u32,
        max_size,
        bink,
        sndcomp: 0,
        track_index: track_index as i32,
        _pad: 0,
        decoder,
        last_frame_epoch: u64::MAX,
        seek_epoch: inst.seek_epoch,
        pcm_scratch: Vec::new(),
    });
    Box::into_raw(track)
}

pub(crate) unsafe fn close_track(track: *mut BinkTrack) {
    if track.is_null() {
        return;
    }
    drop(Box::from_raw(track));
}

pub(crate) unsafe fn get_track_data(track: *mut BinkTrack, dest: *mut c_void) -> u32 {
    if track.is_null() || dest.is_null() {
        return 0;
    }
    let tr = &mut *track;
    if tr.bink.is_null() || tr.track_index < 0 {
        return 0;
    }
    let inst = &mut *(tr.bink as *mut BinkInstance);

    if tr.seek_epoch != inst.seek_epoch {
        tr.decoder.reset();
        tr.last_frame_epoch = u64::MAX;
        tr.seek_epoch = inst.seek_epoch;
    }
    if tr.last_frame_epoch == inst.frame_advance_epoch {
        return 0;
    }

    let cur_frame = inst.rd_u32(0x00c).max(1);
    if !ensure_frame_packet_loaded(inst, cur_frame) {
        return 0;
    }
    let track_idx = tr.track_index as usize;
    let Some(&(start, end)) = inst.cached_track_spans.get(track_idx) else {
        return 0;
    };
    tr.last_frame_epoch = inst.frame_advance_epoch;

    if end <= start || end > inst.cached_packet_len {
        return 0;
    }
    let pkt = &inst.frame_packet_buf[start..end];
    let n_samples = tr.decoder.decode_packet_i16(pkt, &mut tr.pcm_scratch);
    if n_samples == 0 {
        return 0;
    }

    let max_samples = if tr.max_size > 0 {
        (tr.max_size as usize) / 2
    } else {
        n_samples
    };
    let copy_samples = n_samples.min(max_samples);
    let dst_bytes = dest as *mut u8;
    for (i, &sample) in tr.pcm_scratch[..copy_samples].iter().enumerate() {
        let le = sample.to_le_bytes();
        std::ptr::copy_nonoverlapping(le.as_ptr(), dst_bytes.add(i * 2), 2);
    }
    (copy_samples * 2) as u32
}
