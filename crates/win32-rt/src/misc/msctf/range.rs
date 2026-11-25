//! `ITfRange` (25 slots) and `ITfProperty` / `ITfReadOnlyProperty` (11 / 7 slots).

use super::abi::*;
use super::context::{try_add_ref_context, ContextInner};
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// `ITfRange` (25 slots)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfRangeVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut RangeObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut RangeObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut RangeObj) -> u32,
    pub(super) get_text:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut u16, u32, *mut u32) -> HRESULT,
    pub(super) set_text:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *const u16, i32) -> HRESULT,
    pub(super) get_formatted_text:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut *mut c_void) -> HRESULT,
    pub(super) get_embedded: unsafe extern "win64" fn(
        *mut RangeObj,
        TfEditCookie,
        *const u8,
        *const u8,
        *mut *mut c_void,
    ) -> HRESULT,
    pub(super) insert_embedded:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut c_void) -> HRESULT,
    pub(super) shift_start:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut i32, *const c_void) -> HRESULT,
    pub(super) shift_end:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut i32, *const c_void) -> HRESULT,
    pub(super) shift_start_to_range:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32) -> HRESULT,
    pub(super) shift_end_to_range:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32) -> HRESULT,
    pub(super) shift_start_region:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut BOOL) -> HRESULT,
    pub(super) shift_end_region:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut BOOL) -> HRESULT,
    pub(super) is_empty: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut BOOL) -> HRESULT,
    pub(super) collapse: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32) -> HRESULT,
    pub(super) is_equal_start:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut BOOL) -> HRESULT,
    pub(super) is_equal_end:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut BOOL) -> HRESULT,
    pub(super) compare_start:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut i32) -> HRESULT,
    pub(super) compare_end:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut i32) -> HRESULT,
    pub(super) adjust_for_insert:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut BOOL) -> HRESULT,
    pub(super) get_gravity: unsafe extern "win64" fn(*mut RangeObj, *mut i32, *mut i32) -> HRESULT,
    pub(super) set_gravity:
        unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, i32) -> HRESULT,
    pub(super) clone: unsafe extern "win64" fn(*mut RangeObj, *mut *mut c_void) -> HRESULT,
    pub(super) get_context: unsafe extern "win64" fn(*mut RangeObj, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfRangeVtbl>() == 25 * std::mem::size_of::<usize>());

pub(super) struct RangeInner {
    ctx_state: Arc<Mutex<ContextInner>>,
    gravity: Mutex<(i32, i32)>,
}

pub(super) type RangeObj = ComObj<RangeInner>;

impl_single_qi!(range_qi, RangeObj, IID_ITFRANGE);

unsafe extern "win64" fn range_get_text(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _dw_flags: u32,
    _pch_text: *mut u16,
    _cch_max: u32,
    pcch: *mut u32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pcch.is_null() {
        std::ptr::write_unaligned(pcch, 0);
    }
    S_OK
}

unsafe extern "win64" fn range_set_text(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _dw_flags: u32,
    _pch_text: *const u16,
    _cch: i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    S_OK
}

unsafe extern "win64" fn range_get_formatted_text(
    _this: *mut RangeObj,
    _ec: TfEditCookie,
    pp_data: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_data);
    E_NOTIMPL
}

unsafe extern "win64" fn range_get_embedded(
    _this: *mut RangeObj,
    _ec: TfEditCookie,
    _rguid_service: *const u8,
    _riid: *const u8,
    ppunk: *mut *mut c_void,
) -> HRESULT {
    write_null_out(ppunk);
    E_NOTIMPL
}

unsafe extern "win64" fn range_insert_embedded(
    _this: *mut RangeObj,
    _ec: TfEditCookie,
    _dw_flags: u32,
    _p_data: *mut c_void,
) -> HRESULT {
    E_NOTIMPL
}

