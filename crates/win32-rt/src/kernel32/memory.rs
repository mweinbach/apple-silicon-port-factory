//! Virtual memory (VirtualAlloc & co.), heaps (HeapAlloc & co.), Local/Global.
//!
//! VirtualAlloc reservations are tracked with per-range state so that
//! VirtualQuery/VirtualFree behave like Windows (64K allocation granularity,
//! decommitted pages read as zero on recommit).

use crate::heap;
use crate::types::*;
use std::collections::{BTreeMap, BTreeSet};
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
pub const WRITE_WATCH_FLAG_RESET: u32 = 0x01;
const LARGE_PAGE_SIZE: usize = 2 * 1024 * 1024;

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

#[inline]
fn is_writable_prot(prot: u32) -> bool {
    matches!(
        prot & 0xFF,
        PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY
    )
}

#[inline]
fn validate_page_prot(prot: u32) -> Result<(), u32> {
    let base = prot & 0xFF;
    // Per Microsoft Learn (Memory Protection Constants): PAGE_GUARD cannot be
    // combined with PAGE_NOACCESS (or an empty base protection).
    if (prot & PAGE_GUARD) != 0 && (base == 0 || base == PAGE_NOACCESS) {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    Ok(())
}

pub fn win_prot_to_host(p: u32) -> i32 {
    if p & PAGE_GUARD != 0 {
        return libc::PROT_NONE;
    }
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
    write_watch: bool,
}

struct VmState {
    allocs: BTreeMap<usize, Alloc>,
    segs: BTreeMap<usize, Seg>,
    /// Host-page-aligned addresses written inside `MEM_WRITE_WATCH` regions since last reset.
    write_watch_dirty: BTreeSet<usize>,
    /// Host-page-aligned addresses that were resident (`MINCORE_INCORE`) when `MEM_RESET` was applied.
    reset_resident: BTreeSet<usize>,
}

static VM: Mutex<VmState> = Mutex::new(VmState {
    allocs: BTreeMap::new(),
    segs: BTreeMap::new(),
    write_watch_dirty: BTreeSet::new(),
    reset_resident: BTreeSet::new(),
});
/// Set once any committed page in `VM` is marked `PAGE_NOACCESS` or `PAGE_GUARD`
/// (e.g. packed executables using demand-decryption). Lets speculative JIT code
/// readers avoid taking `VM.lock()` on normal games and multi-threaded `.aot` prewarm.
static HAS_NOACCESS_PAGES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Set once any `MEM_WRITE_WATCH` allocation is created so write-fault handlers can
/// skip taking `VM.lock()` when write-watch is unused.
static HAS_WRITE_WATCH_PAGES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

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
    /// Returns `true` if any 4 KiB subpage in `[hp, hp + HOST_PAGE)` is `MEM_COMMIT`.
    fn has_committed_subpage(&self, hp: usize) -> bool {
        let mut sub = hp;
        while sub < hp + HOST_PAGE {
            if let Some((&_s, seg)) = self.segs.range(..=sub).next_back() {
                if sub < seg.end && seg.state == MEM_COMMIT {
                    return true;
                }
            }
            sub += PAGE;
        }
        false
    }
    /// Clear write-watch / reset tracking for host pages in `[start, end)` that no longer
    /// contain any committed 4 KiB subpage (so decommitting a single 4 KiB subpage never
    /// discards tracking for surviving committed siblings in the same 16 KiB host page).
    fn clear_tracking_in(&mut self, start: usize, end: usize) {
        let hstart = start & !(HOST_PAGE - 1);
        let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
        let dirty: Vec<usize> = self
            .write_watch_dirty
            .range(hstart..hend)
            .copied()
            .filter(|&hp| !self.has_committed_subpage(hp))
            .collect();
        for hp in dirty {
            self.write_watch_dirty.remove(&hp);
        }
        let res: Vec<usize> = self
            .reset_resident
            .range(hstart..hend)
            .copied()
            .filter(|&hp| !self.has_committed_subpage(hp))
            .collect();
        for hp in res {
            self.reset_resident.remove(&hp);
        }
    }
    /// Recompute and apply host page protection for `hp` from its surviving committed
    /// 4 KiB subpages: keeps `PROT_NONE` when no subpages are committed or any sibling
    /// is `PAGE_GUARD`/`PAGE_NOACCESS`, and strips `PROT_WRITE` when a clean
    /// `MEM_WRITE_WATCH` writable subpage needs write trapping armed.
    unsafe fn restore_host_page_prot(&self, hp: usize) {
        if self.has_protected_subpage(hp, None) {
            let _ = host_mprotect(hp, HOST_PAGE, libc::PROT_NONE);
            return;
        }
        let mut has_committed = false;
        let mut host_prot = 0i32;
        let mut arm_write_watch = false;
        let is_dirty = self.write_watch_dirty.contains(&hp);
        let mut sub = hp;
        while sub < hp + HOST_PAGE {
            if let Some((&_s, seg)) = self.segs.range(..=sub).next_back() {
                if sub < seg.end && seg.state == MEM_COMMIT {
                    has_committed = true;
                    host_prot |= win_prot_to_host(seg.prot);
                    if !is_dirty
                        && is_writable_prot(seg.prot)
                        && self.alloc_containing(sub).is_some_and(|(_, a)| a.write_watch)
                    {
                        arm_write_watch = true;
                    }
                }
            }
            sub += PAGE;
        }
        let final_prot = if !has_committed {
            libc::PROT_NONE
        } else if arm_write_watch {
            host_prot & !libc::PROT_WRITE
        } else {
            host_prot
        };
        let _ = host_mprotect(hp, HOST_PAGE, final_prot);
    }
    /// Returns `true` if any committed 4 KiB subpage in the enclosing `HOST_PAGE`
    /// outside `exclude_range` (if given) has `PAGE_NOACCESS` or `PAGE_GUARD`.
    fn has_protected_subpage(&self, hpage: usize, exclude_range: Option<(usize, usize)>) -> bool {
        if HOST_PAGE <= PAGE {
            return false;
        }
        let mut sub = hpage;
        while sub < hpage + HOST_PAGE {
            let excluded = exclude_range.is_some_and(|(s, e)| sub >= s && sub < e);
            if !excluded {
                if let Some((&_ss, sseg)) = self.segs.range(..=sub).next_back() {
                    if sub < sseg.end
                        && sseg.state == MEM_COMMIT
                        && ((sseg.prot & 0xFF) == PAGE_NOACCESS || (sseg.prot & PAGE_GUARD) != 0)
                    {
                        return true;
                    }
                }
            }
            sub += PAGE;
        }
        false
    }
    /// Rearm write-watch trapping on host page `hp` after clearing its dirty bit:
    /// inspects EVERY 4 KiB subpage in `[hp, hp + HOST_PAGE)` so a `PAGE_READONLY`
    /// first subpage never prevents stripping `PROT_WRITE` when a sibling subpage is
    /// writable, while preserving `PROT_NONE` if any subpage is `PAGE_GUARD`/`PAGE_NOACCESS`.
    unsafe fn rearm_write_watch_host_page(&self, hp: usize) {
        if self.has_protected_subpage(hp, None) {
            return;
        }
        let mut has_writable = false;
        let mut non_write_prot = 0i32;
        let mut sub = hp;
        while sub < hp + HOST_PAGE {
            if let Some((&_s, seg)) = self.segs.range(..=sub).next_back() {
                if sub < seg.end && seg.state == MEM_COMMIT {
                    if is_writable_prot(seg.prot) {
                        has_writable = true;
                    }
                    non_write_prot |= win_prot_to_host(seg.prot) & !libc::PROT_WRITE;
                }
            }
            sub += PAGE;
        }
        if has_writable {
            let _ = host_mprotect(hp, HOST_PAGE, non_write_prot);
        }
    }
}

