//! Native `amd_ags_x64.dll` (AMD GPU Services) implementation for Apple Silicon Metal.
//!
//! Engines (RE Engine, Nixxes ports, Snowdrop/Anvil) call `agsInitialize`,
//! `agsDriverExtensionsDX12_CreateDevice`, and `agsDriverExtensionsDX12_DestroyDevice`
//! to create and manage their `ID3D12Device` alongside GPU queries.
//!
//! Reporting is strictly honest:
//! * `agsInitialize` / `agsGetGPUInfo` populate the exact 32-byte `AGSGPUInfo`
//!   and 120-byte `AGSDeviceInfo` from `amd_ags.h` with our real Apple Silicon
//!   Metal adapter (`vendorId = 0x106B`, `asicFamily = AGSAsicFamily_Unknown`,
//!   empty AMD driver strings, real VRAM / shared memory).
//! * `agsDriverExtensionsDX12_CreateDevice` forwards to `crate::d3d12::D3D12CreateDevice`
//!   and reports `extensionsSupported = 0` (no AMD-only shader intrinsics).
//! * `agsDriverExtensionsDX12_DestroyDevice` invokes `IUnknown::Release` on the
//!   D3D12 device and writes the updated reference count to `deviceReferences`.

#![allow(non_snake_case)]

use std::collections::HashSet;
use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// `AGSReturnCode` (`amd_ags.h`).
pub const AGS_SUCCESS: i32 = 0;
pub const AGS_FAILURE: i32 = 1;
pub const AGS_INVALID_ARGS: i32 = 2;
pub const AGS_EXTENSION_NOT_SUPPORTED: i32 = 7;
pub const AGS_DX_FAILURE: i32 = 9;

// `AGSDriverVersionResult` (`amd_ags.h`).
pub const AGS_SOFTWAREVERSIONCHECK_OK: i32 = 0;
pub const AGS_SOFTWAREVERSIONCHECK_OLDER: i32 = 1;
pub const AGS_SOFTWAREVERSIONCHECK_UNDEFINED: i32 = 2;

// `AGSAsicFamily` (`amd_ags.h`).
pub const AGS_ASIC_FAMILY_UNKNOWN: u32 = 0;

static EMPTY_CSTR: &[u8] = b"\0";

/// `AGSDeviceInfo` (`amd_ags.h`, 120 bytes on x86_64 Windows).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AGSDeviceInfo {
    pub adapterString: *const u8,
    pub asicFamily: u32,
    pub flags: u32,
    pub vendorId: i32,
    pub deviceId: i32,
    pub revisionId: i32,
    pub numCUs: i32,
    pub numWGPs: i32,
    pub numROPs: i32,
    pub coreClock: i32,
    pub memoryClock: i32,
    pub memoryBandwidth: i32,
    pub teraFlops: f32,
    pub localMemoryInBytes: u64,
    pub sharedMemoryInBytes: u64,
    pub numDisplays: i32,
    pub displays: *mut c_void,
    pub eyefinityEnabled: i32,
    pub eyefinityGridWidth: i32,
    pub eyefinityGridHeight: i32,
    pub eyefinityResolutionX: i32,
    pub eyefinityResolutionY: i32,
    pub eyefinityBezelCompensated: i32,
    pub adlAdapterIndex: i32,
    pub reserved: i32,
}
const _: () = assert!(std::mem::size_of::<AGSDeviceInfo>() == 0x78);
const _: () = assert!(std::mem::offset_of!(AGSDeviceInfo, vendorId) == 0x10);
const _: () = assert!(std::mem::offset_of!(AGSDeviceInfo, localMemoryInBytes) == 0x38);
const _: () = assert!(std::mem::offset_of!(AGSDeviceInfo, sharedMemoryInBytes) == 0x40);
const _: () = assert!(std::mem::offset_of!(AGSDeviceInfo, displays) == 0x50);
const _: () = assert!(std::mem::offset_of!(AGSDeviceInfo, adlAdapterIndex) == 0x70);

/// `AGSGPUInfo` (`amd_ags.h`, 32 bytes on x86_64 Windows).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AGSGPUInfo {
    pub driverVersion: *const u8,
    pub radeonSoftwareVersion: *const u8,
    pub numDevices: i32,
    pub devices: *mut AGSDeviceInfo,
}
const _: () = assert!(std::mem::size_of::<AGSGPUInfo>() == 32);
const _: () = assert!(std::mem::offset_of!(AGSGPUInfo, numDevices) == 0x10);
const _: () = assert!(std::mem::offset_of!(AGSGPUInfo, devices) == 0x18);

