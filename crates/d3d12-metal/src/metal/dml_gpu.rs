//! Optional Metal 4.1 GPU compute backend for DirectML operators using MSL 4.1
//! `<metal_tensor>` (`tensor_inline`) and `<MetalPerformancePrimitives/MetalPerformancePrimitives.h>`
//! (`mpp::tensor_ops::matmul2d`).
//!
//! Opt-in via `PORT_METAL41_DIRECTML_GPU=1` (or master `PORT_METAL41=1`).
//! Keeps all `objc2-metal` calls strictly inside `d3d12-metal` so `win32-rt` never
//! imports Metal directly.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use super::binding::{MtlArgumentTable, UploadRing};
use super::pipeline::MtlComputePipeline;
use super::queue::{ListType, MtlCommandBuffer, Queue};
use super::resource::Resource;

/// Supported DirectML tensor scalar formats for the Metal 4.1 MPP GPU path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmlDataType {
    Float32,
    Float16,
}

/// Fused element-wise activation applied in registers to `matmul2d` cooperative-tensor outputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DmlFusedActivation {
    None,
    Identity,
    Relu,
    Sigmoid,
    Tanh,
    LeakyRelu(f32),
    Elu(f32),
    Linear(f32, f32),
    HardSigmoid(f32, f32),
    ScaledElu(f32, f32),
    ScaledTanh(f32, f32),
    Softplus(f32),
    Softsign,
    ThresholdedRelu(f32),
    Gelu,
    Celu(f32),
    Clip(f32, f32),
}

impl DmlFusedActivation {
    fn encode(self) -> (u32, f32, f32) {
        match self {
            Self::None | Self::Identity => (0, 0.0, 0.0),
            Self::Relu => (1, 0.0, 0.0),
            Self::Sigmoid => (2, 0.0, 0.0),
            Self::Tanh => (3, 0.0, 0.0),
            Self::LeakyRelu(a) => (4, a, 0.0),
            Self::Elu(a) => (5, a, 0.0),
            Self::Linear(a, b) => (6, a, b),
            Self::HardSigmoid(a, b) => (7, a, b),
            Self::ScaledElu(a, g) => (8, a, g),
            Self::ScaledTanh(a, b) => (9, a, b),
            Self::Softplus(s) => (10, s, 0.0),
            Self::Softsign => (11, 0.0, 0.0),
            Self::ThresholdedRelu(a) => (12, a, 0.0),
            Self::Gelu => (13, 0.0, 0.0),
            Self::Celu(a) => (14, a, 0.0),
            Self::Clip(lo, hi) => (15, lo, hi),
        }
    }
}

/// Descriptor for a 2D or batched row-major DirectML GEMM dispatch on Metal 4.1 MPP.
#[derive(Clone, Copy, Debug)]
pub struct DmlGemmDesc {
    pub dtype: DmlDataType,
    pub batches: u32,
    pub m: u32,
    pub n: u32,
    pub k: u32,
    pub trans_a: bool,
    pub trans_b: bool,
    pub alpha: f32,
    pub beta: f32,
    pub a_batch_stride_elems: u64,
    pub b_batch_stride_elems: u64,
    pub out_batch_stride_elems: u64,
    pub c_batch_stride_elems: u64,
    pub c_row_stride_elems: u64,
    pub c_col_stride_elems: u64,
    pub has_c: bool,
    pub activation: DmlFusedActivation,
}

/// Constant-buffer layout matching MSL `GemmParams`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct GemmParams {
    a_va: u64,
    b_va: u64,
    c_va: u64,
    out_va: u64,
    a_batch_stride: u64,
    b_batch_stride: u64,
    c_batch_stride: u64,
    out_batch_stride: u64,
    c_row_stride: u64,
    c_col_stride: u64,
    m: u32,
    n: u32,
    k: u32,
    batches: u32,
    alpha: f32,
    beta: f32,
    has_c: u32,
    act_kind: u32,
    act_p0: f32,
    act_p1: f32,
}

const _: () = assert!(std::mem::size_of::<GemmParams>() == 120);

const SRC: &str = r#"
#include <metal_stdlib>
#include <metal_tensor>
#include <MetalPerformancePrimitives/MetalPerformancePrimitives.h>
using namespace metal;
using namespace mpp;

