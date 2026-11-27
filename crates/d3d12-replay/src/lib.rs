//! Native (arm64) tooling for D3D12 call traces (`d3d12_metal::trace`).
//!
//! * [`stats`]: per-op counts, frames, draws, object churn.
//! * [`stats::dump`]: one line per record.
//! * [`shaders`]: offline conversion of every root signature, shader and PSO in a trace,
//!   plus fixed-function state translation and an unsupported-feature tally.
//! * [`replay`]: ordered replay with an object table and API-misuse checks into a
//!   pluggable [`replay::Backend`].
//! * [`metal_backend`]: real Metal 4 execution backend ([`metal_backend::MetalBackend`])
//!   built on `d3d12-metal`.
//! * [`bench`]: repeatable hardware `gpu_busy` and frame-time benchmark runner.
//! * [`golden`]: GPU texture readback, PNG encode/decode, and pixel-hash verification.
//! * [`synthetic`]: self-contained synthetic D3D12 trace generators.

#![cfg(target_os = "macos")]

pub mod bench;
pub mod golden;
pub mod metal_backend;
pub mod pod;
pub mod replay;
pub mod shaders;
pub mod stats;
pub mod synthetic;

#[cfg(test)]
mod tests;
