//! Drawable acquisition off the app's render thread (sync-interval-0
//! presents of on-screen swapchains).
//!
//! `nextDrawable` blocks while the system still holds the layer's drawables,
//! even with display sync off and every present already on screen (after
//! macOS 27.2: ~2 ms per frame at 117 fps on a 60 Hz display). That time came
//! out of the game's render thread and starved the GPU. Instead, `Present`
//! copies the back buffer into a staging texture on the app's queue and hands
//! the slot to this thread, which waits for a drawable, blits the staging
//! texture into it on the present queue and presents it.
//!
//! * A newer frame replaces one the thread has not started (mailbox); with no
//!   free staging texture the present is dropped.
//! * The app's queue never waits on the present queue: back buffers are only
//!   read by the copy on the app's own queue.

use super::cmdlist::CommandAllocator;
use super::queue::{ListType, Queue};
use super::swapchain::LayerRef;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use objc2_quartz_core::CAMetalDrawable;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};

pub type Tex = Retained<ProtocolObject<dyn MTLTexture>>;

/// Staging textures: one being written, one in the mailbox, one presenting.
pub const SLOTS: usize = 3;
/// Present command buffers in flight.
const RING: usize = 4;
/// Slot state while in the mailbox or being encoded (otherwise the present
/// queue value of the slot's last blit).
const QUEUED: u64 = u64::MAX;

struct Job {
    slot: usize,
    /// Producer queue timeline value after the staging copy (`main` or `sq`).
    src_value: u64,
    from_sq: bool,
}

pub struct SendTex(pub Tex);
unsafe impl Send for SendTex {}
unsafe impl Sync for SendTex {}

struct StagingSlot {
    tex: SendTex,
    /// Pre-created linear UNORM view for `*_sRGB` staging textures so compute
    /// present blits never allocate an ObjC texture view + `defer_release` per frame.
    linear_view: Option<SendTex>,
}

struct Mailbox {
    job: Option<Job>,
    /// The thread is presenting a job it took from the mailbox.
    busy: bool,
}

struct Shared {
    layer: LayerRef,
    pq: Arc<Queue>,
    main: Arc<Queue>,
    sq: Option<Arc<Queue>>,
    staging: Mutex<Vec<StagingSlot>>,
    slot_pv: [AtomicU64; SLOTS],
    mailbox: Mutex<Mailbox>,
    cv: Condvar,
    drain_cv: Condvar,
    quit: AtomicBool,
    opt_present_blit_narrow: AtomicBool,
}

