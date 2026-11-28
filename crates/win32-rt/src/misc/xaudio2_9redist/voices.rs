//! XAudio2 COM vtables and methods for `IXAudio2`, `IXAudio2Voice`, `IXAudio2MasteringVoice`,
//! `IXAudio2SubmixVoice`, and `IXAudio2SourceVoice`.

use super::mixer::{
    choose_output_device, decode_pcm_to_f32, default_xaudio2_matrix, parse_send_list,
    parse_wave_format, spawn_mastering_runtime, CallbackEvent, MasteringVoice, MasteringVoiceState,
    QueuedSourceBuffer, SourceVoice, SourceVoiceState, SubmixVoice, SubmixVoiceState, VoiceFilter,
    VoiceHeader, VoiceKind, VoiceSend, XAudio2Buffer, XAudio2Engine, XAudio2FilterParameters,
    XAudio2PerformanceData, XAudio2VoiceDetails, XAudio2VoiceSends, XAudio2VoiceState,
    E_INVALIDARG, E_NOINTERFACE, E_POINTER, IID_IXAUDIO2, IID_IXAUDIO2_7, S_OK,
    XAUDIO2_DEFAULT_FREQ_RATIO, XAUDIO2_END_OF_STREAM, XAUDIO2_E_INVALID_CALL,
    XAUDIO2_MAX_AUDIO_CHANNELS, XAUDIO2_MAX_FREQ_RATIO, XAUDIO2_MAX_QUEUED_BUFFERS,
    XAUDIO2_MAX_SAMPLE_RATE, XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MIN_FREQ_RATIO,
    XAUDIO2_MIN_SAMPLE_RATE, XAUDIO2_PLAY_TAILS, XAUDIO2_VOICE_NOPITCH,
    XAUDIO2_VOICE_NOSAMPLESPLAYED,
};
use super::xapo::{
    build_effect_chain, EffectBackend, VoiceEffectSlot, XAudio2EffectChain, XAPO_PARAMS_VTBL,
    XAPO_VTBL,
};
use crate::audio::com::{GUID, IID_IUnknown};
use crate::audio::format::{self, WAVEFORMATEX};
use crate::com::{com_call_slot_raw, register_static_ptr_slice, write_null_out};
use crate::strings::{rd, wr};
use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

// ============================================================================
// COM Vtables & VA_DISPATCH Registration
// ============================================================================

#[repr(C, align(16))]
pub(super) struct Vtable<const N: usize>(pub(super) [crate::Ptr; N]);
unsafe impl<const N: usize> Sync for Vtable<N> {}

pub(super) static XAUDIO2_VTBL: Vtable<13> = Vtable([
    crate::Ptr(xa2_query_interface as *const ()),              // 0 (+0x00)
    crate::Ptr(xa2_add_ref as *const ()),                      // 1 (+0x08)
    crate::Ptr(xa2_release as *const ()),                      // 2 (+0x10)
    crate::Ptr(xa2_register_callbacks as *const ()),           // 3 (+0x18)
    crate::Ptr(xa2_unregister_callbacks as *const ()),         // 4 (+0x20)
    crate::Ptr(crate::bridge_entry!(xa2_create_source_voice)), // 5 (+0x28)
    crate::Ptr(xa2_create_submix_voice as *const ()),          // 6 (+0x30)
    crate::Ptr(xa2_create_mastering_voice as *const ()),       // 7 (+0x38)
    crate::Ptr(xa2_start_engine as *const ()),                 // 8 (+0x40)
    crate::Ptr(xa2_stop_engine as *const ()),                  // 9 (+0x48)
    crate::Ptr(xa2_commit_changes as *const ()),               // 10 (+0x50)
    crate::Ptr(xa2_get_perf_data as *const ()),                // 11 (+0x58)
    crate::Ptr(xa2_set_debug_config as *const ()),             // 12 (+0x60)
]);

static MASTER_VOICE_VTBL: Vtable<20> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),            // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),            // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),             // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),                // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),               // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),             // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),        // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),        // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),        // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),        // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()), // 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()), // 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),          // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                   // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),          // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),          // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),            // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),            // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),                // 18 (+0x90)
    crate::Ptr(mv_get_channel_mask as *const ()),                // 19 (+0x98)
]);

static SUBMIX_VOICE_VTBL: Vtable<19> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),            // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),            // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),             // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),                // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),               // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),             // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),        // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),        // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),        // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),        // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()), // 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()), // 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),          // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                   // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),          // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),          // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),            // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),            // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),                // 18 (+0x90)
]);

static SOURCE_VOICE_VTBL: Vtable<29> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),            // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),            // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),             // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),                // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),               // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),             // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),        // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),        // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),        // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),        // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()), // 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()), // 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),          // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                   // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),          // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),          // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),            // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),            // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),                // 18 (+0x90)
    crate::Ptr(sv_start as *const ()),                           // 19 (+0x98)
    crate::Ptr(sv_stop as *const ()),                            // 20 (+0xa0)
    crate::Ptr(sv_submit_source_buffer as *const ()),            // 21 (+0xa8)
    crate::Ptr(sv_flush_source_buffers as *const ()),            // 22 (+0xb0)
    crate::Ptr(sv_discontinuity as *const ()),                   // 23 (+0xb8)
    crate::Ptr(sv_exit_loop as *const ()),                       // 24 (+0xc0)
    crate::Ptr(sv_get_state as *const ()),                       // 25 (+0xc8)
    crate::Ptr(crate::bridge_entry!(sv_set_freq_ratio)),         // 26 (+0xd0)
    crate::Ptr(sv_get_freq_ratio as *const ()),                  // 27 (+0xd8)
    crate::Ptr(sv_set_source_sample_rate as *const ()),          // 28 (+0xe0)
]);

