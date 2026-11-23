//! DXGI flip-model swapchain on a `CAMetalLayer`.
//!
//! The swapchain owns private `MTLTexture` back buffers (what
//! `GetBuffer` returns); `Present` blits the current one into the next
//! drawable and presents it. The layer's drawable size follows the buffer
//! size, so a window/buffer size mismatch is scaled by Core Animation
//! (DXGI_SCALING_STRETCH).
//!
//! Sync-interval-0 presents (tearing allowed) copy the back buffer into a
//! staging texture on the swapchain's queue and hand it to the present
//! thread ([`super::present_thread`]), which waits for a drawable and blits
//! and presents on a separate present queue; frames it cannot take are
//! dropped (mailbox). `nextDrawable` can block for milliseconds per frame
//! even with every present on screen, and that must not stall the app's
//! render thread.
//!
//! Vsync'd presents blit the back buffer on the render thread, on the
//! present queue, which waits for the frame's work on the swapchain's queue
//! (`waitForDrawable` would otherwise stall all later work on that queue).
//!
//! Without a layer (headless runs, `AW2_HEADLESS`) the swapchain is
//! offscreen: back buffers, barriers and `AW2_PRESENT_CAPTURE` work as usual,
//! `Present` has no drawable and paces sync-interval presents to 60 Hz.

use super::cmdlist::CommandAllocator;
use super::queue::{ListType, Queue};
use super::resource::Resource;
use crate::types::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer};
use std::sync::{Arc, Mutex};

/// Runs a closure on the main thread and waits (layer properties are
/// main-thread state). Provided by the caller (win32-rt: `macos::run_sync`).
pub type MainSync<'a> = &'a dyn Fn(Box<dyn FnOnce() + Send + '_>);

pub struct LayerRef(pub Retained<CAMetalLayer>);
unsafe impl Send for LayerRef {}
unsafe impl Sync for LayerRef {}

const RING: usize = 4;

fn vsync_disabled_by_env() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        matches!(
            std::env::var("AW2_VSYNC").as_deref(),
            Ok("0") | Ok("false") | Ok("off")
        )
    })
}

/// `AW2_PRESENT_OFF_MAIN`: offload sync-0 `present_async` back-buffer ->
/// staging-texture blit (and its `All -> Blit` queue barrier) from the main
/// render queue `q0` onto a dedicated staging `MTL4CommandQueue` (`sq`).
/// Default **on** (`AW2_PRESENT_OFF_MAIN=0` disables).
pub fn present_off_main() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_PRESENT_OFF_MAIN").is_none_or(|v| v != "0"))
}

/// `PORT_PRESENT_BLIT_NARROW` (alias `AW2_PRESENT_BLIT_NARROW`, default **on**; `=0` disables):
/// emit `MTLStages::Blit -> MTLStages::Blit` instead of `MTLStages::All -> MTLStages::Blit` on
/// dedicated swapchain blit queues `sq` and `pq` (cross-queue `q0 -> sq` and `sq -> pq` producer
/// visibility is already synchronized via `MTLSharedEvent` `wait`), while keeping
/// `MTLStages::All -> MTLStages::Blit` whenever submitting on `q0`.
pub fn present_blit_narrow() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        std::env::var_os("PORT_PRESENT_BLIT_NARROW")
            .or_else(|| std::env::var_os("AW2_PRESENT_BLIT_NARROW"))
            .is_none_or(|v| v != "0")
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RingOwner {
    Main,
    Present,
    Stage,
}

struct PresentState {
    allocs: Vec<CommandAllocator>,
    cbs: Vec<Retained<super::queue::MtlCommandBuffer>>,
    ring_values: [u64; RING],
    /// Which queue owns the ring slot's recorded command buffer / allocator.
    ring_owner: [RingOwner; RING],
    ring: usize,
    display_sync: bool,
}

pub struct Swapchain {
    /// `None` = offscreen (headless).
    pub layer: Option<LayerRef>,
    last_present: Option<std::time::Instant>,
    pub queue: Arc<Queue>,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub mtl_format: MTLPixelFormat,
    /// Active `DXGI_COLOR_SPACE_TYPE` configured on the `CAMetalLayer`.
    pub color_space: u32,
    pub buffers: Vec<Arc<Resource>>,
    pub current: usize,
    pub presents: u64,
    state: Mutex<PresentState>,
    /// Present queue (on-screen swapchains): the drawable wait and blit run
    /// here, so the main queue never stalls on drawable availability.
    pq: Option<Arc<Queue>>,
    /// Dedicated staging queue (`AW2_PRESENT_OFF_MAIN=1`): copies the rendered
    /// back buffer into a `PresentThread` staging texture off `q0`.
    sq: Option<Arc<Queue>>,
    /// Present-queue value of each back buffer's last vsync'd present (its
    /// blit read).
    buf_pv: Vec<u64>,
    last_pv: u64,
    /// Staging-queue value of each back buffer's last async staging read.
    buf_sv: Vec<u64>,
    last_sv: u64,
    /// Sync-0 presents (on-screen swapchains).
    pthread: Option<super::present_thread::PresentThread>,
    /// The last present went to the present thread.
    async_mode: bool,
    pub(crate) opt_present_off_main: bool,
    pub(crate) opt_present_blit_narrow: bool,
}
unsafe impl Send for Swapchain {}
unsafe impl Sync for Swapchain {}

// Opaque CoreGraphics color space tag matching Objective-C encoding `^{CGColorSpace=}`.
#[repr(C)]
struct CgColorSpace {
    _opaque: [u8; 0],
}
unsafe impl objc2::encode::RefEncode for CgColorSpace {
    const ENCODING_REF: objc2::encode::Encoding =
        objc2::encode::Encoding::Pointer(&objc2::encode::Encoding::Struct("CGColorSpace", &[]));
}

// CoreGraphics color space FFI (available on both macOS 14+ and iOS 17+).
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    static kCGColorSpaceSRGB: *const std::ffi::c_void;
    static kCGColorSpaceExtendedLinearSRGB: *const std::ffi::c_void;
    static kCGColorSpaceITUR_2020: *const std::ffi::c_void;
    static kCGColorSpaceITUR_2100_PQ: *const std::ffi::c_void;
    static kCGColorSpaceITUR_2100_HLG: *const std::ffi::c_void;
    fn CGColorSpaceCreateWithName(name: *const std::ffi::c_void) -> *mut CgColorSpace;
    fn CGColorSpaceRelease(space: *mut CgColorSpace);
}

/// CoreGraphics target for a `DXGI_COLOR_SPACE_TYPE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSpaceSpec {
    pub dxgi: u32,
    pub name: &'static str,
    pub edr: bool,
}

/// Map a `DXGI_COLOR_SPACE_TYPE` value to its Apple color space and EDR flag.
///
/// Leaving `CAMetalLayer.colorspace == nil` disables color matching completely,
/// causing Rec.709/sRGB game frames to display uncorrected on wide-gamut P3
/// macOS/iOS displays and breaking HDR10 PQ / scRGB output.
pub fn dxgi_color_space_spec(dxgi_cs: u32) -> Option<ColorSpaceSpec> {
    Some(match dxgi_cs {
        // DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709 (standard SDR sRGB/Rec.709)
        0 => ColorSpaceSpec { dxgi: 0, name: "kCGColorSpaceSRGB", edr: false },
        // DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709 (linear scRGB / FP16 HDR)
        1 => ColorSpaceSpec { dxgi: 1, name: "kCGColorSpaceExtendedLinearSRGB", edr: true },
        // DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709
        2 => ColorSpaceSpec { dxgi: 2, name: "kCGColorSpaceSRGB", edr: false },
        // DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020 / STUDIO_G22_NONE_P2020
        10 => ColorSpaceSpec { dxgi: 10, name: "kCGColorSpaceITUR_2020", edr: false },
        11 => ColorSpaceSpec { dxgi: 11, name: "kCGColorSpaceITUR_2020", edr: false },
        // DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020 / STUDIO_G2084_NONE_P2020 (HDR10 PQ)
        12 => ColorSpaceSpec { dxgi: 12, name: "kCGColorSpaceITUR_2100_PQ", edr: true },
        17 => ColorSpaceSpec { dxgi: 17, name: "kCGColorSpaceITUR_2100_PQ", edr: true },
        // DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P709 (HLG / PQ fallback)
        14 => ColorSpaceSpec { dxgi: 14, name: "kCGColorSpaceITUR_2100_HLG", edr: true },
        _ => return None,
    })
}

