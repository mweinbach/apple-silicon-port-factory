//! Device-change notifications: `IMMNotificationClient` delivery and
//! device-loss invalidation.
//!
//! CoreAudio listeners ([`backend::watch_device_changes`]: device list,
//! default output device, per-device "alive") only post a rescan request.
//! One Win32 thread ("MMDevAPI notification thread", with a TEB, like the
//! worker thread Windows' MMDevAPI calls clients on) debounces, rescans the
//! host devices, diffs them against the previous snapshot and
//!
//! 1. invalidates the WASAPI streams of removed endpoints (and of endpoints
//!    whose shared-mode mix format changed): their client calls return
//!    `AUDCLNT_E_DEVICE_INVALIDATED` from then on
//!    ([`super::audioclient::invalidate_endpoint`]);
//! 2. calls every registered `IMMNotificationClient`, in Windows order:
//!    `OnDeviceAdded` + `OnDeviceStateChanged(ACTIVE)` for new endpoints,
//!    `OnDeviceStateChanged(NOTPRESENT)` + `OnDeviceRemoved` for gone ones,
//!    `OnPropertyValueChanged(PKEY_AudioEngine_DeviceFormat)` for a mix-format
//!    change, then `OnDefaultDeviceChanged(eRender, role, id)` for eConsole,
//!    eMultimedia and eCommunications when the default render endpoint
//!    changed (id NULL when there is none).
//!
//! Wwise 2022.1.4's client (vtable 0x14268dd50, owner `FUN_1415ecaf0`) queues
//! `(id, kind)` for its audio thread (`FUN_1415ec9d0`): StateChanged -> 2
//! (state ignored), Added -> 0, Removed -> 1, DefaultDeviceChanged(eRender)
//! eMultimedia/eConsole/eCommunications -> 3/4/5, PropertyValueChanged for
//! DeviceFormat and three other endpoint keys -> 2. A NULL id is ignored.
//!
//! Default-device changes don't invalidate anything (as on Windows): a
//! stream on the old default stays valid (its DefaultOutput unit even
//! follows the switch); clients that want to move re-create their stream,
//! which `GetDefaultAudioEndpoint` then opens on the new default.

use super::backend::{self, DeviceInfo};
use super::com::GUID;
use super::mmdevapi::{self, Endpoint, NOTIFY_CLIENTS};
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Mutex, OnceLock};

pub const eConsole: u32 = 0;
pub const eMultimedia: u32 = 1;
pub const eCommunications: u32 = 2;

/// `PROPERTYKEY`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropertyKey {
    pub fmtid: GUID,
    pub pid: u32,
}

/// PKEY_AudioEngine_DeviceFormat {f19f064d-082c-4e27-bc73-6882a1bb8e4c},0.
pub const PKEY_AudioEngine_DeviceFormat: PropertyKey =
    PropertyKey { fmtid: GUID::new(0xf19f_064d, 0x082c, 0x4e27, [0xbc, 0x73, 0x68, 0x82, 0xa1, 0xbb, 0x8e, 0x4c]), pid: 0 };

/// One endpoint as far as notifications are concerned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpState {
    pub id: String,
    /// Shared-mode mix format (rate, channels) the clients see.
    pub mix: (u32, u16),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    pub endpoints: Vec<EpState>,
    /// Default render endpoint (= the first device, see `backend::output_devices`).
    pub default: Option<String>,
}

