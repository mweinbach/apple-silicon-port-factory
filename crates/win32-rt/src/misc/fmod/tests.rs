use super::core::lookup_system;
use super::dsp::{
    FSB5_CHUNK_CHANNELS, FSB5_CHUNK_FREQUENCY, FSB5_CHUNK_LOOP, FSB5_MODE_FADPCM,
    FSB5_MODE_IMAADPCM, FSB5_MODE_PCM8, FSB5_MODE_PCM16, FSB5_MODE_PCM24, FSB5_MODE_PCM32,
    FSB5_MODE_PCMFLOAT, FSB5_SAMPLE_RATES, decode_fsb5_fadpcm, decode_fsb5_ima_adpcm,
};
use super::mixer::MIX_BLOCK_FRAMES;
use super::*;
use crate::audio::audio_test_lock;
use crate::audio::backend::{self, Backend, DeviceInfo};
use std::sync::atomic::Ordering;
use std::time::Duration;

// Architecture-uniform wrappers for exports with float parameters.
unsafe fn call_set_channel_volume(ch: usize, vol: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Channel_SetVolume(ch, 0.0, vol)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Channel_SetVolume(ch, vol)
    }
}

unsafe fn call_set_channel_pitch(ch: usize, pitch: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Channel_SetPitch(ch, 0.0, pitch)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Channel_SetPitch(ch, pitch)
    }
}

unsafe fn call_set_channel_pan(ch: usize, pan: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Channel_SetPan(ch, 0.0, pan)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Channel_SetPan(ch, pan)
    }
}

unsafe fn call_set_channel_3d_minmax(ch: usize, min: f32, max: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Channel_Set3DMinMaxDistance(ch, 0.0, min, max)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Channel_Set3DMinMaxDistance(ch, min, max)
    }
}

unsafe fn call_set_group_volume(g: usize, vol: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_ChannelGroup_SetVolume(g, 0.0, vol)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_ChannelGroup_SetVolume(g, vol)
    }
}

unsafe fn call_set_3d_settings(sys: usize, doppler: f32, dist: f32, rolloff: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_System_Set3DSettings(sys, 0.0, doppler, dist, rolloff)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_System_Set3DSettings(sys, doppler, dist, rolloff)
    }
}

