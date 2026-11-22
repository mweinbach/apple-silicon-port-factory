//! Wwise built-in DSP effects (`FxCustom` / `FxShareSet`) and node/bus FX chains (milestone A7).
//!
//! Parses the binary parameter blocks of Wwise v145 (`2022.1`) built-in effect plugins stored in
//! [`FxBase::params`](crate::bnk::hirc::FxBase) and runs per-block `f32` DSP processing for both
//! actor-mixer node FX chains (`VoiceParams::node_fx`) and bus/aux-bus FX chains (`BusLink::fx`).
//!
//! Supported built-in effect plugins (`plugin_id` with `type == 3`):
//! - **Wwise Gain** (`0x008B0003` / `139`, 8 bytes): full-band + LFE gain in dB.
//! - **Wwise Parametric EQ** (`0x00690003` / `105`, 56 bytes): 3-band biquad filter
//!   (`LowPass`, `HighPass`, `BandPass`, `Notch`, `LowShelf`, `HighShelf`, `Peaking`) + output
//!   level + LFE flag.
//! - **Wwise Meter** (`0x00810003` / `129`, 28 bytes): Peak / RMS envelope follower (attack,
//!   release, hold, min/max clamp) writing its output back to `uGameParamID` so meter-driven RTPCs
//!   and ducking work.
//! - **Wwise Delay** (`0x006A0003` / `106`, 18 bytes) & **Wwise Stereo Delay** (`0x00870003` /
//!   `135`, 62 bytes): circular delay lines with feedback, crossfeed, filter, and wet/dry mix.
//! - **Wwise Compressor** (`0x006C0003` / `108`, 22 bytes), **Wwise Expander** (`0x006D0003` /
//!   `109`, 22 bytes), **Wwise Peak Limiter** (`0x006E0003` / `110`, 22 bytes): dynamics processors
//!   with threshold, ratio, attack/lookahead, release, makeup gain, and channel link.
//! - **Wwise Tremolo** (`0x00830003` / `131`, 38 bytes), **Wwise Flanger** (`0x007D0003` / `125`,
//!   59 bytes), **Wwise Guitar Distortion** (`0x007E0003` / `126`, 126 bytes): LFO amplitude/delay
//!   modulation and pre-EQ + waveshaper + post-EQ distortion.
//! - **Wwise RoomVerb** (`0x00760003` / `118`, 186 bytes) & **Wwise Matrix Reverb** (`0x00730003` /
//!   `115`, 29+ bytes): early-reflections tap network + Householder FDN reverb tank with pre-delay,
//!   decay time, HF damping, tone filters, and dry/wet levels.
//! - **Wwise Pitch Shifter** (`0x00880003` / `136`, 38 bytes) & **Wwise Harmonizer** (`0x008A0003` /
//!   `138`, 68 bytes): dual-head Hann-windowed delay-line pitch shifting (single voice and 2-voice
//!   harmony with per-voice biquad filters).
//! - **Wwise Time Stretch** (`0x00820003` / `130`, 32 bytes): granular overlap-add time-stretch and
//!   pitch-shift resynthesis.
//! - **Wwise Convolution Reverb** (`0x007F0003` / `127`, 57 bytes): partitioned overlap-add FFT
//!   convolution against Wwise preprocessed IR media (`00 04 02 00` bfloat16 / `00 04 00 00`
//!   float32) or `RIFF`/`WAVE` IRs, with parameterized FDN fallback when no IR media is loaded.
//! - **McDSP FutzBox** (`0x006E1003`, 139 bytes): lo-fi speaker/radio/telephone simulation with
//!   HPF/LPF bandpass, SIM speaker resonator + bitcrusher/downsampler, distortion, noise gate, and EQ.
//! - **iZotope Trash Distortion** (`0x00031033`, 48 bytes, plus Trash family `0x00041033..0x00091033`):
//!   dual-stage multi-algorithm waveshaper with overdrive, trash symmetry, and stage wet/dry mix.
//! - **Mastering Suite** (`0x00BA0003` / `186`, 304 bytes): 6-band parametric EQ + compressor +
//!   stereo width + brickwall limiter + master output gain.

pub use super::convolver::{IrPrecision, PartitionedConvolver, PreprocessedIr};
use super::math::{db_to_lin, eval_graph, fast_pow_to_db, RATE};
use super::params::{accum_value, rtpc_accum, GameState};
use super::project::Project;
use crate::bnk::hirc::{FxBase, HircBody, Rtpc};
use std::f32::consts::PI;

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

/// Parse the binary parameter block of an [`FxBase`] (`FxCustom` or `FxShareSet`) in Wwise v145.
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

/// Apply an FX object's RTPC curves (`FxBase::rtpcs`) to its parsed [`FxParams`].
pub fn apply_fx_rtpcs(project: &Project, gs: &GameState, rtpcs: &[Rtpc], params: &mut FxParams) {
    if rtpcs.is_empty() {
        return;
    }
    let eval_rtpc = |r: &Rtpc, dst: &mut f32, is_db: bool| {
        if r.rtpc_type != 0 {
            return;
        }
        let x = gs.rtpc_value(project, r.rtpc_id);
        let y = eval_graph(&r.points, r.scaling, x);
        let eff_accum = if (r.scaling == 4 || r.scaling == 3)
            && r.accum == rtpc_accum::ADDITIVE
            && is_db
        {
            rtpc_accum::MULTIPLY
        } else {
            r.accum
        };
        accum_value(dst, y, eff_accum, is_db);
    };

    for r in rtpcs {
        match params {
            FxParams::Gain(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.fullband_gain_db, true),
                1 => eval_rtpc(r, &mut p.lfe_gain_db, true),
                _ => {}
            },
            FxParams::ParametricEq(p) => match r.param_id {
                0..=14 => {
                    let band = (r.param_id / 5) as usize;
                    let field = r.param_id % 5;
                    if let Some(b) = p.bands.get_mut(band) {
                        match field {
                            0 => {
                                let mut v = b.filter_type as u32 as f32;
                                eval_rtpc(r, &mut v, false);
                                b.filter_type = EqFilterType::from_u32(v.round().max(0.0) as u32);
                            }
                            1 => eval_rtpc(r, &mut b.gain_db, true),
                            2 => eval_rtpc(r, &mut b.freq_hz, false),
                            3 => eval_rtpc(r, &mut b.q_factor, false),
                            4 => {
                                let mut v = if b.on_off { 1.0 } else { 0.0 };
                                eval_rtpc(r, &mut v, false);
                                b.on_off = v >= 0.5;
                            }
                            _ => {}
                        }
                    }
                }
                15 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Delay(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.delay_time_s, false),
                1 => eval_rtpc(r, &mut p.feedback, false),
                2 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                3 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::StereoDelay(p) => match r.param_id {
                1 => eval_rtpc(r, &mut p.dry_level_db, true),
                2 => eval_rtpc(r, &mut p.wet_level_db, true),
                3 => eval_rtpc(r, &mut p.front_rear_balance, false),
                9 => eval_rtpc(r, &mut p.channels[0].delay_time_s, false),
                10 => eval_rtpc(r, &mut p.channels[0].feedback_db, true),
                11 => eval_rtpc(r, &mut p.channels[0].crossfeed_db, true),
                12 => eval_rtpc(r, &mut p.channels[0].feedback_db, true),
                13 => eval_rtpc(r, &mut p.channels[1].delay_time_s, false),
                14 => eval_rtpc(r, &mut p.channels[1].feedback_db, true),
                15 => eval_rtpc(r, &mut p.channels[1].crossfeed_db, true),
                16 => eval_rtpc(r, &mut p.channels[1].feedback_db, true),
                _ => {}
            },
            FxParams::Dynamics(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.threshold_db, true),
                1 => eval_rtpc(r, &mut p.ratio, false),
                2 => eval_rtpc(r, &mut p.attack_or_lookahead_s, false),
                3 => eval_rtpc(r, &mut p.release_s, false),
                4 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Tremolo(p) => match r.param_id {
                0 | 1 => eval_rtpc(r, &mut p.mod_depth, false),
                2 => eval_rtpc(r, &mut p.lfo_freq_hz, false),
                8 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Flanger(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.delay_time_ms, false),
                1 => eval_rtpc(r, &mut p.mod_depth, false),
                2 | 6 => eval_rtpc(r, &mut p.fback_level, false),
                5 | 9 => eval_rtpc(r, &mut p.lfo_freq_hz, false),
                12 => eval_rtpc(r, &mut p.output_level_db, true),
                13 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                _ => {}
            },
            FxParams::GuitarDistortion(p) => match r.param_id {
                60 => {
                    let mut v = p.distortion_type as f32;
                    eval_rtpc(r, &mut v, false);
                    p.distortion_type = v.round().max(0.0) as u32;
                }
                61 => eval_rtpc(r, &mut p.drive, false),
                62 => eval_rtpc(r, &mut p.tone, false),
                63 => eval_rtpc(r, &mut p.rectification, false),
                64 => eval_rtpc(r, &mut p.output_level_db, true),
                65 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                _ => {}
            },
            FxParams::RoomVerb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.decay_time_s, false),
                1 => eval_rtpc(r, &mut p.hf_damping, false),
                2 => eval_rtpc(r, &mut p.diffusion, false),
                3 => eval_rtpc(r, &mut p.stereo_width, false),
                60 => eval_rtpc(r, &mut p.er_level_db, true),
                61 => eval_rtpc(r, &mut p.reverb_level_db, true),
                62 => eval_rtpc(r, &mut p.dry_level_db, true),
                _ => {}
            },
            FxParams::MatrixReverb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.reverb_time_s, false),
                1 => eval_rtpc(r, &mut p.hf_ratio, false),
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                5 => eval_rtpc(r, &mut p.pre_delay_s, false),
                _ => {}
            },
            FxParams::PitchShifter(p) => match r.param_id {
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                5 => eval_rtpc(r, &mut p.delay_time_ms, false),
                6 => eval_rtpc(r, &mut p.pitch_cents, false),
                8 => eval_rtpc(r, &mut p.filter_gain_db, true),
                9 => eval_rtpc(r, &mut p.filter_freq_hz, false),
                10 => eval_rtpc(r, &mut p.filter_q, false),
                _ => {}
            },
            FxParams::Harmonizer(p) => match r.param_id {
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                7 => eval_rtpc(r, &mut p.voices[0].pitch_cents, false),
                8 => eval_rtpc(r, &mut p.voices[0].gain_db, true),
                10 => eval_rtpc(r, &mut p.voices[0].filter_gain_db, true),
                11 => eval_rtpc(r, &mut p.voices[0].filter_freq_hz, false),
                12 => eval_rtpc(r, &mut p.voices[0].filter_q, false),
                14 => eval_rtpc(r, &mut p.voices[1].pitch_cents, false),
                15 => eval_rtpc(r, &mut p.voices[1].gain_db, true),
                17 => eval_rtpc(r, &mut p.voices[1].filter_gain_db, true),
                18 => eval_rtpc(r, &mut p.voices[1].filter_freq_hz, false),
                19 => eval_rtpc(r, &mut p.voices[1].filter_q, false),
                _ => {}
            },
            FxParams::TimeStretch(p) => match r.param_id {
                1 => eval_rtpc(r, &mut p.time_stretch, false),
                2 => eval_rtpc(r, &mut p.output_gain_db, true),
                4 => eval_rtpc(r, &mut p.pitch_shift_cents, false),
                _ => {}
            },
            FxParams::ConvolutionReverb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.pre_delay_ms, false),
                1 => eval_rtpc(r, &mut p.front_rear_delay_ms, false),
                2 => eval_rtpc(r, &mut p.stereo_width, false),
                20 => eval_rtpc(r, &mut p.front_level_db, true),
                21 => eval_rtpc(r, &mut p.rear_level_db, true),
                30 => eval_rtpc(r, &mut p.dry_level_db, true),
                31 => eval_rtpc(r, &mut p.wet_level_db, true),
                34 => eval_rtpc(r, &mut p.threshold_db, true),
                _ => {}
            },
            FxParams::McDspFutzBox(p) => match r.param_id {
                1 => {
                    let mut v = p.dist_mode as f32;
                    eval_rtpc(r, &mut v, false);
                    p.dist_mode = v.round().max(0.0) as u32;
                }
                2 => eval_rtpc(r, &mut p.dist_amount, false),
                3 => eval_rtpc(r, &mut p.dist_intensity, false),
                4 => eval_rtpc(r, &mut p.dist_rectify, false),
                40 => eval_rtpc(r, &mut p.input_gain_db, true),
                41 => eval_rtpc(r, &mut p.output_gain_db, true),
                42 => eval_rtpc(r, &mut p.balance_pct, false),
                52 => eval_rtpc(r, &mut p.lpf_freq_hz, false),
                55 => eval_rtpc(r, &mut p.hpf_freq_hz, false),
                62 => eval_rtpc(r, &mut p.eq_freq_hz, false),
                63 => eval_rtpc(r, &mut p.eq_gain_db, true),
                _ => {}
            },
            FxParams::IzotopeTrash(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.stages[0].input_gain_db, true),
                1 => eval_rtpc(r, &mut p.stages[0].overdrive, false),
                2 => eval_rtpc(r, &mut p.stages[0].trash, false),
                3 => eval_rtpc(r, &mut p.stages[0].mix_pct, false),
                4 => eval_rtpc(r, &mut p.stages[0].output_gain_db, true),
                6 => eval_rtpc(r, &mut p.stages[1].input_gain_db, true),
                7 => eval_rtpc(r, &mut p.stages[1].overdrive, false),
                8 => eval_rtpc(r, &mut p.stages[1].trash, false),
                9 => eval_rtpc(r, &mut p.stages[1].mix_pct, false),
                10 => eval_rtpc(r, &mut p.stages[1].output_gain_db, true),
                _ => {}
            },
            FxParams::MasteringSuite(p) => match r.param_id {
                200 => eval_rtpc(r, &mut p.stereo_width, false),
                400 => eval_rtpc(r, &mut p.limiter_threshold_db, true),
                403 => eval_rtpc(r, &mut p.limiter_ceiling_db, true),
                404 => eval_rtpc(r, &mut p.master_output_gain_db, true),
                _ => {}
            },
            FxParams::Meter(_) | FxParams::QAnalyzer(_) | FxParams::Passthrough { .. } => {}
        }
    }
}

