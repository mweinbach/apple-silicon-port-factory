//! DirectSound 8 stereo software mixer, 3D spatializer, and device/capture cores.

use super::buffer::{
    check_offset_crossed, default_primary_format, is_headless, signal_events, Buffer3dParams,
    Listener3dParams, SecondaryBufferState, DEFAULT_SPEAKER_CONFIG, DS3DMODE_DISABLE,
    DS3DMODE_HEADRELATIVE, DS3D_MAXROLLOFFFACTOR, DS3D_MINROLLOFFFACTOR, DSBCAPS_CTRL3D,
    DSBCAPS_MUTE3DATMAXDISTANCE, DSBFREQUENCY_MAX, DSBFREQUENCY_MIN, DSBPAN_CENTER, DSBPAN_LEFT,
    DSBPAN_RIGHT, DSBPN_OFFSETSTOP, DSBVOLUME_MAX, DSBVOLUME_MIN, DSSCL_NORMAL, SPEED_OF_SOUND,
};
use crate::audio::backend::{self, Backend, DeviceInfo, OutputStream, StreamControl, StreamShared};
use crate::audio::format::{
    vec3_cross, vec3_dot, vec3_len, vec3_normalize, vec3_sub, Vec3, WaveFormat,
};
use crate::audio::ring::Ring;
use std::collections::HashMap;
use std::f32::consts::FRAC_PI_4;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::time::Duration;

// ============================================================================
// Audio DSP & 3D Spatialization (using `crate::audio::format::Vec3`)
// ============================================================================

#[inline]
pub(super) fn ds_mb_to_gain(mb: i32) -> f32 {
    if mb <= DSBVOLUME_MIN {
        0.0
    } else if mb >= DSBVOLUME_MAX {
        1.0
    } else {
        10.0f32.powf((mb as f32) / 2000.0)
    }
}

#[inline]
fn ds_pan_gains(pan_mb: i32) -> (f32, f32) {
    let p = pan_mb.clamp(DSBPAN_LEFT, DSBPAN_RIGHT);
    if p < 0 {
        (1.0, ds_mb_to_gain(p))
    } else if p > 0 {
        (ds_mb_to_gain(-p), 1.0)
    } else {
        (1.0, 1.0)
    }
}

