//! `ID3D11Device` (43 base slots) / `ID3D11Device1` (50 total slots) and
//! `IDXGIDevice1` tear-off + D3D11-aware DXGI adapter/factory bridge.

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList, CopyLoc};
use d3d12_metal::metal::device::Device as MtlDevice;
use d3d12_metal::metal::pipeline::GraphicsPipeline;
use d3d12_metal::metal::query::QueryHeap;
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::metal::views;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};
use dxil_metal::root_signature::RootSignature;
use dxil_metal::InputElement;

use crate::dxgi::swapchain::{FullscreenDesc, SwapChainDesc, SwapChainDesc1};
use crate::types::*;

use super::com::*;
use super::context::{D3d11Context, ImmediateExec};
use super::resources::*;
use super::state::*;
use super::types::*;

/// Exact Windows x64 `ID3D11Device` base vtable length (slots 0..42).
pub const ID3D11_DEVICE_BASE_SLOTS: usize = 43;
/// Exact Windows x64 `ID3D11Device1` vtable length (slots 0..49).
pub const ID3D11_DEVICE1_TOTAL_SLOTS: usize = 50;
/// Exact Windows x64 `IDXGIDevice` base vtable length (slots 0..11).
pub const IDXGI_DEVICE_BASE_SLOTS: usize = 12;
/// Exact Windows x64 `IDXGIDevice1` vtable length (slots 0..13).
pub const IDXGI_DEVICE1_TOTAL_SLOTS: usize = 14;
/// Exact Windows x64 `IDXGIDevice3` vtable length (slots 0..17).
pub const IDXGI_DEVICE3_TOTAL_SLOTS: usize = 18;

const TS_POOL_CHUNK: u32 = 4096;

/// Shared pool of `MTL4CounterHeap` timestamp slots for D3D11 `D3D11_QUERY_TIMESTAMP`.
/// Apple AGX caps total `MTL4CounterHeap` objects per process at ~32, so pooling
/// 4096 queries per heap prevents exhausting the hardware counter-heap limit.
#[derive(Default)]
struct TimestampPool {
    heaps: Vec<Arc<QueryHeap>>,
    free: Vec<(usize, u32)>,
    next_slot: u32,
}

impl TimestampPool {
    fn alloc(&mut self) -> Option<(Arc<QueryHeap>, u32)> {
        if let Some((idx, slot)) = self.free.pop() {
            return Some((self.heaps[idx].clone(), slot));
        }
        if self.heaps.is_empty() || self.next_slot >= TS_POOL_CHUNK {
            let h = QueryHeap::new(d3d12_metal::metal::query::heap_type::TIMESTAMP, TS_POOL_CHUNK).ok()?;
            self.heaps.push(Arc::new(h));
            self.next_slot = 0;
        }
        let slot = self.next_slot;
        self.next_slot += 1;
        Some((self.heaps.last()?.clone(), slot))
    }

    fn free(&mut self, heap: &Arc<QueryHeap>, slot: u32) {
        if let Some(idx) = self.heaps.iter().position(|h| Arc::ptr_eq(h, heap)) {
            self.free.push((idx, slot));
        }
    }
}

/// Shared core behind an `ID3D11Device` and all of its children.
pub struct DeviceCore {
    pub mtl: &'static MtlDevice,
    pub queue: Arc<Queue>,
    pub feature_level: u32,
    pub creation_flags: u32,
    pub exception_mode: AtomicU32,
    pub removed_reason: Mutex<HRESULT>,
    pub gfx_rs: Arc<RootSignature>,
    pub compute_rs: Arc<RootSignature>,
    pub gfx_psos: Mutex<HashMap<GraphicsPsoKey, Arc<GraphicsPipeline>>>,
    /// Shared timestamp query heap pool (avoids AGX's ~32 MTL4CounterHeap ceiling).
    timestamp_pool: Mutex<TimestampPool>,
    /// Immediate Metal 4 command encoder/ring backend.
    pub imm_exec: Mutex<ImmediateExec>,
    /// Non-owning pointer to the primary `Obj<D3d11Device>` while alive (0 once released).
    pub device_com: AtomicUsize,
    /// Non-owning pointer to the immediate `Obj<D3d11Context>` (owned by `D3d11Device`).
    pub imm_ctx_com: AtomicUsize,
    pub gpu_thread_priority: AtomicI32,
    pub max_frame_latency: AtomicU32,
}

impl DeviceCore {
    pub fn new(
        mtl: &'static MtlDevice,
        queue: Arc<Queue>,
        feature_level: u32,
        creation_flags: u32,
    ) -> Result<Arc<Self>, HRESULT> {
        let imm_exec = ImmediateExec::new(queue.clone())
            .map_err(|e| d11_failed!(E_OUTOFMEMORY, "D3D11 immediate exec init: {e}"))?;
        Ok(Arc::new(Self {
            mtl,
            queue,
            feature_level,
            creation_flags,
            exception_mode: AtomicU32::new(0),
            removed_reason: Mutex::new(S_OK),
            gfx_rs: build_graphics_root_signature(),
            compute_rs: build_compute_root_signature(),
            gfx_psos: Mutex::new(HashMap::new()),
            timestamp_pool: Mutex::new(TimestampPool::default()),
            imm_exec: Mutex::new(imm_exec),
            device_com: AtomicUsize::new(0),
            imm_ctx_com: AtomicUsize::new(0),
            gpu_thread_priority: AtomicI32::new(0),
            max_frame_latency: AtomicU32::new(3),
        }))
    }

    /// Submit all recorded immediate commands to the Metal 4 queue.
    pub fn flush_immediate(&self) -> u64 {
        self.imm_exec.lock().unwrap().flush()
    }

    /// Submit all recorded immediate commands and wait for GPU completion.
    pub fn flush_and_wait_idle(&self) {
        self.imm_exec.lock().unwrap().flush();
        self.queue.wait_idle();
    }

    pub fn alloc_timestamp_query_slot(&self) -> Option<(Arc<QueryHeap>, u32)> {
        self.timestamp_pool.lock().unwrap().alloc()
    }

    pub fn free_timestamp_query_slot(&self, heap: &Arc<QueryHeap>, slot: u32) {
        if let Ok(mut pool) = self.timestamp_pool.lock() {
            pool.free(heap, slot);
        }
    }

    /// Upload initial subresource bytes onto the device queue (FIFO-ordered before
    /// any subsequent work on the queue, without stalling the CPU).
    pub unsafe fn upload_initial_buffer(
        &self,
        dst: &Arc<MtlResource>,
        data: *const u8,
        byte_len: usize,
    ) -> Result<(), String> {
        if byte_len == 0 {
            return Ok(());
        }
        if let Some(cpu) = dst.cpu_ptr() {
            std::ptr::copy_nonoverlapping(data, cpu, byte_len);
            return Ok(());
        }
        let up_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: byte_len as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
        let up = Arc::new(
            new_committed_resource(&up_desc, &hp, "d3d11 init buf upload")
                .map_err(|e| format!("{e:?}"))?,
        );
        up.register_ptr();
        let cpu = up.cpu_ptr().ok_or("upload buffer has no cpu_ptr")?;
        std::ptr::copy_nonoverlapping(data, cpu, byte_len);

        let alloc = CommandAllocator::new(ListType::Direct).ok_or("init upload allocator")?;
        let mut cl = CommandList::new(ListType::Direct).ok_or("init upload cmdlist")?;
        cl.begin(&alloc)?;
        cl.copy_buffer(dst, 0, &up, 0, byte_len as u64)?;
        cl.close()?;
        self.queue.submit(&[&cl.cb]);
        self.queue.defer_release(Box::new((alloc, up, dst.clone())));
        Ok(())
    }

