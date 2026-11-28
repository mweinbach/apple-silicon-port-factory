//! PSO-level cache on the Metal 4 pipeline archive machinery (feature `metal`).
//!
//! Converting DXIL to a metallib is cached by [`crate::ShaderPipeline`]. Turning that into a
//! pipeline state means compiling AIR to GPU machine code, which this module caches across runs:
//!
//! * The cache owns the [`MTL4Compiler`]. When a disk directory is configured, the compiler gets
//!   an [`MTL4PipelineDataSetSerializer`] (capture binaries), so every pipeline it builds can be
//!   written out as an [`MTL4Archive`].
//! * On open, every `*.mtl4a` archive in `<dir>/<device+OS id>/` is loaded. A pipeline request
//!   first asks each archive (`newXPipelineStateWithDescriptor:error:`). That is a hit with no
//!   compilation. Only on a miss does the compiler build it, and the serializer captures it.
//! * [`PipelineCache::flush`] writes the session's misses as one new archive. Nothing is
//!   written if there were no misses. When more than `max_archives` archives exist at open,
//!   the session runs in compaction mode instead: lookups go through the compiler with all
//!   archives as `lookupArchives`, which is fast since the binaries come from the archives, and
//!   the serializer captures every pipeline used. Flush then writes one archive and deletes the
//!   old ones. Pipelines not used in that session are dropped.
//! * Optional in-memory deduplication: pass a key (e.g. a hash of the shader cache keys plus
//!   the fixed-function state) and repeated requests return the same pipeline object.
//!
//! Archives are only valid for one GPU and OS build. The directory name hashes
//! `MTLDevice.name` and the OS version string, so an OS update starts a fresh set.
//!
//! ```no_run
//! # use objc2_metal::*;
//! # fn demo(device: &objc2::runtime::ProtocolObject<dyn MTLDevice>, desc: &MTL4ComputePipelineDescriptor) -> Result<(), dxil_metal::Error> {
//! use dxil_metal::pso_cache::PipelineCache;
//! let cache = PipelineCache::from_env(device)?;   // AW2_PSO_CACHE, default <shader cache>/pso
//! let compiler = cache.compiler();                 // use it for MTL4 libraries etc. too
//! let pso = cache.compute_pipeline(Some(0x1234), desc)?;
//! cache.flush()?;                                  // at shutdown / after loading screens
//! # let _ = (compiler, pso); Ok(()) }
//! ```

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSArray, NSError, NSProcessInfo, NSString, NSURL};
use objc2_metal::{
    MTL4Archive, MTL4Compiler, MTL4CompilerDescriptor, MTL4CompilerTaskOptions, MTL4ComputePipelineDescriptor,
    MTL4LibraryDescriptor, MTL4LibraryFunctionDescriptor, MTL4PipelineDataSetSerializer,
    MTL4PipelineDataSetSerializerConfiguration, MTL4PipelineDataSetSerializerDescriptor, MTL4PipelineDescriptor,
    MTLCompileOptions, MTLComputePipelineState, MTLDevice, MTLLanguageVersion, MTLLibrary,
    MTLMathFloatingPointFunctions, MTLMathMode, MTLRenderPipelineState,
};

use crate::Error;

/// `MTLLanguageVersion4_1` (`(4 << 16) + 1`, introduced in macOS 27 / Metal 4.1).
pub const LANGUAGE_VERSION_4_1: MTLLanguageVersion = MTLLanguageVersion((4 << 16) + 1);

/// `MTLFloatingPointConversionRoundingModeTowardZero` (macOS 27 / iOS 27, Metal 4.1 §1.6.3).
const FP_CONVERSION_ROUNDING_RTZ: isize = 1;

/// Options for compiling MSL source via [`PipelineCache::library_from_source_with_compile_opts`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SourceCompileOptions {
    /// Force `MTLMathMode::Fast` and `MTLMathFloatingPointFunctions::Fast`.
    pub fast_math: bool,
    /// Metal 4.1 (`macOS 27` / `iOS 27`): request round-toward-zero narrowing float conversions
    /// (`MTLFloatingPointConversionRoundingModeTowardZero` / `-fmetal-rtz-fp-conversion`) for
    /// DirectX/SPIR-V compatibility (`half(70000.0f) == 65504.0` instead of `+Inf`).
    /// Ignored gracefully on older OSes via `respondsToSelector:`.
    pub rtz_fp_conversion: bool,
}

fn parse_env_bool(name: &str) -> Option<bool> {
    let v = std::env::var(name).ok()?;
    let t = v.trim();
    if t.is_empty() {
        return None;
    }
    Some(!matches!(t.to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off"))
}

/// True when `PORT_METAL41_RTZ_FP` (or master `PORT_METAL41`) opts into MSL 4.1 RTZ float
/// conversion rounding mode. Default `false`.
pub fn rtz_fp_conversion_enabled() -> bool {
    parse_env_bool("PORT_METAL41_RTZ_FP")
        .or_else(|| parse_env_bool("PORT_METAL41"))
        .unwrap_or(false)
}

pub type ComputePipeline = Retained<ProtocolObject<dyn MTLComputePipelineState>>;
pub type RenderPipeline = Retained<ProtocolObject<dyn MTLRenderPipelineState>>;
pub type Library = Retained<ProtocolObject<dyn MTLLibrary>>;

/// Archives beyond which a session compacts (override: `PORT_PSO_MAX_ARCHIVES` / `AW2_PSO_MAX_ARCHIVES`).
/// High on iOS (256) so read-only `.mtl4a` archives are never forced back through `MTL4Compiler`
/// + serializer during gameplay (compacting burns hundreds of MiB of LLVM compiler scratch).
#[cfg(target_os = "ios")]
pub const DEFAULT_MAX_ARCHIVES: usize = 256;
#[cfg(not(target_os = "ios"))]
pub const DEFAULT_MAX_ARCHIVES: usize = 64;
const ARCHIVE_EXT: &str = "mtl4a";
const IDX_EXT: &str = "idx";
const TMP_PREFIX: &str = "tmp-";
const IDX_MAGIC: [u8; 4] = *b"M4IX";
const IDX_VERSION: u32 = 1;
const KIND_COMPUTE: u8 = 1;
const KIND_RENDER: u8 = 2;

/// Number of cold `MTL4Compiler` compiles between automatic mid-frame flush + compiler
/// resets (`PORT_IOS_PSO_RESET_BATCH`, default `12` on iOS, `0` = disabled on macOS).
/// Flushing + recreating `MTL4Compiler` releases Apple's internal LLVM/AGX compiler scratch
/// arenas and checkpoints newly compiled PSOs to disk before `Present()` runs.
fn pso_reset_batch() -> u64 {
    static B: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *B.get_or_init(|| {
        let default_batch: u64 = if cfg!(target_os = "ios") { 12 } else { 0 };
        std::env::var("PORT_IOS_PSO_RESET_BATCH")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(default_batch)
    })
}

/// Snapshot of cache counters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PipelineCacheStats {
    /// Requests served from the in-memory key map.
    pub memory_hits: u64,
    /// Pipelines loaded from an archive without compilation.
    pub archive_hits: u64,
    /// Pipelines built by the compiler. In compaction mode this includes archive-backed ones.
    pub compiles: u64,
    pub failures: u64,
    pub archives_loaded: u64,
    pub archives_written: u64,
    /// Archive files that failed to load (deleted).
    pub archives_rejected: u64,
    pub lookup_nanos: u64,
    pub compile_nanos: u64,
}

#[derive(Default)]
struct Counters {
    memory_hits: AtomicU64,
    archive_hits: AtomicU64,
    archive_probes: AtomicU64,
    in_flight_waits: AtomicU64,
    compiles: AtomicU64,
    failures: AtomicU64,
    archives_loaded: AtomicU64,
    archives_written: AtomicU64,
    archives_rejected: AtomicU64,
    lookup_nanos: AtomicU64,
    compile_nanos: AtomicU64,
}

/// Configuration for [`PipelineCache::open`].
#[derive(Debug, Clone)]
pub struct PipelineCacheConfig {
    /// Root directory (`None` = no disk caching: plain compiler + optional in-memory dedup).
    /// Archives live in `<dir>/<device id>/`.
    pub dir: Option<PathBuf>,
    pub max_archives: usize,
    /// Flush on drop (best effort).
    pub flush_on_drop: bool,
}

impl PipelineCacheConfig {
    /// `PORT_PSO_CACHE` / `AW2_PSO_CACHE` = directory, or `off`/`0`/empty to disable. The default is
    /// `<shader cache root>/pso`, which is off when `PORT_SHADER_CACHE`/`AW2_SHADER_CACHE` is off.
    /// `PORT_PSO_MAX_ARCHIVES` / `AW2_PSO_MAX_ARCHIVES` sets `max_archives`.
    pub fn from_env() -> Self {
        let dir = match std::env::var_os("PORT_PSO_CACHE")
            .or_else(|| std::env::var_os("AW2_PSO_CACHE"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => None,
            Some(v) => Some(PathBuf::from(v)),
            None => crate::CacheConfig::from_env().disk_dir.map(|d| d.join("pso")),
        };
        let max_archives = std::env::var("PORT_PSO_MAX_ARCHIVES")
            .or_else(|_| std::env::var("AW2_PSO_MAX_ARCHIVES"))
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_MAX_ARCHIVES);
        Self { dir, max_archives, flush_on_drop: true }
    }
}

/// The serializer protocol isn't marked thread-safe in the bindings; it is only touched under
/// this mutex (flush). The compiler feeds it internally, which Metal synchronizes.
struct Serializer(Retained<ProtocolObject<dyn MTL4PipelineDataSetSerializer>>);
unsafe impl Send for Serializer {}

/// Bounded concurrency gate for cold `MTL4Compiler` jobs (prevents multi-thread
/// AIR/AGX compiler scratch spikes from tripping iOS Jetsam during level loads).
/// `limit == 0` disables throttling (unlimited concurrency).
struct CompileLimiter {
    limit: usize,
    active: Mutex<usize>,
    cv: Condvar,
}

struct CompilePermit<'a> {
    limiter: Option<&'a CompileLimiter>,
}

