//! `dsound.dll`: DirectSound 8 & DirectSoundCapture 8 runtime backed by
//! `crate::audio::backend`'s CoreAudio / Null output stream.
//!
//! * Device enumeration (`DirectSoundEnumerateA/W`, `DirectSoundCaptureEnumerateA/W`,
//!   `GetDeviceID`) backed by `crate::audio::backend::output_devices()` and
//!   `input_devices()`, invoking guest enumeration callbacks via `crate::call_guest8`.
//! * Real Win64 COM objects with vtables registered in `VA_DISPATCH` via
//!   `crate::com::register_static_ptr_slice`:
//!   - `IDirectSound` / `IDirectSound8`
//!   - `IDirectSoundBuffer` / `IDirectSoundBuffer8` (circular PCM buffer supporting
//!     PCM 8/16/24/32-bit, IEEE float 32/64-bit, and `WAVE_FORMAT_EXTENSIBLE`,
//!     wrap-around `Lock`/`Unlock`, one-shot & looping `Play`/`Stop`, live play/write
//!     cursor tracking, linear-interpolated resampling `SetFrequency`, dB `SetVolume`,
//!     stereo `SetPan`, `DuplicateSoundBuffer` sharing sample memory)
//!   - `IDirectSound3DListener` & `IDirectSound3DBuffer` via `QueryInterface`
//!     (inverse-distance rolloff, `DSBCAPS_MUTE3DATMAXDISTANCE`, sound cones,
//!     left-handed stereo panning, Doppler shift, immediate & deferred settings)
//!   - `IDirectSoundNotify` via `QueryInterface` (offset & stop event signaling)
//!   - `IDirectSoundCapture` / `IDirectSoundCapture8` & `IDirectSoundCaptureBuffer8`
//!   - `IDirectSoundFullDuplex` & `DirectSoundFullDuplexCreate`
//!   - `IClassFactory` via `DllGetClassObject` (`CLSID_DirectSound*`,
//!     `CLSID_DirectSoundCapture*`, `CLSID_DirectSoundFullDuplex`)
//! * Background stereo `f32` software mixer writing into `OutputStream` with
//!   automatic `Backend::Null` fallback when headless (`PORT_HEADLESS=1` /
//!   `AW2_HEADLESS=1`) or `AW2_AUDIO_BACKEND=null`, plus deterministic
//!   `mix_dsound_frames` helper for unit tests.

mod buffer;
mod com_vtbl;
mod mixer;

use self::buffer::{
    device_guid_for, enumerate_devices_impl, resolve_capture_device, resolve_render_device,
    CLASS_E_CLASSNOTAVAILABLE, CLSID_DIRECTSOUND, CLSID_DIRECTSOUND8, CLSID_DIRECTSOUNDCAPTURE,
    CLSID_DIRECTSOUNDCAPTURE8, CLSID_DIRECTSOUNDFULLDUPLEX, DSDEVID_DEFAULTCAPTURE,
    DSDEVID_DEFAULTPLAYBACK, DSDEVID_DEFAULTVOICECAPTURE, DSDEVID_DEFAULTVOICEPLAYBACK,
    DSERR_INVALIDPARAM, DSERR_NOAGGREGATION, DSERR_NODRIVER, DS_OK, GUID_NULL,
};
use self::com_vtbl::{
    alloc_class_factory, alloc_ds_capture, alloc_ds_device, alloc_ds_full_duplex,
    cf_query_interface, cf_release, dsfd_initialize, dsfd_release, FactoryKind, ACTIVE_OBJECTS,
    SERVER_LOCKS,
};
use crate::audio::backend;
use crate::audio::com::GUID;
use crate::com::write_null_out;
use crate::strings::{rd, wr};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (1, "DirectSoundCreate"),
    (2, "DirectSoundEnumerateA"),
    (3, "DirectSoundEnumerateW"),
    (4, "DllCanUnloadNow"),
    (5, "DllGetClassObject"),
    (6, "DirectSoundCaptureCreate"),
    (7, "DirectSoundCaptureEnumerateA"),
    (8, "DirectSoundCaptureEnumerateW"),
    (9, "GetDeviceID"),
    (10, "DirectSoundFullDuplexCreate"),
    (11, "DirectSoundCreate8"),
    (12, "DirectSoundCaptureCreate8"),
];

// ============================================================================
// Exported DLL Entry Points (Ordinals 1..12)
// ============================================================================

