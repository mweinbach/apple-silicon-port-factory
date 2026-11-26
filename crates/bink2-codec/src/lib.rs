//! Clean-room native Rust Bink 2 (`KB2f`..`KB2n`, integer path) decoder and
//! universal `KB2v` hardware-HEVC container codec.
//!
//! Shared between:
//! - `win32-rt` (`bink2w64.dll` runtime, supporting both native Bink 2 and `VideoToolbox` HEVC)
//! - `port-factory` (optional universal offline storage optimizer)
//! - Game-specific archive repackers (e.g. `aw2-data`)

pub mod codec;
pub mod dsp;
#[cfg(target_arch = "aarch64")]
mod dsp_neon;
pub mod hevc_container;
pub mod stream;
pub mod tables;
