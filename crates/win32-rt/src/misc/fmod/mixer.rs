//! Software stereo mixer, 3D spatializer, and CoreAudio output pump for FMOD.

use super::core::{
    FMOD_3D, FMOD_3D_HEADRELATIVE, FMOD_3D_LINEARROLLOFF, FMOD_3D_LINEARSQUAREROLLOFF,
    FMOD_ERR_OUTPUT_INIT, FMOD_LOOP_BIDI, FMOD_LOOP_NORMAL, FMOD_LOOP_OFF,
    FMOD_OUTPUTTYPE_AUTODETECT, FMOD_OUTPUTTYPE_NOSOUND, FMOD_OUTPUTTYPE_NOSOUND_NRT,
    FMOD_SOUND_TYPE_FSB, FMOD_SPEAKERMODE_5POINT1, FMOD_SPEAKERMODE_7POINT1, FMOD_SPEAKERMODE_MONO,
    FMOD_SPEAKERMODE_QUAD, FMOD_SPEAKERMODE_STEREO, FmodVector,
};
use super::dsp::DecodedSample;
use crate::audio::backend::{self, OutputStream, StreamShared};
use crate::audio::format::{vec3_cross, vec3_dot, vec3_len, vec3_normalize, vec3_sub};
use crate::audio::ring::Ring;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

/// Mixer output format (48 kHz stereo interleaved f32).
const MIX_RATE: u32 = 48_000;
const MIX_CHANNELS: u16 = 2;
const MIX_IO_FRAMES: u32 = 512;
const MIX_RING_FRAMES: usize = 4096;
pub(super) const MIX_BLOCK_FRAMES: usize = 256;
const MIX_TARGET_BUFFER_FRAMES: usize = 1536;

