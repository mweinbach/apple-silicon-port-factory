//! `ThreadMgrObj`: `ITfThreadMgr` (14) / `ITfThreadMgrEx` (16) / `ITfThreadMgr2` (17)
//! + `ITfSource` (5) + `ITfSourceSingle` (5) + `ITfUIElementMgr` (8)
//! + `ITfCompartmentMgr` (6) + `ITfKeystrokeMgr` (17).

use super::abi::*;
use super::context::{
    comp_mgr_clear_compartment_impl, comp_mgr_enum_compartments_impl,
    comp_mgr_get_compartment_impl, new_compartment_mgr_obj, new_document_mgr_obj,
    try_add_ref_doc_mgr, CompartmentMap, GLOBAL_COMPARTMENTS,
};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

#[repr(C)]
pub(super) struct ITfThreadMgrExVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) activate: unsafe extern "win64" fn(*mut c_void, *mut TfClientId) -> HRESULT,
    pub(super) deactivate: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    pub(super) create_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) enum_document_mgrs: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_focus: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) set_focus: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    pub(super) associate_focus:
        unsafe extern "win64" fn(*mut c_void, HWND, *mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) is_thread_focus: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT,
    pub(super) get_function_provider:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) enum_function_providers:
        unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_global_compartment:
        unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    // ITfThreadMgrEx (slots 14..=15)
    pub(super) activate_ex: unsafe extern "win64" fn(*mut c_void, *mut TfClientId, u32) -> HRESULT,
    pub(super) get_active_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfThreadMgrExVtbl>() == 16 * std::mem::size_of::<usize>());

#[repr(C)]
pub(super) struct ITfThreadMgr2Vtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) activate: unsafe extern "win64" fn(*mut c_void, *mut TfClientId) -> HRESULT,
    pub(super) deactivate: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    pub(super) create_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) enum_document_mgrs: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_focus: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) set_focus: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    pub(super) is_thread_focus: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT,
    pub(super) get_function_provider:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) enum_function_providers:
        unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_global_compartment:
        unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) activate_ex: unsafe extern "win64" fn(*mut c_void, *mut TfClientId, u32) -> HRESULT,
    pub(super) get_active_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
    pub(super) suspend_keystroke_handling: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    pub(super) resume_keystroke_handling: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfThreadMgr2Vtbl>() == 17 * std::mem::size_of::<usize>());

#[repr(C)]
pub(super) struct ITfUIElementMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) begin_ui_element:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut BOOL, *mut u32) -> HRESULT,
    pub(super) update_ui_element: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    pub(super) end_ui_element: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    pub(super) get_ui_element:
        unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT,
    pub(super) enum_ui_elements: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfUIElementMgrVtbl>() == 8 * std::mem::size_of::<usize>());

#[repr(C)]
pub(super) struct ITfKeystrokeMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) advise_key_event_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *mut c_void, BOOL) -> HRESULT,
    pub(super) unadvise_key_event_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId) -> HRESULT,
    pub(super) get_foreground: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
    pub(super) test_key_down:
        unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    pub(super) test_key_up:
        unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    pub(super) key_down: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    pub(super) key_up: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    pub(super) get_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *const u8, *mut u8) -> HRESULT,
    pub(super) is_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u8, *mut BOOL) -> HRESULT,
    pub(super) preserve_key: unsafe extern "win64" fn(
        *mut c_void,
        TfClientId,
        *const u8,
        *const u8,
        *const u16,
        u32,
    ) -> HRESULT,
    pub(super) unpreserve_key:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u8) -> HRESULT,
    pub(super) set_preserved_key_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u16, u32) -> HRESULT,
    pub(super) get_preserved_key_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut u16) -> HRESULT,
    pub(super) simulate_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *const u8, *mut BOOL) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfKeystrokeMgrVtbl>() == 17 * std::mem::size_of::<usize>());

struct PreservedKeyEntry {
    #[allow(dead_code)]
    tid: TfClientId,
    prekey: TF_PRESERVEDKEY,
    desc: String,
}

pub(super) struct ThreadMgrState {
    activation_count: u32,
    active_flags: u32,
    client_id: TfClientId,
    keystroke_suspend_count: u32,
    focus_doc_mgr: usize,
    hwnd_focus: HashMap<usize, usize>,
    /// Non-owning pointers to live `DocumentMgrObj`s created on this thread.
    pub(super) doc_mgrs: Vec<usize>,
    /// Registered `ITfUIElement*` objects keyed by element ID.
    next_ui_elem_id: u32,
    ui_elements: HashMap<u32, usize>,
    /// `ITfSourceSingle` sinks.
    single_sinks: HashMap<(TfClientId, Guid), usize>,
    /// `ITfKeystrokeMgr` state.
    key_event_sinks: HashMap<TfClientId, usize>,
    foreground_key_tid: Option<TfClientId>,
    preserved_keys: HashMap<Guid, PreservedKeyEntry>,
}

impl Default for ThreadMgrState {
    fn default() -> Self {
        Self {
            activation_count: 0,
            active_flags: 0,
            client_id: 0,
            keystroke_suspend_count: 0,
            focus_doc_mgr: 0,
            hwnd_focus: HashMap::new(),
            doc_mgrs: Vec::new(),
            next_ui_elem_id: 1,
            ui_elements: HashMap::new(),
            single_sinks: HashMap::new(),
            key_event_sinks: HashMap::new(),
            foreground_key_tid: None,
            preserved_keys: HashMap::new(),
        }
    }
}

