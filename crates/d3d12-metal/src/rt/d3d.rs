//! D3D12 DXR structures (x64 layout) and their parsing into owned inputs.
//!
//! `D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS` refers to geometry descriptors by CPU
//! pointer (the game's address space, which is ours in the hybrid runtime), and to instance
//! descriptors by GPU VA. [`parse_inputs`] copies the CPU side out so it can be traced and kept.
#![allow(non_snake_case)]

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE`.
pub const TYPE_TOP_LEVEL: u32 = 0;
pub const TYPE_BOTTOM_LEVEL: u32 = 1;

/// `D3D12_ELEMENTS_LAYOUT`.
pub const LAYOUT_ARRAY: u32 = 0;
pub const LAYOUT_ARRAY_OF_POINTERS: u32 = 1;

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS`.
pub mod build_flags {
    pub const ALLOW_UPDATE: u32 = 0x1;
    pub const ALLOW_COMPACTION: u32 = 0x2;
    pub const PREFER_FAST_TRACE: u32 = 0x4;
    pub const PREFER_FAST_BUILD: u32 = 0x8;
    pub const MINIMIZE_MEMORY: u32 = 0x10;
    pub const PERFORM_UPDATE: u32 = 0x20;
}

/// `D3D12_RAYTRACING_GEOMETRY_TYPE`.
pub const GEOMETRY_TRIANGLES: u32 = 0;
pub const GEOMETRY_PROCEDURAL_AABBS: u32 = 1;

/// `D3D12_RAYTRACING_GEOMETRY_FLAGS`.
pub const GEOMETRY_FLAG_OPAQUE: u32 = 0x1;
pub const GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION: u32 = 0x2;

/// `D3D12_RAYTRACING_INSTANCE_FLAGS` (same bits as `MTLAccelerationStructureInstanceOptions`
/// for the low four).
pub mod instance_flags {
    pub const TRIANGLE_CULL_DISABLE: u32 = 0x1;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: u32 = 0x2;
    pub const FORCE_OPAQUE: u32 = 0x4;
    pub const FORCE_NON_OPAQUE: u32 = 0x8;
}

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE`.
pub mod postbuild {
    pub const COMPACTED_SIZE: u32 = 0;
    pub const TOOLS_VISUALIZATION: u32 = 1;
    pub const SERIALIZATION: u32 = 2;
    pub const CURRENT_SIZE: u32 = 3;
}

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE`.
pub mod copy_mode {
    pub const CLONE: u32 = 0;
    pub const COMPACT: u32 = 1;
    pub const VISUALIZATION_DECODE_FOR_TOOLS: u32 = 2;
    pub const SERIALIZE: u32 = 3;
    pub const DESERIALIZE: u32 = 4;
}

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT`.
pub const AS_ALIGNMENT: u64 = 256;
/// `D3D12_RAYTRACING_INSTANCE_DESCS_BYTE_ALIGNMENT`.
pub const INSTANCE_DESC_ALIGNMENT: u64 = 16;

/// `D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BuildInputs {
    pub Type: u32,
    pub Flags: u32,
    pub NumDescs: u32,
    pub DescsLayout: u32,
    /// `InstanceDescs` (GPU VA), `pGeometryDescs` or `ppGeometryDescs` (CPU pointers).
    pub Descs: u64,
}

/// `D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrianglesDesc {
    pub Transform3x4: u64,
    pub IndexFormat: u32,
    pub VertexFormat: u32,
    pub IndexCount: u32,
    pub VertexCount: u32,
    pub IndexBuffer: u64,
    pub VertexBufferStart: u64,
    pub VertexBufferStride: u64,
}

/// `D3D12_RAYTRACING_GEOMETRY_AABBS_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AabbsDesc {
    pub AABBCount: u64,
    pub Start: u64,
    pub Stride: u64,
}

/// `D3D12_RAYTRACING_GEOMETRY_DESC` (the union as raw words).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GeometryDesc {
    pub Type: u32,
    pub Flags: u32,
    pub u: [u64; 6],
}

