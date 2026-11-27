//! On-demand single-frame GPU `NaN` / `Inf` / HDR-spike pass scanner (`PORT_DIAG_GPU_NAN`).
//!
//! Zero overhead when inactive (one relaxed atomic load per hooked site).
//! Activate either by setting `PORT_DIAG_GPU_NAN=<present_frame>` in the environment
//! or dynamically via `live_ctl` (`gpunan` command -> [`arm_next_frame`]).
//!
//! During the single captured frame, every float 2D render-target / UAV barrier
//! transition and every upscale input/output is scanned in-order on the GPU via
//! `port_diag_tex_scan_2d`. At `Present`, the queue drains once and logs a compact
//! pass-by-pass report identifying the first shader/pass where `NaN`, `Inf`, or
//! extreme HDR values (`|x| > 500`) appear, along with center/sample pixel values.

use super::queue::{MtlCommandBuffer, Queue};
use super::resource::{MtlTexture, Resource};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::Message;
use objc2_metal::*;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

const IDLE: u8 = 0;
const ARMED: u8 = 1;
const CAPTURING: u8 = 2;

static STATE: AtomicU8 = AtomicU8::new(IDLE);
/// Bitmask of narrow-float formats whose hardware write/blend overflows finite values
/// above max-representable to `+Inf` (non-D3D12 behavior, e.g. Apple A19 Pro / iOS 27.3 `24C5049f`):
/// - `BIT_F16` (`1 << 0`): `RGBA16Float`, `RG16Float`, `R16Float`
/// - `BIT_RG11B10` (`1 << 1`): `RG11B10Float`
/// - `BIT_PROBED` (`1 << 7`): probe has completed
const BIT_F16: u8 = 1 << 0;
const BIT_RG11B10: u8 = 1 << 1;
const BIT_PROBED: u8 = 1 << 7;
static F16_OVERFLOW_MASK: AtomicU8 = AtomicU8::new(0);

fn f16_saturate_env_override() -> Option<u8> {
    static ENV: OnceLock<Option<u8>> = OnceLock::new();
    *ENV.get_or_init(|| {
        std::env::var("PORT_F16_SATURATE").ok().and_then(|v| match v.trim() {
            "1" | "true" | "yes" | "on" => Some(BIT_F16 | BIT_RG11B10),
            "f16" => Some(BIT_F16),
            "0" | "false" | "no" | "off" => Some(0),
            _ => None,
        })
    })
}

/// Active narrow-float overflow bitmask (`BIT_F16 | BIT_RG11B10`). Zero on GPUs that
/// saturate finite overflow in hardware (Apple M1-M3, matching D3D12 spec).
#[inline(always)]
pub fn sanitize_f16_mask() -> u8 {
    if let Some(v) = f16_saturate_env_override() {
        return v;
    }
    F16_OVERFLOW_MASK.load(Ordering::Relaxed) & (BIT_F16 | BIT_RG11B10)
}

/// Fast-path check: true iff at least one narrow-float format needs `+Inf` saturation.
#[inline(always)]
pub fn sanitize_f16_inf_active() -> bool {
    sanitize_f16_mask() != 0
}

/// True when a 2D single-sample render target of `fmt` needs `MTLTextureUsage::ShaderWrite`
/// so in-place finite-overflow saturation can run after render-target writes/blends.
///
/// Kept to multi-channel HDR color formats (`RGBA16Float`, plus `RG11B10Float` if probed)
/// so pure-RT `R16Float` / `RG16Float` depth/velocity targets keep lossless framebuffer compression.
#[inline(always)]
pub fn needs_rt_shader_write(fmt: MTLPixelFormat) -> bool {
    let mask = sanitize_f16_mask();
    if mask == 0 {
        return false;
    }
    match fmt {
        MTLPixelFormat::RGBA16Float => mask & BIT_F16 != 0,
        MTLPixelFormat::RG11B10Float => mask & BIT_RG11B10 != 0,
        _ => false,
    }
}