/// Evaluate DirectSound 3D spatialization: returns `(gain_left, gain_right, doppler_ratio)`.
fn compute_3d_spatial(
    buf: &Buffer3dParams,
    buf_flags: u32,
    listener: &Listener3dParams,
) -> (f32, f32, f32) {
    if buf.mode == DS3DMODE_DISABLE {
        return (1.0, 1.0, 1.0);
    }

    let rel = if buf.mode == DS3DMODE_HEADRELATIVE {
        buf.position
    } else {
        vec3_sub(buf.position, listener.position)
    };
    let dist = vec3_len(rel);

    // 1. Inverse-distance rolloff attenuation.
    let dist_gain = if (buf_flags & DSBCAPS_MUTE3DATMAXDISTANCE) != 0 && dist >= buf.max_distance {
        0.0
    } else {
        let min_d = buf.min_distance.max(1e-6);
        let d = dist.clamp(min_d, buf.max_distance.max(min_d));
        let rolloff = listener
            .rolloff_factor
            .clamp(DS3D_MINROLLOFFFACTOR, DS3D_MAXROLLOFFFACTOR);
        if rolloff <= 0.0 || d <= min_d {
            1.0
        } else {
            let eff = min_d + rolloff * (d - min_d);
            (min_d / eff).clamp(0.0, 1.0)
        }
    };

    if dist_gain == 0.0 {
        return (0.0, 0.0, 1.0);
    }

    // 2. Sound cone attenuation.
    let cone_gain = if (buf.inside_cone_angle < 360 || buf.outside_cone_angle < 360) && dist > 1e-6
    {
        let to_listener = Vec3::new(-rel.x / dist, -rel.y / dist, -rel.z / dist);
        if let Some(cone_dir) = vec3_normalize(buf.cone_orientation) {
            let full_angle_deg =
                vec3_dot(cone_dir, to_listener).clamp(-1.0, 1.0).acos().to_degrees() * 2.0;
            let inside = buf.inside_cone_angle as f32;
            let outside = (buf.outside_cone_angle.max(buf.inside_cone_angle)) as f32;
            if full_angle_deg <= inside {
                1.0
            } else if full_angle_deg >= outside || (outside - inside) < 1e-4 {
                ds_mb_to_gain(buf.cone_outside_volume)
            } else {
                let t = (full_angle_deg - inside) / (outside - inside);
                let atten_mb = (t * (buf.cone_outside_volume as f32)).round() as i32;
                ds_mb_to_gain(atten_mb)
            }
        } else {
            1.0
        }
    } else {
        1.0
    };

    // 3. Stereo panning in DirectSound's left-handed basis (+X right, +Y up, +Z forward).
    let (pan_l, pan_r) = if dist <= 1e-6 {
        (
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        )
    } else {
        let dir = Vec3::new(rel.x / dist, rel.y / dist, rel.z / dist);
        let pan_x = if buf.mode == DS3DMODE_HEADRELATIVE {
            dir.x
        } else {
            let front = vec3_normalize(listener.orient_front).unwrap_or(Vec3::new(0.0, 0.0, 1.0));
            let top = vec3_normalize(listener.orient_top).unwrap_or(Vec3::new(0.0, 1.0, 0.0));
            let right =
                vec3_normalize(vec3_cross(top, front)).unwrap_or(Vec3::new(1.0, 0.0, 0.0));
            vec3_dot(dir, right)
        };
        let theta = (pan_x.clamp(-1.0, 1.0) + 1.0) * FRAC_PI_4;
        (theta.cos(), theta.sin())
    };

    // 4. Doppler pitch ratio.
    let doppler = if listener.doppler_factor > 0.0 && dist > 1e-6 {
        let dir = Vec3::new(rel.x / dist, rel.y / dist, rel.z / dist);
        let df = listener.doppler_factor * listener.distance_factor;
        let v_l = if buf.mode == DS3DMODE_HEADRELATIVE {
            0.0
        } else {
            vec3_dot(listener.velocity, dir) * df
        };
        let v_s = vec3_dot(buf.velocity, dir) * df;
        let denom = (SPEED_OF_SOUND + v_s).max(SPEED_OF_SOUND * 0.125);
        let num = (SPEED_OF_SOUND + v_l).max(SPEED_OF_SOUND * 0.125);
        (num / denom).clamp(0.125, 8.0)
    } else {
        1.0
    };

    let gain = dist_gain * cone_gain;
    (gain * pan_l, gain * pan_r, doppler)
}

// ============================================================================
// Playback Device Core & Software Mixer
// ============================================================================

pub(super) struct DeviceMixState {
    pub(super) device: DeviceInfo,
    pub(super) cooperative_level: u32,
    pub(super) speaker_config: u32,
    pub(super) primary_format: WaveFormat,
    pub(super) primary_volume_mb: i32,
    pub(super) primary_pan_mb: i32,
    pub(super) primary_playing: bool,
    pub(super) primary_frames_mixed: u64,
    pub(super) listener_current: Listener3dParams,
    pub(super) listener_deferred: Listener3dParams,
    pub(super) buffers: HashMap<u64, SecondaryBufferState>,
}

impl DeviceMixState {
    fn new(device: DeviceInfo) -> Self {
        let mut primary_format = default_primary_format();
        if (DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&device.rate) {
            primary_format.rate = device.rate;
        }
        Self {
            device,
            cooperative_level: DSSCL_NORMAL,
            speaker_config: DEFAULT_SPEAKER_CONFIG,
            primary_format,
            primary_volume_mb: DSBVOLUME_MAX,
            primary_pan_mb: DSBPAN_CENTER,
            primary_playing: false,
            primary_frames_mixed: 0,
            listener_current: Listener3dParams::default(),
            listener_deferred: Listener3dParams::default(),
            buffers: HashMap::new(),
        }
    }
}