unsafe extern "win64" fn range_shift_start(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _cch_req: i32,
    pcch: *mut i32,
    _p_halt: *const c_void,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pcch.is_null() {
        std::ptr::write_unaligned(pcch, 0);
    }
    S_OK
}

unsafe extern "win64" fn range_shift_end(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _cch_req: i32,
    pcch: *mut i32,
    _p_halt: *const c_void,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pcch.is_null() {
        std::ptr::write_unaligned(pcch, 0);
    }
    S_OK
}

unsafe extern "win64" fn range_shift_start_to_range(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_range: *mut c_void,
    _a_pos: i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_range.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

unsafe extern "win64" fn range_shift_end_to_range(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_range: *mut c_void,
    _a_pos: i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_range.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

unsafe extern "win64" fn range_shift_start_region(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _dir: i32,
    pf_no_region: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pf_no_region.is_null() {
        std::ptr::write_unaligned(pf_no_region, TRUE);
    }
    S_OK
}

unsafe extern "win64" fn range_shift_end_region(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _dir: i32,
    pf_no_region: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pf_no_region.is_null() {
        std::ptr::write_unaligned(pf_no_region, TRUE);
    }
    S_OK
}

unsafe extern "win64" fn range_is_empty(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    pf_empty: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if pf_empty.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_empty, TRUE);
    S_OK
}

unsafe extern "win64" fn range_collapse(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _a_pos: i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    S_OK
}

unsafe extern "win64" fn range_is_equal_start(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_with: *mut c_void,
    _a_pos: i32,
    pf_equal: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_with.is_null() || pf_equal.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_equal, TRUE);
    S_OK
}

unsafe extern "win64" fn range_is_equal_end(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_with: *mut c_void,
    _a_pos: i32,
    pf_equal: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_with.is_null() || pf_equal.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pf_equal, TRUE);
    S_OK
}

unsafe extern "win64" fn range_compare_start(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_with: *mut c_void,
    _a_pos: i32,
    pl_result: *mut i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_with.is_null() || pl_result.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pl_result, 0);
    S_OK
}

unsafe extern "win64" fn range_compare_end(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    p_with: *mut c_void,
    _a_pos: i32,
    pl_result: *mut i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if p_with.is_null() || pl_result.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pl_result, 0);
    S_OK
}

unsafe extern "win64" fn range_adjust_for_insert(
    _this: *mut RangeObj,
    ec: TfEditCookie,
    _cch_insert: u32,
    pf_insert_ok: *mut BOOL,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if !pf_insert_ok.is_null() {
        std::ptr::write_unaligned(pf_insert_ok, TRUE);
    }
    S_OK
}

unsafe extern "win64" fn range_get_gravity(
    this: *mut RangeObj,
    pg_start: *mut i32,
    pg_end: *mut i32,
) -> HRESULT {
    let (gs, ge) = *ComObj::inner(this).gravity.lock().unwrap();
    if !pg_start.is_null() {
        std::ptr::write_unaligned(pg_start, gs);
    }
    if !pg_end.is_null() {
        std::ptr::write_unaligned(pg_end, ge);
    }
    S_OK
}

unsafe extern "win64" fn range_set_gravity(
    this: *mut RangeObj,
    ec: TfEditCookie,
    g_start: i32,
    g_end: i32,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    *ComObj::inner(this).gravity.lock().unwrap() = (g_start, g_end);
    S_OK
}

unsafe extern "win64" fn range_clone(this: *mut RangeObj, pp_clone: *mut *mut c_void) -> HRESULT {
    if pp_clone.is_null() {
        return E_INVALIDARG;
    }
    let inner = ComObj::inner(this);
    let g = *inner.gravity.lock().unwrap();
    let cloned = new_range_obj(Arc::clone(&inner.ctx_state), g);
    std::ptr::write_unaligned(pp_clone, cloned as *mut c_void);
    S_OK
}

