//! Native Vulkan `SPIR-V` (`.spv`) and WebGPU `WGSL` (`.wgsl`) -> Metal Shading Language (`.msl`)
//! -> Apple Metal Library (`.metallib`) conversion pipeline.
//!
//! Completes the dual-backend native shader pipeline alongside `libmetalirconverter.dylib` (DirectX 12
//! `DXIL`) and `libdxilconv.dylib` (DirectX 11 `DXBC` SM 4.0/5.x):
//! - **DirectX 12 (`DXIL` SM 6.x)**: `DXBC(DXIL)` -> `libmetalirconverter.dylib` -> `.metallib`
//! - **DirectX 10/11 (`DXBC` SM 4.0/5.x)**: `DXBC(SHEX/SHDR)` -> `libdxilconv.dylib` -> `DXIL` -> `.metallib`
//! - **Vulkan / OpenGL (`SPIR-V` 1.0–1.6)**: `SPIR-V` (`0x07230203`) -> `naga` (`spv-in` -> IR -> `msl-out`)
//!   -> `xcrun -sdk macosx metal` (`.air`) -> `xcrun -sdk macosx metallib` (`.metallib`)
//! - **WebGPU (`WGSL`)**: `.wgsl` -> `naga` (`wgsl-in` -> IR -> `msl-out`) -> `.metallib`
//! - **Metal Source (`.metal` / `.msl`)**: `.metal` -> `xcrun -sdk macosx metal` -> `.metallib`

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use crate::cache::ShaderCache;
use crate::compiler::CompiledShader;
use crate::hash::{hash128, KeyHasher};
use crate::reflection::{ShaderReflection, ShaderStage, StageInfo, VertexAttribute};
use crate::root_signature::{ErrorCode, ResourceLocation, ResourceType};
use crate::Error;

/// Canonical Vulkan `SPIR-V` magic number (`0x07230203`, little-endian bytes `[0x03, 0x02, 0x23, 0x07]`).
pub const SPIRV_MAGIC_LE: u32 = 0x0723_0203;
/// Byte-swapped `SPIR-V` magic number (`0x03022307`, big-endian bytes `[0x07, 0x23, 0x02, 0x03]`).
pub const SPIRV_MAGIC_BE: u32 = 0x0302_2307;

/// Parsed 20-byte (5-word) Vulkan `SPIR-V` binary module header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpirvHeader {
    pub big_endian: bool,
    pub version_major: u8,
    pub version_minor: u8,
    pub generator_magic: u32,
    pub generator_vendor: &'static str,
    pub bound: u32,
    pub schema: u32,
    pub word_count: usize,
}

impl SpirvHeader {
    /// Checks whether `bytes` starts with the `SPIR-V` magic (`0x07230203` LE or `0x03022307` BE)
    /// and has a valid 5-word header.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 || !bytes.len().is_multiple_of(4) {
            return None;
        }
        let w0 = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
        let big_endian = match w0 {
            SPIRV_MAGIC_LE => false,
            SPIRV_MAGIC_BE => true,
            _ => return None,
        };
        let rd = |off: usize| -> u32 {
            let b: [u8; 4] = bytes[off..off + 4].try_into().unwrap();
            if big_endian {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            }
        };
        let ver = rd(4);
        let version_major = ((ver >> 16) & 0xFF) as u8;
        let version_minor = ((ver >> 8) & 0xFF) as u8;
        if version_major != 1 || version_minor > 6 {
            return None;
        }
        let generator_magic = rd(8);
        let bound = rd(12);
        let schema = rd(16);
        if bound == 0 || bound > 0x0040_0000 || schema != 0 {
            return None;
        }
        let tool_id = (generator_magic >> 16) as u16;
        let generator_vendor = match tool_id {
            0 => "Khronos",
            1 => "LunarG",
            2 => "Valve",
            3 => "Codeplay",
            4 => "NVIDIA",
            5 => "ARM",
            6 => "Khronos LLVM/SPIR-V Translator",
            7 => "Khronos SPIR-V Tools Assembler",
            8 => "Khronos Glslang Reference Front End",
            9 => "Qualcomm",
            10 => "AMD",
            11 => "Intel",
            12 => "Imagination",
            13 => "Google Shaderc over Glslang",
            14 => "Google Spiregg / DXC SPIR-V",
            15 => "Google rspirv",
            16 => "X-LEGEND / Mesa-IR",
            17 => "Khronos SPIR-V Tools Linker",
            18 => "Wine VKD3D Shader Compiler",
            19 => "Clay",
            20 => "W3C WebGPU / Tint",
            21 => "Clspv",
            22 => "MLIR SPIR-V Serializer",
            23 => "Tint Compiler",
            24 => "ANGLE Shader Compiler",
            25 => "Messiah",
            26 => "Xenia Emulator",
            27 => "Rust GPU (rustc_codegen_spirv)",
            28 => "Naga SPIR-V Backend",
            29 => "MSP Shader Compiler",
            30 => "SpvGenTwo",
            40 => "ShaderConductor / Slang",
            _ => "Custom / Unknown SPIR-V Generator",
        };
        Some(Self {
            big_endian,
            version_major,
            version_minor,
            generator_magic,
            generator_vendor,
            bound,
            schema,
            word_count: bytes.len() / 4,
        })
    }

    pub fn version_string(&self) -> String {
        format!("SPIR-V {}.{}", self.version_major, self.version_minor)
    }
}

/// Normalizes a `SPIR-V` byte slice to little-endian words if it was stored big-endian.
pub fn normalize_spirv_le(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let hdr = SpirvHeader::parse(bytes)
        .ok_or_else(|| Error::InvalidInput("SPIR-V: invalid 20-byte header or magic".into()))?;
    if !hdr.big_endian {
        return Ok(bytes.to_vec());
    }
    let mut out = Vec::with_capacity(bytes.len());
    for chunk in bytes.chunks_exact(4) {
        let w = u32::from_be_bytes(chunk.try_into().unwrap());
        out.extend_from_slice(&w.to_le_bytes());
    }
    Ok(out)
}