fn build_wav(audio_format: u16, channels: u16, sample_rate: u32, bits: u16, pcm: &[u8]) -> Vec<u8> {
    let block_align = channels * (bits / 8);
    let byte_rate = sample_rate * block_align as u32;
    let data_len = pcm.len() as u32;
    let riff_size = 4 + (8 + 16) + (8 + data_len + (data_len & 1));
    let mut out = Vec::with_capacity((riff_size + 8) as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_size.to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&audio_format.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&bits.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    out.extend_from_slice(pcm);
    if (pcm.len() & 1) != 0 {
        out.push(0);
    }
    out
}

unsafe fn create_nrt_system() -> usize {
    let mut sys = 0usize;
    assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);
    assert_ne!(sys, 0);
    assert_eq!(FMOD_System_SetOutput(sys, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
    assert_eq!(FMOD_System_Init(sys, 32, 0, 0), FMOD_OK);
    sys
}

#[test]
fn fmod_creates_handle_and_version() {
    unsafe {
        let mut sys = 0usize;
        assert_eq!(FMOD_System_Create(&mut sys, 0), FMOD_OK);
        assert_ne!(sys, 0);
        let mut ver = 0u32;
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, 0x0002_0200);
        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_enumerates_real_backend_drivers() {
    let _g = audio_test_lock();
    backend::force_output_devices(Some(vec![
        DeviceInfo {
            id: 42,
            uid: "ca-uid-studio-display".into(),
            name: "Studio Display Speakers".into(),
            rate: 48_000,
            channels: 6,
            io_frames: 512,
            latency_frames: 512,
            headphones: false,
            is_default: true,
        },
        DeviceInfo {
            id: 99,
            uid: "ca-uid-airpods".into(),
            name: "AirPods Pro".into(),
            rate: 44_100,
            channels: 2,
            io_frames: 256,
            latency_frames: 512,
            headphones: true,
            is_default: false,
        },
    ]));

    unsafe {
        let mut sys = 0usize;
        assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);

        let mut count = 0i32;
        assert_eq!(FMOD_System_GetNumDrivers(sys, &mut count), FMOD_OK);
        assert_eq!(count, 2);

        let mut name_buf = [0u8; 64];
        let mut guid = [0u8; 16];
        let mut rate = 0i32;
        let mut mode = 0u32;
        let mut ch = 0i32;

        assert_eq!(
            FMOD_System_GetDriverInfo(
                sys,
                0,
                name_buf.as_mut_ptr(),
                name_buf.len() as i32,
                guid.as_mut_ptr() as usize,
                &mut rate,
                &mut mode,
                &mut ch,
            ),
            FMOD_OK
        );
        assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "Studio Display Speakers");
        assert_eq!(rate, 48_000);
        assert_eq!(mode, FMOD_SPEAKERMODE_5POINT1);
        assert_eq!(ch, 6);
        assert_ne!(guid, [0u8; 16]);

        assert_eq!(
            FMOD_System_GetDriverInfo(
                sys,
                1,
                name_buf.as_mut_ptr(),
                name_buf.len() as i32,
                0,
                &mut rate,
                &mut mode,
                &mut ch,
            ),
            FMOD_OK
        );
        assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "AirPods Pro");
        assert_eq!(rate, 44_100);
        assert_eq!(mode, FMOD_SPEAKERMODE_STEREO);
        assert_eq!(ch, 2);

        // Out-of-range driver index fails cleanly.
        assert_eq!(
            FMOD_System_GetDriverInfo(sys, 2, name_buf.as_mut_ptr(), 64, 0, &mut rate, &mut mode, &mut ch),
            FMOD_ERR_INVALID_PARAM
        );

        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_wav_formats_memory_and_file_mix_exact_waveforms() {
    unsafe {
        let sys = create_nrt_system();

        // 1) 16-bit stereo WAV from memory
        let mut pcm16 = Vec::new();
        for _ in 0..16 {
            pcm16.extend_from_slice(&16384i16.to_le_bytes()); // L = +0.5
            pcm16.extend_from_slice(&(-8192i16).to_le_bytes()); // R = -0.25
        }
        let wav16 = build_wav(1, 2, 48_000, 16, &pcm16);
        let ex16 = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: wav16.len() as u32,
            ..Default::default()
        };
        let mut snd16 = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                wav16.as_ptr(),
                FMOD_OPENMEMORY,
                &ex16 as *const _ as usize,
                &mut snd16,
            ),
            FMOD_OK
        );

        let (mut stype, mut fmt, mut chs, mut bits) = (0u32, 0u32, 0i32, 0i32);
        assert_eq!(FMOD_Sound_GetFormat(snd16, &mut stype, &mut fmt, &mut chs, &mut bits), FMOD_OK);
        assert_eq!((stype, fmt, chs, bits), (FMOD_SOUND_TYPE_WAV, FMOD_SOUND_FORMAT_PCM16, 2, 16));

        let (mut len_pcm, mut len_bytes, mut len_ms) = (0u32, 0u32, 0u32);
        assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_pcm, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_bytes, FMOD_TIMEUNIT_PCMBYTES), FMOD_OK);
        assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_ms, FMOD_TIMEUNIT_MS), FMOD_OK);
        assert_eq!(len_pcm, 16);
        assert_eq!(len_bytes, 64);
        assert_eq!(len_ms, 0);

        let (mut def_freq, mut def_prio) = (0.0f32, 0i32);
        assert_eq!(FMOD_Sound_GetDefaults(snd16, &mut def_freq, &mut def_prio), FMOD_OK);
        assert_eq!(def_freq, 48_000.0);
        assert_eq!(def_prio, 128);

        let mut ch16 = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd16, 0, 0, &mut ch16), FMOD_OK);

        let mut mix = [0.0f32; 16 * 2];
        assert_eq!(mix_system_frames(sys, &mut mix), Ok(16));
        for f in 0..16 {
            assert!((mix[f * 2] - 0.5).abs() < 1e-4, "L[{f}]={}", mix[f * 2]);
            assert!((mix[f * 2 + 1] - (-0.25)).abs() < 1e-4, "R[{f}]={}", mix[f * 2 + 1]);
        }
        // One-shot sound finished after 16 frames.
        let mut playing = 1i32;
        assert_eq!(FMOD_Channel_IsPlaying(ch16, &mut playing), FMOD_OK);
        assert_eq!(playing, 0);

        // 2) 8-bit, 24-bit, 32-bit PCM, and 32-bit float WAVs (including loading from file!)
        let wav8 = build_wav(1, 1, 48_000, 8, &[192u8; 4]); // (192-128)/128 = +0.5
        let mut pcm24 = Vec::new();
        for _ in 0..4 {
            pcm24.extend_from_slice(&[0x00, 0x00, 0xC0]); // -0.5 in signed 24-bit LE
        }
        let wav24 = build_wav(1, 1, 48_000, 24, &pcm24);
        let mut pcm32 = Vec::new();
        for _ in 0..4 {
            pcm32.extend_from_slice(&(i32::MAX / 2 + 1).to_le_bytes()); // +0.5
        }
        let wav32 = build_wav(1, 1, 48_000, 32, &pcm32);
        let mut pcmf = Vec::new();
        for _ in 0..4 {
            pcmf.extend_from_slice(&0.75f32.to_le_bytes());
        }
        let wavf = build_wav(3, 1, 48_000, 32, &pcmf);

        for (wav_bytes, expected_fmt, expected_amp) in [
            (&wav8, FMOD_SOUND_FORMAT_PCM8, 0.5f32),
            (&wav24, FMOD_SOUND_FORMAT_PCM24, -0.5f32),
            (&wav32, FMOD_SOUND_FORMAT_PCM32, 0.5f32),
            (&wavf, FMOD_SOUND_FORMAT_PCMFLOAT, 0.75f32),
        ] {
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: wav_bytes.len() as u32,
                ..Default::default()
            };
            let mut snd = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    wav_bytes.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex as *const _ as usize,
                    &mut snd,
                ),
                FMOD_OK
            );
            let mut f_out = 0u32;
            assert_eq!(FMOD_Sound_GetFormat(snd, std::ptr::null_mut(), &mut f_out, std::ptr::null_mut(), std::ptr::null_mut()), FMOD_OK);
            assert_eq!(f_out, expected_fmt);

            let mut ch = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 0, &mut ch), FMOD_OK);
            let mut out = [0.0f32; 4 * 2];
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!((out[0] - expected_amp).abs() < 1e-3, "fmt={expected_fmt} got={}", out[0]);
            assert!((out[1] - expected_amp).abs() < 1e-3);
            assert_eq!(FMOD_Sound_Release(snd), FMOD_OK);
        }

        // 3) File-based WAV via Windows path mount (`crate::path`)
        let tmp_dir = std::env::temp_dir().join("win32_rt_fmod_wav_test");
        let _ = std::fs::create_dir_all(&tmp_dir);
        let file_path = tmp_dir.join("tone.wav");
        std::fs::write(&file_path, &wavf).unwrap();
        crate::path::add_mount("F:\\FmodTest", &tmp_dir);

        let win_path = b"F:\\FmodTest\\tone.wav\0";
        let mut file_snd = 0usize;
        assert_eq!(
            FMOD_System_CreateStream(sys, win_path.as_ptr(), FMOD_DEFAULT, 0, &mut file_snd),
            FMOD_OK
        );
        let mut file_ch = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, file_snd, 0, 0, &mut file_ch), FMOD_OK);
        let mut out = [0.0f32; 4 * 2];
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!((out[0] - 0.75).abs() < 1e-4);

        let _ = std::fs::remove_file(&file_path);
        let _ = std::fs::remove_dir(&tmp_dir);
        assert_eq!(FMOD_Sound_Release(file_snd), FMOD_OK);
        assert_eq!(FMOD_Sound_Release(snd16), FMOD_OK);
        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_raw_pcm_channel_group_hierarchy_volume_mute_and_lock() {
    unsafe {
        let sys = create_nrt_system();

        // Create raw float mono sound with constant +0.8 amplitude.
        let samples = [0.8f32; 64];
        let raw_bytes = std::slice::from_raw_parts(
            samples.as_ptr() as *const u8,
            std::mem::size_of_val(&samples),
        );
        let ex = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: raw_bytes.len() as u32,
            fileoffset: 0,
            numchannels: 1,
            defaultfrequency: 48_000,
            format: FMOD_SOUND_FORMAT_PCMFLOAT,
        };
        let mut snd = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                raw_bytes.as_ptr(),
                FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL,
                &ex as *const _ as usize,
                &mut snd,
            ),
            FMOD_OK
        );

        // Build hierarchy: child_grp -> parent_grp -> master
        let mut master = 0usize;
        assert_eq!(FMOD_System_GetMasterChannelGroup(sys, &mut master), FMOD_OK);
        let mut parent_grp = 0usize;
        let mut child_grp = 0usize;
        assert_eq!(FMOD_System_CreateChannelGroup(sys, b"SFX\0".as_ptr(), &mut parent_grp), FMOD_OK);
        assert_eq!(FMOD_System_CreateChannelGroup(sys, b"Footsteps\0".as_ptr(), &mut child_grp), FMOD_OK);
        assert_eq!(FMOD_ChannelGroup_AddGroup(parent_grp, child_grp, 0, std::ptr::null_mut()), FMOD_OK);

        // Cycle detection must reject reparenting parent under its own child.
        assert_eq!(
            FMOD_ChannelGroup_AddGroup(child_grp, parent_grp, 0, std::ptr::null_mut()),
            FMOD_ERR_INVALID_PARAM
        );

        let mut ch = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd, child_grp, 0, &mut ch), FMOD_OK);

        // Effective gain = 0.8 (sample) * 0.5 (ch) * 0.5 (child) * 0.5 (parent) * 0.5 (master) = 0.05
        assert_eq!(call_set_channel_volume(ch, 0.5), FMOD_OK);
        assert_eq!(call_set_group_volume(child_grp, 0.5), FMOD_OK);
        assert_eq!(call_set_group_volume(parent_grp, 0.5), FMOD_OK);
        assert_eq!(call_set_group_volume(master, 0.5), FMOD_OK);

        let mut out = [0.0f32; 8 * 2];
        assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
        assert!((out[0] - 0.05).abs() < 1e-4, "got {}", out[0]);
        assert!((out[1] - 0.05).abs() < 1e-4);

        // Mute parent group -> silence, while play position still advances.
        assert_eq!(FMOD_ChannelGroup_SetMute(parent_grp, 1), FMOD_OK);
        let mut muted = 0i32;
        assert_eq!(FMOD_ChannelGroup_GetMute(parent_grp, &mut muted), FMOD_OK);
        assert_eq!(muted, 1);

        let mut pos_before = 0u32;
        assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_before, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
        assert!(out.iter().all(|&v| v == 0.0));
        let mut pos_after = 0u32;
        assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_after, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(pos_after, pos_before + 8);

        // Pause parent group -> silence AND play position must NOT advance.
        assert_eq!(FMOD_ChannelGroup_SetMute(parent_grp, 0), FMOD_OK);
        assert_eq!(FMOD_ChannelGroup_SetPaused(parent_grp, 1), FMOD_OK);
        let mut grp_paused = 0i32;
        assert_eq!(FMOD_ChannelGroup_GetPaused(parent_grp, &mut grp_paused), FMOD_OK);
        assert_eq!(grp_paused, 1);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
        assert!(out.iter().all(|&v| v == 0.0));
        let mut pos_paused = 0u32;
        assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_paused, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(pos_paused, pos_after);
        assert_eq!(FMOD_ChannelGroup_SetPaused(parent_grp, 0), FMOD_OK);

        // Reassign channel to parent_grp directly so muting child_grp does not silence it.
        assert_eq!(FMOD_Channel_SetChannelGroup(ch, parent_grp), FMOD_OK);
        assert_eq!(FMOD_ChannelGroup_SetMute(child_grp, 1), FMOD_OK);
        assert_eq!(call_set_channel_volume(ch, 1.0), FMOD_OK);
        assert_eq!(call_set_group_volume(child_grp, 1.0), FMOD_OK);
        assert_eq!(call_set_group_volume(parent_grp, 1.0), FMOD_OK);
        assert_eq!(call_set_group_volume(master, 1.0), FMOD_OK);

        let (mut ch_vol, mut grp_vol) = (0.0f32, 0.0f32);
        assert_eq!(FMOD_Channel_GetVolume(ch, &mut ch_vol), FMOD_OK);
        assert_eq!(FMOD_ChannelGroup_GetVolume(parent_grp, &mut grp_vol), FMOD_OK);
        assert!((ch_vol - 1.0).abs() < 1e-5 && (grp_vol - 1.0).abs() < 1e-5);

        assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
        assert!((out[0] - 0.8).abs() < 1e-4);
        assert_eq!(FMOD_ChannelGroup_SetMute(child_grp, 0), FMOD_OK);
        assert_eq!(FMOD_Channel_SetChannelGroup(ch, child_grp), FMOD_OK);

        // Test Sound_Lock / Sound_Unlock mutation.
        let (mut p1, mut p2): (*mut u8, *mut u8) = (std::ptr::null_mut(), std::ptr::null_mut());
        let (mut l1, mut l2) = (0u32, 0u32);
        assert_eq!(
            FMOD_Sound_Lock(snd, 0, raw_bytes.len() as u32, &mut p1, &mut p2, &mut l1, &mut l2),
            FMOD_OK
        );
        assert!(!p1.is_null());
        assert_eq!(l1 as usize, raw_bytes.len());
        assert_eq!(l2, 0);
        for i in 0..64 {
            let bytes = (-0.4f32).to_le_bytes();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), p1.add(i * 4), 4);
        }
        assert_eq!(FMOD_Sound_Unlock(snd, p1, p2, l1, l2), FMOD_OK);

        assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
        assert!((out[0] - (-0.4)).abs() < 1e-4, "post-unlock got {}", out[0]);

        // Stopping parent group stops channels in descendant child_grp!
        assert_eq!(FMOD_ChannelGroup_Stop(parent_grp), FMOD_OK);
        let mut playing = 1i32;
        assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
        assert_eq!(playing, 0);

        // Releasing child_grp succeeds; releasing master fails cleanly.
        assert_eq!(FMOD_ChannelGroup_Release(child_grp), FMOD_OK);
        assert_eq!(FMOD_ChannelGroup_Release(master), FMOD_ERR_INVALID_PARAM);
        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_pause_stop_seek_looping_and_pitch_resampling() {
    unsafe {
        let sys = create_nrt_system();

        // 8-sample ramp [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] at 48 kHz mono.
        let ramp: [f32; 8] = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let ex = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: std::mem::size_of_val(&ramp) as u32,
            numchannels: 1,
            defaultfrequency: 48_000,
            format: FMOD_SOUND_FORMAT_PCMFLOAT,
            ..Default::default()
        };
        let mut snd = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                ramp.as_ptr() as *const u8,
                FMOD_OPENMEMORY | FMOD_OPENRAW,
                &ex as *const _ as usize,
                &mut snd,
            ),
            FMOD_OK
        );

        // Start paused: must output silence and stay at position 0.
        let mut ch = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 1, &mut ch), FMOD_OK);
        let mut paused = 0i32;
        assert_eq!(FMOD_Channel_GetPaused(ch, &mut paused), FMOD_OK);
        assert_eq!(paused, 1);

        let mut out = [0.0f32; 4 * 2];
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!(out.iter().all(|&v| v == 0.0));
        let mut pos = 99u32;
        assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(pos, 0);

        // Loop once (plays twice = 16 frames total), unpause, pull 12 frames.
        assert_eq!(FMOD_Channel_SetLoopCount(ch, 1), FMOD_OK);
        assert_eq!(FMOD_Channel_SetPaused(ch, 0), FMOD_OK);
        let mut out12 = [0.0f32; 12 * 2];
        assert_eq!(mix_system_frames(sys, &mut out12), Ok(12));
        assert!((out12[0] - 0.1).abs() < 1e-4);
        assert!((out12[7 * 2] - 0.8).abs() < 1e-4);
        // Frame 8 wrapped back to ramp[0] = 0.1!
        assert!((out12[8 * 2] - 0.1).abs() < 1e-4);
        assert!((out12[11 * 2] - 0.4).abs() < 1e-4);

        // Still playing (4 frames left on second pass), then finishes after 4 more frames.
        let mut playing = 0i32;
        assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
        assert_eq!(playing, 1);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!((out[3 * 2] - 0.8).abs() < 1e-4);
        assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
        assert_eq!(playing, 0);

        // Linear pitch resampling at 0.5x pitch: frames advance 0.0, 0.5, 1.0, 1.5 -> 0.1, 0.15, 0.2, 0.25
        let mut ch2 = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 1, &mut ch2), FMOD_OK);
        assert_eq!(call_set_channel_pitch(ch2, 0.5), FMOD_OK);
        let mut p_out = 0.0f32;
        assert_eq!(FMOD_Channel_GetPitch(ch2, &mut p_out), FMOD_OK);
        assert!((p_out - 0.5).abs() < 1e-5);
        assert_eq!(FMOD_Channel_SetPosition(ch2, 2, FMOD_TIMEUNIT_PCM), FMOD_OK);
        assert_eq!(FMOD_Channel_SetPaused(ch2, 0), FMOD_OK);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        // Starting at frame 2 (0.3), step 0.5 -> 0.30, 0.35, 0.40, 0.45
        assert!((out[0] - 0.30).abs() < 1e-4, "got {}", out[0]);
        assert!((out[2] - 0.35).abs() < 1e-4, "got {}", out[2]);
        assert!((out[4] - 0.40).abs() < 1e-4, "got {}", out[4]);
        assert!((out[6] - 0.45).abs() < 1e-4, "got {}", out[6]);

        // Explicit Stop halts immediately.
        assert_eq!(FMOD_Channel_Stop(ch2), FMOD_OK);
        assert_eq!(FMOD_Channel_IsPlaying(ch2, &mut playing), FMOD_OK);
        assert_eq!(playing, 0);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!(out.iter().all(|&v| v == 0.0));

        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_2d_pan_and_3d_panning_plus_distance_attenuation() {
    unsafe {
        let sys = create_nrt_system();
        assert_eq!(call_set_3d_settings(sys, 1.0, 1.0, 1.0), FMOD_OK);

        let listener_pos = FmodVector { x: 0.0, y: 0.0, z: 0.0 };
        let listener_vel = FmodVector { x: 0.0, y: 0.0, z: 0.0 };
        let listener_fwd = FmodVector { x: 0.0, y: 0.0, z: 1.0 };
        let listener_up = FmodVector { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(
            FMOD_System_Set3DListenerAttributes(
                sys,
                0,
                &listener_pos,
                &listener_vel,
                &listener_fwd,
                &listener_up,
            ),
            FMOD_OK
        );

        let samples = [0.8f32; 64];
        let ex = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: std::mem::size_of_val(&samples) as u32,
            numchannels: 1,
            defaultfrequency: 48_000,
            format: FMOD_SOUND_FORMAT_PCMFLOAT,
            ..Default::default()
        };

        // 2D panning test
        let mut snd2d = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                samples.as_ptr() as *const u8,
                FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL | FMOD_2D,
                &ex as *const _ as usize,
                &mut snd2d,
            ),
            FMOD_OK
        );
        let mut ch2d = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd2d, 0, 0, &mut ch2d), FMOD_OK);
        assert_eq!(call_set_channel_pan(ch2d, -1.0), FMOD_OK);
        let mut out = [0.0f32; 4 * 2];
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!((out[0] - 0.8).abs() < 1e-4);
        assert!(out[1].abs() < 1e-4);
        assert_eq!(FMOD_Channel_Stop(ch2d), FMOD_OK);

        // 3D panning + distance attenuation test
        let mut snd3d = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                samples.as_ptr() as *const u8,
                FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL | FMOD_3D,
                &ex as *const _ as usize,
                &mut snd3d,
            ),
            FMOD_OK
        );
        let mut ch3d = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd3d, 0, 0, &mut ch3d), FMOD_OK);
        assert_eq!(call_set_channel_3d_minmax(ch3d, 1.0, 100.0), FMOD_OK);

        // Source 2m to listener's right (+X): atten = 1/2 = 0.5, pan = +1.0 -> L = 0, R = 0.4
        let pos_right = FmodVector { x: 2.0, y: 0.0, z: 0.0 };
        assert_eq!(FMOD_Channel_Set3DAttributes(ch3d, &pos_right, std::ptr::null()), FMOD_OK);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!(out[0].abs() < 1e-4, "expected L=0, got {}", out[0]);
        assert!((out[1] - 0.4).abs() < 1e-4, "expected R=0.4, got {}", out[1]);

        // Source 4m to listener's left (-X): atten = 1/4 = 0.25, pan = -1.0 -> L = 0.2, R = 0
        let pos_left = FmodVector { x: -4.0, y: 0.0, z: 0.0 };
        assert_eq!(FMOD_Channel_Set3DAttributes(ch3d, &pos_left, std::ptr::null()), FMOD_OK);
        assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
        assert!((out[0] - 0.2).abs() < 1e-4, "expected L=0.2, got {}", out[0]);
        assert!(out[1].abs() < 1e-4, "expected R=0, got {}", out[1]);

        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_realtime_mixer_thread_pumps_audio_backend() {
    let _g = audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let mut sys = 0usize;
        assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);
        assert_eq!(FMOD_System_Init(sys, 16, 0, 0), FMOD_OK);

        let samples = vec![0.25f32; 48_000];
        let ex = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: (samples.len() * std::mem::size_of::<f32>()) as u32,
            numchannels: 1,
            defaultfrequency: 48_000,
            format: FMOD_SOUND_FORMAT_PCMFLOAT,
            ..Default::default()
        };
        let mut snd = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                samples.as_ptr() as *const u8,
                FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL,
                &ex as *const _ as usize,
                &mut snd,
            ),
            FMOD_OK
        );
        let mut ch = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 0, &mut ch), FMOD_OK);

        // Poll briefly until the real mixer thread pushes blocks into the backend ring.
        let mut mixed = 0u64;
        let mut pos = 0u32;
        for _ in 0..50 {
            std::thread::sleep(Duration::from_millis(10));
            mixed = lookup_system(sys).unwrap().mixed_frames.load(Ordering::Relaxed);
            assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos, FMOD_TIMEUNIT_PCM), FMOD_OK);
            if mixed >= MIX_BLOCK_FRAMES as u64 && pos > 0 {
                break;
            }
        }
        assert!(mixed >= MIX_BLOCK_FRAMES as u64, "mixer thread did not pump frames (mixed={mixed})");
        assert!(pos > 0, "channel play position did not advance on mixer thread");

        assert_eq!(FMOD_System_Close(sys), FMOD_OK);
        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_honest_errors_on_unsupported_containers_and_studio_banks() {
    unsafe {
        let mut studio = 0usize;
        assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
        assert_ne!(studio, 0);

        let mut core1 = 0usize;
        let mut core2 = 0usize;
        assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core1), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core2), FMOD_OK);
        assert_ne!(core1, 0);
        assert_eq!(core1, core2, "GetCoreSystem must return the same linked Core handle");

        // Configure linked Core system NRT and initialize via Studio.
        assert_eq!(FMOD_System_SetOutput(core1, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
        assert_eq!(FMOD_Studio_System_Initialize(studio, 32, 0, 0, 0), FMOD_OK);
        assert_eq!(FMOD_Studio_System_Update(studio), FMOD_OK);

        // Unsupported proprietary FSB5 bank passed to Core CreateSound -> FMOD_ERR_FORMAT (19)
        let fsb5_blob = b"FSB5\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let ex = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: fsb5_blob.len() as u32,
            ..Default::default()
        };
        let mut bad_snd = 0xDEADusize;
        assert_eq!(
            FMOD_System_CreateSound(
                core1,
                fsb5_blob.as_ptr(),
                FMOD_OPENMEMORY,
                &ex as *const _ as usize,
                &mut bad_snd,
            ),
            FMOD_ERR_FORMAT
        );
        assert_eq!(bad_snd, 0);

        // Non-existent Studio .bank file -> FMOD_ERR_FILE_NOTFOUND (18), null handle
        let mut bank = 0xDEADusize;
        assert_eq!(
            FMOD_Studio_System_LoadBankFile(studio, b"Master.bank\0".as_ptr(), 0, &mut bank),
            FMOD_ERR_FILE_NOTFOUND
        );
        assert_eq!(bank, 0);

        // Corrupt/truncated buffer -> FMOD_ERR_FORMAT (19), null handle
        bank = 0xDEADusize;
        assert_eq!(
            FMOD_Studio_System_LoadBankMemory(
                studio,
                fsb5_blob.as_ptr(),
                fsb5_blob.len() as i32,
                FMOD_STUDIO_LOAD_MEMORY,
                FMOD_STUDIO_LOAD_BANK_NORMAL,
                &mut bank,
            ),
            FMOD_ERR_FORMAT
        );
        assert_eq!(bank, 0);

        // Missing Studio event -> FMOD_ERR_EVENT_NOTFOUND (74), null handle
        let mut ev = 0xDEADusize;
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"event:/SFX/Explosion\0".as_ptr(), &mut ev),
            FMOD_ERR_EVENT_NOTFOUND
        );
        assert_eq!(ev, 0);

        // Creating/controlling non-existent Studio event instances fails honestly.
        let mut inst = 0xDEADusize;
        assert_eq!(
            FMOD_Studio_EventDescription_CreateInstance(0, &mut inst),
            FMOD_ERR_INVALID_HANDLE
        );
        assert_eq!(inst, 0);
        assert_eq!(FMOD_Studio_EventInstance_Start(0), FMOD_ERR_INVALID_HANDLE);
        assert_eq!(FMOD_Studio_EventInstance_Stop(0, 0), FMOD_ERR_INVALID_HANDLE);
        assert_eq!(FMOD_Studio_EventInstance_Release(0), FMOD_ERR_INVALID_HANDLE);

        // Releasing Studio also releases its linked Core system.
        assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
        assert_eq!(FMOD_System_Update(core1), FMOD_ERR_INVALID_HANDLE);
    }
}