pub(super) fn ensure_vtbls_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for tbl in [
            &XAUDIO2_VTBL.0[..],
            &MASTER_VOICE_VTBL.0[..],
            &SUBMIX_VOICE_VTBL.0[..],
            &SOURCE_VOICE_VTBL.0[..],
            &XAPO_VTBL.0[..],
            &XAPO_PARAMS_VTBL.0[..],
        ] {
            register_static_ptr_slice(tbl);
        }
    });
}

// ============================================================================
// IXAudio2 Methods
// ============================================================================

pub(super) unsafe extern "win64" fn xa2_query_interface(
    this: *mut XAudio2Engine,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IXAUDIO2 || iid == IID_IXAUDIO2_7 {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        return S_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn xa2_add_ref(this: *mut XAudio2Engine) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn xa2_release(this: *mut XAudio2Engine) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = (*this).refs.fetch_sub(1, Ordering::AcqRel);
    if prev == 1 {
        let engine = Box::from_raw(this);
        engine.core.shutdown_runtime();
        engine.core.shutdown_callbacks();
        drop(engine);
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn xa2_register_callbacks(this: *mut XAudio2Engine, cb: usize) -> i32 {
    if this.is_null() || cb == 0 {
        return E_POINTER;
    }
    let eng = &*this;
    let mut st = eng.core.state.lock().unwrap();
    if !st.engine_callbacks.contains(&cb) {
        st.engine_callbacks.push(cb);
    }
    S_OK
}

unsafe extern "win64" fn xa2_unregister_callbacks(this: *mut XAudio2Engine, cb: usize) {
    if this.is_null() || cb == 0 {
        return;
    }
    let eng = &*this;
    {
        let mut st = eng.core.state.lock().unwrap();
        st.engine_callbacks.retain(|&c| c != cb);
    }
    eng.core.sync_callbacks();
}

pub(super) unsafe extern "win64" fn xa2_create_source_voice(
    this: *mut XAudio2Engine,
    pp_source_voice: *mut *mut SourceVoice,
    source_format: *const WAVEFORMATEX,
    flags: u32,
    max_freq_ratio: f32,
    callback: usize,
    send_list: *const XAudio2VoiceSends,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if pp_source_voice.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_source_voice);
    if this.is_null() {
        return E_POINTER;
    }
    let fmt = match parse_wave_format(source_format) {
        Ok(f) => f,
        Err(hr) => return hr,
    };

    let max_ratio = if !max_freq_ratio.is_finite() || max_freq_ratio == 0.0 {
        XAUDIO2_DEFAULT_FREQ_RATIO
    } else if max_freq_ratio < 1.0 || max_freq_ratio > XAUDIO2_MAX_FREQ_RATIO {
        return XAUDIO2_E_INVALID_CALL;
    } else {
        max_freq_ratio
    };

    let effects = match build_effect_chain(effect_chain, fmt.channels, fmt.sample_rate, false) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let out_ch = effects
        .last()
        .map(|e| e.output_channels)
        .unwrap_or(fmt.channels);

    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        let (use_default, sends) = match parse_send_list(&core, &st, None, None, out_ch, send_list)
        {
            Ok(v) => v,
            Err(hr) => {
                drop(st);
                drop(effects);
                return hr;
            }
        };
        st.sources.insert(
            voice_id,
            SourceVoiceState {
                creation_flags: flags,
                format: fmt,
                max_freq_ratio: max_ratio,
                freq_ratio: 1.0,
                volume: 1.0,
                channel_volumes: vec![1.0; out_ch],
                filter: VoiceFilter::new(fmt.channels),
                effects,
                use_default_master_send: use_default,
                sends,
                callback,
                callback_alive: Arc::new(AtomicBool::new(true)),
                running: false,
                play_tails: false,
                discontinuity: false,
                samples_played: 0,
                samples_played_frac: 0.0,
                queue: VecDeque::with_capacity(16),
            },
        );
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: SOURCE_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Source,
        engine: core,
    });
    wr(pp_source_voice, Box::into_raw(hdr) as *mut SourceVoice);
    S_OK
}
crate::win64_bridge!(fn xa2_create_source_voice(
    *mut XAudio2Engine,
    *mut *mut SourceVoice,
    *const WAVEFORMATEX,
    u32,
    f32,
    usize,
    *const XAudio2VoiceSends,
    *const XAudio2EffectChain,
) -> i32);

pub(super) unsafe extern "win64" fn xa2_create_submix_voice(
    this: *mut XAudio2Engine,
    pp_submix_voice: *mut *mut SubmixVoice,
    input_channels: u32,
    input_sample_rate: u32,
    flags: u32,
    processing_stage: u32,
    send_list: *const XAudio2VoiceSends,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if pp_submix_voice.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_submix_voice);
    if this.is_null() {
        return E_POINTER;
    }
    if input_channels == 0 || input_channels > XAUDIO2_MAX_AUDIO_CHANNELS {
        return XAUDIO2_E_INVALID_CALL;
    }
    if !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&input_sample_rate) {
        return XAUDIO2_E_INVALID_CALL;
    }
    let ch = input_channels as usize;
    let effects = match build_effect_chain(effect_chain, ch, input_sample_rate, false) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let out_ch = effects.last().map(|e| e.output_channels).unwrap_or(ch);

    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        let (use_default, sends) = match parse_send_list(
            &core,
            &st,
            None,
            Some(processing_stage),
            out_ch,
            send_list,
        ) {
            Ok(v) => v,
            Err(hr) => {
                drop(st);
                drop(effects);
                return hr;
            }
        };
        st.submixes.insert(
            voice_id,
            SubmixVoiceState {
                creation_flags: flags,
                channels: ch,
                sample_rate: input_sample_rate,
                processing_stage,
                volume: 1.0,
                channel_volumes: vec![1.0; out_ch],
                filter: VoiceFilter::new(ch),
                effects,
                use_default_master_send: use_default,
                sends,
                mix_buf: Vec::new(),
            },
        );
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: SUBMIX_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Submix,
        engine: core,
    });
    wr(pp_submix_voice, Box::into_raw(hdr) as *mut SubmixVoice);
    S_OK
}

