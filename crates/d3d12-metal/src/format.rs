//! DXGI_FORMAT → Metal pixel format table plus the D3D12 format queries
//! (`FORMAT_SUPPORT`, `FORMAT_INFO`, `MULTISAMPLE_QUALITY_LEVELS`) derived
//! from it.
//!
//! Substitutions (Apple GPUs have no 24-bit depth):
//! * `D24_UNORM_S8_UINT`, `R24G8_TYPELESS`, `R24_UNORM_X8_TYPELESS` →
//!   `Depth32Float_Stencil8`; `X24_TYPELESS_G8_UINT` → `X32_Stencil8`.
//! * `D32_FLOAT_S8X24_UINT` family → `Depth32Float_Stencil8` / `X32_Stencil8`.
//! * `B8G8R8X8_*` → `BGRA8*` (alpha must be forced to 1 by view swizzle).
//! * `R32G32B32_*` have no texture format: buffers / vertex data only.

use objc2_metal::MTLPixelFormat as P;

/// Numeric class of a format's channels.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Num {
    Typeless,
    Float,
    Unorm,
    Snorm,
    Uint,
    Sint,
    Srgb,
    Depth,
    DepthStencil,
    Stencil,
    Video,
}

#[derive(Clone, Copy, Debug)]
pub struct FormatInfo {
    pub dxgi: u32,
    pub name: &'static str,
    /// Metal format for textures (`Invalid` = no texture support).
    pub mtl: P,
    /// Bytes per element (per 4x4 block for block-compressed formats).
    pub bytes: u32,
    /// Block width/height in texels (1, or 4 for BC formats; 2x1 for 4:2:2).
    pub block_w: u32,
    pub block_h: u32,
    pub num: Num,
    /// D3D12 plane count (2 for depth+stencil formats).
    pub planes: u32,
    /// Per-channel bits of the widest channel (for capability heuristics).
    pub chan_bits: u32,
    pub channels: u32,
}

impl FormatInfo {
    pub fn is_bc(&self) -> bool {
        self.block_w == 4
    }
    pub fn is_depth_stencil(&self) -> bool {
        matches!(self.num, Num::Depth | Num::DepthStencil | Num::Stencil)
    }
    pub fn has_stencil(&self) -> bool {
        matches!(self.num, Num::DepthStencil | Num::Stencil)
    }
}

const INVALID: P = P::Invalid;

macro_rules! fmts {
    ($( $id:literal $name:ident => $mtl:ident, $bytes:literal, $bw:literal x $bh:literal, $num:ident, $planes:literal, $bits:literal x $ch:literal; )*) => {
        /// Indexed by DXGI_FORMAT value (gaps filled with UNKNOWN entries).
        pub static TABLE: &[FormatInfo] = &[
            $( FormatInfo { dxgi: $id, name: stringify!($name), mtl: P::$mtl, bytes: $bytes, block_w: $bw, block_h: $bh, num: Num::$num, planes: $planes, chan_bits: $bits, channels: $ch }, )*
        ];
    };
}

