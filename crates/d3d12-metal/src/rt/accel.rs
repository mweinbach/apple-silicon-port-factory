//! DXR acceleration structures on Metal 4 (docs/design/DXR.md §3; review ADVICE_DXR.md M1–M5).
//!
//! The D3D result buffer at the AS's VA holds only the 64-byte
//! `IRRaytracingAccelerationStructureGPUHeader` (+ a TLAS's `u32` instance contributions). The
//! real `MTLAccelerationStructure` lives in a private allocation tracked here by VA. Converted
//! TLAS instance descriptors live at the start of the app's scratch buffer (we report the extra
//! space in the prebuild info), so a TLAS needs no other memory of ours.
//!
//! Encoding functions take a [`Ctx`]: the list's compute encoder, a utility argument table and
//! upload ring. They bind their own table and PSO, so the caller must restore its table and
//! mark its compute PSO unbound afterwards (like the ClearUAV utility kernels).

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSArray;
use objc2_metal::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use super::d3d::{self, build_flags, Geometry, Inputs, PostbuildInfoDesc, PrebuildInfo};
use super::kernels::{self, ConvertParams};
use crate::metal::binding::{MtlArgumentTable, UploadRing};

pub type MtlAccel = ProtocolObject<dyn MTLAccelerationStructure>;
type MtlBuffer = ProtocolObject<dyn MTLBuffer>;
type MtlComputeEncoder = ProtocolObject<dyn MTL4ComputeCommandEncoder>;

/// `IRRaytracingAccelerationStructureGPUHeader` size.
pub const HEADER_SIZE: u64 = 64;
/// `MTLIndirectAccelerationStructureInstanceDescriptor` size.
pub const METAL_INSTANCE_STRIDE: u64 = 72;
/// Compacted-size slots (8 bytes each) in one shared buffer.
const SLOTS: u32 = 8192;

/// The D3D result size we report: header + contributions, rounded to 256 bytes.
pub fn result_size(instance_count: u32) -> u64 {
    (HEADER_SIZE + 4 * instance_count as u64).next_multiple_of(d3d::AS_ALIGNMENT)
}

/// Bytes at the start of a TLAS build's scratch that hold the converted instances.
pub fn instance_area(instance_count: u32) -> u64 {
    (METAL_INSTANCE_STRIDE * instance_count as u64).next_multiple_of(256)
}

/// The header bytes as qwords (`IRRaytracingSetAccelerationStructure`): resource ID, address of
/// the contributions (right after the header), zero padding.
pub fn header_qwords(resource_id: u64, header_va: u64) -> [u64; 8] {
    [resource_id, header_va + HEADER_SIZE, 0, 0, 0, 0, 0, 0]
}

/// `MTLResourceID` as the u64 the header stores.
pub fn resource_id(a: &MtlAccel) -> u64 {
    // SAFETY: MTLResourceID is a transparent u64 (`_impl`).
    unsafe { std::mem::transmute::<MTLResourceID, u64>(a.gpuResourceID()) }
}

/// BLAS vertex `DXGI_FORMAT` → (Metal attribute format reading xyz or xy, element bytes).
pub fn vertex_format(dxgi: u32) -> Option<(MTLAttributeFormat, u64)> {
    use MTLAttributeFormat as F;
    Some(match dxgi {
        6 => (F::Float3, 12),
        16 => (F::Float2, 8),
        10 => (F::Half3, 8),
        34 => (F::Half2, 4),
        13 => (F::Short3Normalized, 8),
        37 => (F::Short2Normalized, 4),
        11 => (F::UShort3Normalized, 8),
        35 => (F::UShort2Normalized, 4),
        24 => (F::UInt1010102Normalized, 4),
        28 => (F::UChar3Normalized, 4),
        49 => (F::UChar2Normalized, 2),
        31 => (F::Char3Normalized, 4),
        51 => (F::Char2Normalized, 2),
        _ => return None,
    })
}

