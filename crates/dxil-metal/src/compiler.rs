//! DXIL -> metallib compilation (uncached). See [`crate::ShaderPipeline`] for the cached API.

use std::ffi::CString;

use crate::dxbc::{self, Container};
use crate::ffi::{self, *};
use crate::hash::{hash128, KeyHasher};
use crate::reflection::{IrReflection, ShaderReflection, ShaderStage};
use crate::root_signature::{flags as rs_flags, take_error, DescriptorRangeType, RootDescriptorKind, RootParameterKind, RootSignature, RootSignatureDesc};
use crate::Error;

/// `IRGPUFamily`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuFamily {
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,
    /// M3/M4 family (M3 Ultra is Apple9).
    Apple9 = 1009,
    Apple10 = 1010,
    Metal3 = 5001,
}

/// `IRStageInCodeGenerationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StageInMode {
    /// Vertex fetch through a `MTLVertexDescriptor` at attribute
    /// `STAGE_IN_ATTRIBUTE_START_INDEX + attribute_index`.
    #[default]
    MetalVertexFetch = 0,
    /// Vertex fetch through a separately synthesized stage-in function
    /// ([`crate::synthesize_stage_in`]) linked at pipeline creation.
    SeparateFunction = 1,
}

/// `IRInputTopology`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputTopology {
    Undefined = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
    Patch = 4,
}

/// `IRDualSourceBlendingConfiguration` / `IRDepthFeedbackConfiguration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Tristate {
    #[default]
    DecideAtRuntime = 0,
    ForceEnabled = 1,
    ForceDisabled = 2,
}

/// `IRCompatibilityFlags`.
pub mod compat {
    pub const BOUNDS_CHECK: u32 = 1 << 0;
    pub const VERTEX_POSITION_INF_TO_NAN: u32 = 1 << 1;
    pub const TEXTURE_MIN_LOD_CLAMP: u32 = 1 << 2;
    pub const SAMPLER_LOD_BIAS: u32 = 1 << 3;
    pub const POSITION_INVARIANCE: u32 = 1 << 4;
    pub const SAMPLE_NAN_TO_ZERO: u32 = 1 << 5;
    pub const TEX_WRITE_ROUNDING_RTZ: u32 = 1 << 6;
    pub const SUPPRESS_2D_COMPUTE_DERIVATIVE_ERRORS: u32 = 1 << 7;
    pub const FORCE_TEXTURE_ARRAY: u32 = 1 << 8;
    pub const DISABLE_NAN_INF_OPTIMIZATION: u32 = 1 << 9;
}

/// `IRCompilerValidationFlags`.
pub mod validation {
    pub const NONE: u32 = 0;
    pub const RAW_ROOT_RESOURCES: u32 = 1;
    pub const ALL_RESOURCES_BOUND: u32 = 1 << 1;
    pub const DXIL: u32 = 1 << 2;
}

/// `IRRayTracingPipelineConfiguration` contents. Must be identical for all shaders of one
/// ray tracing pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RayTracingConfig {
    pub max_attribute_size_in_bytes: u32,
    /// `IRRaytracingPipelineFlags` (0x100 skip triangles, 0x200 skip procedural).
    pub pipeline_flags: u32,
    pub closest_hit_intrinsics: u64,
    pub miss_intrinsics: u64,
    pub any_hit_intrinsics: u64,
    pub callable_intrinsics: u64,
    /// -1 = unlimited.
    pub max_recursive_depth: i32,
    /// `IRRayGenerationCompilationMode` (0 kernel, 1 visible function).
    pub ray_generation_mode: u32,
    /// `IRIntersectionFunctionCompilationMode` (0 visible fn, 1 intersection fn, 2 IFB fn).
    pub intersection_function_mode: u32,
    pub intersection_function_groups: bool,
    pub direct_state_access: bool,
}

impl Default for RayTracingConfig {
    fn default() -> Self {
        Self {
            max_attribute_size_in_bytes: 32,
            pipeline_flags: 0,
            closest_hit_intrinsics: 0x7FFF_FFFF,
            miss_intrinsics: 0x7FFF,
            any_hit_intrinsics: u64::MAX,
            callable_intrinsics: 0x703F,
            max_recursive_depth: -1,
            ray_generation_mode: 0,
            intersection_function_mode: 0,
            intersection_function_groups: false,
            direct_state_access: false,
        }
    }
}

