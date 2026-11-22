//! Raw C ABI of Apple's Metal Shader Converter (`metal_irconverter.h`, IR 4.0.x).
//!
//! The library is `dlopen`ed at runtime (it is not redistributable and may be missing), so every
//! entry point lives in [`Api`] as a function pointer. Struct layouts mirror the C header exactly
//! (all enums are C `int`-sized, passed as `u32`/`i32`). Everything here is `unsafe` and untyped;
//! the safe API is in the other modules.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::Error;

/// Header version we were written against (`ir_version.h`).
pub const IR_VERSION: (u32, u32, u32) = (4, 0, 1);

// ---- opaque handles -------------------------------------------------------------------------
#[repr(C)]
pub struct IRCompiler {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRObject {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRRootSignature {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRMetalLibBinary {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRShaderReflection {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRError {
    _p: [u8; 0],
}
#[repr(C)]
pub struct IRRayTracingPipelineConfiguration {
    _p: [u8; 0],
}

// ---- enums (values) -------------------------------------------------------------------------
pub const IRBytecodeOwnershipNone: c_int = 0;
pub const IRBytecodeOwnershipCopy: c_int = 1;

pub const IRRootSignatureVersion_1_0: u32 = 1;
pub const IRRootSignatureVersion_1_1: u32 = 2;

pub const IRReflectionVersion_1_0: u32 = 1;
pub const IRInputLayoutDescriptorVersion_1: u32 = 1;

pub const IROperatingSystem_macOS: c_int = 0;

pub const IRDescriptorRangeOffsetAppend: u32 = 0xFFFF_FFFF;

// ---- root signature descriptors ------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRDescriptorRange {
    pub RangeType: u32,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub OffsetInDescriptorsFromTableStart: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRRootDescriptorTable {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const IRDescriptorRange,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRRootConstants {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Num32BitValues: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRRootDescriptor {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IRRootParameterU {
    pub DescriptorTable: IRRootDescriptorTable,
    pub Constants: IRRootConstants,
    pub Descriptor: IRRootDescriptor,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRRootParameter {
    pub ParameterType: u32,
    pub u: IRRootParameterU,
    pub ShaderVisibility: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRStaticSamplerDescriptor {
    pub Filter: u32,
    pub AddressU: u32,
    pub AddressV: u32,
    pub AddressW: u32,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: u32,
    pub BorderColor: u32,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub ShaderVisibility: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRRootSignatureDescriptor {
    pub NumParameters: u32,
    pub pParameters: *const IRRootParameter,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const IRStaticSamplerDescriptor,
    pub Flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRDescriptorRange1 {
    pub RangeType: u32,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: u32,
    pub OffsetInDescriptorsFromTableStart: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRRootDescriptorTable1 {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const IRDescriptorRange1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRRootDescriptor1 {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IRRootParameter1U {
    pub DescriptorTable: IRRootDescriptorTable1,
    pub Constants: IRRootConstants,
    pub Descriptor: IRRootDescriptor1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRRootParameter1 {
    pub ParameterType: u32,
    pub u: IRRootParameter1U,
    pub ShaderVisibility: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRRootSignatureDescriptor1 {
    pub NumParameters: u32,
    pub pParameters: *const IRRootParameter1,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const IRStaticSamplerDescriptor,
    pub Flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IRVersionedRootSignatureDescriptorU {
    pub desc_1_0: IRRootSignatureDescriptor,
    pub desc_1_1: IRRootSignatureDescriptor1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRVersionedRootSignatureDescriptor {
    pub version: u32,
    pub u: IRVersionedRootSignatureDescriptorU,
}

// ---- input layout ---------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRInputElementDescriptor1 {
    pub semanticIndex: u32,
    pub format: u32,
    pub inputSlot: u32,
    pub alignedByteOffset: u32,
    pub instanceDataStepRate: u32,
    pub inputSlotClass: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRInputLayoutDescriptor1 {
    pub semanticNames: [*const c_char; 31],
    pub inputElementDescs: [IRInputElementDescriptor1; 31],
    pub numElements: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRVersionedInputLayoutDescriptor {
    pub version: u32,
    pub desc_1_0: IRInputLayoutDescriptor1,
}

// ---- reflection -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRResourceLocation {
    pub resourceType: u32,
    pub space: u32,
    pub slot: u32,
    pub topLevelOffset: u32,
    pub sizeBytes: u64,
    pub resourceName: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRFunctionConstant {
    pub name: *const c_char,
    pub r#type: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRCSInfo_1_0 {
    pub tg_size: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRVertexInputInfo_1_0 {
    pub name: *const c_char,
    pub attributeIndex: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRVSInfo_1_0 {
    pub instance_id_index: c_int,
    pub vertex_id_index: c_int,
    pub vertex_output_size_in_bytes: u32,
    pub needs_draw_params: bool,
    pub vertex_inputs: *mut IRVertexInputInfo_1_0,
    pub num_vertex_inputs: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRFSInfo_1_0 {
    pub num_render_targets: c_int,
    pub rt_index_int: u8,
    pub discards: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRVertexOutputInfo_1_0 {
    pub name: *const c_char,
    pub attributeIndex: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRGSInfo_1_0 {
    pub vertex_outputs: *mut IRVertexOutputInfo_1_0,
    pub num_vertex_outputs: usize,
    pub is_passthrough: bool,
    pub rt_array_index_record_id: i32,
    pub viewport_array_index_record_id: i32,
    pub input_primitive: u32,
    pub max_input_primitives_per_mesh_threadgroup: u32,
    pub max_payload_size_in_bytes: u32,
    pub instance_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IRHSInfo_1_0 {
    pub max_patches_per_object_threadgroup: u32,
    pub max_object_threads_per_patch: u32,
    pub patch_constants_size: u32,
    pub patch_constant_function: *const c_char,
    pub static_payload_size: u32,
    pub payload_size_per_patch: u32,
    pub input_control_point_count: u32,
    pub output_control_point_count: u32,
    pub output_control_point_size: u32,
    pub tessellator_domain: u32,
    pub tessellator_partitioning: u32,
    pub tessellator_output_primitive: u32,
    pub tessellation_type_half: bool,
    pub max_tessellation_factor: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRDSInfo_1_0 {
    pub tessellator_domain: u32,
    pub max_input_prims_per_mesh_threadgroup: u32,
    pub input_control_point_count: u32,
    pub input_control_point_size: u32,
    pub patch_constants_size: u32,
    pub tessellation_type_half: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRMSInfo_1_0 {
    pub max_vertex_output_count: u32,
    pub max_primitive_output_count: u32,
    pub primitive_topology: u32,
    pub max_payload_size_in_bytes: u32,
    pub num_threads: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRASInfo_1_0 {
    pub num_threads: [u32; 3],
    pub max_payload_size_in_bytes: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct IRRTInfo_1_0 {
    pub is_indirect_intersection_function: bool,
}

/// `IRVersioned*Info`: `{ IRReflectionVersion version; union { T info_1_0; }; }`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Versioned<T: Copy> {
    pub version: u32,
    pub info: T,
}

// ---- function table -------------------------------------------------------------------------
macro_rules! api {
    ($( $name:ident : fn($($arg:ty),*) $(-> $ret:ty)? ;)*) => {
        /// All converter entry points, resolved with `dlsym`.
        pub struct Api {
            /// Path of the loaded dylib.
            pub path: PathBuf,
            /// Identity of the loaded dylib (size + mtime), part of every cache key.
            pub fingerprint: String,
            $( pub $name: unsafe extern "C" fn($($arg),*) $(-> $ret)?, )*
        }
        impl Api {
            unsafe fn resolve(handle: *mut c_void, path: PathBuf, fingerprint: String) -> Result<Self, Error> {
                Ok(Self {
                    path,
                    fingerprint,
                    $( $name: {
                        let sym = dlsym(handle, concat!(stringify!($name), "\0").as_ptr().cast());
                        if sym.is_null() {
                            return Err(Error::Load(format!("missing symbol {}", stringify!($name))));
                        }
                        std::mem::transmute::<*mut c_void, unsafe extern "C" fn($($arg),*) $(-> $ret)?>(sym)
                    }, )*
                })
            }
        }
    };
}

type Cstr = *const c_char;

api! {
    IRErrorGetCode: fn(*const IRError) -> u32;
    IRErrorGetPayload: fn(*const IRError) -> *const c_void;
    IRErrorDestroy: fn(*mut IRError);

    IRRootSignatureCreateFromDescriptor: fn(*const IRVersionedRootSignatureDescriptor, *mut *mut IRError) -> *mut IRRootSignature;
    IRRootSignatureDestroy: fn(*mut IRRootSignature);
    IRRootSignatureGetResourceCount: fn(*const IRRootSignature) -> usize;
    IRRootSignatureGetResourceLocations: fn(*const IRRootSignature, *mut IRResourceLocation);
    IRVersionedRootSignatureDescriptorCreateFromBlob: fn(*const u8, u32, *mut *mut IRError) -> *mut IRVersionedRootSignatureDescriptor;
    IRVersionedRootSignatureDescriptorRelease: fn(*mut IRVersionedRootSignatureDescriptor);
    IRVersionedRootSignatureDescriptorCopyJSONString: fn(*const IRVersionedRootSignatureDescriptor) -> Cstr;
    IRVersionedRootSignatureDescriptorReleaseString: fn(Cstr);

    IRObjectCreateFromDXIL: fn(*const u8, usize, c_int) -> *mut IRObject;
    IRObjectDestroy: fn(*mut IRObject);
    IRObjectGetType: fn(*const IRObject) -> u32;
    IRObjectGetMetalIRShaderStage: fn(*const IRObject) -> u32;
    IRObjectGetMetalLibBinary: fn(*const IRObject, u32, *mut IRMetalLibBinary) -> bool;
    IRObjectGetReflection: fn(*const IRObject, u32, *mut IRShaderReflection) -> bool;
    IRObjectGatherRaytracingIntrinsics: fn(*mut IRObject, Cstr) -> u64;

    IRCompilerCreate: fn() -> *mut IRCompiler;
    IRCompilerDestroy: fn(*mut IRCompiler);
    IRCompilerAllocCompileAndLink: fn(*mut IRCompiler, Cstr, *const IRObject, *mut *mut IRError) -> *mut IRObject;
    IRCompilerAllocCombineCompileAndLink: fn(*mut IRCompiler, Cstr, *const IRObject, Cstr, *const IRObject, *mut *mut IRError) -> *mut IRObject;
    IRCompilerSetValidationFlags: fn(*mut IRCompiler, u32);
    IRCompilerSetStageInGenerationMode: fn(*mut IRCompiler, u32);
    IRCompilerSetGlobalRootSignature: fn(*mut IRCompiler, *const IRRootSignature);
    IRCompilerSetLocalRootSignature: fn(*mut IRCompiler, *const IRRootSignature);
    IRCompilerSetHitgroupType: fn(*mut IRCompiler, u32);
    IRCompilerSetRayTracingPipelineConfiguration: fn(*mut IRCompiler, *const IRRayTracingPipelineConfiguration);
    IRCompilerSetCompatibilityFlags: fn(*mut IRCompiler, u32);
    IRCompilerSetInputTopology: fn(*mut IRCompiler, u32);
    IRCompilerEnableGeometryAndTessellationEmulation: fn(*mut IRCompiler, bool);
    IRCompilerSetDualSourceBlendingConfiguration: fn(*mut IRCompiler, u32);
    IRCompilerSetDepthFeedbackConfiguration: fn(*mut IRCompiler, u32);
    IRCompilerSetIntRTMask: fn(*mut IRCompiler, u8);
    IRCompilerSetSampleMask: fn(*mut IRCompiler, u32);
    IRCompilerSetEntryPointName: fn(*mut IRCompiler, Cstr);
    IRCompilerSetMinimumGPUFamily: fn(*mut IRCompiler, u32);
    IRCompilerIgnoreRootSignature: fn(*mut IRCompiler, bool);
    IRCompilerIgnoreDebugInformation: fn(*mut IRCompiler, bool);
    IRCompilerSetMinimumDeploymentTarget: fn(*mut IRCompiler, c_int, Cstr);
    IRCompilerSetFunctionConstantResourceSpace: fn(*mut IRCompiler, u32);
    IRCompilerSetFramebufferFetchResourceSpace: fn(*mut IRCompiler, u32);

    IRRayTracingPipelineConfigurationCreate: fn() -> *mut IRRayTracingPipelineConfiguration;
    IRRayTracingPipelineConfigurationDestroy: fn(*mut IRRayTracingPipelineConfiguration);
    IRRayTracingPipelineConfigurationSetMaxAttributeSizeInBytes: fn(*mut IRRayTracingPipelineConfiguration, u32);
    IRRayTracingPipelineConfigurationSetPipelineFlags: fn(*mut IRRayTracingPipelineConfiguration, u32);
    IRRayTracingPipelineConfigurationSetIntrinsicMasks: fn(*mut IRRayTracingPipelineConfiguration, u64, u64, u64, u64);
    IRRayTracingPipelineConfigurationSetMaxRecursiveDepth: fn(*mut IRRayTracingPipelineConfiguration, c_int);
    IRRayTracingPipelineConfigurationSetRayGenerationCompilationMode: fn(*mut IRRayTracingPipelineConfiguration, u32);
    IRRayTracingPipelineConfigurationSetIntersectionFunctionCompilationMode: fn(*mut IRRayTracingPipelineConfiguration, u32);
    IRRayTracingPipelineConfigurationEnableIntersectionFunctionGroups: fn(*mut IRRayTracingPipelineConfiguration, bool);
    IRRayTracingPipelineConfigurationEnableDirectStateAccess: fn(*mut IRRayTracingPipelineConfiguration, bool);

    IRMetalLibSynthesizeStageInFunction: fn(*const IRCompiler, *const IRShaderReflection, *const IRVersionedInputLayoutDescriptor, *mut IRMetalLibBinary) -> bool;
    IRMetalLibSynthesizeIndirectRayDispatchFunction: fn(*const IRCompiler, *mut IRMetalLibBinary) -> bool;
    IRMetalLibSynthesizeIndirectIntersectionFunction: fn(*const IRCompiler, *mut IRMetalLibBinary) -> bool;
    IRMetalLibBinaryCreate: fn() -> *mut IRMetalLibBinary;
    IRMetalLibBinaryDestroy: fn(*mut IRMetalLibBinary);
    IRMetalLibGetBytecode: fn(*const IRMetalLibBinary, *mut u8) -> usize;
    IRMetalLibGetBytecodeSize: fn(*const IRMetalLibBinary) -> usize;

    IRShaderReflectionCreate: fn() -> *mut IRShaderReflection;
    IRShaderReflectionDestroy: fn(*mut IRShaderReflection);
    IRShaderReflectionCreateFromJSON: fn(Cstr) -> *mut IRShaderReflection;
    IRShaderReflectionCopyJSONString: fn(*const IRShaderReflection) -> Cstr;
    IRShaderReflectionReleaseString: fn(Cstr);
    IRShaderReflectionGetEntryPointFunctionName: fn(*const IRShaderReflection) -> Cstr;
    IRShaderReflectionNeedsFunctionConstants: fn(*const IRShaderReflection) -> bool;
    IRShaderReflectionGetFunctionConstantCount: fn(*const IRShaderReflection) -> usize;
    IRShaderReflectionCopyFunctionConstants: fn(*const IRShaderReflection, *mut IRFunctionConstant);
    IRShaderReflectionReleaseFunctionConstants: fn(*mut IRFunctionConstant, usize);
    IRShaderReflectionGetResourceCount: fn(*const IRShaderReflection) -> usize;
    IRShaderReflectionGetResourceLocations: fn(*const IRShaderReflection, *mut IRResourceLocation);
    IRShaderReflectionCopyComputeInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRCSInfo_1_0>) -> bool;
    IRShaderReflectionCopyVertexInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRVSInfo_1_0>) -> bool;
    IRShaderReflectionCopyFragmentInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRFSInfo_1_0>) -> bool;
    IRShaderReflectionCopyGeometryInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRGSInfo_1_0>) -> bool;
    IRShaderReflectionCopyHullInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRHSInfo_1_0>) -> bool;
    IRShaderReflectionCopyDomainInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRDSInfo_1_0>) -> bool;
    IRShaderReflectionCopyMeshInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRMSInfo_1_0>) -> bool;
    IRShaderReflectionCopyAmplificationInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRASInfo_1_0>) -> bool;
    IRShaderReflectionCopyRaytracingInfo: fn(*const IRShaderReflection, u32, *mut Versioned<IRRTInfo_1_0>) -> bool;
    IRShaderReflectionReleaseComputeInfo: fn(*mut Versioned<IRCSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseVertexInfo: fn(*mut Versioned<IRVSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseFragmentInfo: fn(*mut Versioned<IRFSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseGeometryInfo: fn(*mut Versioned<IRGSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseHullInfo: fn(*mut Versioned<IRHSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseDomainInfo: fn(*mut Versioned<IRDSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseMeshInfo: fn(*mut Versioned<IRMSInfo_1_0>) -> bool;
    IRShaderReflectionReleaseAmplificationInfo: fn(*mut Versioned<IRASInfo_1_0>) -> bool;
    IRShaderReflectionReleaseRaytracingInfo: fn(*mut Versioned<IRRTInfo_1_0>) -> bool;
}

// SAFETY: plain function pointers into a library that is never unloaded.
unsafe impl Send for Api {}
unsafe impl Sync for Api {}

extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}
const RTLD_NOW: c_int = 0x2;
const RTLD_LOCAL: c_int = 0x4;

/// Candidate locations of `libmetalirconverter.dylib`, in search order.
pub fn candidate_paths() -> Vec<PathBuf> {
    let mut c: Vec<PathBuf> = Vec::new();
    if let Ok(p) = std::env::var("METAL_IRCONVERTER_DYLIB") {
        c.push(p.into());
    }
    if let Ok(home) = std::env::var("HOME") {
        // User-local unpack of Apple's official package (no sudo needed); see scripts/setup.sh.
        // Checked before the app bundle copy so prewarmed caches and app runs share the same fingerprint.
        c.push(Path::new(&home).join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            c.push(dir.join("../Frameworks/libmetalirconverter.dylib"));
            c.push(dir.join("libmetalirconverter.dylib"));
        }
    }
    c.push("/usr/local/lib/libmetalirconverter.dylib".into());
    c.push("/opt/metal-shaderconverter/lib/libmetalirconverter.dylib".into());
    c
}

static API: OnceLock<Result<Api, Error>> = OnceLock::new();

/// The process-wide converter API (loaded on first use).
pub fn api() -> Result<&'static Api, Error> {
    API.get_or_init(load).as_ref().map_err(Clone::clone)
}

/// Extract the immutable Mach-O `LC_UUID` from a thin or universal Mach-O binary so the
/// converter cache fingerprint stays identical when `libmetalirconverter.dylib` is copied
/// into `.app/Contents/Frameworks/` or installed on another Mac (where `mtime` differs).
pub(crate) fn macho_uuid(path: &Path) -> Option<[u8; 16]> {
    use std::io::{Read, Seek, SeekFrom};

    fn parse_thin_uuid(f: &mut std::fs::File, slice_off: u64) -> Option<[u8; 16]> {
        f.seek(SeekFrom::Start(slice_off)).ok()?;
        let mut hdr = [0u8; 32];
        f.read_exact(&mut hdr).ok()?;
        let magic = u32::from_le_bytes(hdr[0..4].try_into().ok()?);
        if magic != 0xfeed_facf {
            return None;
        }
        let ncmds = u32::from_le_bytes(hdr[16..20].try_into().ok()?) as usize;
        let sizeofcmds = u32::from_le_bytes(hdr[20..24].try_into().ok()?) as usize;
        if sizeofcmds == 0 || sizeofcmds > 2 * 1024 * 1024 {
            return None;
        }
        let mut cmds = vec![0u8; sizeofcmds];
        f.read_exact(&mut cmds).ok()?;
        let mut off = 0usize;
        for _ in 0..ncmds {
            if off + 8 > cmds.len() {
                break;
            }
            let cmd = u32::from_le_bytes(cmds[off..off + 4].try_into().ok()?);
            let cmdsize = u32::from_le_bytes(cmds[off + 4..off + 8].try_into().ok()?) as usize;
            if cmdsize < 8 || off + cmdsize > cmds.len() {
                break;
            }
            // LC_UUID = 0x1b (cmd + cmdsize + 16-byte uuid = 24 bytes)
            if cmd == 0x1b && cmdsize >= 24 {
                let mut uuid = [0u8; 16];
                uuid.copy_from_slice(&cmds[off + 8..off + 24]);
                return Some(uuid);
            }
            off += cmdsize;
        }
        None
    }

    let mut f = std::fs::File::open(path).ok()?;
    let mut head = [0u8; 8];
    f.read_exact(&mut head).ok()?;
    let magic_be = u32::from_be_bytes(head[0..4].try_into().ok()?);
    let magic_le = u32::from_le_bytes(head[0..4].try_into().ok()?);
    if magic_le == 0xfeed_facf {
        return parse_thin_uuid(&mut f, 0);
    }
    const CPU_TYPE_ARM64: u32 = 0x0100_000c;
    if magic_be == 0xcafe_babe || magic_be == 0xcafe_babf {
        let nfat = u32::from_be_bytes(head[4..8].try_into().ok()?) as usize;
        if nfat == 0 || nfat > 32 {
            return None;
        }
        let is_64 = magic_be == 0xcafe_babf;
        let stride = if is_64 { 32 } else { 20 };
        let mut table = vec![0u8; nfat * stride];
        f.read_exact(&mut table).ok()?;
        let mut fallback_off = None;
        let mut arm64_off = None;
        for i in 0..nfat {
            let e = &table[i * stride..(i + 1) * stride];
            let cputype = u32::from_be_bytes(e[0..4].try_into().ok()?);
            let slice_off = if is_64 {
                u64::from_be_bytes(e[8..16].try_into().ok()?)
            } else {
                u32::from_be_bytes(e[8..12].try_into().ok()?) as u64
            };
            if fallback_off.is_none() {
                fallback_off = Some(slice_off);
            }
            if cputype == CPU_TYPE_ARM64 {
                arm64_off = Some(slice_off);
                break;
            }
        }
        if let Some(off) = arm64_off.or(fallback_off) {
            return parse_thin_uuid(&mut f, off);
        }
    }
    None
}

/// Stable converter fingerprint: prefers immutable Mach-O `LC_UUID` over filesystem `mtime`
/// and file length (which changes when `codesign --force --sign -` replaces Apple's signature
/// inside `.app/Contents/Frameworks/`) so prewarmed shader caches remain valid across re-signing,
/// copies, and different Macs.
pub(crate) fn converter_fingerprint_for_path(cand: &Path) -> String {
    if let Some(uuid) = macho_uuid(cand) {
        let mut hex = String::with_capacity(32);
        for b in uuid {
            use std::fmt::Write as _;
            let _ = write!(hex, "{b:02x}");
        }
        return format!("ir{}.{}.{}-uuid-{hex}", IR_VERSION.0, IR_VERSION.1, IR_VERSION.2);
    }
    let Ok(m) = std::fs::metadata(cand) else {
        return "unknown".into();
    };
    let mtime = m
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_secs());
    format!("ir{}.{}.{}-{}-{}", IR_VERSION.0, IR_VERSION.1, IR_VERSION.2, m.len(), mtime)
}

fn load() -> Result<Api, Error> {
    let mut tried = Vec::new();
    for cand in candidate_paths() {
        if !cand.exists() {
            continue;
        }
        let Ok(cpath) = CString::new(cand.to_string_lossy().as_bytes()) else { continue };
        // SAFETY: plain dlopen of an existing file.
        let h = unsafe { dlopen(cpath.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if h.is_null() {
            let msg = unsafe { dlerror() };
            let msg = if msg.is_null() { String::new() } else { unsafe { CStr::from_ptr(msg) }.to_string_lossy().into_owned() };
            tried.push(format!("{}: {msg}", cand.display()));
            continue;
        }
        let fingerprint = converter_fingerprint_for_path(&cand);
        // SAFETY: symbols are resolved against the documented prototypes above.
        return unsafe { Api::resolve(h, cand, fingerprint) };
    }
    Err(Error::Load(if tried.is_empty() {
        "libmetalirconverter.dylib not found (install Metal Shader Converter or set METAL_IRCONVERTER_DYLIB)".into()
    } else {
        format!("failed to dlopen libmetalirconverter.dylib: {}", tried.join("; "))
    }))
}

/// Copies a C string owned by the converter (may be null).
///
/// # Safety
/// `p` must be null or point to a NUL-terminated string valid for the duration of the call.
pub unsafe fn cstr(p: *const c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        CStr::from_ptr(p).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn struct_sizes_match_c_header() {
        // Sizes computed from metal_irconverter.h on LP64 (same for arm64 and x86_64).
        assert_eq!(size_of::<IRDescriptorRange>(), 20);
        assert_eq!(size_of::<IRDescriptorRange1>(), 24);
        assert_eq!(size_of::<IRRootParameter>(), 32);
        assert_eq!(size_of::<IRRootParameter1>(), 32);
        assert_eq!(size_of::<IRStaticSamplerDescriptor>(), 52);
        assert_eq!(size_of::<IRRootSignatureDescriptor>(), 40);
        assert_eq!(size_of::<IRVersionedRootSignatureDescriptor>(), 48);
        assert_eq!(size_of::<IRInputElementDescriptor1>(), 24);
        assert_eq!(size_of::<IRInputLayoutDescriptor1>(), 31 * 8 + 31 * 24 + 8);
        assert_eq!(align_of::<IRVersionedInputLayoutDescriptor>(), 8);
        assert_eq!(size_of::<IRResourceLocation>(), 32);
        assert_eq!(size_of::<Versioned<IRVSInfo_1_0>>(), 40);
        assert_eq!(size_of::<Versioned<IRMSInfo_1_0>>(), 32);
    }

    #[test]
    fn macho_uuid_thin_and_fat_stable_across_copies() {
        let dir = std::env::temp_dir().join(format!("dxil_metal_uuid_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Build a synthetic thin 64-bit Mach-O with LC_UUID = 01..10.
        let mut thin = vec![0u8; 32 + 24];
        thin[0..4].copy_from_slice(&0xfeed_facfu32.to_le_bytes());
        thin[16..20].copy_from_slice(&1u32.to_le_bytes()); // ncmds = 1
        thin[20..24].copy_from_slice(&24u32.to_le_bytes()); // sizeofcmds = 24
        thin[32..36].copy_from_slice(&0x1bu32.to_le_bytes()); // LC_UUID
        thin[36..40].copy_from_slice(&24u32.to_le_bytes());
        let expected_uuid: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        thin[40..56].copy_from_slice(&expected_uuid);

        let p_thin = dir.join("thin.dylib");
        std::fs::write(&p_thin, &thin).unwrap();
        assert_eq!(macho_uuid(&p_thin), Some(expected_uuid));
        let fp1 = converter_fingerprint_for_path(&p_thin);
        assert!(fp1.ends_with("-uuid-0102030405060708090a0b0c0d0e0f10"), "fp1={fp1}");

        // Appending trailing bytes (simulating `codesign --force --sign -` changing the
        // LC_CODE_SIGNATURE blob size inside `.app/Contents/Frameworks/`) must NOT alter
        // the converter fingerprint!
        let mut resigned = thin.clone();
        resigned.resize(thin.len() + 4096, 0xCC);
        let p_resigned = dir.join("thin_resigned.dylib");
        std::fs::write(&p_resigned, &resigned).unwrap();
        assert_eq!(converter_fingerprint_for_path(&p_resigned), fp1);

        // Build a synthetic universal fat Mach-O with an x86_64 slice (diff UUID) and ARM64 slice.
        let x86_off = 64u32;
        let arm_off = 128u32;
        let mut fat = vec![0u8; 256];
        fat[0..4].copy_from_slice(&0xcafe_babeu32.to_be_bytes());
        fat[4..8].copy_from_slice(&2u32.to_be_bytes()); // nfat = 2
        // arch 0: x86_64
        fat[8..12].copy_from_slice(&0x0100_0007u32.to_be_bytes());
        fat[16..20].copy_from_slice(&x86_off.to_be_bytes());
        fat[20..24].copy_from_slice(&(thin.len() as u32).to_be_bytes());
        // arch 1: arm64
        fat[28..32].copy_from_slice(&0x0100_000cu32.to_be_bytes());
        fat[36..40].copy_from_slice(&arm_off.to_be_bytes());
        fat[40..44].copy_from_slice(&(thin.len() as u32).to_be_bytes());
        // x86 slice with different UUID
        let mut x86_slice = thin.clone();
        x86_slice[40..56].fill(0xAA);
        fat[x86_off as usize..x86_off as usize + x86_slice.len()].copy_from_slice(&x86_slice);
        fat[arm_off as usize..arm_off as usize + thin.len()].copy_from_slice(&thin);

        let p_fat = dir.join("fat.dylib");
        std::fs::write(&p_fat, &fat).unwrap();
        assert_eq!(macho_uuid(&p_fat), Some(expected_uuid));

        // Verify real host libmetalirconverter.dylib (when installed) yields an immutable LC_UUID.
        for cand in candidate_paths() {
            if cand.is_file() {
                assert!(macho_uuid(&cand).is_some(), "expected LC_UUID in {}", cand.display());
                break;
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
