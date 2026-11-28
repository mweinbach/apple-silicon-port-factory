//! ID3D12Device14 (one object answers ID3D12Device..Device14).

use super::com::*;
use super::tr::{self, tr};
use super::vtables::*;
use super::{failed, register_names};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::sync::{Mutex, OnceLock};

pub struct Device {
    pub mtl: &'static d3d12_metal::metal::Device,
    removed_reason: Mutex<HRESULT>,
}

static CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Device {
    fn class() -> &'static Class {
        class_once(&CLASS, || Class {
            name: "ID3D12Device14",
            vtbl: build_named::<Device>(
                "ID3D12Device14",
                ID3D12Device14,
                &[
                    m!("GetNodeCount" => get_node_count),
                    m!("CheckFeatureSupport" => check_feature_support),
                    m!("GetDescriptorHandleIncrementSize" => get_descriptor_handle_increment_size),
                    m!("GetDeviceRemovedReason" => get_device_removed_reason),
                    m!("SetStablePowerState" => set_stable_power_state),
                    m!("GetAdapterLuid" => get_adapter_luid),
                    m!("CreateCommandQueue" => create_command_queue),
                    m!("CreateCommandAllocator" => create_command_allocator),
                    m!("CreateCommandList" => create_command_list),
                    m!("CreateCommandList1" => create_command_list1),
                    m!("CreateFence" => create_fence),
                    m!("CreateDescriptorHeap" => create_descriptor_heap),
                    m!("CreateRenderTargetView" => super::descheap::create_render_target_view),
                    m!("CreateDepthStencilView" => super::descheap::create_depth_stencil_view),
                    m!("CreateCommittedResource" => create_committed_resource),
                    m!("CreateCommittedResource1" => create_committed_resource1),
                    m!("GetCustomHeapProperties" => get_custom_heap_properties),
                    m!("CreateConstantBufferView" => super::descheap::create_constant_buffer_view),
                    m!("CreateShaderResourceView" => super::descheap::create_shader_resource_view),
                    m!("CreateUnorderedAccessView" => super::descheap::create_unordered_access_view),
                    m!("CreateSampler" => super::descheap::create_sampler),
                    m!("CopyDescriptors" => super::descheap::copy_descriptors),
                    m!("CopyDescriptorsSimple" => super::descheap::copy_descriptors_simple),
                    m!("GetCopyableFootprints" => get_copyable_footprints),
                    m!("GetResourceAllocationInfo" => get_resource_allocation_info),
                    m!("GetResourceAllocationInfo1" => get_resource_allocation_info1),
                    m!("CreateRootSignature" => create_root_signature),
                    m!("CreateHeap" => create_heap),
                    m!("CreatePlacedResource" => create_placed_resource),
                    m!("CreateQueryHeap" => create_query_heap),
                    m!("CreateCommandSignature" => create_command_signature),
                    m!("CreateComputePipelineState" => create_compute_pipeline_state),
                    m!("CreateGraphicsPipelineState" => create_graphics_pipeline_state),
                    m!("CreatePipelineState" => create_pipeline_state),
                    m!("CreateStateObject" => create_state_object),
                    m!("AddToStateObject" => add_to_state_object),
                    m!("GetRaytracingAccelerationStructurePrebuildInfo" => get_raytracing_acceleration_structure_prebuild_info),
                    m!("MakeResident" => make_resident),
                    m!("Evict" => evict),
                    m!("EnqueueMakeResident" => enqueue_make_resident),
                    m!("SetResidencyPriority" => set_residency_priority),
                ],
            ),
            iids: &[
                IID_ID3D12Object,
                IID_ID3D12Device,
                IID_ID3D12Device1,
                IID_ID3D12Device2,
                IID_ID3D12Device3,
                IID_ID3D12Device4,
                IID_ID3D12Device5,
                IID_ID3D12Device6,
                IID_ID3D12Device7,
                IID_ID3D12Device8,
                IID_ID3D12Device9,
                IID_ID3D12Device10,
                IID_ID3D12Device11,
                IID_ID3D12Device12,
                IID_ID3D12Device13,
                IID_ID3D12Device14,
            ],
        })
    }

    fn final_release(&self) {
        // The singleton dies with its last reference; a later
        // D3D12CreateDevice makes a new one.
        DEVICE.store(std::ptr::null_mut(), Ordering::Release);
    }
}

pub type PD = *mut Obj<Device>;

