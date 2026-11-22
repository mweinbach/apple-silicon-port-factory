//! Primitive topology and index format translation:
//! * `D3D_PRIMITIVE_TOPOLOGY` (IASetPrimitiveTopology) → `MTLPrimitiveType`, with notes for the
//!   cases Metal can't draw natively.
//! * `D3D12_PRIMITIVE_TOPOLOGY_TYPE` (PSO) → `MTLPrimitiveTopologyClass` and the converter's
//!   input topology.
//! * DXGI index formats → `MTLIndexType` and the converter's runtime index type.
//! * Strip-cut (primitive restart) compatibility.
//!
//! Pure functions. Owned by the shaders engineer; see the tests at the bottom.

use dxil_metal::InputTopology;
use objc2_metal::{MTLIndexType, MTLPrimitiveTopologyClass, MTLPrimitiveType};

/// `D3D_PRIMITIVE_TOPOLOGY` values.
pub mod d3d {
    pub const UNDEFINED: u32 = 0;
    pub const POINTLIST: u32 = 1;
    pub const LINELIST: u32 = 2;
    pub const LINESTRIP: u32 = 3;
    pub const TRIANGLELIST: u32 = 4;
    pub const TRIANGLESTRIP: u32 = 5;
    pub const TRIANGLEFAN: u32 = 6;
    pub const LINELIST_ADJ: u32 = 10;
    pub const LINESTRIP_ADJ: u32 = 11;
    pub const TRIANGLELIST_ADJ: u32 = 12;
    pub const TRIANGLESTRIP_ADJ: u32 = 13;
    /// `D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST`; N control points = 32 + N.
    pub const PATCHLIST_1: u32 = 33;
    pub const PATCHLIST_32: u32 = 64;
}

/// `D3D12_PRIMITIVE_TOPOLOGY_TYPE` values.
pub mod topology_type {
    pub const UNDEFINED: u32 = 0;
    pub const POINT: u32 = 1;
    pub const LINE: u32 = 2;
    pub const TRIANGLE: u32 = 3;
    pub const PATCH: u32 = 4;
}

/// `D3D12_INDEX_BUFFER_STRIP_CUT_VALUE`.
pub mod strip_cut {
    pub const DISABLED: u32 = 0;
    pub const CUT_0XFFFF: u32 = 1;
    pub const CUT_0XFFFFFFFF: u32 = 2;
}

/// How a draw with a given `D3D_PRIMITIVE_TOPOLOGY` maps to Metal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Topology {
    /// The Metal primitive to draw. `None` for patch lists, which have no direct primitive
    /// (they need the converter's tessellation emulation), and for UNDEFINED.
    pub primitive: Option<MTLPrimitiveType>,
    /// Pipeline topology class this draw is compatible with.
    pub class: MTLPrimitiveTopologyClass,
    /// `*_ADJ` topologies. Metal has no adjacency primitives. Adjacency only matters to a
    /// geometry shader, which goes through the converter's geometry emulation; without a GS,
    /// D3D12 ignores the adjacent vertices, and the draw must drop them, e.g. by re-indexing.
    /// `primitive` holds the non-adjacency equivalent.
    pub adjacency: bool,
    /// Triangle fans aren't a Metal primitive and need index rewriting to a triangle list
    /// (`primitive` = Triangle). D3D12 itself rejects fans on most hardware.
    pub fan: bool,
    /// Control points per patch for patch lists (0 otherwise).
    pub patch_control_points: u32,
}

impl Topology {
    /// True for strip topologies, where the strip-cut value matters.
    pub fn is_strip(&self) -> bool {
        matches!(self.primitive, Some(MTLPrimitiveType::LineStrip | MTLPrimitiveType::TriangleStrip))
    }
    /// True when the draw can be issued as-is (no adjacency, fan or patch handling).
    pub fn is_native(&self) -> bool {
        self.primitive.is_some() && !self.adjacency && !self.fan
    }
}

/// Translates `D3D_PRIMITIVE_TOPOLOGY`. `None` for unknown values.
pub fn topology(t: u32) -> Option<Topology> {
    use MTLPrimitiveTopologyClass as C;
    use MTLPrimitiveType as P;
    let mk = |primitive, class, adjacency, fan| Topology { primitive, class, adjacency, fan, patch_control_points: 0 };
    Some(match t {
        d3d::UNDEFINED => mk(None, C::Unspecified, false, false),
        d3d::POINTLIST => mk(Some(P::Point), C::Point, false, false),
        d3d::LINELIST => mk(Some(P::Line), C::Line, false, false),
        d3d::LINESTRIP => mk(Some(P::LineStrip), C::Line, false, false),
        d3d::TRIANGLELIST => mk(Some(P::Triangle), C::Triangle, false, false),
        d3d::TRIANGLESTRIP => mk(Some(P::TriangleStrip), C::Triangle, false, false),
        d3d::TRIANGLEFAN => mk(Some(P::Triangle), C::Triangle, false, true),
        d3d::LINELIST_ADJ => mk(Some(P::Line), C::Line, true, false),
        d3d::LINESTRIP_ADJ => mk(Some(P::LineStrip), C::Line, true, false),
        d3d::TRIANGLELIST_ADJ => mk(Some(P::Triangle), C::Triangle, true, false),
        d3d::TRIANGLESTRIP_ADJ => mk(Some(P::TriangleStrip), C::Triangle, true, false),
        d3d::PATCHLIST_1..=d3d::PATCHLIST_32 => Topology {
            primitive: None,
            // Tessellated output is triangles (or lines/points, decided by the domain shader).
            class: C::Unspecified,
            adjacency: false,
            fan: false,
            patch_control_points: t - d3d::PATCHLIST_1 + 1,
        },
        _ => return None,
    })
}