/// True when `dxgi_cs` (`DXGI_COLOR_SPACE_TYPE`) can be presented accurately on
/// Apple `CAMetalLayer`.
pub fn color_space_supported(dxgi_cs: u32) -> bool {
    dxgi_color_space_spec(dxgi_cs).is_some()
}

/// Default `DXGI_COLOR_SPACE_TYPE` for a newly created swapchain back-buffer format.
///
/// Per DXGI docs, `R16G16B16A16_FLOAT` defaults to linear scRGB (`G10_NONE_P709 = 1`);
/// all UNORM formats (`R8G8B8A8`, `B8G8R8A8`, `R10G10B10A2`) default to SDR sRGB (`0`).
pub fn default_color_space_for_format(dxgi_fmt: u32) -> u32 {
    match dxgi_fmt {
        9 | 10 => 1, // DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709
        _ => 0,      // DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709
    }
}

/// Apply `dxgi_cs` (`CAMetalLayer.colorspace` + `wantsExtendedDynamicRangeContent`)
/// on the main thread.
pub(crate) unsafe fn apply_layer_colorspace(layer: &CAMetalLayer, dxgi_cs: u32) -> Result<ColorSpaceSpec, String> {
    let spec = dxgi_color_space_spec(dxgi_cs)
        .ok_or_else(|| format!("unsupported DXGI_COLOR_SPACE_TYPE {dxgi_cs}"))?;
    let cg_name = match dxgi_cs {
        0 | 2 => kCGColorSpaceSRGB,
        1 => kCGColorSpaceExtendedLinearSRGB,
        10 | 11 => kCGColorSpaceITUR_2020,
        12 | 17 => kCGColorSpaceITUR_2100_PQ,
        14 => kCGColorSpaceITUR_2100_HLG,
        _ => kCGColorSpaceSRGB,
    };
    let cs_ref = CGColorSpaceCreateWithName(cg_name);
    if cs_ref.is_null() {
        return Err(format!("CGColorSpaceCreateWithName({}) returned null", spec.name));
    }
    let _: () = objc2::msg_send![layer, setColorspace: cs_ref];
    CGColorSpaceRelease(cs_ref);
    let _: () = objc2::msg_send![layer, setWantsExtendedDynamicRangeContent: spec.edr];
    Ok(spec)
}

/// DXGI back-buffer format + color space → layer pixel format.
///
/// On iOS, `CAMetalLayer` does not support `RGB10A2Unorm` (macOS-only):
/// - In SDR (`edr == false`), route `R10G10B10A2_*` through `BGRA8Unorm` and
///   convert via `port_present_copy_2d` / `port_present_scale_2d`.
/// - In HDR/EDR (`edr == true`, e.g. HDR10 PQ `cs = 12`), route through
///   `RGBA16Float` so all 10 bits of PQ/wide-gamut precision survive on iOS OLED.
pub fn layer_format_for(dxgi: u32, dxgi_cs: u32) -> Option<MTLPixelFormat> {
    let edr = dxgi_color_space_spec(dxgi_cs).is_some_and(|s| s.edr);
    Some(match dxgi {
        27 | 28 | 87 | 88 | 90 | 92 => MTLPixelFormat::BGRA8Unorm,
        29 | 91 | 93 => MTLPixelFormat::BGRA8Unorm_sRGB,
        #[cfg(target_os = "ios")]
        23 | 24 if edr => MTLPixelFormat::RGBA16Float,
        #[cfg(target_os = "ios")]
        23 | 24 => MTLPixelFormat::BGRA8Unorm,
        #[cfg(not(target_os = "ios"))]
        23 | 24 => {
            let _ = edr;
            MTLPixelFormat::RGB10A2Unorm
        }
        9 | 10 => MTLPixelFormat::RGBA16Float,
        _ => return None,
    })
}

/// DXGI back-buffer format → default layer pixel format.
pub fn layer_format(dxgi: u32) -> Option<MTLPixelFormat> {
    layer_format_for(dxgi, default_color_space_for_format(dxgi))
}

fn make_buffers(w: u32, h: u32, dxgi: u32, n: u32) -> Result<Vec<Arc<Resource>>, String> {
    let desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Alignment: 0,
        Width: w as u64,
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: dxgi,
        SampleCount: 1,
        SampleQuality: 0,
        Layout: 0,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
    };
    let heap = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    (0..n)
        .map(|i| {
            Resource::new_committed(&desc, &heap, MTLTextureUsage::PixelFormatView, &format!("swapchain buffer {i}"))
                .map(|r| {
                    let a = Arc::new(r);
                    a.register_ptr();
                    a
                })
                .map_err(|e| format!("{e:?}"))
        })
        .collect()
}

/// A debug capture (`AW2_PRESENT_CAPTURE`) of one present's back buffer.
struct Capture {
    path: String,
    buf: Retained<ProtocolObject<dyn MTLBuffer>>,
    w: usize,
    h: usize,
    bpp: usize,
    pf: MTLPixelFormat,
}

/// Encode the capture copy of present `n`'s back buffer, if one is due.
fn capture_encode(e: &ProtocolObject<dyn MTL4ComputeCommandEncoder>, src: &ProtocolObject<dyn MTLTexture>, n: u64) -> Option<Capture> {
    let path = if let Some(p) = super::capture::take_pending() {
        p
    } else {
        let (base, _) = super::capture::config().filter(|(_, at)| at.contains(&n))?;
        format!("{base}_{n}.png")
    };
    let d = super::device()?;
    let (w, h) = (src.width(), src.height());
    let bpp = match src.pixelFormat() {
        MTLPixelFormat::RGBA16Float => 8,
        _ => 4,
    };
    let buf = d.dev.newBufferWithLength_options(w * h * bpp, MTLResourceOptions::StorageModeShared)?;
    d.add_resident(ProtocolObject::from_ref(&*buf));
    d.commit_residency();
    unsafe {
        e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
            src,
            0,
            0,
            MTLOrigin { x: 0, y: 0, z: 0 },
            MTLSize { width: w, height: h, depth: 1 },
            &buf,
            0,
            w * bpp,
            w * h * bpp,
        )
    };
    Some(Capture { path, buf, w, h, bpp, pf: src.pixelFormat() })
}

/// Write a capture whose copy has completed.
fn capture_write(c: Capture, n: u64) {
    let Capture { path, buf, w, h, bpp, pf } = c;
    let data = unsafe { std::slice::from_raw_parts(buf.contents().as_ptr() as *const u8, w * h * bpp) };
    match super::capture::to_rgb8(pf, data, w, h, w * bpp) {
        Some(rgb) => match super::capture::write_png(&path, w as u32, h as u32, &rgb) {
            Ok(()) => log::info!(target: "d3d", "present #{n}: captured back buffer ({pf:?} {w}x{h}) -> {path}"),
            Err(e) => log::warn!(target: "d3d", "present #{n}: writing {path}: {e}"),
        },
        None => log::warn!(target: "d3d", "present #{n}: capture of {pf:?} not supported"),
    }
    if let Some(d) = super::device() {
        d.remove_resident(ProtocolObject::from_ref(&*buf));
    }
}