unsafe extern "win64" fn range_get_context(
    this: *mut RangeObj,
    pp_context: *mut *mut c_void,
) -> HRESULT {
    if pp_context.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_context);
    let maybe_ctx = {
        let st = ComObj::inner(this).ctx_state.lock().unwrap();
        try_add_ref_context(st.live_self)
    };
    match maybe_ctx {
        Some(ptr) => {
            std::ptr::write_unaligned(pp_context, ptr as *mut c_void);
            S_OK
        }
        None => TF_E_DISCONNECTED,
    }
}

static ITF_RANGE_VTBL: ITfRangeVtbl = ITfRangeVtbl {
    query_interface: range_qi,
    add_ref: com_add_ref::<RangeInner>,
    release: com_release::<RangeInner>,
    get_text: range_get_text,
    set_text: range_set_text,
    get_formatted_text: range_get_formatted_text,
    get_embedded: range_get_embedded,
    insert_embedded: range_insert_embedded,
    shift_start: range_shift_start,
    shift_end: range_shift_end,
    shift_start_to_range: range_shift_start_to_range,
    shift_end_to_range: range_shift_end_to_range,
    shift_start_region: range_shift_start_region,
    shift_end_region: range_shift_end_region,
    is_empty: range_is_empty,
    collapse: range_collapse,
    is_equal_start: range_is_equal_start,
    is_equal_end: range_is_equal_end,
    compare_start: range_compare_start,
    compare_end: range_compare_end,
    adjust_for_insert: range_adjust_for_insert,
    get_gravity: range_get_gravity,
    set_gravity: range_set_gravity,
    clone: range_clone,
    get_context: range_get_context,
};

pub(super) fn new_range_obj(
    ctx_state: Arc<Mutex<ContextInner>>,
    gravity: (i32, i32),
) -> *mut RangeObj {
    register_vtable_ptrs(&[
        range_qi as *const () as usize,
        com_add_ref::<RangeInner> as *const () as usize,
        com_release::<RangeInner> as *const () as usize,
        range_get_text as *const () as usize,
        range_set_text as *const () as usize,
        range_get_formatted_text as *const () as usize,
        range_get_embedded as *const () as usize,
        range_insert_embedded as *const () as usize,
        range_shift_start as *const () as usize,
        range_shift_end as *const () as usize,
        range_shift_start_to_range as *const () as usize,
        range_shift_end_to_range as *const () as usize,
        range_shift_start_region as *const () as usize,
        range_shift_end_region as *const () as usize,
        range_is_empty as *const () as usize,
        range_collapse as *const () as usize,
        range_is_equal_start as *const () as usize,
        range_is_equal_end as *const () as usize,
        range_compare_start as *const () as usize,
        range_compare_end as *const () as usize,
        range_adjust_for_insert as *const () as usize,
        range_get_gravity as *const () as usize,
        range_set_gravity as *const () as usize,
        range_clone as *const () as usize,
        range_get_context as *const () as usize,
    ]);
    ComObj::new(
        &ITF_RANGE_VTBL as *const ITfRangeVtbl as *const usize,
        RangeInner { ctx_state, gravity: Mutex::new(gravity) },
    )
}

// ---------------------------------------------------------------------------
// `ITfProperty` / `ITfReadOnlyProperty` (11 / 7 slots)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfPropertyVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut PropertyObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut PropertyObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut PropertyObj) -> u32,
    // ITfReadOnlyProperty (slots 3..=6)
    pub(super) get_type: unsafe extern "win64" fn(*mut PropertyObj, *mut u8) -> HRESULT,
    pub(super) enum_ranges:
        unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut *mut c_void, *mut c_void) -> HRESULT,
    pub(super) get_value:
        unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *mut u8) -> HRESULT,
    pub(super) get_context: unsafe extern "win64" fn(*mut PropertyObj, *mut *mut c_void) -> HRESULT,
    // ITfProperty (slots 7..=10)
    pub(super) find_range: unsafe extern "win64" fn(
        *mut PropertyObj,
        TfEditCookie,
        *mut c_void,
        *mut *mut c_void,
        i32,
    ) -> HRESULT,
    pub(super) set_value_store:
        unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *mut c_void) -> HRESULT,
    pub(super) set_value:
        unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *const u8) -> HRESULT,
    pub(super) clear: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfPropertyVtbl>() == 11 * std::mem::size_of::<usize>());

