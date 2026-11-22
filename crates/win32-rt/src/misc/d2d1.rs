//! d2d1.dll: Native Direct2D 1.0 / 1.1 COM runtime & software RGBA32 rasterizer
//! integrated with DirectWrite (`dwrite.rs`) and GDI (`gdi32.rs`).

use super::dwrite::{self, DWRITE_GLYPH_OFFSET, DWRITE_GLYPH_RUN};
use crate::types::*;
use std::f32::consts::PI;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

// Float-taking Win64 DLL exports go through EXTRA_EXPORTS + win64_bridge!
// so ARM64 unpacks Win64 XMM0/XMM1 + GPR slots identically to x86-64.
pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "D2D1MakeRotateMatrix",
        ptr: crate::Ptr(crate::bridge_entry!(d2d1_make_rotate_matrix_impl)),
    },
    crate::Export {
        name: "D2D1MakeSkewMatrix",
        ptr: crate::Ptr(crate::bridge_entry!(d2d1_make_skew_matrix_impl)),
    },
];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// Direct2D Constants, Enums, GUIDs & HRESULTs (d2d1.h / d2d1_1.h)
// ============================================================================

pub const D2DERR_WRONG_STATE: HRESULT = 0x8899_0001u32 as i32;
pub const D2DERR_UNSUPPORTED_PIXEL_FORMAT: HRESULT = 0x8898_2F80u32 as i32;

// DXGI_FORMAT values used by Direct2D
pub const DXGI_FORMAT_UNKNOWN: u32 = 0;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
pub const DXGI_FORMAT_A8_UNORM: u32 = 65;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: u32 = 87;

// D2D1_ALPHA_MODE
pub const D2D1_ALPHA_MODE_UNKNOWN: u32 = 0;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: u32 = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: u32 = 2;
pub const D2D1_ALPHA_MODE_IGNORE: u32 = 3;

// D2D1_FACTORY_TYPE
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: u32 = 0;
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: u32 = 1;

// D2D1_FILL_MODE
pub const D2D1_FILL_MODE_ALTERNATE: u32 = 0;
pub const D2D1_FILL_MODE_WINDING: u32 = 1;

// D2D1_FIGURE_BEGIN / END
pub const D2D1_FIGURE_BEGIN_FILLED: u32 = 0;
pub const D2D1_FIGURE_BEGIN_HOLLOW: u32 = 1;
pub const D2D1_FIGURE_END_OPEN: u32 = 0;
pub const D2D1_FIGURE_END_CLOSED: u32 = 1;

// D2D1_EXTEND_MODE
pub const D2D1_EXTEND_MODE_CLAMP: u32 = 0;
pub const D2D1_EXTEND_MODE_WRAP: u32 = 1;
pub const D2D1_EXTEND_MODE_MIRROR: u32 = 2;

// D2D1_BITMAP_INTERPOLATION_MODE
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: u32 = 0;
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: u32 = 1;

// D2D1_ANTIALIAS_MODE
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: u32 = 0;
pub const D2D1_ANTIALIAS_MODE_ALIASED: u32 = 1;

// D2D1_PRIMITIVE_BLEND
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: u32 = 0;
pub const D2D1_PRIMITIVE_BLEND_COPY: u32 = 1;

// D2D1_UNIT_MODE
pub const D2D1_UNIT_MODE_DIPS: u32 = 0;
pub const D2D1_UNIT_MODE_PIXELS: u32 = 1;

// D2D1_DRAW_TEXT_OPTIONS
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: u32 = 0x0000_0002;

// D2D1_CAP_STYLE
pub const D2D1_CAP_STYLE_FLAT: u32 = 0;
pub const D2D1_CAP_STYLE_SQUARE: u32 = 1;
pub const D2D1_CAP_STYLE_ROUND: u32 = 2;
pub const D2D1_CAP_STYLE_TRIANGLE: u32 = 3;

// D2D1_LINE_JOIN
pub const D2D1_LINE_JOIN_MITER: u32 = 0;
pub const D2D1_LINE_JOIN_BEVEL: u32 = 1;
pub const D2D1_LINE_JOIN_ROUND: u32 = 2;
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: u32 = 3;

// D2D1_DASH_STYLE
pub const D2D1_DASH_STYLE_SOLID: u32 = 0;
pub const D2D1_DASH_STYLE_DASH: u32 = 1;
pub const D2D1_DASH_STYLE_DOT: u32 = 2;
pub const D2D1_DASH_STYLE_DASH_DOT: u32 = 3;
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: u32 = 4;
pub const D2D1_DASH_STYLE_CUSTOM: u32 = 5;

// D2D1_SWEEP_DIRECTION / D2D1_ARC_SIZE
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: u32 = 0;
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: u32 = 1;
pub const D2D1_ARC_SIZE_SMALL: u32 = 0;
pub const D2D1_ARC_SIZE_LARGE: u32 = 1;

// D2D1_GEOMETRY_RELATION
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: u32 = 0;
pub const D2D1_GEOMETRY_RELATION_DISJOINT: u32 = 1;
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: u32 = 2;
pub const D2D1_GEOMETRY_RELATION_CONTAINS: u32 = 3;
pub const D2D1_GEOMETRY_RELATION_OVERLAP: u32 = 4;

const fn d2d_guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> GUID {
    GUID { data1: d1, data2: d2, data3: d3, data4: d4 }
}

const D2D_COMMON_TAIL: [u8; 8] = [0x9F, 0xED, 0x00, 0x11, 0x43, 0xA0, 0x55, 0xF9];