    /// Upload initial texture subresources (`pInitialData[0..subresource_count]`).
    pub unsafe fn upload_initial_texture(
        &self,
        dst: &Arc<MtlResource>,
        initial: *const D3D11_SUBRESOURCE_DATA,
    ) -> Result<(), String> {
        let sub_count = dst.desc.subresource_count();
        let (layouts, total) = d3d12_metal::layout::copyable_footprints(&dst.desc, 0, sub_count, 0)
            .ok_or_else(|| format!("copyable_footprints failed for {:?}", dst.desc))?;
        let up_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: total.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
        let up = Arc::new(
            new_committed_resource(&up_desc, &hp, "d3d11 init tex upload")
                .map_err(|e| format!("{e:?}"))?,
        );
        up.register_ptr();
        let dst_base = up.cpu_ptr().ok_or("upload buffer has no cpu_ptr")?;

        let alloc = CommandAllocator::new(ListType::Direct).ok_or("init tex allocator")?;
        let mut cl = CommandList::new(ListType::Direct).ok_or("init tex cmdlist")?;
        cl.begin(&alloc)?;

        for (i, lay) in layouts.iter().enumerate() {
            let sd = std::ptr::read_unaligned(initial.add(i));
            if sd.pSysMem.is_null() {
                continue;
            }
            let fp = lay.footprint;
            let num_rows = lay.num_rows as usize;
            let row_size = lay.row_size as usize;
            let depth = fp.depth.max(1) as usize;
            let dst_row_pitch = fp.row_pitch as usize;
            let dst_slice_pitch = dst_row_pitch * num_rows;
            let src_row_pitch = if sd.SysMemPitch == 0 { row_size } else { sd.SysMemPitch as usize };
            let src_slice_pitch = if sd.SysMemSlicePitch == 0 {
                src_row_pitch * num_rows
            } else {
                sd.SysMemSlicePitch as usize
            };
            let src_base = sd.pSysMem as *const u8;
            let sub_dst = dst_base.add(fp.offset as usize);
            for z in 0..depth {
                for y in 0..num_rows {
                    let s = src_base.add(z * src_slice_pitch + y * src_row_pitch);
                    let d = sub_dst.add(z * dst_slice_pitch + y * dst_row_pitch);
                    std::ptr::copy_nonoverlapping(s, d, row_size);
                }
            }
            cl.copy_texture_region(
                CopyLoc::Subresource(dst, i as u32),
                [0, 0, 0],
                CopyLoc::Footprint(&up, fp),
                None,
            )?;
        }

        cl.close()?;
        self.queue.submit(&[&cl.cb]);
        self.queue.defer_release(Box::new((alloc, up, dst.clone())));
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ID3D11Device / ID3D11Device1 COM wrapper
// ---------------------------------------------------------------------------

pub struct D3d11Device {
    pub core: Arc<DeviceCore>,
    /// Owned COM ref on the immediate context (only `true` for the primary wrapper).
    owns_imm: bool,
}

impl D3d11Device {
    pub unsafe fn create_pair(
        mtl: &'static MtlDevice,
        queue: Arc<Queue>,
        feature_level: u32,
        creation_flags: u32,
    ) -> Result<(*mut Obj<D3d11Device>, *mut Obj<D3d11Context>), HRESULT> {
        let core = DeviceCore::new(mtl, queue, feature_level, creation_flags)?;
        crate::process::register_exit_hook(d3d12_metal::metal::pipeline::flush_pso_cache_if_initialized);
        let imm = D3d11Context::new_immediate(core.clone());
        core.imm_ctx_com.store(imm as usize, Ordering::Release);
        // Device holds 1 ref on `imm`; caller also gets 1 ref if requested.
        let dev = Obj::new(D3d11Device { core: core.clone(), owns_imm: true });
        core.device_com.store(dev as usize, Ordering::Release);
        Ok((dev, imm))
    }

    /// Return an AddRef'd `ID3D11Device*` for `core` (reusing the primary COM
    /// identity while alive).
    pub unsafe fn get_or_wrap(core: &Arc<DeviceCore>) -> *mut Obj<D3d11Device> {
        let p = core.device_com.load(Ordering::Acquire);
        if p != 0 {
            let obj = p as *mut Obj<D3d11Device>;
            // Only AddRef if refcount > 0 (avoid resurrecting during final_release).
            let mut cur = (*obj).refs.load(Ordering::Acquire);
            while cur > 0 {
                match (*obj).refs.compare_exchange_weak(cur, cur + 1, Ordering::AcqRel, Ordering::Acquire) {
                    Ok(_) => return obj,
                    Err(v) => cur = v,
                }
            }
        }
        Obj::new(D3d11Device { core: core.clone(), owns_imm: false })
    }
}

static DEVICE_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11Device {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&DEVICE_CLASS, || {
            let mut s = iunknown_slots::<D3d11Device>();
            s.extend([
                // [3..42]: ID3D11Device
                ("CreateBuffer", Some(create_buffer as *const ())),
                ("CreateTexture1D", Some(create_texture1d as *const ())),
                ("CreateTexture2D", Some(create_texture2d as *const ())),
                ("CreateTexture3D", Some(create_texture3d as *const ())),
                ("CreateShaderResourceView", Some(dev_create_srv as *const ())),
                ("CreateUnorderedAccessView", Some(dev_create_uav as *const ())),
                ("CreateRenderTargetView", Some(dev_create_rtv as *const ())),
                ("CreateDepthStencilView", Some(dev_create_dsv as *const ())),
                ("CreateInputLayout", Some(create_input_layout as *const ())),
                ("CreateVertexShader", Some(create_vertex_shader as *const ())),
                ("CreateGeometryShader", Some(create_geometry_shader as *const ())),
                ("CreateGeometryShaderWithStreamOutput", Some(create_geometry_shader_so as *const ())),
                ("CreatePixelShader", Some(create_pixel_shader as *const ())),
                ("CreateHullShader", Some(create_hull_shader as *const ())),
                ("CreateDomainShader", Some(create_domain_shader as *const ())),
                ("CreateComputeShader", Some(create_compute_shader as *const ())),
                ("CreateClassLinkage", Some(create_class_linkage as *const ())),
                ("CreateBlendState", Some(create_blend_state as *const ())),
                ("CreateDepthStencilState", Some(create_depth_stencil_state as *const ())),
                ("CreateRasterizerState", Some(create_rasterizer_state as *const ())),
                ("CreateSamplerState", Some(create_sampler_state as *const ())),
                ("CreateQuery", Some(create_query as *const ())),
                ("CreatePredicate", Some(create_predicate as *const ())),
                ("CreateCounter", Some(create_counter as *const ())),
                ("CreateDeferredContext", Some(create_deferred_context as *const ())),
                ("OpenSharedResource", Some(open_shared_resource as *const ())),
                ("CheckFormatSupport", Some(check_format_support as *const ())),
                ("CheckMultisampleQualityLevels", Some(check_multisample_quality_levels as *const ())),
                ("CheckCounterInfo", Some(check_counter_info as *const ())),
                ("CheckCounter", Some(check_counter as *const ())),
                ("CheckFeatureSupport", Some(check_feature_support as *const ())),
                ("GetPrivateData", Some(get_private_data::<D3d11Device> as *const ())),
                ("SetPrivateData", Some(set_private_data::<D3d11Device> as *const ())),
                ("SetPrivateDataInterface", Some(set_private_data_interface::<D3d11Device> as *const ())),
                ("GetFeatureLevel", Some(get_feature_level as *const ())),
                ("GetCreationFlags", Some(get_creation_flags as *const ())),
                ("GetDeviceRemovedReason", Some(get_device_removed_reason as *const ())),
                ("GetImmediateContext", Some(get_immediate_context as *const ())),
                ("SetExceptionMode", Some(set_exception_mode as *const ())),
                ("GetExceptionMode", Some(get_exception_mode as *const ())),
                // [43..49]: ID3D11Device1
                ("GetImmediateContext1", Some(get_immediate_context1 as *const ())),
                ("CreateDeferredContext1", Some(create_deferred_context1 as *const ())),
                ("CreateBlendState1", Some(create_blend_state1 as *const ())),
                ("CreateRasterizerState1", Some(create_rasterizer_state1 as *const ())),
                ("CreateDeviceContextState", Some(create_device_context_state as *const ())),
                ("OpenSharedResource1", Some(open_shared_resource1 as *const ())),
                ("OpenSharedResourceByName", Some(open_shared_resource_by_name as *const ())),
            ]);
            debug_assert_eq!(s.len(), ID3D11_DEVICE1_TOTAL_SLOTS);
            Class {
                name: "ID3D11Device1",
                vtbl: build_vtable("ID3D11Device1", &s),
                iids: &[IID_ID3D11Device, IID_ID3D11Device1],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if matches!(
            *iid,
            crate::dxgi::IID_IDXGIObject
                | IID_IDXGIDevice
                | IID_IDXGIDevice1
                | IID_IDXGIDevice2
                | IID_IDXGIDevice3
        ) {
            let core = Obj::get(this).core.clone();
            return Some(Obj::new(DxgiDevice { core }) as *mut c_void);
        }
        None
    }

    fn final_release(&self) {
        if self.owns_imm {
            self.core.device_com.store(0, Ordering::Release);
            let imm = self.core.imm_ctx_com.swap(0, Ordering::AcqRel);
            if imm != 0 {
                unsafe { Obj::release(imm as *mut Obj<D3d11Context>) };
            }
        }
    }
}

pub type PDev = *mut Obj<D3d11Device>;

pub unsafe fn device_core_from_ptr(p: *mut c_void) -> Option<Arc<DeviceCore>> {
    if let Some(d) = Obj::<D3d11Device>::from_iface(p) {
        return Some(Obj::get(d).core.clone());
    }
    if let Some(dx) = Obj::<DxgiDevice>::from_iface(p) {
        return Some(Obj::get(dx).core.clone());
    }
    None
}

// ---------------------------------------------------------------------------
// ID3D11Device / ID3D11Device1 method implementations
// ---------------------------------------------------------------------------

unsafe extern "win64" fn create_buffer(
    this: PDev,
    desc: *const D3D11_BUFFER_DESC,
    initial: *const D3D11_SUBRESOURCE_DATA,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateBuffer: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    if d.Usage == usage::IMMUTABLE && initial.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateBuffer: IMMUTABLE buffer requires pInitialData");
    }
    let core = Obj::get(this).core.clone();
    let res = match ResourceCore::new_buffer(core.clone(), d) {
        Ok(r) => r,
        Err(hr) => return hr,
    };
    if !initial.is_null() {
        let sd = std::ptr::read_unaligned(initial);
        if !sd.pSysMem.is_null() {
            let mtl = res.current_mtl();
            if let Err(e) = core.upload_initial_buffer(&mtl, sd.pSysMem as *const u8, d.ByteWidth as usize) {
                return d11_failed!(E_FAIL, "CreateBuffer initial upload: {e}");
            }
        }
    }
    if out.is_null() {
        return S_FALSE;
    }
    std::ptr::write_unaligned(out, Obj::new(D3d11Buffer { device: core, core: res }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_texture1d(
    this: PDev,
    desc: *const D3D11_TEXTURE1D_DESC,
    initial: *const D3D11_SUBRESOURCE_DATA,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateTexture1D: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    let core = Obj::get(this).core.clone();
    let res = match ResourceCore::new_texture1d(core.clone(), d) {
        Ok(r) => r,
        Err(hr) => return hr,
    };
    if !initial.is_null() {
        let mtl = res.current_mtl();
        if let Err(e) = core.upload_initial_texture(&mtl, initial) {
            return d11_failed!(E_FAIL, "CreateTexture1D initial upload: {e}");
        }
    }
    if out.is_null() {
        return S_FALSE;
    }
    std::ptr::write_unaligned(out, Obj::new(D3d11Texture1D { device: core, core: res }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_texture2d(
    this: PDev,
    desc: *const D3D11_TEXTURE2D_DESC,
    initial: *const D3D11_SUBRESOURCE_DATA,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateTexture2D: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    let core = Obj::get(this).core.clone();
    let res = match ResourceCore::new_texture2d(core.clone(), d) {
        Ok(r) => r,
        Err(hr) => return hr,
    };
    if !initial.is_null() {
        let mtl = res.current_mtl();
        if let Err(e) = core.upload_initial_texture(&mtl, initial) {
            return d11_failed!(E_FAIL, "CreateTexture2D initial upload: {e}");
        }
    }
    if out.is_null() {
        return S_FALSE;
    }
    std::ptr::write_unaligned(out, Obj::new(D3d11Texture2D { device: core, core: res }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_texture3d(
    this: PDev,
    desc: *const D3D11_TEXTURE3D_DESC,
    initial: *const D3D11_SUBRESOURCE_DATA,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateTexture3D: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    let core = Obj::get(this).core.clone();
    let res = match ResourceCore::new_texture3d(core.clone(), d) {
        Ok(r) => r,
        Err(hr) => return hr,
    };
    if !initial.is_null() {
        let mtl = res.current_mtl();
        if let Err(e) = core.upload_initial_texture(&mtl, initial) {
            return d11_failed!(E_FAIL, "CreateTexture3D initial upload: {e}");
        }
    }
    if out.is_null() {
        return S_FALSE;
    }
    std::ptr::write_unaligned(out, Obj::new(D3d11Texture3D { device: core, core: res }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn dev_create_srv(
    _this: PDev,
    res: *mut c_void,
    desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    create_srv(res, desc, out)
}

unsafe extern "win64" fn dev_create_uav(
    _this: PDev,
    res: *mut c_void,
    desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    create_uav(res, desc, out)
}

unsafe extern "win64" fn dev_create_rtv(
    _this: PDev,
    res: *mut c_void,
    desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    create_rtv(res, desc, out)
}

unsafe extern "win64" fn dev_create_dsv(
    _this: PDev,
    res: *mut c_void,
    desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    create_dsv(res, desc, out)
}

unsafe extern "win64" fn create_input_layout(
    this: PDev,
    descs: *const D3D11_INPUT_ELEMENT_DESC,
    num_elements: u32,
    bytecode: *const c_void,
    bytecode_len: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if (num_elements > 0 && descs.is_null()) || bytecode.is_null() || bytecode_len == 0 {
        return d11_failed!(E_INVALIDARG, "CreateInputLayout: invalid args (n={num_elements}, bc={bytecode_len})");
    }
    let mut elements = Vec::with_capacity(num_elements as usize);
    let mut kh = dxil_metal::hash::KeyHasher::new("d3d11 input layout v1");
    for i in 0..num_elements as usize {
        let e = std::ptr::read_unaligned(descs.add(i));
        let sem = if e.SemanticName.is_null() {
            String::new()
        } else {
            String::from_utf8_lossy(crate::strings::cstr_slice(e.SemanticName)).into_owned()
        };
        kh.bytes(sem.as_bytes())
            .u32(e.SemanticIndex)
            .u32(e.Format)
            .u32(e.InputSlot)
            .u32(e.AlignedByteOffset)
            .u32(e.InputSlotClass)
            .u32(e.InstanceDataStepRate);
        elements.push(InputElement {
            semantic_name: sem,
            semantic_index: e.SemanticIndex,
            format: e.Format,
            input_slot: e.InputSlot,
            aligned_byte_offset: e.AlignedByteOffset,
            input_slot_class: e.InputSlotClass,
            instance_data_step_rate: e.InstanceDataStepRate,
        });
    }
    if out.is_null() {
        return S_FALSE;
    }
    let inner = Arc::new(InputLayoutInner {
        device: Obj::get(this).core.clone(),
        elements,
        hash: kh.finish(),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11InputLayout { inner }) as *mut c_void);
    S_OK
}

unsafe fn validate_dxbc(bytecode: *const c_void, len: usize, stage: &str) -> Result<Vec<u8>, HRESULT> {
    if bytecode.is_null() || len < 32 {
        return Err(d11_failed!(E_INVALIDARG, "Create{stage}Shader: invalid bytecode ({len} bytes)"));
    }
    let bytes = std::slice::from_raw_parts(bytecode as *const u8, len);
    if !dxil_metal::dxbc::Container::is_container(bytes) {
        return Err(d11_failed!(E_INVALIDARG, "Create{stage}Shader: not a DXBC container ({len} bytes)"));
    }
    Ok(bytes.to_vec())
}

unsafe extern "win64" fn create_vertex_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Vertex") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let hash = dxil_metal::hash::hash128(&bytes);
    let inner = Arc::new(ShaderBlob { device: Obj::get(this).core.clone(), bytecode: bytes, hash });
    std::ptr::write_unaligned(out, Obj::new(D3d11VertexShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_pixel_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Pixel") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let hash = dxil_metal::hash::hash128(&bytes);
    let inner = Arc::new(ShaderBlob { device: Obj::get(this).core.clone(), bytecode: bytes, hash });
    std::ptr::write_unaligned(out, Obj::new(D3d11PixelShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_compute_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Compute") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let inner = Arc::new(ComputeShaderInner::new(Obj::get(this).core.clone(), bytes));
    std::ptr::write_unaligned(out, Obj::new(D3d11ComputeShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_geometry_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Geometry") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let hash = dxil_metal::hash::hash128(&bytes);
    let inner = Arc::new(ShaderBlob { device: Obj::get(this).core.clone(), bytecode: bytes, hash });
    std::ptr::write_unaligned(out, Obj::new(D3d11GeometryShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_geometry_shader_so(
    _this: PDev, _bc: *const c_void, _len: usize, _so: *const c_void, _n: u32, _strides: *const u32,
    _n_strides: u32, _rast_stream: u32, _linkage: *mut c_void, out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    stub!("ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL");
    E_NOTIMPL
}

unsafe extern "win64" fn create_hull_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Hull") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let hash = dxil_metal::hash::hash128(&bytes);
    let inner = Arc::new(ShaderBlob { device: Obj::get(this).core.clone(), bytecode: bytes, hash });
    std::ptr::write_unaligned(out, Obj::new(D3d11HullShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_domain_shader(
    this: PDev,
    bytecode: *const c_void,
    len: usize,
    _linkage: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    let bytes = match validate_dxbc(bytecode, len, "Domain") {
        Ok(b) => b,
        Err(hr) => return hr,
    };
    if out.is_null() { return S_FALSE; }
    let hash = dxil_metal::hash::hash128(&bytes);
    let inner = Arc::new(ShaderBlob { device: Obj::get(this).core.clone(), bytecode: bytes, hash });
    std::ptr::write_unaligned(out, Obj::new(D3d11DomainShader { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_class_linkage(this: PDev, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if out.is_null() {
        return S_FALSE;
    }
    std::ptr::write_unaligned(out, D3d11ClassLinkage::new_com(Obj::get(this).core.clone()));
    S_OK
}

unsafe extern "win64" fn create_blend_state(
    this: PDev,
    desc: *const D3D11_BLEND_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateBlendState: null desc");
    }
    if out.is_null() { return S_FALSE; }
    let d0 = std::ptr::read_unaligned(desc);
    let inner = Arc::new(BlendStateInner {
        device: Obj::get(this).core.clone(),
        desc0: d0,
        desc1: d0.to_desc1(),
        d3d12: d0.to_d3d12(),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11BlendState { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_blend_state1(
    this: PDev,
    desc: *const D3D11_BLEND_DESC1,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateBlendState1: null desc");
    }
    if out.is_null() { return S_FALSE; }
    let d1 = std::ptr::read_unaligned(desc);
    let inner = Arc::new(BlendStateInner {
        device: Obj::get(this).core.clone(),
        desc0: d1.to_desc0(),
        desc1: d1,
        d3d12: d1.to_d3d12(),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11BlendState { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_depth_stencil_state(
    this: PDev,
    desc: *const D3D11_DEPTH_STENCIL_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateDepthStencilState: null desc");
    }
    if out.is_null() { return S_FALSE; }
    let d = std::ptr::read_unaligned(desc);
    let mtl = d3d12_metal::state::depth_stencil_state(&d.to_d3d12());
    let inner = Arc::new(DepthStencilStateInner { device: Obj::get(this).core.clone(), desc: d, mtl });
    std::ptr::write_unaligned(out, Obj::new(D3d11DepthStencilState { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_rasterizer_state(
    this: PDev,
    desc: *const D3D11_RASTERIZER_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateRasterizerState: null desc");
    }
    if out.is_null() { return S_FALSE; }
    let d0 = std::ptr::read_unaligned(desc);
    let d1 = d0.to_desc1();
    let mtl = d3d12_metal::state::raster_state(&d1.to_d3d12());
    let inner = Arc::new(RasterizerStateInner {
        device: Obj::get(this).core.clone(),
        desc0: d0,
        desc1: d1,
        scissor_enable: d0.ScissorEnable != 0,
        mtl,
        hash: hash_raster_desc(&d1),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11RasterizerState { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_rasterizer_state1(
    this: PDev,
    desc: *const D3D11_RASTERIZER_DESC1,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateRasterizerState1: null desc");
    }
    if out.is_null() { return S_FALSE; }
    let d1 = std::ptr::read_unaligned(desc);
    let mtl = d3d12_metal::state::raster_state(&d1.to_d3d12());
    let inner = Arc::new(RasterizerStateInner {
        device: Obj::get(this).core.clone(),
        desc0: d1.to_desc0(),
        desc1: d1,
        scissor_enable: d1.ScissorEnable != 0,
        mtl,
        hash: hash_raster_desc(&d1),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11RasterizerState { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_sampler_state(
    this: PDev,
    desc: *const D3D11_SAMPLER_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateSamplerState: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    let entry = match views::sampler_entry(&d.to_d3d12()) {
        Ok(e) => e,
        Err(e) => return d11_failed!(E_INVALIDARG, "CreateSamplerState({d:?}): {e}"),
    };
    if out.is_null() { return S_FALSE; }
    let inner = Arc::new(SamplerStateInner { device: Obj::get(this).core.clone(), desc: d, entry });
    std::ptr::write_unaligned(out, Obj::new(D3d11SamplerState { inner }) as *mut c_void);
    S_OK
}

unsafe fn create_query_inner(
    this: PDev,
    desc: *const D3D11_QUERY_DESC,
    is_predicate: bool,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if desc.is_null() {
        return d11_failed!(E_INVALIDARG, "CreateQuery: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    if out.is_null() { return S_FALSE; }
    let (heap, slot, readback) = match d.Query {
        query_kind::TIMESTAMP => {
            let (h, slot) = match Obj::get(this).core.alloc_timestamp_query_slot() {
                Some((h, s)) => (Some(h), s),
                None => (None, 0),
            };
            let rb = make_query_readback_buf();
            (h, slot, rb)
        }
        query_kind::OCCLUSION | query_kind::OCCLUSION_PREDICATE => {
            let h = d3d12_metal::metal::query::QueryHeap::new(
                d3d12_metal::metal::query::heap_type::OCCLUSION,
                1,
            )
            .ok()
            .map(Arc::new);
            let rb = make_query_readback_buf();
            (h, 0, rb)
        }
        _ => (None, 0, None),
    };
    let inner = Arc::new(QueryInner {
        device: Obj::get(this).core.clone(),
        desc: d,
        is_predicate,
        rt: Mutex::new(QueryRuntime {
            ended: false,
            submitted: false,
            timeline_value: 0,
            heap,
            slot,
            readback,
            pipeline_stats: D3D11_QUERY_DATA_PIPELINE_STATISTICS::default(),
        }),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11Query { inner }) as *mut c_void);
    S_OK
}

fn make_query_readback_buf() -> Option<Arc<MtlResource>> {
    let rdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 16,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let r = Arc::new(new_committed_resource(&rdesc, &hp, "d3d11 query rb").ok()?);
    r.register_ptr();
    Some(r)
}

unsafe extern "win64" fn create_query(this: PDev, desc: *const D3D11_QUERY_DESC, out: *mut *mut c_void) -> HRESULT {
    create_query_inner(this, desc, false, out)
}

unsafe extern "win64" fn create_predicate(this: PDev, desc: *const D3D11_QUERY_DESC, out: *mut *mut c_void) -> HRESULT {
    create_query_inner(this, desc, true, out)
}

/// Native-unsupported: Apple Silicon Metal exposes only GPU timestamp counters
/// (`MTLDevice.counterSets == ["timestamp"]`, `MTL4CounterHeapTypeTimestamp`) and
/// zero device-dependent performance counters (`CheckCounterInfo` reports
/// `LastDeviceDependentCounter = 0`, `NumSimultaneousCounters = 0`).
/// Matches Windows D3D11 drivers with no device-dependent counters by validating
/// `pCounterDesc` (`E_INVALIDARG` on null) and returning `DXGI_ERROR_UNSUPPORTED`
/// (Microsoft Learn: `ID3D11Device::CreateCounter`).
unsafe extern "win64" fn create_counter(
    _this: PDev,
    desc: *const D3D11_COUNTER_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if desc.is_null() {
        return E_INVALIDARG;
    }
    DXGI_ERROR_UNSUPPORTED
}

unsafe extern "win64" fn create_deferred_context(this: PDev, flags: u32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let ctx = D3d11Context::new_deferred(Obj::get(this).core.clone(), flags);
    std::ptr::write_unaligned(out, ctx as *mut c_void);
    S_OK
}

unsafe extern "win64" fn create_deferred_context1(this: PDev, flags: u32, out: *mut *mut c_void) -> HRESULT {
    create_deferred_context(this, flags, out)
}

unsafe extern "win64" fn open_shared_resource(
    this: PDev,
    h: HANDLE,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let res_core = match lookup_legacy_shared_resource(h) {
        Ok(c) => c,
        Err(hr) => return d11_failed!(hr, "ID3D11Device::OpenSharedResource({h:?})"),
    };
    wrap_shared_resource(Obj::get(this).core.clone(), res_core, riid, out)
}

unsafe extern "win64" fn open_shared_resource1(
    this: PDev,
    h: HANDLE,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let res_core = match lookup_nt_shared_resource(h) {
        Ok(c) => c,
        Err(hr) => return d11_failed!(hr, "ID3D11Device1::OpenSharedResource1({h:?})"),
    };
    wrap_shared_resource(Obj::get(this).core.clone(), res_core, riid, out)
}

unsafe extern "win64" fn open_shared_resource_by_name(
    this: PDev,
    name_w: LPCWSTR,
    access: u32,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some(name) = read_wide_unaligned(name_w) else {
        return d11_failed!(E_INVALIDARG, "ID3D11Device1::OpenSharedResourceByName: null lpName");
    };
    let res_core = match lookup_named_shared_resource(&name, access) {
        Ok(c) => c,
        Err(hr) => {
            return d11_failed!(
                hr,
                "ID3D11Device1::OpenSharedResourceByName({name:?}, {access:#x})"
            )
        }
    };
    wrap_shared_resource(Obj::get(this).core.clone(), res_core, riid, out)
}

unsafe extern "win64" fn create_device_context_state(
    this: PDev,
    _flags: u32,
    levels: *const u32,
    n: u32,
    sdk: u32,
    iface: *const Guid,
    chosen: *mut u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if sdk != D3D11_SDK_VERSION || (n > 0 && levels.is_null()) || iface.is_null() {
        return E_INVALIDARG;
    }
    let Some(emulated) = Guid::read(iface) else {
        return E_INVALIDARG;
    };
    if !matches!(
        emulated,
        IID_ID3D11Device
            | IID_ID3D11Device1
            | crate::dxgi::adapter::IID_ID3D10Device
            | IID_ID3D10Device1
    ) {
        return E_INVALIDARG;
    }
    let core = &Obj::get(this).core;
    let selected_level = if n == 0 {
        core.feature_level
    } else {
        let mut found = None;
        for i in 0..n as usize {
            let fl = std::ptr::read_unaligned(levels.add(i));
            if fl <= core.feature_level
                && matches!(
                    fl,
                    D3D_FEATURE_LEVEL_9_1
                        | D3D_FEATURE_LEVEL_9_2
                        | D3D_FEATURE_LEVEL_9_3
                        | D3D_FEATURE_LEVEL_10_0
                        | D3D_FEATURE_LEVEL_10_1
                        | D3D_FEATURE_LEVEL_11_0
                        | D3D_FEATURE_LEVEL_11_1
                )
            {
                found = Some(fl);
                break;
            }
        }
        let Some(fl) = found else {
            return E_INVALIDARG;
        };
        fl
    };
    if !chosen.is_null() {
        std::ptr::write_unaligned(chosen, selected_level);
    }
    if out.is_null() {
        return S_FALSE;
    }
    let st = D3d11DeviceContextState::new_com(
        core.clone(),
        selected_level,
        super::context::ContextState::default(),
    );
    std::ptr::write_unaligned(out, st as *mut c_void);
    S_OK
}

unsafe extern "win64" fn check_format_support(_this: PDev, format: u32, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let (s1, _) = d3d12_metal::format::support(format);
    std::ptr::write_unaligned(out, s1 | if s1 != 0 { 0x20000 } else { 0 }); // + D3D11_FORMAT_SUPPORT_CPU_LOCKABLE
    if s1 == 0 { E_FAIL } else { S_OK }
}

unsafe extern "win64" fn check_multisample_quality_levels(
    _this: PDev,
    format: u32,
    samples: u32,
    out: *mut u32,
) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, d3d12_metal::format::msaa_quality_levels(format, samples));
    S_OK
}

unsafe extern "win64" fn check_counter_info(_this: PDev, out: *mut D3D11_COUNTER_INFO) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, D3D11_COUNTER_INFO::default());
    }
}

/// Native-unsupported: Apple Silicon Metal exposes zero device-dependent performance
/// counters (`CheckCounterInfo` reports `LastDeviceDependentCounter = 0`). Matches
/// Windows D3D11 drivers without device-dependent counters by validating `pDesc`
/// (`E_INVALIDARG` on null) and returning `DXGI_ERROR_UNSUPPORTED`
/// (Microsoft Learn: `ID3D11Device::CheckCounter`).
unsafe extern "win64" fn check_counter(
    _this: PDev,
    desc: *const D3D11_COUNTER_DESC,
    _ty: *mut u32,
    _active: *mut u32,
    _name: *mut u8,
    _name_len: *mut u32,
    _units: *mut u8,
    _units_len: *mut u32,
    _text: *mut u8,
    _text_len: *mut u32,
) -> HRESULT {
    if desc.is_null() {
        return E_INVALIDARG;
    }
    DXGI_ERROR_UNSUPPORTED
}

unsafe extern "win64" fn check_feature_support(_this: PDev, feat: u32, data: *mut c_void, size: u32) -> HRESULT {
    if data.is_null() {
        return E_INVALIDARG;
    }
    let buf = std::slice::from_raw_parts_mut(data as *mut u8, size as usize);
    let put = |b: &mut [u8], vals: &[u32]| {
        for (i, v) in vals.iter().enumerate() {
            b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    };
    let rd = |b: &[u8], i: usize| u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
    match feat {
        feature::THREADING => {
            if size != 8 { return E_INVALIDARG; }
            put(buf, &[1, 1]); // DriverConcurrentCreates, DriverCommandLists
            S_OK
        }
        feature::DOUBLES => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[0]); // Metal has no fp64
            S_OK
        }
        feature::FORMAT_SUPPORT => {
            if size != 8 { return E_INVALIDARG; }
            let (s1, _) = d3d12_metal::format::support(rd(buf, 0));
            put(&mut buf[4..], &[s1 | if s1 != 0 { 0x20000 } else { 0 }]);
            S_OK
        }
        feature::FORMAT_SUPPORT2 => {
            if size != 8 { return E_INVALIDARG; }
            let (_, s2) = d3d12_metal::format::support(rd(buf, 0));
            put(&mut buf[4..], &[s2]);
            S_OK
        }
        feature::D3D10_X_HARDWARE_OPTIONS => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[1]); // ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x
            S_OK
        }
        feature::D3D11_OPTIONS => {
            if size != 56 { return E_INVALIDARG; }
            // OutputMergerLogicOp=0, UAVOnlyForcedSampleCount=0, DiscardAPIs=1, FlagsForUpdateCopy=1,
            // ClearView=1, CopyWithOverlap=1, ConstantBufferPartialUpdate=1, ConstantBufferOffsetting=1,
            // MapNoOverwriteDynamicCB=1, MapNoOverwriteDynamicSRV=1, MSRTVForcedSampleOne=0,
            // SAD4=0, ExtendedDoubles=0, ExtendedResourceSharing=0
            put(buf, &[0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]);
            S_OK
        }
        feature::ARCHITECTURE_INFO => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[1]); // TileBasedDeferredRenderer = TRUE (Apple Silicon TBDR)
            S_OK
        }
        feature::D3D9_OPTIONS | feature::D3D9_SHADOW_SUPPORT | feature::D3D9_SIMPLE_INSTANCING_SUPPORT => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[1]);
            S_OK
        }
        feature::SHADER_MIN_PRECISION_SUPPORT => {
            if size != 8 { return E_INVALIDARG; }
            put(buf, &[2, 2]); // 16-bit
            S_OK
        }
        feature::D3D11_OPTIONS1 => {
            if size != 16 { return E_INVALIDARG; }
            put(buf, &[0, 1, 1, 0]); // TiledResourcesTier=0, MinMaxFiltering=1, ClearViewDepthOnly=1, MapOnDefaultBuffers=0
            S_OK
        }
        feature::MARKER_SUPPORT => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[0]);
            S_OK
        }
        feature::D3D9_OPTIONS1 => {
            if size != 16 { return E_INVALIDARG; }
            put(buf, &[1, 1, 1, 1]);
            S_OK
        }
        feature::D3D11_OPTIONS2 => {
            if size != 32 { return E_INVALIDARG; }
            // PSSpecifiedStencilRef=0, TypedUAVLoadAdditionalFormats=1, ROVs=0,
            // ConservativeRasterTier=0, TiledTier=0, MapOnDefaultTex=0, StdSwizzle=0, UMA=1
            put(buf, &[0, 1, 0, 0, 0, 0, 0, 1]);
            S_OK
        }
        feature::D3D11_OPTIONS3 => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[1]); // VPAndRTArrayIndexFromAnyShaderFeedingRasterizer
            S_OK
        }
        feature::GPU_VIRTUAL_ADDRESS_SUPPORT => {
            if size != 8 { return E_INVALIDARG; }
            put(buf, &[40, 40]);
            S_OK
        }
        feature::D3D11_OPTIONS4 => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[0]);
            S_OK
        }
        feature::SHADER_CACHE => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[0]);
            S_OK
        }
        feature::D3D11_OPTIONS5 => {
            if size != 4 { return E_INVALIDARG; }
            put(buf, &[0]);
            S_OK
        }
        _ => d11_failed!(E_INVALIDARG, "ID3D11Device::CheckFeatureSupport({feat}, size {size})"),
    }
}

unsafe extern "win64" fn get_feature_level(this: PDev) -> u32 {
    Obj::get(this).core.feature_level
}

unsafe extern "win64" fn get_creation_flags(this: PDev) -> u32 {
    Obj::get(this).core.creation_flags
}

unsafe extern "win64" fn get_device_removed_reason(this: PDev) -> HRESULT {
    *Obj::get(this).core.removed_reason.lock().unwrap()
}

unsafe extern "win64" fn get_immediate_context(this: PDev, out: *mut *mut c_void) {
    if out.is_null() {
        return;
    }
    let core = &Obj::get(this).core;
    let p = core.imm_ctx_com.load(Ordering::Acquire);
    if p != 0 {
        com_add_ref(p as *mut c_void);
        std::ptr::write_unaligned(out, p as *mut c_void);
    } else {
        std::ptr::write_unaligned(out, D3d11Context::new_immediate(core.clone()) as *mut c_void);
    }
}

unsafe extern "win64" fn get_immediate_context1(this: PDev, out: *mut *mut c_void) {
    get_immediate_context(this, out);
}

unsafe extern "win64" fn set_exception_mode(this: PDev, flags: u32) -> HRESULT {
    Obj::get(this).core.exception_mode.store(flags, Ordering::Relaxed);
    S_OK
}

unsafe extern "win64" fn get_exception_mode(this: PDev) -> u32 {
    Obj::get(this).core.exception_mode.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Tear-off IDXGIDevice1 + D3D11-capable DXGI adapter/factory bridge
// ---------------------------------------------------------------------------

pub struct DxgiDevice {
    pub core: Arc<DeviceCore>,
}

static DXGI_DEV_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for DxgiDevice {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&DXGI_DEV_CLASS, || {
            let mut s = dxgi_object_slots::<DxgiDevice>();
            s.extend([
                ("GetParent", Some(dxgi_dev_get_parent as *const ())),
                ("GetAdapter", Some(dxgi_dev_get_adapter as *const ())),
                ("CreateSurface", Some(dxgi_dev_create_surface as *const ())),
                ("QueryResourceResidency", Some(dxgi_dev_query_residency as *const ())),
                ("SetGPUThreadPriority", Some(dxgi_dev_set_prio as *const ())),
                ("GetGPUThreadPriority", Some(dxgi_dev_get_prio as *const ())),
                // IDXGIDevice1
                ("SetMaximumFrameLatency", Some(dxgi_dev_set_latency as *const ())),
                ("GetMaximumFrameLatency", Some(dxgi_dev_get_latency as *const ())),
                // IDXGIDevice2
                ("OfferResources", Some(dxgi_dev_offer_resources as *const ())),
                ("ReclaimResources", Some(dxgi_dev_reclaim_resources as *const ())),
                ("EnqueueSetEvent", Some(dxgi_dev_enqueue_set_event as *const ())),
                // IDXGIDevice3
                ("Trim", Some(dxgi_dev_trim as *const ())),
            ]);
            debug_assert_eq!(s.len(), IDXGI_DEVICE3_TOTAL_SLOTS);
            Class {
                name: "IDXGIDevice3",
                vtbl: build_vtable("IDXGIDevice3", &s),
                iids: &[
                    crate::dxgi::IID_IDXGIObject,
                    IID_IDXGIDevice,
                    IID_IDXGIDevice1,
                    IID_IDXGIDevice2,
                    IID_IDXGIDevice3,
                ],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if *iid == IID_ID3D11Device || *iid == IID_ID3D11Device1 {
            return Some(D3d11Device::get_or_wrap(&Obj::get(this).core) as *mut c_void);
        }
        None
    }
}

type PGD = *mut Obj<DxgiDevice>;

unsafe extern "win64" fn dxgi_dev_get_parent(_this: PGD, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let a = new_d3d11_adapter();
    let hr = query_out(a, riid, out);
    com_release(a);
    hr
}

unsafe extern "win64" fn dxgi_dev_get_adapter(_this: PGD, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, new_d3d11_adapter());
    S_OK
}

unsafe extern "win64" fn dxgi_dev_create_surface(
    this: PGD,
    desc: *const DXGI_SURFACE_DESC,
    num_surfaces: u32,
    dxgi_usage: u32,
    shared: *const HANDLE,
    out: *mut *mut c_void,
) -> HRESULT {
    if desc.is_null() || num_surfaces == 0 || out.is_null() {
        return E_INVALIDARG;
    }
    for i in 0..num_surfaces as usize {
        std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
    }
    let sd = std::ptr::read_unaligned(desc);
    if sd.Width == 0 || sd.Height == 0 {
        return E_INVALIDARG;
    }
    let core = &Obj::get(this).core;

    // If a shared handle is provided, open it as an IDXGISurface (valid for num_surfaces == 1).
    if !shared.is_null() {
        let h = std::ptr::read_unaligned(shared);
        if !h.is_null() {
            if num_surfaces != 1 {
                return E_INVALIDARG;
            }
            let res_core = match lookup_legacy_shared_resource(h)
                .or_else(|_| lookup_nt_shared_resource(h))
            {
                Ok(c) => c,
                Err(hr) => return hr,
            };
            return wrap_shared_resource(core.clone(), res_core, &IID_IDXGISurface, out);
        }
    }

    // Translate DXGI_USAGE flags into D3D11_TEXTURE2D_DESC fields.
    let cpu_bits = dxgi_usage & 0xF;
    let (d3d_usage, cpu_access) = match cpu_bits {
        1 => (usage::DYNAMIC, cpu_access::WRITE),
        2 | 3 => (usage::STAGING, cpu_access::READ | cpu_access::WRITE),
        _ => (usage::DEFAULT, 0),
    };
    let mut bind_flags = 0u32;
    if dxgi_usage & 0x10 != 0 {
        bind_flags |= bind_flag::SHADER_RESOURCE;
    }
    if dxgi_usage & 0x20 != 0 {
        bind_flags |= bind_flag::RENDER_TARGET;
    }
    if dxgi_usage & 0x400 != 0 {
        bind_flags |= bind_flag::UNORDERED_ACCESS;
    }
    if d3d_usage == usage::STAGING {
        bind_flags = 0;
    }
    let mut misc_flags = 0u32;
    if dxgi_usage & 0x80 != 0 {
        misc_flags |= misc_flag::SHARED;
    }

    let td = D3D11_TEXTURE2D_DESC {
        Width: sd.Width,
        Height: sd.Height,
        MipLevels: 1,
        ArraySize: 1,
        Format: sd.Format,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: sd.SampleDesc.Count.max(1),
            Quality: sd.SampleDesc.Quality,
        },
        Usage: d3d_usage,
        BindFlags: bind_flags,
        CPUAccessFlags: cpu_access,
        MiscFlags: misc_flags,
    };

    let mut created: Vec<*mut c_void> = Vec::with_capacity(num_surfaces as usize);
    for _ in 0..num_surfaces {
        let res = match ResourceCore::new_texture2d(core.clone(), td) {
            Ok(r) => r,
            Err(hr) => {
                for p in created {
                    com_release(p);
                }
                return hr;
            }
        };
        let tex = Obj::new(D3d11Texture2D { device: core.clone(), core: res.clone() });
        // `DxgiSurface` takes ownership of `tex`'s initial refcount (drops via `com_release`).
        let surf = DxgiSurface::new_com(tex as usize, core.clone(), res, 0);
        created.push(surf);
    }
    for (i, p) in created.into_iter().enumerate() {
        std::ptr::write_unaligned(out.add(i), p);
    }
    S_OK
}

unsafe extern "win64" fn dxgi_dev_query_residency(
    _this: PGD, _res: *const *mut c_void, status: *mut u32, n: u32,
) -> HRESULT {
    if !status.is_null() {
        for i in 0..n as usize {
            std::ptr::write_unaligned(status.add(i), 1); // DXGI_RESIDENCY_FULLY_RESIDENT
        }
    }
    S_OK
}

unsafe extern "win64" fn dxgi_dev_set_prio(this: PGD, prio: i32) -> HRESULT {
    if !(-7..=7).contains(&prio) {
        return E_INVALIDARG;
    }
    Obj::get(this).core.gpu_thread_priority.store(prio, Ordering::Relaxed);
    S_OK
}

unsafe extern "win64" fn dxgi_dev_get_prio(this: PGD, out: *mut i32) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).core.gpu_thread_priority.load(Ordering::Relaxed));
    S_OK
}

unsafe extern "win64" fn dxgi_dev_set_latency(this: PGD, max_latency: u32) -> HRESULT {
    let v = if max_latency == 0 { 3 } else if max_latency <= 16 { max_latency } else { return E_INVALIDARG };
    Obj::get(this).core.max_frame_latency.store(v, Ordering::Relaxed);
    S_OK
}

unsafe extern "win64" fn dxgi_dev_get_latency(this: PGD, out: *mut u32) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).core.max_frame_latency.load(Ordering::Relaxed));
    S_OK
}

unsafe extern "win64" fn dxgi_dev_offer_resources(
    _this: PGD,
    _num: u32,
    _resources: *const *mut c_void,
    _priority: u32,
) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn dxgi_dev_reclaim_resources(
    _this: PGD,
    num: u32,
    _resources: *const *mut c_void,
    discarded: *mut BOOL,
) -> HRESULT {
    if !discarded.is_null() {
        for i in 0..num as usize {
            std::ptr::write_unaligned(discarded.add(i), FALSE);
        }
    }
    S_OK
}

unsafe extern "win64" fn dxgi_dev_enqueue_set_event(this: PGD, event: HANDLE) -> HRESULT {
    if event.is_null() {
        return E_INVALIDARG;
    }
    let Some(ev) = crate::handles::get_as::<crate::handles::Event>(event) else {
        return d11_failed!(E_INVALIDARG, "IDXGIDevice2::EnqueueSetEvent({event:?}): not an event handle");
    };
    let core = &Obj::get(this).core;
    let val = core.flush_immediate();
    core.queue.timeline.on_completion(val, Box::new(move || ev.set()));
    S_OK
}

unsafe extern "win64" fn dxgi_dev_trim(this: PGD) {
    Obj::get(this).core.flush_immediate();
}

// ---------------------------------------------------------------------------
// D3D11-aware DXGI Adapter & Factory wrappers (enables the standard
// `device->QI(IDXGIDevice)->GetAdapter()->GetParent(IDXGIFactory)->CreateSwapChain(device)`
// sequence without modifying `crate::dxgi`).
// ---------------------------------------------------------------------------

struct D3d11DxgiAdapter {
    inner_adapter: usize,
    factory: usize,
}

impl Drop for D3d11DxgiAdapter {
    fn drop(&mut self) {
        unsafe {
            com_release(self.inner_adapter as *mut c_void);
            com_release(self.factory as *mut c_void);
        }
    }
}

static D11_ADAPTER_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11DxgiAdapter {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        use crate::dxgi::adapter::*;
        class_once(&D11_ADAPTER_CLASS, || {
            let mut s = dxgi_object_slots::<D3d11DxgiAdapter>();
            s.extend([
                ("GetParent", Some(adp_get_parent as *const ())),
                ("EnumOutputs", Some(adp_enum_outputs as *const ())),
                ("GetDesc", Some(adp_get_desc as *const ())),
                ("CheckInterfaceSupport", Some(adp_check_iface as *const ())),
                ("GetDesc1", Some(adp_get_desc1 as *const ())),
                ("GetDesc2", Some(adp_get_desc2 as *const ())),
                ("RegisterHardwareContentProtectionTeardownStatusEvent", Some(adp_reg_event as *const ())),
                ("UnregisterHardwareContentProtectionTeardownStatus", Some(adp_unreg as *const ())),
                ("QueryVideoMemoryInfo", Some(adp_query_vmem as *const ())),
                ("SetVideoMemoryReservation", Some(adp_set_vmem as *const ())),
                ("RegisterVideoMemoryBudgetChangeNotificationEvent", Some(adp_reg_event as *const ())),
                ("UnregisterVideoMemoryBudgetChangeNotification", Some(adp_unreg as *const ())),
                ("GetDesc3", Some(adp_get_desc3 as *const ())),
            ]);
            Class {
                name: "IDXGIAdapter4(D3D11)",
                vtbl: build_vtable("IDXGIAdapter4(D3D11)", &s),
                iids: &[
                    crate::dxgi::IID_IDXGIObject,
                    IID_IDXGIAdapter,
                    IID_IDXGIAdapter1,
                    IID_IDXGIAdapter2,
                    IID_IDXGIAdapter3,
                    IID_IDXGIAdapter4,
                ],
            }
        })
    }
}

unsafe fn new_d3d11_adapter() -> *mut c_void {
    let mut base_f: *mut c_void = std::ptr::null_mut();
    let _ = crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut base_f);
    let inner_adapter = if !base_f.is_null() {
        let a = crate::dxgi::adapter::new_adapter(base_f as *mut Obj<crate::dxgi::Factory>) as *mut c_void;
        com_release(base_f);
        a
    } else {
        std::ptr::null_mut()
    };
    let factory = new_d3d11_factory();
    Obj::new(D3d11DxgiAdapter { inner_adapter: inner_adapter as usize, factory: factory as usize }) as *mut c_void
}

unsafe fn adp_call_slot<F>(this: *mut Obj<D3d11DxgiAdapter>, slot: usize) -> Option<(*mut c_void, F)> {
    let p = Obj::get(this).inner_adapter as *mut c_void;
    if p.is_null() { return None; }
    let vt = *(p as *const *const usize);
    Some((p, std::mem::transmute_copy(&*vt.add(slot))))
}

unsafe extern "win64" fn adp_get_parent(this: *mut Obj<D3d11DxgiAdapter>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).factory as *mut c_void, riid, out)
}
unsafe extern "win64" fn adp_enum_outputs(this: *mut Obj<D3d11DxgiAdapter>, i: u32, out: *mut *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 7).map_or(DXGI_ERROR_NOT_FOUND, |(p, f)| f(p, i, out))
}
unsafe extern "win64" fn adp_get_desc(this: *mut Obj<D3d11DxgiAdapter>, out: *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 8).map_or(E_FAIL, |(p, f)| f(p, out))
}
unsafe extern "win64" fn adp_check_iface(_this: *mut Obj<D3d11DxgiAdapter>, iid: *const Guid, umd: *mut i64) -> HRESULT {
    crate::dxgi::adapter::check_interface_support_impl(iid, umd)
}
unsafe extern "win64" fn adp_get_desc1(this: *mut Obj<D3d11DxgiAdapter>, out: *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 10).map_or(E_FAIL, |(p, f)| f(p, out))
}
unsafe extern "win64" fn adp_get_desc2(this: *mut Obj<D3d11DxgiAdapter>, out: *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 11).map_or(E_FAIL, |(p, f)| f(p, out))
}
unsafe extern "win64" fn adp_reg_event(_this: *mut Obj<D3d11DxgiAdapter>, _ev: HANDLE, cookie: *mut u32) -> HRESULT {
    if !cookie.is_null() { std::ptr::write_unaligned(cookie, 1); }
    S_OK
}
unsafe extern "win64" fn adp_unreg(_this: *mut Obj<D3d11DxgiAdapter>, _cookie: u32) {}
unsafe extern "win64" fn adp_query_vmem(this: *mut Obj<D3d11DxgiAdapter>, node: u32, group: u32, out: *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, u32, u32, *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 14).map_or(E_FAIL, |(p, f)| f(p, node, group, out))
}
unsafe extern "win64" fn adp_set_vmem(_this: *mut Obj<D3d11DxgiAdapter>, _node: u32, _group: u32, _bytes: u64) -> HRESULT { S_OK }
unsafe extern "win64" fn adp_get_desc3(this: *mut Obj<D3d11DxgiAdapter>, out: *mut c_void) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT;
    adp_call_slot::<Fn>(this, 18).map_or(E_FAIL, |(p, f)| f(p, out))
}

struct D3d11DxgiFactory {
    inner: usize,
}

impl Drop for D3d11DxgiFactory {
    fn drop(&mut self) {
        if self.inner != 0 {
            unsafe { com_release(self.inner as *mut c_void) };
        }
    }
}

static D11_FACTORY_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11DxgiFactory {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        use crate::dxgi::*;
        class_once(&D11_FACTORY_CLASS, || {
            let mut s = dxgi_object_slots::<D3d11DxgiFactory>();
            s.extend([
                ("GetParent", Some(fac_get_parent as *const ())),
                ("EnumAdapters", Some(fac_enum_adapters as *const ())),
                ("MakeWindowAssociation", Some(fac_make_window_assoc as *const ())),
                ("GetWindowAssociation", Some(fac_get_window_assoc as *const ())),
                ("CreateSwapChain", Some(fac_create_swapchain as *const ())),
                ("CreateSoftwareAdapter", Some(fac_create_sw as *const ())),
                ("EnumAdapters1", Some(fac_enum_adapters as *const ())),
                ("IsCurrent", Some(fac_is_current as *const ())),
                ("IsWindowedStereoEnabled", Some(fac_is_stereo as *const ())),
                ("CreateSwapChainForHwnd", Some(fac_create_swapchain_hwnd as *const ())),
                ("CreateSwapChainForCoreWindow", Some(fac_unsupported as *const ())),
                ("GetSharedResourceAdapterLuid", Some(fac_shared_luid as *const ())),
                ("RegisterStereoStatusWindow", Some(fac_reg_status_win as *const ())),
                ("RegisterStereoStatusEvent", Some(fac_reg_status_ev as *const ())),
                ("UnregisterStereoStatus", Some(fac_unreg as *const ())),
                ("RegisterOcclusionStatusWindow", Some(fac_reg_status_win as *const ())),
                ("RegisterOcclusionStatusEvent", Some(fac_reg_status_ev as *const ())),
                ("UnregisterOcclusionStatus", Some(fac_unreg as *const ())),
                ("CreateSwapChainForComposition", Some(fac_unsupported as *const ())),
                ("GetCreationFlags", Some(fac_get_flags as *const ())),
                ("EnumAdapterByLuid", Some(fac_enum_luid as *const ())),
                ("EnumWarpAdapter", Some(fac_enum_warp as *const ())),
                ("CheckFeatureSupport", Some(fac_check_feat as *const ())),
                ("EnumAdapterByGpuPreference", Some(fac_enum_gpu_pref as *const ())),
                ("RegisterAdaptersChangedEvent", Some(fac_reg_status_ev as *const ())),
                ("UnregisterAdaptersChangedEvent", Some(fac_unreg as *const ())),
            ]);
            Class {
                name: "IDXGIFactory7(D3D11)",
                vtbl: build_vtable("IDXGIFactory7(D3D11)", &s),
                iids: &[
                    IID_IDXGIObject,
                    IID_IDXGIFactory,
                    IID_IDXGIFactory1,
                    IID_IDXGIFactory2,
                    IID_IDXGIFactory3,
                    IID_IDXGIFactory4,
                    IID_IDXGIFactory5,
                    IID_IDXGIFactory6,
                    IID_IDXGIFactory7,
                ],
            }
        })
    }
}

pub unsafe fn new_d3d11_factory() -> *mut c_void {
    let mut inner: *mut c_void = std::ptr::null_mut();
    let _ = crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory7, &mut inner);
    Obj::new(D3d11DxgiFactory { inner: inner as usize }) as *mut c_void
}

unsafe fn fac_slot<F>(this: *mut Obj<D3d11DxgiFactory>, slot: usize) -> Option<(*mut c_void, F)> {
    let p = Obj::get(this).inner as *mut c_void;
    if p.is_null() { return None; }
    let vt = *(p as *const *const usize);
    Some((p, std::mem::transmute_copy(&*vt.add(slot))))
}

unsafe extern "win64" fn fac_get_parent(_this: *mut Obj<D3d11DxgiFactory>, _riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    E_NOINTERFACE
}
unsafe extern "win64" fn fac_enum_adapters(_this: *mut Obj<D3d11DxgiFactory>, i: u32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() { return DXGI_ERROR_INVALID_CALL; }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if i != 0 || d3d12_metal::metal::device().is_none() { return DXGI_ERROR_NOT_FOUND; }
    std::ptr::write_unaligned(out, new_d3d11_adapter());
    S_OK
}
unsafe extern "win64" fn fac_make_window_assoc(this: *mut Obj<D3d11DxgiFactory>, hwnd: HWND, flags: u32) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, HWND, u32) -> HRESULT;
    fac_slot::<Fn>(this, 8).map_or(S_OK, |(p, f)| f(p, hwnd, flags))
}
unsafe extern "win64" fn fac_get_window_assoc(this: *mut Obj<D3d11DxgiFactory>, out: *mut HWND) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut HWND) -> HRESULT;
    fac_slot::<Fn>(this, 9).map_or(E_FAIL, |(p, f)| f(p, out))
}
unsafe extern "win64" fn fac_create_swapchain(
    this: *mut Obj<D3d11DxgiFactory>,
    device: *mut c_void,
    desc: *const SwapChainDesc,
    out: *mut *mut c_void,
) -> HRESULT {
    if desc.is_null() || out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    if let Some(core) = device_core_from_ptr(device) {
        let d = std::ptr::read_unaligned(desc);
        let d1 = SwapChainDesc1 {
            width: d.width,
            height: d.height,
            format: d.format,
            stereo: FALSE,
            sample_count: d.sample_count,
            sample_quality: d.sample_quality,
            buffer_usage: d.buffer_usage,
            buffer_count: d.buffer_count,
            scaling: 0,
            swap_effect: d.swap_effect,
            alpha_mode: 0,
            flags: d.flags,
        };
        let fs = FullscreenDesc {
            refresh_num: d.refresh_num,
            refresh_den: d.refresh_den,
            scanline_ordering: d.scanline_ordering,
            scaling: d.scaling,
            windowed: d.windowed,
        };
        return super::swapchain::create_for_d3d11(core, this as *mut c_void, d.output_window, d1, Some(fs), out);
    }
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void, *const SwapChainDesc, *mut *mut c_void) -> HRESULT;
    fac_slot::<Fn>(this, 10).map_or(DXGI_ERROR_INVALID_CALL, |(p, f)| f(p, device, desc, out))
}
unsafe extern "win64" fn fac_create_swapchain_hwnd(
    this: *mut Obj<D3d11DxgiFactory>,
    device: *mut c_void,
    hwnd: HWND,
    desc: *const SwapChainDesc1,
    fs: *const FullscreenDesc,
    restrict: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if desc.is_null() || out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    if let Some(core) = device_core_from_ptr(device) {
        let d1 = std::ptr::read_unaligned(desc);
        let fs = if fs.is_null() { None } else { Some(std::ptr::read_unaligned(fs)) };
        return super::swapchain::create_for_d3d11(core, this as *mut c_void, hwnd as usize, d1, fs, out);
    }
    type Fn = unsafe extern "win64" fn(*mut c_void, *mut c_void, HWND, *const SwapChainDesc1, *const FullscreenDesc, *mut c_void, *mut *mut c_void) -> HRESULT;
    fac_slot::<Fn>(this, 15).map_or(DXGI_ERROR_INVALID_CALL, |(p, f)| f(p, device, hwnd, desc, fs, restrict, out))
}
unsafe extern "win64" fn fac_create_sw(_this: *mut Obj<D3d11DxgiFactory>, _m: HMODULE, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    DXGI_ERROR_UNSUPPORTED
}
unsafe extern "win64" fn fac_is_current(_this: *mut Obj<D3d11DxgiFactory>) -> BOOL { TRUE }
unsafe extern "win64" fn fac_is_stereo(_this: *mut Obj<D3d11DxgiFactory>) -> BOOL { FALSE }
unsafe extern "win64" fn fac_unsupported(_this: *mut Obj<D3d11DxgiFactory>) -> HRESULT { DXGI_ERROR_UNSUPPORTED }
unsafe extern "win64" fn fac_shared_luid(_this: *mut Obj<D3d11DxgiFactory>, _h: HANDLE, out: *mut u64) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, 0); }
    E_INVALIDARG
}
unsafe extern "win64" fn fac_reg_status_win(_this: *mut Obj<D3d11DxgiFactory>, _hwnd: HWND, _msg: u32, cookie: *mut u32) -> HRESULT {
    if !cookie.is_null() { std::ptr::write_unaligned(cookie, 1); }
    S_OK
}
unsafe extern "win64" fn fac_reg_status_ev(_this: *mut Obj<D3d11DxgiFactory>, _ev: HANDLE, cookie: *mut u32) -> HRESULT {
    if !cookie.is_null() { std::ptr::write_unaligned(cookie, 1); }
    S_OK
}
unsafe extern "win64" fn fac_unreg(_this: *mut Obj<D3d11DxgiFactory>, _cookie: u32) {}
unsafe extern "win64" fn fac_get_flags(_this: *mut Obj<D3d11DxgiFactory>) -> u32 { 0 }
unsafe extern "win64" fn fac_enum_luid(_this: *mut Obj<D3d11DxgiFactory>, luid: u64, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() { return DXGI_ERROR_INVALID_CALL; }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if luid != crate::dxgi::adapter::adapter_luid() { return DXGI_ERROR_NOT_FOUND; }
    let a = new_d3d11_adapter();
    let hr = query_out(a, riid, out);
    com_release(a);
    hr
}
unsafe extern "win64" fn fac_enum_warp(_this: *mut Obj<D3d11DxgiFactory>, _riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    DXGI_ERROR_NOT_FOUND
}
unsafe extern "win64" fn fac_check_feat(this: *mut Obj<D3d11DxgiFactory>, feat: u32, data: *mut c_void, size: u32) -> HRESULT {
    type Fn = unsafe extern "win64" fn(*mut c_void, u32, *mut c_void, u32) -> HRESULT;
    fac_slot::<Fn>(this, 28).map_or(E_INVALIDARG, |(p, f)| f(p, feat, data, size))
}
unsafe extern "win64" fn fac_enum_gpu_pref(_this: *mut Obj<D3d11DxgiFactory>, i: u32, _pref: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() { return DXGI_ERROR_INVALID_CALL; }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if i != 0 || d3d12_metal::metal::device().is_none() { return DXGI_ERROR_NOT_FOUND; }
    let a = new_d3d11_adapter();
    let hr = query_out(a, riid, out);
    com_release(a);
    hr
}
