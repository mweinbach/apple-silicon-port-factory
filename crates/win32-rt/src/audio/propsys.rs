//! `IPropertyStore` for audio endpoints (read-only, as opened with STGM_READ).
//!
//! Keys the game reads (verified in the exe): `PKEY_AudioEngine_DeviceFormat`
//! (VT_BLOB WAVEFORMATEX, Wwise sink `FUN_141652ce0`), `PKEY_Device_FriendlyName`
//! (VT_LPWSTR, AK `FUN_14158d5c0`), `PKEY_Device_ContainerId` (VT_CLSID, Remedy
//! `FUN_1424d25a0` and the Wwise motion sink `FUN_14152b600`). A few more
//! common keys are served for completeness; any other key returns S_OK with
//! VT_EMPTY, which is what Windows does for absent properties.

use super::com::*;
use super::mmdevapi::Endpoint;
use crate::types::*;
use std::ffi::c_void;
use std::sync::Arc;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PROPERTYKEY {
    pub fmtid: GUID,
    pub pid: u32,
}

const fn pkey(fmtid: GUID, pid: u32) -> PROPERTYKEY {
    PROPERTYKEY { fmtid, pid }
}

const FMTID_DEVICE: GUID = GUID::new(0xA45C254E, 0xDF1C, 0x4EFD, [0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0]);
const FMTID_AUDIOENDPOINT: GUID = GUID::new(0x1DA5D803, 0xD492, 0x4EDD, [0x8C, 0x23, 0xE0, 0xC0, 0xFF, 0xEE, 0x7F, 0x0E]);
pub const PKEY_Device_DeviceDesc: PROPERTYKEY = pkey(FMTID_DEVICE, 2);
pub const PKEY_Device_FriendlyName: PROPERTYKEY = pkey(FMTID_DEVICE, 14);
pub const PKEY_DeviceInterface_FriendlyName: PROPERTYKEY =
    pkey(GUID::new(0x026E516E, 0xB814, 0x414B, [0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22]), 2);
pub const PKEY_Device_ContainerId: PROPERTYKEY =
    pkey(GUID::new(0x8C7ED206, 0x3F8A, 0x4827, [0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C]), 2);
pub const PKEY_AudioEngine_DeviceFormat: PROPERTYKEY =
    pkey(GUID::new(0xF19F064D, 0x082C, 0x4E27, [0xBC, 0x73, 0x68, 0x82, 0xA1, 0xBB, 0x8E, 0x4C]), 0);
pub const PKEY_AudioEngine_OEMFormat: PROPERTYKEY =
    pkey(GUID::new(0xE4870E26, 0x3CC5, 0x4CD2, [0xBA, 0x46, 0xCA, 0x0A, 0x9A, 0x70, 0xED, 0x04]), 3);
pub const PKEY_AudioEndpoint_FormFactor: PROPERTYKEY = pkey(FMTID_AUDIOENDPOINT, 0);
pub const PKEY_AudioEndpoint_PhysicalSpeakers: PROPERTYKEY = pkey(FMTID_AUDIOENDPOINT, 3);
pub const PKEY_AudioEndpoint_GUID: PROPERTYKEY = pkey(FMTID_AUDIOENDPOINT, 4);

const KEYS: &[PROPERTYKEY] = &[
    PKEY_Device_FriendlyName,
    PKEY_Device_DeviceDesc,
    PKEY_DeviceInterface_FriendlyName,
    PKEY_Device_ContainerId,
    PKEY_AudioEngine_DeviceFormat,
    PKEY_AudioEngine_OEMFormat,
    PKEY_AudioEndpoint_FormFactor,
    PKEY_AudioEndpoint_PhysicalSpeakers,
    PKEY_AudioEndpoint_GUID,
];

pub const VT_EMPTY: u16 = 0;
pub const VT_UI4: u16 = 19;
pub const VT_LPWSTR: u16 = 31;
pub const VT_BLOB: u16 = 65;
pub const VT_CLSID: u16 = 72;

/// Write a PROPVARIANT (24 bytes) at `pv`. Payload memory is CoTaskMemAlloc'd.
pub enum Value {
    Empty,
    U32(u32),
    Str(String),
    Blob(Vec<u8>),
    Clsid(GUID),
}

pub unsafe fn write_propvariant(pv: *mut u8, v: &Value) -> HRESULT {
    std::ptr::write_bytes(pv, 0, 24);
    let (vt, ok) = match v {
        Value::Empty => (VT_EMPTY, true),
        Value::U32(x) => {
            std::ptr::write_unaligned(pv.add(8) as *mut u32, *x);
            (VT_UI4, true)
        }
        Value::Str(s) => {
            let p = co_wstr(s);
            std::ptr::write_unaligned(pv.add(8) as *mut *mut u16, p);
            (VT_LPWSTR, !p.is_null())
        }
        Value::Blob(b) => {
            let p = co_alloc(b.len());
            if !p.is_null() {
                std::ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
            }
            std::ptr::write_unaligned(pv.add(8) as *mut u32, b.len() as u32);
            std::ptr::write_unaligned(pv.add(16) as *mut *mut u8, p);
            (VT_BLOB, !p.is_null())
        }
        Value::Clsid(g) => {
            let p = co_alloc(16) as *mut GUID;
            if !p.is_null() {
                std::ptr::write_unaligned(p, *g);
            }
            std::ptr::write_unaligned(pv.add(8) as *mut *mut GUID, p);
            (VT_CLSID, !p.is_null())
        }
    };
    if !ok {
        std::ptr::write_bytes(pv, 0, 24);
        return E_OUTOFMEMORY;
    }
    std::ptr::write_unaligned(pv as *mut u16, vt);
    S_OK
}

