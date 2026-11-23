//! d2d1.dll: Native Direct2D 1.0 / 1.1 COM runtime & software RGBA32 rasterizer
//! integrated with DirectWrite (`dwrite.rs`) and GDI (`gdi32.rs`).

mod abi;
mod geometry;
mod raster;
mod render_target;

pub use abi::*;

use crate::com::{
    com_add_ref, com_call_release, com_release, qi_helper_bytes, read_guid_from_bytes,
    register_vtable_once, write_null_out, ComObj, ComPtr, IID_IUnknown,
};
use crate::types::*;
use geometry::{
    drawing_state_block_vtbl, ellipse_geom_vtbl, geom_group_vtbl, path_geom_vtbl, rect_geom_vtbl,
    rounded_rect_geom_vtbl, stroke_style_vtbl, transformed_geom_vtbl, DrawingStateBlockInner,
    GeometryInner, GeometryKind, PathData, StrokeStyleData, StrokeStyleInner,
};
use raster::Surface;
use render_target::{
    dev_create_dc, device_vtbl, new_render_target_com, DeviceInner, PDevice, RenderTargetKind,
};
use std::ffi::c_void;
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
// ID2D1Factory / ID2D1Factory1 Vtable (27 slots)
// ============================================================================

struct FactoryInner {
    _factory_type: u32,
}

type PFactory = *mut ComObj<FactoryInner>;

unsafe extern "win64" fn fac_qi(this: PFactory, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_FACTORY, G_ID2D1_FACTORY1])
}
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
    let obj = ComObj::new(
        rect_geom_vtbl(),
        GeometryInner { factory: fac, kind: GeometryKind::Rect(rect.read_unaligned()) },
    );
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
    let dev = ComObj::new(
        device_vtbl(),
        DeviceInner { factory: fac, max_tex_mem: Mutex::new(256 * 1024 * 1024) },
    );
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
unsafe extern "win64" fn fac_notimpl_effect(
    _: PFactory,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
) -> HRESULT {
    E_NOTIMPL
}
unsafe extern "win64" fn fac_unreg_effect(_: PFactory, _: *const u8) -> HRESULT { E_NOTIMPL }
unsafe extern "win64" fn fac_get_reg_effects(
    _: PFactory,
    _: *mut u8,
    _: u32,
    ret: *mut u32,
    reg: *mut u32,
) -> HRESULT {
    if !ret.is_null() { ret.write_unaligned(0); }
    if !reg.is_null() { reg.write_unaligned(0); }
    S_OK
}
unsafe extern "win64" fn fac_get_eff_props(_: PFactory, _: *const u8, out: *mut *mut c_void) -> HRESULT {
    write_null_out(out);
    E_NOTIMPL
}

fn factory_vtbl() -> *const usize {
    static V: OnceLock<[usize; 27]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            fac_qi as *const () as usize,                         // 0
            com_add_ref::<FactoryInner> as *const () as usize,    // 1
            com_release::<FactoryInner> as *const () as usize,    // 2
            fac_reload_metrics as *const () as usize,             // 3
            fac_get_desktop_dpi as *const () as usize,            // 4
            fac_create_rect_geom as *const () as usize,           // 5
            fac_create_rrect_geom as *const () as usize,          // 6
            fac_create_ellipse_geom as *const () as usize,        // 7
            fac_create_geom_group as *const () as usize,          // 8
            fac_create_transformed_geom as *const () as usize,    // 9
            fac_create_path_geom as *const () as usize,           // 10
            fac_create_stroke_style as *const () as usize,        // 11
            fac_create_drawing_state_block as *const () as usize, // 12
            fac_create_wic_rt as *const () as usize,              // 13
            fac_create_hwnd_rt as *const () as usize,             // 14
            fac_create_dxgi_rt as *const () as usize,             // 15
            fac_create_dc_rt as *const () as usize,               // 16
            fac_create_device as *const () as usize,              // 17 (ID2D1Factory1)
            fac_create_stroke_style1 as *const () as usize,       // 18
            fac_create_path_geom as *const () as usize,           // 19
            fac_create_drawing_state_block as *const () as usize, // 20
            fac_get_eff_props as *const () as usize,              // 21 CreateGdiMetafile
            fac_notimpl_effect as *const () as usize,             // 22 RegisterEffectFromStream
            fac_notimpl_effect as *const () as usize,             // 23 RegisterEffectFromString
            fac_unreg_effect as *const () as usize,               // 24 UnregisterEffect
            fac_get_reg_effects as *const () as usize,            // 25 GetRegisteredEffects
            fac_get_eff_props as *const () as usize,              // 26 GetEffectProperties
        ]
    })
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
    write_null_out(factory);
    if factory_type != D2D1_FACTORY_TYPE_SINGLE_THREADED && factory_type != D2D1_FACTORY_TYPE_MULTI_THREADED {
        return E_INVALIDARG;
    }
    let Some(req) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    if req != IID_IUnknown && req != G_ID2D1_FACTORY && req != G_ID2D1_FACTORY1 {
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
    write_null_out(d2d_device);
    let fac = new_factory_com(D2D1_FACTORY_TYPE_MULTI_THREADED);
    let hr = fac_create_device(fac as PFactory, dxgi_device, d2d_device);
    com_call_release(fac);
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
    write_null_out(d2d_dc);
    let mut dev: *mut c_void = std::ptr::null_mut();
    let hr = D2D1CreateDevice(dxgi_surface, props, &mut dev);
    if hr < 0 {
        return hr;
    }
    let hr2 = dev_create_dc(dev as PDevice, 0, d2d_dc);
    com_call_release(dev);
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
mod tests;
