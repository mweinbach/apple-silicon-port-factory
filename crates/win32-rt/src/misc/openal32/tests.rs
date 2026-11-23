use super::mixer::render_context_frames;
use super::state::*;
use super::*;
use crate::audio::backend::{self, Backend};
use std::time::{Duration, Instant};

/// Guard that serializes OpenAL unit tests and forces the Null audio backend
/// so tests never touch physical CoreAudio hardware or race on `current_context`.
struct TestEnv {
    _audio_guard: crate::audio::AudioTestGuard,
    dev: usize,
    ctx: usize,
}

impl TestEnv {
    fn new(sync: bool) -> Self {
        let audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));
        unsafe {
            let dev = alcOpenDevice(std::ptr::null());
            assert_ne!(dev, 0, "alcOpenDevice failed");
            let attrs = [
                ALC_FREQUENCY,
                48_000,
                ALC_SYNC,
                if sync { 1 } else { 0 },
                0,
            ];
            let ctx = alcCreateContext(dev, attrs.as_ptr());
            assert_ne!(ctx, 0, "alcCreateContext failed");
            assert_eq!(alcMakeContextCurrent(ctx), ALC_TRUE);
            // Clear any prior error state.
            let _ = alGetError();
            let _ = alcGetError(dev);
            Self {
                _audio_guard: audio_guard,
                dev,
                ctx,
            }
        }
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        unsafe {
            alcMakeContextCurrent(0);
            alcDestroyContext(self.ctx);
            alcCloseDevice(self.dev);
        }
    }
}

fn read_cstr_lossy(p: *const u8) -> String {
    assert!(!p.is_null());
    unsafe { crate::strings::from_ansi(p) }
}

#[test]
fn openal_device_context_lifecycle_and_queries() {
    let env = TestEnv::new(true);
    unsafe {
        assert_eq!(alcGetCurrentContext(), env.ctx);
        assert_eq!(alcGetContextsDevice(env.ctx), env.dev);

        // Real device name from crate::audio::backend (Null backend in test).
        let def_name = read_cstr_lossy(alcGetString(0, ALC_DEFAULT_DEVICE_SPECIFIER));
        assert!(!def_name.is_empty());
        let dev_name = read_cstr_lossy(alcGetString(env.dev, ALC_DEVICE_SPECIFIER));
        assert_eq!(dev_name, def_name);

        // Double-NUL-terminated device list when device == 0.
        let list_ptr = alcGetString(0, ALC_DEVICE_SPECIFIER);
        assert!(!list_ptr.is_null());
        let first = read_cstr_lossy(list_ptr);
        assert_eq!(first, def_name);
        let mut off = 0usize;
        while *list_ptr.add(off) != 0 {
            let s = read_cstr_lossy(list_ptr.add(off));
            off += s.len() + 1;
        }
        assert_eq!(*list_ptr.add(off), 0);

        // ALC integer queries.
        let mut major = 0;
        let mut minor = 0;
        let mut freq = 0;
        let mut mono = 0;
        let mut stereo = 0;
        let mut sync = 0;
        let mut attr_sz = 0;
        alcGetIntegerv(env.dev, ALC_MAJOR_VERSION, 1, &mut major);
        alcGetIntegerv(env.dev, ALC_MINOR_VERSION, 1, &mut minor);
        alcGetIntegerv(env.dev, ALC_FREQUENCY, 1, &mut freq);
        alcGetIntegerv(env.dev, ALC_MONO_SOURCES, 1, &mut mono);
        alcGetIntegerv(env.dev, ALC_STEREO_SOURCES, 1, &mut stereo);
        alcGetIntegerv(env.dev, ALC_SYNC, 1, &mut sync);
        alcGetIntegerv(env.dev, ALC_ATTRIBUTES_SIZE, 1, &mut attr_sz);
        assert_eq!(major, 1);
        assert_eq!(minor, 1);
        assert_eq!(freq, 48_000);
        assert!(mono >= 64);
        assert!(stereo >= 16);
        assert_eq!(sync, 1);
        assert_eq!(attr_sz, 11);

        let mut all_attrs = [0i32; 11];
        alcGetIntegerv(env.dev, ALC_ALL_ATTRIBUTES, 11, all_attrs.as_mut_ptr());
        assert_eq!(all_attrs[0], ALC_FREQUENCY);
        assert_eq!(all_attrs[1], 48_000);
        assert_eq!(all_attrs[10], 0);

        // Extensions, strings, proc addresses, enum values.
        assert_eq!(
            alcIsExtensionPresent(env.dev, b"ALC_ENUMERATION_EXT\0".as_ptr()),
            ALC_TRUE
        );
        assert_eq!(
            alcIsExtensionPresent(env.dev, b"ALC_ENUMERATE_ALL_EXT\0".as_ptr()),
            ALC_TRUE
        );
        assert_eq!(
            alcIsExtensionPresent(env.dev, b"ALC_NONEXISTENT\0".as_ptr()),
            ALC_FALSE
        );
        assert_eq!(
            alIsExtensionPresent(b"AL_EXT_FLOAT32\0".as_ptr()),
            AL_TRUE
        );
        assert!(!alcGetProcAddress(env.dev, b"alcOpenDevice\0".as_ptr()).is_null());
        assert!(!alGetProcAddress(b"alSource3f\0".as_ptr()).is_null());
        assert!(!alGetProcAddress(b"alSourcePlay\0".as_ptr()).is_null());
        assert!(alGetProcAddress(b"alDoesNotExist\0".as_ptr()).is_null());
        assert_eq!(
            alGetEnumValue(b"AL_FORMAT_STEREO_FLOAT32\0".as_ptr()),
            AL_FORMAT_STEREO_FLOAT32
        );
        assert!(read_cstr_lossy(alGetString(AL_VERSION)).contains("1.1"));
        assert_eq!(alcGetError(env.dev), ALC_NO_ERROR);
        assert_eq!(alGetError(), AL_NO_ERROR);
    }
}

