//! Direct2D geometry representation, curve flattening, stroke expansion,
//! `ID2D1StrokeStyle/1`, `ID2D1GeometrySink`, all 6 `ID2D1Geometry` subclasses,
//! `ID2D1Layer`, `ID2D1DrawingStateBlock`, and `ID2D1Mesh`.

use super::abi::*;
use crate::com::{
    com_add_ref, com_call_add_ref, com_from_iface, com_release, qi_helper_bytes,
    register_vtable_once, write_null_out, ComObj, ComPtr,
};
use crate::types::*;
use std::f32::consts::PI;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

// ============================================================================
// Geometry Representation, Path Flattening, Stroke Expansion & Hit-Testing
// ============================================================================

#[derive(Clone, Debug)]
pub(super) struct SubPath {
    pub(super) points: Vec<(f32, f32)>,
    pub(super) closed: bool,
    pub(super) filled: bool,
}

#[derive(Clone)]
pub(super) struct FlattenedGeometry {
    pub(super) subpaths: Vec<SubPath>,
    pub(super) fill_mode: u32,
}

fn flatten_cubic(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    tol: f32,
    out: &mut Vec<(f32, f32)>,
) {
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

pub(super) fn rect_subpath(r: D2D1_RECT_F) -> SubPath {
    let l = r.left.min(r.right);
    let rr = r.left.max(r.right);
    let t = r.top.min(r.bottom);
    let b = r.top.max(r.bottom);
    SubPath { points: vec![(l, t), (rr, t), (rr, b), (l, b)], closed: true, filled: true }
}

pub(super) fn rounded_rect_subpath(rr: D2D1_ROUNDED_RECT) -> SubPath {
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

pub(super) fn ellipse_subpath(el: D2D1_ELLIPSE) -> SubPath {
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

pub(super) fn transform_subpaths(subpaths: &[SubPath], m: &D2D1_MATRIX_3X2_F) -> Vec<SubPath> {
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

pub(super) fn expand_strokes(
    subpaths: &[SubPath],
    stroke_width: f32,
    style: Option<&StrokeStyleData>,
) -> Vec<SubPath> {
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
                if line_join == D2D1_LINE_JOIN_ROUND
                    || line_join == D2D1_LINE_JOIN_MITER
                    || line_join == D2D1_LINE_JOIN_BEVEL
                {
                    push_round_disk(&mut polys, p1.0, p1.1, half);
                }
            }
        }
    }
    polys
}

pub(super) fn subpaths_bounds(subpaths: &[SubPath]) -> D2D1_RECT_F {
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

pub(super) fn point_in_subpaths(x: f32, y: f32, subpaths: &[SubPath], fill_mode: u32) -> bool {
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

// ============================================================================
// Internal COM Object Payloads & Downcasters
// ============================================================================

#[derive(Clone)]
pub(super) struct StrokeStyleData {
    pub(super) props: D2D1_STROKE_STYLE_PROPERTIES,
    pub(super) transform_type: u32,
    pub(super) dashes: Vec<f32>,
}

pub(super) struct StrokeStyleInner {
    pub(super) factory: ComPtr,
    pub(super) data: StrokeStyleData,
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
pub(super) struct PathData {
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
                        flatten_cubic(
                            cur,
                            (b.point1.x, b.point1.y),
                            (b.point2.x, b.point2.y),
                            (b.point3.x, b.point3.y),
                            tol,
                            &mut pts,
                        );
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

pub(super) enum GeometryKind {
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

pub(super) struct GeometryInner {
    pub(super) factory: ComPtr,
    pub(super) kind: GeometryKind,
}

impl GeometryInner {
    pub(super) unsafe fn flatten(&self, extra_transform: Option<&D2D1_MATRIX_3X2_F>, tol: f32) -> FlattenedGeometry {
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

pub(super) struct GeometrySinkInner {
    state: Mutex<GeometrySinkState>,
}

pub(super) struct LayerInner {
    pub(super) factory: ComPtr,
    pub(super) size: D2D1_SIZE_F,
}

pub(super) struct DrawingStateBlockInner {
    pub(super) factory: ComPtr,
    pub(super) state: Mutex<(D2D1_DRAWING_STATE_DESCRIPTION, Option<ComPtr>)>,
}

pub(super) struct MeshInner {
    pub(super) factory: ComPtr,
}

pub(super) unsafe fn geometry_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a GeometryInner> {
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

pub(super) unsafe fn stroke_style_from_ptr<'a>(ptr: *mut c_void) -> Option<&'a StrokeStyleData> {
    com_from_iface::<StrokeStyleInner>(ptr, stroke_style_vtbl()).map(|obj| &obj.inner.data)
}

pub(super) unsafe fn drawing_state_block_from_ptr<'a>(
    ptr: *mut c_void,
) -> Option<&'a DrawingStateBlockInner> {
    com_from_iface::<DrawingStateBlockInner>(ptr, drawing_state_block_vtbl()).map(|obj| &obj.inner)
}

// ============================================================================
// ID2D1StrokeStyle / ID2D1StrokeStyle1 Vtable (14 slots)
// ============================================================================

type PStrokeStyle = *mut ComObj<StrokeStyleInner>;

unsafe extern "win64" fn ss_qi(this: PStrokeStyle, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_STROKE_STYLE, G_ID2D1_STROKE_STYLE1])
}
unsafe extern "win64" fn ss_get_factory(this: PStrokeStyle, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
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

pub(super) fn stroke_style_vtbl() -> *const usize {
    static V: OnceLock<[usize; 14]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            ss_qi as *const () as usize,
            com_add_ref::<StrokeStyleInner> as *const () as usize,
            com_release::<StrokeStyleInner> as *const () as usize,
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
        ]
    })
}

// ============================================================================
// ID2D1GeometrySink (15 slots) & All 6 ID2D1Geometry Subclasses
// ============================================================================

pub(super) type PGeomSink = *mut ComObj<GeometrySinkInner>;

unsafe extern "win64" fn gsink_qi(this: PGeomSink, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_SIMPLIFIED_GEOMETRY_SINK, G_ID2D1_GEOMETRY_SINK])
}
unsafe extern "win64" fn gsink_set_fill_mode(this: PGeomSink, mode: u32) {
    if this.is_null() { return; }
    ComObj::inner(this).state.lock().unwrap().fill_mode = mode;
}
unsafe extern "win64" fn gsink_set_seg_flags(_this: PGeomSink, _flags: u32) {}
pub(super) unsafe extern "win64" fn gsink_begin_figure(this: PGeomSink, start: u64, fig_begin: u32) {
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
pub(super) unsafe extern "win64" fn gsink_end_figure(this: PGeomSink, fig_end: u32) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(mut fig) = st.current.take() {
        fig.closed = fig_end == D2D1_FIGURE_END_CLOSED;
        st.figures.push(fig);
    }
}
pub(super) unsafe extern "win64" fn gsink_close(this: PGeomSink) -> HRESULT {
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
pub(super) unsafe extern "win64" fn gsink_add_line(this: PGeomSink, pt: u64) {
    if this.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Line(D2D1_POINT_2F::from_u64(pt)));
    }
}
pub(super) unsafe extern "win64" fn gsink_add_bezier(this: PGeomSink, bez: *const D2D1_BEZIER_SEGMENT) {
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
pub(super) unsafe extern "win64" fn gsink_add_arc(this: PGeomSink, arc: *const D2D1_ARC_SEGMENT) {
    if this.is_null() || arc.is_null() { return; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    if let Some(ref mut fig) = st.current {
        fig.segments.push(PathSegment::Arc(arc.read_unaligned()));
    }
}

fn geom_sink_vtbl() -> *const usize {
    static V: OnceLock<[usize; 15]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            gsink_qi as *const () as usize,
            com_add_ref::<GeometrySinkInner> as *const () as usize,
            com_release::<GeometrySinkInner> as *const () as usize,
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
        ]
    })
}

