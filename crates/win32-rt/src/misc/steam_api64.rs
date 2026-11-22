//! steam_api64.dll (`steam_api64.dll`, `steam_api.dll`):
//! Valve Steamworks SDK Bridge for PC Games.
//!
//! Used by ~85% of top PC games (Elden Ring, Baldur's Gate 3, Counter-Strike 2,
//! Cyberpunk 2077, Black Myth: Wukong, Helldivers 2, Palworld, Monster Hunter, etc.).
//!
//! Two operating modes:
//! 1. **Native Apple Silicon Steamworks IPC bridge (preferred when macOS Steam is running):**
//!    Dynamically `dlopen`s Valve's official arm64 `steamclient.dylib` (from
//!    `PORT_STEAMCLIENT_DYLIB`, `~/Library/Application Support/Steam/...`, or
//!    `/Applications/Steam.app/...`) unless `PORT_STEAM_NATIVE=0`. Connects to the
//!    running macOS Steam client via `Steam_CreateSteamPipe` + `Steam_ConnectToGlobalUser`,
//!    exposes real `ISteamClient` / `ISteamUser` / `ISteamApps` / `ISteamFriends` /
//!    `ISteamUtils` / generic interfaces with Win64 MSVC ↔ ARM64 Itanium C++ ABI
//!    adaptation (including `CSteamID` hidden return-pointer conversion), and pumps
//!    genuine Steam callbacks and `SteamAPICall_t` results via `Steam_BGetCallback` /
//!    `Steam_GetAPICallResult`.
//! 2. **Offline local fallback (when macOS Steam is not running or `PORT_STEAM_NATIVE=0`):**
//!    Provides clean 80-slot COM-like vtables with accurate Steamworks SDK slot indices
//!    (`IsSubscribed=1`, `BLoggedOn=1`, `GetSteamID`, `GetPersonaName="Player"`,
//!    `GetCurrentGameLanguage="english"`, `GetConnectedUniverse=1`, etc.).
//!
//! Key guarantees in both modes:
//! * `SteamAPI_RestartAppIfNecessary(app_id)` ALWAYS returns `false` (`0`) so the
//!   game does not self-terminate trying to relaunch through the Windows Steam client.
//! * `SteamInternal_ContextInit` initializes per-module inline `CSteamAPIContext`
//!   blocks (checking `counter != flag`, populating the context pointer at `p[2]`,
//!   and invoking the game's callback fn at `p[0]` via `crate::call_guest8`).

use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_int, c_void, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Standard offline/local 64-bit SteamID (Individual user in Public universe).
const LOCAL_STEAM_ID: u64 = 0x0110_0001_0000_001A;
/// `SteamAPICallCompleted_t::k_iCallback`.
const K_I_CALLBACK_STEAM_API_CALL_COMPLETED: i32 = 703;
/// `CCallbackBase::k_ECallbackFlagsRegistered`.
const K_E_CALLBACK_FLAGS_REGISTERED: u8 = 0x01;

static ACTIVE_APP_ID: AtomicU32 = AtomicU32::new(480);
static CTX_INIT_COUNTER: AtomicUsize = AtomicUsize::new(1);

static NATIVE_ACTIVE: AtomicBool = AtomicBool::new(false);
static NATIVE_PIPE: AtomicI32 = AtomicI32::new(0);
static NATIVE_USER: AtomicI32 = AtomicI32::new(0);

static PERSONA_NAME: &[u8] = b"Player\0";
static GAME_LANGUAGE: &[u8] = b"english\0";
static AVAILABLE_LANGUAGES: &[u8] = b"english,french,german,spanish,japanese,korean,schinese\0";
static COUNTRY_CODE: &[u8] = b"US\0";
static EMPTY_STR: &[u8] = b"\0";

// ---------------------------------------------------------------------------
// Dynamic macOS arm64 steamclient.dylib loader & IPC session
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct NativeCallbackMsg {
    h_steam_user: i32,
    i_callback: i32,
    pub_param: *mut u8,
    cub_param: i32,
}

type FnCreateInterface = unsafe extern "C" fn(*const c_char, *mut c_int) -> *mut c_void;
type FnCreateSteamPipe = unsafe extern "C" fn() -> i32;
type FnBReleaseSteamPipe = unsafe extern "C" fn(i32) -> u8;
type FnConnectToGlobalUser = unsafe extern "C" fn(i32) -> i32;
type FnReleaseUser = unsafe extern "C" fn(i32, i32);
type FnBLoggedOn = unsafe extern "C" fn(i32, i32) -> u8;
type FnBGetCallback = unsafe extern "C" fn(i32, *mut NativeCallbackMsg) -> u8;
type FnFreeLastCallback = unsafe extern "C" fn(i32);
type FnGetAPICallResult =
    unsafe extern "C" fn(i32, u64, *mut c_void, c_int, c_int, *mut u8) -> u8;
type FnReleaseThreadLocalMemory = unsafe extern "C" fn(c_int);

struct NativeSteamLib {
    _dl_handle: *mut c_void,
    path: PathBuf,
    create_interface: FnCreateInterface,
    create_pipe: FnCreateSteamPipe,
    release_pipe: FnBReleaseSteamPipe,
    connect_global_user: FnConnectToGlobalUser,
    release_user: FnReleaseUser,
    b_logged_on: Option<FnBLoggedOn>,
    b_get_callback: FnBGetCallback,
    free_last_callback: FnFreeLastCallback,
    get_api_call_result: FnGetAPICallResult,
    release_thread_memory: Option<FnReleaseThreadLocalMemory>,
}

unsafe impl Send for NativeSteamLib {}
unsafe impl Sync for NativeSteamLib {}

struct NativeSteamSession {
    lib: &'static NativeSteamLib,
    pipe: i32,
    user: i32,
    native_client: *mut c_void,
    bridged_client: *mut c_void,
}

unsafe impl Send for NativeSteamSession {}

static NATIVE_LIB: OnceLock<Option<NativeSteamLib>> = OnceLock::new();
static NATIVE_SESSION: LazyLock<Mutex<Option<NativeSteamSession>>> = LazyLock::new(|| Mutex::new(None));

fn candidate_steamclient_paths() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(explicit) = std::env::var("PORT_STEAMCLIENT_DYLIB") {
        let trimmed = explicit.trim();
        if !trimmed.is_empty() {
            out.push(PathBuf::from(trimmed));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let h = PathBuf::from(home);
        out.push(
            h.join("Library")
                .join("Application Support")
                .join("Steam")
                .join("Steam.AppBundle")
                .join("Steam")
                .join("Contents")
                .join("MacOS")
                .join("steamclient.dylib"),
        );
    }
    out.push(PathBuf::from(
        "/Applications/Steam.app/Contents/MacOS/steamclient.dylib",
    ));
    out
}

unsafe fn dlsym_opt<T>(handle: *mut c_void, name: &[u8]) -> Option<T> {
    let ptr = libc::dlsym(handle, name.as_ptr() as *const c_char);
    if ptr.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy(&ptr))
    }
}

fn load_native_steam_lib() -> Option<&'static NativeSteamLib> {
    if std::env::var("PORT_STEAM_NATIVE").ok().as_deref() == Some("0") {
        return None;
    }
    NATIVE_LIB
        .get_or_init(|| unsafe {
            for path in candidate_steamclient_paths() {
                if !path.is_file() {
                    continue;
                }
                let Ok(cpath) = CString::new(path.to_string_lossy().as_bytes()) else {
                    continue;
                };
                let handle = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if handle.is_null() {
                    let err_p = libc::dlerror();
                    let err_s = if err_p.is_null() {
                        String::from("unknown dlopen error")
                    } else {
                        std::ffi::CStr::from_ptr(err_p).to_string_lossy().into_owned()
                    };
                    log::debug!(
                        "steam_api64: dlopen({}) failed: {err_s}",
                        path.display()
                    );
                    continue;
                }
                let (
                    Some(create_interface),
                    Some(create_pipe),
                    Some(release_pipe),
                    Some(connect_global_user),
                    Some(release_user),
                    Some(b_get_callback),
                    Some(free_last_callback),
                    Some(get_api_call_result),
                ) = (
                    dlsym_opt::<FnCreateInterface>(handle, b"CreateInterface\0"),
                    dlsym_opt::<FnCreateSteamPipe>(handle, b"Steam_CreateSteamPipe\0"),
                    dlsym_opt::<FnBReleaseSteamPipe>(handle, b"Steam_BReleaseSteamPipe\0"),
                    dlsym_opt::<FnConnectToGlobalUser>(handle, b"Steam_ConnectToGlobalUser\0"),
                    dlsym_opt::<FnReleaseUser>(handle, b"Steam_ReleaseUser\0"),
                    dlsym_opt::<FnBGetCallback>(handle, b"Steam_BGetCallback\0"),
                    dlsym_opt::<FnFreeLastCallback>(handle, b"Steam_FreeLastCallback\0"),
                    dlsym_opt::<FnGetAPICallResult>(handle, b"Steam_GetAPICallResult\0"),
                )
                else {
                    log::debug!(
                        "steam_api64: {} missing required Steam_* symbols",
                        path.display()
                    );
                    libc::dlclose(handle);
                    continue;
                };
                let b_logged_on = dlsym_opt::<FnBLoggedOn>(handle, b"Steam_BLoggedOn\0");
                let release_thread_memory = dlsym_opt::<FnReleaseThreadLocalMemory>(
                    handle,
                    b"Steam_ReleaseThreadLocalMemory\0",
                );
                log::info!(
                    "steam_api64: loaded native macOS steamclient.dylib from {}",
                    path.display()
                );
                return Some(NativeSteamLib {
                    _dl_handle: handle,
                    path,
                    create_interface,
                    create_pipe,
                    release_pipe,
                    connect_global_user,
                    release_user,
                    b_logged_on,
                    b_get_callback,
                    free_last_callback,
                    get_api_call_result,
                    release_thread_memory,
                });
            }
            None
        })
        .as_ref()
}