// ------------------------------------------------------------------------
// Studio float-ABI test wrappers & synthetic FSB5 / RIFF FEV bank builders
// ------------------------------------------------------------------------

unsafe fn call_studio_inst_set_volume(inst: usize, vol: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_EventInstance_SetVolume(inst, 0.0, vol)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_EventInstance_SetVolume(inst, vol)
    }
}

unsafe fn call_studio_inst_set_pitch(inst: usize, pitch: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_EventInstance_SetPitch(inst, 0.0, pitch)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_EventInstance_SetPitch(inst, pitch)
    }
}

unsafe fn call_studio_inst_set_param_name(
    inst: usize,
    name: *const u8,
    val: f32,
    ignore: i32,
) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_EventInstance_SetParameterByName(inst, name, 0, ignore, 0.0, 0.0, val)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_EventInstance_SetParameterByName(inst, name, val, ignore)
    }
}

unsafe fn call_studio_inst_set_param_id(
    inst: usize,
    id: FmodStudioParameterId,
    val: f32,
    ignore: i32,
) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_EventInstance_SetParameterByID(inst, id.to_u64(), 0, ignore, 0.0, 0.0, val)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_EventInstance_SetParameterByID(inst, id.to_u64(), val, ignore)
    }
}

unsafe fn call_studio_sys_set_param_name(
    sys: usize,
    name: *const u8,
    val: f32,
    ignore: i32,
) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_System_SetParameterByName(sys, name, 0, ignore, 0.0, 0.0, val)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_System_SetParameterByName(sys, name, val, ignore)
    }
}