/// BLAS index `DXGI_FORMAT` → (Metal index type, bytes); `None` for non-indexed.
pub fn index_type(dxgi: u32) -> Result<Option<(MTLIndexType, u64)>, String> {
    match dxgi {
        0 => Ok(None),
        57 => Ok(Some((MTLIndexType::UInt16, 2))),
        42 => Ok(Some((MTLIndexType::UInt32, 4))),
        f => Err(format!("unsupported BLAS index format {f}")),
    }
}

/// D3D build flags → Metal usage. `Refit` only with `ALLOW_UPDATE` (it costs trace speed).
pub fn usage(flags: u32) -> MTLAccelerationStructureUsage {
    let mut u = MTLAccelerationStructureUsage::None;
    if flags & build_flags::ALLOW_UPDATE != 0 {
        u |= MTLAccelerationStructureUsage::Refit;
    }
    if flags & build_flags::PREFER_FAST_BUILD != 0 {
        u |= MTLAccelerationStructureUsage::PreferFastBuild;
    } else if flags & build_flags::PREFER_FAST_TRACE != 0 {
        u |= MTLAccelerationStructureUsage::PreferFastIntersection;
    }
    u
}

fn range(va: u64, len: u64) -> MTL4BufferRange {
    MTL4BufferRange { bufferAddress: va, length: if va == 0 { 0 } else { len } }
}

/// Triangle count of a triangles geometry.
fn triangle_count(t: &d3d::TrianglesDesc) -> u32 {
    if t.IndexFormat != 0 {
        t.IndexCount / 3
    } else {
        t.VertexCount / 3
    }
}

/// The Metal descriptor of `inputs`. `instances_va`: converted instances (TLAS builds).
/// Buffer ranges are left empty when `with_buffers` is false (size queries).
pub fn descriptor(inputs: &Inputs, instances_va: u64, with_buffers: bool) -> Result<Retained<MTL4AccelerationStructureDescriptor>, String> {
    match inputs {
        Inputs::Top { flags, count, .. } => {
            let t = MTL4InstanceAccelerationStructureDescriptor::new();
            unsafe {
                if with_buffers {
                    t.setInstanceDescriptorBuffer(range(instances_va, METAL_INSTANCE_STRIDE * *count as u64));
                }
                t.setInstanceDescriptorStride(METAL_INSTANCE_STRIDE as usize);
                t.setInstanceCount(*count as usize);
            }
            t.setInstanceDescriptorType(MTLAccelerationStructureInstanceDescriptorType::Indirect);
            t.setInstanceTransformationMatrixLayout(MTLMatrixLayout::RowMajor);
            t.setUsage(usage(*flags));
            Ok(Retained::into_super(t))
        }
        Inputs::Bottom { flags, geometries } => {
            let mut list: Vec<Retained<MTL4AccelerationStructureGeometryDescriptor>> = Vec::with_capacity(geometries.len());
            for g in geometries {
                let gd: Retained<MTL4AccelerationStructureGeometryDescriptor> = match g {
                    Geometry::Triangles { desc: t, .. } => {
                        let (fmt, elem) = vertex_format(t.VertexFormat).ok_or_else(|| format!("unsupported BLAS vertex format {}", t.VertexFormat))?;
                        let index = index_type(t.IndexFormat)?;
                        let gt = MTL4AccelerationStructureTriangleGeometryDescriptor::new();
                        gt.setVertexFormat(fmt);
                        unsafe {
                            gt.setVertexStride(t.VertexBufferStride as usize);
                            gt.setTriangleCount(triangle_count(t) as usize);
                        }
                        if let Some((ty, _)) = index {
                            gt.setIndexType(ty);
                        }
                        if with_buffers {
                            let vlen = if t.VertexCount == 0 { 0 } else { (t.VertexCount as u64 - 1) * t.VertexBufferStride + elem };
                            unsafe { gt.setVertexBuffer(range(t.VertexBufferStart, vlen)) };
                            if let Some((_, isz)) = index {
                                unsafe { gt.setIndexBuffer(range(t.IndexBuffer, t.IndexCount as u64 * isz)) };
                            }
                            if t.Transform3x4 != 0 {
                                gt.setTransformationMatrixBuffer(range(t.Transform3x4, 48));
                                gt.setTransformationMatrixLayout(MTLMatrixLayout::RowMajor);
                            }
                        }
                        Retained::into_super(gt)
                    }
                    Geometry::Aabbs { desc: a, .. } => {
                        let ga = MTL4AccelerationStructureBoundingBoxGeometryDescriptor::new();
                        unsafe {
                            ga.setBoundingBoxStride(a.Stride as usize);
                            ga.setBoundingBoxCount(a.AABBCount as usize);
                            // P3: the intersection function table holds the MSC triangle (0) and
                            // procedural (1) wrappers.
                            ga.setIntersectionFunctionTableOffset(1);
                        }
                        if with_buffers {
                            ga.setBoundingBoxBuffer(range(a.Start, if a.AABBCount == 0 { 0 } else { (a.AABBCount - 1) * a.Stride + 24 }));
                        }
                        Retained::into_super(ga)
                    }
                };
                gd.setOpaque(g.flags() & d3d::GEOMETRY_FLAG_OPAQUE != 0);
                gd.setAllowDuplicateIntersectionFunctionInvocation(g.flags() & d3d::GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION == 0);
                list.push(gd);
            }
            let p = MTL4PrimitiveAccelerationStructureDescriptor::new();
            p.setGeometryDescriptors(Some(&NSArray::from_retained_slice(&list)));
            p.setUsage(usage(*flags));
            Ok(Retained::into_super(p))
        }
    }
}

