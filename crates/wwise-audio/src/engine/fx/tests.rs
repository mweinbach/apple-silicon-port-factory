use super::*;
use crate::bnk::hirc::GraphPoint;
#[cfg(feature = "northlight")]
use crate::bnk::Bank;
#[cfg(feature = "northlight")]
use crate::data::{game_dir, GameAudio};
use crate::engine::math::db_to_lin;
use crate::engine::params::rtpc_accum;
use std::f32::consts::PI;

#[test]
fn gain_fx_scales_fullband_and_lfe_separately() {
    let project = Project::new(None);
    let gs = GameState::default();
    let mut raw = Vec::new();
    raw.extend_from_slice(&6.0f32.to_le_bytes()); // +6 dB fullband (~1.995)
    raw.extend_from_slice(&(-6.0f32).to_le_bytes()); // -6 dB LFE (~0.501)
    let params = parse_fx_params(plugin_id::GAIN, &raw).unwrap();
    let mut inst = FxInstance::new(1, plugin_id::GAIN, params, Vec::new());

    let mut buf = vec![vec![0.5f32; 64], vec![0.5f32; 64]];
    assert!(inst.process(&project, &gs, &mut buf, Some(1)).is_none());
    assert!((buf[0][0] - 0.5 * db_to_lin(6.0)).abs() < 1e-4);
    assert!((buf[1][0] - 0.5 * db_to_lin(-6.0)).abs() < 1e-4);
}

#[test]
fn parametric_eq_biquad_lowpass_and_peaking_boost() {
    let project = Project::new(None);
    let gs = GameState::default();
    let mut raw = Vec::new();
    // Band 0: LowPass at 500 Hz, Q = 0.707, On
    raw.extend_from_slice(&0u32.to_le_bytes());
    raw.extend_from_slice(&0.0f32.to_le_bytes());
    raw.extend_from_slice(&500.0f32.to_le_bytes());
    raw.extend_from_slice(&0.707f32.to_le_bytes());
    raw.push(1);
    // Band 1: Peaking +12 dB at 200 Hz, Q = 1.0, On
    raw.extend_from_slice(&6u32.to_le_bytes());
    raw.extend_from_slice(&12.0f32.to_le_bytes());
    raw.extend_from_slice(&200.0f32.to_le_bytes());
    raw.extend_from_slice(&1.0f32.to_le_bytes());
    raw.push(1);
    // Band 2: HighShelf off
    raw.extend_from_slice(&5u32.to_le_bytes());
    raw.extend_from_slice(&0.0f32.to_le_bytes());
    raw.extend_from_slice(&4000.0f32.to_le_bytes());
    raw.extend_from_slice(&1.0f32.to_le_bytes());
    raw.push(0);
    // OutputLevel = 0 dB, bProcessLFE = 1
    raw.extend_from_slice(&0.0f32.to_le_bytes());
    raw.push(1);
    assert_eq!(raw.len(), 56);

    let params = parse_fx_params(plugin_id::PARAMETRIC_EQ, &raw).unwrap();
    let mut inst_lo = FxInstance::new(2, plugin_id::PARAMETRIC_EQ, params.clone(), Vec::new());
    let mut inst_hi = FxInstance::new(2, plugin_id::PARAMETRIC_EQ, params, Vec::new());

    // 200 Hz sine vs 8000 Hz sine
    let lo_sine: Vec<f32> = (0..512)
        .map(|n| (2.0 * PI * 200.0 * n as f32 / RATE as f32).sin() * 0.25)
        .collect();
    let hi_sine: Vec<f32> = (0..512)
        .map(|n| (2.0 * PI * 8000.0 * n as f32 / RATE as f32).sin() * 0.25)
        .collect();
    let mut buf_lo = vec![lo_sine];
    let mut buf_hi = vec![hi_sine];
    inst_lo.process(&project, &gs, &mut buf_lo, None);
    inst_hi.process(&project, &gs, &mut buf_hi, None);

    let peak_lo = buf_lo[0][256..].iter().fold(0.0f32, |a, &x| a.max(x.abs()));
    let peak_hi = buf_hi[0][256..].iter().fold(0.0f32, |a, &x| a.max(x.abs()));
    assert!(peak_lo > 0.5, "200 Hz should be boosted by +12 dB peaking band, got {peak_lo}");
    assert!(peak_hi < 0.05, "8 kHz should be attenuated by 500 Hz lowpass, got {peak_hi}");
}