unsafe fn call_studio_sys_set_listener_weight(sys: usize, idx: i32, weight: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_System_SetListenerWeight(sys, idx, 0.0, 0.0, weight)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_System_SetListenerWeight(sys, idx, weight)
    }
}

unsafe fn call_studio_bus_set_volume(bus: usize, vol: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_Bus_SetVolume(bus, 0.0, vol)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_Bus_SetVolume(bus, vol)
    }
}

unsafe fn call_studio_vca_set_volume(vca: usize, vol: f32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        FMOD_Studio_VCA_SetVolume(vca, 0.0, vol)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        FMOD_Studio_VCA_SetVolume(vca, vol)
    }
}

struct TestFsbSample {
    name: String,
    channels: u32,
    sample_rate: u32,
    num_frames: u32,
    loop_points: Option<(u32, u32)>,
    data: Vec<u8>,
}

fn build_fsb5(version: u32, mode: u32, samples: &[TestFsbSample]) -> Vec<u8> {
    let mut sh = Vec::new();
    let mut data_block = Vec::new();
    let mut has_names = false;
    for s in samples {
        if !s.name.is_empty() {
            has_names = true;
        }
    }

    for s in samples {
        // Align each sample's data offset to 16 bytes.
        while data_block.len() % 16 != 0 {
            data_block.push(0);
        }
        let data_off_units = (data_block.len() / 16) as u64;
        data_block.extend_from_slice(&s.data);

        let freq_idx_opt = FSB5_SAMPLE_RATES
            .iter()
            .position(|&r| r == s.sample_rate && r > 0);
        let freq_idx = freq_idx_opt.unwrap_or(0) as u64;

        let (ch_bits, need_ch_chunk): (u64, bool) = match s.channels {
            1 => (0, false),
            2 => (1, false),
            6 => (2, false),
            8 => (3, false),
            _ => (0, true),
        };
        let need_freq_chunk = freq_idx_opt.is_none();

        let mut chunks: Vec<(u32, Vec<u8>)> = Vec::new();
        if need_ch_chunk {
            chunks.push((FSB5_CHUNK_CHANNELS, vec![s.channels as u8]));
        }
        if need_freq_chunk {
            chunks.push((FSB5_CHUNK_FREQUENCY, s.sample_rate.to_le_bytes().to_vec()));
        }
        if let Some((ls, le)) = s.loop_points {
            let mut lp = Vec::with_capacity(8);
            lp.extend_from_slice(&ls.to_le_bytes());
            lp.extend_from_slice(&le.to_le_bytes());
            chunks.push((FSB5_CHUNK_LOOP, lp));
        }

        let has_chunks = if chunks.is_empty() { 0u64 } else { 1u64 };
        let raw_hdr: u64 = has_chunks
            | ((freq_idx & 0x0F) << 1)
            | ((ch_bits & 0x03) << 5)
            | ((data_off_units & 0x07FF_FFFF) << 7)
            | (((s.num_frames as u64) & 0x3FFF_FFFF) << 34);
        sh.extend_from_slice(&raw_hdr.to_le_bytes());

        for (ci, (ctype, payload)) in chunks.iter().enumerate() {
            let more = if ci + 1 < chunks.len() { 1u32 } else { 0u32 };
            let chdr = more
                | (((payload.len() as u32) & 0x00FF_FFFF) << 1)
                | ((ctype & 0x7F) << 25);
            sh.extend_from_slice(&chdr.to_le_bytes());
            sh.extend_from_slice(payload);
        }
    }

    let mut name_table = Vec::new();
    if has_names {
        let mut str_pool = Vec::new();
        let base_off = (samples.len() * 4) as u32;
        for s in samples {
            let off = base_off + str_pool.len() as u32;
            name_table.extend_from_slice(&off.to_le_bytes());
            str_pool.extend_from_slice(s.name.as_bytes());
            str_pool.push(0);
        }
        name_table.extend_from_slice(&str_pool);
    }

    let header_size = if version == 0 { 64usize } else { 60usize };
    let mut out = Vec::with_capacity(header_size + sh.len() + name_table.len() + data_block.len());
    out.extend_from_slice(b"FSB5");
    out.extend_from_slice(&version.to_le_bytes());
    out.extend_from_slice(&(samples.len() as u32).to_le_bytes());
    out.extend_from_slice(&(sh.len() as u32).to_le_bytes());
    out.extend_from_slice(&(name_table.len() as u32).to_le_bytes());
    out.extend_from_slice(&(data_block.len() as u32).to_le_bytes());
    out.extend_from_slice(&mode.to_le_bytes());
    out.resize(header_size, 0);
    out.extend_from_slice(&sh);
    out.extend_from_slice(&name_table);
    out.extend_from_slice(&data_block);
    out
}

/// Build a single 140-byte mono (or 280-byte stereo) FADPCM frame encoding constant
/// levels per channel using `coef_idx = 0` (`[0, 0]`) and `raw_shift = 12` (`shift = 10`,
/// nibble * 4096 => +4 = +16384 = +0.5, -2 = -8192 = -0.25).
fn build_fadpcm_constant_frame(channel_nibbles: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(140 * channel_nibbles.len());
    for &nib in channel_nibbles {
        let n = nib & 0x0F;
        let coefs = 0u32; // filter 0 = [0, 0] for all 8 sections
        let shifts = 0xCCCC_CCCCu32; // raw_shift = 12 for all 8 sections
        out.extend_from_slice(&coefs.to_le_bytes());
        out.extend_from_slice(&shifts.to_le_bytes());
        out.extend_from_slice(&0i16.to_le_bytes()); // hist1
        out.extend_from_slice(&0i16.to_le_bytes()); // hist2
        let byte = n | (n << 4);
        out.extend(std::iter::repeat(byte).take(128));
    }
    out
}

/// Build a single 36-byte/channel FMOD IMA-ADPCM block (64 samples per channel).
/// Setting `step_index = 0` (step = 7 => diff = 7 >> 3 = 0 for nibble 0) preserves
/// `predictor` exactly across all 64 decoded samples!
fn build_ima_constant_block(channel_predictors: &[i16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(36 * channel_predictors.len());
    for &pred in channel_predictors {
        out.extend_from_slice(&pred.to_le_bytes());
        out.push(0); // step_index = 0
        out.push(0); // reserved
    }
    out.resize(36 * channel_predictors.len(), 0); // all nibbles = 0
    out
}

fn riff_chunk(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len() + (payload.len() & 1));
    out.extend_from_slice(fourcc);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if (payload.len() & 1) != 0 {
        out.push(0);
    }
    out
}

fn riff_list(list_type: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(list_type);
    for c in chunks {
        body.extend_from_slice(c);
    }
    riff_chunk(b"LIST", &body)
}

