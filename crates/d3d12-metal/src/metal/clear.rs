//! `ClearUnorderedAccessViewUint/Float` with utility compute kernels (our
//! own MSL; never a substitute for a game shader).
//!
//! * Buffers: the element value is encoded on the CPU (format rules below)
//!   and a fill kernel repeats it over the view's byte range.
//! * Textures: a write kernel per texture type and component class writes
//!   the value into a view of the UAV's mip/slices, per clear rect.
//!
//! Value rules (D3D12): `Uint` clears are bit-precise (the low n bits of
//! each value go to an n-bit channel); `Float` clears convert (float,
//! UNORM/SNORM with saturation; integer formats get the values cast).

use crate::format::{self, Num};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClearValue {
    Uint([u32; 4]),
    Float([f32; 4]),
}

fn f32_to_f16(f: f32) -> u16 {
    let b = f.to_bits();
    let sign = ((b >> 16) & 0x8000) as u16;
    let exp = ((b >> 23) & 0xff) as i32;
    let man = b & 0x7f_ffff;
    if exp == 0xff {
        return sign | 0x7c00 | if man != 0 { 0x200 } else { 0 };
    }
    let e = exp - 127 + 15;
    if e >= 0x1f {
        return sign | 0x7c00;
    }
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let m = (man | 0x80_0000) >> (1 - e);
        return sign | ((m + 0x1000) >> 13) as u16;
    }
    sign | ((e as u32) << 10 | (man + 0x1000) >> 13) as u16
}

fn f16_to_f32(h: u16) -> f32 {
    let s = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let e = ((h >> 10) & 0x1f) as i32;
    let m = (h & 0x3ff) as f32;
    s * match e {
        0 => m * 2f32.powi(-24),
        31 => if m == 0.0 { f32::INFINITY } else { f32::NAN },
        _ => (1.0 + m / 1024.0) * 2f32.powi(e - 15),
    }
}

/// One element of a typed buffer view (`None` = format not handled).
pub fn encode_element(dxgi: u32, v: ClearValue) -> Option<Vec<u8>> {
    let fi = format::info(dxgi);
    let (n, bits) = (fi.channels as usize, fi.chan_bits);
    if n == 0 || n > 4 || !matches!(bits, 8 | 16 | 32) || fi.bytes as usize != n * bits as usize / 8 {
        return None;
    }
    let mut out = Vec::with_capacity(fi.bytes as usize);
    for c in 0..n {
        let x: u32 = match v {
            ClearValue::Uint(u) => u[c],
            ClearValue::Float(f) => {
                let f = f[c];
                let max = ((1u64 << bits) - 1) as f32;
                match fi.num {
                    Num::Float if bits == 32 => f.to_bits(),
                    Num::Float if bits == 16 => f32_to_f16(f) as u32,
                    Num::Unorm | Num::Srgb => (f.clamp(0.0, 1.0) * max).round() as u32,
                    Num::Snorm => {
                        let m = (max / 2.0).floor();
                        ((f.clamp(-1.0, 1.0) * m).round() as i32) as u32
                    }
                    Num::Uint => f.clamp(0.0, max) as u32,
                    Num::Sint => (f.clamp(-(max / 2.0) - 1.0, max / 2.0) as i32) as u32,
                    _ => return None,
                }
            }
        };
        out.extend_from_slice(&x.to_le_bytes()[..bits as usize / 8]);
    }
    Some(out)
}

/// Repeat `elem` to a whole number of dwords: (pattern dwords, period).
pub fn dword_pattern(elem: &[u8]) -> Option<([u32; 4], u32)> {
    let len = match elem.len() {
        1 | 2 | 4 => 4,
        8 => 8,
        12 => 12,
        16 => 16,
        _ => return None,
    };
    let mut b = [0u8; 16];
    for i in 0..len {
        b[i] = elem[i % elem.len()];
    }
    let mut p = [0u32; 4];
    for (i, w) in p.iter_mut().enumerate() {
        *w = u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
    }
    Some((p, len as u32 / 4))
}

/// Component class a texture kernel writes with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Class {
    Float,
    Uint,
    Sint,
}

