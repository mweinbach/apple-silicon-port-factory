//! OpenAL 1.1 3D spatialization, distance attenuation, and stereo software mixer.

use super::state::{
    clamp_f32, lock_state, BufferData, Listener, Source, AL_EXPONENT_DISTANCE,
    AL_EXPONENT_DISTANCE_CLAMPED, AL_INVERSE_DISTANCE, AL_INVERSE_DISTANCE_CLAMPED,
    AL_LINEAR_DISTANCE, AL_LINEAR_DISTANCE_CLAMPED, AL_NONE, AL_PLAYING, AL_STATIC, AL_STOPPED,
    AL_STREAMING, MIX_CHUNK_FRAMES, RING_CAPACITY_FRAMES,
};
use crate::audio::backend::{self, DeviceInfo, OutputStream, StreamShared};
use crate::audio::format::{
    vec3_cross, vec3_dot, vec3_len, vec3_normalize, vec3_sub, Vec3,
};
use crate::audio::ring::Ring;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Host mixer thread + [`OutputStream`] bound to a single OpenAL context.
pub(super) struct MixerRuntime {
    quit: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    stream: Option<OutputStream>,
}

impl MixerRuntime {
    pub(super) fn start(
        ctx_handle: usize,
        dev_info: &DeviceInfo,
        output_rate: u32,
    ) -> Result<Self, String> {
        let rate = output_rate.clamp(8_000, 192_000);
        let ring = Ring::new(RING_CAPACITY_FRAMES, 2);
        let shared = StreamShared::new(ring);
        let mut stream = match OutputStream::open(dev_info, rate, 2, shared.clone(), 512) {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    target: "audio",
                    "openal32: OutputStream::open failed ({e}); falling back to null output"
                );
                let null_dev = backend::null_device();
                OutputStream::open(&null_dev, rate, 2, shared.clone(), 512)?
            }
        };
        if let Err(e) = stream.start() {
            log::warn!(
                target: "audio",
                "openal32: OutputStream::start failed ({e}); falling back to null output"
            );
            drop(stream);
            let ring2 = Ring::new(RING_CAPACITY_FRAMES, 2);
            let shared2 = StreamShared::new(ring2);
            let null_dev = backend::null_device();
            let mut null_stream = OutputStream::open(&null_dev, rate, 2, shared2.clone(), 512)?;
            null_stream.start()?;
            return Self::spawn_thread(ctx_handle, null_stream, shared2);
        }
        Self::spawn_thread(ctx_handle, stream, shared)
    }

    fn spawn_thread(
        ctx_handle: usize,
        stream: OutputStream,
        shared: Arc<StreamShared>,
    ) -> Result<Self, String> {
        let io_frames = (stream.io_frames as usize).max(256);
        let target_frames = (io_frames * 3).clamp(1024, RING_CAPACITY_FRAMES - MIX_CHUNK_FRAMES);
        let quit = Arc::new(AtomicBool::new(false));
        let quit_thread = quit.clone();
        let thread = std::thread::Builder::new()
            .name("openal-mixer".into())
            .spawn(move || {
                let mut mix_buf = [0.0f32; MIX_CHUNK_FRAMES * 2];
                while !quit_thread.load(Ordering::Acquire) {
                    let mut pumped = false;
                    while !quit_thread.load(Ordering::Acquire)
                        && shared.ring.len() + MIX_CHUNK_FRAMES <= target_frames
                    {
                        render_context_frames(ctx_handle, &mut mix_buf);
                        let pushed = shared.ring.push(&mix_buf);
                        if pushed == 0 {
                            break;
                        }
                        pumped = true;
                    }
                    if !pumped || shared.ring.len() >= target_frames / 2 {
                        std::thread::sleep(Duration::from_millis(4));
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            quit,
            thread: Some(thread),
            stream: Some(stream),
        })
    }

    /// Tear down the mixer thread and CoreAudio/Null output stream.
    /// Must NEVER be called while holding `STATE`.
    pub(super) fn stop(mut self) {
        self.quit.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        drop(self.stream.take());
    }
}

impl Drop for MixerRuntime {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        drop(self.stream.take());
    }
}