pub(super) unsafe extern "win64" fn xa2_create_mastering_voice(
    this: *mut XAudio2Engine,
    pp_mastering_voice: *mut *mut MasteringVoice,
    input_channels: u32,
    input_sample_rate: u32,
    flags: u32,
    device_id: *const u16,
    effect_chain: *const XAudio2EffectChain,
    _stream_category: u32,
) -> i32 {
    if pp_mastering_voice.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_mastering_voice);
    if this.is_null() {
        return E_POINTER;
    }
    if input_channels > XAUDIO2_MAX_AUDIO_CHANNELS {
        return XAUDIO2_E_INVALID_CALL;
    }
    if input_sample_rate != 0
        && !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&input_sample_rate)
    {
        return XAUDIO2_E_INVALID_CALL;
    }
    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let requested_device = (!device_id.is_null()).then(|| crate::strings::read_wide_z(device_id, 512));
    let dev = choose_output_device(requested_device.as_deref());
    let channels = if input_channels == 0 {
        crate::audio::mmdevapi::render_mix_channels(&dev) as usize
    } else {
        input_channels as usize
    };
    let sample_rate = if input_sample_rate == 0 {
        if (XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&dev.rate) {
            dev.rate
        } else {
            48_000
        }
    } else {
        input_sample_rate
    };
    let channel_mask = format::default_mask(channels.min(32) as u16);

    if core.state.lock().unwrap().master.is_some() {
        return XAUDIO2_E_INVALID_CALL;
    }

    let effects = match build_effect_chain(effect_chain, channels, sample_rate, true) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };

    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        if st.master.is_some() {
            drop(st);
            drop(effects);
            return XAUDIO2_E_INVALID_CALL;
        }
        st.master = Some(MasteringVoiceState {
            id: voice_id,
            creation_flags: flags,
            channels,
            sample_rate,
            channel_mask,
            volume: 1.0,
            channel_volumes: vec![1.0; channels],
            filter: VoiceFilter::new(channels),
            effects,
        });
    }

    match spawn_mastering_runtime(&core, &dev, sample_rate, channels) {
        Ok(rt) => {
            *core.runtime.lock().unwrap() = Some(rt);
        }
        Err(e) => {
            log::error!(target: "audio", "xaudio2: failed to initialize output stream: {e}");
            let removed = core.state.lock().unwrap().master.take();
            drop(removed);
            return XAUDIO2_E_INVALID_CALL;
        }
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: MASTER_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Mastering,
        engine: core,
    });
    wr(
        pp_mastering_voice,
        Box::into_raw(hdr) as *mut MasteringVoice,
    );
    S_OK
}

unsafe extern "win64" fn xa2_start_engine(this: *mut XAudio2Engine) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let core = &(*this).core;
    core.running.store(true, Ordering::Release);
    let ctrl_and_wake = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref().map(|r| (r.control.clone(), r.wake.clone()))
    };
    if let Some((ctrl, wake)) = ctrl_and_wake {
        if let Err(e) = ctrl.start() {
            log::warn!(target: "audio", "xaudio2: StartEngine failed to start output stream: {e}");
        }
        let _g = wake.0.lock().unwrap();
        wake.1.notify_all();
    }
    S_OK
}

pub(super) unsafe extern "win64" fn xa2_stop_engine(this: *mut XAudio2Engine) {
    if this.is_null() {
        return;
    }
    let core = &(*this).core;
    core.running.store(false, Ordering::Release);
    let ctrl = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref().map(|r| r.control.clone())
    };
    if let Some(c) = ctrl {
        c.stop();
    }
}

unsafe extern "win64" fn xa2_commit_changes(_this: *mut XAudio2Engine, _op_set: u32) -> i32 {
    S_OK
}

pub(super) unsafe extern "win64" fn xa2_get_perf_data(
    this: *mut XAudio2Engine,
    perf: *mut XAudio2PerformanceData,
) {
    if perf.is_null() {
        return;
    }
    if this.is_null() {
        wr(perf, XAudio2PerformanceData::default());
        return;
    }
    let core = &(*this).core;
    let (latency, glitches) = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref()
            .map(|r| {
                (
                    r.shared.ring.len() as u32,
                    r.shared.underrun_frames.load(Ordering::Relaxed) as u32,
                )
            })
            .unwrap_or((0, 0))
    };

    let st = core.state.lock().unwrap();
    let master_rate = st.master.as_ref().map(|m| m.sample_rate).unwrap_or(48_000);
    let total_sources = st.sources.len() as u32;
    let mut active_sources = 0u32;
    let mut active_resamplers = 0u32;
    let mut mem_bytes = 0usize;
    for s in st.sources.values() {
        for b in &s.queue {
            mem_bytes += b.samples.len() * std::mem::size_of::<f32>();
        }
        if s.running && !s.queue.is_empty() {
            active_sources += 1;
            if s.format.sample_rate != master_rate || (s.freq_ratio - 1.0).abs() > 1e-6 {
                active_resamplers += 1;
            }
        }
    }
    let active_submixes = st.submixes.len() as u32;
    wr(
        perf,
        XAudio2PerformanceData {
            audio_cycles_since_last_query: 0,
            total_cycles_since_last_query: 0,
            minimum_cycles_per_quantum: 0,
            maximum_cycles_per_quantum: 0,
            memory_usage_in_bytes: mem_bytes.min(u32::MAX as usize) as u32,
            current_latency_in_samples: latency,
            glitches_since_engine_started: glitches.saturating_add(st.source_glitches),
            active_source_voice_count: active_sources,
            total_source_voice_count: total_sources,
            active_submix_voice_count: active_submixes,
            active_resampler_count: active_resamplers,
            active_matrix_mix_count: active_sources + active_submixes,
            active_xma_source_voices: 0,
            active_xma_streams: 0,
        },
    );
}

