use super::*;
use crate::bnk::hirc::{
    prop, Action, ActionKind, ActionParams, Bus, Event, HircObject, NodeBase, Props, RangedProps, Sound, Source,
    SwitchCntr, SwitchNodeParams,
};

fn make_sound(id: u32, parent: u32, bus: u32, src_id: u32, vol_db: f32) -> HircObject {
    HircObject {
        hirc_type: 0x02,
        id,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: src_id,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: bus,
                direct_parent_id: parent,
                props: Props(vec![(prop::VOLUME, vol_db.to_bits())]),
                ..Default::default()
            },
        }),
    }
}

#[test]
fn switch_cntr_continuous_validation_and_set_switch() {
    let mut p = Project::new(None);
    // Master bus 1
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });
    // Sounds 101 (switch A), 102 (switch B), 103 (shared with bContinuePlayback)
    p.insert_hirc(make_sound(101, 100, 0, 1001, 0.0));
    p.insert_hirc(make_sound(102, 100, 0, 1002, 0.0));
    p.insert_hirc(make_sound(103, 100, 0, 1003, 0.0));
    // SwitchCntr 100: continuous validation, switch group 50, default switch 1
    p.insert_hirc(HircObject {
        hirc_type: 0x06,
        id: 100,
        offset: 0,
        size: 0,
        body: HircBody::SwitchCntr(SwitchCntr {
            node: NodeBase {
                override_bus_id: 1,
                ..Default::default()
            },
            group_type: 0,
            group_id: 50,
            default_switch: 1,
            continuous_validation: true,
            children: vec![101, 102, 103],
            switches: vec![(1, vec![101, 103]), (2, vec![102, 103])],
            params: vec![
                // 101: stop on switch (on_switch_mode = 1), bIsFirstOnly = 1 (bits = 1)
                SwitchNodeParams {
                    node_id: 101,
                    bits: 1,
                    on_switch_mode: 1,
                    fade_out_ms: 0,
                    fade_in_ms: 0,
                },
                // 102: stop on switch (on_switch_mode = 1)
                SwitchNodeParams {
                    node_id: 102,
                    bits: 0,
                    on_switch_mode: 1,
                    fade_out_ms: 0,
                    fade_in_ms: 0,
                },
                // 103: bContinuePlayback = 1 (bits = 2), on_switch_mode = 1
                SwitchNodeParams {
                    node_id: 103,
                    bits: 2,
                    on_switch_mode: 1,
                    fade_out_ms: 0,
                    fade_in_ms: 0,
                },
            ],
        }),
    });
    // Action 200: Play SwitchCntr 100
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 200,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 100,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    // Event 300
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 300,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![200] }),
    });

    let mut r = Renderer::new(&p, GameState::default(), RenderOptions::default());
    for (sid, amp) in [(1001, 0.25f32), (1002, 0.5f32), (1003, 0.125f32)] {
        r.insert_pcm(
            sid,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![amp; FRAME * 4]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
    }

    r.post_event(300, 0).unwrap();
    let mut out = vec![];
    r.render_frame(&mut out).unwrap();
    // Switch 1 active: sounds 101 and 103 started (2 voices)
    assert_eq!(r.voice_count(), 2);

    // Switch to 2: sound 101 stops, sound 103 continues without restarting, sound 102 starts
    r.set_switch(50, 2, None).unwrap();
    assert_eq!(r.voice_count(), 3);
    assert!(r.voices[0].stop_at.is_some(), "sound 101 should be stopping");
    assert!(
        r.voices[1].stop_at.is_none(),
        "sound 103 (bContinuePlayback) should keep playing"
    );
    assert_eq!(r.voices[2].sound_id, 102, "sound 102 should have started");

    // Switch back to 1: sound 101 has bIsFirstOnly=1 so it does NOT restart; 103 continues
    r.set_switch(50, 1, None).unwrap();
    assert_eq!(r.voice_count(), 3, "sound 101 must not restart due to bIsFirstOnly");
}

#[test]
fn bus_ducking_and_recovery() {
    use crate::bnk::hirc::DuckInfo;

    let mut p = Project::new(None);
    // Master bus 1
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });
    // Music bus 2 (child of Master 1), max_duck_volume = -12 dB
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 2,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            override_bus_id: 1,
            max_duck_volume: -12.0,
            ..Default::default()
        }),
    });
    // VO bus 3 (child of Master 1): ducks Music bus 2 by -6.0206 dB (~0.5x), 0ms fade, 0ms recovery
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 3,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            override_bus_id: 1,
            recovery_time_ms: 0,
            ducks: vec![DuckInfo {
                bus_id: 2,
                duck_volume: -6.0206,
                fade_out_ms: 0,
                fade_in_ms: 0,
                fade_curve: 4,
                target_prop: prop::BUS_VOLUME,
            }],
            ..Default::default()
        }),
    });

    // Sound 101 on Music bus 2 (4 frames long, amplitude 1.0)
    p.insert_hirc(make_sound(101, 0, 2, 1001, 0.0));
    // Sound 102 on VO bus 3 (1 frame long, amplitude 0.0 so we measure Music bus 2's output directly)
    p.insert_hirc(make_sound(102, 0, 3, 1002, 0.0));

    for (aid, tid) in [(201, 101), (202, 102)] {
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: aid,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: tid,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play {
                    fade_curve: 4,
                    bank_id: 0,
                    bank_type: 0,
                },
            }),
        });
    }
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 301,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![201, 202] }),
    });

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![1.0; FRAME * 3]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );
    r.insert_pcm(
        1002,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.0; FRAME]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );

    r.post_event(301, 0).unwrap();
    let mut f0_out = vec![];
    r.render_frame(&mut f0_out).unwrap();
    // Frame 0: VO bus 3 is active, so Music bus 2 is ducked by -6.0206 dB (~0.5)
    let f0_end = *f0_out.last().unwrap();
    assert!((f0_end - 0.5).abs() < 0.02, "expected ducked ~0.5, got {}", f0_end);

    // Frame 1: VO bus 3 finished at end of frame 0; Music bus 2 recovers to 1.0
    let mut f1_out = vec![];
    r.render_frame(&mut f1_out).unwrap();
    let f1_end = *f1_out.last().unwrap();
    assert!((f1_end - 1.0).abs() < 0.02, "expected recovered ~1.0, got {}", f1_end);
}