static CREATE_LOCK: Mutex<()> = Mutex::new(());

const MAX_FEATURE_LEVEL: u32 = d3d12_metal::caps::FL_12_1;

pub unsafe fn create_device(adapter: *mut c_void, min_level: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let iid = Guid::read(riid).unwrap_or_default();
    if !adapter.is_null() && Obj::<crate::dxgi::adapter::Adapter>::from_iface(adapter).is_none() {
        log::warn!(target: "d3d", "D3D12CreateDevice: adapter {adapter:?} is not one of ours; using the default adapter");
    }
    if min_level > MAX_FEATURE_LEVEL || min_level < 0x9100 {
        if !out.is_null() {
            *out = std::ptr::null_mut();
        }
        return failed!(DXGI_ERROR_UNSUPPORTED, "D3D12CreateDevice(min level {min_level:#x}, {iid:?}): max is 12_1");
    }
    let Some(mtl) = d3d12_metal::metal::device() else {
        if !out.is_null() {
            *out = std::ptr::null_mut();
        }
        return failed!(DXGI_ERROR_UNSUPPORTED, "D3D12CreateDevice: no Metal 4 device");
    };
    // NULL ppDevice: "would it succeed?"
    if out.is_null() {
        return S_FALSE;
    }
    let _g = CREATE_LOCK.lock().unwrap();
    let existing = DEVICE.load(Ordering::Acquire);
    let hr = if !existing.is_null() {
        query_out(existing, riid, out)
    } else {
        let d = Obj::new(Device { mtl, removed_reason: Mutex::new(S_OK) });
        DEVICE.store(d as *mut c_void, Ordering::Release);
        crate::process::register_exit_hook(d3d12_metal::metal::pipeline::flush_pso_cache_if_initialized);
        let hr = query_out(d as *mut c_void, riid, out);
        Obj::release(d);
        hr
    };
    log::info!(target: "d3d", "D3D12CreateDevice(min level {min_level:#x}, {iid:?}) -> {hr:#x} ({:?})", *out);
    if let Some(id) = tr::created(hr, out) {
        tr::set_device(id);
        tr!(Call::CreateDevice { device: id, feature_level: min_level });
    }
    register_names();
    hr
}

unsafe extern "win64" fn get_node_count(_this: PD) -> u32 {
    1
}

unsafe extern "win64" fn check_feature_support(_this: PD, feature: u32, data: *mut c_void, size: u32) -> HRESULT {
    if data.is_null() {
        return failed!(E_INVALIDARG, "CheckFeatureSupport({feature}): null data");
    }
    let buf = std::slice::from_raw_parts_mut(data as *mut u8, size as usize);
    match d3d12_metal::caps::check_feature_support(feature, buf) {
        Ok(()) => {
            log::debug!(target: "d3d", "CheckFeatureSupport({feature}, {size}) -> {:02x?}", &buf[..buf.len().min(64)]);
            S_OK
        }
        Err(d3d12_metal::caps::CapsError::InvalidArg(e)) => failed!(E_INVALIDARG, "CheckFeatureSupport: {e}"),
        Err(d3d12_metal::caps::CapsError::Unknown(f)) => failed!(E_INVALIDARG, "CheckFeatureSupport: unknown feature {f} (size {size})"),
    }
}

/// CBV/SRV/UAV and sampler descriptors are 24-byte `IRDescriptorTableEntry`s
/// (GPU handle = heap gpuAddress + i * 24); RTV/DSV are host records.
pub const DESCRIPTOR_SIZE_RESOURCE: u32 = 24;
pub const DESCRIPTOR_SIZE_RTV_DSV: u32 = 32;

unsafe extern "win64" fn get_descriptor_handle_increment_size(_this: PD, ty: u32) -> u32 {
    match ty {
        0 | 1 => DESCRIPTOR_SIZE_RESOURCE, // CBV_SRV_UAV, SAMPLER
        2 | 3 => DESCRIPTOR_SIZE_RTV_DSV,  // RTV, DSV
        _ => {
            log::warn!(target: "d3d", "GetDescriptorHandleIncrementSize({ty}): bad type");
            0
        }
    }
}

unsafe extern "win64" fn get_device_removed_reason(this: PD) -> HRESULT {
    *(*this).inner.removed_reason.lock().unwrap()
}

unsafe extern "win64" fn set_stable_power_state(_this: PD, enable: BOOL) -> HRESULT {
    log::debug!(target: "d3d", "SetStablePowerState({enable}) ignored");
    S_OK
}

