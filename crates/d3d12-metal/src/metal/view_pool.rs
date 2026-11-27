//! Metal 4 `MTLTextureViewPool` allocator for lightweight shader-visible texture
//! and typed-buffer views.
//!
//! D3D12 SRVs and UAVs only need a 64-bit `MTLResourceID` inside each 24-byte
//! `IRDescriptorTableEntry` (`dxil_metal::runtime`). Allocating non-identity
//! views from a chunked `MTLTextureViewPool` avoids creating a full Objective-C
//! `MTLTexture` object per view while keeping the exact same shader ABI.
//!
//! Retired slots are returned to the freelist through
//! [`super::queue::defer_release_all`] so a slot is never overwritten while
//! in-flight GPU work may still reference its previous descriptor.
//!
//! Set `PORT_TEXTURE_VIEW_POOL=0` to disable pooling and fall back to classic
//! `newTextureViewWithPixelFormat:...` / `newTextureWithDescriptor:offset:bytesPerRow:`.

use super::resource::{MtlBuffer, MtlTexture};
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Bool};
use objc2_foundation::{NSRange, NSString};
use objc2_metal::*;
use std::sync::{Mutex, OnceLock};

/// Views allocated per `MTLTextureViewPool` chunk.
const CHUNK_SLOTS: usize = 4096;

/// Opaque address of a slot inside the device's chunked `MTLTextureViewPool`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PoolSlot {
    pub chunk: u32,
    pub slot: u32,
}

/// A pooled lightweight texture view (`MTLResourceID` + its pool slot).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PooledView {
    pub id: u64,
    pub slot: PoolSlot,
}

/// Runtime telemetry for the device's `MTLTextureViewPool` allocator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ViewPoolStats {
    pub enabled: bool,
    pub chunks: usize,
    pub capacity: usize,
    pub active_views: usize,
    pub free_slots: usize,
    pub total_texture_views: u64,
    pub total_buffer_views: u64,
    pub total_recycled: u64,
}

/// True when `MTLTextureViewPool` is enabled by config (default ON) and the
/// Metal runtime exposes `MTLResourceViewPoolDescriptor`,
/// `MTLTextureViewDescriptor`, and `-[MTLDevice newTextureViewPoolWithDescriptor:error:]`.
pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        if let Some(v) = super::env_canonical_or_alias("PORT_TEXTURE_VIEW_POOL", "AW2_TEXTURE_VIEW_POOL") {
            if !super::parse_bool_knob(&v) {
                log::info!(target: "d3d", "MTLTextureViewPool disabled via PORT_TEXTURE_VIEW_POOL={v}");
                return false;
            }
        }
        let Some(dev) = super::device() else {
            return false;
        };
        let has_classes = AnyClass::get(c"MTLResourceViewPoolDescriptor").is_some()
            && AnyClass::get(c"MTLTextureViewDescriptor").is_some();
        if !has_classes {
            return false;
        }
        let responds: Bool = unsafe {
            msg_send![&*dev.dev, respondsToSelector: objc2::sel!(newTextureViewPoolWithDescriptor:error:)]
        };
        responds.as_bool()
    })
}

struct PoolChunk {
    /// Retained `id<MTLTextureViewPool>`. Kept alive for the lifetime of the device.
    pool: Retained<AnyObject>,
    /// Next never-used slot index in `0..CHUNK_SLOTS`.
    next_slot: u32,
}

// SAFETY: `MTLTextureViewPool` is only accessed while holding `PoolState`'s `Mutex`.
unsafe impl Send for PoolChunk {}

#[derive(Default)]
struct PoolState {
    chunks: Vec<PoolChunk>,
    free_slots: Vec<PoolSlot>,
    active_views: usize,
    total_texture_views: u64,
    total_buffer_views: u64,
    total_recycled: u64,
    /// Set if pool creation fails once so we don't spam errors on every view.
    creation_failed: bool,
}

fn global_pool() -> &'static Mutex<PoolState> {
    static P: OnceLock<Mutex<PoolState>> = OnceLock::new();
    P.get_or_init(|| Mutex::new(PoolState::default()))
}

