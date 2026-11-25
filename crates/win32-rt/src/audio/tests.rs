//! Vtable-level tests: drive the audio COM objects exactly like the game
//! does (raw `extern "win64"` calls through vtable slots, Windows SDK slot
//! order), replaying the Wwise 2022.1.4 `AkSinkCoreAudio` init + render
//! sequence decompiled from AlanWake2.exe (0x1415ecaf0, 0x141652ce0,
//! 0x1416538d0, 0x141653fc0, 0x141653de0, 0x141654310).
//!
//! These run on the null backend (no host audio). `audible_tone_on_default_device`
//! is `#[ignore]`d and plays a tone through CoreAudio.

use super::backend::{self, Backend};
use super::com::*;
use super::format::{WAVEFORMATEX, WAVEFORMATEXTENSIBLE};
use super::propsys::*;
use super::tap;
use crate::misc::ole32::{CoCreateInstance, CoTaskMemFree, PropVariantClear};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

type P = *mut c_void;

unsafe fn slot(obj: P, i: usize) -> usize {
    *(*(obj as *const *const usize)).add(i)
}

/// `call!(obj, slot, (ArgTy, ...) -> Ret; args...)`
macro_rules! call {
    ($obj:expr, $i:expr, ($($t:ty),*) -> $r:ty; $($a:expr),*) => {{
        let o: P = $obj as P;
        let f: unsafe extern "win64" fn(P $(, $t)*) -> $r = std::mem::transmute(slot(o, $i));
        f(o $(, $a)*)
    }};
}

/// Call a COM method with signature `(this, f32, *const GUID) -> HRESULT` through
/// its real `win64_bridge!` vtable slot (Win64 passes `f32` in `xmm1`, `ctx` in `r8`).
unsafe fn call_bridge_f32_guid(obj: P, idx: usize, level: f32, ctx: *const GUID) -> HRESULT {
    #[cfg(target_arch = "aarch64")]
    {
        let br: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) -> HRESULT = std::mem::transmute(slot(obj, idx));
        br(
            obj as u64,
            0,
            ctx as u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            f64::from_bits(level.to_bits() as u64),
            0.0,
            0.0,
        )
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        call!(obj, idx, (f32, *const GUID) -> HRESULT; level, ctx)
    }
}

unsafe fn release(o: P) -> u32 {
    call!(o, 2, () -> u32;)
}
unsafe fn add_ref(o: P) -> u32 {
    call!(o, 1, () -> u32;)
}
unsafe fn qi(o: P, iid: &GUID) -> (HRESULT, P) {
    let mut out: P = std::ptr::null_mut();
    let hr = call!(o, 0, (*const GUID, *mut P) -> HRESULT; iid, &mut out);
    (hr, out)
}

// IMMDeviceEnumerator
const EN_ENUM: usize = 3;
const EN_DEFAULT: usize = 4;
const EN_GETDEVICE: usize = 5;
const EN_REGISTER: usize = 6;
const EN_UNREGISTER: usize = 7;
// IMMDeviceCollection
const COLL_COUNT: usize = 3;
const COLL_ITEM: usize = 4;
// IMMDevice
const DEV_ACTIVATE: usize = 3;
const DEV_OPENSTORE: usize = 4;
const DEV_GETID: usize = 5;
const DEV_GETSTATE: usize = 6;
// IPropertyStore
const PS_COUNT: usize = 3;
const PS_GETVALUE: usize = 5;
const PS_SETVALUE: usize = 6;
// IAudioClient / 2
const AC_INIT: usize = 3;
const AC_BUFSIZE: usize = 4;
const AC_LATENCY: usize = 5;
const AC_PADDING: usize = 6;
const AC_ISFMT: usize = 7;
const AC_MIXFMT: usize = 8;
const AC_PERIOD: usize = 9;
const AC_START: usize = 10;
const AC_STOP: usize = 11;
const AC_RESET: usize = 12;
const AC_SETEVENT: usize = 13;
const AC_SERVICE: usize = 14;
const AC2_OFFLOAD: usize = 15;
const AC2_PROPS: usize = 16;
// IAudioRenderClient
const RC_GET: usize = 3;
const RC_RELEASE: usize = 4;
// IAudioCaptureClient
const CC_GET: usize = 3;
const CC_RELEASE: usize = 4;
const CC_NEXT: usize = 5;
// IAudioClock
const CK_FREQ: usize = 3;
const CK_POS: usize = 4;

const E_RENDER: u32 = 0;
const E_CAPTURE: u32 = 1;
const E_ALL: u32 = 2;
const E_CONSOLE: u32 = 0;
const DEVICE_STATEMASK_ALL: u32 = 0xF;
const CLSCTX_ALL: u32 = 0x17;
const STGM_READ: u32 = 0;
const VT_EMPTY: u16 = 0;
const VT_UI4: u16 = 19;
const VT_LPWSTR: u16 = 31;
const VT_BLOB: u16 = 65;
const VT_CLSID: u16 = 72;
const SHARED: u32 = 0;
const EVENTCALLBACK: u32 = 0x0004_0000;
const AUTOCONVERTPCM: u32 = 0x8000_0000;
const SRC_DEFAULT_QUALITY: u32 = 0x0800_0000;
const SILENT: u32 = 2;

struct Fixture {
    _g: super::AudioTestGuard,
}
impl Fixture {
    fn null() -> Fixture {
        let g = super::audio_test_lock();
        super::mmdevapi::reset_endpoint_software_gains_for_test();
        super::mmdevapi::reset_sessions_for_test();
        backend::force_backend(Some(Backend::Null));
        super::mmdevapi::refresh();
        super::mmdevapi::refresh_capture();
        Fixture { _g: g }
    }
}

unsafe fn enumerator() -> P {
    let mut e: P = std::ptr::null_mut();
    let hr = CoCreateInstance(
        &CLSID_MMDeviceEnumerator as *const GUID as *const u8,
        std::ptr::null_mut(),
        CLSCTX_ALL,
        &IID_IMMDeviceEnumerator as *const GUID as *const u8,
        &mut e,
    );
    assert_eq!(hr, S_OK);
    assert!(!e.is_null());
    e
}

unsafe fn default_device(e: P) -> P {
    let mut d: P = std::ptr::null_mut();
    assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_RENDER, E_CONSOLE, &mut d), S_OK);
    d
}

unsafe fn activate_client(d: P) -> P {
    let mut c: P = std::ptr::null_mut();
    let hr = call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut c);
    assert_eq!(hr, S_OK);
    c
}

/// Wait until a started render `IAudioClient` finishes playing all queued frames (`GetCurrentPadding == 0`).
unsafe fn wait_render_drained(render_c: P) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        let mut pad = 1u32;
        assert_eq!(call!(render_c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), S_OK);
        if pad == 0 {
            // Brief margin so the playback consumer thread finishes `push_loopback_frames`
            // after updating the ring read pointer.
            std::thread::sleep(std::time::Duration::from_millis(5));
            return;
        }
        assert!(std::time::Instant::now() < deadline, "timed out waiting for render client to drain (pad={pad})");
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

/// Drain loopback capture packets until `want_frames` non-silent f32 frames are collected.
unsafe fn wait_loopback_live_f32(cc: P, want_frames: u32, channels: usize) -> Vec<f32> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    let mut out = Vec::with_capacity(want_frames as usize * channels);
    while (out.len() / channels) < want_frames as usize {
        let mut cap_ptr: *mut u8 = std::ptr::null_mut();
        let mut cap_frames = 0u32;
        let mut cap_flags = 0u32;
        let hr = call!(
            cc,
            CC_GET,
            (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT;
            &mut cap_ptr, &mut cap_frames, &mut cap_flags, std::ptr::null_mut(), std::ptr::null_mut()
        );
        if hr == S_OK {
            if cap_flags & SILENT == 0 && cap_frames > 0 {
                let s = std::slice::from_raw_parts(cap_ptr as *const f32, cap_frames as usize * channels);
                out.extend_from_slice(s);
            }
            assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; cap_frames), S_OK);
            continue;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for {want_frames} live loopback frames (got {})",
            out.len() / channels
        );
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    out
}

/// Drain all currently queued packets on `cc` and assert none contain non-silent audio.
unsafe fn assert_no_live_loopback_audio(cc: P, msg: &str) {
    loop {
        let mut cap_ptr: *mut u8 = std::ptr::null_mut();
        let mut cap_frames = 0u32;
        let mut cap_flags = 0u32;
        let hr = call!(
            cc,
            CC_GET,
            (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT;
            &mut cap_ptr, &mut cap_frames, &mut cap_flags, std::ptr::null_mut(), std::ptr::null_mut()
        );
        if hr != S_OK {
            break;
        }
        assert_ne!(cap_flags & SILENT, 0, "{msg}: received unexpected non-silent packet ({cap_frames} frames)");
        assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; cap_frames), S_OK);
    }
}

unsafe fn wide(p: *const u16) -> String {
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    String::from_utf16_lossy(std::slice::from_raw_parts(p, n))
}

// --- fake IMMNotificationClient (IUnknown + no-op notifications) ---------

static NOTIFY_REFS: AtomicU32 = AtomicU32::new(1);
unsafe extern "win64" fn n_qi(_t: P, _i: *const GUID, o: *mut P) -> HRESULT {
    *o = std::ptr::null_mut();
    E_NOINTERFACE
}
unsafe extern "win64" fn n_add(_t: P) -> u32 {
    NOTIFY_REFS.fetch_add(1, Ordering::SeqCst) + 1
}
unsafe extern "win64" fn n_rel(_t: P) -> u32 {
    NOTIFY_REFS.fetch_sub(1, Ordering::SeqCst) - 1
}
unsafe extern "win64" fn n_ignore(_t: P) -> HRESULT {
    S_OK
}
#[repr(C)]
struct NotifyVtbl([usize; 8]);
#[repr(C)]
struct Notify {
    vtbl: *const NotifyVtbl,
}

#[test]
fn enumerator_devices_and_property_store() {
    let _f = Fixture::null();
    unsafe {
        // CoCreateInstance edge cases.
        let mut o: P = 1 as P;
        let hr = CoCreateInstance(
            &CLSID_MMDeviceEnumerator as *const GUID as *const u8,
            8 as P,
            CLSCTX_ALL,
            &IID_IMMDeviceEnumerator as *const GUID as *const u8,
            &mut o,
        );
        assert_eq!(hr, CLASS_E_NOAGGREGATION);
        assert!(o.is_null());

        let e = enumerator();
        let (hr, unk) = qi(e, &IID_IUnknown);
        assert_eq!(hr, S_OK);
        assert_eq!(unk, e);
        assert_eq!(release(unk), 1);
        assert_eq!(qi(e, &IID_IAudioClient).0, E_NOINTERFACE);

        // Register/Unregister keep a reference on the guest callback.
        NOTIFY_REFS.store(1, Ordering::SeqCst);
        let ig = n_ignore as *const () as usize;
        let vt = NotifyVtbl([n_qi as *const () as usize, n_add as *const () as usize, n_rel as *const () as usize, ig, ig, ig, ig, ig]);
        let n = Notify { vtbl: &vt };
        let np = &n as *const Notify as P;
        assert_eq!(call!(e, EN_REGISTER, (P) -> HRESULT; np), S_OK);
        assert_eq!(NOTIFY_REFS.load(Ordering::SeqCst), 2);
        assert_eq!(call!(e, EN_UNREGISTER, (P) -> HRESULT; np), S_OK);
        assert_eq!(NOTIFY_REFS.load(Ordering::SeqCst), 1);
        assert_eq!(call!(e, EN_UNREGISTER, (P) -> HRESULT; np), E_NOTFOUND);

        // EnumAudioEndpoints(eRender, DEVICE_STATEMASK_ALL), like 0x1415ecaf0.
        let mut coll: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_ENUM, (u32, u32, *mut P) -> HRESULT; E_RENDER, DEVICE_STATEMASK_ALL, &mut coll), S_OK);
        let mut count = 0u32;
        assert_eq!(call!(coll, COLL_COUNT, (*mut u32) -> HRESULT; &mut count), S_OK);
        assert_eq!(count, 1, "null backend exposes exactly one endpoint");
        let mut d0: P = std::ptr::null_mut();
        assert_eq!(call!(coll, COLL_ITEM, (u32, *mut P) -> HRESULT; 0, &mut d0), S_OK);
        let mut bad: P = std::ptr::null_mut();
        assert_eq!(call!(coll, COLL_ITEM, (u32, *mut P) -> HRESULT; 1, &mut bad), E_INVALIDARG);
        // Capture endpoints are exposed honestly (null fallback = "Default Microphone").
        let mut cap: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_ENUM, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, DEVICE_STATEMASK_ALL, &mut cap), S_OK);
        let mut ccount = 0u32;
        assert_eq!(call!(cap, COLL_COUNT, (*mut u32) -> HRESULT; &mut ccount), S_OK);
        assert_eq!(ccount, 1);
        release(cap);
        // eAll includes both render and capture endpoints.
        let mut all: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_ENUM, (u32, u32, *mut P) -> HRESULT; E_ALL, DEVICE_STATEMASK_ALL, &mut all), S_OK);
        let mut acount = 0u32;
        assert_eq!(call!(all, COLL_COUNT, (*mut u32) -> HRESULT; &mut acount), S_OK);
        assert_eq!(acount, 2);
        release(all);
        // GetDefaultAudioEndpoint(eAll) is invalid per Windows MMDevAPI spec.
        let mut bad_def: P = 1 as P;
        assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_ALL, E_CONSOLE, &mut bad_def), E_INVALIDARG);
        assert!(bad_def.is_null());

        let mut id: *mut u16 = std::ptr::null_mut();
        assert_eq!(call!(d0, DEV_GETID, (*mut *mut u16) -> HRESULT; &mut id), S_OK);
        let ids = wide(id);
        assert!(ids.starts_with("{0.0.0.00000000}.{"), "{ids}");
        let mut state = 0u32;
        assert_eq!(call!(d0, DEV_GETSTATE, (*mut u32) -> HRESULT; &mut state), S_OK);
        assert_eq!(state, 1, "DEVICE_STATE_ACTIVE");

        // GetDevice(id) round-trips; default device has the same id.
        let mut d1: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_GETDEVICE, (*const u16, *mut P) -> HRESULT; id, &mut d1), S_OK);
        let d = default_device(e);
        let mut id2: *mut u16 = std::ptr::null_mut();
        call!(d, DEV_GETID, (*mut *mut u16) -> HRESULT; &mut id2);
        assert_eq!(wide(id2), ids);
        CoTaskMemFree(id as P);
        CoTaskMemFree(id2 as P);
        // IMMEndpoint via QI.
        let (hr, ep) = qi(d, &IID_IMMEndpoint);
        assert_eq!(hr, S_OK);
        let mut flow = 9u32;
        assert_eq!(call!(ep, 3, (*mut u32) -> HRESULT; &mut flow), S_OK);
        assert_eq!(flow, E_RENDER);
        release(ep);

        // Property store: Wwise needs DeviceFormat as a VT_BLOB WAVEFORMATEX(TENSIBLE).
        let mut ps: P = std::ptr::null_mut();
        assert_eq!(call!(d, DEV_OPENSTORE, (u32, *mut P) -> HRESULT; STGM_READ, &mut ps), S_OK);
        let mut n = 0u32;
        assert_eq!(call!(ps, PS_COUNT, (*mut u32) -> HRESULT; &mut n), S_OK);
        assert!(n >= 4);
        let mut pv = [0u8; 24];
        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_AudioEngine_DeviceFormat, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_BLOB);
        let cb = u32::from_le_bytes(pv[8..12].try_into().unwrap());
        assert_eq!(cb, 40);
        let blob = usize::from_le_bytes(pv[16..24].try_into().unwrap()) as *const WAVEFORMATEXTENSIBLE;
        let x = std::ptr::read_unaligned(blob);
        let wf = x.Format;
        assert_eq!({ wf.wFormatTag }, 0xFFFE);
        assert_eq!({ wf.nChannels }, 2);
        assert_eq!({ wf.nSamplesPerSec }, 48000);
        assert_eq!({ wf.wBitsPerSample }, 32);
        assert_eq!({ wf.cbSize }, 22);
        assert!({ x.SubFormat } == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT);
        assert_eq!(PropVariantClear(pv.as_mut_ptr()), S_OK);
        assert_eq!(pv, [0u8; 24]);

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_Device_FriendlyName, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_LPWSTR);
        let name = wide(usize::from_le_bytes(pv[8..16].try_into().unwrap()) as *const u16);
        assert!(!name.is_empty());
        PropVariantClear(pv.as_mut_ptr());

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_Device_ContainerId, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_CLSID);
        PropVariantClear(pv.as_mut_ptr());

        let unknown = PROPERTYKEY { fmtid: IID_IUnknown, pid: 77 };
        pv = [0xAA; 24];
        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &unknown, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_EMPTY);
        assert_eq!(call!(ps, PS_SETVALUE, (*const PROPERTYKEY, *const u8) -> HRESULT; &unknown, pv.as_ptr()), STG_E_ACCESSDENIED);
        release(ps);

        // IAudioSessionManager2 activates cleanly and provides IAudioSessionControl2 + ISimpleAudioVolume.
        let mut sm: P = std::ptr::null_mut();
        assert_eq!(call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioSessionManager2, CLSCTX_ALL, std::ptr::null_mut(), &mut sm), S_OK);
        assert!(!sm.is_null());
        let mut sc: P = std::ptr::null_mut();
        assert_eq!(call!(sm, 3, (*const GUID, u32, *mut P) -> HRESULT; std::ptr::null(), 0, &mut sc), S_OK);
        assert!(!sc.is_null());
        let mut sc2: P = std::ptr::null_mut();
        assert_eq!(call!(sc, 0, (*const GUID, *mut P) -> HRESULT; &IID_IAudioSessionControl2, &mut sc2), S_OK);
        assert_eq!(call!(sc2, 16, (BOOL) -> HRESULT; TRUE), S_OK); // SetDuckingPreference
        let mut vol: P = std::ptr::null_mut();
        assert_eq!(call!(sm, 4, (*const GUID, u32, *mut P) -> HRESULT; std::ptr::null(), 0, &mut vol), S_OK);
        assert_eq!(call_bridge_f32_guid(vol, 3, 0.75f32, std::ptr::null()), S_OK);
        let mut got_v = 0.0f32;
        assert_eq!(call!(vol, 4, (*mut f32) -> HRESULT; &mut got_v), S_OK);
        assert_eq!(got_v, 0.75f32);
        assert_eq!(call!(vol, 5, (BOOL, *const GUID) -> HRESULT; TRUE, std::ptr::null()), S_OK);
        let mut muted: BOOL = FALSE;
        assert_eq!(call!(vol, 6, (*mut BOOL) -> HRESULT; &mut muted), S_OK);
        assert_eq!(muted, TRUE);
        release(vol);
        release(sc2);
        release(sc);
        release(sm);

        // ISpatialAudioClient activates cleanly on render endpoints.
        let mut sac: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_ISpatialAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut sac),
            S_OK
        );
        assert!(!sac.is_null());
        assert_eq!(release(sac), 0);

        // IAudioEndpointVolume activates cleanly and supports scalar/dB/channel/mute/step queries.
        let mut epv: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioEndpointVolume, CLSCTX_ALL, std::ptr::null_mut(), &mut epv),
            S_OK
        );
        assert!(!epv.is_null());
        let mut epv_ch = 0u32;
        assert_eq!(call!(epv, 5, (*mut u32) -> HRESULT; &mut epv_ch), S_OK);
        assert_eq!(epv_ch, 2);
        assert_eq!(call_bridge_f32_guid(epv, 7, 0.5f32, std::ptr::null()), S_OK);
        let mut epv_scalar = 0.0f32;
        assert_eq!(call!(epv, 9, (*mut f32) -> HRESULT; &mut epv_scalar), S_OK);
        assert!((epv_scalar - 0.5).abs() < 1e-4);
        assert_eq!(call!(epv, 14, (BOOL, *const GUID) -> HRESULT; TRUE, std::ptr::null()), S_OK);
        let mut epv_mute: BOOL = FALSE;
        assert_eq!(call!(epv, 15, (*mut BOOL) -> HRESULT; &mut epv_mute), S_OK);
        assert_eq!(epv_mute, TRUE);
        let mut min_db = 0.0f32;
        let mut max_db = 0.0f32;
        let mut inc_db = 0.0f32;
        assert_eq!(call!(epv, 20, (*mut f32, *mut f32, *mut f32) -> HRESULT; &mut min_db, &mut max_db, &mut inc_db), S_OK);
        assert!(min_db < max_db && inc_db > 0.0);
        assert_eq!(release(epv), 0);

        // Unknown IIDs return standard COM E_NOINTERFACE.
        let unknown_iid = GUID::new(0x1111_2222, 0x3333, 0x4444, [0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC]);
        let mut o: P = 1 as P;
        let hr = call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &unknown_iid, CLSCTX_ALL, std::ptr::null_mut(), &mut o);
        assert_eq!(hr, E_NOINTERFACE);
        assert!(o.is_null());

        release(d);
        release(d1);
        release(d0);
        release(coll);
        assert_eq!(release(e), 0);
    }
}