impl Snapshot {
    pub fn of(devs: &[DeviceInfo]) -> Snapshot {
        let endpoints: Vec<EpState> = devs
            .iter()
            .map(|d| {
                let e = Endpoint::new(d.clone());
                EpState { mix: (e.mix_rate(), e.mix_channels()), id: e.id }
            })
            .collect();
        let default = endpoints.first().map(|e| e.id.clone());
        Snapshot { endpoints, default }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Note {
    Added(String),
    Removed(String),
    State(String, u32),
    FormatChanged(String),
    Default(u32, Option<String>),
}

/// Notifications (in delivery order) for the change `old` -> `new`.
pub fn diff(old: &Snapshot, new: &Snapshot) -> Vec<Note> {
    let mut v = Vec::new();
    for e in &new.endpoints {
        if !old.endpoints.iter().any(|o| o.id == e.id) {
            v.push(Note::Added(e.id.clone()));
            v.push(Note::State(e.id.clone(), mmdevapi::DEVICE_STATE_ACTIVE));
        }
    }
    for o in &old.endpoints {
        if !new.endpoints.iter().any(|e| e.id == o.id) {
            v.push(Note::State(o.id.clone(), mmdevapi::DEVICE_STATE_NOTPRESENT));
            v.push(Note::Removed(o.id.clone()));
        }
    }
    for e in &new.endpoints {
        if old.endpoints.iter().any(|o| o.id == e.id && o.mix != e.mix) {
            v.push(Note::FormatChanged(e.id.clone()));
        }
    }
    if old.default != new.default {
        for role in [eConsole, eMultimedia, eCommunications] {
            v.push(Note::Default(role, new.default.clone()));
        }
    }
    v
}

/// Endpoints that disappeared (and have not come back).
static REMOVED: Mutex<Option<HashSet<String>>> = Mutex::new(None);

/// False once endpoint `id` was removed (until it reappears). Unknown ids
/// count as present.
pub fn endpoint_present(id: &str) -> bool {
    REMOVED.lock().unwrap().as_ref().is_none_or(|s| !s.contains(id))
}

#[cfg(test)]
pub(crate) fn clear_removed_for_test() {
    if let Some(s) = REMOVED.lock().unwrap().as_mut() {
        s.clear();
    }
}

// ---------------------------------------------------------------------------
// Delivery to guest clients
// ---------------------------------------------------------------------------

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Vtable slot `i` of a guest COM object.
unsafe fn slot(obj: usize, i: usize) -> usize {
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    std::ptr::read_unaligned(vtbl.add(i))
}

unsafe fn deliver_to(cb: usize, n: &Note) {
    match n {
        Note::State(id, st) => {
            let w = wide(id);
            crate::call_guest8(slot(cb, 3), cb as u64, w.as_ptr() as u64, *st as u64, 0, 0, 0, 0, 0);
        }
        Note::Added(id) => {
            let w = wide(id);
            crate::call_guest8(slot(cb, 4), cb as u64, w.as_ptr() as u64, 0, 0, 0, 0, 0, 0);
        }
        Note::Removed(id) => {
            let w = wide(id);
            crate::call_guest8(slot(cb, 5), cb as u64, w.as_ptr() as u64, 0, 0, 0, 0, 0, 0);
        }
        Note::Default(role, id) => {
            let w = id.as_deref().map(wide);
            let p = w.as_ref().map_or(std::ptr::null(), |w| w.as_ptr());
            crate::call_guest8(slot(cb, 6), cb as u64, mmdevapi::eRender as u64, *role as u64, p as u64, 0, 0, 0, 0);
        }
        Note::FormatChanged(id) => {
            let w = wide(id);
            let key = PKEY_AudioEngine_DeviceFormat;
            // PROPERTYKEY (20 bytes) is passed by reference in the Win64 ABI.
            crate::call_guest8(slot(cb, 7), cb as u64, w.as_ptr() as u64, (&key as *const PropertyKey) as u64, 0, 0, 0, 0, 0);
        }
    }
}

/// Call every registered client (each AddRef'd for the duration, so an
/// `UnregisterEndpointNotificationCallback` from inside a callback is safe).
fn deliver(n: &Note) {
    type Unk = unsafe extern "win64" fn(usize) -> u32;
    let clients: Vec<usize> = {
        let g = NOTIFY_CLIENTS.lock().unwrap();
        for &c in g.iter() {
            unsafe { std::mem::transmute::<usize, Unk>(slot(c, 1))(c) };
        }
        g.clone()
    };
    log::info!(target: "audio", "IMMNotificationClient x{}: {n:?}", clients.len());
    for c in clients {
        unsafe {
            deliver_to(c, n);
            std::mem::transmute::<usize, Unk>(slot(c, 2))(c);
        }
    }
}

// ---------------------------------------------------------------------------
// Dispatcher thread
// ---------------------------------------------------------------------------

enum Msg {
    /// A CoreAudio listener fired: rescan the host devices.
    Rescan,
    /// Process this device list (tests).
    Devices(Vec<DeviceInfo>),
    /// Reply once everything queued before has been delivered.
    Flush(mpsc::SyncSender<()>),
}

/// Debounce for bursts of HAL notifications (a hot-plug fires dev#, dOut and
/// livn within a few ms).
const DEBOUNCE: std::time::Duration = std::time::Duration::from_millis(100);

fn process(state: &mut Snapshot, devs: &[DeviceInfo]) {
    let new = Snapshot::of(devs);
    let notes = diff(state, &new);
    *state = new;
    if notes.is_empty() {
        return;
    }
    {
        let mut g = REMOVED.lock().unwrap();
        let set = g.get_or_insert_with(HashSet::new);
        for n in &notes {
            match n {
                Note::Added(id) => {
                    set.remove(id);
                }
                Note::Removed(id) => {
                    set.insert(id.clone());
                }
                _ => {}
            }
        }
    }
    for n in &notes {
        match n {
            Note::Removed(id) => {
                super::audioclient::invalidate_endpoint(id, "device removed");
            }
            Note::FormatChanged(id) => {
                super::audioclient::invalidate_endpoint(id, "mix format changed");
            }
            _ => {}
        }
    }
    for n in &notes {
        deliver(n);
    }
}

static DISPATCH_TX: OnceLock<Mutex<mpsc::Sender<Msg>>> = OnceLock::new();

fn dispatcher() -> &'static Mutex<mpsc::Sender<Msg>> {
    DISPATCH_TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<Msg>();
        let initial = Snapshot::of(&mmdevapi::endpoints().iter().map(|e| e.info.clone()).collect::<Vec<_>>());
        let body = Box::new(move || {
            let mut state = initial;
            let mut queue: VecDeque<Msg> = VecDeque::new();
            loop {
                let msg = match queue.pop_front() {
                    Some(m) => m,
                    None => match rx.recv() {
                        Ok(m) => m,
                        Err(_) => return 0,
                    },
                };
                match msg {
                    Msg::Rescan => {
                        std::thread::sleep(DEBOUNCE);
                        while let Ok(m) = rx.try_recv() {
                            if !matches!(m, Msg::Rescan) {
                                queue.push_back(m);
                            }
                        }
                        let devs = backend::output_devices();
                        mmdevapi::refresh_with(devs.clone());
                        process(&mut state, &devs);
                    }
                    Msg::Devices(devs) => process(&mut state, &devs),
                    Msg::Flush(done) => {
                        let _ = done.send(());
                    }
                }
            }
        });
        match crate::thread::spawn(body, 256 * 1024, false, false) {
            Ok(t) => {
                *t.name.lock().unwrap() = "MMDevAPI notification thread".into();
                log::debug!(target: "audio", "device notification thread tid 0x{:x}", t.tid.load(Ordering::Relaxed));
            }
            Err(e) => log::error!(target: "audio", "cannot spawn the device notification thread: {e}"),
        }
        Mutex::new(tx)
    })
}

