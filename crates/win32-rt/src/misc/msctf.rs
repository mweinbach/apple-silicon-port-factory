//! msctf.dll: Windows Text Services Framework (TSF).
//!
//! Provides ref-counted Win64 COM implementations of:
//! - `CLSID_TF_ThreadMgr`: `ITfThreadMgr` (14 slots), `ITfThreadMgrEx` (16 slots),
//!   `ITfThreadMgr2` (17 slots), `ITfSource`, `ITfSourceSingle`, `ITfUIElementMgr`,
//!   `ITfCompartmentMgr`, `ITfKeystrokeMgr`, with per-thread synchronization.
//! - `ITfDocumentMgr` (9 slots) + `ITfCompartmentMgr` + `ITfSource`, and
//!   `ITfContext` (18 slots) + `ITfSource` + `ITfSourceSingle` + `ITfCompartmentMgr`,
//!   `ITfProperty` / `ITfReadOnlyProperty`, `ITfRange`.
//! - `CLSID_TF_InputProcessorProfiles`: `ITfInputProcessorProfiles` (21 slots),
//!   `ITfInputProcessorProfileMgr` (11 slots), `ITfSource` (reporting active `en-US`
//!   keyboard profile).
//! - `CLSID_TF_CategoryMgr`: `ITfCategoryMgr` (17 slots) with GUID-atom table,
//!   category sets, GUID descriptions, and GUID DWORDs.
//! - `CLSID_TF_DisplayAttributeMgr`: `ITfDisplayAttributeMgr` (6 slots),
//!   `ITfDisplayAttributeInfo` (8 slots), `IEnumTfDisplayAttributeInfo`.
//! - `CLSID_TF_LangBarMgr`: `ITfLangBarMgr` (12 slots).
//! - Standard `msctf.dll` entry points: `TF_CreateThreadMgr`, `TF_GetThreadMgr`,
//!   `TF_CreateInputProcessorProfiles`, `TF_CreateDisplayAttributeMgr`,
//!   `TF_CreateCategoryMgr`, `TF_CreateLangBarMgr`,
//!   `TF_InvalidAssemblyListCacheIfExist`, `SetInputScope`, `SetInputScopes`.

mod abi;
mod context;
mod profiles;
mod range;
mod thread_mgr;

use abi::{
    read_guid_from_bytes, write_null_out, CLASS_E_NOAGGREGATION, CLSID_TF_CATEGORYMGR,
    CLSID_TF_DISPLAYATTRIBUTEMGR, CLSID_TF_INPUTPROCESSORPROFILES, CLSID_TF_LANGBARMGR,
    CLSID_TF_THREADMGR, IID_ITFCATEGORYMGR, IID_ITFDISPLAYATTRIBUTEMGR, IID_ITFLANGBARMGR,
    IID_IUNKNOWN,
};
use profiles::{
    ipp_qi_common, ipp_release_common, ipp_supports_iid, new_category_mgr_obj,
    new_display_attr_mgr_obj, new_input_processor_profiles_obj, new_lang_bar_mgr_obj,
};
use thread_mgr::{
    get_existing_thread_mgr, get_or_create_thread_mgr, tm_qi_common, tm_release_common,
    tm_supports_iid,
};
use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Per-HWND InputScope storage (`SetInputScope`, `SetInputScopes`)
// ---------------------------------------------------------------------------

static INPUT_SCOPES: LazyLock<Mutex<HashMap<usize, Vec<i32>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// ---------------------------------------------------------------------------
// COM class factory entry point (wired into ole32.rs)
// ---------------------------------------------------------------------------