/// One 512-frame Wwise "frame" of a recognisable ramp: sample k of block b.
fn block(b: u32, frames: u32) -> Vec<f32> {
    (0..frames * 2).map(|k| ((b * 1000 + k) % 4096) as f32 / 4096.0 - 0.5).collect()
}

#[test]
fn wwise_sink_sequence_on_null_device() {
    let _f = Fixture::null();
    let dir = std::env::temp_dir().join(format!("wwise-audio-test-{}", std::process::id()));
    let wav = dir.join("sink.wav");
    tap::override_next(Some(wav.clone()));
    unsafe {
        let e = enumerator();
        let d = default_device(e);
        let c = activate_client(d);

        // Not initialized yet.
        let mut u = 0u32;
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut u), AUDCLNT_E_NOT_INITIALIZED);

        // 0x141653fc0: GetMixFormat; IsFormatSupported only if channel counts differ.
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        let m = std::ptr::read_unaligned(mix);
        assert_eq!(({ m.nChannels }, { m.nSamplesPerSec }, { m.nBlockAlign }), (2, 48000, 8));
        let mut want = std::ptr::read_unaligned(mix as *const WAVEFORMATEXTENSIBLE);
        want.Format.nChannels = 6;
        want.Format.nBlockAlign = 24;
        want.Format.nAvgBytesPerSec = 48000 * 24;
        want.dwChannelMask = 0x3F;
        let mut closest: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_ISFMT, (u32, *const WAVEFORMATEX, *mut *mut WAVEFORMATEX) -> HRESULT; SHARED, &want as *const _ as *const WAVEFORMATEX, &mut closest), S_FALSE);
        assert!(!closest.is_null());
        assert_eq!({ std::ptr::read_unaligned(closest).nChannels }, 2);
        CoTaskMemFree(closest as P);
        assert_eq!(call!(c, AC_ISFMT, (u32, *const WAVEFORMATEX, *mut *mut WAVEFORMATEX) -> HRESULT; SHARED, mix, &mut closest), S_OK);
        assert!(closest.is_null());

        // QI IAudioClient2 -> IsOffloadCapable -> SetClientProperties (must be >= 0).
        let (hr, c2) = qi(c, &IID_IAudioClient2);
        assert_eq!(hr, S_OK);
        let mut off: BOOL = 7;
        assert_eq!(call!(c2, AC2_OFFLOAD, (u32, *mut BOOL) -> HRESULT; 0, &mut off), S_OK);
        assert_eq!(off, FALSE);
        let props: [u32; 3] = [12, 0, 6 /* AudioCategory_GameEffects */];
        assert!(call!(c2, AC2_PROPS, (*const u32) -> HRESULT; props.as_ptr()) >= 0);
        release(c2);

        let (mut def, mut min) = (0i64, 0i64);
        assert_eq!(call!(c, AC_PERIOD, (*mut i64, *mut i64) -> HRESULT; &mut def, &mut min), S_OK);
        assert_eq!((def, min), (100_000, 30_000));

        // Initialize(SHARED, EVENTCALLBACK, ...) must return exactly S_OK.
        let init = |fmt: *const WAVEFORMATEX| call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, def, 0, fmt, std::ptr::null());
        assert_eq!(init(mix), S_OK);
        assert_eq!(init(mix), AUDCLNT_E_ALREADY_INITIALIZED);
        assert_eq!(call!(c, AC2_PROPS, (*const u32) -> HRESULT; props.as_ptr()), AUDCLNT_E_ALREADY_INITIALIZED);

        // Start before SetEventHandle is an error.
        assert_eq!(call!(c, AC_START, () -> HRESULT;), AUDCLNT_E_EVENTHANDLE_NOT_SET);
        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert!(!ev.is_null());
        assert_eq!(call!(c, AC_SETEVENT, (HANDLE) -> HRESULT; ev), S_OK);

        let mut buf = 0u32;
        assert_eq!(call!(c, AC_BUFSIZE, (*mut u32) -> HRESULT; &mut buf), S_OK);
        // Sized for 4096-frame device slices (memo §2.1): 2 * (4096 + 16) + 512.
        assert!(buf >= 2 * 4096 + 512, "buffer {buf}");
        let mut lat = 0i64;
        assert_eq!(call!(c, AC_LATENCY, (*mut i64) -> HRESULT; &mut lat), S_OK);
        assert!(lat >= 100_000);

        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);
        let mut ck: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock, &mut ck), S_OK);
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), AUDCLNT_E_WRONG_ENDPOINT_TYPE);

        let get = |n: u32| {
            let mut p: *mut u8 = std::ptr::null_mut();
            let hr = call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; n, &mut p);
            (hr, p)
        };
        let rel = |n: u32, fl: u32| call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; n, fl);

        // Out-of-order and size checks.
        assert_eq!(rel(0, 0), AUDCLNT_E_OUT_OF_ORDER);
        assert_eq!(get(buf + 1).0, AUDCLNT_E_BUFFER_TOO_LARGE);

        // Silent prefill of the whole buffer (0x141652ce0).
        let (hr, p) = get(buf);
        assert_eq!(hr, S_OK);
        assert!(!p.is_null());
        assert_eq!(get(1).0, AUDCLNT_E_OUT_OF_ORDER);
        assert_eq!(rel(buf + 1, SILENT), AUDCLNT_E_INVALID_SIZE);
        assert_eq!(rel(buf, SILENT), S_OK);
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut u), S_OK);
        assert_eq!(u, buf);
        assert_eq!(get(1).0, AUDCLNT_E_BUFFER_TOO_LARGE);

        let mut freq = 0u64;
        assert_eq!(call!(ck, CK_FREQ, (*mut u64) -> HRESULT; &mut freq), S_OK);
        assert_eq!(freq, 48000 * 8);

        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), AUDCLNT_E_NOT_STOPPED);

        // Event loop: IsDataNeeded ((buf - pad) / 512 frames) + OnFrameEnd submits.
        let t0 = std::time::Instant::now();
        let mut blocks = 0u32;
        let mut wakes = 0u32;
        while blocks < 94 {
            // ~1 s at 512 frames / 48 kHz
            let w = crate::kernel32::threads::WaitForSingleObject(ev, 1000);
            assert_eq!(w, 0, "event not signalled (WAIT_TIMEOUT) after {blocks} blocks");
            wakes += 1;
            assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut u), S_OK);
            let mut need = (buf - u) / 512;
            while need > 0 && blocks < 94 {
                let (hr, p) = get(512);
                assert_eq!(hr, S_OK);
                let src = block(blocks, 512);
                std::ptr::copy_nonoverlapping(src.as_ptr() as *const u8, p, src.len() * 4);
                assert_eq!(rel(512, 0), S_OK);
                blocks += 1;
                need -= 1;
            }
            assert!(t0.elapsed().as_secs() < 10, "too slow");
        }
        let elapsed = t0.elapsed().as_secs_f64();
        assert!(wakes > 20, "only {wakes} wakeups");
        // Adaptive headroom: the queue stays near the steady-state target
        // (2 * 512-frame null slices + 512), not the 8.7k-frame buffer.
        {
            let core = super::audioclient::ClientObj::inner(c as *mut super::audioclient::ClientObj).clone();
            let st = core.st.lock().unwrap();
            let s = st.stream.as_ref().unwrap();
            let queued = s.shared.ring.len();
            assert!(queued <= 2 * (512 + 16) + 512 + 512, "queued {queued} frames");
            assert!(s.dropped_silence > 0, "silent prefill beyond target should not be queued");
            assert_eq!(s.shared.underrun_frames.load(Ordering::Relaxed), 0, "underruns on the null device");
        }

        let (mut pos, mut qpc) = (0u64, 0u64);
        assert_eq!(call!(ck, CK_POS, (*mut u64, *mut u64) -> HRESULT; &mut pos, &mut qpc), S_OK);
        assert!(pos > 0 && pos % 8 == 0);
        assert!(qpc > 0);

        assert_eq!(call!(c, AC_RESET, () -> HRESULT;), AUDCLNT_E_NOT_STOPPED);
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_OK);
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_FALSE);
        let _ = call!(c, AC_RESET, () -> HRESULT;);
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut u), S_OK);
        assert_eq!(u, 0);

        CoTaskMemFree(mix as P);
        crate::kernel32::threads::CloseHandle(ev);
        release(ck);
        release(rc);
        assert_eq!(release(c), 0, "render/clock services must not outlive their release");
        release(d);
        release(e);

        // The tap holds the exact submitted bytes: zero prefill + the blocks.
        let bytes = std::fs::read(&wav).unwrap();
        let data = &bytes[tap::DATA_OFFSET as usize..];
        let pre = buf as usize * 8;
        assert_eq!(data.len(), pre + blocks as usize * 512 * 8, "elapsed {elapsed:.2}s");
        assert!(data[..pre].iter().all(|&b| b == 0));
        for b in 0..blocks {
            let want: Vec<u8> = block(b, 512).iter().flat_map(|v| v.to_le_bytes()).collect();
            let off = pre + b as usize * 4096;
            assert!(data[off..off + 4096] == want[..], "block {b} differs");
        }
        let side = std::fs::read_to_string(tap::sidecar_path(&wav)).unwrap();
        assert_eq!(side.lines().count(), 1 + 1 + blocks as usize);
        let _ = std::fs::remove_dir_all(&dir);
    }
}

#[test]
fn autoconvert_pcm16_mono_44k() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);
        let fmt = WAVEFORMATEX {
            wFormatTag: 1,
            nChannels: 1,
            nSamplesPerSec: 44100,
            nAvgBytesPerSec: 88200,
            nBlockAlign: 2,
            wBitsPerSample: 16,
            cbSize: 0,
        };
        let c = activate_client(d);
        let init = |c: P, flags: u32| call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, flags, 200_000, 0, &fmt, std::ptr::null());
        assert_eq!(init(c, 0), AUDCLNT_E_UNSUPPORTED_FORMAT);
        assert_eq!(init(c, AUTOCONVERTPCM | SRC_DEFAULT_QUALITY), S_OK);
        let mut buf = 0u32;
        call!(c, AC_BUFSIZE, (*mut u32) -> HRESULT; &mut buf);
        assert!(buf >= 882, "{buf}"); // >= the requested 20 ms
        // Exclusive mode is refused honestly.
        let c2 = activate_client(d);
        assert_eq!(
            call!(c2, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; 1, 0, 200_000, 200_000, &fmt, std::ptr::null()),
            AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED
        );
        // Timer-driven (no event) stream: submit converted PCM, poll padding drain.
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);
        let mut p: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 441, &mut p), S_OK);
        let s = p as *mut i16;
        for i in 0..441 {
            *s.add(i) = if i % 2 == 0 { 16384 } else { -16384 };
        }
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 441, 0), S_OK);
        let mut pad = 0u32;
        call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad);
        assert!(pad >= 441, "{pad}"); // includes adaptive headroom
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        let t0 = std::time::Instant::now();
        while pad > 0 && t0.elapsed().as_millis() < 2000 {
            std::thread::sleep(std::time::Duration::from_millis(5));
            call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad);
        }
        assert_eq!(pad, 0, "null device did not drain the stream");

        // Verify IAudioClock reports client-format position (441 frames @ 44.1 kHz = 882 bytes)
        // while IAudioClock2::GetDevicePosition reports endpoint-rate frames (480 frames @ 48 kHz).
        let mut ck: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock, &mut ck), S_OK);
        let (hr_ck2, ck2) = qi(ck, &IID_IAudioClock2);
        assert_eq!(hr_ck2, S_OK);
        let mut freq = 0u64;
        let mut pos_bytes = 0u64;
        let mut dev_frames = 0u64;
        let mut qpc = 0u64;
        assert_eq!(call!(ck, 3, (*mut u64) -> HRESULT; &mut freq), S_OK);
        assert_eq!(call!(ck, 4, (*mut u64, *mut u64) -> HRESULT; &mut pos_bytes, &mut qpc), S_OK);
        assert_eq!(call!(ck2, 3, (*mut u64, *mut u64) -> HRESULT; &mut dev_frames, &mut qpc), S_OK);
        assert_eq!(freq, 44_100 * 2);
        assert_eq!(pos_bytes, 441 * 2);
        assert_eq!(
            dev_frames, 480,
            "GetDevicePosition must report endpoint-rate frames (480 @ 48 kHz), not client-rate frames (441 @ 44.1 kHz)"
        );
        assert_eq!(release(ck2), 1);
        assert_eq!(release(ck), 0);

        release(rc);
        release(c2);
        assert_eq!(release(c), 0);
        let _ = add_ref(d);
        release(d);
        release(d);
        release(e);
    }
}

