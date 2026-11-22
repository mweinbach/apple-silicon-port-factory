//! Standardized Shader File Format Taxonomy, Auto-Detector, Inspector, and Unified Raw-File
//! Conversion Pipeline (`DirectX 12 DXIL`, `DirectX 11 DXBC`, `Vulkan SPIR-V`, `WGSL`, `MSL`,
//! `.binrfx` -> Apple Metal `.metallib`).

use std::fmt::Write as _;
use std::path::Path;

use crate::bitcode;
use crate::cache::{CacheConfig, ShaderPipeline};
use crate::compiler::{CompileOptions, CompileRequest, CompiledShader};
use crate::dxbc::{
    build_container, Container, ProgramInfo, ProgramKind, FOURCC_DXIL, FOURCC_HASH, FOURCC_PSV0,
    FOURCC_RDAT, FOURCC_RTS0, FOURCC_SFI0, FOURCC_SHDR, FOURCC_SHEX,
};
use crate::pe::extract_pe_shaders;
use crate::reflection::{ShaderReflection, ShaderStage, StageInfo};
use crate::root_signature::{ErrorCode, RootSignatureDesc};
use crate::spirv_metal::{
    compile_msl_to_metallib, compile_spirv_cached, compile_spirv_uncached, compile_wgsl_uncached,
    extract_pe_spirv_shaders, summarize_spirv, SpirvHeader, SPIRV_MAGIC_BE, SPIRV_MAGIC_LE,
};
use crate::Error;

/// Canonical identifier for every shader file/container format recognized by the translator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderFormatId {
    /// DirectX 12 `DXBC` container carrying `DXIL` SM 6.0–6.8 LLVM 3.7 bitcode (`FOURCC_DXIL`).
    DxbcDxilSm6,
    /// DirectX 12 bare `DXIL` program header or LLVM 3.7 bitcode (`BC\xc0\xde`) without outer `DXBC`.
    DxilRawBitcode,
    /// DirectX 10/11 `DXBC` container carrying FXC SM 4.0–5.1 token stream (`FOURCC_SHEX` / `FOURCC_SHDR`).
    DxbcFxcSm4_5,
    /// DirectX 12 `DXBC` container carrying only a serialized Root Signature (`FOURCC_RTS0`).
    DxbcRootSignature,
    /// Vulkan / OpenGL / OpenCL `SPIR-V` binary module (`0x07230203` LE or `0x03022307` BE).
    VulkanSpirvBinary,
    /// Remedy Northlight `.binrfx` (v6/v7) shader bundle containing stage slots + `DXBC` permutations.
    NorthlightBinrfx,
    /// WebGPU Shading Language source (`.wgsl`).
    WgslSource,
    /// DirectX High-Level Shading Language source (`.hlsl` / `.fx`).
    HlslSource,
    /// Vulkan / OpenGL GLSL source (`.glsl` / `.vert` / `.frag` / `.comp`).
    GlslSource,
    /// Apple Metal Shading Language source (`.metal` / `.msl`).
    MetalMslSource,
    /// Apple Metal LLVM AIR compiled bitcode object (`.air`).
    MetalAirBitcode,
    /// Apple Compiled Metal Library container (`MTLB` / `.metallib`).
    MetalMetallib,
    /// Translator Pre-Compiled Metal Shader Cache Entry (`AW2SHC02` / `.aw2s`).
    TranslatorAw2sCache,
    /// Windows PE executable or DLL (`MZ`) containing embedded `DXBC`/`DXIL` or `SPIR-V` blobs.
    PeEmbeddedContainer,
    /// Unrecognized binary/text blob.
    Unknown,
}

impl ShaderFormatId {
    pub fn code(self) -> &'static str {
        match self {
            Self::DxbcDxilSm6 => "DXBC_DXIL_SM6",
            Self::DxilRawBitcode => "DXIL_RAW_BITCODE",
            Self::DxbcFxcSm4_5 => "DXBC_FXC_SM4_5",
            Self::DxbcRootSignature => "DXBC_ROOT_SIGNATURE",
            Self::VulkanSpirvBinary => "VULKAN_SPIRV_BINARY",
            Self::NorthlightBinrfx => "NORTHLIGHT_BINRFX",
            Self::WgslSource => "WGSL_SOURCE",
            Self::HlslSource => "HLSL_SOURCE",
            Self::GlslSource => "GLSL_SOURCE",
            Self::MetalMslSource => "METAL_MSL_SOURCE",
            Self::MetalAirBitcode => "METAL_AIR_BITCODE",
            Self::MetalMetallib => "METAL_METALLIB",
            Self::TranslatorAw2sCache => "TRANSLATOR_AW2S_CACHE",
            Self::PeEmbeddedContainer => "PE_EMBEDDED_CONTAINER",
            Self::Unknown => "UNKNOWN",
        }
    }

    pub fn spec(self) -> &'static ShaderFormatSpec {
        SHADER_FORMAT_CATALOG
            .iter()
            .find(|s| s.id == self)
            .unwrap_or(&SHADER_FORMAT_CATALOG[0])
    }
}

/// Static specification of a standardized shader file format.
#[derive(Debug, Clone, Copy)]
pub struct ShaderFormatSpec {
    pub id: ShaderFormatId,
    pub code: &'static str,
    pub family: &'static str,
    pub extensions: &'static str,
    pub magic_signature: &'static str,
    pub header_layout: &'static str,
    pub inner_chunks_or_sections: &'static str,
    pub native_metal_pipeline: &'static str,
    pub status: &'static str,
}