/// Device-resident 32-byte constant buffer holding `[65504.0; 4]` at offset 0 (f16) and
/// `[65024.0, 65024.0, 64512.0, 1.0]` at offset 16 (RG11B10Float).
fn sanitize_maxv_base_va() -> Option<u64> {
    static BUF: OnceLock<Option<SendBuf>> = OnceLock::new();
    let b = BUF
        .get_or_init(|| {
            let d = super::device()?;
            let b = d.dev.newBufferWithLength_options(32, MTLResourceOptions::StorageModeShared)?;
            let vals: [f32; 8] = [65504.0, 65504.0, 65504.0, 65504.0, 65024.0, 65024.0, 64512.0, 1.0];
            unsafe {
                std::ptr::copy_nonoverlapping(vals.as_ptr(), b.contents().as_ptr() as *mut f32, 8);
            }
            d.add_resident(ProtocolObject::from_ref(&*b));
            d.commit_residency();
            Some(SendBuf(b))
        })
        .as_ref()?;
    Some(b.0.gpuAddress())
}

/// Returns `(tex_view_or_tex, width, height, maxv_gpu_va)` if `res` is a 2D single-sample
/// texture whose format failed the hardware finite-saturation probe and supports `read_write`.
pub fn sanitize_f16_target(res: &Resource) -> Option<(Retained<MtlTexture>, u32, u32, u64)> {
    let mask = sanitize_f16_mask();
    if mask == 0 || res.desc.is_buffer() {
        return None;
    }
    let t = res.texture()?;
    if t.textureType() != MTLTextureType::Type2D || t.sampleCount() != 1 {
        return None;
    }
    let offset = match t.pixelFormat() {
        MTLPixelFormat::RGBA16Float | MTLPixelFormat::RG16Float | MTLPixelFormat::R16Float
            if mask & BIT_F16 != 0 =>
        {
            0u64
        }
        MTLPixelFormat::RG11B10Float if mask & BIT_RG11B10 != 0 => 16u64,
        _ => return None,
    };
    let usage = t.usage();
    if !usage.contains(MTLTextureUsage::ShaderRead) || !usage.contains(MTLTextureUsage::ShaderWrite) {
        return None;
    }
    let (w, h) = (t.width() as u32, t.height() as u32);
    if w == 0 || h == 0 {
        return None;
    }
    let base_va = sanitize_maxv_base_va()?;
    let tex = if t.mipmapLevelCount() == 1 && t.arrayLength() == 1 {
        t.retain()
    } else {
        res.texture_view(t.pixelFormat(), MTLTextureType::Type2D, (0, 1), (0, 1))?
    };
    Some((tex, w, h, base_va + offset))
}

