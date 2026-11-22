//! DXR Phase 3: Raytracing state objects (`ID3D12StateObject`), shader identifiers
//! (`ID3D12StateObjectProperties::GetShaderIdentifier`), and `DispatchRays` pipeline assembly
//! (docs/design/DXR.md §5, docs/design/ADVICE_DXR.md §3).
//!
//! * A `COLLECTION` state object holds parsed libraries, hit groups, root signatures, exports,
//!   and subobject associations so it can be referenced via `EXISTING_COLLECTION` or grown via
//!   `AddToStateObject`.
//! * A `RAYTRACING_PIPELINE` state object compiles its DXIL exports and hit groups into Metal
//!   visible functions, synthesizes `RaygenIndirection` + the triangle/procedural intersection
//!   wrappers, statically links them into a single `MTLComputePipelineState`, builds the resident
//!   `MTLVisibleFunctionTable` and `MTLIntersectionFunctionTable`, and populates the 32-byte
//!   `IRShaderIdentifier` map for every shader and hit-group export.
#![allow(non_snake_case)]

use std::collections::{BTreeMap, HashMap};
use std::ffi::c_void;
use std::sync::Arc;

use dxil_metal::compiler::{
    compile_combined_hit_group, compile_uncached, synthesize_indirect_intersection,
    synthesize_indirect_ray_dispatch, CompileOptions, CompileRequest, CompiledShader,
    RayTracingConfig,
};
use dxil_metal::dxbc::{self, Container, ProgramKind};
use dxil_metal::hash::{hash128, KeyHasher};
use dxil_metal::root_signature::RootSignature;
use dxil_metal::runtime::{
    INDIRECT_PROCEDURAL_INTERSECTION_FUNCTION_NAME, INDIRECT_TRIANGLE_INTERSECTION_FUNCTION_NAME,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSArray, NSString};
use objc2_metal::*;

use crate::metal::binding::static_sampler_table;
use crate::metal::pipeline::{function_descriptor, library, pso_cache, MtlComputePipeline, MtlLibrary};

// ---------------------------------------------------------------------------
// D3D12 state-object constants & x64 struct layouts
// ---------------------------------------------------------------------------

/// `D3D12_STATE_OBJECT_TYPE`.
pub const STATE_OBJECT_TYPE_COLLECTION: u32 = 0;
pub const STATE_OBJECT_TYPE_RAYTRACING_PIPELINE: u32 = 3;
pub const STATE_OBJECT_TYPE_EXECUTABLE: u32 = 4;

/// `D3D12_STATE_SUBOBJECT_TYPE`.
pub mod subobject_type {
    pub const STATE_OBJECT_CONFIG: u32 = 0;
    pub const GLOBAL_ROOT_SIGNATURE: u32 = 1;
    pub const LOCAL_ROOT_SIGNATURE: u32 = 2;
    pub const NODE_MASK: u32 = 3;
    pub const DXIL_LIBRARY: u32 = 5;
    pub const EXISTING_COLLECTION: u32 = 6;
    pub const SUBOBJECT_TO_EXPORTS_ASSOCIATION: u32 = 7;
    pub const DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION: u32 = 8;
    pub const RAYTRACING_SHADER_CONFIG: u32 = 9;
    pub const RAYTRACING_PIPELINE_CONFIG: u32 = 10;
    pub const HIT_GROUP: u32 = 11;
    pub const RAYTRACING_PIPELINE_CONFIG1: u32 = 12;
}

/// `D3D12_HIT_GROUP_TYPE`.
pub const HIT_GROUP_TYPE_TRIANGLES: u32 = 0;
pub const HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE: u32 = 1;

/// `D3D12_RAYTRACING_PIPELINE_FLAGS`.
pub const PIPELINE_FLAG_NONE: u32 = 0;
pub const PIPELINE_FLAG_SKIP_TRIANGLES: u32 = 0x100;
pub const PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES: u32 = 0x200;

/// `D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES`.
pub const SHADER_IDENTIFIER_SIZE: usize = 32;

/// `kIRRayDispatchIndirectionKernelName`.
pub const RAY_DISPATCH_INDIRECTION_KERNEL_NAME: &str = "RaygenIndirection";
/// `kIRRayDispatchArgumentsBindPoint`.
pub const RAY_DISPATCH_ARGUMENTS_BIND_POINT: usize = 3;

/// `D3D12_STATE_OBJECT_DESC` (x64: 16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawStateObjectDesc {
    pub Type: u32,
    pub NumSubobjects: u32,
    pub pSubobjects: *const RawStateSubobject,
}

/// `D3D12_STATE_SUBOBJECT` (x64: 16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawStateSubobject {
    pub Type: u32,
    pub _pad: u32,
    pub pDesc: *const c_void,
}

/// `D3D12_EXPORT_DESC` (x64: 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawExportDesc {
    pub Name: *const u16,
    pub ExportToRename: *const u16,
    pub Flags: u32,
    pub _pad: u32,
}

/// `D3D12_DXIL_LIBRARY_DESC` (x64: 32 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawDxilLibraryDesc {
    pub pShaderBytecode: *const u8,
    pub BytecodeLength: usize,
    pub NumExports: u32,
    pub _pad: u32,
    pub pExports: *const RawExportDesc,
}

/// `D3D12_EXISTING_COLLECTION_DESC` (x64: 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawExistingCollectionDesc {
    pub pExistingCollection: *mut c_void,
    pub NumExports: u32,
    pub _pad: u32,
    pub pExports: *const RawExportDesc,
}

/// `D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION` (x64: 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawSubobjectToExportsAssociation {
    pub pSubobjectToAssociate: *const RawStateSubobject,
    pub NumExports: u32,
    pub _pad: u32,
    pub pExports: *const *const u16,
}

/// `D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION` (x64: 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawDxilSubobjectToExportsAssociation {
    pub SubobjectToAssociate: *const u16,
    pub NumExports: u32,
    pub _pad: u32,
    pub pExports: *const *const u16,
}

/// `D3D12_HIT_GROUP_DESC` (x64: 40 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RawHitGroupDesc {
    pub HitGroupExport: *const u16,
    pub Type: u32,
    pub _pad: u32,
    pub AnyHitShaderImport: *const u16,
    pub ClosestHitShaderImport: *const u16,
    pub IntersectionShaderImport: *const u16,
}

/// `D3D12_RAYTRACING_SHADER_CONFIG` (8 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShaderConfig {
    pub MaxPayloadSizeInBytes: u32,
    pub MaxAttributeSizeInBytes: u32,
}

/// `D3D12_RAYTRACING_PIPELINE_CONFIG1` (8 bytes; `CONFIG` has only the first `u32`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PipelineConfig {
    pub MaxTraceRecursionDepth: u32,
    pub Flags: u32,
}

