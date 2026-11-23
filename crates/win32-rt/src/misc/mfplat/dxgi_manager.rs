//! `IMFDXGIDeviceManager` COM implementation (`MFCreateDXGIDeviceManager`) for `mfplat.dll`.

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::thread::ThreadId;

use crate::com::{
    com_call_add_ref, com_call_qi, com_call_release, read_guid, register_vtable, write_null_out,
    Guid, IID_IUnknown,
};
use crate::types::{BOOL, E_INVALIDARG, E_NOINTERFACE, E_POINTER, HANDLE, HRESULT, S_OK};

use super::{
    E_HANDLE, IID_IMFDXGIDeviceManager, MF_E_DXGI_DEVICE_NOT_INITIALIZED,
    MF_E_DXGI_NEW_VIDEO_DEVICE, MF_E_DXGI_VIDEO_DEVICE_LOCKED,
};

static NEXT_RESET_TOKEN: AtomicU32 = AtomicU32::new(1);
static NEXT_DEV_HANDLE: AtomicUsize = AtomicUsize::new(0x4D46_0001);

struct DxgiDevMgrState {
    device: usize, // AddRef'd IUnknown*
    generation: u64,
    handles: HashMap<usize, u64>, // handle -> generation at open time
    locked_by: Option<(usize, ThreadId, u32)>,
}

impl Drop for DxgiDevMgrState {
    fn drop(&mut self) {
        if self.device != 0 {
            unsafe { com_call_release(self.device as *mut c_void) };
            self.device = 0;
        }
    }
}

#[repr(C)]
struct DxgiDevMgrObj {
    vtbl: *const usize,
    refs: AtomicU32,
    reset_token: u32,
    state: Mutex<DxgiDevMgrState>,
    unlock_cv: Condvar,
}

#[inline]
unsafe fn devmgr_ref<'a>(this: usize) -> Option<&'a DxgiDevMgrObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const DxgiDevMgrObj))
    }
}

unsafe extern "win64" fn devmgr_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IMFDXGIDeviceManager {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn devmgr_add_ref(this: usize) -> u32 {
    let Some(obj) = devmgr_ref(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn devmgr_release(this: usize) -> u32 {
    let Some(obj) = devmgr_ref(this) else {
        return 0;
    };
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut DxgiDevMgrObj));
    }
    next
}

// Slot 3: CloseDeviceHandle
unsafe extern "win64" fn devmgr_close_handle(this: usize, handle: HANDLE) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let mut st = obj.state.lock().unwrap();
    if st.handles.remove(&h).is_none() {
        return E_HANDLE;
    }
    if st.locked_by.is_some_and(|(lh, _, _)| lh == h) {
        st.locked_by = None;
        obj.unlock_cv.notify_all();
    }
    S_OK
}

// Slot 4: GetVideoService
unsafe extern "win64" fn devmgr_get_video_service(
    this: usize,
    handle: HANDLE,
    riid: *const Guid,
    pp_service: *mut *mut c_void,
) -> HRESULT {
    if pp_service.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_service);
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let dev = {
        let st = obj.state.lock().unwrap();
        let Some(&h_gen) = st.handles.get(&(handle as usize)) else {
            return E_HANDLE;
        };
        if st.device == 0 {
            return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
        }
        if h_gen != st.generation {
            return MF_E_DXGI_NEW_VIDEO_DEVICE;
        }
        com_call_add_ref(st.device as *mut c_void);
        st.device as *mut c_void
    };
    let hr = com_call_qi(dev, &iid, pp_service);
    com_call_release(dev);
    hr
}

// Slot 5: LockDevice
unsafe extern "win64" fn devmgr_lock_device(
    this: usize,
    handle: HANDLE,
    riid: *const Guid,
    pp_unk_device: *mut *mut c_void,
    f_block: BOOL,
) -> HRESULT {
    if pp_unk_device.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_unk_device);
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let me = std::thread::current().id();
    let dev = {
        let mut st = obj.state.lock().unwrap();
        loop {
            let Some(&h_gen) = st.handles.get(&h) else {
                return E_HANDLE;
            };
            if st.device == 0 {
                return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
            }
            if h_gen != st.generation {
                return MF_E_DXGI_NEW_VIDEO_DEVICE;
            }
            match st.locked_by {
                None => {
                    st.locked_by = Some((h, me, 1));
                    com_call_add_ref(st.device as *mut c_void);
                    break st.device as *mut c_void;
                }
                Some((lh, ltid, depth)) if lh == h && ltid == me => {
                    st.locked_by = Some((lh, ltid, depth + 1));
                    com_call_add_ref(st.device as *mut c_void);
                    break st.device as *mut c_void;
                }
                Some(_) => {
                    if f_block == 0 {
                        return MF_E_DXGI_VIDEO_DEVICE_LOCKED;
                    }
                    st = obj.unlock_cv.wait(st).unwrap();
                }
            }
        }
    };
    let hr = com_call_qi(dev, &iid, pp_unk_device);
    com_call_release(dev);
    if hr < 0 {
        let mut st = obj.state.lock().unwrap();
        if let Some((lh, ltid, depth)) = st.locked_by {
            if lh == h && ltid == me {
                if depth <= 1 {
                    st.locked_by = None;
                    obj.unlock_cv.notify_all();
                } else {
                    st.locked_by = Some((lh, ltid, depth - 1));
                }
            }
        }
    }
    hr
}

