//! ID3D12DescriptorHeap and the device's descriptor-writing methods for
//! render-target / depth-stencil views.

use super::com::*;
use super::tr::{self, tr};
use super::failed;
use super::resource;
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::cmdlist::TargetView;
use d3d12_metal::metal::descriptor::{
    bump_desc_heap_gen, heap_kind, DescriptorHeap as MtlHeap, RESOURCE_DESCRIPTOR_SIZE,
    TARGET_DESCRIPTOR_SIZE,
};
use d3d12_metal::types::DescriptorHeapDesc;
use std::ffi::c_void;
use std::sync::OnceLock;

pub struct DescriptorHeap {
    pub h: MtlHeap,
    pub desc: DescriptorHeapDesc,
}

static CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for DescriptorHeap {
    fn class() -> &'static Class {
        class_once(&CLASS, || Class {
            name: "ID3D12DescriptorHeap",
            vtbl: build_named::<DescriptorHeap>(
                "ID3D12DescriptorHeap",
                ID3D12DescriptorHeap,
                &[
                    m!("GetDesc" => get_desc),
                    m!("GetCPUDescriptorHandleForHeapStart" => get_cpu_start),
                    m!("GetGPUDescriptorHandleForHeapStart" => get_gpu_start),
                ],
            ),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12DescriptorHeap],
        })
    }
}

pub type PH = *mut Obj<DescriptorHeap>;

pub unsafe fn from_ptr<'a>(p: *mut c_void) -> Option<&'a DescriptorHeap> {
    Obj::<DescriptorHeap>::from_iface(p).map(|o| &(*o).inner)
}

/// D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
const SHADER_VISIBLE: u32 = 1;

pub unsafe fn create(desc: *const DescriptorHeapDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateDescriptorHeap: null desc");
    }
    let d = std::ptr::read_unaligned(desc);
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    let label = format!("descriptor heap type {} x{}", d.Type, d.NumDescriptors);
    let h = match MtlHeap::new(d.Type, d.NumDescriptors, d.Flags & SHADER_VISIBLE != 0, &label) {
        Ok(h) => h,
        Err(e) => return failed!(E_INVALIDARG, "CreateDescriptorHeap({d:?}): {e}"),
    };
    log::debug!(target: "d3d", "CreateDescriptorHeap({d:?}) cpu {:#x} gpu {:#x}", h.cpu_base, h.gpu_base);
    let o = Obj::new(DescriptorHeap { h, desc: d });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

unsafe extern "win64" fn get_desc(this: PH, ret: *mut DescriptorHeapDesc) -> *mut DescriptorHeapDesc {
    std::ptr::write_unaligned(ret, Obj::get(this).desc);
    ret
}

unsafe extern "win64" fn get_cpu_start(this: PH, ret: *mut usize) -> *mut usize {
    std::ptr::write_unaligned(ret, Obj::get(this).h.cpu_base);
    ret
}

unsafe extern "win64" fn get_gpu_start(this: PH, ret: *mut u64) -> *mut u64 {
    std::ptr::write_unaligned(ret, Obj::get(this).h.gpu_base);
    ret
}

// ---------------------------------------------------------------------------
// RTV / DSV descriptors (host records)
// ---------------------------------------------------------------------------

unsafe fn write_target(handle: usize, v: TargetView) {
    std::ptr::write_unaligned(handle as *mut TargetView, v);
}

/// Read the RTV/DSV record at a CPU descriptor handle.
pub unsafe fn read_target(handle: usize) -> TargetView {
    if handle == 0 {
        return TargetView::NULL;
    }
    std::ptr::read_unaligned(handle as *const TargetView)
}

pub unsafe extern "win64" fn create_render_target_view(_dev: *mut c_void, res: *mut c_void, desc: *const [u32; 6], handle: usize) {
    tr!(Call::CreateRenderTargetView { resource: res as u64, desc: tr::opt(desc), dest: handle as u64 });
    if handle == 0 {
        log::warn!(target: "d3d", "CreateRenderTargetView: null descriptor handle");
        return;
    }
    let Some(r) = resource::from_ptr(res) else {
        // NULL resource = null descriptor.
        write_target(handle, TargetView::NULL);
        return;
    };
    let d = if desc.is_null() { None } else { Some(std::ptr::read_unaligned(desc)) };
    match TargetView::rtv(&r.r, d.as_ref()) {
        Ok(v) => write_target(handle, v),
        Err(e) => {
            log::warn!(target: "d3d", "CreateRenderTargetView({:?}, {d:?}): {e}", r.r.desc);
            write_target(handle, TargetView::NULL);
        }
    }
}