fmts! {
    0 UNKNOWN => Invalid, 0, 1 x 1, Typeless, 0, 0 x 0;
    1 R32G32B32A32_TYPELESS => RGBA32Float, 16, 1 x 1, Typeless, 1, 32 x 4;
    2 R32G32B32A32_FLOAT => RGBA32Float, 16, 1 x 1, Float, 1, 32 x 4;
    3 R32G32B32A32_UINT => RGBA32Uint, 16, 1 x 1, Uint, 1, 32 x 4;
    4 R32G32B32A32_SINT => RGBA32Sint, 16, 1 x 1, Sint, 1, 32 x 4;
    5 R32G32B32_TYPELESS => Invalid, 12, 1 x 1, Typeless, 1, 32 x 3;
    6 R32G32B32_FLOAT => Invalid, 12, 1 x 1, Float, 1, 32 x 3;
    7 R32G32B32_UINT => Invalid, 12, 1 x 1, Uint, 1, 32 x 3;
    8 R32G32B32_SINT => Invalid, 12, 1 x 1, Sint, 1, 32 x 3;
    9 R16G16B16A16_TYPELESS => RGBA16Float, 8, 1 x 1, Typeless, 1, 16 x 4;
    10 R16G16B16A16_FLOAT => RGBA16Float, 8, 1 x 1, Float, 1, 16 x 4;
    11 R16G16B16A16_UNORM => RGBA16Unorm, 8, 1 x 1, Unorm, 1, 16 x 4;
    12 R16G16B16A16_UINT => RGBA16Uint, 8, 1 x 1, Uint, 1, 16 x 4;
    13 R16G16B16A16_SNORM => RGBA16Snorm, 8, 1 x 1, Snorm, 1, 16 x 4;
    14 R16G16B16A16_SINT => RGBA16Sint, 8, 1 x 1, Sint, 1, 16 x 4;
    15 R32G32_TYPELESS => RG32Float, 8, 1 x 1, Typeless, 1, 32 x 2;
    16 R32G32_FLOAT => RG32Float, 8, 1 x 1, Float, 1, 32 x 2;
    17 R32G32_UINT => RG32Uint, 8, 1 x 1, Uint, 1, 32 x 2;
    18 R32G32_SINT => RG32Sint, 8, 1 x 1, Sint, 1, 32 x 2;
    19 R32G8X24_TYPELESS => Depth32Float_Stencil8, 8, 1 x 1, DepthStencil, 2, 32 x 2;
    20 D32_FLOAT_S8X24_UINT => Depth32Float_Stencil8, 8, 1 x 1, DepthStencil, 2, 32 x 2;
    21 R32_FLOAT_X8X24_TYPELESS => Depth32Float_Stencil8, 8, 1 x 1, DepthStencil, 2, 32 x 2;
    22 X32_TYPELESS_G8X24_UINT => X32_Stencil8, 8, 1 x 1, Stencil, 2, 32 x 2;
    23 R10G10B10A2_TYPELESS => RGB10A2Unorm, 4, 1 x 1, Typeless, 1, 10 x 4;
    24 R10G10B10A2_UNORM => RGB10A2Unorm, 4, 1 x 1, Unorm, 1, 10 x 4;
    25 R10G10B10A2_UINT => RGB10A2Uint, 4, 1 x 1, Uint, 1, 10 x 4;
    26 R11G11B10_FLOAT => RG11B10Float, 4, 1 x 1, Float, 1, 11 x 3;
    27 R8G8B8A8_TYPELESS => RGBA8Unorm, 4, 1 x 1, Typeless, 1, 8 x 4;
    28 R8G8B8A8_UNORM => RGBA8Unorm, 4, 1 x 1, Unorm, 1, 8 x 4;
    29 R8G8B8A8_UNORM_SRGB => RGBA8Unorm_sRGB, 4, 1 x 1, Srgb, 1, 8 x 4;
    30 R8G8B8A8_UINT => RGBA8Uint, 4, 1 x 1, Uint, 1, 8 x 4;
    31 R8G8B8A8_SNORM => RGBA8Snorm, 4, 1 x 1, Snorm, 1, 8 x 4;
    32 R8G8B8A8_SINT => RGBA8Sint, 4, 1 x 1, Sint, 1, 8 x 4;
    33 R16G16_TYPELESS => RG16Float, 4, 1 x 1, Typeless, 1, 16 x 2;
    34 R16G16_FLOAT => RG16Float, 4, 1 x 1, Float, 1, 16 x 2;
    35 R16G16_UNORM => RG16Unorm, 4, 1 x 1, Unorm, 1, 16 x 2;
    36 R16G16_UINT => RG16Uint, 4, 1 x 1, Uint, 1, 16 x 2;
    37 R16G16_SNORM => RG16Snorm, 4, 1 x 1, Snorm, 1, 16 x 2;
    38 R16G16_SINT => RG16Sint, 4, 1 x 1, Sint, 1, 16 x 2;
    39 R32_TYPELESS => R32Float, 4, 1 x 1, Typeless, 1, 32 x 1;
    40 D32_FLOAT => Depth32Float, 4, 1 x 1, Depth, 1, 32 x 1;
    41 R32_FLOAT => R32Float, 4, 1 x 1, Float, 1, 32 x 1;
    42 R32_UINT => R32Uint, 4, 1 x 1, Uint, 1, 32 x 1;
    43 R32_SINT => R32Sint, 4, 1 x 1, Sint, 1, 32 x 1;
    44 R24G8_TYPELESS => Depth32Float_Stencil8, 4, 1 x 1, DepthStencil, 2, 24 x 2;
    45 D24_UNORM_S8_UINT => Depth32Float_Stencil8, 4, 1 x 1, DepthStencil, 2, 24 x 2;
    46 R24_UNORM_X8_TYPELESS => Depth32Float_Stencil8, 4, 1 x 1, DepthStencil, 2, 24 x 2;
    47 X24_TYPELESS_G8_UINT => X32_Stencil8, 4, 1 x 1, Stencil, 2, 24 x 2;
    48 R8G8_TYPELESS => RG8Unorm, 2, 1 x 1, Typeless, 1, 8 x 2;
    49 R8G8_UNORM => RG8Unorm, 2, 1 x 1, Unorm, 1, 8 x 2;
    50 R8G8_UINT => RG8Uint, 2, 1 x 1, Uint, 1, 8 x 2;
    51 R8G8_SNORM => RG8Snorm, 2, 1 x 1, Snorm, 1, 8 x 2;
    52 R8G8_SINT => RG8Sint, 2, 1 x 1, Sint, 1, 8 x 2;
    53 R16_TYPELESS => R16Float, 2, 1 x 1, Typeless, 1, 16 x 1;
    54 R16_FLOAT => R16Float, 2, 1 x 1, Float, 1, 16 x 1;
    55 D16_UNORM => Depth16Unorm, 2, 1 x 1, Depth, 1, 16 x 1;
    56 R16_UNORM => R16Unorm, 2, 1 x 1, Unorm, 1, 16 x 1;
    57 R16_UINT => R16Uint, 2, 1 x 1, Uint, 1, 16 x 1;
    58 R16_SNORM => R16Snorm, 2, 1 x 1, Snorm, 1, 16 x 1;
    59 R16_SINT => R16Sint, 2, 1 x 1, Sint, 1, 16 x 1;
    60 R8_TYPELESS => R8Unorm, 1, 1 x 1, Typeless, 1, 8 x 1;
    61 R8_UNORM => R8Unorm, 1, 1 x 1, Unorm, 1, 8 x 1;
    62 R8_UINT => R8Uint, 1, 1 x 1, Uint, 1, 8 x 1;
    63 R8_SNORM => R8Snorm, 1, 1 x 1, Snorm, 1, 8 x 1;
    64 R8_SINT => R8Sint, 1, 1 x 1, Sint, 1, 8 x 1;
    65 A8_UNORM => A8Unorm, 1, 1 x 1, Unorm, 1, 8 x 1;
    66 R1_UNORM => Invalid, 1, 8 x 1, Unorm, 1, 1 x 1;
    67 R9G9B9E5_SHAREDEXP => RGB9E5Float, 4, 1 x 1, Float, 1, 9 x 3;
    68 R8G8_B8G8_UNORM => Invalid, 4, 2 x 1, Unorm, 1, 8 x 3;
    69 G8R8_G8B8_UNORM => Invalid, 4, 2 x 1, Unorm, 1, 8 x 3;
    70 BC1_TYPELESS => BC1_RGBA, 8, 4 x 4, Typeless, 1, 8 x 4;
    71 BC1_UNORM => BC1_RGBA, 8, 4 x 4, Unorm, 1, 8 x 4;
    72 BC1_UNORM_SRGB => BC1_RGBA_sRGB, 8, 4 x 4, Srgb, 1, 8 x 4;
    73 BC2_TYPELESS => BC2_RGBA, 16, 4 x 4, Typeless, 1, 8 x 4;
    74 BC2_UNORM => BC2_RGBA, 16, 4 x 4, Unorm, 1, 8 x 4;
    75 BC2_UNORM_SRGB => BC2_RGBA_sRGB, 16, 4 x 4, Srgb, 1, 8 x 4;
    76 BC3_TYPELESS => BC3_RGBA, 16, 4 x 4, Typeless, 1, 8 x 4;
    77 BC3_UNORM => BC3_RGBA, 16, 4 x 4, Unorm, 1, 8 x 4;
    78 BC3_UNORM_SRGB => BC3_RGBA_sRGB, 16, 4 x 4, Srgb, 1, 8 x 4;
    79 BC4_TYPELESS => BC4_RUnorm, 8, 4 x 4, Typeless, 1, 8 x 1;
    80 BC4_UNORM => BC4_RUnorm, 8, 4 x 4, Unorm, 1, 8 x 1;
    81 BC4_SNORM => BC4_RSnorm, 8, 4 x 4, Snorm, 1, 8 x 1;
    82 BC5_TYPELESS => BC5_RGUnorm, 16, 4 x 4, Typeless, 1, 8 x 2;
    83 BC5_UNORM => BC5_RGUnorm, 16, 4 x 4, Unorm, 1, 8 x 2;
    84 BC5_SNORM => BC5_RGSnorm, 16, 4 x 4, Snorm, 1, 8 x 2;
    85 B5G6R5_UNORM => B5G6R5Unorm, 2, 1 x 1, Unorm, 1, 6 x 3;
    86 B5G5R5A1_UNORM => BGR5A1Unorm, 2, 1 x 1, Unorm, 1, 5 x 4;
    87 B8G8R8A8_UNORM => BGRA8Unorm, 4, 1 x 1, Unorm, 1, 8 x 4;
    88 B8G8R8X8_UNORM => BGRA8Unorm, 4, 1 x 1, Unorm, 1, 8 x 3;
    89 R10G10B10_XR_BIAS_A2_UNORM => Invalid, 4, 1 x 1, Unorm, 1, 10 x 4;
    90 B8G8R8A8_TYPELESS => BGRA8Unorm, 4, 1 x 1, Typeless, 1, 8 x 4;
    91 B8G8R8A8_UNORM_SRGB => BGRA8Unorm_sRGB, 4, 1 x 1, Srgb, 1, 8 x 4;
    92 B8G8R8X8_TYPELESS => BGRA8Unorm, 4, 1 x 1, Typeless, 1, 8 x 3;
    93 B8G8R8X8_UNORM_SRGB => BGRA8Unorm_sRGB, 4, 1 x 1, Srgb, 1, 8 x 3;
    94 BC6H_TYPELESS => BC6H_RGBUfloat, 16, 4 x 4, Typeless, 1, 16 x 3;
    95 BC6H_UF16 => BC6H_RGBUfloat, 16, 4 x 4, Float, 1, 16 x 3;
    96 BC6H_SF16 => BC6H_RGBFloat, 16, 4 x 4, Float, 1, 16 x 3;
    97 BC7_TYPELESS => BC7_RGBAUnorm, 16, 4 x 4, Typeless, 1, 8 x 4;
    98 BC7_UNORM => BC7_RGBAUnorm, 16, 4 x 4, Unorm, 1, 8 x 4;
    99 BC7_UNORM_SRGB => BC7_RGBAUnorm_sRGB, 16, 4 x 4, Srgb, 1, 8 x 4;
}