pub(super) struct PropertyData {
    pub(super) guid_type: Guid,
    pub(super) value: Mutex<OwnedVariant>,
}

pub(super) struct PropertyInner {
    read_only: bool,
    data: Arc<PropertyData>,
    ctx_state: Arc<Mutex<ContextInner>>,
}

pub(super) type PropertyObj = ComObj<PropertyInner>;

unsafe extern "win64" fn prop_qi(
    this: *mut PropertyObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() {
        write_null_out(ppv);
        return E_INVALIDARG;
    }
    if ComObj::inner(this).read_only {
        qi_helper_bytes(this, riid, ppv, &[IID_ITFREADONLYPROPERTY])
    } else {
        qi_helper_bytes(this, riid, ppv, &[IID_ITFREADONLYPROPERTY, IID_ITFPROPERTY])
    }
}

unsafe extern "win64" fn prop_get_type(this: *mut PropertyObj, pguid: *mut u8) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid as *mut Guid, ComObj::inner(this).data.guid_type);
    S_OK
}

unsafe extern "win64" fn prop_enum_ranges(
    _this: *mut PropertyObj,
    ec: TfEditCookie,
    pp_enum: *mut *mut c_void,
    _p_target: *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let en = new_enum_tf_com(IID_IENUMTFRANGES, Vec::new(), 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe extern "win64" fn prop_get_value(
    this: *mut PropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
    pvar_value: *mut u8,
) -> HRESULT {
    if pvar_value.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_bytes(pvar_value, 0, 24);
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let guard = ComObj::inner(this).data.value.lock().unwrap();
    if guard.is_empty() {
        return S_FALSE;
    }
    guard.write_copy_to(pvar_value);
    S_OK
}

unsafe extern "win64" fn prop_get_context(
    this: *mut PropertyObj,
    pp_context: *mut *mut c_void,
) -> HRESULT {
    if pp_context.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_context);
    let maybe_ctx = {
        let st = ComObj::inner(this).ctx_state.lock().unwrap();
        try_add_ref_context(st.live_self)
    };
    match maybe_ctx {
        Some(ptr) => {
            std::ptr::write_unaligned(pp_context, ptr as *mut c_void);
            S_OK
        }
        None => TF_E_DISCONNECTED,
    }
}

unsafe extern "win64" fn prop_find_range(
    this: *mut PropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
    pp_range: *mut *mut c_void,
    _a_pos: i32,
) -> HRESULT {
    if pp_range.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_range);
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let r = new_range_obj(Arc::clone(&ComObj::inner(this).ctx_state), (0, 0));
    std::ptr::write_unaligned(pp_range, r as *mut c_void);
    S_OK
}

unsafe extern "win64" fn prop_set_value_store(
    _this: *mut PropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
    _p_prop_store: *mut c_void,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    S_OK
}

unsafe extern "win64" fn prop_set_value(
    this: *mut PropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
    pvar_value: *const u8,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    if pvar_value.is_null() {
        return E_INVALIDARG;
    }
    let new_val = OwnedVariant::from_raw(pvar_value);
    let old_raw = {
        let mut guard = ComObj::inner(this).data.value.lock().unwrap();
        let old = guard.take_raw();
        *guard = new_val;
        old
    };
    release_variant_raw(old_raw);
    S_OK
}

unsafe extern "win64" fn prop_clear(
    this: *mut PropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
) -> HRESULT {
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let old_raw = ComObj::inner(this).data.value.lock().unwrap().take_raw();
    release_variant_raw(old_raw);
    S_OK
}