fn metal_sizes(desc: &MTL4AccelerationStructureDescriptor) -> Result<MTLAccelerationStructureSizes, String> {
    let d = crate::metal::device().ok_or("no device")?;
    Ok(d.dev.accelerationStructureSizesWithDescriptor(desc))
}

/// A BLAS without geometry: no Metal build; its header points at the dummy BLAS.
fn is_empty(inputs: &Inputs) -> bool {
    match inputs {
        Inputs::Bottom { geometries, .. } => geometries.is_empty() || geometries.iter().all(|g| match g {
            Geometry::Triangles { desc, .. } => triangle_count(desc) == 0,
            Geometry::Aabbs { desc, .. } => desc.AABBCount == 0,
        }),
        Inputs::Top { .. } => false,
    }
}

/// `GetRaytracingAccelerationStructurePrebuildInfo`.
pub fn prebuild_info(inputs: &Inputs) -> Result<PrebuildInfo, String> {
    let n = inputs.instance_count();
    if is_empty(inputs) {
        return Ok(PrebuildInfo { ResultDataMaxSizeInBytes: result_size(0), ScratchDataSizeInBytes: 256, UpdateScratchDataSizeInBytes: 256 });
    }
    let desc = descriptor(inputs, 0, false)?;
    let s = metal_sizes(&desc)?;
    let inst = if inputs.is_top() { instance_area(n) } else { 0 };
    let build = (s.buildScratchBufferSize.max(s.refitScratchBufferSize) as u64).next_multiple_of(256).max(256);
    let refit = (s.refitScratchBufferSize as u64).next_multiple_of(256).max(256);
    Ok(PrebuildInfo { ResultDataMaxSizeInBytes: result_size(n), ScratchDataSizeInBytes: inst + build, UpdateScratchDataSizeInBytes: inst + refit })
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// A Metal AS evicted from the registry. Keep it alive until the GPU work that may still use it
/// has completed (the recording list's allocator reset, or `Queue::defer_release`); dropping it
/// removes it from the device residency set.
pub struct Retired {
    accel: Retained<MtlAccel>,
}
unsafe impl Send for Retired {}

impl Drop for Retired {
    fn drop(&mut self) {
        if let Some(d) = crate::metal::device() {
            d.remove_resident(ProtocolObject::from_ref(&*self.accel));
        }
    }
}

struct Entry {
    /// Caller's resource identity (0 = unknown); a build by another owner replaces the entry.
    owner: u64,
    accel: Retained<MtlAccel>,
    metal_size: usize,
    reported: u64,
    top: bool,
    instances: u32,
    refittable: bool,
    /// Index of this entry's compacted-size slot.
    slot: Option<u32>,
}

/// Registry telemetry (fold `private_bytes` into `QueryVideoMemoryInfo` usage).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub live: u64,
    pub private_bytes: u64,
    pub builds: u64,
    pub refits: u64,
    pub copies: u64,
    pub compactions: u64,
    pub evicted_overlap: u64,
    pub evicted_release: u64,
}