static UNKNOWN: FormatInfo = FormatInfo {
    dxgi: 0,
    name: "UNKNOWN",
    mtl: INVALID,
    bytes: 0,
    block_w: 1,
    block_h: 1,
    num: Num::Video,
    planes: 0,
    chan_bits: 0,
    channels: 0,
};

/// Format info for a DXGI_FORMAT value (video / unknown formats → an
/// entry with `planes == 0`).
pub fn info(dxgi: u32) -> &'static FormatInfo {
    TABLE.get(dxgi as usize).unwrap_or(&UNKNOWN)
}

/// The first typed (non-typeless) DXGI format stored as Metal format `mtl`.
pub fn typed_for_mtl(mtl: P) -> Option<&'static FormatInfo> {
    TABLE.iter().find(|f| f.mtl == mtl && mtl != INVALID && f.num != Num::Typeless)
}

pub fn name(dxgi: u32) -> String {
    match TABLE.get(dxgi as usize) {
        Some(f) => format!("DXGI_FORMAT_{}", f.name),
        None => format!("DXGI_FORMAT({dxgi})"),
    }
}

/// Metal format of a *resource* created with `dxgi`. Typeless formats that
/// may be bound as depth (resource flag ALLOW_DEPTH_STENCIL) get the depth
/// format of their family.
pub fn resource_format(dxgi: u32, allow_depth: bool) -> P {
    if allow_depth {
        match dxgi {
            39 | 41 => return P::Depth32Float,      // R32_TYPELESS / R32_FLOAT
            53 | 56 => return P::Depth16Unorm,      // R16_TYPELESS / R16_UNORM
            44..=47 => return P::Depth32Float_Stencil8,
            19..=22 => return P::Depth32Float_Stencil8,
            _ => {}
        }
    }
    info(dxgi).mtl
}