/// Compiler configuration. Everything here is part of the cache key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompileOptions {
    pub min_gpu_family: GpuFamily,
    /// Minimum macOS version for code generation (e.g. "15.0.0"); `None` = converter default.
    pub min_macos: Option<String>,
    pub compatibility_flags: u32,
    pub validation_flags: u32,
    pub stage_in_mode: StageInMode,
    pub input_topology: Option<InputTopology>,
    pub geometry_tessellation_emulation: bool,
    pub dual_source_blending: Tristate,
    pub depth_feedback: Tristate,
    /// Bitmask of render targets with integer formats (fragment shaders).
    pub int_rt_mask: u8,
    /// Fragment output sample mask (`None` = all samples).
    pub sample_mask: Option<u32>,
    /// Rename the emitted Metal entry point.
    pub metal_function_name: Option<String>,
    pub ray_tracing: Option<RayTracingConfig>,
    /// `IRHitGroupType` (0 triangles, 1 procedural) for hit-group shaders.
    pub hit_group_type: Option<u32>,
    pub ignore_debug_info: bool,
    /// Ignore a root signature embedded in the DXIL (we always pass the PSO's root signature).
    pub ignore_embedded_root_signature: bool,
    pub function_constant_space: Option<u32>,
    pub framebuffer_fetch_space: Option<u32>,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            min_gpu_family: GpuFamily::Apple9,
            min_macos: Some(DEFAULT_MIN_MACOS.to_string()),
            compatibility_flags: 0,
            validation_flags: validation::NONE,
            stage_in_mode: StageInMode::MetalVertexFetch,
            input_topology: None,
            geometry_tessellation_emulation: false,
            dual_source_blending: Tristate::DecideAtRuntime,
            depth_feedback: Tristate::DecideAtRuntime,
            int_rt_mask: 0,
            sample_mask: None,
            metal_function_name: None,
            ray_tracing: None,
            hit_group_type: None,
            ignore_debug_info: true,
            ignore_embedded_root_signature: true,
            function_constant_space: None,
            framebuffer_fetch_space: None,
        }
    }
}

/// Default minimum deployment target: the host OS (macOS 27; converter 4.0.1 accepts it and
/// emits `air64-apple-macosx27.0.0`). Metal libraries built for it won't load on older macOS.
pub const DEFAULT_MIN_MACOS: &str = "27.0.0";

impl CompileOptions {
    pub(crate) fn hash_into(&self, k: &mut KeyHasher) {
        k.u32(self.min_gpu_family as u32);
        k.bytes(self.min_macos.as_deref().unwrap_or("").as_bytes());
        k.u32(self.compatibility_flags).u32(self.validation_flags).u32(self.stage_in_mode as u32);
        k.u32(self.input_topology.map_or(u32::MAX, |t| t as u32));
        k.u32(self.geometry_tessellation_emulation as u32);
        k.u32(self.dual_source_blending as u32).u32(self.depth_feedback as u32);
        k.u32(self.int_rt_mask as u32).u64(self.sample_mask.map_or(u64::MAX, |m| m as u64));
        k.bytes(self.metal_function_name.as_deref().unwrap_or("").as_bytes());
        match &self.ray_tracing {
            None => {
                k.u32(0);
            }
            Some(rt) => {
                k.u32(1).u32(rt.max_attribute_size_in_bytes).u32(rt.pipeline_flags);
                k.u64(rt.closest_hit_intrinsics).u64(rt.miss_intrinsics).u64(rt.any_hit_intrinsics).u64(rt.callable_intrinsics);
                k.u32(rt.max_recursive_depth as u32).u32(rt.ray_generation_mode).u32(rt.intersection_function_mode);
                k.u32(rt.intersection_function_groups as u32).u32(rt.direct_state_access as u32);
            }
        }
        k.u32(self.hit_group_type.unwrap_or(u32::MAX));
        k.u32(self.ignore_debug_info as u32).u32(self.ignore_embedded_root_signature as u32);
        k.u32(self.function_constant_space.unwrap_or(u32::MAX)).u32(self.framebuffer_fetch_space.unwrap_or(u32::MAX));
    }
}

