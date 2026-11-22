//! Compact little-endian binary encoding of compile results, options and errors, shared by
//! the disk cache entry format and the `dxil-metald` helper protocol. Decoding is pure Rust
//! (no converter needed), so cache hits and helper replies never touch the converter
//! in-process.

use crate::compiler::{CompileOptions, CompiledShader, GpuFamily, InputElement, InputTopology, RayTracingConfig, StageInMode, Tristate};
use crate::reflection::{FunctionConstant, ShaderReflection, ShaderStage, StageInfo, VertexAttribute};
use crate::root_signature::{ErrorCode, ResourceLocation, ResourceType};
use crate::Error;

#[derive(Default)]
pub(crate) struct Writer {
    pub buf: Vec<u8>,
}

impl Writer {
    pub fn with_capacity(n: usize) -> Self {
        Self { buf: Vec::with_capacity(n) }
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.u8(v as u8)
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.u32(v as u32)
    }
    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.u32(v.to_bits())
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u128(&mut self, v: u128) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.u64(b.len() as u64);
        self.buf.extend_from_slice(b);
        self
    }
    pub fn str(&mut self, s: &str) -> &mut Self {
        self.bytes(s.as_bytes())
    }
    pub fn opt_u32(&mut self, v: Option<u32>) -> &mut Self {
        match v {
            Some(v) => self.u8(1).u32(v),
            None => self.u8(0),
        }
    }
    pub fn opt_str(&mut self, v: Option<&str>) -> &mut Self {
        match v {
            Some(v) => self.u8(1).str(v),
            None => self.u8(0),
        }
    }
    pub fn opt_bytes(&mut self, v: Option<&[u8]>) -> &mut Self {
        match v {
            Some(v) => self.u8(1).bytes(v),
            None => self.u8(0),
        }
    }
}

pub(crate) struct Reader<'a> {
    b: &'a [u8],
    pos: usize,
}

fn bad(what: &str) -> Error {
    Error::Io(format!("malformed encoding: {what}"))
}

impl<'a> Reader<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Self { b, pos: 0 }
    }
    pub fn remaining(&self) -> usize {
        self.b.len() - self.pos
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(n).filter(|&e| e <= self.b.len()).ok_or_else(|| bad("truncated"))?;
        let s = &self.b[self.pos..end];
        self.pos = end;
        Ok(s)
    }
    pub fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
    pub fn bool(&mut self) -> Result<bool, Error> {
        Ok(self.u8()? != 0)
    }
    pub fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> Result<i32, Error> {
        Ok(self.u32()? as i32)
    }
    pub fn f32(&mut self) -> Result<f32, Error> {
        Ok(f32::from_bits(self.u32()?))
    }
    pub fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub fn u128(&mut self) -> Result<u128, Error> {
        Ok(u128::from_le_bytes(self.take(16)?.try_into().unwrap()))
    }
    pub fn bytes(&mut self) -> Result<&'a [u8], Error> {
        let n = usize::try_from(self.u64()?).map_err(|_| bad("length"))?;
        self.take(n)
    }
    pub fn string(&mut self) -> Result<String, Error> {
        String::from_utf8(self.bytes()?.to_vec()).map_err(|_| bad("utf-8"))
    }
    pub fn opt_u32(&mut self) -> Result<Option<u32>, Error> {
        Ok(if self.bool()? { Some(self.u32()?) } else { None })
    }
    pub fn opt_string(&mut self) -> Result<Option<String>, Error> {
        Ok(if self.bool()? { Some(self.string()?) } else { None })
    }
    pub fn opt_bytes(&mut self) -> Result<Option<&'a [u8]>, Error> {
        Ok(if self.bool()? { Some(self.bytes()?) } else { None })
    }
    fn u32x3(&mut self) -> Result<[u32; 3], Error> {
        Ok([self.u32()?, self.u32()?, self.u32()?])
    }
}

// ---- reflection / compiled shader -----------------------------------------------------------

fn put_attrs(w: &mut Writer, a: &[VertexAttribute]) {
    w.u32(a.len() as u32);
    for a in a {
        w.str(&a.name).u8(a.attribute_index);
    }
}

fn get_attrs(r: &mut Reader) -> Result<Vec<VertexAttribute>, Error> {
    let n = r.u32()?;
    (0..n).map(|_| Ok(VertexAttribute { name: r.string()?, attribute_index: r.u8()? })).collect()
}

