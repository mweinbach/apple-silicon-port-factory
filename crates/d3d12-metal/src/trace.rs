//! D3D12 call trace: a compact, arch-neutral binary log of the API calls the game makes. The
//! x86_64 COM layer writes it when `AW2_D3D12_TRACE=<path>` is set. Native tools read it
//! (`crates/d3d12-replay`), so D3D12 bugs can be reproduced, bisected and profiled without
//! running the game.
//!
//! ```ignore
//! if trace::enabled() {
//!     trace::record(&Call::CreateCommittedResource { device, resource, heap_properties: bytes_of(&hp), .. });
//! }
//! ```
//!
//! # Format (little endian)
//! * Header: `b"AW2DTRC\0"`, then `u32 version` ([`VERSION`]), then `u32 reserved`.
//! * Records: `u32 payload_len | u16 op | u16 thread | u64 t_ns | payload`. `t_ns` counts from
//!   the start of the trace, and `thread` is a small per-process thread index. Records are
//!   appended under one lock, so file order is a valid serialization of the calls. A torn
//!   final record (crash) is ignored by the reader.
//! * Payload: the call's fields in declaration order. Scalars are fixed width. `Blob` is
//!   `u8 tag`, then for tag 0 `u32 len, bytes` (inline) or for tag 1 `u32 index`, a back-reference
//!   to the index-th inline blob of at least [`DEDUP_MIN`] bytes. Shader bytecode and root
//!   signatures repeat constantly, and this stores each one once. Both sides number large
//!   inline blobs in encounter order, so no separate table is needed.
//!
//! Object ids are the COM interface pointer values (0 = null). D3D12 structs are stored as
//! raw bytes in their Windows layout (e.g. `D3D12_RESOURCE_DESC`, 56 bytes), so the layer
//! records them without translating. Pointer-bearing structs, i.e. PSO descs and input
//! layouts, are flattened: see [`GraphicsPipeline`] and [`InputElement`].
//!
//! CPU writes into mapped upload memory are invisible to the API. The layer records them
//! as [`Call::WriteResource`] when it can see them: at `Unmap`, and for persistently mapped
//! upload buffers at `ExecuteCommandLists`, for the ranges it knows changed.
//!
//! Owned by the shaders engineer. Add calls at the end of the table, keeping their numbers,
//! and bump [`VERSION`] on any layout change.

use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

pub const MAGIC: &[u8; 8] = b"AW2DTRC\0";
pub const VERSION: u32 = 1;
/// Blobs at least this large are deduplicated.
pub const DEDUP_MIN: usize = 256;
const HEADER_LEN: usize = 16;
const RECORD_HEADER_LEN: usize = 16;

// ---- field encoding -----------------------------------------------------------------------------

/// Errors decoding a trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceError(pub String);
impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trace: {}", self.0)
    }
}
impl std::error::Error for TraceError {}
fn err<T>(m: impl Into<String>) -> Result<T, TraceError> {
    Err(TraceError(m.into()))
}

/// Encoder state (blob dedup table).
#[derive(Default)]
pub struct Enc {
    pub buf: Vec<u8>,
    blobs: HashMap<(u64, usize), u32>,
    next_blob: u32,
}

/// Decoder state over one trace buffer.
pub struct Dec<'a> {
    data: &'a [u8],
    pos: usize,
    blobs: Vec<&'a [u8]>,
}

impl<'a> Dec<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], TraceError> {
        let s = self.data.get(self.pos..self.pos + n).ok_or_else(|| TraceError("payload too short".into()))?;
        self.pos += n;
        Ok(s)
    }
}

/// A field type that can appear in a [`Call`].
pub trait Field<'a>: Sized {
    fn put(&self, e: &mut Enc);
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError>;
}

macro_rules! scalar_field {
    ($($t:ty),*) => {$(
        impl<'a> Field<'a> for $t {
            fn put(&self, e: &mut Enc) {
                e.buf.extend_from_slice(&self.to_le_bytes());
            }
            fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
                Ok(<$t>::from_le_bytes(d.take(std::mem::size_of::<$t>())?.try_into().unwrap()))
            }
        }
    )*};
}
scalar_field!(u8, u16, u32, i32, u64, i64, f32);

impl<'a> Field<'a> for bool {
    fn put(&self, e: &mut Enc) {
        e.buf.push(*self as u8);
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        Ok(d.take(1)?[0] != 0)
    }
}

impl<'a> Field<'a> for [f32; 4] {
    fn put(&self, e: &mut Enc) {
        for v in self {
            v.put(e);
        }
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        Ok([f32::get(d)?, f32::get(d)?, f32::get(d)?, f32::get(d)?])
    }
}

/// Raw bytes (D3D12 structs, shader bytecode, upload data). Large blobs are deduplicated.
pub type Blob<'a> = &'a [u8];

