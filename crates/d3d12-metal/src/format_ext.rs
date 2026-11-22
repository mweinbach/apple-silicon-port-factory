//! Format rules on top of [`crate::format`]:
//! * DXGI → `MTLVertexFormat` / `MTLAttributeFormat` (input layouts on the Metal vertex-fetch
//!   path; the converter's stage-in path takes DXGI formats directly).
//! * View-cast legality ([`view_cast_ok`]): which SRV/RTV/DSV/UAV format may view a resource
//!   created with a given format. This follows D3D12's typeless-family rules, including
//!   `CastingFullyTypedFormatSupported` (reported TRUE in caps) and the depth/stencil plane views.
//! * sRGB pairs.
//!
//! Owned by the shaders engineer.

use objc2_metal::{MTLAttributeFormat, MTLVertexFormat};

use crate::format::{info, Num};

// ---- vertex formats -----------------------------------------------------------------------------

/// DXGI format of an input element → `MTLVertexFormat`. `None` means it's not a legal
/// vertex format, or Metal has no equivalent (e.g. R10G10B10A2_UINT).
pub fn vertex_format(dxgi: u32) -> Option<MTLVertexFormat> {
    use MTLVertexFormat as V;
    Some(match dxgi {
        2 => V::Float4,
        3 => V::UInt4,
        4 => V::Int4,
        6 => V::Float3,
        7 => V::UInt3,
        8 => V::Int3,
        10 => V::Half4,
        11 => V::UShort4Normalized,
        12 => V::UShort4,
        13 => V::Short4Normalized,
        14 => V::Short4,
        16 => V::Float2,
        17 => V::UInt2,
        18 => V::Int2,
        24 => V::UInt1010102Normalized,
        26 => V::FloatRG11B10,
        28 => V::UChar4Normalized,
        30 => V::UChar4,
        31 => V::Char4Normalized,
        32 => V::Char4,
        34 => V::Half2,
        35 => V::UShort2Normalized,
        36 => V::UShort2,
        37 => V::Short2Normalized,
        38 => V::Short2,
        41 => V::Float,
        42 => V::UInt,
        43 => V::Int,
        49 => V::UChar2Normalized,
        50 => V::UChar2,
        51 => V::Char2Normalized,
        52 => V::Char2,
        54 => V::Half,
        56 => V::UShortNormalized,
        57 => V::UShort,
        58 => V::ShortNormalized,
        59 => V::Short,
        61 => V::UCharNormalized,
        62 => V::UChar,
        63 => V::CharNormalized,
        64 => V::Char,
        67 => V::FloatRGB9E5,
        87 => V::UChar4Normalized_BGRA,
        _ => return None,
    })
}

/// Same mapping as [`vertex_format`] for `MTLStageInputOutputDescriptor` (compute stage-in). The
/// two enums share values.
pub fn attribute_format(dxgi: u32) -> Option<MTLAttributeFormat> {
    vertex_format(dxgi).map(|v| MTLAttributeFormat(v.0))
}

/// Bytes per vertex element (for `D3D12_APPEND_ALIGNED_ELEMENT` resolution). 0 for unknown.
pub fn vertex_format_size(dxgi: u32) -> u32 {
    if vertex_format(dxgi).is_some() {
        info(dxgi).bytes
    } else {
        0
    }
}

// ---- sRGB ---------------------------------------------------------------------------------------

const SRGB_PAIRS: &[(u32, u32)] = &[(28, 29), (71, 72), (74, 75), (77, 78), (87, 91), (88, 93), (98, 99)];

/// The sRGB variant of a linear UNORM format, or of its typeless head (e.g. R8G8B8A8_UNORM /
/// R8G8B8A8_TYPELESS → R8G8B8A8_UNORM_SRGB).
pub fn srgb_of(dxgi: u32) -> Option<u32> {
    let linear = match dxgi {
        27 => 28,
        70 => 71,
        73 => 74,
        76 => 77,
        90 => 87,
        92 => 88,
        97 => 98,
        d => d,
    };
    SRGB_PAIRS.iter().find(|p| p.0 == linear).map(|p| p.1)
}

/// The linear UNORM variant of an sRGB format.
pub fn linear_of(dxgi: u32) -> Option<u32> {
    SRGB_PAIRS.iter().find(|p| p.1 == dxgi).map(|p| p.0)
}

pub fn is_srgb(dxgi: u32) -> bool {
    info(dxgi).num == Num::Srgb
}

// ---- view casting -------------------------------------------------------------------------------

/// Kind of view being created onto a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewKind {
    Srv,
    Rtv,
    Dsv,
    Uav,
}