/// Mix `frames` stereo `f32` frames from all playing DirectSound secondary buffers
/// into `out[..frames * 2]`, advancing play cursors and returning any
/// `IDirectSoundNotify` events triggered during this block.
fn mix_dsound_frames(
    st: &mut DeviceMixState,
    frames: usize,
    out: &mut [f32],
) -> Vec<Arc<crate::handles::Event>> {
    let mut events = Vec::new();
    let total_samples = frames.saturating_mul(2);
    if frames == 0 || out.len() < total_samples {
        return events;
    }
    out[..total_samples].fill(0.0);

    let mix_rate = st.primary_format.rate.max(1) as f64;
    let listener = st.listener_current;
    let master_vol = ds_mb_to_gain(st.primary_volume_mb);
    let (master_pan_l, master_pan_r) = ds_pan_gains(st.primary_pan_mb);

    // Pair with the Release fence in Unlock() so unlocked guest writes are visible.
    std::sync::atomic::fence(Ordering::Acquire);

    for buf in st.buffers.values_mut() {
        if !buf.playing {
            continue;
        }
        let total_frames = buf.sample_mem.total_frames;
        let block_align = buf.sample_mem.format.block_align as u32;
        let buf_len = buf.sample_mem.byte_len() as u32;
        if total_frames == 0 || block_align == 0 || buf_len == 0 {
            buf.playing = false;
            buf.looping = false;
            buf.collect_stop_events(&mut events);
            continue;
        }

        let vol_gain = ds_mb_to_gain(buf.volume_mb);
        let is_3d_active =
            (buf.flags & DSBCAPS_CTRL3D) != 0 && buf.d3d_current.mode != DS3DMODE_DISABLE;
        let (gain_l, gain_r, doppler) = if is_3d_active {
            let (l3, r3, dop) = compute_3d_spatial(&buf.d3d_current, buf.flags, &listener);
            (vol_gain * l3, vol_gain * r3, dop)
        } else {
            let (pl, pr) = ds_pan_gains(buf.pan_mb);
            (vol_gain * pl, vol_gain * pr, 1.0)
        };

        let step = ((buf.frequency as f64) / mix_rate) * (doppler as f64);
        let total_frames_f = total_frames as f64;

        for f in 0..frames {
            if !buf.playing {
                break;
            }
            let pos = buf.play_pos_frames;
            let i0_raw = pos.floor() as usize;
            let i0 = i0_raw % total_frames;
            let frac = (pos - pos.floor()).clamp(0.0, 1.0) as f32;
            let i1 = if i0 + 1 < total_frames {
                i0 + 1
            } else if buf.looping {
                0
            } else {
                i0
            };

            let (l0, r0) = buf.sample_mem.read_stereo_frame(i0);
            let (l1, r1) = buf.sample_mem.read_stereo_frame(i1);
            let mut sl = l0 + (l1 - l0) * frac;
            let mut sr = r0 + (r1 - r0) * frac;
            if is_3d_active && buf.sample_mem.format.channels > 1 {
                let mono = 0.5 * (sl + sr);
                sl = mono;
                sr = mono;
            }
            if !buf.effects.is_empty() {
                for fx in &buf.effects {
                    let (nl, nr) = fx.lock().unwrap().process_sample(sl, sr, mix_rate as f32);
                    sl = nl;
                    sr = nr;
                }
            }

            out[f * 2] += sl * gain_l;
            out[f * 2 + 1] += sr * gain_r;

            let prev_byte = (i0 as u32).saturating_mul(block_align);
            let next_pos = pos + step;

            if next_pos >= total_frames_f {
                if buf.looping {
                    let wrapped_pos = next_pos.rem_euclid(total_frames_f);
                    let full_wrap = (next_pos - pos) >= total_frames_f;
                    let new_byte =
                        ((wrapped_pos.floor() as usize) % total_frames) as u32 * block_align;
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && (full_wrap
                                || check_offset_crossed(
                                    prev_byte, new_byte, buf_len, true, n.offset,
                                ))
                        {
                            events.push(n.event.clone());
                        }
                    }
                    buf.play_pos_frames = wrapped_pos;
                } else {
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && check_offset_crossed(prev_byte, buf_len, buf_len, false, n.offset)
                        {
                            events.push(n.event.clone());
                        }
                    }
                    buf.play_pos_frames = 0.0;
                    buf.playing = false;
                    buf.looping = false;
                    buf.collect_stop_events(&mut events);
                }
            } else {
                let new_byte = ((next_pos.floor() as usize) % total_frames) as u32 * block_align;
                if new_byte != prev_byte {
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && check_offset_crossed(prev_byte, new_byte, buf_len, false, n.offset)
                        {
                            events.push(n.event.clone());
                        }
                    }
                }
                buf.play_pos_frames = next_pos;
            }
        }
    }

    let ml = master_vol * master_pan_l;
    let mr = master_vol * master_pan_r;
    for f in 0..frames {
        let l = out[f * 2] * ml;
        let r = out[f * 2 + 1] * mr;
        out[f * 2] = if l.is_finite() { l } else { 0.0 };
        out[f * 2 + 1] = if r.is_finite() { r } else { 0.0 };
    }
    st.primary_frames_mixed = st.primary_frames_mixed.wrapping_add(frames as u64);

    events
}

