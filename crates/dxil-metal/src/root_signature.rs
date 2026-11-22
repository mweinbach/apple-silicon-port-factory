//! D3D12 root signatures: a Rust model of `D3D12_VERSIONED_ROOT_SIGNATURE_DESC` (1.0/1.1/1.2),
//! the serialized blob format (`RTS0`, as produced by `D3D12SerializeVersionedRootSignature` and
//! consumed by `ID3D12Device::CreateRootSignature`), conversion to Metal Shader Converter root
//! signatures, and the resulting top-level argument buffer layout.

use std::sync::{Arc, OnceLock};

use crate::dxbc::{self, Container, FOURCC_RTS0};
use crate::ffi::{self, *};
use crate::hash::hash128;
use crate::runtime::DESCRIPTOR_TABLE_ENTRY_SIZE;
use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RootSignatureVersion {
    V1_0 = 1,
    V1_1 = 2,
    V1_2 = 3,
}

impl RootSignatureVersion {
    pub fn from_raw(v: u32) -> Option<Self> {
        match v {
            1 => Some(Self::V1_0),
            2 => Some(Self::V1_1),
            3 => Some(Self::V1_2),
            _ => None,
        }
    }
}

/// `D3D12_DESCRIPTOR_RANGE_TYPE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorRangeType {
    Srv = 0,
    Uav = 1,
    Cbv = 2,
    Sampler = 3,
}

impl DescriptorRangeType {
    pub fn from_raw(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Srv),
            1 => Some(Self::Uav),
            2 => Some(Self::Cbv),
            3 => Some(Self::Sampler),
            _ => None,
        }
    }
}