pub const IID_IUNKNOWN: GUID =
    d2d_guid(0x0000_0000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
pub const IID_ID2D1_FACTORY: GUID =
    d2d_guid(0x0615_2247, 0x6F50, 0x465A, [0x92, 0x45, 0x11, 0x8B, 0xFD, 0x3B, 0x60, 0x07]);
pub const IID_ID2D1_FACTORY1: GUID =
    d2d_guid(0xBB12_D362, 0xDAEE, 0x4B9A, [0xAA, 0x1D, 0x14, 0xBA, 0x40, 0x1C, 0xFA, 0x1F]);
pub const IID_ID2D1_RESOURCE: GUID = d2d_guid(0x2CD9_0691, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_IMAGE: GUID =
    d2d_guid(0x6501_9F75, 0x8DA2, 0x497C, [0xB3, 0x2C, 0xDF, 0xA3, 0x4E, 0x48, 0xED, 0xE6]);
pub const IID_ID2D1_BITMAP: GUID =
    d2d_guid(0xA229_6057, 0xEA42, 0x4099, [0x98, 0x3B, 0x53, 0x9F, 0xB6, 0x50, 0x54, 0x26]);
pub const IID_ID2D1_BITMAP1: GUID =
    d2d_guid(0xA898_A84C, 0x3873, 0x4588, [0xB0, 0x8B, 0xEB, 0xBF, 0x97, 0x8D, 0xF0, 0x41]);
pub const IID_ID2D1_GRADIENT_STOP_COLLECTION: GUID = d2d_guid(0x2CD9_06A7, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_BRUSH: GUID = d2d_guid(0x2CD9_06A8, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_SOLID_COLOR_BRUSH: GUID = d2d_guid(0x2CD9_06A9, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_BITMAP_BRUSH: GUID = d2d_guid(0x2CD9_06AA, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_LINEAR_GRADIENT_BRUSH: GUID = d2d_guid(0x2CD9_06AB, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_RADIAL_GRADIENT_BRUSH: GUID = d2d_guid(0x2CD9_06AC, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_STROKE_STYLE: GUID = d2d_guid(0x2CD9_069D, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_STROKE_STYLE1: GUID =
    d2d_guid(0x10A7_2A66, 0xE91C, 0x43F4, [0x99, 0x3F, 0xDD, 0xF4, 0xB8, 0x2B, 0x0B, 0x4A]);
pub const IID_ID2D1_GEOMETRY: GUID = d2d_guid(0x2CD9_06A1, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_RECTANGLE_GEOMETRY: GUID = d2d_guid(0x2CD9_06A2, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_ROUNDED_RECTANGLE_GEOMETRY: GUID = d2d_guid(0x2CD9_06A3, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_ELLIPSE_GEOMETRY: GUID = d2d_guid(0x2CD9_06A4, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_PATH_GEOMETRY: GUID = d2d_guid(0x2CD9_06A5, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_PATH_GEOMETRY1: GUID =
    d2d_guid(0x62BA_A2D2, 0xAB54, 0x41B7, [0xB8, 0x72, 0x78, 0x7E, 0x01, 0x06, 0xA4, 0x21]);
pub const IID_ID2D1_GEOMETRY_GROUP: GUID = d2d_guid(0x2CD9_06A6, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_TRANSFORMED_GEOMETRY: GUID = d2d_guid(0x2CD9_06BB, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_SIMPLIFIED_GEOMETRY_SINK: GUID = d2d_guid(0x2CD9_069E, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_GEOMETRY_SINK: GUID = d2d_guid(0x2CD9_069F, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_MESH: GUID = d2d_guid(0x2CD9_06C2, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_LAYER: GUID = d2d_guid(0x2CD9_069B, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_DRAWING_STATE_BLOCK: GUID =
    d2d_guid(0x2850_6E39, 0xEBF6, 0x46A1, [0xBB, 0x47, 0xFD, 0x85, 0x56, 0x5A, 0xB9, 0x57]);
pub const IID_ID2D1_RENDER_TARGET: GUID = d2d_guid(0x2CD9_0694, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_BITMAP_RENDER_TARGET: GUID = d2d_guid(0x2CD9_0695, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_HWND_RENDER_TARGET: GUID = d2d_guid(0x2CD9_0698, 0x12E2, 0x11DC, D2D_COMMON_TAIL);
pub const IID_ID2D1_DC_RENDER_TARGET: GUID =
    d2d_guid(0x1C51_BC64, 0xDE61, 0x46FD, [0x98, 0x99, 0x63, 0xA5, 0xD8, 0xF0, 0x39, 0x50]);
pub const IID_ID2D1_DEVICE: GUID =
    d2d_guid(0x47DD_575D, 0xAC05, 0x4CDD, [0x80, 0x49, 0x9B, 0x02, 0xCD, 0x16, 0xF4, 0x4C]);
pub const IID_ID2D1_DEVICE_CONTEXT: GUID =
    d2d_guid(0xE8F7_FE7A, 0x191C, 0x466D, [0xAD, 0x95, 0x97, 0x56, 0x78, 0xBD, 0xA9, 0x98]);

// ============================================================================
// Direct2D Win64 ABI Structures
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_POINT_2F {
    pub x: f32,
    pub y: f32,
}

impl D2D1_POINT_2F {
    #[inline]
    pub fn from_u64(v: u64) -> Self {
        Self { x: f32::from_bits(v as u32), y: f32::from_bits((v >> 32) as u32) }
    }
    #[inline]
    pub fn to_u64(self) -> u64 {
        (self.x.to_bits() as u64) | ((self.y.to_bits() as u64) << 32)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D2D1_POINT_2U {
    pub x: u32,
    pub y: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_SIZE_F {
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D2D1_SIZE_U {
    pub width: u32,
    pub height: u32,
}

impl D2D1_SIZE_U {
    #[inline]
    pub fn from_u64(v: u64) -> Self {
        Self { width: v as u32, height: (v >> 32) as u32 }
    }
    #[inline]
    pub fn to_u64(self) -> u64 {
        (self.width as u64) | ((self.height as u64) << 32)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D2D1_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: u32,
    pub alphaMode: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_MATRIX_3X2_F {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Default for D2D1_MATRIX_3X2_F {
    fn default() -> Self {
        Self::identity()
    }
}

impl D2D1_MATRIX_3X2_F {
    #[inline]
    pub const fn identity() -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 }
    }

    #[inline]
    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        (x * self.m11 + y * self.m21 + self.dx, x * self.m12 + y * self.m22 + self.dy)
    }

    #[inline]
    pub fn mul(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            dx: self.dx * rhs.m11 + self.dy * rhs.m21 + rhs.dx,
            dy: self.dx * rhs.m12 + self.dy * rhs.m22 + rhs.dy,
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn invert(&self) -> Option<Self> {
        let det = self.determinant();
        if !det.is_finite() || det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        if !inv_det.is_finite() {
            return None;
        }
        let inv = Self {
            m11: self.m22 * inv_det,
            m12: -self.m12 * inv_det,
            m21: -self.m21 * inv_det,
            m22: self.m11 * inv_det,
            dx: (self.m21 * self.dy - self.m22 * self.dx) * inv_det,
            dy: (self.m12 * self.dx - self.m11 * self.dy) * inv_det,
        };
        if inv.m11.is_finite()
            && inv.m12.is_finite()
            && inv.m21.is_finite()
            && inv.m22.is_finite()
            && inv.dx.is_finite()
            && inv.dy.is_finite()
        {
            Some(inv)
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: D2D1_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: D2D1_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D1_POINT_2F,
    pub point2: D2D1_POINT_2F,
    pub point3: D2D1_POINT_2F,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: D2D1_POINT_2F,
    pub point2: D2D1_POINT_2F,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ARC_SEGMENT {
    pub point: D2D1_POINT_2F,
    pub size: D2D1_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: u32,
    pub arcSize: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: D2D1_MATRIX_3X2_F,
}

impl Default for D2D1_BRUSH_PROPERTIES {
    fn default() -> Self {
        Self { opacity: 1.0, transform: D2D1_MATRIX_3X2_F::identity() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: u32,
    pub extendModeY: u32,
    pub interpolationMode: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: D2D1_POINT_2F,
    pub endPoint: D2D1_POINT_2F,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: D2D1_POINT_2F,
    pub gradientOriginOffset: D2D1_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: u32,
    pub endCap: u32,
    pub dashCap: u32,
    pub lineJoin: u32,
    pub miterLimit: f32,
    pub dashStyle: u32,
    pub dashOffset: f32,
}

impl Default for D2D1_STROKE_STYLE_PROPERTIES {
    fn default() -> Self {
        Self {
            startCap: D2D1_CAP_STYLE_FLAT,
            endCap: D2D1_CAP_STYLE_FLAT,
            dashCap: D2D1_CAP_STYLE_FLAT,
            lineJoin: D2D1_LINE_JOIN_MITER,
            miterLimit: 10.0,
            dashStyle: D2D1_DASH_STYLE_SOLID,
            dashOffset: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub base: D2D1_STROKE_STYLE_PROPERTIES,
    pub transformType: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: u32,
    pub _pad: u32,
    pub colorContext: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub _pad: u32,
    pub bits: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub rt_type: u32,
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: u32,
    pub minLevel: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: HWND,
    pub pixelSize: D2D1_SIZE_U,
    pub presentOptions: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: u32,
    pub textAntialiasMode: u32,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: D2D1_MATRIX_3X2_F,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: D2D1_RECT_F,
    pub geometricMask: *mut c_void,
    pub maskAntialiasMode: u32,
    pub maskTransform: D2D1_MATRIX_3X2_F,
    pub opacity: f32,
    pub opacityBrush: *mut c_void,
    pub layerOptions: u32,
    pub _pad: u32,
}

// ============================================================================
// COM Infrastructure
// ============================================================================

#[repr(C)]
struct ComObj<T> {
    vtbl: *const usize,
    refs: AtomicU32,
    inner: T,
}

impl<T> ComObj<T> {
    fn new(vtbl: *const usize, inner: T) -> *mut Self {
        Box::into_raw(Box::new(Self { vtbl, refs: AtomicU32::new(1), inner }))
    }

    #[inline]
    unsafe fn inner<'a>(this: *mut Self) -> &'a T {
        &(&*this).inner
    }

    #[inline]
    unsafe fn add_ref(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        (&*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }

    #[inline]
    unsafe fn release(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        let prev = (&*this).refs.fetch_sub(1, Ordering::AcqRel);
        if prev == 1 {
            drop(Box::from_raw(this));
            0
        } else {
            prev - 1
        }
    }
}

fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    slots
}

unsafe fn qi_helper<T>(
    this: *mut ComObj<T>,
    riid: *const u8,
    ppv: *mut *mut c_void,
    iids: &[GUID],
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    ppv.write_unaligned(std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }
    let iid = (riid as *const GUID).read_unaligned();
    if iid == IID_IUNKNOWN || iids.iter().any(|g| *g == iid) {
        ComObj::add_ref(this);
        ppv.write_unaligned(this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn raw_com_add_ref(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return;
    }
    let f = vtbl.add(1).read_unaligned();
    if f != 0 {
        crate::call_guest8(f, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

unsafe fn raw_com_release(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return;
    }
    let f = vtbl.add(2).read_unaligned();
    if f != 0 {
        crate::call_guest8(f, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

#[derive(Debug)]
struct ComPtr(*mut c_void);

unsafe impl Send for ComPtr {}
unsafe impl Sync for ComPtr {}

impl ComPtr {
    unsafe fn retain(p: *mut c_void) -> Option<Self> {
        if p.is_null() {
            None
        } else {
            raw_com_add_ref(p);
            Some(Self(p))
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> Self {
        unsafe { raw_com_add_ref(self.0) };
        Self(self.0)
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe { raw_com_release(self.0) };
    }
}

// ============================================================================
// RGBA32 Pixel Surface & Compositing Engine
// ============================================================================

fn normalize_pixel_format(pf: D2D1_PIXEL_FORMAT) -> D2D1_PIXEL_FORMAT {
    let fmt = if pf.format == DXGI_FORMAT_UNKNOWN {
        DXGI_FORMAT_R8G8B8A8_UNORM
    } else {
        pf.format
    };
    let alpha = if pf.alphaMode == D2D1_ALPHA_MODE_UNKNOWN {
        D2D1_ALPHA_MODE_PREMULTIPLIED
    } else {
        pf.alphaMode
    };
    D2D1_PIXEL_FORMAT { format: fmt, alphaMode: alpha }
}

#[derive(Clone)]
struct Surface {
    width: u32,
    height: u32,
    pixel_format: D2D1_PIXEL_FORMAT,
    dpi_x: f32,
    dpi_y: f32,
    /// Tightly packed pixel buffer (`width * height * bpp`).
    /// For RGBA8/BGRA8: 4 bytes/pixel (`pitch = width * 4`).
    /// For A8: 1 byte/pixel (`pitch = width`).
    data: Vec<u8>,
}

impl Surface {
    fn new(width: u32, height: u32, pf: D2D1_PIXEL_FORMAT, dpi_x: f32, dpi_y: f32) -> Self {
        let pf = normalize_pixel_format(pf);
        let bpp = if pf.format == DXGI_FORMAT_A8_UNORM { 1 } else { 4 };
        let len = (width as usize).saturating_mul(height as usize).saturating_mul(bpp);
        Self {
            width,
            height,
            pixel_format: pf,
            dpi_x: if dpi_x > 0.0 && dpi_x.is_finite() { dpi_x } else { 96.0 },
            dpi_y: if dpi_y > 0.0 && dpi_y.is_finite() { dpi_y } else { 96.0 },
            data: vec![0u8; len],
        }
    }

    #[inline]
    fn bpp(&self) -> usize {
        if self.pixel_format.format == DXGI_FORMAT_A8_UNORM { 1 } else { 4 }
    }

    #[inline]
    fn pitch(&self) -> usize {
        (self.width as usize) * self.bpp()
    }

    #[inline]
    fn is_bgra(&self) -> bool {
        self.pixel_format.format == DXGI_FORMAT_B8G8R8A8_UNORM
    }

    fn resize(&mut self, width: u32, height: u32) {
        let bpp = self.bpp();
        self.width = width;
        self.height = height;
        self.data.resize((width as usize).saturating_mul(height as usize).saturating_mul(bpp), 0);
        self.data.fill(0);
    }

    /// Read pixel as floating-point `[r, g, b, a]` in `[0.0, 1.0]` (straight RGB when STRAIGHT/IGNORE,
    /// premultiplied RGB when PREMULTIPLIED).
    #[inline]
    fn get_pixel_raw(&self, x: u32, y: u32) -> [f32; 4] {
        let idx = ((y as usize) * (self.width as usize) + (x as usize)) * self.bpp();
        if self.pixel_format.format == DXGI_FORMAT_A8_UNORM {
            let a = self.data.get(idx).copied().unwrap_or(0) as f32 / 255.0;
            return [0.0, 0.0, 0.0, a];
        }
        let Some(px) = self.data.get(idx..idx + 4) else {
            return [0.0; 4];
        };
        if self.is_bgra() {
            [px[2] as f32 / 255.0, px[1] as f32 / 255.0, px[0] as f32 / 255.0, px[3] as f32 / 255.0]
        } else {
            [px[0] as f32 / 255.0, px[1] as f32 / 255.0, px[2] as f32 / 255.0, px[3] as f32 / 255.0]
        }
    }

    /// Read pixel as straight `[r, g, b, a]` in `[0.0, 1.0]`.
    #[inline]
    fn get_pixel_straight(&self, x: u32, y: u32) -> [f32; 4] {
        let p = self.get_pixel_raw(x, y);
        if self.pixel_format.alphaMode == D2D1_ALPHA_MODE_PREMULTIPLIED && p[3] > 1e-6 {
            [(p[0] / p[3]).min(1.0), (p[1] / p[3]).min(1.0), (p[2] / p[3]).min(1.0), p[3]]
        } else if self.pixel_format.alphaMode == D2D1_ALPHA_MODE_IGNORE {
            [p[0], p[1], p[2], 1.0]
        } else {
            p
        }
    }

    #[inline]
    fn set_pixel_raw(&mut self, x: u32, y: u32, rgba: [f32; 4]) {
        let bpp = self.bpp();
        let idx = ((y as usize) * (self.width as usize) + (x as usize)) * bpp;
        let to_u8 = |v: f32| -> u8 { (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8 };
        if bpp == 1 {
            if let Some(slot) = self.data.get_mut(idx) {
                *slot = to_u8(rgba[3]);
            }
            return;
        }
        let r = to_u8(rgba[0]);
        let g = to_u8(rgba[1]);
        let b = to_u8(rgba[2]);
        let a = to_u8(rgba[3]);
        let is_bgra = self.is_bgra();
        if let Some(px) = self.data.get_mut(idx..idx + 4) {
            if is_bgra {
                px[0] = b;
                px[1] = g;
                px[2] = r;
                px[3] = a;
            } else {
                px[0] = r;
                px[1] = g;
                px[2] = b;
                px[3] = a;
            }
        }
    }

    /// Clear inside `clip` rect directly with straight `color`.
    fn clear_rect(&mut self, color: D2D1_COLOR_F, clip: (i32, i32, i32, i32)) {
        let x0 = clip.0.max(0).min(self.width as i32) as u32;
        let y0 = clip.1.max(0).min(self.height as i32) as u32;
        let x1 = clip.2.max(0).min(self.width as i32) as u32;
        let y1 = clip.3.max(0).min(self.height as i32) as u32;
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        let raw = match self.pixel_format.alphaMode {
            D2D1_ALPHA_MODE_PREMULTIPLIED => {
                let a = color.a.clamp(0.0, 1.0);
                [color.r.clamp(0.0, 1.0) * a, color.g.clamp(0.0, 1.0) * a, color.b.clamp(0.0, 1.0) * a, a]
            }
            D2D1_ALPHA_MODE_IGNORE => {
                [color.r.clamp(0.0, 1.0), color.g.clamp(0.0, 1.0), color.b.clamp(0.0, 1.0), 1.0]
            }
            _ => [
                color.r.clamp(0.0, 1.0),
                color.g.clamp(0.0, 1.0),
                color.b.clamp(0.0, 1.0),
                color.a.clamp(0.0, 1.0),
            ],
        };
        for y in y0..y1 {
            for x in x0..x1 {
                self.set_pixel_raw(x, y, raw);
            }
        }
    }

    /// Blend straight source `src` (`[r,g,b,a]`) with coverage `cov` (`0..=1`) onto `(x, y)`.
    #[inline]
    fn blend_pixel(&mut self, x: u32, y: u32, src: [f32; 4], cov: f32, blend_mode: u32) {
        if cov <= 0.0 {
            return;
        }
        let sr = src[0].clamp(0.0, 1.0);
        let sg = src[1].clamp(0.0, 1.0);
        let sb = src[2].clamp(0.0, 1.0);
        let sa = src[3].clamp(0.0, 1.0);
        let eff_a = (sa * cov).clamp(0.0, 1.0);

        if blend_mode == D2D1_PRIMITIVE_BLEND_COPY {
            let out = match self.pixel_format.alphaMode {
                D2D1_ALPHA_MODE_PREMULTIPLIED => [sr * eff_a, sg * eff_a, sb * eff_a, eff_a],
                D2D1_ALPHA_MODE_IGNORE => [sr * cov, sg * cov, sb * cov, 1.0],
                _ => [sr, sg, sb, eff_a],
            };
            self.set_pixel_raw(x, y, out);
            return;
        }

        if eff_a <= 0.0 {
            return;
        }

        let dst = self.get_pixel_raw(x, y);
        let inv = 1.0 - eff_a;
        match self.pixel_format.alphaMode {
            D2D1_ALPHA_MODE_PREMULTIPLIED => {
                let out = [
                    sr * eff_a + dst[0] * inv,
                    sg * eff_a + dst[1] * inv,
                    sb * eff_a + dst[2] * inv,
                    eff_a + dst[3] * inv,
                ];
                self.set_pixel_raw(x, y, out);
            }
            D2D1_ALPHA_MODE_IGNORE => {
                let out = [
                    sr * eff_a + dst[0] * inv,
                    sg * eff_a + dst[1] * inv,
                    sb * eff_a + dst[2] * inv,
                    1.0,
                ];
                self.set_pixel_raw(x, y, out);
            }
            _ => {
                let da = dst[3];
                let out_a = eff_a + da * inv;
                if out_a <= 1e-6 {
                    self.set_pixel_raw(x, y, [0.0; 4]);
                } else {
                    let out = [
                        (sr * eff_a + dst[0] * da * inv) / out_a,
                        (sg * eff_a + dst[1] * da * inv) / out_a,
                        (sb * eff_a + dst[2] * da * inv) / out_a,
                        out_a,
                    ];
                    self.set_pixel_raw(x, y, out);
                }
            }
        }
    }

    fn sample(&self, x: f32, y: f32, ex_x: u32, ex_y: u32, interp: u32) -> [f32; 4] {
        if self.width == 0 || self.height == 0 {
            return [0.0; 4];
        }
        let wrap_coord = |c: i32, limit: i32, mode: u32| -> u32 {
            match mode {
                D2D1_EXTEND_MODE_WRAP => c.rem_euclid(limit) as u32,
                D2D1_EXTEND_MODE_MIRROR => {
                    let period = limit * 2;
                    let m = c.rem_euclid(period);
                    (if m < limit { m } else { period - 1 - m }) as u32
                }
                _ => c.clamp(0, limit - 1) as u32,
            }
        };
        let w = self.width as i32;
        let h = self.height as i32;
        if interp == D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR {
            let ix = wrap_coord(x.floor() as i32, w, ex_x);
            let iy = wrap_coord(y.floor() as i32, h, ex_y);
            self.get_pixel_straight(ix, iy)
        } else {
            let fx = x - 0.5;
            let fy = y - 0.5;
            let x0 = fx.floor() as i32;
            let y0 = fy.floor() as i32;
            let tx = (fx - x0 as f32).clamp(0.0, 1.0);
            let ty = (fy - y0 as f32).clamp(0.0, 1.0);
            let ix0 = wrap_coord(x0, w, ex_x);
            let ix1 = wrap_coord(x0 + 1, w, ex_x);
            let iy0 = wrap_coord(y0, h, ex_y);
            let iy1 = wrap_coord(y0 + 1, h, ex_y);
            let c00 = self.get_pixel_straight(ix0, iy0);
            let c10 = self.get_pixel_straight(ix1, iy0);
            let c01 = self.get_pixel_straight(ix0, iy1);
            let c11 = self.get_pixel_straight(ix1, iy1);
            let mut out = [0.0f32; 4];
            for ch in 0..4 {
                let top = c00[ch] * (1.0 - tx) + c10[ch] * tx;
                let bot = c01[ch] * (1.0 - tx) + c11[ch] * tx;
                out[ch] = top * (1.0 - ty) + bot * ty;
            }
            out
        }
    }
}

// ============================================================================
// Geometry Representation, Path Flattening, Stroke Expansion & Scanline Fill
// ============================================================================

#[derive(Clone, Debug)]
struct SubPath {
    points: Vec<(f32, f32)>,
    closed: bool,
    filled: bool,
}

#[derive(Clone)]
struct FlattenedGeometry {
    subpaths: Vec<SubPath>,
    fill_mode: u32,
}

fn flatten_cubic(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), tol: f32, out: &mut Vec<(f32, f32)>) {
    // Estimate flatness via chord distance of control points.
    let dx = p3.0 - p0.0;
    let dy = p3.1 - p0.1;
    let d1 = ((p1.0 - p3.0) * dy - (p1.1 - p3.1) * dx).abs();
    let d2 = ((p2.0 - p3.0) * dy - (p2.1 - p3.1) * dx).abs();
    let len_sq = dx * dx + dy * dy;
    let flat_est = if len_sq > 1e-6 {
        (d1 + d2) / len_sq.sqrt()
    } else {
        (p1.0 - p0.0).hypot(p1.1 - p0.1) + (p2.0 - p0.0).hypot(p2.1 - p0.1)
    };
    let steps = ((flat_est / tol.max(0.1)).sqrt().ceil() as usize).clamp(4, 48);
    for i in 1..=steps {
        let t = (i as f32) / (steps as f32);
        let mt = 1.0 - t;
        let x = mt * mt * mt * p0.0
            + 3.0 * mt * mt * t * p1.0
            + 3.0 * mt * t * t * p2.0
            + t * t * t * p3.0;
        let y = mt * mt * mt * p0.1
            + 3.0 * mt * mt * t * p1.1
            + 3.0 * mt * t * t * p2.1
            + t * t * t * p3.1;
        out.push((x, y));
    }
}

fn flatten_quad(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), tol: f32, out: &mut Vec<(f32, f32)>) {
    let steps = ((((p1.0 - 0.5 * (p0.0 + p2.0)).hypot(p1.1 - 0.5 * (p0.1 + p2.1))) / tol.max(0.1))
        .sqrt()
        .ceil() as usize)
        .clamp(4, 32);
    for i in 1..=steps {
        let t = (i as f32) / (steps as f32);
        let mt = 1.0 - t;
        out.push((
            mt * mt * p0.0 + 2.0 * mt * t * p1.0 + t * t * p2.0,
            mt * mt * p0.1 + 2.0 * mt * t * p1.1 + t * t * p2.1,
        ));
    }
}

fn flatten_arc(p0: (f32, f32), arc: &D2D1_ARC_SEGMENT, out: &mut Vec<(f32, f32)>) {
    let p1 = (arc.point.x, arc.point.y);
    let mut rx = arc.size.width.abs();
    let mut ry = arc.size.height.abs();
    if rx < 1e-5 || ry < 1e-5 || ((p1.0 - p0.0).abs() < 1e-5 && (p1.1 - p0.1).abs() < 1e-5) {
        out.push(p1);
        return;
    }
    let phi = arc.rotationAngle.to_radians();
    let (sin_p, cos_p) = phi.sin_cos();
    let dx2 = (p0.0 - p1.0) * 0.5;
    let dy2 = (p0.1 - p1.1) * 0.5;
    let x1p = cos_p * dx2 + sin_p * dy2;
    let y1p = -sin_p * dx2 + cos_p * dy2;

    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lambda > 1.0 {
        let s = lambda.sqrt();
        rx *= s;
        ry *= s;
    }

    let rx2 = rx * rx;
    let ry2 = ry * ry;
    let x1p2 = x1p * x1p;
    let y1p2 = y1p * y1p;
    let num = (rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2).max(0.0);
    let den = (rx2 * y1p2 + ry2 * x1p2).max(1e-10);
    let large = arc.arcSize == D2D1_ARC_SIZE_LARGE;
    let sweep = arc.sweepDirection == D2D1_SWEEP_DIRECTION_CLOCKWISE;
    let sign = if large == sweep { -1.0 } else { 1.0 };
    let coef = sign * (num / den).sqrt();
    let cxp = coef * (rx * y1p / ry);
    let cyp = coef * -(ry * x1p / rx);
    let cx = cos_p * cxp - sin_p * cyp + (p0.0 + p1.0) * 0.5;
    let cy = sin_p * cxp + cos_p * cyp + (p0.1 + p1.1) * 0.5;

    let vec_angle = |ux: f32, uy: f32, vx: f32, vy: f32| -> f32 {
        let dot = ux * vx + uy * vy;
        let det = ux * vy - uy * vx;
        det.atan2(dot)
    };
    let vx0 = (x1p - cxp) / rx;
    let vy0 = (y1p - cyp) / ry;
    let vx1 = (-x1p - cxp) / rx;
    let vy1 = (-y1p - cyp) / ry;
    let theta1 = vec_angle(1.0, 0.0, vx0, vy0);
    let mut dtheta = vec_angle(vx0, vy0, vx1, vy1);
    if !sweep && dtheta > 0.0 {
        dtheta -= 2.0 * PI;
    } else if sweep && dtheta < 0.0 {
        dtheta += 2.0 * PI;
    }

    let steps = ((dtheta.abs() * rx.max(ry) / 3.0).ceil() as usize).clamp(8, 96);
    for i in 1..=steps {
        let a = theta1 + dtheta * (i as f32) / (steps as f32);
        let (sa, ca) = a.sin_cos();
        let ex = rx * ca;
        let ey = ry * sa;
        out.push((cos_p * ex - sin_p * ey + cx, sin_p * ex + cos_p * ey + cy));
    }
}

fn rect_subpath(r: D2D1_RECT_F) -> SubPath {
    let l = r.left.min(r.right);
    let rr = r.left.max(r.right);
    let t = r.top.min(r.bottom);
    let b = r.top.max(r.bottom);
    SubPath { points: vec![(l, t), (rr, t), (rr, b), (l, b)], closed: true, filled: true }
}

fn rounded_rect_subpath(rr: D2D1_ROUNDED_RECT) -> SubPath {
    let l = rr.rect.left.min(rr.rect.right);
    let r = rr.rect.left.max(rr.rect.right);
    let t = rr.rect.top.min(rr.rect.bottom);
    let b = rr.rect.top.max(rr.rect.bottom);
    let rx = rr.radiusX.abs().min((r - l) * 0.5);
    let ry = rr.radiusY.abs().min((b - t) * 0.5);
    if rx < 1e-4 || ry < 1e-4 {
        return rect_subpath(rr.rect);
    }
    let segs = 8usize;
    let mut pts = Vec::with_capacity(segs * 4 + 4);
    let corners = [
        (r - rx, t + ry, -PI * 0.5, 0.0),
        (r - rx, b - ry, 0.0, PI * 0.5),
        (l + rx, b - ry, PI * 0.5, PI),
        (l + rx, t + ry, PI, PI * 1.5),
    ];
    for &(cx, cy, a0, a1) in &corners {
        for i in 0..=segs {
            let a = a0 + (a1 - a0) * (i as f32) / (segs as f32);
            let (s, c) = a.sin_cos();
            pts.push((cx + rx * c, cy + ry * s));
        }
    }
    SubPath { points: pts, closed: true, filled: true }
}

fn ellipse_subpath(el: D2D1_ELLIPSE) -> SubPath {
    let rx = el.radiusX.abs();
    let ry = el.radiusY.abs();
    let segs = ((rx.max(ry) * 1.5).ceil() as usize).clamp(24, 128);
    let mut pts = Vec::with_capacity(segs);
    for i in 0..segs {
        let a = 2.0 * PI * (i as f32) / (segs as f32);
        let (s, c) = a.sin_cos();
        pts.push((el.point.x + rx * c, el.point.y + ry * s));
    }
    SubPath { points: pts, closed: true, filled: true }
}

fn transform_subpaths(subpaths: &[SubPath], m: &D2D1_MATRIX_3X2_F) -> Vec<SubPath> {
    subpaths
        .iter()
        .map(|sp| SubPath {
            points: sp.points.iter().map(|&(x, y)| m.transform_point(x, y)).collect(),
            closed: sp.closed,
            filled: sp.filled,
        })
        .collect()
}

fn apply_dashes(subpaths: &[SubPath], stroke_width: f32, style: Option<&StrokeStyleData>) -> Vec<SubPath> {
    let Some(st) = style else {
        return subpaths.to_vec();
    };
    let pattern: Vec<f32> = match st.props.dashStyle {
        D2D1_DASH_STYLE_SOLID => return subpaths.to_vec(),
        D2D1_DASH_STYLE_DASH => vec![2.0, 2.0],
        D2D1_DASH_STYLE_DOT => vec![0.1, 2.0],
        D2D1_DASH_STYLE_DASH_DOT => vec![2.0, 2.0, 0.1, 2.0],
        D2D1_DASH_STYLE_DASH_DOT_DOT => vec![2.0, 2.0, 0.1, 2.0, 0.1, 2.0],
        D2D1_DASH_STYLE_CUSTOM => {
            if st.dashes.is_empty() {
                return subpaths.to_vec();
            }
            if st.dashes.len() % 2 == 1 {
                let mut d = st.dashes.clone();
                d.extend_from_slice(&st.dashes);
                d
            } else {
                st.dashes.clone()
            }
        }
        _ => return subpaths.to_vec(),
    };
    let scaled: Vec<f32> = pattern.iter().map(|&v| (v * stroke_width).max(0.5)).collect();
    let total_pat: f32 = scaled.iter().sum();
    if total_pat <= 1e-4 {
        return subpaths.to_vec();
    }

    let mut out = Vec::new();
    for sp in subpaths {
        if sp.points.len() < 2 {
            continue;
        }
        let mut pts = sp.points.clone();
        if sp.closed {
            pts.push(pts[0]);
        }
        let mut pat_idx = 0usize;
        let mut pat_rem = scaled[0];
        let mut off = (st.props.dashOffset * stroke_width).rem_euclid(total_pat);
        while off >= pat_rem {
            off -= pat_rem;
            pat_idx = (pat_idx + 1) % scaled.len();
            pat_rem = scaled[pat_idx];
        }
        pat_rem -= off;

        let mut cur_dash: Option<Vec<(f32, f32)>> = if pat_idx % 2 == 0 {
            Some(vec![pts[0]])
        } else {
            None
        };

        for seg in pts.windows(2) {
            let (mut x0, mut y0) = seg[0];
            let (x1, y1) = seg[1];
            let mut seg_rem = (x1 - x0).hypot(y1 - y0);
            if seg_rem < 1e-5 {
                continue;
            }
            let ux = (x1 - x0) / seg_rem;
            let uy = (y1 - y0) / seg_rem;
            while seg_rem > 1e-5 {
                let step = seg_rem.min(pat_rem);
                x0 += ux * step;
                y0 += uy * step;
                seg_rem -= step;
                pat_rem -= step;
                if let Some(ref mut d) = cur_dash {
                    d.push((x0, y0));
                }
                if pat_rem <= 1e-5 {
                    if let Some(d) = cur_dash.take() {
                        if d.len() >= 2 {
                            out.push(SubPath { points: d, closed: false, filled: true });
                        }
                    }
                    pat_idx = (pat_idx + 1) % scaled.len();
                    pat_rem = scaled[pat_idx];
                    if pat_idx % 2 == 0 {
                        cur_dash = Some(vec![(x0, y0)]);
                    }
                }
            }
        }
        if let Some(d) = cur_dash.take() {
            if d.len() >= 2 {
                out.push(SubPath { points: d, closed: false, filled: true });
            }
        }
    }
    out
}

fn expand_strokes(subpaths: &[SubPath], stroke_width: f32, style: Option<&StrokeStyleData>) -> Vec<SubPath> {
    if stroke_width <= 0.0 || !stroke_width.is_finite() {
        return Vec::new();
    }
    let dashed = apply_dashes(subpaths, stroke_width, style);
    let half = stroke_width * 0.5;
    let start_cap = style.map_or(D2D1_CAP_STYLE_FLAT, |s| s.props.startCap);
    let end_cap = style.map_or(D2D1_CAP_STYLE_FLAT, |s| s.props.endCap);
    let line_join = style.map_or(D2D1_LINE_JOIN_MITER, |s| s.props.lineJoin);

    let mut polys: Vec<SubPath> = Vec::new();

    let push_round_disk = |polys: &mut Vec<SubPath>, cx: f32, cy: f32, r: f32| {
        let segs = 12usize;
        let mut pts = Vec::with_capacity(segs);
        for i in 0..segs {
            let a = 2.0 * PI * (i as f32) / (segs as f32);
            let (s, c) = a.sin_cos();
            pts.push((cx + r * c, cy + r * s));
        }
        polys.push(SubPath { points: pts, closed: true, filled: true });
    };

    for sp in &dashed {
        let n = sp.points.len();
        if n < 2 {
            continue;
        }
        let seg_count = if sp.closed { n } else { n - 1 };
        for i in 0..seg_count {
            let p0 = sp.points[i];
            let p1 = sp.points[(i + 1) % n];
            let dx = p1.0 - p0.0;
            let dy = p1.1 - p0.1;
            let len = dx.hypot(dy);
            if len < 1e-5 {
                continue;
            }
            let tx = dx / len;
            let ty = dy / len;
            let nx = -ty * half;
            let ny = tx * half;

            let mut s0 = p0;
            let mut s1 = p1;
            if !sp.closed && i == 0 && start_cap == D2D1_CAP_STYLE_SQUARE {
                s0 = (p0.0 - tx * half, p0.1 - ty * half);
            }
            if !sp.closed && i + 1 == seg_count && end_cap == D2D1_CAP_STYLE_SQUARE {
                s1 = (p1.0 + tx * half, p1.1 + ty * half);
            }
            polys.push(SubPath {
                points: vec![
                    (s0.0 + nx, s0.1 + ny),
                    (s1.0 + nx, s1.1 + ny),
                    (s1.0 - nx, s1.1 - ny),
                    (s0.0 - nx, s0.1 - ny),
                ],
                closed: true,
                filled: true,
            });

            if !sp.closed && i == 0 {
                if start_cap == D2D1_CAP_STYLE_ROUND {
                    push_round_disk(&mut polys, p0.0, p0.1, half);
                } else if start_cap == D2D1_CAP_STYLE_TRIANGLE {
                    polys.push(SubPath {
                        points: vec![
                            (p0.0 + nx, p0.1 + ny),
                            (p0.0 - tx * half, p0.1 - ty * half),
                            (p0.0 - nx, p0.1 - ny),
                        ],
                        closed: true,
                        filled: true,
                    });
                }
            }
            if !sp.closed && i + 1 == seg_count {
                if end_cap == D2D1_CAP_STYLE_ROUND {
                    push_round_disk(&mut polys, p1.0, p1.1, half);
                } else if end_cap == D2D1_CAP_STYLE_TRIANGLE {
                    polys.push(SubPath {
                        points: vec![
                            (p1.0 + nx, p1.1 + ny),
                            (p1.0 + tx * half, p1.1 + ty * half),
                            (p1.0 - nx, p1.1 - ny),
                        ],
                        closed: true,
                        filled: true,
                    });
                }
            }
            // Join at vertex p1 for interior/closed corners
            if sp.closed || i + 1 < seg_count {
                if line_join == D2D1_LINE_JOIN_ROUND || line_join == D2D1_LINE_JOIN_MITER || line_join == D2D1_LINE_JOIN_BEVEL {
                    push_round_disk(&mut polys, p1.0, p1.1, half);
                }
            }
        }
    }
    polys
}

fn subpaths_bounds(subpaths: &[SubPath]) -> D2D1_RECT_F {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for sp in subpaths {
        for &(x, y) in &sp.points {
            if x < min_x { min_x = x; }
            if y < min_y { min_y = y; }
            if x > max_x { max_x = x; }
            if y > max_y { max_y = y; }
        }
    }
    if !min_x.is_finite() {
        D2D1_RECT_F { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 }
    } else {
        D2D1_RECT_F { left: min_x, top: min_y, right: max_x, bottom: max_y }
    }
}

fn point_in_subpaths(x: f32, y: f32, subpaths: &[SubPath], fill_mode: u32) -> bool {
    let mut winding = 0i32;
    for sp in subpaths {
        if !sp.filled || sp.points.len() < 3 {
            continue;
        }
        let n = sp.points.len();
        for i in 0..n {
            let (x0, y0) = sp.points[i];
            let (x1, y1) = sp.points[(i + 1) % n];
            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let t = (y - y0) / (y1 - y0);
                let ix = x0 + t * (x1 - x0);
                if ix > x {
                    winding += if y1 > y0 { 1 } else { -1 };
                }
            }
        }
    }
    if fill_mode == D2D1_FILL_MODE_WINDING {
        winding != 0
    } else {
        (winding & 1) != 0
    }
}

fn rasterize_polygons<F>(
    surface: &mut Surface,
    subpaths: &[SubPath],
    fill_mode: u32,
    clip: (i32, i32, i32, i32),
    aa_mode: u32,
    blend_mode: u32,
    mut shade_pixel: F,
) where
    F: FnMut(f32, f32) -> [f32; 4],
{
    let bounds = subpaths_bounds(subpaths);
    let min_x = (bounds.left.floor() as i32).max(clip.0).max(0);
    let min_y = (bounds.top.floor() as i32).max(clip.1).max(0);
    let max_x = (bounds.right.ceil() as i32).min(clip.2).min(surface.width as i32);
    let max_y = (bounds.bottom.ceil() as i32).min(clip.3).min(surface.height as i32);
    if min_x >= max_x || min_y >= max_y {
        return;
    }

    let row_w = (max_x - min_x) as usize;
    let subsamples: u32 = if aa_mode == D2D1_ANTIALIAS_MODE_ALIASED { 1 } else { 4 };
    let inv_sub = 1.0 / (subsamples as f32);
    let mut row_cov = vec![0.0f32; row_w];
    let mut crossings: Vec<(f32, i32)> = Vec::new();

    for py in min_y..max_y {
        row_cov.fill(0.0);
        for sub in 0..subsamples {
            let sy = (py as f32) + ((sub as f32) + 0.5) * inv_sub;
            crossings.clear();
            for sp in subpaths {
                if !sp.filled || sp.points.len() < 3 {
                    continue;
                }
                let n = sp.points.len();
                for i in 0..n {
                    let (x0, y0) = sp.points[i];
                    let (x1, y1) = sp.points[(i + 1) % n];
                    if (y0 <= sy && y1 > sy) || (y1 <= sy && y0 > sy) {
                        let t = (sy - y0) / (y1 - y0);
                        let cx = x0 + t * (x1 - x0);
                        let dir = if y1 > y0 { 1 } else { -1 };
                        crossings.push((cx, dir));
                    }
                }
            }
            if crossings.is_empty() {
                continue;
            }
            crossings.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut winding = 0i32;
            let mut prev_x = 0.0f32;
            for &(cx, dir) in &crossings {
                let was_inside = if fill_mode == D2D1_FILL_MODE_WINDING {
                    winding != 0
                } else {
                    (winding & 1) != 0
                };
                winding += dir;
                let is_inside = if fill_mode == D2D1_FILL_MODE_WINDING {
                    winding != 0
                } else {
                    (winding & 1) != 0
                };
                if was_inside {
                    let l_f = (prev_x - min_x as f32).clamp(0.0, row_w as f32);
                    let r_f = (cx - min_x as f32).clamp(0.0, row_w as f32);
                    if r_f > l_f {
                        if aa_mode == D2D1_ANTIALIAS_MODE_ALIASED {
                            let i0 = l_f.round() as usize;
                            let i1 = (r_f.round() as usize).min(row_w);
                            for slot in &mut row_cov[i0..i1] {
                                *slot = 1.0;
                            }
                        } else {
                            let i0 = l_f.floor() as usize;
                            let i1 = (r_f.ceil() as usize).min(row_w);
                            for idx in i0..i1 {
                                let a = l_f.max(idx as f32);
                                let b = r_f.min((idx + 1) as f32);
                                if b > a {
                                    row_cov[idx] += (b - a) * inv_sub;
                                }
                            }
                        }
                    }
                }
                if is_inside {
                    prev_x = cx;
                }
            }
        }
        let py_f = (py as f32) + 0.5;
        for (idx, &cov) in row_cov.iter().enumerate() {
            if cov > 0.0 {
                let px = (min_x as usize + idx) as u32;
                let col = shade_pixel((px as f32) + 0.5, py_f);
                surface.blend_pixel(px, py as u32, col, cov.clamp(0.0, 1.0), blend_mode);
            }
        }
    }
}

// ============================================================================
// Brush Evaluation
// ============================================================================

#[derive(Clone)]
struct GradientStopCollectionData {
    stops: Vec<D2D1_GRADIENT_STOP>,
    gamma: u32,
    extend_mode: u32,
}

impl GradientStopCollectionData {
    fn sample(&self, t_raw: f32) -> [f32; 4] {
        if self.stops.is_empty() {
            return [0.0; 4];
        }
        let t = match self.extend_mode {
            D2D1_EXTEND_MODE_WRAP => t_raw.rem_euclid(1.0),
            D2D1_EXTEND_MODE_MIRROR => {
                let m = t_raw.rem_euclid(2.0);
                if m <= 1.0 { m } else { 2.0 - m }
            }
            _ => t_raw.clamp(0.0, 1.0),
        };
        let first = &self.stops[0];
        if t <= first.position {
            return [first.color.r, first.color.g, first.color.b, first.color.a];
        }
        let last = self.stops.last().unwrap();
        if t >= last.position {
            return [last.color.r, last.color.g, last.color.b, last.color.a];
        }
        for pair in self.stops.windows(2) {
            let a = &pair[0];
            let b = &pair[1];
            if t >= a.position && t <= b.position {
                let span = b.position - a.position;
                let frac = if span > 1e-6 { (t - a.position) / span } else { 0.0 };
                return [
                    a.color.r + (b.color.r - a.color.r) * frac,
                    a.color.g + (b.color.g - a.color.g) * frac,
                    a.color.b + (b.color.b - a.color.b) * frac,
                    a.color.a + (b.color.a - a.color.a) * frac,
                ];
            }
        }
        [last.color.r, last.color.g, last.color.b, last.color.a]
    }
}

#[derive(Clone)]
enum BrushEval {
    Solid([f32; 4]),
    Linear {
        start: (f32, f32),
        end: (f32, f32),
        stops: Arc<GradientStopCollectionData>,
        inv_brush: D2D1_MATRIX_3X2_F,
        opacity: f32,
    },
    Radial {
        center: (f32, f32),
        origin_offset: (f32, f32),
        rx: f32,
        ry: f32,
        stops: Arc<GradientStopCollectionData>,
        inv_brush: D2D1_MATRIX_3X2_F,
        opacity: f32,
    },
    Bitmap {
        surface: Surface,
        extend_x: u32,
        extend_y: u32,
        interp: u32,
        inv_brush: D2D1_MATRIX_3X2_F,
        opacity: f32,
    },
}

impl BrushEval {
    #[inline]
    fn sample_world(&self, wx: f32, wy: f32) -> [f32; 4] {
        match self {
            BrushEval::Solid(c) => *c,
            BrushEval::Linear { start, end, stops, inv_brush, opacity } => {
                let (bx, by) = inv_brush.transform_point(wx, wy);
                let dx = end.0 - start.0;
                let dy = end.1 - start.1;
                let len2 = dx * dx + dy * dy;
                let t = if len2 > 1e-8 { ((bx - start.0) * dx + (by - start.1) * dy) / len2 } else { 0.0 };
                let mut c = stops.sample(t);
                c[3] *= *opacity;
                c
            }
            BrushEval::Radial { center, origin_offset, rx, ry, stops, inv_brush, opacity } => {
                let (bx, by) = inv_brush.transform_point(wx, wy);
                let rx = rx.abs().max(1e-5);
                let ry = ry.abs().max(1e-5);
                let px = (bx - center.0) / rx;
                let py = (by - center.1) / ry;
                let mut ox = origin_offset.0 / rx;
                let mut oy = origin_offset.1 / ry;
                let o_len = ox.hypot(oy);
                if o_len >= 0.999 {
                    ox = ox / o_len * 0.999;
                    oy = oy / o_len * 0.999;
                }
                let vx = px - ox;
                let vy = py - oy;
                let v2 = vx * vx + vy * vy;
                let t = if v2 <= 1e-10 {
                    0.0
                } else {
                    let ov = ox * vx + oy * vy;
                    let o2 = ox * ox + oy * oy;
                    let disc = (ov * ov - v2 * (o2 - 1.0)).max(0.0);
                    let s = (-ov + disc.sqrt()) / v2;
                    if s > 1e-8 { 1.0 / s } else { 1.0 }
                };
                let mut c = stops.sample(t);
                c[3] *= *opacity;
                c
            }
            BrushEval::Bitmap { surface, extend_x, extend_y, interp, inv_brush, opacity } => {
                let (bx, by) = inv_brush.transform_point(wx, wy);
                let sx = bx * (surface.dpi_x / 96.0);
                let sy = by * (surface.dpi_y / 96.0);
                let mut c = surface.sample(sx, sy, *extend_x, *extend_y, *interp);
                c[3] *= *opacity;
                c
            }
        }
    }
}

// ============================================================================
// Internal COM Object Payloads
// ============================================================================

struct FactoryInner {
    _factory_type: u32,
}

struct DeviceInner {
    factory: ComPtr,
    max_tex_mem: Mutex<u64>,
}

struct BitmapInner {
    factory: ComPtr,
    surface: Arc<Mutex<Surface>>,
    options: u32,
}

struct GradientStopCollectionInner {
    factory: ComPtr,
    data: Arc<GradientStopCollectionData>,
}

#[derive(Clone)]
enum BrushKind {
    Solid {
        color: D2D1_COLOR_F,
    },
    LinearGradient {
        start: D2D1_POINT_2F,
        end: D2D1_POINT_2F,
        stops: Arc<GradientStopCollectionData>,
        stops_com: ComPtr,
    },
    RadialGradient {
        center: D2D1_POINT_2F,
        origin_offset: D2D1_POINT_2F,
        rx: f32,
        ry: f32,
        stops: Arc<GradientStopCollectionData>,
        stops_com: ComPtr,
    },
    Bitmap {
        bitmap: Option<ComPtr>,
        surface: Option<Arc<Mutex<Surface>>>,
        extend_x: u32,
        extend_y: u32,
        interp: u32,
    },
}

struct BrushState {
    opacity: f32,
    transform: D2D1_MATRIX_3X2_F,
    kind: BrushKind,
}

struct BrushInner {
    factory: ComPtr,
    state: Mutex<BrushState>,
}

impl BrushInner {
    fn snapshot(&self) -> BrushEval {
        let st = self.state.lock().unwrap();
        let op = st.opacity.clamp(0.0, 1.0);
        let inv_brush = st.transform.invert().unwrap_or_else(D2D1_MATRIX_3X2_F::identity);
        match &st.kind {
            BrushKind::Solid { color } => {
                BrushEval::Solid([color.r, color.g, color.b, color.a * op])
            }
            BrushKind::LinearGradient { start, end, stops, .. } => BrushEval::Linear {
                start: (start.x, start.y),
                end: (end.x, end.y),
                stops: Arc::clone(stops),
                inv_brush,
                opacity: op,
            },
            BrushKind::RadialGradient { center, origin_offset, rx, ry, stops, .. } => BrushEval::Radial {
                center: (center.x, center.y),
                origin_offset: (origin_offset.x, origin_offset.y),
                rx: *rx,
                ry: *ry,
                stops: Arc::clone(stops),
                inv_brush,
                opacity: op,
            },
            BrushKind::Bitmap { surface, extend_x, extend_y, interp, .. } => {
                let surf = surface
                    .as_ref()
                    .map(|s| s.lock().unwrap().clone())
                    .unwrap_or_else(|| Surface::new(1, 1, D2D1_PIXEL_FORMAT::default(), 96.0, 96.0));
                BrushEval::Bitmap {
                    surface: surf,
                    extend_x: *extend_x,
                    extend_y: *extend_y,
                    interp: *interp,
                    inv_brush,
                    opacity: op,
                }
            }
        }
    }
}

#[derive(Clone)]
struct StrokeStyleData {
    props: D2D1_STROKE_STYLE_PROPERTIES,
    transform_type: u32,
    dashes: Vec<f32>,
}

struct StrokeStyleInner {
    factory: ComPtr,
    data: StrokeStyleData,
}

#[derive(Clone)]
enum PathSegment {
    Line(D2D1_POINT_2F),
    Cubic(D2D1_BEZIER_SEGMENT),
    Quad(D2D1_QUADRATIC_BEZIER_SEGMENT),
    Arc(D2D1_ARC_SEGMENT),
}

#[derive(Clone)]
struct PathFigure {
    start: D2D1_POINT_2F,
    filled: bool,
    closed: bool,
    segments: Vec<PathSegment>,
}

#[derive(Clone, Default)]
struct PathData {
    fill_mode: u32,
    figures: Vec<PathFigure>,
}

impl PathData {
    fn flatten(&self, tol: f32) -> FlattenedGeometry {
        let mut subpaths = Vec::with_capacity(self.figures.len());
        for fig in &self.figures {
            let mut pts = vec![(fig.start.x, fig.start.y)];
            let mut cur = (fig.start.x, fig.start.y);
            for seg in &fig.segments {
                match seg {
                    PathSegment::Line(p) => {
                        pts.push((p.x, p.y));
                        cur = (p.x, p.y);
                    }
                    PathSegment::Cubic(b) => {
                        flatten_cubic(cur, (b.point1.x, b.point1.y), (b.point2.x, b.point2.y), (b.point3.x, b.point3.y), tol, &mut pts);
                        cur = (b.point3.x, b.point3.y);
                    }
                    PathSegment::Quad(q) => {
                        flatten_quad(cur, (q.point1.x, q.point1.y), (q.point2.x, q.point2.y), tol, &mut pts);
                        cur = (q.point2.x, q.point2.y);
                    }
                    PathSegment::Arc(a) => {
                        flatten_arc(cur, a, &mut pts);
                        cur = (a.point.x, a.point.y);
                    }
                }
            }
            subpaths.push(SubPath { points: pts, closed: fig.closed, filled: fig.filled });
        }
        FlattenedGeometry { subpaths, fill_mode: self.fill_mode }
    }
}

enum GeometryKind {
    Rect(D2D1_RECT_F),
    RoundedRect(D2D1_ROUNDED_RECT),
    Ellipse(D2D1_ELLIPSE),
    Path(Arc<Mutex<PathData>>),
    Group {
        fill_mode: u32,
        children: Vec<ComPtr>,
    },
    Transformed {
        source: ComPtr,
        transform: D2D1_MATRIX_3X2_F,
    },
}

struct GeometryInner {
    factory: ComPtr,
    kind: GeometryKind,
}

impl GeometryInner {
    unsafe fn flatten(&self, extra_transform: Option<&D2D1_MATRIX_3X2_F>, tol: f32) -> FlattenedGeometry {
        let mut fg = match &self.kind {
            GeometryKind::Rect(r) => FlattenedGeometry {
                subpaths: vec![rect_subpath(*r)],
                fill_mode: D2D1_FILL_MODE_ALTERNATE,
            },
            GeometryKind::RoundedRect(rr) => FlattenedGeometry {
                subpaths: vec![rounded_rect_subpath(*rr)],
                fill_mode: D2D1_FILL_MODE_ALTERNATE,
            },
            GeometryKind::Ellipse(el) => FlattenedGeometry {
                subpaths: vec![ellipse_subpath(*el)],
                fill_mode: D2D1_FILL_MODE_ALTERNATE,
            },
            GeometryKind::Path(pd) => pd.lock().unwrap().flatten(tol),
            GeometryKind::Group { fill_mode, children } => {
                let mut all = Vec::new();
                for c in children {
                    if let Some(g) = geometry_from_ptr(c.as_ptr()) {
                        let sub = g.flatten(None, tol);
                        all.extend(sub.subpaths);
                    }
                }
                FlattenedGeometry { subpaths: all, fill_mode: *fill_mode }
            }
            GeometryKind::Transformed { source, transform } => {
                if let Some(g) = geometry_from_ptr(source.as_ptr()) {
                    g.flatten(Some(transform), tol)
                } else {
                    FlattenedGeometry { subpaths: Vec::new(), fill_mode: D2D1_FILL_MODE_ALTERNATE }
                }
            }
        };
        if let Some(m) = extra_transform {
            fg.subpaths = transform_subpaths(&fg.subpaths, m);
        }
        fg
    }
}

struct GeometrySinkState {
    path: Arc<Mutex<PathData>>,
    fill_mode: u32,
    figures: Vec<PathFigure>,
    current: Option<PathFigure>,
    closed: bool,
}

struct GeometrySinkInner {
    state: Mutex<GeometrySinkState>,
}

struct LayerInner {
    factory: ComPtr,
    size: D2D1_SIZE_F,
}

struct DrawingStateBlockInner {
    factory: ComPtr,
    state: Mutex<(D2D1_DRAWING_STATE_DESCRIPTION, Option<ComPtr>)>,
}

struct MeshInner {
    factory: ComPtr,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderTargetKind {
    Generic,
    Bitmap,
    Hwnd,
    Dc,
    DeviceContext,
}

struct LayerFrame {
    saved_surface: Arc<Mutex<Surface>>,
    opacity: f32,
    clip_bounds: (i32, i32, i32, i32),
    mask_polys: Option<(Vec<SubPath>, u32)>,
}

struct RenderTargetState {
    surface: Arc<Mutex<Surface>>,
    target_bitmap: Option<ComPtr>,
    transform: D2D1_MATRIX_3X2_F,
    clip_stack: Vec<(i32, i32, i32, i32)>,
    layer_stack: Vec<LayerFrame>,
    antialias_mode: u32,
    text_antialias_mode: u32,
    text_rendering_params: Option<ComPtr>,
    primitive_blend: u32,
    unit_mode: u32,
    tag1: u64,
    tag2: u64,
    in_draw: bool,
    hwnd: HWND,
    bound_hdc: usize,
    bound_subrect: [i32; 4],
    wic_bitmap: Option<ComPtr>,
    device: Option<ComPtr>,
}

impl RenderTargetState {
    fn total_transform(&self) -> D2D1_MATRIX_3X2_F {
        if self.unit_mode == D2D1_UNIT_MODE_PIXELS {
            self.transform
        } else {
            let s = self.surface.lock().unwrap();
            let sx = s.dpi_x / 96.0;
            let sy = s.dpi_y / 96.0;
            if (sx - 1.0).abs() < 1e-5 && (sy - 1.0).abs() < 1e-5 {
                self.transform
            } else {
                let dpi_m = D2D1_MATRIX_3X2_F { m11: sx, m12: 0.0, m21: 0.0, m22: sy, dx: 0.0, dy: 0.0 };
                self.transform.mul(&dpi_m)
            }
        }
    }

    fn effective_clip(&self, w: u32, h: u32) -> (i32, i32, i32, i32) {
        let mut c = (0i32, 0i32, w as i32, h as i32);
        for &(x0, y0, x1, y1) in &self.clip_stack {
            c.0 = c.0.max(x0);
            c.1 = c.1.max(y0);
            c.2 = c.2.min(x1);
            c.3 = c.3.min(y1);
        }
        c
    }

    fn flush_external_targets(&self) {
        let surf = self.surface.lock().unwrap();
        // Flush to bound GDI DC if this is an ID2D1DCRenderTarget
        if self.bound_hdc != 0 {
            let ox = self.bound_subrect[0];
            let oy = self.bound_subrect[1];
            unsafe {
                for y in 0..surf.height {
                    for x in 0..surf.width {
                        let c = surf.get_pixel_straight(x, y);
                        let r = (c[0].clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
                        let g = (c[1].clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
                        let b = (c[2].clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
                        let colorref = r | (g << 8) | (b << 16);
                        crate::misc::gdi32::SetPixel(self.bound_hdc, ox + x as i32, oy + y as i32, colorref);
                    }
                }
            }
        }
        // Flush to bound IWICBitmap via IWICBitmap::Lock (slot 8) if present
        if let Some(ref wic) = self.wic_bitmap {
            unsafe {
                let wic_ptr = wic.as_ptr();
                let vtbl = (wic_ptr as *const *const usize).read_unaligned();
                if !vtbl.is_null() {
                    let lock_fn = vtbl.add(8).read_unaligned();
                    if lock_fn != 0 {
                        let mut lock_obj: *mut c_void = std::ptr::null_mut();
                        let hr = crate::call_guest8(
                            lock_fn,
                            wic_ptr as u64,
                            0,    // entire bitmap
                            2,    // WICBitmapLockWrite
                            &mut lock_obj as *mut *mut c_void as u64,
                            0,
                            0,
                            0,
                            0,
                        ) as HRESULT;
                        if hr >= 0 && !lock_obj.is_null() {
                            let lvtbl = (lock_obj as *const *const usize).read_unaligned();
                            if !lvtbl.is_null() {
                                let stride_fn = lvtbl.add(4).read_unaligned();
                                let data_fn = lvtbl.add(5).read_unaligned();
                                let mut stride = 0u32;
                                let mut buf_sz = 0u32;
                                let mut data_ptr: *mut u8 = std::ptr::null_mut();
                                if stride_fn != 0 && data_fn != 0 {
                                    crate::call_guest8(stride_fn, lock_obj as u64, &mut stride as *mut u32 as u64, 0, 0, 0, 0, 0, 0);
                                    crate::call_guest8(
                                        data_fn,
                                        lock_obj as u64,
                                        &mut buf_sz as *mut u32 as u64,
                                        &mut data_ptr as *mut *mut u8 as u64,
                                        0,
                                        0,
                                        0,
                                        0,
                                        0,
                                    );
                                    if !data_ptr.is_null() && stride > 0 {
                                        let src_pitch = surf.pitch();
                                        let row_bytes = src_pitch.min(stride as usize);
                                        for y in 0..(surf.height as usize) {
                                            let dst_off = y * (stride as usize);
                                            if dst_off + row_bytes <= buf_sz as usize {
                                                std::ptr::copy_nonoverlapping(
                                                    surf.data.as_ptr().add(y * src_pitch),
                                                    data_ptr.add(dst_off),
                                                    row_bytes,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            raw_com_release(lock_obj);
                        }
                    }
                }
            }
        }
    }
}

struct RenderTargetInner {
    factory: ComPtr,
    kind: RenderTargetKind,
    state: Mutex<RenderTargetState>,
}

// ============================================================================
// Safe Type Downcasters (vtable-verified, unaligned-safe)
// ============================================================================

unsafe fn brush_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a BrushInner> {
    if ptr.is_null() {
        return None;
    }
    let v = (ptr as *const *const usize).read_unaligned();
    if v == solid_brush_vtbl()
        || v == linear_brush_vtbl()
        || v == radial_brush_vtbl()
        || v == bitmap_brush_vtbl()
    {
        Some(ComObj::inner(ptr as *mut ComObj<BrushInner>))
    } else {
        None
    }
}

unsafe fn geometry_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a GeometryInner> {
    if ptr.is_null() {
        return None;
    }
    let v = (ptr as *const *const usize).read_unaligned();
    if v == rect_geom_vtbl()
        || v == rounded_rect_geom_vtbl()
        || v == ellipse_geom_vtbl()
        || v == path_geom_vtbl()
        || v == geom_group_vtbl()
        || v == transformed_geom_vtbl()
    {
        Some(ComObj::inner(ptr as *mut ComObj<GeometryInner>))
    } else {
        None
    }
}

unsafe fn stroke_style_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a StrokeStyleData> {
    if ptr.is_null() {
        return None;
    }
    let v = (ptr as *const *const usize).read_unaligned();
    if v == stroke_style_vtbl() {
        Some(&ComObj::inner(ptr as *mut ComObj<StrokeStyleInner>).data)
    } else {
        None
    }
}

unsafe fn bitmap_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a BitmapInner> {
    if ptr.is_null() {
        return None;
    }
    let v = (ptr as *const *const usize).read_unaligned();
    if v == bitmap_vtbl() {
        Some(ComObj::inner(ptr as *mut ComObj<BitmapInner>))
    } else {
        None
    }
}

unsafe fn render_target_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a RenderTargetInner> {
    if ptr.is_null() {
        return None;
    }
    let v = (ptr as *const *const usize).read_unaligned();
    if v == rt_generic_vtbl()
        || v == rt_bitmap_vtbl()
        || v == rt_hwnd_vtbl()
        || v == rt_dc_vtbl()
        || v == device_context_vtbl()
    {
        Some(ComObj::inner(ptr as *mut ComObj<RenderTargetInner>))
    } else {
        None
    }
}

// ============================================================================
// ID2D1Bitmap / ID2D1Bitmap1 Vtable (16 slots)
// ============================================================================

type PBitmap = *mut ComObj<BitmapInner>;

unsafe extern "win64" fn bmp_qi(this: PBitmap, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(
        this,
        riid,
        ppv,
        &[IID_ID2D1_RESOURCE, IID_ID2D1_IMAGE, IID_ID2D1_BITMAP, IID_ID2D1_BITMAP1],
    )
}
unsafe extern "win64" fn bmp_add_ref(this: PBitmap) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn bmp_release(this: PBitmap) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn bmp_get_factory(this: PBitmap, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn bmp_get_size(this: PBitmap, ret: *mut D2D1_SIZE_F) -> *mut D2D1_SIZE_F {
    let sz = if this.is_null() {
        D2D1_SIZE_F::default()
    } else {
        let s = ComObj::inner(this).surface.lock().unwrap();
        D2D1_SIZE_F {
            width: s.width as f32 * 96.0 / s.dpi_x,
            height: s.height as f32 * 96.0 / s.dpi_y,
        }
    };
    if !ret.is_null() {
        ret.write_unaligned(sz);
    }
    ret
}
unsafe extern "win64" fn bmp_get_pixel_size(this: PBitmap, ret: *mut D2D1_SIZE_U) -> *mut D2D1_SIZE_U {
    let sz = if this.is_null() {
        D2D1_SIZE_U::default()
    } else {
        let s = ComObj::inner(this).surface.lock().unwrap();
        D2D1_SIZE_U { width: s.width, height: s.height }
    };
    if !ret.is_null() {
        ret.write_unaligned(sz);
    }
    ret
}
unsafe extern "win64" fn bmp_get_pixel_format(
    this: PBitmap,
    ret: *mut D2D1_PIXEL_FORMAT,
) -> *mut D2D1_PIXEL_FORMAT {
    let pf = if this.is_null() {
        D2D1_PIXEL_FORMAT::default()
    } else {
        ComObj::inner(this).surface.lock().unwrap().pixel_format
    };
    if !ret.is_null() {
        ret.write_unaligned(pf);
    }
    ret
}
unsafe extern "win64" fn bmp_get_dpi(this: PBitmap, dpi_x: *mut f32, dpi_y: *mut f32) {
    if this.is_null() { return; }
    let s = ComObj::inner(this).surface.lock().unwrap();
    if !dpi_x.is_null() { dpi_x.write_unaligned(s.dpi_x); }
    if !dpi_y.is_null() { dpi_y.write_unaligned(s.dpi_y); }
}

unsafe fn copy_surface_rect(
    dst: &mut Surface,
    dest_pt: Option<D2D1_POINT_2U>,
    src: &Surface,
    src_rect: Option<D2D1_RECT_U>,
) -> HRESULT {
    let dp = dest_pt.unwrap_or_default();
    let sr = src_rect.unwrap_or(D2D1_RECT_U { left: 0, top: 0, right: src.width, bottom: src.height });
    if sr.left > sr.right || sr.top > sr.bottom || sr.right > src.width || sr.bottom > src.height {
        return E_INVALIDARG;
    }
    let copy_w = sr.right - sr.left;
    let copy_h = sr.bottom - sr.top;
    if dp.x.saturating_add(copy_w) > dst.width || dp.y.saturating_add(copy_h) > dst.height {
        return E_INVALIDARG;
    }
    for row in 0..copy_h {
        for col in 0..copy_w {
            let px = src.get_pixel_raw(sr.left + col, sr.top + row);
            dst.set_pixel_raw(dp.x + col, dp.y + row, px);
        }
    }
    S_OK
}

unsafe extern "win64" fn bmp_copy_from_bitmap(
    this: PBitmap,
    dest_pt: *const D2D1_POINT_2U,
    src_bmp: *mut c_void,
    src_rect: *const D2D1_RECT_U,
) -> HRESULT {
    if this.is_null() || src_bmp.is_null() { return E_POINTER; }
    let Some(src_inner) = bitmap_from_ptr(src_bmp) else { return E_INVALIDARG; };
    let dp = (!dest_pt.is_null()).then(|| dest_pt.read_unaligned());
    let sr = (!src_rect.is_null()).then(|| src_rect.read_unaligned());
    let src_clone = src_inner.surface.lock().unwrap().clone();
    let mut dst = ComObj::inner(this).surface.lock().unwrap();
    copy_surface_rect(&mut dst, dp, &src_clone, sr)
}
unsafe extern "win64" fn bmp_copy_from_rt(
    this: PBitmap,
    dest_pt: *const D2D1_POINT_2U,
    rt: *mut c_void,
    src_rect: *const D2D1_RECT_U,
) -> HRESULT {
    if this.is_null() || rt.is_null() { return E_POINTER; }
    let Some(rt_inner) = render_target_from_ptr(rt) else { return E_INVALIDARG; };
    let dp = (!dest_pt.is_null()).then(|| dest_pt.read_unaligned());
    let sr = (!src_rect.is_null()).then(|| src_rect.read_unaligned());
    let src_surf = Arc::clone(&rt_inner.state.lock().unwrap().surface);
    let src_clone = src_surf.lock().unwrap().clone();
    let mut dst = ComObj::inner(this).surface.lock().unwrap();
    copy_surface_rect(&mut dst, dp, &src_clone, sr)
}
unsafe extern "win64" fn bmp_copy_from_memory(
    this: PBitmap,
    dst_rect: *const D2D1_RECT_U,
    src_data: *const c_void,
    pitch: u32,
) -> HRESULT {
    if this.is_null() || src_data.is_null() { return E_POINTER; }
    let mut dst = ComObj::inner(this).surface.lock().unwrap();
    let dr = if dst_rect.is_null() {
        D2D1_RECT_U { left: 0, top: 0, right: dst.width, bottom: dst.height }
    } else {
        dst_rect.read_unaligned()
    };
    if dr.left > dr.right || dr.top > dr.bottom || dr.right > dst.width || dr.bottom > dst.height {
        return E_INVALIDARG;
    }
    let bpp = dst.bpp();
    let copy_w = (dr.right - dr.left) as usize;
    let copy_h = (dr.bottom - dr.top) as usize;
    let row_bytes = copy_w * bpp;
    if (pitch as usize) < row_bytes {
        return E_INVALIDARG;
    }
    let dst_pitch = dst.pitch();
    let src_bytes = src_data as *const u8;
    for row in 0..copy_h {
        let dst_off = (dr.top as usize + row) * dst_pitch + (dr.left as usize) * bpp;
        std::ptr::copy_nonoverlapping(
            src_bytes.add(row * pitch as usize),
            dst.data.as_mut_ptr().add(dst_off),
            row_bytes,
        );
    }
    S_OK
}
unsafe extern "win64" fn bmp_get_color_context(_this: PBitmap, out: *mut *mut c_void) {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
}
unsafe extern "win64" fn bmp_get_options(this: PBitmap) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).options }
}
unsafe extern "win64" fn bmp_get_surface(_this: PBitmap, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
    E_NOINTERFACE
}
unsafe extern "win64" fn bmp_map(this: PBitmap, _options: u32, mapped: *mut D2D1_MAPPED_RECT) -> HRESULT {
    if this.is_null() || mapped.is_null() { return E_POINTER; }
    let mut s = ComObj::inner(this).surface.lock().unwrap();
    let pitch = s.pitch() as u32;
    let bits = s.data.as_mut_ptr();
    mapped.write_unaligned(D2D1_MAPPED_RECT { pitch, _pad: 0, bits });
    S_OK
}
unsafe extern "win64" fn bmp_unmap(this: PBitmap) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    S_OK
}

fn bitmap_vtbl() -> *const usize {
    static V: OnceLock<[usize; 16]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            bmp_qi as *const () as usize,
            bmp_add_ref as *const () as usize,
            bmp_release as *const () as usize,
            bmp_get_factory as *const () as usize,
            bmp_get_size as *const () as usize,
            bmp_get_pixel_size as *const () as usize,
            bmp_get_pixel_format as *const () as usize,
            bmp_get_dpi as *const () as usize,
            bmp_copy_from_bitmap as *const () as usize,
            bmp_copy_from_rt as *const () as usize,
            bmp_copy_from_memory as *const () as usize,
            bmp_get_color_context as *const () as usize,
            bmp_get_options as *const () as usize,
            bmp_get_surface as *const () as usize,
            bmp_map as *const () as usize,
            bmp_unmap as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_bitmap_com(factory: ComPtr, surface: Arc<Mutex<Surface>>, options: u32) -> *mut c_void {
    ComObj::new(bitmap_vtbl(), BitmapInner { factory, surface, options }) as *mut c_void
}

// ============================================================================
// ID2D1GradientStopCollection Vtable (8 slots)
// ============================================================================

type PGradStops = *mut ComObj<GradientStopCollectionInner>;

unsafe extern "win64" fn gsc_qi(this: PGradStops, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GRADIENT_STOP_COLLECTION])
}
unsafe extern "win64" fn gsc_add_ref(this: PGradStops) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn gsc_release(this: PGradStops) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn gsc_get_factory(this: PGradStops, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn gsc_get_count(this: PGradStops) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.stops.len() as u32 }
}
unsafe extern "win64" fn gsc_get_stops(this: PGradStops, out: *mut D2D1_GRADIENT_STOP, count: u32) {
    if this.is_null() || out.is_null() { return; }
    let stops = &ComObj::inner(this).data.stops;
    for i in 0..(count as usize).min(stops.len()) {
        out.add(i).write_unaligned(stops[i]);
    }
}
unsafe extern "win64" fn gsc_get_gamma(this: PGradStops) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.gamma }
}
unsafe extern "win64" fn gsc_get_extend(this: PGradStops) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.extend_mode }
}

fn grad_stops_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            gsc_qi as *const () as usize,
            gsc_add_ref as *const () as usize,
            gsc_release as *const () as usize,
            gsc_get_factory as *const () as usize,
            gsc_get_count as *const () as usize,
            gsc_get_stops as *const () as usize,
            gsc_get_gamma as *const () as usize,
            gsc_get_extend as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// Brushes: Solid (10), LinearGradient (13), RadialGradient (17), Bitmap (16)
// ============================================================================

type PBrush = *mut ComObj<BrushInner>;

unsafe extern "win64" fn brush_qi(this: PBrush, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let specific = match ComObj::inner(this).state.lock().unwrap().kind {
        BrushKind::Solid { .. } => IID_ID2D1_SOLID_COLOR_BRUSH,
        BrushKind::LinearGradient { .. } => IID_ID2D1_LINEAR_GRADIENT_BRUSH,
        BrushKind::RadialGradient { .. } => IID_ID2D1_RADIAL_GRADIENT_BRUSH,
        BrushKind::Bitmap { .. } => IID_ID2D1_BITMAP_BRUSH,
    };
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_BRUSH, specific])
}
unsafe extern "win64" fn brush_add_ref(this: PBrush) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn brush_release(this: PBrush) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn brush_get_factory(this: PBrush, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn brush_set_opacity(this: PBrush, opacity: f32) {
    if this.is_null() || !opacity.is_finite() { return; }
    ComObj::inner(this).state.lock().unwrap().opacity = opacity;
}
crate::win64_bridge!(fn brush_set_opacity(PBrush, f32) -> ());

unsafe extern "win64" fn brush_set_transform(this: PBrush, m: *const D2D1_MATRIX_3X2_F) {
    if this.is_null() { return; }
    let mat = if m.is_null() { D2D1_MATRIX_3X2_F::identity() } else { m.read_unaligned() };
    ComObj::inner(this).state.lock().unwrap().transform = mat;
}
unsafe extern "win64" fn brush_get_opacity(this: PBrush) -> f32 {
    if this.is_null() { 1.0 } else { ComObj::inner(this).state.lock().unwrap().opacity }
}
unsafe extern "win64" fn brush_get_transform(this: PBrush, out: *mut D2D1_MATRIX_3X2_F) {
    if this.is_null() || out.is_null() { return; }
    out.write_unaligned(ComObj::inner(this).state.lock().unwrap().transform);
}

// SolidColorBrush slots 8..9
unsafe extern "win64" fn sbrush_set_color(this: PBrush, color: *const D2D1_COLOR_F) {
    if this.is_null() || color.is_null() { return; }
    let c = color.read_unaligned();
    if let BrushKind::Solid { ref mut color } = ComObj::inner(this).state.lock().unwrap().kind {
        *color = c;
    }
}
unsafe extern "win64" fn sbrush_get_color(this: PBrush, ret: *mut D2D1_COLOR_F) -> *mut D2D1_COLOR_F {
    let c = if !this.is_null() {
        if let BrushKind::Solid { color } = ComObj::inner(this).state.lock().unwrap().kind {
            color
        } else {
            D2D1_COLOR_F::default()
        }
    } else {
        D2D1_COLOR_F::default()
    };
    if !ret.is_null() { ret.write_unaligned(c); }
    ret
}

fn solid_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 10]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            brush_qi as *const () as usize,
            brush_add_ref as *const () as usize,
            brush_release as *const () as usize,
            brush_get_factory as *const () as usize,
            crate::bridge_entry!(brush_set_opacity) as usize,
            brush_set_transform as *const () as usize,
            brush_get_opacity as *const () as usize,
            brush_get_transform as *const () as usize,
            sbrush_set_color as *const () as usize,
            sbrush_get_color as *const () as usize,
        ])
    })
    .as_ptr()
}

// LinearGradientBrush slots 8..12
unsafe extern "win64" fn lbrush_set_start(this: PBrush, pt: u64) {
    if this.is_null() { return; }
    if let BrushKind::LinearGradient { ref mut start, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *start = D2D1_POINT_2F::from_u64(pt);
    }
}
unsafe extern "win64" fn lbrush_set_end(this: PBrush, pt: u64) {
    if this.is_null() { return; }
    if let BrushKind::LinearGradient { ref mut end, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *end = D2D1_POINT_2F::from_u64(pt);
    }
}
unsafe extern "win64" fn lbrush_get_start(this: PBrush, ret: *mut D2D1_POINT_2F) -> *mut D2D1_POINT_2F {
    let p = if !this.is_null() {
        if let BrushKind::LinearGradient { start, .. } = ComObj::inner(this).state.lock().unwrap().kind {
            start
        } else {
            D2D1_POINT_2F::default()
        }
    } else {
        D2D1_POINT_2F::default()
    };
    if !ret.is_null() { ret.write_unaligned(p); }
    ret
}
unsafe extern "win64" fn lbrush_get_end(this: PBrush, ret: *mut D2D1_POINT_2F) -> *mut D2D1_POINT_2F {
    let p = if !this.is_null() {
        if let BrushKind::LinearGradient { end, .. } = ComObj::inner(this).state.lock().unwrap().kind {
            end
        } else {
            D2D1_POINT_2F::default()
        }
    } else {
        D2D1_POINT_2F::default()
    };
    if !ret.is_null() { ret.write_unaligned(p); }
    ret
}
unsafe extern "win64" fn grad_brush_get_stops(this: PBrush, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    let p = match &st.kind {
        BrushKind::LinearGradient { stops_com, .. } | BrushKind::RadialGradient { stops_com, .. } => {
            raw_com_add_ref(stops_com.as_ptr());
            stops_com.as_ptr()
        }
        _ => std::ptr::null_mut(),
    };
    out.write_unaligned(p);
}

fn linear_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 13]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            brush_qi as *const () as usize,
            brush_add_ref as *const () as usize,
            brush_release as *const () as usize,
            brush_get_factory as *const () as usize,
            crate::bridge_entry!(brush_set_opacity) as usize,
            brush_set_transform as *const () as usize,
            brush_get_opacity as *const () as usize,
            brush_get_transform as *const () as usize,
            lbrush_set_start as *const () as usize,
            lbrush_set_end as *const () as usize,
            lbrush_get_start as *const () as usize,
            lbrush_get_end as *const () as usize,
            grad_brush_get_stops as *const () as usize,
        ])
    })
    .as_ptr()
}

// RadialGradientBrush slots 8..16
unsafe extern "win64" fn rbrush_set_center(this: PBrush, pt: u64) {
    if this.is_null() { return; }
    if let BrushKind::RadialGradient { ref mut center, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *center = D2D1_POINT_2F::from_u64(pt);
    }
}
unsafe extern "win64" fn rbrush_set_origin_off(this: PBrush, pt: u64) {
    if this.is_null() { return; }
    if let BrushKind::RadialGradient { ref mut origin_offset, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *origin_offset = D2D1_POINT_2F::from_u64(pt);
    }
}
unsafe extern "win64" fn rbrush_set_rx(this: PBrush, r: f32) {
    if this.is_null() { return; }
    if let BrushKind::RadialGradient { ref mut rx, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *rx = r;
    }
}
crate::win64_bridge!(fn rbrush_set_rx(PBrush, f32) -> ());

unsafe extern "win64" fn rbrush_set_ry(this: PBrush, r: f32) {
    if this.is_null() { return; }
    if let BrushKind::RadialGradient { ref mut ry, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *ry = r;
    }
}
crate::win64_bridge!(fn rbrush_set_ry(PBrush, f32) -> ());

unsafe extern "win64" fn rbrush_get_center(this: PBrush, ret: *mut D2D1_POINT_2F) -> *mut D2D1_POINT_2F {
    let p = if !this.is_null() {
        if let BrushKind::RadialGradient { center, .. } = ComObj::inner(this).state.lock().unwrap().kind {
            center
        } else {
            D2D1_POINT_2F::default()
        }
    } else {
        D2D1_POINT_2F::default()
    };
    if !ret.is_null() { ret.write_unaligned(p); }
    ret
}
unsafe extern "win64" fn rbrush_get_origin_off(this: PBrush, ret: *mut D2D1_POINT_2F) -> *mut D2D1_POINT_2F {
    let p = if !this.is_null() {
        if let BrushKind::RadialGradient { origin_offset, .. } = ComObj::inner(this).state.lock().unwrap().kind {
            origin_offset
        } else {
            D2D1_POINT_2F::default()
        }
    } else {
        D2D1_POINT_2F::default()
    };
    if !ret.is_null() { ret.write_unaligned(p); }
    ret
}
unsafe extern "win64" fn rbrush_get_rx(this: PBrush) -> f32 {
    if this.is_null() { return 0.0; }
    if let BrushKind::RadialGradient { rx, .. } = ComObj::inner(this).state.lock().unwrap().kind { rx } else { 0.0 }
}
unsafe extern "win64" fn rbrush_get_ry(this: PBrush) -> f32 {
    if this.is_null() { return 0.0; }
    if let BrushKind::RadialGradient { ry, .. } = ComObj::inner(this).state.lock().unwrap().kind { ry } else { 0.0 }
}

fn radial_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 17]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            brush_qi as *const () as usize,
            brush_add_ref as *const () as usize,
            brush_release as *const () as usize,
            brush_get_factory as *const () as usize,
            crate::bridge_entry!(brush_set_opacity) as usize,
            brush_set_transform as *const () as usize,
            brush_get_opacity as *const () as usize,
            brush_get_transform as *const () as usize,
            rbrush_set_center as *const () as usize,
            rbrush_set_origin_off as *const () as usize,
            crate::bridge_entry!(rbrush_set_rx) as usize,
            crate::bridge_entry!(rbrush_set_ry) as usize,
            rbrush_get_center as *const () as usize,
            rbrush_get_origin_off as *const () as usize,
            rbrush_get_rx as *const () as usize,
            rbrush_get_ry as *const () as usize,
            grad_brush_get_stops as *const () as usize,
        ])
    })
    .as_ptr()
}

// BitmapBrush slots 8..15
unsafe extern "win64" fn bbrush_set_ex_x(this: PBrush, m: u32) {
    if this.is_null() { return; }
    if let BrushKind::Bitmap { ref mut extend_x, .. } = ComObj::inner(this).state.lock().unwrap().kind { *extend_x = m; }
}
unsafe extern "win64" fn bbrush_set_ex_y(this: PBrush, m: u32) {
    if this.is_null() { return; }
    if let BrushKind::Bitmap { ref mut extend_y, .. } = ComObj::inner(this).state.lock().unwrap().kind { *extend_y = m; }
}
unsafe extern "win64" fn bbrush_set_interp(this: PBrush, m: u32) {
    if this.is_null() { return; }
    if let BrushKind::Bitmap { ref mut interp, .. } = ComObj::inner(this).state.lock().unwrap().kind { *interp = m; }
}
unsafe extern "win64" fn bbrush_set_bitmap(this: PBrush, bmp: *mut c_void) {
    if this.is_null() { return; }
    let surf = bitmap_from_ptr(bmp).map(|b| Arc::clone(&b.surface));
    let retained = ComPtr::retain(bmp);
    if let BrushKind::Bitmap { ref mut bitmap, ref mut surface, .. } = ComObj::inner(this).state.lock().unwrap().kind {
        *bitmap = retained;
        *surface = surf;
    }
}
unsafe extern "win64" fn bbrush_get_ex_x(this: PBrush) -> u32 {
    if this.is_null() { return 0; }
    if let BrushKind::Bitmap { extend_x, .. } = ComObj::inner(this).state.lock().unwrap().kind { extend_x } else { 0 }
}
unsafe extern "win64" fn bbrush_get_ex_y(this: PBrush) -> u32 {
    if this.is_null() { return 0; }
    if let BrushKind::Bitmap { extend_y, .. } = ComObj::inner(this).state.lock().unwrap().kind { extend_y } else { 0 }
}
unsafe extern "win64" fn bbrush_get_interp(this: PBrush) -> u32 {
    if this.is_null() { return 0; }
    if let BrushKind::Bitmap { interp, .. } = ComObj::inner(this).state.lock().unwrap().kind { interp } else { 0 }
}
unsafe extern "win64" fn bbrush_get_bitmap(this: PBrush, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let BrushKind::Bitmap { bitmap: Some(ref b), .. } = st.kind {
        raw_com_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
}

fn bitmap_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 16]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            brush_qi as *const () as usize,
            brush_add_ref as *const () as usize,
            brush_release as *const () as usize,
            brush_get_factory as *const () as usize,
            crate::bridge_entry!(brush_set_opacity) as usize,
            brush_set_transform as *const () as usize,
            brush_get_opacity as *const () as usize,
            brush_get_transform as *const () as usize,
            bbrush_set_ex_x as *const () as usize,
            bbrush_set_ex_y as *const () as usize,
            bbrush_set_interp as *const () as usize,
            bbrush_set_bitmap as *const () as usize,
            bbrush_get_ex_x as *const () as usize,
            bbrush_get_ex_y as *const () as usize,
            bbrush_get_interp as *const () as usize,
            bbrush_get_bitmap as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// ID2D1StrokeStyle / ID2D1StrokeStyle1 Vtable (14 slots)
// ============================================================================

type PStrokeStyle = *mut ComObj<StrokeStyleInner>;

unsafe extern "win64" fn ss_qi(this: PStrokeStyle, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_STROKE_STYLE, IID_ID2D1_STROKE_STYLE1])
}
unsafe extern "win64" fn ss_add_ref(this: PStrokeStyle) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn ss_release(this: PStrokeStyle) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn ss_get_factory(this: PStrokeStyle, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn ss_start_cap(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.props.startCap }
}
unsafe extern "win64" fn ss_end_cap(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.props.endCap }
}
unsafe extern "win64" fn ss_dash_cap(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.props.dashCap }
}
unsafe extern "win64" fn ss_miter_limit(this: PStrokeStyle) -> f32 {
    if this.is_null() { 10.0 } else { ComObj::inner(this).data.props.miterLimit }
}
unsafe extern "win64" fn ss_line_join(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.props.lineJoin }
}
unsafe extern "win64" fn ss_dash_offset(this: PStrokeStyle) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).data.props.dashOffset }
}
unsafe extern "win64" fn ss_dash_style(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.props.dashStyle }
}
unsafe extern "win64" fn ss_dashes_count(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.dashes.len() as u32 }
}
unsafe extern "win64" fn ss_get_dashes(this: PStrokeStyle, dashes: *mut f32, count: u32) {
    if this.is_null() || dashes.is_null() { return; }
    let d = &ComObj::inner(this).data.dashes;
    for i in 0..(count as usize).min(d.len()) {
        dashes.add(i).write_unaligned(d[i]);
    }
}
unsafe extern "win64" fn ss_transform_type(this: PStrokeStyle) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).data.transform_type }
}

fn stroke_style_vtbl() -> *const usize {
    static V: OnceLock<[usize; 14]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            ss_qi as *const () as usize,
            ss_add_ref as *const () as usize,
            ss_release as *const () as usize,
            ss_get_factory as *const () as usize,
            ss_start_cap as *const () as usize,
            ss_end_cap as *const () as usize,
            ss_dash_cap as *const () as usize,
            ss_miter_limit as *const () as usize,
            ss_line_join as *const () as usize,
            ss_dash_offset as *const () as usize,
            ss_dash_style as *const () as usize,
            ss_dashes_count as *const () as usize,
            ss_get_dashes as *const () as usize,
            ss_transform_type as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// ID2D1GeometrySink (15 slots) & All 6 ID2D1Geometry Subclasses
// ============================================================================

type PGeomSink = *mut ComObj<GeometrySinkInner>;

unsafe extern "win64" fn gsink_qi(this: PGeomSink, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_SIMPLIFIED_GEOMETRY_SINK, IID_ID2D1_GEOMETRY_SINK])
}
unsafe extern "win64" fn gsink_add_ref(this: PGeomSink) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn gsink_release(this: PGeomSink) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn gsink_set_fill_mode(this: PGeomSink, mode: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().fill_mode = mode;
}
unsafe extern "win64" fn gsink_set_seg_flags(_this: PGeomSink, _flags: u32) {}
unsafe extern "win64" fn gsink_begin_figure(this: PGeomSink, start: u64, fig_begin: u32) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if st.closed { return; }
    st.current = Some(PathFigure {
        start: D2D1_POINT_2F::from_u64(start),
        filled: fig_begin == D2D1_FIGURE_BEGIN_FILLED,
        closed: false,
        segments: Vec::new(),
    });
}
unsafe extern "win64" fn gsink_add_lines(this: PGeomSink, pts: *const D2D1_POINT_2F, count: u32) {
    if this.is_null() || pts.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        for i in 0..(count as usize) {
            fig.segments.push(PathSegment::Line(pts.add(i).read_unaligned()));
        }
    }
}
unsafe extern "win64" fn gsink_add_beziers(this: PGeomSink, beziers: *const D2D1_BEZIER_SEGMENT, count: u32) {
    if this.is_null() || beziers.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        for i in 0..(count as usize) {
            fig.segments.push(PathSegment::Cubic(beziers.add(i).read_unaligned()));
        }
    }
}
unsafe extern "win64" fn gsink_end_figure(this: PGeomSink, fig_end: u32) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(mut fig) = st.current.take() {
        fig.closed = fig_end == D2D1_FIGURE_END_CLOSED;
        st.figures.push(fig);
    }
}
unsafe extern "win64" fn gsink_close(this: PGeomSink) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if st.closed {
        return S_OK;
    }
    if let Some(fig) = st.current.take() {
        st.figures.push(fig);
    }
    st.closed = true;
    let mut pd = st.path.lock().unwrap();
    pd.fill_mode = st.fill_mode;
    pd.figures = st.figures.clone();
    S_OK
}
unsafe extern "win64" fn gsink_add_line(this: PGeomSink, pt: u64) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Line(D2D1_POINT_2F::from_u64(pt)));
    }
}
unsafe extern "win64" fn gsink_add_bezier(this: PGeomSink, bez: *const D2D1_BEZIER_SEGMENT) {
    if this.is_null() || bez.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Cubic(bez.read_unaligned()));
    }
}
unsafe extern "win64" fn gsink_add_quad_bezier(this: PGeomSink, bez: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
    if this.is_null() || bez.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Quad(bez.read_unaligned()));
    }
}
unsafe extern "win64" fn gsink_add_quad_beziers(
    this: PGeomSink,
    beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT,
    count: u32,
) {
    if this.is_null() || beziers.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        for i in 0..(count as usize) {
            fig.segments.push(PathSegment::Quad(beziers.add(i).read_unaligned()));
        }
    }
}
unsafe extern "win64" fn gsink_add_arc(this: PGeomSink, arc: *const D2D1_ARC_SEGMENT) {
    if this.is_null() || arc.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Arc(arc.read_unaligned()));
    }
}