pub(super) type PGeom = *mut ComObj<GeometryInner>;

unsafe extern "win64" fn geom_qi(this: PGeom, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    match ComObj::inner(this).kind {
        GeometryKind::Rect(_) => {
            qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_RECTANGLE_GEOMETRY])
        }
        GeometryKind::RoundedRect(_) => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_ROUNDED_RECTANGLE_GEOMETRY],
        ),
        GeometryKind::Ellipse(_) => {
            qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_ELLIPSE_GEOMETRY])
        }
        GeometryKind::Path(_) => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_PATH_GEOMETRY, G_ID2D1_PATH_GEOMETRY1],
        ),
        GeometryKind::Group { .. } => {
            qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_GEOMETRY_GROUP])
        }
        GeometryKind::Transformed { .. } => qi_helper_bytes(
            this,
            riid,
            ppv,
            &[G_ID2D1_RESOURCE, G_ID2D1_GEOMETRY, G_ID2D1_TRANSFORMED_GEOMETRY],
        ),
    }
}
unsafe extern "win64" fn geom_get_factory(this: PGeom, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
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

pub(super) unsafe extern "win64" fn geom_fill_contains_pt(
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

pub(super) unsafe extern "win64" fn geom_compute_area(
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

pub(super) unsafe extern "win64" fn geom_compute_length(
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
            com_add_ref::<GeometryInner> as *const () as usize,
            com_release::<GeometryInner> as *const () as usize,
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
pub(super) fn rect_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = rect_geom_get_rect as *const () as usize;
        t
    })
}

unsafe extern "win64" fn rrect_geom_get_rrect(this: PGeom, out: *mut D2D1_ROUNDED_RECT) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::RoundedRect(r) = ComObj::inner(this).kind { out.write_unaligned(r); }
}
pub(super) fn rounded_rect_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = rrect_geom_get_rrect as *const () as usize;
        t
    })
}