#[test]
fn aux_bus_user_and_game_sends() {
    use crate::bnk::hirc::AuxParams;

    let mut p = Project::new(None);
    // Master bus 1
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });
    // AuxBus 10 -> Master bus 1 (0 dB bus volume)
    p.insert_hirc(HircObject {
        hirc_type: 0x12,
        id: 10,
        offset: 0,
        size: 0,
        body: HircBody::AuxBus(Bus {
            override_bus_id: 1,
            ..Default::default()
        }),
    });
    // AuxBus 11 -> Master bus 1 (game aux target)
    p.insert_hirc(HircObject {
        hirc_type: 0x12,
        id: 11,
        offset: 0,
        size: 0,
        body: HircBody::AuxBus(Bus {
            override_bus_id: 1,
            ..Default::default()
        }),
    });

    // Sound 101 -> Master bus 1, with UserAuxSend0 -> AuxBus 10 at -6.0206 dB (~0.5)
    // and bUseGameAuxSends (0x20 | 0x10 | 0x08 | 0x04 = 0x3C)
    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 101,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 1,
                props: Props(vec![(prop::USER_AUX_SEND_VOLUME0, (-6.0206f32).to_bits())]),
                aux: AuxParams {
                    bits: 0x3C,
                    aux_ids: Some([10, 0, 0, 0]),
                    reflections_aux_bus: 0,
                    span: (0, 0),
                },
                ..Default::default()
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 201,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 301,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![201] }),
    });

    let mut gs = GameState::default();
    // GameAuxSend to AuxBus 11 at -12.0412 dB (~0.25)
    gs.set_game_aux_sends(vec![(11, -12.0412)]);

    let mut r = Renderer::new(
        &p,
        gs,
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.4; FRAME]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );
    r.post_event(301, 0).unwrap();
    let out = r.render().unwrap();
    // Dry (1.0) + UserAux0 (0.5) + GameAux (0.25) = 1.75 * 0.4 = 0.70
    let sample = out[FRAME / 2];
    assert!(
        (sample - 0.70).abs() < 0.02,
        "expected dry+user_aux+game_aux ~0.70, got {}",
        sample
    );
}

#[test]
fn max_num_instance_limiting_and_virtual_voices() {
    use crate::bnk::hirc::AdvSettings;

    let mut p = Project::new(None);
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });

    // Sound 101: max_num_instance = 1, bUseVirtualBehavior = 1 (bit 1 = 0x02),
    // virtual_queue_behavior = 2 (Resume), bIsVVoicesOptOverrideParent = 1 (bit 4 = 0x10)
    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 101,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 1,
                props: Props(vec![(prop::PRIORITY, 50.0f32.to_bits())]),
                adv: AdvSettings {
                    bits: 0x02 | 0x10,         // bUseVirtualBehavior | bIsVVoicesOptOverrideParent
                    virtual_queue_behavior: 2, // Resume
                    max_num_instance: 1,
                    below_threshold_behavior: 2, // SetAsVirtualVoice
                    hdr_bits: 0,
                },
                ..Default::default()
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 201,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 301,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![201] }),
    });

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.5; FRAME * 2]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );

    // Post two instances at sample 0: max_num_instance = 1 means oldest (voice 0) becomes Virtual
    // while newest (voice 1) plays physically.
    r.post_event(301, 0).unwrap();
    r.post_event(301, 0).unwrap();
    let mut f0 = vec![];
    r.render_frame(&mut f0).unwrap();
    assert_eq!(r.voices.len(), 2);
    assert_eq!(
        r.voices[0].state,
        VState::Virtual,
        "oldest voice should be virtualized by max_num_instance=1"
    );
    assert_eq!(r.voices[1].state, VState::Playing, "newest voice should play physically");
    // Output amplitude should be 0.5 (single voice playing), not 1.0 (two voices)
    assert!((f0[FRAME - 1] - 0.5).abs() < 0.02);

    // Render frame 1: voice 1 finishes at end of frame 1 (2 frames long)
    let mut f1 = vec![];
    r.render_frame(&mut f1).unwrap();
    assert_eq!(r.voices[1].state, VState::Done);

    // Render frame 2: slot is now free, so virtual voice 0 wakes up with Resume behavior and plays!
    let mut f2 = vec![];
    r.render_frame(&mut f2).unwrap();
    assert_eq!(
        r.voices[0].state,
        VState::Playing,
        "virtual voice should wake up when slot frees"
    );
    assert!((f2[FRAME - 1] - 0.5).abs() < 0.02);
}

#[test]
fn below_threshold_virtual_voice_behaviors() {
    use crate::bnk::hirc::AdvSettings;

    let mut p = Project::new(None);
    p.volume_threshold_db = -20.0;
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });

    // Sound 101: volume = -30 dB (below -20 dB threshold), SetAsVirtualVoice (2), FromElapsedTime (1)
    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 101,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 1,
                props: Props(vec![(prop::VOLUME, (-30.0f32).to_bits())]),
                adv: AdvSettings {
                    bits: 0x10,                // bIsVVoicesOptOverrideParent
                    virtual_queue_behavior: 1, // FromElapsedTime
                    max_num_instance: 0,
                    below_threshold_behavior: 2, // SetAsVirtualVoice
                    hdr_bits: 0,
                },
                ..Default::default()
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 201,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 301,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![201] }),
    });

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    // Frame 0 has amplitude 0.25, Frame 1 has amplitude 0.75
    let mut samples = vec![0.25f32; FRAME];
    samples.extend(vec![0.75f32; FRAME]);
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![samples],
            loop_points: None,
            akd_gain: 1.0,
        },
    );

    r.post_event(301, 0).unwrap();
    let mut f0 = vec![];
    r.render_frame(&mut f0).unwrap();
    // Frame 0: voice is below threshold (-30 dB < -20 dB), so it is Virtual and produces silence,
    // while its cursor advances through Frame 0 (FromElapsedTime).
    assert_eq!(r.voices[0].state, VState::Virtual);
    assert!(f0.iter().all(|&x| x == 0.0), "virtual voice must not mix into output");
    assert_eq!(
        r.voices[0].seg_pos, FRAME,
        "FromElapsedTime must advance cursor while virtual"
    );

    // Raise voice volume above threshold for Frame 1
    r.voices[0].params.volume_db = 0.0;
    r.voices[0].base_lin = 1.0;
    let mut f1 = vec![];
    r.render_frame(&mut f1).unwrap();
    assert_eq!(r.voices[0].state, VState::Done, "voice finishes at end of Frame 1");
    assert!(
        (f1[FRAME - 1] - 0.75).abs() < 0.02,
        "woken FromElapsedTime voice should play Frame 1 samples (0.75)"
    );
}

