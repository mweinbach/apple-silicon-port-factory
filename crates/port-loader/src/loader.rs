//! The PE loader: mapping, relocation, import resolution, static TLS, DLL
//! initialization and the [`win32_rt::LoaderOps`] services.

use crate::thunks;
use anyhow::{anyhow, bail, Context as _, Result};
use pe_image::consts::*;
use pe_image::{ExportTarget, Exports, ImportName, Pe, RuntimeFunction};
use win32_rt::{self as w, teb, ProcName};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, RwLock};

pub const PAGE: usize = 0x1000;

/// Static TLS of one module.
pub struct Tls {
    pub index: u32,
    pub template_va: usize,
    pub template_len: usize,
    pub zero_fill: usize,
    pub align: usize,
}

pub enum Kind {
    Pe {
        exports: Exports,
        pdata: Vec<RuntimeFunction>,
        tls: Option<Tls>,
        entry: usize,
        is_dll: bool,
        /// Executable section RVA ranges `(start_rva, end_rva)` for distinguishing
        /// code exports from data exports.
        exec_ranges: Vec<(u32, u32)>,
    },
    Builtin {
        dll: Option<&'static w::BuiltinDll>,
        /// Export name / ordinal -> address in the fake image (thunk or data).
        by_name: HashMap<String, usize>,
        by_ordinal: HashMap<u16, usize>,
        /// Thunk address -> export name.
        thunk_names: HashMap<usize, String>,
    },
}

pub struct Module {
    /// Lowercase base name (`kernel32.dll`, `vcruntime140.dll`).
    pub name: String,
    /// Windows path (`C:\Games\AlanWake2\vcruntime140.dll`).
    pub win_path: String,
    /// Canonical host path for mapped guest PEs (`None` for builtins).
    pub host_path: Option<PathBuf>,
    pub base: usize,
    pub size: usize,
    pub kind: Kind,
    pub tls_callbacks: Vec<usize>,
    pub initialized: AtomicBool,
    pub thread_calls: AtomicBool,
    pub refs: AtomicU32,
    pub ldr: AtomicUsize,
}

/// Separate record for `LOAD_LIBRARY_AS_DATAFILE` / `LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE` /
/// `LOAD_LIBRARY_AS_IMAGE_RESOURCE` mappings. Never registered in `PEB->Ldr` or `modules`,
/// never resolves imports, and never runs TLS/DllMain.
struct DatafileModule {
    /// Tagged HMODULE (`base | 1` for datafile, `base | 2` for image resource).
    handle: usize,
    base: usize,
    size: usize,
    owned_mmap: bool,
    refs: AtomicU32,
}

impl Module {
    pub fn is_builtin(&self) -> bool {
        matches!(self.kind, Kind::Builtin { .. })
    }
    pub fn contains(&self, a: usize) -> bool {
        a >= self.base && a < self.base + self.size
    }
    fn entry(&self) -> usize {
        match &self.kind {
            Kind::Pe { entry, .. } => *entry,
            _ => 0,
        }
    }
    fn is_dll(&self) -> bool {
        match &self.kind {
            Kind::Pe { is_dll, .. } => *is_dll,
            _ => true,
        }
    }
}

/// Recursive lock (Windows' loader lock is re-entrant: DllMain may call
/// LoadLibrary).
struct ReentrantLock {
    state: Mutex<(u32, u32)>, // (owner tid, depth)
    cv: Condvar,
}

struct ReentrantGuard<'a>(&'a ReentrantLock);

fn host_tid() -> u32 {
    let mut t: u64 = 0;
    unsafe { libc::pthread_threadid_np(0 as libc::pthread_t, &mut t) };
    t as u32
}

impl ReentrantLock {
    const fn new() -> Self {
        ReentrantLock { state: Mutex::new((0, 0)), cv: Condvar::new() }
    }
    fn lock(&self) -> ReentrantGuard<'_> {
        let me = host_tid();
        let mut s = self.state.lock().unwrap();
        while s.1 > 0 && s.0 != me {
            s = self.cv.wait(s).unwrap();
        }
        s.0 = me;
        s.1 += 1;
        ReentrantGuard(self)
    }
}

impl Drop for ReentrantGuard<'_> {
    fn drop(&mut self) {
        let mut s = self.0.state.lock().unwrap();
        s.1 -= 1;
        if s.1 == 0 {
            s.0 = 0;
            self.0.cv.notify_all();
        }
    }
}

/// Fixed capacity of every thread's ThreadLocalStoragePointer array, so
/// DLLs with TLS loaded at runtime can be added to existing threads in place.
const TLS_ARRAY_CAP: usize = 256;

pub struct Loader {
    pub game_dir: PathBuf,
    pub game_win_dir: String,
    pub trace: bool,
    routing: port_manifest::DllRoutingTable,
    v8_jitless: bool,
    modules: RwLock<Vec<Arc<Module>>>,
    datafile_modules: RwLock<Vec<Arc<DatafileModule>>>,
    /// Modules mapped + resolved but not yet initialized (dependency order),
    /// tagged with a sequence number so a nested LoadLibrary only initializes
    /// what it loaded itself.
    pending_init: Mutex<Vec<(u64, Arc<Module>)>>,
    pending_seq: std::sync::atomic::AtomicU64,
    tls_modules: RwLock<Vec<Arc<Module>>>,
    next_tls: AtomicU32,
    lock: ReentrantLock,
    exe: OnceLock<Arc<Module>>,
    /// Cache of trace thunks per (dll, name) so GetProcAddress is stable.
    trace_cache: Mutex<HashMap<(String, String), usize>>,
    /// Immutable guest-visible thunk page for the fixed verified Winsock2 extensions.
    wsa_ext_thunks: OnceLock<crate::builtin_image::FixedThunkPage>,
    /// Count of unresolved static imports (trap thunks installed).
    pub traps: AtomicU32,
    /// `dll!name` of every unresolved static import.
    pub unresolved: Mutex<std::collections::BTreeSet<String>>,
    /// Pre-reserved `PROT_NONE` virtual address windows for dynamically loaded bundled
    /// plugins that have a cached `.aot` (or static/pinned) preferred base, preventing
    /// later `VirtualAlloc` / `malloc` / Metal mappings from claiming those low-VA slots
    /// before `LoadLibrary` runs.
    reserved_aot_slots: Mutex<HashMap<String, (usize, usize)>>,
}

static LOADER: OnceLock<Loader> = OnceLock::new();

pub fn get() -> &'static Loader {
    LOADER.get().expect("loader not initialized")
}

pub fn try_get() -> Option<&'static Loader> {
    LOADER.get()
}

impl Loader {
    fn new_instance(
        game_dir: PathBuf,
        game_win_dir: String,
        trace: bool,
        routing: port_manifest::DllRoutingTable,
        v8_jitless: bool,
    ) -> Self {
        Loader {
            game_dir,
            game_win_dir,
            trace,
            routing,
            v8_jitless,
            modules: RwLock::new(Vec::new()),
            datafile_modules: RwLock::new(Vec::new()),
            pending_init: Mutex::new(Vec::new()),
            pending_seq: std::sync::atomic::AtomicU64::new(0),
            tls_modules: RwLock::new(Vec::new()),
            next_tls: AtomicU32::new(0),
            lock: ReentrantLock::new(),
            exe: OnceLock::new(),
            trace_cache: Mutex::new(HashMap::new()),
            wsa_ext_thunks: OnceLock::new(),
            traps: AtomicU32::new(0),
            unresolved: Mutex::new(Default::default()),
            reserved_aot_slots: Mutex::new(HashMap::new()),
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        if let Ok(mut slots) = self.reserved_aot_slots.lock() {
            for (_, (base, size)) in slots.drain() {
                unsafe {
                    libc::munmap(base as *mut libc::c_void, size);
                }
            }
        }
    }
}

/// Fast header-only peek of a PE32+ file's `(image_base, page_aligned_size_of_image, is_dll)`.
pub(crate) fn peek_pe_preferred_window(path: &Path) -> Option<(usize, usize, bool)> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = [0u8; 4096];
    let n = f.read(&mut buf).ok()?;
    if n < 0x40 || &buf[0..2] != b"MZ" {
        return None;
    }
    let lfanew = u32::from_le_bytes(buf[0x3C..0x40].try_into().ok()?) as usize;
    if lfanew.checked_add(24 + 64)? > n || &buf[lfanew..lfanew + 4] != b"PE\0\0" {
        return None;
    }
    let characteristics = u16::from_le_bytes(buf[lfanew + 22..lfanew + 24].try_into().ok()?);
    let is_dll = (characteristics & 0x2000) != 0;
    let opt = lfanew + 24;
    let magic = u16::from_le_bytes(buf[opt..opt + 2].try_into().ok()?);
    if magic != 0x20B {
        return None;
    }
    let image_base = u64::from_le_bytes(buf[opt + 24..opt + 32].try_into().ok()?) as usize;
    let size_of_image = u32::from_le_bytes(buf[opt + 56..opt + 60].try_into().ok()?) as usize;
    let aligned_size = (size_of_image + PAGE - 1) & !(PAGE - 1);
    (image_base != 0 && aligned_size != 0).then_some((image_base, aligned_size, is_dll))
}

pub fn init_with_routing(
    game_dir: PathBuf,
    game_win_dir: String,
    trace: bool,
    routing: port_manifest::DllRoutingTable,
    v8_jitless: bool,
) -> &'static Loader {
    let l = Loader::new_instance(game_dir, game_win_dir, trace, routing, v8_jitless);
    if LOADER.set(l).is_err() {
        panic!("loader::init called twice");
    }
    let l = get();
    w::set_loader(l);
    l
}

#[allow(dead_code)]
pub fn init(game_dir: PathBuf, game_win_dir: String, trace: bool) -> &'static Loader {
    let extra = std::env::var("PORT_BUILTIN_OVERRIDES")
        .or_else(|_| std::env::var("AW2_BUILTIN_OVERRIDES"))
        .ok();
    let routing = port_manifest::DllRoutingTable::from_legacy_override_spec(extra.as_deref());
    init_with_routing(game_dir, game_win_dir, trace, routing, false)
}

/// How a DLL name resolves.
#[derive(Debug, PartialEq, Eq)]
enum Source {
    GameDir(PathBuf),
    Builtin(String),
}

/// Internal search & mapping options threaded through `load_with_opts` / `load_pe_with_opts`.
#[derive(Clone, Debug, Default)]
struct LoadOptions {
    flags: u32,
    /// Explicit search directories (e.g. from `LdrLoadDll` `PathToFile` semicolon-separated list).
    extra_search_dirs: Vec<PathBuf>,
    /// Module directory propagated for `LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR`,
    /// `LOAD_WITH_ALTERED_SEARCH_PATH`, or offline plugin prewarming.
    dll_load_dir: Option<PathBuf>,
    /// True during static import resolution (unknown DLLs become trapping empty builtins).
    static_import: bool,
    /// True during offline prewarm (map + resolve imports without executing TLS/DllMain).
    prewarm: bool,
}

fn find_in_dir_ci(dir: &Path, base_name: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    let direct = dir.join(base_name);
    if direct.is_file() {
        return Some(direct);
    }
    std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok()).find_map(|e| {
        let n = e.file_name();
        let n = n.to_str()?;
        (n.eq_ignore_ascii_case(base_name) && e.path().is_file()).then(|| e.path())
    })
}

fn parse_search_path_dirs(search_path: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for part in search_path.split(';') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(host) = w::path::to_host(trimmed) {
            if host.is_dir() {
                dirs.push(host);
            }
        }
    }
    dirs
}

fn host_prot(ch: u32) -> i32 {
    let mut p = 0;
    if ch & IMAGE_SCN_MEM_READ != 0 {
        p |= libc::PROT_READ;
    }
    if ch & IMAGE_SCN_MEM_WRITE != 0 {
        p |= libc::PROT_WRITE;
    }
    if ch & IMAGE_SCN_MEM_EXECUTE != 0 {
        #[cfg(target_arch = "x86_64")]
        {
            p |= libc::PROT_EXEC | libc::PROT_READ;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            p |= libc::PROT_READ;
        }
    }
    p
}

fn win_prot(ch: u32) -> u32 {
    use win32_rt::kernel32::memory::*;
    let (r, wr, x) = (ch & IMAGE_SCN_MEM_READ != 0, ch & IMAGE_SCN_MEM_WRITE != 0, ch & IMAGE_SCN_MEM_EXECUTE != 0);
    match (x, wr, r) {
        (true, true, _) => PAGE_EXECUTE_WRITECOPY,
        (true, false, _) => PAGE_EXECUTE_READ,
        (false, true, _) => PAGE_WRITECOPY,
        (false, false, true) => PAGE_READONLY,
        _ => PAGE_NOACCESS,
    }
}