pub unsafe extern "win64" fn create_depth_stencil_view(_dev: *mut c_void, res: *mut c_void, desc: *const [u32; 6], handle: usize) {
    tr!(Call::CreateDepthStencilView { resource: res as u64, desc: tr::opt(desc), dest: handle as u64 });
    if handle == 0 {
        log::warn!(target: "d3d", "CreateDepthStencilView: null descriptor handle");
        return;
    }
    let Some(r) = resource::from_ptr(res) else {
        write_target(handle, TargetView::NULL);
        return;
    };
    let d = if desc.is_null() { None } else { Some(std::ptr::read_unaligned(desc)) };
    match TargetView::dsv(&r.r, d.as_ref()) {
        Ok(v) => write_target(handle, v),
        Err(e) => {
            log::warn!(target: "d3d", "CreateDepthStencilView({:?}, {d:?}): {e}", r.r.desc);
            write_target(handle, TargetView::NULL);
        }
    }
}

// ---------------------------------------------------------------------------
// CBV / SRV / UAV / sampler descriptors (24-byte IR entries)
// ---------------------------------------------------------------------------

use d3d12_metal::metal::views;

unsafe fn write_entry(handle: usize, e: views::Entry) {
    std::ptr::copy_nonoverlapping(e.as_ptr(), handle as *mut u8, e.len());
}

unsafe fn write_or_null(what: &str, handle: usize, r: Result<views::Entry, String>) {
    match r {
        Ok(e) => write_entry(handle, e),
        Err(e) => {
            log::warn!(target: "d3d", "{what}: {e} (null descriptor written)");
            write_entry(handle, [0; 24]);
        }
    }
}

pub unsafe extern "win64" fn create_constant_buffer_view(_dev: *mut c_void, desc: *const (u64, u32), handle: usize) {
    tr!(
        let (va, size) = if desc.is_null() { (0, 0) } else { std::ptr::read_unaligned(desc) };
        Call::CreateConstantBufferView { buffer_location: va, size, dest: handle as u64 }
    );
    if handle == 0 {
        return;
    }
    let e = if desc.is_null() {
        [0; 24]
    } else {
        let (va, size) = std::ptr::read_unaligned(desc);
        views::cbv_entry(va, size)
    };
    write_entry(handle, e);
    bump_desc_heap_gen();
}

pub unsafe extern "win64" fn create_shader_resource_view(_dev: *mut c_void, res: *mut c_void, desc: *const [u8; 40], handle: usize) {
    tr!(Call::CreateShaderResourceView { resource: res as u64, desc: tr::opt(desc), dest: handle as u64 });
    if handle == 0 {
        return;
    }
    let r = resource::from_ptr(res);
    if r.is_none() && !res.is_null() {
        log::error!(target: "d3d", "CreateShaderResourceView: foreign resource {res:?}");
    }
    let d = if desc.is_null() { None } else { Some(std::ptr::read_unaligned(desc)) };
    write_or_null("CreateShaderResourceView", handle, views::srv_entry(r.map(|r| &*r.r), d.as_ref()));
    bump_desc_heap_gen();
}

pub unsafe extern "win64" fn create_unordered_access_view(_dev: *mut c_void, res: *mut c_void, counter: *mut c_void, desc: *const [u8; 40], handle: usize) {
    tr!(Call::CreateUnorderedAccessView { resource: res as u64, counter: counter as u64, desc: tr::opt(desc), dest: handle as u64 });
    if handle == 0 {
        return;
    }
    let r = resource::from_ptr(res);
    if r.is_none() && !res.is_null() {
        log::error!(target: "d3d", "CreateUnorderedAccessView: foreign resource {res:?}");
        d3d12_metal::metal::descriptor::record_uav(handle, None, None);
        write_entry(handle, [0; 24]);
        bump_desc_heap_gen();
        return;
    }
    let c = resource::from_ptr(counter);
    if c.is_none() && !counter.is_null() {
        log::error!(target: "d3d", "CreateUnorderedAccessView: foreign counter resource {counter:?}");
        d3d12_metal::metal::descriptor::record_uav(handle, None, None);
        write_entry(handle, [0; 24]);
        bump_desc_heap_gen();
        return;
    }
    let d = if desc.is_null() { None } else { Some(std::ptr::read_unaligned(desc)) };
    let res_ref = r.map(|r| &*r.r);
    let ctr_ref = c.map(|c| (&*c.r, 0u64));
    match views::uav_entry_with_counter(res_ref, ctr_ref, d.as_ref()) {
        Ok(e) => {
            d3d12_metal::metal::descriptor::record_uav(handle, res_ref, d.as_ref());
            write_entry(handle, e);
        }
        Err(e) => {
            log::warn!(target: "d3d", "CreateUnorderedAccessView: {e} (null descriptor written)");
            d3d12_metal::metal::descriptor::record_uav(handle, None, None);
            write_entry(handle, [0; 24]);
        }
    }
    bump_desc_heap_gen();
}

