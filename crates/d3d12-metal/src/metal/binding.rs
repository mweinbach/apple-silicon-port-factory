//! Root arguments → Metal Shader Converter top-level argument buffers.
//!
//! `SetCompute/GraphicsRoot*` write into a CPU image of the root
//! signature's top-level argument buffer ([`RootState`]). Before each
//! dispatch/draw that follows a change, the image is copied into the command
//! allocator's [`UploadRing`] and its GPU address is bound at
//! `kIRArgumentBufferBindPoint` in the list's `MTL4ArgumentTable` (tables
//! are snapshotted per draw/dispatch, so one table per list is enough).

use super::resource::MtlBuffer;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::root_signature::{RootParameterLayoutKind, StaticSampler, TopLevelArgumentBuffer};
use dxil_metal::runtime::DescriptorTableEntry;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Upload ring (per command allocator)
// ---------------------------------------------------------------------------

const CHUNK: usize = 1 << 20;

struct RingChunk {
    buf: Retained<MtlBuffer>,
    gpu_va: u64,
    cpu_ptr: *mut u8,
    cap: usize,
    private_storage: bool,
}
unsafe impl Send for RingChunk {}

impl Drop for RingChunk {
    fn drop(&mut self) {
        super::device::note_ring_free(self.cap as u64);
        if let Some(dev) = super::device() {
            dev.remove_resident(ProtocolObject::from_ref(&*self.buf));
        }
    }
}

/// Process-wide pool of resident `UploadRing` chunks (shared upload + private GPU scratch),
/// eliminating per-frame `newBuffer` + `MTLResidencySet::commit()` churn across allocator resets.
static RING_POOL: Mutex<Vec<RingChunk>> = Mutex::new(Vec::new());

/// Compute the per-storage-mode byte cap for [`RING_POOL`]: generous on macOS,
/// right-sized on iOS so steady-state upload & ICB scratch chunks never thrash residency
/// (`res_commit == 0`) without inflating `phys_footprint` into the XNU VM compressor band.
fn ring_pool_budget_bytes() -> u64 {
    if !cfg!(target_os = "ios") {
        return 128 << 20;
    }
    if let Some(avail) = super::device::available_headroom_bytes() {
        let total = super::device::process_phys_footprint().saturating_add(avail);
        if total > (6656 << 20) {
            return 24 << 20;
        }
    }
    16 << 20
}

/// Return chunks to [`RING_POOL`], keeping the most useful chunks within `budget_bytes` per
/// storage mode (`shared` vs `private`) and deferring release of any surplus.
fn return_ring_chunks_to_pool(chunks: &mut Vec<RingChunk>) {
    if chunks.is_empty() {
        return;
    }
    let cap_bytes = ring_pool_budget_bytes();
    let mut dropped = Vec::new();
    if let Ok(mut p) = RING_POOL.lock() {
        p.append(chunks);
        for is_priv in [false, true] {
            let mut idxs: Vec<usize> = p
                .iter()
                .enumerate()
                .filter_map(|(i, c)| (c.private_storage == is_priv).then_some(i))
                .collect();
            // Keep larger chunks first (satisfy wider allocations) up to cap_bytes.
            idxs.sort_by(|&a, &b| p[b].cap.cmp(&p[a].cap));
            let mut kept_bytes = 0u64;
            let mut evict = Vec::new();
            for idx in idxs {
                let b = p[idx].cap as u64;
                if kept_bytes == 0 || kept_bytes.saturating_add(b) <= cap_bytes {
                    kept_bytes = kept_bytes.saturating_add(b);
                } else {
                    evict.push(idx);
                }
            }
            evict.sort_unstable_by(|a, b| b.cmp(a));
            for idx in evict {
                dropped.push(p.swap_remove(idx));
            }
        }
    }
    if !dropped.is_empty() {
        super::queue::defer_release_all(dropped);
    }
}

/// Release all idle pooled `UploadRing` buffers back to Metal/XNU once active queues drain.
pub fn trim_pool() -> usize {
    let dropped = if let Ok(mut p) = RING_POOL.lock() {
        std::mem::take(&mut *p)
    } else {
        Vec::new()
    };
    let n = dropped.len();
    if !dropped.is_empty() {
        super::queue::defer_release_all(dropped);
    }
    n
}