struct MixerRuntime {
    sample_rate: u32,
    stop_flag: Arc<AtomicBool>,
    wake: Arc<(Mutex<()>, Condvar)>,
    control: StreamControl,
    mixer_thread: Option<std::thread::JoinHandle<()>>,
    stream: Option<OutputStream>,
}

impl MixerRuntime {
    fn shutdown(mut self) {
        self.stop_flag.store(true, Ordering::Release);
        {
            let _g = self.wake.0.lock().unwrap();
            self.wake.1.notify_all();
        }
        if let Some(t) = self.mixer_thread.take() {
            let _ = t.join();
        }
        self.control.stop();
        drop(self.stream.take());
    }
}

pub(super) struct DsDeviceCore {
    pub(super) initialized: AtomicBool,
    pub(super) is_ds8: bool,
    pub(super) mixer_paused: AtomicBool,
    next_buf_id: AtomicU64,
    pub(super) state: Mutex<DeviceMixState>,
    runtime: Mutex<Option<MixerRuntime>>,
}

impl DsDeviceCore {
    pub(super) fn new(device: DeviceInfo, initialized: bool, is_ds8: bool) -> Arc<Self> {
        Arc::new(Self {
            initialized: AtomicBool::new(initialized),
            is_ds8,
            mixer_paused: AtomicBool::new(false),
            next_buf_id: AtomicU64::new(1),
            state: Mutex::new(DeviceMixState::new(device)),
            runtime: Mutex::new(None),
        })
    }

    pub(super) fn alloc_buf_id(&self) -> u64 {
        self.next_buf_id.fetch_add(1, Ordering::Relaxed)
    }

    pub(super) fn mix_frames(&self, frames: usize, out: &mut [f32]) {
        let events = {
            let mut st = self.state.lock().unwrap();
            mix_dsound_frames(&mut st, frames, out)
        };
        signal_events(events);
    }

    pub(super) fn ensure_mixer_started(self: &Arc<Self>) {
        if self.mixer_paused.load(Ordering::Acquire) {
            return;
        }
        let (dev, sample_rate) = {
            let st = self.state.lock().unwrap();
            (st.device.clone(), st.primary_format.rate)
        };
        let mut rt_guard = self.runtime.lock().unwrap();
        if let Some(existing) = rt_guard.as_ref() {
            if existing.sample_rate == sample_rate {
                let _g = existing.wake.0.lock().unwrap();
                existing.wake.1.notify_all();
                return;
            }
            if let Some(old) = rt_guard.take() {
                drop(rt_guard);
                old.shutdown();
                rt_guard = self.runtime.lock().unwrap();
            }
        }
        if rt_guard.is_some() {
            return;
        }
        match spawn_dsound_mixer(self, &dev, sample_rate) {
            Ok(rt) => *rt_guard = Some(rt),
            Err(e) => {
                log::warn!(target: "audio", "dsound: failed to spawn mixer runtime: {e}");
            }
        }
    }

    fn shutdown_runtime(&self) {
        let rt = self.runtime.lock().unwrap().take();
        if let Some(r) = rt {
            r.shutdown();
        }
    }

    pub(super) fn restart_mixer_if_running(self: &Arc<Self>, new_rate: u32) {
        let running = {
            let rt = self.runtime.lock().unwrap();
            rt.as_ref().is_some_and(|r| r.sample_rate != new_rate)
        };
        if running {
            self.shutdown_runtime();
            self.ensure_mixer_started();
        }
    }
}

