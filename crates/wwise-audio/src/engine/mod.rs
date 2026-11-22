//! Native offline Wwise runtime core (milestone A3, docs/AUDIO.md §8).
//!
//! * [`project`] — object registry over loaded banks, media location (bank DIDX / streamed).
//! * [`params`] — actor-mixer property accumulation, RTPCs at default values, bus chain.
//! * [`resample`] — AK's resampler/pitch shifter (decompiled).
//! * [`mixer`] — AK's mixer gain-ramp kernel (decompiled SSE path).
//! * [`math`] — AK fast dB->lin, curve shapes, transitions.
//! * [`filter`] — AK connection LPF/HPF (cutoff mapping, Butterworth biquad, block-of-4 kernel).
//! * [`speakers`] — channel layouts, direct-assignment matrices and downmix tables.
//! * [`render`] — event/action engine, containers, voices, bus mix, offline render.
//! * [`rng`] — AK's global random generator and draw shapes.
//! * [`ranseq`] — RanSeq selection state (random info / sequence info / loop info).

pub mod convolver;
pub mod filter;
pub mod fx;
pub mod math;
pub mod mixer;
pub mod music;
pub mod params;
pub mod project;
pub mod ranseq;
pub mod render;
pub mod resample;
pub mod rng;
pub mod spatial;
pub mod speakers;

pub use params::GameState;
pub use project::Project;
pub use render::{ExternalSourceDesc, RenderOptions, Renderer, RendererState};

