//! Windows `TEB` / `PEB` / `KUSER_SHARED_DATA` emulation.
//!
//! macOS x86_64 points `%gs` at the pthread TSD array (`tsd[0]` = pthread
//! self). Windows x64 code reads `gs:[0x30]` (TEB self), `gs:[0x58]`
//! (ThreadLocalStoragePointer) and `gs:[0x60]` (PEB). Like Wine on macOS we
//! write those three TSD slots directly:
//!
//! * slot 6  (`gs:0x30`) is `_PTHREAD_TSD_SLOT_RESERVED_WIN64` — reserved by
//!   Apple exactly for this purpose;
//! * slot 11 (`gs:0x58`) is `__PTK_LIBC_TTYNAME_KEY` and slot 12 (`gs:0x60`)
//!   is `__PTK_LIBC_LOCALTIME_KEY`: libc's `ttyname()` / `localtime()` (the
//!   non-`_r` versions) would use our pointers as their static buffers, so
//!   NEVER call those on a Windows thread. We clear all three slots before a
//!   Windows thread exits so libc key destructors never see our pointers.
//!
//! Verified under Rosetta 2 on macOS 27.2 (see `crates/port-loader/README.md`
//! and the `gs_slots_roundtrip` test): writes are visible via both
//! `gs:`-relative loads and `pthread_getspecific`, and pthread keys, Rust
//! `thread_local!`, errno, malloc and std mutexes keep working.
//!
//! Every other `gs:[x]` offset aliases a pthread TSD slot; Windows code that
//! reads e.g. `gs:[0x10]` (StackLimit) directly gets the TSD value instead.
//! The loader scans images for such accesses (`--scan-gs`).

#[cfg(target_arch = "x86_64")]
use std::arch::asm;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub const TEB_SIZE: usize = 0x2000;

/// TEB field offsets (x64).
pub mod off {
    pub const EXCEPTION_LIST: usize = 0x00;
    pub const STACK_BASE: usize = 0x08;
    pub const STACK_LIMIT: usize = 0x10;
    pub const FIBER_DATA: usize = 0x20;
    pub const ARBITRARY_USER_POINTER: usize = 0x28;
    pub const SELF: usize = 0x30;
    pub const CLIENT_ID_PROCESS: usize = 0x40;
    pub const CLIENT_ID_THREAD: usize = 0x48;
    pub const TLS_POINTER: usize = 0x58;
    pub const PEB: usize = 0x60;
    pub const LAST_ERROR: usize = 0x68;
    pub const CURRENT_LOCALE: usize = 0x108;
    pub const LAST_STATUS: usize = 0x1250;
    pub const DEALLOCATION_STACK: usize = 0x1478;
    pub const TLS_SLOTS: usize = 0x1480;
    pub const TLS_EXPANSION_SLOTS: usize = 0x1780;
    pub const FLS_DATA: usize = 0x17C8;
}

pub const TLS_MINIMUM_AVAILABLE: usize = 64;
pub const TLS_EXPANSION_SLOTS: usize = 1024;

#[cfg(not(target_arch = "x86_64"))]
thread_local! {
    static GS_SLOTS: [std::cell::Cell<usize>; 32] = const { [const { std::cell::Cell::new(0) }; 32] };
}

