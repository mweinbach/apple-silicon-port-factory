//! AK API recorder (milestone A2) & `AW2_AUDIO=native` runtime bridge (milestone A8).
//!
//! * `AW2_AK_RECORD=<trace.akt>` hooks the `AK::SoundEngine` / `AK::MusicEngine`
//!   exports that Remedy code calls directly (docs/AUDIO.md §2.2, plus the
//!   AK-internal overloads those forward to), forwards every call to the
//!   original through a trampoline (or to [`wwise_audio::runtime::NativeSoundEngine`]
//!   when `AW2_AUDIO=native`), and appends `(thread, audio frame, op, args, result)`
//!   to a compact trace ([`super::aktrace`]). Bank bytes go once each to `<trace>.banks/`.
//! * `AW2_AUDIO=native` routes all hooked `AK::SoundEngine`, `AK::MusicEngine`,
//!   and `AK::SoundEngine::Query` calls to [`wwise_audio::runtime::NativeSoundEngine`]
//!   and streams rendered 512-sample frames into [`super::backend::OutputStream`]
//!   (CoreAudio AUHAL / Null sink) and optional [`super::tap::Tap`] (`AW2_AUDIO_TAP`).
//!
//! * Hooks are installed by the loader before any game code runs
//!   ([`install_from_env`]; the loader supplies its `hooks::install`).
//! * Nested hooked calls (an overload forwarding to another hooked export,
//!   `SoundEngine::Init` registering plugin callbacks, …) are recorded only at
//!   the outermost level, per thread.
//! * `frame` comes from a `RegisterGlobalCallback(BeginRender)` callback the
//!   recorder adds right after a successful `SoundEngine::Init`, i.e. it is the
//!   number of Wwise audio frames begun so far — the replay timeline.
//! * Queries (`GetSourcePlayPosition`, `Query::*`, `GetIDFromString`, …) do not
//!   change engine state and are not recorded, but are hooked when
//!   `AW2_AUDIO=native` is enabled.
//!
//! The trace is flushed at every `RenderAudio`, `Term` and [`finish`], so a
//! crash loses at most one game frame of calls.

use super::aktrace::{self, Enc, ExtSrc, Op};
use wwise_audio::engine::ExternalSourceDesc;
use wwise_audio::runtime::{
    AudioSink, EventCallbackTarget, NativeSoundEngine, Win64EventCallback, Win64GlobalCallback,
};
use std::cell::Cell;
use std::collections::HashSet;
use std::ffi::c_void;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Native runtime singleton & CoreAudio sink bridge (Milestone A8)
// ---------------------------------------------------------------------------

static NATIVE_OVERRIDE: AtomicU8 = AtomicU8::new(0); // 0 = env, 1 = off, 2 = on

/// Override `AW2_AUDIO=native` mode programmatically (for tests).
pub fn set_native_mode(enabled: Option<bool>) {
    NATIVE_OVERRIDE.store(
        match enabled {
            None => 0,
            Some(false) => 1,
            Some(true) => 2,
        },
        Ordering::Relaxed,
    );
}

/// Is `AW2_AUDIO=native` active?
#[inline]
pub fn native_mode() -> bool {
    match NATIVE_OVERRIDE.load(Ordering::Relaxed) {
        1 => false,
        2 => true,
        _ => {
            static ENV: OnceLock<bool> = OnceLock::new();
            *ENV.get_or_init(|| {
                std::env::var("AW2_AUDIO").is_ok_and(|v| v.eq_ignore_ascii_case("native"))
            })
        }
    }
}

/// Process-wide [`NativeSoundEngine`] singleton used when `AW2_AUDIO=native`.
/// Under `#[cfg(test)]`, skips mounting the on-disk Northlight VFS so unit
/// tests do not scan game directories while holding the audio test lock.
pub fn native_engine() -> &'static NativeSoundEngine {
    static ENGINE: OnceLock<NativeSoundEngine> = OnceLock::new();
    ENGINE.get_or_init(|| {
        log::info!(target: "audio", "NativeSoundEngine singleton initialized");
        #[cfg(test)]
        {
            NativeSoundEngine::with_source(
                wwise_audio::engine::RenderOptions {
                    bus_fx: true,
                    ..wwise_audio::engine::RenderOptions::default()
                },
                None,
            )
        }
        #[cfg(not(test))]
        {
            NativeSoundEngine::default()
        }
    })
}

struct CoreAudioSink {
    inner: Mutex<CoreAudioSinkInner>,
}

struct CoreAudioSinkInner {
    shared: Arc<super::backend::StreamShared>,
    stream: Option<super::backend::OutputStream>,
    tap: Option<super::tap::Tap>,
}

static ACTIVE_SINK: Mutex<Option<Arc<CoreAudioSink>>> = Mutex::new(None);

impl CoreAudioSink {
    fn open(sample_rate: u32, channels: u16) -> Arc<Self> {
        let dev = super::backend::output_devices()
            .into_iter()
            .next()
            .unwrap_or_else(super::backend::null_device);
        let shared = super::backend::StreamShared::new(super::ring::Ring::new(
            sample_rate.max(48_000) as usize,
            channels.max(1) as usize,
        ));
        let mut stream =
            super::backend::OutputStream::open(&dev, sample_rate, channels, shared.clone(), 512)
                .ok();
        if let Some(ref mut s) = stream {
            let _ = s.start();
        }
        let mask = if channels == 1 { 4 } else { 3 };
        let tap = super::tap::next_path().and_then(|p| {
            super::tap::Tap::create(
                p,
                super::format::WaveFormat::float_mix(sample_rate, channels, mask),
            )
            .ok()
        });
        Arc::new(Self {
            inner: Mutex::new(CoreAudioSinkInner {
                shared,
                stream,
                tap,
            }),
        })
    }

    /// Tear down the host `OutputStream` and `Tap` outside any mutex so
    /// `AudioOutputUnitStop`/`AudioUnitUninitialize`/`AudioComponentInstanceDispose`
    /// and thread joins never execute under `CoreAudioSink::inner` or
    /// `NativeSoundEngine::inner`.
    fn close(&self) {
        let (stream, tap) = {
            let mut g = self.inner.lock().unwrap();
            (g.stream.take(), g.tap.take())
        };
        drop(stream);
        drop(tap);
    }
}

fn replace_active_sink(eng: &NativeSoundEngine, next: Option<Arc<CoreAudioSink>>) {
    let prev = {
        let mut g = ACTIVE_SINK.lock().unwrap();
        std::mem::replace(&mut *g, next.clone())
    };
    eng.set_output_sink(next.map(|s| s as Arc<dyn AudioSink>));
    if let Some(old) = prev {
        old.close();
    }
}

impl AudioSink for CoreAudioSink {
    fn push_samples(&self, interleaved: &[f32], channels: usize, _sample_rate: u32) {
        let ch = channels.max(1);
        let frames = interleaved.len() / ch;
        if frames == 0 {
            return;
        }
        let mut g = self.inner.lock().unwrap();
        let pad = g.shared.ring.len() as u32;
        if let Some(ref mut tap) = g.tap {
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    interleaved.as_ptr() as *const u8,
                    interleaved.len() * 4,
                )
            };
            tap.submit(frames as u32, 0, pad, Some(bytes), 0);
        }
        let _ = g.shared.ring.push(interleaved);
    }
}

// ---------------------------------------------------------------------------
// Trace writer
// ---------------------------------------------------------------------------

struct Writer {
    out: BufWriter<File>,
    path: PathBuf,
    banks: PathBuf,
    seen_banks: HashSet<String>,
    records: u64,
}

static WRITER: Mutex<Option<Writer>> = Mutex::new(None);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static FRAMES: AtomicU32 = AtomicU32::new(0);
static NEXT_THREAD: AtomicU8 = AtomicU8::new(0);

thread_local! {
    static DEPTH: Cell<u32> = const { Cell::new(0) };
    static THREAD: Cell<Option<u8>> = const { Cell::new(None) };
}

fn thread_index() -> u8 {
    THREAD.with(|t| {
        t.get().unwrap_or_else(|| {
            let i = NEXT_THREAD.fetch_add(1, Ordering::Relaxed);
            t.set(Some(i));
            i
        })
    })
}

/// Wwise audio frames begun so far (BeginRender callback count).
pub fn frames() -> u32 {
    FRAMES.load(Ordering::Relaxed)
}

/// Is a recording in progress?
pub fn active() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

/// Open `path` for recording (truncates). Hooks must be installed separately.
pub fn start(path: &Path) -> Result<(), String> {
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let mut out = BufWriter::with_capacity(
        1 << 20,
        File::create(path).map_err(|e| format!("{}: {e}", path.display()))?,
    );
    out.write_all(aktrace::MAGIC).map_err(|e| e.to_string())?;
    let banks = aktrace::banks_dir(path);
    std::fs::create_dir_all(&banks).map_err(|e| format!("{}: {e}", banks.display()))?;
    *WRITER.lock().unwrap() = Some(Writer {
        out,
        path: path.to_path_buf(),
        banks,
        seen_banks: HashSet::new(),
        records: 0,
    });
    ACTIVE.store(true, Ordering::Relaxed);
    log::info!(target: "audio", "AK recorder: tracing to {}", path.display());
    Ok(())
}

/// Flush and close the trace (idempotent). Returns the record count.
pub fn finish() -> u64 {
    ACTIVE.store(false, Ordering::Relaxed);
    let mut g = WRITER.lock().unwrap();
    if let Some(mut w) = g.take() {
        let _ = w.out.flush();
        log::info!(target: "audio", "AK recorder: {} records -> {}", w.records, w.path.display());
        return w.records;
    }
    0
}

fn emit(op: Op, thread: u8, frame: u32, ret: u32, payload: &[u8], flush: bool) {
    let mut g = WRITER.lock().unwrap();
    if let Some(w) = g.as_mut() {
        if let Err(e) = aktrace::write_record(&mut w.out, op, thread, frame, ret, payload) {
            log::error!(target: "audio", "AK recorder: write failed ({e}); stopping");
            ACTIVE.store(false, Ordering::Relaxed);
            return;
        }
        w.records += 1;
        if flush {
            let _ = w.out.flush();
        }
    }
}

/// Write a bank's bytes to `<trace>.banks/` once; returns the content hash.
unsafe fn save_bank(ptr: *const u8, size: u32, bank_id: u32) -> u64 {
    if ptr.is_null() || size == 0 {
        return 0;
    }
    let data = std::slice::from_raw_parts(ptr, size as usize);
    let hash = aktrace::fnv1a64(data);
    let name = aktrace::bank_file_name(bank_id, size, hash);
    let mut g = WRITER.lock().unwrap();
    if let Some(w) = g.as_mut() {
        if w.seen_banks.insert(name.clone()) {
            let p = w.banks.join(&name);
            if !p.exists() {
                if let Err(e) = std::fs::write(&p, data) {
                    log::error!(target: "audio", "AK recorder: {}: {e}", p.display());
                }
            }
        }
    }
    hash
}

/// Run `call`; if this is the outermost hooked call on this thread and a
/// recording is active, encode the arguments + result with `enc` and append.
#[inline(always)]
fn record<R: Copy>(op: Op, call: impl FnOnce() -> R, enc: impl FnOnce(&mut Enc, R) -> u32) -> R {
    let outer = DEPTH.with(|d| {
        let v = d.get();
        d.set(v + 1);
        v == 0
    });
    let frame = frames();
    let r = call();
    DEPTH.with(|d| d.set(d.get() - 1));
    if outer && active() {
        let mut e = Enc::default();
        let ret = enc(&mut e, r);
        let flush = matches!(op, Op::RenderAudio | Op::Term);
        emit(op, thread_index(), frame, ret, &e.0, flush);
    }
    r
}

// ---------------------------------------------------------------------------
// Frame counter (global callback)
// ---------------------------------------------------------------------------

/// `AkGlobalCallbackLocation_BeginRender` (1 << 4); the location Remedy
/// registers its own global callback for (0x1424ca310).
pub const LOC_BEGIN_RENDER: u32 = 0x10;

pub type GlobalCallback = unsafe extern "win64" fn(*mut c_void, u32, *mut c_void);
pub type RegisterGlobalCallbackFn =
    unsafe extern "win64" fn(GlobalCallback, u32, *mut c_void, u32, u32, u32) -> i32;

unsafe extern "win64" fn on_begin_render(_ctx: *mut c_void, _loc: u32, _cookie: *mut c_void) {
    FRAMES.fetch_add(1, Ordering::Relaxed);
}

/// Register the BeginRender frame counter with `register` (the original
/// `RegisterGlobalCallback`). Call after `SoundEngine::Init` succeeded.
pub unsafe fn register_frame_counter(register: RegisterGlobalCallbackFn) -> i32 {
    FRAMES.store(0, Ordering::Relaxed);
    register(on_begin_render, LOC_BEGIN_RENDER, std::ptr::null_mut(), 0, 0, 0)
}

// ---------------------------------------------------------------------------
// Wrappers
// ---------------------------------------------------------------------------

macro_rules! wrap {
    ($slot:ident, $op:expr, fn $f:ident($($a:ident: $t:ty),*) -> $r:ty, $native:block, |$e:ident, $ret:ident| $body:block) => {
        static $slot: AtomicUsize = AtomicUsize::new(0);
        #[allow(unused_variables)]
        unsafe extern "win64" fn $f($($a: $t),*) -> $r {
            record(
                $op,
                || {
                    if native_mode() {
                        $native
                    } else {
                        let o: unsafe extern "win64" fn($($t),*) -> $r =
                            std::mem::transmute($slot.load(Ordering::Relaxed));
                        o($($a),*)
                    }
                },
                |$e: &mut Enc, $ret: $r| -> u32 { $body },
            )
        }
    };
}

unsafe fn wslice(p: *const u16) -> Vec<u16> {
    if p.is_null() {
        return Vec::new();
    }
    let mut n = 0;
    while *p.add(n) != 0 && n < 4096 {
        n += 1;
    }
    std::slice::from_raw_parts(p, n).to_vec()
}

unsafe fn cstr(p: *const u8) -> Vec<u8> {
    if p.is_null() {
        return Vec::new();
    }
    std::ffi::CStr::from_ptr(p as *const std::ffi::c_char)
        .to_bytes()
        .to_vec()
}

unsafe fn raw<'a>(p: *const u8, n: usize) -> &'a [u8] {
    if p.is_null() {
        &[]
    } else {
        std::slice::from_raw_parts(p, n)
    }
}

// lifecycle
static T_REGISTER_GLOBAL_CB: AtomicUsize = AtomicUsize::new(0);

wrap!(
    T_INIT,
    Op::Init,
    fn se_init(init: *mut u8, plat: *mut u8) -> i32,
    {
        let eng = native_engine();
        let r = eng.init();
        FRAMES.store(0, Ordering::Relaxed);
        let sink = CoreAudioSink::open(48_000, 2);
        replace_active_sink(eng, Some(sink));
        r
    },
    |e, r| {
        e.bytes(raw(init, aktrace::INIT_SETTINGS_LEN))
            .bytes(raw(plat, aktrace::PLATFORM_SETTINGS_LEN));
        r as u32
    }
);

/// `SoundEngine::Init` hook: record, then start the frame counter.
unsafe extern "win64" fn se_init_hook(init: *mut u8, plat: *mut u8) -> i32 {
    let r = se_init(init, plat);
    if r == 1 && !native_mode() {
        let reg = T_REGISTER_GLOBAL_CB.load(Ordering::Relaxed);
        if reg != 0 {
            let rr = register_frame_counter(std::mem::transmute::<usize, RegisterGlobalCallbackFn>(
                reg,
            ));
            log::info!(target: "audio", "AK recorder: SoundEngine::Init ok; BeginRender frame counter registered ({rr})");
        }
    }
    r
}