#[repr(C)]
pub(super) struct ThreadMgrObj {
    vtbl_tm_ex: *const ITfThreadMgrExVtbl,
    vtbl_tm2: *const ITfThreadMgr2Vtbl,
    vtbl_source: *const ITfSourceVtbl,
    vtbl_source_single: *const ITfSourceSingleVtbl,
    vtbl_ui_elem: *const ITfUIElementMgrVtbl,
    vtbl_comp_mgr: *const ITfCompartmentMgrVtbl,
    vtbl_keystroke: *const ITfKeystrokeMgrVtbl,
    refs: AtomicU32,
    tid: u32,
    state: Arc<Mutex<ThreadMgrState>>,
    sinks: Arc<Mutex<SinkTable>>,
    compartments: Mutex<CompartmentMap>,
}

static NEXT_CLIENT_ID: AtomicU32 = AtomicU32::new(1);
static THREAD_MGR_REGISTRY: LazyLock<Mutex<HashMap<u32, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn alloc_client_id() -> TfClientId {
    loop {
        let id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        if id != 0 {
            return id;
        }
    }
}

pub(super) fn current_tid() -> u32 {
    unsafe { crate::kernel32::threads::GetCurrentThreadId() }
}

pub(super) fn tm_supports_iid(iid: &Guid) -> bool {
    *iid == IID_IUNKNOWN
        || *iid == IID_ITFTHREADMGR
        || *iid == IID_ITFTHREADMGREX
        || *iid == IID_ITFTHREADMGR2
        || *iid == IID_ITFSOURCE
        || *iid == IID_ITFSOURCESINGLE
        || *iid == IID_ITFUIELEMENTMGR
        || *iid == IID_ITFCOMPARTMENTMGR
        || *iid == IID_ITFKEYSTROKEMGR
}

pub(super) unsafe fn tm_qi_common(
    base: *mut ThreadMgrObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let b = base as *mut u8;
    qi_multi_iface(
        &(*base).refs,
        riid,
        ppv,
        base as *mut c_void,
        &[
            (IID_ITFTHREADMGR, base as *mut c_void),
            (IID_ITFTHREADMGREX, base as *mut c_void),
            (IID_ITFTHREADMGR2, b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_tm2)) as *mut c_void),
            (IID_ITFSOURCE, b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_source)) as *mut c_void),
            (
                IID_ITFSOURCESINGLE,
                b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_source_single)) as *mut c_void,
            ),
            (
                IID_ITFUIELEMENTMGR,
                b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_ui_elem)) as *mut c_void,
            ),
            (
                IID_ITFCOMPARTMENTMGR,
                b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_comp_mgr)) as *mut c_void,
            ),
            (
                IID_ITFKEYSTROKEMGR,
                b.add(std::mem::offset_of!(ThreadMgrObj, vtbl_keystroke)) as *mut c_void,
            ),
        ],
    )
}

unsafe fn tm_add_ref_common(base: *mut ThreadMgrObj) -> u32 {
    (*base).refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe fn tm_release_common(base: *mut ThreadMgrObj) -> u32 {
    let obj = &*base;
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    if prev > 1 {
        return prev - 1;
    }
    // Refcount hit 0; coordinate with THREAD_MGR_REGISTRY so concurrent
    // TF_GetThreadMgr / TF_CreateThreadMgr cannot race with destruction.
    {
        let mut reg = THREAD_MGR_REGISTRY.lock().unwrap();
        let cur = obj.refs.load(Ordering::Acquire);
        if cur != 0 {
            return cur;
        }
        if reg.get(&obj.tid).copied() == Some(base as usize) {
            reg.remove(&obj.tid);
        }
    }
    let mut to_release: Vec<*mut c_void> = Vec::new();
    {
        let mut st = obj.state.lock().unwrap();
        if st.focus_doc_mgr != 0 {
            to_release.push(st.focus_doc_mgr as *mut c_void);
            st.focus_doc_mgr = 0;
        }
        for (_, dm) in st.hwnd_focus.drain() {
            if dm != 0 {
                to_release.push(dm as *mut c_void);
            }
        }
        for (_, elem) in st.ui_elements.drain() {
            if elem != 0 {
                to_release.push(elem as *mut c_void);
            }
        }
        for (_, s) in st.single_sinks.drain() {
            if s != 0 {
                to_release.push(s as *mut c_void);
            }
        }
        for (_, s) in st.key_event_sinks.drain() {
            if s != 0 {
                to_release.push(s as *mut c_void);
            }
        }
    }
    to_release.extend(obj.sinks.lock().unwrap().drain_all());
    drop(Box::from_raw(base));
    release_all_ptrs(to_release);
    0
}

impl_primary_iunknown!(
    tm_qi,
    tm_add_ref,
    tm_release,
    ThreadMgrObj,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_tm2,
    tm2_qi,
    tm2_add_ref,
    tm2_release,
    ThreadMgrObj,
    vtbl_tm2,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_source,
    tm_src_qi,
    tm_src_add_ref,
    tm_src_release,
    ThreadMgrObj,
    vtbl_source,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_source_single,
    tm_ss_qi,
    tm_ss_add_ref,
    tm_ss_release,
    ThreadMgrObj,
    vtbl_source_single,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_ui_elem,
    tm_ui_qi,
    tm_ui_add_ref,
    tm_ui_release,
    ThreadMgrObj,
    vtbl_ui_elem,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_comp_mgr,
    tm_cm_qi,
    tm_cm_add_ref,
    tm_cm_release,
    ThreadMgrObj,
    vtbl_comp_mgr,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);
impl_sub_iunknown!(
    tm_from_keystroke,
    tm_ks_qi,
    tm_ks_add_ref,
    tm_ks_release,
    ThreadMgrObj,
    vtbl_keystroke,
    tm_qi_common,
    tm_add_ref_common,
    tm_release_common
);

// Core ThreadMgr method implementations shared by ITfThreadMgr / Ex / 2
unsafe fn tm_activate_impl(base: *mut ThreadMgrObj, ptid: *mut TfClientId, flags: u32) -> HRESULT {
    if ptid.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*base;
    let mut st = obj.state.lock().unwrap();
    if st.activation_count == 0 {
        st.active_flags = flags | TF_TMF_ACTIVATED;
        if st.client_id == 0 {
            st.client_id = alloc_client_id();
        }
    } else {
        st.active_flags |= flags | TF_TMF_ACTIVATED;
    }
    st.activation_count = st.activation_count.saturating_add(1);
    let cid = st.client_id;
    drop(st);
    std::ptr::write_unaligned(ptid, cid);
    S_OK
}

unsafe fn tm_deactivate_impl(base: *mut ThreadMgrObj) -> HRESULT {
    let obj = &*base;
    let old_focus = {
        let mut st = obj.state.lock().unwrap();
        if st.activation_count == 0 {
            return E_UNEXPECTED;
        }
        st.activation_count -= 1;
        if st.activation_count == 0 {
            st.active_flags = 0;
            std::mem::replace(&mut st.focus_doc_mgr, 0) as *mut c_void
        } else {
            std::ptr::null_mut()
        }
    };
    if !old_focus.is_null() {
        let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFTHREADMGREVENTSINK);
        for sink in sinks {
            let _ = com_call_slot(sink, 5, 0, old_focus as u64, 0); // OnSetFocus(NULL, prev)
            com_call_release(sink);
        }
        com_call_release(old_focus);
    }
    S_OK
}

