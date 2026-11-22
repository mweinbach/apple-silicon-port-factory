//! Shader reflection as plain Rust data (extracted from `IRShaderReflection`).

use crate::ffi::{self, *};
use crate::root_signature::{read_locations, ResourceLocation};
use crate::Error;

/// `IRShaderStage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShaderStage {
    Invalid = 0,
    Vertex,
    Fragment,
    Hull,
    Domain,
    Mesh,
    Amplification,
    Geometry,
    Compute,
    ClosestHit,
    Intersection,
    AnyHit,
    Miss,
    RayGeneration,
    Callable,
    StreamOut,
    StageIn,
}

impl ShaderStage {
    pub fn from_raw(v: u32) -> Self {
        use ShaderStage::*;
        const ALL: [ShaderStage; 17] = [
            Invalid,
            Vertex,
            Fragment,
            Hull,
            Domain,
            Mesh,
            Amplification,
            Geometry,
            Compute,
            ClosestHit,
            Intersection,
            AnyHit,
            Miss,
            RayGeneration,
            Callable,
            StreamOut,
            StageIn,
        ];
        ALL.get(v as usize).copied().unwrap_or(Invalid)
    }
    pub fn to_raw(self) -> u32 {
        self as u32
    }
    pub fn is_ray_tracing(self) -> bool {
        matches!(
            self,
            Self::ClosestHit | Self::Intersection | Self::AnyHit | Self::Miss | Self::RayGeneration | Self::Callable
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexAttribute {
    /// Semantic name + index as the converter reports it (e.g. `POSITION0`).
    pub name: String,
    /// Metal attribute index relative to [`crate::runtime::STAGE_IN_ATTRIBUTE_START_INDEX`].
    pub attribute_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionConstant {
    pub name: String,
    /// `IRFunctionConstantType` (Bool 53, Int 29, Int4 32, Float 3).
    pub ty: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StageInfo {
    None,
    Compute {
        threadgroup_size: [u32; 3],
    },
    Vertex {
        instance_id_index: i32,
        vertex_id_index: i32,
        vertex_output_size_in_bytes: u32,
        needs_draw_params: bool,
        inputs: Vec<VertexAttribute>,
    },
    Fragment {
        num_render_targets: i32,
        rt_index_int: u8,
        discards: bool,
    },
    Geometry {
        outputs: Vec<VertexAttribute>,
        is_passthrough: bool,
        rt_array_index_record_id: i32,
        viewport_array_index_record_id: i32,
        input_primitive: u32,
        max_input_primitives_per_mesh_threadgroup: u32,
        max_payload_size_in_bytes: u32,
        instance_count: u32,
    },
    Hull {
        max_patches_per_object_threadgroup: u32,
        max_object_threads_per_patch: u32,
        patch_constants_size: u32,
        patch_constant_function: String,
        static_payload_size: u32,
        payload_size_per_patch: u32,
        input_control_point_count: u32,
        output_control_point_count: u32,
        output_control_point_size: u32,
        tessellator_domain: u32,
        tessellator_partitioning: u32,
        tessellator_output_primitive: u32,
        tessellation_type_half: bool,
        max_tessellation_factor: f32,
    },
    Domain {
        tessellator_domain: u32,
        max_input_prims_per_mesh_threadgroup: u32,
        input_control_point_count: u32,
        input_control_point_size: u32,
        patch_constants_size: u32,
        tessellation_type_half: bool,
    },
    Mesh {
        max_vertex_output_count: u32,
        max_primitive_output_count: u32,
        /// `IRMeshShaderPrimitiveTopology` (0 point, 1 line, 2 triangle, 3 undefined).
        primitive_topology: u32,
        max_payload_size_in_bytes: u32,
        threadgroup_size: [u32; 3],
    },
    Amplification {
        threadgroup_size: [u32; 3],
        max_payload_size_in_bytes: u32,
    },
    RayTracing {
        is_indirect_intersection_function: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaderReflection {
    pub stage: ShaderStage,
    /// Name of the Metal entry point in the metallib.
    pub function_name: String,
    /// Resources and their top-level argument buffer offsets.
    pub resources: Vec<ResourceLocation>,
    pub needs_function_constants: bool,
    pub function_constants: Vec<FunctionConstant>,
    pub info: StageInfo,
    /// The converter's JSON reflection (persisted in the cache; recreates an `IRShaderReflection`
    /// for stage-in synthesis).
    pub json: String,
}

/// Owned `IRShaderReflection*`.
pub(crate) struct IrReflection(pub *mut IRShaderReflection);

impl Drop for IrReflection {
    fn drop(&mut self) {
        if let Ok(api) = ffi::api() {
            unsafe { (api.IRShaderReflectionDestroy)(self.0) };
        }
    }
}

impl IrReflection {
    pub fn from_json(api: &Api, json: &str) -> Result<Self, Error> {
        let c = std::ffi::CString::new(json).map_err(|e| Error::InvalidInput(e.to_string()))?;
        let p = unsafe { (api.IRShaderReflectionCreateFromJSON)(c.as_ptr()) };
        if p.is_null() {
            return Err(Error::InvalidInput("IRShaderReflectionCreateFromJSON failed".into()));
        }
        Ok(Self(p))
    }
}

fn attrs(ptr: *const IRVertexInputInfo_1_0, n: usize) -> Vec<VertexAttribute> {
    if ptr.is_null() {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(ptr, n) }
        .iter()
        .map(|a| VertexAttribute { name: unsafe { cstr(a.name) }, attribute_index: a.attributeIndex })
        .collect()
}

macro_rules! copy_info {
    ($api:ident, $refl:ident, $copy:ident, $release:ident, $ty:ty, |$i:ident| $body:expr) => {{
        let mut v: Versioned<$ty> = std::mem::zeroed();
        if ($api.$copy)($refl, IRReflectionVersion_1_0, &mut v) {
            let $i = v.info;
            let out = $body;
            ($api.$release)(&mut v);
            out
        } else {
            StageInfo::None
        }
    }};
}

impl ShaderReflection {
    /// Extracts everything from a converter reflection object.
    pub(crate) unsafe fn from_ir(api: &Api, refl: *const IRShaderReflection, stage: ShaderStage) -> Self {
        let function_name = cstr((api.IRShaderReflectionGetEntryPointFunctionName)(refl));
        let n = (api.IRShaderReflectionGetResourceCount)(refl);
        let mut locs = vec![std::mem::zeroed::<IRResourceLocation>(); n];
        if n > 0 {
            (api.IRShaderReflectionGetResourceLocations)(refl, locs.as_mut_ptr());
        }
        let resources = read_locations(&locs);
        let needs_function_constants = (api.IRShaderReflectionNeedsFunctionConstants)(refl);
        let fc_n = (api.IRShaderReflectionGetFunctionConstantCount)(refl);
        let mut function_constants = Vec::new();
        if fc_n > 0 {
            let mut fcs = vec![IRFunctionConstant { name: std::ptr::null(), r#type: 0 }; fc_n];
            (api.IRShaderReflectionCopyFunctionConstants)(refl, fcs.as_mut_ptr());
            function_constants = fcs.iter().map(|f| FunctionConstant { name: cstr(f.name), ty: f.r#type }).collect();
            (api.IRShaderReflectionReleaseFunctionConstants)(fcs.as_mut_ptr(), fc_n);
        }
        let info = match stage {
            ShaderStage::Compute => copy_info!(api, refl, IRShaderReflectionCopyComputeInfo, IRShaderReflectionReleaseComputeInfo, IRCSInfo_1_0, |i| {
                StageInfo::Compute { threadgroup_size: i.tg_size }
            }),
            ShaderStage::Vertex => copy_info!(api, refl, IRShaderReflectionCopyVertexInfo, IRShaderReflectionReleaseVertexInfo, IRVSInfo_1_0, |i| {
                StageInfo::Vertex {
                    instance_id_index: i.instance_id_index,
                    vertex_id_index: i.vertex_id_index,
                    vertex_output_size_in_bytes: i.vertex_output_size_in_bytes,
                    needs_draw_params: i.needs_draw_params,
                    inputs: attrs(i.vertex_inputs, i.num_vertex_inputs),
                }
            }),
            ShaderStage::Fragment => copy_info!(api, refl, IRShaderReflectionCopyFragmentInfo, IRShaderReflectionReleaseFragmentInfo, IRFSInfo_1_0, |i| {
                StageInfo::Fragment { num_render_targets: i.num_render_targets, rt_index_int: i.rt_index_int, discards: i.discards }
            }),
            ShaderStage::Geometry => copy_info!(api, refl, IRShaderReflectionCopyGeometryInfo, IRShaderReflectionReleaseGeometryInfo, IRGSInfo_1_0, |i| {
                StageInfo::Geometry {
                    outputs: attrs(i.vertex_outputs.cast(), i.num_vertex_outputs),
                    is_passthrough: i.is_passthrough,
                    rt_array_index_record_id: i.rt_array_index_record_id,
                    viewport_array_index_record_id: i.viewport_array_index_record_id,
                    input_primitive: i.input_primitive,
                    max_input_primitives_per_mesh_threadgroup: i.max_input_primitives_per_mesh_threadgroup,
                    max_payload_size_in_bytes: i.max_payload_size_in_bytes,
                    instance_count: i.instance_count,
                }
            }),
            ShaderStage::Hull => copy_info!(api, refl, IRShaderReflectionCopyHullInfo, IRShaderReflectionReleaseHullInfo, IRHSInfo_1_0, |i| {
                StageInfo::Hull {
                    max_patches_per_object_threadgroup: i.max_patches_per_object_threadgroup,
                    max_object_threads_per_patch: i.max_object_threads_per_patch,
                    patch_constants_size: i.patch_constants_size,
                    patch_constant_function: cstr(i.patch_constant_function),
                    static_payload_size: i.static_payload_size,
                    payload_size_per_patch: i.payload_size_per_patch,
                    input_control_point_count: i.input_control_point_count,
                    output_control_point_count: i.output_control_point_count,
                    output_control_point_size: i.output_control_point_size,
                    tessellator_domain: i.tessellator_domain,
                    tessellator_partitioning: i.tessellator_partitioning,
                    tessellator_output_primitive: i.tessellator_output_primitive,
                    tessellation_type_half: i.tessellation_type_half,
                    max_tessellation_factor: i.max_tessellation_factor,
                }
            }),
            ShaderStage::Domain => copy_info!(api, refl, IRShaderReflectionCopyDomainInfo, IRShaderReflectionReleaseDomainInfo, IRDSInfo_1_0, |i| {
                StageInfo::Domain {
                    tessellator_domain: i.tessellator_domain,
                    max_input_prims_per_mesh_threadgroup: i.max_input_prims_per_mesh_threadgroup,
                    input_control_point_count: i.input_control_point_count,
                    input_control_point_size: i.input_control_point_size,
                    patch_constants_size: i.patch_constants_size,
                    tessellation_type_half: i.tessellation_type_half,
                }
            }),
            ShaderStage::Mesh => copy_info!(api, refl, IRShaderReflectionCopyMeshInfo, IRShaderReflectionReleaseMeshInfo, IRMSInfo_1_0, |i| {
                StageInfo::Mesh {
                    max_vertex_output_count: i.max_vertex_output_count,
                    max_primitive_output_count: i.max_primitive_output_count,
                    primitive_topology: i.primitive_topology,
                    max_payload_size_in_bytes: i.max_payload_size_in_bytes,
                    threadgroup_size: i.num_threads,
                }
            }),
            ShaderStage::Amplification => copy_info!(api, refl, IRShaderReflectionCopyAmplificationInfo, IRShaderReflectionReleaseAmplificationInfo, IRASInfo_1_0, |i| {
                StageInfo::Amplification { threadgroup_size: i.num_threads, max_payload_size_in_bytes: i.max_payload_size_in_bytes }
            }),
            s if s.is_ray_tracing() => copy_info!(api, refl, IRShaderReflectionCopyRaytracingInfo, IRShaderReflectionReleaseRaytracingInfo, IRRTInfo_1_0, |i| {
                StageInfo::RayTracing { is_indirect_intersection_function: i.is_indirect_intersection_function }
            }),
            _ => StageInfo::None,
        };
        let js = (api.IRShaderReflectionCopyJSONString)(refl);
        let json = cstr(js);
        if !js.is_null() {
            (api.IRShaderReflectionReleaseString)(js);
        }
        Self { stage, function_name, resources, needs_function_constants, function_constants, info, json }
    }

    /// Rebuilds reflection from persisted converter JSON.
    pub fn from_json(json: &str, stage: ShaderStage) -> Result<Self, Error> {
        let api = ffi::api()?;
        let r = IrReflection::from_json(api, json)?;
        let mut out = unsafe { Self::from_ir(api, r.0, stage) };
        // Keep the original text so repeated round trips are byte-stable.
        out.json = json.to_string();
        Ok(out)
    }

    /// Threadgroup size for compute / mesh / amplification stages.
    pub fn threadgroup_size(&self) -> Option<[u32; 3]> {
        match self.info {
            StageInfo::Compute { threadgroup_size }
            | StageInfo::Mesh { threadgroup_size, .. }
            | StageInfo::Amplification { threadgroup_size, .. } => Some(threadgroup_size),
            _ => None,
        }
    }
}
