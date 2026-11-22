//! Audio harness (`aw2-loader --audio-harness` / `AW2_AUDIO_HARNESS=1`): after
//! the CRT has initialized (the loader hooks WinMain, 0x141856b20, to
//! [`winmain`]), drive the game's statically linked Wwise 2022.1.4 through its
//! own exported `AK::` API — no Remedy engine code runs:
//!
//! 1. `MemoryMgr::Init`, `StreamMgr::Create`, `StreamMgr::CreateDevice` with a
//!    blocking low-level IO hook + file location resolver implemented here
//!    (host files; needed for `StartOutputCapture`, which writes through the
//!    stream manager);
//! 2. `SoundEngine::Init` (default settings: the "System" sink = WASAPI =
//!    our `audio` module) with our PCM tap pointed at `<dir>/tap.wav`;
//! 3. `LoadBankMemoryCopy(Init.bnk)` + one event bank, listener/emitter game
//!    objects, `StartOutputCapture(<dir>/capture.wav)`, `PostEvent` every
//!    `REPEAT_MS`, `RenderAudio` every 10 ms for `SECONDS`;
//! 4. `StopOutputCapture`, `SoundEngine::Term` (flushes the tap), then align
//!    and diff the two WAVs ([`super::wavcmp`]) and write `<dir>/report.txt`.
//!
//! Banks live in the game's packages; extract them first:
//! `cargo data cat data_pc/audio/events/Init.bnk out/audio_harness/Init.bnk` (and the
//! event bank) — `tools/audio_harness.sh` does this and runs the loader.
//!
//! Environment: `AW2_AUDIO_HARNESS_DIR` (default `out/audio_harness`),
//! `AW2_AUDIO_HARNESS_BANK` (default `play_gui_caseboard_hand_up`),
//! `AW2_AUDIO_HARNESS_EVENT` (default = bank name), `AW2_AUDIO_HARNESS_SECONDS` (4),
//! `AW2_AUDIO_HARNESS_REPEAT_MS` (700), `AW2_AUDIO_HARNESS_TERM=0` (skip Term).
//!
//! Struct layouts used (from the decompiled exe): `AkDeviceSettings`
//! (`GetDefaultDeviceSettings` 0x141674ca0: `uSchedulerTypeFlags` @+0x18,
//! 1 = blocking), `AkFileDesc` (0x30 bytes: iFileSize @0, uSector u64 @8,
//! uCustomParamSize @0x10, pCustomParam @0x18, hFile @0x20, deviceID @0x28;
//! Remedy's resolver 0x1424c0770), `AkIOTransferInfo` (uFilePosition u64 @0,
//! uBufferSize @8, uRequestedSize @0xc; blocking device 0x141679540), vtable
//! slot orders from `IAkLowLevelIOHook` 0x143412880 / `IAkFileLocationResolver`
//! 0x1434129c8 (MSVC puts `Open(AkFileID)` before `Open(const wchar_t*)`,
//! confirmed by snd::WwiseIOHook 0x1424c06c0 forwarding to slot 2).

use super::wavcmp;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

/// WinMain replacement installed by the loader (`--audio-harness`).
pub unsafe extern "win64" fn winmain(_hinst: *mut c_void, _prev: *mut c_void, _cmd: *const u8, _show: i32) -> i32 {
    log::info!(target: "audio", "audio harness: replacing WinMain");
    let code = run();
    log::info!(target: "audio", "audio harness: exit code {code}");
    code
}

pub(super) const AK_SUCCESS: i32 = 1;
const AK_FAIL: i32 = 2;
const AK_FILE_NOT_FOUND: i32 = 66;

pub(super) fn akr(r: i32) -> String {
    let name = match r {
        0 => "AK_NotImplemented",
        1 => "AK_Success",
        2 => "AK_Fail",
        31 => "AK_InvalidParameter",
        52 => "AK_InsufficientMemory",
        54 => "AK_UnknownBankID",
        56 => "AK_BankReadError",
        64 => "AK_WrongBankVersion",
        66 => "AK_FileNotFound",
        102 => "AK_NotInitialized",
        _ => "",
    };
    format!("{r} {name}")
}

// ---------------------------------------------------------------------------
// Exported AK API, resolved by mangled name from AlanWake2.exe's export table
// ---------------------------------------------------------------------------