/// Plays a 440 Hz tone for 1.5 s on the default macOS output through the full
/// WASAPI path. Run manually: `cargo hybrid-test -p win32-rt -- --ignored audible`.
#[test]
#[ignore]
fn audible_tone_on_default_device() {
    let _g = super::audio_test_lock();
    backend::force_backend(Some(Backend::CoreAudio));
    super::mmdevapi::refresh();
    unsafe {
        let e = enumerator();
        let d = default_device(e);
        let c = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix);
        let ch = { std::ptr::read_unaligned(mix).nChannels } as usize;
        assert_eq!(call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, 100_000, 0, mix, std::ptr::null()), S_OK);
        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert_eq!(call!(c, AC_SETEVENT, (HANDLE) -> HRESULT; ev), S_OK);
        let mut buf = 0u32;
        call!(c, AC_BUFSIZE, (*mut u32) -> HRESULT; &mut buf);
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);
        let mut ph = 0f64;
        let mut fill = |n: u32| {
            let mut p: *mut u8 = std::ptr::null_mut();
            assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; n, &mut p), S_OK);
            let s = p as *mut f32;
            for i in 0..n as usize {
                let v = (ph.sin() * 0.2) as f32;
                ph += 2.0 * std::f64::consts::PI * 440.0 / 48000.0;
                for k in 0..ch {
                    *s.add(i * ch + k) = v;
                }
            }
            assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; n, 0), S_OK);
        };
        fill(buf);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        let t0 = std::time::Instant::now();
        let mut timeouts = 0;
        while t0.elapsed().as_millis() < 1500 {
            if crate::kernel32::threads::WaitForSingleObject(ev, 200) != 0 {
                timeouts += 1;
                continue;
            }
            let mut pad = 0u32;
            call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad);
            if buf > pad {
                fill(buf - pad);
            }
        }
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_OK);
        let core = super::audioclient::ClientObj::inner(c as *mut super::audioclient::ClientObj).clone();
        {
            let st = core.st.lock().unwrap();
            let s = st.stream.as_ref().unwrap();
            let cb = s.shared.callbacks.load(Ordering::Relaxed);
            let under = s.shared.underrun_frames.load(Ordering::Relaxed);
            let slice = s.shared.max_slice.load(Ordering::Relaxed);
            eprintln!(
                "audible: device '{}', buffer {buf}, queued at stop {}, callbacks {cb}, underrun frames {under}, max slice {slice}, event timeouts {timeouts}",
                core.ep.info.name,
                s.shared.ring.len()
            );
            assert!(cb > 10, "CoreAudio render callback never ran");
        }
        drop(core);
        CoTaskMemFree(mix as P);
        release(rc);
        release(c);
        release(d);
        release(e);
        crate::kernel32::threads::CloseHandle(ev);
    }
    backend::force_backend(None);
}

#[test]
fn device_override_selects_by_uid_or_name() {
    let dev = |id: u32, uid: &str, name: &str, def: bool| backend::DeviceInfo {
        id,
        uid: uid.into(),
        name: name.into(),
        rate: 48000,
        channels: 2,
        io_frames: 512,
        latency_frames: 0,
        headphones: false,
        is_default: def,
    };
    let base = vec![dev(1, "kvm-uid", "GLKVM", true), dev(2, "BuiltInSpeakerDevice", "MacBook Pro Speakers", false), dev(3, "bt", "AirPods", false)];
    let mut v = base.clone();
    assert!(backend::select_device(&mut v, "speakers"));
    assert_eq!(v[0].id, 2);
    assert!(!v[0].is_default, "an explicitly chosen non-default device is bound via HALOutput");
    assert!(v.iter().all(|d| !d.is_default));
    let mut v = base.clone();
    assert!(backend::select_device(&mut v, "bt"));
    assert_eq!(v[0].id, 3);
    let mut v = base.clone();
    assert!(!backend::select_device(&mut v, "nope"));
    assert_eq!(v[0].id, 1);
    assert!(v[0].is_default);
}

// --- device changes (A1): simulated host events through devnotify ---------

/// Recording IMMNotificationClient.
static NOTES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static REC_REFS: AtomicU32 = AtomicU32::new(1);
unsafe extern "win64" fn r_add(_t: P) -> u32 {
    REC_REFS.fetch_add(1, Ordering::SeqCst) + 1
}
unsafe extern "win64" fn r_rel(_t: P) -> u32 {
    REC_REFS.fetch_sub(1, Ordering::SeqCst) - 1
}
unsafe extern "win64" fn r_state(_t: P, id: *const u16, st: u32) -> HRESULT {
    NOTES.lock().unwrap().push(format!("state {} {st}", wide(id)));
    S_OK
}
unsafe extern "win64" fn r_added(_t: P, id: *const u16) -> HRESULT {
    NOTES.lock().unwrap().push(format!("added {}", wide(id)));
    S_OK
}
unsafe extern "win64" fn r_removed(_t: P, id: *const u16) -> HRESULT {
    NOTES.lock().unwrap().push(format!("removed {}", wide(id)));
    S_OK
}
unsafe extern "win64" fn r_default(_t: P, flow: u32, role: u32, id: *const u16) -> HRESULT {
    let id = if id.is_null() { "NULL".to_string() } else { wide(id) };
    NOTES.lock().unwrap().push(format!("default {flow} {role} {id}"));
    S_OK
}
unsafe extern "win64" fn r_prop(_t: P, id: *const u16, key: *const super::devnotify::PropertyKey) -> HRESULT {
    let k = std::ptr::read_unaligned(key);
    NOTES.lock().unwrap().push(format!("prop {} {:?},{}", wide(id), k.fmtid, k.pid));
    S_OK
}

fn take_notes() -> Vec<String> {
    std::mem::take(&mut *NOTES.lock().unwrap())
}

fn fake_dev(uid: &str, default: bool) -> backend::DeviceInfo {
    backend::DeviceInfo { uid: uid.into(), name: format!("Test {uid}"), is_default: default, ..backend::null_device() }
}

#[test]
fn device_removal_invalidates_streams_and_notifies_clients() {
    use super::devnotify;
    use super::mmdevapi::Endpoint;
    let _f = Fixture::null();
    REC_REFS.store(1, Ordering::SeqCst);
    let null = backend::null_device();
    let usb = fake_dev("aw2-test-usb", false);
    let usb2 = fake_dev("aw2-test-usb2", true);
    let (id_null, id_usb, id_usb2) = (Endpoint::new(null.clone()).id, Endpoint::new(usb.clone()).id, Endpoint::new(usb2.clone()).id);
    unsafe {
        let e = enumerator();
        // Baseline before anyone listens: the null device (default) + a USB device.
        devnotify::simulate(vec![null.clone(), usb.clone()]);

        let vt: [usize; 8] = [
            n_qi as *const () as usize,
            r_add as *const () as usize,
            r_rel as *const () as usize,
            r_state as *const () as usize,
            r_added as *const () as usize,
            r_removed as *const () as usize,
            r_default as *const () as usize,
            r_prop as *const () as usize,
        ];
        let obj: *const [usize; 8] = &vt;
        let np = &obj as *const *const [usize; 8] as P;
        assert_eq!(call!(e, EN_REGISTER, (P) -> HRESULT; np), S_OK);
        take_notes();

        // A started event-driven stream on the USB endpoint.
        let dev = super::mmdevapi::new_device(std::sync::Arc::new(Endpoint::new(usb.clone()))) as P;
        let mut state = 0u32;
        assert_eq!(call!(dev, DEV_GETSTATE, (*mut u32) -> HRESULT; &mut state), S_OK);
        assert_eq!(state, 1);
        let c = activate_client(dev);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        assert_eq!(call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, 100_000, 0, mix, std::ptr::null()), S_OK);
        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert_eq!(call!(c, AC_SETEVENT, (HANDLE) -> HRESULT; ev), S_OK);
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);
        let mut ck: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock, &mut ck), S_OK);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        let mut pad = 0u32;
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), S_OK);
        let mut p: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 512, &mut p), S_OK);
        // Stop (no more periodic events) and drain the event so the
        // invalidation wakeup is observable.
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_OK);
        std::thread::sleep(std::time::Duration::from_millis(50));
        crate::kernel32::threads::ResetEvent(ev);

        // Unplug the USB device.
        devnotify::simulate(vec![null.clone()]);
        assert_eq!(take_notes(), vec![format!("state {id_usb} 4"), format!("removed {id_usb}")]);
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), AUDCLNT_E_DEVICE_INVALIDATED);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 512, 0), AUDCLNT_E_DEVICE_INVALIDATED);
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 512, &mut p), AUDCLNT_E_DEVICE_INVALIDATED);
        let mut pos = 0u64;
        assert_eq!(call!(ck, CK_POS, (*mut u64, *mut u64) -> HRESULT; &mut pos, std::ptr::null_mut()), AUDCLNT_E_DEVICE_INVALIDATED);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), AUDCLNT_E_DEVICE_INVALIDATED);
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), AUDCLNT_E_DEVICE_INVALIDATED);
        // The client was woken up to notice.
        assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 0), 0);
        // The stale IMMDevice reports NOTPRESENT; a new client on it can't initialize.
        assert_eq!(call!(dev, DEV_GETSTATE, (*mut u32) -> HRESULT; &mut state), S_OK);
        assert_eq!(state, 4);
        let c2 = activate_client(dev);
        assert_eq!(call!(c2, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, 100_000, 0, mix, std::ptr::null()), AUDCLNT_E_DEVICE_INVALIDATED);
        release(c2);

        // A new device appears and becomes the default.
        devnotify::simulate(vec![usb2.clone(), null.clone()]);
        assert_eq!(
            take_notes(),
            vec![
                format!("added {id_usb2}"),
                format!("state {id_usb2} 1"),
                format!("default 0 0 {id_usb2}"),
                format!("default 0 1 {id_usb2}"),
                format!("default 0 2 {id_usb2}"),
            ]
        );
        // The null endpoint's streams are untouched by a default change.
        assert!(super::devnotify::endpoint_present(&id_null));

        // Restore the baseline (null device only, default) and unregister.
        devnotify::simulate(vec![null.clone()]);
        let back = take_notes();
        assert_eq!(back.last(), Some(&format!("default 0 2 {id_null}")), "{back:?}");
        assert_eq!(call!(e, EN_UNREGISTER, (P) -> HRESULT; np), S_OK);
        assert_eq!(REC_REFS.load(Ordering::SeqCst), 1);

        CoTaskMemFree(mix as P);
        release(ck);
        release(rc);
        release(c);
        release(dev);
        crate::kernel32::threads::CloseHandle(ev);
        release(e);
    }
}

/// Wait up to `secs` for a recorded notification matching `pred`; returns all notes seen.
fn wait_note(secs: u64, pred: impl Fn(&str) -> bool) -> (bool, Vec<String>) {
    let t0 = std::time::Instant::now();
    let mut seen = Vec::new();
    loop {
        seen.extend(take_notes());
        if seen.iter().any(|n| pred(n)) {
            return (true, seen);
        }
        if t0.elapsed() > std::time::Duration::from_secs(secs) {
            return (false, seen);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

/// Real CoreAudio events: a process-private aggregate device is hot-plugged
/// and removed (with a live stream on it), and the system default output is
/// switched to another device and back. Changes the system default output
/// for a moment, so it only runs with `AW2_AUDIO_TEST_DEVICES=1`.
#[cfg(target_os = "macos")]
#[test]
fn coreaudio_hotplug_and_default_switch() {
    use super::devnotify;
    use super::mmdevapi::Endpoint;
    if std::env::var("AW2_AUDIO_TEST_DEVICES").as_deref() != Ok("1") {
        eprintln!("skipped (set AW2_AUDIO_TEST_DEVICES=1)");
        return;
    }
    let _g = super::audio_test_lock();
    REC_REFS.store(1, Ordering::SeqCst);
    backend::force_backend(Some(Backend::CoreAudio));
    let devs = backend::output_devices();
    assert!(devs.len() >= 2, "needs two output devices: {devs:?}");
    let (def, other) = (devs[0].clone(), devs[1].clone());
    let id_of = |uid: &str| Endpoint::new(backend::DeviceInfo { uid: uid.into(), ..backend::null_device() }).id;
    unsafe {
        let e = enumerator(); // starts the listeners
        devnotify::rescan();
        let vt: [usize; 8] = [
            n_qi as *const () as usize,
            r_add as *const () as usize,
            r_rel as *const () as usize,
            r_state as *const () as usize,
            r_added as *const () as usize,
            r_removed as *const () as usize,
            r_default as *const () as usize,
            r_prop as *const () as usize,
        ];
        let obj: *const [usize; 8] = &vt;
        let np = &obj as *const *const [usize; 8] as P;
        assert_eq!(call!(e, EN_REGISTER, (P) -> HRESULT; np), S_OK);
        take_notes();

        // Hot-plug a private aggregate wrapping the non-default device.
        let uid = format!("aw2-test-aggregate-{}", std::process::id());
        let agg = backend::create_private_aggregate(&uid, "AW2 Test Aggregate", &other.uid).expect("create aggregate");
        let agg_id = id_of(&uid);
        let (ok, seen) = wait_note(5, |n| n == format!("added {agg_id}"));
        assert!(ok, "no OnDeviceAdded for the aggregate: {seen:?}");

        // A running HALOutput stream on it.
        let ep = super::mmdevapi::refresh().into_iter().find(|ep| ep.info.uid == uid).expect("aggregate endpoint");
        let dev = super::mmdevapi::new_device(ep) as P;
        let c = activate_client(dev);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        assert_eq!(call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, 100_000, 0, mix, std::ptr::null()), S_OK);
        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert_eq!(call!(c, AC_SETEVENT, (HANDLE) -> HRESULT; ev), S_OK);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        let mut pad = 0u32;
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), S_OK);

        // Unplug it.
        backend::destroy_private_aggregate(agg).expect("destroy aggregate");
        let (ok, seen) = wait_note(5, |n| n == format!("removed {agg_id}"));
        assert!(ok, "no OnDeviceRemoved for the aggregate: {seen:?}");
        assert!(seen.contains(&format!("state {agg_id} 4")), "{seen:?}");
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), AUDCLNT_E_DEVICE_INVALIDATED);
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), AUDCLNT_E_DEVICE_INVALIDATED);
        release(c);
        release(dev);
        CoTaskMemFree(mix as P);
        crate::kernel32::threads::CloseHandle(ev);

        // Switch the system default output and back.
        backend::set_default_output_device(other.id).expect("set default");
        let want = format!("default 0 0 {}", id_of(&other.uid));
        let (ok, seen) = wait_note(5, |n| n == want);
        backend::set_default_output_device(def.id).expect("restore default");
        assert!(ok, "no OnDefaultDeviceChanged to '{}': {seen:?}", other.name);
        let want = format!("default 0 0 {}", id_of(&def.uid));
        let (ok, seen) = wait_note(5, |n| n == want);
        assert!(ok, "no OnDefaultDeviceChanged back to '{}': {seen:?}", def.name);

        assert_eq!(call!(e, EN_UNREGISTER, (P) -> HRESULT; np), S_OK);
        release(e);
    }
}