unsafe extern "win64" fn xa2_set_debug_config(
    _this: *mut XAudio2Engine,
    _cfg: usize,
    _reserved: usize,
) {
}

// ============================================================================
// Shared IXAudio2Voice Methods (Mastering, Submix, Source)
// ============================================================================

fn find_send_mut(sends: &mut [VoiceSend], dest_id: Option<u64>) -> Option<&mut VoiceSend> {
    match dest_id {
        Some(id) => sends.iter_mut().find(|s| s.dest_id == id),
        None => {
            if sends.len() == 1 {
                Some(&mut sends[0])
            } else {
                None
            }
        }
    }
}

fn find_send(sends: &[VoiceSend], dest_id: Option<u64>) -> Option<&VoiceSend> {
    match dest_id {
        Some(id) => sends.iter().find(|s| s.dest_id == id),
        None => {
            if sends.len() == 1 {
                Some(&sends[0])
            } else {
                None
            }
        }
    }
}

unsafe fn resolve_dest_id(
    this: &VoiceHeader,
    p_dest: *mut VoiceHeader,
) -> Result<Option<u64>, i32> {
    if p_dest.is_null() {
        return Ok(None);
    }
    let dest = &*p_dest;
    if !Arc::ptr_eq(&dest.engine, &this.engine) {
        return Err(XAUDIO2_E_INVALID_CALL);
    }
    Ok(Some(dest.id))
}

pub(super) unsafe extern "win64" fn voice_get_voice_details(
    this: *mut VoiceHeader,
    details: *mut XAudio2VoiceDetails,
) {
    if details.is_null() {
        return;
    }
    if this.is_null() {
        wr(
            details,
            XAudio2VoiceDetails {
                creation_flags: 0,
                active_flags: 0,
                input_channels: 0,
                input_sample_rate: 0,
            },
        );
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let (flags, ch, sr) = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| (m.creation_flags, m.channels as u32, m.sample_rate))
            .unwrap_or((0, 2, 48_000)),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| (s.creation_flags, s.channels as u32, s.sample_rate))
            .unwrap_or((0, 2, 48_000)),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| (s.creation_flags, s.format.channels as u32, s.format.sample_rate))
            .unwrap_or((0, 2, 48_000)),
    };
    wr(
        details,
        XAudio2VoiceDetails {
            creation_flags: flags,
            active_flags: flags,
            input_channels: ch,
            input_sample_rate: sr,
        },
    );
}

pub(super) unsafe extern "win64" fn voice_set_output_voices(
    this: *mut VoiceHeader,
    send_list: *const XAudio2VoiceSends,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering {
        return XAUDIO2_E_INVALID_CALL;
    }
    let mut st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Source => {
            let Some(src_ch) = st.sources.get(&hdr.id).map(|s| s.output_channels()) else {
                return XAUDIO2_E_INVALID_CALL;
            };
            let (use_default, sends) =
                match parse_send_list(&hdr.engine, &st, Some(hdr.id), None, src_ch, send_list) {
                    Ok(v) => v,
                    Err(hr) => return hr,
                };
            if let Some(s) = st.sources.get_mut(&hdr.id) {
                s.use_default_master_send = use_default;
                s.sends = sends;
            }
            S_OK
        }
        VoiceKind::Submix => {
            let Some((src_ch, stage)) = st
                .submixes
                .get(&hdr.id)
                .map(|s| (s.output_channels(), s.processing_stage))
            else {
                return XAUDIO2_E_INVALID_CALL;
            };
            let (use_default, sends) = match parse_send_list(
                &hdr.engine,
                &st,
                Some(hdr.id),
                Some(stage),
                src_ch,
                send_list,
            ) {
                Ok(v) => v,
                Err(hr) => return hr,
            };
            if let Some(s) = st.submixes.get_mut(&hdr.id) {
                s.use_default_master_send = use_default;
                s.sends = sends;
            }
            S_OK
        }
        VoiceKind::Mastering => XAUDIO2_E_INVALID_CALL,
    }
}

fn resync_voice_output_channels(
    new_out_ch: usize,
    channel_volumes: &mut Vec<f32>,
    sends: &mut [VoiceSend],
) {
    if channel_volumes.len() != new_out_ch {
        channel_volumes.resize(new_out_ch, 1.0);
    }
    for send in sends {
        if send.matrix.len() != new_out_ch * send.dest_channels {
            send.matrix = default_xaudio2_matrix(new_out_ch, send.dest_channels);
        }
    }
}