/// The typeless format heading `dxgi`'s cast family (itself for formats without one).
pub fn family(dxgi: u32) -> u32 {
    match dxgi {
        1..=4 => 1,
        5..=8 => 5,
        9..=14 => 9,
        15..=18 => 15,
        19..=22 => 19,
        23..=25 | 89 => 23,
        27..=32 => 27,
        33..=38 => 33,
        39..=43 => 39,
        44..=47 => 44,
        48..=52 => 48,
        53..=59 => 53,
        60..=64 => 60,
        70..=72 => 70,
        73..=75 => 73,
        76..=78 => 76,
        79..=81 => 79,
        82..=84 => 82,
        87 | 90 | 91 => 90,
        88 | 92 | 93 => 92,
        94..=96 => 94,
        97..=99 => 97,
        d => d,
    }
}

pub fn is_typeless(dxgi: u32) -> bool {
    info(dxgi).num == Num::Typeless || matches!(dxgi, 19 | 44)
}

/// Depth/stencil formats that are only legal as DSVs.
fn is_dsv_format(dxgi: u32) -> bool {
    matches!(dxgi, 20 | 40 | 45 | 55)
}

/// SRV formats reading one plane of a depth(/stencil) resource: (view format, resource family).
const DEPTH_PLANE_SRVS: &[(u32, u32)] = &[
    (41, 39), // R32_FLOAT of R32_TYPELESS (D32)
    (56, 53), // R16_UNORM of R16_TYPELESS (D16)
    (46, 44), // R24_UNORM_X8_TYPELESS (depth plane of D24S8)
    (47, 44), // X24_TYPELESS_G8_UINT (stencil plane of D24S8)
    (21, 19), // R32_FLOAT_X8X24_TYPELESS (depth plane of D32S8)
    (22, 19), // X32_TYPELESS_G8X24_UINT (stencil plane of D32S8)
];