wrap!(
    T_TERM,
    Op::Term,
    fn se_term() -> (),
    {
        let eng = native_engine();
        replace_active_sink(eng, None);
        eng.term();
    },
    |e, r| { 0 }
);

wrap!(
    T_MUSIC_INIT,
    Op::MusicInit,
    fn music_init(s: *mut u8) -> i32,
    { native_engine().music_init() },
    |e, r| {
        e.blob(raw(s, 4)); // AkMusicSettings { fStreamingLookAheadRatio }
        r as u32
    }
);

wrap!(
    T_MUSIC_TERM,
    Op::MusicTerm,
    fn music_term() -> (),
    { native_engine().music_term() },
    |e, r| { 0 }
);

unsafe extern "win64" fn register_global_cb(
    f: GlobalCallback,
    loc: u32,
    cookie: *mut c_void,
    ty: u32,
    company: u32,
    plugin: u32,
) -> i32 {
    record(
        Op::RegisterGlobalCallback,
        || {
            if native_mode() {
                let cb: Win64GlobalCallback = std::mem::transmute(f as usize);
                native_engine().register_global_callback_win64(cb, loc, cookie as u64)
            } else {
                let o: RegisterGlobalCallbackFn =
                    std::mem::transmute(T_REGISTER_GLOBAL_CB.load(Ordering::Relaxed));
                o(f, loc, cookie, ty, company, plugin)
            }
        },
        |e, r| {
            e.u64(f as usize as u64)
                .u32(loc)
                .u64(cookie as u64)
                .u32(ty)
                .u32(company)
                .u32(plugin);
            r as u32
        },
    )
}

wrap!(
    T_REG_BUS_VOL_CB,
    Op::RegisterBusVolumeCallback,
    fn reg_bus_vol_cb(bus: u32, f: *const c_void, cookie: *mut c_void) -> i32,
    { wwise_audio::runtime::AK_SUCCESS },
    |e, r| {
        e.u32(bus).u64(f as u64).u64(cookie as u64);
        r as u32
    }
);

// frame boundary: no depth (never nested inside another API call)
static T_RENDER: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn render_audio(allow_sync: bool) -> i32 {
    let frame = frames();
    let r = if native_mode() {
        let eng = native_engine();
        if eng.is_initialized() && !eng.has_output_sink() {
            let sink = CoreAudioSink::open(48_000, 2);
            replace_active_sink(eng, Some(sink));
        }
        let res = match eng.render_audio(allow_sync) {
            Ok(_) => wwise_audio::runtime::AK_SUCCESS,
            Err(e) => {
                log::warn!(target: "audio", "NativeSoundEngine::render_audio error: {e}");
                wwise_audio::runtime::AK_FAIL
            }
        };
        FRAMES.store(eng.frames_rendered(), Ordering::Relaxed);
        res
    } else {
        let o: unsafe extern "win64" fn(bool) -> i32 =
            std::mem::transmute(T_RENDER.load(Ordering::Relaxed));
        o(allow_sync)
    };
    if active() && DEPTH.with(|d| d.get()) == 0 {
        emit(
            Op::RenderAudio,
            thread_index(),
            frame,
            r as u32,
            &[allow_sync as u8],
            true,
        );
    }
    r
}

// game objects
wrap!(
    T_REG_OBJ,
    Op::RegisterGameObj,
    fn register_obj(obj: u64, _name: *const u8) -> i32,
    { native_engine().register_game_obj(obj, None) },
    |e, r| {
        e.u64(obj);
        r as u32
    }
);

wrap!(
    T_UNREG_OBJ,
    Op::UnregisterGameObj,
    fn unregister_obj(obj: u64) -> i32,
    { native_engine().unregister_game_obj(obj) },
    |e, r| {
        e.u64(obj);
        r as u32
    }
);

wrap!(
    T_UNREG_ALL,
    Op::UnregisterAllGameObj,
    fn unregister_all() -> i32,
    { native_engine().unregister_all_game_obj() },
    |e, r| { r as u32 }
);

wrap!(
    T_SET_POS,
    Op::SetPosition,
    fn set_position(obj: u64, t: *const u8, flags: i32) -> i32,
    { native_engine().set_position_bytes(obj, raw(t, aktrace::WORLD_TRANSFORM_LEN)) },
    |e, r| {
        e.u64(obj)
            .bytes(raw(t, aktrace::WORLD_TRANSFORM_LEN))
            .i32(flags);
        r as u32
    }
);

wrap!(
    T_SET_MULTI_POS,
    Op::SetMultiplePositions,
    fn set_multi_pos(obj: u64, t: *const u8, n: u16, ty: i32, flags: i32) -> i32,
    {
        native_engine().set_multiple_positions_bytes(
            obj,
            raw(t, n as usize * aktrace::WORLD_TRANSFORM_LEN),
            n as usize,
            ty,
        )
    },
    |e, r| {
        e.u64(obj)
            .u16(n)
            .i32(ty)
            .i32(flags)
            .bytes(raw(t, n as usize * aktrace::WORLD_TRANSFORM_LEN));
        r as u32
    }
);

wrap!(
    T_SET_DEFAULT_LISTENERS,
    Op::SetDefaultListeners,
    fn set_default_listeners(l: *const u64, n: u32) -> i32,
    {
        let slice = if l.is_null() || n == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(l, n as usize)
        };
        native_engine().set_default_listeners(slice)
    },
    |e, r| {
        e.u32(n);
        for i in 0..n as usize {
            e.u64(if l.is_null() { 0 } else { *l.add(i) });
        }
        r as u32
    }
);

wrap!(
    T_SET_DIST_PROBE,
    Op::SetDistanceProbe,
    fn set_distance_probe(listener: u64, probe: u64) -> i32,
    { native_engine().set_distance_probe(listener, probe) },
    |e, r| {
        e.u64(listener).u64(probe);
        r as u32
    }
);

wrap!(
    T_SET_SCALING,
    Op::SetScalingFactor,
    fn set_scaling(obj: u64, f: f32) -> i32,
    { native_engine().set_scaling_factor(obj, f) },
    |e, r| {
        e.u64(obj).f32(f);
        r as u32
    }
);

wrap!(
    T_SET_OBS_OCC,
    Op::SetObjectObstructionAndOcclusion,
    fn set_obs_occ(obj: u64, listener: u64, obs: f32, occ: f32) -> i32,
    { native_engine().set_object_obstruction_and_occlusion(obj, listener, obs, occ) },
    |e, r| {
        e.u64(obj).u64(listener).f32(obs).f32(occ);
        r as u32
    }
);

wrap!(
    T_SET_AUX,
    Op::SetGameObjectAuxSendValues,
    fn set_aux(obj: u64, v: *const u8, n: u32) -> i32,
    {
        native_engine().set_game_object_aux_send_bytes(
            obj,
            raw(v, n as usize * aktrace::AUX_SEND_LEN),
            n as usize,
        )
    },
    |e, r| {
        e.u64(obj)
            .u32(n)
            .bytes(raw(v, n as usize * aktrace::AUX_SEND_LEN));
        r as u32
    }
);

wrap!(
    T_SET_OUT_BUS_VOL,
    Op::SetGameObjectOutputBusVolume,
    fn set_out_bus_vol(emitter: u64, listener: u64, v: f32) -> i32,
    { native_engine().set_game_object_output_bus_volume(emitter, listener, v) },
    |e, r| {
        e.u64(emitter).u64(listener).f32(v);
        r as u32
    }
);

// game syncs
wrap!(
    T_SET_RTPC,
    Op::SetRTPCValue,
    fn set_rtpc(id: u32, v: f32, obj: u64, ms: i32, curve: i32, bypass: bool) -> i32,
    { native_engine().set_rtpc_value(id, v, obj, ms, curve, bypass) },
    |e, r| {
        e.u32(id)
            .f32(v)
            .u64(obj)
            .i32(ms)
            .i32(curve)
            .u8(bypass as u8);
        r as u32
    }
);

wrap!(
    T_SET_RTPC_PID,
    Op::SetRTPCValueByPlayingID,
    fn set_rtpc_pid(id: u32, v: f32, pid: u32, ms: i32, curve: i32, bypass: bool) -> i32,
    { native_engine().set_rtpc_value_by_playing_id(id, v, pid, ms, curve, bypass) },
    |e, r| {
        e.u32(id)
            .f32(v)
            .u32(pid)
            .i32(ms)
            .i32(curve)
            .u8(bypass as u8);
        r as u32
    }
);

wrap!(
    T_RESET_RTPC,
    Op::ResetRTPCValue,
    fn reset_rtpc(id: u32, obj: u64, ms: i32, curve: i32, bypass: bool) -> i32,
    { native_engine().reset_rtpc_value(id, obj, ms, curve, bypass) },
    |e, r| {
        e.u32(id).u64(obj).i32(ms).i32(curve).u8(bypass as u8);
        r as u32
    }
);

wrap!(
    T_SET_SWITCH,
    Op::SetSwitch,
    fn set_switch(group: u32, state: u32, obj: u64) -> i32,
    { native_engine().set_switch(group, state, obj) },
    |e, r| {
        e.u32(group).u32(state).u64(obj);
        r as u32
    }
);

wrap!(
    T_SET_STATE,
    Op::SetState,
    fn set_state(group: u32, state: u32) -> i32,
    { native_engine().set_state(group, state) },
    |e, r| {
        e.u32(group).u32(state);
        r as u32
    }
);

wrap!(
    T_POST_TRIGGER,
    Op::PostTrigger,
    fn post_trigger(trigger: u32, obj: u64) -> i32,
    { native_engine().post_trigger(trigger, obj) },
    |e, r| {
        e.u32(trigger).u64(obj);
        r as u32
    }
);

// events
wrap!(
    T_POST_EVENT,
    Op::PostEvent,
    fn post_event(
        ev: u32,
        obj: u64,
        flags: u32,
        cb: *const c_void,
        cookie: *mut c_void,
        n: u32,
        ext: *const u8,
        pid: u32
    ) -> u32,
    {
        let mut ext_descs = Vec::new();
        if !ext.is_null() {
            for i in 0..n as usize {
                let p = ext.add(i * 0x20);
                let rd32 = |o: usize| (p.add(o) as *const u32).read_unaligned();
                let ext_cookie = rd32(0);
                let codec = rd32(4);
                let sz_file = (p.add(8) as *const *const u16).read_unaligned();
                let p_mem = (p.add(0x10) as *const *const u8).read_unaligned();
                let mem_size = rd32(0x18);
                let file_id = rd32(0x1c);
                let path = if sz_file.is_null() {
                    None
                } else {
                    let w = wslice(sz_file);
                    (!w.is_empty()).then(|| String::from_utf16_lossy(&w))
                };
                let bytes = if !p_mem.is_null() && mem_size > 0 {
                    Some(std::slice::from_raw_parts(p_mem, mem_size as usize).to_vec())
                } else {
                    None
                };
                ext_descs.push(ExternalSourceDesc {
                    cookie: ext_cookie,
                    codec,
                    path,
                    file_id,
                    bytes,
                });
            }
        }
        let target = if cb.is_null() {
            None
        } else {
            let win_cb: Win64EventCallback = std::mem::transmute(cb as usize);
            Some(EventCallbackTarget::Win64(win_cb))
        };
        native_engine().post_event(ev, obj, flags, target, cookie as u64, ext_descs, pid)
    },
    |e, r| {
        e.u32(ev)
            .u64(obj)
            .u32(flags)
            .u64(cb as u64)
            .u64(cookie as u64)
            .u32(pid)
            .u32(if ext.is_null() { 0 } else { n });
        if !ext.is_null() {
            for i in 0..n as usize {
                let p = ext.add(i * 0x20);
                let rd32 = |o: usize| (p.add(o) as *const u32).read_unaligned();
                let x = ExtSrc {
                    cookie: rd32(0),
                    codec: rd32(4),
                    path: wslice((p.add(8) as *const *const u16).read_unaligned()),
                    has_mem: !(p.add(0x10) as *const *const u8).read_unaligned().is_null(),
                    mem_size: rd32(0x18),
                    file_id: rd32(0x1c),
                };
                e.ext(&x);
            }
        }
        r
    }
);

wrap!(
    T_EXEC_ACTION,
    Op::ExecuteActionOnEvent,
    fn exec_action(ev: u32, action: i32, obj: u64, ms: i32, curve: i32, pid: u32) -> i32,
    { native_engine().execute_action_on_event(ev, action, obj, ms, curve, pid) },
    |e, r| {
        e.u32(ev).i32(action).u64(obj).i32(ms).i32(curve).u32(pid);
        r as u32
    }
);

wrap!(
    T_EXEC_ACTION_NAME,
    Op::ExecuteActionOnEventName,
    fn exec_action_name(name: *const u8, action: i32, obj: u64, ms: i32, curve: i32, pid: u32) -> i32,
    {
        let raw_c = cstr(name);
        let s = String::from_utf8_lossy(&raw_c);
        native_engine().execute_action_on_event_name(&s, action, obj, ms, curve, pid)
    },
    |e, r| {
        e.blob(&cstr(name))
            .i32(action)
            .u64(obj)
            .i32(ms)
            .i32(curve)
            .u32(pid);
        r as u32
    }
);

wrap!(
    T_SEEK,
    Op::SeekOnEvent,
    fn seek_on_event(ev: u32, obj: u64, ms: i32, nearest: bool, pid: u32) -> i32,
    { native_engine().seek_on_event(ev, obj, ms, nearest, pid) },
    |e, r| {
        e.u32(ev).u64(obj).i32(ms).u8(nearest as u8).u32(pid);
        r as u32
    }
);

wrap!(
    T_SEEK_PCT,
    Op::SeekOnEventPercent,
    fn seek_on_event_pct(ev: u32, obj: u64, pct: f32, nearest: bool, pid: u32) -> i32,
    { native_engine().seek_on_event_percent(ev, obj, pct, nearest, pid) },
    |e, r| {
        e.u32(ev).u64(obj).f32(pct).u8(nearest as u8).u32(pid);
        r as u32
    }
);

wrap!(
    T_STOP_ALL,
    Op::StopAll,
    fn stop_all(obj: u64) -> (),
    { native_engine().stop_all(obj) },
    |e, r| {
        e.u64(obj);
        0
    }
);

wrap!(
    T_STOP_PID,
    Op::StopPlayingID,
    fn stop_pid(pid: u32, ms: i32, curve: i32) -> (),
    { native_engine().stop_playing_id(pid, ms, curve) },
    |e, r| {
        e.u32(pid).i32(ms).i32(curve);
        0
    }
);

wrap!(
    T_PIN,
    Op::PinEventInStreamCache,
    fn pin_event(ev: u32, prio: i8, active: i8) -> i32,
    { native_engine().pin_event(ev, prio, active) },
    |e, r| {
        e.u32(ev).u8(prio as u8).u8(active as u8);
        r as u32
    }
);

wrap!(
    T_UNPIN,
    Op::UnpinEventInStreamCache,
    fn unpin_event(ev: u32) -> i32,
    { native_engine().unpin_event(ev) },
    |e, r| {
        e.u32(ev);
        r as u32
    }
);

// banks
wrap!(
    T_LOAD_COPY,
    Op::LoadBankMemoryCopy,
    fn load_bank_copy(p: *const u8, size: u32, out_id: *mut u32) -> i32,
    {
        let bytes = raw(p, size as usize).to_vec();
        match native_engine().load_bank_bytes(format!("mem_{p:p}_{size}"), bytes) {
            Ok(id) => {
                if !out_id.is_null() {
                    *out_id = id;
                }
                wwise_audio::runtime::AK_SUCCESS
            }
            Err(e) => {
                log::warn!(target: "audio", "NativeSoundEngine::load_bank_copy failed: {e}");
                wwise_audio::runtime::AK_FAIL
            }
        }
    },
    |e, r| {
        let id = if out_id.is_null() { 0 } else { *out_id };
        let hash = save_bank(p, size, id);
        e.u64(p as u64).u32(size).u32(id).u64(hash);
        r as u32
    }
);

