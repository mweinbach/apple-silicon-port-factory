//! DXR on Metal 4, behind `AW2_RAYTRACING=1` (docs/design/DXR.md, review ADVICE_DXR.md).
//!
//! * [`d3d`]: the D3D12 structures and owned build inputs.
//! * [`accel`]: acceleration structures (header at the D3D VA, Metal AS on the side).
//! * [`kernels`]: utility compute kernels (instance conversion, header/postbuild writes).

pub mod accel;
pub mod d3d;
pub mod kernels;
pub mod pipeline;

use objc2_metal::MTLDevice;
use std::sync::OnceLock;

/// `AW2_RAYTRACING=1` and the device supports ray tracing. Off by default: M1/M2 have no
/// hardware ray tracing, and DXR is only reported (tier 1.1) when this is on.
pub fn enabled() -> bool {
    static E: OnceLock<bool> = OnceLock::new();
    *E.get_or_init(|| {
        let want = std::env::var("AW2_RAYTRACING").is_ok_and(|v| v == "1");
        if !want {
            return false;
        }
        let Some(d) = crate::metal::device() else { return false };
        let supported = d.dev.supportsRaytracing();
        let name = d.dev.name().to_string();
        let hw = d.dev.supportsFamily(objc2_metal::MTLGPUFamily::Apple9);
        log::info!(target: "d3d", "rt: AW2_RAYTRACING=1 on {name}: supportsRaytracing={supported}, hardware RT (Apple9+)={hw}");
        supported
    })
}