#[test]
fn music_ranseq_chaining_pre_entry_and_post_exit_overlap() {
    use crate::bnk::hirc::{
        MeterInfo, MusicMarker, MusicNode, MusicPlaylistItem, MusicRanSeqCntr, MusicSegment, MusicTrack,
        MusicTransNode, TrackSrcInfo,
    };

    let mut p = Project::new(None);
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });

    // Segment 1 (id 10): duration 30 ms (1440 samples), Entry at 0 ms, Exit at 20 ms (960 samples).
    // Post-exit tail is [20..30 ms] = [960..1440 samples].
    // Track 11 has source 1001 (amplitude 0.4).
    p.insert_hirc(HircObject {
        hirc_type: 0x0b,
        id: 11,
        offset: 0,
        size: 0,
        body: HircBody::MusicTrack(MusicTrack {
            flags: 0,
            sources: vec![Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            }],
            playlist: vec![TrackSrcInfo {
                track_id: 0,
                source_id: 1001,
                event_id: 0,
                play_at: 0.0,
                begin_trim_offset: 0.0,
                end_trim_offset: 0.0,
                src_duration: 30.0,
            }],
            num_sub_track: 1,
            clip_automation: vec![],
            node: NodeBase {
                override_bus_id: 1,
                direct_parent_id: 10,
                ..Default::default()
            },
            track_type: 0,
            switch: None,
            look_ahead_time: 0,
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x0a,
        id: 10,
        offset: 0,
        size: 0,
        body: HircBody::MusicSegment(MusicSegment {
            music: MusicNode {
                flags: 0,
                node: NodeBase {
                    override_bus_id: 1,
                    direct_parent_id: 100,
                    ..Default::default()
                },
                children: vec![11],
                meter: MeterInfo::default(),
                stingers: vec![],
            },
            duration_ms: 30.0,
            markers: vec![
                MusicMarker {
                    id: 43_573_010,
                    position_ms: 0.0,
                    name: String::new(),
                },
                MusicMarker {
                    id: 1_539_036_744,
                    position_ms: 20.0,
                    name: String::new(),
                },
            ],
        }),
    });

    // Segment 2 (id 20): duration 30 ms (1440 samples), Entry at 10 ms (480 samples), Exit at 30 ms.
    // Pre-entry is [0..10 ms] (480 samples). When chained at Segment 1's Exit (960 samples),
    // Segment 2's Entry lands at 960, so its Pre-entry starts at 960 - 480 = 480 samples,
    // and its post-entry overlaps Segment 1's post-exit across [960..1440 samples]!
    // Track 21 has source 1002 (amplitude 0.3).
    p.insert_hirc(HircObject {
        hirc_type: 0x0b,
        id: 21,
        offset: 0,
        size: 0,
        body: HircBody::MusicTrack(MusicTrack {
            flags: 0,
            sources: vec![Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1002,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            }],
            playlist: vec![TrackSrcInfo {
                track_id: 0,
                source_id: 1002,
                event_id: 0,
                play_at: 0.0,
                begin_trim_offset: 0.0,
                end_trim_offset: 0.0,
                src_duration: 30.0,
            }],
            num_sub_track: 1,
            clip_automation: vec![],
            node: NodeBase {
                override_bus_id: 1,
                direct_parent_id: 20,
                ..Default::default()
            },
            track_type: 0,
            switch: None,
            look_ahead_time: 0,
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x0a,
        id: 20,
        offset: 0,
        size: 0,
        body: HircBody::MusicSegment(MusicSegment {
            music: MusicNode {
                flags: 0,
                node: NodeBase {
                    override_bus_id: 1,
                    direct_parent_id: 100,
                    ..Default::default()
                },
                children: vec![21],
                meter: MeterInfo::default(),
                stingers: vec![],
            },
            duration_ms: 30.0,
            markers: vec![
                MusicMarker {
                    id: 43_573_010,
                    position_ms: 10.0,
                    name: String::new(),
                },
                MusicMarker {
                    id: 1_539_036_744,
                    position_ms: 30.0,
                    name: String::new(),
                },
            ],
        }),
    });

    // MusicRanSeqCntr 100: ContinuousSequence (rs_type = 0, loop_count = 1) with children [Seg 10, Seg 20]
    p.insert_hirc(HircObject {
        hirc_type: 0x0d,
        id: 100,
        offset: 0,
        size: 0,
        body: HircBody::MusicRanSeqCntr(MusicRanSeqCntr {
            trans: MusicTransNode {
                music: MusicNode {
                    flags: 0,
                    node: NodeBase {
                        override_bus_id: 1,
                        ..Default::default()
                    },
                    children: vec![10, 20],
                    meter: MeterInfo::default(),
                    stingers: vec![],
                },
                rules: vec![],
            },
            num_playlist_items: 3,
            playlist: vec![MusicPlaylistItem {
                segment_id: 0,
                playlist_item_id: 1,
                rs_type: 0, // ContinuousSequence
                loop_count: 1,
                loop_min: 0,
                loop_max: 0,
                weight: 50,
                avoid_repeat_count: 0,
                is_using_weight: false,
                is_shuffle: false,
                children: vec![
                    MusicPlaylistItem {
                        segment_id: 10,
                        playlist_item_id: 2,
                        rs_type: 0xFFFF_FFFF,
                        loop_count: 1,
                        loop_min: 0,
                        loop_max: 0,
                        weight: 50,
                        avoid_repeat_count: 0,
                        is_using_weight: false,
                        is_shuffle: false,
                        children: vec![],
                    },
                    MusicPlaylistItem {
                        segment_id: 20,
                        playlist_item_id: 3,
                        rs_type: 0xFFFF_FFFF,
                        loop_count: 1,
                        loop_min: 0,
                        loop_max: 0,
                        weight: 50,
                        avoid_repeat_count: 0,
                        is_using_weight: false,
                        is_shuffle: false,
                        children: vec![],
                    },
                ],
            }],
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 200,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 100,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 300,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![200] }),
    });

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.4; 1440]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );
    r.insert_pcm(
        1002,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.3; 1440]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );

    r.post_event(300, 0).unwrap();
    let out = r.render().unwrap();
    assert_eq!(r.voices.len(), 2);
    // Voice 0 (Seg 10): [0 .. 1440]
    assert_eq!(r.voices[0].start, 0);
    assert_eq!(r.voices[0].end, Some(1440));
    // Voice 1 (Seg 20): Entry at 960 minus pre-entry 480 -> starts at 480, ends at 480 + 1440 = 1920!
    assert_eq!(r.voices[1].start, 480);
    assert_eq!(r.voices[1].end, Some(1920));
    // During overlap [480 .. 1440], both voices play (0.4 + 0.3 = 0.7)
    assert!((out[1024] - 0.7).abs() < 0.02, "expected overlap 0.7, got {}", out[1024]);
}