pub unsafe extern "win64" fn create_sampler(_dev: *mut c_void, desc: *const views::SamplerDesc, handle: usize) {
    tr!(Call::CreateSampler { desc: tr::raw(desc), dest: handle as u64 });
    if handle == 0 || desc.is_null() {
        return;
    }
    write_or_null("CreateSampler", handle, views::sampler_entry(&std::ptr::read_unaligned(desc)));
}

#[inline(always)]
fn descriptor_size(ty: u32) -> usize {
    match ty {
        heap_kind::CBV_SRV_UAV | heap_kind::SAMPLER => RESOURCE_DESCRIPTOR_SIZE as usize,
        heap_kind::RTV | heap_kind::DSV => TARGET_DESCRIPTOR_SIZE as usize,
        _ => RESOURCE_DESCRIPTOR_SIZE as usize,
    }
}

#[inline(never)]
pub unsafe extern "win64" fn copy_descriptors_simple(_dev: *mut c_void, n: u32, dst: usize, src: usize, ty: u32) {
    tr!(Call::CopyDescriptorsSimple { count: n, dest: dst as u64, src: src as u64, heap_type: ty });
    if n == 0 || dst == 0 || src == 0 {
        return;
    }
    let sz = descriptor_size(ty);
    if n == 1 && sz == RESOURCE_DESCRIPTOR_SIZE as usize {
        copy_one_entry(dst, src);
    } else {
        std::ptr::copy(src as *const u8, dst as *mut u8, n as usize * sz);
    }
    if ty == heap_kind::CBV_SRV_UAV {
        bump_desc_heap_gen();
    }
}

/// One CBV/SRV/UAV/sampler descriptor (24 bytes) as three 8-byte moves: all loads before the
/// stores (same result as `memmove` for overlapping handles) and no partial or overlapping
/// stores into the write-combined shader-visible heap memory.
#[inline(always)]
unsafe fn copy_one_entry(dst: usize, src: usize) {
    let w = std::ptr::read_unaligned(src as *const [u64; 3]);
    std::ptr::write_unaligned(dst as *mut [u64; 3], w);
}

#[inline(always)]
pub unsafe fn copy_one_resource_descriptor(dst: usize, src: usize) {
    copy_one_entry(dst, src);
    bump_desc_heap_gen();
}

/// Inline `CopyDescriptorsSimple(1, dst, src, CBV_SRV_UAV)` for hot native callers that
/// resolved the device method to [`copy_descriptors_simple`]: `false` when the call must go
/// through it instead (API trace recording is on).
#[inline(always)]
pub unsafe fn copy_one_resource_descriptor_inline(dst: usize, src: usize) -> bool {
    if tr::on() {
        return false;
    }
    if dst != 0 && src != 0 {
        copy_one_resource_descriptor(dst, src);
    }
    true
}

/// Scatter/gather copy of descriptor ranges (sizes NULL = ranges of 1).
pub unsafe extern "win64" fn copy_descriptors(
    _dev: *mut c_void,
    n_dst: u32,
    dst_starts: *const usize,
    dst_sizes: *const u32,
    n_src: u32,
    src_starts: *const usize,
    src_sizes: *const u32,
    ty: u32,
) {
    if n_src == 0 || n_dst == 0 || src_starts.is_null() || dst_starts.is_null() {
        return;
    }
    let sz = descriptor_size(ty);
    let range = |sizes: *const u32, i: usize| -> usize {
        if sizes.is_null() {
            1
        } else {
            std::ptr::read_unaligned(sizes.add(i)) as usize
        }
    };
    let n_dst = n_dst as usize;
    let mut di = 0usize;
    let mut doff = 0usize;
    let mut dlen = range(dst_sizes, 0);
    let mut dst_base = std::ptr::read_unaligned(dst_starts);
    let mut copied = false;
    'sources: for si in 0..n_src as usize {
        let mut src_rem = range(src_sizes, si);
        if src_rem == 0 {
            continue;
        }
        let src_base = std::ptr::read_unaligned(src_starts.add(si));
        let mut soff = 0usize;
        while src_rem > 0 {
            while di < n_dst && doff >= dlen {
                di += 1;
                doff = 0;
                if di < n_dst {
                    dlen = range(dst_sizes, di);
                    dst_base = std::ptr::read_unaligned(dst_starts.add(di));
                }
            }
            if di >= n_dst {
                log::warn!(target: "d3d", "CopyDescriptors: destination ranges smaller than source ranges");
                break 'sources;
            }
            let chunk = src_rem.min(dlen - doff);
            let s_addr = src_base + soff * sz;
            let d_addr = dst_base + doff * sz;
            if chunk == 1 && sz == RESOURCE_DESCRIPTOR_SIZE as usize {
                copy_one_entry(d_addr, s_addr);
            } else {
                std::ptr::copy(s_addr as *const u8, d_addr as *mut u8, chunk * sz);
            }
            copied = true;
            soff += chunk;
            doff += chunk;
            src_rem -= chunk;
        }
    }
    if copied && ty == heap_kind::CBV_SRV_UAV {
        bump_desc_heap_gen();
    }
}

pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            get_desc as *const () as usize,
            get_cpu_start as *const () as usize,
            get_gpu_start as *const () as usize,
            create_render_target_view as *const () as usize,
            create_depth_stencil_view as *const () as usize,
            create_constant_buffer_view as *const () as usize,
            create_shader_resource_view as *const () as usize,
            create_unordered_access_view as *const () as usize,
            create_sampler as *const () as usize,
            copy_descriptors_simple as *const () as usize,
            copy_descriptors as *const () as usize,
        ]
        .into_boxed_slice()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use d3d12_metal::metal::resource::Resource as MtlResource;
    use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
    use std::sync::Arc;

    #[test]
    fn create_unordered_access_view_wires_counter_and_rejects_invalid() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let bdesc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 8192,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        };
        let data_mtl = Arc::new(MtlResource::new_committed_default(&bdesc, &hp, "uav_com_data").unwrap());
        let ctr_mtl = Arc::new(MtlResource::new_committed_default(&bdesc, &hp, "uav_com_ctr").unwrap());
        let data_va = data_mtl.gpu_va;
        let data_com = resource::Resource::wrap(data_mtl, 0, 0) as *mut c_void;
        let ctr_com = resource::Resource::wrap(ctr_mtl, 0, 0) as *mut c_void;

        let mut slot = [0u8; 24];
        let handle = slot.as_mut_ptr() as usize;

        // Structured buffer UAV with CounterOffsetInBytes = 4096.
        let mut desc = [0u8; 40];
        desc[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
        desc[16..20].copy_from_slice(&16u32.to_le_bytes()); // NumElements = 16
        desc[20..24].copy_from_slice(&16u32.to_le_bytes()); // StructureByteStride = 16
        desc[24..32].copy_from_slice(&4096u64.to_le_bytes()); // CounterOffsetInBytes = 4096
        unsafe {
            create_unordered_access_view(std::ptr::null_mut(), data_com, ctr_com, &desc, handle);
        }
        let gpu_va = u64::from_le_bytes(slot[0..8].try_into().unwrap());
        let tex_id = u64::from_le_bytes(slot[8..16].try_into().unwrap());
        let meta = u64::from_le_bytes(slot[16..24].try_into().unwrap());
        assert_eq!(gpu_va, data_va);
        assert_ne!(tex_id, 0, "counter texture-buffer view ID must be written");
        assert_eq!(meta & 0xffff_ffff, 256);
        assert_eq!(meta >> 63, 0);

        // Foreign non-null counter pointer must write a null descriptor (never silently drop).
        let mut fake = [0usize; 4];
        unsafe {
            create_unordered_access_view(
                std::ptr::null_mut(),
                data_com,
                fake.as_mut_ptr() as *mut c_void,
                &desc,
                handle,
            );
        }
        assert_eq!(slot, [0u8; 24]);

        // Counter on raw buffer UAV must fail honestly and write a null descriptor.
        let mut raw_desc = [0u8; 40];
        raw_desc[0..4].copy_from_slice(&39u32.to_le_bytes());
        raw_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
        raw_desc[16..20].copy_from_slice(&16u32.to_le_bytes());
        raw_desc[32..36].copy_from_slice(&1u32.to_le_bytes()); // RAW flag
        unsafe {
            create_unordered_access_view(std::ptr::null_mut(), data_com, ctr_com, &raw_desc, handle);
        }
        assert_eq!(slot, [0u8; 24]);

        unsafe {
            com_release(data_com);
            com_release(ctr_com);
        }
    }
}