#[test]
fn openal_buffer_upload_formats_and_queries() {
    let env = TestEnv::new(true);
    unsafe {
        let mut bufs = [0u32; 4];
        alGenBuffers(4, bufs.as_mut_ptr());
        assert_eq!(alGetError(), AL_NO_ERROR);
        for &b in &bufs {
            assert_ne!(b, 0);
            assert_eq!(alIsBuffer(b), AL_TRUE);
        }

        // 1. MONO8
        let mono8: [u8; 4] = [0, 128, 192, 255];
        alBufferData(bufs[0], AL_FORMAT_MONO8, mono8.as_ptr(), 4, 22_050);

        // 2. MONO16
        let mono16_vals: [i16; 3] = [-32768, 0, 16384];
        let mut mono16_bytes = Vec::new();
        for s in mono16_vals {
            mono16_bytes.extend_from_slice(&s.to_le_bytes());
        }
        alBufferData(
            bufs[1],
            AL_FORMAT_MONO16,
            mono16_bytes.as_ptr(),
            mono16_bytes.len() as i32,
            44_100,
        );

        // 3. STEREO16
        let stereo16_vals: [i16; 4] = [16384, -16384, 8192, -8192];
        let mut stereo16_bytes = Vec::new();
        for s in stereo16_vals {
            stereo16_bytes.extend_from_slice(&s.to_le_bytes());
        }
        alBufferData(
            bufs[2],
            AL_FORMAT_STEREO16,
            stereo16_bytes.as_ptr(),
            stereo16_bytes.len() as i32,
            48_000,
        );

        // 4. STEREO_FLOAT32
        let stereo_f32: [f32; 4] = [0.25, -0.75, 0.5, -0.5];
        let mut stereo_f32_bytes = Vec::new();
        for s in stereo_f32 {
            stereo_f32_bytes.extend_from_slice(&s.to_le_bytes());
        }
        alBufferData(
            bufs[3],
            AL_FORMAT_STEREO_FLOAT32,
            stereo_f32_bytes.as_ptr(),
            stereo_f32_bytes.len() as i32,
            48_000,
        );
        assert_eq!(alGetError(), AL_NO_ERROR);

        // Query properties via i, iv, f, fv.
        let (mut f_i, mut bits, mut ch, mut sz) = (0, 0, 0, 0);
        alGetBufferi(bufs[1], AL_FREQUENCY, &mut f_i);
        alGetBufferiv(bufs[1], AL_BITS, &mut bits);
        alGetBufferi(bufs[1], AL_CHANNELS, &mut ch);
        alGetBufferi(bufs[1], AL_SIZE, &mut sz);
        assert_eq!((f_i, bits, ch, sz), (44_100, 16, 1, 6));

        let mut f_f = 0.0f32;
        let mut sz_f = 0.0f32;
        alGetBufferf(bufs[3], AL_FREQUENCY, &mut f_f);
        alGetBufferfv(bufs[3], AL_SIZE, &mut sz_f);
        assert_eq!(f_f, 48_000.0);
        assert_eq!(sz_f, 16.0);

        // Verify decoded float32 samples directly in device buffer store.
        {
            let st = lock_state();
            let dev = st.devices.get(&env.dev).unwrap();
            let b0 = dev.buffers.get(&bufs[0]).unwrap();
            assert!((b0.samples[0] - (-1.0)).abs() < 1e-5);
            assert!((b0.samples[1] - 0.0).abs() < 1e-5);
            assert!((b0.samples[2] - 0.5).abs() < 1e-5);

            let b1 = dev.buffers.get(&bufs[1]).unwrap();
            assert!((b1.samples[0] - (-1.0)).abs() < 1e-5);
            assert!((b1.samples[1] - 0.0).abs() < 1e-5);
            assert!((b1.samples[2] - 0.5).abs() < 1e-5);

            let b2 = dev.buffers.get(&bufs[2]).unwrap();
            assert_eq!(b2.channels, 2);
            assert!((b2.samples[0] - 0.5).abs() < 1e-5);
            assert!((b2.samples[1] - (-0.5)).abs() < 1e-5);

            let b3 = dev.buffers.get(&bufs[3]).unwrap();
            assert_eq!(b3.samples, stereo_f32);
        }

        alDeleteBuffers(4, bufs.as_ptr());
        assert_eq!(alGetError(), AL_NO_ERROR);
        assert_eq!(alIsBuffer(bufs[0]), AL_FALSE);
    }
}

