//! Native (arm64) tooling for `AW2_D3D12_TRACE` call traces (`d3d12_metal::trace`).
//!
//! * [`stats`]: per-op counts, frames, draws, object churn.
//! * [`stats::dump`]: one line per record.
//! * [`shaders`]: offline conversion of every root signature, shader and PSO in a trace,
//!   plus fixed-function state translation and an unsupported-feature tally.
//! * [`replay`]: ordered replay with an object table and API-misuse checks into a
//!   pluggable [`replay::Backend`] (currently [`replay::NullBackend`]).

#![cfg(target_os = "macos")]

pub mod pod;
pub mod replay;
pub mod shaders;
pub mod stats;

#[cfg(test)]
mod tests;