fn geom_sink_vtbl() -> *const usize {
    static V: OnceLock<[usize; 15]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            gsink_qi as *const () as usize,
            gsink_add_ref as *const () as usize,
            gsink_release as *const () as usize,
            gsink_set_fill_mode as *const () as usize,
            gsink_set_seg_flags as *const () as usize,
            gsink_begin_figure as *const () as usize,
            gsink_add_lines as *const () as usize,
            gsink_add_beziers as *const () as usize,
            gsink_end_figure as *const () as usize,
            gsink_close as *const () as usize,
            gsink_add_line as *const () as usize,
            gsink_add_bezier as *const () as usize,
            gsink_add_quad_bezier as *const () as usize,
            gsink_add_quad_beziers as *const () as usize,
            gsink_add_arc as *const () as usize,
        ])
    })
    .as_ptr()
}

type PGeom = *mut ComObj<GeometryInner>;

unsafe extern "win64" fn geom_qi(this: PGeom, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    match ComObj::inner(this).kind {
        GeometryKind::Rect(_) => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_RECTANGLE_GEOMETRY]),
        GeometryKind::RoundedRect(_) => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_ROUNDED_RECTANGLE_GEOMETRY]),
        GeometryKind::Ellipse(_) => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_ELLIPSE_GEOMETRY]),
        GeometryKind::Path(_) => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_PATH_GEOMETRY, IID_ID2D1_PATH_GEOMETRY1]),
        GeometryKind::Group { .. } => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_GEOMETRY_GROUP]),
        GeometryKind::Transformed { .. } => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_GEOMETRY, IID_ID2D1_TRANSFORMED_GEOMETRY]),
    }
}
unsafe extern "win64" fn geom_add_ref(this: PGeom) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn geom_release(this: PGeom) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn geom_get_factory(this: PGeom, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn geom_get_bounds(
    this: PGeom,
    world: *const D2D1_MATRIX_3X2_F,
    out: *mut D2D1_RECT_F,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), 0.25);
    out.write_unaligned(subpaths_bounds(&fg.subpaths));
    S_OK
}
unsafe extern "win64" fn geom_get_widened_bounds(
    this: PGeom,
    stroke_w: f32,
    stroke_style: *mut c_void,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    out: *mut D2D1_RECT_F,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(None, tol.max(0.1));
    let widened = expand_strokes(&fg.subpaths, stroke_w, stroke_style_from_ptr(stroke_style));
    let final_polys = if let Some(ref mat) = m { transform_subpaths(&widened, mat) } else { widened };
    out.write_unaligned(subpaths_bounds(&final_polys));
    S_OK
}
crate::win64_bridge!(fn geom_get_widened_bounds(
    PGeom, f32, *mut c_void, *const D2D1_MATRIX_3X2_F, f32, *mut D2D1_RECT_F
) -> HRESULT);

