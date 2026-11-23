//! `MFCreateDXGISurfaceBuffer` (`IMFMediaBuffer` + `IMF2DBuffer2` + `IMFDXGIBuffer`)
//! COM implementation for `mfplat.dll`.

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::com::{
    com_call_add_ref, com_call_qi, com_call_release, read_guid, register_vtable, write_null_out,
    Guid, IID_IUnknown,
};
use crate::types::{BOOL, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_POINTER, FALSE, HRESULT, S_OK, TRUE};

use super::buffer::copy_image_rows;
use super::{
    IID_IMF2DBuffer, IID_IMF2DBuffer2, IID_IMFDXGIBuffer, IID_IMFMediaBuffer, MF_E_INVALIDREQUEST,
    MF_E_NOT_FOUND,
};

struct DxgiSurfBufState {
    resource: usize,     // AddRef'd surface/resource COM pointer
    dxgi_surface: usize, // Optional AddRef'd IDXGISurface* for live Map/Unmap
    subresource_index: u32,
    bottom_up: bool,
    height: u32,
    row_bytes: u32,
    max_len: u32,
    cur_len: u32,
    locked_ptr: usize,
    locked_base: usize,
    locked_pitch: i32,
    mapped_dxgi: bool,
    lock_count: u32,
    fallback_buf: Vec<u8>,
    unknowns: HashMap<Guid, usize>, // AddRef'd IUnknown* keyed by Guid
}

impl Drop for DxgiSurfBufState {
    fn drop(&mut self) {
        if self.mapped_dxgi && self.dxgi_surface != 0 {
            unsafe { dxgi_surface_unmap(self.dxgi_surface as *mut c_void) };
            self.mapped_dxgi = false;
        }
        if self.dxgi_surface != 0 {
            unsafe { com_call_release(self.dxgi_surface as *mut c_void) };
            self.dxgi_surface = 0;
        }
        if self.resource != 0 {
            unsafe { com_call_release(self.resource as *mut c_void) };
            self.resource = 0;
        }
        for (_, p) in self.unknowns.drain() {
            if p != 0 {
                unsafe { com_call_release(p as *mut c_void) };
            }
        }
    }
}

#[repr(C)]
struct DxgiSurfBufObj {
    vtbl_media: *const usize, // offset 0: IMFMediaBuffer (canonical IUnknown)
    vtbl_2d: *const usize,    // offset 8: IMF2DBuffer2
    vtbl_dxgi: *const usize,  // offset 16: IMFDXGIBuffer
    refs: AtomicU32,
    state: Mutex<DxgiSurfBufState>,
}

const DXGI_SURF_2D_OFFSET: usize = std::mem::offset_of!(DxgiSurfBufObj, vtbl_2d);
const DXGI_SURF_DXGI_OFFSET: usize = std::mem::offset_of!(DxgiSurfBufObj, vtbl_dxgi);

#[repr(C)]
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
struct DxgiSurfaceDescRaw {
    width: u32,
    height: u32,
    format: u32,
    sample_count: u32,
    sample_quality: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DxgiMappedRectRaw {
    pitch: i32,
    bits: *mut u8,
}

unsafe fn dxgi_surface_get_desc(surf: *mut c_void) -> Option<DxgiSurfaceDescRaw> {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut DxgiSurfaceDescRaw) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(8)));
    let mut desc = DxgiSurfaceDescRaw::default();
    if f(surf, &mut desc) >= 0 {
        Some(desc)
    } else {
        None
    }
}

unsafe fn dxgi_surface_map(surf: *mut c_void, flags: u32) -> Result<DxgiMappedRectRaw, HRESULT> {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut DxgiMappedRectRaw, u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(9)));
    let mut rect = DxgiMappedRectRaw {
        pitch: 0,
        bits: std::ptr::null_mut(),
    };
    let hr = f(surf, &mut rect, flags);
    if hr < 0 { Err(hr) } else { Ok(rect) }
}

unsafe fn dxgi_surface_unmap(surf: *mut c_void) -> HRESULT {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(10)));
    f(surf)
}