pub(super) unsafe extern "win64" fn voice_set_effect_chain(
    this: *mut VoiceHeader,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let voice_info = {
        let st = hdr.engine.state.lock().unwrap();
        match hdr.kind {
            VoiceKind::Mastering => st
                .master
                .as_ref()
                .filter(|m| m.id == hdr.id)
                .map(|m| (m.channels, m.sample_rate, true)),
            VoiceKind::Submix => st
                .submixes
                .get(&hdr.id)
                .map(|s| (s.channels, s.sample_rate, false)),
            VoiceKind::Source => st
                .sources
                .get(&hdr.id)
                .map(|s| (s.format.channels, s.format.sample_rate, false)),
        }
    };
    let Some((in_ch, sample_rate, is_master)) = voice_info else {
        return XAUDIO2_E_INVALID_CALL;
    };

    let new_effects = match build_effect_chain(effect_chain, in_ch, sample_rate, is_master) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let new_out_ch = new_effects
        .last()
        .map(|e| e.output_channels)
        .unwrap_or(in_ch);

    let old_effects = {
        let mut st = hdr.engine.state.lock().unwrap();
        match hdr.kind {
            VoiceKind::Mastering => {
                let Some(m) = st.master.as_mut().filter(|m| m.id == hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                std::mem::replace(&mut m.effects, new_effects)
            }
            VoiceKind::Submix => {
                let Some(s) = st.submixes.get_mut(&hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                let old = std::mem::replace(&mut s.effects, new_effects);
                resync_voice_output_channels(new_out_ch, &mut s.channel_volumes, &mut s.sends);
                old
            }
            VoiceKind::Source => {
                let Some(s) = st.sources.get_mut(&hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                let old = std::mem::replace(&mut s.effects, new_effects);
                resync_voice_output_channels(new_out_ch, &mut s.channel_volumes, &mut s.sends);
                old
            }
        }
    };
    // Drop old effects strictly outside EngineCore.state lock.
    drop(old_effects);
    S_OK
}

pub(super) unsafe extern "win64" fn voice_enable_effect(
    this: *mut VoiceHeader,
    effect_index: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let mut st = hdr.engine.state.lock().unwrap();
    let slot = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get_mut(idx)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
    };
    match slot {
        Some(s) => {
            s.enabled = true;
            S_OK
        }
        None => XAUDIO2_E_INVALID_CALL,
    }
}

pub(super) unsafe extern "win64" fn voice_disable_effect(
    this: *mut VoiceHeader,
    effect_index: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let mut st = hdr.engine.state.lock().unwrap();
    let slot = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get_mut(idx)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
    };
    match slot {
        Some(s) => {
            s.enabled = false;
            S_OK
        }
        None => XAUDIO2_E_INVALID_CALL,
    }
}

pub(super) unsafe extern "win64" fn voice_get_effect_state(
    this: *mut VoiceHeader,
    effect_index: u32,
    p_enabled: *mut i32,
) {
    if p_enabled.is_null() {
        return;
    }
    if this.is_null() {
        wr(p_enabled, 0);
        return;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let st = hdr.engine.state.lock().unwrap();
    let enabled = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
    };
    wr(p_enabled, if enabled { 1 } else { 0 });
}

fn clone_voice_effect_backend(hdr: &VoiceHeader, idx: usize) -> Option<EffectBackend> {
    let st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get(idx))
            .map(|e| e.backend.clone()),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.backend.clone()),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.backend.clone()),
    }
}

pub(super) unsafe extern "win64" fn voice_set_effect_parameters(
    this: *mut VoiceHeader,
    effect_index: u32,
    params: *const c_void,
    byte_size: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || (params.is_null() && byte_size > 0) {
        return E_POINTER;
    }
    let hdr = &*this;
    let Some(backend) = clone_voice_effect_backend(hdr, effect_index as usize) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    match backend {
        EffectBackend::Builtin(att) => att.state.lock().unwrap().set_parameters(params, byte_size),
        EffectBackend::Guest(guest) => {
            if guest.params_ptr == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            let _g = guest.process_lock.lock().unwrap();
            if com_call_slot_raw(
                guest.params_ptr as *mut c_void,
                3,
                &[params as u64, byte_size as u64],
            )
            .is_none()
            {
                return XAUDIO2_E_INVALID_CALL;
            }
            S_OK
        }
    }
}

pub(super) unsafe extern "win64" fn voice_get_effect_parameters(
    this: *mut VoiceHeader,
    effect_index: u32,
    params: *mut c_void,
    byte_size: u32,
) -> i32 {
    if this.is_null() || (params.is_null() && byte_size > 0) {
        return E_POINTER;
    }
    let hdr = &*this;
    let Some(backend) = clone_voice_effect_backend(hdr, effect_index as usize) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    match backend {
        EffectBackend::Builtin(att) => att.state.lock().unwrap().get_parameters(params, byte_size),
        EffectBackend::Guest(guest) => {
            if guest.params_ptr == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            let _g = guest.process_lock.lock().unwrap();
            if com_call_slot_raw(
                guest.params_ptr as *mut c_void,
                4,
                &[params as u64, byte_size as u64],
            )
            .is_none()
            {
                return XAUDIO2_E_INVALID_CALL;
            }
            S_OK
        }
    }
}

pub(super) unsafe extern "win64" fn voice_set_filter_parameters(
    this: *mut VoiceHeader,
    p_params: *const XAudio2FilterParameters,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_params.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let params = rd(p_params);
    let mut st = hdr.engine.state.lock().unwrap();
    let res = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .map(|s| s.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .map(|s| s.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
    };
    match res {
        Ok(()) => S_OK,
        Err(hr) => hr,
    }
}

pub(super) unsafe extern "win64" fn voice_get_filter_parameters(
    this: *mut VoiceHeader,
    p_params: *mut XAudio2FilterParameters,
) {
    if p_params.is_null() {
        return;
    }
    if this.is_null() {
        wr(p_params, XAudio2FilterParameters::default());
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let params = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.filter.params)
            .unwrap_or_default(),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| s.filter.params)
            .unwrap_or_default(),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| s.filter.params)
            .unwrap_or_default(),
    };
    wr(p_params, params);
}

