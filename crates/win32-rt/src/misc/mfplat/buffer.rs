//! `IMFMediaBuffer`, `IMF2DBuffer`, `IMF2DBuffer2`, and `MFCopyImage` helpers
//! for `mfplat.dll`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::com::{read_guid, register_vtable, write_null_out, Guid, IID_IUnknown};
use crate::types::{BOOL, E_INVALIDARG, E_NOINTERFACE, E_POINTER, FALSE, HRESULT, S_OK, TRUE};

use super::{IID_IMF2DBuffer, IID_IMF2DBuffer2, IID_IMFMediaBuffer, MF_E_INVALIDREQUEST};

/// Strided 2D image copy supporting both positive (top-down) and negative (bottom-up) strides.
pub(super) unsafe fn copy_image_rows(
    dst: *mut u8,
    dst_stride: i32,
    src: *const u8,
    src_stride: i32,
    width_bytes: u32,
    lines: u32,
) -> HRESULT {
    if width_bytes == 0 || lines == 0 {
        return S_OK;
    }
    if dst.is_null() || src.is_null() {
        return E_INVALIDARG;
    }
    let w = width_bytes as usize;
    let ds = dst_stride as isize;
    let ss = src_stride as isize;
    for y in 0..lines as isize {
        let d_row = dst.offset(y * ds);
        let s_row = src.offset(y * ss);
        std::ptr::copy(s_row, d_row, w);
    }
    S_OK
}

struct MemBufState {
    /// Backing allocation; never reallocated after creation so `Lock` pointers stay stable.
    storage: Vec<u8>,
    data_offset: usize,
    max_len: u32,
    cur_len: u32,
    row_bytes: u32,
    height: u32,
    stride: u32, // positive magnitude per row
    bottom_up: bool,
    lock_count: u32,
}

impl MemBufState {
    fn new_1d(max_len: u32, alignment: usize) -> Self {
        let align = alignment.max(16).next_power_of_two();
        let alloc_len = (max_len as usize).saturating_add(align);
        let storage = vec![0u8; alloc_len];
        let base = storage.as_ptr() as usize;
        let aligned = (base + align - 1) & !(align - 1);
        let data_offset = aligned - base;
        Self {
            storage,
            data_offset,
            max_len,
            cur_len: 0,
            row_bytes: max_len,
            height: 1,
            stride: max_len,
            bottom_up: false,
            lock_count: 0,
        }
    }

    fn new_2d(row_bytes: u32, height: u32, stride: u32, bottom_up: bool) -> Result<Self, HRESULT> {
        let max_len = stride.checked_mul(height).ok_or(E_INVALIDARG)?;
        let mut s = Self::new_1d(max_len, 16);
        s.row_bytes = row_bytes;
        s.height = height.max(1);
        s.stride = stride;
        s.bottom_up = bottom_up;
        Ok(s)
    }

    fn data_ptr(&mut self) -> *mut u8 {
        unsafe { self.storage.as_mut_ptr().add(self.data_offset) }
    }

    fn scanline0_and_pitch(&mut self) -> (*mut u8, i32) {
        let base = self.data_ptr();
        if self.bottom_up && self.height > 1 {
            let off = (self.height as usize - 1) * (self.stride as usize);
            (unsafe { base.add(off) }, -(self.stride as i32))
        } else {
            (base, self.stride as i32)
        }
    }

    fn contiguous_len(&self) -> u32 {
        self.row_bytes.saturating_mul(self.height)
    }

    fn is_contiguous(&self) -> bool {
        !self.bottom_up && self.stride == self.row_bytes
    }
}

#[repr(C)]
struct MemBufObj {
    vtbl_media: *const usize, // offset 0: IMFMediaBuffer (canonical IUnknown)
    vtbl_2d: *const usize,    // offset 8: IMF2DBuffer2
    refs: AtomicU32,
    state: Mutex<MemBufState>,
}

const MEMBUF_2D_OFFSET: usize = std::mem::offset_of!(MemBufObj, vtbl_2d);

#[inline]
unsafe fn membuf_from_primary<'a>(this: usize) -> Option<&'a MemBufObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const MemBufObj))
    }
}

#[inline]
unsafe fn membuf_from_2d<'a>(this: usize) -> Option<&'a MemBufObj> {
    if this < MEMBUF_2D_OFFSET {
        None
    } else {
        Some(&*((this - MEMBUF_2D_OFFSET) as *const MemBufObj))
    }
}

unsafe fn membuf_qi_inner(obj: &MemBufObj, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let base = obj as *const MemBufObj as usize;
    if iid == IID_IUnknown || iid == IID_IMFMediaBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        S_OK
    } else if iid == IID_IMF2DBuffer || iid == IID_IMF2DBuffer2 {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + MEMBUF_2D_OFFSET) as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn membuf_release_inner(obj: *mut MemBufObj) -> u32 {
    let prev = (*obj).refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(obj));
    }
    next
}

