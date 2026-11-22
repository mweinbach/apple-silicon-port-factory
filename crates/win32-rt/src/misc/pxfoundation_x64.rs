//! `pxfoundation_x64.dll` — native Rust replacement for NVIDIA PhysX 3.4.2 Foundation (`PxFoundation_x64.dll`).
//!
//! Implements all 39 exports (`PxCreateFoundation`, `PxGetFoundation`, plus 37 C++-mangled
//! exports), including the `PxFoundation` singleton and its 16-byte-aligned
//! `PxAllocatorCallback`, `SyncImpl`, `SListImpl`, `ThreadImpl`, `TempAllocator`,
//! `shdfnd::snprintf`, `PxShortestRotation`, and `PxDiagonalize`.

use crate::{Export, Ptr};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicPtr, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, Once, OnceLock};

pub static ORDINALS: &[(u16, &str)] = &[];

static REGISTER_VTABLES: Once = Once::new();
static ASSERT_HANDLER: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static FOUNDATION_REFCOUNT: AtomicU32 = AtomicU32::new(0);
static FOUNDATION_PTR: AtomicPtr<PxFoundationInstance> = AtomicPtr::new(std::ptr::null_mut());

#[repr(C)]
pub struct PxAllocatorBridge {
    pub vtbl: *const [Ptr; 3],
}

#[repr(C)]
pub struct PxErrorBridge {
    pub vtbl: *const [Ptr; 2],
}

#[repr(C)]
pub struct PxFoundationInstance {
    pub vtbl: *const [Ptr; 8],               // +0x00
    pub error_callback: *mut c_void,         // +0x08
    pub user_allocator: *mut c_void,         // +0x10
    pub allocator_bridge: PxAllocatorBridge, // +0x18
    pub error_bridge: PxErrorBridge,         // +0x20
    pub error_level: u32,                    // +0x28
    pub report_alloc_names: u8,              // +0x2c
    pub _pad: [u8; 0x1b0],                   // pad past +0x1d8 as in original Foundation
}

unsafe extern "win64" fn allocator_dtor(
    this: *mut PxAllocatorBridge,
    _flags: u32,
) -> *mut PxAllocatorBridge {
    this
}

pub unsafe extern "win64" fn allocator_allocate(
    _this: *mut PxAllocatorBridge,
    size: usize,
    _type_name: *const u8,
    _filename: *const u8,
    _line: i32,
) -> *mut c_void {
    let f = get_or_init_foundation();
    let user_alloc = std::ptr::read_unaligned(std::ptr::addr_of!((*f).user_allocator));
    if !user_alloc.is_null() {
        let vtbl = std::ptr::read_unaligned(user_alloc as *const usize);
        if vtbl != 0 {
            let alloc_fn = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
            if alloc_fn != 0 && alloc_fn != allocator_allocate as *const () as usize {
                let res = crate::call_guest8(
                    alloc_fn,
                    user_alloc as u64,
                    size as u64,
                    _type_name as u64,
                    _filename as u64,
                    _line as u32 as u64,
                    0,
                    0,
                    0,
                ) as *mut c_void;
                if !res.is_null() {
                    std::ptr::write_bytes(res as *mut u8, 0, size);
                    return res;
                }
            }
        }
    }
    let alloc_size = size.max(16);
    let mut out: *mut libc::c_void = std::ptr::null_mut();
    if libc::posix_memalign(&mut out, 16, alloc_size) != 0 || out.is_null() {
        return std::ptr::null_mut();
    }
    std::ptr::write_bytes(out as *mut u8, 0, alloc_size);
    out as *mut c_void
}

pub unsafe extern "win64" fn allocator_deallocate(_this: *mut PxAllocatorBridge, ptr: *mut c_void) {
    if !ptr.is_null() {
        let f = get_or_init_foundation();
        let user_alloc = std::ptr::read_unaligned(std::ptr::addr_of!((*f).user_allocator));
        if !user_alloc.is_null() {
            let vtbl = std::ptr::read_unaligned(user_alloc as *const usize);
            if vtbl != 0 {
                let dealloc_fn = std::ptr::read_unaligned((vtbl + 0x10) as *const usize);
                if dealloc_fn != 0 && dealloc_fn != allocator_deallocate as *const () as usize {
                    crate::call_guest8(
                        dealloc_fn,
                        user_alloc as u64,
                        ptr as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    );
                    return;
                }
            }
        }
        libc::free(ptr as *mut libc::c_void);
    }
}

static ALLOCATOR_VTBL: [Ptr; 3] = [
    Ptr(allocator_dtor as *const ()),       // 0 (+0x00): ~PxAllocatorCallback
    Ptr(allocator_allocate as *const ()),   // 1 (+0x08): allocate
    Ptr(allocator_deallocate as *const ()), // 2 (+0x10): deallocate
];

unsafe extern "win64" fn error_dtor(this: *mut PxErrorBridge, _flags: u32) -> *mut PxErrorBridge {
    this
}

unsafe extern "win64" fn error_report(
    _this: *mut PxErrorBridge,
    code: u32,
    message: *const u8,
    file: *const u8,
    line: i32,
) {
    let msg = crate::strings::from_ansi_opt(message).unwrap_or_default();
    let f = crate::strings::from_ansi_opt(file).unwrap_or_default();
    log::warn!("physx error({code}) {f}:{line}: {msg}");
}

static ERROR_VTBL: [Ptr; 2] = [
    Ptr(error_dtor as *const ()),   // 0 (+0x00): ~PxErrorCallback
    Ptr(error_report as *const ()), // 1 (+0x08): reportError
];

// ---------------------------------------------------------------------------
// PxFoundation vtable (8 slots, matches Include/foundation/PxFoundation.h)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn foundation_release(_this: *mut PxFoundationInstance) {
    // Keep singleton alive for late static destructors unless refcount hits 0.
}

