//! Thread-safe two-level (memory + disk) cache of converted shaders with concurrent-compile
//! de-duplication, and the [`ShaderPipeline`] front end the D3D12 layer uses.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::Instant;

use crate::compiler::{compile_uncached, stage_in_shader, CompileOptions, CompileRequest, CompiledShader, InputElement};
use crate::ffi;
use crate::hash::{hash128, KeyHasher};
use crate::reflection::ShaderStage;
use crate::root_signature::RootSignature;
use crate::wire::{self, Reader, Writer};
use crate::Error;

static GLOBAL_PIPELINE: OnceLock<Result<ShaderPipeline, Error>> = OnceLock::new();

const FILE_MAGIC: &[u8; 8] = b"AW2SHC02";

/// Cache configuration.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Disk cache root (`None` = memory only).
    pub disk_dir: Option<PathBuf>,
    /// Upper bound on metallib bytes kept in memory (entries beyond it are served from disk).
    pub memory_limit_bytes: u64,
}

impl CacheConfig {
    /// `PORT_SHADER_CACHE` / `AW2_SHADER_CACHE` = directory, or `off`/`0`/empty to disable the disk cache.
    /// Default: `~/Translator/ShaderCache` on macOS, `~/Library/Caches/port-factory/ShaderCache` on iOS
    /// (where `$HOME` itself is read-only), or `$TRANSLATOR_HOME/ShaderCache`.
    pub fn from_env() -> Self {
        let disk_dir = match std::env::var_os("PORT_SHADER_CACHE")
            .or_else(|| std::env::var_os("AW2_SHADER_CACHE"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => None,
            Some(v) => Some(PathBuf::from(v)),
            None => std::env::var_os("TRANSLATOR_HOME")
                .or_else(|| std::env::var_os("AW2_DRIVE_C"))
                .map(|p| Path::new(&p).join("ShaderCache"))
                .or_else(|| {
                    std::env::var_os("HOME").map(|h| {
                        if cfg!(target_os = "ios") {
                            Path::new(&h).join("Library/Caches/port-factory/ShaderCache")
                        } else {
                            Path::new(&h).join("Translator/ShaderCache")
                        }
                    })
                }),
        };
        let default_mem_mb: u64 = if cfg!(target_os = "ios") { 32 } else { 512 };
        let mem_mb = std::env::var("PORT_SHADER_CACHE_MEM_MB")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(default_mem_mb);
        Self { disk_dir, memory_limit_bytes: mem_mb << 20 }
    }
    pub fn memory_only() -> Self {
        Self { disk_dir: None, memory_limit_bytes: 512 << 20 }
    }
}

/// Snapshot of cache counters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub memory_hits: u64,
    pub disk_hits: u64,
    /// Conversions actually run.
    pub compiles: u64,
    pub compile_failures: u64,
    /// Requests served from the negative (known-failing) cache.
    pub negative_hits: u64,
    /// Requests that waited on another thread's in-flight compile of the same key.
    pub dedup_waits: u64,
    pub disk_bytes_read: u64,
    pub disk_bytes_written: u64,
    pub disk_errors: u64,
    pub compile_nanos: u64,
}

#[derive(Default)]
struct Counters {
    memory_hits: AtomicU64,
    disk_hits: AtomicU64,
    compiles: AtomicU64,
    compile_failures: AtomicU64,
    negative_hits: AtomicU64,
    dedup_waits: AtomicU64,
    disk_bytes_read: AtomicU64,
    disk_bytes_written: AtomicU64,
    disk_errors: AtomicU64,
    compile_nanos: AtomicU64,
}

type Cell = Arc<OnceLock<Result<Arc<CompiledShader>, Error>>>;

/// Memory + disk cache keyed by a 128-bit content key.
pub struct ShaderCache {
    config: CacheConfig,
    /// Subdirectory for the loaded converter build (old builds' entries are simply ignored).
    namespace: String,
    /// Optional read-only fallback directory (e.g. bundled `Contents/Resources/ShaderCache/<ns>`).
    fallback_dir: Option<PathBuf>,
    memory: Mutex<HashMap<u128, Arc<CompiledShader>>>,
    memory_bytes: AtomicU64,
    inflight: Mutex<HashMap<u128, Cell>>,
    failed: Mutex<HashMap<u128, Error>>,
    /// Transient failures seen per key (see [`Self::get_or_compile`]).
    transient: Mutex<HashMap<u128, u32>>,
    counters: Counters,
}