/// One-time (~1-2 ms) GPU hardware probe called during `Device` initialization.
///
/// Tests whether writing/blending finite `f32` values above `f16::MAX` (`65504.0`) to
/// `RGBA16Float` and `RG11B10Float` saturates to max finite (D3D12 spec + Apple M1-M3) or
/// overflows to `+Inf` (e.g. A19 Pro / iOS 27.3 `24C5049f`).
pub fn probe_f16_overflow_for_device(dev: &super::device::MtlDevice, dev_name: &str) {
    static PROBED: OnceLock<()> = OnceLock::new();
    PROBED.get_or_init(|| {
        objc2::rc::autoreleasepool(|_| {
            let Some(cq) = dev.newCommandQueue() else {
                return;
            };
            let src = r#"
#include <metal_stdlib>
using namespace metal;
kernel void probe_uav_w(texture2d<float, access::write> t [[texture(0)]], uint2 g [[thread_position_in_grid]]) {
    t.write(float4(70000.0f, 100000.0f, 65504.0f, 1.0f / 0.0f), g);
}
struct VOut { float4 pos [[position]]; };
vertex VOut probe_vs(uint vid [[vertex_id]]) {
    VOut o;
    o.pos = float4((vid == 1) ? 3.0f : -1.0f, (vid == 2) ? 3.0f : -1.0f, 0.0f, 1.0f);
    return o;
}
fragment float4 probe_fs_hi() { return float4(70000.0f, 100000.0f, 65504.0f, 1.0f / 0.0f); }
fragment float4 probe_fs_base() { return float4(64320.0f, 59550.0f, 46620.0f, 64320.0f); }
fragment float4 probe_fs_add() { return float4(2500.0f, 2660.0f, 2820.0f, 2500.0f); }
kernel void probe_rd(texture2d<float, access::read> t [[texture(0)]], device float4* o [[buffer(0)]], uint2 g [[thread_position_in_grid]]) {
    o[0] = t.read(uint2(0, 0));
}
"#;
            let ns_src = objc2_foundation::NSString::from_str(src);
            let compile_probe = |ver: Option<MTLLanguageVersion>| {
                let opts = MTLCompileOptions::new();
                if let Some(v) = ver {
                    opts.setLanguageVersion(v);
                }
                dev.newLibraryWithSource_options_error(&ns_src, Some(&opts))
            };
            let Ok(lib) = compile_probe(Some(dxil_metal::pso_cache::LANGUAGE_VERSION_4_1))
                .or_else(|_| compile_probe(Some(MTLLanguageVersion(4 << 16))))
                .or_else(|_| dev.newLibraryWithSource_options_error(&ns_src, None))
            else {
                log::warn!(target: "d3d", "f16 overflow probe: failed to compile probe library");
                return;
            };
            let Some(f_uav) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_uav_w")) else { return };
            let Some(f_vs) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_vs")) else { return };
            let Some(f_hi) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_fs_hi")) else { return };
            let Some(f_base) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_fs_base")) else { return };
            let Some(f_add) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_fs_add")) else { return };
            let Some(f_rd) = lib.newFunctionWithName(&objc2_foundation::NSString::from_str("probe_rd")) else { return };
            let Ok(p_uav) = dev.newComputePipelineStateWithFunction_error(&f_uav) else { return };
            let Ok(p_rd) = dev.newComputePipelineStateWithFunction_error(&f_rd) else { return };

            let make_rt_pso = |fmt: MTLPixelFormat, fs: &ProtocolObject<dyn MTLFunction>, blend_add: bool| {
                let rpd = MTLRenderPipelineDescriptor::new();
                rpd.setVertexFunction(Some(&f_vs));
                rpd.setFragmentFunction(Some(fs));
                let ca = unsafe { rpd.colorAttachments().objectAtIndexedSubscript(0) };
                ca.setPixelFormat(fmt);
                if blend_add {
                    ca.setBlendingEnabled(true);
                    ca.setRgbBlendOperation(MTLBlendOperation::Add);
                    ca.setSourceRGBBlendFactor(MTLBlendFactor::One);
                    ca.setDestinationRGBBlendFactor(MTLBlendFactor::One);
                    ca.setAlphaBlendOperation(MTLBlendOperation::Add);
                    ca.setSourceAlphaBlendFactor(MTLBlendFactor::One);
                    ca.setDestinationAlphaBlendFactor(MTLBlendFactor::One);
                }
                dev.newRenderPipelineStateWithDescriptor_error(&rpd).ok()
            };

            let run_readback = |fmt: MTLPixelFormat, mode: u8| -> [f32; 4] {
                let td = unsafe {
                    MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(fmt, 1, 1, false)
                };
                td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite | MTLTextureUsage::RenderTarget);
                td.setStorageMode(MTLStorageMode::Private);
                let Some(tex) = dev.newTextureWithDescriptor(&td) else { return [0.0; 4] };
                let Some(buf) = dev.newBufferWithLength_options(16, MTLResourceOptions::StorageModeShared) else { return [0.0; 4] };
                let Some(cb) = cq.commandBuffer() else { return [0.0; 4] };
                let one = MTLSize { width: 1, height: 1, depth: 1 };
                if mode == 0 {
                    if let Some(ce) = cb.computeCommandEncoder() {
                        ce.setComputePipelineState(&p_uav);
                        unsafe { ce.setTexture_atIndex(Some(&tex), 0) };
                        ce.dispatchThreadgroups_threadsPerThreadgroup(one, one);
                        ce.endEncoding();
                    }
                } else {
                    let rpass = MTLRenderPassDescriptor::new();
                    let ca = unsafe { rpass.colorAttachments().objectAtIndexedSubscript(0) };
                    ca.setTexture(Some(&tex));
                    ca.setLoadAction(MTLLoadAction::Clear);
                    ca.setStoreAction(MTLStoreAction::Store);
                    if let Some(re) = cb.renderCommandEncoderWithDescriptor(&rpass) {
                        if mode == 1 {
                            if let Some(pso) = make_rt_pso(fmt, &f_hi, false) {
                                re.setRenderPipelineState(&pso);
                                unsafe { re.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3) };
                            }
                        } else if let (Some(p1), Some(p2)) = (make_rt_pso(fmt, &f_base, false), make_rt_pso(fmt, &f_add, true)) {
                            re.setRenderPipelineState(&p1);
                            unsafe { re.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3) };
                            re.setRenderPipelineState(&p2);
                            unsafe { re.drawPrimitives_vertexStart_vertexCount(MTLPrimitiveType::Triangle, 0, 3) };
                        }
                        re.endEncoding();
                    }
                }
                if let Some(ce) = cb.computeCommandEncoder() {
                    ce.setComputePipelineState(&p_rd);
                    unsafe {
                        ce.setTexture_atIndex(Some(&tex), 0);
                        ce.setBuffer_offset_atIndex(Some(&buf), 0, 0);
                    }
                    ce.dispatchThreadgroups_threadsPerThreadgroup(one, one);
                    ce.endEncoding();
                }
                cb.commit();
                cb.waitUntilCompleted();
                unsafe { *(buf.contents().as_ptr() as *const [f32; 4]) }
            };

            let uav16 = run_readback(MTLPixelFormat::RGBA16Float, 0);
            let uav11 = run_readback(MTLPixelFormat::RG11B10Float, 0);
            let rt16 = run_readback(MTLPixelFormat::RGBA16Float, 1);
            let bld16 = run_readback(MTLPixelFormat::RGBA16Float, 2);
            let bld11 = run_readback(MTLPixelFormat::RG11B10Float, 2);
            let ov_f16 = uav16[0].is_infinite()
                || uav16[1].is_infinite()
                || rt16[0].is_infinite()
                || rt16[1].is_infinite()
                || bld16[0].is_infinite();
            let ov_11 = uav11[0].is_infinite()
                || uav11[1].is_infinite()
                || bld11[0].is_infinite();
            let mut mask = BIT_PROBED;
            if ov_f16 {
                mask |= BIT_F16;
            }
            if ov_11 {
                mask |= BIT_RG11B10;
            }
            F16_OVERFLOW_MASK.store(mask, Ordering::Release);
            log::info!(
                target: "d3d",
                "f16 overflow probe ({dev_name}): uav16={uav16:?} uav11={uav11:?} rt16={rt16:?} blend16={bld16:?} blend11={bld11:?} -> f16={ov_f16} rg11b10={ov_11} (active_mask={:#x})",
                sanitize_f16_mask(),
            );
        });
    });
}