fn alloc_chunk(chunk_idx: usize) -> Option<PoolChunk> {
    let dev = super::device()?;
    let desc_cls = AnyClass::get(c"MTLResourceViewPoolDescriptor")?;
    objc2::rc::autoreleasepool(|_| unsafe {
        let desc: Retained<AnyObject> = msg_send![desc_cls, new];
        let _: () = msg_send![&*desc, setResourceViewCount: CHUNK_SLOTS];
        let label = NSString::from_str(&format!("port d3d12 texture view pool #{chunk_idx}"));
        let _: () = msg_send![&*desc, setLabel: &*label];
        let mut err: *mut AnyObject = std::ptr::null_mut();
        let pool: Option<Retained<AnyObject>> =
            msg_send![&*dev.dev, newTextureViewPoolWithDescriptor: &*desc, error: &mut err];
        if pool.is_none() {
            if !err.is_null() {
                let localized: Option<Retained<NSString>> = msg_send![err, localizedDescription];
                let msg = localized.map(|s| s.to_string()).unwrap_or_else(|| "unknown error".into());
                log::warn!(target: "d3d", "newTextureViewPoolWithDescriptor({CHUNK_SLOTS}) failed: {msg}");
            } else {
                log::warn!(target: "d3d", "newTextureViewPoolWithDescriptor({CHUNK_SLOTS}) returned nil");
            }
            return None;
        }
        Some(PoolChunk { pool: pool.unwrap(), next_slot: 0 })
    })
}

impl PoolState {
    fn alloc_slot(&mut self) -> Option<PoolSlot> {
        if let Some(slot) = self.free_slots.pop() {
            self.active_views += 1;
            return Some(slot);
        }
        if let Some((idx, chunk)) = self.chunks.iter_mut().enumerate().last() {
            if (chunk.next_slot as usize) < CHUNK_SLOTS {
                let slot = chunk.next_slot;
                chunk.next_slot += 1;
                self.active_views += 1;
                return Some(PoolSlot { chunk: idx as u32, slot });
            }
        }
        if self.creation_failed {
            return None;
        }
        let chunk_idx = self.chunks.len();
        let Some(mut chunk) = alloc_chunk(chunk_idx) else {
            self.creation_failed = true;
            return None;
        };
        chunk.next_slot = 1;
        self.chunks.push(chunk);
        self.active_views += 1;
        Some(PoolSlot { chunk: chunk_idx as u32, slot: 0 })
    }

    fn return_slot_unused(&mut self, slot: PoolSlot) {
        self.active_views = self.active_views.saturating_sub(1);
        self.free_slots.push(slot);
    }

    fn release_slots(&mut self, slots: &[PoolSlot]) {
        self.active_views = self.active_views.saturating_sub(slots.len());
        self.total_recycled += slots.len() as u64;
        self.free_slots.extend_from_slice(slots);
    }
}

/// Allocate a lightweight texture view in the device `MTLTextureViewPool`.
///
/// Returns `None` if pooling is disabled/unavailable or Metal rejects the view
/// (the caller should fall back to `newTextureViewWithPixelFormat:...`).
pub fn create_texture_view(
    tex: &MtlTexture,
    fmt: MTLPixelFormat,
    ty: MTLTextureType,
    mips: (u32, u32),
    slices: (u32, u32),
    swizzle: MTLTextureSwizzleChannels,
) -> Option<PooledView> {
    if !enabled() {
        return None;
    }
    let tvd_cls = AnyClass::get(c"MTLTextureViewDescriptor")?;
    let mut state = global_pool().lock().unwrap();
    let slot = state.alloc_slot()?;
    let id = objc2::rc::autoreleasepool(|_| unsafe {
        let tvd: Retained<AnyObject> = msg_send![tvd_cls, new];
        let _: () = msg_send![&*tvd, setPixelFormat: fmt];
        let _: () = msg_send![&*tvd, setTextureType: ty];
        let _: () = msg_send![&*tvd, setLevelRange: NSRange::new(mips.0 as usize, mips.1 as usize)];
        let _: () = msg_send![&*tvd, setSliceRange: NSRange::new(slices.0 as usize, slices.1 as usize)];
        let _: () = msg_send![&*tvd, setSwizzle: swizzle];
        let pool = &state.chunks[slot.chunk as usize].pool;
        let rid: MTLResourceID =
            msg_send![&**pool, setTextureView: tex, descriptor: &*tvd, atIndex: slot.slot as usize];
        std::mem::transmute::<MTLResourceID, u64>(rid)
    });
    if id == 0 {
        state.return_slot_unused(slot);
        return None;
    }
    state.total_texture_views += 1;
    Some(PooledView { id, slot })
}

