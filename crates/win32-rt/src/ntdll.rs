//! ntdll.dll: the subset used via direct imports or `GetProcAddress`.
//! Rtl* exception/unwind functions live in [`crate::unwind`] and are reached
//! through the kernel32 table (lookup falls back between the two).

use crate::heap;
use crate::strings::*;
use crate::teb;
use crate::types::*;
use std::ffi::c_void;

pub static EXPORTS: &[crate::Export] = include!(concat!(env!("OUT_DIR"), "/ntdll_exports.rs"));

pub const STATUS_SUCCESS: NTSTATUS = 0;
pub const STATUS_BUFFER_OVERFLOW: NTSTATUS = 0x8000_0005u32 as i32;
pub const STATUS_NOT_IMPLEMENTED: NTSTATUS = 0xC000_0002u32 as i32;
pub const STATUS_INVALID_INFO_CLASS: NTSTATUS = 0xC000_0003u32 as i32;
pub const STATUS_INFO_LENGTH_MISMATCH: NTSTATUS = 0xC000_0004u32 as i32;
pub const STATUS_INVALID_HANDLE: NTSTATUS = 0xC000_0008u32 as i32;
pub const STATUS_INVALID_PARAMETER: NTSTATUS = 0xC000_000Du32 as i32;
pub const STATUS_NO_MEMORY: NTSTATUS = 0xC000_0017u32 as i32;
pub const STATUS_CONFLICTING_ADDRESSES: NTSTATUS = 0xC000_0018u32 as i32;
pub const STATUS_NOT_MAPPED_VIEW: NTSTATUS = 0xC000_0019u32 as i32;
pub const STATUS_ACCESS_DENIED: NTSTATUS = 0xC000_0022u32 as i32;
pub const STATUS_BUFFER_TOO_SMALL: NTSTATUS = 0xC000_0023u32 as i32;
pub const STATUS_FILE_INVALID: NTSTATUS = 0xC000_0098u32 as i32;
pub const STATUS_INVALID_ADDRESS: NTSTATUS = 0xC000_0141u32 as i32;
pub const STATUS_NO_MORE_ENTRIES: NTSTATUS = 0x8000_001Au32 as i32;
pub const STATUS_PORT_NOT_SET: NTSTATUS = 0xC000_0353u32 as i32;

/// RTL_OSVERSIONINFOW / RTL_OSVERSIONINFOEXW.
pub unsafe extern "win64" fn RtlGetVersion(v: *mut u32) -> NTSTATUS {
    let size = *v;
    *v.add(1) = teb::OS_MAJOR;
    *v.add(2) = teb::OS_MINOR;
    *v.add(3) = teb::OS_BUILD;
    *v.add(4) = 2; // VER_PLATFORM_WIN32_NT
    std::ptr::write_bytes(v.add(5) as *mut u16, 0, 128);
    if size >= 284 {
        let p = v as *mut u8;
        *(p.add(276) as *mut u16) = 0; // SP major
        *(p.add(278) as *mut u16) = 0;
        *(p.add(280) as *mut u16) = 0x100; // VER_SUITE_SINGLEUSERTS
        *p.add(282) = 1; // VER_NT_WORKSTATION
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32) {
    if !major.is_null() {
        *major = teb::OS_MAJOR;
    }
    if !minor.is_null() {
        *minor = teb::OS_MINOR;
    }
    if !build.is_null() {
        *build = 0xF000_0000 | teb::OS_BUILD;
    }
}

pub fn ntstatus_to_win32(s: u32) -> u32 {
    match s {
        0 => 0,
        0x0000_0102 => err::ERROR_TIMEOUT,
        0x0000_0103 => err::ERROR_IO_PENDING,
        0xC000_0005 => err::ERROR_NOACCESS,
        0xC000_0120 => 995,  // STATUS_CANCELLED -> ERROR_OPERATION_ABORTED
        0xC000_009D => 1167, // STATUS_DEVICE_NOT_CONNECTED -> ERROR_DEVICE_NOT_CONNECTED
        0xC000_0185 => 1117, // STATUS_IO_DEVICE_ERROR -> ERROR_IO_DEVICE
        0xC000_0008 => err::ERROR_INVALID_HANDLE,
        0xC000_000D => err::ERROR_INVALID_PARAMETER,
        0xC000_0011 => err::ERROR_HANDLE_EOF,
        0xC000_0017 => err::ERROR_NOT_ENOUGH_MEMORY,
        0xC000_0022 => err::ERROR_ACCESS_DENIED,
        0xC000_0023 => err::ERROR_INSUFFICIENT_BUFFER,
        0xC000_0034 => err::ERROR_FILE_NOT_FOUND,
        0xC000_0035 => err::ERROR_ALREADY_EXISTS,
        0xC000_003A => err::ERROR_PATH_NOT_FOUND,
        0xC000_0002 => err::ERROR_CALL_NOT_IMPLEMENTED,
        0xC000_00BB => err::ERROR_NOT_SUPPORTED,
        0xC000_0135 => err::ERROR_MOD_NOT_FOUND,
        0xC000_0139 => err::ERROR_PROC_NOT_FOUND,
        0x8000_0005 => err::ERROR_MORE_DATA,
        s if s & 0xFFFF_0000 == 0x8007_0000 => s & 0xFFFF,
        _ => 317, // ERROR_MR_MID_NOT_FOUND
    }
}

pub unsafe extern "win64" fn RtlNtStatusToDosError(s: NTSTATUS) -> u32 {
    ntstatus_to_win32(s as u32)
}
pub unsafe extern "win64" fn RtlNtStatusToDosErrorNoTeb(s: NTSTATUS) -> u32 {
    ntstatus_to_win32(s as u32)
}
pub unsafe extern "win64" fn RtlGetLastWin32Error() -> u32 {
    teb::last_error()
}
pub unsafe extern "win64" fn RtlSetLastWin32Error(e: u32) {
    teb::set_last_error(e)
}
pub unsafe extern "win64" fn RtlGetCurrentPeb() -> *mut u8 {
    teb::peb()
}
pub unsafe extern "win64" fn NtCurrentTeb() -> *mut u8 {
    teb::current()
}

pub unsafe extern "win64" fn NtQueryInformationProcess(h: HANDLE, class: u32, out: *mut u8, len: u32, ret: *mut u32) -> NTSTATUS {
    let put_ret = |n: u32| {
        if !ret.is_null() {
            *ret = n;
        }
    };
    match class {
        0 => {
            // PROCESS_BASIC_INFORMATION
            if len < 48 || out.is_null() {
                put_ret(48);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 48);
            *(out.add(8) as *mut usize) = teb::peb() as usize;
            *(out.add(16) as *mut usize) = 1;
            *(out.add(24) as *mut usize) = 8;
            *(out.add(32) as *mut usize) = std::process::id() as usize;
            put_ret(48);
            STATUS_SUCCESS
        }
        4 => {
            // ProcessTimes (KERNEL_USER_TIMES: 32 bytes)
            put_ret(32);
            if len < 32 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 32);
            let now = now_filetime();
            let up = crate::kernel32::time::interrupt_time_100ns();
            (out as *mut u64).write_unaligned(now.saturating_sub(up)); // CreationTime
            (out.add(16) as *mut u64).write_unaligned(up / 4); // KernelTime
            (out.add(24) as *mut u64).write_unaligned(up / 2); // UserTime
            STATUS_SUCCESS
        }
        7 => {
            // ProcessDebugPort: 0 = not being debugged.
            if len < 8 || out.is_null() {
                put_ret(8);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut usize) = 0;
            put_ret(8);
            STATUS_SUCCESS
        }
        0x1A => {
            // ProcessWow64Information: 0 = native 64-bit process.
            if len < 8 || out.is_null() {
                put_ret(8);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut usize) = 0;
            put_ret(8);
            STATUS_SUCCESS
        }
        0x1B | 0x2B => {
            // ProcessImageFileName (0x1B) / ProcessImageFileNameWin32 (0x2B)
            let win_exe = crate::process::try_config()
                .map(|c| c.exe_path.clone())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "C:\\Games\\Game.exe".to_string());
            let path = if class == 0x1B {
                crate::path::win32_to_nt_device_path(&win_exe)
            } else {
                win_exe
            };
            let wide: Vec<u16> = path.encode_utf16().collect();
            let byte_len = wide.len() * 2;
            let needed = (16 + byte_len + 2) as u32;
            put_ret(needed);
            if len < needed || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, needed as usize);
            let buf_ptr = out.add(16) as *mut u16;
            std::ptr::copy_nonoverlapping(wide.as_ptr(), buf_ptr, wide.len());
            (out as *mut u16).write_unaligned(byte_len as u16);
            (out.add(2) as *mut u16).write_unaligned((byte_len + 2) as u16);
            (out.add(8) as *mut usize).write_unaligned(buf_ptr as usize);
            STATUS_SUCCESS
        }
        0x1E => {
            // ProcessDebugObjectHandle: anti-cheats require STATUS_PORT_NOT_SET (0xC0000353)
            if len < 8 || out.is_null() {
                put_ret(8);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut usize) = 0;
            put_ret(8);
            STATUS_PORT_NOT_SET
        }
        0x1F => {
            // ProcessDebugFlags: 1 = NoDebugInherit (= NOT debugged)
            if len < 4 || out.is_null() {
                put_ret(4);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut u32) = 1;
            put_ret(4);
            STATUS_SUCCESS
        }
        0x22 => {
            // ProcessExecuteFlags: MEM_EXECUTE_OPTION_DISABLE (DEP enabled)
            if len < 4 || out.is_null() {
                put_ret(4);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut u32) = 0x2F;
            put_ret(4);
            STATUS_SUCCESS
        }
        0x24 => {
            // ProcessCookie
            if len < 4 || out.is_null() {
                put_ret(4);
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *(out as *mut u32) = 0x4157_3250;
            put_ret(4);
            STATUS_SUCCESS
        }
        _ => {
            let _ = h;
            stub!("NtQueryInformationProcess class {class}");
            STATUS_INVALID_INFO_CLASS
        }
    }
}

