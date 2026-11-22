//! mmdevapi: `IMMDeviceEnumerator`, `IMMDeviceCollection`, `IMMDevice`,
//! `IMMEndpoint` over the host output and capture devices ([`super::backend`]).
//!
//! Render endpoints have Windows ids `{0.0.0.00000000}.{GUID}` and capture
//! endpoints have `{0.0.1.00000000}.{GUID}` with GUIDs derived from the host
//! device UID, so they are stable across runs (AK hashes them with FNV into
//! device ids).
//!
//! `IMMNotificationClient`s registered here are driven by
//! [`super::devnotify`] (CoreAudio device-list / default-device / alive
//! listeners).

use super::backend::{self, DeviceInfo};
use super::com::*;
use super::format::{default_mask, WaveFormat};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

pub const eRender: u32 = 0;
pub const eCapture: u32 = 1;
pub const eAll: u32 = 2;
pub const DEVICE_STATE_ACTIVE: u32 = 1;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4;
pub const DEVICE_STATEMASK_ALL: u32 = 0xF;

/// Deterministic Windows container GUID shared between a USB DualSense HID
/// device node (`misc::setupapi`) and its USB audio-haptics endpoint
/// (`audio::mmdevapi`).
pub fn dualsense_usb_container_id(slot: u32) -> GUID {
    GUID::new(
        0xD5E0_054C,
        0x0CE6,
        0x4000 | ((slot as u16) & 0x0FFF),
        [0x80, 0x54, 0x43, 0x0C, 0xE6, 0x00, (slot >> 8) as u8, slot as u8],
    )
}

/// Format a `GUID` as `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` (matching
/// `ole32!StringFromGUID2` and `SPDRP_BASE_CONTAINERID`).
pub fn format_guid(g: &GUID) -> String {
    format!("{g:?}")
}

/// True if a CoreAudio/virtual `DeviceInfo` represents a USB DualSense
/// controller's audio-haptics endpoint.
pub fn is_dualsense_audio_device(info: &DeviceInfo) -> bool {
    let name = info.name.to_ascii_lowercase();
    let uid = info.uid.to_ascii_lowercase();
    name.contains("dualsense")
        || uid.contains("dualsense")
        || (uid.contains("054c") && (uid.contains("0ce6") || uid.contains("0df2")))
        || (name.contains("wireless controller") && info.channels == 4)
}

/// One render or capture endpoint.
pub struct Endpoint {
    pub flow: u32,
    pub info: DeviceInfo,
    pub id: String,
    pub guid: GUID,
    pub container_id: GUID,
}

fn fnv64(seed: u64, s: &str) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64 ^ seed;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn guid_from(s: &str) -> GUID {
    let a = fnv64(0x6177_3261_7564_696f, s).to_le_bytes();
    let b = fnv64(0x1234_5678_9abc_def0, s).to_le_bytes();
    let mut d4 = [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]];
    d4[0] = (d4[0] & 0x3F) | 0x80; // RFC 4122 variant
    GUID::new(
        u32::from_le_bytes([a[0], a[1], a[2], a[3]]),
        u16::from_le_bytes([a[4], a[5]]),
        (u16::from_le_bytes([a[6], a[7]]) & 0x0FFF) | 0x5000, // name-based
        d4,
    )
}

impl Endpoint {
    pub fn new(info: DeviceInfo) -> Endpoint {
        Self::new_with_slot(info, None)
    }

    pub fn new_with_slot(info: DeviceInfo, dualsense_slot: Option<u32>) -> Endpoint {
        let guid = guid_from(&info.uid);
        let container_id = match dualsense_slot {
            Some(slot) => dualsense_usb_container_id(slot),
            None if is_dualsense_audio_device(&info) => dualsense_usb_container_id(0),
            None => guid_from(&format!("container:{}", info.uid)),
        };
        let id = format!("{{0.0.0.00000000}}.{guid:?}").to_ascii_lowercase();
        Endpoint { flow: eRender, info, id, guid, container_id }
    }

    pub fn new_capture(info: DeviceInfo) -> Endpoint {
        Self::new_capture_with_slot(info, None)
    }

    pub fn new_capture_with_slot(info: DeviceInfo, dualsense_slot: Option<u32>) -> Endpoint {
        let guid = guid_from(&format!("capture:{}", info.uid));
        let container_id = match dualsense_slot {
            Some(slot) => dualsense_usb_container_id(slot),
            None if is_dualsense_audio_device(&info) => dualsense_usb_container_id(0),
            None => guid_from(&format!("container:capture:{}", info.uid)),
        };
        let id = format!("{{0.0.1.00000000}}.{guid:?}").to_ascii_lowercase();
        Endpoint { flow: eCapture, info, id, guid, container_id }
    }

    pub fn is_capture(&self) -> bool {
        self.flow == eCapture
    }

