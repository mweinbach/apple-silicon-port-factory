//! Virtual memory (VirtualAlloc & co.), heaps (HeapAlloc & co.), Local/Global.
//!
//! VirtualAlloc reservations are tracked with per-range state so that
//! VirtualQuery/VirtualFree behave like Windows (64K allocation granularity,
//! decommitted pages read as zero on recommit).

use crate::heap;
use crate::types::*;
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::sync::Mutex;

pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;
pub const MEM_REPLACE_PLACEHOLDER: u32 = 0x4000;
pub const MEM_DECOMMIT: u32 = 0x4000;
pub const MEM_RELEASE: u32 = 0x8000;
pub const MEM_FREE: u32 = 0x10000;
pub const MEM_PRIVATE: u32 = 0x20000;
pub const MEM_MAPPED: u32 = 0x40000;
pub const MEM_RESERVE_PLACEHOLDER: u32 = 0x40000;
pub const MEM_RESET: u32 = 0x80000;
pub const MEM_TOP_DOWN: u32 = 0x100000;
pub const MEM_WRITE_WATCH: u32 = 0x200000;
pub const MEM_RESET_UNDO: u32 = 0x1000000;
pub const MEM_LARGE_PAGES: u32 = 0x20000000;
pub const MEM_IMAGE: u32 = 0x1000000;
pub const MEM_PRESERVE_PLACEHOLDER: u32 = 0x2;
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 0x1;

pub const PAGE_NOACCESS: u32 = 0x01;
pub const PAGE_READONLY: u32 = 0x02;
pub const PAGE_READWRITE: u32 = 0x04;
pub const PAGE_WRITECOPY: u32 = 0x08;
pub const PAGE_EXECUTE: u32 = 0x10;
pub const PAGE_EXECUTE_READ: u32 = 0x20;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
pub const PAGE_GUARD: u32 = 0x100;

const PAGE: usize = 0x1000;
#[cfg(target_arch = "x86_64")]
const HOST_PAGE: usize = 0x1000;
#[cfg(not(target_arch = "x86_64"))]
const HOST_PAGE: usize = 0x4000;
const GRANULARITY: usize = 0x10000;

/// Notified with `[start, end)` whenever guest memory is committed, protected or freed while
/// executable (before or after the change). The x86-64 interpreter caches decoded code and uses
/// this to revalidate run-time generated code (V8 rewrites its code pages between toggling them
/// RWX and RX), see `aw2_game::emu::note_code_region`.
static CODE_REGION_HOOK: std::sync::OnceLock<fn(usize, usize)> = std::sync::OnceLock::new();

pub fn set_code_region_hook(f: fn(usize, usize)) {
    let _ = CODE_REGION_HOOK.set(f);
}

#[inline]
fn is_exec(prot: u32) -> bool {
    prot & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
}

fn note_code_region(start: usize, end: usize) {
    if let Some(f) = CODE_REGION_HOOK.get() {
        f(start, end);
    }
}

