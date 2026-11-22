//! Wwise 3D positioning (milestone A4, docs/AUDIO.md §8): emitter/listener geometry,
//! attenuation, spread/focus/cone, the 3D speaker panner, obstruction/occlusion.
//!
//! * [`vbap`] — speaker setup (`SetSpeakerAngles`) and the planar VBAP pair solve.
//! * [`panner`] — `CAkSpeakerPan` 3D matrix (spread point sets, focus, height spread, 2D blend).

pub mod atten;
pub mod panner;
pub mod vbap;

pub use panner::{positioning_matrix, Cfg, PanParams, Ray3d};
pub use vbap::SpeakerSetup;