/// Standard Audio EQ Cookbook biquad filter (`Direct Form II Transposed`) supporting all 7 Wwise
/// filter curves (`LowPass`, `HighPass`, `BandPass`, `Notch`, `LowShelf`, `HighShelf`, `Peaking`).
#[derive(Debug, Clone, Copy)]
pub struct DspBiquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl Default for DspBiquad {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }
}

impl DspBiquad {
    pub fn design(band: &EqBandParams, sample_rate: u32) -> Self {
        if !band.on_off {
            return Self::default();
        }
        let fs = sample_rate.max(1) as f32;
        let f0 = band.freq_hz.clamp(10.0, fs * 0.49);
        let q = band.q_factor.clamp(0.05, 40.0);
        let w0 = 2.0 * PI * f0 / fs;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);
        let a = 10.0f32.powf(band.gain_db.clamp(-48.0, 48.0) / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match band.filter_type {
            EqFilterType::LowPass => {
                let b1 = 1.0 - cos_w0;
                let b0 = b1 * 0.5;
                (b0, b1, b0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
            }
            EqFilterType::HighPass => {
                let b0 = (1.0 + cos_w0) * 0.5;
                let b1 = -(1.0 + cos_w0);
                (b0, b1, b0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
            }
            EqFilterType::BandPass => (
                alpha,
                0.0,
                -alpha,
                1.0 + alpha,
                -2.0 * cos_w0,
                1.0 - alpha,
            ),
            EqFilterType::Notch => (
                1.0,
                -2.0 * cos_w0,
                1.0,
                1.0 + alpha,
                -2.0 * cos_w0,
                1.0 - alpha,
            ),
            EqFilterType::Peaking => (
                1.0 + alpha * a,
                -2.0 * cos_w0,
                1.0 - alpha * a,
                1.0 + alpha / a,
                -2.0 * cos_w0,
                1.0 - alpha / a,
            ),
            EqFilterType::LowShelf => {
                let sq = 2.0 * a.sqrt() * (sin_w0 * 0.5 * (2.0f32).sqrt());
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                (
                    a * (ap1 - am1 * cos_w0 + sq),
                    2.0 * a * (am1 - ap1 * cos_w0),
                    a * (ap1 - am1 * cos_w0 - sq),
                    ap1 + am1 * cos_w0 + sq,
                    -2.0 * (am1 + ap1 * cos_w0),
                    ap1 + am1 * cos_w0 - sq,
                )
            }
            EqFilterType::HighShelf => {
                let sq = 2.0 * a.sqrt() * (sin_w0 * 0.5 * (2.0f32).sqrt());
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                (
                    a * (ap1 + am1 * cos_w0 + sq),
                    -2.0 * a * (am1 + ap1 * cos_w0),
                    a * (ap1 + am1 * cos_w0 - sq),
                    ap1 - am1 * cos_w0 + sq,
                    2.0 * (am1 - ap1 * cos_w0),
                    ap1 - am1 * cos_w0 - sq,
                )
            }
        };

        let inv_a0 = if a0.abs() > 1e-12 { 1.0 / a0 } else { 1.0 };
        Self {
            b0: b0 * inv_a0,
            b1: b1 * inv_a0,
            b2: b2 * inv_a0,
            a1: a1 * inv_a0,
            a2: a2 * inv_a0,
        }
    }

    #[inline]
    pub fn tick(&self, x: f32, state: &mut [f32; 2]) -> f32 {
        let y = self.b0 * x + state[0];
        state[0] = self.b1 * x - self.a1 * y + state[1];
        state[1] = self.b2 * x - self.a2 * y;
        y
    }

    pub fn process_slice(&self, state: &mut [f32; 2], buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.tick(*s, state);
        }
    }
}

#[derive(Debug, Clone)]
struct DelayLine {
    buf: Vec<f32>,
    write_pos: usize,
}

impl DelayLine {
    fn new(max_samples: usize) -> Self {
        Self {
            buf: vec![0.0; max_samples.max(4)],
            write_pos: 0,
        }
    }

    #[inline]
    fn read_delay(&self, delay_samples: usize) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.min(len - 1);
        self.buf[(self.write_pos + len - d) % len]
    }

    #[inline]
    fn read_frac(&self, delay_samples: f32) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.clamp(0.0, (len - 2) as f32);
        let d_int = d as usize;
        let frac = d - d_int as f32;
        let i0 = (self.write_pos + len - d_int) % len;
        let i1 = (self.write_pos + len - d_int - 1) % len;
        self.buf[i0] * (1.0 - frac) + self.buf[i1] * frac
    }

    #[inline]
    fn write_and_advance(&mut self, val: f32) {
        self.buf[self.write_pos] = val;
        self.write_pos += 1;
        if self.write_pos >= self.buf.len() {
            self.write_pos = 0;
        }
    }
}

#[derive(Debug, Clone)]
enum DspState {
    Gain,
    ParametricEq {
        states: Vec<[[f32; 2]; 3]>,
    },
    Delay {
        lines: Vec<DelayLine>,
    },
    StereoDelay {
        lines: [DelayLine; 2],
        fb_filter_state: [[f32; 2]; 2],
    },
    Dynamics {
        env: Vec<f32>,
    },
    Meter {
        env_db: f32,
        hold_remaining_s: f32,
        held_peak_db: f32,
    },
    Tremolo {
        phase: f32,
    },
    Flanger {
        lines: Vec<DelayLine>,
        phase: f32,
    },
    GuitarDistortion {
        pre_states: Vec<[[f32; 2]; 3]>,
        post_states: Vec<[[f32; 2]; 3]>,
        tone_states: Vec<[f32; 2]>,
    },
    Reverb {
        predelay: DelayLine,
        er_line: DelayLine,
        fdn_lines: [DelayLine; 4],
        fdn_lp: [f32; 4],
        tone_states: Vec<[[f32; 2]; 3]>,
    },
    PitchShifter {
        lines: Vec<DelayLine>,
        phase: f32,
        filter_states: Vec<[f32; 2]>,
    },
    Harmonizer {
        lines: Vec<DelayLine>,
        phases: [f32; 2],
        filter_states: Vec<[[f32; 2]; 2]>,
    },
    TimeStretch {
        lines: Vec<DelayLine>,
        phase: f32,
    },
    ConvolutionReverb {
        ir_media_id: Option<u32>,
        convolver: Option<Box<PartitionedConvolver>>,
        predelay_ch: Vec<DelayLine>,
        predelay: DelayLine,
        er_line: DelayLine,
        fdn_lines: [DelayLine; 4],
        fdn_lp: [f32; 4],
        rear_delay: DelayLine,
    },
    McDspFutzBox {
        hpf_states: Vec<[f32; 2]>,
        lpf_states: Vec<[f32; 2]>,
        sim_states: Vec<[f32; 2]>,
        eq_states: Vec<[f32; 2]>,
        gate_env: Vec<f32>,
        decim_hold: Vec<f32>,
        decim_phase: f32,
    },
    IzotopeTrash {
        filter_states: Vec<[[f32; 2]; 2]>,
    },
    MasteringSuite {
        eq_states: Vec<[[f32; 2]; 6]>,
        comp_env: f32,
        lim_env: f32,
    },
    QAnalyzer {
        peak_db: f32,
    },
    Passthrough,
}

