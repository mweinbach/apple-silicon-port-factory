use super::trace::TRACE_MAGIC;
use super::*;
use crate::engine::render::ExternalSourceDesc;
use std::sync::atomic::AtomicUsize;

/// Build a minimal valid v145 SoundBank with `BKHD` + `DIDX` + `DATA` + `HIRC`
/// containing a PCM16 `.wem` Sound, a Play Action, and an Event.
fn build_synthetic_bank(
    bank_id: u32,
    event_id: u32,
    action_id: u32,
    sound_id: u32,
    media_id: u32,
    plugin_id: u32,
    cookie: u32,
) -> Vec<u8> {
    let wem = build_pcm16_wem(48000, 1, 2048, 0.5);

    let mut bkhd_body = Vec::new();
    bkhd_body.extend_from_slice(&145u32.to_le_bytes()); // version 145
    bkhd_body.extend_from_slice(&bank_id.to_le_bytes());
    bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // language_id
    bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // alt_values
    bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // project_id
    bkhd_body.extend_from_slice(&0u32.to_le_bytes()); // bank_type
    bkhd_body.extend_from_slice(&[0u8; 16]); // hash

    let mut didx_body = Vec::new();
    if plugin_id != 0x0008_0001 {
        didx_body.extend_from_slice(&media_id.to_le_bytes());
        didx_body.extend_from_slice(&0u32.to_le_bytes()); // offset 0
        didx_body.extend_from_slice(&(wem.len() as u32).to_le_bytes());
    }

    let bus_id = 0x8888u32;

    // HIRC objects:
    // 0. Master Bus (type 0x08)
    let mut bus_body = Vec::new();
    bus_body.extend_from_slice(&bus_id.to_le_bytes());
    bus_body.extend_from_slice(&0u32.to_le_bytes()); // override_bus_id = 0 (master)
    bus_body.extend_from_slice(&0u32.to_le_bytes()); // device_shareset_id
    bus_body.push(0); // props
    bus_body.push(0); // positioning
    bus_body.push(0); // aux_bits
    bus_body.extend_from_slice(&0u32.to_le_bytes()); // reflections_aux_bus
    bus_body.push(0); // bits
    bus_body.extend_from_slice(&0u16.to_le_bytes()); // max_num_instance
    bus_body.extend_from_slice(&0x0000_3102u32.to_le_bytes()); // channel_config (stereo: n=2, kind=1, mask=0x3)
    bus_body.push(0); // hdr_bits
    bus_body.extend_from_slice(&0i32.to_le_bytes()); // recovery_time_ms
    bus_body.extend_from_slice(&0.0f32.to_le_bytes()); // max_duck_volume
    bus_body.extend_from_slice(&0u32.to_le_bytes()); // ducks count = 0
    bus_body.push(0); // fx count = 0
    bus_body.extend_from_slice(&0u32.to_le_bytes()); // mixer_fx_id
    bus_body.push(0); // mixer_is_shareset
    bus_body.push(0); // override_attachment_params
    bus_body.push(0); // metadata count = 0
    bus_body.extend_from_slice(&0u16.to_le_bytes()); // rtpcs count = 0
    bus_body.push(0); // state_props count = 0
    bus_body.push(0); // state_groups count = 0

    // 1. Sound (type 0x02)
    let mut sound_body = Vec::new();
    sound_body.extend_from_slice(&sound_id.to_le_bytes());
    sound_body.extend_from_slice(&plugin_id.to_le_bytes());
    sound_body.push(0); // stream_type = Data (in-bank)
    let src_id = if plugin_id == 0x0008_0001 { cookie } else { media_id };
    sound_body.extend_from_slice(&src_id.to_le_bytes());
    sound_body.extend_from_slice(&(wem.len() as u32).to_le_bytes()); // in_memory_media_size
    sound_body.push(0); // source_bits
    if (plugin_id & 0x0f) == 2 {
        sound_body.extend_from_slice(&0u32.to_le_bytes()); // plugin_param_size = 0
    }
    // NodeBaseParams (v145)
    sound_body.push(0); // fx override_parent
    sound_body.push(0); // num_fx
    sound_body.push(0); // metadata override_parent
    sound_body.push(0); // num_metadata
    sound_body.push(0); // override_attachment_params
    sound_body.extend_from_slice(&bus_id.to_le_bytes()); // override_bus_id
    sound_body.extend_from_slice(&0u32.to_le_bytes()); // direct_parent_id
    sound_body.push(0); // priority_bits
    sound_body.push(0); // num_props
    sound_body.push(0); // num_ranged
    sound_body.push(0); // positioning_bits
    sound_body.push(0); // aux_bits
    sound_body.extend_from_slice(&0u32.to_le_bytes()); // reflections_aux_bus
    sound_body.push(0); // adv_bits
    sound_body.push(0); // virtual_queue_behavior
    sound_body.extend_from_slice(&0u16.to_le_bytes()); // max_num_instance
    sound_body.push(0); // below_threshold_behavior
    sound_body.push(0); // hdr_bits
    sound_body.push(0); // num_state_props
    sound_body.push(0); // num_state_groups
    sound_body.extend_from_slice(&0u16.to_le_bytes()); // num_rtpc

    // 2. Action Play (type 0x03)
    let mut action_body = Vec::new();
    action_body.extend_from_slice(&action_id.to_le_bytes());
    action_body.extend_from_slice(&0x0403u16.to_le_bytes()); // Play action
    action_body.extend_from_slice(&sound_id.to_le_bytes()); // id_ext = sound_id
    action_body.push(0); // is_bus
    action_body.push(0); // num_props
    action_body.push(0); // num_ranged
    action_body.push(4); // fade_curve
    action_body.extend_from_slice(&bank_id.to_le_bytes()); // bank_id
    action_body.extend_from_slice(&0u32.to_le_bytes()); // bank_type

    // 3. Event (type 0x04)
    let mut event_body = Vec::new();
    event_body.extend_from_slice(&event_id.to_le_bytes());
    event_body.push(1); // action count = 1
    event_body.extend_from_slice(&action_id.to_le_bytes());

    let mut hirc_body = Vec::new();
    hirc_body.extend_from_slice(&4u32.to_le_bytes()); // 4 objects
    for (ty, body) in [
        (8u8, &bus_body),
        (2u8, &sound_body),
        (3u8, &action_body),
        (4u8, &event_body),
    ] {
        hirc_body.push(ty);
        hirc_body.extend_from_slice(&(body.len() as u32).to_le_bytes());
        hirc_body.extend_from_slice(body);
    }

    let mut bank = Vec::new();
    let write_chunk = |out: &mut Vec<u8>, tag: &[u8; 4], body: &[u8]| {
        out.extend_from_slice(tag);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(body);
    };
    write_chunk(&mut bank, b"BKHD", &bkhd_body);
    if !didx_body.is_empty() {
        write_chunk(&mut bank, b"DIDX", &didx_body);
        write_chunk(&mut bank, b"DATA", &wem);
    }
    write_chunk(&mut bank, b"HIRC", &hirc_body);
    bank
}

