use super::buffer::*;
use super::com_vtbl::*;
use super::mixer::ds_mb_to_gain;
use super::*;
use crate::audio::backend::{self, Backend, DeviceInfo};
use crate::audio::com::{GUID, IID_IUnknown, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT};
use crate::audio::format::{
    WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_PCM,
};
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::time::Duration;

fn pcm_wfx(channels: u16, sample_rate: u32, bits: u16) -> WAVEFORMATEX {
    let align = channels * (bits / 8);
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_PCM,
        nChannels: channels,
        nSamplesPerSec: sample_rate,
        nAvgBytesPerSec: sample_rate * (align as u32),
        nBlockAlign: align,
        wBitsPerSample: bits,
        cbSize: 0,
    }
}

fn float_wfx(channels: u16, sample_rate: u32) -> WAVEFORMATEX {
    let align = channels * 4;
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_IEEE_FLOAT,
        nChannels: channels,
        nSamplesPerSec: sample_rate,
        nAvgBytesPerSec: sample_rate * (align as u32),
        nBlockAlign: align,
        wBitsPerSample: 32,
        cbSize: 0,
    }
}

fn make_desc(flags: u32, buffer_bytes: u32, wfx: *const u8) -> DsBufferDesc {
    DsBufferDesc {
        dw_size: std::mem::size_of::<DsBufferDesc>() as u32,
        dw_flags: flags,
        dw_buffer_bytes: buffer_bytes,
        dw_reserved: 0,
        lpwfx_format: wfx,
        guid_3d_algorithm: GUID_NULL,
    }
}

fn make_cap_desc(flags: u32, buffer_bytes: u32, wfx: *const u8) -> DscBufferDesc {
    DscBufferDesc {
        dw_size: std::mem::size_of::<DscBufferDesc>() as u32,
        dw_flags: flags,
        dw_buffer_bytes: buffer_bytes,
        dw_reserved: 0,
        lpwfx_format: wfx,
        dw_fx_count: 0,
        lp_dsc_fx_desc: std::ptr::null(),
    }
}

unsafe fn make_paused_ds8() -> *mut DsDeviceCom {
    let mut raw: *mut c_void = std::ptr::null_mut();
    assert_eq!(
        DirectSoundCreate8(std::ptr::null(), &mut raw, std::ptr::null_mut()),
        DS_OK
    );
    assert!(!raw.is_null());
    let ds = raw as *mut DsDeviceCom;
    (&*ds).core.mixer_paused.store(true, Ordering::Release);
    assert_eq!(
        ds_set_cooperative_level(ds, std::ptr::null_mut(), DSSCL_PRIORITY),
        DS_OK
    );
    ds
}

unsafe fn write_all_bytes(buf: *mut DsBufferCom, data: &[u8]) {
    let mut p1: *mut c_void = std::ptr::null_mut();
    let mut l1 = 0u32;
    let mut p2: *mut c_void = std::ptr::null_mut();
    let mut l2 = 0u32;
    assert_eq!(
        dsb_lock(
            buf,
            0,
            data.len() as u32,
            &mut p1,
            &mut l1,
            &mut p2,
            &mut l2,
            DSBLOCK_ENTIREBUFFER,
        ),
        DS_OK
    );
    assert_eq!(l1 as usize, data.len());
    assert_eq!(l2, 0);
    std::ptr::copy_nonoverlapping(data.as_ptr(), p1 as *mut u8, data.len());
    assert_eq!(dsb_unlock(buf, p1, l1, p2, l2), DS_OK);
}

#[test]
fn ordinals_and_class_factory_lifecycle() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    assert_eq!(ORDINALS.len(), 12);
    for (idx, &(ord, _)) in ORDINALS.iter().enumerate() {
        assert_eq!(ord as usize, idx + 1);
    }

    unsafe {
        // Unknown CLSID fails cleanly.
        let mut bad: *mut c_void = 0x1234 as *mut c_void;
        assert_eq!(
            DllGetClassObject(
                &GUID_NULL as *const GUID as *const u8,
                &IID_ICLASSFACTORY as *const GUID as *const u8,
                &mut bad,
            ),
            CLASS_E_CLASSNOTAVAILABLE
        );
        assert!(bad.is_null());

        // CLSID_DirectSound8 -> IClassFactory -> uninitialized IDirectSound8.
        let mut cf_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DllGetClassObject(
                &CLSID_DIRECTSOUND8 as *const GUID as *const u8,
                &IID_ICLASSFACTORY as *const GUID as *const u8,
                &mut cf_raw,
            ),
            DS_OK
        );
        assert_eq!(DllCanUnloadNow(), S_FALSE);

        let cf = cf_raw as *mut ClassFactoryCom;
        let mut ds8_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            cf_create_instance(cf, std::ptr::null_mut(), &IID_IDIRECTSOUND8, &mut ds8_raw),
            DS_OK
        );
        let ds8 = ds8_raw as *mut DsDeviceCom;

        // Calling methods before Initialize returns DSERR_UNINITIALIZED.
        let mut caps = DsCaps {
            dw_size: std::mem::size_of::<DsCaps>() as u32,
            ..DsCaps::default()
        };
        assert_eq!(ds_get_caps(ds8, &mut caps), DSERR_UNINITIALIZED);
        assert_eq!(ds_initialize(ds8, std::ptr::null()), DS_OK);
        assert_eq!(ds_initialize(ds8, std::ptr::null()), DSERR_ALREADYINITIALIZED);
        assert_eq!(ds_get_caps(ds8, &mut caps), DS_OK);
        assert_eq!(caps.dw_primary_buffers, 1);

        let mut cert = 99u32;
        assert_eq!(ds_verify_certification(ds8, &mut cert), DS_OK);
        assert_eq!(cert, DS_CERTIFIED);

        // LockServer increments/decrements server locks cleanly.
        assert_eq!(cf_lock_server(cf, TRUE), DS_OK);
        assert_eq!(ds_release(ds8), 0);
        assert_eq!(cf_release(cf), 0);
        assert_eq!(DllCanUnloadNow(), S_FALSE);
        assert_eq!(cf_lock_server(std::ptr::null_mut(), FALSE), DS_OK);
        assert_eq!(DllCanUnloadNow(), S_OK);
    }
}