/// Texture kernel inputs for a view of `dxgi`: (class, uint4, float4).
pub fn texture_value(dxgi: u32, v: ClearValue) -> Result<(Class, [u32; 4], [f32; 4]), String> {
    let fi = format::info(dxgi);
    match (fi.num, v) {
        (Num::Uint, ClearValue::Uint(u)) => Ok((Class::Uint, u, [0.0; 4])),
        (Num::Sint, ClearValue::Uint(u)) => Ok((Class::Sint, u, [0.0; 4])),
        (Num::Uint, ClearValue::Float(f)) => Ok((Class::Uint, f.map(|x| x.max(0.0) as u32), [0.0; 4])),
        (Num::Sint, ClearValue::Float(f)) => Ok((Class::Sint, f.map(|x| (x as i32) as u32), [0.0; 4])),
        (Num::Float | Num::Unorm | Num::Snorm | Num::Srgb, ClearValue::Float(f)) => Ok((Class::Float, [0; 4], f)),
        // Bit-precise uint clear of a float/norm format: decode the bits.
        (Num::Float | Num::Unorm | Num::Snorm | Num::Srgb, ClearValue::Uint(u)) => {
            let bits = fi.chan_bits;
            let mut f = [0f32; 4];
            for c in 0..4 {
                let mask = if bits >= 32 { u32::MAX } else { (1u32 << bits) - 1 };
                let x = u[c] & mask;
                f[c] = match (fi.num, bits) {
                    (Num::Float, 32) => f32::from_bits(x),
                    (Num::Float, 16) => f16_to_f32(x as u16),
                    (Num::Unorm | Num::Srgb, 8 | 16 | 10 | 2) => x as f32 / mask as f32,
                    (Num::Snorm, 8 | 16) => {
                        let sx = ((x << (32 - bits)) as i32) >> (32 - bits);
                        (sx as f32 / ((1u32 << (bits - 1)) - 1) as f32).max(-1.0)
                    }
                    _ => return Err(format!("bit-precise uint clear of {} not implemented", fi.name)),
                };
            }
            Ok((Class::Float, [0; 4], f))
        }
        _ => Err(format!("UAV clear of format {}", fi.name)),
    }
}

/// Fill kernel parameters (matches MSL `FillParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FillParams {
    pub dst: u64,
    pub words: u32,
    pub period: u32,
    pub pattern: [u32; 4],
}

/// Texture kernel parameters (matches MSL `TexParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TexParams {
    pub u: [u32; 4],
    pub f: [f32; 4],
    pub x0: u32,
    pub y0: u32,
    pub z0: u32,
    pub w: u32,
    pub h: u32,
    pub d: u32,
    pub _pad: [u32; 2],
}

/// Parameters for RTV/DSV rect clear draw passes (matches MSL `RectClearParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RectClearParams {
    pub color: [f32; 4],
    pub depth: f32,
    pub _pad: [u32; 3],
}

/// Parameters for FidelityFX FSR3 autogen reactive mask pass (matches MSL `FfxReactiveParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FfxReactiveParams {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub _pad0: u32,
    pub scale: f32,
    pub cutoff_threshold: f32,
    pub binary_value: f32,
    pub _pad1: f32,
}

pub fn bytes_of<T: Copy>(t: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()) }
}

const SRC_HEAD: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct FillParams { device uint* dst; uint words; uint period; uint pattern[4]; };
kernel void port_fill(constant FillParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    if (i < p.words) p.dst[i] = p.pattern[i % p.period];
}
kernel void port_fill_v4(constant FillParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    uint base = i << 2u;
    if (base >= p.words) return;
    if (base + 4u <= p.words) {
        uint per = p.period;
        uint4 v = per == 1u
            ? uint4(p.pattern[0])
            : uint4(
                p.pattern[base % per],
                p.pattern[(base + 1u) % per],
                p.pattern[(base + 2u) % per],
                p.pattern[(base + 3u) % per]
            );
        *(device uint4*)(p.dst + base) = v;
    } else {
        for (uint k = base; k < p.words; ++k) p.dst[k] = p.pattern[k % p.period];
    }
}

struct TexParams { uint4 u; float4 f; uint x0; uint y0; uint z0; uint w; uint h; uint d; uint pad0; uint pad1; };

struct RectClearParams { float4 color; float depth; uint pad0; uint pad1; uint pad2; };
struct RectClearVsOut { float4 pos [[position]]; };
vertex RectClearVsOut port_rect_clear_vs(uint vid [[vertex_id]], constant RectClearParams& p [[buffer(0)]]) {
    float2 uv = float2((vid << 1) & 2, vid & 2);
    RectClearVsOut o;
    o.pos = float4(uv * float2(2.0, -2.0) + float2(-1.0, 1.0), p.depth, 1.0);
    return o;
}
fragment float4 port_rect_clear_fs_f(constant RectClearParams& p [[buffer(0)]]) { return p.color; }
fragment uint4 port_rect_clear_fs_u(constant RectClearParams& p [[buffer(0)]]) { return uint4(max(p.color, 0.0)); }
fragment int4 port_rect_clear_fs_i(constant RectClearParams& p [[buffer(0)]]) { return int4(p.color); }