fn post(m: Msg) {
    let _ = dispatcher().lock().unwrap().send(m);
}

/// CoreAudio listener hook (HAL notification thread). Never blocks inside
/// `OnceLock::get_or_init` if a HAL notification arrives during startup.
fn on_host_change() {
    if let Some(tx) = DISPATCH_TX.get() {
        let _ = tx.lock().unwrap().send(Msg::Rescan);
    }
}

static WATCHING: AtomicBool = AtomicBool::new(false);

/// Start the dispatcher and (CoreAudio backend) the host device listeners.
/// Idempotent; called when an enumerator is created or a client registers.
/// `AW2_AUDIO_DEVICE_EVENTS=0` disables the listeners.
pub fn start() {
    dispatcher();
    static KNOB: std::sync::Once = std::sync::Once::new();
    KNOB.call_once(simulate_unplug_knob);
    if WATCHING.load(Ordering::Acquire) || std::env::var("AW2_AUDIO_DEVICE_EVENTS").is_ok_and(|v| v == "0") {
        return;
    }
    static ONE: Mutex<()> = Mutex::new(());
    let _g = ONE.lock().unwrap();
    if WATCHING.load(Ordering::Acquire) {
        return;
    }
    match backend::watch_device_changes(on_host_change) {
        Ok(()) => {
            WATCHING.store(true, Ordering::Release);
            log::info!(target: "audio", "watching CoreAudio device changes (device list, default output, alive)");
        }
        Err(e) => log::debug!(target: "audio", "device-change events off: {e}"),
    }
}

