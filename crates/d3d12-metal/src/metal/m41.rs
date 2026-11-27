//! Optional Metal 4.1 (`macOS 27` / `iOS 27` / MSL 4.1) feature switches.
//!
//! All Metal 4.1 enhancements are opt-in by default so baseline Metal 4 behavior,
//! shader/PSO disk cache keys, and macOS 26 compatibility remain untouched until
//! enabled via the master `PORT_METAL41=1` switch or any individual sub-knob:
//!
//! | Knob | Default | Purpose |
//! |---|---|---|
//! | `PORT_METAL41` | `0` | Master switch enabling all Metal 4.1 options below (unless individually overridden to `0`) |
//! | `PORT_METAL41_RTZ_FP` | follows `PORT_METAL41` | MSL 4.1 `MTLFloatingPointConversionRoundingModeTowardZero` / `-fmetal-rtz-fp-conversion` |
//! | `PORT_METAL41_SPIRV` | follows `PORT_METAL41` | `dxil-metal::spirv_metal` MSL 4.0/4.1 (`-std=metal4.1` + `naga` `(4, 0)`) |
//! | `PORT_METAL41_VIEW_MIN_LOD` | follows `PORT_METAL41` | Hardware `MTLTextureViewDescriptor.minLOD` for D3D12 `ResourceMinLODClamp` & D3D11 `SetResourceMinLOD` |
//! | `PORT_METAL41_BLOCK_READ` | follows `PORT_METAL41` | MSL 4.1 multipixel `block_read<2, 1>` + SIMD/acq-rel variants for internal `clear.rs` utility kernels |
//! | `PORT_METAL41_METALFX_OFFSETS` | follows `PORT_METAL41` | Zero-copy `MTL4FXTemporalScaler` content & output offsets (`colorContentOffsetX/Y`, `outputOffsetX/Y`, etc.) |
//! | `PORT_METAL41_RGB32_TBUF` | follows `PORT_METAL41` | Read-only `MTLPixelFormatRGB32{Float,Uint,Sint}` texture-buffer SRVs on `Apple10`+ GPUs |
//! | `PORT_METAL41_DIRECTML_GPU` | follows `PORT_METAL41` | GPU DirectML GEMM offload via MSL 4.1 `tensor_inline` + `mpp::tensor_ops::matmul2d` |
//! | `PORT_METAL41_FORWARD_PROGRESS` | `""` | `MTL4ComputePipelineDescriptor.forwardProgressUsage` (`auto`, `weak`, `simdgroup_parallel`) |
//! | `PORT_METAL41_CONTENTION_RELIEF` | `""` | `MTL4ComputePipelineDescriptor.contentionRelief` (`auto`, `none`) |
//! | `PORT_METAL41_PERSISTENT_KERNEL` | `0` | `MTL4ComputePipelineDescriptor.optimizeForPersistentKernel` (ignored when `forward_progress == Weak`) |

use super::parse_bool_knob;

fn env_bool_opt(name: &str) -> Option<bool> {
    std::env::var(name)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| parse_bool_knob(&v))
}

/// Master Metal 4.1 opt-in knob (`PORT_METAL41=1`, default `false`).
#[inline]
pub fn master_enabled() -> bool {
    env_bool_opt("PORT_METAL41").unwrap_or(false)
}

#[inline]
fn sub_knob(name: &str) -> bool {
    env_bool_opt(name).unwrap_or_else(master_enabled)
}

/// Hardware `MTLTextureViewDescriptor.minLOD` for D3D12 `ResourceMinLODClamp` and
/// D3D11 `SetResourceMinLOD` (`PORT_METAL41_VIEW_MIN_LOD`, follows `PORT_METAL41`).
#[inline]
pub fn view_min_lod_enabled() -> bool {
    sub_knob("PORT_METAL41_VIEW_MIN_LOD")
}