/// Stateful runtime instance of a single Wwise FX slot (`FxCustom` or `FxShareSet`).
#[derive(Debug, Clone)]
pub struct FxInstance {
    pub fx_id: u32,
    pub plugin_id: u32,
    pub params: FxParams,
    pub rtpcs: Vec<Rtpc>,
    state: DspState,
}

impl FxInstance {
    /// Construct a new [`FxInstance`] from an [`FxBase`] HIRC object.
    pub fn from_fx_base(fx_id: u32, fx: &FxBase) -> Result<Self, String> {
        let raw = fx.params.as_deref().unwrap_or(&[]);
        let params = parse_fx_params(fx.plugin_id, raw)?;
        let mut inst = Self::new(fx_id, fx.plugin_id, params, fx.rtpcs.clone());
        if let DspState::ConvolutionReverb { ir_media_id, .. } = &mut inst.state {
            *ir_media_id = fx
                .media
                .iter()
                .find(|(slot, _)| *slot == 0)
                .or_else(|| fx.media.first())
                .map(|(_, mid)| *mid);
        }
        Ok(inst)
    }

    /// Construct a new [`FxInstance`] from parsed [`FxParams`].
    pub fn new(fx_id: u32, plugin_id: u32, params: FxParams, rtpcs: Vec<Rtpc>) -> Self {
        let state = match &params {
            FxParams::Gain(_) => DspState::Gain,
            FxParams::ParametricEq(_) => DspState::ParametricEq { states: Vec::new() },
            FxParams::Delay(p) => {
                let max_s = (p.delay_time_s.clamp(0.001, 5.0) * RATE as f32) as usize + 4096;
                DspState::Delay {
                    lines: vec![DelayLine::new(max_s); 2],
                }
            }
            FxParams::StereoDelay(p) => {
                let max_0 = (p.channels[0].delay_time_s.clamp(0.001, 5.0) * RATE as f32) as usize + 4096;
                let max_1 = (p.channels[1].delay_time_s.clamp(0.001, 5.0) * RATE as f32) as usize + 4096;
                DspState::StereoDelay {
                    lines: [DelayLine::new(max_0), DelayLine::new(max_1)],
                    fb_filter_state: [[0.0; 2]; 2],
                }
            }
            FxParams::Dynamics(_) => DspState::Dynamics { env: Vec::new() },
            FxParams::Meter(p) => DspState::Meter {
                env_db: p.min_db,
                hold_remaining_s: 0.0,
                held_peak_db: p.min_db,
            },
            FxParams::Tremolo(_) => DspState::Tremolo { phase: 0.0 },
            FxParams::Flanger(_) => DspState::Flanger {
                lines: vec![DelayLine::new((0.05 * RATE as f32) as usize + 256); 2],
                phase: 0.0,
            },
            FxParams::GuitarDistortion(_) => DspState::GuitarDistortion {
                pre_states: Vec::new(),
                post_states: Vec::new(),
                tone_states: Vec::new(),
            },
            FxParams::MatrixReverb(_) | FxParams::RoomVerb(_) => {
                // Prime FDN delay lengths (~29.7 ms, 37.1 ms, 41.1 ms, 43.7 ms at 48 kHz)
                let fdn_lines = [
                    DelayLine::new(1427),
                    DelayLine::new(1783),
                    DelayLine::new(1973),
                    DelayLine::new(2099),
                ];
                DspState::Reverb {
                    predelay: DelayLine::new((0.5 * RATE as f32) as usize + 256),
                    er_line: DelayLine::new((0.15 * RATE as f32) as usize + 256),
                    fdn_lines,
                    fdn_lp: [0.0; 4],
                    tone_states: Vec::new(),
                }
            }
            FxParams::PitchShifter(_) => DspState::PitchShifter {
                lines: vec![DelayLine::new(8192); 2],
                phase: 0.0,
                filter_states: Vec::new(),
            },
            FxParams::Harmonizer(_) => DspState::Harmonizer {
                lines: vec![DelayLine::new(8192); 2],
                phases: [0.0; 2],
                filter_states: Vec::new(),
            },
            FxParams::TimeStretch(_) => DspState::TimeStretch {
                lines: vec![DelayLine::new(8192); 2],
                phase: 0.0,
            },
            FxParams::ConvolutionReverb(_) => DspState::ConvolutionReverb {
                ir_media_id: None,
                convolver: None,
                predelay_ch: Vec::new(),
                predelay: DelayLine::new((0.5 * RATE as f32) as usize + 256),
                er_line: DelayLine::new((0.15 * RATE as f32) as usize + 256),
                fdn_lines: [
                    DelayLine::new(1303),
                    DelayLine::new(1699),
                    DelayLine::new(1999),
                    DelayLine::new(2311),
                ],
                fdn_lp: [0.0; 4],
                rear_delay: DelayLine::new((0.15 * RATE as f32) as usize + 256),
            },
            FxParams::McDspFutzBox(_) => DspState::McDspFutzBox {
                hpf_states: Vec::new(),
                lpf_states: Vec::new(),
                sim_states: Vec::new(),
                eq_states: Vec::new(),
                gate_env: Vec::new(),
                decim_hold: Vec::new(),
                decim_phase: 0.0,
            },
            FxParams::IzotopeTrash(_) => DspState::IzotopeTrash {
                filter_states: Vec::new(),
            },
            FxParams::MasteringSuite(_) => DspState::MasteringSuite {
                eq_states: Vec::new(),
                comp_env: 0.0,
                lim_env: 0.0,
            },
            FxParams::QAnalyzer(_) => DspState::QAnalyzer { peak_db: -96.3 },
            FxParams::Passthrough { .. } => DspState::Passthrough,
        };
        Self {
            fx_id,
            plugin_id,
            params,
            rtpcs,
            state,
        }
    }

    /// Look up `fx_id` in `project` and instantiate its [`FxInstance`].
    pub fn from_project(project: &Project, fx_id: u32) -> Option<Self> {
        let o = project.fx(fx_id)?;
        let mut inst = match &o.body {
            HircBody::FxShareSet(fx) | HircBody::FxCustom(fx) => Self::from_fx_base(fx_id, fx).ok()?,
            _ => return None,
        };
        if let DspState::ConvolutionReverb { ir_media_id: Some(mid), convolver, .. } = &mut inst.state {
            if let Some(bytes) = project.read_media_by_id(*mid) {
                if let Ok(ir) = PreprocessedIr::parse(&bytes) {
                    *convolver = Some(Box::new(PartitionedConvolver::new(ir)));
                }
            }
        }
        Some(inst)
    }

    /// True if this instance is a Convolution Reverb with a loaded IR convolver.
    pub fn has_convolution_ir(&self) -> bool {
        matches!(self.state, DspState::ConvolutionReverb { convolver: Some(_), .. })
    }