/// Apply host protection for `[start, end)` inside `vm`, preserving `PROT_NONE` on
/// any enclosing 16 KiB host page that contains a `PAGE_GUARD` or `PAGE_NOACCESS`
/// sibling subpage outside `[start, end)`, and stripping `PROT_WRITE` on clean
/// (not-yet-written) host pages of `MEM_WRITE_WATCH` allocations so the first store
/// triggers `try_handle_write_watch_fault`.
unsafe fn apply_range_host_prot(vm: &VmState, start: usize, end: usize, prot: u32, write_watch: bool) -> i32 {
    let base_host = win_prot_to_host(prot);
    let wants_none = (prot & 0xFF) == PAGE_NOACCESS || (prot & PAGE_GUARD) != 0;
    if wants_none {
        return host_mprotect(start, end - start, libc::PROT_NONE);
    }
    let mut hp = start & !(HOST_PAGE - 1);
    let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    while hp < hend {
        let p = if vm.has_protected_subpage(hp, Some((start, end))) {
            libc::PROT_NONE
        } else if write_watch && is_writable_prot(prot) && !vm.write_watch_dirty.contains(&hp) {
            base_host & !libc::PROT_WRITE
        } else {
            base_host
        };
        let rc = host_mprotect(hp, HOST_PAGE, p);
        if rc != 0 {
            return rc;
        }
        hp += HOST_PAGE;
    }
    0
}

unsafe fn host_page_is_incore(hpage: usize) -> bool {
    let mut state: libc::c_char = 0;
    libc::mincore(hpage as *const c_void, HOST_PAGE, &mut state) == 0 && (state & 1) != 0
}

/// Record the enclosing host page of `addr` as dirty if it belongs to a
/// `MEM_WRITE_WATCH` allocation (used when a write is emulated on a 16 KiB host
/// page kept `PROT_NONE` for a `PAGE_GUARD`/`PAGE_NOACCESS` sibling).
pub fn mark_write_watch_dirty(addr: usize) {
    if !HAS_WRITE_WATCH_PAGES.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }
    let page = addr & !(PAGE - 1);
    let hpage = addr & !(HOST_PAGE - 1);
    let mut vm = VM.lock().unwrap();
    if vm.alloc_containing(page).is_some_and(|(_, a)| a.write_watch) {
        vm.write_watch_dirty.insert(hpage);
    }
}

/// Handle a hardware write fault on a clean `MEM_WRITE_WATCH` host page.
///
/// Called from the host `SIGBUS`/`SIGSEGV` handler before guest exception dispatch.
/// Returns `true` when `fault_addr` lies in a committed, writable, non-guard page of
/// a `MEM_WRITE_WATCH` allocation and no sibling 4 KiB subpage on the same 16 KiB host
/// page is `PAGE_GUARD` or `PAGE_NOACCESS`: records the host page as written, enables
/// `PROT_WRITE`, and lets the faulting instruction retry natively.
///
/// When a sibling 4 KiB subpage is `PAGE_GUARD` or `PAGE_NOACCESS`, returns `false`
/// without altering host protection so the fault handler emulates the single store
/// without exposing the protected sibling.
pub unsafe fn try_handle_write_watch_fault(fault_addr: usize, is_write: bool) -> bool {
    if !is_write || !HAS_WRITE_WATCH_PAGES.load(std::sync::atomic::Ordering::Acquire) {
        return false;
    }
    let page = fault_addr & !(PAGE - 1);
    let hpage = fault_addr & !(HOST_PAGE - 1);
    let mut vm = VM.lock().unwrap();
    let Some((_, alloc)) = vm.alloc_containing(page) else {
        return false;
    };
    if !alloc.write_watch {
        return false;
    }
    let Some((&_s, &seg)) = vm.segs.range(..=page).next_back() else {
        return false;
    };
    if page >= seg.end
        || seg.state != MEM_COMMIT
        || !is_writable_prot(seg.prot)
        || (seg.prot & PAGE_GUARD) != 0
    {
        return false;
    }
    if vm.has_protected_subpage(hpage, Some((page, page + PAGE))) {
        return false;
    }
    vm.write_watch_dirty.insert(hpage);
    host_mprotect(hpage, HOST_PAGE, win_prot_to_host(seg.prot)) == 0
}

/// Outcome of consuming a one-shot `PAGE_GUARD` bit on a faulting 4 KiB page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardConsumeResult {
    /// Logical protection before `PAGE_GUARD` was cleared.
    pub old_prot: u32,
    /// `true` when the enclosing 16 KiB host page had to remain `PROT_NONE` because
    /// another 4 KiB sibling subpage in the same host page still has `PAGE_GUARD` or
    /// `PAGE_NOACCESS` (callers must emulate the single faulting access rather than
    /// exposing sibling subpages).
    pub host_kept_none: bool,
}

