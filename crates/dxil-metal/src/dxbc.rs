//! DXBC container parsing/building (the wrapper around DXIL bytecode and serialized root
//! signatures) plus DXIL program-header and SFI0 feature-flag decoding.

use crate::hash::dxbc_checksum;
use crate::Error;

pub const FOURCC_DXIL: [u8; 4] = *b"DXIL";
pub const FOURCC_RTS0: [u8; 4] = *b"RTS0";
pub const FOURCC_SFI0: [u8; 4] = *b"SFI0";
pub const FOURCC_PSV0: [u8; 4] = *b"PSV0";
pub const FOURCC_HASH: [u8; 4] = *b"HASH";
pub const FOURCC_RDAT: [u8; 4] = *b"RDAT";
/// SM5 / SM4 token-stream bytecode (FXC output).
pub const FOURCC_SHEX: [u8; 4] = *b"SHEX";
pub const FOURCC_SHDR: [u8; 4] = *b"SHDR";

/// A parsed (borrowed) DXBC container.
#[derive(Debug, Clone)]
pub struct Container<'a> {
    pub digest: [u8; 16],
    pub parts: Vec<Part<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Part<'a> {
    pub fourcc: [u8; 4],
    pub data: &'a [u8],
}

fn rd32(b: &[u8], off: usize) -> Option<u32> {
    b.get(off..off + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()))
}