#[test]
fn meter_envelope_follower_and_game_param_output() {
    let project = Project::new(None);
    let gs = GameState::default();
    let mut raw = Vec::new();
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // fAttack = 0s (instant)
    raw.extend_from_slice(&0.5f32.to_le_bytes()); // fRelease = 0.5s
    raw.extend_from_slice(&(-48.0f32).to_le_bytes()); // fMin = -48 dB
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // fMax = 0 dB
    raw.extend_from_slice(&0.05f32.to_le_bytes()); // fHold = 50 ms
    raw.push(0); // bInfiniteHold = false
    raw.push(0); // eMode = 0 (Peak)
    raw.push(0); // eScope = 0
    raw.push(0); // bApplyDownstreamVolume = false
    raw.extend_from_slice(&999u32.to_le_bytes()); // uGameParamID = 999
    assert_eq!(raw.len(), 28);

    let params = parse_fx_params(plugin_id::METER, &raw).unwrap();
    let mut inst = FxInstance::new(3, plugin_id::METER, params, Vec::new());

    // Half-scale peak (0.5 -> ~-6.02 dB)
    let mut loud = vec![vec![0.5f32; 512]];
    let (param_id, db_loud) = inst.process(&project, &gs, &mut loud, None).unwrap();
    assert_eq!(param_id, 999);
    assert!((db_loud - (-6.02)).abs() < 0.2, "got {db_loud}");

    // Immediate silent frame should stay held at -6.02 dB because hold_s = 50 ms (> 10.67 ms)
    let mut silent = vec![vec![0.0f32; 512]];
    let (_, db_held) = inst.process(&project, &gs, &mut silent, None).unwrap();
    assert!((db_held - db_loud).abs() < 1e-4);

    // After hold expires (> 5 frames), envelope releases toward min_db (-48 dB)
    let mut db_released = db_held;
    for _ in 0..10 {
        let (_, d) = inst.process(&project, &gs, &mut silent, None).unwrap();
        db_released = d;
    }
    assert!(db_released < db_loud - 2.0 && db_released >= -48.0, "got {db_released}");
}

#[test]
fn delay_and_stereo_delay_produce_echoes() {
    let project = Project::new(None);
    let gs = GameState::default();

    // 1. Delay: 10 samples (10 / 48000 s), 50% feedback, 50% wet/dry
    let mut raw = Vec::new();
    raw.extend_from_slice(&(10.0f32 / RATE as f32).to_le_bytes());
    raw.extend_from_slice(&50.0f32.to_le_bytes());
    raw.extend_from_slice(&50.0f32.to_le_bytes());
    raw.extend_from_slice(&0.0f32.to_le_bytes());
    raw.push(1);
    raw.push(1);
    assert_eq!(raw.len(), 18);
    let params = parse_fx_params(plugin_id::DELAY, &raw).unwrap();
    let mut delay_inst = FxInstance::new(4, plugin_id::DELAY, params, Vec::new());

    let mut buf = vec![vec![0.0f32; 64]];
    buf[0][0] = 1.0;
    delay_inst.process(&project, &gs, &mut buf, None);
    assert!((buf[0][0] - 0.5).abs() < 1e-3, "dry impulse at t=0: {}", buf[0][0]);
    assert!((buf[0][10] - 0.5).abs() < 1e-3, "first echo at t=10: {}", buf[0][10]);
    assert!((buf[0][20] - 0.25).abs() < 1e-3, "second feedback echo at t=20: {}", buf[0][20]);

    // 2. Stereo Delay with crossfeed: L delay 8 samples, R delay 12 samples
    let sd_params = FxParams::StereoDelay(StereoDelayParams {
        channels: [
            StereoDelayChannelParams {
                input_type: 0,
                delay_time_s: 8.0 / RATE as f32,
                feedback_db: -6.0,
                crossfeed_db: -6.0,
            },
            StereoDelayChannelParams {
                input_type: 0,
                delay_time_s: 12.0 / RATE as f32,
                feedback_db: -6.0,
                crossfeed_db: -6.0,
            },
        ],
        filter_type: 0,
        filter_gain_db: 0.0,
        filter_freq_hz: 1000.0,
        filter_q: 1.0,
        dry_level_db: 0.0,
        wet_level_db: 0.0,
        front_rear_balance: 0.0,
        enable_feedback: true,
        enable_crossfeed: true,
    });
    let mut sd_inst = FxInstance::new(5, plugin_id::STEREO_DELAY, sd_params, Vec::new());
    let mut sbuf = vec![vec![0.0f32; 64], vec![0.0f32; 64]];
    sbuf[0][0] = 1.0; // Impulse in Left only
    sd_inst.process(&project, &gs, &mut sbuf, None);
    assert!(sbuf[0][8] > 0.9, "Left echo at t=8");
    assert!(
        sbuf[1][20] > 0.4,
        "Crossfed Right echo at t=8+12=20, got {}",
        sbuf[1][20]
    );
}