impl Loader {
    pub fn modules(&self) -> Vec<Arc<Module>> {
        self.modules.read().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn exe(&self) -> Option<&Arc<Module>> {
        self.exe.get()
    }

    fn find_loaded(&self, norm: &str) -> Option<Arc<Module>> {
        self.modules.read().unwrap().iter().find(|m| m.name == norm).cloned()
    }

    fn find_loaded_by_host_path(&self, host: &Path) -> Option<Arc<Module>> {
        let canon = std::fs::canonicalize(host).unwrap_or_else(|_| host.to_path_buf());
        self.modules
            .read()
            .unwrap()
            .iter()
            .find(|m| m.host_path.as_ref().is_some_and(|p| p == &canon))
            .cloned()
    }

    pub fn module_at(&self, addr: usize) -> Option<Arc<Module>> {
        self.modules.read().unwrap().iter().find(|m| m.contains(addr)).cloned()
    }

    fn module_by_base(&self, base: usize) -> Option<Arc<Module>> {
        self.modules.read().unwrap().iter().find(|m| m.base == base).cloned()
    }

    /// Case-insensitive lookup of a file in the game directory.
    fn find_in_game_dir(&self, base_name: &str) -> Option<PathBuf> {
        find_in_dir_ci(&self.game_dir, base_name)
    }

    /// Resolve a relative path component-by-component case-insensitively under `base_dir`.
    fn find_rel_in_dir(base_dir: &Path, rel: &str) -> Option<PathBuf> {
        let mut cur = base_dir.to_path_buf();
        for comp in rel.split(['\\', '/']).filter(|c| !c.is_empty() && *c != ".") {
            if comp == ".." {
                cur.pop();
                continue;
            }
            let direct = cur.join(comp);
            if direct.exists() {
                cur = direct;
                continue;
            }
            let found = std::fs::read_dir(&cur).ok()?.filter_map(|e| e.ok()).find_map(|e| {
                let n = e.file_name();
                let n = n.to_str()?;
                n.eq_ignore_ascii_case(comp).then(|| e.path())
            })?;
            cur = found;
        }
        cur.is_file().then_some(cur)
    }

    /// Resolve an explicit path (`name` contains `\` or `/`) to a guest PE file on disk.
    fn resolve_explicit_guest_pe(&self, name: &str, norm: &str, allow_sysdir_guest: bool) -> Option<PathBuf> {
        let win_abs = w::path::absolute(name);
        let win_up = win_abs.to_ascii_uppercase();
        let in_sysdir = win_up.starts_with("C:\\WINDOWS\\SYSTEM32\\")
            || win_up.starts_with("C:\\WINDOWS\\SYSWOW64\\");
        if in_sysdir && !allow_sysdir_guest && w::find_builtin(norm).is_some() {
            return None;
        }
        if let Some(h) = w::path::to_host(name) {
            if h.is_file() {
                return Some(h);
            }
        }
        // Fallback for relative paths (e.g. `plugins\foo.pyd`) when CWD differs from `game_dir`.
        if !w::kernel32::module::is_absolute_win_path(name) {
            if let Some(h) = Self::find_rel_in_dir(&self.game_dir, name) {
                return Some(h);
            }
        }
        None
    }

    /// Search for a bare DLL filename according to `LoadLibraryExW` search flags,
    /// `SetDefaultDllDirectories`, `AddDllDirectory`, `SetDllDirectoryW/A`, and
    /// `LdrLoadDll` `PathToFile` hints.
    ///
    /// Returns `(guest_pe_path, allow_system32_builtins)`.
    fn search_bare_guest_pe(
        &self,
        norm: &str,
        opts: &LoadOptions,
        cfg: &w::kernel32::module::DllSearchConfig,
    ) -> (Option<PathBuf>, bool) {
        use w::kernel32::module::*;

        // 0. Explicit `LdrLoadDll` `PathToFile` directories take highest priority.
        for dir in &opts.extra_search_dirs {
            if let Some(p) = find_in_dir_ci(dir, norm) {
                return (Some(p), true);
            }
        }

        let explicit_search = opts.flags & LOAD_LIBRARY_SEARCH_FLAGS_MASK;
        let use_modern_mask = explicit_search != 0
            || ((opts.flags & LOAD_WITH_ALTERED_SEARCH_PATH) == 0 && cfg.default_dirs_mask.is_some());

        if use_modern_mask {
            let mut mask = if explicit_search != 0 {
                explicit_search
            } else {
                cfg.default_dirs_mask.unwrap_or(0)
            };
            if mask & LOAD_LIBRARY_SEARCH_DEFAULT_DIRS != 0 {
                mask |= LOAD_LIBRARY_SEARCH_APPLICATION_DIR
                    | LOAD_LIBRARY_SEARCH_USER_DIRS
                    | LOAD_LIBRARY_SEARCH_SYSTEM32;
            }
            if (mask & LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR != 0) || opts.prewarm {
                if let Some(ref d) = opts.dll_load_dir {
                    if let Some(p) = find_in_dir_ci(d, norm) {
                        return (Some(p), (mask & LOAD_LIBRARY_SEARCH_SYSTEM32) != 0);
                    }
                }
            }
            if mask & LOAD_LIBRARY_SEARCH_APPLICATION_DIR != 0 {
                if let Some(p) = self.find_in_game_dir(norm) {
                    return (Some(p), (mask & LOAD_LIBRARY_SEARCH_SYSTEM32) != 0);
                }
            }
            if mask & LOAD_LIBRARY_SEARCH_USER_DIRS != 0 {
                // Search AddDllDirectory entries (most recently added first), then legacy SetDllDirectory.
                for win_dir in cfg.added_dirs.iter().rev() {
                    if let Some(host_dir) = w::path::to_host(win_dir) {
                        if let Some(p) = find_in_dir_ci(&host_dir, norm) {
                            return (Some(p), (mask & LOAD_LIBRARY_SEARCH_SYSTEM32) != 0);
                        }
                    }
                }
                if let Some(Some(ref legacy_dir)) = cfg.legacy_dll_dir {
                    if let Some(host_dir) = w::path::to_host(legacy_dir) {
                        if let Some(p) = find_in_dir_ci(&host_dir, norm) {
                            return (Some(p), (mask & LOAD_LIBRARY_SEARCH_SYSTEM32) != 0);
                        }
                    }
                }
            }
            let allow_sys32 = (mask & LOAD_LIBRARY_SEARCH_SYSTEM32) != 0;
            if allow_sys32 && w::find_builtin(norm).is_none() {
                if let Some(sys32) = w::path::to_host("C:\\Windows\\System32") {
                    if let Some(p) = find_in_dir_ci(&sys32, norm) {
                        return (Some(p), true);
                    }
                }
            }
            return (None, allow_sys32);
        }

        if opts.flags & LOAD_WITH_ALTERED_SEARCH_PATH != 0 {
            // Altered search path: DLL's own directory replaces the application directory.
            if let Some(ref d) = opts.dll_load_dir {
                if let Some(p) = find_in_dir_ci(d, norm) {
                    return (Some(p), true);
                }
            }
            match &cfg.legacy_dll_dir {
                Some(Some(legacy_dir)) => {
                    if let Some(host_dir) = w::path::to_host(legacy_dir) {
                        if let Some(p) = find_in_dir_ci(&host_dir, norm) {
                            return (Some(p), true);
                        }
                    }
                }
                Some(None) => {
                    // SetDllDirectory("") explicitly removes CWD from search order.
                }
                None => {
                    if let Some(cwd_host) = w::path::to_host(&w::path::cwd()) {
                        if let Some(p) = find_in_dir_ci(&cwd_host, norm) {
                            return (Some(p), true);
                        }
                    }
                }
            }
            if w::find_builtin(norm).is_none() {
                if let Some(sys32) = w::path::to_host("C:\\Windows\\System32") {
                    if let Some(p) = find_in_dir_ci(&sys32, norm) {
                        return (Some(p), true);
                    }
                }
            }
            return (None, true);
        }

        // Standard Windows legacy default search order:
        // 1. Propagated dll_load_dir (during offline prewarm)
        if opts.prewarm {
            if let Some(ref d) = opts.dll_load_dir {
                if let Some(p) = find_in_dir_ci(d, norm) {
                    return (Some(p), true);
                }
            }
        }
        // 2. Application directory (game_dir)
        if let Some(p) = self.find_in_game_dir(norm) {
            return (Some(p), true);
        }
        // 3. SetDllDirectory(dir) OR CWD (unless SetDllDirectory("") disabled CWD)
        match &cfg.legacy_dll_dir {
            Some(Some(legacy_dir)) => {
                if let Some(host_dir) = w::path::to_host(legacy_dir) {
                    if let Some(p) = find_in_dir_ci(&host_dir, norm) {
                        return (Some(p), true);
                    }
                }
            }
            Some(None) => {}
            None => {
                if let Some(cwd_host) = w::path::to_host(&w::path::cwd()) {
                    if let Some(p) = find_in_dir_ci(&cwd_host, norm) {
                        return (Some(p), true);
                    }
                }
            }
        }
        // 4. Non-builtin guest files in System32
        if w::find_builtin(norm).is_none() {
            if let Some(sys32) = w::path::to_host("C:\\Windows\\System32") {
                if let Some(p) = find_in_dir_ci(&sys32, norm) {
                    return (Some(p), true);
                }
            }
        }
        (None, true)
    }

    /// Decide where a DLL comes from using the active [`port_manifest::DllRoutingTable`]
    /// and current Win32 DLL search-path state.
    #[cfg_attr(not(test), allow(dead_code))]
    fn classify(&self, name: &str) -> Option<Source> {
        self.classify_with_opts(name, &LoadOptions::default(), 0)
    }

    fn classify_with_opts(&self, name: &str, opts: &LoadOptions, depth: u8) -> Option<Source> {
        let norm = pe_image::normalize_dll_name(name);
        if norm.starts_with("api-ms-win-") || norm.starts_with("ext-ms-win-") {
            return Some(Source::Builtin(w::canonical_dll(&norm)));
        }
        let has_sep = name.contains('\\') || name.contains('/');
        let is_datafile = (opts.flags & w::kernel32::module::DATAFILE_LOAD_FLAGS_MASK) != 0;

        match self.routing.route(&norm) {
            port_manifest::DllRoute::Disabled => {
                log::info!("{norm}: disabled by DLL routing policy");
                None
            }
            port_manifest::DllRoute::Alias(target) => {
                if depth >= 4 {
                    log::warn!("{norm}: DLL alias chain exceeded depth limit at {target}");
                    return None;
                }
                self.classify_with_opts(&target, opts, depth + 1)
            }
            port_manifest::DllRoute::Builtin { provider } => {
                let target = match provider {
                    Some(p) => w::canonical_dll(&p),
                    None => w::canonical_dll(&norm),
                };
                if has_sep {
                    if !is_datafile && w::find_builtin(&target).is_some() {
                        return Some(Source::Builtin(target));
                    }
                    if let Some(p) = self.resolve_explicit_guest_pe(name, &norm, is_datafile) {
                        return Some(Source::GameDir(p));
                    }
                    return w::find_builtin(&target).map(|_| Source::Builtin(target));
                }
                let cfg = w::kernel32::module::dll_search_config();
                let (guest, allow_sys32) = self.search_bare_guest_pe(&norm, opts, &cfg);
                if allow_sys32 && w::find_builtin(&target).is_some() {
                    if guest.is_some() {
                        log::info!(
                            "{norm}: using builtin {target} instead of the game-dir PE (DLL routing policy)"
                        );
                    }
                    return Some(Source::Builtin(target));
                }
                guest.map(Source::GameDir)
            }
            port_manifest::DllRoute::GuestOnly => {
                if has_sep {
                    self.resolve_explicit_guest_pe(name, &norm, true).map(Source::GameDir)
                } else {
                    let cfg = w::kernel32::module::dll_search_config();
                    self.search_bare_guest_pe(&norm, opts, &cfg).0.map(Source::GameDir)
                }
            }
            port_manifest::DllRoute::Guest => {
                if has_sep {
                    if let Some(p) = self.resolve_explicit_guest_pe(name, &norm, false) {
                        return Some(Source::GameDir(p));
                    }
                    // Explicit System32/SysWOW64 path to a builtin DLL still resolves as Builtin.
                    let win_up = w::path::absolute(name).to_ascii_uppercase();
                    let in_sysdir = win_up.starts_with("C:\\WINDOWS\\SYSTEM32\\")
                        || win_up.starts_with("C:\\WINDOWS\\SYSWOW64\\");
                    if in_sysdir && w::find_builtin(&norm).is_some() {
                        let bname = if norm == "vcruntime140_1.dll" || norm.starts_with("ucrtbase") {
                            w::canonical_dll(&norm)
                        } else {
                            norm
                        };
                        return Some(Source::Builtin(bname));
                    }
                    return None;
                }
                let cfg = w::kernel32::module::dll_search_config();
                let (guest, allow_sys32) = self.search_bare_guest_pe(&norm, opts, &cfg);
                if let Some(p) = guest {
                    return Some(Source::GameDir(p));
                }
                if allow_sys32 && w::find_builtin(&norm).is_some() {
                    let bname = if norm.starts_with("api-ms-win-")
                        || norm.starts_with("ext-ms-win-")
                        || norm == "vcruntime140_1.dll"
                        || norm.starts_with("ucrtbase")
                    {
                        w::canonical_dll(&norm)
                    } else {
                        norm
                    };
                    return Some(Source::Builtin(bname));
                }
                None
            }
        }
    }

    /// Load (or reference) a DLL by name. `static_import` modules that are
    /// unknown become empty builtins (all their imports trap) instead of
    /// failing the whole process.
    pub fn load(&self, name: &str, static_import: bool) -> Result<Arc<Module>> {
        self.load_with_opts(
            name,
            &LoadOptions {
                static_import,
                ..Default::default()
            },
        )
    }

    fn load_with_opts(&self, name: &str, opts: &LoadOptions) -> Result<Arc<Module>> {
        let _g = self.lock.lock();
        let norm = pe_image::normalize_dll_name(name);
        let has_sep = name.contains('\\') || name.contains('/');
        if !has_sep {
            if let Some(m) = self.find_loaded(&norm) {
                m.refs.fetch_add(1, Ordering::Relaxed);
                return Ok(m);
            }
        }
        match self.classify_with_opts(name, opts, 0) {
            Some(Source::GameDir(p)) => {
                if let Some(m) = self.find_loaded_by_host_path(&p) {
                    m.refs.fetch_add(1, Ordering::Relaxed);
                    return Ok(m);
                }
                let win = w::path::to_win(&p).unwrap_or_else(|| format!("{}\\{}", self.game_win_dir, norm));
                // Lifted / natively patched DLLs map at their pinned base (fail-soft: if it
                // is taken the image is relocated and those replacements stay off).
                self.load_pe_with_opts(
                    &p,
                    &norm,
                    win,
                    x86_runtime::dll_native::pinned_base(&norm),
                    !opts.static_import && !opts.prewarm,
                    opts,
                )
            }
            Some(Source::Builtin(c)) => {
                if let Some(m) = self.find_loaded(&c) {
                    m.refs.fetch_add(1, Ordering::Relaxed);
                    return Ok(m);
                }
                Ok(self.add_builtin(&c, w::find_builtin(&c)))
            }
            None if opts.static_import => {
                log::warn!("{norm}: no PE in the search path and no builtin; every import from it will trap");
                Ok(self.add_builtin(&norm, None))
            }
            None => bail!("module {name:?} not found"),
        }
    }

    /// Map a module for `LOAD_LIBRARY_AS_DATAFILE` / `LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE` /
    /// `LOAD_LIBRARY_AS_IMAGE_RESOURCE`. Never resolves imports, never registers in `PEB->Ldr`
    /// or `modules`, and never executes TLS callbacks or `DllMain`.
    fn load_datafile(&self, name: &str, opts: &LoadOptions) -> Result<usize> {
        let _g = self.lock.lock();
        let tag: usize = if (opts.flags & w::kernel32::module::LOAD_LIBRARY_AS_IMAGE_RESOURCE) != 0 {
            2
        } else {
            1
        };
        match self.classify_with_opts(name, opts, 0) {
            Some(Source::GameDir(p)) => {
                let data = std::fs::read(&p).with_context(|| format!("reading {}", p.display()))?;
                let pe = Pe::parse(&data).with_context(|| format!("parsing {}", p.display()))?;
                let h = &pe.headers;
                if h.machine != IMAGE_FILE_MACHINE_AMD64 {
                    bail!("{}: not an x86-64 image (machine 0x{:x})", p.display(), h.machine);
                }
                let size = (h.size_of_image as usize + PAGE - 1) & !(PAGE - 1);
                let base = unsafe { map_image_memory(h.image_base as usize, size, true) }
                    .ok_or_else(|| anyhow!("{}: cannot map 0x{size:x} datafile bytes", p.display()))?;
                let img = unsafe { std::slice::from_raw_parts_mut(base as *mut u8, size) };
                let hdr = (h.size_of_headers as usize).min(data.len()).min(size);
                img[..hdr].copy_from_slice(&data[..hdr]);
                for s in &pe.sections {
                    let n = (s.raw_size.min(s.mem_size()) as usize)
                        .min(data.len().saturating_sub(s.raw_offset as usize));
                    let dst = s.rva as usize;
                    if n > 0 && dst + n <= size {
                        img[dst..dst + n]
                            .copy_from_slice(&data[s.raw_offset as usize..s.raw_offset as usize + n]);
                    }
                }
                let handle = base | tag;
                self.datafile_modules.write().unwrap().push(Arc::new(DatafileModule {
                    handle,
                    base,
                    size,
                    owned_mmap: true,
                    refs: AtomicU32::new(1),
                }));
                log::info!("LoadLibraryEx({name:?}, datafile=0x{:x}) -> 0x{handle:x}", opts.flags);
                Ok(handle)
            }
            Some(Source::Builtin(c)) => {
                let m = if let Some(m) = self.find_loaded(&c) {
                    m
                } else {
                    self.add_builtin(&c, w::find_builtin(&c))
                };
                let handle = m.base | tag;
                let mut dfs = self.datafile_modules.write().unwrap();
                if let Some(existing) = dfs.iter().find(|d| d.handle == handle) {
                    existing.refs.fetch_add(1, Ordering::Relaxed);
                } else {
                    dfs.push(Arc::new(DatafileModule {
                        handle,
                        base: m.base,
                        size: m.size,
                        owned_mmap: false,
                        refs: AtomicU32::new(1),
                    }));
                }
                Ok(handle)
            }
            None => bail!("datafile module {name:?} not found"),
        }
    }

    /// Create the module record for a builtin DLL. Its HMODULE is a fake PE
    /// image with a real export directory whose entries point at jump thunks
    /// to the Rust implementations (see `builtin_image`).
    fn add_builtin(&self, name: &str, dll: Option<&'static w::BuiltinDll>) -> Arc<Module> {
        use crate::builtin_image as bi;
        w::hitch::heartbeat_stage(w::hitch::HeartbeatKind::LoaderMap, &format!("builtin {name}"));
        let mut exports: Vec<bi::ImageExport> = Vec::new();
        if let Some(d) = dll {
            for e in d.exports {
                let ordinal = d.ordinals.iter().find(|(_, n)| *n == e.name).map(|(o, _)| *o);
                let target = e.ptr.0 as usize;
                exports.push(bi::ImageExport {
                    name: Some(e.name.to_string()),
                    ordinal,
                    target,
                    is_code: bi::is_host_code(target),
                });
            }
        }
        let size = bi::build(name, 0, &exports).size;
        let base = unsafe { bi::map_near_host(size) };
        let img = bi::build(name, base, &exports);
        assert_eq!(img.size, size);
        unsafe {
            std::ptr::copy_nonoverlapping(img.bytes.as_ptr(), base as *mut u8, img.bytes.len());
            #[cfg(target_arch = "x86_64")]
            {
                libc::mprotect(base as *mut libc::c_void, img.text.0, libc::PROT_READ);
                libc::mprotect(
                    (base + img.text.0) as *mut libc::c_void,
                    img.text.1 - img.text.0,
                    libc::PROT_READ | libc::PROT_EXEC,
                );
            }
        }
        if !img.unreachable_data.is_empty() {
            log::warn!(
                "{name}: data exports beyond 4 GB of the image base have no EAT entry (GetProcAddress still works): {:?}",
                img.unreachable_data
            );
        }
        {
            use win32_rt::kernel32::memory::{PAGE_EXECUTE_READ, PAGE_READONLY};
            let mut ranges = vec![
                (base, base + img.text.0, PAGE_READONLY),
                (base + img.text.0, base + img.text.1, PAGE_EXECUTE_READ),
            ];
            if img.text.1 < size {
                ranges.push((base + img.text.1, base + size, PAGE_READONLY));
            }
            w::kernel32::memory::register_image(base, size, &ranges);
        }
        if let Some(slot) = img.wow64_prepare_slot {
            w::unwind::set_wow64_prepare_for_exception_slot(slot);
        }
        let win_path = format!("C:\\Windows\\System32\\{name}");
        // Materialize the synthesized builtin PE image into the virtual System32 directory
        // so code that resolves a builtin module's on-disk path (e.g. via
        // NtQueryVirtualMemory(MemoryMappedFilenameInformation) + CreateFileW) can open
        // and inspect a valid PE64 image matching the in-memory sections and exports.
        if let Some(host_p) = w::path::to_host(&win_path) {
            if let Some(parent) = host_p.parent().filter(|p| p.is_dir()) {
                let tmp = parent.join(format!(".{name}.{}.tmp", std::process::id()));
                if std::fs::write(&tmp, &img.bytes).is_ok() {
                    if std::fs::rename(&tmp, &host_p).is_err() {
                        let _ = std::fs::remove_file(&tmp);
                    }
                }
            }
        }
        let m = Arc::new(Module {
            name: name.to_string(),
            win_path: win_path.clone(),
            host_path: None,
            base,
            size,
            kind: Kind::Builtin {
                dll,
                by_name: img.by_name.into_iter().collect(),
                by_ordinal: img.by_ordinal.into_iter().collect(),
                thunk_names: img.thunk_names.into_iter().collect(),
            },
            tls_callbacks: Vec::new(),
            initialized: AtomicBool::new(true),
            thread_calls: AtomicBool::new(false),
            refs: AtomicU32::new(1),
            ldr: AtomicUsize::new(0),
        });
        let e = unsafe { teb::ldr_add_module(base, 0, size as u32, &win_path, 0) };
        unsafe { teb::ldr_mark_initialized(e) };
        m.ldr.store(e as usize, Ordering::Relaxed);
        self.modules.write().unwrap().push(m.clone());
        log::debug!("builtin {name} at 0x{base:x}{}", if dll.is_none() { " (no implementation)" } else { "" });
        m
    }

    /// Map a PE image, relocate it, resolve its imports (recursively loading
    /// dependencies) and queue it for initialization.
    pub fn load_pe(&self, host: &Path, norm: &str, win_path: String, force_base: Option<usize>) -> Result<Arc<Module>> {
        self.load_pe_with_opts(host, norm, win_path, force_base, false, &LoadOptions::default())
    }

    /// Discover and map bundled x86-64 PE32+ plugins (`.dll`, `.pyd`, `.node`, `.ocx`)
    /// from `self.game_dir` and any manifest-declared `plugin_dirs` / `bundled_plugins`
    /// so offline `--prewarm-aot` / `--emit-static-o` precompiles them.
    ///
    /// Maps and resolves static imports ONLY — NEVER executes TLS callbacks or `DllMain`,
    /// and preserves lazy delay imports (missing optional delay-import DLLs are non-fatal).
    pub fn prewarm_map_bundled_plugins(
        &self,
        primary_exe: &Path,
        manifest: Option<&port_manifest::GameManifest>,
    ) -> Vec<Arc<Module>> {
        let _g = self.lock.lock();
        let plugin_dirs = manifest.map_or(&[][..], |m| m.plugin_dirs.as_slice());
        let bundled_plugins = manifest.map_or(&[][..], |m| m.bundled_plugins.as_slice());
        let discovered = port_manifest::discover_bundled_plugins(
            &self.game_dir,
            Some(&self.routing),
            |target| w::find_builtin(target).is_some(),
            plugin_dirs,
            bundled_plugins,
            Some(primary_exe),
        );
        let mut mapped = Vec::new();
        for host in discovered {
            if self.find_loaded_by_host_path(&host).is_some() {
                continue;
            }
            let Some(fname) = host.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let norm = pe_image::normalize_dll_name(fname);
            let win_path = w::path::to_win(&host).unwrap_or_else(|| {
                let rel = host
                    .strip_prefix(&self.game_dir)
                    .ok()
                    .map(|r| r.to_string_lossy().replace('/', "\\"));
                match rel {
                    Some(r) if !r.is_empty() => format!("{}\\{}", self.game_win_dir, r),
                    _ => format!("{}\\{}", self.game_win_dir, norm),
                }
            });
            let opts = LoadOptions {
                flags: 0,
                extra_search_dirs: Vec::new(),
                dll_load_dir: host.parent().map(Path::to_path_buf),
                static_import: true,
                prewarm: true,
            };
            match self.load_pe_with_opts(
                &host,
                &norm,
                win_path,
                x86_runtime::dll_native::pinned_base(&norm),
                false,
                &opts,
            ) {
                Ok(m) => {
                    log::info!(
                        "prewarm: mapped bundled plugin {} ({}) at 0x{:x}",
                        norm,
                        host.display(),
                        m.base
                    );
                    mapped.push(m);
                }
                Err(e) => {
                    log::warn!(
                        "prewarm: skipping bundled plugin {} ({}): {e:#}",
                        norm,
                        host.display()
                    );
                }
            }
        }
        mapped
    }

    /// Reserve `PROT_NONE` virtual address windows at boot for discovered bundled plugins
    /// that already have a cached `.aot` (or static/pinned) preferred base address.
    ///
    /// Dynamic `LoadLibrary` plugins often load after the game has allocated gigabytes of
    /// heap/GPU buffers that consume the low `0x1_5000_0000..0x1_7F00_0000` window used by
    /// offline `--prewarm-aot`. Holding a zero-RAM `PROT_NONE` reservation until the plugin
    /// is loaded lets `load_pe_with_opts` upgrade it in place via `mprotect` and hit its
    /// prewarmed `.aot` cache with zero fallbacks.
    pub fn reserve_bundled_plugin_aot_slots(
        &self,
        primary_exe: &Path,
        manifest: Option<&port_manifest::GameManifest>,
    ) -> usize {
        let _g = self.lock.lock();
        let plugin_dirs = manifest.map_or(&[][..], |m| m.plugin_dirs.as_slice());
        let bundled_plugins = manifest.map_or(&[][..], |m| m.bundled_plugins.as_slice());
        let discovered = port_manifest::discover_bundled_plugins(
            &self.game_dir,
            Some(&self.routing),
            |target| w::find_builtin(target).is_some(),
            plugin_dirs,
            bundled_plugins,
            Some(primary_exe),
        );
        let mut count = 0usize;
        for host in discovered {
            if self.find_loaded_by_host_path(&host).is_some() {
                continue;
            }
            let Some(fname) = host.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let norm = pe_image::normalize_dll_name(fname);
            if self.find_loaded(&norm).is_some() {
                continue;
            }
            let Some((_, size, is_dll)) = peek_pe_preferred_window(&host) else {
                continue;
            };
            if !is_dll {
                continue;
            }
            let Some(pref) = x86_runtime::dll_native::pinned_base(&norm)
                .or_else(|| x86_runtime::emu::exec_mem::static_aot_preferred_base(&norm))
                .or_else(|| x86_runtime::emu::aot_cached_preferred_base(&norm, size))
            else {
                continue;
            };
            if self.reserve_aot_slot(&norm, pref, size) {
                count += 1;
            }
        }
        count
    }

    fn reserve_aot_slot(&self, norm: &str, pref: usize, size: usize) -> bool {
        if pref == 0 || size == 0 {
            return false;
        }
        let mut slots = self.reserved_aot_slots.lock().unwrap();
        if slots.contains_key(norm) {
            return false;
        }
        let p = unsafe {
            libc::mmap(
                pref as *mut libc::c_void,
                size,
                libc::PROT_NONE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            )
        };
        if p == libc::MAP_FAILED {
            return false;
        }
        if p as usize != pref {
            unsafe { libc::munmap(p, size) };
            log::debug!(
                "loader: could not reserve cached AOT slot for {norm} at 0x{pref:x}+0x{size:x} (got 0x{:x})",
                p as usize
            );
            return false;
        }
        slots.insert(norm.to_string(), (pref, size));
        log::debug!("loader: reserved cached AOT slot for {norm} at 0x{pref:x}-0x{:x}", pref + size);
        true
    }

    fn take_reserved_aot_slot(&self, norm: &str, pref: usize, size: usize) -> Option<usize> {
        let (res_base, res_size) = self.reserved_aot_slots.lock().unwrap().remove(norm)?;
        if res_base == pref && res_size == size {
            let rc = unsafe {
                libc::mprotect(
                    res_base as *mut libc::c_void,
                    res_size,
                    libc::PROT_READ | libc::PROT_WRITE,
                )
            };
            if rc == 0 {
                return Some(res_base);
            }
        }
        unsafe { libc::munmap(res_base as *mut libc::c_void, res_size) };
        None
    }

    /// Verify before mapping that every static dependency DLL of `pe` (recursively for
    /// guest PE dependencies) exists either as an already-loaded module, on disk in the
    /// active DLL search path, or as a builtin. Matches Windows `LoadLibrary` returning
    /// `ERROR_MOD_NOT_FOUND` when an optional plugin DLL imports an unavailable dependency.
    fn check_strict_deps(
        &self,
        norm: &str,
        pe: &Pe<'_>,
        dep_opts: &LoadOptions,
        seen: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        for imp in pe.imports()? {
            let dep_norm = pe_image::normalize_dll_name(&imp.dll);
            if !seen.insert(dep_norm.clone()) || self.find_loaded(&dep_norm).is_some() {
                continue;
            }
            match self.classify_with_opts(&imp.dll, dep_opts, 0) {
                Some(Source::Builtin(_)) => {}
                Some(Source::GameDir(dep_path)) => {
                    let dep_data = std::fs::read(&dep_path)
                        .with_context(|| format!("{norm}: reading dependency {}", dep_path.display()))?;
                    let dep_pe = Pe::parse(&dep_data)
                        .with_context(|| format!("{norm}: parsing dependency {}", dep_path.display()))?;
                    self.check_strict_deps(&dep_norm, &dep_pe, dep_opts, seen)?;
                }
                None => bail!("{norm}: missing required dependency DLL {:?}", imp.dll),
            }
        }
        Ok(())
    }

    fn load_pe_with_opts(
        &self,
        host: &Path,
        norm: &str,
        win_path: String,
        force_base: Option<usize>,
        strict_deps: bool,
        opts: &LoadOptions,
    ) -> Result<Arc<Module>> {
        use w::kernel32::module::*;

        w::hitch::heartbeat_stage(w::hitch::HeartbeatKind::LoaderMap, &format!("map {norm}"));
        let data = std::fs::read(host).with_context(|| format!("reading {}", host.display()))?;
        let pe = Pe::parse(&data).with_context(|| format!("parsing {}", host.display()))?;
        let h = &pe.headers;
        if h.machine != IMAGE_FILE_MACHINE_AMD64 {
            bail!("{norm}: not an x86-64 image (machine 0x{:x})", h.machine);
        }
        let dont_resolve = (opts.flags & DONT_RESOLVE_DLL_REFERENCES) != 0;

        let mut dep_opts = LoadOptions {
            flags: opts.flags & !(DONT_RESOLVE_DLL_REFERENCES | DATAFILE_LOAD_FLAGS_MASK),
            extra_search_dirs: opts.extra_search_dirs.clone(),
            dll_load_dir: opts.dll_load_dir.clone(),
            static_import: true,
            prewarm: opts.prewarm,
        };
        let propagate_own_dir = opts.prewarm
            || (opts.flags & (LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_WITH_ALTERED_SEARCH_PATH)) != 0;
        if propagate_own_dir {
            if let Some(parent) = host.parent() {
                dep_opts.dll_load_dir = Some(parent.to_path_buf());
            }
        }

        if !dont_resolve && strict_deps && h.is_dll() {
            let mut seen = std::collections::HashSet::from([norm.to_string()]);
            self.check_strict_deps(norm, &pe, &dep_opts, &mut seen)?;
        }
        let size = (h.size_of_image as usize + PAGE - 1) & !(PAGE - 1);
        let pref = force_base
            .or_else(|| x86_runtime::emu::exec_mem::static_aot_preferred_base(norm))
            .or_else(|| x86_runtime::emu::aot_cached_preferred_base(norm, size))
            .unwrap_or(h.image_base as usize);
        let base = self
            .take_reserved_aot_slot(norm, pref, size)
            .or_else(|| unsafe { map_image_memory(pref, size, !h.relocs_stripped()) })
            .ok_or_else(|| anyhow!("{norm}: cannot map 0x{size:x} bytes (preferred 0x{pref:x})"))?;
        let img = unsafe { std::slice::from_raw_parts_mut(base as *mut u8, size) };
        // Headers + sections.
        let hdr = (h.size_of_headers as usize).min(data.len()).min(size);
        img[..hdr].copy_from_slice(&data[..hdr]);
        let mut exec_ranges = Vec::new();
        for s in &pe.sections {
            let n = (s.raw_size.min(s.mem_size()) as usize).min(data.len().saturating_sub(s.raw_offset as usize));
            let dst = s.rva as usize;
            if n > 0 && dst + n <= size {
                img[dst..dst + n].copy_from_slice(&data[s.raw_offset as usize..s.raw_offset as usize + n]);
            }
            if s.is_exec() {
                let end = s.rva.saturating_add(s.mem_size().max(s.raw_size));
                if end > s.rva {
                    exec_ranges.push((s.rva, end));
                }
            }
        }
        let delta = base as i64 - h.image_base as i64;
        if delta != 0 {
            let relocs = pe.relocations()?;
            let n = pe_image::apply_relocations(img, &relocs, delta)?;
            // Keep OptionalHeader.ImageBase consistent with the actual base.
            let e_lfanew = u32::from_le_bytes(img[0x3C..0x40].try_into().unwrap()) as usize;
            img[e_lfanew + 24 + 24..e_lfanew + 24 + 32].copy_from_slice(&(base as u64).to_le_bytes());
            log::info!("{norm}: relocated 0x{:x} -> 0x{base:x} ({n} fixups)", h.image_base);
        }
        let exports = pe.exports().unwrap_or_default();
        let mut pdata = pe.runtime_functions().unwrap_or_default();
        pdata.sort_by_key(|f| f.begin_rva);
        let tls_dir = if dont_resolve { None } else { pe.tls()? };
        let tls_callbacks: Vec<usize> = if dont_resolve {
            Vec::new()
        } else {
            pe.tls_callback_rvas()?.into_iter().map(|r| base + r as usize).collect()
        };
        let tls = tls_dir.map(|t| {
            let idx = self.next_tls.fetch_add(1, Ordering::Relaxed);
            let reloc = |va: u64| (va as i64 + delta) as usize;
            let align_bits = (t.characteristics >> 20) & 0xF;
            let align = if align_bits == 0 { 16 } else { (1usize << (align_bits - 1)).max(16) };
            unsafe { std::ptr::write_unaligned(reloc(t.index_va) as *mut u32, idx) };
            Tls {
                index: idx,
                template_va: reloc(t.start_raw_data_va),
                template_len: t.template_size(),
                zero_fill: t.size_of_zero_fill as usize,
                align,
            }
        });
        let entry = if h.entry_rva != 0 { base + h.entry_rva as usize } else { 0 };
        let host_canon = std::fs::canonicalize(host).unwrap_or_else(|_| host.to_path_buf());
        let m = Arc::new(Module {
            name: norm.to_string(),
            win_path: win_path.clone(),
            host_path: Some(host_canon),
            base,
            size,
            kind: Kind::Pe {
                exports,
                pdata,
                tls,
                entry,
                is_dll: h.is_dll(),
                exec_ranges,
            },
            tls_callbacks,
            initialized: AtomicBool::new(dont_resolve),
            thread_calls: AtomicBool::new(!dont_resolve),
            refs: AtomicU32::new(1),
            ldr: AtomicUsize::new(0),
        });
        let tls_index = match &m.kind {
            Kind::Pe { tls: Some(t), .. } => t.index as u16,
            _ => 0,
        };
        let e = unsafe { teb::ldr_add_module(base, entry, size as u32, &win_path, tls_index) };
        m.ldr.store(e as usize, Ordering::Relaxed);
        // Register before resolving imports so cycles terminate.
        self.modules.write().unwrap().push(m.clone());
        if !h.is_dll() {
            let _ = self.exe.set(m.clone());
            // Real Windows processes always map ntdll, kernel32, and kernelbase
            // right after the main executable in PEB->Ldr before static imports.
            for core in ["ntdll.dll", "kernel32.dll", "kernelbase.dll"] {
                let _ = self.load(core, false);
            }
        }
        log::info!(
            "mapped {norm} at 0x{base:x}-0x{:x}{}{}",
            base + size,
            if tls_index != 0 || matches!(&m.kind, Kind::Pe { tls: Some(_), .. }) { format!(" (TLS index {tls_index})") } else { String::new() },
            if dont_resolve { " (DONT_RESOLVE_DLL_REFERENCES)" } else { "" }
        );
        if !dont_resolve {
            if h.is_dll() && !std::env::var("AW2_DLL_NATIVE").is_ok_and(|v| v == "0") {
                let n = unsafe { x86_runtime::dll_native::install(norm, base, size) };
                if n != 0 {
                    log::info!("{norm}: {n} native leaf replacement(s)");
                }
            }

            // Imports (delay imports stay strictly lazy: the image's own delay-load helper
            // calls LoadLibrary/GetProcAddress when first used).
            for imp in pe.imports()? {
                let dep = self.load_with_opts(&imp.dll, &dep_opts)?;
                for ent in &imp.entries {
                    let addr = self.resolve_import(&dep, &imp.dll, &ent.name, norm);
                    unsafe { std::ptr::write_unaligned((base + ent.iat_rva as usize) as *mut u64, addr as u64) };
                }
            }
            // Exports that only `jmp [IAT]` now dispatch straight to their bound import.
            if h.is_dll() && !std::env::var("AW2_DLL_NATIVE").is_ok_and(|v| v == "0") {
                unsafe { x86_runtime::dll_native::imports_bound(norm, base, size) };
            }
            // AOT translation cache / JIT profile: the image is relocated and import-bound
            // (caches are loaded at the next `run_init_since`, see `x86_runtime::emu::aot_activate`).
            x86_runtime::emu::aot_module_mapped(norm, base, size);
        }

        // Section protections. Under Rosetta the host page size is 4 KB, the
        // same as the PE section alignment. On native Apple Silicon (`aarch64`)
        // the host page size is 16 KB (`0x4000`) and mapped PE sections are read/written
        // as guest data rather than executed directly as ARM64 code, so keep host pages RW
        // while recording exact 4 KB Windows section protections in `register_image`.
        let mut ranges = Vec::new();
        #[cfg(target_arch = "x86_64")]
        unsafe {
            libc::mprotect(base as *mut libc::c_void, hdr.max(PAGE), libc::PROT_READ);
        }
        for s in &pe.sections {
            let start = base + s.rva as usize;
            let len = (s.mem_size() as usize + PAGE - 1) & !(PAGE - 1);
            if len == 0 {
                continue;
            }
            let mut ch = s.characteristics;
            if ch & IMAGE_SCN_MEM_DISCARDABLE != 0 && ch & (IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE) == 0 {
                ch |= IMAGE_SCN_MEM_READ;
            }
            #[cfg(target_arch = "x86_64")]
            unsafe {
                libc::mprotect(start as *mut libc::c_void, len, host_prot(ch));
            }
            #[cfg(not(target_arch = "x86_64"))]
            let _ = host_prot(ch);
            ranges.push((start, start + len, win_prot(ch)));
        }
        w::kernel32::memory::register_image(base, size, &ranges);

        if let Kind::Pe { tls: Some(_), .. } = &m.kind {
            self.tls_modules.write().unwrap().push(m.clone());
            // Threads that already exist get a block too (runtime LoadLibrary).
            for t in w::thread::all_tebs() {
                unsafe { self.tls_add_block(t, &m) };
            }
        }
        if !dont_resolve {
            let seq = self.pending_seq.fetch_add(1, Ordering::Relaxed);
            self.pending_init.lock().unwrap().push((seq, m.clone()));
        }
        Ok(m)
    }

    /// True if exported address `addr` on `m` refers to executable code rather than
    /// exported data (used to ensure `--trace` never wraps data exports in a code thunk).
    fn is_code_export(m: &Module, addr: usize) -> bool {
        match &m.kind {
            Kind::Pe { exec_ranges, .. } => {
                if addr < m.base || addr >= m.base + m.size {
                    return crate::builtin_image::is_host_code(addr);
                }
                let rva = (addr - m.base) as u32;
                exec_ranges.iter().any(|&(s, e)| rva >= s && rva < e)
            }
            Kind::Builtin { thunk_names, .. } => {
                thunk_names.contains_key(&addr) || crate::builtin_image::is_host_code(addr)
            }
        }
    }

    /// Resolve one import to an address (implementation, forwarder target,
    /// or a trap thunk). Wraps code imports in a trace thunk with `--trace`,
    /// while preserving exact data-export addresses.
    fn resolve_import(&self, dep: &Arc<Module>, dll: &str, name: &ImportName, importer: &str) -> usize {
        let pname = match name {
            ImportName::Name { name, .. } => ProcName::Name(name.as_str()),
            ImportName::Ordinal(o) => ProcName::Ordinal(*o),
        };
        match self.proc_address(dep, pname, 0) {
            Some(a) => {
                if Self::is_code_export(dep, a) {
                    self.maybe_trace(dll, &name.to_string(), importer, a)
                } else {
                    a
                }
            }
            None => {
                self.traps.fetch_add(1, Ordering::Relaxed);
                self.unresolved.lock().unwrap().insert(format!("{}!{name}", dep.name));
                log::debug!("unresolved import {dll}!{name} (by {importer}) -> trap");
                thunks::trap_thunk(&dep.name, &name.to_string(), importer)
            }
        }
    }

    fn maybe_trace(&self, dll: &str, name: &str, importer: &str, target: usize) -> usize {
        if !self.trace {
            return target;
        }
        let key = (pe_image::normalize_dll_name(dll), name.to_string());
        let mut c = self.trace_cache.lock().unwrap();
        *c.entry(key).or_insert_with(|| thunks::trace_thunk(dll, name, importer, target))
    }

    /// Resolve an exported function on `m` by name or `#ordinal`.
    pub fn resolve_export(&self, m: &Arc<Module>, spec: &str) -> Option<usize> {
        if let Some(ord_str) = spec.strip_prefix('#') {
            let ord: u16 = ord_str.parse().ok()?;
            self.proc_address(m, ProcName::Ordinal(ord), 0)
        } else {
            self.proc_address(m, ProcName::Name(spec), 0)
        }
    }

    /// List all direct RVA exports on a PE module as `(ordinal, name, address)`.
    pub fn export_list(&self, m: &Arc<Module>) -> Vec<(u32, String, usize)> {
        let Kind::Pe { exports, .. } = &m.kind else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for e in &exports.entries {
            if let ExportTarget::Rva(rva) = e.target {
                let name = e
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("#{}", e.ordinal));
                out.push((e.ordinal, name, m.base + rva as usize));
            }
        }
        out
    }