impl<'a> Container<'a> {
    pub fn is_container(bytes: &[u8]) -> bool {
        bytes.len() >= 32 && &bytes[0..4] == b"DXBC"
    }

    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        let bad = |m: &str| Error::InvalidInput(format!("DXBC: {m}"));
        if !Self::is_container(bytes) {
            return Err(bad("missing DXBC magic"));
        }
        let size = rd32(bytes, 24).unwrap() as usize;
        let count = rd32(bytes, 28).unwrap() as usize;
        if size > bytes.len() || size < 32 {
            return Err(bad("container size out of range"));
        }
        let bytes = &bytes[..size];
        if count > 1024 || 32 + count * 4 > size {
            return Err(bad("bad part count"));
        }
        let mut parts = Vec::with_capacity(count);
        for i in 0..count {
            let off = rd32(bytes, 32 + i * 4).unwrap() as usize;
            let fourcc: [u8; 4] = bytes.get(off..off + 4).ok_or_else(|| bad("part offset"))?.try_into().unwrap();
            let len = rd32(bytes, off + 4).ok_or_else(|| bad("part header"))? as usize;
            let data = bytes.get(off + 8..off + 8 + len).ok_or_else(|| bad("part size"))?;
            parts.push(Part { fourcc, data });
        }
        Ok(Self { digest: bytes[4..20].try_into().unwrap(), parts })
    }

    pub fn part(&self, fourcc: [u8; 4]) -> Option<&'a [u8]> {
        self.parts.iter().find(|p| p.fourcc == fourcc).map(|p| p.data)
    }

    /// True for FXC (SM4/SM5) output: token-stream bytecode and no `DXIL` part.
    pub fn is_sm4_bytecode(&self) -> bool {
        self.part(FOURCC_DXIL).is_none() && (self.part(FOURCC_SHEX).is_some() || self.part(FOURCC_SHDR).is_some())
    }

    /// Program info from the `DXIL` part, or from the `SHEX`/`SHDR` version token for SM4/SM5 bytecode.
    pub fn program(&self) -> Option<ProgramInfo> {
        if let Some(dxil) = self.part(FOURCC_DXIL) {
            ProgramInfo::parse(dxil)
        } else {
            let shex = self.part(FOURCC_SHEX).or_else(|| self.part(FOURCC_SHDR))?;
            ProgramInfo::parse_shex(shex)
        }
    }

    /// Shader feature flags from `SFI0` (0 when absent).
    pub fn feature_flags(&self) -> u64 {
        self.part(FOURCC_SFI0)
            .and_then(|d| d.get(0..8))
            .map_or(0, |s| u64::from_le_bytes(s.try_into().unwrap()))
    }

    /// Entry point name recorded in `PSV0` (pipeline state validation, runtime info v3+, i.e.
    /// validator 1.8+). Graphics/compute PSOs carry no entry name, so this is how we find it.
    pub fn psv_entry_point(&self) -> Option<String> {
        let d = self.part(FOURCC_PSV0)?;
        let info_size = rd32(d, 0)? as usize;
        if info_size < 52 {
            return None;
        }
        let name_off = rd32(d, 4 + 48)? as usize;
        let mut q = 4 + info_size;
        let nres = rd32(d, q)? as usize;
        q += 4;
        if nres > 0 {
            let bind_size = rd32(d, q)? as usize;
            q += 4 + nres * bind_size;
        }
        let table_size = rd32(d, q)? as usize;
        let table = d.get(q + 4..q + 4 + table_size)?;
        let s = table.get(name_off..)?;
        let end = s.iter().position(|&c| c == 0)?;
        (end > 0).then(|| String::from_utf8_lossy(&s[..end]).into_owned())
    }

    /// Names of the functions the `DXIL` part's bitcode defines. For a non-library shader
    /// that is exactly the entry point; used when `PSV0` predates the entry name (v0-v2).
    pub fn defined_functions(&self) -> Option<Vec<String>> {
        crate::bitcode::defined_functions(crate::bitcode::dxil_part_bitcode(self.part(FOURCC_DXIL)?)?)
    }

    /// Entry point of a non-library shader: `PSV0` (v3+), else the single function the
    /// bitcode defines (skipped for Hull shaders, whose pass-through control-point phase
    /// defines only the patch-constant function such as `pc_main` while the DXIL entry
    /// point name remains `main`).
    pub fn entry_point(&self) -> Option<String> {
        self.psv_entry_point().or_else(|| {
            if self.program().is_some_and(|p| p.kind == ProgramKind::Hull) {
                return None;
            }
            match self.defined_functions() {
                Some(mut f) if f.len() == 1 => f.pop(),
                _ => None,
            }
        })
    }

    /// `MinimumExpectedWaveLaneCount` and `MaximumExpectedWaveLaneCount` from `PSV0` (`PSVRuntimeInfo0`).
    /// `[WaveSize(64)]` shaders record `(64, 64)`; unannotated shaders record `(0, u32::MAX)`.
    pub fn expected_wave_lane_count(&self) -> Option<(u32, u32)> {
        let d = self.part(FOURCC_PSV0)?;
        let info_size = rd32(d, 0)? as usize;
        if info_size < 24 {
            return None;
        }
        Some((rd32(d, 20)?, rd32(d, 24)?))
    }

    /// Resource bindings recorded in `PSV0` (non-library shaders).
    pub fn psv_resources(&self) -> Option<Vec<PsvResource>> {
        let d = self.part(FOURCC_PSV0)?;
        let info_size = rd32(d, 0)? as usize;
        let q = 4 + info_size;
        let n = rd32(d, q)? as usize;
        if n == 0 {
            return Some(Vec::new());
        }
        let stride = rd32(d, q + 4)? as usize;
        if stride < 16 || n > 1 << 16 {
            return None;
        }
        (0..n)
            .map(|i| {
                let o = q + 8 + i * stride;
                Some(PsvResource {
                    res_type: rd32(d, o)?,
                    space: rd32(d, o + 4)?,
                    lower_bound: rd32(d, o + 8)?,
                    upper_bound: rd32(d, o + 12)?,
                    kind: if stride >= 24 { rd32(d, o + 16)? } else { 0 },
                })
            })
            .collect()
    }

    /// Functions of a DXIL library (from the `RDAT` runtime data function table), i.e. the
    /// exports a DXR state object can reference. Empty for non-library shaders.
    pub fn library_functions(&self) -> Vec<LibraryFunction> {
        self.part(FOURCC_RDAT).and_then(parse_rdat_functions).unwrap_or_default()
    }

    /// Every resource binding the shader declares: PSV0 for regular shaders, the RDAT resource
    /// table for libraries.
    pub fn resource_bindings(&self) -> Option<Vec<ResourceBinding>> {
        if let Some(psv) = self.psv_resources() {
            return Some(
                psv.iter()
                    .filter_map(|r| {
                        let range_type = match r.res_type {
                            1 => 3,
                            2 => 2,
                            3..=5 => 0,
                            6..=9 => 1,
                            _ => return None,
                        };
                        Some(ResourceBinding { range_type, space: r.space, lower_bound: r.lower_bound, upper_bound: r.upper_bound })
                    })
                    .collect(),
            );
        }
        self.part(FOURCC_RDAT).and_then(parse_rdat_resources)
    }

    /// The 16-byte shader hash from the `HASH` part (flags u32 + digest), if present.
    pub fn shader_hash(&self) -> Option<[u8; 16]> {
        self.part(FOURCC_HASH)?.get(4..20).map(|s| s.try_into().unwrap())
    }
}