#[test]
fn music_switch_cntr_decision_tree_transition_and_stinger() {
    use crate::bnk::hirc::{
        DecisionNode, DecisionTree, MeterInfo, MusicFade, MusicMarker, MusicNode, MusicSegment, MusicSwitchCntr,
        MusicTrack, MusicTransNode, Stinger, TrackSrcInfo, TransitionRule,
    };
    use crate::engine::music::sync_type;

    let mut p = Project::new(None);
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });

    // Helper to insert a MusicSegment + MusicTrack with a given constant PCM source
    let mut add_seg = |seg_id: u32, track_id: u32, src_id: u32, dur_ms: f64| {
        p.insert_hirc(HircObject {
            hirc_type: 0x0b,
            id: track_id,
            offset: 0,
            size: 0,
            body: HircBody::MusicTrack(MusicTrack {
                flags: 0,
                sources: vec![Source {
                    plugin_id: 0x0001_0001,
                    stream_type: 0,
                    source_id: src_id,
                    in_memory_media_size: 0,
                    source_bits: 0,
                    plugin_params: None,
                }],
                playlist: vec![TrackSrcInfo {
                    track_id: 0,
                    source_id: src_id,
                    event_id: 0,
                    play_at: 0.0,
                    begin_trim_offset: 0.0,
                    end_trim_offset: 0.0,
                    src_duration: dur_ms,
                }],
                num_sub_track: 1,
                clip_automation: vec![],
                node: NodeBase {
                    override_bus_id: 1,
                    direct_parent_id: seg_id,
                    ..Default::default()
                },
                track_type: 0,
                switch: None,
                look_ahead_time: 0,
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x0a,
            id: seg_id,
            offset: 0,
            size: 0,
            body: HircBody::MusicSegment(MusicSegment {
                music: MusicNode {
                    flags: 0,
                    node: NodeBase {
                        override_bus_id: 1,
                        direct_parent_id: 500,
                        ..Default::default()
                    },
                    children: vec![track_id],
                    meter: MeterInfo {
                        grid_period: 10.0, // 480 samples
                        grid_offset: 0.0,
                        tempo: 120.0,
                        time_sig_num_beats: 4,
                        time_sig_beat_value: 4,
                        override_flag: 1,
                    },
                    stingers: vec![],
                },
                duration_ms: dur_ms,
                markers: vec![
                    MusicMarker {
                        id: 43_573_010,
                        position_ms: 0.0,
                        name: String::new(),
                    },
                    MusicMarker {
                        id: 1_539_036_744,
                        position_ms: dur_ms,
                        name: String::new(),
                    },
                ],
            }),
        });
    };

    // Explore segment 10 (src 1001 = 0.25), Combat segment 20 (src 1002 = 0.6), Stinger segment 30 (src 1003 = 0.2)
    add_seg(10, 11, 1001, 40.0);
    add_seg(20, 21, 1002, 40.0);
    add_seg(30, 31, 1003, 20.0);

    // MusicSwitchCntr 500: driven by StateGroup 90 (state 1 = Explore -> 10, state 2 = Combat -> 20, wildcard 0 -> 10),
    // TransitionRule: Any -> Any on NextGrid (sync_type = 1, 10 ms = 480 samples),
    // Stinger on Trigger 777 -> Segment 30 Immediate (sync_play_at = 0).
    p.insert_hirc(HircObject {
        hirc_type: 0x0c,
        id: 500,
        offset: 0,
        size: 0,
        body: HircBody::MusicSwitchCntr(MusicSwitchCntr {
            trans: MusicTransNode {
                music: MusicNode {
                    flags: 0,
                    node: NodeBase {
                        override_bus_id: 1,
                        ..Default::default()
                    },
                    children: vec![10, 20],
                    meter: MeterInfo {
                        grid_period: 10.0,
                        grid_offset: 0.0,
                        tempo: 120.0,
                        time_sig_num_beats: 4,
                        time_sig_beat_value: 4,
                        override_flag: 1,
                    },
                    stingers: vec![Stinger {
                        trigger_id: 777,
                        segment_id: 30,
                        sync_play_at: sync_type::IMMEDIATE,
                        cue_filter_hash: 0,
                        dont_repeat_time: 100,
                        num_segment_look_ahead: 1,
                    }],
                },
                rules: vec![TransitionRule {
                    src_ids: vec![0xFFFF_FFFF],
                    dst_ids: vec![0xFFFF_FFFF],
                    src_fade: MusicFade {
                        transition_time: 0,
                        fade_curve: 4,
                        fade_offset: 0,
                    },
                    src_sync_type: sync_type::NEXT_GRID,
                    src_cue_filter_hash: 0,
                    play_post_exit: false,
                    dst_fade: MusicFade {
                        transition_time: 0,
                        fade_curve: 4,
                        fade_offset: 0,
                    },
                    dst_cue_filter_hash: 0,
                    jump_to_id: 0,
                    jump_to_type: 0,
                    entry_type: 0,
                    play_pre_entry: true,
                    dest_match_source_cue_name: false,
                    transition_object: None,
                }],
            },
            continue_playback: true,
            tree: DecisionTree {
                mode: 0,
                arguments: vec![(90, 1)], // StateGroup 90
                nodes: vec![
                    DecisionNode {
                        key: 0,
                        child_or_audio: 10,
                        weight: 50,
                        probability: 100,
                    },
                    DecisionNode {
                        key: 1,
                        child_or_audio: 10,
                        weight: 50,
                        probability: 100,
                    },
                    DecisionNode {
                        key: 2,
                        child_or_audio: 20,
                        weight: 50,
                        probability: 100,
                    },
                ],
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 200,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 500,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 300,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![200] }),
    });

    let mut gs = GameState::default();
    gs.set_state(90, 1); // Start in Explore (segment 10)
    let mut r = Renderer::new(
        &p,
        gs,
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    for (sid, amp) in [(1001, 0.25f32), (1002, 0.6f32), (1003, 0.2f32)] {
        r.insert_pcm(
            sid,
            Pcm {
                rate: RATE,
                layout: Layout::MONO,
                data: vec![vec![amp; 1920]],
                loop_points: None,
                akd_gain: 1.0,
            },
        );
    }

    r.post_event(300, 0).unwrap();
    let mut f0 = vec![];
    r.render_frame(&mut f0).unwrap();
    // Segment 10 (Track 11) is playing
    assert_eq!(r.voices.len(), 1);
    assert_eq!(r.voices[0].sound_id, 11);

    // At sample 512 (end of Frame 0), switch StateGroup 90 to 2 (Combat -> Segment 20).
    // NextGrid (10 ms = 480 samples) after 512 is 2 * 480 = 960!
    r.set_state(90, 2).unwrap();
    assert_eq!(r.voices.len(), 2);
    assert_eq!(
        r.voices[0].fade_out.map(|(f, _)| f.start),
        Some(960),
        "Explore track fade-out should anchor at NextGrid (960)"
    );
    assert_eq!(
        r.voices[0].stop_at,
        Some(1024),
        "Explore track 0-tick fade-out completes at end of frame containing 960 (1024)"
    );
    assert_eq!(r.voices[1].sound_id, 21, "Combat track 21 should be scheduled");
    assert_eq!(r.voices[1].start, 960, "Combat track should start at NextGrid (960)");

    // Fire Trigger 777: schedules Stinger segment 30 (Track 31) immediately at sample 512
    r.post_trigger(777).unwrap();
    assert_eq!(r.voices.len(), 3);
    assert_eq!(r.voices[2].sound_id, 31, "Stinger track 31 should start on trigger 777");
    assert_eq!(r.voices[2].start, 512);
}

#[test]
fn real_global_bnk_music_switch_cntr_resolves_and_schedules() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/audiobanks/pc");
    let init_path = dir.join("Init.bnk");
    let global_path = dir.join("Global.bnk");
    if !init_path.exists() || !global_path.exists() {
        return;
    }
    let mut p = Project::new(None);
    p.load_bank_bytes("Init.bnk".into(), std::fs::read(&init_path).unwrap())
        .unwrap();
    p.load_bank_bytes("Global.bnk".into(), std::fs::read(&global_path).unwrap())
        .unwrap();

    // Global.bnk contains MusicSwitchCntr 18728212 -> MusicRanSeqCntr 98248585 -> MusicSegment 540312475 -> MusicTrack 912726085
    assert!(matches!(
        p.node(18_728_212).map(|o| &o.body),
        Some(HircBody::MusicSwitchCntr(_))
    ));
    assert!(matches!(
        p.node(98_248_585).map(|o| &o.body),
        Some(HircBody::MusicRanSeqCntr(_))
    ));
    assert!(matches!(
        p.node(540_312_475).map(|o| &o.body),
        Some(HircBody::MusicSegment(_))
    ));
    assert!(matches!(
        p.node(912_726_085).map(|o| &o.body),
        Some(HircBody::MusicTrack(_))
    ));

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            max_seconds: 1.0,
            ..Default::default()
        },
    );
    // Play MusicSwitchCntr 18728212 directly: it should resolve its DecisionTree to MusicRanSeqCntr 98248585,
    // traverse its playlist to MusicSegment 540312475, and schedule active MusicSegment instances!
    let _ = r.play_node(18_728_212, 0, None, 0).unwrap();
    assert!(
        !r.active_music_switches.is_empty(),
        "MusicSwitchCntr should be registered active"
    );
    assert_eq!(r.active_music_switches[0].active_child, 98_248_585);
    assert!(
        !r.active_music_segments.is_empty(),
        "MusicRanSeqCntr should have scheduled MusicSegment 540312475"
    );
    assert_eq!(r.active_music_segments[0].segment_id, 540_312_475);
}