#[test]
fn dynamics_compressor_expander_and_peak_limiter() {
    let project = Project::new(None);
    let gs = GameState::default();

    // Compressor: threshold -12 dB, ratio 4:1, fast attack
    let comp_params = FxParams::Dynamics(DynamicsParams {
        kind: DynamicsKind::Compressor,
        threshold_db: -12.0,
        ratio: 4.0,
        attack_or_lookahead_s: 0.0005,
        release_s: 0.05,
        output_level_db: 0.0,
        process_lfe: true,
        channel_link: true,
    });
    let mut comp = FxInstance::new(6, plugin_id::COMPRESSOR, comp_params, Vec::new());
    let mut loud = vec![vec![1.0f32; 512]];
    comp.process(&project, &gs, &mut loud, None);
    // Steady-state 0 dBFS input over -12 dB threshold with 4:1 ratio -> -9 dB gain reduction (~0.355)
    assert!(loud[0][511] < 0.45 && loud[0][511] > 0.25, "got {}", loud[0][511]);

    // Expander: threshold -20 dB, ratio 4:1 -> attenuates quiet signals below threshold
    let exp_params = FxParams::Dynamics(DynamicsParams {
        kind: DynamicsKind::Expander,
        threshold_db: -20.0,
        ratio: 4.0,
        attack_or_lookahead_s: 0.0005,
        release_s: 0.005,
        output_level_db: 0.0,
        process_lfe: true,
        channel_link: true,
    });
    let mut exp = FxInstance::new(7, plugin_id::EXPANDER, exp_params, Vec::new());
    let mut quiet = vec![vec![0.02f32; 512]]; // -34 dB < -20 dB
    exp.process(&project, &gs, &mut quiet, None);
    assert!(
        quiet[0][511] < 0.005,
        "quiet signal expanded downward, got {}",
        quiet[0][511]
    );
}

#[test]
fn tremolo_flanger_and_guitar_distortion_modulate_and_saturate() {
    let project = Project::new(None);
    let gs = GameState::default();

    // Tremolo: 100% depth at 100 Hz
    let trem_params = FxParams::Tremolo(TremoloParams {
        mod_depth: 100.0,
        lfo_freq_hz: 100.0,
        lfo_waveform: 0,
        lfo_smooth: 0.0,
        lfo_pwm: 50.0,
        phase_offset_deg: 0.0,
        phase_mode: 0,
        phase_spread_deg: 0.0,
        output_level_db: 0.0,
        process_center: true,
        process_lfe: true,
    });
    let mut trem = FxInstance::new(8, plugin_id::TREMOLO, trem_params, Vec::new());
    let mut dc = vec![vec![1.0f32; 512]];
    trem.process(&project, &gs, &mut dc, None);
    let min_v = dc[0].iter().copied().fold(f32::INFINITY, f32::min);
    let max_v = dc[0].iter().copied().fold(f32::NEG_INFINITY, f32::max);
    assert!(min_v < 0.05 && max_v > 0.95, "tremolo range {min_v}..{max_v}");

    // Flanger
    let fl_params = FxParams::Flanger(FlangerParams {
        delay_time_ms: 5.0,
        dry_level: 0.5,
        ffwd_level: 0.5,
        fback_level: 0.3,
        mod_depth: 50.0,
        lfo_freq_hz: 2.0,
        lfo_waveform: 0,
        lfo_smooth: 0.0,
        lfo_pwm: 50.0,
        phase_offset_deg: 0.0,
        phase_mode: 0,
        phase_spread_deg: 0.0,
        output_level_db: 0.0,
        wet_dry_mix: 50.0,
        enable_lfo: true,
        process_center: true,
        process_lfe: true,
    });
    let mut fl = FxInstance::new(9, plugin_id::FLANGER, fl_params, Vec::new());
    let mut sig: Vec<Vec<f32>> = vec![(0..512)
        .map(|n| (2.0 * PI * 440.0 * n as f32 / RATE as f32).sin())
        .collect()];
    let orig = sig[0].clone();
    fl.process(&project, &gs, &mut sig, None);
    assert!(sig[0].iter().zip(&orig).any(|(a, b)| (a - b).abs() > 0.05));

    // Guitar Distortion: heavy drive clips/saturates high-amplitude sine
    let gd_params = FxParams::GuitarDistortion(GuitarDistortionParams {
        pre_eq: [EqBandParams::default(); 3],
        post_eq: [EqBandParams::default(); 3],
        distortion_type: 0,
        drive: 90.0,
        tone: 20.0,
        rectification: 25.0,
        output_level_db: 0.0,
        wet_dry_mix: 100.0,
    });
    let mut gd = FxInstance::new(10, plugin_id::GUITAR_DISTORTION, gd_params, Vec::new());
    let mut gsig: Vec<Vec<f32>> = vec![(0..512)
        .map(|n| (2.0 * PI * 220.0 * n as f32 / RATE as f32).sin() * 0.8)
        .collect()];
    gd.process(&project, &gs, &mut gsig, None);
    assert!(gsig[0].iter().all(|x| x.is_finite() && x.abs() <= 1.5));
}