/// `D3D12_PRIMITIVE_TOPOLOGY_TYPE` → the render pipeline's `inputPrimitiveTopology`. PATCH is
/// `Unspecified`, since the tessellation emulation path builds its own pipelines.
pub fn topology_class(ty: u32) -> MTLPrimitiveTopologyClass {
    match ty {
        topology_type::POINT => MTLPrimitiveTopologyClass::Point,
        topology_type::LINE => MTLPrimitiveTopologyClass::Line,
        topology_type::TRIANGLE => MTLPrimitiveTopologyClass::Triangle,
        _ => MTLPrimitiveTopologyClass::Unspecified,
    }
}

/// `D3D12_PRIMITIVE_TOPOLOGY_TYPE` → the converter's input topology (`CompileOptions::input_topology`,
/// needed when compiling geometry shaders and for geometry/tessellation emulation).
pub fn input_topology(ty: u32) -> InputTopology {
    match ty {
        topology_type::POINT => InputTopology::Point,
        topology_type::LINE => InputTopology::Line,
        topology_type::TRIANGLE => InputTopology::Triangle,
        topology_type::PATCH => InputTopology::Patch,
        _ => InputTopology::Undefined,
    }
}

/// True when a draw topology is legal with a PSO of topology type `ty`, following D3D12's rule.
pub fn compatible(t: u32, ty: u32) -> bool {
    match (topology(t), ty) {
        (None, _) => false,
        (Some(tp), topology_type::PATCH) => tp.patch_control_points > 0,
        (Some(tp), _) if tp.patch_control_points > 0 => false,
        (Some(tp), _) => tp.class == topology_class(ty) && ty != topology_type::UNDEFINED,
    }
}

/// `IRRuntimePrimitiveType` (`RuntimeDrawInfo::primitive_topology` for geometry/tessellation
/// emulation). The converter has no strip-with-adjacency triangle type (`None`), and patch
/// lists all map to `Triangle` (as in `metal_irconverter_runtime.h`).
pub fn ir_primitive_type(t: u32) -> Option<u8> {
    Some(match t {
        d3d::POINTLIST => 0,
        d3d::LINELIST => 1,
        d3d::LINESTRIP => 2,
        d3d::TRIANGLELIST => 3,
        d3d::TRIANGLESTRIP => 4,
        d3d::LINELIST_ADJ => 5,
        d3d::TRIANGLELIST_ADJ => 6,
        d3d::LINESTRIP_ADJ => 7,
        d3d::PATCHLIST_1..=d3d::PATCHLIST_32 => 3,
        _ => return None,
    })
}

/// An index buffer format (`D3D12_INDEX_BUFFER_VIEW::Format`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexFormat {
    pub mtl: MTLIndexType,
    /// Bytes per index.
    pub size: u32,
    /// Value bound at the converter's uniforms bind point for indexed draws
    /// (`IRMetalIndexToIRIndex`).
    pub ir: u16,
    /// The all-ones restart value Metal always applies to strips.
    pub restart: u32,
}

/// DXGI_FORMAT_R16_UINT (57) / R32_UINT (42) → index format; anything else → `None`.
pub fn index_format(dxgi: u32) -> Option<IndexFormat> {
    let (mtl, size, restart) = match dxgi {
        57 => (MTLIndexType::UInt16, 2, 0xFFFF),
        42 => (MTLIndexType::UInt32, 4, 0xFFFF_FFFF),
        _ => return None,
    };
    Some(IndexFormat { mtl, size, ir: dxil_metal::runtime::metal_index_type_to_ir(mtl.0 as u64), restart })
}