/// One shader compilation.
#[derive(Clone, Copy)]
pub struct CompileRequest<'a> {
    /// DXIL container (`D3D12_SHADER_BYTECODE` contents).
    pub dxil: &'a [u8],
    /// DXIL entry point. Required for libraries (DXR exports); for other shaders `None` uses
    /// the module's single entry point.
    pub entry_point: Option<&'a str>,
    /// If set, compilation fails with [`Error::StageMismatch`] when the DXIL is another stage.
    pub expected_stage: Option<ShaderStage>,
    /// Global root signature (`None` = the converter's linear layout, debugging only).
    pub root_signature: Option<&'a RootSignature>,
    /// DXR local root signature.
    pub local_root_signature: Option<&'a RootSignature>,
    pub options: &'a CompileOptions,
}

impl<'a> CompileRequest<'a> {
    pub fn new(dxil: &'a [u8], root_signature: Option<&'a RootSignature>, options: &'a CompileOptions) -> Self {
        Self { dxil, entry_point: None, expected_stage: None, root_signature, local_root_signature: None, options }
    }
    pub fn entry_point(mut self, e: &'a str) -> Self {
        self.entry_point = Some(e);
        self
    }
    pub fn stage(mut self, s: ShaderStage) -> Self {
        self.expected_stage = Some(s);
        self
    }
    pub fn local_root_signature(mut self, rs: &'a RootSignature) -> Self {
        self.local_root_signature = Some(rs);
        self
    }

    /// Cache key: DXIL hash + root signature hashes + entry + options + converter identity.
    pub fn cache_key(&self, converter_fingerprint: &str) -> u128 {
        let mut k = KeyHasher::new("dxil-metal/compile/v1");
        k.u128(hash128(self.dxil));
        k.u128(self.root_signature.map_or(0, |r| r.hash()));
        k.u128(self.local_root_signature.map_or(0, |r| r.hash()));
        k.bytes(self.entry_point.unwrap_or("").as_bytes());
        self.options.hash_into(&mut k);
        k.bytes(converter_fingerprint.as_bytes());
        k.finish()
    }
}

/// A converted shader.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledShader {
    /// `MTLB` metallib bytes (load with `newLibraryWithData:`).
    pub metallib: Vec<u8>,
    /// Metal function name inside the metallib.
    pub function_name: String,
    pub stage: ShaderStage,
    pub reflection: ShaderReflection,
    /// Cache key this result is stored under.
    pub key: u128,
}

/// RAII wrappers.
struct Compiler(*mut IRCompiler);
impl Drop for Compiler {
    fn drop(&mut self) {
        if let Ok(api) = ffi::api() {
            unsafe { (api.IRCompilerDestroy)(self.0) };
        }
    }
}
struct Object(*mut IRObject);
impl Drop for Object {
    fn drop(&mut self) {
        if !self.0.is_null() {
            if let Ok(api) = ffi::api() {
                unsafe { (api.IRObjectDestroy)(self.0) };
            }
        }
    }
}
struct MetalLib(*mut IRMetalLibBinary);
impl Drop for MetalLib {
    fn drop(&mut self) {
        if let Ok(api) = ffi::api() {
            unsafe { (api.IRMetalLibBinaryDestroy)(self.0) };
        }
    }
}
impl MetalLib {
    fn new(api: &Api) -> Self {
        Self(unsafe { (api.IRMetalLibBinaryCreate)() })
    }
    fn bytes(&self, api: &Api) -> Vec<u8> {
        unsafe {
            let n = (api.IRMetalLibGetBytecodeSize)(self.0);
            let mut v = vec![0u8; n];
            (api.IRMetalLibGetBytecode)(self.0, v.as_mut_ptr());
            v
        }
    }
}
struct RtConfig(*mut IRRayTracingPipelineConfiguration);
impl Drop for RtConfig {
    fn drop(&mut self) {
        if let Ok(api) = ffi::api() {
            unsafe { (api.IRRayTracingPipelineConfigurationDestroy)(self.0) };
        }
    }
}

/// A configured compiler plus the objects it borrows (must outlive compilation).
type ConfiguredCompiler = (Compiler, Option<RtConfig>, Option<CString>, Option<CString>, Option<std::sync::Arc<RootSignature>>);