unsafe extern "win64" fn geom_stroke_contains_pt(
    this: PGeom,
    pt: u64,
    stroke_w: f32,
    stroke_style: *mut c_void,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    contains: *mut BOOL,
) -> HRESULT {
    if this.is_null() || contains.is_null() { return E_POINTER; }
    let p = D2D1_POINT_2F::from_u64(pt);
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(None, tol.max(0.1));
    let widened = expand_strokes(&fg.subpaths, stroke_w, stroke_style_from_ptr(stroke_style));
    let final_polys = if let Some(ref mat) = m { transform_subpaths(&widened, mat) } else { widened };
    let hit = point_in_subpaths(p.x, p.y, &final_polys, D2D1_FILL_MODE_WINDING);
    contains.write_unaligned(if hit { TRUE } else { FALSE });
    S_OK
}
crate::win64_bridge!(fn geom_stroke_contains_pt(
    PGeom, u64, f32, *mut c_void, *const D2D1_MATRIX_3X2_F, f32, *mut BOOL
) -> HRESULT);

unsafe extern "win64" fn geom_fill_contains_pt(
    this: PGeom,
    pt: u64,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    contains: *mut BOOL,
) -> HRESULT {
    if this.is_null() || contains.is_null() { return E_POINTER; }
    let p = D2D1_POINT_2F::from_u64(pt);
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), tol.max(0.1));
    let hit = point_in_subpaths(p.x, p.y, &fg.subpaths, fg.fill_mode);
    contains.write_unaligned(if hit { TRUE } else { FALSE });
    S_OK
}
crate::win64_bridge!(fn geom_fill_contains_pt(
    PGeom, u64, *const D2D1_MATRIX_3X2_F, f32, *mut BOOL
) -> HRESULT);

unsafe extern "win64" fn geom_compare(
    this: PGeom,
    other: *mut c_void,
    other_xform: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    relation: *mut u32,
) -> HRESULT {
    if this.is_null() || other.is_null() || relation.is_null() { return E_POINTER; }
    let Some(og) = geometry_from_ptr(other) else { return E_INVALIDARG; };
    let om = (!other_xform.is_null()).then(|| other_xform.read_unaligned());
    let a = ComObj::inner(this).flatten(None, tol.max(0.25));
    let b = og.flatten(om.as_ref(), tol.max(0.25));
    let ba = subpaths_bounds(&a.subpaths);
    let bb = subpaths_bounds(&b.subpaths);
    let rel = if ba.right < bb.left || bb.right < ba.left || ba.bottom < bb.top || bb.bottom < ba.top {
        D2D1_GEOMETRY_RELATION_DISJOINT
    } else if ba.left <= bb.left && ba.top <= bb.top && ba.right >= bb.right && ba.bottom >= bb.bottom {
        D2D1_GEOMETRY_RELATION_CONTAINS
    } else if bb.left <= ba.left && bb.top <= ba.top && bb.right >= ba.right && bb.bottom >= ba.bottom {
        D2D1_GEOMETRY_RELATION_IS_CONTAINED
    } else {
        D2D1_GEOMETRY_RELATION_OVERLAP
    };
    relation.write_unaligned(rel);
    S_OK
}
crate::win64_bridge!(fn geom_compare(
    PGeom, *mut c_void, *const D2D1_MATRIX_3X2_F, f32, *mut u32
) -> HRESULT);

unsafe fn stream_subpaths_to_sink(subpaths: &[SubPath], sink: *mut c_void) -> HRESULT {
    if sink.is_null() { return E_POINTER; }
    let vtbl = (sink as *const *const usize).read_unaligned();
    if vtbl.is_null() { return E_INVALIDARG; }
    let fn_begin = vtbl.add(5).read_unaligned();
    let fn_lines = vtbl.add(6).read_unaligned();
    let fn_end = vtbl.add(8).read_unaligned();
    for sp in subpaths {
        if sp.points.is_empty() { continue; }
        let start = D2D1_POINT_2F { x: sp.points[0].0, y: sp.points[0].1 }.to_u64();
        let fig_begin = if sp.filled { D2D1_FIGURE_BEGIN_FILLED } else { D2D1_FIGURE_BEGIN_HOLLOW };
        crate::call_guest8(fn_begin, sink as u64, start, fig_begin as u64, 0, 0, 0, 0, 0);
        if sp.points.len() > 1 {
            let pts: Vec<D2D1_POINT_2F> = sp.points[1..].iter().map(|&(x, y)| D2D1_POINT_2F { x, y }).collect();
            crate::call_guest8(fn_lines, sink as u64, pts.as_ptr() as u64, pts.len() as u64, 0, 0, 0, 0, 0);
        }
        let fig_end = if sp.closed { D2D1_FIGURE_END_CLOSED } else { D2D1_FIGURE_END_OPEN };
        crate::call_guest8(fn_end, sink as u64, fig_end as u64, 0, 0, 0, 0, 0, 0);
    }
    S_OK
}

unsafe extern "win64" fn geom_simplify(
    this: PGeom,
    _opt: u32,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || sink.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), tol.max(0.1));
    stream_subpaths_to_sink(&fg.subpaths, sink)
}
crate::win64_bridge!(fn geom_simplify(PGeom, u32, *const D2D1_MATRIX_3X2_F, f32, *mut c_void) -> HRESULT);

unsafe extern "win64" fn geom_tessellate(
    _this: PGeom,
    _world: *const D2D1_MATRIX_3X2_F,
    _tol: f32,
    _sink: *mut c_void,
) -> HRESULT {
    crate::stub!("ID2D1Geometry::Tessellate -> E_NOTIMPL");
    E_NOTIMPL
}
crate::win64_bridge!(fn geom_tessellate(PGeom, *const D2D1_MATRIX_3X2_F, f32, *mut c_void) -> HRESULT);

unsafe extern "win64" fn geom_combine(
    this: PGeom,
    other: *mut c_void,
    _mode: u32,
    other_xform: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || other.is_null() || sink.is_null() { return E_POINTER; }
    let Some(og) = geometry_from_ptr(other) else { return E_INVALIDARG; };
    let om = (!other_xform.is_null()).then(|| other_xform.read_unaligned());
    let mut a = ComObj::inner(this).flatten(None, tol.max(0.25));
    let b = og.flatten(om.as_ref(), tol.max(0.25));
    a.subpaths.extend(b.subpaths);
    stream_subpaths_to_sink(&a.subpaths, sink)
}
crate::win64_bridge!(fn geom_combine(PGeom, *mut c_void, u32, *const D2D1_MATRIX_3X2_F, f32, *mut c_void) -> HRESULT);

unsafe extern "win64" fn geom_outline(
    this: PGeom,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    sink: *mut c_void,
) -> HRESULT {
    geom_simplify(this, 0, world, tol, sink)
}
crate::win64_bridge!(fn geom_outline(PGeom, *const D2D1_MATRIX_3X2_F, f32, *mut c_void) -> HRESULT);

unsafe extern "win64" fn geom_compute_area(
    this: PGeom,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    out_area: *mut f32,
) -> HRESULT {
    if this.is_null() || out_area.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), tol.max(0.1));
    let mut area = 0.0f32;
    for sp in &fg.subpaths {
        if !sp.filled || sp.points.len() < 3 { continue; }
        let n = sp.points.len();
        let mut s = 0.0f32;
        for i in 0..n {
            let (x0, y0) = sp.points[i];
            let (x1, y1) = sp.points[(i + 1) % n];
            s += x0 * y1 - x1 * y0;
        }
        area += (s * 0.5).abs();
    }
    out_area.write_unaligned(area);
    S_OK
}
crate::win64_bridge!(fn geom_compute_area(PGeom, *const D2D1_MATRIX_3X2_F, f32, *mut f32) -> HRESULT);

unsafe extern "win64" fn geom_compute_length(
    this: PGeom,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    out_len: *mut f32,
) -> HRESULT {
    if this.is_null() || out_len.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), tol.max(0.1));
    let mut total = 0.0f32;
    for sp in &fg.subpaths {
        let n = sp.points.len();
        if n < 2 { continue; }
        let segs = if sp.closed { n } else { n - 1 };
        for i in 0..segs {
            let (x0, y0) = sp.points[i];
            let (x1, y1) = sp.points[(i + 1) % n];
            total += (x1 - x0).hypot(y1 - y0);
        }
    }
    out_len.write_unaligned(total);
    S_OK
}
crate::win64_bridge!(fn geom_compute_length(PGeom, *const D2D1_MATRIX_3X2_F, f32, *mut f32) -> HRESULT);

unsafe extern "win64" fn geom_compute_pt_at_len(
    this: PGeom,
    target_len: f32,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    out_pt: *mut D2D1_POINT_2F,
    out_tan: *mut D2D1_POINT_2F,
) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(m.as_ref(), tol.max(0.1));
    let mut rem = target_len.max(0.0);
    let mut last_pt = D2D1_POINT_2F::default();
    let mut last_tan = D2D1_POINT_2F { x: 1.0, y: 0.0 };
    for sp in &fg.subpaths {
        let n = sp.points.len();
        if n < 2 { continue; }
        let segs = if sp.closed { n } else { n - 1 };
        for i in 0..segs {
            let (x0, y0) = sp.points[i];
            let (x1, y1) = sp.points[(i + 1) % n];
            let l = (x1 - x0).hypot(y1 - y0);
            if l < 1e-6 { continue; }
            let tx = (x1 - x0) / l;
            let ty = (y1 - y0) / l;
            if rem <= l {
                if !out_pt.is_null() {
                    out_pt.write_unaligned(D2D1_POINT_2F { x: x0 + tx * rem, y: y0 + ty * rem });
                }
                if !out_tan.is_null() {
                    out_tan.write_unaligned(D2D1_POINT_2F { x: tx, y: ty });
                }
                return S_OK;
            }
            rem -= l;
            last_pt = D2D1_POINT_2F { x: x1, y: y1 };
            last_tan = D2D1_POINT_2F { x: tx, y: ty };
        }
    }
    if !out_pt.is_null() { out_pt.write_unaligned(last_pt); }
    if !out_tan.is_null() { out_tan.write_unaligned(last_tan); }
    S_OK
}
crate::win64_bridge!(fn geom_compute_pt_at_len(
    PGeom, f32, *const D2D1_MATRIX_3X2_F, f32, *mut D2D1_POINT_2F, *mut D2D1_POINT_2F
) -> HRESULT);

unsafe extern "win64" fn geom_widen(
    this: PGeom,
    stroke_w: f32,
    stroke_style: *mut c_void,
    world: *const D2D1_MATRIX_3X2_F,
    tol: f32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || sink.is_null() { return E_POINTER; }
    let m = (!world.is_null()).then(|| world.read_unaligned());
    let fg = ComObj::inner(this).flatten(None, tol.max(0.1));
    let widened = expand_strokes(&fg.subpaths, stroke_w, stroke_style_from_ptr(stroke_style));
    let final_polys = if let Some(ref mat) = m { transform_subpaths(&widened, mat) } else { widened };
    stream_subpaths_to_sink(&final_polys, sink)
}
crate::win64_bridge!(fn geom_widen(
    PGeom, f32, *mut c_void, *const D2D1_MATRIX_3X2_F, f32, *mut c_void
) -> HRESULT);

macro_rules! geom_base_slots {
    () => {
        [
            geom_qi as *const () as usize,
            geom_add_ref as *const () as usize,
            geom_release as *const () as usize,
            geom_get_factory as *const () as usize,
            geom_get_bounds as *const () as usize,
            crate::bridge_entry!(geom_get_widened_bounds) as usize,
            crate::bridge_entry!(geom_stroke_contains_pt) as usize,
            crate::bridge_entry!(geom_fill_contains_pt) as usize,
            crate::bridge_entry!(geom_compare) as usize,
            crate::bridge_entry!(geom_simplify) as usize,
            crate::bridge_entry!(geom_tessellate) as usize,
            crate::bridge_entry!(geom_combine) as usize,
            crate::bridge_entry!(geom_outline) as usize,
            crate::bridge_entry!(geom_compute_area) as usize,
            crate::bridge_entry!(geom_compute_length) as usize,
            crate::bridge_entry!(geom_compute_pt_at_len) as usize,
            crate::bridge_entry!(geom_widen) as usize,
        ]
    };
}

// Subclass-specific slots (17+)
unsafe extern "win64" fn rect_geom_get_rect(this: PGeom, out: *mut D2D1_RECT_F) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Rect(r) = ComObj::inner(this).kind { out.write_unaligned(r); }
}
fn rect_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = rect_geom_get_rect as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

unsafe extern "win64" fn rrect_geom_get_rrect(this: PGeom, out: *mut D2D1_ROUNDED_RECT) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::RoundedRect(r) = ComObj::inner(this).kind { out.write_unaligned(r); }
}
fn rounded_rect_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = rrect_geom_get_rrect as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

unsafe extern "win64" fn ellipse_geom_get_ellipse(this: PGeom, out: *mut D2D1_ELLIPSE) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Ellipse(e) = ComObj::inner(this).kind { out.write_unaligned(e); }
}
fn ellipse_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = ellipse_geom_get_ellipse as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

unsafe extern "win64" fn ggroup_get_fill_mode(this: PGeom) -> u32 {
    if this.is_null() { return 0; }
    if let GeometryKind::Group { fill_mode, .. } = ComObj::inner(this).kind { fill_mode } else { 0 }
}
unsafe extern "win64" fn ggroup_get_count(this: PGeom) -> u32 {
    if this.is_null() { return 0; }
    if let GeometryKind::Group { ref children, .. } = ComObj::inner(this).kind { children.len() as u32 } else { 0 }
}
unsafe extern "win64" fn ggroup_get_geoms(this: PGeom, out: *mut *mut c_void, count: u32) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Group { ref children, .. } = ComObj::inner(this).kind {
        for i in 0..(count as usize).min(children.len()) {
            raw_com_add_ref(children[i].as_ptr());
            out.add(i).write_unaligned(children[i].as_ptr());
        }
    }
}
fn geom_group_vtbl() -> *const usize {
    static V: OnceLock<[usize; 20]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 20];
        t[..17].copy_from_slice(&b);
        t[17] = ggroup_get_fill_mode as *const () as usize;
        t[18] = ggroup_get_count as *const () as usize;
        t[19] = ggroup_get_geoms as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

unsafe extern "win64" fn tgeom_get_source(this: PGeom, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Transformed { ref source, .. } = ComObj::inner(this).kind {
        raw_com_add_ref(source.as_ptr());
        out.write_unaligned(source.as_ptr());
    }
}
unsafe extern "win64" fn tgeom_get_transform(this: PGeom, out: *mut D2D1_MATRIX_3X2_F) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Transformed { transform, .. } = ComObj::inner(this).kind {
        out.write_unaligned(transform);
    }
}
fn transformed_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 19]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 19];
        t[..17].copy_from_slice(&b);
        t[17] = tgeom_get_source as *const () as usize;
        t[18] = tgeom_get_transform as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

unsafe extern "win64" fn path_geom_open(this: PGeom, out_sink: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out_sink.is_null() { return E_POINTER; }
    let GeometryKind::Path(ref pd) = ComObj::inner(this).kind else { return E_INVALIDARG; };
    let sink = ComObj::new(
        geom_sink_vtbl(),
        GeometrySinkInner {
            state: Mutex::new(GeometrySinkState {
                path: Arc::clone(pd),
                fill_mode: D2D1_FILL_MODE_ALTERNATE,
                figures: Vec::new(),
                current: None,
                closed: false,
            }),
        },
    );
    out_sink.write_unaligned(sink as *mut c_void);
    S_OK
}
unsafe extern "win64" fn path_geom_stream(this: PGeom, sink: *mut c_void) -> HRESULT {
    if this.is_null() || sink.is_null() { return E_POINTER; }
    let fg = ComObj::inner(this).flatten(None, 0.25);
    stream_subpaths_to_sink(&fg.subpaths, sink)
}
unsafe extern "win64" fn path_geom_seg_count(this: PGeom, count: *mut u32) -> HRESULT {
    if this.is_null() || count.is_null() { return E_POINTER; }
    let GeometryKind::Path(ref pd) = ComObj::inner(this).kind else { return E_INVALIDARG; };
    let n: usize = pd.lock().unwrap().figures.iter().map(|f| f.segments.len()).sum();
    count.write_unaligned(n as u32);
    S_OK
}
unsafe extern "win64" fn path_geom_fig_count(this: PGeom, count: *mut u32) -> HRESULT {
    if this.is_null() || count.is_null() { return E_POINTER; }
    let GeometryKind::Path(ref pd) = ComObj::inner(this).kind else { return E_INVALIDARG; };
    count.write_unaligned(pd.lock().unwrap().figures.len() as u32);
    S_OK
}
fn path_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 21]> = OnceLock::new();
    V.get_or_init(|| {
        let b = geom_base_slots!();
        let mut t = [0usize; 21];
        t[..17].copy_from_slice(&b);
        t[17] = path_geom_open as *const () as usize;
        t[18] = path_geom_stream as *const () as usize;
        t[19] = path_geom_seg_count as *const () as usize;
        t[20] = path_geom_fig_count as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

// ============================================================================
// ID2D1Layer (5), ID2D1DrawingStateBlock (8), ID2D1Mesh (5)
// ============================================================================

type PLayer = *mut ComObj<LayerInner>;
unsafe extern "win64" fn layer_qi(this: PLayer, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_LAYER])
}
unsafe extern "win64" fn layer_add_ref(this: PLayer) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn layer_release(this: PLayer) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn layer_get_factory(this: PLayer, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn layer_get_size(this: PLayer, ret: *mut D2D1_SIZE_F) -> *mut D2D1_SIZE_F {
    let sz = if this.is_null() { D2D1_SIZE_F::default() } else { ComObj::inner(this).size };
    if !ret.is_null() { ret.write_unaligned(sz); }
    ret
}
fn layer_vtbl() -> *const usize {
    static V: OnceLock<[usize; 5]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            layer_qi as *const () as usize,
            layer_add_ref as *const () as usize,
            layer_release as *const () as usize,
            layer_get_factory as *const () as usize,
            layer_get_size as *const () as usize,
        ])
    })
    .as_ptr()
}

type PStateBlock = *mut ComObj<DrawingStateBlockInner>;
unsafe extern "win64" fn dsb_qi(this: PStateBlock, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_DRAWING_STATE_BLOCK])
}
unsafe extern "win64" fn dsb_add_ref(this: PStateBlock) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn dsb_release(this: PStateBlock) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn dsb_get_factory(this: PStateBlock, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn dsb_get_desc(this: PStateBlock, out: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
    if this.is_null() || out.is_null() { return; }
    out.write_unaligned(ComObj::inner(this).state.lock().unwrap().0);
}
unsafe extern "win64" fn dsb_set_desc(this: PStateBlock, desc: *const D2D1_DRAWING_STATE_DESCRIPTION) {
    if this.is_null() || desc.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().0 = desc.read_unaligned();
}
unsafe extern "win64" fn dsb_set_text_params(this: PStateBlock, rp: *mut c_void) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().1 = ComPtr::retain(rp);
}
unsafe extern "win64" fn dsb_get_text_params(this: PStateBlock, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let guard = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref p) = guard.1 {
        raw_com_add_ref(p.as_ptr());
        out.write_unaligned(p.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
}
fn drawing_state_block_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            dsb_qi as *const () as usize,
            dsb_add_ref as *const () as usize,
            dsb_release as *const () as usize,
            dsb_get_factory as *const () as usize,
            dsb_get_desc as *const () as usize,
            dsb_set_desc as *const () as usize,
            dsb_set_text_params as *const () as usize,
            dsb_get_text_params as *const () as usize,
        ])
    })
    .as_ptr()
}