/// `D3D12_RAYTRACING_INSTANCE_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InstanceDesc {
    pub Transform: [[f32; 4]; 3],
    /// `InstanceID : 24`, `InstanceMask : 8`.
    pub id_mask: u32,
    /// `InstanceContributionToHitGroupIndex : 24`, `Flags : 8`.
    pub contribution_flags: u32,
    pub AccelerationStructure: u64,
}

impl InstanceDesc {
    pub fn new(transform: [[f32; 4]; 3], id: u32, mask: u8, contribution: u32, flags: u8, blas: u64) -> InstanceDesc {
        InstanceDesc { Transform: transform, id_mask: (id & 0xff_ffff) | (mask as u32) << 24, contribution_flags: (contribution & 0xff_ffff) | (flags as u32) << 24, AccelerationStructure: blas }
    }
    pub fn id(&self) -> u32 {
        self.id_mask & 0xff_ffff
    }
    pub fn mask(&self) -> u32 {
        self.id_mask >> 24
    }
    pub fn contribution(&self) -> u32 {
        self.contribution_flags & 0xff_ffff
    }
    pub fn flags(&self) -> u32 {
        self.contribution_flags >> 24
    }
}

/// `D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BuildDesc {
    pub DestAccelerationStructureData: u64,
    pub Inputs: BuildInputs,
    pub SourceAccelerationStructureData: u64,
    pub ScratchAccelerationStructureData: u64,
}

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrebuildInfo {
    pub ResultDataMaxSizeInBytes: u64,
    pub ScratchDataSizeInBytes: u64,
    pub UpdateScratchDataSizeInBytes: u64,
}

/// `D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PostbuildInfoDesc {
    pub DestBuffer: u64,
    pub InfoType: u32,
    pub _pad: u32,
}

const _: () = {
    assert!(std::mem::size_of::<BuildInputs>() == 24);
    assert!(std::mem::size_of::<TrianglesDesc>() == 48);
    assert!(std::mem::size_of::<AabbsDesc>() == 24);
    assert!(std::mem::size_of::<GeometryDesc>() == 56);
    assert!(std::mem::size_of::<InstanceDesc>() == 64);
    assert!(std::mem::size_of::<BuildDesc>() == 48);
    assert!(std::mem::size_of::<PrebuildInfo>() == 24);
    assert!(std::mem::size_of::<PostbuildInfoDesc>() == 16);
};

/// One BLAS geometry, owned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Geometry {
    Triangles { flags: u32, desc: TrianglesDesc },
    Aabbs { flags: u32, desc: AabbsDesc },
}

impl Geometry {
    pub fn flags(&self) -> u32 {
        match *self {
            Geometry::Triangles { flags, .. } | Geometry::Aabbs { flags, .. } => flags,
        }
    }

    pub fn from_desc(g: &GeometryDesc) -> Result<Geometry, String> {
        // SAFETY: both union members are plain words within `u`.
        match g.Type {
            GEOMETRY_TRIANGLES => Ok(Geometry::Triangles { flags: g.Flags, desc: unsafe { std::ptr::read_unaligned(g.u.as_ptr().cast()) } }),
            GEOMETRY_PROCEDURAL_AABBS => Ok(Geometry::Aabbs { flags: g.Flags, desc: unsafe { std::ptr::read_unaligned(g.u.as_ptr().cast()) } }),
            t => Err(format!("unsupported raytracing geometry type {t}")),
        }
    }

    pub fn to_desc(&self) -> GeometryDesc {
        let mut g = GeometryDesc::default();
        // SAFETY: the union members fit in `u` (48 bytes).
        unsafe {
            match *self {
                Geometry::Triangles { flags, desc } => {
                    g.Type = GEOMETRY_TRIANGLES;
                    g.Flags = flags;
                    std::ptr::write_unaligned(g.u.as_mut_ptr().cast(), desc);
                }
                Geometry::Aabbs { flags, desc } => {
                    g.Type = GEOMETRY_PROCEDURAL_AABBS;
                    g.Flags = flags;
                    std::ptr::write_unaligned(g.u.as_mut_ptr().cast(), desc);
                }
            }
        }
        g
    }
}