unsafe fn tm_create_document_mgr_impl(base: *mut ThreadMgrObj, ppdim: *mut *mut c_void) -> HRESULT {
    if ppdim.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(ppdim);
    let obj = &*base;
    let dm = new_document_mgr_obj(Arc::clone(&obj.state), Arc::clone(&obj.sinks));
    obj.state.lock().unwrap().doc_mgrs.push(dm as usize);
    let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFTHREADMGREVENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 3, dm as u64, 0, 0); // OnInitDocumentMgr
        com_call_release(sink);
    }
    std::ptr::write_unaligned(ppdim, dm as *mut c_void);
    S_OK
}

unsafe fn tm_enum_document_mgrs_impl(base: *mut ThreadMgrObj, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let obj = &*base;
    let live: Vec<usize> = {
        let st = obj.state.lock().unwrap();
        st.doc_mgrs
            .iter()
            .filter_map(|&raw| try_add_ref_doc_mgr(raw).map(|p| p as usize))
            .collect()
    };
    let en = new_enum_tf_com(IID_IENUMTFDOCUMENTMGRS, live, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe fn tm_get_focus_impl(base: *mut ThreadMgrObj, ppdim_focus: *mut *mut c_void) -> HRESULT {
    if ppdim_focus.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(ppdim_focus);
    let obj = &*base;
    let focused = obj.state.lock().unwrap().focus_doc_mgr as *mut c_void;
    if !focused.is_null() {
        com_call_add_ref(focused);
        std::ptr::write_unaligned(ppdim_focus, focused);
    }
    S_OK
}

unsafe fn tm_set_focus_impl(base: *mut ThreadMgrObj, pdim_focus: *mut c_void) -> HRESULT {
    let obj = &*base;
    if !pdim_focus.is_null() {
        com_call_add_ref(pdim_focus);
    }
    let (changed, prev) = {
        let mut st = obj.state.lock().unwrap();
        if st.focus_doc_mgr == pdim_focus as usize {
            (false, std::ptr::null_mut())
        } else {
            let old = std::mem::replace(&mut st.focus_doc_mgr, pdim_focus as usize) as *mut c_void;
            (true, old)
        }
    };
    if !changed {
        if !pdim_focus.is_null() {
            com_call_release(pdim_focus);
        }
        return S_OK;
    }
    let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFTHREADMGREVENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 5, pdim_focus as u64, prev as u64, 0); // OnSetFocus
        com_call_release(sink);
    }
    if !prev.is_null() {
        com_call_release(prev);
    }
    S_OK
}

unsafe fn tm_associate_focus_impl(
    base: *mut ThreadMgrObj,
    hwnd: HWND,
    pdim_new: *mut c_void,
    ppdim_prev: *mut *mut c_void,
) -> HRESULT {
    write_null_out(ppdim_prev);
    if hwnd.is_null() || ppdim_prev.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*base;
    if !pdim_new.is_null() {
        com_call_add_ref(pdim_new);
    }
    let prev = {
        let mut st = obj.state.lock().unwrap();
        if pdim_new.is_null() {
            st.hwnd_focus.remove(&(hwnd as usize)).unwrap_or(0) as *mut c_void
        } else {
            st.hwnd_focus
                .insert(hwnd as usize, pdim_new as usize)
                .unwrap_or(0) as *mut c_void
        }
    };
    // Ownership of `prev`'s reference transfers directly to `*ppdim_prev`.
    std::ptr::write_unaligned(ppdim_prev, prev);
    // Keep thread focus synchronized if this HWND currently holds Win32 keyboard focus.
    if crate::user32::input::GetFocus() == hwnd {
        let _ = tm_set_focus_impl(base, pdim_new);
    }
    S_OK
}

unsafe fn tm_is_thread_focus_impl(base: *mut ThreadMgrObj, pf_thread_focus: *mut BOOL) -> HRESULT {
    if pf_thread_focus.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*base;
    let st = obj.state.lock().unwrap();
    let focused = st.activation_count > 0 || st.focus_doc_mgr != 0;
    std::ptr::write_unaligned(pf_thread_focus, if focused { TRUE } else { FALSE });
    S_OK
}

unsafe fn tm_get_function_provider_impl(
    _base: *mut ThreadMgrObj,
    clsid: *const u8,
    pp_func_prov: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_func_prov);
    if clsid.is_null() || pp_func_prov.is_null() {
        return E_INVALIDARG;
    }
    TF_E_NOPROVIDER
}