unsafe extern "win64" fn foundation_get_error_callback(
    this: *mut PxFoundationInstance,
) -> *mut c_void {
    let f = if this.is_null() {
        get_or_init_foundation()
    } else {
        this
    };
    let cb = std::ptr::read_unaligned(std::ptr::addr_of!((*f).error_callback));
    if !cb.is_null() {
        cb
    } else {
        std::ptr::addr_of_mut!((*f).error_bridge) as *mut c_void
    }
}

unsafe extern "win64" fn foundation_set_error_level(this: *mut PxFoundationInstance, mask: u32) {
    if !this.is_null() {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).error_level), mask);
    }
}

unsafe extern "win64" fn foundation_get_error_level(this: *const PxFoundationInstance) -> u32 {
    if this.is_null() {
        0xffffffff
    } else {
        std::ptr::read_unaligned(std::ptr::addr_of!((*this).error_level))
    }
}

unsafe extern "win64" fn foundation_get_allocator_callback(
    this: *mut PxFoundationInstance,
) -> *mut PxAllocatorBridge {
    let f = if this.is_null() {
        get_or_init_foundation()
    } else {
        this
    };
    std::ptr::addr_of_mut!((*f).allocator_bridge)
}

unsafe extern "win64" fn foundation_get_report_alloc_names(
    this: *const PxFoundationInstance,
) -> u8 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(std::ptr::addr_of!((*this).report_alloc_names))
    }
}

unsafe extern "win64" fn foundation_set_report_alloc_names(
    this: *mut PxFoundationInstance,
    val: u8,
) {
    if !this.is_null() {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).report_alloc_names), val);
    }
}

unsafe extern "win64" fn foundation_dtor(
    this: *mut PxFoundationInstance,
    _flags: u32,
) -> *mut PxFoundationInstance {
    this
}

static FOUNDATION_VTBL: [Ptr; 8] = [
    Ptr(foundation_release as *const ()),                // 0 (+0x00): release
    Ptr(foundation_get_error_callback as *const ()),     // 1 (+0x08): getErrorCallback
    Ptr(foundation_set_error_level as *const ()),        // 2 (+0x10): setErrorLevel
    Ptr(foundation_get_error_level as *const ()),        // 3 (+0x18): getErrorLevel
    Ptr(foundation_get_allocator_callback as *const ()), // 4 (+0x20): getAllocatorCallback
    Ptr(foundation_get_report_alloc_names as *const ()), // 5 (+0x28): getReportAllocationNames
    Ptr(foundation_set_report_alloc_names as *const ()), // 6 (+0x30): setReportAllocationNames
    Ptr(foundation_dtor as *const ()),                   // 7 (+0x38): ~PxFoundation
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in FOUNDATION_VTBL
            .iter()
            .chain(ALLOCATOR_VTBL.iter())
            .chain(ERROR_VTBL.iter())
        {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

fn get_or_init_foundation() -> *mut PxFoundationInstance {
    ensure_vtables_registered();
    let cur = FOUNDATION_PTR.load(Ordering::Acquire);
    if !cur.is_null() {
        return cur;
    }
    let inst = Box::new(PxFoundationInstance {
        vtbl: &FOUNDATION_VTBL,
        error_callback: std::ptr::null_mut(),
        user_allocator: std::ptr::null_mut(),
        allocator_bridge: PxAllocatorBridge {
            vtbl: &ALLOCATOR_VTBL,
        },
        error_bridge: PxErrorBridge { vtbl: &ERROR_VTBL },
        error_level: 0xffffffff,
        report_alloc_names: 0,
        _pad: [0u8; 0x1b0],
    });
    let raw = Box::into_raw(inst);
    match FOUNDATION_PTR.compare_exchange(
        std::ptr::null_mut(),
        raw,
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        Ok(_) => raw,
        Err(existing) => {
            unsafe {
                let _ = Box::from_raw(raw);
            }
            existing
        }
    }
}

// ---------------------------------------------------------------------------
// Exported C functions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn PxCreateFoundation(
    _version: u32,
    allocator: *mut c_void,
    error_callback: *mut c_void,
) -> *mut PxFoundationInstance {
    let f = get_or_init_foundation();
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*f).user_allocator), allocator);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*f).error_callback), error_callback);
    FOUNDATION_REFCOUNT.store(1, Ordering::Relaxed);
    f
}

pub unsafe extern "win64" fn PxGetFoundation() -> *mut PxFoundationInstance {
    get_or_init_foundation()
}

// ---------------------------------------------------------------------------
// Exported C++-mangled functions
// ---------------------------------------------------------------------------

unsafe extern "win64" fn atomic_decrement(val: *const AtomicI32) -> i32 {
    if val.is_null() {
        return 0;
    }
    if (val as usize) & 3 == 0 {
        (*val).fetch_sub(1, Ordering::SeqCst) - 1
    } else {
        let p = val as *mut i32;
        let next = std::ptr::read_unaligned(p).wrapping_sub(1);
        std::ptr::write_unaligned(p, next);
        next
    }
}

unsafe extern "win64" fn px_set_assert_handler(handler: *mut c_void) {
    ASSERT_HANDLER.store(handler, Ordering::Relaxed);
}

unsafe extern "win64" fn get_allocator() -> *mut PxAllocatorBridge {
    let f = get_or_init_foundation();
    std::ptr::addr_of_mut!((*f).allocator_bridge)
}

unsafe extern "win64" fn foundation_get_instance() -> *mut PxFoundationInstance {
    get_or_init_foundation()
}

unsafe extern "win64" fn foundation_inc_ref_count() {
    FOUNDATION_REFCOUNT.fetch_add(1, Ordering::Relaxed);
}

unsafe extern "win64" fn foundation_dec_ref_count() {
    let _ = FOUNDATION_REFCOUNT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
        Some(v.saturating_sub(1))
    });
}

