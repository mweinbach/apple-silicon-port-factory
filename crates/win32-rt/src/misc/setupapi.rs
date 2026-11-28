//! setupapi.dll: device enumeration for HID game controllers and the primary
//! PCI display adapter.
//!
//! `SetupDiGetClassDevsW`/`A` snapshots:
//! - HID devices from `misc::hid` for `GUID_DEVINTERFACE_HID` (with
//!   `DIGCF_DEVICEINTERFACE`), the `HIDClass` setup class, or `DIGCF_ALLCLASSES`
//!   when the optional `Enumerator` filter is `NULL` or `"HID"`.
//! - The host PCI display adapter (matching `dxgi::adapter` and the seeded
//!   `HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-...}\0000` driver
//!   registry key in `advapi32`) for `GUID_DEVCLASS_DISPLAY`,
//!   `GUID_DEVINTERFACE_DISPLAY_ADAPTER`, or `DIGCF_ALLCLASSES` when the
//!   optional `Enumerator` filter is `NULL` or `"PCI"`.
//!
//! Device `i` of a single-class set exposes exactly one interface at index `i`,
//! as on Windows for single-collection HID devices; callers such as pad
//! libraries rely on interface and device indices lining up.

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
const DIREG_DRV: u32 = 2;
const SPINT_ACTIVE: u32 = 1;
const REG_SZ: u32 = 1;
const REG_MULTI_SZ: u32 = 7;
const SPDRP_BASE_CONTAINERID: u32 = 0x24;
const DEVPROP_TYPE_GUID: u32 = 13;
const DEVPROP_TYPE_BOOLEAN: u32 = 17;
const DEVPROP_TYPE_STRING: u32 = 18;

/// HIDClass setup class {745A17A0-74D3-11D0-B6FE-00A0C90F57DA}.
const GUID_HIDCLASS: [u8; 16] = [0xA0, 0x17, 0x5A, 0x74, 0xD3, 0x74, 0xD0, 0x11, 0xB6, 0xFE, 0x00, 0xA0, 0xC9, 0x0F, 0x57, 0xDA];

/// Display adapter setup class {4D36E968-E325-11CE-BFC1-08002BE10318}.
const GUID_DEVCLASS_DISPLAY: [u8; 16] =
    [0x68, 0xE9, 0x36, 0x4D, 0x25, 0xE3, 0xCE, 0x11, 0xBF, 0xC1, 0x08, 0x00, 0x2B, 0xE1, 0x03, 0x18];

/// Display adapter device interface {5B45201D-F2F2-4F3B-85BB-30FF1F953599}.
const GUID_DEVINTERFACE_DISPLAY_ADAPTER: [u8; 16] =
    [0x1D, 0x20, 0x45, 0x5B, 0xF2, 0xF2, 0x3B, 0x4F, 0x85, 0xBB, 0x30, 0xFF, 0x1F, 0x95, 0x35, 0x99];

/// Standard Windows internal chassis container ID `{00000000-0000-0000-ffff-ffffffffffff}`.
const INTERNAL_CONTAINER_GUID: GUID = GUID::new(0, 0, 0, [0xFF; 8]);
const DISPLAY_CLASS_GUID_STR: &str = "{4d36e968-e325-11ce-bfc1-08002be10318}";
const DISPLAY_DRIVER_KEY_STR: &str = "{4d36e968-e325-11ce-bfc1-08002be10318}\\0000";
const DISPLAY_DRIVER_REG_SUBKEY: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000";
const DISPLAY_DEVINST: u32 = 0x4450_0001;

/// Host PCI display adapter identity shared across `setupapi`, `user32::monitor`,
/// `dxgi::adapter`, and `advapi32` display class registry seeding.
#[derive(Clone, Debug)]
pub(crate) struct DisplayAdapterInfo {
    pub name: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub pci_id: String,
    pub instance_id: String,
    pub interface_path: String,
}

pub(crate) fn primary_display_adapter() -> DisplayAdapterInfo {
    let (name, device_id) = match crate::d3d12::mtl::device() {
        Some(d) => (d.name.clone(), d.pci_device_id()),
        None => ("Apple Silicon GPU".to_string(), 0xc140),
    };
    let vendor_id = crate::dxgi::adapter::VENDOR_APPLE;
    let subsys_id = crate::dxgi::adapter::SUBSYS_APPLE;
    let revision = crate::dxgi::adapter::REVISION_APPLE;
    let pci_id = format!(
        "PCI\\VEN_{vendor_id:04X}&DEV_{device_id:04X}&SUBSYS_{subsys_id:08X}&REV_{revision:02X}"
    );
    let instance_id = format!("{pci_id}\\3&11583659&0&00");
    let interface_path = format!(
        "\\\\?\\pci#ven_{vendor_id:04x}&dev_{device_id:04x}&subsys_{subsys_id:08x}&rev_{revision:02x}#3&11583659&0&00#{{5b45201d-f2f2-4f3b-85bb-30ff1f953599}}"
    );
    DisplayAdapterInfo {
        name,
        vendor_id,
        device_id,
        subsys_id,
        pci_id,
        instance_id,
        interface_path,
    }
}

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