fn wrap_fev_bank(chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(b"FEV ");
    for c in chunks {
        body.extend_from_slice(c);
    }
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

fn push_len_str(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
    buf.extend_from_slice(s.as_bytes());
}

#[test]
fn fmod_fsb5_codecs_core_create_sound_and_subsounds() {
    unsafe {
        let sys = create_nrt_system();

        // 1) FSB5 v0 with PCM16 + custom frequency chunk (24 kHz) + 4-ch quad CHANNELS chunk + LOOP chunk + name table
        let mut quad_pcm16 = Vec::new();
        for frame in 0..8 {
            let amp = if frame < 4 { 8192i16 } else { 16384i16 }; // 0.25 then 0.5
            for _ in 0..4 {
                quad_pcm16.extend_from_slice(&amp.to_le_bytes());
            }
        }
        let fsb_v0 = build_fsb5(
            0,
            FSB5_MODE_PCM16,
            &[
                TestFsbSample {
                    name: "QuadLoop".into(),
                    channels: 4,
                    sample_rate: 48_000,
                    num_frames: 8,
                    loop_points: Some((4, 7)), // loops over frames 4..=7 (+0.5)
                    data: quad_pcm16,
                },
                TestFsbSample {
                    name: "CustomRate".into(),
                    channels: 1,
                    sample_rate: 30_000, // forces CHUNK_FREQUENCY
                    num_frames: 4,
                    loop_points: None,
                    data: [16384i16, 16384, 16384, 16384]
                        .iter()
                        .flat_map(|v| v.to_le_bytes())
                        .collect(),
                },
            ],
        );

        let ex0 = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: fsb_v0.len() as u32,
            ..Default::default()
        };
        let mut snd_v0 = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                fsb_v0.as_ptr(),
                FMOD_OPENMEMORY,
                &ex0 as *const _ as usize,
                &mut snd_v0,
            ),
            FMOD_OK
        );

        let (mut stype, mut fmt, mut chs, mut bits) = (0u32, 0u32, 0i32, 0i32);
        assert_eq!(
            FMOD_Sound_GetFormat(snd_v0, &mut stype, &mut fmt, &mut chs, &mut bits),
            FMOD_OK
        );
        assert_eq!((stype, fmt, chs, bits), (FMOD_SOUND_TYPE_FSB, FMOD_SOUND_FORMAT_PCM16, 4, 16));

        let mut name_buf = [0u8; 64];
        assert_eq!(FMOD_Sound_GetName(snd_v0, name_buf.as_mut_ptr(), 64), FMOD_OK);
        assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "QuadLoop");

        let mut nsubs = 0i32;
        assert_eq!(FMOD_Sound_GetNumSubSounds(snd_v0, &mut nsubs), FMOD_OK);
        assert_eq!(nsubs, 2);

        let mut sub1 = 0usize;
        assert_eq!(FMOD_Sound_GetSubSound(snd_v0, 1, &mut sub1), FMOD_OK);
        assert_ne!(sub1, 0);
        assert_eq!(FMOD_Sound_GetName(sub1, name_buf.as_mut_ptr(), 64), FMOD_OK);
        assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "CustomRate");
        let mut sub1_freq = 0.0f32;
        assert_eq!(FMOD_Sound_GetDefaults(sub1, &mut sub1_freq, std::ptr::null_mut()), FMOD_OK);
        assert_eq!(sub1_freq, 30_000.0);

        // Play QuadLoop (has LOOP chunk 4..=7): frames 0..4 are 0.25, frames 4..12 loop over 0.5!
        let mut ch0 = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd_v0, 0, 0, &mut ch0), FMOD_OK);
        let mut mix = [0.0f32; 12 * 2];
        assert_eq!(mix_system_frames(sys, &mut mix), Ok(12));
        assert!((mix[0] - 0.25).abs() < 1e-3, "frame 0 got {}", mix[0]);
        assert!((mix[4 * 2] - 0.5).abs() < 1e-3, "frame 4 got {}", mix[4 * 2]);
        assert!((mix[10 * 2] - 0.5).abs() < 1e-3, "looped frame 10 got {}", mix[10 * 2]);
        assert_eq!(FMOD_Channel_Stop(ch0), FMOD_OK);

        // 2) FSB5 v1 FADPCM stereo (+0.5 L, -0.25 R)
        let fadpcm_data = build_fadpcm_constant_frame(&[4, 0xE]); // +4*4096=+16384 (+0.5), -2*4096=-8192 (-0.25)
        let fsb_fadpcm = build_fsb5(
            1,
            FSB5_MODE_FADPCM,
            &[TestFsbSample {
                name: "FadpcmTone".into(),
                channels: 2,
                sample_rate: 48_000,
                num_frames: 64,
                loop_points: None,
                data: fadpcm_data,
            }],
        );
        let ex_fad = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: fsb_fadpcm.len() as u32,
            ..Default::default()
        };
        let mut snd_fad = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                fsb_fadpcm.as_ptr(),
                FMOD_OPENMEMORY,
                &ex_fad as *const _ as usize,
                &mut snd_fad,
            ),
            FMOD_OK
        );
        let mut ch_fad = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd_fad, 0, 0, &mut ch_fad), FMOD_OK);
        let mut out_fad = [0.0f32; 8 * 2];
        assert_eq!(mix_system_frames(sys, &mut out_fad), Ok(8));
        for f in 0..8 {
            assert!((out_fad[f * 2] - 0.5).abs() < 1e-3, "FADPCM L[{f}]={}", out_fad[f * 2]);
            assert!((out_fad[f * 2 + 1] - (-0.25)).abs() < 1e-3, "FADPCM R[{f}]={}", out_fad[f * 2 + 1]);
        }
        assert_eq!(FMOD_Channel_Stop(ch_fad), FMOD_OK);

        // 3) FSB5 v1 IMA-ADPCM stereo (+0.75 L, -0.5 R)
        let ima_data = build_ima_constant_block(&[24_576, -16_384]);
        let fsb_ima = build_fsb5(
            1,
            FSB5_MODE_IMAADPCM,
            &[TestFsbSample {
                name: "ImaTone".into(),
                channels: 2,
                sample_rate: 48_000,
                num_frames: 64,
                loop_points: None,
                data: ima_data,
            }],
        );
        let ex_ima = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: fsb_ima.len() as u32,
            ..Default::default()
        };
        let mut snd_ima = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                fsb_ima.as_ptr(),
                FMOD_OPENMEMORY,
                &ex_ima as *const _ as usize,
                &mut snd_ima,
            ),
            FMOD_OK
        );
        let mut ch_ima = 0usize;
        assert_eq!(FMOD_System_PlaySound(sys, snd_ima, 0, 0, &mut ch_ima), FMOD_OK);
        let mut out_ima = [0.0f32; 8 * 2];
        assert_eq!(mix_system_frames(sys, &mut out_ima), Ok(8));
        for f in 0..8 {
            assert!((out_ima[f * 2] - 0.75).abs() < 1e-3, "IMA L[{f}]={}", out_ima[f * 2]);
            assert!((out_ima[f * 2 + 1] - (-0.5)).abs() < 1e-3, "IMA R[{f}]={}", out_ima[f * 2 + 1]);
        }
        assert_eq!(FMOD_Channel_Stop(ch_ima), FMOD_OK);

        // 4) FSB5 PCM8, PCM24, PCM32, and PCMFloat codecs
        let fsb_pcm8 = build_fsb5(
            1,
            FSB5_MODE_PCM8,
            &[TestFsbSample {
                name: "Pcm8".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 4,
                loop_points: None,
                data: vec![192u8; 4], // (192-128)/128 = +0.5
            }],
        );
        let fsb_pcm24 = build_fsb5(
            1,
            FSB5_MODE_PCM24,
            &[TestFsbSample {
                name: "Pcm24".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 4,
                loop_points: None,
                data: [0x00u8, 0x00, 0xC0].repeat(4), // -0.5
            }],
        );
        let fsb_pcm32 = build_fsb5(
            1,
            FSB5_MODE_PCM32,
            &[TestFsbSample {
                name: "Pcm32".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 4,
                loop_points: None,
                data: (i32::MAX / 2 + 1).to_le_bytes().repeat(4), // +0.5
            }],
        );
        let fsb_pcmf = build_fsb5(
            1,
            FSB5_MODE_PCMFLOAT,
            &[TestFsbSample {
                name: "PcmFloat".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 4,
                loop_points: None,
                data: 0.625f32.to_le_bytes().repeat(4),
            }],
        );
        for (blob, expected_amp) in [
            (&fsb_pcm8, 0.5f32),
            (&fsb_pcm24, -0.5f32),
            (&fsb_pcm32, 0.5f32),
            (&fsb_pcmf, 0.625f32),
        ] {
            let ex_p = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: blob.len() as u32,
                ..Default::default()
            };
            let mut snd_p = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    blob.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex_p as *const _ as usize,
                    &mut snd_p,
                ),
                FMOD_OK
            );
            let mut ch_p = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd_p, 0, 0, &mut ch_p), FMOD_OK);
            let mut out_p = [0.0f32; 4 * 2];
            assert_eq!(mix_system_frames(sys, &mut out_p), Ok(4));
            assert!((out_p[0] - expected_amp).abs() < 1e-3, "got {} expected {expected_amp}", out_p[0]);
            assert_eq!(FMOD_Sound_Release(snd_p), FMOD_OK);
        }

        // 5) Non-trivial FADPCM filter prediction (coef_idx=1: [60, 0], hist1=16384, raw_shift=10, nib=+1 -> 16384)
        let mut pred_fadpcm = Vec::with_capacity(140);
        pred_fadpcm.extend_from_slice(&0x1111_1111u32.to_le_bytes()); // coef_idx = 1 ([60, 0])
        pred_fadpcm.extend_from_slice(&0xAAAA_AAAAu32.to_le_bytes()); // raw_shift = 10 (shift = 12)
        pred_fadpcm.extend_from_slice(&16_384i16.to_le_bytes()); // hist1 = 16384
        pred_fadpcm.extend_from_slice(&0i16.to_le_bytes()); // hist2 = 0
        pred_fadpcm.extend(std::iter::repeat(0x11u8).take(128)); // nibble = +1 -> residual = 1024 -> (1024*64 + 16384*60 + 32) >> 6 = 16384!
        let dec_pred = decode_fsb5_fadpcm(&pred_fadpcm, 1, 16);
        assert_eq!(dec_pred.len(), 16);
        for (i, &v) in dec_pred.iter().enumerate() {
            assert!((v - 0.5).abs() < 1e-4, "FADPCM filter1[{i}]={v}");
        }

        // 6) Non-trivial IMA-ADPCM adaptive step transitions (predictor=0, step_index=10, nibs 5 then 10 -> 25 then 8)
        let mut adapt_ima = vec![0u8; 36];
        adapt_ima[2] = 10; // step_index = 10 (step = 19)
        adapt_ima[4] = 0xA5; // low nibble = 5 (+25, idx->14 step=28), high nibble = 10 (-17 -> 8)
        let dec_ima = decode_fsb5_ima_adpcm(&adapt_ima, 1, 2);
        assert!((dec_ima[0] - (25.0 / 32_768.0)).abs() < 1e-5, "IMA[0]={}", dec_ima[0]);
        assert!((dec_ima[1] - (8.0 / 32_768.0)).abs() < 1e-5, "IMA[1]={}", dec_ima[1]);

        // 7) Unsupported codec mode inside an otherwise valid FSB5 fails cleanly with FMOD_ERR_FORMAT
        let fsb_bad_codec = build_fsb5(
            1,
            15, // Vorbis (unsupported here)
            &[TestFsbSample {
                name: "Bad".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 4,
                loop_points: None,
                data: vec![0u8; 16],
            }],
        );
        let ex_bad = FmodCreateSoundExInfoHeader {
            cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
            length: fsb_bad_codec.len() as u32,
            ..Default::default()
        };
        let mut bad_snd = 0usize;
        assert_eq!(
            FMOD_System_CreateSound(
                sys,
                fsb_bad_codec.as_ptr(),
                FMOD_OPENMEMORY,
                &ex_bad as *const _ as usize,
                &mut bad_snd,
            ),
            FMOD_ERR_FORMAT
        );

        assert_eq!(FMOD_Sound_Release(snd_v0), FMOD_OK);
        assert_eq!(FMOD_Sound_Release(snd_fad), FMOD_OK);
        assert_eq!(FMOD_Sound_Release(snd_ima), FMOD_OK);
        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}

