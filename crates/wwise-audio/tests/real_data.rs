//! Tests against the real game data. Each test skips (passes with a note) when the Alan Wake 2
//! install is not found (`$AW2_GAME_DIR` or aw2-data discovery), so CI without the game stays green.

use wwise_audio::bnk::hirc::HircBody;
use wwise_audio::bnk::Bank;
use wwise_audio::codec::decode_wem;
use wwise_audio::data::{game_dir, GameAudio};
use wwise_audio::fnv1_32_lower;
use wwise_audio::wem::Wem;

fn game() -> Option<GameAudio> {
    if game_dir().is_none() {
        eprintln!("skipping: Alan Wake 2 install not found (set AW2_GAME_DIR)");
        return None;
    }
    Some(GameAudio::open().expect("mount game VFS"))
}

fn bank(ga: &GameAudio, name: &str) -> (Vec<u8>, Bank) {
    let buf = ga.read(&format!("data_pc/audio/events/{name}")).expect("read bank");
    let b = Bank::parse(&buf).expect("parse bank");
    (buf, b)
}

#[test]
fn init_bank_parses_completely() {
    let Some(ga) = game() else { return };
    let (_, b) = bank(&ga, "Init.bnk");
    assert_eq!(b.header.version, 145, "Wwise 2022.1 bank generator version");
    assert_eq!(b.header.bank_id, fnv1_32_lower("Init"));
    assert!(b.hirc_errors.is_empty(), "{:?}", &b.hirc_errors[..b.hirc_errors.len().min(5)]);
    let g = b.global_settings.as_ref().expect("STMG");
    assert_eq!(g.max_num_voices, 110);
    assert_eq!(g.volume_threshold, -67.0);
    assert_eq!(g.state_groups.len(), 120);
    assert_eq!(g.rtpc_ramping.len(), 537);
    assert!(!b.plugins.is_empty(), "INIT plugin list");
    assert_eq!(b.custom_platform.as_deref(), Some("Windows"));
}

#[test]
fn bus_ids_are_name_hashes() {
    let Some(ga) = game() else { return };
    let (_, b) = bank(&ga, "Init.bnk");
    let o = b.object(fnv1_32_lower("Master Audio Bus")).expect("Master Audio Bus in Init.bnk");
    assert!(matches!(o.body, HircBody::Bus(_)), "type {:#x}", o.hirc_type);
    // AlanWake2.exe @0x1424cafd0 registers a bus volume callback on GetIDFromString("amb_ambisonics"),
    // but no object with that ShortID ships in any bank (checked with examples/find_id.rs) — the
    // callback is dead with the shipped data. Keep this assertion so a data change is noticed.
    assert!(b.object(fnv1_32_lower("amb_ambisonics")).is_none());
}

#[test]
fn global_bank_parses_and_media_decodes() {
    let Some(ga) = game() else { return };
    let (buf, b) = bank(&ga, "Global.bnk");
    assert_eq!(b.header.version, 145);
    assert!(b.hirc_errors.is_empty());
    // Embedded (non-prefetch) Vorbis media: 1 ch, 48 kHz, 22496 frames (matches vgmstream-cli).
    let m = b.media(&buf, 39952857).expect("DIDX entry 39952857");
    let d = decode_wem(m).expect("decode");
    assert_eq!((d.channels, d.sample_rate, d.samples.len()), (1, 48000, 22496));
    assert!(d.samples.iter().all(|s| s.is_finite()));
}

#[test]
fn prefetch_media_is_prefix_of_streamed_file() {
    let Some(ga) = game() else { return };
    let (buf, b) = bank(&ga, "Global.bnk");
    let m = b.media(&buf, 1000013314).expect("DIDX entry 1000013314");
    assert!(Wem::parse(m).unwrap().truncated, "expected a prefetch (truncated) copy");
    let full = ga.read(&GameAudio::media_path(1000013314)).unwrap();
    assert!(full.len() > m.len() && full[..m.len()] == m[..]);
}