pub(super) struct Ak {
    pub(super) mem_is_init: unsafe extern "win64" fn() -> bool,
    pub(super) mem_defaults: unsafe extern "win64" fn(*mut u8),
    pub(super) mem_init: unsafe extern "win64" fn(*mut u8) -> i32,
    pub(super) stm_create: unsafe extern "win64" fn(*const u8) -> *mut c_void,
    pub(super) stm_device_defaults: unsafe extern "win64" fn(*mut u8),
    pub(super) stm_create_device: unsafe extern "win64" fn(*const u8, *mut c_void) -> u32,
    pub(super) stm_set_resolver: unsafe extern "win64" fn(*mut c_void),
    pub(super) se_defaults: unsafe extern "win64" fn(*mut u8),
    pub(super) se_init: unsafe extern "win64" fn(*mut u8, *mut u8) -> i32,
    pub(super) se_is_init: unsafe extern "win64" fn() -> bool,
    pub(super) se_term: unsafe extern "win64" fn(),
    pub(super) load_bank_copy: unsafe extern "win64" fn(*const c_void, u32, *mut u32) -> i32,
    pub(super) register_obj: unsafe extern "win64" fn(u64) -> i32,
    pub(super) set_default_listeners: unsafe extern "win64" fn(*const u64, u32) -> i32,
    pub(super) id_from_string: unsafe extern "win64" fn(*const u8) -> u32,
    pub(super) post_event: unsafe extern "win64" fn(u32, u64, u32, *const c_void, *mut c_void, u32, *const c_void, u32) -> u32,
    pub(super) render_audio: unsafe extern "win64" fn(bool) -> i32,
    pub(super) start_capture: unsafe extern "win64" fn(*const u16) -> i32,
    pub(super) stop_capture: unsafe extern "win64" fn() -> i32,
    pub(super) stop_all: unsafe extern "win64" fn(u64),
    pub(super) set_position: unsafe extern "win64" fn(u64, *const u8, i32) -> i32,
    pub(super) stream_mgr_global: *const *mut c_void,
}

pub(super) fn resolve(name: &str) -> Result<usize, String> {
    let l = crate::loader();
    let exe = l.get_module_handle(None).ok_or("no exe module")?;
    l.get_proc_address(exe, crate::ProcName::Name(name)).ok_or_else(|| format!("export {name} not found"))
}

macro_rules! ak_fn {
    ($name:expr) => {
        unsafe { std::mem::transmute(resolve($name)?) }
    };
}

impl Ak {
    pub(super) fn load() -> Result<Ak, String> {
        Ok(Ak {
            mem_is_init: ak_fn!("?IsInitialized@MemoryMgr@AK@@YA_NXZ"),
            mem_defaults: ak_fn!("?GetDefaultSettings@MemoryMgr@AK@@YAXAEAUAkMemSettings@@@Z"),
            mem_init: ak_fn!("?Init@MemoryMgr@AK@@YA?AW4AKRESULT@@PEAUAkMemSettings@@@Z"),
            stm_create: ak_fn!("?Create@StreamMgr@AK@@YAPEAVIAkStreamMgr@2@AEBUAkStreamMgrSettings@@@Z"),
            stm_device_defaults: ak_fn!("?GetDefaultDeviceSettings@StreamMgr@AK@@YAXAEAUAkDeviceSettings@@@Z"),
            stm_create_device: ak_fn!("?CreateDevice@StreamMgr@AK@@YAIAEBUAkDeviceSettings@@PEAVIAkLowLevelIOHook@12@@Z"),
            stm_set_resolver: ak_fn!("?SetFileLocationResolver@StreamMgr@AK@@YAXPEAVIAkFileLocationResolver@12@@Z"),
            se_defaults: ak_fn!("?GetDefaultInitSettings@SoundEngine@AK@@YAXAEAUAkInitSettings@@@Z"),
            se_init: ak_fn!("?Init@SoundEngine@AK@@YA?AW4AKRESULT@@PEAUAkInitSettings@@PEAUAkPlatformInitSettings@@@Z"),
            se_is_init: ak_fn!("?IsInitialized@SoundEngine@AK@@YA_NXZ"),
            se_term: ak_fn!("?Term@SoundEngine@AK@@YAXXZ"),
            load_bank_copy: ak_fn!("?LoadBankMemoryCopy@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI@Z"),
            register_obj: ak_fn!("?RegisterGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@_K@Z"),
            set_default_listeners: ak_fn!("?SetDefaultListeners@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_KI@Z"),
            id_from_string: ak_fn!("?GetIDFromString@SoundEngine@AK@@YAIPEBD@Z"),
            post_event: ak_fn!("?PostEvent@SoundEngine@AK@@YAII_KIP6AXW4AkCallbackType@@PEAUAkCallbackInfo@@@ZPEAXIPEAUAkExternalSourceInfo@@I@Z"),
            render_audio: ak_fn!("?RenderAudio@SoundEngine@AK@@YA?AW4AKRESULT@@_N@Z"),
            start_capture: ak_fn!("?StartOutputCapture@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W@Z"),
            stop_capture: ak_fn!("?StopOutputCapture@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
            stop_all: ak_fn!("?StopAll@SoundEngine@AK@@YAX_K@Z"),
            set_position: ak_fn!("?SetPosition@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEBVAkWorldTransform@@W4AkSetPositionFlags@@@Z"),
            stream_mgr_global: resolve("?m_pStreamMgr@IAkStreamMgr@AK@@1PEAV12@EA")? as *const *mut c_void,
        })
    }
}

// ---------------------------------------------------------------------------
// Low-level IO: IAkFileLocationResolver + IAkIOHookBlocking over host files
// ---------------------------------------------------------------------------

static DEVICE_ID: AtomicU32 = AtomicU32::new(u32::MAX);
static NEXT_FILE: AtomicU32 = AtomicU32::new(1);

fn files() -> &'static Mutex<HashMap<u64, (File, PathBuf)>> {
    static F: OnceLock<Mutex<HashMap<u64, (File, PathBuf)>>> = OnceLock::new();
    F.get_or_init(Default::default)
}