fn put_u32x3(w: &mut Writer, v: [u32; 3]) {
    w.u32(v[0]).u32(v[1]).u32(v[2]);
}

pub(crate) fn put_reflection(w: &mut Writer, s: &ShaderReflection) {
    w.u32(s.stage.to_raw()).str(&s.function_name);
    w.u32(s.resources.len() as u32);
    for l in &s.resources {
        w.u32(l.resource_type.to_raw()).u32(l.space).u32(l.slot).u32(l.top_level_offset).u64(l.size_bytes).str(&l.name);
    }
    w.bool(s.needs_function_constants);
    w.u32(s.function_constants.len() as u32);
    for f in &s.function_constants {
        w.str(&f.name).u32(f.ty);
    }
    match &s.info {
        StageInfo::None => {
            w.u8(0);
        }
        StageInfo::Compute { threadgroup_size } => {
            w.u8(1);
            put_u32x3(w, *threadgroup_size);
        }
        StageInfo::Vertex { instance_id_index, vertex_id_index, vertex_output_size_in_bytes, needs_draw_params, inputs } => {
            w.u8(2).i32(*instance_id_index).i32(*vertex_id_index).u32(*vertex_output_size_in_bytes).bool(*needs_draw_params);
            put_attrs(w, inputs);
        }
        StageInfo::Fragment { num_render_targets, rt_index_int, discards } => {
            w.u8(3).i32(*num_render_targets).u8(*rt_index_int).bool(*discards);
        }
        StageInfo::Geometry {
            outputs,
            is_passthrough,
            rt_array_index_record_id,
            viewport_array_index_record_id,
            input_primitive,
            max_input_primitives_per_mesh_threadgroup,
            max_payload_size_in_bytes,
            instance_count,
        } => {
            w.u8(4);
            put_attrs(w, outputs);
            w.bool(*is_passthrough).i32(*rt_array_index_record_id).i32(*viewport_array_index_record_id);
            w.u32(*input_primitive).u32(*max_input_primitives_per_mesh_threadgroup).u32(*max_payload_size_in_bytes).u32(*instance_count);
        }
        StageInfo::Hull {
            max_patches_per_object_threadgroup,
            max_object_threads_per_patch,
            patch_constants_size,
            patch_constant_function,
            static_payload_size,
            payload_size_per_patch,
            input_control_point_count,
            output_control_point_count,
            output_control_point_size,
            tessellator_domain,
            tessellator_partitioning,
            tessellator_output_primitive,
            tessellation_type_half,
            max_tessellation_factor,
        } => {
            w.u8(5).u32(*max_patches_per_object_threadgroup).u32(*max_object_threads_per_patch).u32(*patch_constants_size);
            w.str(patch_constant_function).u32(*static_payload_size).u32(*payload_size_per_patch);
            w.u32(*input_control_point_count).u32(*output_control_point_count).u32(*output_control_point_size);
            w.u32(*tessellator_domain).u32(*tessellator_partitioning).u32(*tessellator_output_primitive);
            w.bool(*tessellation_type_half).f32(*max_tessellation_factor);
        }
        StageInfo::Domain {
            tessellator_domain,
            max_input_prims_per_mesh_threadgroup,
            input_control_point_count,
            input_control_point_size,
            patch_constants_size,
            tessellation_type_half,
        } => {
            w.u8(6).u32(*tessellator_domain).u32(*max_input_prims_per_mesh_threadgroup).u32(*input_control_point_count);
            w.u32(*input_control_point_size).u32(*patch_constants_size).bool(*tessellation_type_half);
        }
        StageInfo::Mesh { max_vertex_output_count, max_primitive_output_count, primitive_topology, max_payload_size_in_bytes, threadgroup_size } => {
            w.u8(7).u32(*max_vertex_output_count).u32(*max_primitive_output_count).u32(*primitive_topology).u32(*max_payload_size_in_bytes);
            put_u32x3(w, *threadgroup_size);
        }
        StageInfo::Amplification { threadgroup_size, max_payload_size_in_bytes } => {
            w.u8(8);
            put_u32x3(w, *threadgroup_size);
            w.u32(*max_payload_size_in_bytes);
        }
        StageInfo::RayTracing { is_indirect_intersection_function } => {
            w.u8(9).bool(*is_indirect_intersection_function);
        }
    }
    w.str(&s.json);
}