struct GemmParams {
    ulong a_va;
    ulong b_va;
    ulong c_va;
    ulong out_va;
    ulong a_batch_stride;
    ulong b_batch_stride;
    ulong c_batch_stride;
    ulong out_batch_stride;
    ulong c_row_stride;
    ulong c_col_stride;
    uint m;
    uint n;
    uint k;
    uint batches;
    float alpha;
    float beta;
    uint has_c;
    uint act_kind;
    float act_p0;
    float act_p1;
};

inline float dml_erf_approx(float x) {
    float sign = x < 0.0f ? -1.0f : 1.0f;
    float ax = abs(x);
    float t = 1.0f / (1.0f + 0.3275911f * ax);
    float y = 1.0f - (((((1.061405429f * t - 1.453152027f) * t) + 1.421413741f) * t - 0.284496736f) * t + 0.254829592f) * t * exp(-ax * ax);
    return sign * y;
}

inline float dml_apply_act(float x, uint kind, float p0, float p1) {
    switch (kind) {
        case 1u:  return max(x, 0.0f);
        case 2u:  return 1.0f / (1.0f + exp(-x));
        case 3u:  return tanh(x);
        case 4u:  return x >= 0.0f ? x : p0 * x;
        case 5u:  return x >= 0.0f ? x : p0 * (exp(x) - 1.0f);
        case 6u:  return p0 * x + p1;
        case 7u:  return clamp(p0 * x + p1, 0.0f, 1.0f);
        case 8u:  return p1 * (x > 0.0f ? x : p0 * (exp(x) - 1.0f));
        case 9u:  return p0 * tanh(p1 * x);
        case 10u: return log(exp(p0 * x) + 1.0f) / p0;
        case 11u: return x / (1.0f + abs(x));
        case 12u: return x > p0 ? x : 0.0f;
        case 13u: return 0.5f * x * (1.0f + dml_erf_approx(x * 0.7071067811865475f));
        case 14u: return max(x, 0.0f) + min(p0 * (exp(x / p0) - 1.0f), 0.0f);
        case 15u: return clamp(x, p0, p1);
        default:  return x;
    }
}

template <typename T, bool TRANS_A, bool TRANS_B>
static inline void dml_gemm_impl(constant GemmParams& p, uint3 tgid) {
    uint batch = tgid.z;
    if (batch >= p.batches) return;
    uint tile_m = tgid.y * 32u;
    uint tile_n = tgid.x * 32u;
    if (tile_m >= p.m || tile_n >= p.n) return;

    device T* a_base = reinterpret_cast<device T*>(p.a_va) + batch * p.a_batch_stride;
    device T* b_base = reinterpret_cast<device T*>(p.b_va) + batch * p.b_batch_stride;
    device T* out_base = reinterpret_cast<device T*>(p.out_va) + batch * p.out_batch_stride;

    int a_d0 = TRANS_A ? int(p.m) : int(p.k);
    int a_d1 = TRANS_A ? int(p.k) : int(p.m);
    int b_d0 = TRANS_B ? int(p.k) : int(p.n);
    int b_d1 = TRANS_B ? int(p.n) : int(p.k);

    auto tA = tensor<device T, dextents<int, 2>, tensor_inline>(a_base, dextents<int, 2>(a_d0, a_d1));
    auto tB = tensor<device T, dextents<int, 2>, tensor_inline>(b_base, dextents<int, 2>(b_d0, b_d1));

    auto mA = tA.slice(TRANS_A ? int(tile_m) : 0, TRANS_A ? 0 : int(tile_m));
    auto mB = tB.slice(TRANS_B ? 0 : int(tile_n), TRANS_B ? int(tile_n) : 0);

    constexpr auto desc = tensor_ops::matmul2d_descriptor(
        32, 32, tensor_ops::dynamic_length_v<int>,
        TRANS_A, TRANS_B, false,
        tensor_ops::matmul2d_descriptor::mode::multiply);
    tensor_ops::matmul2d<desc, execution_simdgroups<4>> op;

    auto cTc = op.template get_destination_cooperative_tensor<decltype(mA), decltype(mB), float>();
    op.run(mA, mB, cTc);

    device const T* c_base = p.has_c != 0u
        ? (reinterpret_cast<device const T*>(p.c_va) + batch * p.c_batch_stride)
        : nullptr;

    for (unsigned short i = 0; i < cTc.get_capacity(); ++i) {
        if (!cTc.is_valid_element(i)) continue;
        auto coord = cTc.get_multidimensional_index(i);
        uint col = tile_n + uint(coord[0]);
        uint row = tile_m + uint(coord[1]);
        if (row >= p.m || col >= p.n) continue;
        float y = p.alpha * float(cTc[i]);
        if (c_base != nullptr && p.beta != 0.0f) {
            float cv = float(c_base[ulong(row) * p.c_row_stride + ulong(col) * p.c_col_stride]);
            y += p.beta * cv;
        }
        if (p.act_kind != 0u) {
            y = dml_apply_act(y, p.act_kind, p.act_p0, p.act_p1);
        }
        out_base[ulong(row) * ulong(p.n) + ulong(col)] = T(y);
    }
}

