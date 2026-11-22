//! D3D12 semantics on Metal.
//!
//! * [`caps`], [`format`]: what the device reports (`CheckFeatureSupport`)
//!   and the DXGI_FORMAT → Metal table.
//! * [`metal`]: the Metal 4 backend — device, queues, resources, descriptor
//!   heaps, command recording, fences, swapchain presentation.
//!
//! The hybrid runtime's COM layer (`win32-rt/src/{dxgi,d3d12}`) is a thin
//! `extern "win64"` shim over this crate; native tools (trace replay) link it
//! directly. Builds for macOS (arm64 / Rosetta x86_64) and iOS (arm64).
#![cfg(any(target_os = "macos", target_os = "ios"))]

pub mod caps;
pub mod format;
pub mod format_ext;
pub mod layout;
pub mod mesh;
pub mod metal;
pub mod rootsig;
pub mod rt;
pub mod state;
pub mod topology;
pub mod trace;
pub mod types;