pub(super) fn base_dir() -> &'static PathBuf {
    static D: OnceLock<PathBuf> = OnceLock::new();
    D.get_or_init(|| {
        let d = PathBuf::from(std::env::var("AW2_AUDIO_HARNESS_DIR").unwrap_or_else(|_| "out/audio_harness".into()));
        let _ = std::fs::create_dir_all(&d);
        d.canonicalize().unwrap_or(d)
    })
}

#[repr(C)]
struct AkFileDesc {
    file_size: i64,
    sector: u64,
    custom_param_size: u32,
    custom_param: *mut c_void,
    h_file: u64,
    device_id: u32,
}
const _: () = assert!(std::mem::size_of::<AkFileDesc>() == 0x30);
const _: () = assert!(std::mem::offset_of!(AkFileDesc, h_file) == 0x20);
const _: () = assert!(std::mem::offset_of!(AkFileDesc, device_id) == 0x28);

#[repr(C)]
struct AkIOTransferInfo {
    file_position: u64,
    buffer_size: u32,
    requested_size: u32,
}

unsafe fn wstr(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    String::from_utf16_lossy(std::slice::from_raw_parts(p, n))
}

/// Map a Wwise file name to a host path: absolute paths as-is, otherwise
/// relative to the harness dir (backslashes normalized).
fn host_path(name: &str) -> PathBuf {
    let n = name.replace('\\', "/");
    if n.starts_with('/') {
        PathBuf::from(n)
    } else {
        base_dir().join(n)
    }
}

unsafe fn open_host(path: PathBuf, mode: u32, sync: *mut bool, desc: *mut AkFileDesc) -> i32 {
    // AkOpenMode: 0 Read, 1 Write, 2 WriteOvrwr, 3 ReadWrite.
    let r = match mode {
        0 => File::open(&path),
        1 | 2 => File::create(&path),
        _ => std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&path),
    };
    match r {
        Ok(f) => {
            let size = f.metadata().map(|m| m.len()).unwrap_or(0);
            let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed) as u64;
            log::info!(target: "audio", "harness IO: open {} (mode {mode}) -> #{id}, {size} bytes", path.display());
            files().lock().unwrap().insert(id, (f, path));
            if !sync.is_null() {
                *sync = true;
            }
            std::ptr::write(
                desc,
                AkFileDesc { file_size: size as i64, sector: 0, custom_param_size: 0, custom_param: std::ptr::null_mut(), h_file: id, device_id: DEVICE_ID.load(Ordering::Relaxed) },
            );
            AK_SUCCESS
        }
        Err(e) => {
            log::warn!(target: "audio", "harness IO: open {} (mode {mode}) failed: {e}", path.display());
            AK_FILE_NOT_FOUND
        }
    }
}