fn root_sig_covers(desc: &RootSignatureDesc, ty: DescriptorRangeType, space: u32, reg: u32) -> bool {
    if ty == DescriptorRangeType::Sampler && desc.static_samplers.iter().any(|s| s.register_space == space && s.shader_register == reg) {
        return true;
    }
    desc.parameters.iter().any(|p| match &p.kind {
        RootParameterKind::DescriptorTable(ranges) => ranges.iter().any(|r| {
            r.range_type == ty
                && r.register_space == space
                && reg >= r.base_shader_register
                && (r.num_descriptors == rs_flags::UNBOUNDED || reg - r.base_shader_register < r.num_descriptors)
        }),
        RootParameterKind::Constants { shader_register, register_space, .. } => {
            ty == DescriptorRangeType::Cbv && *register_space == space && *shader_register == reg
        }
        RootParameterKind::Descriptor(kind, d) => {
            d.register_space == space
                && d.shader_register == reg
                && matches!(
                    (kind, ty),
                    (RootDescriptorKind::Cbv, DescriptorRangeType::Cbv)
                        | (RootDescriptorKind::Srv, DescriptorRangeType::Srv)
                        | (RootDescriptorKind::Uav, DescriptorRangeType::Uav)
                )
        }
    })
}

/// When a DXR library shader is compiled with a global root signature and no local root signature,
/// any resource bindings in RDAT that are not covered by the global root signature (e.g. a
/// hit-group `cbuffer` in `space1`) are synthesized into a local root signature so
/// `createHandleForLib` lowers cleanly.
fn uncovered_local_root_signature(dxil: &[u8], global: &RootSignatureDesc) -> Option<std::sync::Arc<RootSignature>> {
    if global.parameters.is_empty() && global.static_samplers.is_empty() {
        return None;
    }
    let container = Container::parse(dxil).ok()?;
    if container.program()?.kind != dxbc::ProgramKind::Library {
        return None;
    }
    let bindings = container.resource_bindings()?;
    let uncovered: Vec<(DescriptorRangeType, u32, u32, u32)> = bindings
        .into_iter()
        .filter_map(|r| {
            let ty = DescriptorRangeType::from_raw(r.range_type)?;
            if root_sig_covers(global, ty, r.space, r.lower_bound) {
                return None;
            }
            let n = if r.upper_bound == u32::MAX { rs_flags::UNBOUNDED } else { r.upper_bound.saturating_sub(r.lower_bound) + 1 };
            Some((ty, r.space, r.lower_bound, n))
        })
        .collect();
    if uncovered.is_empty() {
        return None;
    }
    let mut desc = RootSignatureDesc::covering_bindings(uncovered);
    desc.flags = rs_flags::LOCAL_ROOT_SIGNATURE;
    RootSignature::new(desc).ok()
}

