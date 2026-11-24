//! Executable-memory policy (`MAP_JIT` vs static read-only signed code) and
//! the [`StaticAotRegistry`] for pre-linked Mach-O `.o` AOT modules.
//!
//! # Why this module exists
//!
//! * On macOS Apple Silicon, `mmap(..., PROT_READ|PROT_WRITE|PROT_EXEC, MAP_JIT)`
//!   and `pthread_jit_write_protect_np` allow per-thread JIT translation and
//!   runtime-relocated `.aot` block caches.
//! * On iOS (non-jailbroken App Store / TestFlight / sideloaded `.ipa` without
//!   dynamic-codesigning entitlements), `MAP_JIT` fails with `EPERM` and
//!   `pthread_jit_write_protect_np` is not even exported in the iOS SDK.
//!
//! Centralizing all executable-memory operations here keeps W^X enforcement
//! clean across macOS and iOS:
//! 1. [`jit_writable`] probes `MAP_JIT` once (honoring `PORT_EXEC_MODE`).
//! 2. [`jit_write_protect`] calls `pthread_jit_write_protect_np` directly on
//!    macOS and resolves it lazily via `dlsym` on iOS (never failing link time
//!    on `aarch64-apple-ios` or `aarch64-apple-ios-sim`).
//! 3. [`register_static_aot_desc`] / [`port_static_aot_register_desc`] /
//!    [`bind_static_aot_module`] / [`lookup_static_aot`] provide a first-class,
//!    zero-`MAP_JIT` execution registry for statically linked, Apple-signed
//!    ARM64 `__TEXT` AOT modules (Milestone #2).

use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

/// Execution backend policy (`PORT_EXEC_MODE`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecMode {
    /// Probe `MAP_JIT` at startup: use JIT/AOT when writable, otherwise run
    /// registered static AOT modules + pure-Rust interpreter.
    Auto,
    /// Prefer dynamic `MAP_JIT` (falls back cleanly if `mmap(MAP_JIT)` fails).
    JitWritable,
    /// Forbid `MAP_JIT`: use only statically linked signed AOT modules and the
    /// pure-Rust interpreter (matches locked-down iOS production builds).
    StaticAotOnly,
    /// Pure-Rust interpreter only (no `MAP_JIT` and no static AOT dispatch).
    InterpreterOnly,
}

impl ExecMode {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "auto" => Some(Self::Auto),
            "jit" | "jit-writable" | "dynamic" => Some(Self::JitWritable),
            "static-aot" | "static" | "no-jit" | "nojit" => Some(Self::StaticAotOnly),
            "interpreter" | "interp" => Some(Self::InterpreterOnly),
            _ => None,
        }
    }
}

/// Process-wide execution policy read from `PORT_EXEC_MODE` (default [`ExecMode::Auto`]).
pub fn exec_mode() -> ExecMode {
    static MODE: OnceLock<ExecMode> = OnceLock::new();
    *MODE.get_or_init(|| {
        std::env::var("PORT_EXEC_MODE")
            .ok()
            .and_then(|v| ExecMode::parse(&v))
            .unwrap_or(ExecMode::Auto)
    })
}

const PROT_RWX: i32 = 1 | 2 | 4;
const MAP_PRIVATE_ANON_JIT: i32 = 0x0002 | 0x1000 | 0x0800;

extern "C" {
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
    fn munmap(addr: *mut u8, len: usize) -> i32;
    fn sys_icache_invalidate(start: *mut u8, len: usize);
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
extern "C" {
    fn pthread_jit_write_protect_np(enabled: i32);
}

/// Probe once whether this process can allocate writable+executable `MAP_JIT` pages.
pub fn probe_jit_writable() -> bool {
    #[cfg(not(target_arch = "aarch64"))]
    {
        false
    }
    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            let p = mmap(core::ptr::null_mut(), 16384, PROT_RWX, MAP_PRIVATE_ANON_JIT, -1, 0);
            if p.is_null() || p as isize == -1 {
                return false;
            }
            let _ = munmap(p, 16384);
            true
        }
    }
}

/// True when dynamic `MAP_JIT` code generation/patching is allowed and supported.
pub fn jit_writable() -> bool {
    static WRITABLE: OnceLock<bool> = OnceLock::new();
    *WRITABLE.get_or_init(|| {
        match exec_mode() {
            ExecMode::StaticAotOnly => {
                log::info!("exec_mem: PORT_EXEC_MODE=static-aot; MAP_JIT disabled");
                false
            }
            ExecMode::InterpreterOnly => {
                log::info!("exec_mem: PORT_EXEC_MODE=interpreter; MAP_JIT and static AOT disabled");
                false
            }
            ExecMode::Auto => {
                #[cfg(target_os = "ios")]
                {
                    log::info!(
                        "exec_mem: iOS target defaults to zero-MAP_JIT static-AOT + interpreter mode (set PORT_EXEC_MODE=jit to probe MAP_JIT)"
                    );
                    false
                }
                #[cfg(not(target_os = "ios"))]
                {
                    let ok = probe_jit_writable();
                    if !ok {
                        log::info!(
                            "exec_mem: MAP_JIT unavailable on this platform/entitlement; running static-AOT + interpreter mode"
                        );
                    }
                    ok
                }
            }
            ExecMode::JitWritable => {
                let ok = probe_jit_writable();
                if !ok {
                    log::info!(
                        "exec_mem: MAP_JIT unavailable on this platform/entitlement; running static-AOT + interpreter mode"
                    );
                }
                ok
            }
        }
    })
}