/// Allocate a lightweight texture-buffer view of `buf` in the device `MTLTextureViewPool`.
///
/// Returns `None` if pooling is disabled/unavailable or Metal rejects the view
/// (the caller should fall back to `newTextureWithDescriptor:offset:bytesPerRow:`).
pub fn create_buffer_texture_view(
    buf: &MtlBuffer,
    desc: &MTLTextureDescriptor,
    offset: u64,
    bytes_per_row: u64,
) -> Option<PooledView> {
    if !enabled() {
        return None;
    }
    let mut state = global_pool().lock().unwrap();
    let slot = state.alloc_slot()?;
    let id = objc2::rc::autoreleasepool(|_| unsafe {
        let pool = &state.chunks[slot.chunk as usize].pool;
        let rid: MTLResourceID = msg_send![
            &**pool,
            setTextureViewFromBuffer: buf,
            descriptor: desc,
            offset: offset as usize,
            bytesPerRow: bytes_per_row as usize,
            atIndex: slot.slot as usize
        ];
        std::mem::transmute::<MTLResourceID, u64>(rid)
    });
    if id == 0 {
        state.return_slot_unused(slot);
        return None;
    }
    state.total_buffer_views += 1;
    Some(PooledView { id, slot })
}

struct RetiredPoolSlots(Vec<PoolSlot>);

impl Drop for RetiredPoolSlots {
    fn drop(&mut self) {
        if self.0.is_empty() {
            return;
        }
        if let Ok(mut state) = global_pool().lock() {
            state.release_slots(&self.0);
        }
    }
}

/// Defer returning `slots` to the pool freelist until all live Metal 4 queues
/// have finished work submitted so far.
pub fn retire_slots(slots: Vec<PoolSlot>) {
    if slots.is_empty() {
        return;
    }
    super::queue::defer_release_all(RetiredPoolSlots(slots));
}

