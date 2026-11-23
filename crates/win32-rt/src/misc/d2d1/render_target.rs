//! Direct2D `ID2D1RenderTarget`, `ID2D1BitmapRenderTarget`, `ID2D1HwndRenderTarget`,
//! `ID2D1DCRenderTarget`, `ID2D1DeviceContext`, and `ID2D1Device`.

use super::abi::*;
use super::geometry::{
    drawing_state_block_from_ptr, ellipse_subpath, expand_strokes, geometry_from_ptr, layer_vtbl,
    mesh_vtbl, point_in_subpaths, rect_subpath, rounded_rect_subpath, stroke_style_from_ptr,
    subpaths_bounds, transform_subpaths, LayerInner, MeshInner, SubPath,
};
use super::raster::{
    bitmap_brush_vtbl, bitmap_from_ptr, brush_from_ptr, grad_stops_from_ptr, grad_stops_vtbl,
    linear_brush_vtbl, new_bitmap_com, radial_brush_vtbl, rasterize_polygons, solid_brush_vtbl,
    BrushEval, BrushInner, BrushKind, BrushState, GradientStopCollectionData,
    GradientStopCollectionInner, Surface,
};
use crate::com::{
    com_add_ref, com_call_add_ref, com_call_release, com_release, qi_helper_bytes,
    read_guid_from_bytes, register_vtable_once, write_null_out, ComObj, ComPtr,
};
use crate::misc::dwrite::{self, DWRITE_GLYPH_OFFSET, DWRITE_GLYPH_RUN};
use crate::strings::read_wide_slice;
use crate::types::*;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

// ============================================================================
// RenderTarget & Device Internal State
// ============================================================================

pub(super) struct DeviceInner {
    pub(super) factory: ComPtr,
    pub(super) max_tex_mem: Mutex<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RenderTargetKind {
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

pub(super) struct RenderTargetState {
    pub(super) surface: Arc<Mutex<Surface>>,
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
                            0, // entire bitmap
                            2, // WICBitmapLockWrite
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
                                    crate::call_guest8(
                                        stride_fn,
                                        lock_obj as u64,
                                        &mut stride as *mut u32 as u64,
                                        0, 0, 0, 0, 0, 0,
                                    );
                                    crate::call_guest8(
                                        data_fn,
                                        lock_obj as u64,
                                        &mut buf_sz as *mut u32 as u64,
                                        &mut data_ptr as *mut *mut u8 as u64,
                                        0, 0, 0, 0, 0,
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
                            com_call_release(lock_obj);
                        }
                    }
                }
            }
        }
    }
}

pub(super) struct RenderTargetInner {
    factory: ComPtr,
    kind: RenderTargetKind,
    pub(super) state: Mutex<RenderTargetState>,
}

pub(super) unsafe fn render_target_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a RenderTargetInner> {
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
// ID2D1RenderTarget (57) + Bitmap/Hwnd/DC RenderTargets + ID2D1DeviceContext (92)
// ============================================================================

pub(super) type PRT = *mut ComObj<RenderTargetInner>;

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
        RenderTargetKind::Generic => {
            qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_RENDER_TARGET])
        }
        RenderTargetKind::Bitmap => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_RENDER_TARGET, G_ID2D1_BITMAP_RENDER_TARGET],
        ),
        RenderTargetKind::Hwnd => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_RENDER_TARGET, G_ID2D1_HWND_RENDER_TARGET],
        ),
        RenderTargetKind::Dc => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_RENDER_TARGET, G_ID2D1_DC_RENDER_TARGET],
        ),
        RenderTargetKind::DeviceContext => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_RENDER_TARGET, G_ID2D1_DEVICE_CONTEXT],
        ),
    }
}
unsafe extern "win64" fn rt_get_factory(this: PRT, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
    out.write_unaligned(f);
}

// Slot 4: CreateBitmap
pub(super) unsafe extern "win64" fn rt_create_bitmap(
    this: PRT,
    size_u64: u64,
    src_data: *const c_void,
    pitch: u32,
    props: *const D2D1_BITMAP_PROPERTIES,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    write_null_out(out);
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
    write_null_out(out);
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
    write_null_out(out);
    if let Some(g) = read_guid_from_bytes(riid) {
        if g == G_ID2D1_BITMAP || g == G_ID2D1_BITMAP1 {
            if let Some(b) = bitmap_from_ptr(data) {
                out.write_unaligned(new_bitmap_com(
                    ComObj::inner(this).factory.clone(),
                    Arc::clone(&b.surface),
                    b.options,
                ));
                return S_OK;
            }
        }
    }
    rt_create_bitmap_from_wic(this, data, props, out)
}

