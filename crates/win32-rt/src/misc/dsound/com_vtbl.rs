//! DirectSound 8 COM object layouts, vtable registration (`crate::com`), and method implementations.

use super::buffer::{
    parse_ds_format, resolve_capture_device, resolve_render_device, signal_events, write_ds_format,
    ActiveLock, Buffer3dParams, BufferEffect, CaptureBufferState, D3dVector, Ds3dBuffer,
    Ds3dListener, DsBufferDesc, DsBufferDesc1, DsCaps, DsEffectDesc, DsFxKind, DsbCaps,
    DsbPositionNotify, DscBufferDesc, DscBufferDesc1, DscCaps, DscbCaps, Listener3dParams,
    NotifyEntry, SampleMemory, SecondaryBufferState, DS3DMODE_DISABLE, DS3D_DEFERRED,
    DS3D_IMMEDIATE, DS3D_MAXCONEANGLE, DS3D_MAXDOPPLERFACTOR, DS3D_MAXROLLOFFFACTOR,
    DS3D_MINDOPPLERFACTOR, DS3D_MINROLLOFFFACTOR, DSBCAPS_CTRL3D, DSBCAPS_CTRLFREQUENCY,
    DSBCAPS_CTRLFX, DSBCAPS_CTRLPAN, DSBCAPS_CTRLPOSITIONNOTIFY, DSBCAPS_CTRLVOLUME,
    DSBCAPS_PRIMARYBUFFER, DSBFREQUENCY_MAX, DSBFREQUENCY_MIN, DSBFREQUENCY_ORIGINAL,
    DSBLOCK_ENTIREBUFFER, DSBLOCK_FROMWRITECURSOR, DSBPAN_LEFT, DSBPAN_RIGHT, DSBPLAY_LOOPING,
    DSBPN_OFFSETSTOP, DSBSIZE_MAX, DSBSIZE_MIN, DSBSTATUS_LOOPING, DSBSTATUS_PLAYING,
    DSBVOLUME_MAX, DSBVOLUME_MIN, DSCAPS_CERTIFIED, DSCAPS_CONTINUOUSRATE, DSCAPS_PRIMARY16BIT,
    DSCAPS_PRIMARY8BIT, DSCAPS_PRIMARYMONO, DSCAPS_PRIMARYSTEREO, DSCAPS_SECONDARY16BIT,
    DSCAPS_SECONDARY8BIT, DSCAPS_SECONDARYMONO, DSCAPS_SECONDARYSTEREO, DSCBLOCK_ENTIREBUFFER,
    DSCBSTART_LOOPING, DSCBSTATUS_CAPTURING, DSCBSTATUS_LOOPING, DSCCAPS_CERTIFIED,
    DSCCAPS_MULTIPLECAPTURE, DSERR_ALREADYINITIALIZED, DSERR_CONTROLUNAVAIL, DSERR_FXUNAVAILABLE,
    DSERR_INVALIDCALL, DSERR_INVALIDPARAM, DSERR_NOAGGREGATION, DSERR_OBJECTNOTFOUND,
    DSERR_PRIOLEVELNEEDED, DSERR_UNINITIALIZED, DSFXR_FAILED, DSFXR_LOCSOFTWARE, DSFXR_UNKNOWN,
    DSFX_LOCHARDWARE, DSSCL_NORMAL, DSSCL_PRIORITY, DSSCL_WRITEPRIMARY, DS_CERTIFIED, DS_OK,
    GUID_NULL, IID_ICLASSFACTORY, IID_IDIRECTSOUND, IID_IDIRECTSOUND3DBUFFER,
    IID_IDIRECTSOUND3DLISTENER, IID_IDIRECTSOUND8, IID_IDIRECTSOUNDBUFFER,
    IID_IDIRECTSOUNDBUFFER8, IID_IDIRECTSOUNDCAPTURE, IID_IDIRECTSOUNDCAPTURE8,
    IID_IDIRECTSOUNDCAPTUREBUFFER, IID_IDIRECTSOUNDCAPTUREBUFFER8, IID_IDIRECTSOUNDFULLDUPLEX,
    IID_IDIRECTSOUNDNOTIFY, STANDARD_CAPTURE_FORMATS,
};
use super::mixer::{DsCaptureCore, DsDeviceCore};
use crate::audio::backend::{self, DeviceInfo};
use crate::audio::com::{GUID, IID_IUnknown};
use crate::audio::format::{vec3_cross, vec3_is_finite, vec3_normalize, Vec3};
use crate::com::{register_static_ptr_slice, write_null_out};
use crate::strings::{rd, wr};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::time::Instant;

// Global DLL lifetime tracking for DllCanUnloadNow
pub(super) static ACTIVE_OBJECTS: AtomicU32 = AtomicU32::new(0);
pub(super) static SERVER_LOCKS: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// COM Object Layouts & Vtables
// ============================================================================

#[repr(C, align(16))]
struct Vtable<const N: usize>([crate::Ptr; N]);
unsafe impl<const N: usize> Sync for Vtable<N> {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FactoryKind {
    DirectSound,
    DirectSound8,
    DirectSoundCapture,
    DirectSoundCapture8,
    DirectSoundFullDuplex,
}

#[repr(C)]
pub(super) struct ClassFactoryCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    kind: FactoryKind,
}

#[repr(C)]
pub(super) struct DsDeviceCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    pub(super) core: Arc<DsDeviceCore>,
}

/// Multi-interface COM object for `IDirectSoundBuffer8` + (`IDirectSound3DBuffer`
/// or `IDirectSound3DListener`) + `IDirectSoundNotify`.
#[repr(C)]
pub(super) struct DsBufferCom {
    vtbl_buf: *const crate::Ptr,
    vtbl_3d: *const crate::Ptr,
    vtbl_notify: *const crate::Ptr,
    refs: AtomicU32,
    device: Arc<DsDeviceCore>,
    is_primary: bool,
    flags: u32,
    buffer_id: u64,
}

#[repr(C)]
pub(super) struct DsCaptureCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    core: Arc<DsCaptureCore>,
}

/// Multi-interface COM object for `IDirectSoundCaptureBuffer8` + `IDirectSoundNotify`.
#[repr(C)]
pub(super) struct DsCaptureBufferCom {
    vtbl_cap_buf: *const crate::Ptr,
    vtbl_notify: *const crate::Ptr,
    refs: AtomicU32,
    #[allow(dead_code)]
    capture: Arc<DsCaptureCore>,
    state: Mutex<CaptureBufferState>,
}

#[repr(C)]
pub(super) struct DsFullDuplexCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    initialized: AtomicBool,
    capture: Mutex<Option<Arc<DsCaptureCore>>>,
    render: Mutex<Option<Arc<DsDeviceCore>>>,
}

// --- Vtable instances ---

static CLASS_FACTORY_VTBL: Vtable<5> = Vtable([
    crate::Ptr(cf_query_interface as *const ()), // 0
    crate::Ptr(cf_add_ref as *const ()),         // 1
    crate::Ptr(cf_release as *const ()),         // 2
    crate::Ptr(cf_create_instance as *const ()), // 3
    crate::Ptr(cf_lock_server as *const ()),     // 4
]);

static DSOUND8_VTBL: Vtable<12> = Vtable([
    crate::Ptr(ds_query_interface as *const ()),        // 0
    crate::Ptr(ds_add_ref as *const ()),                // 1
    crate::Ptr(ds_release as *const ()),                // 2
    crate::Ptr(ds_create_sound_buffer as *const ()),    // 3
    crate::Ptr(ds_get_caps as *const ()),               // 4
    crate::Ptr(ds_duplicate_sound_buffer as *const ()), // 5
    crate::Ptr(ds_set_cooperative_level as *const ()),  // 6
    crate::Ptr(ds_compact as *const ()),                // 7
    crate::Ptr(ds_get_speaker_config as *const ()),     // 8
    crate::Ptr(ds_set_speaker_config as *const ()),     // 9
    crate::Ptr(ds_initialize as *const ()),             // 10
    crate::Ptr(ds_verify_certification as *const ()),   // 11 (IDirectSound8)
]);

static DSBUFFER8_VTBL: Vtable<24> = Vtable([
    crate::Ptr(dsb_query_interface as *const ()),      // 0
    crate::Ptr(dsb_add_ref as *const ()),              // 1
    crate::Ptr(dsb_release as *const ()),              // 2
    crate::Ptr(dsb_get_caps as *const ()),             // 3
    crate::Ptr(dsb_get_current_position as *const ()), // 4
    crate::Ptr(dsb_get_format as *const ()),           // 5
    crate::Ptr(dsb_get_volume as *const ()),           // 6
    crate::Ptr(dsb_get_pan as *const ()),              // 7
    crate::Ptr(dsb_get_frequency as *const ()),        // 8
    crate::Ptr(dsb_get_status as *const ()),           // 9
    crate::Ptr(dsb_initialize as *const ()),           // 10
    crate::Ptr(dsb_lock as *const ()),                 // 11
    crate::Ptr(dsb_play as *const ()),                 // 12
    crate::Ptr(dsb_set_current_position as *const ()), // 13
    crate::Ptr(dsb_set_format as *const ()),           // 14
    crate::Ptr(dsb_set_volume as *const ()),           // 15
    crate::Ptr(dsb_set_pan as *const ()),              // 16
    crate::Ptr(dsb_set_frequency as *const ()),        // 17
    crate::Ptr(dsb_stop as *const ()),                 // 18
    crate::Ptr(dsb_unlock as *const ()),               // 19
    crate::Ptr(dsb_restore as *const ()),              // 20
    crate::Ptr(dsb_set_fx as *const ()),               // 21 (IDirectSoundBuffer8)
    crate::Ptr(dsb_acquire_resources as *const ()),    // 22 (IDirectSoundBuffer8)
    crate::Ptr(dsb_get_object_in_path as *const ()),   // 23 (IDirectSoundBuffer8)
]);