#[test]
fn fmod_studio_riff_fev_bank_playback_buses_vcas_3d_params_and_unload() {
    unsafe {
        let mut studio = 0usize;
        assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
        assert_eq!(FMOD_Studio_System_IsValid(studio), 1);

        let mut core = 0usize;
        assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core), FMOD_OK);
        assert_eq!(FMOD_System_SetOutput(core, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
        assert_eq!(FMOD_Studio_System_Initialize(studio, 64, 0, 0, 0), FMOD_OK);

        // Deterministic GUIDs for strings bank, SFX bank, buses, VCA, and 3 events.
        let strings_bank_guid = FmodGuid::parse_str("{11111111-1111-1111-1111-111111111111}").unwrap();
        let sfx_bank_guid = FmodGuid::parse_str("{22222222-2222-2222-2222-222222222222}").unwrap();
        let bus_sfx_guid = FmodGuid::parse_str("{33333333-3333-3333-3333-333333333333}").unwrap();
        let bus_weapons_guid = FmodGuid::parse_str("{44444444-4444-4444-4444-444444444444}").unwrap();
        let vca_sfx_guid = FmodGuid::parse_str("{55555555-5555-5555-5555-555555555555}").unwrap();
        let ev_pcm16_guid = FmodGuid::parse_str("{aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa}").unwrap();
        let ev_fadpcm_guid = FmodGuid::parse_str("{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}").unwrap();
        let ev_ima3d_guid = FmodGuid::parse_str("{cccccccc-cccc-cccc-cccc-cccccccccccc}").unwrap();

        // 1) Synthesize Master.strings.bank using a real STRR Radix Trie + STRT flat entries,
        //    and load it from a virtual Windows path (`crate::path`).
        let pool = b"event:/SFX/TonePCM16ToneFADPCMSpatialIMA";
        let mut strr = Vec::new();
        strr.extend_from_slice(&3u32.to_le_bytes()); // guid_count
        strr.extend_from_slice(&4u32.to_le_bytes()); // node_count
        strr.extend_from_slice(&(pool.len() as u32).to_le_bytes());
        strr.extend_from_slice(&ev_pcm16_guid.to_le_bytes());
        strr.extend_from_slice(&ev_fadpcm_guid.to_le_bytes());
        strr.extend_from_slice(&ev_ima3d_guid.to_le_bytes());
        let nodes: [(i32, i32, u32, u32); 4] = [
            (-1, -1, 0, 11),  // "event:/SFX/"
            (0, 0, 11, 9),    // "TonePCM16"
            (0, 1, 20, 10),   // "ToneFADPCM"
            (0, 2, 30, 10),   // "SpatialIMA"
        ];
        for (p, gi, off, len) in nodes {
            strr.extend_from_slice(&p.to_le_bytes());
            strr.extend_from_slice(&gi.to_le_bytes());
            strr.extend_from_slice(&off.to_le_bytes());
            strr.extend_from_slice(&len.to_le_bytes());
        }
        strr.extend_from_slice(pool);

        // Flat STRT (type 0) for buses, VCA, and bank paths.
        let mut strt = Vec::new();
        strt.extend_from_slice(&0u32.to_le_bytes()); // type = 0 (inline)
        strt.extend_from_slice(&5u32.to_le_bytes()); // count = 5
        for (g, p) in [
            (strings_bank_guid, "bank:/Master.strings"),
            (sfx_bank_guid, "bank:/SFX"),
            (bus_sfx_guid, "bus:/SFX"),
            (bus_weapons_guid, "bus:/SFX/Weapons"),
            (vca_sfx_guid, "vca:/MasterSFX"),
        ] {
            strt.extend_from_slice(&g.to_le_bytes());
            push_len_str(&mut strt, p);
        }

        let strings_bank_bytes = wrap_fev_bank(&[
            riff_chunk(b"FMT ", &0x0000_0089u32.to_le_bytes()),
            riff_list(
                b"PROJ",
                &[
                    riff_chunk(b"BNKI", &strings_bank_guid.to_le_bytes()),
                    riff_chunk(b"STRR", &strr),
                    riff_chunk(b"STRT", &strt),
                ],
            ),
        ]);

        let tmp_dir = std::env::temp_dir().join("win32_rt_fmod_studio_bank_test");
        let _ = std::fs::create_dir_all(&tmp_dir);
        let strings_file = tmp_dir.join("Master.strings.bank");
        std::fs::write(&strings_file, &strings_bank_bytes).unwrap();
        crate::path::add_mount("G:\\FmodStudio", &tmp_dir);

        let mut strings_bank = 0usize;
        assert_eq!(
            FMOD_Studio_System_LoadBankFile(
                studio,
                b"G:\\FmodStudio\\Master.strings.bank\0".as_ptr(),
                FMOD_STUDIO_LOAD_BANK_NORMAL,
                &mut strings_bank,
            ),
            FMOD_OK
        );
        assert_eq!(FMOD_Studio_Bank_IsValid(strings_bank), 1);

        // Duplicate load of the same BNKI GUID returns FMOD_ERR_EVENT_ALREADY_LOADED.
        let mut dup_bank = 0xDEADusize;
        assert_eq!(
            FMOD_Studio_System_LoadBankMemory(
                studio,
                strings_bank_bytes.as_ptr(),
                strings_bank_bytes.len() as i32,
                FMOD_STUDIO_LOAD_MEMORY,
                FMOD_STUDIO_LOAD_BANK_NORMAL,
                &mut dup_bank,
            ),
            FMOD_ERR_EVENT_ALREADY_LOADED
        );
        assert_eq!(dup_bank, 0);

        // LookupID / LookupPath works from Master.strings.bank before SFX.bank is loaded,
        // while GetEvent still honestly returns FMOD_ERR_EVENT_NOTFOUND!
        let mut looked_up = FmodGuid::ZERO;
        assert_eq!(
            FMOD_Studio_System_LookupID(studio, b"event:/SFX/TonePCM16\0".as_ptr(), &mut looked_up),
            FMOD_OK
        );
        assert_eq!(looked_up, ev_pcm16_guid);
        let mut missing_ev = 0usize;
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"event:/SFX/TonePCM16\0".as_ptr(), &mut missing_ev),
            FMOD_ERR_EVENT_NOTFOUND
        );

        // 2) Synthesize SFX.bank (RIFF FEV)
        let mut buss = Vec::new();
        buss.extend_from_slice(&2u32.to_le_bytes());
        buss.extend_from_slice(&bus_sfx_guid.to_le_bytes());
        buss.extend_from_slice(&FmodGuid::ZERO.to_le_bytes());
        buss.extend_from_slice(&1.0f32.to_le_bytes());
        buss.extend_from_slice(&0u32.to_le_bytes());
        push_len_str(&mut buss, "");
        buss.extend_from_slice(&bus_weapons_guid.to_le_bytes());
        buss.extend_from_slice(&bus_sfx_guid.to_le_bytes());
        buss.extend_from_slice(&1.0f32.to_le_bytes());
        buss.extend_from_slice(&0u32.to_le_bytes());
        push_len_str(&mut buss, "");

        let mut vcas = Vec::new();
        vcas.extend_from_slice(&1u32.to_le_bytes());
        vcas.extend_from_slice(&vca_sfx_guid.to_le_bytes());
        vcas.extend_from_slice(&1.0f32.to_le_bytes());
        push_len_str(&mut vcas, "");
        vcas.extend_from_slice(&1u32.to_le_bytes()); // target_bus_count = 1
        vcas.extend_from_slice(&bus_sfx_guid.to_le_bytes());
        vcas.extend_from_slice(&0u32.to_le_bytes()); // target_event_count = 0

        let mut evnt = Vec::new();
        evnt.extend_from_slice(&3u32.to_le_bytes());
        // Event 0: PCM16 stereo (+0.8 L, +0.4 R), 2D oneshot, routed to bus:/SFX/Weapons
        evnt.extend_from_slice(&ev_pcm16_guid.to_le_bytes());
        evnt.extend_from_slice(&bus_weapons_guid.to_le_bytes());
        evnt.extend_from_slice(&0u32.to_le_bytes());
        evnt.extend_from_slice(&0x02u32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&50.0f32.to_le_bytes());
        evnt.extend_from_slice(&0u32.to_le_bytes());
        push_len_str(&mut evnt, "");
        // Event 1: FADPCM mono (+0.5), 2D looping, routed to bus:/SFX
        evnt.extend_from_slice(&ev_fadpcm_guid.to_le_bytes());
        evnt.extend_from_slice(&bus_sfx_guid.to_le_bytes());
        evnt.extend_from_slice(&1u32.to_le_bytes());
        evnt.extend_from_slice(&0x00u32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&50.0f32.to_le_bytes());
        evnt.extend_from_slice(&0u32.to_le_bytes());
        push_len_str(&mut evnt, "");
        // Event 2: IMA-ADPCM mono (+0.8), 3D looping, routed to bus:/SFX
        evnt.extend_from_slice(&ev_ima3d_guid.to_le_bytes());
        evnt.extend_from_slice(&bus_sfx_guid.to_le_bytes());
        evnt.extend_from_slice(&2u32.to_le_bytes());
        evnt.extend_from_slice(&0x01u32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&1.0f32.to_le_bytes());
        evnt.extend_from_slice(&100.0f32.to_le_bytes());
        evnt.extend_from_slice(&0u32.to_le_bytes());
        push_len_str(&mut evnt, "");

        let intensity_id = FmodStudioParameterId { data1: 0x1001, data2: 0x2002 };
        let distance_id = FmodStudioParameterId { data1: 0x3003, data2: 0x4004 };
        let tod_id = FmodStudioParameterId { data1: 0x5005, data2: 0x6006 };
        let mut parm = Vec::new();
        parm.extend_from_slice(&3u32.to_le_bytes());
        for (owner, pid, min_v, max_v, def_v, flags, ptype, pname) in [
            (ev_pcm16_guid, intensity_id, 0.0f32, 10.0f32, 2.5f32, 0u32, FMOD_STUDIO_PARAMETER_GAME_CONTROLLED, "Intensity"),
            (ev_ima3d_guid, distance_id, 0.0f32, 100.0f32, 0.0f32, FMOD_STUDIO_PARAMETER_AUTOMATIC, FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE, "Distance"),
            (FmodGuid::ZERO, tod_id, 0.0f32, 24.0f32, 12.0f32, FMOD_STUDIO_PARAMETER_GLOBAL, FMOD_STUDIO_PARAMETER_GAME_CONTROLLED, "TimeOfDay"),
        ] {
            parm.extend_from_slice(&owner.to_le_bytes());
            parm.extend_from_slice(&pid.to_u64().to_le_bytes());
            parm.extend_from_slice(&min_v.to_le_bytes());
            parm.extend_from_slice(&max_v.to_le_bytes());
            parm.extend_from_slice(&def_v.to_le_bytes());
            parm.extend_from_slice(&flags.to_le_bytes());
            parm.extend_from_slice(&ptype.to_le_bytes());
            push_len_str(&mut parm, pname);
        }

        // Build 3 embedded FSB5 sub-banks inside a single SND chunk (with leading alignment padding).
        let mut pcm16_data = Vec::new();
        for _ in 0..480 {
            pcm16_data.extend_from_slice(&26_214i16.to_le_bytes());
            pcm16_data.extend_from_slice(&13_107i16.to_le_bytes());
        }
        let fsb_pcm16 = build_fsb5(
            1,
            FSB5_MODE_PCM16,
            &[TestFsbSample {
                name: "TonePCM16".into(),
                channels: 2,
                sample_rate: 48_000,
                num_frames: 480,
                loop_points: None,
                data: pcm16_data,
            }],
        );
        let fsb_fadpcm = build_fsb5(
            1,
            FSB5_MODE_FADPCM,
            &[TestFsbSample {
                name: "ToneFADPCM".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 256,
                loop_points: None,
                data: build_fadpcm_constant_frame(&[4]), // +0.5 mono
            }],
        );
        let fsb_ima = build_fsb5(
            1,
            FSB5_MODE_IMAADPCM,
            &[TestFsbSample {
                name: "SpatialIMA".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 64,
                loop_points: None,
                data: build_ima_constant_block(&[26_214]), // +0.8 mono
            }],
        );

        let mut snd_payload = vec![0u8; 16]; // 16-byte alignment padding before first FSB5
        snd_payload.extend_from_slice(&fsb_pcm16);
        while snd_payload.len() % 16 != 0 {
            snd_payload.push(0);
        }
        snd_payload.extend_from_slice(&fsb_fadpcm);
        while snd_payload.len() % 16 != 0 {
            snd_payload.push(0);
        }
        snd_payload.extend_from_slice(&fsb_ima);

        let sfx_bank_bytes = wrap_fev_bank(&[
            riff_chunk(b"FMT ", &0x0000_0089u32.to_le_bytes()),
            riff_chunk(b"BNKI", &sfx_bank_guid.to_le_bytes()),
            riff_list(
                b"MODL",
                &[
                    riff_chunk(b"BUSS", &buss),
                    riff_chunk(b"VCAS", &vcas),
                    riff_chunk(b"EVNT", &evnt),
                    riff_chunk(b"PARM", &parm),
                ],
            ),
            riff_chunk(b"SNDH", &1u32.to_le_bytes()),
            riff_chunk(b"SND ", &snd_payload),
        ]);

        let mut sfx_bank = 0usize;
        assert_eq!(
            FMOD_Studio_System_LoadBankMemory(
                studio,
                sfx_bank_bytes.as_ptr(),
                sfx_bank_bytes.len() as i32,
                FMOD_STUDIO_LOAD_MEMORY,
                FMOD_STUDIO_LOAD_BANK_NORMAL,
                &mut sfx_bank,
            ),
            FMOD_OK
        );
        assert_eq!(FMOD_Studio_Bank_IsValid(sfx_bank), 1);

        let mut load_state = 0u32;
        assert_eq!(FMOD_Studio_Bank_GetLoadingState(sfx_bank, &mut load_state), FMOD_OK);
        assert_eq!(load_state, FMOD_STUDIO_LOADING_STATE_LOADED);

        // Check Bank list queries.
        let mut ev_count = 0i32;
        assert_eq!(FMOD_Studio_Bank_GetEventCount(sfx_bank, &mut ev_count), FMOD_OK);
        assert_eq!(ev_count, 3);
        let mut ev_list = [0usize; 4];
        let mut got = 0i32;
        assert_eq!(FMOD_Studio_Bank_GetEventList(sfx_bank, ev_list.as_mut_ptr(), 4, &mut got), FMOD_OK);
        assert_eq!(got, 3);

        // Look up events by path (case-insensitive) and by {GUID} string!
        let mut ev_pcm = 0usize;
        let mut ev_fad = 0usize;
        let mut ev_ima = 0usize;
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"event:/sfx/tonepcm16\0".as_ptr(), &mut ev_pcm),
            FMOD_OK
        );
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}\0".as_ptr(), &mut ev_fad),
            FMOD_OK
        );
        assert_eq!(
            FMOD_Studio_System_GetEventByID(studio, &ev_ima3d_guid, &mut ev_ima),
            FMOD_OK
        );

        // Verify EventDescription metadata & path queries (including FMOD_ERR_TRUNCATED!).
        let mut path_buf = [0u8; 64];
        let mut retrieved = 0i32;
        assert_eq!(
            FMOD_Studio_EventDescription_GetPath(ev_pcm, path_buf.as_mut_ptr(), 8, &mut retrieved),
            FMOD_ERR_TRUNCATED
        );
        assert_eq!(retrieved as usize, "event:/SFX/TonePCM16".len() + 1);
        assert_eq!(
            FMOD_Studio_EventDescription_GetPath(ev_pcm, path_buf.as_mut_ptr(), 64, &mut retrieved),
            FMOD_OK
        );
        assert_eq!(crate::strings::from_ansi(path_buf.as_ptr()), "event:/SFX/TonePCM16");

        let (mut len_ms, mut is_oneshot, mut is_3d) = (0i32, 0i32, 0i32);
        assert_eq!(FMOD_Studio_EventDescription_GetLength(ev_pcm, &mut len_ms), FMOD_OK);
        assert_eq!(len_ms, 10); // 480 frames @ 48 kHz = 10 ms
        assert_eq!(FMOD_Studio_EventDescription_IsOneshot(ev_pcm, &mut is_oneshot), FMOD_OK);
        assert_eq!(is_oneshot, 1);
        assert_eq!(FMOD_Studio_EventDescription_Is3D(ev_ima, &mut is_3d), FMOD_OK);
        assert_eq!(is_3d, 1);

        // Look up Buses and VCA.
        let (mut bus_master, mut bus_sfx, mut bus_weapons, mut vca_sfx) = (0usize, 0usize, 0usize, 0usize);
        assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/\0".as_ptr(), &mut bus_master), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/SFX\0".as_ptr(), &mut bus_sfx), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/SFX/Weapons\0".as_ptr(), &mut bus_weapons), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetVCA(studio, b"vca:/MasterSFX\0".as_ptr(), &mut vca_sfx), FMOD_OK);

        // --- A) PCM16 event + Bus hierarchy + VCA scaling + timeline + playback state ---
        let mut inst_pcm = 0usize;
        assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_pcm, &mut inst_pcm), FMOD_OK);
        let mut state = 99u32;
        assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
        assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);

        assert_eq!(FMOD_Studio_EventInstance_Start(inst_pcm), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
        assert_eq!(state, FMOD_STUDIO_PLAYBACK_PLAYING);

        // Mix 48 frames (1 ms) at unity gain: L = 0.8, R = 0.4
        let mut out = [0.0f32; 48 * 2];
        assert_eq!(mix_system_frames(core, &mut out), Ok(48));
        assert!((out[0] - 0.8).abs() < 1e-3, "PCM16 L got {}", out[0]);
        assert!((out[1] - 0.4).abs() < 1e-3, "PCM16 R got {}", out[1]);

        let mut tpos = -1i32;
        assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut tpos), FMOD_OK);
        assert_eq!(tpos, 1);

        // Scale volume: inst=0.5 * bus_weapons=0.5 * bus_sfx=0.5 * vca_sfx=0.5 = 0.0625
        // Expected L = 0.8 * 0.0625 = 0.05, R = 0.4 * 0.0625 = 0.025
        assert_eq!(call_studio_inst_set_volume(inst_pcm, 0.5), FMOD_OK);
        assert_eq!(call_studio_bus_set_volume(bus_weapons, 0.5), FMOD_OK);
        assert_eq!(call_studio_bus_set_volume(bus_sfx, 0.5), FMOD_OK);
        assert_eq!(call_studio_vca_set_volume(vca_sfx, 0.5), FMOD_OK);

        let (mut bvol, mut bfinal) = (0.0f32, 0.0f32);
        assert_eq!(FMOD_Studio_Bus_GetVolume(bus_weapons, &mut bvol, &mut bfinal), FMOD_OK);
        assert!((bvol - 0.5).abs() < 1e-5);
        assert!((bfinal - 0.125).abs() < 1e-5, "bus_weapons final={bfinal}");

        assert_eq!(mix_system_frames(core, &mut out), Ok(48));
        assert!((out[0] - 0.05).abs() < 1e-3, "scaled L got {}", out[0]);
        assert!((out[1] - 0.025).abs() < 1e-3, "scaled R got {}", out[1]);

        // Mute bus_sfx -> silence while timeline still advances.
        assert_eq!(FMOD_Studio_Bus_SetMute(bus_sfx, 1), FMOD_OK);
        assert_eq!(mix_system_frames(core, &mut out), Ok(48));
        assert!(out.iter().all(|&v| v == 0.0));
        assert_eq!(FMOD_Studio_Bus_SetMute(bus_sfx, 0), FMOD_OK);

        // Pause instance -> silence AND timeline stays frozen.
        let mut pos_before_pause = 0i32;
        assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut pos_before_pause), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_SetPaused(inst_pcm, 1), FMOD_OK);
        assert_eq!(mix_system_frames(core, &mut out), Ok(48));
        assert!(out.iter().all(|&v| v == 0.0));
        let mut pos_during_pause = 0i32;
        assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut pos_during_pause), FMOD_OK);
        assert_eq!(pos_during_pause, pos_before_pause);
        assert_eq!(FMOD_Studio_EventInstance_SetPaused(inst_pcm, 0), FMOD_OK);

        // Seek to 9 ms (432 frames) and mix 64 frames -> one-shot finishes cleanly!
        assert_eq!(FMOD_Studio_EventInstance_SetTimelinePosition(inst_pcm, 9), FMOD_OK);
        let mut tail = [0.0f32; 64 * 2];
        assert_eq!(mix_system_frames(core, &mut tail), Ok(64));
        assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
        assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);

        // Local & Global Parameters (by name and by ID, with clamping + error on unknown).
        let (mut pv, mut pfv) = (0.0f32, 0.0f32);
        assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_pcm, b"Intensity\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
        assert!((pv - 2.5).abs() < 1e-5);
        assert_eq!(call_studio_inst_set_param_name(inst_pcm, b"Intensity\0".as_ptr(), 99.0, 0), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_GetParameterByID(inst_pcm, intensity_id.to_u64(), &mut pv, &mut pfv), FMOD_OK);
        assert!((pv - 10.0).abs() < 1e-5, "expected clamped max 10.0, got {pv}");
        assert_eq!(call_studio_inst_set_param_id(inst_pcm, intensity_id, 4.25, 1), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_pcm, b"Intensity\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
        assert!((pv - 4.25).abs() < 1e-5);
        assert_eq!(
            call_studio_inst_set_param_name(inst_pcm, b"NoSuchParam\0".as_ptr(), 1.0, 0),
            FMOD_ERR_EVENT_NOTFOUND
        );
        assert_eq!(call_studio_sys_set_param_name(studio, b"TimeOfDay\0".as_ptr(), 18.5, 0), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetParameterByName(studio, b"TimeOfDay\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
        assert!((pv - 18.5).abs() < 1e-5);

        assert_eq!(FMOD_Studio_EventInstance_Release(inst_pcm), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_pcm), 0);

        // Reset bus & VCA volumes to unity.
        assert_eq!(call_studio_bus_set_volume(bus_weapons, 1.0), FMOD_OK);
        assert_eq!(call_studio_bus_set_volume(bus_sfx, 1.0), FMOD_OK);
        assert_eq!(call_studio_vca_set_volume(vca_sfx, 1.0), FMOD_OK);

        // --- B) FADPCM looping event + pitch + StopAllEvents + fire-and-forget Start+Release ---
        let mut inst_fad = 0usize;
        assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_fad, &mut inst_fad), FMOD_OK);
        assert_eq!(call_studio_inst_set_pitch(inst_fad, 2.0), FMOD_OK);
        let mut pitch_out = 0.0f32;
        assert_eq!(FMOD_Studio_EventInstance_GetPitch(inst_fad, &mut pitch_out, std::ptr::null_mut()), FMOD_OK);
        assert!((pitch_out - 2.0).abs() < 1e-5);
        assert_eq!(FMOD_Studio_EventInstance_Start(inst_fad), FMOD_OK);

        let mut out_fad = [0.0f32; 16 * 2];
        assert_eq!(mix_system_frames(core, &mut out_fad), Ok(16));
        assert!((out_fad[0] - 0.5).abs() < 1e-3, "Studio FADPCM L got {}", out_fad[0]);
        assert!((out_fad[1] - 0.5).abs() < 1e-3, "Studio FADPCM R got {}", out_fad[1]);

        assert_eq!(FMOD_Studio_Bus_StopAllEvents(bus_sfx, FMOD_STUDIO_STOP_IMMEDIATE), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_fad, &mut state), FMOD_OK);
        assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);
        assert_eq!(mix_system_frames(core, &mut out_fad), Ok(16));
        assert!(out_fad.iter().all(|&v| v == 0.0));
        assert_eq!(FMOD_Studio_EventInstance_Release(inst_fad), FMOD_OK);

        // Fire-and-forget one-shot (`Start` then immediate `Release`) keeps mixing until done!
        let mut inst_ff = 0usize;
        assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_pcm, &mut inst_ff), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_Start(inst_ff), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_Release(inst_ff), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_ff), 0);
        let mut ff_mix = [0.0f32; 16 * 2];
        assert_eq!(mix_system_frames(core, &mut ff_mix), Ok(16));
        assert!((ff_mix[0] - 0.8).abs() < 1e-3, "fire-and-forget L got {}", ff_mix[0]);
        assert_eq!(FMOD_Studio_Bus_StopAllEvents(bus_master, FMOD_STUDIO_STOP_IMMEDIATE), FMOD_OK);
        assert_eq!(FMOD_Studio_System_Update(studio), FMOD_OK);

        // --- C) IMA-ADPCM 3D event + Studio 3D listener + automatic Distance param ---
        let listener_attrs = Fmod3DAttributes {
            position: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            velocity: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            forward: FmodVector { x: 0.0, y: 0.0, z: 1.0 },
            up: FmodVector { x: 0.0, y: 1.0, z: 0.0 },
        };
        assert_eq!(
            FMOD_Studio_System_SetListenerAttributes(studio, 0, &listener_attrs, std::ptr::null()),
            FMOD_OK
        );
        assert_eq!(call_studio_sys_set_listener_weight(studio, 0, 1.0), FMOD_OK);

        let mut inst_3d = 0usize;
        assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_ima, &mut inst_3d), FMOD_OK);
        // Position 2m to listener's right (+X): atten = 1/2 = 0.5, pan = +1.0 -> L = 0.0, R = 0.8 * 0.5 = 0.4
        let src_attrs = Fmod3DAttributes {
            position: FmodVector { x: 2.0, y: 0.0, z: 0.0 },
            ..Default::default()
        };
        assert_eq!(FMOD_Studio_EventInstance_Set3DAttributes(inst_3d, &src_attrs), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_Start(inst_3d), FMOD_OK);

        // Automatic Distance parameter reflects 3D distance = 2.0!
        assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_3d, b"Distance\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
        assert!((pfv - 2.0).abs() < 1e-4, "expected auto Distance=2.0, got {pfv}");

        let mut out_3d = [0.0f32; 8 * 2];
        assert_eq!(mix_system_frames(core, &mut out_3d), Ok(8));
        assert!(out_3d[0].abs() < 1e-3, "3D IMA L expected 0, got {}", out_3d[0]);
        assert!((out_3d[1] - 0.4).abs() < 1e-3, "3D IMA R expected 0.4, got {}", out_3d[1]);

        // --- D) Bank Unload stops active audio and invalidates bank/event/instance/bus/VCA ---
        assert_eq!(FMOD_Studio_Bank_Unload(sfx_bank), FMOD_OK);
        assert_eq!(FMOD_Studio_Bank_IsValid(sfx_bank), 0);
        assert_eq!(FMOD_Studio_EventDescription_IsValid(ev_ima), 0);
        assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_3d), 0);
        assert_eq!(FMOD_Studio_Bus_IsValid(bus_sfx), 0);
        assert_eq!(FMOD_Studio_VCA_IsValid(vca_sfx), 0);
        // Master bus stays valid!
        assert_eq!(FMOD_Studio_Bus_IsValid(bus_master), 1);

        // Audio is immediately silent after bank unload.
        assert_eq!(mix_system_frames(core, &mut out_3d), Ok(8));
        assert!(out_3d.iter().all(|&v| v == 0.0));

        let mut ev_after = 0usize;
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"event:/SFX/SpatialIMA\0".as_ptr(), &mut ev_after),
            FMOD_ERR_EVENT_NOTFOUND
        );

        let _ = std::fs::remove_file(&strings_file);
        let _ = std::fs::remove_dir(&tmp_dir);
        assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
    }
}