unsafe extern "win64" fn voice_set_output_filter_parameters(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    p_params: *const XAudio2FilterParameters,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_params.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering {
        return XAUDIO2_E_INVALID_CALL;
    }
    let dest_id = match resolve_dest_id(hdr, p_dest) {
        Ok(id) => id,
        Err(hr) => return hr,
    };
    let params = rd(p_params);
    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let send_opt = match hdr.kind {
        VoiceKind::Source => st.sources.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send_mut(&mut s.sends, dest_id)
        }),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send_mut(&mut s.sends, dest_id)
        }),
        VoiceKind::Mastering => None,
    };
    match send_opt {
        Some(send) => match send.output_filter.set(params) {
            Ok(()) => S_OK,
            Err(hr) => hr,
        },
        None => XAUDIO2_E_INVALID_CALL,
    }
}

unsafe extern "win64" fn voice_get_output_filter_parameters(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    p_params: *mut XAudio2FilterParameters,
) {
    if p_params.is_null() {
        return;
    }
    if this.is_null() {
        wr(p_params, XAudio2FilterParameters::default());
        return;
    }
    let hdr = &*this;
    let Ok(dest_id) = resolve_dest_id(hdr, p_dest) else {
        wr(p_params, XAudio2FilterParameters::default());
        return;
    };
    let st = hdr.engine.state.lock().unwrap();
    let params = match hdr.kind {
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| find_send(&s.sends, dest_id))
            .map(|s| s.output_filter.params)
            .unwrap_or_default(),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| find_send(&s.sends, dest_id))
            .map(|s| s.output_filter.params)
            .unwrap_or_default(),
        VoiceKind::Mastering => XAudio2FilterParameters::default(),
    };
    wr(p_params, params);
}

pub(super) unsafe extern "win64" fn voice_set_volume(
    this: *mut VoiceHeader,
    volume: f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !volume.is_finite() {
        return XAUDIO2_E_INVALID_CALL;
    }
    let v = volume.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL);
    let hdr = &*this;
    let mut st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Mastering => {
            if let Some(m) = st.master.as_mut().filter(|m| m.id == hdr.id) {
                m.volume = v;
            }
        }
        VoiceKind::Submix => {
            if let Some(s) = st.submixes.get_mut(&hdr.id) {
                s.volume = v;
            }
        }
        VoiceKind::Source => {
            if let Some(s) = st.sources.get_mut(&hdr.id) {
                s.volume = v;
            }
        }
    }
    S_OK
}
crate::win64_bridge!(fn voice_set_volume(*mut VoiceHeader, f32, u32) -> i32);

pub(super) unsafe extern "win64" fn voice_get_volume(this: *mut VoiceHeader, out: *mut f32) {
    if out.is_null() {
        return;
    }
    if this.is_null() {
        wr(out, 1.0);
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let v = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.volume)
            .unwrap_or(1.0),
        VoiceKind::Submix => st.submixes.get(&hdr.id).map(|s| s.volume).unwrap_or(1.0),
        VoiceKind::Source => st.sources.get(&hdr.id).map(|s| s.volume).unwrap_or(1.0),
    };
    wr(out, v);
}

pub(super) unsafe extern "win64" fn voice_set_channel_volumes(
    this: *mut VoiceHeader,
    channels: u32,
    p_volumes: *const f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_volumes.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let n = channels as usize;
    let mut vols = Vec::with_capacity(n);
    for i in 0..n {
        let v = rd(p_volumes.add(i));
        if !v.is_finite() {
            return XAUDIO2_E_INVALID_CALL;
        }
        vols.push(v.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL));
    }

    let mut st = hdr.engine.state.lock().unwrap();
    let target = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .map(|m| &mut m.channel_volumes),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).map(|s| &mut s.channel_volumes),
        VoiceKind::Source => st.sources.get_mut(&hdr.id).map(|s| &mut s.channel_volumes),
    };
    let Some(dst) = target else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if dst.len() != n {
        return XAUDIO2_E_INVALID_CALL;
    }
    dst.copy_from_slice(&vols);
    S_OK
}

pub(super) unsafe extern "win64" fn voice_get_channel_volumes(
    this: *mut VoiceHeader,
    channels: u32,
    p_volumes: *mut f32,
) {
    if this.is_null() || p_volumes.is_null() {
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let src = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.channel_volumes.as_slice()),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| s.channel_volumes.as_slice()),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| s.channel_volumes.as_slice()),
    };
    if let Some(vols) = src {
        let count = (channels as usize).min(vols.len());
        for (i, &v) in vols[..count].iter().enumerate() {
            wr(p_volumes.add(i), v);
        }
    }
}