fn resolve_namespace_dir(root: &Path, namespace: &str) -> Option<PathBuf> {
    let exact = root.join(namespace);
    if exact.is_dir() {
        return Some(exact);
    }
    let mut prefixes = Vec::new();
    if let Some((prefix, _suffix)) = namespace.rsplit_once('-') {
        prefixes.push(format!("{prefix}-"));
    }
    // `irX.Y.Z-uuid-<hex>` should also match legacy `irX.Y.Z-<size>-<mtime>` caches built by
    // the same Metal Shader Converter version (e.g. pre-UUID prewarmed caches).
    if let Some((ver_prefix, _rest)) = namespace.split_once("-uuid-") {
        let ver_dash = format!("{ver_prefix}-");
        if !prefixes.contains(&ver_dash) {
            prefixes.push(ver_dash);
        }
    }
    for prefix_dash in prefixes {
        if let Ok(entries) = std::fs::read_dir(root) {
            let mut matches: Vec<PathBuf> = entries
                .flatten()
                .filter_map(|entry| {
                    let name = entry.file_name();
                    let name = name.to_str()?;
                    (name.starts_with(&prefix_dash) && entry.path().is_dir()).then(|| entry.path())
                })
                .collect();
            matches.sort();
            if let Some(first) = matches.into_iter().next() {
                return Some(first);
            }
        }
    }
    None
}

fn fallback_cache_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(v) = std::env::var_os("PORT_SHADER_CACHE_FALLBACK").filter(|v| !v.is_empty()) {
        roots.push(PathBuf::from(v));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            // Flat iOS `.app` bundle layout (`<App>.app/ShaderCache`).
            roots.push(parent.join("ShaderCache"));
            // macOS `.app` bundle layout (`<App>.app/Contents/Resources/ShaderCache`).
            roots.push(parent.join("../Resources/ShaderCache"));
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(Path::new(&home).join("Library/Caches/port-factory/shaders"));
    }
    roots
}

/// Scans a cache root for a precompiled converter namespace subdirectory (`ir*`).
fn find_precompiled_namespace_in(root: &Path) -> Option<String> {
    let entries = std::fs::read_dir(root).ok()?;
    let mut ir_dirs: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name();
        let Some(s) = name.to_str() else {
            continue;
        };
        if s.starts_with("ir") {
            ir_dirs.push(s.to_string());
        }
    }
    ir_dirs.sort();
    ir_dirs.pop()
}

