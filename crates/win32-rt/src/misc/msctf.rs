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

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Types, constants, HRESULTs, and GUIDs (Windows SDK msctf.h / ctfutb.h)
// ---------------------------------------------------------------------------

type Guid = [u8; 16];
type TfClientId = u32;
type TfEditCookie = u32;
type TfGuidAtom = u32;
type HKL = usize;

/// 4-byte-aligned GUID matching C `struct GUID` alignment on Win64.
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct Align4Guid([u8; 16]);

const _: () = assert!(std::mem::size_of::<Align4Guid>() == 16);
const _: () = assert!(std::mem::align_of::<Align4Guid>() == 4);

const fn guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Guid {
    let a = d1.to_le_bytes();
    let b = d2.to_le_bytes();
    let c = d3.to_le_bytes();
    [
        a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d4[0], d4[1], d4[2], d4[3], d4[4], d4[5],
        d4[6], d4[7],
    ]
}

const GUID_NULL: Guid = [0u8; 16];

// Standard COM IIDs
const IID_IUNKNOWN: Guid = guid(0x00000000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
const IID_IENUMGUID: Guid = guid(0x0002E000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

// TSF CLSIDs
const CLSID_TF_THREADMGR: Guid =
    guid(0x529A9E6B, 0x6587, 0x4F23, [0xAB, 0x9E, 0x9C, 0x7D, 0x68, 0x3E, 0x3C, 0x50]);
const CLSID_TF_INPUTPROCESSORPROFILES: Guid =
    guid(0x33C53A50, 0xF456, 0x4884, [0xB0, 0x49, 0x85, 0xFD, 0x64, 0x3E, 0xCF, 0xED]);
const CLSID_TF_CATEGORYMGR: Guid =
    guid(0xA4B544A1, 0x438D, 0x4B41, [0x93, 0x25, 0x86, 0x95, 0x23, 0xE2, 0xD6, 0xC7]);
const CLSID_TF_DISPLAYATTRIBUTEMGR: Guid =
    guid(0x3CE74DE4, 0x53D3, 0x4D74, [0x8B, 0x83, 0x43, 0x1B, 0x38, 0x28, 0xBA, 0x53]);
const CLSID_TF_LANGBARMGR: Guid =
    guid(0xEBB08C45, 0x6C4A, 0x4FDC, [0xAE, 0x53, 0x4E, 0xB8, 0xC4, 0xC7, 0xDB, 0x8E]);

// TSF Category / Profile GUIDs
const GUID_TFCAT_TIP_KEYBOARD: Guid =
    guid(0x34745C63, 0xB2F0, 0x4784, [0x8B, 0x67, 0x5E, 0x12, 0xC8, 0x70, 0x1A, 0x31]);
/// Built-in default en-US keyboard profile identifiers.
const CLSID_EN_US_KEYBOARD_TIP: Guid =
    guid(0x0000897B, 0x83DF, 0x4B96, [0xBE, 0x07, 0x0F, 0xB5, 0x8B, 0x01, 0xC4, 0xA4]);
const GUID_EN_US_KEYBOARD_PROFILE: Guid =
    guid(0x09EA4E4B, 0x46CE, 0x4469, [0xB4, 0x50, 0x0D, 0x3A, 0x2F, 0x04, 0x09, 0x01]);

// Standard display attribute GUIDs
const GUID_DA_INPUT: Guid =
    guid(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x01]);
const GUID_DA_TARGET_CONVERTED: Guid =
    guid(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x02]);
const GUID_DA_CONVERTED: Guid =
    guid(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x03]);