    /// Export lookup on a loaded module, following forwarders.
    fn proc_address(&self, m: &Arc<Module>, name: ProcName<'_>, depth: u32) -> Option<usize> {
        if depth > 8 {
            log::error!("forwarder chain too deep at {}!{name}", m.name);
            return None;
        }
        match &m.kind {
            Kind::Builtin { dll: None, .. } => None,
            Kind::Builtin { dll: Some(dll), by_name, by_ordinal, .. } => match name {
                ProcName::Name(n) => by_name.get(n).copied().or_else(|| w::lookup(&m.name, n).map(|p| p as usize)),
                ProcName::Ordinal(o) => by_ordinal
                    .get(&o)
                    .copied()
                    .or_else(|| {
                        dll.ordinals
                            .iter()
                            .find(|(ord, _)| *ord == o)
                            .and_then(|(_, n)| by_name.get(*n).copied())
                    })
                    .or_else(|| w::lookup_ordinal(&m.name, o).map(|p| p as usize)),
            },
            Kind::Pe { exports, .. } => {
                // Native arm64: the game's own vcruntime140.dll is x86 code that would run in
                // the interpreter. Its pure mem* routines are the hottest guest entry points
                // (every memcpy from game code), so serve them from host implementations.
                #[cfg(not(target_arch = "x86_64"))]
                if let ProcName::Name(n) = name {
                    if m.name == "vcruntime140.dll"
                        && matches!(
                            n,
                            "memcpy"
                                | "memmove"
                                | "memset"
                                | "memcmp"
                                | "memchr"
                                | "strchr"
                                | "strstr"
                                | "wcsrchr"
                                | "wcsstr"
                        )
                    {
                        if let Some(p) = w::lookup("msvcrt.dll", n) {
                            return Some(p as usize);
                        }
                    }
                    if m.name == "msvcp140.dll" && n == "_Thrd_yield" {
                        if let Some(p) = w::lookup("kernel32.dll", "SwitchToThread") {
                            return Some(p as usize);
                        }
                    }
                }
                let e = match name {
                    ProcName::Name(n) => exports.by_name(n),
                    ProcName::Ordinal(o) => exports.by_ordinal(o as u32),
                }?;
                match &e.target {
                    ExportTarget::Rva(r) => Some(m.base + *r as usize),
                    ExportTarget::Forwarder(f) => {
                        let (dll, sym) = f.split_once('.')?;
                        let dep = self.load(dll, true).ok()?;
                        if let Some(o) = sym.strip_prefix('#') {
                            self.proc_address(&dep, ProcName::Ordinal(o.parse().ok()?), depth + 1)
                        } else {
                            self.proc_address(&dep, ProcName::Name(sym), depth + 1)
                        }
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    /// Run TLS callbacks + DllMain(PROCESS_ATTACH) for every module queued by
    /// `load_pe`, in dependency order. The exe is skipped (see `run_exe`).
    /// Returns false if a DllMain failed.
    pub fn run_pending_init(&self, static_load: bool) -> bool {
        self.run_init_since(0, static_load)
    }

    /// Current pending-init sequence number (see `run_init_since`).
    pub fn pending_mark(&self) -> u64 {
        self.pending_seq.load(Ordering::Relaxed)
    }

    /// Initialize pending DLLs queued at or after `mark`.
    pub fn run_init_since(&self, mark: u64, static_load: bool) -> bool {
        let _g = self.lock.lock();
        // AOT translation caches of the images mapped since the last call (before any
        // DllMain runs; the AW2 registry is applied by now).
        x86_runtime::emu::aot_activate();
        loop {
            let next = {
                let mut p = self.pending_init.lock().unwrap();
                let i = p.iter().position(|(seq, m)| *seq >= mark && m.is_dll());
                i.map(|i| p.remove(i).1)
            };
            let Some(m) = next else { return true };
            if m.initialized.swap(true, Ordering::AcqRel) {
                continue;
            }
            w::hitch::heartbeat_stage(
                w::hitch::HeartbeatKind::LoaderDllInit,
                &format!("DllMain({})", m.name),
            );
            let reserved = if static_load { 1usize } else { 0 };
            for &cb in &m.tls_callbacks {
                log::debug!("{}: TLS callback {} (PROCESS_ATTACH)", m.name, w::symbolize(cb));
                unsafe { call_dll_entry(cb, m.base, DLL_PROCESS_ATTACH, reserved) };
            }
            let entry = m.entry();
            if entry != 0 {
                log::info!("{}: DllMain(PROCESS_ATTACH)", m.name);
                let ok = unsafe { call_dll_entry(entry, m.base, DLL_PROCESS_ATTACH, reserved) };
                if ok == 0 {
                    log::error!("{}: DllMain(PROCESS_ATTACH) returned FALSE", m.name);
                    return false;
                }
                log::debug!("{}: DllMain done", m.name);
            }
            // V8 9.4 `--jitless` injection: honor manifest/plugin `v8_jitless` default and
            // `PORT_V8_JITLESS` / `AW2_V8_JITLESS` environment overrides (including `=0` opt-out).
            if m.name == "v8.dll" && self.should_apply_v8_jitless() {
                self.apply_v8_jitless(&m);
            }
            let e = m.ldr.load(Ordering::Relaxed);
            if e != 0 {
                unsafe { teb::ldr_mark_initialized(e as *mut teb::LdrEntry) };
            }
        }
    }

    fn should_apply_v8_jitless(&self) -> bool {
        let env_val = std::env::var("PORT_V8_JITLESS")
            .or_else(|_| std::env::var("AW2_V8_JITLESS"))
            .ok();
        should_apply_v8_jitless_with(self.v8_jitless, env_val.as_deref())
    }

    /// Default (`v8_jitless = true` or `PORT_V8_JITLESS=1`, unless disabled with `=0`): pass
    /// `--jitless` to the game's V8 (9.4.146.24) through its exported
    /// `v8::V8::SetFlagsFromString(const char*, size_t)`, after v8.dll's DllMain and before
    /// cohtml calls `v8::V8::Initialize`. `V8::InitializeOncePerProcessImpl` then runs
    /// `FlagList::EnforceFlagImplications` (no-opt, no-sparkplug, regexp_interpret_all,
    /// no-write_code_using_rwx, ...) and disables `expose_wasm`. In jitless mode V8 commits its
    /// code space RW/R only and reserves no code range, so it never makes guest pages executable
    /// and generates no x86 code for the runtime JIT. This build has no flag freezing.
    fn apply_v8_jitless(&self, m: &Arc<Module>) {
        let set_flags = self.resolve_export(m, V8_SET_FLAGS_FROM_STRING);
        let flag = self.resolve_export(m, V8_FLAG_JITLESS);
        let (Some(set_flags), Some(flag)) = (set_flags, flag) else {
            log::warn!(
                "v8.dll: {V8_SET_FLAGS_FROM_STRING} or {V8_FLAG_JITLESS} is not exported; --jitless not applied"
            );
            return;
        };
        let arg: &'static [u8] = V8_JITLESS_ARG;
        unsafe { call_guest_ptr_len(set_flags, arg.as_ptr() as usize, arg.len()) };
        let v = unsafe { std::ptr::read_volatile(flag as *const u8) };
        if v == 1 {
            log::info!("v8.dll: --jitless applied via v8::V8::SetFlagsFromString (FLAG_jitless=1, AW2_V8_JITLESS)");
        } else {
            log::warn!("v8.dll: SetFlagsFromString(\"--jitless\") left FLAG_jitless={v}; --jitless not applied");
        }
    }

    /// Exe TLS callbacks (PROCESS_ATTACH) then the entry point. Runs on the
    /// initial Windows thread.
    pub fn run_exe(&self) -> u32 {
        let exe = self.exe.get().expect("no exe mapped").clone();
        self.pending_init.lock().unwrap().retain(|(_, m)| !Arc::ptr_eq(m, &exe));
        exe.initialized.store(true, Ordering::Release);
        // Windows `ntdll!LdrpInitializeProcess` invokes `NtTestAlert` after
        // static DLL initialization (`DllMain(PROCESS_ATTACH)`), delivering any
        // APCs queued onto the startup thread (e.g., `overwatch_loader.dll`'s
        // `QueueUserAPC` unpacker callback) before running exe entry code.
        if w::thread::has_pending_apc() {
            log::info!("draining pending startup thread APCs (NtTestAlert)");
            w::thread::run_pending_apcs();
        }
        for &cb in &exe.tls_callbacks {
            log::info!("exe TLS callback {} (PROCESS_ATTACH)", w::symbolize(cb));
            unsafe { call_dll_entry(cb, exe.base, DLL_PROCESS_ATTACH, 0) };
        }
        if w::thread::has_pending_apc() {
            log::info!("draining pending post-TLS APCs (NtTestAlert)");
            w::thread::run_pending_apcs();
        }
        let entry = exe.entry();
        w::hitch::heartbeat_stage(
            w::hitch::HeartbeatKind::LoaderEntry,
            &format!("entry {}", exe.name),
        );
        log::info!("calling exe entry point {}", w::symbolize(entry));
        #[cfg(target_arch = "x86_64")]
        {
            let f: unsafe extern "win64" fn(usize) -> u32 = unsafe { std::mem::transmute(entry) };
            // mainCRTStartup ignores its argument (PEB on real Windows).
            unsafe { f(teb::peb() as usize) }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            unsafe { x86_runtime::exe::call_va8(entry as u64, teb::peb() as u64, 0, 0, 0, 0, 0, 0, 0) as u32 }
        }
    }

    // -----------------------------------------------------------------------
    // Static TLS
    // -----------------------------------------------------------------------

    unsafe fn tls_add_block(&self, t: *mut u8, m: &Module) {
        let Kind::Pe { tls: Some(tls), .. } = &m.kind else { return };
        let mut arr = teb::field::<usize>(t, teb::off::TLS_POINTER) as *mut *mut u8;
        if arr.is_null() {
            arr = libc::calloc(TLS_ARRAY_CAP, 8) as *mut *mut u8;
            teb::set_field(t, teb::off::TLS_POINTER, arr as usize);
            if t == teb::current_raw() {
                teb::set_tls_pointer(t, arr);
            }
        }
        if tls.index as usize >= TLS_ARRAY_CAP {
            log::error!("TLS index {} exceeds capacity {TLS_ARRAY_CAP}", tls.index);
            return;
        }
        let total = (tls.template_len + tls.zero_fill).max(8);
        let mut p: *mut libc::c_void = std::ptr::null_mut();
        libc::posix_memalign(&mut p, tls.align, total);
        let p = p as *mut u8;
        std::ptr::copy_nonoverlapping(tls.template_va as *const u8, p, tls.template_len);
        std::ptr::write_bytes(p.add(tls.template_len), 0, total - tls.template_len);
        *arr.add(tls.index as usize) = p;
    }

    // -----------------------------------------------------------------------
    // Symbolization
    // -----------------------------------------------------------------------

    pub fn symbolize_addr(&self, addr: usize) -> String {
        if let Some((dll, name)) = w::builtin_name_for(addr) {
            return format!("{dll}!{name} [builtin]");
        }
        if let Some(page) = self.wsa_ext_thunks.get() {
            if let Some(name) = page.symbol_for(addr) {
                return format!("{name} [wsa ext thunk]");
            }
        }
        if let Some(m) = self.module_at(addr) {
            if let Kind::Builtin { thunk_names, .. } = &m.kind {
                if let Some(n) = thunk_names.get(&(addr & !(crate::builtin_image::THUNK_SIZE - 1))) {
                    return format!("{}!{n} [builtin thunk]", m.name);
                }
            }
            let rva = (addr - m.base) as u32;
            let mut s = format!("{}+0x{rva:x}", m.name);
            if let Kind::Pe { exports, .. } = &m.kind {
                let sym = crate::symbols::get().and_then(|t| t.for_image(&m.name)).and_then(|t| t.lookup(rva).map(|h| h.to_string()));
                if sym.is_none() {
                    if let Some((n, off)) = exports.nearest(rva) {
                        if off < 0x10000 {
                            s.push_str(&format!(" ({n}+0x{off:x})"));
                        }
                    }
                }
                if !m.is_dll() {
                    s.push_str(&format!(
                        " [va 0x{:x}]",
                        x86_runtime::exe::preferred_base() + rva as u64
                    ));
                }
                if let Some(name) = sym {
                    s.push(' ');
                    s.push_str(&name);
                }
            }
            return s;
        }
        // Host code (the loader binary / win32-rt / system libraries).
        // Cache `dladdr` + demangling results so hitch reports don't repeatedly
        // linear-scan the Mach-O symbol table.
        static HOST_SYM_CACHE: std::sync::OnceLock<Mutex<HashMap<usize, String>>> =
            std::sync::OnceLock::new();
        let cache = HOST_SYM_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        if let Some(cached) = cache.lock().unwrap().get(&addr).cloned() {
            return cached;
        }
        let resolved = unsafe {
            let mut info: libc::Dl_info = std::mem::zeroed();
            if libc::dladdr(addr as *const libc::c_void, &mut info) != 0 && !info.dli_sname.is_null() {
                let n = std::ffi::CStr::from_ptr(info.dli_sname).to_string_lossy();
                let lib = if info.dli_fname.is_null() {
                    String::new()
                } else {
                    let f = std::ffi::CStr::from_ptr(info.dli_fname).to_string_lossy();
                    format!("{}!", f.rsplit('/').next().unwrap_or(&f))
                };
                format!(
                    "0x{addr:x} [host {lib}{}+0x{:x}]",
                    crate::symbols::demangle(&n),
                    addr - info.dli_saddr as usize
                )
            } else {
                format!("0x{addr:x}")
            }
        };
        cache.lock().unwrap().insert(addr, resolved.clone());
        resolved
    }
}

/// `v8::V8::SetFlagsFromString(const char*, size_t)` (the overload cohtml imports).
const V8_SET_FLAGS_FROM_STRING: &str = "?SetFlagsFromString@V8@v8@@SAXPEBD_K@Z";
/// `bool v8::internal::FLAG_jitless` (exported data, .bss of v8.dll).
const V8_FLAG_JITLESS: &str = "?FLAG_jitless@internal@v8@@3_NA";
/// Argument for `SetFlagsFromString`. V8 copies it into its own buffer before parsing.
static V8_JITLESS_ARG: &[u8] = b"--jitless";

/// `AW2_V8_JITLESS`: on by default (unset or anything else); `0`/`false`/`off`/`no` disables
/// (A/B only). Jitless removes V8's run-time generated x86 code (A/B jl1: fps-neutral at
/// 1280x720 render, errors 0, black 0%).
fn v8_jitless_enabled(v: Option<&str>) -> bool {
    !v.map(str::trim)
        .is_some_and(|v| v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("off") || v.eq_ignore_ascii_case("no"))
}

fn should_apply_v8_jitless_with(configured: bool, env_val: Option<&str>) -> bool {
    if let Some(raw) = env_val {
        let t = raw.trim();
        if !t.is_empty() {
            return v8_jitless_enabled(Some(t));
        }
    }
    configured
}

/// Call a guest `void f(const char*, size_t)` (Win64 ABI).
unsafe fn call_guest_ptr_len(f: usize, p: usize, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(usize, usize) = std::mem::transmute(f);
        f(p, len)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        x86_runtime::exe::call_va8(f as u64, p as u64, len as u64, 0, 0, 0, 0, 0, 0);
    }
}

unsafe fn call_dll_entry(f: usize, hinst: usize, reason: u32, reserved: usize) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(usize, u32, usize) -> i32 = std::mem::transmute(f);
        f(hinst, reason, reserved)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        x86_runtime::exe::call_va8(f as u64, hinst as u64, reason as u64, reserved as u64, 0, 0, 0, 0, 0) as i32
    }
}

/// Reserve image memory, preferring `pref`. Returns `None` if the image
/// cannot be relocated and `pref` is unavailable.
unsafe fn map_image_memory(pref: usize, size: usize, relocatable: bool) -> Option<usize> {
    let rw = libc::PROT_READ | libc::PROT_WRITE;
    let flags = libc::MAP_PRIVATE | libc::MAP_ANON;
    let p = libc::mmap(pref as *mut libc::c_void, size, rw, flags, -1, 0);
    if p != libc::MAP_FAILED && p as usize == pref {
        return Some(pref);
    }
    if p != libc::MAP_FAILED {
        libc::munmap(p, size);
    }
    if !relocatable {
        log::error!("image wants fixed base 0x{pref:x} which is unavailable");
        return None;
    }
    // Prefer a deterministic slot in the low iOS/macOS-safe 5..6 GiB window
    // (`0x1_5000_0000..0x1_7F00_0000`, safely above main Mach-O ASLR and below
    // dyld shared cache at `0x1_8000_0000`) so offline `--emit-static-o` builds
    // and iOS device/simulator launches land at the exact same base address even
    // without `extended-virtual-addressing`.
    let mut hint = 0x1_5000_0000usize;
    let step = (size + 0x0010_0000 - 1) & !(0x0010_0000 - 1);
    while hint + size <= 0x1_7F00_0000 {
        let p = libc::mmap(hint as *mut libc::c_void, size, rw, flags, -1, 0);
        if p != libc::MAP_FAILED {
            if p as usize == hint {
                return Some(hint);
            }
            libc::munmap(p, size);
        }
        hint += step.max(0x0010_0000);
    }
    // Fallback: let the kernel pick any free range.
    let p = libc::mmap(std::ptr::null_mut(), size, rw, flags, -1, 0);
    (p != libc::MAP_FAILED).then_some(p as usize)
}

// ---------------------------------------------------------------------------
// LoaderOps
// ---------------------------------------------------------------------------

impl w::LoaderOps for Loader {
    fn load_library(&self, name: &str, flags: u32) -> Result<usize, u32> {
        self.load_library_with_search_path(name, flags, None)
    }

    fn load_library_with_search_path(
        &self,
        name: &str,
        flags: u32,
        search_path: Option<&str>,
    ) -> Result<usize, u32> {
        use w::kernel32::module::*;

        validate_load_library_flags(name, flags)?;
        let extra_search_dirs = search_path
            .map(parse_search_path_dirs)
            .unwrap_or_default();
        let opts = LoadOptions {
            flags,
            extra_search_dirs,
            dll_load_dir: None,
            static_import: false,
            prewarm: false,
        };

        if (flags & DATAFILE_LOAD_FLAGS_MASK) != 0 {
            return match self.load_datafile(name, &opts) {
                Ok(h) => Ok(h),
                Err(e) => {
                    log::warn!("LoadLibraryEx({name:?}, datafile 0x{flags:x}) failed: {e:#}");
                    Err(w::types::err::ERROR_MOD_NOT_FOUND)
                }
            };
        }

        let _g = self.lock.lock();
        let mark = self.pending_mark();
        match self.load_with_opts(name, &opts) {
            Ok(m) => {
                if (flags & DONT_RESOLVE_DLL_REFERENCES) == 0 && !self.run_init_since(mark, false) {
                    return Err(w::types::err::ERROR_DLL_INIT_FAILED);
                }
                log::info!("LoadLibrary({name:?}, 0x{flags:x}) -> {} 0x{:x}", m.name, m.base);
                Ok(m.base)
            }
            Err(e) => {
                log::warn!("LoadLibrary({name:?}, 0x{flags:x}) failed: {e:#}");
                Err(w::types::err::ERROR_MOD_NOT_FOUND)
            }
        }
    }

    fn free_library(&self, hmod: usize) -> bool {
        if hmod & 3 != 0 {
            let mut dfs = self.datafile_modules.write().unwrap();
            if let Some(pos) = dfs.iter().position(|d| d.handle == hmod) {
                let prev = dfs[pos].refs.fetch_sub(1, Ordering::AcqRel);
                if prev <= 1 {
                    let df = dfs.remove(pos);
                    if df.owned_mmap {
                        unsafe {
                            libc::munmap(df.base as *mut libc::c_void, df.size);
                        }
                    }
                }
                return true;
            }
            return false;
        }
        match self.module_by_base(hmod) {
            Some(m) => {
                // Images are never unmapped (DLL_PROCESS_DETACH is not run on
                // FreeLibrary either): keeps code pointers held elsewhere valid.
                m.refs.fetch_sub(1, Ordering::Relaxed);
                true
            }
            None => false,
        }
    }

    fn get_module_handle(&self, name: Option<&str>) -> Option<usize> {
        let Some(name) = name else { return self.exe.get().map(|m| m.base) };
        let norm = pe_image::normalize_dll_name(name);
        if self.routing.is_disabled(&norm) {
            return None;
        }
        if let Some(m) = self.find_loaded(&norm) {
            return Some(m.base);
        }
        // ntdll/kernel32/kernelbase are always loaded on Windows.
        if matches!(norm.as_str(), "kernel32.dll" | "kernelbase.dll" | "ntdll.dll") {
            return self.load(&norm, false).ok().map(|m| m.base);
        }
        let routed = self.routing.resolve_target_dll(&norm);
        if self.routing.is_disabled(&routed) {
            return None;
        }
        if let Some(m) = self.find_loaded(&routed) {
            return Some(m.base);
        }
        let canon = w::canonical_dll(&routed);
        if let Some(m) = self.find_loaded(&canon) {
            return Some(m.base);
        }
        None
    }

    fn get_proc_address(&self, hmod: usize, name: ProcName<'_>) -> Option<usize> {
        if hmod & 3 != 0 {
            // Datafile / image-resource handles cannot resolve procedures.
            return None;
        }
        let m = self.module_by_base(hmod)?;
        let a = self.proc_address(&m, name, 0)?;
        if Self::is_code_export(&m, a) {
            Some(self.maybe_trace(&m.name, &name.to_string(), "GetProcAddress", a))
        } else {
            Some(a)
        }
    }

    fn wsa_extension_thunk(&self, ext: w::WsaExtension) -> Option<usize> {
        let page = self.wsa_ext_thunks.get_or_init(|| {
            let entries: Vec<(&'static str, usize)> = w::WsaExtension::ALL
                .iter()
                .map(|&e| (e.symbol_name(), w::misc::ws2_32::wsa_extension_target(e)))
                .collect();
            crate::builtin_image::build_fixed_thunk_page(&entries)
        });
        page.thunks.get(ext as usize).map(|&(va, _)| va)
    }

    fn module_path(&self, hmod: Option<usize>) -> Option<String> {
        match hmod {
            None => self.exe.get().map(|m| m.win_path.clone()),
            Some(h) => self.module_by_base(h).map(|m| m.win_path.clone()),
        }
    }

    fn module_from_address(&self, addr: usize) -> Option<usize> {
        self.module_at(addr).map(|m| m.base)
    }

    fn is_guest_pe_code(&self, addr: usize) -> bool {
        let Some(m) = self.module_at(addr) else {
            return false;
        };
        match &m.kind {
            Kind::Pe { exec_ranges, .. } => {
                let Some(rva) = addr.checked_sub(m.base) else {
                    return false;
                };
                let Ok(rva32) = u32::try_from(rva) else {
                    return false;
                };
                exec_ranges.iter().any(|&(s, e)| rva32 >= s && rva32 < e)
            }
            Kind::Builtin { .. } => false,
        }
    }

    fn loaded_modules(&self) -> Vec<(usize, u32, String)> {
        self.modules()
            .into_iter()
            .map(|m| (m.base, m.size as u32, m.win_path.clone()))
            .collect()
    }

    fn thread_attach(&self, t: *mut u8, is_initial: bool) {
        let mods = self.tls_modules.read().unwrap().clone();
        unsafe {
            let arr = libc::calloc(TLS_ARRAY_CAP, 8) as *mut *mut u8;
            teb::set_tls_pointer(t, arr);
            for m in &mods {
                self.tls_add_block(t, m);
            }
        }
        if is_initial {
            return;
        }
        let _g = self.lock.lock();
        for m in self.modules() {
            if m.is_builtin() || !m.initialized.load(Ordering::Acquire) || !m.thread_calls.load(Ordering::Relaxed) {
                continue;
            }
            for &cb in &m.tls_callbacks {
                unsafe { call_dll_entry(cb, m.base, DLL_THREAD_ATTACH, 0) };
            }
            if m.is_dll() && m.entry() != 0 {
                unsafe { call_dll_entry(m.entry(), m.base, DLL_THREAD_ATTACH, 0) };
            }
        }
    }

    fn thread_detach(&self, t: *mut u8) {
        {
            let _g = self.lock.lock();
            for m in self.modules().iter().rev() {
                if m.is_builtin() || !m.initialized.load(Ordering::Acquire) || !m.thread_calls.load(Ordering::Relaxed) {
                    continue;
                }
                if m.is_dll() && m.entry() != 0 {
                    unsafe { call_dll_entry(m.entry(), m.base, DLL_THREAD_DETACH, 0) };
                }
                for &cb in &m.tls_callbacks {
                    unsafe { call_dll_entry(cb, m.base, DLL_THREAD_DETACH, 0) };
                }
            }
        }
        unsafe {
            let arr = teb::field::<usize>(t, teb::off::TLS_POINTER) as *mut *mut u8;
            if !arr.is_null() {
                for i in 0..TLS_ARRAY_CAP {
                    let p = *arr.add(i);
                    if !p.is_null() {
                        libc::free(p as *mut libc::c_void);
                    }
                }
                libc::free(arr as *mut libc::c_void);
                teb::set_tls_pointer(t, std::ptr::null_mut());
            }
        }
    }

    fn lookup_function_entry(&self, pc: u64) -> Option<(u64, *const RuntimeFunction)> {
        let m = self.module_at(pc as usize)?;
        let Kind::Pe { pdata, .. } = &m.kind else { return None };
        let rva = (pc as usize - m.base) as u32;
        let f = pe_image::find_runtime_function(pdata, rva)?;
        Some((m.base as u64, f as *const RuntimeFunction))
    }

    fn disable_thread_library_calls(&self, hmod: usize) -> bool {
        match self.module_by_base(hmod) {
            Some(m) => {
                m.thread_calls.store(false, Ordering::Relaxed);
                true
            }
            None => false,
        }
    }

    fn symbolize(&self, addr: usize) -> String {
        self.symbolize_addr(addr)
    }

    fn wait_symbols(&self) {
        crate::symbols::wait();
    }

    fn disassemble(&self, addr: u64, bytes: &[u8], max: usize) -> Vec<String> {
        use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter};
        let mut dec = Decoder::with_ip(64, bytes, addr, DecoderOptions::NONE);
        let mut fmt = IntelFormatter::new();
        let mut ins = Instruction::default();
        let mut out = Vec::new();
        while dec.can_decode() && out.len() < max {
            dec.decode_out(&mut ins);
            let off = (ins.ip() - addr) as usize;
            let raw: String = bytes[off..off + ins.len()].iter().map(|b| format!("{b:02x}")).collect();
            let mut text = String::new();
            fmt.format(&ins, &mut text);
            if ins.is_invalid() {
                text = "(bad)".into();
            }
            out.push(format!("{:x}: {raw:<24} {text}", ins.ip()));
            if ins.is_invalid() {
                break;
            }
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// `PORT_V8_JITLESS` / `AW2_V8_JITLESS` overrides manifest/plugin `v8_jitless` defaults.
    #[test]
    fn v8_jitless_knob() {
        for on in [None, Some(""), Some("1"), Some("true"), Some("ON"), Some(" yes "), Some("2")] {
            assert!(v8_jitless_enabled(on), "{on:?}");
        }
        for off in ["0", "false", "OFF", " no "] {
            assert!(!v8_jitless_enabled(Some(off)), "{off:?}");
        }
        assert!(should_apply_v8_jitless_with(true, None));
        assert!(should_apply_v8_jitless_with(true, Some("")));
        assert!(!should_apply_v8_jitless_with(true, Some("0")));
        assert!(!should_apply_v8_jitless_with(true, Some("false")));
        assert!(!should_apply_v8_jitless_with(false, None));
        assert!(should_apply_v8_jitless_with(false, Some("1")));
        assert!(should_apply_v8_jitless_with(false, Some("true")));
    }

    /// `DllRoutingTable` policies (`Builtin`, `Guest`, `GuestOnly`, `Disabled`, `Alias`, and
    /// legacy `-foo.dll` / `!foo.dll` / `0` specs) drive `Loader::classify` accurately.
    #[test]
    fn dll_routing_table_classification() {
        let dir = std::env::temp_dir().join(format!("port-loader-route-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // Create dummy PE files on disk for game-dir presence tests.
        for fname in ["bink2w64.dll", "nrd.dll", "strict_present.dll", "disabled_present.dll"] {
            std::fs::write(dir.join(fname), b"MZ").unwrap();
        }

        // 1. Default routing + legacy env overrides (`-foo.dll`, `!foo.dll`, `0`).
        let default_tbl = port_manifest::DllRoutingTable::new();
        assert!(default_tbl.is_builtin_override("bink2w64.dll"));
        assert!(default_tbl.is_builtin_override("mediadecoders.windowsdesktop.dll"));
        assert!(default_tbl.is_builtin_override("sl.interposer.dll"));
        assert!(default_tbl.is_builtin_override("nri.dll"));
        assert!(default_tbl.is_builtin_override("nvcloth_x64.dll"));
        assert!(!default_tbl.is_builtin_override("nrd.dll"));

        assert!(!port_manifest::DllRoutingTable::from_legacy_override_spec(Some("-bink2w64.dll"))
            .is_builtin_override("bink2w64.dll"));
        assert!(!port_manifest::DllRoutingTable::from_legacy_override_spec(Some(
            "!mediadecoders.windowsdesktop.dll"
        ))
        .is_builtin_override("mediadecoders.windowsdesktop.dll"));
        assert!(!port_manifest::DllRoutingTable::from_legacy_override_spec(Some("-sl.interposer.dll"))
            .is_builtin_override("sl.interposer.dll"));
        assert!(!port_manifest::DllRoutingTable::from_legacy_override_spec(Some("0"))
            .is_builtin_override("sl.interposer.dll"));

        // 2. Full manifest routing matrix via Loader::classify().
        let manifest = port_manifest::GameManifest::parse(
            r#"
[dlls]
"disabled_present.dll" = "disabled"
"strict_missing.dll" = "guest-only"
"strict_present.dll" = "guest-only"
"custom_video.dll" = "builtin:bink2w64.dll"
"alias_bink.dll" = "alias:bink2w64.dll"
"alias_disabled.dll" = "alias:disabled_present.dll"
"cycle_a.dll" = "alias:cycle_b.dll"
"cycle_b.dll" = "alias:cycle_a.dll"
"#,
        );
        let routing = manifest.routing_table(&[], None);
        let l = Loader::new_instance(dir.clone(), "C:\\Game".into(), false, routing, false);

        // Builtin override beats game-dir PE.
        assert_eq!(l.classify("bink2w64.dll"), Some(Source::Builtin("bink2w64.dll".into())));
        // Explicit Builtin { provider } redirects to provider builtin.
        assert_eq!(l.classify("custom_video.dll"), Some(Source::Builtin("bink2w64.dll".into())));
        // Guest prefers game-dir PE when present, falls back to builtin when absent.
        assert_eq!(l.classify("nrd.dll"), Some(Source::GameDir(dir.join("nrd.dll"))));
        assert_eq!(l.classify("kernel32.dll"), Some(Source::Builtin("kernel32.dll".into())));
        // GuestOnly uses game-dir PE when present, NEVER falls back to builtin even if builtin exists.
        assert_eq!(
            l.classify("strict_present.dll"),
            Some(Source::GameDir(dir.join("strict_present.dll")))
        );
        assert_eq!(l.classify("strict_missing.dll"), None);
        // Verify GuestOnly suppresses builtin fallback for a known builtin DLL too.
        let strict_k32 = port_manifest::GameManifest::parse("[dlls]\n\"kernel32.dll\" = \"guest-only\"\n");
        let l_strict = Loader::new_instance(
            dir.clone(),
            "C:\\Game".into(),
            false,
            strict_k32.routing_table(&[], None),
            false,
        );
        assert_eq!(l_strict.classify("kernel32.dll"), None);
        // Disabled refuses to load even when PE exists in game dir.
        assert_eq!(l.classify("disabled_present.dll"), None);
        // Alias resolves target routing and terminates cycles cleanly.
        assert_eq!(l.classify("alias_bink.dll"), Some(Source::Builtin("bink2w64.dll".into())));
        assert_eq!(l.classify("alias_disabled.dll"), None);
        assert_eq!(l.classify("cycle_a.dll"), None);
        // Verify LoaderOps::get_module_handle respects alias, builtin provider, and disabled routes.
        {
            use win32_rt::LoaderOps as _;
            let bink = l.load("bink2w64.dll", false).expect("load bink2w64 builtin");
            assert_eq!(l.get_module_handle(Some("alias_bink.dll")), Some(bink.base));
            assert_eq!(l.get_module_handle(Some("custom_video.dll")), Some(bink.base));
            assert_eq!(l.get_module_handle(Some("disabled_present.dll")), None);
            assert_eq!(l.get_module_handle(Some("alias_disabled.dll")), None);
        }
        // Legacy `-bink2w64.dll` env override flips bink2w64.dll back to game-dir PE.
        let l_neg = Loader::new_instance(
            dir.clone(),
            "C:\\Game".into(),
            false,
            manifest.routing_table(&[], Some("-bink2w64.dll")),
            false,
        );
        assert_eq!(l_neg.classify("bink2w64.dll"), Some(Source::GameDir(dir.join("bink2w64.dll"))));

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Every builtin export reachable through the synthesized export
    /// directory resolves to exactly what GetProcAddress returns (by name and
    /// by declared ordinal), so code walking the EAT itself (Streamline, NRI)
    /// sees the same functions as the import resolver.
    #[test]
    fn builtin_eat_matches_get_proc_address() {
        let dir = std::env::temp_dir().join(format!("aw2-eat-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let l = init(dir.clone(), "C:\\Game".into(), false);
        let mut checked = 0usize;
        for d in w::builtin_dlls() {
            let m = l.load(d.name, true).unwrap();
            assert!(matches!(m.kind, Kind::Builtin { .. }), "{} not builtin", d.name);
            let img = unsafe { std::slice::from_raw_parts(m.base as *const u8, m.size) };
            let pe = Pe::parse(img).unwrap_or_else(|e| panic!("{}: {e}", d.name));
            let ex = pe.exports().unwrap_or_else(|e| panic!("{}: {e}", d.name));
            assert_eq!(ex.dll_name, m.name);
            for e in d.exports {
                let gpa = l.proc_address(&m, ProcName::Name(e.name), 0).unwrap();
                match ex.by_name(e.name) {
                    Some(x) => {
                        let ExportTarget::Rva(r) = x.target else { panic!("{}!{} forwarder", d.name, e.name) };
                        assert_eq!(m.base + r as usize, gpa, "{}!{}", d.name, e.name);
                        checked += 1;
                    }
                    // Only data exports out of 32-bit RVA range may be absent.
                    None => assert!(!crate::builtin_image::is_host_code(e.ptr.0 as usize), "{}!{} missing", d.name, e.name),
                }
            }
            for &(o, n) in d.ordinals {
                let gpa = l.proc_address(&m, ProcName::Ordinal(o), 0).unwrap();
                assert_eq!(gpa, l.proc_address(&m, ProcName::Name(n), 0).unwrap(), "{}!#{o}", d.name);
                if let Some(x) = ex.by_ordinal(o as u32) {
                    let ExportTarget::Rva(r) = x.target else { panic!() };
                    assert_eq!(m.base + r as usize, gpa, "{}!#{o}", d.name);
                }
            }
        }
        assert!(checked > 1000, "only {checked} exports checked");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Pure-Rust builder for a minimal valid relocated-ready x86-64 PE32+ DLL/plugin fixture.
    ///
    /// Layout (FileAlignment = 0x200, SectionAlignment = 0x1000, SizeOfImage = 0x4000):
    /// - `.text`  @ RVA 0x1000 (raw 0x200, size 0x200):
    ///   - 0x1000: `DllMain` -> writes 1 to `[.data+0x00]` (`dllmain_ran`), returns `TRUE (1)`
    ///   - 0x1020: TLS callback -> writes 1 to `[.data+0x04]` (`tls_ran`), returns
    ///   - 0x1040: exported `compute_fn` (ordinal 10): `lea rax, [rcx + rdx + 42]; ret`
    ///   - 0x1060: ordinal-only export `#11`: `mov eax, 0xCAFE; ret`
    /// - `.rdata` @ RVA 0x2000 (raw 0x400, size 0x600):
    ///   - 0x2000: `IMAGE_EXPORT_DIRECTORY` (`compute_fn`=#10, ordinal-only `#11`, `g_magic_data`=#12 -> RVA 0x3008)
    ///   - 0x2180: optional static `IMAGE_IMPORT_DESCRIPTOR` (when `static_import_dll` is `Some`)
    ///   - 0x2280: optional `ImgDelayDescr` (when `delay_import_dll` is `Some`, lazy & non-fatal)
    ///   - 0x2380: optional `IMAGE_TLS_DIRECTORY64` + callback array (when `with_tls` is true)
    ///   - 0x2480: `.pdata` `RUNTIME_FUNCTION` entries for `0x1000`, `0x1020`, `0x1040`, `0x1060`
    /// - `.data`  @ RVA 0x3000 (raw 0xA00, size 0x200):
    ///   - 0x3000: `dllmain_ran: u32` (0 initially)
    ///   - 0x3004: `tls_ran: u32` (0 initially)
    ///   - 0x3008: `g_magic_data: u32` (`0x1234_5678`)
    ///   - 0x3010: TLS index slot (`u32`)
    ///   - 0x3020: static IAT slot (`u64`)
    ///   - 0x3040: delay-load module handle + delay IAT slot
    fn build_synthetic_plugin_pe(
        dll_name: &str,
        image_base: u64,
        is_dll: bool,
        static_import_dll: Option<(&str, &str)>,
        delay_import_dll: Option<(&str, &str)>,
        with_tls: bool,
    ) -> Vec<u8> {
        let mut img = vec![0u8; 0xC00];
        let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
        let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());
        let wstr = |b: &mut [u8], o: usize, s: &str| {
            b[o..o + s.len()].copy_from_slice(s.as_bytes());
            b[o + s.len()] = 0;
        };

        img[0..2].copy_from_slice(b"MZ");
        let pe = 0x80usize;
        w32(&mut img, 0x3C, pe as u32);
        w32(&mut img, pe, 0x0000_4550);
        w16(&mut img, pe + 4, IMAGE_FILE_MACHINE_AMD64);
        w16(&mut img, pe + 6, 3); // .text, .rdata, .data
        w16(&mut img, pe + 20, 240);
        w16(&mut img, pe + 22, if is_dll { 0x2022 } else { 0x0022 });

        let opt = pe + 24;
        w16(&mut img, opt, 0x20B);
        w32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint
        w64(&mut img, opt + 24, image_base);
        w32(&mut img, opt + 32, 0x1000);
        w32(&mut img, opt + 36, 0x200);
        w16(&mut img, opt + 40, 6);
        w16(&mut img, opt + 48, 6);
        w32(&mut img, opt + 56, 0x4000); // SizeOfImage
        w32(&mut img, opt + 60, 0x200); // SizeOfHeaders
        w16(&mut img, opt + 68, 3);
        w16(&mut img, opt + 70, 0x0040); // DYNAMIC_BASE
        w64(&mut img, opt + 72, 0x0010_0000);
        w64(&mut img, opt + 80, 0x0001_0000);
        w64(&mut img, opt + 88, 0x0010_0000);
        w64(&mut img, opt + 96, 0x0001_0000);
        w32(&mut img, opt + 108, 16);

        // DataDirectory[0]: Export Directory @ RVA 0x2000 (size 0x160)
        w32(&mut img, opt + 112, 0x2000);
        w32(&mut img, opt + 112 + 4, 0x160);
        // DataDirectory[1]: Import Directory @ RVA 0x2180 (if present)
        if static_import_dll.is_some() {
            w32(&mut img, opt + 112 + 8, 0x2180);
            w32(&mut img, opt + 112 + 12, 40);
        }
        // DataDirectory[3]: Exception (.pdata) @ RVA 0x2480 (4 * 12 = 48 bytes)
        w32(&mut img, opt + 112 + 3 * 8, 0x2480);
        w32(&mut img, opt + 112 + 3 * 8 + 4, 48);
        // DataDirectory[9]: TLS Directory @ RVA 0x2380 (40 bytes)
        if with_tls {
            w32(&mut img, opt + 112 + 9 * 8, 0x2380);
            w32(&mut img, opt + 112 + 9 * 8 + 4, 40);
        }
        // DataDirectory[13]: Delay Import Directory @ RVA 0x2280 (64 bytes)
        if delay_import_dll.is_some() {
            w32(&mut img, opt + 112 + 13 * 8, 0x2280);
            w32(&mut img, opt + 112 + 13 * 8 + 4, 64);
        }

        // Section headers
        let sh = opt + 240;
        let write_sec = |b: &mut [u8], idx: usize, name: &[u8], rva: u32, raw_off: u32, raw_sz: u32, ch: u32| {
            let s = sh + idx * 40;
            b[s..s + name.len()].copy_from_slice(name);
            w32(b, s + 8, raw_sz);
            w32(b, s + 12, rva);
            w32(b, s + 16, raw_sz);
            w32(b, s + 20, raw_off);
            w32(b, s + 36, ch);
        };
        write_sec(&mut img, 0, b".text", 0x1000, 0x200, 0x200, 0x6000_0020);
        write_sec(&mut img, 1, b".rdata", 0x2000, 0x400, 0x600, 0x4000_0040);
        write_sec(&mut img, 2, b".data", 0x3000, 0xA00, 0x200, 0xC000_0040);

        // `.text` @ file 0x200 (RVA 0x1000)
        // 0x1000: DllMain -> mov dword ptr [rip + 0x1FF6], 1; mov eax, 1; ret
        let dllmain: &[u8] = &[
            0xC7, 0x05, 0xF6, 0x1F, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0xB8, 0x01, 0x00, 0x00, 0x00,
            0xC3,
        ];
        img[0x200..0x200 + dllmain.len()].copy_from_slice(dllmain);
        // 0x1020: TLS callback -> mov dword ptr [rip + 0x1FDA], 1; ret
        let tls_cb: &[u8] = &[
            0xC7, 0x05, 0xDA, 0x1F, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0xC3,
        ];
        img[0x220..0x220 + tls_cb.len()].copy_from_slice(tls_cb);
        // 0x1040: compute_fn -> lea rax, [rcx + rdx + 42]; ret
        let compute_fn: &[u8] = &[0x48, 0x8D, 0x44, 0x11, 0x2A, 0xC3];
        img[0x240..0x240 + compute_fn.len()].copy_from_slice(compute_fn);
        // 0x1060: ordinal-only #11 -> mov eax, 0xCAFE; ret
        let ord_fn: &[u8] = &[0xB8, 0xFE, 0xCA, 0x00, 0x00, 0xC3];
        img[0x260..0x260 + ord_fn.len()].copy_from_slice(ord_fn);

        // `.rdata` @ file 0x400 (RVA 0x2000)
        // Export Directory @ 0x2000 (file 0x400): Base=10, 3 funcs (#10 compute_fn, #11 ordinal-only, #12 g_magic_data)
        let exp = 0x400usize;
        w32(&mut img, exp + 12, 0x2080); // Name RVA
        w32(&mut img, exp + 16, 10); // Base ordinal
        w32(&mut img, exp + 20, 3); // NumberOfFunctions
        w32(&mut img, exp + 24, 2); // NumberOfNames (sorted: "compute_fn", "g_magic_data")
        w32(&mut img, exp + 28, 0x2040); // AddressOfFunctions
        w32(&mut img, exp + 32, 0x2050); // AddressOfNames
        w32(&mut img, exp + 36, 0x2060); // AddressOfNameOrdinals

        w32(&mut img, exp + 0x40, 0x1040); // index 0 (#10) -> compute_fn (code)
        w32(&mut img, exp + 0x44, 0x1060); // index 1 (#11) -> ordinal-only (code)
        w32(&mut img, exp + 0x48, 0x3008); // index 2 (#12) -> g_magic_data (data in .data)

        w32(&mut img, exp + 0x50, 0x20B0); // "compute_fn"
        w32(&mut img, exp + 0x54, 0x20C0); // "g_magic_data"
        w16(&mut img, exp + 0x60, 0); // compute_fn -> index 0 (#10)
        w16(&mut img, exp + 0x62, 2); // g_magic_data -> index 2 (#12)
        wstr(&mut img, exp + 0x80, dll_name);
        wstr(&mut img, exp + 0xB0, "compute_fn");
        wstr(&mut img, exp + 0xC0, "g_magic_data");

        // Optional static import directory @ RVA 0x2180 (file 0x580)
        if let Some((imp_dll, imp_sym)) = static_import_dll {
            let imp = 0x580usize;
            w32(&mut img, imp, 0x21B0); // OriginalFirstThunk (INT)
            w32(&mut img, imp + 12, 0x21D0); // Name RVA
            w32(&mut img, imp + 16, 0x3020); // FirstThunk (IAT in .data)
            w64(&mut img, imp + 0x30, 0x2200); // INT[0] -> IMAGE_IMPORT_BY_NAME @ 0x2200
            wstr(&mut img, imp + 0x50, imp_dll);
            w16(&mut img, imp + 0x80, 0); // hint
            wstr(&mut img, imp + 0x82, imp_sym);
        }

        // Optional delay-load import directory @ RVA 0x2280 (file 0x680)
        if let Some((dimp_dll, dimp_sym)) = delay_import_dll {
            let dimp = 0x680usize;
            w32(&mut img, dimp, 1); // grAttrs = dlattrRva (1)
            w32(&mut img, dimp + 4, 0x22D0); // szName RVA
            w32(&mut img, dimp + 8, 0x3040); // phmod RVA
            w32(&mut img, dimp + 12, 0x3048); // pIAT RVA
            w32(&mut img, dimp + 16, 0x22C0); // pINT RVA
            w64(&mut img, dimp + 0x40, 0x2300); // delay INT[0] -> IMAGE_IMPORT_BY_NAME @ 0x2300
            wstr(&mut img, dimp + 0x50, dimp_dll);
            w16(&mut img, dimp + 0x80, 0);
            wstr(&mut img, dimp + 0x82, dimp_sym);
        }

        // Optional TLS directory @ RVA 0x2380 (file 0x780)
        if with_tls {
            let tls = 0x780usize;
            w64(&mut img, tls, image_base + 0x3080); // StartAddressOfRawData
            w64(&mut img, tls + 8, image_base + 0x3090); // EndAddressOfRawData
            w64(&mut img, tls + 16, image_base + 0x3010); // AddressOfIndex
            w64(&mut img, tls + 24, image_base + 0x23C0); // AddressOfCallBacks
            // Callback array @ RVA 0x23C0 (file 0x7C0)
            w64(&mut img, tls + 0x40, image_base + 0x1020);
            w64(&mut img, tls + 0x48, 0);
        }

        // `.pdata` @ RVA 0x2480 (file 0x880)
        let pdata = 0x880usize;
        for (i, (b, e)) in [
            (0x1000u32, 0x1010u32),
            (0x1020, 0x102B),
            (0x1040, 0x1046),
            (0x1060, 0x1066),
        ]
        .into_iter()
        .enumerate()
        {
            w32(&mut img, pdata + i * 12, b);
            w32(&mut img, pdata + i * 12 + 4, e);
            w32(&mut img, pdata + i * 12 + 8, 0x2500);
        }
        img[0x900] = 0x01; // UNWIND_INFO v1 @ RVA 0x2500

        // `.data` @ file 0xA00 (RVA 0x3000)
        w32(&mut img, 0xA00, 0); // dllmain_ran = 0
        w32(&mut img, 0xA04, 0); // tls_ran = 0
        w32(&mut img, 0xA08, 0x1234_5678); // g_magic_data

        // Sanity-verify our fixture parses cleanly with pe-image.
        let parsed = Pe::parse(&img).expect("synthetic plugin PE must parse");
        assert!(parsed.exports().unwrap().by_name("compute_fn").is_some());
        if delay_import_dll.is_some() {
            assert_eq!(parsed.delay_imports().unwrap().len(), 1);
        }
        img
    }

    #[test]
    fn dll_search_paths_load_library_ex_and_datafile_modes() {
        use w::kernel32::module::*;
        use w::LoaderOps as _;

        reset_dll_search_state();
        let root = std::env::temp_dir().join(format!("port-loader-search-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let app_dir = root.join("app");
        let user_dir = root.join("user_dirs");
        let legacy_dir = root.join("legacy_dir");
        let cwd_dir = root.join("cwd_dir");
        let nested_dir = app_dir.join("plugins").join("nested");
        let hint_dir = root.join("ldr_hint");
        for d in [&app_dir, &user_dir, &legacy_dir, &cwd_dir, &nested_dir, &hint_dir] {
            std::fs::create_dir_all(d).unwrap();
        }

        w::path::add_mount("T:\\TestPkg", &root);
        w::path::set_cwd("T:\\TestPkg\\cwd_dir");

        // Build distinct synthetic x64 PE32+ plugins.
        std::fs::write(
            app_dir.join("app_only.dll"),
            build_synthetic_plugin_pe("app_only.dll", 0x1_6100_0000, true, None, None, true),
        )
        .unwrap();
        std::fs::write(
            user_dir.join("user_only.dll"),
            build_synthetic_plugin_pe("user_only.dll", 0x1_6200_0000, true, None, None, false),
        )
        .unwrap();
        std::fs::write(
            legacy_dir.join("legacy_only.dll"),
            build_synthetic_plugin_pe("legacy_only.dll", 0x1_6300_0000, true, None, None, false),
        )
        .unwrap();
        std::fs::write(
            cwd_dir.join("cwd_only.dll"),
            build_synthetic_plugin_pe("cwd_only.dll", 0x1_6400_0000, true, None, None, false),
        )
        .unwrap();
        // Nested plugin imports its sibling `nested_dep.dll` (only in `plugins/nested/`)
        // AND delay-imports a nonexistent optional DLL (`optional_missing_delay.dll`).
        std::fs::write(
            nested_dir.join("nested_dep.dll"),
            build_synthetic_plugin_pe("nested_dep.dll", 0x1_6500_0000, true, None, None, false),
        )
        .unwrap();
        std::fs::write(
            nested_dir.join("nested_plugin.pyd"),
            build_synthetic_plugin_pe(
                "nested_plugin.pyd",
                0x1_6600_0000,
                true,
                Some(("nested_dep.dll", "compute_fn")),
                Some(("optional_missing_delay.dll", "MissingOptional")),
                true,
            ),
        )
        .unwrap();
        // Second nested plugin for LOAD_WITH_ALTERED_SEARCH_PATH
        let altered_dir = app_dir.join("plugins").join("altered");
        std::fs::create_dir_all(&altered_dir).unwrap();
        std::fs::write(
            altered_dir.join("alt_dep.dll"),
            build_synthetic_plugin_pe("alt_dep.dll", 0x1_6700_0000, true, None, None, false),
        )
        .unwrap();
        std::fs::write(
            altered_dir.join("alt_plugin.node"),
            build_synthetic_plugin_pe(
                "alt_plugin.node",
                0x1_6800_0000,
                true,
                Some(("alt_dep.dll", "compute_fn")),
                None,
                false,
            ),
        )
        .unwrap();
        // Plugin with an unsatisfied static import (for datafile & DONT_RESOLVE tests)
        std::fs::write(
            app_dir.join("broken_dep.ocx"),
            build_synthetic_plugin_pe(
                "broken_dep.ocx",
                0x1_6900_0000,
                true,
                Some(("definitely_missing_dep.dll", "Nope")),
                None,
                true,
            ),
        )
        .unwrap();
        // Plugin in LdrLoadDll PathToFile hint directory
        std::fs::write(
            hint_dir.join("hint_only.dll"),
            build_synthetic_plugin_pe("hint_only.dll", 0x1_6A00_0000, true, None, None, false),
        )
        .unwrap();

        let l = Loader::new_instance(
            app_dir.clone(),
            "T:\\TestPkg\\app".into(),
            false,
            port_manifest::DllRoutingTable::new(),
            false,
        );

        // 1. Missing DLL returns ERROR_MOD_NOT_FOUND.
        assert_eq!(
            l.load_library("does_not_exist.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );

        // 2. LOAD_LIBRARY_AS_DATAFILE / AS_DATAFILE_EXCLUSIVE / AS_IMAGE_RESOURCE succeed even with
        //    missing static imports, NEVER run TLS/DllMain, return tagged handles, fail GetProcAddress,
        //    do not appear in GetModuleHandle, and FreeLibrary cleanly.
        for (flags, expected_tag) in [
            (LOAD_LIBRARY_AS_DATAFILE, 1usize),
            (LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE, 1usize),
            (LOAD_LIBRARY_AS_IMAGE_RESOURCE, 2usize),
        ] {
            let h = l
                .load_library("broken_dep.ocx", flags)
                .expect("datafile load must succeed without resolving imports");
            assert_eq!(h & 3, expected_tag);
            let base = h & !3;
            unsafe {
                assert_eq!(std::ptr::read_unaligned((base + 0x3000) as *const u32), 0, "DllMain must not run");
                assert_eq!(std::ptr::read_unaligned((base + 0x3004) as *const u32), 0, "TLS must not run");
                assert_eq!(std::ptr::read_unaligned((base + 0x3008) as *const u32), 0x1234_5678);
            }
            assert!(l.get_module_handle(Some("broken_dep.ocx")).is_none());
            assert!(l.get_proc_address(h, ProcName::Name("compute_fn")).is_none());
            assert!(l.free_library(h));
        }

        // 3. DONT_RESOLVE_DLL_REFERENCES maps image without resolving imports or running TLS/DllMain,
        //    returns page-aligned handle, and DOES allow GetProcAddress (code & data exports).
        let h_nores = l
            .load_library("broken_dep.ocx", DONT_RESOLVE_DLL_REFERENCES)
            .expect("DONT_RESOLVE_DLL_REFERENCES must succeed with missing imports");
        assert_eq!(h_nores & 3, 0);
        unsafe {
            assert_eq!(std::ptr::read_unaligned((h_nores + 0x3000) as *const u32), 0);
            assert_eq!(std::ptr::read_unaligned((h_nores + 0x3004) as *const u32), 0);
        }
        assert_eq!(l.get_module_handle(Some("broken_dep.ocx")), Some(h_nores));
        assert_eq!(
            l.get_proc_address(h_nores, ProcName::Name("compute_fn")),
            Some(h_nores + 0x1040)
        );
        assert_eq!(
            l.get_proc_address(h_nores, ProcName::Ordinal(11)),
            Some(h_nores + 0x1060)
        );
        let data_ptr = l
            .get_proc_address(h_nores, ProcName::Name("g_magic_data"))
            .expect("data export must resolve");
        assert_eq!(data_ptr, h_nores + 0x3008);
        assert_eq!(unsafe { std::ptr::read_unaligned(data_ptr as *const u32) }, 0x1234_5678);
        assert!(l.get_proc_address(h_nores, ProcName::Name("no_such_export")).is_none());
        assert!(l.free_library(h_nores));

        // 4. AddDllDirectory + LOAD_LIBRARY_SEARCH_USER_DIRS / RemoveDllDirectory.
        assert_eq!(
            l.load_library("user_only.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        let user_w = w::strings::to_wide_z("T:\\TestPkg\\user_dirs");
        let cookie = unsafe { AddDllDirectory(user_w.as_ptr()) };
        assert!(!cookie.is_null());
        // Still NOT found under legacy flags=0 (AddDllDirectory requires USER_DIRS/DEFAULT_DIRS).
        assert_eq!(
            l.load_library("user_only.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        // Found with LOAD_LIBRARY_SEARCH_USER_DIRS!
        let h_user = l
            .load_library("user_only.dll", LOAD_LIBRARY_SEARCH_USER_DIRS)
            .expect("LOAD_LIBRARY_SEARCH_USER_DIRS must find AddDllDirectory entry");
        assert_ne!(h_user, 0);
        assert_eq!(unsafe { RemoveDllDirectory(cookie) }, w::types::TRUE);

        // 5. SetDllDirectoryW("") removes CWD; SetDllDirectoryW(NULL) restores CWD; SetDllDirectoryW(dir) searches dir.
        let empty_w = w::strings::to_wide_z("");
        assert_eq!(unsafe { SetDllDirectoryW(empty_w.as_ptr()) }, w::types::TRUE);
        assert_eq!(
            l.load_library("cwd_only.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        let legacy_w = w::strings::to_wide_z("T:\\TestPkg\\legacy_dir");
        assert_eq!(unsafe { SetDllDirectoryW(legacy_w.as_ptr()) }, w::types::TRUE);
        assert!(l.load_library("legacy_only.dll", 0).is_ok());
        assert_eq!(unsafe { SetDllDirectoryW(std::ptr::null()) }, w::types::TRUE);
        assert!(l.load_library("cwd_only.dll", 0).is_ok());

        // 6. SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32) blocks app-dir DLLs,
        //    while LOAD_LIBRARY_SEARCH_APPLICATION_DIR finds app-dir DLLs and executes TLS + DllMain!
        assert_eq!(
            unsafe { SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32) },
            w::types::TRUE
        );
        assert_eq!(
            l.load_library("app_only.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        let h_app = l
            .load_library("app_only.dll", LOAD_LIBRARY_SEARCH_APPLICATION_DIR)
            .expect("explicit APPLICATION_DIR must find app_only.dll");
        unsafe {
            assert_eq!(std::ptr::read_unaligned((h_app + 0x3000) as *const u32), 1, "DllMain must run on normal load");
            assert_eq!(std::ptr::read_unaligned((h_app + 0x3004) as *const u32), 1, "TLS callback must run on normal load");
        }
        reset_dll_search_state();

        // 7. Nested plugin loading its sibling dependency:
        //    - Without DLL_LOAD_DIR or ALTERED_SEARCH_PATH -> fails because `nested_dep.dll` is not in app dir.
        assert_eq!(
            l.load_library("T:\\TestPkg\\app\\plugins\\nested\\nested_plugin.pyd", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        //    - With LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS -> finds sibling `nested_dep.dll`,
        //      while missing delay-import `optional_missing_delay.dll` remains lazy & non-fatal!
        let h_nested = l
            .load_library(
                "T:\\TestPkg\\app\\plugins\\nested\\nested_plugin.pyd",
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
            .expect("LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR must resolve sibling dependency");
        assert!(l.get_module_handle(Some("nested_dep.dll")).is_some());
        assert!(l.get_module_handle(Some("optional_missing_delay.dll")).is_none());
        unsafe {
            assert_eq!(std::ptr::read_unaligned((h_nested + 0x3000) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned((h_nested + 0x3004) as *const u32), 1);
        }

        // 8. Nested plugin loading sibling dependency via LOAD_WITH_ALTERED_SEARCH_PATH.
        let h_alt = l
            .load_library(
                "T:\\TestPkg\\app\\plugins\\altered\\alt_plugin.node",
                LOAD_WITH_ALTERED_SEARCH_PATH,
            )
            .expect("LOAD_WITH_ALTERED_SEARCH_PATH must resolve sibling dependency");
        assert_ne!(h_alt, 0);
        assert!(l.get_module_handle(Some("alt_dep.dll")).is_some());

        // 9. LdrLoadDll PathToFile hint directory resolution.
        assert_eq!(
            l.load_library("hint_only.dll", 0),
            Err(w::types::err::ERROR_MOD_NOT_FOUND)
        );
        let h_hint = l
            .load_library_with_search_path("hint_only.dll", 0, Some("T:\\TestPkg\\ldr_hint"))
            .expect("PathToFile search hint must resolve hint_only.dll");
        assert_ne!(h_hint, 0);

        reset_dll_search_state();
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn prewarm_maps_discovered_and_declared_plugins_without_running_tls_or_dllmain() {
        use w::LoaderOps as _;

        let root = std::env::temp_dir().join(format!("port-loader-prewarm-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let sub_py = root.join("plugins/py");
        let sub_node = root.join("addons/node");
        let sub_custom = root.join("declared");
        for d in [&sub_py, &sub_node, &sub_custom] {
            std::fs::create_dir_all(d).unwrap();
        }

        let exe_path = root.join("MainApp.exe");
        std::fs::write(
            &exe_path,
            build_synthetic_plugin_pe("MainApp.exe", 0x1_4000_0000, false, None, None, false),
        )
        .unwrap();
        // Sibling dependency + nested .pyd with TLS + DllMain + missing delay import
        std::fs::write(
            sub_py.join("py_helper.dll"),
            build_synthetic_plugin_pe("py_helper.dll", 0x1_7100_0000, true, None, None, true),
        )
        .unwrap();
        std::fs::write(
            sub_py.join("ext_mod.pyd"),
            build_synthetic_plugin_pe(
                "ext_mod.pyd",
                0x1_7200_0000,
                true,
                Some(("py_helper.dll", "compute_fn")),
                Some(("lazy_absent.dll", "OptionalFunc")),
                true,
            ),
        )
        .unwrap();
        // Nested .node plugin
        std::fs::write(
            sub_node.join("native.node"),
            build_synthetic_plugin_pe("native.node", 0x1_7300_0000, true, None, None, true),
        )
        .unwrap();
        // Declared plugin with custom extension in manifest `bundled_plugins`
        std::fs::write(
            sub_custom.join("custom_mod.ocx"),
            build_synthetic_plugin_pe("custom_mod.ocx", 0x1_7400_0000, true, None, None, true),
        )
        .unwrap();

        let manifest = port_manifest::GameManifest::parse(
            r#"
[runtime]
bundled_plugins = ["declared/custom_mod.ocx"]
plugin_dirs = ["plugins/py"]
"#,
        );
        // Enable `trace = true` to also verify code vs data export identity is preserved under tracing!
        let l = Loader::new_instance(
            root.clone(),
            "C:\\PrewarmApp".into(),
            true,
            manifest.routing_table(&[], None),
            false,
        );
        let _exe = l
            .load_pe(&exe_path, "mainapp.exe", "C:\\PrewarmApp\\MainApp.exe".into(), None)
            .unwrap();

        let mapped_plugins = l.prewarm_map_bundled_plugins(&exe_path, Some(&manifest));
        let names: Vec<String> = mapped_plugins.iter().map(|m| m.name.clone()).collect();
        assert!(names.contains(&"ext_mod.pyd".to_string()), "got {names:?}");
        assert!(names.contains(&"native.node".to_string()), "got {names:?}");
        assert!(names.contains(&"custom_mod.ocx".to_string()), "got {names:?}");
        assert!(l.get_module_handle(Some("py_helper.dll")).is_some());
        // Delay-import DLL must NOT have been eagerly loaded!
        assert!(l.get_module_handle(Some("lazy_absent.dll")).is_none());

        // ZERO TLS callbacks and ZERO DllMain executions during prewarm!
        for m in &mapped_plugins {
            assert!(!m.initialized.load(Ordering::Acquire), "{} must remain uninitialized", m.name);
            unsafe {
                assert_eq!(
                    std::ptr::read_unaligned((m.base + 0x3000) as *const u32),
                    0,
                    "{}: DllMain must NOT run during prewarm",
                    m.name
                );
                assert_eq!(
                    std::ptr::read_unaligned((m.base + 0x3004) as *const u32),
                    0,
                    "{}: TLS callback must NOT run during prewarm",
                    m.name
                );
            }
        }

        // Verify dynamic GetProcAddress by name & ordinal AND code vs data export identity
        // (even with `trace = true`, data export must NEVER be wrapped in a code trace thunk!).
        let ext = mapped_plugins.iter().find(|m| m.name == "ext_mod.pyd").unwrap();
        let code_by_name = l
            .get_proc_address(ext.base, ProcName::Name("compute_fn"))
            .expect("code export by name");
        let code_by_ord = l
            .get_proc_address(ext.base, ProcName::Ordinal(11))
            .expect("ordinal-only code export");
        let data_by_name = l
            .get_proc_address(ext.base, ProcName::Name("g_magic_data"))
            .expect("data export by name");
        assert_ne!(code_by_name, data_by_name);
        assert_ne!(code_by_ord, data_by_name);
        // Data export must point directly to `.data` holding 0x1234_5678 (NOT a trace thunk!).
        assert_eq!(data_by_name, ext.base + 0x3008);
        assert_eq!(
            unsafe { std::ptr::read_unaligned(data_by_name as *const u32) },
            0x1234_5678
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn reserved_aot_slot_holds_preferred_base_until_plugin_loads() {
        let root = std::env::temp_dir().join(format!(
            "port-loader-reserve-slot-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();

        let dll_path = root.join("late_plugin.dll");
        let pe_bytes = build_synthetic_plugin_pe(
            "late_plugin.dll",
            0x1_7500_0000,
            true,
            None,
            None,
            false,
        );
        std::fs::write(&dll_path, &pe_bytes).unwrap();
        let (_, img_size, is_dll) = peek_pe_preferred_window(&dll_path).expect("peek PE");
        assert!(is_dll);
        assert!(img_size > 0);

        // Pick a free aligned virtual address to simulate a prewarmed AOT preferred base.
        let pref = unsafe {
            let p = libc::mmap(
                std::ptr::null_mut(),
                img_size,
                libc::PROT_NONE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(p, libc::MAP_FAILED);
            libc::munmap(p, img_size);
            p as usize
        };

        let l = Loader::new_instance(
            root.clone(),
            "C:\\ReserveTest".into(),
            false,
            port_manifest::DllRoutingTable::default(),
            false,
        );
        assert!(l.reserve_aot_slot("late_plugin.dll", pref, img_size));

        // While reserved, another mmap at the same preferred address must not steal `pref`.
        unsafe {
            let probe = libc::mmap(
                pref as *mut libc::c_void,
                img_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(probe, libc::MAP_FAILED);
            assert_ne!(probe as usize, pref, "reserved slot was stolen before LoadLibrary");
            libc::munmap(probe, img_size);
        }

        // Loading the plugin with matching `pref` upgrades the reservation in place.
        let m = l
            .load_pe(
                &dll_path,
                "late_plugin.dll",
                "C:\\ReserveTest\\late_plugin.dll".into(),
                Some(pref),
            )
            .expect("load_pe into reserved slot");
        assert_eq!(m.base, pref);
        assert_eq!(m.size, img_size);
        assert!(l.reserved_aot_slots.lock().unwrap().is_empty());

        let _ = std::fs::remove_dir_all(&root);
    }
}