// Slot 7: CreateBitmapBrush
pub(super) unsafe extern "win64" fn rt_create_bitmap_brush(
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
pub(super) unsafe extern "win64" fn rt_create_solid_brush(
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
pub(super) unsafe extern "win64" fn rt_create_grad_stops(
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
pub(super) unsafe extern "win64" fn rt_create_linear_brush(
    this: PRT,
    lg_props: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    stops_com: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || lg_props.is_null() || stops_com.is_null() || out.is_null() { return E_POINTER; }
    let Some(gsc) = grad_stops_from_ptr(stops_com) else { return E_INVALIDARG; };
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let lg = lg_props.read_unaligned();
    let stops = Arc::clone(&gsc.data);
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
pub(super) unsafe extern "win64" fn rt_create_radial_brush(
    this: PRT,
    rg_props: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
    b_props: *const D2D1_BRUSH_PROPERTIES,
    stops_com: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || rg_props.is_null() || stops_com.is_null() || out.is_null() { return E_POINTER; }
    let Some(gsc) = grad_stops_from_ptr(stops_com) else { return E_INVALIDARG; };
    let bp = if b_props.is_null() { D2D1_BRUSH_PROPERTIES::default() } else { b_props.read_unaligned() };
    let rg = rg_props.read_unaligned();
    let stops = Arc::clone(&gsc.data);
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
pub(super) unsafe extern "win64" fn rt_create_compat_rt(
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
    let bmp_ref = ComPtr::from_raw(bmp_ptr);
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
pub(super) unsafe extern "win64" fn rt_draw_line(
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
pub(super) unsafe extern "win64" fn rt_draw_rect(
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
pub(super) unsafe extern "win64" fn rt_fill_rect(this: PRT, rect: *const D2D1_RECT_F, brush: *mut c_void) {
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
pub(super) unsafe extern "win64" fn rt_fill_rounded_rect(
    this: PRT,
    rrect: *const D2D1_ROUNDED_RECT,
    brush: *mut c_void,
) {
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
pub(super) unsafe extern "win64" fn rt_fill_ellipse(
    this: PRT,
    ellipse: *const D2D1_ELLIPSE,
    brush: *mut c_void,
) {
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
pub(super) unsafe extern "win64" fn rt_fill_geometry(
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
        let ma = mask_surf.sample(
            sr.left + u * sw,
            sr.top + v * sh,
            D2D1_EXTEND_MODE_CLAMP,
            D2D1_EXTEND_MODE_CLAMP,
            D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
        )[3];
        let mut c = brush_eval.sample_world(wx, wy);
        c[3] *= ma;
        c
    });
}

// Slot 26: DrawBitmap
pub(super) unsafe extern "win64" fn rt_draw_bitmap(
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
        let mut c = src_surf.sample(
            sr.left + u * sw,
            sr.top + v * sh,
            D2D1_EXTEND_MODE_CLAMP,
            D2D1_EXTEND_MODE_CLAMP,
            interp,
        );
        c[3] *= op;
        c
    });
}
crate::win64_bridge!(fn rt_draw_bitmap(
    PRT, *mut c_void, *const D2D1_RECT_F, f32, u32, *const D2D1_RECT_F
) -> ());

// Slot 27: DrawText (integrates directly with dwrite.rs!)
pub(super) unsafe extern "win64" fn rt_draw_text(
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
    let units = read_wide_slice(string, string_len as usize);
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
pub(super) unsafe extern "win64" fn rt_draw_text_layout(
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
        advs.push(if run.glyphAdvances.is_null() {
            run.fontEmSize * 0.5
        } else {
            run.glyphAdvances.add(i).read_unaligned()
        });
        offs.push(if run.glyphOffsets.is_null() {
            DWRITE_GLYPH_OFFSET::default()
        } else {
            run.glyphOffsets.add(i).read_unaligned()
        });
    }
    let origin = D2D1_POINT_2F::from_u64(origin_u64);
    let st = ComObj::inner(this).state.lock().unwrap();
    render_glyph_run_on_rt(&st, &face, run.fontEmSize, origin.x, origin.y, &gids, &advs, &offs, &b.snapshot());
}

// Slots 30..49: Transform, AA, Tags, Layers, Clips, Clear, BeginDraw/EndDraw
pub(super) unsafe extern "win64" fn rt_set_transform(this: PRT, m: *const D2D1_MATRIX_3X2_F) {
    if this.is_null() { return; }
    let mat = if m.is_null() { D2D1_MATRIX_3X2_F::identity() } else { m.read_unaligned() };
    ComObj::inner(this).state.lock().unwrap().transform = mat;
}
pub(super) unsafe extern "win64" fn rt_get_transform(this: PRT, out: *mut D2D1_MATRIX_3X2_F) {
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
        com_call_add_ref(p.as_ptr());
        out.write_unaligned(p.as_ptr());
    } else {
        write_null_out(out);
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
pub(super) unsafe extern "win64" fn rt_push_layer(
    this: PRT,
    params: *const D2D1_LAYER_PARAMETERS,
    _layer: *mut c_void,
) {
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
pub(super) unsafe extern "win64" fn rt_pop_layer(this: PRT) {
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
pub(super) unsafe extern "win64" fn rt_save_state(this: PRT, block: *mut c_void) {
    if this.is_null() { return; }
    let Some(dsb) = drawing_state_block_from_ptr(block) else { return; };
    let st = ComObj::inner(this).state.lock().unwrap();
    let mut dst = dsb.state.lock().unwrap();
    dst.0 = D2D1_DRAWING_STATE_DESCRIPTION {
        antialiasMode: st.antialias_mode,
        textAntialiasMode: st.text_antialias_mode,
        tag1: st.tag1,
        tag2: st.tag2,
        transform: st.transform,
    };
    dst.1 = st.text_rendering_params.clone();
}
pub(super) unsafe extern "win64" fn rt_restore_state(this: PRT, block: *mut c_void) {
    if this.is_null() { return; }
    let Some(dsb) = drawing_state_block_from_ptr(block) else { return; };
    let src = dsb.state.lock().unwrap().clone();
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.antialias_mode = src.0.antialiasMode;
    st.text_antialias_mode = src.0.textAntialiasMode;
    st.tag1 = src.0.tag1;
    st.tag2 = src.0.tag2;
    st.transform = src.0.transform;
    st.text_rendering_params = src.1;
}
pub(super) unsafe extern "win64" fn rt_push_clip(this: PRT, clip_rect: *const D2D1_RECT_F, _aa: u32) {
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
pub(super) unsafe extern "win64" fn rt_pop_clip(this: PRT) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().clip_stack.pop();
}
pub(super) unsafe extern "win64" fn rt_clear(this: PRT, clear_color: *const D2D1_COLOR_F) {
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
pub(super) unsafe extern "win64" fn rt_begin_draw(this: PRT) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().in_draw = true;
}
pub(super) unsafe extern "win64" fn rt_end_draw(this: PRT, t1: *mut u64, t2: *mut u64) -> HRESULT {
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
    let st = ComObj::inner(this).state.lock().unwrap();
    let mut surf = st.surface.lock().unwrap();
    if dpi_x > 0.0 && dpi_x.is_finite() { surf.dpi_x = dpi_x; }
    if dpi_y > 0.0 && dpi_y.is_finite() { surf.dpi_y = dpi_y; }
    if dpi_x == 0.0 && dpi_y == 0.0 {
        surf.dpi_x = 96.0;
        surf.dpi_y = 96.0;
    }
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
pub(super) unsafe extern "win64" fn rt_get_pixel_size(this: PRT, ret: *mut D2D1_SIZE_U) -> *mut D2D1_SIZE_U {
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
            com_add_ref::<RenderTargetInner> as *const () as usize,      // 1
            com_release::<RenderTargetInner> as *const () as usize,      // 2
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
    register_vtable_once(&V, || rt_base_slots!())
}

// ID2D1BitmapRenderTarget (58 slots)
pub(super) unsafe extern "win64" fn brt_get_bitmap(this: PRT, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref b) = st.target_bitmap {
        com_call_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
        S_OK
    } else {
        out.write_unaligned(new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&st.surface), 0));
        S_OK
    }
}
fn rt_bitmap_vtbl() -> *const usize {
    static V: OnceLock<[usize; 58]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = rt_base_slots!();
        let mut t = [0usize; 58];
        t[..57].copy_from_slice(&b);
        t[57] = brt_get_bitmap as *const () as usize;
        t
    })
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
    register_vtable_once(&V, || {
        let b = rt_base_slots!();
        let mut t = [0usize; 60];
        t[..57].copy_from_slice(&b);
        t[57] = hwnd_rt_check_window_state as *const () as usize;
        t[58] = hwnd_rt_resize as *const () as usize;
        t[59] = hwnd_rt_get_hwnd as *const () as usize;
        t
    })
}

// ID2D1DCRenderTarget (58 slots)
pub(super) unsafe extern "win64" fn dc_rt_bind_dc(this: PRT, hdc: usize, sub_rect: *const [i32; 4]) -> HRESULT {
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
    register_vtable_once(&V, || {
        let b = rt_base_slots!();
        let mut t = [0usize; 58];
        t[..57].copy_from_slice(&b);
        t[57] = dc_rt_bind_dc as *const () as usize;
        t
    })
}

// ID2D1DeviceContext extensions (slots 57..91)
pub(super) unsafe extern "win64" fn dc_create_bitmap1(
    this: PRT,
    size_u64: u64,
    src_data: *const c_void,
    pitch: u32,
    props1: *const D2D1_BITMAP_PROPERTIES1,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    write_null_out(out);
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
    write_null_out(out);
    E_NOTIMPL
}
unsafe extern "win64" fn dc_notimpl_2(_: PRT, _: usize, out: *mut *mut c_void) -> HRESULT {
    write_null_out(out);
    E_NOTIMPL
}
unsafe extern "win64" fn dc_notimpl_1(_: PRT, out: *mut *mut c_void) -> HRESULT {
    write_null_out(out);
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
    if matches!(
        fmt,
        DXGI_FORMAT_UNKNOWN | DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_A8_UNORM
    ) {
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
        w += if run.glyphAdvances.is_null() {
            run.fontEmSize * 0.5
        } else {
            run.glyphAdvances.add(i).read_unaligned()
        };
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
        com_call_add_ref(d.as_ptr());
        out.write_unaligned(d.as_ptr());
    } else {
        write_null_out(out);
    }
}
pub(super) unsafe extern "win64" fn dc_set_target(this: PRT, image: *mut c_void) {
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
        com_call_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
    } else {
        write_null_out(out);
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
    _: PRT, _: *mut c_void, _: *const D2D1_RECT_F, _: *const c_void, _: *mut D2D1_RECT_F, _: u32,
) -> HRESULT {
    S_OK
}
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
    register_vtable_once(&V, || {
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
        t
    })
}

pub(super) fn new_render_target_com(
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

pub(super) type PDevice = *mut ComObj<DeviceInner>;

unsafe extern "win64" fn dev_qi(this: PDevice, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_DEVICE])
}
unsafe extern "win64" fn dev_get_factory(this: PDevice, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
    out.write_unaligned(f);
}
pub(super) unsafe extern "win64" fn dev_create_dc(this: PDevice, _opts: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let surf = Arc::new(Mutex::new(Surface::new(512, 512, D2D1_PIXEL_FORMAT::default(), 96.0, 96.0)));
    let bmp_ptr = new_bitmap_com(ComObj::inner(this).factory.clone(), Arc::clone(&surf), 0);
    let bmp_ref = ComPtr::from_raw(bmp_ptr);
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
    _this: PDevice,
    _wic: *mut c_void,
    _doc: *mut c_void,
    _props: *const c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out);
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

pub(super) fn device_vtbl() -> *const usize {
    static V: OnceLock<[usize; 9]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            dev_qi as *const () as usize,
            com_add_ref::<DeviceInner> as *const () as usize,
            com_release::<DeviceInner> as *const () as usize,
            dev_get_factory as *const () as usize,
            dev_create_dc as *const () as usize,
            dev_create_print_control as *const () as usize,
            dev_set_max_tex_mem as *const () as usize,
            dev_get_max_tex_mem as *const () as usize,
            dev_clear_resources as *const () as usize,
        ]
    })
}