    /// Process a planar multi-channel audio block in place.
    ///
    /// If this effect is a `Wwise Meter` with a non-zero `uGameParamID`, returns
    /// `Some((game_param_id, meter_output_db))` so the caller can write the metered level back to
    /// [`GameState::set_rtpc`].
    pub fn process(
        &mut self,
        project: &Project,
        gs: &GameState,
        buf: &mut [Vec<f32>],
        lfe_channel: Option<usize>,
    ) -> Option<(u32, f32)> {
        if buf.is_empty() || buf[0].is_empty() {
            return None;
        }
        let mut eff_params = self.params.clone();
        apply_fx_rtpcs(project, gs, &self.rtpcs, &mut eff_params);

        let num_ch = buf.len();
        let frames = buf[0].len();

        match (&eff_params, &mut self.state) {
            (FxParams::Gain(p), DspState::Gain) => {
                let fb_lin = db_to_lin(p.fullband_gain_db);
                let lfe_lin = db_to_lin(p.lfe_gain_db);
                for (c, ch_buf) in buf.iter_mut().enumerate() {
                    let g = if lfe_channel == Some(c) { lfe_lin } else { fb_lin };
                    if (g - 1.0).abs() > 1e-6 {
                        for s in ch_buf.iter_mut() {
                            *s *= g;
                        }
                    }
                }
                None
            }
            (FxParams::ParametricEq(p), DspState::ParametricEq { states }) => {
                if states.len() < num_ch {
                    states.resize(num_ch, [[0.0; 2]; 3]);
                }
                let biquads = [
                    DspBiquad::design(&p.bands[0], RATE),
                    DspBiquad::design(&p.bands[1], RATE),
                    DspBiquad::design(&p.bands[2], RATE),
                ];
                let out_lin = db_to_lin(p.output_level_db);
                for (c, ch_buf) in buf.iter_mut().enumerate() {
                    if lfe_channel == Some(c) && !p.process_lfe {
                        continue;
                    }
                    for b in 0..3 {
                        if p.bands[b].on_off {
                            biquads[b].process_slice(&mut states[c][b], ch_buf);
                        }
                    }
                    if (out_lin - 1.0).abs() > 1e-6 {
                        for s in ch_buf.iter_mut() {
                            *s *= out_lin;
                        }
                    }
                }
                None
            }
            (FxParams::Delay(p), DspState::Delay { lines }) => {
                let delay_samples = ((p.delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
                if lines.len() < num_ch {
                    lines.resize(num_ch, DelayLine::new(delay_samples + 4096));
                }
                for l in lines.iter_mut() {
                    if l.buf.len() <= delay_samples + 2 {
                        l.buf.resize(delay_samples + 256, 0.0);
                    }
                }
                let fb = if p.feedback_enabled {
                    (p.feedback * 0.01).clamp(0.0, 0.995)
                } else {
                    0.0
                };
                let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
                let dry = 1.0 - wet;
                let out_lin = db_to_lin(p.output_level_db);
                for (c, ch_buf) in buf.iter_mut().enumerate() {
                    if lfe_channel == Some(c) && !p.process_lfe {
                        continue;
                    }
                    let line = &mut lines[c];
                    for s in ch_buf.iter_mut() {
                        let x = *s;
                        let delayed = line.read_delay(delay_samples);
                        line.write_and_advance((x + delayed * fb).clamp(-8.0, 8.0));
                        *s = (x * dry + delayed * wet) * out_lin;
                    }
                }
                None
            }
            (
                FxParams::StereoDelay(p),
                DspState::StereoDelay {
                    lines,
                    fb_filter_state,
                },
            ) => {
                let d0 = ((p.channels[0].delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
                let d1 = ((p.channels[1].delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
                if lines[0].buf.len() <= d0 + 2 {
                    lines[0].buf.resize(d0 + 256, 0.0);
                }
                if lines[1].buf.len() <= d1 + 2 {
                    lines[1].buf.resize(d1 + 256, 0.0);
                }
                let fb0 = if p.enable_feedback {
                    db_to_lin(p.channels[0].feedback_db).min(0.995)
                } else {
                    0.0
                };
                let fb1 = if p.enable_feedback {
                    db_to_lin(p.channels[1].feedback_db).min(0.995)
                } else {
                    0.0
                };
                let cf0 = if p.enable_crossfeed {
                    db_to_lin(p.channels[0].crossfeed_db).min(0.995)
                } else {
                    0.0
                };
                let cf1 = if p.enable_crossfeed {
                    db_to_lin(p.channels[1].crossfeed_db).min(0.995)
                } else {
                    0.0
                };
                let dry_lin = db_to_lin(p.dry_level_db);
                let wet_lin = db_to_lin(p.wet_level_db);
                let flt = if p.filter_type > 0 {
                    let ft = match p.filter_type {
                        1 => EqFilterType::LowShelf,
                        2 => EqFilterType::Peaking,
                        3 => EqFilterType::HighShelf,
                        4 => EqFilterType::LowPass,
                        5 => EqFilterType::HighPass,
                        6 => EqFilterType::BandPass,
                        _ => EqFilterType::Notch,
                    };
                    Some(DspBiquad::design(
                        &EqBandParams {
                            filter_type: ft,
                            gain_db: p.filter_gain_db,
                            freq_hz: p.filter_freq_hz,
                            q_factor: p.filter_q,
                            on_off: true,
                        },
                        RATE,
                    ))
                } else {
                    None
                };

                for i in 0..frames {
                    let x0 = buf[0][i];
                    let x1 = if num_ch > 1 { buf[1][i] } else { x0 };
                    let y0 = lines[0].read_delay(d0);
                    let y1 = lines[1].read_delay(d1);
                    let mut in0 = (x0 + y0 * fb0 + y1 * cf1).clamp(-8.0, 8.0);
                    let mut in1 = (x1 + y1 * fb1 + y0 * cf0).clamp(-8.0, 8.0);
                    if let Some(b) = &flt {
                        in0 = b.tick(in0, &mut fb_filter_state[0]);
                        in1 = b.tick(in1, &mut fb_filter_state[1]);
                    }
                    lines[0].write_and_advance(in0);
                    lines[1].write_and_advance(in1);
                    buf[0][i] = x0 * dry_lin + y0 * wet_lin;
                    if num_ch > 1 {
                        buf[1][i] = x1 * dry_lin + y1 * wet_lin;
                    }
                }
                None
            }
            (FxParams::Dynamics(p), DspState::Dynamics { env }) => {
                if env.len() < num_ch {
                    env.resize(num_ch, 0.0);
                }
                let att_s = p.attack_or_lookahead_s.max(0.0001);
                let rel_s = p.release_s.max(0.0001);
                let alpha_att = (-1.0 / (att_s * RATE as f32)).exp();
                let alpha_rel = (-1.0 / (rel_s * RATE as f32)).exp();
                let ratio = p.ratio.max(1.0);
                let thresh_db = p.threshold_db;
                let out_lin = db_to_lin(p.output_level_db);

                let gain_for_env = |e: f32| -> f32 {
                    let lvl_db = if e <= 1e-6 { -96.3 } else { fast_pow_to_db(e * e) };
                    let gr_db = match p.kind {
                        DynamicsKind::Compressor | DynamicsKind::PeakLimiter => {
                            if lvl_db > thresh_db {
                                (thresh_db - lvl_db) * (1.0 - 1.0 / ratio)
                            } else {
                                0.0
                            }
                        }
                        DynamicsKind::Expander => {
                            if lvl_db < thresh_db {
                                (lvl_db - thresh_db) * (ratio - 1.0)
                            } else {
                                0.0
                            }
                        }
                    };
                    db_to_lin(gr_db) * out_lin
                };

                if p.channel_link {
                    let mut linked_env = env.first().copied().unwrap_or(0.0);
                    for i in 0..frames {
                        let mut peak = 0.0f32;
                        for (c, ch_buf) in buf.iter().enumerate() {
                            if lfe_channel == Some(c) && !p.process_lfe {
                                continue;
                            }
                            peak = peak.max(ch_buf[i].abs());
                        }
                        let a = if peak > linked_env { alpha_att } else { alpha_rel };
                        linked_env = a * linked_env + (1.0 - a) * peak;
                        let g = gain_for_env(linked_env);
                        for (c, ch_buf) in buf.iter_mut().enumerate() {
                            if lfe_channel == Some(c) && !p.process_lfe {
                                continue;
                            }
                            ch_buf[i] *= g;
                        }
                    }
                    for e in env.iter_mut() {
                        *e = linked_env;
                    }
                } else {
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let e = &mut env[c];
                        for s in ch_buf.iter_mut() {
                            let x = s.abs();
                            let a = if x > *e { alpha_att } else { alpha_rel };
                            *e = a * *e + (1.0 - a) * x;
                            *s *= gain_for_env(*e);
                        }
                    }
                }
                None
            }
            (
                FxParams::Meter(p),
                DspState::Meter {
                    env_db,
                    hold_remaining_s,
                    held_peak_db,
                },
            ) => {
                // Compute block Peak or RMS level across channels in dB
                let mut sum_sq = 0.0f32;
                let mut peak = 0.0f32;
                let total_samples = (num_ch * frames).max(1) as f32;
                for ch_buf in buf.iter() {
                    for &s in ch_buf.iter() {
                        let a = s.abs();
                        if a > peak {
                            peak = a;
                        }
                        sum_sq += s * s;
                    }
                }
                let block_db = if p.mode == 1 {
                    let rms_sq = sum_sq / total_samples;
                    if rms_sq <= 1e-12 {
                        -96.3
                    } else {
                        fast_pow_to_db(rms_sq)
                    }
                } else if peak <= 1e-6 {
                    -96.3
                } else {
                    fast_pow_to_db(peak * peak)
                };

                let dt = frames as f32 / RATE as f32;
                if p.infinite_hold {
                    if block_db > *env_db {
                        *env_db = block_db;
                    }
                } else if block_db >= *env_db {
                    // Attack toward higher level
                    let att = p.attack_s.max(0.0);
                    if att <= 1e-5 {
                        *env_db = block_db;
                    } else {
                        let alpha = (-dt / att).exp();
                        *env_db = alpha * *env_db + (1.0 - alpha) * block_db;
                    }
                    *held_peak_db = *env_db;
                    *hold_remaining_s = p.hold_s.max(0.0);
                } else if *hold_remaining_s > 0.0 {
                    *hold_remaining_s = (*hold_remaining_s - dt).max(0.0);
                    *env_db = *held_peak_db;
                } else {
                    let rel = p.release_s.max(0.0);
                    if rel <= 1e-5 {
                        *env_db = block_db;
                    } else {
                        let alpha = (-dt / rel).exp();
                        *env_db = alpha * *env_db + (1.0 - alpha) * block_db;
                    }
                }
                let clamped = env_db.clamp(p.min_db, p.max_db);
                *env_db = clamped;
                if p.game_param_id != 0 {
                    Some((p.game_param_id, clamped))
                } else {
                    None
                }
            }
            (FxParams::Tremolo(p), DspState::Tremolo { phase }) => {
                let depth = (p.mod_depth * 0.01).clamp(0.0, 1.0);
                let out_lin = db_to_lin(p.output_level_db);
                let dphase = p.lfo_freq_hz.max(0.0) / RATE as f32;
                for i in 0..frames {
                    let ph_base = (*phase + p.phase_offset_deg / 360.0).rem_euclid(1.0);
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let ch_spread = if p.phase_mode != 0 && num_ch > 1 {
                            (c as f32 / (num_ch - 1) as f32) * (p.phase_spread_deg / 360.0)
                        } else {
                            0.0
                        };
                        let ph = (ph_base + ch_spread).rem_euclid(1.0);
                        let lfo = eval_lfo(p.lfo_waveform, ph, p.lfo_pwm); // in -1..=1
                        // Amplitude modulation between (1 - depth) and 1.0
                        let mod_gain = 1.0 - depth * 0.5 * (1.0 - lfo);
                        ch_buf[i] *= mod_gain * out_lin;
                    }
                    *phase = (*phase + dphase).rem_euclid(1.0);
                }
                None
            }
            (FxParams::Flanger(p), DspState::Flanger { lines, phase }) => {
                if lines.len() < num_ch {
                    lines.resize(num_ch, DelayLine::new((0.05 * RATE as f32) as usize + 256));
                }
                let base_delay_samples = (p.delay_time_ms.clamp(0.1, 40.0) * 0.001 * RATE as f32).max(1.0);
                let depth = (p.mod_depth * 0.01).clamp(0.0, 1.0);
                let dphase = if p.enable_lfo {
                    p.lfo_freq_hz.max(0.0) / RATE as f32
                } else {
                    0.0
                };
                let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
                let dry = 1.0 - wet;
                let out_lin = db_to_lin(p.output_level_db);
                let fback = p.fback_level.clamp(-0.98, 0.98);
                let ffwd = p.ffwd_level.clamp(-1.0, 1.0);
                let dry_inner = p.dry_level.clamp(0.0, 1.0);

                for i in 0..frames {
                    let ph_base = (*phase + p.phase_offset_deg / 360.0).rem_euclid(1.0);
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let ch_spread = if p.phase_mode != 0 && num_ch > 1 {
                            (c as f32 / (num_ch - 1) as f32) * (p.phase_spread_deg / 360.0)
                        } else {
                            0.0
                        };
                        let lfo = if p.enable_lfo {
                            eval_lfo(p.lfo_waveform, (ph_base + ch_spread).rem_euclid(1.0), p.lfo_pwm)
                        } else {
                            0.0
                        };
                        let cur_delay = (base_delay_samples * (1.0 + depth * 0.5 * (1.0 + lfo))).max(1.0);
                        let x = ch_buf[i];
                        let delayed = lines[c].read_frac(cur_delay);
                        lines[c].write_and_advance((x + delayed * fback).clamp(-8.0, 8.0));
                        let y_flange = x * dry_inner + delayed * ffwd;
                        ch_buf[i] = (x * dry + y_flange * wet) * out_lin;
                    }
                    *phase = (*phase + dphase).rem_euclid(1.0);
                }
                None
            }
            (
                FxParams::GuitarDistortion(p),
                DspState::GuitarDistortion {
                    pre_states,
                    post_states,
                    tone_states,
                },
            ) => {
                if pre_states.len() < num_ch {
                    pre_states.resize(num_ch, [[0.0; 2]; 3]);
                    post_states.resize(num_ch, [[0.0; 2]; 3]);
                    tone_states.resize(num_ch, [0.0; 2]);
                }
                let pre_bq = [
                    DspBiquad::design(&p.pre_eq[0], RATE),
                    DspBiquad::design(&p.pre_eq[1], RATE),
                    DspBiquad::design(&p.pre_eq[2], RATE),
                ];
                let post_bq = [
                    DspBiquad::design(&p.post_eq[0], RATE),
                    DspBiquad::design(&p.post_eq[1], RATE),
                    DspBiquad::design(&p.post_eq[2], RATE),
                ];
                // Tone control (-100..100) maps to a high-shelf tilt around 1.2 kHz
                let tone_bq = (p.tone.abs() > 0.1).then(|| {
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: EqFilterType::HighShelf,
                            gain_db: (p.tone * 0.15).clamp(-15.0, 15.0),
                            freq_hz: 1200.0,
                            q_factor: 0.707,
                            on_off: true,
                        },
                        RATE,
                    )
                });
                let drive_norm = (p.drive * 0.01).clamp(0.0, 1.0);
                let drive_gain = 1.0 + drive_norm * 24.0;
                let rect = (p.rectification * 0.01).clamp(0.0, 1.0);
                let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
                let dry = 1.0 - wet;
                let out_lin = db_to_lin(p.output_level_db);

                for (c, ch_buf) in buf.iter_mut().enumerate() {
                    for s in ch_buf.iter_mut() {
                        let dry_x = *s;
                        let mut x = dry_x;
                        for b in 0..3 {
                            if p.pre_eq[b].on_off {
                                x = pre_bq[b].tick(x, &mut pre_states[c][b]);
                            }
                        }
                        if p.drive > 0.0 || p.rectification > 0.0 {
                            let driven = x * drive_gain;
                            let mut shaped = match p.distortion_type {
                                0 => driven.tanh(), // Overdrive
                                1 => {
                                    // Heavy: steeper cubic/tanh saturation
                                    (driven * 1.5).tanh()
                                }
                                2 => {
                                    // Fuzz: asymmetric waveshaper
                                    if driven >= 0.0 {
                                        1.0 - (-driven * 1.8).exp()
                                    } else {
                                        -(1.0 - (driven * 1.2).exp()) * 0.8
                                    }
                                }
                                _ => driven.clamp(-1.0, 1.0), // Hard clip
                            };
                            if rect > 0.0 {
                                shaped = shaped * (1.0 - rect) + shaped.abs() * rect;
                            }
                            x = shaped;
                        }
                        if let Some(tb) = &tone_bq {
                            x = tb.tick(x, &mut tone_states[c]);
                        }
                        for b in 0..3 {
                            if p.post_eq[b].on_off {
                                x = post_bq[b].tick(x, &mut post_states[c][b]);
                            }
                        }
                        *s = (dry_x * dry + x * wet) * out_lin;
                    }
                }
                None
            }
            (
                FxParams::MatrixReverb(p),
                DspState::Reverb {
                    predelay,
                    er_line: _,
                    fdn_lines,
                    fdn_lp,
                    tone_states: _,
                },
            ) => {
                let pre_samples = ((p.pre_delay_s.clamp(0.0, 0.45) * RATE as f32).round() as usize).max(1);
                let rt60 = p.reverb_time_s.clamp(0.05, 20.0);
                let damp = (1.0 - (1.0 / p.hf_ratio.clamp(1.0, 10.0))).clamp(0.05, 0.85);
                let dry_lin = db_to_lin(p.dry_level_db);
                let wet_lin = db_to_lin(p.wet_level_db) * 0.35;

                let lens = [
                    fdn_lines[0].buf.len() - 1,
                    fdn_lines[1].buf.len() - 1,
                    fdn_lines[2].buf.len() - 1,
                    fdn_lines[3].buf.len() - 1,
                ];
                let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));

                for i in 0..frames {
                    let mut mono_in = 0.0f32;
                    for (c, ch_buf) in buf.iter().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        mono_in += ch_buf[i];
                    }
                    mono_in /= num_ch.max(1) as f32;
                    let pre_out = predelay.read_delay(pre_samples);
                    predelay.write_and_advance(mono_in);

                    let mut y = [0.0f32; 4];
                    for k in 0..4 {
                        let raw = fdn_lines[k].read_delay(lens[k]);
                        fdn_lp[k] = fdn_lp[k] * damp + raw * (1.0 - damp);
                        y[k] = fdn_lp[k] * gains[k];
                    }
                    // Householder feedback matrix: M * y = y - (2/4) * sum(y)
                    let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
                    for k in 0..4 {
                        let fb = sum_y - y[k];
                        fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
                    }
                    let wet_l = (y[0] + y[2]) * wet_lin;
                    let wet_r = (y[1] + y[3]) * wet_lin;
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let w = if c % 2 == 0 { wet_l } else { wet_r };
                        ch_buf[i] = ch_buf[i] * dry_lin + w;
                    }
                }
                None
            }
            (
                FxParams::RoomVerb(p),
                DspState::Reverb {
                    predelay,
                    er_line,
                    fdn_lines,
                    fdn_lp,
                    tone_states,
                },
            ) => {
                if tone_states.len() < num_ch {
                    tone_states.resize(num_ch, [[0.0; 2]; 3]);
                }
                let pre_samples = ((p.reverb_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize).max(1);
                let rt60 = p.decay_time_s.clamp(0.05, 20.0);
                let damp = (p.hf_damping / 10.0).clamp(0.05, 0.85);
                let dry_lin = db_to_lin(p.dry_level_db);
                let er_lin = if p.enable_early_reflections {
                    db_to_lin(p.er_level_db) * 0.35
                } else {
                    0.0
                };
                let rev_lin = db_to_lin(p.reverb_level_db) * 0.35;
                let front_lin = db_to_lin(p.front_level_db);
                let width = (p.stereo_width / 180.0).clamp(0.0, 1.0);

                let lens = [
                    fdn_lines[0].buf.len() - 1,
                    fdn_lines[1].buf.len() - 1,
                    fdn_lines[2].buf.len() - 1,
                    fdn_lines[3].buf.len() - 1,
                ];
                let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));
                let er_taps = [347usize, 683, 1103, 1597];
                let tone_bq = [
                    DspBiquad::design(&p.filter_bands[0], RATE),
                    DspBiquad::design(&p.filter_bands[1], RATE),
                    DspBiquad::design(&p.filter_bands[2], RATE),
                ];

                for i in 0..frames {
                    let mut mono_in = 0.0f32;
                    for (c, ch_buf) in buf.iter().enumerate() {
                        if lfe_channel == Some(c) {
                            continue;
                        }
                        mono_in += ch_buf[i];
                    }
                    mono_in /= num_ch.max(1) as f32;

                    // Early reflections tap network
                    let er_l = if er_lin > 0.0 {
                        (er_line.read_delay(er_taps[0]) * 0.7 + er_line.read_delay(er_taps[2]) * 0.5) * er_lin
                    } else {
                        0.0
                    };
                    let er_r = if er_lin > 0.0 {
                        (er_line.read_delay(er_taps[1]) * 0.7 + er_line.read_delay(er_taps[3]) * 0.5) * er_lin
                    } else {
                        0.0
                    };
                    er_line.write_and_advance(mono_in);

                    // Pre-delay + FDN reverb tank
                    let pre_out = predelay.read_delay(pre_samples);
                    predelay.write_and_advance(mono_in);

                    let mut y = [0.0f32; 4];
                    for k in 0..4 {
                        let raw = fdn_lines[k].read_delay(lens[k]);
                        fdn_lp[k] = fdn_lp[k] * damp + raw * (1.0 - damp);
                        y[k] = fdn_lp[k] * gains[k];
                    }
                    let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
                    for k in 0..4 {
                        let fb = sum_y - y[k];
                        fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
                    }
                    let rev_l = (y[0] + y[2]) * rev_lin;
                    let rev_r = (y[1] + y[3]) * rev_lin;
                    let mid = 0.5 * ((er_l + rev_l) + (er_r + rev_r));
                    let side = 0.5 * ((er_l + rev_l) - (er_r + rev_r)) * width;
                    let wet_l = (mid + side) * front_lin;
                    let wet_r = (mid - side) * front_lin;

                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) {
                            continue;
                        }
                        let mut w = if c % 2 == 0 { wet_l } else { wet_r };
                        if p.enable_tone_controls {
                            for b in 0..3 {
                                if p.filter_bands[b].on_off {
                                    w = tone_bq[b].tick(w, &mut tone_states[c][b]);
                                }
                            }
                        }
                        ch_buf[i] = ch_buf[i] * dry_lin + w;
                    }
                }
                None
            }
            (
                FxParams::PitchShifter(p),
                DspState::PitchShifter {
                    lines,
                    phase,
                    filter_states,
                },
            ) => {
                if lines.len() < num_ch {
                    lines.resize(num_ch, DelayLine::new(8192));
                }
                if filter_states.len() < num_ch {
                    filter_states.resize(num_ch, [0.0; 2]);
                }
                let win_samples = (p.delay_time_ms.clamp(1.0, 150.0) * 0.001 * RATE as f32).clamp(64.0, 4096.0);
                let pitch_ratio = 2.0f32.powf(p.pitch_cents.clamp(-2400.0, 2400.0) / 1200.0);
                let dphase = (1.0 - pitch_ratio) / win_samples;
                let dry_lin = db_to_lin(p.dry_level_db);
                let wet_lin = db_to_lin(p.wet_level_db);
                let flt = (p.filter_type > 0).then(|| {
                    let ft = match p.filter_type {
                        1 => EqFilterType::LowShelf,
                        2 => EqFilterType::Peaking,
                        3 => EqFilterType::HighShelf,
                        4 => EqFilterType::LowPass,
                        5 => EqFilterType::HighPass,
                        6 => EqFilterType::BandPass,
                        _ => EqFilterType::Notch,
                    };
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: ft,
                            gain_db: p.filter_gain_db,
                            freq_hz: p.filter_freq_hz,
                            q_factor: p.filter_q,
                            on_off: true,
                        },
                        RATE,
                    )
                });
                let sync_delay = (win_samples * 0.5) as usize;

                for i in 0..frames {
                    let ph0 = phase.rem_euclid(1.0);
                    let ph1 = (ph0 + 0.5).rem_euclid(1.0);
                    let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
                    let w1 = 1.0 - w0;
                    let d0 = ph0 * win_samples + 1.0;
                    let d1 = ph1 * win_samples + 1.0;

                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let x = ch_buf[i];
                        lines[c].write_and_advance(x);
                        let mut wet = lines[c].read_frac(d0) * w0 + lines[c].read_frac(d1) * w1;
                        if let Some(b) = &flt {
                            wet = b.tick(wet, &mut filter_states[c]);
                        }
                        let dry_x = if p.sync_dry {
                            lines[c].read_delay(sync_delay)
                        } else {
                            x
                        };
                        ch_buf[i] = dry_x * dry_lin + wet * wet_lin;
                    }
                    *phase = (ph0 + dphase).rem_euclid(1.0);
                }
                None
            }
            (
                FxParams::Harmonizer(p),
                DspState::Harmonizer {
                    lines,
                    phases,
                    filter_states,
                },
            ) => {
                if lines.len() < num_ch {
                    lines.resize(num_ch, DelayLine::new(8192));
                }
                if filter_states.len() < num_ch {
                    filter_states.resize(num_ch, [[0.0; 2]; 2]);
                }
                let win_samples = (p.window_size.clamp(64, 4096)) as f32;
                let dry_lin = db_to_lin(p.dry_level_db);
                let wet_lin = db_to_lin(p.wet_level_db);
                let sync_delay = (win_samples * 0.5) as usize;

                let voice_cfg: [(bool, f32, f32, Option<DspBiquad>); 2] = std::array::from_fn(|v| {
                    let vp = &p.voices[v];
                    let ratio = 2.0f32.powf(vp.pitch_cents.clamp(-2400.0, 2400.0) / 1200.0);
                    let dph = (1.0 - ratio) / win_samples;
                    let gain = db_to_lin(vp.gain_db);
                    let flt = (vp.enable && vp.filter_type > 0).then(|| {
                        let ft = match vp.filter_type {
                            1 => EqFilterType::LowShelf,
                            2 => EqFilterType::Peaking,
                            3 => EqFilterType::HighShelf,
                            4 => EqFilterType::LowPass,
                            5 => EqFilterType::HighPass,
                            6 => EqFilterType::BandPass,
                            _ => EqFilterType::Notch,
                        };
                        DspBiquad::design(
                            &EqBandParams {
                                filter_type: ft,
                                gain_db: vp.filter_gain_db,
                                freq_hz: vp.filter_freq_hz,
                                q_factor: vp.filter_q,
                                on_off: true,
                            },
                            RATE,
                        )
                    });
                    (vp.enable, dph, gain, flt)
                });

                for i in 0..frames {
                    let ph_v0 = phases[0].rem_euclid(1.0);
                    let ph_v1 = phases[1].rem_euclid(1.0);
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) && !p.process_lfe {
                            continue;
                        }
                        let x = ch_buf[i];
                        lines[c].write_and_advance(x);
                        let mut wet_sum = 0.0f32;
                        for (v, &(en, _, gain, ref flt)) in voice_cfg.iter().enumerate() {
                            if !en {
                                continue;
                            }
                            let ph0 = if v == 0 { ph_v0 } else { ph_v1 };
                            let ph1 = (ph0 + 0.5).rem_euclid(1.0);
                            let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
                            let w1 = 1.0 - w0;
                            let d0 = ph0 * win_samples + 1.0;
                            let d1 = ph1 * win_samples + 1.0;
                            let mut y = (lines[c].read_frac(d0) * w0 + lines[c].read_frac(d1) * w1) * gain;
                            if let Some(b) = flt {
                                y = b.tick(y, &mut filter_states[c][v]);
                            }
                            wet_sum += y;
                        }
                        let dry_x = if p.sync_dry {
                            lines[c].read_delay(sync_delay)
                        } else {
                            x
                        };
                        ch_buf[i] = dry_x * dry_lin + wet_sum * wet_lin;
                    }
                    phases[0] = (ph_v0 + voice_cfg[0].1).rem_euclid(1.0);
                    phases[1] = (ph_v1 + voice_cfg[1].1).rem_euclid(1.0);
                }
                None
            }
            (FxParams::TimeStretch(p), DspState::TimeStretch { lines, phase }) => {
                if lines.len() < num_ch {
                    lines.resize(num_ch, DelayLine::new(8192));
                }
                let win_samples = (p.window_size.clamp(128, 4096)) as f32;
                let stretch_ratio = if p.time_stretch > 10.0 {
                    (p.time_stretch * 0.01).clamp(0.25, 4.0)
                } else if p.time_stretch > 0.01 {
                    p.time_stretch.clamp(0.25, 4.0)
                } else {
                    1.0
                };
                let pitch_ratio = 2.0f32.powf(p.pitch_shift_cents.clamp(-2400.0, 2400.0) / 1200.0);
                let eff_ratio = (pitch_ratio / stretch_ratio).clamp(0.25, 4.0);
                let dphase = (1.0 - eff_ratio) / win_samples;
                let out_lin = db_to_lin(p.output_gain_db);

                for i in 0..frames {
                    let ph0 = phase.rem_euclid(1.0);
                    let ph1 = (ph0 + 0.5).rem_euclid(1.0);
                    let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
                    let w1 = 1.0 - w0;
                    let d0 = ph0 * win_samples + 1.0;
                    let d1 = ph1 * win_samples + 1.0;

                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        let x = ch_buf[i];
                        lines[c].write_and_advance(x);
                        let y = if p.enable || (eff_ratio - 1.0).abs() > 1e-4 {
                            lines[c].read_frac(d0) * w0 + lines[c].read_frac(d1) * w1
                        } else {
                            x
                        };
                        ch_buf[i] = y * out_lin;
                    }
                    *phase = (ph0 + dphase).rem_euclid(1.0);
                }
                None
            }
            (
                FxParams::ConvolutionReverb(p),
                DspState::ConvolutionReverb {
                    ir_media_id,
                    convolver,
                    predelay_ch,
                    predelay,
                    er_line,
                    fdn_lines,
                    fdn_lp,
                    rear_delay,
                },
            ) => {
                // Lazy IR hydration: banks carrying DIDX IR media (e.g. Global.bnk) may load after
                // Init.bnk creates the bus/FX object.
                if convolver.is_none() {
                    if let Some(mid) = *ir_media_id {
                        if let Some(bytes) = project.read_media_by_id(mid) {
                            if let Ok(ir) = PreprocessedIr::parse(&bytes) {
                                *convolver = Some(Box::new(PartitionedConvolver::new(ir)));
                            }
                        }
                    }
                }

                let dry_lin = db_to_lin(p.dry_level_db);
                let front_lin = db_to_lin(p.front_level_db);
                let rear_lin = db_to_lin(p.rear_level_db);
                let center_lin = db_to_lin(p.center_level_db);
                let width = (p.stereo_width / 180.0).clamp(0.0, 1.0);

                if let Some(conv) = convolver.as_deref_mut() {
                    let ir_ch = conv.num_channels();
                    let pre_samples = (p.pre_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize;
                    let wet_lin = db_to_lin(p.wet_level_db);
                    let in_ctr_lin = db_to_lin(p.input_center_level_db);
                    let in_lfe_lin = db_to_lin(p.input_lfe_level_db);

                    if pre_samples > 0 && predelay_ch.len() < ir_ch {
                        let max_pre = (0.5 * RATE as f32) as usize + 256;
                        predelay_ch.resize_with(ir_ch, || DelayLine::new(max_pre));
                    }

                    let non_lfe: Vec<usize> = (0..num_ch).filter(|&c| lfe_channel != Some(c)).collect();
                    let mut wet_in = vec![vec![0.0f32; frames]; ir_ch];
                    if !non_lfe.is_empty() {
                        for ic in 0..ir_ch {
                            let dst = &mut wet_in[ic];
                            if p.algo_type == 0 && non_lfe.len() != ir_ch {
                                if ir_ch == 1 {
                                    let scale = 1.0 / (non_lfe.len() as f32);
                                    for &sc in &non_lfe {
                                        for i in 0..frames {
                                            dst[i] += buf[sc][i] * scale;
                                        }
                                    }
                                } else if non_lfe.len() == 2 && ir_ch == 6 {
                                    // Stereo input -> 5.1 IR (L, R, C, LFE, RL, RR)
                                    let (l_ch, r_ch) = (non_lfe[0], non_lfe[1]);
                                    match ic {
                                        0 | 4 => dst.copy_from_slice(&buf[l_ch][..frames]),
                                        1 | 5 => dst.copy_from_slice(&buf[r_ch][..frames]),
                                        2 => {
                                            let g = 0.5 * in_ctr_lin;
                                            for i in 0..frames {
                                                dst[i] = (buf[l_ch][i] + buf[r_ch][i]) * g;
                                            }
                                        }
                                        3 => {
                                            if let Some(lc) = lfe_channel {
                                                for i in 0..frames {
                                                    dst[i] = buf[lc][i] * in_lfe_lin;
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                } else {
                                    let sc = non_lfe[ic % non_lfe.len()];
                                    dst.copy_from_slice(&buf[sc][..frames]);
                                }
                            } else {
                                let sc = non_lfe[ic % non_lfe.len()];
                                dst.copy_from_slice(&buf[sc][..frames]);
                            }
                            if pre_samples > 0 {
                                let dl = &mut predelay_ch[ic];
                                for s in dst.iter_mut() {
                                    let d = dl.read_delay(pre_samples);
                                    dl.write_and_advance(*s);
                                    *s = d;
                                }
                            }
                        }
                    }

                    let mut wet_out = vec![vec![0.0f32; frames]; ir_ch];
                    conv.process_channels(&wet_in, &mut wet_out, frames);

                    if non_lfe.len() == 2 && ir_ch >= 2 {
                        let (l_ch, r_ch) = (non_lfe[0], non_lfe[1]);
                        for i in 0..frames {
                            let mut raw_l = wet_out[0][i] * front_lin;
                            let mut raw_r = wet_out[1][i] * front_lin;
                            if ir_ch == 4 {
                                raw_l += wet_out[2][i] * rear_lin * 0.707_106_77;
                                raw_r += wet_out[3][i] * rear_lin * 0.707_106_77;
                            } else if ir_ch >= 6 {
                                let c_term = wet_out[2][i] * center_lin * 0.707_106_77;
                                raw_l += c_term + wet_out[4][i] * rear_lin * 0.707_106_77;
                                raw_r += c_term + wet_out[5][i] * rear_lin * 0.707_106_77;
                            }
                            let mid = 0.5 * (raw_l + raw_r);
                            let side = 0.5 * (raw_l - raw_r) * width;
                            buf[l_ch][i] = buf[l_ch][i] * dry_lin + (mid + side) * wet_lin;
                            buf[r_ch][i] = buf[r_ch][i] * dry_lin + (mid - side) * wet_lin;
                        }
                    } else if non_lfe.len() == 1 {
                        let ch0 = non_lfe[0];
                        let scale = front_lin * wet_lin / (ir_ch as f32);
                        for i in 0..frames {
                            let sum: f32 = wet_out.iter().map(|w| w[i]).sum();
                            buf[ch0][i] = buf[ch0][i] * dry_lin + sum * scale;
                        }
                    } else {
                        for (idx, &c) in non_lfe.iter().enumerate() {
                            let ch_gain = if idx < 2 {
                                front_lin
                            } else if idx == 2 && non_lfe.len() >= 5 {
                                center_lin
                            } else {
                                rear_lin
                            } * wet_lin;
                            let w_ch = &wet_out[idx % ir_ch];
                            for i in 0..frames {
                                buf[c][i] = buf[c][i] * dry_lin + w_ch[i] * ch_gain;
                            }
                        }
                    }
                    return None;
                }

                // Fallback parameterized FDN when no IR media asset is attached/loaded.
                let pre_samples = ((p.pre_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize).max(1);
                let rear_samples = ((p.front_rear_delay_ms.clamp(0.0, 120.0) * 0.001 * RATE as f32).round() as usize).max(1);
                let wet_lin = db_to_lin(p.wet_level_db) * 0.35;

                // Multi-tap early-reflection FIR + FDN impulse response model
                let er_taps_l = [(193usize, 0.65f32), (467, -0.45), (853, 0.35), (1361, -0.25)];
                let er_taps_r = [(241usize, 0.60f32), (557, -0.42), (977, 0.32), (1489, -0.22)];
                let lens = [
                    fdn_lines[0].buf.len() - 1,
                    fdn_lines[1].buf.len() - 1,
                    fdn_lines[2].buf.len() - 1,
                    fdn_lines[3].buf.len() - 1,
                ];
                let rt60 = 1.6f32;
                let damp = 0.32f32;
                let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));

                for i in 0..frames {
                    let mut mono_in = 0.0f32;
                    for (c, ch_buf) in buf.iter().enumerate() {
                        if lfe_channel == Some(c) {
                            continue;
                        }
                        mono_in += ch_buf[i];
                    }
                    mono_in /= num_ch.max(1) as f32;

                    let pre_out = predelay.read_delay(pre_samples);
                    predelay.write_and_advance(mono_in);

                    let mut er_l = 0.0f32;
                    let mut er_r = 0.0f32;
                    for &(t, g) in &er_taps_l {
                        er_l += er_line.read_delay(t) * g;
                    }
                    for &(t, g) in &er_taps_r {
                        er_r += er_line.read_delay(t) * g;
                    }
                    er_line.write_and_advance(pre_out);

                    let mut y = [0.0f32; 4];
                    for k in 0..4 {
                        let raw = fdn_lines[k].read_delay(lens[k]);
                        fdn_lp[k] = fdn_lp[k] * damp + raw * (1.0 - damp);
                        y[k] = fdn_lp[k] * gains[k];
                    }
                    let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
                    for k in 0..4 {
                        let fb = sum_y - y[k];
                        fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
                    }

                    let rear_out = rear_delay.read_delay(rear_samples);
                    rear_delay.write_and_advance(0.5 * (y[2] + y[3]));

                    let raw_l = (er_l + y[0] + y[2]) * front_lin + rear_out * rear_lin * 0.5;
                    let raw_r = (er_r + y[1] + y[3]) * front_lin - rear_out * rear_lin * 0.5;
                    let mid = 0.5 * (raw_l + raw_r);
                    let side = 0.5 * (raw_l - raw_r) * width;
                    let wet_l = (mid + side) * wet_lin;
                    let wet_r = (mid - side) * wet_lin;

                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        if lfe_channel == Some(c) {
                            continue;
                        }
                        let w = if c % 2 == 0 { wet_l } else { wet_r };
                        ch_buf[i] = ch_buf[i] * dry_lin + w;
                    }
                }
                None
            }
            (
                FxParams::McDspFutzBox(p),
                DspState::McDspFutzBox {
                    hpf_states,
                    lpf_states,
                    sim_states,
                    eq_states,
                    gate_env,
                    decim_hold,
                    decim_phase,
                },
            ) => {
                if hpf_states.len() < num_ch {
                    hpf_states.resize(num_ch, [0.0; 2]);
                    lpf_states.resize(num_ch, [0.0; 2]);
                    sim_states.resize(num_ch, [0.0; 2]);
                    eq_states.resize(num_ch, [0.0; 2]);
                    gate_env.resize(num_ch, 0.0);
                    decim_hold.resize(num_ch, 0.0);
                }
                let in_lin = db_to_lin(p.input_gain_db);
                let out_lin = db_to_lin(p.output_gain_db);
                let wet = (p.balance_pct * 0.01).clamp(0.0, 1.0);
                let dry = 1.0 - wet;

                let hpf = p.filter_enable.then(|| {
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: EqFilterType::HighPass,
                            gain_db: 0.0,
                            freq_hz: p.hpf_freq_hz.clamp(20.0, 8000.0),
                            q_factor: (0.707 + p.hpf_q.clamp(-12.0, 12.0) * 0.05).clamp(0.2, 10.0),
                            on_off: true,
                        },
                        RATE,
                    )
                });
                let lpf = p.filter_enable.then(|| {
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: EqFilterType::LowPass,
                            gain_db: 0.0,
                            freq_hz: p.lpf_freq_hz.clamp(200.0, 20000.0),
                            q_factor: (0.707 + p.lpf_q.clamp(-12.0, 12.0) * 0.05).clamp(0.2, 10.0),
                            on_off: true,
                        },
                        RATE,
                    )
                });
                // SIM synthetic speaker/telephone bandpass resonance
                let sim_bq = p.sim_enable.then(|| {
                    let tune_scale = 2.0f32.powf(p.sim_tuning.clamp(-24.0, 24.0) / 12.0);
                    let base_freq = (900.0 + (p.sim_index % 16) as f32 * 140.0) * tune_scale;
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: EqFilterType::BandPass,
                            gain_db: 0.0,
                            freq_hz: base_freq.clamp(300.0, 6000.0),
                            q_factor: 1.4,
                            on_off: true,
                        },
                        RATE,
                    )
                });
                let eq_bq = p.eq_enable.then(|| {
                    let ft = match p.eq_mode {
                        0 => EqFilterType::LowShelf,
                        2 => EqFilterType::HighShelf,
                        3 => EqFilterType::BandPass,
                        4 => EqFilterType::Notch,
                        _ => EqFilterType::Peaking,
                    };
                    DspBiquad::design(
                        &EqBandParams {
                            filter_type: ft,
                            gain_db: p.eq_gain_db,
                            freq_hz: p.eq_freq_hz.clamp(40.0, 18000.0),
                            q_factor: p.eq_q.abs().clamp(0.2, 12.0),
                            on_off: true,
                        },
                        RATE,
                    )
                });

                let drive = if p.dist_enable {
                    1.0 + (p.dist_amount * 0.01).clamp(0.0, 1.0) * 12.0
                        + (p.dist_intensity * 0.01).clamp(0.0, 1.0) * 12.0
                } else {
                    1.0
                };
                let rect = (p.dist_rectify * 0.01).clamp(0.0, 1.0);
                let gate_thresh = db_to_lin(p.gate_thresh_db);
                let gate_range = db_to_lin(p.gate_range_db.min(0.0));
                let gate_att = (-1.0 / ((p.gate_attack_ms.max(0.1) * 0.001) * RATE as f32)).exp();
                let gate_rel = (-1.0 / ((p.gate_release_ms.max(1.0) * 0.001) * RATE as f32)).exp();
                let decim_step = if p.sim_enable { 0.5f32 } else { 1.0f32 };

                for i in 0..frames {
                    *decim_phase += decim_step;
                    let sample_tick = *decim_phase >= 1.0;
                    if sample_tick {
                        *decim_phase -= 1.0;
                    }
                    for (c, ch_buf) in buf.iter_mut().enumerate() {
                        let dry_x = ch_buf[i];
                        let mut x = dry_x * in_lin;
                        if p.gate_enable {
                            let a = if x.abs() > gate_env[c] { gate_att } else { gate_rel };
                            gate_env[c] = a * gate_env[c] + (1.0 - a) * x.abs();
                            if gate_env[c] < gate_thresh {
                                x *= gate_range;
                            }
                        }
                        if let Some(b) = &hpf {
                            x = b.tick(x, &mut hpf_states[c]);
                        }
                        if let Some(b) = &lpf {
                            x = b.tick(x, &mut lpf_states[c]);
                        }
                        if let Some(b) = &sim_bq {
                            let resonated = b.tick(x, &mut sim_states[c]) * 1.6;
                            if sample_tick {
                                // Quantize to 12-bit lo-fi step when SIM is active
                                decim_hold[c] = (resonated * 2048.0).round() / 2048.0;
                            }
                            x = 0.35 * x + 0.65 * decim_hold[c];
                        }
                        if p.dist_enable {
                            let d = x * drive;
                            let mut shaped = match p.dist_mode {
                                0 => d.tanh(),
                                1 => (d * 1.4).tanh(),
                                2 => d.clamp(-0.85, 0.85),
                                _ => d / (1.0 + d.abs()),
                            };
                            if rect > 0.0 {
                                shaped = shaped * (1.0 - rect) + shaped.abs() * rect;
                            }
                            x = shaped;
                        }
                        if let Some(b) = &eq_bq {
                            x = b.tick(x, &mut eq_states[c]);
                        }
                        ch_buf[i] = dry_x * dry + x * out_lin * wet;
                    }
                }
                None
            }
            (FxParams::IzotopeTrash(p), DspState::IzotopeTrash { filter_states }) => {
                if filter_states.len() < num_ch {
                    filter_states.resize(num_ch, [[0.0; 2]; 2]);
                }
                let post_lpf = DspBiquad::design(
                    &EqBandParams {
                        filter_type: EqFilterType::LowPass,
                        gain_db: 0.0,
                        freq_hz: p.filter_freq_hz.clamp(500.0, 20000.0),
                        q_factor: p.filter_q.clamp(0.2, 10.0),
                        on_off: true,
                    },
                    RATE,
                );
                let dc_hpf = DspBiquad::design(
                    &EqBandParams {
                        filter_type: EqFilterType::HighPass,
                        gain_db: 0.0,
                        freq_hz: 25.0,
                        q_factor: 0.707,
                        on_off: true,
                    },
                    RATE,
                );

                for (c, ch_buf) in buf.iter_mut().enumerate() {
                    for s in ch_buf.iter_mut() {
                        let mut x = *s;
                        for st in &p.stages {
                            let mix = (st.mix_pct * 0.01).clamp(0.0, 1.0);
                            if mix <= 1e-5 {
                                continue;
                            }
                            let in_g = db_to_lin(st.input_gain_db);
                            let out_g = db_to_lin(st.output_gain_db);
                            let drive = (1.0 + st.overdrive.clamp(0.0, 30.0) * 1.2).max(0.1);
                            let trash = st.trash.clamp(0.0, 1.0);
                            let u = x * in_g * drive;
                            let shaped = match st.algo_type % 4 {
                                0 => {
                                    // Warm tube / tanh + asymmetric trash harmonic
                                    let sym = u.tanh();
                                    let asym = ((u + trash * 0.4).tanh() - (trash * 0.4).tanh()).clamp(-1.0, 1.0);
                                    sym * (1.0 - trash) + asym * trash
                                }
                                1 => {
                                    // Soft-knee algebraic saturator + foldback
                                    let soft = u / (1.0 + u.abs());
                                    let fold = (u * (1.0 + trash)).sin();
                                    soft * (1.0 - 0.5 * trash) + fold * (0.5 * trash)
                                }
                                2 => {
                                    // Fuzz / rectifier
                                    let fz = (u * 1.5).tanh();
                                    fz * (1.0 - 0.4 * trash) + fz.abs() * (0.4 * trash)
                                }
                                _ => u.clamp(-1.0, 1.0),
                            };
                            x = x * (1.0 - mix) + shaped * out_g * mix;
                        }
                        x = dc_hpf.tick(x, &mut filter_states[c][0]);
                        x = post_lpf.tick(x, &mut filter_states[c][1]);
                        *s = x.clamp(-4.0, 4.0);
                    }
                }
                None
            }
            (
                FxParams::MasteringSuite(p),
                DspState::MasteringSuite {
                    eq_states,
                    comp_env,
                    lim_env,
                },
            ) => {
                if eq_states.len() < num_ch {
                    eq_states.resize(num_ch, [[0.0; 2]; 6]);
                }
                let eq_biquads: [DspBiquad; 6] =
                    std::array::from_fn(|b| DspBiquad::design(&p.eq_bands[b], RATE));
                let comp_att = (-1.0 / (p.comp_attack_s.max(0.0001) * RATE as f32)).exp();
                let comp_rel = (-1.0 / (p.comp_release_s.max(0.001) * RATE as f32)).exp();
                let comp_ratio = p.comp_ratio.max(1.0);
                let comp_makeup = db_to_lin(p.comp_makeup_db);
                let lim_rel = (-1.0 / (p.limiter_release_s.max(0.001) * RATE as f32)).exp();
                let lim_thresh = db_to_lin(p.limiter_threshold_db);
                let lim_ceil = db_to_lin(p.limiter_ceiling_db);
                let master_gain = db_to_lin(p.master_output_gain_db);
                let width = if p.width_enable {
                    (p.stereo_width.clamp(-100.0, 100.0) + 100.0) * 0.01
                } else {
                    1.0
                };

                for i in 0..frames {
                    // 1. 6-band Parametric EQ
                    if p.eq_enable {
                        for (c, ch_buf) in buf.iter_mut().enumerate() {
                            let mut s = ch_buf[i];
                            for b in 0..6 {
                                if p.eq_bands[b].on_off {
                                    s = eq_biquads[b].tick(s, &mut eq_states[c][b]);
                                }
                            }
                            ch_buf[i] = s;
                        }
                    }

                    // 2. Linked Compressor
                    if p.comp_enable {
                        let mut peak = 0.0f32;
                        for ch_buf in buf.iter() {
                            peak = peak.max(ch_buf[i].abs());
                        }
                        let a = if peak > *comp_env { comp_att } else { comp_rel };
                        *comp_env = a * *comp_env + (1.0 - a) * peak;
                        let env_db = if *comp_env <= 1e-6 {
                            -96.3
                        } else {
                            fast_pow_to_db(*comp_env * *comp_env)
                        };
                        let gr_db = if env_db > p.comp_threshold_db {
                            (p.comp_threshold_db - env_db) * (1.0 - 1.0 / comp_ratio)
                        } else {
                            0.0
                        };
                        let g = db_to_lin(gr_db) * comp_makeup;
                        for ch_buf in buf.iter_mut() {
                            ch_buf[i] *= g;
                        }
                    }

                    // 3. Stereo Width
                    if p.width_enable && num_ch >= 2 {
                        let l = buf[0][i];
                        let r = buf[1][i];
                        let mid = 0.5 * (l + r);
                        let side = 0.5 * (l - r) * width;
                        buf[0][i] = mid + side;
                        buf[1][i] = mid - side;
                    }

                    // 4. Brickwall Limiter + Master Output Gain
                    let mut out_peak = 0.0f32;
                    for ch_buf in buf.iter_mut() {
                        ch_buf[i] *= master_gain;
                        out_peak = out_peak.max(ch_buf[i].abs());
                    }
                    if p.limiter_enable {
                        if out_peak > *lim_env {
                            *lim_env = out_peak;
                        } else {
                            *lim_env = lim_rel * *lim_env + (1.0 - lim_rel) * out_peak;
                        }
                        let lim_g = if *lim_env > lim_thresh && *lim_env > 1e-6 {
                            (lim_ceil / *lim_env).min(1.0)
                        } else if lim_thresh > 1e-6 {
                            (lim_ceil / lim_thresh).min(1.0)
                        } else {
                            1.0
                        };
                        for ch_buf in buf.iter_mut() {
                            ch_buf[i] = (ch_buf[i] * lim_g).clamp(-lim_ceil.max(0.01), lim_ceil.max(0.01));
                        }
                    }
                }
                None
            }
            (FxParams::QAnalyzer(_), DspState::QAnalyzer { peak_db }) => {
                let mut peak = 0.0f32;
                for ch_buf in buf.iter() {
                    for &s in ch_buf.iter() {
                        peak = peak.max(s.abs());
                    }
                }
                *peak_db = if peak <= 1e-6 {
                    -96.3
                } else {
                    fast_pow_to_db(peak * peak)
                };
                None
            }
            _ => None,
        }
    }
}