impl Swapchain {
    /// # Safety
    /// `layer` must be a live `CAMetalLayer*`, or null for an offscreen
    /// (headless) swapchain.
    pub unsafe fn new(layer: *mut std::ffi::c_void, queue: Arc<Queue>, w: u32, h: u32, dxgi: u32, count: u32, main: MainSync) -> Result<Swapchain, String> {
        let d = super::device().ok_or("no device")?;
        if queue.kind != ListType::Direct {
            return Err(format!("swapchain on a {:?} queue", queue.kind));
        }
        let init_cs = default_color_space_for_format(dxgi);
        let mtl_format = layer_format_for(dxgi, init_cs).ok_or_else(|| format!("unsupported back buffer format {}", crate::format::name(dxgi)))?;
        let layer = Retained::retain(layer as *mut CAMetalLayer).map(LayerRef);
        let init_sync = !vsync_disabled_by_env();
        if let Some(layer) = &layer {
            let l = layer;
            let dev = &d.dev;
            main(Box::new(move || {
                l.0.setDevice(Some(dev));
                l.0.setPixelFormat(mtl_format);
                l.0.setFramebufferOnly(false);
                l.0.setMaximumDrawableCount(3);
                #[cfg(target_os = "macos")]
                l.0.setDisplaySyncEnabled(init_sync);
                l.0.setDrawableSize(objc2_core_foundation::CGSize { width: w as f64, height: h as f64 });
                if let Err(e) = unsafe { apply_layer_colorspace(&l.0, init_cs) } {
                    log::warn!(target: "d3d", "swapchain: initial colorspace({init_cs}) failed: {e}");
                }
            }));
        }
        let buffers = make_buffers(w, h, dxgi, count)?;
        let mut allocs = Vec::new();
        let mut cbs = Vec::new();
        for _ in 0..RING {
            allocs.push(CommandAllocator::new(ListType::Direct).ok_or("allocator")?);
            cbs.push(d.dev.newCommandBuffer().ok_or("command buffer")?);
        }
        let pq = match &layer {
            Some(l) => {
                let pq = Queue::new(ListType::Direct, "present").ok_or("present queue")?;
                pq.q.addResidencySet(&l.0.residencySet());
                Some(Arc::new(pq))
            }
            None => None,
        };
        let sq = match &layer {
            Some(_) => Some(Arc::new(Queue::new(ListType::Direct, "present staging").ok_or("present staging queue")?)),
            None => None,
        };
        let pthread = match (&layer, &pq) {
            (Some(l), Some(pq)) => {
                let pt = super::present_thread::PresentThread::new(l, pq.clone(), queue.clone(), sq.clone())?;
                pt.set_staging(w, h, staging_format(&buffers)?)?;
                Some(pt)
            }
            _ => None,
        };
        let cs_name = dxgi_color_space_spec(init_cs).map(|s| s.name).unwrap_or("unknown");
        log::info!(target: "d3d", "swapchain: {w}x{h} {} ({mtl_format:?}, {cs_name}), {count} buffers{}", crate::format::name(dxgi), if layer.is_none() { ", offscreen" } else { "" });
        Ok(Swapchain {
            layer,
            last_present: None,
            queue,
            width: w,
            height: h,
            format: dxgi,
            mtl_format,
            color_space: init_cs,
            buffers,
            current: 0,
            presents: 0,
            state: Mutex::new(PresentState {
                allocs,
                cbs,
                ring_values: [0; RING],
                ring_owner: [RingOwner::Present; RING],
                ring: 0,
                display_sync: init_sync,
            }),
            pq,
            sq,
            buf_pv: vec![0; count as usize],
            last_pv: 0,
            buf_sv: vec![0; count as usize],
            last_sv: 0,
            pthread,
            async_mode: false,
            opt_present_off_main: present_off_main(),
            opt_present_blit_narrow: present_blit_narrow(),
        })
    }

    /// `SetColorSpace1`: configure `CAMetalLayer.colorspace`, EDR headroom, and
    /// (on iOS when switching `R10G10B10A2` between SDR and HDR) the drawable format.
    pub fn set_color_space(&mut self, dxgi_cs: u32, main: MainSync) -> Result<(), String> {
        let spec = dxgi_color_space_spec(dxgi_cs)
            .ok_or_else(|| format!("unsupported DXGI_COLOR_SPACE_TYPE {dxgi_cs}"))?;
        let new_mtl = layer_format_for(self.format, dxgi_cs)
            .ok_or_else(|| format!("unsupported back buffer format {}", crate::format::name(self.format)))?;
        if self.color_space == dxgi_cs && self.mtl_format == new_mtl {
            return Ok(());
        }
        if new_mtl != self.mtl_format {
            if let Some(pt) = &self.pthread {
                pt.drain();
            }
            if let Some(pq) = &self.pq {
                pq.wait_idle();
            }
        }
        if let Some(l) = &self.layer {
            let mut err: Option<String> = None;
            let err_ref = &mut err;
            let prev_mtl = self.mtl_format;
            main(Box::new(move || {
                if new_mtl != prev_mtl {
                    l.0.setPixelFormat(new_mtl);
                }
                if let Err(e) = unsafe { apply_layer_colorspace(&l.0, dxgi_cs) } {
                    *err_ref = Some(e);
                }
            }));
            if let Some(e) = err {
                return Err(e);
            }
        }
        self.color_space = dxgi_cs;
        self.mtl_format = new_mtl;
        log::info!(
            target: "d3d",
            "swapchain: SetColorSpace1({dxgi_cs} -> {}, edr={}, layer={new_mtl:?})",
            spec.name,
            spec.edr,
        );
        Ok(())
    }

    /// `ResizeBuffers` (the caller checks that no buffer references remain).
    pub fn resize(&mut self, w: u32, h: u32, dxgi: u32, count: u32, main: MainSync) -> Result<(), String> {
        if let Some(pt) = &self.pthread {
            pt.drain();
        }
        self.queue.wait_idle();
        if let Some(sq) = &self.sq {
            sq.wait_idle();
        }
        if let Some(pq) = &self.pq {
            pq.wait_idle();
        }
        let cs = self.color_space;
        let mtl_format = layer_format_for(dxgi, cs).ok_or_else(|| format!("unsupported back buffer format {}", crate::format::name(dxgi)))?;
        if let Some(l) = &self.layer {
            main(Box::new(move || {
                l.0.setPixelFormat(mtl_format);
                l.0.setDrawableSize(objc2_core_foundation::CGSize { width: w as f64, height: h as f64 });
                let _ = unsafe { apply_layer_colorspace(&l.0, cs) };
            }));
        }
        self.buffers = make_buffers(w, h, dxgi, count)?;
        if let Some(pt) = &self.pthread {
            pt.set_staging(w, h, staging_format(&self.buffers)?)?;
        }
        self.buf_pv = vec![0; count as usize];
        self.last_pv = 0;
        self.buf_sv = vec![0; count as usize];
        self.last_sv = 0;
        self.width = w;
        self.height = h;
        self.format = dxgi;
        self.mtl_format = mtl_format;
        self.current = 0;
        Ok(())
    }

    /// Present the current back buffer. Returns the swapchain queue's
    /// timeline value covering the frame's work (for the frame-latency
    /// waitable).
    pub fn present(&mut self, sync_interval: u32) -> Result<u64, String> {
        objc2::rc::autoreleasepool(|_| {
            let want_sync = sync_interval > 0 && !vsync_disabled_by_env();
            {
                let mut st = self.state.lock().unwrap();
                if st.display_sync != want_sync {
                    st.display_sync = want_sync;
                    #[cfg(target_os = "macos")]
                    if let Some(l) = &self.layer {
                        l.0.setDisplaySyncEnabled(want_sync);
                    }
                }
            }
            let r = if !want_sync && self.pthread.is_some() { self.present_async() } else { self.present_sync(sync_interval, want_sync) };
            if r.is_err() {
                return r;
            }
            self.current = (self.current + 1) % self.buffers.len();
            self.presents += 1;
            r
        })
    }