/// Discovers the best precompiled namespace for [`BackendKind::CacheOnly`]:
/// 1. Explicit `PORT_SHADER_FINGERPRINT` / `AW2_SHADER_FINGERPRINT`
/// 2. Any `ir*` converter directory in `config.disk_dir`
/// 3. Any `ir*` converter directory in [`fallback_cache_roots`]
fn discover_precompiled_namespace(config: &CacheConfig) -> Option<String> {
    if let Some(fp) = std::env::var("PORT_SHADER_FINGERPRINT")
        .or_else(|_| std::env::var("AW2_SHADER_FINGERPRINT"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        return Some(fp);
    }
    if let Some(dir) = config.disk_dir.as_deref() {
        if let Some(ns) = find_precompiled_namespace_in(dir) {
            return Some(ns);
        }
    }
    for root in fallback_cache_roots() {
        if let Some(ns) = find_precompiled_namespace_in(&root) {
            return Some(ns);
        }
    }
    None
}

fn find_bundled_cache_dir(namespace: &str, primary_root: Option<&Path>) -> Option<PathBuf> {
    let primary_canon = primary_root.and_then(|p| std::fs::canonicalize(p).ok());
    for base in fallback_cache_roots() {
        if let Some(dir) = resolve_namespace_dir(&base, namespace) {
            if let Some(pc) = &primary_canon {
                if let Ok(bc) = std::fs::canonicalize(&base) {
                    if &bc == pc {
                        continue;
                    }
                }
            }
            return Some(dir);
        }
    }
    None
}

impl ShaderCache {
    pub fn new(config: CacheConfig, converter_fingerprint: &str) -> Self {
        let raw_ns = converter_fingerprint.replace(['/', ' '], "_");
        // Resolve against an existing converter directory in either the primary writable root
        // or a read-only bundled fallback root (e.g. `Contents/Resources/ShaderCache`) so
        // prewarmed caches from older mtime-keyed or bundled builds stay warm with zero misses.
        let resolved_ns = config
            .disk_dir
            .as_deref()
            .and_then(|d| resolve_namespace_dir(d, &raw_ns))
            .or_else(|| {
                fallback_cache_roots()
                    .into_iter()
                    .find_map(|base| resolve_namespace_dir(&base, &raw_ns))
            })
            .and_then(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
            .unwrap_or(raw_ns);
        let fallback_dir = find_bundled_cache_dir(&resolved_ns, config.disk_dir.as_deref());
        Self {
            config,
            namespace: resolved_ns,
            fallback_dir,
            memory: Mutex::default(),
            memory_bytes: AtomicU64::new(0),
            inflight: Mutex::default(),
            failed: Mutex::default(),
            transient: Mutex::default(),
            counters: Counters::default(),
        }
    }

    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Resolved cache namespace (matched to an existing primary or fallback converter directory
    /// when one shares the converter version+size prefix).
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Read-only seed/fallback directory for this converter namespace, if present.
    pub fn fallback_dir(&self) -> Option<&Path> {
        self.fallback_dir.as_deref()
    }

    /// Path of the disk entry for `key` (if the disk cache is enabled).
    pub fn entry_path(&self, key: u128) -> Option<PathBuf> {
        let hex = format!("{key:032x}");
        self.config.disk_dir.as_ref().map(|d| d.join(&self.namespace).join(&hex[..2]).join(format!("{hex}.aw2s")))
    }

    fn fallback_entry_path(&self, key: u128) -> Option<PathBuf> {
        let hex = format!("{key:032x}");
        self.fallback_dir.as_ref().map(|d| d.join(&hex[..2]).join(format!("{hex}.aw2s")))
    }

    /// True if `key` is already cached in memory, in the primary disk cache, or in the
    /// read-only fallback cache.
    pub fn has_cached_entry(&self, key: u128) -> bool {
        self.memory.lock().unwrap().contains_key(&key)
            || self.entry_path(key).is_some_and(|p| p.exists())
            || self.fallback_entry_path(key).is_some_and(|p| p.exists())
    }

    pub fn stats(&self) -> CacheStats {
        let c = &self.counters;
        CacheStats {
            memory_hits: c.memory_hits.load(Relaxed),
            disk_hits: c.disk_hits.load(Relaxed),
            compiles: c.compiles.load(Relaxed),
            compile_failures: c.compile_failures.load(Relaxed),
            negative_hits: c.negative_hits.load(Relaxed),
            dedup_waits: c.dedup_waits.load(Relaxed),
            disk_bytes_read: c.disk_bytes_read.load(Relaxed),
            disk_bytes_written: c.disk_bytes_written.load(Relaxed),
            disk_errors: c.disk_errors.load(Relaxed),
            compile_nanos: c.compile_nanos.load(Relaxed),
        }
    }

    /// Drops all in-memory entries and remembered failures (disk is untouched).
    pub fn clear_memory(&self) {
        self.memory.lock().unwrap().clear();
        self.memory_bytes.store(0, Relaxed);
        self.failed.lock().unwrap().clear();
        self.transient.lock().unwrap().clear();
    }

    /// Returns the entry for `key`, loading it from disk or running `compile` (exactly once per
    /// key across concurrent callers).
    pub fn get_or_compile(
        &self,
        key: u128,
        compile: impl FnOnce() -> Result<CompiledShader, Error>,
    ) -> Result<Arc<CompiledShader>, Error> {
        if let Some(hit) = self.memory.lock().unwrap().get(&key) {
            self.counters.memory_hits.fetch_add(1, Relaxed);
            return Ok(hit.clone());
        }
        if let Some(e) = self.failed.lock().unwrap().get(&key) {
            self.counters.negative_hits.fetch_add(1, Relaxed);
            return Err(e.clone());
        }
        let cell = {
            let mut inflight = self.inflight.lock().unwrap();
            // Re-check under the lock: a finishing thread inserts into memory before
            // removing its in-flight cell.
            if let Some(hit) = self.memory.lock().unwrap().get(&key) {
                self.counters.memory_hits.fetch_add(1, Relaxed);
                return Ok(hit.clone());
            }
            inflight.entry(key).or_default().clone()
        };
        let mut ran = false;
        let result = cell
            .get_or_init(|| {
                ran = true;
                self.load_or_compile(key, compile)
            })
            .clone();
        if ran {
            match &result {
                Ok(s) => {
                    self.transient.lock().unwrap().remove(&key);
                    let sz = s.metallib.len() as u64;
                    if self.memory_bytes.load(Relaxed) + sz <= self.config.memory_limit_bytes {
                        self.memory_bytes.fetch_add(sz, Relaxed);
                        self.memory.lock().unwrap().insert(key, s.clone());
                    }
                }
                Err(e) if is_deterministic(e) => {
                    self.failed.lock().unwrap().insert(key, e.clone());
                }
                Err(e) => {
                    // Helper crash or timeout, I/O, or Metal: this may be transient (a worker killed
                    // for another reason, a full disk), so retry once. The second failure is
                    // remembered so an input that reliably crashes the converter doesn't keep
                    // taking down workers.
                    let mut t = self.transient.lock().unwrap();
                    let n = t.entry(key).or_default();
                    *n += 1;
                    if *n > TRANSIENT_RETRIES {
                        t.remove(&key);
                        self.failed.lock().unwrap().insert(key, e.clone());
                    }
                }
            }
            self.inflight.lock().unwrap().remove(&key);
        } else {
            self.counters.dedup_waits.fetch_add(1, Relaxed);
        }
        result
    }

    /// Reads and decodes a disk entry; corrupt entries are removed.
    fn read_disk<T>(&self, path: &Path, decode: impl Fn(&[u8]) -> Result<T, Error>) -> Option<T> {
        match std::fs::read(path) {
            Ok(bytes) => match decode(&bytes) {
                Ok(v) => {
                    self.counters.disk_bytes_read.fetch_add(bytes.len() as u64, Relaxed);
                    Some(v)
                }
                Err(e) => {
                    log::warn!(target: "shaders", "corrupt cache entry {}: {e}", path.display());
                    self.counters.disk_errors.fetch_add(1, Relaxed);
                    let _ = std::fs::remove_file(path);
                    None
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => {
                log::warn!(target: "shaders", "cache read {}: {e}", path.display());
                self.counters.disk_errors.fetch_add(1, Relaxed);
                None
            }
        }
    }

    fn write_disk(&self, path: &Path, bytes: &[u8]) {
        match write_atomic(path, bytes) {
            Ok(()) => {
                self.counters.disk_bytes_written.fetch_add(bytes.len() as u64, Relaxed);
            }
            Err(e) => {
                log::warn!(target: "shaders", "cache write {}: {e}", path.display());
                self.counters.disk_errors.fetch_add(1, Relaxed);
            }
        }
    }

    fn load_or_compile(&self, key: u128, compile: impl FnOnce() -> Result<CompiledShader, Error>) -> Result<Arc<CompiledShader>, Error> {
        let path = self.entry_path(key);
        let failure_path = path.as_ref().map(|p| p.with_extension("aw2e"));
        if let Some(path) = &path {
            if let Some(s) = self.read_disk(path, |b| decode_entry(b, key)) {
                self.counters.disk_hits.fetch_add(1, Relaxed);
                return Ok(Arc::new(s));
            }
        }
        if let Some(fb_path) = self.fallback_entry_path(key) {
            if let Some(s) = self.read_disk(&fb_path, |b| decode_entry(b, key)) {
                self.counters.disk_hits.fetch_add(1, Relaxed);
                if let Some(primary) = &path {
                    if primary != &fb_path && !primary.exists() {
                        if let Ok(raw) = std::fs::read(&fb_path) {
                            let _ = write_atomic(primary, &raw);
                        }
                    }
                }
                return Ok(Arc::new(s));
            }
            let fb_fail = fb_path.with_extension("aw2e");
            if let Some(e) = self.read_disk(&fb_fail, |b| decode_failure(b, key)) {
                self.counters.negative_hits.fetch_add(1, Relaxed);
                return Err(e);
            }
        }
        if let Some(fp) = &failure_path {
            if let Some(e) = self.read_disk(fp, |b| decode_failure(b, key)) {
                self.counters.negative_hits.fetch_add(1, Relaxed);
                return Err(e);
            }
        }
        let t = Instant::now();
        let r = compile();
        self.counters.compile_nanos.fetch_add(t.elapsed().as_nanos() as u64, Relaxed);
        self.counters.compiles.fetch_add(1, Relaxed);
        let mut s = match r {
            Ok(s) => s,
            Err(e) => {
                self.counters.compile_failures.fetch_add(1, Relaxed);
                // Only deterministic failures (a function of the key's inputs) are persisted;
                // helper/IO/load problems may be transient.
                if let Some(fp) = failure_path.as_ref().filter(|_| is_deterministic(&e)) {
                    self.write_disk(fp, &encode_failure(key, &e));
                }
                return Err(e);
            }
        };
        s.key = key;
        if let Some(path) = &path {
            self.write_disk(path, &encode_entry(&s));
        }
        Ok(Arc::new(s))
    }
}

/// Retries granted to a key after a non-deterministic failure before it's negative-cached.
const TRANSIENT_RETRIES: u32 = 1;

/// Failures that are a pure function of the request (safe to cache on disk and in memory).
fn is_deterministic(e: &Error) -> bool {
    matches!(e, Error::Convert { .. } | Error::InvalidInput(_) | Error::StageMismatch { .. })
}

pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = path.parent().unwrap();
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".tmp-{}-{}", std::process::id(), SEQ.fetch_add(1, Relaxed)));
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&tmp);
    })
}