fn distance_attenuation(
    model: i32,
    distance: f32,
    ref_dist: f32,
    max_dist: f32,
    rolloff: f32,
) -> f32 {
    let ref_d = ref_dist.max(0.0);
    let max_d = max_dist.max(ref_d);
    let ro = rolloff.max(0.0);
    match model {
        AL_NONE => 1.0,
        AL_INVERSE_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let denom = ref_d + ro * (distance - ref_d);
                if denom <= 1e-6 {
                    1e6
                } else {
                    ref_d / denom
                }
            }
        }
        AL_INVERSE_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                let denom = ref_d + ro * (d - ref_d);
                if denom <= 1e-6 {
                    1.0
                } else {
                    ref_d / denom
                }
            }
        }
        AL_LINEAR_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else {
                let d = distance.min(max_d);
                let span = max_d - ref_d;
                if span <= 1e-6 {
                    if d <= ref_d { 1.0 } else { 0.0 }
                } else {
                    (1.0 - ro * (d - ref_d) / span).max(0.0)
                }
            }
        }
        AL_LINEAR_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                let span = max_d - ref_d;
                if span <= 1e-6 {
                    1.0
                } else {
                    (1.0 - ro * (d - ref_d) / span).max(0.0)
                }
            }
        }
        AL_EXPONENT_DISTANCE => {
            if ro == 0.0 {
                1.0
            } else if distance <= 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                (distance / ref_d).powf(-ro)
            }
        }
        AL_EXPONENT_DISTANCE_CLAMPED => {
            if ro == 0.0 {
                1.0
            } else if ref_d <= 0.0 {
                0.0
            } else {
                let d = clamp_f32(distance, ref_d, max_d);
                (d / ref_d).powf(-ro)
            }
        }
        _ => 1.0,
    }
}

fn cone_attenuation(src: &Source, rel: Vec3, distance: f32) -> f32 {
    if src.cone_inner_angle >= 360.0 || distance <= 1e-6 {
        return 1.0;
    }
    let Some(dir) = vec3_normalize(Vec3::from_array(src.direction)) else {
        return 1.0;
    };
    // Vector from source to listener = -rel / distance.
    let s_to_l = Vec3::new(-rel.x / distance, -rel.y / distance, -rel.z / distance);
    let angle_deg = clamp_f32(vec3_dot(s_to_l, dir), -1.0, 1.0)
        .acos()
        .to_degrees()
        * 2.0;
    if angle_deg <= src.cone_inner_angle {
        1.0
    } else if angle_deg >= src.cone_outer_angle {
        src.cone_outer_gain
    } else {
        let span = (src.cone_outer_angle - src.cone_inner_angle).max(1e-6);
        let t = (angle_deg - src.cone_inner_angle) / span;
        1.0 + t * (src.cone_outer_gain - 1.0)
    }
}

fn compute_doppler_pitch(
    listener: &Listener,
    src: &Source,
    rel: Vec3,
    distance: f32,
    doppler_factor: f32,
    speed_of_sound: f32,
) -> f64 {
    if doppler_factor <= 0.0 || speed_of_sound <= 0.0 || distance <= 1e-6 {
        return 1.0;
    }
    let src_vel = Vec3::from_array(src.velocity);
    let lst_vel = Vec3::from_array(listener.velocity);
    if vec3_len(src_vel) <= 1e-6 && vec3_len(lst_vel) <= 1e-6 {
        return 1.0;
    }
    // SL = unit vector from source to listener = -rel / distance.
    let sl = Vec3::new(-rel.x / distance, -rel.y / distance, -rel.z / distance);
    let max_v = speed_of_sound / doppler_factor;
    let vls = vec3_dot(sl, lst_vel).clamp(-max_v * 0.95, max_v * 0.95);
    let vss = vec3_dot(sl, src_vel).clamp(-max_v * 0.95, max_v * 0.95);
    let num = speed_of_sound - doppler_factor * vls;
    let den = speed_of_sound - doppler_factor * vss;
    if den.abs() <= 1e-6 {
        return 1.0;
    }
    ((num / den) as f64).clamp(0.25, 4.0)
}