/// Address of the current thread's emulated GS block: `gs:[8 * i]` is `*ptr.add(i)` for
/// `i < 32` (slot 0 is unused; `gs_read(0)` returns `pthread_self`). The block lives as long
/// as the thread, so callers may cache the pointer per thread.
#[cfg(not(target_arch = "x86_64"))]
pub fn gs_slots_ptr() -> *const usize {
    GS_SLOTS.with(|s| s.as_ptr() as *const usize)
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn gs_read(offset: usize) -> usize {
    let v: usize;
    asm!("mov {v}, gs:[{o}]", o = in(reg) offset, v = out(reg) v, options(nostack, readonly, preserves_flags));
    v
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub unsafe fn gs_read(offset: usize) -> usize {
    if offset == 0 {
        return libc::pthread_self() as usize;
    }
    let slot = offset / 8;
    GS_SLOTS.with(|s| s.get(slot).map_or(0, |c| c.get()))
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn gs_write(offset: usize, v: usize) {
    asm!("mov gs:[{o}], {v}", o = in(reg) offset, v = in(reg) v, options(nostack, preserves_flags));
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub unsafe fn gs_write(offset: usize, v: usize) {
    let slot = offset / 8;
    GS_SLOTS.with(|s| {
        if let Some(c) = s.get(slot) {
            c.set(v);
        }
    });
}

/// Raw pointer to the current thread's TEB, or null if this thread has none.
#[inline(always)]
pub fn current_raw() -> *mut u8 {
    unsafe { gs_read(off::SELF) as *mut u8 }
}

#[inline(always)]
pub unsafe fn field<T: Copy>(teb: *mut u8, offset: usize) -> T {
    std::ptr::read_unaligned(teb.add(offset) as *const T)
}

#[inline(always)]
pub unsafe fn set_field<T: Copy>(teb: *mut u8, offset: usize, v: T) {
    std::ptr::write_unaligned(teb.add(offset) as *mut T, v)
}

/// Current TEB, creating one ("adopting" the thread) if necessary. Adopted
/// threads get a TEB and static TLS but no DllMain(THREAD_ATTACH).
pub fn current() -> *mut u8 {
    let t = current_raw();
    if !t.is_null() {
        return t;
    }
    adopt_current_thread()
}

#[cold]
fn adopt_current_thread() -> *mut u8 {
    log::debug!("adopting foreign thread {:?} into the Windows runtime", std::thread::current().id());
    let (lo, hi) = unsafe {
        let me = libc::pthread_self();
        let hi = libc::pthread_get_stackaddr_np(me) as usize;
        let lo = hi - libc::pthread_get_stacksize_np(me);
        (lo, hi)
    };
    let teb = unsafe { init_thread(hi, lo, alloc_tid()) };
    if let Some(l) = crate::try_loader() {
        l.thread_attach(teb, true);
    }
    ADOPTED.with(|a| a.set(true));
    teb
}

thread_local! {
    static ADOPTED: AdoptGuard = const { AdoptGuard(std::cell::Cell::new(false)) };
}

struct AdoptGuard(std::cell::Cell<bool>);
impl AdoptGuard {
    fn set(&self, v: bool) {
        self.0.set(v)
    }
}
impl Drop for AdoptGuard {
    fn drop(&mut self) {
        if self.0.get() {
            unsafe { uninstall() };
        }
    }
}

static NEXT_TID: AtomicU32 = AtomicU32::new(0x100);

/// Allocate a Windows thread id (multiples of 4, never 0).
pub fn alloc_tid() -> u32 {
    NEXT_TID.fetch_add(4, Ordering::Relaxed)
}

/// Allocate and install a TEB for the calling thread. `stack_base` is the
/// high end of the stack, `stack_limit` the low end.
pub unsafe fn init_thread(stack_base: usize, stack_limit: usize, tid: u32) -> *mut u8 {
    let teb = libc::calloc(1, TEB_SIZE) as *mut u8;
    assert!(!teb.is_null());
    set_field(teb, off::STACK_BASE, stack_base);
    set_field(teb, off::STACK_LIMIT, stack_limit);
    set_field(teb, off::DEALLOCATION_STACK, stack_limit);
    set_field(teb, off::SELF, teb as usize);
    set_field(teb, off::CLIENT_ID_PROCESS, libc::getpid() as usize);
    set_field(teb, off::CLIENT_ID_THREAD, tid as usize);
    set_field(teb, off::PEB, peb() as usize);
    set_field(teb, off::CURRENT_LOCALE, 0x409u32);
    install(teb);
    teb
}

/// Write gs:0x30/0x58/0x60 (and gs:0x20 FiberData on non-x86_64) from the TEB contents.
pub unsafe fn install(teb: *mut u8) {
    gs_write(off::SELF, teb as usize);
    gs_write(off::TLS_POINTER, field::<usize>(teb, off::TLS_POINTER));
    gs_write(off::PEB, field::<usize>(teb, off::PEB));
    #[cfg(not(target_arch = "x86_64"))]
    gs_write(off::FIBER_DATA, field::<usize>(teb, off::FIBER_DATA));
}

/// Update FiberData in both the TEB and (on non-x86_64) emulated gs:0x20.
pub unsafe fn set_fiber_data(teb: *mut u8, fiber: usize) {
    set_field(teb, off::FIBER_DATA, fiber);
    #[cfg(not(target_arch = "x86_64"))]
    if current_raw() == teb {
        gs_write(off::FIBER_DATA, fiber);
    }
}

/// Update ThreadLocalStoragePointer in both the TEB and gs:0x58.
pub unsafe fn set_tls_pointer(teb: *mut u8, p: *mut *mut u8) {
    set_field(teb, off::TLS_POINTER, p as usize);
    if current_raw() == teb {
        gs_write(off::TLS_POINTER, p as usize);
    }
}

/// Clear the TSD slots we borrowed. Must run before a Windows thread exits.
pub unsafe fn uninstall() {
    gs_write(off::SELF, 0);
    gs_write(off::TLS_POINTER, 0);
    gs_write(off::PEB, 0);
    #[cfg(not(target_arch = "x86_64"))]
    gs_write(off::FIBER_DATA, 0);
}

pub unsafe fn free_teb(teb: *mut u8) {
    let exp: usize = field(teb, off::TLS_EXPANSION_SLOTS);
    if exp != 0 {
        libc::free(exp as *mut libc::c_void);
    }
    libc::free(teb as *mut libc::c_void);
}

pub fn thread_id() -> u32 {
    unsafe { field::<usize>(current(), off::CLIENT_ID_THREAD) as u32 }
}

pub fn last_error() -> u32 {
    unsafe { field(current(), off::LAST_ERROR) }
}

pub fn set_last_error(e: u32) {
    unsafe { set_field(current(), off::LAST_ERROR, e) }
}

// ---------------------------------------------------------------------------
// PEB
// ---------------------------------------------------------------------------

pub mod peb_off {
    pub const BEING_DEBUGGED: usize = 0x02;
    pub const IMAGE_BASE: usize = 0x10;
    pub const LDR: usize = 0x18;
    pub const PROCESS_PARAMETERS: usize = 0x20;
    pub const PROCESS_HEAP: usize = 0x30;
    pub const FAST_PEB_LOCK: usize = 0x38;
    pub const TLS_EXPANSION_COUNTER: usize = 0x6C;
    pub const NUMBER_OF_PROCESSORS: usize = 0xB8;
    pub const NT_GLOBAL_FLAG: usize = 0xBC;
    pub const LOADER_LOCK: usize = 0x110;
    pub const OS_MAJOR: usize = 0x118;
    pub const OS_MINOR: usize = 0x11C;
    pub const OS_BUILD: usize = 0x120;
    pub const OS_PLATFORM_ID: usize = 0x124;
    pub const IMAGE_SUBSYSTEM: usize = 0x128;
    pub const SESSION_ID: usize = 0x2C0;
}

pub const PEB_SIZE: usize = 0x1000;
/// Windows 10 22H2 — what we report everywhere (GetVersionEx, PEB, KUSER).
pub const OS_MAJOR: u32 = 10;
pub const OS_MINOR: u32 = 0;
pub const OS_BUILD: u32 = 19045;

static PEB: AtomicUsize = AtomicUsize::new(0);
static LDR: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub _pad: u32,
    pub buffer: *mut u16,
}

impl UnicodeString {
    pub fn leak(s: &str) -> UnicodeString {
        let w = crate::strings::leak_wide_z(s);
        let len = s.encode_utf16().count() * 2;
        UnicodeString { length: len as u16, maximum_length: (len + 2) as u16, _pad: 0, buffer: w }
    }
}

#[repr(C)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

unsafe fn list_init(head: *mut ListEntry) {
    (*head).flink = head;
    (*head).blink = head;
}

unsafe fn list_push_back(head: *mut ListEntry, e: *mut ListEntry) {
    let last = (*head).blink;
    (*e).flink = head;
    (*e).blink = last;
    (*last).flink = e;
    (*head).blink = e;
}

/// `LDR_DATA_TABLE_ENTRY` (prefix; we allocate 0x120 bytes).
#[repr(C)]
pub struct LdrEntry {
    pub in_load_order: ListEntry,           // 0x00
    pub in_memory_order: ListEntry,         // 0x10
    pub in_init_order: ListEntry,           // 0x20
    pub dll_base: usize,                    // 0x30
    pub entry_point: usize,                 // 0x38
    pub size_of_image: u32,                 // 0x40
    pub _pad: u32,
    pub full_dll_name: UnicodeString,       // 0x48
    pub base_dll_name: UnicodeString,       // 0x58
    pub flags: u32,                         // 0x68
    pub obsolete_load_count: u16,           // 0x6C
    pub tls_index: u16,                     // 0x6E
    pub hash_links: ListEntry,              // 0x70
    pub time_date_stamp: u32,               // 0x80
    pub _rest: [u8; 0xA0],
}

/// Process-wide PEB (allocated on first use).
pub fn peb() -> *mut u8 {
    let p = PEB.load(Ordering::Acquire);
    if p != 0 {
        return p as *mut u8;
    }
    init_peb()
}

#[cold]
fn init_peb() -> *mut u8 {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe {
        let peb = libc::calloc(1, PEB_SIZE) as *mut u8;
        let ldr = libc::calloc(1, 0x60) as *mut u8;
        set_field(ldr, 0, 0x58u32);
        set_field(ldr, 4, 1u8);
        list_init(ldr.add(0x10) as *mut ListEntry);
        list_init(ldr.add(0x20) as *mut ListEntry);
        list_init(ldr.add(0x30) as *mut ListEntry);
        set_field(peb, peb_off::LDR, ldr as usize);
        set_field(peb, peb_off::NUMBER_OF_PROCESSORS, crate::process::num_cpus());
        set_field(peb, peb_off::OS_MAJOR, OS_MAJOR);
        set_field(peb, peb_off::OS_MINOR, OS_MINOR);
        set_field(peb, peb_off::OS_BUILD, OS_BUILD as u16);
        set_field(peb, peb_off::OS_PLATFORM_ID, 2u32);
        set_field(peb, peb_off::SESSION_ID, 1u32);
        set_field(peb, peb_off::PROCESS_HEAP, crate::heap::process_heap() as usize);
        let fast_peb_lock = libc::calloc(1, std::mem::size_of::<crate::sync::CriticalSection>())
            as *mut crate::sync::CriticalSection;
        crate::sync::InitializeCriticalSection(fast_peb_lock);
        set_field(peb, peb_off::FAST_PEB_LOCK, fast_peb_lock as usize);
        let loader_lock = libc::calloc(1, std::mem::size_of::<crate::sync::CriticalSection>())
            as *mut crate::sync::CriticalSection;
        crate::sync::InitializeCriticalSection(loader_lock);
        set_field(peb, peb_off::LOADER_LOCK, loader_lock as usize);
        LDR.store(ldr as usize, Ordering::Release);
        PEB.store(peb as usize, Ordering::Release);
    });
    PEB.load(Ordering::Acquire) as *mut u8
}

/// Fill `PEB.ProcessParameters` (`RTL_USER_PROCESS_PARAMETERS`).
pub unsafe fn set_process_parameters(image_path: &str, cmdline: &str, cur_dir: &str, env_block: *mut u16) {
    let pp = libc::calloc(1, 0x400) as *mut u8;
    set_field(pp, 0, 0x400u32);
    set_field(pp, 4, 0x400u32);
    set_field(pp, 8, 1u32); // PROCESS_PARAMETERS_NORMALIZED
    let full_cmdline = crate::process::merge_manifest_args(cmdline);
    std::ptr::write(pp.add(0x38) as *mut UnicodeString, UnicodeString::leak(cur_dir));
    std::ptr::write(pp.add(0x60) as *mut UnicodeString, UnicodeString::leak(image_path));
    std::ptr::write(pp.add(0x70) as *mut UnicodeString, UnicodeString::leak(&full_cmdline));
    set_field(pp, 0x80, env_block as usize);
    set_field(peb(), peb_off::PROCESS_PARAMETERS, pp as usize);
}

pub unsafe fn set_image_base(base: usize, subsystem: u16) {
    set_field(peb(), peb_off::IMAGE_BASE, base);
    set_field(peb(), peb_off::IMAGE_SUBSYSTEM, subsystem as u32);
}

/// Add a module to the PEB loader lists (for code that walks `PEB->Ldr`).
pub unsafe fn ldr_add_module(base: usize, entry: usize, size: u32, full_name: &str, tls_index: u16) -> *mut LdrEntry {
    peb();
    let ldr = LDR.load(Ordering::Acquire) as *mut u8;
    let e = libc::calloc(1, std::mem::size_of::<LdrEntry>()) as *mut LdrEntry;
    let base_name = full_name.rsplit('\\').next().unwrap_or(full_name);
    (*e).dll_base = base;
    (*e).entry_point = entry;
    (*e).size_of_image = size;
    std::ptr::write(&mut (*e).full_dll_name, UnicodeString::leak(full_name));
    std::ptr::write(&mut (*e).base_dll_name, UnicodeString::leak(base_name));
    (*e).obsolete_load_count = 0xFFFF;
    (*e).tls_index = tls_index;
    list_init(&mut (*e).hash_links);
    list_push_back(ldr.add(0x10) as *mut ListEntry, &mut (*e).in_load_order);
    list_push_back(ldr.add(0x20) as *mut ListEntry, &mut (*e).in_memory_order);
    e
}

pub unsafe fn ldr_mark_initialized(e: *mut LdrEntry) {
    let ldr = LDR.load(Ordering::Acquire) as *mut u8;
    list_push_back(ldr.add(0x30) as *mut ListEntry, &mut (*e).in_init_order);
}

// ---------------------------------------------------------------------------
// KUSER_SHARED_DATA at 0x7FFE0000
// ---------------------------------------------------------------------------

pub const KUSER_SHARED_DATA: usize = 0x7FFE_0000;
pub const KUSER_SHARED_DATA_SIZE: usize = 0x1000;
static KUSER_MAPPED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static KUSER_SHADOW: AtomicUsize = AtomicUsize::new(0);

/// Initialize a `KUSER_SHARED_DATA` page (either at `0x7FFE0000` directly or
/// in a shadow page used by the ARM64 signal trap handler).
unsafe fn init_kuser_page(k: *mut u8) {
    set_field(k, 0x004, 0x0FA0_0000u32); // TickCountMultiplier (15.625 * 2^24)
    set_field(k, 0x02C, 0x8664u16);
    set_field(k, 0x02E, 0x8664u16);
    for (i, c) in "C:\\Windows".encode_utf16().enumerate() {
        set_field(k, 0x030 + i * 2, c);
    }
    set_field(k, 0x260, OS_BUILD);
    set_field(k, 0x264, 1u32); // NtProductWinNt
    set_field(k, 0x268, 1u8);
    set_field(k, 0x26A, 9u16); // PROCESSOR_ARCHITECTURE_AMD64
    set_field(k, 0x26C, OS_MAJOR);
    set_field(k, 0x270, OS_MINOR);
    for f in 0..64u32 {
        set_field(k, 0x274 + f as usize, crate::kernel32::sysinfo::processor_feature(f) as u8);
    }
    set_field(k, 0x2E8, (crate::process::physical_memory() / 4096) as u32);
    set_field(k, 0x300, 10_000_000i64); // QpcFrequency
    update_kuser_times(k);
}

/// Try to map KUSER_SHARED_DATA at `0x7FFE0000`. On Apple Silicon (`aarch64`),
/// XNU reserves the low 4 GB (`__PAGEZERO`), so when `0x7FFE0000` cannot be
/// mapped directly we maintain an up-to-date shadow page (`kuser_shadow_page`)
/// that the ARM64 signal handler services direct reads from.
pub fn map_kuser_shared_data() -> bool {
    unsafe {
        let p = libc::mmap(
            KUSER_SHARED_DATA as *mut libc::c_void,
            KUSER_SHARED_DATA_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        let direct = p != libc::MAP_FAILED && p as usize == KUSER_SHARED_DATA;
        let k_addr = if direct {
            KUSER_SHARED_DATA
        } else {
            if p != libc::MAP_FAILED {
                libc::munmap(p, KUSER_SHARED_DATA_SIZE);
            }
            let shadow = libc::calloc(1, KUSER_SHARED_DATA_SIZE) as usize;
            if shadow == 0 {
                log::warn!("KUSER_SHARED_DATA: shadow page allocation failed");
                return false;
            }
            shadow
        };
        init_kuser_page(k_addr as *mut u8);
        KUSER_SHADOW.store(k_addr, Ordering::Release);
        if direct {
            KUSER_MAPPED.store(true, Ordering::Release);
            log::info!("KUSER_SHARED_DATA mapped at 0x7ffe0000");
        } else {
            log::info!("KUSER_SHARED_DATA shadow page active at 0x{k_addr:x} (emulating 0x7ffe0000 reads)");
        }
        std::thread::Builder::new()
            .name("kuser-shared-data".into())
            .spawn(move || loop {
                update_kuser_times(k_addr as *mut u8);
                // Windows default system clock tick is 15.625 ms (64 Hz).
                std::thread::sleep(std::time::Duration::from_micros(15_625));
            })
            .ok();
        direct
    }
}

pub fn kuser_mapped() -> bool {
    KUSER_MAPPED.load(Ordering::Acquire)
}

/// Backing page for `KUSER_SHARED_DATA` (`0x7FFE0000` if directly mapped, or
/// the shadow page on Apple Silicon). Returns null only if `map_kuser_shared_data`
/// has not been called yet.
pub fn kuser_shadow_page() -> *const u8 {
    KUSER_SHADOW.load(Ordering::Acquire) as *const u8
}

unsafe fn write_ksystem_time(p: *mut u8, v: u64) {
    // High2Time first, then LowPart, then High1Time (Windows reader protocol).
    // Release fences prevent ARM64 store reordering so readers loading
    // High1 -> Low -> High2 cannot observe a torn 64-bit timestamp.
    std::ptr::write_volatile(p.add(8) as *mut i32, (v >> 32) as i32);
    std::sync::atomic::fence(Ordering::Release);
    std::ptr::write_volatile(p as *mut u32, v as u32);
    std::sync::atomic::fence(Ordering::Release);
    std::ptr::write_volatile(p.add(4) as *mut i32, (v >> 32) as i32);
}

fn update_kuser_times(k: *mut u8) {
    let interrupt = crate::kernel32::time::interrupt_time_100ns();
    let system = crate::types::now_filetime();
    // TickCount (KSYSTEM_TIME at 0x320) counts 15.625ms ticks so
    // `(TickCountQuad * TickCountMultiplier) >> 24` equals elapsed milliseconds.
    let ticks = interrupt / 156_250;
    unsafe {
        write_ksystem_time(k.add(0x008), interrupt);
        write_ksystem_time(k.add(0x014), system);
        std::ptr::write_volatile(k as *mut u32, ticks as u32);
        write_ksystem_time(k.add(0x320), ticks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go/no-go experiment for the hybrid runtime: TSD slots 6/11/12 can be
    /// written via gs and read back, and the host runtime keeps working.
    #[test]
    fn gs_slots_roundtrip() {
        std::thread::spawn(|| unsafe {
            assert_eq!(gs_read(0), libc::pthread_self() as usize, "gs:0 must be pthread self");
            assert_eq!(gs_read(0x30), 0);
            let teb = init_thread(0x2000, 0x1000, alloc_tid());
            assert_eq!(gs_read(0x30), teb as usize);
            assert_eq!(gs_read(0x60), peb() as usize);
            #[cfg(target_arch = "x86_64")]
            assert_eq!(libc::pthread_getspecific(6) as usize, teb as usize);
            set_tls_pointer(teb, 0x1234_5678 as *mut *mut u8);
            assert_eq!(gs_read(0x58), 0x1234_5678);
            set_last_error(1234);
            assert_eq!(*(teb.add(0x68) as *const u32), 1234);
            // Host runtime still fine.
            let v = vec![0u8; 1 << 20];
            assert_eq!(v.len(), 1 << 20);
            let mut k: libc::pthread_key_t = 0;
            assert_eq!(libc::pthread_key_create(&mut k, None), 0);
            libc::pthread_setspecific(k, 7 as *const _);
            assert_eq!(libc::pthread_getspecific(k) as usize, 7);
            uninstall();
            assert_eq!(gs_read(0x30), 0);
            free_teb(teb);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn peb_has_initialized_loader_and_fast_peb_locks() {
        unsafe {
            let p = peb();
            let ll: *const crate::sync::CriticalSection = field(p, peb_off::LOADER_LOCK);
            let fpl: *const crate::sync::CriticalSection = field(p, peb_off::FAST_PEB_LOCK);
            assert!(!ll.is_null(), "PEB->LoaderLock (+0x110) must be initialized");
            assert!(!fpl.is_null(), "PEB->FastPebLock (+0x38) must be initialized");
            assert_eq!((*ll).lock_count.load(Ordering::Relaxed), -1);
            assert_eq!((*ll).owning_thread.load(Ordering::Relaxed), 0);
            assert_eq!((*fpl).lock_count.load(Ordering::Relaxed), -1);
            assert_eq!((*fpl).owning_thread.load(Ordering::Relaxed), 0);
        }
    }
}