// Slot 6: OpenDeviceHandle
unsafe extern "win64" fn devmgr_open_handle(this: usize, ph_device: *mut HANDLE) -> HRESULT {
    if ph_device.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ph_device, std::ptr::null_mut());
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.device == 0 {
        return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
    }
    let h = NEXT_DEV_HANDLE.fetch_add(1, Ordering::Relaxed);
    let cur_gen = st.generation;
    st.handles.insert(h, cur_gen);
    std::ptr::write_unaligned(ph_device, h as HANDLE);
    S_OK
}

// Slot 7: ResetDevice
unsafe extern "win64" fn devmgr_reset_device(
    this: usize,
    p_unk_device: *mut c_void,
    reset_token: u32,
) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    if p_unk_device.is_null() || reset_token != obj.reset_token {
        return E_INVALIDARG;
    }
    com_call_add_ref(p_unk_device);
    let old_dev = {
        let mut st = obj.state.lock().unwrap();
        let old = st.device;
        st.device = p_unk_device as usize;
        st.generation = st.generation.wrapping_add(1).max(1);
        st.locked_by = None;
        obj.unlock_cv.notify_all();
        old
    };
    if old_dev != 0 {
        com_call_release(old_dev as *mut c_void);
    }
    S_OK
}

// Slot 8: TestDevice
unsafe extern "win64" fn devmgr_test_device(this: usize, handle: HANDLE) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let Some(&h_gen) = st.handles.get(&(handle as usize)) else {
        return E_HANDLE;
    };
    if st.device == 0 {
        return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
    }
    if h_gen != st.generation {
        return MF_E_DXGI_NEW_VIDEO_DEVICE;
    }
    S_OK
}

// Slot 9: UnlockDevice
unsafe extern "win64" fn devmgr_unlock_device(
    this: usize,
    handle: HANDLE,
    _f_save_state: BOOL,
) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let mut st = obj.state.lock().unwrap();
    if !st.handles.contains_key(&h) {
        return E_HANDLE;
    }
    if let Some((lh, ltid, depth)) = st.locked_by {
        if lh == h {
            if depth <= 1 {
                st.locked_by = None;
                obj.unlock_cv.notify_all();
            } else {
                st.locked_by = Some((lh, ltid, depth - 1));
            }
            return S_OK;
        }
    }
    E_INVALIDARG
}

fn devmgr_vtable() -> &'static [usize; 10] {
    static VTBL: OnceLock<[usize; 10]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            devmgr_qi as *const () as usize,
            devmgr_add_ref as *const () as usize,
            devmgr_release as *const () as usize,
            devmgr_close_handle as *const () as usize,
            devmgr_get_video_service as *const () as usize,
            devmgr_lock_device as *const () as usize,
            devmgr_open_handle as *const () as usize,
            devmgr_reset_device as *const () as usize,
            devmgr_test_device as *const () as usize,
            devmgr_unlock_device as *const () as usize,
        ])
    })
}

pub(super) unsafe fn create_dxgi_device_manager(
    reset_token: *mut u32,
    pp_device_manager: *mut *mut c_void,
) -> HRESULT {
    if reset_token.is_null() || pp_device_manager.is_null() {
        write_null_out(pp_device_manager);
        return E_POINTER;
    }
    let token = NEXT_RESET_TOKEN.fetch_add(1, Ordering::Relaxed).max(1);
    let obj = Box::new(DxgiDevMgrObj {
        vtbl: devmgr_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        reset_token: token,
        state: Mutex::new(DxgiDevMgrState {
            device: 0,
            generation: 0,
            handles: HashMap::new(),
            locked_by: None,
        }),
        unlock_cv: Condvar::new(),
    });
    std::ptr::write_unaligned(reset_token, token);
    std::ptr::write_unaligned(pp_device_manager, Box::into_raw(obj) as *mut c_void);
    S_OK
}