#[inline]
unsafe fn dxgibuf_from_offset<'a>(this: usize, off: usize) -> Option<&'a DxgiSurfBufObj> {
    if this < off {
        None
    } else {
        Some(&*((this - off) as *const DxgiSurfBufObj))
    }
}

unsafe fn dxgibuf_qi_inner(
    obj: &DxgiSurfBufObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let base = obj as *const DxgiSurfBufObj as usize;
    if iid == IID_IUnknown || iid == IID_IMFMediaBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        S_OK
    } else if iid == IID_IMF2DBuffer || iid == IID_IMF2DBuffer2 {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + DXGI_SURF_2D_OFFSET) as *mut c_void);
        S_OK
    } else if iid == IID_IMFDXGIBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + DXGI_SURF_DXGI_OFFSET) as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn dxgibuf_release_inner(obj: *mut DxgiSurfBufObj) -> u32 {
    let prev = (*obj).refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(obj));
    }
    next
}

impl DxgiSurfBufState {
    unsafe fn acquire_lock(&mut self) -> Result<(*mut u8, i32, *mut u8, u32), HRESULT> {
        if self.lock_count > 0 && self.locked_ptr != 0 {
            self.lock_count += 1;
            return Ok((
                self.locked_ptr as *mut u8,
                self.locked_pitch,
                self.locked_base as *mut u8,
                self.max_len,
            ));
        }
        // Prefer live IDXGISurface::Map(READ | WRITE) when wrapping a real DXGI surface.
        if self.dxgi_surface != 0 {
            const DXGI_MAP_READ_WRITE: u32 = 1 | 2;
            if let Ok(rect) = dxgi_surface_map(self.dxgi_surface as *mut c_void, DXGI_MAP_READ_WRITE)
            {
                self.mapped_dxgi = true;
                self.lock_count = 1;
                let abs_pitch = rect.pitch.unsigned_abs();
                let total = abs_pitch.saturating_mul(self.height.max(1));
                self.max_len = self.max_len.max(total);
                let (scan0, pitch) = if self.bottom_up && self.height > 1 {
                    let off = (self.height as usize - 1) * (abs_pitch as usize);
                    (rect.bits.add(off), -(abs_pitch as i32))
                } else {
                    (rect.bits, rect.pitch)
                };
                self.locked_ptr = scan0 as usize;
                self.locked_base = rect.bits as usize;
                self.locked_pitch = pitch;
                return Ok((scan0, pitch, rect.bits, self.max_len));
            }
        }
        // Fallback: if resource is a CPU-mappable D3D12 buffer, lock directly.
        if let Some(d3d_res) = crate::d3d12::resource::from_ptr(self.resource as *mut c_void) {
            if let Some(p) = d3d_res.r.cpu_ptr() {
                let len = d3d_res.r.desc.Width.min(u32::MAX as u64) as u32;
                self.max_len = len;
                self.lock_count = 1;
                self.locked_ptr = p as usize;
                self.locked_base = p as usize;
                self.locked_pitch = self.row_bytes.max(len) as i32;
                return Ok((p, self.locked_pitch, p, len));
            }
        }
        // Fallback shadow linear buffer.
        let need = self
            .max_len
            .max(self.row_bytes.saturating_mul(self.height.max(1))) as usize;
        if self.fallback_buf.len() < need {
            self.fallback_buf.resize(need, 0);
        }
        let base = self.fallback_buf.as_mut_ptr();
        let stride = self.row_bytes.max(1);
        let (scan0, pitch) = if self.bottom_up && self.height > 1 {
            let off = (self.height as usize - 1) * (stride as usize);
            (base.add(off), -(stride as i32))
        } else {
            (base, stride as i32)
        };
        self.lock_count = 1;
        self.locked_ptr = scan0 as usize;
        self.locked_base = base as usize;
        self.locked_pitch = pitch;
        Ok((scan0, pitch, base, self.max_len))
    }

    unsafe fn release_lock(&mut self) {
        if self.lock_count == 0 {
            return;
        }
        self.lock_count -= 1;
        if self.lock_count == 0 {
            if self.mapped_dxgi && self.dxgi_surface != 0 {
                let _ = dxgi_surface_unmap(self.dxgi_surface as *mut c_void);
                self.mapped_dxgi = false;
            }
            self.locked_ptr = 0;
            self.locked_base = 0;
        }
    }
}