wrap!(
    T_LOAD_VIEW,
    Op::LoadBankMemoryView,
    fn load_bank_view(p: *const u8, size: u32, out_id: *mut u32) -> i32,
    {
        let bytes = raw(p, size as usize).to_vec();
        match native_engine().load_bank_bytes(format!("view_{p:p}_{size}"), bytes) {
            Ok(id) => {
                if !out_id.is_null() {
                    *out_id = id;
                }
                wwise_audio::runtime::AK_SUCCESS
            }
            Err(e) => {
                log::warn!(target: "audio", "NativeSoundEngine::load_bank_view failed: {e}");
                wwise_audio::runtime::AK_FAIL
            }
        }
    },
    |e, r| {
        let id = if out_id.is_null() { 0 } else { *out_id };
        let hash = save_bank(p, size, id);
        e.u64(p as u64).u32(size).u32(id).u64(hash);
        r as u32
    }
);

wrap!(
    T_UNLOAD,
    Op::UnloadBank,
    fn unload_bank(id: u32, p: *const u8, ty: u32) -> i32,
    {
        let _ = native_engine().unload_bank(id);
        wwise_audio::runtime::AK_SUCCESS
    },
    |e, r| {
        e.u32(id).u64(p as u64).u32(ty);
        r as u32
    }
);

// outputs
wrap!(
    T_ADD_OUTPUT,
    Op::AddOutput,
    fn add_output(s: *const u8, out_id: *mut u64, l: *const u64, n: u32) -> i32,
    {
        let listeners = if l.is_null() || n == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(l, n as usize)
        };
        let (r, id) =
            native_engine().add_output(raw(s, aktrace::OUTPUT_SETTINGS_LEN), listeners);
        if !out_id.is_null() {
            *out_id = id;
        }
        r
    },
    |e, r| {
        e.bytes(raw(s, aktrace::OUTPUT_SETTINGS_LEN))
            .u32(if l.is_null() { 0 } else { n });
        if !l.is_null() {
            for i in 0..n as usize {
                e.u64(*l.add(i));
            }
        }
        e.u64(if out_id.is_null() { 0 } else { *out_id });
        r as u32
    }
);

wrap!(
    T_REMOVE_OUTPUT,
    Op::RemoveOutput,
    fn remove_output(id: u64) -> i32,
    { native_engine().remove_output(id) },
    |e, r| {
        e.u64(id);
        r as u32
    }
);

wrap!(
    T_SPEAKER_ANGLES,
    Op::SetSpeakerAngles,
    fn set_speaker_angles(a: *const f32, n: u32, height: f32, dev: u64) -> i32,
    {
        let angles = if a.is_null() || n == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(a, n as usize)
        };
        native_engine().set_speaker_angles(angles, height, dev)
    },
    |e, r| {
        e.u32(n);
        for i in 0..n as usize {
            e.f32(if a.is_null() { 0.0 } else { *a.add(i) });
        }
        e.f32(height).u64(dev);
        r as u32
    }
);

// ---------------------------------------------------------------------------
// Native-only query / utility hooks (not recorded in .akt traces)
// ---------------------------------------------------------------------------

static T_IS_INITIALIZED: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn se_is_initialized() -> bool {
    if native_mode() {
        native_engine().is_initialized()
    } else {
        let o: unsafe extern "win64" fn() -> bool =
            std::mem::transmute(T_IS_INITIALIZED.load(Ordering::Relaxed));
        o()
    }
}

static T_GET_ID_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_id_from_string_a(s: *const u8) -> u32 {
    if native_mode() {
        if s.is_null() {
            0
        } else {
            wwise_audio::fnv1_32_lower(&String::from_utf8_lossy(&cstr(s)))
        }
    } else {
        let o: unsafe extern "win64" fn(*const u8) -> u32 =
            std::mem::transmute(T_GET_ID_A.load(Ordering::Relaxed));
        o(s)
    }
}

static T_GET_ID_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_id_from_string_w(s: *const u16) -> u32 {
    if native_mode() {
        if s.is_null() {
            0
        } else {
            wwise_audio::fnv1_32_lower(&String::from_utf16_lossy(&wslice(s)))
        }
    } else {
        let o: unsafe extern "win64" fn(*const u16) -> u32 =
            std::mem::transmute(T_GET_ID_W.load(Ordering::Relaxed));
        o(s)
    }
}

static T_GET_SOURCE_PLAY_POS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_source_play_position(
    pid: u32,
    out_pos: *mut i32,
    extrap: bool,
) -> i32 {
    if native_mode() {
        if out_pos.is_null() {
            return wwise_audio::runtime::AK_INVALID_PARAMETER;
        }
        match native_engine().get_source_play_position(pid) {
            Ok(pos) => {
                *out_pos = pos;
                wwise_audio::runtime::AK_SUCCESS
            }
            Err(code) => code,
        }
    } else {
        let o: unsafe extern "win64" fn(u32, *mut i32, bool) -> i32 =
            std::mem::transmute(T_GET_SOURCE_PLAY_POS.load(Ordering::Relaxed));
        o(pid, out_pos, extrap)
    }
}

static T_GET_SPEAKER_ANGLES: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_speaker_angles(
    angles: *mut f32,
    io_num: *mut u32,
    out_height: *mut f32,
    dev: u64,
) -> i32 {
    if native_mode() {
        if !out_height.is_null() {
            *out_height = 0.0;
        }
        if !io_num.is_null() {
            let n = (*io_num as usize).min(1);
            if !angles.is_null() && n > 0 {
                *angles = 45.0;
            }
            *io_num = 1;
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*mut f32, *mut u32, *mut f32, u64) -> i32 =
            std::mem::transmute(T_GET_SPEAKER_ANGLES.load(Ordering::Relaxed));
        o(angles, io_num, out_height, dev)
    }
}

static T_GET_OUTPUT_ID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_output_id(shareset: u32, dev: u32) -> u64 {
    if native_mode() {
        ((dev as u64) << 32) | (shareset as u64)
    } else {
        let o: unsafe extern "win64" fn(u32, u32) -> u64 =
            std::mem::transmute(T_GET_OUTPUT_ID.load(Ordering::Relaxed));
        o(shareset, dev)
    }
}

static T_GET_OUTPUT_ID_NAME: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_output_id_name(name: *const u8, dev: u32) -> u64 {
    if native_mode() {
        let shareset = if name.is_null() {
            0
        } else {
            wwise_audio::fnv1_32_lower(&String::from_utf8_lossy(&cstr(name)))
        };
        ((dev as u64) << 32) | (shareset as u64)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u32) -> u64 =
            std::mem::transmute(T_GET_OUTPUT_ID_NAME.load(Ordering::Relaxed));
        o(name, dev)
    }
}

static T_QUERY_GET_RTPC: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_rtpc(
    rtpc_id: u32,
    obj: u64,
    pid: u32,
    out_val: *mut f32,
    io_type: *mut i32,
) -> i32 {
    if native_mode() {
        let (val, ty) = native_engine().get_rtpc_value(rtpc_id, obj, pid);
        if !out_val.is_null() {
            *out_val = val;
        }
        if !io_type.is_null() {
            *io_type = ty;
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32, u64, u32, *mut f32, *mut i32) -> i32 =
            std::mem::transmute(T_QUERY_GET_RTPC.load(Ordering::Relaxed));
        o(rtpc_id, obj, pid, out_val, io_type)
    }
}

static T_QUERY_GET_POS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_position(obj: u64, out_pos: *mut u8) -> i32 {
    if native_mode() {
        if out_pos.is_null() {
            return wwise_audio::runtime::AK_INVALID_PARAMETER;
        }
        let mut buf = [0u8; 0x30];
        let r = native_engine().get_position_bytes(obj, &mut buf);
        std::ptr::copy_nonoverlapping(buf.as_ptr(), out_pos, 0x30);
        r
    } else {
        let o: unsafe extern "win64" fn(u64, *mut u8) -> i32 =
            std::mem::transmute(T_QUERY_GET_POS.load(Ordering::Relaxed));
        o(obj, out_pos)
    }
}

static T_QUERY_GET_PIDS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_playing_ids(
    obj: u64,
    io_num: *mut u32,
    out_pids: *mut u32,
) -> i32 {
    if native_mode() {
        if io_num.is_null() {
            return wwise_audio::runtime::AK_INVALID_PARAMETER;
        }
        let pids = native_engine().get_playing_ids_from_game_object(obj);
        let cap = *io_num as usize;
        if !out_pids.is_null() {
            for (i, &pid) in pids.iter().take(cap).enumerate() {
                *out_pids.add(i) = pid;
            }
        }
        *io_num = pids.len() as u32;
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u64, *mut u32, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_PIDS.load(Ordering::Relaxed));
        o(obj, io_num, out_pids)
    }
}

static T_QUERY_MAX_RADIUS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_max_radius(obj: u64) -> f32 {
    if native_mode() {
        100.0
    } else {
        let o: unsafe extern "win64" fn(u64) -> f32 =
            std::mem::transmute(T_QUERY_MAX_RADIUS.load(Ordering::Relaxed));
        o(obj)
    }
}

static T_SET_LISTENERS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_listeners(emitter: u64, listeners: *const u64, count: u32) -> i32 {
    if native_mode() {
        let slice = if listeners.is_null() || count == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(listeners, count as usize)
        };
        native_engine().set_listeners(emitter, slice)
    } else {
        let o: unsafe extern "win64" fn(u64, *const u64, u32) -> i32 =
            std::mem::transmute(T_SET_LISTENERS.load(Ordering::Relaxed));
        o(emitter, listeners, count)
    }
}

static T_START_CAPTURE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn start_output_capture(file: *const u16) -> i32 {
    if native_mode() {
        let raw_name = String::from_utf16_lossy(&wslice(file)).replace('\\', "/");
        let path = if Path::new(&raw_name).is_absolute() || raw_name.contains('/') {
            PathBuf::from(raw_name)
        } else {
            let dir = std::env::var("AW2_AUDIO_HARNESS_DIR")
                .unwrap_or_else(|_| "out/audio_harness".into());
            PathBuf::from(dir).join(raw_name)
        };
        native_engine().start_output_capture(path)
    } else {
        let o: unsafe extern "win64" fn(*const u16) -> i32 =
            std::mem::transmute(T_START_CAPTURE.load(Ordering::Relaxed));
        o(file)
    }
}

static T_STOP_CAPTURE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stop_output_capture() -> i32 {
    if native_mode() {
        match native_engine().stop_output_capture() {
            Ok(()) => wwise_audio::runtime::AK_SUCCESS,
            Err(e) => {
                log::warn!(target: "audio", "NativeSoundEngine::stop_output_capture failed: {e}");
                wwise_audio::runtime::AK_FAIL
            }
        }
    } else {
        let o: unsafe extern "win64" fn() -> i32 =
            std::mem::transmute(T_STOP_CAPTURE.load(Ordering::Relaxed));
        o()
    }
}

// ---------------------------------------------------------------------------
// Milestone A9 — Zero-Original-AK:: Subsystem Shims (MemoryMgr, StreamMgr,
// MusicEngine, SoundEngine default settings, device queries, string/wchar overloads)
// ---------------------------------------------------------------------------

static MEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

static T_MEM_GET_DEF: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_get_default_settings(s: *mut u8) {
    if native_mode() {
        if !s.is_null() {
            std::ptr::write_bytes(s, 0, 0xb8);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u8) =
            std::mem::transmute(T_MEM_GET_DEF.load(Ordering::Relaxed));
        o(s)
    }
}

static T_MEM_INIT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_init(s: *mut u8) -> i32 {
    if native_mode() {
        MEM_INITIALIZED.store(true, Ordering::Release);
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*mut u8) -> i32 =
            std::mem::transmute(T_MEM_INIT.load(Ordering::Relaxed));
        o(s)
    }
}

static T_MEM_IS_INIT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_is_initialized() -> bool {
    if native_mode() {
        MEM_INITIALIZED.load(Ordering::Acquire)
    } else {
        let o: unsafe extern "win64" fn() -> bool =
            std::mem::transmute(T_MEM_IS_INIT.load(Ordering::Relaxed));
        o()
    }
}

static T_MEM_TERM: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_term() {
    if native_mode() {
        MEM_INITIALIZED.store(false, Ordering::Release);
    } else {
        let o: unsafe extern "win64" fn() =
            std::mem::transmute(T_MEM_TERM.load(Ordering::Relaxed));
        o()
    }
}

static T_MEM_INIT_THREAD: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_init_for_thread() {
    if !native_mode() {
        let o: unsafe extern "win64" fn() =
            std::mem::transmute(T_MEM_INIT_THREAD.load(Ordering::Relaxed));
        o()
    }
}

static T_MEM_TERM_THREAD: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_term_for_thread() {
    if !native_mode() {
        let o: unsafe extern "win64" fn() =
            std::mem::transmute(T_MEM_TERM_THREAD.load(Ordering::Relaxed));
        o()
    }
}

static T_MEM_TRIM_THREAD: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_trim_for_thread() {
    if !native_mode() {
        let o: unsafe extern "win64" fn() =
            std::mem::transmute(T_MEM_TRIM_THREAD.load(Ordering::Relaxed));
        o()
    }
}

static T_MEM_MALLOC: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_malloc(pool: i32, size: usize) -> *mut c_void {
    if native_mode() {
        crate::heap::alloc(size.max(1), false) as *mut c_void
    } else {
        let o: unsafe extern "win64" fn(i32, usize) -> *mut c_void =
            std::mem::transmute(T_MEM_MALLOC.load(Ordering::Relaxed));
        o(pool, size)
    }
}

static T_MEM_MALIGN: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_malign(pool: i32, size: usize, align: u32) -> *mut c_void {
    if native_mode() {
        crate::heap::alloc(size.max(1), false) as *mut c_void
    } else {
        let o: unsafe extern "win64" fn(i32, usize, u32) -> *mut c_void =
            std::mem::transmute(T_MEM_MALIGN.load(Ordering::Relaxed));
        o(pool, size, align)
    }
}

static T_MEM_REALLOC: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_realloc(pool: i32, ptr: *mut c_void, size: usize) -> *mut c_void {
    if native_mode() {
        crate::heap::realloc(ptr, size.max(1), false, false) as *mut c_void
    } else {
        let o: unsafe extern "win64" fn(i32, *mut c_void, usize) -> *mut c_void =
            std::mem::transmute(T_MEM_REALLOC.load(Ordering::Relaxed));
        o(pool, ptr, size)
    }
}

static T_MEM_REALLOC_ALIGNED: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_realloc_aligned(
    pool: i32,
    ptr: *mut c_void,
    size: usize,
    align: u32,
) -> *mut c_void {
    if native_mode() {
        crate::heap::realloc(ptr, size.max(1), false, false) as *mut c_void
    } else {
        let o: unsafe extern "win64" fn(i32, *mut c_void, usize, u32) -> *mut c_void =
            std::mem::transmute(T_MEM_REALLOC_ALIGNED.load(Ordering::Relaxed));
        o(pool, ptr, size, align)
    }
}

static T_MEM_FREE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_free(pool: i32, ptr: *mut c_void) {
    if native_mode() {
        if !ptr.is_null() {
            crate::heap::free(ptr);
        }
    } else {
        let o: unsafe extern "win64" fn(i32, *mut c_void) =
            std::mem::transmute(T_MEM_FREE.load(Ordering::Relaxed));
        o(pool, ptr)
    }
}

static T_MEM_GLOBAL_STATS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn mem_get_global_stats(stats: *mut u8) {
    if native_mode() {
        if !stats.is_null() {
            std::ptr::write_bytes(stats, 0, 0x30);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u8) =
            std::mem::transmute(T_MEM_GLOBAL_STATS.load(Ordering::Relaxed));
        o(stats)
    }
}