/// Trim idle pooled `UploadRing` buffers down to `max_bytes_per_mode` per storage mode.
pub fn trim_pool_keep(max_bytes_per_mode: u64) -> usize {
    let mut dropped = Vec::new();
    if let Ok(mut p) = RING_POOL.lock() {
        for is_priv in [false, true] {
            let mut idxs: Vec<usize> = p
                .iter()
                .enumerate()
                .filter_map(|(i, c)| (c.private_storage == is_priv).then_some(i))
                .collect();
            idxs.sort_by(|&a, &b| p[b].cap.cmp(&p[a].cap));
            let mut kept_bytes = 0u64;
            let mut evict = Vec::new();
            for idx in idxs {
                let b = p[idx].cap as u64;
                if kept_bytes.saturating_add(b) <= max_bytes_per_mode {
                    kept_bytes = kept_bytes.saturating_add(b);
                } else {
                    evict.push(idx);
                }
            }
            evict.sort_unstable_by(|a, b| b.cmp(a));
            for idx in evict {
                dropped.push(p.swap_remove(idx));
            }
        }
    }
    let n = dropped.len();
    if !dropped.is_empty() {
        super::queue::defer_release_all(dropped);
    }
    n
}

/// Linear allocator of resident GPU memory (shared CPU-upload or private GPU-only scratch).
/// Reset with its command allocator (D3D12 guarantees the GPU is done with it then).
pub struct UploadRing {
    chunks: Vec<RingChunk>,
    cur: usize,
    off: usize,
    private_storage: bool,
}
unsafe impl Send for UploadRing {}

impl Default for UploadRing {
    fn default() -> Self {
        UploadRing { chunks: Vec::new(), cur: 0, off: 0, private_storage: false }
    }
}

impl UploadRing {
    /// Create a GPU-only (`StorageModePrivate`) scratch ring for GPU-written/GPU-read
    /// transient buffers (e.g. `ExecuteIndirect` ICB expansion scratch). Never call
    /// [`Self::upload`] or dereference the CPU pointer returned by [`Self::alloc`] on it.
    pub fn new_gpu_scratch() -> Self {
        UploadRing { chunks: Vec::new(), cur: 0, off: 0, private_storage: true }
    }

    /// Rewind the cursor to offset 0 without dropping chunks (safe between closed render
    /// passes for GPU-written/GPU-read scratch buffers protected by a GPU queue WAR barrier).
    pub fn rewind(&mut self) {
        self.cur = 0;
        self.off = 0;
    }

    /// Keep up to 2 chunks `<= 2 MiB` local to this allocator and return extra or
    /// oversized chunks to the shared global `RING_POOL` so other in-flight allocators
    /// reuse them with zero `newBuffer` / `MTLResidencySet::commit()` churn.
    #[cfg(any(target_os = "ios", test))]
    fn recycle_excess_chunks(&mut self) {
        const LOCAL_KEEP_MAX: usize = 2 << 20;
        if !self.chunks.is_empty() {
            let mut kept = Vec::with_capacity(2);
            let mut returned = Vec::new();
            for c in self.chunks.drain(..) {
                if c.cap <= LOCAL_KEEP_MAX && kept.len() < 2 {
                    kept.push(c);
                } else {
                    returned.push(c);
                }
            }
            self.chunks = kept;
            if !returned.is_empty() {
                return_ring_chunks_to_pool(&mut returned);
            }
        }
    }

    pub fn reset(&mut self) {
        self.cur = 0;
        self.off = 0;
        // On macOS, command allocators cycle every frame so retaining chunks locally
        // avoids both pool mutex traffic and residency-set churn (`res_commit == 0`).
        // On iOS (tight Jetsam headroom), cap per-allocator retention and recycle surplus
        // chunks through `RING_POOL` so steady-state frames still have zero residency churn.
        #[cfg(target_os = "ios")]
        self.recycle_excess_chunks();
    }