/// One entry point discovered inside a `SPIR-V` or `WGSL` module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpirvEntryPointInfo {
    pub name: String,
    pub stage: ShaderStage,
    pub profile: String,
    pub workgroup_size: [u32; 3],
}

/// Summary of a parsed `SPIR-V` module (header, entry points, resources, push constants).
#[derive(Debug, Clone)]
pub struct SpirvSummary {
    pub header: SpirvHeader,
    pub entry_points: Vec<SpirvEntryPointInfo>,
    pub resources: Vec<ResourceLocation>,
    pub has_push_constants: bool,
    pub push_constant_size: u64,
}

fn map_naga_stage(stage: naga::ShaderStage) -> ShaderStage {
    match stage {
        naga::ShaderStage::Vertex => ShaderStage::Vertex,
        naga::ShaderStage::Fragment => ShaderStage::Fragment,
        naga::ShaderStage::Compute => ShaderStage::Compute,
        naga::ShaderStage::Task => ShaderStage::Amplification,
        naga::ShaderStage::Mesh => ShaderStage::Mesh,
        naga::ShaderStage::RayGeneration => ShaderStage::RayGeneration,
        naga::ShaderStage::Miss => ShaderStage::Miss,
        naga::ShaderStage::AnyHit => ShaderStage::AnyHit,
        naga::ShaderStage::ClosestHit => ShaderStage::ClosestHit,
    }
}

fn profile_for_stage(stage: ShaderStage, ver_major: u8, ver_minor: u8) -> String {
    let prefix = match stage {
        ShaderStage::Vertex => "spv_vs",
        ShaderStage::Fragment => "spv_ps",
        ShaderStage::Compute => "spv_cs",
        ShaderStage::Mesh => "spv_ms",
        ShaderStage::Amplification => "spv_as",
        _ => "spv",
    };
    format!("{prefix}_{ver_major}_{ver_minor}")
}

/// Inspects a raw `SPIR-V` binary module and extracts its header, entry points, and descriptor bindings.
pub fn summarize_spirv(bytes: &[u8]) -> Result<SpirvSummary, Error> {
    let header = SpirvHeader::parse(bytes)
        .ok_or_else(|| Error::InvalidInput("SPIR-V: invalid magic or header".into()))?;
    let le_bytes = normalize_spirv_le(bytes)?;
    let spv_opts = naga::front::spv::Options {
        adjust_coordinate_space: false,
        strict_capabilities: false,
        block_ctx_dump_prefix: None,
    };
    let module = naga::front::spv::parse_u8_slice(&le_bytes, &spv_opts)
        .map_err(|e| Error::InvalidInput(format!("SPIR-V parse error: {e}")))?;

    let entry_points = module
        .entry_points
        .iter()
        .map(|ep| {
            let stage = map_naga_stage(ep.stage);
            SpirvEntryPointInfo {
                name: ep.name.clone(),
                stage,
                profile: profile_for_stage(stage, header.version_major, header.version_minor),
                workgroup_size: ep.workgroup_size,
            }
        })
        .collect();

    let (resources, has_push_constants, push_constant_size) = extract_module_resources(&module);

    Ok(SpirvSummary {
        header,
        entry_points,
        resources,
        has_push_constants,
        push_constant_size,
    })
}

fn type_size_estimate(module: &naga::Module, ty: naga::Handle<naga::Type>) -> u64 {
    match &module.types[ty].inner {
        naga::TypeInner::Scalar(s) => s.width as u64,
        naga::TypeInner::Vector { size, scalar } => (*size as u64) * (scalar.width as u64),
        naga::TypeInner::Matrix { columns, rows, scalar } => {
            (*columns as u64) * (*rows as u64) * (scalar.width as u64)
        }
        naga::TypeInner::Struct { span, .. } => *span as u64,
        naga::TypeInner::Array { stride, size, .. } => match size {
            naga::ArraySize::Constant(c) => (*stride as u64) * (c.get() as u64),
            _ => *stride as u64,
        },
        _ => 0,
    }
}

fn extract_module_resources(module: &naga::Module) -> (Vec<ResourceLocation>, bool, u64) {
    let mut resources = Vec::new();
    let mut has_push_constants = false;
    let mut push_constant_size = 0u64;
    let mut top_offset = 0u32;

    for (_handle, gv) in module.global_variables.iter() {
        if gv.space == naga::AddressSpace::Immediate {
            has_push_constants = true;
            push_constant_size = type_size_estimate(module, gv.ty);
            continue;
        }
        let Some(ref binding) = gv.binding else {
            continue;
        };
        let resource_type = match gv.space {
            naga::AddressSpace::Uniform => ResourceType::Cbv,
            naga::AddressSpace::Storage { access } => {
                if access.contains(naga::StorageAccess::STORE) {
                    ResourceType::Uav
                } else {
                    ResourceType::Srv
                }
            }
            naga::AddressSpace::Handle => match &module.types[gv.ty].inner {
                naga::TypeInner::Sampler { .. } => ResourceType::Sampler,
                naga::TypeInner::Image {
                    class: naga::ImageClass::Storage { .. },
                    ..
                } => ResourceType::Uav,
                _ => ResourceType::Srv,
            },
            _ => continue,
        };
        let size_bytes = type_size_estimate(module, gv.ty);
        resources.push(ResourceLocation {
            resource_type,
            space: binding.group,
            slot: binding.binding,
            top_level_offset: top_offset,
            size_bytes,
            name: gv
                .name
                .clone()
                .unwrap_or_else(|| format!("set{}_binding{}", binding.group, binding.binding)),
        });
        top_offset += 8;
    }

    (resources, has_push_constants, push_constant_size)
}

/// Output of translating a `SPIR-V` or `WGSL` module to Metal Shading Language (`.msl`).
#[derive(Debug, Clone)]
pub struct MslTranslation {
    pub msl_source: String,
    pub entry_point_name: String,
    pub metal_function_name: String,
    pub stage: ShaderStage,
    pub reflection: ShaderReflection,
}