/// Primitive restart: Metal always restarts strips at the all-ones index, and the rule can't
/// be turned off. D3D12 restarts only when the PSO's `IBStripCutValue` matches. Returns true
/// when Metal's behaviour equals D3D12's for this PSO, strip-cut value, index format and
/// topology. When it returns false, a strip draw whose index buffer contains the all-ones
/// value would differ from D3D12 and needs rewriting. That is rare, since engines use cut
/// values deliberately. List topologies ignore restart in both APIs, as long as the index is
/// in range.
pub fn strip_cut_matches(strip_cut_value: u32, index_dxgi: u32, t: u32) -> bool {
    let Some(tp) = topology(t) else { return false };
    if !tp.is_strip() {
        return true;
    }
    match (strip_cut_value, index_dxgi) {
        (strip_cut::CUT_0XFFFF, 57) | (strip_cut::CUT_0XFFFFFFFF, 42) => true,
        // DISABLED, or a cut value for the other index width: D3D12 treats all-ones as a vertex.
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_topologies() {
        let t = topology(d3d::TRIANGLELIST).unwrap();
        assert_eq!(t.primitive, Some(MTLPrimitiveType::Triangle));
        assert_eq!(t.class, MTLPrimitiveTopologyClass::Triangle);
        assert!(t.is_native() && !t.is_strip());
        assert!(topology(d3d::TRIANGLESTRIP).unwrap().is_strip());
        assert!(topology(d3d::LINESTRIP).unwrap().is_strip());
        assert_eq!(topology(d3d::POINTLIST).unwrap().primitive, Some(MTLPrimitiveType::Point));
        assert_eq!(topology(d3d::LINELIST).unwrap().class, MTLPrimitiveTopologyClass::Line);
        assert!(topology(7).is_none() && topology(65).is_none() && topology(14).is_none());
        assert_eq!(topology(d3d::UNDEFINED).unwrap().primitive, None);
    }

    #[test]
    fn special_topologies() {
        let adj = topology(d3d::TRIANGLESTRIP_ADJ).unwrap();
        assert!(adj.adjacency && !adj.is_native());
        assert_eq!(adj.primitive, Some(MTLPrimitiveType::TriangleStrip));
        let fan = topology(d3d::TRIANGLEFAN).unwrap();
        assert!(fan.fan && !fan.is_native());
        for n in 1..=32 {
            let p = topology(32 + n).unwrap();
            assert_eq!((p.patch_control_points, p.primitive), (n, None));
        }
    }

    #[test]
    fn pso_compatibility() {
        assert!(compatible(d3d::TRIANGLESTRIP, topology_type::TRIANGLE));
        assert!(compatible(d3d::TRIANGLELIST_ADJ, topology_type::TRIANGLE));
        assert!(!compatible(d3d::LINELIST, topology_type::TRIANGLE));
        assert!(compatible(d3d::PATCHLIST_1 + 2, topology_type::PATCH));
        assert!(!compatible(d3d::TRIANGLELIST, topology_type::PATCH));
        assert!(!compatible(d3d::PATCHLIST_1, topology_type::TRIANGLE));
        assert!(!compatible(d3d::POINTLIST, topology_type::UNDEFINED));
        assert_eq!(topology_class(topology_type::PATCH), MTLPrimitiveTopologyClass::Unspecified);
        assert_eq!(input_topology(topology_type::PATCH), InputTopology::Patch);
        assert_eq!(input_topology(topology_type::LINE), InputTopology::Line);
    }

    #[test]
    fn ir_primitive_types_follow_runtime_header() {
        assert_eq!(ir_primitive_type(d3d::POINTLIST), Some(0));
        assert_eq!(ir_primitive_type(d3d::TRIANGLESTRIP), Some(4));
        assert_eq!(ir_primitive_type(d3d::LINESTRIP_ADJ), Some(7));
        assert_eq!(ir_primitive_type(d3d::TRIANGLESTRIP_ADJ), None);
        assert_eq!(ir_primitive_type(d3d::PATCHLIST_1 + 15), Some(3));
        assert_eq!(ir_primitive_type(d3d::TRIANGLEFAN), None);
    }

    #[test]
    fn index_formats() {
        let i16 = index_format(57).unwrap();
        assert_eq!((i16.mtl, i16.size, i16.ir, i16.restart), (MTLIndexType::UInt16, 2, 1, 0xFFFF));
        let i32 = index_format(42).unwrap();
        assert_eq!((i32.mtl, i32.size, i32.ir, i32.restart), (MTLIndexType::UInt32, 4, 2, 0xFFFF_FFFF));
        assert!(index_format(0).is_none() && index_format(41).is_none());
    }

    #[test]
    fn strip_cut_rules() {
        assert!(strip_cut_matches(strip_cut::CUT_0XFFFF, 57, d3d::TRIANGLESTRIP));
        assert!(strip_cut_matches(strip_cut::CUT_0XFFFFFFFF, 42, d3d::LINESTRIP));
        assert!(!strip_cut_matches(strip_cut::DISABLED, 57, d3d::TRIANGLESTRIP));
        assert!(!strip_cut_matches(strip_cut::CUT_0XFFFF, 42, d3d::TRIANGLESTRIP));
        assert!(strip_cut_matches(strip_cut::DISABLED, 57, d3d::TRIANGLELIST));
    }
}