/// Compute `(mono_left_gain, mono_right_gain, stereo_gain, doppler_pitch)` for a source.
fn compute_spatial_params(
    listener: &Listener,
    distance_model: i32,
    doppler_factor: f32,
    speed_of_sound: f32,
    src: &Source,
) -> (f32, f32, f32, f64) {
    let min_g = src.min_gain.min(src.max_gain);
    let max_g = src.max_gain;
    let stereo_gain = clamp_f32(src.gain, min_g, max_g) * listener.gain;

    let (rel, right) = if src.source_relative {
        (Vec3::from_array(src.position), Vec3::new(1.0, 0.0, 0.0))
    } else {
        let r = vec3_sub(
            Vec3::from_array(src.position),
            Vec3::from_array(listener.position),
        );
        let fwd = vec3_normalize(Vec3::new(
            listener.orientation[0],
            listener.orientation[1],
            listener.orientation[2],
        ))
        .unwrap_or(Vec3::new(0.0, 0.0, -1.0));
        let up = vec3_normalize(Vec3::new(
            listener.orientation[3],
            listener.orientation[4],
            listener.orientation[5],
        ))
        .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
        let right = vec3_normalize(vec3_cross(fwd, up)).unwrap_or(Vec3::new(1.0, 0.0, 0.0));
        (r, right)
    };

    let dist = vec3_len(rel);
    let att = distance_attenuation(
        distance_model,
        dist,
        src.reference_distance,
        src.max_distance,
        src.rolloff_factor,
    );
    let cone = cone_attenuation(src, rel, dist);
    let mono_gain = clamp_f32(src.gain * att * cone, min_g, max_g) * listener.gain;

    let pan = if dist > 1e-6 {
        let dir = Vec3::new(rel.x / dist, rel.y / dist, rel.z / dist);
        clamp_f32(vec3_dot(dir, right), -1.0, 1.0)
    } else {
        0.0
    };
    // Constant-power stereo pan: pan=-1 -> (1, 0), pan=0 -> (0.707, 0.707), pan=+1 -> (0, 1).
    let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
    let left_gain = mono_gain * angle.cos();
    let right_gain = mono_gain * angle.sin();

    let doppler = if src.source_relative {
        1.0
    } else {
        compute_doppler_pitch(listener, src, rel, dist, doppler_factor, speed_of_sound)
    };

    (left_gain, right_gain, stereo_gain, doppler)
}