impl<'a> Field<'a> for &'a [u8] {
    fn put(&self, e: &mut Enc) {
        if self.len() >= DEDUP_MIN {
            let k = (dxil_metal::hash::xxh64(self, 0), self.len());
            if let Some(&i) = e.blobs.get(&k) {
                e.buf.push(1);
                e.buf.extend_from_slice(&i.to_le_bytes());
                return;
            }
            e.blobs.insert(k, e.next_blob);
            e.next_blob += 1;
        }
        e.buf.push(0);
        e.buf.extend_from_slice(&(self.len() as u32).to_le_bytes());
        e.buf.extend_from_slice(self);
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        match d.take(1)?[0] {
            0 => {
                let n = u32::get(d)? as usize;
                let b = d.take(n)?;
                if n >= DEDUP_MIN {
                    d.blobs.push(b);
                }
                Ok(b)
            }
            1 => {
                let i = u32::get(d)? as usize;
                d.blobs.get(i).copied().ok_or_else(|| TraceError(format!("blob back-reference {i} out of range")))
            }
            t => err(format!("bad blob tag {t}")),
        }
    }
}

impl<'a> Field<'a> for Option<&'a [u8]> {
    fn put(&self, e: &mut Enc) {
        match self {
            None => e.buf.push(0),
            Some(b) => {
                e.buf.push(1);
                b.put(e);
            }
        }
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        Ok(match d.take(1)?[0] {
            0 => None,
            _ => Some(<&[u8]>::get(d)?),
        })
    }
}

impl<'a> Field<'a> for &'a str {
    fn put(&self, e: &mut Enc) {
        self.as_bytes().put(e);
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        std::str::from_utf8(<&[u8]>::get(d)?).map_err(|_| TraceError("invalid utf-8 string".into()))
    }
}

macro_rules! array_field {
    ($name:ident, $t:ty, $n:literal) => {
        /// A little-endian array view (unaligned-safe; build with `from_slice`).
        #[derive(Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name<'a>(pub &'a [u8]);
        impl<'a> $name<'a> {
            pub fn from_slice(v: &'a [$t]) -> Self {
                // SAFETY: plain integers, viewed as bytes; both supported targets are little endian.
                Self(unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) })
            }
            pub fn len(&self) -> usize {
                self.0.len() / $n
            }
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            pub fn get(&self, i: usize) -> $t {
                <$t>::from_le_bytes(self.0[i * $n..i * $n + $n].try_into().unwrap())
            }
            pub fn iter(&self) -> impl Iterator<Item = $t> + 'a {
                let b = self.0;
                b.chunks_exact($n).map(|c| <$t>::from_le_bytes(c.try_into().unwrap()))
            }
            pub fn to_vec(&self) -> Vec<$t> {
                self.iter().collect()
            }
        }
        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.iter()).finish()
            }
        }
        impl<'a> Field<'a> for $name<'a> {
            fn put(&self, e: &mut Enc) {
                e.buf.extend_from_slice(&(self.len() as u32).to_le_bytes());
                e.buf.extend_from_slice(self.0);
            }
            fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
                let n = u32::get(d)? as usize;
                Ok(Self(d.take(n.checked_mul($n).ok_or_else(|| TraceError("array too long".into()))?)?))
            }
        }
    };
}
array_field!(U64s, u64, 8);
array_field!(U32s, u32, 4);

// ---- flattened structs --------------------------------------------------------------------------

/// `D3D12_INPUT_ELEMENT_DESC` with the semantic name inlined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputElement {
    pub semantic_name: String,
    pub semantic_index: u32,
    pub format: u32,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: u32,
    pub instance_data_step_rate: u32,
}

/// Encodes an input layout for [`GraphicsPipeline::input_layout`].
pub fn encode_input_layout(elements: &[InputElement]) -> Vec<u8> {
    let mut e = Enc::default();
    (elements.len() as u32).put(&mut e);
    for el in elements {
        let s: &str = &el.semantic_name;
        s.put(&mut e);
        for v in [el.semantic_index, el.format, el.input_slot, el.aligned_byte_offset, el.input_slot_class, el.instance_data_step_rate] {
            v.put(&mut e);
        }
    }
    e.buf
}

pub fn decode_input_layout(b: &[u8]) -> Result<Vec<InputElement>, TraceError> {
    let mut d = Dec { data: b, pos: 0, blobs: Vec::new() };
    let n = u32::get(&mut d)?;
    (0..n)
        .map(|_| {
            Ok(InputElement {
                semantic_name: <&str>::get(&mut d)?.to_string(),
                semantic_index: u32::get(&mut d)?,
                format: u32::get(&mut d)?,
                input_slot: u32::get(&mut d)?,
                aligned_byte_offset: u32::get(&mut d)?,
                input_slot_class: u32::get(&mut d)?,
                instance_data_step_rate: u32::get(&mut d)?,
            })
        })
        .collect()
}