/// `CoCreateInstance` / `CoGetClassObject` hook for TSF coclasses.
/// Returns `None` if `clsid` is not a TSF class.
pub unsafe fn create_instance(
    clsid: *const u8,
    iid: *const u8,
    outer: *mut c_void,
    out: *mut *mut c_void,
) -> Option<HRESULT> {
    let cls = read_guid_from_bytes(clsid)?;
    let is_tsf = cls == CLSID_TF_THREADMGR
        || cls == CLSID_TF_INPUTPROCESSORPROFILES
        || cls == CLSID_TF_CATEGORYMGR
        || cls == CLSID_TF_DISPLAYATTRIBUTEMGR
        || cls == CLSID_TF_LANGBARMGR;
    if !is_tsf {
        return None;
    }
    if out.is_null() {
        return Some(E_POINTER);
    }
    write_null_out(out);
    if !outer.is_null() {
        return Some(CLASS_E_NOAGGREGATION);
    }
    let Some(want) = read_guid_from_bytes(iid) else {
        return Some(E_INVALIDARG);
    };

    if cls == CLSID_TF_THREADMGR {
        if !tm_supports_iid(&want) {
            return Some(E_NOINTERFACE);
        }
        let base = get_or_create_thread_mgr();
        let hr = tm_qi_common(base, iid, out);
        tm_release_common(base);
        return Some(hr);
    }

    if cls == CLSID_TF_INPUTPROCESSORPROFILES {
        if !ipp_supports_iid(&want) {
            return Some(E_NOINTERFACE);
        }
        let base = new_input_processor_profiles_obj();
        let hr = ipp_qi_common(base, iid, out);
        ipp_release_common(base);
        return Some(hr);
    }

    if cls == CLSID_TF_CATEGORYMGR {
        if want != IID_IUNKNOWN && want != IID_ITFCATEGORYMGR {
            return Some(E_NOINTERFACE);
        }
        let obj = new_category_mgr_obj();
        std::ptr::write_unaligned(out, obj as *mut c_void);
        return Some(S_OK);
    }

    if cls == CLSID_TF_DISPLAYATTRIBUTEMGR {
        if want != IID_IUNKNOWN && want != IID_ITFDISPLAYATTRIBUTEMGR {
            return Some(E_NOINTERFACE);
        }
        let obj = new_display_attr_mgr_obj();
        std::ptr::write_unaligned(out, obj as *mut c_void);
        return Some(S_OK);
    }

    if cls == CLSID_TF_LANGBARMGR {
        if want != IID_IUNKNOWN && want != IID_ITFLANGBARMGR {
            return Some(E_NOINTERFACE);
        }
        let obj = new_lang_bar_mgr_obj();
        std::ptr::write_unaligned(out, obj as *mut c_void);
        return Some(S_OK);
    }

    None
}

// ---------------------------------------------------------------------------
// Exported `msctf.dll` Win64 entry points
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn TF_CreateThreadMgr(pptim: *mut *mut c_void) -> HRESULT {
    if pptim.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pptim);
    let tm = get_or_create_thread_mgr();
    std::ptr::write_unaligned(pptim, tm as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_GetThreadMgr(pptim: *mut *mut c_void) -> HRESULT {
    if pptim.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pptim);
    if let Some(tm) = get_existing_thread_mgr() {
        std::ptr::write_unaligned(pptim, tm as *mut c_void);
    }
    S_OK
}

pub unsafe extern "win64" fn TF_CreateInputProcessorProfiles(ppipr: *mut *mut c_void) -> HRESULT {
    if ppipr.is_null() {
        return E_INVALIDARG;
    }
    let obj = new_input_processor_profiles_obj();
    std::ptr::write_unaligned(ppipr, obj as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_CreateDisplayAttributeMgr(ppdam: *mut *mut c_void) -> HRESULT {
    if ppdam.is_null() {
        return E_INVALIDARG;
    }
    let obj = new_display_attr_mgr_obj();
    std::ptr::write_unaligned(ppdam, obj as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_CreateCategoryMgr(ppcat: *mut *mut c_void) -> HRESULT {
    if ppcat.is_null() {
        return E_INVALIDARG;
    }
    let obj = new_category_mgr_obj();
    std::ptr::write_unaligned(ppcat, obj as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_CreateLangBarMgr(pppbm: *mut *mut c_void) -> HRESULT {
    if pppbm.is_null() {
        return E_INVALIDARG;
    }
    let obj = new_lang_bar_mgr_obj();
    std::ptr::write_unaligned(pppbm, obj as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_InvalidAssemblyListCacheIfExist() -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn SetInputScope(hwnd: HWND, inputscope: i32) -> HRESULT {
    if hwnd.is_null() {
        return E_INVALIDARG;
    }
    let mut map = INPUT_SCOPES.lock().unwrap();
    if inputscope == 0 {
        map.remove(&(hwnd as usize));
    } else {
        map.insert(hwnd as usize, vec![inputscope]);
    }
    S_OK
}

pub unsafe extern "win64" fn SetInputScopes(
    hwnd: HWND,
    p_input_scopes: *const i32,
    c_input_scopes: u32,
    _ppsz_phrase_list: *const LPCWSTR,
    _c_phrases: u32,
    _psz_reg_exp: LPCWSTR,
    _psz_srgs: LPCWSTR,
) -> HRESULT {
    if hwnd.is_null() {
        return E_INVALIDARG;
    }
    if c_input_scopes > 0 && p_input_scopes.is_null() {
        return E_INVALIDARG;
    }
    let mut scopes = Vec::with_capacity(c_input_scopes as usize);
    for i in 0..c_input_scopes as usize {
        scopes.push(std::ptr::read_unaligned(p_input_scopes.add(i)));
    }
    let mut map = INPUT_SCOPES.lock().unwrap();
    if scopes.is_empty() {
        map.remove(&(hwnd as usize));
    } else {
        map.insert(hwnd as usize, scopes);
    }
    S_OK
}

#[cfg(test)]
mod tests;