unsafe extern "win64" fn foundation_error_impl(
    this: *mut PxFoundationInstance,
    code: u32,
    file: *const u8,
    line: i32,
    message_fmt: *const u8,
    va: *const u8,
) {
    let mut buf = [0u8; 512];
    if !message_fmt.is_null() {
        let _ = crate::ucrt::printf::__stdio_common_vsprintf(
            crate::ucrt::printf::STANDARD_SNPRINTF_BEHAVIOR,
            buf.as_mut_ptr(),
            buf.len(),
            message_fmt,
            std::ptr::null_mut(),
            va,
        );
    }
    let f = if this.is_null() {
        get_or_init_foundation()
    } else {
        this
    };
    let mask = std::ptr::read_unaligned(std::ptr::addr_of!((*f).error_level));
    if (code & mask) == 0 && code != 0 {
        return;
    }
    let cb = std::ptr::read_unaligned(std::ptr::addr_of!((*f).error_callback));
    if !cb.is_null() {
        let vtbl = std::ptr::read_unaligned(cb as *const usize);
        if vtbl != 0 {
            let report_fn = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
            if report_fn != 0 && report_fn != error_report as *const () as usize {
                crate::call_guest8(
                    report_fn,
                    cb as u64,
                    code as u64,
                    buf.as_ptr() as u64,
                    file as u64,
                    line as u32 as u64,
                    0,
                    0,
                    0,
                );
                return;
            }
        }
    }
    let msg = crate::strings::from_ansi(buf.as_ptr());
    let file_s = crate::strings::from_ansi_opt(file).unwrap_or_default();
    log::warn!("pxfoundation!error({code}) {file_s}:{line}: {msg}");
}

unsafe extern "win64" fn foundation_error_va(args: *const u64) {
    let this = std::ptr::read_unaligned(args) as usize as *mut PxFoundationInstance;
    let code = std::ptr::read_unaligned(args.add(1)) as u32;
    let file = std::ptr::read_unaligned(args.add(2)) as usize as *const u8;
    let line = std::ptr::read_unaligned(args.add(3)) as i32;
    let fmt = std::ptr::read_unaligned(args.add(4)) as usize as *const u8;
    let va = args.add(5) as *const u8;
    foundation_error_impl(this, code, file, line, fmt, va);
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "win64" fn foundation_error() {
    std::arch::naked_asm!(
        "mov [rsp+8], rcx",
        "mov [rsp+16], rdx",
        "mov [rsp+24], r8",
        "mov [rsp+32], r9",
        "sub rsp, 40",
        "lea rcx, [rsp+48]",
        "call {imp}",
        "add rsp, 40",
        "ret",
        imp = sym foundation_error_va,
    )
}

#[cfg(not(target_arch = "x86_64"))]
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn foundation_error(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) {
    let args = [
        a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15,
    ];
    foundation_error_va(args.as_ptr());
}

/// Variadic implementation of `physx::shdfnd::snprintf(char* dst, size_t dstSize, const char* fmt, ...)`.
///
/// Uses Win64 contiguous 8-byte argument slots (matching `PsString.cpp`'s C99 `snprintf`
/// contract: always NUL-terminates when `dst != NULL && dstSize > 0`, and returns the full
/// length that would have been written excluding the NUL terminator).
unsafe extern "win64" fn shdfnd_snprintf_va(args: *const u64) -> i32 {
    let buf = std::ptr::read_unaligned(args) as usize as *mut u8;
    let count = std::ptr::read_unaligned(args.add(1)) as usize;
    let fmt = std::ptr::read_unaligned(args.add(2)) as usize as *const u8;
    if fmt.is_null() {
        if !buf.is_null() && count > 0 {
            std::ptr::write_unaligned(buf, 0);
        }
        return 0;
    }
    let va = args.add(3) as *const u8;
    crate::ucrt::printf::__stdio_common_vsprintf(
        crate::ucrt::printf::STANDARD_SNPRINTF_BEHAVIOR,
        buf,
        count,
        fmt,
        std::ptr::null_mut(),
        va,
    )
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "win64" fn shdfnd_snprintf() {
    std::arch::naked_asm!(
        "mov [rsp+8], rcx",
        "mov [rsp+16], rdx",
        "mov [rsp+24], r8",
        "mov [rsp+32], r9",
        "sub rsp, 40",
        "lea rcx, [rsp+48]",
        "call {imp}",
        "add rsp, 40",
        "ret",
        imp = sym shdfnd_snprintf_va,
    )
}

#[cfg(not(target_arch = "x86_64"))]
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn shdfnd_snprintf(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) -> i32 {
    let args = [
        a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15,
    ];
    shdfnd_snprintf_va(args.as_ptr())
}

unsafe extern "win64" fn shdfnd_stricmp(a: *const u8, b: *const u8) -> i32 {
    if a == b {
        return 0;
    }
    if a.is_null() {
        return -1;
    }
    if b.is_null() {
        return 1;
    }
    libc::strcasecmp(a as *const libc::c_char, b as *const libc::c_char)
}

// ---------------------------------------------------------------------------
// SyncImpl (8-byte wrapper around a Win32 manual-reset event HANDLE)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn sync_ctor(this: *mut *mut c_void) -> *mut *mut c_void {
    if !this.is_null() {
        let h = crate::kernel32::threads::CreateEventA(
            std::ptr::null_mut(),
            1,
            0,
            std::ptr::null(),
        );
        std::ptr::write_unaligned(this, h);
    }
    this
}

unsafe extern "win64" fn sync_dtor(this: *mut *mut c_void) {
    if !this.is_null() {
        let h = std::ptr::read_unaligned(this);
        if !h.is_null() {
            crate::kernel32::threads::CloseHandle(h);
            std::ptr::write_unaligned(this, std::ptr::null_mut());
        }
    }
}

unsafe extern "win64" fn sync_set(this: *mut *mut c_void) {
    if !this.is_null() {
        let h = std::ptr::read_unaligned(this);
        if !h.is_null() {
            crate::kernel32::threads::SetEvent(h);
        }
    }
}

unsafe extern "win64" fn sync_reset(this: *mut *mut c_void) {
    if !this.is_null() {
        let h = std::ptr::read_unaligned(this);
        if !h.is_null() {
            crate::kernel32::threads::ResetEvent(h);
        }
    }
}

unsafe extern "win64" fn sync_wait(this: *mut *mut c_void, ms: u32) -> u8 {
    if this.is_null() {
        return 1;
    }
    let h = std::ptr::read_unaligned(this);
    if h.is_null() {
        return 1;
    }
    (crate::kernel32::threads::WaitForSingleObject(h, ms) == 0) as u8
}

unsafe extern "win64" fn sync_get_size() -> u32 {
    8
}

// ---------------------------------------------------------------------------
// SListImpl (16-byte wrapper around Windows SLIST_HEADER)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn slist_ctor(
    this: *mut crate::sync::SListHeader,
) -> *mut crate::sync::SListHeader {
    if !this.is_null() {
        crate::sync::InitializeSListHead(this);
    }
    this
}