mod resolver {
    use super::*;
    pub unsafe extern "win64" fn dtor(this: *mut c_void, _flags: u32) -> *mut c_void {
        this
    }
    /// Open(AkFileID, AkOpenMode, AkFileSystemFlags*, bool& io_bSyncOpen, AkFileDesc&)
    pub unsafe extern "win64" fn open_id(_this: *mut c_void, id: u32, mode: u32, flags: *const u32, sync: *mut bool, desc: *mut AkFileDesc) -> i32 {
        // AkFileSystemFlags { uCompanyID, uCodecID, ... }: codec 0 = bank, else media.
        let codec = if flags.is_null() { 0 } else { *flags.add(1) };
        let name = if codec == 0 { format!("{id}.bnk") } else { format!("{id}.wem") };
        open_host(host_path(&name), mode, sync, desc)
    }
    pub unsafe extern "win64" fn open_name(_this: *mut c_void, name: *const u16, mode: u32, _flags: *const u32, sync: *mut bool, desc: *mut AkFileDesc) -> i32 {
        open_host(host_path(&wstr(name)), mode, sync, desc)
    }
    pub unsafe extern "win64" fn not_impl(_this: *mut c_void) -> i32 {
        0 // AK_NotImplemented (OutputSearchedPaths)
    }
}

mod hook {
    use super::*;
    pub unsafe extern "win64" fn dtor(this: *mut c_void, _flags: u32) -> *mut c_void {
        this
    }
    pub unsafe extern "win64" fn close(_this: *mut c_void, desc: *mut AkFileDesc) -> i32 {
        match files().lock().unwrap().remove(&(*desc).h_file) {
            Some((f, p)) => {
                let _ = f.sync_all();
                log::info!(target: "audio", "harness IO: close {}", p.display());
                AK_SUCCESS
            }
            None => AK_FAIL,
        }
    }
    pub unsafe extern "win64" fn block_size(_this: *mut c_void, _desc: *mut AkFileDesc) -> u32 {
        1
    }
    pub unsafe extern "win64" fn device_desc(_this: *mut c_void, _out: *mut c_void) {}
    pub unsafe extern "win64" fn device_data(_this: *mut c_void) -> u32 {
        0
    }
    pub unsafe extern "win64" fn read(_this: *mut c_void, desc: *mut AkFileDesc, _heur: *const c_void, buf: *mut u8, xfer: *mut AkIOTransferInfo) -> i32 {
        let x = &*xfer;
        let mut g = files().lock().unwrap();
        let Some((f, _)) = g.get_mut(&(*desc).h_file) else { return AK_FAIL };
        let out = std::slice::from_raw_parts_mut(buf, x.requested_size as usize);
        let r = f.seek(SeekFrom::Start(x.file_position)).and_then(|_| {
            let mut n = 0;
            while n < out.len() {
                match f.read(&mut out[n..])? {
                    0 => break,
                    k => n += k,
                }
            }
            Ok(n)
        });
        match r {
            Ok(n) if n == out.len() => AK_SUCCESS,
            Ok(n) => {
                out[n..].fill(0);
                AK_SUCCESS // past EOF: the stream manager clamps by iFileSize
            }
            Err(_) => AK_FAIL,
        }
    }
    pub unsafe extern "win64" fn write(_this: *mut c_void, desc: *mut AkFileDesc, _heur: *const c_void, data: *const u8, xfer: *mut AkIOTransferInfo) -> i32 {
        let x = &*xfer;
        let mut g = files().lock().unwrap();
        let Some((f, _)) = g.get_mut(&(*desc).h_file) else { return AK_FAIL };
        let src = std::slice::from_raw_parts(data, x.requested_size as usize);
        match f.seek(SeekFrom::Start(x.file_position)).and_then(|_| f.write_all(src)) {
            Ok(()) => AK_SUCCESS,
            Err(_) => AK_FAIL,
        }
    }
}

type Ptr = *mut c_void;

#[repr(C)]
struct ResolverVtbl {
    dtor: unsafe extern "win64" fn(Ptr, u32) -> Ptr,
    open_id: unsafe extern "win64" fn(Ptr, u32, u32, *const u32, *mut bool, *mut AkFileDesc) -> i32,
    open_name: unsafe extern "win64" fn(Ptr, *const u16, u32, *const u32, *mut bool, *mut AkFileDesc) -> i32,
    output_searched_paths_a: unsafe extern "win64" fn(Ptr) -> i32,
    output_searched_paths_b: unsafe extern "win64" fn(Ptr) -> i32,
}

#[repr(C)]
struct HookVtbl {
    dtor: unsafe extern "win64" fn(Ptr, u32) -> Ptr,
    close: unsafe extern "win64" fn(Ptr, *mut AkFileDesc) -> i32,
    block_size: unsafe extern "win64" fn(Ptr, *mut AkFileDesc) -> u32,
    device_desc: unsafe extern "win64" fn(Ptr, Ptr),
    device_data: unsafe extern "win64" fn(Ptr) -> u32,
    read: unsafe extern "win64" fn(Ptr, *mut AkFileDesc, *const c_void, *mut u8, *mut AkIOTransferInfo) -> i32,
    write: unsafe extern "win64" fn(Ptr, *mut AkFileDesc, *const c_void, *const u8, *mut AkIOTransferInfo) -> i32,
}