pub(crate) fn get_reflection(r: &mut Reader) -> Result<ShaderReflection, Error> {
    let stage = ShaderStage::from_raw(r.u32()?);
    let function_name = r.string()?;
    let n = r.u32()?;
    let resources = (0..n)
        .map(|_| {
            Ok(ResourceLocation {
                resource_type: ResourceType::from_raw(r.u32()?),
                space: r.u32()?,
                slot: r.u32()?,
                top_level_offset: r.u32()?,
                size_bytes: r.u64()?,
                name: r.string()?,
            })
        })
        .collect::<Result<_, Error>>()?;
    let needs_function_constants = r.bool()?;
    let n = r.u32()?;
    let function_constants = (0..n).map(|_| Ok(FunctionConstant { name: r.string()?, ty: r.u32()? })).collect::<Result<_, Error>>()?;
    let info = match r.u8()? {
        0 => StageInfo::None,
        1 => StageInfo::Compute { threadgroup_size: r.u32x3()? },
        2 => StageInfo::Vertex {
            instance_id_index: r.i32()?,
            vertex_id_index: r.i32()?,
            vertex_output_size_in_bytes: r.u32()?,
            needs_draw_params: r.bool()?,
            inputs: get_attrs(r)?,
        },
        3 => StageInfo::Fragment { num_render_targets: r.i32()?, rt_index_int: r.u8()?, discards: r.bool()? },
        4 => StageInfo::Geometry {
            outputs: get_attrs(r)?,
            is_passthrough: r.bool()?,
            rt_array_index_record_id: r.i32()?,
            viewport_array_index_record_id: r.i32()?,
            input_primitive: r.u32()?,
            max_input_primitives_per_mesh_threadgroup: r.u32()?,
            max_payload_size_in_bytes: r.u32()?,
            instance_count: r.u32()?,
        },
        5 => StageInfo::Hull {
            max_patches_per_object_threadgroup: r.u32()?,
            max_object_threads_per_patch: r.u32()?,
            patch_constants_size: r.u32()?,
            patch_constant_function: r.string()?,
            static_payload_size: r.u32()?,
            payload_size_per_patch: r.u32()?,
            input_control_point_count: r.u32()?,
            output_control_point_count: r.u32()?,
            output_control_point_size: r.u32()?,
            tessellator_domain: r.u32()?,
            tessellator_partitioning: r.u32()?,
            tessellator_output_primitive: r.u32()?,
            tessellation_type_half: r.bool()?,
            max_tessellation_factor: r.f32()?,
        },
        6 => StageInfo::Domain {
            tessellator_domain: r.u32()?,
            max_input_prims_per_mesh_threadgroup: r.u32()?,
            input_control_point_count: r.u32()?,
            input_control_point_size: r.u32()?,
            patch_constants_size: r.u32()?,
            tessellation_type_half: r.bool()?,
        },
        7 => StageInfo::Mesh {
            max_vertex_output_count: r.u32()?,
            max_primitive_output_count: r.u32()?,
            primitive_topology: r.u32()?,
            max_payload_size_in_bytes: r.u32()?,
            threadgroup_size: r.u32x3()?,
        },
        8 => StageInfo::Amplification { threadgroup_size: r.u32x3()?, max_payload_size_in_bytes: r.u32()? },
        9 => StageInfo::RayTracing { is_indirect_intersection_function: r.bool()? },
        _ => return Err(bad("stage info tag")),
    };
    let json = r.string()?;
    Ok(ShaderReflection { stage, function_name, resources, needs_function_constants, function_constants, info, json })
}

/// Shader body (everything but the key).
pub(crate) fn put_shader(w: &mut Writer, s: &CompiledShader) {
    w.u32(s.stage.to_raw()).str(&s.function_name);
    put_reflection(w, &s.reflection);
    w.bytes(&s.metallib);
}

pub(crate) fn get_shader(r: &mut Reader, key: u128) -> Result<CompiledShader, Error> {
    let stage = ShaderStage::from_raw(r.u32()?);
    let function_name = r.string()?;
    let reflection = get_reflection(r)?;
    let metallib = r.bytes()?.to_vec();
    Ok(CompiledShader { metallib, function_name, stage, reflection, key })
}