pub(super) fn speakermode_for_channels(channels: u32) -> u32 {
    match channels {
        1 => FMOD_SPEAKERMODE_MONO,
        2 => FMOD_SPEAKERMODE_STEREO,
        4 => FMOD_SPEAKERMODE_QUAD,
        6 => FMOD_SPEAKERMODE_5POINT1,
        8 => FMOD_SPEAKERMODE_7POINT1,
        _ => FMOD_SPEAKERMODE_STEREO,
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Listener3D {
    pub(super) pos: FmodVector,
    pub(super) vel: FmodVector,
    pub(super) forward: FmodVector,
    pub(super) up: FmodVector,
}

impl Default for Listener3D {
    fn default() -> Self {
        Self {
            pos: FmodVector::new(0.0, 0.0, 0.0),
            vel: FmodVector::new(0.0, 0.0, 0.0),
            forward: FmodVector::new(0.0, 0.0, 1.0),
            up: FmodVector::new(0.0, 1.0, 0.0),
        }
    }
}

impl Listener3D {
    /// Unit right vector in FMOD's left-handed coordinate system (`up x forward`).
    fn right_axis(&self) -> FmodVector {
        vec3_normalize(vec3_cross(self.up, self.forward))
            .unwrap_or(FmodVector::new(1.0, 0.0, 0.0))
    }
}

pub(super) struct Sound {
    pub(super) name: String,
    pub(super) sound_type: u32,
    pub(super) format: u32,
    pub(super) channels: u32,
    pub(super) sample_rate: u32,
    pub(super) bits_per_sample: u32,
    pub(super) default_frequency: f32,
    pub(super) default_priority: i32,
    pub(super) mode: u32,
    pub(super) loop_count: i32,
    pub(super) loop_start: usize,
    pub(super) loop_end: usize,
    /// Pinned raw PCM buffer in `format` (never reallocated; safe to expose via Lock/Unlock).
    pub(super) raw_pcm: Box<[u8]>,
    /// Decoded interleaved f32 samples (length = `num_frames * channels`).
    pub(super) samples: Arc<Vec<f32>>,
    pub(super) num_frames: usize,
    pub(super) min_distance: f32,
    pub(super) max_distance: f32,
    pub(super) locked: bool,
    /// Decoded sub-sounds when created from a multi-sample FSB5 container.
    pub(super) fsb_subsounds: Vec<DecodedSample>,
    /// Lazily materialized `Sound` handles for `FMOD_Sound_GetSubSound`.
    pub(super) subsound_handles: Vec<Option<usize>>,
}

pub(super) struct ChannelGroup {
    pub(super) name: String,
    pub(super) parent: Option<usize>,
    pub(super) volume: f32,
    pub(super) mute: bool,
    pub(super) paused: bool,
    pub(super) pitch: f32,
}

pub(super) struct Channel {
    pub(super) sound_id: usize,
    pub(super) group_id: usize,
    pub(super) samples: Arc<Vec<f32>>,
    pub(super) sound_channels: usize,
    pub(super) sound_sample_rate: u32,
    pub(super) sound_frame_bytes: usize,
    pub(super) num_frames: usize,
    pub(super) loop_start: usize,
    pub(super) loop_end: usize,
    pub(super) play_pos: f64,
    pub(super) frequency: f32,
    pub(super) pitch: f32,
    pub(super) volume: f32,
    pub(super) pan: f32,
    pub(super) mute: bool,
    pub(super) paused: bool,
    pub(super) stopped: bool,
    pub(super) mode: u32,
    pub(super) loop_count: i32,
    pub(super) loops_remaining: i32,
    pub(super) pos_3d: FmodVector,
    pub(super) vel_3d: FmodVector,
    pub(super) min_distance: f32,
    pub(super) max_distance: f32,
}

pub(super) struct EffectiveGroup {
    pub(super) volume: f32,
    pub(super) mute: bool,
    pub(super) paused: bool,
    pub(super) pitch: f32,
}

pub(super) fn resolve_group_chain(
    groups: &HashMap<usize, ChannelGroup>,
    start: usize,
) -> EffectiveGroup {
    let mut volume = 1.0f32;
    let mut mute = false;
    let mut paused = false;
    let mut pitch = 1.0f32;
    let mut cur = Some(start);
    let mut hops = 0usize;
    while let Some(gid) = cur {
        if hops >= 64 {
            break;
        }
        hops += 1;
        let Some(g) = groups.get(&gid) else {
            break;
        };
        volume *= g.volume;
        mute |= g.mute;
        paused |= g.paused;
        pitch *= g.pitch;
        cur = g.parent;
    }
    EffectiveGroup { volume, mute, paused, pitch }
}

pub(super) fn group_has_ancestor(
    groups: &HashMap<usize, ChannelGroup>,
    group_id: usize,
    ancestor: usize,
) -> bool {
    let mut cur = Some(group_id);
    let mut hops = 0usize;
    while let Some(gid) = cur {
        if gid == ancestor {
            return true;
        }
        if hops >= 64 {
            break;
        }
        hops += 1;
        cur = groups.get(&gid).and_then(|g| g.parent);
    }
    false
}

/// FMOD linear balance pan law: center (`0.0`) is unity `(1.0, 1.0)`,
/// `-1.0` is full left `(1.0, 0.0)`, `+1.0` is full right `(0.0, 1.0)`.
#[inline]
fn pan_gains(pan: f32) -> (f32, f32) {
    let p = pan.clamp(-1.0, 1.0);
    if p < 0.0 {
        (1.0, (1.0 + p).clamp(0.0, 1.0))
    } else if p > 0.0 {
        ((1.0 - p).clamp(0.0, 1.0), 1.0)
    } else {
        (1.0, 1.0)
    }
}

#[inline]
fn read_stereo_frame(samples: &[f32], channels: usize, frame: usize) -> (f32, f32) {
    let base = frame * channels;
    match channels {
        1 => {
            let s = samples[base];
            (s, s)
        }
        2 => (samples[base], samples[base + 1]),
        4 => (
            (samples[base] + samples[base + 2]) * 0.5,
            (samples[base + 1] + samples[base + 3]) * 0.5,
        ),
        n if n >= 6 => {
            const SQRT_HALF: f32 = core::f32::consts::FRAC_1_SQRT_2;
            let mut l = samples[base]
                + samples[base + 2] * SQRT_HALF
                + samples[base + 4] * SQRT_HALF;
            let mut r = samples[base + 1]
                + samples[base + 2] * SQRT_HALF
                + samples[base + 5] * SQRT_HALF;
            if n >= 8 {
                l += samples[base + 6] * SQRT_HALF;
                r += samples[base + 7] * SQRT_HALF;
            }
            (l, r)
        }
        _ => {
            let mut l = 0.0f32;
            let mut r = 0.0f32;
            let mut lc = 0u32;
            let mut rc = 0u32;
            for c in 0..channels {
                if c % 2 == 0 {
                    l += samples[base + c];
                    lc += 1;
                } else {
                    r += samples[base + c];
                    rc += 1;
                }
            }
            (
                if lc > 0 { l / lc as f32 } else { 0.0 },
                if rc > 0 { r / rc as f32 } else { 0.0 },
            )
        }
    }
}

pub(super) struct MixerState {
    pub(super) master_group_id: usize,
    pub(super) initialized: bool,
    pub(super) max_channels: usize,
    pub(super) output_type: u32,
    pub(super) driver_id: i32,
    pub(super) doppler_scale: f32,
    pub(super) distance_factor: f32,
    pub(super) rolloff_scale: f32,
    pub(super) listener: Listener3D,
    pub(super) sounds: HashMap<usize, Sound>,
    pub(super) groups: HashMap<usize, ChannelGroup>,
    pub(super) channels: HashMap<usize, Channel>,
}

impl MixerState {
    pub(super) fn new(master_group_id: usize) -> Self {
        let mut groups = HashMap::new();
        groups.insert(
            master_group_id,
            ChannelGroup {
                name: "Master".into(),
                parent: None,
                volume: 1.0,
                mute: false,
                paused: false,
                pitch: 1.0,
            },
        );
        Self {
            master_group_id,
            initialized: false,
            max_channels: 64,
            output_type: FMOD_OUTPUTTYPE_AUTODETECT,
            driver_id: 0,
            doppler_scale: 1.0,
            distance_factor: 1.0,
            rolloff_scale: 1.0,
            listener: Listener3D::default(),
            sounds: HashMap::new(),
            groups,
            channels: HashMap::new(),
        }
    }

    pub(super) fn mix_frames(&mut self, out: &mut [f32]) -> usize {
        let frames = out.len() / 2;
        if frames == 0 {
            return 0;
        }
        out[..frames * 2].fill(0.0);

        let listener = self.listener;
        let right = listener.right_axis();
        let distance_factor = self.distance_factor.max(1e-6);
        let rolloff_scale = self.rolloff_scale.max(0.0);
        let groups = &self.groups;

        for ch in self.channels.values_mut() {
            if ch.stopped {
                continue;
            }
            if ch.num_frames == 0 || ch.sound_channels == 0 {
                ch.stopped = true;
                continue;
            }

            let grp = resolve_group_chain(groups, ch.group_id);
            if ch.paused || grp.paused {
                continue;
            }

            let effective_pitch = (ch.pitch * grp.pitch).max(0.0);
            let step = ((ch.frequency.abs() as f64) * (effective_pitch as f64)) / (MIX_RATE as f64);
            if !step.is_finite() || step <= 0.0 {
                continue;
            }

            let muted = ch.mute || grp.mute;
            let effective_vol = ch.volume * grp.volume;
            let is_3d = (ch.mode & FMOD_3D) != 0;

            let (gain_l, gain_r) = if muted || effective_vol == 0.0 {
                (0.0, 0.0)
            } else if is_3d {
                let rel = if (ch.mode & FMOD_3D_HEADRELATIVE) != 0 {
                    ch.pos_3d
                } else {
                    vec3_sub(ch.pos_3d, listener.pos)
                };
                let dist = vec3_len(rel);
                let min_d = (ch.min_distance * distance_factor).max(1e-6);
                let max_d = (ch.max_distance * distance_factor).max(min_d);

                let atten = if dist <= min_d || rolloff_scale == 0.0 {
                    1.0
                } else if (ch.mode & FMOD_3D_LINEARROLLOFF) != 0 {
                    (1.0 - (dist - min_d) / (max_d - min_d).max(1e-6)).clamp(0.0, 1.0)
                } else if (ch.mode & FMOD_3D_LINEARSQUAREROLLOFF) != 0 {
                    let t = (1.0 - (dist - min_d) / (max_d - min_d).max(1e-6)).clamp(0.0, 1.0);
                    t * t
                } else {
                    let d = dist.min(max_d);
                    min_d / (min_d + rolloff_scale * (d - min_d))
                };

                let pan_3d = if dist > 1e-6 {
                    (vec3_dot(rel, right) / dist).clamp(-1.0, 1.0)
                } else {
                    0.0
                };
                let (pl, pr) = pan_gains((pan_3d + ch.pan).clamp(-1.0, 1.0));
                (effective_vol * atten * pl, effective_vol * atten * pr)
            } else {
                let (pl, pr) = pan_gains(ch.pan);
                (effective_vol * pl, effective_vol * pr)
            };

            let samples = &ch.samples;
            let nframes = ch.num_frames;
            let nch = ch.sound_channels;
            let loop_start = ch.loop_start.min(nframes);
            let loop_end = if ch.loop_end > loop_start && ch.loop_end <= nframes {
                ch.loop_end
            } else {
                nframes
            };
            let loop_len = (loop_end - loop_start).max(1) as f64;

            for f in 0..frames {
                while !ch.stopped {
                    let bound = if ch.loops_remaining != 0 {
                        loop_end as f64
                    } else {
                        nframes as f64
                    };
                    if ch.play_pos < bound {
                        break;
                    }
                    if ch.loops_remaining == -1 {
                        ch.play_pos =
                            (loop_start as f64) + ((ch.play_pos - loop_end as f64) % loop_len);
                    } else if ch.loops_remaining > 0 {
                        ch.loops_remaining -= 1;
                        ch.play_pos -= loop_len;
                    } else {
                        ch.play_pos = nframes as f64;
                        ch.stopped = true;
                    }
                }
                if ch.stopped {
                    break;
                }

                if !muted && (gain_l != 0.0 || gain_r != 0.0) {
                    let idx0 = (ch.play_pos.floor() as usize).min(nframes - 1);
                    let frac = (ch.play_pos - idx0 as f64) as f32;
                    let idx1 = if ch.loops_remaining != 0 && idx0 + 1 >= loop_end {
                        loop_start
                    } else if idx0 + 1 < nframes {
                        idx0 + 1
                    } else if ch.loops_remaining != 0 {
                        loop_start
                    } else {
                        idx0
                    };

                    let (l0, r0) = read_stereo_frame(samples, nch, idx0);
                    let (l1, r1) = read_stereo_frame(samples, nch, idx1);
                    let sl = l0 + (l1 - l0) * frac;
                    let sr = r0 + (r1 - r0) * frac;

                    if is_3d {
                        let mono = (sl + sr) * 0.5;
                        out[f * 2] += mono * gain_l;
                        out[f * 2 + 1] += mono * gain_r;
                    } else {
                        out[f * 2] += sl * gain_l;
                        out[f * 2 + 1] += sr * gain_r;
                    }
                }

                ch.play_pos += step;
                while !ch.stopped {
                    let bound = if ch.loops_remaining != 0 {
                        loop_end as f64
                    } else {
                        nframes as f64
                    };
                    if ch.play_pos < bound {
                        break;
                    }
                    if ch.loops_remaining == -1 {
                        ch.play_pos =
                            (loop_start as f64) + ((ch.play_pos - loop_end as f64) % loop_len);
                    } else if ch.loops_remaining > 0 {
                        ch.loops_remaining -= 1;
                        ch.play_pos -= loop_len;
                    } else {
                        ch.play_pos = nframes as f64;
                        ch.stopped = true;
                    }
                }
            }
        }

        for s in &mut out[..frames * 2] {
            *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }
        frames
    }
}

pub(super) struct AudioOutput {
    stream: OutputStream,
    stop: Arc<AtomicBool>,
    wake: Arc<(Mutex<bool>, Condvar)>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        {
            let (lock, cv) = &*self.wake;
            if let Ok(mut stopped) = lock.lock() {
                *stopped = true;
                cv.notify_all();
            }
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        self.stream.stop();
    }
}

pub(super) struct FmodSystem {
    pub(super) mixer: Arc<Mutex<MixerState>>,
    pub(super) audio: Mutex<Option<AudioOutput>>,
    pub(super) mixed_frames: Arc<AtomicU64>,
}

pub(super) fn sound_from_decoded_sample(sample: &DecodedSample, mode: u32) -> Sound {
    let mut effective_mode = mode;
    let loop_count = if (mode & FMOD_LOOP_OFF) != 0 {
        0
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 {
        -1
    } else if sample.has_loop {
        effective_mode |= FMOD_LOOP_NORMAL;
        -1
    } else {
        0
    };
    Sound {
        name: sample.name.clone(),
        sound_type: FMOD_SOUND_TYPE_FSB,
        format: sample.fmod_format,
        channels: sample.channels,
        sample_rate: sample.sample_rate,
        bits_per_sample: sample.bits_per_sample,
        default_frequency: sample.sample_rate as f32,
        default_priority: 128,
        mode: effective_mode,
        loop_count,
        loop_start: sample.loop_start,
        loop_end: sample.loop_end,
        raw_pcm: sample.raw_pcm.clone().into_boxed_slice(),
        samples: sample.samples.clone(),
        num_frames: sample.num_frames,
        min_distance: 1.0,
        max_distance: 10_000.0,
        locked: false,
        fsb_subsounds: Vec::new(),
        subsound_handles: Vec::new(),
    }
}

pub(super) fn start_audio_output(
    sys: &Arc<FmodSystem>,
    output_type: u32,
    driver_id: i32,
) -> Result<Option<AudioOutput>, i32> {
    if output_type == FMOD_OUTPUTTYPE_NOSOUND_NRT {
        return Ok(None);
    }

    let dev = if output_type == FMOD_OUTPUTTYPE_NOSOUND {
        backend::null_device()
    } else {
        let devs = backend::output_devices();
        if driver_id >= 0 {
            devs.get(driver_id as usize)
                .or_else(|| devs.first())
                .cloned()
                .unwrap_or_else(backend::null_device)
        } else {
            devs.into_iter().next().unwrap_or_else(backend::null_device)
        }
    };

    let open_and_start =
        |d: &backend::DeviceInfo| -> Result<(OutputStream, Arc<StreamShared>), String> {
            let ring = Ring::new(MIX_RING_FRAMES, MIX_CHANNELS as usize);
            let shared = StreamShared::new(ring);
            let mut s = OutputStream::open(d, MIX_RATE, MIX_CHANNELS, shared.clone(), MIX_IO_FRAMES)?;
            s.start()?;
            Ok((s, shared))
        };

    let (stream, shared) = match open_and_start(&dev) {
        Ok(pair) => pair,
        Err(e) => {
            log::warn!(
                target: "audio",
                "fmod: audio stream init/start failed on '{}' ({e}); falling back to null output",
                dev.name
            );
            let null_dev = backend::null_device();
            open_and_start(&null_dev).map_err(|_| FMOD_ERR_OUTPUT_INIT)?
        }
    };

    log::info!(
        target: "audio",
        "fmod: initialized {} mixer at {} Hz stereo ({:?})",
        dev.name,
        MIX_RATE,
        stream.kind
    );

    let stop = Arc::new(AtomicBool::new(false));
    let wake = Arc::new((Mutex::new(false), Condvar::new()));
    let stop_t = stop.clone();
    let wake_t = wake.clone();
    let mixer_arc = sys.mixer.clone();
    let mixed_counter = sys.mixed_frames.clone();

    let thread = std::thread::Builder::new()
        .name("fmod-mixer".into())
        .spawn(move || {
            let mut block = [0.0f32; MIX_BLOCK_FRAMES * 2];
            while !stop_t.load(Ordering::Acquire) {
                let mut queued = shared.ring.len();
                while queued < MIX_TARGET_BUFFER_FRAMES && !stop_t.load(Ordering::Acquire) {
                    {
                        let mut m = mixer_arc.lock().unwrap_or_else(|e| e.into_inner());
                        m.mix_frames(&mut block);
                    }
                    let pushed = shared.ring.push(&block);
                    if pushed == 0 {
                        break;
                    }
                    mixed_counter.fetch_add(pushed as u64, Ordering::Relaxed);
                    queued += pushed;
                }
                let (lock, cv) = &*wake_t;
                let guard = lock.lock().unwrap_or_else(|e| e.into_inner());
                if !*guard {
                    let _ = cv.wait_timeout(guard, Duration::from_millis(4));
                }
            }
        })
        .map_err(|_| FMOD_ERR_OUTPUT_INIT)?;

    Ok(Some(AudioOutput {
        stream,
        stop,
        wake,
        thread: Some(thread),
    }))
}