pub struct PresentThread {
    sh: Arc<Shared>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl PresentThread {
    pub fn new(layer: &LayerRef, pq: Arc<Queue>, main: Arc<Queue>, sq: Option<Arc<Queue>>) -> Result<PresentThread, String> {
        let sh = Arc::new(Shared {
            layer: LayerRef(layer.0.clone()),
            pq,
            main,
            sq,
            staging: Mutex::new(Vec::new()),
            slot_pv: std::array::from_fn(|_| AtomicU64::new(0)),
            mailbox: Mutex::new(Mailbox { job: None, busy: false }),
            cv: Condvar::new(),
            drain_cv: Condvar::new(),
            quit: AtomicBool::new(false),
            opt_present_blit_narrow: AtomicBool::new(super::swapchain::present_blit_narrow()),
        });
        let s = sh.clone();
        let handle = std::thread::Builder::new().name("port-present".into()).spawn(move || run(s)).map_err(|e| format!("present thread: {e}"))?;
        Ok(PresentThread { sh, handle: Some(handle) })
    }

    #[cfg(test)]
    pub(crate) fn set_present_blit_narrow(&self, on: bool) {
        self.sh.opt_present_blit_narrow.store(on, Ordering::Release);
    }

    /// Replace the staging textures (`w`x`h`, the back buffers' format). The
    /// thread must be drained.
    pub fn set_staging(&self, w: u32, h: u32, fmt: MTLPixelFormat) -> Result<(), String> {
        let d = super::device().ok_or("no device")?;
        let linear_fmt = match fmt {
            MTLPixelFormat::RGBA8Unorm_sRGB => Some(MTLPixelFormat::RGBA8Unorm),
            MTLPixelFormat::BGRA8Unorm_sRGB => Some(MTLPixelFormat::BGRA8Unorm),
            _ => None,
        };
        let mut st = self.sh.staging.lock().unwrap();
        for s in st.drain(..) {
            d.remove_resident(ProtocolObject::from_ref(&*s.tex.0));
        }
        for _ in 0..SLOTS {
            let td = unsafe { MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(fmt, w as usize, h as usize, false) };
            td.setUsage(MTLTextureUsage::ShaderRead | MTLTextureUsage::PixelFormatView);
            td.setStorageMode(MTLStorageMode::Private);
            let t = d.dev.newTextureWithDescriptor(&td).ok_or("staging texture")?;
            t.setLabel(Some(&objc2_foundation::NSString::from_str("present staging")));
            d.add_resident(ProtocolObject::from_ref(&*t));
            let linear_view = linear_fmt.and_then(|lf| t.newTextureViewWithPixelFormat(lf)).map(SendTex);
            st.push(StagingSlot { tex: SendTex(t), linear_view });
        }
        d.commit_residency();
        Ok(())
    }

    pub fn staging(&self, slot: usize) -> Option<Tex> {
        self.sh.staging.lock().unwrap().get(slot).map(|s| s.tex.0.clone())
    }

    /// A staging slot whose last blit has completed.
    pub fn free_slot(&self) -> Option<usize> {
        let done = self.sh.pq.timeline.completed();
        (0..SLOTS).find(|&s| {
            let v = self.sh.slot_pv[s].load(Ordering::Acquire);
            v != QUEUED && v <= done
        })
    }

    /// Mark `slot` taken before encoding the copy into it.
    pub fn claim(&self, slot: usize) {
        self.sh.slot_pv[slot].store(QUEUED, Ordering::Release);
    }

    /// Present `slot` once its producer queue (`main` or `sq`) reaches
    /// `src_value`. Replaces a frame the thread has not started.
    pub fn submit(&self, slot: usize, src_value: u64, from_sq: bool) {
        let mut m = self.sh.mailbox.lock().unwrap();
        if let Some(old) = m.job.replace(Job { slot, src_value, from_sq }) {
            // Never encoded: its slot's previous blit had completed.
            self.sh.slot_pv[old.slot].store(0, Ordering::Release);
            super::encstats::inc(&super::encstats::c::present_mbox);
        }
        self.sh.cv.notify_one();
    }

    /// Wait until no job is queued or presenting and both the staging queue
    /// (if any) and present queue are idle.
    pub fn drain(&self) {
        {
            let mut m = self.sh.mailbox.lock().unwrap();
            while m.job.is_some() || m.busy {
                m = self.sh.drain_cv.wait(m).unwrap();
            }
        }
        if let Some(sq) = &self.sh.sq {
            sq.wait_idle();
        }
        self.sh.pq.wait_idle();
    }
}

impl Drop for PresentThread {
    fn drop(&mut self) {
        self.sh.quit.store(true, Ordering::Release);
        {
            let _m = self.sh.mailbox.lock().unwrap();
            self.sh.cv.notify_one();
        }
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        if let Some(sq) = &self.sh.sq {
            sq.wait_idle();
        }
        self.sh.pq.wait_idle();
        if let Some(d) = super::device() {
            for s in self.sh.staging.lock().unwrap().drain(..) {
                d.remove_resident(ProtocolObject::from_ref(&*s.tex.0));
            }
        }
    }
}

struct Ring {
    allocs: Vec<CommandAllocator>,
    cbs: Vec<Retained<super::queue::MtlCommandBuffer>>,
    values: [u64; RING],
    next: usize,
}

fn run(sh: Arc<Shared>) {
    #[cfg(target_vendor = "apple")]
    unsafe {
        // Keep drawable acquisition and present blit on P-cores on Apple Silicon / iOS.
        libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0);
    }
    let notify_exit = || {
        let mut m = sh.mailbox.lock().unwrap();
        if let Some(j) = m.job.take() {
            sh.slot_pv[j.slot].store(0, Ordering::Release);
        }
        m.busy = false;
        sh.drain_cv.notify_all();
    };
    let Some(d) = super::device() else {
        notify_exit();
        return;
    };
    let mut ring = Ring { allocs: Vec::new(), cbs: Vec::new(), values: [0; RING], next: 0 };
    for _ in 0..RING {
        let (Some(a), Some(cb)) = (CommandAllocator::new(ListType::Direct), d.dev.newCommandBuffer()) else {
            log::error!(target: "d3d", "present thread: no command allocator/buffer");
            notify_exit();
            return;
        };
        ring.allocs.push(a);
        ring.cbs.push(cb);
    }
    // Dedicated argument table for this thread so present blits never contend on a global mutex.
    let arg_table = {
        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxTextureBindCount(2);
        d.dev.newArgumentTableWithDescriptor_error(&atd).ok()
    };
    loop {
        let mut job = {
            let mut m = sh.mailbox.lock().unwrap();
            if m.busy {
                m.busy = false;
                if m.job.is_none() {
                    sh.drain_cv.notify_all();
                }
            }
            loop {
                if sh.quit.load(Ordering::Acquire) {
                    sh.drain_cv.notify_all();
                    return;
                }
                if let Some(j) = m.job.take() {
                    m.busy = true;
                    break j;
                }
                m = sh.cv.wait(m).unwrap();
            }
        };
        let r = objc2::rc::autoreleasepool(|_| present_one(&sh, &mut ring, &mut job, arg_table.as_deref()));
        if let Err(e) = r {
            log::warn!(target: "d3d", "present thread: {e}");
            sh.slot_pv[job.slot].store(0, Ordering::Release);
        }
    }
}