/// `AGSDX12DeviceCreationParams` (`amd_ags.h`, 32 bytes on x86_64 Windows).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AGSDX12DeviceCreationParams {
    pub pAdapter: usize,
    pub iid: [u8; 16],
    pub FeatureLevel: u32,
}
const _: () = assert!(std::mem::size_of::<AGSDX12DeviceCreationParams>() == 32);
const _: () = assert!(std::mem::offset_of!(AGSDX12DeviceCreationParams, FeatureLevel) == 0x18);

/// `AGSDX12ReturnedParams` (`amd_ags.h`, 16 bytes on x86_64 Windows).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AGSDX12ReturnedParams {
    pub pDevice: usize,
    pub extensionsSupported: u32,
}
const _: () = assert!(std::mem::size_of::<AGSDX12ReturnedParams>() == 16);
const _: () = assert!(std::mem::offset_of!(AGSDX12ReturnedParams, extensionsSupported) == 0x08);

struct AgsContext {
    adapter_name: *mut c_char,
    devices: *mut AGSDeviceInfo,
    num_devices: usize,
}

unsafe impl Send for AgsContext {}

impl AgsContext {
    fn new() -> Self {
        if let Some(d) = crate::d3d12::mtl::device() {
            let clean_name: String = d.name.chars().filter(|&c| c != '\0').collect();
            let cstr = CString::new(clean_name).unwrap_or_else(|_| CString::new("Metal").unwrap());
            let name_ptr = cstr.into_raw();
            let vram = d.reported_vram();
            let shared = crate::d3d12::mtl::physical_memory().saturating_sub(vram);
            // bit 0 = isAPU, bit 1 = isPrimaryDevice
            let flags = u32::from(d.unified_memory) | (1 << 1);
            let dev = AGSDeviceInfo {
                adapterString: name_ptr as *const u8,
                asicFamily: AGS_ASIC_FAMILY_UNKNOWN,
                flags,
                vendorId: crate::dxgi::adapter::VENDOR_APPLE as i32,
                deviceId: d.pci_device_id() as i32,
                revisionId: 0,
                numCUs: 0,
                numWGPs: 0,
                numROPs: 0,
                coreClock: 0,
                memoryClock: 0,
                memoryBandwidth: 0,
                teraFlops: 0.0,
                localMemoryInBytes: vram,
                sharedMemoryInBytes: shared,
                numDisplays: 0,
                displays: std::ptr::null_mut(),
                eyefinityEnabled: 0,
                eyefinityGridWidth: 0,
                eyefinityGridHeight: 0,
                eyefinityResolutionX: 0,
                eyefinityResolutionY: 0,
                eyefinityBezelCompensated: 0,
                adlAdapterIndex: -1,
                reserved: 0,
            };
            let boxed_slice: Box<[AGSDeviceInfo]> = vec![dev].into_boxed_slice();
            let devices_ptr = Box::into_raw(boxed_slice) as *mut AGSDeviceInfo;
            Self {
                adapter_name: name_ptr,
                devices: devices_ptr,
                num_devices: 1,
            }
        } else {
            Self {
                adapter_name: std::ptr::null_mut(),
                devices: std::ptr::null_mut(),
                num_devices: 0,
            }
        }
    }

    fn gpu_info(&self) -> AGSGPUInfo {
        AGSGPUInfo {
            driverVersion: EMPTY_CSTR.as_ptr(),
            radeonSoftwareVersion: EMPTY_CSTR.as_ptr(),
            numDevices: self.num_devices as i32,
            devices: self.devices,
        }
    }
}

impl Drop for AgsContext {
    fn drop(&mut self) {
        unsafe {
            if !self.devices.is_null() && self.num_devices > 0 {
                let slice_ptr =
                    std::ptr::slice_from_raw_parts_mut(self.devices, self.num_devices);
                drop(Box::from_raw(slice_ptr));
            }
            if !self.adapter_name.is_null() {
                drop(CString::from_raw(self.adapter_name));
            }
        }
    }
}

fn ags_contexts() -> &'static Mutex<HashSet<usize>> {
    static SET: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();
    SET.get_or_init(|| Mutex::new(HashSet::new()))
}

fn cmdlist_slot(method: &str) -> usize {
    crate::d3d12::vtables::ID3D12GraphicsCommandList10
        .iter()
        .position(|&name| name == method)
        .expect("ID3D12GraphicsCommandList10 marker method")
}