/// Builds a DXBC container (with a valid checksum) from parts.
pub fn build_container(parts: &[([u8; 4], &[u8])]) -> Vec<u8> {
    let header = 32 + parts.len() * 4;
    let total = header + parts.iter().map(|(_, d)| 8 + d.len()).sum::<usize>();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"DXBC");
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(parts.len() as u32).to_le_bytes());
    let mut off = header;
    for (_, d) in parts {
        out.extend_from_slice(&(off as u32).to_le_bytes());
        off += 8 + d.len();
    }
    for (fcc, d) in parts {
        out.extend_from_slice(fcc);
        out.extend_from_slice(&(d.len() as u32).to_le_bytes());
        out.extend_from_slice(d);
    }
    let digest = dxbc_checksum(&out[20..]);
    out[4..20].copy_from_slice(&digest);
    out
}

/// Recomputes a container's checksum in place (e.g. dxilconv output, which is unsigned).
pub fn sign_container(bytes: &mut [u8]) -> Result<(), Error> {
    Container::parse(bytes)?;
    let size = rd32(bytes, 24).unwrap() as usize;
    let digest = dxbc_checksum(&bytes[20..size]);
    bytes[4..20].copy_from_slice(&digest);
    Ok(())
}

/// Verifies the container checksum.
pub fn verify_checksum(bytes: &[u8]) -> bool {
    Container::parse(bytes).is_ok_and(|c| {
        let size = rd32(bytes, 24).unwrap() as usize;
        dxbc_checksum(&bytes[20..size]) == c.digest
    })
}

/// One `RuntimeDataFunctionInfo` record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryFunction {
    /// Mangled name (e.g. `\x01?rayGenMain@@YAXXZ`).
    pub name: String,
    /// Export name (e.g. `rayGenMain`): pass this as the entry point.
    pub unmangled_name: String,
    pub kind: ProgramKind,
    pub payload_size: u32,
    pub attribute_size: u32,
    /// SFI0-style feature bits used by this function.
    pub features: u64,
    /// `MinShaderTarget` (kind << 16 | major << 4 | minor).
    pub min_shader_target: u32,
}

/// Returns RDAT part `ty` (RuntimeDataHeader { Version, PartCount, PartOffsets[] }, each part
/// { Type, Size, data }); 1 = string buffer, 3 = resource table, 4 = function table.
fn rdat_part(r: &[u8], ty: u32) -> Option<&[u8]> {
    let part_count = rd32(r, 4)? as usize;
    for i in 0..part_count.min(64) {
        let off = rd32(r, 8 + i * 4)? as usize;
        let size = rd32(r, off + 4)? as usize;
        if rd32(r, off)? == ty {
            return r.get(off + 8..off + 8 + size);
        }
    }
    None
}

/// RDAT resource table (`RuntimeDataResourceInfo`: Class, Kind, ID, Space, LowerBound,
/// UpperBound, Name, Flags).
fn parse_rdat_resources(r: &[u8]) -> Option<Vec<ResourceBinding>> {
    let t = rdat_part(r, 3)?;
    let (count, stride) = (rd32(t, 0)? as usize, rd32(t, 4)? as usize);
    if stride < 24 {
        return None;
    }
    (0..count)
        .map(|i| {
            let o = 8 + i * stride;
            let class = rd32(t, o)?;
            (class <= 3).then_some(())?;
            Some(ResourceBinding { range_type: class, space: rd32(t, o + 12)?, lower_bound: rd32(t, o + 16)?, upper_bound: rd32(t, o + 20)? })
        })
        .collect()
}