/// Canonical table of all 14 standardized shader file formats across DirectX 12, DirectX 11,
/// Vulkan, WebGPU, Engine Bundles, and Apple Metal.
pub const SHADER_FORMAT_CATALOG: &[ShaderFormatSpec] = &[
    ShaderFormatSpec {
        id: ShaderFormatId::DxbcDxilSm6,
        code: "DXBC_DXIL_SM6",
        family: "DirectX 12 (SM 6.0-6.8)",
        extensions: ".dxil,.cso,.bin,.shader",
        magic_signature: "44 58 42 43 ('DXBC' @ +0) + 'DXIL' (44 58 49 4C) part",
        header_layout: "32B DXBC Header: magic[4]='DXBC', md5_digest[16], ver=1.0(u16x2), total_size(u32), part_count(u32), part_offsets[part_count]",
        inner_chunks_or_sections: "DXIL (24B ProgramHeader + 'BC\\xc0\\xde' LLVM 3.7 bitcode), PSV0 (Pipeline State Validation + resources + wave size), RTS0 (Root Signature), RDAT (DXR Library Runtime Data), SFI0 (Feature Flags), HASH",
        native_metal_pipeline: "DXBC(DXIL) -> RootSignature(RTS0/PSV0/RDAT) -> libmetalirconverter.dylib -> Metal IR (.metallib) -> ~/Translator/ShaderCache/*.aw2s",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::DxilRawBitcode,
        code: "DXIL_RAW_BITCODE",
        family: "DirectX 12 Bare DXIL",
        extensions: ".dxil,.bc",
        magic_signature: "42 43 C0 DE ('BC\\xc0\\xde' @ +0) or 'DXIL' @ +8",
        header_layout: "24B DxilProgramHeader (ProgramVersion u32, SizeInUint32 u32, 'DXIL'[4], DxilVersion u32, BitcodeOffset u32, BitcodeSize u32) or raw LLVM 3.7 bitcode stream",
        inner_chunks_or_sections: "MODULE_BLOCK (8), PARAMATTR_BLOCK (9), TYPE_BLOCK (17), FUNCTION_BLOCK (12), VALUE_SYMTAB (14), METADATA_BLOCK (15: dx.entryPoints, dx.resources, dx.shaderModel)",
        native_metal_pipeline: "Wrap in signed DXBC(DXIL) container -> libmetalirconverter.dylib -> Metal IR (.metallib) -> ~/Translator/ShaderCache/*.aw2s",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::DxbcFxcSm4_5,
        code: "DXBC_FXC_SM4_5",
        family: "DirectX 10/11 (SM 4.0-5.1)",
        extensions: ".dxbc,.cso,.fxo,.vso,.pso,.csh",
        magic_signature: "44 58 42 43 ('DXBC' @ +0) + 'SHEX' (53 48 45 58) or 'SHDR' (53 48 44 52) part",
        header_layout: "32B DXBC Header + SHEX/SHDR token stream (VersionToken u32 = kind<<16 | major<<4 | minor, LengthInDwords u32, D3D10_SB_OPCODE_* tokens)",
        inner_chunks_or_sections: "SHEX/SHDR (SM4/5 Opcode Stream), ISGN (Input Signature), OSGN (Output Signature), PCSG (Patch Constant Signature), RDEF (Resource Definitions), STAT",
        native_metal_pipeline: "DXBC(SHEX/SHDR) -> libdxilconv.dylib (IDxbcConverter) -> Signed DXBC(DXIL SM 6.0) -> libmetalirconverter.dylib -> Metal IR (.metallib)",
        status: "native_bridged",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::DxbcRootSignature,
        code: "DXBC_ROOT_SIGNATURE",
        family: "DirectX 12 Root Signature",
        extensions: ".rs,.rts0,.cso",
        magic_signature: "44 58 42 43 ('DXBC' @ +0) + 'RTS0' (52 54 53 30) part only",
        header_layout: "32B DXBC Header + RTS0 part (Version u32=1|2, NumParameters u32, RootParametersOffset u32, NumStaticSamplers u32, StaticSamplersOffset u32, Flags u32)",
        inner_chunks_or_sections: "RTS0 (DescriptorTable / RootConstants / RootCBV / RootSRV / RootUAV + StaticSamplers)",
        native_metal_pipeline: "RootSignatureDesc::from_blob -> TopLevelArgumentBuffer layout + IRRootSignatureCreateFromDescriptor",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::VulkanSpirvBinary,
        code: "VULKAN_SPIRV_BINARY",
        family: "Vulkan / OpenGL / OpenCL (SPIR-V 1.0-1.6)",
        extensions: ".spv,.spirv",
        magic_signature: "03 02 23 07 (0x07230203 LE) or 07 23 02 03 (0x03022307 BE) @ +0",
        header_layout: "20B (5-word) Header: magic=0x07230203, version=0x00MMmm00 (1.0..1.6), generator_magic(tool_id<<16|ver), bound(max_id+1), schema=0",
        inner_chunks_or_sections: "Word-stream instructions (wc=w>>16, op=w&0xFFFF): OpCapability(17), OpExtInstImport(11), OpMemoryModel(14), OpEntryPoint(15), OpExecutionMode(16), OpDecorate(71 DescriptorSet/Binding), OpFunction(54)..OpFunctionEnd(56)",
        native_metal_pipeline: "SPIR-V -> naga (spv-in -> Validator -> msl-out Metal 3.1) -> xcrun -sdk macosx metal (.air) -> xcrun metallib (.metallib) -> ~/Translator/ShaderCache/*.aw2s",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::NorthlightBinrfx,
        code: "NORTHLIGHT_BINRFX",
        family: "Remedy Northlight Engine Bundle",
        extensions: ".binrfx",
        magic_signature: "06 00 00 00 or 07 00 00 00 (version 6/7 u32 LE) + 6 stage slots",
        header_layout: "Northlight RFX v6/v7 Header: version(u32), 6 stage presence flags (VS,PS,GS,HS,DS,CS), constant buffers, SRVs, UAVs, samplers, permutation table",
        inner_chunks_or_sections: "Per-permutation compressed/raw DXBC(DXIL) shader containers + engine reflection tables",
        native_metal_pipeline: "aw2_data::rfx::RfxFile::parse -> extract DXBC(DXIL) permutations + covering RootSignature -> ShaderPipeline -> ~/Translator/ShaderCache/*.aw2s",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::WgslSource,
        code: "WGSL_SOURCE",
        family: "WebGPU Shading Language",
        extensions: ".wgsl",
        magic_signature: "UTF-8 text ('@vertex', '@fragment', '@compute', '@workgroup_size', '@group(')",
        header_layout: "UTF-8 WGSL source module",
        inner_chunks_or_sections: "struct declarations, @group(G) @binding(B) var<uniform|storage>, @vertex/@fragment/@compute fn entry points",
        native_metal_pipeline: "WGSL -> naga (wgsl-in -> Validator -> msl-out Metal 3.1) -> xcrun metal (.air) -> xcrun metallib (.metallib)",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::HlslSource,
        code: "HLSL_SOURCE",
        family: "DirectX HLSL Source",
        extensions: ".hlsl,.fx,.hlsli,.ush,.usf",
        magic_signature: "UTF-8 text ('cbuffer', 'StructuredBuffer', 'SV_Position', 'SV_Target', '[numthreads(')",
        header_layout: "UTF-8 HLSL source file",
        inner_chunks_or_sections: "Constant buffers (b#), textures/buffers (t#), UAVs (u#), samplers (s#), shader stage entry functions",
        native_metal_pipeline: "D3DCompile / dxc (-T <profile>) -> DXBC/DXIL -> libmetalirconverter.dylib -> Metal (.metallib)",
        status: "native_bridged",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::GlslSource,
        code: "GLSL_SOURCE",
        family: "Vulkan / OpenGL GLSL Source",
        extensions: ".glsl,.vert,.frag,.comp,.geom,.tesc,.tese",
        magic_signature: "UTF-8 text ('#version 450', 'layout(set =', 'layout(binding =', 'gl_Position')",
        header_layout: "UTF-8 GLSL source file",
        inner_chunks_or_sections: "#version directive, layout(set, binding) uniforms/buffers, void main()",
        native_metal_pipeline: "glslang/shaderc -> SPIR-V (.spv) -> naga (msl-out) -> xcrun metal -> .metallib",
        status: "supported_via_spirv",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::MetalMslSource,
        code: "METAL_MSL_SOURCE",
        family: "Apple Metal Shading Language Source",
        extensions: ".metal,.msl",
        magic_signature: "UTF-8 text ('#include <metal_stdlib>', 'using namespace metal;', 'kernel void', 'vertex ', 'fragment ')",
        header_layout: "UTF-8 Metal C++14/C++17 shading language source",
        inner_chunks_or_sections: "stage_in structs, [[buffer(N)]] / [[texture(N)]] / [[sampler(N)]] bindings, vertex/fragment/kernel functions",
        native_metal_pipeline: "xcrun -sdk macosx metal -std=metal3.1 -O2 -c -> .air -> xcrun metallib -> .metallib",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::MetalAirBitcode,
        code: "METAL_AIR_BITCODE",
        family: "Apple Metal LLVM AIR Bitcode",
        extensions: ".air",
        magic_signature: "DE C0 17 0B (LLVM bitcode wrapper) or 42 43 C0 DE ('BC\\xc0\\xde')",
        header_layout: "20B LLVM Bitcode Wrapper (Magic=0x0B17C0DE, Version=0, Offset, Size, CPUType) + AIR bitcode stream",
        inner_chunks_or_sections: "air.vertex / air.fragment / air.kernel metadata + Apple GPU LLVM IR",
        native_metal_pipeline: "xcrun -sdk macosx metallib <in.air> -o <out.metallib>",
        status: "native_direct",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::MetalMetallib,
        code: "METAL_METALLIB",
        family: "Apple Compiled Metal Library",
        extensions: ".metallib",
        magic_signature: "4D 54 4C 42 ('MTLB' @ +0)",
        header_layout: "MTLB container header: magic[4]='MTLB', platform/version tags, file size, function list offset, bitcode payload offset",
        inner_chunks_or_sections: "NAME (entry function names), MDSZ, TYPE, HASH, Embedded AIR bitcode / GPU machine code",
        native_metal_pipeline: "Ready for direct execution via MTLDevice::newLibraryWithData:",
        status: "native_target",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::TranslatorAw2sCache,
        code: "TRANSLATOR_AW2S_CACHE",
        family: "Translator Pre-Compiled Metal Cache Entry",
        extensions: ".aw2s",
        magic_signature: "41 57 32 53 48 43 30 32 ('AW2SHC02' @ +0)",
        header_layout: "8B Magic 'AW2SHC02' + 16B Key(u128 LE) + Wire ShaderReflection + MTLB .metallib bytes + 16B XXH128 checksum",
        inner_chunks_or_sections: "ShaderStage, Metal function_name, ResourceLocations[], FunctionConstants[], StageInfo, Reflection JSON, MTLB .metallib",
        native_metal_pipeline: "Zero-copy pure-Rust decode in ShaderCache -> MTLDevice::newLibraryWithData:",
        status: "native_target",
    },
    ShaderFormatSpec {
        id: ShaderFormatId::PeEmbeddedContainer,
        code: "PE_EMBEDDED_CONTAINER",
        family: "Windows PE Executable / DLL with Embedded Shaders",
        extensions: ".exe,.dll",
        magic_signature: "4D 5A ('MZ' @ +0) + 'PE\\0\\0' + embedded 'DXBC' (44 58 42 43) or 'SPIR-V' (03 02 23 07) blobs",
        header_layout: "DOS Header + PE32+ OptionalHeader + Section Table (.text, .rdata, .data) containing embedded shader arrays",
        inner_chunks_or_sections: "Embedded DXBC(DXIL SM6), DXBC(SHEX/SHDR SM4/5), and/or Vulkan SPIR-V (0x07230203) binary modules",
        native_metal_pipeline: "extract_pe_shaders + extract_pe_spirv_shaders -> DX12/DX11 + Vulkan native pipelines -> ~/Translator/ShaderCache/*.aw2s",
        status: "native_direct",
    },
];

