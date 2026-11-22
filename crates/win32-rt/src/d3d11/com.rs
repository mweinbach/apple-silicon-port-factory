//! Direct3D 11 COM helpers built on `crate::d3d12::com`'s arena/thunk machinery.
//!
//! Vtables are allocated in non-executable DATA memory, jump/trap thunks live
//! separately in executable CODE memory (on x86_64) or direct native pointers
//! registered with `crate::register_native_va` (on aarch64).

pub use crate::d3d12::com::{
    add_ref, build_vtable, class_once, com_add_ref, com_query, com_release, dxgi_object_slots,
    get_private_data, iunknown_slots, qi, query_out, release, set_private_data,
    set_private_data_interface, Class, ComClass, Guid, Obj, DXGI_ERROR_INVALID_CALL,
    DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_FOUND, DXGI_ERROR_UNSUPPORTED,
    DXGI_ERROR_WAS_STILL_DRAWING,
};

use std::ffi::c_void;
use std::sync::Arc;

use super::device::{D3d11Device, DeviceCore};

/// Build the 7 `ID3D11DeviceChild` prefix slots in exact Windows x64 order:
/// `[0] QueryInterface`, `[1] AddRef`, `[2] Release`, `[3] GetDevice`,
/// `[4] GetPrivateData`, `[5] SetPrivateData`, `[6] SetPrivateDataInterface`.
pub fn d3d11_device_child_slots<T: ComClass>(
    get_device_fn: unsafe extern "win64" fn(*mut Obj<T>, *mut *mut c_void),
) -> Vec<(&'static str, Option<*const ()>)> {
    let mut s = iunknown_slots::<T>();
    s.push(("GetDevice", Some(get_device_fn as *const ())));
    s.push(("GetPrivateData", Some(get_private_data::<T> as *const ())));
    s.push(("SetPrivateData", Some(set_private_data::<T> as *const ())));
    s.push(("SetPrivateDataInterface", Some(set_private_data_interface::<T> as *const ())));
    debug_assert_eq!(s.len(), 7);
    s
}

/// Write an AddRef'd `ID3D11Device*` to `out` from a shared `DeviceCore`.
///
/// If the primary `ID3D11Device` COM wrapper is still live, `AddRef`s and
/// returns that exact pointer (preserving COM identity). If the caller already
/// released the outer device while keeping a child alive, returns a fresh
/// wrapper sharing the same `Arc<DeviceCore>`.
pub unsafe fn write_device_out(core: &Arc<DeviceCore>, out: *mut *mut c_void) {
    if out.is_null() {
        return;
    }
    std::ptr::write_unaligned(out, D3d11Device::get_or_wrap(core) as *mut c_void);
}

/// Helper for `D3D11_DEVICE_CHILD` types that hold `core: Arc<DeviceCore>`.
pub trait HasDeviceCore: ComClass {
    fn device_core(&self) -> &Arc<DeviceCore>;
}

pub unsafe extern "win64" fn child_get_device<T: HasDeviceCore>(
    this: *mut Obj<T>,
    out: *mut *mut c_void,
) {
    write_device_out(Obj::get(this).device_core(), out);
}

/// Convenience builder for `ID3D11DeviceChild` classes implementing `HasDeviceCore`.
pub fn child_slots<T: HasDeviceCore>() -> Vec<(&'static str, Option<*const ()>)> {
    d3d11_device_child_slots::<T>(child_get_device::<T>)
}

/// Log a failing D3D11 HRESULT.
pub fn fail(what: std::fmt::Arguments<'_>, hr: crate::types::HRESULT) -> crate::types::HRESULT {
    log::warn!(target: "d3d", "d3d11: {what} -> {hr:#010x}");
    hr
}

macro_rules! d11_failed {
    ($hr:expr, $($arg:tt)*) => {
        $crate::d3d11::com::fail(format_args!($($arg)*), $hr)
    };
}
pub(crate) use d11_failed;