#[test]
fn enumerate_devices_and_get_device_id() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));
    backend::force_output_devices(Some(vec![
        DeviceInfo {
            id: 10,
            uid: "test-out-1".into(),
            name: "Studio Monitors".into(),
            rate: 48_000,
            channels: 2,
            io_frames: 256,
            latency_frames: 256,
            headphones: false,
            is_default: true,
        },
        DeviceInfo {
            id: 11,
            uid: "test-out-2".into(),
            name: "USB Headphones".into(),
            rate: 48_000,
            channels: 2,
            io_frames: 256,
            latency_frames: 256,
            headphones: true,
            is_default: false,
        },
    ]));
    backend::force_input_devices(Some(vec![DeviceInfo {
        id: 20,
        uid: "test-in-1".into(),
        name: "Studio Mic".into(),
        rate: 48_000,
        channels: 1,
        io_frames: 256,
        latency_frames: 256,
        headphones: false,
        is_default: true,
    }]));

    struct EnumRec {
        entries: Vec<(Option<GUID>, String, String)>,
        stop_after: usize,
    }

    unsafe extern "win64" fn enum_cb_a(
        lp_guid: *const GUID,
        desc: *const u8,
        module: *const u8,
        ctx: *mut c_void,
    ) -> BOOL {
        let rec = &mut *(ctx as *mut EnumRec);
        rec.entries.push((
            GUID::read(lp_guid),
            crate::strings::from_ansi(desc),
            crate::strings::from_ansi(module),
        ));
        if rec.entries.len() >= rec.stop_after {
            FALSE
        } else {
            TRUE
        }
    }

    unsafe extern "win64" fn enum_cb_w(
        lp_guid: *const GUID,
        desc: *const u16,
        module: *const u16,
        ctx: *mut c_void,
    ) -> BOOL {
        let rec = &mut *(ctx as *mut EnumRec);
        rec.entries.push((
            GUID::read(lp_guid),
            crate::strings::from_wide(desc),
            crate::strings::from_wide(module),
        ));
        if rec.entries.len() >= rec.stop_after {
            FALSE
        } else {
            TRUE
        }
    }

    unsafe {
        // Render ANSI enumeration (all 3 entries: Primary + 2 output devices).
        let mut rec_a = EnumRec {
            entries: Vec::new(),
            stop_after: 99,
        };
        assert_eq!(
            DirectSoundEnumerateA(
                enum_cb_a as *const c_void,
                &mut rec_a as *mut EnumRec as *mut c_void,
            ),
            DS_OK
        );
        assert_eq!(rec_a.entries.len(), 3);
        assert!(rec_a.entries[0].0.is_none());
        assert_eq!(rec_a.entries[0].1, "Primary Sound Driver");
        assert_eq!(rec_a.entries[1].1, "Studio Monitors");
        assert_eq!(rec_a.entries[1].2, "test-out-1");
        assert_eq!(rec_a.entries[2].1, "USB Headphones");

        // Render Wide enumeration with early stop after 2 entries.
        let mut rec_w = EnumRec {
            entries: Vec::new(),
            stop_after: 2,
        };
        assert_eq!(
            DirectSoundEnumerateW(
                enum_cb_w as *const c_void,
                &mut rec_w as *mut EnumRec as *mut c_void,
            ),
            DS_OK
        );
        assert_eq!(rec_w.entries.len(), 2);
        assert_eq!(rec_w.entries[1].0, rec_a.entries[1].0);

        // Capture ANSI & Wide enumeration.
        let mut cap_a = EnumRec {
            entries: Vec::new(),
            stop_after: 99,
        };
        assert_eq!(
            DirectSoundCaptureEnumerateA(
                enum_cb_a as *const c_void,
                &mut cap_a as *mut EnumRec as *mut c_void,
            ),
            DS_OK
        );
        assert_eq!(cap_a.entries.len(), 2);
        assert!(cap_a.entries[0].0.is_none());
        assert_eq!(cap_a.entries[0].1, "Primary Sound Capture Driver");
        assert_eq!(cap_a.entries[1].1, "Studio Mic");

        let mut cap_w = EnumRec {
            entries: Vec::new(),
            stop_after: 99,
        };
        assert_eq!(
            DirectSoundCaptureEnumerateW(
                enum_cb_w as *const c_void,
                &mut cap_w as *mut EnumRec as *mut c_void,
            ),
            DS_OK
        );
        assert_eq!(cap_w.entries.len(), 2);
        assert_eq!(cap_w.entries[1].0, cap_a.entries[1].0);

        // GetDeviceID resolves default playback, default capture, specific GUIDs, and rejects unknown.
        let mut out_guid = GUID_NULL;
        assert_eq!(
            GetDeviceID(
                &DSDEVID_DEFAULTPLAYBACK as *const GUID as *const u8,
                &mut out_guid as *mut GUID as *mut u8,
            ),
            DS_OK
        );
        assert_eq!(Some(out_guid), rec_a.entries[1].0);

        let mut cap_guid = GUID_NULL;
        assert_eq!(
            GetDeviceID(
                &DSDEVID_DEFAULTCAPTURE as *const GUID as *const u8,
                &mut cap_guid as *mut GUID as *mut u8,
            ),
            DS_OK
        );
        assert_eq!(Some(cap_guid), cap_a.entries[1].0);

        let second_guid = rec_a.entries[2].0.unwrap();
        let mut echo_guid = GUID_NULL;
        assert_eq!(
            GetDeviceID(
                &second_guid as *const GUID as *const u8,
                &mut echo_guid as *mut GUID as *mut u8,
            ),
            DS_OK
        );
        assert_eq!(echo_guid, second_guid);

        let unknown = GUID::new(0x1111_2222, 0x3333, 0x4444, [5; 8]);
        assert_eq!(
            GetDeviceID(
                &unknown as *const GUID as *const u8,
                &mut echo_guid as *mut GUID as *mut u8,
            ),
            DSERR_NODRIVER
        );

        // Opening DirectSound8 on an enumerated specific device GUID succeeds.
        let mut ds_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DirectSoundCreate8(
                &second_guid as *const GUID as *const u8,
                &mut ds_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        assert_eq!(ds_release(ds_raw as *mut DsDeviceCom), 0);
    }
}