unsafe extern "win64" fn slist_dtor(_this: *mut crate::sync::SListHeader) {}

unsafe extern "win64" fn slist_push(
    this: *mut crate::sync::SListHeader,
    entry: *mut *mut c_void,
) {
    if !this.is_null() && !entry.is_null() {
        crate::sync::InterlockedPushEntrySList(this, entry);
    }
}

unsafe extern "win64" fn slist_pop(this: *mut crate::sync::SListHeader) -> *mut c_void {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        crate::sync::InterlockedPopEntrySList(this)
    }
}

unsafe extern "win64" fn slist_get_size() -> u32 {
    16
}

// ---------------------------------------------------------------------------
// ThreadImpl (0x30-byte structure, matches PsWindowsThread.cpp::_ThreadImpl)
// ---------------------------------------------------------------------------

const THREAD_NOT_STARTED: u32 = 0;
const THREAD_STARTED: u32 = 1;
const THREAD_STOPPED: u32 = 2;

#[repr(C)]
pub struct ThreadImpl {
    pub handle: usize,       // +0x00 (Win32 HANDLE)
    pub quit_now: AtomicI32, // +0x08 (LONG quitNow)
    pub state: u32,          // +0x0c (0 = NotStarted, 1 = Started, 2 = Stopped)
    pub thread_id: u32,      // +0x10 (DWORD threadID)
    pub _pad: u32,           // +0x14
    pub fn_ptr: usize,       // +0x18 (ThreadImpl::ExecuteFn fn)
    pub runnable: usize,     // +0x20 (void* arg / Runnable*)
    pub affinity_mask: u32,  // +0x28 (uint32_t affinityMask)
    pub _pad2: u32,          // +0x2c
}
const _: () = assert!(std::mem::size_of::<ThreadImpl>() == 0x30);

/// Tracks whether a spawned `ThreadImpl` is still live so a worker thread exiting
/// after `~ThreadImpl` never writes into freed guest memory.
fn live_threads() -> &'static Mutex<HashMap<usize, Arc<Mutex<bool>>>> {
    static MAP: OnceLock<Mutex<HashMap<usize, Arc<Mutex<bool>>>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe extern "win64" fn thread_ctor(this: *mut ThreadImpl) -> *mut ThreadImpl {
    if !this.is_null() {
        std::ptr::write_bytes(this as *mut u8, 0, 0x30);
    }
    this
}

unsafe extern "win64" fn thread_dtor(this: *mut ThreadImpl) {
    if this.is_null() {
        return;
    }
    thread_signal_quit(this);
    let h = std::ptr::read_unaligned(std::ptr::addr_of!((*this).handle));
    if h != 0 {
        if let Some(alive) = live_threads().lock().unwrap().remove(&h) {
            *alive.lock().unwrap() = false;
        }
        crate::kernel32::threads::CloseHandle(h as *mut c_void);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).handle), 0);
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).state), THREAD_STOPPED);
}

unsafe extern "win64" fn thread_start(this: *mut ThreadImpl, stack_size: u32, runnable: usize) {
    if this.is_null() {
        return;
    }
    if std::ptr::read_unaligned(std::ptr::addr_of!((*this).state)) != THREAD_NOT_STARTED {
        return;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).state), THREAD_STARTED);

    let fn_ptr = std::ptr::read_unaligned(std::ptr::addr_of!((*this).fn_ptr));
    let mut arg = std::ptr::read_unaligned(std::ptr::addr_of!((*this).runnable));
    if runnable != 0 && arg == 0 && fn_ptr == 0 {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).runnable), runnable);
        arg = runnable;
    }

    let size = if stack_size == 0 {
        crate::thread::DEFAULT_STACK_SIZE.load(Ordering::Relaxed)
    } else {
        stack_size as usize
    };
    let alive = Arc::new(Mutex::new(true));
    let alive_worker = Arc::clone(&alive);
    let this_addr = this as usize;

    let body = Box::new(move || {
        unsafe {
            if fn_ptr != 0 {
                crate::call_guest8(fn_ptr, arg as u64, 0, 0, 0, 0, 0, 0, 0);
            } else if arg != 0 {
                // physx::shdfnd::Runnable vtable: slot 0 = ~Runnable(), slot 1 (+0x08) = execute()
                let vtbl = std::ptr::read_unaligned(arg as *const usize);
                if vtbl != 0 {
                    let exec_fn = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
                    if exec_fn != 0 {
                        crate::call_guest8(exec_fn, arg as u64, 0, 0, 0, 0, 0, 0, 0);
                    }
                }
            }
            let guard = alive_worker.lock().unwrap();
            if *guard {
                let impl_ptr = this_addr as *mut ThreadImpl;
                std::ptr::write_unaligned(
                    std::ptr::addr_of_mut!((*impl_ptr).state),
                    THREAD_STOPPED,
                );
            }
        }
        0u32
    });

    match crate::thread::spawn(body, size, true, false) {
        Ok(t) => {
            let tid = t.tid.load(Ordering::Relaxed) as u32;
            let h = crate::handles::insert(t.clone());
            live_threads().lock().unwrap().insert(h as usize, alive);
            std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).handle), h as usize);
            std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).thread_id), tid);
            t.resume();
        }
        Err(e) => {
            log::warn!("pxfoundation!ThreadImpl::start failed to spawn thread: {e}");
            std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).state), THREAD_NOT_STARTED);
        }
    }
}