// ---- errors -----------------------------------------------------------------------------------

pub(crate) fn put_error(w: &mut Writer, e: &Error) {
    match e {
        Error::Load(m) => w.u8(0).str(m),
        Error::InvalidInput(m) => w.u8(1).str(m),
        Error::Convert { code, message } => w.u8(2).u32(*code as u32).str(message),
        Error::StageMismatch { expected, actual } => w.u8(3).u32(expected.to_raw()).u32(actual.to_raw()),
        Error::Io(m) => w.u8(4).str(m),
        Error::Helper(m) => w.u8(5).str(m),
        Error::Metal(m) => w.u8(6).str(m),
    };
}

pub(crate) fn get_error(r: &mut Reader) -> Result<Error, Error> {
    Ok(match r.u8()? {
        0 => Error::Load(r.string()?),
        1 => Error::InvalidInput(r.string()?),
        2 => Error::Convert { code: ErrorCode::from_raw(r.u32()?), message: r.string()? },
        3 => Error::StageMismatch { expected: ShaderStage::from_raw(r.u32()?), actual: ShaderStage::from_raw(r.u32()?) },
        4 => Error::Io(r.string()?),
        5 => Error::Helper(r.string()?),
        6 => Error::Metal(r.string()?),
        _ => return Err(bad("error tag")),
    })
}

// ---- options ----------------------------------------------------------------------------------

pub(crate) fn put_options(w: &mut Writer, o: &CompileOptions) {
    w.u32(o.min_gpu_family as u32).opt_str(o.min_macos.as_deref());
    w.u32(o.compatibility_flags).u32(o.validation_flags).u32(o.stage_in_mode as u32);
    w.opt_u32(o.input_topology.map(|t| t as u32));
    w.bool(o.geometry_tessellation_emulation).u32(o.dual_source_blending as u32).u32(o.depth_feedback as u32);
    w.u8(o.int_rt_mask).opt_u32(o.sample_mask).opt_str(o.metal_function_name.as_deref());
    match &o.ray_tracing {
        None => {
            w.u8(0);
        }
        Some(rt) => {
            w.u8(1).u32(rt.max_attribute_size_in_bytes).u32(rt.pipeline_flags);
            w.u64(rt.closest_hit_intrinsics).u64(rt.miss_intrinsics).u64(rt.any_hit_intrinsics).u64(rt.callable_intrinsics);
            w.i32(rt.max_recursive_depth).u32(rt.ray_generation_mode).u32(rt.intersection_function_mode);
            w.bool(rt.intersection_function_groups).bool(rt.direct_state_access);
        }
    }
    w.opt_u32(o.hit_group_type).bool(o.ignore_debug_info).bool(o.ignore_embedded_root_signature);
    w.opt_u32(o.function_constant_space).opt_u32(o.framebuffer_fetch_space);
}

pub(crate) fn get_options(r: &mut Reader) -> Result<CompileOptions, Error> {
    let min_gpu_family = match r.u32()? {
        1006 => GpuFamily::Apple6,
        1007 => GpuFamily::Apple7,
        1008 => GpuFamily::Apple8,
        1009 => GpuFamily::Apple9,
        1010 => GpuFamily::Apple10,
        5001 => GpuFamily::Metal3,
        _ => return Err(bad("gpu family")),
    };
    let tristate = |v: u32| match v {
        1 => Tristate::ForceEnabled,
        2 => Tristate::ForceDisabled,
        _ => Tristate::DecideAtRuntime,
    };
    Ok(CompileOptions {
        min_gpu_family,
        min_macos: r.opt_string()?,
        compatibility_flags: r.u32()?,
        validation_flags: r.u32()?,
        stage_in_mode: if r.u32()? == 1 { StageInMode::SeparateFunction } else { StageInMode::MetalVertexFetch },
        input_topology: r.opt_u32()?.map(|t| match t {
            1 => InputTopology::Point,
            2 => InputTopology::Line,
            3 => InputTopology::Triangle,
            4 => InputTopology::Patch,
            _ => InputTopology::Undefined,
        }),
        geometry_tessellation_emulation: r.bool()?,
        dual_source_blending: tristate(r.u32()?),
        depth_feedback: tristate(r.u32()?),
        int_rt_mask: r.u8()?,
        sample_mask: r.opt_u32()?,
        metal_function_name: r.opt_string()?,
        ray_tracing: if r.bool()? {
            Some(RayTracingConfig {
                max_attribute_size_in_bytes: r.u32()?,
                pipeline_flags: r.u32()?,
                closest_hit_intrinsics: r.u64()?,
                miss_intrinsics: r.u64()?,
                any_hit_intrinsics: r.u64()?,
                callable_intrinsics: r.u64()?,
                max_recursive_depth: r.i32()?,
                ray_generation_mode: r.u32()?,
                intersection_function_mode: r.u32()?,
                intersection_function_groups: r.bool()?,
                direct_state_access: r.bool()?,
            })
        } else {
            None
        },
        hit_group_type: r.opt_u32()?,
        ignore_debug_info: r.bool()?,
        ignore_embedded_root_signature: r.bool()?,
        function_constant_space: r.opt_u32()?,
        framebuffer_fetch_space: r.opt_u32()?,
    })
}