#[test]
fn capture_endpoint_enum_properties_and_silent_packets() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();

        // GetDefaultAudioEndpoint(eCapture, eConsole) returns the capture endpoint.
        let mut cap_dev: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, E_CONSOLE, &mut cap_dev), S_OK);
        assert!(!cap_dev.is_null());

        // Endpoint ID has the Windows capture prefix {0.0.1.00000000}.{GUID}.
        let mut cap_id: *mut u16 = std::ptr::null_mut();
        assert_eq!(call!(cap_dev, DEV_GETID, (*mut *mut u16) -> HRESULT; &mut cap_id), S_OK);
        let cap_ids = wide(cap_id);
        assert!(cap_ids.starts_with("{0.0.1.00000000}.{"), "{cap_ids}");

        // GetDevice(cap_id) round-trips the capture endpoint.
        let mut by_id: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_GETDEVICE, (*const u16, *mut P) -> HRESULT; cap_id, &mut by_id), S_OK);
        assert!(!by_id.is_null());
        CoTaskMemFree(cap_id as P);
        release(by_id);

        // EnumAudioEndpoints(eCapture) and EnumAudioEndpoints(eAll).
        let mut cap_coll: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_ENUM, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, DEVICE_STATEMASK_ALL, &mut cap_coll), S_OK);
        let mut cap_count = 0u32;
        assert_eq!(call!(cap_coll, COLL_COUNT, (*mut u32) -> HRESULT; &mut cap_count), S_OK);
        assert_eq!(cap_count, 1);
        let mut cap_item: P = std::ptr::null_mut();
        assert_eq!(call!(cap_coll, COLL_ITEM, (u32, *mut P) -> HRESULT; 0, &mut cap_item), S_OK);
        release(cap_item);
        release(cap_coll);

        let mut all_coll: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_ENUM, (u32, u32, *mut P) -> HRESULT; E_ALL, DEVICE_STATEMASK_ALL, &mut all_coll), S_OK);
        let mut all_count = 0u32;
        assert_eq!(call!(all_coll, COLL_COUNT, (*mut u32) -> HRESULT; &mut all_count), S_OK);
        assert_eq!(all_count, 2);
        let mut flows = Vec::new();
        for i in 0..all_count {
            let mut item: P = std::ptr::null_mut();
            assert_eq!(call!(all_coll, COLL_ITEM, (u32, *mut P) -> HRESULT; i, &mut item), S_OK);
            let (hr, ep) = qi(item, &IID_IMMEndpoint);
            assert_eq!(hr, S_OK);
            let mut fl = 99u32;
            assert_eq!(call!(ep, 3, (*mut u32) -> HRESULT; &mut fl), S_OK);
            flows.push(fl);
            release(ep);
            release(item);
        }
        assert_eq!(flows, vec![E_RENDER, E_CAPTURE]);
        release(all_coll);

        // IMMEndpoint::GetDataFlow on cap_dev == eCapture (1).
        let (hr, ep) = qi(cap_dev, &IID_IMMEndpoint);
        assert_eq!(hr, S_OK);
        let mut flow = 99u32;
        assert_eq!(call!(ep, 3, (*mut u32) -> HRESULT; &mut flow), S_OK);
        assert_eq!(flow, E_CAPTURE);
        release(ep);

        // IPropertyStore exposes FriendlyName, Interface FriendlyName, DeviceFormat, FormFactor, ContainerId.
        let mut ps: P = std::ptr::null_mut();
        assert_eq!(call!(cap_dev, DEV_OPENSTORE, (u32, *mut P) -> HRESULT; STGM_READ, &mut ps), S_OK);
        let mut pv = [0u8; 24];
        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_Device_FriendlyName, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_LPWSTR);
        let fname = wide(usize::from_le_bytes(pv[8..16].try_into().unwrap()) as *const u16);
        assert!(fname.contains("Default Microphone"), "{fname}");
        PropVariantClear(pv.as_mut_ptr());

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_DeviceInterface_FriendlyName, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_LPWSTR);
        let ifname = wide(usize::from_le_bytes(pv[8..16].try_into().unwrap()) as *const u16);
        assert_eq!(ifname, "Default Microphone");
        PropVariantClear(pv.as_mut_ptr());

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_AudioEngine_DeviceFormat, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_BLOB);
        let blob = usize::from_le_bytes(pv[16..24].try_into().unwrap()) as *const WAVEFORMATEXTENSIBLE;
        let x = std::ptr::read_unaligned(blob);
        assert_eq!({ x.Format.nSamplesPerSec }, 48000);
        assert_eq!({ x.Format.nChannels }, 2);
        assert_eq!({ x.Format.wBitsPerSample }, 32);
        assert!({ x.SubFormat } == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT);
        PropVariantClear(pv.as_mut_ptr());

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_AudioEndpoint_FormFactor, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_UI4);
        assert_eq!(u32::from_le_bytes(pv[8..12].try_into().unwrap()), 4); // Microphone
        PropVariantClear(pv.as_mut_ptr());

        assert_eq!(call!(ps, PS_GETVALUE, (*const PROPERTYKEY, *mut u8) -> HRESULT; &PKEY_Device_ContainerId, pv.as_mut_ptr()), S_OK);
        assert_eq!(u16::from_le_bytes([pv[0], pv[1]]), VT_CLSID);
        let cid_ptr = usize::from_le_bytes(pv[8..16].try_into().unwrap()) as *const [u8; 16];
        let cid_bytes = std::ptr::read_unaligned(cid_ptr);
        assert_ne!(cid_bytes, [0u8; 16], "capture endpoint container GUID must be non-zero");
        PropVariantClear(pv.as_mut_ptr());
        release(ps);

        // Activate IAudioClient on capture endpoint and drive IAudioCaptureClient.
        let c = activate_client(cap_dev);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        let m = std::ptr::read_unaligned(mix);
        assert_eq!(({ m.nSamplesPerSec }, { m.nChannels }, { m.wBitsPerSample }), (48000, 2, 32));

        let mut closest: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_ISFMT, (u32, *const WAVEFORMATEX, *mut *mut WAVEFORMATEX) -> HRESULT; SHARED, mix, &mut closest), S_OK);
        assert!(closest.is_null());

        let (mut def_p, mut min_p) = (0i64, 0i64);
        assert_eq!(call!(c, AC_PERIOD, (*mut i64, *mut i64) -> HRESULT; &mut def_p, &mut min_p), S_OK);
        assert_eq!((def_p, min_p), (100_000, 30_000));

        assert_eq!(call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, std::ptr::null()), S_OK);
        let mut buf_frames = 0u32;
        assert_eq!(call!(c, AC_BUFSIZE, (*mut u32) -> HRESULT; &mut buf_frames), S_OK);
        assert!(buf_frames >= 480 * 4, "capture buffer {buf_frames}");

        // Render client is rejected on a capture endpoint; capture client + clock succeed.
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), AUDCLNT_E_WRONG_ENDPOINT_TYPE);
        assert!(rc.is_null());
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert!(!cc.is_null());
        let mut ck: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock, &mut ck), S_OK);

        // Before Start: no packet ready -> GetNextPacketSize == 0, GetBuffer == AUDCLNT_S_BUFFER_EMPTY.
        let mut next_pkt = 99u32;
        assert_eq!(call!(cc, CC_NEXT, (*mut u32) -> HRESULT; &mut next_pkt), S_OK);
        assert_eq!(next_pkt, 0);
        // Exercise deliberately unaligned out-parameter pointers (offset +1 in stack byte array).
        let mut unaligned = [0xCCu8; 64];
        let p_data = unaligned.as_mut_ptr().add(1) as *mut *mut u8;
        let p_frames = unaligned.as_mut_ptr().add(9) as *mut u32;
        let p_flags = unaligned.as_mut_ptr().add(13) as *mut u32;
        let p_devpos = unaligned.as_mut_ptr().add(17) as *mut u64;
        let p_qpc = unaligned.as_mut_ptr().add(25) as *mut u64;
        assert_eq!(
            call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; p_data, p_frames, p_flags, p_devpos, p_qpc),
            AUDCLNT_S_BUFFER_EMPTY
        );
        assert!(std::ptr::read_unaligned(p_data).is_null());
        assert_eq!(std::ptr::read_unaligned(p_frames), 0);
        assert_eq!(std::ptr::read_unaligned(p_flags), 0);

        // Start and wait for real wall-clock stream cadence (>= 10 ms period).
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);
        std::thread::sleep(std::time::Duration::from_millis(35));

        let mut pad = 0u32;
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), S_OK);
        assert!(pad >= 480, "expected >= 1 capture packet queued after 35 ms, got {pad}");
        assert_eq!(call!(cc, CC_NEXT, (*mut u32) -> HRESULT; &mut next_pkt), S_OK);
        assert_eq!(next_pkt, 480);

        // GetBuffer returns a silent packet (AUDCLNT_BUFFERFLAGS_SILENT = 0x2) with zeroed bytes.
        assert_eq!(
            call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; p_data, p_frames, p_flags, p_devpos, p_qpc),
            S_OK
        );
        let data_ptr = std::ptr::read_unaligned(p_data);
        let got_frames = std::ptr::read_unaligned(p_frames);
        let got_flags = std::ptr::read_unaligned(p_flags);
        let got_devpos = std::ptr::read_unaligned(p_devpos);
        let got_qpc = std::ptr::read_unaligned(p_qpc);
        assert!(!data_ptr.is_null());
        assert_eq!(got_frames, 480);
        assert_eq!(got_flags, SILENT);
        assert_eq!(got_devpos, 0);
        assert!(got_qpc > 0);
        let pcm = std::slice::from_raw_parts(data_ptr, got_frames as usize * 8);
        assert!(pcm.iter().all(|&b| b == 0), "silent capture packet must be zeroed");

        // Second GetBuffer before ReleaseBuffer -> OUT_OF_ORDER; wrong size -> INVALID_SIZE.
        assert_eq!(
            call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; p_data, p_frames, p_flags, std::ptr::null_mut(), std::ptr::null_mut()),
            AUDCLNT_E_OUT_OF_ORDER
        );
        assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; 123), AUDCLNT_E_INVALID_SIZE);

        // ReleaseBuffer(0) releases lock without consuming the packet; re-acquire and consume.
        assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; 0), S_OK);
        assert_eq!(
            call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; p_data, p_frames, p_flags, p_devpos, p_qpc),
            S_OK
        );
        assert_eq!(std::ptr::read_unaligned(p_frames), 480);
        assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; 480), S_OK);
        assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; 0), AUDCLNT_E_OUT_OF_ORDER);

        // IAudioClock position reflects the 480 frames released (480 * 8 bytes).
        let (mut pos, mut qpc) = (0u64, 0u64);
        assert_eq!(call!(ck, CK_POS, (*mut u64, *mut u64) -> HRESULT; &mut pos, &mut qpc), S_OK);
        assert_eq!(pos, 480 * 8);
        assert!(qpc > 0);

        // Stop & Reset clear any remaining buffered capture packets.
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_OK);
        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_FALSE);
        let _ = call!(c, AC_RESET, () -> HRESULT;);
        assert_eq!(call!(c, AC_PADDING, (*mut u32) -> HRESULT; &mut pad), S_OK);
        assert_eq!(pad, 0);
        assert_eq!(call!(cc, CC_NEXT, (*mut u32) -> HRESULT; &mut next_pkt), S_OK);
        assert_eq!(next_pkt, 0);

        CoTaskMemFree(mix as P);
        release(ck);
        release(cc);
        assert_eq!(release(c), 0, "capture client must be cleanly refcounted to 0");
        assert_eq!(release(cap_dev), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn capture_endpoint_mono_and_event_callback() {
    let _g = super::audio_test_lock();
    backend::force_backend(Some(Backend::Null));
    backend::force_input_devices(Some(vec![backend::DeviceInfo {
        uid: "test-mono-mic".into(),
        name: "Test Mono Mic".into(),
        channels: 1,
        ..backend::null_capture_device()
    }]));
    super::mmdevapi::refresh_capture();

    unsafe {
        let e = enumerator();
        let mut cap_dev: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, E_CONSOLE, &mut cap_dev), S_OK);
        let c = activate_client(cap_dev);

        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        let m = std::ptr::read_unaligned(mix);
        assert_eq!(({ m.nSamplesPerSec }, { m.nChannels }, { m.nBlockAlign }), (48000, 1, 4));

        assert_eq!(call!(c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, EVENTCALLBACK, 100_000, 0, mix, std::ptr::null()), S_OK);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), AUDCLNT_E_EVENTHANDLE_NOT_SET);

        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert!(!ev.is_null());
        assert_eq!(call!(c, AC_SETEVENT, (HANDLE) -> HRESULT; ev), S_OK);

        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(c, AC_START, () -> HRESULT;), S_OK);

        // Wait for capture timer event and read at least one silent mono packet.
        let mut got_pkt = false;
        for _ in 0..10 {
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 500), 0);
            let mut next = 0u32;
            assert_eq!(call!(cc, CC_NEXT, (*mut u32) -> HRESULT; &mut next), S_OK);
            if next > 0 {
                let (mut data, mut frames, mut flags) = (std::ptr::null_mut::<u8>(), 0u32, 0u32);
                assert_eq!(
                    call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; &mut data, &mut frames, &mut flags, std::ptr::null_mut(), std::ptr::null_mut()),
                    S_OK
                );
                assert_eq!(frames, 480);
                assert_eq!(flags, SILENT);
                assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; frames), S_OK);
                got_pkt = true;
                break;
            }
        }
        assert!(got_pkt, "event-driven capture stream never produced a silent packet");

        assert_eq!(call!(c, AC_STOP, () -> HRESULT;), S_OK);
        CoTaskMemFree(mix as P);
        crate::kernel32::threads::CloseHandle(ev);
        release(cc);
        assert_eq!(release(c), 0);
        assert_eq!(release(cap_dev), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn capture_host_input_query_is_readonly_and_nonempty() {
    let _g = super::audio_test_lock();
    backend::force_backend(None);
    let devs = backend::input_devices();
    assert!(!devs.is_empty(), "capture endpoint query must return host default input or fallback");
    assert!(!devs[0].name.is_empty());
    assert!(devs[0].rate > 0);
    assert!(devs[0].channels > 0);
}

// ---------------------------------------------------------------------------
// Spatial Audio (ISpatialAudioClient / RenderStream / SpatialAudioObject)
// ---------------------------------------------------------------------------

// ISpatialAudioClient vtable slots (0..=10)
const SAC_GET_STATIC_POS: usize = 3;
const SAC_GET_NATIVE_MASK: usize = 4;
const SAC_GET_MAX_DYN: usize = 5;
const SAC_GET_FMT_ENUM: usize = 6;
const SAC_GET_MAX_FRAMES: usize = 7;
const SAC_IS_FMT_SUPPORTED: usize = 8;
const SAC_IS_STREAM_AVAIL: usize = 9;
const SAC_ACTIVATE_STREAM: usize = 10;

// IAudioFormatEnumerator vtable slots (0..=4)
const FE_GET_COUNT: usize = 3;
const FE_GET_FORMAT: usize = 4;

// ISpatialAudioObjectRenderStream vtable slots (0..=10)
const SRS_GET_AVAIL_DYN: usize = 3;
const SRS_GET_SERVICE: usize = 4;
const SRS_START: usize = 5;
const SRS_STOP: usize = 6;
const SRS_RESET: usize = 7;
const SRS_BEGIN_UPDATE: usize = 8;
const SRS_END_UPDATE: usize = 9;
const SRS_ACTIVATE_OBJ: usize = 10;

// ISpatialAudioObject vtable slots (0..=8)
const SO_GET_BUFFER: usize = 3;
const SO_SET_EOS: usize = 4;
const SO_IS_ACTIVE: usize = 5;
const SO_GET_TYPE: usize = 6;
const SO_SET_POSITION: usize = 7;
const SO_SET_VOLUME: usize = 8;

/// Call `ISpatialAudioObject::SetPosition` (slot 7) via the real COM vtable
/// slot, exercising the `win64_bridge!` XMM1/XMM2/XMM3 adapter on aarch64.
unsafe fn spatial_set_position(obj: P, x: f32, y: f32, z: f32) -> HRESULT {
    #[cfg(target_arch = "aarch64")]
    {
        let br: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) -> HRESULT = std::mem::transmute(slot(obj, SO_SET_POSITION));
        br(
            obj as u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            f64::from_bits(x.to_bits() as u64),
            f64::from_bits(y.to_bits() as u64),
            f64::from_bits(z.to_bits() as u64),
        )
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        call!(obj, SO_SET_POSITION, (f32, f32, f32) -> HRESULT; x, y, z)
    }
}

/// Call `ISpatialAudioObject::SetVolume` (slot 8) via the real COM vtable slot,
/// exercising the `win64_bridge!` XMM1 adapter on aarch64.
unsafe fn spatial_set_volume(obj: P, vol: f32) -> HRESULT {
    #[cfg(target_arch = "aarch64")]
    {
        let br: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) -> HRESULT = std::mem::transmute(slot(obj, SO_SET_VOLUME));
        br(
            obj as u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            f64::from_bits(vol.to_bits() as u64),
            0.0,
            0.0,
        )
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        call!(obj, SO_SET_VOLUME, (f32) -> HRESULT; vol)
    }
}

/// Build a 24-byte `VT_BLOB` `PROPVARIANT` pointing to `params`.
fn blob_propvariant<T>(params: &T) -> [u8; 24] {
    let mut pv = [0u8; 24];
    pv[0..2].copy_from_slice(&VT_BLOB.to_le_bytes());
    pv[8..12].copy_from_slice(&(std::mem::size_of::<T>() as u32).to_le_bytes());
    pv[16..24].copy_from_slice(&(params as *const T as usize).to_le_bytes());
    pv
}