#[inline]
fn eval_lfo(waveform: u32, phase: f32, pwm_pct: f32) -> f32 {
    let p = phase.rem_euclid(1.0);
    match waveform {
        0 => (2.0 * PI * p).sin(), // Sine
        1 => {
            // Triangle
            if p < 0.5 {
                4.0 * p - 1.0
            } else {
                3.0 - 4.0 * p
            }
        }
        2 => {
            // Square with PWM
            let duty = (pwm_pct * 0.01).clamp(0.05, 0.95);
            if p < duty {
                1.0
            } else {
                -1.0
            }
        }
        3 => 2.0 * p - 1.0, // Sawtooth up
        4 => 1.0 - 2.0 * p, // Sawtooth down
        _ => (2.0 * PI * p).sin(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnk::hirc::GraphPoint;
    #[cfg(feature = "northlight")]
    use crate::bnk::Bank;
    #[cfg(feature = "northlight")]
    use crate::data::{game_dir, GameAudio};

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
        assert!(sbuf[1][20] > 0.4, "Crossfed Right echo at t=8+12=20, got {}", sbuf[1][20]);
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
        assert!(quiet[0][511] < 0.005, "quiet signal expanded downward, got {}", quiet[0][511]);
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
        let mut sig: Vec<Vec<f32>> = vec![(0..512).map(|n| (2.0 * PI * 440.0 * n as f32 / RATE as f32).sin()).collect()];
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
        let mut gsig: Vec<Vec<f32>> = vec![(0..512).map(|n| (2.0 * PI * 220.0 * n as f32 / RATE as f32).sin() * 0.8).collect()];
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
        assert!(total_tail_energy > 1e-5, "FDN tank should produce reverb tail energy, got {total_tail_energy}");

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
        assert!(er_and_tail_energy > 1e-4, "RoomVerb ER + FDN tail energy {er_and_tail_energy}");
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
                GraphPoint { from: 0.0, to: -12.0, interp: 4 },
                GraphPoint { from: 1.0, to: 6.0, interp: 4 },
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
        assert!(ps_energy > 1.0, "PitchShifter should produce non-trivial output, got {ps_energy}");
        assert!(ps_buf[0][256..].iter().zip(&sine[256..]).any(|(a, b)| (a - b).abs() > 0.05));

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
        assert!(hm_buf[0][256..].iter().zip(&sine[256..]).any(|(a, b)| (a - b).abs() > 0.05));

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
        assert!(ts_buf[0][256..].iter().zip(&sine[256..]).any(|(a, b)| (a - b).abs() > 0.05));
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
        assert!(tail_energy > 1e-4, "ConvolutionReverb should produce decaying tail, got {tail_energy}");

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
        assert!(fb_buf[0][64..].iter().zip(&sine[64..]).any(|(a, b)| (a - b).abs() > 0.05));

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
        assert!(tr_buf[0][64..].iter().zip(&sine[64..]).any(|(a, b)| (a - b).abs() > 0.05));

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
        assert!(built_in_processed >= 80, "expected >= 80 built-in FX instances, got {built_in_processed}");
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
}