pub unsafe extern "win64" fn NtSetInformationProcess(
    _h: HANDLE,
    class: u32,
    _info: *const c_void,
    _len: u32,
) -> NTSTATUS {
    log::debug!("NtSetInformationProcess(class={class}) accepted");
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtSetInformationThread(_h: HANDLE, class: u32, info: *const c_void, len: u32) -> NTSTATUS {
    // Anti-cheats probe ThreadHideFromDebugger (0x11) with non-zero length or non-null
    // buffer to detect naive stubs that unconditionally return STATUS_SUCCESS!
    if class == 0x11 {
        if len != 0 || !info.is_null() {
            return STATUS_INFO_LENGTH_MISMATCH;
        }
        return STATUS_SUCCESS;
    }
    log::debug!("NtSetInformationThread(class={class}) accepted");
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtQueryInformationThread(h: HANDLE, class: u32, out: *mut u8, len: u32, ret: *mut u32) -> NTSTATUS {
    match class {
        0 => {
            // THREAD_BASIC_INFORMATION
            if len < 48 || out.is_null() {
                if !ret.is_null() {
                    *ret = 48;
                }
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 48);
            let tid = crate::kernel32::threads::GetThreadId(h);
            let tebp = crate::thread::find_by_tid(tid)
                .map(|t| t.teb.load(std::sync::atomic::Ordering::Relaxed))
                .unwrap_or(teb::current() as usize);
            *(out.add(4 + 4) as *mut usize) = tebp; // ExitStatus(4)+pad, TebBaseAddress
            *(out.add(16) as *mut usize) = std::process::id() as usize;
            *(out.add(24) as *mut usize) = tid as usize;
            if !ret.is_null() {
                *ret = 48;
            }
            STATUS_SUCCESS
        }
        0x11 => {
            // ThreadHideFromDebugger
            if len != 1 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *out = 0;
            if !ret.is_null() {
                *ret = 1;
            }
            STATUS_SUCCESS
        }
        _ => {
            stub!("NtQueryInformationThread class {class}");
            STATUS_INVALID_INFO_CLASS
        }
    }
}

pub unsafe extern "win64" fn NtQuerySystemInformation(class: u32, out: *mut u8, len: u32, ret: *mut u32) -> NTSTATUS {
    let put_ret = |n: u32| {
        if !ret.is_null() {
            *ret = n;
        }
    };
    match class {
        0 => {
            // SystemBasicInformation (64 bytes on x64)
            put_ret(64);
            if len < 64 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 64);
            (out.add(4) as *mut u32).write_unaligned(156_250); // TimerResolution
            (out.add(8) as *mut u32).write_unaligned(0x1000); // PageSize
            let total_pages = (crate::process::physical_memory() / 0x1000) as u32;
            (out.add(12) as *mut u32).write_unaligned(total_pages); // NumberOfPhysicalPages
            (out.add(16) as *mut u32).write_unaligned(1); // LowestPhysicalPageNumber
            (out.add(20) as *mut u32).write_unaligned(total_pages); // HighestPhysicalPageNumber
            (out.add(24) as *mut u32).write_unaligned(0x10000); // AllocationGranularity
            (out.add(32) as *mut usize).write_unaligned(0x10000); // MinimumUserModeAddress
            (out.add(40) as *mut usize).write_unaligned(0x7FFF_FFFE_FFFF); // MaximumUserModeAddress
            let n = crate::process::num_cpus();
            let mask = if n >= 64 { usize::MAX } else { (1usize << n) - 1 };
            (out.add(48) as *mut usize).write_unaligned(mask); // ActiveProcessorsAffinityMask
            *out.add(56) = n as u8; // NumberOfProcessors
            STATUS_SUCCESS
        }
        1 => {
            // SystemProcessorInformation (12 bytes)
            put_ret(12);
            if len < 12 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 12);
            (out as *mut u16).write_unaligned(9); // PROCESSOR_ARCHITECTURE_AMD64
            (out.add(2) as *mut u16).write_unaligned(6); // ProcessorLevel
            (out.add(8) as *mut u32).write_unaligned(0); // ProcessorFeatureBits
            STATUS_SUCCESS
        }
        2 => {
            // SystemPerformanceInformation (312 bytes)
            put_ret(312);
            if len < 312 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, len as usize);
            let avail_pages = (crate::kernel32::sysinfo::available_physical_memory() / 0x1000) as u32;
            (out.add(8) as *mut u32).write_unaligned(avail_pages); // AvailablePages
            STATUS_SUCCESS
        }
        3 => {
            // SystemTimeOfDayInformation (48 bytes)
            put_ret(48);
            if len < 48 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, 48);
            let now = now_filetime();
            let up = crate::kernel32::time::interrupt_time_100ns();
            (out as *mut u64).write_unaligned(now.saturating_sub(up)); // BootTime
            (out.add(8) as *mut u64).write_unaligned(now); // CurrentTime
            STATUS_SUCCESS
        }
        5 => {
            // SystemProcessInformation (single entry for our process, no debugger/cheat processes)
            let needed = 256u32;
            put_ret(needed);
            if len < needed || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, needed as usize);
            (out.add(4) as *mut u32).write_unaligned(1); // NumberOfThreads
            (out.add(80) as *mut usize).write_unaligned(std::process::id() as usize); // UniqueProcessId
            STATUS_SUCCESS
        }
        0x0B => {
            // SystemModuleInformation (RTL_PROCESS_MODULES with 1 clean ntoskrnl.exe entry)
            let needed = 304u32;
            put_ret(needed);
            if len < needed || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            std::ptr::write_bytes(out, 0, needed as usize);
            (out as *mut u32).write_unaligned(1); // NumberOfModules = 1
            let path = b"\\SystemRoot\\system32\\ntoskrnl.exe\0";
            std::ptr::copy_nonoverlapping(path.as_ptr(), out.add(48), path.len());
            STATUS_SUCCESS
        }
        0x23 => {
            // SystemKernelDebuggerInformation (2 bytes):
            // KdDebuggerEnabled = 0, KdDebuggerNotPresent = 1
            put_ret(2);
            if len < 2 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            *out = 0; // KdDebuggerEnabled = FALSE
            *out.add(1) = 1; // KdDebuggerNotPresent = TRUE
            STATUS_SUCCESS
        }
        0x67 => {
            // SystemCodeIntegrityInformation (8 bytes):
            // Length = 8, CodeIntegrityOptions = 0x01 (CODEINTEGRITY_OPTION_ENABLED, TESTSIGNING = 0)
            put_ret(8);
            if len < 8 || out.is_null() {
                return STATUS_INFO_LENGTH_MISMATCH;
            }
            (out as *mut u32).write_unaligned(8);
            (out.add(4) as *mut u32).write_unaligned(0x01); // CODEINTEGRITY_OPTION_ENABLED
            STATUS_SUCCESS
        }
        _ => {
            stub!("NtQuerySystemInformation class {class}");
            put_ret(0);
            STATUS_INVALID_INFO_CLASS
        }
    }
}

pub unsafe extern "win64" fn RtlVerifyVersionInfo(
    v: *const crate::kernel32::sysinfo::OSVERSIONINFOEXW,
    type_mask: u32,
    cond_mask: u64,
) -> NTSTATUS {
    if crate::kernel32::sysinfo::VerifyVersionInfoW(v, type_mask, cond_mask) != 0 {
        STATUS_SUCCESS
    } else {
        0xC000_0059u32 as i32 // STATUS_REVISION_MISMATCH
    }
}

pub unsafe extern "win64" fn RtlAreLongPathsEnabled() -> u8 {
    1
}

pub unsafe extern "win64" fn RtlDllShutdownInProgress() -> u8 {
    0
}

static TIMER_RES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(156_250);