struct Registry {
    map: BTreeMap<u64, Entry>,
    null_blas: Option<Retained<MtlAccel>>,
    slots: Option<Retained<MtlBuffer>>,
    free_slots: Vec<u32>,
    /// Largest reported size of any entry (bounds the overlap search).
    max_reported: u64,
    stats: Stats,
}
unsafe impl Send for Registry {}

fn registry() -> std::sync::MutexGuard<'static, Registry> {
    static R: OnceLock<Mutex<Registry>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(Registry { map: BTreeMap::new(), null_blas: None, slots: None, free_slots: Vec::new(), max_reported: 0, stats: Stats::default() })).lock().unwrap()
}

pub fn stats() -> Stats {
    let r = registry();
    Stats { live: r.map.len() as u64, private_bytes: r.map.values().map(|e| e.metal_size as u64).sum(), ..r.stats }
}

/// What the registry knows about the AS at `va`: (is TLAS, instance count, Metal size).
pub fn entry_info(va: u64) -> Option<(bool, u32, usize)> {
    registry().map.get(&va).map(|e| (e.top, e.instances, e.metal_size))
}

/// Drop every AS whose reported range intersects `[va, va + len)`: call when a buffer resource
/// is released, or a placed resource is created over the range (aliased AS contents are
/// undefined in D3D12).
pub fn evict_range(va: u64, len: u64) -> Vec<Retired> {
    let mut r = registry();
    let out = r.evict(va, len);
    r.stats.evicted_release += out.len() as u64;
    out
}

impl Registry {
    fn evict(&mut self, va: u64, len: u64) -> Vec<Retired> {
        let end = va.saturating_add(len.max(1));
        let lo = va.saturating_sub(self.max_reported);
        let hits: Vec<u64> = self.map.range(lo..end).filter(|(k, e)| *k + e.reported > va).map(|(k, _)| *k).collect();
        let mut out = Vec::with_capacity(hits.len());
        for k in hits {
            let e = self.map.remove(&k).unwrap();
            if let Some(s) = e.slot {
                self.free_slots.push(s);
            }
            out.push(Retired { accel: e.accel });
        }
        out
    }

    /// The Metal AS for a build/copy into `va`: the existing one when the owner matches and it
    /// is big enough (M3: identity is stable across same-size rebuilds), else a new one, retiring
    /// whatever overlaps the reported range.
    #[allow(clippy::too_many_arguments)]
    fn ensure(&mut self, va: u64, owner: u64, needed: usize, top: bool, instances: u32, refittable: bool, keep: &mut Vec<Retired>) -> Result<Retained<MtlAccel>, String> {
        let reported = result_size(instances);
        self.max_reported = self.max_reported.max(reported);
        if let Some(e) = self.map.get_mut(&va) {
            if e.owner == owner && e.metal_size >= needed {
                // A reused range may still overlap a neighbour if it grew (TLAS with more instances).
                let (top0, n0) = (e.top, e.instances);
                e.top = top;
                e.instances = instances;
                e.reported = reported;
                e.refittable = refittable;
                let accel = e.accel.clone();
                if result_size(n0) < reported || top0 != top {
                    let neighbours: Vec<u64> = self.map.range(va + 1..va + reported).map(|(k, _)| *k).collect();
                    for k in neighbours {
                        let e = self.map.remove(&k).unwrap();
                        keep.push(Retired { accel: e.accel });
                        self.stats.evicted_overlap += 1;
                    }
                }
                return Ok(accel);
            }
        }
        let old = self.evict(va, reported);
        self.stats.evicted_overlap += old.len() as u64;
        keep.extend(old);
        let d = crate::metal::device().ok_or("no device")?;
        let size = needed.max(256);
        let accel = d.dev.newAccelerationStructureWithSize(size).ok_or_else(|| format!("newAccelerationStructureWithSize({size})"))?;
        d.add_resident(ProtocolObject::from_ref(&*accel));
        self.map.insert(va, Entry { owner, accel: accel.clone(), metal_size: size, reported, top, instances, refittable, slot: None });
        Ok(accel)
    }

