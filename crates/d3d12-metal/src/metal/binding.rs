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

/// Linear allocator of shared, resident GPU memory. Reset with its command
/// allocator (D3D12 guarantees the GPU is done with it then).
pub struct UploadRing {
    chunks: Vec<RingChunk>,
    cur: usize,
    off: usize,
}
unsafe impl Send for UploadRing {}

impl Default for UploadRing {
    fn default() -> Self {
        UploadRing { chunks: Vec::new(), cur: 0, off: 0 }
    }
}

impl UploadRing {
    /// Rewind the cursor to offset 0 without dropping chunks (safe between closed render
    /// passes for GPU-written/GPU-read scratch buffers protected by a GPU queue WAR barrier).
    pub fn rewind(&mut self) {
        self.cur = 0;
        self.off = 0;
    }

    pub fn reset(&mut self) {
        self.cur = 0;
        self.off = 0;
        // On iOS, release any oversized transient spike chunks (> 1 MiB) and any extra
        // chunks beyond 2 standard 1 MiB chunks on allocator reset via deferred GPU release
        // so loading/indirect-draw spikes don't pin dozens of MiB across allocators.
        #[cfg(target_os = "ios")]
        if !self.chunks.is_empty() {
            let mut kept = Vec::with_capacity(2);
            let mut retired = Vec::new();
            for c in self.chunks.drain(..) {
                if c.cap <= CHUNK && kept.len() < 2 {
                    kept.push(c);
                } else {
                    retired.push(c);
                }
            }
            self.chunks = kept;
            if !retired.is_empty() {
                super::queue::defer_release_all(retired);
            }
        }
    }

    /// Copy `bytes` in; returns their GPU address.
    #[inline]
    pub fn upload(&mut self, bytes: &[u8], align: usize) -> Result<u64, String> {
        let (va, p) = self.alloc(bytes.len(), align)?;
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len()) };
        Ok(va)
    }

    /// Reserve `n` bytes (contents undefined, e.g. GPU-written scratch);
    /// returns (GPU address, CPU pointer).
    #[inline]
    pub fn alloc(&mut self, n: usize, align: usize) -> Result<(u64, *mut u8), String> {
        if let Some(c) = self.chunks.get(self.cur) {
            let at = self.off.next_multiple_of(align);
            if at + n <= c.cap {
                self.off = at + n;
                return Ok((c.gpu_va + at as u64, unsafe { c.cpu_ptr.add(at) }));
            }
        }
        let (b, at) = self.alloc_buffer(n, align)?;
        Ok((b.gpuAddress() + at as u64, unsafe { (b.contents().as_ptr() as *mut u8).add(at) }))
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
            let d = super::device().ok_or("no device")?;
            let size = CHUNK.max(n.next_multiple_of(4096));
            let opts = MTLResourceOptions::StorageModeShared | MTLResourceOptions::CPUCacheModeWriteCombined | MTLResourceOptions::HazardTrackingModeUntracked;
            let b = d.dev.newBufferWithLength_options(size, opts).ok_or_else(|| format!("upload ring: newBuffer({size})"))?;
            b.setLabel(Some(&objc2_foundation::NSString::from_str("port root argument ring")));
            d.add_resident(ProtocolObject::from_ref(&*b));
            let gpu_va = b.gpuAddress();
            let cpu_ptr = b.contents().as_ptr() as *mut u8;
            let cap = b.length();
            super::device::note_ring_alloc(cap as u64);
            self.chunks.push(RingChunk { buf: b, gpu_va, cpu_ptr, cap });
            self.cur = self.chunks.len() - 1;
            self.off = 0;
        }
    }
}

impl Drop for UploadRing {
    fn drop(&mut self) {
        if !self.chunks.is_empty() {
            let retired = std::mem::take(&mut self.chunks);
            super::queue::defer_release_all(retired);
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