/// Flattened `D3D12_GRAPHICS_PIPELINE_STATE_DESC`, or the equivalent pipeline state stream.
/// Unused shader stages are empty blobs.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphicsPipeline<'a> {
    pub root_signature: u64,
    pub vs: Blob<'a>,
    pub ps: Blob<'a>,
    pub ds: Blob<'a>,
    pub hs: Blob<'a>,
    pub gs: Blob<'a>,
    /// Stream output is unsupported, so this is recorded only to detect it. Raw
    /// `D3D12_STREAM_OUTPUT_DESC` with `NumEntries`, or empty.
    pub stream_output: Blob<'a>,
    /// `D3D12_BLEND_DESC` (328 bytes; `state::BlendDesc`).
    pub blend: Blob<'a>,
    pub sample_mask: u32,
    /// `D3D12_RASTERIZER_DESC` / `DESC1` / `DESC2` raw bytes (`raster_version` = 0/1/2).
    pub rasterizer: Blob<'a>,
    pub raster_version: u8,
    /// `D3D12_DEPTH_STENCIL_DESC` / `1` / `2` raw bytes (`depth_stencil_version` = 0/1/2).
    pub depth_stencil: Blob<'a>,
    pub depth_stencil_version: u8,
    /// [`encode_input_layout`] bytes.
    pub input_layout: Blob<'a>,
    pub ib_strip_cut_value: u32,
    pub primitive_topology_type: u32,
    pub rtv_formats: U32s<'a>,
    pub dsv_format: u32,
    pub sample_count: u32,
    pub sample_quality: u32,
    pub node_mask: u32,
    pub flags: u32,
}

impl<'a> Field<'a> for GraphicsPipeline<'a> {
    fn put(&self, e: &mut Enc) {
        self.root_signature.put(e);
        for b in [self.vs, self.ps, self.ds, self.hs, self.gs, self.stream_output, self.blend] {
            b.put(e);
        }
        self.sample_mask.put(e);
        self.rasterizer.put(e);
        self.raster_version.put(e);
        self.depth_stencil.put(e);
        self.depth_stencil_version.put(e);
        self.input_layout.put(e);
        for v in [self.ib_strip_cut_value, self.primitive_topology_type] {
            v.put(e);
        }
        self.rtv_formats.put(e);
        for v in [self.dsv_format, self.sample_count, self.sample_quality, self.node_mask, self.flags] {
            v.put(e);
        }
    }
    fn get(d: &mut Dec<'a>) -> Result<Self, TraceError> {
        Ok(Self {
            root_signature: Field::get(d)?,
            vs: Field::get(d)?,
            ps: Field::get(d)?,
            ds: Field::get(d)?,
            hs: Field::get(d)?,
            gs: Field::get(d)?,
            stream_output: Field::get(d)?,
            blend: Field::get(d)?,
            sample_mask: Field::get(d)?,
            rasterizer: Field::get(d)?,
            raster_version: Field::get(d)?,
            depth_stencil: Field::get(d)?,
            depth_stencil_version: Field::get(d)?,
            input_layout: Field::get(d)?,
            ib_strip_cut_value: Field::get(d)?,
            primitive_topology_type: Field::get(d)?,
            rtv_formats: Field::get(d)?,
            dsv_format: Field::get(d)?,
            sample_count: Field::get(d)?,
            sample_quality: Field::get(d)?,
            node_mask: Field::get(d)?,
            flags: Field::get(d)?,
        })
    }
}

// ---- the call table -----------------------------------------------------------------------------