#[test]
fn openal_mixer_play_pause_stop_rewind_looping_and_pitch() {
    let env = TestEnv::new(true);
    unsafe {
        let mut buf = 0u32;
        let mut src = 0u32;
        alGenBuffers(1, &mut buf);
        alGenSources(1, &mut src);
        assert_eq!(alIsSource(src), AL_TRUE);

        // 8 stereo float32 frames: ramp 0.1, 0.2, ..., 0.8 on L, -0.1, ..., -0.8 on R.
        let mut pcm = Vec::new();
        for i in 1..=8 {
            let v = i as f32 * 0.1;
            pcm.extend_from_slice(&v.to_le_bytes());
            pcm.extend_from_slice(&(-v).to_le_bytes());
        }
        alBufferData(
            buf,
            AL_FORMAT_STEREO_FLOAT32,
            pcm.as_ptr(),
            pcm.len() as i32,
            48_000,
        );
        alSourcei(src, AL_BUFFER, buf as i32);

        let mut state_val = 0;
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_INITIAL);

        // Play first 4 frames.
        alSourcePlay(src);
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_PLAYING);

        let mut out = [0.0f32; 8]; // 4 stereo frames
        render_context_frames(env.ctx, &mut out);
        assert!((out[0] - 0.1).abs() < 1e-5);
        assert!((out[1] - (-0.1)).abs() < 1e-5);
        assert!((out[6] - 0.4).abs() < 1e-5);
        assert!((out[7] - (-0.4)).abs() < 1e-5);

        // Pause: must output silence and preserve sample offset = 4.
        alSourcePause(src);
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_PAUSED);
        let mut offset = 0;
        let mut byte_off = 0;
        let mut sec_off = 0.0f32;
        alGetSourcei(src, AL_SAMPLE_OFFSET, &mut offset);
        alGetSourcei(src, AL_BYTE_OFFSET, &mut byte_off);
        alGetSourcef(src, AL_SEC_OFFSET, &mut sec_off);
        assert_eq!(offset, 4);
        assert_eq!(byte_off, 4 * 8); // 4 stereo float32 frames = 32 bytes
        assert!((sec_off - (4.0 / 48_000.0)).abs() < 1e-6);

        render_context_frames(env.ctx, &mut out);
        assert!(out.iter().all(|&x| x == 0.0));

        // Suspend / process context gates mixer output cleanly.
        alSourcePlay(src);
        alcSuspendContext(env.ctx);
        render_context_frames(env.ctx, &mut out);
        assert!(out.iter().all(|&x| x == 0.0));
        alcProcessContext(env.ctx);

        // Finish remaining 4 frames + 2 trailing silent frames.
        let mut out6 = [0.0f32; 12]; // 6 stereo frames
        render_context_frames(env.ctx, &mut out6);
        assert!((out6[0] - 0.5).abs() < 1e-5);
        assert!((out6[6] - 0.8).abs() < 1e-5);
        assert_eq!(out6[8], 0.0);
        assert_eq!(out6[10], 0.0);
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_STOPPED);

        // Rewind -> AL_INITIAL, then enable looping and render 12 frames (wraps past 8).
        alSourceRewind(src);
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_INITIAL);
        alSourcei(src, AL_LOOPING, 1);
        alSourcePlayv(1, &src);

        let mut loop_out = [0.0f32; 24]; // 12 stereo frames
        render_context_frames(env.ctx, &mut loop_out);
        // Frame 8 wraps back to sample 0 (0.1, -0.1).
        assert!((loop_out[16] - 0.1).abs() < 1e-5);
        assert!((loop_out[17] - (-0.1)).abs() < 1e-5);
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_PLAYING);

        // Pitch = 2.0: samples advance 2 frames per output frame.
        alSourceStopv(1, &src);
        alSourcei(src, AL_LOOPING, 0);
        al_source_f(src, AL_PITCH, 2.0);
        alSourcePlay(src);
        let mut pitch_out = [0.0f32; 10]; // 5 stereo frames (buffer finishes after 4)
        render_context_frames(env.ctx, &mut pitch_out);
        assert!((pitch_out[0] - 0.1).abs() < 1e-5); // frame 0
        assert!((pitch_out[2] - 0.3).abs() < 1e-5); // frame 2
        assert!((pitch_out[4] - 0.5).abs() < 1e-5); // frame 4
        assert!((pitch_out[6] - 0.7).abs() < 1e-5); // frame 6
        assert_eq!(pitch_out[8], 0.0); // finished
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        assert_eq!(state_val, AL_STOPPED);
        assert_eq!(alGetError(), AL_NO_ERROR);
    }
}

