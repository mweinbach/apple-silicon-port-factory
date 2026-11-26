//! TSF compartments (`ITfCompartmentMgr`, `ITfCompartment`), contexts (`ITfContext`),
//! and document managers (`ITfDocumentMgr`).

use super::abi::*;
use super::range::{new_property_obj, new_range_obj, new_tracked_property_obj, PropertyData};
use super::thread_mgr::ThreadMgrState;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

// ---------------------------------------------------------------------------
// Compartments (`ITfCompartmentMgr` [6 slots], `ITfCompartment` [5 slots] + `ITfSource`)
// ---------------------------------------------------------------------------

struct CompartmentCell {
    guid: Guid,
    value: Mutex<OwnedVariant>,
    sinks: Mutex<SinkTable>,
}

impl CompartmentCell {
    fn new(guid: Guid) -> Arc<Self> {
        Arc::new(Self {
            guid,
            value: Mutex::new(OwnedVariant::default()),
            sinks: Mutex::new(SinkTable::default()),
        })
    }
}

impl Drop for CompartmentCell {
    fn drop(&mut self) {
        let sinks = self.sinks.lock().unwrap().drain_all();
        unsafe { release_all_ptrs(sinks) };
    }
}

#[derive(Default)]
pub(super) struct CompartmentMap {
    entries: HashMap<Guid, Arc<CompartmentCell>>,
}

impl CompartmentMap {
    fn get_or_create(&mut self, guid: Guid) -> Arc<CompartmentCell> {
        Arc::clone(self.entries.entry(guid).or_insert_with(|| CompartmentCell::new(guid)))
    }
}

pub(super) static GLOBAL_COMPARTMENTS: LazyLock<Arc<Mutex<CompartmentMap>>> =
    LazyLock::new(|| Arc::new(Mutex::new(CompartmentMap::default())));

#[repr(C)]
pub(super) struct ITfCompartmentVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) set_value: unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
    pub(super) get_value: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfCompartmentVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
struct CompartmentObj {
    vtbl_comp: *const ITfCompartmentVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    cell: Arc<CompartmentCell>,
}

unsafe fn comp_qi_common(
    base: *mut CompartmentObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let src_ptr = (base as *mut u8).add(std::mem::offset_of!(CompartmentObj, vtbl_source))
        as *mut c_void;
    qi_multi_iface(
        &(*base).refs,
        riid,
        ppv,
        base as *mut c_void,
        &[(IID_ITFCOMPARTMENT, base as *mut c_void), (IID_ITFSOURCE, src_ptr)],
    )
}

unsafe fn comp_add_ref_common(base: *mut CompartmentObj) -> u32 {
    (*base).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn comp_release_common(base: *mut CompartmentObj) -> u32 {
    let rem = (*base).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(base));
    }
    rem
}

impl_primary_iunknown!(
    comp_qi,
    comp_add_ref,
    comp_release,
    CompartmentObj,
    comp_qi_common,
    comp_add_ref_common,
    comp_release_common
);
impl_sub_iunknown!(
    comp_from_source,
    comp_src_qi,
    comp_src_add_ref,
    comp_src_release,
    CompartmentObj,
    vtbl_source,
    comp_qi_common,
    comp_add_ref_common,
    comp_release_common
);

unsafe extern "win64" fn comp_set_value(
    this: *mut c_void,
    _tid: TfClientId,
    pvar_value: *const u8,
) -> HRESULT {
    if pvar_value.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const CompartmentObj);
    let new_val = OwnedVariant::from_raw(pvar_value);
    let old_raw = {
        let mut guard = obj.cell.value.lock().unwrap();
        let old = guard.take_raw();
        *guard = new_val;
        old
    };
    release_variant_raw(old_raw);
    let guid_copy = obj.cell.guid;
    let sinks = snapshot_sinks_addref(&obj.cell.sinks, &IID_ITFCOMPARTMENTEVENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 3, guid_as_ptr(&guid_copy) as u64, 0, 0);
        com_call_release(sink);
    }
    S_OK
}

unsafe extern "win64" fn comp_get_value(this: *mut c_void, pvar_value: *mut u8) -> HRESULT {
    if pvar_value.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_bytes(pvar_value, 0, 24);
    let obj = &*(this as *const CompartmentObj);
    let guard = obj.cell.value.lock().unwrap();
    if guard.is_empty() {
        return S_FALSE;
    }
    guard.write_copy_to(pvar_value);
    S_OK
}

unsafe extern "win64" fn comp_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*comp_from_source(this);
    sink_advise(&obj.cell.sinks, riid, punk, pdw_cookie)
}

unsafe extern "win64" fn comp_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*comp_from_source(this);
    sink_unadvise(&obj.cell.sinks, cookie)
}

static ITF_COMPARTMENT_VTBL: ITfCompartmentVtbl = ITfCompartmentVtbl {
    query_interface: comp_qi,
    add_ref: comp_add_ref,
    release: comp_release,
    set_value: comp_set_value,
    get_value: comp_get_value,
};

static ITF_COMPARTMENT_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: comp_src_qi,
    add_ref: comp_src_add_ref,
    release: comp_src_release,
    advise_sink: comp_src_advise,
    unadvise_sink: comp_src_unadvise,
};

fn new_compartment_obj(cell: Arc<CompartmentCell>) -> *mut CompartmentObj {
    register_vtable_ptrs(&[
        comp_qi as *const () as usize,
        comp_add_ref as *const () as usize,
        comp_release as *const () as usize,
        comp_set_value as *const () as usize,
        comp_get_value as *const () as usize,
        comp_src_qi as *const () as usize,
        comp_src_add_ref as *const () as usize,
        comp_src_release as *const () as usize,
        comp_src_advise as *const () as usize,
        comp_src_unadvise as *const () as usize,
    ]);
    Box::into_raw(Box::new(CompartmentObj {
        vtbl_comp: &ITF_COMPARTMENT_VTBL,
        vtbl_source: &ITF_COMPARTMENT_SRC_VTBL,
        refs: AtomicU32::new(1),
        cell,
    }))
}