#[repr(C)]
struct Obj<V: 'static> {
    vtbl: &'static V,
}

static RESOLVER_VTBL: ResolverVtbl = ResolverVtbl {
    dtor: resolver::dtor,
    open_id: resolver::open_id,
    open_name: resolver::open_name,
    output_searched_paths_a: resolver::not_impl,
    output_searched_paths_b: resolver::not_impl,
};
static HOOK_VTBL: HookVtbl = HookVtbl {
    dtor: hook::dtor,
    close: hook::close,
    block_size: hook::block_size,
    device_desc: hook::device_desc,
    device_data: hook::device_data,
    read: hook::read,
    write: hook::write,
};
static RESOLVER: Obj<ResolverVtbl> = Obj { vtbl: &RESOLVER_VTBL };
static HOOK: Obj<HookVtbl> = Obj { vtbl: &HOOK_VTBL };

// ---------------------------------------------------------------------------
// Driver
// ---------------------------------------------------------------------------

pub(super) fn env_or(k: &str, d: &str) -> String {
    std::env::var(k).ok().filter(|s| !s.is_empty()).unwrap_or_else(|| d.to_string())
}

pub(super) fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Run the harness; returns the process exit code (0 = success).
///
/// `AW2_AUDIO_HARNESS_MODE`: `realtime` (default; tap vs capture, A1),
/// `replay` (offline replay of an `AW2_AK_RECORD` trace, A2) or `events`
/// (offline single-post / batch renders, A2/A3) — see [`super::akoracle`].
pub fn run() -> i32 {
    let mode = env_or("AW2_AUDIO_HARNESS_MODE", "realtime");
    let r = match mode.as_str() {
        "realtime" => unsafe { drive() },
        "replay" => unsafe { super::akoracle::replay() },
        "events" => unsafe { super::akoracle::events() },
        m => Err(format!("unknown AW2_AUDIO_HARNESS_MODE={m} (realtime|replay|events)")),
    };
    let code = match r {
        Ok(code) => code,
        Err(e) => {
            log::error!(target: "audio", "audio harness ({mode}) failed: {e}");
            1
        }
    };
    super::akrec::finish();
    code
}

/// Read `<dir>/<name>.bnk`.
pub(super) fn read_bank(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    let p = dir.join(format!("{name}.bnk"));
    std::fs::read(&p).map_err(|e| format!("{}: {e} (extract it: cargo data cat data_pc/audio/events/{name}.bnk {})", p.display(), p.display()))
}

/// What [`boot`] reports.
pub(super) struct Boot {
    pub spf: u32,
    pub rate: u32,
    /// `AkRuntimeEnvironmentMgr` SIMD flags (+8): 1 SSE, 2 SSE2, 4 SSE3,
    /// 8 SSSE3, 0x10 SSE4.1, 0x20 AVX, 0x40 F16C, 0x80 AVX2 (0x141606280).
    pub simd: Option<u32>,
    /// Mixer kernel select global 0x143931f80 `(original, current)`.
    pub mixer_simd: (u32, u32),
}

/// `AkRuntimeEnvironmentMgr::Instance()->m_uSIMDSupport` (decoded flags).
pub(super) fn simd_flags() -> Option<u32> {
    let inst: unsafe extern "win64" fn() -> *const u8 = unsafe { std::mem::transmute(resolve("?Instance@AkRuntimeEnvironmentMgr@AK@@SAPEAV12@XZ").ok()?) };
    let p = unsafe { inst() };
    (!p.is_null()).then(|| unsafe { (p.add(8) as *const u32).read_unaligned() })
}

pub(super) fn simd_desc(f: u32) -> String {
    let names = [(1, "SSE"), (2, "SSE2"), (4, "SSE3"), (8, "SSSE3"), (0x10, "SSE4.1"), (0x20, "AVX"), (0x40, "F16C"), (0x80, "AVX2")];
    let set: Vec<&str> = names.iter().filter(|(b, _)| f & b != 0).map(|(_, n)| *n).collect();
    format!("{f:#x} [{}] (AVX2 bit 0x80 {}; ROSETTA_ADVERTISE_AVX={})", set.join(" "), if f & 0x80 != 0 { "SET" } else { "clear" }, std::env::var("ROSETTA_ADVERTISE_AVX").unwrap_or_else(|_| "<unset>".into()))
}