/// MSL 4.1 multipixel `block_read<2, 1>` + SIMD reduction / release-atomic utility kernels
/// (`PORT_METAL41_BLOCK_READ`, follows `PORT_METAL41`).
#[inline]
pub fn block_read_kernels_enabled() -> bool {
    sub_knob("PORT_METAL41_BLOCK_READ")
}

/// Zero-copy `MTL4FXTemporalScaler` content/output subrect offsets
/// (`PORT_METAL41_METALFX_OFFSETS`, follows `PORT_METAL41`).
#[inline]
pub fn metalfx_offsets_enabled() -> bool {
    sub_knob("PORT_METAL41_METALFX_OFFSETS")
}

/// Read-only `MTLPixelFormatRGB32{Float,Uint,Sint}` texture-buffer SRVs on `Apple10`+ GPUs
/// (`PORT_METAL41_RGB32_TBUF`, follows `PORT_METAL41`).
#[inline]
pub fn rgb32_texture_buffer_enabled() -> bool {
    sub_knob("PORT_METAL41_RGB32_TBUF")
}

/// GPU DirectML GEMM offload via MSL 4.1 `tensor_inline` + `mpp::tensor_ops::matmul2d`
/// (`PORT_METAL41_DIRECTML_GPU`, follows `PORT_METAL41`).
#[inline]
pub fn directml_gpu_enabled() -> bool {
    sub_knob("PORT_METAL41_DIRECTML_GPU")
}

/// `MTLForwardProgressUsage` (`MTLPipeline.h`, macOS 27 / iOS 27).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum ForwardProgressUsage {
    Automatic = 0,
    Weak = 1,
    SimdGroupParallel = 2,
}

/// `MTLContentionRelief` (`MTLTypes.h`, macOS 27 / iOS 27).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum ContentionRelief {
    Automatic = 0,
    None = 1,
}

/// Metal 4.1 `MTL4ComputePipelineDescriptor` scheduling configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ComputeSchedulingOpts {
    pub forward_progress: Option<ForwardProgressUsage>,
    pub contention_relief: Option<ContentionRelief>,
    pub optimize_for_persistent_kernel: bool,
}

impl ComputeSchedulingOpts {
    /// True when at least one non-default scheduling override is requested.
    #[inline]
    pub fn is_active(&self) -> bool {
        self.forward_progress.is_some()
            || self.contention_relief.is_some()
            || self.optimize_for_persistent_kernel
    }

    /// Reads `PORT_METAL41_FORWARD_PROGRESS`, `PORT_METAL41_CONTENTION_RELIEF`, and
    /// `PORT_METAL41_PERSISTENT_KERNEL` from the environment.
    pub fn from_env() -> Self {
        let forward_progress = std::env::var("PORT_METAL41_FORWARD_PROGRESS")
            .ok()
            .and_then(|v| match v.trim().to_ascii_lowercase().as_str() {
                "auto" | "automatic" => Some(ForwardProgressUsage::Automatic),
                "weak" => Some(ForwardProgressUsage::Weak),
                "simd" | "simdgroup" | "simdgroup_parallel" | "parallel" => {
                    Some(ForwardProgressUsage::SimdGroupParallel)
                }
                _ => None,
            });
        let contention_relief = std::env::var("PORT_METAL41_CONTENTION_RELIEF")
            .ok()
            .and_then(|v| match v.trim().to_ascii_lowercase().as_str() {
                "auto" | "automatic" => Some(ContentionRelief::Automatic),
                "none" | "off" | "0" => Some(ContentionRelief::None),
                _ => None,
            });
        let mut optimize_for_persistent_kernel =
            env_bool_opt("PORT_METAL41_PERSISTENT_KERNEL").unwrap_or(false);
        // Metal 4.1 contract (`MTL4ComputePipeline.h`): `optimizeForPersistentKernel = YES`
        // is invalid when `forwardProgressUsage == MTLForwardProgressUsageWeak`.
        if optimize_for_persistent_kernel && forward_progress == Some(ForwardProgressUsage::Weak) {
            log::warn!(
                target: "d3d",
                "m41: ignoring PORT_METAL41_PERSISTENT_KERNEL=1 because forwardProgressUsage=Weak is incompatible"
            );
            optimize_for_persistent_kernel = false;
        }
        Self { forward_progress, contention_relief, optimize_for_persistent_kernel }
    }