/// Max texture scans recorded in a single diagnostic frame (`2048 * 96 B = 192 KiB`).
const MAX_ENTRIES: usize = 2048;
/// Number of `u32` words per scan slot (must match `port_diag_tex_scan_2d` in `clear.rs`).
pub const SLOT_WORDS: usize = 24;
pub const SLOT_BYTES: usize = SLOT_WORDS * 4;

fn env_target_frame() -> Option<u64> {
    static TARGET: OnceLock<Option<u64>> = OnceLock::new();
    *TARGET.get_or_init(|| {
        std::env::var("PORT_DIAG_GPU_NAN")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|&n| n >= 2)
    })
}

struct SendBuf(Retained<ProtocolObject<dyn MTLBuffer>>);
unsafe impl Send for SendBuf {}
unsafe impl Sync for SendBuf {}

struct SendTable(Retained<ProtocolObject<dyn MTL4ArgumentTable>>);
unsafe impl Send for SendTable {}
unsafe impl Sync for SendTable {}

struct SendTex(#[allow(dead_code)] Retained<MtlTexture>);
unsafe impl Send for SendTex {}
unsafe impl Sync for SendTex {}

struct EntryMeta {
    tag: &'static str,
    shader_kind: &'static str,
    shader_key: u64,
    vs_key: u64,
    res_ptr: usize,
    fmt: MTLPixelFormat,
    w: u32,
    h: u32,
    s0: u32,
    s1: u32,
    label: String,
}

struct CaptureState {
    buf: Option<SendBuf>,
    tables: Vec<SendTable>,
    entries: Vec<EntryMeta>,
    keep_views: Vec<SendTex>,
    env_fired: bool,
}

fn state() -> &'static Mutex<CaptureState> {
    static S: OnceLock<Mutex<CaptureState>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(CaptureState {
            buf: None,
            tables: Vec::new(),
            entries: Vec::new(),
            keep_views: Vec::new(),
            env_fired: false,
        })
    })
}