fn present_one(
    sh: &Shared,
    ring: &mut Ring,
    job: &mut Job,
    arg_table: Option<&ProtocolObject<dyn MTL4ArgumentTable>>,
) -> Result<(), String> {
    // Wait for the ring slot's prior present to complete BEFORE acquiring a drawable
    // so Core Animation drawables are never held idle across a CPU wait.
    let i = ring.next;
    sh.pq.timeline.wait_cpu(ring.values[i], u64::MAX);
    ring.allocs[i].reset();

    let t = super::encstats::active().then(std::time::Instant::now);
    let drawable = sh.layer.0.nextDrawable().ok_or("nextDrawable returned nil")?;
    if let Some(t) = t {
        super::encstats::add(&super::encstats::c::present_acq_us, t.elapsed().as_micros() as u64);
    }
    ring.next = (i + 1) % RING;

    // If a fresher frame arrived while blocked in `nextDrawable()`, present the
    // newest frame instead of a 1-frame-stale one.
    {
        let mut m = sh.mailbox.lock().unwrap();
        if let Some(newer) = m.job.take() {
            sh.slot_pv[job.slot].store(0, Ordering::Release);
            super::encstats::inc(&super::encstats::c::present_mbox);
            *job = newer;
        }
    }

    let (src, src_linear) = {
        let st = sh.staging.lock().unwrap();
        let slot = st.get(job.slot).ok_or("no staging texture")?;
        (slot.tex.0.clone(), slot.linear_view.as_ref().map(|v| v.0.clone()))
    };
    let cb = ring.cbs[i].clone();
    cb.beginCommandBufferWithAllocator(&ring.allocs[i].a);
    let e = cb.computeCommandEncoder().ok_or("compute encoder")?;
    let after_stages = if sh.opt_present_blit_narrow.load(Ordering::Relaxed) {
        super::encstats::inc(&super::encstats::c::present_blit_narrow);
        MTLStages::Blit
    } else {
        MTLStages::All
    };
    let dst_tex = drawable.texture();
    let before_stages = if needs_compute_blit(&src, &dst_tex) {
        MTLStages::Dispatch
    } else {
        MTLStages::Blit
    };
    e.barrierAfterQueueStages_beforeStages_visibilityOptions(after_stages, before_stages, MTL4VisibilityOptions::Device);
    let keep_views = blit_with(&e, &src, &dst_tex, src_linear.as_deref(), arg_table);
    e.endEncoding();
    cb.endCommandBuffer();
    let md: &ProtocolObject<dyn MTLDrawable> = ProtocolObject::from_ref(&*drawable);
    let src_tl = match (job.from_sq, sh.sq.as_deref()) {
        (true, Some(sq)) => &sq.timeline,
        _ => &sh.main.timeline,
    };
    sh.pq.wait(src_tl, job.src_value);
    sh.pq.q.waitForDrawable(md);
    let pv = sh.pq.submit(&[&cb]);
    if !keep_views.is_empty() {
        sh.pq.defer_release(Box::new(keep_views));
    }
    sh.pq.q.signalDrawable(md);
    md.present();
    ring.values[i] = pv;
    sh.slot_pv[job.slot].store(pv, Ordering::Release);
    super::encstats::inc(&super::encstats::c::present_pq);
    Ok(())
}