    /// Deterministic PSO cache key salt (0 when inactive so default PSO keys never change).
    pub fn cache_key_salt(&self) -> u128 {
        if !self.is_active() {
            return 0;
        }
        let fp = self.forward_progress.map_or(0u128, |v| (v as u128) + 1);
        let cr = self.contention_relief.map_or(0u128, |v| (v as u128) + 1);
        let pk = self.optimize_for_persistent_kernel as u128;
        0x4d34_315f_5343_4844_0000_0000_0000_0000_u128 | (fp << 8) | (cr << 4) | pk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_and_sub_knobs_override_cleanly() {
        let _lock = crate::metal::env_test_lock();
        let keys = [
            "PORT_METAL41",
            "PORT_METAL41_VIEW_MIN_LOD",
            "PORT_METAL41_BLOCK_READ",
            "PORT_METAL41_METALFX_OFFSETS",
            "PORT_METAL41_RGB32_TBUF",
            "PORT_METAL41_DIRECTML_GPU",
            "PORT_METAL41_FORWARD_PROGRESS",
            "PORT_METAL41_CONTENTION_RELIEF",
            "PORT_METAL41_PERSISTENT_KERNEL",
        ];
        let saved: Vec<_> = keys.iter().map(|&k| (k, std::env::var_os(k))).collect();
        for &k in &keys {
            std::env::remove_var(k);
        }
        assert!(!master_enabled());
        assert!(!view_min_lod_enabled());
        assert!(!block_read_kernels_enabled());
        assert!(!metalfx_offsets_enabled());
        assert!(!rgb32_texture_buffer_enabled());
        assert!(!directml_gpu_enabled());
        assert_eq!(ComputeSchedulingOpts::from_env().cache_key_salt(), 0);

        std::env::set_var("PORT_METAL41", "1");
        assert!(master_enabled());
        assert!(view_min_lod_enabled());
        assert!(block_read_kernels_enabled());
        assert!(metalfx_offsets_enabled());
        assert!(rgb32_texture_buffer_enabled());
        assert!(directml_gpu_enabled());

        std::env::set_var("PORT_METAL41_BLOCK_READ", "0");
        assert!(!block_read_kernels_enabled());
        assert!(view_min_lod_enabled());

        std::env::set_var("PORT_METAL41", "0");
        std::env::set_var("PORT_METAL41_VIEW_MIN_LOD", "1");
        assert!(!master_enabled());
        assert!(view_min_lod_enabled());

        // Weak + Persistent must auto-clear persistent flag per Metal 4.1 contract.
        std::env::set_var("PORT_METAL41_FORWARD_PROGRESS", "weak");
        std::env::set_var("PORT_METAL41_PERSISTENT_KERNEL", "1");
        let s = ComputeSchedulingOpts::from_env();
        assert_eq!(s.forward_progress, Some(ForwardProgressUsage::Weak));
        assert!(!s.optimize_for_persistent_kernel);
        assert_ne!(s.cache_key_salt(), 0);

        std::env::set_var("PORT_METAL41_FORWARD_PROGRESS", "simdgroup_parallel");
        std::env::set_var("PORT_METAL41_CONTENTION_RELIEF", "none");
        let s2 = ComputeSchedulingOpts::from_env();
        assert_eq!(s2.forward_progress, Some(ForwardProgressUsage::SimdGroupParallel));
        assert_eq!(s2.contention_relief, Some(ContentionRelief::None));
        assert!(s2.optimize_for_persistent_kernel);
        assert_ne!(s2.cache_key_salt(), s.cache_key_salt());

        for (k, v) in saved {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
    }
}