// TSF Interface IIDs (exact values from msctf.h / ctfutb.h)
const IID_ITFTHREADMGR: Guid =
    guid(0xAA80E801, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_ITFTHREADMGREX: Guid =
    guid(0x3E90ADE3, 0x7594, 0x4CB0, [0xBB, 0x58, 0x69, 0x62, 0x8F, 0x5F, 0x45, 0x8C]);
const IID_ITFTHREADMGR2: Guid =
    guid(0x0AB198EF, 0x6477, 0x4EE8, [0x88, 0x12, 0x67, 0x80, 0xED, 0xB8, 0x2D, 0x5E]);
const IID_ITFSOURCE: Guid =
    guid(0x4EA48A35, 0x60AE, 0x446F, [0x8F, 0xD6, 0xE6, 0xA8, 0xD8, 0x24, 0x59, 0xF7]);
const IID_ITFSOURCESINGLE: Guid =
    guid(0x73131F9C, 0x56A9, 0x49DD, [0xB0, 0xEE, 0xD0, 0x46, 0x63, 0x3F, 0x75, 0x28]);
const IID_ITFUIELEMENTMGR: Guid =
    guid(0xEA1EA135, 0x19DF, 0x11D7, [0xA6, 0xD2, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
const IID_ITFUIELEMENTSINK: Guid =
    guid(0xEA1EA136, 0x19DF, 0x11D7, [0xA6, 0xD2, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
const IID_IENUMTFUIELEMENTS: Guid =
    guid(0x887AA91E, 0xACBA, 0x4931, [0x84, 0xDA, 0x3C, 0x52, 0x08, 0xCF, 0x54, 0x3F]);
const IID_ITFCOMPARTMENTMGR: Guid =
    guid(0x7DCF57AC, 0x18AD, 0x438B, [0x82, 0x4D, 0x97, 0x9B, 0xFF, 0xB7, 0x4B, 0x7C]);
const IID_ITFCOMPARTMENT: Guid =
    guid(0xBB08F7A9, 0x607A, 0x4384, [0x86, 0x23, 0x05, 0x68, 0x92, 0xB6, 0x43, 0x71]);
const IID_ITFCOMPARTMENTEVENTSINK: Guid =
    guid(0x743ABD5F, 0xF26D, 0x48DF, [0x8C, 0xC5, 0x23, 0x84, 0x92, 0x41, 0x9B, 0x64]);
const IID_ITFKEYSTROKEMGR: Guid =
    guid(0xAA80E7F0, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_ITFDOCUMENTMGR: Guid =
    guid(0xAA80E7F4, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_IENUMTFDOCUMENTMGRS: Guid =
    guid(0xAA80E808, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_ITFCONTEXT: Guid =
    guid(0xAA80E7FD, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_IENUMTFCONTEXTS: Guid =
    guid(0x8F1A7EA6, 0x1654, 0x4502, [0xA8, 0x6E, 0xB2, 0x90, 0x23, 0x44, 0xD5, 0x07]);
const IID_IENUMTFCONTEXTVIEWS: Guid =
    guid(0xF0C0F8DD, 0xCF38, 0x44E1, [0xBB, 0x0F, 0x68, 0xCF, 0x0D, 0x55, 0x1C, 0x78]);
const IID_ITFPROPERTY: Guid =
    guid(0xE2449660, 0x9542, 0x11D2, [0xBF, 0x46, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
const IID_ITFREADONLYPROPERTY: Guid =
    guid(0x17D49A3D, 0xF8B8, 0x4B2F, [0xB2, 0x54, 0x52, 0x31, 0x9D, 0xD6, 0x4C, 0x53]);
const IID_IENUMTFPROPERTIES: Guid =
    guid(0x19188CB0, 0xACA9, 0x11D2, [0xAF, 0xC5, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
const IID_ITFRANGE: Guid =
    guid(0xAA80E7FF, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_IENUMTFRANGES: Guid =
    guid(0xF99D3F40, 0x8E32, 0x11D2, [0xBF, 0x46, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
const IID_ITFTHREADMGREVENTSINK: Guid =
    guid(0xAA80E80E, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
const IID_IENUMTFFUNCTIONPROVIDERS: Guid =
    guid(0xE4B24DB0, 0x0990, 0x11D3, [0x8D, 0xF0, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
const IID_ITFINPUTPROCESSORPROFILES: Guid =
    guid(0x1F02B6C5, 0x7842, 0x4EE6, [0x8A, 0x0B, 0x9A, 0x24, 0x18, 0x3A, 0x95, 0xCA]);
const IID_ITFINPUTPROCESSORPROFILEMGR: Guid =
    guid(0x71C6E74C, 0x0F28, 0x11D8, [0xA8, 0x2A, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
const IID_IENUMTFLANGUAGEPROFILES: Guid =
    guid(0x3D61BF11, 0xAC5F, 0x42C8, [0xA4, 0xCB, 0x93, 0x1B, 0xCC, 0x28, 0xC7, 0x44]);
const IID_IENUMTFINPUTPROCESSORPROFILES: Guid =
    guid(0x71C6E74D, 0x0F28, 0x11D8, [0xA8, 0x2A, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
const IID_ITFCATEGORYMGR: Guid =
    guid(0xC3ACEFB5, 0xF69D, 0x4905, [0x93, 0x8F, 0xFC, 0xAD, 0xCF, 0x4B, 0xE8, 0x30]);
const IID_ITFDISPLAYATTRIBUTEMGR: Guid =
    guid(0x8DED7393, 0x5DB1, 0x475C, [0x9E, 0x71, 0xA3, 0x91, 0x11, 0xB0, 0xFF, 0x67]);
const IID_ITFDISPLAYATTRIBUTEINFO: Guid =
    guid(0x70528852, 0x2F26, 0x4AEA, [0x8C, 0x96, 0x21, 0x51, 0x50, 0x57, 0x89, 0x32]);
const IID_IENUMTFDISPLAYATTRIBUTEINFO: Guid =
    guid(0x7CEF04D7, 0xCB75, 0x4E80, [0xA7, 0xAB, 0x5F, 0x5B, 0xC7, 0xD3, 0x32, 0xDE]);
const IID_ITFLANGBARMGR: Guid =
    guid(0x87955690, 0xE627, 0x11D2, [0x8D, 0xDB, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);

// HRESULTs
const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110u32 as i32;
const E_UNEXPECTED: HRESULT = 0x8000_FFFFu32 as i32;
const CONNECT_E_NOCONNECTION: HRESULT = 0x8004_0200u32 as i32;
const CONNECT_E_ADVISELIMIT: HRESULT = 0x8004_0201u32 as i32;
const CONNECT_E_CANNOTCONNECT: HRESULT = 0x8004_0202u32 as i32;
const TF_E_STACKFULL: HRESULT = 0x8004_0501u32 as i32;
const TF_E_NOPROVIDER: HRESULT = 0x8004_0503u32 as i32;
const TF_E_DISCONNECTED: HRESULT = 0x8004_0504u32 as i32;
const TF_E_INVALIDVIEW: HRESULT = 0x8004_0505u32 as i32;
const TF_E_ALREADY_EXISTS: HRESULT = 0x8004_0506u32 as i32;
const TF_E_NOLOCK: HRESULT = 0x8004_0201u32 as i32;
const TF_E_NOSELECTION: HRESULT = 0x8004_0205u32 as i32;
const TF_E_SYNCHRONOUS: HRESULT = 0x8004_0208u32 as i32;

// TSF flags & constants
const TF_INVALID_COOKIE: u32 = 0xFFFF_FFFF;
const TF_INVALID_EDIT_COOKIE: TfEditCookie = 0;
const TF_INVALID_GUIDATOM: TfGuidAtom = 0;
const TF_TMF_ACTIVATED: u32 = 0x8000_0000;
const TF_POPF_ALL: u32 = 0x0000_0001;
const TF_ES_SYNC: u32 = 0x0000_0001;
const TF_ES_READ: u32 = 0x0000_0002;
const TF_ES_READWRITE: u32 = 0x0000_0006;
const TF_ES_ASYNC: u32 = 0x0000_0008;
const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 0x0001;
const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 0x0002;
const TF_IPP_FLAG_ACTIVE: u32 = 0x0001;
const TF_IPP_FLAG_ENABLED: u32 = 0x0002;
const LANGID_EN_US: u16 = 0x0409;
const HKL_EN_US: HKL = 0x0409_0409;

// TF_DA_* constants
const TF_CT_NONE: i32 = 0;
const TF_CT_SYSCOLOR: i32 = 1;
const TF_LS_NONE: i32 = 0;
const TF_LS_SOLID: i32 = 1;
const TF_LS_DOT: i32 = 2;
const TF_ATTR_INPUT: i32 = 0;
const TF_ATTR_TARGET_CONVERTED: i32 = 1;
const TF_ATTR_CONVERTED: i32 = 2;

// VARIANT VT_* constants
const VT_EMPTY: u16 = 0;
const VT_BSTR: u16 = 8;
const VT_DISPATCH: u16 = 9;
const VT_UNKNOWN: u16 = 13;

// ---------------------------------------------------------------------------
// Win64 ABI structs (verified against msctf.h / textstor.h)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_STATUS {
    dw_dynamic_flags: u32,
    dw_static_flags: u32,
}
const _: () = assert!(std::mem::size_of::<TF_STATUS>() == 8);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct TF_PRESERVEDKEY {
    u_vkey: u32,
    u_modifiers: u32,
}
const _: () = assert!(std::mem::size_of::<TF_PRESERVEDKEY>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_SELECTIONSTYLE {
    ase: i32,
    f_interim_char: BOOL,
}
const _: () = assert!(std::mem::size_of::<TF_SELECTIONSTYLE>() == 8);

#[repr(C)]
#[derive(Clone, Copy)]
struct TF_SELECTION {
    range: *mut c_void,
    style: TF_SELECTIONSTYLE,
}
const _: () = assert!(std::mem::size_of::<TF_SELECTION>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_LANGUAGEPROFILE {
    clsid: Align4Guid,
    langid: u16,
    catid: Align4Guid,
    f_active: BOOL,
    guid_profile: Align4Guid,
}
const _: () = assert!(std::mem::size_of::<TF_LANGUAGEPROFILE>() == 56);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, langid) == 16);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, catid) == 20);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, f_active) == 36);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, guid_profile) == 40);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_INPUTPROCESSORPROFILE {
    dw_profile_type: u32,
    langid: u16,
    clsid: Align4Guid,
    guid_profile: Align4Guid,
    catid: Align4Guid,
    hkl_substitute: HKL,
    dw_caps: u32,
    hkl: HKL,
    dw_flags: u32,
}
const _: () = assert!(std::mem::size_of::<TF_INPUTPROCESSORPROFILE>() == 88);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, clsid) == 8);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, hkl_substitute) == 56);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, hkl) == 72);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_DA_COLOR {
    ty: i32,
    val: u32,
}
const _: () = assert!(std::mem::size_of::<TF_DA_COLOR>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TF_DISPLAYATTRIBUTE {
    cr_text: TF_DA_COLOR,
    cr_bk: TF_DA_COLOR,
    ls_style: i32,
    f_bold_line: BOOL,
    cr_line: TF_DA_COLOR,
    b_attr: i32,
}
const _: () = assert!(std::mem::size_of::<TF_DISPLAYATTRIBUTE>() == 36);

// ---------------------------------------------------------------------------
// Low-level COM / guest helpers
// ---------------------------------------------------------------------------

fn register_vtable_ptrs(ptrs: &[usize]) {
    for &addr in ptrs {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
}

unsafe fn read_guid(p: *const u8) -> Option<Guid> {
    if p.is_null() {
        return None;
    }
    let mut b = [0u8; 16];
    std::ptr::copy_nonoverlapping(p, b.as_mut_ptr(), 16);
    Some(b)
}

unsafe fn write_guid(dst: *mut u8, g: &Guid) {
    std::ptr::copy_nonoverlapping(g.as_ptr(), dst, 16);
}

unsafe fn read_wide_len(ptr: *const u16, len: u32) -> String {
    if ptr.is_null() || len == 0 {
        return String::new();
    }
    let n = len as usize;
    let mut units = Vec::with_capacity(n);
    for i in 0..n {
        let u = std::ptr::read_unaligned(ptr.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
    }
    String::from_utf16_lossy(&units)
}

unsafe fn alloc_bstr_str(s: &str) -> *mut u16 {
    let w = crate::strings::to_wide_z(s);
    super::oleaut32::SysAllocString(w.as_ptr())
}

/// Invoke `IUnknown::QueryInterface` (slot 0) on any COM object (guest or native).
unsafe fn com_call_qi(obj: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if obj.is_null() {
        return E_POINTER;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return E_POINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(0));
    if method == 0 {
        return E_NOINTERFACE;
    }
    crate::call_guest8(method, obj as u64, riid as u64, ppv as u64, 0, 0, 0, 0, 0) as HRESULT
}

/// Invoke `IUnknown::AddRef` (slot 1) on any COM object (guest or native).
unsafe fn com_call_add_ref(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(1));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke `IUnknown::Release` (slot 2) on any COM object (guest or native).
unsafe fn com_call_release(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(2));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke arbitrary COM method slot with up to 3 extra arguments (beyond `this`).
unsafe fn com_call_slot(obj: *mut c_void, slot: usize, a1: u64, a2: u64, a3: u64) -> HRESULT {
    if obj.is_null() {
        return E_POINTER;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return E_POINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    if method == 0 {
        return E_NOTIMPL;
    }
    crate::call_guest8(method, obj as u64, a1, a2, a3, 0, 0, 0, 0) as HRESULT
}

// ---------------------------------------------------------------------------
// Owned Win64 VARIANT storage (24 bytes) with guest-safe refcounting & BSTR copy
// ---------------------------------------------------------------------------

struct OwnedVariant([u8; 24]);

impl Default for OwnedVariant {
    fn default() -> Self {
        Self([0u8; 24])
    }
}

impl OwnedVariant {
    fn is_empty(&self) -> bool {
        u16::from_le_bytes([self.0[0], self.0[1]]) == VT_EMPTY
    }

    unsafe fn from_raw(src: *const u8) -> Self {
        let mut raw = [0u8; 24];
        std::ptr::copy_nonoverlapping(src, raw.as_mut_ptr(), 24);
        let vt = u16::from_le_bytes([raw[0], raw[1]]);
        match vt {
            VT_BSTR => {
                let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
                let dup = if s.is_null() {
                    std::ptr::null_mut()
                } else {
                    super::oleaut32::SysAllocStringLen(s, super::oleaut32::SysStringLen(s))
                };
                std::ptr::write_unaligned(raw.as_mut_ptr().add(8) as *mut *mut u16, dup);
            }
            VT_UNKNOWN | VT_DISPATCH => {
                let unk = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut c_void);
                if !unk.is_null() {
                    com_call_add_ref(unk);
                }
            }
            _ => {}
        }
        Self(raw)
    }

    unsafe fn write_copy_to(&self, dst: *mut u8) {
        let mut raw = self.0;
        let vt = u16::from_le_bytes([raw[0], raw[1]]);
        match vt {
            VT_BSTR => {
                let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
                let dup = if s.is_null() {
                    std::ptr::null_mut()
                } else {
                    super::oleaut32::SysAllocStringLen(s, super::oleaut32::SysStringLen(s))
                };
                std::ptr::write_unaligned(raw.as_mut_ptr().add(8) as *mut *mut u16, dup);
            }
            VT_UNKNOWN | VT_DISPATCH => {
                let unk = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut c_void);
                if !unk.is_null() {
                    com_call_add_ref(unk);
                }
            }
            _ => {}
        }
        std::ptr::copy_nonoverlapping(raw.as_ptr(), dst, 24);
    }

    fn take_raw(&mut self) -> [u8; 24] {
        std::mem::replace(&mut self.0, [0u8; 24])
    }
}

unsafe fn release_variant_raw(raw: [u8; 24]) {
    let vt = u16::from_le_bytes([raw[0], raw[1]]);
    match vt {
        VT_BSTR => {
            let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
            if !s.is_null() {
                super::oleaut32::SysFreeString(s);
            }
        }
        VT_UNKNOWN | VT_DISPATCH => {
            let unk = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut c_void);
            if !unk.is_null() {
                com_call_release(unk);
            }
        }
        _ => {}
    }
}

impl Drop for OwnedVariant {
    fn drop(&mut self) {
        let raw = self.take_raw();
        unsafe { release_variant_raw(raw) };
    }
}

// ---------------------------------------------------------------------------
// Reusable ITfSource sink table (ref-counted sinks + cookie allocation)
// ---------------------------------------------------------------------------

struct SinkEntry {
    iid: Guid,
    punk: usize,
}

struct SinkTable {
    next_cookie: u32,
    entries: HashMap<u32, SinkEntry>,
}

impl Default for SinkTable {
    fn default() -> Self {
        Self { next_cookie: 1, entries: HashMap::new() }
    }
}

impl SinkTable {
    fn insert(&mut self, iid: Guid, punk: *mut c_void) -> u32 {
        let mut cookie = self.next_cookie;
        while cookie == 0 || cookie == TF_INVALID_COOKIE || self.entries.contains_key(&cookie) {
            cookie = cookie.wrapping_add(1);
        }
        self.next_cookie = cookie.wrapping_add(1);
        self.entries.insert(cookie, SinkEntry { iid, punk: punk as usize });
        cookie
    }

    fn remove(&mut self, cookie: u32) -> Option<*mut c_void> {
        self.entries.remove(&cookie).map(|e| e.punk as *mut c_void)
    }

    fn snapshot_for_iid(&self, target_iid: &Guid) -> Vec<*mut c_void> {
        self.entries
            .values()
            .filter(|e| &e.iid == target_iid)
            .map(|e| e.punk as *mut c_void)
            .collect()
    }

    fn drain_all(&mut self) -> Vec<*mut c_void> {
        self.entries.drain().map(|(_, e)| e.punk as *mut c_void).collect()
    }
}

/// Acquire `riid` from `punk` via COM `QueryInterface` and register it in `table`.
/// Performs COM QI outside the mutex lock so guest callbacks cannot deadlock.
unsafe fn sink_advise(
    table: &Mutex<SinkTable>,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    if !pdw_cookie.is_null() {
        std::ptr::write_unaligned(pdw_cookie, TF_INVALID_COOKIE);
    }
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if punk.is_null() || pdw_cookie.is_null() {
        return E_INVALIDARG;
    }
    let mut iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi(punk, riid, &mut iface);
    if hr < 0 || iface.is_null() {
        return CONNECT_E_CANNOTCONNECT;
    }
    let cookie = table.lock().unwrap().insert(iid, iface);
    std::ptr::write_unaligned(pdw_cookie, cookie);
    S_OK
}

unsafe fn sink_unadvise(table: &Mutex<SinkTable>, cookie: u32) -> HRESULT {
    let removed = table.lock().unwrap().remove(cookie);
    match removed {
        Some(ptr) => {
            com_call_release(ptr);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

/// Snapshot matching sinks, `AddRef` each before releasing the mutex (only safe if
/// callers don't call back in `AddRef`, or even better: collect raw ptrs under lock
/// ONLY for notification — note in single-threaded apartment TSF, sinks live on the
/// calling thread, and bumping AddRef on a copy before callback keeps the sink alive
/// if the callback calls `UnadviseSink` on itself).
unsafe fn snapshot_sinks_addref(table: &Mutex<SinkTable>, iid: &Guid) -> Vec<*mut c_void> {
    let raw = table.lock().unwrap().snapshot_for_iid(iid);
    for &p in &raw {
        com_call_add_ref(p);
    }
    raw
}

unsafe fn release_all_ptrs(ptrs: Vec<*mut c_void>) {
    for p in ptrs {
        com_call_release(p);
    }
}

// ---------------------------------------------------------------------------
// Generic TSF COM pointer enumerator (`IEnumTf*`: Clone, Next, Reset, Skip)
// ---------------------------------------------------------------------------

#[repr(C)]
struct IEnumTfComVtbl {
    query_interface: unsafe extern "win64" fn(*mut EnumTfComObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumTfComObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumTfComObj) -> u32,
    clone: unsafe extern "win64" fn(*mut EnumTfComObj, *mut *mut c_void) -> HRESULT,
    next: unsafe extern "win64" fn(*mut EnumTfComObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut EnumTfComObj) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut EnumTfComObj, u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<IEnumTfComVtbl>() == 7 * std::mem::size_of::<usize>());

#[repr(C)]
struct EnumTfComObj {
    vtbl: *const IEnumTfComVtbl,
    refs: AtomicU32,
    enum_iid: Guid,
    state: Mutex<(Vec<usize>, usize)>,
}

unsafe extern "win64" fn enum_tf_com_qi(
    this: *mut EnumTfComObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*this;
    if iid == IID_IUNKNOWN || iid == obj.enum_iid {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn enum_tf_com_add_ref(this: *mut EnumTfComObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn enum_tf_com_release(this: *mut EnumTfComObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        let boxed = Box::from_raw(this);
        let items: Vec<*mut c_void> = {
            let mut st = boxed.state.lock().unwrap();
            st.0.drain(..).map(|p| p as *mut c_void).collect()
        };
        drop(boxed);
        release_all_ptrs(items);
    }
    rem
}

unsafe extern "win64" fn enum_tf_com_clone(this: *mut EnumTfComObj, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
    let obj = &*this;
    let (items, pos) = {
        let st = obj.state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    for &p in &items {
        com_call_add_ref(p as *mut c_void);
    }
    let cloned = new_enum_tf_com(obj.enum_iid, items, pos);
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

unsafe extern "win64" fn enum_tf_com_next(
    this: *mut EnumTfComObj,
    ul_count: u32,
    rg_out: *mut *mut c_void,
    pc_fetched: *mut u32,
) -> HRESULT {
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, 0);
    }
    if ul_count > 1 && pc_fetched.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && rg_out.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let picked: Vec<usize> = {
        let mut st = obj.state.lock().unwrap();
        let avail = st.0.len().saturating_sub(st.1);
        let n = (ul_count as usize).min(avail);
        let slice = st.0[st.1..st.1 + n].to_vec();
        st.1 += n;
        slice
    };
    for (i, &p) in picked.iter().enumerate() {
        let ptr = p as *mut c_void;
        com_call_add_ref(ptr);
        std::ptr::write_unaligned(rg_out.add(i), ptr);
    }
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, picked.len() as u32);
    }
    if picked.len() == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn enum_tf_com_reset(this: *mut EnumTfComObj) -> HRESULT {
    let obj = &*this;
    obj.state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn enum_tf_com_skip(this: *mut EnumTfComObj, ul_count: u32) -> HRESULT {
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (ul_count as usize).min(avail);
    st.1 += step;
    if step == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

static IENUM_TF_COM_VTBL: IEnumTfComVtbl = IEnumTfComVtbl {
    query_interface: enum_tf_com_qi,
    add_ref: enum_tf_com_add_ref,
    release: enum_tf_com_release,
    clone: enum_tf_com_clone,
    next: enum_tf_com_next,
    reset: enum_tf_com_reset,
    skip: enum_tf_com_skip,
};

fn register_enum_tf_com_vtable() {
    register_vtable_ptrs(&[
        enum_tf_com_qi as *const () as usize,
        enum_tf_com_add_ref as *const () as usize,
        enum_tf_com_release as *const () as usize,
        enum_tf_com_clone as *const () as usize,
        enum_tf_com_next as *const () as usize,
        enum_tf_com_reset as *const () as usize,
        enum_tf_com_skip as *const () as usize,
    ]);
}

/// Takes ownership of one ref per element in `items`.
fn new_enum_tf_com(enum_iid: Guid, items: Vec<usize>, pos: usize) -> *mut EnumTfComObj {
    register_enum_tf_com_vtable();
    Box::into_raw(Box::new(EnumTfComObj {
        vtbl: &IENUM_TF_COM_VTBL,
        refs: AtomicU32::new(1),
        enum_iid,
        state: Mutex::new((items, pos)),
    }))
}

// ---------------------------------------------------------------------------
// Standard COM `IEnumGUID` (Next, Skip, Reset, Clone — comcat.h order!)
// ---------------------------------------------------------------------------

#[repr(C)]
struct IEnumGUIDVtbl {
    query_interface: unsafe extern "win64" fn(*mut EnumGuidObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumGuidObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumGuidObj) -> u32,
    next: unsafe extern "win64" fn(*mut EnumGuidObj, u32, *mut u8, *mut u32) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut EnumGuidObj, u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut EnumGuidObj) -> HRESULT,
    clone: unsafe extern "win64" fn(*mut EnumGuidObj, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<IEnumGUIDVtbl>() == 7 * std::mem::size_of::<usize>());

#[repr(C)]
struct EnumGuidObj {
    vtbl: *const IEnumGUIDVtbl,
    refs: AtomicU32,
    state: Mutex<(Vec<Guid>, usize)>,
}

unsafe extern "win64" fn enum_guid_qi(
    this: *mut EnumGuidObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_IENUMGUID {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn enum_guid_add_ref(this: *mut EnumGuidObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn enum_guid_release(this: *mut EnumGuidObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}

unsafe extern "win64" fn enum_guid_next(
    this: *mut EnumGuidObj,
    celt: u32,
    rgelt: *mut u8,
    pcelt_fetched: *mut u32,
) -> HRESULT {
    if !pcelt_fetched.is_null() {
        std::ptr::write_unaligned(pcelt_fetched, 0);
    }
    if celt > 1 && pcelt_fetched.is_null() {
        return E_INVALIDARG;
    }
    if celt > 0 && rgelt.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (celt as usize).min(avail);
    for i in 0..n {
        write_guid(rgelt.add(i * 16), &st.0[st.1 + i]);
    }
    st.1 += n;
    if !pcelt_fetched.is_null() {
        std::ptr::write_unaligned(pcelt_fetched, n as u32);
    }
    if n == celt as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn enum_guid_skip(this: *mut EnumGuidObj, celt: u32) -> HRESULT {
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (celt as usize).min(avail);
    st.1 += step;
    if step == celt as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn enum_guid_reset(this: *mut EnumGuidObj) -> HRESULT {
    let obj = &*this;
    obj.state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn enum_guid_clone(this: *mut EnumGuidObj, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
    let obj = &*this;
    let (items, pos) = {
        let st = obj.state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    let cloned = new_enum_guid(items, pos);
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

static IENUM_GUID_VTBL: IEnumGUIDVtbl = IEnumGUIDVtbl {
    query_interface: enum_guid_qi,
    add_ref: enum_guid_add_ref,
    release: enum_guid_release,
    next: enum_guid_next,
    skip: enum_guid_skip,
    reset: enum_guid_reset,
    clone: enum_guid_clone,
};

fn new_enum_guid(items: Vec<Guid>, pos: usize) -> *mut EnumGuidObj {
    register_vtable_ptrs(&[
        enum_guid_qi as *const () as usize,
        enum_guid_add_ref as *const () as usize,
        enum_guid_release as *const () as usize,
        enum_guid_next as *const () as usize,
        enum_guid_skip as *const () as usize,
        enum_guid_reset as *const () as usize,
        enum_guid_clone as *const () as usize,
    ]);
    Box::into_raw(Box::new(EnumGuidObj {
        vtbl: &IENUM_GUID_VTBL,
        refs: AtomicU32::new(1),
        state: Mutex::new((items, pos)),
    }))
}

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
struct CompartmentMap {
    entries: HashMap<Guid, Arc<CompartmentCell>>,
}

impl CompartmentMap {
    fn get_or_create(&mut self, guid: Guid) -> Arc<CompartmentCell> {
        Arc::clone(self.entries.entry(guid).or_insert_with(|| CompartmentCell::new(guid)))
    }
}

static GLOBAL_COMPARTMENTS: LazyLock<Arc<Mutex<CompartmentMap>>> =
    LazyLock::new(|| Arc::new(Mutex::new(CompartmentMap::default())));

#[repr(C)]
struct ITfSourceVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    advise_sink: unsafe extern "win64" fn(*mut c_void, *const u8, *mut c_void, *mut u32) -> HRESULT,
    unadvise_sink: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfSourceVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
struct ITfSourceSingleVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    advise_single_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8, *mut c_void) -> HRESULT,
    unadvise_single_sink: unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfSourceSingleVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
struct ITfCompartmentVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    set_value: unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
    get_value: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfCompartmentVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
struct CompartmentObj {
    vtbl_comp: *const ITfCompartmentVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    cell: Arc<CompartmentCell>,
}

unsafe fn comp_from_source(this: *mut c_void) -> *mut CompartmentObj {
    (this as *mut u8).sub(std::mem::offset_of!(CompartmentObj, vtbl_source)) as *mut CompartmentObj
}

unsafe fn comp_qi_common(base: *mut CompartmentObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*base;
    if iid == IID_IUNKNOWN || iid == IID_ITFCOMPARTMENT {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        S_OK
    } else if iid == IID_ITFSOURCE {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let p = (base as *mut u8).add(std::mem::offset_of!(CompartmentObj, vtbl_source)) as *mut c_void;
        std::ptr::write_unaligned(ppv, p);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn comp_add_ref_common(base: *mut CompartmentObj) -> u32 {
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn comp_release_common(base: *mut CompartmentObj) -> u32 {
    let obj = &*base;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(base));
    }
    rem
}

unsafe extern "win64" fn comp_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    comp_qi_common(this as *mut CompartmentObj, riid, ppv)
}
unsafe extern "win64" fn comp_add_ref(this: *mut c_void) -> u32 {
    comp_add_ref_common(this as *mut CompartmentObj)
}
unsafe extern "win64" fn comp_release(this: *mut c_void) -> u32 {
    comp_release_common(this as *mut CompartmentObj)
}
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
        let _ = com_call_slot(sink, 3, guid_copy.as_ptr() as u64, 0, 0);
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

unsafe extern "win64" fn comp_src_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    comp_qi_common(comp_from_source(this), riid, ppv)
}
unsafe extern "win64" fn comp_src_add_ref(this: *mut c_void) -> u32 {
    comp_add_ref_common(comp_from_source(this))
}
unsafe extern "win64" fn comp_src_release(this: *mut c_void) -> u32 {
    comp_release_common(comp_from_source(this))
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

#[repr(C)]
struct ITfCompartmentMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    get_compartment: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    clear_compartment: unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
    enum_compartments: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfCompartmentMgrVtbl>() == 6 * std::mem::size_of::<usize>());

unsafe fn comp_mgr_get_compartment_impl(
    store: &Mutex<CompartmentMap>,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    if !ppcomp.is_null() {
        std::ptr::write_unaligned(ppcomp, std::ptr::null_mut());
    }
    let Some(g) = read_guid(rguid) else {
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

unsafe fn comp_mgr_clear_compartment_impl(
    store: &Mutex<CompartmentMap>,
    _tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    let cell = store.lock().unwrap().entries.get(&g).cloned();
    if let Some(c) = cell {
        let old_raw = c.value.lock().unwrap().take_raw();
        release_variant_raw(old_raw);
        let sinks = snapshot_sinks_addref(&c.sinks, &IID_ITFCOMPARTMENTEVENTSINK);
        for sink in sinks {
            let _ = com_call_slot(sink, 3, g.as_ptr() as u64, 0, 0);
            com_call_release(sink);
        }
    }
    S_OK
}

unsafe fn comp_mgr_enum_compartments_impl(
    store: &Mutex<CompartmentMap>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
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
#[repr(C)]
struct CompartmentMgrObj {
    vtbl: *const ITfCompartmentMgrVtbl,
    refs: AtomicU32,
    store: Arc<Mutex<CompartmentMap>>,
}

unsafe extern "win64" fn cmgr_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFCOMPARTMENTMGR {
        let obj = &*(this as *const CompartmentMgrObj);
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn cmgr_add_ref(this: *mut c_void) -> u32 {
    let obj = &*(this as *const CompartmentMgrObj);
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn cmgr_release(this: *mut c_void) -> u32 {
    let base = this as *mut CompartmentMgrObj;
    let obj = &*base;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(base));
    }
    rem
}
unsafe extern "win64" fn cmgr_get_compartment(
    this: *mut c_void,
    rguid: *const u8,
    ppcomp: *mut *mut c_void,
) -> HRESULT {
    let obj = &*(this as *const CompartmentMgrObj);
    comp_mgr_get_compartment_impl(&obj.store, rguid, ppcomp)
}
unsafe extern "win64" fn cmgr_clear_compartment(
    this: *mut c_void,
    tid: TfClientId,
    rguid: *const u8,
) -> HRESULT {
    let obj = &*(this as *const CompartmentMgrObj);
    comp_mgr_clear_compartment_impl(&obj.store, tid, rguid)
}
unsafe extern "win64" fn cmgr_enum_compartments(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    let obj = &*(this as *const CompartmentMgrObj);
    comp_mgr_enum_compartments_impl(&obj.store, pp_enum)
}

static STANDALONE_COMPARTMENT_MGR_VTBL: ITfCompartmentMgrVtbl = ITfCompartmentMgrVtbl {
    query_interface: cmgr_qi,
    add_ref: cmgr_add_ref,
    release: cmgr_release,
    get_compartment: cmgr_get_compartment,
    clear_compartment: cmgr_clear_compartment,
    enum_compartments: cmgr_enum_compartments,
};

fn new_compartment_mgr_obj(store: Arc<Mutex<CompartmentMap>>) -> *mut CompartmentMgrObj {
    register_vtable_ptrs(&[
        cmgr_qi as *const () as usize,
        cmgr_add_ref as *const () as usize,
        cmgr_release as *const () as usize,
        cmgr_get_compartment as *const () as usize,
        cmgr_clear_compartment as *const () as usize,
        cmgr_enum_compartments as *const () as usize,
    ]);
    Box::into_raw(Box::new(CompartmentMgrObj {
        vtbl: &STANDALONE_COMPARTMENT_MGR_VTBL,
        refs: AtomicU32::new(1),
        store,
    }))
}

// ---------------------------------------------------------------------------
// `ITfRange` (25 slots) & `ITfProperty` / `ITfReadOnlyProperty` (11 / 7 slots)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ITfRangeVtbl {
    query_interface: unsafe extern "win64" fn(*mut RangeObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut RangeObj) -> u32,
    release: unsafe extern "win64" fn(*mut RangeObj) -> u32,
    get_text: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut u16, u32, *mut u32) -> HRESULT,
    set_text: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *const u16, i32) -> HRESULT,
    get_formatted_text: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut *mut c_void) -> HRESULT,
    get_embedded: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *const u8, *const u8, *mut *mut c_void) -> HRESULT,
    insert_embedded: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut c_void) -> HRESULT,
    shift_start: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut i32, *const c_void) -> HRESULT,
    shift_end: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut i32, *const c_void) -> HRESULT,
    shift_start_to_range: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32) -> HRESULT,
    shift_end_to_range: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32) -> HRESULT,
    shift_start_region: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut BOOL) -> HRESULT,
    shift_end_region: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, *mut BOOL) -> HRESULT,
    is_empty: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut BOOL) -> HRESULT,
    collapse: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32) -> HRESULT,
    is_equal_start: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut BOOL) -> HRESULT,
    is_equal_end: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut BOOL) -> HRESULT,
    compare_start: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut i32) -> HRESULT,
    compare_end: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, *mut c_void, i32, *mut i32) -> HRESULT,
    adjust_for_insert: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, u32, *mut BOOL) -> HRESULT,
    get_gravity: unsafe extern "win64" fn(*mut RangeObj, *mut i32, *mut i32) -> HRESULT,
    set_gravity: unsafe extern "win64" fn(*mut RangeObj, TfEditCookie, i32, i32) -> HRESULT,
    clone: unsafe extern "win64" fn(*mut RangeObj, *mut *mut c_void) -> HRESULT,
    get_context: unsafe extern "win64" fn(*mut RangeObj, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfRangeVtbl>() == 25 * std::mem::size_of::<usize>());

#[repr(C)]
struct RangeObj {
    vtbl: *const ITfRangeVtbl,
    refs: AtomicU32,
    ctx_state: Arc<Mutex<ContextInner>>,
    gravity: Mutex<(i32, i32)>,
}

unsafe extern "win64" fn range_qi(this: *mut RangeObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFRANGE {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn range_add_ref(this: *mut RangeObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn range_release(this: *mut RangeObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
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
    if !pp_data.is_null() {
        std::ptr::write_unaligned(pp_data, std::ptr::null_mut());
    }
    E_NOTIMPL
}
unsafe extern "win64" fn range_get_embedded(
    _this: *mut RangeObj,
    _ec: TfEditCookie,
    _rguid_service: *const u8,
    _riid: *const u8,
    ppunk: *mut *mut c_void,
) -> HRESULT {
    if !ppunk.is_null() {
        std::ptr::write_unaligned(ppunk, std::ptr::null_mut());
    }
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
unsafe extern "win64" fn range_collapse(_this: *mut RangeObj, ec: TfEditCookie, _a_pos: i32) -> HRESULT {
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
    let obj = &*this;
    let (gs, ge) = *obj.gravity.lock().unwrap();
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
    let obj = &*this;
    *obj.gravity.lock().unwrap() = (g_start, g_end);
    S_OK
}
unsafe extern "win64" fn range_clone(this: *mut RangeObj, pp_clone: *mut *mut c_void) -> HRESULT {
    if pp_clone.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let g = *obj.gravity.lock().unwrap();
    let cloned = new_range_obj(Arc::clone(&obj.ctx_state), g);
    std::ptr::write_unaligned(pp_clone, cloned as *mut c_void);
    S_OK
}
unsafe extern "win64" fn range_get_context(this: *mut RangeObj, pp_context: *mut *mut c_void) -> HRESULT {
    if pp_context.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_context, std::ptr::null_mut());
    let obj = &*this;
    let maybe_ctx = {
        let st = obj.ctx_state.lock().unwrap();
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
    add_ref: range_add_ref,
    release: range_release,
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

fn new_range_obj(ctx_state: Arc<Mutex<ContextInner>>, gravity: (i32, i32)) -> *mut RangeObj {
    register_vtable_ptrs(&[
        range_qi as *const () as usize,
        range_add_ref as *const () as usize,
        range_release as *const () as usize,
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
    Box::into_raw(Box::new(RangeObj {
        vtbl: &ITF_RANGE_VTBL,
        refs: AtomicU32::new(1),
        ctx_state,
        gravity: Mutex::new(gravity),
    }))
}

#[repr(C)]
struct ITfPropertyVtbl {
    query_interface: unsafe extern "win64" fn(*mut PropertyObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut PropertyObj) -> u32,
    release: unsafe extern "win64" fn(*mut PropertyObj) -> u32,
    // ITfReadOnlyProperty (slots 3..=6)
    get_type: unsafe extern "win64" fn(*mut PropertyObj, *mut u8) -> HRESULT,
    enum_ranges: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut *mut c_void, *mut c_void) -> HRESULT,
    get_value: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *mut u8) -> HRESULT,
    get_context: unsafe extern "win64" fn(*mut PropertyObj, *mut *mut c_void) -> HRESULT,
    // ITfProperty (slots 7..=10)
    find_range: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *mut *mut c_void, i32) -> HRESULT,
    set_value_store: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *mut c_void) -> HRESULT,
    set_value: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void, *const u8) -> HRESULT,
    clear: unsafe extern "win64" fn(*mut PropertyObj, TfEditCookie, *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfPropertyVtbl>() == 11 * std::mem::size_of::<usize>());

struct PropertyData {
    guid_type: Guid,
    value: Mutex<OwnedVariant>,
}

#[repr(C)]
struct PropertyObj {
    vtbl: *const ITfPropertyVtbl,
    refs: AtomicU32,
    read_only: bool,
    data: Arc<PropertyData>,
    ctx_state: Arc<Mutex<ContextInner>>,
}

unsafe extern "win64" fn prop_qi(this: *mut PropertyObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*this;
    if iid == IID_IUNKNOWN
        || iid == IID_ITFREADONLYPROPERTY
        || (!obj.read_only && iid == IID_ITFPROPERTY)
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn prop_add_ref(this: *mut PropertyObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn prop_release(this: *mut PropertyObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn prop_get_type(this: *mut PropertyObj, pguid: *mut u8) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    write_guid(pguid, &obj.data.guid_type);
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
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
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
    let obj = &*this;
    let guard = obj.data.value.lock().unwrap();
    if guard.is_empty() {
        return S_FALSE;
    }
    guard.write_copy_to(pvar_value);
    S_OK
}
unsafe extern "win64" fn prop_get_context(this: *mut PropertyObj, pp_context: *mut *mut c_void) -> HRESULT {
    if pp_context.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_context, std::ptr::null_mut());
    let obj = &*this;
    let maybe_ctx = {
        let st = obj.ctx_state.lock().unwrap();
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
    std::ptr::write_unaligned(pp_range, std::ptr::null_mut());
    if ec == TF_INVALID_EDIT_COOKIE {
        return TF_E_NOLOCK;
    }
    let obj = &*this;
    let r = new_range_obj(Arc::clone(&obj.ctx_state), (0, 0));
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
    let obj = &*this;
    let new_val = OwnedVariant::from_raw(pvar_value);
    let old_raw = {
        let mut guard = obj.data.value.lock().unwrap();
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
    let obj = &*this;
    let old_raw = obj.data.value.lock().unwrap().take_raw();
    release_variant_raw(old_raw);
    S_OK
}

static ITF_PROPERTY_VTBL: ITfPropertyVtbl = ITfPropertyVtbl {
    query_interface: prop_qi,
    add_ref: prop_add_ref,
    release: prop_release,
    get_type: prop_get_type,
    enum_ranges: prop_enum_ranges,
    get_value: prop_get_value,
    get_context: prop_get_context,
    find_range: prop_find_range,
    set_value_store: prop_set_value_store,
    set_value: prop_set_value,
    clear: prop_clear,
};

fn new_property_obj(
    data: Arc<PropertyData>,
    ctx_state: Arc<Mutex<ContextInner>>,
    read_only: bool,
) -> *mut PropertyObj {
    register_vtable_ptrs(&[
        prop_qi as *const () as usize,
        prop_add_ref as *const () as usize,
        prop_release as *const () as usize,
        prop_get_type as *const () as usize,
        prop_enum_ranges as *const () as usize,
        prop_get_value as *const () as usize,
        prop_get_context as *const () as usize,
        prop_find_range as *const () as usize,
        prop_set_value_store as *const () as usize,
        prop_set_value as *const () as usize,
        prop_clear as *const () as usize,
    ]);
    Box::into_raw(Box::new(PropertyObj {
        vtbl: &ITF_PROPERTY_VTBL,
        refs: AtomicU32::new(1),
        read_only,
        data,
        ctx_state,
    }))
}

// ---------------------------------------------------------------------------
// `ITfContext` (18 slots) + `ITfSource` + `ITfSourceSingle` + `ITfCompartmentMgr`
// ---------------------------------------------------------------------------

#[repr(C)]
struct ITfContextVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    request_edit_session:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *mut c_void, u32, *mut HRESULT) -> HRESULT,
    in_write_session: unsafe extern "win64" fn(*mut c_void, TfClientId, *mut BOOL) -> HRESULT,
    get_selection:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, u32, u32, *mut u8, *mut u32) -> HRESULT,
    set_selection: unsafe extern "win64" fn(*mut c_void, TfEditCookie, u32, *const u8) -> HRESULT,
    get_start: unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut *mut c_void) -> HRESULT,
    get_end: unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut *mut c_void) -> HRESULT,
    get_active_view: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enum_views: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_status: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
    get_property: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    get_app_property: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    track_properties: unsafe extern "win64" fn(
        *mut c_void,
        *const *const u8,
        u32,
        *const *const u8,
        u32,
        *mut *mut c_void,
    ) -> HRESULT,
    enum_properties: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    create_range_backup:
        unsafe extern "win64" fn(*mut c_void, TfEditCookie, *mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfContextVtbl>() == 18 * std::mem::size_of::<usize>());

struct StoredSelection {
    range: usize,
    style: TF_SELECTIONSTYLE,
}

struct ContextInner {
    live_self: usize,
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
struct ContextObj {
    vtbl_ctx: *const ITfContextVtbl,
    vtbl_source: *const ITfSourceVtbl,
    vtbl_source_single: *const ITfSourceSingleVtbl,
    vtbl_comp_mgr: *const ITfCompartmentMgrVtbl,
    refs: AtomicU32,
    inner: Arc<Mutex<ContextInner>>,
    sinks: Mutex<SinkTable>,
    compartments: Mutex<CompartmentMap>,
}

unsafe fn try_add_ref_context(raw: usize) -> Option<*mut ContextObj> {
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

unsafe fn ctx_from_source(this: *mut c_void) -> *mut ContextObj {
    (this as *mut u8).sub(std::mem::offset_of!(ContextObj, vtbl_source)) as *mut ContextObj
}
unsafe fn ctx_from_source_single(this: *mut c_void) -> *mut ContextObj {
    (this as *mut u8).sub(std::mem::offset_of!(ContextObj, vtbl_source_single)) as *mut ContextObj
}
unsafe fn ctx_from_comp_mgr(this: *mut c_void) -> *mut ContextObj {
    (this as *mut u8).sub(std::mem::offset_of!(ContextObj, vtbl_comp_mgr)) as *mut ContextObj
}

unsafe fn ctx_qi_common(base: *mut ContextObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*base;
    let target = if iid == IID_IUNKNOWN || iid == IID_ITFCONTEXT {
        base as *mut c_void
    } else if iid == IID_ITFSOURCE {
        (base as *mut u8).add(std::mem::offset_of!(ContextObj, vtbl_source)) as *mut c_void
    } else if iid == IID_ITFSOURCESINGLE {
        (base as *mut u8).add(std::mem::offset_of!(ContextObj, vtbl_source_single)) as *mut c_void
    } else if iid == IID_ITFCOMPARTMENTMGR {
        (base as *mut u8).add(std::mem::offset_of!(ContextObj, vtbl_comp_mgr)) as *mut c_void
    } else {
        return E_NOINTERFACE;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel);
    std::ptr::write_unaligned(ppv, target);
    S_OK
}

unsafe fn ctx_add_ref_common(base: *mut ContextObj) -> u32 {
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
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

unsafe extern "win64" fn ctx_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ctx_qi_common(this as *mut ContextObj, riid, ppv)
}
unsafe extern "win64" fn ctx_add_ref(this: *mut c_void) -> u32 {
    ctx_add_ref_common(this as *mut ContextObj)
}
unsafe extern "win64" fn ctx_release(this: *mut c_void) -> u32 {
    ctx_release_common(this as *mut ContextObj)
}
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
        std::ptr::write_unaligned(p_selection.add(i * std::mem::size_of::<TF_SELECTION>()) as *mut TF_SELECTION, out);
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
    std::ptr::write_unaligned(pp_start, std::ptr::null_mut());
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
    std::ptr::write_unaligned(pp_end, std::ptr::null_mut());
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
    std::ptr::write_unaligned(pp_view, std::ptr::null_mut());
    TF_E_INVALIDVIEW
}
unsafe extern "win64" fn ctx_enum_views(_this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_enum, new_enum_tf_com(IID_IENUMTFCONTEXTVIEWS, Vec::new(), 0) as *mut c_void);
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
    if !pp_prop.is_null() {
        std::ptr::write_unaligned(pp_prop, std::ptr::null_mut());
    }
    let Some(g) = read_guid(guid_prop) else {
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
    if !pp_prop.is_null() {
        std::ptr::write_unaligned(pp_prop, std::ptr::null_mut());
    }
    let Some(g) = read_guid(guid_prop) else {
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
    _this: *mut c_void,
    _prg_prop: *const *const u8,
    _c_prop: u32,
    _prg_app_prop: *const *const u8,
    _c_app_prop: u32,
    pp_property: *mut *mut c_void,
) -> HRESULT {
    if !pp_property.is_null() {
        std::ptr::write_unaligned(pp_property, std::ptr::null_mut());
    }
    stub!("ITfContext::TrackProperties -> E_NOTIMPL");
    E_NOTIMPL
}
unsafe extern "win64" fn ctx_enum_properties(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
    let obj = &*(this as *const ContextObj);
    let props: Vec<Arc<PropertyData>> = obj.inner.lock().unwrap().properties.values().cloned().collect();
    let ptrs: Vec<usize> = props
        .into_iter()
        .map(|d| new_property_obj(d, Arc::clone(&obj.inner), false) as usize)
        .collect();
    std::ptr::write_unaligned(pp_enum, new_enum_tf_com(IID_IENUMTFPROPERTIES, ptrs, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn ctx_get_document_mgr(this: *mut c_void, pp_dm: *mut *mut c_void) -> HRESULT {
    if pp_dm.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_dm, std::ptr::null_mut());
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
    if !pp_backup.is_null() {
        std::ptr::write_unaligned(pp_backup, std::ptr::null_mut());
    }
    E_NOTIMPL
}

unsafe extern "win64" fn ctx_src_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ctx_qi_common(ctx_from_source(this), riid, ppv)
}
unsafe extern "win64" fn ctx_src_add_ref(this: *mut c_void) -> u32 {
    ctx_add_ref_common(ctx_from_source(this))
}
unsafe extern "win64" fn ctx_src_release(this: *mut c_void) -> u32 {
    ctx_release_common(ctx_from_source(this))
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

unsafe extern "win64" fn ctx_ss_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ctx_qi_common(ctx_from_source_single(this), riid, ppv)
}
unsafe extern "win64" fn ctx_ss_add_ref(this: *mut c_void) -> u32 {
    ctx_add_ref_common(ctx_from_source_single(this))
}
unsafe extern "win64" fn ctx_ss_release(this: *mut c_void) -> u32 {
    ctx_release_common(ctx_from_source_single(this))
}
unsafe extern "win64" fn ctx_ss_advise_single(
    this: *mut c_void,
    tid: TfClientId,
    riid: *const u8,
    punk: *mut c_void,
) -> HRESULT {
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if punk.is_null() {
        return E_INVALIDARG;
    }
    let mut iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi(punk, riid, &mut iface);
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
    let Some(iid) = read_guid(riid) else {
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

unsafe extern "win64" fn ctx_cm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ctx_qi_common(ctx_from_comp_mgr(this), riid, ppv)
}
unsafe extern "win64" fn ctx_cm_add_ref(this: *mut c_void) -> u32 {
    ctx_add_ref_common(ctx_from_comp_mgr(this))
}
unsafe extern "win64" fn ctx_cm_release(this: *mut c_void) -> u32 {
    ctx_release_common(ctx_from_comp_mgr(this))
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
struct ITfDocumentMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    create_context: unsafe extern "win64" fn(
        *mut c_void,
        TfClientId,
        u32,
        *mut c_void,
        *mut *mut c_void,
        *mut TfEditCookie,
    ) -> HRESULT,
    push: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    pop: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    get_top: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_base: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enum_contexts: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfDocumentMgrVtbl>() == 9 * std::mem::size_of::<usize>());

struct DocMgrInner {
    live_self: usize,
    tm_state: Arc<Mutex<ThreadMgrState>>,
    tm_sinks: Arc<Mutex<SinkTable>>,
    contexts: Vec<usize>,
    next_text_store_cookie: TfEditCookie,
}

#[repr(C)]
struct DocumentMgrObj {
    vtbl_dm: *const ITfDocumentMgrVtbl,
    vtbl_comp_mgr: *const ITfCompartmentMgrVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    inner: Arc<Mutex<DocMgrInner>>,
    compartments: Mutex<CompartmentMap>,
    sinks: Mutex<SinkTable>,
}

unsafe fn try_add_ref_doc_mgr(raw: usize) -> Option<*mut DocumentMgrObj> {
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

unsafe fn dm_from_comp_mgr(this: *mut c_void) -> *mut DocumentMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(DocumentMgrObj, vtbl_comp_mgr)) as *mut DocumentMgrObj
}
unsafe fn dm_from_source(this: *mut c_void) -> *mut DocumentMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(DocumentMgrObj, vtbl_source)) as *mut DocumentMgrObj
}

unsafe fn dm_qi_common(base: *mut DocumentMgrObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*base;
    let target = if iid == IID_IUNKNOWN || iid == IID_ITFDOCUMENTMGR {
        base as *mut c_void
    } else if iid == IID_ITFCOMPARTMENTMGR {
        (base as *mut u8).add(std::mem::offset_of!(DocumentMgrObj, vtbl_comp_mgr)) as *mut c_void
    } else if iid == IID_ITFSOURCE {
        (base as *mut u8).add(std::mem::offset_of!(DocumentMgrObj, vtbl_source)) as *mut c_void
    } else {
        return E_NOINTERFACE;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel);
    std::ptr::write_unaligned(ppv, target);
    S_OK
}

unsafe fn dm_add_ref_common(base: *mut DocumentMgrObj) -> u32 {
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
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
    // Notify OnUninitDocumentMgr while `base` is still valid? No — refcount is already 0,
    // so only release remaining contexts and sinks after freeing to avoid resurrection UB.
    let _ = tm_sinks;
    let sinks = obj.sinks.lock().unwrap().drain_all();
    drop(Box::from_raw(base));
    release_all_ptrs(contexts);
    release_all_ptrs(sinks);
    0
}

unsafe extern "win64" fn dm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    dm_qi_common(this as *mut DocumentMgrObj, riid, ppv)
}
unsafe extern "win64" fn dm_add_ref(this: *mut c_void) -> u32 {
    dm_add_ref_common(this as *mut DocumentMgrObj)
}
unsafe extern "win64" fn dm_release(this: *mut c_void) -> u32 {
    dm_release_common(this as *mut DocumentMgrObj)
}
unsafe extern "win64" fn dm_create_context(
    this: *mut c_void,
    tid_owner: TfClientId,
    dw_flags: u32,
    punk: *mut c_void,
    ppic: *mut *mut c_void,
    pec_text_store: *mut TfEditCookie,
) -> HRESULT {
    if !ppic.is_null() {
        std::ptr::write_unaligned(ppic, std::ptr::null_mut());
    }
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
    std::ptr::write_unaligned(ppic, std::ptr::null_mut());
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
    std::ptr::write_unaligned(ppic, std::ptr::null_mut());
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
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
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

unsafe extern "win64" fn dm_cm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    dm_qi_common(dm_from_comp_mgr(this), riid, ppv)
}
unsafe extern "win64" fn dm_cm_add_ref(this: *mut c_void) -> u32 {
    dm_add_ref_common(dm_from_comp_mgr(this))
}
unsafe extern "win64" fn dm_cm_release(this: *mut c_void) -> u32 {
    dm_release_common(dm_from_comp_mgr(this))
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

unsafe extern "win64" fn dm_src_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    dm_qi_common(dm_from_source(this), riid, ppv)
}
unsafe extern "win64" fn dm_src_add_ref(this: *mut c_void) -> u32 {
    dm_add_ref_common(dm_from_source(this))
}
unsafe extern "win64" fn dm_src_release(this: *mut c_void) -> u32 {
    dm_release_common(dm_from_source(this))
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

fn new_document_mgr_obj(
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

// ---------------------------------------------------------------------------
// `ThreadMgrObj`: `ITfThreadMgr` (14) / `ITfThreadMgrEx` (16) / `ITfThreadMgr2` (17)
//                 + `ITfSource` (5) + `ITfSourceSingle` (5)
//                 + `ITfUIElementMgr` (8) + `ITfCompartmentMgr` (6)
//                 + `ITfKeystrokeMgr` (17)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ITfThreadMgrExVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    activate: unsafe extern "win64" fn(*mut c_void, *mut TfClientId) -> HRESULT,
    deactivate: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    create_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enum_document_mgrs: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_focus: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    set_focus: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    associate_focus:
        unsafe extern "win64" fn(*mut c_void, HWND, *mut c_void, *mut *mut c_void) -> HRESULT,
    is_thread_focus: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT,
    get_function_provider:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    enum_function_providers: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_global_compartment: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    // ITfThreadMgrEx (slots 14..=15)
    activate_ex: unsafe extern "win64" fn(*mut c_void, *mut TfClientId, u32) -> HRESULT,
    get_active_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfThreadMgrExVtbl>() == 16 * std::mem::size_of::<usize>());

#[repr(C)]
struct ITfThreadMgr2Vtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    activate: unsafe extern "win64" fn(*mut c_void, *mut TfClientId) -> HRESULT,
    deactivate: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    create_document_mgr: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enum_document_mgrs: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_focus: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    set_focus: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    is_thread_focus: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT,
    get_function_provider:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    enum_function_providers: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_global_compartment: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    activate_ex: unsafe extern "win64" fn(*mut c_void, *mut TfClientId, u32) -> HRESULT,
    get_active_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
    suspend_keystroke_handling: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    resume_keystroke_handling: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfThreadMgr2Vtbl>() == 17 * std::mem::size_of::<usize>());

#[repr(C)]
struct ITfUIElementMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    begin_ui_element:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut BOOL, *mut u32) -> HRESULT,
    update_ui_element: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    end_ui_element: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    get_ui_element: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT,
    enum_ui_elements: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfUIElementMgrVtbl>() == 8 * std::mem::size_of::<usize>());

#[repr(C)]
struct ITfKeystrokeMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    advise_key_event_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *mut c_void, BOOL) -> HRESULT,
    unadvise_key_event_sink: unsafe extern "win64" fn(*mut c_void, TfClientId) -> HRESULT,
    get_foreground: unsafe extern "win64" fn(*mut c_void, *mut u8) -> HRESULT,
    test_key_down: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    test_key_up: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    key_down: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    key_up: unsafe extern "win64" fn(*mut c_void, usize, isize, *mut BOOL) -> HRESULT,
    get_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *const u8, *mut u8) -> HRESULT,
    is_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u8, *mut BOOL) -> HRESULT,
    preserve_key: unsafe extern "win64" fn(
        *mut c_void,
        TfClientId,
        *const u8,
        *const u8,
        *const u16,
        u32,
    ) -> HRESULT,
    unpreserve_key: unsafe extern "win64" fn(*mut c_void, *const u8, *const u8) -> HRESULT,
    set_preserved_key_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u16, u32) -> HRESULT,
    get_preserved_key_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut u16) -> HRESULT,
    simulate_preserved_key:
        unsafe extern "win64" fn(*mut c_void, *mut c_void, *const u8, *mut BOOL) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfKeystrokeMgrVtbl>() == 17 * std::mem::size_of::<usize>());

struct PreservedKeyEntry {
    #[allow(dead_code)]
    tid: TfClientId,
    prekey: TF_PRESERVEDKEY,
    desc: String,
}

struct ThreadMgrState {
    activation_count: u32,
    active_flags: u32,
    client_id: TfClientId,
    keystroke_suspend_count: u32,
    focus_doc_mgr: usize,
    hwnd_focus: HashMap<usize, usize>,
    /// Non-owning pointers to live `DocumentMgrObj`s created on this thread.
    doc_mgrs: Vec<usize>,
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
struct ThreadMgrObj {
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

fn current_tid() -> u32 {
    unsafe { crate::kernel32::threads::GetCurrentThreadId() }
}

unsafe fn tm_from_tm2(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_tm2)) as *mut ThreadMgrObj
}
unsafe fn tm_from_source(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_source)) as *mut ThreadMgrObj
}
unsafe fn tm_from_source_single(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_source_single)) as *mut ThreadMgrObj
}
unsafe fn tm_from_ui_elem(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_ui_elem)) as *mut ThreadMgrObj
}
unsafe fn tm_from_comp_mgr(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_comp_mgr)) as *mut ThreadMgrObj
}
unsafe fn tm_from_keystroke(this: *mut c_void) -> *mut ThreadMgrObj {
    (this as *mut u8).sub(std::mem::offset_of!(ThreadMgrObj, vtbl_keystroke)) as *mut ThreadMgrObj
}