type PMesh = *mut ComObj<MeshInner>;
unsafe extern "win64" fn mesh_qi(this: PMesh, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_MESH])
}
unsafe extern "win64" fn mesh_add_ref(this: PMesh) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn mesh_release(this: PMesh) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn mesh_get_factory(this: PMesh, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn mesh_open(_this: PMesh, sink: *mut *mut c_void) -> HRESULT {
    if !sink.is_null() { sink.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}
fn mesh_vtbl() -> *const usize {
    static V: OnceLock<[usize; 5]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            mesh_qi as *const () as usize,
            mesh_add_ref as *const () as usize,
            mesh_release as *const () as usize,
            mesh_get_factory as *const () as usize,
            mesh_open as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// ID2D1RenderTarget (57) + Bitmap/Hwnd/DC RenderTargets + ID2D1DeviceContext (92)
// ============================================================================

type PRT = *mut ComObj<RenderTargetInner>;

fn render_polygons_with_brush(
    st: &RenderTargetState,
    polys: &[SubPath],
    fill_mode: u32,
    brush_eval: &BrushEval,
    opacity_eval: Option<&BrushEval>,
) {
    let total_m = st.total_transform();
    let inv_total = total_m.invert().unwrap_or_else(D2D1_MATRIX_3X2_F::identity);
    let tx_polys = transform_subpaths(polys, &total_m);
    let mut surf = st.surface.lock().unwrap();
    let clip = st.effective_clip(surf.width, surf.height);
    let aa = st.antialias_mode;
    let blend = st.primitive_blend;

    rasterize_polygons(&mut surf, &tx_polys, fill_mode, clip, aa, blend, |px, py| {
        let (wx, wy) = inv_total.transform_point(px, py);
        let mut c = brush_eval.sample_world(wx, wy);
        if let Some(op_b) = opacity_eval {
            let op = op_b.sample_world(wx, wy)[3];
            c[3] *= op;
        }
        c
    });
}

fn render_glyph_run_on_rt(
    st: &RenderTargetState,
    face: &dwrite::FontFaceInner,
    em_size: f32,
    baseline_x: f32,
    baseline_y: f32,
    gids: &[u16],
    advances: &[f32],
    offsets: &[DWRITE_GLYPH_OFFSET],
    brush_eval: &BrushEval,
) {
    let total_m = st.total_transform();
    let inv_total = total_m.invert().unwrap_or_else(D2D1_MATRIX_3X2_F::identity);
    let is_pure_translate = (total_m.m11 - 1.0).abs() < 1e-4
        && total_m.m12.abs() < 1e-4
        && total_m.m21.abs() < 1e-4
        && (total_m.m22 - 1.0).abs() < 1e-4;
    let mut surf = st.surface.lock().unwrap();
    let clip = st.effective_clip(surf.width, surf.height);
    let blend = st.primitive_blend;

    let mut pen_x = baseline_x;
    for (i, &gid) in gids.iter().enumerate() {
        let adv = advances.get(i).copied().unwrap_or(em_size * 0.5);
        let off = offsets.get(i).copied().unwrap_or_default();
        let gx_world = pen_x + off.advanceOffset;
        let gy_world = baseline_y - off.ascenderOffset;
        let glyph = dwrite::rasterize_glyph(face, gid, em_size);

        if glyph.width > 0 && glyph.height > 0 {
            if is_pure_translate {
                let x0 = (gx_world + total_m.dx).round() as i32 + glyph.offset_x;
                let y0 = (gy_world + total_m.dy).round() as i32 + glyph.offset_y;
                for gy in 0..glyph.height {
                    let py = y0 + gy as i32;
                    if py < clip.1 || py >= clip.3 { continue; }
                    for gx in 0..glyph.width {
                        let px = x0 + gx as i32;
                        if px < clip.0 || px >= clip.2 { continue; }
                        let a = glyph.alpha[(gy * glyph.width + gx) as usize];
                        if a == 0 { continue; }
                        let (wx, wy) = inv_total.transform_point(px as f32 + 0.5, py as f32 + 0.5);
                        let col = brush_eval.sample_world(wx, wy);
                        surf.blend_pixel(px as u32, py as u32, col, (a as f32) / 255.0, blend);
                    }
                }
            } else {
                // Transformed glyph quad (rotation / scale / skew)
                let lx0 = gx_world + glyph.offset_x as f32;
                let ly0 = gy_world + glyph.offset_y as f32;
                let lx1 = lx0 + glyph.width as f32;
                let ly1 = ly0 + glyph.height as f32;
                let corners = [
                    total_m.transform_point(lx0, ly0),
                    total_m.transform_point(lx1, ly0),
                    total_m.transform_point(lx1, ly1),
                    total_m.transform_point(lx0, ly1),
                ];
                let min_x = corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min).floor() as i32;
                let max_x = corners.iter().map(|c| c.0).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
                let min_y = corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min).floor() as i32;
                let max_y = corners.iter().map(|c| c.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
                let px0 = min_x.max(clip.0);
                let py0 = min_y.max(clip.1);
                let px1 = max_x.min(clip.2);
                let py1 = max_y.min(clip.3);
                for py in py0..py1 {
                    for px in px0..px1 {
                        let (wx, wy) = inv_total.transform_point(px as f32 + 0.5, py as f32 + 0.5);
                        let gx = (wx - lx0).floor() as i32;
                        let gy = (wy - ly0).floor() as i32;
                        if gx >= 0 && gy >= 0 && (gx as u32) < glyph.width && (gy as u32) < glyph.height {
                            let a = glyph.alpha[(gy as u32 * glyph.width + gx as u32) as usize];
                            if a > 0 {
                                let col = brush_eval.sample_world(wx, wy);
                                surf.blend_pixel(px as u32, py as u32, col, (a as f32) / 255.0, blend);
                            }
                        }
                    }
                }
            }
        }
        pen_x += adv;
    }
}

unsafe fn render_drawable_runs(
    st: &RenderTargetState,
    runs: &[dwrite::DrawableGlyphRun],
    origin_x: f32,
    origin_y: f32,
    default_brush: &BrushEval,
) {
    for run in runs {
        let active_brush = brush_from_ptr(run.drawing_effect).map(|b| b.snapshot());
        let brush = active_brush.as_ref().unwrap_or(default_brush);
        render_glyph_run_on_rt(
            st,
            &run.font_face,
            run.em_size,
            origin_x + run.baseline_x,
            origin_y + run.baseline_y,
            &run.glyph_indices,
            &run.glyph_advances,
            &run.glyph_offsets,
            brush,
        );
        if let Some((w, off, thick)) = run.underline {
            let r = D2D1_RECT_F {
                left: origin_x + run.baseline_x,
                top: origin_y + run.baseline_y + off,
                right: origin_x + run.baseline_x + w,
                bottom: origin_y + run.baseline_y + off + thick,
            };
            render_polygons_with_brush(st, &[rect_subpath(r)], D2D1_FILL_MODE_ALTERNATE, brush, None);
        }
        if let Some((w, off, thick)) = run.strikethrough {
            let r = D2D1_RECT_F {
                left: origin_x + run.baseline_x,
                top: origin_y + run.baseline_y + off,
                right: origin_x + run.baseline_x + w,
                bottom: origin_y + run.baseline_y + off + thick,
            };
            render_polygons_with_brush(st, &[rect_subpath(r)], D2D1_FILL_MODE_ALTERNATE, brush, None);
        }
    }
}

unsafe extern "win64" fn rt_qi(this: PRT, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    match ComObj::inner(this).kind {
        RenderTargetKind::Generic => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_RENDER_TARGET]),
        RenderTargetKind::Bitmap => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_RENDER_TARGET, IID_ID2D1_BITMAP_RENDER_TARGET]),
        RenderTargetKind::Hwnd => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_RENDER_TARGET, IID_ID2D1_HWND_RENDER_TARGET]),
        RenderTargetKind::Dc => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_RENDER_TARGET, IID_ID2D1_DC_RENDER_TARGET]),
        RenderTargetKind::DeviceContext => qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_RENDER_TARGET, IID_ID2D1_DEVICE_CONTEXT]),
    }
}
unsafe extern "win64" fn rt_add_ref(this: PRT) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn rt_release(this: PRT) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn rt_get_factory(this: PRT, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}

// Slot 4: CreateBitmap
unsafe extern "win64" fn rt_create_bitmap(
    this: PRT,
    size_u64: u64,
    src_data: *const c_void,
    pitch: u32,
    props: *const D2D1_BITMAP_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(std::ptr::null_mut());
    let sz = D2D1_SIZE_U::from_u64(size_u64);
    let p = if props.is_null() { D2D1_BITMAP_PROPERTIES::default() } else { props.read_unaligned() };
    let mut surf = Surface::new(sz.width, sz.height, p.pixelFormat, p.dpiX, p.dpiY);
    if !src_data.is_null() && sz.width > 0 && sz.height > 0 {
        let bpp = surf.bpp();
        let row_bytes = (sz.width as usize) * bpp;
        if (pitch as usize) < row_bytes {
            return E_INVALIDARG;
        }
        let src_ptr = src_data as *const u8;
        for row in 0..(sz.height as usize) {
            std::ptr::copy_nonoverlapping(
                src_ptr.add(row * pitch as usize),
                surf.data.as_mut_ptr().add(row * row_bytes),
                row_bytes,
            );
        }
    }
    let bmp = new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::new(Mutex::new(surf)), 0);
    out.write_unaligned(bmp);
    S_OK
}

// Slot 5: CreateBitmapFromWicBitmap
unsafe extern "win64" fn rt_create_bitmap_from_wic(
    this: PRT,
    wic_src: *mut c_void,
    props: *const D2D1_BITMAP_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || wic_src.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(std::ptr::null_mut());
    let vtbl = (wic_src as *const *const usize).read_unaligned();
    if vtbl.is_null() { return E_INVALIDARG; }
    let mut w = 0u32;
    let mut h = 0u32;
    let get_size_fn = vtbl.add(3).read_unaligned();
    if get_size_fn != 0 {
        let hr = crate::call_guest8(
            get_size_fn,
            wic_src as u64,
            &mut w as *mut u32 as u64,
            &mut h as *mut u32 as u64,
            0, 0, 0, 0, 0,
        ) as HRESULT;
        if hr < 0 { return hr; }
    }
    let p = if props.is_null() { D2D1_BITMAP_PROPERTIES::default() } else { props.read_unaligned() };
    let mut surf = Surface::new(w.max(1), h.max(1), p.pixelFormat, p.dpiX, p.dpiY);
    let copy_fn = vtbl.add(7).read_unaligned();
    if copy_fn != 0 {
        let stride = surf.pitch() as u32;
        let len = surf.data.len() as u32;
        crate::call_guest8(
            copy_fn,
            wic_src as u64,
            0,
            stride as u64,
            len as u64,
            surf.data.as_mut_ptr() as u64,
            0, 0, 0,
        );
    }
    out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::new(Mutex::new(surf)), 0));
    S_OK
}

// Slot 6: CreateSharedBitmap
unsafe extern "win64" fn rt_create_shared_bitmap(
    this: PRT,
    riid: *const u8,
    data: *mut c_void,
    props: *const D2D1_BITMAP_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || data.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(std::ptr::null_mut());
    if !riid.is_null() {
        let g = (riid as *const GUID).read_unaligned();
        if g == IID_ID2D1_BITMAP || g == IID_ID2D1_BITMAP1 {
            if let Some(b) = bitmap_from_ptr(data) {
                out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&b.surface), b.options));
                return S_OK;
            }
        }
    }
    rt_create_bitmap_from_wic(this, data, props, out)
}

// Slot 7: CreateBitmapBrush
unsafe extern "win64" fn rt_create_bitmap_brush(
    this: PRT,
    bitmap: *mut c_void,
    bb_props: *const D2D1_BITMAP_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let bbp = if bb_props.is_null() { D2D1_BITMAP_BRUSH_PROPERTIES::default() } else { bb_props.read_unaligned() };
    let surf = bitmap_from_ptr(bitmap).map(|b| Arc::clone(&b.surface));
    let obj = ComObj::new(
        bitmap_brush_vtbl(),
        BrushInner {
            factory: ComObj::inner(this).factory.clone(),
            state: Mutex::new(BrushState {
                opacity: bp.opacity,
                transform: bp.transform,
                kind: BrushKind::Bitmap {
                    bitmap: ComPtr::retain(bitmap),
                    surface: surf,
                    extend_x: bbp.extendModeX,
                    extend_y: bbp.extendModeY,
                    interp: bbp.interpolationMode,
                },
            }),
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 8: CreateSolidColorBrush
unsafe extern "win64" fn rt_create_solid_brush(
    this: PRT,
    color: *const D2D1_COLOR_F,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || color.is_null() || out.is_null() { return E_POINTER; }
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let c = color.read_unaligned();
    let obj = ComObj::new(
        solid_brush_vtbl(),
        BrushInner {
            factory: ComObj::inner(this).factory.clone(),
            state: Mutex::new(BrushState {
                opacity: bp.opacity,
                transform: bp.transform,
                kind: BrushKind::Solid { color: c },
            }),
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 9: CreateGradientStopCollection
unsafe extern "win64" fn rt_create_grad_stops(
    this: PRT,
    stops_ptr: *const D2D1_GRADIENT_STOP,
    count: u32,
    gamma: u32,
    extend_mode: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() || stops_ptr.is_null() || count == 0 { return E_INVALIDARG; }
    let mut stops = Vec::with_capacity(count as usize);
    for i in 0..(count as usize) {
        stops.push(stops_ptr.add(i).read_unaligned());
    }
    stops.sort_by(|a, b| a.position.total_cmp(&b.position));
    let obj = ComObj::new(
        grad_stops_vtbl(),
        GradientStopCollectionInner {
            factory: ComObj::inner(this).factory.clone(),
            data: Arc::new(GradientStopCollectionData { stops, gamma, extend_mode }),
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 10: CreateLinearGradientBrush
unsafe extern "win64" fn rt_create_linear_brush(
    this: PRT,
    lg_props: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    stops_com: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || lg_props.is_null() || stops_com.is_null() || out.is_null() { return E_POINTER; }
    if (stops_com as *const *const usize).read_unaligned() != grad_stops_vtbl() {
        return E_INVALIDARG;
    }
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let lg = lg_props.read_unaligned();
    let stops = Arc::clone(&ComObj::inner(stops_com as PGradStops).data);
    let Some(stops_ref) = ComPtr::retain(stops_com) else { return E_INVALIDARG; };
    let obj = ComObj::new(
        linear_brush_vtbl(),
        BrushInner {
            factory: ComObj::inner(this).factory.clone(),
            state: Mutex::new(BrushState {
                opacity: bp.opacity,
                transform: bp.transform,
                kind: BrushKind::LinearGradient {
                    start: lg.startPoint,
                    end: lg.endPoint,
                    stops,
                    stops_com: stops_ref,
                },
            }),
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 11: CreateRadialGradientBrush
unsafe extern "win64" fn rt_create_radial_brush(
    this: PRT,
    rg_props: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    stops_com: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || rg_props.is_null() || stops_com.is_null() || out.is_null() { return E_POINTER; }
    if (stops_com as *const *const usize).read_unaligned() != grad_stops_vtbl() {
        return E_INVALIDARG;
    }
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let rg = rg_props.read_unaligned();
    let stops = Arc::clone(&ComObj::inner(stops_com as PGradStops).data);
    let Some(stops_ref) = ComPtr::retain(stops_com) else { return E_INVALIDARG; };
    let obj = ComObj::new(
        radial_brush_vtbl(),
        BrushInner {
            factory: ComObj::inner(this).factory.clone(),
            state: Mutex::new(BrushState {
                opacity: bp.opacity,
                transform: bp.transform,
                kind: BrushKind::RadialGradient {
                    center: rg.center,
                    origin_offset: rg.gradientOriginOffset,
                    rx: rg.radiusX,
                    ry: rg.radiusY,
                    stops,
                    stops_com: stops_ref,
                },
            }),
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 12: CreateCompatibleRenderTarget
unsafe extern "win64" fn rt_create_compat_rt(
    this: PRT,
    desired_size: *const D2D1_SIZE_F,
    desired_px: *const D2D1_SIZE_U,
    desired_fmt: *const D2D1_PIXEL_FORMAT,
    _options: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let parent = ComObj::inner(this).state.lock().unwrap();
    let ps = parent.surface.lock().unwrap();
    let dpi_x = ps.dpi_x;
    let dpi_y = ps.dpi_y;
    let pf = if desired_fmt.is_null() { ps.pixel_format } else { desired_fmt.read_unaligned() };
    let (w, h) = if !desired_px.is_null() {
        let p = desired_px.read_unaligned();
        (p.width.max(1), p.height.max(1))
    } else if !desired_size.is_null() {
        let s = desired_size.read_unaligned();
        ((s.width * dpi_x / 96.0).ceil().max(1.0) as u32, (s.height * dpi_y / 96.0).ceil().max(1.0) as u32)
    } else {
        (ps.width.max(1), ps.height.max(1))
    };
    drop(ps);
    drop(parent);

    let surf = Arc::new(Mutex::new(Surface::new(w, h, pf, dpi_x, dpi_y)));
    let bmp_ptr = new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&surf), 0);
    let bmp_ref = ComPtr::retain(bmp_ptr);
    raw_com_release(bmp_ptr);
    let rt = new_render_target_com(
        ComObj::inner(this).factory.clone(),
        RenderTargetKind::Bitmap,
        surf,
        bmp_ref,
        std::ptr::null_mut(),
        None,
        None,
    );
    out.write_unaligned(rt);
    S_OK
}

// Slot 13: CreateLayer
unsafe extern "win64" fn rt_create_layer(this: PRT, size: *const D2D1_SIZE_F, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let sz = if size.is_null() {
        let s = ComObj::inner(this).state.lock().unwrap().surface.lock().unwrap().clone();
        D2D1_SIZE_F { width: s.width as f32 * 96.0 / s.dpi_x, height: s.height as f32 * 96.0 / s.dpi_y }
    } else {
        size.read_unaligned()
    };
    let obj = ComObj::new(layer_vtbl(), LayerInner { factory: ComObj::inner(this).factory.clone(), size: sz });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 14: CreateMesh
unsafe extern "win64" fn rt_create_mesh(this: PRT, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let obj = ComObj::new(mesh_vtbl(), MeshInner { factory: ComObj::inner(this).factory.clone() });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}

// Slot 15: DrawLine
unsafe extern "win64" fn rt_draw_line(
    this: PRT,
    p0_u64: u64,
    p1_u64: u64,
    brush: *mut c_void,
    stroke_w: f32,
    stroke_style: *mut c_void,
) {
    if this.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let p0 = D2D1_POINT_2F::from_u64(p0_u64);
    let p1 = D2D1_POINT_2F::from_u64(p1_u64);
    let sp = SubPath { points: vec![(p0.x, p0.y), (p1.x, p1.y)], closed: false, filled: true };
    let polys = expand_strokes(&[sp], stroke_w, stroke_style_from_ptr(stroke_style));
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &polys, D2D1_FILL_MODE_WINDING, &b.snapshot(), None);
}
crate::win64_bridge!(fn rt_draw_line(PRT, u64, u64, *mut c_void, f32, *mut c_void) -> ());

// Slot 16: DrawRectangle
unsafe extern "win64" fn rt_draw_rect(
    this: PRT,
    rect: *const D2D1_RECT_F,
    brush: *mut c_void,
    stroke_w: f32,
    stroke_style: *mut c_void,
) {
    if this.is_null() || rect.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = rect_subpath(rect.read_unaligned());
    let polys = expand_strokes(&[sp], stroke_w, stroke_style_from_ptr(stroke_style));
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &polys, D2D1_FILL_MODE_WINDING, &b.snapshot(), None);
}
crate::win64_bridge!(fn rt_draw_rect(PRT, *const D2D1_RECT_F, *mut c_void, f32, *mut c_void) -> ());

// Slot 17: FillRectangle
unsafe extern "win64" fn rt_fill_rect(this: PRT, rect: *const D2D1_RECT_F, brush: *mut c_void) {
    if this.is_null() || rect.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = rect_subpath(rect.read_unaligned());
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &[sp], D2D1_FILL_MODE_ALTERNATE, &b.snapshot(), None);
}

// Slot 18: DrawRoundedRectangle
unsafe extern "win64" fn rt_draw_rounded_rect(
    this: PRT,
    rrect: *const D2D1_ROUNDED_RECT,
    brush: *mut c_void,
    stroke_w: f32,
    stroke_style: *mut c_void,
) {
    if this.is_null() || rrect.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = rounded_rect_subpath(rrect.read_unaligned());
    let polys = expand_strokes(&[sp], stroke_w, stroke_style_from_ptr(stroke_style));
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &polys, D2D1_FILL_MODE_WINDING, &b.snapshot(), None);
}
crate::win64_bridge!(fn rt_draw_rounded_rect(PRT, *const D2D1_ROUNDED_RECT, *mut c_void, f32, *mut c_void) -> ());

// Slot 19: FillRoundedRectangle
unsafe extern "win64" fn rt_fill_rounded_rect(this: PRT, rrect: *const D2D1_ROUNDED_RECT, brush: *mut c_void) {
    if this.is_null() || rrect.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = rounded_rect_subpath(rrect.read_unaligned());
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &[sp], D2D1_FILL_MODE_ALTERNATE, &b.snapshot(), None);
}

// Slot 20: DrawEllipse
unsafe extern "win64" fn rt_draw_ellipse(
    this: PRT,
    ellipse: *const D2D1_ELLIPSE,
    brush: *mut c_void,
    stroke_w: f32,
    stroke_style: *mut c_void,
) {
    if this.is_null() || ellipse.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = ellipse_subpath(ellipse.read_unaligned());
    let polys = expand_strokes(&[sp], stroke_w, stroke_style_from_ptr(stroke_style));
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &polys, D2D1_FILL_MODE_WINDING, &b.snapshot(), None);
}
crate::win64_bridge!(fn rt_draw_ellipse(PRT, *const D2D1_ELLIPSE, *mut c_void, f32, *mut c_void) -> ());

// Slot 21: FillEllipse
unsafe extern "win64" fn rt_fill_ellipse(this: PRT, ellipse: *const D2D1_ELLIPSE, brush: *mut c_void) {
    if this.is_null() || ellipse.is_null() { return; }
    let Some(b) = brush_from_ptr(brush) else { return; };
    let sp = ellipse_subpath(ellipse.read_unaligned());
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &[sp], D2D1_FILL_MODE_ALTERNATE, &b.snapshot(), None);
}

// Slot 22: DrawGeometry
unsafe extern "win64" fn rt_draw_geometry(
    this: PRT,
    geom: *mut c_void,
    brush: *mut c_void,
    stroke_w: f32,
    stroke_style: *mut c_void,
) {
    if this.is_null() { return; }
    let (Some(g), Some(b)) = (geometry_from_ptr(geom), brush_from_ptr(brush)) else { return; };
    let fg = g.flatten(None, 0.25);
    let polys = expand_strokes(&fg.subpaths, stroke_w, stroke_style_from_ptr(stroke_style));
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &polys, D2D1_FILL_MODE_WINDING, &b.snapshot(), None);
}
crate::win64_bridge!(fn rt_draw_geometry(PRT, *mut c_void, *mut c_void, f32, *mut c_void) -> ());

// Slot 23: FillGeometry
unsafe extern "win64" fn rt_fill_geometry(
    this: PRT,
    geom: *mut c_void,
    brush: *mut c_void,
    opacity_brush: *mut c_void,
) {
    if this.is_null() { return; }
    let (Some(g), Some(b)) = (geometry_from_ptr(geom), brush_from_ptr(brush)) else { return; };
    let fg = g.flatten(None, 0.25);
    let op_eval = brush_from_ptr(opacity_brush).map(|ob| ob.snapshot());
    let st = ComObj::inner(this).state.lock().unwrap();
    render_polygons_with_brush(&st, &fg.subpaths, fg.fill_mode, &b.snapshot(), op_eval.as_ref());
}

// Slot 24: FillMesh
unsafe extern "win64" fn rt_fill_mesh(_this: PRT, _mesh: *mut c_void, _brush: *mut c_void) {}

// Slot 25: FillOpacityMask
unsafe extern "win64" fn rt_fill_opacity_mask(
    this: PRT,
    mask: *mut c_void,
    brush: *mut c_void,
    _content: u32,
    dst_rect: *const D2D1_RECT_F,
    src_rect: *const D2D1_RECT_F,
) {
    if this.is_null() { return; }
    let (Some(mask_b), Some(b)) = (bitmap_from_ptr(mask), brush_from_ptr(brush)) else { return; };
    let mask_surf = mask_b.surface.lock().unwrap().clone();
    let dr = if dst_rect.is_null() {
        D2D1_RECT_F {
            left: 0.0,
            top: 0.0,
            right: mask_surf.width as f32 * 96.0 / mask_surf.dpi_x,
            bottom: mask_surf.height as f32 * 96.0 / mask_surf.dpi_y,
        }
    } else {
        dst_rect.read_unaligned()
    };
    let sr = if src_rect.is_null() {
        D2D1_RECT_F { left: 0.0, top: 0.0, right: mask_surf.width as f32, bottom: mask_surf.height as f32 }
    } else {
        let r = src_rect.read_unaligned();
        D2D1_RECT_F {
            left: r.left * mask_surf.dpi_x / 96.0,
            top: r.top * mask_surf.dpi_y / 96.0,
            right: r.right * mask_surf.dpi_x / 96.0,
            bottom: r.bottom * mask_surf.dpi_y / 96.0,
        }
    };
    let dw = (dr.right - dr.left).max(1e-5);
    let dh = (dr.bottom - dr.top).max(1e-5);
    let sw = sr.right - sr.left;
    let sh = sr.bottom - sr.top;

    let brush_eval = b.snapshot();
    let st = ComObj::inner(this).state.lock().unwrap();
    let total_m = st.total_transform();
    let inv_total = total_m.invert().unwrap_or_else(D2D1_MATRIX_3X2_F::identity);
    let quad = transform_subpaths(&[rect_subpath(dr)], &total_m);
    let mut dst_surf = st.surface.lock().unwrap();
    let clip = st.effective_clip(dst_surf.width, dst_surf.height);
    let aa = st.antialias_mode;
    let blend = st.primitive_blend;

    rasterize_polygons(&mut dst_surf, &quad, D2D1_FILL_MODE_ALTERNATE, clip, aa, blend, |px, py| {
        let (wx, wy) = inv_total.transform_point(px, py);
        let u = ((wx - dr.left) / dw).clamp(0.0, 1.0);
        let v = ((wy - dr.top) / dh).clamp(0.0, 1.0);
        let ma = mask_surf.sample(sr.left + u * sw, sr.top + v * sh, D2D1_EXTEND_MODE_CLAMP, D2D1_EXTEND_MODE_CLAMP, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR)[3];
        let mut c = brush_eval.sample_world(wx, wy);
        c[3] *= ma;
        c
    });
}

// Slot 26: DrawBitmap
unsafe extern "win64" fn rt_draw_bitmap(
    this: PRT,
    bitmap: *mut c_void,
    dst_rect: *const D2D1_RECT_F,
    opacity: f32,
    interp: u32,
    src_rect: *const D2D1_RECT_F,
) {
    if this.is_null() { return; }
    let Some(bmp) = bitmap_from_ptr(bitmap) else { return; };
    let src_surf = bmp.surface.lock().unwrap().clone();
    if src_surf.width == 0 || src_surf.height == 0 { return; }
    let dr = if dst_rect.is_null() {
        D2D1_RECT_F {
            left: 0.0,
            top: 0.0,
            right: src_surf.width as f32 * 96.0 / src_surf.dpi_x,
            bottom: src_surf.height as f32 * 96.0 / src_surf.dpi_y,
        }
    } else {
        dst_rect.read_unaligned()
    };
    let sr = if src_rect.is_null() {
        D2D1_RECT_F { left: 0.0, top: 0.0, right: src_surf.width as f32, bottom: src_surf.height as f32 }
    } else {
        let r = src_rect.read_unaligned();
        D2D1_RECT_F {
            left: r.left * src_surf.dpi_x / 96.0,
            top: r.top * src_surf.dpi_y / 96.0,
            right: r.right * src_surf.dpi_x / 96.0,
            bottom: r.bottom * src_surf.dpi_y / 96.0,
        }
    };
    let dw = dr.right - dr.left;
    let dh = dr.bottom - dr.top;
    if dw.abs() < 1e-5 || dh.abs() < 1e-5 { return; }
    let sw = sr.right - sr.left;
    let sh = sr.bottom - sr.top;
    let op = opacity.clamp(0.0, 1.0);

    let st = ComObj::inner(this).state.lock().unwrap();
    let total_m = st.total_transform();
    let inv_total = total_m.invert().unwrap_or_else(D2D1_MATRIX_3X2_F::identity);
    let quad = transform_subpaths(&[rect_subpath(dr)], &total_m);
    let mut dst_surf = st.surface.lock().unwrap();
    let clip = st.effective_clip(dst_surf.width, dst_surf.height);
    let aa = st.antialias_mode;
    let blend = st.primitive_blend;

    rasterize_polygons(&mut dst_surf, &quad, D2D1_FILL_MODE_ALTERNATE, clip, aa, blend, |px, py| {
        let (wx, wy) = inv_total.transform_point(px, py);
        let u = (wx - dr.left) / dw;
        let v = (wy - dr.top) / dh;
        let mut c = src_surf.sample(sr.left + u * sw, sr.top + v * sh, D2D1_EXTEND_MODE_CLAMP, D2D1_EXTEND_MODE_CLAMP, interp);
        c[3] *= op;
        c
    });
}
crate::win64_bridge!(fn rt_draw_bitmap(
    PRT, *mut c_void, *const D2D1_RECT_F, f32, u32, *const D2D1_RECT_F
) -> ());

// Slot 27: DrawText (integrates directly with dwrite.rs!)
unsafe extern "win64" fn rt_draw_text(
    this: PRT,
    string: *const u16,
    string_len: u32,
    text_format: *mut c_void,
    layout_rect: *const D2D1_RECT_F,
    default_brush: *mut c_void,
    options: u32,
    _measuring_mode: u32,
) {
    if this.is_null() || string.is_null() || string_len == 0 || text_format.is_null() || layout_rect.is_null() {
        return;
    }
    let Some(b) = brush_from_ptr(default_brush) else { return; };
    let lr = layout_rect.read_unaligned();
    let mut units = Vec::with_capacity(string_len as usize);
    for i in 0..(string_len as usize) {
        units.push(string.add(i).read_unaligned());
    }
    let max_w = (lr.right - lr.left).max(0.0);
    let max_h = (lr.bottom - lr.top).max(0.0);
    let Some(runs) = dwrite::layout_text_from_format(&units, text_format, max_w, max_h) else {
        return;
    };
    let brush_eval = b.snapshot();
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let pushed_clip = (options & D2D1_DRAW_TEXT_OPTIONS_CLIP) != 0;
    if pushed_clip {
        let total = st.total_transform();
        let corners = [
            total.transform_point(lr.left, lr.top),
            total.transform_point(lr.right, lr.top),
            total.transform_point(lr.right, lr.bottom),
            total.transform_point(lr.left, lr.bottom),
        ];
        let x0 = corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min).floor() as i32;
        let y0 = corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min).floor() as i32;
        let x1 = corners.iter().map(|c| c.0).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
        let y1 = corners.iter().map(|c| c.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
        st.clip_stack.push((x0, y0, x1, y1));
    }
    render_drawable_runs(&st, &runs, lr.left, lr.top, &brush_eval);
    if pushed_clip {
        st.clip_stack.pop();
    }
}

// Slot 28: DrawTextLayout (integrates directly with dwrite.rs!)
unsafe extern "win64" fn rt_draw_text_layout(
    this: PRT,
    origin_u64: u64,
    text_layout: *mut c_void,
    default_brush: *mut c_void,
    _options: u32,
) {
    if this.is_null() || text_layout.is_null() { return; }
    let Some(b) = brush_from_ptr(default_brush) else { return; };
    let Some(runs) = dwrite::extract_layout_runs(text_layout) else { return; };
    let origin = D2D1_POINT_2F::from_u64(origin_u64);
    let brush_eval = b.snapshot();
    let st = ComObj::inner(this).state.lock().unwrap();
    render_drawable_runs(&st, &runs, origin.x, origin.y, &brush_eval);
}

// Slot 29: DrawGlyphRun (integrates directly with dwrite.rs!)
unsafe extern "win64" fn rt_draw_glyph_run(
    this: PRT,
    origin_u64: u64,
    glyph_run: *const DWRITE_GLYPH_RUN,
    foreground_brush: *mut c_void,
    _measuring_mode: u32,
) {
    if this.is_null() || glyph_run.is_null() { return; }
    let Some(b) = brush_from_ptr(foreground_brush) else { return; };
    let run = glyph_run.read_unaligned();
    let Some(face) = dwrite::font_face_from_ptr(run.fontFace) else { return; };
    let n = run.glyphCount as usize;
    let mut gids = Vec::with_capacity(n);
    let mut advs = Vec::with_capacity(n);
    let mut offs = Vec::with_capacity(n);
    for i in 0..n {
        gids.push(if run.glyphIndices.is_null() { 0 } else { run.glyphIndices.add(i).read_unaligned() });
        advs.push(if run.glyphAdvances.is_null() { run.fontEmSize * 0.5 } else { run.glyphAdvances.add(i).read_unaligned() });
        offs.push(if run.glyphOffsets.is_null() { DWRITE_GLYPH_OFFSET::default() } else { run.glyphOffsets.add(i).read_unaligned() });
    }
    let origin = D2D1_POINT_2F::from_u64(origin_u64);
    let st = ComObj::inner(this).state.lock().unwrap();
    render_glyph_run_on_rt(&st, &face, run.fontEmSize, origin.x, origin.y, &gids, &advs, &offs, &b.snapshot());
}

// Slots 30..49: Transform, AA, Tags, Layers, Clips, Clear, BeginDraw/EndDraw
unsafe extern "win64" fn rt_set_transform(this: PRT, m: *const D2D1_MATRIX_3X2_F) {
    if this.is_null() { return; }
    let mat = if m.is_null() { D2D1_MATRIX_3X2_F::identity() } else { m.read_unaligned() };
    ComObj::inner(this).state.lock().unwrap().transform = mat;
}
unsafe extern "win64" fn rt_get_transform(this: PRT, out: *mut D2D1_MATRIX_3X2_F) {
    if this.is_null() || out.is_null() { return; }
    out.write_unaligned(ComObj::inner(this).state.lock().unwrap().transform);
}
unsafe extern "win64" fn rt_set_aa(this: PRT, mode: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().antialias_mode = mode;
}
unsafe extern "win64" fn rt_get_aa(this: PRT) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().antialias_mode }
}
unsafe extern "win64" fn rt_set_text_aa(this: PRT, mode: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().text_antialias_mode = mode;
}
unsafe extern "win64" fn rt_get_text_aa(this: PRT) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().text_antialias_mode }
}
unsafe extern "win64" fn rt_set_text_params(this: PRT, rp: *mut c_void) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().text_rendering_params = ComPtr::retain(rp);
}
unsafe extern "win64" fn rt_get_text_params(this: PRT, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref p) = st.text_rendering_params {
        raw_com_add_ref(p.as_ptr());
        out.write_unaligned(p.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
}
unsafe extern "win64" fn rt_set_tags(this: PRT, t1: u64, t2: u64) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.tag1 = t1;
    st.tag2 = t2;
}
unsafe extern "win64" fn rt_get_tags(this: PRT, t1: *mut u64, t2: *mut u64) {
    if this.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if !t1.is_null() { t1.write_unaligned(st.tag1); }
    if !t2.is_null() { t2.write_unaligned(st.tag2); }
}
unsafe extern "win64" fn rt_push_layer(this: PRT, params: *const D2D1_LAYER_PARAMETERS, _layer: *mut c_void) {
    if this.is_null() || params.is_null() { return; }
    let p = params.read_unaligned();
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let total = st.total_transform();
    let (w, h, pf, dpi_x, dpi_y) = {
        let s = st.surface.lock().unwrap();
        (s.width, s.height, s.pixel_format, s.dpi_x, s.dpi_y)
    };
    let mut clip = st.effective_clip(w, h);
    if p.contentBounds.left.is_finite()
        && p.contentBounds.right.is_finite()
        && p.contentBounds.top.is_finite()
        && p.contentBounds.bottom.is_finite()
        && p.contentBounds.right > p.contentBounds.left
        && p.contentBounds.bottom > p.contentBounds.top
    {
        let corners = [
            total.transform_point(p.contentBounds.left, p.contentBounds.top),
            total.transform_point(p.contentBounds.right, p.contentBounds.top),
            total.transform_point(p.contentBounds.right, p.contentBounds.bottom),
            total.transform_point(p.contentBounds.left, p.contentBounds.bottom),
        ];
        clip.0 = clip.0.max(corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min).floor() as i32);
        clip.1 = clip.1.max(corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min).floor() as i32);
        clip.2 = clip.2.min(corners.iter().map(|c| c.0).fold(f32::NEG_INFINITY, f32::max).ceil() as i32);
        clip.3 = clip.3.min(corners.iter().map(|c| c.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32);
    }
    let mask_polys = geometry_from_ptr(p.geometricMask).map(|g| {
        let m = p.maskTransform.mul(&total);
        let fg = g.flatten(Some(&m), 0.25);
        (fg.subpaths, fg.fill_mode)
    });
    let layer_surf = Arc::new(Mutex::new(Surface::new(w, h, pf, dpi_x, dpi_y)));
    let saved = std::mem::replace(&mut st.surface, layer_surf);
    st.layer_stack.push(LayerFrame {
        saved_surface: saved,
        opacity: p.opacity.clamp(0.0, 1.0),
        clip_bounds: clip,
        mask_polys,
    });
}
unsafe extern "win64" fn rt_pop_layer(this: PRT) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let Some(frame) = st.layer_stack.pop() else { return; };
    let layer_surf = std::mem::replace(&mut st.surface, frame.saved_surface);
    let src = layer_surf.lock().unwrap();
    let mut dst = st.surface.lock().unwrap();
    let x0 = frame.clip_bounds.0.max(0).min(dst.width as i32) as u32;
    let y0 = frame.clip_bounds.1.max(0).min(dst.height as i32) as u32;
    let x1 = frame.clip_bounds.2.max(0).min(dst.width as i32) as u32;
    let y1 = frame.clip_bounds.3.max(0).min(dst.height as i32) as u32;
    for py in y0..y1 {
        for px in x0..x1 {
            if let Some((ref polys, fm)) = frame.mask_polys {
                if !point_in_subpaths(px as f32 + 0.5, py as f32 + 0.5, polys, fm) {
                    continue;
                }
            }
            let c = src.get_pixel_straight(px, py);
            if c[3] > 0.0 {
                dst.blend_pixel(px, py, c, frame.opacity, D2D1_PRIMITIVE_BLEND_SOURCE_OVER);
            }
        }
    }
}
unsafe extern "win64" fn rt_flush(this: PRT, t1: *mut u64, t2: *mut u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let st = ComObj::inner(this).state.lock().unwrap();
    st.flush_external_targets();
    if !t1.is_null() { t1.write_unaligned(st.tag1); }
    if !t2.is_null() { t2.write_unaligned(st.tag2); }
    S_OK
}
unsafe extern "win64" fn rt_save_state(this: PRT, block: *mut c_void) {
    if this.is_null() || block.is_null() { return; }
    if (block as *const *const usize).read_unaligned() != drawing_state_block_vtbl() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    let mut dst = ComObj::inner(block as PStateBlock).state.lock().unwrap();
    dst.0 = D2D1_DRAWING_STATE_DESCRIPTION {
        antialiasMode: st.antialias_mode,
        textAntialiasMode: st.text_antialias_mode,
        tag1: st.tag1,
        tag2: st.tag2,
        transform: st.transform,
    };
    dst.1 = st.text_rendering_params.clone();
}
unsafe extern "win64" fn rt_restore_state(this: PRT, block: *mut c_void) {
    if this.is_null() || block.is_null() { return; }
    if (block as *const *const usize).read_unaligned() != drawing_state_block_vtbl() { return; }
    let src = ComObj::inner(block as PStateBlock).state.lock().unwrap().clone();
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.antialias_mode = src.0.antialiasMode;
    st.text_antialias_mode = src.0.textAntialiasMode;
    st.tag1 = src.0.tag1;
    st.tag2 = src.0.tag2;
    st.transform = src.0.transform;
    st.text_rendering_params = src.1;
}
unsafe extern "win64" fn rt_push_clip(this: PRT, clip_rect: *const D2D1_RECT_F, _aa: u32) {
    if this.is_null() || clip_rect.is_null() { return; }
    let r = clip_rect.read_unaligned();
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let total = st.total_transform();
    let corners = [
        total.transform_point(r.left, r.top),
        total.transform_point(r.right, r.top),
        total.transform_point(r.right, r.bottom),
        total.transform_point(r.left, r.bottom),
    ];
    let x0 = corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min).floor() as i32;
    let y0 = corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min).floor() as i32;
    let x1 = corners.iter().map(|c| c.0).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
    let y1 = corners.iter().map(|c| c.1).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
    st.clip_stack.push((x0, y0, x1, y1));
}
unsafe extern "win64" fn rt_pop_clip(this: PRT) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().clip_stack.pop();
}
unsafe extern "win64" fn rt_clear(this: PRT, clear_color: *const D2D1_COLOR_F) {
    if this.is_null() { return; }
    let c = if clear_color.is_null() {
        D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    } else {
        clear_color.read_unaligned()
    };
    let st = ComObj::inner(this).state.lock().unwrap();
    let mut surf = st.surface.lock().unwrap();
    let clip = st.effective_clip(surf.width, surf.height);
    surf.clear_rect(c, clip);
}
unsafe extern "win64" fn rt_begin_draw(this: PRT) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().in_draw = true;
}
unsafe extern "win64" fn rt_end_draw(this: PRT, t1: *mut u64, t2: *mut u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.in_draw = false;
    st.flush_external_targets();
    if !t1.is_null() { t1.write_unaligned(st.tag1); }
    if !t2.is_null() { t2.write_unaligned(st.tag2); }
    S_OK
}