/// Steps 1–2: MemoryMgr, StreamMgr + blocking IO device over
/// `AW2_AUDIO_HARNESS_DIR`, `SoundEngine::Init` with default settings (the
/// System sink opens our WASAPI client; `tap` = its PCM tap path).
pub(super) unsafe fn boot(ak: &Ak, tap: Option<PathBuf>) -> Result<Boot, String> {
    // 1. Memory + streaming.
    if !(ak.mem_is_init)() {
        let mut mem = vec![0u8; 1024];
        (ak.mem_defaults)(mem.as_mut_ptr());
        let r = (ak.mem_init)(mem.as_mut_ptr());
        if r != AK_SUCCESS {
            return Err(format!("MemoryMgr::Init -> {}", akr(r)));
        }
    } else {
        log::info!(target: "audio", "harness: MemoryMgr already initialized");
    }
    if (*ak.stream_mgr_global).is_null() {
        let settings = [0u8; 64]; // AkStreamMgrSettings is empty in 2022.1 (GetDefaultSettings is a no-op)
        if (ak.stm_create)(settings.as_ptr()).is_null() {
            return Err("StreamMgr::Create returned NULL".into());
        }
    }
    let mut dev = vec![0u8; 256];
    (ak.stm_device_defaults)(dev.as_mut_ptr());
    let sched = u32::from_le_bytes(dev[0x18..0x1c].try_into().unwrap());
    dev[0x18..0x1c].copy_from_slice(&1u32.to_le_bytes()); // AK_SCHEDULER_BLOCKING
    let id = (ak.stm_create_device)(dev.as_ptr(), &HOOK as *const Obj<HookVtbl> as *mut c_void);
    if id == u32::MAX {
        return Err("StreamMgr::CreateDevice failed".into());
    }
    DEVICE_ID.store(id, Ordering::Relaxed);
    (ak.stm_set_resolver)(&RESOLVER as *const Obj<ResolverVtbl> as *mut c_void);
    log::info!(target: "audio", "harness: blocking IO device {id} (default scheduler flags {sched:#x}), resolver set");

    // 2. Sound engine (the WASAPI sink opens our IAudioClient; tap it).
    super::tap::override_next(tap);
    let mut init = vec![0u8; 0x400];
    let mut plat = vec![0u8; 0x200];
    (ak.se_defaults)(init.as_mut_ptr());
    platform_defaults(&mut plat);
    let spf = u32::from_le_bytes(init[0x18..0x1c].try_into().unwrap());
    let rate = u32::from_le_bytes(plat[0x3c..0x40].try_into().unwrap());
    let r = (ak.se_init)(init.as_mut_ptr(), plat.as_mut_ptr());
    if r != AK_SUCCESS || !(ak.se_is_init)() {
        return Err(format!("SoundEngine::Init -> {}", akr(r)));
    }
    let simd = simd_flags();
    log::info!(target: "audio", "harness: SoundEngine::Init OK ({spf} samples/frame, platform rate {rate}); AK SIMD flags {}", simd.map_or("?".into(), simd_desc));
    let mixer_simd = mixer_simd_select(env_or("AW2_AK_FORCE_SSE", "0") == "1")?;
    Ok(Boot { spf, rate, simd, mixer_simd })
}

/// AK mixer kernel select global (0x143931f80; set to 1 by 0x14163cbd0 during
/// Init when AVX2 is reported; the AkMixer entry stubs 0x14163ccd0.. use the
/// AVX2+FMA kernels when it is 1, else the SSE kernels). Returns
/// `(original, current)`. `force_sse` writes 0.
pub(super) fn mixer_simd_select(force_sse: bool) -> Result<(u32, u32), String> {
    const VA: usize = 0x1_4393_1f80;
    let exe = crate::loader().get_module_handle(None).ok_or("no exe module")?;
    let p = (exe + (VA - 0x1_4000_0000)) as *mut u32;
    let orig = unsafe { p.read_volatile() };
    if orig > 1 {
        return Err(format!("mixer SIMD select @{VA:#x} = {orig:#x}, expected 0 or 1 (wrong exe?)"));
    }
    if force_sse {
        unsafe { p.write_volatile(0) };
    }
    let now = unsafe { p.read_volatile() };
    log::info!(target: "audio", "harness: AK mixer SIMD select @{VA:#x}: original {orig} ({}), now {now}{}", if orig == 1 { "AVX2+FMA kernels" } else { "SSE kernels" }, if force_sse { " (AW2_AK_FORCE_SSE=1: SSE kernels, no FMA)" } else { "" });
    Ok((orig, now))
}

