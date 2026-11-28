use super::mixer::*;
use super::voices::*;
use super::x3daudio::*;
use super::xapo::*;
use super::*;
use crate::audio::backend::{self, Backend};
use crate::audio::com::{GUID, IID_IUnknown, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT};
use crate::audio::format::{
    WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_PCM,
};
use std::f32::consts::{FRAC_1_SQRT_2, TAU};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn pcm_wfx(channels: u16, sample_rate: u32, bits: u16) -> WAVEFORMATEX {
    let block_align = channels * (bits / 8);
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_PCM,
        nChannels: channels,
        nSamplesPerSec: sample_rate,
        nAvgBytesPerSec: sample_rate * block_align as u32,
        nBlockAlign: block_align,
        wBitsPerSample: bits,
        cbSize: 0,
    }
}

fn float_wfx(channels: u16, sample_rate: u32) -> WAVEFORMATEX {
    let block_align = channels * 4;
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_IEEE_FLOAT,
        nChannels: channels,
        nSamplesPerSec: sample_rate,
        nAvgBytesPerSec: sample_rate * block_align as u32,
        nBlockAlign: block_align,
        wBitsPerSample: 32,
        cbSize: 0,
    }
}

fn make_stopped_test_engine(
    master_ch: u32,
    master_rate: u32,
) -> (*mut XAudio2Engine, *mut MasteringVoice) {
    unsafe {
        let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
        assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
        // Stop before creating mastering voice so the background mixer thread stays idle
        // while unit tests pull deterministic quantums via `mix_quantum`.
        xa2_stop_engine(xa2);
        let mut mv: *mut MasteringVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_mastering_voice(
                xa2,
                &mut mv,
                master_ch,
                master_rate,
                0,
                std::ptr::null(),
                std::ptr::null(),
                6
            ),
            S_OK
        );
        (xa2, mv)
    }
}

#[test]
fn mastering_voice_resolves_wasapi_and_swd_device_id() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));
    let default_dev = backend::null_device();
    let ds_dev = backend::dualsense_usb_device(0);
    let ds_uid = ds_dev.uid.clone();
    let ds_ep_id = crate::audio::mmdevapi::render_endpoint_id_for_uid(&ds_uid);
    backend::force_output_devices(Some(vec![default_dev.clone(), ds_dev]));

    struct RestoreDevices;
    impl Drop for RestoreDevices {
        fn drop(&mut self) {
            backend::force_output_devices(None);
        }
    }
    let _restore = RestoreDevices;

    unsafe {
        // 1. Direct WASAPI endpoint ID (`{0.0.0.00000000}.{guid}`) selects the DualSense USB
        //    4-channel endpoint and defaults input_channels == 0 to 4 channels.
        let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
        assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
        xa2_stop_engine(xa2);
        let ds_id_w = crate::strings::to_wide_z(&ds_ep_id);
        let mut mv: *mut MasteringVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_mastering_voice(
                xa2,
                &mut mv,
                0,
                0,
                0,
                ds_id_w.as_ptr(),
                std::ptr::null(),
                6
            ),
            S_OK
        );
        assert_eq!(
            (&(*xa2).core).runtime.lock().unwrap().as_ref().unwrap().device_uid,
            ds_uid
        );
        let mut details = XAudio2VoiceDetails {
            creation_flags: 0,
            active_flags: 0,
            input_channels: 0,
            input_sample_rate: 0,
        };
        voice_get_voice_details(mv as *mut VoiceHeader, &mut details);
        assert_eq!({ details.input_channels }, 4);
        assert_eq!({ details.input_sample_rate }, 48_000);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);

        // 2. WinRT/PnP SWD device interface path (`\\?\SWD#MMDEVAPI#{...}#{guid}`) also matches.
        let swd_path = format!(
            r"\\?\SWD#MMDEVAPI#{}#{{e6327cad-d664-4845-a290-00e5a8880222}}",
            ds_ep_id.to_ascii_uppercase()
        );
        assert_eq!(choose_output_device(Some(&swd_path)).uid, ds_uid);
        assert!(!choose_output_device(Some(&swd_path)).is_default);

        // 3. Unknown szDeviceId falls back cleanly to the default output device.
        assert_eq!(choose_output_device(Some("{0.0.0.00000000}.{deadbeef}")).uid, default_dev.uid);
        assert_eq!(choose_output_device(None).uid, default_dev.uid);
    }
}