static ITF_PROPERTY_VTBL: ITfPropertyVtbl = ITfPropertyVtbl {
    query_interface: prop_qi,
    add_ref: com_add_ref::<PropertyInner>,
    release: com_release::<PropertyInner>,
    get_type: prop_get_type,
    enum_ranges: prop_enum_ranges,
    get_value: prop_get_value,
    get_context: prop_get_context,
    find_range: prop_find_range,
    set_value_store: prop_set_value_store,
    set_value: prop_set_value,
    clear: prop_clear,
};

pub(super) fn new_property_obj(
    data: Arc<PropertyData>,
    ctx_state: Arc<Mutex<ContextInner>>,
    read_only: bool,
) -> *mut PropertyObj {
    register_vtable_ptrs(&[
        prop_qi as *const () as usize,
        com_add_ref::<PropertyInner> as *const () as usize,
        com_release::<PropertyInner> as *const () as usize,
        prop_get_type as *const () as usize,
        prop_enum_ranges as *const () as usize,
        prop_get_value as *const () as usize,
        prop_get_context as *const () as usize,
        prop_find_range as *const () as usize,
        prop_set_value_store as *const () as usize,
        prop_set_value as *const () as usize,
        prop_clear as *const () as usize,
    ]);
    ComObj::new(
        &ITF_PROPERTY_VTBL as *const ITfPropertyVtbl as *const usize,
        PropertyInner { read_only, data, ctx_state },
    )
}

// ---------------------------------------------------------------------------
// `IEnumTfPropertyValue` (7 slots) & tracked composite `ITfReadOnlyProperty`
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct IEnumTfPropertyValueVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut EnumTfPropValObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut EnumTfPropValObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut EnumTfPropValObj) -> u32,
    pub(super) clone: unsafe extern "win64" fn(*mut EnumTfPropValObj, *mut *mut c_void) -> HRESULT,
    pub(super) next:
        unsafe extern "win64" fn(*mut EnumTfPropValObj, u32, *mut TF_PROPERTYVAL, *mut u32) -> HRESULT,
    pub(super) reset: unsafe extern "win64" fn(*mut EnumTfPropValObj) -> HRESULT,
    pub(super) skip: unsafe extern "win64" fn(*mut EnumTfPropValObj, u32) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<IEnumTfPropertyValueVtbl>() == 7 * std::mem::size_of::<usize>());

pub(super) struct EnumTfPropValInner {
    items: Vec<(Guid, OwnedVariant)>,
    pos: Mutex<usize>,
}

pub(super) type EnumTfPropValObj = ComObj<EnumTfPropValInner>;

impl_single_qi!(enum_pv_qi, EnumTfPropValObj, IID_IENUMTFPROPERTYVALUE);