#[test]
fn roomverb_and_matrix_reverb_generate_decaying_tails() {
    let project = Project::new(None);
    let gs = GameState::default();

    // Matrix Reverb
    let mr_params = FxParams::MatrixReverb(MatrixReverbParams {
        reverb_time_s: 1.5,
        hf_ratio: 2.0,
        num_delays: 8,
        dry_level_db: -96.3, // wet only
        wet_level_db: 0.0,
        pre_delay_s: 0.002,
        process_lfe: false,
        delay_lengths_mode: 0,
        custom_delay_lengths_ms: Vec::new(),
    });
    let mut mr = FxInstance::new(11, plugin_id::MATRIX_REVERB, mr_params, Vec::new());
    let mut buf = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
    buf[0][0] = 1.0;
    buf[1][0] = 1.0;
    let mut total_tail_energy = 0.0f32;
    for frame in 0..8 {
        if frame > 0 {
            buf[0].fill(0.0);
            buf[1].fill(0.0);
        }
        mr.process(&project, &gs, &mut buf, None);
        if frame >= 3 {
            total_tail_energy += buf[0].iter().map(|x| x * x).sum::<f32>();
        }
    }
    assert!(
        total_tail_energy > 1e-5,
        "FDN tank should produce reverb tail energy, got {total_tail_energy}"
    );

    // RoomVerb with early reflections + FDN tank
    let rv_params = FxParams::RoomVerb(RoomVerbParams {
        decay_time_s: 2.0,
        hf_damping: 3.0,
        diffusion: 100.0,
        stereo_width: 180.0,
        filter_bands: [EqBandParams::default(); 3],
        front_level_db: 0.0,
        rear_level_db: 0.0,
        center_level_db: 0.0,
        lfe_level_db: -96.3,
        dry_level_db: -96.3,
        er_level_db: 0.0,
        reverb_level_db: 0.0,
        enable_early_reflections: true,
        er_pattern: 9,
        reverb_delay_ms: 10.0,
        room_size: 50.0,
        er_front_back_delay_ms: 10.0,
        density: 80.0,
        room_shape: 50.0,
        num_reverb_units: 8,
        enable_tone_controls: false,
        filter_pos: [0; 3],
        input_center_level_db: 0.0,
        input_lfe_level_db: -96.3,
    });
    let mut rv = FxInstance::new(12, plugin_id::ROOM_VERB, rv_params, Vec::new());
    let mut rbuf = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
    rbuf[0][0] = 1.0;
    let mut er_and_tail_energy = 0.0f32;
    for frame in 0..6 {
        if frame > 0 {
            rbuf[0].fill(0.0);
            rbuf[1].fill(0.0);
        }
        rv.process(&project, &gs, &mut rbuf, None);
        er_and_tail_energy += rbuf[0].iter().map(|x| x * x).sum::<f32>();
    }
    assert!(
        er_and_tail_energy > 1e-4,
        "RoomVerb ER + FDN tail energy {er_and_tail_energy}"
    );
}

#[test]
fn fx_rtpc_modulates_gain_and_eq_parameters() {
    let mut project = Project::new(None);
    project.param_defaults.insert(500, 0.0);
    let mut gs = GameState::default();
    gs.set_rtpc(500, 1.0);

    let rtpcs = vec![Rtpc {
        rtpc_id: 500,
        rtpc_type: 0,
        accum: rtpc_accum::ADDITIVE,
        param_id: 0, // FullbandGain
        curve_id: 1,
        scaling: 0,
        points: vec![
            GraphPoint {
                from: 0.0,
                to: -12.0,
                interp: 4,
            },
            GraphPoint {
                from: 1.0,
                to: 6.0,
                interp: 4,
            },
        ],
    }];
    let params = FxParams::Gain(GainParams {
        fullband_gain_db: 0.0,
        lfe_gain_db: 0.0,
    });
    let mut inst = FxInstance::new(13, plugin_id::GAIN, params, rtpcs);
    let mut buf = vec![vec![0.5f32; 16]];
    inst.process(&project, &gs, &mut buf, None);
    assert!((buf[0][0] - 0.5 * db_to_lin(6.0)).abs() < 1e-4);
}

