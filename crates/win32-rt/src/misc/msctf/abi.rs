//! Shared Win64 ABI definitions, TSF GUIDs/constants, VARIANT/sink helpers,
//! generic enumerators, and COM boilerplate macros for `msctf.dll`.

pub(super) use crate::com::{
    com_add_ref, com_call_add_ref, com_call_qi_bytes, com_call_release, com_call_slot, com_release,
    qi_helper_bytes, read_guid_from_bytes, register_vtable_ptrs, write_guid, write_null_out,
    ComObj, Guid, IID_IUnknown,
};
pub(super) use crate::strings::read_wide_len;
pub(super) use crate::types::*;

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Types, constants, HRESULTs, and GUIDs (Windows SDK msctf.h / ctfutb.h)
// ---------------------------------------------------------------------------

pub(super) type TfClientId = u32;
pub(super) type TfEditCookie = u32;
pub(super) type TfGuidAtom = u32;
pub(super) type HKL = usize;

#[inline]
pub(super) fn guid_as_ptr(g: &Guid) -> *const u8 {
    (g as *const Guid) as *const u8
}

pub(super) const GUID_NULL: Guid = Guid::ZERO;

// Standard COM IIDs
pub(super) const IID_IUNKNOWN: Guid = IID_IUnknown;
pub(super) const IID_IENUMGUID: Guid =
    Guid::new(0x0002E000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

// TSF CLSIDs
pub(super) const CLSID_TF_THREADMGR: Guid =
    Guid::new(0x529A9E6B, 0x6587, 0x4F23, [0xAB, 0x9E, 0x9C, 0x7D, 0x68, 0x3E, 0x3C, 0x50]);
pub(super) const CLSID_TF_INPUTPROCESSORPROFILES: Guid =
    Guid::new(0x33C53A50, 0xF456, 0x4884, [0xB0, 0x49, 0x85, 0xFD, 0x64, 0x3E, 0xCF, 0xED]);
pub(super) const CLSID_TF_CATEGORYMGR: Guid =
    Guid::new(0xA4B544A1, 0x438D, 0x4B41, [0x93, 0x25, 0x86, 0x95, 0x23, 0xE2, 0xD6, 0xC7]);
pub(super) const CLSID_TF_DISPLAYATTRIBUTEMGR: Guid =
    Guid::new(0x3CE74DE4, 0x53D3, 0x4D74, [0x8B, 0x83, 0x43, 0x1B, 0x38, 0x28, 0xBA, 0x53]);
pub(super) const CLSID_TF_LANGBARMGR: Guid =
    Guid::new(0xEBB08C45, 0x6C4A, 0x4FDC, [0xAE, 0x53, 0x4E, 0xB8, 0xC4, 0xC7, 0xDB, 0x8E]);

// TSF Category / Profile GUIDs
pub(super) const GUID_TFCAT_TIP_KEYBOARD: Guid =
    Guid::new(0x34745C63, 0xB2F0, 0x4784, [0x8B, 0x67, 0x5E, 0x12, 0xC8, 0x70, 0x1A, 0x31]);
/// Built-in default en-US keyboard profile identifiers.
pub(super) const CLSID_EN_US_KEYBOARD_TIP: Guid =
    Guid::new(0x0000897B, 0x83DF, 0x4B96, [0xBE, 0x07, 0x0F, 0xB5, 0x8B, 0x01, 0xC4, 0xA4]);
pub(super) const GUID_EN_US_KEYBOARD_PROFILE: Guid =
    Guid::new(0x09EA4E4B, 0x46CE, 0x4469, [0xB4, 0x50, 0x0D, 0x3A, 0x2F, 0x04, 0x09, 0x01]);

// Standard display attribute GUIDs
pub(super) const GUID_DA_INPUT: Guid =
    Guid::new(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x01]);
pub(super) const GUID_DA_TARGET_CONVERTED: Guid =
    Guid::new(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x02]);
pub(super) const GUID_DA_CONVERTED: Guid =
    Guid::new(0x9C755F70, 0x2E38, 0x48E3, [0x9F, 0x3C, 0x4F, 0x1B, 0x7A, 0x22, 0x00, 0x03]);

