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
    pub base: usize,
    pub size: usize,
    pub kind: Kind,
    pub tls_callbacks: Vec<usize>,
    pub initialized: AtomicBool,
    pub thread_calls: AtomicBool,
    pub refs: AtomicU32,
    pub ldr: AtomicUsize,
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
    /// Count of unresolved static imports (trap thunks installed).
    pub traps: AtomicU32,
    /// `dll!name` of every unresolved static import.
    pub unresolved: Mutex<std::collections::BTreeSet<String>>,
}

static LOADER: OnceLock<Loader> = OnceLock::new();

pub fn get() -> &'static Loader {
    LOADER.get().expect("loader not initialized")
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
            pending_init: Mutex::new(Vec::new()),
            pending_seq: std::sync::atomic::AtomicU64::new(0),
            tls_modules: RwLock::new(Vec::new()),
            next_tls: AtomicU32::new(0),
            lock: ReentrantLock::new(),
            exe: OnceLock::new(),
            trace_cache: Mutex::new(HashMap::new()),
            traps: AtomicU32::new(0),
            unresolved: Mutex::new(Default::default()),
        }
    }
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

    pub fn module_at(&self, addr: usize) -> Option<Arc<Module>> {
        self.modules.read().unwrap().iter().find(|m| m.contains(addr)).cloned()
    }

    fn module_by_base(&self, base: usize) -> Option<Arc<Module>> {
        self.modules.read().unwrap().iter().find(|m| m.base == base).cloned()
    }

    /// Case-insensitive lookup of a file in the game directory.
    fn find_in_game_dir(&self, base_name: &str) -> Option<PathBuf> {
        let direct = self.game_dir.join(base_name);
        if direct.is_file() {
            return Some(direct);
        }
        std::fs::read_dir(&self.game_dir).ok()?.filter_map(|e| e.ok()).find_map(|e| {
            let n = e.file_name();
            let n = n.to_str()?;
            n.eq_ignore_ascii_case(base_name).then(|| e.path())
        })
    }

    /// Locate a guest PE file in `game_dir` or at an explicit non-System32 Windows path.
    fn find_guest_pe(&self, name: &str, norm: &str) -> Option<PathBuf> {
        if let Some(p) = self.find_in_game_dir(norm) {
            return Some(p);
        }
        if name.contains('\\') || name.contains('/') {
            // Never load our materialized builtin PE images in C:\Windows\System32\
            // (or SysWOW64) as guest PEs — keep them routed through Source::Builtin.
            let win_abs = w::path::absolute(name);
            let win_up = win_abs.to_ascii_uppercase();
            let in_sysdir = win_up.starts_with("C:\\WINDOWS\\SYSTEM32\\")
                || win_up.starts_with("C:\\WINDOWS\\SYSWOW64\\");
            if !(in_sysdir && w::find_builtin(norm).is_some()) {
                if let Some(h) = w::path::to_host(name) {
                    if h.is_file() {
                        return Some(h);
                    }
                }
            }
        }
        None
    }

    /// Decide where a DLL comes from using the active [`port_manifest::DllRoutingTable`]:
    /// API sets always map to builtins; explicit `Disabled`/`Alias`/`Builtin`/`GuestOnly`/`Guest`
    /// policies govern game-dir PEs vs native ARM64 builtins.
    fn classify(&self, name: &str) -> Option<Source> {
        self.classify_with_depth(name, 0)
    }

    fn classify_with_depth(&self, name: &str, depth: u8) -> Option<Source> {
        let norm = pe_image::normalize_dll_name(name);
        if norm.starts_with("api-ms-win-") || norm.starts_with("ext-ms-win-") {
            return Some(Source::Builtin(w::canonical_dll(&norm)));
        }
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
                self.classify_with_depth(&target, depth + 1)
            }
            port_manifest::DllRoute::Builtin { provider } => {
                let target = match provider {
                    Some(p) => w::canonical_dll(&p),
                    None => w::canonical_dll(&norm),
                };
                if w::find_builtin(&target).is_some() {
                    if self.find_in_game_dir(&norm).is_some() {
                        log::info!(
                            "{norm}: using builtin {target} instead of the game-dir PE (DLL routing policy)"
                        );
                    }
                    return Some(Source::Builtin(target));
                }
                self.find_guest_pe(name, &norm).map(Source::GameDir)
            }
            port_manifest::DllRoute::GuestOnly => self.find_guest_pe(name, &norm).map(Source::GameDir),
            port_manifest::DllRoute::Guest => {
                if let Some(p) = self.find_guest_pe(name, &norm) {
                    return Some(Source::GameDir(p));
                }
                if w::find_builtin(&norm).is_some() {
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
        let _g = self.lock.lock();
        let norm = pe_image::normalize_dll_name(name);
        if let Some(m) = self.find_loaded(&norm) {
            m.refs.fetch_add(1, Ordering::Relaxed);
            return Ok(m);
        }
        match self.classify(name) {
            Some(Source::GameDir(p)) => {
                let win = w::path::to_win(&p).unwrap_or_else(|| format!("{}\\{}", self.game_win_dir, norm));
                // Lifted / natively patched DLLs map at their pinned base (fail-soft: if it
                // is taken the image is relocated and those replacements stay off).
                self.load_pe_inner(&p, &norm, win, x86_runtime::dll_native::pinned_base(&norm), !static_import)
            }
            Some(Source::Builtin(c)) => {
                if let Some(m) = self.find_loaded(&c) {
                    m.refs.fetch_add(1, Ordering::Relaxed);
                    return Ok(m);
                }
                Ok(self.add_builtin(&c, w::find_builtin(&c)))
            }
            None if static_import => {
                log::warn!("{norm}: no PE in the game dir and no builtin; every import from it will trap");
                Ok(self.add_builtin(&norm, None))
            }
            None => bail!("module {name:?} not found"),
        }
    }

    /// Create the module record for a builtin DLL. Its HMODULE is a fake PE
    /// image with a real export directory whose entries point at jump thunks
    /// to the Rust implementations (see `builtin_image`).
    fn add_builtin(&self, name: &str, dll: Option<&'static w::BuiltinDll>) -> Arc<Module> {
        use crate::builtin_image as bi;
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
        self.load_pe_inner(host, norm, win_path, force_base, false)
    }

    /// Verify before mapping that every static dependency DLL of `pe` (recursively for
    /// game-dir PE dependencies) exists either as an already-loaded module, on disk, or
    /// as a builtin. Matches Windows `LoadLibrary` returning `ERROR_MOD_NOT_FOUND` when
    /// an optional plugin DLL imports an unavailable redistributable (e.g. `mfc140u.dll`).
    fn check_strict_deps(&self, norm: &str, pe: &Pe<'_>, seen: &mut std::collections::HashSet<String>) -> Result<()> {
        for imp in pe.imports()? {
            let dep_norm = pe_image::normalize_dll_name(&imp.dll);
            if !seen.insert(dep_norm.clone()) || self.find_loaded(&dep_norm).is_some() {
                continue;
            }
            match self.classify(&imp.dll) {
                Some(Source::Builtin(_)) => {}
                Some(Source::GameDir(dep_path)) => {
                    let dep_data = std::fs::read(&dep_path)
                        .with_context(|| format!("{norm}: reading dependency {}", dep_path.display()))?;
                    let dep_pe = Pe::parse(&dep_data)
                        .with_context(|| format!("{norm}: parsing dependency {}", dep_path.display()))?;
                    self.check_strict_deps(&dep_norm, &dep_pe, seen)?;
                }
                None => bail!("{norm}: missing required dependency DLL {:?}", imp.dll),
            }
        }
        Ok(())
    }

    fn load_pe_inner(
        &self,
        host: &Path,
        norm: &str,
        win_path: String,
        force_base: Option<usize>,
        strict_deps: bool,
    ) -> Result<Arc<Module>> {
        let data = std::fs::read(host).with_context(|| format!("reading {}", host.display()))?;
        let pe = Pe::parse(&data).with_context(|| format!("parsing {}", host.display()))?;
        let h = &pe.headers;
        if h.machine != IMAGE_FILE_MACHINE_AMD64 {
            bail!("{norm}: not an x86-64 image (machine 0x{:x})", h.machine);
        }
        if strict_deps && h.is_dll() {
            let mut seen = std::collections::HashSet::from([norm.to_string()]);
            self.check_strict_deps(norm, &pe, &mut seen)?;
        }
        let size = (h.size_of_image as usize + PAGE - 1) & !(PAGE - 1);
        let pref = force_base
            .or_else(|| x86_runtime::emu::exec_mem::static_aot_preferred_base(norm))
            .or_else(|| x86_runtime::emu::aot_cached_preferred_base(norm, size))
            .unwrap_or(h.image_base as usize);
        let base = unsafe { map_image_memory(pref, size, !h.relocs_stripped()) }
            .ok_or_else(|| anyhow!("{norm}: cannot map 0x{size:x} bytes (preferred 0x{pref:x})"))?;
        let img = unsafe { std::slice::from_raw_parts_mut(base as *mut u8, size) };
        // Headers + sections.
        let hdr = (h.size_of_headers as usize).min(data.len()).min(size);
        img[..hdr].copy_from_slice(&data[..hdr]);
        for s in &pe.sections {
            let n = (s.raw_size.min(s.mem_size()) as usize).min(data.len().saturating_sub(s.raw_offset as usize));
            let dst = s.rva as usize;
            if n > 0 && dst + n <= size {
                img[dst..dst + n].copy_from_slice(&data[s.raw_offset as usize..s.raw_offset as usize + n]);
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
        let tls_dir = pe.tls()?;
        let tls_callbacks: Vec<usize> = pe.tls_callback_rvas()?.into_iter().map(|r| base + r as usize).collect();
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
        let m = Arc::new(Module {
            name: norm.to_string(),
            win_path: win_path.clone(),
            base,
            size,
            kind: Kind::Pe { exports, pdata, tls, entry, is_dll: h.is_dll() },
            tls_callbacks,
            initialized: AtomicBool::new(false),
            thread_calls: AtomicBool::new(true),
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
            "mapped {norm} at 0x{base:x}-0x{:x}{}",
            base + size,
            if tls_index != 0 || matches!(&m.kind, Kind::Pe { tls: Some(_), .. }) { format!(" (TLS index {tls_index})") } else { String::new() }
        );
        if h.is_dll() && !std::env::var("AW2_DLL_NATIVE").is_ok_and(|v| v == "0") {
            let n = unsafe { x86_runtime::dll_native::install(norm, base, size) };
            if n != 0 {
                log::info!("{norm}: {n} native leaf replacement(s)");
            }
        }

        // Imports (delay imports stay lazy: the image's own delay-load helper
        // calls LoadLibrary/GetProcAddress when first used).
        for imp in pe.imports()? {
            let dep = self.load(&imp.dll, true)?;
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
        let seq = self.pending_seq.fetch_add(1, Ordering::Relaxed);
        self.pending_init.lock().unwrap().push((seq, m.clone()));
        Ok(m)
    }

    /// Resolve one import to an address (implementation, forwarder target,
    /// or a trap thunk). Wraps it in a trace thunk with `--trace`.
    fn resolve_import(&self, dep: &Arc<Module>, dll: &str, name: &ImportName, importer: &str) -> usize {
        let pname = match name {
            ImportName::Name { name, .. } => ProcName::Name(name.as_str()),
            ImportName::Ordinal(o) => ProcName::Ordinal(*o),
        };
        match self.proc_address(dep, pname, 0) {
            Some(a) => self.maybe_trace(dll, &name.to_string(), importer, a),
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
        const LOAD_LIBRARY_AS_DATAFILE: u32 = 0x2;
        const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 0x20;
        const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: u32 = 0x40;
        if flags & (LOAD_LIBRARY_AS_DATAFILE | LOAD_LIBRARY_AS_IMAGE_RESOURCE | LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE) != 0 {
            log::warn!("STUB LoadLibraryEx({name:?}, 0x{flags:x}): data-file loads are mapped as normal images");
        }
        let _g = self.lock.lock();
        let mark = self.pending_mark();
        match self.load(name, false) {
            Ok(m) => {
                if !self.run_init_since(mark, false) {
                    return Err(1114); // ERROR_DLL_INIT_FAILED
                }
                log::info!("LoadLibrary({name:?}) -> {} 0x{:x}", m.name, m.base);
                Ok(m.base)
            }
            Err(e) => {
                log::warn!("LoadLibrary({name:?}) failed: {e:#}");
                Err(126) // ERROR_MOD_NOT_FOUND
            }
        }
    }

    fn free_library(&self, hmod: usize) -> bool {
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
        let m = self.module_by_base(hmod)?;
        let a = self.proc_address(&m, name, 0)?;
        Some(self.maybe_trace(&m.name, &name.to_string(), "GetProcAddress", a))
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
}