// Primary IMFMediaBuffer slots (0..7)
unsafe extern "win64" fn dxgisurf_media_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_media_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_media_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    dxgibuf_release_inner(this as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_media_lock(
    this: usize,
    ppb_buffer: *mut *mut u8,
    pcb_max_length: *mut u32,
    pcb_current_length: *mut u32,
) -> HRESULT {
    if ppb_buffer.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((_, _, base, max_len)) => {
            std::ptr::write_unaligned(ppb_buffer, base);
            if !pcb_max_length.is_null() {
                std::ptr::write_unaligned(pcb_max_length, max_len);
            }
            if !pcb_current_length.is_null() {
                std::ptr::write_unaligned(pcb_current_length, st.cur_len);
            }
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_media_unlock(this: usize) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    obj.state.lock().unwrap().release_lock();
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_get_current_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().cur_len);
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_set_current_length(this: usize, len: u32) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.max_len > 0 && len > st.max_len {
        return E_INVALIDARG;
    }
    st.cur_len = len;
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_get_max_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().max_len);
    S_OK
}

// Secondary IMF2DBuffer2 slots (0..11)
unsafe extern "win64" fn dxgisurf_2d_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_2d_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_2d_release(this: usize) -> u32 {
    if this < DXGI_SURF_2D_OFFSET {
        return 0;
    }
    dxgibuf_release_inner((this - DXGI_SURF_2D_OFFSET) as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_2d_lock2d(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((scan0, pitch, _, _)) => {
            std::ptr::write_unaligned(ppb_scanline0, scan0);
            std::ptr::write_unaligned(pl_pitch, pitch);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_2d_unlock2d(this: usize) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    obj.state.lock().unwrap().release_lock();
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_get_scanline0_and_pitch(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    if st.lock_count == 0 || st.locked_ptr == 0 {
        return MF_E_INVALIDREQUEST;
    }
    std::ptr::write_unaligned(ppb_scanline0, st.locked_ptr as *mut u8);
    std::ptr::write_unaligned(pl_pitch, st.locked_pitch);
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_is_contiguous_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let contig = !st.bottom_up && st.dxgi_surface == 0;
    std::ptr::write_unaligned(out, if contig { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_get_contiguous_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let len = st.row_bytes.saturating_mul(st.height.max(1)).max(st.max_len);
    std::ptr::write_unaligned(out, len);
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_contiguous_copy_to(
    this: usize,
    dest: *mut u8,
    dest_len: u32,
) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.row_bytes.saturating_mul(st.height.max(1));
    if dest_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let hr = copy_image_rows(
        dest,
        st.row_bytes as i32,
        scan0,
        pitch,
        st.row_bytes,
        st.height.max(1),
    );
    st.release_lock();
    hr
}

unsafe extern "win64" fn dxgisurf_2d_contiguous_copy_from(
    this: usize,
    src: *const u8,
    src_len: u32,
) -> HRESULT {
    if src.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.row_bytes.saturating_mul(st.height.max(1));
    if src_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let hr = copy_image_rows(
        scan0,
        pitch,
        src,
        st.row_bytes as i32,
        st.row_bytes,
        st.height.max(1),
    );
    st.release_lock();
    if hr >= 0 {
        st.cur_len = needed;
    }
    hr
}

unsafe extern "win64" fn dxgisurf_2d_lock2d_size(
    this: usize,
    _flags: u32,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
    ppb_buffer_start: *mut *mut u8,
    pcb_buffer_length: *mut u32,
) -> HRESULT {
    if ppb_scanline0.is_null()
        || pl_pitch.is_null()
        || ppb_buffer_start.is_null()
        || pcb_buffer_length.is_null()
    {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((scan0, pitch, base, max_len)) => {
            std::ptr::write_unaligned(ppb_scanline0, scan0);
            std::ptr::write_unaligned(pl_pitch, pitch);
            std::ptr::write_unaligned(ppb_buffer_start, base);
            std::ptr::write_unaligned(pcb_buffer_length, max_len);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_2d_copy2d_to(this: usize, dest_2d: *mut c_void) -> HRESULT {
    if dest_2d.is_null() {
        return E_POINTER;
    }
    if dest_2d as usize == this {
        return S_OK;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let (src_scan0, src_pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let dvt = std::ptr::read_unaligned(dest_2d as *const *const usize);
    let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(3)));
    let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(4)));
    let mut dst_scan0 = std::ptr::null_mut();
    let mut dst_pitch = 0i32;
    let lhr = lock2d(dest_2d, &mut dst_scan0, &mut dst_pitch);
    let hr = if lhr < 0 {
        lhr
    } else {
        let chr = copy_image_rows(
            dst_scan0,
            dst_pitch,
            src_scan0,
            src_pitch,
            st.row_bytes,
            st.height.max(1),
        );
        let _ = unlock2d(dest_2d);
        chr
    };
    st.release_lock();
    hr
}

// Secondary IMFDXGIBuffer slots (0..6)
unsafe extern "win64" fn dxgisurf_dxgi_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_dxgi_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_dxgi_release(this: usize) -> u32 {
    if this < DXGI_SURF_DXGI_OFFSET {
        return 0;
    }
    dxgibuf_release_inner((this - DXGI_SURF_DXGI_OFFSET) as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_get_resource(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let (Some(obj), Some(iid)) = (
        dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET),
        read_guid(riid),
    ) else {
        return E_INVALIDARG;
    };
    let res = {
        let st = obj.state.lock().unwrap();
        if st.resource == 0 {
            return E_FAIL;
        }
        com_call_add_ref(st.resource as *mut c_void);
        st.resource as *mut c_void
    };
    let hr = com_call_qi(res, &iid, ppv);
    com_call_release(res);
    hr
}

unsafe extern "win64" fn dxgisurf_get_subresource_index(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().subresource_index);
    S_OK
}

unsafe extern "win64" fn dxgisurf_get_unknown(
    this: usize,
    guid_key: *const Guid,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let (Some(obj), Some(k), Some(iid)) = (
        dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET),
        read_guid(guid_key),
        read_guid(riid),
    ) else {
        return E_INVALIDARG;
    };
    let unk = {
        let st = obj.state.lock().unwrap();
        match st.unknowns.get(&k).copied() {
            Some(p) if p != 0 => {
                com_call_add_ref(p as *mut c_void);
                p as *mut c_void
            }
            _ => return MF_E_NOT_FOUND,
        }
    };
    let hr = com_call_qi(unk, &iid, ppv);
    com_call_release(unk);
    hr
}

unsafe extern "win64" fn dxgisurf_set_unknown(
    this: usize,
    guid_key: *const Guid,
    p_unk: *mut c_void,
) -> HRESULT {
    let (Some(obj), Some(k)) = (
        dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET),
        read_guid(guid_key),
    ) else {
        return E_INVALIDARG;
    };
    if !p_unk.is_null() {
        com_call_add_ref(p_unk);
    }
    let old = {
        let mut st = obj.state.lock().unwrap();
        if p_unk.is_null() {
            st.unknowns.remove(&k)
        } else {
            st.unknowns.insert(k, p_unk as usize)
        }
    };
    if let Some(prev) = old {
        if prev != 0 {
            com_call_release(prev as *mut c_void);
        }
    }
    S_OK
}

fn dxgisurf_media_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_media_qi as *const () as usize,
            dxgisurf_media_add_ref as *const () as usize,
            dxgisurf_media_release as *const () as usize,
            dxgisurf_media_lock as *const () as usize,
            dxgisurf_media_unlock as *const () as usize,
            dxgisurf_media_get_current_length as *const () as usize,
            dxgisurf_media_set_current_length as *const () as usize,
            dxgisurf_media_get_max_length as *const () as usize,
        ])
    })
}