kernel void port_upscale_2d_f(texture2d<float, access::sample> src [[texture(0)]], texture2d<float, access::write> dst [[texture(1)]], constant TexParams& p [[buffer(0)]], uint3 g [[thread_position_in_grid]]) {
    if (g.x >= p.w || g.y >= p.h) return;
    constexpr sampler s(coord::pixel, address::clamp_to_edge, filter::linear);
    float2 src_sz = float2(p.x0 > 0 ? p.x0 : src.get_width(), p.y0 > 0 ? p.y0 : src.get_height());
    float2 src_off = float2(p.u.x, p.u.y);
    float2 uv = src_off + (float2(g.xy) + 0.5) * (src_sz / float2(p.w, p.h));
    uv = clamp(uv, src_off + 0.5, src_off + max(src_sz - 0.5, 0.5));
    dst.write(src.sample(s, uv), g.xy + uint2(p.u.z, p.u.w));
}

struct FfxReactiveParams { uint width; uint height; uint flags; uint pad0; float scale; float cutoff_threshold; float binary_value; float pad1; };
inline float3 port_ffx_tonemap(float3 rgb) {
    return rgb / (max(max(0.0f, rgb.r), max(rgb.g, rgb.b)) + 1.0f);
}
inline float3 port_ffx_inv_tonemap(float3 rgb) {
    return rgb / max(1e-6f, 1.0f - max(rgb.r, max(rgb.g, rgb.b)));
}
kernel void port_ffx_gen_reactive_2d(
    texture2d<float, access::read> opaque_tex [[texture(0)]],
    texture2d<float, access::read> pre_upscale_tex [[texture(1)]],
    texture2d<float, access::write> out_reactive [[texture(2)]],
    constant FfxReactiveParams& p [[buffer(0)]],
    uint2 g [[thread_position_in_grid]])
{
    if (g.x >= p.width || g.y >= p.height) return;
    float3 c_opaque = opaque_tex.read(g).rgb;
    float3 c_pre = pre_upscale_tex.read(g).rgb;
    if ((p.flags & 1u) != 0u) {
        c_opaque = port_ffx_tonemap(c_opaque);
        c_pre = port_ffx_tonemap(c_pre);
    }
    if ((p.flags & 2u) != 0u) {
        c_opaque = port_ffx_inv_tonemap(c_opaque);
        c_pre = port_ffx_inv_tonemap(c_pre);
    }
    float3 delta = abs(c_pre - c_opaque);
    float val = ((p.flags & 8u) != 0u)
        ? max(delta.x, max(delta.y, delta.z))
        : length(delta);
    val *= p.scale;
    if ((p.flags & 4u) != 0u) {
        val = (val < p.cutoff_threshold) ? 0.0f : p.binary_value;
    }
    out_reactive.write(float4(val, val, val, val), g);
}

kernel void port_present_copy_2d(texture2d<float, access::read> src [[texture(0)]], texture2d<float, access::write> dst [[texture(1)]], uint2 g [[thread_position_in_grid]]) {
    if (g.x < dst.get_width() && g.y < dst.get_height() && g.x < src.get_width() && g.y < src.get_height()) {
        dst.write(src.read(g), g);
    }
}

kernel void port_present_scale_2d(texture2d<float, access::sample> src [[texture(0)]], texture2d<float, access::write> dst [[texture(1)]], uint2 g [[thread_position_in_grid]]) {
    uint dw = dst.get_width(), dh = dst.get_height();
    if (g.x >= dw || g.y >= dh) return;
    float sw = float(max(src.get_width(), 1u));
    float sh = float(max(src.get_height(), 1u));
    float fw = float(dw), fh = float(dh);
    float scale = min(fw / sw, fh / sh);
    float rw = sw * scale;
    float rh = sh * scale;
    float ox = (fw - rw) * 0.5;
    float oy = (fh - rh) * 0.5;
    float px = (float(g.x) + 0.5) - ox;
    float py = (float(g.y) + 0.5) - oy;
    if (px < 0.0 || py < 0.0 || px >= rw || py >= rh) {
        dst.write(float4(0.0, 0.0, 0.0, 1.0), g);
        return;
    }
    constexpr sampler s(coord::normalized, address::clamp_to_edge, filter::linear);
    dst.write(src.sample(s, float2(px / rw, py / rh)), g);
}