#[test]
fn openal_streaming_queue_and_unqueue() {
    let env = TestEnv::new(true);
    unsafe {
        let mut bufs = [0u32; 3];
        let mut src = 0u32;
        alGenBuffers(3, bufs.as_mut_ptr());
        alGenSources(1, &mut src);

        // Three 4-frame stereo float32 buffers with distinct DC levels 0.25, 0.50, 0.75.
        for (i, &val) in [0.25f32, 0.50, 0.75].iter().enumerate() {
            let mut bytes = Vec::new();
            for _ in 0..4 {
                bytes.extend_from_slice(&val.to_le_bytes());
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            alBufferData(
                bufs[i],
                AL_FORMAT_STEREO_FLOAT32,
                bytes.as_ptr(),
                bytes.len() as i32,
                48_000,
            );
        }

        alSourceQueueBuffers(src, 3, bufs.as_ptr());
        assert_eq!(alGetError(), AL_NO_ERROR);

        let (mut stype, mut queued, mut processed) = (0, 0, 0);
        alGetSourcei(src, AL_SOURCE_TYPE, &mut stype);
        alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
        alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
        assert_eq!(stype, AL_STREAMING);
        assert_eq!(queued, 3);
        assert_eq!(processed, 0);

        // Play and consume exactly buffer 0 (4 frames).
        alSourcePlay(src);
        let mut out4 = [0.0f32; 8];
        render_context_frames(env.ctx, &mut out4);
        assert!(out4.iter().all(|&x| (x - 0.25).abs() < 1e-5));

        alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
        assert_eq!(processed, 1);

        // Unqueuing 2 when only 1 is processed must fail with AL_INVALID_VALUE atomically.
        let mut unq2 = [0u32; 2];
        alSourceUnqueueBuffers(src, 2, unq2.as_mut_ptr());
        assert_eq!(alGetError(), AL_INVALID_VALUE);
        alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
        assert_eq!(queued, 3);

        // Unqueue 1 processed buffer -> must return bufs[0].
        let mut unq1 = 0u32;
        alSourceUnqueueBuffers(src, 1, &mut unq1);
        assert_eq!(alGetError(), AL_NO_ERROR);
        assert_eq!(unq1, bufs[0]);
        alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
        alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
        assert_eq!((queued, processed), (2, 0));

        // Render 10 frames: 4 frames of 0.50, 4 frames of 0.75, 2 frames of silence.
        let mut out10 = [0.0f32; 20];
        render_context_frames(env.ctx, &mut out10);
        for i in 0..8 {
            assert!((out10[i] - 0.50).abs() < 1e-5);
        }
        for i in 8..16 {
            assert!((out10[i] - 0.75).abs() < 1e-5);
        }
        for i in 16..20 {
            assert_eq!(out10[i], 0.0);
        }

        let mut state_val = 0;
        alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
        alGetSourcei(src, AL_BUFFERS_PROCESSED, &mut processed);
        assert_eq!(state_val, AL_STOPPED);
        assert_eq!(processed, 2);

        let mut rest = [0u32; 2];
        alSourceUnqueueBuffers(src, 2, rest.as_mut_ptr());
        assert_eq!(alGetError(), AL_NO_ERROR);
        assert_eq!(rest, [bufs[1], bufs[2]]);
        alGetSourcei(src, AL_BUFFERS_QUEUED, &mut queued);
        assert_eq!(queued, 0);
    }
}

#[test]
fn openal_3d_panning_listener_and_distance_models() {
    let env = TestEnv::new(true);
    unsafe {
        let mut buf = 0u32;
        let mut src = 0u32;
        alGenBuffers(1, &mut buf);
        alGenSources(1, &mut src);

        // Constant 1.0 mono float32 buffer (looping).
        let mut bytes = Vec::new();
        for _ in 0..16 {
            bytes.extend_from_slice(&1.0f32.to_le_bytes());
        }
        alBufferData(
            buf,
            AL_FORMAT_MONO_FLOAT32,
            bytes.as_ptr(),
            bytes.len() as i32,
            48_000,
        );
        alSourcei(src, AL_BUFFER, buf as i32);
        alSourcei(src, AL_LOOPING, 1);

        // Disable distance attenuation first to verify pure 3D stereo panning.
        alDistanceModel(AL_NONE);
        assert_eq!(alGetInteger(AL_DISTANCE_MODEL), AL_NONE);

        // Listener at origin looking -Z with up +Y => right = +X.
        al_listener_3f(AL_POSITION, 0.0, 0.0, 0.0);
        let orient = [0.0f32, 0.0, -1.0, 0.0, 1.0, 0.0];
        alListenerfv(AL_ORIENTATION, orient.as_ptr());

        let mut got_orient = [0.0f32; 6];
        alGetListenerfv(AL_ORIENTATION, got_orient.as_mut_ptr());
        assert_eq!(got_orient, orient);

        // Source hard right (+10, 0, 0): L ~ 0, R ~ 1.
        al_source_3f(src, AL_POSITION, 10.0, 0.0, 0.0);
        alSourcePlay(src);
        let mut frame = [0.0f32; 2];
        render_context_frames(env.ctx, &mut frame);
        assert!(frame[0].abs() < 1e-4, "expected L~0, got {}", frame[0]);
        assert!((frame[1] - 1.0).abs() < 1e-4, "expected R~1, got {}", frame[1]);

        // Source hard left (-10, 0, 0): L ~ 1, R ~ 0.
        al_source_3f(src, AL_POSITION, -10.0, 0.0, 0.0);
        render_context_frames(env.ctx, &mut frame);
        assert!((frame[0] - 1.0).abs() < 1e-4, "expected L~1, got {}", frame[0]);
        assert!(frame[1].abs() < 1e-4, "expected R~0, got {}", frame[1]);

        // Rotate listener to face +X, up +Y => right = cross((1,0,0), (0,1,0)) = (0,0,+1).
        // Source at (0, 0, +10) should now pan hard right!
        let orient_x = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0];
        alListenerfv(AL_ORIENTATION, orient_x.as_ptr());
        al_source_3f(src, AL_POSITION, 0.0, 0.0, 10.0);
        render_context_frames(env.ctx, &mut frame);
        assert!(frame[0].abs() < 1e-4);
        assert!((frame[1] - 1.0).abs() < 1e-4);

        // Distance attenuation: center source ahead along +X at distance = 4.0.
        // Reset orientation to default (-Z forward, +Y up), place source at (0, 0, -4).
        alListenerfv(AL_ORIENTATION, orient.as_ptr());
        al_source_3f(src, AL_POSITION, 0.0, 0.0, -4.0);
        al_source_f(src, AL_REFERENCE_DISTANCE, 1.0);
        al_source_f(src, AL_ROLLOFF_FACTOR, 1.0);
        al_source_f(src, AL_MAX_DISTANCE, 10.0);

        // Inverse distance clamped at d=4, ref=1, rolloff=1 => gain = 1 / (1 + 1*(4-1)) = 0.25.
        // Centered pan multiplies by cos(pi/4) = FRAC_1_SQRT_2.
        alDistanceModel(AL_INVERSE_DISTANCE_CLAMPED);
        render_context_frames(env.ctx, &mut frame);
        let exp_inv = 0.25 * std::f32::consts::FRAC_1_SQRT_2;
        assert!((frame[0] - exp_inv).abs() < 1e-4);
        assert!((frame[1] - exp_inv).abs() < 1e-4);

        // Linear distance clamped at d=4, ref=1, max=10, rolloff=1 => 1 - (3/9) = 2/3.
        alDistanceModel(AL_LINEAR_DISTANCE_CLAMPED);
        render_context_frames(env.ctx, &mut frame);
        let exp_lin = (2.0 / 3.0) * std::f32::consts::FRAC_1_SQRT_2;
        assert!((frame[0] - exp_lin).abs() < 1e-4);

        // Exponent distance clamped at d=4, ref=1, rolloff=2 => (4/1)^(-2) = 1/16 = 0.0625.
        al_source_f(src, AL_ROLLOFF_FACTOR, 2.0);
        alDistanceModel(AL_EXPONENT_DISTANCE_CLAMPED);
        render_context_frames(env.ctx, &mut frame);
        let exp_pow = 0.0625 * std::f32::consts::FRAC_1_SQRT_2;
        assert!((frame[0] - exp_pow).abs() < 1e-4);

        // Listener gain halves output.
        al_listener_f(AL_GAIN, 0.5);
        render_context_frames(env.ctx, &mut frame);
        assert!((frame[0] - exp_pow * 0.5).abs() < 1e-4);

        // AL_MIN_GAIN / AL_MAX_GAIN clamping.
        al_source_f(src, AL_MIN_GAIN, 0.5);
        render_context_frames(env.ctx, &mut frame);
        let exp_min = 0.5 * 0.5 * std::f32::consts::FRAC_1_SQRT_2;
        assert!((frame[0] - exp_min).abs() < 1e-4);

        // AL_SOURCE_RELATIVE ignores listener offset/orientation.
        al_listener_f(AL_GAIN, 1.0);
        al_listener_3f(AL_POSITION, 100.0, 0.0, 0.0);
        al_source_f(src, AL_MIN_GAIN, 0.0);
        alDistanceModel(AL_NONE);
        alSourcei(src, AL_SOURCE_RELATIVE, 1);
        al_source_3f(src, AL_POSITION, 10.0, 0.0, 0.0);
        render_context_frames(env.ctx, &mut frame);
        assert!(frame[0].abs() < 1e-4);
        assert!((frame[1] - 1.0).abs() < 1e-4);

        // Global Doppler / speed of sound / enable-disable queries.
        alDopplerFactor(2.0);
        alSpeedOfSound(340.0);
        assert_eq!(alGetFloat(AL_DOPPLER_FACTOR), 2.0);
        assert_eq!(alGetFloat(AL_SPEED_OF_SOUND), 340.0);
        assert_eq!(alGetBoolean(AL_DOPPLER_FACTOR), AL_TRUE);
        alEnable(AL_SOURCE_DISTANCE_MODEL);
        assert_eq!(alIsEnabled(AL_SOURCE_DISTANCE_MODEL), AL_TRUE);
        alDisable(AL_SOURCE_DISTANCE_MODEL);
        assert_eq!(alIsEnabled(AL_SOURCE_DISTANCE_MODEL), AL_FALSE);

        assert_eq!(alGetError(), AL_NO_ERROR);
    }
}