#[test]
fn fmod_studio_standalone_fsb5_bank_and_parse_id() {
    unsafe {
        let mut parsed_guid = FmodGuid::ZERO;
        assert_eq!(
            FMOD_Studio_ParseID(
                b"{01234567-89ab-cdef-0123-456789abcdef}\0".as_ptr(),
                &mut parsed_guid,
            ),
            FMOD_OK
        );
        assert_eq!(parsed_guid.data1, 0x0123_4567);
        assert_eq!(parsed_guid.data2, 0x89ab);
        assert_eq!(parsed_guid.data3, 0xcdef);
        assert_eq!(parsed_guid.data4, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
        assert_eq!(
            FMOD_Studio_ParseID(b"not-a-guid\0".as_ptr(), &mut parsed_guid),
            FMOD_ERR_FORMAT
        );

        let mut studio = 0usize;
        let mut core = 0usize;
        assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
        assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core), FMOD_OK);
        assert_eq!(FMOD_System_SetOutput(core, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
        assert_eq!(FMOD_Studio_System_Initialize(studio, 16, 0, 0, 0), FMOD_OK);

        // Load a standalone FSB5 bank directly into FMOD Studio.
        let standalone_fsb = build_fsb5(
            1,
            FSB5_MODE_FADPCM,
            &[TestFsbSample {
                name: "UI/Click".into(),
                channels: 1,
                sample_rate: 48_000,
                num_frames: 64,
                loop_points: None,
                data: build_fadpcm_constant_frame(&[2]), // +2 * 4096 = +8192 = +0.25
            }],
        );
        let mut bank = 0usize;
        assert_eq!(
            FMOD_Studio_System_LoadBankMemory(
                studio,
                standalone_fsb.as_ptr(),
                standalone_fsb.len() as i32,
                FMOD_STUDIO_LOAD_MEMORY,
                FMOD_STUDIO_LOAD_BANK_NORMAL,
                &mut bank,
            ),
            FMOD_OK
        );

        let mut ev = 0usize;
        assert_eq!(
            FMOD_Studio_System_GetEvent(studio, b"event:/UI/Click\0".as_ptr(), &mut ev),
            FMOD_OK
        );
        let mut inst = 0usize;
        assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev, &mut inst), FMOD_OK);
        assert_eq!(FMOD_Studio_EventInstance_Start(inst), FMOD_OK);

        let mut mix = [0.0f32; 8 * 2];
        assert_eq!(mix_system_frames(core, &mut mix), Ok(8));
        for f in 0..8 {
            assert!((mix[f * 2] - 0.25).abs() < 1e-3, "L[{f}]={}", mix[f * 2]);
            assert!((mix[f * 2 + 1] - 0.25).abs() < 1e-3, "R[{f}]={}", mix[f * 2 + 1]);
        }

        assert_eq!(FMOD_Studio_System_UnloadAll(studio), FMOD_OK);
        assert_eq!(FMOD_Studio_Bank_IsValid(bank), 0);
        assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
    }
}

#[test]
fn fmod_get_version_respects_middleware_config() {
    struct ResetFmod;
    impl Drop for ResetFmod {
        fn drop(&mut self) {
            crate::middleware::clear_family_version("fmod");
        }
    }
    let _reset = ResetFmod;

    unsafe {
        let mut sys = 0usize;
        assert_eq!(FMOD_System_Create(&mut sys, 0), FMOD_OK);
        assert_ne!(sys, 0);

        let mut ver = 0u32;
        crate::middleware::clear_family_version("fmod");
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, FMOD_VERSION);

        crate::middleware::set_family_version("fmod", "1.10");
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, 0x0001_1000);

        crate::middleware::set_family_version("fmod", "2.00");
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, 0x0002_0000);

        crate::middleware::set_family_version("fmod", "2.02");
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, 0x0002_0200);

        crate::middleware::clear_family_version("fmod");
        assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
        assert_eq!(ver, FMOD_VERSION);

        assert_eq!(FMOD_System_Release(sys), FMOD_OK);
    }
}