pub(super) unsafe fn comp_mgr_get_compartment_impl(
    store: &Mutex<CompartmentMap>,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    write_null_out(ppcomp);
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    if ppcomp.is_null() {
        return E_INVALIDARG;
    }
    let cell = store.lock().unwrap().get_or_create(g);
    let obj = new_compartment_obj(cell);
    std::ptr::write_unaligned(ppcomp, obj as *mut c_void);
    S_OK
}

pub(super) unsafe fn comp_mgr_clear_compartment_impl(
    store: &Mutex<CompartmentMap>,
    _tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid_from_bytes(rguid) else {
        return E_INVALIDARG;
    };
    let cell = store.lock().unwrap().entries.get(&g).cloned();
    if let Some(c) = cell {
        let old_raw = c.value.lock().unwrap().take_raw();
        release_variant_raw(old_raw);
        let sinks = snapshot_sinks_addref(&c.sinks, &IID_ITFCOMPARTMENTEVENTSINK);
        for sink in sinks {
            let _ = com_call_slot(sink, 3, guid_as_ptr(&g) as u64, 0, 0);
            com_call_release(sink);
        }
    }
    S_OK
}

pub(super) unsafe fn comp_mgr_enum_compartments_impl(
    store: &Mutex<CompartmentMap>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let guids: Vec<Guid> = {
        let map = store.lock().unwrap();
        map.entries
            .iter()
            .filter_map(|(&g, cell)| (!cell.value.lock().unwrap().is_empty()).then_some(g))
            .collect()
    };
    let en = new_enum_guid(guids, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

/// Standalone `ITfCompartmentMgr` object (returned by `GetGlobalCompartment`).
pub(super) struct CompartmentMgrInner {
    store: Arc<Mutex<CompartmentMap>>,
}

pub(super) type CompartmentMgrObj = ComObj<CompartmentMgrInner>;

unsafe extern "win64" fn cmgr_qi(
    this: *mut c_void,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper_bytes(this as *mut CompartmentMgrObj, riid, ppv, &[IID_ITFCOMPARTMENTMGR])
}
unsafe extern "win64" fn cmgr_add_ref(this: *mut c_void) -> u32 {
    ComObj::add_ref(this as *mut CompartmentMgrObj)
}
unsafe extern "win64" fn cmgr_release(this: *mut c_void) -> u32 {
    ComObj::release(this as *mut CompartmentMgrObj)
}
unsafe extern "win64" fn cmgr_get_compartment(
    this: *mut c_void,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    comp_mgr_get_compartment_impl(&ComObj::inner(this as *mut CompartmentMgrObj).store, rguid, ppcomp)
}
unsafe extern "win64" fn cmgr_clear_compartment(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    comp_mgr_clear_compartment_impl(&ComObj::inner(this as *mut CompartmentMgrObj).store, tid, rguid)
}
unsafe extern "win64" fn cmgr_enum_compartments(
    this: *mut c_void,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    comp_mgr_enum_compartments_impl(&ComObj::inner(this as *mut CompartmentMgrObj).store, pp_enum)
}

static STANDALONE_COMPARTMENT_MGR_VTBL: ITfCompartmentMgrVtbl = ITfCompartmentMgrVtbl {
    query_interface: cmgr_qi,
    add_ref: cmgr_add_ref,
    release: cmgr_release,
    get_compartment: cmgr_get_compartment,
    clear_compartment: cmgr_clear_compartment,
    enum_compartments: cmgr_enum_compartments,
};

pub(super) fn new_compartment_mgr_obj(store: Arc<Mutex<CompartmentMap>>) -> *mut CompartmentMgrObj {
    register_vtable_ptrs(&[
        cmgr_qi as *const () as usize,
        cmgr_add_ref as *const () as usize,
        cmgr_release as *const () as usize,
        cmgr_get_compartment as *const () as usize,
        cmgr_clear_compartment as *const () as usize,
        cmgr_enum_compartments as *const () as usize,
    ]);
    ComObj::new(
        &STANDALONE_COMPARTMENT_MGR_VTBL as *const ITfCompartmentMgrVtbl as *const usize,
        CompartmentMgrInner { store },
    )
}

// ---------------------------------------------------------------------------
// `ITfContext` (18 slots) + `ITfSource` + `ITfSourceSingle` + `ITfCompartmentMgr`
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfContextVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) request_edit_session:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *mut c_void, u32, *mut HRESULT) -> HRESULT,
    pub(super) in_write_session:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *mut BOOL) -> HRESULT,
    pub(super) get_selection:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, u32, u32, *mut u8, *mut u32) -> HRESULT,
    pub(super) set_selection:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, u32, *const u8) -> HRESULT,
    pub(super) get_start:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut *mut c_void) -> HRESULT,
    pub(super) get_end:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut *mut c_void) -> HRESULT,
    pub(super) get_active_view: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) enum_views: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_status: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
    pub(super) get_property:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) get_app_property:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) track_properties: unsafe extern "win64" fn(
        *mut c_void,
        *const *const u8,
        u32,
        *const *const u8,
        u32,
        *mut *mut c_void,
    ) -> HRESULT,
    pub(super) enum_properties: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) create_range_backup:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfContextVtbl>() == 18 * std::mem::size_of::<usize>());

struct StoredSelection {
    range: usize,
    style: TF_SELECTIONSTYLE,
}