    /// Copy `bytes` in; returns their GPU address.
    #[inline]
    pub fn upload(&mut self, bytes: &[u8], align: usize) -> Result<u64, String> {
        debug_assert!(!self.private_storage, "UploadRing::upload called on StorageModePrivate ring");
        let (va, p) = self.alloc(bytes.len(), align)?;
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len()) };
        Ok(va)
    }

    /// Reserve `n` bytes (contents undefined, e.g. GPU-written scratch);
    /// returns `(GPU address, CPU pointer)` (`cpu_ptr` is null for `new_gpu_scratch()`).
    #[inline]
    pub fn alloc(&mut self, n: usize, align: usize) -> Result<(u64, *mut u8), String> {
        if let Some(c) = self.chunks.get(self.cur) {
            let at = self.off.next_multiple_of(align);
            if at + n <= c.cap {
                self.off = at + n;
                let ptr = if c.cpu_ptr.is_null() {
                    std::ptr::null_mut()
                } else {
                    unsafe { c.cpu_ptr.add(at) }
                };
                return Ok((c.gpu_va + at as u64, ptr));
            }
        }
        let (b, at) = self.alloc_buffer(n, align)?;
        let ptr = if self.private_storage {
            std::ptr::null_mut()
        } else {
            unsafe { (b.contents().as_ptr() as *mut u8).add(at) }
        };
        Ok((b.gpuAddress() + at as u64, ptr))
    }

    /// Reserve `n` bytes and return `(MTLBuffer, byte_offset)`.
    pub fn alloc_buffer(&mut self, n: usize, align: usize) -> Result<(Retained<MtlBuffer>, usize), String> {
        loop {
            if let Some(c) = self.chunks.get(self.cur) {
                let at = self.off.next_multiple_of(align);
                if at + n <= c.cap {
                    self.off = at + n;
                    return Ok((c.buf.clone(), at));
                }
                self.cur += 1;
                self.off = 0;
                continue;
            }
            let size = CHUNK.max(n.next_multiple_of(4096));
            // Reuse a warm resident chunk from RING_POOL (best-fit smallest chunk >= size
            // and <= size*4 so 1 MiB requests never steal giant multi-MiB chunks).
            let pooled = {
                let mut p = RING_POOL.lock().unwrap();
                let max_cap = size.saturating_mul(4);
                let mut best: Option<usize> = None;
                for (i, c) in p.iter().enumerate() {
                    if c.private_storage == self.private_storage && c.cap >= size && c.cap <= max_cap {
                        if best.is_none_or(|bi| c.cap < p[bi].cap) {
                            best = Some(i);
                        }
                    }
                }
                best.map(|i| p.swap_remove(i))
            };
            let chunk = if let Some(c) = pooled {
                c
            } else {
                let d = super::device().ok_or("no device")?;
                let (opts, label) = if self.private_storage {
                    (
                        MTLResourceOptions::StorageModePrivate | MTLResourceOptions::HazardTrackingModeUntracked,
                        "port gpu scratch ring",
                    )
                } else {
                    (
                        MTLResourceOptions::StorageModeShared
                            | MTLResourceOptions::CPUCacheModeWriteCombined
                            | MTLResourceOptions::HazardTrackingModeUntracked,
                        "port root argument ring",
                    )
                };
                let b = d.dev.newBufferWithLength_options(size, opts).ok_or_else(|| format!("upload ring: newBuffer({size})"))?;
                b.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
                d.add_resident(ProtocolObject::from_ref(&*b));
                let gpu_va = b.gpuAddress();
                let cpu_ptr = if self.private_storage {
                    std::ptr::null_mut()
                } else {
                    b.contents().as_ptr() as *mut u8
                };
                let cap = b.length();
                super::device::note_ring_alloc(cap as u64);
                RingChunk { buf: b, gpu_va, cpu_ptr, cap, private_storage: self.private_storage }
            };
            self.chunks.push(chunk);
            self.cur = self.chunks.len() - 1;
            self.off = 0;
        }
    }
}

impl Drop for UploadRing {
    fn drop(&mut self) {
        if !self.chunks.is_empty() {
            // CommandAllocator guarantees GPU completion before Drop; recycle into RING_POOL
            // so future allocators reuse resident chunks without residency churn.
            return_ring_chunks_to_pool(&mut self.chunks);
        }
    }
}

struct StaticIndexBuf {
    _buf: Retained<MtlBuffer>,
    base_va: u64,
}
unsafe impl Send for StaticIndexBuf {}
unsafe impl Sync for StaticIndexBuf {}

