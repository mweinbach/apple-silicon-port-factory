//! RGBA32 pixel surface, scanline rasterizer, `ID2D1Bitmap` / `ID2D1Bitmap1`,
//! `ID2D1GradientStopCollection`, and Direct2D brushes (`Solid`, `LinearGradient`,
//! `RadialGradient`, `Bitmap`).

use super::abi::*;
use super::geometry::{subpaths_bounds, SubPath};
use super::render_target::render_target_from_ptr;
use crate::com::{
    com_add_ref, com_call_add_ref, com_from_iface, com_release, qi_helper_bytes,
    register_vtable_once, write_null_out, ComObj, ComPtr,
};
use crate::types::*;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

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
pub(super) struct Surface {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) pixel_format: D2D1_PIXEL_FORMAT,
    pub(super) dpi_x: f32,
    pub(super) dpi_y: f32,
    /// Tightly packed pixel buffer (`width * height * bpp`).
    /// For RGBA8/BGRA8: 4 bytes/pixel (`pitch = width * 4`).
    /// For A8: 1 byte/pixel (`pitch = width`).
    pub(super) data: Vec<u8>,
}

impl Surface {
    pub(super) fn new(width: u32, height: u32, pf: D2D1_PIXEL_FORMAT, dpi_x: f32, dpi_y: f32) -> Self {
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
    pub(super) fn bpp(&self) -> usize {
        if self.pixel_format.format == DXGI_FORMAT_A8_UNORM { 1 } else { 4 }
    }

    #[inline]
    pub(super) fn pitch(&self) -> usize {
        (self.width as usize) * self.bpp()
    }

    #[inline]
    fn is_bgra(&self) -> bool {
        self.pixel_format.format == DXGI_FORMAT_B8G8R8A8_UNORM
    }

    pub(super) fn resize(&mut self, width: u32, height: u32) {
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
    pub(super) fn get_pixel_straight(&self, x: u32, y: u32) -> [f32; 4] {
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
    pub(super) fn clear_rect(&mut self, color: D2D1_COLOR_F, clip: (i32, i32, i32, i32)) {
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
    pub(super) fn blend_pixel(&mut self, x: u32, y: u32, src: [f32; 4], cov: f32, blend_mode: u32) {
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

    pub(super) fn sample(&self, x: f32, y: f32, ex_x: u32, ex_y: u32, interp: u32) -> [f32; 4] {
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

pub(super) fn rasterize_polygons<F>(
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
pub(super) struct GradientStopCollectionData {
    pub(super) stops: Vec<D2D1_GRADIENT_STOP>,
    pub(super) gamma: u32,
    pub(super) extend_mode: u32,
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
pub(super) enum BrushEval {
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
    pub(super) fn sample_world(&self, wx: f32, wy: f32) -> [f32; 4] {
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
// Internal COM Object Payloads & Downcasters
// ============================================================================

pub(super) struct BitmapInner {
    factory: ComPtr,
    pub(super) surface: Arc<Mutex<Surface>>,
    pub(super) options: u32,
}

pub(super) struct GradientStopCollectionInner {
    pub(super) factory: ComPtr,
    pub(super) data: Arc<GradientStopCollectionData>,
}

#[derive(Clone)]
pub(super) enum BrushKind {
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

pub(super) struct BrushState {
    pub(super) opacity: f32,
    pub(super) transform: D2D1_MATRIX_3X2_F,
    pub(super) kind: BrushKind,
}

pub(super) struct BrushInner {
    pub(super) factory: ComPtr,
    pub(super) state: Mutex<BrushState>,
}

impl BrushInner {
    pub(super) fn snapshot(&self) -> BrushEval {
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

pub(super) unsafe fn brush_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a BrushInner> {
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

pub(super) unsafe fn bitmap_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a BitmapInner> {
    com_from_iface::<BitmapInner>(ptr, bitmap_vtbl()).map(|obj| &obj.inner)
}

pub(super) unsafe fn grad_stops_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a GradientStopCollectionInner> {
    com_from_iface::<GradientStopCollectionInner>(ptr, grad_stops_vtbl()).map(|obj| &obj.inner)
}

// ============================================================================
// ID2D1Bitmap / ID2D1Bitmap1 Vtable (16 slots)
// ============================================================================

pub(super) type PBitmap = *mut ComObj<BitmapInner>;

unsafe extern "win64" fn bmp_qi(this: PBitmap, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(
        this,
        riid,
        ppv,
        &[G_ID2D1_RESOURCE, G_ID2D1_IMAGE, G_ID2D1_BITMAP, G_ID2D1_BITMAP1],
    )
}
unsafe extern "win64" fn bmp_get_factory(this: PBitmap, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
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
pub(super) unsafe extern "win64" fn bmp_get_pixel_size(this: PBitmap, ret: *mut D2D1_SIZE_U) -> *mut D2D1_SIZE_U {
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
pub(super) unsafe extern "win64" fn bmp_copy_from_rt(
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
    write_null_out(out);
}
unsafe extern "win64" fn bmp_get_options(this: PBitmap) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).options }
}
unsafe extern "win64" fn bmp_get_surface(_this: PBitmap, out: *mut *mut c_void) -> HRESULT {
    write_null_out(out);
    E_NOINTERFACE
}
pub(super) unsafe extern "win64" fn bmp_map(this: PBitmap, _options: u32, mapped: *mut D2D1_MAPPED_RECT) -> HRESULT {
    if this.is_null() || mapped.is_null() { return E_POINTER; }
    let mut s = ComObj::inner(this).surface.lock().unwrap();
    let pitch = s.pitch() as u32;
    let bits = s.data.as_mut_ptr();
    mapped.write_unaligned(D2D1_MAPPED_RECT { pitch, _pad: 0, bits });
    S_OK
}
pub(super) unsafe extern "win64" fn bmp_unmap(this: PBitmap) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    S_OK
}

fn bitmap_vtbl() -> *const usize {
    static V: OnceLock<[usize; 16]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            bmp_qi as *const () as usize,
            com_add_ref::<BitmapInner> as *const () as usize,
            com_release::<BitmapInner> as *const () as usize,
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
        ]
    })
}

pub(super) fn new_bitmap_com(factory: ComPtr, surface: Arc<Mutex<Surface>>, options: u32) -> *mut c_void {
    ComObj::new(bitmap_vtbl(), BitmapInner { factory, surface, options }) as *mut c_void
}

// ============================================================================
// ID2D1GradientStopCollection Vtable (8 slots)
// ============================================================================

type PGradStops = *mut ComObj<GradientStopCollectionInner>;

unsafe extern "win64" fn gsc_qi(this: PGradStops, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_GRADIENT_STOP_COLLECTION])
}
unsafe extern "win64" fn gsc_get_factory(this: PGradStops, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
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

pub(super) fn grad_stops_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            gsc_qi as *const () as usize,
            com_add_ref::<GradientStopCollectionInner> as *const () as usize,
            com_release::<GradientStopCollectionInner> as *const () as usize,
            gsc_get_factory as *const () as usize,
            gsc_get_count as *const () as usize,
            gsc_get_stops as *const () as usize,
            gsc_get_gamma as *const () as usize,
            gsc_get_extend as *const () as usize,
        ]
    })
}

// ============================================================================
// Brushes: Solid (10), LinearGradient (13), RadialGradient (17), Bitmap (16)
// ============================================================================

pub(super) type PBrush = *mut ComObj<BrushInner>;

unsafe extern "win64" fn brush_qi(this: PBrush, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let specific = match ComObj::inner(this).state.lock().unwrap().kind {
        BrushKind::Solid { .. } => G_ID2D1_SOLID_COLOR_BRUSH,
        BrushKind::LinearGradient { .. } => G_ID2D1_LINEAR_GRADIENT_BRUSH,
        BrushKind::RadialGradient { .. } => G_ID2D1_RADIAL_GRADIENT_BRUSH,
        BrushKind::Bitmap { .. } => G_ID2D1_BITMAP_BRUSH,
    };
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_BRUSH, specific])
}
unsafe extern "win64" fn brush_get_factory(this: PBrush, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
    out.write_unaligned(f);
}
pub(super) unsafe extern "win64" fn brush_set_opacity(this: PBrush, opacity: f32) {
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

pub(super) fn solid_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 10]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            brush_qi as *const () as usize,
            com_add_ref::<BrushInner> as *const () as usize,
            com_release::<BrushInner> as *const () as usize,
            brush_get_factory as *const () as usize,
            crate::bridge_entry!(brush_set_opacity) as usize,
            brush_set_transform as *const () as usize,
            brush_get_opacity as *const () as usize,
            brush_get_transform as *const () as usize,
            sbrush_set_color as *const () as usize,
            sbrush_get_color as *const () as usize,
        ]
    })
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
            com_call_add_ref(stops_com.as_ptr());
            stops_com.as_ptr()
        }
        _ => std::ptr::null_mut(),
    };
    out.write_unaligned(p);
}

pub(super) fn linear_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 13]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            brush_qi as *const () as usize,
            com_add_ref::<BrushInner> as *const () as usize,
            com_release::<BrushInner> as *const () as usize,
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
        ]
    })
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

pub(super) fn radial_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 17]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            brush_qi as *const () as usize,
            com_add_ref::<BrushInner> as *const () as usize,
            com_release::<BrushInner> as *const () as usize,
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
        ]
    })
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
        com_call_add_ref(b.as_ptr());
        out.write_unaligned(b.as_ptr());
    } else {
        write_null_out(out);
    }
}

pub(super) fn bitmap_brush_vtbl() -> *const usize {
    static V: OnceLock<[usize; 16]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            brush_qi as *const () as usize,
            com_add_ref::<BrushInner> as *const () as usize,
            com_release::<BrushInner> as *const () as usize,
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
        ]
    })
}