fn build_pcm16_wem(sample_rate: u32, channels: u16, frames: usize, amp: f32) -> Vec<u8> {
    let block_align = channels * 2;
    let data_len = (frames * block_align as usize) as u32;
    let mut w = Vec::with_capacity(44 + data_len as usize);
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + data_len).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&18u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes()); // PCM
    w.extend_from_slice(&channels.to_le_bytes());
    w.extend_from_slice(&sample_rate.to_le_bytes());
    w.extend_from_slice(&(sample_rate * block_align as u32).to_le_bytes());
    w.extend_from_slice(&block_align.to_le_bytes());
    w.extend_from_slice(&16u16.to_le_bytes());
    w.extend_from_slice(&0u16.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&data_len.to_le_bytes());
    let s = (amp.clamp(-1.0, 1.0) * 32767.0) as i16;
    for _ in 0..(frames * channels as usize) {
        w.extend_from_slice(&s.to_le_bytes());
    }
    w
}

#[test]
fn bank_load_post_event_and_end_of_event_callback() {
    let engine = NativeSoundEngine::default();

    let bank_bytes = build_synthetic_bank(0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x0001_0001, 0);
    let bank_id = engine.load_bank_bytes("test.bnk", bank_bytes).unwrap();
    assert_eq!(bank_id, 0x1111);

    engine.register_game_obj(100, Some("Emitter"));
    engine.set_default_listeners(&[100]);

    let duration_count = Arc::new(AtomicUsize::new(0));
    let end_count = Arc::new(AtomicUsize::new(0));
    let d_clone = duration_count.clone();
    let e_clone = end_count.clone();

    let pid = engine.post_event_rust(
        0x2222,
        100,
        AK_END_OF_EVENT | AK_DURATION | AK_ENABLE_GET_SOURCE_PLAY_POSITION,
        0xdead_beef,
        Vec::new(),
        move |ev| {
            assert_eq!(ev.cookie, 0xdead_beef);
            assert_eq!(ev.event_id, 0x2222);
            assert_eq!(ev.game_obj_id, 100);
            if ev.callback_type == AK_DURATION {
                d_clone.fetch_add(1, Ordering::SeqCst);
            } else if ev.callback_type == AK_END_OF_EVENT {
                e_clone.fetch_add(1, Ordering::SeqCst);
            }
        },
    );
    assert!(pid > 0);

    // 2048 samples = 4 frames of 512 samples.
    let mut total_energy = 0.0f64;
    for i in 0..6 {
        let samples = engine.render_audio(true).unwrap();
        total_energy += samples.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>();
        if i == 0 {
            // During playback, source play position should be queryable.
            assert!(engine.get_source_play_position(pid).is_ok());
        }
    }
    assert!(
        total_energy > 1.0,
        "expected non-silent rendered audio, got energy={total_energy}"
    );
    assert_eq!(
        duration_count.load(Ordering::SeqCst),
        1,
        "AK_Duration callback should fire once"
    );
    assert_eq!(
        end_count.load(Ordering::SeqCst),
        1,
        "AK_EndOfEvent callback should fire once"
    );

    // Unloading the bank succeeds and removes the event.
    assert!(engine.unload_bank(bank_id));
}