#[test]
fn circular_lock_unlock_wrap_around() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();
        let wfx = pcm_wfx(1, 48_000, 16);
        let desc = make_desc(0, 64, &wfx as *const WAVEFORMATEX as *const u8);
        let mut buf_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &desc as *const DsBufferDesc as *const c_void,
                &mut buf_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let buf = buf_raw as *mut DsBufferCom;

        // Wrap-around lock: offset 56, length 24 -> 8 bytes at [56..64], 16 bytes at [0..16].
        let mut p1: *mut c_void = std::ptr::null_mut();
        let mut l1 = 0u32;
        let mut p2: *mut c_void = std::ptr::null_mut();
        let mut l2 = 0u32;
        assert_eq!(
            dsb_lock(buf, 56, 24, &mut p1, &mut l1, &mut p2, &mut l2, 0),
            DS_OK
        );
        assert_eq!(l1, 8);
        assert_eq!(l2, 16);
        assert!(!p1.is_null() && !p2.is_null());
        std::slice::from_raw_parts_mut(p1 as *mut u8, 8).fill(0xAA);
        std::slice::from_raw_parts_mut(p2 as *mut u8, 16).fill(0x55);
        assert_eq!(dsb_unlock(buf, p1, 8, p2, 16), DS_OK);

        // Unlocking an already-unlocked pointer fails cleanly.
        assert_eq!(dsb_unlock(buf, p1, 8, p2, 16), DSERR_INVALIDPARAM);

        // Entire-buffer lock verifies exact wrapped layout.
        assert_eq!(
            dsb_lock(
                buf,
                0,
                0,
                &mut p1,
                &mut l1,
                &mut p2,
                &mut l2,
                DSBLOCK_ENTIREBUFFER,
            ),
            DS_OK
        );
        assert_eq!(l1, 64);
        assert_eq!(l2, 0);
        assert!(p2.is_null());
        let all = std::slice::from_raw_parts(p1 as *const u8, 64);
        assert!(all[0..16].iter().all(|&b| b == 0x55));
        assert!(all[16..56].iter().all(|&b| b == 0x00));
        assert!(all[56..64].iter().all(|&b| b == 0xAA));
        assert_eq!(dsb_unlock(buf, p1, 64, std::ptr::null_mut(), 0), DS_OK);

        // Out-of-bounds offset/size rejected.
        assert_eq!(
            dsb_lock(buf, 64, 4, &mut p1, &mut l1, &mut p2, &mut l2, 0),
            DSERR_INVALIDPARAM
        );
        assert_eq!(
            dsb_lock(buf, 0, 68, &mut p1, &mut l1, &mut p2, &mut l2, 0),
            DSERR_INVALIDPARAM
        );

        assert_eq!(dsb_release(buf), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn playback_waveforms_pcm8_16_24_32_float_oneshot_and_looping() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();

        // Helper to test a single mono 4-frame waveform across formats.
        let verify_format = |wfx_bytes: &[u8], payload: &[u8], expected: [f32; 4], tol: f32| {
            let desc = make_desc(0, payload.len() as u32, wfx_bytes.as_ptr());
            let mut raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &desc as *const DsBufferDesc as *const c_void,
                    &mut raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let buf = raw as *mut DsBufferCom;
            write_all_bytes(buf, payload);
            assert_eq!(dsb_play(buf, 0, 0, 0), DS_OK);

            let mut status = 0u32;
            assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
            assert_eq!(status, DSBSTATUS_PLAYING);

            // Mix 6 frames: 4 frames of signal + 2 trailing silence after one-shot stops.
            let mut out = [0.0f32; 12];
            (&*ds).core.mix_frames(6, &mut out);
            for i in 0..4 {
                assert!(
                    (out[i * 2] - expected[i]).abs() <= tol,
                    "frame {i}: got {}, want {}",
                    out[i * 2],
                    expected[i]
                );
                assert!((out[i * 2 + 1] - expected[i]).abs() <= tol);
            }
            assert_eq!(&out[8..12], &[0.0; 4]);
            assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
            assert_eq!(status, 0);
            assert_eq!(dsb_release(buf), 0);
        };

        // 1. PCM 8-bit unsigned: [128 -> 0.0, 192 -> 0.5, 64 -> -0.5, 0 -> -1.0]
        let w8 = pcm_wfx(1, 48_000, 8);
        let w8_bytes = std::slice::from_raw_parts(&w8 as *const _ as *const u8, 18);
        verify_format(w8_bytes, &[128, 192, 64, 0], [0.0, 0.5, -0.5, -1.0], 1e-4);

        // 2. PCM 16-bit signed LE
        let w16 = pcm_wfx(1, 48_000, 16);
        let w16_bytes = std::slice::from_raw_parts(&w16 as *const _ as *const u8, 18);
        let p16: Vec<u8> = [0i16, 16384, -16384, -32768]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        verify_format(w16_bytes, &p16, [0.0, 0.5, -0.5, -1.0], 1e-4);

        // 3. PCM 24-bit signed packed LE
        let w24 = pcm_wfx(1, 48_000, 24);
        let w24_bytes = std::slice::from_raw_parts(&w24 as *const _ as *const u8, 18);
        let p24: Vec<u8> = [0i32, 4_194_304, -4_194_304, -8_388_608]
            .iter()
            .flat_map(|v| {
                let b = v.to_le_bytes();
                [b[0], b[1], b[2]]
            })
            .collect();
        verify_format(w24_bytes, &p24, [0.0, 0.5, -0.5, -1.0], 1e-4);

        // 4. PCM 32-bit signed LE
        let w32 = pcm_wfx(1, 48_000, 32);
        let w32_bytes = std::slice::from_raw_parts(&w32 as *const _ as *const u8, 18);
        let p32: Vec<u8> = [0i32, 1_073_741_824, -1_073_741_824, i32::MIN]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        verify_format(w32_bytes, &p32, [0.0, 0.5, -0.5, -1.0], 1e-4);

        // 5. IEEE float32
        let wf32 = float_wfx(1, 48_000);
        let wf32_bytes = std::slice::from_raw_parts(&wf32 as *const _ as *const u8, 18);
        let pf32: Vec<u8> = [0.25f32, -0.75, 0.5, -0.25]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        verify_format(wf32_bytes, &pf32, [0.25, -0.75, 0.5, -0.25], 1e-6);

        // 6. WAVE_FORMAT_EXTENSIBLE (float32) + looping wrap + Stop + cursor tracking.
        let ext = WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE,
                nChannels: 1,
                nSamplesPerSec: 48_000,
                nAvgBytesPerSec: 48_000 * 4,
                nBlockAlign: 4,
                wBitsPerSample: 32,
                cbSize: 22,
            },
            wValidBitsPerSample: 32,
            dwChannelMask: 0x4,
            SubFormat: KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
        };
        let desc = make_desc(0, pf32.len() as u32, &ext as *const _ as *const u8);
        let mut raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &desc as *const DsBufferDesc as *const c_void,
                &mut raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let loop_buf = raw as *mut DsBufferCom;
        write_all_bytes(loop_buf, &pf32);
        assert_eq!(dsb_play(loop_buf, 0, 0, DSBPLAY_LOOPING), DS_OK);

        let mut status = 0u32;
        assert_eq!(dsb_get_status(loop_buf, &mut status), DS_OK);
        assert_eq!(status, DSBSTATUS_PLAYING | DSBSTATUS_LOOPING);

        let mut loop_out = [0.0f32; 14]; // 7 frames -> wraps around 4-frame buffer
        (&*ds).core.mix_frames(7, &mut loop_out);
        let want = [0.25f32, -0.75, 0.5, -0.25, 0.25, -0.75, 0.5];
        for i in 0..7 {
            assert!((loop_out[i * 2] - want[i]).abs() < 1e-5);
        }
        let (mut play_c, mut write_c) = (99u32, 99u32);
        assert_eq!(
            dsb_get_current_position(loop_buf, &mut play_c, &mut write_c),
            DS_OK
        );
        assert_eq!(play_c, 3 * 4); // 7 % 4 == 3 frames * 4 bytes
        assert_ne!(write_c, play_c); // write cursor leads play cursor while playing

        assert_eq!(dsb_stop(loop_buf), DS_OK);
        assert_eq!(
            dsb_get_current_position(loop_buf, &mut play_c, &mut write_c),
            DS_OK
        );
        assert_eq!(play_c, 12);
        assert_eq!(write_c, 12); // stopped: write cursor == play cursor

        assert_eq!(dsb_release(loop_buf), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn volume_pan_frequency_and_primary_master_volume() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();

        // Primary buffer with CTRLVOLUME so we can verify master scaling too.
        let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRLVOLUME, 0, std::ptr::null());
        let mut prim_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &p_desc as *const DsBufferDesc as *const c_void,
                &mut prim_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let prim = prim_raw as *mut DsBufferCom;

        // Secondary mono float32 buffer at 24 kHz (half of 48 kHz mix rate).
        let wfx = float_wfx(1, 24_000);
        let samples: Vec<u8> = [0.0f32, 1.0, 0.0, 1.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let s_desc = make_desc(
            DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLPAN | DSBCAPS_CTRLFREQUENCY,
            samples.len() as u32,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let mut sec_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &s_desc as *const DsBufferDesc as *const c_void,
                &mut sec_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let sec = sec_raw as *mut DsBufferCom;
        write_all_bytes(sec, &samples);

        // Resampling at 24 kHz -> 48 kHz (step = 0.5): frames 0,1,2 -> 0.0, 0.5, 1.0.
        assert_eq!(dsb_play(sec, 0, 0, DSBPLAY_LOOPING), DS_OK);
        let mut out = [0.0f32; 6];
        (&*ds).core.mix_frames(3, &mut out);
        assert!((out[0] - 0.0).abs() < 1e-5);
        assert!((out[2] - 0.5).abs() < 1e-5);
        assert!((out[4] - 1.0).abs() < 1e-5);

        // SetFrequency(48_000) -> 1:1 stepping; SetFrequency(0) resets to 24_000.
        assert_eq!(dsb_set_frequency(sec, 48_000), DS_OK);
        let mut got_f = 0u32;
        assert_eq!(dsb_get_frequency(sec, &mut got_f), DS_OK);
        assert_eq!(got_f, 48_000);
        assert_eq!(dsb_set_frequency(sec, DSBFREQUENCY_ORIGINAL), DS_OK);
        assert_eq!(dsb_get_frequency(sec, &mut got_f), DS_OK);
        assert_eq!(got_f, 24_000);

        // Constant 1.0 buffer for exact volume & pan verification.
        let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        write_all_bytes(sec, &ones);
        assert_eq!(dsb_set_current_position(sec, 0), DS_OK);

        // -2000 mB = -20 dB = 0.1 linear; hard left pan -> L=0.1, R=0.0.
        assert_eq!(dsb_set_volume(sec, -2000), DS_OK);
        assert_eq!(dsb_set_pan(sec, DSBPAN_LEFT), DS_OK);
        let mut got_v = 0i32;
        let mut got_p = 0i32;
        assert_eq!(dsb_get_volume(sec, &mut got_v), DS_OK);
        assert_eq!(dsb_get_pan(sec, &mut got_p), DS_OK);
        assert_eq!(got_v, -2000);
        assert_eq!(got_p, DSBPAN_LEFT);

        let mut vp_out = [0.0f32; 4];
        (&*ds).core.mix_frames(2, &mut vp_out);
        assert!((vp_out[0] - 0.1).abs() < 1e-4, "L={}", vp_out[0]);
        assert!(vp_out[1].abs() < 1e-6, "R={}", vp_out[1]);

        // Hard right pan + primary master volume -600 mB (~0.5012) -> L=0.0, R≈0.05012.
        assert_eq!(dsb_set_pan(sec, DSBPAN_RIGHT), DS_OK);
        assert_eq!(dsb_set_volume(prim, -600), DS_OK);
        (&*ds).core.mix_frames(2, &mut vp_out);
        assert!(vp_out[0].abs() < 1e-6);
        let want_r = 0.1 * ds_mb_to_gain(-600);
        assert!(
            (vp_out[1] - want_r).abs() < 1e-4,
            "R={} want={}",
            vp_out[1],
            want_r
        );

        // DSBVOLUME_MIN -> exact silence.
        assert_eq!(dsb_set_volume(sec, DSBVOLUME_MIN), DS_OK);
        (&*ds).core.mix_frames(2, &mut vp_out);
        assert_eq!(vp_out, [0.0; 4]);

        assert_eq!(dsb_release(sec), 0);
        assert_eq!(dsb_release(prim), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn duplicate_sound_buffer_shares_sample_memory_with_independent_playback() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();
        let wfx = float_wfx(1, 48_000);
        let raw_init: Vec<u8> = [0.25f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        let desc = make_desc(
            DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLPAN,
            raw_init.len() as u32,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let mut orig_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &desc as *const DsBufferDesc as *const c_void,
                &mut orig_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let orig = orig_raw as *mut DsBufferCom;
        write_all_bytes(orig, &raw_init);

        let mut dup_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(ds_duplicate_sound_buffer(ds, orig_raw, &mut dup_raw), DS_OK);
        let dup = dup_raw as *mut DsBufferCom;

        // Pan orig hard left and dup hard right; both mix simultaneously!
        assert_eq!(dsb_set_pan(orig, DSBPAN_LEFT), DS_OK);
        assert_eq!(dsb_set_pan(dup, DSBPAN_RIGHT), DS_OK);
        assert_eq!(dsb_play(orig, 0, 0, DSBPLAY_LOOPING), DS_OK);
        assert_eq!(dsb_play(dup, 0, 0, DSBPLAY_LOOPING), DS_OK);

        let mut out = [0.0f32; 4];
        (&*ds).core.mix_frames(2, &mut out);
        assert!((out[0] - 0.25).abs() < 1e-5);
        assert!((out[1] - 0.25).abs() < 1e-5);

        // Mutating sample memory through `dup` is immediately visible to `orig`.
        let updated: Vec<u8> = [0.75f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        write_all_bytes(dup, &updated);
        assert_eq!(dsb_stop(dup), DS_OK);
        (&*ds).core.mix_frames(2, &mut out);
        assert!((out[0] - 0.75).abs() < 1e-5);
        assert!(out[1].abs() < 1e-6);

        // Releasing `orig` leaves `dup`'s shared sample memory alive and playable.
        assert_eq!(dsb_release(orig), 0);
        assert_eq!(dsb_play(dup, 0, 0, DSBPLAY_LOOPING), DS_OK);
        (&*ds).core.mix_frames(2, &mut out);
        assert!(out[0].abs() < 1e-6);
        assert!((out[1] - 0.75).abs() < 1e-5);

        assert_eq!(dsb_release(dup), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn three_d_attenuation_panning_cone_and_deferred_commit() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();

        // Primary buffer + IDirectSound3DListener via QueryInterface.
        let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
        let mut prim_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &p_desc as *const DsBufferDesc as *const c_void,
                &mut prim_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let prim = prim_raw as *mut DsBufferCom;
        let mut listener: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(prim, &IID_IDIRECTSOUND3DLISTENER, &mut listener),
            DS_OK
        );
        assert!(!listener.is_null());

        // Secondary 3D mono buffer + IDirectSound3DBuffer via QueryInterface.
        let wfx = float_wfx(1, 48_000);
        let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
        let s_desc = make_desc(
            DSBCAPS_CTRL3D | DSBCAPS_MUTE3DATMAXDISTANCE,
            ones.len() as u32,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let mut sec_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &s_desc as *const DsBufferDesc as *const c_void,
                &mut sec_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let sec = sec_raw as *mut DsBufferCom;
        write_all_bytes(sec, &ones);

        let mut buf3d: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(sec, &IID_IDIRECTSOUND3DBUFFER, &mut buf3d),
            DS_OK
        );
        assert!(!buf3d.is_null());

        // Configure min_distance = 2.0, max_distance = 10.0.
        assert_eq!(ds3db_set_min_distance(buf3d, 2.0, DS3D_IMMEDIATE), DS_OK);
        assert_eq!(ds3db_set_max_distance(buf3d, 10.0, DS3D_IMMEDIATE), DS_OK);
        assert_eq!(dsb_play(sec, 0, 0, DSBPLAY_LOOPING), DS_OK);

        // Hard left at distance 4.0 -> attenuation = 2.0 / 4.0 = 0.5, pan = (1.0, 0.0).
        assert_eq!(
            ds3db_set_position(buf3d, -4.0, 0.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );
        let mut out = [0.0f32; 4];
        (&*ds).core.mix_frames(2, &mut out);
        assert!((out[0] - 0.5).abs() < 1e-4, "left={}", out[0]);
        assert!(out[1].abs() < 1e-4, "right={}", out[1]);

        // Deferred move to hard right at distance 8.0 -> attenuation = 0.25, NOT active yet!
        assert_eq!(
            ds3db_set_position(buf3d, 8.0, 0.0, 0.0, DS3D_DEFERRED),
            DS_OK
        );
        (&*ds).core.mix_frames(2, &mut out);
        assert!((out[0] - 0.5).abs() < 1e-4);

        // CommitDeferredSettings applies the deferred move.
        assert_eq!(ds3dl_commit_deferred_settings(listener), DS_OK);
        (&*ds).core.mix_frames(2, &mut out);
        assert!(out[0].abs() < 1e-4, "left={}", out[0]);
        assert!((out[1] - 0.25).abs() < 1e-4, "right={}", out[1]);

        // Beyond max_distance (12.0 >= 10.0) with DSBCAPS_MUTE3DATMAXDISTANCE -> silence.
        assert_eq!(
            ds3db_set_position(buf3d, 12.0, 0.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );
        (&*ds).core.mix_frames(2, &mut out);
        assert_eq!(out, [0.0; 4]);

        // Sound cone: emitter at (0, 0, 2) pointing away (+Z) with 60/120 cone & -2000 mB outside.
        assert_eq!(
            ds3db_set_position(buf3d, 0.0, 0.0, 2.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3db_set_cone_orientation(buf3d, 0.0, 0.0, 1.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(ds3db_set_cone_angles(buf3d, 60, 120, DS3D_IMMEDIATE), DS_OK);
        assert_eq!(
            ds3db_set_cone_outside_volume(buf3d, -2000, DS3D_IMMEDIATE),
            DS_OK
        );
        (&*ds).core.mix_frames(2, &mut out);
        let expected_rear = 0.1 * std::f32::consts::FRAC_1_SQRT_2;
        assert!((out[0] - expected_rear).abs() < 1e-4, "rear L={}", out[0]);
        assert!((out[1] - expected_rear).abs() < 1e-4, "rear R={}", out[1]);

        // DS3DMODE_DISABLE bypasses 3D attenuation -> unity center (1.0, 1.0).
        assert_eq!(
            ds3db_set_mode(buf3d, DS3DMODE_DISABLE, DS3D_IMMEDIATE),
            DS_OK
        );
        (&*ds).core.mix_frames(2, &mut out);
        assert!((out[0] - 1.0).abs() < 1e-4);
        assert!((out[1] - 1.0).abs() < 1e-4);

        // IUnknown identity across sub-interfaces.
        let mut unk_a: *mut c_void = std::ptr::null_mut();
        let mut unk_b: *mut c_void = std::ptr::null_mut();
        assert_eq!(dsb_query_interface(sec, &IID_IUnknown, &mut unk_a), DS_OK);
        assert_eq!(
            ds3db_query_interface(buf3d, &IID_IUnknown, &mut unk_b),
            DS_OK
        );
        assert_eq!(unk_a, unk_b);
        dsb_release(unk_a as *mut DsBufferCom);
        dsb_release(unk_b as *mut DsBufferCom);

        assert_eq!(ds3db_release(buf3d), 1);
        assert_eq!(dsb_release(sec), 0);
        assert_eq!(ds3dl_release(listener), 1);
        assert_eq!(dsb_release(prim), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn capture_buffers_and_full_duplex_create() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let mut dsc_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DirectSoundCaptureCreate8(std::ptr::null(), &mut dsc_raw, std::ptr::null_mut()),
            DS_OK
        );
        let dsc = dsc_raw as *mut DsCaptureCom;

        let mut caps = DscCaps {
            dw_size: std::mem::size_of::<DscCaps>() as u32,
            ..DscCaps::default()
        };
        assert_eq!(dsc_get_caps(dsc, &mut caps), DS_OK);
        assert!(caps.dw_channels >= 2);
        assert_ne!(caps.dw_formats, 0);

        let wfx = pcm_wfx(2, 48_000, 16);
        let buf_bytes = 48_000 * 4 / 10; // 100 ms circular capture buffer
        let cdesc = make_cap_desc(0, buf_bytes, &wfx as *const WAVEFORMATEX as *const u8);
        let mut cb_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsc_create_capture_buffer(
                dsc,
                &cdesc as *const DscBufferDesc as *const c_void,
                &mut cb_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let cb = cb_raw as *mut DsCaptureBufferCom;

        // Wrap-around Lock/Unlock on capture buffer verifies digital silence & circularity.
        let mut p1: *mut c_void = std::ptr::null_mut();
        let mut l1 = 0u32;
        let mut p2: *mut c_void = std::ptr::null_mut();
        let mut l2 = 0u32;
        assert_eq!(
            dscb_lock(cb, buf_bytes - 16, 32, &mut p1, &mut l1, &mut p2, &mut l2, 0),
            DS_OK
        );
        assert_eq!(l1, 16);
        assert_eq!(l2, 16);
        assert!(std::slice::from_raw_parts(p1 as *const u8, 16)
            .iter()
            .all(|&b| b == 0));
        assert!(std::slice::from_raw_parts(p2 as *const u8, 16)
            .iter()
            .all(|&b| b == 0));
        assert_eq!(dscb_unlock(cb, p1, 16, p2, 16), DS_OK);

        // Start looping capture and observe cursor advance.
        assert_eq!(dscb_start(cb, DSCBSTART_LOOPING), DS_OK);
        let mut status = 0u32;
        assert_eq!(dscb_get_status(cb, &mut status), DS_OK);
        assert_eq!(status, DSCBSTATUS_CAPTURING | DSCBSTATUS_LOOPING);

        std::thread::sleep(Duration::from_millis(15));
        let (mut cap_pos, mut read_pos) = (0u32, 0u32);
        assert_eq!(
            dscb_get_current_position(cb, &mut cap_pos, &mut read_pos),
            DS_OK
        );
        assert!(cap_pos > 0, "capture cursor did not advance");

        assert_eq!(dscb_stop(cb), DS_OK);
        assert_eq!(dscb_get_status(cb, &mut status), DS_OK);
        assert_eq!(status, 0);

        assert_eq!(dscb_release(cb), 0);
        assert_eq!(dsc_release(dsc), 0);

        // DirectSoundFullDuplexCreate builds full-duplex + capture8 + render8 buffers together.
        let rdesc = make_desc(
            DSBCAPS_CTRLVOLUME,
            4096,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let (mut fd_raw, mut fd_cb, mut fd_rb) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(
            DirectSoundFullDuplexCreate(
                std::ptr::null(),
                std::ptr::null(),
                &cdesc as *const DscBufferDesc as *const c_void,
                &rdesc as *const DsBufferDesc as *const c_void,
                std::ptr::null_mut(),
                DSSCL_PRIORITY,
                &mut fd_raw,
                &mut fd_cb,
                &mut fd_rb,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        assert!(!fd_raw.is_null() && !fd_cb.is_null() && !fd_rb.is_null());

        let mut out_wfx = WAVEFORMATEX::default();
        let mut written = 0u32;
        assert_eq!(
            dsb_get_format(
                fd_rb as *mut DsBufferCom,
                &mut out_wfx as *mut WAVEFORMATEX as *mut u8,
                18,
                &mut written,
            ),
            DS_OK
        );
        assert_eq!(written, 18);
        assert_eq!({ out_wfx.nSamplesPerSec }, 48_000);

        assert_eq!(dscb_release(fd_cb as *mut DsCaptureBufferCom), 0);
        assert_eq!(dsb_release(fd_rb as *mut DsBufferCom), 0);
        assert_eq!(dsfd_release(fd_raw as *mut DsFullDuplexCom), 0);
    }
}

#[test]
fn realtime_null_backend_mixer_plays_and_completes_oneshot() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let mut ds_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DirectSoundCreate8(std::ptr::null(), &mut ds_raw, std::ptr::null_mut()),
            DS_OK
        );
        let ds = ds_raw as *mut DsDeviceCom;

        // 960 stereo frames = 20 ms at 48 kHz.
        let wfx = pcm_wfx(2, 48_000, 16);
        let pcm = vec![0u8; 960 * 4];
        let desc = make_desc(
            DSBCAPS_CTRLPOSITIONNOTIFY,
            pcm.len() as u32,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let mut buf_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &desc as *const DsBufferDesc as *const c_void,
                &mut buf_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let buf = buf_raw as *mut DsBufferCom;
        write_all_bytes(buf, &pcm);

        // Attach a DSBPN_OFFSETSTOP event notification via IDirectSoundNotify.
        let ev_handle = crate::kernel32::threads::CreateEventW(
            std::ptr::null_mut(),
            TRUE,
            FALSE,
            std::ptr::null(),
        );
        assert!(!ev_handle.is_null());
        let mut notify: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(buf, &IID_IDIRECTSOUNDNOTIFY, &mut notify),
            DS_OK
        );
        let pn = DsbPositionNotify {
            dw_offset: DSBPN_OFFSETSTOP,
            h_event_notify: ev_handle,
        };
        assert_eq!(dsn_set_notification_positions(notify, 1, &pn), DS_OK);

        // Start real-time playback on Null OutputStream background mixer thread.
        assert_eq!(dsb_play(buf, 0, 0, 0), DS_OK);

        let wait = crate::kernel32::threads::WaitForSingleObject(ev_handle, 500);
        assert_eq!(
            wait, WAIT_OBJECT_0,
            "background mixer did not complete one-shot buffer"
        );

        let mut status = 99u32;
        assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
        assert_eq!(status, 0);

        crate::kernel32::threads::CloseHandle(ev_handle);
        assert_eq!(dsn_release(notify), 1);
        assert_eq!(dsb_release(buf), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn ds7_compat_controls_format_listener_orientation_doppler_and_notify_offset() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        // DirectSoundCreate (DS7) exposes IDirectSound but NOT IDirectSound8.
        let mut ds7_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DirectSoundCreate(std::ptr::null(), &mut ds7_raw, std::ptr::null_mut()),
            DS_OK
        );
        let ds7 = ds7_raw as *mut DsDeviceCom;
        let mut qi_out: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_query_interface(ds7, &IID_IDIRECTSOUND8, &mut qi_out),
            E_NOINTERFACE
        );
        assert_eq!(
            ds_query_interface(ds7, &IID_IDIRECTSOUND, &mut qi_out),
            DS_OK
        );
        assert_eq!(ds_release(qi_out as *mut DsDeviceCom), 1);

        // Primary buffer SetFormat requires DSSCL_PRIORITY+.
        let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
        let mut prim_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds7,
                &p_desc as *const DsBufferDesc as *const c_void,
                &mut prim_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let prim = prim_raw as *mut DsBufferCom;
        let fmt44 = pcm_wfx(2, 44_100, 16);
        assert_eq!(
            dsb_set_format(prim, &fmt44 as *const WAVEFORMATEX as *const u8),
            DSERR_PRIOLEVELNEEDED
        );
        assert_eq!(
            ds_set_cooperative_level(ds7, std::ptr::null_mut(), DSSCL_PRIORITY),
            DS_OK
        );
        assert_eq!(
            dsb_set_format(prim, &fmt44 as *const WAVEFORMATEX as *const u8),
            DS_OK
        );

        // GetFormat(NULL) queries required byte size.
        let mut req_sz = 0u32;
        assert_eq!(
            dsb_get_format(prim, std::ptr::null_mut(), 0, &mut req_sz),
            DS_OK
        );
        assert_eq!(req_sz, 18);

        // Secondary buffer without CTRL flags rejects SetVolume/SetPan/SetFrequency & SetFormat.
        let wfx = float_wfx(1, 44_100);
        let s_desc = make_desc(0, 64, &wfx as *const WAVEFORMATEX as *const u8);
        let mut sec_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds7,
                &s_desc as *const DsBufferDesc as *const c_void,
                &mut sec_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let sec = sec_raw as *mut DsBufferCom;
        assert_eq!(
            dsb_query_interface(sec, &IID_IDIRECTSOUNDBUFFER8, &mut qi_out),
            E_NOINTERFACE
        );
        assert_eq!(dsb_set_volume(sec, -100), DSERR_CONTROLUNAVAIL);
        assert_eq!(dsb_set_pan(sec, 100), DSERR_CONTROLUNAVAIL);
        assert_eq!(dsb_set_frequency(sec, 22_050), DSERR_CONTROLUNAVAIL);
        assert_eq!(
            dsb_set_format(sec, &fmt44 as *const WAVEFORMATEX as *const u8),
            DSERR_INVALIDCALL
        );

        // DSBLOCK_FROMWRITECURSOR locks from the current write cursor.
        assert_eq!(dsb_set_current_position(sec, 16), DS_OK);
        let (mut p1, mut l1, mut p2, mut l2) =
            (std::ptr::null_mut(), 0u32, std::ptr::null_mut(), 0u32);
        assert_eq!(
            dsb_lock(
                sec,
                0,
                8,
                &mut p1,
                &mut l1,
                &mut p2,
                &mut l2,
                DSBLOCK_FROMWRITECURSOR
            ),
            DS_OK
        );
        assert_eq!(l1, 8);
        assert_eq!(l2, 0);
        assert_eq!(dsb_unlock(sec, p1, l1, p2, l2), DS_OK);

        assert_eq!(dsb_release(sec), 0);
        assert_eq!(dsb_release(prim), 0);
        assert_eq!(ds_release(ds7), 0);

        // 3D listener orientation, Doppler shift, HEADRELATIVE mode, and mid-buffer Notify.
        let ds = make_paused_ds8();
        let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
        let mut prim_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &p_desc as *const DsBufferDesc as *const c_void,
                &mut prim_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let prim = prim_raw as *mut DsBufferCom;
        let mut listener: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(prim, &IID_IDIRECTSOUND3DLISTENER, &mut listener),
            DS_OK
        );

        // Rotate listener to face +X (front=(1,0,0), top=(0,1,0)) -> right = cross(top, front) = (0,0,-1).
        // So an emitter at (0,0,-4) is hard RIGHT!
        assert_eq!(
            ds3dl_set_orientation(listener, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3dl_set_distance_factor(listener, 1.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3dl_set_rolloff_factor(listener, 1.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3dl_set_doppler_factor(listener, 1.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3dl_set_position(listener, 0.0, 0.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3dl_set_velocity(listener, 0.0, 0.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );

        let mut lparams = Ds3dListener {
            dw_size: std::mem::size_of::<Ds3dListener>() as u32,
            v_position: D3dVector::default(),
            v_velocity: D3dVector::default(),
            v_orient_front: D3dVector::default(),
            v_orient_top: D3dVector::default(),
            fl_distance_factor: 0.0,
            fl_rolloff_factor: 0.0,
            fl_doppler_factor: 0.0,
        };
        assert_eq!(ds3dl_get_all_parameters(listener, &mut lparams), DS_OK);
        assert!((lparams.v_orient_front.x - 1.0).abs() < 1e-5);

        // 8-frame mono ramp buffer with CTRL3D + CTRLPOSITIONNOTIFY.
        let wfx48 = float_wfx(1, 48_000);
        let ramp: Vec<u8> = [0.0f32, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let b_desc = make_desc(
            DSBCAPS_CTRL3D | DSBCAPS_CTRLPOSITIONNOTIFY,
            ramp.len() as u32,
            &wfx48 as *const WAVEFORMATEX as *const u8,
        );
        let mut b_raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &b_desc as *const DsBufferDesc as *const c_void,
                &mut b_raw,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let buf = b_raw as *mut DsBufferCom;
        write_all_bytes(buf, &ramp);

        let mut buf3d: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(buf, &IID_IDIRECTSOUND3DBUFFER, &mut buf3d),
            DS_OK
        );
        assert_eq!(ds3db_set_min_distance(buf3d, 2.0, DS3D_IMMEDIATE), DS_OK);
        assert_eq!(
            ds3db_set_position(buf3d, 0.0, 0.0, -4.0, DS3D_IMMEDIATE),
            DS_OK
        );

        // Attach offset notification at byte 16 (frame 4).
        let ev_mid = crate::kernel32::threads::CreateEventW(
            std::ptr::null_mut(),
            TRUE,
            FALSE,
            std::ptr::null(),
        );
        let mut notify: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_query_interface(buf, &IID_IDIRECTSOUNDNOTIFY, &mut notify),
            DS_OK
        );
        let pn = DsbPositionNotify {
            dw_offset: 16,
            h_event_notify: ev_mid,
        };
        assert_eq!(dsn_set_notification_positions(notify, 1, &pn), DS_OK);

        assert_eq!(dsb_play(buf, 0, 0, DSBPLAY_LOOPING), DS_OK);
        let mut out = [0.0f32; 12]; // 6 frames crosses byte 16!
        (&*ds).core.mix_frames(6, &mut out);
        // Frame 4 (ramp=0.5) at distance 4 with min_distance 2 -> 0.5 * 0.5 = 0.25 on RIGHT, 0 on LEFT!
        assert!(out[8].abs() < 1e-4, "left={}", out[8]);
        assert!((out[9] - 0.25).abs() < 1e-4, "right={}", out[9]);
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(ev_mid, 0),
            WAIT_OBJECT_0
        );

        // DS3DMODE_HEADRELATIVE ignores listener rotation: (-2, 0, 0) is hard LEFT at min_distance.
        assert_eq!(
            ds3db_set_mode(buf3d, DS3DMODE_HEADRELATIVE, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3db_set_position(buf3d, -2.0, 0.0, 0.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(dsb_set_current_position(buf, 16), DS_OK); // frame 4 -> 0.5
        let mut hr_out = [0.0f32; 2];
        (&*ds).core.mix_frames(1, &mut hr_out);
        assert!((hr_out[0] - 0.5).abs() < 1e-4, "headrel L={}", hr_out[0]);
        assert!(hr_out[1].abs() < 1e-4, "headrel R={}", hr_out[1]);

        // Approaching velocity causes Doppler speedup (> 1.0 frames advanced per frame).
        assert_eq!(
            ds3db_set_mode(buf3d, DS3DMODE_NORMAL, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3db_set_position(buf3d, 0.0, 0.0, 10.0, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(
            ds3db_set_velocity(buf3d, 0.0, 0.0, -SPEED_OF_SOUND * 0.5, DS3D_IMMEDIATE),
            DS_OK
        );
        assert_eq!(dsb_set_current_position(buf, 0), DS_OK);
        let mut dop_out = [0.0f32; 4];
        (&*ds).core.mix_frames(2, &mut dop_out);
        let (mut play_c, mut write_c) = (0u32, 0u32);
        assert_eq!(
            dsb_get_current_position(buf, &mut play_c, &mut write_c),
            DS_OK
        );
        // Speedup is 2x: 2 mixed frames advance 4 source frames (16 bytes)!
        assert_eq!(play_c, 16);

        crate::kernel32::threads::CloseHandle(ev_mid);
        assert_eq!(dsn_release(notify), 2);
        assert_eq!(ds3db_release(buf3d), 1);
        assert_eq!(dsb_release(buf), 0);
        assert_eq!(ds3dl_release(listener), 1);
        assert_eq!(dsb_release(prim), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn dsound8_set_fx_get_object_in_path_and_dsp_processing() {
    unsafe {
        let ds = make_paused_ds8();
        let wfx = float_wfx(2, 48_000);
        let mut impulse = vec![0.0f32; 1024 * 2];
        impulse[0] = 1.0;
        impulse[1] = 1.0;
        let bytes: Vec<u8> = impulse.iter().flat_map(|v| v.to_le_bytes()).collect();

        // Buffer WITHOUT DSBCAPS_CTRLFX -> DSERR_CONTROLUNAVAIL on SetFX(1).
        let no_fx_desc = make_desc(DSBCAPS_CTRLVOLUME, bytes.len() as u32, &wfx as *const WAVEFORMATEX as *const u8);
        let mut raw_nofx: *mut c_void = std::ptr::null_mut();
        assert_eq!(ds_create_sound_buffer(ds, &no_fx_desc as *const DsBufferDesc as *const c_void, &mut raw_nofx, std::ptr::null_mut()), DS_OK);
        let fx_echo = DsEffectDesc {
            dw_size: std::mem::size_of::<DsEffectDesc>() as u32,
            dw_flags: 0,
            guid_dsfx_class: GUID_DSFX_STANDARD_ECHO,
            dw_reserved1: 0,
            dw_reserved2: 0,
        };
        let mut res = 0u32;
        assert_eq!(dsb_set_fx(raw_nofx as *mut DsBufferCom, 1, &fx_echo as *const DsEffectDesc as *const c_void, &mut res), DSERR_CONTROLUNAVAIL);
        assert_eq!(dsb_release(raw_nofx as *mut DsBufferCom), 0);

        // Buffer WITH DSBCAPS_CTRLFX -> SetFX(Echo) succeeds, GetObjectInPath returns IDirectSoundFXEcho.
        let fx_desc = make_desc(DSBCAPS_CTRLFX | DSBCAPS_CTRLVOLUME, bytes.len() as u32, &wfx as *const WAVEFORMATEX as *const u8);
        let mut raw_buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(ds_create_sound_buffer(ds, &fx_desc as *const DsBufferDesc as *const c_void, &mut raw_buf, std::ptr::null_mut()), DS_OK);
        let buf = raw_buf as *mut DsBufferCom;
        write_all_bytes(buf, &bytes);

        assert_eq!(dsb_set_fx(buf, 1, &fx_echo as *const DsEffectDesc as *const c_void, &mut res), DS_OK);
        assert_eq!(res, DSFXR_LOCSOFTWARE);

        // GetObjectInPath returns IDirectSoundFXEcho with SetAllParameters/GetAllParameters.
        let mut fx_obj: *mut c_void = std::ptr::null_mut();
        assert_eq!(dsb_get_object_in_path(buf, &GUID_DSFX_STANDARD_ECHO, 0, &IID_IDIRECTSOUNDFXECHO, &mut fx_obj), DS_OK);
        assert!(!fx_obj.is_null());
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct EchoParams { wet_dry: f32, feedback: f32, left_ms: f32, right_ms: f32, pan_delay: i32 }
        let set_p = EchoParams { wet_dry: 50.0, feedback: 50.0, left_ms: 1.0, right_ms: 1.0, pan_delay: 0 };
        let vtbl = *(fx_obj as *const *const usize);
        let set_fn: unsafe extern "win64" fn(*mut c_void, *const c_void) -> HRESULT = std::mem::transmute(*vtbl.add(3));
        let get_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = std::mem::transmute(*vtbl.add(4));
        let rel_fn: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vtbl.add(2));
        assert_eq!(set_fn(fx_obj, &set_p as *const EchoParams as *const c_void), DS_OK);
        let mut got_p = EchoParams { wet_dry: 0.0, feedback: 0.0, left_ms: 0.0, right_ms: 0.0, pan_delay: 0 };
        assert_eq!(get_fn(fx_obj, &mut got_p as *mut EchoParams as *mut c_void), DS_OK);
        assert_eq!(got_p.left_ms, 1.0);
        assert_eq!(rel_fn(fx_obj), 0);

        // Play impulse and verify Echo produces a delayed repeat at 1 ms (~44 frames @ 44.1 kHz / 48 @ 48 kHz).
        let mix_rate = (&*ds).core.state.lock().unwrap().primary_format.rate;
        let expected_delay = ((0.001 * mix_rate as f32) as usize).max(1);
        assert_eq!(dsb_play(buf, 0, 0, 0), DS_OK);
        let mut out = vec![0.0f32; 128 * 2];
        (&*ds).core.mix_frames(128, &mut out);
        assert!((out[0] - 0.5).abs() < 1e-3, "dry impulse={}", out[0]);
        assert!(
            out[expected_delay * 2].abs() > 0.2,
            "expected echo tap at {expected_delay} frames (mix_rate={mix_rate}), got {}",
            out[expected_delay * 2]
        );
        assert_eq!(dsb_stop(buf), DS_OK);

        // Unknown DMO CLSID returns DSERR_FXUNAVAILABLE with DSFXR_UNKNOWN.
        let bad_fx = DsEffectDesc {
            dw_size: std::mem::size_of::<DsEffectDesc>() as u32,
            dw_flags: 0,
            guid_dsfx_class: GUID::new(0xDEAD_BEEF, 0, 0, [1; 8]),
            dw_reserved1: 0,
            dw_reserved2: 0,
        };
        assert_eq!(dsb_set_fx(buf, 1, &bad_fx as *const DsEffectDesc as *const c_void, &mut res), DSERR_FXUNAVAILABLE);
        assert_eq!(res, DSFXR_UNKNOWN);

        // SetFX(0) removes all effects cleanly.
        assert_eq!(dsb_set_fx(buf, 0, std::ptr::null(), std::ptr::null_mut()), DS_OK);
        assert_eq!(dsb_get_object_in_path(buf, &GUID_DSFX_STANDARD_ECHO, 0, &IID_IDIRECTSOUNDFXECHO, &mut fx_obj), DSERR_OBJECTNOTFOUND);

        assert_eq!(dsb_release(buf), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn dsound8_i3dl2_reverb_full_vtable_preset_and_quality() {
    let _audio_guard = crate::audio::audio_test_lock();
    backend::force_backend(Some(Backend::Null));

    unsafe {
        let ds = make_paused_ds8();
        let wfx = float_wfx(2, 48_000);
        let desc = make_desc(
            DSBCAPS_CTRLFX | DSBCAPS_CTRLVOLUME,
            1024 * 8,
            &wfx as *const WAVEFORMATEX as *const u8,
        );
        let mut raw_buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            ds_create_sound_buffer(
                ds,
                &desc as *const DsBufferDesc as *const c_void,
                &mut raw_buf,
                std::ptr::null_mut(),
            ),
            DS_OK
        );
        let buf = raw_buf as *mut DsBufferCom;

        let fx_reverb = DsEffectDesc {
            dw_size: std::mem::size_of::<DsEffectDesc>() as u32,
            dw_flags: 0,
            guid_dsfx_class: GUID_DSFX_STANDARD_I3DL2REVERB,
            dw_reserved1: 0,
            dw_reserved2: 0,
        };
        let mut res = 0u32;
        assert_eq!(
            dsb_set_fx(buf, 1, &fx_reverb as *const DsEffectDesc as *const c_void, &mut res),
            DS_OK
        );
        assert_eq!(res, DSFXR_LOCSOFTWARE);

        // GetObjectInPath with GUID_All_Objects matches installed effects; GUID_NULL does not.
        let mut null_fx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_get_object_in_path(
                buf,
                &GUID_NULL,
                0,
                &IID_IDIRECTSOUNDFXI3DL2REVERB,
                &mut null_fx,
            ),
            DSERR_OBJECTNOTFOUND
        );
        assert!(null_fx.is_null());

        let mut wildcard_fx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_get_object_in_path(
                buf,
                &GUID_ALL_OBJECTS,
                0,
                &IID_IDIRECTSOUNDFXI3DL2REVERB,
                &mut wildcard_fx,
            ),
            DS_OK
        );
        assert!(!wildcard_fx.is_null());
        let wvtbl = *(wildcard_fx as *const *const usize);
        let wrel: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*wvtbl.add(2));
        assert_eq!(wrel(wildcard_fx), 0);

        let mut fx_obj: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dsb_get_object_in_path(
                buf,
                &GUID_DSFX_STANDARD_I3DL2REVERB,
                0,
                &IID_IDIRECTSOUNDFXI3DL2REVERB,
                &mut fx_obj,
            ),
            DS_OK
        );
        assert!(!fx_obj.is_null());

        let vtbl = *(fx_obj as *const *const usize);
        let qi_fn: unsafe extern "win64" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(*vtbl.add(0));
        let rel_fn: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vtbl.add(2));
        let get_all_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT =
            std::mem::transmute(*vtbl.add(4));
        let set_preset_fn: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT =
            std::mem::transmute(*vtbl.add(5));
        let get_preset_fn: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
            std::mem::transmute(*vtbl.add(6));
        let set_quality_fn: unsafe extern "win64" fn(*mut c_void, i32) -> HRESULT =
            std::mem::transmute(*vtbl.add(7));
        let get_quality_fn: unsafe extern "win64" fn(*mut c_void, *mut i32) -> HRESULT =
            std::mem::transmute(*vtbl.add(8));

        // QI for IID_IDIRECTSOUNDFXI3DL2REVERB succeeds.
        let mut qi_out: *mut c_void = std::ptr::null_mut();
        assert_eq!(qi_fn(fx_obj, &IID_IDIRECTSOUNDFXI3DL2REVERB, &mut qi_out), DS_OK);
        assert_eq!(qi_out, fx_obj);
        assert_eq!(rel_fn(qi_out), 1);

        // Default preset (0 = DEFAULT) and default quality (2).
        let mut preset = 99u32;
        assert_eq!(get_preset_fn(fx_obj, &mut preset), DS_OK);
        assert_eq!(preset, 0);
        let mut quality = -1i32;
        assert_eq!(get_quality_fn(fx_obj, &mut quality), DS_OK);
        assert_eq!(quality, DSFX_I3DL2REVERB_QUALITY_DEFAULT);

        // SetPreset(9 = CAVE) updates both GetPreset and the 48-byte DSFXI3DL2Reverb struct.
        assert_eq!(set_preset_fn(fx_obj, 9), DS_OK);
        assert_eq!(get_preset_fn(fx_obj, &mut preset), DS_OK);
        assert_eq!(preset, 9);
        let mut raw_params = [0u8; 48];
        assert_eq!(get_all_fn(fx_obj, raw_params.as_mut_ptr() as *mut c_void), DS_OK);
        let decay_time = f32::from_le_bytes(raw_params[12..16].try_into().unwrap());
        assert!((decay_time - 2.91).abs() < 1e-4, "CAVE preset decay_time={decay_time}");

        // Out-of-range preset -> DSERR_INVALIDPARAM; preset unchanged.
        assert_eq!(set_preset_fn(fx_obj, 30), DSERR_INVALIDPARAM);
        assert_eq!(get_preset_fn(fx_obj, &mut preset), DS_OK);
        assert_eq!(preset, 9);

        // SetQuality / GetQuality bounds (0..=3).
        assert_eq!(set_quality_fn(fx_obj, 3), DS_OK);
        assert_eq!(get_quality_fn(fx_obj, &mut quality), DS_OK);
        assert_eq!(quality, 3);
        assert_eq!(set_quality_fn(fx_obj, -1), DSERR_INVALIDPARAM);
        assert_eq!(set_quality_fn(fx_obj, 4), DSERR_INVALIDPARAM);
        assert_eq!(get_quality_fn(fx_obj, &mut quality), DS_OK);
        assert_eq!(quality, 3);

        assert_eq!(rel_fn(fx_obj), 0);
        assert_eq!(dsb_release(buf), 0);
        assert_eq!(ds_release(ds), 0);
    }
}

#[test]
fn dsound8_compressor_honors_attack_release_predelay_and_sample_rate() {
    let make_comp = |attack_ms: f32, release_ms: f32, predelay_ms: f32| -> BufferEffect {
        let mut fx = BufferEffect::new(DsFxKind::Compressor);
        let mut raw = Vec::with_capacity(24);
        for f in [0.0f32, attack_ms, release_ms, -20.0, 10.0, predelay_ms] {
            raw.extend_from_slice(&f.to_le_bytes());
        }
        fx.params = raw;
        fx
    };

    // 1. Faster fAttack compresses a 1.0 step more strongly after 2 ms (96 samples @ 48 kHz).
    let mut fast_atk = make_comp(1.0, 200.0, 0.0);
    let mut slow_atk = make_comp(200.0, 200.0, 0.0);
    let mut out_fast = 0.0f32;
    let mut out_slow = 0.0f32;
    for _ in 0..96 {
        out_fast = fast_atk.process_sample(1.0, 1.0, 48_000.0).0;
        out_slow = slow_atk.process_sample(1.0, 1.0, 48_000.0).0;
    }
    assert!(
        out_fast < out_slow * 0.4,
        "fast attack ({out_fast}) should attenuate much more than slow attack ({out_slow})"
    );

    // 2. Faster fRelease recovers gain much more quickly after signal drops to 0.05 (below -20 dB).
    let mut fast_rel = make_comp(0.5, 50.0, 0.0);
    let mut slow_rel = make_comp(0.5, 2000.0, 0.0);
    for _ in 0..480 {
        fast_rel.process_sample(1.0, 1.0, 48_000.0);
        slow_rel.process_sample(1.0, 1.0, 48_000.0);
    }
    // 50 ms at 48 kHz = 2400 samples: fast release should recover well above slow release.
    let mut rel_fast = 0.0f32;
    let mut rel_slow = 0.0f32;
    for _ in 0..2400 {
        rel_fast = fast_rel.process_sample(0.05, 0.05, 48_000.0).0;
        rel_slow = slow_rel.process_sample(0.05, 0.05, 48_000.0).0;
    }
    assert!(
        rel_fast > rel_slow * 1.5,
        "fast release ({rel_fast}) should recover much faster than slow release ({rel_slow})"
    );

    // 3. Attack time constant is sample-rate independent: 10 ms at 24 kHz vs 48 kHz matches.
    let mut fx_24k = make_comp(10.0, 200.0, 0.0);
    let mut fx_48k = make_comp(10.0, 200.0, 0.0);
    let mut y_24k = 0.0f32;
    let mut y_48k = 0.0f32;
    for _ in 0..240 {
        y_24k = fx_24k.process_sample(1.0, 1.0, 24_000.0).0;
    }
    for _ in 0..480 {
        y_48k = fx_48k.process_sample(1.0, 1.0, 48_000.0).0;
    }
    assert!(
        (y_24k - y_48k).abs() < 1e-3,
        "10 ms attack should be sample-rate independent (24k={y_24k}, 48k={y_48k})"
    );

    // 4. Non-zero fPredelay delays the audio path by round(predelay_ms * sr / 1000) samples.
    let mut fx_pre = make_comp(10.0, 200.0, 1.0);
    let first = fx_pre.process_sample(0.05, 0.05, 48_000.0).0;
    assert!(first.abs() < 1e-6, "1 ms predelay should delay sample 0, got {first}");
    let mut delayed = 0.0f32;
    for _ in 0..48 {
        delayed = fx_pre.process_sample(0.0, 0.0, 48_000.0).0;
    }
    assert!(
        (delayed - 0.05).abs() < 1e-4,
        "impulse should emerge after 48 samples (1 ms @ 48 kHz), got {delayed}"
    );
}