    /// Shared-mode mix format channel count. Default **stereo** whatever the
    /// device has (AUHAL maps channel n to device channel n). Capture endpoints
    /// expose mono if the device has 1 channel, else stereo. USB DualSense
    /// audio-haptics endpoints expose 4 channels (headphone L/R + haptics L/R).
    /// With `AW2_AUDIO_CHANNELS=device` the device's own count is used when
    /// Wwise can map it (1/2/4/6/8 -> mono/stereo/quad/5.1/7.1, `FUN_141654e50`);
    /// an explicit `AW2_AUDIO_CHANNELS=1|2|4|6|8` forces a count.
    pub fn mix_channels(&self) -> u16 {
        if self.flow == eCapture {
            return if self.info.channels == 1 { 1 } else { 2 };
        }
        let mappable = |c: u32| matches!(c, 1 | 2 | 4 | 6 | 8);
        match std::env::var("AW2_AUDIO_CHANNELS").as_deref() {
            Ok("device") if mappable(self.info.channels) => self.info.channels as u16,
            Ok(s) => match s.parse::<u32>() {
                Ok(c) if mappable(c) => c as u16,
                _ => 2,
            },
            Err(_) if is_dualsense_audio_device(&self.info) && self.info.channels >= 4 => 4,
            Err(_) => 2,
        }
    }

    /// Shared-mode mix format rate. Default **48 kHz** (Wwise's engine rate,
    /// `DAT_1438152ac`), so Wwise never needs AUTOCONVERTPCM or its own SRC and
    /// its output reaches the tap bit-exact; AUHAL resamples to the device.
    /// `AW2_AUDIO_RATE=device` (or a number) overrides for render endpoints.
    pub fn mix_rate(&self) -> u32 {
        if self.flow == eCapture {
            return 48000;
        }
        match std::env::var("AW2_AUDIO_RATE").as_deref() {
            Ok("device") => self.info.rate,
            Ok(s) => s.parse::<u32>().ok().filter(|r| (8000..=192_000).contains(r)).unwrap_or(48000),
            Err(_) => 48000,
        }
    }

    pub fn mix_format(&self) -> WaveFormat {
        let ch = self.mix_channels();
        WaveFormat::float_mix(self.mix_rate(), ch, default_mask(ch))
    }

    /// Windows style "Speakers (Device Name)" / "Headphones (...)" / "Microphone (...)".
    pub fn friendly_name(&self) -> String {
        let kind = if self.flow == eCapture {
            "Microphone"
        } else if self.info.headphones {
            "Headphones"
        } else {
            "Speakers"
        };
        format!("{kind} ({})", self.info.name)
    }
}

static ENDPOINTS: Mutex<Vec<Arc<Endpoint>>> = Mutex::new(Vec::new());
static CAPTURE_ENDPOINTS: Mutex<Vec<Arc<Endpoint>>> = Mutex::new(Vec::new());

/// Re-scan host render devices (default first). Existing `Arc<Endpoint>`s stay valid.
pub fn refresh() -> Vec<Arc<Endpoint>> {
    refresh_with(backend::output_devices())
}

/// Replace the render endpoint list with `devs` (default first).
pub fn refresh_with(mut devs: Vec<DeviceInfo>) -> Vec<Arc<Endpoint>> {
    // Endpoints the notification layer reported as removed stay hidden until
    // it reports them back (only differs from the host list while a removal
    // is simulated, `AW2_AUDIO_SIMULATE_UNPLUG`).
    devs.retain(|d| super::devnotify::endpoint_present(&Endpoint::new(d.clone()).id));
    let mut g = ENDPOINTS.lock().unwrap();
    let mut out = Vec::with_capacity(devs.len());
    let mut ds_slot = 0u32;
    for d in devs {
        let slot = if is_dualsense_audio_device(&d) {
            let s = ds_slot;
            ds_slot += 1;
            Some(s)
        } else {
            None
        };
        let ep = Endpoint::new_with_slot(d.clone(), slot);
        // `is_default` picks the AU flavour (DefaultOutput follows switches),
        // so a device that stopped/started being the default gets a new Endpoint.
        let reuse = g
            .iter()
            .find(|e| {
                e.info.uid == d.uid
                    && e.info.id == d.id
                    && e.info.rate == d.rate
                    && e.info.channels == d.channels
                    && e.info.is_default == d.is_default
                    && e.container_id == ep.container_id
            })
            .cloned();
        out.push(reuse.unwrap_or_else(|| {
            log::info!(target: "audio", "endpoint '{}' ({} Hz, {} ch, uid {}) -> {}", d.name, d.rate, d.channels, d.uid, ep.id);
            Arc::new(ep)
        }));
    }
    *g = out.clone();
    out
}

/// Re-scan host capture devices (default first).
pub fn refresh_capture() -> Vec<Arc<Endpoint>> {
    refresh_capture_with(backend::input_devices())
}

/// Replace the capture endpoint list with `devs` (default first).
pub fn refresh_capture_with(mut devs: Vec<DeviceInfo>) -> Vec<Arc<Endpoint>> {
    devs.retain(|d| super::devnotify::endpoint_present(&Endpoint::new_capture(d.clone()).id));
    let mut g = CAPTURE_ENDPOINTS.lock().unwrap();
    let mut out = Vec::with_capacity(devs.len());
    let mut ds_slot = 0u32;
    for d in devs {
        let slot = if is_dualsense_audio_device(&d) {
            let s = ds_slot;
            ds_slot += 1;
            Some(s)
        } else {
            None
        };
        let ep = Endpoint::new_capture_with_slot(d.clone(), slot);
        let reuse = g
            .iter()
            .find(|e| {
                e.info.uid == d.uid
                    && e.info.id == d.id
                    && e.info.rate == d.rate
                    && e.info.channels == d.channels
                    && e.info.is_default == d.is_default
                    && e.container_id == ep.container_id
            })
            .cloned();
        out.push(reuse.unwrap_or_else(|| {
            log::info!(target: "audio", "capture endpoint '{}' ({} Hz, {} ch, uid {}) -> {}", d.name, d.rate, d.channels, d.uid, ep.id);
            Arc::new(ep)
        }));
    }
    *g = out.clone();
    out
}