#[test]
fn spatial_audio_client_enumeration_positions_and_validation() {
    use super::spatial::*;
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        // Capture endpoints reject ISpatialAudioClient honestly.
        let mut cap_dev: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, E_CONSOLE, &mut cap_dev), S_OK);
        let mut bad_sac: P = 1 as P;
        assert_eq!(
            call!(cap_dev, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_ISpatialAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut bad_sac),
            AUDCLNT_E_WRONG_ENDPOINT_TYPE
        );
        assert!(bad_sac.is_null());
        release(cap_dev);

        // Activate ISpatialAudioClient on the render endpoint.
        let mut sac: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_ISpatialAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut sac),
            S_OK
        );
        assert!(!sac.is_null());

        let (hr, sac_qi) = qi(sac, &IID_ISpatialAudioClient);
        assert_eq!(hr, S_OK);
        assert_eq!(sac_qi, sac);
        assert_eq!(release(sac_qi), 1);
        assert_eq!(qi(sac, &IID_IAudioClient).0, E_NOINTERFACE);

        // Native static mask covers full 7.1.4 bed.
        let mut mask = 0u32;
        assert_eq!(call!(sac, SAC_GET_NATIVE_MASK, (*mut u32) -> HRESULT; &mut mask), S_OK);
        assert_eq!(mask, NATIVE_STATIC_MASK);
        assert_ne!(mask & AudioObjectType_FrontLeft, 0);
        assert_ne!(mask & AudioObjectType_FrontRight, 0);
        assert_ne!(mask & AudioObjectType_FrontCenter, 0);
        assert_ne!(mask & AudioObjectType_LowFrequency, 0);
        assert_ne!(mask & AudioObjectType_TopFrontLeft, 0);
        assert_ne!(mask & AudioObjectType_TopBackRight, 0);

        // Max dynamic objects == 128.
        let mut max_dyn = 0u32;
        assert_eq!(call!(sac, SAC_GET_MAX_DYN, (*mut u32) -> HRESULT; &mut max_dyn), S_OK);
        assert_eq!(max_dyn, MAX_DYNAMIC_OBJECTS);

        // Static object 3D positions (right-handed: +X right, +Y up, -Z front).
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_FrontLeft, &mut x, &mut y, &mut z), S_OK);
        assert!(x < 0.0 && y == 0.0 && z < 0.0, "FrontLeft=({x},{y},{z})");
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_FrontRight, &mut x, &mut y, &mut z), S_OK);
        assert!(x > 0.0 && y == 0.0 && z < 0.0, "FrontRight=({x},{y},{z})");
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_FrontCenter, &mut x, &mut y, &mut z), S_OK);
        assert!(x == 0.0 && y == 0.0 && z < 0.0, "FrontCenter=({x},{y},{z})");
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_TopFrontLeft, &mut x, &mut y, &mut z), S_OK);
        assert!(x < 0.0 && y > 0.0 && z < 0.0, "TopFrontLeft=({x},{y},{z})");
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_BackRight, &mut x, &mut y, &mut z), S_OK);
        assert!(x > 0.0 && y == 0.0 && z > 0.0, "BackRight=({x},{y},{z})");
        // Invalid types rejected.
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_Dynamic, &mut x, &mut y, &mut z), E_INVALIDARG);
        assert_eq!(call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_None, &mut x, &mut y, &mut z), E_INVALIDARG);
        assert_eq!(
            call!(sac, SAC_GET_STATIC_POS, (u32, *mut f32, *mut f32, *mut f32) -> HRESULT; AudioObjectType_FrontLeft | AudioObjectType_FrontRight, &mut x, &mut y, &mut z),
            E_INVALIDARG
        );

        // IAudioFormatEnumerator returns 48 kHz mono 32-bit IEEE float.
        let mut fe: P = std::ptr::null_mut();
        assert_eq!(call!(sac, SAC_GET_FMT_ENUM, (*mut P) -> HRESULT; &mut fe), S_OK);
        assert!(!fe.is_null());
        let (hr, fe_qi) = qi(fe, &IID_IAudioFormatEnumerator);
        assert_eq!(hr, S_OK);
        assert_eq!(release(fe_qi), 1);

        let mut fmt_count = 0u32;
        assert_eq!(call!(fe, FE_GET_COUNT, (*mut u32) -> HRESULT; &mut fmt_count), S_OK);
        assert_eq!(fmt_count, 1);

        let mut wfx: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(fe, FE_GET_FORMAT, (u32, *mut *mut WAVEFORMATEX) -> HRESULT; 0, &mut wfx), S_OK);
        assert!(!wfx.is_null());
        let f = std::ptr::read_unaligned(wfx);
        assert_eq!({ f.wFormatTag }, super::format::WAVE_FORMAT_IEEE_FLOAT);
        assert_eq!({ f.nChannels }, 1);
        assert_eq!({ f.nSamplesPerSec }, 48_000);
        assert_eq!({ f.wBitsPerSample }, 32);
        assert_eq!({ f.nBlockAlign }, 4);
        assert_eq!({ f.nAvgBytesPerSec }, 48_000 * 4);
        assert_eq!({ f.cbSize }, 0);

        let mut bad_wfx: *mut WAVEFORMATEX = 1 as *mut WAVEFORMATEX;
        assert_eq!(call!(fe, FE_GET_FORMAT, (u32, *mut *mut WAVEFORMATEX) -> HRESULT; 1, &mut bad_wfx), E_INVALIDARG);
        assert!(bad_wfx.is_null());

        // Format support & GetMaxFrameCount (plain IEEE float + WAVEFORMATEXTENSIBLE).
        assert_eq!(call!(sac, SAC_IS_FMT_SUPPORTED, (*const WAVEFORMATEX) -> HRESULT; wfx), S_OK);
        let mut max_frames = 0u32;
        assert_eq!(call!(sac, SAC_GET_MAX_FRAMES, (*const WAVEFORMATEX, *mut u32) -> HRESULT; wfx, &mut max_frames), S_OK);
        assert_eq!(max_frames, QUANTUM_FRAMES);

        let ext = super::format::WaveFormat::float_mix(48_000, 1, 0).to_extensible();
        assert_eq!(call!(sac, SAC_IS_FMT_SUPPORTED, (*const WAVEFORMATEX) -> HRESULT; &ext.Format), S_OK);

        let stereo = super::format::WaveFormat::float_mix(48_000, 2, 3).to_extensible();
        assert_eq!(call!(sac, SAC_IS_FMT_SUPPORTED, (*const WAVEFORMATEX) -> HRESULT; &stereo.Format), AUDCLNT_E_UNSUPPORTED_FORMAT);
        assert_eq!(call!(sac, SAC_GET_MAX_FRAMES, (*const WAVEFORMATEX, *mut u32) -> HRESULT; &stereo.Format, &mut max_frames), AUDCLNT_E_UNSUPPORTED_FORMAT);

        let pcm16 = WAVEFORMATEX {
            wFormatTag: 1,
            nChannels: 1,
            nSamplesPerSec: 48_000,
            nAvgBytesPerSec: 96_000,
            nBlockAlign: 2,
            wBitsPerSample: 16,
            cbSize: 0,
        };
        assert_eq!(call!(sac, SAC_IS_FMT_SUPPORTED, (*const WAVEFORMATEX) -> HRESULT; &pcm16), AUDCLNT_E_UNSUPPORTED_FORMAT);

        // Stream availability checks.
        assert_eq!(
            call!(sac, SAC_IS_STREAM_AVAIL, (*const GUID, *const u8) -> HRESULT; &IID_ISpatialAudioObjectRenderStream, std::ptr::null()),
            S_OK
        );
        assert_eq!(
            call!(sac, SAC_IS_STREAM_AVAIL, (*const GUID, *const u8) -> HRESULT; &IID_IAudioClient, std::ptr::null()),
            SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE
        );

        CoTaskMemFree(wfx as P);
        assert_eq!(release(fe), 0);
        assert_eq!(release(sac), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn spatial_audio_mix_quantum_static_and_dynamic_panning() {
    use super::spatial::*;
    let _g = super::audio_test_lock();
    let frames = QUANTUM_FRAMES as usize;
    let dc = vec![1.0f32; frames];
    let rms_lr = |stereo: &[f32]| -> (f32, f32) {
        let mut sl = 0.0f64;
        let mut sr = 0.0f64;
        for ch in stereo.chunks_exact(2) {
            sl += (ch[0] as f64) * (ch[0] as f64);
            sr += (ch[1] as f64) * (ch[1] as f64);
        }
        let n = (stereo.len() / 2) as f64;
        ((sl / n).sqrt() as f32, (sr / n).sqrt() as f32)
    };

    // 1. Static bed channels: FrontLeft, FrontRight, FrontCenter, Side/Back/Top.
    let fl = mix_quantum(&[SpatialSource { object_type: AudioObjectType_FrontLeft, position: [0.0; 3], volume: 1.0, samples: dc.clone() }], frames);
    assert_eq!(fl.len(), frames * 2);
    assert!((fl[0] - 1.0).abs() < 1e-6 && fl[1].abs() < 1e-6, "FrontLeft=({}, {})", fl[0], fl[1]);

    let fr = mix_quantum(&[SpatialSource { object_type: AudioObjectType_FrontRight, position: [0.0; 3], volume: 1.0, samples: dc.clone() }], frames);
    assert!(fr[0].abs() < 1e-6 && (fr[1] - 1.0).abs() < 1e-6, "FrontRight=({}, {})", fr[0], fr[1]);

    let fc = mix_quantum(&[SpatialSource { object_type: AudioObjectType_FrontCenter, position: [0.0; 3], volume: 1.0, samples: dc.clone() }], frames);
    assert!((fc[0] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-5);
    assert!((fc[0] - fc[1]).abs() < 1e-6, "FrontCenter=({}, {})", fc[0], fc[1]);

    let tfl = mix_quantum(&[SpatialSource { object_type: AudioObjectType_TopFrontLeft, position: [0.0; 3], volume: 1.0, samples: dc.clone() }], frames);
    assert!(tfl[0] > 0.7 && tfl[1] == 0.0);
    let br = mix_quantum(&[SpatialSource { object_type: AudioObjectType_BackRight, position: [0.0; 3], volume: 1.0, samples: dc.clone() }], frames);
    assert!(br[0] == 0.0 && br[1] > 0.7);

    // 2. Dynamic 3D objects: hard left (-1,0,0), hard right (+1,0,0), center (0,0,-1).
    let dyn_l = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [-1.0, 0.0, 0.0], volume: 1.0, samples: dc.clone() }], frames);
    let (l, r) = rms_lr(&dyn_l);
    assert!(l > 0.99 && r < 1e-5, "hard left RMS=({l}, {r})");

    let dyn_r = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [1.0, 0.0, 0.0], volume: 1.0, samples: dc.clone() }], frames);
    let (l, r) = rms_lr(&dyn_r);
    assert!(l < 1e-5 && r > 0.99, "hard right RMS=({l}, {r})");

    let dyn_c = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [0.0, 0.0, -1.0], volume: 1.0, samples: dc.clone() }], frames);
    let (l, r) = rms_lr(&dyn_c);
    assert!((l - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-5 && (l - r).abs() < 1e-6, "center RMS=({l}, {r})");

    // 3. Intermediate azimuth + subtle ITD timing cue on an impulse at sample 0.
    let mut impulse = vec![0.0f32; frames];
    impulse[0] = 1.0;
    let imp_r = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [0.5, 0.0, -0.866_025_4], volume: 1.0, samples: impulse.clone() }], frames);
    let peak_l = (0..frames).max_by(|&a, &b| imp_r[a * 2].abs().total_cmp(&imp_r[b * 2].abs())).unwrap();
    let peak_r = (0..frames).max_by(|&a, &b| imp_r[a * 2 + 1].abs().total_cmp(&imp_r[b * 2 + 1].abs())).unwrap();
    assert_eq!(peak_r, 0, "ipsilateral right ear should have zero ITD delay");
    assert!((12..=20).contains(&peak_l), "contralateral left ear ITD peak expected ~15-16 samples, got {peak_l}");
    assert!(imp_r[peak_r * 2 + 1] > imp_r[peak_l * 2], "right ear should be louder than left ear");

    let imp_l = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [-0.5, 0.0, -0.866_025_4], volume: 1.0, samples: impulse }], frames);
    let peak_l2 = (0..frames).max_by(|&a, &b| imp_l[a * 2].abs().total_cmp(&imp_l[b * 2].abs())).unwrap();
    let peak_r2 = (0..frames).max_by(|&a, &b| imp_l[a * 2 + 1].abs().total_cmp(&imp_l[b * 2 + 1].abs())).unwrap();
    assert_eq!(peak_l2, 0);
    assert_eq!(peak_r2, peak_l, "ITD must be left/right mirror symmetric");

    // 4. Distance attenuation & volume scaling.
    let near = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [0.0, 0.0, -1.0], volume: 1.0, samples: dc.clone() }], frames);
    let far = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [0.0, 0.0, -4.0], volume: 1.0, samples: dc.clone() }], frames);
    assert!((far[0] * 4.0 - near[0]).abs() < 1e-5, "inverse distance attenuation at 4m: near={}, far={}", near[0], far[0]);

    let half_vol = mix_quantum(&[SpatialSource { object_type: AudioObjectType_Dynamic, position: [0.0, 0.0, -1.0], volume: 0.5, samples: dc.clone() }], frames);
    assert!((half_vol[0] * 2.0 - near[0]).abs() < 1e-5);

    // 5. Combined static + dynamic mix sums linearly and clamps safely.
    let combo = mix_quantum(
        &[
            SpatialSource { object_type: AudioObjectType_FrontLeft, position: [0.0; 3], volume: 0.4, samples: dc.clone() },
            SpatialSource { object_type: AudioObjectType_Dynamic, position: [1.0, 0.0, 0.0], volume: 0.6, samples: dc },
        ],
        frames,
    );
    assert!((combo[0] - 0.4).abs() < 1e-5 && (combo[1] - 0.6).abs() < 1e-5, "combo=({}, {})", combo[0], combo[1]);
}