/// Pre-allocated resident GPU addresses for the 16-byte-aligned `u16` index-type
/// uniform (`NON_INDEXED_DRAW = 0`, `UInt16 = 1`, `UInt32 = 2`) bound at
/// `ARGUMENT_BUFFER_UNIFORMS_BIND_POINT` on every draw call.
#[inline]
pub fn static_index_type_va(ir: u16) -> Result<u64, String> {
    static BUF: OnceLock<Option<StaticIndexBuf>> = OnceLock::new();
    if (ir as usize) < 4 {
        let b = BUF
            .get_or_init(|| {
                let d = super::device()?;
                let opts = MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked;
                let buf = d.dev.newBufferWithLength_options(64, opts)?;
                buf.setLabel(Some(&objc2_foundation::NSString::from_str("port static index type uniforms")));
                let ptr = buf.contents().as_ptr() as *mut u8;
                unsafe {
                    std::ptr::write_bytes(ptr, 0, 64);
                    for i in 0..4u16 {
                        std::ptr::copy_nonoverlapping(&i.to_le_bytes() as *const u8, ptr.add(i as usize * 16), 2);
                    }
                }
                d.add_resident(ProtocolObject::from_ref(&*buf));
                let base_va = buf.gpuAddress();
                Some(StaticIndexBuf { _buf: buf, base_va })
            })
            .as_ref()
            .ok_or("no device for static index type buffer")?;
        return Ok(b.base_va + (ir as u64) * 16);
    }
    Err(format!("unexpected index type uniform {ir}"))
}

// ---------------------------------------------------------------------------
// Static samplers
// ---------------------------------------------------------------------------

fn static_sampler_desc(s: &StaticSampler) -> super::views::SamplerDesc {
    let border = match s.border_color {
        0 => [0.0; 4],
        1 | 3 => [0.0, 0.0, 0.0, 1.0],
        _ => [1.0; 4],
    };
    super::views::SamplerDesc {
        filter: s.filter,
        address: [s.address_u, s.address_v, s.address_w],
        mip_lod_bias: s.mip_lod_bias,
        max_anisotropy: s.max_anisotropy,
        comparison: s.comparison_func,
        border,
        min_lod: s.min_lod,
        max_lod: s.max_lod,
    }
}

struct SamplerTables(Mutex<HashMap<u128, Retained<MtlBuffer>>>);
unsafe impl Send for SamplerTables {}
unsafe impl Sync for SamplerTables {}

/// GPU address of the root signature's static sampler table (one
/// `IRDescriptorTableEntry` per sampler, declaration order), created once
/// per root signature and kept for the process lifetime.
pub fn static_sampler_table(rs: &RootSignature) -> Result<Option<u64>, String> {
    let samplers = &rs.desc().static_samplers;
    if samplers.is_empty() {
        return Ok(None);
    }
    static T: OnceLock<SamplerTables> = OnceLock::new();
    let t = T.get_or_init(|| SamplerTables(Mutex::new(HashMap::new())));
    let mut m = t.0.lock().unwrap();
    if let Some(b) = m.get(&rs.hash()) {
        return Ok(Some(b.gpuAddress()));
    }
    let mut bytes = Vec::with_capacity(samplers.len() * 24);
    for s in samplers {
        let st = super::views::sampler_state(&static_sampler_desc(s))?;
        let id: u64 = unsafe { std::mem::transmute::<MTLResourceID, u64>(st.gpuResourceID()) };
        bytes.extend_from_slice(&DescriptorTableEntry::sampler(id, s.mip_lod_bias).to_bytes());
    }
    let d = super::device().ok_or("no device")?;
    let b = d.dev.newBufferWithLength_options(bytes.len(), MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked).ok_or("newBuffer for static samplers")?;
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), b.contents().as_ptr() as *mut u8, bytes.len()) };
    b.setLabel(Some(&objc2_foundation::NSString::from_str("port static samplers")));
    d.add_resident(ProtocolObject::from_ref(&*b));
    let a = b.gpuAddress();
    m.insert(rs.hash(), b);
    Ok(Some(a))
}

// ---------------------------------------------------------------------------
// Root state
// ---------------------------------------------------------------------------

/// One pipeline's (compute or graphics) root signature + argument image.
#[derive(Default)]
pub struct RootState {
    pub rs: Option<Arc<RootSignature>>,
    args: Option<TopLevelArgumentBuffer>,
    dirty: bool,
    /// GPU address of the last uploaded image.
    pub bound: u64,
    /// Monotonic generation bumped whenever root signature, table handle, or root VA mutates.
    pub bind_gen: u64,
    /// `AW2_FAST_ENC_STATE=1`: skip dirtying `args` when a root parameter write
    /// produces identical TLAB bytes.
    pub(crate) dedup: bool,
}