// --- AK::StreamMgr ---

unsafe extern "win64" fn dummy_stream_mgr_vfn(_this: *mut c_void) -> usize {
    0
}

#[repr(C)]
struct DummyStreamMgr {
    vtable: *const [unsafe extern "win64" fn(*mut c_void) -> usize; 16],
}
unsafe impl Sync for DummyStreamMgr {}

static DUMMY_STREAM_MGR_VTBL: [unsafe extern "win64" fn(*mut c_void) -> usize; 16] =
    [dummy_stream_mgr_vfn; 16];
static DUMMY_STREAM_MGR: DummyStreamMgr = DummyStreamMgr {
    vtable: &DUMMY_STREAM_MGR_VTBL,
};

static FILE_LOCATION_RESOLVER: AtomicUsize = AtomicUsize::new(0);
static CURRENT_LANG_BUF: Mutex<[u16; 64]> = Mutex::new({
    let mut b = [0u16; 64];
    b[0] = b'e' as u16;
    b[1] = b'n' as u16;
    b[2] = b'(' as u16;
    b[3] = b'U' as u16;
    b[4] = b'S' as u16;
    b[5] = b')' as u16;
    b
});

static T_STM_CREATE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_create(settings: *const u8) -> *mut c_void {
    if native_mode() {
        &DUMMY_STREAM_MGR as *const DummyStreamMgr as *mut c_void
    } else {
        let o: unsafe extern "win64" fn(*const u8) -> *mut c_void =
            std::mem::transmute(T_STM_CREATE.load(Ordering::Relaxed));
        o(settings)
    }
}

static T_STM_GET_DEF_DEV: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_get_default_device_settings(s: *mut u8) {
    if native_mode() {
        if !s.is_null() {
            std::ptr::write_bytes(s, 0, 0x38);
            (s.add(0x08) as *mut u32).write_unaligned(0x0020_0000);
            (s.add(0x0c) as *mut u32).write_unaligned(0x0000_1000);
            (s.add(0x14) as *mut u32).write_unaligned(0x0000_4000);
            (s.add(0x18) as *mut u32).write_unaligned(1);
            (s.add(0x1c) as *mut u32).write_unaligned(1);
            (s.add(0x24) as *mut u32).write_unaligned(0x0002_0000);
            (s.add(0x28) as *mut f32).write_unaligned(380.0);
            (s.add(0x2c) as *mut u32).write_unaligned(8);
            (s.add(0x34) as *mut u32).write_unaligned(0xffff_ffff);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u8) =
            std::mem::transmute(T_STM_GET_DEF_DEV.load(Ordering::Relaxed));
        o(s)
    }
}

static T_STM_CREATE_DEV: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_create_device(
    settings: *const u8,
    hook_io: *mut c_void,
) -> u32 {
    if native_mode() {
        0
    } else {
        let o: unsafe extern "win64" fn(*const u8, *mut c_void) -> u32 =
            std::mem::transmute(T_STM_CREATE_DEV.load(Ordering::Relaxed));
        o(settings, hook_io)
    }
}

static T_STM_DESTROY_DEV: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_destroy_device(dev_id: u32) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32) -> i32 =
            std::mem::transmute(T_STM_DESTROY_DEV.load(Ordering::Relaxed));
        o(dev_id)
    }
}

static T_STM_GET_RESOLVER: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_get_file_location_resolver() -> *mut c_void {
    if native_mode() {
        FILE_LOCATION_RESOLVER.load(Ordering::Acquire) as *mut c_void
    } else {
        let o: unsafe extern "win64" fn() -> *mut c_void =
            std::mem::transmute(T_STM_GET_RESOLVER.load(Ordering::Relaxed));
        o()
    }
}

static T_STM_SET_RESOLVER: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_set_file_location_resolver(r: *mut c_void) {
    if native_mode() {
        FILE_LOCATION_RESOLVER.store(r as usize, Ordering::Release);
    } else {
        let o: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(T_STM_SET_RESOLVER.load(Ordering::Relaxed));
        o(r)
    }
}

static T_STM_SET_LANG: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_set_current_language(lang: *const u16) -> i32 {
    if native_mode() {
        let w = wslice(lang);
        let mut g = CURRENT_LANG_BUF.lock().unwrap();
        g.fill(0);
        let copy_len = w.len().min(63);
        g[..copy_len].copy_from_slice(&w[..copy_len]);
        drop(g);
        let s = String::from_utf16_lossy(&w);
        native_engine().set_current_language(&s)
    } else {
        let o: unsafe extern "win64" fn(*const u16) -> i32 =
            std::mem::transmute(T_STM_SET_LANG.load(Ordering::Relaxed));
        o(lang)
    }
}

static T_STM_GET_LANG: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_get_current_language() -> *const u16 {
    if native_mode() {
        let g = CURRENT_LANG_BUF.lock().unwrap();
        g.as_ptr()
    } else {
        let o: unsafe extern "win64" fn() -> *const u16 =
            std::mem::transmute(T_STM_GET_LANG.load(Ordering::Relaxed));
        o()
    }
}

static T_STM_ADD_LANG_OBS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_add_lang_observer(
    cb: *const c_void,
    cookie: *mut c_void,
) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*const c_void, *mut c_void) -> i32 =
            std::mem::transmute(T_STM_ADD_LANG_OBS.load(Ordering::Relaxed));
        o(cb, cookie)
    }
}

static T_STM_REM_LANG_OBS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_remove_lang_observer(cookie: *mut c_void) {
    if !native_mode() {
        let o: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(T_STM_REM_LANG_OBS.load(Ordering::Relaxed));
        o(cookie)
    }
}

static T_STM_FLUSH_CACHES: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn stream_mgr_flush_all_caches() {
    if !native_mode() {
        let o: unsafe extern "win64" fn() =
            std::mem::transmute(T_STM_FLUSH_CACHES.load(Ordering::Relaxed));
        o()
    }
}

// --- Default Init Settings & MusicEngine::GetPlayingSegmentInfo ---

static T_SE_GET_DEF_INIT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn se_get_default_init_settings(s: *mut u8) {
    if native_mode() {
        if !s.is_null() {
            std::ptr::write_bytes(s, 0, aktrace::INIT_SETTINGS_LEN);
            (s.add(0x08) as *mut u32).write_unaligned(0xff);
            (s.add(0x0c) as *mut u32).write_unaligned(0x0004_0000);
            (s.add(0x14) as *mut u32).write_unaligned(1);
            (s.add(0x18) as *mut u32).write_unaligned(0x200);
            (s.add(0x1c) as *mut u32).write_unaligned(0x0010_0000);
            (s.add(0x20) as *mut u32).write_unaligned(0x0080_0000);
            (s.add(0x60) as *mut u32).write_unaligned(1000);
            (s.add(0x64) as *mut u16).write_unaligned(0x0101);
            (s.add(0x84) as *mut f32).write_unaligned(1.0);
            (s.add(0x88) as *mut u32).write_unaligned(0x8000);
            (s.add(0x8c) as *mut f32).write_unaligned(16.0);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u8) =
            std::mem::transmute(T_SE_GET_DEF_INIT.load(Ordering::Relaxed));
        o(s)
    }
}

static T_SE_GET_DEF_PLAT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn se_get_default_platform_init_settings(s: *mut u8) {
    if native_mode() {
        if !s.is_null() {
            std::ptr::write_bytes(s, 0, aktrace::PLATFORM_SETTINGS_LEN);
            (s.add(0x28) as *mut u32).write_unaligned(0x0002_0000);
            (s.add(0x38) as *mut u16).write_unaligned(4);
            (s.add(0x3c) as *mut u32).write_unaligned(48_000);
            (s.add(0x40) as *mut u8).write_unaligned(1);
            (s.add(0x44) as *mut u32).write_unaligned(0x80);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u8) =
            std::mem::transmute(T_SE_GET_DEF_PLAT.load(Ordering::Relaxed));
        o(s)
    }
}

static T_MUSIC_GET_DEF_INIT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn music_get_default_init_settings(s: *mut f32) {
    if native_mode() {
        if !s.is_null() {
            s.write_unaligned(1.0);
        }
    } else {
        let o: unsafe extern "win64" fn(*mut f32) =
            std::mem::transmute(T_MUSIC_GET_DEF_INIT.load(Ordering::Relaxed));
        o(s)
    }
}

static T_MUSIC_GET_SEG_INFO: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn music_get_playing_segment_info(
    pid: u32,
    out_info: *mut wwise_audio::runtime::AkSegmentInfo,
    extrap: bool,
) -> i32 {
    if native_mode() {
        if out_info.is_null() {
            return wwise_audio::runtime::AK_INVALID_PARAMETER;
        }
        match native_engine().get_playing_segment_info(pid) {
            Some(info) => {
                out_info.write_unaligned(info);
                wwise_audio::runtime::AK_SUCCESS
            }
            None => wwise_audio::runtime::AK_FAIL,
        }
    } else {
        let o: unsafe extern "win64" fn(u32, *mut wwise_audio::runtime::AkSegmentInfo, bool) -> i32 =
            std::mem::transmute(T_MUSIC_GET_SEG_INFO.load(Ordering::Relaxed));
        o(pid, out_info, extrap)
    }
}

// --- SoundEngine Settings, Callbacks, Banks & Device Queries ---

static T_GET_AUDIO_SETTINGS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_audio_settings(out: *mut u32) -> i32 {
    if native_mode() {
        if !out.is_null() {
            out.write_unaligned(512);
            out.add(1).write_unaligned(48_000);
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*mut u32) -> i32 =
            std::mem::transmute(T_GET_AUDIO_SETTINGS.load(Ordering::Relaxed));
        o(out)
    }
}

static T_GET_SAMPLE_RATE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_sample_rate() -> u32 {
    if native_mode() {
        48_000
    } else {
        let o: unsafe extern "win64" fn() -> u32 =
            std::mem::transmute(T_GET_SAMPLE_RATE.load(Ordering::Relaxed));
        o()
    }
}

static T_GET_SAMPLE_TICK: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_sample_tick() -> u64 {
    if native_mode() {
        native_engine().sample_tick()
    } else {
        let o: unsafe extern "win64" fn() -> u64 =
            std::mem::transmute(T_GET_SAMPLE_TICK.load(Ordering::Relaxed));
        o()
    }
}

static T_GET_BUFFER_TICK: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_buffer_tick() -> u32 {
    if native_mode() {
        native_engine().frames_rendered()
    } else {
        let o: unsafe extern "win64" fn() -> u32 =
            std::mem::transmute(T_GET_BUFFER_TICK.load(Ordering::Relaxed));
        o()
    }
}

static T_GET_SPEAKER_CFG: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_speaker_configuration(out_cfg: *mut u32, dev: u64) -> *mut u32 {
    if native_mode() {
        if !out_cfg.is_null() {
            out_cfg.write_unaligned(0x0000_3102); // stereo
        }
        out_cfg
    } else {
        let o: unsafe extern "win64" fn(*mut u32, u64) -> *mut u32 =
            std::mem::transmute(T_GET_SPEAKER_CFG.load(Ordering::Relaxed));
        o(out_cfg, dev)
    }
}

static T_GET_OUT_DEV_CFG: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_output_device_configuration(
    dev: u64,
    out_cfg: *mut u32,
    out_caps: *mut u8,
) -> i32 {
    if native_mode() {
        if !out_cfg.is_null() {
            out_cfg.write_unaligned(0x0000_3102);
        }
        if !out_caps.is_null() {
            std::ptr::write_bytes(out_caps, 0, 16);
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u64, *mut u32, *mut u8) -> i32 =
            std::mem::transmute(T_GET_OUT_DEV_CFG.load(Ordering::Relaxed));
        o(dev, out_cfg, out_caps)
    }
}

static T_GET_PANNING_RULE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_panning_rule(out_rule: *mut i32, dev: u64) -> i32 {
    if native_mode() {
        if !out_rule.is_null() {
            out_rule.write_unaligned(native_engine().panning_rule(dev));
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*mut i32, u64) -> i32 =
            std::mem::transmute(T_GET_PANNING_RULE.load(Ordering::Relaxed));
        o(out_rule, dev)
    }
}

static T_SET_PANNING_RULE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_panning_rule(rule: i32, dev: u64) -> i32 {
    if native_mode() {
        native_engine().set_panning_rule(rule, dev)
    } else {
        let o: unsafe extern "win64" fn(i32, u64) -> i32 =
            std::mem::transmute(T_SET_PANNING_RULE.load(Ordering::Relaxed));
        o(rule, dev)
    }
}

static T_SET_OUT_VOL: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_output_volume(dev: u64, vol: f32) -> i32 {
    if native_mode() {
        native_engine().set_output_volume(dev, vol)
    } else {
        let o: unsafe extern "win64" fn(u64, f32) -> i32 =
            std::mem::transmute(T_SET_OUT_VOL.load(Ordering::Relaxed));
        o(dev, vol)
    }
}

static T_SET_VOL_THRESH: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_volume_threshold(thresh: f32) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(f32) -> i32 =
            std::mem::transmute(T_SET_VOL_THRESH.load(Ordering::Relaxed));
        o(thresh)
    }
}

static T_SET_MAX_VOICES: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_max_num_voices_limit(max_voices: u16) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u16) -> i32 =
            std::mem::transmute(T_SET_MAX_VOICES.load(Ordering::Relaxed));
        o(max_voices)
    }
}

static T_SET_RNG_SEED: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_random_seed(seed: u32) {
    if native_mode() {
        native_engine().set_random_seed(seed);
    } else {
        let o: unsafe extern "win64" fn(u32) =
            std::mem::transmute(T_SET_RNG_SEED.load(Ordering::Relaxed));
        o(seed)
    }
}

static T_SUSPEND: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn se_suspend(render_anyway: bool, fade_out: bool) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(bool, bool) -> i32 =
            std::mem::transmute(T_SUSPEND.load(Ordering::Relaxed));
        o(render_anyway, fade_out)
    }
}

static T_WAKEUP: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn se_wakeup_from_suspend(delay_ms: u32) -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32) -> i32 =
            std::mem::transmute(T_WAKEUP.load(Ordering::Relaxed));
        o(delay_ms)
    }
}

static T_CANCEL_EV_CB: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn cancel_event_callback(pid: u32) {
    if native_mode() {
        native_engine().cancel_event_callback(pid);
    } else {
        let o: unsafe extern "win64" fn(u32) =
            std::mem::transmute(T_CANCEL_EV_CB.load(Ordering::Relaxed));
        o(pid)
    }
}

static T_CANCEL_EV_COOKIE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn cancel_event_callback_cookie(cookie: *mut c_void) {
    if native_mode() {
        native_engine().cancel_event_callback_cookie(cookie as u64);
    } else {
        let o: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(T_CANCEL_EV_COOKIE.load(Ordering::Relaxed));
        o(cookie)
    }
}

static T_CANCEL_EV_OBJ: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn cancel_event_callback_game_object(obj: u64) {
    if native_mode() {
        native_engine().cancel_event_callback_game_object(obj);
    } else {
        let o: unsafe extern "win64" fn(u64) =
            std::mem::transmute(T_CANCEL_EV_OBJ.load(Ordering::Relaxed));
        o(obj)
    }
}

static T_CANCEL_BANK_COOKIE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn cancel_bank_callback_cookie(cookie: *mut c_void) {
    if !native_mode() {
        let o: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(T_CANCEL_BANK_COOKIE.load(Ordering::Relaxed));
        o(cookie)
    }
}

static T_UNREG_GLOBAL_CB: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn unregister_global_cb(f: GlobalCallback, loc: u32) -> i32 {
    if native_mode() {
        let cb: Win64GlobalCallback = std::mem::transmute(f as usize);
        native_engine().unregister_global_callback_win64(cb, loc)
    } else {
        let o: unsafe extern "win64" fn(GlobalCallback, u32) -> i32 =
            std::mem::transmute(T_UNREG_GLOBAL_CB.load(Ordering::Relaxed));
        o(f, loc)
    }
}