/// Fast-path check: true only while a single diagnostic frame is actively being recorded.
#[inline(always)]
pub fn is_capturing() -> bool {
    STATE.load(Ordering::Relaxed) == CAPTURING
}

/// Arm the scanner to capture the next full frame (safe to call from `live_ctl` or tests).
pub fn arm_next_frame() {
    let prev = STATE.swap(ARMED, Ordering::AcqRel);
    log::info!(target: "d3d", "gpunan: armed for next frame (prev_state={prev})");
}

/// Formats supported by `port_diag_tex_scan_2d` (`texture2d<float, access::read>`).
pub fn is_scannable_format(fmt: MTLPixelFormat) -> bool {
    matches!(
        fmt,
        MTLPixelFormat::RG11B10Float
            | MTLPixelFormat::RGBA16Float
            | MTLPixelFormat::RGBA32Float
            | MTLPixelFormat::RG16Float
            | MTLPixelFormat::RG32Float
            | MTLPixelFormat::R16Float
            | MTLPixelFormat::R32Float
            | MTLPixelFormat::RGB10A2Unorm
            | MTLPixelFormat::RGBA8Unorm
            | MTLPixelFormat::BGRA8Unorm
    )
}

fn ensure_buffer(st: &mut CaptureState) -> Option<u64> {
    let d = super::device()?;
    if st.buf.is_none() {
        let b = d
            .dev
            .newBufferWithLength_options(MAX_ENTRIES * SLOT_BYTES, MTLResourceOptions::StorageModeShared)?;
        d.add_resident(ProtocolObject::from_ref(&*b));
        d.commit_residency();
        st.buf = Some(SendBuf(b));
    }
    Some(st.buf.as_ref()?.0.gpuAddress())
}

fn ensure_slot_table(st: &mut CaptureState, slot: usize) -> Option<Retained<ProtocolObject<dyn MTL4ArgumentTable>>> {
    let d = super::device()?;
    while st.tables.len() <= slot {
        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxBufferBindCount(1);
        atd.setMaxTextureBindCount(1);
        let t = d.dev.newArgumentTableWithDescriptor_error(&atd).ok()?;
        st.tables.push(SendTable(t));
    }
    Some(st.tables[slot].0.clone())
}

/// Fast pre-check before splitting an encoder: true iff `res` is a scannable 2D single-sample texture.
pub fn can_scan_resource(res: &Resource) -> bool {
    if res.desc.is_buffer() {
        return false;
    }
    let Some(t) = res.texture() else {
        return false;
    };
    t.textureType() == MTLTextureType::Type2D
        && t.sampleCount() == 1
        && t.usage().contains(MTLTextureUsage::ShaderRead)
        && is_scannable_format(t.pixelFormat())
        && t.width() > 0
        && t.height() > 0
}

fn reset_buffer(st: &mut CaptureState) {
    if let Some(ref b) = st.buf {
        let ptr = b.0.contents().as_ptr() as *mut u32;
        unsafe {
            std::ptr::write_bytes(ptr, 0, MAX_ENTRIES * SLOT_WORDS);
            for i in 0..MAX_ENTRIES {
                *ptr.add(i * SLOT_WORDS + 12) = 0xFFFF_FFFFu32;
            }
        }
    }
    st.entries.clear();
    st.keep_views.clear();
}