fn tm_supports_iid(iid: &Guid) -> bool {
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

unsafe fn tm_qi_common(base: *mut ThreadMgrObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let target = if iid == IID_IUNKNOWN || iid == IID_ITFTHREADMGR || iid == IID_ITFTHREADMGREX {
        base as *mut c_void
    } else if iid == IID_ITFTHREADMGR2 {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_tm2)) as *mut c_void
    } else if iid == IID_ITFSOURCE {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_source)) as *mut c_void
    } else if iid == IID_ITFSOURCESINGLE {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_source_single)) as *mut c_void
    } else if iid == IID_ITFUIELEMENTMGR {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_ui_elem)) as *mut c_void
    } else if iid == IID_ITFCOMPARTMENTMGR {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_comp_mgr)) as *mut c_void
    } else if iid == IID_ITFKEYSTROKEMGR {
        (base as *mut u8).add(std::mem::offset_of!(ThreadMgrObj, vtbl_keystroke)) as *mut c_void
    } else {
        return E_NOINTERFACE;
    };
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel);
    std::ptr::write_unaligned(ppv, target);
    S_OK
}

unsafe fn tm_add_ref_common(base: *mut ThreadMgrObj) -> u32 {
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn tm_release_common(base: *mut ThreadMgrObj) -> u32 {
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
    std::ptr::write_unaligned(ppdim, std::ptr::null_mut());
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
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
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
    std::ptr::write_unaligned(ppdim_focus, std::ptr::null_mut());
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
    if !ppdim_prev.is_null() {
        std::ptr::write_unaligned(ppdim_prev, std::ptr::null_mut());
    }
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
    if !pp_func_prov.is_null() {
        std::ptr::write_unaligned(pp_func_prov, std::ptr::null_mut());
    }
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
unsafe extern "win64" fn tm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(this as *mut ThreadMgrObj, riid, ppv)
}
unsafe extern "win64" fn tm_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(this as *mut ThreadMgrObj)
}
unsafe extern "win64" fn tm_release(this: *mut c_void) -> u32 {
    tm_release_common(this as *mut ThreadMgrObj)
}
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
unsafe extern "win64" fn tm2_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_tm2(this), riid, ppv)
}
unsafe extern "win64" fn tm2_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_tm2(this))
}
unsafe extern "win64" fn tm2_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_tm2(this))
}
unsafe extern "win64" fn tm2_activate(this: *mut c_void, ptid: *mut TfClientId) -> HRESULT {
    tm_activate_impl(tm_from_tm2(this), ptid, 0)
}
unsafe extern "win64" fn tm2_deactivate(this: *mut c_void) -> HRESULT {
    tm_deactivate_impl(tm_from_tm2(this))
}
unsafe extern "win64" fn tm2_create_doc_mgr(this: *mut c_void, ppdim: *mut *mut c_void) -> HRESULT {
    tm_create_document_mgr_impl(tm_from_tm2(this), ppdim)
}
unsafe extern "win64" fn tm2_enum_doc_mgrs(this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
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
unsafe extern "win64" fn tm2_enum_func_providers(this: *mut c_void, pp: *mut *mut c_void) -> HRESULT {
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
unsafe extern "win64" fn tm_src_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_source(this), riid, ppv)
}
unsafe extern "win64" fn tm_src_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_source(this))
}
unsafe extern "win64" fn tm_src_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_source(this))
}
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
unsafe extern "win64" fn tm_ss_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_source_single(this), riid, ppv)
}
unsafe extern "win64" fn tm_ss_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_source_single(this))
}
unsafe extern "win64" fn tm_ss_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_source_single(this))
}
unsafe extern "win64" fn tm_ss_advise_single(
    this: *mut c_void,
    tid: TfClientId,
    riid: *const u8,
    punk: *mut c_void,
) -> HRESULT {
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if punk.is_null() {
        return E_INVALIDARG;
    }
    let mut iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi(punk, riid, &mut iface);
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
    let Some(iid) = read_guid(riid) else {
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
unsafe extern "win64" fn tm_ui_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_ui_elem(this), riid, ppv)
}
unsafe extern "win64" fn tm_ui_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_ui_elem(this))
}
unsafe extern "win64" fn tm_ui_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_ui_elem(this))
}
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
    std::ptr::write_unaligned(pp_element, std::ptr::null_mut());
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
    std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
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
unsafe extern "win64" fn tm_cm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_comp_mgr(this), riid, ppv)
}
unsafe extern "win64" fn tm_cm_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_comp_mgr(this))
}
unsafe extern "win64" fn tm_cm_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_comp_mgr(this))
}
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
unsafe extern "win64" fn tm_ks_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    tm_qi_common(tm_from_keystroke(this), riid, ppv)
}
unsafe extern "win64" fn tm_ks_add_ref(this: *mut c_void) -> u32 {
    tm_add_ref_common(tm_from_keystroke(this))
}
unsafe extern "win64" fn tm_ks_release(this: *mut c_void) -> u32 {
    tm_release_common(tm_from_keystroke(this))
}
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
unsafe extern "win64" fn tm_ks_unadvise_key_event_sink(this: *mut c_void, tid: TfClientId) -> HRESULT {
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
    write_guid(pclsid, &GUID_NULL);
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
    write_guid(pguid, &GUID_NULL);
    let prekey = std::ptr::read_unaligned(pprekey as *const TF_PRESERVEDKEY);
    let obj = &*tm_from_keystroke(this);
    let st = obj.state.lock().unwrap();
    if let Some((&g, _)) = st.preserved_keys.iter().find(|(_, e)| e.prekey == prekey) {
        write_guid(pguid, &g);
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
    let Some(g) = read_guid(rguid) else {
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
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if prekey_ptr.is_null() {
        return E_INVALIDARG;
    }
    let prekey = std::ptr::read_unaligned(prekey_ptr as *const TF_PRESERVEDKEY);
    let desc = read_wide_len(pch_desc, cch_desc);
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
    let Some(g) = read_guid(rguid) else {
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
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc);
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
    if !pbstr_desc.is_null() {
        std::ptr::write_unaligned(pbstr_desc, std::ptr::null_mut());
    }
    let Some(g) = read_guid(rguid) else {
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
unsafe fn get_or_create_thread_mgr() -> *mut ThreadMgrObj {
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
unsafe fn get_existing_thread_mgr() -> Option<*mut ThreadMgrObj> {
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

// ---------------------------------------------------------------------------
// `ITfInputProcessorProfiles` (21 slots) + `ITfInputProcessorProfileMgr` (11 slots)
// + `ITfSource` (5 slots) + `IEnumTfLanguageProfiles` + `IEnumTfInputProcessorProfiles`
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct LanguageProfileRecord {
    clsid: Guid,
    langid: u16,
    catid: Guid,
    guid_profile: Guid,
    desc: String,
    enabled: bool,
    enabled_by_default: bool,
    active: bool,
    hkl: HKL,
    hkl_substitute: HKL,
    profile_type: u32,
}

struct ProfilesState {
    current_langid: u16,
    registered_tips: Vec<Guid>,
    profiles: Vec<LanguageProfileRecord>,
    defaults: HashMap<(u16, Guid), (Guid, Guid)>,
}

impl ProfilesState {
    fn new_default() -> Self {
        let mut defaults = HashMap::new();
        defaults.insert(
            (LANGID_EN_US, GUID_TFCAT_TIP_KEYBOARD),
            (CLSID_EN_US_KEYBOARD_TIP, GUID_EN_US_KEYBOARD_PROFILE),
        );
        Self {
            current_langid: LANGID_EN_US,
            registered_tips: vec![CLSID_EN_US_KEYBOARD_TIP],
            profiles: vec![LanguageProfileRecord {
                clsid: CLSID_EN_US_KEYBOARD_TIP,
                langid: LANGID_EN_US,
                catid: GUID_TFCAT_TIP_KEYBOARD,
                guid_profile: GUID_EN_US_KEYBOARD_PROFILE,
                desc: String::from("English (United States)"),
                enabled: true,
                enabled_by_default: true,
                active: true,
                hkl: HKL_EN_US,
                hkl_substitute: HKL_EN_US,
                profile_type: TF_PROFILETYPE_KEYBOARDLAYOUT,
            }],
            defaults,
        }
    }

    fn language_list(&self) -> Vec<u16> {
        let mut langs: Vec<u16> = Vec::new();
        if !langs.contains(&self.current_langid) {
            langs.push(self.current_langid);
        }
        for p in &self.profiles {
            if !langs.contains(&p.langid) {
                langs.push(p.langid);
            }
        }
        langs
    }
}

static PROFILES_STATE: LazyLock<Mutex<ProfilesState>> =
    LazyLock::new(|| Mutex::new(ProfilesState::new_default()));

// Enumerator for TF_LANGUAGEPROFILE (`IEnumTfLanguageProfiles`: Clone, Next, Reset, Skip)
#[repr(C)]
struct IEnumTfLanguageProfilesVtbl {
    query_interface:
        unsafe extern "win64" fn(*mut EnumLangProfilesObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> u32,
    clone: unsafe extern "win64" fn(*mut EnumLangProfilesObj, *mut *mut c_void) -> HRESULT,
    next: unsafe extern "win64" fn(*mut EnumLangProfilesObj, u32, *mut u8, *mut u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut EnumLangProfilesObj) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut EnumLangProfilesObj, u32) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<IEnumTfLanguageProfilesVtbl>() == 7 * std::mem::size_of::<usize>());

#[repr(C)]
struct EnumLangProfilesObj {
    vtbl: *const IEnumTfLanguageProfilesVtbl,
    refs: AtomicU32,
    state: Mutex<(Vec<TF_LANGUAGEPROFILE>, usize)>,
}

unsafe extern "win64" fn elp_qi(
    this: *mut EnumLangProfilesObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_IENUMTFLANGUAGEPROFILES {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn elp_add_ref(this: *mut EnumLangProfilesObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn elp_release(this: *mut EnumLangProfilesObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn elp_clone(this: *mut EnumLangProfilesObj, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let (items, pos) = {
        let st = obj.state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    std::ptr::write_unaligned(pp_enum, new_enum_lang_profiles(items, pos) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn elp_next(
    this: *mut EnumLangProfilesObj,
    ul_count: u32,
    p_profile: *mut u8,
    pc_fetch: *mut u32,
) -> HRESULT {
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, 0);
    }
    if ul_count > 1 && pc_fetch.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && p_profile.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (ul_count as usize).min(avail);
    let stride = std::mem::size_of::<TF_LANGUAGEPROFILE>();
    for i in 0..n {
        std::ptr::write_unaligned(p_profile.add(i * stride) as *mut TF_LANGUAGEPROFILE, st.0[st.1 + i]);
    }
    st.1 += n;
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, n as u32);
    }
    if n == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn elp_reset(this: *mut EnumLangProfilesObj) -> HRESULT {
    let obj = &*this;
    obj.state.lock().unwrap().1 = 0;
    S_OK
}
unsafe extern "win64" fn elp_skip(this: *mut EnumLangProfilesObj, ul_count: u32) -> HRESULT {
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (ul_count as usize).min(avail);
    st.1 += step;
    if step == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

static IENUM_TF_LANGUAGE_PROFILES_VTBL: IEnumTfLanguageProfilesVtbl = IEnumTfLanguageProfilesVtbl {
    query_interface: elp_qi,
    add_ref: elp_add_ref,
    release: elp_release,
    clone: elp_clone,
    next: elp_next,
    reset: elp_reset,
    skip: elp_skip,
};

fn new_enum_lang_profiles(items: Vec<TF_LANGUAGEPROFILE>, pos: usize) -> *mut EnumLangProfilesObj {
    register_vtable_ptrs(&[
        elp_qi as *const () as usize,
        elp_add_ref as *const () as usize,
        elp_release as *const () as usize,
        elp_clone as *const () as usize,
        elp_next as *const () as usize,
        elp_reset as *const () as usize,
        elp_skip as *const () as usize,
    ]);
    Box::into_raw(Box::new(EnumLangProfilesObj {
        vtbl: &IENUM_TF_LANGUAGE_PROFILES_VTBL,
        refs: AtomicU32::new(1),
        state: Mutex::new((items, pos)),
    }))
}

// Enumerator for TF_INPUTPROCESSORPROFILE (`IEnumTfInputProcessorProfiles`)
#[repr(C)]
struct IEnumTfInputProcessorProfilesVtbl {
    query_interface:
        unsafe extern "win64" fn(*mut EnumIppObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumIppObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumIppObj) -> u32,
    clone: unsafe extern "win64" fn(*mut EnumIppObj, *mut *mut c_void) -> HRESULT,
    next: unsafe extern "win64" fn(*mut EnumIppObj, u32, *mut u8, *mut u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut EnumIppObj) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut EnumIppObj, u32) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<IEnumTfInputProcessorProfilesVtbl>() == 7 * std::mem::size_of::<usize>()
);

#[repr(C)]
struct EnumIppObj {
    vtbl: *const IEnumTfInputProcessorProfilesVtbl,
    refs: AtomicU32,
    state: Mutex<(Vec<TF_INPUTPROCESSORPROFILE>, usize)>,
}

unsafe extern "win64" fn eipp_qi(
    this: *mut EnumIppObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_IENUMTFINPUTPROCESSORPROFILES {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn eipp_add_ref(this: *mut EnumIppObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn eipp_release(this: *mut EnumIppObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn eipp_clone(this: *mut EnumIppObj, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let (items, pos) = {
        let st = obj.state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    std::ptr::write_unaligned(pp_enum, new_enum_ipp(items, pos) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn eipp_next(
    this: *mut EnumIppObj,
    ul_count: u32,
    p_profile: *mut u8,
    pc_fetch: *mut u32,
) -> HRESULT {
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, 0);
    }
    if ul_count > 1 && pc_fetch.is_null() {
        return E_INVALIDARG;
    }
    if ul_count > 0 && p_profile.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (ul_count as usize).min(avail);
    let stride = std::mem::size_of::<TF_INPUTPROCESSORPROFILE>();
    for i in 0..n {
        std::ptr::write_unaligned(
            p_profile.add(i * stride) as *mut TF_INPUTPROCESSORPROFILE,
            st.0[st.1 + i],
        );
    }
    st.1 += n;
    if !pc_fetch.is_null() {
        std::ptr::write_unaligned(pc_fetch, n as u32);
    }
    if n == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn eipp_reset(this: *mut EnumIppObj) -> HRESULT {
    let obj = &*this;
    obj.state.lock().unwrap().1 = 0;
    S_OK
}
unsafe extern "win64" fn eipp_skip(this: *mut EnumIppObj, ul_count: u32) -> HRESULT {
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let step = (ul_count as usize).min(avail);
    st.1 += step;
    if step == ul_count as usize {
        S_OK
    } else {
        S_FALSE
    }
}

static IENUM_TF_IPP_VTBL: IEnumTfInputProcessorProfilesVtbl = IEnumTfInputProcessorProfilesVtbl {
    query_interface: eipp_qi,
    add_ref: eipp_add_ref,
    release: eipp_release,
    clone: eipp_clone,
    next: eipp_next,
    reset: eipp_reset,
    skip: eipp_skip,
};

fn new_enum_ipp(items: Vec<TF_INPUTPROCESSORPROFILE>, pos: usize) -> *mut EnumIppObj {
    register_vtable_ptrs(&[
        eipp_qi as *const () as usize,
        eipp_add_ref as *const () as usize,
        eipp_release as *const () as usize,
        eipp_clone as *const () as usize,
        eipp_next as *const () as usize,
        eipp_reset as *const () as usize,
        eipp_skip as *const () as usize,
    ]);
    Box::into_raw(Box::new(EnumIppObj {
        vtbl: &IENUM_TF_IPP_VTBL,
        refs: AtomicU32::new(1),
        state: Mutex::new((items, pos)),
    }))
}

#[repr(C)]
struct ITfInputProcessorProfilesVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    register: unsafe extern "win64" fn(*mut c_void, *const u8) -> HRESULT,
    unregister: unsafe extern "win64" fn(*mut c_void, *const u8) -> HRESULT,
    add_language_profile: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        u16,
        *const u8,
        *const u16,
        u32,
        *const u16,
        u32,
        u32,
    ) -> HRESULT,
    remove_language_profile: unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8) -> HRESULT,
    enum_input_processor_info: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    get_default_language_profile:
        unsafe extern "win64" fn(*mut c_void, u16, *const u8, *mut u8, *mut u8) -> HRESULT,
    set_default_language_profile:
        unsafe extern "win64" fn(*mut c_void, u16, *const u8, *const u8) -> HRESULT,
    activate_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8) -> HRESULT,
    get_active_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut u16, *mut u8) -> HRESULT,
    get_language_profile_description:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, *mut *mut u16) -> HRESULT,
    get_current_language: unsafe extern "win64" fn(*mut c_void, *mut u16) -> HRESULT,
    change_current_language: unsafe extern "win64" fn(*mut c_void, u16) -> HRESULT,
    get_language_list: unsafe extern "win64" fn(*mut c_void, *mut *mut u16, *mut u32) -> HRESULT,
    enum_language_profiles: unsafe extern "win64" fn(*mut c_void, u16, *mut *mut c_void) -> HRESULT,
    enable_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, BOOL) -> HRESULT,
    is_enabled_language_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, *mut BOOL) -> HRESULT,
    enable_language_profile_by_default:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, BOOL) -> HRESULT,
    substitute_keyboard_layout:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, HKL) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<ITfInputProcessorProfilesVtbl>() == 21 * std::mem::size_of::<usize>()
);

#[repr(C)]
struct ITfInputProcessorProfileMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    activate_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        u32,
    ) -> HRESULT,
    deactivate_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        u32,
    ) -> HRESULT,
    get_profile: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u16,
        *const u8,
        *const u8,
        HKL,
        *mut u8,
    ) -> HRESULT,
    enum_profiles: unsafe extern "win64" fn(*mut c_void, u16, *mut *mut c_void) -> HRESULT,
    release_input_processor: unsafe extern "win64" fn(*mut c_void, *const u8, u32) -> HRESULT,
    register_profile: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        u16,
        *const u8,
        *const u16,
        u32,
        *const u16,
        u32,
        u32,
        HKL,
        u32,
        BOOL,
        u32,
    ) -> HRESULT,
    unregister_profile:
        unsafe extern "win64" fn(*mut c_void, *const u8, u16, *const u8, u32) -> HRESULT,
    get_active_profile: unsafe extern "win64" fn(*mut c_void, *const u8, *mut u8) -> HRESULT,
}
const _: () = assert!(
    std::mem::size_of::<ITfInputProcessorProfileMgrVtbl>() == 11 * std::mem::size_of::<usize>()
);

#[repr(C)]
struct InputProcessorProfilesObj {
    vtbl_profiles: *const ITfInputProcessorProfilesVtbl,
    vtbl_profile_mgr: *const ITfInputProcessorProfileMgrVtbl,
    vtbl_source: *const ITfSourceVtbl,
    refs: AtomicU32,
    sinks: Mutex<SinkTable>,
}

fn ipp_supports_iid(iid: &Guid) -> bool {
    *iid == IID_IUNKNOWN
        || *iid == IID_ITFINPUTPROCESSORPROFILES
        || *iid == IID_ITFINPUTPROCESSORPROFILEMGR
        || *iid == IID_ITFSOURCE
}

unsafe fn ipp_from_mgr(this: *mut c_void) -> *mut InputProcessorProfilesObj {
    (this as *mut u8).sub(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_profile_mgr))
        as *mut InputProcessorProfilesObj
}
unsafe fn ipp_from_source(this: *mut c_void) -> *mut InputProcessorProfilesObj {
    (this as *mut u8).sub(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_source))
        as *mut InputProcessorProfilesObj
}