/// Entry format (v2): magic, key, [`wire::put_shader`] body (stage, function name, decoded
/// reflection incl. converter JSON, metallib), trailing XXH128 of everything before it.
/// Decoding is pure Rust, so disk hits never need the converter.
fn encode_entry(s: &CompiledShader) -> Vec<u8> {
    let mut w = Writer::with_capacity(s.metallib.len() + s.reflection.json.len() + 512);
    w.buf.extend_from_slice(FILE_MAGIC);
    w.u128(s.key);
    wire::put_shader(&mut w, s);
    let h = hash128(&w.buf);
    w.u128(h);
    w.buf
}

fn decode_entry(b: &[u8], key: u128) -> Result<CompiledShader, Error> {
    let bad = |m: &str| Error::Io(format!("cache entry: {m}"));
    if b.len() < 8 + 16 + 16 || &b[..8] != FILE_MAGIC {
        return Err(bad("bad magic"));
    }
    let (body, tail) = b.split_at(b.len() - 16);
    if hash128(body).to_le_bytes() != tail {
        return Err(bad("checksum mismatch"));
    }
    let mut r = Reader::new(&body[8..]);
    if r.u128()? != key {
        return Err(bad("key mismatch"));
    }
    let s = wire::get_shader(&mut r, key)?;
    if r.remaining() != 0 {
        return Err(bad("trailing bytes"));
    }
    Ok(s)
}

