//! CPU-side mirror of `metal_irconverter_runtime.h`: bind points, descriptor table entry
//! encoding (`IRDescriptorTableSet*`), and draw-parameter structs the converted shaders read.
//!
//! Metal objects are represented by the raw values the shaders consume (`gpuAddress`,
//! `gpuResourceID._impl`), so this module needs no Metal bindings and works in the x86_64 loader.

/// Buffer index of the SM 6.6 `ResourceDescriptorHeap` (`kIRDescriptorHeapBindPoint`).
pub const DESCRIPTOR_HEAP_BIND_POINT: u64 = 0;
/// Buffer index of the SM 6.6 `SamplerDescriptorHeap` (`kIRSamplerHeapBindPoint`).
pub const SAMPLER_HEAP_BIND_POINT: u64 = 1;
/// Buffer index of the top-level argument buffer (`kIRArgumentBufferBindPoint`).
pub const ARGUMENT_BUFFER_BIND_POINT: u64 = 2;
pub const ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT: u64 = 3;
/// `IRRuntimeDrawParams` (vertex stage, `kIRArgumentBufferDrawArgumentsBindPoint`).
pub const ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT: u64 = 4;
/// `uint16_t` index type (vertex stage, `kIRArgumentBufferUniformsBindPoint`).
pub const ARGUMENT_BUFFER_UNIFORMS_BIND_POINT: u64 = 5;
/// `IRRuntimeVertexBuffers` for geometry emulation (`kIRVertexBufferBindPoint`).
pub const VERTEX_BUFFER_BIND_POINT: u64 = 6;
/// Tessellator lookup tables buffer (`kIRRuntimeTessellatorTablesBindPoint`).
pub const TESSELLATOR_TABLES_BIND_POINT: u64 = 7;
/// First Metal vertex attribute index used for stage-in (`kIRStageInAttributeStartIndex`).
pub const STAGE_IN_ATTRIBUTE_START_INDEX: u64 = 11;

pub const INDIRECT_TRIANGLE_INTERSECTION_FUNCTION_NAME: &str = "irconverter.wrapper.intersection.function.triangle";
pub const INDIRECT_PROCEDURAL_INTERSECTION_FUNCTION_NAME: &str = "irconverter.wrapper.intersection.function.procedural";
pub const TRIANGLE_PASSTHROUGH_GEOMETRY_SHADER: &str = "irconverter_domain_shader_triangle_passthrough";
pub const LINE_PASSTHROUGH_GEOMETRY_SHADER: &str = "irconverter_domain_shader_line_passthrough";
pub const POINT_PASSTHROUGH_GEOMETRY_SHADER: &str = "irconverter_domain_shader_point_passthrough";
pub const FUNCTION_GROUP_RAY_GENERATION: &str = "rayGen";
pub const FUNCTION_GROUP_CLOSEST_HIT: &str = "closestHit";
pub const FUNCTION_GROUP_MISS: &str = "miss";

/// `kIRNonIndexedDraw`.
pub const NON_INDEXED_DRAW: u16 = 0;

/// Size of one descriptor in a descriptor heap / table (`sizeof(IRDescriptorTableEntry)`), i.e.
/// the `GetDescriptorHandleIncrementSize` the D3D12 layer should report for shader-visible heaps.
pub const DESCRIPTOR_TABLE_ENTRY_SIZE: usize = 24;

const BUF_SIZE_MASK: u64 = 0xffff_ffff;
const TEX_VIEW_OFFSET: u64 = 32;
const TEX_VIEW_MASK: u64 = 0xff;
const TYPED_BUFFER_OFFSET: u64 = 63;

/// `IRDescriptorTableEntry`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DescriptorTableEntry {
    pub gpu_va: u64,
    pub texture_view_id: u64,
    pub metadata: u64,
}