pub fn endpoints() -> Vec<Arc<Endpoint>> {
    let g = ENDPOINTS.lock().unwrap();
    if !g.is_empty() {
        return g.clone();
    }
    drop(g);
    refresh()
}

pub fn capture_endpoints() -> Vec<Arc<Endpoint>> {
    let g = CAPTURE_ENDPOINTS.lock().unwrap();
    if !g.is_empty() {
        return g.clone();
    }
    drop(g);
    refresh_capture()
}

pub fn all_endpoints() -> Vec<Arc<Endpoint>> {
    let mut v = endpoints();
    v.extend(capture_endpoints());
    v
}

// ---------------------------------------------------------------------------
// IMMDeviceEnumerator
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IMMDeviceEnumeratorVtbl {
    qi: unsafe extern "win64" fn(*mut EnumObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut EnumObj) -> u32,
    release: unsafe extern "win64" fn(*mut EnumObj) -> u32,
    enum_audio_endpoints: unsafe extern "win64" fn(*mut EnumObj, u32, u32, *mut *mut CollObj) -> HRESULT,
    get_default_audio_endpoint: unsafe extern "win64" fn(*mut EnumObj, u32, u32, *mut *mut DeviceObj) -> HRESULT,
    get_device: unsafe extern "win64" fn(*mut EnumObj, *const u16, *mut *mut DeviceObj) -> HRESULT,
    register_cb: unsafe extern "win64" fn(*mut EnumObj, *mut c_void) -> HRESULT,
    unregister_cb: unsafe extern "win64" fn(*mut EnumObj, *mut c_void) -> HRESULT,
}

pub type EnumObj = ComObj<IMMDeviceEnumeratorVtbl, ()>;

/// Registered `IMMNotificationClient`s (guest pointers, AddRef'd).
pub(super) static NOTIFY_CLIENTS: Mutex<Vec<usize>> = Mutex::new(Vec::new());

mod enumerator {
    use super::*;
    crate::com_iunknown!(EnumObj, [IID_IMMDeviceEnumerator]);

    pub unsafe extern "win64" fn enum_audio_endpoints(_t: *mut EnumObj, flow: u32, mask: u32, out: *mut *mut CollObj) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        if flow > eAll || mask & !DEVICE_STATEMASK_ALL != 0 || mask == 0 {
            return E_INVALIDARG;
        }
        let list = if mask & DEVICE_STATE_ACTIVE == 0 {
            Vec::new()
        } else {
            match flow {
                eRender => refresh(),
                eCapture => refresh_capture(),
                eAll => {
                    let mut v = refresh();
                    v.extend(refresh_capture());
                    v
                }
                _ => Vec::new(),
            }
        };
        log::debug!(target: "audio", "EnumAudioEndpoints(flow {flow}, mask {mask:#x}) -> {} endpoint(s)", list.len());
        std::ptr::write_unaligned(out, CollObj::create(&COLL_VTBL, list));
        S_OK
    }

    pub unsafe extern "win64" fn get_default_audio_endpoint(_t: *mut EnumObj, flow: u32, role: u32, out: *mut *mut DeviceObj) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        if (flow != eRender && flow != eCapture) || role > 2 {
            return E_INVALIDARG;
        }
        let first = if flow == eCapture {
            refresh_capture().into_iter().next()
        } else {
            refresh().into_iter().next()
        };
        match first {
            Some(ep) => {
                log::debug!(
                    target: "audio",
                    "GetDefaultAudioEndpoint({}, role {role}) -> {} '{}'",
                    if flow == eCapture { "capture" } else { "render" },
                    ep.id,
                    ep.info.name
                );
                std::ptr::write_unaligned(out, new_device(ep));
                S_OK
            }
            None => E_NOTFOUND,
        }
    }

    pub unsafe extern "win64" fn get_device(_t: *mut EnumObj, id: *const u16, out: *mut *mut DeviceObj) -> HRESULT {
        if out.is_null() || id.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        let s = crate::strings::from_wide(id).to_ascii_lowercase();
        let found = endpoints()
            .into_iter()
            .chain(capture_endpoints())
            .chain(refresh())
            .chain(refresh_capture())
            .find(|e| e.id == s);
        match found {
            Some(ep) => {
                std::ptr::write_unaligned(out, new_device(ep));
                S_OK
            }
            None => {
                log::info!(target: "audio", "GetDevice({s}) -> E_NOTFOUND");
                E_NOTFOUND
            }
        }
    }

    /// Call IUnknown::AddRef (slot 1) or Release (slot 2) on a guest object.
    unsafe fn unknown_call(obj: *mut c_void, slot: usize) -> u32 {
        let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
        let method = std::ptr::read_unaligned(vtbl.add(slot));
        crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
    }

    pub unsafe extern "win64" fn register_cb(_t: *mut EnumObj, cb: *mut c_void) -> HRESULT {
        if cb.is_null() {
            return E_POINTER;
        }
        unknown_call(cb, 1);
        NOTIFY_CLIENTS.lock().unwrap().push(cb as usize);
        log::info!(target: "audio", "IMMNotificationClient {cb:p} registered");
        super::super::devnotify::start();
        S_OK
    }

    pub unsafe extern "win64" fn unregister_cb(_t: *mut EnumObj, cb: *mut c_void) -> HRESULT {
        if cb.is_null() {
            return E_POINTER;
        }
        let found = {
            let mut g = NOTIFY_CLIENTS.lock().unwrap();
            g.iter().position(|&p| p == cb as usize).map(|i| g.remove(i))
        };
        match found {
            Some(_) => {
                unknown_call(cb, 2);
                S_OK
            }
            None => E_NOTFOUND,
        }
    }
}

