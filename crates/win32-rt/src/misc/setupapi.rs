//! setupapi.dll: device enumeration.
//!
//! Only HID devices exist (from `misc::hid`, which the platform backend
//! fills with game controllers). `SetupDiGetClassDevsW` snapshots them for
//! `GUID_DEVINTERFACE_HID` (with `DIGCF_DEVICEINTERFACE`) or the HIDClass
//! setup class; any other class gives a valid, empty set.
//!
//! Device `i` of a set exposes exactly one interface, interface `i`, as on
//! Windows for single-collection HID devices; callers such as the game's pad
//! library rely on interface and device indices lining up.

pub use crate::audio::mmdevapi::{dualsense_usb_container_id, format_guid};
use crate::misc::hid::{self, Device, GUID_DEVINTERFACE_HID};
use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_NO_MORE_ITEMS: u32 = 259;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_INVALID_DATA: u32 = 13;
const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_INVALID_USER_BUFFER: u32 = 1784;
const ERROR_INVALID_FLAGS: u32 = 1004;
const ERROR_NOT_FOUND: u32 = 1168;
const ERROR_NO_SUCH_DEVICE_INTERFACE: u32 = 0xE000_022E;

const DIGCF_ALLCLASSES: u32 = 0x4;
const DIGCF_DEVICEINTERFACE: u32 = 0x10;
const DIODI_NO_ADD: u32 = 0x1;
const SPINT_ACTIVE: u32 = 1;
const REG_SZ: u32 = 1;
const REG_MULTI_SZ: u32 = 7;
const SPDRP_BASE_CONTAINERID: u32 = 0x24;
const DEVPROP_TYPE_GUID: u32 = 13;
const DEVPROP_TYPE_BOOLEAN: u32 = 17;
const DEVPROP_TYPE_STRING: u32 = 18;

/// HIDClass setup class {745A17A0-74D3-11D0-B6FE-00A0C90F57DA}.
const GUID_HIDCLASS: [u8; 16] = [0xA0, 0x17, 0x5A, 0x74, 0xD3, 0x74, 0xD0, 0x11, 0xB6, 0xFE, 0x00, 0xA0, 0xC9, 0x0F, 0x57, 0xDA];

/// DEVPKEY_Device_ContainerId {8C7ED206-3F8A-4827-B3AB-AE9E1FAEFC6C}, 2.
const DEVPKEY_DEVICE_CONTAINERID_FMTID: [u8; 16] =
    [0x06, 0xD2, 0x7E, 0x8C, 0x8A, 0x3F, 0x27, 0x48, 0xB3, 0xAB, 0xAE, 0x9E, 0x1F, 0xAE, 0xFC, 0x6C];
const DEVPKEY_DEVICE_CONTAINERID_PID: u32 = 2;

/// DEVPKEY_Device_InstanceId {78C34FC8-104A-4ACA-9EA4-524D52996E57}, 256.
const DEVPKEY_DEVICE_INSTANCEID_FMTID: [u8; 16] =
    [0xC8, 0x4F, 0xC3, 0x78, 0x4A, 0x10, 0xCA, 0x4A, 0x9E, 0xA4, 0x52, 0x4D, 0x52, 0x99, 0x6E, 0x57];
const DEVPKEY_DEVICE_INSTANCEID_PID: u32 = 256;

/// DEVPKEY_DeviceInterface_Enabled (pid 2) and DEVPKEY_DeviceInterface_ClassGuid (pid 3)
/// {026E516E-B814-414B-83CD-856D6FEF4822}.
const DEVPKEY_DEVICEINTERFACE_FMTID: [u8; 16] =
    [0x6E, 0x51, 0x6E, 0x02, 0x14, 0xB8, 0x4B, 0x41, 0x83, 0xCD, 0x85, 0x6D, 0x6F, 0xEF, 0x48, 0x22];
const DEVPKEY_DEVICEINTERFACE_ENABLED_PID: u32 = 2;
const DEVPKEY_DEVICEINTERFACE_CLASSGUID_PID: u32 = 3;

const SONY_VID: u16 = 0x054C;
const DUALSENSE_PID: u16 = 0x0CE6;
const DUALSENSE_EDGE_PID: u16 = 0x0DF2;

fn is_dualsense_usb(d: &Device) -> bool {
    !d.desc.bluetooth && d.desc.vendor_id == SONY_VID && matches!(d.desc.product_id, DUALSENSE_PID | DUALSENSE_EDGE_PID)
}

fn guid_to_bytes(g: &GUID) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[0..4].copy_from_slice(&g.data1.to_le_bytes());
    b[4..6].copy_from_slice(&g.data2.to_le_bytes());
    b[6..8].copy_from_slice(&g.data3.to_le_bytes());
    b[8..16].copy_from_slice(&g.data4);
    b
}

/// Deterministic Windows container GUID for device `di` in `devices`.
/// USB DualSense controllers share [`dualsense_usb_container_id`] with their
/// paired USB audio-haptics endpoint in `audio::mmdevapi`.
fn device_container_guid(devices: &[Arc<Device>], di: usize) -> GUID {
    let d = &devices[di];
    if is_dualsense_usb(d) {
        let slot = devices[..di].iter().filter(|other| is_dualsense_usb(other)).count() as u32;
        return dualsense_usb_container_id(slot);
    }
    let desc = &d.desc;
    let a = 0xa2c0_0000u32 ^ desc.location;
    let d3 = (0x8000 | ((desc.bluetooth as u16) << 12) | (desc.version & 0x0fff)).to_be_bytes();
    let hi = ((desc.id >> 32) as u16).to_be_bytes();
    let lo = (desc.id as u32).to_be_bytes();
    GUID::new(a, desc.vendor_id, desc.product_id, [d3[0], d3[1], hi[0], hi[1], lo[0], lo[1], lo[2], lo[3]])
}

struct DevSet {
    devices: Vec<Arc<Device>>,
}

fn sets() -> &'static Mutex<HashMap<usize, DevSet>> {
    static S: OnceLock<Mutex<HashMap<usize, DevSet>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

fn set_err(e: u32) {
    crate::teb::set_last_error(e);
}