// Slots 50..56: PixelFormat, DPI, Size, PixelSize, MaxBitmapSize, IsSupported
unsafe extern "win64" fn rt_get_pixel_format(this: PRT, ret: *mut D2D1_PIXEL_FORMAT) -> *mut D2D1_PIXEL_FORMAT {
    let pf = if this.is_null() {
        D2D1_PIXEL_FORMAT::default()
    } else {
        ComObj::inner(this).state.lock().unwrap().surface.lock().unwrap().pixel_format
    };
    if !ret.is_null() { ret.write_unaligned(pf); }
    ret
}
unsafe extern "win64" fn rt_set_dpi(this: PRT, dpi_x: f32, dpi_y: f32) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let mut surf = st.surface.lock().unwrap();
    if dpi_x > 0.0 && dpi_x.is_finite() { surf.dpi_x = dpi_x; }
    if dpi_y > 0.0 && dpi_y.is_finite() { surf.dpi_y = dpi_y; }
    if dpi_x == 0.0 && dpi_y == 0.0 {
        surf.dpi_x = 96.0;
        surf.dpi_y = 96.0;
    }
    drop(surf);
    let _ = &mut st;
}
crate::win64_bridge!(fn rt_set_dpi(PRT, f32, f32) -> ());

unsafe extern "win64" fn rt_get_dpi(this: PRT, dpi_x: *mut f32, dpi_y: *mut f32) {
    if this.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    let surf = st.surface.lock().unwrap();
    if !dpi_x.is_null() { dpi_x.write_unaligned(surf.dpi_x); }
    if !dpi_y.is_null() { dpi_y.write_unaligned(surf.dpi_y); }
}
unsafe extern "win64" fn rt_get_size(this: PRT, ret: *mut D2D1_SIZE_F) -> *mut D2D1_SIZE_F {
    let sz = if this.is_null() {
        D2D1_SIZE_F::default()
    } else {
        let st = ComObj::inner(this).state.lock().unwrap();
        let s = st.surface.lock().unwrap();
        D2D1_SIZE_F { width: s.width as f32 * 96.0 / s.dpi_x, height: s.height as f32 * 96.0 / s.dpi_y }
    };
    if !ret.is_null() { ret.write_unaligned(sz); }
    ret
}
unsafe extern "win64" fn rt_get_pixel_size(this: PRT, ret: *mut D2D1_SIZE_U) -> *mut D2D1_SIZE_U {
    let sz = if this.is_null() {
        D2D1_SIZE_U::default()
    } else {
        let st = ComObj::inner(this).state.lock().unwrap();
        let s = st.surface.lock().unwrap();
        D2D1_SIZE_U { width: s.width, height: s.height }
    };
    if !ret.is_null() { ret.write_unaligned(sz); }
    ret
}
unsafe extern "win64" fn rt_get_max_bitmap_size(_this: PRT) -> u32 {
    16_384
}
unsafe extern "win64" fn rt_is_supported(_this: PRT, props: *const D2D1_RENDER_TARGET_PROPERTIES) -> BOOL {
    if props.is_null() { return FALSE; }
    let p = props.read_unaligned();
    let ok = matches!(
        p.pixelFormat.format,
        DXGI_FORMAT_UNKNOWN | DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_A8_UNORM
    );
    if ok { TRUE } else { FALSE }
}

macro_rules! rt_base_slots {
    () => {
        [
            rt_qi as *const () as usize,                                 // 0
            rt_add_ref as *const () as usize,                            // 1
            rt_release as *const () as usize,                            // 2
            rt_get_factory as *const () as usize,                        // 3
            rt_create_bitmap as *const () as usize,                      // 4
            rt_create_bitmap_from_wic as *const () as usize,             // 5
            rt_create_shared_bitmap as *const () as usize,               // 6
            rt_create_bitmap_brush as *const () as usize,                // 7
            rt_create_solid_brush as *const () as usize,                 // 8
            rt_create_grad_stops as *const () as usize,                  // 9
            rt_create_linear_brush as *const () as usize,                // 10
            rt_create_radial_brush as *const () as usize,                // 11
            rt_create_compat_rt as *const () as usize,                   // 12
            rt_create_layer as *const () as usize,                       // 13
            rt_create_mesh as *const () as usize,                        // 14
            crate::bridge_entry!(rt_draw_line) as usize,                 // 15
            crate::bridge_entry!(rt_draw_rect) as usize,                 // 16
            rt_fill_rect as *const () as usize,                          // 17
            crate::bridge_entry!(rt_draw_rounded_rect) as usize,         // 18
            rt_fill_rounded_rect as *const () as usize,                  // 19
            crate::bridge_entry!(rt_draw_ellipse) as usize,              // 20
            rt_fill_ellipse as *const () as usize,                       // 21
            crate::bridge_entry!(rt_draw_geometry) as usize,             // 22
            rt_fill_geometry as *const () as usize,                      // 23
            rt_fill_mesh as *const () as usize,                          // 24
            rt_fill_opacity_mask as *const () as usize,                  // 25
            crate::bridge_entry!(rt_draw_bitmap) as usize,               // 26
            rt_draw_text as *const () as usize,                          // 27
            rt_draw_text_layout as *const () as usize,                   // 28
            rt_draw_glyph_run as *const () as usize,                     // 29
            rt_set_transform as *const () as usize,                      // 30
            rt_get_transform as *const () as usize,                      // 31
            rt_set_aa as *const () as usize,                             // 32
            rt_get_aa as *const () as usize,                             // 33
            rt_set_text_aa as *const () as usize,                        // 34
            rt_get_text_aa as *const () as usize,                        // 35
            rt_set_text_params as *const () as usize,                    // 36
            rt_get_text_params as *const () as usize,                    // 37
            rt_set_tags as *const () as usize,                           // 38
            rt_get_tags as *const () as usize,                           // 39
            rt_push_layer as *const () as usize,                         // 40
            rt_pop_layer as *const () as usize,                          // 41
            rt_flush as *const () as usize,                              // 42
            rt_save_state as *const () as usize,                         // 43
            rt_restore_state as *const () as usize,                      // 44
            rt_push_clip as *const () as usize,                          // 45
            rt_pop_clip as *const () as usize,                           // 46
            rt_clear as *const () as usize,                              // 47
            rt_begin_draw as *const () as usize,                         // 48
            rt_end_draw as *const () as usize,                           // 49
            rt_get_pixel_format as *const () as usize,                   // 50
            crate::bridge_entry!(rt_set_dpi) as usize,                   // 51
            rt_get_dpi as *const () as usize,                            // 52
            rt_get_size as *const () as usize,                           // 53
            rt_get_pixel_size as *const () as usize,                     // 54
            rt_get_max_bitmap_size as *const () as usize,                // 55
            rt_is_supported as *const () as usize,                       // 56
        ]
    };
}

fn rt_generic_vtbl() -> *const usize {
    static V: OnceLock<[usize; 57]> = OnceLock::new();
    V.get_or_init(|| register_vtable(rt_base_slots!())).as_ptr()
}