/// Allocate a `MAP_JIT` RWX region of `len` bytes, returning `None` cleanly when
/// `MAP_JIT` is disabled by policy or rejected by the kernel.
pub unsafe fn alloc_jit(len: usize) -> Option<*mut u8> {
    if !jit_writable() || len == 0 {
        return None;
    }
    let p = mmap(core::ptr::null_mut(), len, PROT_RWX, MAP_PRIVATE_ANON_JIT, -1, 0);
    if p.is_null() || p as isize == -1 {
        None
    } else {
        Some(p)
    }
}

/// Release a `MAP_JIT` region allocated by [`alloc_jit`].
pub unsafe fn free_jit(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len != 0 {
        munmap(ptr, len);
    }
}

/// Toggle per-thread Apple Silicon APRR JIT write protection (`false` = RW, `true` = RX).
/// On iOS, resolved lazily via `dlsym` so binaries never reference a missing SDK symbol.
#[inline(always)]
pub unsafe fn jit_write_protect(enabled: bool) {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        pthread_jit_write_protect_np(enabled as i32);
    }
    #[cfg(all(target_arch = "aarch64", target_os = "ios"))]
    {
        type WriteProtectFn = unsafe extern "C" fn(i32);
        static FN: OnceLock<Option<WriteProtectFn>> = OnceLock::new();
        let f = *FN.get_or_init(|| {
            let sym = win32_rt::libc::dlsym(
                win32_rt::libc::RTLD_DEFAULT,
                c"pthread_jit_write_protect_np".as_ptr(),
            );
            if sym.is_null() {
                None
            } else {
                Some(core::mem::transmute::<*mut core::ffi::c_void, WriteProtectFn>(sym))
            }
        });
        if let Some(f) = f {
            f(enabled as i32);
        }
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios"))))]
    {
        let _ = enabled;
    }
}

/// Flush instruction cache for `[start, start + len)`.
#[inline(always)]
pub unsafe fn icache_invalidate(start: *mut u8, len: usize) {
    if !start.is_null() && len > 0 {
        sys_icache_invalidate(start, len);
    }
}

// ---------------------------------------------------------------------------
// Static AOT Registry (signed, read-only __TEXT ARM64 modules — zero MAP_JIT)
// ---------------------------------------------------------------------------

/// Magic constant (`"SAOT"`) for [`StaticAotModuleDesc`].
pub const STATIC_AOT_DESC_MAGIC: u32 = 0x5341_4F54;
/// Static module ABI. Version 2 reads the clock mode from the current JIT scratch layout.
pub const STATIC_AOT_DESC_VERSION: u32 = 2;

/// Module descriptor flags (`StaticAotModuleDesc::flags`).
pub const STATIC_AOT_MOD_HAS_RAS: u32 = 1 << 0;
pub const STATIC_AOT_MOD_HAS_XMM_PIN: u32 = 1 << 1;
pub const STATIC_AOT_MOD_HAS_AOT_PROBE: u32 = 1 << 2;

/// One block entry inside a statically linked, signed ARM64 AOT module.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticAotEntry {
    /// Guest RVA relative to the PE image base.
    pub rva: u32,
    /// Byte offset from the module's signed `__TEXT` code base.
    pub code_offset: u32,
    /// Bit 0 (`STATIC_AOT_FLAG_HOST_CALLABLE`): block IP is host-callable.
    pub flags: u32,
}
const _: () = assert!(core::mem::size_of::<StaticAotEntry>() == 12);

/// Entry flag indicating the block targets a host-callable function.
pub const STATIC_AOT_FLAG_HOST_CALLABLE: u32 = 1 << 0;

/// Descriptor embedded in `__DATA_CONST,__const` of every static Mach-O `.o` emitted
/// by [`super::static_macho`]. Auto-registered before `main()` via `__DATA,__mod_init_func`
/// calling [`port_static_aot_register_desc`].
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StaticAotModuleDesc {
    pub magic: u32,
    pub version: u32,
    pub knobs: u64,
    /// Windows PE base address the ARM64 blocks were compiled for (0 = base-agnostic).
    pub image_base: u64,
    pub image_size: u64,
    pub image_hash: [u64; 2],
    /// Runtime address of the module's signed `__TEXT` stub + block code blob.
    pub code_base: usize,
    pub code_len: u32,
    /// Byte offset of the `enter` stub from `code_base`.
    pub enter_off: u32,
    /// Byte offset of the `lookup_rip` stub from `code_base`.
    pub lookup_rip_off: u32,
    /// Bitmask of `STATIC_AOT_MOD_HAS_*`.
    pub flags: u32,
    pub entry_count: u32,
    pub _pad: u32,
    pub entries: *const StaticAotEntry,
    pub module_name: *const c_char,
}
const _: () = assert!(core::mem::size_of::<StaticAotModuleDesc>() == 96);