impl RootState {
    /// `SetXxxRootSignature`: a different root signature resets all root
    /// arguments; setting the current one again keeps them.
    pub fn set_root_signature(&mut self, rs: Arc<RootSignature>) -> Result<(), String> {
        if self.rs.as_ref().is_some_and(|c| Arc::ptr_eq(c, &rs)) {
            return Ok(());
        }
        let mut args = rs.layout().new_argument_buffer();
        if let Some(t) = static_sampler_table(&rs)? {
            args.set_static_sampler_table(rs.layout(), t);
        }
        self.args = Some(args);
        self.rs = Some(rs);
        self.dirty = true;
        self.bind_gen = self.bind_gen.wrapping_add(1);
        Ok(())
    }

    pub fn set_table(&mut self, i: u32, gpu_handle: u64) -> Result<(), String> {
        let dedup = self.dedup;
        let (Some(rs), Some(args)) = (self.rs.as_deref(), self.args.as_mut()) else {
            return Err("no root signature set".into());
        };
        let idx = i as usize;
        let Some(p) = rs.layout().parameters.get(idx) else {
            return Err(format!("root parameter {i} out of range ({} parameters)", rs.layout().parameters.len()));
        };
        if !matches!(p.kind, RootParameterLayoutKind::DescriptorTable { .. }) {
            return Err(format!("root parameter {idx} is not a descriptor table"));
        }
        if dedup {
            let off = p.offset as usize;
            if args.bytes.get(off..off + 8) == Some(&gpu_handle.to_le_bytes()) {
                super::encstats::inc(&super::encstats::c::enc_state_skip);
                return Ok(());
            }
        }
        args.set_descriptor_table(rs.layout(), idx, gpu_handle);
        self.dirty = true;
        self.bind_gen = self.bind_gen.wrapping_add(1);
        Ok(())
    }

    pub fn set_descriptor(&mut self, i: u32, va: u64) -> Result<(), String> {
        let dedup = self.dedup;
        let (Some(rs), Some(args)) = (self.rs.as_deref(), self.args.as_mut()) else {
            return Err("no root signature set".into());
        };
        let idx = i as usize;
        let Some(p) = rs.layout().parameters.get(idx) else {
            return Err(format!("root parameter {i} out of range ({} parameters)", rs.layout().parameters.len()));
        };
        if !matches!(p.kind, RootParameterLayoutKind::Descriptor { .. }) {
            return Err(format!("root parameter {idx} is not a root descriptor"));
        }
        if dedup {
            let off = p.offset as usize;
            if args.bytes.get(off..off + 8) == Some(&va.to_le_bytes()) {
                super::encstats::inc(&super::encstats::c::enc_state_skip);
                return Ok(());
            }
        }
        args.set_root_descriptor(rs.layout(), idx, va);
        self.dirty = true;
        self.bind_gen = self.bind_gen.wrapping_add(1);
        Ok(())
    }

    pub fn set_constants(&mut self, i: u32, dest: u32, values: &[u32]) -> Result<(), String> {
        let dedup = self.dedup;
        let (Some(rs), Some(args)) = (self.rs.as_deref(), self.args.as_mut()) else {
            return Err("no root signature set".into());
        };
        let idx = i as usize;
        let Some(p) = rs.layout().parameters.get(idx) else {
            return Err(format!("root parameter {i} out of range ({} parameters)", rs.layout().parameters.len()));
        };
        let RootParameterLayoutKind::Constants { num_32bit_values, .. } = p.kind else {
            return Err(format!("root parameter {idx} is not root constants"));
        };
        if dest as usize + values.len() > num_32bit_values as usize {
            return Err(format!("root constants {dest}+{} out of range ({num_32bit_values})", values.len()));
        }
        if values.is_empty() {
            return Ok(());
        }
        if dedup {
            let base = p.offset as usize + (dest as usize) * 4;
            let unchanged = values.iter().enumerate().all(|(k, v)| {
                let o = base + k * 4;
                args.bytes.get(o..o + 4) == Some(&v.to_le_bytes())
            });
            if unchanged {
                super::encstats::inc(&super::encstats::c::enc_state_skip);
                return Ok(());
            }
        }
        args.set_constants(rs.layout(), idx, dest, values);
        self.dirty = true;
        Ok(())
    }