// ---------------------------------------------------------------------------
// Win64 MSVC <-> ARM64 Itanium C++ vtable bridge
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
enum SteamKind {
    Generic = 0,
    Client = 1,
    User = 2,
    Apps = 3,
    Friends = 4,
    Utils = 5,
}

impl SteamKind {
    const fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Client,
            2 => Self::User,
            3 => Self::Apps,
            4 => Self::Friends,
            5 => Self::Utils,
            _ => Self::Generic,
        }
    }

    fn from_version(ver: &str) -> Self {
        let lower = ver.to_ascii_lowercase();
        if lower.starts_with("steamclient") {
            Self::Client
        } else if lower.starts_with("steamapps") {
            Self::Apps
        } else if lower.starts_with("steamfriends") {
            Self::Friends
        } else if lower.starts_with("steamutils") {
            Self::Utils
        } else if lower.starts_with("steamuser") && !lower.starts_with("steamuserstats") {
            Self::User
        } else {
            Self::Generic
        }
    }
}

#[repr(C)]
struct GenericSteamVtbl {
    slots: [usize; 80],
}

#[repr(C)]
struct GenericSteamInterface {
    vtbl: *const GenericSteamVtbl,
}

#[repr(C)]
struct BridgedSteamObject {
    vtbl: *const GenericSteamVtbl,
    native: *mut c_void,
    kind: SteamKind,
}

struct BridgedCache {
    by_key: HashMap<(usize, SteamKind), usize>,
    all_ptrs: HashSet<usize>,
}

static BRIDGED_CACHE: LazyLock<Mutex<BridgedCache>> = LazyLock::new(|| {
    Mutex::new(BridgedCache {
        by_key: HashMap::new(),
        all_ptrs: HashSet::new(),
    })
});

unsafe fn is_bridged_ptr(p: *mut c_void) -> bool {
    if p.is_null() {
        return false;
    }
    BRIDGED_CACHE.lock().unwrap().all_ptrs.contains(&(p as usize))
}

unsafe extern "win64" fn bridge_slot<const K: usize>(
    this: *mut c_void,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void, u64, u64, u64, u64, u64, u64, u64) -> u64 =
        std::mem::transmute(*nvtbl.add(K));
    target(native, a1, a2, a3, a4, a5, a6, a7)
}

/// Win64 MSVC returns 8-byte `CSteamID` via hidden `out_ptr` in `rdx`, whereas ARM64 returns `u64` in `x0`.
unsafe extern "win64" fn bridge_csteamid_0<const K: usize>(
    this: *mut c_void,
    out_id: *mut u64,
) -> *mut u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void) -> u64 = std::mem::transmute(*nvtbl.add(K));
    let id = target(native);
    if !out_id.is_null() && (out_id as usize) > 0x10000 {
        std::ptr::write_unaligned(out_id, id);
        return out_id;
    }
    id as *mut u64
}

unsafe extern "win64" fn bridge_csteamid_1<const K: usize>(
    this: *mut c_void,
    out_id: *mut u64,
    a1: u64,
) -> *mut u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void, u64) -> u64 = std::mem::transmute(*nvtbl.add(K));
    let id = target(native, a1);
    if !out_id.is_null() && (out_id as usize) > 0x10000 {
        std::ptr::write_unaligned(out_id, id);
        return out_id;
    }
    id as *mut u64
}

unsafe extern "win64" fn bridge_csteamid_2<const K: usize>(
    this: *mut c_void,
    out_id: *mut u64,
    a1: u64,
    a2: u64,
) -> *mut u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void, u64, u64) -> u64 =
        std::mem::transmute(*nvtbl.add(K));
    let id = target(native, a1, a2);
    if !out_id.is_null() && (out_id as usize) > 0x10000 {
        std::ptr::write_unaligned(out_id, id);
        return out_id;
    }
    id as *mut u64
}

unsafe extern "C" fn native_warning_hook_trampoline(severity: c_int, text: *const c_char) {
    let msg = if text.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(text).to_string_lossy().into_owned()
    };
    log::debug!("steamclient[severity={severity}]: {msg}");
}

/// Intercept `SetWarningMessageHook` so native ARM64 Steam never jumps directly into x86-64 guest code.
unsafe extern "win64" fn bridge_set_warning_hook<const K: usize>(
    this: *mut c_void,
    guest_fn: usize,
) {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void, usize) = std::mem::transmute(*nvtbl.add(K));
    let native_cb = if guest_fn == 0 {
        0
    } else {
        native_warning_hook_trampoline as *const () as usize
    };
    target(native, native_cb);
}

/// Wrap `ISteamClient::GetISteam*` return values in matching `BridgedSteamObject` adapters.
unsafe extern "win64" fn bridge_client_child<const K: usize, const KIND: u8>(
    this: *mut c_void,
    a1: u64,
    a2: u64,
    a3: u64,
) -> *mut c_void {
    let obj = &*(this as *const BridgedSteamObject);
    let native = obj.native;
    let nvtbl = *(native as *const *const usize);
    let target: unsafe extern "C" fn(*mut c_void, u64, u64, u64) -> *mut c_void =
        std::mem::transmute(*nvtbl.add(K));
    let raw = target(native, a1, a2, a3);
    if raw.is_null() {
        return std::ptr::null_mut();
    }
    let kind = if K == 12 {
        // GetISteamGenericInterface(hUser, hPipe, pchVersion)
        let ver = crate::strings::from_ansi_opt(a3 as LPCSTR).unwrap_or_default();
        SteamKind::from_version(&ver)
    } else {
        SteamKind::from_u8(KIND)
    };
    wrap_native_interface(raw, kind)
}

macro_rules! bridge_slots_80 {
    () => {
        [
            bridge_slot::<0> as *const () as usize,
            bridge_slot::<1> as *const () as usize,
            bridge_slot::<2> as *const () as usize,
            bridge_slot::<3> as *const () as usize,
            bridge_slot::<4> as *const () as usize,
            bridge_slot::<5> as *const () as usize,
            bridge_slot::<6> as *const () as usize,
            bridge_slot::<7> as *const () as usize,
            bridge_slot::<8> as *const () as usize,
            bridge_slot::<9> as *const () as usize,
            bridge_slot::<10> as *const () as usize,
            bridge_slot::<11> as *const () as usize,
            bridge_slot::<12> as *const () as usize,
            bridge_slot::<13> as *const () as usize,
            bridge_slot::<14> as *const () as usize,
            bridge_slot::<15> as *const () as usize,
            bridge_slot::<16> as *const () as usize,
            bridge_slot::<17> as *const () as usize,
            bridge_slot::<18> as *const () as usize,
            bridge_slot::<19> as *const () as usize,
            bridge_slot::<20> as *const () as usize,
            bridge_slot::<21> as *const () as usize,
            bridge_slot::<22> as *const () as usize,
            bridge_slot::<23> as *const () as usize,
            bridge_slot::<24> as *const () as usize,
            bridge_slot::<25> as *const () as usize,
            bridge_slot::<26> as *const () as usize,
            bridge_slot::<27> as *const () as usize,
            bridge_slot::<28> as *const () as usize,
            bridge_slot::<29> as *const () as usize,
            bridge_slot::<30> as *const () as usize,
            bridge_slot::<31> as *const () as usize,
            bridge_slot::<32> as *const () as usize,
            bridge_slot::<33> as *const () as usize,
            bridge_slot::<34> as *const () as usize,
            bridge_slot::<35> as *const () as usize,
            bridge_slot::<36> as *const () as usize,
            bridge_slot::<37> as *const () as usize,
            bridge_slot::<38> as *const () as usize,
            bridge_slot::<39> as *const () as usize,
            bridge_slot::<40> as *const () as usize,
            bridge_slot::<41> as *const () as usize,
            bridge_slot::<42> as *const () as usize,
            bridge_slot::<43> as *const () as usize,
            bridge_slot::<44> as *const () as usize,
            bridge_slot::<45> as *const () as usize,
            bridge_slot::<46> as *const () as usize,
            bridge_slot::<47> as *const () as usize,
            bridge_slot::<48> as *const () as usize,
            bridge_slot::<49> as *const () as usize,
            bridge_slot::<50> as *const () as usize,
            bridge_slot::<51> as *const () as usize,
            bridge_slot::<52> as *const () as usize,
            bridge_slot::<53> as *const () as usize,
            bridge_slot::<54> as *const () as usize,
            bridge_slot::<55> as *const () as usize,
            bridge_slot::<56> as *const () as usize,
            bridge_slot::<57> as *const () as usize,
            bridge_slot::<58> as *const () as usize,
            bridge_slot::<59> as *const () as usize,
            bridge_slot::<60> as *const () as usize,
            bridge_slot::<61> as *const () as usize,
            bridge_slot::<62> as *const () as usize,
            bridge_slot::<63> as *const () as usize,
            bridge_slot::<64> as *const () as usize,
            bridge_slot::<65> as *const () as usize,
            bridge_slot::<66> as *const () as usize,
            bridge_slot::<67> as *const () as usize,
            bridge_slot::<68> as *const () as usize,
            bridge_slot::<69> as *const () as usize,
            bridge_slot::<70> as *const () as usize,
            bridge_slot::<71> as *const () as usize,
            bridge_slot::<72> as *const () as usize,
            bridge_slot::<73> as *const () as usize,
            bridge_slot::<74> as *const () as usize,
            bridge_slot::<75> as *const () as usize,
            bridge_slot::<76> as *const () as usize,
            bridge_slot::<77> as *const () as usize,
            bridge_slot::<78> as *const () as usize,
            bridge_slot::<79> as *const () as usize,
        ]
    };
}