unsafe fn ipp_qi_common(
    base: *mut InputProcessorProfilesObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let target = if iid == IID_IUNKNOWN || iid == IID_ITFINPUTPROCESSORPROFILES {
        base as *mut c_void
    } else if iid == IID_ITFINPUTPROCESSORPROFILEMGR {
        (base as *mut u8).add(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_profile_mgr))
            as *mut c_void
    } else if iid == IID_ITFSOURCE {
        (base as *mut u8).add(std::mem::offset_of!(InputProcessorProfilesObj, vtbl_source))
            as *mut c_void
    } else {
        return E_NOINTERFACE;
    };
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel);
    std::ptr::write_unaligned(ppv, target);
    S_OK
}
unsafe fn ipp_add_ref_common(base: *mut InputProcessorProfilesObj) -> u32 {
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe fn ipp_release_common(base: *mut InputProcessorProfilesObj) -> u32 {
    let obj = &*base;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        let sinks = obj.sinks.lock().unwrap().drain_all();
        drop(Box::from_raw(base));
        release_all_ptrs(sinks);
    }
    rem
}

fn to_tf_ipp(r: &LanguageProfileRecord) -> TF_INPUTPROCESSORPROFILE {
    let mut flags = 0u32;
    if r.active {
        flags |= TF_IPP_FLAG_ACTIVE;
    }
    if r.enabled {
        flags |= TF_IPP_FLAG_ENABLED;
    }
    TF_INPUTPROCESSORPROFILE {
        dw_profile_type: r.profile_type,
        langid: r.langid,
        clsid: Align4Guid(r.clsid),
        guid_profile: Align4Guid(r.guid_profile),
        catid: Align4Guid(r.catid),
        hkl_substitute: r.hkl_substitute,
        dw_caps: 0,
        hkl: r.hkl,
        dw_flags: flags,
    }
}