#[test]
fn spatial_audio_render_stream_end_to_end_and_lifecycle() {
    use super::spatial::*;
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        let mut sac: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_ISpatialAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut sac),
            S_OK
        );

        let mut fe: P = std::ptr::null_mut();
        assert_eq!(call!(sac, SAC_GET_FMT_ENUM, (*mut P) -> HRESULT; &mut fe), S_OK);
        let mut wfx: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(fe, FE_GET_FORMAT, (u32, *mut *mut WAVEFORMATEX) -> HRESULT; 0, &mut wfx), S_OK);

        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        assert!(!ev.is_null());

        let params = SpatialAudioObjectRenderStreamActivationParams {
            object_format: wfx,
            static_object_type_mask: AudioObjectType_FrontLeft | AudioObjectType_FrontRight | AudioObjectType_FrontCenter,
            min_dynamic_object_count: 0,
            max_dynamic_object_count: MAX_DYNAMIC_OBJECTS,
            category: 6, // AudioCategory_GameEffects
            event_handle: ev,
            notify_object: std::ptr::null_mut(),
        };
        let pv = blob_propvariant(&params);

        let mut stream: P = std::ptr::null_mut();
        assert_eq!(
            call!(sac, SAC_ACTIVATE_STREAM, (*const u8, *const GUID, *mut P) -> HRESULT; pv.as_ptr(), &IID_ISpatialAudioObjectRenderStream, &mut stream),
            S_OK
        );
        assert!(!stream.is_null());

        // Second active stream on same client -> AUDCLNT_E_ALREADY_INITIALIZED.
        let mut dup_stream: P = 1 as P;
        assert_eq!(
            call!(sac, SAC_ACTIVATE_STREAM, (*const u8, *const GUID, *mut P) -> HRESULT; pv.as_ptr(), &IID_ISpatialAudioObjectRenderStream, &mut dup_stream),
            AUDCLNT_E_ALREADY_INITIALIZED
        );
        assert!(dup_stream.is_null());

        // QI for base & derived stream IIDs; GetService for unsupported service -> E_NOINTERFACE.
        let (hr, base_s) = qi(stream, &IID_ISpatialAudioObjectRenderStreamBase);
        assert_eq!(hr, S_OK);
        assert_eq!(base_s, stream);
        assert_eq!(release(base_s), 1);
        let mut svc: P = 1 as P;
        assert_eq!(call!(stream, SRS_GET_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioEndpointVolume, &mut svc), E_NOINTERFACE);
        assert!(svc.is_null());

        let mut avail = 0u32;
        assert_eq!(call!(stream, SRS_GET_AVAIL_DYN, (*mut u32) -> HRESULT; &mut avail), S_OK);
        assert_eq!(avail, MAX_DYNAMIC_OBJECTS);

        // Out-of-order checks before BeginUpdatingAudioObjects.
        assert_eq!(call!(stream, SRS_END_UPDATE, () -> HRESULT;), SPTLAUDCLNT_E_OUT_OF_ORDER);
        let mut early_obj: P = 1 as P;
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, &mut early_obj), SPTLAUDCLNT_E_OUT_OF_ORDER);
        assert!(early_obj.is_null());

        // Start stream; Start again or Reset while running must return SPTLAUDCLNT_E_STREAM_NOT_STOPPED.
        assert_eq!(call!(stream, SRS_START, () -> HRESULT;), S_OK);
        assert_eq!(call!(stream, SRS_START, () -> HRESULT;), SPTLAUDCLNT_E_STREAM_NOT_STOPPED);
        assert_eq!(call!(stream, SRS_RESET, () -> HRESULT;), SPTLAUDCLNT_E_STREAM_NOT_STOPPED);

        // Quantum 0: immediate wakeup from Start(); activate static + dynamic + EOS dynamic objects.
        assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 500), WAIT_OBJECT_0);
        let mut frame_count = 0u32;
        assert_eq!(call!(stream, SRS_BEGIN_UPDATE, (*mut u32, *mut u32) -> HRESULT; &mut avail, &mut frame_count), S_OK);
        assert_eq!(avail, MAX_DYNAMIC_OBJECTS);
        assert_eq!(frame_count, QUANTUM_FRAMES);
        // Double BeginUpdatingAudioObjects -> SPTLAUDCLNT_E_OUT_OF_ORDER.
        assert_eq!(call!(stream, SRS_BEGIN_UPDATE, (*mut u32, *mut u32) -> HRESULT; &mut avail, &mut frame_count), SPTLAUDCLNT_E_OUT_OF_ORDER);

        // Activate static FrontLeft object.
        let mut obj_fl: P = std::ptr::null_mut();
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_FrontLeft, &mut obj_fl), S_OK);
        // Duplicate static channel fails with SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE; unrequested static fails with STATIC_OBJECT_NOT_AVAILABLE.
        let mut dup_fl: P = 1 as P;
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_FrontLeft, &mut dup_fl), SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE);
        assert!(dup_fl.is_null());
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_BackLeft, &mut dup_fl), SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE);

        // Static object properties: QI, type, active, SetPosition rejected, SetVolume accepted.
        let (hr, obj_base) = qi(obj_fl, &IID_ISpatialAudioObjectBase);
        assert_eq!(hr, S_OK);
        assert_eq!(release(obj_base), 1);
        let mut ot = 0u32;
        assert_eq!(call!(obj_fl, SO_GET_TYPE, (*mut u32) -> HRESULT; &mut ot), S_OK);
        assert_eq!(ot, AudioObjectType_FrontLeft);
        let mut active: BOOL = FALSE;
        assert_eq!(call!(obj_fl, SO_IS_ACTIVE, (*mut BOOL) -> HRESULT; &mut active), S_OK);
        assert_eq!(active, TRUE);
        assert_eq!(spatial_set_position(obj_fl, 1.0, 0.0, 0.0), SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED);
        assert_eq!(spatial_set_volume(obj_fl, 0.5), S_OK);

        let (mut fl_buf, mut fl_len) = (std::ptr::null_mut::<u8>(), 0u32);
        assert_eq!(call!(obj_fl, SO_GET_BUFFER, (*mut *mut u8, *mut u32) -> HRESULT; &mut fl_buf, &mut fl_len), S_OK);
        assert_eq!(fl_len, QUANTUM_FRAMES * 4);
        std::slice::from_raw_parts_mut(fl_buf as *mut f32, QUANTUM_FRAMES as usize).fill(0.5);

        // Activate dynamic object hard right (+1, 0, 0) at volume 0.8 (via win64_bridge! on aarch64).
        let mut obj_dyn: P = std::ptr::null_mut();
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, &mut obj_dyn), S_OK);
        assert_eq!(spatial_set_position(obj_dyn, 1.0, 0.0, 0.0), S_OK);
        assert_eq!(spatial_set_volume(obj_dyn, 0.8), S_OK);
        assert_eq!(spatial_set_volume(obj_dyn, 1.5), E_INVALIDARG);
        assert_eq!(spatial_set_volume(obj_dyn, -0.1), E_INVALIDARG);
        assert_eq!(spatial_set_position(obj_dyn, f32::NAN, 0.0, 0.0), E_INVALIDARG);
        let (mut dyn_buf, mut dyn_len) = (std::ptr::null_mut::<u8>(), 0u32);
        assert_eq!(call!(obj_dyn, SO_GET_BUFFER, (*mut *mut u8, *mut u32) -> HRESULT; &mut dyn_buf, &mut dyn_len), S_OK);
        assert_eq!(dyn_len, QUANTUM_FRAMES * 4);
        std::slice::from_raw_parts_mut(dyn_buf as *mut f32, QUANTUM_FRAMES as usize).fill(0.5);

        // Activate a second dynamic object centered at (0, 0, -1), write 120 frames, SetEndOfStream(120) and Release BEFORE EndUpdatingAudioObjects.
        let mut obj_eos: P = std::ptr::null_mut();
        assert_eq!(call!(stream, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, &mut obj_eos), S_OK);
        assert_eq!(spatial_set_position(obj_eos, 0.0, 0.0, -1.0), S_OK);
        assert_eq!(spatial_set_volume(obj_eos, 1.0), S_OK);
        let (mut eos_buf, mut eos_len) = (std::ptr::null_mut::<u8>(), 0u32);
        assert_eq!(call!(obj_eos, SO_GET_BUFFER, (*mut *mut u8, *mut u32) -> HRESULT; &mut eos_buf, &mut eos_len), S_OK);
        std::slice::from_raw_parts_mut(eos_buf as *mut f32, 120).fill(0.2);
        assert_eq!(call!(obj_eos, SO_SET_EOS, (u32) -> HRESULT; 120), S_OK);
        assert_eq!(release(obj_eos), 0);

        assert_eq!(call!(stream, SRS_END_UPDATE, () -> HRESULT;), S_OK);

        // Verify the mixed stereo buffer matches exact static FL + dynamic hard-right + center EOS mix!
        let stream_core = SpatialStreamObj::inner(stream as *mut SpatialStreamObj).clone();
        let mixed = stream_core.last_mixed_quantum();
        assert_eq!(mixed.len(), QUANTUM_FRAMES as usize * 2);
        let c_eos = 0.2 * std::f32::consts::FRAC_1_SQRT_2;
        // Sample 0: L = 0.5*0.5 (FL) + c_eos, R = 0.5*0.8 (dyn hard right) + c_eos.
        assert!((mixed[0] - (0.25 + c_eos)).abs() < 1e-4, "L[0]={} expected {}", mixed[0], 0.25 + c_eos);
        assert!((mixed[1] - (0.40 + c_eos)).abs() < 1e-4, "R[0]={} expected {}", mixed[1], 0.40 + c_eos);
        // Sample 200 (after EOS frame 120): L = 0.25 (FL only), R = 0.40 (dyn hard right only).
        assert!((mixed[200 * 2] - 0.25).abs() < 1e-4, "L[200]={}", mixed[200 * 2]);
        assert!((mixed[200 * 2 + 1] - 0.40).abs() < 1e-4, "R[200]={}", mixed[200 * 2 + 1]);

        // EOS object was freed after EndUpdatingAudioObjects -> 127 dynamic slots available.
        assert_eq!(call!(stream, SRS_GET_AVAIL_DYN, (*mut u32) -> HRESULT; &mut avail), S_OK);
        assert_eq!(avail, MAX_DYNAMIC_OBJECTS - 1);

        // Wait for 4 subsequent 10 ms quanta driven by the live Null OutputStream clock.
        for _ in 0..4 {
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 500), WAIT_OBJECT_0);
            assert_eq!(call!(stream, SRS_BEGIN_UPDATE, (*mut u32, *mut u32) -> HRESULT; &mut avail, &mut frame_count), S_OK);
            assert_eq!(frame_count, QUANTUM_FRAMES);
            assert_eq!(call!(stream, SRS_END_UPDATE, () -> HRESULT;), S_OK);
        }
        assert!(stream_core.shared().callbacks.load(Ordering::Relaxed) >= 3, "OutputStream clock callbacks never ran");
        drop(stream_core);

        // Stop + Reset deactivates live objects cleanly.
        assert_eq!(call!(stream, SRS_STOP, () -> HRESULT;), S_OK);
        assert_eq!(call!(stream, SRS_STOP, () -> HRESULT;), S_FALSE);
        assert_eq!(call!(stream, SRS_RESET, () -> HRESULT;), S_OK);
        assert_eq!(call!(obj_fl, SO_IS_ACTIVE, (*mut BOOL) -> HRESULT; &mut active), S_OK);
        assert_eq!(active, FALSE);
        assert_eq!(call!(obj_dyn, SO_IS_ACTIVE, (*mut BOOL) -> HRESULT; &mut active), S_OK);
        assert_eq!(active, FALSE);

        assert_eq!(release(obj_fl), 0);
        assert_eq!(release(obj_dyn), 0);
        assert_eq!(release(stream), 0);

        // Now that the first stream is released, activate a capped stream (max_dynamic = 3)
        // to test SPTLAUDCLNT_E_NO_MORE_OBJECTS exhaustion and recovery on Release.
        let small_params = SpatialAudioObjectRenderStreamActivationParams {
            object_format: wfx,
            static_object_type_mask: 0,
            min_dynamic_object_count: 1,
            max_dynamic_object_count: 3,
            category: 6,
            event_handle: ev,
            notify_object: std::ptr::null_mut(),
        };
        let small_pv = blob_propvariant(&small_params);
        let mut s2: P = std::ptr::null_mut();
        assert_eq!(
            call!(sac, SAC_ACTIVATE_STREAM, (*const u8, *const GUID, *mut P) -> HRESULT; small_pv.as_ptr(), &IID_ISpatialAudioObjectRenderStream, &mut s2),
            S_OK
        );
        assert_eq!(call!(s2, SRS_BEGIN_UPDATE, (*mut u32, *mut u32) -> HRESULT; &mut avail, &mut frame_count), S_OK);
        assert_eq!(avail, 3);
        let mut objs = [std::ptr::null_mut::<c_void>(); 3];
        for o in &mut objs {
            assert_eq!(call!(s2, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, o), S_OK);
        }
        let mut overflow: P = 1 as P;
        assert_eq!(call!(s2, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, &mut overflow), SPTLAUDCLNT_E_NO_MORE_OBJECTS);
        assert!(overflow.is_null());
        // Release one unwritten dynamic object -> slot frees immediately.
        assert_eq!(release(objs[0]), 0);
        assert_eq!(call!(s2, SRS_ACTIVATE_OBJ, (u32, *mut P) -> HRESULT; AudioObjectType_Dynamic, &mut objs[0]), S_OK);
        assert_eq!(call!(s2, SRS_END_UPDATE, () -> HRESULT;), S_OK);
        for o in objs {
            assert_eq!(release(o), 0);
        }
        assert_eq!(release(s2), 0);

        CoTaskMemFree(wfx as P);
        crate::kernel32::threads::CloseHandle(ev);
        assert_eq!(release(fe), 0);
        assert_eq!(release(sac), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn wasapi_services_buffer_limits_and_loopback_capture() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        // 1. IAudioClient2::GetBufferSizeLimits validates format & returns shared-mode bounds.
        let render_c = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        let mut min_hns = 0i64;
        let mut max_hns = 0i64;
        assert_eq!(
            call!(render_c, 17, (*const WAVEFORMATEX, BOOL, *mut i64, *mut i64) -> HRESULT; mix, FALSE, &mut min_hns, &mut max_hns),
            S_OK
        );
        assert_eq!(min_hns, super::audioclient::DEFAULT_PERIOD_HNS);
        assert_eq!(max_hns, super::audioclient::MAX_BUFFER_DURATION_HNS);
        assert_eq!(
            call!(render_c, 17, (*const WAVEFORMATEX, BOOL, *mut i64, *mut i64) -> HRESULT; mix, TRUE, &mut min_hns, &mut max_hns),
            S_OK
        );
        assert_eq!(min_hns, super::audioclient::MIN_PERIOD_HNS);
        assert_eq!(
            call!(render_c, 17, (*const WAVEFORMATEX, BOOL, *mut i64, *mut i64) -> HRESULT; std::ptr::null(), FALSE, &mut min_hns, &mut max_hns),
            E_POINTER
        );

        // 2. Initialize render stream and verify GetService for ISimpleAudioVolume, IAudioStreamVolume,
        //    IAudioSessionControl2, IAudioClock2, plus unknown IID -> E_NOINTERFACE.
        assert_eq!(
            call!(render_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);

        let mut sv: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_ISimpleAudioVolume, &mut sv), S_OK);
        assert_eq!(call_bridge_f32_guid(sv, 3, 0.5f32, std::ptr::null()), S_OK);

        let mut stv: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioStreamVolume, &mut stv), S_OK);
        let mut ch_cnt = 0u32;
        assert_eq!(call!(stv, 3, (*mut u32) -> HRESULT; &mut ch_cnt), S_OK);
        assert_eq!(ch_cnt, 2);
        let vols = [1.0f32, 0.5f32];
        assert_eq!(call!(stv, 6, (u32, *const f32) -> HRESULT; 2, vols.as_ptr()), S_OK);
        let mut got_vols = [0.0f32; 2];
        assert_eq!(call!(stv, 7, (u32, *mut f32) -> HRESULT; 2, got_vols.as_mut_ptr()), S_OK);
        assert_eq!(got_vols, vols);

        let mut sc: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioSessionControl2, &mut sc), S_OK);
        let mut pid = 0u32;
        assert_eq!(call!(sc, 14, (*mut u32) -> HRESULT; &mut pid), S_OK);
        assert_eq!(pid, std::process::id());

        // Documented Windows acquisition path: GetService(IID_IAudioClock) -> QI(IID_IAudioClock2),
        // with full COM symmetry (QI back to IAudioClock) and identical IUnknown identity.
        let mut ck: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock, &mut ck), S_OK);
        let (hr_ck2, ck2_via_qi) = qi(ck, &IID_IAudioClock2);
        assert_eq!(hr_ck2, S_OK);
        assert!(!ck2_via_qi.is_null());
        let mut dpos_qi = 0u64;
        let mut qpc_qi = 0u64;
        assert_eq!(call!(ck2_via_qi, 3, (*mut u64, *mut u64) -> HRESULT; &mut dpos_qi, &mut qpc_qi), S_OK);
        assert!(qpc_qi > 0);
        let (hr_back, ck_back) = qi(ck2_via_qi, &IID_IAudioClock);
        assert_eq!(hr_back, S_OK);
        assert_eq!(ck_back, ck);
        let (hr_u1, u1) = qi(ck, &IID_IUnknown);
        let (hr_u2, u2) = qi(ck2_via_qi, &IID_IUnknown);
        assert_eq!((hr_u1, hr_u2), (S_OK, S_OK));
        assert_eq!(u1, u2);
        assert_eq!(release(u2), 4);
        assert_eq!(release(u1), 3);
        assert_eq!(release(ck_back), 2);
        assert_eq!(release(ck2_via_qi), 1);
        assert_eq!(release(ck), 0);

        // Direct GetService(IID_IAudioClock2) also remains supported.
        let mut ck2: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioClock2, &mut ck2), S_OK);
        let mut dpos = 0u64;
        let mut qpc = 0u64;
        assert_eq!(call!(ck2, 3, (*mut u64, *mut u64) -> HRESULT; &mut dpos, &mut qpc), S_OK);
        assert!(qpc > 0);

        let bogus = GUID::new(0xAAAA_BBBB, 0xCCCC, 0xDDDD, [1, 2, 3, 4, 5, 6, 7, 8]);
        let mut missing: P = 1 as P;
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &bogus, &mut missing), E_NOINTERFACE);
        assert!(missing.is_null());

        // 3. Open a loopback capture client on the SAME render endpoint and verify rendered
        //    audio (with ISimpleAudioVolume=0.5 and IAudioStreamVolume=[1.0, 0.5]) arrives intact.
        let lb_c = activate_client(d);
        assert_eq!(
            call!(lb_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);

        // Submit 64 frames of (0.8, 0.4) to render client -> effective output should be (0.4, 0.1).
        // Before Start, prefilled frames must NOT appear in loopback capture.
        let mut rp: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        let samples = std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2);
        for pair in samples.chunks_exact_mut(2) {
            pair[0] = 0.8;
            pair[1] = 0.4;
        }
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        std::thread::sleep(std::time::Duration::from_millis(25));
        assert_no_live_loopback_audio(cc, "stopped prefilled render stream must not feed loopback");

        // Starting playback consumes the 64 frames and delivers them with applied gains.
        assert_eq!(call!(render_c, AC_START, () -> HRESULT;), S_OK);
        let got_f32 = wait_loopback_live_f32(cc, 64, 2);
        assert_eq!(got_f32.len(), 64 * 2);
        assert!((got_f32[0] - 0.4).abs() < 1e-4, "L={} expected 0.4", got_f32[0]);
        assert!((got_f32[1] - 0.1).abs() < 1e-4, "R={} expected 0.1", got_f32[1]);

        // Loopback on a capture endpoint still fails with AUDCLNT_E_WRONG_ENDPOINT_TYPE.
        let mut cap_dev: P = std::ptr::null_mut();
        assert_eq!(call!(e, EN_DEFAULT, (u32, u32, *mut P) -> HRESULT; E_CAPTURE, E_CONSOLE, &mut cap_dev), S_OK);
        let cap_c = activate_client(cap_dev);
        assert_eq!(
            call!(cap_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            AUDCLNT_E_WRONG_ENDPOINT_TYPE
        );

        // 4. Spatial stream GetService supports IAudioClock, IAudioClock2, ISimpleAudioVolume, IAudioStreamVolume, IAudioSessionControl2.
        let mut sac: P = std::ptr::null_mut();
        assert_eq!(call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_ISpatialAudioClient, CLSCTX_ALL, std::ptr::null_mut(), &mut sac), S_OK);
        let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
        let obj_fmt = super::spatial::preferred_object_format();
        let sp_params = super::spatial::SpatialAudioObjectRenderStreamActivationParams {
            object_format: &obj_fmt,
            static_object_type_mask: super::spatial::AudioObjectType_FrontLeft | super::spatial::AudioObjectType_FrontRight,
            min_dynamic_object_count: 0,
            max_dynamic_object_count: 2,
            category: 6,
            event_handle: ev,
            notify_object: std::ptr::null_mut(),
        };
        let mut pv = [0u8; 24];
        (pv.as_mut_ptr() as *mut u16).write_unaligned(VT_BLOB);
        (pv.as_mut_ptr().add(8) as *mut u32).write_unaligned(std::mem::size_of_val(&sp_params) as u32);
        (pv.as_mut_ptr().add(16) as *mut usize).write_unaligned(&sp_params as *const _ as usize);
        let mut sp_stream: P = std::ptr::null_mut();
        assert_eq!(call!(sac, 10, (*const u8, *const GUID, *mut P) -> HRESULT; pv.as_ptr(), &super::spatial::IID_ISpatialAudioObjectRenderStream, &mut sp_stream), S_OK);
        for iid in [IID_IAudioClock, IID_IAudioClock2, IID_ISimpleAudioVolume, IID_IAudioStreamVolume, IID_IAudioSessionControl2] {
            let mut s: P = std::ptr::null_mut();
            assert_eq!(call!(sp_stream, 4, (*const GUID, *mut P) -> HRESULT; &iid, &mut s), S_OK, "spatial GetService({iid:?})");
            assert!(!s.is_null());
            if iid == IID_IAudioClock {
                let (hr_sck2, sck2) = qi(s, &IID_IAudioClock2);
                assert_eq!(hr_sck2, S_OK);
                let mut sp_pos = 0u64;
                let mut sp_qpc = 0u64;
                assert_eq!(call!(sck2, 3, (*mut u64, *mut u64) -> HRESULT; &mut sp_pos, &mut sp_qpc), S_OK);
                assert!(sp_qpc > 0);
                assert_eq!(release(sck2), 1);
            }
            assert_eq!(release(s), 0);
        }
        assert_eq!(release(sp_stream), 0);
        crate::kernel32::threads::CloseHandle(ev);
        assert_eq!(release(sac), 0);

        CoTaskMemFree(mix as P);
        assert_eq!(release(cap_c), 0);
        assert_eq!(release(cap_dev), 0);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb_c), 0);
        assert_eq!(release(ck2), 0);
        assert_eq!(release(sc), 0);
        assert_eq!(release(stv), 0);
        assert_eq!(release(sv), 0);
        assert_eq!(release(rc), 0);
        assert_eq!(release(render_c), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

/// Call a 4-arg COM vtable slot `(this, u32, f32, *const GUID) -> HRESULT` via the win64 bridge.
unsafe fn call_bridge_u32_f32_guid(obj: P, idx: usize, ch: u32, val: f32, ctx: *const GUID) -> HRESULT {
    #[cfg(target_arch = "aarch64")]
    {
        let br: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) -> HRESULT = std::mem::transmute(slot(obj, idx));
        br(
            obj as u64,
            ch as u64,
            0,
            ctx as u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            0.0,
            f64::from_bits(val.to_bits() as u64),
            0.0,
        )
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        call!(obj, idx, (u32, f32, *const GUID) -> HRESULT; ch, val, ctx)
    }
}