pub(super) struct ContextInner {
    pub(super) live_self: usize,
    doc_mgr_inner: Arc<Mutex<DocMgrInner>>,
    #[allow(dead_code)]
    owner_tid: TfClientId,
    text_store_unk: usize,
    text_store_cookie: TfEditCookie,
    active_edit_cookie: TfEditCookie,
    in_write_session_tid: Option<TfClientId>,
    next_edit_cookie: TfEditCookie,
    selections: Vec<StoredSelection>,
    properties: HashMap<Guid, Arc<PropertyData>>,
    single_sinks: HashMap<(TfClientId, Guid), usize>,
}

#[repr(C)]
pub(super) struct ContextObj {
    vtbl_ctx: *const ITfContextVtbl,
    vtbl_source: *const ITfSourceVtbl,
    vtbl_source_single: *const ITfSourceSingleVtbl,
    vtbl_comp_mgr: *const ITfCompartmentMgrVtbl,
    refs: AtomicU32,
    inner: Arc<Mutex<ContextInner>>,
    sinks: Mutex<SinkTable>,
    compartments: Mutex<CompartmentMap>,
}

pub(super) unsafe fn try_add_ref_context(raw: usize) -> Option<*mut ContextObj> {
    if raw == 0 {
        return None;
    }
    let ptr = raw as *mut ContextObj;
    let obj = &*ptr;
    obj.refs
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |r| (r > 0).then_some(r + 1))
        .ok()
        .map(|_| ptr)
}

unsafe fn ctx_qi_common(
    base: *mut ContextObj,
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
            (IID_ITFCONTEXT, base as *mut c_void),
            (IID_ITFSOURCE, b.add(std::mem::offset_of!(ContextObj, vtbl_source)) as *mut c_void),
            (
                IID_ITFSOURCESINGLE,
                b.add(std::mem::offset_of!(ContextObj, vtbl_source_single)) as *mut c_void,
            ),
            (
                IID_ITFCOMPARTMENTMGR,
                b.add(std::mem::offset_of!(ContextObj, vtbl_comp_mgr)) as *mut c_void,
            ),
        ],
    )
}