static ENUM_VTBL: IMMDeviceEnumeratorVtbl = IMMDeviceEnumeratorVtbl {
    qi: enumerator::qi,
    add_ref: enumerator::add_ref,
    release: enumerator::release,
    enum_audio_endpoints: enumerator::enum_audio_endpoints,
    get_default_audio_endpoint: enumerator::get_default_audio_endpoint,
    get_device: enumerator::get_device,
    register_cb: enumerator::register_cb,
    unregister_cb: enumerator::unregister_cb,
};

/// `CoCreateInstance(CLSID_MMDeviceEnumerator, ...)`.
pub unsafe fn create_enumerator(iid: &GUID, out: *mut *mut c_void) -> HRESULT {
    super::devnotify::start();
    let obj = EnumObj::create(&ENUM_VTBL, ());
    let hr = enumerator::qi(obj, iid, out);
    EnumObj::release(obj);
    hr
}

// ---------------------------------------------------------------------------
// IMMDeviceCollection
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IMMDeviceCollectionVtbl {
    qi: unsafe extern "win64" fn(*mut CollObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut CollObj) -> u32,
    release: unsafe extern "win64" fn(*mut CollObj) -> u32,
    get_count: unsafe extern "win64" fn(*mut CollObj, *mut u32) -> HRESULT,
    item: unsafe extern "win64" fn(*mut CollObj, u32, *mut *mut DeviceObj) -> HRESULT,
}

pub type CollObj = ComObj<IMMDeviceCollectionVtbl, Vec<Arc<Endpoint>>>;

mod collection {
    use super::*;
    crate::com_iunknown!(CollObj, [IID_IMMDeviceCollection]);

    pub unsafe extern "win64" fn get_count(this: *mut CollObj, n: *mut u32) -> HRESULT {
        write_out(n, CollObj::inner(this).len() as u32)
    }
    pub unsafe extern "win64" fn item(this: *mut CollObj, i: u32, out: *mut *mut DeviceObj) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        match CollObj::inner(this).get(i as usize) {
            Some(ep) => {
                std::ptr::write_unaligned(out, new_device(ep.clone()));
                S_OK
            }
            None => E_INVALIDARG,
        }
    }
}

static COLL_VTBL: IMMDeviceCollectionVtbl = IMMDeviceCollectionVtbl {
    qi: collection::qi,
    add_ref: collection::add_ref,
    release: collection::release,
    get_count: collection::get_count,
    item: collection::item,
};

// ---------------------------------------------------------------------------
// IMMDevice (+ IMMEndpoint on a secondary vtable)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IMMDeviceVtbl {
    qi: unsafe extern "win64" fn(*mut DeviceObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut DeviceObj) -> u32,
    release: unsafe extern "win64" fn(*mut DeviceObj) -> u32,
    activate: unsafe extern "win64" fn(*mut DeviceObj, *const GUID, u32, *const u8, *mut *mut c_void) -> HRESULT,
    open_property_store: unsafe extern "win64" fn(*mut DeviceObj, u32, *mut *mut c_void) -> HRESULT,
    get_id: unsafe extern "win64" fn(*mut DeviceObj, *mut *mut u16) -> HRESULT,
    get_state: unsafe extern "win64" fn(*mut DeviceObj, *mut u32) -> HRESULT,
}

#[repr(C)]
pub struct IMMEndpointVtbl {
    qi: unsafe extern "win64" fn(*mut *const IMMEndpointVtbl, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut *const IMMEndpointVtbl) -> u32,
    release: unsafe extern "win64" fn(*mut *const IMMEndpointVtbl) -> u32,
    get_data_flow: unsafe extern "win64" fn(*mut *const IMMEndpointVtbl, *mut u32) -> HRESULT,
}

#[repr(C)]
pub struct DeviceInner {
    endpoint_vtbl: *const IMMEndpointVtbl,
    pub ep: Arc<Endpoint>,
}
unsafe impl Send for DeviceInner {}
unsafe impl Sync for DeviceInner {}

pub type DeviceObj = ComObj<IMMDeviceVtbl, DeviceInner>;

const ENDPOINT_IFACE_OFFSET: usize = std::mem::offset_of!(DeviceObj, inner) + std::mem::offset_of!(DeviceInner, endpoint_vtbl);