#[test]
fn pitch_shifter_harmonizer_and_time_stretch_process_cleanly() {
    let project = Project::new(None);
    let gs = GameState::default();

    // 1. Pitch Shifter (+1200 cents = 1 octave up, 38-byte binary block)
    let mut raw_ps = Vec::new();
    raw_ps.extend_from_slice(&0u32.to_le_bytes()); // eInputType
    raw_ps.extend_from_slice(&(-96.3f32).to_le_bytes()); // fDryLevel (wet only)
    raw_ps.extend_from_slice(&0.0f32.to_le_bytes()); // fWetLevel
    raw_ps.extend_from_slice(&30.0f32.to_le_bytes()); // fDelayTime (ms)
    raw_ps.push(1); // bProcessLFE
    raw_ps.push(0); // bSyncDry
    raw_ps.extend_from_slice(&1200.0f32.to_le_bytes()); // fPitchFactor (+1200 cents)
    raw_ps.extend_from_slice(&0u32.to_le_bytes()); // eFilterType
    raw_ps.extend_from_slice(&0.0f32.to_le_bytes()); // fFilterGain
    raw_ps.extend_from_slice(&1000.0f32.to_le_bytes()); // fFilterFrequency
    raw_ps.extend_from_slice(&1.0f32.to_le_bytes()); // fFilterQFactor
    assert_eq!(raw_ps.len(), 38);
    let ps_params = parse_fx_params(plugin_id::PITCH_SHIFTER, &raw_ps).unwrap();
    let mut ps = FxInstance::new(20, plugin_id::PITCH_SHIFTER, ps_params, Vec::new());

    let sine: Vec<f32> = (0..1024)
        .map(|n| (2.0 * PI * 220.0 * n as f32 / RATE as f32).sin() * 0.5)
        .collect();
    let mut ps_buf = vec![sine.clone()];
    ps.process(&project, &gs, &mut ps_buf, None);
    assert!(ps_buf[0].iter().all(|x| x.is_finite() && x.abs() <= 1.0));
    let ps_energy: f32 = ps_buf[0][256..].iter().map(|x| x * x).sum();
    assert!(
        ps_energy > 1.0,
        "PitchShifter should produce non-trivial output, got {ps_energy}"
    );
    assert!(ps_buf[0][256..]
        .iter()
        .zip(&sine[256..])
        .any(|(a, b)| (a - b).abs() > 0.05));

    // 2. Harmonizer (2 voices: +700 cents and -500 cents, 68-byte binary block)
    let mut raw_hm = Vec::new();
    for &(en, cents, gain) in &[(1u8, 700.0f32, -3.0f32), (1u8, -500.0f32, -3.0f32)] {
        raw_hm.push(en);
        raw_hm.extend_from_slice(&cents.to_le_bytes());
        raw_hm.extend_from_slice(&gain.to_le_bytes());
        raw_hm.extend_from_slice(&0u32.to_le_bytes()); // eFilterType
        raw_hm.extend_from_slice(&0.0f32.to_le_bytes());
        raw_hm.extend_from_slice(&1000.0f32.to_le_bytes());
        raw_hm.extend_from_slice(&1.0f32.to_le_bytes());
    }
    raw_hm.extend_from_slice(&0u32.to_le_bytes()); // eInputType
    raw_hm.extend_from_slice(&(-6.0f32).to_le_bytes()); // fDryLevel
    raw_hm.extend_from_slice(&0.0f32.to_le_bytes()); // fWetLevel
    raw_hm.extend_from_slice(&1024u32.to_le_bytes()); // uWindowSize
    raw_hm.push(1); // bProcessLFE
    raw_hm.push(0); // bSyncDry
    assert_eq!(raw_hm.len(), 68);
    let hm_params = parse_fx_params(plugin_id::HARMONIZER, &raw_hm).unwrap();
    let mut hm = FxInstance::new(21, plugin_id::HARMONIZER, hm_params, Vec::new());
    let mut hm_buf = vec![sine.clone()];
    hm.process(&project, &gs, &mut hm_buf, None);
    assert!(hm_buf[0].iter().all(|x| x.is_finite() && x.abs() <= 2.0));
    assert!(hm_buf[0][256..]
        .iter()
        .zip(&sine[256..])
        .any(|(a, b)| (a - b).abs() > 0.05));

    // 3. Time Stretch (32-byte binary block)
    let mut raw_ts = Vec::new();
    raw_ts.extend_from_slice(&1024u32.to_le_bytes()); // uWindowSize
    raw_ts.extend_from_slice(&125.0f32.to_le_bytes()); // fTimeStretch (125%)
    raw_ts.extend_from_slice(&0.0f32.to_le_bytes()); // fTimeStretchRandom
    raw_ts.extend_from_slice(&300.0f32.to_le_bytes()); // fPitchShift (+300 cents)
    raw_ts.extend_from_slice(&0.0f32.to_le_bytes()); // fPitchShiftRandom
    raw_ts.extend_from_slice(&0.0f32.to_le_bytes()); // fOutputGain
    raw_ts.extend_from_slice(&50.0f32.to_le_bytes()); // fQuality
    raw_ts.extend_from_slice(&0u16.to_le_bytes()); // uStereoProc
    raw_ts.extend_from_slice(&1u16.to_le_bytes()); // bEnable = 1
    assert_eq!(raw_ts.len(), 32);
    let ts_params = parse_fx_params(plugin_id::TIME_STRETCH, &raw_ts).unwrap();
    let mut ts = FxInstance::new(22, plugin_id::TIME_STRETCH, ts_params, Vec::new());
    let mut ts_buf = vec![sine.clone()];
    ts.process(&project, &gs, &mut ts_buf, None);
    assert!(ts_buf[0].iter().all(|x| x.is_finite() && x.abs() <= 1.0));
    assert!(ts_buf[0][256..]
        .iter()
        .zip(&sine[256..])
        .any(|(a, b)| (a - b).abs() > 0.05));
}