// ---------------------------------------------------------------------------
// DispatchRays & IRShaderIdentifier ABI (`ir_raytracing.h`)
// ---------------------------------------------------------------------------

/// `IRShaderIdentifier` (32 bytes, copied into shader tables by the application via
/// `GetShaderIdentifier`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShaderIdentifier {
    /// VFT index of the fused any-hit / intersection function (0 if none).
    pub intersection_shader_handle: u64,
    /// VFT index of the raygen / miss / callable / closest-hit function (0 if none).
    pub shader_handle: u64,
    /// GPU VA of the local root signature's static sampler table (0 if none).
    pub local_root_signature_samplers_buffer: u64,
    pub pad0: u64,
}

impl ShaderIdentifier {
    pub fn as_bytes(&self) -> &[u8; SHADER_IDENTIFIER_SIZE] {
        // SAFETY: 4 u64 fields = 32 bytes with no padding.
        unsafe { &*(self as *const Self as *const [u8; SHADER_IDENTIFIER_SIZE]) }
    }
}

/// `D3D12_GPU_VIRTUAL_ADDRESS_RANGE` / `IRVirtualAddressRange` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VaRange {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
}

/// `D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE` / `IRVirtualAddressRangeAndStride` (24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VaRangeAndStride {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
    pub StrideInBytes: u64,
}

/// `D3D12_DISPATCH_RAYS_DESC` / `IRDispatchRaysDescriptor` (104 bytes on x64).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DispatchRaysDesc {
    pub RayGenerationShaderRecord: VaRange,
    pub MissShaderTable: VaRangeAndStride,
    pub HitGroupTable: VaRangeAndStride,
    pub CallableShaderTable: VaRangeAndStride,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub _pad: u32,
}

/// `IRDispatchRaysArgument` (152 bytes, bound at `kIRRayDispatchArgumentsBindPoint = 3`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DispatchRaysArgument {
    pub dispatch_rays_desc: DispatchRaysDesc,
    pub grs: u64,
    pub res_desc_heap: u64,
    pub smp_desc_heap: u64,
    pub visible_function_table: u64,
    pub intersection_function_table: u64,
    pub intersection_function_tables: u64,
}

impl DispatchRaysArgument {
    pub fn as_bytes(&self) -> &[u8; std::mem::size_of::<Self>()] {
        // SAFETY: repr(C) struct of u64/u32 fields with explicit padding (152 bytes).
        unsafe { &*(self as *const Self as *const [u8; std::mem::size_of::<Self>()]) }
    }
}

const _: () = {
    assert!(std::mem::size_of::<RawStateObjectDesc>() == 16);
    assert!(std::mem::size_of::<RawStateSubobject>() == 16);
    assert!(std::mem::size_of::<RawExportDesc>() == 24);
    assert!(std::mem::size_of::<RawDxilLibraryDesc>() == 32);
    assert!(std::mem::size_of::<RawExistingCollectionDesc>() == 24);
    assert!(std::mem::size_of::<RawSubobjectToExportsAssociation>() == 24);
    assert!(std::mem::size_of::<RawDxilSubobjectToExportsAssociation>() == 24);
    assert!(std::mem::size_of::<RawHitGroupDesc>() == 40);
    assert!(std::mem::size_of::<ShaderIdentifier>() == SHADER_IDENTIFIER_SIZE);
    assert!(std::mem::size_of::<VaRange>() == 16);
    assert!(std::mem::size_of::<VaRangeAndStride>() == 24);
    assert!(std::mem::size_of::<DispatchRaysDesc>() == 104);
    assert!(std::mem::size_of::<DispatchRaysArgument>() == 152);
};

// ---------------------------------------------------------------------------
// Owned state-object description (parsed from D3D12_STATE_OBJECT_DESC)
// ---------------------------------------------------------------------------

/// One export rename/selection entry (`D3D12_EXPORT_DESC`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportSpec {
    /// External name exposed by the library or collection.
    pub name: String,
    /// Internal name inside the DXIL library or existing collection (`None` = same as `name`).
    pub export_to_rename: Option<String>,
    pub flags: u32,
}

/// One `D3D12_DXIL_LIBRARY_DESC`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxilLibraryDesc {
    pub dxil: Arc<[u8]>,
    /// Empty = export all functions defined in the library's RDAT table.
    pub exports: Vec<ExportSpec>,
}

/// One `D3D12_HIT_GROUP_DESC`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HitGroupDesc {
    pub export: String,
    pub hit_group_type: u32,
    pub any_hit: Option<String>,
    pub closest_hit: Option<String>,
    pub intersection: Option<String>,
}

/// A resolved shader export from a DXIL library.
#[derive(Clone, Debug)]
pub struct ShaderExport {
    /// Exported name in the state object.
    pub export_name: String,
    /// Original unmangled function name inside `dxil`.
    pub target_name: String,
    pub kind: ProgramKind,
    pub dxil: Arc<[u8]>,
}

/// Owned representation of a state object (either a `COLLECTION` or a `RAYTRACING_PIPELINE`).
#[derive(Clone, Default)]
pub struct StateObjectDesc {
    /// `STATE_OBJECT_TYPE_COLLECTION` (0) or `STATE_OBJECT_TYPE_RAYTRACING_PIPELINE` (3).
    pub ty: u32,
    pub flags: u32,
    pub node_mask: u32,
    /// Resolved shader exports in insertion order (`export_name` -> `ShaderExport`).
    pub exports: BTreeMap<String, ShaderExport>,
    /// Resolved hit groups in insertion order (`export` -> `HitGroupDesc`).
    pub hit_groups: BTreeMap<String, HitGroupDesc>,
    /// Global root signature (`D3D12_GLOBAL_ROOT_SIGNATURE`).
    pub global_rs: Option<Arc<RootSignature>>,
    /// Default local root signature (`SUBOBJECT_TO_EXPORTS_ASSOCIATION` with empty exports, or
    /// the sole unassociated `LOCAL_ROOT_SIGNATURE`).
    pub default_local_rs: Option<Arc<RootSignature>>,
    /// Per-export (shader or hit group) local root signature.
    pub export_local_rs: BTreeMap<String, Arc<RootSignature>>,
    pub shader_config: Option<ShaderConfig>,
    pub pipeline_config: Option<PipelineConfig>,
}

unsafe fn read_wstr_opt(p: *const u16) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut n = 0usize;
    while *p.add(n) != 0 {
        n += 1;
    }
    Some(String::from_utf16_lossy(std::slice::from_raw_parts(p, n)))
}

unsafe fn read_exports(num: u32, p: *const RawExportDesc) -> Result<Vec<ExportSpec>, String> {
    if num == 0 {
        return Ok(Vec::new());
    }
    if p.is_null() {
        return Err(format!("NumExports={num} with null pExports"));
    }
    let mut out = Vec::with_capacity(num as usize);
    for i in 0..num as usize {
        let e = &*p.add(i);
        let name = read_wstr_opt(e.Name).ok_or_else(|| format!("export {i} has null Name"))?;
        let export_to_rename = read_wstr_opt(e.ExportToRename);
        out.push(ExportSpec { name, export_to_rename, flags: e.Flags });
    }
    Ok(out)
}