#[test]
fn xaudio2_bink_lifecycle() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
        assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
        assert!(!xa2.is_null());

        // QueryInterface for IXAudio2 and unknown IID.
        let mut qi_out: *mut c_void = std::ptr::null_mut();
        assert_eq!(xa2_query_interface(xa2, &IID_IXAUDIO2, &mut qi_out), S_OK);
        assert_eq!(qi_out, xa2 as *mut c_void);
        assert_eq!(xa2_release(xa2), 1);

        let unknown = GUID::new(0x1111_2222, 0x3333, 0x4444, [5; 8]);
        assert_eq!(
            xa2_query_interface(xa2, &unknown, &mut qi_out),
            E_NOINTERFACE
        );
        assert!(qi_out.is_null());

        xa2_stop_engine(xa2);
        let mut mv: *mut MasteringVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_mastering_voice(
                xa2,
                &mut mv,
                0,
                0,
                0,
                std::ptr::null(),
                std::ptr::null(),
                6
            ),
            S_OK
        );
        assert!(!mv.is_null());

        let mut details = XAudio2VoiceDetails {
            creation_flags: 0,
            active_flags: 0,
            input_channels: 0,
            input_sample_rate: 0,
        };
        voice_get_voice_details(mv as *mut VoiceHeader, &mut details);
        assert_eq!({ details.input_channels }, 2);
        assert_eq!({ details.input_sample_rate }, 48_000);

        let mut mask = 0u32;
        assert_eq!(mv_get_channel_mask(mv, &mut mask), S_OK);
        assert_eq!(mask, 0x3);

        let wfx = pcm_wfx(2, 48_000, 16);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                XAUDIO2_VOICE_NOPITCH,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );
        assert!(!sv.is_null());

        let pcm = vec![0u8; 3000];
        let buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: pcm.len() as u32,
            audio_data: pcm.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0x1234,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut st = XAudio2VoiceState {
            current_buffer_context: 0,
            buffers_queued: 16,
            samples_played: 0,
        };
        sv_get_state(sv, &mut st, 0);
        assert_eq!({ st.buffers_queued }, 1);
        assert_eq!({ st.current_buffer_context }, 0x1234);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn pcm_formats_and_additive_mixing() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);

        // Voice 1: 16-bit signed PCM stereo (L=+0.5, R=-0.5).
        let wfx16 = pcm_wfx(2, 48_000, 16);
        let mut sv16: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv16,
                &wfx16,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );
        let pcm16: Vec<u8> = [16384i16, -16384i16, 16384i16, -16384i16]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let buf16 = XAudio2Buffer {
            flags: 0,
            audio_bytes: pcm16.len() as u32,
            audio_data: pcm16.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 1,
        };
        assert_eq!(sv_submit_source_buffer(sv16, &buf16, 0), S_OK);
        assert_eq!(sv_start(sv16, 0, 0), S_OK);

        // Voice 2: WAVE_FORMAT_EXTENSIBLE 32-bit IEEE float stereo (L=+0.25, R=+0.75).
        let ext_wfx = WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE,
                nChannels: 2,
                nSamplesPerSec: 48_000,
                nAvgBytesPerSec: 48_000 * 8,
                nBlockAlign: 8,
                wBitsPerSample: 32,
                cbSize: 22,
            },
            wValidBitsPerSample: 32,
            dwChannelMask: 0x3,
            SubFormat: KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
        };
        let mut svf: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut svf,
                &ext_wfx as *const WAVEFORMATEXTENSIBLE as *const WAVEFORMATEX,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );
        let fdata: Vec<u8> = [0.25f32, 0.75f32, 0.25f32, 0.75f32]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let buff = XAudio2Buffer {
            flags: 0,
            audio_bytes: fdata.len() as u32,
            audio_data: fdata.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 2,
        };
        assert_eq!(sv_submit_source_buffer(svf, &buff, 0), S_OK);
        assert_eq!(sv_start(svf, 0, 0), S_OK);

        let mut out = [0.0f32; 8];
        let _ = (*xa2).core.mix_quantum(4, &mut out);
        // Frames 0..2: L = 0.5 + 0.25 = 0.75, R = -0.5 + 0.75 = 0.25; frames 2..4 silent.
        assert!((out[0] - 0.75).abs() < 1e-5);
        assert!((out[1] - 0.25).abs() < 1e-5);
        assert!((out[2] - 0.75).abs() < 1e-5);
        assert!((out[3] - 0.25).abs() < 1e-5);
        assert_eq!(out[4], 0.0);
        assert_eq!(out[5], 0.0);

        // Also verify 8-bit, 24-bit, and 32-bit PCM decoding directly.
        let fmt8 = parse_wave_format(&pcm_wfx(1, 48_000, 8)).unwrap();
        let (d8, n8) = decode_pcm_to_f32(&fmt8, &[0, 128, 192]);
        assert_eq!(n8, 3);
        assert!((d8[0] - (-1.0)).abs() < 1e-5);
        assert!((d8[1] - 0.0).abs() < 1e-5);
        assert!((d8[2] - 0.5).abs() < 1e-5);

        let fmt24 = parse_wave_format(&pcm_wfx(1, 48_000, 24)).unwrap();
        let (d24, n24) = decode_pcm_to_f32(&fmt24, &[0x00, 0x00, 0x80, 0x00, 0x00, 0x40]);
        assert_eq!(n24, 2);
        assert!((d24[0] - (-1.0)).abs() < 1e-5);
        assert!((d24[1] - 0.5).abs() < 1e-5);

        let fmt32 = parse_wave_format(&pcm_wfx(1, 48_000, 32)).unwrap();
        let raw32: Vec<u8> = [i32::MIN, 1 << 30]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let (d32, n32) = decode_pcm_to_f32(&fmt32, &raw32);
        assert_eq!(n32, 2);
        assert!((d32[0] - (-1.0)).abs() < 1e-5);
        assert!((d32[1] - 0.5).abs() < 1e-5);

        voice_destroy_voice(sv16 as *mut VoiceHeader);
        voice_destroy_voice(svf as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn resampling_and_frequency_ratio() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        // 24 kHz mono source -> 2x upsampling to 48 kHz master.
        let wfx = float_wfx(1, 24_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                4.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        let src_samples = [0.0f32, 1.0, 2.0, 3.0];
        let raw: Vec<u8> = src_samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        let buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = [0.0f32; 8];
        let _ = (*xa2).core.mix_quantum(8, &mut out);
        assert!((out[0] - 0.0).abs() < 1e-5);
        assert!((out[1] - 0.5).abs() < 1e-5);
        assert!((out[2] - 1.0).abs() < 1e-5);
        assert!((out[3] - 1.5).abs() < 1e-5);
        assert!((out[4] - 2.0).abs() < 1e-5);
        assert!((out[5] - 2.5).abs() < 1e-5);
        assert!((out[6] - 3.0).abs() < 1e-5);

        // Change source sample rate to 48 kHz and set frequency ratio = 2.0 -> skips every other sample.
        assert_eq!(sv_set_source_sample_rate(sv, 48_000), S_OK);
        assert_eq!(sv_set_freq_ratio(sv, 2.0, 0), S_OK);
        let mut got_ratio = 0.0f32;
        sv_get_freq_ratio(sv, &mut got_ratio);
        assert_eq!(got_ratio, 2.0);

        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        let mut out2 = [0.0f32; 4];
        let _ = (*xa2).core.mix_quantum(4, &mut out2);
        assert!((out2[0] - 0.0).abs() < 1e-5);
        assert!((out2[1] - 2.0).abs() < 1e-5);
        assert_eq!(out2[2], 0.0);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn play_region_looping_and_exit_loop() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        let data: Vec<f32> = (0..10).map(|i| i as f32).collect();
        let raw: Vec<u8> = data.iter().flat_map(|v| v.to_le_bytes()).collect();
        let buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 1,
            play_length: 8, // 1..9
            loop_begin: 2,
            loop_length: 4, // 2..6
            loop_count: 2,  // plays 2..6 three times total
            context: 0xAA,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = [0.0f32; 20];
        let _ = (*xa2).core.mix_quantum(20, &mut out);
        let expected = [
            1.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        for (i, (&got, &exp)) in out.iter().zip(expected.iter()).enumerate() {
            assert!(
                (got - exp).abs() < 1e-5,
                "frame {i}: got {got} expected {exp}"
            );
        }

        let mut st = XAudio2VoiceState {
            current_buffer_context: 1,
            buffers_queued: 9,
            samples_played: 0,
        };
        sv_get_state(sv, &mut st, 0);
        assert_eq!({ st.buffers_queued }, 0);
        assert_eq!({ st.samples_played }, 16);

        // Infinite loop + ExitLoop.
        let inf_buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 0,
            play_length: 4,
            loop_begin: 0,
            loop_length: 4,
            loop_count: XAUDIO2_LOOP_INFINITE,
            context: 0xBB,
        };
        assert_eq!(sv_submit_source_buffer(sv, &inf_buf, 0), S_OK);
        let mut loop_out = [0.0f32; 16];
        let _ = (*xa2).core.mix_quantum(16, &mut loop_out);
        sv_get_state(sv, &mut st, 0);
        assert_eq!({ st.buffers_queued }, 1);
        assert_eq!({ st.current_buffer_context }, 0xBB);

        assert_eq!(sv_exit_loop(sv, 0), S_OK);
        let mut tail_out = [0.0f32; 8];
        let _ = (*xa2).core.mix_quantum(8, &mut tail_out);
        sv_get_state(sv, &mut st, 0);
        assert_eq!({ st.buffers_queued }, 0);

        // FlushSourceBuffers when stopped empties queue.
        assert_eq!(sv_stop(sv, 0, 0), S_OK);
        assert_eq!(sv_submit_source_buffer(sv, &inf_buf, 0), S_OK);
        assert_eq!(sv_flush_source_buffers(sv), S_OK);
        sv_get_state(sv, &mut st, 0);
        assert_eq!({ st.buffers_queued }, 0);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn volume_matrix_panning_and_filter() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);
        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        assert_eq!(voice_set_volume(sv as *mut VoiceHeader, 0.5, 0), S_OK);
        let mut vol = 0.0f32;
        voice_get_volume(sv as *mut VoiceHeader, &mut vol);
        assert_eq!(vol, 0.5);

        let ch_vols = [0.5f32];
        assert_eq!(
            voice_set_channel_volumes(sv as *mut VoiceHeader, 1, ch_vols.as_ptr(), 0),
            S_OK
        );
        let mut got_ch_vols = [0.0f32; 1];
        voice_get_channel_volumes(sv as *mut VoiceHeader, 1, got_ch_vols.as_mut_ptr());
        assert_eq!(got_ch_vols, ch_vols);
        assert_eq!(
            voice_set_channel_volumes(sv as *mut VoiceHeader, 1, [1.0f32].as_ptr(), 0),
            S_OK
        );

        // Pan mono -> stereo: Left=0.25, Right=0.75 (XAudio2 layout: [D=0,S=0]=0.25, [D=1,S=0]=0.75).
        let mat = [0.25f32, 0.75f32];
        assert_eq!(
            voice_set_output_matrix(
                sv as *mut VoiceHeader,
                std::ptr::null_mut(),
                1,
                2,
                mat.as_ptr(),
                0
            ),
            S_OK
        );
        let mut got_mat = [0.0f32; 2];
        voice_get_output_matrix(
            sv as *mut VoiceHeader,
            std::ptr::null_mut(),
            1,
            2,
            got_mat.as_mut_ptr(),
        );
        assert_eq!(got_mat, mat);

        let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        let buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: ones.len() as u32,
            audio_data: ones.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = [0.0f32; 8];
        let _ = (*xa2).core.mix_quantum(4, &mut out);
        assert!((out[0] - 0.125).abs() < 1e-5); // 1.0 * 0.5 * 0.25
        assert!((out[1] - 0.375).abs() < 1e-5); // 1.0 * 0.5 * 0.75

        // Low-pass filter should attenuate Nyquist (+1/-1) far more than DC (+1).
        let lpf = XAudio2FilterParameters {
            filter_type: FILTER_LOW_PASS,
            frequency: 0.1,
            one_over_q: 1.0,
        };
        assert_eq!(
            voice_set_filter_parameters(sv as *mut VoiceHeader, &lpf, 0),
            S_OK
        );
        let mut got_lpf = XAudio2FilterParameters::default();
        voice_get_filter_parameters(sv as *mut VoiceHeader, &mut got_lpf);
        assert_eq!({ got_lpf.filter_type }, FILTER_LOW_PASS);
        assert!((got_lpf.frequency - 0.1).abs() < 1e-6);

        let mut alt = vec![0.0f32; 128];
        for (i, v) in alt.iter_mut().enumerate() {
            *v = if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        let alt_raw: Vec<u8> = alt.iter().flat_map(|v| v.to_le_bytes()).collect();
        let alt_buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: alt_raw.len() as u32,
            audio_data: alt_raw.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0,
        };
        assert_eq!(voice_set_volume(sv as *mut VoiceHeader, 1.0, 0), S_OK);
        assert_eq!(
            voice_set_output_matrix(
                sv as *mut VoiceHeader,
                std::ptr::null_mut(),
                1,
                2,
                [1.0f32, 1.0f32].as_ptr(),
                0
            ),
            S_OK
        );
        assert_eq!(sv_submit_source_buffer(sv, &alt_buf, 0), S_OK);
        let mut nyq_out = vec![0.0f32; 256];
        let _ = (*xa2).core.mix_quantum(128, &mut nyq_out);
        let steady_peak = nyq_out[128..]
            .iter()
            .map(|v| v.abs())
            .fold(0.0f32, f32::max);
        assert!(
            steady_peak < 0.05,
            "LPF failed to attenuate Nyquist: peak={steady_peak}"
        );

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn submix_voice_multi_stage_routing() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);

        // Stage 1 submix -> Mastering voice.
        let mut sub1: *mut SubmixVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_submix_voice(
                xa2,
                &mut sub1,
                2,
                48_000,
                0,
                1,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        // Stage 0 submix -> Stage 1 submix.
        let sub0_send = XAudio2SendDescriptor {
            flags: 0,
            output_voice: sub1 as *mut VoiceHeader,
        };
        let sub0_sends = XAudio2VoiceSends {
            send_count: 1,
            sends: &sub0_send,
        };
        let mut sub0: *mut SubmixVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_submix_voice(
                xa2,
                &mut sub0,
                2,
                48_000,
                0,
                0,
                &sub0_sends,
                std::ptr::null()
            ),
            S_OK
        );

        // Source -> Stage 0 submix.
        let src_send = XAudio2SendDescriptor {
            flags: 0,
            output_voice: sub0 as *mut VoiceHeader,
        };
        let src_sends = XAudio2VoiceSends {
            send_count: 1,
            sends: &src_send,
        };
        let wfx = float_wfx(2, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                &src_sends,
                std::ptr::null()
            ),
            S_OK
        );

        assert_eq!(voice_set_volume(sub0 as *mut VoiceHeader, 0.5, 0), S_OK);
        assert_eq!(voice_set_volume(sub1 as *mut VoiceHeader, 0.5, 0), S_OK);
        assert_eq!(voice_set_volume(mv as *mut VoiceHeader, 0.8, 0), S_OK);

        let raw: Vec<u8> = [1.0f32, -1.0, 1.0, -1.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let buf = XAudio2Buffer {
            flags: 0,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = [0.0f32; 4];
        let _ = (*xa2).core.mix_quantum(2, &mut out);
        // 1.0 * 0.5 * 0.5 * 0.8 = 0.2
        assert!((out[0] - 0.2).abs() < 1e-5);
        assert!((out[1] - (-0.2)).abs() < 1e-5);

        // Route source to empty send list -> silent.
        let empty_sends = XAudio2VoiceSends {
            send_count: 0,
            sends: std::ptr::null(),
        };
        assert_eq!(
            voice_set_output_voices(sv as *mut VoiceHeader, &empty_sends),
            S_OK
        );
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        let mut silent = [9.0f32; 4];
        let _ = (*xa2).core.mix_quantum(2, &mut silent);
        assert_eq!(silent, [0.0; 4]);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(sub0 as *mut VoiceHeader);
        voice_destroy_voice(sub1 as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn voice_callbacks_delivered_safely() {
    #[repr(C)]
    #[allow(dead_code)]
    struct MockVoiceCbVtbl {
        on_pass_start: unsafe extern "win64" fn(*mut MockVoiceCb, u32),
        on_pass_end: unsafe extern "win64" fn(*mut MockVoiceCb),
        on_stream_end: unsafe extern "win64" fn(*mut MockVoiceCb),
        on_buf_start: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
        on_buf_end: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
        on_loop_end: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
        on_error: unsafe extern "win64" fn(*mut MockVoiceCb, usize, i32),
    }

    #[repr(C)]
    #[allow(dead_code)]
    struct MockVoiceCb {
        vtbl: *const MockVoiceCbVtbl,
        starts: AtomicU32,
        ends: AtomicU32,
        loops: AtomicU32,
        streams: AtomicU32,
        last_ctx: AtomicU64,
    }

    unsafe extern "win64" fn on_pass_start(_this: *mut MockVoiceCb, _bytes: u32) {}
    unsafe extern "win64" fn on_pass_end(_this: *mut MockVoiceCb) {}
    unsafe extern "win64" fn on_stream_end(this: *mut MockVoiceCb) {
        (*this).streams.fetch_add(1, Ordering::AcqRel);
    }
    unsafe extern "win64" fn on_buf_start(this: *mut MockVoiceCb, ctx: usize) {
        (*this).starts.fetch_add(1, Ordering::AcqRel);
        (*this).last_ctx.store(ctx as u64, Ordering::Release);
    }
    unsafe extern "win64" fn on_buf_end(this: *mut MockVoiceCb, ctx: usize) {
        (*this).ends.fetch_add(1, Ordering::AcqRel);
        (*this).last_ctx.store(ctx as u64, Ordering::Release);
    }
    unsafe extern "win64" fn on_loop_end(this: *mut MockVoiceCb, ctx: usize) {
        (*this).loops.fetch_add(1, Ordering::AcqRel);
        (*this).last_ctx.store(ctx as u64, Ordering::Release);
    }
    unsafe extern "win64" fn on_error(_this: *mut MockVoiceCb, _ctx: usize, _hr: i32) {}

    static MOCK_VTBL: MockVoiceCbVtbl = MockVoiceCbVtbl {
        on_pass_start,
        on_pass_end,
        on_stream_end,
        on_buf_start,
        on_buf_end,
        on_loop_end,
        on_error,
    };

    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        let cb = Box::new(MockVoiceCb {
            vtbl: &MOCK_VTBL,
            starts: AtomicU32::new(0),
            ends: AtomicU32::new(0),
            loops: AtomicU32::new(0),
            streams: AtomicU32::new(0),
            last_ctx: AtomicU64::new(0),
        });
        let cb_ptr = &*cb as *const MockVoiceCb as usize;

        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                cb_ptr,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        let raw: Vec<u8> = [0.25f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        let buf = XAudio2Buffer {
            flags: XAUDIO2_END_OF_STREAM,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 0,
            play_length: 4,
            loop_begin: 0,
            loop_length: 4,
            loop_count: 1,
            context: 0xCAFE,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = [0.0f32; 12];
        let events = (*xa2).core.mix_quantum(12, &mut out);
        (*xa2).core.dispatch_callbacks(events);
        (*xa2).core.sync_callbacks();

        assert_eq!(cb.starts.load(Ordering::Acquire), 1);
        assert_eq!(cb.loops.load(Ordering::Acquire), 1);
        assert_eq!(cb.ends.load(Ordering::Acquire), 1);
        assert_eq!(cb.streams.load(Ordering::Acquire), 1);
        assert_eq!(cb.last_ctx.load(Ordering::Acquire), 0xCAFE);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn realtime_null_backend_mixer_drains_buffers() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
        assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
        let mut mv: *mut MasteringVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_mastering_voice(
                xa2,
                &mut mv,
                2,
                48_000,
                0,
                std::ptr::null(),
                std::ptr::null(),
                6
            ),
            S_OK
        );
        let wfx = pcm_wfx(2, 48_000, 16);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        // 960 stereo frames = 20 ms at 48 kHz.
        let pcm = vec![0u8; 960 * 4];
        let buf = XAudio2Buffer {
            flags: XAUDIO2_END_OF_STREAM,
            audio_bytes: pcm.len() as u32,
            audio_data: pcm.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0x55,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let deadline = std::time::Instant::now() + Duration::from_millis(500);
        let mut st = XAudio2VoiceState {
            current_buffer_context: 0,
            buffers_queued: 1,
            samples_played: 0,
        };
        while std::time::Instant::now() < deadline {
            sv_get_state(sv, &mut st, 0);
            if st.buffers_queued == 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!({ st.buffers_queued }, 0);
        assert_eq!({ st.samples_played }, 960);

        let mut perf = XAudio2PerformanceData::default();
        xa2_get_perf_data(xa2, &mut perf);
        assert_eq!({ perf.total_source_voice_count }, 1);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn x3daudio_attenuation_panning_doppler_and_surround() {
    unsafe {
        let mut handle = [0u8; 20];
        assert_eq!(
            X3DAudioInitialize(0x3, X3DAUDIO_SPEED_OF_SOUND, &mut handle),
            S_OK
        );

        let listener = X3DAudioListener {
            orient_front: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            orient_top: X3DAudioVector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            position: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            cone: std::ptr::null(),
        };

        let base_emitter = X3DAudioEmitter {
            cone: std::ptr::null(),
            orient_front: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            orient_top: X3DAudioVector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            position: X3DAudioVector {
                x: -10.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            inner_radius: 0.0,
            inner_radius_angle: 0.0,
            channel_count: 1,
            channel_radius: 0.0,
            channel_azimuths: std::ptr::null(),
            volume_curve: std::ptr::null(),
            lfe_curve: std::ptr::null(),
            lpf_direct_curve: std::ptr::null(),
            lpf_reverb_curve: std::ptr::null(),
            reverb_curve: std::ptr::null(),
            curve_distance_scaler: 2.0,
            doppler_scaler: 1.0,
        };

        let mut mat = [0.0f32; 2];
        let mut dsp = X3DAudioDspSettings {
            matrix_coefficients: mat.as_mut_ptr(),
            delay_times: std::ptr::null_mut(),
            src_channel_count: 1,
            dst_channel_count: 2,
            lpf_direct_coefficient: 0.0,
            lpf_reverb_coefficient: 0.0,
            reverb_level: 0.0,
            doppler_factor: 1.0,
            emitter_to_listener_angle: 0.0,
            emitter_to_listener_distance: 0.0,
            emitter_velocity_component: 0.0,
            listener_velocity_component: 0.0,
        };

        let flags = X3DAUDIO_CALCULATE_MATRIX
            | X3DAUDIO_CALCULATE_DOPPLER
            | X3DAUDIO_CALCULATE_LPF_DIRECT
            | X3DAUDIO_CALCULATE_EMITTER_ANGLE;

        let call_x3d = |h: &[u8; 20],
                        l: &X3DAudioListener,
                        e: &X3DAudioEmitter,
                        f: u32,
                        d: &mut X3DAudioDspSettings| {
            X3DAudioCalculate(
                h,
                l as *const X3DAudioListener as *const c_void,
                e as *const X3DAudioEmitter as *const c_void,
                f,
                d as *mut X3DAudioDspSettings as *mut c_void,
            );
        };

        // Hard left at distance 10 with scaler 2 -> attenuation = 2 / 10 = 0.2.
        call_x3d(&handle, &listener, &base_emitter, flags, &mut dsp);
        assert!((dsp.emitter_to_listener_distance - 10.0).abs() < 1e-4);
        assert!((mat[0] - 0.2).abs() < 1e-4, "left={}", mat[0]);
        assert!(mat[1].abs() < 1e-4, "right={}", mat[1]);
        assert!((dsp.lpf_direct_coefficient - 0.75).abs() < 1e-4);

        // Hard right at distance 10.
        let right_emitter = X3DAudioEmitter {
            position: X3DAudioVector {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            ..base_emitter
        };
        call_x3d(&handle, &listener, &right_emitter, flags, &mut dsp);
        assert!(mat[0].abs() < 1e-4, "left={}", mat[0]);
        assert!((mat[1] - 0.2).abs() < 1e-4, "right={}", mat[1]);

        // Center front at distance 2 (within CurveDistanceScaler) + approaching Doppler.
        let approach_emitter = X3DAudioEmitter {
            position: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            velocity: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: -34.35,
            },
            ..base_emitter
        };
        call_x3d(&handle, &listener, &approach_emitter, flags, &mut dsp);
        assert!((mat[0] - FRAC_1_SQRT_2).abs() < 1e-4);
        assert!((mat[1] - FRAC_1_SQRT_2).abs() < 1e-4);
        assert!((dsp.doppler_factor - (343.5 / (343.5 - 34.35))).abs() < 1e-3);

        // Custom volume curve: (0->1.0, 5->0.5, 10->0.0), evaluated at scaled_dist = 2.5 -> 0.75.
        let pts = [
            X3DAudioDistanceCurvePoint {
                distance: 0.0,
                dsp_setting: 1.0,
            },
            X3DAudioDistanceCurvePoint {
                distance: 5.0,
                dsp_setting: 0.5,
            },
            X3DAudioDistanceCurvePoint {
                distance: 10.0,
                dsp_setting: 0.0,
            },
        ];
        let curve = X3DAudioDistanceCurve {
            points: pts.as_ptr(),
            point_count: pts.len() as u32,
        };
        let custom_emitter = X3DAudioEmitter {
            position: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 2.5,
            },
            curve_distance_scaler: 1.0,
            volume_curve: &curve,
            ..base_emitter
        };
        call_x3d(&handle, &listener, &custom_emitter, flags, &mut dsp);
        let expected_ch = 0.75 * FRAC_1_SQRT_2;
        assert!((mat[0] - expected_ch).abs() < 1e-4);
        assert!((mat[1] - expected_ch).abs() < 1e-4);

        // 5.1 surround (0x3F = FL, FR, FC, LFE, BL, BR): sound straight ahead -> FC (index 2).
        let mut handle51 = [0u8; 20];
        assert_eq!(
            X3DAudioInitialize(0x3F, X3DAUDIO_SPEED_OF_SOUND, &mut handle51),
            S_OK
        );
        let mut mat51 = [0.0f32; 6];
        let mut dsp51 = X3DAudioDspSettings {
            matrix_coefficients: mat51.as_mut_ptr(),
            dst_channel_count: 6,
            ..dsp
        };
        let front_emitter = X3DAudioEmitter {
            position: X3DAudioVector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            curve_distance_scaler: 1.0,
            ..base_emitter
        };
        call_x3d(
            &handle51,
            &listener,
            &front_emitter,
            X3DAUDIO_CALCULATE_MATRIX,
            &mut dsp51,
        );
        assert!((mat51[2] - 1.0).abs() < 1e-4, "FC={}", mat51[2]);
        assert!(mat51[4].abs() < 1e-4 && mat51[5].abs() < 1e-4);
    }
}

fn submit_f32_buf(sv: *mut SourceVoice, samples: &[f32]) {
    unsafe {
        let raw: Vec<u8> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        let buf = XAudio2Buffer {
            flags: XAUDIO2_END_OF_STREAM,
            audio_bytes: raw.len() as u32,
            audio_data: raw.as_ptr(),
            play_begin: 0,
            play_length: 0,
            loop_begin: 0,
            loop_length: 0,
            loop_count: 0,
            context: 0,
        };
        assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
    }
}

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|&v| v * v).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

#[test]
fn xapo_com_interfaces_and_createfx_clsids() {
    unsafe {
        // Null pointer checks.
        assert_eq!(CreateAudioReverb(std::ptr::null_mut()), E_POINTER);
        assert_eq!(CreateAudioVolumeMeter(std::ptr::null_mut()), E_POINTER);
        let mut out = 0xDEADusize;
        assert_eq!(CreateFX(std::ptr::null(), &mut out, 0, 0), E_POINTER);
        assert_eq!(out, 0);

        // Unknown CLSID -> REGDB_E_CLASSNOTREG.
        let unknown_clsid = GUID::new(0x1234_5678, 0x9ABC, 0xDEF0, [1, 2, 3, 4, 5, 6, 7, 8]);
        out = 0xDEAD;
        assert_eq!(
            CreateFX(&unknown_clsid as *const GUID as *const u8, &mut out, 0, 0),
            REGDB_E_CLASSNOTREG
        );
        assert_eq!(out, 0);

        // Create all 6 builtin effects (including legacy 2.7 XAPOFX CLSIDs) and test COM QI.
        let mut objs: Vec<(usize, GUID)> = Vec::new();
        let mut rev = 0usize;
        assert_eq!(CreateAudioReverb(&mut rev), S_OK);
        assert_ne!(rev, 0);
        objs.push((rev, CLSID_AUDIO_REVERB));

        let mut vm = 0usize;
        assert_eq!(CreateAudioVolumeMeter(&mut vm), S_OK);
        assert_ne!(vm, 0);
        objs.push((vm, CLSID_AUDIO_VOLUME_METER));

        for &(clsid, expected_clsid) in &[
            (CLSID_FX_EQ, CLSID_FX_EQ),
            (CLSID_FX_MASTERING_LIMITER, CLSID_FX_MASTERING_LIMITER),
            (CLSID_FX_REVERB, CLSID_FX_REVERB),
            (CLSID_FX_ECHO, CLSID_FX_ECHO),
            (CLSID_FX_EQ_27, CLSID_FX_EQ),
            (CLSID_FX_MASTERING_LIMITER_27, CLSID_FX_MASTERING_LIMITER),
            (CLSID_FX_REVERB_27, CLSID_FX_REVERB),
            (CLSID_FX_ECHO_27, CLSID_FX_ECHO),
        ] {
            let mut fx = 0usize;
            assert_eq!(
                CreateFX(&clsid as *const GUID as *const u8, &mut fx, 0, 0),
                S_OK
            );
            assert_ne!(fx, 0);
            objs.push((fx, expected_clsid));
        }

        let bad_iid = GUID::new(0xAAAA_BBBB, 0xCCCC, 0xDDDD, [0xEE; 8]);
        let f32_2ch = float_wfx(2, 48_000);
        let pcm16_2ch = pcm_wfx(2, 48_000, 16);

        for (obj, expected_clsid) in objs {
            let xapo = obj as *mut BuiltinXapoCom;
            // QI for IXAPO returns primary interface pointer.
            let mut p_xapo: *mut c_void = std::ptr::null_mut();
            assert_eq!(xapo_query_interface(xapo, &IID_IXAPO, &mut p_xapo), S_OK);
            assert_eq!(p_xapo as usize, obj);

            // QI for IXAPOParameters returns distinct secondary interface pointer.
            let mut p_params: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                xapo_query_interface(xapo, &IID_IXAPO_PARAMETERS, &mut p_params),
                S_OK
            );
            assert!(!p_params.is_null());
            assert_ne!(p_params as usize, obj);

            // Round-trip QI from IXAPOParameters -> IUnknown & IXAPO obeys COM identity.
            let mut p_unk_back: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                xapo_params_query_interface(p_params, &IID_IUnknown, &mut p_unk_back),
                S_OK
            );
            assert_eq!(p_unk_back as usize, obj);

            let mut p_xapo_back: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                xapo_params_query_interface(p_params, &IID_IXAPO, &mut p_xapo_back),
                S_OK
            );
            assert_eq!(p_xapo_back as usize, obj);

            let mut p_bad: *mut c_void = 0x1234 as *mut c_void;
            assert_eq!(
                xapo_params_query_interface(p_params, &bad_iid, &mut p_bad),
                E_NOINTERFACE
            );
            assert!(p_bad.is_null());

            // Registration properties.
            let mut props_ptr: *mut XapoRegistrationProperties = std::ptr::null_mut();
            assert_eq!(
                xapo_get_registration_properties(xapo, &mut props_ptr),
                S_OK
            );
            assert!(!props_ptr.is_null());
            let props = std::ptr::read_unaligned(props_ptr);
            assert_eq!({ props.clsid }, expected_clsid);
            assert_eq!({ props.min_input_buffer_count }, 1);
            assert_eq!({ props.max_output_buffer_count }, 1);
            let friendly = { props.friendly_name };
            assert_ne!(friendly[0], 0);
            crate::heap::free(props_ptr as *mut c_void);

            // Format negotiation & frame counts.
            let mut sup: *mut WAVEFORMATEX = std::ptr::null_mut();
            assert_eq!(
                xapo_is_input_format_supported(xapo, &f32_2ch, &f32_2ch, &mut sup),
                S_OK
            );
            assert_eq!(
                xapo_is_output_format_supported(xapo, &f32_2ch, &pcm16_2ch, &mut sup),
                XAPO_E_FORMAT_UNSUPPORTED
            );
            assert!(!sup.is_null());
            let suggested = std::ptr::read_unaligned(sup);
            assert_eq!({ suggested.wFormatTag }, WAVE_FORMAT_IEEE_FLOAT);
            assert_eq!({ suggested.wBitsPerSample }, 32);
            crate::heap::free(sup as *mut c_void);

            assert_eq!(xapo_calc_input_frames(xapo, 480), 480);
            assert_eq!(xapo_calc_output_frames(xapo, 480), 480);

            // Release all 5 refs (initial + p_xapo + p_params + p_unk_back + p_xapo_back).
            assert_eq!(xapo_params_release(p_params), 4);
            assert_eq!(xapo_release(p_xapo_back as *mut BuiltinXapoCom), 3);
            assert_eq!(xapo_release(p_unk_back as *mut BuiltinXapoCom), 2);
            assert_eq!(xapo_release(p_xapo as *mut BuiltinXapoCom), 1);
            assert_eq!(xapo_release(xapo), 0);
        }
    }
}

#[test]
fn audio_reverb_and_fxreverb_decaying_tail() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);
        let mut rev_ptr = 0usize;
        assert_eq!(CreateAudioReverb(&mut rev_ptr), S_OK);

        let desc = XAudio2EffectDescriptor {
            effect: rev_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 2,
        };
        let chain = XAudio2EffectChain {
            effect_count: 1,
            effects: &desc,
        };

        // Attach AudioReverb directly via CreateSubmixVoice.
        let mut sub: *mut SubmixVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_submix_voice(xa2, &mut sub, 2, 48_000, 0, 0, std::ptr::null(), &chain),
            S_OK
        );
        // Caller releases its own COM ref; voice still holds the effect alive.
        assert_eq!(xapo_release(rev_ptr as *mut BuiltinXapoCom), 1);

        // Configure reverb parameters and verify round-trip (2.9 + 2.7 struct sizes).
        let rev_params = XAudio2FxReverbParameters {
            wet_dry_mix: 100.0,
            reflections_delay: 2,
            reverb_delay: 4,
            decay_time: 0.6,
            density: 100.0,
            room_size: 80.0,
            ..XAudio2FxReverbParameters::default()
        };
        assert_eq!(
            voice_set_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &rev_params as *const XAudio2FxReverbParameters as *const c_void,
                std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
                0
            ),
            S_OK
        );
        let mut got_rev = XAudio2FxReverbParameters::default();
        assert_eq!(
            voice_get_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &mut got_rev as *mut XAudio2FxReverbParameters as *mut c_void,
                std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
            ),
            S_OK
        );
        assert!((got_rev.decay_time - 0.6).abs() < 1e-6);
        assert!((got_rev.room_size - 80.0).abs() < 1e-6);

        let mut got_rev27 = std::mem::zeroed::<XAudio2FxReverbParameters27>();
        assert_eq!(
            voice_get_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &mut got_rev27 as *mut XAudio2FxReverbParameters27 as *mut c_void,
                std::mem::size_of::<XAudio2FxReverbParameters27>() as u32,
            ),
            S_OK
        );
        assert!((got_rev27.decay_time - 0.6).abs() < 1e-6);

        // Route a stereo source voice into the reverb submix and fire a brief impulse burst.
        let send = XAudio2SendDescriptor {
            flags: 0,
            output_voice: sub as *mut VoiceHeader,
        };
        let sends = XAudio2VoiceSends {
            send_count: 1,
            sends: &send,
        };
        let wfx = float_wfx(2, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, &sends, std::ptr::null()),
            S_OK
        );
        let mut burst = vec![0.0f32; 64 * 2];
        for i in 0..8 {
            burst[i * 2] = 0.8;
            burst[i * 2 + 1] = 0.8;
        }
        submit_f32_buf(sv, &burst);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        // Mix 1.0 second total across 100 quantums of 480 frames (10 ms each).
        let mut blocks = Vec::new();
        for _ in 0..100 {
            let mut q = vec![0.0f32; 480 * 2];
            let _ = (&*xa2).core.mix_quantum(480, &mut q);
            blocks.push(q);
        }
        // Source burst finished inside block 0; blocks 3..15 are early/mid tail, blocks 40..60 late tail.
        let early_rms = rms(&blocks[5]);
        let mid_rms = rms(&blocks[25]);
        let late_rms = rms(&blocks[75]);
        assert!(
            early_rms > 0.01,
            "expected audible reverb tail, early_rms={early_rms}"
        );
        assert!(
            mid_rms > 0.0005,
            "expected sustained mid tail, mid_rms={mid_rms}"
        );
        assert!(
            early_rms > mid_rms && mid_rms > late_rms,
            "expected decaying reverb tail: early={early_rms} mid={mid_rms} late={late_rms}"
        );
        // All samples finite and bounded.
        for b in &blocks {
            for &s in b {
                assert!(s.is_finite() && s.abs() < 4.0);
            }
        }

        // DisableLateField should silence the late tail.
        let no_late = XAudio2FxReverbParameters {
            disable_late_field: 1,
            ..rev_params
        };
        assert_eq!(
            voice_set_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &no_late as *const XAudio2FxReverbParameters as *const c_void,
                std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
                0
            ),
            S_OK
        );
        let mut after_disable = vec![0.0f32; 480 * 2];
        for _ in 0..10 {
            let _ = (&*xa2).core.mix_quantum(480, &mut after_disable);
        }
        assert!(
            rms(&after_disable) < 1e-5,
            "DisableLateField should eliminate late tail, got {}",
            rms(&after_disable)
        );

        // Now swap in CreateFX(CLSID_FX_REVERB) on the submix and confirm decaying tail too.
        let mut fxrev_ptr = 0usize;
        assert_eq!(
            CreateFX(
                &CLSID_FX_REVERB as *const GUID as *const u8,
                &mut fxrev_ptr,
                0,
                0
            ),
            S_OK
        );
        let fxdesc = XAudio2EffectDescriptor {
            effect: fxrev_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 2,
        };
        let fxchain = XAudio2EffectChain {
            effect_count: 1,
            effects: &fxdesc,
        };
        assert_eq!(
            voice_set_effect_chain(sub as *mut VoiceHeader, &fxchain),
            S_OK
        );
        assert_eq!(xapo_release(fxrev_ptr as *mut BuiltinXapoCom), 1);

        let fx_params = FxReverbParameters {
            diffusion: 0.9,
            room_size: 0.85,
        };
        assert_eq!(
            voice_set_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &fx_params as *const FxReverbParameters as *const c_void,
                std::mem::size_of::<FxReverbParameters>() as u32,
                0
            ),
            S_OK
        );
        let mut got_fx = FxReverbParameters::default();
        assert_eq!(
            voice_get_effect_parameters(
                sub as *mut VoiceHeader,
                0,
                &mut got_fx as *mut FxReverbParameters as *mut c_void,
                std::mem::size_of::<FxReverbParameters>() as u32,
            ),
            S_OK
        );
        assert!((got_fx.room_size - 0.85).abs() < 1e-6);

        submit_f32_buf(sv, &burst);
        let mut fx_blocks = Vec::new();
        for _ in 0..80 {
            let mut q = vec![0.0f32; 480 * 2];
            let _ = (&*xa2).core.mix_quantum(480, &mut q);
            fx_blocks.push(q);
        }
        let fx_early = rms(&fx_blocks[5]);
        let fx_late = rms(&fx_blocks[65]);
        assert!(fx_early > 0.005, "FXReverb early tail too weak: {fx_early}");
        assert!(
            fx_early > fx_late,
            "FXReverb tail should decay: early={fx_early} late={fx_late}"
        );

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(sub as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn volume_meter_live_peak_and_rms() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);
        let mut vm_ptr = 0usize;
        assert_eq!(CreateAudioVolumeMeter(&mut vm_ptr), S_OK);

        let desc = XAudio2EffectDescriptor {
            effect: vm_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 2,
        };
        let chain = XAudio2EffectChain {
            effect_count: 1,
            effects: &desc,
        };
        assert_eq!(
            voice_set_effect_chain(mv as *mut VoiceHeader, &chain),
            S_OK
        );
        assert_eq!(xapo_release(vm_ptr as *mut BuiltinXapoCom), 1);

        let wfx = float_wfx(2, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        // 480 stereo frames: L = constant +0.5, R = 10 full cycles of sine amp 0.8.
        let frames = 480usize;
        let mut stereo = Vec::with_capacity(frames * 2);
        for i in 0..frames {
            let phase = TAU * 10.0 * (i as f32) / (frames as f32);
            stereo.push(0.5);
            stereo.push(0.8 * phase.sin());
        }
        submit_f32_buf(sv, &stereo);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut out = vec![0.0f32; frames * 2];
        let _ = (&*xa2).core.mix_quantum(frames, &mut out);

        // Volume meter must pass audio through unmodified.
        assert!((out[0] - 0.5).abs() < 1e-5);
        assert!((out[25] - stereo[25]).abs() < 1e-5);

        // Query both peak and RMS.
        let mut peaks = [0.0f32; 2];
        let mut rms_vals = [0.0f32; 2];
        let mut levels = XAudio2FxVolumeMeterLevels {
            peak_levels: peaks.as_mut_ptr(),
            rms_levels: rms_vals.as_mut_ptr(),
            channel_count: 2,
        };
        assert_eq!(
            voice_get_effect_parameters(
                mv as *mut VoiceHeader,
                0,
                &mut levels as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
            ),
            S_OK
        );
        assert!((peaks[0] - 0.5).abs() < 1e-4, "L peak={}", peaks[0]);
        assert!((rms_vals[0] - 0.5).abs() < 1e-4, "L rms={}", rms_vals[0]);
        assert!((peaks[1] - 0.8).abs() < 1e-2, "R peak={}", peaks[1]);
        assert!(
            (rms_vals[1] - (0.8 * FRAC_1_SQRT_2)).abs() < 1e-2,
            "R rms={}",
            rms_vals[1]
        );

        // Peak-only query (`rms_levels = null`) must succeed.
        let mut peaks_only = [0.0f32; 2];
        let mut lv_peak_only = XAudio2FxVolumeMeterLevels {
            peak_levels: peaks_only.as_mut_ptr(),
            rms_levels: std::ptr::null_mut(),
            channel_count: 2,
        };
        assert_eq!(
            voice_get_effect_parameters(
                mv as *mut VoiceHeader,
                0,
                &mut lv_peak_only as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
            ),
            S_OK
        );
        assert!((peaks_only[0] - 0.5).abs() < 1e-4);

        // Next silent quantum resets reported levels to 0.0.
        let _ = (&*xa2).core.mix_quantum(frames, &mut out);
        assert_eq!(
            voice_get_effect_parameters(
                mv as *mut VoiceHeader,
                0,
                &mut levels as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
            ),
            S_OK
        );
        assert_eq!(peaks, [0.0, 0.0]);
        assert_eq!(rms_vals, [0.0, 0.0]);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn fxeq_four_band_parametric_biquad_dsp() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        let mut eq_ptr = 0usize;
        assert_eq!(
            CreateFX(&CLSID_FX_EQ as *const GUID as *const u8, &mut eq_ptr, 0, 0),
            S_OK
        );
        let desc = XAudio2EffectDescriptor {
            effect: eq_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 1,
        };
        let chain = XAudio2EffectChain {
            effect_count: 1,
            effects: &desc,
        };
        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, std::ptr::null(), &chain),
            S_OK
        );
        assert_eq!(xapo_release(eq_ptr as *mut BuiltinXapoCom), 1);

        // Invalid out-of-range gain must fail.
        let bad_eq = FxEqParameters {
            gain0: 99.0,
            ..FxEqParameters::default()
        };
        assert_eq!(
            voice_set_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &bad_eq as *const FxEqParameters as *const c_void,
                std::mem::size_of::<FxEqParameters>() as u32,
                0
            ),
            E_INVALIDARG
        );

        // Boost 2000 Hz by 4x (+12 dB) and cut 100 Hz by 0.25x (-12 dB).
        let eq = FxEqParameters {
            frequency_center0: 100.0,
            gain0: 0.25,
            bandwidth0: 0.5,
            frequency_center2: 2000.0,
            gain2: 4.0,
            bandwidth2: 0.5,
            ..FxEqParameters::default()
        };
        assert_eq!(
            voice_set_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &eq as *const FxEqParameters as *const c_void,
                std::mem::size_of::<FxEqParameters>() as u32,
                0
            ),
            S_OK
        );
        let mut got_eq = FxEqParameters::default();
        assert_eq!(
            voice_get_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &mut got_eq as *mut FxEqParameters as *mut c_void,
                std::mem::size_of::<FxEqParameters>() as u32,
            ),
            S_OK
        );
        assert!((got_eq.gain2 - 4.0).abs() < 1e-6);

        let n = 4800usize;
        let sine_2k: Vec<f32> = (0..n)
            .map(|i| 0.2 * (TAU * 2000.0 * (i as f32) / 48_000.0).sin())
            .collect();
        let sine_100: Vec<f32> = (0..n)
            .map(|i| 0.2 * (TAU * 100.0 * (i as f32) / 48_000.0).sin())
            .collect();

        let in_2k_rms = rms(&sine_2k[n / 2..]);
        let in_100_rms = rms(&sine_100[n / 2..]);

        submit_f32_buf(sv, &sine_2k);
        assert_eq!(sv_start(sv, 0, 0), S_OK);
        let mut out_2k = vec![0.0f32; n];
        let _ = (&*xa2).core.mix_quantum(n, &mut out_2k);
        let gain_2k = rms(&out_2k[n / 2..]) / in_2k_rms;
        assert!(
            (gain_2k - 4.0).abs() < 0.35,
            "expected ~4x boost at 2000 Hz, got {gain_2k}"
        );

        submit_f32_buf(sv, &sine_100);
        let mut out_100 = vec![0.0f32; n];
        let _ = (&*xa2).core.mix_quantum(n, &mut out_100);
        let gain_100 = rms(&out_100[n / 2..]) / in_100_rms;
        assert!(
            (gain_100 - 0.25).abs() < 0.08,
            "expected ~0.25x cut at 100 Hz, got {gain_100}"
        );

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn fx_mastering_limiter_peak_envelope_and_release() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(2, 48_000);
        let mut lim_ptr = 0usize;
        assert_eq!(
            CreateFX(
                &CLSID_FX_MASTERING_LIMITER as *const GUID as *const u8,
                &mut lim_ptr,
                0,
                0
            ),
            S_OK
        );
        let desc = XAudio2EffectDescriptor {
            effect: lim_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 2,
        };
        let chain = XAudio2EffectChain {
            effect_count: 1,
            effects: &desc,
        };
        assert_eq!(
            voice_set_effect_chain(mv as *mut VoiceHeader, &chain),
            S_OK
        );
        assert_eq!(xapo_release(lim_ptr as *mut BuiltinXapoCom), 1);

        let wfx = float_wfx(2, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        // Hot signal (amplitude 3.0) with default Loudness = 1000 -> brickwall ceiling <= 1.0.
        let hot = vec![3.0f32; 256 * 2];
        submit_f32_buf(sv, &hot);
        let mut out = vec![0.0f32; 256 * 2];
        let _ = (&*xa2).core.mix_quantum(256, &mut out);
        let max_hot = out.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
        assert!(
            max_hot <= 1.001 && max_hot >= 0.99,
            "limiter ceiling expected ~1.0, got {max_hot}"
        );

        // Lower Loudness to 500 -> ceiling drops to ~0.5.
        let p500 = FxMasteringLimiterParameters {
            release: 1,
            loudness: 500,
        };
        assert_eq!(
            voice_set_effect_parameters(
                mv as *mut VoiceHeader,
                0,
                &p500 as *const FxMasteringLimiterParameters as *const c_void,
                std::mem::size_of::<FxMasteringLimiterParameters>() as u32,
                0
            ),
            S_OK
        );
        let med = vec![0.9f32; 512 * 2];
        submit_f32_buf(sv, &med);
        let mut out500 = vec![0.0f32; 512 * 2];
        let _ = (&*xa2).core.mix_quantum(512, &mut out500);
        let max_500 = out500.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
        assert!(
            max_500 <= 0.501 && max_500 >= 0.49,
            "limiter ceiling at Loudness=500 expected ~0.5, got {max_500}"
        );

        // Release speed test: after a 4.0 transient, a 0.2 signal recovers faster with Release=1ms than 20ms.
        let measure_recovery = |release_ms: u32| -> f32 {
            let p = FxMasteringLimiterParameters {
                release: release_ms,
                loudness: 1000,
            };
            assert_eq!(
                voice_set_effect_parameters(
                    mv as *mut VoiceHeader,
                    0,
                    &p as *const FxMasteringLimiterParameters as *const c_void,
                    std::mem::size_of::<FxMasteringLimiterParameters>() as u32,
                    0
                ),
                S_OK
            );
            // Drain silence first so envelope starts clean.
            let mut sil = vec![0.0f32; 4800 * 2];
            let _ = (&*xa2).core.mix_quantum(4800, &mut sil);
            // 10 frames of 4.0 followed by 240 frames (5 ms) of 0.2.
            let mut sig = vec![0.2f32; 250 * 2];
            for i in 0..20 {
                sig[i] = 4.0;
            }
            submit_f32_buf(sv, &sig);
            let mut res = vec![0.0f32; 250 * 2];
            let _ = (&*xa2).core.mix_quantum(250, &mut res);
            res[249 * 2]
        };
        let fast_rec = measure_recovery(1);
        let slow_rec = measure_recovery(20);
        assert!(
            fast_rec > slow_rec + 0.03,
            "Release=1ms ({fast_rec}) should recover faster than Release=20ms ({slow_rec})"
        );

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn fx_echo_delay_and_feedback_dsp() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        let init = FxEchoInitData { max_delay: 200.0 };
        let mut echo_ptr = 0usize;
        assert_eq!(
            CreateFX(
                &CLSID_FX_ECHO as *const GUID as *const u8,
                &mut echo_ptr,
                &init as *const FxEchoInitData as usize,
                std::mem::size_of::<FxEchoInitData>() as u32
            ),
            S_OK
        );
        let desc = XAudio2EffectDescriptor {
            effect: echo_ptr as *mut c_void,
            initial_state: 1,
            output_channels: 1,
        };
        let chain = XAudio2EffectChain {
            effect_count: 1,
            effects: &desc,
        };
        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, std::ptr::null(), &chain),
            S_OK
        );
        assert_eq!(xapo_release(echo_ptr as *mut BuiltinXapoCom), 1);

        // Delay > MaxDelay (200 ms) must be rejected.
        let too_long = FxEchoParameters {
            wet_dry_mix: 0.5,
            feedback: 0.5,
            delay: 300.0,
        };
        assert_eq!(
            voice_set_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &too_long as *const FxEchoParameters as *const c_void,
                std::mem::size_of::<FxEchoParameters>() as u32,
                0
            ),
            E_INVALIDARG
        );

        // 10 ms = exactly 480 samples at 48 kHz.
        let echo_p = FxEchoParameters {
            wet_dry_mix: 0.5,
            feedback: 0.5,
            delay: 10.0,
        };
        assert_eq!(
            voice_set_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &echo_p as *const FxEchoParameters as *const c_void,
                std::mem::size_of::<FxEchoParameters>() as u32,
                0
            ),
            S_OK
        );

        // Submit a single-frame impulse and mix across multiple quantums (including empty queue tail).
        submit_f32_buf(sv, &[1.0f32]);
        assert_eq!(sv_start(sv, 0, 0), S_OK);

        let mut all = Vec::with_capacity(2000);
        for _ in 0..10 {
            let mut q = vec![0.0f32; 200];
            let _ = (&*xa2).core.mix_quantum(200, &mut q);
            all.extend_from_slice(&q);
        }
        assert!((all[0] - 0.5).abs() < 1e-4, "dry impulse={}", all[0]);
        assert!(all[100].abs() < 1e-6, "between echoes={}", all[100]);
        assert!((all[480] - 0.5).abs() < 1e-4, "echo 1={}", all[480]);
        assert!((all[960] - 0.25).abs() < 1e-4, "echo 2={}", all[960]);
        assert!((all[1440] - 0.125).abs() < 1e-4, "echo 3={}", all[1440]);
        assert!((all[1920] - 0.0625).abs() < 1e-4, "echo 4={}", all[1920]);

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn live_effect_chain_ordering_toggling_and_guest_com_xapo() {
    // Custom guest COM object implementing IXAPO + IXAPOParameters with real Win64 vtables.
    #[repr(C)]
    struct MockGuestXapo {
        #[allow(dead_code)]
        xapo_vtbl: *const [crate::Ptr; 13],
        #[allow(dead_code)]
        params_vtbl: *const [crate::Ptr; 5],
        refs: AtomicU32,
        locked_calls: AtomicU32,
        unlocked_calls: Arc<AtomicU32>,
        process_calls: AtomicU32,
        last_enabled: AtomicU32,
        gain_bits: AtomicU32,
        destroyed: Arc<AtomicBool>,
    }

    unsafe fn from_params(p: *mut c_void) -> *mut MockGuestXapo {
        (p as *mut u8).sub(std::mem::offset_of!(MockGuestXapo, params_vtbl)) as *mut MockGuestXapo
    }

    unsafe extern "win64" fn g_qi(
        this: *mut MockGuestXapo,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> i32 {
        std::ptr::write_unaligned(ppv, std::ptr::null_mut());
        let iid = std::ptr::read_unaligned(riid);
        let obj = &*this;
        if iid == IID_IUnknown || iid == IID_IXAPO {
            obj.refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this as *mut c_void);
            S_OK
        } else if iid == IID_IXAPO_PARAMETERS {
            obj.refs.fetch_add(1, Ordering::AcqRel);
            let p =
                (this as *mut u8).add(std::mem::offset_of!(MockGuestXapo, params_vtbl))
                    as *mut c_void;
            std::ptr::write_unaligned(ppv, p);
            S_OK
        } else {
            E_NOINTERFACE
        }
    }
    unsafe extern "win64" fn g_add_ref(this: *mut MockGuestXapo) -> u32 {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel) + 1
    }
    unsafe extern "win64" fn g_release(this: *mut MockGuestXapo) -> u32 {
        let prev = {
            let obj = &*this;
            obj.refs.fetch_sub(1, Ordering::AcqRel)
        };
        if prev == 1 {
            let boxed = Box::from_raw(this);
            boxed.destroyed.store(true, Ordering::Release);
            0
        } else {
            prev - 1
        }
    }
    unsafe extern "win64" fn g_get_reg(
        _this: *mut MockGuestXapo,
        _pp: *mut *mut XapoRegistrationProperties,
    ) -> i32 {
        E_NOINTERFACE
    }
    unsafe extern "win64" fn g_fmt_ok(
        _this: *mut MockGuestXapo,
        _a: *const WAVEFORMATEX,
        _b: *const WAVEFORMATEX,
        _c: *mut *mut WAVEFORMATEX,
    ) -> i32 {
        S_OK
    }
    unsafe extern "win64" fn g_init(
        _this: *mut MockGuestXapo,
        _d: *const c_void,
        _s: u32,
    ) -> i32 {
        S_OK
    }
    unsafe extern "win64" fn g_reset(_this: *mut MockGuestXapo) {}
    unsafe extern "win64" fn g_lock(
        this: *mut MockGuestXapo,
        _ic: u32,
        _ip: *const XapoLockForProcessBufferParameters,
        _oc: u32,
        _op: *const XapoLockForProcessBufferParameters,
    ) -> i32 {
        let obj = &*this;
        obj.locked_calls.fetch_add(1, Ordering::AcqRel);
        S_OK
    }
    unsafe extern "win64" fn g_unlock(this: *mut MockGuestXapo) {
        let obj = &*this;
        obj.unlocked_calls.fetch_add(1, Ordering::AcqRel);
    }
    unsafe extern "win64" fn g_process(
        this: *mut MockGuestXapo,
        _ic: u32,
        p_in: *const XapoProcessBufferParameters,
        _oc: u32,
        p_out: *mut XapoProcessBufferParameters,
        is_enabled: i32,
    ) {
        let obj = &*this;
        obj.process_calls.fetch_add(1, Ordering::AcqRel);
        obj.last_enabled.store(is_enabled as u32, Ordering::Release);
        let in_p = std::ptr::read_unaligned(p_in);
        let mut out_p = std::ptr::read_unaligned(p_out);
        // Verify 16-byte alignment required by XAPO spec.
        assert_eq!((in_p.buffer as usize) % 16, 0);
        assert_eq!((out_p.buffer as usize) % 16, 0);
        let frames = in_p.valid_frame_count as usize;
        if in_p.buffer_flags == XAPO_BUFFER_SILENT {
            out_p.buffer_flags = XAPO_BUFFER_SILENT;
            out_p.valid_frame_count = in_p.valid_frame_count;
            std::ptr::write_unaligned(p_out, out_p);
            return;
        }
        let gain = if is_enabled != 0 {
            f32::from_bits(obj.gain_bits.load(Ordering::Acquire))
        } else {
            1.0
        };
        let src = std::slice::from_raw_parts(in_p.buffer as *const f32, frames);
        let dst = std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames);
        for i in 0..frames {
            dst[i] = src[i] * gain;
        }
        out_p.buffer_flags = XAPO_BUFFER_VALID;
        out_p.valid_frame_count = in_p.valid_frame_count;
        std::ptr::write_unaligned(p_out, out_p);
    }
    unsafe extern "win64" fn g_frames(_this: *mut MockGuestXapo, n: u32) -> u32 {
        n
    }
    unsafe extern "win64" fn gp_qi(
        p: *mut c_void,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> i32 {
        g_qi(from_params(p), riid, ppv)
    }
    unsafe extern "win64" fn gp_add_ref(p: *mut c_void) -> u32 {
        g_add_ref(from_params(p))
    }
    unsafe extern "win64" fn gp_release(p: *mut c_void) -> u32 {
        g_release(from_params(p))
    }
    unsafe extern "win64" fn gp_set_params(p: *mut c_void, data: *const c_void, size: u32) {
        if !data.is_null() && size >= 4 {
            let g = std::ptr::read_unaligned(data as *const f32);
            let obj = &*from_params(p);
            obj.gain_bits.store(g.to_bits(), Ordering::Release);
        }
    }
    unsafe extern "win64" fn gp_get_params(p: *mut c_void, data: *mut c_void, size: u32) {
        if !data.is_null() && size >= 4 {
            let obj = &*from_params(p);
            let g = f32::from_bits(obj.gain_bits.load(Ordering::Acquire));
            std::ptr::write_unaligned(data as *mut f32, g);
        }
    }

    static GUEST_XAPO_VTBL: Vtable<13> = Vtable([
        crate::Ptr(g_qi as *const ()),
        crate::Ptr(g_add_ref as *const ()),
        crate::Ptr(g_release as *const ()),
        crate::Ptr(g_get_reg as *const ()),
        crate::Ptr(g_fmt_ok as *const ()),
        crate::Ptr(g_fmt_ok as *const ()),
        crate::Ptr(g_init as *const ()),
        crate::Ptr(g_reset as *const ()),
        crate::Ptr(g_lock as *const ()),
        crate::Ptr(g_unlock as *const ()),
        crate::Ptr(g_process as *const ()),
        crate::Ptr(g_frames as *const ()),
        crate::Ptr(g_frames as *const ()),
    ]);
    static GUEST_PARAMS_VTBL: Vtable<5> = Vtable([
        crate::Ptr(gp_qi as *const ()),
        crate::Ptr(gp_add_ref as *const ()),
        crate::Ptr(gp_release as *const ()),
        crate::Ptr(gp_set_params as *const ()),
        crate::Ptr(gp_get_params as *const ()),
    ]);

    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let (xa2, mv) = make_stopped_test_engine(1, 48_000);
        let wfx = float_wfx(1, 48_000);
        let mut sv: *mut SourceVoice = std::ptr::null_mut();
        assert_eq!(
            xa2_create_source_voice(
                xa2,
                &mut sv,
                &wfx,
                0,
                2.0,
                0,
                std::ptr::null(),
                std::ptr::null()
            ),
            S_OK
        );

        // Chain: [0] Guest gain XAPO (3.0x) -> [1] Builtin FXMasteringLimiter (ceiling 1.0) -> [2] Builtin VolumeMeter.
        let destroyed = Arc::new(AtomicBool::new(false));
        let unlocked_calls = Arc::new(AtomicU32::new(0));
        let guest = Box::into_raw(Box::new(MockGuestXapo {
            xapo_vtbl: &GUEST_XAPO_VTBL.0,
            params_vtbl: &GUEST_PARAMS_VTBL.0,
            refs: AtomicU32::new(1),
            locked_calls: AtomicU32::new(0),
            unlocked_calls: unlocked_calls.clone(),
            process_calls: AtomicU32::new(0),
            last_enabled: AtomicU32::new(99),
            gain_bits: AtomicU32::new(3.0f32.to_bits()),
            destroyed: destroyed.clone(),
        }));

        let mut lim_ptr = 0usize;
        assert_eq!(
            CreateFX(
                &CLSID_FX_MASTERING_LIMITER as *const GUID as *const u8,
                &mut lim_ptr,
                0,
                0
            ),
            S_OK
        );
        let mut vm_ptr = 0usize;
        assert_eq!(CreateAudioVolumeMeter(&mut vm_ptr), S_OK);

        let descs = [
            XAudio2EffectDescriptor {
                effect: guest as *mut c_void,
                initial_state: 1,
                output_channels: 1,
            },
            XAudio2EffectDescriptor {
                effect: lim_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 1,
            },
            XAudio2EffectDescriptor {
                effect: vm_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 1,
            },
        ];
        let chain = XAudio2EffectChain {
            effect_count: 3,
            effects: descs.as_ptr(),
        };
        assert_eq!(
            voice_set_effect_chain(sv as *mut VoiceHeader, &chain),
            S_OK
        );
        let guest_ref = &*guest;
        assert_eq!(guest_ref.locked_calls.load(Ordering::Acquire), 1);

        // Caller releases its own refs; voice keeps all three alive.
        assert_eq!(g_release(guest), 2); // IXAPO + IXAPOParameters refs held by voice
        assert_eq!(xapo_release(lim_ptr as *mut BuiltinXapoCom), 1);
        assert_eq!(xapo_release(vm_ptr as *mut BuiltinXapoCom), 1);

        // Update guest gain parameter to 4.0 via IXAudio2Voice::SetEffectParameters & read back.
        let new_gain = 4.0f32;
        assert_eq!(
            voice_set_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &new_gain as *const f32 as *const c_void,
                4,
                0
            ),
            S_OK
        );
        let mut read_gain = 0.0f32;
        assert_eq!(
            voice_get_effect_parameters(
                sv as *mut VoiceHeader,
                0,
                &mut read_gain as *mut f32 as *mut c_void,
                4
            ),
            S_OK
        );
        assert_eq!(read_gain, 4.0);

        // Check initial enabled states.
        let mut en = 0i32;
        voice_get_effect_state(sv as *mut VoiceHeader, 0, &mut en);
        assert_eq!(en, 1);
        voice_get_effect_state(sv as *mut VoiceHeader, 1, &mut en);
        assert_eq!(en, 1);

        // Input 0.5 -> Guest (4.0x = 2.0) -> Limiter (clamped to 1.0) -> VolumeMeter (measures 1.0).
        assert_eq!(sv_start(sv, 0, 0), S_OK);
        submit_f32_buf(sv, &[0.5f32; 32]);
        let mut out = [0.0f32; 32];
        let _ = (&*xa2).core.mix_quantum(32, &mut out);
        assert!(
            (out[16] - 1.0).abs() < 1e-4,
            "chained limited output={}",
            out[16]
        );
        assert_eq!(guest_ref.last_enabled.load(Ordering::Acquire), 1);

        let mut peak = [0.0f32; 1];
        let mut lv = XAudio2FxVolumeMeterLevels {
            peak_levels: peak.as_mut_ptr(),
            rms_levels: std::ptr::null_mut(),
            channel_count: 1,
        };
        assert_eq!(
            voice_get_effect_parameters(
                sv as *mut VoiceHeader,
                2,
                &mut lv as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32
            ),
            S_OK
        );
        assert!(
            (peak[0] - 1.0).abs() < 1e-4,
            "meter after limiter={}",
            peak[0]
        );

        // Disable limiter (slot 1): now 0.5 -> Guest (4.0x = 2.0) -> bypass -> meter reads 2.0!
        assert_eq!(voice_disable_effect(sv as *mut VoiceHeader, 1, 0), S_OK);
        voice_get_effect_state(sv as *mut VoiceHeader, 1, &mut en);
        assert_eq!(en, 0);
        submit_f32_buf(sv, &[0.5f32; 32]);
        let _ = (&*xa2).core.mix_quantum(32, &mut out);
        assert!(
            (out[16] - 2.0).abs() < 1e-4,
            "limiter disabled output={}",
            out[16]
        );

        // Disable guest XAPO (slot 0) as well: passes 0.5 straight through with IsEnabled=0.
        assert_eq!(voice_disable_effect(sv as *mut VoiceHeader, 0, 0), S_OK);
        submit_f32_buf(sv, &[0.5f32; 32]);
        let _ = (&*xa2).core.mix_quantum(32, &mut out);
        assert!(
            (out[16] - 0.5).abs() < 1e-4,
            "guest disabled output={}",
            out[16]
        );
        assert_eq!(guest_ref.last_enabled.load(Ordering::Acquire), 0);

        // Re-enable both (slot 0 & slot 1): output returns to limited 1.0!
        assert_eq!(voice_enable_effect(sv as *mut VoiceHeader, 0, 0), S_OK);
        assert_eq!(voice_enable_effect(sv as *mut VoiceHeader, 1, 0), S_OK);
        submit_f32_buf(sv, &[0.5f32; 32]);
        let _ = (&*xa2).core.mix_quantum(32, &mut out);
        assert!(
            (out[16] - 1.0).abs() < 1e-4,
            "re-enabled output={}",
            out[16]
        );

        // Out-of-bounds effect index returns XAUDIO2_E_INVALID_CALL.
        assert_eq!(
            voice_enable_effect(sv as *mut VoiceHeader, 9, 0),
            XAUDIO2_E_INVALID_CALL
        );

        // Clearing the effect chain unlocks & releases guest COM object cleanly.
        assert_eq!(guest_ref.process_calls.load(Ordering::Acquire), 4);
        assert!(!destroyed.load(Ordering::Acquire));
        assert_eq!(
            voice_set_effect_chain(sv as *mut VoiceHeader, std::ptr::null()),
            S_OK
        );
        assert_eq!(unlocked_calls.load(Ordering::Acquire), 1);
        assert!(destroyed.load(Ordering::Acquire));

        voice_destroy_voice(sv as *mut VoiceHeader);
        voice_destroy_voice(mv as *mut VoiceHeader);
        assert_eq!(xa2_release(xa2), 0);
    }
}