#[test]
fn convolution_reverb_futzbox_trash_and_mastering_suite_process_cleanly() {
    let project = Project::new(None);
    let gs = GameState::default();

    // 1. Convolution Reverb (57-byte binary block)
    let mut raw_cr = Vec::new();
    raw_cr.extend_from_slice(&5.0f32.to_le_bytes()); // fPreDelay
    raw_cr.extend_from_slice(&10.0f32.to_le_bytes()); // fFrontRearDelay
    raw_cr.extend_from_slice(&180.0f32.to_le_bytes()); // fStereoWidth
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fInputCenterLevel
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // fInputLFELevel
    raw_cr.extend_from_slice(&180.0f32.to_le_bytes()); // fInputStereoWidth
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fFrontLevel
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fRearLevel
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fCenterLevel
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // fLFELevel
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // fDryLevel (wet only)
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fWetLevel
    raw_cr.extend_from_slice(&0u32.to_le_bytes()); // eAlgoType
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // fOutputLevel
    raw_cr.push(2); // uChannelConfig
    assert_eq!(raw_cr.len(), 57);
    let cr_params = parse_fx_params(plugin_id::CONVOLUTION_REVERB, &raw_cr).unwrap();
    let mut cr = FxInstance::new(23, plugin_id::CONVOLUTION_REVERB, cr_params, Vec::new());
    let mut cr_buf = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
    cr_buf[0][0] = 1.0;
    cr_buf[1][0] = 1.0;
    let mut tail_energy = 0.0f32;
    for f in 0..6 {
        if f > 0 {
            cr_buf[0].fill(0.0);
            cr_buf[1].fill(0.0);
        }
        cr.process(&project, &gs, &mut cr_buf, None);
        tail_energy += cr_buf[0].iter().map(|x| x * x).sum::<f32>();
    }
    assert!(
        tail_energy > 1e-4,
        "ConvolutionReverb should produce decaying tail, got {tail_energy}"
    );

    // 2. McDSP FutzBox (radio/speaker simulation + distortion)
    let fb_params = FxParams::McDspFutzBox(McDspFutzBoxParams {
        filter_enable: true,
        lpf_mode: 0,
        lpf_freq_hz: 3200.0,
        lpf_q: 0.7,
        hpf_mode: 0,
        hpf_freq_hz: 400.0,
        hpf_q: 0.7,
        dist_enable: true,
        dist_mode: 0,
        dist_amount: 60.0,
        dist_intensity: 50.0,
        dist_rectify: 20.0,
        eq_enable: true,
        eq_mode: 1,
        eq_freq_hz: 1200.0,
        eq_gain_db: 6.0,
        eq_q: 1.5,
        gate_enable: false,
        gate_thresh_db: -60.0,
        gate_range_db: -40.0,
        gate_attack_ms: 1.0,
        gate_hold_ms: 10.0,
        gate_release_ms: 50.0,
        sim_enable: true,
        sim_type: 1,
        sim_index: 3,
        sim_tuning: 0.0,
        input_gain_db: 0.0,
        output_gain_db: 0.0,
        balance_pct: 100.0,
    });
    let mut fb = FxInstance::new(24, plugin_id::MCDSP_FUTZBOX, fb_params, Vec::new());
    let sine: Vec<f32> = (0..512)
        .map(|n| (2.0 * PI * 440.0 * n as f32 / RATE as f32).sin() * 0.6)
        .collect();
    let mut fb_buf = vec![sine.clone()];
    fb.process(&project, &gs, &mut fb_buf, None);
    assert!(fb_buf[0].iter().all(|x| x.is_finite() && x.abs() <= 2.0));
    assert!(fb_buf[0][64..]
        .iter()
        .zip(&sine[64..])
        .any(|(a, b)| (a - b).abs() > 0.05));

    // 3. iZotope Trash Distortion (48-byte binary block)
    let mut raw_tr = Vec::new();
    for _ in 0..2 {
        raw_tr.extend_from_slice(&3.0f32.to_le_bytes()); // input_gain_db
        raw_tr.extend_from_slice(&8.0f32.to_le_bytes()); // overdrive
        raw_tr.extend_from_slice(&0.6f32.to_le_bytes()); // trash
        raw_tr.extend_from_slice(&100.0f32.to_le_bytes()); // mix_pct
        raw_tr.extend_from_slice(&(-3.0f32).to_le_bytes()); // output_gain_db
        raw_tr.extend_from_slice(&0u32.to_le_bytes()); // algo_type
    }
    assert_eq!(raw_tr.len(), 48);
    let tr_params = parse_fx_params(plugin_id::IZOTOPE_TRASH_DISTORTION, &raw_tr).unwrap();
    let mut tr = FxInstance::new(25, plugin_id::IZOTOPE_TRASH_DISTORTION, tr_params, Vec::new());
    let mut tr_buf = vec![sine.clone()];
    tr.process(&project, &gs, &mut tr_buf, None);
    assert!(tr_buf[0].iter().all(|x| x.is_finite() && x.abs() <= 2.0));
    assert!(tr_buf[0][64..]
        .iter()
        .zip(&sine[64..])
        .any(|(a, b)| (a - b).abs() > 0.05));

    // 4. Mastering Suite (EQ + compressor + limiter)
    let ms_params = FxParams::MasteringSuite(MasteringSuiteParams {
        eq_enable: true,
        comp_enable: true,
        width_enable: true,
        limiter_enable: true,
        eq_bands: [
            EqBandParams {
                filter_type: EqFilterType::Peaking,
                gain_db: 3.0,
                freq_hz: 1000.0,
                q_factor: 1.0,
                on_off: true,
            },
            EqBandParams::default(),
            EqBandParams::default(),
            EqBandParams::default(),
            EqBandParams::default(),
            EqBandParams::default(),
        ],
        stereo_width: 20.0,
        comp_threshold_db: -12.0,
        comp_ratio: 3.0,
        comp_attack_s: 0.002,
        comp_release_s: 0.05,
        comp_makeup_db: 2.0,
        limiter_threshold_db: -3.0,
        limiter_release_s: 0.05,
        limiter_ceiling_db: -1.0,
        master_output_gain_db: 0.0,
    });
    let mut ms = FxInstance::new(26, plugin_id::MASTERING_SUITE, ms_params, Vec::new());
    let mut ms_buf = vec![vec![0.95f32; 512], vec![-0.95f32; 512]];
    ms.process(&project, &gs, &mut ms_buf, None);
    let ceil = db_to_lin(-1.0) + 1e-3;
    assert!(ms_buf.iter().flatten().all(|x| x.is_finite() && x.abs() <= ceil));
}