pub(crate) fn put_input_layout(w: &mut Writer, layout: &[InputElement]) {
    w.u32(layout.len() as u32);
    for e in layout {
        w.str(&e.semantic_name).u32(e.semantic_index).u32(e.format).u32(e.input_slot);
        w.u32(e.aligned_byte_offset).u32(e.input_slot_class).u32(e.instance_data_step_rate);
    }
}

pub(crate) fn get_input_layout(r: &mut Reader) -> Result<Vec<InputElement>, Error> {
    let n = r.u32()?;
    (0..n)
        .map(|_| {
            Ok(InputElement {
                semantic_name: r.string()?,
                semantic_index: r.u32()?,
                format: r.u32()?,
                input_slot: r.u32()?,
                aligned_byte_offset: r.u32()?,
                input_slot_class: r.u32()?,
                instance_data_step_rate: r.u32()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_round_trip() {
        let mut o = CompileOptions { min_macos: None, sample_mask: Some(3), hit_group_type: Some(1), ..Default::default() };
        o.ray_tracing = Some(RayTracingConfig { max_recursive_depth: 4, direct_state_access: true, ..Default::default() });
        o.input_topology = Some(InputTopology::Triangle);
        o.dual_source_blending = Tristate::ForceDisabled;
        o.metal_function_name = Some("main0".into());
        for o in [CompileOptions::default(), o] {
            let mut w = Writer::default();
            put_options(&mut w, &o);
            let mut r = Reader::new(&w.buf);
            assert_eq!(get_options(&mut r).unwrap(), o);
            assert_eq!(r.remaining(), 0);
        }
    }

    #[test]
    fn reflection_and_errors_round_trip() {
        let refl = ShaderReflection {
            stage: ShaderStage::Vertex,
            function_name: "vs".into(),
            resources: vec![ResourceLocation {
                resource_type: ResourceType::Table,
                space: 1,
                slot: 2,
                top_level_offset: 8,
                size_bytes: 8,
                name: "t".into(),
            }],
            needs_function_constants: true,
            function_constants: vec![FunctionConstant { name: "fc".into(), ty: 53 }],
            info: StageInfo::Vertex {
                instance_id_index: -1,
                vertex_id_index: 3,
                vertex_output_size_in_bytes: 64,
                needs_draw_params: true,
                inputs: vec![VertexAttribute { name: "POSITION0".into(), attribute_index: 0 }],
            },
            json: "{}".into(),
        };
        let s = CompiledShader { metallib: vec![1, 2, 3], function_name: "vs".into(), stage: ShaderStage::Vertex, reflection: refl, key: 7 };
        let mut w = Writer::default();
        put_shader(&mut w, &s);
        assert_eq!(get_shader(&mut Reader::new(&w.buf), 7).unwrap(), s);

        for e in [
            Error::Convert { code: ErrorCode::UnsupportedInstruction, message: "x".into() },
            Error::StageMismatch { expected: ShaderStage::Vertex, actual: ShaderStage::Fragment },
            Error::Helper("crashed".into()),
        ] {
            let mut w = Writer::default();
            put_error(&mut w, &e);
            assert_eq!(get_error(&mut Reader::new(&w.buf)).unwrap(), e);
        }
        assert!(get_shader(&mut Reader::new(&w.buf[..2]), 0).is_err());
    }
}