pub fn new_device(ep: Arc<Endpoint>) -> *mut DeviceObj {
    DeviceObj::create(&DEVICE_VTBL, DeviceInner { endpoint_vtbl: &ENDPOINT_VTBL, ep })
}

mod device {
    use super::*;

    pub unsafe extern "win64" fn qi(this: *mut DeviceObj, riid: *const GUID, out: *mut *mut c_void) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        let Some(iid) = GUID::read(riid) else { return E_POINTER };
        if iid == IID_IUnknown || iid == IID_IMMDevice {
            DeviceObj::add_ref(this);
            std::ptr::write_unaligned(out, this as *mut c_void);
            S_OK
        } else if iid == IID_IMMEndpoint {
            DeviceObj::add_ref(this);
            std::ptr::write_unaligned(out, (this as *mut u8).add(ENDPOINT_IFACE_OFFSET) as *mut c_void);
            S_OK
        } else {
            log::debug!(target: "audio", "IMMDevice::QueryInterface({iid:?}) -> E_NOINTERFACE");
            E_NOINTERFACE
        }
    }
    pub unsafe extern "win64" fn add_ref(this: *mut DeviceObj) -> u32 {
        DeviceObj::add_ref(this)
    }
    pub unsafe extern "win64" fn release(this: *mut DeviceObj) -> u32 {
        DeviceObj::release(this)
    }

    pub unsafe extern "win64" fn activate(this: *mut DeviceObj, riid: *const GUID, ctx: u32, _params: *const u8, out: *mut *mut c_void) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        let Some(iid) = GUID::read(riid) else { return E_POINTER };
        let ep = DeviceObj::inner(this).ep.clone();
        if iid == IID_IAudioClient || iid == IID_IAudioClient2 || iid == IID_IAudioClient3 {
            log::debug!(target: "audio", "IMMDevice::Activate(IAudioClient {iid:?}, ctx {ctx:#x}) on '{}'", ep.info.name);
            std::ptr::write_unaligned(out, super::super::audioclient::new_client(ep) as *mut c_void);
            return S_OK;
        }
        if iid == IID_IAudioSessionManager || iid == IID_IAudioSessionManager2 {
            log::debug!(target: "audio", "IMMDevice::Activate(IAudioSessionManager2, ctx {ctx:#x}) on '{}'", ep.info.name);
            std::ptr::write_unaligned(out, super::new_session_manager(ep) as *mut c_void);
            return S_OK;
        }
        if iid == IID_ISpatialAudioClient {
            log::debug!(target: "audio", "IMMDevice::Activate(ISpatialAudioClient, ctx {ctx:#x}) on '{}'", ep.info.name);
            return super::super::spatial::create_spatial_audio_client(ep, out);
        }
        let what = if iid == IID_IAudioEndpointVolume {
            "IAudioEndpointVolume"
        } else {
            "unknown interface"
        };
        stub!("IMMDevice::Activate({iid:?} = {what}) -> E_NOINTERFACE");
        E_NOINTERFACE
    }

    pub unsafe extern "win64" fn open_property_store(this: *mut DeviceObj, access: u32, out: *mut *mut c_void) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        if access != 0 {
            // STGM_WRITE / STGM_READWRITE need admin rights on Windows too.
            return E_ACCESSDENIED;
        }
        std::ptr::write_unaligned(out, super::super::propsys::new_store(DeviceObj::inner(this).ep.clone()) as *mut c_void);
        S_OK
    }

    pub unsafe extern "win64" fn get_id(this: *mut DeviceObj, out: *mut *mut u16) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let p = co_wstr(&DeviceObj::inner(this).ep.id);
        std::ptr::write_unaligned(out, p);
        if p.is_null() {
            E_OUTOFMEMORY
        } else {
            S_OK
        }
    }

    pub unsafe extern "win64" fn get_state(this: *mut DeviceObj, out: *mut u32) -> HRESULT {
        let present = super::super::devnotify::endpoint_present(&DeviceObj::inner(this).ep.id);
        write_out(out, if present { DEVICE_STATE_ACTIVE } else { DEVICE_STATE_NOTPRESENT })
    }

    unsafe fn outer(this: *mut *const IMMEndpointVtbl) -> *mut DeviceObj {
        (this as *mut u8).sub(ENDPOINT_IFACE_OFFSET) as *mut DeviceObj
    }
    pub unsafe extern "win64" fn ep_qi(this: *mut *const IMMEndpointVtbl, riid: *const GUID, out: *mut *mut c_void) -> HRESULT {
        qi(outer(this), riid, out)
    }
    pub unsafe extern "win64" fn ep_add_ref(this: *mut *const IMMEndpointVtbl) -> u32 {
        DeviceObj::add_ref(outer(this))
    }
    pub unsafe extern "win64" fn ep_release(this: *mut *const IMMEndpointVtbl) -> u32 {
        DeviceObj::release(outer(this))
    }
    pub unsafe extern "win64" fn ep_get_data_flow(this: *mut *const IMMEndpointVtbl, out: *mut u32) -> HRESULT {
        write_out(out, DeviceObj::inner(outer(this)).ep.flow)
    }
}

const E_ACCESSDENIED: HRESULT = 0x8007_0005u32 as i32;

static DEVICE_VTBL: IMMDeviceVtbl = IMMDeviceVtbl {
    qi: device::qi,
    add_ref: device::add_ref,
    release: device::release,
    activate: device::activate,
    open_property_store: device::open_property_store,
    get_id: device::get_id,
    get_state: device::get_state,
};