/// Mix `out_interleaved.len() / 2` stereo float32 frames for `ctx_handle`.
/// Called both by the host mixer thread and synchronously in deterministic tests.
pub(super) fn render_context_frames(ctx_handle: usize, out_interleaved: &mut [f32]) {
    out_interleaved.fill(0.0);
    let num_frames = out_interleaved.len() / 2;
    if num_frames == 0 {
        return;
    }

    let mut st = lock_state();
    let Some(ctx_ref) = st.contexts.get(&ctx_handle) else {
        return;
    };
    if ctx_ref.suspended {
        return;
    }
    let dev_handle = ctx_ref.device;
    let output_rate = ctx_ref.output_rate.max(1) as f64;
    let listener = ctx_ref.listener.clone();
    let distance_model = ctx_ref.distance_model;
    let doppler_factor = ctx_ref.doppler_factor;
    let speed_of_sound = ctx_ref.speed_of_sound;

    // Clone Arc<BufferData> table references so we can mutably borrow `st.contexts`.
    let buffers: HashMap<u32, Arc<BufferData>> = match st.devices.get(&dev_handle) {
        Some(dev) => dev.buffers.clone(),
        None => return,
    };

    let Some(ctx) = st.contexts.get_mut(&ctx_handle) else {
        return;
    };

    for src in ctx.sources.values_mut() {
        if src.state != AL_PLAYING {
            continue;
        }
        if src.queue.is_empty() {
            src.state = AL_STOPPED;
            src.processed = 0;
            src.frame_pos = 0.0;
            continue;
        }

        let (mono_l, mono_r, stereo_gain, doppler) =
            compute_spatial_params(&listener, distance_model, doppler_factor, speed_of_sound, src);

        for frame_idx in 0..num_frames {
            // Skip any 0-frame or missing buffers safely (bounded to avoid infinite loop).
            let mut skips = 0usize;
            let cur_buf = loop {
                if src.processed >= src.queue.len() {
                    if src.looping && !src.queue.is_empty() && skips <= src.queue.len() {
                        src.processed = 0;
                    } else {
                        src.state = AL_STOPPED;
                        src.frame_pos = 0.0;
                        break None;
                    }
                }
                let bid = src.queue[src.processed];
                if let Some(b) = buffers.get(&bid) {
                    if b.frames() > 0 {
                        break Some(b);
                    }
                }
                skips += 1;
                if skips > src.queue.len() {
                    src.state = AL_STOPPED;
                    src.processed = src.queue.len();
                    src.frame_pos = 0.0;
                    break None;
                }
                src.processed += 1;
                src.frame_pos = 0.0;
            };

            let Some(buf) = cur_buf else {
                break;
            };

            let total_frames = buf.frames();
            let pos = src.frame_pos.max(0.0);
            let idx0 = (pos.floor() as usize).min(total_frames - 1);
            let frac = (pos - idx0 as f64).clamp(0.0, 1.0) as f32;

            let next_sample = |ch_stereo: bool| -> (f32, f32) {
                if idx0 + 1 < total_frames {
                    if ch_stereo {
                        buf.sample_stereo(idx0 + 1)
                    } else {
                        let s = buf.sample_mono(idx0 + 1);
                        (s, s)
                    }
                } else if src.source_type == AL_STATIC && src.looping {
                    if ch_stereo {
                        buf.sample_stereo(0)
                    } else {
                        let s = buf.sample_mono(0);
                        (s, s)
                    }
                } else if src.source_type == AL_STREAMING {
                    let next_idx = if src.processed + 1 < src.queue.len() {
                        Some(src.processed + 1)
                    } else if src.looping && !src.queue.is_empty() {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(ni) = next_idx {
                        if let Some(nb) = buffers.get(&src.queue[ni]) {
                            if nb.frames() > 0 {
                                return if ch_stereo {
                                    nb.sample_stereo(0)
                                } else {
                                    let s = nb.sample_mono(0);
                                    (s, s)
                                };
                            }
                        }
                    }
                    if ch_stereo {
                        buf.sample_stereo(idx0)
                    } else {
                        let s = buf.sample_mono(idx0);
                        (s, s)
                    }
                } else if ch_stereo {
                    buf.sample_stereo(idx0)
                } else {
                    let s = buf.sample_mono(idx0);
                    (s, s)
                }
            };

            if buf.channels == 1 {
                let s0 = buf.sample_mono(idx0);
                let (s1, _) = next_sample(false);
                let s = s0 + (s1 - s0) * frac;
                out_interleaved[frame_idx * 2] += s * mono_l;
                out_interleaved[frame_idx * 2 + 1] += s * mono_r;
            } else {
                let (l0, r0) = buf.sample_stereo(idx0);
                let (l1, r1) = next_sample(true);
                let sl = l0 + (l1 - l0) * frac;
                let sr = r0 + (r1 - r0) * frac;
                out_interleaved[frame_idx * 2] += sl * stereo_gain;
                out_interleaved[frame_idx * 2 + 1] += sr * stereo_gain;
            }

            let eff_doppler = if buf.channels == 1 { doppler } else { 1.0 };
            let step = ((buf.freq as f64 / output_rate) * (src.pitch as f64) * eff_doppler)
                .max(1e-6);
            src.frame_pos += step;

            // Advance across buffer boundary (handles pitch > 1 skipping multiple frames).
            let mut cur_total = total_frames as f64;
            let mut adv_guard = 0usize;
            while src.frame_pos >= cur_total {
                if src.source_type == AL_STATIC {
                    if src.looping {
                        src.frame_pos %= cur_total;
                        break;
                    } else {
                        src.state = AL_STOPPED;
                        src.processed = src.queue.len();
                        src.frame_pos = 0.0;
                        break;
                    }
                } else {
                    src.frame_pos -= cur_total;
                    src.processed += 1;
                    if src.processed >= src.queue.len() {
                        if src.looping && !src.queue.is_empty() {
                            src.processed = 0;
                        } else {
                            src.state = AL_STOPPED;
                            src.frame_pos = 0.0;
                            break;
                        }
                    }
                    adv_guard += 1;
                    if adv_guard > src.queue.len() + 1 {
                        src.state = AL_STOPPED;
                        src.frame_pos = 0.0;
                        break;
                    }
                    let next_bid = src.queue[src.processed];
                    cur_total = buffers.get(&next_bid).map(|b| b.frames()).unwrap_or(0) as f64;
                    if cur_total <= 0.0 {
                        cur_total = 1.0;
                        src.frame_pos = 1.0;
                    }
                }
            }
            if src.state != AL_PLAYING {
                break;
            }
        }
    }

    for s in out_interleaved.iter_mut() {
        *s = clamp_f32(*s, -1.0, 1.0);
    }
}