/// Auto-detects the [`ShaderFormatId`] of a raw file or byte buffer from its magic bytes,
/// container chunks, and optional file path extension.
pub fn detect_shader_format(path: Option<&Path>, bytes: &[u8]) -> ShaderFormatId {
    let ext = path
        .and_then(|p| p.extension())
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    // 1. Translator cache entry (`AW2SHC02`)
    if bytes.len() >= 8 && &bytes[0..8] == b"AW2SHC02" {
        return ShaderFormatId::TranslatorAw2sCache;
    }

    // 2. Compiled Apple Metal Library (`MTLB`)
    if bytes.len() >= 4 && &bytes[0..4] == b"MTLB" {
        return ShaderFormatId::MetalMetallib;
    }

    // 3. DirectX `DXBC` Container (`DXIL` SM6, `SHEX`/`SHDR` SM4/5, or `RTS0` Root Signature)
    if Container::is_container(bytes) {
        if let Ok(c) = Container::parse(bytes) {
            if c.part(FOURCC_DXIL).is_some() {
                return ShaderFormatId::DxbcDxilSm6;
            }
            if c.part(FOURCC_SHEX).is_some() || c.part(FOURCC_SHDR).is_some() {
                return ShaderFormatId::DxbcFxcSm4_5;
            }
            if c.part(FOURCC_RTS0).is_some() {
                return ShaderFormatId::DxbcRootSignature;
            }
        }
        return ShaderFormatId::DxbcDxilSm6;
    }

    // 4. Vulkan `SPIR-V` Binary (`0x07230203` LE or `0x03022307` BE)
    if bytes.len() >= 20 {
        let w0 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if (w0 == SPIRV_MAGIC_LE || w0 == SPIRV_MAGIC_BE) && SpirvHeader::parse(bytes).is_some() {
            return ShaderFormatId::VulkanSpirvBinary;
        }
    }

    // 5. Bare DXIL Program Header (`DXIL` at +8) or LLVM Bitcode (`BC\xc0\xde` / `0x0B17C0DE`)
    if bytes.len() >= 24 && &bytes[8..12] == b"DXIL" && ProgramInfo::parse(bytes).is_some() {
        return ShaderFormatId::DxilRawBitcode;
    }
    if bytes.len() >= 4 && (&bytes[0..4] == b"BC\xc0\xde" || &bytes[0..4] == b"\xde\xc0\x17\x0b") {
        if ext == "air" {
            return ShaderFormatId::MetalAirBitcode;
        }
        return ShaderFormatId::DxilRawBitcode;
    }

    // 6. Windows PE Executable / DLL (`MZ`)
    if bytes.len() >= 64 && &bytes[0..2] == b"MZ" {
        return ShaderFormatId::PeEmbeddedContainer;
    }

    // 7. Remedy Northlight `.binrfx` bundle
    if ext == "binrfx" {
        return ShaderFormatId::NorthlightBinrfx;
    }
    if bytes.len() >= 32 {
        let v = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if (v == 6 || v == 7) && bytes.windows(4).any(|w| w == b"DXBC") {
            return ShaderFormatId::NorthlightBinrfx;
        }
    }

    // 8. Text / Source shader formats by extension or content heuristics
    match ext.as_str() {
        "wgsl" => return ShaderFormatId::WgslSource,
        "metal" | "msl" => return ShaderFormatId::MetalMslSource,
        "hlsl" | "fx" | "hlsli" | "ush" | "usf" => return ShaderFormatId::HlslSource,
        "glsl" | "vert" | "frag" | "comp" | "geom" | "tesc" | "tese" => {
            return ShaderFormatId::GlslSource
        }
        "spv" | "spirv" => return ShaderFormatId::VulkanSpirvBinary,
        "dxil" => return ShaderFormatId::DxbcDxilSm6,
        "dxbc" | "fxo" => return ShaderFormatId::DxbcFxcSm4_5,
        _ => {}
    }

    if let Ok(text) = std::str::from_utf8(&bytes[..bytes.len().min(4096)]) {
        if text.contains("#include <metal_stdlib>") || text.contains("using namespace metal") {
            return ShaderFormatId::MetalMslSource;
        }
        if text.contains("@compute")
            || text.contains("@vertex")
            || text.contains("@fragment")
            || text.contains("@workgroup_size")
        {
            return ShaderFormatId::WgslSource;
        }
        if text.contains("#version ") && (text.contains("layout(") || text.contains("gl_Position"))
        {
            return ShaderFormatId::GlslSource;
        }
        if text.contains("SV_Position")
            || text.contains("SV_Target")
            || text.contains("[numthreads(")
            || text.contains("StructuredBuffer<")
        {
            return ShaderFormatId::HlslSource;
        }
    }

    ShaderFormatId::Unknown
}