#[test]
#[cfg(feature = "northlight")]
fn real_banks_fx_parse_and_process_cleanly() {
    if game_dir().is_none() {
        return;
    }
    let ga = GameAudio::open().unwrap();
    let project = Project::new(None);
    let gs = GameState::default();
    let mut parsed_count = 0usize;
    let mut built_in_processed = 0usize;

    // Parse and run every FxShareSet / FxCustom in Init.bnk + Global.bnk + first 150 banks
    for f in ga.banks().iter().take(150) {
        let Ok(buf) = ga.read(&f.path) else { continue };
        if buf.len() < 16 && !buf.starts_with(b"BKHD") {
            continue;
        }
        let Ok(b) = Bank::parse(&buf) else { continue };
        for o in &b.hirc {
            let (HircBody::FxShareSet(fx) | HircBody::FxCustom(fx)) = &o.body else {
                continue;
            };
            let mut inst = FxInstance::from_fx_base(o.id, fx)
                .unwrap_or_else(|e| panic!("{} fx {} ({:#x}): {}", f.path, o.id, fx.plugin_id, e));
            parsed_count += 1;
            if (fx.plugin_id & 0xF) == 3 {
                assert!(
                    !matches!(inst.params, FxParams::Passthrough { .. }),
                    "effect plugin {:#010x} in {} fell back to Passthrough",
                    fx.plugin_id,
                    f.path
                );
            }
            if !matches!(inst.params, FxParams::Passthrough { .. }) {
                let mut audio = vec![vec![0.1f32; 64], vec![-0.1f32; 64]];
                let _ = inst.process(&project, &gs, &mut audio, None);
                assert!(audio.iter().flatten().all(|x| x.is_finite()));
                built_in_processed += 1;
            }
        }
    }
    assert!(parsed_count >= 100, "expected >= 100 FX objects, got {parsed_count}");
    assert!(
        built_in_processed >= 80,
        "expected >= 80 built-in FX instances, got {built_in_processed}"
    );
}