fn dxgisurf_2d_vtable() -> &'static [usize; 12] {
    static VTBL: OnceLock<[usize; 12]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_2d_qi as *const () as usize,
            dxgisurf_2d_add_ref as *const () as usize,
            dxgisurf_2d_release as *const () as usize,
            dxgisurf_2d_lock2d as *const () as usize,
            dxgisurf_2d_unlock2d as *const () as usize,
            dxgisurf_2d_get_scanline0_and_pitch as *const () as usize,
            dxgisurf_2d_is_contiguous_format as *const () as usize,
            dxgisurf_2d_get_contiguous_length as *const () as usize,
            dxgisurf_2d_contiguous_copy_to as *const () as usize,
            dxgisurf_2d_contiguous_copy_from as *const () as usize,
            dxgisurf_2d_lock2d_size as *const () as usize,
            dxgisurf_2d_copy2d_to as *const () as usize,
        ])
    })
}

fn dxgisurf_dxgi_vtable() -> &'static [usize; 7] {
    static VTBL: OnceLock<[usize; 7]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_dxgi_qi as *const () as usize,
            dxgisurf_dxgi_add_ref as *const () as usize,
            dxgisurf_dxgi_release as *const () as usize,
            dxgisurf_get_resource as *const () as usize,
            dxgisurf_get_subresource_index as *const () as usize,
            dxgisurf_get_unknown as *const () as usize,
            dxgisurf_set_unknown as *const () as usize,
        ])
    })
}