/// `D3D12_ROOT_SIGNATURE_FLAGS` bits used by the translation layer.
pub mod flags {
    pub const ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT: u32 = 0x1;
    pub const LOCAL_ROOT_SIGNATURE: u32 = 0x80;
    pub const CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED: u32 = 0x400;
    pub const SAMPLER_HEAP_DIRECTLY_INDEXED: u32 = 0x800;
    /// `D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND`.
    pub const DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = 0xFFFF_FFFF;
    /// `NumDescriptors` value for an unbounded range.
    pub const UNBOUNDED: u32 = 0xFFFF_FFFF;
    /// 1.0 semantics expressed as 1.1 flags (D3D12 spec "Root Signature Version 1.1").
    pub const RANGE_V1_0_DEFAULT_CBV_SRV_UAV: u32 = 0x1 | 0x2; // DESCRIPTORS_VOLATILE | DATA_VOLATILE
    pub const RANGE_V1_0_DEFAULT_SAMPLER: u32 = 0x1; // DESCRIPTORS_VOLATILE
    pub const ROOT_DESCRIPTOR_V1_0_DEFAULT: u32 = 0x2; // DATA_VOLATILE
    /// `D3D12_SAMPLER_FLAG_UINT_BORDER_COLOR` (1.2 static samplers).
    pub const SAMPLER_FLAG_UINT_BORDER_COLOR: u32 = 0x1;
    pub const SAMPLER_FLAG_NON_NORMALIZED_COORDINATES: u32 = 0x2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorRange {
    pub range_type: DescriptorRangeType,
    /// [`flags::UNBOUNDED`] for unbounded.
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    /// `D3D12_DESCRIPTOR_RANGE_FLAGS` (0 for 1.0 blobs).
    pub flags: u32,
    /// [`flags::DESCRIPTOR_RANGE_OFFSET_APPEND`] or an explicit descriptor offset.
    pub offset_in_descriptors_from_table_start: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RootDescriptor {
    pub shader_register: u32,
    pub register_space: u32,
    /// `D3D12_ROOT_DESCRIPTOR_FLAGS` (0 for 1.0 blobs).
    pub flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RootDescriptorKind {
    Cbv,
    Srv,
    Uav,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RootParameterKind {
    DescriptorTable(Vec<DescriptorRange>),
    Constants { shader_register: u32, register_space: u32, num_32bit_values: u32 },
    Descriptor(RootDescriptorKind, RootDescriptor),
}

impl RootParameterKind {
    /// `D3D12_ROOT_PARAMETER_TYPE`.
    pub fn raw_type(&self) -> u32 {
        match self {
            Self::DescriptorTable(_) => 0,
            Self::Constants { .. } => 1,
            Self::Descriptor(RootDescriptorKind::Cbv, _) => 2,
            Self::Descriptor(RootDescriptorKind::Srv, _) => 3,
            Self::Descriptor(RootDescriptorKind::Uav, _) => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootParameter {
    pub kind: RootParameterKind,
    /// `D3D12_SHADER_VISIBILITY` (0 = all).
    pub visibility: u32,
}

/// `D3D12_STATIC_SAMPLER_DESC1` (1.0/1.1 blobs have `flags == 0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StaticSampler {
    pub filter: u32,
    pub address_u: u32,
    pub address_v: u32,
    pub address_w: u32,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: u32,
    pub border_color: u32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_register: u32,
    pub register_space: u32,
    pub visibility: u32,
    pub flags: u32,
}

impl Default for StaticSampler {
    fn default() -> Self {
        Self {
            filter: 0x15, // MIN_MAG_MIP_LINEAR
            address_u: 1,
            address_v: 1,
            address_w: 1,
            mip_lod_bias: 0.0,
            max_anisotropy: 1,
            comparison_func: 1,
            border_color: 0,
            min_lod: 0.0,
            max_lod: f32::MAX,
            shader_register: 0,
            register_space: 0,
            visibility: 0,
            flags: 0,
        }
    }
}

/// A complete root signature description.
#[derive(Debug, Clone, PartialEq)]
pub struct RootSignatureDesc {
    pub version: RootSignatureVersion,
    pub parameters: Vec<RootParameter>,
    pub static_samplers: Vec<StaticSampler>,
    pub flags: u32,
}

// ---- serialized format -----------------------------------------------------------------------

struct Rd<'a>(&'a [u8]);
impl Rd<'_> {
    fn u32(&self, off: usize) -> Result<u32, Error> {
        self.0
            .get(off..off + 4)
            .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
            .ok_or_else(|| Error::InvalidInput(format!("root signature: read past end at {off:#x}")))
    }
    fn f32(&self, off: usize) -> Result<f32, Error> {
        self.u32(off).map(f32::from_bits)
    }
}

impl RootSignatureDesc {
    /// Parses a serialized root signature: a DXBC container with an `RTS0` part (what
    /// `D3D12SerializeRootSignature` returns, and what shaders with embedded root signatures
    /// carry) or a bare `RTS0` payload.
    pub fn from_blob(blob: &[u8]) -> Result<Self, Error> {
        if Container::is_container(blob) {
            let c = Container::parse(blob)?;
            let rts0 = c
                .part(FOURCC_RTS0)
                .ok_or_else(|| Error::InvalidInput("DXBC container has no RTS0 part".into()))?;
            Self::from_rts0(rts0)
        } else {
            Self::from_rts0(blob)
        }
    }

    /// Extracts the root signature embedded in a DXIL shader, if any.
    pub fn from_dxil(dxil: &[u8]) -> Result<Option<Self>, Error> {
        let c = Container::parse(dxil)?;
        c.part(FOURCC_RTS0).map(Self::from_rts0).transpose()
    }

    /// Parses a bare `RTS0` payload.
    pub fn from_rts0(b: &[u8]) -> Result<Self, Error> {
        let r = Rd(b);
        let bad = |m: String| Error::InvalidInput(format!("root signature: {m}"));
        let raw_version = r.u32(0)?;
        let version = RootSignatureVersion::from_raw(raw_version).ok_or_else(|| bad(format!("unknown version {raw_version:#x}")))?;
        let v11 = version >= RootSignatureVersion::V1_1;
        let num_params = r.u32(4)? as usize;
        let params_off = r.u32(8)? as usize;
        let num_samplers = r.u32(12)? as usize;
        let samplers_off = r.u32(16)? as usize;
        let flags = r.u32(20)?;
        if num_params > 64 * 1024 || num_samplers > 64 * 1024 {
            return Err(bad("implausible counts".into()));
        }
        let mut parameters = Vec::with_capacity(num_params);
        for i in 0..num_params {
            let p = params_off + i * 12;
            let ty = r.u32(p)?;
            let visibility = r.u32(p + 4)?;
            let payload = r.u32(p + 8)? as usize;
            let kind = match ty {
                0 => {
                    let n = r.u32(payload)? as usize;
                    let off = r.u32(payload + 4)? as usize;
                    if n > 64 * 1024 {
                        return Err(bad("implausible range count".into()));
                    }
                    let stride = if v11 { 24 } else { 20 };
                    let mut ranges = Vec::with_capacity(n);
                    for j in 0..n {
                        let q = off + j * stride;
                        let rt = r.u32(q)?;
                        ranges.push(DescriptorRange {
                            range_type: DescriptorRangeType::from_raw(rt).ok_or_else(|| bad(format!("bad range type {rt}")))?,
                            num_descriptors: r.u32(q + 4)?,
                            base_shader_register: r.u32(q + 8)?,
                            register_space: r.u32(q + 12)?,
                            flags: if v11 { r.u32(q + 16)? } else { 0 },
                            offset_in_descriptors_from_table_start: r.u32(q + if v11 { 20 } else { 16 })?,
                        });
                    }
                    RootParameterKind::DescriptorTable(ranges)
                }
                1 => RootParameterKind::Constants {
                    shader_register: r.u32(payload)?,
                    register_space: r.u32(payload + 4)?,
                    num_32bit_values: r.u32(payload + 8)?,
                },
                2..=4 => RootParameterKind::Descriptor(
                    [RootDescriptorKind::Cbv, RootDescriptorKind::Srv, RootDescriptorKind::Uav][ty as usize - 2],
                    RootDescriptor {
                        shader_register: r.u32(payload)?,
                        register_space: r.u32(payload + 4)?,
                        flags: if v11 { r.u32(payload + 8)? } else { 0 },
                    },
                ),
                t => return Err(bad(format!("unknown root parameter type {t}"))),
            };
            parameters.push(RootParameter { kind, visibility });
        }
        let sstride = if version >= RootSignatureVersion::V1_2 { 56 } else { 52 };
        let mut static_samplers = Vec::with_capacity(num_samplers);
        for i in 0..num_samplers {
            let s = samplers_off + i * sstride;
            static_samplers.push(StaticSampler {
                filter: r.u32(s)?,
                address_u: r.u32(s + 4)?,
                address_v: r.u32(s + 8)?,
                address_w: r.u32(s + 12)?,
                mip_lod_bias: r.f32(s + 16)?,
                max_anisotropy: r.u32(s + 20)?,
                comparison_func: r.u32(s + 24)?,
                border_color: r.u32(s + 28)?,
                min_lod: r.f32(s + 32)?,
                max_lod: r.f32(s + 36)?,
                shader_register: r.u32(s + 40)?,
                register_space: r.u32(s + 44)?,
                visibility: r.u32(s + 48)?,
                flags: if sstride == 56 { r.u32(s + 52)? } else { 0 },
            });
        }
        Ok(Self { version, parameters, static_samplers, flags })
    }

    /// Serializes to a bare `RTS0` payload in the layout DXC/d3d12 use:
    /// header, parameter array, per-parameter payloads (tables followed by their ranges),
    /// then static samplers.
    pub fn to_rts0(&self) -> Vec<u8> {
        let v11 = self.version >= RootSignatureVersion::V1_1;
        let mut out = vec![0u8; 24 + self.parameters.len() * 12];
        let put = |out: &mut Vec<u8>, at: usize, v: u32| out[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put(&mut out, 0, self.version as u32);
        put(&mut out, 4, self.parameters.len() as u32);
        put(&mut out, 8, 24);
        put(&mut out, 12, self.static_samplers.len() as u32);
        put(&mut out, 20, self.flags);
        for (i, p) in self.parameters.iter().enumerate() {
            let at = 24 + i * 12;
            put(&mut out, at, p.kind.raw_type());
            put(&mut out, at + 4, p.visibility);
            let payload_off = out.len() as u32;
            put(&mut out, at + 8, payload_off);
            let push = |out: &mut Vec<u8>, v: u32| out.extend_from_slice(&v.to_le_bytes());
            match &p.kind {
                RootParameterKind::DescriptorTable(ranges) => {
                    push(&mut out, ranges.len() as u32);
                    let ranges_off = out.len() as u32 + 4;
                    push(&mut out, if ranges.is_empty() { 0 } else { ranges_off });
                    for r in ranges {
                        push(&mut out, r.range_type as u32);
                        push(&mut out, r.num_descriptors);
                        push(&mut out, r.base_shader_register);
                        push(&mut out, r.register_space);
                        if v11 {
                            push(&mut out, r.flags);
                        }
                        push(&mut out, r.offset_in_descriptors_from_table_start);
                    }
                }
                RootParameterKind::Constants { shader_register, register_space, num_32bit_values } => {
                    push(&mut out, *shader_register);
                    push(&mut out, *register_space);
                    push(&mut out, *num_32bit_values);
                }
                RootParameterKind::Descriptor(_, d) => {
                    push(&mut out, d.shader_register);
                    push(&mut out, d.register_space);
                    if v11 {
                        push(&mut out, d.flags);
                    }
                }
            }
        }
        let soff = out.len() as u32;
        put(&mut out, 16, if self.static_samplers.is_empty() { 0 } else { soff });
        for s in &self.static_samplers {
            for v in [
                s.filter,
                s.address_u,
                s.address_v,
                s.address_w,
                s.mip_lod_bias.to_bits(),
                s.max_anisotropy,
                s.comparison_func,
                s.border_color,
                s.min_lod.to_bits(),
                s.max_lod.to_bits(),
                s.shader_register,
                s.register_space,
                s.visibility,
            ] {
                out.extend_from_slice(&v.to_le_bytes());
            }
            if self.version >= RootSignatureVersion::V1_2 {
                out.extend_from_slice(&s.flags.to_le_bytes());
            }
        }
        out
    }

    /// Serializes to a DXBC container with a single `RTS0` part (the `ID3DBlob` contents
    /// `D3D12SerializeVersionedRootSignature` returns).
    pub fn to_blob(&self) -> Vec<u8> {
        dxbc::build_container(&[(FOURCC_RTS0, &self.to_rts0())])
    }

    /// Stable hash of the canonical serialization (independent of container digest/padding).
    pub fn canonical_hash(&self) -> u128 {
        hash128(&self.to_rts0())
    }

    /// Computes the top-level argument buffer layout (pure Rust, no converter needed).
    pub fn layout(&self) -> RootSignatureLayout {
        RootSignatureLayout::compute(self)
    }

    /// Diagnostic helper: a 1.1 root signature that binds every resource a shader reports in its
    /// linear-layout reflection (compiled without a root signature) through two descriptor
    /// tables — CBV/SRV/UAV, then samplers. Game shaders carry no embedded root signature (the
    /// game builds them at runtime), so tests and the prewarm tool use this to exercise the
    /// root-signature code path on real DXIL.
    pub fn covering(resources: &[ResourceLocation]) -> Self {
        Self::covering_bindings(resources.iter().filter_map(|r| {
            let t = match r.resource_type {
                ResourceType::Srv => DescriptorRangeType::Srv,
                ResourceType::Uav => DescriptorRangeType::Uav,
                ResourceType::Cbv => DescriptorRangeType::Cbv,
                ResourceType::Sampler => DescriptorRangeType::Sampler,
                _ => return None,
            };
            Some((t, r.space, r.slot, (r.size_bytes / DESCRIPTOR_TABLE_ENTRY_SIZE as u64).max(1) as u32))
        }))
    }

    /// Like [`covering`](Self::covering), from the DXIL's own binding tables (PSV0, or RDAT for
    /// libraries) — works for shaders the converter refuses to compile without a root signature.
    /// For SM4/SM5 (`SHEX`/`SHDR`) containers, converts via `dxilconv` first to read `PSV0`.
    pub fn covering_dxil(dxil: &[u8]) -> Option<Self> {
        let parsed = Container::parse(dxil).ok()?;
        let converted;
        let container = if parsed.is_sm4_bytecode() {
            converted = crate::dxilconv::convert_signed(dxil).ok()?;
            Container::parse(&converted).ok()?
        } else {
            parsed
        };
        let res = container.resource_bindings()?;
        Some(Self::covering_bindings(res.iter().filter_map(|r| {
            let t = DescriptorRangeType::from_raw(r.range_type)?;
            let n = if r.upper_bound == u32::MAX { flags::UNBOUNDED } else { r.upper_bound.saturating_sub(r.lower_bound) + 1 };
            Some((t, r.space, r.lower_bound, n))
        })))
    }

    /// Two-table root signature from `(type, space, base register, count)` bindings.
    pub fn covering_bindings(bindings: impl IntoIterator<Item = (DescriptorRangeType, u32, u32, u32)>) -> Self {
        let mut views: Vec<DescriptorRange> = Vec::new();
        let mut samplers: Vec<DescriptorRange> = Vec::new();
        for (range_type, space, reg, n) in bindings {
            let range = DescriptorRange {
                range_type,
                num_descriptors: n,
                base_shader_register: reg,
                register_space: space,
                flags: 0,
                offset_in_descriptors_from_table_start: flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
            };
            let list = if range_type == DescriptorRangeType::Sampler { &mut samplers } else { &mut views };
            // An unbounded range must be last in its table (APPEND after it is invalid).
            if n == flags::UNBOUNDED {
                list.push(range);
            } else {
                let pos = list.iter().position(|r| r.num_descriptors == flags::UNBOUNDED).unwrap_or(list.len());
                list.insert(pos, range);
            }
        }
        let mut parameters = Vec::new();
        for mut t in [views, samplers] {
            // Each unbounded range gets its own table so it can be last.
            while t.len() > 1 && t[t.len() - 2].num_descriptors == flags::UNBOUNDED {
                let r = t.remove(t.len() - 2);
                parameters.push(RootParameter { kind: RootParameterKind::DescriptorTable(vec![r]), visibility: 0 });
            }
            if !t.is_empty() {
                parameters.push(RootParameter { kind: RootParameterKind::DescriptorTable(t), visibility: 0 });
            }
        }
        Self {
            version: RootSignatureVersion::V1_1,
            parameters,
            static_samplers: Vec::new(),
            flags: flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        }
    }
}

// ---- argument buffer layout ------------------------------------------------------------------

/// Where a root parameter lives in the top-level argument buffer
/// (bound at `kIRArgumentBufferBindPoint`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootParameterLayout {
    /// Byte offset in the top-level argument buffer.
    pub offset: u32,
    /// Byte size of the entry: 8 for tables / root descriptors (a GPU address), `4 * N` for constants.
    pub size: u32,
    pub visibility: u32,
    pub kind: RootParameterLayoutKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootParameterLayoutKind {
    /// The entry holds the GPU address of an array of `IRDescriptorTableEntry` (24 bytes each),
    /// i.e. `heap_base + descriptor_index * 24`.
    DescriptorTable { sampler: bool, ranges: Vec<TableRangeLayout> },
    /// The entry holds `num_32bit_values` inline dwords.
    Constants { shader_register: u32, register_space: u32, num_32bit_values: u32 },
    /// The entry holds a raw GPU virtual address (`D3D12_GPU_VIRTUAL_ADDRESS` translated to Metal).
    Descriptor { kind: RootDescriptorKind, shader_register: u32, register_space: u32 },
}

/// A descriptor range with its `APPEND` offset resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableRangeLayout {
    pub range_type: DescriptorRangeType,
    pub base_shader_register: u32,
    pub register_space: u32,
    /// `None` = unbounded.
    pub num_descriptors: Option<u32>,
    /// Resolved offset (in descriptors) from the table start.
    pub descriptor_offset: u32,
    /// `descriptor_offset * 24`.
    pub byte_offset: u64,
}

/// Static samplers: the converter appends one 8-byte entry after the root parameters holding
/// the GPU address of an `IRDescriptorTableEntry` array with one sampler per static sampler, in
/// root-signature declaration order (encode with `DescriptorTableEntry::sampler`, using the
/// sampler's `mip_lod_bias` as `lod_bias`). The D3D12 layer creates these samplers once per root
/// signature. Verified on the GPU by `tests/gpu_static_samplers.rs`: declaration order (not
/// register order) is what the converted shader reads, and the samplers are not embedded in the
/// shader. The generated code is identical to a sampler descriptor table at the same offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticSamplerTableLayout {
    /// Byte offset of the table pointer in the top-level argument buffer.
    pub offset: u32,
    /// Number of samplers in the table.
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootSignatureLayout {
    pub parameters: Vec<RootParameterLayout>,
    /// Present iff the root signature declares static samplers.
    pub static_samplers: Option<StaticSamplerTableLayout>,
    /// Total size of the top-level argument buffer in bytes.
    pub size: u32,
    pub flags: u32,
}

impl RootSignatureLayout {
    /// Metal Shader Converter's top-level argument buffer rule: root parameters are laid out in
    /// declaration order; descriptor tables and root descriptors take one `uint64_t` (8-byte
    /// aligned), root constants take `Num32BitValues` dwords (4-byte aligned); if there are static
    /// samplers, one more 8-byte-aligned table pointer follows (see [`StaticSamplerTableLayout`]).
    /// (Cross-checked against `IRRootSignatureGetResourceLocations` in tests.)
    pub fn compute(desc: &RootSignatureDesc) -> Self {
        let mut off = 0u32;
        let mut parameters = Vec::with_capacity(desc.parameters.len());
        for p in &desc.parameters {
            let (align, size, kind) = match &p.kind {
                RootParameterKind::DescriptorTable(ranges) => {
                    let mut next = 0u32;
                    let mut out = Vec::with_capacity(ranges.len());
                    for r in ranges {
                        let start = if r.offset_in_descriptors_from_table_start == flags::DESCRIPTOR_RANGE_OFFSET_APPEND {
                            next
                        } else {
                            r.offset_in_descriptors_from_table_start
                        };
                        let count = (r.num_descriptors != flags::UNBOUNDED).then_some(r.num_descriptors);
                        next = start.saturating_add(count.unwrap_or(0));
                        out.push(TableRangeLayout {
                            range_type: r.range_type,
                            base_shader_register: r.base_shader_register,
                            register_space: r.register_space,
                            num_descriptors: count,
                            descriptor_offset: start,
                            byte_offset: start as u64 * DESCRIPTOR_TABLE_ENTRY_SIZE as u64,
                        });
                    }
                    let sampler = ranges.first().is_some_and(|r| r.range_type == DescriptorRangeType::Sampler);
                    (8, 8, RootParameterLayoutKind::DescriptorTable { sampler, ranges: out })
                }
                RootParameterKind::Constants { shader_register, register_space, num_32bit_values } => (
                    4,
                    4 * num_32bit_values,
                    RootParameterLayoutKind::Constants {
                        shader_register: *shader_register,
                        register_space: *register_space,
                        num_32bit_values: *num_32bit_values,
                    },
                ),
                RootParameterKind::Descriptor(kind, d) => (
                    8,
                    8,
                    RootParameterLayoutKind::Descriptor {
                        kind: *kind,
                        shader_register: d.shader_register,
                        register_space: d.register_space,
                    },
                ),
            };
            off = off.next_multiple_of(align);
            parameters.push(RootParameterLayout { offset: off, size, visibility: p.visibility, kind });
            off += size;
        }
        let static_samplers = (!desc.static_samplers.is_empty()).then(|| {
            off = off.next_multiple_of(8);
            let t = StaticSamplerTableLayout { offset: off, count: desc.static_samplers.len() as u32 };
            off += 8;
            t
        });
        Self { parameters, static_samplers, size: off, flags: desc.flags }
    }

    /// A zeroed top-level argument buffer image of the right size.
    pub fn new_argument_buffer(&self) -> TopLevelArgumentBuffer {
        TopLevelArgumentBuffer { bytes: vec![0; self.size.max(8) as usize] }
    }
}

/// CPU-side image of a top-level argument buffer (what `SetGraphicsRoot*` calls write into).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopLevelArgumentBuffer {
    pub bytes: Vec<u8>,
}

impl TopLevelArgumentBuffer {
    fn put(&mut self, at: u32, b: &[u8]) {
        self.bytes[at as usize..at as usize + b.len()].copy_from_slice(b);
    }
    /// `SetGraphicsRootDescriptorTable`: `table_gpu_address = heap.gpuAddress + index * 24`.
    pub fn set_descriptor_table(&mut self, layout: &RootSignatureLayout, param: usize, table_gpu_address: u64) {
        let p = &layout.parameters[param];
        debug_assert!(matches!(p.kind, RootParameterLayoutKind::DescriptorTable { .. }));
        self.put(p.offset, &table_gpu_address.to_le_bytes());
    }
    /// Points the static sampler slot at the root signature's sampler table.
    pub fn set_static_sampler_table(&mut self, layout: &RootSignatureLayout, table_gpu_address: u64) {
        let t = layout.static_samplers.expect("root signature has no static samplers");
        self.put(t.offset, &table_gpu_address.to_le_bytes());
    }
    /// `SetGraphicsRootConstantBufferView/ShaderResourceView/UnorderedAccessView`.
    pub fn set_root_descriptor(&mut self, layout: &RootSignatureLayout, param: usize, gpu_va: u64) {
        let p = &layout.parameters[param];
        debug_assert!(matches!(p.kind, RootParameterLayoutKind::Descriptor { .. }));
        self.put(p.offset, &gpu_va.to_le_bytes());
    }
    /// `SetGraphicsRoot32BitConstants(param, values, dest_offset_in_32bit_values)`.
    pub fn set_constants(&mut self, layout: &RootSignatureLayout, param: usize, dest_offset: u32, values: &[u32]) {
        let p = &layout.parameters[param];
        let RootParameterLayoutKind::Constants { num_32bit_values, .. } = p.kind else {
            panic!("root parameter {param} is not root constants");
        };
        assert!(dest_offset as usize + values.len() <= num_32bit_values as usize, "root constants out of range");
        for (i, v) in values.iter().enumerate() {
            self.put(p.offset + (dest_offset + i as u32) * 4, &v.to_le_bytes());
        }
    }
}

// ---- converter root signature ----------------------------------------------------------------

/// A root signature ready for compilation: parsed description, layout, and the converter object.
pub struct RootSignature {
    desc: RootSignatureDesc,
    layout: RootSignatureLayout,
    hash: u128,
    /// The converter object, created on first use (never, when compiling in the helper
    /// process). Stored as an address so the cell is Send + Sync.
    ir: OnceLock<Result<usize, Error>>,
}

impl Drop for RootSignature {
    fn drop(&mut self) {
        if let Some(Ok(p)) = self.ir.get() {
            if let Ok(api) = ffi::api() {
                // SAFETY: created by IRRootSignatureCreateFromDescriptor and owned by us.
                unsafe { (api.IRRootSignatureDestroy)(*p as *mut ffi::IRRootSignature) };
            }
        }
    }
}

impl std::fmt::Debug for RootSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RootSignature").field("hash", &format_args!("{:032x}", self.hash)).field("desc", &self.desc).finish()
    }
}

/// Converter resource location (`IRResourceLocation`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLocation {
    pub resource_type: ResourceType,
    pub space: u32,
    pub slot: u32,
    pub top_level_offset: u32,
    pub size_bytes: u64,
    pub name: String,
}