/// Creates and configures a compiler.
fn make_compiler(api: &Api, req: &CompileRequest) -> Result<ConfiguredCompiler, Error> {
    let o = req.options;
    unsafe {
        let c = Compiler((api.IRCompilerCreate)());
        if c.0.is_null() {
            return Err(Error::Convert { code: crate::ErrorCode::Unknown, message: "IRCompilerCreate returned null".into() });
        }
        (api.IRCompilerSetMinimumGPUFamily)(c.0, o.min_gpu_family as u32);
        let os = match &o.min_macos {
            Some(v) => {
                let s = CString::new(v.as_str()).map_err(|e| Error::InvalidInput(e.to_string()))?;
                (api.IRCompilerSetMinimumDeploymentTarget)(c.0, IROperatingSystem_macOS, s.as_ptr());
                Some(s)
            }
            None => None,
        };
        (api.IRCompilerSetCompatibilityFlags)(c.0, o.compatibility_flags);
        (api.IRCompilerSetValidationFlags)(c.0, o.validation_flags);
        (api.IRCompilerSetStageInGenerationMode)(c.0, o.stage_in_mode as u32);
        if let Some(t) = o.input_topology {
            (api.IRCompilerSetInputTopology)(c.0, t as u32);
        }
        (api.IRCompilerEnableGeometryAndTessellationEmulation)(c.0, o.geometry_tessellation_emulation);
        (api.IRCompilerSetDualSourceBlendingConfiguration)(c.0, o.dual_source_blending as u32);
        (api.IRCompilerSetDepthFeedbackConfiguration)(c.0, o.depth_feedback as u32);
        (api.IRCompilerSetIntRTMask)(c.0, o.int_rt_mask);
        if let Some(m) = o.sample_mask {
            (api.IRCompilerSetSampleMask)(c.0, m);
        }
        let name = match &o.metal_function_name {
            Some(n) => {
                let s = CString::new(n.as_str()).map_err(|e| Error::InvalidInput(e.to_string()))?;
                (api.IRCompilerSetEntryPointName)(c.0, s.as_ptr());
                Some(s)
            }
            None => None,
        };
        (api.IRCompilerIgnoreDebugInformation)(c.0, o.ignore_debug_info);
        (api.IRCompilerIgnoreRootSignature)(c.0, o.ignore_embedded_root_signature);
        if let Some(s) = o.function_constant_space {
            (api.IRCompilerSetFunctionConstantResourceSpace)(c.0, s);
        }
        if let Some(s) = o.framebuffer_fetch_space {
            (api.IRCompilerSetFramebufferFetchResourceSpace)(c.0, s);
        }
        if let Some(h) = o.hit_group_type {
            (api.IRCompilerSetHitgroupType)(c.0, h);
        }
        let rt = match &o.ray_tracing {
            Some(rt) => {
                let cfg = RtConfig((api.IRRayTracingPipelineConfigurationCreate)());
                (api.IRRayTracingPipelineConfigurationSetMaxAttributeSizeInBytes)(cfg.0, rt.max_attribute_size_in_bytes);
                (api.IRRayTracingPipelineConfigurationSetPipelineFlags)(cfg.0, rt.pipeline_flags);
                (api.IRRayTracingPipelineConfigurationSetIntrinsicMasks)(
                    cfg.0,
                    rt.closest_hit_intrinsics,
                    rt.miss_intrinsics,
                    rt.any_hit_intrinsics,
                    rt.callable_intrinsics,
                );
                (api.IRRayTracingPipelineConfigurationSetMaxRecursiveDepth)(cfg.0, rt.max_recursive_depth);
                (api.IRRayTracingPipelineConfigurationSetRayGenerationCompilationMode)(cfg.0, rt.ray_generation_mode);
                (api.IRRayTracingPipelineConfigurationSetIntersectionFunctionCompilationMode)(cfg.0, rt.intersection_function_mode);
                (api.IRRayTracingPipelineConfigurationEnableIntersectionFunctionGroups)(cfg.0, rt.intersection_function_groups);
                (api.IRRayTracingPipelineConfigurationEnableDirectStateAccess)(cfg.0, rt.direct_state_access);
                (api.IRCompilerSetRayTracingPipelineConfiguration)(c.0, cfg.0);
                Some(cfg)
            }
            None => None,
        };
        if let Some(rs) = req.root_signature {
            (api.IRCompilerSetGlobalRootSignature)(c.0, rs.raw()?);
        }
        let mut synth_lrs = None;
        if let Some(rs) = req.local_root_signature {
            (api.IRCompilerSetLocalRootSignature)(c.0, rs.raw()?);
        } else if let Some(global) = req.root_signature {
            if let Some(lrs) = uncovered_local_root_signature(req.dxil, global.desc()) {
                (api.IRCompilerSetLocalRootSignature)(c.0, lrs.raw()?);
                synth_lrs = Some(lrs);
            }
        }
        Ok((c, rt, os, name, synth_lrs))
    }
}