#[test]
fn dialogue_external_source_resolves_and_plays() {
    let engine = NativeSoundEngine::default();

    // External source plugin_id = 0x0008_0001, cookie = 0x1234_5678
    let cookie = 0x1234_5678u32;
    let bank_bytes = build_synthetic_bank(0xaaaa, 0xbbbb, 0xcccc, 0xdddd, 0, 0x0008_0001, cookie);
    engine.load_bank_bytes("dialogue.bnk", bank_bytes).unwrap();

    let ext_wem = build_pcm16_wem(48000, 1, 1024, 0.75);
    let ended = Arc::new(AtomicBool::new(false));
    let ended_clone = ended.clone();

    let pid = engine.post_event_rust(
        0xbbbb,
        1,
        AK_END_OF_EVENT,
        42,
        vec![ExternalSourceDesc {
            cookie,
            codec: 1,
            file_id: 0,
            path: None,
            bytes: Some(ext_wem),
        }],
        move |ev| {
            if ev.callback_type == AK_END_OF_EVENT {
                ended_clone.store(true, Ordering::SeqCst);
            }
        },
    );
    assert!(pid > 0);

    let samples = engine.render_frames(4).unwrap();
    let peak = samples.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
    assert!(peak > 0.1, "external source audio should play (peak={peak})");
    assert!(
        ended.load(Ordering::SeqCst),
        "external source event should fire AK_EndOfEvent"
    );
}