unsafe fn read_guid(p: *const u8) -> Option<[u8; 16]> {
    if p.is_null() {
        return None;
    }
    let mut g = [0u8; 16];
    std::ptr::copy_nonoverlapping(p, g.as_mut_ptr(), 16);
    Some(g)
}

fn with_set<R>(set: *mut c_void, f: impl FnOnce(&DevSet) -> R) -> Option<R> {
    sets().lock().unwrap().get(&(set as usize)).map(f)
}

pub unsafe extern "win64" fn SetupDiGetClassDevsW(guid: *const u8, _enumr: *const u16, _hwnd: HWND, flags: u32) -> *mut c_void {
    let g = read_guid(guid);
    let hid_set = (flags & DIGCF_DEVICEINTERFACE != 0 && g == Some(GUID_DEVINTERFACE_HID))
        || (flags & (DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES) == 0 && g == Some(GUID_HIDCLASS))
        || flags & DIGCF_ALLCLASSES != 0;
    let devices = if hid_set {
        hid::ensure_started();
        hid::devices()
    } else {
        Vec::new()
    };
    log::debug!("SetupDiGetClassDevsW(flags={flags:#x}, hid={hid_set}) -> {} device(s)", devices.len());
    // A unique, never-dereferenced address serves as HDEVINFO.
    let key = Box::into_raw(Box::new(0u64)) as usize;
    sets().lock().unwrap().insert(key, DevSet { devices });
    key as *mut c_void
}

pub unsafe extern "win64" fn SetupDiDestroyDeviceInfoList(set: *mut c_void) -> BOOL {
    if sets().lock().unwrap().remove(&(set as usize)).is_some() {
        drop(Box::from_raw(set as *mut u64));
        TRUE
    } else {
        set_err(ERROR_INVALID_HANDLE);
        FALSE
    }
}

/// SP_DEVINFO_DATA { DWORD cbSize; GUID ClassGuid; DWORD DevInst; ULONG_PTR Reserved; } (32 bytes)
unsafe fn fill_devinfo(out: *mut u8, d: &Device, idx: usize) {
    std::ptr::copy_nonoverlapping(GUID_HIDCLASS.as_ptr(), out.add(4), 16);
    (out.add(20) as *mut u32).write_unaligned(d.inst);
    (out.add(24) as *mut usize).write_unaligned(idx + 1);
}

/// Index of the device a SP_DEVINFO_DATA refers to (Reserved = index + 1).
unsafe fn devinfo_index(data: *const u8) -> Option<usize> {
    if data.is_null() || (data as *const u32).read_unaligned() != 32 {
        return None;
    }
    let r = (data.add(24) as *const usize).read_unaligned();
    r.checked_sub(1)
}