    /// Sync-0 on-screen present: copy the back buffer into a staging texture
    /// (on `sq` when `AW2_PRESENT_OFF_MAIN=1`, otherwise on `self.queue`) and
    /// hand it to the present thread.
    fn present_async(&mut self) -> Result<u64, String> {
        let (Some(pq), Some(pt)) = (self.pq.as_deref(), self.pthread.as_ref()) else {
            return Err("no present thread".into());
        };
        if !self.async_mode {
            // Back buffers may still be read by vsync'd blits.
            if pq.timeline.completed() < self.last_pv {
                self.queue.wait(&pq.timeline, self.last_pv);
                if let Some(sq) = self.sq.as_deref() {
                    sq.wait(&pq.timeline, self.last_pv);
                }
            }
            self.async_mode = true;
        }
        let n = self.presents + 1;
        let capture_due = super::capture::has_pending()
            || super::capture::config().is_some_and(|(_, at)| at.contains(&n));
        let slot = match pt.free_slot() {
            Some(s) => s,
            None if capture_due => {
                // At most two slots are queued; the third's blit is in flight.
                if let Some(sq) = self.sq.as_deref() {
                    sq.wait_idle();
                }
                pq.wait_idle();
                pt.free_slot().ok_or("no free staging texture")?
            }
            None => {
                super::encstats::inc(&super::encstats::c::present_drop);
                return Ok(self.queue.flush_timeline());
            }
        };
        let dst = pt.staging(slot).ok_or("no staging texture")?;
        let src = self.buffers[self.current].texture().ok_or("back buffer is not a texture")?;
        let mut st = self.state.lock().unwrap();
        let i = st.ring;
        st.ring = (i + 1) % RING;
        let ring_q: &Queue = match st.ring_owner[i] {
            RingOwner::Main => &self.queue,
            RingOwner::Stage => self.sq.as_deref().unwrap_or(&self.queue),
            RingOwner::Present => pq,
        };
        ring_q.timeline.wait_cpu(st.ring_values[i], u64::MAX);
        st.allocs[i].reset();
        let cb = st.cbs[i].clone();
        cb.beginCommandBufferWithAllocator(&st.allocs[i].a);
        let e = cb.computeCommandEncoder().ok_or("compute encoder")?;
        let use_sq = self.opt_present_off_main && self.sq.is_some();
        let after_stages = if use_sq && self.opt_present_blit_narrow {
            super::encstats::inc(&super::encstats::c::present_blit_narrow);
            MTLStages::Blit
        } else {
            MTLStages::All
        };
        e.barrierAfterQueueStages_beforeStages_visibilityOptions(after_stages, MTLStages::Blit, MTL4VisibilityOptions::Device);
        let keep_views = super::present_thread::blit(&e, src, &dst);
        let capture = capture_encode(&e, src, n);
        e.endEncoding();
        cb.endCommandBuffer();
        if self.opt_present_off_main {
            if let Some(sq) = self.sq.as_deref() {
                super::encstats::inc(&super::encstats::c::present_sq);
                let vm = self.queue.flush_timeline();
                sq.wait(&self.queue.timeline, vm);
                let sv = sq.submit(&[&cb]);
                if !keep_views.is_empty() {
                    sq.defer_release(Box::new(keep_views));
                }
                st.ring_values[i] = sv;
                st.ring_owner[i] = RingOwner::Stage;
                self.buf_sv[self.current] = sv;
                self.last_sv = sv;
                // Protect the next frame's back buffer if its earlier staging read
                // on `sq` is somehow still in flight.
                let nb = (self.current + 1) % self.buffers.len();
                if self.buf_sv[nb] > sq.timeline.completed() {
                    super::encstats::inc(&super::encstats::c::present_sq_wait);
                    self.queue.wait(&sq.timeline, self.buf_sv[nb]);
                }
                drop(st);
                pt.claim(slot);
                pt.submit(slot, sv, true);
                if let Some(c) = capture {
                    sq.timeline.wait_cpu(sv, u64::MAX);
                    capture_write(c, n);
                }
                return Ok(vm);
            }
        }
        let v = self.queue.submit(&[&cb]);
        if !keep_views.is_empty() {
            self.queue.defer_release(Box::new(keep_views));
        }
        st.ring_values[i] = v;
        st.ring_owner[i] = RingOwner::Main;
        drop(st);
        pt.claim(slot);
        pt.submit(slot, v, false);
        if let Some(c) = capture {
            self.queue.timeline.wait_cpu(v, u64::MAX);
            capture_write(c, n);
        }
        Ok(v)
    }

    /// Vsync'd on-screen present (blit on the present queue from this
    /// thread) or offscreen present.
    fn present_sync(&mut self, sync_interval: u32, _want_sync: bool) -> Result<u64, String> {
        if self.async_mode {
            if let Some(pt) = &self.pthread {
                pt.drain();
            }
            if let Some(sq) = self.sq.as_deref() {
                if sq.timeline.completed() < self.last_sv {
                    self.queue.wait(&sq.timeline, self.last_sv);
                }
            }
            self.async_mode = false;
        }
        let pq = self.pq.clone();
        let mut st = self.state.lock().unwrap();
        let pres_q: &Queue = pq.as_deref().unwrap_or(&self.queue);
        let i = st.ring;
        // Wait for the ring slot's previous submission BEFORE acquiring `nextDrawable()`
        // so a CAMetalDrawable is never held across a CPU timeline wait.
        let ring_q: &Queue = match st.ring_owner[i] {
            RingOwner::Main => &self.queue,
            RingOwner::Stage => self.sq.as_deref().unwrap_or(&self.queue),
            RingOwner::Present => pres_q,
        };
        ring_q.timeline.wait_cpu(st.ring_values[i], u64::MAX);
        st.allocs[i].reset();
        let t_acq = super::encstats::active().then(std::time::Instant::now);
        let drawable = match &self.layer {
            Some(l) => Some(l.0.nextDrawable().ok_or("nextDrawable returned nil")?),
            None => None,
        };
        if let Some(t) = t_acq {
            super::encstats::add(&super::encstats::c::present_acq_us, t.elapsed().as_micros() as u64);
        }
        st.ring = (i + 1) % RING;
        let cb = st.cbs[i].clone();
        cb.beginCommandBufferWithAllocator(&st.allocs[i].a);
        let src = self.buffers[self.current].texture().ok_or("back buffer is not a texture")?;
        let e = cb.computeCommandEncoder().ok_or("compute encoder")?;
        let use_pq = drawable.is_some() && pq.is_some();
        let after_stages = if use_pq && self.opt_present_blit_narrow {
            super::encstats::inc(&super::encstats::c::present_blit_narrow);
            MTLStages::Blit
        } else {
            MTLStages::All
        };
        let dst_tex = drawable.as_ref().map(|d| d.texture());
        let compute_blit = dst_tex.as_ref().is_some_and(|dst| super::present_thread::needs_compute_blit(src, dst));
        if self.presents == 0 {
            if let Some(dst) = &dst_tex {
                log::info!(
                    target: "d3d",
                    "swapchain first present: src={:?} {}x{}, dst={:?} {}x{}, compute_blit={compute_blit}",
                    src.pixelFormat(),
                    src.width(),
                    src.height(),
                    dst.pixelFormat(),
                    dst.width(),
                    dst.height(),
                );
            }
        }
        let before_stages = if compute_blit {
            MTLStages::Blit | MTLStages::Dispatch
        } else {
            MTLStages::Blit
        };
        e.barrierAfterQueueStages_beforeStages_visibilityOptions(after_stages, before_stages, MTL4VisibilityOptions::Device);
        let keep_views = if let Some(dst) = &dst_tex {
            super::present_thread::blit(&e, src, dst)
        } else {
            Vec::new()
        };
        let n = self.presents + 1;
        let capture = capture_encode(&e, src, n);
        e.endEncoding();
        cb.endCommandBuffer();
        let (v, pv, owner) = match &drawable {
            Some(drawable) => {
                let md: &ProtocolObject<dyn MTLDrawable> = ProtocolObject::from_ref(&**drawable);
                let pq = pq.as_ref().ok_or("no present queue")?;
                super::encstats::inc(&super::encstats::c::present_pq);
                let vm = self.queue.flush_timeline();
                pq.wait(&self.queue.timeline, vm);
                pq.q.waitForDrawable(md);
                let pv = pq.submit(&[&cb]);
                if !keep_views.is_empty() {
                    pq.defer_release(Box::new(keep_views));
                }
                pq.q.signalDrawable(md);
                md.present();
                self.buf_pv[self.current] = pv;
                self.last_pv = pv;
                // The next frame renders into the next buffer: its
                // previous present's blit must have read it (which also
                // covers `pv` directly when `buffers.len() == 1`).
                let nb = (self.current + 1) % self.buffers.len();
                if self.buf_pv[nb] > pq.timeline.completed() {
                    super::encstats::inc(&super::encstats::c::present_bbwait);
                    self.queue.wait(&pq.timeline, self.buf_pv[nb]);
                }
                (vm, pv, RingOwner::Present)
            }
            None => {
                let v = self.queue.submit(&[&cb]);
                // Offscreen: no display to wait on; pace vsync'd presents.
                if sync_interval > 0 {
                    let period = std::time::Duration::from_micros(16_667 * sync_interval.min(4) as u64);
                    if let Some(t) = self.last_present {
                        if let Some(rest) = period.checked_sub(t.elapsed()) {
                            std::thread::sleep(rest);
                        }
                    }
                }
                self.last_present = Some(std::time::Instant::now());
                (v, v, RingOwner::Main)
            }
        };
        st.ring_values[i] = pv;
        st.ring_owner[i] = owner;
        drop(st);
        if let Some(c) = capture {
            pres_q.timeline.wait_cpu(pv, u64::MAX);
            capture_write(c, n);
        }
        Ok(v)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        // Stop the present thread and wait for all queues to go idle before
        // Rust drops `self.buffers` and `self.state` (command allocators).
        drop(self.pthread.take());
        self.queue.wait_idle();
        if let Some(sq) = &self.sq {
            sq.wait_idle();
        }
        if let Some(pq) = &self.pq {
            pq.wait_idle();
        }
    }
}