impl Drop for CompilePermit<'_> {
    fn drop(&mut self) {
        if let Some(l) = self.limiter {
            let mut active = l.active.lock().unwrap();
            *active = active.saturating_sub(1);
            l.cv.notify_one();
        }
    }
}

impl CompileLimiter {
    fn from_env() -> Self {
        let default_limit: usize = if cfg!(target_os = "ios") { 1 } else { 0 };
        let limit = std::env::var("PORT_MAX_PSO_COMPILES")
            .ok()
            .and_then(|v| v.trim().parse::<usize>().ok())
            .unwrap_or(default_limit);
        if limit > 0 {
            log::info!("pso cache: limiting concurrent cold MTL4Compiler jobs to {limit}");
        }
        Self { limit, active: Mutex::new(0), cv: Condvar::new() }
    }

    fn acquire(&self) -> CompilePermit<'_> {
        if self.limit == 0 {
            return CompilePermit { limiter: None };
        }
        let mut active = self.active.lock().unwrap();
        while *active >= self.limit {
            active = self.cv.wait(active).unwrap();
        }
        *active += 1;
        CompilePermit { limiter: Some(self) }
    }
}

struct InFlightSlot {
    done: Mutex<bool>,
    cv: Condvar,
}

/// Per-key singleflight table preventing duplicate concurrent archive probes and
/// `MTL4Compiler` invocations when multiple threads request the same PSO key.
#[derive(Default)]
struct InFlightTable {
    active: Mutex<HashMap<u128, Arc<InFlightSlot>>>,
}

struct InFlightGuard<'a> {
    table: &'a InFlightTable,
    key: u128,
    slot: Arc<InFlightSlot>,
}

impl Drop for InFlightGuard<'_> {
    fn drop(&mut self) {
        {
            let mut map = self.table.active.lock().unwrap();
            map.remove(&self.key);
        }
        {
            let mut done = self.slot.done.lock().unwrap();
            *done = true;
        }
        self.slot.cv.notify_all();
    }
}

#[derive(Default)]
struct BgFlushState {
    requested: bool,
    in_flight: bool,
    sync_waiters: usize,
    shutdown: bool,
}

/// Shared serializer & archive-writing state accessible from both foreground calls
/// and the background incremental serializer thread.
struct FlushCore {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    compiler: RwLock<Retained<ProtocolObject<dyn MTL4Compiler>>>,
    serializer: Mutex<Option<Serializer>>,
    /// `(kind, key)` pairs captured by the current `serializer` instance.
    serializer_keys: Mutex<HashSet<(u8, u128)>>,
    compile_lock: RwLock<()>,
    /// Archive files loaded at open (deleted only after a final compacting flush).
    archive_paths: Vec<PathBuf>,
    /// `<dir>/<device id>` when disk caching is on.
    device_dir: Option<PathBuf>,
    compacting: bool,
    dirty: AtomicBool,
    compiles_since_flush: AtomicU64,
    flushed: AtomicBool,
    counters: Counters,
    bg_mu: Mutex<BgFlushState>,
    bg_cv: Condvar,
}
unsafe impl Send for FlushCore {}
unsafe impl Sync for FlushCore {}

fn idx_path(archive_path: &Path) -> PathBuf {
    archive_path.with_extension(IDX_EXT)
}

fn read_archive_index(archive_path: &Path) -> Option<Vec<(u8, u128)>> {
    let expected_len = std::fs::metadata(archive_path).ok()?.len();
    let idx_p = idx_path(archive_path);
    let b = std::fs::read(&idx_p).ok()?;
    if b.len() < 28 || b[..4] != IDX_MAGIC {
        let _ = std::fs::remove_file(&idx_p);
        return None;
    }
    let version = u32::from_le_bytes(b[4..8].try_into().ok()?);
    let archive_len = u64::from_le_bytes(b[8..16].try_into().ok()?);
    let count = u32::from_le_bytes(b[16..20].try_into().ok()?) as usize;
    let payload_end = 20usize.checked_add(count.checked_mul(17)?)?;
    if version != IDX_VERSION || archive_len != expected_len || b.len() != payload_end + 8 {
        let _ = std::fs::remove_file(&idx_p);
        return None;
    }
    let expected_sum = crate::hash::xxh64(&b[..payload_end], 0).to_le_bytes();
    if b[payload_end..] != expected_sum {
        let _ = std::fs::remove_file(&idx_p);
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let off = 20 + i * 17;
        let kind = b[off];
        let key = u128::from_le_bytes(b[off + 1..off + 17].try_into().ok()?);
        out.push((kind, key));
    }
    Some(out)
}

fn write_archive_index(archive_path: &Path, keys: &[(u8, u128)]) -> Result<(), Error> {
    let archive_len = std::fs::metadata(archive_path)
        .map_err(|e| Error::Io(format!("pso cache: stat {}: {e}", archive_path.display())))?
        .len();
    let mut buf = Vec::with_capacity(28 + keys.len() * 17);
    buf.extend_from_slice(&IDX_MAGIC);
    buf.extend_from_slice(&IDX_VERSION.to_le_bytes());
    buf.extend_from_slice(&archive_len.to_le_bytes());
    buf.extend_from_slice(&(keys.len() as u32).to_le_bytes());
    for &(kind, key) in keys {
        buf.push(kind);
        buf.extend_from_slice(&key.to_le_bytes());
    }
    let sum = crate::hash::xxh64(&buf, 0).to_le_bytes();
    buf.extend_from_slice(&sum);
    let dst = idx_path(archive_path);
    let tmp_name = dst
        .file_name()
        .map(|n| format!("{TMP_PREFIX}{}", n.to_string_lossy()))
        .unwrap_or_else(|| format!("{TMP_PREFIX}index.{IDX_EXT}"));
    let tmp = dst.with_file_name(tmp_name);
    std::fs::write(&tmp, &buf)
        .map_err(|e| Error::Io(format!("pso cache: write {}: {e}", tmp.display())))?;
    if let Err(e) = std::fs::rename(&tmp, &dst) {
        let _ = std::fs::remove_file(&tmp);
        return Err(Error::Io(format!("pso cache: rename {}: {e}", dst.display())));
    }
    Ok(())
}

/// Pipeline-state cache; see the [module docs](self). Thread-safe.
pub struct PipelineCache {
    flush_core: Arc<FlushCore>,
    bg_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
    compile_limiter: CompileLimiter,
    post_batch_hook: Mutex<Option<fn()>>,
    archives: Vec<Retained<ProtocolObject<dyn MTL4Archive>>>,
    /// Persistent + runtime index mapping `(kind, key)` to the archive index holding it.
    key_to_archive: RwLock<HashMap<(u8, u128), usize>>,
    /// Legacy or corrupt-index archive indices that lack a verified `.idx` sidecar.
    unindexed_archives: Vec<usize>,
    /// Human-readable `MTLDevice.name | macOS build` identity string.
    device_id_str: String,
    flush_on_drop: bool,
    compute: RwLock<HashMap<u128, ComputePipeline>>,
    render: RwLock<HashMap<u128, RenderPipeline>>,
    in_flight_compute: InFlightTable,
    in_flight_render: InFlightTable,
}