/// Bump the version when the compile front end changes so that an input which used to fail
/// may now succeed: 02 took entry points from DXIL bitcode when PSV0 has no name, 03 converts
/// SM4/SM5 DXBC through dxilconv. Older failure entries are then dropped and retried, while
/// converted shaders stay cached.
const FAILURE_MAGIC: &[u8; 8] = b"AW2SHE03";

/// Failure entry (`.aw2e`): magic, key, [`wire::put_error`], trailing XXH128.
fn encode_failure(key: u128, e: &Error) -> Vec<u8> {
    let mut w = Writer::default();
    w.buf.extend_from_slice(FAILURE_MAGIC);
    w.u128(key);
    wire::put_error(&mut w, e);
    let h = hash128(&w.buf);
    w.u128(h);
    w.buf
}

fn decode_failure(b: &[u8], key: u128) -> Result<Error, Error> {
    let bad = |m: &str| Error::Io(format!("cache failure entry: {m}"));
    if b.len() >= 8 && b[..6] == FAILURE_MAGIC[..6] && &b[..8] != FAILURE_MAGIC {
        return Err(bad("stale (recorded by an older front end); retrying"));
    }
    if b.len() < 8 + 16 + 16 || &b[..8] != FAILURE_MAGIC {
        return Err(bad("bad magic"));
    }
    let (body, tail) = b.split_at(b.len() - 16);
    if hash128(body).to_le_bytes() != tail {
        return Err(bad("checksum mismatch"));
    }
    let mut r = Reader::new(&body[8..]);
    if r.u128()? != key {
        return Err(bad("key mismatch"));
    }
    wire::get_error(&mut r)
}

// ---- pipeline front end ----------------------------------------------------------------------

/// Where conversions run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Converter loaded into this process.
    InProcess,
    /// `dxil-metald` worker processes (crash isolation; native arm64 from the x86_64 loader).
    Helper,
    /// Precompiled `.aw2s` cache-only mode: zero Metal Shader Converter calls (pure-Rust root
    /// signatures + disk/bundled shader cache). Default on iOS where `libmetalirconverter.dylib`
    /// is unavailable; misses fail honestly with [`Error::Load`].
    CacheOnly,
}

impl BackendKind {
    /// `PORT_SHADER_BACKEND` / `AW2_SHADER_BACKEND` = `inprocess` | `helper` | `cache-only` | `auto` (default).
    /// `auto` uses `cache-only` on iOS, otherwise the helper when found, else in-process (falling
    /// back to `cache-only` if a precompiled cache exists when MSC is absent).
    pub fn from_env() -> Option<Self> {
        let raw = std::env::var("PORT_SHADER_BACKEND")
            .or_else(|_| std::env::var("AW2_SHADER_BACKEND"))
            .ok();
        match raw.as_deref() {
            Some("inprocess" | "in-process" | "local") => Some(Self::InProcess),
            Some("helper" | "remote") => Some(Self::Helper),
            Some("cache-only" | "cacheonly" | "cache" | "precompiled") => Some(Self::CacheOnly),
            _ => None,
        }
    }
}

enum Backend {
    InProcess,
    Helper(crate::helper::HelperClient),
    CacheOnly,
}