pub fn value_for(ep: &Endpoint, key: &PROPERTYKEY) -> Value {
    let k = *key;
    if k == PKEY_Device_FriendlyName {
        Value::Str(ep.friendly_name())
    } else if k == PKEY_Device_DeviceDesc {
        Value::Str(ep.info.name.clone())
    } else if k == PKEY_DeviceInterface_FriendlyName {
        Value::Str(ep.info.name.clone())
    } else if k == PKEY_Device_ContainerId {
        Value::Clsid(ep.container_id)
    } else if k == PKEY_AudioEngine_DeviceFormat || k == PKEY_AudioEngine_OEMFormat {
        Value::Blob(ep.mix_format().extensible_bytes().to_vec())
    } else if k == PKEY_AudioEndpoint_FormFactor {
        let ff = if ep.is_capture() {
            4 // Microphone
        } else if ep.info.headphones {
            3 // Headphones
        } else {
            1 // Speakers
        };
        Value::U32(ff)
    } else if k == PKEY_AudioEndpoint_PhysicalSpeakers {
        Value::U32(ep.mix_format().mask)
    } else if k == PKEY_AudioEndpoint_GUID {
        Value::Str(format!("{:?}", ep.guid))
    } else {
        Value::Empty
    }
}

// --- IPropertyStore -------------------------------------------------------

#[repr(C)]
pub struct IPropertyStoreVtbl {
    qi: unsafe extern "win64" fn(*mut StoreObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut StoreObj) -> u32,
    release: unsafe extern "win64" fn(*mut StoreObj) -> u32,
    get_count: unsafe extern "win64" fn(*mut StoreObj, *mut u32) -> HRESULT,
    get_at: unsafe extern "win64" fn(*mut StoreObj, u32, *mut PROPERTYKEY) -> HRESULT,
    get_value: unsafe extern "win64" fn(*mut StoreObj, *const PROPERTYKEY, *mut u8) -> HRESULT,
    set_value: unsafe extern "win64" fn(*mut StoreObj, *const PROPERTYKEY, *const u8) -> HRESULT,
    commit: unsafe extern "win64" fn(*mut StoreObj) -> HRESULT,
}

pub type StoreObj = ComObj<IPropertyStoreVtbl, Arc<Endpoint>>;

mod store {
    use super::*;
    crate::com_iunknown!(StoreObj, [IID_IPropertyStore]);

    pub unsafe extern "win64" fn get_count(_this: *mut StoreObj, n: *mut u32) -> HRESULT {
        write_out(n, KEYS.len() as u32)
    }
    pub unsafe extern "win64" fn get_at(_this: *mut StoreObj, i: u32, key: *mut PROPERTYKEY) -> HRESULT {
        if key.is_null() {
            return E_POINTER;
        }
        match KEYS.get(i as usize) {
            Some(k) => write_out(key, *k),
            None => E_INVALIDARG,
        }
    }
    pub unsafe extern "win64" fn get_value(this: *mut StoreObj, key: *const PROPERTYKEY, pv: *mut u8) -> HRESULT {
        if key.is_null() || pv.is_null() {
            return E_POINTER;
        }
        let k = std::ptr::read_unaligned(key);
        let v = value_for(StoreObj::inner(this), &k);
        if matches!(v, Value::Empty) {
            log::debug!(target: "audio", "IPropertyStore::GetValue({:?},{}) -> VT_EMPTY", k.fmtid, k.pid);
        }
        write_propvariant(pv, &v)
    }
    pub unsafe extern "win64" fn set_value(_this: *mut StoreObj, _key: *const PROPERTYKEY, _pv: *const u8) -> HRESULT {
        STG_E_ACCESSDENIED
    }
    pub unsafe extern "win64" fn commit(_this: *mut StoreObj) -> HRESULT {
        STG_E_ACCESSDENIED
    }
}

static STORE_VTBL: IPropertyStoreVtbl = IPropertyStoreVtbl {
    qi: store::qi,
    add_ref: store::add_ref,
    release: store::release,
    get_count: store::get_count,
    get_at: store::get_at,
    get_value: store::get_value,
    set_value: store::set_value,
    commit: store::commit,
};

pub fn new_store(ep: Arc<Endpoint>) -> *mut StoreObj {
    StoreObj::create(&STORE_VTBL, ep)
}