/// Build inputs with the CPU-side descriptors copied out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inputs {
    Bottom { flags: u32, geometries: Vec<Geometry> },
    /// Instance descriptors stay on the GPU: `instances` is a VA of `count` descs, or of `count`
    /// VAs of descs when `pointers`.
    Top { flags: u32, count: u32, instances: u64, pointers: bool },
}

impl Inputs {
    pub fn flags(&self) -> u32 {
        match *self {
            Inputs::Bottom { flags, .. } | Inputs::Top { flags, .. } => flags,
        }
    }

    pub fn is_top(&self) -> bool {
        matches!(self, Inputs::Top { .. })
    }

    /// Instance count of a TLAS (0 for a BLAS).
    pub fn instance_count(&self) -> u32 {
        match *self {
            Inputs::Top { count, .. } => count,
            Inputs::Bottom { .. } => 0,
        }
    }

    /// Copy a D3D inputs struct into owned [`Inputs`].
    ///
    /// # Safety
    /// See [`parse_inputs`].
    pub unsafe fn from_raw(raw: &BuildInputs) -> Result<Inputs, String> {
        parse_inputs(raw)
    }

    /// Back to the D3D struct. `geometry_descs` receives the BLAS descs that `Descs` points at
    /// (as an `ARRAY`); it must outlive the use of the returned struct.
    pub fn to_raw(&self, geometry_descs: &mut Vec<GeometryDesc>) -> BuildInputs {
        match self {
            Inputs::Top { flags, count, instances, pointers } => BuildInputs { Type: TYPE_TOP_LEVEL, Flags: *flags, NumDescs: *count, DescsLayout: if *pointers { LAYOUT_ARRAY_OF_POINTERS } else { LAYOUT_ARRAY }, Descs: *instances },
            Inputs::Bottom { flags, geometries } => {
                *geometry_descs = geometries.iter().map(Geometry::to_desc).collect();
                BuildInputs { Type: TYPE_BOTTOM_LEVEL, Flags: *flags, NumDescs: geometries.len() as u32, DescsLayout: LAYOUT_ARRAY, Descs: geometry_descs.as_ptr() as u64 }
            }
        }
    }
}