/// Compiles one shader without touching any cache.
pub fn compile_uncached(req: &CompileRequest) -> Result<CompiledShader, Error> {
    let api = ffi::api()?;
    let container = Container::parse(req.dxil)?;
    if container.is_sm4_bytecode() {
        // FXC output: convert to DXIL with dxilconv (optional library). A missing library is
        // `Error::Load`, which the cache doesn't persist, so installing it later takes effect.
        // The result keeps the key of the original DXBC bytes.
        let dxil = crate::dxilconv::convert_signed(req.dxil)?;
        let mut s = compile_uncached(&CompileRequest {
            dxil: &dxil,
            entry_point: req.entry_point.or(Some("main")),
            ..*req
        })?;
        s.key = req.cache_key(&api.fingerprint);
        return Ok(s);
    }
    if container.part(dxbc::FOURCC_DXIL).is_none() {
        return Err(Error::InvalidInput("container has no DXIL or SM4/SM5 bytecode part".into()));
    }
    // Graphics/compute PSOs carry no entry name: take it from PSV0. (Passing a wrong name makes
    // the converter return an empty object with stage Invalid rather than an error.)
    let is_library = container.program().is_some_and(|p| p.kind == dxbc::ProgramKind::Library);
    let entry_name = match req.entry_point {
        // An empty name crashes the converter (observed with DXIL libraries).
        Some("") => return Err(Error::InvalidInput("empty entry point name".into())),
        Some(e) => e.to_string(),
        None if is_library => match container.psv_entry_point() {
            Some(e) => e,
            None => return Err(Error::InvalidInput("library shaders need an export name".into())),
        },
        // PSV0 only records the name from version 3 (validator 1.8+). For older output, the
        // entry point is the one function the bitcode defines; "main" is a last resort.
        None => match container.entry_point() {
            Some(e) => e,
            None => "main".to_string(),
        },
    };
    // Converter 4.0.1 segfaults on an unknown export name in a library: validate against RDAT.
    if is_library {
        let funcs = container.library_functions();
        if !funcs.is_empty() && !funcs.iter().any(|f| f.unmangled_name == entry_name || f.name == entry_name) {
            let names: Vec<&str> = funcs.iter().map(|f| f.unmangled_name.as_str()).collect();
            return Err(Error::InvalidInput(format!("library has no export '{entry_name}' (exports: {})", names.join(", "))));
        }
    }
    let entry = CString::new(entry_name.as_str()).map_err(|e| Error::InvalidInput(e.to_string()))?;
    let (compiler, _rt, _os, _name, _lrs) = make_compiler(api, req)?;
    let lowered = crate::bitcode::lower_dxil_container(req.dxil);
    let dxil_bytes = lowered.as_deref().unwrap_or(req.dxil);
    unsafe {
        let input = Object((api.IRObjectCreateFromDXIL)(dxil_bytes.as_ptr(), dxil_bytes.len(), IRBytecodeOwnershipNone));
        if input.0.is_null() {
            return Err(Error::Convert { code: crate::ErrorCode::UnrecognizedDXILHeader, message: "IRObjectCreateFromDXIL returned null".into() });
        }
        let mut err: *mut IRError = std::ptr::null_mut();
        let out = Object((api.IRCompilerAllocCompileAndLink)(compiler.0, entry.as_ptr(), input.0, &mut err));
        if out.0.is_null() {
            return Err(take_error(api, err, "IRCompilerAllocCompileAndLink"));
        }
        if !err.is_null() {
            (api.IRErrorDestroy)(err);
        }
        finish(api, &out, req).map_err(|e| match e {
            Error::Convert { code, message } if message.starts_with("entry point") => {
                Error::Convert { code, message: format!("entry point '{entry_name}' not found in DXIL") }
            }
            e => e,
        })
    }
}

unsafe fn finish(api: &Api, out: &Object, req: &CompileRequest) -> Result<CompiledShader, Error> {
    let stage = ShaderStage::from_raw((api.IRObjectGetMetalIRShaderStage)(out.0));
    if stage == ShaderStage::Invalid {
        return Err(Error::Convert { code: crate::ErrorCode::Unknown, message: "entry point not found".into() });
    }
    if let Some(want) = req.expected_stage {
        if want != stage {
            return Err(Error::StageMismatch { expected: want, actual: stage });
        }
    }
    let lib = MetalLib::new(api);
    if !(api.IRObjectGetMetalLibBinary)(out.0, stage.to_raw(), lib.0) {
        return Err(Error::Convert { code: crate::ErrorCode::Unknown, message: format!("no metallib for stage {stage:?}") });
    }
    let metallib = lib.bytes(api);
    let refl = IrReflection((api.IRShaderReflectionCreate)());
    let reflection = if (api.IRObjectGetReflection)(out.0, stage.to_raw(), refl.0) {
        ShaderReflection::from_ir(api, refl.0, stage)
    } else {
        return Err(Error::Convert { code: crate::ErrorCode::Unknown, message: format!("no reflection for stage {stage:?}") });
    };
    Ok(CompiledShader {
        metallib,
        function_name: reflection.function_name.clone(),
        stage,
        reflection,
        key: req.cache_key(&api.fingerprint),
    })
}