unsafe extern "win64" fn enum_pv_clone(
    this: *mut EnumTfPropValObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let inner = ComObj::inner(this);
    let pos = *inner.pos.lock().unwrap();
    let cloned_items: Vec<(Guid, OwnedVariant)> = inner
        .items
        .iter()
        .map(|(g, v)| {
            let mut raw = [0u8; 24];
            v.write_copy_to(raw.as_mut_ptr());
            (*g, OwnedVariant::from_raw(raw.as_ptr()))
        })
        .collect();
    let cloned = new_enum_tf_prop_val(cloned_items, pos);
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

unsafe extern "win64" fn enum_pv_next(
    this: *mut EnumTfPropValObj,
    ul_count: u32,
    rg_values: *mut TF_PROPERTYVAL,
    pc_fetched: *mut u32,
) -> HRESULT {
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, 0);
    }
    if ul_count > 1 && pc_fetched.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && rg_values.is_null() {
        return E_INVALIDARG;
    }
    let inner = ComObj::inner(this);
    let (start, n) = {
        let mut pos = inner.pos.lock().unwrap();
        let avail = inner.items.len().saturating_sub(*pos);
        let n = (ul_count as usize).min(avail);
        let start = *pos;
        *pos += n;
        (start, n)
    };
    for i in 0..n {
        let (guid, ref var) = inner.items[start + i];
        let mut out_val = TF_PROPERTYVAL { guid_id: guid, var_value: [0u8; 24] };
        var.write_copy_to(out_val.var_value.as_mut_ptr());
        std::ptr::write_unaligned(rg_values.add(i), out_val);
    }
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, n as u32);
    }
    if n == ul_count as usize { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn enum_pv_reset(this: *mut EnumTfPropValObj) -> HRESULT {
    *ComObj::inner(this).pos.lock().unwrap() = 0;
    S_OK
}

unsafe extern "win64" fn enum_pv_skip(this: *mut EnumTfPropValObj, ul_count: u32) -> HRESULT {
    let inner = ComObj::inner(this);
    let mut pos = inner.pos.lock().unwrap();
    let avail = inner.items.len().saturating_sub(*pos);
    let step = (ul_count as usize).min(avail);
    *pos += step;
    if step == ul_count as usize { S_OK } else { S_FALSE }
}

static IENUM_TF_PROP_VAL_VTBL: IEnumTfPropertyValueVtbl = IEnumTfPropertyValueVtbl {
    query_interface: enum_pv_qi,
    add_ref: com_add_ref::<EnumTfPropValInner>,
    release: com_release::<EnumTfPropValInner>,
    clone: enum_pv_clone,
    next: enum_pv_next,
    reset: enum_pv_reset,
    skip: enum_pv_skip,
};

fn new_enum_tf_prop_val(items: Vec<(Guid, OwnedVariant)>, pos: usize) -> *mut EnumTfPropValObj {
    register_vtable_ptrs(&[
        enum_pv_qi as *const () as usize,
        com_add_ref::<EnumTfPropValInner> as *const () as usize,
        com_release::<EnumTfPropValInner> as *const () as usize,
        enum_pv_clone as *const () as usize,
        enum_pv_next as *const () as usize,
        enum_pv_reset as *const () as usize,
        enum_pv_skip as *const () as usize,
    ]);
    ComObj::new(
        &IENUM_TF_PROP_VAL_VTBL as *const IEnumTfPropertyValueVtbl as *const usize,
        EnumTfPropValInner { items, pos: Mutex::new(pos) },
    )
}

#[repr(C)]
pub(super) struct ITfReadOnlyPropertyVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut TrackedPropertyObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut TrackedPropertyObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut TrackedPropertyObj) -> u32,
    pub(super) get_type: unsafe extern "win64" fn(*mut TrackedPropertyObj, *mut u8) -> HRESULT,
    pub(super) enum_ranges: unsafe extern "win64" fn(
        *mut TrackedPropertyObj,
        TfEditCookie,
        *mut *mut c_void,
        *mut c_void,
    ) -> HRESULT,
    pub(super) get_value: unsafe extern "win64" fn(
        *mut TrackedPropertyObj,
        TfEditCookie,
        *mut c_void,
        *mut u8,
    ) -> HRESULT,
    pub(super) get_context:
        unsafe extern "win64" fn(*mut TrackedPropertyObj, *mut *mut c_void) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfReadOnlyPropertyVtbl>() == 7 * std::mem::size_of::<usize>());

pub(super) struct TrackedPropertyInner {
    props: Vec<Arc<PropertyData>>,
    ctx_state: Arc<Mutex<ContextInner>>,
}

pub(super) type TrackedPropertyObj = ComObj<TrackedPropertyInner>;

impl_single_qi!(tprop_qi, TrackedPropertyObj, IID_ITFREADONLYPROPERTY);

unsafe extern "win64" fn tprop_get_type(
    _this: *mut TrackedPropertyObj,
    pguid: *mut u8,
) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid as *mut Guid, GUID_NULL);
    S_OK
}