#[derive(Clone)]
enum DevEntry {
    Hid {
        dev: Arc<Device>,
        hid_index: usize,
        dualsense_slot: u32,
    },
    Display(DisplayAdapterInfo),
}

impl DevEntry {
    fn class_guid(&self) -> &'static [u8; 16] {
        match self {
            DevEntry::Hid { .. } => &GUID_HIDCLASS,
            DevEntry::Display(_) => &GUID_DEVCLASS_DISPLAY,
        }
    }

    fn interface_guid(&self) -> &'static [u8; 16] {
        match self {
            DevEntry::Hid { .. } => &GUID_DEVINTERFACE_HID,
            DevEntry::Display(_) => &GUID_DEVINTERFACE_DISPLAY_ADAPTER,
        }
    }

    fn dev_inst(&self) -> u32 {
        match self {
            DevEntry::Hid { dev, .. } => dev.inst,
            DevEntry::Display(_) => DISPLAY_DEVINST,
        }
    }

    fn interface_path(&self) -> &str {
        match self {
            DevEntry::Hid { dev, .. } => &dev.path,
            DevEntry::Display(d) => &d.interface_path,
        }
    }

    fn instance_id(&self) -> String {
        match self {
            DevEntry::Hid { dev, hid_index, .. } => {
                format!("HID\\VID_{:04X}&PID_{:04X}\\{hid_index}", dev.desc.vendor_id, dev.desc.product_id)
            }
            DevEntry::Display(d) => d.instance_id.clone(),
        }
    }
}

/// Snapshot HID devices with their unfiltered index and USB DualSense slot so
/// instance-ID filtering and single-interface opens preserve stable identity.
fn hid_entries_with_identity(devs: Vec<Arc<Device>>) -> Vec<DevEntry> {
    let mut ds_slot = 0u32;
    devs.into_iter()
        .enumerate()
        .map(|(hid_index, dev)| {
            let dualsense_slot = if is_dualsense_usb(&dev) {
                let slot = ds_slot;
                ds_slot += 1;
                slot
            } else {
                0
            };
            DevEntry::Hid { dev, hid_index, dualsense_slot }
        })
        .collect()
}

/// Deterministic Windows container GUID for a HID device `d` at USB DualSense slot `slot`.
fn hid_container_guid(d: &Device, dualsense_slot: u32) -> GUID {
    if is_dualsense_usb(d) {
        return dualsense_usb_container_id(dualsense_slot);
    }
    let desc = &d.desc;
    let a = 0xa2c0_0000u32 ^ desc.location;
    let d3 = (0x8000 | ((desc.bluetooth as u16) << 12) | (desc.version & 0x0fff)).to_be_bytes();
    let hi = ((desc.id >> 32) as u16).to_be_bytes();
    let lo = (desc.id as u32).to_be_bytes();
    GUID::new(a, desc.vendor_id, desc.product_id, [d3[0], d3[1], hi[0], hi[1], lo[0], lo[1], lo[2], lo[3]])
}

/// Deterministic Windows container GUID for `entry`.
/// USB DualSense controllers share [`dualsense_usb_container_id`] with their
/// paired USB audio-haptics endpoint in `audio::mmdevapi`.
fn device_container_guid(entry: &DevEntry) -> GUID {
    match entry {
        DevEntry::Hid { dev, dualsense_slot, .. } => hid_container_guid(dev, *dualsense_slot),
        DevEntry::Display(_) => INTERNAL_CONTAINER_GUID,
    }
}