pub unsafe extern "win64" fn NtSetTimerResolution(desired: u32, set: u8, current: *mut u32) -> NTSTATUS {
    if set != 0 {
        TIMER_RES.store(desired.max(5000), std::sync::atomic::Ordering::Relaxed);
    }
    if !current.is_null() {
        *current = TIMER_RES.load(std::sync::atomic::Ordering::Relaxed);
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtQueryTimerResolution(min: *mut u32, max: *mut u32, cur: *mut u32) -> NTSTATUS {
    *min = 156_250;
    *max = 5000;
    *cur = TIMER_RES.load(std::sync::atomic::Ordering::Relaxed);
    STATUS_SUCCESS
}

/// Convert an NT `PLARGE_INTEGER` timeout/interval (100ns units; negative = relative,
/// positive = absolute UTC FILETIME, null = infinite) into an `Option<Duration>`.
/// Preserves sub-millisecond non-zero intervals (minimum 1 µs) so callers do not
/// silently truncate short relative waits to 0 ms.
unsafe fn nt_timeout_to_duration(timeout: *const i64) -> Option<std::time::Duration> {
    if timeout.is_null() {
        return None;
    }
    let v = std::ptr::read_unaligned(timeout);
    if v == 0 {
        return Some(std::time::Duration::ZERO);
    }
    if v < 0 {
        let nanos = v.unsigned_abs().saturating_mul(100).max(1_000);
        Some(std::time::Duration::from_nanos(nanos))
    } else {
        let now = now_filetime() as i64;
        if v <= now {
            Some(std::time::Duration::ZERO)
        } else {
            let delta_100ns = (v - now) as u64;
            let nanos = delta_100ns.saturating_mul(100).max(1_000);
            Some(std::time::Duration::from_nanos(nanos))
        }
    }
}

pub unsafe extern "win64" fn NtDelayExecution(alertable: u8, interval: *const i64) -> NTSTATUS {
    if interval.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let dur = nt_timeout_to_duration(interval);
    if alertable != 0 {
        if crate::handles::alertable_sleep_duration(dur) {
            return 0x0000_00C0; // STATUS_USER_APC
        }
        return STATUS_SUCCESS;
    }
    match dur {
        None => loop {
            std::thread::park();
        },
        Some(d) if d.is_zero() => std::thread::yield_now(),
        Some(d) => std::thread::sleep(d),
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtYieldExecution() -> NTSTATUS {
    libc::sched_yield();
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtClose(h: HANDLE) -> NTSTATUS {
    if crate::kernel32::threads::CloseHandle(h) != 0 {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    }
}

// Heap
pub unsafe extern "win64" fn RtlAllocateHeap(h: HANDLE, flags: u32, size: usize) -> *mut c_void {
    crate::kernel32::memory::HeapAlloc(h, flags, size)
}
pub unsafe extern "win64" fn RtlFreeHeap(h: HANDLE, flags: u32, p: *mut c_void) -> u8 {
    crate::kernel32::memory::HeapFree(h, flags, p) as u8
}
pub unsafe extern "win64" fn RtlReAllocateHeap(h: HANDLE, flags: u32, p: *mut c_void, size: usize) -> *mut c_void {
    crate::kernel32::memory::HeapReAlloc(h, flags, p, size)
}
pub unsafe extern "win64" fn RtlSizeHeap(_h: HANDLE, _flags: u32, p: *mut c_void) -> usize {
    heap::size(p).unwrap_or(usize::MAX)
}

// Memory
pub unsafe extern "win64" fn RtlZeroMemory(d: *mut u8, n: usize) {
    std::ptr::write_bytes(d, 0, n)
}
pub unsafe extern "win64" fn RtlFillMemory(d: *mut u8, n: usize, v: u8) {
    std::ptr::write_bytes(d, v, n)
}
pub unsafe extern "win64" fn RtlMoveMemory(d: *mut u8, s: *const u8, n: usize) {
    std::ptr::copy(s, d, n)
}
pub unsafe extern "win64" fn RtlCopyMemory(d: *mut u8, s: *const u8, n: usize) {
    std::ptr::copy(s, d, n)
}
pub unsafe extern "win64" fn RtlCompareMemory(a: *const u8, b: *const u8, n: usize) -> usize {
    (0..n).find(|&i| *a.add(i) != *b.add(i)).unwrap_or(n)
}

// Strings
#[repr(C)]
pub struct UNICODE_STRING {
    pub len: u16,
    pub max: u16,
    pub buf: *mut u16,
}

pub unsafe extern "win64" fn RtlInitUnicodeString(us: *mut UNICODE_STRING, s: LPCWSTR) {
    let n = if s.is_null() { 0 } else { wcslen(s) };
    (*us).len = (n * 2) as u16;
    (*us).max = if s.is_null() { 0 } else { (n * 2 + 2) as u16 };
    (*us).buf = s as *mut u16;
}

pub unsafe extern "win64" fn RtlFreeUnicodeString(us: *mut UNICODE_STRING) {
    if !(*us).buf.is_null() {
        heap::free((*us).buf as *mut c_void);
        (*us).buf = std::ptr::null_mut();
    }
}

static RAND_STATE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub unsafe extern "win64" fn RtlRandomEx(seed: *mut u32) -> u32 {
    let s = if seed.is_null() { RAND_STATE.load(std::sync::atomic::Ordering::Relaxed) } else { *seed };
    let n = s.wrapping_mul(0x7FFF_FFED).wrapping_add(0x7FFF_FFC3) % 0x7FFF_FFFF;
    if !seed.is_null() {
        *seed = n;
    } else {
        RAND_STATE.store(n, std::sync::atomic::Ordering::Relaxed);
    }
    n
}
pub unsafe extern "win64" fn RtlRandom(seed: *mut u32) -> u32 {
    RtlRandomEx(seed)
}
pub unsafe extern "win64" fn RtlGenRandom(buf: *mut c_void, len: u32) -> u8 {
    libc::arc4random_buf(buf, len as usize);
    1
}

// Critical sections under their Rtl names.
pub unsafe extern "win64" fn RtlInitializeCriticalSection(cs: *mut crate::sync::CriticalSection) -> NTSTATUS {
    crate::sync::InitializeCriticalSection(cs);
    STATUS_SUCCESS
}
pub unsafe extern "win64" fn RtlInitializeCriticalSectionAndSpinCount(cs: *mut crate::sync::CriticalSection, spin: u32) -> NTSTATUS {
    crate::sync::InitializeCriticalSectionAndSpinCount(cs, spin);
    STATUS_SUCCESS
}
pub unsafe extern "win64" fn RtlEnterCriticalSection(cs: *mut crate::sync::CriticalSection) -> NTSTATUS {
    crate::sync::EnterCriticalSection(cs);
    STATUS_SUCCESS
}
pub unsafe extern "win64" fn RtlLeaveCriticalSection(cs: *mut crate::sync::CriticalSection) -> NTSTATUS {
    crate::sync::LeaveCriticalSection(cs);
    STATUS_SUCCESS
}
pub unsafe extern "win64" fn RtlTryEnterCriticalSection(cs: *mut crate::sync::CriticalSection) -> u8 {
    crate::sync::TryEnterCriticalSection(cs) as u8
}
pub unsafe extern "win64" fn RtlDeleteCriticalSection(cs: *mut crate::sync::CriticalSection) -> NTSTATUS {
    crate::sync::DeleteCriticalSection(cs);
    STATUS_SUCCESS
}
pub unsafe extern "win64" fn RtlIsCriticalSectionLockedByThread(cs: *const crate::sync::CriticalSection) -> u32 {
    if cs.is_null() {
        return 0;
    }
    crate::sync::is_cs_locked_by_thread(cs, teb::thread_id() as usize) as u32
}
pub unsafe extern "win64" fn RtlAcquireSRWLockExclusive(p: *mut c_void) {
    crate::sync::AcquireSRWLockExclusive(p)
}
pub unsafe extern "win64" fn RtlReleaseSRWLockExclusive(p: *mut c_void) {
    crate::sync::ReleaseSRWLockExclusive(p)
}
pub unsafe extern "win64" fn RtlAcquireSRWLockShared(p: *mut c_void) {
    crate::sync::AcquireSRWLockShared(p)
}
pub unsafe extern "win64" fn RtlReleaseSRWLockShared(p: *mut c_void) {
    crate::sync::ReleaseSRWLockShared(p)
}
pub unsafe extern "win64" fn RtlInitializeSRWLock(p: *mut c_void) {
    crate::sync::InitializeSRWLock(p)
}

pub unsafe extern "win64" fn NtAllocateVirtualMemoryEx(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    region_size: *mut usize,
    alloc_type: u32,
    page_prot: u32,
    ext_params: *mut c_void,
    ext_count: u32,
) -> NTSTATUS {
    if base_addr.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let p = crate::kernel32::memory::VirtualAlloc2(
        proc_,
        *base_addr,
        *region_size,
        alloc_type,
        page_prot,
        ext_params,
        ext_count,
    );
    if p.is_null() {
        0xC000_0017u32 as i32 // STATUS_NO_MEMORY
    } else {
        *base_addr = p;
        STATUS_SUCCESS
    }
}

pub unsafe extern "win64" fn RtlQueryWnfStateData(
    change_stamp: *mut u32,
    _state_name: u64,
    _cb: *const c_void,
    _ctx: *mut c_void,
    _type_id: *const c_void,
) -> NTSTATUS {
    if !change_stamp.is_null() {
        *change_stamp = 0;
    }
    0xC000_0034u32 as i32 // STATUS_OBJECT_NAME_NOT_FOUND
}

pub unsafe extern "win64" fn RtlSubscribeWnfStateChangeNotification(
    sub_out: *mut usize,
    _state_name: u64,
    _change_stamp: u32,
    _cb: *const c_void,
    _ctx: *mut c_void,
    _type_id: *const c_void,
    _group: u32,
    _flags: u32,
) -> NTSTATUS {
    static NEXT_WNF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x1000);
    if !sub_out.is_null() {
        *sub_out = NEXT_WNF.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn RtlUnsubscribeWnfNotificationWaitForCompletion(_sub: usize) -> NTSTATUS {
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtCreateTransaction(
    out: *mut HANDLE,
    _access: u32,
    _obj_attr: *const c_void,
    _uow: *const u8,
    _tm: HANDLE,
    _create_opts: u32,
    _iso_level: u32,
    _iso_flags: u32,
    _timeout: *const i64,
    _desc: *const UNICODE_STRING,
) -> NTSTATUS {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("NtCreateTransaction -> STATUS_NOT_IMPLEMENTED");
    STATUS_NOT_IMPLEMENTED
}

pub unsafe extern "win64" fn NtCommitTransaction(_txn: HANDLE, _wait: u8) -> NTSTATUS {
    stub!("NtCommitTransaction -> STATUS_NOT_IMPLEMENTED");
    STATUS_NOT_IMPLEMENTED
}

pub unsafe extern "win64" fn RtlQueryPerformanceCounter(out: *mut i64) -> u8 {
    crate::kernel32::time::QueryPerformanceCounter(out) as u8
}
pub unsafe extern "win64" fn RtlQueryPerformanceFrequency(out: *mut i64) -> u8 {
    crate::kernel32::time::QueryPerformanceFrequency(out) as u8
}

pub unsafe extern "win64" fn RtlIsProcessorFeaturePresent(f: u32) -> u8 {
    crate::kernel32::sysinfo::processor_feature(f) as u8
}

pub unsafe extern "win64" fn LdrAddRefDll(_flags: u32, _h: HMODULE) -> NTSTATUS {
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn EtwEventRegister(_g: *const GUID, _cb: *const c_void, _ctx: *mut c_void, h: *mut u64) -> u32 {
    *h = 0;
    0
}
pub unsafe extern "win64" fn EtwEventUnregister(_h: u64) -> u32 {
    0
}
pub unsafe extern "win64" fn EtwEventWrite(_h: u64, _d: *const c_void, _n: u32, _data: *const c_void) -> u32 {
    0
}

// ---------------------------------------------------------------------------
// Direct Virtual Memory, Context, VEH & Loader NT APIs (Anti-Cheat / Packer)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn NtAllocateVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    _zero_bits: usize,
    region_size: *mut usize,
    alloc_type: u32,
    protect: u32,
) -> NTSTATUS {
    if base_addr.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let p = crate::kernel32::memory::VirtualAllocEx(proc_, *base_addr, *region_size, alloc_type, protect);
    if p.is_null() {
        0xC000_0017u32 as i32 // STATUS_NO_MEMORY
    } else {
        *base_addr = p;
        *region_size = (*region_size + 0xFFF) & !0xFFF;
        STATUS_SUCCESS
    }
}

pub unsafe extern "win64" fn ZwAllocateVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    zero_bits: usize,
    region_size: *mut usize,
    alloc_type: u32,
    protect: u32,
) -> NTSTATUS {
    NtAllocateVirtualMemory(proc_, base_addr, zero_bits, region_size, alloc_type, protect)
}

pub unsafe extern "win64" fn NtFreeVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    region_size: *mut usize,
    free_type: u32,
) -> NTSTATUS {
    if base_addr.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if crate::kernel32::memory::VirtualFreeEx(proc_, *base_addr, *region_size, free_type) != 0 {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_PARAMETER
    }
}

pub unsafe extern "win64" fn ZwFreeVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    region_size: *mut usize,
    free_type: u32,
) -> NTSTATUS {
    NtFreeVirtualMemory(proc_, base_addr, region_size, free_type)
}

pub unsafe extern "win64" fn NtProtectVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    region_size: *mut usize,
    new_protect: u32,
    old_protect: *mut u32,
) -> NTSTATUS {
    if base_addr.is_null() || region_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if crate::kernel32::memory::VirtualProtectEx(proc_, *base_addr, *region_size, new_protect, old_protect) != 0 {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_PARAMETER
    }
}

pub unsafe extern "win64" fn ZwProtectVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut *mut c_void,
    region_size: *mut usize,
    new_protect: u32,
    old_protect: *mut u32,
) -> NTSTATUS {
    NtProtectVirtualMemory(proc_, base_addr, region_size, new_protect, old_protect)
}

pub unsafe extern "win64" fn NtQueryVirtualMemory(
    proc_: HANDLE,
    base_addr: *const c_void,
    info_class: u32,
    info: *mut c_void,
    len: usize,
    ret_len: *mut usize,
) -> NTSTATUS {
    if !proc_.is_null() && !crate::kernel32::process::is_self(proc_) {
        if !ret_len.is_null() {
            std::ptr::write_unaligned(ret_len, 0);
        }
        log::debug!("NtQueryVirtualMemory(proc={proc_:p}, class={info_class}, {base_addr:p}) -> STATUS_ACCESS_DENIED");
        return STATUS_ACCESS_DENIED;
    }
    match info_class {
        0 => {
            // MemoryBasicInformation
            let got = crate::kernel32::memory::VirtualQueryEx(
                proc_,
                base_addr,
                info as *mut crate::kernel32::memory::MemoryBasicInformation,
                len,
            );
            log::trace!("NtQueryVirtualMemory(Basic, proc={proc_:p}, {base_addr:p}) -> got={got}");
            if !ret_len.is_null() {
                std::ptr::write_unaligned(ret_len, got);
            }
            if got == 0 {
                STATUS_INVALID_PARAMETER
            } else {
                STATUS_SUCCESS
            }
        }
        2 => {
            // MemoryMappedFilenameInformation (UNICODE_STRING + NUL-terminated UTF-16 NT device path)
            let addr = base_addr as usize;
            let mapped = crate::try_loader()
                .and_then(|l| l.module_from_address(addr).and_then(|h| l.module_path(Some(h))))
                .map(Some)
                .or_else(|| crate::kernel32::file::mapped_file_path_at(addr));
            let win_path = match mapped {
                Some(Some(p)) => p,
                Some(None) => {
                    if !ret_len.is_null() {
                        std::ptr::write_unaligned(ret_len, 0);
                    }
                    log::debug!("NtQueryVirtualMemory(MemoryMappedFilenameInformation, {base_addr:p}) -> STATUS_FILE_INVALID (anonymous section)");
                    return STATUS_FILE_INVALID;
                }
                None => {
                    if !ret_len.is_null() {
                        std::ptr::write_unaligned(ret_len, 0);
                    }
                    log::debug!("NtQueryVirtualMemory(MemoryMappedFilenameInformation, {base_addr:p}) -> STATUS_INVALID_ADDRESS");
                    return STATUS_INVALID_ADDRESS;
                }
            };
            let nt_path = crate::path::win32_to_nt_device_path(&win_path);
            let wide: Vec<u16> = nt_path.encode_utf16().collect();
            let byte_len = wide.len() * 2;
            let needed = 16 + byte_len + 2;
            if !ret_len.is_null() {
                std::ptr::write_unaligned(ret_len, needed);
            }
            if info.is_null() || len < needed {
                log::debug!(
                    "NtQueryVirtualMemory(MemoryMappedFilenameInformation, {base_addr:p}, len={len}) -> STATUS_BUFFER_OVERFLOW (need {needed} for {nt_path:?})"
                );
                return STATUS_BUFFER_OVERFLOW;
            }
            let out = info as *mut u8;
            std::ptr::write_bytes(out, 0, needed);
            let buf_ptr = out.add(16) as *mut u16;
            for (i, &c) in wide.iter().enumerate() {
                std::ptr::write_unaligned(buf_ptr.add(i), c);
            }
            std::ptr::write_unaligned(buf_ptr.add(wide.len()), 0u16);
            std::ptr::write_unaligned(out as *mut u16, byte_len as u16);
            std::ptr::write_unaligned(out.add(2) as *mut u16, (byte_len + 2) as u16);
            std::ptr::write_unaligned(out.add(8) as *mut usize, buf_ptr as usize);
            log::debug!("NtQueryVirtualMemory(MemoryMappedFilenameInformation, {base_addr:p}) -> {nt_path:?}");
            STATUS_SUCCESS
        }
        _ => {
            if !ret_len.is_null() {
                std::ptr::write_unaligned(ret_len, 0);
            }
            log::debug!("NtQueryVirtualMemory(class={info_class}, {base_addr:p}) -> STATUS_INVALID_INFO_CLASS");
            STATUS_INVALID_INFO_CLASS
        }
    }
}

pub unsafe extern "win64" fn ZwQueryVirtualMemory(
    proc_: HANDLE,
    base_addr: *const c_void,
    info_class: u32,
    info: *mut c_void,
    len: usize,
    ret_len: *mut usize,
) -> NTSTATUS {
    NtQueryVirtualMemory(proc_, base_addr, info_class, info, len, ret_len)
}

pub unsafe extern "win64" fn NtReadVirtualMemory(
    _proc: HANDLE,
    base_addr: *const c_void,
    buffer: *mut c_void,
    size: usize,
    bytes_read: *mut usize,
) -> NTSTATUS {
    if base_addr.is_null() || buffer.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    std::ptr::copy_nonoverlapping(base_addr as *const u8, buffer as *mut u8, size);
    if !bytes_read.is_null() {
        *bytes_read = size;
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtWriteVirtualMemory(
    proc_: HANDLE,
    base_addr: *mut c_void,
    buffer: *const c_void,
    size: usize,
    bytes_written: *mut usize,
) -> NTSTATUS {
    if base_addr.is_null() || buffer.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    std::ptr::copy_nonoverlapping(buffer as *const u8, base_addr as *mut u8, size);
    let _ = crate::kernel32::memory::FlushInstructionCache(proc_, base_addr, size);
    if !bytes_written.is_null() {
        *bytes_written = size;
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtFlushInstructionCache(
    proc_: HANDLE,
    base_addr: *const c_void,
    len: usize,
) -> NTSTATUS {
    let _ = crate::kernel32::memory::FlushInstructionCache(proc_, base_addr, len);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwFlushInstructionCache(
    proc_: HANDLE,
    base_addr: *const c_void,
    len: usize,
) -> NTSTATUS {
    NtFlushInstructionCache(proc_, base_addr, len)
}

pub unsafe extern "win64" fn NtGetContextThread(
    _thread: HANDLE,
    ctx: *mut crate::unwind::Context,
) -> NTSTATUS {
    if ctx.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let flags = (*ctx).context_flags;
    crate::unwind::RtlCaptureContext(ctx);
    (*ctx).context_flags = flags;
    // Ensure hardware debug registers Dr0..Dr3, Dr6, Dr7 report clean (0 = no hardware breakpoints)
    (*ctx).dr = [0; 6];
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn LdrLockLoaderLock(
    _flags: u32,
    disposition: *mut u32,
    cookie: *mut usize,
) -> NTSTATUS {
    if !disposition.is_null() {
        *disposition = 1; // LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_ACQUIRED
    }
    if !cookie.is_null() {
        *cookie = 0x4C44_5231;
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn LdrUnlockLoaderLock(_flags: u32, _cookie: usize) -> NTSTATUS {
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtSetContextThread(
    _thread: HANDLE,
    ctx: *const crate::unwind::Context,
) -> NTSTATUS {
    if ctx.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtQueryPerformanceCounter(
    counter: *mut i64,
    freq: *mut i64,
) -> NTSTATUS {
    if counter.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    crate::kernel32::time::QueryPerformanceCounter(counter);
    if !freq.is_null() {
        crate::kernel32::time::QueryPerformanceFrequency(freq);
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwQueryPerformanceCounter(
    counter: *mut i64,
    freq: *mut i64,
) -> NTSTATUS {
    NtQueryPerformanceCounter(counter, freq)
}

pub unsafe extern "win64" fn NtOpenProcess(
    out_handle: *mut HANDLE,
    _access: u32,
    _obj_attr: *const c_void,
    _client_id: *const c_void,
) -> NTSTATUS {
    if out_handle.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    *out_handle = crate::handles::CURRENT_PROCESS;
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtOpenThread(
    out_handle: *mut HANDLE,
    _access: u32,
    _obj_attr: *const c_void,
    _client_id: *const c_void,
) -> NTSTATUS {
    if out_handle.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    *out_handle = crate::handles::CURRENT_THREAD;
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn NtDuplicateObject(
    _src_proc: HANDLE,
    src_handle: HANDLE,
    _dst_proc: HANDLE,
    target_handle: *mut HANDLE,
    _desired_access: u32,
    _handle_attributes: u32,
    _options: u32,
) -> NTSTATUS {
    if !target_handle.is_null() {
        *target_handle = src_handle;
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn RtlAddVectoredExceptionHandler(
    first: u32,
    handler: crate::unwind::VectoredHandler,
) -> *mut c_void {
    crate::unwind::AddVectoredExceptionHandler(first, handler)
}

pub unsafe extern "win64" fn RtlRemoveVectoredExceptionHandler(handle: *mut c_void) -> u32 {
    crate::unwind::RemoveVectoredExceptionHandler(handle)
}

pub unsafe extern "win64" fn RtlAddVectoredContinueHandler(
    first: u32,
    handler: crate::unwind::VectoredHandler,
) -> *mut c_void {
    crate::unwind::AddVectoredExceptionHandler(first, handler)
}

pub unsafe extern "win64" fn RtlRemoveVectoredContinueHandler(handle: *mut c_void) -> u32 {
    crate::unwind::RemoveVectoredExceptionHandler(handle)
}

pub unsafe extern "win64" fn LdrGetDllHandle(
    _pw_path: LPCWSTR,
    _unused: *const c_void,
    module_file_name: *const UNICODE_STRING,
    p_hmodule: *mut HMODULE,
) -> NTSTATUS {
    if module_file_name.is_null() || p_hmodule.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let h = crate::kernel32::module::GetModuleHandleW((*module_file_name).buf);
    *p_hmodule = h;
    if h.is_null() {
        0xC000_0135u32 as i32 // STATUS_DLL_NOT_FOUND
    } else {
        STATUS_SUCCESS
    }
}

pub unsafe extern "win64" fn NtContinue(
    ctx: *const crate::unwind::Context,
    test_alert: u8,
) -> NTSTATUS {
    if ctx.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if test_alert != 0 && crate::thread::has_pending_apc() {
        crate::thread::run_pending_apcs();
    }
    if crate::unwind::capture_ntcontinue(&*ctx) {
        return STATUS_SUCCESS;
    }
    crate::unwind::RtlRestoreContext(ctx as *mut _, std::ptr::null_mut());
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwContinue(
    ctx: *const crate::unwind::Context,
    test_alert: u8,
) -> NTSTATUS {
    NtContinue(ctx, test_alert)
}


pub unsafe extern "win64" fn LdrGetProcedureAddress(
    hmodule: HMODULE,
    func_name: *const u8,
    ordinal: u16,
    func_addr: *mut *const c_void,
) -> NTSTATUS {
    if func_addr.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let p = if !func_name.is_null() {
        // ANSI_STRING: { Length: u16, MaximumLength: u16, pad: u32, Buffer: *const u8 }
        let buf = std::ptr::read_unaligned(func_name.add(8) as *const *const u8);
        crate::kernel32::module::GetProcAddress(hmodule, buf)
    } else {
        crate::kernel32::module::GetProcAddress(hmodule, ordinal as usize as *const u8)
    };
    std::ptr::write_unaligned(func_addr, p);
    if p.is_null() {
        0xC000_0139u32 as i32 // STATUS_ENTRYPOINT_NOT_FOUND
    } else {
        STATUS_SUCCESS
    }
}

/// Extract an optional section/object name from a Windows x64 `OBJECT_ATTRIBUTES*`.
/// Layout: `{ Length: u32, pad: u32, RootDirectory: HANDLE, ObjectName: *const UNICODE_STRING, ... }`.
unsafe fn object_attributes_name(obj_attr: *const c_void) -> Option<String> {
    if obj_attr.is_null() {
        return None;
    }
    let name_ptr = std::ptr::read_unaligned((obj_attr as *const u8).add(16) as *const *const UNICODE_STRING);
    if name_ptr.is_null() {
        return None;
    }
    let us = std::ptr::read_unaligned(name_ptr);
    if us.buf.is_null() || us.len == 0 {
        return None;
    }
    let words = (us.len as usize) / 2;
    let mut tmp = Vec::with_capacity(words);
    for i in 0..words {
        tmp.push(std::ptr::read_unaligned(us.buf.add(i)));
    }
    let s = String::from_utf16_lossy(&tmp);
    let trimmed = s
        .strip_prefix("\\BaseNamedObjects\\")
        .or_else(|| s.strip_prefix("\\Sessions\\1\\BaseNamedObjects\\"))
        .unwrap_or(&s);
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub unsafe extern "win64" fn NtCreateSection(
    section_handle: *mut HANDLE,
    _desired_access: u32,
    object_attributes: *const c_void,
    maximum_size: *const i64,
    section_page_protection: u32,
    _allocation_attributes: u32,
    file_handle: HANDLE,
) -> NTSTATUS {
    if section_handle.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let max_size = if maximum_size.is_null() {
        0u64
    } else {
        let v = std::ptr::read_unaligned(maximum_size);
        if v < 0 {
            return STATUS_INVALID_PARAMETER;
        }
        v as u64
    };
    let backing = if file_handle.is_null() { INVALID_HANDLE_VALUE } else { file_handle };
    let name = object_attributes_name(object_attributes);
    let h = crate::kernel32::file::create_mapping(backing, section_page_protection, max_size, name);
    if h.is_null() {
        let e = teb::last_error();
        log::debug!("NtCreateSection(prot={section_page_protection:#x}, size={max_size:#x}, file={file_handle:p}) failed: win32={e}");
        return match e {
            err::ERROR_INVALID_HANDLE => STATUS_INVALID_HANDLE,
            err::ERROR_ACCESS_DENIED => STATUS_ACCESS_DENIED,
            err::ERROR_NOT_ENOUGH_MEMORY => STATUS_NO_MEMORY,
            _ => STATUS_INVALID_PARAMETER,
        };
    }
    std::ptr::write_unaligned(section_handle, h);
    log::debug!("NtCreateSection(prot={section_page_protection:#x}, size={max_size:#x}) -> {h:p}");
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwCreateSection(
    section_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    maximum_size: *const i64,
    section_page_protection: u32,
    allocation_attributes: u32,
    file_handle: HANDLE,
) -> NTSTATUS {
    NtCreateSection(
        section_handle,
        desired_access,
        object_attributes,
        maximum_size,
        section_page_protection,
        allocation_attributes,
        file_handle,
    )
}

pub unsafe extern "win64" fn NtMapViewOfSection(
    section_handle: HANDLE,
    process_handle: HANDLE,
    base_address: *mut *mut c_void,
    _zero_bits: usize,
    _commit_size: usize,
    section_offset: *mut i64,
    view_size: *mut usize,
    _inherit_disposition: u32,
    allocation_type: u32,
    win32_protect: u32,
) -> NTSTATUS {
    if !process_handle.is_null() && process_handle != crate::handles::CURRENT_PROCESS {
        return STATUS_ACCESS_DENIED;
    }
    if base_address.is_null() || view_size.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(total_size) = crate::kernel32::file::mapping_size(section_handle) else {
        return STATUS_INVALID_HANDLE;
    };
    let req_base = std::ptr::read_unaligned(base_address);
    let offset = if section_offset.is_null() {
        0u64
    } else {
        let v = std::ptr::read_unaligned(section_offset);
        if v < 0 {
            return STATUS_INVALID_PARAMETER;
        }
        v as u64
    };
    if offset > total_size {
        return STATUS_INVALID_PARAMETER;
    }
    let req_size = std::ptr::read_unaligned(view_size);
    let actual_len = if req_size == 0 {
        (total_size - offset) as usize
    } else {
        req_size.min((total_size - offset) as usize)
    };
    let p = crate::kernel32::file::MapViewOfFile3(
        section_handle,
        process_handle,
        req_base,
        offset,
        actual_len,
        allocation_type,
        win32_protect,
        std::ptr::null_mut(),
        0,
    );
    if p.is_null() {
        let e = teb::last_error();
        log::debug!("NtMapViewOfSection({section_handle:p}, base={req_base:p}, off={offset:#x}, len={actual_len:#x}, prot={win32_protect:#x}) failed: win32={e}");
        return match e {
            err::ERROR_INVALID_HANDLE => STATUS_INVALID_HANDLE,
            err::ERROR_ACCESS_DENIED => STATUS_ACCESS_DENIED,
            _ => STATUS_CONFLICTING_ADDRESSES,
        };
    }
    std::ptr::write_unaligned(base_address, p);
    std::ptr::write_unaligned(view_size, (actual_len + 0xFFF) & !0xFFF);
    log::debug!("NtMapViewOfSection({section_handle:p}, off={offset:#x}, len={actual_len:#x}, prot={win32_protect:#x}) -> {p:p}");
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwMapViewOfSection(
    section_handle: HANDLE,
    process_handle: HANDLE,
    base_address: *mut *mut c_void,
    zero_bits: usize,
    commit_size: usize,
    section_offset: *mut i64,
    view_size: *mut usize,
    inherit_disposition: u32,
    allocation_type: u32,
    win32_protect: u32,
) -> NTSTATUS {
    NtMapViewOfSection(
        section_handle,
        process_handle,
        base_address,
        zero_bits,
        commit_size,
        section_offset,
        view_size,
        inherit_disposition,
        allocation_type,
        win32_protect,
    )
}

pub unsafe extern "win64" fn NtUnmapViewOfSection(
    process_handle: HANDLE,
    base_address: *mut c_void,
) -> NTSTATUS {
    if !process_handle.is_null() && process_handle != crate::handles::CURRENT_PROCESS {
        return STATUS_ACCESS_DENIED;
    }
    if crate::kernel32::file::UnmapViewOfFile(base_address) != 0 {
        STATUS_SUCCESS
    } else {
        log::debug!("NtUnmapViewOfSection({base_address:p}) -> STATUS_NOT_MAPPED_VIEW");
        STATUS_NOT_MAPPED_VIEW
    }
}

pub unsafe extern "win64" fn ZwUnmapViewOfSection(
    process_handle: HANDLE,
    base_address: *mut c_void,
) -> NTSTATUS {
    NtUnmapViewOfSection(process_handle, base_address)
}

pub unsafe extern "win64" fn NtCreateThreadEx(
    thread_handle: *mut HANDLE,
    _desired_access: u32,
    _object_attributes: *const c_void,
    process_handle: HANDLE,
    start_routine: *const c_void,
    argument: *mut c_void,
    create_flags: u32,
    _zero_bits: usize,
    stack_size: usize,
    maximum_stack_size: usize,
    _attribute_list: *mut c_void,
) -> NTSTATUS {
    if thread_handle.is_null() || start_routine.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if !process_handle.is_null() && process_handle != crate::handles::CURRENT_PROCESS {
        return STATUS_ACCESS_DENIED;
    }
    // THREAD_CREATE_FLAGS_CREATE_SUSPENDED = 0x1 -> Win32 CREATE_SUSPENDED = 0x4
    let win32_flags = if create_flags & 0x1 != 0 { 0x4 } else { 0 };
    let stack = maximum_stack_size.max(stack_size);
    let entry: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(start_routine);
    let h = crate::kernel32::threads::CreateThread(
        std::ptr::null_mut(),
        stack,
        Some(entry),
        argument,
        win32_flags,
        std::ptr::null_mut(),
    );
    if h.is_null() {
        return STATUS_NO_MEMORY;
    }
    std::ptr::write_unaligned(thread_handle, h);
    log::debug!("NtCreateThreadEx(start={start_routine:p}, flags={create_flags:#x}) -> {h:p}");
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwCreateThreadEx(
    thread_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    process_handle: HANDLE,
    start_routine: *const c_void,
    argument: *mut c_void,
    create_flags: u32,
    zero_bits: usize,
    stack_size: usize,
    maximum_stack_size: usize,
    attribute_list: *mut c_void,
) -> NTSTATUS {
    NtCreateThreadEx(
        thread_handle,
        desired_access,
        object_attributes,
        process_handle,
        start_routine,
        argument,
        create_flags,
        zero_bits,
        stack_size,
        maximum_stack_size,
        attribute_list,
    )
}

pub unsafe extern "win64" fn NtGetNextThread(
    process_handle: HANDLE,
    thread_handle: HANDLE,
    _desired_access: u32,
    _handle_attributes: u32,
    _flags: u32,
    new_thread_handle: *mut HANDLE,
) -> NTSTATUS {
    if new_thread_handle.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if !process_handle.is_null() && process_handle != crate::handles::CURRENT_PROCESS {
        return STATUS_ACCESS_DENIED;
    }
    let threads = crate::thread::all_threads();
    let next = if thread_handle.is_null() {
        threads.first().cloned()
    } else {
        let cur = if thread_handle == crate::handles::CURRENT_THREAD {
            crate::thread::current_thread_object()
        } else {
            crate::handles::get_as::<crate::thread::ThreadObj>(thread_handle)
        };
        let Some(cur) = cur else {
            return STATUS_INVALID_HANDLE;
        };
        let pos = threads.iter().position(|t| std::sync::Arc::ptr_eq(t, &cur));
        pos.and_then(|i| threads.get(i + 1).cloned())
    };
    match next {
        Some(t) => {
            let h = crate::handles::insert(t);
            std::ptr::write_unaligned(new_thread_handle, h);
            STATUS_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(new_thread_handle, std::ptr::null_mut());
            STATUS_NO_MORE_ENTRIES
        }
    }
}

pub unsafe extern "win64" fn ZwGetNextThread(
    process_handle: HANDLE,
    thread_handle: HANDLE,
    desired_access: u32,
    handle_attributes: u32,
    flags: u32,
    new_thread_handle: *mut HANDLE,
) -> NTSTATUS {
    NtGetNextThread(
        process_handle,
        thread_handle,
        desired_access,
        handle_attributes,
        flags,
        new_thread_handle,
    )
}

pub unsafe extern "win64" fn KiUserExceptionDispatcher() {
    stub!("KiUserExceptionDispatcher called directly");
}

pub unsafe extern "win64" fn KiUserApcDispatcher() {
    stub!("KiUserApcDispatcher called directly");
}

pub unsafe extern "win64" fn KiUserCallbackDispatcher() {
    stub!("KiUserCallbackDispatcher called directly");
}

pub unsafe extern "win64" fn RtlDispatchException(
    rec: *mut crate::unwind::ExceptionRecord,
    ctx: *mut crate::unwind::Context,
) -> u8 {
    if rec.is_null() || ctx.is_null() {
        return 0;
    }
    if crate::unwind::dispatch_veh_guest(&mut *rec, &mut *ctx) {
        1
    } else {
        0
    }
}

pub unsafe extern "win64" fn NtRaiseException(
    rec: *mut crate::unwind::ExceptionRecord,
    ctx: *mut crate::unwind::Context,
    first_chance: u8,
) -> NTSTATUS {
    if rec.is_null() || ctx.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    if first_chance != 0 && crate::unwind::dispatch_veh_guest(&mut *rec, &mut *ctx) {
        crate::unwind::RtlRestoreContext(ctx, std::ptr::null_mut());
        return STATUS_SUCCESS;
    }
    crate::unwind::dispatch_exception(&mut *rec, &mut *ctx);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwRaiseException(
    rec: *mut crate::unwind::ExceptionRecord,
    ctx: *mut crate::unwind::Context,
    first_chance: u8,
) -> NTSTATUS {
    NtRaiseException(rec, ctx, first_chance)
}

unsafe fn object_attributes_raw_name(obj_attr: *const c_void) -> Option<String> {
    if obj_attr.is_null() {
        return None;
    }
    let name_ptr = std::ptr::read_unaligned((obj_attr as *const u8).add(16) as *const *const UNICODE_STRING);
    if name_ptr.is_null() {
        return None;
    }
    let us = std::ptr::read_unaligned(name_ptr);
    if us.buf.is_null() || us.len == 0 {
        return None;
    }
    let words = (us.len as usize) / 2;
    let mut tmp = Vec::with_capacity(words);
    for i in 0..words {
        tmp.push(std::ptr::read_unaligned(us.buf.add(i)));
    }
    Some(String::from_utf16_lossy(&tmp))
}

fn nt_path_to_win32(nt: &str) -> String {
    if let Some(win) = crate::path::nt_device_path_to_win32(nt) {
        return win;
    }
    let stripped = nt
        .strip_prefix("\\??\\")
        .or_else(|| nt.strip_prefix("\\DosDevices\\"))
        .or_else(|| nt.strip_prefix("\\\\?\\"))
        .unwrap_or(nt);
    crate::path::nt_device_path_to_win32(stripped).unwrap_or_else(|| stripped.to_string())
}

pub unsafe extern "win64" fn NtCreateFile(
    file_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    io_status_block: *mut u8,
    _allocation_size: *const i64,
    file_attributes: u32,
    share_access: u32,
    create_disposition: u32,
    create_options: u32,
    _ea_buffer: *mut c_void,
    _ea_length: u32,
) -> NTSTATUS {
    if file_handle.is_null() || object_attributes.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(raw) = object_attributes_raw_name(object_attributes) else {
        return STATUS_INVALID_PARAMETER;
    };
    let win_path = nt_path_to_win32(&raw);
    let wpath = to_wide_z(&win_path);
    // FILE_SUPERSEDE=0, FILE_OPEN=1, FILE_CREATE=2, FILE_OPEN_IF=3, FILE_OVERWRITE=4, FILE_OVERWRITE_IF=5
    let win_disp = match create_disposition {
        0 | 5 => 2, // CREATE_ALWAYS
        1 => 3,     // OPEN_EXISTING
        2 => 1,     // CREATE_NEW
        3 => 4,     // OPEN_ALWAYS
        4 => 5,     // TRUNCATE_EXISTING
        _ => 3,
    };
    // FILE_DIRECTORY_FILE = 0x01 -> FILE_FLAG_BACKUP_SEMANTICS = 0x0200_0000
    let flags = file_attributes | if create_options & 0x1 != 0 { 0x0200_0000 } else { 0 };
    let h = crate::kernel32::file::CreateFileW(
        wpath.as_ptr(),
        desired_access,
        share_access,
        std::ptr::null_mut(),
        win_disp,
        flags,
        std::ptr::null_mut(),
    );
    if h == INVALID_HANDLE_VALUE || h.is_null() {
        let e = teb::last_error();
        let st = match e {
            err::ERROR_FILE_NOT_FOUND => 0xC000_0034u32 as i32, // STATUS_OBJECT_NAME_NOT_FOUND
            err::ERROR_PATH_NOT_FOUND => 0xC000_003Au32 as i32, // STATUS_OBJECT_PATH_NOT_FOUND
            err::ERROR_ACCESS_DENIED => STATUS_ACCESS_DENIED,
            err::ERROR_ALREADY_EXISTS | err::ERROR_FILE_EXISTS => 0xC000_0035u32 as i32, // STATUS_OBJECT_NAME_COLLISION
            _ => 0xC000_0001u32 as i32, // STATUS_UNSUCCESSFUL
        };
        if !io_status_block.is_null() {
            std::ptr::write_unaligned(io_status_block as *mut i32, st);
            std::ptr::write_unaligned(io_status_block.add(8) as *mut usize, 0);
        }
        return st;
    }
    std::ptr::write_unaligned(file_handle, h);
    if !io_status_block.is_null() {
        std::ptr::write_unaligned(io_status_block as *mut i32, STATUS_SUCCESS);
        // FILE_OPENED = 1, FILE_CREATED = 2
        let info: usize = if create_disposition == 2 { 2 } else { 1 };
        std::ptr::write_unaligned(io_status_block.add(8) as *mut usize, info);
    }
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwCreateFile(
    file_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    io_status_block: *mut u8,
    allocation_size: *const i64,
    file_attributes: u32,
    share_access: u32,
    create_disposition: u32,
    create_options: u32,
    ea_buffer: *mut c_void,
    ea_length: u32,
) -> NTSTATUS {
    NtCreateFile(
        file_handle,
        desired_access,
        object_attributes,
        io_status_block,
        allocation_size,
        file_attributes,
        share_access,
        create_disposition,
        create_options,
        ea_buffer,
        ea_length,
    )
}

pub unsafe extern "win64" fn NtOpenFile(
    file_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    io_status_block: *mut u8,
    share_access: u32,
    open_options: u32,
) -> NTSTATUS {
    NtCreateFile(
        file_handle,
        desired_access,
        object_attributes,
        io_status_block,
        std::ptr::null(),
        0,
        share_access,
        1, // FILE_OPEN
        open_options,
        std::ptr::null_mut(),
        0,
    )
}

pub unsafe extern "win64" fn ZwOpenFile(
    file_handle: *mut HANDLE,
    desired_access: u32,
    object_attributes: *const c_void,
    io_status_block: *mut u8,
    share_access: u32,
    open_options: u32,
) -> NTSTATUS {
    NtOpenFile(file_handle, desired_access, object_attributes, io_status_block, share_access, open_options)
}

const STATUS_OBJECT_NAME_NOT_FOUND: NTSTATUS = 0xC000_0034u32 as i32;

pub unsafe extern "win64" fn NtQueryAttributesFile(
    object_attributes: *const c_void,
    file_info: *mut u8,
) -> NTSTATUS {
    if object_attributes.is_null() || file_info.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(raw) = object_attributes_raw_name(object_attributes) else {
        return STATUS_INVALID_PARAMETER;
    };
    let win_path = nt_path_to_win32(&raw);
    let mut wide: Vec<u16> = win_path.encode_utf16().collect();
    wide.push(0);
    // WIN32_FILE_ATTRIBUTE_DATA is 36 bytes:
    // +0 attrs(u32), +4 creation(u64), +12 access(u64), +20 write(u64), +28 size_hi(u32), +32 size_lo(u32)
    let mut fad = [0u8; 36];
    if crate::kernel32::file::GetFileAttributesExW(wide.as_ptr(), 0, fad.as_mut_ptr()) == 0 {
        return STATUS_OBJECT_NAME_NOT_FOUND;
    }
    let attrs = u32::from_le_bytes(fad[0..4].try_into().unwrap());
    let creation = u64::from_le_bytes(fad[4..12].try_into().unwrap());
    let access = u64::from_le_bytes(fad[12..20].try_into().unwrap());
    let write = u64::from_le_bytes(fad[20..28].try_into().unwrap());
    std::ptr::write_bytes(file_info, 0, 40);
    std::ptr::write_unaligned(file_info.add(0) as *mut u64, creation);
    std::ptr::write_unaligned(file_info.add(8) as *mut u64, access);
    std::ptr::write_unaligned(file_info.add(16) as *mut u64, write);
    std::ptr::write_unaligned(file_info.add(24) as *mut u64, write);
    std::ptr::write_unaligned(file_info.add(32) as *mut u32, attrs);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwQueryAttributesFile(
    object_attributes: *const c_void,
    file_info: *mut u8,
) -> NTSTATUS {
    NtQueryAttributesFile(object_attributes, file_info)
}

pub unsafe extern "win64" fn NtQueryFullAttributesFile(
    object_attributes: *const c_void,
    file_info: *mut u8,
) -> NTSTATUS {
    if object_attributes.is_null() || file_info.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let Some(raw) = object_attributes_raw_name(object_attributes) else {
        return STATUS_INVALID_PARAMETER;
    };
    let win_path = nt_path_to_win32(&raw);
    let mut wide: Vec<u16> = win_path.encode_utf16().collect();
    wide.push(0);
    let mut fad = [0u8; 36];
    if crate::kernel32::file::GetFileAttributesExW(wide.as_ptr(), 0, fad.as_mut_ptr()) == 0 {
        return STATUS_OBJECT_NAME_NOT_FOUND;
    }
    let attrs = u32::from_le_bytes(fad[0..4].try_into().unwrap());
    let creation = u64::from_le_bytes(fad[4..12].try_into().unwrap());
    let access = u64::from_le_bytes(fad[12..20].try_into().unwrap());
    let write = u64::from_le_bytes(fad[20..28].try_into().unwrap());
    let size_hi = u32::from_le_bytes(fad[28..32].try_into().unwrap()) as u64;
    let size_lo = u32::from_le_bytes(fad[32..36].try_into().unwrap()) as u64;
    let size = (size_hi << 32) | size_lo;
    let alloc = (size + 4095) & !4095;
    std::ptr::write_bytes(file_info, 0, 56);
    std::ptr::write_unaligned(file_info.add(0) as *mut u64, creation);
    std::ptr::write_unaligned(file_info.add(8) as *mut u64, access);
    std::ptr::write_unaligned(file_info.add(16) as *mut u64, write);
    std::ptr::write_unaligned(file_info.add(24) as *mut u64, write);
    std::ptr::write_unaligned(file_info.add(32) as *mut u64, alloc);
    std::ptr::write_unaligned(file_info.add(40) as *mut u64, size);
    std::ptr::write_unaligned(file_info.add(48) as *mut u32, attrs);
    STATUS_SUCCESS
}

pub unsafe extern "win64" fn ZwQueryFullAttributesFile(
    object_attributes: *const c_void,
    file_info: *mut u8,
) -> NTSTATUS {
    NtQueryFullAttributesFile(object_attributes, file_info)
}

pub unsafe extern "win64" fn NtWaitForSingleObject(
    handle: HANDLE,
    alertable: u8,
    timeout: *const i64,
) -> NTSTATUS {
    let dur = nt_timeout_to_duration(timeout);
    if handle == crate::handles::CURRENT_PROCESS {
        if alertable != 0 {
            if crate::handles::alertable_sleep_duration(dur) {
                return 0x0000_00C0; // STATUS_USER_APC
            }
            return 0x0000_0102; // STATUS_TIMEOUT
        }
        match dur {
            None => loop {
                std::thread::park();
            },
            Some(d) if d.is_zero() => {}
            Some(d) => std::thread::sleep(d),
        }
        return 0x0000_0102; // STATUS_TIMEOUT
    }
    let Some(obj) = crate::handles::get(handle) else {
        return STATUS_INVALID_HANDLE;
    };
    let r = crate::handles::wait_for_duration(
        std::slice::from_ref(&obj),
        false,
        dur,
        alertable != 0,
    );
    match r {
        crate::handles::WaitResult::Object(_) => STATUS_SUCCESS,
        crate::handles::WaitResult::Abandoned(_) => 0x0000_0080, // STATUS_ABANDONED_WAIT_0
        crate::handles::WaitResult::Timeout => 0x0000_0102,      // STATUS_TIMEOUT
        crate::handles::WaitResult::IoCompletion => 0x0000_00C0, // STATUS_USER_APC
        crate::handles::WaitResult::Failed(_) => STATUS_INVALID_HANDLE,
    }
}

pub unsafe extern "win64" fn ZwWaitForSingleObject(
    handle: HANDLE,
    alertable: u8,
    timeout: *const i64,
) -> NTSTATUS {
    NtWaitForSingleObject(handle, alertable, timeout)
}

pub unsafe extern "win64" fn NtReadFile(
    file: HANDLE,
    _event: HANDLE,
    _apc_routine: *const c_void,
    _apc_ctx: *mut c_void,
    io_status_block: *mut u8,
    buffer: *mut c_void,
    length: u32,
    byte_offset: *const i64,
    _key: *const u32,
) -> NTSTATUS {
    let mut ov: crate::kernel32::file::OVERLAPPED = std::mem::zeroed();
    let ov_ptr = if !byte_offset.is_null() {
        let off = std::ptr::read_unaligned(byte_offset);
        if off >= 0 {
            ov.offset = off as u32;
            ov.offset_high = (off as u64 >> 32) as u32;
            &mut ov as *mut _
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    };
    let mut got: u32 = 0;
    let ok = crate::kernel32::file::ReadFile(file, buffer, length, &mut got, ov_ptr);
    let st = if ok != 0 {
        STATUS_SUCCESS
    } else if crate::teb::last_error() == err::ERROR_HANDLE_EOF {
        0xC000_0011u32 as i32 // STATUS_END_OF_FILE
    } else {
        STATUS_INVALID_HANDLE
    };
    if !io_status_block.is_null() {
        std::ptr::write_unaligned(io_status_block as *mut i32, st);
        std::ptr::write_unaligned(io_status_block.add(8) as *mut usize, got as usize);
    }
    st
}

pub unsafe extern "win64" fn ZwReadFile(
    file: HANDLE,
    event: HANDLE,
    apc_routine: *const c_void,
    apc_ctx: *mut c_void,
    io_status_block: *mut u8,
    buffer: *mut c_void,
    length: u32,
    byte_offset: *const i64,
    key: *const u32,
) -> NTSTATUS {
    NtReadFile(file, event, apc_routine, apc_ctx, io_status_block, buffer, length, byte_offset, key)
}

pub unsafe extern "win64" fn NtWriteFile(
    file: HANDLE,
    _event: HANDLE,
    _apc_routine: *const c_void,
    _apc_ctx: *mut c_void,
    io_status_block: *mut u8,
    buffer: *const c_void,
    length: u32,
    byte_offset: *const i64,
    _key: *const u32,
) -> NTSTATUS {
    let mut ov: crate::kernel32::file::OVERLAPPED = std::mem::zeroed();
    let ov_ptr = if !byte_offset.is_null() {
        let off = std::ptr::read_unaligned(byte_offset);
        if off >= 0 {
            ov.offset = off as u32;
            ov.offset_high = (off as u64 >> 32) as u32;
            &mut ov as *mut _
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    };
    let mut wrote: u32 = 0;
    let ok = crate::kernel32::file::WriteFile(file, buffer, length, &mut wrote, ov_ptr);
    let st = if ok != 0 { STATUS_SUCCESS } else { STATUS_INVALID_HANDLE };
    if !io_status_block.is_null() {
        std::ptr::write_unaligned(io_status_block as *mut i32, st);
        std::ptr::write_unaligned(io_status_block.add(8) as *mut usize, wrote as usize);
    }
    st
}

pub unsafe extern "win64" fn ZwWriteFile(
    file: HANDLE,
    event: HANDLE,
    apc_routine: *const c_void,
    apc_ctx: *mut c_void,
    io_status_block: *mut u8,
    buffer: *const c_void,
    length: u32,
    byte_offset: *const i64,
    key: *const u32,
) -> NTSTATUS {
    NtWriteFile(file, event, apc_routine, apc_ctx, io_status_block, buffer, length, byte_offset, key)
}

pub unsafe extern "win64" fn LdrLoadDll(
    _path_to_file: LPCWSTR,
    flags: *const u32,
    module_file_name: *const UNICODE_STRING,
    module_handle: *mut HMODULE,
) -> NTSTATUS {
    if module_file_name.is_null() || module_handle.is_null() {
        return STATUS_INVALID_PARAMETER;
    }
    let us = std::ptr::read_unaligned(module_file_name);
    if us.buf.is_null() || us.len == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    let words = (us.len as usize) / 2;
    let mut tmp = Vec::with_capacity(words + 1);
    for i in 0..words {
        tmp.push(std::ptr::read_unaligned(us.buf.add(i)));
    }
    tmp.push(0);
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) };
    let h = crate::kernel32::module::LoadLibraryExW(tmp.as_ptr(), std::ptr::null_mut(), fl);
    std::ptr::write_unaligned(module_handle, h);
    if h.is_null() {
        0xC000_0135u32 as i32 // STATUS_DLL_NOT_FOUND
    } else {
        STATUS_SUCCESS
    }
}

pub unsafe extern "win64" fn LdrUnloadDll(hmod: HMODULE) -> NTSTATUS {
    if crate::kernel32::module::FreeLibrary(hmod) != 0 {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn LdrInitializeThunk(_ctx: *mut c_void, _ntdll_base: *mut c_void) {
    stub!("LdrInitializeThunk called directly");
}

pub unsafe extern "win64" fn RtlUserThreadStart(start: *const c_void, param: *mut c_void) {
    let _ = crate::kernel32::threads::BaseThreadInitThunk(0, start, param);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nt_create_map_unmap_section_roundtrip() {
        unsafe {
            let mut sec: HANDLE = std::ptr::null_mut();
            let max_sz: i64 = 0x10000;
            let st = NtCreateSection(
                &mut sec,
                0xF001F,
                std::ptr::null(),
                &max_sz,
                crate::kernel32::memory::PAGE_EXECUTE_READWRITE,
                0x0800_0000,
                std::ptr::null_mut(),
            );
            assert_eq!(st, STATUS_SUCCESS);
            assert!(!sec.is_null());

            let mut view1: *mut c_void = std::ptr::null_mut();
            let mut view1_sz: usize = 0;
            let st = NtMapViewOfSection(
                sec,
                crate::handles::CURRENT_PROCESS,
                &mut view1,
                0,
                0,
                std::ptr::null_mut(),
                &mut view1_sz,
                2,
                0,
                crate::kernel32::memory::PAGE_READWRITE,
            );
            assert_eq!(st, STATUS_SUCCESS);
            assert!(!view1.is_null());
            assert_eq!(view1_sz, 0x10000);

            let mut view2: *mut c_void = std::ptr::null_mut();
            let mut view2_sz: usize = 0;
            let st = NtMapViewOfSection(
                sec,
                crate::handles::CURRENT_PROCESS,
                &mut view2,
                0,
                0,
                std::ptr::null_mut(),
                &mut view2_sz,
                2,
                0,
                crate::kernel32::memory::PAGE_EXECUTE_READ,
            );
            assert_eq!(st, STATUS_SUCCESS);
            assert!(!view2.is_null());
            assert_ne!(view1, view2);

            // Dual-mapped views share backing memory.
            *(view1 as *mut u32) = 0xDEAD_BEEF;
            assert_eq!(*(view2 as *const u32), 0xDEAD_BEEF);

            assert_eq!(NtUnmapViewOfSection(crate::handles::CURRENT_PROCESS, view1), STATUS_SUCCESS);
            assert_eq!(NtUnmapViewOfSection(crate::handles::CURRENT_PROCESS, view2), STATUS_SUCCESS);
            assert_eq!(NtClose(sec), STATUS_SUCCESS);
        }
    }

    #[test]
    fn nt_get_next_thread_rejects_null_and_foreign_process() {
        unsafe {
            let mut out: HANDLE = std::ptr::null_mut();
            assert_eq!(
                NtGetNextThread(crate::handles::CURRENT_PROCESS, std::ptr::null_mut(), 0, 0, 0, std::ptr::null_mut()),
                STATUS_INVALID_PARAMETER
            );
            assert_eq!(
                NtGetNextThread(0x1234 as HANDLE, std::ptr::null_mut(), 0, 0, 0, &mut out),
                STATUS_ACCESS_DENIED
            );
        }
    }

    #[test]
    fn nt_query_virtual_memory_mapped_filename() {
        crate::path::add_mount("V:\\", std::path::Path::new("/"));
        unsafe {
            // 1. File-backed section reports NT device path and STATUS_BUFFER_OVERFLOW on undersized buffer.
            let tmp = std::env::temp_dir().join(format!("win32_rt_vmname_test_{}.bin", std::process::id()));
            std::fs::write(&tmp, vec![0x5Au8; 0x2000]).unwrap();
            let win_str = format!("V:{}", tmp.to_string_lossy().replace('/', "\\"));
            let win_w = to_wide_z(&win_str);
            let fh = crate::kernel32::file::CreateFileW(
                win_w.as_ptr(),
                0x8000_0000,
                7,
                std::ptr::null_mut(),
                3,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(fh, INVALID_HANDLE_VALUE);
            let mh = crate::kernel32::file::CreateFileMappingW(
                fh,
                std::ptr::null_mut(),
                crate::kernel32::memory::PAGE_READONLY,
                0,
                0,
                std::ptr::null(),
            );
            assert!(!mh.is_null());
            let view = crate::kernel32::file::MapViewOfFile(mh, 4 /* FILE_MAP_READ */, 0, 0, 0);
            assert!(!view.is_null());

            let mut need: usize = 0;
            let mut tiny = [0u8; 16];
            let st_small = NtQueryVirtualMemory(
                crate::handles::CURRENT_PROCESS,
                (view as *const u8).add(0x123) as *const c_void,
                2,
                tiny.as_mut_ptr() as *mut c_void,
                tiny.len(),
                &mut need,
            );
            assert_eq!(st_small, STATUS_BUFFER_OVERFLOW);
            assert!(need > 16);

            let mut buf = vec![0u8; need];
            let mut got: usize = 0;
            let st_ok = NtQueryVirtualMemory(
                crate::handles::CURRENT_PROCESS,
                (view as *const u8).add(0x123) as *const c_void,
                2,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                &mut got,
            );
            assert_eq!(st_ok, STATUS_SUCCESS);
            assert_eq!(got, need);
            let byte_len = std::ptr::read_unaligned(buf.as_ptr() as *const u16) as usize;
            let ptr = std::ptr::read_unaligned(buf.as_ptr().add(8) as *const *const u16);
            let words: Vec<u16> = (0..byte_len / 2).map(|i| std::ptr::read_unaligned(ptr.add(i))).collect();
            let got_path = String::from_utf16_lossy(&words);
            let expected = crate::path::win32_to_nt_device_path(&win_str);
            assert_eq!(got_path, expected);

            // 2. Anonymous pagefile-backed section returns STATUS_FILE_INVALID.
            let anon_mh = crate::kernel32::file::CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                std::ptr::null_mut(),
                crate::kernel32::memory::PAGE_READWRITE,
                0,
                0x2000,
                std::ptr::null(),
            );
            assert!(!anon_mh.is_null());
            let anon_view = crate::kernel32::file::MapViewOfFile(anon_mh, 2, 0, 0, 0);
            assert!(!anon_view.is_null());
            let st_anon = NtQueryVirtualMemory(
                crate::handles::CURRENT_PROCESS,
                anon_view,
                2,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                &mut got,
            );
            assert_eq!(st_anon, STATUS_FILE_INVALID);

            // 3. Private memory returns STATUS_INVALID_ADDRESS.
            let priv_mem = crate::kernel32::memory::VirtualAlloc(
                std::ptr::null_mut(),
                0x2000,
                0x3000,
                crate::kernel32::memory::PAGE_READWRITE,
            );
            assert!(!priv_mem.is_null());
            let st_priv = NtQueryVirtualMemory(
                crate::handles::CURRENT_PROCESS,
                priv_mem,
                2,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                &mut got,
            );
            assert_eq!(st_priv, STATUS_INVALID_ADDRESS);

            crate::kernel32::memory::VirtualFree(priv_mem, 0, 0x8000);
            crate::kernel32::file::UnmapViewOfFile(anon_view);
            crate::kernel32::threads::CloseHandle(anon_mh);
            crate::kernel32::file::UnmapViewOfFile(view);
            crate::kernel32::threads::CloseHandle(mh);
            crate::kernel32::threads::CloseHandle(fh);
            let _ = std::fs::remove_file(&tmp);
        }
    }

    #[test]
    fn nt_delay_execution_sub_ms_and_wait_syscall_status() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::time::{Duration, Instant};

        unsafe {
            assert_eq!(NtDelayExecution(0, std::ptr::null()), STATUS_INVALID_PARAMETER);

            // Sub-millisecond non-zero relative interval (-5000 * 100ns = 500 us) must sleep instead of truncating to 0.
            let sub_ms: i64 = -5_000;
            let t0 = Instant::now();
            assert_eq!(NtDelayExecution(0, &sub_ms), STATUS_SUCCESS);
            assert!(t0.elapsed() >= Duration::from_micros(100), "sub-ms NtDelayExecution truncated to 0");

            // Positive absolute FILETIME timeout on NtWaitForSingleObject (~15 ms in the future).
            let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());
            let abs_deadline = (now_filetime() as i64) + 150_000;
            let t1 = Instant::now();
            let st_abs = NtWaitForSingleObject(ev, 0, &abs_deadline);
            assert_eq!(st_abs, 0x0000_0102); // STATUS_TIMEOUT
            assert!(t1.elapsed() >= Duration::from_millis(5));

            // Direct ntsyscall NT_WAIT_FOR_SINGLE_OBJECT maps statuses cleanly.
            let zero: i64 = 0;
            let st_to = crate::ntsyscall::dispatch_nt_syscall(
                crate::ntsyscall::ssn::NT_WAIT_FOR_SINGLE_OBJECT,
                ev as u64,
                0,
                &zero as *const i64 as u64,
                0,
                0,
            );
            assert_eq!(st_to, 0x0000_0102);

            assert_eq!(crate::kernel32::threads::SetEvent(ev), TRUE);
            let st_ok = crate::ntsyscall::dispatch_nt_syscall(
                crate::ntsyscall::ssn::NT_WAIT_FOR_SINGLE_OBJECT,
                ev as u64,
                0,
                &zero as *const i64 as u64,
                0,
                0,
            );
            assert_eq!(st_ok, STATUS_SUCCESS);

            let st_bad = crate::ntsyscall::dispatch_nt_syscall(
                crate::ntsyscall::ssn::NT_WAIT_FOR_SINGLE_OBJECT,
                0xDEAD_BEEFu64,
                0,
                &zero as *const i64 as u64,
                0,
                0,
            );
            assert_eq!(st_bad, STATUS_INVALID_HANDLE);
            assert_eq!(crate::kernel32::threads::CloseHandle(ev), TRUE);

            // Direct ntsyscall NT_WAIT_FOR_SINGLE_OBJECT honors alertable=1 -> STATUS_USER_APC (0xC0).
            unsafe extern "win64" fn apc_cb(param: usize) {
                (*(param as *const AtomicBool)).store(true, Ordering::Release);
            }
            let apc_ran = std::sync::Arc::new(AtomicBool::new(false));
            let apc_flag = apc_ran.clone();
            let thr = crate::thread::spawn(
                Box::new(move || {
                    let ev2 = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
                    let cur = crate::thread::current_thread_object().unwrap();
                    crate::thread::queue_apc(&cur, apc_cb, std::sync::Arc::as_ptr(&apc_flag) as usize);
                    let wait_50ms: i64 = -500_000;
                    let st = crate::ntsyscall::dispatch_nt_syscall(
                        crate::ntsyscall::ssn::NT_WAIT_FOR_SINGLE_OBJECT,
                        ev2 as u64,
                        1, // alertable
                        &wait_50ms as *const i64 as u64,
                        0,
                        0,
                    );
                    assert_eq!(crate::kernel32::threads::CloseHandle(ev2), TRUE);
                    st as u32
                }),
                64 * 1024,
                false,
                false,
            )
            .expect("spawn alertable ntsyscall test thread");
            let th = crate::handles::insert(thr);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(th, 5_000), WAIT_OBJECT_0);
            let mut exit_st = 0u32;
            assert_eq!(crate::kernel32::threads::GetExitCodeThread(th, &mut exit_st), TRUE);
            assert_eq!(exit_st, 0x0000_00C0); // STATUS_USER_APC
            assert!(apc_ran.load(Ordering::Acquire));
            assert_eq!(crate::kernel32::threads::CloseHandle(th), TRUE);
        }
    }
}