kernel void port_diag_tex_scan_2d(
    texture2d<float, access::read> tex [[texture(0)]],
    device atomic_uint* out [[buffer(0)]],
    uint2 g [[thread_position_in_grid]])
{
    uint w = tex.get_width(), h = tex.get_height();
    if (g.x >= w || g.y >= h) return;
    float4 v = tex.read(g);
    if (g.x == (w >> 1) && g.y == (h >> 1)) {
        atomic_store_explicit(&out[16], as_type<uint>(v.x), memory_order_relaxed);
        atomic_store_explicit(&out[17], as_type<uint>(v.y), memory_order_relaxed);
        atomic_store_explicit(&out[18], as_type<uint>(v.z), memory_order_relaxed);
        atomic_store_explicit(&out[19], as_type<uint>(v.w), memory_order_relaxed);
    }
    if (g.x == ((w * 2u) / 3u) && g.y == ((h * 7u) / 8u)) {
        atomic_store_explicit(&out[20], as_type<uint>(v.x), memory_order_relaxed);
        atomic_store_explicit(&out[21], as_type<uint>(v.y), memory_order_relaxed);
        atomic_store_explicit(&out[22], as_type<uint>(v.z), memory_order_relaxed);
        atomic_store_explicit(&out[23], as_type<uint>(v.w), memory_order_relaxed);
    }
    bool nr = isnan(v.x), ng = isnan(v.y), nb = isnan(v.z), na = isnan(v.w);
    bool ir = isinf(v.x), ig = isinf(v.y), ib = isinf(v.z), ia = isinf(v.w);
    bool hr = !nr && !ir && abs(v.x) > 500.0;
    bool hg = !ng && !ig && abs(v.y) > 500.0;
    bool hb = !nb && !ib && abs(v.z) > 500.0;
    if (nr) atomic_fetch_add_explicit(&out[0], 1u, memory_order_relaxed);
    if (ng) atomic_fetch_add_explicit(&out[1], 1u, memory_order_relaxed);
    if (nb) atomic_fetch_add_explicit(&out[2], 1u, memory_order_relaxed);
    if (na) atomic_fetch_add_explicit(&out[3], 1u, memory_order_relaxed);
    if (ir) atomic_fetch_add_explicit(&out[4], 1u, memory_order_relaxed);
    if (ig) atomic_fetch_add_explicit(&out[5], 1u, memory_order_relaxed);
    if (ib) atomic_fetch_add_explicit(&out[6], 1u, memory_order_relaxed);
    if (ia) atomic_fetch_add_explicit(&out[7], 1u, memory_order_relaxed);
    if (hr) atomic_fetch_add_explicit(&out[8], 1u, memory_order_relaxed);
    if (hg) atomic_fetch_add_explicit(&out[9], 1u, memory_order_relaxed);
    if (hb) atomic_fetch_add_explicit(&out[10], 1u, memory_order_relaxed);
    if (nr || ng || nb || ir || ig || ib || hr || hg || hb) {
        if (atomic_load_explicit(&out[12], memory_order_relaxed) == 0xFFFFFFFFu) {
            uint expected = 0xFFFFFFFFu;
            if (atomic_compare_exchange_weak_explicit(&out[12], &expected, (g.y << 16) | g.x, memory_order_relaxed, memory_order_relaxed)) {
                atomic_store_explicit(&out[13], as_type<uint>(v.x), memory_order_relaxed);
                atomic_store_explicit(&out[14], as_type<uint>(v.y), memory_order_relaxed);
                atomic_store_explicit(&out[15], as_type<uint>(v.z), memory_order_relaxed);
            }
        }
    }
}

kernel void port_clamp_f16_inf_2d(
    texture2d<float, access::read_write> tex [[texture(0)]],
    constant float4& maxv [[buffer(0)]],
    uint2 g [[thread_position_in_grid]])
{
    uint bx = g.x << 1u, by = g.y << 1u;
    uint w = tex.get_width(), h = tex.get_height();
    if (bx >= w || by >= h) return;
    for (uint dy = 0u; dy < 2u; ++dy) {
        uint y = by + dy;
        if (y >= h) break;
        for (uint dx = 0u; dx < 2u; ++dx) {
            uint x = bx + dx;
            if (x >= w) break;
            float4 v = tex.read(uint2(x, y));
            if (isinf(v.x) || isinf(v.y) || isinf(v.z) || isinf(v.w)) {
                v.x = isinf(v.x) ? copysign(maxv.x, v.x) : v.x;
                v.y = isinf(v.y) ? copysign(maxv.y, v.y) : v.y;
                v.z = isinf(v.z) ? copysign(maxv.z, v.z) : v.z;
                v.w = isinf(v.w) ? copysign(maxv.w, v.w) : v.w;
                tex.write(v, uint2(x, y));
            }
        }
    }
}

"#;