unsafe fn tm_enum_function_providers_impl(
    _base: *mut ThreadMgrObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let en = new_enum_tf_com(IID_IENUMTFFUNCTIONPROVIDERS, Vec::new(), 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe fn tm_get_global_compartment_impl(
    _base: *mut ThreadMgrObj,
    pp_comp_mgr: *mut *mut c_void,
) -> HRESULT {
    if pp_comp_mgr.is_null() {
        return E_INVALIDARG;
    }
    let mgr = new_compartment_mgr_obj(Arc::clone(&*GLOBAL_COMPARTMENTS));
    std::ptr::write_unaligned(pp_comp_mgr, mgr as *mut c_void);
    S_OK
}

unsafe fn tm_get_active_flags_impl(base: *mut ThreadMgrObj, lpdw_flags: *mut u32) -> HRESULT {
    if lpdw_flags.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*base;
    let flags = obj.state.lock().unwrap().active_flags;
    std::ptr::write_unaligned(lpdw_flags, flags);
    S_OK
}

// ITfThreadMgr / ITfThreadMgrEx thunks (offset 0)
unsafe extern "win64" fn tm_activate(this: *mut c_void, ptid: *mut TfClientId) -> HRESULT {
    tm_activate_impl(this as *mut ThreadMgrObj, ptid, 0)
}
unsafe extern "win64" fn tm_deactivate(this: *mut c_void) -> HRESULT {
    tm_deactivate_impl(this as *mut ThreadMgrObj)
}
unsafe extern "win64" fn tm_create_doc_mgr(this: *mut c_void, ppdim: *mut *mut c_void) -> HRESULT {
    tm_create_document_mgr_impl(this as *mut ThreadMgrObj, ppdim)
}
unsafe extern "win64" fn tm_enum_doc_mgrs(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    tm_enum_document_mgrs_impl(this as *mut ThreadMgrObj, pp_enum)
}
unsafe extern "win64" fn tm_get_focus(this: *mut c_void, ppdim: *mut *mut c_void) -> HRESULT {
    tm_get_focus_impl(this as *mut ThreadMgrObj, ppdim)
}
unsafe extern "win64" fn tm_set_focus(this: *mut c_void, pdim: *mut c_void) -> HRESULT {
    tm_set_focus_impl(this as *mut ThreadMgrObj, pdim)
}
unsafe extern "win64" fn tm_associate_focus(
    this: *mut c_void,
    hwnd: HWND,
    pdim_new: *mut c_void,
    ppdim_prev: *mut *mut c_void,
) -> HRESULT {
    tm_associate_focus_impl(this as *mut ThreadMgrObj, hwnd, pdim_new, ppdim_prev)
}
unsafe extern "win64" fn tm_is_thread_focus(this: *mut c_void, pf: *mut BOOL) -> HRESULT {
    tm_is_thread_focus_impl(this as *mut ThreadMgrObj, pf)
}
unsafe extern "win64" fn tm_get_func_provider(
    this: *mut c_void,
    clsid: *const u8,
    pp: *mut *mut c_void,
) -> HRESULT {
    tm_get_function_provider_impl(this as *mut ThreadMgrObj, clsid, pp)
}
unsafe extern "win64" fn tm_enum_func_providers(this: *mut c_void, pp: *mut *mut c_void) -> HRESULT {
    tm_enum_function_providers_impl(this as *mut ThreadMgrObj, pp)
}
unsafe extern "win64" fn tm_get_global_comp(this: *mut c_void, pp: *mut *mut c_void) -> HRESULT {
    tm_get_global_compartment_impl(this as *mut ThreadMgrObj, pp)
}
unsafe extern "win64" fn tm_activate_ex(
    this: *mut c_void,
    ptid: *mut TfClientId,
    flags: u32,
) -> HRESULT {
    tm_activate_impl(this as *mut ThreadMgrObj, ptid, flags)
}
unsafe extern "win64" fn tm_get_active_flags(this: *mut c_void, flags: *mut u32) -> HRESULT {
    tm_get_active_flags_impl(this as *mut ThreadMgrObj, flags)
}

// ITfThreadMgr2 thunks (offset_of!(ThreadMgrObj, vtbl_tm2))
unsafe extern "win64" fn tm2_activate(this: *mut c_void, ptid: *mut TfClientId) -> HRESULT {
    tm_activate_impl(tm_from_tm2(this), ptid, 0)
}
unsafe extern "win64" fn tm2_deactivate(this: *mut c_void) -> HRESULT {
    tm_deactivate_impl(tm_from_tm2(this))
}
unsafe extern "win64" fn tm2_create_doc_mgr(this: *mut c_void, ppdim: *mut *mut c_void) -> HRESULT {
    tm_create_document_mgr_impl(tm_from_tm2(this), ppdim)
}
unsafe extern "win64" fn tm2_enum_doc_mgrs(
    this: *mut c_void,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    tm_enum_document_mgrs_impl(tm_from_tm2(this), pp_enum)
}
unsafe extern "win64" fn tm2_get_focus(this: *mut c_void, ppdim: *mut *mut c_void) -> HRESULT {
    tm_get_focus_impl(tm_from_tm2(this), ppdim)
}
unsafe extern "win64" fn tm2_set_focus(this: *mut c_void, pdim: *mut c_void) -> HRESULT {
    tm_set_focus_impl(tm_from_tm2(this), pdim)
}
unsafe extern "win64" fn tm2_is_thread_focus(this: *mut c_void, pf: *mut BOOL) -> HRESULT {
    tm_is_thread_focus_impl(tm_from_tm2(this), pf)
}
unsafe extern "win64" fn tm2_get_func_provider(
    this: *mut c_void,
    clsid: *const u8,
    pp: *mut *mut c_void,
) -> HRESULT {
    tm_get_function_provider_impl(tm_from_tm2(this), clsid, pp)
}
unsafe extern "win64" fn tm2_enum_func_providers(
    this: *mut c_void,
    pp: *mut *mut c_void,
) -> HRESULT {
    tm_enum_function_providers_impl(tm_from_tm2(this), pp)
}
unsafe extern "win64" fn tm2_get_global_comp(this: *mut c_void, pp: *mut *mut c_void) -> HRESULT {
    tm_get_global_compartment_impl(tm_from_tm2(this), pp)
}
unsafe extern "win64" fn tm2_activate_ex(
    this: *mut c_void,
    ptid: *mut TfClientId,
    flags: u32,
) -> HRESULT {
    tm_activate_impl(tm_from_tm2(this), ptid, flags)
}
unsafe extern "win64" fn tm2_get_active_flags(this: *mut c_void, flags: *mut u32) -> HRESULT {
    tm_get_active_flags_impl(tm_from_tm2(this), flags)
}
unsafe extern "win64" fn tm2_suspend_keystrokes(this: *mut c_void) -> HRESULT {
    let obj = &*tm_from_tm2(this);
    let mut st = obj.state.lock().unwrap();
    st.keystroke_suspend_count = st.keystroke_suspend_count.saturating_add(1);
    S_OK
}
unsafe extern "win64" fn tm2_resume_keystrokes(this: *mut c_void) -> HRESULT {
    let obj = &*tm_from_tm2(this);
    let mut st = obj.state.lock().unwrap();
    if st.keystroke_suspend_count == 0 {
        return E_UNEXPECTED;
    }
    st.keystroke_suspend_count -= 1;
    S_OK
}

// ITfSource thunks on ThreadMgrObj
unsafe extern "win64" fn tm_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*tm_from_source(this);
    sink_advise(&obj.sinks, riid, punk, pdw_cookie)
}
unsafe extern "win64" fn tm_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*tm_from_source(this);
    sink_unadvise(&obj.sinks, cookie)
}