static DS3DBUFFER_VTBL: Vtable<21> = Vtable([
    crate::Ptr(ds3db_query_interface as *const ()),               // 0
    crate::Ptr(ds3db_add_ref as *const ()),                       // 1
    crate::Ptr(ds3db_release as *const ()),                       // 2
    crate::Ptr(ds3db_get_all_parameters as *const ()),            // 3
    crate::Ptr(ds3db_get_cone_angles as *const ()),               // 4
    crate::Ptr(ds3db_get_cone_orientation as *const ()),          // 5
    crate::Ptr(ds3db_get_cone_outside_volume as *const ()),       // 6
    crate::Ptr(ds3db_get_max_distance as *const ()),              // 7
    crate::Ptr(ds3db_get_min_distance as *const ()),              // 8
    crate::Ptr(ds3db_get_mode as *const ()),                      // 9
    crate::Ptr(ds3db_get_position as *const ()),                  // 10
    crate::Ptr(ds3db_get_velocity as *const ()),                  // 11
    crate::Ptr(ds3db_set_all_parameters as *const ()),            // 12
    crate::Ptr(ds3db_set_cone_angles as *const ()),               // 13
    crate::Ptr(crate::bridge_entry!(ds3db_set_cone_orientation)), // 14
    crate::Ptr(ds3db_set_cone_outside_volume as *const ()),       // 15
    crate::Ptr(crate::bridge_entry!(ds3db_set_max_distance)),     // 16
    crate::Ptr(crate::bridge_entry!(ds3db_set_min_distance)),     // 17
    crate::Ptr(ds3db_set_mode as *const ()),                      // 18
    crate::Ptr(crate::bridge_entry!(ds3db_set_position)),         // 19
    crate::Ptr(crate::bridge_entry!(ds3db_set_velocity)),         // 20
]);

static DS3DLISTENER_VTBL: Vtable<18> = Vtable([
    crate::Ptr(ds3dl_query_interface as *const ()),              // 0
    crate::Ptr(ds3dl_add_ref as *const ()),                      // 1
    crate::Ptr(ds3dl_release as *const ()),                      // 2
    crate::Ptr(ds3dl_get_all_parameters as *const ()),           // 3
    crate::Ptr(ds3dl_get_distance_factor as *const ()),          // 4
    crate::Ptr(ds3dl_get_doppler_factor as *const ()),           // 5
    crate::Ptr(ds3dl_get_orientation as *const ()),              // 6
    crate::Ptr(ds3dl_get_position as *const ()),                 // 7
    crate::Ptr(ds3dl_get_rolloff_factor as *const ()),           // 8
    crate::Ptr(ds3dl_get_velocity as *const ()),                 // 9
    crate::Ptr(ds3dl_set_all_parameters as *const ()),           // 10
    crate::Ptr(crate::bridge_entry!(ds3dl_set_distance_factor)), // 11
    crate::Ptr(crate::bridge_entry!(ds3dl_set_doppler_factor)),  // 12
    crate::Ptr(crate::bridge_entry!(ds3dl_set_orientation)),     // 13
    crate::Ptr(crate::bridge_entry!(ds3dl_set_position)),        // 14
    crate::Ptr(crate::bridge_entry!(ds3dl_set_rolloff_factor)),  // 15
    crate::Ptr(crate::bridge_entry!(ds3dl_set_velocity)),        // 16
    crate::Ptr(ds3dl_commit_deferred_settings as *const ()),     // 17
]);

static DSNOTIFY_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dsn_query_interface as *const ()),            // 0
    crate::Ptr(dsn_add_ref as *const ()),                    // 1
    crate::Ptr(dsn_release as *const ()),                    // 2
    crate::Ptr(dsn_set_notification_positions as *const ()), // 3
]);

static DSCAPTURE8_VTBL: Vtable<6> = Vtable([
    crate::Ptr(dsc_query_interface as *const ()),       // 0
    crate::Ptr(dsc_add_ref as *const ()),               // 1
    crate::Ptr(dsc_release as *const ()),               // 2
    crate::Ptr(dsc_create_capture_buffer as *const ()), // 3
    crate::Ptr(dsc_get_caps as *const ()),              // 4
    crate::Ptr(dsc_initialize as *const ()),            // 5
]);

static DSCAPTUREBUFFER8_VTBL: Vtable<14> = Vtable([
    crate::Ptr(dscb_query_interface as *const ()),      // 0
    crate::Ptr(dscb_add_ref as *const ()),              // 1
    crate::Ptr(dscb_release as *const ()),              // 2
    crate::Ptr(dscb_get_caps as *const ()),             // 3
    crate::Ptr(dscb_get_current_position as *const ()), // 4
    crate::Ptr(dscb_get_format as *const ()),           // 5
    crate::Ptr(dscb_get_status as *const ()),           // 6
    crate::Ptr(dscb_initialize as *const ()),           // 7
    crate::Ptr(dscb_lock as *const ()),                 // 8
    crate::Ptr(dscb_start as *const ()),                // 9
    crate::Ptr(dscb_stop as *const ()),                 // 10
    crate::Ptr(dscb_unlock as *const ()),               // 11
    crate::Ptr(dscb_get_object_in_path as *const ()),   // 12 (IDirectSoundCaptureBuffer8)
    crate::Ptr(dscb_get_fx_status as *const ()),        // 13 (IDirectSoundCaptureBuffer8)
]);

static DSCAPTURE_NOTIFY_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dscn_query_interface as *const ()),            // 0
    crate::Ptr(dscn_add_ref as *const ()),                    // 1
    crate::Ptr(dscn_release as *const ()),                    // 2
    crate::Ptr(dscn_set_notification_positions as *const ()), // 3
]);

static DSFULLDUPLEX_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dsfd_query_interface as *const ()), // 0
    crate::Ptr(dsfd_add_ref as *const ()),         // 1
    crate::Ptr(dsfd_release as *const ()),         // 2
    crate::Ptr(dsfd_initialize as *const ()),      // 3
]);

fn ensure_vtbls_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for tbl in [
            &CLASS_FACTORY_VTBL.0[..],
            &DSOUND8_VTBL.0[..],
            &DSBUFFER8_VTBL.0[..],
            &DS3DBUFFER_VTBL.0[..],
            &DS3DLISTENER_VTBL.0[..],
            &DSNOTIFY_VTBL.0[..],
            &DSCAPTURE8_VTBL.0[..],
            &DSCAPTUREBUFFER8_VTBL.0[..],
            &DSCAPTURE_NOTIFY_VTBL.0[..],
            &DSFULLDUPLEX_VTBL.0[..],
        ] {
            register_static_ptr_slice(tbl);
        }
    });
}

// ============================================================================
// COM Object Allocation Helpers
// ============================================================================

pub(super) fn alloc_ds_device(
    device: DeviceInfo,
    initialized: bool,
    is_ds8: bool,
) -> *mut DsDeviceCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsDeviceCom {
        vtbl: DSOUND8_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        core: DsDeviceCore::new(device, initialized, is_ds8),
    }))
}

fn alloc_ds_buffer(
    device: Arc<DsDeviceCore>,
    is_primary: bool,
    flags: u32,
    buffer_id: u64,
) -> *mut DsBufferCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    let vtbl_3d = if is_primary {
        DS3DLISTENER_VTBL.0.as_ptr()
    } else {
        DS3DBUFFER_VTBL.0.as_ptr()
    };
    Box::into_raw(Box::new(DsBufferCom {
        vtbl_buf: DSBUFFER8_VTBL.0.as_ptr(),
        vtbl_3d,
        vtbl_notify: DSNOTIFY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        device,
        is_primary,
        flags,
        buffer_id,
    }))
}

pub(super) fn alloc_ds_capture(device: DeviceInfo, initialized: bool) -> *mut DsCaptureCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsCaptureCom {
        vtbl: DSCAPTURE8_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        core: DsCaptureCore::new(device, initialized),
    }))
}

fn alloc_ds_capture_buffer(
    capture: Arc<DsCaptureCore>,
    flags: u32,
    sample_mem: Arc<SampleMemory>,
) -> *mut DsCaptureBufferCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsCaptureBufferCom {
        vtbl_cap_buf: DSCAPTUREBUFFER8_VTBL.0.as_ptr(),
        vtbl_notify: DSCAPTURE_NOTIFY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        capture,
        state: Mutex::new(CaptureBufferState::new(flags, sample_mem)),
    }))
}

pub(super) fn alloc_ds_full_duplex() -> *mut DsFullDuplexCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsFullDuplexCom {
        vtbl: DSFULLDUPLEX_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        initialized: AtomicBool::new(false),
        capture: Mutex::new(None),
        render: Mutex::new(None),
    }))
}

pub(super) fn alloc_class_factory(kind: FactoryKind) -> *mut ClassFactoryCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(ClassFactoryCom {
        vtbl: CLASS_FACTORY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        kind,
    }))
}

// ============================================================================
// IClassFactory Methods
// ============================================================================

