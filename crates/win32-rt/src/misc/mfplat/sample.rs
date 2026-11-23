//! `IMFSample` COM implementation (inherits `IMFAttributes`, slots 33..46) for `mfplat.dll`.

use std::ffi::c_void;
use std::sync::atomic::AtomicU32;
use std::sync::{Mutex, OnceLock};

use crate::com::{com_call_add_ref, com_call_release, register_vtable, Guid};
use crate::types::{E_INVALIDARG, E_OUTOFMEMORY, E_POINTER, HRESULT, S_OK};

use super::attributes::{fill_attr_slots, qi_attr_common, AttrHeader, AttrStore};
use super::buffer::{
    buf_get_current_length, buf_get_max_length, buf_lock, buf_set_current_length, buf_unlock,
    create_memory_buffer,
};
use super::{
    E_UNEXPECTED, IID_IMFSample, MF_E_BUFFERTOOSMALL, MF_E_NO_SAMPLE_DURATION,
    MF_E_NO_SAMPLE_TIMESTAMP,
};

#[repr(C)]
struct SampleObj {
    hdr: AttrHeader,
    sample: Mutex<SampleState>,
}

#[derive(Default)]
struct SampleState {
    flags: u32,
    sample_time: Option<i64>,
    sample_duration: Option<i64>,
    buffers: Vec<usize>, // AddRef'd IMFMediaBuffer*
}

impl Drop for SampleState {
    fn drop(&mut self) {
        for p in self.buffers.drain(..) {
            if p != 0 {
                unsafe { com_call_release(p as *mut c_void) };
            }
        }
    }
}

#[inline]
unsafe fn sample_ref<'a>(this: usize) -> Option<&'a SampleObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const SampleObj))
    }
}

unsafe extern "win64" fn sample_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_attr_common(this, riid, ppv, Some(IID_IMFSample))
}

unsafe extern "win64" fn sample_get_flags(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, s.sample.lock().unwrap().flags);
    S_OK
}

unsafe extern "win64" fn sample_set_flags(this: usize, flags: u32) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().flags = flags;
    S_OK
}

unsafe extern "win64" fn sample_get_time(this: usize, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    match s.sample.lock().unwrap().sample_time {
        Some(t) => {
            std::ptr::write_unaligned(out, t);
            S_OK
        }
        None => MF_E_NO_SAMPLE_TIMESTAMP,
    }
}

unsafe extern "win64" fn sample_set_time(this: usize, t: i64) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().sample_time = Some(t);
    S_OK
}

unsafe extern "win64" fn sample_get_duration(this: usize, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    match s.sample.lock().unwrap().sample_duration {
        Some(d) => {
            std::ptr::write_unaligned(out, d);
            S_OK
        }
        None => MF_E_NO_SAMPLE_DURATION,
    }
}

unsafe extern "win64" fn sample_set_duration(this: usize, d: i64) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().sample_duration = Some(d);
    S_OK
}

unsafe extern "win64" fn sample_get_buffer_count(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, s.sample.lock().unwrap().buffers.len() as u32);
    S_OK
}

unsafe extern "win64" fn sample_get_buffer_by_index(
    this: usize,
    index: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let st = s.sample.lock().unwrap();
    let Some(&buf) = st.buffers.get(index as usize) else {
        return E_INVALIDARG;
    };
    com_call_add_ref(buf as *mut c_void);
    std::ptr::write_unaligned(out, buf as *mut c_void);
    S_OK
}