/// Encode a GPU scan of a single 2D texture onto `cb` (caller must have closed any active encoder).
#[allow(clippy::too_many_arguments)]
pub fn encode_scan_texture(
    cb: &MtlCommandBuffer,
    tex: &MtlTexture,
    tag: &'static str,
    shader_kind: &'static str,
    shader_key: u64,
    vs_key: u64,
    res_ptr: usize,
    s0: u32,
    s1: u32,
    label: &str,
) -> bool {
    if !is_capturing() {
        return false;
    }
    if tex.textureType() != MTLTextureType::Type2D || tex.sampleCount() != 1 {
        return false;
    }
    let fmt = tex.pixelFormat();
    if !is_scannable_format(fmt) || !tex.usage().contains(MTLTextureUsage::ShaderRead) {
        return false;
    }
    let (w, h) = (tex.width() as u32, tex.height() as u32);
    if w == 0 || h == 0 {
        return false;
    }
    let use_m41_b2 = super::m41::block_read_kernels_enabled();
    let kernel_name = if use_m41_b2 {
        "port_diag_tex_scan_2d_b2"
    } else {
        "port_diag_tex_scan_2d"
    };
    let Ok(pso) = super::clear::kernel(kernel_name) else {
        return false;
    };
    let mut st = state().lock().unwrap();
    if st.entries.len() >= MAX_ENTRIES {
        return false;
    }
    let slot = st.entries.len();
    let Some(base_va) = ensure_buffer(&mut st) else {
        return false;
    };
    let Some(tbl) = ensure_slot_table(&mut st, slot) else {
        return false;
    };
    let slot_va = base_va + (slot * SLOT_BYTES) as u64;
    let Some(e) = cb.computeCommandEncoder() else {
        return false;
    };
    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
        MTLStages::All,
        MTLStages::Dispatch,
        MTL4VisibilityOptions::Device,
    );
    e.setComputePipelineState(pso);
    unsafe {
        tbl.setAddress_atIndex(slot_va, 0);
        tbl.setTexture_atIndex(tex.gpuResourceID(), 0);
        e.setArgumentTable(Some(&tbl));
        e.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize {
                width: (w as usize).div_ceil(if use_m41_b2 { 16 } else { 8 }),
                height: (h as usize).div_ceil(8),
                depth: 1,
            },
            MTLSize { width: 8, height: 8, depth: 1 },
        );
    }
    e.barrierAfterStages_beforeQueueStages_visibilityOptions(
        MTLStages::Dispatch,
        MTLStages::All,
        MTL4VisibilityOptions::Device,
    );
    e.endEncoding();

    st.entries.push(EntryMeta {
        tag,
        shader_kind,
        shader_key,
        vs_key,
        res_ptr,
        fmt,
        w,
        h,
        s0,
        s1,
        label: label.to_string(),
    });
    true
}

/// Convenience wrapper to scan a D3D12 [`Resource`]'s mip-0 2D texture view.
#[allow(clippy::too_many_arguments)]
pub fn encode_scan_resource(
    cb: &MtlCommandBuffer,
    res: &Resource,
    tag: &'static str,
    shader_kind: &'static str,
    shader_key: u64,
    vs_key: u64,
    s0: u32,
    s1: u32,
) -> bool {
    if !is_capturing() || !can_scan_resource(res) {
        return false;
    }
    let Some(t) = res.texture() else {
        return false;
    };
    let res_ptr = res as *const Resource as usize;
    if t.mipmapLevelCount() == 1 && t.arrayLength() == 1 {
        return encode_scan_texture(cb, t, tag, shader_kind, shader_key, vs_key, res_ptr, s0, s1, "");
    }
    let Some(view) = res.texture_view(t.pixelFormat(), MTLTextureType::Type2D, (0, 1), (0, 1)) else {
        return false;
    };
    let ok = encode_scan_texture(cb, &view, tag, shader_kind, shader_key, vs_key, res_ptr, s0, s1, "");
    if ok {
        state().lock().unwrap().keep_views.push(SendTex(view));
    }
    ok
}