    /// Read the raw 64-bit GPU address / descriptor-table GPU handle stored for
    /// root parameter `i` (returns `None` when no root signature is set or `i`
    /// is out of range / not an 8-byte parameter).
    #[inline]
    pub fn param_u64(&self, i: usize) -> Option<u64> {
        let (rs, args) = (self.rs.as_deref()?, self.args.as_ref()?);
        let p = rs.layout().parameters.get(i)?;
        if p.size != 8 {
            return None;
        }
        let off = p.offset as usize;
        let b: [u8; 8] = args.bytes.get(off..off + 8)?.try_into().ok()?;
        Some(u64::from_le_bytes(b))
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Upload the image if it changed; returns its GPU address.
    #[inline]
    pub fn flush(&mut self, ring: &mut UploadRing) -> Result<u64, String> {
        if self.dirty {
            let args = self.args.as_ref().ok_or("no root signature set")?;
            self.bound = ring.upload(&args.bytes, 256)?;
            self.dirty = false;
        }
        Ok(self.bound)
    }

    /// The image must be re-bound (e.g. a new list or allocator).
    pub fn invalidate(&mut self) {
        if self.args.is_some() {
            self.dirty = true;
        }
    }
}

pub type MtlArgumentTable = ProtocolObject<dyn MTL4ArgumentTable>;

/// An argument table covering every IR bind point we use.
pub fn new_argument_table(label: &str) -> Option<Retained<MtlArgumentTable>> {
    let d = super::device()?;
    let desc = MTL4ArgumentTableDescriptor::new();
    desc.setMaxBufferBindCount(31);
    desc.setInitializeBindings(true);
    desc.setSupportAttributeStrides(true);
    desc.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
    d.dev.newArgumentTableWithDescriptor_error(&desc).map_err(|e| log::error!(target: "d3d", "newArgumentTable: {e}")).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_ring_pools_shared_and_private_chunks_and_trims() {
        if crate::metal::device().is_none() {
            return;
        }
        trim_pool();

        // Shared CPU-writable ring: keeps small chunks local, recycles >4 MiB chunks into pool.
        let mut shared = UploadRing::default();
        let va0 = shared.upload(&[0xABu8; 256], 256).expect("shared upload 0");
        assert_ne!(va0, 0);
        // Trigger an oversized chunk (> 4 MiB).
        let big = vec![0xCDu8; (4 << 20) + 4096];
        let va1 = shared.upload(&big, 256).expect("shared upload big");
        assert_ne!(va1, 0);
        assert_eq!(shared.chunks.len(), 2);
        shared.recycle_excess_chunks();
        assert_eq!(shared.chunks.len(), 1);
        assert!(RING_POOL.lock().unwrap().iter().any(|c| !c.private_storage && c.cap >= (4 << 20) + 4096));

        // Private GPU-only scratch ring: cpu_ptr is null, chunks pool separately from shared.
        let mut scratch = UploadRing::new_gpu_scratch();
        let (sva0, sptr0) = scratch.alloc(64, 16).expect("scratch alloc 0");
        assert_ne!(sva0, 0);
        assert!(sptr0.is_null(), "private GPU scratch must not expose a CPU pointer");
        let (sva1, sptr1) = scratch.alloc((4 << 20) + 4096, 256).expect("scratch alloc big");
        assert_ne!(sva1, 0);
        assert!(sptr1.is_null());
        scratch.recycle_excess_chunks();
        assert_eq!(scratch.chunks.len(), 1);
        assert!(RING_POOL.lock().unwrap().iter().any(|c| c.private_storage && c.cap >= (4 << 20) + 4096));

        // Re-allocating the same private size reuses the pooled chunk (zero new buffer allocation).
        let pooled_va = RING_POOL
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.private_storage && c.cap >= (4 << 20) + 4096)
            .map(|c| c.gpu_va)
            .unwrap();
        let (sva2, _) = scratch.alloc((4 << 20) + 4096, 256).expect("scratch reuse big");
        assert_eq!(sva2, pooled_va, "second scratch alloc should reuse pooled private chunk");

        drop(shared);
        drop(scratch);
        assert!(!RING_POOL.lock().unwrap().is_empty());
        trim_pool();
        assert!(RING_POOL.lock().unwrap().is_empty());
    }
}