/// Struct return (MSVC): `LUID* GetAdapterLuid(this, LUID* ret)`.
unsafe extern "win64" fn get_adapter_luid(_this: PD, ret: *mut u64) -> *mut u64 {
    *ret = crate::dxgi::adapter::adapter_luid();
    ret
}

// ---------------------------------------------------------------------------
// Object creation
// ---------------------------------------------------------------------------

unsafe extern "win64" fn create_command_queue(_this: PD, desc: *const d3d12_metal::types::CommandQueueDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::queue::create(desc, riid, out);
    tr!(Call::CreateCommandQueue { device: tr::device(), queue: tr::created(hr, out).unwrap_or(0), desc: tr::raw(desc) });
    hr
}

unsafe extern "win64" fn create_command_allocator(_this: PD, ty: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::queue::create_allocator(ty, riid, out);
    tr!(Call::CreateCommandAllocator { device: tr::device(), allocator: tr::created(hr, out).unwrap_or(0), list_type: ty });
    hr
}

unsafe extern "win64" fn create_command_list(_this: PD, _node: u32, ty: u32, alloc: *mut c_void, pso: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::cmdlist::create(ty, alloc, pso, riid, out);
    tr!(Call::CreateCommandList { device: tr::device(), list: tr::created(hr, out).unwrap_or(0), list_type: ty, allocator: alloc as u64, initial_pso: pso as u64 });
    hr
}

unsafe extern "win64" fn create_command_list1(_this: PD, _node: u32, ty: u32, _flags: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::cmdlist::create1(ty, riid, out);
    tr!(Call::CreateCommandList { device: tr::device(), list: tr::created(hr, out).unwrap_or(0), list_type: ty, allocator: 0, initial_pso: 0 });
    hr
}

unsafe extern "win64" fn create_fence(_this: PD, initial: u64, flags: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    if flags & !1 != 0 {
        // SHARED / SHARED_CROSS_ADAPTER: no cross-process sharing here.
        log::warn!(target: "d3d", "CreateFence: flags {flags:#x} (sharing) ignored");
    }
    let Some(f) = d3d12_metal::metal::fence::Fence::new(initial) else { return failed!(E_OUTOFMEMORY, "CreateFence: newSharedEvent failed") };
    let o = Obj::new(super::fence::Fence { f, flags });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    tr!(Call::CreateFence { device: tr::device(), fence: tr::created(hr, out).unwrap_or(0), initial_value: initial, flags });
    hr
}

unsafe extern "win64" fn create_descriptor_heap(_this: PD, desc: *const d3d12_metal::types::DescriptorHeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::descheap::create(desc, riid, out);
    tr!(
        let id = tr::created(hr, out).unwrap_or(0);
        let h = super::descheap::from_ptr(id as *mut c_void);
        Call::CreateDescriptorHeap { device: tr::device(), heap: id, desc: tr::raw(desc), cpu_start: h.map_or(0, |h| h.h.cpu_base as u64), gpu_start: h.map_or(0, |h| h.h.gpu_base) }
    );
    hr
}

unsafe extern "win64" fn create_committed_resource(
    _this: PD,
    props: *const d3d12_metal::types::HeapProperties,
    heap_flags: u32,
    desc: *const d3d12_metal::types::ResourceDesc,
    state: u32,
    clear: *const d3d12_metal::types::ClearValue,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    let hr = super::resource::create_committed(props, heap_flags, desc, state, clear, riid, out);
    trace_committed(hr, props, heap_flags, desc, state, clear, out);
    hr
}

unsafe fn trace_committed(hr: HRESULT, props: *const d3d12_metal::types::HeapProperties, heap_flags: u32, desc: *const d3d12_metal::types::ResourceDesc, state: u32, clear: *const d3d12_metal::types::ClearValue, out: *mut *mut c_void) {
    tr!(
        let id = tr::created(hr, out).unwrap_or(0);
        Call::CreateCommittedResource { device: tr::device(), resource: id, heap_properties: tr::raw(props), heap_flags, desc: tr::raw(desc), initial_state: state, clear_value: tr::opt(clear), gpu_va: tr::resource_va(id) }
    );
}

