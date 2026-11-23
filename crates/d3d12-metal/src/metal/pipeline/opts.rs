//! Environment-variable option switches, cache key salts, fragment UAV write
//! flag, and typed fixed-function render PSO state key.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

macro_rules! pso_default_on_opt {
    ($(#[$meta:meta])* $name:ident, $env:literal) => {
        $(#[$meta])*
        pub fn $name() -> bool {
            static V: OnceLock<bool> = OnceLock::new();
            *V.get_or_init(|| std::env::var_os($env).is_none_or(|v| v != "0"))
        }
    };
}

macro_rules! pso_default_off_opt {
    ($(#[$meta:meta])* $name:ident, $env:literal) => {
        $(#[$meta])*
        pub fn $name() -> bool {
            static V: OnceLock<bool> = OnceLock::new();
            *V.get_or_init(|| std::env::var_os($env).is_some_and(|v| v == "1"))
        }
    };
}

pso_default_on_opt!(
    /// `AW2_FAST_BIND_PSO`: dirty-track `MTL4ArgumentTable` buffer/VB bindings on
    /// `CommandList`, deduplicate `compute_clear_batch` encoder state, and pin
    /// internal MSL compute PSO threadgroup sizes (`[16,16,1]` for 2D clear/upscale,
    /// `[256,1,1]` for buffer fill, `[64,1,1]` for indirect expanders).
    /// Default **on** (`AW2_FAST_BIND_PSO=0` disables).
    fast_bind_pso,
    "AW2_FAST_BIND_PSO"
);

pso_default_on_opt!(
    /// `AW2_SHADER_IMMUTABLE_OPT`: specialize Metal 4 compute/graphics/mesh PSO descriptors
    /// (`MTLShaderValidation::Disabled`, `MTL4ShaderReflection::None`, `support*BinaryLinking = false`,
    /// compute/mesh `MTL4IndirectCommandBufferSupportState::Disabled`, explicit `Identity` color
    /// attachment mapping, mesh SIMD32 execution-width hints) and force `MTLMathMode::Fast` on
    /// internal MSL helper libraries, while keeping `.aw2s` DXIL->metallib disk cache keys untouched.
    /// Default **on** (`AW2_SHADER_IMMUTABLE_OPT=0` disables).
    shader_immutable_opt,
    "AW2_SHADER_IMMUTABLE_OPT"
);

/// PSO-cache key salt when `AW2_SHADER_IMMUTABLE_OPT=1` is active (`"IMMUT_OPT1"`).
pub const IMMUTABLE_OPT_KEY_SALT: u128 = 0x494d_4d55_545f_4f50_5431_u128;

pso_default_off_opt!(
    /// `AW2_SHADER_PSO_OPT_V2=1`: refine SM 6.6 `GraphicsPipeline` / `MeshPipeline`
    /// `writes_uav` and `pre_raster_reads` from MSC's cached `"ResourceUsages"` reflection
    /// bitmask (zero `.aw2s` cold misses), and specialize Metal 4 Render/Mesh PSO descriptors
    /// (elide dead AGX TBDR blend-read preambles on `write_mask == None` or identity
    /// `1*Src + 0*Dst` attachments, pin `maxVertexAmplificationCount = 1` and
    /// `MTL4AlphaToOneState::Disabled`).
    /// Default **off** (`AW2_SHADER_PSO_OPT_V2=1` enables).
    shader_pso_opt_v2,
    "AW2_SHADER_PSO_OPT_V2"
);

/// PSO-cache key salt when `AW2_SHADER_PSO_OPT_V2=1` is active (`"PSO_OPT2"`).
pub const SHADER_PSO_OPT_V2_SALT: u128 = 0x5053_4f5f_4f50_5432_u128;

pso_default_off_opt!(
    /// `AW2_MSC_COMPUTE_ACCESS=1` (default off): refine `ComputeAccess` (`uav_slots`,
    /// `read_slots`, `bounded_read_slots`, `uav_classes`, `read_classes`) from MSC's
    /// cached `"UsedResources"` and `"ResourceUsages"` reflection JSON without
    /// touching `.aw2s` shader cache keys or PSO archive keys.
    msc_compute_access,
    "AW2_MSC_COMPUTE_ACCESS"
);

pso_default_off_opt!(
    /// `AW2_MSC_STAGE_USAGE=1` (default off): refine `GraphicsPipeline` / `MeshPipeline`
    /// `pre_raster_reads` & `writes_uav` and `ComputeAccess` `uav_classes` / `read_classes` /
    /// `uav_srv_classes` from MSC's cached `"ResourceUsages"` JSON with zero `.aw2s` or PSO
    /// archive cache key changes (`0` cold PSO compiles).
    msc_stage_usage,
    "AW2_MSC_STAGE_USAGE"
);

pso_default_off_opt!(
    /// `AW2_AIR_NO_RES_HEAP=1` (default off): when a cached `.aw2s` compute shader's embedded
    /// `.metallib` `RBUF` reflection proves `struct.res_desc_heap_ab` (`buffer(0)`) is unused
    /// (`unused == 1`), bypass DXIL `-HV 2021` `SFI0` `RESOURCE_HEAP_INDEXING` opacity and
    /// refine `ComputeAccess` (`uav_slots`, `read_slots`, `bounded_read_slots`) from MSC
    /// `"UsedResources"` + `"ResourceUsages"` with zero PSO salt.
    air_no_res_heap,
    "AW2_AIR_NO_RES_HEAP"
);

pso_default_on_opt!(
    /// `AW2_SM66_GFX_STAGE_EXACT` (default **on**; `AW2_SM66_GFX_STAGE_EXACT=0` disables):
    /// zero-salt MSC `"ResourceUsages"` + `metallib_res_desc_heap_unused` + `"UsedResources"`
    /// CBV/SRV/UAV inspection on SM 6.5 / SM 6.6 `GraphicsPipeline` and `MeshPipeline` stages
    /// (`0` cold PSO compiles).
    sm66_gfx_stage_exact,
    "AW2_SM66_GFX_STAGE_EXACT"
);

pso_default_off_opt!(
    /// `AW2_CS_MID_QB_EXACT_V2=1` (default off): zero-salt MSC `"ResourceUsages"` +
    /// `metallib_res_desc_heap_unused` compute stage classification paired with sound
    /// `stage_gen`-stamped mid-compute `Transition` deferral (`0` cold PSO compiles).
    cs_mid_qb_exact_v2,
    "AW2_CS_MID_QB_EXACT_V2"
);

pso_default_on_opt!(
    /// `AW2_SM66_CS_STAGE_EXACT` (default **on**, `AW2_SM66_CS_STAGE_EXACT=0` disables): zero-salt
    /// compute shader reflection exactness (`UsedResources` + `ResourceUsages` +
    /// `metallib_res_desc_heap_unused` + exact CBV buffer check) on `ComputeAccess` without
    /// `cs_mid_qb_exact_v2`'s `cs_end_after` side-effects.
    sm66_cs_stage_exact,
    "AW2_SM66_CS_STAGE_EXACT"
);

pso_default_on_opt!(
    /// `AW2_CS_PSO_TG`: specialize `MTL4ComputePipelineDescriptor` with DXIL `[numthreads(x, y, z)]`
    /// (`maxTotalThreadsPerThreadgroup`, `requiredThreadsPerThreadgroup`, and SIMD32 multiple hint).
    /// Default **on** (`AW2_CS_PSO_TG=0` disables).
    cs_pso_tg,
    "AW2_CS_PSO_TG"
);

pso_default_on_opt!(
    /// `AW2_GFX_LAZY_ICB`: compile graphics PSOs with `MTL4IndirectCommandBufferSupportState::Disabled`
    /// by default and lazily compile an `Enabled` variant only when `execute_indirect_icb` uses the PSO.
    /// Default **on** (`AW2_GFX_LAZY_ICB=0` disables).
    gfx_lazy_icb,
    "AW2_GFX_LAZY_ICB"
);

static ANY_FRAG_BUF_WRITE: AtomicBool = AtomicBool::new(false);

/// True once any compiled pixel shader with buffer UAV writes (`(ru & 4) != 0` or opaque UAV)
/// or any occlusion query (`visibilityResultBuffer`) has been created/started in this process.
#[inline]
pub fn any_frag_buf_write() -> bool {
    ANY_FRAG_BUF_WRITE.load(Ordering::Relaxed)
}

#[inline]
pub fn note_frag_buf_write() {
    ANY_FRAG_BUF_WRITE.store(true, Ordering::Relaxed);
}

/// Typed render-pipeline fixed-function state key hashed directly into `KeyHasher`
/// without heap `format!` allocations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RenderPsoStateKey {
    pub blend: crate::state::BlendState,
    pub rtv_len: u8,
    pub rtv_formats: [u32; 8],
    pub sample_count: u32,
    pub extra: u32,
}

impl RenderPsoStateKey {
    #[inline]
    pub(crate) fn new(
        blend: crate::state::BlendState,
        rtv_formats: &[u32],
        sample_count: u32,
        extra: u32,
    ) -> Self {
        let mut fmts = [0u32; 8];
        let n = rtv_formats.len().min(8);
        fmts[..n].copy_from_slice(&rtv_formats[..n]);
        Self { blend, rtv_len: n as u8, rtv_formats: fmts, sample_count, extra }
    }

    pub(crate) fn write_to(&self, key: &mut dxil_metal::hash::KeyHasher) {
        let flags = (self.blend.alpha_to_coverage as u32)
            | ((self.blend.dual_source as u32) << 1)
            | ((self.blend.uses_blend_constant as u32) << 2)
            | ((self.rtv_len as u32) << 8);
        key.u32(flags);
        for a in &self.blend.attachments {
            key.u32(a.enabled as u32)
                .u32(a.src_rgb.0 as u32)
                .u32(a.dst_rgb.0 as u32)
                .u32(a.op_rgb.0 as u32)
                .u32(a.src_alpha.0 as u32)
                .u32(a.dst_alpha.0 as u32)
                .u32(a.op_alpha.0 as u32)
                .u32(a.write_mask.0 as u32);
        }
        key.u32(self.blend.unsupported_logic_ops.len as u32);
        for &(idx, op) in self.blend.unsupported_logic_ops.as_slice() {
            key.u32(idx as u32).u32(op);
        }
        key.u32(self.blend.invalid.len as u32);
        for &(idx, v) in self.blend.invalid.as_slice() {
            key.u32(idx as u32).u32(v);
        }
        for &f in &self.rtv_formats[..self.rtv_len as usize] {
            key.u32(f);
        }
        key.u32(self.sample_count).u32(self.extra);
    }
}