struct DevSet {
    devices: Vec<DevEntry>,
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

unsafe fn get_class_devs(guid: *const u8, enumerator: Option<String>, flags: u32) -> *mut c_void {
    let g = read_guid(guid);
    let enum_upper = enumerator
        .map(|s| s.trim().to_ascii_uppercase())
        .filter(|s| !s.is_empty());

    let hid_class_ok = (flags & DIGCF_DEVICEINTERFACE != 0 && g == Some(GUID_DEVINTERFACE_HID))
        || (flags & (DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES) == 0 && g == Some(GUID_HIDCLASS))
        || (flags & DIGCF_ALLCLASSES != 0);
    let hid_enum_ok = match enum_upper.as_deref() {
        None | Some("HID") => true,
        Some(e) => e.starts_with("HID\\"),
    };

    let disp_class_ok = (flags & DIGCF_DEVICEINTERFACE != 0 && g == Some(GUID_DEVINTERFACE_DISPLAY_ADAPTER))
        || (flags & (DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES) == 0 && g == Some(GUID_DEVCLASS_DISPLAY))
        || (flags & DIGCF_ALLCLASSES != 0);
    let disp_enum_ok = match enum_upper.as_deref() {
        None | Some("PCI") => true,
        Some(e) => e.starts_with("PCI\\"),
    };

    let mut devices = Vec::new();
    if hid_class_ok && hid_enum_ok {
        hid::ensure_started();
        for entry in hid_entries_with_identity(hid::devices()) {
            if let Some(ref e) = enum_upper {
                if e.starts_with("HID\\") && !entry.instance_id().to_ascii_uppercase().starts_with(e) {
                    continue;
                }
            }
            devices.push(entry);
        }
    }
    if disp_class_ok && disp_enum_ok {
        let disp = primary_display_adapter();
        let matches_inst = match enum_upper.as_deref() {
            None | Some("PCI") => true,
            Some(e) => disp.instance_id.to_ascii_uppercase().starts_with(e),
        };
        if matches_inst {
            devices.push(DevEntry::Display(disp));
        }
    }

    log::debug!(
        "SetupDiGetClassDevs(flags={flags:#x}, enum={enum_upper:?}, hid={}, display={}) -> {} device(s)",
        hid_class_ok && hid_enum_ok,
        disp_class_ok && disp_enum_ok,
        devices.len()
    );
    // A unique, never-dereferenced address serves as HDEVINFO.
    let key = Box::into_raw(Box::new(0u64)) as usize;
    sets().lock().unwrap().insert(key, DevSet { devices });
    key as *mut c_void
}

pub unsafe extern "win64" fn SetupDiGetClassDevsW(guid: *const u8, enumr: *const u16, _hwnd: HWND, flags: u32) -> *mut c_void {
    let enumerator = crate::strings::from_wide_opt(enumr);
    get_class_devs(guid, enumerator, flags)
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
unsafe fn fill_devinfo(out: *mut u8, d: &DevEntry, idx: usize) {
    std::ptr::copy_nonoverlapping(d.class_guid().as_ptr(), out.add(4), 16);
    (out.add(20) as *mut u32).write_unaligned(d.dev_inst());
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
        Some(Ok(())) => {
            set_err(0);
            TRUE
        }
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
    let Some(target_guid) = read_guid(guid) else {
        set_err(ERROR_NO_MORE_ITEMS);
        return FALSE;
    };
    if target_guid != GUID_DEVINTERFACE_HID && target_guid != GUID_DEVINTERFACE_DISPLAY_ADAPTER {
        set_err(ERROR_NO_MORE_ITEMS);
        return FALSE;
    }
    let r = with_set(set, |s| {
        let di = if dev.is_null() {
            s.devices
                .iter()
                .enumerate()
                .filter(|(_, e)| *e.interface_guid() == target_guid)
                .nth(idx as usize)
                .map(|(i, _)| i)
                .ok_or(ERROR_NO_MORE_ITEMS)?
        } else {
            match (devinfo_index(dev), idx) {
                (Some(i), 0) => match s.devices.get(i) {
                    Some(e) if *e.interface_guid() == target_guid => i,
                    _ => return Err(ERROR_NO_MORE_ITEMS),
                },
                (Some(_), _) => return Err(ERROR_NO_MORE_ITEMS),
                (None, _) => return Err(ERROR_INVALID_PARAMETER),
            }
        };
        std::ptr::copy_nonoverlapping(target_guid.as_ptr(), data.add(4), 16);
        (data.add(20) as *mut u32).write_unaligned(SPINT_ACTIVE);
        (data.add(24) as *mut usize).write_unaligned(di + 1);
        Ok(())
    });
    match r {
        Some(Ok(())) => {
            set_err(0);
            TRUE
        }
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
    let path: Vec<u16> = dev.interface_path().encode_utf16().chain(std::iter::once(0)).collect();
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
    set_err(0);
    TRUE
}

/// Registry properties of a HID or PCI Display device node.
fn property(entry: &DevEntry, prop: u32) -> Option<(u32, Vec<String>)> {
    let sz = |s: String| Some((REG_SZ, vec![s]));
    match entry {
        DevEntry::Hid { dev: d, .. } => {
            let x = &d.desc;
            match prop {
                0x00 => {
                    // SPDRP_DEVICEDESC
                    let desc = x
                        .product
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .unwrap_or("HID-compliant game controller")
                        .to_string();
                    sz(desc)
                }
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
                SPDRP_BASE_CONTAINERID => sz(format_guid(&device_container_guid(entry))),
                _ => None,
            }
        }
        DevEntry::Display(d) => match prop {
            0x00 => sz(d.name.clone()),                                                  // SPDRP_DEVICEDESC
            0x01 => {
                // SPDRP_HARDWAREID
                let ven_dev = format!("PCI\\VEN_{:04X}&DEV_{:04X}", d.vendor_id, d.device_id);
                let ven_dev_subsys = format!("{ven_dev}&SUBSYS_{:08X}", d.subsys_id);
                Some((
                    REG_MULTI_SZ,
                    vec![
                        d.pci_id.clone(),
                        ven_dev_subsys,
                        format!("{ven_dev}&CC_030000"),
                        format!("{ven_dev}&CC_0300"),
                        ven_dev,
                    ],
                ))
            }
            0x07 => sz("Display".into()),                                                // SPDRP_CLASS
            0x08 => sz(DISPLAY_CLASS_GUID_STR.into()),                                   // SPDRP_CLASSGUID
            0x09 => sz(DISPLAY_DRIVER_KEY_STR.into()),                                   // SPDRP_DRIVER
            0x0B => sz("Apple Inc.".into()),                                             // SPDRP_MFG
            0x0C => sz(d.name.clone()),                                                  // SPDRP_FRIENDLYNAME
            0x16 => sz("PCI".into()),                                                    // SPDRP_ENUMERATOR_NAME
            SPDRP_BASE_CONTAINERID => sz(format_guid(&INTERNAL_CONTAINER_GUID)),
            _ => None,
        },
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
    let Some(res) = with_set(set, |s| match s.devices.get(di) {
        Some(entry) => Ok(property(entry, prop)),
        None => Err(ERROR_INVALID_PARAMETER),
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
    set_err(0);
    TRUE
}

pub unsafe extern "win64" fn SetupDiGetClassDevsA(
    guid: *const u8,
    enumr: *const u8,
    _hwnd: HWND,
    flags: u32,
) -> *mut c_void {
    let enumerator = crate::strings::from_ansi_opt(enumr);
    get_class_devs(guid, enumerator, flags)
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
    let bytes = dev.interface_path().as_bytes();
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
    set_err(0);
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
    let Some(res) = with_set(set, |s| match s.devices.get(di) {
        Some(entry) => Ok(property(entry, prop)),
        None => Err(ERROR_INVALID_PARAMETER),
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
    set_err(0);
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
    let s = dev.instance_id();
    let w: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    if !required.is_null() {
        std::ptr::write_unaligned(required, w.len() as u32);
    }
    if id.is_null() || (size as usize) < w.len() {
        set_err(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), id, w.len());
    set_err(0);
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
    let s = dev.instance_id();
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
    set_err(0);
    TRUE
}

pub unsafe extern "win64" fn SetupDiOpenDevRegKey(
    set: *mut c_void,
    devinfo: *const u8,
    _scope: u32,
    _hw_profile: u32,
    key_type: u32,
    sam: u32,
) -> HANDLE {
    if key_type == DIREG_DRV {
        if let Some(di) = devinfo_index(devinfo) {
            let is_display = with_set(set, |s| matches!(s.devices.get(di), Some(DevEntry::Display(_))));
            if is_display == Some(true) {
                let sub = crate::strings::to_wide_z(DISPLAY_DRIVER_REG_SUBKEY);
                let mut hkey: HANDLE = std::ptr::null_mut();
                const HKEY_LOCAL_MACHINE: HANDLE = 0x8000_0002usize as HANDLE;
                let rc = crate::advapi32::RegOpenKeyExW(HKEY_LOCAL_MACHINE, sub.as_ptr(), 0, sam, &mut hkey);
                if rc == 0 && !hkey.is_null() {
                    set_err(0);
                    return hkey;
                }
            }
        }
    }
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
        let Some(entry) = s.devices.get(di) else {
            return Err(ERROR_INVALID_PARAMETER);
        };
        if fmtid == DEVPKEY_DEVICE_CONTAINERID_FMTID && pid == DEVPKEY_DEVICE_CONTAINERID_PID {
            Ok(Some(device_container_guid(entry)))
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
            set_err(0);
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
    let snapshot = if can_add { Some(hid_entries_with_identity(hid::devices())) } else { None };
    let mut map = sets().lock().unwrap();
    let Some(s) = map.get_mut(&(set as usize)) else {
        set_err(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let found_idx = s
        .devices
        .iter()
        .position(|d| d.interface_path().eq_ignore_ascii_case(&target))
        .or_else(|| {
            if let Some(entry) = snapshot
                .into_iter()
                .flatten()
                .find(|e| e.interface_path().eq_ignore_ascii_case(&target))
            {
                let idx = s.devices.len();
                s.devices.push(entry);
                return Some(idx);
            }
            if can_add {
                let disp = primary_display_adapter();
                if disp.interface_path.eq_ignore_ascii_case(&target) {
                    let idx = s.devices.len();
                    s.devices.push(DevEntry::Display(disp));
                    return Some(idx);
                }
            }
            None
        });
    let Some(di) = found_idx else {
        set_err(ERROR_NO_SUCH_DEVICE_INTERFACE);
        return FALSE;
    };
    if !data.is_null() {
        std::ptr::copy_nonoverlapping(s.devices[di].interface_guid().as_ptr(), data.add(4), 16);
        (data.add(20) as *mut u32).write_unaligned(SPINT_ACTIVE);
        (data.add(24) as *mut usize).write_unaligned(di + 1);
    }
    set_err(0);
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
        let Some(entry) = s.devices.get(di) else {
            return Err(ERROR_INVALID_PARAMETER);
        };
        if fmtid == DEVPKEY_DEVICE_CONTAINERID_FMTID && pid == DEVPKEY_DEVICE_CONTAINERID_PID {
            Ok(Some((
                DEVPROP_TYPE_GUID,
                guid_to_bytes(&device_container_guid(entry)).to_vec(),
            )))
        } else if fmtid == DEVPKEY_DEVICEINTERFACE_FMTID
            && pid == DEVPKEY_DEVICEINTERFACE_CLASSGUID_PID
        {
            Ok(Some((DEVPROP_TYPE_GUID, entry.interface_guid().to_vec())))
        } else if fmtid == DEVPKEY_DEVICEINTERFACE_FMTID
            && pid == DEVPKEY_DEVICEINTERFACE_ENABLED_PID
        {
            Ok(Some((DEVPROP_TYPE_BOOLEAN, vec![0xFFu8])))
        } else if fmtid == DEVPKEY_DEVICE_INSTANCEID_FMTID && pid == DEVPKEY_DEVICE_INSTANCEID_PID
        {
            let s_id = entry.instance_id();
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
        let maybe_guid = if dev_inst == DISPLAY_DEVINST {
            Some(INTERNAL_CONTAINER_GUID)
        } else {
            let devs = hid::devices();
            devs.iter().position(|d| d.inst == dev_inst).map(|di| {
                let slot = devs[..di].iter().filter(|other| is_dualsense_usb(other)).count() as u32;
                hid_container_guid(&devs[di], slot)
            })
        };
        if let Some(guid) = maybe_guid {
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
    fn setupdi_clears_last_error_on_success_and_reports_product_desc() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        hid::tests::install_fake_dualsense(0xD5_0099);
        let _ds = FakeDualSenseGuard(0xD5_0099);
        unsafe {
            let set = SetupDiGetClassDevsW(GUID_DEVINTERFACE_HID.as_ptr(), std::ptr::null(), std::ptr::null_mut(), 0x12);
            assert!(!set.is_null());
            let mut i = 0u32;
            let mut checked = false;
            loop {
                let mut dd = [0u8; 32];
                dd[0] = 32;
                // Poison LastError before calling SetupDiEnumDeviceInfo & SetupDiGetDeviceRegistryPropertyW.
                crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
                if SetupDiEnumDeviceInfo(set, i, dd.as_mut_ptr()) == FALSE {
                    break;
                }
                assert_eq!(crate::teb::last_error(), 0, "SetupDiEnumDeviceInfo must clear stale LastError on success");

                let mut ty = 0u32;
                let mut req = 0u32;
                assert_eq!(
                    SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), 0x00, &mut ty, std::ptr::null_mut(), 0, &mut req),
                    FALSE
                );
                assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);

                let mut buf = vec![0u8; req as usize];
                assert_eq!(
                    SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), 0x00, &mut ty, buf.as_mut_ptr(), req, &mut req),
                    TRUE
                );
                assert_eq!(
                    crate::teb::last_error(),
                    0,
                    "SetupDiGetDeviceRegistryPropertyW must clear stale LastError on success"
                );
                let w: Vec<u16> = buf.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&c| c != 0).collect();
                let desc = String::from_utf16(&w).unwrap();
                if desc.contains("Wireless Controller") || desc.contains("DualSense") {
                    checked = true;
                    break;
                }
                i += 1;
            }
            assert!(checked, "SPDRP_DEVICEDESC should report device product string");
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);
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

    #[test]
    fn enumerates_pci_display_adapter_and_opens_driver_key() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        hid::tests::install_fake_dualsense(0xD5_00B1);
        let _ds = FakeDualSenseGuard(0xD5_00B1);
        let expected = primary_display_adapter();

        unsafe {
            // 1. SetupDiGetClassDevsW(NULL, L"PCI", NULL, DIGCF_ALLCLASSES): returns only PCI display adapter (excludes HID).
            let pci_w = crate::strings::to_wide_z("PCI");
            let set = SetupDiGetClassDevsW(std::ptr::null(), pci_w.as_ptr(), std::ptr::null_mut(), DIGCF_ALLCLASSES);
            assert!(!set.is_null());

            let mut dd = [0u8; 32];
            dd[0] = 32;
            assert_eq!(SetupDiEnumDeviceInfo(set, 0, dd.as_mut_ptr()), TRUE);
            assert_eq!(&dd[4..20], &GUID_DEVCLASS_DISPLAY);
            assert_eq!(SetupDiEnumDeviceInfo(set, 1, dd.as_mut_ptr()), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_NO_MORE_ITEMS);

            // Helper to read REG_SZ / first entry of REG_MULTI_SZ via SetupDiGetDeviceRegistryPropertyW.
            let read_prop = |prop: u32| -> (u32, String) {
                let mut ty = 0u32;
                let mut req = 0u32;
                let mut buf = [0u8; 1024];
                assert_eq!(
                    SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), prop, &mut ty, buf.as_mut_ptr(), buf.len() as u32, &mut req),
                    TRUE
                );
                assert_eq!(crate::teb::last_error(), 0);
                let w: Vec<u16> = buf[..req as usize]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .take_while(|&u| u != 0)
                    .collect();
                (ty, String::from_utf16(&w).unwrap())
            };

            assert_eq!(read_prop(0x07), (REG_SZ, "Display".to_string()));
            assert_eq!(read_prop(0x08), (REG_SZ, DISPLAY_CLASS_GUID_STR.to_string()));
            assert_eq!(read_prop(0x09), (REG_SZ, DISPLAY_DRIVER_KEY_STR.to_string()));
            assert_eq!(read_prop(0x00), (REG_SZ, expected.name.clone()));
            assert_eq!(read_prop(0x0C), (REG_SZ, expected.name.clone()));
            assert_eq!(read_prop(0x16), (REG_SZ, "PCI".to_string()));
            assert_eq!(read_prop(0x01), (REG_MULTI_SZ, expected.pci_id.clone()));
            assert_eq!(read_prop(SPDRP_BASE_CONTAINERID), (REG_SZ, format_guid(&INTERNAL_CONTAINER_GUID)));

            // Instance ID matches PCI\VEN_...
            let mut inst_w = [0u16; 256];
            let mut req_ch = 0u32;
            assert_eq!(
                SetupDiGetDeviceInstanceIdW(set, dd.as_ptr(), inst_w.as_mut_ptr(), inst_w.len() as u32, &mut req_ch),
                TRUE
            );
            assert_eq!(crate::strings::from_wide(inst_w.as_ptr()), expected.instance_id);

            // SetupDiOpenDevRegKey(..., DIREG_DRV, ...) opens the seeded display driver registry key.
            let hkey = SetupDiOpenDevRegKey(set, dd.as_ptr(), 1, 0, DIREG_DRV, 0x20019);
            assert_ne!(hkey, INVALID_HANDLE_VALUE);
            let val_name = crate::strings::to_wide_z("DriverVersion");
            let mut rty = 0u32;
            let mut rbuf = [0u8; 128];
            let mut rlen = rbuf.len() as u32;
            assert_eq!(
                crate::advapi32::RegQueryValueExW(hkey, val_name.as_ptr(), std::ptr::null_mut(), &mut rty, rbuf.as_mut_ptr(), &mut rlen),
                0
            );
            assert_eq!(
                crate::strings::from_wide(rbuf.as_ptr() as *const u16),
                crate::dxgi::adapter::UMD_DRIVER_VERSION_STR
            );
            assert_eq!(crate::advapi32::RegCloseKey(hkey), 0);
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);

            // 2. SetupDiGetClassDevsA(NULL, "PCI", NULL, DIGCF_ALLCLASSES) & GUID_DEVINTERFACE_DISPLAY_ADAPTER.
            let set_a = SetupDiGetClassDevsA(std::ptr::null(), b"PCI\0".as_ptr(), std::ptr::null_mut(), DIGCF_ALLCLASSES);
            assert!(!set_a.is_null());
            let mut dd_a = [0u8; 32];
            dd_a[0] = 32;
            assert_eq!(SetupDiEnumDeviceInfo(set_a, 0, dd_a.as_mut_ptr()), TRUE);
            let mut ifd = [0u8; 32];
            ifd[0] = 32;
            assert_eq!(
                SetupDiEnumDeviceInterfaces(set_a, std::ptr::null(), GUID_DEVINTERFACE_DISPLAY_ADAPTER.as_ptr(), 0, ifd.as_mut_ptr()),
                TRUE
            );
            let mut need = 0u32;
            assert_eq!(
                SetupDiGetDeviceInterfaceDetailW(set_a, ifd.as_ptr(), std::ptr::null_mut(), 0, &mut need, std::ptr::null_mut()),
                FALSE
            );
            let mut detail = vec![0u8; need as usize];
            detail[0] = 8;
            assert_eq!(
                SetupDiGetDeviceInterfaceDetailW(set_a, ifd.as_ptr(), detail.as_mut_ptr(), need, &mut need, std::ptr::null_mut()),
                TRUE
            );
            let wpath: Vec<u16> = detail[4..].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&u| u != 0).collect();
            assert_eq!(String::from_utf16(&wpath).unwrap(), expected.interface_path);
            assert_eq!(SetupDiDestroyDeviceInfoList(set_a), TRUE);

            // 3. EnumDisplayDevicesW reports matching GPU name and PCI hardware ID.
            let mut disp_dev = [0u8; 840];
            std::ptr::write_unaligned(disp_dev.as_mut_ptr() as *mut u32, 840u32);
            assert_eq!(
                crate::user32::monitor::EnumDisplayDevicesW(std::ptr::null(), 0, disp_dev.as_mut_ptr(), 0),
                TRUE
            );
            let dev_str = crate::strings::from_wide(disp_dev.as_ptr().add(68) as *const u16);
            let dev_id = crate::strings::from_wide(disp_dev.as_ptr().add(328) as *const u16);
            assert_eq!(dev_str, expected.name);
            assert_eq!(dev_id, expected.pci_id);
        }
    }

    #[test]
    fn filtered_hid_snapshot_preserves_instance_id_and_dualsense_slot() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        hid::tests::install_fake_dualsense(0xD5_00C1);
        let _ds1 = FakeDualSenseGuard(0xD5_00C1);
        hid::tests::install_fake_dualsense(0xD5_00C2);
        let _ds2 = FakeDualSenseGuard(0xD5_00C2);

        let all = hid_entries_with_identity(hid::devices());
        let second = all
            .iter()
            .find(|e| matches!(e, DevEntry::Hid { dev, .. } if dev.desc.id == 0xD5_00C2))
            .expect("second fake DualSense present");
        let (expected_inst_id, expected_dev_inst, expected_slot, expected_path) = match second {
            DevEntry::Hid { dev, hid_index, dualsense_slot } => {
                assert!(*hid_index >= 1, "second DualSense must have nonzero hid_index");
                assert!(*dualsense_slot >= 1, "second USB DualSense must have nonzero slot");
                (second.instance_id(), dev.inst, *dualsense_slot, dev.path.clone())
            }
            DevEntry::Display(_) => unreachable!(),
        };
        let expected_cid = dualsense_usb_container_id(expected_slot);
        let expected_cid_bytes = guid_to_bytes(&expected_cid);
        let expected_cid_str = format_guid(&expected_cid);

        unsafe {
            // Confirm CM_Get_DevNode_PropertyW agrees on the second DualSense's container ID.
            let mut key_cid = [0u8; 20];
            key_cid[0..16].copy_from_slice(&DEVPKEY_DEVICE_CONTAINERID_FMTID);
            key_cid[16..20].copy_from_slice(&DEVPKEY_DEVICE_CONTAINERID_PID.to_le_bytes());
            let mut cm_ty = 0u32;
            let mut cm_buf = [0u8; 16];
            let mut cm_len = 16u32;
            assert_eq!(
                CM_Get_DevNode_PropertyW(
                    expected_dev_inst,
                    key_cid.as_ptr(),
                    &mut cm_ty,
                    cm_buf.as_mut_ptr(),
                    &mut cm_len,
                    0
                ),
                0
            );
            assert_eq!(cm_buf, expected_cid_bytes);

            // 1. Instance-ID enumerator filter selects only the second DualSense into index 0.
            let enum_w = crate::strings::to_wide_z(&expected_inst_id);
            let set = SetupDiGetClassDevsW(
                std::ptr::null(),
                enum_w.as_ptr(),
                std::ptr::null_mut(),
                DIGCF_ALLCLASSES,
            );
            assert!(!set.is_null());
            let mut dd = [0u8; 32];
            dd[0] = 32;
            assert_eq!(SetupDiEnumDeviceInfo(set, 0, dd.as_mut_ptr()), TRUE);
            assert_eq!(SetupDiEnumDeviceInfo(set, 1, dd.as_mut_ptr()), FALSE);

            let mut inst_w = [0u16; 256];
            let mut req_ch = 0u32;
            assert_eq!(
                SetupDiGetDeviceInstanceIdW(set, dd.as_ptr(), inst_w.as_mut_ptr(), inst_w.len() as u32, &mut req_ch),
                TRUE
            );
            assert_eq!(crate::strings::from_wide(inst_w.as_ptr()), expected_inst_id);

            let mut inst_a = [0u8; 256];
            assert_eq!(
                SetupDiGetDeviceInstanceIdA(set, dd.as_ptr(), inst_a.as_mut_ptr(), inst_a.len() as u32, &mut req_ch),
                TRUE
            );
            let nul = inst_a.iter().position(|&b| b == 0).unwrap();
            assert_eq!(std::str::from_utf8(&inst_a[..nul]).unwrap(), expected_inst_id);

            let mut pty = 0u32;
            let mut preq = 0u32;
            let mut gbuf = [0u8; 16];
            assert_eq!(
                SetupDiGetDevicePropertyW(set, dd.as_ptr(), key_cid.as_ptr(), &mut pty, gbuf.as_mut_ptr(), 16, &mut preq, 0),
                TRUE
            );
            assert_eq!(gbuf, expected_cid_bytes);

            let mut rty = 0u32;
            let mut rreq = 0u32;
            let mut wbuf = [0u8; 256];
            assert_eq!(
                SetupDiGetDeviceRegistryPropertyW(set, dd.as_ptr(), SPDRP_BASE_CONTAINERID, &mut rty, wbuf.as_mut_ptr(), 256, &mut rreq),
                TRUE
            );
            assert_eq!(crate::strings::from_wide(wbuf.as_ptr() as *const u16), expected_cid_str);
            assert_eq!(SetupDiDestroyDeviceInfoList(set), TRUE);

            // 2. Opening the second DualSense interface into an empty set also preserves its identity & slot.
            let empty_guid = [0x33u8; 16];
            let set2 = SetupDiGetClassDevsW(empty_guid.as_ptr(), std::ptr::null(), std::ptr::null_mut(), DIGCF_DEVICEINTERFACE);
            let path_w = crate::strings::to_wide_z(&expected_path);
            let mut ifd = [0u8; 32];
            ifd[0] = 32;
            assert_eq!(SetupDiOpenDeviceInterfaceW(set2, path_w.as_ptr(), 0, ifd.as_mut_ptr()), TRUE);

            let mut gbuf2 = [0u8; 16];
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set2, ifd.as_ptr(), key_cid.as_ptr(), &mut pty, gbuf2.as_mut_ptr(), 16, &mut preq, 0),
                TRUE
            );
            assert_eq!(gbuf2, expected_cid_bytes);

            let mut key_id = [0u8; 20];
            key_id[0..16].copy_from_slice(&DEVPKEY_DEVICE_INSTANCEID_FMTID);
            key_id[16..20].copy_from_slice(&DEVPKEY_DEVICE_INSTANCEID_PID.to_le_bytes());
            let mut sbuf = [0u8; 256];
            assert_eq!(
                SetupDiGetDeviceInterfacePropertyW(set2, ifd.as_ptr(), key_id.as_ptr(), &mut pty, sbuf.as_mut_ptr(), 256, &mut preq, 0),
                TRUE
            );
            assert_eq!(crate::strings::from_wide(sbuf.as_ptr() as *const u16), expected_inst_id);
            assert_eq!(SetupDiDestroyDeviceInfoList(set2), TRUE);
        }
    }
}