unsafe extern "win64" fn create_committed_resource1(
    _this: PD,
    props: *const d3d12_metal::types::HeapProperties,
    heap_flags: u32,
    desc: *const d3d12_metal::types::ResourceDesc,
    state: u32,
    clear: *const d3d12_metal::types::ClearValue,
    session: *mut c_void,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !session.is_null() {
        return failed!(E_INVALIDARG, "CreateCommittedResource1: protected sessions are not supported");
    }
    let hr = super::resource::create_committed(props, heap_flags, desc, state, clear, riid, out);
    trace_committed(hr, props, heap_flags, desc, state, clear, out);
    hr
}

/// Struct return: `D3D12_HEAP_PROPERTIES* (this, ret, nodeMask, heapType)`.
unsafe extern "win64" fn get_custom_heap_properties(_this: PD, ret: *mut d3d12_metal::types::HeapProperties, _node: u32, ty: u32) -> *mut d3d12_metal::types::HeapProperties {
    *ret = d3d12_metal::types::HeapProperties { Type: ty, ..Default::default() }.custom();
    ret
}

// ---------------------------------------------------------------------------
// Layout queries
// ---------------------------------------------------------------------------

unsafe extern "win64" fn get_copyable_footprints(
    _this: PD,
    desc: *const d3d12_metal::types::ResourceDesc,
    first: u32,
    n: u32,
    base: u64,
    layouts: *mut d3d12_metal::layout::PlacedFootprint,
    rows: *mut u32,
    row_sizes: *mut u64,
    total: *mut u64,
) {
    let r = if desc.is_null() { None } else { d3d12_metal::layout::copyable_footprints(&*desc, first, n, base) };
    let Some((ls, t)) = r else {
        log::warn!(target: "d3d", "GetCopyableFootprints({:?}, {first}, {n}): invalid", if desc.is_null() { None } else { Some(*desc) });
        for i in 0..n as usize {
            if !layouts.is_null() {
                *layouts.add(i) = Default::default();
            }
            if !rows.is_null() {
                *rows.add(i) = u32::MAX;
            }
            if !row_sizes.is_null() {
                *row_sizes.add(i) = u64::MAX;
            }
        }
        if !total.is_null() {
            *total = u64::MAX;
        }
        return;
    };
    for (i, l) in ls.iter().enumerate() {
        if !layouts.is_null() {
            *layouts.add(i) = l.footprint;
        }
        if !rows.is_null() {
            *rows.add(i) = l.num_rows;
        }
        if !row_sizes.is_null() {
            *row_sizes.add(i) = l.row_size;
        }
    }
    if !total.is_null() {
        *total = t;
    }
}

unsafe fn compute_allocation_info(
    n: u32,
    descs: *const d3d12_metal::types::ResourceDesc,
    infos1: *mut d3d12_metal::types::AllocationInfo1,
) -> d3d12_metal::types::AllocationInfo {
    use d3d12_metal::types::{AllocationInfo, AllocationInfo1};
    let fail = AllocationInfo { SizeInBytes: u64::MAX, Alignment: 0 };
    if n == 0 {
        return AllocationInfo { SizeInBytes: 0, Alignment: 0 };
    }
    if descs.is_null() {
        return fail;
    }
    let (mut size, mut align) = (0u64, 0u64);
    for i in 0..n as usize {
        let d = std::ptr::read_unaligned(descs.add(i));
        match d3d12_metal::metal::resource::allocation_info(&d) {
            Ok((s, a)) => {
                let off = size.div_ceil(a).saturating_mul(a);
                if !infos1.is_null() {
                    std::ptr::write_unaligned(
                        infos1.add(i),
                        AllocationInfo1 { Offset: off, Alignment: a, SizeInBytes: s },
                    );
                }
                size = off.saturating_add(s);
                align = align.max(a);
            }
            Err(e) => {
                log::debug!(target: "d3d", "GetResourceAllocationInfo({d:?}): {e:?}");
                if !infos1.is_null() {
                    for j in 0..n as usize {
                        std::ptr::write_unaligned(
                            infos1.add(j),
                            AllocationInfo1 { Offset: u64::MAX, Alignment: 0, SizeInBytes: u64::MAX },
                        );
                    }
                }
                return fail;
            }
        }
    }
    let total = if align > 0 { size.div_ceil(align).saturating_mul(align) } else { size };
    AllocationInfo { SizeInBytes: total, Alignment: align }
}

