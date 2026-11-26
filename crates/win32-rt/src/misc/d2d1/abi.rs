//! Direct2D 1.0 / 1.1 ABI constants, HRESULTs, IIDs, and `#[repr(C)]` structures (`d2d1.h` / `d2d1_1.h`).

use crate::com::{guid, Guid, IID_IUnknown};
use crate::types::*;
use std::ffi::c_void;

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

pub(super) const G_ID2D1_FACTORY: Guid = guid("06152247-6f50-465a-9245-118bfd3b6007");
pub(super) const G_ID2D1_FACTORY1: Guid = guid("bb12d362-daee-4b9a-aa1d-14ba401cfa1f");
pub(super) const G_ID2D1_RESOURCE: Guid = guid("2cd90691-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_IMAGE: Guid = guid("65019f75-8da2-497c-b32c-dfa34e48ede6");
pub(super) const G_ID2D1_BITMAP: Guid = guid("a2296057-ea42-4099-983b-539fb6505426");
pub(super) const G_ID2D1_BITMAP1: Guid = guid("a898a84c-3873-4588-b08b-ebbf978df041");
pub(super) const G_ID2D1_GRADIENT_STOP_COLLECTION: Guid =
    guid("2cd906a7-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_BRUSH: Guid = guid("2cd906a8-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_SOLID_COLOR_BRUSH: Guid = guid("2cd906a9-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_BITMAP_BRUSH: Guid = guid("2cd906aa-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_LINEAR_GRADIENT_BRUSH: Guid =
    guid("2cd906ab-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_RADIAL_GRADIENT_BRUSH: Guid =
    guid("2cd906ac-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_STROKE_STYLE: Guid = guid("2cd9069d-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_STROKE_STYLE1: Guid = guid("10a72a66-e91c-43f4-993f-ddf4b82b0b4a");
pub(super) const G_ID2D1_GEOMETRY: Guid = guid("2cd906a1-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_RECTANGLE_GEOMETRY: Guid = guid("2cd906a2-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_ROUNDED_RECTANGLE_GEOMETRY: Guid =
    guid("2cd906a3-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_ELLIPSE_GEOMETRY: Guid = guid("2cd906a4-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_PATH_GEOMETRY: Guid = guid("2cd906a5-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_PATH_GEOMETRY1: Guid = guid("62baa2d2-ab54-41b7-b872-787e0106a421");
pub(super) const G_ID2D1_GEOMETRY_GROUP: Guid = guid("2cd906a6-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_TRANSFORMED_GEOMETRY: Guid = guid("2cd906bb-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_SIMPLIFIED_GEOMETRY_SINK: Guid =
    guid("2cd9069e-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_GEOMETRY_SINK: Guid = guid("2cd9069f-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_MESH: Guid = guid("2cd906c2-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_LAYER: Guid = guid("2cd9069b-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_DRAWING_STATE_BLOCK: Guid = guid("28506e39-ebf6-46a1-bb47-fd85565ab957");
pub(super) const G_ID2D1_RENDER_TARGET: Guid = guid("2cd90694-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_BITMAP_RENDER_TARGET: Guid = guid("2cd90695-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_HWND_RENDER_TARGET: Guid = guid("2cd90698-12e2-11dc-9fed-001143a055f9");
pub(super) const G_ID2D1_DC_RENDER_TARGET: Guid = guid("1c51bc64-de61-46fd-9899-63a5d8f03950");
pub(super) const G_ID2D1_DEVICE: Guid = guid("47dd575d-ac05-4cdd-8049-9b02cd16f44c");
pub(super) const G_ID2D1_DEVICE_CONTEXT: Guid = guid("e8f7fe7a-191c-466d-ad95-975678bda998");

pub const IID_IUNKNOWN: GUID = IID_IUnknown.to_win_guid();
pub const IID_ID2D1_FACTORY: GUID = G_ID2D1_FACTORY.to_win_guid();
pub const IID_ID2D1_FACTORY1: GUID = G_ID2D1_FACTORY1.to_win_guid();
pub const IID_ID2D1_RESOURCE: GUID = G_ID2D1_RESOURCE.to_win_guid();
pub const IID_ID2D1_IMAGE: GUID = G_ID2D1_IMAGE.to_win_guid();
pub const IID_ID2D1_BITMAP: GUID = G_ID2D1_BITMAP.to_win_guid();
pub const IID_ID2D1_BITMAP1: GUID = G_ID2D1_BITMAP1.to_win_guid();
pub const IID_ID2D1_GRADIENT_STOP_COLLECTION: GUID = G_ID2D1_GRADIENT_STOP_COLLECTION.to_win_guid();
pub const IID_ID2D1_BRUSH: GUID = G_ID2D1_BRUSH.to_win_guid();
pub const IID_ID2D1_SOLID_COLOR_BRUSH: GUID = G_ID2D1_SOLID_COLOR_BRUSH.to_win_guid();
pub const IID_ID2D1_BITMAP_BRUSH: GUID = G_ID2D1_BITMAP_BRUSH.to_win_guid();
pub const IID_ID2D1_LINEAR_GRADIENT_BRUSH: GUID = G_ID2D1_LINEAR_GRADIENT_BRUSH.to_win_guid();
pub const IID_ID2D1_RADIAL_GRADIENT_BRUSH: GUID = G_ID2D1_RADIAL_GRADIENT_BRUSH.to_win_guid();
pub const IID_ID2D1_STROKE_STYLE: GUID = G_ID2D1_STROKE_STYLE.to_win_guid();
pub const IID_ID2D1_STROKE_STYLE1: GUID = G_ID2D1_STROKE_STYLE1.to_win_guid();
pub const IID_ID2D1_GEOMETRY: GUID = G_ID2D1_GEOMETRY.to_win_guid();
pub const IID_ID2D1_RECTANGLE_GEOMETRY: GUID = G_ID2D1_RECTANGLE_GEOMETRY.to_win_guid();
pub const IID_ID2D1_ROUNDED_RECTANGLE_GEOMETRY: GUID =
    G_ID2D1_ROUNDED_RECTANGLE_GEOMETRY.to_win_guid();
pub const IID_ID2D1_ELLIPSE_GEOMETRY: GUID = G_ID2D1_ELLIPSE_GEOMETRY.to_win_guid();
pub const IID_ID2D1_PATH_GEOMETRY: GUID = G_ID2D1_PATH_GEOMETRY.to_win_guid();
pub const IID_ID2D1_PATH_GEOMETRY1: GUID = G_ID2D1_PATH_GEOMETRY1.to_win_guid();
pub const IID_ID2D1_GEOMETRY_GROUP: GUID = G_ID2D1_GEOMETRY_GROUP.to_win_guid();
pub const IID_ID2D1_TRANSFORMED_GEOMETRY: GUID = G_ID2D1_TRANSFORMED_GEOMETRY.to_win_guid();
pub const IID_ID2D1_SIMPLIFIED_GEOMETRY_SINK: GUID = G_ID2D1_SIMPLIFIED_GEOMETRY_SINK.to_win_guid();
pub const IID_ID2D1_GEOMETRY_SINK: GUID = G_ID2D1_GEOMETRY_SINK.to_win_guid();
pub const IID_ID2D1_MESH: GUID = G_ID2D1_MESH.to_win_guid();
pub const IID_ID2D1_LAYER: GUID = G_ID2D1_LAYER.to_win_guid();
pub const IID_ID2D1_DRAWING_STATE_BLOCK: GUID = G_ID2D1_DRAWING_STATE_BLOCK.to_win_guid();
pub const IID_ID2D1_RENDER_TARGET: GUID = G_ID2D1_RENDER_TARGET.to_win_guid();
pub const IID_ID2D1_BITMAP_RENDER_TARGET: GUID = G_ID2D1_BITMAP_RENDER_TARGET.to_win_guid();
pub const IID_ID2D1_HWND_RENDER_TARGET: GUID = G_ID2D1_HWND_RENDER_TARGET.to_win_guid();
pub const IID_ID2D1_DC_RENDER_TARGET: GUID = G_ID2D1_DC_RENDER_TARGET.to_win_guid();
pub const IID_ID2D1_DEVICE: GUID = G_ID2D1_DEVICE.to_win_guid();
pub const IID_ID2D1_DEVICE_CONTEXT: GUID = G_ID2D1_DEVICE_CONTEXT.to_win_guid();

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_TRIANGLE {
    pub point1: D2D1_POINT_2F,
    pub point2: D2D1_POINT_2F,
    pub point3: D2D1_POINT_2F,
}
const _: () = assert!(std::mem::size_of::<D2D1_TRIANGLE>() == 24);

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