#[test]
fn streamed_media_decodes_full_length() {
    let Some(ga) = game() else { return };
    let buf = ga.read("data_pc/audio/media/999877740.wem").unwrap();
    let w = Wem::parse(&buf).unwrap();
    assert_eq!(w.vorbis.as_ref().unwrap().num_samples, 404156);
    let d = decode_wem(&buf).unwrap();
    assert_eq!((d.channels, d.sample_rate), (2, 48000));
    assert_eq!(d.samples.len(), 404156 * 2, "frame count must equal the header's sample count");
    let peak = d.samples.iter().fold(0f32, |a, s| a.max(s.abs()));
    assert!(peak > 0.01 && peak <= 1.5, "peak {peak}");
}

#[test]
fn dialogue_external_source_cookie_exists() {
    // The exe's dialogue PostEvent wrapper (@0x1424da570) passes one AkExternalSourceInfo with
    // iExternalSrcCookie = GetIDFromString("external_source_01") and idCodec 4 (Vorbis).
    // Banks must contain External Source sources (plugin 0x00080001) using that cookie. Observed:
    // all 183 External Sources in the game data live in Global.bnk and use exactly this cookie.
    let Some(ga) = game() else { return };
    let cookie = fnv1_32_lower("external_source_01");
    let (_, b) = bank(&ga, "Global.bnk");
    let ext: Vec<_> = b.hirc.iter().flat_map(|o| o.sources()).filter(|s| s.plugin_id == 0x0008_0001).collect();
    assert_eq!(ext.len(), 183);
    assert!(ext.iter().all(|s| s.source_id == cookie), "cookie {cookie:#x}");
}