pub(super) unsafe fn create_dxgi_surface_buffer(
    riid: *const Guid,
    punk_surface: *mut c_void,
    u_subresource_index: u32,
    f_bottom_up_when_linear: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_buffer);
    let Some(req_iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if punk_surface.is_null() {
        return E_INVALIDARG;
    }

    // Verify the surface supports the requested IID (e.g., ID3D11Texture2D, IDXGISurface, ID3D12Resource).
    let mut queried: *mut c_void = std::ptr::null_mut();
    let qhr = com_call_qi(punk_surface, &req_iid, &mut queried);
    if qhr < 0 || queried.is_null() {
        return if qhr < 0 { qhr } else { E_NOINTERFACE };
    }

    // Also probe IDXGISurface for live 2D dimensions & Map/Unmap support.
    let mut dxgi_surf: *mut c_void = std::ptr::null_mut();
    let _ = com_call_qi(punk_surface, &crate::d3d11::types::IID_IDXGISurface, &mut dxgi_surf);

    let (height, row_bytes, max_len) = if !dxgi_surf.is_null() {
        if let Some(d) = dxgi_surface_get_desc(dxgi_surf) {
            let fi = d3d12_metal::format::info(d.format);
            let bpp = if fi.bytes > 0 { fi.bytes as u32 } else { 4 };
            let rb = d.width.saturating_mul(bpp);
            let total = rb.saturating_mul(d.height.max(1));
            (d.height.max(1), rb, total)
        } else {
            (1, 0, 0)
        }
    } else if let Some(d3d_res) = crate::d3d12::resource::from_ptr(queried) {
        let w = d3d_res.r.desc.Width.min(u32::MAX as u64) as u32;
        let h = d3d_res.r.desc.Height.max(1);
        (h, w, w.saturating_mul(h))
    } else {
        (1, 0, 0)
    };

    let obj = Box::new(DxgiSurfBufObj {
        vtbl_media: dxgisurf_media_vtable().as_ptr(),
        vtbl_2d: dxgisurf_2d_vtable().as_ptr(),
        vtbl_dxgi: dxgisurf_dxgi_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        state: Mutex::new(DxgiSurfBufState {
            resource: queried as usize,
            dxgi_surface: dxgi_surf as usize,
            subresource_index: u_subresource_index,
            bottom_up: f_bottom_up_when_linear != 0,
            height,
            row_bytes,
            max_len,
            cur_len: max_len,
            locked_ptr: 0,
            locked_base: 0,
            locked_pitch: row_bytes as i32,
            mapped_dxgi: false,
            lock_count: 0,
            fallback_buf: Vec::new(),
            unknowns: HashMap::new(),
        }),
    });
    std::ptr::write_unaligned(pp_buffer, Box::into_raw(obj) as *mut c_void);
    S_OK
}