#[test]
fn openal_error_codes_and_validation() {
    let env = TestEnv::new(true);
    unsafe {
        // AL_INVALID_NAME on bad source/buffer ID.
        alSourcePlay(999_999);
        assert_eq!(alGetError(), AL_INVALID_NAME);
        assert_eq!(alGetError(), AL_NO_ERROR); // cleared on read

        // Sticky first error: only first error is kept until alGetError().
        alSourcePlay(999_999); // AL_INVALID_NAME
        alDistanceModel(0x1234_5678); // AL_INVALID_VALUE (ignored while error pending)
        assert_eq!(alGetError(), AL_INVALID_NAME);
        assert_eq!(alGetError(), AL_NO_ERROR);

        // AL_INVALID_ENUM on unknown format.
        let mut buf = 0u32;
        let mut src = 0u32;
        alGenBuffers(1, &mut buf);
        alGenSources(1, &mut src);
        let sample = [0u8; 4];
        alBufferData(buf, 0x9999, sample.as_ptr(), 4, 48_000);
        assert_eq!(alGetError(), AL_INVALID_ENUM);

        // AL_INVALID_VALUE on misaligned size for STEREO16 (must be multiple of 4).
        alBufferData(buf, AL_FORMAT_STEREO16, sample.as_ptr(), 3, 48_000);
        assert_eq!(alGetError(), AL_INVALID_VALUE);

        // AL_INVALID_VALUE on invalid pitch / negative gain / null output ptr.
        al_source_f(src, AL_PITCH, 0.0);
        assert_eq!(alGetError(), AL_INVALID_VALUE);
        al_source_f(src, AL_GAIN, -0.5);
        assert_eq!(alGetError(), AL_INVALID_VALUE);
        alGetSourcei(src, AL_SOURCE_STATE, std::ptr::null_mut());
        assert_eq!(alGetError(), AL_INVALID_VALUE);

        // AL_INVALID_OPERATION: deleting an attached buffer, or queueing on static source.
        alBufferData(buf, AL_FORMAT_MONO16, sample.as_ptr(), 4, 48_000);
        alSourcei(src, AL_BUFFER, buf as i32);
        assert_eq!(alGetError(), AL_NO_ERROR);

        alDeleteBuffers(1, &buf);
        assert_eq!(alGetError(), AL_INVALID_OPERATION);
        assert_eq!(alIsBuffer(buf), AL_TRUE);

        alSourceQueueBuffers(src, 1, &buf);
        assert_eq!(alGetError(), AL_INVALID_OPERATION);

        // ALC errors.
        assert_eq!(alcMakeContextCurrent(0xDEAD_BEEF), ALC_FALSE);
        assert_eq!(alcGetError(0), ALC_INVALID_CONTEXT);
        assert_eq!(alcGetError(0xDEAD_BEEF), ALC_INVALID_DEVICE);
        let mut v = 0;
        alcGetIntegerv(env.dev, 0x7777, 1, &mut v);
        assert_eq!(alcGetError(env.dev), ALC_INVALID_ENUM);
    }
}