/// Copy a D3D inputs struct into owned [`Inputs`].
///
/// # Safety
/// For a BLAS, `raw.Descs` must be a valid CPU pointer to `NumDescs` geometry descs (`ARRAY`)
/// or to `NumDescs` pointers to them (`ARRAY_OF_POINTERS`), as D3D12 requires.
pub unsafe fn parse_inputs(raw: &BuildInputs) -> Result<Inputs, String> {
    let n = raw.NumDescs as usize;
    let pointers = match raw.DescsLayout {
        LAYOUT_ARRAY => false,
        LAYOUT_ARRAY_OF_POINTERS => true,
        l => return Err(format!("unknown DescsLayout {l}")),
    };
    match raw.Type {
        TYPE_TOP_LEVEL => Ok(Inputs::Top { flags: raw.Flags, count: raw.NumDescs, instances: raw.Descs, pointers }),
        TYPE_BOTTOM_LEVEL => {
            if n > 0 && raw.Descs == 0 {
                return Err("BLAS inputs with NumDescs > 0 and no geometry descs".into());
            }
            let mut geometries = Vec::with_capacity(n);
            for i in 0..n {
                let p = if pointers { std::ptr::read_unaligned((raw.Descs as *const u64).add(i)) as *const GeometryDesc } else { (raw.Descs as *const GeometryDesc).add(i) };
                if p.is_null() {
                    return Err(format!("null geometry desc pointer {i}"));
                }
                geometries.push(Geometry::from_desc(&std::ptr::read_unaligned(p))?);
            }
            Ok(Inputs::Bottom { flags: raw.Flags, geometries })
        }
        t => Err(format!("unknown acceleration structure type {t}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tri(vb: u64) -> Geometry {
        Geometry::Triangles { flags: GEOMETRY_FLAG_OPAQUE, desc: TrianglesDesc { Transform3x4: 0, IndexFormat: 57, VertexFormat: 6, IndexCount: 6, VertexCount: 4, IndexBuffer: 0x1000, VertexBufferStart: vb, VertexBufferStride: 12 } }
    }

    #[test]
    fn geometry_round_trip() {
        let g = tri(0x2000);
        assert_eq!(Geometry::from_desc(&g.to_desc()).unwrap(), g);
        let a = Geometry::Aabbs { flags: 2, desc: AabbsDesc { AABBCount: 3, Start: 0x3000, Stride: 32 } };
        let d = a.to_desc();
        assert_eq!((d.Type, d.u[0], d.u[1], d.u[2]), (GEOMETRY_PROCEDURAL_AABBS, 3, 0x3000, 32));
        assert_eq!(Geometry::from_desc(&d).unwrap(), a);
    }

    #[test]
    fn triangles_union_offsets() {
        // D3D12_RAYTRACING_GEOMETRY_DESC: Triangles at offset 8; VertexBuffer.StrideInBytes at 48.
        let d = tri(0x2000).to_desc();
        let b: [u8; 56] = unsafe { std::mem::transmute(d) };
        assert_eq!(u32::from_le_bytes(b[16..20].try_into().unwrap()), 57);
        assert_eq!(u32::from_le_bytes(b[20..24].try_into().unwrap()), 6);
        assert_eq!(u64::from_le_bytes(b[32..40].try_into().unwrap()), 0x1000);
        assert_eq!(u64::from_le_bytes(b[40..48].try_into().unwrap()), 0x2000);
        assert_eq!(u64::from_le_bytes(b[48..56].try_into().unwrap()), 12);
    }

    #[test]
    fn parse_array_and_pointers() {
        let descs = [tri(0x2000).to_desc(), tri(0x4000).to_desc()];
        let raw = BuildInputs { Type: TYPE_BOTTOM_LEVEL, Flags: build_flags::ALLOW_UPDATE, NumDescs: 2, DescsLayout: LAYOUT_ARRAY, Descs: descs.as_ptr() as u64 };
        let want = Inputs::Bottom { flags: 1, geometries: vec![tri(0x2000), tri(0x4000)] };
        assert_eq!(unsafe { parse_inputs(&raw) }.unwrap(), want);
        let ptrs = [&descs[0] as *const GeometryDesc as u64, &descs[1] as *const GeometryDesc as u64];
        let raw = BuildInputs { DescsLayout: LAYOUT_ARRAY_OF_POINTERS, Descs: ptrs.as_ptr() as u64, ..raw };
        assert_eq!(unsafe { parse_inputs(&raw) }.unwrap(), want);

        let mut keep = Vec::new();
        let back = want.to_raw(&mut keep);
        assert_eq!(unsafe { parse_inputs(&back) }.unwrap(), want);

        let top = BuildInputs { Type: TYPE_TOP_LEVEL, Flags: 0, NumDescs: 5, DescsLayout: LAYOUT_ARRAY, Descs: 0xABC0 };
        assert_eq!(unsafe { parse_inputs(&top) }.unwrap(), Inputs::Top { flags: 0, count: 5, instances: 0xABC0, pointers: false });
        assert!(unsafe { parse_inputs(&BuildInputs { Type: 7, ..top }) }.is_err());
    }

    #[test]
    fn instance_bitfields() {
        let i = InstanceDesc::new([[0.0; 4]; 3], 0x12_3456, 0xAB, 0x65_4321, 0x0C, 0x1000);
        assert_eq!((i.id(), i.mask(), i.contribution(), i.flags()), (0x12_3456, 0xAB, 0x65_4321, 0x0C));
        let b: [u8; 64] = unsafe { std::mem::transmute(i) };
        assert_eq!(b[48..52], [0x56, 0x34, 0x12, 0xAB]);
        assert_eq!(b[52..56], [0x21, 0x43, 0x65, 0x0C]);
    }
}