/// The typed texture clear kernels (1-pixel/thread + 2x2-pixel/thread `*_t4` variants),
/// written out (no MSL macros: template commas break preprocessor argument lists).
fn source() -> String {
    let mut s = SRC_HEAD.to_string();
    for (cls, t, val) in [("f", "float", "p.f"), ("u", "uint", "p.u"), ("i", "int", "as_type<int4>(p.u)")] {
        for (name, tex, coord) in [
            ("1d", "texture1d", "x"),
            ("1da", "texture1d_array", "x, z"),
            ("2d", "texture2d", "uint2(x, y)"),
            ("2da", "texture2d_array", "uint2(x, y), z"),
            ("3d", "texture3d", "uint3(x, y, z)"),
        ] {
            s += &format!(
                "kernel void port_clear_{name}_{cls}({tex}<{t}, access::write> tex [[texture(0)]], constant TexParams& p [[buffer(0)]], uint3 g [[thread_position_in_grid]]) {{\n    if (g.x >= p.w || g.y >= p.h || g.z >= p.d) return;\n    uint x = g.x + p.x0, y = g.y + p.y0, z = g.z + p.z0;\n    tex.write({val}, {coord});\n}}\n"
            );
        }
        s += &format!(
            "kernel void port_clear_2d_{cls}_t4(texture2d<{t}, access::write> tex [[texture(0)]], constant TexParams& p [[buffer(0)]], uint3 g [[thread_position_in_grid]]) {{\n    uint bx = g.x << 1u, by = g.y << 1u;\n    if (bx >= p.w || by >= p.h || g.z >= p.d) return;\n    auto v = {val};\n    uint x0 = bx + p.x0, y0 = by + p.y0;\n    tex.write(v, uint2(x0, y0));\n    bool rx = (bx + 1u) < p.w, ry = (by + 1u) < p.h;\n    if (rx) tex.write(v, uint2(x0 + 1u, y0));\n    if (ry) tex.write(v, uint2(x0, y0 + 1u));\n    if (rx && ry) tex.write(v, uint2(x0 + 1u, y0 + 1u));\n}}\n"
        );
        s += &format!(
            "kernel void port_clear_2da_{cls}_t4(texture2d_array<{t}, access::write> tex [[texture(0)]], constant TexParams& p [[buffer(0)]], uint3 g [[thread_position_in_grid]]) {{\n    uint bx = g.x << 1u, by = g.y << 1u;\n    if (bx >= p.w || by >= p.h || g.z >= p.d) return;\n    auto v = {val};\n    uint x0 = bx + p.x0, y0 = by + p.y0, z = g.z + p.z0;\n    tex.write(v, uint2(x0, y0), z);\n    bool rx = (bx + 1u) < p.w, ry = (by + 1u) < p.h;\n    if (rx) tex.write(v, uint2(x0 + 1u, y0), z);\n    if (ry) tex.write(v, uint2(x0, y0 + 1u), z);\n    if (rx && ry) tex.write(v, uint2(x0 + 1u, y0 + 1u), z);\n}}\n"
        );
    }
    s
}

pub type MtlComputePso = ProtocolObject<dyn MTLComputePipelineState>;
pub type MtlRenderPso = ProtocolObject<dyn MTLRenderPipelineState>;
pub type MtlDepthStencil = ProtocolObject<dyn MTLDepthStencilState>;

struct Kernels {
    lib: Result<Retained<ProtocolObject<dyn MTLLibrary>>, String>,
    compute: Result<HashMap<String, Retained<MtlComputePso>>, String>,
    fast_compute: std::sync::Mutex<HashMap<String, Retained<MtlComputePso>>>,
    rtv_pso: std::sync::Mutex<HashMap<(usize, usize), Retained<MtlRenderPso>>>,
    dsv_pso: std::sync::Mutex<HashMap<(usize, usize), Retained<MtlRenderPso>>>,
    dsv_dss: std::sync::Mutex<HashMap<(bool, bool), Retained<MtlDepthStencil>>>,
}
unsafe impl Send for Kernels {}
unsafe impl Sync for Kernels {}

fn default_tg(name: &str) -> [u32; 3] {
    if name == "port_fill" || name == "port_fill_v4" {
        [64, 1, 1]
    } else if name.ends_with("_t4") || name == "port_clamp_f16_inf_2d" {
        [16, 8, 1]
    } else {
        [8, 8, 1]
    }
}

fn fast_tg(name: &str) -> [u32; 3] {
    if name == "port_fill" || name == "port_fill_v4" {
        [256, 1, 1]
    } else if name.ends_with("_t4") || name == "port_clamp_f16_inf_2d" {
        [16, 8, 1]
    } else if name == "port_upscale_2d_f" || name.starts_with("port_clear_2d_") {
        [16, 16, 1]
    } else {
        [8, 8, 1]
    }
}