#[test]
fn native_sound_engine_multi_bank_gameplay_simulation() {
    use wwise_audio::engine::render::ExternalSourceDesc;
    use wwise_audio::runtime::{
        NativeSoundEngine, AK_DURATION, AK_ENABLE_GET_SOURCE_PLAY_POSITION, AK_END_OF_EVENT, AK_SUCCESS,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let Some(ga) = game() else { return };
    let (_, init_bnk) = bank(&ga, "Init.bnk");
    let (global_bytes, global_bnk) = bank(&ga, "Global.bnk");

    // 1. Initialize NativeSoundEngine and load shipped Init.bnk + Global.bnk.
    let engine = NativeSoundEngine::default();
    assert_eq!(engine.init(), AK_SUCCESS);
    assert_eq!(engine.music_init(), AK_SUCCESS);

    let init_id = engine.load_bank_name("Init.bnk").expect("load Init.bnk");
    let global_id = engine.load_bank_name("Global.bnk").expect("load Global.bnk");
    assert_eq!(init_id, fnv1_32_lower("Init"));
    assert_eq!(global_id, global_bnk.header.bank_id);

    let events = engine.loaded_event_ids();
    assert!(!events.is_empty(), "Global.bnk should register playable Event IDs");

    // 2. Register 3D listener and multiple game objects, set initial 3D positions & spatial params.
    const LISTENER_OBJ: u64 = 1;
    const PLAYER_OBJ: u64 = 100;
    const AMBIENT_OBJ: u64 = 200;

    assert_eq!(engine.register_game_obj(LISTENER_OBJ, Some("MainListener")), AK_SUCCESS);
    assert_eq!(engine.register_game_obj(PLAYER_OBJ, Some("SagaAnderson")), AK_SUCCESS);
    assert_eq!(engine.register_game_obj(AMBIENT_OBJ, Some("ForestEmitter")), AK_SUCCESS);
    assert_eq!(engine.set_default_listeners(&[LISTENER_OBJ]), AK_SUCCESS);
    assert_eq!(
        engine.set_position(LISTENER_OBJ, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
        AK_SUCCESS
    );
    assert_eq!(
        engine.set_position(PLAYER_OBJ, [1.5, 0.0, 2.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]),
        AK_SUCCESS
    );
    assert_eq!(
        engine.set_position(AMBIENT_OBJ, [-4.0, 1.0, 6.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        AK_SUCCESS
    );
    assert_eq!(engine.set_object_obstruction_and_occlusion(AMBIENT_OBJ, LISTENER_OBJ, 0.15, 0.10), AK_SUCCESS);

    // 3. Configure States, Switches, and RTPCs from Init.bnk's STMG table and Global.bnk's SwitchCntrs.
    let stmg = init_bnk.global_settings.as_ref().expect("Init.bnk STMG");
    if let Some(sg) = stmg.state_groups.first() {
        let state_val = sg.default_transition_time.max(1);
        assert_eq!(engine.set_state(sg.id, state_val), AK_SUCCESS);
        assert_eq!(engine.get_state(sg.id), state_val);
    }

    // Set valid Switch/State values for SwitchCntr nodes in Global.bnk so switch-routed events play.
    for obj in &global_bnk.hirc {
        if let HircBody::SwitchCntr(sc) = &obj.body {
            if let Some(&(sw_id, ref children)) = sc.switches.iter().find(|(_, ch)| !ch.is_empty()) {
                if sc.group_type == 1 {
                    engine.set_state(sc.group_id, sw_id);
                } else {
                    engine.set_switch(sc.group_id, sw_id, PLAYER_OBJ);
                    engine.set_switch(sc.group_id, sw_id, AMBIENT_OBJ);
                }
                let _ = children;
            }
        }
    }

    if let Some(rtpc) = stmg.rtpc_ramping.first() {
        assert_eq!(
            engine.set_rtpc_value(rtpc.rtpc_id, rtpc.default_value, PLAYER_OBJ, 0, 4, true),
            AK_SUCCESS
        );
        // Schedule a smooth 50ms RTPC ramp on the emitter.
        assert_eq!(
            engine.set_rtpc_value(rtpc.rtpc_id, rtpc.default_value + 10.0, PLAYER_OBJ, 50, 4, false),
            AK_SUCCESS
        );
    }

    // 4. Post shipped Play events from Global.bnk (both in-bank/streamed 3D SFX and dialogue external sources).
    let gameplay_events: &[u32] = &[
        60493057, // 3D stereo SFX container in Global.bnk (sound 8549033, media 655697706)
        70414808, // 3D multi-voice container in Global.bnk (sounds 553954901 + 383323369)
        71489451, // 3D multi-voice container with node FX in Global.bnk
        14179934, // 3D dialogue External Source event in Global.bnk
        18505755, // 3D dialogue External Source event with obstruction/occlusion RTPCs in Global.bnk
    ];
    for &ev_id in gameplay_events {
        assert!(
            events.binary_search(&ev_id).is_ok(),
            "Global.bnk should contain event {ev_id}"
        );
    }

    let embedded_wem = global_bnk
        .media(&global_bytes, 39952857)
        .expect("Global.bnk embedded media 39952857")
        .to_vec();
    let ext_cookie = fnv1_32_lower("external_source_01");
    let callbacks_received = Arc::new(AtomicUsize::new(0));

    let mut posted_pids = Vec::new();
    for (idx, &ev_id) in gameplay_events.iter().enumerate() {
        let cb_counter = callbacks_received.clone();
        let pid = engine.post_event_rust(
            ev_id,
            if idx % 2 == 0 { PLAYER_OBJ } else { AMBIENT_OBJ },
            AK_END_OF_EVENT | AK_DURATION | AK_ENABLE_GET_SOURCE_PLAY_POSITION,
            0xCAFE_0000 + idx as u64,
            vec![ExternalSourceDesc {
                cookie: ext_cookie,
                codec: 4,
                file_id: 39952857,
                path: None,
                bytes: Some(embedded_wem.clone()),
            }],
            move |_ev| {
                cb_counter.fetch_add(1, Ordering::SeqCst);
            },
        );
        assert!(pid > 0);
        posted_pids.push(pid);
    }

    // 5. Step render_audio and mix multiple 512-sample stereo buffers while moving emitters in 3D space.
    let mut stereo_buf = vec![0.0f32; 512 * 2 * 4]; // 4 frames of 512 stereo samples per batch
    let mut all_samples = Vec::with_capacity(512 * 2 * 16);

    for step in 0..4 {
        let angle = step as f64 * 0.4;
        engine.set_position(
            PLAYER_OBJ,
            [2.0 * angle.cos(), 0.0, 2.0 * angle.sin()],
            [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0],
        );
        let single_frame = engine.render_audio(true).expect("render_audio single frame");
        assert_eq!(single_frame.len(), 512 * 2);
        all_samples.extend_from_slice(&single_frame);

        let frames_mixed = engine
            .mix_stereo_interleaved(&mut stereo_buf)
            .expect("mix_stereo_interleaved batch");
        assert_eq!(frames_mixed, 512 * 4);
        all_samples.extend_from_slice(&stereo_buf);
    }

    assert_eq!(engine.frames_rendered(), 20, "expected 20 rendered 512-sample frames");
    assert!(
        all_samples.iter().all(|s| s.is_finite()),
        "all mixed stereo samples must be finite (non-NaN, non-Inf)"
    );
    let peak = all_samples.iter().fold(0.0f32, |m, &s| m.max(s.abs()));
    let energy: f64 = all_samples.iter().map(|&s| (s as f64) * (s as f64)).sum();
    eprintln!(
        "native_sound_engine_multi_bank_gameplay_simulation: gameplay_events={}, callbacks={}, peak={peak}, energy={energy}",
        gameplay_events.len(),
        callbacks_received.load(Ordering::SeqCst)
    );
    assert!(
        peak > 1e-4 && energy > 1e-3,
        "expected non-silent mixed gameplay audio from Global.bnk (peak={peak}, energy={energy})"
    );
}


#[test]
fn init_and_global_convolution_reverb_irs_parse_and_convolve() {
    use wwise_audio::engine::convolver::{IrPrecision, PreprocessedIr};
    use wwise_audio::engine::fx::{plugin_id, FxInstance};
    use wwise_audio::engine::params::{resolve_bus_chain, GameState};
    use wwise_audio::engine::project::Project;

    let Some(ga) = game() else { return };
    let (init_bytes, _) = bank(&ga, "Init.bnk");
    let (global_bytes, global_bnk) = bank(&ga, "Global.bnk");

    // 1. Parse every `00 04 02 00` preprocessed IR entry in Global.bnk's DIDX chunk.
    let mut ir_count = 0usize;
    for m in &global_bnk.media_index {
        let bytes = global_bnk.media(&global_bytes, m.id).expect("DIDX slice");
        if bytes.starts_with(&[0x00, 0x04, 0x02, 0x00]) {
            let ir = PreprocessedIr::parse(bytes)
                .unwrap_or_else(|e| panic!("IR media {} failed to parse: {e}", m.id));
            assert_eq!(ir.precision, IrPrecision::BFloat16);
            assert_eq!(ir.block_length, 1024);
            assert_eq!(ir.fft_length, 2048);
            assert_eq!(ir.sample_rate, 48_000);
            assert!(ir.num_partitions > 0);
            assert!(ir.num_channels >= 1 && ir.num_channels <= 6);
            ir_count += 1;
        }
    }
    assert!(ir_count >= 40, "expected >= 40 preprocessed IRs in Global.bnk, got {ir_count}");

    // 2. Load Init.bnk + Global.bnk into Project and run a real Convolution Reverb FX instance.
    let mut project = Project::new(None);
    project.load_bank_bytes("Init.bnk".into(), init_bytes).unwrap();
    project.load_bank_bytes("Global.bnk".into(), global_bytes).unwrap();

    let fx_id = project.banks[0]
        .bank
        .hirc
        .iter()
        .find_map(|o| match &o.body {
            HircBody::FxShareSet(fx) | HircBody::FxCustom(fx)
                if fx.plugin_id == plugin_id::CONVOLUTION_REVERB
                    && fx.media.iter().any(|(_, mid)| project.read_media_by_id(*mid).is_some()) =>
            {
                Some(o.id)
            }
            _ => None,
        })
        .expect("at least one Convolution Reverb FX referencing a Global.bnk IR");

    let mut inst = FxInstance::from_project(&project, fx_id).expect("ConvolutionReverb FxInstance");
    assert!(inst.has_convolution_ir(), "expected real partitioned FFT convolver to hydrate from Global.bnk");

    let gs = GameState::default();
    let mut tail_energy = 0.0f64;
    for f in 0..8 {
        let mut buf = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
        if f == 0 {
            buf[0][0] = 1.0;
            buf[1][0] = 1.0;
        }
        inst.process(&project, &gs, &mut buf, None);
        assert!(buf.iter().flatten().all(|x| x.is_finite()), "frame {f} produced non-finite sample");
        tail_energy += buf[0].iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>();
    }
    assert!(tail_energy > 1e-4, "real ConvolutionReverb IR should produce non-trivial tail energy, got {tail_energy}");

    // 3. Verify filtered bus 1134869537 against CAkBus::IsMixingBus (sub_1415ca5a0):
    //    - On shipped Init.bnk, bus 1134869537 sets positioning panner_type = 2 -> mix_node = true.
    //    - On patch-2d Init.bnk (the A3 test setup, which clears panner_type to 0), bus 1134869537
    //      is pass-through when bus FX are bypassed (`mix_node(false) == false`), folding its
    //      BusVolume (-3 dB) and OutputBusLPF (17) into the voice connection and eliminating the
    //      two spurious `FastPow10(0)` stages (`1 / FastPow10(0)^2` offset).
    let shipped_chain = resolve_bus_chain(&project, &gs, 1134869537).expect("shipped bus 1134869537 chain");
    assert_eq!(shipped_chain[0].id, 1134869537);
    assert!((shipped_chain[0].bus_volume_db - -3.0).abs() < 1e-4);
    assert!((shipped_chain[0].out_lpf - 17.0).abs() < 1e-4);
    assert_eq!(shipped_chain[0].panner_type, 2);
    assert!(shipped_chain[0].mix_node(false), "shipped panner_type=2 makes bus 1134869537 a mix node");

    let (patched_init, _) = wwise_audio::bnk::patch::patch_2d(&project.banks[0].bytes).expect("patch_2d Init.bnk");
    let mut patched_proj = Project::new(None);
    patched_proj.load_bank_bytes("Init.bnk".into(), patched_init).unwrap();
    let patched_chain = resolve_bus_chain(&patched_proj, &gs, 1134869537).expect("patch-2d bus 1134869537 chain");
    assert_eq!(patched_chain[0].panner_type, 0);
    assert!(!patched_chain[0].mix_node(false), "patch-2d bus 1134869537 must be pass-through when bus_fx = false");
    assert!(patched_chain[0].mix_node(true), "patch-2d bus 1134869537 must be a mix node when bus_fx = true");
}

#[test]
#[ignore = "scans all 5,009 banks; run with --release -- --ignored"]
fn all_banks_parse_without_errors() {
    let Some(ga) = game() else { return };
    let (mut ok, mut placeholders) = (0, 0);
    for f in ga.banks() {
        let buf = ga.read(&f.path).unwrap();
        if buf.len() < 16 && !buf.starts_with(b"BKHD") {
            placeholders += 1; // shipped 1-byte "0" placeholder files
            continue;
        }
        let b = Bank::parse(&buf).unwrap_or_else(|e| panic!("{}: {}", f.path, e));
        assert!(b.hirc_errors.is_empty(), "{}: {:?}", f.path, b.hirc_errors.first());
        ok += 1;
    }
    assert_eq!((ok, placeholders), (4984, 25));
}