// Primary IMFMediaBuffer vtable methods (slots 0..7)
unsafe extern "win64" fn membuf_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    membuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn membuf_add_ref(this: usize) -> u32 {
    let Some(obj) = membuf_from_primary(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn membuf_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    membuf_release_inner(this as *mut MemBufObj)
}

unsafe extern "win64" fn membuf_lock(
    this: usize,
    ppb_buffer: *mut *mut u8,
    pcb_max_length: *mut u32,
    pcb_current_length: *mut u32,
) -> HRESULT {
    if ppb_buffer.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let ptr = st.data_ptr();
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_buffer, ptr);
    if !pcb_max_length.is_null() {
        std::ptr::write_unaligned(pcb_max_length, st.max_len);
    }
    if !pcb_current_length.is_null() {
        std::ptr::write_unaligned(pcb_current_length, st.cur_len);
    }
    S_OK
}

unsafe extern "win64" fn membuf_unlock(this: usize) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    st.lock_count = st.lock_count.saturating_sub(1);
    S_OK
}

unsafe extern "win64" fn membuf_get_current_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().cur_len);
    S_OK
}

unsafe extern "win64" fn membuf_set_current_length(this: usize, len: u32) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if len > st.max_len {
        return E_INVALIDARG;
    }
    st.cur_len = len;
    S_OK
}

unsafe extern "win64" fn membuf_get_max_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().max_len);
    S_OK
}

// Secondary IMF2DBuffer / IMF2DBuffer2 vtable methods (slots 0..11)
unsafe extern "win64" fn membuf2d_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let Some(obj) = membuf_from_2d(this) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    membuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn membuf2d_add_ref(this: usize) -> u32 {
    let Some(obj) = membuf_from_2d(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn membuf2d_release(this: usize) -> u32 {
    if this < MEMBUF_2D_OFFSET {
        return 0;
    }
    membuf_release_inner((this - MEMBUF_2D_OFFSET) as *mut MemBufObj)
}

unsafe extern "win64" fn membuf2d_lock2d(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let (scan0, pitch) = st.scanline0_and_pitch();
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    S_OK
}

unsafe extern "win64" fn membuf2d_unlock2d(this: usize) -> HRESULT {
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    st.lock_count = st.lock_count.saturating_sub(1);
    S_OK
}

unsafe extern "win64" fn membuf2d_get_scanline0_and_pitch(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.lock_count == 0 {
        return MF_E_INVALIDREQUEST;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    S_OK
}

unsafe extern "win64" fn membuf2d_is_contiguous_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    std::ptr::write_unaligned(out, if st.is_contiguous() { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn membuf2d_get_contiguous_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    std::ptr::write_unaligned(out, st.contiguous_len());
    S_OK
}

unsafe extern "win64" fn membuf2d_contiguous_copy_to(
    this: usize,
    dest: *mut u8,
    dest_len: u32,
) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.contiguous_len();
    if dest_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    let row_bytes = st.row_bytes;
    let height = st.height;
    copy_image_rows(dest, row_bytes as i32, scan0, pitch, row_bytes, height)
}

unsafe extern "win64" fn membuf2d_contiguous_copy_from(
    this: usize,
    src: *const u8,
    src_len: u32,
) -> HRESULT {
    if src.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.contiguous_len();
    if src_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    let row_bytes = st.row_bytes;
    let height = st.height;
    let hr = copy_image_rows(scan0, pitch, src, row_bytes as i32, row_bytes, height);
    if hr >= 0 {
        st.cur_len = st.max_len;
    }
    hr
}

unsafe extern "win64" fn membuf2d_lock2d_size(
    this: usize,
    _lock_flags: u32,
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
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let base = st.data_ptr();
    let (scan0, pitch) = st.scanline0_and_pitch();
    let max_len = st.max_len;
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    std::ptr::write_unaligned(ppb_buffer_start, base);
    std::ptr::write_unaligned(pcb_buffer_length, max_len);
    S_OK
}

unsafe extern "win64" fn membuf2d_copy2d_to(this: usize, dest_2d: *mut c_void) -> HRESULT {
    if dest_2d.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let (src_scan0, src_pitch, row_bytes, height) = {
        let mut st = obj.state.lock().unwrap();
        let (s0, p) = st.scanline0_and_pitch();
        (s0, p, st.row_bytes, st.height)
    };
    let dvt = std::ptr::read_unaligned(dest_2d as *const *const usize);
    let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(3)));
    let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(4)));
    let mut dst_scan0 = std::ptr::null_mut();
    let mut dst_pitch = 0i32;
    let hr = lock2d(dest_2d, &mut dst_scan0, &mut dst_pitch);
    if hr < 0 {
        return hr;
    }
    let copy_hr = copy_image_rows(dst_scan0, dst_pitch, src_scan0, src_pitch, row_bytes, height);
    let _ = unlock2d(dest_2d);
    copy_hr
}

fn membuf_media_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            membuf_qi as *const () as usize,
            membuf_add_ref as *const () as usize,
            membuf_release as *const () as usize,
            membuf_lock as *const () as usize,
            membuf_unlock as *const () as usize,
            membuf_get_current_length as *const () as usize,
            membuf_set_current_length as *const () as usize,
            membuf_get_max_length as *const () as usize,
        ])
    })
}