/// `IRResourceType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Table,
    Constant,
    Cbv,
    Srv,
    Uav,
    Sampler,
    Invalid,
}

impl ResourceType {
    pub fn from_raw(v: u32) -> Self {
        match v {
            0 => Self::Table,
            1 => Self::Constant,
            2 => Self::Cbv,
            3 => Self::Srv,
            4 => Self::Uav,
            5 => Self::Sampler,
            _ => Self::Invalid,
        }
    }
    pub fn to_raw(self) -> u32 {
        self as u32
    }
}

pub(crate) unsafe fn read_locations(ptr: &[IRResourceLocation]) -> Vec<ResourceLocation> {
    ptr.iter()
        .map(|l| ResourceLocation {
            resource_type: ResourceType::from_raw(l.resourceType),
            space: l.space,
            slot: l.slot,
            top_level_offset: l.topLevelOffset,
            size_bytes: l.sizeBytes,
            name: cstr(l.resourceName),
        })
        .collect()
}

pub(crate) fn take_error(api: &Api, err: *mut IRError, what: &str) -> Error {
    if err.is_null() {
        return Error::Convert { code: ErrorCode::Unknown, message: what.to_string() };
    }
    let code = unsafe { (api.IRErrorGetCode)(err) };
    unsafe { (api.IRErrorDestroy)(err) };
    let code = ErrorCode::from_raw(code);
    Error::Convert { code, message: format!("{what}: {code:?}") }
}