kernel void dml_gemm_f32_nn(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<float, false, false>(p, tgid); }
kernel void dml_gemm_f32_nt(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<float, false, true>(p, tgid); }
kernel void dml_gemm_f32_tn(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<float, true, false>(p, tgid); }
kernel void dml_gemm_f32_tt(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<float, true, true>(p, tgid); }

kernel void dml_gemm_f16_nn(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<half, false, false>(p, tgid); }
kernel void dml_gemm_f16_nt(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<half, false, true>(p, tgid); }
kernel void dml_gemm_f16_tn(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<half, true, false>(p, tgid); }
kernel void dml_gemm_f16_tt(constant GemmParams& p [[buffer(0)]], uint3 tgid [[threadgroup_position_in_grid]]) { dml_gemm_impl<half, true, true>(p, tgid); }
"#;

const KERNEL_NAMES: [&str; 8] = [
    "dml_gemm_f32_nn",
    "dml_gemm_f32_nt",
    "dml_gemm_f32_tn",
    "dml_gemm_f32_tt",
    "dml_gemm_f16_nn",
    "dml_gemm_f16_nt",
    "dml_gemm_f16_tn",
    "dml_gemm_f16_tt",
];

struct Kernels(Result<HashMap<&'static str, Retained<MtlComputePipeline>>, String>);
unsafe impl Send for Kernels {}
unsafe impl Sync for Kernels {}

fn kernel(name: &str) -> Result<&'static MtlComputePipeline, String> {
    static K: OnceLock<Kernels> = OnceLock::new();
    let k = K.get_or_init(|| {
        Kernels((|| {
            let lib = super::pipeline::library_from_source_with_opts("dml mpp gemm", SRC, false)?;
            let mut m = HashMap::new();
            for n in KERNEL_NAMES {
                let p = super::pipeline::compute_pipeline_from_function_tg(&lib, n, Some([128, 1, 1]))?;
                m.insert(n, p);
            }
            Ok(m)
        })())
    });
    match &k.0 {
        Ok(m) => m.get(name).map(|p| &**p).ok_or_else(|| format!("no dml mpp kernel {name}")),
        Err(e) => Err(e.clone()),
    }
}

struct Runner {
    queue: Queue,
    alloc: Retained<ProtocolObject<dyn MTL4CommandAllocator>>,
    cb: Retained<MtlCommandBuffer>,
    table: Retained<MtlArgumentTable>,
    ring: UploadRing,
}
unsafe impl Send for Runner {}

impl Runner {
    fn new() -> Result<Self, String> {
        let dev = super::device().ok_or_else(|| "dml_gpu: no Metal device".to_string())?;
        let queue = Queue::new(ListType::Compute, "DirectML MPP GPU")
            .ok_or_else(|| "dml_gpu: create compute queue failed".to_string())?;
        let alloc = dev
            .dev
            .newCommandAllocator()
            .ok_or_else(|| "dml_gpu: create command allocator failed".to_string())?;
        let cb = dev
            .dev
            .newCommandBuffer()
            .ok_or_else(|| "dml_gpu: create command buffer failed".to_string())?;
        let table = super::binding::new_argument_table("dml mpp")
            .ok_or_else(|| "dml_gpu: create argument table failed".to_string())?;
        let ring = UploadRing::default();
        Ok(Self { queue, alloc, cb, table, ring })
    }
}

fn runner() -> &'static Mutex<Option<Runner>> {
    static R: OnceLock<Mutex<Option<Runner>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(None))
}

fn kernel_name(dtype: DmlDataType, trans_a: bool, trans_b: bool) -> &'static str {
    match (dtype, trans_a, trans_b) {
        (DmlDataType::Float32, false, false) => "dml_gemm_f32_nn",
        (DmlDataType::Float32, false, true) => "dml_gemm_f32_nt",
        (DmlDataType::Float32, true, false) => "dml_gemm_f32_tn",
        (DmlDataType::Float32, true, true) => "dml_gemm_f32_tt",
        (DmlDataType::Float16, false, false) => "dml_gemm_f16_nn",
        (DmlDataType::Float16, false, true) => "dml_gemm_f16_nt",
        (DmlDataType::Float16, true, false) => "dml_gemm_f16_tn",
        (DmlDataType::Float16, true, true) => "dml_gemm_f16_tt",
    }
}