#[test]
fn openal_async_host_mixer_thread_pumps_and_tears_down() {
    let _env = TestEnv::new(false);
    unsafe {
        let mut buf = 0u32;
        let mut src = 0u32;
        alGenBuffers(1, &mut buf);
        alGenSources(1, &mut src);

        // ~10 ms mono16 buffer at 48 kHz (480 samples).
        let pcm = vec![0u8; 480 * 2];
        alBufferData(buf, AL_FORMAT_MONO16, pcm.as_ptr(), pcm.len() as i32, 48_000);
        alSourcei(src, AL_BUFFER, buf as i32);
        alSourcePlay(src);

        // Host mixer thread + Null OutputStream must consume the buffer and stop it.
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut state_val = AL_PLAYING;
        while Instant::now() < deadline {
            alGetSourcei(src, AL_SOURCE_STATE, &mut state_val);
            if state_val == AL_STOPPED {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            state_val, AL_STOPPED,
            "async host mixer thread did not finish short buffer in time"
        );
        assert_eq!(alGetError(), AL_NO_ERROR);
    }
    // Drop of TestEnv exercises alcDestroyContext + alcCloseDevice teardown.
}

#[cfg(target_arch = "aarch64")]
#[test]
fn openal_win64_bridge_aarch64_mixed_float_slots() {
    let _env = TestEnv::new(true);
    unsafe {
        let mut src = 0u32;
        alGenSources(1, &mut src);

        // Win64 ABI for alSource3f(src, AL_POSITION, 1.5, -2.25, 3.75):
        //   slot 1 (x0) = src
        //   slot 2 (x1) = AL_POSITION
        //   slot 3 (v2/xmm2) = 1.5f32
        //   slot 4 (v3/xmm3) = -2.25f32
        //   slot 5 (x4/stack slot 5) = 3.75f32.to_bits()
        let x2 = f64::from_bits(1.5f32.to_bits() as u64);
        let x3 = f64::from_bits((-2.25f32).to_bits() as u64);
        al_source_3f::bridge(
            src as u64,
            AL_POSITION as u32 as u64,
            0xDEAD,
            0xBEEF,
            3.75f32.to_bits() as u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            0.0,
            x2,
            x3,
        );

        let (mut px, mut py, mut pz) = (0.0f32, 0.0, 0.0);
        alGetSource3f(src, AL_POSITION, &mut px, &mut py, &mut pz);
        assert_eq!((px, py, pz), (1.5, -2.25, 3.75));
        assert_eq!(alGetError(), AL_NO_ERROR);
    }
}