/// Detailed inspection of any raw shader file or PE container.
#[derive(Debug, Clone)]
pub struct ShaderInspection {
    pub name: String,
    pub byte_len: usize,
    pub format: ShaderFormatId,
    pub shader_model_or_version: String,
    pub container_parts: Vec<String>,
    pub entry_points: Vec<(String, String)>, // (entry_point, stage_or_profile)
    pub resource_count: usize,
    pub has_root_signature_or_push_constants: bool,
    pub wave_or_workgroup_info: String,
    pub embedded_dxbc_count: usize,
    pub embedded_spirv_count: usize,
}

impl ShaderInspection {
    pub fn format_summary(&self) -> String {
        let spec = self.format.spec();
        let mut out = String::new();
        let _ = writeln!(out, "  File / Blob            : {} ({} bytes)", self.name, self.byte_len);
        let _ = writeln!(
            out,
            "  Detected Format        : {} ({})",
            spec.code, spec.family
        );
        let _ = writeln!(out, "  Magic / Signature      : {}", spec.magic_signature);
        let _ = writeln!(
            out,
            "  Shader Model / Version : {}",
            self.shader_model_or_version
        );
        if !self.container_parts.is_empty() {
            let _ = writeln!(
                out,
                "  Container Parts        : {}",
                self.container_parts.join(", ")
            );
        }
        if !self.entry_points.is_empty() {
            let eps: Vec<String> = self
                .entry_points
                .iter()
                .map(|(n, p)| format!("{n} ({p})"))
                .collect();
            let _ = writeln!(out, "  Entry Points           : {}", eps.join(", "));
        }
        let _ = writeln!(out, "  Resource Bindings      : {}", self.resource_count);
        let _ = writeln!(
            out,
            "  RootSig / PushConst    : {}",
            self.has_root_signature_or_push_constants
        );
        if !self.wave_or_workgroup_info.is_empty() {
            let _ = writeln!(
                out,
                "  Wave / Workgroup Info  : {}",
                self.wave_or_workgroup_info
            );
        }
        if self.embedded_dxbc_count > 0 || self.embedded_spirv_count > 0 {
            let _ = writeln!(
                out,
                "  Embedded PE Shaders    : {} DirectX (DXBC/DXIL), {} Vulkan (SPIR-V)",
                self.embedded_dxbc_count, self.embedded_spirv_count
            );
        }
        let _ = writeln!(
            out,
            "  Native Metal Pipeline  : {}",
            spec.native_metal_pipeline
        );
        out
    }
}