pub unsafe extern "win64" fn DirectSoundCreate(
    guid: *const u8,
    pp_ds: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    write_null_out(pp_ds);
    if pp_ds.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_render_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_device(dev, true, false);
    wr(pp_ds, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundCreate8(
    guid: *const u8,
    pp_ds8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    write_null_out(pp_ds8);
    if pp_ds8.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_render_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_device(dev, true, true);
    wr(pp_ds8, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundEnumerateA(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, false, false)
}

pub unsafe extern "win64" fn DirectSoundEnumerateW(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, false, true)
}

pub unsafe extern "win64" fn DirectSoundCaptureCreate(
    guid: *const u8,
    pp_dsc: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    write_null_out(pp_dsc);
    if pp_dsc.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_capture_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_capture(dev, true);
    wr(pp_dsc, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundCaptureCreate8(
    guid: *const u8,
    pp_dsc8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    DirectSoundCaptureCreate(guid, pp_dsc8, unk)
}

pub unsafe extern "win64" fn DirectSoundCaptureEnumerateA(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, true, false)
}

pub unsafe extern "win64" fn DirectSoundCaptureEnumerateW(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, true, true)
}

pub unsafe extern "win64" fn DirectSoundFullDuplexCreate(
    capture_guid: *const u8,
    render_guid: *const u8,
    dsc_desc: *const c_void,
    ds_desc: *const c_void,
    hwnd: HWND,
    level: u32,
    pp_dsfd: *mut *mut c_void,
    pp_dscb8: *mut *mut c_void,
    pp_dsb8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    for p in [pp_dsfd, pp_dscb8, pp_dsb8] {
        write_null_out(p);
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if pp_dsfd.is_null() || pp_dscb8.is_null() || pp_dsb8.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let fd = alloc_ds_full_duplex();
    let hr = dsfd_initialize(
        fd,
        capture_guid as *const GUID,
        render_guid as *const GUID,
        dsc_desc,
        ds_desc,
        hwnd,
        level,
        pp_dscb8,
        pp_dsb8,
    );
    if hr != DS_OK {
        dsfd_release(fd);
        return hr;
    }
    wr(pp_dsfd, fd as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn GetDeviceID(src: *const u8, dst: *mut u8) -> HRESULT {
    if src.is_null() || dst.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let g = rd(src as *const GUID);
    if g == GUID_NULL {
        return DSERR_INVALIDPARAM;
    }
    if g == DSDEVID_DEFAULTPLAYBACK || g == DSDEVID_DEFAULTVOICEPLAYBACK {
        let devs = backend::output_devices();
        let dev = devs.first().cloned().unwrap_or_else(backend::null_device);
        wr(dst as *mut GUID, device_guid_for(&dev.uid, false));
        return DS_OK;
    }
    if g == DSDEVID_DEFAULTCAPTURE || g == DSDEVID_DEFAULTVOICECAPTURE {
        let devs = backend::input_devices();
        let dev = devs
            .first()
            .cloned()
            .unwrap_or_else(backend::null_capture_device);
        wr(dst as *mut GUID, device_guid_for(&dev.uid, true));
        return DS_OK;
    }
    for dev in backend::output_devices() {
        let dg = device_guid_for(&dev.uid, false);
        if dg == g {
            wr(dst as *mut GUID, dg);
            return DS_OK;
        }
    }
    for dev in backend::input_devices() {
        let dg = device_guid_for(&dev.uid, true);
        if dg == g {
            wr(dst as *mut GUID, dg);
            return DS_OK;
        }
    }
    DSERR_NODRIVER
}

pub unsafe extern "win64" fn DllGetClassObject(
    rclsid: *const u8,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(clsid) = GUID::read(rclsid as *const GUID) else {
        return E_POINTER;
    };
    let Some(iid) = GUID::read(riid as *const GUID) else {
        return E_POINTER;
    };
    let kind = if clsid == CLSID_DIRECTSOUND {
        FactoryKind::DirectSound
    } else if clsid == CLSID_DIRECTSOUND8 {
        FactoryKind::DirectSound8
    } else if clsid == CLSID_DIRECTSOUNDCAPTURE {
        FactoryKind::DirectSoundCapture
    } else if clsid == CLSID_DIRECTSOUNDCAPTURE8 {
        FactoryKind::DirectSoundCapture8
    } else if clsid == CLSID_DIRECTSOUNDFULLDUPLEX {
        FactoryKind::DirectSoundFullDuplex
    } else {
        return CLASS_E_CLASSNOTAVAILABLE;
    };

    let cf = alloc_class_factory(kind);
    let hr = cf_query_interface(cf, &iid, ppv);
    cf_release(cf);
    hr
}

pub unsafe extern "win64" fn DllCanUnloadNow() -> HRESULT {
    if ACTIVE_OBJECTS.load(Ordering::Acquire) == 0 && SERVER_LOCKS.load(Ordering::Acquire) == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

#[cfg(test)]
mod tests;