unsafe extern "win64" fn thread_signal_quit(this: *mut ThreadImpl) {
    if this.is_null() {
        return;
    }
    let quit_ptr = std::ptr::addr_of!((*this).quit_now);
    if (quit_ptr as usize) & 3 == 0 {
        (*quit_ptr).fetch_add(1, Ordering::SeqCst);
    } else {
        let p = quit_ptr as *mut i32;
        let next = std::ptr::read_unaligned(p).wrapping_add(1);
        std::ptr::write_unaligned(p, next);
    }
}

unsafe extern "win64" fn thread_wait_for_quit(this: *mut ThreadImpl) -> u8 {
    if this.is_null() {
        return 0;
    }
    if std::ptr::read_unaligned(std::ptr::addr_of!((*this).state)) == THREAD_NOT_STARTED {
        return 0;
    }
    let h = std::ptr::read_unaligned(std::ptr::addr_of!((*this).handle));
    if h != 0 {
        crate::kernel32::threads::WaitForSingleObject(h as *mut c_void, crate::types::INFINITE);
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).state), THREAD_STOPPED);
    1
}

unsafe extern "win64" fn thread_quit_is_signalled(this: *mut ThreadImpl) -> u8 {
    if this.is_null() {
        return 1;
    }
    let quit_ptr = std::ptr::addr_of!((*this).quit_now);
    let v = if (quit_ptr as usize) & 3 == 0 {
        (*quit_ptr).load(Ordering::SeqCst)
    } else {
        std::ptr::read_unaligned(quit_ptr as *const i32)
    };
    (v != 0) as u8
}

unsafe extern "win64" fn thread_quit(this: *mut ThreadImpl) {
    if this.is_null() {
        return;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).state), THREAD_STOPPED);
    let tid = std::ptr::read_unaligned(std::ptr::addr_of!((*this).thread_id));
    if tid != 0
        && crate::thread::current_thread_object().is_some()
        && crate::kernel32::threads::GetCurrentThreadId() == tid
    {
        crate::thread::exit_current_thread(0);
    }
}

unsafe extern "win64" fn thread_set_affinity_mask(this: *mut ThreadImpl, mask: u32) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = std::ptr::read_unaligned(std::ptr::addr_of!((*this).affinity_mask));
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).affinity_mask), mask);
    prev
}

unsafe extern "win64" fn thread_set_name(this: *mut ThreadImpl, name: *const u8) {
    if this.is_null() || name.is_null() {
        return;
    }
    let h = std::ptr::read_unaligned(std::ptr::addr_of!((*this).handle));
    if h != 0 {
        if let Some(s) = crate::strings::from_ansi_opt(name) {
            if let Some(t) = crate::handles::get_as::<crate::thread::ThreadObj>(h as *mut c_void) {
                *t.name.lock().unwrap() = s;
            }
        }
    }
}

unsafe extern "win64" fn thread_get_default_stack_size() -> u32 {
    0x10_0000
}

unsafe extern "win64" fn thread_get_id() -> u64 {
    crate::kernel32::threads::GetCurrentThreadId() as u64
}

unsafe extern "win64" fn thread_get_size() -> u32 {
    0x30
}

// ---------------------------------------------------------------------------
// TempAllocator
// ---------------------------------------------------------------------------

unsafe extern "win64" fn temp_allocator_allocate(
    _this: *mut c_void,
    size: usize,
    filename: *const u8,
    line: i32,
) -> *mut c_void {
    if size == 0 {
        return std::ptr::null_mut();
    }
    allocator_allocate(
        std::ptr::null_mut(),
        size,
        b"TempAlloc\0".as_ptr(),
        filename,
        line,
    )
}

unsafe extern "win64" fn temp_allocator_deallocate(_this: *mut c_void, ptr: *mut c_void) {
    allocator_deallocate(std::ptr::null_mut(), ptr);
}

// ---------------------------------------------------------------------------
// Math utilities: PxShortestRotation & PxDiagonalize
// ---------------------------------------------------------------------------

pub fn shortest_rotation_quat(v0: [f32; 3], v1: [f32; 3]) -> [f32; 4] {
    let d = v0[0] * v1[0] + v0[1] * v1[1] + v0[2] * v1[2];
    let cross = [
        v0[1] * v1[2] - v0[2] * v1[1],
        v0[2] * v1[0] - v0[0] * v1[2],
        v0[0] * v1[1] - v0[1] * v1[0],
    ];
    if d > -1.0 {
        let s = ((1.0 + d) * 2.0).sqrt();
        if s > 1e-6 {
            let r = 1.0 / s;
            return [cross[0] * r, cross[1] * r, cross[2] * r, 0.5 * s];
        }
    }
    if v0[0].abs() < v0[1].abs() {
        let len = (v0[1] * v0[1] + v0[2] * v0[2]).sqrt().max(1e-6);
        [0.0, -v0[2] / len, v0[1] / len, 0.0]
    } else {
        let len = (v0[0] * v0[0] + v0[2] * v0[2]).sqrt().max(1e-6);
        [v0[2] / len, 0.0, -v0[0] / len, 0.0]
    }
}

unsafe extern "win64" fn px_shortest_rotation(
    sret: *mut [f32; 4],
    v0: *const [f32; 3],
    v1: *const [f32; 3],
) -> *mut [f32; 4] {
    if !sret.is_null() {
        let a = if v0.is_null() {
            [1.0, 0.0, 0.0]
        } else {
            std::ptr::read_unaligned(v0)
        };
        let b = if v1.is_null() {
            [1.0, 0.0, 0.0]
        } else {
            std::ptr::read_unaligned(v1)
        };
        std::ptr::write_unaligned(sret, shortest_rotation_quat(a, b));
    }
    sret
}

#[inline]
fn indexed_rotation(axis: usize, s: f32, c: f32) -> [f32; 4] {
    let mut v = [0.0f32; 3];
    v[axis] = s;
    [v[0], v[1], v[2], c]
}