/// True when `src -> dst` differs in dimensions (aspect-preserving scale +
/// center fit), channel ordering (`RGBA8*` <-> `BGRA8*`), or bit layout (e.g.
/// `RGB10A2Unorm` -> `BGRA8Unorm` for iOS `CAMetalLayer`) and must run through
/// a compute present kernel instead of a raw `copyFromTexture` bit-copy.
pub fn needs_compute_blit(
    src: &ProtocolObject<dyn MTLTexture>,
    dst: &ProtocolObject<dyn MTLTexture>,
) -> bool {
    if src.width() != dst.width() || src.height() != dst.height() {
        return true;
    }
    let (sf, df) = (src.pixelFormat(), dst.pixelFormat());
    if sf == df {
        return false;
    }
    matches!(
        (sf, df),
        (MTLPixelFormat::RGBA8Unorm, MTLPixelFormat::BGRA8Unorm)
            | (MTLPixelFormat::RGBA8Unorm_sRGB, MTLPixelFormat::BGRA8Unorm_sRGB)
            | (MTLPixelFormat::BGRA8Unorm, MTLPixelFormat::RGBA8Unorm)
            | (MTLPixelFormat::BGRA8Unorm_sRGB, MTLPixelFormat::RGBA8Unorm_sRGB)
            | (MTLPixelFormat::RGBA8Unorm, MTLPixelFormat::BGRA8Unorm_sRGB)
            | (MTLPixelFormat::RGBA8Unorm_sRGB, MTLPixelFormat::BGRA8Unorm)
            | (MTLPixelFormat::RGB10A2Unorm, MTLPixelFormat::BGRA8Unorm)
            | (MTLPixelFormat::RGB10A2Unorm, MTLPixelFormat::BGRA8Unorm_sRGB)
            | (MTLPixelFormat::RGB10A2Unorm, MTLPixelFormat::RGBA16Float)
            | (MTLPixelFormat::RGBA16Float, MTLPixelFormat::BGRA8Unorm)
            | (MTLPixelFormat::RGBA16Float, MTLPixelFormat::BGRA8Unorm_sRGB)
    )
}

struct BlitArgTable {
    table: Retained<ProtocolObject<dyn MTL4ArgumentTable>>,
}
unsafe impl Send for BlitArgTable {}
unsafe impl Sync for BlitArgTable {}

fn blit_arg_table() -> Option<&'static Mutex<BlitArgTable>> {
    static T: std::sync::OnceLock<Option<Mutex<BlitArgTable>>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let d = super::device()?;
        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxTextureBindCount(2);
        let table = d.dev.newArgumentTableWithDescriptor_error(&atd).ok()?;
        Some(Mutex::new(BlitArgTable { table }))
    })
    .as_ref()
}

/// Copy `src` into the drawable texture `dst`.
///
/// * Exact size + matching format: hardware `copyFromTexture`.
/// * Exact size + different format (`RGBA8*` <-> `BGRA8*`, `RGB10A2` -> `BGRA8`,
///   etc.): 1:1 compute conversion (`port_present_copy_2d`).
/// * Mismatched dimensions (window resize or display/back-buffer resolution
///   mismatch): aspect-preserving centered bilinear scale with black margins
///   (`port_present_scale_2d`).
///
/// Returns any temporary texture views that must stay alive until the command
/// buffer completes on the GPU.
pub fn blit(
    e: &ProtocolObject<dyn MTL4ComputeCommandEncoder>,
    src: &ProtocolObject<dyn MTLTexture>,
    dst: &ProtocolObject<dyn MTLTexture>,
) -> Vec<SendTex> {
    blit_with(e, src, dst, None, None)
}