/// Whether a `kind` view with format `view` may be created on a resource of format `resource`.
/// `view == 0` (DXGI_FORMAT_UNKNOWN) means "the resource's format" and is legal only for
/// fully typed resources.
pub fn view_cast_ok(resource: u32, view: u32, kind: ViewKind) -> bool {
    let res_info = info(resource);
    if res_info.planes == 0 && resource != 0 {
        return false; // unknown / video
    }
    let view = if view == 0 {
        if is_typeless(resource) {
            return false;
        }
        resource
    } else {
        view
    };
    let v = info(view);
    if v.planes == 0 || is_typeless(view) {
        return false; // views need a typed format (the *_X8_TYPELESS plane formats count as typed)
    }
    // Kind-specific format legality.
    match kind {
        ViewKind::Dsv => {
            if !is_dsv_format(view) {
                return false;
            }
        }
        ViewKind::Rtv | ViewKind::Uav => {
            if v.is_depth_stencil() || v.is_bc() || v.block_w != 1 {
                return false;
            }
            if kind == ViewKind::Uav && is_srgb(view) {
                return false;
            }
        }
        ViewKind::Srv => {
            if is_dsv_format(view) {
                return false; // D32_FLOAT etc. are not SRV formats; use R32_FLOAT
            }
        }
    }
    if view == resource {
        return true;
    }
    // Depth: DSV formats on their typeless family; SRV plane formats on the typeless family.
    if is_dsv_format(view) {
        return family(view) == family(resource) && is_typeless(resource);
    }
    if let Some(&(_, fam)) = DEPTH_PLANE_SRVS.iter().find(|p| p.0 == view) {
        if fam == 44 || fam == 19 {
            // Plane views of D24S8 / D32S8 resources (typed or typeless): SRV only.
            return family(resource) == fam && kind == ViewKind::Srv;
        }
        // R32_FLOAT / R16_UNORM are ordinary members of their family (below).
    }
    // Ordinary family casts. A typeless resource can be viewed with any member of its family.
    // A fully typed one can too (CastingFullyTypedFormatSupported), but not to or from depth.
    if family(view) != family(resource) {
        return false;
    }
    if res_info.is_depth_stencil() && !is_typeless(resource) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ViewKind::*;

    #[test]
    fn vertex_formats() {
        assert_eq!(vertex_format(2), Some(MTLVertexFormat::Float4));
        assert_eq!(vertex_format(6), Some(MTLVertexFormat::Float3));
        assert_eq!(vertex_format(28), Some(MTLVertexFormat::UChar4Normalized));
        assert_eq!(vertex_format(87), Some(MTLVertexFormat::UChar4Normalized_BGRA));
        assert_eq!(vertex_format(24), Some(MTLVertexFormat::UInt1010102Normalized));
        assert_eq!(vertex_format(25), None); // R10G10B10A2_UINT: no Metal vertex format
        assert_eq!(vertex_format(29), None); // sRGB isn't a vertex format
        assert_eq!(vertex_format(40), None);
        assert_eq!(attribute_format(16), Some(MTLAttributeFormat::Float2));
        assert_eq!((vertex_format_size(6), vertex_format_size(54), vertex_format_size(40)), (12, 2, 0));
        // Every mapped format has a sane size, and normalized/int types agree with the DXGI class.
        for d in 0..=132u32 {
            if let Some(v) = vertex_format(d) {
                assert!(vertex_format_size(d) > 0, "{d}");
                let n = info(d).num;
                let normalized = matches!(v.0, 7..=12 | 19..=24 | 40..=42 | 47 | 48 | 51 | 52);
                let integer = matches!(v.0, 1..=6 | 13..=18 | 32..=39 | 45 | 46 | 49 | 50);
                match n {
                    Num::Unorm | Num::Snorm => assert!(normalized, "{d} {v:?}"),
                    Num::Uint | Num::Sint => assert!(integer, "{d} {v:?}"),
                    _ => assert!(!normalized && !integer, "{d} {v:?}"),
                }
            }
        }
    }

    #[test]
    fn srgb_pairs() {
        assert_eq!(srgb_of(28), Some(29));
        assert_eq!(srgb_of(27), Some(29));
        assert_eq!(srgb_of(87), Some(91));
        assert_eq!(srgb_of(97), Some(99));
        assert_eq!(srgb_of(10), None);
        assert_eq!(linear_of(72), Some(71));
        assert_eq!(linear_of(28), None);
        for &(l, s) in SRGB_PAIRS {
            assert!(is_srgb(s) && !is_srgb(l));
            assert_eq!(info(l).mtl.0 != 0, info(s).mtl.0 != 0);
        }
    }

    #[test]
    fn typeless_families() {
        assert!(view_cast_ok(27, 28, Srv));
        assert!(view_cast_ok(27, 29, Rtv));
        assert!(view_cast_ok(27, 30, Uav));
        assert!(!view_cast_ok(27, 29, Uav)); // no sRGB UAVs
        assert!(!view_cast_ok(27, 0, Srv)); // typeless needs a typed view
        assert!(!view_cast_ok(27, 27, Srv));
        assert!(!view_cast_ok(27, 87, Srv)); // BGRA is another family
        assert!(view_cast_ok(90, 91, Srv));
        assert!(view_cast_ok(70, 72, Srv));
        assert!(!view_cast_ok(70, 71, Rtv)); // BC can't be RTV
        assert!(!view_cast_ok(70, 71, Uav));
        assert!(view_cast_ok(39, 42, Uav));
        assert!(view_cast_ok(39, 41, Uav)); // R32_TYPELESS needn't be depth
        assert!(view_cast_ok(53, 56, Rtv));
        assert!(view_cast_ok(23, 89, Srv));
    }

    #[test]
    fn fully_typed_casts() {
        assert!(view_cast_ok(28, 0, Srv));
        assert!(view_cast_ok(28, 28, Rtv));
        assert!(view_cast_ok(28, 29, Srv)); // CastingFullyTypedFormatSupported
        assert!(view_cast_ok(41, 42, Uav));
        assert!(!view_cast_ok(28, 41, Srv));
        assert!(!view_cast_ok(10, 2, Srv));
    }

    #[test]
    fn depth_views() {
        // D32 via typeless.
        assert!(view_cast_ok(39, 40, Dsv));
        assert!(view_cast_ok(39, 41, Srv));
        assert!(!view_cast_ok(39, 40, Srv)); // D32_FLOAT is not an SRV format
        assert!(!view_cast_ok(39, 40, Rtv));
        // Typed D32: DSV only, no casts.
        assert!(view_cast_ok(40, 0, Dsv));
        assert!(view_cast_ok(40, 40, Dsv));
        assert!(!view_cast_ok(40, 41, Srv));
        assert!(!view_cast_ok(40, 0, Srv));
        // D24S8 and its planes.
        assert!(view_cast_ok(44, 45, Dsv));
        assert!(view_cast_ok(44, 46, Srv));
        assert!(view_cast_ok(44, 47, Srv));
        assert!(view_cast_ok(45, 46, Srv)); // typed D24S8 still exposes planes
        assert!(!view_cast_ok(44, 46, Uav));
        assert!(!view_cast_ok(44, 46, Dsv));
        // D32S8.
        assert!(view_cast_ok(19, 20, Dsv));
        assert!(view_cast_ok(19, 21, Srv));
        assert!(view_cast_ok(19, 22, Srv));
        // D16.
        assert!(view_cast_ok(53, 55, Dsv));
        assert!(view_cast_ok(53, 56, Srv));
        assert!(!view_cast_ok(55, 56, Srv));
        // DSV on colour formats is never legal.
        assert!(!view_cast_ok(28, 28, Dsv));
        assert!(!view_cast_ok(41, 41, Dsv));
        assert!(!view_cast_ok(39, 41, Dsv));
    }

    #[test]
    fn families_are_consistent_with_format_table() {
        for d in 1..=99u32 {
            let f = family(d);
            if f != d && info(d).planes != 0 {
                // Members of a family share the element size, the head included.
                assert_eq!(info(d).bytes, info(f).bytes, "format {d} family {f}");
                assert_eq!(info(d).block_w, info(f).block_w, "format {d} family {f}");
            }
        }
    }
}