pub(super) unsafe extern "win64" fn cf_query_interface(
    this: *mut ClassFactoryCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_ICLASSFACTORY {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn cf_add_ref(this: *mut ClassFactoryCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn cf_release(this: *mut ClassFactoryCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

pub(super) unsafe extern "win64" fn cf_create_instance(
    this: *mut ClassFactoryCom,
    p_unk_outer: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let kind = {
        let obj = &*this;
        obj.kind
    };
    match kind {
        FactoryKind::DirectSound | FactoryKind::DirectSound8 => {
            let is_ds8 = kind == FactoryKind::DirectSound8;
            let raw = alloc_ds_device(backend::null_device(), false, is_ds8);
            let hr = ds_query_interface(raw, &iid, ppv);
            ds_release(raw);
            hr
        }
        FactoryKind::DirectSoundCapture | FactoryKind::DirectSoundCapture8 => {
            let raw = alloc_ds_capture(backend::null_capture_device(), false);
            let hr = dsc_query_interface(raw, &iid, ppv);
            dsc_release(raw);
            hr
        }
        FactoryKind::DirectSoundFullDuplex => {
            let raw = alloc_ds_full_duplex();
            let hr = dsfd_query_interface(raw, &iid, ppv);
            dsfd_release(raw);
            hr
        }
    }
}

pub(super) unsafe extern "win64" fn cf_lock_server(
    _this: *mut ClassFactoryCom,
    f_lock: BOOL,
) -> HRESULT {
    if f_lock != 0 {
        SERVER_LOCKS.fetch_add(1, Ordering::AcqRel);
    } else {
        let _ = SERVER_LOCKS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
            Some(n.saturating_sub(1))
        });
    }
    DS_OK
}

// ============================================================================
// IDirectSound / IDirectSound8 Methods
// ============================================================================

pub(super) unsafe extern "win64" fn ds_query_interface(
    this: *mut DsDeviceCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let obj = &*this;
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUND
        || (obj.core.is_ds8 && iid == IID_IDIRECTSOUND8)
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn ds_add_ref(this: *mut DsDeviceCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn ds_release(this: *mut DsDeviceCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe fn create_sound_buffer_core(
    core: &Arc<DsDeviceCore>,
    desc_ptr: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if pp_buf.is_null() {
        return DSERR_INVALIDPARAM;
    }
    write_null_out(pp_buf);
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if desc_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }

    let dw_size = rd(desc_ptr as *const u32);
    let (flags, buffer_bytes, reserved, wfx_ptr) = if dw_size == 24 {
        let d = rd(desc_ptr as *const DsBufferDesc1);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else if dw_size == 40 {
        let d = rd(desc_ptr as *const DsBufferDesc);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else {
        return DSERR_INVALIDPARAM;
    };

    if reserved != 0 {
        return DSERR_INVALIDPARAM;
    }

    let is_primary = (flags & DSBCAPS_PRIMARYBUFFER) != 0;
    if is_primary {
        if buffer_bytes != 0 || !wfx_ptr.is_null() {
            return DSERR_INVALIDPARAM;
        }
        let buf = alloc_ds_buffer(core.clone(), true, flags, 0);
        wr(pp_buf, buf as *mut c_void);
        return DS_OK;
    }

    if !(DSBSIZE_MIN..=DSBSIZE_MAX).contains(&buffer_bytes) {
        return DSERR_INVALIDPARAM;
    }
    let fmt = match parse_ds_format(wfx_ptr) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    if (buffer_bytes as usize) < (fmt.block_align as usize) {
        return DSERR_INVALIDPARAM;
    }

    let sample_mem = SampleMemory::new(buffer_bytes as usize, fmt);
    let buf_id = core.alloc_buf_id();
    {
        let mut st = core.state.lock().unwrap();
        st.buffers
            .insert(buf_id, SecondaryBufferState::new(flags, sample_mem));
    }
    let buf = alloc_ds_buffer(core.clone(), false, flags, buf_id);
    wr(pp_buf, buf as *mut c_void);
    DS_OK
}

pub(super) unsafe extern "win64" fn ds_create_sound_buffer(
    this: *mut DsDeviceCom,
    desc: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        write_null_out(pp_buf);
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        obj.core.clone()
    };
    create_sound_buffer_core(&core, desc, pp_buf, p_unk_outer)
}

pub(super) unsafe extern "win64" fn ds_get_caps(
    this: *mut DsDeviceCom,
    caps: *mut DsCaps,
) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let sz = rd(caps as *const u32);
    if sz != std::mem::size_of::<DsCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let out = DsCaps {
        dw_size: sz,
        dw_flags: DSCAPS_PRIMARYMONO
            | DSCAPS_PRIMARYSTEREO
            | DSCAPS_PRIMARY8BIT
            | DSCAPS_PRIMARY16BIT
            | DSCAPS_CONTINUOUSRATE
            | DSCAPS_CERTIFIED
            | DSCAPS_SECONDARYMONO
            | DSCAPS_SECONDARYSTEREO
            | DSCAPS_SECONDARY8BIT
            | DSCAPS_SECONDARY16BIT,
        dw_min_secondary_sample_rate: DSBFREQUENCY_MIN,
        dw_max_secondary_sample_rate: DSBFREQUENCY_MAX,
        dw_primary_buffers: 1,
        dw_play_cpu_overhead_sw_buffers: 1,
        ..DsCaps::default()
    };
    wr(caps, out);
    DS_OK
}

pub(super) unsafe extern "win64" fn ds_duplicate_sound_buffer(
    this: *mut DsDeviceCom,
    orig_ptr: *mut c_void,
    pp_dup: *mut *mut c_void,
) -> HRESULT {
    if pp_dup.is_null() {
        return DSERR_INVALIDPARAM;
    }
    write_null_out(pp_dup);
    if this.is_null() || orig_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        if !obj.core.initialized.load(Ordering::Acquire) {
            return DSERR_UNINITIALIZED;
        }
        obj.core.clone()
    };

    // Validate that `orig_ptr` is one of our DirectSound buffers belonging to this device.
    let orig_vtbl = rd(orig_ptr as *const *const crate::Ptr);
    if orig_vtbl != DSBUFFER8_VTBL.0.as_ptr() {
        return DSERR_INVALIDPARAM;
    }
    let orig = &*(orig_ptr as *const DsBufferCom);
    if !Arc::ptr_eq(&orig.device, &core) {
        return DSERR_INVALIDPARAM;
    }
    if orig.is_primary {
        return DSERR_INVALIDCALL;
    }

    let dup_id = core.alloc_buf_id();
    let flags = {
        let mut st = core.state.lock().unwrap();
        let Some(src) = st.buffers.get(&orig.buffer_id) else {
            return DSERR_INVALIDPARAM;
        };
        let mut dup_state = SecondaryBufferState::new(src.flags, src.sample_mem.clone());
        dup_state.frequency = src.frequency;
        dup_state.volume_mb = src.volume_mb;
        dup_state.pan_mb = src.pan_mb;
        dup_state.d3d_current = src.d3d_current;
        dup_state.d3d_deferred = src.d3d_deferred;
        let f = dup_state.flags;
        st.buffers.insert(dup_id, dup_state);
        f
    };

    let dup_buf = alloc_ds_buffer(core, false, flags, dup_id);
    wr(pp_dup, dup_buf as *mut c_void);
    DS_OK
}

pub(super) unsafe extern "win64" fn ds_set_cooperative_level(
    this: *mut DsDeviceCom,
    _hwnd: HWND,
    dw_level: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&dw_level) {
        return DSERR_INVALIDPARAM;
    }
    obj.core.state.lock().unwrap().cooperative_level = dw_level;
    DS_OK
}

unsafe extern "win64" fn ds_compact(this: *mut DsDeviceCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let coop = obj.core.state.lock().unwrap().cooperative_level;
    if coop < DSSCL_PRIORITY {
        return DSERR_PRIOLEVELNEEDED;
    }
    DS_OK
}

unsafe extern "win64" fn ds_get_speaker_config(
    this: *mut DsDeviceCom,
    out: *mut u32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let cfg = obj.core.state.lock().unwrap().speaker_config;
    wr(out, cfg);
    DS_OK
}

unsafe extern "win64" fn ds_set_speaker_config(this: *mut DsDeviceCom, cfg: u32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    obj.core.state.lock().unwrap().speaker_config = cfg;
    DS_OK
}

pub(super) unsafe extern "win64" fn ds_initialize(
    this: *mut DsDeviceCom,
    guid_ptr: *const GUID,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }
    let guid_opt = GUID::read(guid_ptr);
    let dev = match resolve_render_device(guid_opt) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    {
        let mut st = obj.core.state.lock().unwrap();
        if (DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&dev.rate) {
            st.primary_format.rate = dev.rate;
        }
        st.device = dev;
    }
    obj.core.initialized.store(true, Ordering::Release);
    DS_OK
}

pub(super) unsafe extern "win64" fn ds_verify_certification(
    this: *mut DsDeviceCom,
    certified: *mut u32,
) -> HRESULT {
    if this.is_null() || certified.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    wr(certified, DS_CERTIFIED);
    DS_OK
}

// ============================================================================
// IDirectSoundBuffer / IDirectSoundBuffer8 Methods + Sub-Interface Dispatch
// ============================================================================

unsafe fn dsbuf_qi_common(
    base: *mut DsBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if base.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let obj = &*base;
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUNDBUFFER
        || (!obj.is_primary && obj.device.is_ds8 && iid == IID_IDIRECTSOUNDBUFFER8)
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, base as *mut c_void);
        return DS_OK;
    }
    if (obj.flags & DSBCAPS_CTRL3D) != 0
        && ((obj.is_primary && iid == IID_IDIRECTSOUND3DLISTENER)
            || (!obj.is_primary && iid == IID_IDIRECTSOUND3DBUFFER))
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsBufferCom, vtbl_3d));
        wr(ppv, sub as *mut c_void);
        return DS_OK;
    }
    if (obj.flags & DSBCAPS_CTRLPOSITIONNOTIFY) != 0 && iid == IID_IDIRECTSOUNDNOTIFY {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsBufferCom, vtbl_notify));
        wr(ppv, sub as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe fn dsbuf_add_ref_common(base: *mut DsBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn dsbuf_release_common(base: *mut DsBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*base;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        let boxed = Box::from_raw(base);
        if !boxed.is_primary {
            boxed
                .device
                .state
                .lock()
                .unwrap()
                .buffers
                .remove(&boxed.buffer_id);
        }
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(boxed);
        0
    } else {
        prev - 1
    }
}

#[inline]
unsafe fn dsbuf_from_3d(this: *mut c_void) -> *mut DsBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsBufferCom, vtbl_3d)) as *mut DsBufferCom
    }
}