/// Metal format of a *view* (SRV/UAV/RTV/DSV) with format `dxgi` onto a
/// resource whose Metal format is `res`.
pub fn view_format(dxgi: u32, res: P) -> P {
    match dxgi {
        // Depth plane of D24S8 / D32S8 resources. Metal can't view
        // Depth32Float_Stencil8 as Depth32Float; sampling the combined
        // texture itself reads depth.
        46 | 21 | 45 | 20 | 44 | 19 => {
            if res == P::Depth32Float_Stencil8 {
                return P::Depth32Float_Stencil8;
            }
        }
        47 | 22 => return P::X32_Stencil8,
        // R32_FLOAT SRV of a D32 resource.
        41 | 39 if res == P::Depth32Float => return P::Depth32Float,
        56 | 53 if res == P::Depth16Unorm => return P::Depth16Unorm,
        _ => {}
    }
    if dxgi == 0 {
        return res;
    }
    info(dxgi).mtl
}

// ---------------------------------------------------------------------------
// D3D12_FORMAT_SUPPORT1/2
// ---------------------------------------------------------------------------

pub mod support1 {
    pub const BUFFER: u32 = 0x1;
    pub const IA_VERTEX_BUFFER: u32 = 0x2;
    pub const IA_INDEX_BUFFER: u32 = 0x4;
    pub const TEXTURE1D: u32 = 0x10;
    pub const TEXTURE2D: u32 = 0x20;
    pub const TEXTURE3D: u32 = 0x40;
    pub const TEXTURECUBE: u32 = 0x80;
    pub const SHADER_LOAD: u32 = 0x100;
    pub const SHADER_SAMPLE: u32 = 0x200;
    pub const SHADER_SAMPLE_COMPARISON: u32 = 0x400;
    pub const MIP: u32 = 0x1000;
    pub const RENDER_TARGET: u32 = 0x4000;
    pub const BLENDABLE: u32 = 0x8000;
    pub const DEPTH_STENCIL: u32 = 0x10000;
    pub const MULTISAMPLE_RESOLVE: u32 = 0x40000;
    pub const DISPLAY: u32 = 0x80000;
    pub const CAST_WITHIN_BIT_LAYOUT: u32 = 0x100000;
    pub const MULTISAMPLE_RENDERTARGET: u32 = 0x200000;
    pub const MULTISAMPLE_LOAD: u32 = 0x400000;
    pub const SHADER_GATHER: u32 = 0x800000;
    pub const BACK_BUFFER_CAST: u32 = 0x1000000;
    pub const TYPED_UNORDERED_ACCESS_VIEW: u32 = 0x2000000;
    pub const SHADER_GATHER_COMPARISON: u32 = 0x4000000;
}