static T_CLEAR_BANKS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn clear_banks() -> i32 {
    if native_mode() {
        native_engine().clear_banks()
    } else {
        let o: unsafe extern "win64" fn() -> i32 =
            std::mem::transmute(T_CLEAR_BANKS.load(Ordering::Relaxed));
        o()
    }
}

static T_CLEAR_PREPARED: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn clear_prepared_events() -> i32 {
    if native_mode() {
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn() -> i32 =
            std::mem::transmute(T_CLEAR_PREPARED.load(Ordering::Relaxed));
        o()
    }
}

static T_GET_DEV_LIST1: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_device_list_1(
    shareset: u32,
    io_max: *mut u32,
    out_descs: *mut u8,
) -> i32 {
    if native_mode() {
        if !io_max.is_null() {
            io_max.write_unaligned(0);
        }
        let _ = (shareset, out_descs);
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32, *mut u32, *mut u8) -> i32 =
            std::mem::transmute(T_GET_DEV_LIST1.load(Ordering::Relaxed));
        o(shareset, io_max, out_descs)
    }
}

static T_GET_DEV_LIST2: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_device_list_2(
    company: u32,
    plugin: u32,
    io_max: *mut u32,
    out_descs: *mut u8,
) -> i32 {
    if native_mode() {
        if !io_max.is_null() {
            io_max.write_unaligned(0);
        }
        let _ = (company, plugin, out_descs);
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32, u32, *mut u32, *mut u8) -> i32 =
            std::mem::transmute(T_GET_DEV_LIST2.load(Ordering::Relaxed));
        o(company, plugin, io_max, out_descs)
    }
}

static T_GET_DEV_SPATIAL: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_device_spatial_audio_support(dev_id: u32) -> i32 {
    if native_mode() {
        let _ = dev_id;
        wwise_audio::runtime::AK_FAIL
    } else {
        let o: unsafe extern "win64" fn(u32) -> i32 =
            std::mem::transmute(T_GET_DEV_SPATIAL.load(Ordering::Relaxed));
        o(dev_id)
    }
}

static T_GET_DEVICE_ID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn ak_get_device_id(dev: *mut c_void) -> u32 {
    if native_mode() {
        if dev.is_null() {
            return 0xffff_ffff;
        }
        let vtbl = *(dev as *const *const usize);
        if vtbl.is_null() {
            return 0;
        }
        let get_id: unsafe extern "win64" fn(*mut c_void, *mut *mut u16) -> i32 =
            std::mem::transmute(*vtbl.add(5));
        let mut pwstr: *mut u16 = std::ptr::null_mut();
        if get_id(dev, &mut pwstr) >= 0 && !pwstr.is_null() {
            let s = String::from_utf16_lossy(&wslice(pwstr));
            crate::heap::free(pwstr as *mut c_void);
            wwise_audio::fnv1_32_lower(&s)
        } else {
            0
        }
    } else {
        let o: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(T_GET_DEVICE_ID.load(Ordering::Relaxed));
        o(dev)
    }
}

static T_GET_DEVICE_ID_NAME: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn ak_get_device_id_from_name(name: *mut u16) -> u32 {
    if native_mode() {
        if name.is_null() {
            0
        } else {
            wwise_audio::fnv1_32_lower(&String::from_utf16_lossy(&wslice(name)))
        }
    } else {
        let o: unsafe extern "win64" fn(*mut u16) -> u32 =
            std::mem::transmute(T_GET_DEVICE_ID_NAME.load(Ordering::Relaxed));
        o(name)
    }
}

static T_GET_WIN_DEV: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn ak_get_windows_device(
    index: i32,
    out_id: *mut u32,
    out_dev: *mut *mut c_void,
    state_mask: u32,
) -> bool {
    if native_mode() {
        if !out_id.is_null() {
            out_id.write_unaligned(0);
        }
        if !out_dev.is_null() {
            out_dev.write_unaligned(std::ptr::null_mut());
        }
        let _ = (index, state_mask);
        true
    } else {
        let o: unsafe extern "win64" fn(i32, *mut u32, *mut *mut c_void, u32) -> bool =
            std::mem::transmute(T_GET_WIN_DEV.load(Ordering::Relaxed));
        o(index, out_id, out_dev, state_mask)
    }
}

static T_GET_WIN_DEV_COUNT: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn ak_get_windows_device_count(state_mask: u32) -> u32 {
    if native_mode() {
        let _ = state_mask;
        1
    } else {
        let o: unsafe extern "win64" fn(u32) -> u32 =
            std::mem::transmute(T_GET_WIN_DEV_COUNT.load(Ordering::Relaxed));
        o(state_mask)
    }
}

static DEFAULT_DEV_NAME_W: [u16; 18] = [
    b'C' as u16, b'o' as u16, b'r' as u16, b'e' as u16, b'A' as u16, b'u' as u16, b'd' as u16,
    b'i' as u16, b'o' as u16, b' ' as u16, b'D' as u16, b'e' as u16, b'f' as u16, b'a' as u16,
    b'u' as u16, b'l' as u16, b't' as u16, 0,
];

static T_GET_WIN_DEV_NAME: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn ak_get_windows_device_name(
    index: i32,
    out_id: *mut u32,
    state_mask: u32,
) -> *const u16 {
    if native_mode() {
        if !out_id.is_null() {
            out_id.write_unaligned(0);
        }
        let _ = (index, state_mask);
        DEFAULT_DEV_NAME_W.as_ptr()
    } else {
        let o: unsafe extern "win64" fn(i32, *mut u32, u32) -> *const u16 =
            std::mem::transmute(T_GET_WIN_DEV_NAME.load(Ordering::Relaxed));
        o(index, out_id, state_mask)
    }
}

static T_GET_OUTPUT_ID_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn get_output_id_w(name: *const u16, dev: u32) -> u64 {
    if native_mode() {
        let shareset = if name.is_null() {
            0
        } else {
            wwise_audio::fnv1_32_lower(&String::from_utf16_lossy(&wslice(name)))
        };
        ((dev as u64) << 32) | (shareset as u64)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u32) -> u64 =
            std::mem::transmute(T_GET_OUTPUT_ID_W.load(Ordering::Relaxed));
        o(name, dev)
    }
}

// --- Additional Query::* Exports ---

static T_QUERY_ACTIVE_OBJS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_active_game_objects(ak_array: *mut u8) -> i32 {
    if native_mode() {
        if !ak_array.is_null() {
            let items = (ak_array as *const *mut u64).read_unaligned();
            let len_ptr = ak_array.add(8) as *mut u32;
            let cap = (ak_array.add(12) as *const u32).read_unaligned() as usize;
            let cur_len = len_ptr.read_unaligned() as usize;
            if !items.is_null() && cap > cur_len {
                let objs = native_engine().active_game_objects();
                let mut written = cur_len;
                for obj in objs {
                    if written >= cap {
                        break;
                    }
                    items.add(written).write_unaligned(obj);
                    written += 1;
                }
                len_ptr.write_unaligned(written as u32);
            }
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(*mut u8) -> i32 =
            std::mem::transmute(T_QUERY_ACTIVE_OBJS.load(Ordering::Relaxed));
        o(ak_array)
    }
}

static T_QUERY_GET_RTPC_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_rtpc_a(
    name: *const u8,
    obj: u64,
    pid: u32,
    out_val: *mut f32,
    io_type: *mut i32,
) -> i32 {
    if native_mode() {
        let id = get_id_from_string_a(name);
        query_get_rtpc(id, obj, pid, out_val, io_type)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64, u32, *mut f32, *mut i32) -> i32 =
            std::mem::transmute(T_QUERY_GET_RTPC_A.load(Ordering::Relaxed));
        o(name, obj, pid, out_val, io_type)
    }
}

static T_QUERY_GET_RTPC_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_rtpc_w(
    name: *const u16,
    obj: u64,
    pid: u32,
    out_val: *mut f32,
    io_type: *mut i32,
) -> i32 {
    if native_mode() {
        let id = get_id_from_string_w(name);
        query_get_rtpc(id, obj, pid, out_val, io_type)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64, u32, *mut f32, *mut i32) -> i32 =
            std::mem::transmute(T_QUERY_GET_RTPC_W.load(Ordering::Relaxed));
        o(name, obj, pid, out_val, io_type)
    }
}

static T_QUERY_GET_STATE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_state(group: u32, out_state: *mut u32) -> i32 {
    if native_mode() {
        if !out_state.is_null() {
            out_state.write_unaligned(native_engine().get_state(group));
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_STATE.load(Ordering::Relaxed));
        o(group, out_state)
    }
}

static T_QUERY_GET_STATE_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_state_a(group: *const u8, out_state: *mut u32) -> i32 {
    if native_mode() {
        query_get_state(get_id_from_string_a(group), out_state)
    } else {
        let o: unsafe extern "win64" fn(*const u8, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_STATE_A.load(Ordering::Relaxed));
        o(group, out_state)
    }
}

static T_QUERY_GET_STATE_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_state_w(group: *const u16, out_state: *mut u32) -> i32 {
    if native_mode() {
        query_get_state(get_id_from_string_w(group), out_state)
    } else {
        let o: unsafe extern "win64" fn(*const u16, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_STATE_W.load(Ordering::Relaxed));
        o(group, out_state)
    }
}

static T_QUERY_GET_SWITCH: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_switch(group: u32, obj: u64, out_switch: *mut u32) -> i32 {
    if native_mode() {
        if !out_switch.is_null() {
            out_switch.write_unaligned(native_engine().get_switch(group, obj));
        }
        wwise_audio::runtime::AK_SUCCESS
    } else {
        let o: unsafe extern "win64" fn(u32, u64, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_SWITCH.load(Ordering::Relaxed));
        o(group, obj, out_switch)
    }
}

static T_QUERY_GET_SWITCH_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_switch_a(
    group: *const u8,
    obj: u64,
    out_switch: *mut u32,
) -> i32 {
    if native_mode() {
        query_get_switch(get_id_from_string_a(group), obj, out_switch)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_SWITCH_A.load(Ordering::Relaxed));
        o(group, obj, out_switch)
    }
}

static T_QUERY_GET_SWITCH_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_switch_w(
    group: *const u16,
    obj: u64,
    out_switch: *mut u32,
) -> i32 {
    if native_mode() {
        query_get_switch(get_id_from_string_w(group), obj, out_switch)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64, *mut u32) -> i32 =
            std::mem::transmute(T_QUERY_GET_SWITCH_W.load(Ordering::Relaxed));
        o(group, obj, out_switch)
    }
}

static T_QUERY_EV_FROM_PID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_event_id_from_playing_id(pid: u32) -> u32 {
    if native_mode() {
        native_engine().get_event_id_from_playing_id(pid)
    } else {
        let o: unsafe extern "win64" fn(u32) -> u32 =
            std::mem::transmute(T_QUERY_EV_FROM_PID.load(Ordering::Relaxed));
        o(pid)
    }
}

static T_QUERY_OBJ_FROM_PID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_game_object_from_playing_id(pid: u32) -> u64 {
    if native_mode() {
        native_engine().get_game_object_from_playing_id(pid)
    } else {
        let o: unsafe extern "win64" fn(u32) -> u64 =
            std::mem::transmute(T_QUERY_OBJ_FROM_PID.load(Ordering::Relaxed));
        o(pid)
    }
}

static T_QUERY_IS_OBJ_ACTIVE: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_is_game_object_active(obj: u64) -> bool {
    if native_mode() {
        native_engine().is_game_object_active(obj)
    } else {
        let o: unsafe extern "win64" fn(u64) -> bool =
            std::mem::transmute(T_QUERY_IS_OBJ_ACTIVE.load(Ordering::Relaxed));
        o(obj)
    }
}

static T_QUERY_LISTENER_POS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn query_get_listener_position(listener: u64, out_pos: *mut u8) -> i32 {
    if native_mode() {
        query_get_position(listener, out_pos)
    } else {
        let o: unsafe extern "win64" fn(u64, *mut u8) -> i32 =
            std::mem::transmute(T_QUERY_LISTENER_POS.load(Ordering::Relaxed));
        o(listener, out_pos)
    }
}

// --- String / Wchar Overloads of SoundEngine Commands & Bank Loaders ---

static T_POST_EVENT_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn post_event_a(
    name: *const u8,
    obj: u64,
    flags: u32,
    cb: *const c_void,
    cookie: *mut c_void,
    n: u32,
    ext: *const u8,
    pid: u32,
) -> u32 {
    if native_mode() {
        post_event(get_id_from_string_a(name), obj, flags, cb, cookie, n, ext, pid)
    } else {
        let o: unsafe extern "win64" fn(
            *const u8,
            u64,
            u32,
            *const c_void,
            *mut c_void,
            u32,
            *const u8,
            u32,
        ) -> u32 = std::mem::transmute(T_POST_EVENT_A.load(Ordering::Relaxed));
        o(name, obj, flags, cb, cookie, n, ext, pid)
    }
}

static T_POST_EVENT_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn post_event_w(
    name: *const u16,
    obj: u64,
    flags: u32,
    cb: *const c_void,
    cookie: *mut c_void,
    n: u32,
    ext: *const u8,
    pid: u32,
) -> u32 {
    if native_mode() {
        post_event(get_id_from_string_w(name), obj, flags, cb, cookie, n, ext, pid)
    } else {
        let o: unsafe extern "win64" fn(
            *const u16,
            u64,
            u32,
            *const c_void,
            *mut c_void,
            u32,
            *const u8,
            u32,
        ) -> u32 = std::mem::transmute(T_POST_EVENT_W.load(Ordering::Relaxed));
        o(name, obj, flags, cb, cookie, n, ext, pid)
    }
}

static T_SET_RTPC_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_rtpc_a(
    name: *const u8,
    v: f32,
    obj: u64,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        set_rtpc(get_id_from_string_a(name), v, obj, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u8, f32, u64, i32, i32, bool) -> i32 =
            std::mem::transmute(T_SET_RTPC_A.load(Ordering::Relaxed));
        o(name, v, obj, ms, curve, bypass)
    }
}

static T_SET_RTPC_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_rtpc_w(
    name: *const u16,
    v: f32,
    obj: u64,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        set_rtpc(get_id_from_string_w(name), v, obj, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u16, f32, u64, i32, i32, bool) -> i32 =
            std::mem::transmute(T_SET_RTPC_W.load(Ordering::Relaxed));
        o(name, v, obj, ms, curve, bypass)
    }
}

static T_SET_RTPC_PID_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_rtpc_pid_a(
    name: *const u8,
    v: f32,
    pid: u32,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        set_rtpc_pid(get_id_from_string_a(name), v, pid, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u8, f32, u32, i32, i32, bool) -> i32 =
            std::mem::transmute(T_SET_RTPC_PID_A.load(Ordering::Relaxed));
        o(name, v, pid, ms, curve, bypass)
    }
}

static T_SET_RTPC_PID_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_rtpc_pid_w(
    name: *const u16,
    v: f32,
    pid: u32,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        set_rtpc_pid(get_id_from_string_w(name), v, pid, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u16, f32, u32, i32, i32, bool) -> i32 =
            std::mem::transmute(T_SET_RTPC_PID_W.load(Ordering::Relaxed));
        o(name, v, pid, ms, curve, bypass)
    }
}

static T_RESET_RTPC_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn reset_rtpc_a(
    name: *const u8,
    obj: u64,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        reset_rtpc(get_id_from_string_a(name), obj, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64, i32, i32, bool) -> i32 =
            std::mem::transmute(T_RESET_RTPC_A.load(Ordering::Relaxed));
        o(name, obj, ms, curve, bypass)
    }
}