pub unsafe extern "win64" fn SetupDiEnumDeviceInfo(set: *mut c_void, idx: u32, data: *mut u8) -> BOOL {
    if data.is_null() || (data as *const u32).read_unaligned() != 32 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return FALSE;
    }
    let r = with_set(set, |s| {
        let Some(d) = s.devices.get(idx as usize) else {
            return Err(ERROR_NO_MORE_ITEMS);
        };
        fill_devinfo(data, d, idx as usize);
        Ok(())
    });
    match r {
        Some(Ok(())) => TRUE,
        Some(Err(e)) => {
            set_err(e);
            FALSE
        }
        None => {
            set_err(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

/// SP_DEVICE_INTERFACE_DATA { DWORD cbSize; GUID InterfaceClassGuid; DWORD Flags; ULONG_PTR Reserved; } (32 bytes)
pub unsafe extern "win64" fn SetupDiEnumDeviceInterfaces(set: *mut c_void, dev: *const u8, guid: *const u8, idx: u32, data: *mut u8) -> BOOL {
    if data.is_null() || (data as *const u32).read_unaligned() != 32 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return FALSE;
    }
    if read_guid(guid) != Some(GUID_DEVINTERFACE_HID) {
        set_err(ERROR_NO_MORE_ITEMS);
        return FALSE;
    }
    let r = with_set(set, |s| {
        // Restricted to one device: it has exactly one interface (index 0).
        let di = if dev.is_null() {
            idx as usize
        } else {
            match (devinfo_index(dev), idx) {
                (Some(i), 0) => i,
                (Some(_), _) => return Err(ERROR_NO_MORE_ITEMS),
                (None, _) => return Err(ERROR_INVALID_PARAMETER),
            }
        };
        if di >= s.devices.len() {
            return Err(ERROR_NO_MORE_ITEMS);
        }
        std::ptr::copy_nonoverlapping(GUID_DEVINTERFACE_HID.as_ptr(), data.add(4), 16);
        (data.add(20) as *mut u32).write_unaligned(SPINT_ACTIVE);
        (data.add(24) as *mut usize).write_unaligned(di + 1);
        Ok(())
    });
    match r {
        Some(Ok(())) => TRUE,
        Some(Err(e)) => {
            set_err(e);
            FALSE
        }
        None => {
            set_err(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

/// SP_DEVICE_INTERFACE_DETAIL_DATA_W { DWORD cbSize (= 8 on x64); WCHAR DevicePath[]; }
pub unsafe extern "win64" fn SetupDiGetDeviceInterfaceDetailW(
    set: *mut c_void,
    iface: *const u8,
    detail: *mut u8,
    size: u32,
    required: *mut u32,
    devinfo: *mut u8,
) -> BOOL {
    let Some(di) = devinfo_index(iface) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(dev) = with_set(set, |s| s.devices.get(di).cloned()) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let Some(dev) = dev else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let path: Vec<u16> = dev.path.encode_utf16().chain(std::iter::once(0)).collect();
    let need = 4 + 2 * path.len() as u32;
    if !required.is_null() {
        std::ptr::write_unaligned(required, need);
    }
    if detail.is_null() {
        set_err(if size == 0 { ERROR_INSUFFICIENT_BUFFER } else { ERROR_INVALID_USER_BUFFER });
        return FALSE;
    }
    if (detail as *const u32).read_unaligned() != 8 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return FALSE;
    }
    if size < need {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(path.as_ptr() as *const u8, detail.add(4), path.len() * 2);
    if !devinfo.is_null() {
        if (devinfo as *const u32).read_unaligned() != 32 {
            set_err(ERROR_INVALID_USER_BUFFER);
            return FALSE;
        }
        fill_devinfo(devinfo, &dev, di);
    }
    TRUE
}

/// Registry properties of a HID device node.
fn property(devices: &[Arc<Device>], di: usize, prop: u32) -> Option<(u32, Vec<String>)> {
    let d = devices.get(di)?;
    let x = &d.desc;
    let sz = |s: String| Some((REG_SZ, vec![s]));
    match prop {
        0x00 => sz("HID-compliant game controller".into()), // SPDRP_DEVICEDESC
        0x01 => {
            // SPDRP_HARDWAREID
            let bus = if x.bluetooth {
                format!("HID\\{{00001124-0000-1000-8000-00805f9b34fb}}_VID&0002{:04X}_PID&{:04X}", x.vendor_id, x.product_id)
            } else {
                format!("HID\\VID_{:04X}&PID_{:04X}", x.vendor_id, x.product_id)
            };
            let up = format!("HID_DEVICE_UP:{:04X}_U:{:04X}", d.caps.usage_page, d.caps.usage);
            Some((REG_MULTI_SZ, vec![format!("{bus}&REV_{:04X}", x.version), bus, "HID_DEVICE_SYSTEM_GAME".into(), up, "HID_DEVICE".into()]))
        }
        0x07 => sz("HIDClass".into()),                                           // SPDRP_CLASS
        0x08 => sz("{745a17a0-74d3-11d0-b6fe-00a0c90f57da}".into()),             // SPDRP_CLASSGUID
        0x0B => sz("(Standard system devices)".into()),                          // SPDRP_MFG
        0x16 => sz("HID".into()),                                                // SPDRP_ENUMERATOR_NAME
        SPDRP_BASE_CONTAINERID => sz(format_guid(&device_container_guid(devices, di))),
        _ => None,
    }
}

pub unsafe extern "win64" fn SetupDiGetDeviceRegistryPropertyW(
    set: *mut c_void,
    devinfo: *const u8,
    prop: u32,
    reg_type: *mut u32,
    buf: *mut u8,
    size: u32,
    required: *mut u32,
) -> BOOL {
    let Some(di) = devinfo_index(devinfo) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(res) = with_set(set, |s| {
        if di >= s.devices.len() {
            Err(ERROR_INVALID_PARAMETER)
        } else {
            Ok(property(&s.devices, di, prop))
        }
    }) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let prop_val = match res {
        Ok(v) => v,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    let Some((ty, strings)) = prop_val else {
        log::debug!("SetupDiGetDeviceRegistryPropertyW(prop={prop:#x}): not present");
        set_err(ERROR_INVALID_DATA);
        return FALSE;
    };
    let mut w: Vec<u16> = Vec::new();
    for s in &strings {
        w.extend(s.encode_utf16());
        w.push(0);
    }
    if ty == REG_MULTI_SZ {
        w.push(0);
    }
    let need = (w.len() * 2) as u32;
    if !required.is_null() {
        std::ptr::write_unaligned(required, need);
    }
    if !reg_type.is_null() {
        std::ptr::write_unaligned(reg_type, ty);
    }
    if buf.is_null() || size < need {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, buf, need as usize);
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetClassDevsA(
    guid: *const u8,
    _enumr: *const u8,
    hwnd: HWND,
    flags: u32,
) -> *mut c_void {
    SetupDiGetClassDevsW(guid, std::ptr::null(), hwnd, flags)
}

pub unsafe extern "win64" fn SetupDiGetDeviceInterfaceDetailA(
    set: *mut c_void,
    iface: *const u8,
    detail: *mut u8,
    size: u32,
    required: *mut u32,
    devinfo: *mut u8,
) -> BOOL {
    let Some(di) = devinfo_index(iface) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(dev) = with_set(set, |s| s.devices.get(di).cloned()) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let Some(dev) = dev else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let bytes = dev.path.as_bytes();
    let need = 4 + bytes.len() as u32 + 1;
    if !required.is_null() {
        std::ptr::write_unaligned(required, need);
    }
    if detail.is_null() {
        set_err(if size == 0 { ERROR_INSUFFICIENT_BUFFER } else { ERROR_INVALID_USER_BUFFER });
        return FALSE;
    }
    if (detail as *const u32).read_unaligned() != 8 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return FALSE;
    }
    if size < need {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), detail.add(4), bytes.len());
    *detail.add(4 + bytes.len()) = 0;
    if !devinfo.is_null() {
        if (devinfo as *const u32).read_unaligned() != 32 {
            set_err(ERROR_INVALID_USER_BUFFER);
            return FALSE;
        }
        fill_devinfo(devinfo, &dev, di);
    }
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceRegistryPropertyA(
    set: *mut c_void,
    devinfo: *const u8,
    prop: u32,
    reg_type: *mut u32,
    buf: *mut u8,
    size: u32,
    required: *mut u32,
) -> BOOL {
    let Some(di) = devinfo_index(devinfo) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(res) = with_set(set, |s| {
        if di >= s.devices.len() {
            Err(ERROR_INVALID_PARAMETER)
        } else {
            Ok(property(&s.devices, di, prop))
        }
    }) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let prop_val = match res {
        Ok(v) => v,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    let Some((ty, strings)) = prop_val else {
        set_err(ERROR_INVALID_DATA);
        return FALSE;
    };
    let mut b: Vec<u8> = Vec::new();
    for s in &strings {
        b.extend_from_slice(s.as_bytes());
        b.push(0);
    }
    if ty == REG_MULTI_SZ {
        b.push(0);
    }
    let need = b.len() as u32;
    if !required.is_null() {
        std::ptr::write_unaligned(required, need);
    }
    if !reg_type.is_null() {
        std::ptr::write_unaligned(reg_type, ty);
    }
    if buf.is_null() || size < need {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(b.as_ptr(), buf, b.len());
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceInstanceIdW(
    set: *mut c_void,
    devinfo: *const u8,
    id: *mut u16,
    size: u32,
    required: *mut u32,
) -> BOOL {
    let Some(di) = devinfo_index(devinfo) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(Some(dev)) = with_set(set, |s| s.devices.get(di).cloned()) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let s = format!("HID\\VID_{:04X}&PID_{:04X}\\{}", dev.desc.vendor_id, dev.desc.product_id, di);
    let w: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    if !required.is_null() {
        std::ptr::write_unaligned(required, w.len() as u32);
    }
    if id.is_null() || (size as usize) < w.len() {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), id, w.len());
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceInstanceIdA(
    set: *mut c_void,
    devinfo: *const u8,
    id: *mut u8,
    size: u32,
    required: *mut u32,
) -> BOOL {
    let Some(di) = devinfo_index(devinfo) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(Some(dev)) = with_set(set, |s| s.devices.get(di).cloned()) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let s = format!("HID\\VID_{:04X}&PID_{:04X}\\{}", dev.desc.vendor_id, dev.desc.product_id, di);
    let need = s.len() as u32 + 1;
    if !required.is_null() {
        std::ptr::write_unaligned(required, need);
    }
    if id.is_null() || size < need {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), id, s.len());
    *id.add(s.len()) = 0;
    TRUE
}

pub unsafe extern "win64" fn SetupDiOpenDevRegKey(
    _set: *mut c_void,
    _devinfo: *const u8,
    _scope: u32,
    _hw_profile: u32,
    _key_type: u32,
    _sam: u32,
) -> HANDLE {
    set_err(2); // ERROR_FILE_NOT_FOUND
    INVALID_HANDLE_VALUE
}

pub unsafe extern "win64" fn SetupDiGetDevicePropertyW(
    set: *mut c_void,
    devinfo: *const u8,
    prop_key: *const u8,
    prop_type: *mut u32,
    buf: *mut u8,
    size: u32,
    required: *mut u32,
    flags: u32,
) -> BOOL {
    if flags != 0 {
        set_err(ERROR_INVALID_FLAGS);
        return FALSE;
    }
    if prop_key.is_null() {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(di) = devinfo_index(devinfo) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(fmtid) = read_guid(prop_key) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let pid = (prop_key.add(16) as *const u32).read_unaligned();
    let Some(res) = with_set(set, |s| {
        if di >= s.devices.len() {
            Err(ERROR_INVALID_PARAMETER)
        } else if fmtid == DEVPKEY_DEVICE_CONTAINERID_FMTID && pid == DEVPKEY_DEVICE_CONTAINERID_PID {
            Ok(Some(device_container_guid(&s.devices, di)))
        } else {
            Ok(None)
        }
    }) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match res {
        Ok(Some(guid)) => {
            let bytes = guid_to_bytes(&guid);
            if !prop_type.is_null() {
                std::ptr::write_unaligned(prop_type, DEVPROP_TYPE_GUID);
            }
            if !required.is_null() {
                std::ptr::write_unaligned(required, 16);
            }
            if buf.is_null() || size < 16 {
                set_err(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, 16);
            TRUE
        }
        Ok(None) => {
            if !required.is_null() {
                std::ptr::write_unaligned(required, 0);
            }
            set_err(ERROR_NOT_FOUND);
            FALSE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetupDiOpenDeviceInterfaceW(
    set: *mut c_void,
    device_path: LPCWSTR,
    open_flags: u32,
    data: *mut u8,
) -> BOOL {
    if device_path.is_null() {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if (open_flags & !DIODI_NO_ADD) != 0 {
        set_err(ERROR_INVALID_FLAGS);
        return FALSE;
    }
    if !data.is_null() && (data as *const u32).read_unaligned() != 32 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return FALSE;
    }
    let target = crate::strings::from_wide(device_path);
    let can_add = (open_flags & DIODI_NO_ADD) == 0;
    if can_add {
        hid::ensure_started();
    }
    let snapshot = if can_add { Some(hid::devices()) } else { None };
    let mut map = sets().lock().unwrap();
    let Some(s) = map.get_mut(&(set as usize)) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let found_idx = s
        .devices
        .iter()
        .position(|d| d.path.eq_ignore_ascii_case(&target))
        .or_else(|| {
            let dev = snapshot
                .as_ref()?
                .iter()
                .find(|d| d.path.eq_ignore_ascii_case(&target))?
                .clone();
            let idx = s.devices.len();
            s.devices.push(dev);
            Some(idx)
        });
    let Some(di) = found_idx else {
        set_err(ERROR_NO_SUCH_DEVICE_INTERFACE);
        return FALSE;
    };
    if !data.is_null() {
        std::ptr::copy_nonoverlapping(GUID_DEVINTERFACE_HID.as_ptr(), data.add(4), 16);
        (data.add(20) as *mut u32).write_unaligned(SPINT_ACTIVE);
        (data.add(24) as *mut usize).write_unaligned(di + 1);
    }
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceInterfacePropertyW(
    set: *mut c_void,
    iface: *const u8,
    prop_key: *const u8,
    prop_type: *mut u32,
    buf: *mut u8,
    size: u32,
    required: *mut u32,
    flags: u32,
) -> BOOL {
    if flags != 0 {
        set_err(ERROR_INVALID_FLAGS);
        return FALSE;
    }
    if prop_key.is_null() {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(di) = devinfo_index(iface) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let Some(fmtid) = read_guid(prop_key) else {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let pid = (prop_key.add(16) as *const u32).read_unaligned();
    let Some(res) = with_set(set, |s| {
        if di >= s.devices.len() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if fmtid == DEVPKEY_DEVICE_CONTAINERID_FMTID && pid == DEVPKEY_DEVICE_CONTAINERID_PID {
            Ok(Some((
                DEVPROP_TYPE_GUID,
                guid_to_bytes(&device_container_guid(&s.devices, di)).to_vec(),
            )))
        } else if fmtid == DEVPKEY_DEVICEINTERFACE_FMTID
            && pid == DEVPKEY_DEVICEINTERFACE_CLASSGUID_PID
        {
            Ok(Some((DEVPROP_TYPE_GUID, GUID_DEVINTERFACE_HID.to_vec())))
        } else if fmtid == DEVPKEY_DEVICEINTERFACE_FMTID
            && pid == DEVPKEY_DEVICEINTERFACE_ENABLED_PID
        {
            Ok(Some((DEVPROP_TYPE_BOOLEAN, vec![0xFFu8])))
        } else if fmtid == DEVPKEY_DEVICE_INSTANCEID_FMTID && pid == DEVPKEY_DEVICE_INSTANCEID_PID
        {
            let dev = &s.devices[di];
            let s_id = format!(
                "HID\\VID_{:04X}&PID_{:04X}\\{}",
                dev.desc.vendor_id, dev.desc.product_id, di
            );
            let mut bytes = Vec::with_capacity((s_id.len() + 1) * 2);
            for u in s_id.encode_utf16().chain(std::iter::once(0u16)) {
                bytes.extend_from_slice(&u.to_le_bytes());
            }
            Ok(Some((DEVPROP_TYPE_STRING, bytes)))
        } else {
            Ok(None)
        }
    }) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match res {
        Ok(Some((ty, data))) => {
            let need = data.len() as u32;
            if !prop_type.is_null() {
                std::ptr::write_unaligned(prop_type, ty);
            }
            if !required.is_null() {
                std::ptr::write_unaligned(required, need);
            }
            if buf.is_null() || size < need {
                set_err(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(data.as_ptr(), buf, data.len());
            TRUE
        }
        Ok(None) => {
            if !required.is_null() {
                std::ptr::write_unaligned(required, 0);
            }
            set_err(ERROR_NOT_FOUND);
            FALSE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetupGetInfDriverStoreLocationW(
    _file_name: LPCWSTR,
    _alt_platform: *const c_void,
    _locale_name: LPCWSTR,
    _return_buf: LPWSTR,
    _return_buf_size: u32,
    required: *mut u32,
) -> BOOL {
    if !required.is_null() {
        std::ptr::write_unaligned(required, 0);
    }
    set_err(2); // ERROR_FILE_NOT_FOUND
    FALSE
}

pub unsafe extern "win64" fn SetupDiSetDeviceInstallParamsA(
    _set: *mut c_void,
    _devinfo: *const u8,
    _params: *const u8,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetupDiSetDeviceInstallParamsW(
    _set: *mut c_void,
    _devinfo: *const u8,
    _params: *const u8,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceInstallParamsA(
    _set: *mut c_void,
    _devinfo: *const u8,
    params: *mut u8,
) -> BOOL {
    if params.is_null() {
        set_err(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb = (params as *const u32).read_unaligned() as usize;
    if cb > 4 {
        std::ptr::write_bytes(params.add(4), 0, cb - 4);
    }
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetDeviceInstallParamsW(
    set: *mut c_void,
    devinfo: *const u8,
    params: *mut u8,
) -> BOOL {
    SetupDiGetDeviceInstallParamsA(set, devinfo, params)
}

pub unsafe extern "win64" fn SetupDiBuildDriverInfoList(
    _set: *mut c_void,
    _devinfo: *mut u8,
    _driver_type: u32,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetupDiDestroyDriverInfoList(
    _set: *mut c_void,
    _devinfo: *mut u8,
    _driver_type: u32,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetupDiEnumDriverInfoA(
    _set: *mut c_void,
    _devinfo: *mut u8,
    _driver_type: u32,
    _index: u32,
    _data: *mut u8,
) -> BOOL {
    set_err(ERROR_NO_MORE_ITEMS);
    FALSE
}

pub unsafe extern "win64" fn SetupDiEnumDriverInfoW(
    set: *mut c_void,
    devinfo: *mut u8,
    driver_type: u32,
    index: u32,
    data: *mut u8,
) -> BOOL {
    SetupDiEnumDriverInfoA(set, devinfo, driver_type, index, data)
}

pub unsafe extern "win64" fn SetupDiGetDriverInfoDetailA(
    _set: *mut c_void,
    _devinfo: *mut u8,
    _drv_data: *mut u8,
    _detail: *mut u8,
    _detail_size: u32,
    required: *mut u32,
) -> BOOL {
    if !required.is_null() {
        std::ptr::write_unaligned(required, 0);
    }
    set_err(ERROR_NO_MORE_ITEMS);
    FALSE
}

pub unsafe extern "win64" fn SetupDiGetDriverInfoDetailW(
    set: *mut c_void,
    devinfo: *mut u8,
    drv_data: *mut u8,
    detail: *mut u8,
    detail_size: u32,
    required: *mut u32,
) -> BOOL {
    SetupDiGetDriverInfoDetailA(set, devinfo, drv_data, detail, detail_size, required)
}

pub unsafe extern "win64" fn CM_Get_Device_ID_List_SizeW(
    len: *mut u32,
    _filter: LPCWSTR,
    _flags: u32,
) -> u32 {
    if !len.is_null() {
        *len = 2;
    }
    0 // CR_SUCCESS
}

pub unsafe extern "win64" fn CM_Get_Device_ID_ListW(
    _filter: LPCWSTR,
    buffer: LPWSTR,
    buffer_len: u32,
    _flags: u32,
) -> u32 {
    if !buffer.is_null() && buffer_len >= 2 {
        *buffer = 0;
        *buffer.add(1) = 0;
    }
    0 // CR_SUCCESS
}

pub unsafe extern "win64" fn CM_Locate_DevNodeW(
    dev_inst: *mut u32,
    _device_id: LPCWSTR,
    _flags: u32,
) -> u32 {
    if !dev_inst.is_null() {
        *dev_inst = 1;
    }
    0 // CR_SUCCESS
}

pub unsafe extern "win64" fn CM_Get_DevNode_PropertyW(
    dev_inst: u32,
    prop_key: *const u8,
    prop_type: *mut u32,
    buf: *mut u8,
    len: *mut u32,
    flags: u32,
) -> u32 {
    if flags != 0 {
        return 0x04; // CR_INVALID_FLAG
    }
    if prop_key.is_null() || len.is_null() {
        return 0x03; // CR_INVALID_POINTER
    }
    let Some(fmtid) = read_guid(prop_key) else {
        return 0x03;
    };
    let pid = (prop_key.add(16) as *const u32).read_unaligned();
    if fmtid == DEVPKEY_DEVICE_CONTAINERID_FMTID && pid == DEVPKEY_DEVICE_CONTAINERID_PID {
        let devs = hid::devices();
        if let Some(di) = devs.iter().position(|d| d.inst == dev_inst) {
            let guid = device_container_guid(&devs, di);
            let bytes = guid_to_bytes(&guid);
            if !prop_type.is_null() {
                std::ptr::write_unaligned(prop_type, DEVPROP_TYPE_GUID);
            }
            let cap = std::ptr::read_unaligned(len);
            std::ptr::write_unaligned(len, 16);
            if buf.is_null() || cap < 16 {
                return 0x1A; // CR_BUFFER_SMALL
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, 16);
            return 0; // CR_SUCCESS
        }
    }
    std::ptr::write_unaligned(len, 0);
    0x25 // CR_NO_SUCH_VALUE
}

pub unsafe extern "win64" fn CM_MapCrToWin32Err(cr: u32, default_err: u32) -> u32 {
    if cr == 0 {
        0
    } else {
        default_err
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::backend::{self, Backend};
    use crate::audio::com::{CLSID_MMDeviceEnumerator, IID_IMMDeviceEnumerator};
    use crate::audio::propsys::{PKEY_Device_ContainerId, PROPERTYKEY, VT_CLSID};
    use crate::misc::ole32::{CoCreateInstance, PropVariantClear, StringFromGUID2};

    struct FakeDualSenseGuard(u64);
    impl Drop for FakeDualSenseGuard {
        fn drop(&mut self) {
            hid::device_removed(self.0);
        }
    }

    #[test]
    fn enumerates_hid_devices_like_windows() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        hid::tests::install_fake_dualsense(0xD5_0002);
        let _ds = FakeDualSenseGuard(0xD5_0002);
        unsafe {
            let set = SetupDiGetClassDevsW(GUID_DEVINTERFACE_HID.as_ptr(), std::ptr::null(), std::ptr::null_mut(), 0x12);
            assert!(!set.is_null());
            let mut found = None;
            let mut i = 0;
            loop {
                let mut ifd = [0u8; 32];
                ifd[0] = 32;
                if SetupDiEnumDeviceInterfaces(set, std::ptr::null(), GUID_DEVINTERFACE_HID.as_ptr(), i, ifd.as_mut_ptr()) == FALSE {
                    assert_eq!(crate::teb::last_error(), ERROR_NO_MORE_ITEMS);
                    break;
                }
                let mut need = 0u32;
                assert_eq!(SetupDiGetDeviceInterfaceDetailW(set, ifd.as_ptr(), std::ptr::null_mut(), 0, &mut need, std::ptr::null_mut()), FALSE);
                assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
                let mut detail = vec![0u8; need as usize];
                detail[0] = 8;
                assert_eq!(SetupDiGetDeviceInterfaceDetailW(set, ifd.as_ptr(), detail.as_mut_ptr(), need, &mut need, std::ptr::null_mut()), TRUE);
                let wpath: Vec<u16> = detail[4..].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&c| c != 0).collect();
                let path = String::from_utf16(&wpath).unwrap();
                if path.contains("vid_054c&pid_0ce6") {
                    // Device i, same index: container ID property.
                    let mut dd = [0u8; 32];
                    dd[0] = 32;
                    assert_eq!(SetupDiEnumDeviceInfo(set, i, dd.as_mut_ptr()), TRUE);
                    let mut buf = [0u8; 0x2000];
                    let (mut ty, mut got) = (0u32, 0u32);
                    assert_eq!(SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut ty, buf.as_mut_ptr(), 0x2000, &mut got), TRUE);
                    assert_eq!(ty, REG_SZ);
                    assert_eq!(got, 39 * 2, "\"{{8-4-4-4-12}}\" + NUL");
                    found = Some(path);
                }
                i += 1;
            }
            let path = found.expect("fake DualSense enumerated");
            assert!(path.starts_with("\\\\?\\hid#vid_054c&pid_0ce6#"));
            assert!(path.ends_with("#{4d1e55b2-f16f-11cf-88cb-001111000030}"));
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);
            assert_eq!(SetupDiDestroyDeviceInfoList(set), FALSE);
            // Other classes: valid but empty.
            let other = [0x11u8; 16];
            let set = SetupDiGetClassDevsW(other.as_ptr(), std::ptr::null(), std::ptr::null_mut(), 0x12);
            let mut ifd = [0u8; 32];
            ifd[0] = 32;
            assert_eq!(SetupDiEnumDeviceInterfaces(set, std::ptr::null(), other.as_ptr(), 0, ifd.as_mut_ptr()), FALSE);
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);
        }
    }

    #[test]
    fn dualsense_usb_container_id_pairs_setupapi_and_mmdevapi() {
        // Strict lock order: TEST_SERIAL first, audio_test_lock second.
        let _hid_guard = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));
        backend::force_output_devices(Some(vec![backend::null_device(), backend::dualsense_usb_device(0)]));

        hid::tests::install_fake_dualsense(0xD5_0042);
        let _ds = FakeDualSenseGuard(0xD5_0042);

        unsafe {
            // 1. SetupAPI side: SPDRP_BASE_CONTAINERID (W + A) and DEVPKEY_Device_ContainerId.
            let set = SetupDiGetClassDevsW(GUID_DEVINTERFACE_HID.as_ptr(), std::ptr::null(), std::ptr::null_mut(), 0x12);
            assert!(!set.is_null());

            let mut setup_cid_w = None;
            let mut setup_cid_a = None;
            let mut setup_devprop_guid = None;

            let mut i = 0u32;
            loop {
                let mut ifd = [0u8; 32];
                ifd[0] = 32;
                if SetupDiEnumDeviceInterfaces(set, std::ptr::null(), GUID_DEVINTERFACE_HID.as_ptr(), i, ifd.as_mut_ptr()) == FALSE {
                    break;
                }
                let mut need = 0u32;
                let _ = SetupDiGetDeviceInterfaceDetailW(set, ifd.as_ptr(), std::ptr::null_mut(), 0, &mut need, std::ptr::null_mut());
                let mut detail = vec![0u8; need as usize];
                detail[0] = 8;
                assert_eq!(SetupDiGetDeviceInterfaceDetailW(set, ifd.as_ptr(), detail.as_mut_ptr(), need, &mut need, std::ptr::null_mut()), TRUE);
                let wpath: Vec<u16> = detail[4..].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&c| c != 0).collect();
                let path = String::from_utf16(&wpath).unwrap();
                if path.contains("vid_054c&pid_0ce6") {
                    let mut dd = [0u8; 32];
                    dd[0] = 32;
                    assert_eq!(SetupDiEnumDeviceInfo(set, i, dd.as_mut_ptr()), TRUE);

                    // SPDRP_BASE_CONTAINERID (W): size probe -> ERROR_INSUFFICIENT_BUFFER, then read.
                    let (mut ty, mut req) = (0u32, 0u32);
                    assert_eq!(
                        SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut ty, std::ptr::null_mut(), 0, &mut req),
                        FALSE
                    );
                    assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
                    assert_eq!(ty, REG_SZ);
                    assert_eq!(req, 39 * 2);
                    let mut wbuf = vec![0u8; req as usize];
                    assert_eq!(
                        SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut ty, wbuf.as_mut_ptr(), req, &mut req),
                        TRUE
                    );
                    let wide_chars: Vec<u16> = wbuf.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&c| c != 0).collect();
                    setup_cid_w = Some(String::from_utf16(&wide_chars).unwrap());

                    // SPDRP_BASE_CONTAINERID (A): size probe -> ERROR_INSUFFICIENT_BUFFER, then read.
                    let (mut tya, mut reqa) = (0u32, 0u32);
                    assert_eq!(
                        SetupDiGetDeviceRegistryPropertyA(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut tya, std::ptr::null_mut(), 0, &mut reqa),
                        FALSE
                    );
                    assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
                    assert_eq!(tya, REG_SZ);
                    assert_eq!(reqa, 39);
                    let mut abuf = vec![0u8; reqa as usize];
                    assert_eq!(
                        SetupDiGetDeviceRegistryPropertyA(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut tya, abuf.as_mut_ptr(), reqa, &mut reqa),
                        TRUE
                    );
                    let nul = abuf.iter().position(|&b| b == 0).unwrap();
                    setup_cid_a = Some(String::from_utf8(abuf[..nul].to_vec()).unwrap());

                    // SetupDiGetDevicePropertyW(DEVPKEY_Device_ContainerId): DEVPROP_TYPE_GUID (13).
                    let mut key_bytes = [0u8; 20];
                    key_bytes[0..16].copy_from_slice(&DEVPKEY_DEVICE_CONTAINERID_FMTID);
                    key_bytes[16..20].copy_from_slice(&DEVPKEY_DEVICE_CONTAINERID_PID.to_le_bytes());
                    let (mut pty, mut preq) = (0u32, 0u32);
                    assert_eq!(
                        SetupDiGetDevicePropertyW(set, dd.as_ptr(), key_bytes.as_ptr(), &mut pty, std::ptr::null_mut(), 0, &mut preq, 0),
                        FALSE
                    );
                    assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
                    assert_eq!(pty, DEVPROP_TYPE_GUID);
                    assert_eq!(preq, 16);
                    let mut gbuf = [0u8; 16];
                    assert_eq!(
                        SetupDiGetDevicePropertyW(set, dd.as_ptr(), key_bytes.as_ptr(), &mut pty, gbuf.as_mut_ptr(), 16, &mut preq, 0),
                        TRUE
                    );
                    setup_devprop_guid = Some(gbuf);

                    // Unknown DEVPKEY -> ERROR_NOT_FOUND.
                    let mut bad_key = [0u8; 20];
                    bad_key[16..20].copy_from_slice(&99u32.to_le_bytes());
                    assert_eq!(
                        SetupDiGetDevicePropertyW(set, dd.as_ptr(), bad_key.as_ptr(), &mut pty, gbuf.as_mut_ptr(), 16, &mut preq, 0),
                        FALSE
                    );
                    assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);
                    break;
                }
                i += 1;
            }
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);

            let setup_cid_w = setup_cid_w.expect("DualSense SPDRP_BASE_CONTAINERID W");
            let setup_cid_a = setup_cid_a.expect("DualSense SPDRP_BASE_CONTAINERID A");
            let setup_devprop_guid = setup_devprop_guid.expect("DualSense DEVPKEY_Device_ContainerId");
            assert_eq!(setup_cid_w, setup_cid_a);
            assert_eq!(setup_cid_w, format_guid(&dualsense_usb_container_id(0)));
            let mut devprop_str_w = [0u16; 64];
            assert_eq!(StringFromGUID2(setup_devprop_guid.as_ptr(), devprop_str_w.as_mut_ptr(), 64), 39);
            let devprop_str = String::from_utf16_lossy(&devprop_str_w[..38]);
            assert_eq!(devprop_str, setup_cid_w);

            // 2. MMDevAPI side: EnumAudioEndpoints(eRender) -> IPropertyStore::GetValue(PKEY_Device_ContainerId).
            type P = *mut c_void;
            let slot = |obj: P, idx: usize| -> usize { *(*(obj as *const *const usize)).add(idx) };
            let mut enum_obj: P = std::ptr::null_mut();
            assert_eq!(
                CoCreateInstance(
                    &CLSID_MMDeviceEnumerator as *const GUID as *const u8,
                    std::ptr::null_mut(),
                    0x17,
                    &IID_IMMDeviceEnumerator as *const GUID as *const u8,
                    &mut enum_obj,
                ),
                S_OK
            );
            let enum_ep: unsafe extern "win64" fn(P, u32, u32, *mut P) -> HRESULT = std::mem::transmute(slot(enum_obj, 3));
            let mut coll: P = std::ptr::null_mut();
            assert_eq!(enum_ep(enum_obj, 0 /* eRender */, 0xF, &mut coll), S_OK);
            let get_count: unsafe extern "win64" fn(P, *mut u32) -> HRESULT = std::mem::transmute(slot(coll, 3));
            let get_item: unsafe extern "win64" fn(P, u32, *mut P) -> HRESULT = std::mem::transmute(slot(coll, 4));
            let mut count = 0u32;
            assert_eq!(get_count(coll, &mut count), S_OK);
            assert_eq!(count, 2);

            let mut matched_audio_cid = None;
            for idx in 0..count {
                let mut dev: P = std::ptr::null_mut();
                assert_eq!(get_item(coll, idx, &mut dev), S_OK);
                let open_ps: unsafe extern "win64" fn(P, u32, *mut P) -> HRESULT = std::mem::transmute(slot(dev, 4));
                let mut ps: P = std::ptr::null_mut();
                assert_eq!(open_ps(dev, 0, &mut ps), S_OK);
                let get_val: unsafe extern "win64" fn(P, *const PROPERTYKEY, *mut u8) -> HRESULT = std::mem::transmute(slot(ps, 5));
                let mut pv = [0u8; 24];
                assert_eq!(get_val(ps, &PKEY_Device_ContainerId, pv.as_mut_ptr()), S_OK);
                assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_CLSID);
                let puuid = usize::from_le_bytes(pv[8..16].try_into().unwrap()) as *const u8;
                assert!(!puuid.is_null());
                let mut audio_guid_bytes = [0u8; 16];
                std::ptr::copy_nonoverlapping(puuid, audio_guid_bytes.as_mut_ptr(), 16);
                let mut audio_str_w = [0u16; 64];
                assert_eq!(StringFromGUID2(puuid, audio_str_w.as_mut_ptr(), 64), 39);
                let audio_cid = String::from_utf16_lossy(&audio_str_w[..38]);
                assert_eq!(PropVariantClear(pv.as_mut_ptr()), S_OK);
                if audio_cid == setup_cid_w {
                    assert_eq!(audio_guid_bytes, setup_devprop_guid);
                    matched_audio_cid = Some(audio_cid);
                }
                let rel_ps: unsafe extern "win64" fn(P) -> u32 = std::mem::transmute(slot(ps, 2));
                rel_ps(ps);
                let rel_dev: unsafe extern "win64" fn(P) -> u32 = std::mem::transmute(slot(dev, 2));
                rel_dev(dev);
            }
            assert_eq!(matched_audio_cid.as_deref(), Some(setup_cid_w.as_str()));

            let rel_coll: unsafe extern "win64" fn(P) -> u32 = std::mem::transmute(slot(coll, 2));
            rel_coll(coll);
            let rel_enum: unsafe extern "win64" fn(P) -> u32 = std::mem::transmute(slot(enum_obj, 2));
            rel_enum(enum_obj);
        }
    }

    #[test]
    fn open_device_interface_and_get_interface_property_w() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        hid::tests::install_fake_dualsense(0xD5_0099);
        let _ds = FakeDualSenseGuard(0xD5_0099);
        unsafe {
            let ds_path = hid::devices()
                .into_iter()
                .find(|d| d.desc.id == 0xD5_0099)
                .expect("fake DualSense installed")
                .path
                .clone();
            let ds_path_upper = crate::strings::to_wide_z(&ds_path.to_ascii_uppercase());

            // Start with an empty device set (unrelated class), then open interface into it.
            let other_guid = [0x22u8; 16];
            let set = SetupDiGetClassDevsW(other_guid.as_ptr(), std::ptr::null(), std::ptr::null_mut(), DIGCF_DEVICEINTERFACE);
            assert!(!set.is_null());

            // DIODI_NO_ADD fails when interface is not yet in the set.
            let mut ifd = [0u8; 32];
            ifd[0] = 32;
            assert_eq!(SetupDiOpenDeviceInterfaceW(set, ds_path_upper.as_ptr(), DIODI_NO_ADD, ifd.as_mut_ptr()), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_NO_SUCH_DEVICE_INTERFACE);

            // Default open adds and populates SP_DEVICE_INTERFACE_DATA.
            assert_eq!(SetupDiOpenDeviceInterfaceW(set, ds_path_upper.as_ptr(), 0, ifd.as_mut_ptr()), TRUE);
            assert_eq!(&ifd[4..20], &GUID_DEVINTERFACE_HID);

            // DEVPKEY_DeviceInterface_Enabled -> DEVPROP_TYPE_BOOLEAN (0xFF).
            let mut key_en = [0u8; 20];
            key_en[0..16].copy_from_slice(&DEVPKEY_DEVICEINTERFACE_FMTID);
            key_en[16..20].copy_from_slice(&DEVPKEY_DEVICEINTERFACE_ENABLED_PID.to_le_bytes());
            let (mut ty, mut req) = (0u32, 0u32);
            let mut bval = 0u8;
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set, ifd.as_ptr(), key_en.as_ptr(), &mut ty, &mut bval, 1, &mut req, 0),
                TRUE
            );
            assert_eq!(ty, DEVPROP_TYPE_BOOLEAN);
            assert_eq!(req, 1);
            assert_eq!(bval, 0xFF);

            // DEVPKEY_Device_InstanceId -> DEVPROP_TYPE_STRING.
            let mut key_id = [0u8; 20];
            key_id[0..16].copy_from_slice(&DEVPKEY_DEVICE_INSTANCEID_FMTID);
            key_id[16..20].copy_from_slice(&DEVPKEY_DEVICE_INSTANCEID_PID.to_le_bytes());
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set, ifd.as_ptr(), key_id.as_ptr(), &mut ty, std::ptr::null_mut(), 0, &mut req, 0),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(ty, DEVPROP_TYPE_STRING);
            let mut sbuf = vec![0u8; req as usize];
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set, ifd.as_ptr(), key_id.as_ptr(), &mut ty, sbuf.as_mut_ptr(), req, &mut req, 0),
                TRUE
            );
            let inst_str = crate::strings::from_wide(sbuf.as_ptr() as *const u16);
            assert!(inst_str.starts_with("HID\\VID_054C&PID_0CE6\\"));

            // Unknown property -> ERROR_NOT_FOUND.
            let bad_key = [0x99u8; 20];
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set, ifd.as_ptr(), bad_key.as_ptr(), &mut ty, sbuf.as_mut_ptr(), req, &mut req, 0),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);

            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);
        }
    }
}