unsafe fn register_static_vtbl(slots: [usize; 80]) -> &'static GenericSteamVtbl {
    for f in slots {
        crate::register_native_va(f as u64, f);
    }
    Box::leak(Box::new(GenericSteamVtbl { slots }))
}

unsafe fn bridged_vtbl_for(kind: SteamKind) -> &'static GenericSteamVtbl {
    match kind {
        SteamKind::Generic => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| register_static_vtbl(bridge_slots_80!()))
        }
        SteamKind::Client => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| {
                let mut slots = bridge_slots_80!();
                slots[5] = bridge_client_child::<5, { SteamKind::User as u8 }> as *const () as usize;
                slots[6] = bridge_client_child::<6, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[8] = bridge_client_child::<8, { SteamKind::Friends as u8 }> as *const () as usize;
                slots[9] = bridge_client_child::<9, { SteamKind::Utils as u8 }> as *const () as usize;
                slots[10] = bridge_client_child::<10, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[11] = bridge_client_child::<11, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[12] = bridge_client_child::<12, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[13] = bridge_client_child::<13, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[14] = bridge_client_child::<14, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[15] = bridge_client_child::<15, { SteamKind::Apps as u8 }> as *const () as usize;
                slots[16] = bridge_client_child::<16, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[17] = bridge_client_child::<17, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[18] = bridge_client_child::<18, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[19] = bridge_client_child::<19, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[22] = bridge_set_warning_hook::<22> as *const () as usize;
                slots[24] = bridge_client_child::<24, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[26] = bridge_client_child::<26, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[27] = bridge_client_child::<27, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[28] = bridge_client_child::<28, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[29] = bridge_client_child::<29, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[30] = bridge_client_child::<30, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[31] = bridge_client_child::<31, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[35] = bridge_client_child::<35, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[36] = bridge_client_child::<36, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[37] = bridge_client_child::<37, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[38] = bridge_client_child::<38, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[39] = bridge_client_child::<39, { SteamKind::Generic as u8 }> as *const () as usize;
                slots[40] = bridge_client_child::<40, { SteamKind::Generic as u8 }> as *const () as usize;
                register_static_vtbl(slots)
            })
        }
        SteamKind::User => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| {
                let mut slots = bridge_slots_80!();
                slots[2] = bridge_csteamid_0::<2> as *const () as usize; // GetSteamID
                register_static_vtbl(slots)
            })
        }
        SteamKind::Apps => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| {
                let mut slots = bridge_slots_80!();
                slots[20] = bridge_csteamid_0::<20> as *const () as usize; // GetAppOwner
                register_static_vtbl(slots)
            })
        }
        SteamKind::Friends => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| {
                let mut slots = bridge_slots_80!();
                slots[4] = bridge_csteamid_2::<4> as *const () as usize; // GetFriendByIndex
                slots[19] = bridge_csteamid_1::<19> as *const () as usize; // GetClanByIndex
                slots[25] = bridge_csteamid_2::<25> as *const () as usize; // GetFriendFromSourceByIndex
                slots[39] = bridge_csteamid_1::<39> as *const () as usize; // GetClanOwner
                slots[41] = bridge_csteamid_2::<41> as *const () as usize; // GetClanOfficerByIndex
                slots[51] = bridge_csteamid_1::<51> as *const () as usize; // GetCoplayFriend
                slots[57] = bridge_csteamid_2::<57> as *const () as usize; // GetChatMemberByIndex
                register_static_vtbl(slots)
            })
        }
        SteamKind::Utils => {
            static VTBL: OnceLock<&'static GenericSteamVtbl> = OnceLock::new();
            VTBL.get_or_init(|| {
                let mut slots = bridge_slots_80!();
                slots[16] = bridge_set_warning_hook::<16> as *const () as usize;
                register_static_vtbl(slots)
            })
        }
    }
}

unsafe fn wrap_native_interface(native: *mut c_void, kind: SteamKind) -> *mut c_void {
    if native.is_null() {
        return std::ptr::null_mut();
    }
    let mut cache = BRIDGED_CACHE.lock().unwrap();
    if let Some(&existing) = cache.by_key.get(&(native as usize, kind)) {
        return existing as *mut c_void;
    }
    let vtbl = bridged_vtbl_for(kind) as *const GenericSteamVtbl;
    let obj = Box::into_raw(Box::new(BridgedSteamObject { vtbl, native, kind })) as usize;
    cache.by_key.insert((native as usize, kind), obj);
    cache.all_ptrs.insert(obj);
    obj as *mut c_void
}

unsafe fn try_connect_native_steam(app_id: u32) -> bool {
    let mut sess_guard = NATIVE_SESSION.lock().unwrap();
    if sess_guard.is_some() {
        return true;
    }
    let Some(lib) = load_native_steam_lib() else {
        return false;
    };
    let app_str = app_id.to_string();
    std::env::set_var("SteamAppId", &app_str);
    std::env::set_var("SteamGameId", &app_str);

    let pipe = (lib.create_pipe)();
    if pipe <= 0 {
        log::info!(
            "steam_api64: macOS Steam client not running (Steam_CreateSteamPipe={pipe}); using offline shim"
        );
        return false;
    }
    let user = (lib.connect_global_user)(pipe);
    if user <= 0 {
        log::info!(
            "steam_api64: macOS Steam client has no active global user (pipe={pipe}, user={user}); using offline shim"
        );
        let _ = (lib.release_pipe)(pipe);
        return false;
    }

    let mut native_client = std::ptr::null_mut();
    for ver in [
        b"SteamClient021\0".as_slice(),
        b"SteamClient020\0".as_slice(),
        b"SteamClient019\0".as_slice(),
        b"SteamClient017\0".as_slice(),
    ] {
        let mut rc = 0;
        let ptr = (lib.create_interface)(ver.as_ptr() as *const c_char, &mut rc);
        if !ptr.is_null() {
            native_client = ptr;
            break;
        }
    }
    if native_client.is_null() {
        log::warn!("steam_api64: CreateInterface(SteamClient*) returned NULL; using offline shim");
        (lib.release_user)(pipe, user);
        let _ = (lib.release_pipe)(pipe);
        return false;
    }

    let bridged_client = wrap_native_interface(native_client, SteamKind::Client);
    NATIVE_PIPE.store(pipe, Ordering::Release);
    NATIVE_USER.store(user, Ordering::Release);
    NATIVE_ACTIVE.store(true, Ordering::Release);
    *sess_guard = Some(NativeSteamSession {
        lib,
        pipe,
        user,
        native_client,
        bridged_client,
    });
    log::info!(
        "steam_api64: connected live native ARM64 Steamworks IPC via {} (app_id={app_id}, pipe={pipe}, user={user})",
        lib.path.display()
    );
    true
}