#[inline]
fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[3] * b[0] + b[3] * a[0] + a[1] * b[2] - b[1] * a[2],
        a[3] * b[1] + b[3] * a[1] + a[2] * b[0] - b[2] * a[0],
        a[3] * b[2] + b[3] * a[2] + a[0] * b[1] - b[0] * a[1],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

#[inline]
fn quat_normalize(q: [f32; 4]) -> [f32; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len > 0.0 {
        let inv = 1.0 / len;
        [q[0] * inv, q[1] * inv, q[2] * inv, q[3] * inv]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

/// Convert unit quaternion `[x, y, z, w]` to column-major `PxMat33` (`mat[col][row]`).
#[inline]
fn mat33_from_quat(q: [f32; 4]) -> [[f32; 3]; 3] {
    let [x, y, z, w] = q;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x2 * x;
    let yy = y2 * y;
    let zz = z2 * z;
    let xy = x2 * y;
    let xz = x2 * z;
    let xw = x2 * w;
    let yz = y2 * z;
    let yw = y2 * w;
    let zw = z2 * w;
    [
        [1.0 - yy - zz, xy + zw, xz - yw],
        [xy - zw, 1.0 - xx - zz, yz + xw],
        [xz + yw, yz - xw, 1.0 - xx - yy],
    ]
}

#[inline]
fn mat33_transpose(m: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

#[inline]
fn mat33_mul(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let mut out = [[0.0f32; 3]; 3];
    for col in 0..3 {
        for row in 0..3 {
            out[col][row] =
                a[0][row] * b[col][0] + a[1][row] * b[col][1] + a[2][row] * b[col][2];
        }
    }
    out
}

/// Symmetric 3×3 Jacobi eigendecomposition matching PhysX 3.4 `PsMathUtils.cpp::PxDiagonalize`.
///
/// Given column-major symmetric `PxMat33` `m`, returns `(eigenvalues, mass_frame_quat)` such
/// that `R(q) * diag(eigenvalues) * R(q)^T == m`.
pub fn diagonalize_mat33(m_raw: [f32; 9]) -> ([f32; 3], [f32; 4]) {
    const MAX_ITERS: u32 = 24;
    let m = [
        [m_raw[0], m_raw[1], m_raw[2]],
        [m_raw[3], m_raw[4], m_raw[5]],
        [m_raw[6], m_raw[7], m_raw[8]],
    ];

    let mut q = [0.0f32, 0.0, 0.0, 1.0];
    for _ in 0..MAX_ITERS {
        let axes = mat33_from_quat(q);
        let d = mat33_mul(mat33_mul(mat33_transpose(axes), m), axes);

        let d0 = d[1][2].abs();
        let d1 = d[0][2].abs();
        let d2 = d[0][1].abs();
        let a = if d0 > d1 && d0 > d2 {
            0usize
        } else if d1 > d2 {
            1usize
        } else {
            2usize
        };

        let a1 = (a + 1) % 3;
        let a2 = (a1 + 1) % 3;
        let off = d[a1][a2];
        if off == 0.0 || (d[a1][a1] - d[a2][a2]).abs() > 2e6 * (2.0 * off).abs() {
            break;
        }

        let w = (d[a1][a1] - d[a2][a2]) / (2.0 * off);
        let absw = w.abs();

        let r = if absw > 1000.0 {
            indexed_rotation(a, 1.0 / (4.0 * w), 1.0)
        } else {
            let t = 1.0 / (absw + (w * w + 1.0).sqrt());
            let h = 1.0 / (t * t + 1.0).sqrt();
            let sign_w = if w >= 0.0 { 1.0 } else { -1.0 };
            indexed_rotation(
                a,
                ((1.0 - h) * 0.5).max(0.0).sqrt() * sign_w,
                ((1.0 + h) * 0.5).sqrt(),
            )
        };

        q = quat_normalize(quat_mul(q, r));
    }

    let axes = mat33_from_quat(q);
    let d = mat33_mul(mat33_mul(mat33_transpose(axes), m), axes);
    ([d[0][0], d[1][1], d[2][2]], q)
}

unsafe extern "win64" fn px_diagonalize(
    sret: *mut [f32; 3],
    mat: *const [f32; 9],
    axes: *mut [f32; 4],
) -> *mut [f32; 3] {
    let (evals, q) = if mat.is_null() {
        ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0])
    } else {
        diagonalize_mat33(std::ptr::read_unaligned(mat))
    };
    if !axes.is_null() {
        std::ptr::write_unaligned(axes, q);
    }
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, evals);
    }
    sret
}