/// Inspects `bytes` and returns a structured [`ShaderInspection`] describing its format,
/// header fields, chunks, entry points, and resource bindings.
pub fn inspect_shader_bytes(
    name: &str,
    path: Option<&Path>,
    bytes: &[u8],
) -> Result<ShaderInspection, Error> {
    let format = detect_shader_format(path, bytes);
    let mut shader_model_or_version = "n/a".to_string();
    let mut container_parts = Vec::new();
    let mut entry_points = Vec::new();
    let mut resource_count = 0usize;
    let mut has_root_signature_or_push_constants = false;
    let mut wave_or_workgroup_info = String::new();
    let mut embedded_dxbc_count = 0usize;
    let mut embedded_spirv_count = 0usize;

    match format {
        ShaderFormatId::DxbcDxilSm6
        | ShaderFormatId::DxbcFxcSm4_5
        | ShaderFormatId::DxbcRootSignature => {
            let c = Container::parse(bytes)?;
            for p in &c.parts {
                let fcc = String::from_utf8_lossy(&p.fourcc);
                container_parts.push(format!("{}({}B)", fcc, p.data.len()));
            }
            if let Some(prog) = c.program() {
                shader_model_or_version = format!(
                    "{} (SM {}.{}, DXIL v{:#x})",
                    prog.profile(),
                    prog.major,
                    prog.minor,
                    prog.dxil_version
                );
                if prog.kind == ProgramKind::Library {
                    for lf in c.library_functions() {
                        entry_points.push((lf.unmangled_name, lf.kind.short_name().to_string()));
                    }
                } else {
                    let ep = c.entry_point().unwrap_or_else(|| "main".to_string());
                    entry_points.push((ep, prog.profile()));
                }
            } else if c.part(FOURCC_RTS0).is_some() {
                shader_model_or_version = "RootSignature (RTS0)".to_string();
            }
            if let Some(res) = c.resource_bindings() {
                resource_count = res.len();
            }
            has_root_signature_or_push_constants = c.part(FOURCC_RTS0).is_some();
            if let Some((min_w, max_w)) = c.expected_wave_lane_count() {
                if min_w > 0 {
                    wave_or_workgroup_info = format!("WaveSize({min_w}..{max_w})");
                } else {
                    wave_or_workgroup_info = "Wave32/unpinned".to_string();
                }
            }
            let _ = (FOURCC_PSV0, FOURCC_RDAT, FOURCC_SFI0, FOURCC_HASH);
        }
        ShaderFormatId::DxilRawBitcode => {
            if let Some(prog) = ProgramInfo::parse(bytes) {
                shader_model_or_version = format!("Bare DXIL {}", prog.profile());
                let bc = bitcode::dxil_part_bitcode(bytes).unwrap_or(bytes);
                let funcs = bitcode::defined_functions(bc).unwrap_or_default();
                for f in funcs {
                    entry_points.push((f, prog.profile()));
                }
            } else {
                shader_model_or_version = "LLVM 3.7 Bitcode (BC\\xc0\\xde)".to_string();
                let funcs = bitcode::defined_functions(bytes).unwrap_or_default();
                for f in funcs {
                    entry_points.push((f, "dxil_bc".to_string()));
                }
            }
        }
        ShaderFormatId::VulkanSpirvBinary => {
            let sum = summarize_spirv(bytes)?;
            shader_model_or_version = format!(
                "{} (Generator: {}, Bound: {}, Words: {})",
                sum.header.version_string(),
                sum.header.generator_vendor,
                sum.header.bound,
                sum.header.word_count
            );
            for ep in &sum.entry_points {
                entry_points.push((ep.name.clone(), ep.profile.clone()));
                if ep.stage == ShaderStage::Compute {
                    wave_or_workgroup_info = format!(
                        "WorkgroupSize({}, {}, {})",
                        ep.workgroup_size[0], ep.workgroup_size[1], ep.workgroup_size[2]
                    );
                }
            }
            resource_count = sum.resources.len();
            has_root_signature_or_push_constants = sum.has_push_constants;
        }
        ShaderFormatId::WgslSource => {
            let src = std::str::from_utf8(bytes)
                .map_err(|_| Error::InvalidInput("WGSL source is not valid UTF-8".into()))?;
            let trans = crate::spirv_metal::translate_wgsl_to_msl(src, None)?;
            shader_model_or_version = "WebGPU WGSL 1.0 -> MSL 3.1".to_string();
            entry_points.push((trans.entry_point_name, format!("{:?}", trans.stage)));
            resource_count = trans.reflection.resources.len();
            if let Some(tg) = trans.reflection.threadgroup_size() {
                wave_or_workgroup_info = format!("WorkgroupSize({}, {}, {})", tg[0], tg[1], tg[2]);
            }
        }
        ShaderFormatId::MetalMslSource => {
            shader_model_or_version = "Apple Metal Shading Language (MSL)".to_string();
        }
        ShaderFormatId::MetalMetallib => {
            shader_model_or_version = "Apple Compiled Metal Library (MTLB)".to_string();
        }
        ShaderFormatId::TranslatorAw2sCache => {
            shader_model_or_version = "Translator Cached Metal Shader (AW2SHC02)".to_string();
        }
        ShaderFormatId::PeEmbeddedContainer => {
            let dxbc_list = extract_pe_shaders(name, bytes);
            let spirv_list = extract_pe_spirv_shaders(name, bytes);
            embedded_dxbc_count = dxbc_list.len();
            embedded_spirv_count = spirv_list.len();
            shader_model_or_version = format!(
                "PE Image ({} DXBC/DXIL, {} Vulkan SPIR-V)",
                embedded_dxbc_count, embedded_spirv_count
            );
            for sh in dxbc_list.iter().take(6) {
                for exp in sh.exports() {
                    entry_points.push((
                        format!("{}@{:#x}", exp.entry_point, sh.file_offset),
                        exp.profile,
                    ));
                }
            }
            for spv in spirv_list.iter().take(6) {
                for ep in &spv.entry_points {
                    entry_points.push((
                        format!("{}@{:#x}", ep.name, spv.file_offset),
                        ep.profile.clone(),
                    ));
                }
            }
        }
        _ => {}
    }

    Ok(ShaderInspection {
        name: name.to_string(),
        byte_len: bytes.len(),
        format,
        shader_model_or_version,
        container_parts,
        entry_points,
        resource_count,
        has_root_signature_or_push_constants,
        wave_or_workgroup_info,
        embedded_dxbc_count,
        embedded_spirv_count,
    })
}