/// Query a child interface from the live native `ISteamClient`, wrapped in a Win64-compatible bridge.
unsafe fn query_native_interface(ver: &str) -> Option<*mut c_void> {
    if !NATIVE_ACTIVE.load(Ordering::Acquire) {
        return None;
    }
    let (native_client, bridged_client, pipe, user) = {
        let g = NATIVE_SESSION.lock().unwrap();
        let s = g.as_ref()?;
        (s.native_client, s.bridged_client, s.pipe, s.user)
    };
    let kind = SteamKind::from_version(ver);
    if kind == SteamKind::Client {
        return Some(bridged_client);
    }
    let Ok(cver) = CString::new(ver) else {
        return None;
    };
    let nvtbl = *(native_client as *const *const usize);
    let raw = match kind {
        SteamKind::User => {
            let f: unsafe extern "C" fn(*mut c_void, i32, i32, *const c_char) -> *mut c_void =
                std::mem::transmute(*nvtbl.add(5));
            f(native_client, user, pipe, cver.as_ptr())
        }
        SteamKind::Friends => {
            let f: unsafe extern "C" fn(*mut c_void, i32, i32, *const c_char) -> *mut c_void =
                std::mem::transmute(*nvtbl.add(8));
            f(native_client, user, pipe, cver.as_ptr())
        }
        SteamKind::Utils => {
            let f: unsafe extern "C" fn(*mut c_void, i32, *const c_char) -> *mut c_void =
                std::mem::transmute(*nvtbl.add(9));
            f(native_client, pipe, cver.as_ptr())
        }
        SteamKind::Apps => {
            let f: unsafe extern "C" fn(*mut c_void, i32, i32, *const c_char) -> *mut c_void =
                std::mem::transmute(*nvtbl.add(15));
            f(native_client, user, pipe, cver.as_ptr())
        }
        SteamKind::Generic => {
            let f: unsafe extern "C" fn(*mut c_void, i32, i32, *const c_char) -> *mut c_void =
                std::mem::transmute(*nvtbl.add(12));
            f(native_client, user, pipe, cver.as_ptr())
        }
        SteamKind::Client => unreachable!(),
    };
    if raw.is_null() {
        None
    } else {
        Some(wrap_native_interface(raw, kind))
    }
}

unsafe fn native_call_0(this: *mut c_void, slot: usize) -> u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let nvtbl = *(obj.native as *const *const usize);
    let f: unsafe extern "C" fn(*mut c_void) -> u64 = std::mem::transmute(*nvtbl.add(slot));
    f(obj.native)
}

unsafe fn native_call_1(this: *mut c_void, slot: usize, a1: u64) -> u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let nvtbl = *(obj.native as *const *const usize);
    let f: unsafe extern "C" fn(*mut c_void, u64) -> u64 = std::mem::transmute(*nvtbl.add(slot));
    f(obj.native, a1)
}

unsafe fn native_call_4(
    this: *mut c_void,
    slot: usize,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
) -> u64 {
    let obj = &*(this as *const BridgedSteamObject);
    let nvtbl = *(obj.native as *const *const usize);
    let f: unsafe extern "C" fn(*mut c_void, u64, u64, u64, u64) -> u64 =
        std::mem::transmute(*nvtbl.add(slot));
    f(obj.native, a1, a2, a3, a4)
}

// ---------------------------------------------------------------------------
// Callback & CallResult dispatch manager
// ---------------------------------------------------------------------------

#[derive(Default)]
struct CallbackMgr {
    /// Registered broadcast callbacks: `(p_callback_base, i_callback)`.
    callbacks: Vec<(usize, i32)>,
    /// Registered async call results: `(p_callback_base, h_steam_api_call)`.
    call_results: Vec<(usize, u64)>,
}

static CALLBACK_MGR: LazyLock<Mutex<CallbackMgr>> = LazyLock::new(|| Mutex::new(CallbackMgr::default()));

// ---------------------------------------------------------------------------
// Offline fallback vtables (used when macOS Steam is not running or disabled)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn steam_vtbl_ret_one(
    _this: *mut c_void,
    _a1: u64,
    _a2: u64,
    _a3: u64,
) -> u64 {
    1
}

unsafe extern "win64" fn steam_vtbl_ret_zero(
    _this: *mut c_void,
    _a1: u64,
    _a2: u64,
    _a3: u64,
) -> u64 {
    0
}

unsafe extern "win64" fn steam_vtbl_get_steam_id(
    _this: *mut c_void,
    out_id: *mut u64,
) -> *mut u64 {
    // Win64 C++ ABI: returning an 8-byte struct (`CSteamID`) by value in a member
    // function passes a hidden `ret_ptr` in `rdx` (second parameter) and returns `rdx` in `rax`.
    if !out_id.is_null() && (out_id as usize) > 0x10000 {
        std::ptr::write_unaligned(out_id, LOCAL_STEAM_ID);
        return out_id;
    }
    LOCAL_STEAM_ID as *mut u64
}

unsafe extern "win64" fn steam_vtbl_get_persona_name(_this: *mut c_void) -> *const c_char {
    PERSONA_NAME.as_ptr() as *const c_char
}

unsafe extern "win64" fn steam_vtbl_get_language(_this: *mut c_void) -> *const c_char {
    GAME_LANGUAGE.as_ptr() as *const c_char
}

unsafe extern "win64" fn steam_vtbl_get_available_languages(_this: *mut c_void) -> *const c_char {
    AVAILABLE_LANGUAGES.as_ptr() as *const c_char
}

unsafe extern "win64" fn steam_vtbl_get_country(_this: *mut c_void) -> *const c_char {
    COUNTRY_CODE.as_ptr() as *const c_char
}

unsafe extern "win64" fn steam_vtbl_get_empty_str(_this: *mut c_void) -> *const c_char {
    EMPTY_STR.as_ptr() as *const c_char
}

unsafe extern "win64" fn steam_vtbl_get_app_id(_this: *mut c_void) -> u32 {
    read_steam_appid()
}

unsafe extern "win64" fn steam_vtbl_get_server_time(_this: *mut c_void) -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as u32)
}

unsafe extern "win64" fn steam_client_get_generic(
    _this: *mut c_void,
    _h_user: i32,
    _h_pipe: i32,
    ver: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver).unwrap_or_default();
    steam_interface_for_version(&v)
}

unsafe extern "win64" fn steam_client_get_user(
    _this: *mut c_void,
    _h_user: i32,
    _h_pipe: i32,
    ver: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver).unwrap_or_else(|| String::from("SteamUser021"));
    query_native_interface(&v).unwrap_or_else(|| offline_user_singleton())
}

unsafe extern "win64" fn steam_client_get_friends(
    _this: *mut c_void,
    _h_user: i32,
    _h_pipe: i32,
    ver: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver).unwrap_or_else(|| String::from("SteamFriends017"));
    query_native_interface(&v).unwrap_or_else(|| offline_friends_singleton())
}

unsafe extern "win64" fn steam_client_get_utils(
    _this: *mut c_void,
    _h_pipe: i32,
    ver: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver).unwrap_or_else(|| String::from("SteamUtils010"));
    query_native_interface(&v).unwrap_or_else(|| offline_utils_singleton())
}

unsafe extern "win64" fn steam_client_get_apps(
    _this: *mut c_void,
    _h_user: i32,
    _h_pipe: i32,
    ver: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver)
        .unwrap_or_else(|| String::from("STEAMAPPS_INTERFACE_VERSION008"));
    query_native_interface(&v).unwrap_or_else(|| offline_apps_singleton())
}

unsafe extern "win64" fn steam_client_get_any(
    _this: *mut c_void,
    _a1: u64,
    _a2: u64,
    _a3: u64,
) -> *mut c_void {
    offline_interface_singleton()
}

unsafe fn register_vtbl_fns(slots: &[usize; 80]) -> *mut c_void {
    for &f in slots {
        crate::register_native_va(f as u64, f);
    }
    let vtbl = Box::into_raw(Box::new(GenericSteamVtbl { slots: *slots }));
    Box::into_raw(Box::new(GenericSteamInterface { vtbl })) as *mut c_void
}

unsafe fn offline_interface_singleton() -> *mut c_void {
    static INSTANCE: OnceLock<usize> = OnceLock::new();
    let addr = *INSTANCE.get_or_init(|| {
        let zero_fn = steam_vtbl_ret_zero as *const () as usize;
        let one_fn = steam_vtbl_ret_one as *const () as usize;
        let id_fn = steam_vtbl_get_steam_id as *const () as usize;
        let mut slots = [zero_fn; 80];
        slots[0] = one_fn;
        slots[1] = one_fn;
        slots[2] = id_fn;
        register_vtbl_fns(&slots) as usize
    });
    addr as *mut c_void
}

unsafe fn offline_user_singleton() -> *mut c_void {
    offline_interface_singleton()
}