fn begin_event_slot() -> usize {
    static SLOT: OnceLock<usize> = OnceLock::new();
    *SLOT.get_or_init(|| cmdlist_slot("BeginEvent"))
}

fn end_event_slot() -> usize {
    static SLOT: OnceLock<usize> = OnceLock::new();
    *SLOT.get_or_init(|| cmdlist_slot("EndEvent"))
}

fn set_marker_slot() -> usize {
    static SLOT: OnceLock<usize> = OnceLock::new();
    *SLOT.get_or_init(|| cmdlist_slot("SetMarker"))
}

fn parse_radeon_version(s: &str) -> Option<u32> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    let patch: u32 = match parts.next() {
        Some(p) => p.parse().ok()?,
        None => 0,
    };
    Some((major << 22) | ((minor & 0x3FF) << 12) | (patch & 0xFFF))
}

pub extern "win64" fn agsInitialize(
    _ags_version: i32,
    _config: usize,
    context_out: *mut usize,
    gpu_info_out: *mut AGSGPUInfo,
) -> i32 {
    if context_out.is_null() {
        return AGS_INVALID_ARGS;
    }
    let ctx = Box::new(AgsContext::new());
    let info = ctx.gpu_info();
    let ptr = Box::into_raw(ctx) as usize;
    ags_contexts().lock().unwrap().insert(ptr);
    unsafe {
        std::ptr::write_unaligned(context_out, ptr);
        if !gpu_info_out.is_null() {
            std::ptr::write_unaligned(gpu_info_out, info);
        }
    }
    AGS_SUCCESS
}

pub extern "win64" fn agsDeInitialize(context: usize) -> i32 {
    if context == 0 || !ags_contexts().lock().unwrap().remove(&context) {
        return AGS_INVALID_ARGS;
    }
    unsafe { drop(Box::from_raw(context as *mut AgsContext)) };
    AGS_SUCCESS
}

pub extern "win64" fn agsGetGPUInfo(context: usize, gpu_info_out: *mut AGSGPUInfo) -> i32 {
    if context == 0 || gpu_info_out.is_null() {
        return AGS_INVALID_ARGS;
    }
    let guard = ags_contexts().lock().unwrap();
    if !guard.contains(&context) {
        return AGS_INVALID_ARGS;
    }
    let info = unsafe { (*(context as *const AgsContext)).gpu_info() };
    drop(guard);
    unsafe { std::ptr::write_unaligned(gpu_info_out, info) };
    AGS_SUCCESS
}

pub extern "win64" fn agsCheckDriverVersion(
    radeon_software_version: *const u8,
    minimum_version: u32,
) -> i32 {
    if radeon_software_version.is_null() {
        return AGS_SOFTWAREVERSIONCHECK_UNDEFINED;
    }
    let text = unsafe { CStr::from_ptr(radeon_software_version as *const c_char) }
        .to_string_lossy();
    match parse_radeon_version(&text) {
        Some(reported) if reported >= minimum_version => AGS_SOFTWAREVERSIONCHECK_OK,
        Some(_) => AGS_SOFTWAREVERSIONCHECK_OLDER,
        None => AGS_SOFTWAREVERSIONCHECK_UNDEFINED,
    }
}

pub extern "win64" fn agsGetVersionNumber() -> i32 {
    // AGS 6.2.0
    (6 << 22) | (2 << 12)
}

pub extern "win64" fn agsDriverExtensionsDX12_CreateDevice(
    _context: usize,
    creation_params: *const AGSDX12DeviceCreationParams,
    _extension_params: usize,
    returned_params: *mut AGSDX12ReturnedParams,
) -> i32 {
    if creation_params.is_null() || returned_params.is_null() {
        return AGS_INVALID_ARGS;
    }
    let cp = unsafe { std::ptr::read_unaligned(creation_params) };
    let mut dev_ptr: *mut c_void = std::ptr::null_mut();
    let hr = unsafe {
        crate::d3d12::D3D12CreateDevice(
            cp.pAdapter as *mut c_void,
            cp.FeatureLevel,
            cp.iid.as_ptr() as *const crate::d3d12::com::Guid,
            &mut dev_ptr,
        )
    };
    if hr < 0 {
        unsafe {
            std::ptr::write_unaligned(
                returned_params,
                AGSDX12ReturnedParams {
                    pDevice: 0,
                    extensionsSupported: 0,
                },
            );
        }
        return AGS_DX_FAILURE;
    }
    unsafe {
        // Apple Silicon Metal exposes no AMD-only DX12 shader intrinsics or RGP markers.
        std::ptr::write_unaligned(
            returned_params,
            AGSDX12ReturnedParams {
                pDevice: dev_ptr as usize,
                extensionsSupported: 0,
            },
        );
    }
    AGS_SUCCESS
}