/// Compiles a DXR hit group's intersection + any-hit pair into one fused Metal function
/// (`IRCompilerAllocCombineCompileAndLink`). Set `options.hit_group_type` first.
pub fn compile_combined_hit_group(
    intersection: Option<(&[u8], &str)>,
    any_hit: Option<(&[u8], &str)>,
    root_signature: Option<&RootSignature>,
    local_root_signature: Option<&RootSignature>,
    options: &CompileOptions,
) -> Result<CompiledShader, Error> {
    let api = ffi::api()?;
    let base = intersection.or(any_hit).ok_or_else(|| Error::InvalidInput("hit group needs intersection or any-hit".into()))?;
    let req = CompileRequest {
        dxil: base.0,
        entry_point: Some(base.1),
        expected_stage: None,
        root_signature,
        local_root_signature,
        options,
    };
    let (compiler, _rt, _os, _name, _lrs) = make_compiler(api, &req)?;
    unsafe {
        let mk = |p: Option<(&[u8], &str)>| -> Result<(Object, Option<CString>, Option<Vec<u8>>), Error> {
            match p {
                Some((b, n)) => {
                    let lowered = crate::bitcode::lower_dxil_container(b);
                    let bytes = lowered.as_deref().unwrap_or(b);
                    Ok((
                        Object((api.IRObjectCreateFromDXIL)(bytes.as_ptr(), bytes.len(), IRBytecodeOwnershipNone)),
                        Some(CString::new(n).map_err(|e| Error::InvalidInput(e.to_string()))?),
                        lowered,
                    ))
                }
                None => Ok((Object(std::ptr::null_mut()), None, None)),
            }
        };
        let (io, iname, _ilowered) = mk(intersection)?;
        let (ao, aname, _alowered) = mk(any_hit)?;
        let mut err = std::ptr::null_mut();
        let out = Object((api.IRCompilerAllocCombineCompileAndLink)(
            compiler.0,
            iname.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
            io.0,
            aname.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
            ao.0,
            &mut err,
        ));
        if out.0.is_null() {
            return Err(take_error(api, err, "IRCompilerAllocCombineCompileAndLink"));
        }
        finish(api, &out, &req)
    }
}

/// Bitmask of ray tracing intrinsics an export uses (`IRObjectGatherRaytracingIntrinsics`),
/// for [`RayTracingConfig`] intrinsic masks.
pub fn gather_raytracing_intrinsics(dxil: &[u8], entry_point: &str) -> Result<u64, Error> {
    let api = ffi::api()?;
    let e = CString::new(entry_point).map_err(|e| Error::InvalidInput(e.to_string()))?;
    let lowered = crate::bitcode::lower_dxil_container(dxil);
    let dxil_bytes = lowered.as_deref().unwrap_or(dxil);
    unsafe {
        let o = Object((api.IRObjectCreateFromDXIL)(dxil_bytes.as_ptr(), dxil_bytes.len(), IRBytecodeOwnershipNone));
        if o.0.is_null() {
            return Err(Error::InvalidInput("IRObjectCreateFromDXIL returned null".into()));
        }
        Ok((api.IRObjectGatherRaytracingIntrinsics)(o.0, e.as_ptr()))
    }
}

// ---- stage-in --------------------------------------------------------------------------------

/// Name of the function `IRMetalLibSynthesizeStageInFunction` emits.
pub const STAGE_IN_FUNCTION_NAME: &str = "irconverter_stage_in_shader";

/// One `D3D12_INPUT_ELEMENT_DESC`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputElement {
    pub semantic_name: String,
    pub semantic_index: u32,
    /// `DXGI_FORMAT`.
    pub format: u32,
    pub input_slot: u32,
    /// `D3D12_APPEND_ALIGNED_ELEMENT` (0xFFFFFFFF) is resolved by [`resolve_append_aligned`].
    pub aligned_byte_offset: u32,
    /// 0 per-vertex, 1 per-instance.
    pub input_slot_class: u32,
    pub instance_data_step_rate: u32,
}

/// Resolves `D3D12_APPEND_ALIGNED_ELEMENT` offsets per input slot, given a DXGI format size
/// function (`format -> bytes`).
pub fn resolve_append_aligned(elements: &mut [InputElement], format_size: impl Fn(u32) -> u32) {
    let mut next = std::collections::HashMap::<u32, u32>::new();
    for e in elements.iter_mut() {
        let cur = next.entry(e.input_slot).or_insert(0);
        if e.aligned_byte_offset == 0xFFFF_FFFF {
            // D3D12 aligns appended elements to 4 bytes (min of element size and 4).
            e.aligned_byte_offset = cur.next_multiple_of(4);
        }
        *cur = e.aligned_byte_offset + format_size(e.format);
    }
}