pub static EXTRA_EXPORTS: &[Export] = &[
    Export {
        name: "?atomicDecrement@shdfnd@physx@@YAHPECH@Z",
        ptr: Ptr(atomic_decrement as *const ()),
    },
    Export {
        name: "?PxSetAssertHandler@physx@@YAXAEAVPxAssertHandler@1@@Z",
        ptr: Ptr(px_set_assert_handler as *const ()),
    },
    Export {
        name: "?PxShortestRotation@physx@@YA?AVPxQuat@1@AEBVPxVec3@1@0@Z",
        ptr: Ptr(px_shortest_rotation as *const ()),
    },
    Export {
        name: "?getAllocator@shdfnd@physx@@YAAEAVPxAllocatorCallback@2@XZ",
        ptr: Ptr(get_allocator as *const ()),
    },
    Export {
        name: "?getInstance@Foundation@shdfnd@physx@@SAAEAV123@XZ",
        ptr: Ptr(foundation_get_instance as *const ()),
    },
    Export {
        name: "?incRefCount@Foundation@shdfnd@physx@@SAXXZ",
        ptr: Ptr(foundation_inc_ref_count as *const ()),
    },
    Export {
        name: "?decRefCount@Foundation@shdfnd@physx@@SAXXZ",
        ptr: Ptr(foundation_dec_ref_count as *const ()),
    },
    Export {
        name: "?error@Foundation@shdfnd@physx@@QEAAXW4Enum@PxErrorCode@3@PEBDH1ZZ",
        ptr: Ptr(foundation_error as *const ()),
    },
    Export {
        name: "?snprintf@shdfnd@physx@@YAHPEAD_KPEBDZZ",
        ptr: Ptr(shdfnd_snprintf as *const ()),
    },
    Export {
        name: "?stricmp@shdfnd@physx@@YAHPEBD0@Z",
        ptr: Ptr(shdfnd_stricmp as *const ()),
    },
    Export {
        name: "??1SyncImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(sync_dtor as *const ()),
    },
    Export {
        name: "?set@SyncImpl@shdfnd@physx@@QEAAXXZ",
        ptr: Ptr(sync_set as *const ()),
    },
    Export {
        name: "?reset@SyncImpl@shdfnd@physx@@QEAAXXZ",
        ptr: Ptr(sync_reset as *const ()),
    },
    Export {
        name: "?getSize@SyncImpl@shdfnd@physx@@SAIXZ",
        ptr: Ptr(sync_get_size as *const ()),
    },
    Export {
        name: "??0SListImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(slist_ctor as *const ()),
    },
    Export {
        name: "??1SListImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(slist_dtor as *const ()),
    },
    Export {
        name: "?push@SListImpl@shdfnd@physx@@QEAAXPEAVSListEntry@23@@Z",
        ptr: Ptr(slist_push as *const ()),
    },
    Export {
        name: "?pop@SListImpl@shdfnd@physx@@QEAAPEAVSListEntry@23@XZ",
        ptr: Ptr(slist_pop as *const ()),
    },
    Export {
        name: "?getDefaultStackSize@ThreadImpl@shdfnd@physx@@SAIXZ",
        ptr: Ptr(thread_get_default_stack_size as *const ()),
    },
    Export {
        name: "?getId@ThreadImpl@shdfnd@physx@@SA_KXZ",
        ptr: Ptr(thread_get_id as *const ()),
    },
    Export {
        name: "?start@ThreadImpl@shdfnd@physx@@QEAAXIPEAVRunnable@23@@Z",
        ptr: Ptr(thread_start as *const ()),
    },
    Export {
        name: "?signalQuit@ThreadImpl@shdfnd@physx@@QEAAXXZ",
        ptr: Ptr(thread_signal_quit as *const ()),
    },
    Export {
        name: "?waitForQuit@ThreadImpl@shdfnd@physx@@QEAA_NXZ",
        ptr: Ptr(thread_wait_for_quit as *const ()),
    },
    Export {
        name: "?getSize@SListImpl@shdfnd@physx@@SAIXZ",
        ptr: Ptr(slist_get_size as *const ()),
    },
    Export {
        name: "?setAffinityMask@ThreadImpl@shdfnd@physx@@QEAAII@Z",
        ptr: Ptr(thread_set_affinity_mask as *const ()),
    },
    Export {
        name: "?setName@ThreadImpl@shdfnd@physx@@QEAAXPEBD@Z",
        ptr: Ptr(thread_set_name as *const ()),
    },
    Export {
        name: "?PxDiagonalize@physx@@YA?AVPxVec3@1@AEBVPxMat33@1@AEAVPxQuat@1@@Z",
        ptr: Ptr(px_diagonalize as *const ()),
    },
    Export {
        name: "??0ThreadImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(thread_ctor as *const ()),
    },
    Export {
        name: "??1ThreadImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(thread_dtor as *const ()),
    },
    Export {
        name: "?quitIsSignalled@ThreadImpl@shdfnd@physx@@QEAA_NXZ",
        ptr: Ptr(thread_quit_is_signalled as *const ()),
    },
    Export {
        name: "?quit@ThreadImpl@shdfnd@physx@@QEAAXXZ",
        ptr: Ptr(thread_quit as *const ()),
    },
    Export {
        name: "?getSize@ThreadImpl@shdfnd@physx@@SAIXZ",
        ptr: Ptr(thread_get_size as *const ()),
    },
    Export {
        name: "?wait@SyncImpl@shdfnd@physx@@QEAA_NI@Z",
        ptr: Ptr(sync_wait as *const ()),
    },
    Export {
        name: "?allocate@TempAllocator@shdfnd@physx@@QEAAPEAX_KPEBDH@Z",
        ptr: Ptr(temp_allocator_allocate as *const ()),
    },
    Export {
        name: "?deallocate@TempAllocator@shdfnd@physx@@QEAAXPEAX@Z",
        ptr: Ptr(temp_allocator_deallocate as *const ()),
    },
    Export {
        name: "?errorImpl@Foundation@shdfnd@physx@@QEAAXW4Enum@PxErrorCode@3@PEBDH1PEAD@Z",
        ptr: Ptr(foundation_error_impl as *const ()),
    },
    Export {
        name: "??0SyncImpl@shdfnd@physx@@QEAA@XZ",
        ptr: Ptr(sync_ctor as *const ()),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn px_foundation_and_utilities() {
        assert_eq!(EXTRA_EXPORTS.len(), 37);
        unsafe {
            let f = PxCreateFoundation(0x0100_0000, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!f.is_null());
            assert_eq!(PxGetFoundation(), f);

            let alloc = get_allocator();
            assert!(!alloc.is_null());
            let mem = allocator_allocate(alloc, 128, b"test\0".as_ptr(), b"file.cpp\0".as_ptr(), 1);
            assert!(!mem.is_null());
            assert_eq!((mem as usize) & 0xf, 0);
            allocator_deallocate(alloc, mem);

            let mut q = [0.0f32; 4];
            let v0 = [1.0f32, 0.0, 0.0];
            let v1 = [0.0f32, 1.0, 0.0];
            px_shortest_rotation(&mut q, &v0, &v1);
            assert!((q[2] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-4);
            assert!((q[3] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-4);
        }
    }

    fn verify_diagonalization(m_raw: [f32; 9]) {
        let mut evals = [0.0f32; 3];
        let mut q = [0.0f32; 4];
        unsafe {
            px_diagonalize(&mut evals, &m_raw, &mut q);
        }

        // Quaternion must be unit length.
        let q_len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
        assert!((q_len - 1.0).abs() < 1e-5, "non-unit quat: {q:?} len={q_len}");

        // R = mat33_from_quat(q) must be orthogonal: R^T * R == I.
        let r = mat33_from_quat(q);
        let rt = mat33_transpose(r);
        let prod = mat33_mul(rt, r);
        for c in 0..3 {
            for row in 0..3 {
                let expected = if c == row { 1.0 } else { 0.0 };
                assert!(
                    (prod[c][row] - expected).abs() < 1e-4,
                    "R^T*R[{c}][{row}] = {}, expected {expected}",
                    prod[c][row]
                );
            }
        }

        // Reconstruction: R * diag(evals) * R^T == M.
        let diag = [
            [evals[0], 0.0, 0.0],
            [0.0, evals[1], 0.0],
            [0.0, 0.0, evals[2]],
        ];
        let recon = mat33_mul(mat33_mul(r, diag), rt);
        for c in 0..3 {
            for row in 0..3 {
                let orig = m_raw[c * 3 + row];
                assert!(
                    (recon[c][row] - orig).abs() < 1e-3,
                    "reconstruction mismatch at [{c}][{row}]: got {}, expected {orig} (evals={evals:?}, q={q:?})",
                    recon[c][row]
                );
            }
        }
    }

    #[test]
    fn px_diagonalize_orthogonal_and_reconstructs() {
        // Already diagonal
        verify_diagonalization([5.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 9.0]);
        // Equal diagonal entries with off-diagonal coupling (w == 0 edge case)
        verify_diagonalization([2.0, 1.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 5.0]);
        // General dense symmetric inertia-like tensor
        verify_diagonalization([6.0, -2.0, 2.0, -2.0, 3.0, -1.0, 2.0, -1.0, 3.0]);
        // Symmetric matrix with negative eigenvalues and strong coupling
        verify_diagonalization([1.5, 2.3, -1.7, 2.3, -0.5, 3.1, -1.7, 3.1, 4.2]);
    }

    #[test]
    fn shdfnd_snprintf_formats_win64_varargs() {
        unsafe {
            let mut buf = [0u8; 128];
            let s_arg = b"physx\0";
            let fmt = b"%d %u %lld %llu %#x %p %.2f %g %s %c %05d\0";
            let args: [u64; 14] = [
                buf.as_mut_ptr() as u64,
                buf.len() as u64,
                fmt.as_ptr() as u64,
                (-42i64) as u64,
                99u64,
                (-1_000_000_000_000i64) as u64,
                12_345_678_901_234u64,
                0xabu64,
                0x1234u64,
                3.14159f64.to_bits(),
                2.5f64.to_bits(),
                s_arg.as_ptr() as u64,
                b'Z' as u64,
                7u64,
            ];
            let n = shdfnd_snprintf_va(args.as_ptr());
            let out = CStr::from_ptr(buf.as_ptr() as *const libc::c_char)
                .to_str()
                .unwrap();
            assert_eq!(
                out,
                "-42 99 -1000000000000 12345678901234 0xab 0000000000001234 3.14 2.5 physx Z 00007"
            );
            assert_eq!(n as usize, out.len());

            // Truncation: NUL-terminated and returns full required length.
            let mut small = [0xCCu8; 6];
            let t_fmt = b"hello %d\0";
            let t_args: [u64; 4] = [
                small.as_mut_ptr() as u64,
                small.len() as u64,
                t_fmt.as_ptr() as u64,
                12345,
            ];
            let needed = shdfnd_snprintf_va(t_args.as_ptr());
            assert_eq!(needed, 11);
            assert_eq!(&small, b"hello\0");
        }
    }

    #[repr(C)]
    struct TestRunnable {
        vtbl: *const [usize; 2],
        ran: AtomicU32,
        thread_ptr: *mut ThreadImpl,
    }

    unsafe extern "win64" fn test_runnable_dtor(_this: *mut TestRunnable) {}

    unsafe extern "win64" fn test_runnable_execute(this: *mut TestRunnable) {
        (*this).ran.fetch_add(1, Ordering::SeqCst);
        let th = (*this).thread_ptr;
        while thread_quit_is_signalled(th) == 0 {
            std::thread::yield_now();
        }
        (*this).ran.fetch_add(10, Ordering::SeqCst);
    }

    #[test]
    fn thread_impl_starts_executes_and_wakes_wait_for_quit() {
        unsafe {
            let mut impl_obj: ThreadImpl = std::mem::zeroed();
            thread_ctor(&mut impl_obj);
            assert_eq!(impl_obj.state, THREAD_NOT_STARTED);
            assert_eq!(thread_wait_for_quit(&mut impl_obj), 0);

            let vtbl = [
                test_runnable_dtor as *const () as usize,
                test_runnable_execute as *const () as usize,
            ];
            let mut runnable = TestRunnable {
                vtbl: &vtbl,
                ran: AtomicU32::new(0),
                thread_ptr: &mut impl_obj,
            };

            thread_start(&mut impl_obj, 0, &mut runnable as *mut TestRunnable as usize);
            assert_ne!(impl_obj.handle, 0);
            assert_ne!(impl_obj.thread_id, 0);

            // Wait until the worker thread enters execute().
            while runnable.ran.load(Ordering::SeqCst) == 0 {
                std::thread::yield_now();
            }
            thread_signal_quit(&mut impl_obj);
            assert_eq!(thread_quit_is_signalled(&mut impl_obj), 1);

            assert_eq!(thread_wait_for_quit(&mut impl_obj), 1);
            assert_eq!(runnable.ran.load(Ordering::SeqCst), 11);
            assert_eq!(impl_obj.state, THREAD_STOPPED);

            thread_dtor(&mut impl_obj);
            assert_eq!(impl_obj.handle, 0);
        }
    }
}