#[test]
fn node_and_bus_fx_chain_meter_writeback_and_bypass_action() {
    use crate::bnk::hirc::{FxBase, FxChain, FxSlot};
    use crate::engine::fx::plugin_id;

    let mut p = Project::new(None);

    // FX 501: Wwise Gain (+6.0206 dB -> ~2.0x) on Sound 101 slot 0
    let mut gain_bytes = Vec::new();
    gain_bytes.extend_from_slice(&6.0206f32.to_le_bytes());
    gain_bytes.extend_from_slice(&0.0f32.to_le_bytes());
    p.insert_hirc(HircObject {
        hirc_type: 0x11,
        id: 501,
        offset: 0,
        size: 0,
        body: HircBody::FxCustom(FxBase {
            plugin_id: plugin_id::GAIN,
            params: Some(gain_bytes),
            media: Vec::new(),
            rtpcs: Vec::new(),
            states: Default::default(),
            property_values: Vec::new(),
        }),
    });

    // FX 502: Wwise Meter on Bus 1 slot 0 writing Peak dB to GameParameter 888
    let mut meter_bytes = Vec::new();
    meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // attack = 0
    meter_bytes.extend_from_slice(&0.1f32.to_le_bytes()); // release = 0.1
    meter_bytes.extend_from_slice(&(-48.0f32).to_le_bytes()); // min = -48
    meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // max = 0
    meter_bytes.extend_from_slice(&0.0f32.to_le_bytes()); // hold = 0
    meter_bytes.extend_from_slice(&[0, 0, 0, 0]);
    meter_bytes.extend_from_slice(&888u32.to_le_bytes()); // uGameParamID = 888
    p.insert_hirc(HircObject {
        hirc_type: 0x10,
        id: 502,
        offset: 0,
        size: 0,
        body: HircBody::FxShareSet(FxBase {
            plugin_id: plugin_id::METER,
            params: Some(meter_bytes),
            media: Vec::new(),
            rtpcs: Vec::new(),
            states: Default::default(),
            property_values: Vec::new(),
        }),
    });

    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            fx: FxChain {
                override_parent: false,
                bypass_bits: 0,
                bypass_pos: None,
                slots: vec![FxSlot {
                    index: 0,
                    fx_id: 502,
                    is_shareset: true,
                    is_rendered: false,
                }],
            },
            ..Default::default()
        }),
    });

    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 101,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 1,
                fx: FxChain {
                    override_parent: true,
                    bypass_bits: 0,
                    bypass_pos: None,
                    slots: vec![FxSlot {
                        index: 0,
                        fx_id: 501,
                        is_shareset: false,
                        is_rendered: false,
                    }],
                },
                ..Default::default()
            },
        }),
    });

    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 201,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    // Action 202: BypassFx on Sound 101 slot 0 (target_mask = 1 << 0)
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 202,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x1A02,
            kind: ActionKind::BypassFx,
            target_id: 101,
            target_is_bus: false,
            props: Props::default(),
            ranged: RangedProps::default(),
            params: ActionParams::BypassFx {
                bypass: true,
                target_mask: 0x01,
                exceptions: Vec::new(),
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 301,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![201] }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x04,
        id: 302,
        offset: 0,
        size: 0,
        body: HircBody::Event(Event { actions: vec![202] }),
    });

    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            bus_fx: true,
            ..Default::default()
        },
    );
    r.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![0.25; FRAME * 2]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );

    // Frame 0: Gain FX (+6 dB) doubles 0.25 -> ~0.50; Bus Meter writes ~-6.02 dB to GameParameter 888!
    r.post_event(301, 0).unwrap();
    let mut f0 = vec![];
    r.render_frame(&mut f0).unwrap();
    assert!(
        (f0[FRAME / 2] - 0.50).abs() < 0.02,
        "expected ~0.50 with +6 dB Gain FX, got {}",
        f0[FRAME / 2]
    );
    let metered = r
        .gs
        .params
        .get(&888)
        .copied()
        .expect("Meter FX should write GameParam 888");
    assert!(
        (metered - (-6.02)).abs() < 0.25,
        "expected ~-6.02 dB meter RTPC, got {metered}"
    );

    // Frame 1: Post BypassFx event 302 at sample 512 -> Gain FX is bypassed, output drops back to 0.25!
    r.post_event(302, 512).unwrap();
    let mut f1 = vec![];
    r.render_frame(&mut f1).unwrap();
    assert!(
        (f1[FRAME / 2] - 0.25).abs() < 0.02,
        "expected 0.25 after BypassFx, got {}",
        f1[FRAME / 2]
    );
}