unsafe fn ctx_add_ref_common(base: *mut ContextObj) -> u32 {
    (*base).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn ctx_release_common(base: *mut ContextObj) -> u32 {
    let obj = &*base;
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    if prev > 1 {
        return prev - 1;
    }
    let to_release: Vec<*mut c_void> = {
        let mut st = obj.inner.lock().unwrap();
        let cur = obj.refs.load(Ordering::Acquire);
        if cur != 0 {
            return cur;
        }
        st.live_self = 0;
        let mut list = Vec::new();
        if st.text_store_unk != 0 {
            list.push(st.text_store_unk as *mut c_void);
            st.text_store_unk = 0;
        }
        for sel in st.selections.drain(..) {
            if sel.range != 0 {
                list.push(sel.range as *mut c_void);
            }
        }
        for (_, p) in st.single_sinks.drain() {
            if p != 0 {
                list.push(p as *mut c_void);
            }
        }
        list
    };
    let mut sinks = obj.sinks.lock().unwrap().drain_all();
    sinks.extend(to_release);
    drop(Box::from_raw(base));
    release_all_ptrs(sinks);
    0
}

fn ctx_cookie_valid(st: &ContextInner, ec: TfEditCookie) -> bool {
    ec != TF_INVALID_EDIT_COOKIE && (ec == st.text_store_cookie || ec == st.active_edit_cookie)
}

impl_primary_iunknown!(
    ctx_qi,
    ctx_add_ref,
    ctx_release,
    ContextObj,
    ctx_qi_common,
    ctx_add_ref_common,
    ctx_release_common
);
impl_sub_iunknown!(
    ctx_from_source,
    ctx_src_qi,
    ctx_src_add_ref,
    ctx_src_release,
    ContextObj,
    vtbl_source,
    ctx_qi_common,
    ctx_add_ref_common,
    ctx_release_common
);
impl_sub_iunknown!(
    ctx_from_source_single,
    ctx_ss_qi,
    ctx_ss_add_ref,
    ctx_ss_release,
    ContextObj,
    vtbl_source_single,
    ctx_qi_common,
    ctx_add_ref_common,
    ctx_release_common
);
impl_sub_iunknown!(
    ctx_from_comp_mgr,
    ctx_cm_qi,
    ctx_cm_add_ref,
    ctx_cm_release,
    ContextObj,
    vtbl_comp_mgr,
    ctx_qi_common,
    ctx_add_ref_common,
    ctx_release_common
);

unsafe extern "win64" fn ctx_request_edit_session(
    this: *mut c_void,
    tid: TfClientId,
    pes: *mut c_void,
    dw_flags: u32,
    phr_session: *mut HRESULT,
) -> HRESULT {
    if !phr_session.is_null() {
        std::ptr::write_unaligned(phr_session, E_FAIL);
    }
    if pes.is_null() || phr_session.is_null() {
        return E_INVALIDARG;
    }
    if (dw_flags & (TF_ES_SYNC | TF_ES_ASYNC)) == (TF_ES_SYNC | TF_ES_ASYNC) {
        return E_INVALIDARG;
    }
    if (dw_flags & TF_ES_READ) == 0 {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let is_write = (dw_flags & TF_ES_READWRITE) == TF_ES_READWRITE;
    let ec = {
        let mut st = obj.inner.lock().unwrap();
        if st.active_edit_cookie != TF_INVALID_EDIT_COOKIE {
            if (dw_flags & TF_ES_SYNC) != 0 {
                std::ptr::write_unaligned(phr_session, TF_E_SYNCHRONOUS);
                return S_OK;
            }
            return TF_E_SYNCHRONOUS;
        }
        let cookie = st.next_edit_cookie;
        st.next_edit_cookie = cookie.wrapping_add(1).max(1);
        st.active_edit_cookie = cookie;
        st.in_write_session_tid = is_write.then_some(tid);
        cookie
    };
    com_call_add_ref(pes);
    let session_hr = com_call_slot(pes, 3, ec as u64, 0, 0);
    com_call_release(pes);
    {
        let mut st = obj.inner.lock().unwrap();
        st.active_edit_cookie = TF_INVALID_EDIT_COOKIE;
        st.in_write_session_tid = None;
    }
    std::ptr::write_unaligned(phr_session, session_hr);
    S_OK
}

unsafe extern "win64" fn ctx_in_write_session(
    this: *mut c_void,
    tid: TfClientId,
    pf_write_session: *mut BOOL,
) -> HRESULT {
    if pf_write_session.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let in_write = obj.inner.lock().unwrap().in_write_session_tid == Some(tid);
    std::ptr::write_unaligned(pf_write_session, if in_write { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn ctx_get_selection(
    this: *mut c_void,
    ec: TfEditCookie,
    ul_index: u32,
    ul_count: u32,
    p_selection: *mut u8,
    pc_fetched: *mut u32,
) -> HRESULT {
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, 0);
    }
    if p_selection.is_null() || pc_fetched.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let snapshot: Vec<StoredSelection> = {
        let st = obj.inner.lock().unwrap();
        if !ctx_cookie_valid(&st, ec) {
            return TF_E_NOLOCK;
        }
        if st.selections.is_empty() {
            return TF_E_NOSELECTION;
        }
        let start = if ul_index == u32::MAX { 0 } else { ul_index as usize };
        if start >= st.selections.len() {
            return TF_E_NOSELECTION;
        }
        st.selections[start..]
            .iter()
            .take(ul_count as usize)
            .map(|s| StoredSelection { range: s.range, style: s.style })
            .collect()
    };
    for (i, s) in snapshot.iter().enumerate() {
        if s.range != 0 {
            com_call_add_ref(s.range as *mut c_void);
        }
        let out = TF_SELECTION { range: s.range as *mut c_void, style: s.style };
        std::ptr::write_unaligned(
            p_selection.add(i * std::mem::size_of::<TF_SELECTION>()) as *mut TF_SELECTION,
            out,
        );
    }
    std::ptr::write_unaligned(pc_fetched, snapshot.len() as u32);
    S_OK
}

unsafe extern "win64" fn ctx_set_selection(
    this: *mut c_void,
    ec: TfEditCookie,
    ul_count: u32,
    p_selection: *const u8,
) -> HRESULT {
    if ul_count > 0 && p_selection.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let mut next_sels = Vec::with_capacity(ul_count as usize);
    for i in 0..ul_count as usize {
        let sel = std::ptr::read_unaligned(
            p_selection.add(i * std::mem::size_of::<TF_SELECTION>()) as *const TF_SELECTION,
        );
        if !sel.range.is_null() {
            com_call_add_ref(sel.range);
        }
        next_sels.push(StoredSelection { range: sel.range as usize, style: sel.style });
    }
    let old_ranges: Vec<*mut c_void> = {
        let mut st = obj.inner.lock().unwrap();
        if !ctx_cookie_valid(&st, ec) {
            drop(st);
            for s in next_sels {
                if s.range != 0 {
                    com_call_release(s.range as *mut c_void);
                }
            }
            return TF_E_NOLOCK;
        }
        std::mem::replace(&mut st.selections, next_sels)
            .into_iter()
            .filter_map(|s| (s.range != 0).then_some(s.range as *mut c_void))
            .collect()
    };
    release_all_ptrs(old_ranges);
    S_OK
}

unsafe extern "win64" fn ctx_get_start(
    this: *mut c_void,
    ec: TfEditCookie,
    pp_start: *mut *mut c_void,
) -> HRESULT {
    if pp_start.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_start);
    let obj = &*(this as *const ContextObj);
    if !ctx_cookie_valid(&obj.inner.lock().unwrap(), ec) {
        return TF_E_NOLOCK;
    }
    let r = new_range_obj(Arc::clone(&obj.inner), (0, 0));
    std::ptr::write_unaligned(pp_start, r as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ctx_get_end(
    this: *mut c_void,
    ec: TfEditCookie,
    pp_end: *mut *mut c_void,
) -> HRESULT {
    if pp_end.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_end);
    let obj = &*(this as *const ContextObj);
    if !ctx_cookie_valid(&obj.inner.lock().unwrap(), ec) {
        return TF_E_NOLOCK;
    }
    let r = new_range_obj(Arc::clone(&obj.inner), (0, 0));
    std::ptr::write_unaligned(pp_end, r as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ctx_get_active_view(_this: *mut c_void, pp_view: *mut *mut c_void) -> HRESULT {
    if pp_view.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_view);
    TF_E_INVALIDVIEW
}

unsafe extern "win64" fn ctx_enum_views(_this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(
        pp_enum,
        new_enum_tf_com(IID_IENUMTFCONTEXTVIEWS, Vec::new(), 0) as *mut c_void,
    );
    S_OK
}

unsafe extern "win64" fn ctx_get_status(_this: *mut c_void, pdcs: *mut u8) -> HRESULT {
    if pdcs.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pdcs as *mut TF_STATUS, TF_STATUS::default());
    S_OK
}

unsafe extern "win64" fn ctx_get_property(
    this: *mut c_void,
    guid_prop: *const u8,
    pp_prop: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_prop);
    let Some(g) = read_guid_from_bytes(guid_prop) else {
        return E_INVALIDARG;
    };
    if pp_prop.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let data = {
        let mut st = obj.inner.lock().unwrap();
        Arc::clone(st.properties.entry(g).or_insert_with(|| {
            Arc::new(PropertyData { guid_type: g, value: Mutex::new(OwnedVariant::default()) })
        }))
    };
    let p = new_property_obj(data, Arc::clone(&obj.inner), false);
    std::ptr::write_unaligned(pp_prop, p as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ctx_get_app_property(
    this: *mut c_void,
    guid_prop: *const u8,
    pp_prop: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_prop);
    let Some(g) = read_guid_from_bytes(guid_prop) else {
        return E_INVALIDARG;
    };
    if pp_prop.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const ContextObj);
    let data = {
        let mut st = obj.inner.lock().unwrap();
        Arc::clone(st.properties.entry(g).or_insert_with(|| {
            Arc::new(PropertyData { guid_type: g, value: Mutex::new(OwnedVariant::default()) })
        }))
    };
    let p = new_property_obj(data, Arc::clone(&obj.inner), true);
    std::ptr::write_unaligned(pp_prop, p as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ctx_track_properties(
    this: *mut c_void,
    prg_prop: *const *const u8,
    c_prop: u32,
    prg_app_prop: *const *const u8,
    c_app_prop: u32,
    pp_property: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_property);
    if pp_property.is_null() {
        return E_INVALIDARG;
    }
    if (c_prop == 0 && c_app_prop == 0)
        || (c_prop > 0 && prg_prop.is_null())
        || (c_app_prop > 0 && prg_app_prop.is_null())
    {
        return E_INVALIDARG;
    }
    let mut guids = Vec::with_capacity((c_prop as usize).saturating_add(c_app_prop as usize));
    for i in 0..c_prop as usize {
        let gp = std::ptr::read_unaligned(prg_prop.add(i));
        let Some(g) = read_guid_from_bytes(gp) else {
            return E_INVALIDARG;
        };
        guids.push(g);
    }
    for i in 0..c_app_prop as usize {
        let gp = std::ptr::read_unaligned(prg_app_prop.add(i));
        let Some(g) = read_guid_from_bytes(gp) else {
            return E_INVALIDARG;
        };
        guids.push(g);
    }
    let obj = &*(this as *const ContextObj);
    let props: Vec<Arc<PropertyData>> = {
        let mut st = obj.inner.lock().unwrap();
        guids
            .into_iter()
            .map(|g| {
                Arc::clone(st.properties.entry(g).or_insert_with(|| {
                    Arc::new(PropertyData {
                        guid_type: g,
                        value: Mutex::new(OwnedVariant::default()),
                    })
                }))
            })
            .collect()
    };
    let tracked = new_tracked_property_obj(props, Arc::clone(&obj.inner));
    std::ptr::write_unaligned(pp_property, tracked as *mut c_void);
    S_OK
}

unsafe extern "win64" fn ctx_enum_properties(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let obj = &*(this as *const ContextObj);
    let props: Vec<Arc<PropertyData>> =
        obj.inner.lock().unwrap().properties.values().cloned().collect();
    let ptrs: Vec<usize> = props
        .into_iter()
        .map(|d| new_property_obj(d, Arc::clone(&obj.inner), false) as usize)
        .collect();
    std::ptr::write_unaligned(
        pp_enum,
        new_enum_tf_com(IID_IENUMTFPROPERTIES, ptrs, 0) as *mut c_void,
    );
    S_OK
}

unsafe extern "win64" fn ctx_get_document_mgr(this: *mut c_void, pp_dm: *mut *mut c_void) -> HRESULT {
    if pp_dm.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_dm);
    let obj = &*(this as *const ContextObj);
    let dm_inner = Arc::clone(&obj.inner.lock().unwrap().doc_mgr_inner);
    let maybe_dm = {
        let st = dm_inner.lock().unwrap();
        try_add_ref_doc_mgr(st.live_self)
    };
    match maybe_dm {
        Some(ptr) => {
            std::ptr::write_unaligned(pp_dm, ptr as *mut c_void);
            S_OK
        }
        None => TF_E_DISCONNECTED,
    }
}

unsafe extern "win64" fn ctx_create_range_backup(
    _this: *mut c_void,
    _ec: TfEditCookie,
    _p_range: *mut c_void,
    pp_backup: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_backup);
    E_NOTIMPL
}

unsafe extern "win64" fn ctx_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*ctx_from_source(this);
    sink_advise(&obj.sinks, riid, punk, pdw_cookie)
}