static ENDPOINT_VTBL: IMMEndpointVtbl = IMMEndpointVtbl {
    qi: device::ep_qi,
    add_ref: device::ep_add_ref,
    release: device::ep_release,
    get_data_flow: device::ep_get_data_flow,
};

// ---------------------------------------------------------------------------
// IAudioSessionManager / IAudioSessionManager2 + IAudioSessionControl2 + ISimpleAudioVolume
// ---------------------------------------------------------------------------

struct SessionState {
    ep: Arc<Endpoint>,
    volume_bits: AtomicU32, // f32::to_bits (default 1.0)
    muted: std::sync::atomic::AtomicBool,
    display_name: Mutex<String>,
    icon_path: Mutex<String>,
    grouping: Mutex<GUID>,
}

impl SessionState {
    fn new(ep: Arc<Endpoint>) -> Arc<Self> {
        Arc::new(Self {
            ep,
            volume_bits: AtomicU32::new(1.0f32.to_bits()),
            muted: std::sync::atomic::AtomicBool::new(false),
            display_name: Mutex::new(String::new()),
            icon_path: Mutex::new(String::new()),
            grouping: Mutex::new(GUID::new(0, 0, 0, [0; 8])),
        })
    }
}

#[repr(C)]
struct IAudioSessionManager2Vtbl {
    qi: unsafe extern "win64" fn(*mut SessionManagerObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut SessionManagerObj) -> u32,
    release: unsafe extern "win64" fn(*mut SessionManagerObj) -> u32,
    get_audio_session_control: unsafe extern "win64" fn(*mut SessionManagerObj, *const GUID, u32, *mut *mut c_void) -> HRESULT,
    get_simple_audio_volume: unsafe extern "win64" fn(*mut SessionManagerObj, *const GUID, u32, *mut *mut c_void) -> HRESULT,
    get_session_enumerator: unsafe extern "win64" fn(*mut SessionManagerObj, *mut *mut c_void) -> HRESULT,
    register_session_notification: unsafe extern "win64" fn(*mut SessionManagerObj, *mut c_void) -> HRESULT,
    unregister_session_notification: unsafe extern "win64" fn(*mut SessionManagerObj, *mut c_void) -> HRESULT,
    register_duck_notification: unsafe extern "win64" fn(*mut SessionManagerObj, *const u16, *mut c_void) -> HRESULT,
    unregister_duck_notification: unsafe extern "win64" fn(*mut SessionManagerObj, *mut c_void) -> HRESULT,
}

type SessionManagerObj = ComObj<IAudioSessionManager2Vtbl, Arc<SessionState>>;

mod session_mgr {
    use super::*;
    com_iunknown!(SessionManagerObj, [IID_IAudioSessionManager, IID_IAudioSessionManager2]);

    pub unsafe extern "win64" fn get_audio_session_control(
        this: *mut SessionManagerObj,
        _guid: *const GUID,
        _flags: u32,
        out: *mut *mut c_void,
    ) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let st = SessionManagerObj::inner(this).clone();
        std::ptr::write_unaligned(out, SessionControlObj::create(&SESSION_CTRL_VTBL, st) as *mut c_void);
        S_OK
    }

    pub unsafe extern "win64" fn get_simple_audio_volume(
        this: *mut SessionManagerObj,
        _guid: *const GUID,
        _flags: u32,
        out: *mut *mut c_void,
    ) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let st = SessionManagerObj::inner(this).clone();
        std::ptr::write_unaligned(out, SimpleVolumeObj::create(&SIMPLE_VOL_VTBL, st) as *mut c_void);
        S_OK
    }

    pub unsafe extern "win64" fn get_session_enumerator(_this: *mut SessionManagerObj, out: *mut *mut c_void) -> HRESULT {
        if !out.is_null() {
            std::ptr::write_unaligned(out, std::ptr::null_mut());
        }
        E_NOTIMPL
    }

    pub unsafe extern "win64" fn ok1(_this: *mut SessionManagerObj, _a: *mut c_void) -> HRESULT {
        S_OK
    }

    pub unsafe extern "win64" fn ok2(_this: *mut SessionManagerObj, _a: *const u16, _b: *mut c_void) -> HRESULT {
        S_OK
    }
}

static SESSION_MGR_VTBL: IAudioSessionManager2Vtbl = IAudioSessionManager2Vtbl {
    qi: session_mgr::qi,
    add_ref: session_mgr::add_ref,
    release: session_mgr::release,
    get_audio_session_control: session_mgr::get_audio_session_control,
    get_simple_audio_volume: session_mgr::get_simple_audio_volume,
    get_session_enumerator: session_mgr::get_session_enumerator,
    register_session_notification: session_mgr::ok1,
    unregister_session_notification: session_mgr::ok1,
    register_duck_notification: session_mgr::ok2,
    unregister_duck_notification: session_mgr::ok1,
};

fn new_session_manager(ep: Arc<Endpoint>) -> *mut SessionManagerObj {
    SessionManagerObj::create(&SESSION_MGR_VTBL, SessionState::new(ep))
}