#[test]
fn fade_in_to_fade_out_handoff_starts_from_current_ratio() {
    // 10-tick linear fade-in (100 ms -> transition_ticks(4800) == 10), interrupted at
    // frame 4 (t = 4 * FRAME) by a 4-tick linear fade-out (40 ms -> 4 ticks).
    let fi = Fade {
        start: 0,
        len: ms_to_samples(100.0),
        curve: 4, /* Linear */
    };
    assert_eq!(fi.ticks(), 10);
    let mut v = Voice {
        sound_id: 1,
        playing_id: 1,
        event_id: 1,
        game_object_id: 0,
        chain: vec![1],
        params: super::super::params::VoiceParams::default(),
        pcm: Rc::new(Pcm {
            rate: RATE,
            layout: Layout::MONO,
            data: vec![vec![1.0; FRAME * 20]],
            loop_points: None,
            akd_gain: 1.0,
        }),
        start: 0,
        state: VState::Playing,
        evicted_by_limit: false,
        segments: vec![(0, FRAME * 20)],
        seg: 0,
        seg_pos: 0,
        res: Resampler::new(1, RATE, false),
        base_lin: 1.0,
        ray_user_aux: 1.0,
        ray_game_aux: 1.0,
        ray_lpf: 0.0,
        ray_hpf: 0.0,
        prev_gain: Some(0.0),
        aux_prev_gain: HashMap::new(),
        aux_filters: HashMap::new(),
        node_fx: vec![],
        fade_in: Some(fi),
        fade_out: None,
        stop_at: None,
        end: Some((FRAME * 20) as u64),
        matrix: vec![],
        filter: None,
        pause_fade: None,
        clip_origin_sample: 0,
        clip_automations: vec![],
    };

    // Frames 0..4 ramp linearly: 0.1, 0.2, 0.3, 0.4.
    for k in 1..=4u64 {
        let g = Renderer::envelope(&v, k * FRAME as u64);
        let want = k as f32 * 0.1;
        assert!(
            (g - want).abs() < 1e-5,
            "fade-in frame {}: got {g}, want {want}",
            k - 1
        );
    }

    // Stop at sample 4 * FRAME with 40 ms (4-tick) linear fade-out: must hand off from 0.4 -> 0.0.
    stop_voice(&mut v, 4 * FRAME as u64, ms_to_samples(40.0), 4 /* Linear */);
    let (_, captured_from) = v.fade_out.expect("fade_out scheduled");
    assert!(
        (captured_from - 0.4).abs() < 1e-5,
        "expected captured handoff 0.4, got {captured_from}"
    );

    // Frames 4..8 must descend smoothly from 0.4 -> 0.3 -> 0.2 -> 0.1 -> 0.0 (zero jump to 1.0).
    let expected_out = [0.3f32, 0.2, 0.1, 0.0];
    for (i, &want) in expected_out.iter().enumerate() {
        let t = (5 + i as u64) * FRAME as u64;
        let g = Renderer::envelope(&v, t);
        assert!(
            (g - want).abs() < 1e-5,
            "fade-out step {i} (t={t}): got {g}, want {want}"
        );
    }

    // Even before `fade_out.start`, scheduled fade-out must not prematurely suppress fade-in.
    assert!((Renderer::envelope(&v, 2 * FRAME as u64) - 0.2).abs() < 1e-5);
}