unsafe fn flatten_buffers_into(bufs: &[usize], dest: *mut c_void) -> HRESULT {
    let mut total = 0u32;
    for &b in bufs {
        match buf_get_current_length(b as *mut c_void) {
            Ok(len) => match total.checked_add(len) {
                Some(t) => total = t,
                None => return E_OUTOFMEMORY,
            },
            Err(hr) => return hr,
        }
    }
    let max_len = match buf_get_max_length(dest) {
        Ok(m) => m,
        Err(hr) => return hr,
    };
    if max_len < total {
        return MF_E_BUFFERTOOSMALL;
    }
    let (dst_ptr, _, _) = match buf_lock(dest) {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let mut offset = 0usize;
    let mut copy_hr = S_OK;
    for &b in bufs {
        let bp = b as *mut c_void;
        match buf_lock(bp) {
            Ok((src_ptr, _, cur_len)) => {
                let n = cur_len as usize;
                if n > 0 && !src_ptr.is_null() && !dst_ptr.is_null() {
                    std::ptr::copy_nonoverlapping(src_ptr, dst_ptr.add(offset), n);
                }
                offset += n;
                let _ = buf_unlock(bp);
            }
            Err(hr) => {
                copy_hr = hr;
                break;
            }
        }
    }
    let _ = buf_unlock(dest);
    if copy_hr < 0 {
        return copy_hr;
    }
    buf_set_current_length(dest, total)
}

unsafe extern "win64" fn sample_convert_to_contiguous_buffer(
    this: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        if st.buffers.is_empty() {
            return E_UNEXPECTED;
        }
        if st.buffers.len() == 1 {
            let single = st.buffers[0] as *mut c_void;
            com_call_add_ref(single);
            std::ptr::write_unaligned(out, single);
            return S_OK;
        }
        for &b in &st.buffers {
            com_call_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };

    let mut total = 0u32;
    let mut total_hr = S_OK;
    for &b in &snapshot {
        match buf_get_current_length(b as *mut c_void) {
            Ok(len) => match total.checked_add(len) {
                Some(t) => total = t,
                None => {
                    total_hr = E_OUTOFMEMORY;
                    break;
                }
            },
            Err(hr) => {
                total_hr = hr;
                break;
            }
        }
    }
    if total_hr < 0 {
        for b in snapshot {
            com_call_release(b as *mut c_void);
        }
        return total_hr;
    }

    let mut new_buf: *mut c_void = std::ptr::null_mut();
    let hr = create_memory_buffer(total, &mut new_buf);
    if hr < 0 {
        for b in snapshot {
            com_call_release(b as *mut c_void);
        }
        return hr;
    }
    let fhr = flatten_buffers_into(&snapshot, new_buf);
    for b in snapshot {
        com_call_release(b as *mut c_void);
    }
    if fhr < 0 {
        com_call_release(new_buf);
        return fhr;
    }

    // Replace sample's buffer list with [new_buf] (keep 1 ref in sample, 1 ref for caller).
    com_call_add_ref(new_buf);
    let old_bufs = {
        let mut st = s.sample.lock().unwrap();
        std::mem::replace(&mut st.buffers, vec![new_buf as usize])
    };
    for old in old_bufs {
        if old != 0 {
            com_call_release(old as *mut c_void);
        }
    }
    std::ptr::write_unaligned(out, new_buf);
    S_OK
}

unsafe extern "win64" fn sample_add_buffer(this: usize, buf: *mut c_void) -> HRESULT {
    if buf.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    com_call_add_ref(buf);
    s.sample.lock().unwrap().buffers.push(buf as usize);
    S_OK
}

unsafe extern "win64" fn sample_remove_buffer_by_index(this: usize, index: u32) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let removed = {
        let mut st = s.sample.lock().unwrap();
        if (index as usize) >= st.buffers.len() {
            return E_INVALIDARG;
        }
        st.buffers.remove(index as usize)
    };
    if removed != 0 {
        com_call_release(removed as *mut c_void);
    }
    S_OK
}

unsafe extern "win64" fn sample_remove_all_buffers(this: usize) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let old = {
        let mut st = s.sample.lock().unwrap();
        std::mem::take(&mut st.buffers)
    };
    for p in old {
        if p != 0 {
            com_call_release(p as *mut c_void);
        }
    }
    S_OK
}

unsafe extern "win64" fn sample_get_total_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        for &b in &st.buffers {
            com_call_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };
    let mut total = 0u32;
    let mut hr = S_OK;
    for b in snapshot {
        if hr >= 0 {
            match buf_get_current_length(b as *mut c_void) {
                Ok(len) => match total.checked_add(len) {
                    Some(t) => total = t,
                    None => hr = E_OUTOFMEMORY,
                },
                Err(e) => hr = e,
            }
        }
        com_call_release(b as *mut c_void);
    }
    if hr >= 0 {
        std::ptr::write_unaligned(out, total);
    }
    hr
}

unsafe extern "win64" fn sample_copy_to_buffer(this: usize, dest: *mut c_void) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        for &b in &st.buffers {
            com_call_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };
    let hr = flatten_buffers_into(&snapshot, dest);
    for b in snapshot {
        com_call_release(b as *mut c_void);
    }
    hr
}

pub(super) fn sample_vtable() -> &'static [usize; 47] {
    static VTBL: OnceLock<[usize; 47]> = OnceLock::new();
    VTBL.get_or_init(|| {
        let base = fill_attr_slots::<SampleObj>(sample_qi as *const () as usize);
        let mut full = [0usize; 47];
        full[..33].copy_from_slice(&base);
        full[33] = sample_get_flags as *const () as usize;
        full[34] = sample_set_flags as *const () as usize;
        full[35] = sample_get_time as *const () as usize;
        full[36] = sample_set_time as *const () as usize;
        full[37] = sample_get_duration as *const () as usize;
        full[38] = sample_set_duration as *const () as usize;
        full[39] = sample_get_buffer_count as *const () as usize;
        full[40] = sample_get_buffer_by_index as *const () as usize;
        full[41] = sample_convert_to_contiguous_buffer as *const () as usize;
        full[42] = sample_add_buffer as *const () as usize;
        full[43] = sample_remove_buffer_by_index as *const () as usize;
        full[44] = sample_remove_all_buffers as *const () as usize;
        full[45] = sample_get_total_length as *const () as usize;
        full[46] = sample_copy_to_buffer as *const () as usize;
        register_vtable(full)
    })
}

pub(super) unsafe fn create_sample(pp_imf_sample: *mut *mut c_void) -> HRESULT {
    if pp_imf_sample.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(SampleObj {
        hdr: AttrHeader {
            vtbl: sample_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(8),
        },
        sample: Mutex::new(SampleState::default()),
    });
    std::ptr::write_unaligned(pp_imf_sample, Box::into_raw(obj) as *mut c_void);
    S_OK
}