fn kernels() -> &'static Kernels {
    static K: OnceLock<Kernels> = OnceLock::new();
    K.get_or_init(|| {
        let init = (|| -> Result<(Retained<ProtocolObject<dyn MTLLibrary>>, HashMap<String, Retained<MtlComputePso>>), String> {
            let lib = super::pipeline::library_from_source("clear kernels", &source())?;
            let mut m = HashMap::new();
            let mut names = vec![
                "port_fill".to_string(),
                "port_fill_v4".to_string(),
                "port_upscale_2d_f".to_string(),
                "port_ffx_gen_reactive_2d".to_string(),
                "port_present_copy_2d".to_string(),
                "port_present_scale_2d".to_string(),
                "port_diag_tex_scan_2d".to_string(),
                "port_clamp_f16_inf_2d".to_string(),
            ];
            for t in ["1d", "1da", "2d", "2da", "3d"] {
                for c in ["f", "u", "i"] {
                    names.push(format!("port_clear_{t}_{c}"));
                }
            }
            for t in ["2d", "2da"] {
                for c in ["f", "u", "i"] {
                    names.push(format!("port_clear_{t}_{c}_t4"));
                }
            }
            for n in names {
                let p = super::pipeline::compute_pipeline_from_function(&lib, &n)?;
                m.insert(n, p);
            }
            Ok((lib, m))
        })();
        let (lib, compute) = match init {
            Ok((l, c)) => (Ok(l), Ok(c)),
            Err(e) => (Err(e.clone()), Err(e)),
        };
        Kernels {
            lib,
            compute,
            fast_compute: std::sync::Mutex::new(HashMap::new()),
            rtv_pso: std::sync::Mutex::new(HashMap::new()),
            dsv_pso: std::sync::Mutex::new(HashMap::new()),
            dsv_dss: std::sync::Mutex::new(HashMap::new()),
        }
    })
}

/// A clear kernel by name (`port_fill`, `port_clear_<2d|2da|1d|1da|3d>_<f|u|i>`).
pub fn kernel(name: &str) -> Result<&'static MtlComputePso, String> {
    let k = kernels();
    match &k.compute {
        Ok(m) => m.get(name).map(|p| &**p).ok_or_else(|| format!("no clear kernel {name}")),
        Err(e) => Err(e.clone()),
    }
}

/// Returns `(pso, threads_per_threadgroup)` for `name`, using threadgroup-pinned
/// SIMD32-multiple PSOs (`[16,16,1]` for 2D clear/upscale, `[256,1,1]` for buffer fill)
/// when `fast` (`AW2_FAST_BIND_PSO=1`) is enabled.
pub fn kernel_for_dispatch(name: &str, fast: bool) -> Result<(Retained<MtlComputePso>, [u32; 3]), String> {
    let k = kernels();
    if !fast {
        return match &k.compute {
            Ok(m) => m
                .get(name)
                .cloned()
                .map(|p| (p, default_tg(name)))
                .ok_or_else(|| format!("no clear kernel {name}")),
            Err(e) => Err(e.clone()),
        };
    }
    let tg = fast_tg(name);
    if let Some(p) = k.fast_compute.lock().unwrap().get(name).cloned() {
        return Ok((p, tg));
    }
    let lib = k.lib.as_ref().map_err(Clone::clone)?;
    let p = super::pipeline::compute_pipeline_from_function_tg(lib, name, Some(tg))?;
    k.fast_compute.lock().unwrap().insert(name.to_string(), p.clone());
    Ok((p, tg))
}

/// Cached render pipeline for `ClearRenderTargetView` with rects.
pub fn rtv_rect_pipeline(fmt: MTLPixelFormat, samples: usize) -> Result<Retained<MtlRenderPso>, String> {
    let k = kernels();
    let lib = k.lib.as_ref().map_err(|e| e.clone())?;
    let key = (fmt.0, samples.max(1));
    let cached = k.rtv_pso.lock().unwrap().get(&key).cloned();
    if let Some(p) = cached {
        return Ok(p);
    }
    let fs_name = match format::TABLE.iter().find(|f| f.mtl == fmt && f.num != Num::Typeless).map(|f| f.num) {
        Some(Num::Uint) => "port_rect_clear_fs_u",
        Some(Num::Sint) => "port_rect_clear_fs_i",
        _ => "port_rect_clear_fs_f",
    };
    let desc = MTL4RenderPipelineDescriptor::new();
    desc.setVertexFunctionDescriptor(Some(&super::pipeline::function_descriptor(lib, "port_rect_clear_vs")));
    desc.setFragmentFunctionDescriptor(Some(&super::pipeline::function_descriptor(lib, fs_name)));
    unsafe { desc.setRasterSampleCount(samples.max(1)) };
    desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Triangle);
    let ca = unsafe { desc.colorAttachments().objectAtIndexedSubscript(0) };
    ca.setPixelFormat(fmt);
    let pso = super::pipeline::pso_cache()?.render_pipeline(None, &desc).map_err(|e| format!("rtv rect clear PSO ({fmt:?}): {e}"))?;
    k.rtv_pso.lock().unwrap().insert(key, pso.clone());
    Ok(pso)
}