/// Snapshot current `MTLTextureViewPool` allocation statistics.
pub fn stats() -> ViewPoolStats {
    let is_on = enabled();
    let state = global_pool().lock().unwrap();
    ViewPoolStats {
        enabled: is_on,
        chunks: state.chunks.len(),
        capacity: state.chunks.len() * CHUNK_SLOTS,
        active_views: state.active_views,
        free_slots: state.free_slots.len(),
        total_texture_views: state.total_texture_views,
        total_buffer_views: state.total_buffer_views,
        total_recycled: state.total_recycled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::queue::{self, ListType, Queue};
    use crate::metal::resource::{with_resource_registry, Resource};
    use crate::metal::views;
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    fn tex_desc_2d_array(w: u64, h: u32, mips: u16, slices: u16, flags: u32) -> ResourceDesc {
        ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: w,
            Height: h,
            DepthOrArraySize: slices,
            MipLevels: mips,
            Format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
            SampleCount: 1,
            Layout: 0,
            Flags: flags,
            ..Default::default()
        }
    }

    fn buf_desc(bytes: u64, flags: u32) -> ResourceDesc {
        ResourceDesc {
            Dimension: dim::BUFFER,
            Width: bytes,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: flags,
            ..Default::default()
        }
    }

    #[test]
    fn pool_allocates_caches_registers_and_recycles() {
        if super::super::device().is_none() || !enabled() {
            return;
        }
        queue::collect_all();
        let before = stats();

        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let tex = Resource::new_committed_default(
            &tex_desc_2d_array(8, 8, 2, 2, resource_flags::ALLOW_UNORDERED_ACCESS),
            &hp,
            "view_pool_test_tex",
        )
        .unwrap();
        let buf = Resource::new_committed_default(&buf_desc(256, 0), &hp, "view_pool_test_buf").unwrap();

        let tex_key = &tex as *const Resource as usize;
        let buf_key = &buf as *const Resource as usize;

        // 1. Identity SRV uses the base texture's gpuResourceID directly (0 pool slots).
        let id_entry = views::srv_entry(Some(&tex), None).unwrap();
        let id_tex = u64::from_le_bytes(id_entry[8..16].try_into().unwrap());
        assert_ne!(id_tex, 0);
        let after_identity = stats();
        assert_eq!(after_identity.total_texture_views, before.total_texture_views);

        // 2. Non-identity SRV (mip 1, slice 1, custom BGRA swizzle) allocates 1 pooled texture view.
        //    Shader4ComponentMapping: R<-B(2), G<-G(1), B<-R(0), A<-A(3) = 2 | (1<<3) | (0<<6) | (3<<9) = 0x060A.
        let mut srv_desc = [0u8; 40];
        srv_desc[0..4].copy_from_slice(&28u32.to_le_bytes()); // R8G8B8A8_UNORM
        srv_desc[4..8].copy_from_slice(&5u32.to_le_bytes()); // TEXTURE2DARRAY
        srv_desc[8..12].copy_from_slice(&0x060Au32.to_le_bytes());
        srv_desc[16..20].copy_from_slice(&1u32.to_le_bytes()); // MostDetailedMip = 1
        srv_desc[20..24].copy_from_slice(&1u32.to_le_bytes()); // MipLevels = 1
        srv_desc[24..28].copy_from_slice(&1u32.to_le_bytes()); // FirstArraySlice = 1
        srv_desc[28..32].copy_from_slice(&1u32.to_le_bytes()); // ArraySize = 1

        let pooled_entry = views::srv_entry(Some(&tex), Some(&srv_desc)).unwrap();
        let pooled_tex_id = u64::from_le_bytes(pooled_entry[8..16].try_into().unwrap());
        assert_ne!(pooled_tex_id, 0);
        assert_ne!(pooled_tex_id, id_tex);

        let after_tex_view = stats();
        assert_eq!(after_tex_view.total_texture_views, before.total_texture_views + 1);
        assert_eq!(after_tex_view.active_views, before.active_views + 1);
        assert_eq!(
            with_resource_registry(|r| r.lookup_texture_id(pooled_tex_id)),
            Some(tex_key),
            "pooled texture view ID must resolve in ResRegistry"
        );

        // 3. Repeat lookup with identical desc hits Resource.pooled_views cache (0 new pool slots).
        let cached_entry = views::srv_entry(Some(&tex), Some(&srv_desc)).unwrap();
        assert_eq!(cached_entry, pooled_entry);
        assert_eq!(stats().total_texture_views, after_tex_view.total_texture_views);

        // 4. Typed-buffer SRV (R32_UINT) allocates 1 pooled buffer texture view.
        let mut buf_srv = [0u8; 40];
        buf_srv[0..4].copy_from_slice(&42u32.to_le_bytes()); // DXGI_FORMAT_R32_UINT
        buf_srv[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
        buf_srv[8..12].copy_from_slice(&0x1688u32.to_le_bytes());
        buf_srv[16..24].copy_from_slice(&0u64.to_le_bytes()); // FirstElement
        buf_srv[24..28].copy_from_slice(&16u32.to_le_bytes()); // NumElements

        let buf_entry = views::srv_entry(Some(&buf), Some(&buf_srv)).unwrap();
        let pooled_buf_id = u64::from_le_bytes(buf_entry[8..16].try_into().unwrap());
        assert_ne!(pooled_buf_id, 0);
        let after_buf_view = stats();
        assert_eq!(after_buf_view.total_buffer_views, before.total_buffer_views + 1);
        assert_eq!(after_buf_view.active_views, before.active_views + 2);
        assert_eq!(
            with_resource_registry(|r| r.lookup_texture_id(pooled_buf_id)),
            Some(buf_key),
            "pooled typed-buffer view ID must resolve in ResRegistry"
        );

        // 5. Dropping resources removes ResRegistry entries and recycles pool slots.
        drop(tex);
        drop(buf);
        queue::collect_all();
        assert_eq!(with_resource_registry(|r| r.lookup_texture_id(pooled_tex_id)), None);
        assert_eq!(with_resource_registry(|r| r.lookup_texture_id(pooled_buf_id)), None);
        let after_drop = stats();
        assert_eq!(after_drop.active_views, before.active_views);
        assert!(after_drop.total_recycled >= before.total_recycled + 2);
        assert!(after_drop.free_slots >= 2);
    }

    #[test]
    fn gpu_shader_reads_and_writes_pooled_views() {
        let Some(dev) = super::super::device() else {
            return;
        };
        if !enabled() {
            return;
        }
        queue::collect_all();

        let q = Queue::new(ListType::Direct, "vp_gpu_test").unwrap();
        let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
        let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };

        // 8x8 2-mip, 2-slice RGBA8Unorm texture (StorageModePrivate).
        let tex = Resource::new_committed_default(
            &tex_desc_2d_array(8, 8, 2, 2, resource_flags::ALLOW_UNORDERED_ACCESS),
            &hp_def,
            "vp_gpu_tex",
        )
        .unwrap();

        // Typed buffer (UPLOAD = StorageModeShared) with 4 known u32 elements.
        let tbuf = Resource::new_committed_default(&buf_desc(256, 0), &hp_up, "vp_gpu_tbuf").unwrap();
        let expected_u32s: [u32; 4] = [0x1122_3344, 0x5566_7788, 0x99AA_BBCC, 0xDDEE_FF00];
        unsafe {
            let p = tbuf.cpu_ptr().unwrap() as *mut u32;
            for (i, &v) in expected_u32s.iter().enumerate() {
                p.add(i).write(v);
            }
        }

        // Readback buffer: 4 floats (swizzled RGBA from mip 1 slice 1) + 4 u32s (typed buffer).
        let out_buf = Resource::new_committed_default(&buf_desc(256, 0), &hp_rb, "vp_gpu_out").unwrap();

        // Descriptor 0: pooled UAV for mip 1, slice 1 (TEXTURE2DARRAY).
        let mut uav_desc = [0u8; 40];
        uav_desc[0..4].copy_from_slice(&28u32.to_le_bytes()); // R8G8B8A8_UNORM
        uav_desc[4..8].copy_from_slice(&5u32.to_le_bytes()); // TEXTURE2DARRAY
        uav_desc[8..12].copy_from_slice(&1u32.to_le_bytes()); // MipSlice = 1
        uav_desc[12..16].copy_from_slice(&1u32.to_le_bytes()); // FirstArraySlice = 1
        uav_desc[16..20].copy_from_slice(&1u32.to_le_bytes()); // ArraySize = 1
        let e_uav = views::uav_entry(Some(&tex), Some(&uav_desc)).unwrap();

        // Descriptor 1: pooled SRV for mip 1, slice 1 with R<->B swizzle (0x060A).
        let mut srv_desc = [0u8; 40];
        srv_desc[0..4].copy_from_slice(&28u32.to_le_bytes());
        srv_desc[4..8].copy_from_slice(&5u32.to_le_bytes()); // TEXTURE2DARRAY
        srv_desc[8..12].copy_from_slice(&0x060Au32.to_le_bytes()); // R<-B, G<-G, B<-R, A<-A
        srv_desc[16..20].copy_from_slice(&1u32.to_le_bytes()); // MostDetailedMip = 1
        srv_desc[20..24].copy_from_slice(&1u32.to_le_bytes()); // MipLevels = 1
        srv_desc[24..28].copy_from_slice(&1u32.to_le_bytes()); // FirstArraySlice = 1
        srv_desc[28..32].copy_from_slice(&1u32.to_le_bytes()); // ArraySize = 1
        let e_srv = views::srv_entry(Some(&tex), Some(&srv_desc)).unwrap();

        // Descriptor 2: pooled typed-buffer SRV (R32_UINT, 4 elements).
        let mut tbuf_srv = [0u8; 40];
        tbuf_srv[0..4].copy_from_slice(&42u32.to_le_bytes()); // R32_UINT
        tbuf_srv[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
        tbuf_srv[8..12].copy_from_slice(&0x1688u32.to_le_bytes());
        tbuf_srv[16..24].copy_from_slice(&0u64.to_le_bytes()); // FirstElement
        tbuf_srv[24..28].copy_from_slice(&4u32.to_le_bytes()); // NumElements
        let e_tbuf = views::srv_entry(Some(&tbuf), Some(&tbuf_srv)).unwrap();

        let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "vp_gpu_heap").unwrap();
        unsafe {
            let base = heap.cpu_base as *mut u8;
            std::ptr::copy_nonoverlapping(e_uav.as_ptr(), base, 24);
            std::ptr::copy_nonoverlapping(e_srv.as_ptr(), base.add(24), 24);
            std::ptr::copy_nonoverlapping(e_tbuf.as_ptr(), base.add(48), 24);
        }

        // MSL compute kernels reading the exact 24-byte IRDescriptorTableEntry layout.
        let src = r#"
            #include <metal_stdlib>
            using namespace metal;
            struct TexUavEntry {
                uint64_t gpu_va;
                texture2d_array<float, access::write> tex;
                uint64_t meta;
            };
            struct TexSrvEntry {
                uint64_t gpu_va;
                texture2d_array<float, access::read> tex;
                uint64_t meta;
            };
            struct BufSrvEntry {
                uint64_t gpu_va;
                texture_buffer<uint, access::read> tbuf;
                uint64_t meta;
            };
            struct OutData {
                float4 color;
                uint4 values;
            };
            kernel void write_pooled(
                device const TexUavEntry* uav [[buffer(0)]]
            ) {
                // Write (R=1.0, G=0.5, B=0.25, A=1.0) into mip 1, slice 1 at (1, 1).
                uav[0].tex.write(float4(1.0f, 0.5f, 0.25f, 1.0f), uint2(1, 1), 0);
            }
            kernel void read_pooled(
                device const uchar* heap_bytes [[buffer(0)]],
                device OutData* out [[buffer(1)]]
            ) {
                device const TexSrvEntry* srv = reinterpret_cast<device const TexSrvEntry*>(heap_bytes + 24);
                device const BufSrvEntry* bsv = reinterpret_cast<device const BufSrvEntry*>(heap_bytes + 48);
                out->color = srv->tex.read(uint2(1, 1), 0);
                out->values = uint4(
                    bsv->tbuf.read(0u).x,
                    bsv->tbuf.read(1u).x,
                    bsv->tbuf.read(2u).x,
                    bsv->tbuf.read(3u).x
                );
            }
        "#;

        let lib = dev
            .dev
            .newLibraryWithSource_options_error(&NSString::from_str(src), None)
            .expect("compile pooled view test MSL");
        let f_write = lib.newFunctionWithName(&NSString::from_str("write_pooled")).unwrap();
        let f_read = lib.newFunctionWithName(&NSString::from_str("read_pooled")).unwrap();
        let p_write = dev.dev.newComputePipelineStateWithFunction_error(&f_write).unwrap();
        let p_read = dev.dev.newComputePipelineStateWithFunction_error(&f_read).unwrap();

        dev.commit_residency();
        let alloc = dev.dev.newCommandAllocator().unwrap();
        let cb = dev.dev.newCommandBuffer().unwrap();
        cb.beginCommandBufferWithAllocator(&alloc);

        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxBufferBindCount(2);
        let at = dev.dev.newArgumentTableWithDescriptor_error(&atd).unwrap();

        let enc = cb.computeCommandEncoder().unwrap();
        enc.setComputePipelineState(&p_write);
        unsafe { at.setAddress_atIndex(heap.gpu_base, 0) };
        enc.setArgumentTable(Some(&at));
        enc.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: 1, height: 1, depth: 1 },
            MTLSize { width: 1, height: 1, depth: 1 },
        );
        enc.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
            MTLStages::Dispatch,
            MTLStages::Dispatch,
            MTL4VisibilityOptions::Device,
        );
        enc.setComputePipelineState(&p_read);
        unsafe {
            at.setAddress_atIndex(heap.gpu_base, 0);
            at.setAddress_atIndex(out_buf.gpu_va, 1);
        }
        enc.setArgumentTable(Some(&at));
        enc.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: 1, height: 1, depth: 1 },
            MTLSize { width: 1, height: 1, depth: 1 },
        );
        enc.endEncoding();
        cb.endCommandBuffer();

        q.submit(&[&*cb]);
        q.wait_idle();

        let (color, values) = unsafe {
            let p = out_buf.cpu_ptr().unwrap();
            let c = std::ptr::read_unaligned(p as *const [f32; 4]);
            let v = std::ptr::read_unaligned(p.add(16) as *const [u32; 4]);
            (c, v)
        };
        // Swizzle swapped R (1.0) and B (0.25) -> expect (0.25, 0.5, 1.0, 1.0).
        assert!((color[0] - 0.25).abs() < 0.01, "swizzled R mismatch: {color:?}");
        assert!((color[1] - 0.50).abs() < 0.01, "swizzled G mismatch: {color:?}");
        assert!((color[2] - 1.00).abs() < 0.01, "swizzled B mismatch: {color:?}");
        assert!((color[3] - 1.00).abs() < 0.01, "swizzled A mismatch: {color:?}");
        assert_eq!(values, expected_u32s, "pooled typed-buffer SRV values mismatch");

        // Verify immediate recycle once the queue is already idle.
        let before_drop = stats();
        drop(tex);
        drop(tbuf);
        queue::collect_all();
        assert_eq!(stats().active_views, before_drop.active_views - 3);

        // Verify deferred recycle while GPU work is genuinely in-flight: gate the queue on a
        // fence so `timeline.completed() < last_submitted()` when the resource drops, confirm
        // the slot stays pinned, then release the fence and confirm recycling after completion.
        let gate = crate::metal::fence::Fence::new(0).unwrap();
        let gated_tex = Resource::new_committed_default(
            &tex_desc_2d_array(4, 4, 2, 1, 0),
            &hp_def,
            "vp_gated_tex",
        )
        .unwrap();
        let mut gated_srv = [0u8; 40];
        gated_srv[0..4].copy_from_slice(&28u32.to_le_bytes());
        gated_srv[4..8].copy_from_slice(&4u32.to_le_bytes()); // TEXTURE2D
        gated_srv[8..12].copy_from_slice(&0x1688u32.to_le_bytes());
        gated_srv[16..20].copy_from_slice(&1u32.to_le_bytes()); // MostDetailedMip = 1
        gated_srv[20..24].copy_from_slice(&1u32.to_le_bytes()); // MipLevels = 1
        let _ = views::srv_entry(Some(&gated_tex), Some(&gated_srv)).unwrap();
        let pinned_before = stats().active_views;

        q.wait(&gate, 1);
        alloc.reset();
        cb.beginCommandBufferWithAllocator(&alloc);
        cb.endCommandBuffer();
        q.submit(&[&*cb]);

        drop(gated_tex);
        queue::collect_all();
        assert_eq!(
            stats().active_views,
            pinned_before,
            "pooled slot must stay pinned while in-flight GPU work is gated"
        );

        gate.signal_cpu(1);
        q.wait_idle();
        queue::collect_all();
        assert_eq!(
            stats().active_views,
            pinned_before - 1,
            "pooled slot must recycle after gated GPU work completes"
        );
    }
}