unsafe extern "win64" fn ipp_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ipp_qi_common(this as *mut InputProcessorProfilesObj, riid, ppv)
}
unsafe extern "win64" fn ipp_add_ref(this: *mut c_void) -> u32 {
    ipp_add_ref_common(this as *mut InputProcessorProfilesObj)
}
unsafe extern "win64" fn ipp_release(this: *mut c_void) -> u32 {
    ipp_release_common(this as *mut InputProcessorProfilesObj)
}
unsafe extern "win64" fn ipp_register(_this: *mut c_void, rclsid: *const u8) -> HRESULT {
    let Some(clsid) = read_guid(rclsid) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    S_OK
}
unsafe extern "win64" fn ipp_unregister(_this: *mut c_void, rclsid: *const u8) -> HRESULT {
    let Some(clsid) = read_guid(rclsid) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.registered_tips.retain(|g| *g != clsid);
    st.profiles.retain(|p| p.clsid != clsid);
    S_OK
}
unsafe extern "win64" fn ipp_add_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
    _pch_icon_file: *const u16,
    _cch_file: u32,
    _u_icon_index: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc);
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    if let Some(existing) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        existing.desc = desc;
    } else {
        st.profiles.push(LanguageProfileRecord {
            clsid,
            langid,
            catid: GUID_TFCAT_TIP_KEYBOARD,
            guid_profile: gp,
            desc,
            enabled: true,
            enabled_by_default: true,
            active: false,
            hkl: ((langid as usize) << 16) | (langid as usize),
            hkl_substitute: 0,
            profile_type: TF_PROFILETYPE_INPUTPROCESSOR,
        });
    }
    S_OK
}
unsafe extern "win64" fn ipp_remove_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.profiles
        .retain(|p| !(p.clsid == clsid && p.langid == langid && p.guid_profile == gp));
    S_OK
}
unsafe extern "win64" fn ipp_enum_info(_this: *mut c_void, pp_enum: *mut *mut c_void) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let tips = PROFILES_STATE.lock().unwrap().registered_tips.clone();
    std::ptr::write_unaligned(pp_enum, new_enum_guid(tips, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn ipp_get_default_lang_profile(
    _this: *mut c_void,
    langid: u16,
    catid: *const u8,
    pclsid: *mut u8,
    pguid_profile: *mut u8,
) -> HRESULT {
    if !pclsid.is_null() {
        write_guid(pclsid, &GUID_NULL);
    }
    if !pguid_profile.is_null() {
        write_guid(pguid_profile, &GUID_NULL);
    }
    let Some(cat) = read_guid(catid) else {
        return E_INVALIDARG;
    };
    if pclsid.is_null() || pguid_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let target_lang = if langid == 0 { st.current_langid } else { langid };
    if let Some((clsid, gp)) = st.defaults.get(&(target_lang, cat)) {
        write_guid(pclsid, clsid);
        write_guid(pguid_profile, gp);
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn ipp_set_default_lang_profile(
    _this: *mut c_void,
    langid: u16,
    rclsid: *const u8,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.defaults.insert((langid, GUID_TFCAT_TIP_KEYBOARD), (clsid, gp));
    S_OK
}
unsafe extern "win64" fn ipp_activate_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    let mut found = false;
    for p in &mut st.profiles {
        if p.clsid == clsid && p.langid == langid && p.guid_profile == gp {
            p.active = true;
            found = true;
        } else if p.langid == langid {
            p.active = false;
        }
    }
    if found {
        st.current_langid = langid;
        S_OK
    } else {
        E_INVALIDARG
    }
}
unsafe extern "win64" fn ipp_get_active_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    plangid: *mut u16,
    pguid_profile: *mut u8,
) -> HRESULT {
    if !plangid.is_null() {
        std::ptr::write_unaligned(plangid, 0);
    }
    if !pguid_profile.is_null() {
        write_guid(pguid_profile, &GUID_NULL);
    }
    let Some(clsid) = read_guid(rclsid) else {
        return E_INVALIDARG;
    };
    if plangid.is_null() || pguid_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st.profiles.iter().find(|p| p.clsid == clsid && p.active) {
        std::ptr::write_unaligned(plangid, p.langid);
        write_guid(pguid_profile, &p.guid_profile);
        S_OK
    } else {
        S_FALSE
    }
}
unsafe extern "win64" fn ipp_get_lang_profile_desc(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pbstr_profile: *mut *mut u16,
) -> HRESULT {
    if !pbstr_profile.is_null() {
        std::ptr::write_unaligned(pbstr_profile, std::ptr::null_mut());
    }
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    if pbstr_profile.is_null() {
        return E_INVALIDARG;
    }
    let desc = PROFILES_STATE
        .lock()
        .unwrap()
        .profiles
        .iter()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
        .map(|p| p.desc.clone());
    match desc {
        Some(s) => {
            std::ptr::write_unaligned(pbstr_profile, alloc_bstr_str(&s));
            S_OK
        }
        None => E_FAIL,
    }
}
unsafe extern "win64" fn ipp_get_current_language(_this: *mut c_void, plangid: *mut u16) -> HRESULT {
    if plangid.is_null() {
        return E_INVALIDARG;
    }
    let lang = PROFILES_STATE.lock().unwrap().current_langid;
    std::ptr::write_unaligned(plangid, lang);
    S_OK
}
unsafe extern "win64" fn ipp_change_current_language(_this: *mut c_void, langid: u16) -> HRESULT {
    let mut st = PROFILES_STATE.lock().unwrap();
    st.current_langid = langid;
    S_OK
}
unsafe extern "win64" fn ipp_get_language_list(
    _this: *mut c_void,
    pp_lang_id: *mut *mut u16,
    pul_count: *mut u32,
) -> HRESULT {
    if !pp_lang_id.is_null() {
        std::ptr::write_unaligned(pp_lang_id, std::ptr::null_mut());
    }
    if !pul_count.is_null() {
        std::ptr::write_unaligned(pul_count, 0);
    }
    if pp_lang_id.is_null() || pul_count.is_null() {
        return E_INVALIDARG;
    }
    let langs = PROFILES_STATE.lock().unwrap().language_list();
    let bytes = langs.len() * std::mem::size_of::<u16>();
    let buf = super::ole32::CoTaskMemAlloc(bytes.max(2)) as *mut u16;
    if buf.is_null() {
        return E_OUTOFMEMORY;
    }
    for (i, &lang) in langs.iter().enumerate() {
        std::ptr::write_unaligned(buf.add(i), lang);
    }
    std::ptr::write_unaligned(pp_lang_id, buf);
    std::ptr::write_unaligned(pul_count, langs.len() as u32);
    S_OK
}
unsafe extern "win64" fn ipp_enum_lang_profiles(
    _this: *mut c_void,
    langid: u16,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let items: Vec<TF_LANGUAGEPROFILE> = st
        .profiles
        .iter()
        .filter(|p| langid == 0 || p.langid == langid)
        .map(|p| TF_LANGUAGEPROFILE {
            clsid: Align4Guid(p.clsid),
            langid: p.langid,
            catid: Align4Guid(p.catid),
            f_active: if p.active { TRUE } else { FALSE },
            guid_profile: Align4Guid(p.guid_profile),
        })
        .collect();
    std::ptr::write_unaligned(pp_enum, new_enum_lang_profiles(items, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn ipp_enable_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    f_enable: BOOL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.enabled = f_enable != 0;
        S_OK
    } else {
        E_INVALIDARG
    }
}
unsafe extern "win64" fn ipp_is_enabled_lang_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pf_enable: *mut BOOL,
) -> HRESULT {
    if !pf_enable.is_null() {
        std::ptr::write_unaligned(pf_enable, FALSE);
    }
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    if pf_enable.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let enabled = st
        .profiles
        .iter()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
        .is_some_and(|p| p.enabled);
    std::ptr::write_unaligned(pf_enable, if enabled { TRUE } else { FALSE });
    S_OK
}
unsafe extern "win64" fn ipp_enable_lang_profile_by_default(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    f_enable: BOOL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.enabled_by_default = f_enable != 0;
        S_OK
    } else {
        E_INVALIDARG
    }
}
unsafe extern "win64" fn ipp_substitute_keyboard_layout(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    hkl: HKL,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    if let Some(p) = st
        .profiles
        .iter_mut()
        .find(|p| p.clsid == clsid && p.langid == langid && p.guid_profile == gp)
    {
        p.hkl_substitute = hkl;
        S_OK
    } else {
        E_INVALIDARG
    }
}

// ITfInputProcessorProfileMgr thunks
unsafe extern "win64" fn ippm_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ipp_qi_common(ipp_from_mgr(this), riid, ppv)
}
unsafe extern "win64" fn ippm_add_ref(this: *mut c_void) -> u32 {
    ipp_add_ref_common(ipp_from_mgr(this))
}
unsafe extern "win64" fn ippm_release(this: *mut c_void) -> u32 {
    ipp_release_common(ipp_from_mgr(this))
}
unsafe extern "win64" fn ippm_activate_profile(
    _this: *mut c_void,
    _dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    _hkl: HKL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(c), Some(gp)) = (read_guid(clsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    for p in &mut st.profiles {
        p.active = p.langid == langid && (p.clsid == c && p.guid_profile == gp);
    }
    st.current_langid = langid;
    S_OK
}
unsafe extern "win64" fn ippm_deactivate_profile(
    _this: *mut c_void,
    _dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    _hkl: HKL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(c), Some(gp)) = (read_guid(clsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    for p in &mut st.profiles {
        if p.langid == langid && p.clsid == c && p.guid_profile == gp {
            p.active = false;
        }
    }
    S_OK
}
unsafe extern "win64" fn ippm_get_profile(
    _this: *mut c_void,
    dw_profile_type: u32,
    langid: u16,
    clsid: *const u8,
    guid_profile: *const u8,
    hkl: HKL,
    p_profile: *mut u8,
) -> HRESULT {
    if p_profile.is_null() {
        return E_INVALIDARG;
    }
    let c = read_guid(clsid).unwrap_or(GUID_NULL);
    let gp = read_guid(guid_profile).unwrap_or(GUID_NULL);
    let st = PROFILES_STATE.lock().unwrap();
    let found = st.profiles.iter().find(|p| {
        (dw_profile_type == 0 || p.profile_type == dw_profile_type)
            && (langid == 0 || p.langid == langid)
            && (c == GUID_NULL || p.clsid == c)
            && (gp == GUID_NULL || p.guid_profile == gp)
            && (hkl == 0 || p.hkl == hkl)
    });
    match found {
        Some(r) => {
            std::ptr::write_unaligned(p_profile as *mut TF_INPUTPROCESSORPROFILE, to_tf_ipp(r));
            S_OK
        }
        None => E_FAIL,
    }
}
unsafe extern "win64" fn ippm_enum_profiles(
    _this: *mut c_void,
    langid: u16,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let items: Vec<TF_INPUTPROCESSORPROFILE> = st
        .profiles
        .iter()
        .filter(|p| langid == 0 || p.langid == langid)
        .map(to_tf_ipp)
        .collect();
    std::ptr::write_unaligned(pp_enum, new_enum_ipp(items, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn ippm_release_input_processor(
    _this: *mut c_void,
    _rclsid: *const u8,
    _dw_flags: u32,
) -> HRESULT {
    S_OK
}
unsafe extern "win64" fn ippm_register_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    pch_desc: *const u16,
    cch_desc: u32,
    _pch_icon_file: *const u16,
    _cch_file: u32,
    _u_icon_index: u32,
    hkl_substitute: HKL,
    _dw_preferred_layout: u32,
    b_enabled_by_default: BOOL,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch_desc);
    let mut st = PROFILES_STATE.lock().unwrap();
    if !st.registered_tips.contains(&clsid) {
        st.registered_tips.push(clsid);
    }
    st.profiles.push(LanguageProfileRecord {
        clsid,
        langid,
        catid: GUID_TFCAT_TIP_KEYBOARD,
        guid_profile: gp,
        desc,
        enabled: b_enabled_by_default != 0,
        enabled_by_default: b_enabled_by_default != 0,
        active: false,
        hkl: ((langid as usize) << 16) | (langid as usize),
        hkl_substitute,
        profile_type: TF_PROFILETYPE_INPUTPROCESSOR,
    });
    S_OK
}
unsafe extern "win64" fn ippm_unregister_profile(
    _this: *mut c_void,
    rclsid: *const u8,
    langid: u16,
    guid_profile: *const u8,
    _dw_flags: u32,
) -> HRESULT {
    let (Some(clsid), Some(gp)) = (read_guid(rclsid), read_guid(guid_profile)) else {
        return E_INVALIDARG;
    };
    let mut st = PROFILES_STATE.lock().unwrap();
    st.profiles
        .retain(|p| !(p.clsid == clsid && p.langid == langid && p.guid_profile == gp));
    S_OK
}
unsafe extern "win64" fn ippm_get_active_profile(
    _this: *mut c_void,
    catid: *const u8,
    p_profile: *mut u8,
) -> HRESULT {
    let Some(cat) = read_guid(catid) else {
        return E_INVALIDARG;
    };
    if p_profile.is_null() {
        return E_INVALIDARG;
    }
    let st = PROFILES_STATE.lock().unwrap();
    let found = st
        .profiles
        .iter()
        .find(|p| p.catid == cat && p.active)
        .or_else(|| st.profiles.iter().find(|p| p.catid == cat));
    match found {
        Some(r) => {
            std::ptr::write_unaligned(p_profile as *mut TF_INPUTPROCESSORPROFILE, to_tf_ipp(r));
            S_OK
        }
        None => S_FALSE,
    }
}

// ITfSource thunks on InputProcessorProfilesObj
unsafe extern "win64" fn ipp_src_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    ipp_qi_common(ipp_from_source(this), riid, ppv)
}
unsafe extern "win64" fn ipp_src_add_ref(this: *mut c_void) -> u32 {
    ipp_add_ref_common(ipp_from_source(this))
}
unsafe extern "win64" fn ipp_src_release(this: *mut c_void) -> u32 {
    ipp_release_common(ipp_from_source(this))
}
unsafe extern "win64" fn ipp_src_advise(
    this: *mut c_void,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    let obj = &*ipp_from_source(this);
    sink_advise(&obj.sinks, riid, punk, pdw_cookie)
}
unsafe extern "win64" fn ipp_src_unadvise(this: *mut c_void, cookie: u32) -> HRESULT {
    let obj = &*ipp_from_source(this);
    sink_unadvise(&obj.sinks, cookie)
}

static ITF_IPP_VTBL: ITfInputProcessorProfilesVtbl = ITfInputProcessorProfilesVtbl {
    query_interface: ipp_qi,
    add_ref: ipp_add_ref,
    release: ipp_release,
    register: ipp_register,
    unregister: ipp_unregister,
    add_language_profile: ipp_add_lang_profile,
    remove_language_profile: ipp_remove_lang_profile,
    enum_input_processor_info: ipp_enum_info,
    get_default_language_profile: ipp_get_default_lang_profile,
    set_default_language_profile: ipp_set_default_lang_profile,
    activate_language_profile: ipp_activate_lang_profile,
    get_active_language_profile: ipp_get_active_lang_profile,
    get_language_profile_description: ipp_get_lang_profile_desc,
    get_current_language: ipp_get_current_language,
    change_current_language: ipp_change_current_language,
    get_language_list: ipp_get_language_list,
    enum_language_profiles: ipp_enum_lang_profiles,
    enable_language_profile: ipp_enable_lang_profile,
    is_enabled_language_profile: ipp_is_enabled_lang_profile,
    enable_language_profile_by_default: ipp_enable_lang_profile_by_default,
    substitute_keyboard_layout: ipp_substitute_keyboard_layout,
};

static ITF_IPPM_VTBL: ITfInputProcessorProfileMgrVtbl = ITfInputProcessorProfileMgrVtbl {
    query_interface: ippm_qi,
    add_ref: ippm_add_ref,
    release: ippm_release,
    activate_profile: ippm_activate_profile,
    deactivate_profile: ippm_deactivate_profile,
    get_profile: ippm_get_profile,
    enum_profiles: ippm_enum_profiles,
    release_input_processor: ippm_release_input_processor,
    register_profile: ippm_register_profile,
    unregister_profile: ippm_unregister_profile,
    get_active_profile: ippm_get_active_profile,
};

static ITF_IPP_SRC_VTBL: ITfSourceVtbl = ITfSourceVtbl {
    query_interface: ipp_src_qi,
    add_ref: ipp_src_add_ref,
    release: ipp_src_release,
    advise_sink: ipp_src_advise,
    unadvise_sink: ipp_src_unadvise,
};

fn new_input_processor_profiles_obj() -> *mut InputProcessorProfilesObj {
    register_vtable_ptrs(&[
        ipp_qi as *const () as usize,
        ipp_add_ref as *const () as usize,
        ipp_release as *const () as usize,
        ipp_register as *const () as usize,
        ipp_unregister as *const () as usize,
        ipp_add_lang_profile as *const () as usize,
        ipp_remove_lang_profile as *const () as usize,
        ipp_enum_info as *const () as usize,
        ipp_get_default_lang_profile as *const () as usize,
        ipp_set_default_lang_profile as *const () as usize,
        ipp_activate_lang_profile as *const () as usize,
        ipp_get_active_lang_profile as *const () as usize,
        ipp_get_lang_profile_desc as *const () as usize,
        ipp_get_current_language as *const () as usize,
        ipp_change_current_language as *const () as usize,
        ipp_get_language_list as *const () as usize,
        ipp_enum_lang_profiles as *const () as usize,
        ipp_enable_lang_profile as *const () as usize,
        ipp_is_enabled_lang_profile as *const () as usize,
        ipp_enable_lang_profile_by_default as *const () as usize,
        ipp_substitute_keyboard_layout as *const () as usize,
        ippm_qi as *const () as usize,
        ippm_add_ref as *const () as usize,
        ippm_release as *const () as usize,
        ippm_activate_profile as *const () as usize,
        ippm_deactivate_profile as *const () as usize,
        ippm_get_profile as *const () as usize,
        ippm_enum_profiles as *const () as usize,
        ippm_release_input_processor as *const () as usize,
        ippm_register_profile as *const () as usize,
        ippm_unregister_profile as *const () as usize,
        ippm_get_active_profile as *const () as usize,
        ipp_src_qi as *const () as usize,
        ipp_src_add_ref as *const () as usize,
        ipp_src_release as *const () as usize,
        ipp_src_advise as *const () as usize,
        ipp_src_unadvise as *const () as usize,
    ]);
    Box::into_raw(Box::new(InputProcessorProfilesObj {
        vtbl_profiles: &ITF_IPP_VTBL,
        vtbl_profile_mgr: &ITF_IPPM_VTBL,
        vtbl_source: &ITF_IPP_SRC_VTBL,
        refs: AtomicU32::new(1),
        sinks: Mutex::new(SinkTable::default()),
    }))
}

// ---------------------------------------------------------------------------
// `ITfCategoryMgr` (17 slots)
// ---------------------------------------------------------------------------

struct CategoryState {
    next_atom: TfGuidAtom,
    guid_to_atom: HashMap<Guid, TfGuidAtom>,
    atom_to_guid: HashMap<TfGuidAtom, Guid>,
    /// `(clsid, catid, rguid)` registrations.
    categories: Vec<(Guid, Guid, Guid)>,
    descriptions: HashMap<Guid, String>,
    dwords: HashMap<Guid, u32>,
}

impl Default for CategoryState {
    fn default() -> Self {
        Self {
            next_atom: 1,
            guid_to_atom: HashMap::new(),
            atom_to_guid: HashMap::new(),
            categories: vec![(CLSID_EN_US_KEYBOARD_TIP, GUID_TFCAT_TIP_KEYBOARD, CLSID_EN_US_KEYBOARD_TIP)],
            descriptions: HashMap::new(),
            dwords: HashMap::new(),
        }
    }
}

static CATEGORY_STATE: LazyLock<Mutex<CategoryState>> =
    LazyLock::new(|| Mutex::new(CategoryState::default()));

#[repr(C)]
struct ITfCategoryMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut CategoryMgrObj) -> u32,
    release: unsafe extern "win64" fn(*mut CategoryMgrObj) -> u32,
    register_category: unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, *const u8) -> HRESULT,
    unregister_category:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, *const u8) -> HRESULT,
    enum_categories_in_item:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    enum_items_in_category:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    find_closest_category: unsafe extern "win64" fn(
        *mut CategoryMgrObj,
        *const u8,
        *mut u8,
        *const *const u8,
        u32,
    ) -> HRESULT,
    register_guid_description:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, *const u16, u32) -> HRESULT,
    unregister_guid_description:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8) -> HRESULT,
    get_guid_description:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut *mut u16) -> HRESULT,
    register_guid_dword:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8, u32) -> HRESULT,
    unregister_guid_dword:
        unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *const u8) -> HRESULT,
    get_guid_dword: unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut u32) -> HRESULT,
    register_guid: unsafe extern "win64" fn(*mut CategoryMgrObj, *const u8, *mut TfGuidAtom) -> HRESULT,
    get_guid: unsafe extern "win64" fn(*mut CategoryMgrObj, TfGuidAtom, *mut u8) -> HRESULT,
    is_equal_tf_guid_atom:
        unsafe extern "win64" fn(*mut CategoryMgrObj, TfGuidAtom, *const u8, *mut BOOL) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfCategoryMgrVtbl>() == 17 * std::mem::size_of::<usize>());

#[repr(C)]
struct CategoryMgrObj {
    vtbl: *const ITfCategoryMgrVtbl,
    refs: AtomicU32,
}