/// `IRErrorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ErrorCode {
    NoError,
    ShaderRequiresRootSignature,
    UnrecognizedRootSignatureDescriptor,
    UnrecognizedParameterTypeInRootSignature,
    ResourceNotReferencedByRootSignature,
    ShaderIncompatibleWithDualSourceBlending,
    UnsupportedWaveSize,
    UnsupportedInstruction,
    CompilationError,
    FailedToSynthesizeStageInFunction,
    FailedToSynthesizeStreamOutFunction,
    FailedToSynthesizeIndirectIntersectionFunction,
    UnableToVerifyModule,
    UnableToLinkModule,
    UnrecognizedDXILHeader,
    InvalidRaytracingAttribute,
    NullHullShaderInputOutputMismatch,
    InvalidRaytracingUserAttributeSize,
    IncorrectHitgroupType,
    FP64Usage,
    Unknown,
}

impl ErrorCode {
    pub fn from_raw(v: u32) -> Self {
        use ErrorCode::*;
        const ALL: [ErrorCode; 20] = [
            NoError,
            ShaderRequiresRootSignature,
            UnrecognizedRootSignatureDescriptor,
            UnrecognizedParameterTypeInRootSignature,
            ResourceNotReferencedByRootSignature,
            ShaderIncompatibleWithDualSourceBlending,
            UnsupportedWaveSize,
            UnsupportedInstruction,
            CompilationError,
            FailedToSynthesizeStageInFunction,
            FailedToSynthesizeStreamOutFunction,
            FailedToSynthesizeIndirectIntersectionFunction,
            UnableToVerifyModule,
            UnableToLinkModule,
            UnrecognizedDXILHeader,
            InvalidRaytracingAttribute,
            NullHullShaderInputOutputMismatch,
            InvalidRaytracingUserAttributeSize,
            IncorrectHitgroupType,
            FP64Usage,
        ];
        ALL.get(v as usize).copied().unwrap_or(Unknown)
    }
}