/// `IRBufferView` without Metal objects: addresses/IDs supplied by the caller.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BufferView {
    /// `buffer.gpuAddress + bufferOffset`.
    pub gpu_va: u64,
    /// Size in bytes visible to the shader (bounds checks use the low 32 bits).
    pub size: u64,
    /// `gpuResourceID._impl` of a texture-buffer view for typed buffers / append counters (0 if none).
    pub texture_buffer_view_id: u64,
    /// Element offset of the view inside the texture buffer (8 bits).
    pub texture_view_offset_in_elements: u32,
    /// True for typed buffers (`Buffer<T>` / `RWBuffer<T>`) accessed through the texture view.
    pub typed_buffer: bool,
}

/// `IRDescriptorTableGetBufferMetadata`.
pub fn buffer_metadata(size: u64, texture_view_offset_in_elements: u32, typed_buffer: bool) -> u64 {
    (size & BUF_SIZE_MASK)
        | ((texture_view_offset_in_elements as u64 & TEX_VIEW_MASK) << TEX_VIEW_OFFSET)
        | ((typed_buffer as u64) << TYPED_BUFFER_OFFSET)
}

impl DescriptorTableEntry {
    /// Null descriptor.
    pub const NULL: Self = Self { gpu_va: 0, texture_view_id: 0, metadata: 0 };

    /// `IRDescriptorTableSetBuffer` (raw buffers, structured buffers, CBVs).
    pub fn buffer(gpu_va: u64, metadata: u64) -> Self {
        Self { gpu_va, texture_view_id: 0, metadata }
    }

    /// `IRDescriptorTableSetBufferView`.
    pub fn buffer_view(v: &BufferView) -> Self {
        Self {
            gpu_va: v.gpu_va,
            texture_view_id: v.texture_buffer_view_id,
            metadata: buffer_metadata(v.size, v.texture_view_offset_in_elements, v.typed_buffer),
        }
    }

    /// `IRDescriptorTableSetTexture(entry, texture, minLODClamp, metadata)`;
    /// `texture_resource_id` is `texture.gpuResourceID._impl`. `metadata` must currently be 0.
    pub fn texture(texture_resource_id: u64, min_lod_clamp: f32, metadata: u32) -> Self {
        Self {
            gpu_va: 0,
            texture_view_id: texture_resource_id,
            metadata: min_lod_clamp.to_bits() as u64 | ((metadata as u64) << 32),
        }
    }

    /// `IRDescriptorTableSetSampler(entry, sampler, lodBias)`;
    /// `sampler_resource_id` is `sampler.gpuResourceID._impl`.
    pub fn sampler(sampler_resource_id: u64, lod_bias: f32) -> Self {
        Self { gpu_va: sampler_resource_id, texture_view_id: 0, metadata: lod_bias.to_bits() as u64 }
    }

    pub fn to_bytes(&self) -> [u8; 24] {
        let mut b = [0u8; 24];
        b[0..8].copy_from_slice(&self.gpu_va.to_le_bytes());
        b[8..16].copy_from_slice(&self.texture_view_id.to_le_bytes());
        b[16..24].copy_from_slice(&self.metadata.to_le_bytes());
        b
    }
}

/// `IRRuntimeVertexBuffer`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeVertexBuffer {
    pub addr: u64,
    pub length: u32,
    pub stride: u32,
}

impl RuntimeVertexBuffer {
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut b = [0u8; 16];
        b[0..8].copy_from_slice(&self.addr.to_le_bytes());
        b[8..12].copy_from_slice(&self.length.to_le_bytes());
        b[12..16].copy_from_slice(&self.stride.to_le_bytes());
        b
    }
}

/// `IRRuntimeDrawArgument`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeDrawArgument {
    pub vertex_count_per_instance: u32,
    pub instance_count: u32,
    pub start_vertex_location: u32,
    pub start_instance_location: u32,
}

/// `IRRuntimeDrawIndexedArgument`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeDrawIndexedArgument {
    pub index_count_per_instance: u32,
    pub instance_count: u32,
    pub start_index_location: u32,
    pub base_vertex_location: i32,
    pub start_instance_location: u32,
}

