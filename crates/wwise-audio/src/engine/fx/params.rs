//! Wwise v145 FX plugin IDs, parameter structs, and binary parameter block parsers.

use crate::engine::convolver::PreprocessedIr;

pub mod plugin_id {
    pub const Q_ANALYZER: u32 = 0x0000_0403;
    pub const IZOTOPE_TRASH_DISTORTION: u32 = 0x0003_1033;
    pub const IZOTOPE_TRASH_DELAY: u32 = 0x0004_1033;
    pub const IZOTOPE_TRASH_DYNAMICS: u32 = 0x0005_1033;
    pub const IZOTOPE_TRASH_FILTERS: u32 = 0x0006_1033;
    pub const IZOTOPE_TRASH_BOX_MODELER: u32 = 0x0007_1033;
    pub const IZOTOPE_TRASH_MULTIBAND: u32 = 0x0009_1033;
    pub const PARAMETRIC_EQ: u32 = 0x0069_0003;
    pub const DELAY: u32 = 0x006A_0003;
    pub const COMPRESSOR: u32 = 0x006C_0003;
    pub const EXPANDER: u32 = 0x006D_0003;
    pub const PEAK_LIMITER: u32 = 0x006E_0003;
    pub const MCDSP_FUTZBOX: u32 = 0x006E_1003;
    pub const MATRIX_REVERB: u32 = 0x0073_0003;
    pub const ROOM_VERB: u32 = 0x0076_0003;
    pub const FLANGER: u32 = 0x007D_0003;
    pub const GUITAR_DISTORTION: u32 = 0x007E_0003;
    pub const CONVOLUTION_REVERB: u32 = 0x007F_0003;
    pub const METER: u32 = 0x0081_0003;
    pub const TIME_STRETCH: u32 = 0x0082_0003;
    pub const TREMOLO: u32 = 0x0083_0003;
    pub const STEREO_DELAY: u32 = 0x0087_0003;
    pub const PITCH_SHIFTER: u32 = 0x0088_0003;
    pub const HARMONIZER: u32 = 0x008A_0003;
    pub const GAIN: u32 = 0x008B_0003;
    pub const MASTERING_SUITE: u32 = 0x00BA_0003;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqFilterType {
    LowPass = 0,
    HighPass = 1,
    BandPass = 2,
    Notch = 3,
    LowShelf = 4,
    HighShelf = 5,
    Peaking = 6,
}

impl EqFilterType {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::LowPass,
            1 => Self::HighPass,
            2 => Self::BandPass,
            3 => Self::Notch,
            4 => Self::LowShelf,
            5 => Self::HighShelf,
            _ => Self::Peaking,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqBandParams {
    pub filter_type: EqFilterType,
    pub gain_db: f32,
    pub freq_hz: f32,
    pub q_factor: f32,
    pub on_off: bool,
}

impl Default for EqBandParams {
    fn default() -> Self {
        Self {
            filter_type: EqFilterType::Peaking,
            gain_db: 0.0,
            freq_hz: 1000.0,
            q_factor: 1.0,
            on_off: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GainParams {
    pub fullband_gain_db: f32,
    pub lfe_gain_db: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParametricEqParams {
    pub bands: [EqBandParams; 3],
    pub output_level_db: f32,
    pub process_lfe: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DelayParams {
    pub delay_time_s: f32,
    pub feedback: f32,
    pub wet_dry_mix: f32,
    pub output_level_db: f32,
    pub feedback_enabled: bool,
    pub process_lfe: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StereoDelayChannelParams {
    pub input_type: u32,
    pub delay_time_s: f32,
    pub feedback_db: f32,
    pub crossfeed_db: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StereoDelayParams {
    pub channels: [StereoDelayChannelParams; 2],
    pub filter_type: u32,
    pub filter_gain_db: f32,
    pub filter_freq_hz: f32,
    pub filter_q: f32,
    pub dry_level_db: f32,
    pub wet_level_db: f32,
    pub front_rear_balance: f32,
    pub enable_feedback: bool,
    pub enable_crossfeed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicsKind {
    Compressor,
    Expander,
    PeakLimiter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicsParams {
    pub kind: DynamicsKind,
    pub threshold_db: f32,
    pub ratio: f32,
    /// Attack time (s) for Compressor/Expander, or LookAhead time (s) for PeakLimiter.
    pub attack_or_lookahead_s: f32,
    pub release_s: f32,
    pub output_level_db: f32,
    pub process_lfe: bool,
    pub channel_link: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeterParams {
    pub attack_s: f32,
    pub release_s: f32,
    pub min_db: f32,
    pub max_db: f32,
    pub hold_s: f32,
    pub infinite_hold: bool,
    /// `0` = Peak, `1` = RMS.
    pub mode: u8,
    pub scope: u8,
    pub apply_downstream_volume: bool,
    pub game_param_id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TremoloParams {
    pub mod_depth: f32,
    pub lfo_freq_hz: f32,
    pub lfo_waveform: u32,
    pub lfo_smooth: f32,
    pub lfo_pwm: f32,
    pub phase_offset_deg: f32,
    pub phase_mode: u32,
    pub phase_spread_deg: f32,
    pub output_level_db: f32,
    pub process_center: bool,
    pub process_lfe: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlangerParams {
    pub delay_time_ms: f32,
    pub dry_level: f32,
    pub ffwd_level: f32,
    pub fback_level: f32,
    pub mod_depth: f32,
    pub lfo_freq_hz: f32,
    pub lfo_waveform: u32,
    pub lfo_smooth: f32,
    pub lfo_pwm: f32,
    pub phase_offset_deg: f32,
    pub phase_mode: u32,
    pub phase_spread_deg: f32,
    pub output_level_db: f32,
    pub wet_dry_mix: f32,
    pub enable_lfo: bool,
    pub process_center: bool,
    pub process_lfe: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuitarDistortionParams {
    pub pre_eq: [EqBandParams; 3],
    pub post_eq: [EqBandParams; 3],
    /// `0` = Overdrive, `1` = Heavy, `2` = Fuzz, `3` = Clip.
    pub distortion_type: u32,
    pub drive: f32,
    pub tone: f32,
    pub rectification: f32,
    pub output_level_db: f32,
    pub wet_dry_mix: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixReverbParams {
    pub reverb_time_s: f32,
    pub hf_ratio: f32,
    pub num_delays: u32,
    pub dry_level_db: f32,
    pub wet_level_db: f32,
    pub pre_delay_s: f32,
    pub process_lfe: bool,
    pub delay_lengths_mode: u32,
    pub custom_delay_lengths_ms: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoomVerbParams {
    pub decay_time_s: f32,
    pub hf_damping: f32,
    pub diffusion: f32,
    pub stereo_width: f32,
    pub filter_bands: [EqBandParams; 3],
    pub front_level_db: f32,
    pub rear_level_db: f32,
    pub center_level_db: f32,
    pub lfe_level_db: f32,
    pub dry_level_db: f32,
    pub er_level_db: f32,
    pub reverb_level_db: f32,
    pub enable_early_reflections: bool,
    pub er_pattern: u32,
    pub reverb_delay_ms: f32,
    pub room_size: f32,
    pub er_front_back_delay_ms: f32,
    pub density: f32,
    pub room_shape: f32,
    pub num_reverb_units: u32,
    pub enable_tone_controls: bool,
    pub filter_pos: [u32; 3],
    pub input_center_level_db: f32,
    pub input_lfe_level_db: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PitchShifterParams {
    pub input_type: u32,
    pub dry_level_db: f32,
    pub wet_level_db: f32,
    pub delay_time_ms: f32,
    pub process_lfe: bool,
    pub sync_dry: bool,
    pub pitch_cents: f32,
    pub filter_type: u32,
    pub filter_gain_db: f32,
    pub filter_freq_hz: f32,
    pub filter_q: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HarmonizerVoiceParams {
    pub enable: bool,
    pub pitch_cents: f32,
    pub gain_db: f32,
    pub filter_type: u32,
    pub filter_gain_db: f32,
    pub filter_freq_hz: f32,
    pub filter_q: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarmonizerParams {
    pub voices: [HarmonizerVoiceParams; 2],
    pub input_type: u32,
    pub dry_level_db: f32,
    pub wet_level_db: f32,
    pub window_size: u32,
    pub process_lfe: bool,
    pub sync_dry: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeStretchParams {
    pub window_size: u32,
    pub time_stretch: f32,
    pub time_stretch_random: f32,
    pub pitch_shift_cents: f32,
    pub pitch_shift_random: f32,
    pub output_gain_db: f32,
    pub quality: f32,
    pub stereo_proc: u16,
    pub enable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvolutionReverbParams {
    pub pre_delay_ms: f32,
    pub front_rear_delay_ms: f32,
    pub stereo_width: f32,
    pub input_center_level_db: f32,
    pub input_lfe_level_db: f32,
    pub input_stereo_width: f32,
    pub front_level_db: f32,
    pub rear_level_db: f32,
    pub center_level_db: f32,
    pub lfe_level_db: f32,
    pub dry_level_db: f32,
    pub wet_level_db: f32,
    pub algo_type: u32,
    /// `fThreshold` in dB (offset `0x34` in the 57-byte parameter block, verified at `0x1414ee9b1`):
    /// spectral silence threshold used for sparse bin skipping (not an output gain multiplier).
    pub threshold_db: f32,
    pub channel_config: u8,
}

/// Parse Convolution Reverb IR media bytes (Wwise preprocessed `00 04 02 00` bfloat16 /
/// `00 04 00 00` float32 spectral IRs, or standard `RIFF`/`WAVE` / `.wem` impulse responses).
pub fn parse_ir_media(bytes: &[u8]) -> Result<PreprocessedIr, String> {
    PreprocessedIr::parse(bytes)
}

#[derive(Debug, Clone, PartialEq)]
pub struct McDspFutzBoxParams {
    pub filter_enable: bool,
    pub lpf_mode: u32,
    pub lpf_freq_hz: f32,
    pub lpf_q: f32,
    pub hpf_mode: u32,
    pub hpf_freq_hz: f32,
    pub hpf_q: f32,
    pub dist_enable: bool,
    pub dist_mode: u32,
    pub dist_amount: f32,
    pub dist_intensity: f32,
    pub dist_rectify: f32,
    pub eq_enable: bool,
    pub eq_mode: u32,
    pub eq_freq_hz: f32,
    pub eq_gain_db: f32,
    pub eq_q: f32,
    pub gate_enable: bool,
    pub gate_thresh_db: f32,
    pub gate_range_db: f32,
    pub gate_attack_ms: f32,
    pub gate_hold_ms: f32,
    pub gate_release_ms: f32,
    pub sim_enable: bool,
    pub sim_type: u32,
    pub sim_index: u32,
    pub sim_tuning: f32,
    pub input_gain_db: f32,
    pub output_gain_db: f32,
    pub balance_pct: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrashDistortionStage {
    pub input_gain_db: f32,
    pub overdrive: f32,
    pub trash: f32,
    pub mix_pct: f32,
    pub output_gain_db: f32,
    pub algo_type: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IzotopeTrashParams {
    pub stages: [TrashDistortionStage; 2],
    pub filter_freq_hz: f32,
    pub filter_q: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MasteringSuiteParams {
    pub eq_enable: bool,
    pub comp_enable: bool,
    pub width_enable: bool,
    pub limiter_enable: bool,
    pub eq_bands: [EqBandParams; 6],
    pub stereo_width: f32,
    pub comp_threshold_db: f32,
    pub comp_ratio: f32,
    pub comp_attack_s: f32,
    pub comp_release_s: f32,
    pub comp_makeup_db: f32,
    pub limiter_threshold_db: f32,
    pub limiter_release_s: f32,
    pub limiter_ceiling_db: f32,
    pub master_output_gain_db: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QAnalyzerParams {
    pub labels: [String; 4],
    pub enabled: [bool; 4],
    pub thresholds_db: [f32; 4],
    pub window_s: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FxParams {
    Gain(GainParams),
    ParametricEq(ParametricEqParams),
    Delay(DelayParams),
    StereoDelay(StereoDelayParams),
    Dynamics(DynamicsParams),
    Meter(MeterParams),
    Tremolo(TremoloParams),
    Flanger(FlangerParams),
    GuitarDistortion(GuitarDistortionParams),
    MatrixReverb(MatrixReverbParams),
    RoomVerb(RoomVerbParams),
    PitchShifter(PitchShifterParams),
    Harmonizer(HarmonizerParams),
    TimeStretch(TimeStretchParams),
    ConvolutionReverb(ConvolutionReverbParams),
    McDspFutzBox(McDspFutzBoxParams),
    IzotopeTrash(IzotopeTrashParams),
    MasteringSuite(MasteringSuiteParams),
    QAnalyzer(QAnalyzerParams),
    /// Non-built-in or pass-through FX plugin.
    Passthrough { plugin_id: u32, raw_len: usize },
}

struct ByteReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn u8(&mut self) -> Result<u8, String> {
        if self.pos + 1 > self.buf.len() {
            return Err(format!("unexpected EOF at {} (len {})", self.pos, self.buf.len()));
        }
        let v = self.buf[self.pos];
        self.pos += 1;
        Ok(v)
    }

    fn bool(&mut self) -> Result<bool, String> {
        Ok(self.u8()? != 0)
    }

    fn u16(&mut self) -> Result<u16, String> {
        if self.pos + 2 > self.buf.len() {
            return Err(format!("unexpected EOF at {} (len {})", self.pos, self.buf.len()));
        }
        let v = u16::from_le_bytes(self.buf[self.pos..self.pos + 2].try_into().unwrap());
        self.pos += 2;
        Ok(v)
    }

    fn u32(&mut self) -> Result<u32, String> {
        if self.pos + 4 > self.buf.len() {
            return Err(format!("unexpected EOF at {} (len {})", self.pos, self.buf.len()));
        }
        let v = u32::from_le_bytes(self.buf[self.pos..self.pos + 4].try_into().unwrap());
        self.pos += 4;
        Ok(v)
    }

    fn c_string(&mut self) -> Result<String, String> {
        let start = self.pos;
        while self.pos < self.buf.len() && self.buf[self.pos] != 0 {
            self.pos += 1;
        }
        if self.pos >= self.buf.len() {
            return Err(format!("unterminated C string starting at {start}"));
        }
        let s = String::from_utf8_lossy(&self.buf[start..self.pos]).into_owned();
        self.pos += 1;
        Ok(s)
    }

    fn f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_bits(self.u32()?))
    }

    fn eq_band(&mut self) -> Result<EqBandParams, String> {
        let filter_type = EqFilterType::from_u32(self.u32()?);
        let gain_db = self.f32()?;
        let freq_hz = self.f32()?;
        let q_factor = self.f32()?;
        let on_off = self.bool()?;
        Ok(EqBandParams {
            filter_type,
            gain_db,
            freq_hz,
            q_factor,
            on_off,
        })
    }
}

/// Parse the binary parameter block of an [`FxBase`](crate::bnk::hirc::FxBase) (`FxCustom` or
/// `FxShareSet`) in Wwise v145.
pub fn parse_fx_params(plugin_id: u32, bytes: &[u8]) -> Result<FxParams, String> {
    let mut r = ByteReader::new(bytes);
    match plugin_id {
        plugin_id::GAIN => {
            let fullband_gain_db = r.f32()?;
            let lfe_gain_db = r.f32()?;
            Ok(FxParams::Gain(GainParams {
                fullband_gain_db,
                lfe_gain_db,
            }))
        }
        plugin_id::PARAMETRIC_EQ => {
            let b0 = r.eq_band()?;
            let b1 = r.eq_band()?;
            let b2 = r.eq_band()?;
            let output_level_db = r.f32()?;
            let process_lfe = r.bool()?;
            Ok(FxParams::ParametricEq(ParametricEqParams {
                bands: [b0, b1, b2],
                output_level_db,
                process_lfe,
            }))
        }
        plugin_id::DELAY => {
            let delay_time_s = r.f32()?;
            let feedback = r.f32()?;
            let wet_dry_mix = r.f32()?;
            let output_level_db = r.f32()?;
            let feedback_enabled = r.bool()?;
            let process_lfe = r.bool()?;
            Ok(FxParams::Delay(DelayParams {
                delay_time_s,
                feedback,
                wet_dry_mix,
                output_level_db,
                feedback_enabled,
                process_lfe,
            }))
        }
        plugin_id::STEREO_DELAY => {
            let mut channels = [StereoDelayChannelParams {
                input_type: 0,
                delay_time_s: 0.0,
                feedback_db: -96.3,
                crossfeed_db: -96.3,
            }; 2];
            for ch in &mut channels {
                ch.input_type = r.u32()?;
                ch.delay_time_s = r.f32()?;
                ch.feedback_db = r.f32()?;
                ch.crossfeed_db = r.f32()?;
            }
            let filter_type = r.u32()?;
            let filter_gain_db = r.f32()?;
            let filter_freq_hz = r.f32()?;
            let filter_q = r.f32()?;
            let dry_level_db = r.f32()?;
            let wet_level_db = r.f32()?;
            let front_rear_balance = r.f32()?;
            let enable_feedback = r.bool()?;
            let enable_crossfeed = r.bool()?;
            Ok(FxParams::StereoDelay(StereoDelayParams {
                channels,
                filter_type,
                filter_gain_db,
                filter_freq_hz,
                filter_q,
                dry_level_db,
                wet_level_db,
                front_rear_balance,
                enable_feedback,
                enable_crossfeed,
            }))
        }
        plugin_id::COMPRESSOR | plugin_id::EXPANDER | plugin_id::PEAK_LIMITER => {
            let kind = match plugin_id {
                plugin_id::COMPRESSOR => DynamicsKind::Compressor,
                plugin_id::EXPANDER => DynamicsKind::Expander,
                _ => DynamicsKind::PeakLimiter,
            };
            let threshold_db = r.f32()?;
            let ratio = r.f32()?;
            let attack_or_lookahead_s = r.f32()?;
            let release_s = r.f32()?;
            let output_level_db = r.f32()?;
            let process_lfe = r.bool()?;
            let channel_link = r.bool()?;
            Ok(FxParams::Dynamics(DynamicsParams {
                kind,
                threshold_db,
                ratio,
                attack_or_lookahead_s,
                release_s,
                output_level_db,
                process_lfe,
                channel_link,
            }))
        }
        plugin_id::METER => {
            let attack_s = r.f32()?;
            let release_s = r.f32()?;
            let min_db = r.f32()?;
            let max_db = r.f32()?;
            let hold_s = r.f32()?;
            let infinite_hold = r.bool()?;
            let mode = r.u8()?;
            let scope = r.u8()?;
            let apply_downstream_volume = r.bool()?;
            let game_param_id = r.u32()?;
            Ok(FxParams::Meter(MeterParams {
                attack_s,
                release_s,
                min_db,
                max_db,
                hold_s,
                infinite_hold,
                mode,
                scope,
                apply_downstream_volume,
                game_param_id,
            }))
        }
        plugin_id::TREMOLO => {
            let mod_depth = r.f32()?;
            let lfo_freq_hz = r.f32()?;
            let lfo_waveform = r.u32()?;
            let lfo_smooth = r.f32()?;
            let lfo_pwm = r.f32()?;
            let phase_offset_deg = r.f32()?;
            let phase_mode = r.u32()?;
            let phase_spread_deg = r.f32()?;
            let output_level_db = r.f32()?;
            let process_center = r.bool()?;
            let process_lfe = r.bool()?;
            Ok(FxParams::Tremolo(TremoloParams {
                mod_depth,
                lfo_freq_hz,
                lfo_waveform,
                lfo_smooth,
                lfo_pwm,
                phase_offset_deg,
                phase_mode,
                phase_spread_deg,
                output_level_db,
                process_center,
                process_lfe,
            }))
        }
        plugin_id::FLANGER => {
            let delay_time_ms = r.f32()?;
            let dry_level = r.f32()?;
            let ffwd_level = r.f32()?;
            let fback_level = r.f32()?;
            let mod_depth = r.f32()?;
            let lfo_freq_hz = r.f32()?;
            let lfo_waveform = r.u32()?;
            let lfo_smooth = r.f32()?;
            let lfo_pwm = r.f32()?;
            let phase_offset_deg = r.f32()?;
            let phase_mode = r.u32()?;
            let phase_spread_deg = r.f32()?;
            let output_level_db = r.f32()?;
            let wet_dry_mix = r.f32()?;
            let enable_lfo = r.bool()?;
            let process_center = r.bool()?;
            let process_lfe = r.bool()?;
            Ok(FxParams::Flanger(FlangerParams {
                delay_time_ms,
                dry_level,
                ffwd_level,
                fback_level,
                mod_depth,
                lfo_freq_hz,
                lfo_waveform,
                lfo_smooth,
                lfo_pwm,
                phase_offset_deg,
                phase_mode,
                phase_spread_deg,
                output_level_db,
                wet_dry_mix,
                enable_lfo,
                process_center,
                process_lfe,
            }))
        }
        plugin_id::GUITAR_DISTORTION => {
            let pre_eq = [r.eq_band()?, r.eq_band()?, r.eq_band()?];
            let post_eq = [r.eq_band()?, r.eq_band()?, r.eq_band()?];
            let distortion_type = r.u32()?;
            let drive = r.f32()?;
            let tone = r.f32()?;
            let rectification = r.f32()?;
            let output_level_db = r.f32()?;
            let wet_dry_mix = r.f32()?;
            Ok(FxParams::GuitarDistortion(GuitarDistortionParams {
                pre_eq,
                post_eq,
                distortion_type,
                drive,
                tone,
                rectification,
                output_level_db,
                wet_dry_mix,
            }))
        }
        plugin_id::MATRIX_REVERB => {
            let reverb_time_s = r.f32()?;
            let hf_ratio = r.f32()?;
            let num_delays = r.u32()?;
            let dry_level_db = r.f32()?;
            let wet_level_db = r.f32()?;
            let pre_delay_s = r.f32()?;
            let process_lfe = r.bool()?;
            let delay_lengths_mode = r.u32()?;
            let mut custom_delay_lengths_ms = Vec::new();
            if delay_lengths_mode == 1 {
                for _ in 0..num_delays.min(32) {
                    custom_delay_lengths_ms.push(r.f32()?);
                }
            }
            Ok(FxParams::MatrixReverb(MatrixReverbParams {
                reverb_time_s,
                hf_ratio,
                num_delays,
                dry_level_db,
                wet_level_db,
                pre_delay_s,
                process_lfe,
                delay_lengths_mode,
                custom_delay_lengths_ms,
            }))
        }
        plugin_id::ROOM_VERB => {
            let decay_time_s = r.f32()?;
            let hf_damping = r.f32()?;
            let diffusion = r.f32()?;
            let stereo_width = r.f32()?;
            let f1_gain = r.f32()?;
            let f1_freq = r.f32()?;
            let f1_q = r.f32()?;
            let f2_gain = r.f32()?;
            let f2_freq = r.f32()?;
            let f2_q = r.f32()?;
            let f3_gain = r.f32()?;
            let f3_freq = r.f32()?;
            let f3_q = r.f32()?;
            let front_level_db = r.f32()?;
            let rear_level_db = r.f32()?;
            let center_level_db = r.f32()?;
            let lfe_level_db = r.f32()?;
            let dry_level_db = r.f32()?;
            let er_level_db = r.f32()?;
            let reverb_level_db = r.f32()?;
            let enable_early_reflections = r.bool()?;
            let er_pattern = r.u32()?;
            let reverb_delay_ms = r.f32()?;
            let room_size = r.f32()?;
            let er_front_back_delay_ms = r.f32()?;
            let density = r.f32()?;
            let room_shape = r.f32()?;
            let num_reverb_units = r.u32()?;
            let enable_tone_controls = r.bool()?;
            let f1_pos = r.u32()?;
            let f1_curve = r.u32()?;
            let f2_pos = r.u32()?;
            let f2_curve = r.u32()?;
            let f3_pos = r.u32()?;
            let f3_curve = r.u32()?;
            let input_center_level_db = r.f32()?;
            let input_lfe_level_db = r.f32()?;
            // Remaining 44 bytes are AlgorithmTunings (11 floats)
            let map_curve = |c: u32| -> EqFilterType {
                match c {
                    0 => EqFilterType::LowShelf,
                    1 => EqFilterType::Peaking,
                    _ => EqFilterType::HighShelf,
                }
            };
            let filter_bands = [
                EqBandParams {
                    filter_type: map_curve(f1_curve),
                    gain_db: f1_gain,
                    freq_hz: f1_freq,
                    q_factor: f1_q,
                    on_off: enable_tone_controls && f1_pos != 0,
                },
                EqBandParams {
                    filter_type: map_curve(f2_curve),
                    gain_db: f2_gain,
                    freq_hz: f2_freq,
                    q_factor: f2_q,
                    on_off: enable_tone_controls && f2_pos != 0,
                },
                EqBandParams {
                    filter_type: map_curve(f3_curve),
                    gain_db: f3_gain,
                    freq_hz: f3_freq,
                    q_factor: f3_q,
                    on_off: enable_tone_controls && f3_pos != 0,
                },
            ];
            Ok(FxParams::RoomVerb(RoomVerbParams {
                decay_time_s,
                hf_damping,
                diffusion,
                stereo_width,
                filter_bands,
                front_level_db,
                rear_level_db,
                center_level_db,
                lfe_level_db,
                dry_level_db,
                er_level_db,
                reverb_level_db,
                enable_early_reflections,
                er_pattern,
                reverb_delay_ms,
                room_size,
                er_front_back_delay_ms,
                density,
                room_shape,
                num_reverb_units,
                enable_tone_controls,
                filter_pos: [f1_pos, f2_pos, f3_pos],
                input_center_level_db,
                input_lfe_level_db,
            }))
        }
        plugin_id::PITCH_SHIFTER => {
            let input_type = r.u32()?;
            let dry_level_db = r.f32()?;
            let wet_level_db = r.f32()?;
            let delay_time_ms = r.f32()?;
            let process_lfe = r.bool()?;
            let sync_dry = r.bool()?;
            let pitch_cents = r.f32()?;
            let filter_type = r.u32()?;
            let filter_gain_db = r.f32()?;
            let filter_freq_hz = r.f32()?;
            let filter_q = r.f32()?;
            Ok(FxParams::PitchShifter(PitchShifterParams {
                input_type,
                dry_level_db,
                wet_level_db,
                delay_time_ms,
                process_lfe,
                sync_dry,
                pitch_cents,
                filter_type,
                filter_gain_db,
                filter_freq_hz,
                filter_q,
            }))
        }
        plugin_id::HARMONIZER => {
            let mut voices = [HarmonizerVoiceParams {
                enable: false,
                pitch_cents: 0.0,
                gain_db: 0.0,
                filter_type: 0,
                filter_gain_db: 0.0,
                filter_freq_hz: 1000.0,
                filter_q: 1.0,
            }; 2];
            for v in &mut voices {
                v.enable = r.bool()?;
                v.pitch_cents = r.f32()?;
                v.gain_db = r.f32()?;
                v.filter_type = r.u32()?;
                v.filter_gain_db = r.f32()?;
                v.filter_freq_hz = r.f32()?;
                v.filter_q = r.f32()?;
            }
            let input_type = r.u32()?;
            let dry_level_db = r.f32()?;
            let wet_level_db = r.f32()?;
            let window_size = r.u32()?;
            let process_lfe = r.bool()?;
            let sync_dry = r.bool()?;
            Ok(FxParams::Harmonizer(HarmonizerParams {
                voices,
                input_type,
                dry_level_db,
                wet_level_db,
                window_size,
                process_lfe,
                sync_dry,
            }))
        }
        plugin_id::TIME_STRETCH => {
            let window_size = r.u32()?;
            let time_stretch = r.f32()?;
            let time_stretch_random = r.f32()?;
            let pitch_shift_cents = r.f32()?;
            let pitch_shift_random = r.f32()?;
            let output_gain_db = r.f32()?;
            let quality = r.f32()?;
            let stereo_proc = r.u16()?;
            let enable = r.u16()? == 1;
            Ok(FxParams::TimeStretch(TimeStretchParams {
                window_size,
                time_stretch,
                time_stretch_random,
                pitch_shift_cents,
                pitch_shift_random,
                output_gain_db,
                quality,
                stereo_proc,
                enable,
            }))
        }
        plugin_id::CONVOLUTION_REVERB => {
            let pre_delay_ms = r.f32()?;
            let front_rear_delay_ms = r.f32()?;
            let stereo_width = r.f32()?;
            let input_center_level_db = r.f32()?;
            let input_lfe_level_db = r.f32()?;
            let input_stereo_width = if bytes.len() >= 0x34 { r.f32()? } else { 180.0 };
            let front_level_db = r.f32()?;
            let rear_level_db = r.f32()?;
            let center_level_db = r.f32()?;
            let lfe_level_db = r.f32()?;
            let dry_level_db = r.f32()?;
            let wet_level_db = r.f32()?;
            let algo_type = r.u32()?;
            let threshold_db = if bytes.len() >= 0x38 { r.f32()? } else { -96.3 };
            let channel_config = if bytes.len() >= 0x39 { r.u8()? } else { 2 };
            Ok(FxParams::ConvolutionReverb(ConvolutionReverbParams {
                pre_delay_ms,
                front_rear_delay_ms,
                stereo_width,
                input_center_level_db,
                input_lfe_level_db,
                input_stereo_width,
                front_level_db,
                rear_level_db,
                center_level_db,
                lfe_level_db,
                dry_level_db,
                wet_level_db,
                algo_type,
                threshold_db,
                channel_config,
            }))
        }
        plugin_id::MCDSP_FUTZBOX => {
            // 139 bytes total (FUN_1417c27f0)
            let filter_enable = r.bool()?;
            let lpf_mode = r.u32()?;
            let lpf_freq_hz = r.f32()?;
            let lpf_q = r.f32()?;
            let hpf_mode = r.u32()?;
            let hpf_freq_hz = r.f32()?;
            let hpf_q = r.f32()?;

            let dist_enable = r.bool()?;
            let dist_mode = r.u32()?;
            let dist_amount = r.f32()?;
            let dist_intensity = r.f32()?;
            let dist_rectify = r.f32()?;

            let eq_enable = r.bool()?;
            let eq_mode = r.u32()?;
            let eq_freq_hz = r.f32()?;
            let eq_gain_db = r.f32()?;
            let eq_q = r.f32()?;

            // Noise generator block (0x3b..0x5d: 1 bool + 6 f32s + 1 bool + 2 f32s = 34 bytes)
            let _noise_enable = r.bool()?;
            for _ in 0..6 {
                let _ = r.f32()?;
            }
            let _noise_duck_enable = r.bool()?;
            let _ = r.f32()?;
            let _ = r.f32()?;

            // Gate block (0x5d..0x72: 1 bool + 5 f32s = 21 bytes)
            let gate_enable = r.bool()?;
            let gate_thresh_db = r.f32()?;
            let gate_range_db = r.f32()?;
            let gate_attack_ms = r.f32()?;
            let gate_hold_ms = r.f32()?;
            let gate_release_ms = r.f32()?;

            // SIM block (0x72..0x7f: 1 bool + 2 u32s + 1 f32 = 13 bytes)
            let sim_enable = r.bool()?;
            let sim_type = r.u32()?;
            let sim_index = r.u32()?;
            let sim_tuning = r.f32()?;

            // Global block (0x7f..0x8b: 3 f32s = 12 bytes)
            let input_gain_db = r.f32()?;
            let output_gain_db = r.f32()?;
            let balance_pct = r.f32()?;

            Ok(FxParams::McDspFutzBox(McDspFutzBoxParams {
                filter_enable,
                lpf_mode,
                lpf_freq_hz,
                lpf_q,
                hpf_mode,
                hpf_freq_hz,
                hpf_q,
                dist_enable,
                dist_mode,
                dist_amount,
                dist_intensity,
                dist_rectify,
                eq_enable,
                eq_mode,
                eq_freq_hz,
                eq_gain_db,
                eq_q,
                gate_enable,
                gate_thresh_db,
                gate_range_db,
                gate_attack_ms,
                gate_hold_ms,
                gate_release_ms,
                sim_enable,
                sim_type,
                sim_index,
                sim_tuning,
                input_gain_db,
                output_gain_db,
                balance_pct,
            }))
        }
        plugin_id::IZOTOPE_TRASH_DISTORTION => {
            // 48 bytes (12 words, FUN_14178d360): 2 cascaded 6-word stages
            let mut stages = [TrashDistortionStage {
                input_gain_db: 0.0,
                overdrive: 5.0,
                trash: 0.5,
                mix_pct: 100.0,
                output_gain_db: 0.0,
                algo_type: 0,
            }; 2];
            for st in &mut stages {
                st.input_gain_db = r.f32()?;
                st.overdrive = r.f32()?;
                st.trash = r.f32()?;
                st.mix_pct = r.f32()?;
                st.output_gain_db = r.f32()?;
                st.algo_type = r.u32()?;
            }
            Ok(FxParams::IzotopeTrash(IzotopeTrashParams {
                stages,
                filter_freq_hz: 8000.0,
                filter_q: 0.707,
            }))
        }
        plugin_id::IZOTOPE_TRASH_MULTIBAND
        | plugin_id::IZOTOPE_TRASH_BOX_MODELER
        | plugin_id::IZOTOPE_TRASH_DELAY
        | plugin_id::IZOTOPE_TRASH_DYNAMICS
        | plugin_id::IZOTOPE_TRASH_FILTERS => {
            // Other iZotope Trash family modules mapped into the 2-stage Trash processor
            let mut stages = [TrashDistortionStage {
                input_gain_db: 0.0,
                overdrive: 4.0,
                trash: 0.4,
                mix_pct: 100.0,
                output_gain_db: 0.0,
                algo_type: 0,
            }; 2];
            if bytes.len() >= 24 {
                stages[0].input_gain_db = r.f32()?.clamp(-24.0, 24.0);
                stages[0].overdrive = r.f32()?.clamp(0.0, 20.0);
                stages[0].trash = r.f32()?.clamp(0.0, 1.0);
                stages[0].mix_pct = r.f32()?.clamp(0.0, 100.0);
                stages[0].output_gain_db = r.f32()?.clamp(-24.0, 24.0);
                stages[0].algo_type = r.u32()?;
            }
            Ok(FxParams::IzotopeTrash(IzotopeTrashParams {
                stages,
                filter_freq_hz: 6000.0,
                filter_q: 0.707,
            }))
        }
        plugin_id::MASTERING_SUITE => {
            // 304 bytes (0x130, FUN_1417b72e0)
            let eq_enable = r.bool()?;
            let comp_enable = r.bool()?;
            let width_enable = r.bool()?;
            let limiter_enable = r.bool()?;
            let _num_eq_bands = r.u32()?;
            let mut band_on = [false; 6];
            for on in &mut band_on {
                *on = r.bool()?;
            }
            let map_ms_filter = |curve: u32| -> EqFilterType {
                match curve {
                    0 => EqFilterType::Peaking,
                    1 => EqFilterType::LowShelf,
                    2 => EqFilterType::HighShelf,
                    3 => EqFilterType::LowPass,
                    4 => EqFilterType::HighPass,
                    _ => EqFilterType::Notch,
                }
            };
            let mut eq_bands = [EqBandParams::default(); 6];
            for (i, b) in eq_bands.iter_mut().enumerate() {
                let ftype = r.u32()?;
                let freq_hz = r.f32()?;
                let q_factor = r.f32()?;
                let gain_db = r.f32()?;
                *b = EqBandParams {
                    filter_type: map_ms_filter(ftype),
                    gain_db,
                    freq_hz,
                    q_factor: q_factor.max(0.1),
                    on_off: eq_enable && band_on[i],
                };
            }
            // Stereo width & pan (0x6e..0x7a: 3 f32s)
            let stereo_width = r.f32()?;
            let _pan_l = r.f32()?;
            let _pan_r = r.f32()?;
            // Compressor flags & crossovers (0x7a..0x8b: 5 bools + 3 f32s)
            let _comp_link = r.bool()?;
            let mut comp_band_on = [false; 4];
            for on in &mut comp_band_on {
                *on = r.bool()?;
            }
            let _xover_0 = r.f32()?;
            let _xover_1 = r.f32()?;
            let _xover_2 = r.f32()?;
            // First compressor band (0x8b..0xa3: 6 f32s), plus bands 1..3 (18 f32s)
            let comp_threshold_db = r.f32()?;
            let comp_ratio = r.f32()?.max(1.0);
            let comp_attack_s = (r.f32()? * 0.001).max(0.0001);
            let comp_release_s = (r.f32()? * 0.001).max(0.001);
            let _comp_knee = r.f32()?;
            let comp_makeup_db = r.f32()?;
            for _ in 0..(3 * 6 + 12) {
                let _ = r.f32()?;
            }
            // Limiter (0x11b..0x130: 5 f32s + 1 u8)
            let limiter_threshold_db = r.f32()?;
            let _limiter_knee = r.f32()?;
            let limiter_release_s = (r.f32()? * 0.001).max(0.001);
            let limiter_ceiling_db = r.f32()?;
            let master_output_gain_db = r.f32()?;
            let _limiter_link = r.bool()?;

            Ok(FxParams::MasteringSuite(MasteringSuiteParams {
                eq_enable,
                comp_enable,
                width_enable,
                limiter_enable,
                eq_bands,
                stereo_width,
                comp_threshold_db,
                comp_ratio,
                comp_attack_s,
                comp_release_s,
                comp_makeup_db,
                limiter_threshold_db,
                limiter_release_s,
                limiter_ceiling_db,
                master_output_gain_db,
            }))
        }
        plugin_id::Q_ANALYZER => {
            // 4 C-strings + 4 bools + 4 f32 thresholds + 1 f32 window (FUN_1418319a0)
            let labels = [r.c_string()?, r.c_string()?, r.c_string()?, r.c_string()?];
            let enabled = [r.bool()?, r.bool()?, r.bool()?, r.bool()?];
            let thresholds_db = [r.f32()?, r.f32()?, r.f32()?, r.f32()?];
            let window_s = r.f32()?;
            Ok(FxParams::QAnalyzer(QAnalyzerParams {
                labels,
                enabled,
                thresholds_db,
                window_s,
            }))
        }
        _ => Ok(FxParams::Passthrough {
            plugin_id,
            raw_len: bytes.len(),
        }),
    }
}