/// `AW2_AUDIO_SIMULATE_UNPLUG=<secs>[,<replug secs>]`: `secs` after the
/// first enumerator, process the host device list without the default
/// endpoint (its streams are invalidated, clients get StateChanged/Removed
/// and DefaultDeviceChanged), then after `replug secs` (default 2) rescan
/// for real (it comes back: Added + DefaultDeviceChanged). Exercises the
/// client's device-lost path in a real run without touching hardware.
fn simulate_unplug_knob() {
    let Ok(spec) = std::env::var("AW2_AUDIO_SIMULATE_UNPLUG") else { return };
    let mut it = spec.split(',').map(|s| s.trim().parse::<f64>());
    let (Some(Ok(at)), replug) = (it.next(), it.next()) else {
        log::warn!(target: "audio", "AW2_AUDIO_SIMULATE_UNPLUG={spec:?}: expected <secs>[,<replug secs>]");
        return;
    };
    let replug = replug.and_then(|r| r.ok()).unwrap_or(2.0);
    let _ = std::thread::Builder::new().name("wwise-audio-unplug-sim".into()).spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs_f64(at.max(0.0)));
        let mut devs = backend::output_devices();
        let gone = devs.remove(0);
        log::warn!(target: "audio", "AW2_AUDIO_SIMULATE_UNPLUG: '{}' unplugged (simulated)", gone.name);
        post(Msg::Devices(devs));
        std::thread::sleep(std::time::Duration::from_secs_f64(replug.max(0.0)));
        log::warn!(target: "audio", "AW2_AUDIO_SIMULATE_UNPLUG: '{}' plugged back (rescan)", gone.name);
        post(Msg::Rescan);
    });
}

/// Wait until every queued change has been processed and delivered.
pub fn flush() {
    let (tx, rx) = mpsc::sync_channel(1);
    post(Msg::Flush(tx));
    let _ = rx.recv_timeout(std::time::Duration::from_secs(10));
}

/// Process `devs` (default first) as if the host device list had changed
/// to it, and wait for delivery. Tests only; real changes come from CoreAudio.
pub fn simulate(devs: Vec<DeviceInfo>) {
    post(Msg::Devices(devs));
    flush();
}

/// Ask for a host rescan (as a CoreAudio listener would) and wait for it.
pub fn rescan() {
    post(Msg::Rescan);
    flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dev(uid: &str, rate: u32, def: bool) -> DeviceInfo {
        DeviceInfo { id: 0, uid: uid.into(), name: uid.into(), rate, channels: 2, io_frames: 512, latency_frames: 0, headphones: false, is_default: def }
    }

    #[test]
    fn diff_orders_like_windows() {
        let a = dev("a", 48000, true);
        let b = dev("b", 44100, false);
        let ida = Endpoint::new(a.clone()).id;
        let idb = Endpoint::new(b.clone()).id;
        let s0 = Snapshot::of(&[a.clone()]);
        let s1 = Snapshot::of(&[a.clone(), b.clone()]);
        assert_eq!(diff(&s0, &s0), vec![]);
        assert_eq!(diff(&s0, &s1), vec![Note::Added(idb.clone()), Note::State(idb.clone(), mmdevapi::DEVICE_STATE_ACTIVE)]);
        // b becomes the default, a is unplugged.
        let s2 = Snapshot::of(&[DeviceInfo { is_default: true, ..b.clone() }]);
        assert_eq!(
            diff(&s1, &s2),
            vec![
                Note::State(ida.clone(), mmdevapi::DEVICE_STATE_NOTPRESENT),
                Note::Removed(ida.clone()),
                Note::Default(eConsole, Some(idb.clone())),
                Note::Default(eMultimedia, Some(idb.clone())),
                Note::Default(eCommunications, Some(idb.clone())),
            ]
        );
        // Last device gone: default becomes NULL.
        let d = diff(&s2, &Snapshot::default());
        assert_eq!(d.last(), Some(&Note::Default(eCommunications, None)));
    }

    #[test]
    fn property_key_layout_matches_wwise_check() {
        // FUN_1415ed530 compares the key as two u64s + pid.
        let k = PKEY_AudioEngine_DeviceFormat;
        let b: [u8; 20] = unsafe { std::mem::transmute(k) };
        assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 0x4e27_082c_f19f_064d);
        assert_eq!(u64::from_le_bytes(b[8..16].try_into().unwrap()), 0x4c8e_bba1_8268_73bc);
        assert_eq!(u32::from_le_bytes(b[16..20].try_into().unwrap()), 0);
    }
}