impl Drop for DsDeviceCore {
    fn drop(&mut self) {
        self.shutdown_runtime();
    }
}

fn spawn_dsound_mixer(
    core: &Arc<DsDeviceCore>,
    requested_dev: &DeviceInfo,
    sample_rate: u32,
) -> Result<MixerRuntime, String> {
    let dev = if is_headless() || backend::backend() == Backend::Null {
        backend::null_device()
    } else {
        requested_dev.clone()
    };
    let channels: usize = 2;
    let io_request: u32 = 512;
    let quantum_frames = (sample_rate as usize / 100).clamp(64, 2048);
    let ring_cap = (2 * backend::MAX_SLICE as usize)
        .max(sample_rate as usize / 4)
        .max(quantum_frames * 8);
    let shared = StreamShared::new(Ring::new(ring_cap, channels));

    let mut stream =
        match OutputStream::open(&dev, sample_rate, channels as u16, shared.clone(), io_request) {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    target: "audio",
                    "dsound: CoreAudio open failed on '{}' ({e}); falling back to Null backend",
                    dev.name
                );
                OutputStream::open(
                    &backend::null_device(),
                    sample_rate,
                    channels as u16,
                    shared.clone(),
                    quantum_frames as u32,
                )?
            }
        };

    let io_frames = stream.io_frames.max(128) as usize;
    let mut control = stream.control();
    if let Err(e) = control.start() {
        log::warn!(
            target: "audio",
            "dsound: CoreAudio start failed ({e}); falling back to Null backend"
        );
        drop(stream);
        stream = OutputStream::open(
            &backend::null_device(),
            sample_rate,
            channels as u16,
            shared.clone(),
            quantum_frames as u32,
        )?;
        control = stream.control();
        let _ = control.start();
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let wake = Arc::new((Mutex::new(()), Condvar::new()));
    let weak_core: Weak<DsDeviceCore> = Arc::downgrade(core);
    let stop_worker = stop_flag.clone();
    let wake_worker = wake.clone();
    let shared_worker = shared.clone();
    let target_buffered = (io_frames * 2 + quantum_frames)
        .min(ring_cap / 2)
        .max(quantum_frames * 2);

    let mixer_thread = std::thread::Builder::new()
        .name("dsound-mixer".into())
        .spawn(move || {
            let mut mix_buf = vec![0.0f32; quantum_frames * channels];
            let poll_dur = Duration::from_millis(4);
            while !stop_worker.load(Ordering::Acquire) {
                let Some(eng) = weak_core.upgrade() else {
                    break;
                };
                if !eng.mixer_paused.load(Ordering::Acquire) {
                    let mut quantums = 0usize;
                    while !stop_worker.load(Ordering::Acquire)
                        && !eng.mixer_paused.load(Ordering::Acquire)
                        && shared_worker.ring.len() + quantum_frames <= target_buffered
                        && quantums < 4
                    {
                        eng.mix_frames(quantum_frames, &mut mix_buf);
                        let pushed = shared_worker.ring.push(&mix_buf);
                        if pushed < quantum_frames {
                            break;
                        }
                        quantums += 1;
                    }
                }
                if stop_worker.load(Ordering::Acquire) {
                    break;
                }
                let g = wake_worker.0.lock().unwrap();
                let _ = wake_worker.1.wait_timeout(g, poll_dur).unwrap();
            }
        })
        .map_err(|e| e.to_string())?;

    log::info!(
        target: "audio",
        "dsound: mixer started {} Hz stereo ({:?}, io={} frames, quantum={} frames)",
        sample_rate,
        stream.kind,
        stream.io_frames,
        quantum_frames
    );

    Ok(MixerRuntime {
        sample_rate,
        stop_flag,
        wake,
        control,
        mixer_thread: Some(mixer_thread),
        stream: Some(stream),
    })
}

pub(super) struct DsCaptureCore {
    pub(super) initialized: AtomicBool,
    pub(super) device: Mutex<DeviceInfo>,
}

impl DsCaptureCore {
    pub(super) fn new(device: DeviceInfo, initialized: bool) -> Arc<Self> {
        Arc::new(Self {
            initialized: AtomicBool::new(initialized),
            device: Mutex::new(device),
        })
    }
}