unsafe fn host_mprotect(addr: usize, len: usize, prot: i32) -> i32 {
    let start = addr & !(HOST_PAGE - 1);
    let end = (addr + len + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    #[cfg(not(target_arch = "x86_64"))]
    let prot = if prot & libc::PROT_WRITE != 0 {
        prot & !libc::PROT_EXEC
    } else {
        prot
    };
    libc::mprotect(start as *mut c_void, end - start, prot)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemAddressRequirements {
    pub lowest_starting_address: usize,
    pub highest_ending_address: usize,
    pub alignment: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemExtendedParameter {
    pub ty_flags: u64,
    pub value: u64,
}

pub unsafe fn parse_extended_alignment(ext: *const c_void, count: u32) -> Result<usize, u32> {
    if count == 0 || ext.is_null() {
        return Ok(GRANULARITY);
    }
    let params = std::slice::from_raw_parts(ext as *const MemExtendedParameter, count as usize);
    let mut align = GRANULARITY;
    for p in params {
        let ty = (p.ty_flags & 0xFF) as u32;
        if ty == 1 {
            // MemExtendedParameterAddressRequirements
            if p.value != 0 {
                let req = &*(p.value as *const MemAddressRequirements);
                if req.alignment != 0 {
                    if !req.alignment.is_power_of_two() || req.alignment < GRANULARITY {
                        return Err(err::ERROR_INVALID_PARAMETER);
                    }
                    align = req.alignment;
                }
            }
        }
    }
    Ok(align)
}

pub fn win_prot_to_host(p: u32) -> i32 {
    match p & 0xff {
        PAGE_NOACCESS => libc::PROT_NONE,
        PAGE_READONLY => libc::PROT_READ,
        PAGE_READWRITE | PAGE_WRITECOPY => libc::PROT_READ | libc::PROT_WRITE,
        PAGE_EXECUTE => libc::PROT_EXEC | libc::PROT_READ,
        PAGE_EXECUTE_READ => libc::PROT_READ | libc::PROT_EXEC,
        PAGE_EXECUTE_WRITECOPY | PAGE_EXECUTE_READWRITE => libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        _ => libc::PROT_READ | libc::PROT_WRITE,
    }
}

#[derive(Clone, Copy, Debug)]
struct Seg {
    end: usize,
    state: u32, // MEM_COMMIT / MEM_RESERVE
    prot: u32,
}

#[derive(Clone, Copy, Debug)]
struct Alloc {
    size: usize,
    prot: u32,
    kind: u32, // MEM_PRIVATE / MEM_IMAGE / MEM_MAPPED
}

struct VmState {
    allocs: BTreeMap<usize, Alloc>,
    segs: BTreeMap<usize, Seg>,
}

static VM: Mutex<VmState> = Mutex::new(VmState { allocs: BTreeMap::new(), segs: BTreeMap::new() });
/// Set once any committed page in `VM` is marked `PAGE_NOACCESS` or `PAGE_GUARD`
/// (e.g. packed executables using demand-decryption). Lets speculative JIT code
/// readers avoid taking `VM.lock()` on normal games and multi-threaded `.aot` prewarm.
static HAS_NOACCESS_PAGES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

impl VmState {
    fn alloc_containing(&self, addr: usize) -> Option<(usize, Alloc)> {
        let (&b, a) = self.allocs.range(..=addr).next_back()?;
        (addr < b + a.size).then_some((b, *a))
    }
    /// Split segments so that `at` is a segment boundary.
    fn split(&mut self, at: usize) {
        if let Some((&s, seg)) = self.segs.range(..at).next_back() {
            if seg.end > at {
                let seg = *seg;
                self.segs.insert(s, Seg { end: at, ..seg });
                self.segs.insert(at, seg);
            }
        }
    }
    fn set_range(&mut self, start: usize, end: usize, state: u32, prot: u32) {
        if state == MEM_COMMIT && ((prot & 0xFF) == PAGE_NOACCESS || (prot & PAGE_GUARD) != 0) {
            HAS_NOACCESS_PAGES.store(true, std::sync::atomic::Ordering::Release);
        }
        self.split(start);
        self.split(end);
        let keys: Vec<usize> = self.segs.range(start..end).map(|(k, _)| *k).collect();
        for k in keys {
            self.segs.remove(&k);
        }
        self.segs.insert(start, Seg { end, state, prot });
    }
    fn remove_range(&mut self, start: usize, end: usize) {
        self.split(start);
        self.split(end);
        let keys: Vec<usize> = self.segs.range(start..end).map(|(k, _)| *k).collect();
        for k in keys {
            self.segs.remove(&k);
        }
    }
}

/// Register a PE image (or other host-managed range) so VirtualQuery
/// reports it as MEM_IMAGE with the given per-range protections.
pub fn register_image(base: usize, size: usize, ranges: &[(usize, usize, u32)]) {
    let mut vm = VM.lock().unwrap();
    vm.allocs.insert(base, Alloc { size, prot: PAGE_EXECUTE_WRITECOPY, kind: MEM_IMAGE });
    vm.set_range(base, base + size, MEM_COMMIT, PAGE_READONLY);
    for &(s, e, p) in ranges {
        vm.set_range(s, e, MEM_COMMIT, p);
    }
}

/// Returns `true` if `addr` lies inside a guest-owned allocation tracked in `VM`
/// (registered PE image, `VirtualAlloc`, or `MapViewOfFile` — never host Mach-O code).
pub fn is_guest_vm_addr(addr: usize) -> bool {
    let page = addr & !(PAGE - 1);
    VM.lock().unwrap().alloc_containing(page).is_some()
}

/// Returns `false` if any 16 KiB host page overlapping `[addr, addr + len)` contains
/// a `PAGE_NOACCESS`, `PAGE_GUARD`, or uncommitted (`MEM_RESERVE`) guest page that would
/// fault a speculative read (such as JIT successor flag-liveness lookahead).
/// Fast-paths lock-free when the process has never created protected/noaccess pages.
pub fn is_speculative_read_safe(addr: usize, len: usize) -> bool {
    if !HAS_NOACCESS_PAGES.load(std::sync::atomic::Ordering::Acquire) {
        return true;
    }
    let Some(end) = addr.checked_add(len) else {
        return false;
    };
    let hstart = addr & !(HOST_PAGE - 1);
    let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    let vm = VM.lock().unwrap();
    let mut sub = hstart;
    while sub < hend {
        if let Some((&_s, seg)) = vm.segs.range(..=sub).next_back() {
            if sub < seg.end {
                if seg.state != MEM_COMMIT
                    || (seg.prot & 0xFF) == PAGE_NOACCESS
                    || (seg.prot & PAGE_GUARD) != 0
                {
                    return false;
                }
            }
        }
        sub += PAGE;
    }
    true
}

pub fn update_protection(start: usize, end: usize, prot: u32) {
    let mut vm = VM.lock().unwrap();
    vm.set_range(start, end, MEM_COMMIT, prot);
}

/// Record a mapped view at `[base, base + size)` in VM tracking (used by MapViewOfFile / MapViewOfFile3).
pub fn register_mapped_view(base: usize, size: usize, prot: u32) {
    {
        let mut vm = VM.lock().unwrap();
        vm.allocs.insert(base, Alloc { size, prot, kind: MEM_MAPPED });
        vm.set_range(base, base + size, MEM_COMMIT, prot);
    }
    if is_exec(prot) {
        note_code_region(base, base + size);
    }
}

pub fn unregister_mapped_view(base: usize, size: usize) {
    let mut vm = VM.lock().unwrap();
    vm.allocs.remove(&base);
    vm.remove_range(base, base + size);
}

/// Total bytes currently `MEM_COMMIT` inside guest `MEM_PRIVATE` `VirtualAlloc` regions.
pub fn committed_private_bytes() -> u64 {
    let vm = VM.lock().unwrap();
    let mut total = 0u64;
    for (&start, seg) in &vm.segs {
        if seg.state == MEM_COMMIT {
            if let Some((_, a)) = vm.alloc_containing(start) {
                if a.kind == MEM_PRIVATE {
                    total = total.saturating_add(seg.end.saturating_sub(start) as u64);
                }
            }
        }
    }
    total
}

unsafe fn mmap_reserve(addr: usize, size: usize) -> Option<usize> {
    let p = libc::mmap(addr as *mut c_void, size, libc::PROT_NONE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
    if p == libc::MAP_FAILED {
        return None;
    }
    Some(p as usize)
}

/// Reserve `size` bytes aligned to `align` (or exactly at `addr` if non-zero).
unsafe fn reserve_aligned(addr: usize, size: usize, align: usize) -> Option<usize> {
    let align = align.max(GRANULARITY);
    if addr != 0 {
        let p = mmap_reserve(addr, size)?;
        if p != addr {
            libc::munmap(p as *mut c_void, size);
            return None;
        }
        return Some(p);
    }
    let p = mmap_reserve(0, size + align)?;
    let aligned = (p + align - 1) & !(align - 1);
    if aligned > p {
        libc::munmap(p as *mut c_void, aligned - p);
    }
    let tail = p + size + align - (aligned + size);
    if tail > 0 {
        libc::munmap((aligned + size) as *mut c_void, tail);
    }
    Some(aligned)
}

pub unsafe fn virtual_alloc(addr: usize, size: usize, ty: u32, prot: u32) -> Result<usize, u32> {
    virtual_alloc_aligned(addr, size, ty, prot, GRANULARITY)
}

pub unsafe fn virtual_alloc_aligned(addr: usize, size: usize, ty: u32, prot: u32, align: usize) -> Result<usize, u32> {
    let r = virtual_alloc_aligned_impl(addr, size, ty, prot, align);
    if let Ok(p) = r {
        if is_exec(prot) && ty & MEM_COMMIT != 0 {
            let start = p & !(PAGE - 1);
            note_code_region(start, (p.max(addr) + size + PAGE - 1) & !(PAGE - 1));
        }
    }
    r
}

unsafe fn virtual_alloc_aligned_impl(addr: usize, size: usize, ty: u32, prot: u32, align: usize) -> Result<usize, u32> {
    if size == 0 {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    if memdiag() && (prot & 0xF0 != 0 || ty & MEM_RESET != 0) {
        memdiag_log(format_args!("VirtualAlloc(addr={addr:#x}, size={size:#x}, ty={ty:#x}, prot={prot:#x})"));
    }
    if ty & MEM_LARGE_PAGES != 0 {
        stub!("MEM_LARGE_PAGES ignored");
    }
    if ty & MEM_WRITE_WATCH != 0 {
        stub!("MEM_WRITE_WATCH not tracked");
    }
    if prot & PAGE_GUARD != 0 {
        stub!("PAGE_GUARD not emulated (treated as normal pages)");
    }
    if ty & MEM_RESET != 0 {
        discard_host_pages(addr, size);
        return Ok(addr);
    }
    if ty & MEM_RESET_UNDO != 0 {
        stub!("MEM_RESET_UNDO: contents may have been discarded");
        return Ok(addr);
    }
    let mut vm = VM.lock().unwrap();
    if ty & MEM_RESERVE != 0 && ty & MEM_REPLACE_PLACEHOLDER != 0 {
        // Replace an existing placeholder at `addr` of `size`.
        if addr == 0 {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let sz = (size + PAGE - 1) & !(PAGE - 1);
        vm.allocs.insert(addr, Alloc { size: sz, prot, kind: MEM_PRIVATE });
        vm.set_range(addr, addr + sz, MEM_RESERVE, 0);
        if ty & MEM_COMMIT != 0 {
            if host_mprotect(addr, sz, win_prot_to_host(prot)) != 0 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            vm.set_range(addr, addr + sz, MEM_COMMIT, prot);
        }
        return Ok(addr);
    }
    if ty & (MEM_RESERVE | MEM_RESERVE_PLACEHOLDER) != 0 {
        let a = addr & !(GRANULARITY - 1);
        let sz = ((addr + size + PAGE - 1) & !(PAGE - 1)) - if addr != 0 { a } else { 0 };
        let base = reserve_aligned(a, sz, align).ok_or(if addr != 0 { err::ERROR_INVALID_ADDRESS } else { err::ERROR_NOT_ENOUGH_MEMORY })?;
        vm.allocs.insert(base, Alloc { size: sz, prot, kind: MEM_PRIVATE });
        vm.set_range(base, base + sz, MEM_RESERVE, 0);
        if ty & MEM_COMMIT != 0 {
            if host_mprotect(base, sz, win_prot_to_host(prot)) != 0 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            vm.set_range(base, base + sz, MEM_COMMIT, prot);
        }
        return Ok(if addr != 0 { addr & !(PAGE - 1) } else { base });
    }
    if ty & MEM_COMMIT != 0 {
        if addr == 0 {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let start = addr & !(PAGE - 1);
        let end = (addr + size + PAGE - 1) & !(PAGE - 1);
        let Some((b, a)) = vm.alloc_containing(start) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        if end > b + a.size {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        if host_mprotect(start, end - start, win_prot_to_host(prot)) != 0 {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        vm.set_range(start, end, MEM_COMMIT, prot);
        return Ok(addr);
    }
    Err(err::ERROR_INVALID_PARAMETER)
}

pub unsafe fn virtual_free(addr: usize, size: usize, ty: u32) -> Result<(), u32> {
    let mut vm = VM.lock().unwrap();
    if ty & MEM_RELEASE != 0 && ty & MEM_PRESERVE_PLACEHOLDER != 0 {
        // Split placeholder at [addr, addr + size).
        if size == 0 {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        let Some((b, a)) = vm.alloc_containing(addr) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        let end = addr + size;
        if end > b + a.size {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        vm.allocs.remove(&b);
        if addr > b {
            vm.allocs.insert(b, Alloc { size: addr - b, ..a });
        }
        vm.allocs.insert(addr, Alloc { size, ..a });
        if end < b + a.size {
            vm.allocs.insert(end, Alloc { size: (b + a.size) - end, ..a });
        }
        vm.split(addr);
        vm.split(end);
        return Ok(());
    }
    if ty & MEM_RELEASE != 0 && ty & MEM_COALESCE_PLACEHOLDERS != 0 {
        if size == 0 {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        let end = addr + size;
        let keys: Vec<usize> = vm.allocs.range(addr..end).map(|(k, _)| *k).collect();
        if keys.is_empty() {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let first = vm.allocs.get(&keys[0]).copied().unwrap();
        for k in keys {
            vm.allocs.remove(&k);
        }
        vm.allocs.insert(addr, Alloc { size, ..first });
        vm.set_range(addr, end, MEM_RESERVE, 0);
        return Ok(());
    }
    if ty & MEM_RELEASE != 0 {
        let Some((b, a)) = vm.alloc_containing(addr) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        if a.kind != MEM_PRIVATE {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        if size == 0 {
            if addr != b {
                return Err(err::ERROR_INVALID_ADDRESS);
            }
            libc::munmap(b as *mut c_void, a.size);
            vm.allocs.remove(&b);
            vm.remove_range(b, b + a.size);
            return Ok(());
        }
        // mimalloc over-allocates with VirtualAlloc(MEM_RESERVE) and then trims
        // the unaligned head/tail with VirtualFree(ptr, trim_size, MEM_RELEASE)
        // before calling VirtualAlloc(aligned_ptr, seg_size, MEM_COMMIT).
        // Split the allocation record so the remaining middle range stays valid!
        let end = (addr + size + PAGE - 1) & !(PAGE - 1);
        if addr < b || end > b + a.size {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        libc::munmap(addr as *mut c_void, end - addr);
        vm.allocs.remove(&b);
        if addr > b {
            vm.allocs.insert(b, Alloc { size: addr - b, ..a });
        }
        if end < b + a.size {
            vm.allocs.insert(end, Alloc { size: (b + a.size) - end, ..a });
        }
        vm.remove_range(addr, end);
        return Ok(());
    }
    if ty & MEM_DECOMMIT != 0 {
        let Some((b, a)) = vm.alloc_containing(addr) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        let start = addr & !(PAGE - 1);
        let end = if size == 0 { b + a.size } else { (addr + size + PAGE - 1) & !(PAGE - 1) };
        if end > start {
            // Always release every complete 16 KiB host page inside [start, end) via
            // `mmap(PROT_NONE, MAP_FIXED)` so physical RAM is returned immediately to
            // XNU (`phys_footprint` / iOS Jetsam), and only zero any <16 KiB unaligned
            // head/tail sub-pages so a later recommit reads zero.
            let inner_start = (start + HOST_PAGE - 1) & !(HOST_PAGE - 1);
            let inner_end = end & !(HOST_PAGE - 1);
            if inner_end > inner_start {
                let p = libc::mmap(
                    inner_start as *mut c_void,
                    inner_end - inner_start,
                    libc::PROT_NONE,
                    libc::MAP_PRIVATE | libc::MAP_ANON | libc::MAP_FIXED,
                    -1,
                    0,
                );
                if p == libc::MAP_FAILED {
                    return Err(err::ERROR_INVALID_PARAMETER);
                }
                if start < inner_start {
                    let _ = host_mprotect(start, inner_start - start, libc::PROT_READ | libc::PROT_WRITE);
                    std::ptr::write_bytes(start as *mut u8, 0, inner_start - start);
                }
                if inner_end < end {
                    let _ = host_mprotect(inner_end, end - inner_end, libc::PROT_READ | libc::PROT_WRITE);
                    std::ptr::write_bytes(inner_end as *mut u8, 0, end - inner_end);
                }
            } else {
                // Entire range lies inside one or two partial 16 KiB host pages.
                let _ = host_mprotect(start, end - start, libc::PROT_READ | libc::PROT_WRITE);
                std::ptr::write_bytes(start as *mut u8, 0, end - start);
            }
        }
        vm.set_range(start, end, MEM_RESERVE, 0);
        return Ok(());
    }
    Err(err::ERROR_INVALID_PARAMETER)
}

pub unsafe extern "win64" fn VirtualAlloc(addr: *mut c_void, size: usize, ty: u32, prot: u32) -> *mut c_void {
    match virtual_alloc(addr as usize, size, ty, prot) {
        Ok(p) => p as *mut c_void,
        Err(e) => {
            log::debug!("VirtualAlloc({addr:p}, 0x{size:x}, 0x{ty:x}, 0x{prot:x}) failed: {e}");
            crate::teb::set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn VirtualAllocEx(proc_: HANDLE, addr: *mut c_void, size: usize, ty: u32, prot: u32) -> *mut c_void {
    if !proc_.is_null() && proc_ != crate::handles::CURRENT_PROCESS {
        stub!("VirtualAllocEx on another process");
        crate::teb::set_last_error(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    VirtualAlloc(addr, size, ty, prot)
}

pub unsafe extern "win64" fn VirtualAlloc2(proc_: HANDLE, addr: *mut c_void, size: usize, ty: u32, prot: u32, ext: *mut c_void, next: u32) -> *mut c_void {
    if !proc_.is_null() && proc_ != crate::handles::CURRENT_PROCESS {
        stub!("VirtualAlloc2 on another process");
        crate::teb::set_last_error(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    let align = match parse_extended_alignment(ext, next) {
        Ok(a) => a,
        Err(e) => {
            crate::teb::set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    match virtual_alloc_aligned(addr as usize, size, ty, prot, align) {
        Ok(p) => p as *mut c_void,
        Err(e) => {
            log::debug!("VirtualAlloc2({addr:p}, 0x{size:x}, 0x{ty:x}, 0x{prot:x}, align=0x{align:x}) failed: {e}");
            crate::teb::set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn VirtualAlloc2FromApp(proc_: HANDLE, addr: *mut c_void, size: usize, ty: u32, prot: u32, ext: *mut c_void, next: u32) -> *mut c_void {
    VirtualAlloc2(proc_, addr, size, ty, prot, ext, next)
}

pub unsafe extern "win64" fn GetWriteWatch(
    _flags: u32,
    _base: *mut c_void,
    _region_size: usize,
    _addresses: *mut *mut c_void,
    count: *mut usize,
    granularity: *mut u32,
) -> u32 {
    if !count.is_null() {
        *count = 0;
    }
    if !granularity.is_null() {
        *granularity = PAGE as u32;
    }
    0
}

pub unsafe extern "win64" fn ResetWriteWatch(_base: *mut c_void, _region_size: usize) -> u32 {
    0
}

pub unsafe extern "win64" fn VirtualFree(addr: *mut c_void, size: usize, ty: u32) -> BOOL {
    let was_exec = query(addr as usize).filter(|i| is_exec(i.protect)).map(|i| (i.base_address, i.region_size));
    match virtual_free(addr as usize, size, ty) {
        Ok(()) => {
            if let Some((b, s)) = was_exec {
                let end = if size == 0 { b + s } else { (addr as usize + size + PAGE - 1) & !(PAGE - 1) };
                note_code_region(addr as usize & !(PAGE - 1), end.max(addr as usize + PAGE));
            }
            TRUE
        }
        Err(e) => {
            crate::teb::set_last_error(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn VirtualFreeEx(_p: HANDLE, addr: *mut c_void, size: usize, ty: u32) -> BOOL {
    VirtualFree(addr, size, ty)
}

fn memdiag() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_MEMDIAG").is_ok_and(|s| s.trim() == "1"))
}

fn memdiag_log(args: std::fmt::Arguments) {
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    if n <= 200 || n.is_power_of_two() {
        log::warn!("memdiag #{n}: {args}");
    }
}

static RECENT_HPAGE_PROT: [std::sync::atomic::AtomicUsize; 512] =
    [const { std::sync::atomic::AtomicUsize::new(0) }; 512];

fn note_hpage_prot_change(start: usize, end: usize) {
    let mut hp = start & !(HOST_PAGE - 1);
    let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    while hp < hend {
        RECENT_HPAGE_PROT[(hp >> 14) & 511].store(hp, std::sync::atomic::Ordering::Release);
        hp += HOST_PAGE;
    }
}

/// True if `VirtualProtect` recently transitioned protection involving `PAGE_NOACCESS` /
/// `PAGE_GUARD` on the 16 KiB host page containing `hpage`.
pub fn hpage_had_prot_change(hpage: usize) -> bool {
    let aligned = hpage & !(HOST_PAGE - 1);
    RECENT_HPAGE_PROT[(aligned >> 14) & 511].load(std::sync::atomic::Ordering::Acquire) == aligned
}

pub unsafe extern "win64" fn VirtualProtect(addr: *mut c_void, size: usize, prot: u32, old: *mut u32) -> BOOL {
    let start = addr as usize & !(PAGE - 1);
    let end = (addr as usize + size + PAGE - 1) & !(PAGE - 1);
    if prot & PAGE_GUARD != 0 {
        stub!("PAGE_GUARD not emulated");
    }
    let prev = query(start).map(|i| i.protect).unwrap_or(PAGE_READWRITE);
    if memdiag() && (prot | prev) & 0xF0 != 0 {
        memdiag_log(format_args!("VirtualProtect({addr:p}, {size:#x}, {prot:#x}) prev={prev:#x}"));
    }
    let mut host_prot = win_prot_to_host(prot);
    let mut deferred_noaccess_sibling = false;
    let mprotect_rc = {
        let mut vm = VM.lock().unwrap();
        if vm.alloc_containing(start).is_some() {
            vm.set_range(start, end, MEM_COMMIT, prot);
        }
        // On 16 KiB host pages (Apple Silicon), if a sub-host-page non-writable
        // VirtualProtect (e.g. PAGE_EXECUTE_READ after decrypting one 4 KiB page)
        // still shares its 16 KiB hardware page with another PAGE_NOACCESS 4 KiB
        // sub-page, keep the host page PROT_NONE until all sibling sub-pages are
        // unlocked so other threads cannot race into still-encrypted siblings.
        let is_writable = matches!(prot & 0xFF, PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY);
        if HOST_PAGE > PAGE && !is_writable && prot != PAGE_NOACCESS && (prot & PAGE_GUARD) == 0 && (end - start) < HOST_PAGE {
            let hbase = start & !(HOST_PAGE - 1);
            let mut sub = hbase;
            while sub < hbase + HOST_PAGE {
                if let Some((&_s, seg)) = vm.segs.range(..=sub).next_back() {
                    if sub < seg.end && seg.state == MEM_COMMIT && (seg.prot == PAGE_NOACCESS || (seg.prot & PAGE_GUARD) != 0) {
                        deferred_noaccess_sibling = true;
                        break;
                    }
                }
                sub += PAGE;
            }
            if deferred_noaccess_sibling {
                host_prot = libc::PROT_NONE;
            }
        }
        if prev == PAGE_NOACCESS
            || (prev & PAGE_GUARD) != 0
            || prot == PAGE_NOACCESS
            || (prot & PAGE_GUARD) != 0
            || deferred_noaccess_sibling
        {
            note_hpage_prot_change(start, end);
        }
        // Keep `host_mprotect` serialized under `VM` so concurrent VirtualProtect calls
        // on sibling 4 KiB sub-pages within one 16 KiB host page cannot apply out of order.
        host_mprotect(start, end - start, host_prot)
    };
    if mprotect_rc != 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_ADDRESS);
        return FALSE;
    }
    if !old.is_null() {
        *old = if prev == 0 { PAGE_NOACCESS } else { prev };
    }
    if is_exec(prot) || is_exec(prev) {
        note_code_region(start, end);
    }
    TRUE
}

/// Ensure the 16 KiB host page containing `addr` has at least `PROT_READ`
/// (fallback when a sibling 4 KiB sub-page is permanently `PAGE_NOACCESS`).
pub unsafe fn ensure_host_page_readable(addr: usize) {
    let _ = host_mprotect(addr, PAGE, libc::PROT_READ);
}

pub unsafe extern "win64" fn VirtualProtectEx(_p: HANDLE, addr: *mut c_void, size: usize, prot: u32, old: *mut u32) -> BOOL {
    VirtualProtect(addr, size, prot, old)
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct MemoryBasicInformation {
    pub base_address: usize,
    pub allocation_base: usize,
    pub allocation_protect: u32,
    pub partition_id: u16,
    pub _pad: u16,
    pub region_size: usize,
    pub state: u32,
    pub protect: u32,
    pub ty: u32,
    pub _pad2: u32,
}

pub fn query(addr: usize) -> Option<MemoryBasicInformation> {
    let page = addr & !(PAGE - 1);
    let vm = VM.lock().unwrap();
    if let Some((b, a)) = vm.alloc_containing(page) {
        let (&s, seg) = vm.segs.range(..=page).next_back()?;
        let _ = s;
        // Extend over following contiguous segments with identical state/protection.
        let mut end = seg.end;
        for (&ns, n) in vm.segs.range(seg.end..b + a.size) {
            if ns == end && n.state == seg.state && n.prot == seg.prot {
                end = n.end;
            } else {
                break;
            }
        }
        return Some(MemoryBasicInformation {
            base_address: page,
            allocation_base: b,
            allocation_protect: a.prot,
            partition_id: 0,
            _pad: 0,
            region_size: end - page,
            state: seg.state,
            protect: if seg.state == MEM_COMMIT { seg.prot } else { 0 },
            ty: a.kind,
            _pad2: 0,
        });
    }
    drop(vm);
    // Thread stacks: report the whole stack as one committed region.
    for t in crate::thread::all_tebs().into_iter().chain(std::iter::once(crate::teb::current_raw())) {
        if t.is_null() {
            continue;
        }
        let (lo, hi): (usize, usize) = unsafe {
            (crate::teb::field(t, crate::teb::off::DEALLOCATION_STACK), crate::teb::field(t, crate::teb::off::STACK_BASE))
        };
        if page >= lo && page < hi {
            return Some(MemoryBasicInformation {
                base_address: page,
                allocation_base: lo,
                allocation_protect: PAGE_READWRITE,
                partition_id: 0,
                _pad: 0,
                region_size: hi - page,
                state: MEM_COMMIT,
                protect: PAGE_READWRITE,
                ty: MEM_PRIVATE,
                _pad2: 0,
            });
        }
    }
    host_query(page)
}

#[repr(C, packed(4))]
struct VmRegionBasicInfo64 {
    protection: i32,
    max_protection: i32,
    inheritance: u32,
    shared: u32,
    reserved: u32,
    offset: u64,
    behavior: i32,
    user_wired_count: u16,
}
const VM_REGION_BASIC_INFO_64: i32 = 9;
extern "C" {
    fn mach_vm_region(
        task: libc::mach_port_t,
        address: *mut u64,
        size: *mut u64,
        flavor: i32,
        info: *mut i32,
        count: *mut u32,
        object_name: *mut libc::mach_port_t,
    ) -> i32;
}

/// Fallback for host-owned memory (malloc heap, host libraries): ask Mach.
fn host_query(page: usize) -> Option<MemoryBasicInformation> {
    extern "C" {
        static mach_task_self_: libc::mach_port_t;
    }
    unsafe {
        let mut address: u64 = page as u64;
        let mut size: u64 = 0;
        let mut info: VmRegionBasicInfo64 = std::mem::zeroed();
        let mut count = (std::mem::size_of::<VmRegionBasicInfo64>() / 4) as u32;
        let mut obj: libc::mach_port_t = 0;
        let task = mach_task_self_;
        let rc = mach_vm_region(
            task,
            &mut address,
            &mut size,
            VM_REGION_BASIC_INFO_64,
            &mut info as *mut _ as *mut i32,
            &mut count,
            &mut obj,
        );
        if rc != 0 {
            return None;
        }
        if address as usize > page {
            // Free gap before the next region.
            return Some(MemoryBasicInformation {
                base_address: page,
                allocation_base: 0,
                allocation_protect: 0,
                partition_id: 0,
                _pad: 0,
                region_size: address as usize - page,
                state: MEM_FREE,
                protect: PAGE_NOACCESS,
                ty: 0,
                _pad2: 0,
            });
        }
        let p = info.protection;
        let prot = match (p & 1 != 0, p & 2 != 0, p & 4 != 0) {
            (_, _, true) if p & 2 != 0 => PAGE_EXECUTE_READWRITE,
            (_, _, true) => PAGE_EXECUTE_READ,
            (_, true, _) => PAGE_READWRITE,
            (true, _, _) => PAGE_READONLY,
            _ => PAGE_NOACCESS,
        };
        Some(MemoryBasicInformation {
            base_address: page,
            allocation_base: address as usize,
            allocation_protect: prot,
            partition_id: 0,
            _pad: 0,
            region_size: (address + size) as usize - page,
            state: MEM_COMMIT,
            protect: prot,
            ty: MEM_PRIVATE,
            _pad2: 0,
        })
    }
}

pub unsafe extern "win64" fn VirtualQuery(addr: *const c_void, out: *mut MemoryBasicInformation, len: usize) -> usize {
    if len < std::mem::size_of::<MemoryBasicInformation>() {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    match query(addr as usize) {
        Some(i) => {
            *out = i;
            std::mem::size_of::<MemoryBasicInformation>()
        }
        None => {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            0
        }
    }
}

pub unsafe extern "win64" fn VirtualQueryEx(_p: HANDLE, addr: *const c_void, out: *mut MemoryBasicInformation, len: usize) -> usize {
    VirtualQuery(addr, out, len)
}

fn is_self_process_handle(h: HANDLE) -> bool {
    h == crate::handles::CURRENT_PROCESS
        || crate::handles::get_as::<crate::kernel32::process::ProcessObj>(h).is_some()
}

pub unsafe extern "win64" fn ReadProcessMemory(
    proc_: HANDLE,
    base_addr: *const c_void,
    buf: *mut c_void,
    size: usize,
    bytes_read: *mut usize,
) -> BOOL {
    if !bytes_read.is_null() {
        std::ptr::write_unaligned(bytes_read, 0);
    }
    if !is_self_process_handle(proc_) {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if buf.is_null() && size > 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if size == 0 {
        return TRUE;
    }
    let addr = base_addr as usize;
    // Handle KUSER_SHARED_DATA shadow page reads on Apple Silicon.
    const KUSER_BASE: usize = 0x7FFE_0000;
    const KUSER_END: usize = 0x7FFE_1000;
    if addr >= KUSER_BASE && addr.saturating_add(size) <= KUSER_END {
        let shadow = crate::teb::kuser_shadow_page();
        std::ptr::copy_nonoverlapping(shadow.add(addr - KUSER_BASE), buf as *mut u8, size);
        if !bytes_read.is_null() {
            std::ptr::write_unaligned(bytes_read, size);
        }
        return TRUE;
    }
    extern "C" {
        static mach_task_self_: u32;
        fn mach_vm_read_overwrite(task: u32, addr: u64, size: u64, data: u64, outsize: *mut u64) -> i32;
    }
    let mut got: u64 = 0;
    let kr = mach_vm_read_overwrite(mach_task_self_, addr as u64, size as u64, buf as u64, &mut got);
    if kr != 0 {
        crate::teb::set_last_error(err::ERROR_PARTIAL_COPY);
        return FALSE;
    }
    if !bytes_read.is_null() {
        std::ptr::write_unaligned(bytes_read, got as usize);
    }
    TRUE
}

pub unsafe extern "win64" fn WriteProcessMemory(
    proc_: HANDLE,
    base_addr: *mut c_void,
    buf: *const c_void,
    size: usize,
    bytes_written: *mut usize,
) -> BOOL {
    if !bytes_written.is_null() {
        std::ptr::write_unaligned(bytes_written, 0);
    }
    if !is_self_process_handle(proc_) {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if (base_addr.is_null() || buf.is_null()) && size > 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if size == 0 {
        return TRUE;
    }
    std::ptr::copy_nonoverlapping(buf as *const u8, base_addr as *mut u8, size);
    let _ = FlushInstructionCache(proc_, base_addr, size);
    if !bytes_written.is_null() {
        std::ptr::write_unaligned(bytes_written, size);
    }
    TRUE
}

pub unsafe extern "win64" fn VirtualLock(_a: *mut c_void, _s: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn VirtualUnlock(_a: *mut c_void, _s: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn FlushInstructionCache(_p: HANDLE, a: *const c_void, s: usize) -> BOOL {
    if !a.is_null() && s > 0 {
        let start = (a as usize) & !(PAGE - 1);
        let end = ((a as usize).saturating_add(s) + PAGE - 1) & !(PAGE - 1);
        note_code_region(start, end);
    }
    TRUE
}

pub unsafe extern "win64" fn FlushProcessWriteBuffers() {
    // Windows issues an IPI to every CPU. A local SeqCst fence is weaker;
    // documented so it can be revisited if lock-free code misbehaves.
    stub!("FlushProcessWriteBuffers approximated by a local fence");
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

pub unsafe extern "win64" fn PrefetchVirtualMemory(_p: HANDLE, _n: usize, _e: *mut c_void, _f: u32) -> BOOL {
    TRUE
}

/// Release only the complete host pages lying entirely inside `[addr, addr + size)`.
///
/// Windows discards at 4 KB granularity but Apple Silicon host pages are 16 KB, and the kernel
/// rounds an `madvise` range outward to whole host pages. Using `MADV_FREE_REUSABLE` (9 on
/// Darwin) immediately deducts reclaimed pages from XNU's `phys_footprint` ledger (critical
/// for staying under iOS Jetsam limits), falling back to `MADV_FREE` if rejected.
unsafe fn discard_host_pages(addr: usize, size: usize) {
    const MADV_FREE_REUSABLE: i32 = 9;
    let start = (addr + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    let end = (addr + size) & !(HOST_PAGE - 1);
    if end > start {
        if libc::madvise(start as *mut c_void, end - start, MADV_FREE_REUSABLE) != 0 {
            libc::madvise(start as *mut c_void, end - start, libc::MADV_FREE);
        }
    }
}

pub unsafe extern "win64" fn DiscardVirtualMemory(a: *mut c_void, s: usize) -> u32 {
    if memdiag() {
        memdiag_log(format_args!("DiscardVirtualMemory({a:p}, {s:#x})"));
    }
    discard_host_pages(a as usize, s);
    0
}

pub unsafe extern "win64" fn OfferVirtualMemory(_a: *mut c_void, _s: usize, _p: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn ReclaimVirtualMemory(_a: *const c_void, _s: usize) -> u32 {
    0
}

// ---------------------------------------------------------------------------
// Heaps
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetProcessHeap() -> HANDLE {
    heap::process_heap()
}

pub unsafe extern "win64" fn GetProcessHeaps(n: u32, out: *mut HANDLE) -> u32 {
    if n >= 1 && !out.is_null() {
        *out = heap::process_heap();
    }
    1
}

pub unsafe extern "win64" fn HeapCreate(_opts: u32, _init: usize, _max: usize) -> HANDLE {
    heap::create_heap()
}

pub unsafe extern "win64" fn HeapDestroy(_h: HANDLE) -> BOOL {
    // Blocks of a destroyed heap are not tracked per heap; they leak.
    stub!("HeapDestroy does not free outstanding blocks");
    TRUE
}

pub unsafe extern "win64" fn HeapAlloc(_h: HANDLE, flags: u32, size: usize) -> *mut c_void {
    let p = heap::alloc(size, flags & heap::HEAP_ZERO_MEMORY != 0);
    if p.is_null() {
        crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
    }
    p
}

pub unsafe extern "win64" fn HeapFree(_h: HANDLE, _flags: u32, p: *mut c_void) -> BOOL {
    if heap::free(p) {
        TRUE
    } else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn HeapReAlloc(_h: HANDLE, flags: u32, p: *mut c_void, size: usize) -> *mut c_void {
    let r = heap::realloc(p, size, flags & heap::HEAP_ZERO_MEMORY != 0, flags & heap::HEAP_REALLOC_IN_PLACE_ONLY != 0);
    if r.is_null() {
        crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
    }
    r
}

pub unsafe extern "win64" fn HeapSize(_h: HANDLE, _flags: u32, p: *const c_void) -> usize {
    heap::size(p as *mut c_void).unwrap_or(usize::MAX)
}

pub unsafe extern "win64" fn HeapValidate(_h: HANDLE, _f: u32, p: *const c_void) -> BOOL {
    if p.is_null() {
        TRUE
    } else {
        heap::is_ours(p as *mut c_void) as BOOL
    }
}

pub unsafe extern "win64" fn HeapCompact(_h: HANDLE, _f: u32) -> usize {
    0x10_0000
}

pub unsafe extern "win64" fn HeapLock(_h: HANDLE) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn HeapUnlock(_h: HANDLE) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn HeapWalk(_h: HANDLE, _e: *mut c_void) -> BOOL {
    stub!("HeapWalk unsupported");
    crate::teb::set_last_error(err::ERROR_NO_MORE_ITEMS);
    FALSE
}

pub unsafe extern "win64" fn HeapSetInformation(_h: HANDLE, _c: u32, _i: *mut c_void, _l: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn HeapQueryInformation(_h: HANDLE, class: u32, info: *mut c_void, len: usize, ret: *mut usize) -> BOOL {
    if class == 0 {
        // HeapCompatibilityInformation: 2 = low-fragmentation heap
        if !ret.is_null() {
            *ret = 4;
        }
        if len < 4 || info.is_null() {
            crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        *(info as *mut u32) = 2;
        return TRUE;
    }
    crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
    FALSE
}

// Local*/Global*: the handle is the pointer itself (LMEM_MOVEABLE blocks
// are never actually moved).
const LMEM_ZEROINIT: u32 = 0x40;

pub unsafe extern "win64" fn LocalAlloc(flags: u32, size: usize) -> *mut c_void {
    heap::alloc(size.max(1), flags & LMEM_ZEROINIT != 0)
}
pub unsafe extern "win64" fn LocalFree(p: *mut c_void) -> *mut c_void {
    if heap::free(p) {
        std::ptr::null_mut()
    } else {
        p
    }
}
pub unsafe extern "win64" fn LocalReAlloc(p: *mut c_void, size: usize, flags: u32) -> *mut c_void {
    heap::realloc(p, size, flags & LMEM_ZEROINIT != 0, false)
}
pub unsafe extern "win64" fn LocalLock(p: *mut c_void) -> *mut c_void {
    p
}
pub unsafe extern "win64" fn LocalUnlock(_p: *mut c_void) -> BOOL {
    FALSE
}
pub unsafe extern "win64" fn LocalSize(p: *mut c_void) -> usize {
    heap::size(p).unwrap_or(0)
}
pub unsafe extern "win64" fn GlobalAlloc(flags: u32, size: usize) -> *mut c_void {
    LocalAlloc(flags, size)
}
pub unsafe extern "win64" fn GlobalFree(p: *mut c_void) -> *mut c_void {
    LocalFree(p)
}
pub unsafe extern "win64" fn GlobalReAlloc(p: *mut c_void, size: usize, flags: u32) -> *mut c_void {
    LocalReAlloc(p, size, flags)
}
pub unsafe extern "win64" fn GlobalLock(p: *mut c_void) -> *mut c_void {
    p
}
pub unsafe extern "win64" fn GlobalUnlock(_p: *mut c_void) -> BOOL {
    FALSE
}
pub unsafe extern "win64" fn GlobalSize(p: *mut c_void) -> usize {
    heap::size(p).unwrap_or(0)
}
pub unsafe extern "win64" fn GlobalHandle(p: *const c_void) -> *mut c_void {
    p as *mut c_void
}

pub unsafe extern "win64" fn IsBadWritePtr(lp: *mut c_void, ucb: usize) -> BOOL {
    (lp.is_null() && ucb > 0) as BOOL
}

pub unsafe extern "win64" fn IsBadReadPtr(lp: *const c_void, ucb: usize) -> BOOL {
    (lp.is_null() && ucb > 0) as BOOL
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reserve_commit_decommit() {
        unsafe {
            let p = virtual_alloc(0, 0x40000, MEM_RESERVE, PAGE_NOACCESS).unwrap();
            assert_eq!(p % GRANULARITY, 0);
            let q = query(p).unwrap();
            assert_eq!(q.state, MEM_RESERVE);
            assert_eq!(q.region_size, 0x40000);
            virtual_alloc(p + 0x1000, 0x2000, MEM_COMMIT, PAGE_READWRITE).unwrap();
            *((p + 0x1000) as *mut u32) = 0xabcd;
            let q = query(p + 0x1000).unwrap();
            assert_eq!((q.state, q.protect, q.region_size, q.allocation_base), (MEM_COMMIT, PAGE_READWRITE, 0x2000, p));
            virtual_free(p + 0x1000, 0x1000, MEM_DECOMMIT).unwrap();
            virtual_alloc(p + 0x1000, 0x1000, MEM_COMMIT, PAGE_READWRITE).unwrap();
            assert_eq!(*((p + 0x1000) as *mut u32), 0);
            virtual_free(p, 0, MEM_RELEASE).unwrap();
            assert!(virtual_free(p, 0, MEM_RELEASE).is_err());
        }
    }

    #[test]
    fn decommit_unaligned_releases_inner_host_pages_and_preserves_neighbors() {
        unsafe {
            let p = virtual_alloc(0, 0x20000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE).unwrap();
            // Fill 128 KiB with non-zero bytes
            std::ptr::write_bytes(p as *mut u8, 0x5A, 0x20000);
            // Decommit an unaligned 64 KiB slice [p + 4 KiB, p + 68 KiB), spanning 3 complete
            // 16 KiB inner host pages plus partial 4 KiB head/tail sub-pages.
            virtual_free(p + 0x1000, 0x10000, MEM_DECOMMIT).unwrap();
            // Neighboring live bytes before and after must remain untouched.
            assert_eq!(*(p as *const u8), 0x5A);
            assert_eq!(*((p + 0x0FFF) as *const u8), 0x5A);
            assert_eq!(*((p + 0x11000) as *const u8), 0x5A);
            assert_eq!(*((p + 0x1FFFF) as *const u8), 0x5A);
            // Recommit the decommitted range and verify every byte reads zero.
            virtual_alloc(p + 0x1000, 0x10000, MEM_COMMIT, PAGE_READWRITE).unwrap();
            let slice = std::slice::from_raw_parts((p + 0x1000) as *const u8, 0x10000);
            assert!(slice.iter().all(|&b| b == 0));
            // Neighbors still intact after recommit.
            assert_eq!(*(p as *const u8), 0x5A);
            assert_eq!(*((p + 0x11000) as *const u8), 0x5A);
            virtual_free(p, 0, MEM_RELEASE).unwrap();
        }
    }
}