#[test]
fn endpoint_channel_volume_attenuates_playback_and_tap_converter_normalizes_format() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        // Activate IAudioEndpointVolume. First set master to -6.0206 dB (exact 0.5 amplitude),
        // then re-set channel 1 to the exact scalar its getter already reports (must NOT double-attenuate),
        // and set channel 0 to 0.0 (silence).
        const MINUS_6DB: f32 = -6.020_6;
        let mut epv: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioEndpointVolume, CLSCTX_ALL, std::ptr::null_mut(), &mut epv),
            S_OK
        );
        assert_eq!(call_bridge_f32_guid(epv, 6 /* SetMasterVolumeLevel */, MINUS_6DB, std::ptr::null()), S_OK);
        let mut master_db = 0.0f32;
        assert_eq!(call!(epv, 8 /* GetMasterVolumeLevel */, (*mut f32) -> HRESULT; &mut master_db), S_OK);
        assert!((master_db - MINUS_6DB).abs() < 1e-3, "GetMasterVolumeLevel={master_db}");
        let mut ch1_reported_scalar = -1.0f32;
        assert_eq!(call!(epv, 13 /* GetChannelVolumeLevelScalar */, (u32, *mut f32) -> HRESULT; 1, &mut ch1_reported_scalar), S_OK);
        // Re-applying channel 1's reported scalar must keep channel 1 at -6.0206 dB (0.5 amplitude),
        // NOT multiply master and channel gains into 0.25!
        assert_eq!(call_bridge_u32_f32_guid(epv, 11 /* SetChannelVolumeLevelScalar */, 1, ch1_reported_scalar, std::ptr::null()), S_OK);
        assert_eq!(call_bridge_u32_f32_guid(epv, 11 /* SetChannelVolumeLevelScalar */, 0, 0.0f32, std::ptr::null()), S_OK);

        let mut got0 = -1.0f32;
        let mut got1 = -1.0f32;
        assert_eq!(call!(epv, 13 /* GetChannelVolumeLevelScalar */, (u32, *mut f32) -> HRESULT; 0, &mut got0), S_OK);
        assert_eq!(call!(epv, 13 /* GetChannelVolumeLevelScalar */, (u32, *mut f32) -> HRESULT; 1, &mut got1), S_OK);
        assert!((got0 - 0.0).abs() < 1e-6);
        assert!((got1 - ch1_reported_scalar).abs() < 1e-6);

        // A second IAudioEndpointVolume activation on the same endpoint must observe the shared state
        // and receive IAudioEndpointVolumeCallback::OnNotify with the caller's event-context GUID.
        let mut epv2: P = std::ptr::null_mut();
        assert_eq!(
            call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioEndpointVolume, CLSCTX_ALL, std::ptr::null_mut(), &mut epv2),
            S_OK
        );
        #[repr(C)]
        struct NotifyData {
            guid: GUID,
            muted: i32,
            master: f32,
            n_ch: u32,
            ch_vols: [f32; 8],
        }
        #[repr(C)]
        struct TestVolCb {
            vtbl: *const [usize; 4],
            refs: AtomicU32,
            calls: usize,
            epv_to_unregister: P,
            refs_after_reentrant_unregister: u32,
            last_guid: GUID,
            last_muted: i32,
            last_master: f32,
            last_ch1: f32,
        }
        unsafe extern "win64" fn test_vol_add_ref(this: *mut TestVolCb) -> u32 {
            (*this).refs.fetch_add(1, Ordering::SeqCst) + 1
        }
        unsafe extern "win64" fn test_vol_release(this: *mut TestVolCb) -> u32 {
            (*this).refs.fetch_sub(1, Ordering::SeqCst) - 1
        }
        unsafe extern "win64" fn test_vol_on_notify(this: *mut TestVolCb, data: *const NotifyData) -> HRESULT {
            let cb = &mut *this;
            let d = std::ptr::read_unaligned(data);
            cb.calls += 1;
            cb.last_guid = d.guid;
            cb.last_muted = d.muted;
            cb.last_master = d.master;
            cb.last_ch1 = d.ch_vols[1];
            // If armed, unregister mid-dispatch and assert the registration's COM reference
            // stays alive until `OnNotify` finishes (no premature Release / UAF).
            if !cb.epv_to_unregister.is_null() {
                let epv_ptr = cb.epv_to_unregister;
                cb.epv_to_unregister = std::ptr::null_mut();
                let hr = call!(epv_ptr, 4 /* UnregisterControlChangeNotify */, (*mut c_void) -> HRESULT; this as *mut c_void);
                assert_eq!(hr, S_OK);
                cb.refs_after_reentrant_unregister = cb.refs.load(Ordering::SeqCst);
            }
            S_OK
        }
        crate::register_native_va(test_vol_add_ref as *const () as u64, test_vol_add_ref as *const () as usize);
        crate::register_native_va(test_vol_release as *const () as u64, test_vol_release as *const () as usize);
        crate::register_native_va(test_vol_on_notify as *const () as u64, test_vol_on_notify as *const () as usize);
        let cb_vtbl: [usize; 4] = [
            0,
            test_vol_add_ref as *const () as usize,
            test_vol_release as *const () as usize,
            test_vol_on_notify as *const () as usize,
        ];
        let mut cb_obj = TestVolCb {
            vtbl: &cb_vtbl,
            refs: AtomicU32::new(1),
            calls: 0,
            epv_to_unregister: std::ptr::null_mut(),
            refs_after_reentrant_unregister: 0,
            last_guid: GUID::new(0, 0, 0, [0; 8]),
            last_muted: -1,
            last_master: -1.0,
            last_ch1: -1.0,
        };
        assert_eq!(call!(epv2, 3 /* RegisterControlChangeNotify */, (*mut c_void) -> HRESULT; &mut cb_obj as *mut TestVolCb as *mut c_void), S_OK);
        assert_eq!(cb_obj.refs.load(Ordering::SeqCst), 2, "RegisterControlChangeNotify must AddRef the callback");
        let ctx_guid = GUID::new(0x1234_5678, 0x9ABC, 0xDEF0, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut got0_2 = -1.0f32;
        let mut got1_2 = -1.0f32;
        assert_eq!(call!(epv2, 13, (u32, *mut f32) -> HRESULT; 0, &mut got0_2), S_OK);
        assert_eq!(call!(epv2, 13, (u32, *mut f32) -> HRESULT; 1, &mut got1_2), S_OK);
        assert!((got0_2 - 0.0).abs() < 1e-6);
        assert!((got1_2 - ch1_reported_scalar).abs() < 1e-6);
        // Arm re-entrant unregistration during dispatch: callback refcount must stay >= 2
        // throughout `OnNotify` and drop back to 1 once dispatch completes.
        cb_obj.epv_to_unregister = epv2;
        assert_eq!(call!(epv, 14 /* SetMute */, (i32, *const GUID) -> HRESULT; 1, &ctx_guid), S_OK);
        assert_eq!(cb_obj.calls, 1, "OnNotify must fire when mute changes on another activation");
        assert_eq!(cb_obj.last_guid, ctx_guid);
        assert_eq!(cb_obj.last_muted, 1);
        assert!((cb_obj.last_ch1 - ch1_reported_scalar).abs() < 1e-6);
        assert_eq!(
            cb_obj.refs_after_reentrant_unregister, 2,
            "in-flight dispatch must retain callback COM ref until OnNotify returns"
        );
        assert_eq!(
            cb_obj.refs.load(Ordering::SeqCst),
            1,
            "COM ref must be released once both unregister and in-flight OnNotify finish"
        );
        let mut muted2 = 0i32;
        assert_eq!(call!(epv2, 15 /* GetMute */, (*mut i32) -> HRESULT; &mut muted2), S_OK);
        assert_eq!(muted2, 1, "second activation must observe mute set via first activation");
        assert_eq!(call!(epv2, 14 /* SetMute */, (i32, *const GUID) -> HRESULT; 0, std::ptr::null()), S_OK);
        assert_eq!(cb_obj.calls, 1, "unregistered callback must not fire");
        assert_eq!(release(epv2), 0);

        // Render (0.8, 0.8) and verify loopback sees (0.0, 0.4) — proving endpoint channel gains
        // actually affect rendered playback rather than sitting in private COM state.
        let render_c = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        assert_eq!(
            call!(render_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);

        let lb_c = activate_client(d);
        assert_eq!(
            call!(lb_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);

        let mut rp: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 32, &mut rp), S_OK);
        let samples = std::slice::from_raw_parts_mut(rp as *mut f32, 32 * 2);
        samples.fill(0.8);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 32, 0), S_OK);
        assert_eq!(call!(render_c, AC_START, () -> HRESULT;), S_OK);

        let got = wait_loopback_live_f32(cc, 32, 2);
        assert_eq!(got.len(), 32 * 2);
        assert!(got[0].abs() < 1e-5, "channel 0 must be silenced by SetChannelVolumeLevelScalar(0, 0.0), got {}", got[0]);
        assert!((got[1] - 0.4).abs() < 1e-3, "channel 1 must be attenuated by -6.0206 dB (0.5x -> 0.4, not double-attenuated or linear scalar), got {}", got[1]);

        // Also verify audio-tapered scalar 0.5 (-32.625 dB -> ~0.023375 amplitude) attenuates via dB, not linear 0.5.
        assert_eq!(call_bridge_u32_f32_guid(epv, 11 /* SetChannelVolumeLevelScalar */, 1, 0.5f32, std::ptr::null()), S_OK);
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 32, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 32 * 2).fill(0.8);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 32, 0), S_OK);
        let got_taper = wait_loopback_live_f32(cc, 32, 2);
        let want_taper = 0.8 * 10.0f32.powf(-32.625 / 20.0);
        assert!(
            (got_taper[1] - want_taper).abs() < 1e-3,
            "audio-tapered scalar 0.5 must attenuate by -32.625 dB ({want_taper}), got {}",
            got_taper[1]
        );

        // Restore master/channel gains to 1.0 so other tests start clean.
        assert_eq!(call_bridge_f32_guid(epv, 7 /* SetMasterVolumeLevelScalar */, 1.0f32, std::ptr::null()), S_OK);

        CoTaskMemFree(mix as P);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb_c), 0);
        assert_eq!(release(rc), 0);
        assert_eq!(release(render_c), 0);
        assert_eq!(release(epv), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }

    // Verify TapFormatConverter handles stereo 48 kHz -> mono 48 kHz downmix and 48 kHz -> 44.1 kHz resampling.
    let mut downmix = super::backend::TapFormatConverter::new(48_000, 2, 48_000, 1);
    let stereo_in = [0.2f32, 0.6, 1.0, 0.0, -0.4, 0.4];
    let mono_out = downmix.convert(&stereo_in);
    assert_eq!(mono_out.len(), 3);
    assert!((mono_out[0] - 0.4).abs() < 1e-6);
    assert!((mono_out[1] - 0.5).abs() < 1e-6);
    assert!(mono_out[2].abs() < 1e-6);

    let mut resample = super::backend::TapFormatConverter::new(48_000, 2, 44_100, 1);
    let mut stereo_48k = Vec::with_capacity(480 * 2);
    for _ in 0..480 {
        stereo_48k.push(0.6);
        stereo_48k.push(0.2);
    }
    let mono_44k = resample.convert(&stereo_48k);
    // 480 frames @ 48 kHz = 10 ms -> 441 frames @ 44.1 kHz
    assert_eq!(mono_44k.len(), 441);
    assert!(mono_44k.iter().all(|&s| (s - 0.4).abs() < 1e-5));
}