unsafe extern "win64" fn ellipse_geom_get_ellipse(this: PGeom, out: *mut D2D1_ELLIPSE) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Ellipse(e) = ComObj::inner(this).kind { out.write_unaligned(e); }
}
pub(super) fn ellipse_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 18];
        t[..17].copy_from_slice(&b);
        t[17] = ellipse_geom_get_ellipse as *const () as usize;
        t
    })
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
            com_call_add_ref(children[i].as_ptr());
            out.add(i).write_unaligned(children[i].as_ptr());
        }
    }
}
pub(super) fn geom_group_vtbl() -> *const usize {
    static V: OnceLock<[usize; 20]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 20];
        t[..17].copy_from_slice(&b);
        t[17] = ggroup_get_fill_mode as *const () as usize;
        t[18] = ggroup_get_count as *const () as usize;
        t[19] = ggroup_get_geoms as *const () as usize;
        t
    })
}

unsafe extern "win64" fn tgeom_get_source(this: PGeom, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Transformed { ref source, .. } = ComObj::inner(this).kind {
        com_call_add_ref(source.as_ptr());
        out.write_unaligned(source.as_ptr());
    }
}
unsafe extern "win64" fn tgeom_get_transform(this: PGeom, out: *mut D2D1_MATRIX_3X2_F) {
    if this.is_null() || out.is_null() { return; }
    if let GeometryKind::Transformed { transform, .. } = ComObj::inner(this).kind {
        out.write_unaligned(transform);
    }
}
pub(super) fn transformed_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 19]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 19];
        t[..17].copy_from_slice(&b);
        t[17] = tgeom_get_source as *const () as usize;
        t[18] = tgeom_get_transform as *const () as usize;
        t
    })
}

pub(super) unsafe extern "win64" fn path_geom_open(this: PGeom, out_sink: *mut *mut c_void) -> HRESULT {
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
pub(super) fn path_geom_vtbl() -> *const usize {
    static V: OnceLock<[usize; 21]> = OnceLock::new();
    register_vtable_once(&V, || {
        let b = geom_base_slots!();
        let mut t = [0usize; 21];
        t[..17].copy_from_slice(&b);
        t[17] = path_geom_open as *const () as usize;
        t[18] = path_geom_stream as *const () as usize;
        t[19] = path_geom_seg_count as *const () as usize;
        t[20] = path_geom_fig_count as *const () as usize;
        t
    })
}

// ============================================================================
// ID2D1Layer (5), ID2D1DrawingStateBlock (8), ID2D1Mesh (5)
// ============================================================================

type PLayer = *mut ComObj<LayerInner>;
unsafe extern "win64" fn layer_qi(this: PLayer, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_LAYER])
}
unsafe extern "win64" fn layer_get_factory(this: PLayer, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn layer_get_size(this: PLayer, ret: *mut D2D1_SIZE_F) -> *mut D2D1_SIZE_F {
    let sz = if this.is_null() { D2D1_SIZE_F::default() } else { ComObj::inner(this).size };
    if !ret.is_null() { ret.write_unaligned(sz); }
    ret
}
pub(super) fn layer_vtbl() -> *const usize {
    static V: OnceLock<[usize; 5]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            layer_qi as *const () as usize,
            com_add_ref::<LayerInner> as *const () as usize,
            com_release::<LayerInner> as *const () as usize,
            layer_get_factory as *const () as usize,
            layer_get_size as *const () as usize,
        ]
    })
}

type PStateBlock = *mut ComObj<DrawingStateBlockInner>;
unsafe extern "win64" fn dsb_qi(this: PStateBlock, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_DRAWING_STATE_BLOCK])
}
unsafe extern "win64" fn dsb_get_factory(this: PStateBlock, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
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
        com_call_add_ref(p.as_ptr());
        out.write_unaligned(p.as_ptr());
    } else {
        write_null_out(out);
    }
}
pub(super) fn drawing_state_block_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            dsb_qi as *const () as usize,
            com_add_ref::<DrawingStateBlockInner> as *const () as usize,
            com_release::<DrawingStateBlockInner> as *const () as usize,
            dsb_get_factory as *const () as usize,
            dsb_get_desc as *const () as usize,
            dsb_set_desc as *const () as usize,
            dsb_set_text_params as *const () as usize,
            dsb_get_text_params as *const () as usize,
        ]
    })
}

type PMesh = *mut ComObj<MeshInner>;
unsafe extern "win64" fn mesh_qi(this: PMesh, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_ID2D1_RESOURCE, G_ID2D1_MESH])
}
unsafe extern "win64" fn mesh_get_factory(this: PMesh, out: *mut *mut c_void) {
    if this.is_null() || out.is_null() { return; }
    let f = ComObj::inner(this).factory.as_ptr();
    com_call_add_ref(f);
    out.write_unaligned(f);
}
unsafe extern "win64" fn mesh_open(_this: PMesh, sink: *mut *mut c_void) -> HRESULT {
    write_null_out(sink);
    E_NOTIMPL
}
pub(super) fn mesh_vtbl() -> *const usize {
    static V: OnceLock<[usize; 5]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            mesh_qi as *const () as usize,
            com_add_ref::<MeshInner> as *const () as usize,
            com_release::<MeshInner> as *const () as usize,
            mesh_get_factory as *const () as usize,
            mesh_open as *const () as usize,
        ]
    })
}