/// Cached render pipeline and depth-stencil state for `ClearDepthStencilView` with rects.
pub fn dsv_rect_pipeline(fmt: MTLPixelFormat, samples: usize, clear_depth: bool, clear_stencil: bool) -> Result<(Retained<MtlRenderPso>, Retained<MtlDepthStencil>), String> {
    let k = kernels();
    let lib = k.lib.as_ref().map_err(|e| e.clone())?;
    let d = super::device().ok_or("no device")?;
    let pso_key = (fmt.0, samples.max(1));
    let cached_pso = k.dsv_pso.lock().unwrap().get(&pso_key).cloned();
    let pso = if let Some(p) = cached_pso {
        p
    } else {
        let desc = MTL4RenderPipelineDescriptor::new();
        desc.setVertexFunctionDescriptor(Some(&super::pipeline::function_descriptor(lib, "port_rect_clear_vs")));
        unsafe { desc.setRasterSampleCount(samples.max(1)) };
        desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Triangle);
        let p = super::pipeline::pso_cache()?.render_pipeline(None, &desc).map_err(|e| format!("dsv rect clear PSO ({fmt:?}): {e}"))?;
        k.dsv_pso.lock().unwrap().insert(pso_key, p.clone());
        p
    };
    let dss_key = (clear_depth, clear_stencil);
    let cached_dss = k.dsv_dss.lock().unwrap().get(&dss_key).cloned();
    let dss = if let Some(s) = cached_dss {
        s
    } else {
        let desc = MTLDepthStencilDescriptor::new();
        if clear_depth {
            desc.setDepthCompareFunction(MTLCompareFunction::Always);
            desc.setDepthWriteEnabled(true);
        } else {
            desc.setDepthCompareFunction(MTLCompareFunction::Always);
            desc.setDepthWriteEnabled(false);
        }
        if clear_stencil {
            let st = MTLStencilDescriptor::new();
            st.setStencilCompareFunction(MTLCompareFunction::Always);
            st.setDepthStencilPassOperation(MTLStencilOperation::Replace);
            st.setDepthFailureOperation(MTLStencilOperation::Replace);
            st.setStencilFailureOperation(MTLStencilOperation::Replace);
            st.setReadMask(0xff);
            st.setWriteMask(0xff);
            desc.setFrontFaceStencil(Some(&st));
            desc.setBackFaceStencil(Some(&st));
        }
        let s = d.dev.newDepthStencilStateWithDescriptor(&desc).ok_or("dsv rect clear depth-stencil state")?;
        k.dsv_dss.lock().unwrap().insert(dss_key, s.clone());
        s
    };
    Ok((pso, dss))
}

/// Kernel name suffix for a texture view type.
pub fn texture_kind(ty: MTLTextureType) -> Option<&'static str> {
    Some(match ty {
        MTLTextureType::Type1D => "1d",
        MTLTextureType::Type1DArray => "1da",
        MTLTextureType::Type2D => "2d",
        MTLTextureType::Type2DArray => "2da",
        MTLTextureType::Type3D => "3d",
        _ => return None,
    })
}