/// Struct return: `D3D12_RESOURCE_ALLOCATION_INFO* (this, ret, mask, n, descs)`.
unsafe extern "win64" fn get_resource_allocation_info(
    _this: PD,
    ret: *mut d3d12_metal::types::AllocationInfo,
    _mask: u32,
    n: u32,
    descs: *const d3d12_metal::types::ResourceDesc,
) -> *mut d3d12_metal::types::AllocationInfo {
    std::ptr::write_unaligned(ret, compute_allocation_info(n, descs, std::ptr::null_mut()));
    ret
}

/// Struct return: `D3D12_RESOURCE_ALLOCATION_INFO* (this, ret, mask, n, descs, infos1)`.
unsafe extern "win64" fn get_resource_allocation_info1(
    _this: PD,
    ret: *mut d3d12_metal::types::AllocationInfo,
    _mask: u32,
    n: u32,
    descs: *const d3d12_metal::types::ResourceDesc,
    infos1: *mut d3d12_metal::types::AllocationInfo1,
) -> *mut d3d12_metal::types::AllocationInfo {
    std::ptr::write_unaligned(ret, compute_allocation_info(n, descs, infos1));
    ret
}

unsafe extern "win64" fn create_root_signature(_this: PD, _node: u32, blob: *const u8, size: usize, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::pso::create_root_signature(blob, size, riid, out);
    tr!(Call::CreateRootSignature { device: tr::device(), root_signature: tr::created(hr, out).unwrap_or(0), blob: tr::raw_n(blob, size) });
    hr
}

unsafe extern "win64" fn create_heap(_this: PD, desc: *const d3d12_metal::types::HeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::heap::create_heap(desc, riid, out);
    tr!(Call::CreateHeap { device: tr::device(), heap: tr::created(hr, out).unwrap_or(0), desc: tr::raw(desc) });
    hr
}

unsafe extern "win64" fn create_placed_resource(
    _this: PD,
    heap: *mut c_void,
    offset: u64,
    desc: *const d3d12_metal::types::ResourceDesc,
    state: u32,
    clear: *const d3d12_metal::types::ClearValue,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    let hr = super::heap::create_placed_resource(heap, offset, desc, state, clear, riid, out);
    tr!(
        let id = tr::created(hr, out).unwrap_or(0);
        Call::CreatePlacedResource { device: tr::device(), resource: id, heap: heap as u64, offset, desc: tr::raw(desc), initial_state: state, clear_value: tr::opt(clear), gpu_va: tr::resource_va(id) }
    );
    hr
}

unsafe extern "win64" fn create_query_heap(_this: PD, desc: *const super::heap::QueryHeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::heap::create_query_heap(desc, riid, out);
    tr!(Call::CreateQueryHeap { device: tr::device(), heap: tr::created(hr, out).unwrap_or(0), desc: tr::raw(desc) });
    hr
}

unsafe extern "win64" fn create_command_signature(_this: PD, desc: *const super::pso::CommandSignatureDesc, rs: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::pso::create_command_signature(desc, rs, riid, out);
    tr!(
        let (stride, args) = if desc.is_null() { (0, &[][..]) } else { ((*desc).byte_stride, tr::raw_n((*desc).args, (*desc).num_args as usize)) };
        Call::CreateCommandSignature { device: tr::device(), signature: tr::created(hr, out).unwrap_or(0), root_signature: rs as u64, byte_stride: stride, arguments: args }
    );
    hr
}

unsafe extern "win64" fn create_compute_pipeline_state(_this: PD, desc: *const super::pso::ComputePipelineDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let hr = super::pso::create_compute_pipeline(desc, riid, out);
    tr!(
        let (rs, cs, flags) = if desc.is_null() { (0, &[][..], 0) } else { ((*desc).root_signature as u64, (*desc).cs.bytes().unwrap_or(&[]), (*desc).flags) };
        Call::CreateComputePipelineState { device: tr::device(), pso: tr::created(hr, out).unwrap_or(0), root_signature: rs, cs, flags }
    );
    hr
}

unsafe extern "win64" fn create_graphics_pipeline_state(_this: PD, desc: *const u8, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    super::pso::create_graphics_pipeline(desc, riid, out)
}

unsafe extern "win64" fn create_pipeline_state(_this: PD, desc: *const super::pso::StreamDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    super::pso::create_pipeline_state(desc, riid, out)
}

unsafe extern "win64" fn create_state_object(
    _this: PD,
    desc: *const d3d12_metal::rt::pipeline::RawStateObjectDesc,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    super::pso::create_state_object(desc, riid, out)
}