/// Staging texture format: the back buffers' own Metal format.
fn staging_format(buffers: &[Arc<Resource>]) -> Result<MTLPixelFormat, String> {
    Ok(buffers.first().and_then(|b| b.texture()).ok_or("back buffer is not a texture")?.pixelFormat())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metal::cmdlist::{CommandList, TargetView};
    use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};
    use std::sync::atomic::Ordering::Relaxed;

    #[test]
    fn present_off_main_offloads_staging_blit_to_sq_and_preserves_pixels_across_resize_on_gpu() {
        if std::env::var_os("AW2_PRESENT_OFF_MAIN").is_none() {
            assert!(super::present_off_main());
        }
        if super::super::device().is_none() {
            return;
        }
        let q0 = Arc::new(Queue::new(ListType::Direct, "g32 main q0").unwrap());
        let layer = CAMetalLayer::new();
        let layer_ptr = Retained::as_ptr(&layer) as *mut std::ffi::c_void;
        let mut sc = unsafe {
            Swapchain::new(layer_ptr, q0.clone(), 8, 8, 28, 2, &|f| f()).unwrap()
        };
        sc.opt_present_off_main = true;
        assert!(sc.sq.is_some());

        // Clear backbuffer 0 to bright green on `q0`.
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut cl = CommandList::new(ListType::Direct).unwrap();
        let rtv0 = TargetView::rtv(&sc.buffers[0], None).unwrap();
        cl.begin(&alloc).unwrap();
        unsafe { cl.clear_rtv(&rtv0, [0.0, 1.0, 0.0, 1.0], &[]).unwrap() };
        cl.close().unwrap();
        let vm1 = q0.submit(&[&cl.cb]);

        let sq_cnt0 = super::super::encstats::c::present_sq.load(Relaxed);
        let ret_v = sc.present_async().unwrap();
        sc.current = (sc.current + 1) % sc.buffers.len();
        sc.presents += 1;

        assert_eq!(ret_v, vm1, "present_async on sq must return q0's last_submitted value");
        assert_eq!(q0.last_submitted(), vm1, "present_async must NOT commit onto q0");
        assert!(super::super::encstats::c::present_sq.load(Relaxed) > sq_cnt0);
        let sq = sc.sq.as_ref().unwrap().clone();
        assert!(sq.last_submitted() >= 1);

        // Drain the present thread and read back `staging(0)` on `q0` after waiting on `sq.timeline`.
        sc.pthread.as_ref().unwrap().drain();
        let staged = sc.pthread.as_ref().unwrap().staging(0).unwrap();
        let rb_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 256 * 8,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let rb = Resource::new_committed(
            &rb_desc,
            &HeapProperties { Type: heap_type::READBACK, ..Default::default() },
            MTLTextureUsage::empty(),
            "g32 staged rb",
        )
        .unwrap();
        let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
        let cb2 = super::super::device().unwrap().dev.newCommandBuffer().unwrap();
        cb2.beginCommandBufferWithAllocator(&alloc2.a);
        let enc = cb2.computeCommandEncoder().unwrap();
        enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
            MTLStages::All,
            MTLStages::Blit,
            MTL4VisibilityOptions::Device,
        );
        unsafe {
            enc.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
                &staged,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize { width: 8, height: 8, depth: 1 },
                rb.buffer().unwrap(),
                0,
                256,
                256 * 8,
            );
        }
        enc.endEncoding();
        cb2.endCommandBuffer();
        q0.wait(&sq.timeline, sc.last_sv);
        q0.submit(&[&cb2]);
        q0.wait_idle();

        let px0 = unsafe { *(rb.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(px0, 0xff00_ff00, "staging texture on sq must contain green backbuffer pixels");

        // Verify `resize` drains `pt` + `sq` cleanly and resets `buf_sv`/`last_sv`.
        sc.resize(16, 16, 28, 2, &|f| f()).unwrap();
        assert_eq!(sc.last_sv, 0);
        assert_eq!(sc.buf_sv, vec![0, 0]);
        let rtv_resized = TargetView::rtv(&sc.buffers[0], None).unwrap();
        alloc.reset();
        cl.begin(&alloc).unwrap();
        unsafe { cl.clear_rtv(&rtv_resized, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
        cl.close().unwrap();
        let vm2 = q0.submit(&[&cl.cb]);
        let ret_v2 = sc.present_async().unwrap();
        assert_eq!(ret_v2, vm2);
        sc.pthread.as_ref().unwrap().drain();
    }

    #[test]
    fn submit_coalesce_preserves_nonblack_swapchain_present_and_sq_timeline_on_gpu() {
        if super::super::device().is_none() {
            return;
        }
        let q0 = Arc::new(Queue::new(ListType::Direct, "g35 coalesce q0").unwrap());
        q0.set_submit_coalesce(true);
        let layer = CAMetalLayer::new();
        let layer_ptr = Retained::as_ptr(&layer) as *mut std::ffi::c_void;
        let mut sc = unsafe {
            Swapchain::new(layer_ptr, q0.clone(), 8, 8, 28, 2, &|f| f()).unwrap()
        };
        sc.opt_present_off_main = true;
        // Simulate process-wide `AW2_SUBMIT_COALESCE=1` across all Direct queues (`q0`, `sq`, `pq`).
        if let Some(sq) = &sc.sq {
            sq.set_submit_coalesce(true);
        }
        if let Some(pq) = &sc.pq {
            pq.set_submit_coalesce(true);
        }

        // Submit backbuffer 0 green clear via `submit_d3d_lists` (`ExecuteCommandLists` path).
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut cl = CommandList::new(ListType::Direct).unwrap();
        let rtv0 = TargetView::rtv(&sc.buffers[0], None).unwrap();
        cl.begin(&alloc).unwrap();
        unsafe { cl.clear_rtv(&rtv0, [0.0, 1.0, 0.0, 1.0], &[]).unwrap() };
        cl.close().unwrap();
        assert_eq!(
            q0.submit_d3d_lists(&[&cl.cb], Vec::new()),
            0,
            "D3D12 ExecuteCommandLists on q0 must coalesce lazily before Present"
        );

        let ret_v = sc.present_async().unwrap();
        sc.current = (sc.current + 1) % sc.buffers.len();
        sc.presents += 1;

        assert!(ret_v >= 1, "present_async must flush q0.last_submitted() to >= 1");
        assert!(sc.last_sv >= 1, "sq.submit must eagerly signal sq.timeline >= 1 (regression fix for g34_C black%=100%)");
        let sq = sc.sq.as_ref().unwrap().clone();
        sq.timeline.wait_cpu(sc.last_sv, u64::MAX);
        sc.pthread.as_ref().unwrap().drain();

        let staged = sc.pthread.as_ref().unwrap().staging(0).unwrap();
        let rb_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 256 * 8,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let rb = Resource::new_committed(
            &rb_desc,
            &HeapProperties { Type: heap_type::READBACK, ..Default::default() },
            MTLTextureUsage::empty(),
            "g35 coalesce staged rb",
        )
        .unwrap();
        let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
        let cb2 = super::super::device().unwrap().dev.newCommandBuffer().unwrap();
        cb2.beginCommandBufferWithAllocator(&alloc2.a);
        let enc = cb2.computeCommandEncoder().unwrap();
        enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
            MTLStages::All,
            MTLStages::Blit,
            MTL4VisibilityOptions::Device,
        );
        unsafe {
            enc.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
                &staged,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize { width: 8, height: 8, depth: 1 },
                rb.buffer().unwrap(),
                0,
                256,
                256 * 8,
            );
        }
        enc.endEncoding();
        cb2.endCommandBuffer();
        q0.wait(&sq.timeline, sc.last_sv);
        let read_v = q0.submit(&[&cb2]);
        assert!(read_v > ret_v);
        q0.wait_idle();

        let px0 = unsafe { *(rb.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(px0, 0xff00_ff00, "staging texture after coalesced q0 + eager sq must be green (0% black)");
    }

    #[test]
    fn present_blit_narrow_narrows_sq_and_pq_while_preserving_main_queue_guard_on_gpu() {
        if std::env::var_os("PORT_PRESENT_BLIT_NARROW").is_none()
            && std::env::var_os("AW2_PRESENT_BLIT_NARROW").is_none()
        {
            assert!(super::present_blit_narrow(), "PORT_PRESENT_BLIT_NARROW must default ON");
        }
        if super::super::device().is_none() {
            return;
        }
        let q0 = Arc::new(Queue::new(ListType::Direct, "g36 narrow q0").unwrap());
        let layer = CAMetalLayer::new();
        let layer_ptr = Retained::as_ptr(&layer) as *mut std::ffi::c_void;
        let mut sc = unsafe {
            Swapchain::new(layer_ptr, q0.clone(), 8, 8, 28, 2, &|f| f()).unwrap()
        };
        sc.opt_present_off_main = true;
        sc.opt_present_blit_narrow = true;
        sc.pthread.as_ref().unwrap().set_present_blit_narrow(true);

        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut cl = CommandList::new(ListType::Direct).unwrap();
        let rtv0 = TargetView::rtv(&sc.buffers[0], None).unwrap();
        cl.begin(&alloc).unwrap();
        unsafe { cl.clear_rtv(&rtv0, [0.0, 1.0, 0.0, 1.0], &[]).unwrap() };
        cl.close().unwrap();
        let _ = q0.submit_d3d_lists(&[&cl.cb], Vec::new());

        let narrow0 = super::super::encstats::c::present_blit_narrow.load(Relaxed);
        let ret_v = sc.present_async().unwrap();
        assert!(ret_v >= 1);
        sc.pthread.as_ref().unwrap().drain();
        let narrow1 = super::super::encstats::c::present_blit_narrow.load(Relaxed);
        assert!(
            narrow1 >= narrow0 + 1,
            "present_async on sq (+ pq present_one) must use narrow Blit -> Blit queue barrier"
        );

        let sq = sc.sq.as_ref().unwrap().clone();
        let staged = sc.pthread.as_ref().unwrap().staging(0).unwrap();
        let rb_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 256 * 8,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let rb = Resource::new_committed(
            &rb_desc,
            &HeapProperties { Type: heap_type::READBACK, ..Default::default() },
            MTLTextureUsage::empty(),
            "g36 narrow staged rb",
        )
        .unwrap();
        let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
        let cb2 = super::super::device().unwrap().dev.newCommandBuffer().unwrap();
        cb2.beginCommandBufferWithAllocator(&alloc2.a);
        let enc = cb2.computeCommandEncoder().unwrap();
        enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
            MTLStages::All,
            MTLStages::Blit,
            MTL4VisibilityOptions::Device,
        );
        unsafe {
            enc.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
                &staged,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize { width: 8, height: 8, depth: 1 },
                rb.buffer().unwrap(),
                0,
                256,
                256 * 8,
            );
        }
        enc.endEncoding();
        cb2.endCommandBuffer();
        q0.wait(&sq.timeline, sc.last_sv);
        q0.submit(&[&cb2]);
        q0.wait_idle();
        let px0 = unsafe { *(rb.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(px0, 0xff00_ff00, "narrow Blit -> Blit on sq must preserve green pixels");

        // Fallback onto `q0` (`opt_present_off_main = false`, plus offscreen `present_sync`):
        // must keep `MTLStages::All -> MTLStages::Blit` (no `present_blit_narrow` increment on `q0`).
        sc.current = 1;
        sc.presents += 1;
        sc.opt_present_off_main = false;
        sc.pthread.as_ref().unwrap().set_present_blit_narrow(false);
        let rtv1 = TargetView::rtv(&sc.buffers[1], None).unwrap();
        alloc.reset();
        cl.begin(&alloc).unwrap();
        unsafe { cl.clear_rtv(&rtv1, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
        cl.close().unwrap();
        let _ = q0.submit_d3d_lists(&[&cl.cb], Vec::new());

        let narrow_before_q0 = super::super::encstats::c::present_blit_narrow.load(Relaxed);
        let _ = sc.present_async().unwrap();
        sc.pthread.as_ref().unwrap().drain();
        assert_eq!(
            super::super::encstats::c::present_blit_narrow.load(Relaxed),
            narrow_before_q0,
            "main-queue fallback must keep MTLStages::All -> MTLStages::Blit"
        );

        let mut offscreen = unsafe {
            Swapchain::new(std::ptr::null_mut(), q0.clone(), 8, 8, 28, 2, &|f| f()).unwrap()
        };
        offscreen.opt_present_blit_narrow = true;
        let narrow_before_offscreen = super::super::encstats::c::present_blit_narrow.load(Relaxed);
        offscreen.present_sync(0, false).unwrap();
        q0.wait_idle();
        assert_eq!(
            super::super::encstats::c::present_blit_narrow.load(Relaxed),
            narrow_before_offscreen,
            "offscreen present_sync on q0 must keep MTLStages::All -> MTLStages::Blit"
        );
    }

    #[test]
    fn present_blit_swizzles_rgba8_to_bgra8_and_srgb_without_channel_swap_on_gpu() {
        let Some(d) = super::super::device() else { return };
        let q = Arc::new(Queue::new(ListType::Direct, "present swizzle test").unwrap());

        for (src_fmt, dst_fmt, rgba_bytes) in [
            (MTLPixelFormat::RGBA8Unorm, MTLPixelFormat::BGRA8Unorm, [0xFFu8, 0x40, 0x10, 0xFF]),
            (MTLPixelFormat::RGBA8Unorm_sRGB, MTLPixelFormat::BGRA8Unorm_sRGB, [0xCCu8, 0x55, 0x22, 0xFF]),
        ] {
            let src_td = unsafe {
                MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(src_fmt, 4, 4, false)
            };
            src_td.setStorageMode(MTLStorageMode::Shared);
            src_td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView);
            let src = d.dev.newTextureWithDescriptor(&src_td).unwrap();

            let dst_td = unsafe {
                MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(dst_fmt, 4, 4, false)
            };
            dst_td.setStorageMode(MTLStorageMode::Shared);
            dst_td.setUsage(
                if dst_fmt == MTLPixelFormat::BGRA8Unorm_sRGB {
                    MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView
                } else {
                    MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite | MTLTextureUsage::PixelFormatView
                },
            );
            let dst = d.dev.newTextureWithDescriptor(&dst_td).unwrap();

            // Fill source with 4x4 copies of `rgba_bytes` = [R, G, B, A].
            let src_pixels: Vec<u8> = rgba_bytes.iter().copied().cycle().take(4 * 4 * 4).collect();
            unsafe {
                src.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                    MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                    0,
                    std::ptr::NonNull::new(src_pixels.as_ptr() as *mut std::ffi::c_void).unwrap(),
                    16,
                );
            }

            d.add_resident(ProtocolObject::from_ref(&*src));
            d.add_resident(ProtocolObject::from_ref(&*dst));
            d.commit_residency();

            assert!(super::super::present_thread::needs_compute_blit(&src, &dst));

            let alloc = CommandAllocator::new(ListType::Direct).unwrap();
            let cb = d.dev.newCommandBuffer().unwrap();
            cb.beginCommandBufferWithAllocator(&alloc.a);
            let enc = cb.computeCommandEncoder().unwrap();
            let keep = super::super::present_thread::blit(&enc, &src, &dst);
            enc.endEncoding();
            cb.endCommandBuffer();

            q.submit(&[&cb]);
            if !keep.is_empty() {
                q.defer_release(Box::new(keep));
            }
            q.wait_idle();

            let mut out = [0u8; 4 * 4 * 4];
            unsafe {
                dst.getBytes_bytesPerRow_fromRegion_mipmapLevel(
                    std::ptr::NonNull::new(out.as_mut_ptr() as *mut std::ffi::c_void).unwrap(),
                    16,
                    MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                    0,
                );
            }
            // BGRA8 memory order must be [B, G, R, A] (swizzled from [R, G, B, A]).
            let expect_bgra = [rgba_bytes[2], rgba_bytes[1], rgba_bytes[0], rgba_bytes[3]];
            for px in 0..16 {
                assert_eq!(
                    &out[px * 4..px * 4 + 4],
                    &expect_bgra,
                    "present blit {:?} -> {:?} pixel {px} must preserve R/G/B channels without swap",
                    src_fmt,
                    dst_fmt
                );
            }

            d.remove_resident(ProtocolObject::from_ref(&*src));
            d.remove_resident(ProtocolObject::from_ref(&*dst));
        }
    }

    #[test]
    fn present_blit_converts_rgb10a2_to_bgra8_on_gpu() {
        let Some(d) = super::super::device() else { return };
        let q = Arc::new(Queue::new(ListType::Direct, "present rgb10a2 test").unwrap());

        let src_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::RGB10A2Unorm,
                4,
                4,
                false,
            )
        };
        src_td.setStorageMode(MTLStorageMode::Shared);
        src_td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView);
        let src = d.dev.newTextureWithDescriptor(&src_td).unwrap();

        let dst_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::BGRA8Unorm,
                4,
                4,
                false,
            )
        };
        dst_td.setStorageMode(MTLStorageMode::Shared);
        dst_td.setUsage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        let dst = d.dev.newTextureWithDescriptor(&dst_td).unwrap();

        // Pack RGB10A2: R=1023 (~1.0 -> 0xFF), G=512 (~0.5 -> 0x80), B=256 (~0.25 -> 0x40), A=3 (1.0 -> 0xFF).
        let packed: u32 = 1023u32 | (512u32 << 10) | (256u32 << 20) | (3u32 << 30);
        let src_pixels: Vec<u8> = (0..16).flat_map(|_| packed.to_le_bytes()).collect();
        unsafe {
            src.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                0,
                std::ptr::NonNull::new(src_pixels.as_ptr() as *mut std::ffi::c_void).unwrap(),
                16,
            );
        }

        d.add_resident(ProtocolObject::from_ref(&*src));
        d.add_resident(ProtocolObject::from_ref(&*dst));
        d.commit_residency();

        assert!(super::super::present_thread::needs_compute_blit(&src, &dst));

        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let cb = d.dev.newCommandBuffer().unwrap();
        cb.beginCommandBufferWithAllocator(&alloc.a);
        let enc = cb.computeCommandEncoder().unwrap();
        let keep = super::super::present_thread::blit(&enc, &src, &dst);
        enc.endEncoding();
        cb.endCommandBuffer();

        q.submit(&[&cb]);
        if !keep.is_empty() {
            q.defer_release(Box::new(keep));
        }
        q.wait_idle();

        let mut out = [0u8; 4 * 4 * 4];
        unsafe {
            dst.getBytes_bytesPerRow_fromRegion_mipmapLevel(
                std::ptr::NonNull::new(out.as_mut_ptr() as *mut std::ffi::c_void).unwrap(),
                16,
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                0,
            );
        }
        for px in 0..16 {
            let b = out[px * 4];
            let g = out[px * 4 + 1];
            let r = out[px * 4 + 2];
            let a = out[px * 4 + 3];
            assert_eq!(r, 0xFF, "pixel {px} R");
            assert!((g as i32 - 0x80).abs() <= 1, "pixel {px} G={g:#x}");
            assert!((b as i32 - 0x40).abs() <= 1, "pixel {px} B={b:#x}");
            assert_eq!(a, 0xFF, "pixel {px} A");
        }

        d.remove_resident(ProtocolObject::from_ref(&*src));
        d.remove_resident(ProtocolObject::from_ref(&*dst));
    }

    #[test]
    fn present_blit_scales_and_centers_mismatched_sizes_on_gpu() {
        let Some(d) = super::super::device() else { return };
        let q = Arc::new(Queue::new(ListType::Direct, "present scale test").unwrap());

        // 4x2 RGB10A2Unorm back buffer -> 8x8 BGRA8Unorm drawable.
        // Aspect-preserving centered fit maps 4x2 into the centered 8x4 band (y in 2..6),
        // with opaque black letterbox bars at y in 0..2 and y in 6..8.
        let src_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::RGB10A2Unorm,
                4,
                2,
                false,
            )
        };
        src_td.setStorageMode(MTLStorageMode::Shared);
        src_td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView);
        let src = d.dev.newTextureWithDescriptor(&src_td).unwrap();

        let dst_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::BGRA8Unorm,
                8,
                8,
                false,
            )
        };
        dst_td.setStorageMode(MTLStorageMode::Shared);
        dst_td.setUsage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        let dst = d.dev.newTextureWithDescriptor(&dst_td).unwrap();

        // Fill source with solid orange-ish color: R=1.0, G=0.5, B=0.25, A=1.0.
        let packed: u32 = 1023u32 | (512u32 << 10) | (256u32 << 20) | (3u32 << 30);
        let src_pixels: Vec<u8> = (0..8).flat_map(|_| packed.to_le_bytes()).collect();
        unsafe {
            src.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 2, depth: 1 } },
                0,
                std::ptr::NonNull::new(src_pixels.as_ptr() as *mut std::ffi::c_void).unwrap(),
                16,
            );
        }
        // Pre-fill dst with non-zero sentinel bytes to verify letterbox margins are cleared to black.
        let sentinel = vec![0x77u8; 8 * 8 * 4];
        unsafe {
            dst.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 8, height: 8, depth: 1 } },
                0,
                std::ptr::NonNull::new(sentinel.as_ptr() as *mut std::ffi::c_void).unwrap(),
                32,
            );
        }

        d.add_resident(ProtocolObject::from_ref(&*src));
        d.add_resident(ProtocolObject::from_ref(&*dst));
        d.commit_residency();

        assert!(super::super::present_thread::needs_compute_blit(&src, &dst));

        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let cb = d.dev.newCommandBuffer().unwrap();
        cb.beginCommandBufferWithAllocator(&alloc.a);
        let enc = cb.computeCommandEncoder().unwrap();
        let keep = super::super::present_thread::blit(&enc, &src, &dst);
        enc.endEncoding();
        cb.endCommandBuffer();

        q.submit(&[&cb]);
        if !keep.is_empty() {
            q.defer_release(Box::new(keep));
        }
        q.wait_idle();

        let mut out = [0u8; 8 * 8 * 4];
        unsafe {
            dst.getBytes_bytesPerRow_fromRegion_mipmapLevel(
                std::ptr::NonNull::new(out.as_mut_ptr() as *mut std::ffi::c_void).unwrap(),
                32,
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 8, height: 8, depth: 1 } },
                0,
            );
        }
        for y in 0..8usize {
            for x in 0..8usize {
                let off = (y * 8 + x) * 4;
                let (b, g, r, a) = (out[off], out[off + 1], out[off + 2], out[off + 3]);
                if (2..6).contains(&y) {
                    assert_eq!(r, 0xFF, "center ({x},{y}) R");
                    assert!((g as i32 - 0x80).abs() <= 1, "center ({x},{y}) G={g:#x}");
                    assert!((b as i32 - 0x40).abs() <= 1, "center ({x},{y}) B={b:#x}");
                    assert_eq!(a, 0xFF, "center ({x},{y}) A");
                } else {
                    assert_eq!((r, g, b, a), (0, 0, 0, 0xFF), "letterbox ({x},{y}) must be opaque black");
                }
            }
        }

        d.remove_resident(ProtocolObject::from_ref(&*src));
        d.remove_resident(ProtocolObject::from_ref(&*dst));
    }

    #[test]
    fn dxgi_color_space_mapping_and_cametallayer_application() {
        assert_eq!(default_color_space_for_format(24), 0); // R10G10B10A2_UNORM -> sRGB
        assert_eq!(default_color_space_for_format(28), 0); // R8G8B8A8_UNORM -> sRGB
        assert_eq!(default_color_space_for_format(10), 1); // R16G16B16A16_FLOAT -> scRGB linear

        assert!(color_space_supported(0));
        assert!(color_space_supported(1));
        assert!(color_space_supported(10));
        assert!(color_space_supported(12));
        assert!(color_space_supported(17));
        assert!(!color_space_supported(999));

        assert!(!dxgi_color_space_spec(0).unwrap().edr);
        assert!(dxgi_color_space_spec(1).unwrap().edr);
        assert!(dxgi_color_space_spec(12).unwrap().edr);

        // Apply real CoreGraphics color spaces onto a live CAMetalLayer and verify
        // colorspace becomes non-nil and wantsExtendedDynamicRangeContent toggles cleanly.
        let layer = CAMetalLayer::new();
        let sdr = unsafe { apply_layer_colorspace(&layer, 0) }.unwrap();
        assert_eq!(sdr.name, "kCGColorSpaceSRGB");
        let cs_ptr: *const CgColorSpace = unsafe { objc2::msg_send![&*layer, colorspace] };
        assert!(!cs_ptr.is_null(), "CAMetalLayer.colorspace must be non-nil for SDR sRGB");
        let edr: bool = unsafe { objc2::msg_send![&*layer, wantsExtendedDynamicRangeContent] };
        assert!(!edr);

        let hdr = unsafe { apply_layer_colorspace(&layer, 12) }.unwrap();
        assert_eq!(hdr.name, "kCGColorSpaceITUR_2100_PQ");
        let cs_ptr2: *const CgColorSpace = unsafe { objc2::msg_send![&*layer, colorspace] };
        assert!(!cs_ptr2.is_null());
        let edr2: bool = unsafe { objc2::msg_send![&*layer, wantsExtendedDynamicRangeContent] };
        assert!(edr2, "HDR10 PQ must enable wantsExtendedDynamicRangeContent");

        let scrgb = unsafe { apply_layer_colorspace(&layer, 1) }.unwrap();
        assert_eq!(scrgb.name, "kCGColorSpaceExtendedLinearSRGB");
        let edr3: bool = unsafe { objc2::msg_send![&*layer, wantsExtendedDynamicRangeContent] };
        assert!(edr3, "linear scRGB must enable wantsExtendedDynamicRangeContent");

        assert!(unsafe { apply_layer_colorspace(&layer, 999) }.is_err());
    }

    #[test]
    fn present_blit_preserves_srgb_bytes_on_srgb_to_unorm_gpu() {
        let Some(d) = super::super::device() else { return };
        let q = Arc::new(Queue::new(ListType::Direct, "present srgb->unorm test").unwrap());

        let src_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::RGBA8Unorm_sRGB,
                4,
                4,
                false,
            )
        };
        src_td.setStorageMode(MTLStorageMode::Shared);
        src_td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView);
        let src = d.dev.newTextureWithDescriptor(&src_td).unwrap();

        let dst_td = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::BGRA8Unorm,
                4,
                4,
                false,
            )
        };
        dst_td.setStorageMode(MTLStorageMode::Shared);
        dst_td.setUsage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        let dst = d.dev.newTextureWithDescriptor(&dst_td).unwrap();

        // Mid-grey sRGB byte 0x80 (~0.502 encoded; ~0.216 if erroneously linearized!).
        let px: [u8; 4] = [0x80, 0x40, 0xC0, 0xFF];
        let src_bytes: Vec<u8> = (0..16).flat_map(|_| px).collect();
        unsafe {
            src.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                0,
                std::ptr::NonNull::new(src_bytes.as_ptr() as *mut std::ffi::c_void).unwrap(),
                16,
            );
        }

        d.add_resident(ProtocolObject::from_ref(&*src));
        d.add_resident(ProtocolObject::from_ref(&*dst));
        d.commit_residency();

        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let cb = d.dev.newCommandBuffer().unwrap();
        cb.beginCommandBufferWithAllocator(&alloc.a);
        let enc = cb.computeCommandEncoder().unwrap();
        let keep = super::super::present_thread::blit(&enc, &src, &dst);
        enc.endEncoding();
        cb.endCommandBuffer();
        q.submit(&[&cb]);
        if !keep.is_empty() {
            q.defer_release(Box::new(keep));
        }
        q.wait_idle();

        let mut out = [0u8; 16 * 4];
        unsafe {
            dst.getBytes_bytesPerRow_fromRegion_mipmapLevel(
                std::ptr::NonNull::new(out.as_mut_ptr() as *mut std::ffi::c_void).unwrap(),
                16,
                MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: 4, height: 4, depth: 1 } },
                0,
            );
        }
        // BGRA byte order must preserve raw sRGB-encoded values [B=0xC0, G=0x40, R=0x80, A=0xFF]
        // without double-gamma darkening (which would crush 0x80 down to ~0x37).
        for p in out.chunks_exact(4) {
            assert_eq!(p, &[0xC0, 0x40, 0x80, 0xFF]);
        }

        d.remove_resident(ProtocolObject::from_ref(&*src));
        d.remove_resident(ProtocolObject::from_ref(&*dst));
    }
}