macro_rules! calls {
    ($( $(#[$m:meta])* $id:literal $name:ident { $($f:ident : $t:ty),* $(,)? } )*) => {
        /// One recorded API call. Ids are COM pointers. `Blob`s are raw Windows-layout structs
        /// unless noted.
        #[derive(Debug, Clone, PartialEq)]
        pub enum Call<'a> {
            $( $(#[$m])* $name { $($f: $t),* }, )*
        }

        impl<'a> Call<'a> {
            pub fn op(&self) -> u16 {
                match self { $( Call::$name { .. } => $id, )* }
            }
            pub fn name(&self) -> &'static str {
                match self { $( Call::$name { .. } => stringify!($name), )* }
            }
            pub fn op_name(op: u16) -> Option<&'static str> {
                match op { $( $id => Some(stringify!($name)), )* _ => None }
            }
            #[allow(unused_variables)]
            pub fn encode(&self, e: &mut Enc) {
                match self { $( Call::$name { $($f),* } => { $( Field::put($f, e); )* } )* }
            }
            pub fn decode(op: u16, d: &mut Dec<'a>) -> Result<Self, TraceError> {
                match op {
                    $( $id => Ok(Call::$name { $($f: <$t as Field<'a>>::get(d)?),* }), )*
                    _ => err(format!("unknown op {op}")),
                }
            }
        }
    };
}

calls! {
    // Device / objects
    1 CreateDevice { device: u64, feature_level: u32 }
    2 CreateCommandQueue { device: u64, queue: u64, desc: Blob<'a> }
    3 CreateCommandAllocator { device: u64, allocator: u64, list_type: u32 }
    4 CreateCommandList { device: u64, list: u64, list_type: u32, allocator: u64, initial_pso: u64 }
    5 CreateDescriptorHeap { device: u64, heap: u64, desc: Blob<'a>, cpu_start: u64, gpu_start: u64 }
    6 CreateHeap { device: u64, heap: u64, desc: Blob<'a> }
    /// `heap_properties` = D3D12_HEAP_PROPERTIES, `desc` = D3D12_RESOURCE_DESC (or DESC1).
    7 CreateCommittedResource { device: u64, resource: u64, heap_properties: Blob<'a>, heap_flags: u32, desc: Blob<'a>, initial_state: u32, clear_value: Option<&'a [u8]>, gpu_va: u64 }
    8 CreatePlacedResource { device: u64, resource: u64, heap: u64, offset: u64, desc: Blob<'a>, initial_state: u32, clear_value: Option<&'a [u8]>, gpu_va: u64 }
    9 CreateReservedResource { device: u64, resource: u64, desc: Blob<'a>, initial_state: u32, clear_value: Option<&'a [u8]>, gpu_va: u64 }
    /// `blob` = serialized root signature (DXBC container or RTS0).
    10 CreateRootSignature { device: u64, root_signature: u64, blob: Blob<'a> }
    11 CreateGraphicsPipelineState { device: u64, pso: u64, desc: GraphicsPipeline<'a> }
    12 CreateComputePipelineState { device: u64, pso: u64, root_signature: u64, cs: Blob<'a>, flags: u32 }
    13 CreateCommandSignature { device: u64, signature: u64, root_signature: u64, byte_stride: u32, arguments: Blob<'a> }
    14 CreateFence { device: u64, fence: u64, initial_value: u64, flags: u32 }
    15 CreateQueryHeap { device: u64, heap: u64, desc: Blob<'a> }
    /// Final release of an object (refcount reached zero).
    16 Release { object: u64 }
    /// Mesh-shader PSO (pipeline stream with AS/MS subobjects). `desc` carries everything else
    /// (its `vs`, `ds`, `hs`, `gs` are empty); `amplification` is empty without an AS.
    17 CreateMeshPipelineState { device: u64, pso: u64, amplification: Blob<'a>, mesh: Blob<'a>, desc: GraphicsPipeline<'a> }
    /// `ID3D12Device5::CreateStateObject`.
    18 CreateStateObject {
        device: u64,
        state_object: u64,
        state_object_type: u32,
        num_subobjects: u32,
        num_exports: u32,
        num_hit_groups: u32,
    }
    /// `ID3D12Device7::AddToStateObject`.
    19 AddToStateObject {
        device: u64,
        state_object: u64,
        base: u64,
        state_object_type: u32,
        num_subobjects: u32,
        num_exports: u32,
        num_hit_groups: u32,
    }
    // Descriptors (`dest` = CPU descriptor handle)
    20 CreateShaderResourceView { resource: u64, desc: Option<&'a [u8]>, dest: u64 }
    21 CreateUnorderedAccessView { resource: u64, counter: u64, desc: Option<&'a [u8]>, dest: u64 }
    22 CreateRenderTargetView { resource: u64, desc: Option<&'a [u8]>, dest: u64 }
    23 CreateDepthStencilView { resource: u64, desc: Option<&'a [u8]>, dest: u64 }
    24 CreateConstantBufferView { buffer_location: u64, size: u32, dest: u64 }
    25 CreateSampler { desc: Blob<'a>, dest: u64 }
    26 CopyDescriptorsSimple { count: u32, dest: u64, src: u64, heap_type: u32 }
    /// `dest_ranges`/`src_ranges`: (start handle, count) pairs flattened.
    27 CopyDescriptors { dest_ranges: U64s<'a>, src_ranges: U64s<'a>, heap_type: u32 }
    // Memory
    /// CPU writes into an upload resource (see module docs).
    30 WriteResource { resource: u64, subresource: u32, offset: u64, data: Blob<'a> }
    // Command lists
    40 Reset { list: u64, allocator: u64, pso: u64 }
    41 Close { list: u64 }
    /// `barriers`: D3D12_RESOURCE_BARRIER array (32 bytes each).
    42 ResourceBarrier { list: u64, barriers: Blob<'a> }
    43 SetPipelineState { list: u64, pso: u64 }
    44 SetGraphicsRootSignature { list: u64, root_signature: u64 }
    45 SetComputeRootSignature { list: u64, root_signature: u64 }
    46 SetDescriptorHeaps { list: u64, heaps: U64s<'a> }
    47 SetRootDescriptorTable { list: u64, compute: bool, param: u32, gpu_handle: u64 }
    48 SetRoot32BitConstants { list: u64, compute: bool, param: u32, dest_offset: u32, values: U32s<'a> }
    /// `kind`: 0 = CBV, 1 = SRV, 2 = UAV.
    49 SetRootView { list: u64, compute: bool, param: u32, kind: u8, gpu_va: u64 }
    50 IASetPrimitiveTopology { list: u64, topology: u32 }
    /// `views`: D3D12_VERTEX_BUFFER_VIEW array (16 bytes each).
    51 IASetVertexBuffers { list: u64, start_slot: u32, views: Blob<'a> }
    /// `view`: D3D12_INDEX_BUFFER_VIEW (16 bytes); `None` = unbind.
    52 IASetIndexBuffer { list: u64, view: Option<&'a [u8]> }
    /// D3D12_VIEWPORT array (24 bytes each).
    53 RSSetViewports { list: u64, viewports: Blob<'a> }
    /// D3D12_RECT array (16 bytes each).
    54 RSSetScissorRects { list: u64, rects: Blob<'a> }
    /// `rtvs`: CPU handles (already expanded when `RTsSingleHandleToDescriptorRange`).
    55 OMSetRenderTargets { list: u64, rtvs: U64s<'a>, dsv: u64 }
    56 OMSetBlendFactor { list: u64, factor: [f32; 4] }
    57 OMSetStencilRef { list: u64, stencil_ref: u32 }
    58 OMSetDepthBounds { list: u64, min: f32, max: f32 }
    /// `ID3D12GraphicsCommandList8::OMSetFrontAndBackStencilRef`.
    59 OMSetFrontAndBackStencilRef { list: u64, front: u32, back: u32 }
    60 ClearRenderTargetView { list: u64, rtv: u64, color: [f32; 4], rects: Blob<'a> }
    61 ClearDepthStencilView { list: u64, dsv: u64, flags: u32, depth: f32, stencil: u8, rects: Blob<'a> }
    62 ClearUnorderedAccessViewUint { list: u64, gpu_handle: u64, cpu_handle: u64, resource: u64, values: U32s<'a>, rects: Blob<'a> }
    63 ClearUnorderedAccessViewFloat { list: u64, gpu_handle: u64, cpu_handle: u64, resource: u64, values: [f32; 4], rects: Blob<'a> }
    64 DiscardResource { list: u64, resource: u64, region: Option<&'a [u8]> }
    70 DrawInstanced { list: u64, vertex_count: u32, instance_count: u32, start_vertex: u32, start_instance: u32 }
    71 DrawIndexedInstanced { list: u64, index_count: u32, instance_count: u32, start_index: u32, base_vertex: i32, start_instance: u32 }
    72 Dispatch { list: u64, x: u32, y: u32, z: u32 }
    /// `ID3D12GraphicsCommandList6::DispatchMesh`.
    74 DispatchMesh { list: u64, x: u32, y: u32, z: u32 }
    73 ExecuteIndirect { list: u64, signature: u64, max_count: u32, args: u64, args_offset: u64, count: u64, count_offset: u64 }
    80 CopyBufferRegion { list: u64, dst: u64, dst_offset: u64, src: u64, src_offset: u64, size: u64 }
    /// `dst`/`src`: D3D12_TEXTURE_COPY_LOCATION (pResource replaced by the id at offset 0);
    /// `src_box`: D3D12_BOX or None.
    81 CopyTextureRegion { list: u64, dst: Blob<'a>, dst_x: u32, dst_y: u32, dst_z: u32, src: Blob<'a>, src_box: Option<&'a [u8]> }
    82 CopyResource { list: u64, dst: u64, src: u64 }
    83 ResolveSubresource { list: u64, dst: u64, dst_sub: u32, src: u64, src_sub: u32, format: u32 }
    84 BeginQuery { list: u64, heap: u64, query_type: u32, index: u32 }
    85 EndQuery { list: u64, heap: u64, query_type: u32, index: u32 }
    86 ResolveQueryData { list: u64, heap: u64, query_type: u32, start: u32, count: u32, dst: u64, dst_offset: u64 }
    /// `ID3D12GraphicsCommandList4::BuildRaytracingAccelerationStructure`.
    /// `geometries`: flattened `D3D12_RAYTRACING_GEOMETRY_DESC`s (56 B each) for BLAS;
    /// `postbuild`: `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC`s (16 B each).
    87 BuildRaytracingAccelerationStructure {
        list: u64,
        dest: u64,
        source: u64,
        scratch: u64,
        inputs_type: u32,
        flags: u32,
        num_descs: u32,
        descs_layout: u32,
        instances_va: u64,
        geometries: Blob<'a>,
        postbuild: Blob<'a>,
    }
    /// `ID3D12GraphicsCommandList4::EmitRaytracingAccelerationStructurePostbuildInfo`.
    88 EmitRaytracingAccelerationStructurePostbuildInfo { list: u64, dest_buffer: u64, info_type: u32, sources: U64s<'a> }
    /// `ID3D12GraphicsCommandList4::CopyRaytracingAccelerationStructure`.
    89 CopyRaytracingAccelerationStructure { list: u64, dest: u64, source: u64, mode: u32 }
    /// PIX/debug markers. `kind`: 0 = begin, 1 = end, 2 = set.
    90 Marker { list: u64, kind: u8, text: &'a str }
    /// `ID3D12GraphicsCommandList4::SetPipelineState1`.
    91 SetPipelineState1 { list: u64, state_object: u64 }
    /// `ID3D12GraphicsCommandList4::DispatchRays`. `desc`: `D3D12_DISPATCH_RAYS_DESC` (104 B).
    92 DispatchRays { list: u64, width: u32, height: u32, depth: u32, desc: Blob<'a> }
    /// `ID3D12GraphicsCommandList::ClearState`.
    93 ClearState { list: u64, pso: u64 }
    /// `ID3D12GraphicsCommandList::SetPredication`.
    94 SetPredication { list: u64, buffer: u64, aligned_offset: u64, operation: u32 }
    // Queues, fences, presentation
    100 ExecuteCommandLists { queue: u64, lists: U64s<'a> }
    101 QueueSignal { queue: u64, fence: u64, value: u64 }
    102 QueueWait { queue: u64, fence: u64, value: u64 }
    103 FenceSignal { fence: u64, value: u64 }
    /// `desc`: DXGI_SWAP_CHAIN_DESC1.
    110 CreateSwapChain { queue: u64, swapchain: u64, desc: Blob<'a> }
    111 GetSwapChainBuffer { swapchain: u64, index: u32, resource: u64 }
    112 ResizeBuffers { swapchain: u64, count: u32, width: u32, height: u32, format: u32, flags: u32 }
    113 Present { swapchain: u64, sync_interval: u32, flags: u32 }
    /// End of a frame boundary hint (written after Present returns).
    114 FrameEnd { frame: u64 }
}

// ---- writer -------------------------------------------------------------------------------------

/// Trace writer over any `Write`. Usually used through the global [`record`].
pub struct Writer<W: Write> {
    out: W,
    enc: Enc,
    start: Instant,
    records: u64,
}

impl<W: Write> Writer<W> {
    pub fn new(mut out: W) -> std::io::Result<Self> {
        out.write_all(MAGIC)?;
        out.write_all(&VERSION.to_le_bytes())?;
        out.write_all(&0u32.to_le_bytes())?;
        Ok(Self { out, enc: Enc::default(), start: Instant::now(), records: 0 })
    }

    pub fn write(&mut self, call: &Call, thread: u16) -> std::io::Result<()> {
        self.enc.buf.clear();
        call.encode(&mut self.enc);
        let mut h = [0u8; RECORD_HEADER_LEN];
        h[0..4].copy_from_slice(&(self.enc.buf.len() as u32).to_le_bytes());
        h[4..6].copy_from_slice(&call.op().to_le_bytes());
        h[6..8].copy_from_slice(&thread.to_le_bytes());
        h[8..16].copy_from_slice(&(self.start.elapsed().as_nanos() as u64).to_le_bytes());
        self.out.write_all(&h)?;
        self.out.write_all(&self.enc.buf)?;
        self.records += 1;
        Ok(())
    }

    pub fn records(&self) -> u64 {
        self.records
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
    pub fn into_inner(self) -> W {
        self.out
    }
}

type FileWriter = Writer<std::io::BufWriter<std::fs::File>>;

static STATE: AtomicU8 = AtomicU8::new(0); // 0 = unknown, 1 = off, 2 = on
static WRITER: OnceLock<Option<Mutex<FileWriter>>> = OnceLock::new();

fn writer() -> Option<&'static Mutex<FileWriter>> {
    WRITER
        .get_or_init(|| {
            let path = std::env::var_os("AW2_D3D12_TRACE").filter(|p| !p.is_empty())?;
            let w = std::fs::File::create(&path).and_then(|f| Writer::new(std::io::BufWriter::with_capacity(1 << 20, f)));
            match w {
                Ok(w) => {
                    log::info!("d3d12 trace: writing {}", std::path::Path::new(&path).display());
                    Some(Mutex::new(w))
                }
                Err(e) => {
                    log::warn!("d3d12 trace: cannot create {}: {e}", std::path::Path::new(&path).display());
                    None
                }
            }
        })
        .as_ref()
}

/// True when `AW2_D3D12_TRACE` is set and the file could be created. This is cheap after the
/// first call, so check it before building expensive `Call`s.
#[inline]
pub fn enabled() -> bool {
    match STATE.load(Ordering::Relaxed) {
        1 => false,
        2 => true,
        _ => {
            let on = writer().is_some();
            STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
            on
        }
    }
}

fn thread_index() -> u16 {
    static NEXT: AtomicU16 = AtomicU16::new(0);
    thread_local!(static IDX: u16 = NEXT.fetch_add(1, Ordering::Relaxed));
    IDX.with(|i| *i)
}

/// Appends `call` to the global trace (no-op when tracing is off).
pub fn record(call: &Call) {
    if !enabled() {
        return;
    }
    if let Some(w) = writer() {
        let t = thread_index();
        let mut w = w.lock().unwrap_or_else(|p| p.into_inner());
        if let Err(e) = w.write(call, t) {
            log::warn!("d3d12 trace: write failed: {e}; tracing stopped");
            STATE.store(1, Ordering::Relaxed);
        } else if matches!(call, Call::Present { .. } | Call::FrameEnd { .. }) {
            let _ = w.flush(); // a crash loses at most the current frame
        }
    }
}

/// Flushes the global trace (call at shutdown, `ExitProcess`).
pub fn flush() {
    if let Some(w) = WRITER.get().and_then(|w| w.as_ref()) {
        let _ = w.lock().unwrap_or_else(|p| p.into_inner()).flush();
    }
}

// ---- reader -------------------------------------------------------------------------------------

/// One decoded record.
#[derive(Debug, Clone, PartialEq)]
pub struct Record<'a> {
    /// Index in the trace (0-based).
    pub index: u64,
    pub thread: u16,
    pub t_ns: u64,
    pub call: Call<'a>,
}

/// Iterates over the records of a trace held in memory. A torn final record ends the
/// iteration; `truncated()` reports it.
pub struct Reader<'a> {
    dec: Dec<'a>,
    index: u64,
    truncated: bool,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, TraceError> {
        if data.len() < HEADER_LEN || &data[..8] != MAGIC {
            return err("not an aw2 d3d12 trace");
        }
        let v = u32::from_le_bytes(data[8..12].try_into().unwrap());
        if v != VERSION {
            return err(format!("trace version {v}, this build reads {VERSION}"));
        }
        Ok(Self { dec: Dec { data, pos: HEADER_LEN, blobs: Vec::new() }, index: 0, truncated: false })
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Record<'a>, TraceError>;
    fn next(&mut self) -> Option<Self::Item> {
        let d = &mut self.dec;
        let rest = d.data.len() - d.pos;
        if rest == 0 {
            return None;
        }
        if rest < RECORD_HEADER_LEN {
            self.truncated = true;
            return None;
        }
        let h = &d.data[d.pos..d.pos + RECORD_HEADER_LEN];
        let len = u32::from_le_bytes(h[0..4].try_into().unwrap()) as usize;
        let op = u16::from_le_bytes(h[4..6].try_into().unwrap());
        let thread = u16::from_le_bytes(h[6..8].try_into().unwrap());
        let t_ns = u64::from_le_bytes(h[8..16].try_into().unwrap());
        if rest - RECORD_HEADER_LEN < len {
            self.truncated = true;
            return None;
        }
        let start = d.pos + RECORD_HEADER_LEN;
        let end = start + len;
        // Decode within the record's bounds.
        let mut sub = Dec { data: &d.data[..end], pos: start, blobs: std::mem::take(&mut d.blobs) };
        let r = Call::decode(op, &mut sub).and_then(|call| {
            if sub.pos != end {
                return err(format!("record {} ({}) has {} trailing bytes", self.index, Call::op_name(op).unwrap_or("?"), end - sub.pos));
            }
            Ok(call)
        });
        d.blobs = sub.blobs;
        d.pos = end;
        let index = self.index;
        self.index += 1;
        Some(r.map(|call| Record { index, thread, t_ns, call }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_calls<'a>(shader: &'a [u8], il: &'a [u8], ids: &'a [u64], consts: &'a [u32], rtv: &'a [u32]) -> Vec<Call<'a>> {
        vec![
            Call::CreateDevice { device: 0x1000, feature_level: 0xc200 },
            Call::CreateCommittedResource {
                device: 0x1000,
                resource: 0x2000,
                heap_properties: &[1, 0, 0, 0],
                heap_flags: 0,
                desc: &[7; 56],
                initial_state: 0,
                clear_value: None,
                gpu_va: 0x4_0000_0000,
            },
            Call::CreateGraphicsPipelineState {
                device: 0x1000,
                pso: 0x3000,
                desc: GraphicsPipeline {
                    root_signature: 0x2800,
                    vs: shader,
                    ps: shader, // deduplicated
                    ds: &[],
                    hs: &[],
                    gs: &[],
                    stream_output: &[],
                    blend: &[0; 328],
                    sample_mask: !0,
                    rasterizer: &[1; 44],
                    raster_version: 0,
                    depth_stencil: &[2; 52],
                    depth_stencil_version: 0,
                    input_layout: il,
                    ib_strip_cut_value: 0,
                    primitive_topology_type: 3,
                    rtv_formats: U32s::from_slice(rtv),
                    dsv_format: 40,
                    sample_count: 1,
                    sample_quality: 0,
                    node_mask: 0,
                    flags: 0,
                },
            },
            Call::CreateComputePipelineState { device: 0x1000, pso: 0x3100, root_signature: 0x2800, cs: shader, flags: 0 },
            Call::SetDescriptorHeaps { list: 0x5000, heaps: U64s::from_slice(ids) },
            Call::SetRoot32BitConstants { list: 0x5000, compute: true, param: 1, dest_offset: 2, values: U32s::from_slice(consts) },
            Call::OMSetBlendFactor { list: 0x5000, factor: [0.25, 0.5, 0.75, 1.0] },
            Call::OMSetFrontAndBackStencilRef { list: 0x5000, front: 0x11, back: 0x22 },
            Call::ClearState { list: 0x5000, pso: 0x3000 },
            Call::SetPredication { list: 0x5000, buffer: 0x2000, aligned_offset: 64, operation: 0 },
            Call::ClearRenderTargetView {
                list: 0x5000,
                rtv: 0x88,
                color: [0.1, 0.2, 0.3, 1.0],
                rects: &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0],
            },
            Call::ClearDepthStencilView {
                list: 0x5000,
                dsv: 0x77,
                flags: 3,
                depth: 0.0,
                stencil: 0x80,
                rects: &[0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0],
            },
            Call::DrawIndexedInstanced { list: 0x5000, index_count: 36, instance_count: 1, start_index: 0, base_vertex: -4, start_instance: 0 },
            Call::Marker { list: 0x5000, kind: 0, text: "GBuffer" },
            Call::IASetIndexBuffer { list: 0x5000, view: Some(&[9; 16]) },
            Call::ExecuteCommandLists { queue: 0x6000, lists: U64s::from_slice(ids) },
            Call::CreateMeshPipelineState {
                device: 0x1000,
                pso: 0x3200,
                amplification: &[],
                mesh: shader,
                desc: GraphicsPipeline {
                    root_signature: 0x2800,
                    vs: &[],
                    ps: shader,
                    ds: &[],
                    hs: &[],
                    gs: &[],
                    stream_output: &[],
                    blend: &[0; 328],
                    sample_mask: !0,
                    rasterizer: &[1; 40],
                    raster_version: 2,
                    depth_stencil: &[2; 60],
                    depth_stencil_version: 2,
                    input_layout: &[],
                    ib_strip_cut_value: 0,
                    primitive_topology_type: 0,
                    rtv_formats: U32s::from_slice(rtv),
                    dsv_format: 0,
                    sample_count: 1,
                    sample_quality: 0,
                    node_mask: 0,
                    flags: 0,
                },
            },
            Call::DispatchMesh { list: 0x5000, x: 8, y: 2, z: 1 },
            Call::BuildRaytracingAccelerationStructure {
                list: 0x5000,
                dest: 0x4_0001_0000,
                source: 0,
                scratch: 0x4_0002_0000,
                inputs_type: 1,
                flags: 4,
                num_descs: 1,
                descs_layout: 0,
                instances_va: 0,
                geometries: &[0; 56],
                postbuild: &[0; 16],
            },
            Call::EmitRaytracingAccelerationStructurePostbuildInfo {
                list: 0x5000,
                dest_buffer: 0x4_0003_0000,
                info_type: 0,
                sources: U64s::from_slice(ids),
            },
            Call::CopyRaytracingAccelerationStructure {
                list: 0x5000,
                dest: 0x4_0004_0000,
                source: 0x4_0001_0000,
                mode: 1,
            },
            Call::CreateStateObject {
                device: 0x1000,
                state_object: 0x3300,
                state_object_type: 0,
                num_subobjects: 2,
                num_exports: 3,
                num_hit_groups: 1,
            },
            Call::AddToStateObject {
                device: 0x1000,
                state_object: 0x3400,
                base: 0x3300,
                state_object_type: 3,
                num_subobjects: 4,
                num_exports: 4,
                num_hit_groups: 2,
            },
            Call::SetPipelineState1 { list: 0x5000, state_object: 0x3400 },
            Call::DispatchRays {
                list: 0x5000,
                width: 64,
                height: 32,
                depth: 1,
                desc: &[0; 104],
            },
            Call::Present { swapchain: 0x7000, sync_interval: 1, flags: 0 },
        ]
    }

    #[test]
    fn round_trip_with_dedup() {
        let shader: Vec<u8> = (0..4096u32).map(|i| (i * 7) as u8).collect();
        let il = encode_input_layout(&[InputElement {
            semantic_name: "POSITION".into(),
            semantic_index: 0,
            format: 6,
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: 0,
            instance_data_step_rate: 0,
        }]);
        let ids = [0x10u64, 0x20];
        let consts = [1u32, 2, 3];
        let rtv = [28u32, 10];
        let calls = sample_calls(&shader, &il, &ids, &consts, &rtv);
        let mut w = Writer::new(Vec::new()).unwrap();
        for (i, c) in calls.iter().enumerate() {
            w.write(c, (i % 3) as u16).unwrap();
        }
        assert_eq!(w.records(), calls.len() as u64);
        let bytes = w.into_inner();
        // The 4 KiB shader appears five times but is stored once.
        assert!(bytes.len() < shader.len() + 2560, "trace is {} bytes", bytes.len());

        let r = Reader::new(&bytes).unwrap();
        let back: Vec<Record> = r.map(|r| r.unwrap()).collect();
        assert_eq!(back.len(), calls.len());
        for (i, (rec, c)) in back.iter().zip(&calls).enumerate() {
            assert_eq!(&rec.call, c, "record {i}");
            assert_eq!(rec.thread, (i % 3) as u16);
            assert_eq!(rec.index, i as u64);
        }
        if let Call::CreateGraphicsPipelineState { desc, .. } = &back[2].call {
            assert_eq!(desc.rtv_formats.to_vec(), rtv);
            let layout = decode_input_layout(desc.input_layout).unwrap();
            assert_eq!(layout[0].semantic_name, "POSITION");
        } else {
            panic!()
        }
        assert!(back.windows(2).all(|w| w[0].t_ns <= w[1].t_ns));
    }

    #[test]
    fn torn_tail_and_bad_input() {
        let calls = sample_calls(&[1, 2, 3], &[0, 0, 0, 0], &[], &[], &[]);
        let mut w = Writer::new(Vec::new()).unwrap();
        for c in &calls {
            w.write(c, 0).unwrap();
        }
        let bytes = w.into_inner();
        let cut = &bytes[..bytes.len() - 3];
        let mut r = Reader::new(cut).unwrap();
        let n = r.by_ref().filter(|x| x.is_ok()).count();
        assert_eq!(n, calls.len() - 1);
        assert!(r.truncated());
        assert!(Reader::new(b"nope").is_err());
        let mut v = bytes.clone();
        v[8] = 99;
        assert!(Reader::new(&v).is_err());
        // An unknown op is an error but doesn't derail the reader.
        let mut v = bytes[..HEADER_LEN].to_vec();
        v.extend_from_slice(&[0, 0, 0, 0, 0xEE, 0xEE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut r = Reader::new(&v).unwrap();
        assert!(r.next().unwrap().is_err());
        assert!(r.next().is_none());
    }

    #[test]
    fn op_numbers_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for op in 0..=u16::MAX {
            if let Some(n) = Call::op_name(op) {
                assert!(seen.insert(n), "{n} listed twice");
            }
        }
        assert!(seen.len() > 60);
    }
}