pub mod support2 {
    pub const UAV_ATOMIC_ADD: u32 = 0x1;
    pub const UAV_ATOMIC_BITWISE_OPS: u32 = 0x2;
    pub const UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: u32 = 0x4;
    pub const UAV_ATOMIC_EXCHANGE: u32 = 0x8;
    pub const UAV_ATOMIC_SIGNED_MIN_OR_MAX: u32 = 0x10;
    pub const UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: u32 = 0x20;
    pub const UAV_TYPED_LOAD: u32 = 0x40;
    pub const UAV_TYPED_STORE: u32 = 0x80;
}

/// Metal "read-write texture tier 2" formats (shader `read_write` access):
/// RGBA32/RGBA16/RGBA8 float/uint/sint/unorm/snorm, R32/R16/R8 float/uint/sint.
fn metal_rw_tier2(f: &FormatInfo) -> bool {
    use Num::*;
    match (f.channels, f.chan_bits) {
        (4, 32) => matches!(f.num, Float | Uint | Sint),
        (4, 16) => matches!(f.num, Float | Uint | Sint | Unorm | Snorm),
        (4, 8) => matches!(f.num, Unorm | Uint | Sint | Snorm) && f.mtl != P::BGRA8Unorm,
        (1, 32) => matches!(f.num, Float | Uint | Sint),
        (1, 16) => matches!(f.num, Float | Uint | Sint),
        (1, 8) => matches!(f.num, Unorm | Uint | Sint) && f.mtl != P::A8Unorm,
        _ => false,
    }
}