fn membuf_2d_vtable() -> &'static [usize; 12] {
    static VTBL: OnceLock<[usize; 12]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            membuf2d_qi as *const () as usize,
            membuf2d_add_ref as *const () as usize,
            membuf2d_release as *const () as usize,
            membuf2d_lock2d as *const () as usize,
            membuf2d_unlock2d as *const () as usize,
            membuf2d_get_scanline0_and_pitch as *const () as usize,
            membuf2d_is_contiguous_format as *const () as usize,
            membuf2d_get_contiguous_length as *const () as usize,
            membuf2d_contiguous_copy_to as *const () as usize,
            membuf2d_contiguous_copy_from as *const () as usize,
            membuf2d_lock2d_size as *const () as usize,
            membuf2d_copy2d_to as *const () as usize,
        ])
    })
}

fn create_membuf_obj(state: MemBufState) -> *mut c_void {
    let obj = Box::new(MemBufObj {
        vtbl_media: membuf_media_vtable().as_ptr(),
        vtbl_2d: membuf_2d_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        state: Mutex::new(state),
    });
    Box::into_raw(obj) as *mut c_void
}

pub(super) unsafe fn create_memory_buffer(
    cb_max_length: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_buffer, create_membuf_obj(MemBufState::new_1d(cb_max_length, 16)));
    S_OK
}

pub(super) unsafe fn create_aligned_memory_buffer(
    cb_max_length: u32,
    cb_alignment: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    // MF_*_BYTE_ALIGNMENT constants are `align - 1` masks (e.g. MF_16_BYTE_ALIGNMENT = 15).
    let align = (cb_alignment as usize).saturating_add(1).clamp(16, 4096);
    std::ptr::write_unaligned(
        pp_buffer,
        create_membuf_obj(MemBufState::new_1d(cb_max_length, align)),
    );
    S_OK
}

pub(super) unsafe fn create_2d_media_buffer(
    dw_width: u32,
    dw_height: u32,
    dw_fourcc: u32,
    f_bottom_up: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_buffer);
    if dw_width == 0 || dw_height == 0 {
        return E_INVALIDARG;
    }
    // Common FOURCC / D3DFORMAT bytes-per-pixel & height multipliers.
    let (row_bytes, total_rows) = match dw_fourcc {
        // D3DFMT_A8R8G8B8 (21), D3DFMT_X8R8G8B8 (22), AYUV
        21 | 22 | 0x5655_5941 => (dw_width.checked_mul(4), dw_height),
        // D3DFMT_R8G8B8 (20)
        20 => (dw_width.checked_mul(3), dw_height),
        // YUY2, UYVY, D3DFMT_R5G6B5 (23)
        0x3259_5559 | 0x5956_5955 | 23 => (dw_width.checked_mul(2), dw_height),
        // NV12, YV12, IYUV, I420 (4:2:0 planar: height * 3 / 2 rows of width bytes)
        0x3231_564E | 0x3231_5659 | 0x5655_5949 | 0x3032_3449 => {
            (Some(dw_width), dw_height.saturating_mul(3) / 2)
        }
        // P010 (16-bit 4:2:0)
        0x3031_3050 => (dw_width.checked_mul(2), dw_height.saturating_mul(3) / 2),
        _ => (dw_width.checked_mul(4), dw_height),
    };
    let Some(row_bytes) = row_bytes else {
        return E_INVALIDARG;
    };
    let stride = row_bytes.next_multiple_of(16);
    let state = match MemBufState::new_2d(row_bytes, total_rows, stride, f_bottom_up != 0) {
        Ok(s) => s,
        Err(hr) => return hr,
    };
    std::ptr::write_unaligned(pp_buffer, create_membuf_obj(state));
    S_OK
}

// ============================================================================
// IMFMediaBuffer COM vtable call helpers (shared by sample.rs & tests.rs)
// ============================================================================

/// Invoke `IMFMediaBuffer::Lock` through a buffer COM pointer's vtable.
pub(super) unsafe fn buf_lock(buf: *mut c_void) -> Result<(*mut u8, u32, u32), HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut u32, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(3)));
    let mut ptr = std::ptr::null_mut();
    let mut max_len = 0u32;
    let mut cur_len = 0u32;
    let hr = f(buf, &mut ptr, &mut max_len, &mut cur_len);
    if hr < 0 {
        Err(hr)
    } else {
        Ok((ptr, max_len, cur_len))
    }
}

pub(super) unsafe fn buf_unlock(buf: *mut c_void) -> HRESULT {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(4)));
    f(buf)
}

pub(super) unsafe fn buf_get_current_length(buf: *mut c_void) -> Result<u32, HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(5)));
    let mut len = 0u32;
    let hr = f(buf, &mut len);
    if hr < 0 { Err(hr) } else { Ok(len) }
}

pub(super) unsafe fn buf_set_current_length(buf: *mut c_void, len: u32) -> HRESULT {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(6)));
    f(buf, len)
}

pub(super) unsafe fn buf_get_max_length(buf: *mut c_void) -> Result<u32, HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(7)));
    let mut len = 0u32;
    let hr = f(buf, &mut len);
    if hr < 0 { Err(hr) } else { Ok(len) }
}