unsafe extern "win64" fn ctx_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*ctx_from_source(this);
    sink_unadvise(&obj.sinks, cookie)
}

unsafe extern "win64" fn ctx_ss_advise_single(
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
    let obj = &*ctx_from_source_single(this);
    let mut st = obj.inner.lock().unwrap();
    if st.single_sinks.contains_key(&(tid, iid)) {
        drop(st);
        com_call_release(iface);
        return CONNECT_E_ADVISELIMIT;
    }
    st.single_sinks.insert((tid, iid), iface as usize);
    S_OK
}

unsafe extern "win64" fn ctx_ss_unadvise_single(
    this: *mut c_void,
    tid: TfClientId,
    riid: *const u8,
) -> HRESULT {
    let Some(iid) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*ctx_from_source_single(this);
    let removed = obj.inner.lock().unwrap().single_sinks.remove(&(tid, iid));
    match removed {
        Some(p) => {
            com_call_release(p as *mut c_void);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

unsafe extern "win64" fn ctx_cm_get(
    this: *mut c_void,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    let obj = &*ctx_from_comp_mgr(this);
    comp_mgr_get_compartment_impl(&obj.compartments, rguid, ppcomp)
}

unsafe extern "win64" fn ctx_cm_clear(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let obj = &*ctx_from_comp_mgr(this);
    comp_mgr_clear_compartment_impl(&obj.compartments, tid, rguid)
}

unsafe extern "win64" fn ctx_cm_enum(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    let obj = &*ctx_from_comp_mgr(this);
    comp_mgr_enum_compartments_impl(&obj.compartments, pp_enum)
}

static ITF_CONTEXT_VTBL: ITfContextVtbl = ITfContextVtbl {
    query_interface: ctx_qi,
    add_ref: ctx_add_ref,
    release: ctx_release,
    request_edit_session: ctx_request_edit_session,
    in_write_session: ctx_in_write_session,
    get_selection: ctx_get_selection,
    set_selection: ctx_set_selection,
    get_start: ctx_get_start,
    get_end: ctx_get_end,
    get_active_view: ctx_get_active_view,
    enum_views: ctx_enum_views,
    get_status: ctx_get_status,
    get_property: ctx_get_property,
    get_app_property: ctx_get_app_property,
    track_properties: ctx_track_properties,
    enum_properties: ctx_enum_properties,
    get_document_mgr: ctx_get_document_mgr,
    create_range_backup: ctx_create_range_backup,
};

static ITF_CONTEXT_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: ctx_src_qi,
    add_ref: ctx_src_add_ref,
    release: ctx_src_release,
    advise_sink: ctx_src_advise,
    unadvise_sink: ctx_src_unadvise,
};

static ITF_CONTEXT_SS_VTBL: ITfSourceSingleVtbl = ITfSourceSingleVtbl {
    query_interface: ctx_ss_qi,
    add_ref: ctx_ss_add_ref,
    release: ctx_ss_release,
    advise_single_sink: ctx_ss_advise_single,
    unadvise_single_sink: ctx_ss_unadvise_single,
};

static ITF_CONTEXT_CM_VTBL: ITfCompartmentMgrVtbl = ITfCompartmentMgrVtbl {
    query_interface: ctx_cm_qi,
    add_ref: ctx_cm_add_ref,
    release: ctx_cm_release,
    get_compartment: ctx_cm_get,
    clear_compartment: ctx_cm_clear,
    enum_compartments: ctx_cm_enum,
};

fn new_context_obj(
    doc_mgr_inner: Arc<Mutex<DocMgrInner>>,
    owner_tid: TfClientId,
    text_store_unk: usize,
    text_store_cookie: TfEditCookie,
) -> *mut ContextObj {
    register_vtable_ptrs(&[
        ctx_qi as *const () as usize,
        ctx_add_ref as *const () as usize,
        ctx_release as *const () as usize,
        ctx_request_edit_session as *const () as usize,
        ctx_in_write_session as *const () as usize,
        ctx_get_selection as *const () as usize,
        ctx_set_selection as *const () as usize,
        ctx_get_start as *const () as usize,
        ctx_get_end as *const () as usize,
        ctx_get_active_view as *const () as usize,
        ctx_enum_views as *const () as usize,
        ctx_get_status as *const () as usize,
        ctx_get_property as *const () as usize,
        ctx_get_app_property as *const () as usize,
        ctx_track_properties as *const () as usize,
        ctx_enum_properties as *const () as usize,
        ctx_get_document_mgr as *const () as usize,
        ctx_create_range_backup as *const () as usize,
        ctx_src_qi as *const () as usize,
        ctx_src_add_ref as *const () as usize,
        ctx_src_release as *const () as usize,
        ctx_src_advise as *const () as usize,
        ctx_src_unadvise as *const () as usize,
        ctx_ss_qi as *const () as usize,
        ctx_ss_add_ref as *const () as usize,
        ctx_ss_release as *const () as usize,
        ctx_ss_advise_single as *const () as usize,
        ctx_ss_unadvise_single as *const () as usize,
        ctx_cm_qi as *const () as usize,
        ctx_cm_add_ref as *const () as usize,
        ctx_cm_release as *const () as usize,
        ctx_cm_get as *const () as usize,
        ctx_cm_clear as *const () as usize,
        ctx_cm_enum as *const () as usize,
    ]);
    let inner = Arc::new(Mutex::new(ContextInner {
        live_self: 0,
        doc_mgr_inner,
        owner_tid,
        text_store_unk,
        text_store_cookie,
        active_edit_cookie: TF_INVALID_EDIT_COOKIE,
        in_write_session_tid: None,
        next_edit_cookie: text_store_cookie.wrapping_add(1).max(2),
        selections: Vec::new(),
        properties: HashMap::new(),
        single_sinks: HashMap::new(),
    }));
    let ptr = Box::into_raw(Box::new(ContextObj {
        vtbl_ctx: &ITF_CONTEXT_VTBL,
        vtbl_source: &ITF_CONTEXT_SRC_VTBL,
        vtbl_source_single: &ITF_CONTEXT_SS_VTBL,
        vtbl_comp_mgr: &ITF_CONTEXT_CM_VTBL,
        refs: AtomicU32::new(1),
        inner: Arc::clone(&inner),
        sinks: Mutex::new(SinkTable::default()),
        compartments: Mutex::new(CompartmentMap::default()),
    }));
    inner.lock().unwrap().live_self = ptr as usize;
    ptr
}

// ---------------------------------------------------------------------------
// `ITfDocumentMgr` (9 slots) + `ITfCompartmentMgr` + `ITfSource`
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfDocumentMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) create_context: unsafe extern "win64" fn(
        *mut c_void,
        TfClientId,
        u32,
        *mut c_void,
        *mut *mut c_void,
        *mut TfEditCookie,
    ) -> HRESULT,
    pub(super) push: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    pub(super) pop: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    pub(super) get_top: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) get_base: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub(super) enum_contexts: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfDocumentMgrVtbl>() == 9 * std::mem::size_of::<usize>());