unsafe extern "win64" fn cat_qi(
    this: *mut CategoryMgrObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFCATEGORYMGR {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn cat_add_ref(this: *mut CategoryMgrObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn cat_release(this: *mut CategoryMgrObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn cat_register_category(
    _this: *mut CategoryMgrObj,
    rclsid: *const u8,
    rcatid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let (Some(c), Some(cat), Some(g)) = (read_guid(rclsid), read_guid(rcatid), read_guid(rguid)) else {
        return E_INVALIDARG;
    };
    let mut st = CATEGORY_STATE.lock().unwrap();
    if !st.categories.contains(&(c, cat, g)) {
        st.categories.push((c, cat, g));
    }
    S_OK
}
unsafe extern "win64" fn cat_unregister_category(
    _this: *mut CategoryMgrObj,
    rclsid: *const u8,
    rcatid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let (Some(c), Some(cat), Some(g)) = (read_guid(rclsid), read_guid(rcatid), read_guid(rguid)) else {
        return E_INVALIDARG;
    };
    let mut st = CATEGORY_STATE.lock().unwrap();
    st.categories.retain(|e| *e != (c, cat, g));
    S_OK
}
unsafe extern "win64" fn cat_enum_categories_in_item(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if !pp_enum.is_null() {
        std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    let mut cats: Vec<Guid> = Vec::new();
    for &(_, cat, item) in &st.categories {
        if item == g && !cats.contains(&cat) {
            cats.push(cat);
        }
    }
    std::ptr::write_unaligned(pp_enum, new_enum_guid(cats, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn cat_enum_items_in_category(
    _this: *mut CategoryMgrObj,
    rcatid: *const u8,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if !pp_enum.is_null() {
        std::ptr::write_unaligned(pp_enum, std::ptr::null_mut());
    }
    let Some(cat) = read_guid(rcatid) else {
        return E_INVALIDARG;
    };
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    let mut items: Vec<Guid> = Vec::new();
    for &(_, c, item) in &st.categories {
        if c == cat && !items.contains(&item) {
            items.push(item);
        }
    }
    std::ptr::write_unaligned(pp_enum, new_enum_guid(items, 0) as *mut c_void);
    S_OK
}
unsafe extern "win64" fn cat_find_closest_category(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pcatid: *mut u8,
    ppcatid_list: *const *const u8,
    ul_count: u32,
) -> HRESULT {
    if !pcatid.is_null() {
        write_guid(pcatid, &GUID_NULL);
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pcatid.is_null() || (ul_count > 0 && ppcatid_list.is_null()) {
        return E_INVALIDARG;
    }
    let st = CATEGORY_STATE.lock().unwrap();
    if ul_count == 0 {
        if let Some(&(_, cat, _)) = st.categories.iter().find(|&&(_, _, item)| item == g) {
            write_guid(pcatid, &cat);
            return S_OK;
        }
        return S_FALSE;
    }
    for i in 0..ul_count as usize {
        let cand_ptr = std::ptr::read_unaligned(ppcatid_list.add(i));
        if let Some(cand) = read_guid(cand_ptr) {
            if st.categories.iter().any(|&(_, cat, item)| item == g && cat == cand) {
                write_guid(pcatid, &cand);
                return S_OK;
            }
        }
    }
    S_FALSE
}
unsafe extern "win64" fn cat_register_guid_desc(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
    pch_desc: *const u16,
    cch: u32,
) -> HRESULT {
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    let desc = read_wide_len(pch_desc, cch);
    CATEGORY_STATE.lock().unwrap().descriptions.insert(g, desc);
    S_OK
}
unsafe extern "win64" fn cat_unregister_guid_desc(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().descriptions.remove(&g);
    S_OK
}
unsafe extern "win64" fn cat_get_guid_desc(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pbstr_desc: *mut *mut u16,
) -> HRESULT {
    if !pbstr_desc.is_null() {
        std::ptr::write_unaligned(pbstr_desc, std::ptr::null_mut());
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pbstr_desc.is_null() {
        return E_INVALIDARG;
    }
    let found = CATEGORY_STATE.lock().unwrap().descriptions.get(&g).cloned();
    match found {
        Some(s) => {
            std::ptr::write_unaligned(pbstr_desc, alloc_bstr_str(&s));
            S_OK
        }
        None => E_FAIL,
    }
}
unsafe extern "win64" fn cat_register_guid_dword(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
    dw: u32,
) -> HRESULT {
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().dwords.insert(g, dw);
    S_OK
}
unsafe extern "win64" fn cat_unregister_guid_dword(
    _this: *mut CategoryMgrObj,
    _rclsid: *const u8,
    rguid: *const u8,
) -> HRESULT {
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    CATEGORY_STATE.lock().unwrap().dwords.remove(&g);
    S_OK
}
unsafe extern "win64" fn cat_get_guid_dword(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pdw: *mut u32,
) -> HRESULT {
    if !pdw.is_null() {
        std::ptr::write_unaligned(pdw, 0);
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pdw.is_null() {
        return E_INVALIDARG;
    }
    match CATEGORY_STATE.lock().unwrap().dwords.get(&g).copied() {
        Some(v) => {
            std::ptr::write_unaligned(pdw, v);
            S_OK
        }
        None => S_FALSE,
    }
}
unsafe extern "win64" fn cat_register_guid(
    _this: *mut CategoryMgrObj,
    rguid: *const u8,
    pguidatom: *mut TfGuidAtom,
) -> HRESULT {
    if !pguidatom.is_null() {
        std::ptr::write_unaligned(pguidatom, TF_INVALID_GUIDATOM);
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pguidatom.is_null() {
        return E_INVALIDARG;
    }
    let mut st = CATEGORY_STATE.lock().unwrap();
    let atom = if let Some(&existing) = st.guid_to_atom.get(&g) {
        existing
    } else {
        let mut a = st.next_atom;
        while a == TF_INVALID_GUIDATOM || st.atom_to_guid.contains_key(&a) {
            a = a.wrapping_add(1);
        }
        st.next_atom = a.wrapping_add(1);
        st.guid_to_atom.insert(g, a);
        st.atom_to_guid.insert(a, g);
        a
    };
    std::ptr::write_unaligned(pguidatom, atom);
    S_OK
}
unsafe extern "win64" fn cat_get_guid(
    _this: *mut CategoryMgrObj,
    guidatom: TfGuidAtom,
    pguid: *mut u8,
) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    write_guid(pguid, &GUID_NULL);
    match CATEGORY_STATE.lock().unwrap().atom_to_guid.get(&guidatom).copied() {
        Some(g) => {
            write_guid(pguid, &g);
            S_OK
        }
        None => S_FALSE,
    }
}
unsafe extern "win64" fn cat_is_equal_atom(
    _this: *mut CategoryMgrObj,
    guidatom: TfGuidAtom,
    rguid: *const u8,
    pf_equal: *mut BOOL,
) -> HRESULT {
    if !pf_equal.is_null() {
        std::ptr::write_unaligned(pf_equal, FALSE);
    }
    let Some(g) = read_guid(rguid) else {
        return E_INVALIDARG;
    };
    if pf_equal.is_null() {
        return E_INVALIDARG;
    }
    let eq = CATEGORY_STATE
        .lock()
        .unwrap()
        .atom_to_guid
        .get(&guidatom)
        .is_some_and(|found| *found == g);
    std::ptr::write_unaligned(pf_equal, if eq { TRUE } else { FALSE });
    S_OK
}

static ITF_CATEGORY_MGR_VTBL: ITfCategoryMgrVtbl = ITfCategoryMgrVtbl {
    query_interface: cat_qi,
    add_ref: cat_add_ref,
    release: cat_release,
    register_category: cat_register_category,
    unregister_category: cat_unregister_category,
    enum_categories_in_item: cat_enum_categories_in_item,
    enum_items_in_category: cat_enum_items_in_category,
    find_closest_category: cat_find_closest_category,
    register_guid_description: cat_register_guid_desc,
    unregister_guid_description: cat_unregister_guid_desc,
    get_guid_description: cat_get_guid_desc,
    register_guid_dword: cat_register_guid_dword,
    unregister_guid_dword: cat_unregister_guid_dword,
    get_guid_dword: cat_get_guid_dword,
    register_guid: cat_register_guid,
    get_guid: cat_get_guid,
    is_equal_tf_guid_atom: cat_is_equal_atom,
};

fn new_category_mgr_obj() -> *mut CategoryMgrObj {
    register_vtable_ptrs(&[
        cat_qi as *const () as usize,
        cat_add_ref as *const () as usize,
        cat_release as *const () as usize,
        cat_register_category as *const () as usize,
        cat_unregister_category as *const () as usize,
        cat_enum_categories_in_item as *const () as usize,
        cat_enum_items_in_category as *const () as usize,
        cat_find_closest_category as *const () as usize,
        cat_register_guid_desc as *const () as usize,
        cat_unregister_guid_desc as *const () as usize,
        cat_get_guid_desc as *const () as usize,
        cat_register_guid_dword as *const () as usize,
        cat_unregister_guid_dword as *const () as usize,
        cat_get_guid_dword as *const () as usize,
        cat_register_guid as *const () as usize,
        cat_get_guid as *const () as usize,
        cat_is_equal_atom as *const () as usize,
    ]);
    Box::into_raw(Box::new(CategoryMgrObj {
        vtbl: &ITF_CATEGORY_MGR_VTBL,
        refs: AtomicU32::new(1),
    }))
}

// ---------------------------------------------------------------------------
// `ITfDisplayAttributeMgr` (6 slots) & `ITfDisplayAttributeInfo` (8 slots)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct DisplayAttrEntry {
    guid: Guid,
    owner_clsid: Guid,
    desc: String,
    default_attr: TF_DISPLAYATTRIBUTE,
    current_attr: TF_DISPLAYATTRIBUTE,
}

struct DisplayAttrState {
    entries: Vec<DisplayAttrEntry>,
}

impl DisplayAttrState {
    fn new_default() -> Self {
        let input = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            cr_bk: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            ls_style: TF_LS_DOT,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_INPUT,
        };
        let target_conv = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_SYSCOLOR, val: 14 }, // COLOR_HIGHLIGHTTEXT
            cr_bk: TF_DA_COLOR { ty: TF_CT_SYSCOLOR, val: 13 },   // COLOR_HIGHLIGHT
            ls_style: TF_LS_NONE,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_TARGET_CONVERTED,
        };
        let converted = TF_DISPLAYATTRIBUTE {
            cr_text: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            cr_bk: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            ls_style: TF_LS_SOLID,
            f_bold_line: FALSE,
            cr_line: TF_DA_COLOR { ty: TF_CT_NONE, val: 0 },
            b_attr: TF_ATTR_CONVERTED,
        };
        Self {
            entries: vec![
                DisplayAttrEntry {
                    guid: GUID_DA_INPUT,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Input"),
                    default_attr: input,
                    current_attr: input,
                },
                DisplayAttrEntry {
                    guid: GUID_DA_TARGET_CONVERTED,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Target Converted"),
                    default_attr: target_conv,
                    current_attr: target_conv,
                },
                DisplayAttrEntry {
                    guid: GUID_DA_CONVERTED,
                    owner_clsid: CLSID_EN_US_KEYBOARD_TIP,
                    desc: String::from("Converted"),
                    default_attr: converted,
                    current_attr: converted,
                },
            ],
        }
    }
}

static DISPLAY_ATTR_STATE: LazyLock<Mutex<DisplayAttrState>> =
    LazyLock::new(|| Mutex::new(DisplayAttrState::new_default()));

#[repr(C)]
struct ITfDisplayAttributeInfoVtbl {
    query_interface:
        unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> u32,
    release: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> u32,
    get_guid: unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut u8) -> HRESULT,
    get_description: unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut *mut u16) -> HRESULT,
    get_attribute_info: unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *mut u8) -> HRESULT,
    set_attribute_info: unsafe extern "win64" fn(*mut DisplayAttrInfoObj, *const u8) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut DisplayAttrInfoObj) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfDisplayAttributeInfoVtbl>() == 8 * std::mem::size_of::<usize>());

#[repr(C)]
struct DisplayAttrInfoObj {
    vtbl: *const ITfDisplayAttributeInfoVtbl,
    refs: AtomicU32,
    guid: Guid,
}

unsafe extern "win64" fn dai_qi(
    this: *mut DisplayAttrInfoObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFDISPLAYATTRIBUTEINFO {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn dai_add_ref(this: *mut DisplayAttrInfoObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn dai_release(this: *mut DisplayAttrInfoObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn dai_get_guid(this: *mut DisplayAttrInfoObj, pguid: *mut u8) -> HRESULT {
    if pguid.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    write_guid(pguid, &obj.guid);
    S_OK
}
unsafe extern "win64" fn dai_get_desc(this: *mut DisplayAttrInfoObj, pbstr: *mut *mut u16) -> HRESULT {
    if pbstr.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pbstr, std::ptr::null_mut());
    let obj = &*this;
    let st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter().find(|e| e.guid == obj.guid) {
        std::ptr::write_unaligned(pbstr, alloc_bstr_str(&e.desc));
        S_OK
    } else {
        E_FAIL
    }
}
unsafe extern "win64" fn dai_get_attr(this: *mut DisplayAttrInfoObj, pda: *mut u8) -> HRESULT {
    if pda.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter().find(|e| e.guid == obj.guid) {
        std::ptr::write_unaligned(pda as *mut TF_DISPLAYATTRIBUTE, e.current_attr);
        S_OK
    } else {
        E_FAIL
    }
}
unsafe extern "win64" fn dai_set_attr(this: *mut DisplayAttrInfoObj, pda: *const u8) -> HRESULT {
    if pda.is_null() {
        return E_INVALIDARG;
    }
    let new_attr = std::ptr::read_unaligned(pda as *const TF_DISPLAYATTRIBUTE);
    let obj = &*this;
    let mut st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter_mut().find(|e| e.guid == obj.guid) {
        e.current_attr = new_attr;
        S_OK
    } else {
        E_FAIL
    }
}
unsafe extern "win64" fn dai_reset(this: *mut DisplayAttrInfoObj) -> HRESULT {
    let obj = &*this;
    let mut st = DISPLAY_ATTR_STATE.lock().unwrap();
    if let Some(e) = st.entries.iter_mut().find(|e| e.guid == obj.guid) {
        e.current_attr = e.default_attr;
        S_OK
    } else {
        E_FAIL
    }
}

static ITF_DISPLAY_ATTR_INFO_VTBL: ITfDisplayAttributeInfoVtbl = ITfDisplayAttributeInfoVtbl {
    query_interface: dai_qi,
    add_ref: dai_add_ref,
    release: dai_release,
    get_guid: dai_get_guid,
    get_description: dai_get_desc,
    get_attribute_info: dai_get_attr,
    set_attribute_info: dai_set_attr,
    reset: dai_reset,
};

fn new_display_attr_info_obj(guid: Guid) -> *mut DisplayAttrInfoObj {
    register_vtable_ptrs(&[
        dai_qi as *const () as usize,
        dai_add_ref as *const () as usize,
        dai_release as *const () as usize,
        dai_get_guid as *const () as usize,
        dai_get_desc as *const () as usize,
        dai_get_attr as *const () as usize,
        dai_set_attr as *const () as usize,
        dai_reset as *const () as usize,
    ]);
    Box::into_raw(Box::new(DisplayAttrInfoObj {
        vtbl: &ITF_DISPLAY_ATTR_INFO_VTBL,
        refs: AtomicU32::new(1),
        guid,
    }))
}

#[repr(C)]
struct ITfDisplayAttributeMgrVtbl {
    query_interface:
        unsafe extern "win64" fn(*mut DisplayAttrMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> u32,
    release: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> u32,
    on_update_info: unsafe extern "win64" fn(*mut DisplayAttrMgrObj) -> HRESULT,
    enum_display_attribute_info:
        unsafe extern "win64" fn(*mut DisplayAttrMgrObj, *mut *mut c_void) -> HRESULT,
    get_display_attribute_info:
        unsafe extern "win64" fn(*mut DisplayAttrMgrObj, *const u8, *mut *mut c_void, *mut u8) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfDisplayAttributeMgrVtbl>() == 6 * std::mem::size_of::<usize>());

#[repr(C)]
struct DisplayAttrMgrObj {
    vtbl: *const ITfDisplayAttributeMgrVtbl,
    refs: AtomicU32,
}

unsafe extern "win64" fn dam_qi(
    this: *mut DisplayAttrMgrObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFDISPLAYATTRIBUTEMGR {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn dam_add_ref(this: *mut DisplayAttrMgrObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn dam_release(this: *mut DisplayAttrMgrObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}
unsafe extern "win64" fn dam_on_update_info(_this: *mut DisplayAttrMgrObj) -> HRESULT {
    S_OK
}
unsafe extern "win64" fn dam_enum_info(
    _this: *mut DisplayAttrMgrObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    let guids: Vec<Guid> = DISPLAY_ATTR_STATE
        .lock()
        .unwrap()
        .entries
        .iter()
        .map(|e| e.guid)
        .collect();
    let ptrs: Vec<usize> = guids.into_iter().map(|g| new_display_attr_info_obj(g) as usize).collect();
    let en = new_enum_tf_com(IID_IENUMTFDISPLAYATTRIBUTEINFO, ptrs, 0);
    std::ptr::write_unaligned(pp_enum, en as *mut c_void);
    S_OK
}
unsafe extern "win64" fn dam_get_info(
    _this: *mut DisplayAttrMgrObj,
    guid: *const u8,
    pp_info: *mut *mut c_void,
    pclsid_owner: *mut u8,
) -> HRESULT {
    if !pp_info.is_null() {
        std::ptr::write_unaligned(pp_info, std::ptr::null_mut());
    }
    if !pclsid_owner.is_null() {
        write_guid(pclsid_owner, &GUID_NULL);
    }
    let Some(g) = read_guid(guid) else {
        return E_INVALIDARG;
    };
    if pp_info.is_null() {
        return E_INVALIDARG;
    }
    let owner = DISPLAY_ATTR_STATE
        .lock()
        .unwrap()
        .entries
        .iter()
        .find(|e| e.guid == g)
        .map(|e| e.owner_clsid);
    match owner {
        Some(clsid) => {
            let info = new_display_attr_info_obj(g);
            std::ptr::write_unaligned(pp_info, info as *mut c_void);
            if !pclsid_owner.is_null() {
                write_guid(pclsid_owner, &clsid);
            }
            S_OK
        }
        None => E_INVALIDARG,
    }
}

static ITF_DISPLAY_ATTR_MGR_VTBL: ITfDisplayAttributeMgrVtbl = ITfDisplayAttributeMgrVtbl {
    query_interface: dam_qi,
    add_ref: dam_add_ref,
    release: dam_release,
    on_update_info: dam_on_update_info,
    enum_display_attribute_info: dam_enum_info,
    get_display_attribute_info: dam_get_info,
};

fn new_display_attr_mgr_obj() -> *mut DisplayAttrMgrObj {
    register_vtable_ptrs(&[
        dam_qi as *const () as usize,
        dam_add_ref as *const () as usize,
        dam_release as *const () as usize,
        dam_on_update_info as *const () as usize,
        dam_enum_info as *const () as usize,
        dam_get_info as *const () as usize,
    ]);
    Box::into_raw(Box::new(DisplayAttrMgrObj {
        vtbl: &ITF_DISPLAY_ATTR_MGR_VTBL,
        refs: AtomicU32::new(1),
    }))
}

// ---------------------------------------------------------------------------
// `ITfLangBarMgr` (12 slots — ctfutb.h)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ITfLangBarMgrVtbl {
    query_interface: unsafe extern "win64" fn(*mut LangBarMgrObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut LangBarMgrObj) -> u32,
    release: unsafe extern "win64" fn(*mut LangBarMgrObj) -> u32,
    advise_event_sink:
        unsafe extern "win64" fn(*mut LangBarMgrObj, *mut c_void, HWND, u32, *mut u32) -> HRESULT,
    unadvise_event_sink: unsafe extern "win64" fn(*mut LangBarMgrObj, u32) -> HRESULT,
    get_thread_marshal_interface:
        unsafe extern "win64" fn(*mut LangBarMgrObj, u32, u32, *const u8, *mut *mut c_void) -> HRESULT,
    get_thread_lang_bar_item_mgr:
        unsafe extern "win64" fn(*mut LangBarMgrObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    get_input_processor_profiles:
        unsafe extern "win64" fn(*mut LangBarMgrObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    restore_last_focus: unsafe extern "win64" fn(*mut LangBarMgrObj, *mut u32, BOOL) -> HRESULT,
    set_modal_input: unsafe extern "win64" fn(*mut LangBarMgrObj, *mut c_void, u32, u32) -> HRESULT,
    show_floating: unsafe extern "win64" fn(*mut LangBarMgrObj, u32) -> HRESULT,
    get_show_floating_status: unsafe extern "win64" fn(*mut LangBarMgrObj, *mut u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfLangBarMgrVtbl>() == 12 * std::mem::size_of::<usize>());

struct LangBarState {
    floating_flags: u32,
    next_cookie: u32,
    sinks: HashMap<u32, usize>,
}

#[repr(C)]
struct LangBarMgrObj {
    vtbl: *const ITfLangBarMgrVtbl,
    refs: AtomicU32,
    state: Mutex<LangBarState>,
}

unsafe extern "win64" fn lbm_qi(
    this: *mut LangBarMgrObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN || iid == IID_ITFLANGBARMGR {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn lbm_add_ref(this: *mut LangBarMgrObj) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn lbm_release(this: *mut LangBarMgrObj) -> u32 {
    let obj = &*this;
    let rem = obj.refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        let sinks: Vec<*mut c_void> = obj
            .state
            .lock()
            .unwrap()
            .sinks
            .drain()
            .map(|(_, p)| p as *mut c_void)
            .collect();
        drop(Box::from_raw(this));
        release_all_ptrs(sinks);
    }
    rem
}
unsafe extern "win64" fn lbm_advise_event_sink(
    this: *mut LangBarMgrObj,
    p_sink: *mut c_void,
    _hwnd: HWND,
    _dw_flags: u32,
    pdw_cookie: *mut u32,
) -> HRESULT {
    if !pdw_cookie.is_null() {
        std::ptr::write_unaligned(pdw_cookie, TF_INVALID_COOKIE);
    }
    if p_sink.is_null() || pdw_cookie.is_null() {
        return E_INVALIDARG;
    }
    com_call_add_ref(p_sink);
    let obj = &*this;
    let cookie = {
        let mut st = obj.state.lock().unwrap();
        let mut c = st.next_cookie;
        while c == 0 || c == TF_INVALID_COOKIE || st.sinks.contains_key(&c) {
            c = c.wrapping_add(1);
        }
        st.next_cookie = c.wrapping_add(1);
        st.sinks.insert(c, p_sink as usize);
        c
    };
    std::ptr::write_unaligned(pdw_cookie, cookie);
    S_OK
}
unsafe extern "win64" fn lbm_unadvise_event_sink(this: *mut LangBarMgrObj, dw_cookie: u32) -> HRESULT {
    let obj = &*this;
    let removed = obj.state.lock().unwrap().sinks.remove(&dw_cookie);
    match removed {
        Some(p) => {
            com_call_release(p as *mut c_void);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}
unsafe extern "win64" fn lbm_get_thread_marshal_interface(
    _this: *mut LangBarMgrObj,
    _dw_thread_id: u32,
    _dw_type: u32,
    _riid: *const u8,
    ppunk: *mut *mut c_void,
) -> HRESULT {
    if !ppunk.is_null() {
        std::ptr::write_unaligned(ppunk, std::ptr::null_mut());
    }
    E_NOTIMPL
}
unsafe extern "win64" fn lbm_get_thread_item_mgr(
    _this: *mut LangBarMgrObj,
    _dw_thread_id: u32,
    pplbie: *mut *mut c_void,
    pdw_threadid: *mut u32,
) -> HRESULT {
    if !pplbie.is_null() {
        std::ptr::write_unaligned(pplbie, std::ptr::null_mut());
    }
    if !pdw_threadid.is_null() {
        std::ptr::write_unaligned(pdw_threadid, current_tid());
    }
    E_NOTIMPL
}
unsafe extern "win64" fn lbm_get_input_processor_profiles(
    _this: *mut LangBarMgrObj,
    dw_thread_id: u32,
    ppaip: *mut *mut c_void,
    pdw_threadid: *mut u32,
) -> HRESULT {
    if ppaip.is_null() {
        return E_INVALIDARG;
    }
    let prof = new_input_processor_profiles_obj();
    std::ptr::write_unaligned(ppaip, prof as *mut c_void);
    if !pdw_threadid.is_null() {
        let tid = if dw_thread_id == 0 { current_tid() } else { dw_thread_id };
        std::ptr::write_unaligned(pdw_threadid, tid);
    }
    S_OK
}
unsafe extern "win64" fn lbm_restore_last_focus(
    _this: *mut LangBarMgrObj,
    pdw_thread_id: *mut u32,
    _f_prev: BOOL,
) -> HRESULT {
    if !pdw_thread_id.is_null() {
        std::ptr::write_unaligned(pdw_thread_id, current_tid());
    }
    S_OK
}
unsafe extern "win64" fn lbm_set_modal_input(
    _this: *mut LangBarMgrObj,
    _p_sink: *mut c_void,
    _dw_thread_id: u32,
    _dw_flags: u32,
) -> HRESULT {
    S_OK
}
unsafe extern "win64" fn lbm_show_floating(this: *mut LangBarMgrObj, dw_flags: u32) -> HRESULT {
    let obj = &*this;
    obj.state.lock().unwrap().floating_flags = dw_flags;
    S_OK
}
unsafe extern "win64" fn lbm_get_show_floating_status(
    this: *mut LangBarMgrObj,
    pdw_flags: *mut u32,
) -> HRESULT {
    if pdw_flags.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*this;
    let flags = obj.state.lock().unwrap().floating_flags;
    std::ptr::write_unaligned(pdw_flags, flags);
    S_OK
}

static ITF_LANG_BAR_MGR_VTBL: ITfLangBarMgrVtbl = ITfLangBarMgrVtbl {
    query_interface: lbm_qi,
    add_ref: lbm_add_ref,
    release: lbm_release,
    advise_event_sink: lbm_advise_event_sink,
    unadvise_event_sink: lbm_unadvise_event_sink,
    get_thread_marshal_interface: lbm_get_thread_marshal_interface,
    get_thread_lang_bar_item_mgr: lbm_get_thread_item_mgr,
    get_input_processor_profiles: lbm_get_input_processor_profiles,
    restore_last_focus: lbm_restore_last_focus,
    set_modal_input: lbm_set_modal_input,
    show_floating: lbm_show_floating,
    get_show_floating_status: lbm_get_show_floating_status,
};

fn new_lang_bar_mgr_obj() -> *mut LangBarMgrObj {
    register_vtable_ptrs(&[
        lbm_qi as *const () as usize,
        lbm_add_ref as *const () as usize,
        lbm_release as *const () as usize,
        lbm_advise_event_sink as *const () as usize,
        lbm_unadvise_event_sink as *const () as usize,
        lbm_get_thread_marshal_interface as *const () as usize,
        lbm_get_thread_item_mgr as *const () as usize,
        lbm_get_input_processor_profiles as *const () as usize,
        lbm_restore_last_focus as *const () as usize,
        lbm_set_modal_input as *const () as usize,
        lbm_show_floating as *const () as usize,
        lbm_get_show_floating_status as *const () as usize,
    ]);
    Box::into_raw(Box::new(LangBarMgrObj {
        vtbl: &ITF_LANG_BAR_MGR_VTBL,
        refs: AtomicU32::new(1),
        state: Mutex::new(LangBarState {
            floating_flags: 0x0008, // TF_SFT_HIDDEN
            next_cookie: 1,
            sinks: HashMap::new(),
        }),
    }))
}

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
    let cls = read_guid(clsid)?;
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
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if !outer.is_null() {
        return Some(CLASS_E_NOAGGREGATION);
    }
    let Some(want) = read_guid(iid) else {
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
    std::ptr::write_unaligned(pptim, std::ptr::null_mut());
    let tm = get_or_create_thread_mgr();
    std::ptr::write_unaligned(pptim, tm as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn TF_GetThreadMgr(pptim: *mut *mut c_void) -> HRESULT {
    if pptim.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pptim, std::ptr::null_mut());
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
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    const IID_ICLASSFACTORY: Guid =
        guid(0x00000001, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

    // Mock ITfUIElement + ITfUIElementSink + ITfThreadMgrEventSink + ITfCompartmentEventSink
    #[repr(C)]
    struct MockSinkVtbl {
        qi: unsafe extern "win64" fn(*mut MockSink, *const u8, *mut *mut c_void) -> HRESULT,
        add_ref: unsafe extern "win64" fn(*mut MockSink) -> u32,
        release: unsafe extern "win64" fn(*mut MockSink) -> u32,
        slot3: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
        slot4: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
        slot5: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
        slot6: unsafe extern "win64" fn(*mut MockSink, u64) -> HRESULT,
        slot7: unsafe extern "win64" fn(*mut MockSink, u64) -> HRESULT,
    }

    #[repr(C)]
    struct MockSink {
        vtbl: *const MockSinkVtbl,
        accept_iid: Guid,
        refs: AtomicU32,
        slot3_calls: AtomicU32,
        slot4_calls: AtomicU32,
        slot5_calls: AtomicU32,
        slot6_calls: AtomicU32,
        slot7_calls: AtomicU32,
        suppress_ui: bool,
    }

    unsafe extern "win64" fn mock_sink_qi(
        this: *mut MockSink,
        riid: *const u8,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        if ppv.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(ppv, std::ptr::null_mut());
        let obj = &*this;
        let Some(g) = read_guid(riid) else {
            return E_INVALIDARG;
        };
        if g == IID_IUNKNOWN || g == obj.accept_iid {
            obj.refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this as *mut c_void);
            S_OK
        } else {
            E_NOINTERFACE
        }
    }
    unsafe extern "win64" fn mock_sink_add_ref(this: *mut MockSink) -> u32 {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel) + 1
    }
    unsafe extern "win64" fn mock_sink_release(this: *mut MockSink) -> u32 {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel) - 1
    }
    unsafe extern "win64" fn mock_sink_slot3(this: *mut MockSink, _a1: u64, a2: u64) -> HRESULT {
        let obj = &*this;
        obj.slot3_calls.fetch_add(1, Ordering::AcqRel);
        if obj.suppress_ui && a2 != 0 {
            std::ptr::write_unaligned(a2 as *mut BOOL, FALSE);
        }
        S_OK
    }
    unsafe extern "win64" fn mock_sink_slot4(this: *mut MockSink, _a1: u64, _a2: u64) -> HRESULT {
        let obj = &*this;
        obj.slot4_calls.fetch_add(1, Ordering::AcqRel);
        S_OK
    }
    unsafe extern "win64" fn mock_sink_slot5(this: *mut MockSink, _a1: u64, _a2: u64) -> HRESULT {
        let obj = &*this;
        obj.slot5_calls.fetch_add(1, Ordering::AcqRel);
        S_OK
    }
    unsafe extern "win64" fn mock_sink_slot6(this: *mut MockSink, _a1: u64) -> HRESULT {
        let obj = &*this;
        obj.slot6_calls.fetch_add(1, Ordering::AcqRel);
        S_OK
    }
    unsafe extern "win64" fn mock_sink_slot7(this: *mut MockSink, _a1: u64) -> HRESULT {
        let obj = &*this;
        obj.slot7_calls.fetch_add(1, Ordering::AcqRel);
        S_OK
    }

    static MOCK_SINK_VTBL: MockSinkVtbl = MockSinkVtbl {
        qi: mock_sink_qi,
        add_ref: mock_sink_add_ref,
        release: mock_sink_release,
        slot3: mock_sink_slot3,
        slot4: mock_sink_slot4,
        slot5: mock_sink_slot5,
        slot6: mock_sink_slot6,
        slot7: mock_sink_slot7,
    };

    fn make_mock_sink(iid: Guid, suppress_ui: bool) -> MockSink {
        MockSink {
            vtbl: &MOCK_SINK_VTBL,
            accept_iid: iid,
            refs: AtomicU32::new(1),
            slot3_calls: AtomicU32::new(0),
            slot4_calls: AtomicU32::new(0),
            slot5_calls: AtomicU32::new(0),
            slot6_calls: AtomicU32::new(0),
            slot7_calls: AtomicU32::new(0),
            suppress_ui,
        }
    }

    // Mock ITfEditSession
    #[repr(C)]
    struct MockEditSessionVtbl {
        qi: unsafe extern "win64" fn(*mut MockEditSession, *const u8, *mut *mut c_void) -> HRESULT,
        add_ref: unsafe extern "win64" fn(*mut MockEditSession) -> u32,
        release: unsafe extern "win64" fn(*mut MockEditSession) -> u32,
        do_edit_session: unsafe extern "win64" fn(*mut MockEditSession, TfEditCookie) -> HRESULT,
    }
    #[repr(C)]
    struct MockEditSession {
        vtbl: *const MockEditSessionVtbl,
        refs: AtomicU32,
        ctx: *mut c_void,
        tid: TfClientId,
        saw_cookie: AtomicU32,
        saw_in_write: AtomicU32,
    }
    unsafe extern "win64" fn mes_qi(
        this: *mut MockEditSession,
        riid: *const u8,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        if ppv.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(ppv, std::ptr::null_mut());
        if read_guid(riid) == Some(IID_IUNKNOWN) {
            let obj = &*this;
            obj.refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this as *mut c_void);
            S_OK
        } else {
            E_NOINTERFACE
        }
    }
    unsafe extern "win64" fn mes_add_ref(this: *mut MockEditSession) -> u32 {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel) + 1
    }
    unsafe extern "win64" fn mes_release(this: *mut MockEditSession) -> u32 {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel) - 1
    }
    unsafe extern "win64" fn mes_do_edit(this: *mut MockEditSession, ec: TfEditCookie) -> HRESULT {
        let obj = &*this;
        obj.saw_cookie.store(ec, Ordering::Release);
        let ctx_vt = &*(*(obj.ctx as *const *const ITfContextVtbl));
        let mut in_write: BOOL = FALSE;
        assert_eq!((ctx_vt.in_write_session)(obj.ctx, obj.tid, &mut in_write), S_OK);
        obj.saw_in_write.store(in_write as u32, Ordering::Release);
        S_OK
    }
    static MOCK_EDIT_SESSION_VTBL: MockEditSessionVtbl = MockEditSessionVtbl {
        qi: mes_qi,
        add_ref: mes_add_ref,
        release: mes_release,
        do_edit_session: mes_do_edit,
    };

    #[test]
    fn com_creation_co_get_class_object_and_thread_mgr_lifecycle() {
        // Run on a dedicated thread so TF_GetThreadMgr starts strictly empty.
        std::thread::spawn(|| unsafe {
            // Before creation, TF_GetThreadMgr returns S_OK with NULL pointer.
            let mut got: *mut c_void = 0x1234usize as *mut c_void;
            assert_eq!(TF_GetThreadMgr(&mut got), S_OK);
            assert!(got.is_null());

            // CoCreateInstance(CLSID_TF_ThreadMgr, IID_ITfThreadMgr)
            let mut tm: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                super::super::ole32::CoCreateInstance(
                    CLSID_TF_THREADMGR.as_ptr(),
                    std::ptr::null_mut(),
                    1,
                    IID_ITFTHREADMGR.as_ptr(),
                    &mut tm,
                ),
                S_OK
            );
            assert!(!tm.is_null());

            // TF_GetThreadMgr now returns the exact same per-thread manager.
            let mut tm_get: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_GetThreadMgr(&mut tm_get), S_OK);
            assert_eq!(tm_get, tm);
            com_call_release(tm_get);

            // TF_CreateThreadMgr also returns the synchronized per-thread manager.
            let mut tm_create: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateThreadMgr(&mut tm_create), S_OK);
            assert_eq!(tm_create, tm);
            com_call_release(tm_create);

            // CoCreateInstance with IID_ITfThreadMgrEx and IID_ITfThreadMgr2
            let mut tm_ex: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                super::super::ole32::CoCreateInstance(
                    CLSID_TF_THREADMGR.as_ptr(),
                    std::ptr::null_mut(),
                    1,
                    IID_ITFTHREADMGREX.as_ptr(),
                    &mut tm_ex,
                ),
                S_OK
            );
            assert_eq!(tm_ex, tm);

            let mut tm2: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm, IID_ITFTHREADMGR2.as_ptr(), &mut tm2), S_OK);
            assert!(!tm2.is_null());
            assert_ne!(tm2, tm); // distinct vtable offset!
            // COM identity rule: QI(IUnknown) from ITfThreadMgr2 returns canonical offset-0 pointer.
            let mut unk: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm2, IID_IUNKNOWN.as_ptr(), &mut unk), S_OK);
            assert_eq!(unk, tm);
            com_call_release(unk);

            // ActivateEx / GetActiveFlags / Activate / Deactivate + ITfThreadMgr2 keystroke suspend
            let tm_vt = &*(*(tm_ex as *const *const ITfThreadMgrExVtbl));
            let tm2_vt = &*(*(tm2 as *const *const ITfThreadMgr2Vtbl));

            let mut active_flags = 0xDEADu32;
            assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
            assert_eq!(active_flags, 0);

            let mut cid: TfClientId = 0;
            assert_eq!((tm_vt.activate_ex)(tm_ex, &mut cid, 0x0004), S_OK);
            assert_ne!(cid, 0);
            assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
            assert_eq!(active_flags, 0x0004 | TF_TMF_ACTIVATED);

            // Nested Activate through ITfThreadMgr2 shares exact same thread state
            let mut cid2: TfClientId = 0;
            assert_eq!((tm2_vt.activate)(tm2, &mut cid2), S_OK);
            assert_eq!(cid2, cid);

            let mut is_focus: BOOL = FALSE;
            assert_eq!((tm2_vt.is_thread_focus)(tm2, &mut is_focus), S_OK);
            assert_eq!(is_focus, TRUE);

            assert_eq!((tm2_vt.suspend_keystroke_handling)(tm2), S_OK);
            assert_eq!((tm2_vt.resume_keystroke_handling)(tm2), S_OK);
            assert_eq!((tm2_vt.resume_keystroke_handling)(tm2), E_UNEXPECTED);

            assert_eq!((tm_vt.deactivate)(tm_ex), S_OK);
            assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
            assert_ne!(active_flags, 0); // still active once
            assert_eq!((tm_vt.deactivate)(tm_ex), S_OK);
            assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
            assert_eq!(active_flags, 0);
            assert_eq!((tm_vt.deactivate)(tm_ex), E_UNEXPECTED);

            // CoGetClassObject returns real IClassFactory for CLSID_TF_ThreadMgr
            let mut cf: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                super::super::ole32::CoGetClassObject(
                    CLSID_TF_THREADMGR.as_ptr(),
                    1,
                    std::ptr::null_mut(),
                    IID_ICLASSFACTORY.as_ptr(),
                    &mut cf,
                ),
                S_OK
            );
            assert!(!cf.is_null());
            let mut from_cf: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                com_call_slot(
                    cf,
                    3, // IClassFactory::CreateInstance
                    0,
                    IID_ITFTHREADMGR.as_ptr() as u64,
                    (&mut from_cf as *mut *mut c_void) as u64,
                ),
                S_OK
            );
            assert_eq!(from_cf, tm);
            com_call_release(from_cf);
            com_call_release(cf);

            // Release all refs to ThreadMgr; TF_GetThreadMgr must return NULL again.
            com_call_release(tm2);
            com_call_release(tm_ex);
            assert_eq!(com_call_release(tm), 0);

            let mut after_drop: *mut c_void = 0x9999usize as *mut c_void;
            assert_eq!(TF_GetThreadMgr(&mut after_drop), S_OK);
            assert!(after_drop.is_null());
        })
        .join()
        .unwrap();
    }

    #[test]
    fn document_mgr_context_stack_focus_and_edit_session() {
        std::thread::spawn(|| unsafe {
            let mut tm: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);
            let tm_vt = &*(*(tm as *const *const ITfThreadMgrExVtbl));

            let mut cid: TfClientId = 0;
            assert_eq!((tm_vt.activate)(tm, &mut cid), S_OK);

            // Advise ITfThreadMgrEventSink to observe OnInitDocumentMgr, OnSetFocus, OnPushContext, OnPopContext
            let mut src: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm, IID_ITFSOURCE.as_ptr(), &mut src), S_OK);
            let src_vt = &*(*(src as *const *const ITfSourceVtbl));
            let mut tm_sink = make_mock_sink(IID_ITFTHREADMGREVENTSINK, false);
            let mut sink_cookie = 0u32;
            assert_eq!(
                (src_vt.advise_sink)(
                    src,
                    IID_ITFTHREADMGREVENTSINK.as_ptr(),
                    (&mut tm_sink as *mut MockSink) as *mut c_void,
                    &mut sink_cookie,
                ),
                S_OK
            );
            assert_eq!(tm_sink.refs.load(Ordering::Acquire), 2);

            // Create two DocumentMgrs
            let mut dm1: *mut c_void = std::ptr::null_mut();
            let mut dm2: *mut c_void = std::ptr::null_mut();
            assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm1), S_OK);
            assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm2), S_OK);
            assert_eq!(tm_sink.slot3_calls.load(Ordering::Acquire), 2); // OnInitDocumentMgr x2

            // CreateContext / Push / GetTop / GetBase / EnumContexts / Pop
            let dm_vt = &*(*(dm1 as *const *const ITfDocumentMgrVtbl));
            let (mut ctx1, mut ec1) = (std::ptr::null_mut(), 0u32);
            let (mut ctx2, mut ec2) = (std::ptr::null_mut(), 0u32);
            assert_eq!((dm_vt.create_context)(dm1, cid, 0, std::ptr::null_mut(), &mut ctx1, &mut ec1), S_OK);
            assert_eq!((dm_vt.create_context)(dm1, cid, 0, std::ptr::null_mut(), &mut ctx2, &mut ec2), S_OK);
            assert_ne!(ec1, TF_INVALID_EDIT_COOKIE);
            assert_ne!(ec2, TF_INVALID_EDIT_COOKIE);

            assert_eq!((dm_vt.push)(dm1, ctx1), S_OK);
            assert_eq!((dm_vt.push)(dm1, ctx2), S_OK);
            assert_eq!(tm_sink.slot6_calls.load(Ordering::Acquire), 2); // OnPushContext x2

            let mut top: *mut c_void = std::ptr::null_mut();
            let mut base: *mut c_void = std::ptr::null_mut();
            assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
            assert_eq!((dm_vt.get_base)(dm1, &mut base), S_OK);
            assert_eq!(top, ctx2);
            assert_eq!(base, ctx1);
            com_call_release(top);
            com_call_release(base);

            // ITfContext::GetDocumentMgr & RequestEditSession + ITfProperty roundtrip
            let ctx_vt = &*(*(ctx1 as *const *const ITfContextVtbl));
            let mut owner_dm: *mut c_void = std::ptr::null_mut();
            assert_eq!((ctx_vt.get_document_mgr)(ctx1, &mut owner_dm), S_OK);
            assert_eq!(owner_dm, dm1);
            com_call_release(owner_dm);

            let mut es = MockEditSession {
                vtbl: &MOCK_EDIT_SESSION_VTBL,
                refs: AtomicU32::new(1),
                ctx: ctx1,
                tid: cid,
                saw_cookie: AtomicU32::new(0),
                saw_in_write: AtomicU32::new(0),
            };
            let mut hr_sess = E_FAIL;
            assert_eq!(
                (ctx_vt.request_edit_session)(
                    ctx1,
                    cid,
                    (&mut es as *mut MockEditSession) as *mut c_void,
                    TF_ES_SYNC | TF_ES_READWRITE,
                    &mut hr_sess,
                ),
                S_OK
            );
            assert_eq!(hr_sess, S_OK);
            assert_ne!(es.saw_cookie.load(Ordering::Acquire), 0);
            assert_eq!(es.saw_in_write.load(Ordering::Acquire), TRUE as u32);
            assert_eq!(es.refs.load(Ordering::Acquire), 1);

            // Pop top context -> ctx1 becomes top; Pop(TF_POPF_ALL) empties stack
            assert_eq!((dm_vt.pop)(dm1, 0), S_OK);
            assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
            assert_eq!(top, ctx1);
            com_call_release(top);
            assert_eq!((dm_vt.pop)(dm1, TF_POPF_ALL), S_OK);
            assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
            assert!(top.is_null());
            assert_eq!(tm_sink.slot7_calls.load(Ordering::Acquire), 2); // OnPopContext x2

            // SetFocus / GetFocus / AssociateFocus
            assert_eq!((tm_vt.set_focus)(tm, dm1), S_OK);
            let mut focused: *mut c_void = std::ptr::null_mut();
            assert_eq!((tm_vt.get_focus)(tm, &mut focused), S_OK);
            assert_eq!(focused, dm1);
            com_call_release(focused);
            assert_eq!(tm_sink.slot5_calls.load(Ordering::Acquire), 1); // OnSetFocus

            let fake_hwnd = 0x000A_BCDEusize as HWND;
            let mut prev_assoc: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!((tm_vt.associate_focus)(tm, fake_hwnd, dm1, &mut prev_assoc), S_OK);
            assert!(prev_assoc.is_null());

            assert_eq!((tm_vt.associate_focus)(tm, fake_hwnd, dm2, &mut prev_assoc), S_OK);
            assert_eq!(prev_assoc, dm1);
            com_call_release(prev_assoc);

            assert_eq!((tm_vt.associate_focus)(tm, fake_hwnd, std::ptr::null_mut(), &mut prev_assoc), S_OK);
            assert_eq!(prev_assoc, dm2);
            com_call_release(prev_assoc);

            // UnadviseSink releases sink ref
            assert_eq!((src_vt.unadvise_sink)(src, sink_cookie), S_OK);
            assert_eq!(tm_sink.refs.load(Ordering::Acquire), 1);
            assert_eq!((src_vt.unadvise_sink)(src, sink_cookie), CONNECT_E_NOCONNECTION);

            // Cleanup
            com_call_release(ctx2);
            com_call_release(ctx1);
            assert_eq!((tm_vt.deactivate)(tm), S_OK);
            com_call_release(dm2);
            com_call_release(dm1);
            com_call_release(src);
            assert_eq!(com_call_release(tm), 0);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn ui_element_mgr_compartments_and_profiles_category_display_attrs() {
        std::thread::spawn(|| unsafe {
            let mut tm: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);

            // ITfUIElementMgr + ITfSource(ITfUIElementSink)
            let mut src: *mut c_void = std::ptr::null_mut();
            let mut ui_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm, IID_ITFSOURCE.as_ptr(), &mut src), S_OK);
            assert_eq!(com_call_qi(tm, IID_ITFUIELEMENTMGR.as_ptr(), &mut ui_mgr), S_OK);
            assert_ne!(src, tm);
            assert_ne!(ui_mgr, tm);
            assert_ne!(src, ui_mgr);

            let src_vt = &*(*(src as *const *const ITfSourceVtbl));
            let ui_vt = &*(*(ui_mgr as *const *const ITfUIElementMgrVtbl));

            let mut ui_sink = make_mock_sink(IID_ITFUIELEMENTSINK, true);
            let mut ui_cookie = 0u32;
            assert_eq!(
                (src_vt.advise_sink)(
                    src,
                    IID_ITFUIELEMENTSINK.as_ptr(),
                    (&mut ui_sink as *mut MockSink) as *mut c_void,
                    &mut ui_cookie,
                ),
                S_OK
            );

            let mut elem = make_mock_sink(IID_IUNKNOWN, false);
            let elem_ptr = (&mut elem as *mut MockSink) as *mut c_void;
            let mut show: BOOL = TRUE;
            let mut elem_id = 0u32;
            assert_eq!((ui_vt.begin_ui_element)(ui_mgr, elem_ptr, &mut show, &mut elem_id), S_OK);
            assert_ne!(elem_id, 0);
            assert_eq!(show, FALSE); // sink suppressed default UI!
            assert_eq!(ui_sink.slot3_calls.load(Ordering::Acquire), 1);

            assert_eq!((ui_vt.update_ui_element)(ui_mgr, elem_id), S_OK);
            assert_eq!(ui_sink.slot4_calls.load(Ordering::Acquire), 1);

            let mut fetched_elem: *mut c_void = std::ptr::null_mut();
            assert_eq!((ui_vt.get_ui_element)(ui_mgr, elem_id, &mut fetched_elem), S_OK);
            assert_eq!(fetched_elem, elem_ptr);
            com_call_release(fetched_elem);

            let mut ui_enum: *mut c_void = std::ptr::null_mut();
            assert_eq!((ui_vt.enum_ui_elements)(ui_mgr, &mut ui_enum), S_OK);
            let e_vt = &*(*(ui_enum as *const *const IEnumTfComVtbl));
            let mut out_elem: *mut c_void = std::ptr::null_mut();
            let mut fetched = 0u32;
            assert_eq!((e_vt.next)(ui_enum as *mut EnumTfComObj, 1, &mut out_elem, &mut fetched), S_OK);
            assert_eq!(fetched, 1);
            assert_eq!(out_elem, elem_ptr);
            com_call_release(out_elem);
            com_call_release(ui_enum);

            assert_eq!((ui_vt.end_ui_element)(ui_mgr, elem_id), S_OK);
            assert_eq!(ui_sink.slot5_calls.load(Ordering::Acquire), 1);
            assert_eq!(elem.refs.load(Ordering::Acquire), 1);
            assert_eq!((src_vt.unadvise_sink)(src, ui_cookie), S_OK);
            assert_eq!(ui_sink.refs.load(Ordering::Acquire), 1);

            // ITfCompartmentMgr + ITfCompartment + ITfCompartmentEventSink + IEnumGUID
            let mut comp_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm, IID_ITFCOMPARTMENTMGR.as_ptr(), &mut comp_mgr), S_OK);
            let cm_vt = &*(*(comp_mgr as *const *const ITfCompartmentMgrVtbl));

            let test_comp_guid = guid(0x11223344, 0x5566, 0x7788, [1, 2, 3, 4, 5, 6, 7, 8]);
            let mut comp: *mut c_void = std::ptr::null_mut();
            assert_eq!((cm_vt.get_compartment)(comp_mgr, test_comp_guid.as_ptr(), &mut comp), S_OK);
            let comp_vt = &*(*(comp as *const *const ITfCompartmentVtbl));

            let mut comp_src: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(comp, IID_ITFSOURCE.as_ptr(), &mut comp_src), S_OK);
            let csrc_vt = &*(*(comp_src as *const *const ITfSourceVtbl));
            let mut comp_sink = make_mock_sink(IID_ITFCOMPARTMENTEVENTSINK, false);
            let mut comp_cookie = 0u32;
            assert_eq!(
                (csrc_vt.advise_sink)(
                    comp_src,
                    IID_ITFCOMPARTMENTEVENTSINK.as_ptr(),
                    (&mut comp_sink as *mut MockSink) as *mut c_void,
                    &mut comp_cookie,
                ),
                S_OK
            );

            // Empty compartment returns S_FALSE
            let mut var_out = [0xFFu8; 24];
            assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_FALSE);
            assert_eq!(u16::from_le_bytes([var_out[0], var_out[1]]), VT_EMPTY);

            // Set VT_I4 = 42
            let mut var_in = [0u8; 24];
            std::ptr::write_unaligned(var_in.as_mut_ptr() as *mut u16, 3u16); // VT_I4
            std::ptr::write_unaligned(var_in.as_mut_ptr().add(8) as *mut i32, 42);
            assert_eq!((comp_vt.set_value)(comp, 1, var_in.as_ptr()), S_OK);
            assert_eq!(comp_sink.slot3_calls.load(Ordering::Acquire), 1);

            assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_OK);
            assert_eq!(std::ptr::read_unaligned(var_out.as_ptr() as *const u16), 3u16);
            assert_eq!(std::ptr::read_unaligned(var_out.as_ptr().add(8) as *const i32), 42);

            // EnumCompartments (IEnumGUID: Next at slot 3!)
            let mut comp_enum: *mut c_void = std::ptr::null_mut();
            assert_eq!((cm_vt.enum_compartments)(comp_mgr, &mut comp_enum), S_OK);
            let eg_vt = &*(*(comp_enum as *const *const IEnumGUIDVtbl));
            let mut enum_g = [0u8; 16];
            let mut eg_fetched = 0u32;
            assert_eq!(
                (eg_vt.next)(comp_enum as *mut EnumGuidObj, 1, enum_g.as_mut_ptr(), &mut eg_fetched),
                S_OK
            );
            assert_eq!(eg_fetched, 1);
            assert_eq!(enum_g, test_comp_guid);
            com_call_release(comp_enum);

            assert_eq!((cm_vt.clear_compartment)(comp_mgr, 1, test_comp_guid.as_ptr()), S_OK);
            assert_eq!(comp_sink.slot3_calls.load(Ordering::Acquire), 2);
            assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_FALSE);
            assert_eq!((csrc_vt.unadvise_sink)(comp_src, comp_cookie), S_OK);
            com_call_release(comp_src);
            com_call_release(comp);

            // ITfInputProcessorProfiles (en-US active profile, GetCurrentLanguage, GetLanguageList, GetDefaultLanguageProfile)
            let mut profiles: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                super::super::ole32::CoCreateInstance(
                    CLSID_TF_INPUTPROCESSORPROFILES.as_ptr(),
                    std::ptr::null_mut(),
                    1,
                    IID_ITFINPUTPROCESSORPROFILES.as_ptr(),
                    &mut profiles,
                ),
                S_OK
            );
            let p_vt = &*(*(profiles as *const *const ITfInputProcessorProfilesVtbl));
            let mut cur_lang = 0u16;
            assert_eq!((p_vt.get_current_language)(profiles, &mut cur_lang), S_OK);
            assert_eq!(cur_lang, LANGID_EN_US);

            let mut langs_ptr: *mut u16 = std::ptr::null_mut();
            let mut langs_cnt = 0u32;
            assert_eq!((p_vt.get_language_list)(profiles, &mut langs_ptr, &mut langs_cnt), S_OK);
            assert!(langs_cnt >= 1);
            assert_eq!(std::ptr::read_unaligned(langs_ptr), LANGID_EN_US);
            super::super::ole32::CoTaskMemFree(langs_ptr as *mut c_void);

            let mut def_clsid = [0u8; 16];
            let mut def_guid = [0u8; 16];
            assert_eq!(
                (p_vt.get_default_language_profile)(
                    profiles,
                    LANGID_EN_US,
                    GUID_TFCAT_TIP_KEYBOARD.as_ptr(),
                    def_clsid.as_mut_ptr(),
                    def_guid.as_mut_ptr(),
                ),
                S_OK
            );
            assert_eq!(def_clsid, CLSID_EN_US_KEYBOARD_TIP);
            assert_eq!(def_guid, GUID_EN_US_KEYBOARD_PROFILE);

            // ITfInputProcessorProfileMgr via QI
            let mut prof_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(profiles, IID_ITFINPUTPROCESSORPROFILEMGR.as_ptr(), &mut prof_mgr), S_OK);
            let pm_vt = &*(*(prof_mgr as *const *const ITfInputProcessorProfileMgrVtbl));
            let mut ipp_rec = TF_INPUTPROCESSORPROFILE::default();
            assert_eq!(
                (pm_vt.get_active_profile)(
                    prof_mgr,
                    GUID_TFCAT_TIP_KEYBOARD.as_ptr(),
                    (&mut ipp_rec as *mut TF_INPUTPROCESSORPROFILE) as *mut u8,
                ),
                S_OK
            );
            assert_eq!(ipp_rec.langid, LANGID_EN_US);
            com_call_release(prof_mgr);
            com_call_release(profiles);

            // ITfCategoryMgr GUID-atom roundtrip
            let mut cat_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateCategoryMgr(&mut cat_mgr), S_OK);
            let cat_vt = &*(*(cat_mgr as *const *const ITfCategoryMgrVtbl));
            let mut atom: TfGuidAtom = 0;
            assert_eq!((cat_vt.register_guid)(cat_mgr as *mut CategoryMgrObj, test_comp_guid.as_ptr(), &mut atom), S_OK);
            assert_ne!(atom, TF_INVALID_GUIDATOM);
            let mut got_g = [0u8; 16];
            assert_eq!((cat_vt.get_guid)(cat_mgr as *mut CategoryMgrObj, atom, got_g.as_mut_ptr()), S_OK);
            assert_eq!(got_g, test_comp_guid);
            let mut eq: BOOL = FALSE;
            assert_eq!(
                (cat_vt.is_equal_tf_guid_atom)(cat_mgr as *mut CategoryMgrObj, atom, test_comp_guid.as_ptr(), &mut eq),
                S_OK
            );
            assert_eq!(eq, TRUE);
            com_call_release(cat_mgr);

            // ITfDisplayAttributeMgr enumerator & info lookup
            let mut da_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateDisplayAttributeMgr(&mut da_mgr), S_OK);
            let da_vt = &*(*(da_mgr as *const *const ITfDisplayAttributeMgrVtbl));
            let mut da_enum: *mut c_void = std::ptr::null_mut();
            assert_eq!((da_vt.enum_display_attribute_info)(da_mgr as *mut DisplayAttrMgrObj, &mut da_enum), S_OK);
            let dae_vt = &*(*(da_enum as *const *const IEnumTfComVtbl));
            let mut da_info: *mut c_void = std::ptr::null_mut();
            let mut da_fetched = 0u32;
            assert_eq!((dae_vt.next)(da_enum as *mut EnumTfComObj, 1, &mut da_info, &mut da_fetched), S_OK);
            assert_eq!(da_fetched, 1);
            assert!(!da_info.is_null());
            let dai_vt = &*(*(da_info as *const *const ITfDisplayAttributeInfoVtbl));
            let mut attr = TF_DISPLAYATTRIBUTE::default();
            assert_eq!(
                (dai_vt.get_attribute_info)(
                    da_info as *mut DisplayAttrInfoObj,
                    (&mut attr as *mut TF_DISPLAYATTRIBUTE) as *mut u8,
                ),
                S_OK
            );
            assert_eq!(attr.ls_style, TF_LS_DOT);
            com_call_release(da_info);
            com_call_release(da_enum);
            com_call_release(da_mgr);

            // ITfLangBarMgr + SetInputScope / SetInputScopes / TF_InvalidAssemblyListCacheIfExist
            let mut lbm: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateLangBarMgr(&mut lbm), S_OK);
            let lbm_vt = &*(*(lbm as *const *const ITfLangBarMgrVtbl));
            assert_eq!((lbm_vt.show_floating)(lbm as *mut LangBarMgrObj, 0x0001), S_OK);
            let mut fl = 0u32;
            assert_eq!((lbm_vt.get_show_floating_status)(lbm as *mut LangBarMgrObj, &mut fl), S_OK);
            assert_eq!(fl, 0x0001);
            com_call_release(lbm);

            let hwnd = 0x0005_0001usize as HWND;
            assert_eq!(SetInputScope(hwnd, 57), S_OK);
            let scopes = [1i32, 57i32];
            assert_eq!(
                SetInputScopes(
                    hwnd,
                    scopes.as_ptr(),
                    2,
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                S_OK
            );
            assert_eq!(SetInputScope(hwnd, 0), S_OK);
            assert_eq!(TF_InvalidAssemblyListCacheIfExist(), S_OK);

            com_call_release(comp_mgr);
            com_call_release(ui_mgr);
            com_call_release(src);
            assert_eq!(com_call_release(tm), 0);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn keystroke_mgr_enumerators_properties_and_exports_lookup() {
        // Verify all 9 msctf.dll exports resolve through win32-rt's DLL table.
        for sym in [
            "TF_CreateThreadMgr",
            "TF_GetThreadMgr",
            "TF_CreateInputProcessorProfiles",
            "TF_CreateDisplayAttributeMgr",
            "TF_CreateCategoryMgr",
            "TF_CreateLangBarMgr",
            "TF_InvalidAssemblyListCacheIfExist",
            "SetInputScope",
            "SetInputScopes",
        ] {
            assert!(
                crate::lookup("msctf.dll", sym).is_some(),
                "msctf.dll!{sym} missing from builtin export table"
            );
        }

        std::thread::spawn(|| unsafe {
            // CoCreateInstance covers all 5 TSF CLSIDs and enforces aggregation/interface checks.
            for (clsid, iid) in [
                (CLSID_TF_THREADMGR, IID_ITFTHREADMGR),
                (CLSID_TF_INPUTPROCESSORPROFILES, IID_ITFINPUTPROCESSORPROFILES),
                (CLSID_TF_CATEGORYMGR, IID_ITFCATEGORYMGR),
                (CLSID_TF_DISPLAYATTRIBUTEMGR, IID_ITFDISPLAYATTRIBUTEMGR),
                (CLSID_TF_LANGBARMGR, IID_ITFLANGBARMGR),
            ] {
                let mut obj: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    super::super::ole32::CoCreateInstance(
                        clsid.as_ptr(),
                        std::ptr::null_mut(),
                        1,
                        iid.as_ptr(),
                        &mut obj,
                    ),
                    S_OK
                );
                assert!(!obj.is_null());
                com_call_release(obj);

                // Aggregation must fail cleanly with CLASS_E_NOAGGREGATION.
                assert_eq!(
                    super::super::ole32::CoCreateInstance(
                        clsid.as_ptr(),
                        0x1usize as *mut c_void,
                        1,
                        iid.as_ptr(),
                        &mut obj,
                    ),
                    CLASS_E_NOAGGREGATION
                );
            }

            // ITfKeystrokeMgr preserved keys, key event sink, and key down/up queries
            let mut tm: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);
            let tm_vt = &*(*(tm as *const *const ITfThreadMgrExVtbl));
            let mut cid: TfClientId = 0;
            assert_eq!((tm_vt.activate)(tm, &mut cid), S_OK);

            let mut ks: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(tm, IID_ITFKEYSTROKEMGR.as_ptr(), &mut ks), S_OK);
            let ks_vt = &*(*(ks as *const *const ITfKeystrokeMgrVtbl));

            let mut eaten: BOOL = TRUE;
            assert_eq!((ks_vt.test_key_down)(ks, 0x41, 0, &mut eaten), S_OK);
            assert_eq!(eaten, FALSE);
            assert_eq!((ks_vt.key_down)(ks, 0x41, 0, &mut eaten), S_OK);
            assert_eq!(eaten, FALSE);

            let pk_guid = guid(0xAABBCCDD, 0x1122, 0x3344, [5, 6, 7, 8, 9, 10, 11, 12]);
            let prekey = TF_PRESERVEDKEY { u_vkey: 0x20, u_modifiers: 0x0002 }; // Ctrl+Space
            let desc_w = crate::strings::to_wide("Toggle IME");
            assert_eq!(
                (ks_vt.preserve_key)(
                    ks,
                    cid,
                    pk_guid.as_ptr(),
                    (&prekey as *const TF_PRESERVEDKEY) as *const u8,
                    desc_w.as_ptr(),
                    desc_w.len() as u32,
                ),
                S_OK
            );
            let mut registered: BOOL = FALSE;
            assert_eq!(
                (ks_vt.is_preserved_key)(
                    ks,
                    pk_guid.as_ptr(),
                    (&prekey as *const TF_PRESERVEDKEY) as *const u8,
                    &mut registered,
                ),
                S_OK
            );
            assert_eq!(registered, TRUE);

            let mut bstr_desc: *mut u16 = std::ptr::null_mut();
            assert_eq!((ks_vt.get_preserved_key_description)(ks, pk_guid.as_ptr(), &mut bstr_desc), S_OK);
            assert_eq!(crate::strings::from_wide(bstr_desc), "Toggle IME");
            super::super::oleaut32::SysFreeString(bstr_desc);

            assert_eq!(
                (ks_vt.unpreserve_key)(ks, pk_guid.as_ptr(), (&prekey as *const TF_PRESERVEDKEY) as *const u8),
                S_OK
            );
            com_call_release(ks);

            // ITfDocumentMgr::EnumContexts + ITfContext::GetProperty + ITfRange + GetGlobalCompartment
            let mut dm: *mut c_void = std::ptr::null_mut();
            assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm), S_OK);
            let dm_vt = &*(*(dm as *const *const ITfDocumentMgrVtbl));
            let (mut ctx, mut ec) = (std::ptr::null_mut(), 0u32);
            assert_eq!((dm_vt.create_context)(dm, cid, 0, std::ptr::null_mut(), &mut ctx, &mut ec), S_OK);
            assert_eq!((dm_vt.push)(dm, ctx), S_OK);

            let mut enum_ctx: *mut c_void = std::ptr::null_mut();
            assert_eq!((dm_vt.enum_contexts)(dm, &mut enum_ctx), S_OK);
            let ec_vt = &*(*(enum_ctx as *const *const IEnumTfComVtbl));
            let mut got_ctx: *mut c_void = std::ptr::null_mut();
            let mut fetched = 0u32;
            assert_eq!((ec_vt.next)(enum_ctx as *mut EnumTfComObj, 1, &mut got_ctx, &mut fetched), S_OK);
            assert_eq!(fetched, 1);
            assert_eq!(got_ctx, ctx);
            com_call_release(got_ctx);
            com_call_release(enum_ctx);

            let ctx_vt = &*(*(ctx as *const *const ITfContextVtbl));
            let mut prop: *mut c_void = std::ptr::null_mut();
            assert_eq!((ctx_vt.get_property)(ctx, pk_guid.as_ptr(), &mut prop), S_OK);
            let prop_vt = &*(*(prop as *const *const ITfPropertyVtbl));
            let mut var_in = [0u8; 24];
            std::ptr::write_unaligned(var_in.as_mut_ptr() as *mut u16, 3u16); // VT_I4
            std::ptr::write_unaligned(var_in.as_mut_ptr().add(8) as *mut i32, 777);
            assert_eq!((prop_vt.set_value)(prop as *mut PropertyObj, ec, std::ptr::null_mut(), var_in.as_ptr()), S_OK);
            let mut var_out = [0u8; 24];
            assert_eq!((prop_vt.get_value)(prop as *mut PropertyObj, ec, std::ptr::null_mut(), var_out.as_mut_ptr()), S_OK);
            assert_eq!(std::ptr::read_unaligned(var_out.as_ptr().add(8) as *const i32), 777);
            com_call_release(prop);

            let mut start_range: *mut c_void = std::ptr::null_mut();
            assert_eq!((ctx_vt.get_start)(ctx, ec, &mut start_range), S_OK);
            let r_vt = &*(*(start_range as *const *const ITfRangeVtbl));
            let mut is_empty: BOOL = FALSE;
            assert_eq!((r_vt.is_empty)(start_range as *mut RangeObj, ec, &mut is_empty), S_OK);
            assert_eq!(is_empty, TRUE);
            com_call_release(start_range);

            let mut gcomp_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!((tm_vt.get_global_compartment)(tm, &mut gcomp_mgr), S_OK);
            assert!(!gcomp_mgr.is_null());
            com_call_release(gcomp_mgr);

            // ITfInputProcessorProfiles::EnumLanguageProfiles
            let mut profiles: *mut c_void = std::ptr::null_mut();
            assert_eq!(TF_CreateInputProcessorProfiles(&mut profiles), S_OK);
            let p_vt = &*(*(profiles as *const *const ITfInputProcessorProfilesVtbl));
            let mut lp_enum: *mut c_void = std::ptr::null_mut();
            assert_eq!((p_vt.enum_language_profiles)(profiles, LANGID_EN_US, &mut lp_enum), S_OK);
            let elp_vt = &*(*(lp_enum as *const *const IEnumTfLanguageProfilesVtbl));
            let mut lp = TF_LANGUAGEPROFILE::default();
            let mut lp_fetched = 0u32;
            assert_eq!(
                (elp_vt.next)(
                    lp_enum as *mut EnumLangProfilesObj,
                    1,
                    (&mut lp as *mut TF_LANGUAGEPROFILE) as *mut u8,
                    &mut lp_fetched,
                ),
                S_OK
            );
            assert_eq!(lp_fetched, 1);
            assert_eq!(lp.langid, LANGID_EN_US);
            assert_eq!(lp.f_active, TRUE);
            com_call_release(lp_enum);
            com_call_release(profiles);

            assert_eq!((dm_vt.pop)(dm, TF_POPF_ALL), S_OK);
            com_call_release(ctx);
            com_call_release(dm);
            assert_eq!((tm_vt.deactivate)(tm), S_OK);
            assert_eq!(com_call_release(tm), 0);
        })
        .join()
        .unwrap();
    }
}