unsafe extern "win64" fn tprop_enum_ranges(
    this: *mut TrackedPropertyObj,
    ec: TfEditCookie,
    pp_enum: *mut *mut c_void,
    p_target: *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let inner = ComObj::inner(this);
    let any_set = inner.props.iter().any(|p| !p.value.lock().unwrap().is_empty());
    let items = if any_set {
        let r = if !p_target.is_null() {
            let mut cloned: *mut c_void = std::ptr::null_mut();
            if com_call_slot(p_target, 23, &mut cloned as *mut *mut c_void as u64, 0, 0) == S_OK
                && !cloned.is_null()
            {
                cloned as usize
            } else {
                new_range_obj(Arc::clone(&inner.ctx_state), (0, 0)) as usize
            }
        } else {
            new_range_obj(Arc::clone(&inner.ctx_state), (0, 0)) as usize
        };
        vec![r]
    } else {
        Vec::new()
    };
    let en = new_enum_tf_com(IID_IENUMTFRANGES, items, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}

unsafe extern "win64" fn tprop_get_value(
    this: *mut TrackedPropertyObj,
    ec: TfEditCookie,
    _p_range: *mut c_void,
    pvar_value: *mut u8,
) -> HRESULT {
    if pvar_value.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_bytes(pvar_value, 0, 24);
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let inner = ComObj::inner(this);
    let mut items = Vec::with_capacity(inner.props.len());
    for prop in &inner.props {
        let guard = prop.value.lock().unwrap();
        let mut raw = [0u8; 24];
        guard.write_copy_to(raw.as_mut_ptr());
        items.push((prop.guid_type, OwnedVariant::from_raw(raw.as_ptr())));
    }
    let enumerator = new_enum_tf_prop_val(items, 0);
    std::ptr::write_unaligned(pvar_value as *mut u16, VT_UNKNOWN);
    std::ptr::write_unaligned(pvar_value.add(8) as *mut usize, enumerator as usize);
    S_OK
}

unsafe extern "win64" fn tprop_get_context(
    this: *mut TrackedPropertyObj,
    pp_context: *mut *mut c_void,
) -> HRESULT {
    if pp_context.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_context);
    let maybe_ctx = {
        let st = ComObj::inner(this).ctx_state.lock().unwrap();
        try_add_ref_context(st.live_self)
    };
    match maybe_ctx {
        Some(ptr) => {
            std::ptr::write_unaligned(pp_context, ptr as *mut c_void);
            S_OK
        }
        None => TF_E_DISCONNECTED,
    }
}

static ITF_TRACKED_PROPERTY_VTBL: ITfReadOnlyPropertyVtbl = ITfReadOnlyPropertyVtbl {
    query_interface: tprop_qi,
    add_ref: com_add_ref::<TrackedPropertyInner>,
    release: com_release::<TrackedPropertyInner>,
    get_type: tprop_get_type,
    enum_ranges: tprop_enum_ranges,
    get_value: tprop_get_value,
    get_context: tprop_get_context,
};

pub(super) fn new_tracked_property_obj(
    props: Vec<Arc<PropertyData>>,
    ctx_state: Arc<Mutex<ContextInner>>,
) -> *mut TrackedPropertyObj {
    register_vtable_ptrs(&[
        tprop_qi as *const () as usize,
        com_add_ref::<TrackedPropertyInner> as *const () as usize,
        com_release::<TrackedPropertyInner> as *const () as usize,
        tprop_get_type as *const () as usize,
        tprop_enum_ranges as *const () as usize,
        tprop_get_value as *const () as usize,
        tprop_get_context as *const () as usize,
    ]);
    ComObj::new(
        &ITF_TRACKED_PROPERTY_VTBL as *const ITfReadOnlyPropertyVtbl as *const usize,
        TrackedPropertyInner { props, ctx_state },
    )
}