unsafe fn offline_apps_singleton() -> *mut c_void {
    static INSTANCE: OnceLock<usize> = OnceLock::new();
    let addr = *INSTANCE.get_or_init(|| {
        let zero_fn = steam_vtbl_ret_zero as *const () as usize;
        let one_fn = steam_vtbl_ret_one as *const () as usize;
        let id_fn = steam_vtbl_get_steam_id as *const () as usize;
        let lang_fn = steam_vtbl_get_language as *const () as usize;
        let langs_fn = steam_vtbl_get_available_languages as *const () as usize;
        let empty_fn = steam_vtbl_get_empty_str as *const () as usize;
        let mut slots = [zero_fn; 80];
        slots[0] = one_fn;    // BIsSubscribed
        slots[4] = lang_fn;   // GetCurrentGameLanguage
        slots[5] = langs_fn;  // GetAvailableGameLanguages
        slots[6] = one_fn;    // BIsSubscribedApp
        slots[7] = one_fn;    // BIsDlcInstalled
        slots[16] = one_fn;   // MarkContentCorrupt
        slots[19] = one_fn;   // BIsAppInstalled
        slots[20] = id_fn;    // GetAppOwner
        slots[21] = empty_fn; // GetLaunchQueryParam
        slots[23] = one_fn;   // GetAppBuildId
        slots[29] = one_fn;   // SetDlcContext
        register_vtbl_fns(&slots) as usize
    });
    addr as *mut c_void
}

unsafe fn offline_friends_singleton() -> *mut c_void {
    static INSTANCE: OnceLock<usize> = OnceLock::new();
    let addr = *INSTANCE.get_or_init(|| {
        let zero_fn = steam_vtbl_ret_zero as *const () as usize;
        let one_fn = steam_vtbl_ret_one as *const () as usize;
        let id_fn = steam_vtbl_get_steam_id as *const () as usize;
        let name_fn = steam_vtbl_get_persona_name as *const () as usize;
        let empty_fn = steam_vtbl_get_empty_str as *const () as usize;
        let mut slots = [zero_fn; 80];
        slots[0] = name_fn;   // GetPersonaName
        slots[2] = one_fn;    // GetPersonaState (k_EPersonaStateOnline)
        slots[4] = id_fn;     // GetFriendByIndex
        slots[7] = name_fn;   // GetFriendPersonaName
        slots[9] = name_fn;   // GetFriendPersonaNameHistory
        slots[19] = id_fn;    // GetClanByIndex
        slots[25] = id_fn;    // GetFriendFromSourceByIndex
        slots[39] = id_fn;    // GetClanOwner
        slots[41] = id_fn;    // GetClanOfficerByIndex
        slots[45] = empty_fn; // GetFriendRichPresence
        slots[47] = empty_fn; // GetFriendRichPresenceKeyByIndex
        slots[51] = id_fn;    // GetCoplayFriend
        slots[57] = id_fn;    // GetChatMemberByIndex
        register_vtbl_fns(&slots) as usize
    });
    addr as *mut c_void
}

unsafe fn offline_utils_singleton() -> *mut c_void {
    static INSTANCE: OnceLock<usize> = OnceLock::new();
    let addr = *INSTANCE.get_or_init(|| {
        let zero_fn = steam_vtbl_ret_zero as *const () as usize;
        let one_fn = steam_vtbl_ret_one as *const () as usize;
        let time_fn = steam_vtbl_get_server_time as *const () as usize;
        let country_fn = steam_vtbl_get_country as *const () as usize;
        let appid_fn = steam_vtbl_get_app_id as *const () as usize;
        let lang_fn = steam_vtbl_get_language as *const () as usize;
        let mut slots = [zero_fn; 80];
        slots[0] = one_fn;     // GetSecondsSinceAppActive
        slots[2] = one_fn;     // GetConnectedUniverse (k_EUniversePublic = 1)
        slots[3] = time_fn;    // GetServerRealTime
        slots[4] = country_fn; // GetIPCountry
        slots[9] = appid_fn;   // GetAppID
        slots[23] = lang_fn;   // GetSteamUILanguage
        register_vtbl_fns(&slots) as usize
    });
    addr as *mut c_void
}

unsafe fn offline_client_singleton() -> *mut c_void {
    static INSTANCE: OnceLock<usize> = OnceLock::new();
    let addr = *INSTANCE.get_or_init(|| {
        let one_fn = steam_vtbl_ret_one as *const () as usize;
        let any_fn = steam_client_get_any as *const () as usize;
        let mut slots = [any_fn; 80];
        slots[0] = one_fn; // CreateSteamPipe
        slots[1] = one_fn; // BReleaseSteamPipe
        slots[2] = one_fn; // ConnectToGlobalUser
        slots[3] = one_fn; // CreateLocalUser
        slots[4] = one_fn; // ReleaseUser
        slots[5] = steam_client_get_user as *const () as usize;
        slots[8] = steam_client_get_friends as *const () as usize;
        slots[9] = steam_client_get_utils as *const () as usize;
        slots[12] = steam_client_get_generic as *const () as usize;
        slots[15] = steam_client_get_apps as *const () as usize;
        register_vtbl_fns(&slots) as usize
    });
    addr as *mut c_void
}

unsafe fn steam_interface_for_version(ver: &str) -> *mut c_void {
    if let Some(live) = query_native_interface(ver) {
        return live;
    }
    match SteamKind::from_version(ver) {
        SteamKind::Apps => offline_apps_singleton(),
        SteamKind::Friends => offline_friends_singleton(),
        SteamKind::Utils => offline_utils_singleton(),
        SteamKind::Client => offline_client_singleton(),
        SteamKind::User => offline_user_singleton(),
        SteamKind::Generic => offline_interface_singleton(),
    }
}