/// Atomically consume a one-shot `PAGE_GUARD` bit on the faulting 4 KiB page and
/// restore host protection if no sibling 4 KiB subpage in the enclosing host page
/// still carries `PAGE_GUARD` or `PAGE_NOACCESS`.
///
/// Mirrors Windows clearing `PAGE_GUARD` before delivering `STATUS_GUARD_PAGE_VIOLATION`,
/// while preserving remaining 4 KiB sibling protections on 16 KiB Apple Silicon host pages.
pub unsafe fn consume_guard_page(fault_addr: usize) -> Option<GuardConsumeResult> {
    let page = fault_addr & !(PAGE - 1);
    let hpage = fault_addr & !(HOST_PAGE - 1);
    let mut vm = VM.lock().unwrap();
    let write_watch = vm.alloc_containing(page).is_some_and(|(_, a)| a.write_watch);
    let (&_s, &seg) = vm.segs.range(..=page).next_back()?;
    if page >= seg.end || seg.state != MEM_COMMIT || (seg.prot & PAGE_GUARD) == 0 {
        return None;
    }
    let old_prot = seg.prot;
    let new_prot = old_prot & !PAGE_GUARD;
    vm.set_range(page, page + PAGE, MEM_COMMIT, new_prot);

    let sibling_protected = vm.has_protected_subpage(hpage, Some((page, page + PAGE)));

    if !sibling_protected {
        let host_p = if write_watch && is_writable_prot(new_prot) && !vm.write_watch_dirty.contains(&hpage) {
            win_prot_to_host(new_prot) & !libc::PROT_WRITE
        } else {
            win_prot_to_host(new_prot)
        };
        let _ = host_mprotect(page, PAGE, host_p);
    }
    note_hpage_prot_change(page, page + PAGE);
    Some(GuardConsumeResult {
        old_prot,
        host_kept_none: sibling_protected,
    })
}