struct DocMgrInner {
    live_self: usize,
    tm_state: Arc<Mutex<ThreadMgrState>>,
    tm_sinks: Arc<Mutex<SinkTable>>,
    contexts: Vec<usize>,
    next_text_store_cookie: TfEditCookie,
}

#[repr(C)]
pub(super) struct DocumentMgrObj {
    vtbl_dm: *const ITfDocumentMgrVtbl,
    vtbl_comp_mgr: *const ITfCompartmentMgrVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    inner: Arc<Mutex<DocMgrInner>>,
    compartments: Mutex<CompartmentMap>,
    sinks: Mutex<SinkTable>,
}

pub(super) unsafe fn try_add_ref_doc_mgr(raw: usize) -> Option<*mut DocumentMgrObj> {
    if raw == 0 {
        return None;
    }
    let ptr = raw as *mut DocumentMgrObj;
    let obj = &*ptr;
    obj.refs
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |r| (r > 0).then_some(r + 1))
        .ok()
        .map(|_| ptr)
}

unsafe fn dm_qi_common(
    base: *mut DocumentMgrObj,
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
            (IID_ITFDOCUMENTMGR, base as *mut c_void),
            (
                IID_ITFCOMPARTMENTMGR,
                b.add(std::mem::offset_of!(DocumentMgrObj, vtbl_comp_mgr)) as *mut c_void,
            ),
            (IID_ITFSOURCE, b.add(std::mem::offset_of!(DocumentMgrObj, vtbl_source)) as *mut c_void),
        ],
    )
}