/// Executes a DirectML `GEMM` directly on Apple Silicon GPU matrix units using MSL 4.1
/// `tensor_inline` + `mpp::tensor_ops::matmul2d` when `PORT_METAL41_DIRECTML_GPU=1`
/// (or `PORT_METAL41=1`) is active.
///
/// Returns `Ok(false)` when the GPU option is disabled or the inputs are ineligible,
/// allowing the caller to fall back cleanly to the CPU engine.
pub fn try_execute_gemm(
    a: (&Resource, u64),
    b: (&Resource, u64),
    c: Option<(&Resource, u64)>,
    out: (&Resource, u64),
    desc: &DmlGemmDesc,
) -> Result<bool, String> {
    if !super::m41::directml_gpu_enabled() {
        return Ok(false);
    }
    execute_gemm_force(a, b, c, out, desc)?;
    Ok(true)
}

/// Unconditionally runs the Metal 4.1 MPP `matmul2d` GEMM kernel (used by [`try_execute_gemm`]
/// and GPU unit tests).
pub fn execute_gemm_force(
    a: (&Resource, u64),
    b: (&Resource, u64),
    c: Option<(&Resource, u64)>,
    out: (&Resource, u64),
    desc: &DmlGemmDesc,
) -> Result<(), String> {
    if desc.batches == 0 || desc.m == 0 || desc.n == 0 || desc.k == 0 {
        return Ok(());
    }
    if a.0.gpu_va == 0 || b.0.gpu_va == 0 || out.0.gpu_va == 0 {
        return Err("dml_gpu: GEMM requires GPU-resident buffer resources".into());
    }
    let c_va = match c {
        Some((cr, off)) if desc.has_c => {
            if cr.gpu_va == 0 {
                return Err("dml_gpu: C bias tensor requires GPU-resident buffer".into());
            }
            cr.gpu_va.saturating_add(off)
        }
        _ => 0,
    };

    let pso = kernel(kernel_name(desc.dtype, desc.trans_a, desc.trans_b))?;
    let (act_kind, act_p0, act_p1) = desc.activation.encode();
    let params = GemmParams {
        a_va: a.0.gpu_va.saturating_add(a.1),
        b_va: b.0.gpu_va.saturating_add(b.1),
        c_va,
        out_va: out.0.gpu_va.saturating_add(out.1),
        a_batch_stride: desc.a_batch_stride_elems,
        b_batch_stride: desc.b_batch_stride_elems,
        c_batch_stride: desc.c_batch_stride_elems,
        out_batch_stride: desc.out_batch_stride_elems,
        c_row_stride: desc.c_row_stride_elems,
        c_col_stride: desc.c_col_stride_elems,
        m: desc.m,
        n: desc.n,
        k: desc.k,
        batches: desc.batches,
        alpha: desc.alpha,
        beta: desc.beta,
        has_c: u32::from(desc.has_c && c_va != 0),
        act_kind,
        act_p0,
        act_p1,
    };
    let bytes = unsafe {
        std::slice::from_raw_parts(
            &params as *const GemmParams as *const u8,
            std::mem::size_of::<GemmParams>(),
        )
    };

    let mut guard = runner().lock().unwrap();
    if guard.is_none() {
        *guard = Some(Runner::new()?);
    }
    let r = guard.as_mut().unwrap();
    r.ring.reset();
    let params_va = r.ring.upload(bytes, 256)?;
    r.alloc.reset();
    r.cb.beginCommandBufferWithAllocator(&r.alloc);
    let enc = r
        .cb
        .computeCommandEncoder()
        .ok_or_else(|| "dml_gpu: computeCommandEncoder returned nil".to_string())?;
    enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
        MTLStages::All,
        MTLStages::Dispatch,
        MTL4VisibilityOptions::Device,
    );
    unsafe { r.table.setAddress_atIndex(params_va, 0) };
    enc.setArgumentTable(Some(&r.table));
    enc.setComputePipelineState(pso);
    enc.dispatchThreadgroups_threadsPerThreadgroup(
        MTLSize {
            width: (desc.n as usize).div_ceil(32),
            height: (desc.m as usize).div_ceil(32),
            depth: desc.batches as usize,
        },
        MTLSize { width: 128, height: 1, depth: 1 },
    );
    enc.barrierAfterStages_beforeQueueStages_visibilityOptions(
        MTLStages::Dispatch,
        MTLStages::All,
        MTL4VisibilityOptions::Device,
    );
    enc.endEncoding();
    r.cb.endCommandBuffer();
    let ticket = r.queue.submit(&[&r.cb]);
    r.queue.timeline.wait_cpu(ticket, u64::MAX);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    fn make_buf(bytes: &[u8]) -> Resource {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: (bytes.len() as u64).max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let r = Resource::new_committed_default(&desc, &hp, "dml_gpu test").unwrap();
        let dst = r.shared_slice(0, bytes.len()).unwrap().as_ptr() as *mut u8;
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len()) };
        r
    }

    fn f32s_to_bytes(v: &[f32]) -> Vec<u8> {
        v.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    fn read_f32s(r: &Resource, n: usize) -> Vec<f32> {
        let s = r.shared_slice(0, n * 4).unwrap();
        (0..n)
            .map(|i| f32::from_le_bytes(s[i * 4..i * 4 + 4].try_into().unwrap()))
            .collect()
    }

    #[test]
    fn mpp_matmul2d_gemm_f32_non_multiple_tile_transposes_bias_and_relu() {
        if crate::metal::device().is_none() {
            return;
        }
        // Non-multiple-of-32 dimensions (M=5, K=7, N=6), batched (B=2), trans_b=true,
        // alpha=2.0, beta=0.5, row-broadcast C[1, N], fused ReLU.
        let (batches, m, k, n) = (2usize, 5usize, 7usize, 6usize);
        let mut a_data = vec![0.0f32; batches * m * k];
        let mut b_data = vec![0.0f32; batches * n * k]; // trans_b => stored as [B, N, K]
        let mut c_data = vec![0.0f32; batches * n];     // broadcast across M rows
        for (i, v) in a_data.iter_mut().enumerate() {
            *v = ((i % 11) as f32 - 5.0) * 0.25;
        }
        for (i, v) in b_data.iter_mut().enumerate() {
            *v = ((i % 7) as f32 - 3.0) * 0.5;
        }
        for (i, v) in c_data.iter_mut().enumerate() {
            *v = (i as f32) * 0.3 - 1.0;
        }
        let a_res = make_buf(&f32s_to_bytes(&a_data));
        let b_res = make_buf(&f32s_to_bytes(&b_data));
        let c_res = make_buf(&f32s_to_bytes(&c_data));
        let out_res = make_buf(&vec![0u8; batches * m * n * 4]);

        let desc = DmlGemmDesc {
            dtype: DmlDataType::Float32,
            batches: batches as u32,
            m: m as u32,
            n: n as u32,
            k: k as u32,
            trans_a: false,
            trans_b: true,
            alpha: 2.0,
            beta: 0.5,
            a_batch_stride_elems: (m * k) as u64,
            b_batch_stride_elems: (n * k) as u64,
            out_batch_stride_elems: (m * n) as u64,
            c_batch_stride_elems: n as u64,
            c_row_stride_elems: 0,
            c_col_stride_elems: 1,
            has_c: true,
            activation: DmlFusedActivation::Relu,
        };
        execute_gemm_force((&a_res, 0), (&b_res, 0), Some((&c_res, 0)), (&out_res, 0), &desc)
            .expect("execute_gemm_force f32");
        let got = read_f32s(&out_res, batches * m * n);

        for bi in 0..batches {
            for r in 0..m {
                for col in 0..n {
                    let mut acc = 0.0f32;
                    for kk in 0..k {
                        acc += a_data[bi * m * k + r * k + kk] * b_data[bi * n * k + col * k + kk];
                    }
                    let exp = (2.0 * acc + 0.5 * c_data[bi * n + col]).max(0.0);
                    let g = got[bi * m * n + r * n + col];
                    assert!(
                        (g - exp).abs() < 1e-3,
                        "mismatch at ({bi},{r},{col}): got={g}, exp={exp}"
                    );
                }
            }
        }
    }

    fn f32_to_f16_bits(val: f32) -> u16 {
        let bits = val.to_bits();
        let sign = ((bits >> 16) & 0x8000) as u16;
        let exp = ((bits >> 23) & 0xff) as i32;
        let frac = bits & 0x007f_ffff;
        if exp == 255 {
            return if frac == 0 { sign | 0x7c00 } else { sign | 0x7e00 | ((frac >> 13) as u16) };
        }
        let h_exp = exp - 127 + 15;
        if h_exp >= 31 {
            return sign | 0x7c00;
        }
        if h_exp <= 0 {
            if h_exp < -10 {
                return sign;
            }
            let m = (frac | 0x0080_0000) >> (14 - h_exp);
            return sign | ((m >> 1) as u16).saturating_add((m & 1) as u16);
        }
        let rf = frac + 0x0000_1000;
        if rf >= 0x0080_0000 {
            let ne = h_exp + 1;
            if ne >= 31 { sign | 0x7c00 } else { sign | ((ne as u16) << 10) }
        } else {
            sign | ((h_exp as u16) << 10) | ((rf >> 13) as u16)
        }
    }

    fn f16_bits_to_f32(bits: u16) -> f32 {
        let sign = ((bits & 0x8000) as u32) << 16;
        let exp = ((bits >> 10) & 0x1f) as u32;
        let frac = (bits & 0x03ff) as u32;
        let out = if exp == 0 {
            if frac == 0 {
                sign
            } else {
                let mut m = frac;
                let mut e = 127i32 - 14;
                while (m & 0x0400) == 0 {
                    m <<= 1;
                    e -= 1;
                }
                m &= 0x03ff;
                sign | ((e as u32) << 23) | (m << 13)
            }
        } else if exp == 31 {
            sign | 0x7f80_0000 | (frac << 13)
        } else {
            sign | ((exp + 127 - 15) << 23) | (frac << 13)
        };
        f32::from_bits(out)
    }

    #[test]
    fn mpp_matmul2d_gemm_f16_trans_a_and_gelu() {
        if crate::metal::device().is_none() {
            return;
        }
        // FP16 GEMM spanning >1 tile (M=35, K=9, N=33) with trans_a=true and fused GELU.
        let (m, k, n) = (35usize, 9usize, 33usize);
        let mut a_f32 = vec![0.0f32; k * m]; // trans_a => stored as [K, M]
        let mut b_f32 = vec![0.0f32; k * n];
        for (i, v) in a_f32.iter_mut().enumerate() {
            *v = ((i % 9) as f32 - 4.0) * 0.125;
        }
        for (i, v) in b_f32.iter_mut().enumerate() {
            *v = ((i % 5) as f32 - 2.0) * 0.25;
        }
        let a_bytes: Vec<u8> = a_f32.iter().flat_map(|&v| f32_to_f16_bits(v).to_le_bytes()).collect();
        let b_bytes: Vec<u8> = b_f32.iter().flat_map(|&v| f32_to_f16_bits(v).to_le_bytes()).collect();
        let a_res = make_buf(&a_bytes);
        let b_res = make_buf(&b_bytes);
        let out_res = make_buf(&vec![0u8; m * n * 2]);

        let desc = DmlGemmDesc {
            dtype: DmlDataType::Float16,
            batches: 1,
            m: m as u32,
            n: n as u32,
            k: k as u32,
            trans_a: true,
            trans_b: false,
            alpha: 1.0,
            beta: 0.0,
            a_batch_stride_elems: 0,
            b_batch_stride_elems: 0,
            out_batch_stride_elems: 0,
            c_batch_stride_elems: 0,
            c_row_stride_elems: 0,
            c_col_stride_elems: 0,
            has_c: false,
            activation: DmlFusedActivation::Gelu,
        };
        execute_gemm_force((&a_res, 0), (&b_res, 0), None, (&out_res, 0), &desc)
            .expect("execute_gemm_force f16");

        let raw = out_res.shared_slice(0, m * n * 2).unwrap();
        let erf_approx = |x: f32| -> f32 {
            let sign = if x < 0.0 { -1.0 } else { 1.0 };
            let ax = x.abs();
            let t = 1.0 / (1.0 + 0.3275911 * ax);
            let y = 1.0
                - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
                    + 0.254829592)
                    * t
                    * (-ax * ax).exp();
            sign * y
        };
        for r in 0..m {
            for col in 0..n {
                let mut acc = 0.0f32;
                for kk in 0..k {
                    acc += a_f32[kk * m + r] * b_f32[kk * n + col];
                }
                let exp = 0.5 * acc * (1.0 + erf_approx(acc * std::f32::consts::FRAC_1_SQRT_2));
                let idx = (r * n + col) * 2;
                let got = f16_bits_to_f32(u16::from_le_bytes([raw[idx], raw[idx + 1]]));
                assert!(
                    (got - exp).abs() < 2e-2,
                    "f16 mismatch at ({r},{col}): got={got}, exp={exp}"
                );
            }
        }
    }
}