/// Synthesizes the separate stage-in function for a vertex shader compiled with
/// [`StageInMode::SeparateFunction`]. Returns metallib bytes.
pub fn synthesize_stage_in(vertex: &CompiledShader, layout: &[InputElement], options: &CompileOptions) -> Result<Vec<u8>, Error> {
    let api = ffi::api()?;
    if vertex.stage != ShaderStage::Vertex {
        return Err(Error::StageMismatch { expected: ShaderStage::Vertex, actual: vertex.stage });
    }
    if layout.len() > 31 {
        return Err(Error::InvalidInput(format!("input layout has {} elements (max 31)", layout.len())));
    }
    let refl = IrReflection::from_json(api, &vertex.reflection.json)?;
    let names: Vec<CString> = layout
        .iter()
        .map(|e| CString::new(e.semantic_name.as_str()))
        .collect::<Result<_, _>>()
        .map_err(|e| Error::InvalidInput(e.to_string()))?;
    let mut desc = IRVersionedInputLayoutDescriptor {
        version: IRInputLayoutDescriptorVersion_1,
        desc_1_0: IRInputLayoutDescriptor1 {
            semanticNames: [std::ptr::null(); 31],
            inputElementDescs: [IRInputElementDescriptor1::default(); 31],
            numElements: layout.len() as u32,
        },
    };
    for (i, e) in layout.iter().enumerate() {
        desc.desc_1_0.semanticNames[i] = names[i].as_ptr();
        desc.desc_1_0.inputElementDescs[i] = IRInputElementDescriptor1 {
            semanticIndex: e.semantic_index,
            format: e.format,
            inputSlot: e.input_slot,
            alignedByteOffset: e.aligned_byte_offset,
            instanceDataStepRate: e.instance_data_step_rate,
            inputSlotClass: e.input_slot_class,
        };
    }
    let dummy = CompileRequest::new(&[], None, options);
    let (compiler, ..) = make_compiler(api, &dummy)?;
    let lib = MetalLib::new(api);
    let ok = unsafe { (api.IRMetalLibSynthesizeStageInFunction)(compiler.0, refl.0, &desc, lib.0) };
    if !ok {
        return Err(Error::Convert {
            code: crate::ErrorCode::FailedToSynthesizeStageInFunction,
            message: "IRMetalLibSynthesizeStageInFunction failed".into(),
        });
    }
    Ok(lib.bytes(api))
}

/// [`synthesize_stage_in`] packaged as a [`CompiledShader`] (stage `StageIn`, function
/// [`STAGE_IN_FUNCTION_NAME`]) stored under `key`.
pub fn stage_in_shader(vertex: &CompiledShader, layout: &[InputElement], options: &CompileOptions, key: u128) -> Result<CompiledShader, Error> {
    let metallib = synthesize_stage_in(vertex, layout, options)?;
    Ok(CompiledShader {
        function_name: STAGE_IN_FUNCTION_NAME.to_string(),
        stage: ShaderStage::StageIn,
        reflection: ShaderReflection {
            stage: ShaderStage::StageIn,
            function_name: STAGE_IN_FUNCTION_NAME.to_string(),
            resources: Vec::new(),
            needs_function_constants: false,
            function_constants: Vec::new(),
            info: crate::reflection::StageInfo::None,
            json: String::new(),
        },
        metallib,
        key,
    })
}

/// Synthesizes the indirect ray dispatch kernel (`IRMetalLibSynthesizeIndirectRayDispatchFunction`).
pub fn synthesize_indirect_ray_dispatch(options: &CompileOptions) -> Result<Vec<u8>, Error> {
    let api = ffi::api()?;
    let dummy = CompileRequest::new(&[], None, options);
    let (compiler, ..) = make_compiler(api, &dummy)?;
    let lib = MetalLib::new(api);
    if !unsafe { (api.IRMetalLibSynthesizeIndirectRayDispatchFunction)(compiler.0, lib.0) } {
        return Err(Error::Convert { code: crate::ErrorCode::Unknown, message: "indirect ray dispatch synthesis failed".into() });
    }
    Ok(lib.bytes(api))
}

/// Synthesizes the indirect intersection function for the current hit group configuration.
pub fn synthesize_indirect_intersection(options: &CompileOptions) -> Result<Vec<u8>, Error> {
    let api = ffi::api()?;
    let dummy = CompileRequest::new(&[], None, options);
    let (compiler, ..) = make_compiler(api, &dummy)?;
    let lib = MetalLib::new(api);
    if !unsafe { (api.IRMetalLibSynthesizeIndirectIntersectionFunction)(compiler.0, lib.0) } {
        return Err(Error::Convert {
            code: crate::ErrorCode::FailedToSynthesizeIndirectIntersectionFunction,
            message: "indirect intersection synthesis failed".into(),
        });
    }
    Ok(lib.bytes(api))
}
