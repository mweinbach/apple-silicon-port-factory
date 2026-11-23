//! `directml.dll`: Native DirectML (`IDMLDevice` / `IDMLDevice1`) runtime on D3D12.
//!
//! Provides real DirectML COM objects (every vtable slot registered via
//! [`crate::com::register_vtable_once`]) and real CPU-backed D3D12 buffer tensor
//! execution inside `IDMLCommandRecorder::RecordDispatch` (handling both
//! `IDMLCompiledOperator` dispatch and `IDMLOperatorInitializer` initialization),
//! accessing D3D12 buffer memory strictly through [`crate::d3d12`] and
//! `d3d12_metal` safe Rust wrappers (zero `objc2-metal` imports).

mod abi;
mod com_objs;
mod graph;
mod ops;

pub use abi::*;

use std::ffi::c_void;

use crate::com::{com_call_release, read_guid, write_null_out, Guid};
use crate::d3d12::com::DXGI_ERROR_UNSUPPORTED;
use crate::strings::wr;
use crate::types::{E_INVALIDARG, E_OUTOFMEMORY, E_POINTER, HRESULT, S_OK};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// DLL Exports: DMLCreateDevice, DMLCreateDevice1, DMLCreateDeviceAndDirect3D12Device
// ============================================================================

pub unsafe extern "win64" fn DMLCreateDevice(
    d3d12_device: *mut c_void,
    flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    DMLCreateDevice1(d3d12_device, flags, DML_FEATURE_LEVEL_1_0, riid, ppv)
}

pub unsafe extern "win64" fn DMLCreateDevice1(
    d3d12_device: *mut c_void,
    flags: u32,
    minimum_feature_level: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if d3d12_device.is_null() || (flags & !DML_CREATE_DEVICE_FLAG_DEBUG) != 0 {
        return E_INVALIDARG;
    }
    if minimum_feature_level < DML_FEATURE_LEVEL_1_0 {
        return E_INVALIDARG;
    }
    if minimum_feature_level > abi::MAX_SUPPORTED_FEATURE_LEVEL {
        return DXGI_ERROR_UNSUPPORTED;
    }
    let hr = com_objs::create_dml_device(d3d12_device, flags, &iid, ppv);
    log::info!(target: "d3d", "DMLCreateDevice1(flags={flags:#x}, fl={minimum_feature_level:#x}) -> {hr:#x}");
    hr
}

pub unsafe extern "win64" fn DMLCreateDeviceAndDirect3D12Device(
    adapter: *mut c_void,
    min_d3d_feature_level: u32,
    dml_flags: u32,
    min_dml_feature_level: u32,
    d3d12_riid: *const Guid,
    pp_d3d12_device: *mut *mut c_void,
    dml_riid: *const Guid,
    pp_dml_device: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_d3d12_device);
    write_null_out(pp_dml_device);
    if pp_dml_device.is_null() {
        return E_POINTER;
    }
    let d3d_fl = if min_d3d_feature_level == 0 {
        0xB000 // D3D_FEATURE_LEVEL_11_0
    } else {
        min_d3d_feature_level
    };
    let d3d_iid = read_guid(d3d12_riid).unwrap_or(crate::d3d12::vtables::IID_ID3D12Device);
    let mut d3d_dev: *mut c_void = std::ptr::null_mut();
    let hr = crate::d3d12::D3D12CreateDevice(adapter, d3d_fl, &d3d_iid, &mut d3d_dev);
    if hr < 0 || d3d_dev.is_null() {
        return if hr < 0 { hr } else { E_OUTOFMEMORY };
    }
    let dml_fl = if min_dml_feature_level == 0 {
        DML_FEATURE_LEVEL_1_0
    } else {
        min_dml_feature_level
    };
    let dml_iid_val = read_guid(dml_riid).unwrap_or(IID_IDMLDevice);
    let dml_hr = DMLCreateDevice1(d3d_dev, dml_flags, dml_fl, &dml_iid_val, pp_dml_device);
    if dml_hr < 0 {
        com_call_release(d3d_dev);
        return dml_hr;
    }
    if !pp_d3d12_device.is_null() {
        wr(pp_d3d12_device, d3d_dev);
    } else {
        com_call_release(d3d_dev);
    }
    S_OK
}

#[cfg(test)]
mod tests;