pub extern "win64" fn agsDriverExtensionsDX12_DestroyDevice(
    _context: usize,
    device: usize,
    device_refs: *mut u32,
) -> i32 {
    if device == 0 {
        if !device_refs.is_null() {
            unsafe { std::ptr::write_unaligned(device_refs, 0) };
        }
        return AGS_INVALID_ARGS;
    }
    let refs = unsafe {
        let vtbl = std::ptr::read_unaligned(device as *const usize);
        // IUnknown::Release is vtable slot 2 (+0x10).
        let release = std::ptr::read_unaligned((vtbl + 0x10) as *const usize);
        crate::call_guest8(release, device as u64, 0, 0, 0, 0, 0, 0, 0) as u32
    };
    if !device_refs.is_null() {
        unsafe { std::ptr::write_unaligned(device_refs, refs) };
    }
    AGS_SUCCESS
}

pub extern "win64" fn agsDriverExtensionsDX12_PushMarker(
    _context: usize,
    cmd_list: usize,
    data: *const u8,
) -> i32 {
    if cmd_list == 0 || data.is_null() {
        return AGS_INVALID_ARGS;
    }
    let list_ptr = cmd_list as *mut c_void;
    if unsafe { crate::d3d12::cmdlist::from_ptr(list_ptr) }.is_none() {
        // Native-unsupported (AMD GPUOpen AGS SDK `amd_ags.h`): on non-AMD GPUs
        // `agsDriverExtensionsDX12_CreateDevice` reports `extensionsSupported = 0`,
        // and marker calls without an AMD driver extension return `AGS_EXTENSION_NOT_SUPPORTED` (7).
        log::debug!("agsDriverExtensionsDX12_PushMarker: non-AMD/foreign command list {cmd_list:#x} -> AGS_EXTENSION_NOT_SUPPORTED");
        return AGS_EXTENSION_NOT_SUPPORTED;
    }
    unsafe {
        let len = CStr::from_ptr(data as *const c_char).to_bytes_with_nul().len() as u32;
        let vtbl = std::ptr::read_unaligned(cmd_list as *const *const usize);
        let begin_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, u32) =
            std::mem::transmute(std::ptr::read_unaligned(vtbl.add(begin_event_slot())));
        begin_fn(list_ptr, 1, data as *const c_void, len);
    }
    AGS_SUCCESS
}

pub extern "win64" fn agsDriverExtensionsDX12_PopMarker(
    _context: usize,
    cmd_list: usize,
) -> i32 {
    if cmd_list == 0 {
        return AGS_INVALID_ARGS;
    }
    let list_ptr = cmd_list as *mut c_void;
    if unsafe { crate::d3d12::cmdlist::from_ptr(list_ptr) }.is_none() {
        // Native-unsupported (AMD GPUOpen AGS SDK `amd_ags.h`): returns
        // `AGS_EXTENSION_NOT_SUPPORTED` (7) when AMD driver extensions are absent.
        log::debug!("agsDriverExtensionsDX12_PopMarker: non-AMD/foreign command list {cmd_list:#x} -> AGS_EXTENSION_NOT_SUPPORTED");
        return AGS_EXTENSION_NOT_SUPPORTED;
    }
    unsafe {
        let vtbl = std::ptr::read_unaligned(cmd_list as *const *const usize);
        let end_fn: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(std::ptr::read_unaligned(vtbl.add(end_event_slot())));
        end_fn(list_ptr);
    }
    AGS_SUCCESS
}