pub(super) unsafe extern "win64" fn voice_set_output_matrix(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    src_channels: u32,
    dst_channels: u32,
    p_matrix: *const f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_matrix.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering || src_channels == 0 || dst_channels == 0 {
        return XAUDIO2_E_INVALID_CALL;
    }
    let dest_id = match resolve_dest_id(hdr, p_dest) {
        Ok(id) => id,
        Err(hr) => return hr,
    };
    let s_ch = src_channels as usize;
    let d_ch = dst_channels as usize;
    let total = match s_ch.checked_mul(d_ch) {
        Some(n) => n,
        None => return XAUDIO2_E_INVALID_CALL,
    };
    let mut mat = Vec::with_capacity(total);
    for i in 0..total {
        let v = rd(p_matrix.add(i));
        if !v.is_finite() {
            return XAUDIO2_E_INVALID_CALL;
        }
        mat.push(v.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL));
    }

    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let (voice_ch, send_opt) = match hdr.kind {
        VoiceKind::Source => match st.sources.get_mut(&hdr.id) {
            Some(s) => {
                let out_ch = s.output_channels();
                if s.use_default_master_send {
                    if let Some((mid, mch)) = master_snap {
                        if s.sends.len() != 1
                            || s.sends[0].dest_id != mid
                            || s.sends[0].dest_channels != mch
                        {
                            s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                        }
                    }
                }
                (out_ch, find_send_mut(&mut s.sends, dest_id))
            }
            None => return XAUDIO2_E_INVALID_CALL,
        },
        VoiceKind::Submix => match st.submixes.get_mut(&hdr.id) {
            Some(s) => {
                let out_ch = s.output_channels();
                if s.use_default_master_send {
                    if let Some((mid, mch)) = master_snap {
                        if s.sends.len() != 1
                            || s.sends[0].dest_id != mid
                            || s.sends[0].dest_channels != mch
                        {
                            s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                        }
                    }
                }
                (out_ch, find_send_mut(&mut s.sends, dest_id))
            }
            None => return XAUDIO2_E_INVALID_CALL,
        },
        VoiceKind::Mastering => return XAUDIO2_E_INVALID_CALL,
    };
    if voice_ch != s_ch {
        return XAUDIO2_E_INVALID_CALL;
    }
    let Some(send) = send_opt else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if send.dest_channels != d_ch {
        return XAUDIO2_E_INVALID_CALL;
    }
    send.matrix = mat;
    S_OK
}

pub(super) unsafe extern "win64" fn voice_get_output_matrix(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    src_channels: u32,
    dst_channels: u32,
    p_matrix: *mut f32,
) {
    if this.is_null() || p_matrix.is_null() || src_channels == 0 || dst_channels == 0 {
        return;
    }
    let hdr = &*this;
    let Ok(dest_id) = resolve_dest_id(hdr, p_dest) else {
        return;
    };
    let s_ch = src_channels as usize;
    let d_ch = dst_channels as usize;
    let total = s_ch.saturating_mul(d_ch);

    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let mat_opt = match hdr.kind {
        VoiceKind::Source => st.sources.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send(&s.sends, dest_id).map(|send| send.matrix.clone())
        }),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send(&s.sends, dest_id).map(|send| send.matrix.clone())
        }),
        VoiceKind::Mastering => None,
    };
    if let Some(mat) = mat_opt {
        let count = total.min(mat.len());
        for (i, &v) in mat[..count].iter().enumerate() {
            wr(p_matrix.add(i), v);
        }
    }
}

pub(super) unsafe extern "win64" fn voice_destroy_voice(this: *mut VoiceHeader) {
    if this.is_null() {
        return;
    }
    let hdr = Box::from_raw(this);
    let mut need_cb_sync = false;
    let old_effects: Vec<VoiceEffectSlot> = match hdr.kind {
        VoiceKind::Source => {
            let mut st = hdr.engine.state.lock().unwrap();
            if let Some(mut src) = st.sources.remove(&hdr.id) {
                src.callback_alive.store(false, Ordering::Release);
                need_cb_sync = src.callback != 0;
                std::mem::take(&mut src.effects)
            } else {
                Vec::new()
            }
        }
        VoiceKind::Submix => {
            let mut st = hdr.engine.state.lock().unwrap();
            st.submixes
                .remove(&hdr.id)
                .map(|mut s| std::mem::take(&mut s.effects))
                .unwrap_or_default()
        }
        VoiceKind::Mastering => {
            hdr.engine.shutdown_runtime();
            let mut st = hdr.engine.state.lock().unwrap();
            if st.master.as_ref().is_some_and(|m| m.id == hdr.id) {
                st.master
                    .take()
                    .map(|mut m| std::mem::take(&mut m.effects))
                    .unwrap_or_default()
            } else {
                Vec::new()
            }
        }
    };
    drop(old_effects);
    if need_cb_sync {
        hdr.engine.sync_callbacks();
    }
}

// ============================================================================
// IXAudio2MasteringVoice-Specific Methods
// ============================================================================

pub(super) unsafe extern "win64" fn mv_get_channel_mask(
    this: *const MasteringVoice,
    mask: *mut u32,
) -> i32 {
    if mask.is_null() {
        return E_POINTER;
    }
    let m = if this.is_null() {
        0x3
    } else {
        let hdr = &(*this).0;
        let st = hdr.engine.state.lock().unwrap();
        st.master
            .as_ref()
            .filter(|mv| mv.id == hdr.id)
            .map(|mv| mv.channel_mask)
            .unwrap_or(0x3)
    };
    wr(mask, m);
    S_OK
}

// ============================================================================
// IXAudio2SourceVoice-Specific Methods
// ============================================================================

pub(super) unsafe extern "win64" fn sv_start(
    this: *mut SourceVoice,
    _flags: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.running = true;
        src.play_tails = false;
    }
    S_OK
}

pub(super) unsafe extern "win64" fn sv_stop(
    this: *mut SourceVoice,
    flags: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.running = false;
        src.play_tails = (flags & XAUDIO2_PLAY_TAILS) != 0;
    }
    S_OK
}