/// Result of converting a raw shader file (DirectX 12 `.dxil`/`.cso`, DirectX 11 `.dxbc`,
/// Vulkan `.spv`, WebGPU `.wgsl`, or `.metal`) into a native Apple `.metallib`.
#[derive(Debug, Clone)]
pub struct RawShaderConversion {
    pub source_format: ShaderFormatId,
    pub compiled: CompiledShader,
    /// Generated Metal Shading Language (`.msl`) source when converted via the Vulkan `SPIR-V` /
    /// `WGSL` / `MSL` pipeline (`None` when converted directly to Metal IR bitcode via
    /// `libmetalirconverter.dylib`).
    pub msl_source: Option<String>,
    pub pipeline_used: String,
}

/// Converts any raw DirectX 12 (`DXBC_DXIL_SM6`, `DXIL_RAW_BITCODE`), DirectX 11 (`DXBC_FXC_SM4_5`),
/// Vulkan (`VULKAN_SPIRV_BINARY`), WebGPU (`WGSL_SOURCE`), or Metal (`METAL_MSL_SOURCE`,
/// `METAL_METALLIB`) shader payload into a native [`CompiledShader`] (`.metallib` + reflection).
pub fn convert_raw_shader_to_metal(
    path: Option<&Path>,
    bytes: &[u8],
    entry_point: Option<&str>,
    pipeline: Option<&ShaderPipeline>,
) -> Result<RawShaderConversion, Error> {
    let format = detect_shader_format(path, bytes);

    match format {
        ShaderFormatId::DxbcDxilSm6 | ShaderFormatId::DxbcFxcSm4_5 => {
            let owned_pipe;
            let pipe = match pipeline {
                Some(p) => p,
                None => {
                    owned_pipe = ShaderPipeline::new(CacheConfig::from_env())?;
                    &owned_pipe
                }
            };
            let opts = CompileOptions::default();
            let c = Container::parse(bytes)?;
            let resolved_ep = entry_point
                .map(|s| s.to_string())
                .or_else(|| c.entry_point())
                .unwrap_or_else(|| "main".to_string());

            let compiled = if c.part(FOURCC_RTS0).is_some() {
                if let Ok(rs) = pipe.root_signature(bytes) {
                    let req = CompileRequest::new(bytes, Some(rs.as_ref()), &opts)
                        .entry_point(&resolved_ep);
                    pipe.compile(&req).or_else(|_| {
                        let cov = RootSignatureDesc::covering_dxil(bytes)
                            .and_then(|d| pipe.root_signature(&d.to_blob()).ok());
                        let req2 = CompileRequest::new(bytes, cov.as_deref(), &opts)
                            .entry_point(&resolved_ep);
                        pipe.compile(&req2)
                    })?
                } else {
                    let cov = RootSignatureDesc::covering_dxil(bytes)
                        .and_then(|d| pipe.root_signature(&d.to_blob()).ok());
                    let req =
                        CompileRequest::new(bytes, cov.as_deref(), &opts).entry_point(&resolved_ep);
                    pipe.compile(&req)?
                }
            } else {
                let cov = RootSignatureDesc::covering_dxil(bytes)
                    .and_then(|d| pipe.root_signature(&d.to_blob()).ok());
                let req =
                    CompileRequest::new(bytes, cov.as_deref(), &opts).entry_point(&resolved_ep);
                pipe.compile(&req)?
            };

            let pipeline_used = if format == ShaderFormatId::DxbcFxcSm4_5 {
                "DirectX 10/11 DXBC (SHEX/SHDR) -> libdxilconv.dylib -> DXIL SM 6.0 -> libmetalirconverter.dylib -> Metal .metallib".to_string()
            } else {
                "DirectX 12 DXBC (DXIL SM 6.x) -> libmetalirconverter.dylib -> Metal .metallib".to_string()
            };

            Ok(RawShaderConversion {
                source_format: format,
                compiled: (*compiled).clone(),
                msl_source: None,
                pipeline_used,
            })
        }
        ShaderFormatId::DxilRawBitcode => {
            // Wrap bare DXIL program header in a signed DXBC container and run through ShaderPipeline
            let wrapped = build_container(&[(FOURCC_DXIL, bytes)]);
            let mut conv = convert_raw_shader_to_metal(None, &wrapped, entry_point, pipeline)?;
            conv.source_format = ShaderFormatId::DxilRawBitcode;
            conv.pipeline_used = "DirectX 12 Bare DXIL -> Wrapped Signed DXBC -> libmetalirconverter.dylib -> Metal .metallib".to_string();
            Ok(conv)
        }
        ShaderFormatId::VulkanSpirvBinary => {
            let (compiled, msl) = compile_spirv_uncached(bytes, entry_point)?;
            if let Some(p) = pipeline {
                let _ = compile_spirv_cached(p.cache(), bytes, entry_point);
            }
            Ok(RawShaderConversion {
                source_format: format,
                compiled,
                msl_source: Some(msl),
                pipeline_used: "Vulkan SPIR-V (0x07230203) -> naga (spv-in -> msl-out Metal 3.1) -> xcrun metal (.air) -> xcrun metallib (.metallib)".to_string(),
            })
        }
        ShaderFormatId::WgslSource => {
            let src = std::str::from_utf8(bytes)
                .map_err(|_| Error::InvalidInput("WGSL source is not valid UTF-8".into()))?;
            let (compiled, msl) = compile_wgsl_uncached(src, entry_point)?;
            Ok(RawShaderConversion {
                source_format: format,
                compiled,
                msl_source: Some(msl),
                pipeline_used: "WebGPU WGSL -> naga (wgsl-in -> msl-out Metal 3.1) -> xcrun metal (.air) -> xcrun metallib (.metallib)".to_string(),
            })
        }
        ShaderFormatId::MetalMslSource => {
            let src = std::str::from_utf8(bytes)
                .map_err(|_| Error::InvalidInput("Metal source is not valid UTF-8".into()))?;
            let metallib = compile_msl_to_metallib(src)?;
            let fn_name = entry_point.unwrap_or("main0").to_string();
            let stage = if src.contains("kernel ") {
                ShaderStage::Compute
            } else if src.contains("vertex ") {
                ShaderStage::Vertex
            } else {
                ShaderStage::Fragment
            };
            let refl = ShaderReflection {
                stage,
                function_name: fn_name.clone(),
                resources: Vec::new(),
                needs_function_constants: false,
                function_constants: Vec::new(),
                info: StageInfo::None,
                json: format!("{{\"source_format\":\"MSL\",\"function_name\":\"{fn_name}\"}}"),
            };
            Ok(RawShaderConversion {
                source_format: format,
                compiled: CompiledShader {
                    metallib,
                    function_name: fn_name,
                    stage,
                    reflection: refl,
                    key: crate::hash::hash128(bytes),
                },
                msl_source: Some(src.to_string()),
                pipeline_used: "Apple Metal Source (.metal) -> xcrun metal (.air) -> xcrun metallib (.metallib)".to_string(),
            })
        }
        ShaderFormatId::MetalMetallib => {
            let fn_name = entry_point.unwrap_or("main0").to_string();
            let refl = ShaderReflection {
                stage: ShaderStage::Compute,
                function_name: fn_name.clone(),
                resources: Vec::new(),
                needs_function_constants: false,
                function_constants: Vec::new(),
                info: StageInfo::None,
                json: "{}".to_string(),
            };
            Ok(RawShaderConversion {
                source_format: format,
                compiled: CompiledShader {
                    metallib: bytes.to_vec(),
                    function_name: fn_name,
                    stage: ShaderStage::Compute,
                    reflection: refl,
                    key: crate::hash::hash128(bytes),
                },
                msl_source: None,
                pipeline_used: "Pre-compiled Apple Metal Library (MTLB pass-through)".to_string(),
            })
        }
        other => Err(Error::Convert {
            code: ErrorCode::Unknown,
            message: format!(
                "raw shader format {} ({}) cannot be compiled as a single shader blob",
                other.code(),
                other.spec().family
            ),
        }),
    }
}