impl RootSignature {
    /// Parses a serialized blob (see [`RootSignatureDesc::from_blob`]) and creates the converter object.
    pub fn from_blob(blob: &[u8]) -> Result<Arc<Self>, Error> {
        Self::new(RootSignatureDesc::from_blob(blob)?)
    }

    /// Creates the root signature and validates it with the converter right away.
    pub fn new(desc: RootSignatureDesc) -> Result<Arc<Self>, Error> {
        let rs = Self::new_deferred(desc);
        rs.raw()?;
        Ok(rs)
    }

    /// Creates the root signature without touching the converter (pure Rust). The converter
    /// object is created on first in-process use; helper-backed pipelines never create it.
    pub fn new_deferred(desc: RootSignatureDesc) -> Arc<Self> {
        Arc::new(Self { layout: desc.layout(), hash: desc.canonical_hash(), desc, ir: OnceLock::new() })
    }

    pub fn desc(&self) -> &RootSignatureDesc {
        &self.desc
    }
    pub fn layout(&self) -> &RootSignatureLayout {
        &self.layout
    }
    /// Canonical content hash (cache key component).
    pub fn hash(&self) -> u128 {
        self.hash
    }
    pub(crate) fn raw(&self) -> Result<*const ffi::IRRootSignature, Error> {
        self.ir
            .get_or_init(|| {
                let api = ffi::api()?;
                // SAFETY: the descriptor arrays live for the duration of the call.
                unsafe { create_ir_root_signature(api, &self.desc) }.map(|p| p as usize)
            })
            .clone()
            .map(|p| p as *const ffi::IRRootSignature)
    }