#[test]
fn adpcm_ima_alaw_mulaw_wave_formats_and_rejection() {
    unsafe {
        // 1. Microsoft ADPCM (WAVE_FORMAT_ADPCM = 0x0002) mono block decode.
        #[repr(C, packed(2))]
        struct AdpcmWfx {
            wfx: WAVEFORMATEX,
            samples_per_block: u16,
            num_coef: u16,
            coefs: [(i16, i16); 7],
        }
        let adpcm_hdr = AdpcmWfx {
            wfx: WAVEFORMATEX {
                wFormatTag: 0x0002,
                nChannels: 1,
                nSamplesPerSec: 44_100,
                nAvgBytesPerSec: 22_050,
                nBlockAlign: 9, // 7-byte header + 2 data bytes (4 nibbles -> 6 samples total)
                wBitsPerSample: 4,
                cbSize: 32,
            },
            samples_per_block: 6,
            num_coef: 7,
            coefs: [
                (256, 0),
                (512, -256),
                (0, 0),
                (192, 64),
                (240, 0),
                (460, -208),
                (392, -232),
            ],
        };
        let fmt = parse_wave_format(&adpcm_hdr.wfx).expect("MS-ADPCM must parse");
        let mut blk = Vec::new();
        blk.push(0u8);
        blk.extend_from_slice(&100i16.to_le_bytes());
        blk.extend_from_slice(&16384i16.to_le_bytes());
        blk.extend_from_slice(&0i16.to_le_bytes());
        blk.push(0x10); // high=+1, low=0
        blk.push(0xF0); // high=-1, low=0
        let (out, frames) = decode_pcm_to_f32(&fmt, &blk);
        assert_eq!(frames, 6);
        assert_eq!(out.len(), 6);
        assert!(out[0].abs() < 1e-4);
        assert!((out[1] - 0.5).abs() < 1e-4);
        assert!((out[2] - (16484.0 / 32768.0)).abs() < 1e-4);

        // 2. IMA/DVI ADPCM (0x0011) mono block decode.
        #[repr(C, packed(2))]
        struct ImaWfx {
            wfx: WAVEFORMATEX,
            samples_per_block: u16,
        }
        let ima_hdr = ImaWfx {
            wfx: WAVEFORMATEX {
                wFormatTag: 0x0011,
                nChannels: 1,
                nSamplesPerSec: 44_100,
                nAvgBytesPerSec: 22_050,
                nBlockAlign: 6,
                wBitsPerSample: 4,
                cbSize: 2,
            },
            samples_per_block: 5,
        };
        let ima_fmt = parse_wave_format(&ima_hdr.wfx).expect("IMA-ADPCM must parse");
        let mut ima_blk = Vec::new();
        ima_blk.extend_from_slice(&8192i16.to_le_bytes());
        ima_blk.push(10);
        ima_blk.push(0);
        ima_blk.push(0x00);
        ima_blk.push(0x00);
        let (ima_out, ima_frames) = decode_pcm_to_f32(&ima_fmt, &ima_blk);
        assert_eq!(ima_frames, 5);
        assert!((ima_out[0] - 0.25).abs() < 1e-4);

        // 3. A-law (0x0006) & u-law (0x0007).
        let alaw_wfx = WAVEFORMATEX {
            wFormatTag: 0x0006,
            nChannels: 1,
            nSamplesPerSec: 8_000,
            nAvgBytesPerSec: 8_000,
            nBlockAlign: 1,
            wBitsPerSample: 8,
            cbSize: 0,
        };
        let alaw_fmt = parse_wave_format(&alaw_wfx).expect("A-law must parse");
        let (a_out, a_n) = decode_pcm_to_f32(&alaw_fmt, &[0xD5, 0x2A]);
        assert_eq!(a_n, 2);
        assert!(a_out[0].abs() < 0.01);
        assert!(a_out[1].abs() > 0.9);

        let mulaw_wfx = WAVEFORMATEX { wFormatTag: 0x0007, ..alaw_wfx };
        let mulaw_fmt = parse_wave_format(&mulaw_wfx).expect("u-law must parse");
        let (u_out, u_n) = decode_pcm_to_f32(&mulaw_fmt, &[0xFF, 0x00]);
        assert_eq!(u_n, 2);
        assert!(u_out[0].abs() < 0.01);
        assert!(u_out[1] < -0.9);

        // 4. Unsupported format tag & unsupported EXTENSIBLE SubFormat -> XAUDIO2_E_INVALID_CALL.
        let bad_tag = WAVEFORMATEX { wFormatTag: 0x9999, ..alaw_wfx };
        assert_eq!(parse_wave_format(&bad_tag).unwrap_err(), XAUDIO2_E_INVALID_CALL);
        let bad_ext = WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE,
                nChannels: 2,
                nSamplesPerSec: 48_000,
                nAvgBytesPerSec: 384_000,
                nBlockAlign: 8,
                wBitsPerSample: 32,
                cbSize: 22,
            },
            wValidBitsPerSample: 32,
            dwChannelMask: 3,
            SubFormat: GUID::new(0x0000_0166, 0, 0, [0; 8]),
        };
        assert_eq!(
            parse_wave_format(&bad_ext.Format).unwrap_err(),
            XAUDIO2_E_INVALID_CALL
        );
    }
}