fn read_steam_appid() -> u32 {
    if let Ok(s) = std::env::var("SteamAppId") {
        if let Ok(id) = s.trim().parse::<u32>() {
            if id != 0 {
                return id;
            }
        }
    }
    if let Ok(txt) = std::fs::read_to_string("steam_appid.txt") {
        if let Ok(id) = txt.trim().parse::<u32>() {
            if id != 0 {
                return id;
            }
        }
    }
    ACTIVE_APP_ID.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Public Win64 Steamworks SDK exports
// ---------------------------------------------------------------------------

/// Returns `false` (`0`) so the executable never quits to relaunch via `steam.exe`.
pub unsafe extern "win64" fn SteamAPI_RestartAppIfNecessary(un_own_app_id: u32) -> u8 {
    if un_own_app_id != 0 {
        ACTIVE_APP_ID.store(un_own_app_id, Ordering::Relaxed);
    }
    let app_id = read_steam_appid();
    log::info!(
        "steam_api64!SteamAPI_RestartAppIfNecessary(app_id={app_id}) -> false (continue in-process)"
    );
    0
}

pub unsafe extern "win64" fn SteamAPI_Init() -> u8 {
    let app_id = read_steam_appid();
    let live = try_connect_native_steam(app_id);
    log::info!(
        "steam_api64!SteamAPI_Init(app_id={app_id}) -> true (mode={})",
        if live { "native-ipc" } else { "offline-shim" }
    );
    1
}

pub unsafe extern "win64" fn SteamAPI_InitSafe() -> u8 {
    SteamAPI_Init()
}

/// Modern Steamworks SDK v1.58+ flat initializer (`ESteamAPIInitResult`: `0 = k_ESteamAPIInitResult_OK`).
pub unsafe extern "win64" fn SteamAPI_InitFlat(err_msg: *mut u8) -> c_int {
    if !err_msg.is_null() {
        std::ptr::write_unaligned(err_msg, 0);
    }
    let _ = SteamAPI_Init();
    0 // k_ESteamAPIInitResult_OK
}

pub unsafe extern "win64" fn SteamAPI_InitEx(err_msg: *mut u8) -> c_int {
    SteamAPI_InitFlat(err_msg)
}

pub unsafe extern "win64" fn SteamAPI_Shutdown() {
    log::debug!("steam_api64!SteamAPI_Shutdown()");
    NATIVE_ACTIVE.store(false, Ordering::Release);
    NATIVE_PIPE.store(0, Ordering::Release);
    NATIVE_USER.store(0, Ordering::Release);
    let sess = NATIVE_SESSION.lock().unwrap().take();
    if let Some(s) = sess {
        (s.lib.release_user)(s.pipe, s.user);
        let _ = (s.lib.release_pipe)(s.pipe);
    }
    {
        let mut mgr = CALLBACK_MGR.lock().unwrap();
        mgr.callbacks.clear();
        mgr.call_results.clear();
    }
    CTX_INIT_COUNTER.fetch_add(1, Ordering::AcqRel);
}

pub unsafe extern "win64" fn SteamAPI_IsSteamRunning() -> u8 {
    if NATIVE_ACTIVE.load(Ordering::Acquire) {
        return 1;
    }
    1
}

pub unsafe extern "win64" fn SteamAPI_RunCallbacks() {
    let (lib, pipe) = {
        let g = NATIVE_SESSION.lock().unwrap();
        let Some(s) = g.as_ref() else {
            return;
        };
        (s.lib, s.pipe)
    };
    let mut msg = NativeCallbackMsg::default();
    while (lib.b_get_callback)(pipe, &mut msg) != 0 {
        // 1. Async SteamAPICall_t completion (iCallback == 703)
        if msg.i_callback == K_I_CALLBACK_STEAM_API_CALL_COMPLETED
            && !msg.pub_param.is_null()
            && msg.cub_param >= 16
        {
            let h_async_call = std::ptr::read_unaligned(msg.pub_param as *const u64);
            let expected_cb = std::ptr::read_unaligned(msg.pub_param.add(8) as *const i32);
            let cub_param = std::ptr::read_unaligned(msg.pub_param.add(12) as *const u32);
            let matched: Vec<usize> = {
                let mut mgr = CALLBACK_MGR.lock().unwrap();
                let mut out = Vec::new();
                mgr.call_results.retain(|&(cb, handle)| {
                    if handle == h_async_call {
                        out.push(cb);
                        false
                    } else {
                        true
                    }
                });
                out
            };
            if !matched.is_empty() {
                let mut buf = vec![0u8; (cub_param as usize).max(16)];
                let mut failed: u8 = 0;
                let ok = (lib.get_api_call_result)(
                    pipe,
                    h_async_call,
                    buf.as_mut_ptr() as *mut c_void,
                    cub_param as c_int,
                    expected_cb,
                    &mut failed,
                );
                let io_fail = if ok == 0 || failed != 0 { 1u64 } else { 0u64 };
                for cb_ptr in matched {
                    let vtbl = std::ptr::read_unaligned(cb_ptr as *const usize);
                    if vtbl > 0x10000 {
                        // MSVC Win64 CCallbackBase slot 0 = Run(void*, bool, SteamAPICall_t)
                        let run_call_result = std::ptr::read_unaligned(vtbl as *const usize);
                        if run_call_result != 0 {
                            let _ = crate::call_guest8(
                                run_call_result,
                                cb_ptr as u64,
                                buf.as_ptr() as u64,
                                io_fail,
                                h_async_call,
                                0,
                                0,
                                0,
                                0,
                            );
                        }
                    }
                }
            }
        }

        // 2. Broadcast callbacks matching msg.i_callback
        let listeners: Vec<usize> = {
            let mgr = CALLBACK_MGR.lock().unwrap();
            mgr.callbacks
                .iter()
                .filter_map(|&(cb, id)| (id == msg.i_callback).then_some(cb))
                .collect()
        };
        for cb_ptr in listeners {
            let vtbl = std::ptr::read_unaligned(cb_ptr as *const usize);
            if vtbl > 0x10000 {
                // MSVC Win64 CCallbackBase slot 1 (+0x08) = Run(void*)
                let run_broadcast = std::ptr::read_unaligned((vtbl + 8) as *const usize);
                if run_broadcast != 0 {
                    let _ = crate::call_guest8(
                        run_broadcast,
                        cb_ptr as u64,
                        msg.pub_param as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    );
                }
            }
        }

        (lib.free_last_callback)(pipe);
    }
}

pub unsafe extern "win64" fn SteamAPI_ReleaseCurrentThreadMemory() {
    let f = {
        let g = NATIVE_SESSION.lock().unwrap();
        g.as_ref().and_then(|s| s.lib.release_thread_memory)
    };
    if let Some(release_fn) = f {
        release_fn(0);
    }
}

pub unsafe extern "win64" fn SteamAPI_GetHSteamUser() -> i32 {
    if NATIVE_ACTIVE.load(Ordering::Acquire) {
        NATIVE_USER.load(Ordering::Acquire)
    } else {
        1
    }
}

pub unsafe extern "win64" fn SteamAPI_GetHSteamPipe() -> i32 {
    if NATIVE_ACTIVE.load(Ordering::Acquire) {
        NATIVE_PIPE.load(Ordering::Acquire)
    } else {
        1
    }
}

pub unsafe extern "win64" fn SteamAPI_GetSteamInstallPath() -> *const c_char {
    static PATH: &[u8] = b"C:\\Program Files (x86)\\Steam\0";
    PATH.as_ptr() as *const c_char
}

pub unsafe extern "win64" fn SteamAPI_RegisterCallback(p_callback: *mut c_void, i_callback: c_int) {
    if p_callback.is_null() {
        return;
    }
    let base = p_callback as *mut u8;
    let flags = std::ptr::read_unaligned(base.add(8));
    std::ptr::write_unaligned(base.add(8), flags | K_E_CALLBACK_FLAGS_REGISTERED);
    std::ptr::write_unaligned(base.add(12) as *mut i32, i_callback);
    let mut mgr = CALLBACK_MGR.lock().unwrap();
    let entry = (p_callback as usize, i_callback);
    if !mgr.callbacks.contains(&entry) {
        mgr.callbacks.push(entry);
    }
}

pub unsafe extern "win64" fn SteamAPI_UnregisterCallback(p_callback: *mut c_void) {
    if p_callback.is_null() {
        return;
    }
    let base = p_callback as *mut u8;
    let flags = std::ptr::read_unaligned(base.add(8));
    std::ptr::write_unaligned(base.add(8), flags & !K_E_CALLBACK_FLAGS_REGISTERED);
    let mut mgr = CALLBACK_MGR.lock().unwrap();
    mgr.callbacks.retain(|&(cb, _)| cb != p_callback as usize);
}

pub unsafe extern "win64" fn SteamAPI_RegisterCallResult(
    p_callback: *mut c_void,
    h_api_call: u64,
) {
    if p_callback.is_null() || h_api_call == 0 {
        return;
    }
    let base = p_callback as *mut u8;
    let flags = std::ptr::read_unaligned(base.add(8));
    std::ptr::write_unaligned(base.add(8), flags | K_E_CALLBACK_FLAGS_REGISTERED);
    let mut mgr = CALLBACK_MGR.lock().unwrap();
    let entry = (p_callback as usize, h_api_call);
    if !mgr.call_results.contains(&entry) {
        mgr.call_results.push(entry);
    }
}

pub unsafe extern "win64" fn SteamAPI_UnregisterCallResult(
    p_callback: *mut c_void,
    h_api_call: u64,
) {
    if p_callback.is_null() {
        return;
    }
    let mut mgr = CALLBACK_MGR.lock().unwrap();
    mgr.call_results
        .retain(|&(cb, h)| !(cb == p_callback as usize && h == h_api_call));
}

pub unsafe extern "win64" fn SteamAPI_SetMiniDumpComment(_msg: LPCSTR) {}

pub unsafe extern "win64" fn SteamAPI_WriteMiniDump(
    _structured_exception_code: u32,
    _exception_info: *mut c_void,
    _build_id: u32,
) {
}

/// `SteamInternal_ContextInit(pContextInitData)`:
/// `pContextInitData` is `{ fn(*mut c_void), usize flag, *mut c_void ctx }` followed by
/// the inline `CSteamAPIContext` storage (`&p[2]`).
pub unsafe extern "win64" fn SteamInternal_ContextInit(p_context_init_data: *mut usize) -> *mut c_void {
    if p_context_init_data.is_null() {
        return offline_interface_singleton();
    }
    let cb = std::ptr::read_unaligned(p_context_init_data);
    let flag = std::ptr::read_unaligned(p_context_init_data.add(1));
    let ctx_ptr = p_context_init_data.add(2) as *mut c_void;
    let counter = CTX_INIT_COUNTER.load(Ordering::Acquire);
    if flag != counter {
        std::ptr::write_unaligned(p_context_init_data.add(1), counter);
        if cb != 0 {
            let _ = crate::call_guest8(cb, ctx_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    }
    ctx_ptr
}

pub unsafe extern "win64" fn SteamInternal_CreateInterface(ver: LPCSTR) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(ver).unwrap_or_default();
    log::debug!("steam_api64!SteamInternal_CreateInterface({v:?})");
    steam_interface_for_version(&v)
}

pub unsafe extern "win64" fn SteamInternal_FindOrCreateUserInterface(
    _h_steam_user: i32,
    psz_version: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(psz_version).unwrap_or_default();
    log::debug!("steam_api64!SteamInternal_FindOrCreateUserInterface({v:?})");
    steam_interface_for_version(&v)
}

pub unsafe extern "win64" fn SteamInternal_FindOrCreateGameServerInterface(
    _h_steam_user: i32,
    psz_version: LPCSTR,
) -> *mut c_void {
    let v = crate::strings::from_ansi_opt(psz_version).unwrap_or_default();
    log::debug!("steam_api64!SteamInternal_FindOrCreateGameServerInterface({v:?})");
    steam_interface_for_version(&v)
}

pub unsafe extern "win64" fn SteamAPI_ManualDispatch_Init() {}

pub unsafe extern "win64" fn SteamAPI_ManualDispatch_RunFrame(_h_steam_pipe: i32) {}

pub unsafe extern "win64" fn SteamAPI_ManualDispatch_GetNextCallback(
    h_steam_pipe: i32,
    p_callback_msg: *mut c_void,
) -> u8 {
    if p_callback_msg.is_null() {
        return 0;
    }
    let (lib, pipe) = {
        let g = NATIVE_SESSION.lock().unwrap();
        let Some(s) = g.as_ref() else {
            return 0;
        };
        (s.lib, if h_steam_pipe > 0 { h_steam_pipe } else { s.pipe })
    };
    let mut msg = NativeCallbackMsg::default();
    let ok = (lib.b_get_callback)(pipe, &mut msg);
    if ok != 0 {
        std::ptr::write_unaligned(p_callback_msg as *mut NativeCallbackMsg, msg);
    }
    ok
}

pub unsafe extern "win64" fn SteamAPI_ManualDispatch_FreeLastCallback(h_steam_pipe: i32) {
    let (lib, pipe) = {
        let g = NATIVE_SESSION.lock().unwrap();
        let Some(s) = g.as_ref() else {
            return;
        };
        (s.lib, if h_steam_pipe > 0 { h_steam_pipe } else { s.pipe })
    };
    (lib.free_last_callback)(pipe);
}

pub unsafe extern "win64" fn SteamAPI_ManualDispatch_GetAPICallResult(
    h_steam_pipe: i32,
    h_steam_api_call: u64,
    p_callback: *mut c_void,
    cub_callback: c_int,
    i_callback_expected: c_int,
    pb_failed: *mut u8,
) -> u8 {
    let (lib, pipe) = {
        let g = NATIVE_SESSION.lock().unwrap();
        match g.as_ref() {
            Some(s) => (s.lib, if h_steam_pipe > 0 { h_steam_pipe } else { s.pipe }),
            None => {
                if !pb_failed.is_null() {
                    std::ptr::write_unaligned(pb_failed, 1);
                }
                return 0;
            }
        }
    };
    (lib.get_api_call_result)(
        pipe,
        h_steam_api_call,
        p_callback,
        cub_callback,
        i_callback_expected,
        pb_failed,
    )
}

// Versioned interface accessors & flat API helpers
pub unsafe extern "win64" fn SteamClient() -> *mut c_void {
    steam_interface_for_version("SteamClient021")
}
pub unsafe extern "win64" fn SteamUser() -> *mut c_void {
    steam_interface_for_version("SteamUser021")
}
pub unsafe extern "win64" fn SteamFriends() -> *mut c_void {
    steam_interface_for_version("SteamFriends017")
}
pub unsafe extern "win64" fn SteamUtils() -> *mut c_void {
    steam_interface_for_version("SteamUtils010")
}
pub unsafe extern "win64" fn SteamMatchmaking() -> *mut c_void {
    steam_interface_for_version("SteamMatchMaking009")
}
pub unsafe extern "win64" fn SteamUserStats() -> *mut c_void {
    steam_interface_for_version("STEAMUSERSTATS_INTERFACE_VERSION012")
}
pub unsafe extern "win64" fn SteamApps() -> *mut c_void {
    steam_interface_for_version("STEAMAPPS_INTERFACE_VERSION008")
}
pub unsafe extern "win64" fn SteamNetworking() -> *mut c_void {
    steam_interface_for_version("SteamNetworking006")
}
pub unsafe extern "win64" fn SteamMatchmakingServers() -> *mut c_void {
    steam_interface_for_version("SteamMatchMakingServers002")
}
pub unsafe extern "win64" fn SteamRemoteStorage() -> *mut c_void {
    steam_interface_for_version("STEAMREMOTESTORAGE_INTERFACE_VERSION016")
}
pub unsafe extern "win64" fn SteamScreenshots() -> *mut c_void {
    steam_interface_for_version("STEAMSCREENSHOTS_INTERFACE_VERSION003")
}
pub unsafe extern "win64" fn SteamHTTP() -> *mut c_void {
    steam_interface_for_version("STEAMHTTP_INTERFACE_VERSION003")
}
pub unsafe extern "win64" fn SteamController() -> *mut c_void {
    steam_interface_for_version("SteamController008")
}
pub unsafe extern "win64" fn SteamUGC() -> *mut c_void {
    steam_interface_for_version("STEAMUGC_INTERFACE_VERSION017")
}
pub unsafe extern "win64" fn SteamAppList() -> *mut c_void {
    steam_interface_for_version("STEAMAPPLIST_INTERFACE_VERSION001")
}
pub unsafe extern "win64" fn SteamMusic() -> *mut c_void {
    steam_interface_for_version("STEAMMUSIC_INTERFACE_VERSION001")
}
pub unsafe extern "win64" fn SteamMusicRemote() -> *mut c_void {
    steam_interface_for_version("STEAMMUSICREMOTE_INTERFACE_VERSION001")
}
pub unsafe extern "win64" fn SteamHTMLSurface() -> *mut c_void {
    steam_interface_for_version("STEAMHTMLSURFACE_INTERFACE_VERSION005")
}
pub unsafe extern "win64" fn SteamInventory() -> *mut c_void {
    steam_interface_for_version("STEAMINVENTORY_INTERFACE_V003")
}
pub unsafe extern "win64" fn SteamVideo() -> *mut c_void {
    steam_interface_for_version("STEAMVIDEO_INTERFACE_V002")
}
pub unsafe extern "win64" fn SteamParentalSettings() -> *mut c_void {
    steam_interface_for_version("STEAMPARENTALSETTINGS_INTERFACE_VERSION001")
}
pub unsafe extern "win64" fn SteamInput() -> *mut c_void {
    steam_interface_for_version("SteamInput006")
}
pub unsafe extern "win64" fn SteamParties() -> *mut c_void {
    steam_interface_for_version("SteamParties002")
}
pub unsafe extern "win64" fn SteamRemotePlay() -> *mut c_void {
    steam_interface_for_version("STEAMREMOTEPLAY_INTERFACE_VERSION002")
}
pub unsafe extern "win64" fn SteamNetworkingSockets() -> *mut c_void {
    steam_interface_for_version("SteamNetworkingSockets012")
}
pub unsafe extern "win64" fn SteamNetworkingUtils() -> *mut c_void {
    steam_interface_for_version("SteamNetworkingUtils004")
}
pub unsafe extern "win64" fn SteamNetworkingMessages() -> *mut c_void {
    steam_interface_for_version("SteamNetworkingMessages002")
}

// Flat C APIs used by newer Steamworks SDKs (v1.50+)
pub unsafe extern "win64" fn SteamAPI_SteamUser_v021() -> *mut c_void {
    steam_interface_for_version("SteamUser021")
}
pub unsafe extern "win64" fn SteamAPI_SteamUser_v023() -> *mut c_void {
    steam_interface_for_version("SteamUser023")
}
pub unsafe extern "win64" fn SteamAPI_SteamFriends_v017() -> *mut c_void {
    steam_interface_for_version("SteamFriends017")
}
pub unsafe extern "win64" fn SteamAPI_SteamUtils_v010() -> *mut c_void {
    steam_interface_for_version("SteamUtils010")
}
pub unsafe extern "win64" fn SteamAPI_SteamApps_v008() -> *mut c_void {
    steam_interface_for_version("STEAMAPPS_INTERFACE_VERSION008")
}
pub unsafe extern "win64" fn SteamAPI_SteamUserStats_v012() -> *mut c_void {
    steam_interface_for_version("STEAMUSERSTATS_INTERFACE_VERSION012")
}
pub unsafe extern "win64" fn SteamAPI_SteamRemoteStorage_v016() -> *mut c_void {
    steam_interface_for_version("STEAMREMOTESTORAGE_INTERFACE_VERSION016")
}
pub unsafe extern "win64" fn SteamAPI_SteamInput_v006() -> *mut c_void {
    steam_interface_for_version("SteamInput006")
}

pub unsafe extern "win64" fn SteamAPI_ISteamUser_GetSteamID(self_ptr: *mut c_void) -> u64 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 2);
    }
    LOCAL_STEAM_ID
}
pub unsafe extern "win64" fn SteamAPI_ISteamUser_BLoggedOn(self_ptr: *mut c_void) -> u8 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 1) as u8;
    }
    if let Some((f, pipe, user)) = {
        let g = NATIVE_SESSION.lock().unwrap();
        g.as_ref().and_then(|s| s.lib.b_logged_on.map(|f| (f, s.pipe, s.user)))
    } {
        return f(pipe, user);
    }
    1
}
pub unsafe extern "win64" fn SteamAPI_ISteamUser_GetAuthSessionTicket(
    self_ptr: *mut c_void,
    p_ticket: *mut c_void,
    cb_max_ticket: c_int,
    pcb_ticket: *mut u32,
    p_identity: *const c_void,
) -> u32 {
    if is_bridged_ptr(self_ptr) {
        return native_call_4(
            self_ptr,
            13,
            p_ticket as u64,
            cb_max_ticket as u64,
            pcb_ticket as u64,
            p_identity as u64,
        ) as u32;
    }
    if !pcb_ticket.is_null() {
        std::ptr::write_unaligned(pcb_ticket, 0);
    }
    0 // k_HAuthTicketInvalid
}
pub unsafe extern "win64" fn SteamAPI_ISteamUser_CancelAuthTicket(
    self_ptr: *mut c_void,
    h_auth_ticket: u32,
) {
    if is_bridged_ptr(self_ptr) {
        let _ = native_call_1(self_ptr, 16, h_auth_ticket as u64);
    }
}
pub unsafe extern "win64" fn SteamAPI_ISteamUser_GetAuthTicketForWebApi(
    self_ptr: *mut c_void,
    pch_identity: LPCSTR,
) -> u32 {
    if is_bridged_ptr(self_ptr) {
        return native_call_1(self_ptr, 32, pch_identity as u64) as u32;
    }
    0
}
pub unsafe extern "win64" fn SteamAPI_ISteamApps_BIsSubscribed(self_ptr: *mut c_void) -> u8 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 0) as u8;
    }
    1
}
pub unsafe extern "win64" fn SteamAPI_ISteamApps_BIsDlcInstalled(
    self_ptr: *mut c_void,
    app_id: u32,
) -> u8 {
    if is_bridged_ptr(self_ptr) {
        return native_call_1(self_ptr, 7, app_id as u64) as u8;
    }
    1
}
pub unsafe extern "win64" fn SteamAPI_ISteamApps_GetCurrentGameLanguage(
    self_ptr: *mut c_void,
) -> *const c_char {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 4) as *const c_char;
    }
    GAME_LANGUAGE.as_ptr() as *const c_char
}
pub unsafe extern "win64" fn SteamAPI_ISteamApps_GetAvailableGameLanguages(
    self_ptr: *mut c_void,
) -> *const c_char {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 5) as *const c_char;
    }
    AVAILABLE_LANGUAGES.as_ptr() as *const c_char
}
pub unsafe extern "win64" fn SteamAPI_ISteamFriends_GetPersonaName(
    self_ptr: *mut c_void,
) -> *const c_char {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 0) as *const c_char;
    }
    PERSONA_NAME.as_ptr() as *const c_char
}
pub unsafe extern "win64" fn SteamAPI_ISteamUtils_GetAppID(self_ptr: *mut c_void) -> u32 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 9) as u32;
    }
    read_steam_appid()
}
pub unsafe extern "win64" fn SteamAPI_ISteamUtils_GetIPCountry(
    self_ptr: *mut c_void,
) -> *const c_char {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 4) as *const c_char;
    }
    COUNTRY_CODE.as_ptr() as *const c_char
}
pub unsafe extern "win64" fn SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(
    self_ptr: *mut c_void,
) -> u8 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 34) as u8;
    }
    0
}
pub unsafe extern "win64" fn SteamAPI_ISteamUtils_IsOverlayEnabled(self_ptr: *mut c_void) -> u8 {
    if is_bridged_ptr(self_ptr) {
        return native_call_0(self_ptr, 17) as u8;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_vtables_have_accurate_steamworks_slot_indices() {
        unsafe {
            let apps = offline_apps_singleton() as *const GenericSteamInterface;
            let avtbl = &*(*apps).vtbl;
            let get_lang: unsafe extern "win64" fn(*mut c_void) -> *const c_char =
                std::mem::transmute(avtbl.slots[4]);
            assert_eq!(
                crate::strings::from_ansi(get_lang(apps as *mut c_void) as *const u8),
                "english"
            );

            let friends = offline_friends_singleton() as *const GenericSteamInterface;
            let fvtbl = &*(*friends).vtbl;
            let get_name: unsafe extern "win64" fn(*mut c_void) -> *const c_char =
                std::mem::transmute(fvtbl.slots[0]);
            let get_friend_name: unsafe extern "win64" fn(*mut c_void, u64) -> *const c_char =
                std::mem::transmute(fvtbl.slots[7]);
            let get_rich_pres: unsafe extern "win64" fn(*mut c_void, u64, *const u8) -> *const c_char =
                std::mem::transmute(fvtbl.slots[45]);
            assert_eq!(
                crate::strings::from_ansi(get_name(friends as *mut c_void) as *const u8),
                "Player"
            );
            assert_eq!(
                crate::strings::from_ansi(get_friend_name(friends as *mut c_void, LOCAL_STEAM_ID) as *const u8),
                "Player"
            );
            assert_eq!(
                crate::strings::from_ansi(get_rich_pres(friends as *mut c_void, LOCAL_STEAM_ID, b"k\0".as_ptr()) as *const u8),
                ""
            );

            let utils = offline_utils_singleton() as *const GenericSteamInterface;
            let uvtbl = &*(*utils).vtbl;
            let get_univ: unsafe extern "win64" fn(*mut c_void) -> u32 =
                std::mem::transmute(uvtbl.slots[2]);
            let get_time: unsafe extern "win64" fn(*mut c_void) -> u32 =
                std::mem::transmute(uvtbl.slots[3]);
            let get_country: unsafe extern "win64" fn(*mut c_void) -> *const c_char =
                std::mem::transmute(uvtbl.slots[4]);
            let get_ui_lang: unsafe extern "win64" fn(*mut c_void) -> *const c_char =
                std::mem::transmute(uvtbl.slots[23]);
            assert_eq!(get_univ(utils as *mut c_void), 1);
            assert!(get_time(utils as *mut c_void) > 1_700_000_000);
            assert_eq!(
                crate::strings::from_ansi(get_country(utils as *mut c_void) as *const u8),
                "US"
            );
            assert_eq!(
                crate::strings::from_ansi(get_ui_lang(utils as *mut c_void) as *const u8),
                "english"
            );
        }
    }

    #[test]
    fn bridged_csteamid_adapter_translates_arm64_u64_to_win64_out_ptr() {
        #[repr(C)]
        struct FakeNativeUser {
            vtbl: *const [usize; 4],
        }
        unsafe extern "C" fn fake_logged_on(_this: *mut c_void) -> u8 {
            1
        }
        unsafe extern "C" fn fake_get_steam_id(_this: *mut c_void) -> u64 {
            0x0110_0001_1234_5678
        }
        let fake_vtbl: [usize; 4] = [
            0,
            fake_logged_on as *const () as usize,
            fake_get_steam_id as *const () as usize,
            0,
        ];
        let mut fake = FakeNativeUser { vtbl: &fake_vtbl };
        unsafe {
            let bridged = wrap_native_interface(&mut fake as *mut FakeNativeUser as *mut c_void, SteamKind::User);
            assert!(is_bridged_ptr(bridged));
            let iface = &*(bridged as *const GenericSteamInterface);
            let vtbl = &*iface.vtbl;
            let get_id: unsafe extern "win64" fn(*mut c_void, *mut u64) -> *mut u64 =
                std::mem::transmute(vtbl.slots[2]);
            let mut out = 0u64;
            let ret = get_id(bridged, &mut out);
            assert_eq!(ret, &mut out as *mut u64);
            assert_eq!(out, 0x0110_0001_1234_5678);
            assert_eq!(SteamAPI_ISteamUser_GetSteamID(bridged), 0x0110_0001_1234_5678);
            assert_eq!(SteamAPI_ISteamUser_BLoggedOn(bridged), 1);
        }
    }

    #[test]
    fn callback_registration_updates_win64_flags_and_id() {
        #[repr(C)]
        struct FakeCallback {
            vtbl: usize,
            flags: u8,
            _pad: [u8; 3],
            i_callback: i32,
        }
        let mut cb = FakeCallback {
            vtbl: 0,
            flags: 0,
            _pad: [0; 3],
            i_callback: 0,
        };
        unsafe {
            let p = &mut cb as *mut FakeCallback as *mut c_void;
            SteamAPI_RegisterCallback(p, 163); // GetAuthSessionTicketResponse_t
            assert_eq!(cb.flags & K_E_CALLBACK_FLAGS_REGISTERED, K_E_CALLBACK_FLAGS_REGISTERED);
            assert_eq!(cb.i_callback, 163);
            SteamAPI_UnregisterCallback(p);
            assert_eq!(cb.flags & K_E_CALLBACK_FLAGS_REGISTERED, 0);

            SteamAPI_RegisterCallResult(p, 0xCAFE_BABE);
            assert_eq!(cb.flags & K_E_CALLBACK_FLAGS_REGISTERED, K_E_CALLBACK_FLAGS_REGISTERED);
            SteamAPI_UnregisterCallResult(p, 0xCAFE_BABE);
        }
    }

    #[test]
    fn native_steam_init_handles_absent_or_running_client_cleanly() {
        unsafe {
            assert_eq!(SteamAPI_RestartAppIfNecessary(480), 0);
            assert_eq!(SteamAPI_Init(), 1);
            let apps = SteamInternal_FindOrCreateUserInterface(
                SteamAPI_GetHSteamUser(),
                b"STEAMAPPS_INTERFACE_VERSION008\0".as_ptr(),
            );
            assert!(!apps.is_null());
            SteamAPI_RunCallbacks();
            SteamAPI_ReleaseCurrentThreadMemory();
            SteamAPI_Shutdown();
        }
    }
}