/// Register a PE image (or other host-managed range) so VirtualQuery
/// reports it as MEM_IMAGE with the given per-range protections.
pub fn register_image(base: usize, size: usize, ranges: &[(usize, usize, u32)]) {
    let mut vm = VM.lock().unwrap();
    vm.allocs.insert(base, Alloc { size, prot: PAGE_EXECUTE_WRITECOPY, kind: MEM_IMAGE, write_watch: false });
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

/// Returns `true` strictly if `addr` lies inside a committed, executable guest-owned
/// private or mapped virtual memory allocation (`VirtualAlloc` / `MapViewOfFile`),
/// excluding `MEM_IMAGE` (synthetic builtins are handled separately from real guest PEs),
/// uncommitted `MEM_RESERVE`, non-executable pages, `PAGE_NOACCESS`, and `PAGE_GUARD`.
pub fn is_guest_exec_vm_addr(addr: usize) -> bool {
    if addr == 0 {
        return false;
    }
    let page = addr & !(PAGE - 1);
    let vm = VM.lock().unwrap();
    let Some((_, alloc)) = vm.alloc_containing(page) else {
        return false;
    };
    if alloc.kind != MEM_PRIVATE && alloc.kind != MEM_MAPPED {
        return false;
    }
    let Some((&_s, seg)) = vm.segs.range(..=page).next_back() else {
        return false;
    };
    page < seg.end
        && seg.state == MEM_COMMIT
        && is_exec(seg.prot)
        && (seg.prot & 0xFF) != PAGE_NOACCESS
        && (seg.prot & PAGE_GUARD) == 0
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
        vm.allocs.insert(base, Alloc { size, prot, kind: MEM_MAPPED, write_watch: false });
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
    validate_page_prot(prot)?;
    if ty & MEM_LARGE_PAGES != 0 {
        // Per Microsoft Learn (VirtualAlloc / Large-Page Support):
        // - MEM_LARGE_PAGES requires MEM_RESERVE | MEM_COMMIT, cannot combine with
        //   MEM_WRITE_WATCH, PAGE_GUARD, or PAGE_NOACCESS, and requires 2 MiB
        //   multiple size and address alignment.
        // - Even with valid parameters, Windows fails with ERROR_PRIVILEGE_NOT_HELD
        //   when the process token lacks SeLockMemoryPrivilege (and Apple Silicon
        //   XNU does not expose 2 MiB user superpages).
        if (ty & (MEM_RESERVE | MEM_COMMIT)) != (MEM_RESERVE | MEM_COMMIT)
            || (ty & MEM_WRITE_WATCH) != 0
            || (prot & PAGE_GUARD) != 0
            || (prot & 0xFF) == PAGE_NOACCESS
            || (size % LARGE_PAGE_SIZE) != 0
            || (addr != 0 && (addr % LARGE_PAGE_SIZE) != 0)
        {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        return Err(err::ERROR_PRIVILEGE_NOT_HELD);
    }
    if ty & MEM_WRITE_WATCH != 0 && (ty & MEM_RESERVE) == 0 {
        // Per Microsoft Learn (VirtualAlloc): MEM_WRITE_WATCH must be specified with MEM_RESERVE.
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    if (prot & PAGE_GUARD) != 0 && (ty & MEM_COMMIT) == 0 && (ty & (MEM_RESET | MEM_RESET_UNDO)) == 0 {
        // Guard pages only apply to committed pages.
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    if ty & MEM_RESET != 0 {
        if addr == 0 {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let start = addr & !(PAGE - 1);
        let end = (addr.saturating_add(size) + PAGE - 1) & !(PAGE - 1);
        let mut vm = VM.lock().unwrap();
        let Some((b, a)) = vm.alloc_containing(start) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        if end > b + a.size {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        // Record which complete host pages are currently resident before marking them
        // MADV_FREE_REUSABLE so MEM_RESET_UNDO can verify whether XNU purged any of them.
        let mut hp = (addr + HOST_PAGE - 1) & !(HOST_PAGE - 1);
        let hend = addr.saturating_add(size) & !(HOST_PAGE - 1);
        while hp < hend {
            if host_page_is_incore(hp) {
                vm.reset_resident.insert(hp);
            }
            hp += HOST_PAGE;
        }
        discard_host_pages(addr, size);
        return Ok(addr);
    }
    if ty & MEM_RESET_UNDO != 0 {
        if addr == 0 {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let start = addr & !(PAGE - 1);
        let end = (addr.saturating_add(size) + PAGE - 1) & !(PAGE - 1);
        let mut vm = VM.lock().unwrap();
        let Some((b, a)) = vm.alloc_containing(start) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        if end > b + a.size {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        const MADV_FREE_REUSE: i32 = 8;
        let hstart = (addr + HOST_PAGE - 1) & !(HOST_PAGE - 1);
        let hend = addr.saturating_add(size) & !(HOST_PAGE - 1);
        let mut any_discarded = false;
        if hend > hstart {
            // Cancel MADV_FREE_REUSABLE in XNU so resident pages remain intact.
            let _ = libc::madvise(hstart as *mut c_void, hend - hstart, MADV_FREE_REUSE);
            let tracked: Vec<usize> = vm.reset_resident.range(hstart..hend).copied().collect();
            for hp in tracked {
                vm.reset_resident.remove(&hp);
                if !host_page_is_incore(hp) {
                    any_discarded = true;
                }
            }
        }
        if any_discarded {
            // Per Microsoft Learn (VirtualAlloc MEM_RESET_UNDO): fail when any page in the
            // range was discarded by the OS while in MEM_RESET state.
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        return Ok(addr);
    }
    let write_watch = (ty & MEM_WRITE_WATCH) != 0;
    if write_watch {
        HAS_WRITE_WATCH_PAGES.store(true, std::sync::atomic::Ordering::Release);
    }
    let mut vm = VM.lock().unwrap();
    if ty & MEM_RESERVE != 0 && ty & MEM_REPLACE_PLACEHOLDER != 0 {
        // Replace an existing placeholder at `addr` of `size`.
        if addr == 0 {
            return Err(err::ERROR_INVALID_ADDRESS);
        }
        let sz = (size + PAGE - 1) & !(PAGE - 1);
        vm.allocs.insert(addr, Alloc { size: sz, prot, kind: MEM_PRIVATE, write_watch });
        vm.set_range(addr, addr + sz, MEM_RESERVE, 0);
        if ty & MEM_COMMIT != 0 {
            if apply_range_host_prot(&vm, addr, addr + sz, prot, write_watch) != 0 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            vm.set_range(addr, addr + sz, MEM_COMMIT, prot);
            if (prot & PAGE_GUARD) != 0 || (prot & 0xFF) == PAGE_NOACCESS {
                note_hpage_prot_change(addr, addr + sz);
            }
        }
        return Ok(addr);
    }
    if ty & (MEM_RESERVE | MEM_RESERVE_PLACEHOLDER) != 0 {
        let a = addr & !(GRANULARITY - 1);
        let sz = ((addr + size + PAGE - 1) & !(PAGE - 1)) - if addr != 0 { a } else { 0 };
        let base = reserve_aligned(a, sz, align).ok_or(if addr != 0 { err::ERROR_INVALID_ADDRESS } else { err::ERROR_NOT_ENOUGH_MEMORY })?;
        vm.allocs.insert(base, Alloc { size: sz, prot, kind: MEM_PRIVATE, write_watch });
        vm.set_range(base, base + sz, MEM_RESERVE, 0);
        if ty & MEM_COMMIT != 0 {
            if apply_range_host_prot(&vm, base, base + sz, prot, write_watch) != 0 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            vm.set_range(base, base + sz, MEM_COMMIT, prot);
            if (prot & PAGE_GUARD) != 0 || (prot & 0xFF) == PAGE_NOACCESS {
                note_hpage_prot_change(base, base + sz);
            }
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
        if apply_range_host_prot(&vm, start, end, prot, a.write_watch) != 0 {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        vm.set_range(start, end, MEM_COMMIT, prot);
        if (prot & PAGE_GUARD) != 0 || (prot & 0xFF) == PAGE_NOACCESS {
            note_hpage_prot_change(start, end);
        }
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
            vm.clear_tracking_in(b, b + a.size);
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
        vm.clear_tracking_in(addr, end);
        return Ok(());
    }
    if ty & MEM_DECOMMIT != 0 {
        let Some((b, a)) = vm.alloc_containing(addr) else {
            return Err(err::ERROR_INVALID_ADDRESS);
        };
        let start = addr & !(PAGE - 1);
        let end = if size == 0 { b + a.size } else { (addr + size + PAGE - 1) & !(PAGE - 1) };
        let mut touched_partial_hpages = [0usize; 2];
        let mut touched_partial_count = 0usize;
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
                    touched_partial_hpages[touched_partial_count] = start & !(HOST_PAGE - 1);
                    touched_partial_count += 1;
                }
                if inner_end < end {
                    let _ = host_mprotect(inner_end, end - inner_end, libc::PROT_READ | libc::PROT_WRITE);
                    std::ptr::write_bytes(inner_end as *mut u8, 0, end - inner_end);
                    touched_partial_hpages[touched_partial_count] = inner_end & !(HOST_PAGE - 1);
                    touched_partial_count += 1;
                }
            } else {
                // Entire range lies inside one or two partial 16 KiB host pages.
                let _ = host_mprotect(start, end - start, libc::PROT_READ | libc::PROT_WRITE);
                std::ptr::write_bytes(start as *mut u8, 0, end - start);
                let h0 = start & !(HOST_PAGE - 1);
                let h1 = (end - 1) & !(HOST_PAGE - 1);
                touched_partial_hpages[touched_partial_count] = h0;
                touched_partial_count += 1;
                if h1 != h0 {
                    touched_partial_hpages[touched_partial_count] = h1;
                    touched_partial_count += 1;
                }
            }
        }
        vm.set_range(start, end, MEM_RESERVE, 0);
        vm.clear_tracking_in(start, end);
        for &hp in &touched_partial_hpages[..touched_partial_count] {
            vm.restore_host_page_prot(hp);
            note_hpage_prot_change(hp, hp + HOST_PAGE);
        }
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
    if is_self_process_handle(proc_) {
        return VirtualAlloc(addr, size, ty, prot);
    }
    if crate::handles::get_as::<crate::kernel32::process::ChildProcessObj>(proc_).is_some() {
        // Cross-process VM allocation requires Mach `task_for_pid` (blocked by macOS SIP/sandbox
        // without debugger entitlements). Matches Windows when the handle lacks PROCESS_VM_OPERATION.
        crate::teb::set_last_error(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn VirtualAlloc2(proc_: HANDLE, addr: *mut c_void, size: usize, ty: u32, prot: u32, ext: *mut c_void, next: u32) -> *mut c_void {
    // Per Microsoft Learn (VirtualAlloc2): a NULL process handle targets the current process.
    if !proc_.is_null() && !is_self_process_handle(proc_) {
        if crate::handles::get_as::<crate::kernel32::process::ChildProcessObj>(proc_).is_some() {
            // Cross-process VM allocation is unsupported on macOS/iOS without debugger entitlements;
            // matches Windows when the handle lacks PROCESS_VM_OPERATION.
            crate::teb::set_last_error(err::ERROR_ACCESS_DENIED);
        } else {
            crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        }
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
    flags: u32,
    base: *mut c_void,
    region_size: usize,
    addresses: *mut *mut c_void,
    count: *mut usize,
    granularity: *mut u32,
) -> u32 {
    if (flags & !WRITE_WATCH_FLAG_RESET) != 0
        || base.is_null()
        || region_size == 0
        || addresses.is_null()
        || count.is_null()
        || granularity.is_null()
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    }
    let start = base as usize;
    let Some(end) = start.checked_add(region_size) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    };
    let hstart = start & !(HOST_PAGE - 1);
    let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    // Read caller capacity BEFORE acquiring `VM` so a fault on a clean `MEM_WRITE_WATCH`
    // or `PAGE_GUARD` output buffer cannot self-deadlock on `VM`.
    let cap = std::ptr::read_unaligned(count);
    let dirty: Vec<usize> = {
        let mut vm = VM.lock().unwrap();
        let Some((alloc_base, alloc)) = vm.alloc_containing(start & !(PAGE - 1)) else {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return u32::MAX;
        };
        if !alloc.write_watch || end > alloc_base + alloc.size {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return u32::MAX;
        }
        let dirty: Vec<usize> = vm.write_watch_dirty.range(hstart..hend).take(cap).copied().collect();
        if (flags & WRITE_WATCH_FLAG_RESET) != 0 {
            // Per Microsoft Learn (GetWriteWatch): when WRITE_WATCH_FLAG_RESET is set,
            // reset only the pages whose addresses were returned in `addresses`.
            for &hp in &dirty {
                vm.write_watch_dirty.remove(&hp);
                vm.rearm_write_watch_host_page(hp);
            }
        }
        dirty
    };
    // Write guest output buffers strictly AFTER releasing `VM` so output buffers residing
    // on clean `MEM_WRITE_WATCH` or `PAGE_GUARD` pages fault cleanly without deadlocking.
    for (i, &hp) in dirty.iter().enumerate() {
        std::ptr::write_unaligned(addresses.add(i), hp as *mut c_void);
    }
    std::ptr::write_unaligned(count, dirty.len());
    std::ptr::write_unaligned(granularity, HOST_PAGE as u32);
    0
}

pub unsafe extern "win64" fn ResetWriteWatch(base: *mut c_void, region_size: usize) -> u32 {
    if base.is_null() || region_size == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    }
    let start = base as usize;
    let Some(end) = start.checked_add(region_size) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    };
    let hstart = start & !(HOST_PAGE - 1);
    let hend = (end + HOST_PAGE - 1) & !(HOST_PAGE - 1);
    let mut vm = VM.lock().unwrap();
    let Some((alloc_base, alloc)) = vm.alloc_containing(start & !(PAGE - 1)) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    };
    if !alloc.write_watch || end > alloc_base + alloc.size {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return u32::MAX;
    }
    let dirty: Vec<usize> = vm.write_watch_dirty.range(hstart..hend).copied().collect();
    for hp in dirty {
        vm.write_watch_dirty.remove(&hp);
        vm.rearm_write_watch_host_page(hp);
    }
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
    if size == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if let Err(e) = validate_page_prot(prot) {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    let start = addr as usize & !(PAGE - 1);
    let end = (addr as usize + size + PAGE - 1) & !(PAGE - 1);
    let prev = query(start).map(|i| i.protect).unwrap_or(PAGE_READWRITE);
    if memdiag() && (prot | prev) & 0xF0 != 0 {
        memdiag_log(format_args!("VirtualProtect({addr:p}, {size:#x}, {prot:#x}) prev={prev:#x}"));
    }
    let mut deferred_noaccess_sibling = false;
    let mprotect_rc = {
        let mut vm = VM.lock().unwrap();
        let write_watch = vm.alloc_containing(start).is_some_and(|(_, a)| a.write_watch);
        if vm.alloc_containing(start).is_some() {
            vm.set_range(start, end, MEM_COMMIT, prot);
        }
        // On 16 KiB host pages (Apple Silicon), if a sub-host-page non-writable
        // VirtualProtect (e.g. PAGE_EXECUTE_READ after decrypting one 4 KiB page)
        // still shares its 16 KiB hardware page with another PAGE_NOACCESS / PAGE_GUARD
        // 4 KiB sub-page, keep the host page PROT_NONE until all sibling sub-pages are
        // unlocked so other threads cannot race into still-encrypted siblings.
        let is_writable = is_writable_prot(prot);
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
        if deferred_noaccess_sibling {
            host_mprotect(start, end - start, libc::PROT_NONE)
        } else {
            apply_range_host_prot(&vm, start, end, prot, write_watch)
        }
    };
    if mprotect_rc != 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_ADDRESS);
        return FALSE;
    }
    if !old.is_null() {
        std::ptr::write_unaligned(old, if prev == 0 { PAGE_NOACCESS } else { prev });
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
    // Issue a real process-wide IPI/serialization barrier across every active thread in
    // this task. On Darwin, `thread_suspend` invokes XNU `thread_wait`, delivering a
    // cross-CPU AST/IPI to any core running that thread and waiting until it leaves
    // userspace (full hardware memory barrier + pipeline serialization) before we
    // immediately resume it. Serializing callers with `FLUSH_LOCK` prevents mutual
    // suspension, and zero heap allocations occur between suspend and resume.
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    #[cfg(target_vendor = "apple")]
    {
        static FLUSH_LOCK: Mutex<()> = Mutex::new(());
        let _g = FLUSH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        extern "C" {
            static mach_task_self_: libc::mach_port_t;
            fn mach_thread_self() -> libc::mach_port_t;
            fn task_threads(
                target_task: libc::mach_port_t,
                act_list: *mut *mut libc::mach_port_t,
                act_list_cnt: *mut u32,
            ) -> i32;
            fn thread_suspend(target_act: libc::mach_port_t) -> i32;
            fn thread_resume(target_act: libc::mach_port_t) -> i32;
            fn mach_port_deallocate(task: libc::mach_port_t, name: libc::mach_port_t) -> i32;
            fn mach_vm_deallocate(task: libc::mach_port_t, address: u64, size: u64) -> i32;
        }
        let task = mach_task_self_;
        let me = mach_thread_self();
        let mut list: *mut libc::mach_port_t = std::ptr::null_mut();
        let mut count: u32 = 0;
        if task_threads(task, &mut list, &mut count) == 0 && !list.is_null() {
            for i in 0..count as usize {
                let port = *list.add(i);
                if port != me && thread_suspend(port) == 0 {
                    let _ = thread_resume(port);
                }
                let _ = mach_port_deallocate(task, port);
            }
            let bytes = (count as u64) * (std::mem::size_of::<libc::mach_port_t>() as u64);
            let _ = mach_vm_deallocate(task, list as u64, bytes);
        }
        let _ = mach_port_deallocate(task, me);
    }
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

pub unsafe extern "win64" fn PrefetchVirtualMemory(_p: HANDLE, _n: usize, _e: *mut c_void, _f: u32) -> BOOL {
    TRUE
}

/// Release only the complete host pages lying entirely inside `[addr, addr + size)`.
///
/// Windows discards at 4 KB granularity but Apple Silicon host pages are 16 KB, and the kernel
/// rounds an `madvise` range outward to whole host pages. Using `MADV_FREE_REUSABLE` (7 on
/// Darwin) immediately deducts reclaimed pages from XNU's `phys_footprint` ledger (critical
/// for staying under iOS Jetsam limits), falling back to `MADV_FREE` if rejected.
unsafe fn discard_host_pages(addr: usize, size: usize) {
    const MADV_FREE_REUSABLE: i32 = 7;
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
    let all = heap::all_heaps();
    if !out.is_null() && (n as usize) >= all.len() {
        for (i, &h) in all.iter().enumerate() {
            std::ptr::write_unaligned(out.add(i), h);
        }
    }
    all.len() as u32
}

pub unsafe extern "win64" fn HeapCreate(_opts: u32, _init: usize, _max: usize) -> HANDLE {
    heap::create_heap()
}

pub unsafe extern "win64" fn HeapDestroy(h: HANDLE) -> BOOL {
    if heap::destroy_heap(h) {
        TRUE
    } else {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

pub unsafe extern "win64" fn HeapAlloc(h: HANDLE, flags: u32, size: usize) -> *mut c_void {
    let p = heap::alloc_in(h, size, flags & heap::HEAP_ZERO_MEMORY != 0);
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

pub unsafe extern "win64" fn HeapWalk(h: HANDLE, entry: *mut c_void) -> BOOL {
    if entry.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    // PROCESS_HEAP_ENTRY (x64, 40 bytes):
    // +0 lpData(*mut c_void), +8 cbData(u32), +12 cbOverhead(u8),
    // +13 iRegionIndex(u8), +14 wFlags(u16; 0x0004 = PROCESS_HEAP_ENTRY_BUSY)
    let prev = std::ptr::read_unaligned(entry as *const *const c_void);
    match heap::walk_next(h, prev) {
        Some((ptr, sz)) => {
            let p = entry as *mut u8;
            std::ptr::write_bytes(p, 0, 40);
            std::ptr::write_unaligned(p as *mut *mut c_void, ptr);
            std::ptr::write_unaligned(p.add(8) as *mut u32, sz.min(u32::MAX as usize) as u32);
            std::ptr::write_unaligned(p.add(12), 16u8);
            std::ptr::write_unaligned(p.add(13), 0u8);
            std::ptr::write_unaligned(p.add(14) as *mut u16, 0x0004u16); // PROCESS_HEAP_ENTRY_BUSY
            TRUE
        }
        None => {
            crate::teb::set_last_error(err::ERROR_NO_MORE_ITEMS);
            FALSE
        }
    }
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

    #[test]
    fn custom_heap_walk_and_destroy_track_and_free_blocks() {
        unsafe {
            let h = HeapCreate(0, 0, 0);
            assert!(!h.is_null());
            let p1 = HeapAlloc(h, heap::HEAP_ZERO_MEMORY, 64);
            let p2 = HeapAlloc(h, heap::HEAP_ZERO_MEMORY, 128);
            assert!(!p1.is_null() && !p2.is_null());

            // Walk live entries on custom heap
            let mut entry = [0u8; 40];
            let mut seen = Vec::new();
            while HeapWalk(h, entry.as_mut_ptr() as *mut c_void) == TRUE {
                let ptr = (entry.as_ptr() as *const *mut c_void).read_unaligned();
                let sz = (entry.as_ptr().add(8) as *const u32).read_unaligned();
                let flags = (entry.as_ptr().add(14) as *const u16).read_unaligned();
                assert_eq!(flags, 0x0004);
                seen.push((ptr, sz));
            }
            assert_eq!(crate::teb::last_error(), err::ERROR_NO_MORE_ITEMS);
            assert_eq!(seen.len(), 2);
            assert!(seen.contains(&(p1, 64)));
            assert!(seen.contains(&(p2, 128)));

            // Destroying the heap frees outstanding blocks cleanly and rejects double-destroy.
            assert_eq!(HeapDestroy(h), TRUE);
            assert_eq!(HeapDestroy(h), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_HANDLE);
        }
    }

    #[test]
    fn mem_large_pages_matches_windows_without_privilege() {
        unsafe {
            // Invalid flag combination / misalignment -> ERROR_INVALID_PARAMETER.
            assert!(VirtualAlloc(std::ptr::null_mut(), 0x1000, MEM_RESERVE | MEM_COMMIT | MEM_LARGE_PAGES, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert!(VirtualAlloc(std::ptr::null_mut(), LARGE_PAGE_SIZE, MEM_RESERVE | MEM_LARGE_PAGES, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Valid 2 MiB MEM_RESERVE | MEM_COMMIT | MEM_LARGE_PAGES without SeLockMemoryPrivilege -> ERROR_PRIVILEGE_NOT_HELD.
            assert!(VirtualAlloc(std::ptr::null_mut(), LARGE_PAGE_SIZE, MEM_RESERVE | MEM_COMMIT | MEM_LARGE_PAGES, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_PRIVILEGE_NOT_HELD);
        }
    }

    #[test]
    fn page_guard_validation_and_oneshot_consumption() {
        unsafe {
            // PAGE_GUARD | PAGE_NOACCESS is invalid per Windows docs.
            let p = VirtualAlloc(std::ptr::null_mut(), 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
            assert!(!p.is_null());
            let mut old = 0u32;
            assert_eq!(VirtualProtect(p, 0x1000, PAGE_NOACCESS | PAGE_GUARD, &mut old), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Setting PAGE_GUARD on a single 4 KiB subpage inside a 16 KiB host page clears
            // cleanly when no sibling subpage is guarded/noaccess.
            assert_eq!(VirtualProtect(p, PAGE, PAGE_READWRITE | PAGE_GUARD, &mut old), TRUE);
            assert_eq!(old, PAGE_READWRITE);
            let q = query(p as usize).unwrap();
            assert_eq!(q.protect, PAGE_READWRITE | PAGE_GUARD);

            let res = consume_guard_page(p as usize).expect("first consume");
            assert_eq!(res.old_prot, PAGE_READWRITE | PAGE_GUARD);
            assert!(!res.host_kept_none);
            assert_eq!(consume_guard_page(p as usize), None);
            let q2 = query(p as usize).unwrap();
            assert_eq!(q2.protect, PAGE_READWRITE);
            *(p as *mut u32) = 0x1234_5678;
            assert_eq!(*(p as *const u32), 0x1234_5678);

            // Guarding a full 16 KiB range and consuming only the first 4 KiB subpage preserves
            // sibling PAGE_GUARD protections (keeping the host page PROT_NONE until all siblings clear).
            assert_eq!(VirtualProtect(p, HOST_PAGE, PAGE_READWRITE | PAGE_GUARD, &mut old), TRUE);
            let first = consume_guard_page(p as usize).expect("consume subpage 0");
            assert_eq!(first.old_prot, PAGE_READWRITE | PAGE_GUARD);
            if HOST_PAGE > PAGE {
                assert!(first.host_kept_none, "host page must stay PROT_NONE while sibling subpages remain guarded");
                assert_eq!(query((p as usize) + PAGE).unwrap().protect, PAGE_READWRITE | PAGE_GUARD);
                for sub in 1..(HOST_PAGE / PAGE) {
                    let r = consume_guard_page((p as usize) + sub * PAGE).expect("consume sibling");
                    assert_eq!(r.host_kept_none, sub + 1 < HOST_PAGE / PAGE);
                }
            }
            // Once all siblings are consumed, native access is restored.
            *(p as *mut u32) = 0x5566_7788;
            assert_eq!(*(p as *const u32), 0x5566_7788);

            // Guarding page 0 next to a PAGE_NOACCESS sibling keeps the host page PROT_NONE
            // so the PAGE_NOACCESS sibling is never exposed.
            if HOST_PAGE > PAGE {
                assert_eq!(VirtualProtect((p as usize + PAGE) as *mut c_void, PAGE, PAGE_NOACCESS, &mut old), TRUE);
                assert_eq!(VirtualProtect(p, PAGE, PAGE_READWRITE | PAGE_GUARD, &mut old), TRUE);
                let r = consume_guard_page(p as usize).expect("consume next to PAGE_NOACCESS");
                assert!(r.host_kept_none, "PAGE_NOACCESS sibling must remain protected");
                assert_eq!(VirtualProtect((p as usize + PAGE) as *mut c_void, PAGE, PAGE_READWRITE, &mut old), TRUE);
            }

            // VirtualAlloc with MEM_COMMIT | PAGE_GUARD directly (single 4 KiB page).
            let p2 = VirtualAlloc(std::ptr::null_mut(), PAGE, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE | PAGE_GUARD);
            assert!(!p2.is_null());
            assert_eq!(query(p2 as usize).unwrap().protect, PAGE_READWRITE | PAGE_GUARD);
            let r2 = consume_guard_page(p2 as usize).expect("consume p2");
            assert!(!r2.host_kept_none);
            *(p2 as *mut u32) = 0xCAFE_BABE;
            assert_eq!(*(p2 as *const u32), 0xCAFE_BABE);

            assert_eq!(VirtualFree(p, 0, MEM_RELEASE), TRUE);
            assert_eq!(VirtualFree(p2, 0, MEM_RELEASE), TRUE);
        }
    }

    #[test]
    fn write_watch_tracks_resets_and_validates() {
        unsafe {
            // MEM_WRITE_WATCH without MEM_RESERVE fails with ERROR_INVALID_PARAMETER.
            assert!(VirtualAlloc(std::ptr::null_mut(), 0x10000, MEM_COMMIT | MEM_WRITE_WATCH, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            let region_sz = HOST_PAGE * 4;
            let base = VirtualAlloc(std::ptr::null_mut(), region_sz, MEM_RESERVE | MEM_COMMIT | MEM_WRITE_WATCH, PAGE_READWRITE);
            assert!(!base.is_null());

            // Initially 0 dirty pages.
            let mut addrs = [std::ptr::null_mut(); 8];
            let mut count = addrs.len();
            let mut gran = 0u32;
            assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
            assert_eq!(count, 0);
            assert_eq!(gran, HOST_PAGE as u32);

            // Simulate hardware write faults on host page 1 and host page 3.
            let p1 = (base as usize) + HOST_PAGE;
            let p3 = (base as usize) + HOST_PAGE * 3;
            assert!(try_handle_write_watch_fault(p1 + 16, true));
            *(p1 as *mut u32) = 0xAAAA_BBBB;
            assert!(try_handle_write_watch_fault(p3 + 64, true));
            *(p3 as *mut u32) = 0xCCCC_DDDD;

            // Non-reset query returns both pages in ascending order.
            count = addrs.len();
            assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
            assert_eq!(count, 2);
            assert_eq!(addrs[0] as usize, p1);
            assert_eq!(addrs[1] as usize, p3);

            // Partial reset (capacity 1) resets ONLY the first returned page (p1), leaving p3 dirty.
            count = 1;
            assert_eq!(GetWriteWatch(WRITE_WATCH_FLAG_RESET, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
            assert_eq!(count, 1);
            assert_eq!(addrs[0] as usize, p1);

            count = addrs.len();
            assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
            assert_eq!(count, 1);
            assert_eq!(addrs[0] as usize, p3);

            // ResetWriteWatch clears remaining dirty pages.
            assert_eq!(ResetWriteWatch(base, region_sz), 0);
            count = addrs.len();
            assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
            assert_eq!(count, 0);

            // On 16 KiB host pages, a PAGE_GUARD or PAGE_NOACCESS sibling subpage prevents
            // try_handle_write_watch_fault from lifting host protection on the whole host page;
            // emulated stores record dirtiness via mark_write_watch_dirty instead.
            if HOST_PAGE > PAGE {
                let mut old = 0u32;
                let p0 = base as usize;
                assert_eq!(VirtualProtect((p0 + PAGE) as *mut c_void, PAGE, PAGE_READWRITE | PAGE_GUARD, &mut old), TRUE);
                assert!(
                    !try_handle_write_watch_fault(p0 + 16, true),
                    "write-watch fast path must not unprotect a host page with a PAGE_GUARD sibling"
                );
                mark_write_watch_dirty(p0 + 16);
                count = addrs.len();
                assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
                assert_eq!(count, 1);
                assert_eq!(addrs[0] as usize, p0);
                // Clear sibling guard and verify normal reset.
                assert_eq!(VirtualProtect((p0 + PAGE) as *mut c_void, PAGE, PAGE_READWRITE, &mut old), TRUE);
                assert_eq!(ResetWriteWatch(base, region_sz), 0);
            }

            // On 16 KiB host pages, decommitting a single 4 KiB subpage must preserve
            // write-watch trapping on clean surviving siblings and preserve dirty state on
            // already-written surviving siblings until all subpages are decommitted or reset.
            if HOST_PAGE > PAGE {
                #[cfg(target_vendor = "apple")]
                unsafe fn query_host_vm_prot(addr: usize) -> i32 {
                    const VM_REGION_BASIC_INFO_64: i32 = 9;
                    const VM_REGION_BASIC_INFO_COUNT_64: u32 = 9;
                    extern "C" {
                        fn mach_task_self() -> u32;
                        fn mach_vm_region(
                            target_task: u32,
                            address: *mut u64,
                            size: *mut u64,
                            flavor: i32,
                            info: *mut i32,
                            info_cnt: *mut u32,
                            object_name: *mut u32,
                        ) -> i32;
                    }
                    let mut query_addr = addr as u64;
                    let mut query_size = 0u64;
                    let mut info = [0i32; 9];
                    let mut count = VM_REGION_BASIC_INFO_COUNT_64;
                    let mut obj = 0u32;
                    let kr = mach_vm_region(
                        mach_task_self(),
                        &mut query_addr,
                        &mut query_size,
                        VM_REGION_BASIC_INFO_64,
                        info.as_mut_ptr(),
                        &mut count,
                        &mut obj,
                    );
                    assert_eq!(kr, 0, "mach_vm_region failed: {kr}");
                    assert!(query_addr <= addr as u64 && (addr as u64) < query_addr + query_size);
                    info[0]
                }

                // 1. Clean host page: decommit subpage 0 -> host page must rearm read-only (no VM_PROT_WRITE).
                let p2 = (base as usize) + HOST_PAGE * 2;
                assert_eq!(VirtualFree(p2 as *mut c_void, PAGE, MEM_DECOMMIT), TRUE);
                #[cfg(target_vendor = "apple")]
                assert_eq!(
                    query_host_vm_prot(p2) & libc::PROT_WRITE,
                    0,
                    "clean write-watch host page must keep PROT_WRITE stripped after sibling subpage decommit"
                );
                // Subsequent write fault on surviving sibling subpage 1 is still tracked.
                assert!(try_handle_write_watch_fault(p2 + PAGE + 8, true));
                *((p2 + PAGE) as *mut u32) = 0x1357_2468;
                count = addrs.len();
                assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
                assert_eq!(count, 1);
                assert_eq!(addrs[0] as usize, p2);

                // 2. Dirty host page: decommit sibling subpage 2 -> dirty record for surviving subpage 1 stays intact.
                assert_eq!(VirtualFree((p2 + PAGE * 2) as *mut c_void, PAGE, MEM_DECOMMIT), TRUE);
                count = addrs.len();
                assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
                assert_eq!(count, 1);
                assert_eq!(addrs[0] as usize, p2);

                // ResetWriteWatch clears dirty state and rearms write trapping on the remaining committed subpages.
                assert_eq!(ResetWriteWatch(base, region_sz), 0);
                #[cfg(target_vendor = "apple")]
                assert_eq!(query_host_vm_prot(p2) & libc::PROT_WRITE, 0);

                // 3. Decommit remaining committed subpages -> host page drops to PROT_NONE and tracking clears.
                assert!(try_handle_write_watch_fault(p2 + PAGE + 16, true));
                assert_eq!(VirtualFree((p2 + PAGE) as *mut c_void, PAGE, MEM_DECOMMIT), TRUE);
                assert_eq!(VirtualFree((p2 + PAGE * 3) as *mut c_void, PAGE, MEM_DECOMMIT), TRUE);
                count = addrs.len();
                assert_eq!(GetWriteWatch(0, base, region_sz, addrs.as_mut_ptr(), &mut count, &mut gran), 0);
                assert_eq!(count, 0);
                #[cfg(target_vendor = "apple")]
                assert_eq!(query_host_vm_prot(p2), 0);
            }

            // Non-write-watch region rejected with u32::MAX / ERROR_INVALID_PARAMETER.
            let plain = VirtualAlloc(std::ptr::null_mut(), HOST_PAGE, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
            assert!(!plain.is_null());
            assert_eq!(GetWriteWatch(0, plain, HOST_PAGE, addrs.as_mut_ptr(), &mut count, &mut gran), u32::MAX);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(ResetWriteWatch(plain, HOST_PAGE), u32::MAX);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(VirtualFree(base, 0, MEM_RELEASE), TRUE);
            assert_eq!(VirtualFree(plain, 0, MEM_RELEASE), TRUE);
        }
    }

    #[test]
    fn mem_reset_and_undo_and_virtual_alloc_ex_and_flush_buffers() {
        unsafe {
            // MEM_RESET + MEM_RESET_UNDO preserves resident pages and validates addresses.
            let sz = HOST_PAGE * 2;
            let p = VirtualAlloc(std::ptr::null_mut(), sz, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
            assert!(!p.is_null());
            *(p as *mut u64) = 0x1122_3344_5566_7788;
            assert_eq!(VirtualAlloc(p, sz, MEM_RESET, PAGE_READWRITE), p);
            let undo = VirtualAlloc(p, sz, MEM_RESET_UNDO, PAGE_READWRITE);
            if !undo.is_null() {
                assert_eq!(undo, p);
                assert_eq!(*(p as *const u64), 0x1122_3344_5566_7788);
            }
            assert!(VirtualAlloc(std::ptr::null_mut(), sz, MEM_RESET_UNDO, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_ADDRESS);
            assert_eq!(VirtualFree(p, 0, MEM_RELEASE), TRUE);

            // VirtualAllocEx: CURRENT_PROCESS and OpenProcess(self) succeed; NULL/bogus -> ERROR_INVALID_HANDLE.
            let p_ex = VirtualAllocEx(crate::handles::CURRENT_PROCESS, std::ptr::null_mut(), 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
            assert!(!p_ex.is_null());
            assert_eq!(VirtualFree(p_ex, 0, MEM_RELEASE), TRUE);

            let self_h = crate::kernel32::process::OpenProcess(0x1FFFFF, FALSE, std::process::id());
            assert!(!self_h.is_null());
            let p_ex2 = VirtualAllocEx(self_h, std::ptr::null_mut(), 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
            assert!(!p_ex2.is_null());
            assert_eq!(VirtualFree(p_ex2, 0, MEM_RELEASE), TRUE);
            assert_eq!(crate::kernel32::threads::CloseHandle(self_h), TRUE);

            assert!(VirtualAllocEx(std::ptr::null_mut(), std::ptr::null_mut(), 0x1000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_HANDLE);

            // VirtualAlloc2: NULL process handle targets current process per Windows docs.
            let p2 = VirtualAlloc2(std::ptr::null_mut(), std::ptr::null_mut(), 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE, std::ptr::null_mut(), 0);
            assert!(!p2.is_null());
            assert_eq!(VirtualFree(p2, 0, MEM_RELEASE), TRUE);
            assert!(VirtualAlloc2(0xDEADusize as HANDLE, std::ptr::null_mut(), 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE, std::ptr::null_mut(), 0).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_HANDLE);

            // FlushProcessWriteBuffers completes cleanly under multi-threaded load.
            let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let s2 = std::sync::Arc::clone(&stop);
            let worker = std::thread::spawn(move || {
                while !s2.load(std::sync::atomic::Ordering::Relaxed) {
                    std::hint::spin_loop();
                }
            });
            for _ in 0..8 {
                FlushProcessWriteBuffers();
            }
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            worker.join().unwrap();
        }
    }
}