static T_RESET_RTPC_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn reset_rtpc_w(
    name: *const u16,
    obj: u64,
    ms: i32,
    curve: i32,
    bypass: bool,
) -> i32 {
    if native_mode() {
        reset_rtpc(get_id_from_string_w(name), obj, ms, curve, bypass)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64, i32, i32, bool) -> i32 =
            std::mem::transmute(T_RESET_RTPC_W.load(Ordering::Relaxed));
        o(name, obj, ms, curve, bypass)
    }
}

static T_SET_SWITCH_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_switch_a(group: *const u8, state: *const u8, obj: u64) -> i32 {
    if native_mode() {
        set_switch(get_id_from_string_a(group), get_id_from_string_a(state), obj)
    } else {
        let o: unsafe extern "win64" fn(*const u8, *const u8, u64) -> i32 =
            std::mem::transmute(T_SET_SWITCH_A.load(Ordering::Relaxed));
        o(group, state, obj)
    }
}

static T_SET_SWITCH_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_switch_w(group: *const u16, state: *const u16, obj: u64) -> i32 {
    if native_mode() {
        set_switch(get_id_from_string_w(group), get_id_from_string_w(state), obj)
    } else {
        let o: unsafe extern "win64" fn(*const u16, *const u16, u64) -> i32 =
            std::mem::transmute(T_SET_SWITCH_W.load(Ordering::Relaxed));
        o(group, state, obj)
    }
}

static T_SET_STATE_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_state_a(group: *const u8, state: *const u8) -> i32 {
    if native_mode() {
        set_state(get_id_from_string_a(group), get_id_from_string_a(state))
    } else {
        let o: unsafe extern "win64" fn(*const u8, *const u8) -> i32 =
            std::mem::transmute(T_SET_STATE_A.load(Ordering::Relaxed));
        o(group, state)
    }
}

static T_SET_STATE_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn set_state_w(group: *const u16, state: *const u16) -> i32 {
    if native_mode() {
        set_state(get_id_from_string_w(group), get_id_from_string_w(state))
    } else {
        let o: unsafe extern "win64" fn(*const u16, *const u16) -> i32 =
            std::mem::transmute(T_SET_STATE_W.load(Ordering::Relaxed));
        o(group, state)
    }
}

static T_POST_TRIGGER_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn post_trigger_a(trigger: *const u8, obj: u64) -> i32 {
    if native_mode() {
        post_trigger(get_id_from_string_a(trigger), obj)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64) -> i32 =
            std::mem::transmute(T_POST_TRIGGER_A.load(Ordering::Relaxed));
        o(trigger, obj)
    }
}

static T_POST_TRIGGER_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn post_trigger_w(trigger: *const u16, obj: u64) -> i32 {
    if native_mode() {
        post_trigger(get_id_from_string_w(trigger), obj)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64) -> i32 =
            std::mem::transmute(T_POST_TRIGGER_W.load(Ordering::Relaxed));
        o(trigger, obj)
    }
}

static T_EXEC_ACTION_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn exec_action_w(
    name: *const u16,
    action: i32,
    obj: u64,
    ms: i32,
    curve: i32,
    pid: u32,
) -> i32 {
    if native_mode() {
        exec_action(get_id_from_string_w(name), action, obj, ms, curve, pid)
    } else {
        let o: unsafe extern "win64" fn(*const u16, i32, u64, i32, i32, u32) -> i32 =
            std::mem::transmute(T_EXEC_ACTION_W.load(Ordering::Relaxed));
        o(name, action, obj, ms, curve, pid)
    }
}

static T_EXEC_ACTION_PID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn exec_action_on_playing_id(
    action: i32,
    pid: u32,
    ms: i32,
    curve: i32,
) {
    if native_mode() {
        let _ = native_engine().execute_action_on_event(0, action, 0, ms, curve, pid);
    } else {
        let o: unsafe extern "win64" fn(i32, u32, i32, i32) =
            std::mem::transmute(T_EXEC_ACTION_PID.load(Ordering::Relaxed));
        o(action, pid, ms, curve)
    }
}

static T_SEEK_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn seek_on_event_a(
    name: *const u8,
    obj: u64,
    ms: i32,
    nearest: bool,
    pid: u32,
) -> i32 {
    if native_mode() {
        seek_on_event(get_id_from_string_a(name), obj, ms, nearest, pid)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64, i32, bool, u32) -> i32 =
            std::mem::transmute(T_SEEK_A.load(Ordering::Relaxed));
        o(name, obj, ms, nearest, pid)
    }
}

static T_SEEK_PCT_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn seek_on_event_pct_a(
    name: *const u8,
    obj: u64,
    pct: f32,
    nearest: bool,
    pid: u32,
) -> i32 {
    if native_mode() {
        seek_on_event_pct(get_id_from_string_a(name), obj, pct, nearest, pid)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u64, f32, bool, u32) -> i32 =
            std::mem::transmute(T_SEEK_PCT_A.load(Ordering::Relaxed));
        o(name, obj, pct, nearest, pid)
    }
}

static T_SEEK_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn seek_on_event_w(
    name: *const u16,
    obj: u64,
    ms: i32,
    nearest: bool,
    pid: u32,
) -> i32 {
    if native_mode() {
        seek_on_event(get_id_from_string_w(name), obj, ms, nearest, pid)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64, i32, bool, u32) -> i32 =
            std::mem::transmute(T_SEEK_W.load(Ordering::Relaxed));
        o(name, obj, ms, nearest, pid)
    }
}

static T_SEEK_PCT_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn seek_on_event_pct_w(
    name: *const u16,
    obj: u64,
    pct: f32,
    nearest: bool,
    pid: u32,
) -> i32 {
    if native_mode() {
        seek_on_event_pct(get_id_from_string_w(name), obj, pct, nearest, pid)
    } else {
        let o: unsafe extern "win64" fn(*const u16, u64, f32, bool, u32) -> i32 =
            std::mem::transmute(T_SEEK_PCT_W.load(Ordering::Relaxed));
        o(name, obj, pct, nearest, pid)
    }
}

static T_PIN_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn pin_event_a(name: *const u8, prio: i8, active: i8) -> i32 {
    if native_mode() {
        pin_event(get_id_from_string_a(name), prio, active)
    } else {
        let o: unsafe extern "win64" fn(*const u8, i8, i8) -> i32 =
            std::mem::transmute(T_PIN_A.load(Ordering::Relaxed));
        o(name, prio, active)
    }
}

static T_PIN_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn pin_event_w(name: *const u16, prio: i8, active: i8) -> i32 {
    if native_mode() {
        pin_event(get_id_from_string_w(name), prio, active)
    } else {
        let o: unsafe extern "win64" fn(*const u16, i8, i8) -> i32 =
            std::mem::transmute(T_PIN_W.load(Ordering::Relaxed));
        o(name, prio, active)
    }
}

static T_UNPIN_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn unpin_event_a(name: *const u8) -> i32 {
    if native_mode() {
        unpin_event(get_id_from_string_a(name))
    } else {
        let o: unsafe extern "win64" fn(*const u8) -> i32 =
            std::mem::transmute(T_UNPIN_A.load(Ordering::Relaxed));
        o(name)
    }
}

static T_UNPIN_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn unpin_event_w(name: *const u16) -> i32 {
    if native_mode() {
        unpin_event(get_id_from_string_w(name))
    } else {
        let o: unsafe extern "win64" fn(*const u16) -> i32 =
            std::mem::transmute(T_UNPIN_W.load(Ordering::Relaxed));
        o(name)
    }
}

static T_LOAD_BANK_ID: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn load_bank_id(id: u32, bank_type: u32) -> i32 {
    if native_mode() {
        let _ = bank_type;
        match native_engine().load_bank_name(&id.to_string()) {
            Ok(_) => wwise_audio::runtime::AK_SUCCESS,
            Err(_) => wwise_audio::runtime::AK_FAIL,
        }
    } else {
        let o: unsafe extern "win64" fn(u32, u32) -> i32 =
            std::mem::transmute(T_LOAD_BANK_ID.load(Ordering::Relaxed));
        o(id, bank_type)
    }
}

static T_LOAD_BANK_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn load_bank_a(name: *const u8, out_id: *mut u32, bank_type: u32) -> i32 {
    if native_mode() {
        let _ = bank_type;
        let s = String::from_utf8_lossy(&cstr(name)).into_owned();
        match native_engine().load_bank_name(&s) {
            Ok(id) => {
                if !out_id.is_null() {
                    out_id.write_unaligned(id);
                }
                wwise_audio::runtime::AK_SUCCESS
            }
            Err(_) => wwise_audio::runtime::AK_FAIL,
        }
    } else {
        let o: unsafe extern "win64" fn(*const u8, *mut u32, u32) -> i32 =
            std::mem::transmute(T_LOAD_BANK_A.load(Ordering::Relaxed));
        o(name, out_id, bank_type)
    }
}

static T_LOAD_BANK_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn load_bank_w(name: *const u16, out_id: *mut u32, bank_type: u32) -> i32 {
    if native_mode() {
        let _ = bank_type;
        let s = String::from_utf16_lossy(&wslice(name));
        match native_engine().load_bank_name(&s) {
            Ok(id) => {
                if !out_id.is_null() {
                    out_id.write_unaligned(id);
                }
                wwise_audio::runtime::AK_SUCCESS
            }
            Err(_) => wwise_audio::runtime::AK_FAIL,
        }
    } else {
        let o: unsafe extern "win64" fn(*const u16, *mut u32, u32) -> i32 =
            std::mem::transmute(T_LOAD_BANK_W.load(Ordering::Relaxed));
        o(name, out_id, bank_type)
    }
}

static T_LOAD_COPY_EX: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn load_bank_copy_ex(
    p: *const u8,
    size: u32,
    out_id: *mut u32,
    out_type: *mut u32,
) -> i32 {
    if native_mode() {
        if !out_type.is_null() {
            out_type.write_unaligned(0);
        }
        load_bank_copy(p, size, out_id)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u32, *mut u32, *mut u32) -> i32 =
            std::mem::transmute(T_LOAD_COPY_EX.load(Ordering::Relaxed));
        o(p, size, out_id, out_type)
    }
}

static T_LOAD_VIEW_EX: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn load_bank_view_ex(
    p: *const u8,
    size: u32,
    out_id: *mut u32,
    out_type: *mut u32,
) -> i32 {
    if native_mode() {
        if !out_type.is_null() {
            out_type.write_unaligned(0);
        }
        load_bank_view(p, size, out_id)
    } else {
        let o: unsafe extern "win64" fn(*const u8, u32, *mut u32, *mut u32) -> i32 =
            std::mem::transmute(T_LOAD_VIEW_EX.load(Ordering::Relaxed));
        o(p, size, out_id, out_type)
    }
}

static T_UNLOAD_A: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn unload_bank_a(name: *const u8, p: *const u8, ty: u32) -> i32 {
    if native_mode() {
        unload_bank(get_id_from_string_a(name), p, ty)
    } else {
        let o: unsafe extern "win64" fn(*const u8, *const u8, u32) -> i32 =
            std::mem::transmute(T_UNLOAD_A.load(Ordering::Relaxed));
        o(name, p, ty)
    }
}

static T_UNLOAD_W: AtomicUsize = AtomicUsize::new(0);
unsafe extern "win64" fn unload_bank_w(name: *const u16, p: *const u8, ty: u32) -> i32 {
    if native_mode() {
        unload_bank(get_id_from_string_w(name), p, ty)
    } else {
        let o: unsafe extern "win64" fn(*const u16, *const u8, u32) -> i32 =
            std::mem::transmute(T_UNLOAD_W.load(Ordering::Relaxed));
        o(name, p, ty)
    }
}

// Hooks whose by-value arguments mix integers and floats need an aarch64
// host-bridge entry (see `crate::bridge`); the table registers `bridge_entry!(f)`.
crate::win64_bridge!(fn set_scaling(u64, f32) -> i32);
crate::win64_bridge!(fn set_obs_occ(u64, u64, f32, f32) -> i32);
crate::win64_bridge!(fn set_out_bus_vol(u64, u64, f32) -> i32);
crate::win64_bridge!(fn set_rtpc(u32, f32, u64, i32, i32, bool) -> i32);
crate::win64_bridge!(fn set_rtpc_pid(u32, f32, u32, i32, i32, bool) -> i32);
crate::win64_bridge!(fn seek_on_event_pct(u32, u64, f32, bool, u32) -> i32);
crate::win64_bridge!(fn set_speaker_angles(*const f32, u32, f32, u64) -> i32);
crate::win64_bridge!(fn set_output_volume(u64, f32) -> i32);
crate::win64_bridge!(fn set_rtpc_a(*const u8, f32, u64, i32, i32, bool) -> i32);
crate::win64_bridge!(fn set_rtpc_w(*const u16, f32, u64, i32, i32, bool) -> i32);
crate::win64_bridge!(fn set_rtpc_pid_a(*const u8, f32, u32, i32, i32, bool) -> i32);
crate::win64_bridge!(fn set_rtpc_pid_w(*const u16, f32, u32, i32, i32, bool) -> i32);
crate::win64_bridge!(fn seek_on_event_pct_a(*const u8, u64, f32, bool, u32) -> i32);
crate::win64_bridge!(fn seek_on_event_pct_w(*const u16, u64, f32, bool, u32) -> i32);

struct Hook {
    mangled: &'static str,
    target: usize,
    slot: &'static AtomicUsize,
}

macro_rules! hook {
    ($slot:ident, $f:expr, $m:literal) => {
        Hook {
            mangled: $m,
            target: $f as *const () as usize,
            slot: &$slot,
        }
    };
}