// ITfSourceSingle thunks on ThreadMgrObj
unsafe extern "win64" fn tm_ss_advise_single(
    this: *mut c_void,
    tid: TfClientId,
    riid: *const u8,
    punk: *mut c_void,
) -> HRESULT {
    let Some(iid) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    if punk.is_null() {
        return E_INVALIDARG;
    }
    let mut iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi_bytes(punk, riid, &mut iface);
    if hr < 0 || iface.is_null() {
        return CONNECT_E_CANNOTCONNECT;
    }
    let obj = &*tm_from_source_single(this);
    let mut st = obj.state.lock().unwrap();
    if st.single_sinks.contains_key(&(tid, iid)) {
        drop(st);
        com_call_release(iface);
        return CONNECT_E_ADVISELIMIT;
    }
    st.single_sinks.insert((tid, iid), iface as usize);
    S_OK
}
unsafe extern "win64" fn tm_ss_unadvise_single(
    this: *mut c_void,
    tid: TfClientId,
    riid: *const u8,
) -> HRESULT {
    let Some(iid) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*tm_from_source_single(this);
    let removed = obj.state.lock().unwrap().single_sinks.remove(&(tid, iid));
    match removed {
        Some(p) => {
            com_call_release(p as *mut c_void);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

// ITfUIElementMgr thunks on ThreadMgrObj
unsafe extern "win64" fn tm_ui_begin(
    this: *mut c_void,
    element: *mut c_void,
    pb_show: *mut BOOL,
    pdw_id: *mut u32,
) -> HRESULT {
    if element.is_null() || pb_show.is_null() || pdw_id.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*tm_from_ui_elem(this);
    com_call_add_ref(element);
    let id = {
        let mut st = obj.state.lock().unwrap();
        let mut id = st.next_ui_elem_id;
        while id == 0 || id == TF_INVALID_COOKIE || st.ui_elements.contains_key(&id) {
            id = id.wrapping_add(1);
        }
        st.next_ui_elem_id = id.wrapping_add(1);
        st.ui_elements.insert(id, element as usize);
        id
    };
    let mut show: BOOL = TRUE;
    let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFUIELEMENTSINK);
    for sink in sinks {
        let mut sink_show: BOOL = TRUE;
        let _ = com_call_slot(
            sink,
            3,
            id as u64,
            (&mut sink_show as *mut BOOL) as u64,
            0,
        );
        if sink_show == FALSE {
            show = FALSE;
        }
        com_call_release(sink);
    }
    std::ptr::write_unaligned(pb_show, show);
    std::ptr::write_unaligned(pdw_id, id);
    S_OK
}
unsafe extern "win64" fn tm_ui_update(this: *mut c_void, id: u32) -> HRESULT {
    let obj = &*tm_from_ui_elem(this);
    if !obj.state.lock().unwrap().ui_elements.contains_key(&id) {
        return E_INVALIDARG;
    }
    let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFUIELEMENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 4, id as u64, 0, 0);
        com_call_release(sink);
    }
    S_OK
}
unsafe extern "win64" fn tm_ui_end(this: *mut c_void, id: u32) -> HRESULT {
    let obj = &*tm_from_ui_elem(this);
    let removed = obj.state.lock().unwrap().ui_elements.remove(&id);
    let Some(elem) = removed else {
        return E_INVALIDARG;
    };
    let sinks = snapshot_sinks_addref(&obj.sinks, &IID_ITFUIELEMENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 5, id as u64, 0, 0);
        com_call_release(sink);
    }
    com_call_release(elem as *mut c_void);
    S_OK
}
unsafe extern "win64" fn tm_ui_get(
    this: *mut c_void,
    id: u32,
    pp_element: *mut *mut c_void,
) -> HRESULT {
    if pp_element.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_element);
    let obj = &*tm_from_ui_elem(this);
    let found = obj.state.lock().unwrap().ui_elements.get(&id).copied();
    match found {
        Some(elem) => {
            let ptr = elem as *mut c_void;
            com_call_add_ref(ptr);
            std::ptr::write_unaligned(pp_element, ptr);
            S_OK
        }
        None => E_INVALIDARG,
    }
}
unsafe extern "win64" fn tm_ui_enum(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let obj = &*tm_from_ui_elem(this);
    let items: Vec<usize> = obj.state.lock().unwrap().ui_elements.values().copied().collect();
    for &p in &items {
        com_call_add_ref(p as *mut c_void);
    }
    let en = new_enum_tf_com(IID_IENUMTFUIELEMENTS, items, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

// ITfCompartmentMgr thunks on ThreadMgrObj
unsafe extern "win64" fn tm_cm_get(
    this: *mut c_void,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    let obj = &*tm_from_comp_mgr(this);
    comp_mgr_get_compartment_impl(&obj.compartments, rguid, ppcomp)
}
unsafe extern "win64" fn tm_cm_clear(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let obj = &*tm_from_comp_mgr(this);
    comp_mgr_clear_compartment_impl(&obj.compartments, tid, rguid)
}
unsafe extern "win64" fn tm_cm_enum(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    let obj = &*tm_from_comp_mgr(this);
    comp_mgr_enum_compartments_impl(&obj.compartments, pp_enum)
}

// ITfKeystrokeMgr thunks on ThreadMgrObj
unsafe extern "win64" fn tm_ks_advise_key_event_sink(
    this: *mut c_void,
    tid: TfClientId,
    p_sink: *mut c_void,
    f_foreground: BOOL,
) -> HRESULT {
    if p_sink.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*tm_from_keystroke(this);
    {
        let mut st = obj.state.lock().unwrap();
        if st.key_event_sinks.contains_key(&tid) {
            return CONNECT_E_ADVISELIMIT;
        }
        st.key_event_sinks.insert(tid, p_sink as usize);
        if f_foreground != 0 {
            st.foreground_key_tid = Some(tid);
        }
    }
    com_call_add_ref(p_sink);
    S_OK
}
unsafe extern "win64" fn tm_ks_unadvise_key_event_sink(
    this: *mut c_void,
    tid: TfClientId,
) -> HRESULT {
    let obj = &*tm_from_keystroke(this);
    let removed = {
        let mut st = obj.state.lock().unwrap();
        if st.foreground_key_tid == Some(tid) {
            st.foreground_key_tid = None;
        }
        st.key_event_sinks.remove(&tid)
    };
    match removed {
        Some(p) => {
            com_call_release(p as *mut c_void);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}
unsafe extern "win64" fn tm_ks_get_foreground(this: *mut c_void, pclsid: *mut u8) -> HRESULT {
    if pclsid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pclsid as *mut Guid, GUID_NULL);
    let obj = &*tm_from_keystroke(this);
    if obj.state.lock().unwrap().foreground_key_tid.is_some() {
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn tm_ks_test_or_eat(
    _this: *mut c_void,
    _wparam: usize,
    _lparam: isize,
    pf_eaten: *mut BOOL,
) -> HRESULT {
    if pf_eaten.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_eaten, FALSE);
    S_OK
}
unsafe extern "win64" fn tm_ks_get_preserved_key(
    this: *mut c_void,
    _pic: *mut c_void,
    pprekey: *const u8,
    pguid: *mut u8,
) -> HRESULT {
    if pprekey.is_null() || pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid as *mut Guid, GUID_NULL);
    let prekey = std::ptr::read_unaligned(pprekey as *const TF_PRESERVEDKEY);
    let obj = &*tm_from_keystroke(this);
    let st = obj.state.lock().unwrap();
    if let Some((&g, _)) = st.preserved_keys.iter().find(|(_, e)| e.prekey == prekey) {
        write_guid(pguid as *mut Guid, g);
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn tm_ks_is_preserved_key(
    this: *mut c_void,
    rguid: *const u8,
    pprekey: *const u8,
    pf_registered: *mut BOOL,
) -> HRESULT {
    if pf_registered.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_registered, FALSE);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pprekey.is_null() {
        return E_INVALIDARG;
    }
    let prekey = std::ptr::read_unaligned(pprekey as *const TF_PRESERVEDKEY);
    let obj = &*tm_from_keystroke(this);
    let found = obj
        .state
        .lock()
        .unwrap()
        .preserved_keys
        .get(&g)
        .is_some_and(|e| e.prekey == prekey);
    std::ptr::write_unaligned(pf_registered, if found { TRUE } else { FALSE });
    if found {
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn tm_ks_preserve_key(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
    prekey_ptr: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if prekey_ptr.is_null() {
        return E_INVALIDARG;
    }
    let prekey = std::ptr::read_unaligned(prekey_ptr as *const TF_PRESERVEDKEY);
    let desc = read_wide_len(pch_desc, cch_desc as usize);
    let obj = &*tm_from_keystroke(this);
    let mut st = obj.state.lock().unwrap();
    if st.preserved_keys.contains_key(&g) {
        return TF_E_ALREADY_EXISTS;
    }
    st.preserved_keys.insert(g, PreservedKeyEntry { tid, prekey, desc });
    S_OK
}
unsafe extern "win64" fn tm_ks_unpreserve_key(
    this: *mut c_void,
    rguid: *const u8,
    _pprekey: *const u8,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    let obj = &*tm_from_keystroke(this);
    if obj.state.lock().unwrap().preserved_keys.remove(&g).is_some() {
        S_OK
    } else {
        CONNECT_E_NOCONNECTION
    }
}
unsafe extern "win64" fn tm_ks_set_preserved_key_desc(
    this: *mut c_void,
    rguid: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc as usize);
    let obj = &*tm_from_keystroke(this);
    let mut st = obj.state.lock().unwrap();
    match st.preserved_keys.get_mut(&g) {
        Some(e) => {
            e.desc = desc;
            S_OK
        }
        None => E_FAIL,
    }
}
unsafe extern "win64" fn tm_ks_get_preserved_key_desc(
    this: *mut c_void,
    rguid: *const u8,
    pbstr_desc: *mut *mut u16,
) -> HRESULT {
    write_null_out(pbstr_desc);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if pbstr_desc.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*tm_from_keystroke(this);
    let desc = obj.state.lock().unwrap().preserved_keys.get(&g).map(|e| e.desc.clone());
    match desc {
        Some(s) => {
            std::ptr::write_unaligned(pbstr_desc, alloc_bstr_str(&s));
            S_OK
        }
        None => E_FAIL,
    }
}
unsafe extern "win64" fn tm_ks_simulate_preserved_key(
    _this: *mut c_void,
    _pic: *mut c_void,
    _rguid: *const u8,
    pf_eaten: *mut BOOL,
) -> HRESULT {
    if pf_eaten.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_eaten, FALSE);
    S_OK
}

static ITF_THREAD_MGR_EX_VTBL: ITfThreadMgrExVtbl = ITfThreadMgrExVtbl {
    query_interface: tm_qi,
    add_ref: tm_add_ref,
    release: tm_release,
    activate: tm_activate,
    deactivate: tm_deactivate,
    create_document_mgr: tm_create_doc_mgr,
    enum_document_mgrs: tm_enum_doc_mgrs,
    get_focus: tm_get_focus,
    set_focus: tm_set_focus,
    associate_focus: tm_associate_focus,
    is_thread_focus: tm_is_thread_focus,
    get_function_provider: tm_get_func_provider,
    enum_function_providers: tm_enum_func_providers,
    get_global_compartment: tm_get_global_comp,
    activate_ex: tm_activate_ex,
    get_active_flags: tm_get_active_flags,
};

static ITF_THREAD_MGR2_VTBL: ITfThreadMgr2Vtbl = ITfThreadMgr2Vtbl {
    query_interface: tm2_qi,
    add_ref: tm2_add_ref,
    release: tm2_release,
    activate: tm2_activate,
    deactivate: tm2_deactivate,
    create_document_mgr: tm2_create_doc_mgr,
    enum_document_mgrs: tm2_enum_doc_mgrs,
    get_focus: tm2_get_focus,
    set_focus: tm2_set_focus,
    is_thread_focus: tm2_is_thread_focus,
    get_function_provider: tm2_get_func_provider,
    enum_function_providers: tm2_enum_func_providers,
    get_global_compartment: tm2_get_global_comp,
    activate_ex: tm2_activate_ex,
    get_active_flags: tm2_get_active_flags,
    suspend_keystroke_handling: tm2_suspend_keystrokes,
    resume_keystroke_handling: tm2_resume_keystrokes,
};

static ITF_THREAD_MGR_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: tm_src_qi,
    add_ref: tm_src_add_ref,
    release: tm_src_release,
    advise_sink: tm_src_advise,
    unadvise_sink: tm_src_unadvise,
};

static ITF_THREAD_MGR_SS_VTBL: ITfSourceSingleVtbl = ITfSourceSingleVtbl {
    query_interface: tm_ss_qi,
    add_ref: tm_ss_add_ref,
    release: tm_ss_release,
    advise_single_sink: tm_ss_advise_single,
    unadvise_single_sink: tm_ss_unadvise_single,
};

static ITF_UI_ELEMENT_MGR_VTBL: ITfUIElementMgrVtbl = ITfUIElementMgrVtbl {
    query_interface: tm_ui_qi,
    add_ref: tm_ui_add_ref,
    release: tm_ui_release,
    begin_ui_element: tm_ui_begin,
    update_ui_element: tm_ui_update,
    end_ui_element: tm_ui_end,
    get_ui_element: tm_ui_get,
    enum_ui_elements: tm_ui_enum,
};

static ITF_THREAD_MGR_CM_VTBL: ITfCompartmentMgrVtbl = ITfCompartmentMgrVtbl {
    query_interface: tm_cm_qi,
    add_ref: tm_cm_add_ref,
    release: tm_cm_release,
    get_compartment: tm_cm_get,
    clear_compartment: tm_cm_clear,
    enum_compartments: tm_cm_enum,
};

static ITF_KEYSTROKE_MGR_VTBL: ITfKeystrokeMgrVtbl = ITfKeystrokeMgrVtbl {
    query_interface: tm_ks_qi,
    add_ref: tm_ks_add_ref,
    release: tm_ks_release,
    advise_key_event_sink: tm_ks_advise_key_event_sink,
    unadvise_key_event_sink: tm_ks_unadvise_key_event_sink,
    get_foreground: tm_ks_get_foreground,
    test_key_down: tm_ks_test_or_eat,
    test_key_up: tm_ks_test_or_eat,
    key_down: tm_ks_test_or_eat,
    key_up: tm_ks_test_or_eat,
    get_preserved_key: tm_ks_get_preserved_key,
    is_preserved_key: tm_ks_is_preserved_key,
    preserve_key: tm_ks_preserve_key,
    unpreserve_key: tm_ks_unpreserve_key,
    set_preserved_key_description: tm_ks_set_preserved_key_desc,
    get_preserved_key_description: tm_ks_get_preserved_key_desc,
    simulate_preserved_key: tm_ks_simulate_preserved_key,
};

fn register_thread_mgr_vtables() {
    register_vtable_ptrs(&[
        tm_qi as *const () as usize,
        tm_add_ref as *const () as usize,
        tm_release as *const () as usize,
        tm_activate as *const () as usize,
        tm_deactivate as *const () as usize,
        tm_create_doc_mgr as *const () as usize,
        tm_enum_doc_mgrs as *const () as usize,
        tm_get_focus as *const () as usize,
        tm_set_focus as *const () as usize,
        tm_associate_focus as *const () as usize,
        tm_is_thread_focus as *const () as usize,
        tm_get_func_provider as *const () as usize,
        tm_enum_func_providers as *const () as usize,
        tm_get_global_comp as *const () as usize,
        tm_activate_ex as *const () as usize,
        tm_get_active_flags as *const () as usize,
        tm2_qi as *const () as usize,
        tm2_add_ref as *const () as usize,
        tm2_release as *const () as usize,
        tm2_activate as *const () as usize,
        tm2_deactivate as *const () as usize,
        tm2_create_doc_mgr as *const () as usize,
        tm2_enum_doc_mgrs as *const () as usize,
        tm2_get_focus as *const () as usize,
        tm2_set_focus as *const () as usize,
        tm2_is_thread_focus as *const () as usize,
        tm2_get_func_provider as *const () as usize,
        tm2_enum_func_providers as *const () as usize,
        tm2_get_global_comp as *const () as usize,
        tm2_activate_ex as *const () as usize,
        tm2_get_active_flags as *const () as usize,
        tm2_suspend_keystrokes as *const () as usize,
        tm2_resume_keystrokes as *const () as usize,
        tm_src_qi as *const () as usize,
        tm_src_add_ref as *const () as usize,
        tm_src_release as *const () as usize,
        tm_src_advise as *const () as usize,
        tm_src_unadvise as *const () as usize,
        tm_ss_qi as *const () as usize,
        tm_ss_add_ref as *const () as usize,
        tm_ss_release as *const () as usize,
        tm_ss_advise_single as *const () as usize,
        tm_ss_unadvise_single as *const () as usize,
        tm_ui_qi as *const () as usize,
        tm_ui_add_ref as *const () as usize,
        tm_ui_release as *const () as usize,
        tm_ui_begin as *const () as usize,
        tm_ui_update as *const () as usize,
        tm_ui_end as *const () as usize,
        tm_ui_get as *const () as usize,
        tm_ui_enum as *const () as usize,
        tm_cm_qi as *const () as usize,
        tm_cm_add_ref as *const () as usize,
        tm_cm_release as *const () as usize,
        tm_cm_get as *const () as usize,
        tm_cm_clear as *const () as usize,
        tm_cm_enum as *const () as usize,
        tm_ks_qi as *const () as usize,
        tm_ks_add_ref as *const () as usize,
        tm_ks_release as *const () as usize,
        tm_ks_advise_key_event_sink as *const () as usize,
        tm_ks_unadvise_key_event_sink as *const () as usize,
        tm_ks_get_foreground as *const () as usize,
        tm_ks_test_or_eat as *const () as usize,
        tm_ks_get_preserved_key as *const () as usize,
        tm_ks_is_preserved_key as *const () as usize,
        tm_ks_preserve_key as *const () as usize,
        tm_ks_unpreserve_key as *const () as usize,
        tm_ks_set_preserved_key_desc as *const () as usize,
        tm_ks_get_preserved_key_desc as *const () as usize,
        tm_ks_simulate_preserved_key as *const () as usize,
    ]);
}

/// Returns an `AddRef`'d `ThreadMgrObj` for the calling Win32 thread, creating
/// one if none is currently alive on that thread.
pub(super) unsafe fn get_or_create_thread_mgr() -> *mut ThreadMgrObj {
    register_thread_mgr_vtables();
    let tid = current_tid();
    let mut reg = THREAD_MGR_REGISTRY.lock().unwrap();
    if let Some(&raw) = reg.get(&tid) {
        let ptr = raw as *mut ThreadMgrObj;
        let obj = &*ptr;
        if obj
            .refs
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |r| (r > 0).then_some(r + 1))
            .is_ok()
        {
            return ptr;
        }
    }
    let ptr = Box::into_raw(Box::new(ThreadMgrObj {
        vtbl_tm_ex: &ITF_THREAD_MGR_EX_VTBL,
        vtbl_tm2: &ITF_THREAD_MGR2_VTBL,
        vtbl_source: &ITF_THREAD_MGR_SRC_VTBL,
        vtbl_source_single: &ITF_THREAD_MGR_SS_VTBL,
        vtbl_ui_elem: &ITF_UI_ELEMENT_MGR_VTBL,
        vtbl_comp_mgr: &ITF_THREAD_MGR_CM_VTBL,
        vtbl_keystroke: &ITF_KEYSTROKE_MGR_VTBL,
        refs: AtomicU32::new(1),
        tid,
        state: Arc::new(Mutex::new(ThreadMgrState::default())),
        sinks: Arc::new(Mutex::new(SinkTable::default())),
        compartments: Mutex::new(CompartmentMap::default()),
    }));
    reg.insert(tid, ptr as usize);
    ptr
}

/// Returns an `AddRef`'d `ThreadMgrObj` for the calling Win32 thread if one
/// already exists, or `None` if no thread manager is live on this thread.
pub(super) unsafe fn get_existing_thread_mgr() -> Option<*mut ThreadMgrObj> {
    let tid = current_tid();
    let reg = THREAD_MGR_REGISTRY.lock().unwrap();
    let raw = reg.get(&tid).copied()?;
    let ptr = raw as *mut ThreadMgrObj;
    let obj = &*ptr;
    obj.refs
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |r| (r > 0).then_some(r + 1))
        .ok()
        .map(|_| ptr)
}
