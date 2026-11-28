//! ID3D12Heap1 (placement `MTLHeap`), placed resources and ID3D12QueryHeap.

use super::com::*;
use super::failed;
use super::resource::Resource;
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::heap::Heap as MtlHeap;
use d3d12_metal::metal::resource::{CreateError, Resource as MtlResource};
use d3d12_metal::types::{ClearValue, HeapDesc, ResourceDesc};
use std::ffi::c_void;
use std::sync::{Arc, OnceLock};

// ---------------------------------------------------------------------------
// Heaps
// ---------------------------------------------------------------------------

pub struct Heap {
    pub h: Arc<MtlHeap>,
}

static HEAP_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Heap {
    fn class() -> &'static Class {
        class_once(&HEAP_CLASS, || Class {
            name: "ID3D12Heap1",
            vtbl: build_named::<Heap>("ID3D12Heap1", ID3D12Heap1, &[m!("GetDesc" => heap_get_desc), m!("SetName" => heap_set_name)]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12Heap, IID_ID3D12Heap1],
        })
    }
}

type PH = *mut Obj<Heap>;

pub unsafe fn create_heap(desc: *const HeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateHeap: null desc");
    }
    let mut d = std::ptr::read_unaligned(desc);
    if d.Alignment == 0 {
        d.Alignment = d3d12_metal::metal::resource::DEFAULT_ALIGNMENT;
    }
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let h = match MtlHeap::new(&d, "") {
        Ok(h) => h,
        Err(CreateError::OutOfMemory(e)) => return failed!(E_OUTOFMEMORY, "CreateHeap({d:?}): {e}"),
        Err(e) => return failed!(E_INVALIDARG, "CreateHeap({d:?}): {e:?}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    log::debug!(target: "d3d", "CreateHeap({:#x} bytes, heap type {}, flags {:#x})", d.SizeInBytes, d.Properties.Type, d.Flags);
    let o = Obj::new(Heap { h: Arc::new(h) });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

unsafe extern "win64" fn heap_get_desc(this: PH, ret: *mut HeapDesc) -> *mut HeapDesc {
    std::ptr::write_unaligned(ret, Obj::get(this).h.desc);
    ret
}

unsafe extern "win64" fn heap_set_name(this: PH, name: *const u16) -> HRESULT {
    let s = read_wstr(name);
    Obj::get(this).h.set_label(&s);
    set_name::<Heap>(this, name)
}

pub unsafe fn create_placed_resource(
    heap: *mut c_void,
    offset: u64,
    desc: *const ResourceDesc,
    state: u32,
    _clear: *const ClearValue,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    let Some(h) = Obj::<Heap>::from_iface(heap) else { return failed!(E_INVALIDARG, "CreatePlacedResource: foreign heap {heap:?}") };
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreatePlacedResource: null desc");
    }
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let d = std::ptr::read_unaligned(desc);
    let h = &Obj::get(h).h;
    let r = match MtlResource::new_placed_default(h, offset, &d, "") {
        Ok(r) => r,
        Err(CreateError::OutOfMemory(e)) => return failed!(E_OUTOFMEMORY, "CreatePlacedResource({d:?} at {offset:#x}): {e}"),
        Err(e) => return failed!(E_INVALIDARG, "CreatePlacedResource({d:?} at {offset:#x}): {e:?}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    let o = Resource::wrap(Arc::new(r), h.desc.Flags, state);
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

// ---------------------------------------------------------------------------
// Query heaps
// ---------------------------------------------------------------------------

/// `D3D12_QUERY_HEAP_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QueryHeapDesc {
    pub ty: u32,
    pub count: u32,
    pub node_mask: u32,
}

/// Timestamp (`TIMESTAMP`, `COPY_QUEUE_TIMESTAMP`) and occlusion (`OCCLUSION`,
/// supporting both counting and `BINARY_OCCLUSION` queries) heaps are real
/// (`d3d12_metal::metal::query`); pipeline-statistics queries still trap in
/// `BeginQuery`/`EndQuery`/`ResolveQueryData` so no fabricated results reach
/// the game.
pub struct QueryHeap {
    pub desc: QueryHeapDesc,
    pub q: d3d12_metal::metal::query::QueryHeap,
}

static QH_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for QueryHeap {
    fn class() -> &'static Class {
        class_once(&QH_CLASS, || Class {
            name: "ID3D12QueryHeap",
            vtbl: build_named::<QueryHeap>("ID3D12QueryHeap", ID3D12QueryHeap, &[]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12QueryHeap],
        })
    }
}

pub unsafe fn query_heap_from_ptr<'a>(p: *mut c_void) -> Option<&'a QueryHeap> {
    Obj::<QueryHeap>::from_iface(p).map(|o| &(*o).inner)
}

pub unsafe fn create_query_heap(desc: *const QueryHeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateQueryHeap: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    if !matches!(d.ty, 0 | 1 | 2 | 5 | 7) {
        return failed!(E_INVALIDARG, "CreateQueryHeap({d:?}): unsupported query heap type");
    }
    if d.count == 0 {
        return failed!(E_INVALIDARG, "CreateQueryHeap({d:?}): count must be > 0");
    }
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    let q = match d3d12_metal::metal::query::QueryHeap::try_new(d.ty, d.count) {
        Ok(q) => q,
        Err(d3d12_metal::metal::query::QueryHeapError::InvalidArg(e)) => {
            return failed!(E_INVALIDARG, "CreateQueryHeap({d:?}): {e}");
        }
        Err(d3d12_metal::metal::query::QueryHeapError::OutOfMemory(e)) => {
            return failed!(E_OUTOFMEMORY, "CreateQueryHeap({d:?}): {e}");
        }
    };
    log::debug!(target: "d3d", "CreateQueryHeap({d:?})");
    let o = Obj::new(QueryHeap { desc: d, q });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_query_heap_validates_args_and_supports_1m_heaps() {
        unsafe {
            let mut out: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_query_heap(std::ptr::null(), &IID_ID3D12QueryHeap, &mut out), E_INVALIDARG);

            let bad_ty = QueryHeapDesc { ty: 99, count: 16, node_mask: 0 };
            assert_eq!(create_query_heap(&bad_ty, &IID_ID3D12QueryHeap, &mut out), E_INVALIDARG);

            let zero_cnt = QueryHeapDesc { ty: 0, count: 0, node_mask: 0 };
            assert_eq!(create_query_heap(&zero_cnt, &IID_ID3D12QueryHeap, &mut out), E_INVALIDARG);

            let valid_probe = QueryHeapDesc { ty: 0, count: 1_048_576, node_mask: 0 };
            assert_eq!(create_query_heap(&valid_probe, &IID_ID3D12QueryHeap, std::ptr::null_mut()), S_FALSE);

            if d3d12_metal::metal::device().is_none() {
                return;
            }
            for (ty, count) in [(0u32, 1_048_576u32), (1, 1_048_576), (1, 128), (2, 64), (5, 65_536)] {
                let d = QueryHeapDesc { ty, count, node_mask: 0 };
                let mut p: *mut c_void = std::ptr::null_mut();
                let hr = create_query_heap(&d, &IID_ID3D12QueryHeap, &mut p);
                assert_eq!(hr, S_OK, "CreateQueryHeap({d:?}) failed with {hr:#x}");
                assert!(!p.is_null());
                let qh = query_heap_from_ptr(p).expect("valid QueryHeap COM object");
                assert_eq!(qh.desc.ty, ty);
                assert_eq!(qh.desc.count, count);
                let obj = Obj::<QueryHeap>::from_iface(p).unwrap();
                Obj::release(obj);
            }
        }
    }
}