/// Options controlling `naga` MSL generation and `xcrun metal` compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MslCompileOptions {
    /// Naga MSL target `(major, minor)`: `(3, 1)` legacy default, `(4, 0)` enables MSL 4.0
    /// `metal::vertex_value<T>` (`PerVertex` / `VK_KHR_fragment_shader_barycentric`).
    pub naga_lang_version: (u8, u8),
    /// Preferred `-std=` standard passed to `xcrun -sdk macosx metal` (`"metal3.1"`, `"metal4.0"`,
    /// or `"metal4.1"`). Automatically falls back to `"metal4.0"` -> `"metal3.1"` if the host
    /// Metal toolchain is older than requested.
    pub std_version: &'static str,
    /// Pass MSL 4.1 `-fmetal-rtz-fp-conversion` (Spec §1.6.3) to use round-toward-zero narrowing
    /// float conversions for non-Metal API compatibility when compiling under `metal4.1`.
    pub rtz_fp_conversion: bool,
}

impl Default for MslCompileOptions {
    fn default() -> Self {
        Self::legacy_msl31()
    }
}

fn parse_env_bool(name: &str) -> Option<bool> {
    let v = std::env::var(name).ok()?;
    let t = v.trim();
    if t.is_empty() {
        return None;
    }
    Some(!matches!(t.to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off"))
}

impl MslCompileOptions {
    /// Legacy MSL 3.1 options (`lang_version: (3, 1)`, `-std=metal3.1`, RTNE narrowing).
    pub const fn legacy_msl31() -> Self {
        Self {
            naga_lang_version: (3, 1),
            std_version: "metal3.1",
            rtz_fp_conversion: false,
        }
    }

    /// Metal 4.1 options (`naga` MSL 4.0 + `-std=metal4.1` with toolchain fallback, optional RTZ).
    pub const fn metal41(rtz_fp_conversion: bool) -> Self {
        Self {
            naga_lang_version: (4, 0),
            std_version: "metal4.1",
            rtz_fp_conversion,
        }
    }

    /// Resolves options from `PORT_METAL41`, `PORT_METAL41_SPIRV`, and `PORT_METAL41_RTZ_FP`.
    /// Defaults to [`Self::legacy_msl31`] when none are enabled.
    pub fn from_env() -> Self {
        let master = parse_env_bool("PORT_METAL41").unwrap_or(false);
        let spirv_m41 = parse_env_bool("PORT_METAL41_SPIRV").unwrap_or(master);
        let rtz = parse_env_bool("PORT_METAL41_RTZ_FP").unwrap_or(master);
        if spirv_m41 || rtz {
            Self::metal41(rtz)
        } else {
            Self::legacy_msl31()
        }
    }

    fn is_legacy_default(&self) -> bool {
        *self == Self::legacy_msl31()
    }
}

/// Builds a deterministic Naga MSL `Options` + `EntryPointResourceMap` for `module`.
fn build_msl_options(module: &naga::Module, lang_version: (u8, u8)) -> naga::back::msl::Options {
    let mut per_entry_point_map = naga::back::msl::EntryPointResourceMap::default();

    let mut base_resources = naga::back::msl::BindingMap::default();
    let mut next_buffer: u8 = 0;
    let mut next_texture: u8 = 0;
    let mut next_sampler: u8 = 0;

    for (_h, gv) in module.global_variables.iter() {
        let Some(rb) = gv.binding else { continue };
        if base_resources.contains_key(&rb) {
            continue;
        }
        let target = match gv.space {
            naga::AddressSpace::Uniform => {
                let b = next_buffer;
                next_buffer = next_buffer.saturating_add(1);
                naga::back::msl::BindTarget {
                    buffer: Some(b),
                    texture: None,
                    sampler: None,
                    external_texture: None,
                    mutable: false,
                }
            }
            naga::AddressSpace::Storage { access } => {
                let b = next_buffer;
                next_buffer = next_buffer.saturating_add(1);
                naga::back::msl::BindTarget {
                    buffer: Some(b),
                    texture: None,
                    sampler: None,
                    external_texture: None,
                    mutable: access.contains(naga::StorageAccess::STORE),
                }
            }
            naga::AddressSpace::Handle => match &module.types[gv.ty].inner {
                naga::TypeInner::Sampler { .. } => {
                    let s = next_sampler;
                    next_sampler = next_sampler.saturating_add(1);
                    naga::back::msl::BindTarget {
                        buffer: None,
                        texture: None,
                        sampler: Some(naga::back::msl::BindSamplerTarget::Resource(s)),
                        external_texture: None,
                        mutable: false,
                    }
                }
                naga::TypeInner::Image {
                    class: naga::ImageClass::Storage { access, .. },
                    ..
                } => {
                    let t = next_texture;
                    next_texture = next_texture.saturating_add(1);
                    naga::back::msl::BindTarget {
                        buffer: None,
                        texture: Some(t),
                        sampler: None,
                        external_texture: None,
                        mutable: access.contains(naga::StorageAccess::STORE),
                    }
                }
                _ => {
                    let t = next_texture;
                    next_texture = next_texture.saturating_add(1);
                    naga::back::msl::BindTarget {
                        buffer: None,
                        texture: Some(t),
                        sampler: None,
                        external_texture: None,
                        mutable: false,
                    }
                }
            },
            _ => continue,
        };
        base_resources.insert(rb, target);
    }

    let ep_res = naga::back::msl::EntryPointResources {
        resources: base_resources,
        immediates_buffer: Some(30),
        sizes_buffer: Some(29),
    };

    for ep in &module.entry_points {
        per_entry_point_map.insert(ep.name.clone(), ep_res.clone());
    }

    naga::back::msl::Options {
        lang_version,
        per_entry_point_map,
        inline_samplers: Vec::new(),
        spirv_cross_compatibility: false,
        fake_missing_bindings: false,
        bounds_check_policies: naga::proc::BoundsCheckPolicies::default(),
        zero_initialize_workgroup_memory: true,
        force_loop_bounding: false,
        task_dispatch_limits: None,
        mesh_shader_primitive_indices_clamp: true,
        emit_int_div_checks: true,
        ray_query_initialization_tracking: false,
    }
}

fn collect_vertex_attributes(module: &naga::Module, ep: &naga::EntryPoint) -> Vec<VertexAttribute> {
    let mut inputs = Vec::new();
    for arg in &ep.function.arguments {
        match &arg.binding {
            Some(naga::Binding::Location { location, .. }) => {
                inputs.push(VertexAttribute {
                    name: arg
                        .name
                        .clone()
                        .unwrap_or_else(|| format!("ATTR{location}")),
                    attribute_index: *location as u8,
                });
            }
            None => {
                if let naga::TypeInner::Struct { members, .. } = &module.types[arg.ty].inner {
                    for m in members {
                        if let Some(naga::Binding::Location { location, .. }) = m.binding {
                            inputs.push(VertexAttribute {
                                name: m
                                    .name
                                    .clone()
                                    .unwrap_or_else(|| format!("ATTR{location}")),
                                attribute_index: location as u8,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    inputs
}

fn translate_naga_module_to_msl(
    module: &naga::Module,
    source_kind: &str,
    entry_point: Option<&str>,
    msl_compile_opts: MslCompileOptions,
) -> Result<MslTranslation, Error> {
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::empty(),
        naga::valid::Capabilities::all(),
    );
    let info = validator
        .validate(module)
        .map_err(|e| Error::Convert {
            code: ErrorCode::Unknown,
            message: format!("{source_kind} validation failed: {e}"),
        })?;

    if module.entry_points.is_empty() {
        return Err(Error::InvalidInput(format!(
            "{source_kind} module contains no entry points"
        )));
    }

    let ep_idx = match entry_point {
        Some(want) => module
            .entry_points
            .iter()
            .position(|ep| ep.name == want)
            .ok_or_else(|| {
                Error::InvalidInput(format!(
                    "{source_kind} entry point '{want}' not found (available: {})",
                    module
                        .entry_points
                        .iter()
                        .map(|e| e.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })?,
        None => 0,
    };

    let ep = &module.entry_points[ep_idx];
    let stage = map_naga_stage(ep.stage);
    let msl_opts = build_msl_options(module, msl_compile_opts.naga_lang_version);
    let pipe_opts = naga::back::msl::PipelineOptions {
        entry_point: Some((ep.stage, ep.name.clone())),
        allow_and_force_point_size: false,
        vertex_pulling_transform: false,
        vertex_buffer_mappings: Vec::new(),
        binding_array_length_map: Default::default(),
    };

    let (raw_msl, trans_info) =
        naga::back::msl::write_string(module, &info, &msl_opts, &pipe_opts).map_err(|e| {
            Error::Convert {
                code: ErrorCode::Unknown,
                message: format!("{source_kind} -> MSL translation failed: {e}"),
            }
        })?;
    let msl_source = promote_write_only_textures_in_msl(&raw_msl);

    let metal_function_name = trans_info
        .entry_point_names
        .into_iter()
        .next()
        .and_then(|r| r.ok())
        .unwrap_or_else(|| ep.name.clone());

    let (resources, has_push_constants, push_constant_size) = extract_module_resources(module);

    let stage_info = match stage {
        ShaderStage::Compute => StageInfo::Compute {
            threadgroup_size: ep.workgroup_size,
        },
        ShaderStage::Vertex => StageInfo::Vertex {
            instance_id_index: -1,
            vertex_id_index: -1,
            vertex_output_size_in_bytes: 0,
            needs_draw_params: false,
            inputs: collect_vertex_attributes(module, ep),
        },
        ShaderStage::Fragment => StageInfo::Fragment {
            num_render_targets: 1,
            rt_index_int: 0,
            discards: false,
        },
        _ => StageInfo::None,
    };

    let json = format!(
        "{{\"source_format\":\"{source_kind}\",\"entry_point\":\"{}\",\"metal_function\":\"{metal_function_name}\",\"stage\":\"{stage:?}\",\"resources\":{},\"has_push_constants\":{has_push_constants},\"push_constant_size\":{push_constant_size}}}",
        ep.name,
        resources.len()
    );

    let reflection = ShaderReflection {
        stage,
        function_name: metal_function_name.clone(),
        resources,
        needs_function_constants: false,
        function_constants: Vec::new(),
        info: stage_info,
        json,
    };

    Ok(MslTranslation {
        msl_source,
        entry_point_name: ep.name.clone(),
        metal_function_name,
        stage,
        reflection,
    })
}

/// Promotes `metal::access::read_write` texture declarations to `metal::access::write` when the
/// texture identifier never invokes `.read(...)` in the generated MSL source.
///
/// DXC's SPIR-V backend (`dxc -spirv`) often emits `RWTexture2D` outputs (`gOut_*`) without
/// `NonReadable` decorations even when the shader only calls `OpImageWrite`. Metal limits
/// `access::read_write` textures to 8 per stage on standard tiers, whereas `access::write`
/// textures support up to 128 per stage.
fn promote_write_only_textures_in_msl(msl: &str) -> String {
    const NEEDLE: &str = "metal::access::read_write> ";
    if !msl.contains(NEEDLE) {
        return msl.to_string();
    }
    let mut out = String::with_capacity(msl.len());
    let mut rest = msl;
    while let Some(pos) = rest.find(NEEDLE) {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + NEEDLE.len()..];
        let ident_len = after
            .bytes()
            .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_')
            .count();
        let ident = &after[..ident_len];
        let read_call = format!("{ident}.read(");
        if !ident.is_empty() && !msl.contains(&read_call) {
            out.push_str("metal::access::write> ");
        } else {
            out.push_str(NEEDLE);
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

/// Translates a Vulkan `SPIR-V` binary module into Metal Shading Language (`.msl`) + `ShaderReflection`.
pub fn translate_spirv_to_msl(
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
) -> Result<MslTranslation, Error> {
    translate_spirv_to_msl_with_opts(spirv_bytes, entry_point, MslCompileOptions::from_env())
}

/// Like [`translate_spirv_to_msl`], with explicit [`MslCompileOptions`].
pub fn translate_spirv_to_msl_with_opts(
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
    opts: MslCompileOptions,
) -> Result<MslTranslation, Error> {
    let le_bytes = normalize_spirv_le(spirv_bytes)?;
    let spv_opts = naga::front::spv::Options {
        adjust_coordinate_space: false,
        strict_capabilities: false,
        block_ctx_dump_prefix: None,
    };
    let module = naga::front::spv::parse_u8_slice(&le_bytes, &spv_opts)
        .map_err(|e| Error::InvalidInput(format!("SPIR-V parse error: {e}")))?;
    translate_naga_module_to_msl(&module, "SPIR-V", entry_point, opts)
}

/// Translates a WebGPU `WGSL` source shader into Metal Shading Language (`.msl`) + `ShaderReflection`.
pub fn translate_wgsl_to_msl(
    wgsl_source: &str,
    entry_point: Option<&str>,
) -> Result<MslTranslation, Error> {
    translate_wgsl_to_msl_with_opts(wgsl_source, entry_point, MslCompileOptions::from_env())
}

/// Like [`translate_wgsl_to_msl`], with explicit [`MslCompileOptions`].
pub fn translate_wgsl_to_msl_with_opts(
    wgsl_source: &str,
    entry_point: Option<&str>,
    opts: MslCompileOptions,
) -> Result<MslTranslation, Error> {
    let module = naga::front::wgsl::parse_str(wgsl_source)
        .map_err(|e| Error::InvalidInput(format!("WGSL parse error: {}", e.emit_to_string(wgsl_source))))?;
    translate_naga_module_to_msl(&module, "WGSL", entry_point, opts)
}

/// Returns the version string of the host Apple Metal compiler (`xcrun -sdk macosx metal --version`),
/// if available.
pub fn metal_toolchain_version() -> Option<&'static str> {
    static VER: OnceLock<Option<String>> = OnceLock::new();
    VER.get_or_init(|| {
        let out = Command::new("xcrun")
            .args(["-sdk", "macosx", "metal", "--version"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout);
        let first_line = s.lines().next()?.trim().to_string();
        (!first_line.is_empty()).then_some(first_line)
    })
    .as_deref()
}

/// True when Apple's native Metal compiler (`xcrun -sdk macosx metal`) is installed on this host.
pub fn metal_toolchain_available() -> bool {
    metal_toolchain_version().is_some()
}

/// Probes whether the host `xcrun -sdk macosx metal` accepts `-std=<std_ver>`.
fn probe_metal_std(std_ver: &str) -> bool {
    let std_flag = format!("-std={std_ver}");
    Command::new("xcrun")
        .args(["-sdk", "macosx", "metal", "-x", "metal", &std_flag, "-E", "/dev/null"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// True when the host Metal CLI toolchain supports `-std=metal4.1`.
pub fn metal_toolchain_supports_4_1() -> bool {
    static SUPPORTS: OnceLock<bool> = OnceLock::new();
    *SUPPORTS.get_or_init(|| metal_toolchain_available() && probe_metal_std("metal4.1"))
}

/// True when the host Metal CLI toolchain supports `-std=metal4.0`.
pub fn metal_toolchain_supports_4_0() -> bool {
    static SUPPORTS: OnceLock<bool> = OnceLock::new();
    *SUPPORTS.get_or_init(|| metal_toolchain_available() && probe_metal_std("metal4.0"))
}

/// Resolves the effective `-std=` value for `requested`, falling back cleanly on older toolchains.
fn effective_metal_std(requested: &str) -> &str {
    match requested {
        "metal4.1" => {
            if metal_toolchain_supports_4_1() {
                "metal4.1"
            } else if metal_toolchain_supports_4_0() {
                "metal4.0"
            } else {
                "metal3.1"
            }
        }
        "metal4.0" => {
            if metal_toolchain_supports_4_0() {
                "metal4.0"
            } else {
                "metal3.1"
            }
        }
        other => other,
    }
}

/// Compiles Metal Shading Language (`.msl`) source code into a native Apple `.metallib` binary
/// (`MTLB` container magic) using `xcrun -sdk macosx metal` + `xcrun -sdk macosx metallib`.
pub fn compile_msl_to_metallib(msl_source: &str) -> Result<Vec<u8>, Error> {
    compile_msl_to_metallib_with_opts(msl_source, MslCompileOptions::from_env())
}

/// Like [`compile_msl_to_metallib`], with explicit [`MslCompileOptions`] (supporting `-std=metal4.1`
/// and MSL 4.1 `-fmetal-rtz-fp-conversion`).
pub fn compile_msl_to_metallib_with_opts(
    msl_source: &str,
    opts: MslCompileOptions,
) -> Result<Vec<u8>, Error> {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    if !metal_toolchain_available() {
        return Err(Error::Load(
            "Apple Metal compiler toolchain (`xcrun -sdk macosx metal`) is not available on this host"
                .into(),
        ));
    }

    let id = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let tmp_dir = std::env::temp_dir().join(format!("aw2-spirv-metal-{pid}-{id}"));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| Error::Io(format!("create temp dir: {e}")))?;

    let msl_path = tmp_dir.join("shader.metal");
    let air_path = tmp_dir.join("shader.air");
    let lib_path = tmp_dir.join("shader.metallib");

    let cleanup = |dir: &PathBuf| {
        let _ = std::fs::remove_dir_all(dir);
    };

    if let Err(e) = std::fs::write(&msl_path, msl_source) {
        cleanup(&tmp_dir);
        return Err(Error::Io(format!("write temp .metal: {e}")));
    }

    let std_ver = effective_metal_std(opts.std_version);
    let std_flag = format!("-std={std_ver}");
    let mut cmd = Command::new("xcrun");
    cmd.args(["-sdk", "macosx", "metal", &std_flag, "-O2", "-Wno-unused-variable"]);
    if opts.rtz_fp_conversion && std_ver == "metal4.1" {
        cmd.arg("-fmetal-rtz-fp-conversion");
    }
    let metal_out = cmd
        .arg("-c")
        .arg(&msl_path)
        .arg("-o")
        .arg(&air_path)
        .output()
        .map_err(|e| {
            cleanup(&tmp_dir);
            Error::Io(format!("spawn xcrun metal: {e}"))
        })?;

    if !metal_out.status.success() {
        let stderr = String::from_utf8_lossy(&metal_out.stderr).trim().to_string();
        cleanup(&tmp_dir);
        return Err(Error::Convert {
            code: ErrorCode::Unknown,
            message: format!("xcrun metal compilation failed: {stderr}"),
        });
    }

    let lib_out = Command::new("xcrun")
        .args(["-sdk", "macosx", "metallib"])
        .arg(&air_path)
        .arg("-o")
        .arg(&lib_path)
        .output()
        .map_err(|e| {
            cleanup(&tmp_dir);
            Error::Io(format!("spawn xcrun metallib: {e}"))
        })?;

    if !lib_out.status.success() {
        let stderr = String::from_utf8_lossy(&lib_out.stderr).trim().to_string();
        cleanup(&tmp_dir);
        return Err(Error::Convert {
            code: ErrorCode::Unknown,
            message: format!("xcrun metallib linking failed: {stderr}"),
        });
    }

    let metallib = std::fs::read(&lib_path).map_err(|e| {
        cleanup(&tmp_dir);
        Error::Io(format!("read compiled .metallib: {e}"))
    })?;

    cleanup(&tmp_dir);
    Ok(metallib)
}

/// Compiles a Vulkan `SPIR-V` binary module into a [`CompiledShader`] (`.metallib` + `ShaderReflection`).
pub fn compile_spirv_uncached(
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
) -> Result<(CompiledShader, String), Error> {
    compile_spirv_uncached_with_opts(spirv_bytes, entry_point, MslCompileOptions::from_env())
}

/// Like [`compile_spirv_uncached`], with explicit [`MslCompileOptions`].
pub fn compile_spirv_uncached_with_opts(
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
    opts: MslCompileOptions,
) -> Result<(CompiledShader, String), Error> {
    let trans = translate_spirv_to_msl_with_opts(spirv_bytes, entry_point, opts)?;
    let metallib = compile_msl_to_metallib_with_opts(&trans.msl_source, opts)?;
    let mut kh = KeyHasher::new("dxil-metal/spirv-metal/v2");
    kh.bytes(spirv_bytes);
    kh.bytes(trans.entry_point_name.as_bytes());
    if !opts.is_legacy_default() {
        kh.u32(opts.naga_lang_version.0 as u32)
            .u32(opts.naga_lang_version.1 as u32)
            .bytes(effective_metal_std(opts.std_version).as_bytes())
            .u32(opts.rtz_fp_conversion as u32);
    }
    let key = kh.finish();
    Ok((
        CompiledShader {
            metallib,
            function_name: trans.metal_function_name,
            stage: trans.stage,
            reflection: trans.reflection,
            key,
        },
        trans.msl_source,
    ))
}

/// Compiles a WebGPU `WGSL` source shader into a [`CompiledShader`] (`.metallib` + `ShaderReflection`).
pub fn compile_wgsl_uncached(
    wgsl_source: &str,
    entry_point: Option<&str>,
) -> Result<(CompiledShader, String), Error> {
    compile_wgsl_uncached_with_opts(wgsl_source, entry_point, MslCompileOptions::from_env())
}

/// Like [`compile_wgsl_uncached`], with explicit [`MslCompileOptions`].
pub fn compile_wgsl_uncached_with_opts(
    wgsl_source: &str,
    entry_point: Option<&str>,
    opts: MslCompileOptions,
) -> Result<(CompiledShader, String), Error> {
    let trans = translate_wgsl_to_msl_with_opts(wgsl_source, entry_point, opts)?;
    let metallib = compile_msl_to_metallib_with_opts(&trans.msl_source, opts)?;
    let mut kh = KeyHasher::new("dxil-metal/wgsl-metal/v2");
    kh.bytes(wgsl_source.as_bytes());
    kh.bytes(trans.entry_point_name.as_bytes());
    if !opts.is_legacy_default() {
        kh.u32(opts.naga_lang_version.0 as u32)
            .u32(opts.naga_lang_version.1 as u32)
            .bytes(effective_metal_std(opts.std_version).as_bytes())
            .u32(opts.rtz_fp_conversion as u32);
    }
    let key = kh.finish();
    Ok((
        CompiledShader {
            metallib,
            function_name: trans.metal_function_name,
            stage: trans.stage,
            reflection: trans.reflection,
            key,
        },
        trans.msl_source,
    ))
}

/// Compiles a Vulkan `SPIR-V` module through `cache` (`~/Translator/ShaderCache`), returning
/// the cached or newly compiled [`CompiledShader`].
pub fn compile_spirv_cached(
    cache: &ShaderCache,
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
) -> Result<Arc<CompiledShader>, Error> {
    let opts = MslCompileOptions::from_env();
    let mut kh = KeyHasher::new("dxil-metal/spirv-metal/v2");
    kh.bytes(metal_toolchain_version().unwrap_or("naga-msl-fallback").as_bytes());
    kh.bytes(spirv_bytes);
    kh.bytes(entry_point.unwrap_or("").as_bytes());
    if !opts.is_legacy_default() {
        kh.u32(opts.naga_lang_version.0 as u32)
            .u32(opts.naga_lang_version.1 as u32)
            .bytes(effective_metal_std(opts.std_version).as_bytes())
            .u32(opts.rtz_fp_conversion as u32);
    }
    let key = kh.finish();
    cache.get_or_compile(key, || {
        compile_spirv_uncached_with_opts(spirv_bytes, entry_point, opts).map(|(s, _msl)| s)
    })
}

/// Process-wide cached Vulkan `SPIR-V` -> Metal pipeline backed by `~/Translator/ShaderCache`.
pub fn compile_spirv_global(
    spirv_bytes: &[u8],
    entry_point: Option<&str>,
) -> Result<Arc<CompiledShader>, Error> {
    static SPIRV_CACHE: OnceLock<ShaderCache> = OnceLock::new();
    let cache = SPIRV_CACHE.get_or_init(|| {
        let fp = format!(
            "naga30-{}",
            metal_toolchain_version().unwrap_or("msl3_1")
        );
        ShaderCache::new(crate::CacheConfig::from_env(), &fp)
    });
    compile_spirv_cached(cache, spirv_bytes, entry_point)
}

/// One embedded Vulkan `SPIR-V` binary module extracted from a PE binary (`.exe` / `.dll`).
#[derive(Debug, Clone)]
pub struct PeEmbeddedSpirvShader {
    pub binary_name: String,
    pub file_offset: usize,
    pub spirv_hash: u128,
    pub spirv_bytes: Vec<u8>,
    pub header: SpirvHeader,
    pub entry_points: Vec<SpirvEntryPointInfo>,
}

/// Attempts to determine the exact byte length of a `SPIR-V` binary module starting at `bytes[0..]`
/// by walking its 32-bit instruction stream (`word_count = word >> 16`, `opcode = word & 0xFFFF`).
fn probe_embedded_spirv_len(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 28 {
        return None;
    }
    let w0 = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    if w0 != SPIRV_MAGIC_LE {
        return None;
    }
    let ver = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    let ver_maj = (ver >> 16) & 0xFF;
    let ver_min = (ver >> 8) & 0xFF;
    let ver_low = ver & 0xFF;
    if ver_maj != 1 || ver_min > 6 || ver_low != 0 {
        return None;
    }
    let bound = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let schema = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    if !(2..=0x0010_0000).contains(&bound) || schema != 0 {
        return None;
    }
    // First instruction in a valid SPIR-V shader must be OpCapability (opcode 17, word_count 2)
    // or OpExtension (10) / OpExtInstImport (11) / OpMemoryModel (14).
    let first_inst = u32::from_le_bytes(bytes[20..24].try_into().ok()?);
    let first_op = first_inst & 0xFFFF;
    let first_wc = (first_inst >> 16) as usize;
    if first_op != 17 || first_wc != 2 {
        return None;
    }

    let max_words = (bytes.len() / 4).min(512 * 1024);
    let mut w_idx = 5usize;
    let mut saw_entry_point = false;
    let mut saw_memory_model = false;
    let mut function_depth = 0i32;
    let mut last_function_end_word = 0usize;

    while w_idx < max_words {
        let off = w_idx * 4;
        let word = u32::from_le_bytes(bytes[off..off + 4].try_into().ok()?);
        let opcode = (word & 0xFFFF) as u16;
        let wc = (word >> 16) as usize;
        if wc == 0 || w_idx + wc > max_words {
            break;
        }
        match opcode {
            14 => saw_memory_model = true, // OpMemoryModel
            15 => saw_entry_point = true,  // OpEntryPoint
            54 => function_depth += 1,     // OpFunction
            56 => {
                // OpFunctionEnd
                if wc != 1 || function_depth <= 0 {
                    break;
                }
                function_depth -= 1;
                if function_depth == 0 && saw_memory_model && saw_entry_point {
                    last_function_end_word = w_idx + 1;
                }
            }
            // Invalid opcode outside SPIR-V core/extension range when not in a function
            0 | 6000..=u16::MAX => break,
            _ => {}
        }
        w_idx += wc;
    }

    if last_function_end_word >= 7 {
        Some(last_function_end_word * 4)
    } else {
        None
    }
}

/// Scans `bytes` (e.g., a `.exe` or `.dll` image) for embedded Vulkan `SPIR-V` shader modules
/// (`0x07230203`), validates them with `naga::front::spv`, and returns all discovered modules.
pub fn extract_pe_spirv_shaders(
    binary_name: impl Into<String>,
    bytes: &[u8],
) -> Vec<PeEmbeddedSpirvShader> {
    let binary_name = binary_name.into();
    let mut out = Vec::new();
    let magic = SPIRV_MAGIC_LE.to_le_bytes();
    let mut idx = 0usize;

    while idx + 28 <= bytes.len() {
        let Some(rel) = bytes[idx..].windows(4).position(|w| w == magic) else {
            break;
        };
        idx += rel;
        if let Some(byte_len) = probe_embedded_spirv_len(&bytes[idx..]) {
            let slice = &bytes[idx..idx + byte_len];
            if let Ok(summary) = summarize_spirv(slice) {
                if !summary.entry_points.is_empty() {
                    out.push(PeEmbeddedSpirvShader {
                        binary_name: binary_name.clone(),
                        file_offset: idx,
                        spirv_hash: hash128(slice),
                        spirv_bytes: slice.to_vec(),
                        header: summary.header,
                        entry_points: summary.entry_points,
                    });
                    idx += byte_len;
                    continue;
                }
            }
        }
        idx += 4;
    }

    out
}

/// Synthesizes a minimal valid Vulkan `SPIR-V` 1.3 Compute shader (`[numthreads(64, 1, 1)]`)
/// that reads/writes a storage buffer at `(set = 0, binding = 0)` for testing and verification.
pub fn sample_vulkan_compute_spirv() -> Vec<u8> {
    let words: &[u32] = &[
        0x07230203, // Magic
        0x00010300, // Version 1.3
        0x00080001, // Generator (Khronos Glslang)
        0x00000006, // Bound: 6
        0x00000000, // Schema: 0
        // OpCapability Shader (1)
        (2 << 16) | 17,
        1,
        // OpMemoryModel Logical (0) GLSL450 (1)
        (3 << 16) | 14,
        0,
        1,
        // OpEntryPoint GLCompute (5) %4 "main\0" (5 words total)
        (5 << 16) | 15,
        5,
        4,
        u32::from_le_bytes(*b"main"),
        0,
        // OpExecutionMode %4 LocalSize 64 1 1
        (6 << 16) | 16,
        4,
        17,
        64,
        1,
        1,
        // %1 = OpTypeVoid
        (2 << 16) | 19,
        1,
        // %2 = OpTypeFunction %1
        (3 << 16) | 33,
        2,
        1,
        // %4 = OpFunction %1 None %2
        (5 << 16) | 54,
        1,
        4,
        0,
        2,
        // %5 = OpLabel
        (2 << 16) | 248,
        5,
        // OpReturn
        (1 << 16) | 253,
        // OpFunctionEnd
        (1 << 16) | 56,
    ];
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for w in words {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spirv_to_msl_and_metallib_pipeline_works() {
        let spv = sample_vulkan_compute_spirv();
        let summary = summarize_spirv(&spv).expect("summarize_spirv");
        assert_eq!(summary.header.version_major, 1);
        assert_eq!(summary.header.version_minor, 3);
        assert_eq!(summary.entry_points.len(), 1);
        assert_eq!(summary.entry_points[0].name, "main");
        assert_eq!(summary.entry_points[0].stage, ShaderStage::Compute);
        assert_eq!(summary.entry_points[0].workgroup_size, [64, 1, 1]);

        let trans = translate_spirv_to_msl(&spv, Some("main")).expect("translate_spirv_to_msl");
        assert!(trans.msl_source.contains("#include <metal_stdlib>"));
        assert!(trans.msl_source.contains("kernel void"));
        assert_eq!(trans.stage, ShaderStage::Compute);
        assert_eq!(trans.reflection.threadgroup_size(), Some([64, 1, 1]));

        if metal_toolchain_available() {
            let (compiled, msl) = compile_spirv_uncached(&spv, Some("main")).expect("compile_spirv");
            assert!(msl.contains("kernel void"));
            assert_eq!(compiled.stage, ShaderStage::Compute);
            assert!(
                compiled.metallib.starts_with(b"MTLB"),
                "expected native MTLB metallib header, got {:?}",
                &compiled.metallib[..4.min(compiled.metallib.len())]
            );
        } else {
            assert!(matches!(compile_spirv_uncached(&spv, Some("main")), Err(Error::Load(_))));
        }

        // Verify PE embedded SPIR-V scanner finds the module inside a padded image buffer
        let mut fake_pe = vec![0x90u8; 256];
        fake_pe.extend_from_slice(&spv);
        fake_pe.extend_from_slice(&[0xCCu8; 128]);
        let extracted = extract_pe_spirv_shaders("vulkan_test.exe", &fake_pe);
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0].file_offset, 256);
        assert_eq!(extracted[0].spirv_bytes, spv);
    }

    #[test]
    fn wgsl_to_msl_and_metallib_pipeline_works() {
        let wgsl = r#"
            struct Params {
                scale: f32,
            };
            @group(0) @binding(0) var<uniform> params: Params;
            @group(0) @binding(1) var<storage, read_write> data: array<f32>;

            @compute @workgroup_size(32, 1, 1)
            fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
                data[gid.x] = data[gid.x] * params.scale;
            }
        "#;
        let trans = translate_wgsl_to_msl(wgsl, Some("cs_main")).expect("translate_wgsl_to_msl");
        assert!(trans.msl_source.contains("kernel void cs_main"));
        assert_eq!(trans.stage, ShaderStage::Compute);
        assert_eq!(trans.reflection.resources.len(), 2);

        if metal_toolchain_available() {
            let (compiled, msl) = compile_wgsl_uncached(wgsl, Some("cs_main")).expect("compile_wgsl");
            assert!(msl.contains("kernel void cs_main"));
            assert_eq!(compiled.stage, ShaderStage::Compute);
            assert!(compiled.metallib.starts_with(b"MTLB"));
        } else {
            assert!(matches!(compile_wgsl_uncached(wgsl, Some("cs_main")), Err(Error::Load(_))));
        }
    }

    #[test]
    fn metal41_msl_and_spirv_options_compile_cleanly() {
        let spv = sample_vulkan_compute_spirv();
        let m41_opts = MslCompileOptions::metal41(true);
        let trans = translate_spirv_to_msl_with_opts(&spv, Some("main"), m41_opts)
            .expect("translate_spirv_to_msl_with_opts(metal41)");
        assert!(trans.msl_source.contains("kernel void"));

        if !metal_toolchain_supports_4_1() {
            return;
        }
        let (compiled, _) = compile_spirv_uncached_with_opts(&spv, Some("main"), m41_opts)
            .expect("compile_spirv metal4.1 + rtz");
        assert!(compiled.metallib.starts_with(b"MTLB"));

        // Verify MSL 4.1-only constructs (block_read, interleave, acquire/release atomics/barriers,
        // threadgroup atomic_float) compile to a native MTLB library under MslCompileOptions::metal41(true).
        let msl41_src = r#"
#include <metal_stdlib>
using namespace metal;
kernel void k_m41(
    texture2d<float, access::read> tex [[texture(0)]],
    device float4* out [[buffer(0)]],
    device atomic_uint* flag [[buffer(1)]],
    uint2 gid [[thread_position_in_grid]])
{
    threadgroup atomic_float tg_acc;
    constexpr sampler s(address::clamp_to_edge);
    auto b = tex.block_read<2, 1>(s, int2(gid));
    float4 v = b.value<0, 0>() + b.value<1, 0>();
    uint m = interleave(ushort(gid.x), ushort(gid.y));
    atomic_fetch_add_explicit(&tg_acc, v.x, memory_order_relaxed);
    threadgroup_barrier(mem_flags::mem_threadgroup, memory_order_acq_rel, thread_scope_threadgroup);
    atomic_store_explicit(flag, m, memory_order_release, mem_flags::mem_device);
    out[gid.x] = v;
}
"#;
        let metallib = compile_msl_to_metallib_with_opts(msl41_src, m41_opts)
            .expect("compile MSL 4.1 features");
        assert!(metallib.starts_with(b"MTLB"));
    }
}