fn create_compiler_and_serializer(
    device: &ProtocolObject<dyn MTLDevice>,
    capture: bool,
) -> Result<(Retained<ProtocolObject<dyn MTL4Compiler>>, Option<Serializer>), Error> {
    objc2::rc::autoreleasepool(|_| {
        let desc = guarded("MTL4CompilerDescriptor::new", MTL4CompilerDescriptor::new)?;
        desc.setLabel(Some(&NSString::from_str("aw2 pipeline cache")));
        let serializer = capture.then(|| {
            match guarded("newPipelineDataSetSerializerWithDescriptor", || {
                let sd = MTL4PipelineDataSetSerializerDescriptor::new();
                sd.setConfiguration(MTL4PipelineDataSetSerializerConfiguration::CaptureBinaries);
                device.newPipelineDataSetSerializerWithDescriptor(&sd)
            }) {
                Ok(s) => Some(s),
                Err(e) => {
                    log::warn!("pso cache: serializer unavailable ({e}); continuing without archive capture");
                    None
                }
            }
        }).flatten();
        if let Some(s) = &serializer {
            desc.setPipelineDataSetSerializer(Some(s));
        }
        let compiler = guarded("newCompilerWithDescriptor", || device.newCompilerWithDescriptor_error(&desc))
            .and_then(|r| r.map_err(|e| ns_err("newCompilerWithDescriptor", &e)))?;
        Ok((compiler, serializer.map(Serializer)))
    })
}

fn ns_err(what: &str, e: &NSError) -> Error {
    Error::Metal(format!("{what}: {}", e.localizedDescription()))
}

const XPC_RETRY_BACKOFFS_MS: &[u64] = &[50, 150];

fn is_transient_xpc_interrupt(err: &Error) -> bool {
    let Error::Metal(msg) = err else {
        return false;
    };
    !msg.contains("Objective-C exception")
        && (msg.contains("XPC_ERROR_CONNECTION_INTERRUPTED")
            || msg.contains("interrupted connection"))
}

/// Retries a Metal compiler call when Apple's out-of-process `MTLCompilerService`
/// XPC helper restarts while concurrent threads are compiling pipelines/libraries.
fn retry_on_xpc_interrupt<T>(
    what: &str,
    backoffs_ms: &[u64],
    mut f: impl FnMut() -> Result<T, Error>,
) -> Result<T, Error> {
    let mut attempt = 0usize;
    loop {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) if attempt < backoffs_ms.len() && is_transient_xpc_interrupt(&e) => {
                let wait_ms = backoffs_ms[attempt];
                attempt += 1;
                log::warn!(
                    "pso cache: {what} hit transient Metal compiler XPC disconnect (retry {attempt}/{} after {wait_ms}ms): {e}",
                    backoffs_ms.len()
                );
                if wait_ms > 0 {
                    std::thread::sleep(Duration::from_millis(wait_ms));
                }
            }
            Err(e) => return Err(e),
        }
    }
}

/// Runs Metal calls that report misuse by raising an Objective-C exception (which would
/// otherwise abort the process across the Rust boundary) and turns the exception into an error.
fn guarded<R>(what: &str, f: impl FnOnce() -> R) -> Result<R, Error> {
    objc2::exception::catch(std::panic::AssertUnwindSafe(f)).map_err(|e| {
        Error::Metal(format!("{what}: Objective-C exception: {}", e.map_or_else(|| "nil".to_string(), |e| format!("{e:?}"))))
    })
}

/// Structural check of a serialized `MTL4Archive`: a fat container with magic `cbfebabe`, then a
/// big-endian `u32` arch count, then `{cputype, cpusubtype, offset, size, align}` entries. Every
/// slice must lie inside the file. This was observed on macOS 27 / AGX G15X. It catches junk
/// and truncated files, which Metal itself does not survive.
fn check_archive_file(p: &Path) -> Result<(), Error> {
    let bad = |why: &str| Error::Io(format!("{}: not an MTL4 archive ({why})", p.display()));
    let b = std::fs::read(p).map_err(|e| Error::Io(format!("{}: {e}", p.display())))?;
    let be = |o: usize| b.get(o..o + 4).map(|s| u32::from_be_bytes(s.try_into().unwrap()));
    if be(0) != Some(0xcbfe_babe) {
        return Err(bad("magic"));
    }
    let n = be(4).ok_or_else(|| bad("header"))? as usize;
    if n == 0 || n > 64 {
        return Err(bad("arch count"));
    }
    for i in 0..n {
        let e = 8 + i * 20;
        let (off, size) = (be(e + 8).ok_or_else(|| bad("arch table"))?, be(e + 12).ok_or_else(|| bad("arch table"))?);
        if off as u64 + size as u64 > b.len() as u64 {
            return Err(bad("truncated"));
        }
    }
    Ok(())
}

fn url(p: &Path) -> Retained<NSURL> {
    NSURL::fileURLWithPath(&NSString::from_str(&p.to_string_lossy()))
}

fn nanos(t: Instant) -> u64 {
    t.elapsed().as_nanos() as u64
}

/// Identity that pipeline binaries are valid for: GPU name + OS version/build.
pub fn device_identity(device: &ProtocolObject<dyn MTLDevice>) -> String {
    let os = NSProcessInfo::processInfo().operatingSystemVersionString();
    format!("{} | {}", device.name(), os)
}

/// Directory name for [`device_identity`]: a readable GPU name plus a hash of the full identity.
fn device_dir_name(device: &ProtocolObject<dyn MTLDevice>) -> String {
    let id = device_identity(device);
    let name: String = device
        .name()
        .to_string()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    format!("{}-{:016x}", name.trim_matches('-'), crate::hash::xxh64(id.as_bytes(), 0))
}

impl PipelineCache {
    /// Opens the cache configured by [`PipelineCacheConfig::from_env`].
    pub fn from_env(device: &ProtocolObject<dyn MTLDevice>) -> Result<Self, Error> {
        Self::open(device, &PipelineCacheConfig::from_env())
    }