#[inline]
unsafe fn dsbuf_from_notify(this: *mut c_void) -> *mut DsBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsBufferCom, vtbl_notify)) as *mut DsBufferCom
    }
}

pub(super) unsafe extern "win64" fn dsb_query_interface(
    this: *mut DsBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn dsb_add_ref(this: *mut DsBufferCom) -> u32 {
    dsbuf_add_ref_common(this)
}

pub(super) unsafe extern "win64" fn dsb_release(this: *mut DsBufferCom) -> u32 {
    dsbuf_release_common(this)
}

unsafe extern "win64" fn dsb_get_caps(this: *mut DsBufferCom, caps: *mut DsbCaps) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = rd(caps as *const u32);
    if sz != std::mem::size_of::<DsbCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let byte_len = if obj.is_primary {
        // Primary buffers report a nominal buffer size (40 ms of primary mix format).
        (st.primary_format.rate / 25) * (st.primary_format.block_align as u32)
    } else {
        match st.buffers.get(&obj.buffer_id) {
            Some(b) => b.sample_mem.byte_len() as u32,
            None => return DSERR_INVALIDCALL,
        }
    };
    wr(
        caps,
        DsbCaps {
            dw_size: sz,
            dw_flags: obj.flags,
            dw_buffer_bytes: byte_len,
            dw_unlock_transfer_rate: 0,
            dw_play_cpu_overhead: 0,
        },
    );
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_current_position(
    this: *mut DsBufferCom,
    play_out: *mut u32,
    write_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let (play_pos, write_pos) = if obj.is_primary {
        let prim_frames = (st.primary_format.rate / 25).max(1) as u64;
        let block = st.primary_format.block_align as u32;
        if st.primary_playing {
            let pf = (st.primary_frames_mixed % prim_frames) as u32;
            let lead =
                ((st.primary_format.rate / 100).max(1)).min((prim_frames as u32 / 2).max(1));
            let wf = ((pf + lead) % prim_frames as u32) * block;
            (pf * block, wf)
        } else {
            let pf = (st.primary_frames_mixed % prim_frames) as u32 * block;
            (pf, pf)
        }
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.current_cursors()
    };
    if !play_out.is_null() {
        wr(play_out, play_pos);
    }
    if !write_out.is_null() {
        wr(write_out, write_pos);
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_format(
    this: *mut DsBufferCom,
    wfx_out: *mut u8,
    allocated: u32,
    written_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let fmt = {
        let st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            st.primary_format
        } else {
            let Some(b) = st.buffers.get(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            b.sample_mem.format
        }
    };
    write_ds_format(&fmt, wfx_out, allocated, written_out)
}

pub(super) unsafe extern "win64" fn dsb_get_volume(
    this: *mut DsBufferCom,
    out: *mut i32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let vol = if obj.is_primary {
        st.primary_volume_mb
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.volume_mb
    };
    wr(out, vol);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_pan(
    this: *mut DsBufferCom,
    out: *mut i32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let pan = if obj.is_primary {
        st.primary_pan_mb
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.pan_mb
    };
    wr(out, pan);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_frequency(
    this: *mut DsBufferCom,
    out: *mut u32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let freq = if obj.is_primary {
        st.primary_format.rate
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.frequency
    };
    wr(out, freq);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_status(
    this: *mut DsBufferCom,
    out: *mut u32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let status = if obj.is_primary {
        if st.primary_playing {
            DSBSTATUS_PLAYING | DSBSTATUS_LOOPING
        } else {
            0
        }
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        let mut s = 0u32;
        if b.playing {
            s |= DSBSTATUS_PLAYING;
            if b.looping {
                s |= DSBSTATUS_LOOPING;
            }
        }
        s
    };
    wr(out, status);
    DS_OK
}

unsafe extern "win64" fn dsb_initialize(
    _this: *mut DsBufferCom,
    _ds: *mut c_void,
    _desc: *const c_void,
) -> HRESULT {
    DSERR_ALREADYINITIALIZED
}

pub(super) unsafe extern "win64" fn dsb_lock(
    this: *mut DsBufferCom,
    dw_offset: u32,
    dw_bytes: u32,
    ppv_ptr1: *mut *mut c_void,
    pdw_bytes1: *mut u32,
    ppv_ptr2: *mut *mut c_void,
    pdw_bytes2: *mut u32,
    dw_flags: u32,
) -> HRESULT {
    write_null_out(ppv_ptr1);
    if !pdw_bytes1.is_null() {
        wr(pdw_bytes1, 0);
    }
    write_null_out(ppv_ptr2);
    if !pdw_bytes2.is_null() {
        wr(pdw_bytes2, 0);
    }
    if this.is_null() || ppv_ptr1.is_null() || pdw_bytes1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if ppv_ptr2.is_null() != pdw_bytes2.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }

    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let buf_len = b.sample_mem.byte_len() as u32;
    let offset = if (dw_flags & DSBLOCK_FROMWRITECURSOR) != 0 {
        b.current_cursors().1
    } else {
        dw_offset
    };
    let bytes = if (dw_flags & DSBLOCK_ENTIREBUFFER) != 0 {
        buf_len
    } else {
        dw_bytes
    };

    if offset >= buf_len || bytes == 0 || bytes > buf_len {
        return DSERR_INVALIDPARAM;
    }

    let base = b.sample_mem.base_mut_ptr();
    let first_len = bytes.min(buf_len - offset);
    let second_len = bytes - first_len;
    let ptr1 = base.add(offset as usize) as *mut c_void;
    let (ptr2, actual_len2) = if second_len > 0 && !ppv_ptr2.is_null() {
        (base as *mut c_void, second_len)
    } else {
        (std::ptr::null_mut(), 0)
    };

    b.locks.push(ActiveLock {
        ptr1: ptr1 as usize,
        len1: first_len,
        ptr2: ptr2 as usize,
        len2: actual_len2,
    });

    wr(ppv_ptr1, ptr1);
    wr(pdw_bytes1, first_len);
    if !ppv_ptr2.is_null() {
        wr(ppv_ptr2, ptr2);
        wr(pdw_bytes2, actual_len2);
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_play(
    this: *mut DsBufferCom,
    dw_reserved1: u32,
    _dw_priority: u32,
    dw_flags: u32,
) -> HRESULT {
    if this.is_null() || dw_reserved1 != 0 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let looping = (dw_flags & DSBPLAY_LOOPING) != 0;
    {
        let mut st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            if !looping {
                return DSERR_INVALIDPARAM;
            }
            st.primary_playing = true;
        } else {
            let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            b.playing = true;
            b.looping = looping;
        }
    }
    obj.device.ensure_mixer_started();
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_set_current_position(
    this: *mut DsBufferCom,
    dw_new_position: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let buf_len = b.sample_mem.byte_len() as u32;
    if dw_new_position >= buf_len {
        return DSERR_INVALIDPARAM;
    }
    let block = (b.sample_mem.format.block_align as u32).max(1);
    b.play_pos_frames = (dw_new_position / block) as f64;
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_set_format(
    this: *mut DsBufferCom,
    wfx: *const u8,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let fmt = match parse_ds_format(wfx) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    {
        let mut st = obj.device.state.lock().unwrap();
        if st.cooperative_level < DSSCL_PRIORITY {
            return DSERR_PRIOLEVELNEEDED;
        }
        st.primary_format = fmt;
    }
    obj.device.restart_mixer_if_running(fmt.rate);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_set_volume(
    this: *mut DsBufferCom,
    l_volume: i32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&l_volume) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if (obj.flags & DSBCAPS_CTRLVOLUME) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    let mut st = obj.device.state.lock().unwrap();
    if obj.is_primary {
        st.primary_volume_mb = l_volume;
    } else {
        let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.volume_mb = l_volume;
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_set_pan(
    this: *mut DsBufferCom,
    l_pan: i32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !(DSBPAN_LEFT..=DSBPAN_RIGHT).contains(&l_pan) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if (obj.flags & DSBCAPS_CTRLPAN) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    let mut st = obj.device.state.lock().unwrap();
    if obj.is_primary {
        st.primary_pan_mb = l_pan;
    } else {
        let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.pan_mb = l_pan;
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_set_frequency(
    this: *mut DsBufferCom,
    dw_frequency: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_CONTROLUNAVAIL;
    }
    if (obj.flags & DSBCAPS_CTRLFREQUENCY) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    if dw_frequency != DSBFREQUENCY_ORIGINAL
        && !(DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&dw_frequency)
    {
        return DSERR_INVALIDPARAM;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    b.frequency = if dw_frequency == DSBFREQUENCY_ORIGINAL {
        b.sample_mem.format.rate
    } else {
        dw_frequency
    };
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_stop(this: *mut DsBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut events = Vec::new();
    {
        let mut st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            st.primary_playing = false;
        } else {
            let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            let was_playing = b.playing;
            b.playing = false;
            b.looping = false;
            if was_playing {
                b.collect_stop_events(&mut events);
            }
        }
    }
    signal_events(events);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_unlock(
    this: *mut DsBufferCom,
    pv_ptr1: *mut c_void,
    dw_bytes1: u32,
    pv_ptr2: *mut c_void,
    dw_bytes2: u32,
) -> HRESULT {
    if this.is_null() || pv_ptr1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let pos = b.locks.iter().position(|lk| {
        lk.ptr1 == (pv_ptr1 as usize)
            && dw_bytes1 <= lk.len1
            && (dw_bytes2 == 0 || (lk.ptr2 == (pv_ptr2 as usize) && dw_bytes2 <= lk.len2))
    });
    let Some(idx) = pos else {
        return DSERR_INVALIDPARAM;
    };
    b.locks.swap_remove(idx);
    std::sync::atomic::fence(Ordering::Release);
    DS_OK
}

unsafe extern "win64" fn dsb_restore(this: *mut DsBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DS_OK
}

// ---------------------------------------------------------------------------
// DirectSound 8 Effect Parameter COM Object (`IDirectSoundFX*`)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct DsFxCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    effect: Arc<Mutex<BufferEffect>>,
}

unsafe extern "win64" fn dsfx_qi(
    this: *mut DsFxCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    write_null_out(ppv);
    if this.is_null() || riid.is_null() || ppv.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let iid = rd(riid);
    let obj = &*this;
    let param_iid = obj.effect.lock().unwrap().kind.param_iid();
    if iid == IID_IUnknown || iid == param_iid {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        DS_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn dsfx_add_ref(this: *mut DsFxCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dsfx_release(this: *mut DsFxCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let rem = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::Relaxed);
        drop(Box::from_raw(this));
    }
    rem
}

unsafe extern "win64" fn dsfx_set_all_parameters(this: *mut DsFxCom, params: *const c_void) -> HRESULT {
    if this.is_null() || params.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let mut fx = (*this).effect.lock().unwrap();
    let len = fx.params.len();
    let slice = std::slice::from_raw_parts(params as *const u8, len);
    fx.params.copy_from_slice(slice);
    DS_OK
}

unsafe extern "win64" fn dsfx_get_all_parameters(this: *mut DsFxCom, params: *mut c_void) -> HRESULT {
    if this.is_null() || params.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let fx = (*this).effect.lock().unwrap();
    std::ptr::copy_nonoverlapping(fx.params.as_ptr(), params as *mut u8, fx.params.len());
    DS_OK
}

unsafe extern "win64" fn dsfx_i3dl2_set_preset(this: *mut DsFxCom, dw_preset: u32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let Some(params) = super::buffer::i3dl2_preset_params(dw_preset) else {
        return DSERR_INVALIDPARAM;
    };
    let mut fx = (*this).effect.lock().unwrap();
    fx.params = params;
    fx.i3dl2_preset = dw_preset;
    DS_OK
}

unsafe extern "win64" fn dsfx_i3dl2_get_preset(this: *mut DsFxCom, pdw_preset: *mut u32) -> HRESULT {
    if this.is_null() || pdw_preset.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let fx = (*this).effect.lock().unwrap();
    wr(pdw_preset, fx.i3dl2_preset);
    DS_OK
}

unsafe extern "win64" fn dsfx_i3dl2_set_quality(this: *mut DsFxCom, l_quality: i32) -> HRESULT {
    if this.is_null()
        || !(super::buffer::DSFX_I3DL2REVERB_QUALITY_MIN..=super::buffer::DSFX_I3DL2REVERB_QUALITY_MAX)
            .contains(&l_quality)
    {
        return DSERR_INVALIDPARAM;
    }
    let mut fx = (*this).effect.lock().unwrap();
    fx.i3dl2_quality = l_quality;
    DS_OK
}

unsafe extern "win64" fn dsfx_i3dl2_get_quality(this: *mut DsFxCom, pl_quality: *mut i32) -> HRESULT {
    if this.is_null() || pl_quality.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let fx = (*this).effect.lock().unwrap();
    wr(pl_quality, fx.i3dl2_quality);
    DS_OK
}

static DSFX_VTBL: Vtable<5> = Vtable([
    crate::Ptr(dsfx_qi as *const ()),
    crate::Ptr(dsfx_add_ref as *const ()),
    crate::Ptr(dsfx_release as *const ()),
    crate::Ptr(dsfx_set_all_parameters as *const ()),
    crate::Ptr(dsfx_get_all_parameters as *const ()),
]);

static DSFX_I3DL2REVERB_VTBL: Vtable<9> = Vtable([
    crate::Ptr(dsfx_qi as *const ()),                 // 0: QueryInterface
    crate::Ptr(dsfx_add_ref as *const ()),            // 1: AddRef
    crate::Ptr(dsfx_release as *const ()),            // 2: Release
    crate::Ptr(dsfx_set_all_parameters as *const ()), // 3: SetAllParameters
    crate::Ptr(dsfx_get_all_parameters as *const ()), // 4: GetAllParameters
    crate::Ptr(dsfx_i3dl2_set_preset as *const ()),   // 5: SetPreset
    crate::Ptr(dsfx_i3dl2_get_preset as *const ()),   // 6: GetPreset
    crate::Ptr(dsfx_i3dl2_set_quality as *const ()),  // 7: SetQuality
    crate::Ptr(dsfx_i3dl2_get_quality as *const ()),  // 8: GetQuality
]);

fn create_dsfx_com(effect: Arc<Mutex<BufferEffect>>) -> *mut DsFxCom {
    static REG: Once = Once::new();
    REG.call_once(|| {
        register_static_ptr_slice(&DSFX_VTBL.0);
        register_static_ptr_slice(&DSFX_I3DL2REVERB_VTBL.0);
    });
    let is_i3dl2 = effect.lock().unwrap().kind == DsFxKind::I3dl2Reverb;
    let vtbl = if is_i3dl2 {
        DSFX_I3DL2REVERB_VTBL.0.as_ptr()
    } else {
        DSFX_VTBL.0.as_ptr()
    };
    ACTIVE_OBJECTS.fetch_add(1, Ordering::Relaxed);
    Box::into_raw(Box::new(DsFxCom {
        vtbl,
        refs: AtomicU32::new(1),
        effect,
    }))
}

pub(super) unsafe extern "win64" fn dsb_set_fx(
    this: *mut DsBufferCom,
    dw_effects_count: u32,
    p_dsfx_desc: *const c_void,
    pdw_result_codes: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    if b.playing || !b.locks.is_empty() {
        return DSERR_INVALIDCALL;
    }
    if dw_effects_count == 0 {
        b.effects.clear();
        return DS_OK;
    }
    if (b.flags & DSBCAPS_CTRLFX) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    if p_dsfx_desc.is_null() {
        return DSERR_INVALIDPARAM;
    }

    let descs = p_dsfx_desc as *const DsEffectDesc;
    let mut new_effects = Vec::with_capacity(dw_effects_count as usize);
    let mut codes = vec![DSFXR_FAILED; dw_effects_count as usize];
    let mut any_failed = false;

    for i in 0..(dw_effects_count as usize) {
        let d = rd(descs.add(i));
        if d.dw_size != std::mem::size_of::<DsEffectDesc>() as u32 {
            return DSERR_INVALIDPARAM;
        }
        if (d.dw_flags & DSFX_LOCHARDWARE) != 0 {
            codes[i] = DSFXR_FAILED;
            any_failed = true;
            continue;
        }
        match DsFxKind::from_clsid(d.guid_dsfx_class) {
            Some(kind) => {
                new_effects.push(Arc::new(Mutex::new(BufferEffect::new(kind))));
                codes[i] = DSFXR_LOCSOFTWARE;
            }
            None => {
                codes[i] = DSFXR_UNKNOWN;
                any_failed = true;
            }
        }
    }

    if !pdw_result_codes.is_null() {
        for (i, &code) in codes.iter().enumerate() {
            wr(pdw_result_codes.add(i), code);
        }
    }
    if any_failed {
        // DirectSound 8 SetFX is atomic: on any unavailable/unknown DMO CLSID, leave existing FX intact.
        log::debug!(target: "audio", "IDirectSoundBuffer8::SetFX({dw_effects_count}): unrecognized/hardware FX -> DSERR_FXUNAVAILABLE");
        return DSERR_FXUNAVAILABLE;
    }
    b.effects = new_effects;
    DS_OK
}

unsafe extern "win64" fn dsb_acquire_resources(
    this: *mut DsBufferCom,
    _dw_flags: u32,
    dw_effects_count: u32,
    pdw_result_codes: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if dw_effects_count == 0 {
        return DS_OK;
    }
    let st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    if dw_effects_count as usize != b.effects.len() {
        return DSERR_INVALIDPARAM;
    }
    if !pdw_result_codes.is_null() {
        for i in 0..(dw_effects_count as usize) {
            wr(pdw_result_codes.add(i), DSFXR_LOCSOFTWARE);
        }
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dsb_get_object_in_path(
    this: *mut DsBufferCom,
    rguid_object: *const GUID,
    dw_index: u32,
    rguid_interface: *const GUID,
    pp_object: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_object);
    if this.is_null() || rguid_object.is_null() || rguid_interface.is_null() || pp_object.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj_guid = rd(rguid_object);
    let iface_guid = rd(rguid_interface);
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let mut matched = 0u32;
    for fx in &b.effects {
        let kind = fx.lock().unwrap().kind;
        if obj_guid == GUID_NULL || obj_guid == kind.clsid() {
            if matched == dw_index {
                if iface_guid != IID_IUnknown && iface_guid != kind.param_iid() {
                    return E_NOINTERFACE;
                }
                let com = create_dsfx_com(fx.clone());
                wr(pp_object, com as *mut c_void);
                return DS_OK;
            }
            matched += 1;
        }
    }
    DSERR_OBJECTNOTFOUND
}

// ============================================================================
// IDirectSound3DBuffer Methods
// ============================================================================

pub(super) unsafe extern "win64" fn ds3db_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_3d(this), riid, ppv)
}

unsafe extern "win64" fn ds3db_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_3d(this))
}

pub(super) unsafe extern "win64" fn ds3db_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_3d(this))
}

unsafe fn with_3d_buf<R>(
    this: *mut c_void,
    f: impl FnOnce(&SecondaryBufferState) -> R,
) -> Result<R, HRESULT> {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let obj = &*base;
    let st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get(&obj.buffer_id) else {
        return Err(DSERR_INVALIDCALL);
    };
    Ok(f(b))
}

unsafe fn with_3d_buf_mut(
    this: *mut c_void,
    dw_apply: u32,
    f: impl FnOnce(&mut Buffer3dParams) -> HRESULT,
) -> HRESULT {
    if dw_apply != DS3D_IMMEDIATE && dw_apply != DS3D_DEFERRED {
        return DSERR_INVALIDPARAM;
    }
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let mut next = if dw_apply == DS3D_DEFERRED {
        b.d3d_deferred
    } else {
        b.d3d_current
    };
    let hr = f(&mut next);
    if hr != DS_OK {
        return hr;
    }
    b.d3d_deferred = next;
    if dw_apply == DS3D_IMMEDIATE {
        b.d3d_current = next;
    }
    DS_OK
}

unsafe extern "win64" fn ds3db_get_all_parameters(
    this: *mut c_void,
    out: *mut Ds3dBuffer,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = rd(out as *const u32);
    if sz != std::mem::size_of::<Ds3dBuffer>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.to_abi()) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_angles(
    this: *mut c_void,
    inside_out: *mut u32,
    outside_out: *mut u32,
) -> HRESULT {
    if inside_out.is_null() || outside_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| {
        (
            b.d3d_current.inside_cone_angle,
            b.d3d_current.outside_cone_angle,
        )
    }) {
        Ok((i, o)) => {
            wr(inside_out, i);
            wr(outside_out, o);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_orientation(
    this: *mut c_void,
    out: *mut D3dVector,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.cone_orientation) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_outside_volume(
    this: *mut c_void,
    out: *mut i32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.cone_outside_volume) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_max_distance(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.max_distance) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_min_distance(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.min_distance) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_mode(this: *mut c_void, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.mode) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_position(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.position) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_velocity(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.velocity) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_set_all_parameters(
    this: *mut c_void,
    params_ptr: *const Ds3dBuffer,
    dw_apply: u32,
) -> HRESULT {
    if params_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let p = rd(params_ptr);
    if p.dw_size != std::mem::size_of::<Ds3dBuffer>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    if !vec3_is_finite(p.v_position)
        || !vec3_is_finite(p.v_velocity)
        || !vec3_is_finite(p.v_cone_orientation)
        || p.dw_inside_cone_angle > DS3D_MAXCONEANGLE
        || p.dw_outside_cone_angle > DS3D_MAXCONEANGLE
        || p.dw_inside_cone_angle > p.dw_outside_cone_angle
        || !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&p.l_cone_outside_volume)
        || !p.fl_min_distance.is_finite()
        || !p.fl_max_distance.is_finite()
        || p.fl_min_distance <= 0.0
        || p.fl_max_distance < p.fl_min_distance
        || p.dw_mode > DS3DMODE_DISABLE
    {
        return DSERR_INVALIDPARAM;
    }
    let cone_dir = vec3_normalize(p.v_cone_orientation).unwrap_or(Vec3::new(0.0, 0.0, 1.0));
    with_3d_buf_mut(this, dw_apply, |dst| {
        *dst = Buffer3dParams {
            position: p.v_position,
            velocity: p.v_velocity,
            inside_cone_angle: p.dw_inside_cone_angle,
            outside_cone_angle: p.dw_outside_cone_angle,
            cone_orientation: cone_dir,
            cone_outside_volume: p.l_cone_outside_volume,
            min_distance: p.fl_min_distance,
            max_distance: p.fl_max_distance,
            mode: p.dw_mode,
        };
        DS_OK
    })
}

pub(super) unsafe extern "win64" fn ds3db_set_cone_angles(
    this: *mut c_void,
    inside: u32,
    outside: u32,
    dw_apply: u32,
) -> HRESULT {
    if inside > DS3D_MAXCONEANGLE || outside > DS3D_MAXCONEANGLE || inside > outside {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.inside_cone_angle = inside;
        dst.outside_cone_angle = outside;
        DS_OK
    })
}

pub(super) unsafe extern "win64" fn ds3db_set_cone_orientation(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = Vec3::new(x, y, z);
    let Some(norm) = vec3_normalize(v) else {
        return DSERR_INVALIDPARAM;
    };
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.cone_orientation = norm;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_cone_orientation(*mut c_void, f32, f32, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3db_set_cone_outside_volume(
    this: *mut c_void,
    vol: i32,
    dw_apply: u32,
) -> HRESULT {
    if !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&vol) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.cone_outside_volume = vol;
        DS_OK
    })
}

pub(super) unsafe extern "win64" fn ds3db_set_max_distance(
    this: *mut c_void,
    max_d: f32,
    dw_apply: u32,
) -> HRESULT {
    if !max_d.is_finite() || max_d <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        if max_d < dst.min_distance {
            return DSERR_INVALIDPARAM;
        }
        dst.max_distance = max_d;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_max_distance(*mut c_void, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3db_set_min_distance(
    this: *mut c_void,
    min_d: f32,
    dw_apply: u32,
) -> HRESULT {
    if !min_d.is_finite() || min_d <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        if min_d > dst.max_distance {
            return DSERR_INVALIDPARAM;
        }
        dst.min_distance = min_d;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_min_distance(*mut c_void, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3db_set_mode(
    this: *mut c_void,
    mode: u32,
    dw_apply: u32,
) -> HRESULT {
    if mode > DS3DMODE_DISABLE {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.mode = mode;
        DS_OK
    })
}

pub(super) unsafe extern "win64" fn ds3db_set_position(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = Vec3::new(x, y, z);
    if !vec3_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.position = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_position(*mut c_void, f32, f32, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3db_set_velocity(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = Vec3::new(x, y, z);
    if !vec3_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.velocity = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_velocity(*mut c_void, f32, f32, f32, u32) -> HRESULT);

// ============================================================================
// IDirectSound3DListener Methods
// ============================================================================

unsafe extern "win64" fn ds3dl_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_3d(this), riid, ppv)
}

unsafe extern "win64" fn ds3dl_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_3d(this))
}

pub(super) unsafe extern "win64" fn ds3dl_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_3d(this))
}

unsafe fn with_listener<R>(
    this: *mut c_void,
    f: impl FnOnce(&Listener3dParams) -> R,
) -> Result<R, HRESULT> {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let obj = &*base;
    let st = obj.device.state.lock().unwrap();
    Ok(f(&st.listener_current))
}

unsafe fn with_listener_mut(
    this: *mut c_void,
    dw_apply: u32,
    f: impl FnOnce(&mut Listener3dParams) -> HRESULT,
) -> HRESULT {
    if dw_apply != DS3D_IMMEDIATE && dw_apply != DS3D_DEFERRED {
        return DSERR_INVALIDPARAM;
    }
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    let mut next = if dw_apply == DS3D_DEFERRED {
        st.listener_deferred
    } else {
        st.listener_current
    };
    let hr = f(&mut next);
    if hr != DS_OK {
        return hr;
    }
    st.listener_deferred = next;
    if dw_apply == DS3D_IMMEDIATE {
        st.listener_current = next;
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn ds3dl_get_all_parameters(
    this: *mut c_void,
    out: *mut Ds3dListener,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = rd(out as *const u32);
    if sz != std::mem::size_of::<Ds3dListener>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.to_abi()) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_distance_factor(
    this: *mut c_void,
    out: *mut f32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.distance_factor) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_doppler_factor(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.doppler_factor) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_orientation(
    this: *mut c_void,
    front_out: *mut D3dVector,
    top_out: *mut D3dVector,
) -> HRESULT {
    if front_out.is_null() && top_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| (l.orient_front, l.orient_top)) {
        Ok((f, t)) => {
            if !front_out.is_null() {
                wr(front_out, f);
            }
            if !top_out.is_null() {
                wr(top_out, t);
            }
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_position(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.position) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_rolloff_factor(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.rolloff_factor) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_velocity(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.velocity) {
        Ok(v) => {
            wr(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_set_all_parameters(
    this: *mut c_void,
    params_ptr: *const Ds3dListener,
    dw_apply: u32,
) -> HRESULT {
    if params_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let p = rd(params_ptr);
    if p.dw_size != std::mem::size_of::<Ds3dListener>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let Some(front) = vec3_normalize(p.v_orient_front) else {
        return DSERR_INVALIDPARAM;
    };
    let Some(top) = vec3_normalize(p.v_orient_top) else {
        return DSERR_INVALIDPARAM;
    };
    if vec3_normalize(vec3_cross(top, front)).is_none()
        || !vec3_is_finite(p.v_position)
        || !vec3_is_finite(p.v_velocity)
        || !p.fl_distance_factor.is_finite()
        || p.fl_distance_factor <= 0.0
        || !p.fl_rolloff_factor.is_finite()
        || !(DS3D_MINROLLOFFFACTOR..=DS3D_MAXROLLOFFFACTOR).contains(&p.fl_rolloff_factor)
        || !p.fl_doppler_factor.is_finite()
        || !(DS3D_MINDOPPLERFACTOR..=DS3D_MAXDOPPLERFACTOR).contains(&p.fl_doppler_factor)
    {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        *dst = Listener3dParams {
            position: p.v_position,
            velocity: p.v_velocity,
            orient_front: front,
            orient_top: top,
            distance_factor: p.fl_distance_factor,
            rolloff_factor: p.fl_rolloff_factor,
            doppler_factor: p.fl_doppler_factor,
        };
        DS_OK
    })
}

pub(super) unsafe extern "win64" fn ds3dl_set_distance_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || factor <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.distance_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_distance_factor(*mut c_void, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_set_doppler_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || !(DS3D_MINDOPPLERFACTOR..=DS3D_MAXDOPPLERFACTOR).contains(&factor) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.doppler_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_doppler_factor(*mut c_void, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_set_orientation(
    this: *mut c_void,
    x_front: f32,
    y_front: f32,
    z_front: f32,
    x_top: f32,
    y_top: f32,
    z_top: f32,
    dw_apply: u32,
) -> HRESULT {
    let Some(front) = vec3_normalize(Vec3::new(x_front, y_front, z_front)) else {
        return DSERR_INVALIDPARAM;
    };
    let Some(top) = vec3_normalize(Vec3::new(x_top, y_top, z_top)) else {
        return DSERR_INVALIDPARAM;
    };
    if vec3_normalize(vec3_cross(top, front)).is_none() {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.orient_front = front;
        dst.orient_top = top;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_orientation(*mut c_void, f32, f32, f32, f32, f32, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_set_position(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = Vec3::new(x, y, z);
    if !vec3_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.position = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_position(*mut c_void, f32, f32, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_set_rolloff_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || !(DS3D_MINROLLOFFFACTOR..=DS3D_MAXROLLOFFFACTOR).contains(&factor) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.rolloff_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_rolloff_factor(*mut c_void, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_set_velocity(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = Vec3::new(x, y, z);
    if !vec3_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.velocity = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_velocity(*mut c_void, f32, f32, f32, u32) -> HRESULT);

pub(super) unsafe extern "win64" fn ds3dl_commit_deferred_settings(this: *mut c_void) -> HRESULT {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    st.listener_current = st.listener_deferred;
    for buf in st.buffers.values_mut() {
        buf.d3d_current = buf.d3d_deferred;
    }
    DS_OK
}

// ============================================================================
// IDirectSoundNotify (Playback Buffer)
// ============================================================================

unsafe fn parse_notifies(
    count: u32,
    notifies: *const DsbPositionNotify,
    buf_len: u32,
) -> Result<Vec<NotifyEntry>, HRESULT> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if notifies.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..(count as usize) {
        let entry = rd(notifies.add(i));
        if entry.dw_offset != DSBPN_OFFSETSTOP && entry.dw_offset >= buf_len {
            return Err(DSERR_INVALIDPARAM);
        }
        let Some(ev) = crate::handles::get_as::<crate::handles::Event>(entry.h_event_notify) else {
            return Err(DSERR_INVALIDPARAM);
        };
        out.push(NotifyEntry {
            offset: entry.dw_offset,
            event: ev,
        });
    }
    Ok(out)
}

unsafe extern "win64" fn dsn_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_notify(this), riid, ppv)
}

unsafe extern "win64" fn dsn_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_notify(this))
}

pub(super) unsafe extern "win64" fn dsn_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_notify(this))
}

pub(super) unsafe extern "win64" fn dsn_set_notification_positions(
    this: *mut c_void,
    count: u32,
    notifies: *const DsbPositionNotify,
) -> HRESULT {
    let base = dsbuf_from_notify(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    if b.playing {
        return DSERR_INVALIDCALL;
    }
    let buf_len = b.sample_mem.byte_len() as u32;
    let parsed = match parse_notifies(count, notifies, buf_len) {
        Ok(v) => v,
        Err(hr) => return hr,
    };
    b.notifies = parsed;
    DS_OK
}

// ============================================================================
// IDirectSoundCapture / IDirectSoundCapture8 Methods
// ============================================================================

unsafe extern "win64" fn dsc_query_interface(
    this: *mut DsCaptureCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IDIRECTSOUNDCAPTURE || iid == IID_IDIRECTSOUNDCAPTURE8 {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn dsc_add_ref(this: *mut DsCaptureCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn dsc_release(this: *mut DsCaptureCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe fn create_capture_buffer_core(
    core: &Arc<DsCaptureCore>,
    desc_ptr: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if pp_buf.is_null() {
        return DSERR_INVALIDPARAM;
    }
    write_null_out(pp_buf);
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if desc_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }

    let dw_size = rd(desc_ptr as *const u32);
    let (flags, buffer_bytes, reserved, wfx_ptr) = if dw_size == 24 {
        let d = rd(desc_ptr as *const DscBufferDesc1);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else if dw_size == 40 {
        let d = rd(desc_ptr as *const DscBufferDesc);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else {
        return DSERR_INVALIDPARAM;
    };

    if reserved != 0 || !(DSBSIZE_MIN..=DSBSIZE_MAX).contains(&buffer_bytes) {
        return DSERR_INVALIDPARAM;
    }
    let fmt = match parse_ds_format(wfx_ptr) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    if (buffer_bytes as usize) < (fmt.block_align as usize) {
        return DSERR_INVALIDPARAM;
    }

    let sample_mem = SampleMemory::new(buffer_bytes as usize, fmt);
    let buf = alloc_ds_capture_buffer(core.clone(), flags, sample_mem);
    wr(pp_buf, buf as *mut c_void);
    DS_OK
}

pub(super) unsafe extern "win64" fn dsc_create_capture_buffer(
    this: *mut DsCaptureCom,
    desc: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        write_null_out(pp_buf);
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        obj.core.clone()
    };
    create_capture_buffer_core(&core, desc, pp_buf, p_unk_outer)
}

pub(super) unsafe extern "win64" fn dsc_get_caps(
    this: *mut DsCaptureCom,
    caps: *mut DscCaps,
) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let sz = rd(caps as *const u32);
    if sz != std::mem::size_of::<DscCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let ch = obj.core.device.lock().unwrap().channels.max(2);
    wr(
        caps,
        DscCaps {
            dw_size: sz,
            dw_flags: DSCCAPS_CERTIFIED | DSCCAPS_MULTIPLECAPTURE,
            dw_formats: STANDARD_CAPTURE_FORMATS,
            dw_channels: ch,
        },
    );
    DS_OK
}

unsafe extern "win64" fn dsc_initialize(
    this: *mut DsCaptureCom,
    guid_ptr: *const GUID,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }
    let guid_opt = GUID::read(guid_ptr);
    let dev = match resolve_capture_device(guid_opt) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    *obj.core.device.lock().unwrap() = dev;
    obj.core.initialized.store(true, Ordering::Release);
    DS_OK
}

// ============================================================================
// IDirectSoundCaptureBuffer / IDirectSoundCaptureBuffer8 + Notify
// ============================================================================

unsafe fn dscbuf_qi_common(
    base: *mut DsCaptureBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if base.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let obj = &*base;
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUNDCAPTUREBUFFER
        || iid == IID_IDIRECTSOUNDCAPTUREBUFFER8
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, base as *mut c_void);
        return DS_OK;
    }
    if iid == IID_IDIRECTSOUNDNOTIFY {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsCaptureBufferCom, vtbl_notify));
        wr(ppv, sub as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe fn dscbuf_add_ref_common(base: *mut DsCaptureBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn dscbuf_release_common(base: *mut DsCaptureBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*base;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(base));
        0
    } else {
        prev - 1
    }
}

#[inline]
unsafe fn dscbuf_from_notify(this: *mut c_void) -> *mut DsCaptureBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsCaptureBufferCom, vtbl_notify))
            as *mut DsCaptureBufferCom
    }
}

unsafe extern "win64" fn dscb_query_interface(
    this: *mut DsCaptureBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dscbuf_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn dscb_add_ref(this: *mut DsCaptureBufferCom) -> u32 {
    dscbuf_add_ref_common(this)
}

pub(super) unsafe extern "win64" fn dscb_release(this: *mut DsCaptureBufferCom) -> u32 {
    dscbuf_release_common(this)
}

unsafe extern "win64" fn dscb_get_caps(
    this: *mut DsCaptureBufferCom,
    caps: *mut DscbCaps,
) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = rd(caps as *const u32);
    if sz != std::mem::size_of::<DscbCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.state.lock().unwrap();
    wr(
        caps,
        DscbCaps {
            dw_size: sz,
            dw_flags: st.flags,
            dw_buffer_bytes: st.sample_mem.byte_len() as u32,
            dw_reserved: 0,
        },
    );
    DS_OK
}

pub(super) unsafe extern "win64" fn dscb_get_current_position(
    this: *mut DsCaptureBufferCom,
    cap_out: *mut u32,
    read_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let (cap_pos, read_pos, events) = {
        let mut st = obj.state.lock().unwrap();
        st.advance_and_cursors()
    };
    signal_events(events);
    if !cap_out.is_null() {
        wr(cap_out, cap_pos);
    }
    if !read_out.is_null() {
        wr(read_out, read_pos);
    }
    DS_OK
}

unsafe extern "win64" fn dscb_get_format(
    this: *mut DsCaptureBufferCom,
    wfx_out: *mut u8,
    allocated: u32,
    written_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let fmt = obj.state.lock().unwrap().sample_mem.format;
    write_ds_format(&fmt, wfx_out, allocated, written_out)
}

pub(super) unsafe extern "win64" fn dscb_get_status(
    this: *mut DsCaptureBufferCom,
    status_out: *mut u32,
) -> HRESULT {
    if this.is_null() || status_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let (status, events) = {
        let mut st = obj.state.lock().unwrap();
        let (_, _, ev) = st.advance_and_cursors();
        let mut s = 0u32;
        if st.capturing {
            s |= DSCBSTATUS_CAPTURING;
            if st.looping {
                s |= DSCBSTATUS_LOOPING;
            }
        }
        (s, ev)
    };
    signal_events(events);
    wr(status_out, status);
    DS_OK
}

unsafe extern "win64" fn dscb_initialize(
    _this: *mut DsCaptureBufferCom,
    _dsc: *mut c_void,
    _desc: *const c_void,
) -> HRESULT {
    DSERR_ALREADYINITIALIZED
}

pub(super) unsafe extern "win64" fn dscb_lock(
    this: *mut DsCaptureBufferCom,
    dw_offset: u32,
    dw_bytes: u32,
    ppv_ptr1: *mut *mut c_void,
    pdw_bytes1: *mut u32,
    ppv_ptr2: *mut *mut c_void,
    pdw_bytes2: *mut u32,
    dw_flags: u32,
) -> HRESULT {
    write_null_out(ppv_ptr1);
    if !pdw_bytes1.is_null() {
        wr(pdw_bytes1, 0);
    }
    write_null_out(ppv_ptr2);
    if !pdw_bytes2.is_null() {
        wr(pdw_bytes2, 0);
    }
    if this.is_null() || ppv_ptr1.is_null() || pdw_bytes1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if ppv_ptr2.is_null() != pdw_bytes2.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let buf_len = st.sample_mem.byte_len() as u32;
    let bytes = if (dw_flags & DSCBLOCK_ENTIREBUFFER) != 0 {
        buf_len
    } else {
        dw_bytes
    };
    if dw_offset >= buf_len || bytes == 0 || bytes > buf_len {
        return DSERR_INVALIDPARAM;
    }
    let base = st.sample_mem.base_mut_ptr();
    let first_len = bytes.min(buf_len - dw_offset);
    let second_len = bytes - first_len;
    let ptr1 = base.add(dw_offset as usize) as *mut c_void;
    let (ptr2, actual_len2) = if second_len > 0 && !ppv_ptr2.is_null() {
        (base as *mut c_void, second_len)
    } else {
        (std::ptr::null_mut(), 0)
    };
    st.locks.push(ActiveLock {
        ptr1: ptr1 as usize,
        len1: first_len,
        ptr2: ptr2 as usize,
        len2: actual_len2,
    });
    wr(ppv_ptr1, ptr1);
    wr(pdw_bytes1, first_len);
    if !ppv_ptr2.is_null() {
        wr(ppv_ptr2, ptr2);
        wr(pdw_bytes2, actual_len2);
    }
    DS_OK
}

pub(super) unsafe extern "win64" fn dscb_start(
    this: *mut DsCaptureBufferCom,
    dw_flags: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    st.capturing = true;
    st.looping = (dw_flags & DSCBSTART_LOOPING) != 0;
    st.start_instant = Some(Instant::now());
    DS_OK
}

pub(super) unsafe extern "win64" fn dscb_stop(this: *mut DsCaptureBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let events = {
        let mut st = obj.state.lock().unwrap();
        let (_, _, mut ev) = st.advance_and_cursors();
        if st.capturing {
            st.capturing = false;
            st.looping = false;
            st.start_instant = None;
            for n in &st.notifies {
                if n.offset == DSBPN_OFFSETSTOP {
                    ev.push(n.event.clone());
                }
            }
        }
        ev
    };
    signal_events(events);
    DS_OK
}

pub(super) unsafe extern "win64" fn dscb_unlock(
    this: *mut DsCaptureBufferCom,
    pv_ptr1: *mut c_void,
    dw_bytes1: u32,
    pv_ptr2: *mut c_void,
    dw_bytes2: u32,
) -> HRESULT {
    if this.is_null() || pv_ptr1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let pos = st.locks.iter().position(|lk| {
        lk.ptr1 == (pv_ptr1 as usize)
            && dw_bytes1 <= lk.len1
            && (dw_bytes2 == 0 || (lk.ptr2 == (pv_ptr2 as usize) && dw_bytes2 <= lk.len2))
    });
    let Some(idx) = pos else {
        return DSERR_INVALIDPARAM;
    };
    st.locks.swap_remove(idx);
    std::sync::atomic::fence(Ordering::Release);
    DS_OK
}

unsafe extern "win64" fn dscb_get_object_in_path(
    this: *mut DsCaptureBufferCom,
    _rguid_object: *const GUID,
    _dw_index: u32,
    _rguid_interface: *const GUID,
    pp_object: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_object);
    if this.is_null() || pp_object.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DSERR_OBJECTNOTFOUND
}

unsafe extern "win64" fn dscb_get_fx_status(
    this: *mut DsCaptureBufferCom,
    dw_fx_count: u32,
    pdw_fx_status: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if dw_fx_count == 0 {
        return DS_OK;
    }
    if pdw_fx_status.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DSERR_INVALIDPARAM
}

unsafe extern "win64" fn dscn_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dscbuf_qi_common(dscbuf_from_notify(this), riid, ppv)
}

unsafe extern "win64" fn dscn_add_ref(this: *mut c_void) -> u32 {
    dscbuf_add_ref_common(dscbuf_from_notify(this))
}

unsafe extern "win64" fn dscn_release(this: *mut c_void) -> u32 {
    dscbuf_release_common(dscbuf_from_notify(this))
}

unsafe extern "win64" fn dscn_set_notification_positions(
    this: *mut c_void,
    count: u32,
    notifies: *const DsbPositionNotify,
) -> HRESULT {
    let base = dscbuf_from_notify(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.state.lock().unwrap();
    if st.capturing {
        return DSERR_INVALIDCALL;
    }
    let buf_len = st.sample_mem.byte_len() as u32;
    let parsed = match parse_notifies(count, notifies, buf_len) {
        Ok(v) => v,
        Err(hr) => return hr,
    };
    st.notifies = parsed;
    DS_OK
}

// ============================================================================
// IDirectSoundFullDuplex Methods
// ============================================================================

unsafe extern "win64" fn dsfd_query_interface(
    this: *mut DsFullDuplexCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IDIRECTSOUNDFULLDUPLEX {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn dsfd_add_ref(this: *mut DsFullDuplexCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn dsfd_release(this: *mut DsFullDuplexCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

pub(super) unsafe extern "win64" fn dsfd_initialize(
    this: *mut DsFullDuplexCom,
    capture_guid: *const GUID,
    render_guid: *const GUID,
    dsc_desc: *const c_void,
    ds_desc: *const c_void,
    _hwnd: HWND,
    dw_level: u32,
    pp_dscb8: *mut *mut c_void,
    pp_dsb8: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_dscb8);
    write_null_out(pp_dsb8);
    if this.is_null()
        || dsc_desc.is_null()
        || ds_desc.is_null()
        || pp_dscb8.is_null()
        || pp_dsb8.is_null()
    {
        return DSERR_INVALIDPARAM;
    }
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&dw_level) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }

    let cap_dev = match resolve_capture_device(GUID::read(capture_guid)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let ren_dev = match resolve_render_device(GUID::read(render_guid)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };

    let cap_core = DsCaptureCore::new(cap_dev, true);
    let ren_core = DsDeviceCore::new(ren_dev, true, true);
    ren_core.state.lock().unwrap().cooperative_level = dw_level;

    let mut raw_cap_buf: *mut c_void = std::ptr::null_mut();
    let hr_cap =
        create_capture_buffer_core(&cap_core, dsc_desc, &mut raw_cap_buf, std::ptr::null_mut());
    if hr_cap != DS_OK {
        return hr_cap;
    }

    let mut raw_ren_buf: *mut c_void = std::ptr::null_mut();
    let hr_ren =
        create_sound_buffer_core(&ren_core, ds_desc, &mut raw_ren_buf, std::ptr::null_mut());
    if hr_ren != DS_OK {
        dscb_release(raw_cap_buf as *mut DsCaptureBufferCom);
        return hr_ren;
    }

    let mut dscb8: *mut c_void = std::ptr::null_mut();
    let hr_qi_cap = dscb_query_interface(
        raw_cap_buf as *mut DsCaptureBufferCom,
        &IID_IDIRECTSOUNDCAPTUREBUFFER8,
        &mut dscb8,
    );
    dscb_release(raw_cap_buf as *mut DsCaptureBufferCom);
    if hr_qi_cap != DS_OK {
        dsb_release(raw_ren_buf as *mut DsBufferCom);
        return hr_qi_cap;
    }

    let mut dsb8: *mut c_void = std::ptr::null_mut();
    let hr_qi_ren = dsb_query_interface(
        raw_ren_buf as *mut DsBufferCom,
        &IID_IDIRECTSOUNDBUFFER8,
        &mut dsb8,
    );
    dsb_release(raw_ren_buf as *mut DsBufferCom);
    if hr_qi_ren != DS_OK {
        dscb_release(dscb8 as *mut DsCaptureBufferCom);
        return hr_qi_ren;
    }

    *obj.capture.lock().unwrap() = Some(cap_core);
    *obj.render.lock().unwrap() = Some(ren_core);
    obj.initialized.store(true, Ordering::Release);

    wr(pp_dscb8, dscb8);
    wr(pp_dsb8, dsb8);
    DS_OK
}