pub(super) unsafe extern "win64" fn sv_submit_source_buffer(
    this: *mut SourceVoice,
    buf: *const XAudio2Buffer,
    _wma: usize,
) -> i32 {
    if this.is_null() || buf.is_null() {
        return E_POINTER;
    }
    let b = rd(buf);
    if b.audio_bytes > 0 && b.audio_data.is_null() {
        return E_INVALIDARG;
    }
    let hdr = &(*this).0;
    let fmt = {
        let st = hdr.engine.state.lock().unwrap();
        let Some(src) = st.sources.get(&hdr.id) else {
            return XAUDIO2_E_INVALID_CALL;
        };
        if src.queue.len() >= XAUDIO2_MAX_QUEUED_BUFFERS {
            return XAUDIO2_E_INVALID_CALL;
        }
        src.format
    };

    let (samples, total_frames) = if b.audio_bytes == 0 {
        (Vec::new(), 0usize)
    } else {
        let raw = std::slice::from_raw_parts(b.audio_data, b.audio_bytes as usize);
        decode_pcm_to_f32(&fmt, raw)
    };

    let play_begin = (b.play_begin as usize).min(total_frames);
    let play_end = if b.play_length == 0 {
        total_frames
    } else {
        play_begin
            .saturating_add(b.play_length as usize)
            .min(total_frames)
    };

    let (loop_begin, loop_end, loops_remaining) = if b.loop_count > 0 && total_frames > 0 {
        let lb = (b.loop_begin as usize).min(total_frames);
        let le = if b.loop_length == 0 {
            play_end
        } else {
            lb.saturating_add(b.loop_length as usize).min(total_frames)
        };
        if le > lb {
            (lb, le, b.loop_count)
        } else {
            (0, 0, 0)
        }
    } else {
        (0, 0, 0)
    };

    let queued = QueuedSourceBuffer {
        samples,
        play_begin,
        play_end,
        loop_begin,
        loop_end,
        loops_remaining,
        end_of_stream: (b.flags & XAUDIO2_END_OF_STREAM) != 0,
        context: b.context,
        started: false,
        pos: play_begin as f64,
    };

    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.discontinuity = false;
        src.queue.push_back(queued);
    }
    S_OK
}

pub(super) unsafe extern "win64" fn sv_flush_source_buffers(this: *mut SourceVoice) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut events = Vec::new();
    {
        let mut st = hdr.engine.state.lock().unwrap();
        if let Some(src) = st.sources.get_mut(&hdr.id) {
            let keep_front = src.running && src.queue.front().is_some_and(|b| b.started);
            let drain_start = if keep_front { 1 } else { 0 };
            while src.queue.len() > drain_start {
                if let Some(removed) = src.queue.remove(drain_start) {
                    if src.callback != 0 {
                        events.push(CallbackEvent::VoiceBufferEnd {
                            cb: src.callback,
                            alive: src.callback_alive.clone(),
                            ctx: removed.context,
                        });
                    }
                }
            }
        }
    }
    if !events.is_empty() {
        hdr.engine.dispatch_callbacks(events);
    }
    S_OK
}

unsafe extern "win64" fn sv_discontinuity(this: *mut SourceVoice) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.discontinuity = true;
    }
    S_OK
}

pub(super) unsafe extern "win64" fn sv_exit_loop(this: *mut SourceVoice, _op_set: u32) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        if let Some(front) = src.queue.front_mut() {
            front.loops_remaining = 0;
        }
    }
    S_OK
}

pub(super) unsafe extern "win64" fn sv_get_state(
    this: *mut SourceVoice,
    state: *mut XAudio2VoiceState,
    flags: u32,
) {
    if state.is_null() {
        return;
    }
    if this.is_null() {
        wr(
            state,
            XAudio2VoiceState {
                current_buffer_context: 0,
                buffers_queued: 0,
                samples_played: 0,
            },
        );
        return;
    }
    let hdr = &(*this).0;
    let st = hdr.engine.state.lock().unwrap();
    let (ctx, queued, played) = st
        .sources
        .get(&hdr.id)
        .map(|src| {
            let ctx = src.queue.front().map(|b| b.context).unwrap_or(0);
            let played = if (flags & XAUDIO2_VOICE_NOSAMPLESPLAYED) != 0 {
                0
            } else {
                src.samples_played
            };
            (ctx, src.queue.len() as u32, played)
        })
        .unwrap_or((0, 0, 0));
    wr(
        state,
        XAudio2VoiceState {
            current_buffer_context: ctx,
            buffers_queued: queued,
            samples_played: played,
        },
    );
}

pub(super) unsafe extern "win64" fn sv_set_freq_ratio(
    this: *mut SourceVoice,
    ratio: f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !ratio.is_finite() {
        return XAUDIO2_E_INVALID_CALL;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    let Some(src) = st.sources.get_mut(&hdr.id) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if (src.creation_flags & XAUDIO2_VOICE_NOPITCH) != 0 {
        if (ratio - 1.0).abs() > 1e-6 {
            return XAUDIO2_E_INVALID_CALL;
        }
        src.freq_ratio = 1.0;
        return S_OK;
    }
    if ratio < XAUDIO2_MIN_FREQ_RATIO || ratio > src.max_freq_ratio {
        return XAUDIO2_E_INVALID_CALL;
    }
    src.freq_ratio = ratio;
    S_OK
}
crate::win64_bridge!(fn sv_set_freq_ratio(*mut SourceVoice, f32, u32) -> i32);

pub(super) unsafe extern "win64" fn sv_get_freq_ratio(this: *const SourceVoice, out: *mut f32) {
    if out.is_null() {
        return;
    }
    let ratio = if this.is_null() {
        1.0
    } else {
        let hdr = &(*this).0;
        let st = hdr.engine.state.lock().unwrap();
        st.sources.get(&hdr.id).map(|s| s.freq_ratio).unwrap_or(1.0)
    };
    wr(out, ratio);
}

pub(super) unsafe extern "win64" fn sv_set_source_sample_rate(
    this: *mut SourceVoice,
    new_sample_rate: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&new_sample_rate) {
        return XAUDIO2_E_INVALID_CALL;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    let Some(src) = st.sources.get_mut(&hdr.id) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if !src.queue.is_empty() {
        return XAUDIO2_E_INVALID_CALL;
    }
    src.format.sample_rate = new_sample_rate;
    S_OK
}