unsafe fn dm_add_ref_common(base: *mut DocumentMgrObj) -> u32 {
    (*base).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn dm_release_common(base: *mut DocumentMgrObj) -> u32 {
    let obj = &*base;
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    if prev > 1 {
        return prev - 1;
    }
    let (tm_state, tm_sinks, contexts) = {
        let mut st = obj.inner.lock().unwrap();
        let cur = obj.refs.load(Ordering::Acquire);
        if cur != 0 {
            return cur;
        }
        st.live_self = 0;
        let ctxs: Vec<*mut c_void> = st.contexts.drain(..).map(|p| p as *mut c_void).collect();
        (Arc::clone(&st.tm_state), Arc::clone(&st.tm_sinks), ctxs)
    };
    // Remove from ThreadMgr's weak document-manager list before freeing.
    {
        let mut tm = tm_state.lock().unwrap();
        tm.doc_mgrs.retain(|&p| p != base as usize);
    }
    let _ = tm_sinks;
    let sinks = obj.sinks.lock().unwrap().drain_all();
    drop(Box::from_raw(base));
    release_all_ptrs(contexts);
    release_all_ptrs(sinks);
    0
}

impl_primary_iunknown!(
    dm_qi,
    dm_add_ref,
    dm_release,
    DocumentMgrObj,
    dm_qi_common,
    dm_add_ref_common,
    dm_release_common
);
impl_sub_iunknown!(
    dm_from_comp_mgr,
    dm_cm_qi,
    dm_cm_add_ref,
    dm_cm_release,
    DocumentMgrObj,
    vtbl_comp_mgr,
    dm_qi_common,
    dm_add_ref_common,
    dm_release_common
);
impl_sub_iunknown!(
    dm_from_source,
    dm_src_qi,
    dm_src_add_ref,
    dm_src_release,
    DocumentMgrObj,
    vtbl_source,
    dm_qi_common,
    dm_add_ref_common,
    dm_release_common
);

unsafe extern "win64" fn dm_create_context(
    this: *mut c_void,
    tid_owner: TfClientId,
    dw_flags: u32,
    punk: *mut c_void,
    ppic: *mut *mut c_void,
    pec_text_store: *mut TfEditCookie,
) -> HRESULT {
    write_null_out(ppic);
    if !pec_text_store.is_null() {
        std::ptr::write_unaligned(pec_text_store, TF_INVALID_EDIT_COOKIE);
    }
    if dw_flags != 0 || ppic.is_null() || pec_text_store.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const DocumentMgrObj);
    let cookie = {
        let mut st = obj.inner.lock().unwrap();
        let c = st.next_text_store_cookie;
        st.next_text_store_cookie = c.wrapping_add(1).max(1);
        c
    };
    if !punk.is_null() {
        com_call_add_ref(punk);
    }
    let ctx = new_context_obj(Arc::clone(&obj.inner), tid_owner, punk as usize, cookie);
    std::ptr::write_unaligned(ppic, ctx as *mut c_void);
    std::ptr::write_unaligned(pec_text_store, cookie);
    S_OK
}

unsafe extern "win64" fn dm_push(this: *mut c_void, pic: *mut c_void) -> HRESULT {
    if pic.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const DocumentMgrObj);
    let tm_sinks = {
        let mut st = obj.inner.lock().unwrap();
        if st.contexts.len() >= 16 {
            return TF_E_STACKFULL;
        }
        st.contexts.push(pic as usize);
        Arc::clone(&st.tm_sinks)
    };
    com_call_add_ref(pic);
    let sinks = snapshot_sinks_addref(&tm_sinks, &IID_ITFTHREADMGREVENTSINK);
    for sink in sinks {
        let _ = com_call_slot(sink, 6, pic as u64, 0, 0); // OnPushContext
        com_call_release(sink);
    }
    S_OK
}

unsafe extern "win64" fn dm_pop(this: *mut c_void, dw_flags: u32) -> HRESULT {
    if (dw_flags & !TF_POPF_ALL) != 0 {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const DocumentMgrObj);
    let (popped, tm_sinks) = {
        let mut st = obj.inner.lock().unwrap();
        let sinks = Arc::clone(&st.tm_sinks);
        if (dw_flags & TF_POPF_ALL) != 0 {
            let mut v: Vec<*mut c_void> = st.contexts.drain(..).map(|p| p as *mut c_void).collect();
            v.reverse(); // top-down notification order
            (v, sinks)
        } else {
            match st.contexts.pop() {
                Some(top) => (vec![top as *mut c_void], sinks),
                None => return E_FAIL,
            }
        }
    };
    for ctx in popped {
        let sinks = snapshot_sinks_addref(&tm_sinks, &IID_ITFTHREADMGREVENTSINK);
        for sink in sinks {
            let _ = com_call_slot(sink, 7, ctx as u64, 0, 0); // OnPopContext
            com_call_release(sink);
        }
        com_call_release(ctx);
    }
    S_OK
}

