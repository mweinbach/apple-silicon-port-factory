//! ID3D12Resource2 over `d3d12_metal::metal::resource::Resource`.

use super::com::*;
use super::tr::tr;
use super::failed;
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{HeapProperties, ResourceDesc};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};

pub struct Resource {
    pub r: Arc<MtlResource>,
    pub heap_flags: u32,
    pub initial_state: u32,
    maps: AtomicU32,
}

static CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Resource {
    fn class() -> &'static Class {
        class_once(&CLASS, || Class {
            name: "ID3D12Resource2",
            vtbl: build_named::<Resource>(
                "ID3D12Resource2",
                ID3D12Resource2,
                &[
                    m!("Map" => map),
                    m!("Unmap" => unmap),
                    m!("GetDesc" => get_desc),
                    m!("GetDesc1" => get_desc1),
                    m!("GetGPUVirtualAddress" => get_gpu_virtual_address),
                    m!("GetHeapProperties" => get_heap_properties),
                    m!("SetName" => set_name_res),
                ],
            ),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12Resource, IID_ID3D12Resource1, IID_ID3D12Resource2],
        })
    }
}

pub type PR = *mut Obj<Resource>;

impl Resource {
    pub fn wrap(r: Arc<MtlResource>, heap_flags: u32, initial_state: u32) -> PR {
        r.register_ptr();
        Obj::new(Resource { r, heap_flags, initial_state, maps: AtomicU32::new(0) })
    }
}

/// The backend resource of an `ID3D12Resource*` we created (None for
/// foreign/NULL pointers).
pub unsafe fn from_ptr<'a>(p: *mut c_void) -> Option<&'a Resource> {
    Obj::<Resource>::from_iface(p).map(|o| &(*o).inner)
}

/// `CreateCommittedResource*` (heap flags, initial state and clear value
/// are recorded; textures start with undefined contents).
pub unsafe fn create_committed(
    props: *const HeapProperties,
    heap_flags: u32,
    desc: *const ResourceDesc,
    state: u32,
    _clear: *const d3d12_metal::types::ClearValue,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if props.is_null() || desc.is_null() {
        return failed!(E_INVALIDARG, "CreateCommittedResource: null heap properties / desc");
    }
    let (p, d) = (*props, *desc);
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::ResourceAlloc, d.Width);
    let r = match MtlResource::new_committed_default(&d, &p, "") {
        Ok(r) => r,
        Err(d3d12_metal::metal::resource::CreateError::OutOfMemory(e)) => return failed!(E_OUTOFMEMORY, "CreateCommittedResource({d:?}, {p:?}): {e}"),
        Err(e) => return failed!(E_INVALIDARG, "CreateCommittedResource({d:?}, {p:?}): {e:?}"),
    };
    let elapsed_ns = scope.elapsed_ns();
    drop(scope);
    let bytes = r.allocated_bytes();
    crate::hitch::record_resource_alloc(bytes, "CommittedResource", elapsed_ns);
    if out.is_null() {
        return S_FALSE;
    }
    log::trace!(target: "d3d", "CreateCommittedResource({d:?}, heap {}) va {:#x}", p.Type, r.gpu_va);
    let o = Resource::wrap(Arc::new(r), heap_flags, state);
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

unsafe extern "win64" fn set_name_res(this: PR, name: *const u16) -> HRESULT {
    let s = read_wstr(name);
    Obj::get(this).r.set_label(&s);
    set_name::<Resource>(this, name)
}

unsafe extern "win64" fn map(this: PR, sub: u32, _read: *const c_void, out: *mut *mut c_void) -> HRESULT {
    let r = Obj::get(this);
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if sub != 0 {
        return failed!(E_INVALIDARG, "ID3D12Resource::Map(subresource {sub}) of {:?}: only buffers are mappable", r.r.desc);
    }
    let Some(p) = r.r.cpu_ptr() else {
        return failed!(E_INVALIDARG, "ID3D12Resource::Map of a resource on a non-CPU-visible heap ({:?})", r.r.heap);
    };
    r.maps.fetch_add(1, Ordering::AcqRel);
    if r.r.heap.Type != d3d12_metal::types::heap_type::READBACK {
        super::tr::track_mapped(this as u64, p, r.r.desc.Width as usize);
    }
    if !out.is_null() {
        *out = p as *mut c_void;
    }
    S_OK
}

/// `written` = D3D12_RANGE (NULL = whole resource; Begin == End = nothing).
/// Persistently mapped upload buffers are not traced (no write tracking).
unsafe extern "win64" fn unmap(this: PR, _sub: u32, _written: *const c_void) {
    let r = Obj::get(this);
    tr!(
        let (b, e) = if _written.is_null() { (0usize, r.r.desc.Width as usize) } else { std::ptr::read_unaligned(_written as *const (usize, usize)) };
        let e = e.min(r.r.desc.Width as usize);
        let data: &[u8] = match r.r.cpu_ptr() { Some(p) if e > b => std::slice::from_raw_parts((p as *const u8).add(b), e - b), _ => &[] };
        Call::WriteResource { resource: this as u64, subresource: _sub, offset: b as u64, data }
    );
    if r.maps.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_sub(1)) == Ok(1) {
        super::tr::untrack(this as u64);
    }
}

unsafe extern "win64" fn get_desc(this: PR, ret: *mut ResourceDesc) -> *mut ResourceDesc {
    *ret = Obj::get(this).r.desc;
    ret
}

/// `D3D12_RESOURCE_DESC1` = DESC + SamplerFeedbackMipRegion (72 bytes).
#[repr(C)]
pub struct ResourceDesc1 {
    pub desc: ResourceDesc,
    pub mip_region: [u32; 3],
}

unsafe extern "win64" fn get_desc1(this: PR, ret: *mut ResourceDesc1) -> *mut ResourceDesc1 {
    *ret = ResourceDesc1 { desc: Obj::get(this).r.desc, mip_region: [0; 3] };
    ret
}

unsafe extern "win64" fn get_gpu_virtual_address(this: PR) -> u64 {
    Obj::get(this).r.gpu_va
}

unsafe extern "win64" fn get_heap_properties(this: PR, props: *mut HeapProperties, flags: *mut u32) -> HRESULT {
    let r = Obj::get(this);
    if !props.is_null() {
        let mut h = r.r.heap;
        h.CreationNodeMask = 1;
        h.VisibleNodeMask = 1;
        *props = h;
    }
    if !flags.is_null() {
        *flags = r.heap_flags;
    }
    S_OK
}

pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            map as *const () as usize,
            unmap as *const () as usize,
            get_desc as *const () as usize,
            get_desc1 as *const () as usize,
            get_gpu_virtual_address as *const () as usize,
            get_heap_properties as *const () as usize,
            set_name_res as *const () as usize,
        ]
        .into_boxed_slice()
    })
}