unsafe extern "win64" fn add_to_state_object(
    _this: PD,
    addition: *const d3d12_metal::rt::pipeline::RawStateObjectDesc,
    base: *mut c_void,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    super::pso::add_to_state_object(addition, base, riid, out)
}

unsafe extern "win64" fn get_raytracing_acceleration_structure_prebuild_info(
    _this: PD,
    desc: *const d3d12_metal::rt::d3d::BuildInputs,
    info: *mut d3d12_metal::rt::d3d::PrebuildInfo,
) {
    if info.is_null() {
        return;
    }
    if desc.is_null() {
        *info = Default::default();
        return;
    }
    match d3d12_metal::rt::d3d::Inputs::from_raw(&*desc).and_then(|i| d3d12_metal::rt::accel::prebuild_info(&i)) {
        Ok(pi) => {
            log::trace!(target: "d3d", "GetRaytracingAccelerationStructurePrebuildInfo({:?}) -> {pi:?}", *desc);
            *info = pi;
        }
        Err(e) => {
            log::warn!(target: "d3d", "GetRaytracingAccelerationStructurePrebuildInfo({:?}): {e}", *desc);
            *info = Default::default();
        }
    }
}

unsafe extern "win64" fn make_resident(_this: PD, _n: u32, _objects: *const *mut c_void) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn evict(_this: PD, _n: u32, _objects: *const *mut c_void) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn enqueue_make_resident(_this: PD, _flags: u32, _n: u32, _objects: *const *mut c_void, fence: *mut c_void, value: u64) -> HRESULT {
    if !fence.is_null() {
        let Some(f) = super::fence::from_ptr(fence) else {
            return failed!(E_INVALIDARG, "EnqueueMakeResident: foreign fence {fence:?}");
        };
        f.f.signal_cpu(value);
    }
    S_OK
}

unsafe extern "win64" fn set_residency_priority(_this: PD, _n: u32, _objects: *const *mut c_void, _priorities: *const u32) -> HRESULT {
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use d3d12_metal::metal::resource::{DEFAULT_ALIGNMENT, SMALL_ALIGNMENT};
    use d3d12_metal::types::{dim, AllocationInfo, AllocationInfo1, ResourceDesc};

    #[test]
    fn get_resource_allocation_info_rejects_unsupported_4k_and_populates_info1() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        unsafe {
            // 128x128x32 mips=4 R8_UNORM requires 16 KiB on Metal, so Alignment=4096 must
            // return SizeInBytes=UINT64_MAX, Alignment=0 (D3D12 small-alignment probe failure).
            let probe_4k = ResourceDesc {
                Dimension: dim::TEXTURE2D,
                Alignment: SMALL_ALIGNMENT,
                Width: 128,
                Height: 128,
                DepthOrArraySize: 32,
                MipLevels: 4,
                Format: 61,
                SampleCount: 1,
                ..Default::default()
            };
            let mut info = AllocationInfo::default();
            get_resource_allocation_info(std::ptr::null_mut(), &mut info, 0, 1, &probe_4k);
            assert_eq!(info, AllocationInfo { SizeInBytes: u64::MAX, Alignment: 0 });

            // Multi-resource GetResourceAllocationInfo1 with a 4 KiB small texture + 64 KiB array.
            let small = ResourceDesc {
                Dimension: dim::TEXTURE2D,
                Alignment: SMALL_ALIGNMENT,
                Width: 64,
                Height: 64,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: 61,
                SampleCount: 1,
                ..Default::default()
            };
            let large = ResourceDesc { Alignment: 0, ..probe_4k };
            let descs = [small, large];
            let mut infos1 = [AllocationInfo1::default(); 2];
            get_resource_allocation_info1(
                std::ptr::null_mut(),
                &mut info,
                0,
                2,
                descs.as_ptr(),
                infos1.as_mut_ptr(),
            );
            assert_eq!(infos1[0].Offset, 0);
            assert_eq!(infos1[0].Alignment, SMALL_ALIGNMENT);
            assert!(infos1[0].SizeInBytes >= SMALL_ALIGNMENT);
            assert_eq!(infos1[1].Alignment, DEFAULT_ALIGNMENT);
            assert_eq!(infos1[1].Offset, DEFAULT_ALIGNMENT);
            assert_eq!(info.Alignment, DEFAULT_ALIGNMENT);
            assert_eq!(info.SizeInBytes, infos1[1].Offset + infos1[1].SizeInBytes);
        }
    }
}