pub fn class_suffix(c: Class) -> &'static str {
    match c {
        Class::Float => "f",
        Class::Uint => "u",
        Class::Sint => "i",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_encoding() {
        // R32_UINT (42), R32_FLOAT (41), R16G16_UNORM (35), R8G8B8A8_UNORM (28).
        assert_eq!(encode_element(42, ClearValue::Uint([7, 0, 0, 0])), Some(7u32.to_le_bytes().to_vec()));
        assert_eq!(encode_element(41, ClearValue::Float([1.5, 0.0, 0.0, 0.0])), Some(1.5f32.to_bits().to_le_bytes().to_vec()));
        assert_eq!(encode_element(35, ClearValue::Float([1.0, 0.0, 0.0, 0.0])), Some(vec![0xff, 0xff, 0, 0]));
        assert_eq!(encode_element(28, ClearValue::Uint([0x1ff, 2, 3, 4])), Some(vec![0xff, 2, 3, 4]));
        assert_eq!(dword_pattern(&[1, 2]), Some(([0x0201_0201, 0, 0, 0], 1)));
        assert_eq!(dword_pattern(&[1, 0, 0, 0, 2, 0, 0, 0]), Some(([1, 2, 0, 0], 2)));
        assert_eq!(f32_to_f16(1.0), 0x3c00);
        assert_eq!(f16_to_f32(0x3c00), 1.0);
    }

    #[test]
    fn texture_values() {
        // R8G8B8A8_UNORM uint clear: bit-precise 0xff -> 1.0.
        let (c, _, f) = texture_value(28, ClearValue::Uint([0xff, 0, 0x80, 0])).unwrap();
        assert_eq!(c, Class::Float);
        assert_eq!(f[0], 1.0);
        assert!((f[2] - 128.0 / 255.0).abs() < 1e-6);
        assert_eq!(texture_value(42, ClearValue::Float([3.7, 0.0, 0.0, 0.0])).unwrap().1[0], 3);
    }

    #[test]
    fn params_layout() {
        assert_eq!(std::mem::size_of::<FillParams>(), 32);
        assert_eq!(std::mem::size_of::<TexParams>(), 64);
    }

    #[test]
    fn kernels_compile() {
        if super::super::device().is_none() {
            return;
        }
        kernel("port_fill").unwrap();
        kernel("port_clear_3d_i").unwrap();
    }

    /// Runs the fill and a 2D uint clear kernel on the GPU via Metal 4.
    #[test]
    fn kernels_run() {
        let Some(d) = super::super::device() else { return };
        let opts = MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked;
        // Fill: words 1..7 of a 8-word buffer with a period-2 pattern.
        let buf = d.dev.newBufferWithLength_options(32, opts).unwrap();
        let fp = FillParams { dst: buf.gpuAddress() + 4, words: 6, period: 2, pattern: [0xA, 0xB, 0, 0] };
        let fpb = unsafe {
            d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(&fp as *const _ as *mut _).unwrap(), 32, opts)
        }
        .unwrap();
        // 2D R32_UINT texture 8x8: clear [2,2)-(6,4) to 9.
        let td = unsafe { MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(MTLPixelFormat::R32Uint, 8, 8, false) };
        td.setUsage(MTLTextureUsage::ShaderWrite | MTLTextureUsage::ShaderRead);
        td.setStorageMode(MTLStorageMode::Shared);
        td.setHazardTrackingMode(MTLHazardTrackingMode::Untracked);
        let tex = d.dev.newTextureWithDescriptor(&td).unwrap();
        let tp = TexParams { u: [9, 0, 0, 0], x0: 2, y0: 2, z0: 0, w: 4, h: 2, d: 1, ..Default::default() };
        let tpb = unsafe {
            d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(&tp as *const _ as *mut _).unwrap(), 64, opts)
        }
        .unwrap();

        let rsd = MTLResidencySetDescriptor::new();
        let rs = d.dev.newResidencySetWithDescriptor_error(&rsd).unwrap();
        for a in [
            ProtocolObject::from_ref(&*buf),
            ProtocolObject::from_ref(&*fpb),
            ProtocolObject::from_ref(&*tpb),
            ProtocolObject::from_ref(&*tex),
        ] {
            rs.addAllocation(a);
        }
        rs.commit();

        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxBufferBindCount(1);
        atd.setMaxTextureBindCount(1);
        let table = d.dev.newArgumentTableWithDescriptor_error(&atd).unwrap();

        let q = d.dev.newMTL4CommandQueue().unwrap();
        q.addResidencySet(&rs);
        let alloc = d.dev.newCommandAllocator().unwrap();
        let cmd = d.dev.newCommandBuffer().unwrap();
        cmd.beginCommandBufferWithAllocator(&alloc);
        let e = cmd.computeCommandEncoder().unwrap();
        unsafe {
            e.setComputePipelineState(kernel("port_fill").unwrap());
            table.setAddress_atIndex(fpb.gpuAddress(), 0);
            e.setArgumentTable(Some(&table));
            e.dispatchThreads_threadsPerThreadgroup(MTLSize { width: 6, height: 1, depth: 1 }, MTLSize { width: 64, height: 1, depth: 1 });
            e.setComputePipelineState(kernel("port_clear_2d_u").unwrap());
            table.setAddress_atIndex(tpb.gpuAddress(), 0);
            table.setTexture_atIndex(tex.gpuResourceID(), 0);
            e.setArgumentTable(Some(&table));
            e.dispatchThreads_threadsPerThreadgroup(MTLSize { width: 4, height: 2, depth: 1 }, MTLSize { width: 8, height: 8, depth: 1 });
        }
        e.endEncoding();
        cmd.endCommandBuffer();
        let ev = d.dev.newSharedEvent().unwrap();
        let mut cbs = [std::ptr::NonNull::from(&*cmd)];
        unsafe { q.commit_count(std::ptr::NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
        q.signalEvent_value(ProtocolObject::from_ref(&*ev), 1);
        assert!(ev.waitUntilSignaledValue_timeoutMS(1, 5000));

        let o = unsafe { std::slice::from_raw_parts(buf.contents().as_ptr() as *const u32, 8) };
        assert_eq!(o, &[0, 0xA, 0xB, 0xA, 0xB, 0xA, 0xB, 0]);
        let mut px = [0u32; 64];
        unsafe {
            tex.getBytes_bytesPerRow_fromRegion_mipmapLevel(
                std::ptr::NonNull::new(px.as_mut_ptr() as *mut _).unwrap(),
                32,
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 8, height: 8, depth: 1 } },
                0,
            )
        };
        for y in 0..8 {
            for x in 0..8 {
                let want = if (2..6).contains(&x) && (2..4).contains(&y) { 9 } else { 0 };
                assert_eq!(px[y * 8 + x], want, "({x},{y})");
            }
        }
    }
}