#[repr(C)]
struct IAudioSessionControl2Vtbl {
    qi: unsafe extern "win64" fn(*mut SessionControlObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut SessionControlObj) -> u32,
    release: unsafe extern "win64" fn(*mut SessionControlObj) -> u32,
    get_state: unsafe extern "win64" fn(*mut SessionControlObj, *mut u32) -> HRESULT,
    get_display_name: unsafe extern "win64" fn(*mut SessionControlObj, *mut *mut u16) -> HRESULT,
    set_display_name: unsafe extern "win64" fn(*mut SessionControlObj, *const u16, *const GUID) -> HRESULT,
    get_icon_path: unsafe extern "win64" fn(*mut SessionControlObj, *mut *mut u16) -> HRESULT,
    set_icon_path: unsafe extern "win64" fn(*mut SessionControlObj, *const u16, *const GUID) -> HRESULT,
    get_grouping_param: unsafe extern "win64" fn(*mut SessionControlObj, *mut GUID) -> HRESULT,
    set_grouping_param: unsafe extern "win64" fn(*mut SessionControlObj, *const GUID, *const GUID) -> HRESULT,
    register_notif: unsafe extern "win64" fn(*mut SessionControlObj, *mut c_void) -> HRESULT,
    unregister_notif: unsafe extern "win64" fn(*mut SessionControlObj, *mut c_void) -> HRESULT,
    get_session_id: unsafe extern "win64" fn(*mut SessionControlObj, *mut *mut u16) -> HRESULT,
    get_session_instance_id: unsafe extern "win64" fn(*mut SessionControlObj, *mut *mut u16) -> HRESULT,
    get_process_id: unsafe extern "win64" fn(*mut SessionControlObj, *mut u32) -> HRESULT,
    is_system_sounds: unsafe extern "win64" fn(*mut SessionControlObj) -> HRESULT,
    set_ducking_pref: unsafe extern "win64" fn(*mut SessionControlObj, BOOL) -> HRESULT,
}

type SessionControlObj = ComObj<IAudioSessionControl2Vtbl, Arc<SessionState>>;

mod session_ctrl {
    use super::*;
    com_iunknown!(SessionControlObj, [IID_IAudioSessionControl, IID_IAudioSessionControl2]);

    pub unsafe extern "win64" fn get_state(_this: *mut SessionControlObj, out: *mut u32) -> HRESULT {
        write_out(out, 1) // AudioSessionStateActive
    }
    pub unsafe extern "win64" fn get_display_name(this: *mut SessionControlObj, out: *mut *mut u16) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let s = SessionControlObj::inner(this).display_name.lock().unwrap().clone();
        std::ptr::write_unaligned(out, co_wstr(&s));
        S_OK
    }
    pub unsafe extern "win64" fn set_display_name(this: *mut SessionControlObj, val: *const u16, _ctx: *const GUID) -> HRESULT {
        let s = if val.is_null() { String::new() } else { crate::strings::from_wide(val) };
        *SessionControlObj::inner(this).display_name.lock().unwrap() = s;
        S_OK
    }
    pub unsafe extern "win64" fn get_icon_path(this: *mut SessionControlObj, out: *mut *mut u16) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let s = SessionControlObj::inner(this).icon_path.lock().unwrap().clone();
        std::ptr::write_unaligned(out, co_wstr(&s));
        S_OK
    }
    pub unsafe extern "win64" fn set_icon_path(this: *mut SessionControlObj, val: *const u16, _ctx: *const GUID) -> HRESULT {
        let s = if val.is_null() { String::new() } else { crate::strings::from_wide(val) };
        *SessionControlObj::inner(this).icon_path.lock().unwrap() = s;
        S_OK
    }
    pub unsafe extern "win64" fn get_grouping_param(this: *mut SessionControlObj, out: *mut GUID) -> HRESULT {
        write_out(out, *SessionControlObj::inner(this).grouping.lock().unwrap())
    }
    pub unsafe extern "win64" fn set_grouping_param(this: *mut SessionControlObj, val: *const GUID, _ctx: *const GUID) -> HRESULT {
        if let Some(g) = GUID::read(val) {
            *SessionControlObj::inner(this).grouping.lock().unwrap() = g;
        }
        S_OK
    }
    pub unsafe extern "win64" fn ok1(_this: *mut SessionControlObj, _a: *mut c_void) -> HRESULT {
        S_OK
    }
    pub unsafe extern "win64" fn get_session_id(this: *mut SessionControlObj, out: *mut *mut u16) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let id = format!("{}|session", SessionControlObj::inner(this).ep.id);
        std::ptr::write_unaligned(out, co_wstr(&id));
        S_OK
    }
    pub unsafe extern "win64" fn get_session_instance_id(this: *mut SessionControlObj, out: *mut *mut u16) -> HRESULT {
        if out.is_null() {
            return E_POINTER;
        }
        let id = format!("{}|session|{}", SessionControlObj::inner(this).ep.id, std::process::id());
        std::ptr::write_unaligned(out, co_wstr(&id));
        S_OK
    }
    pub unsafe extern "win64" fn get_process_id(_this: *mut SessionControlObj, out: *mut u32) -> HRESULT {
        write_out(out, std::process::id())
    }
    pub unsafe extern "win64" fn is_system_sounds(_this: *mut SessionControlObj) -> HRESULT {
        1 // S_FALSE
    }
    pub unsafe extern "win64" fn set_ducking_pref(_this: *mut SessionControlObj, _opt_out: BOOL) -> HRESULT {
        S_OK
    }
}