/// Serializes `SHADER_FORMAT_CATALOG` into a standardized TSV database (`db/shader_formats.tsv`).
pub fn sync_shader_formats_tsv(tsv_path: &Path) -> std::io::Result<usize> {
    if let Some(parent) = tsv_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = String::new();
    out.push_str("# Standardized Shader File Format & Native Metal Conversion Pipeline Database\n");
    out.push_str("# format_code\tfamily\textensions\tmagic_signature\theader_layout\tinner_chunks_or_sections\tnative_metal_pipeline\tstatus\n");
    for spec in SHADER_FORMAT_CATALOG {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            spec.code,
            spec.family,
            spec.extensions,
            spec.magic_signature,
            spec.header_layout,
            spec.inner_chunks_or_sections,
            spec.native_metal_pipeline,
            spec.status
        );
    }
    std::fs::write(tsv_path, out)?;
    Ok(SHADER_FORMAT_CATALOG.len())
}

/// Formats a human-readable table of all standardized shader file formats and their native Metal pipelines.
pub fn format_shader_catalog_table() -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "================================================================================"
    );
    let _ = writeln!(
        out,
        "=== Standardized Shader File Formats & Native Metal Conversion Pipelines ({}) ===",
        SHADER_FORMAT_CATALOG.len()
    );
    let _ = writeln!(
        out,
        "================================================================================"
    );
    for (idx, s) in SHADER_FORMAT_CATALOG.iter().enumerate() {
        let _ = writeln!(
            out,
            "  {:>2}. {:<24} | {:<40} | Exts: {}",
            idx + 1,
            s.code,
            s.family,
            s.extensions
        );
        let _ = writeln!(out, "      Magic / Signature : {}", s.magic_signature);
        let _ = writeln!(out, "      Header Structure  : {}", s.header_layout);
        let _ = writeln!(
            out,
            "      Inner Chunks      : {}",
            s.inner_chunks_or_sections
        );
        let _ = writeln!(
            out,
            "      Metal Pipeline    : {} [{}]\n",
            s.native_metal_pipeline, s.status
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spirv_metal::sample_vulkan_compute_spirv;

    #[test]
    fn detects_and_converts_vulkan_spirv_and_directx_formats() {
        // 1. Vulkan SPIR-V format detection, inspection, and conversion to Metal .metallib
        let spv = sample_vulkan_compute_spirv();
        assert_eq!(
            detect_shader_format(Some(Path::new("test.spv")), &spv),
            ShaderFormatId::VulkanSpirvBinary
        );
        let insp = inspect_shader_bytes("test.spv", Some(Path::new("test.spv")), &spv)
            .expect("inspect spirv");
        assert_eq!(insp.format, ShaderFormatId::VulkanSpirvBinary);
        assert_eq!(insp.entry_points.len(), 1);
        assert_eq!(insp.entry_points[0].0, "main");

        if crate::spirv_metal::metal_toolchain_available() {
            let conv = convert_raw_shader_to_metal(Some(Path::new("test.spv")), &spv, Some("main"), None)
                .expect("convert spirv to metal");
            assert_eq!(conv.source_format, ShaderFormatId::VulkanSpirvBinary);
            assert!(conv.compiled.metallib.starts_with(b"MTLB"));
            assert!(conv.msl_source.as_deref().unwrap().contains("kernel void"));
        } else {
            let trans = crate::spirv_metal::translate_spirv_to_msl(&spv, Some("main"))
                .expect("translate spirv to msl");
            assert!(trans.msl_source.contains("kernel void"));
            assert!(convert_raw_shader_to_metal(Some(Path::new("test.spv")), &spv, Some("main"), None).is_err());
        }

        // 2. DirectX 12 DXIL & DirectX 11 DXBC format detection and conversion (from NRD.dll if present)
        let nrd_path = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(|d| std::path::PathBuf::from(d).join("NRD.dll"));
        if let Some(nrd) = nrd_path.as_deref().filter(|p| p.exists() && crate::converter_available()) {
            let bytes = std::fs::read(nrd).expect("read NRD.dll");
            assert_eq!(
                detect_shader_format(Some(nrd), &bytes),
                ShaderFormatId::PeEmbeddedContainer
            );
            let extracted = extract_pe_shaders("NRD.dll", &bytes);
            let dxil_blob = &extracted
                .iter()
                .find(|s| !s.is_sm4_5)
                .expect("DXIL shader in NRD.dll")
                .dxbc_bytes;
            assert_eq!(
                detect_shader_format(Some(Path::new("nrd_cs.dxil")), dxil_blob),
                ShaderFormatId::DxbcDxilSm6
            );
            let dx12_conv = convert_raw_shader_to_metal(
                Some(Path::new("nrd_cs.dxil")),
                dxil_blob,
                None,
                None,
            )
            .expect("convert DX12 DXIL to Metal .metallib");
            assert_eq!(dx12_conv.source_format, ShaderFormatId::DxbcDxilSm6);
            assert!(dx12_conv.compiled.metallib.starts_with(b"MTLB"));

            if crate::dxilconv::available() {
                let dxbc_blob = &extracted
                    .iter()
                    .find(|s| s.is_sm4_5)
                    .expect("SM5 DXBC shader in NRD.dll")
                    .dxbc_bytes;
                assert_eq!(
                    detect_shader_format(Some(Path::new("nrd_cs.dxbc")), dxbc_blob),
                    ShaderFormatId::DxbcFxcSm4_5
                );
                let dx11_conv = convert_raw_shader_to_metal(
                    Some(Path::new("nrd_cs.dxbc")),
                    dxbc_blob,
                    None,
                    None,
                )
                .expect("convert DX11 DXBC to Metal .metallib");
                assert_eq!(dx11_conv.source_format, ShaderFormatId::DxbcFxcSm4_5);
                assert!(dx11_conv.compiled.metallib.starts_with(b"MTLB"));
            }
        }
    }
}