/// `(Support1, Support2)` for `D3D12_FEATURE_FORMAT_SUPPORT`.
pub fn support(dxgi: u32) -> (u32, u32) {
    use support1::*;
    use support2::*;
    let f = info(dxgi);
    if f.planes == 0 && dxgi != 0 {
        return (0, 0);
    }
    if dxgi == 0 {
        return (BUFFER, 0);
    }
    let mut s1 = 0;
    let mut s2 = 0;
    let numeric = !matches!(f.num, Num::Typeless | Num::Video);
    // Buffers: every numeric non-block non-depth format can be a typed
    // buffer / vertex element.
    if numeric && !f.is_bc() && !f.is_depth_stencil() && f.block_w == 1 {
        s1 |= BUFFER | IA_VERTEX_BUFFER | SHADER_LOAD;
        if dxgi == 42 || dxgi == 57 {
            s1 |= IA_INDEX_BUFFER;
        }
    }
    if f.mtl == INVALID {
        return (s1, s2);
    }
    s1 |= TEXTURE2D | MIP | CAST_WITHIN_BIT_LAYOUT;
    if f.num == Num::Typeless {
        if !f.is_bc() {
            s1 |= TEXTURE1D | TEXTURE3D | TEXTURECUBE;
        } else {
            s1 |= TEXTURE3D | TEXTURECUBE;
        }
        return (s1, s2);
    }
    if f.is_depth_stencil() {
        s1 |= TEXTURE1D | TEXTURECUBE | DEPTH_STENCIL | MULTISAMPLE_RENDERTARGET;
        if f.num != Num::Stencil {
            s1 |= SHADER_LOAD | SHADER_SAMPLE | SHADER_SAMPLE_COMPARISON | SHADER_GATHER | SHADER_GATHER_COMPARISON | MULTISAMPLE_LOAD;
        } else {
            s1 |= SHADER_LOAD | MULTISAMPLE_LOAD;
        }
        return (s1, s2);
    }
    s1 |= TEXTURE3D | TEXTURECUBE | SHADER_LOAD | SHADER_GATHER;
    if !f.is_bc() {
        s1 |= TEXTURE1D;
    }
    let integer = matches!(f.num, Num::Uint | Num::Sint);
    if !integer {
        s1 |= SHADER_SAMPLE;
    }
    if f.is_bc() || f.mtl == P::RGB9E5Float {
        return (s1, s2);
    }
    // Colour-renderable.
    s1 |= RENDER_TARGET | MULTISAMPLE_RENDERTARGET | MULTISAMPLE_LOAD;
    if !integer {
        s1 |= BLENDABLE | MULTISAMPLE_RESOLVE;
    }
    if matches!(dxgi, 24 | 28 | 29 | 87 | 91 | 10) {
        s1 |= DISPLAY | BACK_BUFFER_CAST;
    }
    // Typed UAV: Metal can write every renderable colour format from shaders;
    // typed loads need read_write tier 2 (others are reported anyway for
    // R11G11B10/R10G10B10A2 so the game's OPTIONS gate holds: [A] see memo).
    s1 |= TYPED_UNORDERED_ACCESS_VIEW;
    s2 |= UAV_TYPED_STORE;
    if metal_rw_tier2(f) {
        s2 |= UAV_TYPED_LOAD;
    }
    if matches!(dxgi, 42 | 43) {
        s2 |= UAV_ATOMIC_ADD | UAV_ATOMIC_BITWISE_OPS | UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE | UAV_ATOMIC_EXCHANGE;
        s2 |= if dxgi == 43 { UAV_ATOMIC_SIGNED_MIN_OR_MAX } else { UAV_ATOMIC_UNSIGNED_MIN_OR_MAX };
    }
    (s1, s2)
}

/// Supported MSAA sample counts (Apple GPUs: 1, 2, 4; 8 not on Apple7+ for
/// all formats — be conservative).
pub fn msaa_quality_levels(dxgi: u32, samples: u32) -> u32 {
    let f = info(dxgi);
    if f.mtl == INVALID || f.is_bc() || f.num == Num::Typeless && dxgi != 0 {
        return if samples == 1 && f.mtl != INVALID { 1 } else { 0 };
    }
    match samples {
        1 | 2 | 4 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_indexed_by_value() {
        for (i, f) in TABLE.iter().enumerate() {
            assert_eq!(f.dxgi as usize, i, "{}", f.name);
        }
        assert_eq!(info(45).mtl, P::Depth32Float_Stencil8);
        assert_eq!(info(87).name, "B8G8R8A8_UNORM");
        assert_eq!(info(130).planes, 0);
    }

    #[test]
    fn supports() {
        let (s1, s2) = support(28);
        assert!(s1 & support1::RENDER_TARGET != 0 && s1 & support1::DISPLAY != 0);
        assert!(s2 & support2::UAV_TYPED_LOAD != 0);
        let (s1, _) = support(45);
        assert!(s1 & support1::DEPTH_STENCIL != 0);
        let (s1, _) = support(6);
        assert_eq!(s1 & support1::TEXTURE2D, 0);
        assert!(s1 & support1::IA_VERTEX_BUFFER != 0);
    }
}