    /// The resident dummy BLAS for inactive instances / empty BLASes (one degenerate triangle),
    /// built synchronously on first use.
    fn null_blas(&mut self) -> Result<u64, String> {
        if let Some(a) = &self.null_blas {
            return Ok(resource_id(a));
        }
        let a = build_null_blas()?;
        let id = resource_id(&a);
        self.null_blas = Some(a);
        Ok(id)
    }

    /// The compacted-size slot of the entry at `va` (zeroed): (slot GPU VA, index).
    fn slot(&mut self, va: u64) -> Result<Option<u64>, String> {
        if self.slots.is_none() {
            let d = crate::metal::device().ok_or("no device")?;
            let b = d.dev.newBufferWithLength_options(SLOTS as usize * 8, MTLResourceOptions::StorageModeShared).ok_or("rt slot buffer")?;
            d.add_resident(ProtocolObject::from_ref(&*b));
            self.slots = Some(b);
            self.free_slots = (0..SLOTS).rev().collect();
        }
        let Some(e) = self.map.get_mut(&va) else { return Ok(None) };
        let idx = match e.slot {
            Some(i) => i,
            None => match self.free_slots.pop() {
                Some(i) => {
                    e.slot = Some(i);
                    i
                }
                None => return Ok(None),
            },
        };
        let b = self.slots.as_ref().unwrap();
        unsafe { *(b.contents().as_ptr() as *mut u64).add(idx as usize) = 0 };
        Ok(Some(b.gpuAddress() + idx as u64 * 8))
    }

    /// The compacted size Metal wrote for the entry at `va`, if known.
    fn compacted_size(&self, va: u64) -> Option<usize> {
        let e = self.map.get(&va)?;
        let b = self.slots.as_ref()?;
        let v = unsafe { std::ptr::read_volatile((b.contents().as_ptr() as *const u64).add(e.slot? as usize)) };
        (v != 0).then_some(v as usize)
    }
}