fn blit_with(
    e: &ProtocolObject<dyn MTL4ComputeCommandEncoder>,
    src: &ProtocolObject<dyn MTLTexture>,
    dst: &ProtocolObject<dyn MTLTexture>,
    cached_src_linear: Option<&ProtocolObject<dyn MTLTexture>>,
    dedicated_table: Option<&ProtocolObject<dyn MTL4ArgumentTable>>,
) -> Vec<SendTex> {
    let (sw, sh) = (src.width(), src.height());
    let (dw, dh) = (dst.width(), dst.height());
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 {
        return Vec::new();
    }
    let same_size = sw == dw && sh == dh;
    if same_size && src.pixelFormat() == dst.pixelFormat() {
        unsafe {
            e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                src,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize { width: dw, height: dh, depth: 1 },
                dst,
                0,
                0,
                MTLOrigin { x: 0, y: 0, z: 0 },
            );
        }
        return Vec::new();
    }
    if needs_compute_blit(src, dst) {
        // Always read `*_sRGB` back buffers through their raw linear UNORM twin
        // and write `*_sRGB` drawables through their raw linear UNORM twin so
        // already-sRGB-encoded bytes transfer losslessly without double-gamma
        // darkening (and `ShaderWrite` remains legal on sRGB drawables).
        let src_view = if cached_src_linear.is_some() {
            None
        } else {
            match src.pixelFormat() {
                MTLPixelFormat::RGBA8Unorm_sRGB => src.newTextureViewWithPixelFormat(MTLPixelFormat::RGBA8Unorm),
                MTLPixelFormat::BGRA8Unorm_sRGB => src.newTextureViewWithPixelFormat(MTLPixelFormat::BGRA8Unorm),
                _ => None,
            }
        };
        let dst_view = match dst.pixelFormat() {
            MTLPixelFormat::BGRA8Unorm_sRGB => dst.newTextureViewWithPixelFormat(MTLPixelFormat::BGRA8Unorm),
            MTLPixelFormat::RGBA8Unorm_sRGB => dst.newTextureViewWithPixelFormat(MTLPixelFormat::RGBA8Unorm),
            _ => None,
        };
        let s_tex: &ProtocolObject<dyn MTLTexture> = cached_src_linear.or(src_view.as_deref()).unwrap_or(src);
        let d_tex: &ProtocolObject<dyn MTLTexture> = dst_view.as_deref().unwrap_or(dst);
        let kernel_name = if same_size { "port_present_copy_2d" } else { "port_present_scale_2d" };
        if let Ok(pso) = super::clear::kernel(kernel_name) {
            let dispatch_with_table = |table: &ProtocolObject<dyn MTL4ArgumentTable>| unsafe {
                e.setComputePipelineState(pso);
                table.setTexture_atIndex(s_tex.gpuResourceID(), 0);
                table.setTexture_atIndex(d_tex.gpuResourceID(), 1);
                e.setArgumentTable(Some(table));
                e.dispatchThreads_threadsPerThreadgroup(
                    MTLSize { width: dw, height: dh, depth: 1 },
                    MTLSize { width: 8, height: 8, depth: 1 },
                );
            };
            let dispatched = if let Some(table) = dedicated_table {
                dispatch_with_table(table);
                true
            } else if let Some(tbl_mtx) = blit_arg_table() {
                let tbl = tbl_mtx.lock().unwrap();
                dispatch_with_table(&tbl.table);
                true
            } else {
                false
            };
            if dispatched {
                let mut keep = Vec::new();
                if let Some(v) = src_view {
                    keep.push(SendTex(v));
                }
                if let Some(v) = dst_view {
                    keep.push(SendTex(v));
                }
                return keep;
            }
        }
        log::warn!(target: "d3d", "present: compute blit ({kernel_name}) unavailable for {:?} -> {:?}", src.pixelFormat(), dst.pixelFormat());
        return Vec::new();
    }
    let cached_match = cached_src_linear.filter(|v| v.pixelFormat() == dst.pixelFormat());
    let view = if cached_match.is_some() {
        None
    } else {
        src.newTextureViewWithPixelFormat(dst.pixelFormat())
    };
    let blit_src = cached_match.or(view.as_deref()).unwrap_or(src);
    if blit_src.pixelFormat() != dst.pixelFormat() {
        log::warn!(target: "d3d", "present: back buffer {:?} vs drawable {:?}: format conversion not implemented", src.pixelFormat(), dst.pixelFormat());
        return Vec::new();
    }
    let w = sw.min(dw);
    let h = sh.min(dh);
    unsafe {
        e.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            blit_src,
            0,
            0,
            MTLOrigin { x: 0, y: 0, z: 0 },
            MTLSize { width: w, height: h, depth: 1 },
            dst,
            0,
            0,
            MTLOrigin { x: 0, y: 0, z: 0 },
        );
    }
    view.into_iter().map(SendTex).collect()
}