// TSF Interface IIDs (exact values from msctf.h / ctfutb.h)
pub(super) const IID_ITFTHREADMGR: Guid =
    Guid::new(0xAA80E801, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_ITFTHREADMGREX: Guid =
    Guid::new(0x3E90ADE3, 0x7594, 0x4CB0, [0xBB, 0x58, 0x69, 0x62, 0x8F, 0x5F, 0x45, 0x8C]);
pub(super) const IID_ITFTHREADMGR2: Guid =
    Guid::new(0x0AB198EF, 0x6477, 0x4EE8, [0x88, 0x12, 0x67, 0x80, 0xED, 0xB8, 0x2D, 0x5E]);
pub(super) const IID_ITFSOURCE: Guid =
    Guid::new(0x4EA48A35, 0x60AE, 0x446F, [0x8F, 0xD6, 0xE6, 0xA8, 0xD8, 0x24, 0x59, 0xF7]);
pub(super) const IID_ITFSOURCESINGLE: Guid =
    Guid::new(0x73131F9C, 0x56A9, 0x49DD, [0xB0, 0xEE, 0xD0, 0x46, 0x63, 0x3F, 0x75, 0x28]);
pub(super) const IID_ITFUIELEMENTMGR: Guid =
    Guid::new(0xEA1EA135, 0x19DF, 0x11D7, [0xA6, 0xD2, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
pub(super) const IID_ITFUIELEMENTSINK: Guid =
    Guid::new(0xEA1EA136, 0x19DF, 0x11D7, [0xA6, 0xD2, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
pub(super) const IID_IENUMTFUIELEMENTS: Guid =
    Guid::new(0x887AA91E, 0xACBA, 0x4931, [0x84, 0xDA, 0x3C, 0x52, 0x08, 0xCF, 0x54, 0x3F]);
pub(super) const IID_ITFCOMPARTMENTMGR: Guid =
    Guid::new(0x7DCF57AC, 0x18AD, 0x438B, [0x82, 0x4D, 0x97, 0x9B, 0xFF, 0xB7, 0x4B, 0x7C]);
pub(super) const IID_ITFCOMPARTMENT: Guid =
    Guid::new(0xBB08F7A9, 0x607A, 0x4384, [0x86, 0x23, 0x05, 0x68, 0x92, 0xB6, 0x43, 0x71]);
pub(super) const IID_ITFCOMPARTMENTEVENTSINK: Guid =
    Guid::new(0x743ABD5F, 0xF26D, 0x48DF, [0x8C, 0xC5, 0x23, 0x84, 0x92, 0x41, 0x9B, 0x64]);
pub(super) const IID_ITFKEYSTROKEMGR: Guid =
    Guid::new(0xAA80E7F0, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_ITFDOCUMENTMGR: Guid =
    Guid::new(0xAA80E7F4, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_IENUMTFDOCUMENTMGRS: Guid =
    Guid::new(0xAA80E808, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_ITFCONTEXT: Guid =
    Guid::new(0xAA80E7FD, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_IENUMTFCONTEXTS: Guid =
    Guid::new(0x8F1A7EA6, 0x1654, 0x4502, [0xA8, 0x6E, 0xB2, 0x90, 0x23, 0x44, 0xD5, 0x07]);
pub(super) const IID_IENUMTFCONTEXTVIEWS: Guid =
    Guid::new(0xF0C0F8DD, 0xCF38, 0x44E1, [0xBB, 0x0F, 0x68, 0xCF, 0x0D, 0x55, 0x1C, 0x78]);
pub(super) const IID_ITFPROPERTY: Guid =
    Guid::new(0xE2449660, 0x9542, 0x11D2, [0xBF, 0x46, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
pub(super) const IID_ITFREADONLYPROPERTY: Guid =
    Guid::new(0x17D49A3D, 0xF8B8, 0x4B2F, [0xB2, 0x54, 0x52, 0x31, 0x9D, 0xD6, 0x4C, 0x53]);
pub(super) const IID_IENUMTFPROPERTIES: Guid =
    Guid::new(0x19188CB0, 0xACA9, 0x11D2, [0xAF, 0xC5, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
pub(super) const IID_IENUMTFPROPERTYVALUE: Guid =
    Guid::new(0x8ED8981B, 0x7C10, 0x4D7D, [0x9F, 0xB3, 0xAB, 0x72, 0xE9, 0xC7, 0x5F, 0x72]);
pub(super) const IID_ITFRANGE: Guid =
    Guid::new(0xAA80E7FF, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_IENUMTFRANGES: Guid =
    Guid::new(0xF99D3F40, 0x8E32, 0x11D2, [0xBF, 0x46, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
pub(super) const IID_ITFTHREADMGREVENTSINK: Guid =
    Guid::new(0xAA80E80E, 0x2021, 0x11D2, [0x93, 0xE0, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E]);
pub(super) const IID_IENUMTFFUNCTIONPROVIDERS: Guid =
    Guid::new(0xE4B24DB0, 0x0990, 0x11D3, [0x8D, 0xF0, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);
pub(super) const IID_ITFINPUTPROCESSORPROFILES: Guid =
    Guid::new(0x1F02B6C5, 0x7842, 0x4EE6, [0x8A, 0x0B, 0x9A, 0x24, 0x18, 0x3A, 0x95, 0xCA]);
pub(super) const IID_ITFINPUTPROCESSORPROFILEMGR: Guid =
    Guid::new(0x71C6E74C, 0x0F28, 0x11D8, [0xA8, 0x2A, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
pub(super) const IID_IENUMTFLANGUAGEPROFILES: Guid =
    Guid::new(0x3D61BF11, 0xAC5F, 0x42C8, [0xA4, 0xCB, 0x93, 0x1B, 0xCC, 0x28, 0xC7, 0x44]);
pub(super) const IID_IENUMTFINPUTPROCESSORPROFILES: Guid =
    Guid::new(0x71C6E74D, 0x0F28, 0x11D8, [0xA8, 0x2A, 0x00, 0x06, 0x5B, 0x84, 0x43, 0x5C]);
pub(super) const IID_ITFCATEGORYMGR: Guid =
    Guid::new(0xC3ACEFB5, 0xF69D, 0x4905, [0x93, 0x8F, 0xFC, 0xAD, 0xCF, 0x4B, 0xE8, 0x30]);
pub(super) const IID_ITFDISPLAYATTRIBUTEMGR: Guid =
    Guid::new(0x8DED7393, 0x5DB1, 0x475C, [0x9E, 0x71, 0xA3, 0x91, 0x11, 0xB0, 0xFF, 0x67]);
pub(super) const IID_ITFDISPLAYATTRIBUTEINFO: Guid =
    Guid::new(0x70528852, 0x2F26, 0x4AEA, [0x8C, 0x96, 0x21, 0x51, 0x50, 0x57, 0x89, 0x32]);
pub(super) const IID_IENUMTFDISPLAYATTRIBUTEINFO: Guid =
    Guid::new(0x7CEF04D7, 0xCB75, 0x4E80, [0xA7, 0xAB, 0x5F, 0x5B, 0xC7, 0xD3, 0x32, 0xDE]);
pub(super) const IID_ITFLANGBARMGR: Guid =
    Guid::new(0x87955690, 0xE627, 0x11D2, [0x8D, 0xDB, 0x00, 0x10, 0x5A, 0x27, 0x99, 0xB5]);

// HRESULTs
pub(super) const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110u32 as i32;
pub(super) const E_UNEXPECTED: HRESULT = 0x8000_FFFFu32 as i32;
pub(super) const CONNECT_E_NOCONNECTION: HRESULT = 0x8004_0200u32 as i32;
pub(super) const CONNECT_E_ADVISELIMIT: HRESULT = 0x8004_0201u32 as i32;
pub(super) const CONNECT_E_CANNOTCONNECT: HRESULT = 0x8004_0202u32 as i32;
pub(super) const TF_E_STACKFULL: HRESULT = 0x8004_0501u32 as i32;
pub(super) const TF_E_NOPROVIDER: HRESULT = 0x8004_0503u32 as i32;
pub(super) const TF_E_DISCONNECTED: HRESULT = 0x8004_0504u32 as i32;
pub(super) const TF_E_INVALIDVIEW: HRESULT = 0x8004_0505u32 as i32;
pub(super) const TF_E_ALREADY_EXISTS: HRESULT = 0x8004_0506u32 as i32;
pub(super) const TF_E_NOLOCK: HRESULT = 0x8004_0201u32 as i32;
pub(super) const TF_E_NOSELECTION: HRESULT = 0x8004_0205u32 as i32;
pub(super) const TF_E_SYNCHRONOUS: HRESULT = 0x8004_0208u32 as i32;

// TSF flags & constants
pub(super) const TF_INVALID_COOKIE: u32 = 0xFFFF_FFFF;
pub(super) const TF_INVALID_EDIT_COOKIE: TfEditCookie = 0;
pub(super) const TF_INVALID_GUIDATOM: TfGuidAtom = 0;
pub(super) const TF_TMF_ACTIVATED: u32 = 0x8000_0000;
pub(super) const TF_POPF_ALL: u32 = 0x0000_0001;
pub(super) const TF_ES_SYNC: u32 = 0x0000_0001;
pub(super) const TF_ES_READ: u32 = 0x0000_0002;
pub(super) const TF_ES_READWRITE: u32 = 0x0000_0006;
pub(super) const TF_ES_ASYNC: u32 = 0x0000_0008;
pub(super) const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 0x0001;
pub(super) const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 0x0002;
pub(super) const TF_IPP_FLAG_ACTIVE: u32 = 0x0001;
pub(super) const TF_IPP_FLAG_ENABLED: u32 = 0x0002;
pub(super) const LANGID_EN_US: u16 = 0x0409;
pub(super) const HKL_EN_US: HKL = 0x0409_0409;

// TF_DA_* constants
pub(super) const TF_CT_NONE: i32 = 0;
pub(super) const TF_CT_SYSCOLOR: i32 = 1;
pub(super) const TF_LS_NONE: i32 = 0;
pub(super) const TF_LS_SOLID: i32 = 1;
pub(super) const TF_LS_DOT: i32 = 2;
pub(super) const TF_ATTR_INPUT: i32 = 0;
pub(super) const TF_ATTR_TARGET_CONVERTED: i32 = 1;
pub(super) const TF_ATTR_CONVERTED: i32 = 2;

// VARIANT VT_* constants
pub(super) const VT_EMPTY: u16 = 0;
pub(super) const VT_BSTR: u16 = 8;
pub(super) const VT_DISPATCH: u16 = 9;
pub(super) const VT_UNKNOWN: u16 = 13;

// ---------------------------------------------------------------------------
// Win64 ABI structs (verified against msctf.h / textstor.h)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_STATUS {
    pub(super) dw_dynamic_flags: u32,
    pub(super) dw_static_flags: u32,
}
const _: () = assert!(std::mem::size_of::<TF_STATUS>() == 8);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct TF_PRESERVEDKEY {
    pub(super) u_vkey: u32,
    pub(super) u_modifiers: u32,
}
const _: () = assert!(std::mem::size_of::<TF_PRESERVEDKEY>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_SELECTIONSTYLE {
    pub(super) ase: i32,
    pub(super) f_interim_char: BOOL,
}
const _: () = assert!(std::mem::size_of::<TF_SELECTIONSTYLE>() == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct TF_SELECTION {
    pub(super) range: *mut c_void,
    pub(super) style: TF_SELECTIONSTYLE,
}
const _: () = assert!(std::mem::size_of::<TF_SELECTION>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_LANGUAGEPROFILE {
    pub(super) clsid: Guid,
    pub(super) langid: u16,
    pub(super) catid: Guid,
    pub(super) f_active: BOOL,
    pub(super) guid_profile: Guid,
}
const _: () = assert!(std::mem::size_of::<TF_LANGUAGEPROFILE>() == 56);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, langid) == 16);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, catid) == 20);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, f_active) == 36);
const _: () = assert!(std::mem::offset_of!(TF_LANGUAGEPROFILE, guid_profile) == 40);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_INPUTPROCESSORPROFILE {
    pub(super) dw_profile_type: u32,
    pub(super) langid: u16,
    pub(super) clsid: Guid,
    pub(super) guid_profile: Guid,
    pub(super) catid: Guid,
    pub(super) hkl_substitute: HKL,
    pub(super) dw_caps: u32,
    pub(super) hkl: HKL,
    pub(super) dw_flags: u32,
}
const _: () = assert!(std::mem::size_of::<TF_INPUTPROCESSORPROFILE>() == 88);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, clsid) == 8);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, hkl_substitute) == 56);
const _: () = assert!(std::mem::offset_of!(TF_INPUTPROCESSORPROFILE, hkl) == 72);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_DA_COLOR {
    pub(super) ty: i32,
    pub(super) val: u32,
}
const _: () = assert!(std::mem::size_of::<TF_DA_COLOR>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct TF_DISPLAYATTRIBUTE {
    pub(super) cr_text: TF_DA_COLOR,
    pub(super) cr_bk: TF_DA_COLOR,
    pub(super) ls_style: i32,
    pub(super) f_bold_line: BOOL,
    pub(super) cr_line: TF_DA_COLOR,
    pub(super) b_attr: i32,
}
const _: () = assert!(std::mem::size_of::<TF_DISPLAYATTRIBUTE>() == 36);

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct TF_PROPERTYVAL {
    pub(super) guid_id: Guid,
    pub(super) var_value: [u8; 24],
}
const _: () = assert!(std::mem::size_of::<TF_PROPERTYVAL>() == 40);
const _: () = assert!(std::mem::offset_of!(TF_PROPERTYVAL, var_value) == 16);

// ---------------------------------------------------------------------------
// Shared COM vtable structs (`ITfSource`, `ITfSourceSingle`, `ITfCompartmentMgr`)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct ITfSourceVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) advise_sink:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut c_void, *mut u32) -> HRESULT,
    pub(super) unadvise_sink: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<ITfSourceVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
pub(super) struct ITfSourceSingleVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) advise_single_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8, *mut c_void) -> HRESULT,
    pub(super) unadvise_single_sink:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfSourceSingleVtbl>() == 5 * std::mem::size_of::<usize>());

#[repr(C)]
pub(super) struct ITfCompartmentMgrVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut c_void) -> u32,
    pub(super) get_compartment:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) clear_compartment:
        unsafe extern "win64" fn(*mut c_void, TfClientId, *const u8) -> HRESULT,
    pub(super) enum_compartments: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
const _: () =
    assert!(std::mem::size_of::<ITfCompartmentMgrVtbl>() == 6 * std::mem::size_of::<usize>());

// ---------------------------------------------------------------------------
// COM IUnknown boilerplate helpers & local macros over `crate::com`
// ---------------------------------------------------------------------------

/// Multi-interface `QueryInterface` helper for multi-vtable TSF objects.
/// Always answers `IID_IUnknown` with `base` (COM identity rule).
pub(super) unsafe fn qi_multi_iface(
    refs: &AtomicU32,
    riid: *const u8,
    ppv: *mut *mut c_void,
    base: *mut c_void,
    entries: &[(Guid, *mut c_void)],
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    let target = if iid == IID_IUNKNOWN {
        base
    } else if let Some(&(_, ptr)) = entries.iter().find(|&&(g, _)| g == iid) {
        ptr
    } else {
        return E_NOINTERFACE;
    };
    refs.fetch_add(1, Ordering::AcqRel);
    std::ptr::write_unaligned(ppv, target);
    S_OK
}

/// Generate a single-interface `IUnknown::QueryInterface` thunk backed by `crate::com::qi_helper_bytes`.
macro_rules! impl_single_qi {
    ($qi:ident, $obj:ty, $iid:expr) => {
        unsafe extern "win64" fn $qi(
            this: *mut $obj,
            riid: *const u8,
            ppv: *mut *mut std::ffi::c_void,
        ) -> crate::types::HRESULT {
            crate::com::qi_helper_bytes(this, riid, ppv, &[$iid])
        }
    };
}
pub(super) use impl_single_qi;

/// Generate offset-0 `IUnknown` thunks (`QueryInterface`, `AddRef`, `Release`) for a multi-vtable COM object.
macro_rules! impl_primary_iunknown {
    ($qi:ident, $add_ref:ident, $release:ident, $obj:ty, $qi_common:path, $add_ref_common:path, $release_common:path) => {
        unsafe extern "win64" fn $qi(
            this: *mut std::ffi::c_void,
            riid: *const u8,
            ppv: *mut *mut std::ffi::c_void,
        ) -> crate::types::HRESULT {
            $qi_common(this as *mut $obj, riid, ppv)
        }
        unsafe extern "win64" fn $add_ref(this: *mut std::ffi::c_void) -> u32 {
            $add_ref_common(this as *mut $obj)
        }
        unsafe extern "win64" fn $release(this: *mut std::ffi::c_void) -> u32 {
            $release_common(this as *mut $obj)
        }
    };
}
pub(super) use impl_primary_iunknown;

/// Generate container-of adjustment + `IUnknown` thunks for a secondary embedded vtable on a multi-vtable COM object.
macro_rules! impl_sub_iunknown {
    ($from:ident, $qi:ident, $add_ref:ident, $release:ident, $obj:ty, $field:ident, $qi_common:path, $add_ref_common:path, $release_common:path) => {
        #[inline]
        unsafe fn $from(this: *mut std::ffi::c_void) -> *mut $obj {
            (this as *mut u8).sub(std::mem::offset_of!($obj, $field)) as *mut $obj
        }
        unsafe extern "win64" fn $qi(
            this: *mut std::ffi::c_void,
            riid: *const u8,
            ppv: *mut *mut std::ffi::c_void,
        ) -> crate::types::HRESULT {
            $qi_common($from(this), riid, ppv)
        }
        unsafe extern "win64" fn $add_ref(this: *mut std::ffi::c_void) -> u32 {
            $add_ref_common($from(this))
        }
        unsafe extern "win64" fn $release(this: *mut std::ffi::c_void) -> u32 {
            $release_common($from(this))
        }
    };
}
pub(super) use impl_sub_iunknown;

// ---------------------------------------------------------------------------
// BSTR & Owned Win64 VARIANT storage (24 bytes)
// ---------------------------------------------------------------------------

pub(super) unsafe fn alloc_bstr_str(s: &str) -> *mut u16 {
    let w = crate::strings::to_wide_z(s);
    crate::misc::oleaut32::SysAllocString(w.as_ptr())
}

pub(super) struct OwnedVariant([u8; 24]);

impl Default for OwnedVariant {
    fn default() -> Self {
        Self([0u8; 24])
    }
}

impl OwnedVariant {
    pub(super) fn is_empty(&self) -> bool {
        u16::from_le_bytes([self.0[0], self.0[1]]) == VT_EMPTY
    }

    pub(super) unsafe fn from_raw(src: *const u8) -> Self {
        let mut raw = [0u8; 24];
        std::ptr::copy_nonoverlapping(src, raw.as_mut_ptr(), 24);
        let vt = u16::from_le_bytes([raw[0], raw[1]]);
        match vt {
            VT_BSTR => {
                let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
                let dup = if s.is_null() {
                    std::ptr::null_mut()
                } else {
                    crate::misc::oleaut32::SysAllocStringLen(
                        s,
                        crate::misc::oleaut32::SysStringLen(s),
                    )
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

    pub(super) unsafe fn write_copy_to(&self, dst: *mut u8) {
        let mut raw = self.0;
        let vt = u16::from_le_bytes([raw[0], raw[1]]);
        match vt {
            VT_BSTR => {
                let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
                let dup = if s.is_null() {
                    std::ptr::null_mut()
                } else {
                    crate::misc::oleaut32::SysAllocStringLen(
                        s,
                        crate::misc::oleaut32::SysStringLen(s),
                    )
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

    pub(super) fn take_raw(&mut self) -> [u8; 24] {
        std::mem::replace(&mut self.0, [0u8; 24])
    }
}

pub(super) unsafe fn release_variant_raw(raw: [u8; 24]) {
    let vt = u16::from_le_bytes([raw[0], raw[1]]);
    match vt {
        VT_BSTR => {
            let s = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *mut u16);
            if !s.is_null() {
                crate::misc::oleaut32::SysFreeString(s);
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

pub(super) struct SinkTable {
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

    pub(super) fn drain_all(&mut self) -> Vec<*mut c_void> {
        self.entries.drain().map(|(_, e)| e.punk as *mut c_void).collect()
    }
}

/// Acquire `riid` from `punk` via COM `QueryInterface` and register it in `table`.
/// Performs COM QI outside the mutex lock so guest callbacks cannot deadlock.
pub(super) unsafe fn sink_advise(
    table: &Mutex<SinkTable>,
    riid: *const u8,
    punk: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    if !pdw_cookie.is_null() {
        std::ptr::write_unaligned(pdw_cookie, TF_INVALID_COOKIE);
    }
    let Some(iid) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    if punk.is_null() || pdw_cookie.is_null() {
        return E_INVALIDARG;
    }
    let mut iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi_bytes(punk, riid, &mut iface);
    if hr < 0 || iface.is_null() {
        return CONNECT_E_CANNOTCONNECT;
    }
    let cookie = table.lock().unwrap().insert(iid, iface);
    std::ptr::write_unaligned(pdw_cookie, cookie);
    S_OK
}

pub(super) unsafe fn sink_unadvise(table: &Mutex<SinkTable>, cookie: u32) -> HRESULT {
    let removed = table.lock().unwrap().remove(cookie);
    match removed {
        Some(ptr) => {
            com_call_release(ptr);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

/// Snapshot matching sinks and `AddRef` each before notifying callers so a sink
/// callback that invokes `UnadviseSink` on itself stays alive for the call.
pub(super) unsafe fn snapshot_sinks_addref(
    table: &Mutex<SinkTable>,
    iid: &Guid,
) -> Vec<*mut c_void> {
    let raw = table.lock().unwrap().snapshot_for_iid(iid);
    for &p in &raw {
        com_call_add_ref(p);
    }
    raw
}

pub(super) unsafe fn release_all_ptrs(ptrs: Vec<*mut c_void>) {
    for p in ptrs {
        com_call_release(p);
    }
}

// ---------------------------------------------------------------------------
// Generic TSF COM pointer enumerator (`IEnumTf*`: Clone, Next, Reset, Skip)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct IEnumTfComVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut EnumTfComObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut EnumTfComObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut EnumTfComObj) -> u32,
    pub(super) clone: unsafe extern "win64" fn(*mut EnumTfComObj, *mut *mut c_void) -> HRESULT,
    pub(super) next:
        unsafe extern "win64" fn(*mut EnumTfComObj, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    pub(super) reset: unsafe extern "win64" fn(*mut EnumTfComObj) -> HRESULT,
    pub(super) skip: unsafe extern "win64" fn(*mut EnumTfComObj, u32) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<IEnumTfComVtbl>() == 7 * std::mem::size_of::<usize>());

pub(super) struct EnumTfComInner {
    enum_iid: Guid,
    state: Mutex<(Vec<usize>, usize)>,
}

impl Drop for EnumTfComInner {
    fn drop(&mut self) {
        let items: Vec<*mut c_void> = {
            let mut st = self.state.lock().unwrap();
            st.0.drain(..).map(|p| p as *mut c_void).collect()
        };
        unsafe { release_all_ptrs(items) };
    }
}

pub(super) type EnumTfComObj = ComObj<EnumTfComInner>;

unsafe extern "win64" fn enum_tf_com_qi(
    this: *mut EnumTfComObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() {
        write_null_out(ppv);
        return E_INVALIDARG;
    }
    qi_helper_bytes(this, riid, ppv, &[ComObj::inner(this).enum_iid])
}

unsafe extern "win64" fn enum_tf_com_clone(
    this: *mut EnumTfComObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let inner = ComObj::inner(this);
    let (items, pos) = {
        let st = inner.state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    for &p in &items {
        com_call_add_ref(p as *mut c_void);
    }
    let cloned = new_enum_tf_com(inner.enum_iid, items, pos);
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
    let inner = ComObj::inner(this);
    let picked: Vec<usize> = {
        let mut st = inner.state.lock().unwrap();
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
    ComObj::inner(this).state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn enum_tf_com_skip(this: *mut EnumTfComObj, ul_count: u32) -> HRESULT {
    let mut st = ComObj::inner(this).state.lock().unwrap();
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
    add_ref: com_add_ref::<EnumTfComInner>,
    release: com_release::<EnumTfComInner>,
    clone: enum_tf_com_clone,
    next: enum_tf_com_next,
    reset: enum_tf_com_reset,
    skip: enum_tf_com_skip,
};

fn register_enum_tf_com_vtable() {
    register_vtable_ptrs(&[
        enum_tf_com_qi as *const () as usize,
        com_add_ref::<EnumTfComInner> as *const () as usize,
        com_release::<EnumTfComInner> as *const () as usize,
        enum_tf_com_clone as *const () as usize,
        enum_tf_com_next as *const () as usize,
        enum_tf_com_reset as *const () as usize,
        enum_tf_com_skip as *const () as usize,
    ]);
}

/// Takes ownership of one ref per element in `items`.
pub(super) fn new_enum_tf_com(enum_iid: Guid, items: Vec<usize>, pos: usize) -> *mut EnumTfComObj {
    register_enum_tf_com_vtable();
    ComObj::new(
        &IENUM_TF_COM_VTBL as *const IEnumTfComVtbl as *const usize,
        EnumTfComInner { enum_iid, state: Mutex::new((items, pos)) },
    )
}

// ---------------------------------------------------------------------------
// Standard COM `IEnumGUID` (Next, Skip, Reset, Clone — comcat.h order!)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct IEnumGUIDVtbl {
    pub(super) query_interface:
        unsafe extern "win64" fn(*mut EnumGuidObj, *const u8, *mut *mut c_void) -> HRESULT,
    pub(super) add_ref: unsafe extern "win64" fn(*mut EnumGuidObj) -> u32,
    pub(super) release: unsafe extern "win64" fn(*mut EnumGuidObj) -> u32,
    pub(super) next: unsafe extern "win64" fn(*mut EnumGuidObj, u32, *mut u8, *mut u32) -> HRESULT,
    pub(super) skip: unsafe extern "win64" fn(*mut EnumGuidObj, u32) -> HRESULT,
    pub(super) reset: unsafe extern "win64" fn(*mut EnumGuidObj) -> HRESULT,
    pub(super) clone: unsafe extern "win64" fn(*mut EnumGuidObj, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<IEnumGUIDVtbl>() == 7 * std::mem::size_of::<usize>());

pub(super) struct EnumGuidInner {
    state: Mutex<(Vec<Guid>, usize)>,
}

pub(super) type EnumGuidObj = ComObj<EnumGuidInner>;

impl_single_qi!(enum_guid_qi, EnumGuidObj, IID_IENUMGUID);

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
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let avail = st.0.len().saturating_sub(st.1);
    let n = (celt as usize).min(avail);
    for i in 0..n {
        write_guid(rgelt.add(i * 16) as *mut Guid, st.0[st.1 + i]);
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
    let mut st = ComObj::inner(this).state.lock().unwrap();
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
    ComObj::inner(this).state.lock().unwrap().1 = 0;
    S_OK
}

unsafe extern "win64" fn enum_guid_clone(
    this: *mut EnumGuidObj,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_INVALIDARG;
    }
    write_null_out(pp_enum);
    let (items, pos) = {
        let st = ComObj::inner(this).state.lock().unwrap();
        (st.0.clone(), st.1)
    };
    let cloned = new_enum_guid(items, pos);
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

static IENUM_GUID_VTBL: IEnumGUIDVtbl = IEnumGUIDVtbl {
    query_interface: enum_guid_qi,
    add_ref: com_add_ref::<EnumGuidInner>,
    release: com_release::<EnumGuidInner>,
    next: enum_guid_next,
    skip: enum_guid_skip,
    reset: enum_guid_reset,
    clone: enum_guid_clone,
};

pub(super) fn new_enum_guid(items: Vec<Guid>, pos: usize) -> *mut EnumGuidObj {
    register_vtable_ptrs(&[
        enum_guid_qi as *const () as usize,
        com_add_ref::<EnumGuidInner> as *const () as usize,
        com_release::<EnumGuidInner> as *const () as usize,
        enum_guid_next as *const () as usize,
        enum_guid_skip as *const () as usize,
        enum_guid_reset as *const () as usize,
        enum_guid_clone as *const () as usize,
    ]);
    ComObj::new(
        &IENUM_GUID_VTBL as *const IEnumGUIDVtbl as *const usize,
        EnumGuidInner { state: Mutex::new((items, pos)) },
    )
}