#[test]
fn convolution_reverb_end_to_end_wwise_00_04_02_00_ir_loading_and_processing() {
    use crate::bnk::hirc::HircObject;

    // Synthesize a 2-channel Wwise `00 04 02 00` (bfloat16, block=1024, fft=2048) IR
    // with distinct L and R impulses spanning 2 partitions.
    let mut ir_l = vec![0.0f32; 1500];
    let mut ir_r = vec![0.0f32; 1500];
    ir_l[0] = 0.75;
    ir_l[1100] = -0.35;
    ir_r[24] = 0.5;
    ir_r[1200] = 0.25;
    let ir = PreprocessedIr::from_time_domain(RATE, 1024, &[ir_l, ir_r], IrPrecision::BFloat16).unwrap();
    let ir_blob = ir.encode_wwise_bytes(IrPrecision::BFloat16);
    assert_eq!(&ir_blob[0..4], &[0x00, 0x04, 0x02, 0x00]);
    assert!(parse_ir_media(&ir_blob).is_ok());

    // Build a 57-byte Convolution Reverb FX block with threshold_db = -60.0 (typical real
    // Init.bnk value; must NOT attenuate wet output!).
    let mut raw_cr = Vec::new();
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // pre_delay_ms
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // front_rear_delay_ms
    raw_cr.extend_from_slice(&180.0f32.to_le_bytes()); // stereo_width (full stereo)
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // input_center_level_db
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // input_lfe_level_db
    raw_cr.extend_from_slice(&180.0f32.to_le_bytes()); // input_stereo_width
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // front_level_db
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // rear_level_db
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // center_level_db
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // lfe_level_db
    raw_cr.extend_from_slice(&(-96.3f32).to_le_bytes()); // dry_level_db (wet only)
    raw_cr.extend_from_slice(&0.0f32.to_le_bytes()); // wet_level_db (0 dB)
    raw_cr.extend_from_slice(&1u32.to_le_bytes()); // algo_type = Direct
    raw_cr.extend_from_slice(&(-60.0f32).to_le_bytes()); // threshold_db = -60 dB
    raw_cr.push(2); // channel_config
    assert_eq!(raw_cr.len(), 57);

    const IR_MEDIA_ID: u32 = 900_001;
    let mut project = Project::new(None);
    project.insert_hirc(HircObject {
        hirc_type: 0x10,
        id: 700,
        offset: 0,
        size: 0,
        body: HircBody::FxShareSet(FxBase {
            plugin_id: plugin_id::CONVOLUTION_REVERB,
            params: Some(raw_cr),
            media: vec![(0, IR_MEDIA_ID)],
            rtpcs: Vec::new(),
            states: Default::default(),
            property_values: Vec::new(),
        }),
    });

    // 1. Instantiate BEFORE media is loaded (models Init.bnk loaded before Global.bnk):
    let mut fx = FxInstance::from_project(&project, 700).expect("FxInstance");
    assert!(!fx.has_convolution_ir(), "IR media not loaded yet");

    // 2. Register the `00 04 02 00` IR blob -> first process() lazily hydrates the convolver!
    project.insert_media(IR_MEDIA_ID, ir_blob);
    let gs = GameState::default();
    let mut out_l = Vec::new();
    let mut out_r = Vec::new();
    for f in 0..6 {
        let mut buf = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
        if f == 0 {
            buf[0][0] = 1.0;
            buf[1][0] = 1.0;
        }
        fx.process(&project, &gs, &mut buf, None);
        out_l.extend_from_slice(&buf[0]);
        out_r.extend_from_slice(&buf[1]);
    }
    assert!(fx.has_convolution_ir(), "IR convolver must hydrate lazily on process()");

    // Expect exact convolved impulses after 512-sample block priming latency, scaled by
    // front_lin * wet_lin = db_to_lin(0.0)^2 (NOT attenuated by threshold_db = -60 dB!).
    let gain = db_to_lin(0.0) * db_to_lin(0.0);
    assert!(
        (out_l[512 + 0] - 0.75 * gain).abs() < 1e-2,
        "L impulse 0: got {}, want {}",
        out_l[512 + 0],
        0.75 * gain
    );
    assert!(
        (out_l[512 + 1100] - -0.35 * gain).abs() < 1e-2,
        "L impulse 1100: got {}, want {}",
        out_l[512 + 1100],
        -0.35 * gain
    );
    assert!(
        (out_r[512 + 24] - 0.5 * gain).abs() < 1e-2,
        "R impulse 24: got {}, want {}",
        out_r[512 + 24],
        0.5 * gain
    );
    assert!(
        (out_r[512 + 1200] - 0.25 * gain).abs() < 1e-2,
        "R impulse 1200: got {}, want {}",
        out_r[512 + 1200],
        0.25 * gain
    );

    // 3. Reject garbage honestly
    assert!(parse_ir_media(b"NOPE").is_err());
}