impl StateObjectDesc {
    /// Parse a raw `D3D12_STATE_OBJECT_DESC` (or `pAddition` for `AddToStateObject`), resolving
    /// COM `ID3D12RootSignature*` and `ID3D12StateObject*` pointers through caller callbacks.
    ///
    /// If `base` is `Some`, starts from a clone of `base` (`ID3D12Device7::AddToStateObject`).
    ///
    /// # Safety
    /// `raw` and all nested subobject pointers must be valid x64 D3D12 descriptors.
    pub unsafe fn from_raw(
        raw: *const RawStateObjectDesc,
        base: Option<&StateObjectDesc>,
        resolve_rs: impl Fn(*mut c_void) -> Option<Arc<RootSignature>>,
        resolve_collection: impl Fn(*mut c_void) -> Option<Arc<StateObject>>,
    ) -> Result<StateObjectDesc, String> {
        if raw.is_null() {
            return Err("null D3D12_STATE_OBJECT_DESC".into());
        }
        let d = &*raw;
        if !matches!(
            d.Type,
            STATE_OBJECT_TYPE_COLLECTION | STATE_OBJECT_TYPE_RAYTRACING_PIPELINE | STATE_OBJECT_TYPE_EXECUTABLE
        ) {
            return Err(format!("unsupported D3D12_STATE_OBJECT_TYPE {}", d.Type));
        }
        if d.NumSubobjects > 0 && d.pSubobjects.is_null() {
            return Err(format!("NumSubobjects={} with null pSubobjects", d.NumSubobjects));
        }

        let mut out = match base {
            Some(b) => {
                let mut cloned = b.clone();
                cloned.ty = d.Type;
                cloned
            }
            None => StateObjectDesc { ty: d.Type, node_mask: 1, ..Default::default() },
        };

        let subobjects = if d.NumSubobjects == 0 {
            &[][..]
        } else {
            std::slice::from_raw_parts(d.pSubobjects, d.NumSubobjects as usize)
        };

        // Map subobject address -> parsed local/global root signature or config for
        // `D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION`.
        enum AssocTarget {
            LocalRs(Arc<RootSignature>),
            GlobalRs(Arc<RootSignature>),
            ShaderConfig(ShaderConfig),
            PipelineConfig(PipelineConfig),
            Other,
        }
        let mut by_addr: HashMap<usize, AssocTarget> = HashMap::new();
        let mut local_rs_seen: Vec<Arc<RootSignature>> = Vec::new();
        let mut explicit_local_assoc = false;

        // Pass 1: parse all subobjects except export associations.
        for (idx, sub) in subobjects.iter().enumerate() {
            let sub_addr = sub as *const RawStateSubobject as usize;
            if sub.pDesc.is_null() {
                return Err(format!("subobject {idx} (type {}) has null pDesc", sub.Type));
            }
            match sub.Type {
                subobject_type::STATE_OBJECT_CONFIG => {
                    let flags = *(sub.pDesc as *const u32);
                    out.flags |= flags;
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
                subobject_type::NODE_MASK => {
                    out.node_mask = *(sub.pDesc as *const u32);
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
                subobject_type::GLOBAL_ROOT_SIGNATURE => {
                    let ptr = *(sub.pDesc as *const *mut c_void);
                    if !ptr.is_null() {
                        let rs = resolve_rs(ptr)
                            .ok_or_else(|| format!("subobject {idx}: invalid global root signature {ptr:?}"))?;
                        out.global_rs = Some(rs.clone());
                        by_addr.insert(sub_addr, AssocTarget::GlobalRs(rs));
                    } else {
                        by_addr.insert(sub_addr, AssocTarget::Other);
                    }
                }
                subobject_type::LOCAL_ROOT_SIGNATURE => {
                    let ptr = *(sub.pDesc as *const *mut c_void);
                    if !ptr.is_null() {
                        let rs = resolve_rs(ptr)
                            .ok_or_else(|| format!("subobject {idx}: invalid local root signature {ptr:?}"))?;
                        local_rs_seen.push(rs.clone());
                        by_addr.insert(sub_addr, AssocTarget::LocalRs(rs));
                    } else {
                        by_addr.insert(sub_addr, AssocTarget::Other);
                    }
                }
                subobject_type::DXIL_LIBRARY => {
                    let lib = &*(sub.pDesc as *const RawDxilLibraryDesc);
                    if lib.pShaderBytecode.is_null() || lib.BytecodeLength == 0 {
                        return Err(format!("subobject {idx}: empty DXIL_LIBRARY bytecode"));
                    }
                    let bytes: Arc<[u8]> =
                        std::slice::from_raw_parts(lib.pShaderBytecode, lib.BytecodeLength).into();
                    let exports = read_exports(lib.NumExports, lib.pExports)?;
                    out.add_dxil_library(&DxilLibraryDesc { dxil: bytes, exports })?;
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
                subobject_type::EXISTING_COLLECTION => {
                    let ec = &*(sub.pDesc as *const RawExistingCollectionDesc);
                    let col = resolve_collection(ec.pExistingCollection).ok_or_else(|| {
                        format!("subobject {idx}: invalid existing collection {:?}", ec.pExistingCollection)
                    })?;
                    let exports = read_exports(ec.NumExports, ec.pExports)?;
                    out.merge_collection(&col.desc, &exports)?;
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
                subobject_type::HIT_GROUP => {
                    let hg = &*(sub.pDesc as *const RawHitGroupDesc);
                    let export = read_wstr_opt(hg.HitGroupExport)
                        .ok_or_else(|| format!("subobject {idx}: HIT_GROUP has null HitGroupExport"))?;
                    if hg.Type != HIT_GROUP_TYPE_TRIANGLES && hg.Type != HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE {
                        return Err(format!("subobject {idx}: invalid hit group type {}", hg.Type));
                    }
                    let desc = HitGroupDesc {
                        export: export.clone(),
                        hit_group_type: hg.Type,
                        any_hit: read_wstr_opt(hg.AnyHitShaderImport),
                        closest_hit: read_wstr_opt(hg.ClosestHitShaderImport),
                        intersection: read_wstr_opt(hg.IntersectionShaderImport),
                    };
                    out.hit_groups.insert(export, desc);
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
                subobject_type::RAYTRACING_SHADER_CONFIG => {
                    let cfg = *(sub.pDesc as *const ShaderConfig);
                    out.merge_shader_config(cfg);
                    by_addr.insert(sub_addr, AssocTarget::ShaderConfig(cfg));
                }
                subobject_type::RAYTRACING_PIPELINE_CONFIG => {
                    let depth = *(sub.pDesc as *const u32);
                    let cfg = PipelineConfig {
                        MaxTraceRecursionDepth: depth,
                        Flags: out.pipeline_config.map_or(0, |c| c.Flags),
                    };
                    out.merge_pipeline_config(cfg);
                    by_addr.insert(sub_addr, AssocTarget::PipelineConfig(cfg));
                }
                subobject_type::RAYTRACING_PIPELINE_CONFIG1 => {
                    let cfg = *(sub.pDesc as *const PipelineConfig);
                    out.merge_pipeline_config(cfg);
                    by_addr.insert(sub_addr, AssocTarget::PipelineConfig(cfg));
                }
                subobject_type::SUBOBJECT_TO_EXPORTS_ASSOCIATION
                | subobject_type::DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION => {
                    // Handled in Pass 2 so forward references within `pSubobjects` work.
                }
                other => {
                    log::warn!(target: "d3d", "CreateStateObject: ignoring subobject type {other} at index {idx}");
                    by_addr.insert(sub_addr, AssocTarget::Other);
                }
            }
        }

        // Pass 2: resolve `SUBOBJECT_TO_EXPORTS_ASSOCIATION`.
        for (idx, sub) in subobjects.iter().enumerate() {
            match sub.Type {
                subobject_type::SUBOBJECT_TO_EXPORTS_ASSOCIATION => {
                    let assoc = &*(sub.pDesc as *const RawSubobjectToExportsAssociation);
                    let target_addr = assoc.pSubobjectToAssociate as usize;
                    let Some(target) = by_addr.get(&target_addr) else {
                        log::warn!(
                            target: "d3d",
                            "CreateStateObject: subobject {idx} association points to unknown subobject {:?}",
                            assoc.pSubobjectToAssociate
                        );
                        continue;
                    };
                    let mut names = Vec::with_capacity(assoc.NumExports as usize);
                    if assoc.NumExports > 0 && !assoc.pExports.is_null() {
                        for i in 0..assoc.NumExports as usize {
                            if let Some(s) = read_wstr_opt(*assoc.pExports.add(i)) {
                                names.push(s);
                            }
                        }
                    }
                    match target {
                        AssocTarget::LocalRs(rs) => {
                            explicit_local_assoc = true;
                            if names.is_empty() {
                                out.default_local_rs = Some(rs.clone());
                            } else {
                                for n in names {
                                    out.export_local_rs.insert(n, rs.clone());
                                }
                            }
                        }
                        AssocTarget::GlobalRs(rs) => {
                            out.global_rs = Some(rs.clone());
                        }
                        AssocTarget::ShaderConfig(cfg) => out.merge_shader_config(*cfg),
                        AssocTarget::PipelineConfig(cfg) => out.merge_pipeline_config(*cfg),
                        AssocTarget::Other => {}
                    }
                }
                subobject_type::DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION => {
                    let assoc = &*(sub.pDesc as *const RawDxilSubobjectToExportsAssociation);
                    let sub_name = read_wstr_opt(assoc.SubobjectToAssociate).unwrap_or_default();
                    log::debug!(
                        target: "d3d",
                        "CreateStateObject: DXIL subobject association '{sub_name}' ({} exports)",
                        assoc.NumExports
                    );
                }
                _ => {}
            }
        }

        // If a single `LOCAL_ROOT_SIGNATURE` subobject was provided with no explicit
        // `SUBOBJECT_TO_EXPORTS_ASSOCIATION`, treat it as the default local root signature.
        if !explicit_local_assoc && out.default_local_rs.is_none() && local_rs_seen.len() == 1 {
            out.default_local_rs = local_rs_seen.into_iter().next();
        }

        Ok(out)
    }

    pub fn merge_shader_config(&mut self, cfg: ShaderConfig) {
        let cur = self.shader_config.get_or_insert(cfg);
        cur.MaxPayloadSizeInBytes = cur.MaxPayloadSizeInBytes.max(cfg.MaxPayloadSizeInBytes);
        cur.MaxAttributeSizeInBytes = cur.MaxAttributeSizeInBytes.max(cfg.MaxAttributeSizeInBytes);
    }

    pub fn merge_pipeline_config(&mut self, cfg: PipelineConfig) {
        let cur = self.pipeline_config.get_or_insert(cfg);
        cur.MaxTraceRecursionDepth = cur.MaxTraceRecursionDepth.max(cfg.MaxTraceRecursionDepth);
        cur.Flags |= cfg.Flags;
    }

    /// Add exports from a DXIL library subobject.
    pub fn add_dxil_library(&mut self, lib: &DxilLibraryDesc) -> Result<(), String> {
        let container = Container::parse(&lib.dxil).map_err(|e| format!("DXIL library: {e}"))?;
        let funcs = container.library_functions();

        // If the DXIL library embeds a global root signature (`RTS0`) and none has been set yet,
        // pick it up as a fallback.
        if self.global_rs.is_none() && container.part(dxbc::FOURCC_RTS0).is_some() {
            if let Ok(p) = dxil_metal::ShaderPipeline::global() {
                if let Ok(rs) = p.root_signature(&lib.dxil) {
                    self.global_rs = Some(rs);
                }
            }
        }

        if lib.exports.is_empty() {
            for f in &funcs {
                if is_rt_kind(f.kind) {
                    self.exports.insert(
                        f.unmangled_name.clone(),
                        ShaderExport {
                            export_name: f.unmangled_name.clone(),
                            target_name: f.unmangled_name.clone(),
                            kind: f.kind,
                            dxil: lib.dxil.clone(),
                        },
                    );
                }
            }
        } else {
            for exp in &lib.exports {
                let target = exp.export_to_rename.as_deref().unwrap_or(&exp.name);
                // Match against RDAT unmangled or mangled function name.
                if let Some(f) = funcs.iter().find(|f| f.unmangled_name == target || f.name == target) {
                    if is_rt_kind(f.kind) {
                        self.exports.insert(
                            exp.name.clone(),
                            ShaderExport {
                                export_name: exp.name.clone(),
                                target_name: f.unmangled_name.clone(),
                                kind: f.kind,
                                dxil: lib.dxil.clone(),
                            },
                        );
                    }
                } else if funcs.is_empty() {
                    // Fallback if a container lacks RDAT: assume export exists and let MSC resolve.
                    self.exports.insert(
                        exp.name.clone(),
                        ShaderExport {
                            export_name: exp.name.clone(),
                            target_name: target.to_string(),
                            kind: ProgramKind::RayGeneration,
                            dxil: lib.dxil.clone(),
                        },
                    );
                } else {
                    // Export may name a hit group or subobject defined inside the library; ignore if
                    // not a function export unless no functions matched at all.
                    log::debug!(
                        target: "d3d",
                        "DXIL_LIBRARY: export '{}' (target '{target}') not in RDAT function table",
                        exp.name
                    );
                }
            }
        }
        Ok(())
    }

    /// Merge an existing `COLLECTION` state object into this description.
    pub fn merge_collection(&mut self, col: &StateObjectDesc, exports: &[ExportSpec]) -> Result<(), String> {
        if exports.is_empty() {
            for (k, v) in &col.exports {
                self.exports.insert(k.clone(), v.clone());
            }
            for (k, v) in &col.hit_groups {
                self.hit_groups.insert(k.clone(), v.clone());
            }
            for (k, v) in &col.export_local_rs {
                self.export_local_rs.entry(k.clone()).or_insert_with(|| v.clone());
            }
        } else {
            for exp in exports {
                let src_name = exp.export_to_rename.as_deref().unwrap_or(&exp.name);
                if let Some(se) = col.exports.get(src_name) {
                    let mut cloned = se.clone();
                    cloned.export_name = exp.name.clone();
                    self.exports.insert(exp.name.clone(), cloned);
                    if let Some(lrs) = col.local_rs_for_export(src_name) {
                        self.export_local_rs.insert(exp.name.clone(), lrs.clone());
                    }
                } else if let Some(hg) = col.hit_groups.get(src_name) {
                    let mut cloned = hg.clone();
                    cloned.export = exp.name.clone();
                    for imp in [cloned.closest_hit.as_deref(), cloned.any_hit.as_deref(), cloned.intersection.as_deref()]
                        .into_iter()
                        .flatten()
                    {
                        if let Some(se) = col.exports.get(imp) {
                            self.exports.entry(imp.to_string()).or_insert_with(|| se.clone());
                        }
                    }
                    self.hit_groups.insert(exp.name.clone(), cloned);
                    if let Some(lrs) = col.local_rs_for_export(src_name) {
                        self.export_local_rs.insert(exp.name.clone(), lrs.clone());
                    }
                } else {
                    return Err(format!("existing collection has no export '{src_name}'"));
                }
            }
        }
        if self.global_rs.is_none() {
            self.global_rs = col.global_rs.clone();
        }
        if self.default_local_rs.is_none() {
            self.default_local_rs = col.default_local_rs.clone();
        }
        if let Some(sc) = col.shader_config {
            self.merge_shader_config(sc);
        }
        if let Some(pc) = col.pipeline_config {
            self.merge_pipeline_config(pc);
        }
        Ok(())
    }

    /// Resolve the local root signature associated with `export_name`.
    pub fn local_rs_for_export(&self, export_name: &str) -> Option<&Arc<RootSignature>> {
        self.export_local_rs.get(export_name).or(self.default_local_rs.as_ref())
    }

    /// Resolve the local root signature for a hit group (checks the hit group export name first,
    /// then its constituent shader imports, then the default local root signature).
    pub fn local_rs_for_hit_group(&self, hg: &HitGroupDesc) -> Option<&Arc<RootSignature>> {
        if let Some(rs) = self.export_local_rs.get(&hg.export) {
            return Some(rs);
        }
        for imp in [hg.closest_hit.as_deref(), hg.any_hit.as_deref(), hg.intersection.as_deref()]
            .into_iter()
            .flatten()
        {
            if let Some(rs) = self.export_local_rs.get(imp) {
                return Some(rs);
            }
        }
        self.default_local_rs.as_ref()
    }

    /// Build the `RayTracingConfig` passed to Metal Shader Converter for every shader in this
    /// pipeline. Uses the `All` intrinsic defaults per `ADVICE_DXR.md` §3.2 so `AddToStateObject`
    /// never invalidates previously compiled shaders.
    pub fn ray_tracing_config(&self) -> RayTracingConfig {
        let max_attr = self
            .shader_config
            .map_or(32, |c| c.MaxAttributeSizeInBytes.max(8));
        let (max_depth, flags) = match self.pipeline_config {
            Some(c) => (c.MaxTraceRecursionDepth as i32, c.Flags & 0x300),
            None => (-1, 0),
        };
        RayTracingConfig {
            max_attribute_size_in_bytes: max_attr,
            pipeline_flags: flags,
            closest_hit_intrinsics: 0x7FFF_FFFF,
            miss_intrinsics: 0x7FFF,
            any_hit_intrinsics: u64::MAX,
            callable_intrinsics: 0x703F,
            max_recursive_depth: max_depth,
            // IRRayGenerationCompilationVisibleFunction = 1
            ray_generation_mode: 1,
            // IRIntersectionFunctionCompilationVisibleFunction = 0
            intersection_function_mode: 0,
            intersection_function_groups: false,
            direct_state_access: false,
        }
    }
}

fn is_rt_kind(k: ProgramKind) -> bool {
    matches!(
        k,
        ProgramKind::RayGeneration
            | ProgramKind::Miss
            | ProgramKind::ClosestHit
            | ProgramKind::AnyHit
            | ProgramKind::Intersection
            | ProgramKind::Callable
    )
}

// ---------------------------------------------------------------------------
// Compiled raytracing pipeline & StateObject
// ---------------------------------------------------------------------------

pub type MtlVisibleFunctionTable = ProtocolObject<dyn MTLVisibleFunctionTable>;
pub type MtlIntersectionFunctionTable = ProtocolObject<dyn MTLIntersectionFunctionTable>;

/// Compiled Metal 4 resources for an executable `RAYTRACING_PIPELINE` state object.
pub struct RaytracingPipeline {
    /// Linked compute pipeline with `RaygenIndirection` as the entry kernel and all visible +
    /// intersection wrapper functions statically linked.
    pub pso: Retained<MtlComputePipeline>,
    /// Resident visible function table (index 0 = null, 1..=N = compiled visible functions).
    pub vft: Retained<MtlVisibleFunctionTable>,
    /// Resident intersection function table (index 0 = triangle wrapper, index 1 = procedural wrapper).
    pub ift: Retained<MtlIntersectionFunctionTable>,
    pub vft_gpu_id: u64,
    pub ift_gpu_id: u64,
    /// Kept alive for the lifetime of the pipeline.
    pub libraries: Vec<Retained<MtlLibrary>>,
}
unsafe impl Send for RaytracingPipeline {}
unsafe impl Sync for RaytracingPipeline {}

impl Drop for RaytracingPipeline {
    fn drop(&mut self) {
        if let Some(d) = crate::metal::device() {
            d.remove_resident(ProtocolObject::from_ref(&*self.vft));
            d.remove_resident(ProtocolObject::from_ref(&*self.ift));
        }
    }
}

/// Backing representation of `ID3D12StateObject` and `ID3D12StateObjectProperties1`.
pub struct StateObject {
    pub desc: StateObjectDesc,
    /// `Some` for `D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE`, `None` for `COLLECTION`.
    pub pipeline: Option<RaytracingPipeline>,
    /// 32-byte `IRShaderIdentifier` per export name (raygen, miss, callable, and hit groups).
    pub identifiers: HashMap<String, ShaderIdentifier>,
    /// Pipeline stack size (stored for `GetPipelineStackSize` / `SetPipelineStackSize`).
    pub stack_size: std::sync::atomic::AtomicU64,
}

impl StateObject {
    /// Build a `StateObject` from its parsed description.
    pub fn new(desc: StateObjectDesc) -> Result<Arc<StateObject>, String> {
        if desc.ty == STATE_OBJECT_TYPE_COLLECTION {
            return Ok(Arc::new(StateObject {
                desc,
                pipeline: None,
                identifiers: HashMap::new(),
                stack_size: std::sync::atomic::AtomicU64::new(0),
            }));
        }
        let (pipeline, identifiers) = compile_raytracing_pipeline(&desc)?;
        let default_stack = desc
            .pipeline_config
            .map_or(1, |c| c.MaxTraceRecursionDepth.max(1) as u64)
            * 2048;
        Ok(Arc::new(StateObject {
            desc,
            pipeline: Some(pipeline),
            identifiers,
            stack_size: std::sync::atomic::AtomicU64::new(default_stack),
        }))
    }

    /// Look up the 32-byte `IRShaderIdentifier` for `export_name`.
    pub fn shader_identifier(&self, export_name: &str) -> Option<&ShaderIdentifier> {
        self.identifiers.get(export_name)
    }
}

fn new_mtl_library(bytes: &[u8], label: &str) -> Result<Retained<MtlLibrary>, String> {
    let d = crate::metal::device().ok_or("no Metal device")?;
    let data = dispatch2::DispatchData::from_bytes(bytes);
    let lib = d
        .dev
        .newLibraryWithData_error(&data)
        .map_err(|e| format!("newLibraryWithData({label}): {e}"))?;
    Ok(lib)
}

fn compile_export_visible_fn(
    exp: &ShaderExport,
    global_rs: Option<&RootSignature>,
    local_rs: Option<&RootSignature>,
    rt_cfg: &RayTracingConfig,
    hit_group_type: Option<u32>,
    unique_fn_name: &str,
) -> Result<Arc<CompiledShader>, String> {
    let opts = CompileOptions {
        metal_function_name: Some(unique_fn_name.to_string()),
        ray_tracing: Some(*rt_cfg),
        hit_group_type,
        ..CompileOptions::default()
    };
    let mut req = CompileRequest::new(&exp.dxil, global_rs, &opts).entry_point(&exp.target_name);
    if let Some(lrs) = local_rs {
        req = req.local_root_signature(lrs);
    }
    match dxil_metal::ShaderPipeline::global() {
        Ok(p) => p.compile(&req).map_err(|e| format!("compile export '{}' ({}): {e}", exp.export_name, exp.target_name)),
        Err(_) => compile_uncached(&req)
            .map(Arc::new)
            .map_err(|e| format!("compile export '{}' ({}): {e}", exp.export_name, exp.target_name)),
    }
}

fn compile_hit_group_intersection_fn(
    hg: &HitGroupDesc,
    intersection: Option<&ShaderExport>,
    any_hit: Option<&ShaderExport>,
    global_rs: Option<&RootSignature>,
    local_rs: Option<&RootSignature>,
    rt_cfg: &RayTracingConfig,
    unique_fn_name: &str,
) -> Result<Arc<CompiledShader>, String> {
    match (intersection, any_hit) {
        (Some(isect), Some(ah)) => {
            let opts = CompileOptions {
                metal_function_name: Some(unique_fn_name.to_string()),
                ray_tracing: Some(*rt_cfg),
                hit_group_type: Some(hg.hit_group_type),
                ..CompileOptions::default()
            };
            compile_combined_hit_group(
                Some((&isect.dxil, isect.target_name.as_str())),
                Some((&ah.dxil, ah.target_name.as_str())),
                global_rs,
                local_rs,
                &opts,
            )
            .map(Arc::new)
            .map_err(|e| format!("compile combined hit group '{}': {e}", hg.export))
        }
        (Some(exp), None) | (None, Some(exp)) => compile_export_visible_fn(
            exp,
            global_rs,
            local_rs,
            rt_cfg,
            Some(hg.hit_group_type),
            unique_fn_name,
        ),
        (None, None) => Err(format!("hit group '{}' has neither intersection nor any-hit shader", hg.export)),
    }
}

fn local_samplers_va(local_rs: Option<&Arc<RootSignature>>) -> Result<u64, String> {
    match local_rs {
        Some(rs) => Ok(static_sampler_table(rs)?.unwrap_or(0)),
        None => Ok(0),
    }
}

fn compile_raytracing_pipeline(
    desc: &StateObjectDesc,
) -> Result<(RaytracingPipeline, HashMap<String, ShaderIdentifier>), String> {
    let d = crate::metal::device().ok_or("no Metal device")?;
    let rt_cfg = desc.ray_tracing_config();
    let global_rs = desc.global_rs.as_deref();

    let mut pso_key = KeyHasher::new("d3d12-metal rt pipeline v1");
    let base_opts = CompileOptions { ray_tracing: Some(rt_cfg), ..CompileOptions::default() };

    // 1. Synthesize the `RaygenIndirection` compute kernel and the two indirect intersection
    //    wrapper functions (triangle = slot 0, procedural = slot 1).
    let indirection_bytes = synthesize_indirect_ray_dispatch(&base_opts)
        .map_err(|e| format!("synthesize_indirect_ray_dispatch: {e}"))?;
    pso_key.u128(hash128(&indirection_bytes));
    let indirection_lib = new_mtl_library(&indirection_bytes, "RaygenIndirection")?;

    let tri_opts = CompileOptions { hit_group_type: Some(HIT_GROUP_TYPE_TRIANGLES), ..base_opts.clone() };
    let tri_bytes = synthesize_indirect_intersection(&tri_opts)
        .map_err(|e| format!("synthesize_indirect_intersection(triangle): {e}"))?;
    pso_key.u128(hash128(&tri_bytes));
    let tri_lib = new_mtl_library(&tri_bytes, "IndirectTriangleIntersection")?;

    let proc_opts = CompileOptions { hit_group_type: Some(HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE), ..base_opts };
    let proc_bytes = synthesize_indirect_intersection(&proc_opts)
        .map_err(|e| format!("synthesize_indirect_intersection(procedural): {e}"))?;
    pso_key.u128(hash128(&proc_bytes));
    let proc_lib = new_mtl_library(&proc_bytes, "IndirectProceduralIntersection")?;

    let mut libraries: Vec<Retained<MtlLibrary>> = vec![indirection_lib.clone(), tri_lib.clone(), proc_lib.clone()];

    // Visible functions: 1-based VFT slot -> (MTLLibrary, function_name).
    // Slot 0 is reserved for the null handle.
    let mut vft_entries: Vec<(Retained<MtlLibrary>, String)> = Vec::new();
    let mut identifiers: HashMap<String, ShaderIdentifier> = HashMap::new();

    // Cache `(export_name, local_rs_hash, hit_group_type) -> vft_index` so multiple hit groups
    // sharing the same closest-hit shader and local root signature share the same VFT slot.
    let mut compiled_export_slots: HashMap<(String, u128, Option<u32>), u64> = HashMap::new();

    // 2. Compile standalone raygen, miss, and callable exports.
    //    (Closest-hit, any-hit, and intersection shaders are compiled per hit group in step 3,
    //    where hit-group local root signature associations and hit group types are known.)
    for (export_name, exp) in &desc.exports {
        if !matches!(
            exp.kind,
            ProgramKind::RayGeneration | ProgramKind::Miss | ProgramKind::Callable
        ) {
            continue;
        }
        let local_rs = desc.local_rs_for_export(export_name);
        let lrs_hash = local_rs.map_or(0, |r| r.hash());
        let slot = if let Some(&s) = compiled_export_slots.get(&(export_name.clone(), lrs_hash, None)) {
            s
        } else {
            let vft_slot = (vft_entries.len() + 1) as u64;
            let fn_name = format!("aw2_rt_fn_{vft_slot}");
            let compiled = compile_export_visible_fn(
                exp,
                global_rs,
                local_rs.map(|r| r.as_ref()),
                &rt_cfg,
                None,
                &fn_name,
            )?;
            pso_key.u128(compiled.key);
            let lib = library(&compiled)?;
            libraries.push(lib.clone());
            vft_entries.push((lib, compiled.function_name.clone()));
            compiled_export_slots.insert((export_name.clone(), lrs_hash, None), vft_slot);
            vft_slot
        };

        let samplers_va = local_samplers_va(local_rs)?;
        identifiers.insert(
            export_name.clone(),
            ShaderIdentifier {
                intersection_shader_handle: 0,
                shader_handle: slot,
                local_root_signature_samplers_buffer: samplers_va,
                pad0: 0,
            },
        );
    }

    // 3. Compile hit groups (closest-hit into `shader_handle`, fused any-hit/intersection into
    //    `intersection_shader_handle`).
    for (hg_name, hg) in &desc.hit_groups {
        let local_rs = desc.local_rs_for_hit_group(hg);
        let lrs_hash = local_rs.map_or(0, |r| r.hash());

        let closest_hit_slot = match &hg.closest_hit {
            Some(ch_name) if !ch_name.is_empty() => {
                let exp = desc
                    .exports
                    .get(ch_name)
                    .ok_or_else(|| format!("hit group '{hg_name}': unknown closest-hit import '{ch_name}'"))?;
                let key = (ch_name.clone(), lrs_hash, Some(hg.hit_group_type));
                if let Some(&s) = compiled_export_slots.get(&key) {
                    s
                } else {
                    let vft_slot = (vft_entries.len() + 1) as u64;
                    let fn_name = format!("aw2_rt_fn_{vft_slot}");
                    let compiled = compile_export_visible_fn(
                        exp,
                        global_rs,
                        local_rs.map(|r| r.as_ref()),
                        &rt_cfg,
                        Some(hg.hit_group_type),
                        &fn_name,
                    )?;
                    pso_key.u128(compiled.key);
                    let lib = library(&compiled)?;
                    libraries.push(lib.clone());
                    vft_entries.push((lib, compiled.function_name.clone()));
                    compiled_export_slots.insert(key, vft_slot);
                    vft_slot
                }
            }
            _ => 0,
        };

        let ah_exp = match &hg.any_hit {
            Some(n) if !n.is_empty() => Some(
                desc.exports
                    .get(n)
                    .ok_or_else(|| format!("hit group '{hg_name}': unknown any-hit import '{n}'"))?,
            ),
            _ => None,
        };
        let isect_exp = match &hg.intersection {
            Some(n) if !n.is_empty() => Some(
                desc.exports
                    .get(n)
                    .ok_or_else(|| format!("hit group '{hg_name}': unknown intersection import '{n}'"))?,
            ),
            _ => None,
        };

        let intersection_slot = if ah_exp.is_some() || isect_exp.is_some() {
            let vft_slot = (vft_entries.len() + 1) as u64;
            let fn_name = format!("aw2_rt_fn_{vft_slot}");
            let compiled = compile_hit_group_intersection_fn(
                hg,
                isect_exp,
                ah_exp,
                global_rs,
                local_rs.map(|r| r.as_ref()),
                &rt_cfg,
                &fn_name,
            )?;
            pso_key.u128(compiled.key);
            let lib = library(&compiled)?;
            libraries.push(lib.clone());
            vft_entries.push((lib, compiled.function_name.clone()));
            vft_slot
        } else {
            0
        };

        let samplers_va = local_samplers_va(local_rs)?;
        identifiers.insert(
            hg_name.clone(),
            ShaderIdentifier {
                intersection_shader_handle: intersection_slot,
                shader_handle: closest_hit_slot,
                local_root_signature_samplers_buffer: samplers_va,
                pad0: 0,
            },
        );
    }

    // 4. Statically link `RaygenIndirection` + all visible functions + both intersection wrappers
    //    into an `MTLComputePipelineState`.
    let mut linked_descs: Vec<Retained<MTL4FunctionDescriptor>> = Vec::with_capacity(vft_entries.len() + 2);
    linked_descs.push(Retained::into_super(function_descriptor(
        &tri_lib,
        INDIRECT_TRIANGLE_INTERSECTION_FUNCTION_NAME,
    )));
    linked_descs.push(Retained::into_super(function_descriptor(
        &proc_lib,
        INDIRECT_PROCEDURAL_INTERSECTION_FUNCTION_NAME,
    )));
    for (lib, fn_name) in &vft_entries {
        linked_descs.push(Retained::into_super(function_descriptor(lib, fn_name)));
    }

    let linking = MTL4StaticLinkingDescriptor::new();
    let linked_array = NSArray::from_retained_slice(&linked_descs);
    linking.setFunctionDescriptors(Some(&linked_array));

    let pso_desc = MTL4ComputePipelineDescriptor::new();
    pso_desc.setComputeFunctionDescriptor(Some(&function_descriptor(
        &indirection_lib,
        RAY_DISPATCH_INDIRECTION_KERNEL_NAME,
    )));
    pso_desc.setStaticLinkingDescriptor(Some(&linking));
    pso_desc.setLabel(Some(&NSString::from_str("aw2 raytracing pipeline")));

    let pso = pso_cache()?
        .compute_pipeline(Some(pso_key.finish()), &pso_desc)
        .map_err(|e| format!("raytracing compute pipeline: {e}"))?;

    // 5. Build and populate the `MTLVisibleFunctionTable` (slot 0 = null, slots 1..=N = functions).
    let vft_desc = MTLVisibleFunctionTableDescriptor::new();
    unsafe { vft_desc.setFunctionCount(vft_entries.len() + 1) };
    let vft = pso
        .newVisibleFunctionTableWithDescriptor(&vft_desc)
        .ok_or("newVisibleFunctionTableWithDescriptor failed")?;
    for (i, (_lib, fn_name)) in vft_entries.iter().enumerate() {
        let slot = i + 1;
        let handle = pso
            .functionHandleWithName(&NSString::from_str(fn_name))
            .ok_or_else(|| format!("functionHandleWithName({fn_name}) returned nil"))?;
        unsafe { vft.setFunction_atIndex(Some(&handle), slot) };
    }

    // 6. Build and populate the `MTLIntersectionFunctionTable` (slot 0 = triangle, slot 1 = procedural).
    let ift_desc = MTLIntersectionFunctionTableDescriptor::new();
    ift_desc.setFunctionCount(2);
    let ift = pso
        .newIntersectionFunctionTableWithDescriptor(&ift_desc)
        .ok_or("newIntersectionFunctionTableWithDescriptor failed")?;
    let tri_handle = pso
        .functionHandleWithName(&NSString::from_str(INDIRECT_TRIANGLE_INTERSECTION_FUNCTION_NAME))
        .ok_or("functionHandleWithName(triangle wrapper) returned nil")?;
    let proc_handle = pso
        .functionHandleWithName(&NSString::from_str(INDIRECT_PROCEDURAL_INTERSECTION_FUNCTION_NAME))
        .ok_or("functionHandleWithName(procedural wrapper) returned nil")?;
    ift.setFunction_atIndex(Some(&tri_handle), 0);
    ift.setFunction_atIndex(Some(&proc_handle), 1);

    // Make VFT and IFT resident on the device.
    d.add_resident(ProtocolObject::from_ref(&*vft));
    d.add_resident(ProtocolObject::from_ref(&*ift));

    let vft_gpu_id: u64 = unsafe { std::mem::transmute::<MTLResourceID, u64>(vft.gpuResourceID()) };
    let ift_gpu_id: u64 = unsafe { std::mem::transmute::<MTLResourceID, u64>(ift.gpuResourceID()) };

    Ok((
        RaytracingPipeline {
            pso,
            vft,
            ift,
            vft_gpu_id,
            ift_gpu_id,
            libraries,
        },
        identifiers,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn shader_identifier_and_dispatch_rays_layouts_match_msc() {
        assert_eq!(std::mem::size_of::<ShaderIdentifier>(), 32);
        assert_eq!(std::mem::size_of::<DispatchRaysDesc>(), 104);
        assert_eq!(std::mem::size_of::<DispatchRaysArgument>(), 152);

        let id = ShaderIdentifier {
            intersection_shader_handle: 3,
            shader_handle: 7,
            local_root_signature_samplers_buffer: 0x1234_5678,
            pad0: 0,
        };
        let b = id.as_bytes();
        assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 3);
        assert_eq!(u64::from_le_bytes(b[8..16].try_into().unwrap()), 7);
        assert_eq!(u64::from_le_bytes(b[16..24].try_into().unwrap()), 0x1234_5678);
    }

    #[test]
    fn parses_collection_and_associations() {
        let hg_name = wide("HitGroup0");
        let ch_name = wide("ClosestHit0");
        let ah_name = wide("AnyHit0");

        let hg_raw = RawHitGroupDesc {
            HitGroupExport: hg_name.as_ptr(),
            Type: HIT_GROUP_TYPE_TRIANGLES,
            _pad: 0,
            AnyHitShaderImport: ah_name.as_ptr(),
            ClosestHitShaderImport: ch_name.as_ptr(),
            IntersectionShaderImport: std::ptr::null(),
        };
        let sc_raw = ShaderConfig { MaxPayloadSizeInBytes: 16, MaxAttributeSizeInBytes: 8 };
        let pc_raw = PipelineConfig { MaxTraceRecursionDepth: 2, Flags: PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES };

        let dummy_lrs = RootSignature::new_deferred(dxil_metal::RootSignatureDesc::covering(&[]));
        let lrs_ptr: *mut c_void = 0xDEAD_BEEFusize as *mut c_void;
        let export_ptrs = [hg_name.as_ptr()];

        let mut subs = [
            RawStateSubobject {
                Type: subobject_type::HIT_GROUP,
                _pad: 0,
                pDesc: (&hg_raw as *const RawHitGroupDesc).cast(),
            },
            RawStateSubobject {
                Type: subobject_type::RAYTRACING_SHADER_CONFIG,
                _pad: 0,
                pDesc: (&sc_raw as *const ShaderConfig).cast(),
            },
            RawStateSubobject {
                Type: subobject_type::RAYTRACING_PIPELINE_CONFIG1,
                _pad: 0,
                pDesc: (&pc_raw as *const PipelineConfig).cast(),
            },
            RawStateSubobject {
                Type: subobject_type::LOCAL_ROOT_SIGNATURE,
                _pad: 0,
                pDesc: (&lrs_ptr as *const *mut c_void).cast(),
            },
            RawStateSubobject {
                Type: subobject_type::SUBOBJECT_TO_EXPORTS_ASSOCIATION,
                _pad: 0,
                pDesc: std::ptr::null(),
            },
        ];
        let assoc_raw = RawSubobjectToExportsAssociation {
            pSubobjectToAssociate: &subs[3],
            NumExports: 1,
            _pad: 0,
            pExports: export_ptrs.as_ptr(),
        };
        subs[4].pDesc = (&assoc_raw as *const RawSubobjectToExportsAssociation).cast();

        let desc_raw = RawStateObjectDesc {
            Type: STATE_OBJECT_TYPE_COLLECTION,
            NumSubobjects: subs.len() as u32,
            pSubobjects: subs.as_ptr(),
        };

        let parsed = unsafe {
            StateObjectDesc::from_raw(
                &desc_raw,
                None,
                |p| (p == lrs_ptr).then(|| dummy_lrs.clone()),
                |_| None,
            )
        }
        .unwrap();

        assert_eq!(parsed.ty, STATE_OBJECT_TYPE_COLLECTION);
        assert_eq!(parsed.hit_groups.len(), 1);
        let hg = &parsed.hit_groups["HitGroup0"];
        assert_eq!(hg.closest_hit.as_deref(), Some("ClosestHit0"));
        assert_eq!(hg.any_hit.as_deref(), Some("AnyHit0"));
        assert!(parsed.local_rs_for_hit_group(hg).is_some());
        assert_eq!(parsed.shader_config, Some(sc_raw));
        assert_eq!(parsed.pipeline_config, Some(pc_raw));
    }
}