fn parse_rdat_functions(r: &[u8]) -> Option<Vec<LibraryFunction>> {
    let strings = rdat_part(r, 1).unwrap_or(&[]);
    let t = rdat_part(r, 4)?;
    let (count, stride) = (rd32(t, 0)? as usize, rd32(t, 4)? as usize);
    if stride < 44 {
        return None;
    }
    let s = |o: u32| -> String {
        strings
            .get(o as usize..)
            .map(|b| String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned())
            .unwrap_or_default()
    };
    (0..count)
        .map(|i| {
            let rec = t.get(8 + i * stride..8 + (i + 1) * stride)?;
            Some(LibraryFunction {
                name: s(rd32(rec, 0)?),
                unmangled_name: s(rd32(rec, 4)?),
                kind: ProgramKind::from_raw(rd32(rec, 16)? as u16),
                payload_size: rd32(rec, 20)?,
                attribute_size: rd32(rec, 24)?,
                features: rd32(rec, 28)? as u64 | (rd32(rec, 32)? as u64) << 32,
                min_shader_target: rd32(rec, 40)?,
            })
        })
        .collect()
}

/// A resource binding range, from PSV0 or RDAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceBinding {
    /// `D3D12_DESCRIPTOR_RANGE_TYPE`: 0 SRV, 1 UAV, 2 CBV, 3 sampler.
    pub range_type: u32,
    pub space: u32,
    pub lower_bound: u32,
    /// Inclusive; `u32::MAX` for unbounded arrays.
    pub upper_bound: u32,
}

/// One `PSVResourceBindInfo0` entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PsvResource {
    /// `PSVResourceType`: 1 sampler, 2 CBV, 3-5 SRV (typed/raw/structured),
    /// 6-9 UAV (typed/raw/structured/structured+counter).
    pub res_type: u32,
    pub space: u32,
    pub lower_bound: u32,
    /// Inclusive; `u32::MAX` for unbounded arrays.
    pub upper_bound: u32,
    /// `DXIL::ResourceKind` (`PSVResourceBindInfo1`); 0 when unknown (v0 records).
    pub kind: u32,
}

impl PsvResource {
    /// SRV or UAV (`res_type` 3-9).
    pub fn is_srv_uav(&self) -> bool {
        (3..=9).contains(&self.res_type)
    }

    /// Texture kinds (1-9 Texture*, 17-18 FeedbackTexture*).
    pub fn is_texture(&self) -> bool {
        matches!(self.kind, 1..=9 | 17 | 18)
    }

    /// Buffer kinds (typed/raw/structured buffer, tbuffer, acceleration structure).
    pub fn is_buffer(&self) -> bool {
        matches!(self.kind, 10 | 11 | 12 | 15 | 16)
    }
}

/// DXIL shader kind (`DXIL::ShaderKind`, high 16 bits of the program version).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProgramKind {
    Pixel,
    Vertex,
    Geometry,
    Hull,
    Domain,
    Compute,
    Library,
    RayGeneration,
    Intersection,
    AnyHit,
    ClosestHit,
    Miss,
    Callable,
    Mesh,
    Amplification,
    Node,
    Unknown(u16),
}