/// `AW2_AUDIO_HARNESS_EVENT_ID` (decimal or 0x-hex ShortID) if set, else
/// `GetIDFromString(AW2_AUDIO_HARNESS_EVENT)`.
pub(super) unsafe fn event_id_from_env(ak: &Ak, default_name: &str) -> (u32, String) {
    if let Some(id) = std::env::var("AW2_AUDIO_HARNESS_EVENT_ID").ok().and_then(|s| parse_id(&s)) {
        return (id, format!("{id}"));
    }
    let event = env_or("AW2_AUDIO_HARNESS_EVENT", default_name);
    let cname = std::ffi::CString::new(event.clone()).unwrap();
    ((ak.id_from_string)(cname.as_ptr() as *const u8), event)
}

/// Decimal or `0x` hex u32.
pub(super) fn parse_id(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(h, 16).ok()
    } else {
        s.parse().ok()
    }
}

unsafe fn drive() -> Result<i32, String> {
    let dir = base_dir().clone();
    let bank = env_or("AW2_AUDIO_HARNESS_BANK", "play_gui_caseboard_hand_up");
    let seconds: f64 = env_or("AW2_AUDIO_HARNESS_SECONDS", "4").parse().unwrap_or(4.0);
    let repeat_ms: u64 = env_or("AW2_AUDIO_HARNESS_REPEAT_MS", "700").parse().unwrap_or(700);
    let init_bnk = read_bank(&dir, "Init")?;
    let ev_bnk = read_bank(&dir, &bank)?;
    let tap_path = dir.join("tap.wav");
    let cap_path = dir.join("capture.wav");
    let _ = std::fs::remove_file(&tap_path);
    let _ = std::fs::remove_file(&cap_path);

    let ak = Ak::load()?;
    log::info!(target: "audio", "harness: AK API resolved from the exe export table");
    boot(&ak, Some(tap_path.clone()))?;

    // 3. Banks, game objects, capture, events.
    for (name, b) in [("Init", &init_bnk), (bank.as_str(), &ev_bnk)] {
        let mut bank_id = 0u32;
        let r = (ak.load_bank_copy)(b.as_ptr() as *const c_void, b.len() as u32, &mut bank_id);
        if r != AK_SUCCESS {
            return Err(format!("LoadBankMemoryCopy({name}.bnk) -> {}", akr(r)));
        }
        log::info!(target: "audio", "harness: loaded {name}.bnk ({} bytes) -> bank id {bank_id:#x}", b.len());
    }
    const LISTENER: u64 = 1;
    const EMITTER: u64 = 2;
    for o in [LISTENER, EMITTER] {
        let r = (ak.register_obj)(o);
        if r != AK_SUCCESS {
            return Err(format!("RegisterGameObj({o}) -> {}", akr(r)));
        }
    }
    let r = (ak.set_default_listeners)(&LISTENER, 1);
    if r != AK_SUCCESS {
        return Err(format!("SetDefaultListeners -> {}", akr(r)));
    }
    let (event_id, event) = event_id_from_env(&ak, &bank);
    let cap_w = wide_z(&cap_path.to_string_lossy());
    let r = (ak.start_capture)(cap_w.as_ptr());
    if r != AK_SUCCESS {
        return Err(format!("StartOutputCapture -> {}", akr(r)));
    }
    log::info!(target: "audio", "harness: capture -> {}; posting '{event}' ({event_id:#x}) every {repeat_ms} ms for {seconds} s", cap_path.display());

    let t0 = std::time::Instant::now();
    let mut next_post = std::time::Duration::from_millis(200);
    let mut posted = 0u32;
    let mut failed_posts = 0u32;
    let mut tick = 0u32;
    while t0.elapsed().as_secs_f64() < seconds {
        if t0.elapsed() >= next_post && t0.elapsed().as_secs_f64() < seconds - 1.0 {
            let pid = (ak.post_event)(event_id, EMITTER, 0, std::ptr::null(), std::ptr::null_mut(), 0, std::ptr::null(), 0);
            if pid == 0 {
                failed_posts += 1;
            } else {
                posted += 1;
            }
            next_post += std::time::Duration::from_millis(repeat_ms);
        }
        // Move the emitter on a circle around the listener (exercises
        // SetPosition / AkWorldTransform in recorded traces).
        let a = tick as f64 * 0.05;
        let t = world_transform([a.cos() * 3.0, 0.0, a.sin() * 3.0]);
        (ak.set_position)(EMITTER, t.as_ptr(), 3);
        tick += 1;
        (ak.render_audio)(true);
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    log::info!(target: "audio", "harness: posted {posted} events ({failed_posts} failed, playing id 0)");
    let r = (ak.stop_capture)();
    (ak.render_audio)(true);
    std::thread::sleep(std::time::Duration::from_millis(300));
    log::info!(target: "audio", "harness: StopOutputCapture -> {}", akr(r));
    (ak.stop_all)(u64::MAX); // AK_INVALID_GAME_OBJECT = all
    (ak.render_audio)(true);
    if env_or("AW2_AUDIO_HARNESS_TERM", "1") != "0" {
        (ak.se_term)();
        log::info!(target: "audio", "harness: SoundEngine::Term done");
    } else {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // 4. Compare.
    let report = compare_files(&tap_path, &cap_path);
    let text = match &report {
        Ok((c, ta, ca)) => format!(
            "bank {bank}, event {event} ({event_id:#x}), {posted} posts\ntap     : {} ({} ch {} Hz {} bit{}, {} frames)\ncapture : {} ({} ch {} Hz {} bit{}, {} frames)\nresult  : {c}\nverdict : {}\n",
            tap_path.display(),
            ta.channels,
            ta.rate,
            ta.bits,
            if ta.float { " float" } else { "" },
            ta.frames(),
            cap_path.display(),
            ca.channels,
            ca.rate,
            ca.bits,
            if ca.float { " float" } else { "" },
            ca.frames(),
            verdict(c)
        ),
        Err(e) => format!("comparison failed: {e}\n"),
    };
    log::info!(target: "audio", "harness report:\n{text}");
    let _ = std::fs::write(dir.join("report.txt"), &text);
    Ok(match report {
        Ok((c, _, _)) if c.bit_exact() => 0,
        _ => 2,
    })
}

/// `AK::SoundEngine::GetDefaultPlatformInitSettings` (0x141580b40 +
/// 0x1415edcc0) reproduced, except `hWnd`: the original calls
/// `user32!GetForegroundWindow`, which the harness should not depend on; the
/// WASAPI sink doesn't use the window.
fn platform_defaults(p: &mut [u8]) {
    let mut put = |o: usize, v: &[u8]| p[o..o + v.len()].copy_from_slice(v);
    // hWnd @0 = NULL. Three AkThreadProperties {nPriority=1, dwAffinityMask=0, uStackSize=128K}.
    for o in [0x08, 0x14, 0x2c] {
        put(o, &1u32.to_le_bytes());
        put(o + 4, &0u32.to_le_bytes());
        put(o + 8, &0x20000u32.to_le_bytes());
    }
    put(0x20, &0u64.to_le_bytes());
    put(0x28, &0x20000u32.to_le_bytes());
    put(0x38, &4u16.to_le_bytes()); // uNumRefillsInVoice
    put(0x3c, &48000u32.to_le_bytes()); // uSampleRate
    put(0x40, &[1]); // bGlobalFocus
    put(0x44, &0x80u32.to_le_bytes()); // uMaxSystemAudioObjects
}

/// `AkWorldTransform` (0x30 bytes): front (0,0,1), top (0,1,0) as f32 at
/// +0/+0xc, position as 3×f64 at +0x18 (SetPositionInternal 0x141589660).
pub(super) fn world_transform(pos: [f64; 3]) -> [u8; 0x30] {
    let mut t = [0u8; 0x30];
    let f = [0.0f32, 0.0, 1.0, 0.0, 1.0, 0.0];
    for (i, v) in f.iter().enumerate() {
        t[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    for (i, v) in pos.iter().enumerate() {
        t[0x18 + i * 8..0x20 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    t
}

fn verdict(c: &wavcmp::Comparison) -> &'static str {
    if c.bit_exact() && c.wwise_i16.is_some() {
        "BIT-EXACT (capture is 16-bit; tap converted with Wwise's (i16)(x*32767.0f))"
    } else if c.bit_exact() {
        "BIT-EXACT"
    } else if c.active_frames > 0 && c.lsb > 0.0 && c.max_abs_diff <= c.lsb {
        "MATCH within 1 LSB of the integer capture format"
    } else if c.active_frames == 0 {
        "NO SIGNAL"
    } else {
        "MISMATCH"
    }
}

fn compare_files(tap: &Path, cap: &Path) -> Result<(wavcmp::Comparison, wavcmp::Wav, wavcmp::Wav), String> {
    let ta = wavcmp::read_wav(&std::fs::read(tap).map_err(|e| format!("{}: {e}", tap.display()))?)?;
    let ca = wavcmp::read_wav(&std::fs::read(cap).map_err(|e| format!("{}: {e}", cap.display()))?)?;
    let c = wavcmp::compare(&ta, &ca, 48000);
    Ok((c, ta, ca))
}