static SESSION_CTRL_VTBL: IAudioSessionControl2Vtbl = IAudioSessionControl2Vtbl {
    qi: session_ctrl::qi,
    add_ref: session_ctrl::add_ref,
    release: session_ctrl::release,
    get_state: session_ctrl::get_state,
    get_display_name: session_ctrl::get_display_name,
    set_display_name: session_ctrl::set_display_name,
    get_icon_path: session_ctrl::get_icon_path,
    set_icon_path: session_ctrl::set_icon_path,
    get_grouping_param: session_ctrl::get_grouping_param,
    set_grouping_param: session_ctrl::set_grouping_param,
    register_notif: session_ctrl::ok1,
    unregister_notif: session_ctrl::ok1,
    get_session_id: session_ctrl::get_session_id,
    get_session_instance_id: session_ctrl::get_session_instance_id,
    get_process_id: session_ctrl::get_process_id,
    is_system_sounds: session_ctrl::is_system_sounds,
    set_ducking_pref: session_ctrl::set_ducking_pref,
};

#[repr(C)]
struct ISimpleAudioVolumeVtbl {
    qi: unsafe extern "win64" fn(*mut SimpleVolumeObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut SimpleVolumeObj) -> u32,
    release: unsafe extern "win64" fn(*mut SimpleVolumeObj) -> u32,
    set_master_volume: unsafe extern "win64" fn(*mut SimpleVolumeObj, f32, *const GUID) -> HRESULT,
    get_master_volume: unsafe extern "win64" fn(*mut SimpleVolumeObj, *mut f32) -> HRESULT,
    set_mute: unsafe extern "win64" fn(*mut SimpleVolumeObj, BOOL, *const GUID) -> HRESULT,
    get_mute: unsafe extern "win64" fn(*mut SimpleVolumeObj, *mut BOOL) -> HRESULT,
}

type SimpleVolumeObj = ComObj<ISimpleAudioVolumeVtbl, Arc<SessionState>>;

mod simple_vol {
    use super::*;
    com_iunknown!(SimpleVolumeObj, [IID_ISimpleAudioVolume]);

    pub unsafe extern "win64" fn set_master_volume(this: *mut SimpleVolumeObj, level: f32, _ctx: *const GUID) -> HRESULT {
        if !(0.0..=1.0).contains(&level) {
            return E_INVALIDARG;
        }
        SimpleVolumeObj::inner(this).volume_bits.store(level.to_bits(), Ordering::Relaxed);
        S_OK
    }
    pub unsafe extern "win64" fn get_master_volume(this: *mut SimpleVolumeObj, out: *mut f32) -> HRESULT {
        let v = f32::from_bits(SimpleVolumeObj::inner(this).volume_bits.load(Ordering::Relaxed));
        write_out(out, v)
    }
    pub unsafe extern "win64" fn set_mute(this: *mut SimpleVolumeObj, mute: BOOL, _ctx: *const GUID) -> HRESULT {
        SimpleVolumeObj::inner(this).muted.store(mute != 0, Ordering::Relaxed);
        S_OK
    }
    pub unsafe extern "win64" fn get_mute(this: *mut SimpleVolumeObj, out: *mut BOOL) -> HRESULT {
        let m = SimpleVolumeObj::inner(this).muted.load(Ordering::Relaxed);
        write_out(out, if m { TRUE } else { FALSE })
    }
}

static SIMPLE_VOL_VTBL: ISimpleAudioVolumeVtbl = ISimpleAudioVolumeVtbl {
    qi: simple_vol::qi,
    add_ref: simple_vol::add_ref,
    release: simple_vol::release,
    set_master_volume: simple_vol::set_master_volume,
    get_master_volume: simple_vol::get_master_volume,
    set_mute: simple_vol::set_mute,
    get_mute: simple_vol::get_mute,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoint_ids_are_stable_and_windows_shaped() {
        let a = Endpoint::new(backend::null_device());
        let b = Endpoint::new(backend::null_device());
        assert_eq!(a.id, b.id);
        assert!(a.id.starts_with("{0.0.0.00000000}.{") && a.id.ends_with('}'), "{}", a.id);
        assert_eq!(a.id.len(), "{0.0.0.00000000}.".len() + 38);
        assert_ne!(a.guid, a.container_id);
        assert_eq!(a.flow, eRender);
        assert_eq!(a.mix_format().channels, 2);

        let c = Endpoint::new_capture(backend::null_capture_device());
        assert!(c.id.starts_with("{0.0.1.00000000}.{") && c.id.ends_with('}'), "{}", c.id);
        assert_eq!(c.id.len(), "{0.0.1.00000000}.".len() + 38);
        assert_eq!(c.flow, eCapture);
        assert_eq!(c.mix_format().rate, 48000);
        assert_eq!(c.mix_format().channels, 2);

        let ds = Endpoint::new(backend::dualsense_usb_device(0));
        assert_eq!(ds.container_id, dualsense_usb_container_id(0));
        assert_eq!(format_guid(&ds.container_id).len(), 38);
        assert_eq!(ds.mix_format().channels, 4);
    }
}