#[test]
fn fade_handoff_sample_accurate_continuity_across_render_frames() {
    let mut p = Project::new(None);
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });
    p.insert_hirc(make_sound(101, 0, 1, 1001, 0.0));
    // Action 201: Play 101 with 100 ms (10-tick) Linear fade-in
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 201,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 100u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 4,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    // Action 202: Stop 101 with 40 ms (4-tick) Linear fade-out
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 202,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0102,
            kind: ActionKind::Stop,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Active {
                fade_curve: 4,
                specific: None,
                exceptions: vec![],
            },
        }),
    });
    // Action 203: Pause 101 with 40 ms (4-tick) Linear fade-out
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 203,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0202,
            kind: ActionKind::Pause,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Active {
                fade_curve: 4,
                specific: None,
                exceptions: vec![],
            },
        }),
    });
    // Action 204: Resume 101 with 40 ms (4-tick) Linear fade-in
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 204,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0302,
            kind: ActionKind::Resume,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Active {
                fade_curve: 4,
                specific: None,
                exceptions: vec![],
            },
        }),
    });
    // Action 205: Play 101 with 100 ms SCurve (5) fade-in, Stop 206 with 40 ms SCurve (5) fade-out
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 205,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0403,
            kind: ActionKind::Play,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 100u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Play {
                fade_curve: 5,
                bank_id: 0,
                bank_type: 0,
            },
        }),
    });
    p.insert_hirc(HircObject {
        hirc_type: 0x03,
        id: 206,
        offset: 0,
        size: 0,
        body: HircBody::Action(Action {
            action_type: 0x0102,
            kind: ActionKind::Stop,
            target_id: 101,
            target_is_bus: false,
            props: Props(vec![(prop::TRANSITION_TIME, 40u32)]),
            ranged: RangedProps::default(),
            params: ActionParams::Active {
                fade_curve: 5,
                specific: None,
                exceptions: vec![],
            },
        }),
    });
    for (eid, aid) in [
        (301, 201),
        (302, 202),
        (303, 203),
        (304, 204),
        (305, 205),
        (306, 206),
    ] {
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: eid,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![aid] }),
        });
    }

    let dc_pcm = || Pcm {
        rate: RATE,
        layout: Layout::MONO,
        data: vec![vec![1.0; FRAME * 24]],
        loop_points: None,
        akd_gain: 1.0,
    };
    let unity_scale = db_to_lin(0.0).powi(4); // voice (3) + master (1)

    // --- Case A: Linear fade-in (10 ticks) -> Stop fade-out (4 ticks) at frame 4 ---
    let mut r = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r.insert_pcm(1001, dc_pcm());
    r.post_event(301, 0).unwrap();
    r.post_event(302, 4 * FRAME as u64).unwrap();
    let out = r.render().unwrap();
    assert_eq!(out.len(), 8 * FRAME, "4 fade-in frames + 4 fade-out frames");
    let max_step = out
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0f32, f32::max);
    assert!(
        max_step < 0.0005,
        "expected sample-continuous ramp across handoff, max step = {max_step}"
    );
    let peak_idx = 4 * FRAME - 1;
    assert!(
        (out[peak_idx] - 0.4 * unity_scale).abs() < 1e-4,
        "peak at handoff should be 0.4, got {}",
        out[peak_idx]
    );
    assert!(
        out[4 * FRAME] < out[peak_idx],
        "fade-out must descend immediately at handoff sample"
    );
    assert!(out.last().unwrap().abs() < 1e-5, "fade-out must reach 0.0 cleanly");

    // --- Case B: Curved (SCurve) fade-in -> curved fade-out at frame 5 ---
    let mut r_sc = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r_sc.insert_pcm(1001, dc_pcm());
    r_sc.post_event(305, 0).unwrap();
    r_sc.post_event(306, 5 * FRAME as u64).unwrap();
    let out_sc = r_sc.render().unwrap();
    let handoff_jump = (out_sc[5 * FRAME] - out_sc[5 * FRAME - 1]).abs();
    assert!(handoff_jump < 0.001, "SCurve handoff discontinuity: {handoff_jump}");
    assert!(
        out_sc[5 * FRAME] < out_sc[5 * FRAME - 1],
        "SCurve fade-out must descend from handoff"
    );
    assert!(out_sc.last().unwrap().abs() < 1e-5);

    // --- Case C: fade-in -> pause mid-fade-in (frame 4, ratio 0.4) -> resume mid-pause-fade-out (frame 6, ratio 0.2) ---
    let mut r_pr = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 1,
            ..Default::default()
        },
    );
    r_pr.insert_pcm(1001, dc_pcm());
    r_pr.post_event(301, 0).unwrap();
    r_pr.post_event(303, 4 * FRAME as u64).unwrap(); // Pause at 0.4 over 4 ticks -> 0.3, 0.2, ...
    r_pr.post_event(304, 6 * FRAME as u64).unwrap(); // Resume mid-pause-fade at 0.2 over 4 ticks -> 0.4, 0.6, 0.8, 1.0
    let mut out_pr = Vec::new();
    for _ in 0..12 {
        r_pr.render_frame(&mut out_pr).unwrap();
    }
    let max_step_pr = out_pr
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0f32, f32::max);
    assert!(
        max_step_pr < 0.001,
        "pause/resume mid-fade handoff discontinuity: max step = {max_step_pr}"
    );
    // At end of frame 3 (handoff to Pause): 0.4
    assert!((out_pr[4 * FRAME - 1] - 0.4 * unity_scale).abs() < 1e-4);
    // At end of frame 5 (handoff to Resume mid-pause-fade): 0.2
    assert!((out_pr[6 * FRAME - 1] - 0.2 * unity_scale).abs() < 1e-4);
    // After 4 resume ticks (frame 10): smoothly back to full unity 1.0!
    assert!((out_pr[10 * FRAME - 1] - 1.0 * unity_scale).abs() < 1e-4);
}