#[test]
fn rtpc_ramping_and_command_queue_ordering() {
    let engine = NativeSoundEngine::default();

    let rtpc_id = 0x9999u32;
    engine.set_rtpc_value(rtpc_id, 0.0, 0, 0, 4, true);
    assert_eq!(engine.get_rtpc_value(rtpc_id, 0, 0), (0.0, 1));

    // 4 frames of 512 samples at 48 kHz = 2048 samples = 42.6667 ms.
    // Use a 40 ms linear ramp (1920 samples) so frames 1, 2, 3, 4 step smoothly to 100.0.
    engine.set_rtpc_value(rtpc_id, 100.0, 0, 40, 4, false);

    engine.render_audio(true).unwrap();
    let (v1, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
    // 512 / 1920 * 100.0 = 26.6667
    assert!(
        (v1 - 26.6667).abs() < 1.0,
        "after 1 frame of 40ms ramp, expected ~26.7, got {v1}"
    );

    engine.render_audio(true).unwrap();
    let (v2, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
    // 1024 / 1920 * 100.0 = 53.3333
    assert!(
        (v2 - 53.3333).abs() < 1.0,
        "after 2 frames of 40ms ramp, expected ~53.3, got {v2}"
    );

    engine.render_frames(2).unwrap();
    let (v4, _) = engine.get_rtpc_value(rtpc_id, 0, 0);
    assert!(
        (v4 - 100.0).abs() < 1e-3,
        "after 4 frames of 40ms ramp, expected 100.0, got {v4}"
    );
}

#[test]
fn trace_replay_on_native_sound_engine() {
    let dir = std::env::temp_dir().join(format!("aw2_native_trace_test_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let bank_id = 0x11223344u32;
    let event_id = 0x55667788u32;
    let bank_bytes = build_synthetic_bank(bank_id, event_id, 0x3333, 0x4444, 0x5555, 0x0001_0001, 0);
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in &bank_bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let bank_fname = format!("bank_{bank_id:08x}_{}_{h:016x}.bnk", bank_bytes.len());
    std::fs::write(dir.join(&bank_fname), &bank_bytes).unwrap();

    // Build an AW2AKT01 trace: LoadBankMemoryCopy (50) -> RegisterGameObj (11) -> PostEvent (40) -> RenderAudio (10) x 3
    let mut trace = TRACE_MAGIC.to_vec();
    let push_rec = |buf: &mut Vec<u8>, op: u8, ret: u32, payload: &[u8]| {
        buf.push(op);
        buf.push(0); // thread
        buf.extend_from_slice(&0u32.to_le_bytes()); // frame
        buf.extend_from_slice(&ret.to_le_bytes());
        buf.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(payload);
    };

    // Op 50: LoadBankMemoryCopy (ptr u64, size u32, bank_id u32, hash u64)
    let mut p_load = Vec::new();
    p_load.extend_from_slice(&0x1000u64.to_le_bytes());
    p_load.extend_from_slice(&(bank_bytes.len() as u32).to_le_bytes());
    p_load.extend_from_slice(&bank_id.to_le_bytes());
    p_load.extend_from_slice(&h.to_le_bytes());
    push_rec(&mut trace, 50, 1, &p_load);

    // Op 11: RegisterGameObj (obj u64)
    push_rec(&mut trace, 11, 1, &100u64.to_le_bytes());

    // Op 40: PostEvent (ev u32, obj u64, flags u32, cb u64, cookie u64, pid u32, n u32)
    let mut p_post = Vec::new();
    p_post.extend_from_slice(&event_id.to_le_bytes());
    p_post.extend_from_slice(&100u64.to_le_bytes());
    p_post.extend_from_slice(&AK_END_OF_EVENT.to_le_bytes());
    p_post.extend_from_slice(&0u64.to_le_bytes());
    p_post.extend_from_slice(&0u64.to_le_bytes());
    p_post.extend_from_slice(&0u32.to_le_bytes());
    p_post.extend_from_slice(&0u32.to_le_bytes());
    push_rec(&mut trace, 40, 77, &p_post);

    // Op 10: RenderAudio x 3
    for _ in 0..3 {
        push_rec(&mut trace, 10, 1, &[1u8]);
    }

    let engine = NativeSoundEngine::default();
    let report = engine.replay_trace(&trace, &dir, 3).unwrap();
    assert_eq!(report.banks_loaded, 1);
    assert_eq!(report.events_posted, 1);
    assert_eq!(report.frames_rendered, 6);
    assert_eq!(report.callbacks_fired, 1);
    let peak = report.samples.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
    assert!(
        peak > 0.1,
        "replayed trace should produce non-silent audio (peak={peak})"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cancel_callbacks_queries_and_clear_banks() {
    let engine = NativeSoundEngine::default();
    assert_eq!(engine.current_language(), "en(US)");
    assert_eq!(engine.set_current_language("fr(FR)"), AK_SUCCESS);
    assert_eq!(engine.current_language(), "fr(FR)");

    let bank_bytes = build_synthetic_bank(0x7001, 0x7002, 0x7003, 0x7004, 0x7005, 0x0001_0001, 0);
    engine.load_bank_bytes("test_cancel.bnk", bank_bytes).unwrap();
    engine.register_game_obj(500, Some("CancelEmitter"));
    engine.set_default_listeners(&[500]);
    engine.set_state(0x100, 0x200);
    engine.set_switch(0x300, 0x400, 500);
    assert_eq!(engine.get_state(0x100), 0x200);
    assert_eq!(engine.get_switch(0x300, 500), 0x400);
    assert_eq!(engine.active_game_objects(), vec![500]);

    let cb_count = Arc::new(AtomicUsize::new(0));
    let cb1 = cb_count.clone();
    let pid1 = engine.post_event_rust(
        0x7002,
        500,
        AK_END_OF_EVENT | AK_DURATION,
        0xaaaa,
        Vec::new(),
        move |_| {
            cb1.fetch_add(1, Ordering::SeqCst);
        },
    );
    // Cancel before command queue is drained
    engine.cancel_event_callback(pid1);

    let cb2 = cb_count.clone();
    let _pid2 = engine.post_event_rust(
        0x7002,
        500,
        AK_END_OF_EVENT | AK_DURATION,
        0xbbbb,
        Vec::new(),
        move |_| {
            cb2.fetch_add(1, Ordering::SeqCst);
        },
    );
    engine.cancel_event_callback_cookie(0xbbbb);

    engine.render_frames(6).unwrap();
    assert_eq!(
        cb_count.load(Ordering::SeqCst),
        0,
        "cancelled callbacks must not fire"
    );
    assert_eq!(engine.clear_banks(), AK_SUCCESS);
}