    /// The converter's view of the top-level argument buffer (`IRRootSignatureGetResourceLocations`).
    pub fn converter_resource_locations(&self) -> Result<Vec<ResourceLocation>, Error> {
        let api = ffi::api()?;
        let ir = self.raw()?;
        unsafe {
            let n = (api.IRRootSignatureGetResourceCount)(ir);
            let mut v = vec![std::mem::zeroed::<IRResourceLocation>(); n];
            if n > 0 {
                (api.IRRootSignatureGetResourceLocations)(ir, v.as_mut_ptr());
            }
            Ok(read_locations(&v))
        }
    }
}

/// Builds the IR versioned descriptor (arrays kept alive on the stack frame) and creates the
/// root signature.
unsafe fn create_ir_root_signature(api: &Api, desc: &RootSignatureDesc) -> Result<*mut IRRootSignature, Error> {
    let samplers: Vec<IRStaticSamplerDescriptor> = desc
        .static_samplers
        .iter()
        .map(|s| IRStaticSamplerDescriptor {
            Filter: s.filter,
            AddressU: s.address_u,
            AddressV: s.address_v,
            AddressW: s.address_w,
            MipLODBias: s.mip_lod_bias,
            MaxAnisotropy: s.max_anisotropy,
            ComparisonFunc: s.comparison_func,
            // 1.2 UINT border colors (3 = opaque black uint, 4 = opaque white uint) have no IR
            // equivalent; the float variants produce the same texel values for 0/1.
            BorderColor: match s.border_color {
                3 => 1,
                4 => 2,
                c => c,
            },
            MinLOD: s.min_lod,
            MaxLOD: s.max_lod,
            ShaderRegister: s.shader_register,
            RegisterSpace: s.register_space,
            ShaderVisibility: s.visibility,
        })
        .collect();
    let mut err: *mut IRError = std::ptr::null_mut();
    // The converter only accepts 1.1 descriptors (1.0 is rejected with
    // UnrecognizedRootSignatureDescriptor), so 1.0 signatures are expressed with the flags
    // the D3D12 spec defines as 1.0 semantics (the converter's own blob parser does the same).
    let v10 = desc.version == RootSignatureVersion::V1_0;
    let rs = {
        let ranges: Vec<Vec<IRDescriptorRange1>> = desc
            .parameters
            .iter()
            .map(|p| match &p.kind {
                RootParameterKind::DescriptorTable(r) => r
                    .iter()
                    .map(|r| IRDescriptorRange1 {
                        RangeType: r.range_type as u32,
                        NumDescriptors: r.num_descriptors,
                        BaseShaderRegister: r.base_shader_register,
                        RegisterSpace: r.register_space,
                        Flags: if v10 {
                            if r.range_type == DescriptorRangeType::Sampler {
                                flags::RANGE_V1_0_DEFAULT_SAMPLER
                            } else {
                                flags::RANGE_V1_0_DEFAULT_CBV_SRV_UAV
                            }
                        } else {
                            r.flags
                        },
                        OffsetInDescriptorsFromTableStart: r.offset_in_descriptors_from_table_start,
                    })
                    .collect(),
                _ => Vec::new(),
            })
            .collect();
        let params: Vec<IRRootParameter1> = desc
            .parameters
            .iter()
            .zip(&ranges)
            .map(|(p, r)| IRRootParameter1 {
                ParameterType: p.kind.raw_type(),
                u: match &p.kind {
                    RootParameterKind::DescriptorTable(_) => IRRootParameter1U {
                        DescriptorTable: IRRootDescriptorTable1 { NumDescriptorRanges: r.len() as u32, pDescriptorRanges: r.as_ptr() },
                    },
                    RootParameterKind::Constants { shader_register, register_space, num_32bit_values } => IRRootParameter1U {
                        Constants: IRRootConstants {
                            ShaderRegister: *shader_register,
                            RegisterSpace: *register_space,
                            Num32BitValues: *num_32bit_values,
                        },
                    },
                    RootParameterKind::Descriptor(_, d) => IRRootParameter1U {
                        Descriptor: IRRootDescriptor1 { ShaderRegister: d.shader_register, RegisterSpace: d.register_space, Flags: if v10 { flags::ROOT_DESCRIPTOR_V1_0_DEFAULT } else { d.flags } },
                    },
                },
                ShaderVisibility: p.visibility,
            })
            .collect();
        let v = IRVersionedRootSignatureDescriptor {
            version: IRRootSignatureVersion_1_1,
            u: IRVersionedRootSignatureDescriptorU {
                desc_1_1: IRRootSignatureDescriptor1 {
                    NumParameters: params.len() as u32,
                    pParameters: params.as_ptr(),
                    NumStaticSamplers: samplers.len() as u32,
                    pStaticSamplers: samplers.as_ptr(),
                    Flags: desc.flags,
                },
            },
        };
        (api.IRRootSignatureCreateFromDescriptor)(&v, &mut err)
    };
    if rs.is_null() {
        return Err(take_error(api, err, "IRRootSignatureCreateFromDescriptor"));
    }
    if !err.is_null() {
        (api.IRErrorDestroy)(err);
    }
    Ok(rs)
}

/// Parses a blob with the converter's own deserializer and returns its JSON form (diagnostics /
/// cross-checking our parser).
pub fn converter_blob_to_json(blob: &[u8]) -> Result<String, Error> {
    let api = ffi::api()?;
    unsafe {
        let mut err = std::ptr::null_mut();
        let d = (api.IRVersionedRootSignatureDescriptorCreateFromBlob)(blob.as_ptr(), blob.len() as u32, &mut err);
        if d.is_null() {
            return Err(take_error(api, err, "IRVersionedRootSignatureDescriptorCreateFromBlob"));
        }
        let s = (api.IRVersionedRootSignatureDescriptorCopyJSONString)(d);
        let json = cstr(s);
        if !s.is_null() {
            (api.IRVersionedRootSignatureDescriptorReleaseString)(s);
        }
        (api.IRVersionedRootSignatureDescriptorRelease)(d);
        Ok(json)
    }
}