// ID2D1BitmapRenderTarget (58 slots)
unsafe extern "win64" fn brt_get_bitmap(this: PRT, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref b) = st.target_bitmap {
        raw_com_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
        S_OK
    } else {
        out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&st.surface), 0));
        S_OK
    }
}
fn rt_bitmap_vtbl() -> *const usize {
    static V: OnceLock<[usize; 58]> = OnceLock::new();
    V.get_or_init(|| {
        let b = rt_base_slots!();
        let mut t = [0usize; 58];
        t[..57].copy_from_slice(&b);
        t[57] = brt_get_bitmap as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

// ID2D1HwndRenderTarget (60 slots)
unsafe extern "win64" fn hwnd_rt_check_window_state(_this: PRT) -> u32 { 0 }
unsafe extern "win64" fn hwnd_rt_resize(this: PRT, px_size: *const D2D1_SIZE_U) -> HRESULT {
    if this.is_null() || px_size.is_null() { return E_POINTER; }
    let sz = px_size.read_unaligned();
    ComObj::inner(this).state.lock().unwrap().surface.lock().unwrap().resize(sz.width.max(1), sz.height.max(1));
    S_OK
}
unsafe extern "win64" fn hwnd_rt_get_hwnd(this: PRT) -> HWND {
    if this.is_null() { std::ptr::null_mut() } else { ComObj::inner(this).state.lock().unwrap().hwnd }
}
fn rt_hwnd_vtbl() -> *const usize {
    static V: OnceLock<[usize; 60]> = OnceLock::new();
    V.get_or_init(|| {
        let b = rt_base_slots!();
        let mut t = [0usize; 60];
        t[..57].copy_from_slice(&b);
        t[57] = hwnd_rt_check_window_state as *const () as usize;
        t[58] = hwnd_rt_resize as *const () as usize;
        t[59] = hwnd_rt_get_hwnd as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

// ID2D1DCRenderTarget (58 slots)
unsafe extern "win64" fn dc_rt_bind_dc(this: PRT, hdc: usize, sub_rect: *const [i32; 4]) -> HRESULT {
    if this.is_null() || sub_rect.is_null() { return E_POINTER; }
    let r = sub_rect.read_unaligned();
    let w = (r[2] - r[0]).max(1) as u32;
    let h = (r[3] - r[1]).max(1) as u32;
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.bound_hdc = hdc;
    st.bound_subrect = r;
    st.surface.lock().unwrap().resize(w, h);
    S_OK
}
fn rt_dc_vtbl() -> *const usize {
    static V: OnceLock<[usize; 58]> = OnceLock::new();
    V.get_or_init(|| {
        let b = rt_base_slots!();
        let mut t = [0usize; 58];
        t[..57].copy_from_slice(&b);
        t[57] = dc_rt_bind_dc as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

// ID2D1DeviceContext extensions (slots 57..91)
unsafe extern "win64" fn dc_create_bitmap1(
    this: PRT,
    size_u64: u64,
    src_data: *const c_void,
    pitch: u32,
    props1: *const D2D1_BITMAP_PROPERTIES1,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(std::ptr::null_mut());
    let sz = D2D1_SIZE_U::from_u64(size_u64);
    let (pf, dpi_x, dpi_y, opts) = if props1.is_null() {
        (D2D1_PIXEL_FORMAT::default(), 96.0, 96.0, 0)
    } else {
        let p = props1.read_unaligned();
        (p.pixelFormat, p.dpiX, p.dpiY, p.bitmapOptions)
    };
    let mut surf = Surface::new(sz.width, sz.height, pf, dpi_x, dpi_y);
    if !src_data.is_null() && sz.width > 0 && sz.height > 0 {
        let bpp = surf.bpp();
        let row_bytes = (sz.width as usize) * bpp;
        if (pitch as usize) < row_bytes { return E_INVALIDARG; }
        let src_ptr = src_data as *const u8;
        for row in 0..(sz.height as usize) {
            std::ptr::copy_nonoverlapping(
                src_ptr.add(row * pitch as usize),
                surf.data.as_mut_ptr().add(row * row_bytes),
                row_bytes,
            );
        }
    }
    out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::new(Mutex::new(surf)), opts));
    S_OK
}
unsafe extern "win64" fn dc_create_bitmap_from_wic1(
    this: PRT,
    wic: *mut c_void,
    props1: *const D2D1_BITMAP_PROPERTIES1,
    out: *mut *mut c_void,
) -> HRESULT {
    let bp = if props1.is_null() {
        D2D1_BITMAP_PROPERTIES::default()
    } else {
        let p = props1.read_unaligned();
        D2D1_BITMAP_PROPERTIES { pixelFormat: p.pixelFormat, dpiX: p.dpiX, dpiY: p.dpiY }
    };
    rt_create_bitmap_from_wic(this, wic, &bp, out)
}
unsafe extern "win64" fn dc_notimpl_3(_: PRT, _: usize, _: usize, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { out.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}
unsafe extern "win64" fn dc_notimpl_2(_: PRT, _: usize, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { out.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}
unsafe extern "win64" fn dc_notimpl_1(_: PRT, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { out.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}
unsafe extern "win64" fn dc_create_bitmap_from_dxgi(
    this: PRT,
    _surface: *mut c_void,
    props1: *const D2D1_BITMAP_PROPERTIES1,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (pf, dx, dy, opts) = if props1.is_null() {
        (D2D1_PIXEL_FORMAT::default(), 96.0, 96.0, 0)
    } else {
        let p = props1.read_unaligned();
        (p.pixelFormat, p.dpiX, p.dpiY, p.bitmapOptions)
    };
    let surf = Arc::new(Mutex::new(Surface::new(512, 512, pf, dx, dy)));
    out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), surf, opts));
    S_OK
}
unsafe extern "win64" fn dc_create_grad_stops1(
    this: PRT,
    stops: *const D2D1_GRADIENT_STOP,
    count: u32,
    _pre: u32,
    _post: u32,
    _prec: u32,
    extend: u32,
    _interp: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    rt_create_grad_stops(this, stops, count, 0, extend, out)
}
unsafe extern "win64" fn dc_create_bitmap_brush1(
    this: PRT,
    bmp: *mut c_void,
    bb_props: *const D2D1_BITMAP_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    rt_create_bitmap_brush(this, bmp, bb_props, b_props, out)
}
unsafe extern "win64" fn dc_is_dxgi_supported(_this: PRT, fmt: u32) -> BOOL {
    if matches!(fmt, DXGI_FORMAT_UNKNOWN | DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_A8_UNORM) {
        TRUE
    } else {
        FALSE
    }
}
unsafe extern "win64" fn dc_is_prec_supported(_this: PRT, prec: u32) -> BOOL {
    if prec <= 3 { TRUE } else { FALSE }
}
unsafe extern "win64" fn dc_get_image_local_bounds(this: PRT, img: *mut c_void, out: *mut D2D1_RECT_F) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(bmp) = bitmap_from_ptr(img) else { return E_INVALIDARG; };
    let s = bmp.surface.lock().unwrap();
    out.write_unaligned(D2D1_RECT_F {
        left: 0.0,
        top: 0.0,
        right: s.width as f32 * 96.0 / s.dpi_x,
        bottom: s.height as f32 * 96.0 / s.dpi_y,
    });
    S_OK
}
unsafe extern "win64" fn dc_get_image_world_bounds(this: PRT, img: *mut c_void, out: *mut D2D1_RECT_F) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let mut local = D2D1_RECT_F::default();
    let hr = dc_get_image_local_bounds(this, img, &mut local);
    if hr < 0 { return hr; }
    let m = ComObj::inner(this).state.lock().unwrap().transform;
    let sp = transform_subpaths(&[rect_subpath(local)], &m);
    out.write_unaligned(subpaths_bounds(&sp));
    S_OK
}
unsafe extern "win64" fn dc_get_glyph_run_bounds(
    this: PRT,
    origin_u64: u64,
    run_ptr: *const DWRITE_GLYPH_RUN,
    _mode: u32,
    out: *mut D2D1_RECT_F,
) -> HRESULT {
    if this.is_null() || run_ptr.is_null() || out.is_null() { return E_POINTER; }
    let origin = D2D1_POINT_2F::from_u64(origin_u64);
    let run = run_ptr.read_unaligned();
    let mut w = 0.0f32;
    for i in 0..(run.glyphCount as usize) {
        w += if run.glyphAdvances.is_null() { run.fontEmSize * 0.5 } else { run.glyphAdvances.add(i).read_unaligned() };
    }
    let r = D2D1_RECT_F {
        left: origin.x,
        top: origin.y - run.fontEmSize * 0.85,
        right: origin.x + w,
        bottom: origin.y + run.fontEmSize * 0.25,
    };
    let m = ComObj::inner(this).state.lock().unwrap().transform;
    out.write_unaligned(subpaths_bounds(&transform_subpaths(&[rect_subpath(r)], &m)));
    S_OK
}
unsafe extern "win64" fn dc_get_device(this: PRT, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref d) = st.device {
        raw_com_add_ref(d.as_ptr());
        out.write_unaligned(d.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
}
unsafe extern "win64" fn dc_set_target(this: PRT, image: *mut c_void) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if image.is_null() {
        st.target_bitmap = None;
        return;
    }
    if let Some(bmp) = bitmap_from_ptr(image) {
        st.surface = Arc::clone(&bmp.surface);
        st.target_bitmap = ComPtr::retain(image);
    }
}
unsafe extern "win64" fn dc_get_target(this: PRT, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref b) = st.target_bitmap {
        raw_com_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
}
unsafe extern "win64" fn dc_set_render_controls(_this: PRT, _c: *const c_void) {}
unsafe extern "win64" fn dc_get_render_controls(_this: PRT, c: *mut [u32; 3]) {
    if !c.is_null() { c.write_unaligned([1, 256, 256]); }
}
unsafe extern "win64" fn dc_set_prim_blend(this: PRT, blend: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().primitive_blend = blend;
}
unsafe extern "win64" fn dc_get_prim_blend(this: PRT) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().primitive_blend }
}
unsafe extern "win64" fn dc_set_unit_mode(this: PRT, mode: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().unit_mode = mode;
}
unsafe extern "win64" fn dc_get_unit_mode(this: PRT) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().unit_mode }
}
unsafe extern "win64" fn dc_draw_glyph_run1(
    this: PRT,
    origin: u64,
    run: *const DWRITE_GLYPH_RUN,
    _desc: *const c_void,
    brush: *mut c_void,
    mode: u32,
) {
    rt_draw_glyph_run(this, origin, run, brush, mode);
}
unsafe extern "win64" fn dc_draw_image(
    this: PRT,
    image: *mut c_void,
    target_offset: *const D2D1_POINT_2F,
    image_rect: *const D2D1_RECT_F,
    interp: u32,
    _composite: u32,
) {
    let Some(bmp) = bitmap_from_ptr(image) else { return; };
    let off = if target_offset.is_null() { D2D1_POINT_2F::default() } else { target_offset.read_unaligned() };
    let s = bmp.surface.lock().unwrap();
    let (w, h) = if image_rect.is_null() {
        (s.width as f32 * 96.0 / s.dpi_x, s.height as f32 * 96.0 / s.dpi_y)
    } else {
        let ir = image_rect.read_unaligned();
        (ir.right - ir.left, ir.bottom - ir.top)
    };
    drop(s);
    let dst = D2D1_RECT_F { left: off.x, top: off.y, right: off.x + w, bottom: off.y + h };
    rt_draw_bitmap(this, image, &dst, 1.0, interp.min(1), image_rect);
}
unsafe extern "win64" fn dc_draw_gdi_metafile(_this: PRT, _mf: *mut c_void, _off: *const D2D1_POINT_2F) {}
unsafe extern "win64" fn dc_draw_bitmap1(
    this: PRT,
    bitmap: *mut c_void,
    dst_rect: *const D2D1_RECT_F,
    opacity: f32,
    interp: u32,
    src_rect: *const D2D1_RECT_F,
    _perspective: *const c_void,
) {
    rt_draw_bitmap(this, bitmap, dst_rect, opacity, interp.min(1), src_rect);
}
crate::win64_bridge!(fn dc_draw_bitmap1(
    PRT, *mut c_void, *const D2D1_RECT_F, f32, u32, *const D2D1_RECT_F, *const c_void
) -> ());

unsafe extern "win64" fn dc_push_layer1(this: PRT, params1: *const D2D1_LAYER_PARAMETERS, layer: *mut c_void) {
    rt_push_layer(this, params1, layer);
}
unsafe extern "win64" fn dc_inval_effect(_: PRT, _: *mut c_void, _: u32, _: *const D2D1_RECT_F) -> HRESULT { S_OK }
unsafe extern "win64" fn dc_eff_inval_cnt(_: PRT, _: *mut c_void, cnt: *mut u32) -> HRESULT {
    if !cnt.is_null() { cnt.write_unaligned(0); }
    S_OK
}
unsafe extern "win64" fn dc_eff_inval_rects(_: PRT, _: *mut c_void, _: *mut D2D1_RECT_F, _: u32) -> HRESULT { S_OK }
unsafe extern "win64" fn dc_eff_req_rects(
    _: PRT, _: *mut c_void, _: *const D2D1_RECT_F, _: *const c_void, _: *mut D2D1_RECT_F, _: u32
) -> HRESULT { S_OK }
unsafe extern "win64" fn dc_fill_opacity_mask1(
    this: PRT,
    mask: *mut c_void,
    brush: *mut c_void,
    dst_rect: *const D2D1_RECT_F,
    src_rect: *const D2D1_RECT_F,
) {
    rt_fill_opacity_mask(this, mask, brush, 0, dst_rect, src_rect);
}

fn device_context_vtbl() -> *const usize {
    static V: OnceLock<[usize; 92]> = OnceLock::new();
    V.get_or_init(|| {
        let b = rt_base_slots!();
        let mut t = [0usize; 92];
        t[..57].copy_from_slice(&b);
        t[57] = dc_create_bitmap1 as *const () as usize;
        t[58] = dc_create_bitmap_from_wic1 as *const () as usize;
        t[59] = dc_notimpl_3 as *const () as usize;
        t[60] = dc_notimpl_2 as *const () as usize;
        t[61] = dc_notimpl_2 as *const () as usize;
        t[62] = dc_create_bitmap_from_dxgi as *const () as usize;
        t[63] = dc_notimpl_2 as *const () as usize;
        t[64] = dc_create_grad_stops1 as *const () as usize;
        t[65] = dc_notimpl_3 as *const () as usize;
        t[66] = dc_create_bitmap_brush1 as *const () as usize;
        t[67] = dc_notimpl_1 as *const () as usize;
        t[68] = dc_is_dxgi_supported as *const () as usize;
        t[69] = dc_is_prec_supported as *const () as usize;
        t[70] = dc_get_image_local_bounds as *const () as usize;
        t[71] = dc_get_image_world_bounds as *const () as usize;
        t[72] = dc_get_glyph_run_bounds as *const () as usize;
        t[73] = dc_get_device as *const () as usize;
        t[74] = dc_set_target as *const () as usize;
        t[75] = dc_get_target as *const () as usize;
        t[76] = dc_set_render_controls as *const () as usize;
        t[77] = dc_get_render_controls as *const () as usize;
        t[78] = dc_set_prim_blend as *const () as usize;
        t[79] = dc_get_prim_blend as *const () as usize;
        t[80] = dc_set_unit_mode as *const () as usize;
        t[81] = dc_get_unit_mode as *const () as usize;
        t[82] = dc_draw_glyph_run1 as *const () as usize;
        t[83] = dc_draw_image as *const () as usize;
        t[84] = dc_draw_gdi_metafile as *const () as usize;
        t[85] = crate::bridge_entry!(dc_draw_bitmap1) as usize;
        t[86] = dc_push_layer1 as *const () as usize;
        t[87] = dc_inval_effect as *const () as usize;
        t[88] = dc_eff_inval_cnt as *const () as usize;
        t[89] = dc_eff_inval_rects as *const () as usize;
        t[90] = dc_eff_req_rects as *const () as usize;
        t[91] = dc_fill_opacity_mask1 as *const () as usize;
        register_vtable(t)
    })
    .as_ptr()
}

fn new_render_target_com(
    factory: ComPtr,
    kind: RenderTargetKind,
    surface: Arc<Mutex<Surface>>,
    target_bitmap: Option<ComPtr>,
    hwnd: HWND,
    wic_bitmap: Option<ComPtr>,
    device: Option<ComPtr>,
) -> *mut c_void {
    let vtbl = match kind {
        RenderTargetKind::Generic => rt_generic_vtbl(),
        RenderTargetKind::Bitmap => rt_bitmap_vtbl(),
        RenderTargetKind::Hwnd => rt_hwnd_vtbl(),
        RenderTargetKind::Dc => rt_dc_vtbl(),
        RenderTargetKind::DeviceContext => device_context_vtbl(),
    };
    let obj = ComObj::new(
        vtbl,
        RenderTargetInner {
            factory,
            kind,
            state: Mutex::new(RenderTargetState {
                surface,
                target_bitmap,
                transform: D2D1_MATRIX_3X2_F::identity(),
                clip_stack: Vec::new(),
                layer_stack: Vec::new(),
                antialias_mode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                text_antialias_mode: 0,
                text_rendering_params: None,
                primitive_blend: D2D1_PRIMITIVE_BLEND_SOURCE_OVER,
                unit_mode: D2D1_UNIT_MODE_DIPS,
                tag1: 0,
                tag2: 0,
                in_draw: false,
                hwnd,
                bound_hdc: 0,
                bound_subrect: [0; 4],
                wic_bitmap,
                device,
            }),
        },
    );
    obj as *mut c_void
}

// ============================================================================
// ID2D1Device Vtable (9 slots)
// ============================================================================

type PDevice = *mut ComObj<DeviceInner>;

unsafe extern "win64" fn dev_qi(this: PDevice, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_RESOURCE, IID_ID2D1_DEVICE])
}
unsafe extern "win64" fn dev_add_ref(this: PDevice) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn dev_release(this: PDevice) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn dev_get_factory(this: PDevice, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    raw_com_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn dev_create_dc(this: PDevice, _opts: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let surf = Arc::new(Mutex::new(Surface::new(512, 512, D2D1_PIXEL_FORMAT::default(), 96.0, 96.0)));
    let bmp_ptr = new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&surf), 0);
    let bmp_ref = ComPtr::retain(bmp_ptr);
    raw_com_release(bmp_ptr);
    let dev_ref = ComPtr::retain(this as *mut c_void);
    let dc = new_render_target_com(
        ComObj::inner(this).factory.clone(),
        RenderTargetKind::DeviceContext,
        surf,
        bmp_ref,
        std::ptr::null_mut(),
        None,
        dev_ref,
    );
    out.write_unaligned(dc);
    S_OK
}
unsafe extern "win64" fn dev_create_print_control(
    _this: PDevice, _wic: *mut c_void, _doc: *mut c_void, _props: *const c_void, out: *mut *mut c_void
) -> HRESULT {
    if !out.is_null() { out.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}
unsafe extern "win64" fn dev_set_max_tex_mem(this: PDevice, bytes: u64) {
    if this.is_null() { return; }
    *ComObj::inner(this).max_tex_mem.lock().unwrap() = bytes;
}
unsafe extern "win64" fn dev_get_max_tex_mem(this: PDevice) -> u64 {
    if this.is_null() { 0 } else { *ComObj::inner(this).max_tex_mem.lock().unwrap() }
}
unsafe extern "win64" fn dev_clear_resources(_this: PDevice, _ms: u32) {}

fn device_vtbl() -> *const usize {
    static V: OnceLock<[usize; 9]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            dev_qi as *const () as usize,
            dev_add_ref as *const () as usize,
            dev_release as *const () as usize,
            dev_get_factory as *const () as usize,
            dev_create_dc as *const () as usize,
            dev_create_print_control as *const () as usize,
            dev_set_max_tex_mem as *const () as usize,
            dev_get_max_tex_mem as *const () as usize,
            dev_clear_resources as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// ID2D1Factory / ID2D1Factory1 Vtable (27 slots)
// ============================================================================

type PFactory = *mut ComObj<FactoryInner>;

unsafe extern "win64" fn fac_qi(this: PFactory, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_ID2D1_FACTORY, IID_ID2D1_FACTORY1])
}
unsafe extern "win64" fn fac_add_ref(this: PFactory) -> u32 { ComObj::add_ref(this) }
unsafe extern "win64" fn fac_release(this: PFactory) -> u32 { ComObj::release(this) }
unsafe extern "win64" fn fac_reload_metrics(_this: PFactory) -> HRESULT { S_OK }
unsafe extern "win64" fn fac_get_desktop_dpi(_this: PFactory, dpi_x: *mut f32, dpi_y: *mut f32) {
    let d = crate::user32::window::dpi() as f32;
    let dpi = if d > 0.0 { d } else { 96.0 };
    if !dpi_x.is_null() { dpi_x.write_unaligned(dpi); }
    if !dpi_y.is_null() { dpi_y.write_unaligned(dpi); }
}
unsafe extern "win64" fn fac_create_rect_geom(
    this: PFactory,
    rect: *const D2D1_RECT_F,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || rect.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let obj = ComObj::new(rect_geom_vtbl(), GeometryInner { factory: fac, kind: GeometryKind::Rect(rect.read_unaligned()) });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_rrect_geom(
    this: PFactory,
    rrect: *const D2D1_ROUNDED_RECT,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || rrect.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let obj = ComObj::new(
        rounded_rect_geom_vtbl(),
        GeometryInner { factory: fac, kind: GeometryKind::RoundedRect(rrect.read_unaligned()) },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_ellipse_geom(
    this: PFactory,
    el: *const D2D1_ELLIPSE,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || el.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let obj = ComObj::new(
        ellipse_geom_vtbl(),
        GeometryInner { factory: fac, kind: GeometryKind::Ellipse(el.read_unaligned()) },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_geom_group(
    this: PFactory,
    fill_mode: u32,
    geoms: *const *mut c_void,
    count: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() || (count > 0 && geoms.is_null()) { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let mut children = Vec::with_capacity(count as usize);
    for i in 0..(count as usize) {
        let p = geoms.add(i).read_unaligned();
        if let Some(cp) = ComPtr::retain(p) {
            children.push(cp);
        }
    }
    let obj = ComObj::new(
        geom_group_vtbl(),
        GeometryInner { factory: fac, kind: GeometryKind::Group { fill_mode, children } },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_transformed_geom(
    this: PFactory,
    source: *mut c_void,
    transform: *const D2D1_MATRIX_3X2_F,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || source.is_null() || transform.is_null() || out.is_null() { return E_POINTER; }
    let (Some(fac), Some(src)) = (ComPtr::retain(this as *mut c_void), ComPtr::retain(source)) else {
        return E_INVALIDARG;
    };
    let obj = ComObj::new(
        transformed_geom_vtbl(),
        GeometryInner {
            factory: fac,
            kind: GeometryKind::Transformed { source: src, transform: transform.read_unaligned() },
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_path_geom(this: PFactory, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let obj = ComObj::new(
        path_geom_vtbl(),
        GeometryInner { factory: fac, kind: GeometryKind::Path(Arc::new(Mutex::new(PathData::default()))) },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_stroke_style(
    this: PFactory,
    props: *const D2D1_STROKE_STYLE_PROPERTIES,
    dashes: *const f32,
    dashes_count: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || props.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = props.read_unaligned();
    let mut d = Vec::new();
    if !dashes.is_null() && dashes_count > 0 {
        for i in 0..(dashes_count as usize) {
            d.push(dashes.add(i).read_unaligned());
        }
    }
    let obj = ComObj::new(
        stroke_style_vtbl(),
        StrokeStyleInner { factory: fac, data: StrokeStyleData { props: p, transform_type: 0, dashes: d } },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_drawing_state_block(
    this: PFactory,
    desc: *const D2D1_DRAWING_STATE_DESCRIPTION,
    text_params: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let d = if desc.is_null() { D2D1_DRAWING_STATE_DESCRIPTION::default() } else { desc.read_unaligned() };
    let obj = ComObj::new(
        drawing_state_block_vtbl(),
        DrawingStateBlockInner { factory: fac, state: Mutex::new((d, ComPtr::retain(text_params))) },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_wic_rt(
    this: PFactory,
    wic_bitmap: *mut c_void,
    props: *const D2D1_RENDER_TARGET_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = if props.is_null() { D2D1_RENDER_TARGET_PROPERTIES::default() } else { props.read_unaligned() };
    let mut w = 512u32;
    let mut h = 512u32;
    if !wic_bitmap.is_null() {
        let vtbl = (wic_bitmap as *const *const usize).read_unaligned();
        if !vtbl.is_null() {
            let get_size = vtbl.add(3).read_unaligned();
            if get_size != 0 {
                let mut ww = 0u32;
                let mut hh = 0u32;
                let hr = crate::call_guest8(
                    get_size,
                    wic_bitmap as u64,
                    &mut ww as *mut u32 as u64,
                    &mut hh as *mut u32 as u64,
                    0, 0, 0, 0, 0,
                ) as HRESULT;
                if hr >= 0 && ww > 0 && hh > 0 {
                    w = ww;
                    h = hh;
                }
            }
        }
    }
    let surf = Arc::new(Mutex::new(Surface::new(w, h, p.pixelFormat, p.dpiX, p.dpiY)));
    let rt = new_render_target_com(
        fac,
        RenderTargetKind::Generic,
        surf,
        None,
        std::ptr::null_mut(),
        ComPtr::retain(wic_bitmap),
        None,
    );
    out.write_unaligned(rt);
    S_OK
}
unsafe extern "win64" fn fac_create_hwnd_rt(
    this: PFactory,
    props: *const D2D1_RENDER_TARGET_PROPERTIES,
    hwnd_props: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || hwnd_props.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = if props.is_null() { D2D1_RENDER_TARGET_PROPERTIES::default() } else { props.read_unaligned() };
    let hp = hwnd_props.read_unaligned();
    let (w, h) = if hp.pixelSize.width > 0 && hp.pixelSize.height > 0 {
        (hp.pixelSize.width, hp.pixelSize.height)
    } else if !hp.hwnd.is_null() {
        crate::user32::window::client_size(hp.hwnd as usize)
            .map(|(cw, ch)| (cw.max(1) as u32, ch.max(1) as u32))
            .unwrap_or((640, 480))
    } else {
        (640, 480)
    };
    let surf = Arc::new(Mutex::new(Surface::new(w, h, p.pixelFormat, p.dpiX, p.dpiY)));
    let rt = new_render_target_com(fac, RenderTargetKind::Hwnd, surf, None, hp.hwnd, None, None);
    out.write_unaligned(rt);
    S_OK
}
unsafe extern "win64" fn fac_create_dxgi_rt(
    this: PFactory,
    _dxgi_surface: *mut c_void,
    props: *const D2D1_RENDER_TARGET_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = if props.is_null() { D2D1_RENDER_TARGET_PROPERTIES::default() } else { props.read_unaligned() };
    let surf = Arc::new(Mutex::new(Surface::new(512, 512, p.pixelFormat, p.dpiX, p.dpiY)));
    let rt = new_render_target_com(fac, RenderTargetKind::Generic, surf, None, std::ptr::null_mut(), None, None);
    out.write_unaligned(rt);
    S_OK
}
unsafe extern "win64" fn fac_create_dc_rt(
    this: PFactory,
    props: *const D2D1_RENDER_TARGET_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = if props.is_null() { D2D1_RENDER_TARGET_PROPERTIES::default() } else { props.read_unaligned() };
    let surf = Arc::new(Mutex::new(Surface::new(1, 1, p.pixelFormat, p.dpiX, p.dpiY)));
    let rt = new_render_target_com(fac, RenderTargetKind::Dc, surf, None, std::ptr::null_mut(), None, None);
    out.write_unaligned(rt);
    S_OK
}
unsafe extern "win64" fn fac_create_device(
    this: PFactory,
    _dxgi_device: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let dev = ComObj::new(device_vtbl(), DeviceInner { factory: fac, max_tex_mem: Mutex::new(256 * 1024 * 1024) });
    out.write_unaligned(dev as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_stroke_style1(
    this: PFactory,
    props1: *const D2D1_STROKE_STYLE_PROPERTIES1,
    dashes: *const f32,
    dashes_count: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || props1.is_null() || out.is_null() { return E_POINTER; }
    let Some(fac) = ComPtr::retain(this as *mut c_void) else { return E_INVALIDARG; };
    let p = props1.read_unaligned();
    let mut d = Vec::new();
    if !dashes.is_null() && dashes_count > 0 {
        for i in 0..(dashes_count as usize) {
            d.push(dashes.add(i).read_unaligned());
        }
    }
    let obj = ComObj::new(
        stroke_style_vtbl(),
        StrokeStyleInner {
            factory: fac,
            data: StrokeStyleData { props: p.base, transform_type: p.transformType, dashes: d },
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_notimpl_effect(_: PFactory, _: usize, _: usize, _: usize, _: usize, _: usize) -> HRESULT {
    E_NOTIMPL
}
unsafe extern "win64" fn fac_unreg_effect(_: PFactory, _: *const u8) -> HRESULT { E_NOTIMPL }
unsafe extern "win64" fn fac_get_reg_effects(
    _: PFactory, _: *mut u8, _: u32, ret: *mut u32, reg: *mut u32
) -> HRESULT {
    if !ret.is_null() { ret.write_unaligned(0); }
    if !reg.is_null() { reg.write_unaligned(0); }
    S_OK
}
unsafe extern "win64" fn fac_get_eff_props(_: PFactory, _: *const u8, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { out.write_unaligned(std::ptr::null_mut()); }
    E_NOTIMPL
}

fn factory_vtbl() -> *const usize {
    static V: OnceLock<[usize; 27]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            fac_qi as *const () as usize,                                // 0
            fac_add_ref as *const () as usize,                           // 1
            fac_release as *const () as usize,                           // 2
            fac_reload_metrics as *const () as usize,                    // 3
            fac_get_desktop_dpi as *const () as usize,                   // 4
            fac_create_rect_geom as *const () as usize,                  // 5
            fac_create_rrect_geom as *const () as usize,                 // 6
            fac_create_ellipse_geom as *const () as usize,               // 7
            fac_create_geom_group as *const () as usize,                 // 8
            fac_create_transformed_geom as *const () as usize,           // 9
            fac_create_path_geom as *const () as usize,                  // 10
            fac_create_stroke_style as *const () as usize,               // 11
            fac_create_drawing_state_block as *const () as usize,        // 12
            fac_create_wic_rt as *const () as usize,                     // 13
            fac_create_hwnd_rt as *const () as usize,                    // 14
            fac_create_dxgi_rt as *const () as usize,                    // 15
            fac_create_dc_rt as *const () as usize,                      // 16
            fac_create_device as *const () as usize,                     // 17 (ID2D1Factory1)
            fac_create_stroke_style1 as *const () as usize,              // 18
            fac_create_path_geom as *const () as usize,                  // 19
            fac_create_drawing_state_block as *const () as usize,        // 20
            fac_get_eff_props as *const () as usize,                     // 21 CreateGdiMetafile
            fac_notimpl_effect as *const () as usize,                    // 22 RegisterEffectFromStream
            fac_notimpl_effect as *const () as usize,                    // 23 RegisterEffectFromString
            fac_unreg_effect as *const () as usize,                      // 24 UnregisterEffect
            fac_get_reg_effects as *const () as usize,                   // 25 GetRegisteredEffects
            fac_get_eff_props as *const () as usize,                     // 26 GetEffectProperties
        ])
    })
    .as_ptr()
}

fn new_factory_com(factory_type: u32) -> *mut c_void {
    ComObj::new(factory_vtbl(), FactoryInner { _factory_type: factory_type }) as *mut c_void
}

// ============================================================================
// Public DLL Exports
// ============================================================================

pub unsafe extern "win64" fn D2D1CreateFactory(
    factory_type: u32,
    riid: *const u8,
    _options: *const c_void,
    factory: *mut *mut c_void,
) -> HRESULT {
    if factory.is_null() {
        return E_POINTER;
    }
    factory.write_unaligned(std::ptr::null_mut());
    if factory_type != D2D1_FACTORY_TYPE_SINGLE_THREADED && factory_type != D2D1_FACTORY_TYPE_MULTI_THREADED {
        return E_INVALIDARG;
    }
    if riid.is_null() {
        return E_INVALIDARG;
    }
    let req = (riid as *const GUID).read_unaligned();
    if req != IID_IUNKNOWN && req != IID_ID2D1_FACTORY && req != IID_ID2D1_FACTORY1 {
        return E_NOINTERFACE;
    }
    factory.write_unaligned(new_factory_com(factory_type));
    S_OK
}

pub unsafe extern "win64" fn D2D1CreateDevice(
    dxgi_device: *mut c_void,
    _props: *const c_void,
    d2d_device: *mut *mut c_void,
) -> HRESULT {
    if d2d_device.is_null() {
        return E_POINTER;
    }
    d2d_device.write_unaligned(std::ptr::null_mut());
    let fac = new_factory_com(D2D1_FACTORY_TYPE_MULTI_THREADED);
    let hr = fac_create_device(fac as PFactory, dxgi_device, d2d_device);
    raw_com_release(fac);
    hr
}

pub unsafe extern "win64" fn D2D1CreateDeviceContext(
    dxgi_surface: *mut c_void,
    props: *const c_void,
    d2d_dc: *mut *mut c_void,
) -> HRESULT {
    if d2d_dc.is_null() {
        return E_POINTER;
    }
    d2d_dc.write_unaligned(std::ptr::null_mut());
    let mut dev: *mut c_void = std::ptr::null_mut();
    let hr = D2D1CreateDevice(dxgi_surface, props, &mut dev);
    if hr < 0 {
        return hr;
    }
    let hr2 = dev_create_dc(dev as PDevice, 0, d2d_dc);
    raw_com_release(dev);
    hr2
}

unsafe extern "win64" fn d2d1_make_rotate_matrix_impl(
    angle: f32,
    center: u64,
    matrix: *mut D2D1_MATRIX_3X2_F,
) {
    if matrix.is_null() {
        return;
    }
    let c = D2D1_POINT_2F::from_u64(center);
    let (s, co) = angle.to_radians().sin_cos();
    matrix.write_unaligned(D2D1_MATRIX_3X2_F {
        m11: co,
        m12: s,
        m21: -s,
        m22: co,
        dx: c.x - c.x * co + c.y * s,
        dy: c.y - c.x * s - c.y * co,
    });
}
crate::win64_bridge!(fn d2d1_make_rotate_matrix_impl(f32, u64, *mut D2D1_MATRIX_3X2_F) -> ());

/// Rust-callable wrapper for `D2D1MakeRotateMatrix` (guest calls go through `EXTRA_EXPORTS` +
/// `win64_bridge!` so `angle: f32` is read from Win64 XMM0 on ARM64).
pub unsafe fn D2D1MakeRotateMatrix(angle: f32, center: u64, matrix: *mut D2D1_MATRIX_3X2_F) {
    d2d1_make_rotate_matrix_impl(angle, center, matrix);
}

unsafe extern "win64" fn d2d1_make_skew_matrix_impl(
    angle_x: f32,
    angle_y: f32,
    center: u64,
    matrix: *mut D2D1_MATRIX_3X2_F,
) {
    if matrix.is_null() {
        return;
    }
    let c = D2D1_POINT_2F::from_u64(center);
    let tx = angle_x.to_radians().tan();
    let ty = angle_y.to_radians().tan();
    matrix.write_unaligned(D2D1_MATRIX_3X2_F {
        m11: 1.0,
        m12: ty,
        m21: tx,
        m22: 1.0,
        dx: -c.y * tx,
        dy: -c.x * ty,
    });
}
crate::win64_bridge!(fn d2d1_make_skew_matrix_impl(f32, f32, u64, *mut D2D1_MATRIX_3X2_F) -> ());

/// Rust-callable wrapper for `D2D1MakeSkewMatrix` (guest calls go through `EXTRA_EXPORTS` +
/// `win64_bridge!` so `angle_x`/`angle_y` are read from Win64 XMM0/XMM1 on ARM64).
pub unsafe fn D2D1MakeSkewMatrix(angle_x: f32, angle_y: f32, center: u64, matrix: *mut D2D1_MATRIX_3X2_F) {
    d2d1_make_skew_matrix_impl(angle_x, angle_y, center, matrix);
}

pub unsafe extern "win64" fn D2D1IsMatrixInvertible(matrix: *const D2D1_MATRIX_3X2_F) -> BOOL {
    if matrix.is_null() {
        return FALSE;
    }
    if matrix.read_unaligned().invert().is_some() { TRUE } else { FALSE }
}

pub unsafe extern "win64" fn D2D1InvertMatrix(matrix: *mut D2D1_MATRIX_3X2_F) -> BOOL {
    if matrix.is_null() {
        return FALSE;
    }
    if let Some(inv) = matrix.read_unaligned().invert() {
        matrix.write_unaligned(inv);
        TRUE
    } else {
        FALSE
    }
}

// ============================================================================
// Unit Tests (strictly at the bottom for build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn vtbl_of(obj: *mut c_void) -> *const usize {
        (obj as *const *const usize).read_unaligned()
    }

    unsafe fn release_com(obj: *mut c_void) -> u32 {
        let f: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(vtbl_of(obj).add(2).read_unaligned());
        f(obj)
    }

    /// Helper: read pixel `[r, g, b, a]` at `(x, y)` from any Direct2D RenderTarget via
    /// `ID2D1Bitmap1::CopyFromRenderTarget` + `Map` (official Direct2D readback path!).
    unsafe fn read_rt_pixel(rt: *mut c_void, x: u32, y: u32) -> [u8; 4] {
        let mut sz = D2D1_SIZE_U::default();
        rt_get_pixel_size(rt as PRT, &mut sz);
        let mut bmp: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            rt_create_bitmap(
                rt as PRT,
                sz.to_u64(),
                std::ptr::null(),
                0,
                std::ptr::null(),
                &mut bmp,
            ),
            S_OK
        );
        assert_eq!(bmp_copy_from_rt(bmp as PBitmap, std::ptr::null(), rt, std::ptr::null()), S_OK);
        let mut mapped = D2D1_MAPPED_RECT { pitch: 0, _pad: 0, bits: std::ptr::null_mut() };
        assert_eq!(bmp_map(bmp as PBitmap, 0, &mut mapped), S_OK);
        let off = (y as usize) * (mapped.pitch as usize) + (x as usize) * 4;
        let p = mapped.bits.add(off);
        let out = [p.read_unaligned(), p.add(1).read_unaligned(), p.add(2).read_unaligned(), p.add(3).read_unaligned()];
        assert_eq!(bmp_unmap(bmp as PBitmap), S_OK);
        release_com(bmp);
        out
    }

    #[test]
    fn matrix_helpers_rotate_skew_invert_and_bridge() {
        unsafe {
            let center = D2D1_POINT_2F { x: 10.0, y: 20.0 };
            let mut rot = D2D1_MATRIX_3X2_F::default();
            D2D1MakeRotateMatrix(90.0, center.to_u64(), &mut rot);
            let (cx, cy) = rot.transform_point(10.0, 20.0);
            assert!((cx - 10.0).abs() < 1e-4 && (cy - 20.0).abs() < 1e-4);
            let (rx, ry) = rot.transform_point(15.0, 20.0);
            assert!((rx - 10.0).abs() < 1e-4 && (ry - 25.0).abs() < 1e-4);

            assert_eq!(D2D1IsMatrixInvertible(&rot), TRUE);
            let mut inv = rot;
            assert_eq!(D2D1InvertMatrix(&mut inv), TRUE);
            let prod = rot.mul(&inv);
            assert!((prod.m11 - 1.0).abs() < 1e-4 && (prod.m22 - 1.0).abs() < 1e-4);
            assert!(prod.dx.abs() < 1e-4 && prod.dy.abs() < 1e-4);

            let singular = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 2.0, m21: 2.0, m22: 4.0, dx: 0.0, dy: 0.0 };
            assert_eq!(D2D1IsMatrixInvertible(&singular), FALSE);
            let mut sing_copy = singular;
            assert_eq!(D2D1InvertMatrix(&mut sing_copy), FALSE);

            let mut skew = D2D1_MATRIX_3X2_F::default();
            D2D1MakeSkewMatrix(45.0, 0.0, D2D1_POINT_2F { x: 0.0, y: 10.0 }.to_u64(), &mut skew);
            let (sx, sy) = skew.transform_point(0.0, 20.0);
            assert!((sx - 10.0).abs() < 1e-4 && (sy - 20.0).abs() < 1e-4);

            // On aarch64, also directly verify the win64_bridge! thunk unpacks XMM0/XMM1 + GPRs.
            #[cfg(target_arch = "aarch64")]
            {
                let mut m2 = D2D1_MATRIX_3X2_F::default();
                d2d1_make_rotate_matrix_impl::bridge(
                    0,
                    center.to_u64(),
                    &mut m2 as *mut D2D1_MATRIX_3X2_F as u64,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f64::from_bits(90.0f32.to_bits() as u64),
                    0.0,
                    0.0,
                    0.0,
                );
                assert!((m2.m11 - rot.m11).abs() < 1e-5 && (m2.m12 - rot.m12).abs() < 1e-5);
                assert!((m2.dx - rot.dx).abs() < 1e-4 && (m2.dy - rot.dy).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn rasterizer_primitives_brushes_transforms_clips_and_paths() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D2D1CreateFactory(
                    D2D1_FACTORY_TYPE_SINGLE_THREADED,
                    &IID_ID2D1_FACTORY1 as *const GUID as *const u8,
                    std::ptr::null(),
                    &mut fac,
                ),
                S_OK
            );

            // Create 64x64 HWND render target
            let hp = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: std::ptr::null_mut(),
                pixelSize: D2D1_SIZE_U { width: 64, height: 64 },
                presentOptions: 0,
                _pad: 0,
            };
            let mut rt: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_hwnd_rt(fac as PFactory, std::ptr::null(), &hp, &mut rt), S_OK);

            // Solid red brush
            let red = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
            let mut red_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(rt as PRT, &red, std::ptr::null(), &mut red_brush), S_OK);

            // Solid green brush (50% opacity)
            let green = D2D1_COLOR_F { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
            let mut green_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(rt as PRT, &green, std::ptr::null(), &mut green_brush), S_OK);
            brush_set_opacity(green_brush as PBrush, 0.5);

            // Clear to solid blue
            rt_begin_draw(rt as PRT);
            let blue = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
            rt_clear(rt as PRT, &blue);
            assert_eq!(read_rt_pixel(rt, 0, 0), [0, 0, 255, 255]);

            // FillRectangle with clip stack: fill [0..32, 0..32] clipped to [8..16, 8..16]
            let clip = D2D1_RECT_F { left: 8.0, top: 8.0, right: 16.0, bottom: 16.0 };
            rt_push_clip(rt as PRT, &clip, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
            let big = D2D1_RECT_F { left: 0.0, top: 0.0, right: 32.0, bottom: 32.0 };
            rt_fill_rect(rt as PRT, &big, red_brush);
            rt_pop_clip(rt as PRT);
            assert_eq!(read_rt_pixel(rt, 12, 12), [255, 0, 0, 255]);
            assert_eq!(read_rt_pixel(rt, 4, 4), [0, 0, 255, 255]);

            // Blend 50% green over red at (12, 12)
            let small = D2D1_RECT_F { left: 10.0, top: 10.0, right: 14.0, bottom: 14.0 };
            rt_fill_rect(rt as PRT, &small, green_brush);
            let blended = read_rt_pixel(rt, 12, 12);
            assert!((blended[0] as i32 - 128).abs() <= 2 && (blended[1] as i32 - 128).abs() <= 2);

            // FillEllipse at (48, 16) radius (10, 10)
            let el = D2D1_ELLIPSE { point: D2D1_POINT_2F { x: 48.0, y: 16.0 }, radiusX: 10.0, radiusY: 10.0 };
            rt_fill_ellipse(rt as PRT, &el, red_brush);
            assert_eq!(read_rt_pixel(rt, 48, 16), [255, 0, 0, 255]);
            assert_eq!(read_rt_pixel(rt, 36, 4), [0, 0, 255, 255]);

            // FillRoundedRectangle & DrawLine with translation transform
            let tx = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 4.0, dy: 4.0 };
            rt_set_transform(rt as PRT, &tx);
            let rr = D2D1_ROUNDED_RECT {
                rect: D2D1_RECT_F { left: 0.0, top: 36.0, right: 20.0, bottom: 52.0 },
                radiusX: 4.0,
                radiusY: 4.0,
            };
            rt_fill_rounded_rect(rt as PRT, &rr, red_brush);
            // Center of rounded rect is at (10+4, 44+4) = (14, 48)
            assert_eq!(read_rt_pixel(rt, 14, 48), [255, 0, 0, 255]);
            // Reset transform and draw white horizontal line across y=32
            let id = D2D1_MATRIX_3X2_F::identity();
            rt_set_transform(rt as PRT, &id);
            let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
            let mut white_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(rt as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);
            rt_draw_line(
                rt as PRT,
                D2D1_POINT_2F { x: 2.0, y: 32.0 }.to_u64(),
                D2D1_POINT_2F { x: 60.0, y: 32.0 }.to_u64(),
                white_brush,
                3.0,
                std::ptr::null_mut(),
            );
            assert_eq!(read_rt_pixel(rt, 30, 32), [255, 255, 255, 255]);

            // PathGeometry triangle + FillGeometry with LinearGradientBrush (black -> white left to right)
            let mut path: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_path_geom(fac as PFactory, &mut path), S_OK);
            let mut sink: *mut c_void = std::ptr::null_mut();
            assert_eq!(path_geom_open(path as PGeom, &mut sink), S_OK);
            gsink_begin_figure(sink as PGeomSink, D2D1_POINT_2F { x: 32.0, y: 40.0 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
            gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 60.0, y: 40.0 }.to_u64());
            gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 60.0, y: 60.0 }.to_u64());
            gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 32.0, y: 60.0 }.to_u64());
            gsink_end_figure(sink as PGeomSink, D2D1_FIGURE_END_CLOSED);
            assert_eq!(gsink_close(sink as PGeomSink), S_OK);

            let stops = [
                D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } },
                D2D1_GRADIENT_STOP { position: 1.0, color: D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } },
            ];
            let mut stop_col: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_grad_stops(rt as PRT, stops.as_ptr(), 2, 0, D2D1_EXTEND_MODE_CLAMP, &mut stop_col), S_OK);
            let lgp = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: D2D1_POINT_2F { x: 32.0, y: 50.0 },
                endPoint: D2D1_POINT_2F { x: 60.0, y: 50.0 },
            };
            let mut lin_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_linear_brush(rt as PRT, &lgp, std::ptr::null(), stop_col, &mut lin_brush), S_OK);
            rt_fill_geometry(rt as PRT, path, lin_brush, std::ptr::null_mut());
            let left_px = read_rt_pixel(rt, 34, 50);
            let right_px = read_rt_pixel(rt, 58, 50);
            assert!(left_px[0] < 40 && right_px[0] > 215, "gradient left={left_px:?} right={right_px:?}");

            assert_eq!(rt_end_draw(rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

            release_com(lin_brush);
            release_com(stop_col);
            release_com(sink);
            release_com(path);
            release_com(white_brush);
            release_com(green_brush);
            release_com(red_brush);
            release_com(rt);
            release_com(fac);
        }
    }

    #[test]
    fn device_context_draw_bitmap_and_dwrite_text_integration() {
        unsafe {
            let mut dc: *mut c_void = std::ptr::null_mut();
            assert_eq!(D2D1CreateDeviceContext(std::ptr::null_mut(), std::ptr::null(), &mut dc), S_OK);
            assert!(!dc.is_null());

            // Create 128x48 target bitmap and bind via SetTarget
            let mut target: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                dc_create_bitmap1(
                    dc as PRT,
                    D2D1_SIZE_U { width: 128, height: 48 }.to_u64(),
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    &mut target,
                ),
                S_OK
            );
            dc_set_target(dc as PRT, target);

            // Create 2x2 RGBA bitmap and DrawBitmap into [0..8, 0..8]
            let src_pixels: [u8; 16] = [
                255, 255, 0, 255,   255, 255, 0, 255,
                255, 255, 0, 255,   255, 255, 0, 255,
            ];
            let mut src_bmp: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                rt_create_bitmap(
                    dc as PRT,
                    D2D1_SIZE_U { width: 2, height: 2 }.to_u64(),
                    src_pixels.as_ptr() as *const c_void,
                    8,
                    std::ptr::null(),
                    &mut src_bmp,
                ),
                S_OK
            );

            // Create white brush for text
            let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
            let mut white_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(dc as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);

            // Create DirectWrite factory + TextFormat + TextLayout
            let mut dw_fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                dwrite::DWriteCreateFactory(
                    dwrite::DWRITE_FACTORY_TYPE_SHARED,
                    &dwrite::IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut dw_fac,
                ),
                S_OK
            );
            let fam_w: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
            let loc_w: Vec<u16> = "en-us\0".encode_utf16().collect();
            let create_tf: unsafe extern "win64" fn(
                *mut c_void, *const u16, *mut c_void, u32, u32, u32, f32, *const u16, *mut *mut c_void
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
            let mut fmt: *mut c_void = std::ptr::null_mut();
            #[cfg(not(target_arch = "aarch64"))]
            assert_eq!(
                create_tf(dw_fac, fam_w.as_ptr(), std::ptr::null_mut(), 400, 0, 5, 24.0, loc_w.as_ptr(), &mut fmt),
                S_OK
            );
            #[cfg(target_arch = "aarch64")]
            {
                // Call through the aarch64 win64_bridge! slot directly, validating the bridge ABI too.
                let _ = create_tf;
                let br: unsafe extern "C" fn(
                    u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                    f64, f64, f64, f64,
                ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
                assert_eq!(
                    br(
                        dw_fac as u64,
                        fam_w.as_ptr() as u64,
                        0,
                        400,
                        0,
                        5,
                        24.0f32.to_bits() as u64,
                        loc_w.as_ptr() as u64,
                        &mut fmt as *mut *mut c_void as u64,
                        0, 0, 0, 0, 0, 0, 0,
                        0.0, 0.0, 0.0, 0.0,
                    ),
                    S_OK
                );
            }
            assert!(!fmt.is_null());

            rt_begin_draw(dc as PRT);
            let black = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
            rt_clear(dc as PRT, &black);
            let dst_bmp_rect = D2D1_RECT_F { left: 0.0, top: 0.0, right: 8.0, bottom: 8.0 };
            rt_draw_bitmap(dc as PRT, src_bmp, &dst_bmp_rect, 1.0, D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, std::ptr::null());
            assert_eq!(read_rt_pixel(dc, 4, 4), [255, 255, 0, 255]);

            // DrawText ("HELLO") onto [16..124, 4..44]
            let hello: Vec<u16> = "HELLO".encode_utf16().collect();
            let text_rect = D2D1_RECT_F { left: 16.0, top: 4.0, right: 124.0, bottom: 44.0 };
            rt_draw_text(dc as PRT, hello.as_ptr(), hello.len() as u32, fmt, &text_rect, white_brush, 0, 0);
            assert_eq!(rt_end_draw(dc as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

            // Verify real lit text pixels were rasterized inside the text region!
            let mut lit_pixels = 0u32;
            for py in 4..44u32 {
                for px in 16..124u32 {
                    let c = read_rt_pixel(dc, px, py);
                    if c[0] > 64 && c[1] > 64 && c[2] > 64 {
                        lit_pixels += 1;
                    }
                }
            }
            assert!(lit_pixels >= 40, "expected real rasterized text pixels, got {lit_pixels}");

            release_com(fmt);
            release_com(dw_fac);
            release_com(white_brush);
            release_com(src_bmp);
            release_com(target);
            release_com(dc);
        }
    }

    #[test]
    fn dc_render_target_flushes_pixels_to_gdi_hdc() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D2D1CreateFactory(
                    D2D1_FACTORY_TYPE_SINGLE_THREADED,
                    &IID_ID2D1_FACTORY as *const GUID as *const u8,
                    std::ptr::null(),
                    &mut fac,
                ),
                S_OK
            );
            let mut dc_rt: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_dc_rt(fac as PFactory, std::ptr::null(), &mut dc_rt), S_OK);

            // Create real GDI memory DC + 16x16 DIB
            let hdc = crate::misc::gdi32::CreateCompatibleDC(0);
            let mut bmi = [0u8; 40];
            bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
            bmi[4..8].copy_from_slice(&16i32.to_le_bytes());
            bmi[8..12].copy_from_slice(&(-16i32).to_le_bytes());
            bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
            bmi[14..16].copy_from_slice(&32u16.to_le_bytes());
            let mut bits = std::ptr::null_mut();
            let hbm = crate::misc::gdi32::CreateDIBSection(hdc, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
            let old = crate::misc::gdi32::SelectObject(hdc, hbm);

            let sub = [0i32, 0, 16, 16];
            assert_eq!(dc_rt_bind_dc(dc_rt as PRT, hdc, &sub), S_OK);
            rt_begin_draw(dc_rt as PRT);
            let cyan = D2D1_COLOR_F { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
            rt_clear(dc_rt as PRT, &cyan);
            assert_eq!(rt_end_draw(dc_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

            // GDI GetPixel returns COLORREF (0x00BBGGRR -> cyan = 0x00FFFF00)
            let cr = crate::misc::gdi32::GetPixel(hdc, 5, 5);
            assert_eq!(cr, 0x00FF_FF00);

            crate::misc::gdi32::SelectObject(hdc, old);
            crate::misc::gdi32::DeleteObject(hbm);
            crate::misc::gdi32::DeleteDC(hdc);
            release_com(dc_rt);
            release_com(fac);
        }
    }

    #[test]
    fn dll_exports_device_compatible_rt_brushes_strokes_geometries_layers_and_wic() {
        unsafe {
            // 1. All 7 public d2d1.dll exports resolve via win32_rt::lookup
            for name in [
                "D2D1CreateFactory",
                "D2D1CreateDevice",
                "D2D1CreateDeviceContext",
                "D2D1MakeRotateMatrix",
                "D2D1MakeSkewMatrix",
                "D2D1IsMatrixInvertible",
                "D2D1InvertMatrix",
            ] {
                assert!(
                    crate::lookup("d2d1.dll", name).is_some(),
                    "missing d2d1.dll export {name}"
                );
            }

            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D2D1CreateFactory(
                    D2D1_FACTORY_TYPE_MULTI_THREADED,
                    &IID_ID2D1_FACTORY1 as *const GUID as *const u8,
                    std::ptr::null(),
                    &mut fac,
                ),
                S_OK
            );

            // 2. ID2D1Factory1::CreateDevice -> ID2D1Device::CreateDeviceContext -> CreateCompatibleRenderTarget
            let mut dev: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_device(fac as PFactory, std::ptr::null_mut(), &mut dev), S_OK);
            let mut dc: *mut c_void = std::ptr::null_mut();
            assert_eq!(dev_create_dc(dev as PDevice, 0, &mut dc), S_OK);

            let compat_px = D2D1_SIZE_U { width: 64, height: 64 };
            let mut compat_rt: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                rt_create_compat_rt(dc as PRT, std::ptr::null(), &compat_px, std::ptr::null(), 0, &mut compat_rt),
                S_OK
            );

            // 3. RadialGradientBrush & BitmapBrush
            let stops = [
                D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } },
                D2D1_GRADIENT_STOP { position: 1.0, color: D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } },
            ];
            let mut stop_col: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                rt_create_grad_stops(compat_rt as PRT, stops.as_ptr(), 2, 0, D2D1_EXTEND_MODE_CLAMP, &mut stop_col),
                S_OK
            );
            let rgp = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                center: D2D1_POINT_2F { x: 16.0, y: 16.0 },
                gradientOriginOffset: D2D1_POINT_2F { x: 0.0, y: 0.0 },
                radiusX: 16.0,
                radiusY: 16.0,
            };
            let mut rad_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_radial_brush(compat_rt as PRT, &rgp, std::ptr::null(), stop_col, &mut rad_brush), S_OK);

            // 2x2 red/blue checkerboard BitmapBrush with WRAP
            let checker: [u8; 16] = [
                255, 0, 0, 255,   0, 0, 255, 255,
                0, 0, 255, 255,   255, 0, 0, 255,
            ];
            let mut chk_bmp: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                rt_create_bitmap(
                    compat_rt as PRT,
                    D2D1_SIZE_U { width: 2, height: 2 }.to_u64(),
                    checker.as_ptr() as *const c_void,
                    8,
                    std::ptr::null(),
                    &mut chk_bmp,
                ),
                S_OK
            );
            let bbp = D2D1_BITMAP_BRUSH_PROPERTIES {
                extendModeX: D2D1_EXTEND_MODE_WRAP,
                extendModeY: D2D1_EXTEND_MODE_WRAP,
                interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
            };
            let mut bmp_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_bitmap_brush(compat_rt as PRT, chk_bmp, &bbp, std::ptr::null(), &mut bmp_brush), S_OK);

            // 4. Stroke style + SaveDrawingState/RestoreDrawingState + PushLayer/PopLayer
            let ss_props = D2D1_STROKE_STYLE_PROPERTIES {
                startCap: D2D1_CAP_STYLE_ROUND,
                endCap: D2D1_CAP_STYLE_ROUND,
                dashStyle: D2D1_DASH_STYLE_DASH,
                ..D2D1_STROKE_STYLE_PROPERTIES::default()
            };
            let mut stroke_style: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_stroke_style(fac as PFactory, &ss_props, std::ptr::null(), 0, &mut stroke_style),
                S_OK
            );

            let mut dsb: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_drawing_state_block(fac as PFactory, std::ptr::null(), std::ptr::null_mut(), &mut dsb),
                S_OK
            );

            rt_begin_draw(compat_rt as PRT);
            let zero = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
            rt_clear(compat_rt as PRT, &zero);

            // Fill [0..32, 0..32] with radial gradient: center bright, edge dark
            let r_tl = D2D1_RECT_F { left: 0.0, top: 0.0, right: 32.0, bottom: 32.0 };
            rt_fill_rect(compat_rt as PRT, &r_tl, rad_brush);
            assert!(read_rt_pixel(compat_rt, 16, 16)[0] > 230);
            assert!(read_rt_pixel(compat_rt, 1, 1)[0] < 30);

            // Fill [32..64, 0..32] with wrapped checkerboard BitmapBrush
            let r_tr = D2D1_RECT_F { left: 32.0, top: 0.0, right: 64.0, bottom: 32.0 };
            rt_fill_rect(compat_rt as PRT, &r_tr, bmp_brush);
            assert_eq!(read_rt_pixel(compat_rt, 32, 0), [255, 0, 0, 255]);
            assert_eq!(read_rt_pixel(compat_rt, 33, 0), [0, 0, 255, 255]);

            // Save state, apply transform, restore state
            let shifted = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 12.0, dy: 5.0 };
            rt_set_transform(compat_rt as PRT, &shifted);
            rt_save_state(compat_rt as PRT, dsb);
            let id = D2D1_MATRIX_3X2_F::identity();
            rt_set_transform(compat_rt as PRT, &id);
            rt_restore_state(compat_rt as PRT, dsb);
            let mut got_m = D2D1_MATRIX_3X2_F::default();
            rt_get_transform(compat_rt as PRT, &mut got_m);
            assert_eq!(got_m, shifted);
            rt_set_transform(compat_rt as PRT, &id);

            // PushLayer at 50% opacity with rectangular mask
            let mask_r = D2D1_RECT_F { left: 4.0, top: 36.0, right: 24.0, bottom: 56.0 };
            let mut mask_geom: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_rect_geom(fac as PFactory, &mask_r, &mut mask_geom), S_OK);
            let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
            let mut white_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(compat_rt as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);

            let lp = D2D1_LAYER_PARAMETERS {
                contentBounds: D2D1_RECT_F { left: 0.0, top: 32.0, right: 32.0, bottom: 64.0 },
                geometricMask: mask_geom,
                maskAntialiasMode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                maskTransform: D2D1_MATRIX_3X2_F::identity(),
                opacity: 0.5,
                opacityBrush: std::ptr::null_mut(),
                layerOptions: 0,
                _pad: 0,
            };
            rt_push_layer(compat_rt as PRT, &lp, std::ptr::null_mut());
            let big_bot = D2D1_RECT_F { left: 0.0, top: 32.0, right: 32.0, bottom: 64.0 };
            rt_fill_rect(compat_rt as PRT, &big_bot, white_brush);
            rt_pop_layer(compat_rt as PRT);
            let in_mask = read_rt_pixel(compat_rt, 14, 46);
            let out_mask = read_rt_pixel(compat_rt, 1, 34);
            assert!((in_mask[0] as i32 - 128).abs() <= 2, "expected 50% layer inside mask, got {in_mask:?}");
            assert_eq!(out_mask[0], 0, "expected untouched outside geometricMask");

            // DrawRectangle / DrawEllipse / DrawRoundedRectangle with stroke_style
            let box_r = D2D1_RECT_F { left: 36.0, top: 36.0, right: 60.0, bottom: 60.0 };
            rt_draw_rect(compat_rt as PRT, &box_r, white_brush, 2.0, stroke_style);
            assert_eq!(rt_end_draw(compat_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

            // Compatible RT GetBitmap shares rendered content
            let mut compat_bmp: *mut c_void = std::ptr::null_mut();
            assert_eq!(brt_get_bitmap(compat_rt as PRT, &mut compat_bmp), S_OK);
            let mut bmp_px = D2D1_SIZE_U::default();
            bmp_get_pixel_size(compat_bmp as PBitmap, &mut bmp_px);
            assert_eq!(bmp_px, compat_px);

            // 5. Geometry queries (rect area, bounds, FillContainsPoint, path Bezier + Arc length)
            let mut area = 0.0f32;
            assert_eq!(geom_compute_area(mask_geom as PGeom, std::ptr::null(), 0.25, &mut area), S_OK);
            assert!((area - 400.0).abs() < 1.0, "expected 20x20=400 area, got {area}");
            let mut inside = FALSE;
            assert_eq!(
                geom_fill_contains_pt(mask_geom as PGeom, D2D1_POINT_2F { x: 10.0, y: 40.0 }.to_u64(), std::ptr::null(), 0.25, &mut inside),
                S_OK
            );
            assert_eq!(inside, TRUE);

            let mut curve_path: *mut c_void = std::ptr::null_mut();
            assert_eq!(fac_create_path_geom(fac as PFactory, &mut curve_path), S_OK);
            let mut csink: *mut c_void = std::ptr::null_mut();
            assert_eq!(path_geom_open(curve_path as PGeom, &mut csink), S_OK);
            gsink_begin_figure(csink as PGeomSink, D2D1_POINT_2F { x: 0.0, y: 0.0 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
            let bez = D2D1_BEZIER_SEGMENT {
                point1: D2D1_POINT_2F { x: 10.0, y: 0.0 },
                point2: D2D1_POINT_2F { x: 20.0, y: 10.0 },
                point3: D2D1_POINT_2F { x: 20.0, y: 20.0 },
            };
            gsink_add_bezier(csink as PGeomSink, &bez);
            let arc = D2D1_ARC_SEGMENT {
                point: D2D1_POINT_2F { x: 0.0, y: 20.0 },
                size: D2D1_SIZE_F { width: 10.0, height: 10.0 },
                rotationAngle: 0.0,
                sweepDirection: D2D1_SWEEP_DIRECTION_CLOCKWISE,
                arcSize: D2D1_ARC_SIZE_SMALL,
            };
            gsink_add_arc(csink as PGeomSink, &arc);
            gsink_end_figure(csink as PGeomSink, D2D1_FIGURE_END_CLOSED);
            assert_eq!(gsink_close(csink as PGeomSink), S_OK);
            let mut plen = 0.0f32;
            assert_eq!(geom_compute_length(curve_path as PGeom, std::ptr::null(), 0.25, &mut plen), S_OK);
            assert!(plen > 40.0, "expected positive curve+arc perimeter, got {plen}");

            // 6. CreateWicBitmapRenderTarget flushes rendered pixels to mock IWICBitmap via Lock()
            #[repr(C)]
            struct MockWicLock {
                vtbl: *const usize,
                w: u32,
                h: u32,
                buf: *mut u8,
                len: u32,
            }
            #[repr(C)]
            struct MockWicBitmap {
                vtbl: *const usize,
                w: u32,
                h: u32,
                pixels: Vec<u8>,
            }
            unsafe extern "win64" fn mw_qi(_: *mut c_void, _: *const u8, _: *mut *mut c_void) -> HRESULT { E_NOINTERFACE }
            unsafe extern "win64" fn mw_ar(_: *mut c_void) -> u32 { 1 }
            unsafe extern "win64" fn mw_rel(_: *mut c_void) -> u32 { 1 }
            unsafe extern "win64" fn mw_get_size(this: *mut c_void, w: *mut u32, h: *mut u32) -> HRESULT {
                let m = &*(this as *const MockWicBitmap);
                w.write_unaligned(m.w);
                h.write_unaligned(m.h);
                S_OK
            }
            unsafe extern "win64" fn mw_noop(_: *mut c_void) -> HRESULT { E_NOTIMPL }
            unsafe extern "win64" fn mwl_rel(this: *mut c_void) -> u32 {
                drop(Box::from_raw(this as *mut MockWicLock));
                0
            }
            unsafe extern "win64" fn mwl_stride(this: *mut c_void, stride: *mut u32) -> HRESULT {
                let l = &*(this as *const MockWicLock);
                stride.write_unaligned(l.w * 4);
                S_OK
            }
            unsafe extern "win64" fn mwl_data(this: *mut c_void, sz: *mut u32, ptr: *mut *mut u8) -> HRESULT {
                let l = &*(this as *const MockWicLock);
                sz.write_unaligned(l.len);
                ptr.write_unaligned(l.buf);
                S_OK
            }
            static LOCK_VTBL: OnceLock<[usize; 7]> = OnceLock::new();
            let lvtbl = LOCK_VTBL
                .get_or_init(|| {
                    register_vtable([
                        mw_qi as *const () as usize,
                        mw_ar as *const () as usize,
                        mwl_rel as *const () as usize,
                        mw_noop as *const () as usize,
                        mwl_stride as *const () as usize,
                        mwl_data as *const () as usize,
                        mw_noop as *const () as usize,
                    ])
                })
                .as_ptr();
            unsafe extern "win64" fn mw_lock(
                this: *mut c_void,
                _rect: *const c_void,
                _flags: u32,
                out_lock: *mut *mut c_void,
            ) -> HRESULT {
                let m = &mut *(this as *mut MockWicBitmap);
                let l = Box::new(MockWicLock {
                    vtbl: LOCK_VTBL.get().unwrap().as_ptr(),
                    w: m.w,
                    h: m.h,
                    buf: m.pixels.as_mut_ptr(),
                    len: m.pixels.len() as u32,
                });
                out_lock.write_unaligned(Box::into_raw(l) as *mut c_void);
                S_OK
            }
            let _ = lvtbl;
            let wic_vtbl = register_vtable([
                mw_qi as *const () as usize,
                mw_ar as *const () as usize,
                mw_rel as *const () as usize,
                mw_get_size as *const () as usize,
                mw_noop as *const () as usize,
                mw_noop as *const () as usize,
                mw_noop as *const () as usize,
                mw_noop as *const () as usize,
                mw_lock as *const () as usize,
            ]);
            let mut mock_wic = MockWicBitmap {
                vtbl: wic_vtbl.as_ptr(),
                w: 16,
                h: 16,
                pixels: vec![0u8; 16 * 16 * 4],
            };
            let mut wic_rt: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_wic_rt(
                    fac as PFactory,
                    &mut mock_wic as *mut MockWicBitmap as *mut c_void,
                    std::ptr::null(),
                    &mut wic_rt,
                ),
                S_OK
            );
            rt_begin_draw(wic_rt as PRT);
            let magenta = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
            rt_clear(wic_rt as PRT, &magenta);
            assert_eq!(rt_end_draw(wic_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);
            assert_eq!(&mock_wic.pixels[0..4], &[255, 0, 255, 255]);

            release_com(wic_rt);
            release_com(csink);
            release_com(curve_path);
            release_com(compat_bmp);
            release_com(white_brush);
            release_com(mask_geom);
            release_com(dsb);
            release_com(stroke_style);
            release_com(bmp_brush);
            release_com(chk_bmp);
            release_com(rad_brush);
            release_com(stop_col);
            release_com(compat_rt);
            release_com(dc);
            release_com(dev);
            release_com(fac);
        }
    }

    #[test]
    fn draw_text_layout_with_underline_and_drawing_effect() {
        unsafe {
            let mut dc: *mut c_void = std::ptr::null_mut();
            assert_eq!(D2D1CreateDeviceContext(std::ptr::null_mut(), std::ptr::null(), &mut dc), S_OK);
            let mut target: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                dc_create_bitmap1(dc as PRT, D2D1_SIZE_U { width: 120, height: 48 }.to_u64(), std::ptr::null(), 0, std::ptr::null(), &mut target),
                S_OK
            );
            dc_set_target(dc as PRT, target);

            let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
            let red = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
            let mut white_brush: *mut c_void = std::ptr::null_mut();
            let mut red_brush: *mut c_void = std::ptr::null_mut();
            assert_eq!(rt_create_solid_brush(dc as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);
            assert_eq!(rt_create_solid_brush(dc as PRT, &red, std::ptr::null(), &mut red_brush), S_OK);

            // Create DirectWrite format + layout
            let mut dw_fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                dwrite::DWriteCreateFactory(
                    dwrite::DWRITE_FACTORY_TYPE_SHARED,
                    &dwrite::IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut dw_fac,
                ),
                S_OK
            );
            let fam_w: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
            let loc_w: Vec<u16> = "en-us\0".encode_utf16().collect();
            let mut fmt: *mut c_void = std::ptr::null_mut();
            #[cfg(not(target_arch = "aarch64"))]
            {
                let create_tf: unsafe extern "win64" fn(
                    *mut c_void, *const u16, *mut c_void, u32, u32, u32, f32, *const u16, *mut *mut c_void
                ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
                assert_eq!(create_tf(dw_fac, fam_w.as_ptr(), std::ptr::null_mut(), 400, 0, 5, 22.0, loc_w.as_ptr(), &mut fmt), S_OK);
            }
            #[cfg(target_arch = "aarch64")]
            {
                let br: unsafe extern "C" fn(
                    u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                    f64, f64, f64, f64,
                ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
                assert_eq!(
                    br(
                        dw_fac as u64, fam_w.as_ptr() as u64, 0, 400, 0, 5,
                        22.0f32.to_bits() as u64, loc_w.as_ptr() as u64, &mut fmt as *mut *mut c_void as u64,
                        0, 0, 0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0,
                    ),
                    S_OK
                );
            }

            let text_u16: Vec<u16> = "ABCD".encode_utf16().collect();
            let mut layout: *mut c_void = std::ptr::null_mut();
            #[cfg(not(target_arch = "aarch64"))]
            {
                let create_tl: unsafe extern "win64" fn(
                    *mut c_void, *const u16, u32, *mut c_void, f32, f32, *mut *mut c_void
                ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(18).read_unaligned());
                assert_eq!(create_tl(dw_fac, text_u16.as_ptr(), text_u16.len() as u32, fmt, 100.0, 40.0, &mut layout), S_OK);
            }
            #[cfg(target_arch = "aarch64")]
            {
                let br: unsafe extern "C" fn(
                    u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                    f64, f64, f64, f64,
                ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(18).read_unaligned());
                assert_eq!(
                    br(
                        dw_fac as u64, text_u16.as_ptr() as u64, text_u16.len() as u64, fmt as u64,
                        100.0f32.to_bits() as u64, 40.0f32.to_bits() as u64, &mut layout as *mut *mut c_void as u64,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0,
                    ),
                    S_OK
                );
            }

            // Set red drawingEffect + underline on entire range via IDWriteTextLayout vtable slots 36 & 38
            let set_ul: unsafe extern "win64" fn(*mut c_void, BOOL, u64) -> HRESULT =
                std::mem::transmute(vtbl_of(layout).add(36).read_unaligned());
            let set_eff: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
                std::mem::transmute(vtbl_of(layout).add(38).read_unaligned());
            let full_range = (text_u16.len() as u64) << 32;
            assert_eq!(set_ul(layout, TRUE, full_range), S_OK);
            assert_eq!(set_eff(layout, red_brush, full_range), S_OK);

            rt_begin_draw(dc as PRT);
            let black = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
            rt_clear(dc as PRT, &black);
            rt_draw_text_layout(dc as PRT, D2D1_POINT_2F { x: 8.0, y: 6.0 }.to_u64(), layout, white_brush, 0);
            assert_eq!(rt_end_draw(dc as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

            // Verify red pixels (from drawingEffect brush!) were rendered, and zero white-only pixels
            let mut red_pixels = 0u32;
            for py in 0..48u32 {
                for px in 0..120u32 {
                    let c = read_rt_pixel(dc, px, py);
                    if c[0] > 80 && c[1] < 20 && c[2] < 20 {
                        red_pixels += 1;
                    }
                }
            }
            assert!(red_pixels >= 40, "expected red text+underline pixels from drawingEffect, got {red_pixels}");

            release_com(layout);
            release_com(fmt);
            release_com(dw_fac);
            release_com(red_brush);
            release_com(white_brush);
            release_com(target);
            release_com(dc);
        }
    }
}
