//! DXIL -> Metal shader pipeline for the D3D12-on-Metal layer, built on Apple's Metal Shader
//! Converter (`libmetalirconverter.dylib`, `dlopen`ed; works in arm64 tools and the x86_64
//! loader alike).
//!
//! Typical D3D12 usage:
//! ```no_run
//! # fn demo(rs_blob: &[u8], vs: &[u8], ps: &[u8]) -> Result<(), dxil_metal::Error> {
//! use dxil_metal::{CompileOptions, ShaderPipeline, ShaderStage};
//! let p = ShaderPipeline::global()?;                      // AW2_SHADER_CACHE-configured
//! let rs = p.root_signature(rs_blob)?;                    // CreateRootSignature
//! let layout = rs.layout();                               // top-level argument buffer layout
//! let opts = CompileOptions::default();
//! let vs = p.compile_stage(ShaderStage::Vertex, vs, Some(&rs), &opts)?;   // CreateGraphicsPipelineState
//! let ps = p.compile_stage(ShaderStage::Fragment, ps, Some(&rs), &opts)?;
//! // vs.metallib -> newLibraryWithData:, vs.function_name -> newFunctionWithName:
//! # let _ = (layout, ps); Ok(()) }
//! ```

pub mod bitcode;
pub mod cache;
pub mod compiler;
pub mod dxbc;
pub mod dxilconv;
pub mod ffi;
pub mod formats;
pub mod hash;
pub mod helper;
pub mod manifest;
pub mod pe;
#[cfg(feature = "metal")]
pub mod pso_cache;
pub mod reflection;
pub mod rfx;
pub mod root_signature;
pub mod runtime;
pub mod spirv_metal;
mod wire;

pub use cache::{BackendKind, CacheConfig, CacheStats, ShaderCache, ShaderPipeline};
pub use compiler::{
    compat, compile_combined_hit_group, compile_uncached, gather_raytracing_intrinsics, resolve_append_aligned, stage_in_shader,
    synthesize_indirect_intersection, synthesize_indirect_ray_dispatch, synthesize_stage_in, validation, CompileOptions,
    CompileRequest, CompiledShader, GpuFamily, InputElement, InputTopology, RayTracingConfig, StageInMode, Tristate,
};
pub use reflection::{FunctionConstant, ShaderReflection, ShaderStage, StageInfo, VertexAttribute};
pub use root_signature::{
    DescriptorRange, DescriptorRangeType, ErrorCode, ResourceLocation, ResourceType, RootDescriptor, RootDescriptorKind,
    RootParameter, RootParameterKind, RootParameterLayout, RootParameterLayoutKind, RootSignature, RootSignatureDesc,
    RootSignatureLayout, RootSignatureVersion, StaticSampler, StaticSamplerTableLayout, TableRangeLayout, TopLevelArgumentBuffer,
};

/// Errors from this crate. Cloneable so cached failures can be handed to every requester.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// The converter library could not be loaded.
    Load(String),
    /// Malformed DXIL / root signature / arguments.
    InvalidInput(String),
    /// The converter rejected the input.
    Convert { code: ErrorCode, message: String },
    /// The DXIL is not the stage the caller asked for.
    StageMismatch { expected: ShaderStage, actual: ShaderStage },
    /// Cache I/O problem.
    Io(String),
    /// The `dxil-metald` helper could not be started, or died while converting (e.g. a
    /// converter crash on this input).
    Helper(String),
    /// A Metal API call failed (`pso_cache`).
    Metal(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Load(m) => write!(f, "metal shader converter unavailable: {m}"),
            Error::InvalidInput(m) => write!(f, "invalid input: {m}"),
            Error::Convert { message, .. } => write!(f, "conversion failed: {message}"),
            Error::StageMismatch { expected, actual } => write!(f, "expected {expected:?} shader, got {actual:?}"),
            Error::Io(m) => write!(f, "{m}"),
            Error::Helper(m) => write!(f, "shader helper: {m}"),
            Error::Metal(m) => write!(f, "metal: {m}"),
        }
    }
}

impl std::error::Error for Error {}

/// True when the converter library can be loaded (tests skip otherwise).
pub fn converter_available() -> bool {
    ffi::api().is_ok()
}