fn table() -> Vec<Hook> {
    vec![
        hook!(T_INIT, se_init_hook, "?Init@SoundEngine@AK@@YA?AW4AKRESULT@@PEAUAkInitSettings@@PEAUAkPlatformInitSettings@@@Z"),
        hook!(T_TERM, se_term, "?Term@SoundEngine@AK@@YAXXZ"),
        hook!(T_MUSIC_INIT, music_init, "?Init@MusicEngine@AK@@YA?AW4AKRESULT@@PEAUAkMusicSettings@@@Z"),
        hook!(T_MUSIC_TERM, music_term, "?Term@MusicEngine@AK@@YAXXZ"),
        hook!(T_REGISTER_GLOBAL_CB, register_global_cb, "?RegisterGlobalCallback@SoundEngine@AK@@YA?AW4AKRESULT@@P6AXPEAVIAkGlobalPluginContext@2@W4AkGlobalCallbackLocation@@PEAX@ZI2W4AkPluginType@@II@Z"),
        hook!(T_REG_BUS_VOL_CB, reg_bus_vol_cb, "?RegisterBusVolumeCallback@SoundEngine@AK@@YA?AW4AKRESULT@@IP6AXPEAUAkSpeakerVolumeMatrixCallbackInfo@@@ZPEAX@Z"),
        hook!(T_RENDER, render_audio, "?RenderAudio@SoundEngine@AK@@YA?AW4AKRESULT@@_N@Z"),
        // RegisterGameObj(_K) and (_K, PEBD) are folded to one body (0x1415860a0).
        hook!(T_REG_OBJ, register_obj, "?RegisterGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEBD@Z"),
        hook!(T_UNREG_OBJ, unregister_obj, "?UnregisterGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@_K@Z"),
        hook!(T_UNREG_ALL, unregister_all, "?UnregisterAllGameObj@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
        hook!(T_SET_POS, set_position, "?SetPosition@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEBVAkWorldTransform@@W4AkSetPositionFlags@@@Z"),
        hook!(T_SET_MULTI_POS, set_multi_pos, "?SetMultiplePositions@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEBVAkWorldTransform@@GW4MultiPositionType@12@W4AkSetPositionFlags@@@Z"),
        hook!(T_SET_DEFAULT_LISTENERS, set_default_listeners, "?SetDefaultListeners@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_KI@Z"),
        hook!(T_SET_DIST_PROBE, set_distance_probe, "?SetDistanceProbe@SoundEngine@AK@@YA?AW4AKRESULT@@_K0@Z"),
        hook!(T_SET_SCALING, crate::bridge_entry!(set_scaling), "?SetScalingFactor@SoundEngine@AK@@YA?AW4AKRESULT@@_KM@Z"),
        hook!(T_SET_OBS_OCC, crate::bridge_entry!(set_obs_occ), "?SetObjectObstructionAndOcclusion@SoundEngine@AK@@YA?AW4AKRESULT@@_K0MM@Z"),
        hook!(T_SET_AUX, set_aux, "?SetGameObjectAuxSendValues@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEAUAkAuxSendValue@@I@Z"),
        hook!(T_SET_OUT_BUS_VOL, crate::bridge_entry!(set_out_bus_vol), "?SetGameObjectOutputBusVolume@SoundEngine@AK@@YA?AW4AKRESULT@@_K0M@Z"),
        hook!(T_SET_RTPC, crate::bridge_entry!(set_rtpc), "?SetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@IM_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_RTPC_PID, crate::bridge_entry!(set_rtpc_pid), "?SetRTPCValueByPlayingID@SoundEngine@AK@@YA?AW4AKRESULT@@IMIHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_RESET_RTPC, reset_rtpc, "?ResetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@I_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_SWITCH, set_switch, "?SetSwitch@SoundEngine@AK@@YA?AW4AKRESULT@@II_K@Z"),
        hook!(T_SET_STATE, set_state, "?SetState@SoundEngine@AK@@YA?AW4AKRESULT@@II@Z"),
        hook!(T_POST_TRIGGER, post_trigger, "?PostTrigger@SoundEngine@AK@@YA?AW4AKRESULT@@I_K@Z"),
        hook!(T_POST_EVENT, post_event, "?PostEvent@SoundEngine@AK@@YAII_KIP6AXW4AkCallbackType@@PEAUAkCallbackInfo@@@ZPEAXIPEAUAkExternalSourceInfo@@I@Z"),
        hook!(T_EXEC_ACTION, exec_action, "?ExecuteActionOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@IW4AkActionOnEventType@12@_KHW4AkCurveInterpolation@@I@Z"),
        hook!(T_EXEC_ACTION_NAME, exec_action_name, "?ExecuteActionOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDW4AkActionOnEventType@12@_KHW4AkCurveInterpolation@@I@Z"),
        hook!(T_SEEK, seek_on_event, "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@I_KH_NI@Z"),
        hook!(T_SEEK_PCT, crate::bridge_entry!(seek_on_event_pct), "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@I_KM_NI@Z"),
        hook!(T_STOP_ALL, stop_all, "?StopAll@SoundEngine@AK@@YAX_K@Z"),
        hook!(T_STOP_PID, stop_pid, "?StopPlayingID@SoundEngine@AK@@YAXIHW4AkCurveInterpolation@@@Z"),
        hook!(T_PIN, pin_event, "?PinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@ICC@Z"),
        hook!(T_UNPIN, unpin_event, "?UnpinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@I@Z"),
        hook!(T_LOAD_COPY, load_bank_copy, "?LoadBankMemoryCopy@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI@Z"),
        hook!(T_LOAD_VIEW, load_bank_view, "?LoadBankMemoryView@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI@Z"),
        hook!(T_UNLOAD, unload_bank, "?UnloadBank@SoundEngine@AK@@YA?AW4AKRESULT@@IPEBXI@Z"),
        hook!(T_ADD_OUTPUT, add_output, "?AddOutput@SoundEngine@AK@@YA?AW4AKRESULT@@AEBUAkOutputSettings@@PEA_KPEB_KI@Z"),
        hook!(T_REMOVE_OUTPUT, remove_output, "?RemoveOutput@SoundEngine@AK@@YA?AW4AKRESULT@@_K@Z"),
        hook!(T_SPEAKER_ANGLES, crate::bridge_entry!(set_speaker_angles), "?SetSpeakerAngles@SoundEngine@AK@@YA?AW4AKRESULT@@PEBMIM_K@Z"),
    ]
}

fn native_only_table() -> Vec<Hook> {
    vec![
        hook!(T_IS_INITIALIZED, se_is_initialized, "?IsInitialized@SoundEngine@AK@@YA_NXZ"),
        hook!(T_GET_ID_A, get_id_from_string_a, "?GetIDFromString@SoundEngine@AK@@YAIPEBD@Z"),
        hook!(T_GET_ID_W, get_id_from_string_w, "?GetIDFromString@SoundEngine@AK@@YAIPEB_W@Z"),
        hook!(T_GET_SOURCE_PLAY_POS, get_source_play_position, "?GetSourcePlayPosition@SoundEngine@AK@@YA?AW4AKRESULT@@IPEAH_N@Z"),
        hook!(T_GET_SPEAKER_ANGLES, get_speaker_angles, "?GetSpeakerAngles@SoundEngine@AK@@YA?AW4AKRESULT@@PEAMAEAIAEAM_K@Z"),
        hook!(T_GET_OUTPUT_ID, get_output_id, "?GetOutputID@SoundEngine@AK@@YA_KII@Z"),
        hook!(T_GET_OUTPUT_ID_NAME, get_output_id_name, "?GetOutputID@SoundEngine@AK@@YA_KPEBDI@Z"),
        hook!(T_GET_OUTPUT_ID_W, get_output_id_w, "?GetOutputID@SoundEngine@AK@@YA_KPEB_WI@Z"),
        hook!(T_QUERY_GET_RTPC, query_get_rtpc, "?GetRTPCValue@Query@SoundEngine@AK@@YA?AW4AKRESULT@@I_KIAEAMAEAW4RTPCValue_type@123@@Z"),
        hook!(T_QUERY_GET_RTPC_A, query_get_rtpc_a, "?GetRTPCValue@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_KIAEAMAEAW4RTPCValue_type@123@@Z"),
        hook!(T_QUERY_GET_RTPC_W, query_get_rtpc_w, "?GetRTPCValue@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_KIAEAMAEAW4RTPCValue_type@123@@Z"),
        hook!(T_QUERY_GET_POS, query_get_position, "?GetPosition@Query@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEAVAkWorldTransform@@@Z"),
        hook!(T_QUERY_LISTENER_POS, query_get_listener_position, "?GetListenerPosition@Query@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEAVAkWorldTransform@@@Z"),
        hook!(T_QUERY_GET_PIDS, query_get_playing_ids, "?GetPlayingIDsFromGameObject@Query@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEAIPEAI@Z"),
        hook!(T_QUERY_MAX_RADIUS, query_get_max_radius, "?GetMaxRadius@Query@SoundEngine@AK@@YAM_K@Z"),
        hook!(T_QUERY_ACTIVE_OBJS, query_get_active_game_objects, "?GetActiveGameObjects@Query@SoundEngine@AK@@YA?AW4AKRESULT@@AEAV?$AkArray@_K_KU?$AkArrayAllocatorNoAlign@$0A@@@UAkGrowByPolicy_Proportional@@U?$AkAssignmentMovePolicy@_K@@@@@Z"),
        hook!(T_QUERY_GET_STATE, query_get_state, "?GetState@Query@SoundEngine@AK@@YA?AW4AKRESULT@@IAEAI@Z"),
        hook!(T_QUERY_GET_STATE_A, query_get_state_a, "?GetState@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDAEAI@Z"),
        hook!(T_QUERY_GET_STATE_W, query_get_state_w, "?GetState@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WAEAI@Z"),
        hook!(T_QUERY_GET_SWITCH, query_get_switch, "?GetSwitch@Query@SoundEngine@AK@@YA?AW4AKRESULT@@I_KAEAI@Z"),
        hook!(T_QUERY_GET_SWITCH_A, query_get_switch_a, "?GetSwitch@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_KAEAI@Z"),
        hook!(T_QUERY_GET_SWITCH_W, query_get_switch_w, "?GetSwitch@Query@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_KAEAI@Z"),
        hook!(T_QUERY_EV_FROM_PID, query_get_event_id_from_playing_id, "?GetEventIDFromPlayingID@Query@SoundEngine@AK@@YAII@Z"),
        hook!(T_QUERY_OBJ_FROM_PID, query_get_game_object_from_playing_id, "?GetGameObjectFromPlayingID@Query@SoundEngine@AK@@YA_KI@Z"),
        hook!(T_QUERY_IS_OBJ_ACTIVE, query_get_is_game_object_active, "?GetIsGameObjectActive@Query@SoundEngine@AK@@YA_N_K@Z"),
        hook!(T_SET_LISTENERS, set_listeners, "?SetListeners@SoundEngine@AK@@YA?AW4AKRESULT@@_KPEB_KI@Z"),
        hook!(T_START_CAPTURE, start_output_capture, "?StartOutputCapture@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W@Z"),
        hook!(T_STOP_CAPTURE, stop_output_capture, "?StopOutputCapture@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
        // Milestone A9: MemoryMgr
        hook!(T_MEM_GET_DEF, mem_get_default_settings, "?GetDefaultSettings@MemoryMgr@AK@@YAXAEAUAkMemSettings@@@Z"),
        hook!(T_MEM_INIT, mem_init, "?Init@MemoryMgr@AK@@YA?AW4AKRESULT@@PEAUAkMemSettings@@@Z"),
        hook!(T_MEM_IS_INIT, mem_is_initialized, "?IsInitialized@MemoryMgr@AK@@YA_NXZ"),
        hook!(T_MEM_TERM, mem_term, "?Term@MemoryMgr@AK@@YAXXZ"),
        hook!(T_MEM_INIT_THREAD, mem_init_for_thread, "?InitForThread@MemoryMgr@AK@@YAXXZ"),
        hook!(T_MEM_TERM_THREAD, mem_term_for_thread, "?TermForThread@MemoryMgr@AK@@YAXXZ"),
        hook!(T_MEM_TRIM_THREAD, mem_trim_for_thread, "?TrimForThread@MemoryMgr@AK@@YAXXZ"),
        hook!(T_MEM_MALLOC, mem_malloc, "?Malloc@MemoryMgr@AK@@YAPEAXH_K@Z"),
        hook!(T_MEM_MALIGN, mem_malign, "?Malign@MemoryMgr@AK@@YAPEAXH_KI@Z"),
        hook!(T_MEM_REALLOC, mem_realloc, "?Realloc@MemoryMgr@AK@@YAPEAXHPEAX_K@Z"),
        hook!(T_MEM_REALLOC_ALIGNED, mem_realloc_aligned, "?ReallocAligned@MemoryMgr@AK@@YAPEAXHPEAX_KI@Z"),
        hook!(T_MEM_FREE, mem_free, "?Free@MemoryMgr@AK@@YAXHPEAX@Z"),
        hook!(T_MEM_GLOBAL_STATS, mem_get_global_stats, "?GetGlobalStats@MemoryMgr@AK@@YAXAEAUGlobalStats@12@@Z"),
        // Milestone A9: StreamMgr
        hook!(T_STM_CREATE, stream_mgr_create, "?Create@StreamMgr@AK@@YAPEAVIAkStreamMgr@2@AEBUAkStreamMgrSettings@@@Z"),
        hook!(T_STM_GET_DEF_DEV, stream_mgr_get_default_device_settings, "?GetDefaultDeviceSettings@StreamMgr@AK@@YAXAEAUAkDeviceSettings@@@Z"),
        hook!(T_STM_CREATE_DEV, stream_mgr_create_device, "?CreateDevice@StreamMgr@AK@@YAIAEBUAkDeviceSettings@@PEAVIAkLowLevelIOHook@12@@Z"),
        hook!(T_STM_DESTROY_DEV, stream_mgr_destroy_device, "?DestroyDevice@StreamMgr@AK@@YA?AW4AKRESULT@@I@Z"),
        hook!(T_STM_GET_RESOLVER, stream_mgr_get_file_location_resolver, "?GetFileLocationResolver@StreamMgr@AK@@YAPEAVIAkFileLocationResolver@12@XZ"),
        hook!(T_STM_SET_RESOLVER, stream_mgr_set_file_location_resolver, "?SetFileLocationResolver@StreamMgr@AK@@YAXPEAVIAkFileLocationResolver@12@@Z"),
        hook!(T_STM_SET_LANG, stream_mgr_set_current_language, "?SetCurrentLanguage@StreamMgr@AK@@YA?AW4AKRESULT@@PEB_W@Z"),
        hook!(T_STM_GET_LANG, stream_mgr_get_current_language, "?GetCurrentLanguage@StreamMgr@AK@@YAPEB_WXZ"),
        hook!(T_STM_ADD_LANG_OBS, stream_mgr_add_lang_observer, "?AddLanguageChangeObserver@StreamMgr@AK@@YA?AW4AKRESULT@@P6AXQEB_WPEAX@Z1@Z"),
        hook!(T_STM_REM_LANG_OBS, stream_mgr_remove_lang_observer, "?RemoveLanguageChangeObserver@StreamMgr@AK@@YAXPEAX@Z"),
        hook!(T_STM_FLUSH_CACHES, stream_mgr_flush_all_caches, "?FlushAllCaches@StreamMgr@AK@@YAXXZ"),
        // Milestone A9: SoundEngine & MusicEngine settings + segment info
        hook!(T_SE_GET_DEF_INIT, se_get_default_init_settings, "?GetDefaultInitSettings@SoundEngine@AK@@YAXAEAUAkInitSettings@@@Z"),
        hook!(T_SE_GET_DEF_PLAT, se_get_default_platform_init_settings, "?GetDefaultPlatformInitSettings@SoundEngine@AK@@YAXAEAUAkPlatformInitSettings@@@Z"),
        hook!(T_MUSIC_GET_DEF_INIT, music_get_default_init_settings, "?GetDefaultInitSettings@MusicEngine@AK@@YAXAEAUAkMusicSettings@@@Z"),
        hook!(T_MUSIC_GET_SEG_INFO, music_get_playing_segment_info, "?GetPlayingSegmentInfo@MusicEngine@AK@@YA?AW4AKRESULT@@IAEAUAkSegmentInfo@@_N@Z"),
        // Milestone A9: SoundEngine config, callbacks, banks, devices & string/wchar overloads
        hook!(T_GET_AUDIO_SETTINGS, get_audio_settings, "?GetAudioSettings@SoundEngine@AK@@YA?AW4AKRESULT@@AEAUAkAudioSettings@@@Z"),
        hook!(T_GET_SAMPLE_RATE, get_sample_rate, "?GetSampleRate@SoundEngine@AK@@YAIXZ"),
        hook!(T_GET_SAMPLE_TICK, get_sample_tick, "?GetSampleTick@SoundEngine@AK@@YA_KXZ"),
        hook!(T_GET_BUFFER_TICK, get_buffer_tick, "?GetBufferTick@SoundEngine@AK@@YAIXZ"),
        hook!(T_GET_SPEAKER_CFG, get_speaker_configuration, "?GetSpeakerConfiguration@SoundEngine@AK@@YA?AUAkChannelConfig@@_K@Z"),
        hook!(T_GET_OUT_DEV_CFG, get_output_device_configuration, "?GetOutputDeviceConfiguration@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEAUAkChannelConfig@@AEAUAk3DAudioSinkCapabilities@@@Z"),
        hook!(T_GET_PANNING_RULE, get_panning_rule, "?GetPanningRule@SoundEngine@AK@@YA?AW4AKRESULT@@AEAW4AkPanningRule@@_K@Z"),
        hook!(T_SET_PANNING_RULE, set_panning_rule, "?SetPanningRule@SoundEngine@AK@@YA?AW4AKRESULT@@W4AkPanningRule@@_K@Z"),
        hook!(T_SET_OUT_VOL, crate::bridge_entry!(set_output_volume), "?SetOutputVolume@SoundEngine@AK@@YA?AW4AKRESULT@@_KM@Z"),
        hook!(T_SET_VOL_THRESH, set_volume_threshold, "?SetVolumeThreshold@SoundEngine@AK@@YA?AW4AKRESULT@@M@Z"),
        hook!(T_SET_MAX_VOICES, set_max_num_voices_limit, "?SetMaxNumVoicesLimit@SoundEngine@AK@@YA?AW4AKRESULT@@G@Z"),
        hook!(T_SET_RNG_SEED, set_random_seed, "?SetRandomSeed@SoundEngine@AK@@YAXI@Z"),
        hook!(T_SUSPEND, se_suspend, "?Suspend@SoundEngine@AK@@YA?AW4AKRESULT@@_N0@Z"),
        hook!(T_WAKEUP, se_wakeup_from_suspend, "?WakeupFromSuspend@SoundEngine@AK@@YA?AW4AKRESULT@@I@Z"),
        hook!(T_CANCEL_EV_CB, cancel_event_callback, "?CancelEventCallback@SoundEngine@AK@@YAXI@Z"),
        hook!(T_CANCEL_EV_COOKIE, cancel_event_callback_cookie, "?CancelEventCallbackCookie@SoundEngine@AK@@YAXPEAX@Z"),
        hook!(T_CANCEL_EV_OBJ, cancel_event_callback_game_object, "?CancelEventCallbackGameObject@SoundEngine@AK@@YAX_K@Z"),
        hook!(T_CANCEL_BANK_COOKIE, cancel_bank_callback_cookie, "?CancelBankCallbackCookie@SoundEngine@AK@@YAXPEAX@Z"),
        hook!(T_UNREG_GLOBAL_CB, unregister_global_cb, "?UnregisterGlobalCallback@SoundEngine@AK@@YA?AW4AKRESULT@@P6AXPEAVIAkGlobalPluginContext@2@W4AkGlobalCallbackLocation@@PEAX@ZI@Z"),
        hook!(T_CLEAR_BANKS, clear_banks, "?ClearBanks@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
        hook!(T_CLEAR_PREPARED, clear_prepared_events, "?ClearPreparedEvents@SoundEngine@AK@@YA?AW4AKRESULT@@XZ"),
        hook!(T_GET_DEV_LIST1, get_device_list_1, "?GetDeviceList@SoundEngine@AK@@YA?AW4AKRESULT@@IAEAIPEAUAkDeviceDescription@@@Z"),
        hook!(T_GET_DEV_LIST2, get_device_list_2, "?GetDeviceList@SoundEngine@AK@@YA?AW4AKRESULT@@IIAEAIPEAUAkDeviceDescription@@@Z"),
        hook!(T_GET_DEV_SPATIAL, get_device_spatial_audio_support, "?GetDeviceSpatialAudioSupport@SoundEngine@AK@@YA?AW4AKRESULT@@I@Z"),
        hook!(T_GET_DEVICE_ID, ak_get_device_id, "?GetDeviceID@AK@@YAIPEAUIMMDevice@@@Z"),
        hook!(T_GET_DEVICE_ID_NAME, ak_get_device_id_from_name, "?GetDeviceIDFromName@AK@@YAIPEA_W@Z"),
        hook!(T_GET_WIN_DEV, ak_get_windows_device, "?GetWindowsDevice@AK@@YA_NHAEAIPEAPEAUIMMDevice@@W4AkAudioDeviceState@@@Z"),
        hook!(T_GET_WIN_DEV_COUNT, ak_get_windows_device_count, "?GetWindowsDeviceCount@AK@@YAIW4AkAudioDeviceState@@@Z"),
        hook!(T_GET_WIN_DEV_NAME, ak_get_windows_device_name, "?GetWindowsDeviceName@AK@@YAPEB_WHAEAIW4AkAudioDeviceState@@@Z"),
        hook!(T_POST_EVENT_A, post_event_a, "?PostEvent@SoundEngine@AK@@YAIPEBD_KIP6AXW4AkCallbackType@@PEAUAkCallbackInfo@@@ZPEAXIPEAUAkExternalSourceInfo@@I@Z"),
        hook!(T_POST_EVENT_W, post_event_w, "?PostEvent@SoundEngine@AK@@YAIPEB_W_KIP6AXW4AkCallbackType@@PEAUAkCallbackInfo@@@ZPEAXIPEAUAkExternalSourceInfo@@I@Z"),
        hook!(T_SET_RTPC_A, crate::bridge_entry!(set_rtpc_a), "?SetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDM_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_RTPC_W, crate::bridge_entry!(set_rtpc_w), "?SetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WM_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_RTPC_PID_A, crate::bridge_entry!(set_rtpc_pid_a), "?SetRTPCValueByPlayingID@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDMIHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_RTPC_PID_W, crate::bridge_entry!(set_rtpc_pid_w), "?SetRTPCValueByPlayingID@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WMIHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_RESET_RTPC_A, reset_rtpc_a, "?ResetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_RESET_RTPC_W, reset_rtpc_w, "?ResetRTPCValue@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_KHW4AkCurveInterpolation@@_N@Z"),
        hook!(T_SET_SWITCH_A, set_switch_a, "?SetSwitch@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD0_K@Z"),
        hook!(T_SET_SWITCH_W, set_switch_w, "?SetSwitch@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W0_K@Z"),
        hook!(T_SET_STATE_A, set_state_a, "?SetState@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD0@Z"),
        hook!(T_SET_STATE_W, set_state_w, "?SetState@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W0@Z"),
        hook!(T_POST_TRIGGER_A, post_trigger_a, "?PostTrigger@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_K@Z"),
        hook!(T_POST_TRIGGER_W, post_trigger_w, "?PostTrigger@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_K@Z"),
        hook!(T_EXEC_ACTION_W, exec_action_w, "?ExecuteActionOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WW4AkActionOnEventType@12@_KHW4AkCurveInterpolation@@I@Z"),
        hook!(T_EXEC_ACTION_PID, exec_action_on_playing_id, "?ExecuteActionOnPlayingID@SoundEngine@AK@@YAXW4AkActionOnEventType@12@IHW4AkCurveInterpolation@@@Z"),
        hook!(T_SEEK_A, seek_on_event_a, "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_KH_NI@Z"),
        hook!(T_SEEK_PCT_A, crate::bridge_entry!(seek_on_event_pct_a), "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD_KM_NI@Z"),
        hook!(T_SEEK_W, seek_on_event_w, "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_KH_NI@Z"),
        hook!(T_SEEK_PCT_W, crate::bridge_entry!(seek_on_event_pct_w), "?SeekOnEvent@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W_KM_NI@Z"),
        hook!(T_PIN_A, pin_event_a, "?PinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDCC@Z"),
        hook!(T_PIN_W, pin_event_w, "?PinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WCC@Z"),
        hook!(T_UNPIN_A, unpin_event_a, "?UnpinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@PEBD@Z"),
        hook!(T_UNPIN_W, unpin_event_w, "?UnpinEventInStreamCache@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_W@Z"),
        hook!(T_LOAD_BANK_ID, load_bank_id, "?LoadBank@SoundEngine@AK@@YA?AW4AKRESULT@@II@Z"),
        hook!(T_LOAD_BANK_A, load_bank_a, "?LoadBank@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDAEAII@Z"),
        hook!(T_LOAD_BANK_W, load_bank_w, "?LoadBank@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WAEAII@Z"),
        hook!(T_LOAD_COPY_EX, load_bank_copy_ex, "?LoadBankMemoryCopy@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI1@Z"),
        hook!(T_LOAD_VIEW_EX, load_bank_view_ex, "?LoadBankMemoryView@SoundEngine@AK@@YA?AW4AKRESULT@@PEBXIAEAI1@Z"),
        hook!(T_UNLOAD_A, unload_bank_a, "?UnloadBank@SoundEngine@AK@@YA?AW4AKRESULT@@PEBDPEBXI@Z"),
        hook!(T_UNLOAD_W, unload_bank_w, "?UnloadBank@SoundEngine@AK@@YA?AW4AKRESULT@@PEB_WPEBXI@Z"),
    ]
}

/// Number of exports the recorder hooks (for tests / logs).
pub fn hook_count() -> usize {
    table().len()
}

/// Total number of AK exports hooked when `AW2_AUDIO=native` is active.
pub fn native_hook_count() -> usize {
    table().len() + native_only_table().len()
}

/// Hook installer supplied by the loader: `(addr, func_end, target) -> trampoline`.
pub type Installer<'a> = dyn FnMut(usize, Option<usize>, usize) -> Result<usize, String> + 'a;

/// If `AW2_AK_RECORD=<trace>` or `AW2_AUDIO=native` is set: hook every export
/// in the table (plus query/capture exports in native mode) and optionally open
/// the trace file. Returns the number of hooks installed (0 when neither is set).
pub fn install_from_env(install: &mut Installer<'_>) -> Result<usize, String> {
    let rec_path = std::env::var_os("AW2_AK_RECORD").filter(|v| !v.is_empty());
    let is_native = native_mode();
    if rec_path.is_none() && !is_native {
        return Ok(0);
    }
    let l = crate::loader();
    let exe = l.get_module_handle(None).ok_or("no exe module")?;
    let mut hooks = table();
    if is_native {
        hooks.extend(native_only_table());
    }
    let total = hooks.len();
    let mut n = 0;
    let mut seen = HashSet::new();
    for h in hooks {
        let addr = l
            .get_proc_address(exe, crate::ProcName::Name(h.mangled))
            .ok_or_else(|| format!("AK hook: export {} not found", h.mangled))?;
        if !seen.insert(addr) {
            return Err(format!(
                "AK hook: {} shares 0x{addr:x} with another hooked export",
                h.mangled
            ));
        }
        let end = l.lookup_function_entry(addr as u64).and_then(|(base, f)| {
            let f = unsafe { &*f };
            (base as usize + f.begin_rva as usize == addr)
                .then_some(base as usize + f.end_rva as usize)
        });
        match install(addr, end, h.target) {
            Ok(tramp) => {
                h.slot.store(tramp, Ordering::Relaxed);
                n += 1;
                log::debug!(target: "audio", "AK hook: hooked {} @0x{addr:x}", h.mangled);
            }
            Err(e) => {
                log::error!(target: "audio", "AK hook: cannot hook {} @0x{addr:x}: {e}", h.mangled)
            }
        }
    }
    if let Some(path) = rec_path {
        start(Path::new(&path))?;
    }
    if is_native {
        let _ = native_engine();
        log::info!(target: "audio", "AW2_AUDIO=native: {n}/{total} AK exports routed to NativeSoundEngine");
    } else {
        log::info!(target: "audio", "AK recorder: {n}/{total} AK exports hooked");
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::backend::{self, Backend};
    use crate::audio::tap;

    #[test]
    fn native_mode_hooks_dispatch_and_capture_wav() {
        let _guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));
        set_native_mode(Some(true));

        let dir = std::env::temp_dir().join(format!("aw2-akrec-native-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let tap_wav = dir.join("tap.wav");
        let cap_wav = dir.join("capture.wav");
        tap::override_next(Some(tap_wav.clone()));

        unsafe {
            assert_eq!(se_init_hook(std::ptr::null_mut(), std::ptr::null_mut()), 1);
            assert!(se_is_initialized());
            assert_eq!(music_init(std::ptr::null_mut()), 1);

            let listener: u64 = 1;
            let emitter: u64 = 2;
            assert_eq!(register_obj(listener, std::ptr::null()), 1);
            assert_eq!(register_obj(emitter, std::ptr::null()), 1);
            assert_eq!(set_default_listeners(&listener, 1), 1);

            let rtpc_id = get_id_from_string_a(b"test_rtpc\0".as_ptr());
            assert_eq!(rtpc_id, wwise_audio::fnv1_32_lower("test_rtpc"));
            assert_eq!(set_rtpc(rtpc_id, 42.5, emitter, 0, 4, true), 1);

            let mut val = 0.0f32;
            let mut ty = 2i32;
            assert_eq!(query_get_rtpc(rtpc_id, emitter, 0, &mut val, &mut ty), 1);
            assert!((val - 42.5).abs() < 1e-4);

            let cap_w: Vec<u16> = cap_wav
                .to_string_lossy()
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            assert_eq!(start_output_capture(cap_w.as_ptr()), 1);

            for _ in 0..4 {
                assert_eq!(render_audio(true), 1);
            }
            assert!(frames() >= 4);

            assert_eq!(stop_output_capture(), 1);
            music_term();
            se_term();
            assert!(!se_is_initialized());
        }

        assert!(tap_wav.is_file(), "tap.wav should be created by CoreAudioSink");
        assert!(cap_wav.is_file(), "capture.wav should be created by start/stop_output_capture");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn zero_original_ak_lifecycle_and_subsystems() {
        let _guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));
        set_native_mode(Some(true));
        assert_eq!(native_hook_count(), 155);

        unsafe {
            // 1. MemoryMgr
            let mut mem_settings = [0xffu8; 0xb8];
            mem_get_default_settings(mem_settings.as_mut_ptr());
            assert!(mem_settings.iter().all(|&b| b == 0));
            assert_eq!(mem_init(mem_settings.as_mut_ptr()), 1);
            assert!(mem_is_initialized());
            let p = mem_malloc(0, 64);
            assert!(!p.is_null());
            let p2 = mem_realloc(0, p, 128);
            assert!(!p2.is_null());
            mem_free(0, p2);

            // 2. StreamMgr
            let stm = stream_mgr_create(std::ptr::null());
            assert!(!stm.is_null());
            let mut dev_settings = [0u8; 0x38];
            stream_mgr_get_default_device_settings(dev_settings.as_mut_ptr());
            assert_eq!((dev_settings.as_ptr().add(0x08) as *const u32).read_unaligned(), 0x0020_0000);
            assert_eq!(stream_mgr_create_device(dev_settings.as_ptr(), std::ptr::null_mut()), 0);
            let lang_w: Vec<u16> = "en(US)".encode_utf16().chain(std::iter::once(0)).collect();
            assert_eq!(stream_mgr_set_current_language(lang_w.as_ptr()), 1);
            assert!(!stream_mgr_get_current_language().is_null());

            // 3. DefaultInitSettings & Init
            let mut se_settings = [0u8; aktrace::INIT_SETTINGS_LEN];
            se_get_default_init_settings(se_settings.as_mut_ptr());
            assert_eq!((se_settings.as_ptr().add(0x18) as *const u32).read_unaligned(), 512);
            let mut plat_settings = [0u8; aktrace::PLATFORM_SETTINGS_LEN];
            se_get_default_platform_init_settings(plat_settings.as_mut_ptr());
            assert_eq!((plat_settings.as_ptr().add(0x3c) as *const u32).read_unaligned(), 48_000);
            let mut mus_settings = 0.0f32;
            music_get_default_init_settings(&mut mus_settings);
            assert_eq!(mus_settings, 1.0);

            assert_eq!(se_init_hook(se_settings.as_mut_ptr(), plat_settings.as_mut_ptr()), 1);
            assert_eq!(music_init(&mut mus_settings as *mut f32 as *mut u8), 1);
            assert_eq!(get_sample_rate(), 48_000);

            // 4. String / Wchar overloads & Queries
            assert_eq!(set_state_a(b"state_grp\0".as_ptr(), b"state_val\0".as_ptr()), 1);
            let mut st_out = 0u32;
            assert_eq!(query_get_state_a(b"state_grp\0".as_ptr(), &mut st_out), 1);
            assert_eq!(st_out, wwise_audio::fnv1_32_lower("state_val"));

            // 5. Teardown
            assert_eq!(clear_banks(), 1);
            music_term();
            se_term();
            assert_eq!(stream_mgr_destroy_device(0), 1);
            mem_term();
            assert!(!mem_is_initialized());
        }
    }
}