/// Backing storage for a static AOT module's sorted entry table.
/// Statically linked `__TEXT,__const` descriptors borrow directly with zero heap copy;
/// dynamic / unsorted registrations own a compact `Arc<[StaticAotEntry]>`.
#[derive(Clone, Debug)]
enum StaticEntries {
    Borrowed(&'static [StaticAotEntry]),
    Owned(Arc<[StaticAotEntry]>),
}

impl StaticEntries {
    #[inline]
    fn as_slice(&self) -> &[StaticAotEntry] {
        match self {
            StaticEntries::Borrowed(s) => s,
            StaticEntries::Owned(a) => a,
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

#[derive(Clone, Debug)]
struct RegisteredStaticModule {
    name_lower: String,
    compiled_image_base: u64,
    image_size: u64,
    image_hash: [u64; 2],
    knobs: u64,
    code_base: usize,
    code_len: u32,
    enter_off: u32,
    lookup_rip_off: u32,
    flags: u32,
    entries: StaticEntries,
}

#[derive(Clone, Debug)]
struct BoundStaticModule {
    name_lower: String,
    guest_base: u64,
    guest_end: u64,
    code_base: usize,
    enter_addr: usize,
    lookup_rip_addr: usize,
    has_ras: bool,
    entries: StaticEntries,
}

/// Immutable snapshot published on every static AOT bind/unbind.
struct StaticAotSnapshot {
    /// Bounding guest span `[min_guest, max_guest)` across all bound modules for O(1) reject.
    min_guest: u64,
    max_guest: u64,
    /// Sorted, merged half-open guest ranges `[start, end)` across all bound modules.
    ranges: Box<[(u64, u64)]>,
    /// Open-addressed probe table (`[shift, mask, (ip, code) * cap]`), shared with `STATIC_PROBE`.
    probe_base: usize,
    /// Compact sorted `(guest_ip, host_code_addr)` table for `STATIC_AOT_FLAG_HOST_CALLABLE`
    /// blocks (checked only on probe miss).
    host_entries: Box<[(u64, usize)]>,
}

/// Owned heap backing for the active published static AOT tables.
///
/// While [`STATIC_READERS_ACTIVE`] is `false` (during sequential initial PE boot before
/// any guest thread executes), replacing this drops the previous probe table and snapshot
/// instead of leaking hundreds of MiB of intermediate hash tables. Once any reader has
/// observed a published table, superseded allocations are retired in `RETIRED_STATIC` so
/// lock-free readers remain sound for process lifetime.
struct PublishedStatic {
    _probe: Option<Box<[u64]>>,
    _snap: Option<Box<StaticAotSnapshot>>,
}
unsafe impl Send for PublishedStatic {}

static STATIC_REGISTERED: RwLock<Vec<RegisteredStaticModule>> = RwLock::new(Vec::new());
static STATIC_BOUND: RwLock<Vec<BoundStaticModule>> = RwLock::new(Vec::new());
static STATIC_PUBLISHED: Mutex<Option<PublishedStatic>> = Mutex::new(None);
static RETIRED_STATIC: Mutex<Vec<PublishedStatic>> = Mutex::new(Vec::new());
/// Flipped to `true` the first time any thread reads `STATIC_PROBE`, `STATIC_SNAP`, or
/// `STATIC_ENTER` (starts lock-free retirement instead of freeing superseded boot tables).
static STATIC_READERS_ACTIVE: AtomicBool = AtomicBool::new(false);
/// Fast lock-free counter of bound static AOT blocks (0 => zero overhead on lookup).
static STATIC_BOUND_BLOCKS: AtomicUsize = AtomicUsize::new(0);
/// Published heap probe table (`[shift, mask, (ip, fast) * (mask + 1)]`) for bound static
/// AOT blocks, matching the exact layout read by `aot_probe` in signed `__TEXT` stubs.
static STATIC_PROBE: AtomicUsize = AtomicUsize::new(0);
/// Immutable leaked snapshot for lock-free O(1) [`lookup_static_aot`].
static STATIC_SNAP: AtomicPtr<StaticAotSnapshot> = AtomicPtr::new(core::ptr::null_mut());
/// Monotonic generation bumped (skipping 0) on every static AOT bind/unbind.
static STATIC_AOT_GEN: AtomicU32 = AtomicU32::new(1);
/// Active signed `__TEXT` entry stub `(enter, lookup_rip, has_ras)` for zero-`MAP_JIT` runs.
static STATIC_ENTER: AtomicUsize = AtomicUsize::new(0);
static STATIC_LOOKUP_RIP: AtomicUsize = AtomicUsize::new(0);
static STATIC_HAS_RAS: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "aarch64")]
const STATIC_JMUL: u64 = super::jit::JMUL;
#[cfg(not(target_arch = "aarch64"))]
const STATIC_JMUL: u64 = 0x9E37_79B9_7F4A_7C15;

#[cfg(test)]
std::thread_local! {
    /// Test-only counter of [`lookup_static_aot`] invocations on this thread (verifies
    /// negative caching eliminates repeated registry lookups).
    static TEST_STATIC_LOOKUPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Number of [`lookup_static_aot`] calls made on this thread (tests).
#[cfg(test)]
pub(crate) fn test_static_lookups() -> u64 {
    TEST_STATIC_LOOKUPS.with(|c| c.get())
}

/// Reset the per-thread [`lookup_static_aot`] call counter (tests).
#[cfg(test)]
pub(crate) fn test_reset_static_lookups() {
    TEST_STATIC_LOOKUPS.with(|c| c.set(0));
}

/// Monotonic generation counter for bound static AOT modules (never 0).
/// Bumped whenever static AOT modules are bound or unbound.
#[inline(always)]
pub fn static_aot_gen() -> u32 {
    let g = STATIC_AOT_GEN.load(Ordering::Acquire);
    if g == 0 { 1 } else { g }
}

/// Advance `STATIC_AOT_GEN` skipping `0`. Caller must hold `STATIC_BOUND.write()`.
fn bump_static_aot_gen_locked() {
    let cur = STATIC_AOT_GEN.load(Ordering::Relaxed);
    let next = match cur.wrapping_add(1) {
        0 => 1,
        n => n,
    };
    STATIC_AOT_GEN.store(next, Ordering::Release);
}

fn norm_module_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    base.to_ascii_lowercase()
}

/// C-ABI registration entrypoint invoked automatically by `__DATA,__mod_init_func`
/// constructors inside static Mach-O `.o` AOT modules (or explicitly from Swift/C).
#[no_mangle]
pub unsafe extern "C" fn port_static_aot_register_desc(desc: *const StaticAotModuleDesc) {
    if desc.is_null() {
        return;
    }
    let _ = register_static_aot_desc(&*desc);
}

/// Register a signed static AOT module from its embedded [`StaticAotModuleDesc`].
/// Returns `true` if accepted.
pub unsafe fn register_static_aot_desc(desc: &StaticAotModuleDesc) -> bool {
    if desc.magic != STATIC_AOT_DESC_MAGIC || desc.version != STATIC_AOT_DESC_VERSION {
        log::warn!(
            "exec_mem: ignoring static AOT descriptor with bad magic/version ({:#x}/v{})",
            desc.magic,
            desc.version
        );
        return false;
    }
    if desc.code_base == 0 || desc.entries.is_null() || desc.entry_count == 0 || desc.module_name.is_null() {
        return false;
    }
    let Ok(name) = CStr::from_ptr(desc.module_name).to_str() else {
        return false;
    };
    let slice: &'static [StaticAotEntry] = core::slice::from_raw_parts(desc.entries, desc.entry_count as usize);
    let entries = if slice.windows(2).all(|w| w[0].rva < w[1].rva) {
        StaticEntries::Borrowed(slice)
    } else {
        let mut sorted = slice.to_vec();
        sorted.sort_unstable_by_key(|e| e.rva);
        sorted.dedup_by_key(|e| e.rva);
        StaticEntries::Owned(Arc::from(sorted))
    };
    register_static_aot_internal(
        name,
        desc.image_base,
        desc.image_size,
        desc.image_hash,
        desc.knobs,
        desc.code_base,
        desc.code_len,
        desc.enter_off,
        desc.lookup_rip_off,
        desc.flags,
        entries,
    );
    true
}

/// Register a statically linked, Apple-signed ARM64 AOT module for PE image `name`
/// (e.g. `"Game.exe"` or `"foo.dll"`). `code_base` points into read-only executable
/// `__TEXT` memory emitted by the offline static Mach-O `.o` compiler.
///
/// Safe to call before PE loading (e.g. from C/Swift startup or `__attribute__((constructor))`).
pub fn register_static_aot_module(name: &str, code_base: usize, entries: &[StaticAotEntry]) {
    if code_base == 0 || entries.is_empty() {
        return;
    }
    let mut sorted = entries.to_vec();
    sorted.sort_unstable_by_key(|e| e.rva);
    sorted.dedup_by_key(|e| e.rva);
    register_static_aot_internal(name, 0, 0, [0, 0], 0, code_base, 0, 0, 0, 0, StaticEntries::Owned(Arc::from(sorted)));
}

#[allow(clippy::too_many_arguments)]
fn register_static_aot_internal(
    name: &str,
    compiled_image_base: u64,
    image_size: u64,
    image_hash: [u64; 2],
    knobs: u64,
    code_base: usize,
    code_len: u32,
    enter_off: u32,
    lookup_rip_off: u32,
    flags: u32,
    entries: StaticEntries,
) {
    let name_lower = norm_module_name(name);
    let count = entries.len();

    let mut reg = STATIC_REGISTERED.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|m| m.name_lower != name_lower);
    reg.push(RegisteredStaticModule {
        name_lower: name_lower.clone(),
        compiled_image_base,
        image_size,
        image_hash,
        knobs,
        code_base,
        code_len,
        enter_off,
        lookup_rip_off,
        flags,
        entries,
    });
    log::info!(
        "exec_mem: registered static AOT module {name_lower} ({count} blocks, code_base={code_base:#x}, compiled_base={compiled_image_base:#x})"
    );
}

/// If a static AOT module is registered for PE image `name` with a fixed compiled image
/// base, return that base so the PE loader can prefer mapping the PE at the exact address.
pub fn static_aot_preferred_base(name: &str) -> Option<usize> {
    let name_lower = norm_module_name(name);
    let reg = STATIC_REGISTERED.read().unwrap_or_else(|e| e.into_inner());
    reg.iter()
        .find(|m| m.name_lower == name_lower && m.compiled_image_base != 0)
        .map(|m| m.compiled_image_base as usize)
}

/// Build an open-addressed probe table (`[shift, mask, (ip, code) * cap]`) directly
/// from `list` in two passes (zero intermediate `live` Vec allocation), matching the
/// exact layout read by `aot_probe` in signed `__TEXT` stubs.
fn build_static_probe_from_modules(
    list: &[BoundStaticModule],
) -> (Option<Box<[u64]>>, Vec<(u64, usize)>) {
    let mut live_count = 0usize;
    let mut host_count = 0usize;
    for m in list {
        let span = m.guest_end.saturating_sub(m.guest_base);
        for e in m.entries.as_slice() {
            if (e.rva as u64) >= span {
                continue;
            }
            if m.guest_base.wrapping_add(e.rva as u64) == 0 {
                continue;
            }
            if (e.flags & STATIC_AOT_FLAG_HOST_CALLABLE) != 0 {
                host_count += 1;
            } else {
                live_count += 1;
            }
        }
    }
    let mut host_entries: Vec<(u64, usize)> = Vec::with_capacity(host_count);
    let mut probe_box = if live_count == 0 {
        None
    } else {
        // On iOS (strict 6 GiB Jetsam ceiling), a ~73% max load factor (`live * 4 / 3`)
        // halves the static probe table from 128 MiB -> 64 MiB for 3M+ blocks while
        // keeping average linear-probe length ~2.4 slots.
        let target = if cfg!(target_os = "ios") {
            live_count.saturating_mul(4) / 3 + 1
        } else {
            live_count.saturating_mul(2)
        };
        let bits = target.max(2).next_power_of_two().trailing_zeros().max(10);
        let cap = 1usize << bits;
        let shift = 64 - bits as u64;
        let mask = cap as u64 - 1;
        let mut buf = vec![0u64; 2 + 2 * cap].into_boxed_slice();
        buf[0] = shift;
        buf[1] = mask;
        Some((buf, shift, cap - 1))
    };

    for m in list {
        let span = m.guest_end.saturating_sub(m.guest_base);
        for e in m.entries.as_slice() {
            if (e.rva as u64) >= span {
                continue;
            }
            let ip = m.guest_base.wrapping_add(e.rva as u64);
            if ip == 0 {
                continue;
            }
            let code = m.code_base.wrapping_add(e.code_offset as usize);
            if (e.flags & STATIC_AOT_FLAG_HOST_CALLABLE) != 0 {
                host_entries.push((ip, code));
            } else if let Some((ref mut buf, shift, mask_usize)) = probe_box {
                let mut i = (ip.wrapping_mul(STATIC_JMUL) >> shift) as usize & mask_usize;
                loop {
                    let slot = 2 + 2 * i;
                    let k = buf[slot];
                    if k == 0 || k == ip {
                        buf[slot + 1] = code as u64;
                        buf[slot] = ip;
                        break;
                    }
                    i = (i + 1) & mask_usize;
                }
            }
        }
    }
    host_entries.sort_unstable_by_key(|&(ip, _)| ip);
    host_entries.dedup_by_key(|e| e.0);
    (probe_box.map(|(b, _, _)| b), host_entries)
}

/// Publish a fresh [`StaticAotSnapshot`], `STATIC_PROBE`, entry stub atomics, block count,
/// and bump [`STATIC_AOT_GEN`] (last, with Release ordering). Caller holds `STATIC_BOUND.write()`.
fn publish_static_aot_locked(list: &[BoundStaticModule]) {
    let mut raw_ranges: Vec<(u64, u64)> = list
        .iter()
        .filter(|m| m.guest_end > m.guest_base && !m.entries.is_empty())
        .map(|m| (m.guest_base, m.guest_end))
        .collect();
    raw_ranges.sort_unstable();
    let mut ranges: Vec<(u64, u64)> = Vec::with_capacity(raw_ranges.len());
    for (s, e) in raw_ranges {
        if let Some(last) = ranges.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        ranges.push((s, e));
    }

    let (probe_box, host_entries) = build_static_probe_from_modules(list);
    let probe_base = probe_box.as_ref().map_or(0, |b| b.as_ptr() as usize);
    let snap_box = if ranges.is_empty() {
        None
    } else {
        let min_guest = ranges[0].0;
        let max_guest = ranges[ranges.len() - 1].1;
        Some(Box::new(StaticAotSnapshot {
            min_guest,
            max_guest,
            ranges: ranges.into_boxed_slice(),
            probe_base,
            host_entries: host_entries.into_boxed_slice(),
        }))
    };
    let snap_ptr = snap_box
        .as_ref()
        .map_or(core::ptr::null_mut(), |b| (&**b as *const StaticAotSnapshot) as *mut StaticAotSnapshot);

    STATIC_PROBE.store(probe_base, Ordering::Release);
    STATIC_SNAP.store(snap_ptr, Ordering::Release);
    // Pair with the SeqCst fence in `mark_static_readers_active` so a concurrent first
    // reader either sees the new pointers or forces us to observe `STATIC_READERS_ACTIVE == true`.
    core::sync::atomic::fence(Ordering::SeqCst);

    let prev = STATIC_PUBLISHED
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .replace(PublishedStatic { _probe: probe_box, _snap: snap_box });
    if let Some(old) = prev {
        if STATIC_READERS_ACTIVE.load(Ordering::Acquire) {
            // Concurrent lock-free readers may still reference the previous snapshot/probe.
            RETIRED_STATIC.lock().unwrap_or_else(|e| e.into_inner()).push(old);
        } else {
            // Sequential boot before any guest thread started: drop old tables cleanly
            // instead of leaking ~320 MiB of intermediate probe tables.
            drop(old);
        }
    }

    let enter_info = list
        .iter()
        .find(|b| b.enter_addr != 0 && b.lookup_rip_addr != 0)
        .map(|b| (b.enter_addr, b.lookup_rip_addr, b.has_ras));
    match enter_info {
        Some((e, lr, ras)) => {
            STATIC_HAS_RAS.store(ras, Ordering::Release);
            STATIC_LOOKUP_RIP.store(lr, Ordering::Release);
            STATIC_ENTER.store(e, Ordering::Release);
        }
        None => {
            STATIC_ENTER.store(0, Ordering::Release);
            STATIC_LOOKUP_RIP.store(0, Ordering::Release);
            STATIC_HAS_RAS.store(false, Ordering::Release);
        }
    }

    let total: usize = list.iter().map(|b| b.entries.len()).sum();
    STATIC_BOUND_BLOCKS.store(total, Ordering::Release);
    // Generation bump MUST be published last so Acquire readers see the complete snapshot.
    bump_static_aot_gen_locked();
}

/// Bind any pre-registered static AOT module matching PE image `name` to its live
/// mapped virtual range `[guest_base, guest_base + guest_size)`.
/// Returns the number of bound blocks (0 if no static AOT module matches `name` or if
/// the PE landed at a different base than the static AOT code was compiled for).
pub fn bind_static_aot_module(name: &str, guest_base: usize, guest_size: usize) -> usize {
    if exec_mode() == ExecMode::InterpreterOnly || guest_base == 0 || guest_size == 0 {
        return 0;
    }
    let name_lower = norm_module_name(name);
    let matched = {
        let reg = STATIC_REGISTERED.read().unwrap_or_else(|e| e.into_inner());
        reg.iter().find(|m| m.name_lower == name_lower).cloned()
    };
    let Some(m) = matched else {
        return 0;
    };
    if m.compiled_image_base != 0 && m.compiled_image_base != guest_base as u64 {
        log::warn!(
            "exec_mem: static AOT module {name_lower} compiled for base {:#x}, but PE mapped at {guest_base:#x}; skipping static AOT (falling back to interpreter)",
            m.compiled_image_base
        );
        return 0;
    }
    if m.image_size != 0 && m.image_size != guest_size as u64 {
        log::warn!(
            "exec_mem: static AOT module {name_lower} compiled for image size {:#x}, but PE size is {guest_size:#x}; skipping static AOT",
            m.image_size
        );
        return 0;
    }
    let count = m.entries.len();
    let enter_addr = if m.code_len > 0 && m.enter_off < m.code_len {
        m.code_base.wrapping_add(m.enter_off as usize)
    } else {
        0
    };
    let lookup_rip_addr = if m.code_len > 0 && m.lookup_rip_off < m.code_len {
        m.code_base.wrapping_add(m.lookup_rip_off as usize)
    } else {
        0
    };
    let has_ras = (m.flags & STATIC_AOT_MOD_HAS_RAS) != 0;
    let bound = BoundStaticModule {
        name_lower: m.name_lower.clone(),
        guest_base: guest_base as u64,
        guest_end: (guest_base as u64).saturating_add(guest_size as u64),
        code_base: m.code_base,
        enter_addr,
        lookup_rip_addr,
        has_ras,
        entries: m.entries,
    };
    {
        let mut list = STATIC_BOUND.write().unwrap_or_else(|e| e.into_inner());
        list.retain(|b| b.name_lower != bound.name_lower);
        list.push(bound);
        publish_static_aot_locked(&list);
    }
    log::info!(
        "exec_mem: bound static AOT module {name_lower} at {guest_base:#x}..{:#x} ({count} blocks, enter={enter_addr:#x}, ras={has_ras}, knobs={:#x}, hash={:016x}{:016x})",
        guest_base.saturating_add(guest_size),
        m.knobs,
        m.image_hash[0],
        m.image_hash[1]
    );
    count
}

#[inline(always)]
fn mark_static_readers_active() {
    if !STATIC_READERS_ACTIVE.load(Ordering::Relaxed) {
        STATIC_READERS_ACTIVE.store(true, Ordering::Relaxed);
        core::sync::atomic::fence(Ordering::SeqCst);
    }
}

/// Active signed `__TEXT` entry stub `(enter_addr, lookup_rip_addr, has_ras)` for
/// zero-`MAP_JIT` static AOT execution, if at least one executable static module is bound.
#[inline]
pub fn static_aot_enter_stub() -> Option<(usize, usize, bool)> {
    mark_static_readers_active();
    let enter = STATIC_ENTER.load(Ordering::Acquire);
    if enter == 0 {
        return None;
    }
    let lr = STATIC_LOOKUP_RIP.load(Ordering::Acquire);
    let ras = STATIC_HAS_RAS.load(Ordering::Acquire);
    Some((enter, lr, ras))
}

/// Shared heap probe table address for bound static AOT blocks (`0` if none).
#[inline]
pub fn static_probe_table() -> usize {
    mark_static_readers_active();
    STATIC_PROBE.load(Ordering::Acquire)
}

/// True if at least one static AOT module is currently bound.
#[inline]
pub fn has_bound_static_aot() -> bool {
    STATIC_BOUND_BLOCKS.load(Ordering::Acquire) != 0
}

#[inline]
fn in_sorted_ranges(ranges: &[(u64, u64)], ip: u64) -> bool {
    if ranges.len() <= 8 {
        for &(s, e) in ranges {
            if ip < s {
                return false;
            }
            if ip < e {
                return true;
            }
        }
        false
    } else {
        let idx = ranges.partition_point(|&(s, _)| s <= ip);
        idx > 0 && ip < ranges[idx - 1].1
    }
}

#[inline(always)]
unsafe fn probe_static_table(probe_base: usize, guest_ip: u64) -> Option<usize> {
    if guest_ip == 0 {
        return None;
    }
    let hdr = probe_base as *const u64;
    let shift = *hdr;
    let mask = *hdr.add(1) as usize;
    let ents = hdr.add(2) as *const [u64; 2];
    let mut i = (guest_ip.wrapping_mul(STATIC_JMUL) >> shift) as usize & mask;
    loop {
        let [k, code] = *ents.add(i);
        if k == guest_ip {
            return if code != 0 { Some(code as usize) } else { None };
        }
        if k == 0 {
            return None;
        }
        i = (i + 1) & mask;
    }
}

/// Look up a statically linked, signed ARM64 AOT block for `guest_ip` (lock-free O(1)).
/// Returns `Some((host_code_addr, host_callable))`.
#[inline]
pub fn lookup_static_aot(guest_ip: u64) -> Option<(usize, bool)> {
    #[cfg(test)]
    TEST_STATIC_LOOKUPS.with(|c| c.set(c.get().wrapping_add(1)));

    mark_static_readers_active();
    let snap_ptr = STATIC_SNAP.load(Ordering::Acquire);
    if snap_ptr.is_null() {
        return None;
    }
    // SAFETY: once `mark_static_readers_active` runs, any subsequent publication retires
    // superseded snapshots into `RETIRED_STATIC` rather than dropping them.
    let snap = unsafe { &*snap_ptr };
    if guest_ip < snap.min_guest || guest_ip >= snap.max_guest {
        return None;
    }
    if snap.ranges.len() > 1 && !in_sorted_ranges(&snap.ranges, guest_ip) {
        return None;
    }
    if snap.probe_base != 0 {
        // SAFETY: `probe_base` points to an immutable leaked `[shift, mask, (ip, code) * cap]` table.
        if let Some(code) = unsafe { probe_static_table(snap.probe_base, guest_ip) } {
            return Some((code, false));
        }
    }
    if !snap.host_entries.is_empty() {
        if let Ok(idx) = snap.host_entries.binary_search_by_key(&guest_ip, |&(ip, _)| ip) {
            return Some((snap.host_entries[idx].1, true));
        }
    }
    None
}

/// Return `(registered_modules, bound_modules, bound_blocks)` for diagnostics.
pub fn static_aot_stats() -> (usize, usize, usize) {
    let reg = STATIC_REGISTERED.read().unwrap_or_else(|e| e.into_inner()).len();
    let bound = STATIC_BOUND.read().unwrap_or_else(|e| e.into_inner()).len();
    let blocks = STATIC_BOUND_BLOCKS.load(Ordering::Acquire);
    (reg, bound, blocks)
}

#[cfg(test)]
pub(crate) fn unregister_static_aot_for_test(name: &str) {
    let key = norm_module_name(name);
    STATIC_REGISTERED.write().unwrap().retain(|m| m.name_lower != key);
    let mut list = STATIC_BOUND.write().unwrap();
    list.retain(|b| b.name_lower != key);
    publish_static_aot_locked(&list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_mode_parses_all_policies() {
        assert_eq!(ExecMode::parse(""), Some(ExecMode::Auto));
        assert_eq!(ExecMode::parse("auto"), Some(ExecMode::Auto));
        assert_eq!(ExecMode::parse("jit"), Some(ExecMode::JitWritable));
        assert_eq!(ExecMode::parse("static-aot"), Some(ExecMode::StaticAotOnly));
        assert_eq!(ExecMode::parse("no-jit"), Some(ExecMode::StaticAotOnly));
        assert_eq!(ExecMode::parse("interpreter"), Some(ExecMode::InterpreterOnly));
        assert_eq!(ExecMode::parse("bogus"), None);
    }

    #[test]
    fn static_aot_registry_register_bind_and_lookup() {
        let entries = [
            StaticAotEntry { rva: 0x2000, code_offset: 0x400, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
            StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
        ];
        register_static_aot_module("C:\\Game\\StaticUnitTest.DLL", 0x1_0000_0000, &entries);
        let bound = bind_static_aot_module("staticunittest.dll", 0x1_4000_0000, 0x10_0000);
        assert_eq!(bound, 2);

        assert_eq!(lookup_static_aot(0x1_4000_1000), Some((0x1_0000_0100, false)));
        assert_eq!(lookup_static_aot(0x1_4000_2000), Some((0x1_0000_0400, true)));
        assert_eq!(lookup_static_aot(0x1_4000_1500), None);
        assert_eq!(lookup_static_aot(0x1_5000_1000), None);
        assert_ne!(static_probe_table(), 0);

        unregister_static_aot_for_test("staticunittest.dll");
    }

    #[test]
    fn static_aot_desc_rejects_mismatched_base() {
        let name = std::ffi::CString::new("basecheck.dll").unwrap();
        let entries = [StaticAotEntry { rva: 0x1000, code_offset: 0x1000, flags: 0 }];
        let desc = StaticAotModuleDesc {
            magic: STATIC_AOT_DESC_MAGIC,
            version: STATIC_AOT_DESC_VERSION,
            knobs: 0,
            image_base: 0x1_8000_0000,
            image_size: 0x2_0000,
            image_hash: [0x11, 0x22],
            code_base: 0x3_0000_0000,
            code_len: 0x2000,
            enter_off: 0,
            lookup_rip_off: 0x200,
            flags: STATIC_AOT_MOD_HAS_RAS,
            entry_count: 1,
            _pad: 0,
            entries: entries.as_ptr(),
            module_name: name.as_ptr(),
        };
        let legacy_clock = StaticAotModuleDesc { version: 1, ..desc };
        assert!(!unsafe { register_static_aot_desc(&legacy_clock) });
        assert!(unsafe { register_static_aot_desc(&desc) });
        assert_eq!(static_aot_preferred_base("basecheck.dll"), Some(0x1_8000_0000));
        // Wrong runtime base must honestly refuse binding.
        assert_eq!(bind_static_aot_module("basecheck.dll", 0x1_9000_0000, 0x2_0000), 0);
        // Matching base binds cleanly and exposes the signed enter stub.
        assert_eq!(bind_static_aot_module("basecheck.dll", 0x1_8000_0000, 0x2_0000), 1);
        assert_eq!(static_aot_enter_stub(), Some((0x3_0000_0000, 0x3_0000_0200, true)));
        unregister_static_aot_for_test("basecheck.dll");
    }

    #[test]
    fn static_aot_snapshot_ranges_generation_and_host_entries() {
        let g0 = static_aot_gen();
        assert_ne!(g0, 0);

        let mod_a = [
            StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
            StaticAotEntry { rva: 0x2000, code_offset: 0x200, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
        ];
        let mod_b = [
            StaticAotEntry { rva: 0x3000, code_offset: 0x300, flags: 0 },
            StaticAotEntry { rva: 0x4000, code_offset: 0x400, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
        ];
        register_static_aot_module("snap_a.dll", 0x5_0000_0000, &mod_a);
        register_static_aot_module("snap_b.dll", 0x6_0000_0000, &mod_b);

        assert_eq!(bind_static_aot_module("snap_a.dll", 0x2_0000_0000, 0x1_0000), 2);
        let g1 = static_aot_gen();
        assert_ne!(g1, g0);
        assert_ne!(g1, 0);

        assert_eq!(bind_static_aot_module("snap_b.dll", 0x2_4000_0000, 0x1_0000), 2);
        let g2 = static_aot_gen();
        assert_ne!(g2, g1);

        // Snapshot exposes merged/sorted ranges and lock-free probe + host_entries lookup.
        let snap = unsafe { &*STATIC_SNAP.load(Ordering::Acquire) };
        assert_eq!(snap.min_guest, 0x2_0000_0000);
        assert_eq!(snap.max_guest, 0x2_4001_0000);
        assert_eq!(&*snap.ranges, &[(0x2_0000_0000, 0x2_0001_0000), (0x2_4000_0000, 0x2_4001_0000)]);
        assert_eq!(snap.host_entries.len(), 2);
        assert_ne!(snap.probe_base, 0);
        assert_eq!(snap.probe_base, static_probe_table());

        test_reset_static_lookups();
        assert_eq!(lookup_static_aot(0x1_FFFF_F000), None); // below min_guest
        assert_eq!(lookup_static_aot(0x2_2000_0000), None); // in gap between modules
        assert_eq!(lookup_static_aot(0x2_5000_0000), None); // above max_guest
        assert_eq!(lookup_static_aot(0x2_0000_1000), Some((0x5_0000_0100, false)));
        assert_eq!(lookup_static_aot(0x2_0000_2000), Some((0x5_0000_0200, true)));
        assert_eq!(lookup_static_aot(0x2_4000_3000), Some((0x6_0000_0300, false)));
        assert_eq!(lookup_static_aot(0x2_4000_4000), Some((0x6_0000_0400, true)));
        assert_eq!(lookup_static_aot(0x2_4000_3500), None); // inside range, absent entry
        assert_eq!(test_static_lookups(), 8);

        unregister_static_aot_for_test("snap_a.dll");
        let g3 = static_aot_gen();
        assert_ne!(g3, g2);
        assert_eq!(lookup_static_aot(0x2_0000_1000), None);
        assert_eq!(lookup_static_aot(0x2_4000_3000), Some((0x6_0000_0300, false)));

        unregister_static_aot_for_test("snap_b.dll");
        let g4 = static_aot_gen();
        assert_ne!(g4, g3);
        assert_eq!(lookup_static_aot(0x2_4000_3000), None);
        assert_eq!(static_probe_table(), 0);
    }
}