#[test]
fn bus_1134869537_passthrough_gain_staging_eliminates_fast_pow10_offset() {
    use crate::bnk::hirc::{FxChain, FxSlot};

    let mut p = Project::new(None);
    // Master bus 1 (override_bus_id = 0 -> 1 FastPow10 stage)
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus::default()),
    });
    // Non-master mixing bus 2 (explicit channel_config -> 2 FastPow10 stages)
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 2,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            override_bus_id: 1,
            channel_config: 0x3102,
            ..Default::default()
        }),
    });
    // Filtered bus 1134869537: BusVolume = -3 dB, OutputBusLPF = 17, channel_config = 0,
    // active FX slot bypassed in bus_fx=false mode -> pass-through (0 stages of its own)!
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 1134869537,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            override_bus_id: 2,
            props: Props(vec![
                (prop::BUS_VOLUME, (-3.0f32).to_bits()),
                (prop::OUTPUT_BUS_LPF, 17.0f32.to_bits()),
            ]),
            fx: FxChain {
                override_parent: false,
                bypass_bits: 0,
                bypass_pos: None,
                slots: vec![FxSlot {
                    index: 3,
                    fx_id: 9999,
                    is_shareset: false,
                    is_rendered: false,
                }],
            },
            ..Default::default()
        }),
    });
    // Standard 0 dB pass-through bus 3 -> mixing bus 2 -> master bus 1 (like event 4152114336)
    p.insert_hirc(HircObject {
        hirc_type: 0x08,
        id: 3,
        offset: 0,
        size: 0,
        body: HircBody::Bus(Bus {
            override_bus_id: 2,
            ..Default::default()
        }),
    });
    // Sound 101 (vol = -5, makeup = +3) -> bus 1134869537 (matches event 1318950597!)
    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 101,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1001,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 1134869537,
                props: Props(vec![
                    (prop::VOLUME, (-5.0f32).to_bits()),
                    (prop::MAKE_UP_GAIN, 3.0f32.to_bits()),
                ]),
                ..Default::default()
            },
        }),
    });
    // Sound 102 (vol = -2, makeup = +6) -> bus 3 (matches event 4152114336!)
    p.insert_hirc(HircObject {
        hirc_type: 0x02,
        id: 102,
        offset: 0,
        size: 0,
        body: HircBody::Sound(Sound {
            source: Source {
                plugin_id: 0x0001_0001,
                stream_type: 0,
                source_id: 1002,
                in_memory_media_size: 0,
                source_bits: 0,
                plugin_params: None,
            },
            node: NodeBase {
                override_bus_id: 3,
                props: Props(vec![
                    (prop::VOLUME, (-2.0f32).to_bits()),
                    (prop::MAKE_UP_GAIN, 6.0f32.to_bits()),
                ]),
                ..Default::default()
            },
        }),
    });
    for (aid, eid, tid) in [(201, 301, 101), (202, 302, 102)] {
        p.insert_hirc(HircObject {
            hirc_type: 0x03,
            id: aid,
            offset: 0,
            size: 0,
            body: HircBody::Action(Action {
                action_type: 0x0403,
                kind: ActionKind::Play,
                target_id: tid,
                target_is_bus: false,
                props: Props::default(),
                ranged: RangedProps::default(),
                params: ActionParams::Play {
                    fade_curve: 4,
                    bank_id: 0,
                    bank_type: 0,
                },
            }),
        });
        p.insert_hirc(HircObject {
            hirc_type: 0x04,
            id: eid,
            offset: 0,
            size: 0,
            body: HircBody::Event(Event { actions: vec![aid] }),
        });
    }

    // 1. Filtered bus 1134869537 with bus_fx = false:
    //    Exact Wwise 6-stage product: db(-5) * db(+3) * db(-3) * db(0)^3
    //    (eliminates the 2 extra FastPow10(0) stages -> exact 1 / FastPow10(0)^2 fix!).
    let mut r1 = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 2,
            bus_fx: false,
            ..Default::default()
        },
    );
    r1.insert_pcm(
        1001,
        Pcm {
            rate: RATE,
            layout: Layout::STEREO,
            data: vec![vec![1.0; FRAME * 2], vec![1.0; FRAME * 2]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );
    r1.post_event(301, 0).unwrap();
    let out1 = r1.render().unwrap();
    // DC steady state after LPF=17 settles (last sample of frame 1)
    let got1 = out1[out1.len() - 2];
    let want1 = db_to_lin(-5.0) * db_to_lin(3.0) * db_to_lin(-3.0) * db_to_lin(0.0).powi(3);
    let old_buggy1 = want1 * db_to_lin(0.0).powi(2);
    assert!(
        (got1 - want1).abs() < 1e-6,
        "bus 1134869537 gain mismatch: got {got1:.8}, want {want1:.8} (old buggy was {old_buggy1:.8})"
    );
    // Verify LPF=17 filter was applied on the voice connection (and NOT double-applied on bus 1134869537)
    assert!(
        r1.voices[0].filter.is_some(),
        "folded OutputBusLPF=17 must live on voice connection"
    );
    assert!(
        r1.bus_filters.get(&1134869537).is_none(),
        "pass-through bus must not double-filter"
    );

    // 2. Standard 0 dB pass-through bus 3: exact 6-stage product db(-2) * db(+6) * db(0)^4 preserved!
    let mut r2 = Renderer::new(
        &p,
        GameState::default(),
        RenderOptions {
            out_channels: 2,
            bus_fx: false,
            ..Default::default()
        },
    );
    r2.insert_pcm(
        1002,
        Pcm {
            rate: RATE,
            layout: Layout::STEREO,
            data: vec![vec![1.0; FRAME], vec![1.0; FRAME]],
            loop_points: None,
            akd_gain: 1.0,
        },
    );
    r2.post_event(302, 0).unwrap();
    let out2 = r2.render().unwrap();
    let got2 = out2[out2.len() - 2];
    let want2 = db_to_lin(-2.0) * db_to_lin(6.0) * db_to_lin(0.0).powi(4);
    assert!(
        (got2 - want2).abs() < 1e-6,
        "standard pass-through bus gain mismatch: got {got2:.8}, want {want2:.8}"
    );
}