/// Called after DXGI present `#n` (1-based) on `queue`.
///
/// * If armed (or `PORT_DIAG_GPU_NAN == n + 1`), transitions to `CAPTURING` for frame `n + 1`.
/// * If currently `CAPTURING`, stops capturing, waits for `queue` to finish, and logs the report.
pub fn on_present(n: u64, queue: &Queue) {
    if let Some(target) = env_target_frame() {
        if n + 1 == target {
            let mut st = state().lock().unwrap();
            if !st.env_fired {
                st.env_fired = true;
                STATE.store(ARMED, Ordering::Release);
            }
        }
    }
    let cur = STATE.load(Ordering::Acquire);
    if cur == IDLE {
        return;
    }
    if cur == ARMED {
        let mut st = state().lock().unwrap();
        if ensure_buffer(&mut st).is_some() {
            reset_buffer(&mut st);
            STATE.store(CAPTURING, Ordering::Release);
            log::info!(target: "d3d", "gpunan: START capturing frame #{}", n + 1);
        } else {
            STATE.store(IDLE, Ordering::Release);
            log::warn!(target: "d3d", "gpunan: failed to allocate diagnostic scan buffer");
        }
        return;
    }
    if cur == CAPTURING {
        STATE.store(IDLE, Ordering::Release);
        queue.wait_idle();
        let mut st = state().lock().unwrap();
        let total = st.entries.len();
        let Some(ref buf) = st.buf else {
            st.entries.clear();
            st.keep_views.clear();
            return;
        };
        let ptr = buf.0.contents().as_ptr() as *const u32;
        let mut bad_indices = Vec::new();
        for i in 0..total {
            let w = unsafe { std::slice::from_raw_parts(ptr.add(i * SLOT_WORDS), SLOT_WORDS) };
            let any_nan = w[0] | w[1] | w[2] | w[3];
            let any_inf = w[4] | w[5] | w[6] | w[7];
            let any_huge = w[8] | w[9] | w[10];
            if any_nan != 0 || any_inf != 0 || any_huge != 0 {
                bad_indices.push(i);
            }
        }
        log::info!(
            target: "d3d",
            "gpunan: frame #{n} DONE — scanned {total} float textures, {} flagged (nan/inf/|x|>500)",
            bad_indices.len()
        );
        let first_bad = bad_indices.first().copied();
        let mut logged = 0usize;
        for (i, meta) in st.entries.iter().enumerate() {
            let w = unsafe { std::slice::from_raw_parts(ptr.add(i * SLOT_WORDS), SLOT_WORDS) };
            let is_bad = (w[0] | w[1] | w[2] | w[3] | w[4] | w[5] | w[6] | w[7] | w[8] | w[9] | w[10]) != 0;
            let is_upscale = meta.tag.starts_with("upscale");
            let near_first_bad = first_bad.is_some_and(|fb| i + 5 >= fb && i <= fb + 10);
            if !(is_bad || is_upscale || near_first_bad) {
                continue;
            }
            if logged >= 80 && !is_upscale {
                continue;
            }
            logged += 1;
            let f = |idx: usize| f32::from_bits(w[idx]);
            let bad_xy = w[12];
            let bx = bad_xy & 0xFFFF;
            let by = bad_xy >> 16;
            log::info!(
                target: "d3d",
                "gpunan[{i:03}] {} {}({:016x}, vs={:016x}) res={:#x} {:?} {}x{} s={:#x}->{:#x} nan=({},{},{},{}) inf=({},{},{},{}) huge=({},{},{}) bad@({},{})=({:.3e},{:.3e},{:.3e}) mid=({:.3e},{:.3e},{:.3e},{:.3e}) bot=({:.3e},{:.3e},{:.3e}) {}",
                meta.tag,
                meta.shader_kind,
                meta.shader_key,
                meta.vs_key,
                meta.res_ptr,
                meta.fmt,
                meta.w,
                meta.h,
                meta.s0,
                meta.s1,
                w[0], w[1], w[2], w[3],
                w[4], w[5], w[6], w[7],
                w[8], w[9], w[10],
                bx, by,
                f(13), f(14), f(15),
                f(16), f(17), f(18), f(19),
                f(20), f(21), f(22),
                meta.label,
            );
        }
        st.entries.clear();
        st.keep_views.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metal::cmdlist::{CommandAllocator, CommandList};
    use crate::metal::queue::ListType;
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    #[test]
    fn gpunan_detects_nan_and_inf_on_gpu() {
        let Some(q) = Queue::new(ListType::Direct, "gpunan test q") else {
            return;
        };
        let desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Alignment: 0,
            Width: 16,
            Height: 16,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: 10, // R16G16B16A16_FLOAT
            SampleCount: 1,
            SampleQuality: 0,
            Layout: 0,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let res = Resource::new_committed_default(&desc, &hp, "gpunan test tex").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut cl = CommandList::new(ListType::Direct).unwrap();
        cl.begin(&alloc).unwrap();
        // Clear to (+Inf, NaN, 1.0, 1.0)
        cl.clear_uav(
            &res,
            None,
            crate::metal::clear::ClearValue::Float([f32::INFINITY, f32::NAN, 1.0, 1.0]),
            &[],
        )
        .unwrap();

        arm_next_frame();
        on_present(1, &q);
        assert!(is_capturing());
        // A 3D texture barrier must be safely skipped without throwing a Metal view exception.
        let desc3d = ResourceDesc {
            Dimension: dim::TEXTURE3D,
            Alignment: 0,
            Width: 8,
            Height: 8,
            DepthOrArraySize: 4,
            MipLevels: 1,
            Format: 10,
            SampleCount: 1,
            SampleQuality: 0,
            Layout: 0,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        };
        let res3d = Resource::new_committed_default(&desc3d, &hp, "gpunan test 3d").unwrap();
        assert!(!can_scan_resource(&res3d));
        cl.barrier_list(&[crate::metal::cmdlist::Barrier::Transition(
            &res3d as *const Resource as usize,
            0x8,
            0x40,
        )]);
        // A UAV->SRV transition barrier closes the encoder and automatically encodes our GPU scan.
        cl.barrier_list(&[crate::metal::cmdlist::Barrier::Transition(
            &res as *const Resource as usize,
            0x8,
            0x40,
        )]);
        cl.close().unwrap();
        let _ = q.submit(&[&cl.cb]);
        q.wait_idle();

        let (nan_g, inf_r, mid_b) = {
            let st = state().lock().unwrap();
            let ptr = st.buf.as_ref().unwrap().0.contents().as_ptr() as *const u32;
            let w = unsafe { std::slice::from_raw_parts(ptr, SLOT_WORDS) };
            (w[1], w[4], f32::from_bits(w[18]))
        };
        on_present(2, &q);
        assert!(!is_capturing());
        assert_eq!(inf_r, 256, "all 16x16 R pixels should be Inf");
        assert_eq!(nan_g, 256, "all 16x16 G pixels should be NaN");
        assert!((mid_b - 1.0).abs() < 1e-3, "mid B pixel should be 1.0, got {mid_b}");
    }

    #[test]
    fn clamp_f16_inf_kernel_saturates_infinities_without_touching_finite() {
        let Some(q) = Queue::new(ListType::Direct, "clamp f16 test q") else {
            return;
        };
        let desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Alignment: 0,
            Width: 8,
            Height: 8,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: 10, // R16G16B16A16_FLOAT
            SampleCount: 1,
            SampleQuality: 0,
            Layout: 0,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let res = Resource::new_committed_default(&desc, &hp, "clamp f16 tex").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut cl = CommandList::new(ListType::Direct).unwrap();
        cl.begin(&alloc).unwrap();
        cl.clear_uav(
            &res,
            None,
            crate::metal::clear::ClearValue::Float([f32::INFINITY, f32::NEG_INFINITY, 123.5, 1.0]),
            &[],
        )
        .unwrap();
        // Arm gpunan BEFORE the barrier so barrier_list runs sanitize first, closes the
        // compute encoder, and then scans the sanitized texture in order.
        arm_next_frame();
        on_present(10, &q);
        let prev = F16_OVERFLOW_MASK.swap(BIT_PROBED | BIT_F16, Ordering::AcqRel);
        cl.barrier_list(&[crate::metal::cmdlist::Barrier::Transition(
            &res as *const Resource as usize,
            0x8,
            0x40,
        )]);
        F16_OVERFLOW_MASK.store(prev, Ordering::Release);
        cl.close().unwrap();
        let _ = q.submit(&[&cl.cb]);
        q.wait_idle();

        let (inf_any, mid_r, mid_g, mid_b) = {
            let st = state().lock().unwrap();
            let ptr = st.buf.as_ref().unwrap().0.contents().as_ptr() as *const u32;
            let w = unsafe { std::slice::from_raw_parts(ptr, SLOT_WORDS) };
            (
                w[4] | w[5] | w[6] | w[7],
                f32::from_bits(w[16]),
                f32::from_bits(w[17]),
                f32::from_bits(w[18]),
            )
        };
        on_present(11, &q);
        assert_eq!(inf_any, 0, "all +/-Inf channels should be saturated to +/-65504");
        assert!((mid_r - 65504.0).abs() < 1.0, "expected +65504, got {mid_r}");
        assert!((mid_g + 65504.0).abs() < 1.0, "expected -65504, got {mid_g}");
        assert!((mid_b - 123.5).abs() < 0.25, "finite B should stay 123.5, got {mid_b}");
    }
}