/// The production entry point: root signature creation (deduplicated) and cached compilation.
pub struct ShaderPipeline {
    cache: ShaderCache,
    root_signatures: Mutex<HashMap<u128, Arc<RootSignature>>>,
    fingerprint: String,
    backend: Backend,
    manifest: RwLock<Option<Arc<crate::manifest::ManifestWriter>>>,
}

impl ShaderPipeline {
    /// Opens the cache with the backend chosen by [`BackendKind::from_env`] (or `auto`).
    pub fn new(config: CacheConfig) -> Result<Self, Error> {
        match BackendKind::from_env() {
            Some(kind) => Self::with_backend(config, kind),
            None if cfg!(target_os = "ios") => Self::with_backend(config, BackendKind::CacheOnly),
            None if crate::helper::find_helper().is_some() => {
                Self::with_backend(config.clone(), BackendKind::Helper).or_else(|e| {
                    log::warn!(target: "shaders", "{e}; converting in-process");
                    Self::open_inprocess_or_cache_only(config)
                })
            }
            None => Self::open_inprocess_or_cache_only(config),
        }
    }

    fn open_inprocess_or_cache_only(config: CacheConfig) -> Result<Self, Error> {
        match Self::with_backend(config.clone(), BackendKind::InProcess) {
            Ok(p) => Ok(p),
            Err(e) => {
                if discover_precompiled_namespace(&config).is_some() {
                    log::warn!(
                        target: "shaders",
                        "{e}; precompiled shader cache found, falling back to cache-only mode"
                    );
                    Self::with_backend(config, BackendKind::CacheOnly)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Opens the cache with an explicit backend. `Helper` fails if `dxil-metald` can't be
    /// found ([`crate::helper::find_helper`]) or started.
    pub fn with_backend(config: CacheConfig, kind: BackendKind) -> Result<Self, Error> {
        match kind {
            BackendKind::InProcess => {
                let raw_fingerprint = ffi::api()?.fingerprint.clone();
                let cache = ShaderCache::new(config, &raw_fingerprint);
                let fingerprint = cache.namespace().to_string();
                Ok(Self { cache, root_signatures: Mutex::default(), fingerprint, backend: Backend::InProcess, manifest: RwLock::default() })
            }
            BackendKind::Helper => {
                let exe = crate::helper::find_helper()
                    .ok_or_else(|| Error::Helper(format!("{} not found (set PORT_SHADERD or AW2_SHADERD)", crate::helper::HELPER_NAME)))?;
                let workers = std::env::var("PORT_SHADERD_WORKERS")
                    .or_else(|_| std::env::var("AW2_SHADERD_WORKERS"))
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| std::thread::available_parallelism().map_or(4, |n| n.get()));
                Self::with_helper(config, &exe, workers)
            }
            BackendKind::CacheOnly => {
                let raw_fingerprint = discover_precompiled_namespace(&config)
                    .unwrap_or_else(|| "cache-only".to_string());
                let cache = ShaderCache::new(config, &raw_fingerprint);
                let fingerprint = cache.namespace().to_string();
                log::info!(
                    target: "shaders",
                    "shader pipeline running in cache-only mode (namespace={fingerprint}, fallback={:?})",
                    cache.fallback_dir()
                );
                Ok(Self {
                    cache,
                    root_signatures: Mutex::default(),
                    fingerprint,
                    backend: Backend::CacheOnly,
                    manifest: RwLock::default(),
                })
            }
        }
    }

    /// Helper backend with an explicit `dxil-metald` path and worker limit.
    pub fn with_helper(config: CacheConfig, exe: &Path, max_workers: usize) -> Result<Self, Error> {
        let client = crate::helper::HelperClient::connect(exe, max_workers)?;
        log::info!(target: "shaders", "converting via {} ({} workers max, {})", exe.display(), max_workers, client.fingerprint());
        let raw_fingerprint = client.fingerprint().to_string();
        let cache = ShaderCache::new(config, &raw_fingerprint);
        let fingerprint = cache.namespace().to_string();
        Ok(Self { cache, root_signatures: Mutex::default(), fingerprint, backend: Backend::Helper(client), manifest: RwLock::default() })
    }

    /// The helper client (for diagnostics), if this pipeline uses one.
    pub fn helper(&self) -> Option<&crate::helper::HelperClient> {
        match &self.backend {
            Backend::InProcess | Backend::CacheOnly => None,
            Backend::Helper(c) => Some(c),
        }
    }

    /// Process-wide pipeline configured from the environment ([`CacheConfig::from_env`],
    /// [`BackendKind::from_env`]).
    pub fn global() -> Result<&'static ShaderPipeline, Error> {
        GLOBAL_PIPELINE
            .get_or_init(|| {
                let p = ShaderPipeline::new(CacheConfig::from_env())?;
                p.set_manifest(crate::manifest::ManifestWriter::from_env().map(Arc::new));
                Ok(p)
            })
            .as_ref()
            .map_err(Clone::clone)
    }

    /// Drops in-memory metallib buffers from the process-wide [`ShaderCache`] if it has
    /// already been initialized (disk cache is untouched). Safe under memory pressure.
    pub fn trim_memory_if_initialized() -> u64 {
        if let Some(Ok(p)) = GLOBAL_PIPELINE.get() {
            let before = p.cache.memory_bytes.load(Relaxed);
            p.cache.clear_memory();
            return before;
        }
        0
    }

    /// Records every [`compile`](Self::compile) request into `manifest` (for
    /// `dxil-metal prewarm --manifest`). [`global`](Self::global) enables this from the
    /// environment ([`crate::manifest::ManifestWriter::from_env`]).
    pub fn set_manifest(&self, manifest: Option<Arc<crate::manifest::ManifestWriter>>) {
        *self.manifest.write().unwrap() = manifest;
    }
    pub fn manifest(&self) -> Option<Arc<crate::manifest::ManifestWriter>> {
        self.manifest.read().unwrap().clone()
    }

    pub fn backend(&self) -> BackendKind {
        match self.backend {
            Backend::InProcess => BackendKind::InProcess,
            Backend::Helper(_) => BackendKind::Helper,
            Backend::CacheOnly => BackendKind::CacheOnly,
        }
    }
    pub fn cache(&self) -> &ShaderCache {
        &self.cache
    }
    pub fn stats(&self) -> CacheStats {
        self.cache.stats()
    }
    /// Identity of the converter build (part of every key).
    pub fn converter_fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// `ID3D12Device::CreateRootSignature`: parses the blob (DXBC container with RTS0, or bare
    /// RTS0) and returns a shared root signature; identical signatures are deduplicated.
    /// In-process, the converter validates it immediately; with the helper or cache-only backend,
    /// the layout is computed directly in pure Rust with zero converter calls.
    pub fn root_signature(&self, blob: &[u8]) -> Result<Arc<RootSignature>, Error> {
        let desc = crate::RootSignatureDesc::from_blob(blob)?;
        let h = desc.canonical_hash();
        if let Some(rs) = self.root_signatures.lock().unwrap().get(&h) {
            return Ok(rs.clone());
        }
        let rs = match self.backend {
            Backend::InProcess => RootSignature::new(desc)?,
            Backend::Helper(_) | Backend::CacheOnly => RootSignature::new_deferred(desc),
        };
        Ok(self.root_signatures.lock().unwrap().entry(h).or_insert(rs).clone())
    }

    /// Cached compile.
    pub fn compile(&self, req: &CompileRequest) -> Result<Arc<CompiledShader>, Error> {
        if let Some(m) = self.manifest.read().unwrap().as_ref() {
            if let Err(e) = m.record(req) {
                log::warn!(target: "shaders", "shader manifest: {e}");
            }
        }
        let mut key = req.cache_key(&self.fingerprint);
        // Non-library shaders prewarmed via `rfx`/`pe` pass `Some(entry_point)`, whereas runtime
        // `compile_stage` passes `None`. Check the explicit entry-point key if `None` is not cached
        // (checking memory, primary disk cache, AND read-only bundled fallback cache).
        if req.entry_point.is_none() && !self.cache.has_cached_entry(key) {
            if let Ok(c) = crate::dxbc::Container::parse(req.dxil) {
                if c.program().is_none_or(|p| p.kind != crate::dxbc::ProgramKind::Library) {
                    let ep = c.entry_point().unwrap_or_else(|| "main".to_string());
                    let alt = CompileRequest { entry_point: Some(&ep), ..*req }.cache_key(&self.fingerprint);
                    if self.cache.has_cached_entry(alt) {
                        key = alt;
                    }
                }
            }
        }
        // The expected stage is not part of the key (it doesn't change the output): compile
        // without it and check afterwards, so cache hits are checked too and a mismatching
        // request can't poison the negative cache for correct ones.
        let inner = CompileRequest { expected_stage: None, ..*req };
        let s = self.cache.get_or_compile(key, || match &self.backend {
            Backend::InProcess => compile_uncached(&inner),
            Backend::Helper(c) => c.compile(&inner, key),
            Backend::CacheOnly => Err(Error::Load(format!(
                "precompiled shader cache miss for key {key:032x} (namespace={}; precompile on macOS with `dxil-metal prewarm`)",
                self.fingerprint
            ))),
        })?;
        match req.expected_stage {
            Some(want) if want != s.stage => Err(Error::StageMismatch { expected: want, actual: s.stage }),
            _ => Ok(s),
        }
    }

    /// Convenience: compile `dxil` as `stage` against `root_signature`.
    pub fn compile_stage(
        &self,
        stage: ShaderStage,
        dxil: &[u8],
        root_signature: Option<&RootSignature>,
        options: &CompileOptions,
    ) -> Result<Arc<CompiledShader>, Error> {
        self.compile(&CompileRequest::new(dxil, root_signature, options).stage(stage))
    }

    /// Cached stage-in function synthesis for a vertex shader compiled with
    /// `StageInMode::SeparateFunction`.
    pub fn stage_in(&self, vertex: &CompiledShader, layout: &[InputElement], options: &CompileOptions) -> Result<Arc<CompiledShader>, Error> {
        let mut k = KeyHasher::new("dxil-metal/stage-in/v1");
        k.u128(vertex.key);
        for e in layout {
            k.bytes(e.semantic_name.as_bytes()).u32(e.semantic_index).u32(e.format).u32(e.input_slot);
            k.u32(e.aligned_byte_offset).u32(e.input_slot_class).u32(e.instance_data_step_rate);
        }
        options.hash_into(&mut k);
        k.bytes(self.fingerprint.as_bytes());
        let key = k.finish();
        self.cache.get_or_compile(key, || match &self.backend {
            Backend::InProcess => stage_in_shader(vertex, layout, options, key),
            Backend::Helper(c) => c.stage_in(vertex, layout, options, key),
            Backend::CacheOnly => Err(Error::Load(format!(
                "precompiled stage-in cache miss for key {key:032x} (namespace={}; precompile on macOS with `dxil-metal prewarm`)",
                self.fingerprint
            ))),
        })
    }
}

#[cfg(test)]
mod cache_only_tests {
    use super::*;
    use crate::reflection::{ShaderReflection, ShaderStage, StageInfo};

    fn dummy_compiled(key: u128, stage: ShaderStage, metallib: &[u8]) -> CompiledShader {
        CompiledShader {
            key,
            stage,
            function_name: "main".to_string(),
            metallib: metallib.to_vec(),
            reflection: ShaderReflection {
                stage,
                function_name: "main".to_string(),
                resources: Vec::new(),
                needs_function_constants: false,
                function_constants: Vec::new(),
                info: StageInfo::None,
                json: "{}".to_string(),
            },
        }
    }

    #[test]
    fn cache_only_hits_precompiled_and_rejects_misses_without_msc() {
        let tmp = std::env::temp_dir().join(format!("dxil_cache_only_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let ns = "ir4.0.1-uuid-cafebabe";
        std::fs::create_dir_all(tmp.join(ns)).unwrap();

        let cfg = CacheConfig { disk_dir: Some(tmp.clone()), memory_limit_bytes: 16 << 20 };
        let pipe = ShaderPipeline::with_backend(cfg, BackendKind::CacheOnly).expect("cache-only open");
        assert_eq!(pipe.backend(), BackendKind::CacheOnly);
        assert_eq!(pipe.converter_fingerprint(), ns);

        // Seed one precompiled shader entry directly in the `.aw2s` cache format.
        let opts = CompileOptions::default();
        let fake_dxil = b"DXBC_TEST_PRECOMPILED_SHADER";
        let req = CompileRequest::new(fake_dxil, None, &opts).stage(ShaderStage::Fragment);
        let key = req.cache_key(pipe.converter_fingerprint());
        let seeded = dummy_compiled(key, ShaderStage::Fragment, b"MTLB_FAKE_AIR");
        let entry_path = pipe.cache().entry_path(key).unwrap();
        write_atomic(&entry_path, &encode_entry(&seeded)).unwrap();

        // Hit succeeds purely from disk with zero Metal Shader Converter dependency.
        let hit = pipe.compile(&req).expect("precompiled hit");
        assert_eq!(hit.stage, ShaderStage::Fragment);
        assert_eq!(hit.metallib, b"MTLB_FAKE_AIR");
        assert_eq!(pipe.stats().disk_hits, 1);

        // Miss fails honestly with Error::Load and does not persist a negative `.aw2e` file.
        let miss_req = CompileRequest::new(b"DXBC_MISSING", None, &opts).stage(ShaderStage::Vertex);
        let miss_key = miss_req.cache_key(pipe.converter_fingerprint());
        let err = pipe.compile(&miss_req).expect_err("miss must fail honestly");
        assert!(matches!(err, Error::Load(_)), "unexpected error variant: {err:?}");
        let fail_path = pipe.cache().entry_path(miss_key).unwrap().with_extension("aw2e");
        assert!(!fail_path.exists(), "cache-only miss must not poison disk failure cache");

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