#[test]
fn ms_adpcm_sustained_large_nibbles_bounds_delta_without_overflow() {
    unsafe {
        #[repr(C, packed(2))]
        struct AdpcmWfx {
            wfx: WAVEFORMATEX,
            samples_per_block: u16,
            num_coef: u16,
            coefs: [(i16, i16); 2],
        }
        // 32-byte mono block: 7-byte header + 25 payload bytes = 50 nibbles -> 52 samples.
        let hdr = AdpcmWfx {
            wfx: WAVEFORMATEX {
                wFormatTag: 0x0002,
                nChannels: 1,
                nSamplesPerSec: 44_100,
                nAvgBytesPerSec: 22_050,
                nBlockAlign: 32,
                wBitsPerSample: 4,
                cbSize: 12,
            },
            samples_per_block: 52,
            num_coef: 2,
            coefs: [(256, 0), (i16::MIN, i16::MIN)],
        };
        let fmt = parse_wave_format(&hdr.wfx).expect("MS-ADPCM format must parse");

        // Sustained 0x77 nibbles (+7) with initial delta=32767: previously overflowed i32.
        let mut blk_pos = Vec::with_capacity(32);
        blk_pos.push(0u8);
        blk_pos.extend_from_slice(&i16::MAX.to_le_bytes()); // delta = 32767
        blk_pos.extend_from_slice(&i16::MAX.to_le_bytes()); // samp1 = 32767
        blk_pos.extend_from_slice(&0i16.to_le_bytes()); // samp2 = 0
        blk_pos.resize(32, 0x77);
        let (out_pos, frames_pos) = decode_pcm_to_f32(&fmt, &blk_pos);
        assert_eq!(frames_pos, 52);
        assert_eq!(out_pos.len(), 52);
        assert!(out_pos.iter().all(|s| s.is_finite() && (-1.0..=1.0).contains(s)));
        // Positive nibbles saturate cleanly to +32767/32768 without wrapping negative.
        assert!(out_pos[2..].iter().all(|&s| s > 0.99));

        // Sustained 0x88 nibbles (-8, table factor 768) with extreme coefs (i16::MIN, i16::MIN)
        // and extreme samp1/samp2 = i16::MIN must also decode cleanly without overflow.
        let mut blk_neg = Vec::with_capacity(32);
        blk_neg.push(1u8);
        blk_neg.extend_from_slice(&i16::MAX.to_le_bytes());
        blk_neg.extend_from_slice(&i16::MIN.to_le_bytes());
        blk_neg.extend_from_slice(&i16::MIN.to_le_bytes());
        blk_neg.resize(32, 0x88);
        let (out_neg, frames_neg) = decode_pcm_to_f32(&fmt, &blk_neg);
        assert_eq!(frames_neg, 52);
        assert_eq!(out_neg.len(), 52);
        assert!(out_neg.iter().all(|s| s.is_finite() && (-1.0..=1.0).contains(s)));
    }
}