impl ProgramKind {
    pub fn from_raw(v: u16) -> Self {
        use ProgramKind::*;
        match v {
            0 => Pixel,
            1 => Vertex,
            2 => Geometry,
            3 => Hull,
            4 => Domain,
            5 => Compute,
            6 => Library,
            7 => RayGeneration,
            8 => Intersection,
            9 => AnyHit,
            10 => ClosestHit,
            11 => Miss,
            12 => Callable,
            13 => Mesh,
            14 => Amplification,
            15 => Node,
            x => Unknown(x),
        }
    }
    pub fn short_name(self) -> &'static str {
        use ProgramKind::*;
        match self {
            Pixel => "ps",
            Vertex => "vs",
            Geometry => "gs",
            Hull => "hs",
            Domain => "ds",
            Compute => "cs",
            Library => "lib",
            RayGeneration => "raygen",
            Intersection => "intersection",
            AnyHit => "anyhit",
            ClosestHit => "closesthit",
            Miss => "miss",
            Callable => "callable",
            Mesh => "ms",
            Amplification => "as",
            Node => "node",
            Unknown(_) => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramInfo {
    pub kind: ProgramKind,
    pub major: u8,
    pub minor: u8,
    /// `DxilBitcodeHeader::DxilVersion` (validator/DXIL version, e.g. 0x105 = 1.5).
    pub dxil_version: u32,
}

impl ProgramInfo {
    pub fn parse(dxil_part: &[u8]) -> Option<Self> {
        let pv = rd32(dxil_part, 0)?;
        let magic = dxil_part.get(8..12)?;
        if magic != b"DXIL" {
            return None;
        }
        Some(Self {
            kind: ProgramKind::from_raw((pv >> 16) as u16),
            major: ((pv >> 4) & 0xF) as u8,
            minor: (pv & 0xF) as u8,
            dxil_version: rd32(dxil_part, 12)?,
        })
    }
    /// Parses the SM4/SM5 version token at offset 0 of a `SHEX` or `SHDR` part.
    pub fn parse_shex(shex_part: &[u8]) -> Option<Self> {
        let pv = rd32(shex_part, 0)?;
        Some(Self {
            kind: ProgramKind::from_raw((pv >> 16) as u16),
            major: ((pv >> 4) & 0xF) as u8,
            minor: (pv & 0xF) as u8,
            dxil_version: 0,
        })
    }
    /// e.g. `cs_6_5`.
    pub fn profile(&self) -> String {
        format!("{}_{}_{}", self.kind.short_name(), self.major, self.minor)
    }
}

/// Names of SFI0 feature bits (`D3D_SHADER_REQUIRES_*` / `DXIL::ShaderFeatureInfo`).
pub const FEATURE_NAMES: &[&str] = &[
    "Doubles",
    "EarlyDepthStencil",
    "UAVsAtEveryStage",
    "64UAVs",
    "MinimumPrecision",
    "11_1_DoubleExtensions",
    "11_1_ShaderExtensions",
    "Level9ComparisonFiltering",
    "TiledResources",
    "StencilRef",
    "InnerCoverage",
    "TypedUAVLoadAdditionalFormats",
    "ROVs",
    "ViewportAndRTArrayIndexFromAnyShader",
    "WaveOps",
    "Int64Ops",
    "ViewID",
    "Barycentrics",
    "Native16BitOps",
    "ShadingRate",
    "Raytracing_Tier_1_1",
    "SamplerFeedback",
    "AtomicInt64OnTypedResource",
    "AtomicInt64OnGroupShared",
    "DerivativesInMeshAndAmpShaders",
    "ResourceDescriptorHeapIndexing",
    "SamplerDescriptorHeapIndexing",
    "WaveMMA",
    "AtomicInt64OnDescriptorHeapResource",
    "AdvancedTextureOps",
    "WriteableMSAATextures",
    "SampleCmpGradientOrBias",
    "ExtendedCommandInfo",
];

/// Decodes SFI0 bits into names (`bit<N>` for unknown bits).
pub fn feature_names(flags: u64) -> Vec<String> {
    (0..64)
        .filter(|b| flags >> b & 1 == 1)
        .map(|b| FEATURE_NAMES.get(b as usize).map_or_else(|| format!("bit{b}"), |s| s.to_string()))
        .collect()
}

/// Summary of a DXIL blob used for diagnostics / failure taxonomy.
#[derive(Debug, Clone)]
pub struct DxilSummary {
    pub program: Option<ProgramInfo>,
    /// Entry point from PSV0 (non-library shaders).
    pub entry_point: Option<String>,
    pub features: u64,
    pub has_root_signature: bool,
    /// Library exports (DXR), from RDAT.
    pub library_functions: Vec<LibraryFunction>,
    pub shader_hash: Option<[u8; 16]>,
}

pub fn summarize(dxil: &[u8]) -> Result<DxilSummary, Error> {
    let c = Container::parse(dxil)?;
    Ok(DxilSummary {
        program: c.program(),
        entry_point: c.psv_entry_point(),
        features: c.feature_flags(),
        has_root_signature: c.part(FOURCC_RTS0).is_some(),
        library_functions: c.library_functions(),
        shader_hash: c.shader_hash(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_parse_roundtrip_and_checksum() {
        for n in [0usize, 3, 20, 27, 28, 40, 63, 64, 100, 200] {
            let payload: Vec<u8> = (0..n as u8).collect();
            let c = build_container(&[(FOURCC_RTS0, &payload), (*b"TEST", b"xyz")]);
            assert!(verify_checksum(&c));
            let p = Container::parse(&c).unwrap();
            assert_eq!(p.part(FOURCC_RTS0).unwrap(), &payload[..]);
            assert_eq!(p.part(*b"TEST").unwrap(), b"xyz");
        }
    }
}