unsafe extern "win64" fn dm_get_top(this: *mut c_void, ppic: *mut *mut c_void) -> HRESULT {
    if ppic.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(ppic);
    let obj = &*(this as *const DocumentMgrObj);
    let top = obj.inner.lock().unwrap().contexts.last().copied();
    if let Some(p) = top {
        let ptr = p as *mut c_void;
        com_call_add_ref(ptr);
        std::ptr::write_unaligned(ppic, ptr);
    }
    S_OK
}

unsafe extern "win64" fn dm_get_base(this: *mut c_void, ppic: *mut *mut c_void) -> HRESULT {
    if ppic.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(ppic);
    let obj = &*(this as *const DocumentMgrObj);
    let base = obj.inner.lock().unwrap().contexts.first().copied();
    if let Some(p) = base {
        let ptr = p as *mut c_void;
        com_call_add_ref(ptr);
        std::ptr::write_unaligned(ppic, ptr);
    }
    S_OK
}

unsafe extern "win64" fn dm_enum_contexts(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let obj = &*(this as *const DocumentMgrObj);
    let mut items: Vec<usize> = obj.inner.lock().unwrap().contexts.clone();
    items.reverse(); // Windows enumerates contexts from top to base
    for &p in &items {
        com_call_add_ref(p as *mut c_void);
    }
    let en = new_enum_tf_com(IID_IENUMTFCONTEXTS, items, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe extern "win64" fn dm_cm_get(
    this: *mut c_void,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    let obj = &*dm_from_comp_mgr(this);
    comp_mgr_get_compartment_impl(&obj.compartments, rguid, ppcomp)
}

unsafe extern "win64" fn dm_cm_clear(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let obj = &*dm_from_comp_mgr(this);
    comp_mgr_clear_compartment_impl(&obj.compartments, tid, rguid)
}

unsafe extern "win64" fn dm_cm_enum(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    let obj = &*dm_from_comp_mgr(this);
    comp_mgr_enum_compartments_impl(&obj.compartments, pp_enum)
}

unsafe extern "win64" fn dm_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*dm_from_source(this);
    sink_advise(&obj.sinks, riid, punk, pdw_cookie)
}

unsafe extern "win64" fn dm_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*dm_from_source(this);
    sink_unadvise(&obj.sinks, cookie)
}

static ITF_DOCUMENT_MGR_VTBL: ITfDocumentMgrVtbl = ITfDocumentMgrVtbl {
    query_interface: dm_qi,
    add_ref: dm_add_ref,
    release: dm_release,
    create_context: dm_create_context,
    push: dm_push,
    pop: dm_pop,
    get_top: dm_get_top,
    get_base: dm_get_base,
    enum_contexts: dm_enum_contexts,
};

static ITF_DOCUMENT_MGR_CM_VTBL: ITfCompartmentMgrVtbl = ITfCompartmentMgrVtbl {
    query_interface: dm_cm_qi,
    add_ref: dm_cm_add_ref,
    release: dm_cm_release,
    get_compartment: dm_cm_get,
    clear_compartment: dm_cm_clear,
    enum_compartments: dm_cm_enum,
};

static ITF_DOCUMENT_MGR_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: dm_src_qi,
    add_ref: dm_src_add_ref,
    release: dm_src_release,
    advise_sink: dm_src_advise,
    unadvise_sink: dm_src_unadvise,
};

pub(super) fn new_document_mgr_obj(
    tm_state: Arc<Mutex<ThreadMgrState>>,
    tm_sinks: Arc<Mutex<SinkTable>>,
) -> *mut DocumentMgrObj {
    register_vtable_ptrs(&[
        dm_qi as *const () as usize,
        dm_add_ref as *const () as usize,
        dm_release as *const () as usize,
        dm_create_context as *const () as usize,
        dm_push as *const () as usize,
        dm_pop as *const () as usize,
        dm_get_top as *const () as usize,
        dm_get_base as *const () as usize,
        dm_enum_contexts as *const () as usize,
        dm_cm_qi as *const () as usize,
        dm_cm_add_ref as *const () as usize,
        dm_cm_release as *const () as usize,
        dm_cm_get as *const () as usize,
        dm_cm_clear as *const () as usize,
        dm_cm_enum as *const () as usize,
        dm_src_qi as *const () as usize,
        dm_src_add_ref as *const () as usize,
        dm_src_release as *const () as usize,
        dm_src_advise as *const () as usize,
        dm_src_unadvise as *const () as usize,
    ]);
    let inner = Arc::new(Mutex::new(DocMgrInner {
        live_self: 0,
        tm_state,
        tm_sinks,
        contexts: Vec::new(),
        next_text_store_cookie: 1,
    }));
    let ptr = Box::into_raw(Box::new(DocumentMgrObj {
        vtbl_dm: &ITF_DOCUMENT_MGR_VTBL,
        vtbl_comp_mgr: &ITF_DOCUMENT_MGR_CM_VTBL,
        vtbl_source: &ITF_DOCUMENT_MGR_SRC_VTBL,
        refs: AtomicU32::new(1),
        inner: Arc::clone(&inner),
        compartments: Mutex::new(CompartmentMap::default()),
        sinks: Mutex::new(SinkTable::default()),
    }));
    inner.lock().unwrap().live_self = ptr as usize;
    ptr
}