/// `IRRuntimeDrawParams` (a union; 20 bytes). Bind with `setVertexBytes` at
/// [`ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT`] when the vertex shader reflection says
/// `needs_draw_params`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union RuntimeDrawParams {
    pub draw: RuntimeDrawArgument,
    pub draw_indexed: RuntimeDrawIndexedArgument,
}

impl RuntimeDrawParams {
    pub fn to_bytes(&self) -> [u8; 20] {
        // SAFETY: both variants are plain u32 data; the union is 20 bytes.
        unsafe { std::mem::transmute_copy(self) }
    }
}

/// `IRMetalIndexToIRIndex`: `MTLIndexType` (UInt16 = 0, UInt32 = 1) to the value bound at
/// [`ARGUMENT_BUFFER_UNIFORMS_BIND_POINT`]; non-indexed draws bind [`NON_INDEXED_DRAW`].
pub fn metal_index_type_to_ir(mtl_index_type: u64) -> u16 {
    (mtl_index_type + 1) as u16
}

/// `IRRuntimeDrawInfo` (geometry/tessellation emulation and mesh pipelines).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeDrawInfo {
    pub index_type: u16,
    pub primitive_topology: u8,
    pub threads_per_patch: u8,
    pub max_input_primitives_per_mesh_threadgroup: u16,
    pub object_threadgroup_vertex_stride: u16,
    pub mesh_threadgroup_primitive_stride: u16,
    pub gs_instance_count: u16,
    pub patches_per_object_threadgroup: u16,
    pub input_control_points_per_patch: u16,
    pub index_buffer: u64,
}

impl RuntimeDrawInfo {
    pub fn to_bytes(&self) -> [u8; 24] {
        let mut b = [0u8; 24];
        b[0..2].copy_from_slice(&self.index_type.to_le_bytes());
        b[2] = self.primitive_topology;
        b[3] = self.threads_per_patch;
        b[4..6].copy_from_slice(&self.max_input_primitives_per_mesh_threadgroup.to_le_bytes());
        b[6..8].copy_from_slice(&self.object_threadgroup_vertex_stride.to_le_bytes());
        b[8..10].copy_from_slice(&self.mesh_threadgroup_primitive_stride.to_le_bytes());
        b[10..12].copy_from_slice(&self.gs_instance_count.to_le_bytes());
        b[12..14].copy_from_slice(&self.patches_per_object_threadgroup.to_le_bytes());
        b[14..16].copy_from_slice(&self.input_control_points_per_patch.to_le_bytes());
        b[16..24].copy_from_slice(&self.index_buffer.to_le_bytes());
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn layouts_match_runtime_header() {
        assert_eq!(size_of::<DescriptorTableEntry>(), DESCRIPTOR_TABLE_ENTRY_SIZE);
        assert_eq!(size_of::<RuntimeDrawParams>(), 20);
        assert_eq!(size_of::<RuntimeVertexBuffer>(), 16);
        assert_eq!(size_of::<RuntimeDrawInfo>(), 24);
    }

    #[test]
    fn encodings_match_runtime_header() {
        let e = DescriptorTableEntry::buffer_view(&BufferView {
            gpu_va: 0x1000,
            size: 0x1_0000_0040,
            texture_buffer_view_id: 7,
            texture_view_offset_in_elements: 0x1ff,
            typed_buffer: true,
        });
        assert_eq!(e.metadata, 0x40 | (0xff << 32) | (1 << 63));
        assert_eq!(e.texture_view_id, 7);
        let t = DescriptorTableEntry::texture(42, 1.5, 0);
        assert_eq!(t, DescriptorTableEntry { gpu_va: 0, texture_view_id: 42, metadata: 1.5f32.to_bits() as u64 });
        let s = DescriptorTableEntry::sampler(9, -0.5);
        assert_eq!(s, DescriptorTableEntry { gpu_va: 9, texture_view_id: 0, metadata: (-0.5f32).to_bits() as u64 });
        assert_eq!(metal_index_type_to_ir(1), 2);
    }
}