fn build_null_blas() -> Result<Retained<MtlAccel>, String> {
    let d = crate::metal::device().ok_or("no device")?;
    let dev = &d.dev;
    let vb = dev.newBufferWithLength_options(64, MTLResourceOptions::StorageModeShared).ok_or("null BLAS vertices")?;
    let t = MTL4AccelerationStructureTriangleGeometryDescriptor::new();
    unsafe {
        t.setVertexBuffer(range(vb.gpuAddress(), 36));
        t.setVertexStride(12);
        t.setTriangleCount(1);
    }
    t.setVertexFormat(MTLAttributeFormat::Float3);
    let desc = MTL4PrimitiveAccelerationStructureDescriptor::new();
    desc.setGeometryDescriptors(Some(&NSArray::from_retained_slice(&[Retained::into_super(t)])));
    let s = dev.accelerationStructureSizesWithDescriptor(&desc);
    let accel = dev.newAccelerationStructureWithSize(s.accelerationStructureSize.max(256)).ok_or("null BLAS")?;
    let scratch = dev.newBufferWithLength_options(s.buildScratchBufferSize.max(256), MTLResourceOptions::StorageModePrivate).ok_or("null BLAS scratch")?;

    let rsd = MTLResidencySetDescriptor::new();
    let rs = dev.newResidencySetWithDescriptor_error(&rsd).map_err(|e| format!("residency set: {e}"))?;
    rs.addAllocation(ProtocolObject::from_ref(&*vb));
    rs.addAllocation(ProtocolObject::from_ref(&*scratch));
    rs.addAllocation(ProtocolObject::from_ref(&*accel));
    rs.commit();
    let q = dev.newMTL4CommandQueue().ok_or("queue")?;
    let alloc = dev.newCommandAllocator().ok_or("allocator")?;
    let cb = dev.newCommandBuffer().ok_or("command buffer")?;
    cb.beginCommandBufferWithAllocator(&alloc);
    cb.useResidencySet(&rs);
    let enc = cb.computeCommandEncoder().ok_or("compute encoder")?;
    unsafe { enc.buildAccelerationStructure_descriptor_scratchBuffer(&accel, &desc, range(scratch.gpuAddress(), scratch.length() as u64)) };
    enc.endEncoding();
    cb.endCommandBuffer();
    let ev = dev.newSharedEvent().ok_or("event")?;
    let mut cbs = [std::ptr::NonNull::from(&*cb)];
    unsafe { q.commit_count(std::ptr::NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
    q.signalEvent_value(ProtocolObject::from_ref(&*ev), 1);
    if !ev.waitUntilSignaledValue_timeoutMS(1, 10_000) {
        return Err("null BLAS build timed out".into());
    }
    d.add_resident(ProtocolObject::from_ref(&*accel));
    Ok(accel)
}

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

/// Where AS commands are recorded. The functions leave `table` and their own PSO bound on `enc`.
pub struct Ctx<'a> {
    pub enc: &'a MtlComputeEncoder,
    /// Utility table with at least one buffer binding.
    pub table: &'a MtlArgumentTable,
    pub ring: &'a mut UploadRing,
    /// Metal objects replaced while recording; keep them until this list's GPU work is done.
    pub keep: &'a mut Vec<Retired>,
}

fn barrier(enc: &MtlComputeEncoder, after: MTLStages, before: MTLStages) {
    enc.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(after, before, MTL4VisibilityOptions::Device);
}

fn warn_once(flag: &AtomicU32, bit: u32, msg: &str) {
    if flag.fetch_or(bit, Ordering::Relaxed) & bit == 0 {
        log::warn!(target: "d3d", "rt: {msg} (logged once)");
    }
}
static WARNED: AtomicU32 = AtomicU32::new(0);

/// `D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC`, with owned inputs.
#[derive(Clone, Debug)]
pub struct BuildArgs {
    pub dest: u64,
    pub source: u64,
    pub scratch: u64,
    pub inputs: Inputs,
    /// Identity of the destination's buffer resource (0 if unknown).
    pub owner: u64,
}

/// `BuildRaytracingAccelerationStructure` (with its optional postbuild info).
///
/// Order on the encoder: [TLAS: instance conversion + header, barrier Dispatch → AS]
/// [BLAS: header write] → build or refit → [barrier AS → AS, postbuild]. No barrier between
/// independent builds: the app's UAV barrier orders them for consumers.
pub fn encode_build(ctx: &mut Ctx, a: &BuildArgs, postbuild: &[PostbuildInfoDesc]) -> Result<(), String> {
    if a.dest == 0 || !a.dest.is_multiple_of(d3d::AS_ALIGNMENT) {
        return Err(format!("AS build into {:#x} (needs a 256-byte aligned VA)", a.dest));
    }
    let inputs = &a.inputs;
    let flags = inputs.flags();
    let top = inputs.is_top();
    let n = inputs.instance_count();
    let mut reg = registry();

    if is_empty(inputs) {
        let null = reg.null_blas()?;
        reg.evict(a.dest, result_size(0)).into_iter().for_each(|r| ctx.keep.push(r));
        kernels::write_qwords(ctx.enc, ctx.table, ctx.ring, a.dest, &header_qwords(null, a.dest))?;
        drop(reg);
        return encode_postbuild_locked(ctx, &[a.dest], postbuild, false);
    }

    let instances_va = if top { a.scratch } else { 0 };
    let metal_scratch = if top { a.scratch + instance_area(n) } else { a.scratch };
    let desc = descriptor(inputs, instances_va, true)?;
    let sizes = metal_sizes(&desc)?;

    // Refit source (in place when SourceAccelerationStructureData is 0 or the destination).
    let source_va = if a.source == 0 { a.dest } else { a.source };
    let mut refit_from = None;
    if flags & build_flags::PERFORM_UPDATE != 0 {
        match reg.map.get(&source_va) {
            Some(e) if e.refittable && e.top == top => refit_from = Some((e.accel.clone(), e.metal_size)),
            _ => warn_once(&WARNED, 1, "PERFORM_UPDATE without a refittable source: full rebuild"),
        }
    }
    let refittable = flags & build_flags::ALLOW_UPDATE != 0;
    let needed = match &refit_from {
        Some((_, src_size)) => (*src_size).max(sizes.accelerationStructureSize),
        None => sizes.accelerationStructureSize,
    };
    let dst = reg.ensure(a.dest, a.owner, needed, top, n, refittable, ctx.keep)?;
    let dst_id = resource_id(&dst);

    if top {
        if a.scratch == 0 {
            return Err("TLAS build without scratch".into());
        }
        let Inputs::Top { instances, pointers, .. } = *inputs else { unreachable!() };
        let null = reg.null_blas()?;
        let p = ConvertParams { src: instances, dst: instances_va, header: a.dest, tlas_asid: dst_id, null_asid: null, count: n, pointers: pointers as u32 };
        kernels::dispatch(ctx.enc, ctx.table, ctx.ring, "aw2_rt_instances", &p, n)?;
        barrier(ctx.enc, MTLStages::Dispatch, MTLStages::AccelerationStructure);
    } else {
        kernels::write_qwords(ctx.enc, ctx.table, ctx.ring, a.dest, &header_qwords(dst_id, a.dest))?;
    }

    unsafe {
        match refit_from {
            Some((src, _)) => {
                let len = sizes.refitScratchBufferSize as u64;
                let scratch = range(if len == 0 { 0 } else { metal_scratch }, len);
                let in_place = source_va == a.dest;
                ctx.enc.refitAccelerationStructure_descriptor_destination_scratchBuffer(&src, &desc, if in_place { None } else { Some(&dst) }, scratch);
                reg.stats.refits += 1;
            }
            None => {
                if metal_scratch == 0 && sizes.buildScratchBufferSize > 0 {
                    return Err("AS build without scratch".into());
                }
                ctx.enc.buildAccelerationStructure_descriptor_scratchBuffer(&dst, &desc, range(metal_scratch, sizes.buildScratchBufferSize as u64));
                reg.stats.builds += 1;
            }
        }
    }
    drop(reg);
    encode_postbuild_locked(ctx, &[a.dest], postbuild, true)
}

/// `EmitRaytracingAccelerationStructurePostbuildInfo` over `sources`.
pub fn encode_postbuild(ctx: &mut Ctx, desc: &PostbuildInfoDesc, sources: &[u64]) -> Result<(), String> {
    encode_postbuild_locked(ctx, sources, std::slice::from_ref(desc), false)
}

/// Writes each postbuild desc's info for every source, consecutively at `DestBuffer`.
/// `after_build`: the sources were just built on this encoder (Metal's compacted-size query
/// then needs an AS → AS barrier).
fn encode_postbuild_locked(ctx: &mut Ctx, sources: &[u64], descs: &[PostbuildInfoDesc], after_build: bool) -> Result<(), String> {
    use d3d::postbuild::*;
    let mut barrier_done = !after_build;
    for pd in descs {
        let mut values = Vec::with_capacity(sources.len() * 2);
        let mut reg = registry();
        for &va in sources {
            let reported = reg.map.get(&va).map(|e| e.reported).unwrap_or_else(|| result_size(0));
            match pd.InfoType {
                COMPACTED_SIZE => {
                    values.push(reported);
                    // Metal's own compacted size, for an exact COMPACT copy later.
                    if let (Some(slot), Some(accel)) = (reg.slot(va)?, reg.map.get(&va).map(|e| e.accel.clone())) {
                        if !barrier_done {
                            barrier(ctx.enc, MTLStages::AccelerationStructure, MTLStages::AccelerationStructure);
                            barrier_done = true;
                        }
                        unsafe { ctx.enc.writeCompactedAccelerationStructureSize_toBuffer(&accel, range(slot, 8)) };
                    }
                }
                CURRENT_SIZE => values.push(reported),
                TOOLS_VISUALIZATION => {
                    warn_once(&WARNED, 2, "TOOLS_VISUALIZATION postbuild info: writing the header size");
                    values.push(reported);
                }
                SERIALIZATION => {
                    warn_once(&WARNED, 4, "SERIALIZATION postbuild info: writing the header size, 0 BLAS pointers");
                    values.extend([reported, 0]);
                }
                t => return Err(format!("unknown postbuild info type {t}")),
            }
        }
        drop(reg);
        for (i, chunk) in values.chunks(8).enumerate() {
            kernels::write_qwords(ctx.enc, ctx.table, ctx.ring, pd.DestBuffer + i as u64 * 64, chunk)?;
        }
    }
    Ok(())
}

/// `CopyRaytracingAccelerationStructure`.
pub fn encode_copy(ctx: &mut Ctx, dest: u64, source: u64, mode: u32, owner: u64) -> Result<(), String> {
    use d3d::copy_mode::*;
    match mode {
        CLONE | COMPACT => {}
        VISUALIZATION_DECODE_FOR_TOOLS | SERIALIZE | DESERIALIZE => {
            warn_once(&WARNED, 8, &format!("AS copy mode {mode} is not supported: skipped"));
            return Ok(());
        }
        m => return Err(format!("unknown AS copy mode {m}")),
    }
    if !dest.is_multiple_of(d3d::AS_ALIGNMENT) || dest == 0 {
        return Err(format!("AS copy into {dest:#x} (needs a 256-byte aligned VA)"));
    }
    let mut reg = registry();
    let Some(src) = reg.map.get(&source) else {
        return Err(format!("AS copy from {source:#x}: no acceleration structure there"));
    };
    let (src_accel, src_size, top, n, refittable) = (src.accel.clone(), src.metal_size, src.top, src.instances, src.refittable);
    let compacted = if mode == COMPACT { reg.compacted_size(source) } else { None };
    let size = compacted.unwrap_or(src_size);
    let dst = reg.ensure(dest, owner, size, top, n, refittable, ctx.keep)?;
    unsafe {
        if compacted.is_some() {
            ctx.enc.copyAndCompactAccelerationStructure_toAccelerationStructure(&src_accel, &dst);
            reg.stats.compactions += 1;
        } else {
            ctx.enc.copyAccelerationStructure_toAccelerationStructure(&src_accel, &dst);
            reg.stats.copies += 1;
        }
    }
    let id = resource_id(&dst);
    drop(reg);
    // M1: the destination gets its own header and contributions array.
    kernels::write_qwords(ctx.enc, ctx.table, ctx.ring, dest, &header_qwords(id, dest))?;
    if top {
        kernels::copy_dwords(ctx.enc, ctx.table, ctx.ring, dest + HEADER_SIZE, source + HEADER_SIZE, n)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_and_header() {
        assert_eq!(result_size(0), 256);
        assert_eq!(result_size(48), 256);
        assert_eq!(result_size(49), 512);
        assert_eq!(instance_area(1), 256);
        assert_eq!(instance_area(4), 512);
        assert_eq!(header_qwords(7, 0x1000), [7, 0x1040, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn formats_and_flags() {
        assert_eq!(vertex_format(6), Some((MTLAttributeFormat::Float3, 12)));
        assert_eq!(vertex_format(10), Some((MTLAttributeFormat::Half3, 8)));
        assert_eq!(vertex_format(2), None);
        assert_eq!(index_type(57).unwrap(), Some((MTLIndexType::UInt16, 2)));
        assert_eq!(index_type(0).unwrap(), None);
        assert!(index_type(28).is_err());
        assert_eq!(usage(0), MTLAccelerationStructureUsage::None);
        assert_eq!(usage(build_flags::ALLOW_UPDATE | build_flags::PREFER_FAST_BUILD), MTLAccelerationStructureUsage::Refit | MTLAccelerationStructureUsage::PreferFastBuild);
        assert_eq!(usage(build_flags::PREFER_FAST_TRACE | build_flags::ALLOW_COMPACTION), MTLAccelerationStructureUsage::PreferFastIntersection);
    }
}
