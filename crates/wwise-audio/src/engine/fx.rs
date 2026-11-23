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

mod biquad;
mod dynamics;
mod modulation;
mod params;
mod reverb;
mod rtpc;
#[cfg(test)]
mod tests;
mod third_party;

pub use super::convolver::{IrPrecision, PartitionedConvolver, PreprocessedIr};
pub use biquad::DspBiquad;
pub use params::{
    parse_fx_params, parse_ir_media, plugin_id, ConvolutionReverbParams, DelayParams, DynamicsKind,
    DynamicsParams, EqBandParams, EqFilterType, FlangerParams, FxParams, GainParams, GuitarDistortionParams,
    HarmonizerParams, HarmonizerVoiceParams, IzotopeTrashParams, MasteringSuiteParams, MatrixReverbParams,
    McDspFutzBoxParams, MeterParams, ParametricEqParams, PitchShifterParams, QAnalyzerParams, RoomVerbParams,
    StereoDelayChannelParams, StereoDelayParams, TimeStretchParams, TrashDistortionStage, TremoloParams,
};
pub use rtpc::apply_fx_rtpcs;

use super::math::RATE;
use super::params::GameState;
use super::project::Project;
use crate::bnk::hirc::{FxBase, HircBody, Rtpc};
use biquad::{process_delay, process_gain, process_parametric_eq, process_stereo_delay, DelayLine};
use dynamics::{process_dynamics, process_meter, process_q_analyzer};
use modulation::{
    process_flanger, process_guitar_distortion, process_harmonizer, process_pitch_shifter, process_time_stretch,
    process_tremolo, GuitarDistortionState, HarmonizerState, PitchShifterState,
};
use reverb::{
    process_convolution_reverb, process_matrix_reverb, process_room_verb, ConvolutionReverbState, ReverbState,
};
use third_party::{
    process_izotope_trash, process_mastering_suite, process_mcdsp_futzbox, FutzBoxState, MasteringSuiteState,
};

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
    GuitarDistortion(GuitarDistortionState),
    Reverb(ReverbState),
    PitchShifter(PitchShifterState),
    Harmonizer(HarmonizerState),
    TimeStretch {
        lines: Vec<DelayLine>,
        phase: f32,
    },
    ConvolutionReverb(ConvolutionReverbState),
    McDspFutzBox(FutzBoxState),
    IzotopeTrash {
        filter_states: Vec<[[f32; 2]; 2]>,
    },
    MasteringSuite(MasteringSuiteState),
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
        if let DspState::ConvolutionReverb(st) = &mut inst.state {
            st.ir_media_id = fx
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
            FxParams::GuitarDistortion(_) => DspState::GuitarDistortion(GuitarDistortionState::default()),
            FxParams::MatrixReverb(_) | FxParams::RoomVerb(_) => DspState::Reverb(ReverbState::new()),
            FxParams::PitchShifter(_) => DspState::PitchShifter(PitchShifterState::new()),
            FxParams::Harmonizer(_) => DspState::Harmonizer(HarmonizerState::new()),
            FxParams::TimeStretch(_) => DspState::TimeStretch {
                lines: vec![DelayLine::new(8192); 2],
                phase: 0.0,
            },
            FxParams::ConvolutionReverb(_) => DspState::ConvolutionReverb(ConvolutionReverbState::new()),
            FxParams::McDspFutzBox(_) => DspState::McDspFutzBox(FutzBoxState::default()),
            FxParams::IzotopeTrash(_) => DspState::IzotopeTrash {
                filter_states: Vec::new(),
            },
            FxParams::MasteringSuite(_) => DspState::MasteringSuite(MasteringSuiteState::default()),
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
        if let DspState::ConvolutionReverb(st) = &mut inst.state {
            if let Some(mid) = st.ir_media_id {
                if let Some(bytes) = project.read_media_by_id(mid) {
                    if let Ok(ir) = PreprocessedIr::parse(&bytes) {
                        st.convolver = Some(Box::new(PartitionedConvolver::new(ir)));
                    }
                }
            }
        }
        Some(inst)
    }

    /// True if this instance is a Convolution Reverb with a loaded IR convolver.
    pub fn has_convolution_ir(&self) -> bool {
        matches!(&self.state, DspState::ConvolutionReverb(st) if st.convolver.is_some())
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

        match (&eff_params, &mut self.state) {
            (FxParams::Gain(p), DspState::Gain) => {
                process_gain(p, buf, lfe_channel);
                None
            }
            (FxParams::ParametricEq(p), DspState::ParametricEq { states }) => {
                process_parametric_eq(p, states, buf, lfe_channel);
                None
            }
            (FxParams::Delay(p), DspState::Delay { lines }) => {
                process_delay(p, lines, buf, lfe_channel);
                None
            }
            (FxParams::StereoDelay(p), DspState::StereoDelay { lines, fb_filter_state }) => {
                process_stereo_delay(p, lines, fb_filter_state, buf);
                None
            }
            (FxParams::Dynamics(p), DspState::Dynamics { env }) => {
                process_dynamics(p, env, buf, lfe_channel);
                None
            }
            (
                FxParams::Meter(p),
                DspState::Meter {
                    env_db,
                    hold_remaining_s,
                    held_peak_db,
                },
            ) => process_meter(p, env_db, hold_remaining_s, held_peak_db, buf),
            (FxParams::Tremolo(p), DspState::Tremolo { phase }) => {
                process_tremolo(p, phase, buf, lfe_channel);
                None
            }
            (FxParams::Flanger(p), DspState::Flanger { lines, phase }) => {
                process_flanger(p, lines, phase, buf, lfe_channel);
                None
            }
            (FxParams::GuitarDistortion(p), DspState::GuitarDistortion(st)) => {
                process_guitar_distortion(p, st, buf);
                None
            }
            (FxParams::MatrixReverb(p), DspState::Reverb(st)) => {
                process_matrix_reverb(p, st, buf, lfe_channel);
                None
            }
            (FxParams::RoomVerb(p), DspState::Reverb(st)) => {
                process_room_verb(p, st, buf, lfe_channel);
                None
            }
            (FxParams::PitchShifter(p), DspState::PitchShifter(st)) => {
                process_pitch_shifter(p, st, buf, lfe_channel);
                None
            }
            (FxParams::Harmonizer(p), DspState::Harmonizer(st)) => {
                process_harmonizer(p, st, buf, lfe_channel);
                None
            }
            (FxParams::TimeStretch(p), DspState::TimeStretch { lines, phase }) => {
                process_time_stretch(p, lines, phase, buf);
                None
            }
            (FxParams::ConvolutionReverb(p), DspState::ConvolutionReverb(st)) => {
                process_convolution_reverb(p, st, project, buf, lfe_channel);
                None
            }
            (FxParams::McDspFutzBox(p), DspState::McDspFutzBox(st)) => {
                process_mcdsp_futzbox(p, st, buf);
                None
            }
            (FxParams::IzotopeTrash(p), DspState::IzotopeTrash { filter_states }) => {
                process_izotope_trash(p, filter_states, buf);
                None
            }
            (FxParams::MasteringSuite(p), DspState::MasteringSuite(st)) => {
                process_mastering_suite(p, st, buf);
                None
            }
            (FxParams::QAnalyzer(_), DspState::QAnalyzer { peak_db }) => {
                process_q_analyzer(peak_db, buf);
                None
            }
            _ => None,
        }
    }
}