    /// Creates the compiler and loads the device's archives. If the directory is unusable,
    /// it logs and continues without disk caching.
    pub fn open(device: &ProtocolObject<dyn MTLDevice>, config: &PipelineCacheConfig) -> Result<Self, Error> {
        let device_id_str = device_identity(device);
        log::info!("pso cache: opening for device {device_id_str}");
        let mut device_dir = config.dir.as_ref().map(|d| d.join(device_dir_name(device)));
        if let Some(d) = &device_dir {
            if let Err(e) = std::fs::create_dir_all(d) {
                log::warn!("pso cache: cannot create {}: {e}; disk caching off", d.display());
                device_dir = None;
            } else {
                let _ = std::fs::write(d.join("device.txt"), &device_id_str);
            }
        }
        let counters = Counters::default();

        let mut archives = Vec::new();
        let mut archive_paths = Vec::new();
        let mut key_to_archive: HashMap<(u8, u128), usize> = HashMap::new();
        let mut unindexed_archives: Vec<usize> = Vec::new();
        let mut orphan_idxs: Vec<PathBuf> = Vec::new();
        if let Some(d) = &device_dir {
            let mut files: Vec<PathBuf> = std::fs::read_dir(d)
                .map(|it| it.filter_map(|e| e.ok().map(|e| e.path())).collect())
                .unwrap_or_default();
            files.sort();
            let mut live_archives: HashSet<PathBuf> = HashSet::new();
            for p in files {
                let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                if name.starts_with(TMP_PREFIX) {
                    let _ = std::fs::remove_file(&p); // torn write from a crashed session
                    continue;
                }
                if p.extension().and_then(|e| e.to_str()) == Some(IDX_EXT) {
                    orphan_idxs.push(p);
                    continue;
                }
                if p.extension().and_then(|e| e.to_str()) != Some(ARCHIVE_EXT) {
                    continue;
                }
                // Metal raises (doesNotRecognizeSelector inside MTL4ArchiveImpl::loadFromURL)
                // on a garbage file and is left unstable afterwards, so check the container first.
                let r = check_archive_file(&p).and_then(|()| {
                    guarded("newArchiveWithURL", || device.newArchiveWithURL_error(&url(&p)))
                        .and_then(|r| r.map_err(|e| ns_err("newArchiveWithURL", &e)))
                });
                match r {
                    Ok(a) => {
                        let slot = archives.len();
                        if let Some(entries) = read_archive_index(&p) {
                            for (kind, key) in entries {
                                key_to_archive.insert((kind, key), slot);
                            }
                        } else {
                            unindexed_archives.push(slot);
                        }
                        live_archives.insert(p.clone());
                        archives.push(a);
                        archive_paths.push(p);
                        counters.archives_loaded.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(e) => {
                        log::warn!("pso cache: dropping unreadable archive {}: {e}", p.display());
                        let _ = std::fs::remove_file(&p);
                        let _ = std::fs::remove_file(idx_path(&p));
                        counters.archives_rejected.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
            for ip in orphan_idxs {
                let matching_archive = ip.with_extension(ARCHIVE_EXT);
                if !live_archives.contains(&matching_archive) {
                    let _ = std::fs::remove_file(&ip);
                }
            }
        }

        let (compiler, serializer) = create_compiler_and_serializer(device, device_dir.is_some())?;
        let compacting = archives.len() > config.max_archives.max(1);
        if !archives.is_empty() {
            log::info!(
                "pso cache: loaded {} archive(s) ({} indexed keys, {} unindexed)",
                archives.len(),
                key_to_archive.len(),
                unindexed_archives.len()
            );
        }
        if compacting {
            log::info!("pso cache: {} archives > {}; compacting this session", archives.len(), config.max_archives);
        }
        let flush_core = Arc::new(FlushCore {
            device: objc2::Message::retain(device),
            compiler: RwLock::new(compiler),
            serializer: Mutex::new(serializer),
            serializer_keys: Mutex::new(HashSet::new()),
            compile_lock: RwLock::new(()),
            archive_paths,
            device_dir,
            compacting,
            dirty: AtomicBool::new(false),
            compiles_since_flush: AtomicU64::new(0),
            flushed: AtomicBool::new(false),
            counters,
            bg_mu: Mutex::new(BgFlushState::default()),
            bg_cv: Condvar::new(),
        });
        Ok(Self {
            flush_core,
            bg_thread: Mutex::new(None),
            compile_limiter: CompileLimiter::from_env(),
            post_batch_hook: Mutex::new(None),
            archives,
            key_to_archive: RwLock::new(key_to_archive),
            unindexed_archives,
            device_id_str,
            flush_on_drop: config.flush_on_drop,
            compute: RwLock::new(HashMap::new()),
            render: RwLock::new(HashMap::new()),
            in_flight_compute: InFlightTable::default(),
            in_flight_render: InFlightTable::default(),
        })
    }

    /// The compiler pipelines are built with. The D3D12 layer can use it for everything else
    /// too, e.g. `newLibraryWithDescriptor`. Pipelines built on it directly are also captured
    /// for the next archive.
    pub fn compiler(&self) -> Retained<ProtocolObject<dyn MTL4Compiler>> {
        self.flush_core.compiler.read().unwrap().clone()
    }

    /// Register a callback invoked after an automatic mid-frame PSO batch flush + compiler
    /// reset (`PORT_IOS_PSO_RESET_BATCH`), e.g. to trim transient `MTLLibrary` / metallib
    /// caches and trigger Darwin malloc pressure relief.
    pub fn set_post_batch_hook(&self, hook: fn()) {
        *self.post_batch_hook.lock().unwrap() = Some(hook);
    }

    /// Directory holding this device's archives (`None` = disk caching off).
    pub fn dir(&self) -> Option<&Path> {
        self.flush_core.device_dir.as_deref()
    }

    /// Parent root directory containing per-device subdirectories (`None` = disk caching off).
    pub fn root_dir(&self) -> Option<&Path> {
        self.flush_core.device_dir.as_deref().and_then(Path::parent)
    }

    /// Human-readable `MTLDevice.name | macOS build` identity string.
    pub fn device_identity_str(&self) -> &str {
        &self.device_id_str
    }

    /// True when disk caching is active and this `(GPU, OS build)` directory has zero loaded
    /// `MTL4Archive` binaries (i.e. first launch on a new Mac or after a macOS update).
    pub fn is_cold_device(&self) -> bool {
        self.flush_core.device_dir.is_some() && self.archives.is_empty()
    }

    pub fn is_compacting(&self) -> bool {
        self.flush_core.compacting
    }

    /// Total individual `MTL4Archive` lookup calls issued (for diagnostics/tests verifying
    /// indexed O(1) lookups and 0-probe misses).
    pub fn archive_probes(&self) -> u64 {
        self.flush_core.counters.archive_probes.load(Ordering::Relaxed)
    }

    /// Number of threads that coalesced behind an already in-flight compile for the same key.
    pub fn in_flight_waits(&self) -> u64 {
        self.flush_core.counters.in_flight_waits.load(Ordering::Relaxed)
    }

    fn task_options(&self) -> Option<Retained<MTL4CompilerTaskOptions>> {
        (self.flush_core.compacting && !self.archives.is_empty()).then(|| {
            let o = MTL4CompilerTaskOptions::new();
            o.setLookupArchives(Some(&NSArray::from_retained_slice(&self.archives)));
            o
        })
    }

    fn probe_archive<P>(
        &self,
        archive: &ProtocolObject<dyn MTL4Archive>,
        from_archive: &impl Fn(&ProtocolObject<dyn MTL4Archive>) -> Result<P, Retained<NSError>>,
    ) -> Option<P> {
        self.flush_core.counters.archive_probes.fetch_add(1, Ordering::Relaxed);
        let r = guarded("archive lookup", || from_archive(archive));
        if let Err(e) = &r {
            log::warn!("pso cache: {e}");
        }
        match r {
            Ok(Ok(p)) => Some(p),
            _ => None,
        }
    }

    fn build<P: Clone>(
        &self,
        kind: u8,
        key: Option<u128>,
        memo: &RwLock<HashMap<u128, P>>,
        in_flight: &InFlightTable,
        from_archive: impl Fn(&ProtocolObject<dyn MTL4Archive>) -> Result<P, Retained<NSError>>,
        compile: impl Fn(&ProtocolObject<dyn MTL4Compiler>, Option<&MTL4CompilerTaskOptions>) -> Result<P, Retained<NSError>>,
        what: &str,
    ) -> Result<P, Error> {
        // 1. Fast path + per-key singleflight deduplication so concurrent requests for
        //    the same PSO key never perform redundant archive probes or MTL4Compiler builds.
        let _in_flight_guard = if let Some(k) = key {
            loop {
                if let Some(p) = memo.read().unwrap().get(&k) {
                    self.flush_core.counters.memory_hits.fetch_add(1, Ordering::Relaxed);
                    return Ok(p.clone());
                }
                let wait_slot = {
                    let mut map = in_flight.active.lock().unwrap();
                    if let Some(p) = memo.read().unwrap().get(&k) {
                        self.flush_core.counters.memory_hits.fetch_add(1, Ordering::Relaxed);
                        return Ok(p.clone());
                    }
                    if let Some(existing) = map.get(&k) {
                        Some(existing.clone())
                    } else {
                        let slot = Arc::new(InFlightSlot {
                            done: Mutex::new(false),
                            cv: Condvar::new(),
                        });
                        map.insert(k, slot.clone());
                        drop(map);
                        break Some(InFlightGuard { table: in_flight, key: k, slot });
                    }
                };
                if let Some(slot) = wait_slot {
                    self.flush_core.counters.in_flight_waits.fetch_add(1, Ordering::Relaxed);
                    let mut done = slot.done.lock().unwrap();
                    while !*done {
                        done = slot.cv.wait(done).unwrap();
                    }
                }
            }
        } else {
            None
        };

        let mut did_compile = false;
        let p = objc2::rc::autoreleasepool(|_| -> Result<P, Error> {
            if !self.flush_core.compacting && !self.archives.is_empty() {
                let t = Instant::now();
                let hit = if let Some(k) = key {
                    let indexed_slot = self.key_to_archive.read().unwrap().get(&(kind, k)).copied();
                    let mut found = None;
                    if let Some(idx) = indexed_slot {
                        if let Some(a) = self.archives.get(idx) {
                            found = self.probe_archive(a, &from_archive);
                        }
                    }
                    if found.is_none() {
                        // Only probe legacy archives that lack a verified `.idx` sidecar;
                        // indexed archives are skipped in O(1) without touching ObjC/catch.
                        for &idx in &self.unindexed_archives {
                            if Some(idx) == indexed_slot {
                                continue;
                            }
                            if let Some(p) = self.probe_archive(&self.archives[idx], &from_archive) {
                                self.key_to_archive.write().unwrap().insert((kind, k), idx);
                                found = Some(p);
                                break;
                            }
                        }
                    }
                    found
                } else {
                    let mut found = None;
                    for a in &self.archives {
                        if let Some(p) = self.probe_archive(a, &from_archive) {
                            found = Some(p);
                            break;
                        }
                    }
                    found
                };
                self.flush_core.counters.lookup_nanos.fetch_add(nanos(t), Ordering::Relaxed);
                if let Some(p) = hit {
                    self.flush_core.counters.archive_hits.fetch_add(1, Ordering::Relaxed);
                    return Ok(p);
                }
            }
            let _permit = self.compile_limiter.acquire();
            // Another worker may have finished compiling this exact key while we waited.
            if let Some(k) = key {
                if let Some(p) = memo.read().unwrap().get(&k) {
                    self.flush_core.counters.memory_hits.fetch_add(1, Ordering::Relaxed);
                    return Ok(p.clone());
                }
            }
            let t = Instant::now();
            let r = retry_on_xpc_interrupt(what, XPC_RETRY_BACKOFFS_MS, || {
                // Re-acquire locks per attempt so we never hold `compile_lock` or `compiler`
                // across retry backoff sleeps, while still keeping `compile_lock` held from a
                // successful compile through serializer key/dirty updates.
                let opts = self.task_options();
                let _compile_guard = self.flush_core.compile_lock.read().unwrap();
                let compiler = self.flush_core.compiler.read().unwrap();
                let res = guarded(what, || compile(&*compiler, opts.as_deref()))
                    .and_then(|r| r.map_err(|e| ns_err(what, &e)));
                drop(compiler);
                if res.is_ok() {
                    if let Some(k) = key {
                        self.flush_core.serializer_keys.lock().unwrap().insert((kind, k));
                    }
                    self.flush_core.counters.compiles.fetch_add(1, Ordering::Relaxed);
                    self.flush_core.compiles_since_flush.fetch_add(1, Ordering::Relaxed);
                    self.flush_core.dirty.store(true, Ordering::Release);
                    did_compile = true;
                }
                res
            });
            self.flush_core.counters.compile_nanos.fetch_add(nanos(t), Ordering::Relaxed);
            match r {
                Ok(p) => Ok(p),
                Err(e) => {
                    self.flush_core.counters.failures.fetch_add(1, Ordering::Relaxed);
                    Err(e)
                }
            }
        })?;
        let out = if let Some(k) = key {
            // Insert before releasing `_in_flight_guard` so waking waiters immediately hit `memo`.
            memo.write().unwrap().entry(k).or_insert(p).clone()
        } else {
            p
        };
        drop(_in_flight_guard);
        if did_compile && !self.flush_core.compacting {
            let batch = pso_reset_batch();
            if batch > 0 && self.flush_core.compiles_since_flush.load(Ordering::Relaxed) >= batch {
                match self.flush_and_reset_compiler_if_dirty() {
                    Ok(Some(_)) => {
                        if let Some(hook) = *self.post_batch_hook.lock().unwrap() {
                            hook();
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        log::warn!("pso cache: mid-frame batch flush/reset failed: {e}");
                    }
                }
            }
        }
        Ok(out)
    }

    /// Compiles an MSL source string into a [`MTLLibrary`] using this cache's [`MTL4Compiler`]
    /// and [`LANGUAGE_VERSION_4_1`] (`MTLLanguageVersion4_1` / macOS 27), falling back to
    /// Metal 4.0 / default language version on older macOS builds.
    pub fn library_from_source(&self, name: &str, source: &str) -> Result<Library, Error> {
        self.library_from_source_with_opts(name, source, false)
    }

    /// Like [`Self::library_from_source`], optionally forcing `MTLMathMode::Fast` and
    /// `MTLMathFloatingPointFunctions::Fast` on [`MTLCompileOptions`]. Inherits
    /// [`rtz_fp_conversion_enabled`] (`PORT_METAL41_RTZ_FP` / `PORT_METAL41`).
    pub fn library_from_source_with_opts(
        &self,
        name: &str,
        source: &str,
        fast_math: bool,
    ) -> Result<Library, Error> {
        self.library_from_source_with_compile_opts(
            name,
            source,
            SourceCompileOptions {
                fast_math,
                rtz_fp_conversion: rtz_fp_conversion_enabled(),
            },
        )
    }

    /// Compiles an MSL source string with explicit [`SourceCompileOptions`] (fast-math and
    /// Metal 4.1 RTZ narrowing float conversion rounding mode).
    pub fn library_from_source_with_compile_opts(
        &self,
        name: &str,
        source: &str,
        compile_opts: SourceCompileOptions,
    ) -> Result<Library, Error> {
        let try_compile = |ver: Option<MTLLanguageVersion>| -> Result<Library, Error> {
            let _permit = self.compile_limiter.acquire();
            retry_on_xpc_interrupt(name, XPC_RETRY_BACKOFFS_MS, || {
                objc2::rc::autoreleasepool(|_| {
                    let opts = MTLCompileOptions::new();
                    if let Some(v) = ver {
                        opts.setLanguageVersion(v);
                    }
                    if compile_opts.fast_math {
                        opts.setMathMode(MTLMathMode::Fast);
                        opts.setMathFloatingPointFunctions(MTLMathFloatingPointFunctions::Fast);
                    }
                    if compile_opts.rtz_fp_conversion && ver == Some(LANGUAGE_VERSION_4_1) {
                        unsafe {
                            let has_sel: objc2::runtime::Bool = objc2::msg_send![
                                &*opts,
                                respondsToSelector: objc2::sel!(setFloatingPointConversionRoundingMode:)
                            ];
                            if has_sel.as_bool() {
                                let _: () = objc2::msg_send![
                                    &*opts,
                                    setFloatingPointConversionRoundingMode: FP_CONVERSION_ROUNDING_RTZ
                                ];
                            }
                        }
                    }
                    let desc = MTL4LibraryDescriptor::new();
                    desc.setSource(Some(&NSString::from_str(source)));
                    desc.setOptions(Some(&opts));
                    if !name.is_empty() {
                        desc.setName(Some(&NSString::from_str(name)));
                    }
                    let _compile_guard = self.flush_core.compile_lock.read().unwrap();
                    let compiler = self.flush_core.compiler.read().unwrap();
                    guarded(name, || compiler.newLibraryWithDescriptor_error(&desc))
                        .and_then(|r| r.map_err(|e| ns_err(name, &e)))
                })
            })
        };
        match try_compile(Some(LANGUAGE_VERSION_4_1)) {
            Ok(lib) => Ok(lib),
            Err(e) if is_transient_xpc_interrupt(&e) => Err(e),
            Err(_) => match try_compile(Some(MTLLanguageVersion(4 << 16))) {
                Ok(lib) => Ok(lib),
                Err(e) if is_transient_xpc_interrupt(&e) => Err(e),
                Err(_) => try_compile(None),
            },
        }
    }

    /// Builds a [`MTLComputePipelineState`] for `function_name` in `lib` through [`MTL4Compiler`]
    /// and the archive cache.
    pub fn compute_pipeline_from_function(
        &self,
        key: Option<u128>,
        lib: &ProtocolObject<dyn MTLLibrary>,
        function_name: &str,
    ) -> Result<ComputePipeline, Error> {
        let fn_desc = MTL4LibraryFunctionDescriptor::new();
        fn_desc.setLibrary(Some(lib));
        fn_desc.setName(Some(&NSString::from_str(function_name)));
        let desc = MTL4ComputePipelineDescriptor::new();
        desc.setComputeFunctionDescriptor(Some(&fn_desc));
        desc.setLabel(Some(&NSString::from_str(function_name)));
        self.compute_pipeline(key, &desc)
    }

    /// `CreateComputePipelineState`. `key` enables in-memory dedup.
    pub fn compute_pipeline(&self, key: Option<u128>, desc: &MTL4ComputePipelineDescriptor) -> Result<ComputePipeline, Error> {
        self.build(
            KIND_COMPUTE,
            key,
            &self.compute,
            &self.in_flight_compute,
            |a| a.newComputePipelineStateWithDescriptor_error(desc),
            |c, o| c.newComputePipelineStateWithDescriptor_compilerTaskOptions_error(desc, o),
            "compute pipeline",
        )
    }

    /// `CreateGraphicsPipelineState`. `desc` is a render, mesh or tile pipeline descriptor.
    /// `key` enables in-memory dedup.
    pub fn render_pipeline(&self, key: Option<u128>, desc: &MTL4PipelineDescriptor) -> Result<RenderPipeline, Error> {
        self.build(
            KIND_RENDER,
            key,
            &self.render,
            &self.in_flight_render,
            |a| a.newRenderPipelineStateWithDescriptor_error(desc),
            |c, o| c.newRenderPipelineStateWithDescriptor_compilerTaskOptions_error(desc, o),
            "render pipeline",
        )
    }

    fn ensure_bg_flusher(&self) {
        let mut slot = self.bg_thread.lock().unwrap();
        if slot.is_some() {
            return;
        }
        let core = self.flush_core.clone();
        match std::thread::Builder::new()
            .name("port-pso-flush".into())
            .spawn(move || bg_flush_worker(core))
        {
            Ok(h) => *slot = Some(h),
            Err(e) => {
                log::warn!("pso cache: failed to spawn background flush thread: {e}");
            }
        }
    }

    /// Schedules an asynchronous mid-session incremental flush on a background utility
    /// thread so `Present` never blocks on `serializeAsArchiveAndFlushToURL` or
    /// `compile_lock.write()`. Returns immediately in O(1) when the cache is clean
    /// (warm cache) or in compacting mode.
    pub fn flush_incremental_async(&self) {
        let core = &self.flush_core;
        if core.device_dir.is_none()
            || core.compacting
            || !core.dirty.load(Ordering::Acquire)
        {
            return;
        }
        self.ensure_bg_flusher();
        let mut st = core.bg_mu.lock().unwrap();
        if st.shutdown {
            return;
        }
        st.requested = true;
        core.bg_cv.notify_one();
    }

    /// Waits for any in-flight background flush to finish cleanly (preventing torn `tmp-*`
    /// files) and blocks new background flushes for the duration of `f`.
    fn with_bg_flusher_quiesced<R>(&self, f: impl FnOnce() -> R) -> R {
        let core = &self.flush_core;
        {
            let mut st = core.bg_mu.lock().unwrap();
            st.sync_waiters += 1;
            st.requested = false;
            while st.in_flight {
                st = core.bg_cv.wait(st).unwrap();
            }
        }
        struct Release<'a>(&'a FlushCore);
        impl Drop for Release<'_> {
            fn drop(&mut self) {
                let mut st = self.0.bg_mu.lock().unwrap();
                st.sync_waiters = st.sync_waiters.saturating_sub(1);
                self.0.bg_cv.notify_all();
            }
        }
        let _rel = Release(core);
        f()
    }

    /// Flushes newly compiled pipelines mid-session only when at least `min_compiles` cold
    /// compilations have accumulated since the last flush. Never deletes pre-existing
    /// archives during a compacting session (only [`Self::flush`] on shutdown may compact).
    pub fn flush_if_dirty_compiles(&self, min_compiles: u64) -> Result<Option<PathBuf>, Error> {
        if self.flush_core.compacting
            || self.flush_core.compiles_since_flush.load(Ordering::Relaxed) < min_compiles
        {
            return Ok(None);
        }
        self.flush_incremental()
    }

    /// Mid-session flush: persists newly compiled pipelines to a new incremental `s-*.mtl4a`
    /// archive without deleting any existing archives (no-op in compacting mode, so a partial
    /// mid-session flush at the main menu can never wipe level archives on disk).
    pub fn flush_incremental(&self) -> Result<Option<PathBuf>, Error> {
        if self.flush_core.compacting {
            return Ok(None);
        }
        self.with_bg_flusher_quiesced(|| self.flush_core.flush_inner(false, false))
    }

    /// Mid-session flush + `MTL4Compiler` recreation: writes any newly compiled pipelines to
    /// disk, then replaces `MTL4Compiler` + `MTL4PipelineDataSetSerializer` inside an
    /// autorelease pool so Apple's internal LLVM/AGX compiler scratch arenas are returned to
    /// XNU immediately (critical during cold level-load PSO bursts on 6 GiB iOS Jetsam).
    pub fn flush_and_reset_compiler(&self) -> Result<Option<PathBuf>, Error> {
        if self.flush_core.compacting {
            return Ok(None);
        }
        self.with_bg_flusher_quiesced(|| self.flush_core.flush_inner(true, false))
    }

    /// Like [`Self::flush_and_reset_compiler`], but skips compiler recreation if another
    /// thread already flushed this batch (avoids duplicate resets when multiple workers cross
    /// the batch threshold simultaneously).
    fn flush_and_reset_compiler_if_dirty(&self) -> Result<Option<PathBuf>, Error> {
        if self.flush_core.compacting {
            return Ok(None);
        }
        self.with_bg_flusher_quiesced(|| self.flush_core.flush_inner(true, true))
    }

    /// Final flush (called at shutdown / [`Drop`]): drains any in-flight background archive
    /// write so no torn `tmp-*` files remain, writes newly compiled pipelines as a new
    /// archive + `.idx` sidecar, and if this session was compacting, removes superseded archives.
    pub fn flush(&self) -> Result<Option<PathBuf>, Error> {
        self.with_bg_flusher_quiesced(|| self.flush_core.flush_inner(false, false))
    }

    pub fn stats(&self) -> PipelineCacheStats {
        self.flush_core.stats()
    }
}

fn bg_flush_worker(core: Arc<FlushCore>) {
    loop {
        {
            let mut st = core.bg_mu.lock().unwrap();
            while (!st.requested || st.sync_waiters > 0) && !st.shutdown {
                st = core.bg_cv.wait(st).unwrap();
            }
            if st.shutdown {
                st.requested = false;
                st.in_flight = false;
                core.bg_cv.notify_all();
                return;
            }
            st.requested = false;
            st.in_flight = true;
        }
        match core.flush_inner(false, false) {
            Ok(Some(p)) => log::info!("pso cache (bg): wrote {}", p.display()),
            Ok(None) => {}
            Err(e) => log::warn!("pso cache (bg): flush failed: {e}"),
        }
        {
            let mut st = core.bg_mu.lock().unwrap();
            st.in_flight = false;
            core.bg_cv.notify_all();
        }
    }
}

impl FlushCore {
    fn stats(&self) -> PipelineCacheStats {
        let c = &self.counters;
        let g = |a: &AtomicU64| a.load(Ordering::Relaxed);
        PipelineCacheStats {
            memory_hits: g(&c.memory_hits),
            archive_hits: g(&c.archive_hits),
            compiles: g(&c.compiles),
            failures: g(&c.failures),
            archives_loaded: g(&c.archives_loaded),
            archives_written: g(&c.archives_written),
            archives_rejected: g(&c.archives_rejected),
            lookup_nanos: g(&c.lookup_nanos),
            compile_nanos: g(&c.compile_nanos),
        }
    }

    fn flush_inner(&self, reset_compiler: bool, reset_only_if_wrote: bool) -> Result<Option<PathBuf>, Error> {
        let Some(dir) = &self.device_dir else {
            return Ok(None);
        };
        if !reset_compiler
            && ((self.compacting && self.flushed.load(Ordering::Acquire))
                || !self.dirty.load(Ordering::Acquire))
        {
            return Ok(None);
        }
        let _compile_guard = self.compile_lock.write().unwrap();
        let mut ser_slot = self.serializer.lock().unwrap();
        let wrote = if let Some(ser) = ser_slot.as_ref() {
            if (self.compacting && self.flushed.load(Ordering::Acquire))
                || !self.dirty.swap(false, Ordering::AcqRel)
            {
                None
            } else {
                objc2::rc::autoreleasepool(|_| -> Result<Option<PathBuf>, Error> {
                    static FLUSH_SEQ: AtomicU64 = AtomicU64::new(0);
                    let seq = FLUSH_SEQ.fetch_add(1, Ordering::Relaxed);
                    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
                    let name = format!(
                        "{}{stamp:x}-{}-{seq}.{ARCHIVE_EXT}",
                        if self.compacting { "compact-" } else { "s-" },
                        std::process::id()
                    );
                    let tmp = dir.join(format!("{TMP_PREFIX}{name}"));
                    let dst = dir.join(&name);
                    let r = guarded("serializeAsArchiveAndFlushToURL", || {
                        ser.0.serializeAsArchiveAndFlushToURL_error(&url(&tmp))
                    })
                    .and_then(|r| r.map_err(|e| ns_err("serializeAsArchiveAndFlushToURL", &e)));
                    if let Err(e) = r {
                        self.dirty.store(true, Ordering::Release);
                        let _ = std::fs::remove_file(&tmp);
                        return Err(e);
                    }
                    // `serializeAsArchiveAndFlushToURL` drains the serializer's captured
                    // dataset, so drain `serializer_keys` into this archive's `.idx` only
                    // after serialization succeeds (preserving keys on error for retry).
                    let keys_snapshot: Vec<(u8, u128)> = {
                        let mut ks = self.serializer_keys.lock().unwrap();
                        ks.drain().collect()
                    };
                    self.compiles_since_flush.store(0, Ordering::Relaxed);
                    self.flushed.store(true, Ordering::Release);
                    std::fs::rename(&tmp, &dst)
                        .map_err(|e| Error::Io(format!("pso cache: rename {}: {e}", dst.display())))?;
                    if let Err(e) = write_archive_index(&dst, &keys_snapshot) {
                        log::warn!("pso cache: failed to write archive index for {}: {e}", dst.display());
                    }
                    self.counters.archives_written.fetch_add(1, Ordering::Relaxed);
                    if self.compacting {
                        for p in &self.archive_paths {
                            let _ = std::fs::remove_file(p);
                            let _ = std::fs::remove_file(idx_path(p));
                        }
                    }
                    let st = self.stats();
                    log::info!(
                        "pso cache: wrote {} (keys={} archive_hits={} memory_hits={} compiles={})",
                        dst.display(),
                        keys_snapshot.len(),
                        st.archive_hits,
                        st.memory_hits,
                        st.compiles
                    );
                    Ok(Some(dst))
                })?
            }
        } else {
            None
        };

        if reset_compiler && !self.compacting && (!reset_only_if_wrote || wrote.is_some()) {
            objc2::rc::autoreleasepool(|_| {
                match create_compiler_and_serializer(&self.device, true) {
                    Ok((new_compiler, new_ser)) => {
                        let mut comp_slot = self.compiler.write().unwrap();
                        let old_compiler = std::mem::replace(&mut *comp_slot, new_compiler);
                        let old_ser = std::mem::replace(&mut *ser_slot, new_ser);
                        self.serializer_keys.lock().unwrap().clear();
                        drop(old_compiler);
                        drop(old_ser);
                    }
                    Err(e) => {
                        log::warn!("pso cache: failed to recreate MTL4Compiler after flush: {e}");
                    }
                }
            });
        }
        Ok(wrote)
    }
}

impl Drop for PipelineCache {
    fn drop(&mut self) {
        {
            let mut st = self.flush_core.bg_mu.lock().unwrap();
            st.shutdown = true;
            st.requested = false;
            self.flush_core.bg_cv.notify_all();
        }
        if let Some(h) = self.bg_thread.lock().unwrap().take() {
            let _ = h.join();
        }
        if self.flush_on_drop {
            if let Err(e) = self.flush() {
                log::warn!("pso cache: flush on drop failed: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use objc2_metal::MTLBuffer;

    #[test]
    fn incremental_flush_never_wipes_archives_and_reset_compiler_succeeds() {
        let Some(dev) = objc2_metal::MTLCreateSystemDefaultDevice() else {
            return;
        };
        if !dev.supportsFamily(objc2_metal::MTLGPUFamily::Metal4) {
            return;
        }
        let dir = std::env::temp_dir().join(format!("pso-cache-reset-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = PipelineCacheConfig {
            dir: Some(dir.clone()),
            max_archives: 64,
            flush_on_drop: false,
        };
        let cache = PipelineCache::open(&dev, &cfg).expect("open pso cache");
        let lib = cache
            .library_from_source(
                "noop",
                "kernel void k_noop(device uint *out [[buffer(0)]]) { out[0] = 1; }",
            )
            .expect("compile msl");
        let _pso = cache
            .compute_pipeline_from_function(Some(0x1111), &lib, "k_noop")
            .expect("compile pso");
        let first = cache.flush_and_reset_compiler().expect("flush+reset");
        let first_path = first.expect("expected archive written before compiler reset");
        // Compiling a second pipeline after resetting MTL4Compiler must still work cleanly.
        let _pso2 = cache
            .compute_pipeline_from_function(Some(0x2222), &lib, "k_noop")
            .expect("compile pso after reset");
        let second = cache.flush_incremental().expect("incremental flush");
        let second_path = second.expect("expected second incremental archive");
        drop(cache);

        // Re-open in compacting mode (2 archives > max_archives=1): mid-session incremental
        // flushes MUST NOT delete pre-existing archives on disk.
        let compact_cfg = PipelineCacheConfig {
            dir: Some(dir.clone()),
            max_archives: 1,
            flush_on_drop: false,
        };
        let compact_cache = PipelineCache::open(&dev, &compact_cfg).expect("open compacting cache");
        assert!(compact_cache.is_compacting());
        let lib2 = compact_cache
            .library_from_source(
                "noop2",
                "kernel void k_noop2(device uint *out [[buffer(0)]]) { out[0] = 2; }",
            )
            .expect("compile msl2");
        let _pso3 = compact_cache
            .compute_pipeline_from_function(Some(0x3333), &lib2, "k_noop2")
            .expect("compile pso in compacting mode");
        assert!(compact_cache.flush_incremental().expect("incremental in compact").is_none());
        assert!(compact_cache.flush_if_dirty_compiles(1).expect("burst in compact").is_none());
        assert!(first_path.exists(), "mid-session flush must never delete first archive");
        assert!(second_path.exists(), "mid-session flush must never delete second archive");
        drop(compact_cache);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn metal41_rtz_fp_conversion_option_saturates_f16_overflow() {
        use objc2_metal::{
            MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
            MTLResourceOptions, MTLSize,
        };
        let Some(dev) = objc2_metal::MTLCreateSystemDefaultDevice() else {
            return;
        };
        if !dev.supportsFamily(objc2_metal::MTLGPUFamily::Metal4) {
            return;
        }
        // Only assert RTZ rounding behavior when macOS 27+ exposes the selector.
        let probe_opts = MTLCompileOptions::new();
        let has_rtz: objc2::runtime::Bool = unsafe {
            objc2::msg_send![
                &*probe_opts,
                respondsToSelector: objc2::sel!(setFloatingPointConversionRoundingMode:)
            ]
        };
        if !has_rtz.as_bool() {
            return;
        }
        let cfg = PipelineCacheConfig { dir: None, max_archives: 8, flush_on_drop: false };
        let cache = PipelineCache::open(&dev, &cfg).expect("open memory-only pso cache");
        let src = r#"
#include <metal_stdlib>
using namespace metal;
kernel void cvt_f16(device const float* inp [[buffer(0)]], device half* outp [[buffer(1)]], uint i [[thread_position_in_grid]]) {
    outp[i] = half(inp[i]);
}
"#;
        let run = |rtz: bool| -> [u16; 4] {
            let lib = cache
                .library_from_source_with_compile_opts(
                    if rtz { "cvt_rtz" } else { "cvt_rtne" },
                    src,
                    SourceCompileOptions { fast_math: false, rtz_fp_conversion: rtz },
                )
                .expect("compile msl4.1 cvt");
            let pso = cache
                .compute_pipeline_from_function(None, &lib, "cvt_f16")
                .expect("build pso");
            let in_buf = dev
                .newBufferWithLength_options(16, MTLResourceOptions::StorageModeShared)
                .unwrap();
            let out_buf = dev
                .newBufferWithLength_options(8, MTLResourceOptions::StorageModeShared)
                .unwrap();
            let vals: [f32; 4] = [70000.0, 65520.0, 65504.0, 1.0];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    vals.as_ptr(),
                    in_buf.contents().as_ptr() as *mut f32,
                    4,
                );
            }
            let cq = dev.newCommandQueue().unwrap();
            let cb = cq.commandBuffer().unwrap();
            let ce = cb.computeCommandEncoder().unwrap();
            ce.setComputePipelineState(&pso);
            unsafe {
                ce.setBuffer_offset_atIndex(Some(&in_buf), 0, 0);
                ce.setBuffer_offset_atIndex(Some(&out_buf), 0, 1);
            }
            ce.dispatchThreads_threadsPerThreadgroup(
                MTLSize { width: 4, height: 1, depth: 1 },
                MTLSize { width: 4, height: 1, depth: 1 },
            );
            ce.endEncoding();
            cb.commit();
            cb.waitUntilCompleted();
            unsafe { *(out_buf.contents().as_ptr() as *const [u16; 4]) }
        };
        let rtne = run(false);
        let rtz = run(true);
        assert_eq!(rtne[0], 0x7c00, "RTNE 70000.0f -> +Inf");
        assert_eq!(rtne[1], 0x7c00, "RTNE 65520.0f -> +Inf");
        assert_eq!(rtz[0], 0x7bff, "Metal 4.1 RTZ 70000.0f -> 65504.0");
        assert_eq!(rtz[1], 0x7bff, "Metal 4.1 RTZ 65520.0f -> 65504.0");
        assert_eq!(rtz[2], 0x7bff, "65504.0f -> 65504.0");
        assert_eq!(rtz[3], 0x3c00, "1.0f -> 1.0");
    }

    #[test]
    fn key_to_archive_index_avoids_linear_probing_on_hits_and_misses() {
        let Some(dev) = objc2_metal::MTLCreateSystemDefaultDevice() else {
            return;
        };
        if !dev.supportsFamily(objc2_metal::MTLGPUFamily::Metal4) {
            return;
        }
        let dir = std::env::temp_dir().join(format!("pso-cache-idx-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = PipelineCacheConfig {
            dir: Some(dir.clone()),
            max_archives: 64,
            flush_on_drop: false,
        };

        // Write 4 separate archives, each holding one unique compute shader key.
        let sources = [
            ("k0", "kernel void k0(device uint *o [[buffer(0)]]) { o[0] = 10; }"),
            ("k1", "kernel void k1(device uint *o [[buffer(0)]]) { o[0] = 20; }"),
            ("k2", "kernel void k2(device uint *o [[buffer(0)]]) { o[0] = 30; }"),
            ("k3", "kernel void k3(device uint *o [[buffer(0)]]) { o[0] = 40; }"),
        ];
        {
            let cache = PipelineCache::open(&dev, &cfg).expect("open cold cache");
            for (i, (name, src)) in sources.iter().enumerate() {
                let lib = cache.library_from_source(name, src).expect("compile msl");
                let _pso = cache
                    .compute_pipeline_from_function(Some(0xA000 + i as u128), &lib, name)
                    .expect("compile pso");
                let arch = cache
                    .flush_and_reset_compiler()
                    .expect("flush+reset")
                    .expect("archive written");
                assert!(idx_path(&arch).is_file(), "sidecar .idx must be written alongside archive");
            }
        }

        // Re-open warm cache: all 4 archives must be indexed.
        let warm = PipelineCache::open(&dev, &cfg).expect("open warm cache");
        assert_eq!(warm.stats().archives_loaded, 4);
        assert!(warm.unindexed_archives.is_empty(), "all 4 archives must be indexed");

        // Hit key in the LAST archive (#3): must do exactly 1 archive probe (O(1)), not 4.
        let lib3 = warm.library_from_source("k3", sources[3].1).expect("msl k3");
        let _pso3 = warm
            .compute_pipeline_from_function(Some(0xA003), &lib3, "k3")
            .expect("hit k3");
        assert_eq!(warm.stats().archive_hits, 1);
        assert_eq!(warm.stats().compiles, 0);
        assert_eq!(warm.archive_probes(), 1, "indexed hit must probe only its owning archive");

        // Miss on a brand-new key: must perform 0 archive probes (zero linear probing!).
        let probes_before_miss = warm.archive_probes();
        let lib_miss = warm
            .library_from_source("k_miss", "kernel void k_miss(device uint *o [[buffer(0)]]) { o[0] = 99; }")
            .expect("msl miss");
        let _pso_miss = warm
            .compute_pipeline_from_function(Some(0xBEEF), &lib_miss, "k_miss")
            .expect("compile miss");
        assert_eq!(warm.stats().compiles, 1);
        assert_eq!(
            warm.archive_probes(),
            probes_before_miss,
            "indexed miss must NOT linearly probe any MTL4Archive"
        );

        drop(warm);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn in_flight_compile_deduplication_coalesces_concurrent_same_key_requests() {
        let Some(dev) = objc2_metal::MTLCreateSystemDefaultDevice() else {
            return;
        };
        if !dev.supportsFamily(objc2_metal::MTLGPUFamily::Metal4) {
            return;
        }
        let cfg = PipelineCacheConfig { dir: None, max_archives: 8, flush_on_drop: false };
        let cache = Arc::new(PipelineCache::open(&dev, &cfg).expect("open memory cache"));
        let lib = cache
            .library_from_source(
                "dedup",
                "kernel void k_dedup(device uint *o [[buffer(0)]], uint i [[thread_position_in_grid]]) { o[i] = i * 7 + 3; }",
            )
            .expect("compile msl");

        let threads = 6usize;
        let barrier = Arc::new(std::sync::Barrier::new(threads));
        std::thread::scope(|scope| {
            for _ in 0..threads {
                let c = cache.clone();
                let b = barrier.clone();
                let l = lib.clone();
                scope.spawn(move || {
                    b.wait();
                    let _pso = c
                        .compute_pipeline_from_function(Some(0xDEAD_C0DE), &l, "k_dedup")
                        .expect("dedup compile");
                });
            }
        });
        let st = cache.stats();
        assert_eq!(st.compiles, 1, "concurrent same-key compiles must coalesce to 1 compile: {st:?}");
        assert_eq!(st.memory_hits, (threads - 1) as u64, "remaining threads must hit memo: {st:?}");
    }

    #[test]
    fn background_async_flush_coordinates_cleanly_with_final_flush_and_leaves_no_tmp_files() {
        let Some(dev) = objc2_metal::MTLCreateSystemDefaultDevice() else {
            return;
        };
        if !dev.supportsFamily(objc2_metal::MTLGPUFamily::Metal4) {
            return;
        }
        let dir = std::env::temp_dir().join(format!("pso-cache-bg-flush-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let cfg = PipelineCacheConfig {
            dir: Some(dir.clone()),
            max_archives: 64,
            flush_on_drop: true,
        };
        let dev_dir;
        {
            let cache = PipelineCache::open(&dev, &cfg).expect("open bg cache");
            dev_dir = cache.dir().expect("device dir").to_path_buf();
            // Clean cache async flush must be an immediate no-op without spawning/writing.
            cache.flush_incremental_async();
            assert_eq!(cache.stats().archives_written, 0);

            let lib1 = cache
                .library_from_source("bg1", "kernel void k_bg1(device uint *o [[buffer(0)]]) { o[0] = 1; }")
                .expect("msl1");
            let _p1 = cache
                .compute_pipeline_from_function(Some(0xB1), &lib1, "k_bg1")
                .expect("pso1");
            // Kick off background flush, immediately compile another PSO and run ExitProcess final flush().
            cache.flush_incremental_async();
            let lib2 = cache
                .library_from_source("bg2", "kernel void k_bg2(device uint *o [[buffer(0)]]) { o[0] = 2; }")
                .expect("msl2");
            let _p2 = cache
                .compute_pipeline_from_function(Some(0xB2), &lib2, "k_bg2")
                .expect("pso2");
            let _ = cache.flush().expect("final flush");
            assert!(cache.stats().archives_written >= 1);
        }

        // Verify zero leftover `tmp-*` files exist after shutdown.
        for entry in std::fs::read_dir(&dev_dir).expect("read dev dir").flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            assert!(!name.starts_with(TMP_PREFIX), "leftover temp file after flush: {name}");
        }

        // Re-open warm cache and confirm BOTH pipelines hit archives with 0 compiles.
        let warm = PipelineCache::open(&dev, &cfg).expect("reopen warm cache");
        let lib1 = warm
            .library_from_source("bg1", "kernel void k_bg1(device uint *o [[buffer(0)]]) { o[0] = 1; }")
            .expect("msl1");
        let lib2 = warm
            .library_from_source("bg2", "kernel void k_bg2(device uint *o [[buffer(0)]]) { o[0] = 2; }")
            .expect("msl2");
        let _p1 = warm.compute_pipeline_from_function(Some(0xB1), &lib1, "k_bg1").expect("hit1");
        let _p2 = warm.compute_pipeline_from_function(Some(0xB2), &lib2, "k_bg2").expect("hit2");
        let st = warm.stats();
        assert_eq!((st.archive_hits, st.compiles), (2, 0), "{st:?}");
        drop(warm);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn xpc_interrupt_retry_recovers_on_transient_disconnect_and_bounds_attempts() {
        let xpc_err = || {
            Error::Metal(
                "compute pipeline: Compilation failed due to an interrupted connection: XPC_ERROR_CONNECTION_INTERRUPTED. This error occurred after multiple retries.".into(),
            )
        };
        assert!(is_transient_xpc_interrupt(&xpc_err()));
        assert!(!is_transient_xpc_interrupt(&Error::Metal("compute pipeline: syntax error".into())));
        assert!(!is_transient_xpc_interrupt(&Error::Metal(
            "compute pipeline: Objective-C exception: interrupted connection".into()
        )));
        assert!(!is_transient_xpc_interrupt(&Error::Io("XPC_ERROR_CONNECTION_INTERRUPTED".into())));

        // 1) Recovers when a transient XPC disconnect clears on retry.
        let mut calls = 0usize;
        let ok = retry_on_xpc_interrupt("test", &[0, 0], || {
            calls += 1;
            if calls < 3 {
                Err(xpc_err())
            } else {
                Ok(42u32)
            }
        });
        assert_eq!(ok.unwrap(), 42);
        assert_eq!(calls, 3);

        // 2) Bounds retries when the XPC disconnect persists.
        let mut calls = 0usize;
        let err = retry_on_xpc_interrupt::<u32>("test", &[0, 0], || {
            calls += 1;
            Err(xpc_err())
        });
        assert!(err.is_err());
        assert_eq!(calls, 3, "1 initial attempt + 2 bounded retries");

        // 3) Does not retry non-XPC compiler errors.
        let mut calls = 0usize;
        let err = retry_on_xpc_interrupt::<u32>("test", &[0, 0], || {
            calls += 1;
            Err(Error::Metal("compute pipeline: invalid shader".into()))
        });
        assert!(err.is_err());
        assert_eq!(calls, 1, "non-transient compiler errors must fail immediately");
    }
}