#[test]
fn loopback_bus_serializes_concurrent_producers_and_resamples_fallback_rate() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        // 1. Create a 44.1 kHz mono loopback capture client (differs from 48 kHz stereo render).
        let lb_c = activate_client(d);
        let mono_44k = WAVEFORMATEX {
            wFormatTag: super::format::WAVE_FORMAT_IEEE_FLOAT,
            nChannels: 1,
            nSamplesPerSec: 44_100,
            nAvgBytesPerSec: 44_100 * 4,
            nBlockAlign: 4,
            wBitsPerSample: 32,
            cbSize: 0,
        };
        assert_eq!(
            call!(
                lb_c,
                AC_INIT,
                (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT;
                SHARED,
                super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK | super::audioclient::AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM,
                1_000_000,
                0,
                &mono_44k,
                std::ptr::null()
            ),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);

        // 2. Push 480 frames @ 48 kHz stereo from two concurrent render threads into the same endpoint.
        let render_c1 = activate_client(d);
        let render_c2 = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(render_c1, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        assert_eq!(
            call!(render_c1, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 1_000_000, 0, mix, std::ptr::null()),
            S_OK
        );
        assert_eq!(
            call!(render_c2, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 1_000_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut rc1: P = std::ptr::null_mut();
        let mut rc2: P = std::ptr::null_mut();
        assert_eq!(call!(render_c1, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc1), S_OK);
        assert_eq!(call!(render_c2, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc2), S_OK);
        assert_eq!(call!(render_c1, AC_START, () -> HRESULT;), S_OK);
        assert_eq!(call!(render_c2, AC_START, () -> HRESULT;), S_OK);

        let rc1_addr = rc1 as usize;
        let rc2_addr = rc2 as usize;
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let b1 = barrier.clone();
        let t1 = std::thread::spawn(move || {
            let rc = rc1_addr as P;
            b1.wait();
            for _ in 0..4 {
                let mut ptr: *mut u8 = std::ptr::null_mut();
                assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 480, &mut ptr), S_OK);
                let slice = std::slice::from_raw_parts_mut(ptr as *mut f32, 480 * 2);
                slice.fill(0.3);
                assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 480, 0), S_OK);
            }
        });
        let b2 = barrier.clone();
        let t2 = std::thread::spawn(move || {
            let rc = rc2_addr as P;
            b2.wait();
            for _ in 0..4 {
                let mut ptr: *mut u8 = std::ptr::null_mut();
                assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 480, &mut ptr), S_OK);
                let slice = std::slice::from_raw_parts_mut(ptr as *mut f32, 480 * 2);
                slice.fill(0.4);
                assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 480, 0), S_OK);
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        // Wait for both started render streams to finish playing all 1920 frames on their
        // playback timelines before draining the mixed loopback feed.
        wait_render_drained(render_c1);
        wait_render_drained(render_c2);

        // Drain all mixed + resampled 44.1 kHz mono packets: two simultaneous 40 ms streams
        // (4 * 480 frames @ 48 kHz each) must mix on a shared 40 ms timeline -> 1764 frames
        // @ 44.1 kHz (NOT 3528 concatenated frames), with combined amplitude 0.3 + 0.4 = 0.7.
        let mut total_frames = 0u32;
        let mut samples = Vec::new();
        loop {
            let mut next = 0u32;
            assert_eq!(call!(cc, 5 /* GetNextPacketSize */, (*mut u32) -> HRESULT; &mut next), S_OK);
            if next == 0 {
                break;
            }
            let mut p: *mut u8 = std::ptr::null_mut();
            let mut nf = 0u32;
            let mut fl = 0u32;
            let mut dp = 0u64;
            let mut qpc = 0u64;
            let hr = call!(cc, CC_GET, (*mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> HRESULT; &mut p, &mut nf, &mut fl, &mut dp, &mut qpc);
            assert_eq!(hr, S_OK);
            if fl & super::audioclient::AUDCLNT_BUFFERFLAGS_SILENT == 0 {
                let s = std::slice::from_raw_parts(p as *const f32, nf as usize);
                samples.extend_from_slice(s);
                total_frames += nf;
            }
            assert_eq!(call!(cc, CC_RELEASE, (u32) -> HRESULT; nf), S_OK);
        }
        assert_eq!(
            total_frames, 1764,
            "two simultaneous 40 ms streams @ 48 kHz must mix to 1764 frames @ 44.1 kHz (shared timeline)"
        );
        assert!(
            samples.iter().all(|&v| (v - 0.7).abs() < 1e-4),
            "mixed loopback amplitude must be 0.3 + 0.4 = 0.7, got {:?}",
            &samples[..samples.len().min(8)]
        );

        CoTaskMemFree(mix as P);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb_c), 0);
        assert_eq!(release(rc1), 0);
        assert_eq!(release(rc2), 0);
        assert_eq!(release(render_c1), 0);
        assert_eq!(release(render_c2), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn loopback_capture_gates_arrival_on_started_state_and_discards_stopped_audio() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        let render_c = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        assert_eq!(
            call!(render_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut rc: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);

        // Initialize loopback capture client WITHOUT calling Start yet.
        let lb_c = activate_client(d);
        assert_eq!(
            call!(lb_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);

        // 1. Prefill a STOPPED render stream while loopback IS started, then Reset it:
        //    unplayed / reset audio must never leak into loopback capture.
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);
        let mut rp: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2).fill(0.6);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        std::thread::sleep(std::time::Duration::from_millis(25));
        assert_no_live_loopback_audio(cc, "prefilled unstarted render stream must not publish to loopback");
        assert_eq!(call!(render_c, AC_RESET, () -> HRESULT;), S_OK);
        assert_eq!(call!(render_c, AC_START, () -> HRESULT;), S_OK);
        std::thread::sleep(std::time::Duration::from_millis(25));
        assert_no_live_loopback_audio(cc, "reset render frames must never publish after Start");
        assert_eq!(call!(lb_c, AC_STOP, () -> HRESULT;), S_OK);
        assert_eq!(call!(lb_c, AC_RESET, () -> HRESULT;), S_OK);

        // 2. Play audio while loopback client is STOPPED: polling must return 0 frames.
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2).fill(0.9);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        wait_render_drained(render_c);

        let mut next = 99u32;
        assert_eq!(call!(cc, 5 /* GetNextPacketSize */, (*mut u32) -> HRESULT; &mut next), S_OK);
        assert_eq!(next, 0, "stopped loopback client must not enqueue audio when polled");

        // Starting loopback must NOT deliver stale audio played prior to Start.
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);
        assert_eq!(call!(cc, 5, (*mut u32) -> HRESULT; &mut next), S_OK);
        assert_eq!(next, 0, "Start must discard audio played while stopped");

        // 3. Play audio while loopback is started, wait for playback consumption, then Stop and drain.
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 32, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 32 * 2).fill(0.25);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 32, 0), S_OK);
        wait_render_drained(render_c);
        assert_eq!(call!(lb_c, AC_STOP, () -> HRESULT;), S_OK);

        let got = wait_loopback_live_f32(cc, 32, 2);
        assert_eq!(got.len(), 32 * 2);
        assert!((got[0] - 0.25).abs() < 1e-4);

        // 4. Play MORE audio while stopped + wait past a packet period: polling must stay at 0
        //    (neither live audio nor re-armed silent packets may arrive while stopped).
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2).fill(0.75);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        wait_render_drained(render_c);
        std::thread::sleep(std::time::Duration::from_millis(25));
        assert_eq!(call!(cc, 5, (*mut u32) -> HRESULT; &mut next), S_OK);
        assert_eq!(next, 0, "stopped loopback client must not enqueue audio or silent packets");

        // Restarting must NOT deliver the 0.75 audio played while stopped.
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);
        assert_eq!(call!(cc, 5, (*mut u32) -> HRESULT; &mut next), S_OK);
        assert_eq!(next, 0, "restarting loopback must not deliver audio from stopped interval");

        CoTaskMemFree(mix as P);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb_c), 0);
        assert_eq!(release(rc), 0);
        assert_eq!(release(render_c), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn shared_audio_session_shares_volume_and_mute_across_clients_and_manager() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        let session_a = GUID::new(0xAAAA_1111, 0x2222, 0x3333, [1, 2, 3, 4, 5, 6, 7, 8]);
        let session_b = GUID::new(0xBBBB_4444, 0x5555, 0x6666, [8, 7, 6, 5, 4, 3, 2, 1]);

        let c1 = activate_client(d);
        let c2 = activate_client(d);
        let c3 = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(c1, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);

        // c1 and c2 join session_a; c3 joins session_b.
        assert_eq!(call!(c1, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, &session_a), S_OK);
        assert_eq!(call!(c2, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, &session_a), S_OK);
        assert_eq!(call!(c3, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, &session_b), S_OK);

        let mut vol1: P = std::ptr::null_mut();
        let mut vol2: P = std::ptr::null_mut();
        let mut vol3: P = std::ptr::null_mut();
        assert_eq!(call!(c1, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_ISimpleAudioVolume, &mut vol1), S_OK);
        assert_eq!(call!(c2, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_ISimpleAudioVolume, &mut vol2), S_OK);
        assert_eq!(call!(c3, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_ISimpleAudioVolume, &mut vol3), S_OK);

        // Activate IAudioSessionManager on the endpoint and open session_a's ISimpleAudioVolume.
        let mut mgr: P = std::ptr::null_mut();
        assert_eq!(call!(d, DEV_ACTIVATE, (*const GUID, u32, *mut c_void, *mut P) -> HRESULT; &IID_IAudioSessionManager, CLSCTX_ALL, std::ptr::null_mut(), &mut mgr), S_OK);
        let mut mgr_vol_a: P = std::ptr::null_mut();
        assert_eq!(call!(mgr, 4 /* GetSimpleAudioVolume */, (*const GUID, u32, *mut P) -> HRESULT; &session_a, 0, &mut mgr_vol_a), S_OK);

        // Changing volume via c1 must immediately reflect in c2 and IAudioSessionManager, not c3.
        assert_eq!(call_bridge_f32_guid(vol1, 3 /* SetMasterVolume */, 0.25f32, std::ptr::null()), S_OK);
        let mut v2 = -1.0f32;
        let mut vm = -1.0f32;
        let mut v3 = -1.0f32;
        assert_eq!(call!(vol2, 4 /* GetMasterVolume */, (*mut f32) -> HRESULT; &mut v2), S_OK);
        assert_eq!(call!(mgr_vol_a, 4 /* GetMasterVolume */, (*mut f32) -> HRESULT; &mut vm), S_OK);
        assert_eq!(call!(vol3, 4 /* GetMasterVolume */, (*mut f32) -> HRESULT; &mut v3), S_OK);
        assert!((v2 - 0.25).abs() < 1e-6);
        assert!((vm - 0.25).abs() < 1e-6);
        assert!((v3 - 1.0).abs() < 1e-6);

        // Start a loopback capture and verify c2 renders at session_a's 0.25 gain, then silences when c1 mutes.
        let mut rc2: P = std::ptr::null_mut();
        assert_eq!(call!(c2, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc2), S_OK);
        assert_eq!(call!(c2, AC_START, () -> HRESULT;), S_OK);
        let lb = activate_client(d);
        assert_eq!(
            call!(lb, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(lb, AC_START, () -> HRESULT;), S_OK);

        let mut rp: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc2, RC_GET, (u32, *mut *mut u8) -> HRESULT; 32, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 32 * 2).fill(0.8);
        assert_eq!(call!(rc2, RC_RELEASE, (u32, u32) -> HRESULT; 32, 0), S_OK);

        let got = wait_loopback_live_f32(cc, 32, 2);
        assert_eq!(got.len(), 32 * 2);
        assert!((got[0] - 0.2).abs() < 1e-4, "c2 must observe c1's 0.25 session volume: got {}", got[0]);

        // Mute session_a via c1 -> c2's GetMute == 1 and c2's submitted audio is silenced; c3 stays unmuted.
        assert_eq!(call!(vol1, 5 /* SetMute */, (i32, *const GUID) -> HRESULT; 1, std::ptr::null()), S_OK);
        let mut m2 = 0i32;
        let mut mm = 0i32;
        let mut m3 = 1i32;
        assert_eq!(call!(vol2, 6 /* GetMute */, (*mut i32) -> HRESULT; &mut m2), S_OK);
        assert_eq!(call!(mgr_vol_a, 6 /* GetMute */, (*mut i32) -> HRESULT; &mut mm), S_OK);
        assert_eq!(call!(vol3, 6 /* GetMute */, (*mut i32) -> HRESULT; &mut m3), S_OK);
        assert_eq!(m2, 1);
        assert_eq!(mm, 1);
        assert_eq!(m3, 0);

        assert_eq!(call!(rc2, RC_GET, (u32, *mut *mut u8) -> HRESULT; 32, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 32 * 2).fill(0.8);
        assert_eq!(call!(rc2, RC_RELEASE, (u32, u32) -> HRESULT; 32, 0), S_OK);
        wait_render_drained(c2);
        assert_no_live_loopback_audio(cc, "muted session_a must silence c2 render submission");

        CoTaskMemFree(mix as P);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb), 0);
        assert_eq!(release(rc2), 0);
        assert_eq!(release(mgr_vol_a), 0);
        assert_eq!(release(mgr), 0);
        assert_eq!(release(vol3), 0);
        assert_eq!(release(vol2), 0);
        assert_eq!(release(vol1), 0);
        assert_eq!(release(c3), 0);
        assert_eq!(release(c2), 0);
        assert_eq!(release(c1), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}

#[test]
fn session_and_stream_volume_apply_at_playback_consumption_for_queued_audio() {
    let _f = Fixture::null();
    unsafe {
        let e = enumerator();
        let d = default_device(e);

        let render_c = activate_client(d);
        let mut mix: *mut WAVEFORMATEX = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_MIXFMT, (*mut *mut WAVEFORMATEX) -> HRESULT; &mut mix), S_OK);
        let sess = GUID::new(0xCCDD_1122, 0x3344, 0x5566, [9, 8, 7, 6, 5, 4, 3, 2]);
        assert_eq!(
            call!(render_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, 0, 200_000, 0, mix, &sess),
            S_OK
        );
        let mut rc: P = std::ptr::null_mut();
        let mut sv: P = std::ptr::null_mut();
        let mut stv: P = std::ptr::null_mut();
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioRenderClient, &mut rc), S_OK);
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_ISimpleAudioVolume, &mut sv), S_OK);
        assert_eq!(call!(render_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioStreamVolume, &mut stv), S_OK);

        let lb_c = activate_client(d);
        assert_eq!(
            call!(lb_c, AC_INIT, (u32, u32, i64, i64, *const WAVEFORMATEX, *const GUID) -> HRESULT; SHARED, super::audioclient::AUDCLNT_STREAMFLAGS_LOOPBACK, 200_000, 0, mix, std::ptr::null()),
            S_OK
        );
        let mut cc: P = std::ptr::null_mut();
        assert_eq!(call!(lb_c, AC_SERVICE, (*const GUID, *mut P) -> HRESULT; &IID_IAudioCaptureClient, &mut cc), S_OK);
        assert_eq!(call!(lb_c, AC_START, () -> HRESULT;), S_OK);

        // 1. Prefill render buffer BEFORE calling SetMute(TRUE) and Start:
        //    queued audio must play muted, not unmuted.
        let mut rp: *mut u8 = std::ptr::null_mut();
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2).fill(0.8);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        assert_eq!(call!(sv, 5 /* SetMute */, (i32, *const GUID) -> HRESULT; 1, std::ptr::null()), S_OK);
        assert_eq!(call!(render_c, AC_START, () -> HRESULT;), S_OK);
        wait_render_drained(render_c);
        assert_no_live_loopback_audio(cc, "SetMute(TRUE) after prefill must mute already-queued audio");
        assert_eq!(call!(render_c, AC_STOP, () -> HRESULT;), S_OK);

        // 2. Prefill while MUTED, then unmute + adjust session & channel volumes BEFORE Start:
        //    queued samples must be preserved and scaled by the live gains at consumption time.
        assert_eq!(call!(rc, RC_GET, (u32, *mut *mut u8) -> HRESULT; 64, &mut rp), S_OK);
        std::slice::from_raw_parts_mut(rp as *mut f32, 64 * 2).fill(0.8);
        assert_eq!(call!(rc, RC_RELEASE, (u32, u32) -> HRESULT; 64, 0), S_OK);
        assert_eq!(call!(sv, 5 /* SetMute */, (i32, *const GUID) -> HRESULT; 0, std::ptr::null()), S_OK);
        assert_eq!(call_bridge_f32_guid(sv, 3 /* SetMasterVolume */, 0.5f32, std::ptr::null()), S_OK);
        let ch_vols = [1.0f32, 0.25f32];
        assert_eq!(call!(stv, 6 /* SetAllVolumes */, (u32, *const f32) -> HRESULT; 2, ch_vols.as_ptr()), S_OK);
        assert_eq!(call!(render_c, AC_START, () -> HRESULT;), S_OK);
        let got = wait_loopback_live_f32(cc, 64, 2);
        assert_eq!(got.len(), 64 * 2);
        assert!((got[0] - 0.4).abs() < 1e-4, "unmuted prefilled L must play at 0.8*0.5=0.4, got {}", got[0]);
        assert!((got[1] - 0.1).abs() < 1e-4, "unmuted prefilled R must play at 0.8*0.5*0.25=0.1, got {}", got[1]);

        CoTaskMemFree(mix as P);
        assert_eq!(release(cc), 0);
        assert_eq!(release(lb_c), 0);
        assert_eq!(release(stv), 0);
        assert_eq!(release(sv), 0);
        assert_eq!(release(rc), 0);
        assert_eq!(release(render_c), 0);
        assert_eq!(release(d), 0);
        assert_eq!(release(e), 0);
    }
}