pub extern "win64" fn agsDriverExtensionsDX12_SetMarker(
    _context: usize,
    cmd_list: usize,
    data: *const u8,
) -> i32 {
    if cmd_list == 0 || data.is_null() {
        return AGS_INVALID_ARGS;
    }
    let list_ptr = cmd_list as *mut c_void;
    if unsafe { crate::d3d12::cmdlist::from_ptr(list_ptr) }.is_none() {
        // Native-unsupported (AMD GPUOpen AGS SDK `amd_ags.h`): returns
        // `AGS_EXTENSION_NOT_SUPPORTED` (7) when AMD driver extensions are absent.
        log::debug!("agsDriverExtensionsDX12_SetMarker: non-AMD/foreign command list {cmd_list:#x} -> AGS_EXTENSION_NOT_SUPPORTED");
        return AGS_EXTENSION_NOT_SUPPORTED;
    }
    unsafe {
        let len = CStr::from_ptr(data as *const c_char).to_bytes_with_nul().len() as u32;
        let vtbl = std::ptr::read_unaligned(cmd_list as *const *const usize);
        let marker_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, u32) =
            std::mem::transmute(std::ptr::read_unaligned(vtbl.add(set_marker_slot())));
        marker_fn(list_ptr, 1, data as *const c_void, len);
    }
    AGS_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn ags_init_populates_honest_gpu_info_and_deinitializes() {
        assert_eq!(
            agsInitialize(0, 0, std::ptr::null_mut(), std::ptr::null_mut()),
            AGS_INVALID_ARGS
        );

        let mut ctx = 0usize;
        let mut info = AGSGPUInfo {
            driverVersion: std::ptr::null(),
            radeonSoftwareVersion: std::ptr::null(),
            numDevices: -1,
            devices: std::ptr::null_mut(),
        };
        assert_eq!(agsInitialize(agsGetVersionNumber(), 0, &mut ctx, &mut info), AGS_SUCCESS);
        assert_ne!(ctx, 0);
        assert!(!info.driverVersion.is_null());
        assert!(!info.radeonSoftwareVersion.is_null());

        unsafe {
            assert_eq!(
                CStr::from_ptr(info.driverVersion as *const c_char).to_bytes(),
                b""
            );
            assert_eq!(
                CStr::from_ptr(info.radeonSoftwareVersion as *const c_char).to_bytes(),
                b""
            );

            if let Some(mtl) = crate::d3d12::mtl::device() {
                assert_eq!(info.numDevices, 1);
                assert!(!info.devices.is_null());
                let dev = std::ptr::read_unaligned(info.devices);
                assert_eq!(dev.vendorId, crate::dxgi::adapter::VENDOR_APPLE as i32);
                assert_eq!(dev.deviceId, mtl.pci_device_id() as i32);
                assert_eq!(dev.asicFamily, AGS_ASIC_FAMILY_UNKNOWN);
                assert_eq!(dev.localMemoryInBytes, mtl.reported_vram());
                assert!(!dev.adapterString.is_null());
                let name = CStr::from_ptr(dev.adapterString as *const c_char).to_string_lossy();
                assert_eq!(name, mtl.name);
            } else {
                assert_eq!(info.numDevices, 0);
                assert!(info.devices.is_null());
            }
        }

        let mut info2 = AGSGPUInfo {
            driverVersion: std::ptr::null(),
            radeonSoftwareVersion: std::ptr::null(),
            numDevices: -1,
            devices: std::ptr::null_mut(),
        };
        assert_eq!(agsGetGPUInfo(ctx, &mut info2), AGS_SUCCESS);
        assert_eq!(info2.numDevices, info.numDevices);

        assert_eq!(agsDeInitialize(ctx), AGS_SUCCESS);
        assert_eq!(agsDeInitialize(ctx), AGS_INVALID_ARGS);
        assert_eq!(agsGetGPUInfo(ctx, &mut info2), AGS_INVALID_ARGS);
    }

    #[test]
    fn ags_destroy_device_invokes_iunknown_release() {
        #[repr(C)]
        struct MockDevice {
            #[allow(dead_code)]
            vtbl: *const [usize; 3],
            refs: AtomicU32,
        }
        unsafe extern "win64" fn mock_qi(_: *mut MockDevice) -> i32 {
            0
        }
        unsafe extern "win64" fn mock_add_ref(this: *mut MockDevice) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn mock_release(this: *mut MockDevice) -> u32 {
            (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1
        }

        let vtbl = [
            mock_qi as *const () as usize,
            mock_add_ref as *const () as usize,
            mock_release as *const () as usize,
        ];
        let mut dev = MockDevice {
            vtbl: &vtbl,
            refs: AtomicU32::new(2),
        };
        let dev_ptr = &mut dev as *mut MockDevice as usize;

        let mut refs = 99u32;
        assert_eq!(
            agsDriverExtensionsDX12_DestroyDevice(0, 0, &mut refs),
            AGS_INVALID_ARGS
        );
        assert_eq!(refs, 0);

        assert_eq!(
            agsDriverExtensionsDX12_DestroyDevice(0, dev_ptr, &mut refs),
            AGS_SUCCESS
        );
        assert_eq!(refs, 1);
        assert_eq!(dev.refs.load(Ordering::Acquire), 1);

        assert_eq!(
            agsDriverExtensionsDX12_DestroyDevice(0, dev_ptr, &mut refs),
            AGS_SUCCESS
        );
        assert_eq!(refs, 0);
        assert_eq!(dev.refs.load(Ordering::Acquire), 0);
    }

    #[test]
    fn ags_create_and_destroy_real_d3d12_device_when_metal_available() {
        assert_eq!(
            agsDriverExtensionsDX12_CreateDevice(0, std::ptr::null(), 0, std::ptr::null_mut()),
            AGS_INVALID_ARGS
        );
        assert_eq!(
            agsDriverExtensionsDX12_PushMarker(0, 0, b"pass\0".as_ptr()),
            AGS_INVALID_ARGS
        );
        assert_eq!(
            agsDriverExtensionsDX12_PopMarker(0, 0),
            AGS_INVALID_ARGS
        );
        assert_eq!(
            agsDriverExtensionsDX12_SetMarker(0, 0, b"pt\0".as_ptr()),
            AGS_INVALID_ARGS
        );
        // Foreign / non-AMD command list returns AGS_EXTENSION_NOT_SUPPORTED (7).
        let mut fake_list = 0usize;
        let fake_ptr = &mut fake_list as *mut usize as usize;
        assert_eq!(
            agsDriverExtensionsDX12_PushMarker(0, fake_ptr, b"pass\0".as_ptr()),
            AGS_EXTENSION_NOT_SUPPORTED
        );
        assert_eq!(
            agsDriverExtensionsDX12_PopMarker(0, fake_ptr),
            AGS_EXTENSION_NOT_SUPPORTED
        );
        assert_eq!(
            agsDriverExtensionsDX12_SetMarker(0, fake_ptr, b"pt\0".as_ptr()),
            AGS_EXTENSION_NOT_SUPPORTED
        );

        if crate::d3d12::mtl::device().is_none() {
            return;
        }
        let mut ctx = 0usize;
        assert_eq!(
            agsInitialize(agsGetVersionNumber(), 0, &mut ctx, std::ptr::null_mut()),
            AGS_SUCCESS
        );
        let iid_bytes: [u8; 16] =
            unsafe { std::mem::transmute(crate::d3d12::vtables::IID_ID3D12Device) };
        let cp = AGSDX12DeviceCreationParams {
            pAdapter: 0,
            iid: iid_bytes,
            FeatureLevel: 0xc100, // D3D_FEATURE_LEVEL_12_1
        };
        let mut ret = AGSDX12ReturnedParams {
            pDevice: 0,
            extensionsSupported: 0xFFFF,
        };
        assert_eq!(
            agsDriverExtensionsDX12_CreateDevice(ctx, &cp, 0, &mut ret),
            AGS_SUCCESS
        );
        assert_ne!(ret.pDevice, 0);
        assert_eq!(ret.extensionsSupported, 0);

        let mut refs = 99u32;
        assert_eq!(
            agsDriverExtensionsDX12_DestroyDevice(ctx, ret.pDevice, &mut refs),
            AGS_SUCCESS
        );
        assert_eq!(agsDeInitialize(ctx), AGS_SUCCESS);
    }

    #[test]
    fn ags_check_driver_version_is_honest() {
        let min = (24 << 22) | (5 << 12) | 1;
        assert_eq!(
            agsCheckDriverVersion(std::ptr::null(), min),
            AGS_SOFTWAREVERSIONCHECK_UNDEFINED
        );
        assert_eq!(
            agsCheckDriverVersion(b"\0".as_ptr(), min),
            AGS_SOFTWAREVERSIONCHECK_UNDEFINED
        );
        assert_eq!(
            agsCheckDriverVersion(b"24.5.1\0".as_ptr(), min),
            AGS_SOFTWAREVERSIONCHECK_OK
        );
        assert_eq!(
            agsCheckDriverVersion(b"25.1.0\0".as_ptr(), min),
            AGS_SOFTWAREVERSIONCHECK_OK
        );
        assert_eq!(
            agsCheckDriverVersion(b"23.12.1\0".as_ptr(), min),
            AGS_SOFTWAREVERSIONCHECK_OLDER
        );
    }
}
