//! Process-level kernel32 APIs: command line, environment, exit, startup
//! info, pointer encoding, error mode, debugging, FormatMessage, toolhelp,
//! job objects and misc.

use crate::handles::{self, KObject};
use crate::process as pr;
use crate::strings::*;
use crate::teb;
use crate::types::*;
use crate::unwind;
use std::any::Any;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

fn set_err(e: u32) {
    teb::set_last_error(e);
}

// ---------------------------------------------------------------------------
// Last error
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetLastError() -> u32 {
    teb::last_error()
}
pub unsafe extern "win64" fn SetLastError(e: u32) {
    teb::set_last_error(e)
}
pub unsafe extern "win64" fn SetLastErrorEx(e: u32, _t: u32) {
    teb::set_last_error(e)
}
pub unsafe extern "win64" fn RestoreLastError(e: u32) {
    teb::set_last_error(e)
}

// ---------------------------------------------------------------------------
// Command line / environment
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetCommandLineW() -> LPWSTR {
    pr::cmdline_w()
}
pub unsafe extern "win64" fn GetCommandLineA() -> LPSTR {
    pr::cmdline_a()
}

pub unsafe extern "win64" fn GetEnvironmentStringsW() -> LPWSTR {
    pr::env_block_w()
}
pub unsafe extern "win64" fn GetEnvironmentStrings() -> LPSTR {
    pr::env_block_a()
}
pub unsafe extern "win64" fn GetEnvironmentStringsA() -> LPSTR {
    pr::env_block_a()
}
pub unsafe extern "win64" fn FreeEnvironmentStringsW(_p: LPWSTR) -> BOOL {
    // Blocks are generation-cached and owned by crate::process.
    TRUE
}
pub unsafe extern "win64" fn FreeEnvironmentStringsA(_p: LPSTR) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetEnvironmentVariableW(name: LPCWSTR, buf: LPWSTR, size: u32) -> u32 {
    let n = from_wide(name);
    match pr::get_env(&n) {
        Some(v) => {
            let w = to_wide(&v);
            if (w.len() as u32) < size {
                std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
                *buf.add(w.len()) = 0;
                set_err(0);
                w.len() as u32
            } else {
                w.len() as u32 + 1
            }
        }
        None => {
            set_err(err::ERROR_ENVVAR_NOT_FOUND);
            0
        }
    }
}

pub unsafe extern "win64" fn GetEnvironmentVariableA(name: LPCSTR, buf: LPSTR, size: u32) -> u32 {
    let n = from_ansi(name);
    match pr::get_env(&n) {
        Some(v) => {
            if (v.len() as u32) < size {
                write_ansi_buf(&v, buf, size as usize);
                set_err(0);
                v.len() as u32
            } else {
                v.len() as u32 + 1
            }
        }
        None => {
            set_err(err::ERROR_ENVVAR_NOT_FOUND);
            0
        }
    }
}

pub unsafe extern "win64" fn SetEnvironmentVariableW(name: LPCWSTR, value: LPCWSTR) -> BOOL {
    let n = from_wide(name);
    if n.is_empty() || n.contains('=') {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    pr::set_env(&n, from_wide_opt(value).as_deref());
    TRUE
}

pub unsafe extern "win64" fn SetEnvironmentVariableA(name: LPCSTR, value: LPCSTR) -> BOOL {
    let n = from_ansi(name);
    if n.is_empty() || n.contains('=') {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    pr::set_env(&n, from_ansi_opt(value).as_deref());
    TRUE
}

pub fn expand_env(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find('%') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        match after.find('%') {
            Some(j) => {
                let name = &after[..j];
                match pr::get_env(name) {
                    Some(v) if !name.is_empty() => {
                        out.push_str(&v);
                        rest = &after[j + 1..];
                    }
                    _ => {
                        out.push('%');
                        rest = after;
                    }
                }
            }
            None => {
                out.push_str(&rest[i..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

pub unsafe extern "win64" fn ExpandEnvironmentStringsW(src: LPCWSTR, dst: LPWSTR, size: u32) -> u32 {
    let e = expand_env(&from_wide(src));
    let w = to_wide_z(&e);
    if (w.len() as u32) <= size && !dst.is_null() {
        std::ptr::copy_nonoverlapping(w.as_ptr(), dst, w.len());
    }
    w.len() as u32
}

pub unsafe extern "win64" fn ExpandEnvironmentStringsA(src: LPCSTR, dst: LPSTR, size: u32) -> u32 {
    let e = expand_env(&from_ansi(src));
    let n = e.len() as u32 + 1;
    if n <= size && !dst.is_null() {
        write_ansi_buf(&e, dst, size as usize);
    }
    n
}

// ---------------------------------------------------------------------------
// Process identity / exit
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetCurrentProcess() -> HANDLE {
    handles::CURRENT_PROCESS
}
pub unsafe extern "win64" fn GetCurrentProcessId() -> u32 {
    std::process::id()
}
pub unsafe extern "win64" fn GetProcessId(h: HANDLE) -> u32 {
    if h == handles::CURRENT_PROCESS || handles::get_as::<ProcessObj>(h).is_some() {
        std::process::id()
    } else if let Some(c) = handles::get_as::<ChildProcessObj>(h) {
        c.pid
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        0
    }
}

/// Handle to our own process obtained through OpenProcess.
pub struct ProcessObj;
impl KObject for ProcessObj {
    fn type_name(&self) -> &'static str {
        "Process"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Handle to a real child guest process launched by `CreateProcessW` on macOS.
pub struct ChildProcessObj {
    pub pid: u32,
    pub exe_win: String,
    pub exit_code: handles::Guarded<Option<u32>>,
    #[cfg(not(target_os = "ios"))]
    terminated_code: handles::Guarded<Option<u32>>,
    #[cfg(not(target_os = "ios"))]
    os_pid: std::sync::atomic::AtomicI32,
}

impl ChildProcessObj {
    pub fn current_exit_code(&self) -> Option<u32> {
        let g = handles::wait_lock();
        *self.exit_code.get(&g)
    }

    #[cfg(not(target_os = "ios"))]
    fn finish_reaped(&self, guest_code: Option<u32>, fallback_code: u32) {
        self.os_pid.store(0, Ordering::Release);
        {
            let g = handles::wait_lock();
            let slot = self.exit_code.get(&g);
            if slot.is_none() {
                let term_code = *self.terminated_code.get(&g);
                // Prefer an explicit 32-bit guest exit code if the child completed ExitProcess/return;
                // otherwise prefer the 32-bit code passed to TerminateProcess over POSIX signal fallback.
                *slot = Some(guest_code.or(term_code).unwrap_or(fallback_code));
            }
        }
        handles::notify_object_key(handles::obj_key(self), usize::MAX);
    }
}

impl KObject for ChildProcessObj {
    fn type_name(&self) -> &'static str {
        "ChildProcess"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn handles::Waitable> {
        Some(self)
    }
}

impl handles::Waitable for ChildProcessObj {
    fn is_signaled(&self, g: &handles::WaitGuard, _tid: u32) -> bool {
        self.exit_code.get(g).is_some()
    }
    fn acquire(&self, _g: &handles::WaitGuard, _tid: u32) -> bool {
        false
    }
}

/// Companion waitable thread handle returned in `PROCESS_INFORMATION.hThread` for a child process.
pub struct ChildThreadObj {
    pub tid: u32,
    pub process: Arc<ChildProcessObj>,
}

impl KObject for ChildThreadObj {
    fn type_name(&self) -> &'static str {
        "ChildThread"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn handles::Waitable> {
        Some(self)
    }
}

impl handles::Waitable for ChildThreadObj {
    fn is_signaled(&self, g: &handles::WaitGuard, _tid: u32) -> bool {
        self.process.exit_code.get(g).is_some()
    }
    fn acquire(&self, _g: &handles::WaitGuard, _tid: u32) -> bool {
        false
    }
    fn wait_key(&self) -> usize {
        handles::obj_key(&*self.process)
    }
}

fn child_processes() -> &'static std::sync::Mutex<std::collections::HashMap<u32, std::sync::Weak<ChildProcessObj>>> {
    static MAP: OnceLock<std::sync::Mutex<std::collections::HashMap<u32, std::sync::Weak<ChildProcessObj>>>> =
        OnceLock::new();
    MAP.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

pub unsafe extern "win64" fn OpenProcess(_access: u32, inherit: BOOL, pid: u32) -> HANDLE {
    let flags = if inherit != 0 { handles::HANDLE_FLAG_INHERIT } else { 0 };
    if pid == std::process::id() {
        return handles::insert_with_flags(Arc::new(ProcessObj), flags);
    }
    if pid == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let found = {
        let mut m = child_processes().lock().unwrap();
        let obj = m.get(&pid).and_then(|w| w.upgrade());
        if obj.is_none() {
            m.remove(&pid);
        }
        obj
    };
    if let Some(child) = found {
        return handles::insert_with_flags(child, flags);
    }
    log::debug!("OpenProcess({pid}): not our process or known child");
    set_err(err::ERROR_ACCESS_DENIED);
    std::ptr::null_mut()
}

pub fn is_self(h: HANDLE) -> bool {
    h == handles::CURRENT_PROCESS || handles::get_as::<ProcessObj>(h).is_some()
}

pub unsafe extern "win64" fn ExitProcess(code: u32) -> ! {
    log::info!("ExitProcess({code}) called from {}", crate::symbolize(return_address()));
    pr::exit_process(code)
}

pub unsafe extern "win64" fn TerminateProcess(h: HANDLE, code: u32) -> BOOL {
    if is_self(h) {
        log::info!("TerminateProcess(self, {code}) called from {}", crate::symbolize(return_address()));
        pr::exit_process(code)
    }
    if let Some(child) = handles::get_as::<ChildProcessObj>(h) {
        #[cfg(not(target_os = "ios"))]
        {
            // Record the requested full 32-bit exit code before sending SIGKILL so the reaper
            // cannot observe signal death before `terminated_code` is visible.
            let already_exited = {
                let g = handles::wait_lock();
                if child.exit_code.get(&g).is_some() {
                    true
                } else {
                    let tslot = child.terminated_code.get(&g);
                    if tslot.is_none() {
                        *tslot = Some(code);
                    }
                    false
                }
            };
            if already_exited {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            let pid = child.os_pid.swap(0, Ordering::AcqRel);
            if pid > 0 {
                if libc::kill(pid, libc::SIGKILL) != 0 && last_errno() == libc::ESRCH {
                    set_err(err::ERROR_ACCESS_DENIED);
                    return FALSE;
                }
            }
            return TRUE;
        }
        #[cfg(target_os = "ios")]
        {
            let _ = (child, code);
            set_err(err::ERROR_NOT_SUPPORTED);
            return FALSE;
        }
    }
    set_err(err::ERROR_ACCESS_DENIED);
    FALSE
}

pub unsafe extern "win64" fn GetExitCodeProcess(h: HANDLE, code: *mut u32) -> BOOL {
    if code.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if is_self(h) {
        std::ptr::write_unaligned(code, crate::kernel32::threads::STILL_ACTIVE);
        return TRUE;
    }
    if let Some(child) = handles::get_as::<ChildProcessObj>(h) {
        let c = child
            .current_exit_code()
            .unwrap_or(crate::kernel32::threads::STILL_ACTIVE);
        std::ptr::write_unaligned(code, c);
        return TRUE;
    }
    set_err(err::ERROR_INVALID_HANDLE);
    FALSE
}

pub unsafe extern "win64" fn OpenProcessToken(proc_: HANDLE, access: u32, out: *mut HANDLE) -> BOOL {
    crate::advapi32::OpenProcessToken(proc_, access, out)
}

#[inline(always)]
fn return_address() -> usize {
    // Best effort: the caller of the import thunk is on the stack just above us;
    // the loader's trace mode logs precise call sites.
    0
}

pub unsafe extern "win64" fn GetStartupInfoW(si: *mut u8) {
    std::ptr::write_bytes(si, 0, 104);
    (si as *mut u32).write_unaligned(104);
    (si.add(80) as *mut HANDLE).write_unaligned(crate::kernel32::file::std_handle(0));
    (si.add(88) as *mut HANDLE).write_unaligned(crate::kernel32::file::std_handle(1));
    (si.add(96) as *mut HANDLE).write_unaligned(crate::kernel32::file::std_handle(2));
}
pub unsafe extern "win64" fn GetStartupInfoA(si: *mut u8) {
    GetStartupInfoW(si)
}

pub unsafe extern "win64" fn QueryFullProcessImageNameW(h: HANDLE, _flags: u32, buf: LPWSTR, size: *mut u32) -> BOOL {
    if buf.is_null() || size.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let p_owned: String;
    let p: &str = if is_self(h) {
        &pr::config().exe_path
    } else if let Some(child) = handles::get_as::<ChildProcessObj>(h) {
        p_owned = child.exe_win.clone();
        &p_owned
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let n = p.encode_utf16().count() as u32;
    let cap = std::ptr::read_unaligned(size);
    if n + 1 > cap {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_wide_buf(p, buf, cap as usize);
    std::ptr::write_unaligned(size, n);
    TRUE
}

pub unsafe extern "win64" fn QueryFullProcessImageNameA(h: HANDLE, _flags: u32, buf: LPSTR, size: *mut u32) -> BOOL {
    if buf.is_null() || size.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let p_owned: String;
    let p: &str = if is_self(h) {
        &pr::config().exe_path
    } else if let Some(child) = handles::get_as::<ChildProcessObj>(h) {
        p_owned = child.exe_win.clone();
        &p_owned
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let cap = std::ptr::read_unaligned(size);
    if p.len() as u32 + 1 > cap {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_ansi_buf(p, buf, cap as usize);
    std::ptr::write_unaligned(size, p.len() as u32);
    TRUE
}

pub unsafe extern "win64" fn K32GetProcessImageFileNameW(h: HANDLE, buf: LPWSTR, size: u32) -> u32 {
    let mut s = size;
    if QueryFullProcessImageNameW(h, 0, buf, &mut s) != 0 {
        s
    } else {
        0
    }
}

pub unsafe extern "win64" fn ProcessIdToSessionId(_pid: u32, out: *mut u32) -> BOOL {
    *out = 1;
    TRUE
}
pub unsafe extern "win64" fn WTSGetActiveConsoleSessionId() -> u32 {
    1
}

pub unsafe extern "win64" fn GetProcessAffinityMask(_h: HANDLE, proc_mask: *mut usize, sys_mask: *mut usize) -> BOOL {
    let n = pr::num_cpus().min(64);
    let m = if n == 64 { usize::MAX } else { (1usize << n) - 1 };
    if !proc_mask.is_null() {
        *proc_mask = m;
    }
    if !sys_mask.is_null() {
        *sys_mask = m;
    }
    TRUE
}

pub unsafe extern "win64" fn SetProcessAffinityMask(_h: HANDLE, mask: usize) -> BOOL {
    log::debug!("SetProcessAffinityMask({mask:#x}) ignored (macOS has no hard affinity)");
    TRUE
}

static PRIORITY_CLASS: AtomicU32 = AtomicU32::new(0x20); // NORMAL_PRIORITY_CLASS

pub unsafe extern "win64" fn GetPriorityClass(_h: HANDLE) -> u32 {
    PRIORITY_CLASS.load(Ordering::Relaxed)
}
pub unsafe extern "win64" fn SetPriorityClass(_h: HANDLE, c: u32) -> BOOL {
    PRIORITY_CLASS.store(c, Ordering::Relaxed);
    TRUE
}
pub unsafe extern "win64" fn SetProcessPriorityBoost(_h: HANDLE, _d: BOOL) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetProcessWorkingSetSize(_h: HANDLE, _min: usize, _max: usize) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetProcessWorkingSetSizeEx(_h: HANDLE, _min: usize, _max: usize, _f: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn GetProcessWorkingSetSize(_h: HANDLE, min: *mut usize, max: *mut usize) -> BOOL {
    *min = 200 << 10;
    *max = 1380 << 10;
    TRUE
}
pub unsafe extern "win64" fn SetProcessDEPPolicy(_f: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn IsWow64Process(_h: HANDLE, out: *mut BOOL) -> BOOL {
    *out = FALSE;
    TRUE
}
pub unsafe extern "win64" fn IsWow64Process2(_h: HANDLE, proc_m: *mut u16, native: *mut u16) -> BOOL {
    *proc_m = 0; // IMAGE_FILE_MACHINE_UNKNOWN = not WOW64
    if !native.is_null() {
        *native = 0x8664;
    }
    TRUE
}
pub unsafe extern "win64" fn GetProcessVersion(_pid: u32) -> u32 {
    (teb::OS_MAJOR << 16) | teb::OS_MINOR
}
pub unsafe extern "win64" fn GetProcessHandleCount(_h: HANDLE, n: *mut u32) -> BOOL {
    *n = 64;
    TRUE
}
pub unsafe extern "win64" fn SetProcessInformation(_h: HANDLE, class: u32, _i: *const c_void, _n: u32) -> BOOL {
    log::debug!("SetProcessInformation(class={class}) ignored");
    TRUE
}
pub unsafe extern "win64" fn GetProcessInformation(_h: HANDLE, class: u32, info: *mut c_void, n: u32) -> BOOL {
    if info.is_null() || n == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(info as *mut u8, 0, n as usize);
    if class == 0 && n >= 4 {
        // ProcessMemoryPriority -> MEMORY_PRIORITY_NORMAL (5)
        *(info as *mut u32) = 5;
    }
    TRUE
}
pub unsafe extern "win64" fn SetThreadInformation(_h: HANDLE, class: u32, _i: *const c_void, _n: u32) -> BOOL {
    log::debug!("SetThreadInformation(class={class}) ignored");
    TRUE
}
pub unsafe extern "win64" fn GetThreadInformation(_h: HANDLE, class: u32, info: *mut c_void, n: u32) -> BOOL {
    if info.is_null() || n == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(info as *mut u8, 0, n as usize);
    if class == 0 && n >= 4 {
        // ThreadMemoryPriority -> MEMORY_PRIORITY_NORMAL (5)
        *(info as *mut u32) = 5;
    }
    TRUE
}
pub unsafe extern "win64" fn SetProcessMitigationPolicy(_p: u32, _b: *const c_void, _n: usize) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn GetProcessMitigationPolicy(_h: HANDLE, _p: u32, b: *mut c_void, n: usize) -> BOOL {
    std::ptr::write_bytes(b as *mut u8, 0, n);
    TRUE
}
pub unsafe extern "win64" fn SetSearchPathMode(_f: u32) -> BOOL {
    TRUE
}

fn task_memory() -> (u64, u64) {
    // On iOS, Jetsam accounts against `task_vm_info.phys_footprint` (which includes Metal
    // GPU allocations and compressed pages), whereas `resident_size` under-reports by 1-2 GiB.
    #[cfg(target_os = "ios")]
    {
        let fp = crate::d3d12::mtl::process_phys_footprint();
        if fp != 0 {
            static PEAK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let peak = PEAK.fetch_max(fp, std::sync::atomic::Ordering::Relaxed).max(fp);
            return (fp, peak);
        }
    }
    // (resident, peak) via mach task_info(MACH_TASK_BASIC_INFO).
    extern "C" {
        static mach_task_self_: libc::mach_port_t;
    }
    unsafe {
        let mut info: libc::mach_task_basic_info = std::mem::zeroed();
        let mut count = (std::mem::size_of::<libc::mach_task_basic_info>() / std::mem::size_of::<libc::natural_t>()) as u32;
        let r = libc::task_info(
            mach_task_self_,
            libc::MACH_TASK_BASIC_INFO,
            &mut info as *mut _ as libc::task_info_t,
            &mut count,
        );
        if r == 0 {
            (info.resident_size, info.resident_size_max)
        } else {
            (0, 0)
        }
    }
}

pub unsafe extern "win64" fn K32GetProcessMemoryInfo(_h: HANDLE, out: *mut u8, cb: u32) -> BOOL {
    if cb < 72 {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    let (rss, peak) = task_memory();
    std::ptr::write_bytes(out, 0, cb as usize);
    (out as *mut u32).write_unaligned(cb);
    let put = |off: usize, v: u64| (out.add(off) as *mut u64).write_unaligned(v);
    put(8, peak);
    put(16, rss);
    put(56, rss); // PagefileUsage ~ private commit
    put(64, peak);
    if cb >= 80 {
        put(72, rss); // PROCESS_MEMORY_COUNTERS_EX.PrivateUsage
    }
    TRUE
}

pub unsafe extern "win64" fn GetProcessIoCounters(_h: HANDLE, out: *mut u64) -> BOOL {
    std::ptr::write_bytes(out, 0, 6);
    TRUE
}

// ---------------------------------------------------------------------------
// ProcThreadAttributeList & CreateProcessW/A
// ---------------------------------------------------------------------------

const PROC_THREAD_ATTR_MAGIC: u32 = 0x5054_414C; // "PTAL"
const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: usize = 0x0002_0002;
const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: usize = 0x0002_0007;

#[cfg(any(not(target_os = "ios"), test))]
const DEBUG_PROCESS: u32 = 0x0000_0001;
#[cfg(any(not(target_os = "ios"), test))]
const DEBUG_ONLY_THIS_PROCESS: u32 = 0x0000_0002;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_SUSPENDED: u32 = 0x0000_0004;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_UNICODE_ENVIRONMENT: u32 = 0x0000_0400;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_SEPARATE_WOW_VDM: u32 = 0x0000_0800;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_SHARED_WOW_VDM: u32 = 0x0000_1000;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_PROTECTED_PROCESS: u32 = 0x0004_0000;
#[cfg(any(not(target_os = "ios"), test))]
const EXTENDED_STARTUPINFO_PRESENT: u32 = 0x0008_0000;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_SECURE_PROCESS: u32 = 0x0040_0000;
#[cfg(any(not(target_os = "ios"), test))]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(any(not(target_os = "ios"), test))]
const STARTF_USESTDHANDLES: u32 = 0x0000_0100;
#[cfg(any(not(target_os = "ios"), test))]
const ERROR_BAD_EXE_FORMAT: u32 = 193;
#[cfg(any(not(target_os = "ios"), test))]
const ERROR_DIRECTORY: u32 = 267;

#[repr(C)]
#[derive(Clone, Copy)]
struct ProcThreadAttrHeader {
    magic: u32,
    capacity: u32,
    count: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ProcThreadAttrEntry {
    attribute: usize,
    value_ptr: usize,
    value_size: usize,
}

fn attr_list_bytes(count: u32) -> Option<usize> {
    std::mem::size_of::<ProcThreadAttrHeader>()
        .checked_add((count as usize).checked_mul(std::mem::size_of::<ProcThreadAttrEntry>())?)
}

pub unsafe extern "win64" fn InitializeProcThreadAttributeList(
    attr_list: *mut c_void,
    attr_count: u32,
    flags: u32,
    size: *mut usize,
) -> BOOL {
    if flags != 0 || size.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(needed) = attr_list_bytes(attr_count) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let avail = std::ptr::read_unaligned(size);
    std::ptr::write_unaligned(size, needed);
    if attr_list.is_null() || avail < needed {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::write_bytes(attr_list as *mut u8, 0, needed);
    std::ptr::write_unaligned(
        attr_list as *mut ProcThreadAttrHeader,
        ProcThreadAttrHeader {
            magic: PROC_THREAD_ATTR_MAGIC,
            capacity: attr_count,
            count: 0,
            _pad: 0,
        },
    );
    TRUE
}

pub unsafe extern "win64" fn UpdateProcThreadAttribute(
    attr_list: *mut c_void,
    flags: u32,
    attribute: usize,
    value: *mut c_void,
    cb_size: usize,
    previous_value: *mut c_void,
    return_size: *mut usize,
) -> BOOL {
    if flags != 0 || attr_list.is_null() || !previous_value.is_null() || !return_size.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut hdr = std::ptr::read_unaligned(attr_list as *const ProcThreadAttrHeader);
    if hdr.magic != PROC_THREAD_ATTR_MAGIC {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    match attribute {
        PROC_THREAD_ATTRIBUTE_HANDLE_LIST => {
            if value.is_null() || cb_size == 0 || (cb_size % std::mem::size_of::<HANDLE>()) != 0 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
        }
        PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY => {
            if value.is_null() || (cb_size != 8 && cb_size != 16) {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
        }
        _ => {
            set_err(err::ERROR_NOT_SUPPORTED);
            return FALSE;
        }
    }
    let entries_base = (attr_list as *mut u8).add(std::mem::size_of::<ProcThreadAttrHeader>())
        as *mut ProcThreadAttrEntry;
    for i in 0..hdr.count as usize {
        let mut ent = std::ptr::read_unaligned(entries_base.add(i));
        if ent.attribute == attribute {
            ent.value_ptr = value as usize;
            ent.value_size = cb_size;
            std::ptr::write_unaligned(entries_base.add(i), ent);
            return TRUE;
        }
    }
    if hdr.count >= hdr.capacity {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::write_unaligned(
        entries_base.add(hdr.count as usize),
        ProcThreadAttrEntry {
            attribute,
            value_ptr: value as usize,
            value_size: cb_size,
        },
    );
    hdr.count += 1;
    std::ptr::write_unaligned(attr_list as *mut ProcThreadAttrHeader, hdr);
    TRUE
}

pub unsafe extern "win64" fn DeleteProcThreadAttributeList(attr_list: *mut c_void) {
    if attr_list.is_null() {
        return;
    }
    let mut hdr = std::ptr::read_unaligned(attr_list as *const ProcThreadAttrHeader);
    if hdr.magic == PROC_THREAD_ATTR_MAGIC {
        hdr.magic = 0;
        hdr.count = 0;
        std::ptr::write_unaligned(attr_list as *mut ProcThreadAttrHeader, hdr);
    }
}

#[cfg(not(target_os = "ios"))]
unsafe fn extract_handle_list(attr_list: *const c_void) -> Result<Option<Vec<HANDLE>>, u32> {
    if attr_list.is_null() {
        return Ok(None);
    }
    let hdr = std::ptr::read_unaligned(attr_list as *const ProcThreadAttrHeader);
    if hdr.magic != PROC_THREAD_ATTR_MAGIC || hdr.count > hdr.capacity {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    let entries_base = (attr_list as *const u8).add(std::mem::size_of::<ProcThreadAttrHeader>())
        as *const ProcThreadAttrEntry;
    let mut out = None;
    for i in 0..hdr.count as usize {
        let ent = std::ptr::read_unaligned(entries_base.add(i));
        match ent.attribute {
            PROC_THREAD_ATTRIBUTE_HANDLE_LIST => {
                if ent.value_ptr == 0 || ent.value_size == 0 || (ent.value_size % std::mem::size_of::<HANDLE>()) != 0 {
                    return Err(err::ERROR_INVALID_PARAMETER);
                }
                let n = ent.value_size / std::mem::size_of::<HANDLE>();
                let ptr = ent.value_ptr as *const HANDLE;
                let mut hs = Vec::with_capacity(n);
                for j in 0..n {
                    hs.push(std::ptr::read_unaligned(ptr.add(j)));
                }
                out = Some(hs);
            }
            PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY => {}
            _ => return Err(err::ERROR_NOT_SUPPORTED),
        }
    }
    Ok(out)
}

#[cfg(not(target_os = "ios"))]
fn resolve_guest_exe(raw_exe: &str) -> Result<(String, std::path::PathBuf), u32> {
    let mut candidates: Vec<String> = Vec::new();
    let has_sep = raw_exe.contains('\\') || raw_exe.contains('/') || raw_exe.contains(':');
    let add_with_ext = |list: &mut Vec<String>, base: String| {
        let has_ext = base
            .rsplit(['\\', '/'])
            .next()
            .is_some_and(|f| f.contains('.'));
        if !has_ext {
            list.push(format!("{base}.exe"));
        }
        list.push(base);
    };
    if has_sep {
        add_with_ext(&mut candidates, crate::path::absolute(raw_exe));
    } else {
        let cwd = crate::path::cwd();
        add_with_ext(&mut candidates, format!("{}\\{raw_exe}", cwd.trim_end_matches('\\')));
        if let Some(path_env) = pr::get_env("Path") {
            for dir in path_env.split(';').filter(|d| !d.trim().is_empty()) {
                add_with_ext(&mut candidates, format!("{}\\{raw_exe}", dir.trim().trim_end_matches('\\')));
            }
        }
    }
    for win in candidates {
        if let Some(host) = crate::path::to_host(&win) {
            if host.is_file() {
                return Ok((crate::path::normalize(&win), host));
            }
        }
    }
    Err(err::ERROR_FILE_NOT_FOUND)
}

#[cfg(not(target_os = "ios"))]
fn is_pe_file(host: &std::path::Path) -> bool {
    use std::io::Read;
    let Ok(mut f) = std::fs::File::open(host) else {
        return false;
    };
    let mut hdr = [0u8; 64];
    if f.read_exact(&mut hdr).is_err() || &hdr[0..2] != b"MZ" {
        return false;
    }
    let off = u32::from_le_bytes([hdr[0x3C], hdr[0x3D], hdr[0x3E], hdr[0x3F]]) as u64;
    use std::io::Seek;
    if f.seek(std::io::SeekFrom::Start(off)).is_err() {
        return false;
    }
    let mut sig = [0u8; 4];
    f.read_exact(&mut sig).is_ok() && &sig == b"PE\0\0"
}

#[cfg(not(target_os = "ios"))]
unsafe fn parse_env_block(env: *const c_void, unicode: bool) -> Vec<(String, String)> {
    let mut vars = Vec::new();
    if unicode {
        let mut p = env as *const u16;
        loop {
            if std::ptr::read_unaligned(p) == 0 {
                break;
            }
            let mut u16s = Vec::new();
            while std::ptr::read_unaligned(p) != 0 {
                u16s.push(std::ptr::read_unaligned(p));
                p = p.add(1);
            }
            p = p.add(1);
            let s = String::from_utf16_lossy(&u16s);
            if let Some((k, v)) = s.split_once('=') {
                if !k.is_empty() {
                    vars.push((k.to_string(), v.to_string()));
                }
            }
        }
    } else {
        let mut p = env as *const u8;
        loop {
            if *p == 0 {
                break;
            }
            let mut bytes = Vec::new();
            while *p != 0 {
                bytes.push(*p);
                p = p.add(1);
            }
            p = p.add(1);
            let s = String::from_utf8_lossy(&bytes);
            if let Some((k, v)) = s.split_once('=') {
                if !k.is_empty() {
                    vars.push((k.to_string(), v.to_string()));
                }
            }
        }
    }
    vars
}

#[cfg(not(target_os = "ios"))]
fn is_inheritable_handle(h: HANDLE) -> bool {
    let Some(flags) = handles::get_flags(h) else {
        return false;
    };
    if (flags & handles::HANDLE_FLAG_INHERIT) != 0 {
        return true;
    }
    // Standard handles default to inheritable unless a different handle is passed.
    (0..3).any(|i| crate::kernel32::file::std_handle(i) == h)
}

#[cfg(not(target_os = "ios"))]
fn build_child_runner_command(
    runner: &std::path::Path,
    exe_host: &std::path::Path,
    exe_win: &str,
    effective_cmdline: &str,
    child_cwd_win: &str,
    child_env_vars: &[(String, String)],
    headless: bool,
) -> std::process::Command {
    let mut command = std::process::Command::new(runner);
    // Start from an empty host environment and populate ONLY immutable minimal platform
    // necessities captured at startup, followed by explicit runtime-owned protocol values.
    // Guest variables travel strictly inside `PORT_CHILD_ENV` and can never inject host
    // loader/AOT/dyld variables into the native child runner.
    command.env_clear();
    for (k, v) in pr::trusted_child_host_env() {
        command.env(k, v);
    }
    command.arg("--pe").arg(exe_host);
    for (win_prefix, host_dir) in crate::path::list_mounts() {
        command.arg("--mount").arg(format!("{win_prefix}={}", host_dir.display()));
    }
    command.env("PORT_CHILD_STDIO", "1");
    command.env("PORT_CHILD_EXE_WIN", exe_win);
    command.env("PORT_CHILD_CMDLINE", effective_cmdline);
    command.env("PORT_CHILD_CWD", child_cwd_win);
    command.env("PORT_CHILD_ENV", pr::encode_env_list(child_env_vars));
    command.env("PORT_NO_BASE_RETRY", "1");
    if headless {
        command.env("PORT_HEADLESS", "1");
        command.env("AW2_HEADLESS", "1");
    }
    command
}

#[cfg(not(target_os = "ios"))]
fn dup_handle_to_stdio(h: HANDLE, allowed_list: Option<&[HANDLE]>) -> Result<std::process::Stdio, u32> {
    use std::os::unix::io::FromRawFd;
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return Ok(std::process::Stdio::null());
    }
    if let Some(list) = allowed_list {
        if !list.contains(&h) {
            return Ok(std::process::Stdio::null());
        }
    } else if !is_inheritable_handle(h) {
        return Ok(std::process::Stdio::null());
    }
    let Some((raw_fd, _kind)) = crate::kernel32::file::file_fd_and_kind(h) else {
        return Err(err::ERROR_INVALID_HANDLE);
    };
    let dup_fd = unsafe { libc::dup(raw_fd) };
    if dup_fd < 0 {
        return Err(errno_to_win32(last_errno()));
    }
    unsafe {
        let _ = libc::fcntl(dup_fd, libc::F_SETFD, 0);
        Ok(std::process::Stdio::from_raw_fd(dup_fd))
    }
}

pub unsafe extern "win64" fn CreateProcessW(
    app: LPCWSTR,
    cmd: LPWSTR,
    _pa: *mut c_void,
    _ta: *mut c_void,
    inherit: BOOL,
    flags: u32,
    env: *mut c_void,
    cwd: LPCWSTR,
    si: *mut c_void,
    pi: *mut c_void,
) -> BOOL {
    #[cfg(target_os = "ios")]
    {
        let _ = (inherit, flags, env, cwd, si, pi);
        stub!(
            "CreateProcessW unavailable on iOS: app={:?} cmd={:?}",
            from_wide_opt(app),
            from_wide_opt(cmd)
        );
        set_err(err::ERROR_NOT_SUPPORTED);
        return FALSE;
    }

    #[cfg(not(target_os = "ios"))]
    {
        if si.is_null() || pi.is_null() {
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let app_s = from_wide_opt(app).filter(|s| !s.is_empty());
        let cmd_s = from_wide_opt(cmd).filter(|s| !s.is_empty());
        if app_s.is_none() && cmd_s.is_none() {
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }

        let unsupported_flags = DEBUG_PROCESS
            | DEBUG_ONLY_THIS_PROCESS
            | CREATE_SUSPENDED
            | CREATE_SEPARATE_WOW_VDM
            | CREATE_SHARED_WOW_VDM
            | CREATE_PROTECTED_PROCESS
            | CREATE_SECURE_PROCESS;
        if (flags & unsupported_flags) != 0 {
            log::warn!("CreateProcessW: unsupported creation flags {flags:#x}");
            set_err(err::ERROR_NOT_SUPPORTED);
            return FALSE;
        }

        let si_bytes = si as *const u8;
        let cb = (si_bytes as *const u32).read_unaligned();
        let extended = (flags & EXTENDED_STARTUPINFO_PRESENT) != 0;
        let min_cb = if extended { 112 } else { 104 };
        if cb < min_cb {
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let si_flags = (si_bytes.add(60) as *const u32).read_unaligned();
        let h_stdin = (si_bytes.add(80) as *const HANDLE).read_unaligned();
        let h_stdout = (si_bytes.add(88) as *const HANDLE).read_unaligned();
        let h_stderr = (si_bytes.add(96) as *const HANDLE).read_unaligned();

        let handle_list = if extended {
            let attr_ptr = (si_bytes.add(104) as *const *const c_void).read_unaligned();
            match extract_handle_list(attr_ptr) {
                Ok(list) => list,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
        } else {
            None
        };

        if let Some(ref list) = handle_list {
            if inherit == FALSE {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let use_std = (si_flags & STARTF_USESTDHANDLES) != 0;
            for &h in list {
                if h.is_null() || h == INVALID_HANDLE_VALUE || handles::get(h).is_none() {
                    set_err(err::ERROR_INVALID_HANDLE);
                    return FALSE;
                }
                if !is_inheritable_handle(h) {
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
                let is_redirected_std = use_std
                    && (h == h_stdin || h == h_stdout || h == h_stderr)
                    && crate::kernel32::file::file_fd_and_kind(h).is_some();
                if !is_redirected_std {
                    log::warn!(
                        "CreateProcessW: PROC_THREAD_ATTRIBUTE_HANDLE_LIST entry {h:p} cannot be transferred (only redirected stdio handles are supported)"
                    );
                    set_err(err::ERROR_NOT_SUPPORTED);
                    return FALSE;
                }
            }
        }

        let raw_exe = if let Some(ref a) = app_s {
            a.clone()
        } else {
            let parts = pr::split_cmdline(cmd_s.as_deref().unwrap_or(""));
            let Some(first) = parts.into_iter().next().filter(|s| !s.is_empty()) else {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            };
            first
        };

        let (exe_win, exe_host) = match resolve_guest_exe(&raw_exe) {
            Ok(pair) => pair,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        if !is_pe_file(&exe_host) {
            set_err(ERROR_BAD_EXE_FORMAT);
            return FALSE;
        }

        let child_cwd_win = if let Some(c) = from_wide_opt(cwd).filter(|s| !s.is_empty()) {
            let abs = crate::path::absolute(&c);
            let Some(host_cwd) = crate::path::to_host(&abs) else {
                set_err(ERROR_DIRECTORY);
                return FALSE;
            };
            if !host_cwd.is_dir() {
                set_err(ERROR_DIRECTORY);
                return FALSE;
            }
            abs
        } else {
            crate::path::cwd()
        };

        let Some(runner) = pr::runner_exe() else {
            log::warn!("CreateProcessW: port-runner executable not located");
            set_err(err::ERROR_NOT_SUPPORTED);
            return FALSE;
        };

        let effective_cmdline = cmd_s.unwrap_or_else(|| format!("\"{exe_win}\""));
        let child_env_vars = if env.is_null() {
            pr::env_vars()
        } else {
            parse_env_block(env, (flags & CREATE_UNICODE_ENVIRONMENT) != 0)
        };

        let headless = (flags & CREATE_NO_WINDOW) != 0
            || (si_flags & STARTF_USESTDHANDLES) != 0
            || pr::trusted_host_headless();
        let mut command = build_child_runner_command(
            &runner,
            &exe_host,
            &exe_win,
            &effective_cmdline,
            &child_cwd_win,
            &child_env_vars,
            headless,
        );

        if (si_flags & STARTF_USESTDHANDLES) != 0 && inherit != FALSE {
            let allowed = handle_list.as_deref();
            match dup_handle_to_stdio(h_stdin, allowed) {
                Ok(s) => {
                    command.stdin(s);
                }
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
            match dup_handle_to_stdio(h_stdout, allowed) {
                Ok(s) => {
                    command.stdout(s);
                }
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
            match dup_handle_to_stdio(h_stderr, allowed) {
                Ok(s) => {
                    command.stderr(s);
                }
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
        }

        // Dedicated parent-child pipe to report the full 32-bit guest Windows exit code.
        // Both ends start FD_CLOEXEC so unrelated concurrent spawns never inherit either end;
        // `pre_exec` clears FD_CLOEXEC solely on the write end for this child, and the child
        // immediately re-applies FD_CLOEXEC and removes `PORT_EXIT_CODE_FD` at startup.
        struct OwnedFd(i32);
        impl Drop for OwnedFd {
            fn drop(&mut self) {
                if self.0 >= 0 {
                    unsafe {
                        let _ = libc::close(self.0);
                    }
                    self.0 = -1;
                }
            }
        }
        let mut exit_pipe = [-1i32; 2];
        if libc::pipe(exit_pipe.as_mut_ptr()) != 0 {
            set_err(errno_to_win32(last_errno()));
            return FALSE;
        }
        for &fd in &exit_pipe {
            let fl = libc::fcntl(fd, libc::F_GETFD);
            if fl >= 0 {
                let _ = libc::fcntl(fd, libc::F_SETFD, fl | libc::FD_CLOEXEC);
            }
        }
        let exit_read_fd = OwnedFd(exit_pipe[0]);
        let exit_write_fd = OwnedFd(exit_pipe[1]);
        let child_exit_wfd = exit_write_fd.0;
        command.env("PORT_EXIT_CODE_FD", child_exit_wfd.to_string());
        {
            use std::os::unix::process::CommandExt;
            command.pre_exec(move || {
                let fl = libc::fcntl(child_exit_wfd, libc::F_GETFD);
                if fl < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::fcntl(child_exit_wfd, libc::F_SETFD, fl & !libc::FD_CLOEXEC) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }

        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(e) => {
                set_err(e.raw_os_error().map(errno_to_win32).unwrap_or(err::ERROR_ACCESS_DENIED));
                return FALSE;
            }
        };
        // Close parent's write end immediately so EOF arrives as soon as the child exits.
        drop(exit_write_fd);

        let pid = child.id();
        let proc_obj = Arc::new(ChildProcessObj {
            pid,
            exe_win,
            exit_code: handles::Guarded::new(None),
            terminated_code: handles::Guarded::new(None),
            os_pid: std::sync::atomic::AtomicI32::new(pid as i32),
        });
        child_processes()
            .lock()
            .unwrap()
            .insert(pid, Arc::downgrade(&proc_obj));
        let thread_obj = Arc::new(ChildThreadObj {
            tid: pid,
            process: Arc::clone(&proc_obj),
        });

        let proc_for_reaper = Arc::clone(&proc_obj);
        let _ = std::thread::Builder::new()
            .name(format!("win32-child-reap-{pid}"))
            .spawn(move || {
                use std::os::unix::process::ExitStatusExt;
                let fallback_code = match child.wait() {
                    Ok(status) => {
                        if let Some(c) = status.code() {
                            c as u32
                        } else if let Some(sig) = status.signal() {
                            0xC000_0000u32 | (sig as u32)
                        } else {
                            0
                        }
                    }
                    Err(_) => 1,
                };
                // Read the full 32-bit Windows exit code written by `exit_process` (if any).
                let mut code_buf = [0u8; 4];
                let mut got = 0usize;
                while got < 4 {
                    let r = libc::read(
                        exit_read_fd.0,
                        code_buf.as_mut_ptr().add(got) as *mut c_void,
                        4 - got,
                    );
                    if r < 0 {
                        if last_errno() == libc::EINTR {
                            continue;
                        }
                        break;
                    }
                    if r == 0 {
                        break;
                    }
                    got += r as usize;
                }
                drop(exit_read_fd);
                let guest_code = (got == 4).then(|| u32::from_le_bytes(code_buf));
                proc_for_reaper.finish_reaped(guest_code, fallback_code);
            });

        let h_proc = handles::insert(proc_obj);
        let h_thread = handles::insert(thread_obj);
        let pi_bytes = pi as *mut u8;
        std::ptr::write_unaligned(pi_bytes as *mut HANDLE, h_proc);
        std::ptr::write_unaligned(pi_bytes.add(8) as *mut HANDLE, h_thread);
        std::ptr::write_unaligned(pi_bytes.add(16) as *mut u32, pid);
        std::ptr::write_unaligned(pi_bytes.add(20) as *mut u32, pid);
        TRUE
    }
}

pub unsafe extern "win64" fn CreateProcessA(
    app: LPCSTR,
    cmd: LPSTR,
    pa: *mut c_void,
    ta: *mut c_void,
    inherit: BOOL,
    flags: u32,
    env: *mut c_void,
    cwd: LPCSTR,
    si: *mut c_void,
    pi: *mut c_void,
) -> BOOL {
    let w_app = from_ansi_opt(app).map(|s| to_wide_z(&s));
    let mut w_cmd = from_ansi_opt(cmd).map(|s| to_wide_z(&s));
    let w_cwd = from_ansi_opt(cwd).map(|s| to_wide_z(&s));
    CreateProcessW(
        w_app.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        w_cmd.as_mut().map_or(std::ptr::null_mut(), |v| v.as_mut_ptr()),
        pa,
        ta,
        inherit,
        flags,
        env,
        w_cwd.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        si,
        pi,
    )
}

// ---------------------------------------------------------------------------
// Job objects (accepted, no effect)
// ---------------------------------------------------------------------------

pub struct JobObj;
impl KObject for JobObj {
    fn type_name(&self) -> &'static str {
        "Job"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub unsafe extern "win64" fn CreateJobObjectW(_sec: *mut c_void, _name: LPCWSTR) -> HANDLE {
    handles::insert(Arc::new(JobObj))
}
pub unsafe extern "win64" fn CreateJobObjectA(_sec: *mut c_void, _name: LPCSTR) -> HANDLE {
    handles::insert(Arc::new(JobObj))
}
pub unsafe extern "win64" fn AssignProcessToJobObject(_job: HANDLE, _p: HANDLE) -> BOOL {
    stub!("job objects have no effect");
    TRUE
}
pub unsafe extern "win64" fn SetInformationJobObject(_job: HANDLE, class: u32, _i: *const c_void, _n: u32) -> BOOL {
    stub!("SetInformationJobObject class {class} ignored");
    TRUE
}
pub unsafe extern "win64" fn IsProcessInJob(_p: HANDLE, _job: HANDLE, out: *mut BOOL) -> BOOL {
    *out = FALSE;
    TRUE
}

// ---------------------------------------------------------------------------
// Toolhelp (our own process, live threads, and loaded modules)
// ---------------------------------------------------------------------------

const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
const TH32CS_SNAPTHREAD: u32 = 0x0000_0004;
const TH32CS_SNAPMODULE: u32 = 0x0000_0008;
const TH32CS_SNAPMODULE32: u32 = 0x0000_0010;

pub struct SnapshotObj {
    proc_pos: std::sync::Mutex<usize>,
    thread_pos: std::sync::Mutex<usize>,
    mod_pos: std::sync::Mutex<usize>,
    flags: u32,
    threads: Vec<(u32, i32)>,
    modules: Vec<(usize, u32, String)>,
}
impl KObject for SnapshotObj {
    fn type_name(&self) -> &'static str {
        "Snapshot"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub unsafe extern "win64" fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> HANDLE {
    let my_pid = std::process::id();
    let include_mine = pid == 0 || pid == my_pid;

    let threads = if (flags & TH32CS_SNAPTHREAD) != 0 {
        crate::thread::all_threads()
            .into_iter()
            .filter_map(|t| {
                let tid = t.tid.load(Ordering::Relaxed) as u32;
                if tid != 0 && t.exit_code().is_none() {
                    Some((tid, 8 + t.priority.load(Ordering::Relaxed)))
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    let modules = if include_mine && (flags & (TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32)) != 0 {
        crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default()
    } else {
        Vec::new()
    };

    handles::insert(Arc::new(SnapshotObj {
        proc_pos: std::sync::Mutex::new(0),
        thread_pos: std::sync::Mutex::new(0),
        mod_pos: std::sync::Mutex::new(0),
        flags,
        threads,
        modules,
    }))
}

fn exe_basename() -> String {
    let p = &pr::config().exe_path;
    p.rsplit('\\').next().unwrap_or(p).to_string()
}

unsafe fn process_entry(h: HANDLE, out: *mut u8, first: bool, wide: bool) -> BOOL {
    let Some(s) = handles::get_as::<SnapshotObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let mut pos = s.proc_pos.lock().unwrap();
    if first {
        *pos = 0;
    }
    if (s.flags & TH32CS_SNAPPROCESS) == 0 || *pos >= 1 {
        set_err(err::ERROR_NO_MORE_FILES);
        return FALSE;
    }
    *pos += 1;
    let size = if wide { 568 } else { 304 };
    std::ptr::write_bytes(out.add(4), 0, size - 4);
    (out.add(8) as *mut u32).write_unaligned(std::process::id());
    (out.add(28) as *mut u32).write_unaligned(crate::thread::all_tebs().len() as u32);
    (out.add(36) as *mut i32).write_unaligned(8);
    if wide {
        write_wide_trunc(&exe_basename(), out.add(44) as *mut u16, 260);
    } else {
        write_ansi_trunc(&exe_basename(), out.add(44), 260);
    }
    TRUE
}

pub unsafe extern "win64" fn Process32FirstW(h: HANDLE, out: *mut u8) -> BOOL {
    process_entry(h, out, true, true)
}
pub unsafe extern "win64" fn Process32NextW(h: HANDLE, out: *mut u8) -> BOOL {
    process_entry(h, out, false, true)
}
pub unsafe extern "win64" fn Process32First(h: HANDLE, out: *mut u8) -> BOOL {
    process_entry(h, out, true, false)
}
pub unsafe extern "win64" fn Process32Next(h: HANDLE, out: *mut u8) -> BOOL {
    process_entry(h, out, false, false)
}

unsafe fn thread_entry(h: HANDLE, out: *mut u8, first: bool) -> BOOL {
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(s) = handles::get_as::<SnapshotObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let cb = std::ptr::read_unaligned(out as *const u32);
    if cb < 28 {
        set_err(24); // ERROR_BAD_LENGTH
        return FALSE;
    }
    let mut pos = s.thread_pos.lock().unwrap();
    if first {
        *pos = 0;
    }
    let Some(&(tid, base_pri)) = s.threads.get(*pos) else {
        set_err(err::ERROR_NO_MORE_FILES);
        return FALSE;
    };
    *pos += 1;
    // THREADENTRY32 (28 bytes):
    // +0x00 dwSize, +0x04 cntUsage, +0x08 th32ThreadID, +0x0C th32OwnerProcessID,
    // +0x10 tpBasePri, +0x14 tpDeltaPri, +0x18 dwFlags
    std::ptr::write_unaligned(out as *mut u32, 28u32);
    std::ptr::write_unaligned(out.add(0x04) as *mut u32, 0u32);
    std::ptr::write_unaligned(out.add(0x08) as *mut u32, tid);
    std::ptr::write_unaligned(out.add(0x0C) as *mut u32, std::process::id());
    std::ptr::write_unaligned(out.add(0x10) as *mut i32, base_pri);
    std::ptr::write_unaligned(out.add(0x14) as *mut i32, 0i32);
    std::ptr::write_unaligned(out.add(0x18) as *mut u32, 0u32);
    TRUE
}

pub unsafe extern "win64" fn Thread32First(h: HANDLE, out: *mut u8) -> BOOL {
    thread_entry(h, out, true)
}
pub unsafe extern "win64" fn Thread32Next(h: HANDLE, out: *mut u8) -> BOOL {
    thread_entry(h, out, false)
}

unsafe fn module_entry(h: HANDLE, out: *mut u8, first: bool, wide: bool) -> BOOL {
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(s) = handles::get_as::<SnapshotObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let expected: u32 = if wide { 1080 } else { 568 };
    let cb = std::ptr::read_unaligned(out as *const u32);
    if cb < expected {
        set_err(24); // ERROR_BAD_LENGTH
        return FALSE;
    }
    let mut pos = s.mod_pos.lock().unwrap();
    if first {
        *pos = 0;
    }
    let Some((base, size, path)) = s.modules.get(*pos) else {
        set_err(err::ERROR_NO_MORE_FILES);
        return FALSE;
    };
    *pos += 1;
    std::ptr::write_bytes(out.add(4), 0, (expected as usize) - 4);
    // Win64 MODULEENTRY32 / MODULEENTRY32W:
    // +0x00 dwSize, +0x04 th32ModuleID, +0x08 th32ProcessID,
    // +0x0C GlblcntUsage, +0x10 ProccntUsage, +0x18 modBaseAddr,
    // +0x20 modBaseSize, +0x28 hModule, +0x30 szModule[256], +0x130/+0x230 szExePath[260]
    std::ptr::write_unaligned(out as *mut u32, expected);
    std::ptr::write_unaligned(out.add(0x04) as *mut u32, 1u32);
    std::ptr::write_unaligned(out.add(0x08) as *mut u32, std::process::id());
    std::ptr::write_unaligned(out.add(0x0C) as *mut u32, 0xFFFFu32);
    std::ptr::write_unaligned(out.add(0x10) as *mut u32, 0xFFFFu32);
    std::ptr::write_unaligned(out.add(0x18) as *mut usize, *base);
    std::ptr::write_unaligned(out.add(0x20) as *mut u32, *size);
    std::ptr::write_unaligned(out.add(0x28) as *mut usize, *base);
    let mod_name = path.rsplit(['\\', '/']).next().unwrap_or(path);
    if wide {
        write_wide_trunc(mod_name, out.add(0x30) as *mut u16, 256);
        write_wide_trunc(path, out.add(0x230) as *mut u16, 260);
    } else {
        write_ansi_trunc(mod_name, out.add(0x30), 256);
        write_ansi_trunc(path, out.add(0x130), 260);
    }
    TRUE
}

pub unsafe extern "win64" fn Module32FirstW(h: HANDLE, out: *mut u8) -> BOOL {
    module_entry(h, out, true, true)
}
pub unsafe extern "win64" fn Module32NextW(h: HANDLE, out: *mut u8) -> BOOL {
    module_entry(h, out, false, true)
}
pub unsafe extern "win64" fn Module32First(h: HANDLE, out: *mut u8) -> BOOL {
    module_entry(h, out, true, false)
}
pub unsafe extern "win64" fn Module32Next(h: HANDLE, out: *mut u8) -> BOOL {
    module_entry(h, out, false, false)
}

pub unsafe extern "win64" fn CorExitProcess(code: u32) -> ! {
    ExitProcess(code)
}

pub unsafe extern "win64" fn AppPolicyGetProcessTerminationMethod(_token: HANDLE, policy: *mut u32) -> i32 {
    if !policy.is_null() {
        *policy = 0; // AppPolicyProcessTerminationMethod_ExitProcess
    }
    0
}

pub unsafe extern "win64" fn AppPolicyGetThreadInitializationType(_token: HANDLE, policy: *mut u32) -> i32 {
    if !policy.is_null() {
        *policy = 0; // AppPolicyThreadInitializationType_None
    }
    0
}

pub unsafe extern "win64" fn AppPolicyGetShowDeveloperDiagnostic(_token: HANDLE, policy: *mut u32) -> i32 {
    if !policy.is_null() {
        *policy = 0; // AppPolicyShowDeveloperDiagnostic_None
    }
    0
}

pub unsafe extern "win64" fn AppPolicyGetWindowingModel(_token: HANDLE, policy: *mut u32) -> i32 {
    if !policy.is_null() {
        *policy = 1; // AppPolicyWindowingModel_ClassicDesktop
    }
    0
}

// ---------------------------------------------------------------------------
// Pointer encoding
// ---------------------------------------------------------------------------

fn process_cookie() -> usize {
    static C: OnceLock<usize> = OnceLock::new();
    *C.get_or_init(|| {
        let mut v: u64 = 0;
        unsafe { libc::arc4random_buf(&mut v as *mut u64 as *mut c_void, 8) };
        v as usize
    })
}

pub fn encode_pointer(p: usize) -> usize {
    let c = process_cookie();
    (p ^ c).rotate_right((c & 0x3F) as u32)
}
pub fn decode_pointer(p: usize) -> usize {
    let c = process_cookie();
    p.rotate_left((c & 0x3F) as u32) ^ c
}

pub unsafe extern "win64" fn EncodePointer(p: *mut c_void) -> *mut c_void {
    encode_pointer(p as usize) as *mut c_void
}
pub unsafe extern "win64" fn DecodePointer(p: *mut c_void) -> *mut c_void {
    decode_pointer(p as usize) as *mut c_void
}
pub unsafe extern "win64" fn EncodeSystemPointer(p: *mut c_void) -> *mut c_void {
    encode_pointer(p as usize) as *mut c_void
}
pub unsafe extern "win64" fn DecodeSystemPointer(p: *mut c_void) -> *mut c_void {
    decode_pointer(p as usize) as *mut c_void
}

// ---------------------------------------------------------------------------
// Debugging / exceptions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn OutputDebugStringA(s: LPCSTR) {
    if !s.is_null() {
        log::info!(target: "debugstr", "{}", from_ansi(s).trim_end());
    }
}
pub unsafe extern "win64" fn OutputDebugStringW(s: LPCWSTR) {
    if !s.is_null() {
        log::info!(target: "debugstr", "{}", from_wide(s).trim_end());
    }
}

pub unsafe extern "win64" fn IsDebuggerPresent() -> BOOL {
    FALSE
}
pub unsafe extern "win64" fn CheckRemoteDebuggerPresent(_h: HANDLE, out: *mut BOOL) -> BOOL {
    *out = FALSE;
    TRUE
}

pub unsafe extern "win64" fn DebugBreak() {
    log::error!("DebugBreak() called; raising SIGTRAP");
    #[cfg(target_arch = "x86_64")]
    std::arch::asm!("int3");
    #[cfg(target_arch = "aarch64")]
    std::arch::asm!("brk #0xf000");
}

pub unsafe extern "win64" fn SetUnhandledExceptionFilter(f: *mut c_void) -> *mut c_void {
    unwind::UNHANDLED_FILTER.swap(f as usize, Ordering::SeqCst) as *mut c_void
}

/// Called by CRT code (e.g. `__report_gsfailure`, `abort`) with exception pointers.
pub unsafe extern "win64" fn UnhandledExceptionFilter(ptrs: *mut [*mut c_void; 2]) -> i32 {
    if !ptrs.is_null() {
        let rec = (*ptrs)[0] as *const unwind::ExceptionRecord;
        if !rec.is_null() {
            log::error!("UnhandledExceptionFilter: {}", unwind::describe_exception(&*rec));
        }
        let ctx = (*ptrs)[1] as *const unwind::Context;
        if !ctx.is_null() {
            unwind::log_backtrace(&*ctx);
        }
    }
    let f = unwind::UNHANDLED_FILTER.load(Ordering::SeqCst);
    if f != 0 && std::env::var_os("AW2_SKIP_UNHANDLED_FILTER").is_none() {
        let f: unsafe extern "win64" fn(*mut [*mut c_void; 2]) -> i32 = std::mem::transmute(f);
        return f(ptrs);
    }
    1 // EXCEPTION_EXECUTE_HANDLER
}

static ERROR_MODE: AtomicU32 = AtomicU32::new(0);

pub unsafe extern "win64" fn SetErrorMode(m: u32) -> u32 {
    ERROR_MODE.swap(m, Ordering::Relaxed)
}
pub unsafe extern "win64" fn GetErrorMode() -> u32 {
    ERROR_MODE.load(Ordering::Relaxed)
}
pub unsafe extern "win64" fn SetThreadErrorMode(m: u32, old: *mut u32) -> BOOL {
    let o = ERROR_MODE.swap(m, Ordering::Relaxed);
    if !old.is_null() {
        *old = o;
    }
    TRUE
}
pub unsafe extern "win64" fn GetThreadErrorMode() -> u32 {
    ERROR_MODE.load(Ordering::Relaxed)
}

pub unsafe extern "win64" fn FatalAppExitA(_a: u32, msg: LPCSTR) {
    log::error!("FatalAppExit: {}", from_ansi(msg));
    pr::exit_process(255)
}
pub unsafe extern "win64" fn FatalAppExitW(_a: u32, msg: LPCWSTR) {
    log::error!("FatalAppExit: {}", from_wide(msg));
    pr::exit_process(255)
}

pub unsafe extern "win64" fn Beep(_f: u32, _d: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn MulDiv(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let r = a as i64 * b as i64;
    // Round half away from zero, like Windows.
    let q = if (r < 0) != (c < 0) { (r - (c as i64).abs() / 2 * (c as i64).signum()) / c as i64 } else { (r + c as i64 / 2) / c as i64 };
    if q > i32::MAX as i64 || q < i32::MIN as i64 {
        -1
    } else {
        q as i32
    }
}

// ---------------------------------------------------------------------------
// FormatMessage
// ---------------------------------------------------------------------------

const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x100;
const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x200;
const FORMAT_MESSAGE_FROM_STRING: u32 = 0x400;
const FORMAT_MESSAGE_FROM_HMODULE: u32 = 0x800;
const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x1000;
const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 0x2000;

pub fn system_message(code: u32) -> Option<&'static str> {
    let code = if code & 0xFFFF_0000 == 0x8007_0000 { code & 0xFFFF } else { code };
    Some(match code {
        0 => "The operation completed successfully.",
        1 => "Incorrect function.",
        2 => "The system cannot find the file specified.",
        3 => "The system cannot find the path specified.",
        4 => "The system cannot open the file.",
        5 => "Access is denied.",
        6 => "The handle is invalid.",
        8 => "Not enough memory resources are available to process this command.",
        13 => "The data is invalid.",
        14 => "Not enough memory resources are available to complete this operation.",
        18 => "There are no more files.",
        32 => "The process cannot access the file because it is being used by another process.",
        38 => "Reached the end of the file.",
        50 => "The request is not supported.",
        80 => "The file exists.",
        87 => "The parameter is incorrect.",
        109 => "The pipe has been ended.",
        112 => "There is not enough space on the disk.",
        120 => "This function is not supported on this system.",
        122 => "The data area passed to a system call is too small.",
        123 => "The filename, directory name, or volume label syntax is incorrect.",
        126 => "The specified module could not be found.",
        127 => "The specified procedure could not be found.",
        183 => "Cannot create a file when that file already exists.",
        203 => "The system could not find the environment option that was entered.",
        234 => "More data is available.",
        259 => "No more data is available.",
        487 => "Attempt to access invalid address.",
        995 => "The I/O operation has been aborted because of either a thread exit or an application request.",
        996 => "Overlapped I/O event is not in a signaled state.",
        997 => "Overlapped I/O operation is in progress.",
        998 => "Invalid access to memory location.",
        1114 => "A dynamic link library (DLL) initialization routine failed.",
        1168 => "Element not found.",
        1460 => "This operation returned because the timeout period expired.",
        _ => return None,
    })
}

/// Expand `%1`..`%99`, `%n`, `%%`, `%0` etc. `arg(i)` returns argument *i* (1-based) as raw u64.
unsafe fn format_inserts(fmt: &str, ignore: bool, wide_args: bool, arg: &mut dyn FnMut(usize) -> u64) -> String {
    if ignore {
        return fmt.to_string();
    }
    let c: Vec<char> = fmt.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        if c[i] != '%' || i + 1 >= c.len() {
            out.push(c[i]);
            i += 1;
            continue;
        }
        let n = c[i + 1];
        match n {
            '0' => break,
            'n' => {
                out.push_str("\r\n");
                i += 2;
            }
            '%' | '.' | '!' | ' ' => {
                out.push(n);
                i += 2;
            }
            't' => {
                out.push('\t');
                i += 2;
            }
            d if d.is_ascii_digit() => {
                let mut j = i + 1;
                let mut idx = 0usize;
                while j < c.len() && c[j].is_ascii_digit() && j < i + 3 {
                    idx = idx * 10 + c[j].to_digit(10).unwrap() as usize;
                    j += 1;
                }
                let mut spec = String::from("s");
                if j < c.len() && c[j] == '!' {
                    if let Some(end) = c[j + 1..].iter().position(|&x| x == '!') {
                        spec = c[j + 1..j + 1 + end].iter().collect();
                        j += end + 2;
                    }
                }
                let v = arg(idx);
                let conv = spec.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '-' || ch == '.' || ch == '#');
                let conv = conv.trim_start_matches(['l', 'h', 'I', '6', '4', 'w']);
                match conv.chars().next().unwrap_or('s') {
                    's' if wide_args => out.push_str(&from_wide(v as *const u16)),
                    's' => out.push_str(&from_ansi(v as *const u8)),
                    'S' if wide_args => out.push_str(&from_ansi(v as *const u8)),
                    'S' => out.push_str(&from_wide(v as *const u16)),
                    'd' | 'i' => out.push_str(&(v as i32).to_string()),
                    'u' => out.push_str(&(v as u32).to_string()),
                    'x' => out.push_str(&format!("{:x}", v as u32)),
                    'X' => out.push_str(&format!("{:X}", v as u32)),
                    'p' => out.push_str(&format!("{:016X}", v)),
                    'c' => out.push(char::from_u32(v as u32).unwrap_or('?')),
                    _ => out.push_str(&format!("{v}")),
                }
                i = j;
            }
            other => {
                out.push(other);
                i += 2;
            }
        }
    }
    out
}

unsafe fn format_message(flags: u32, source: *const c_void, id: u32, args: *const u64, wide: bool) -> Result<String, u32> {
    let fmt: String = if flags & FORMAT_MESSAGE_FROM_STRING != 0 {
        if wide {
            from_wide(source as *const u16)
        } else {
            from_ansi(source as *const u8)
        }
    } else if flags & FORMAT_MESSAGE_FROM_SYSTEM != 0 {
        match system_message(id) {
            Some(m) => format!("{m}\r\n"),
            None if flags & FORMAT_MESSAGE_FROM_HMODULE != 0 => {
                stub!("message tables in modules are not supported (id {id:#x})");
                return Err(317);
            }
            None => {
                log::debug!("FormatMessage: no system text for {id:#x}");
                return Err(317); // ERROR_MR_MID_NOT_FOUND
            }
        }
    } else {
        stub!("FormatMessage from module message table (id {id:#x})");
        return Err(317);
    };
    let array = flags & FORMAT_MESSAGE_ARGUMENT_ARRAY != 0;
    // Non-array args are a `va_list*`; on Win64 a va_list is a pointer to 8-byte slots.
    let base: *const u64 = if args.is_null() {
        std::ptr::null()
    } else if array {
        args
    } else {
        *(args as *const *const u64)
    };
    let mut get = |i: usize| -> u64 {
        if base.is_null() || i == 0 {
            0
        } else {
            *base.add(i - 1)
        }
    };
    let mut s = format_inserts(&fmt, flags & FORMAT_MESSAGE_IGNORE_INSERTS != 0 || base.is_null(), wide, &mut get);
    let width = flags & 0xFF;
    if width == 0xFF {
        s = s.replace("\r\n", " ");
    }
    Ok(s)
}

pub unsafe extern "win64" fn FormatMessageW(flags: u32, source: *const c_void, id: u32, _lang: u32, buf: LPWSTR, size: u32, args: *const u64) -> u32 {
    let s = match format_message(flags, source, id, args, true) {
        Ok(s) => s,
        Err(e) => {
            set_err(e);
            return 0;
        }
    };
    let w = to_wide_z(&s);
    if flags & FORMAT_MESSAGE_ALLOCATE_BUFFER != 0 {
        let p = crate::heap::alloc(w.len().max(size as usize) * 2, true) as *mut u16;
        std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
        *(buf as *mut *mut u16) = p;
        return w.len() as u32 - 1;
    }
    if w.len() > size as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    w.len() as u32 - 1
}

pub unsafe extern "win64" fn FormatMessageA(flags: u32, source: *const c_void, id: u32, _lang: u32, buf: LPSTR, size: u32, args: *const u64) -> u32 {
    let s = match format_message(flags, source, id, args, false) {
        Ok(s) => s,
        Err(e) => {
            set_err(e);
            return 0;
        }
    };
    let n = s.len();
    if flags & FORMAT_MESSAGE_ALLOCATE_BUFFER != 0 {
        let p = crate::heap::alloc((n + 1).max(size as usize), true) as *mut u8;
        std::ptr::copy_nonoverlapping(s.as_ptr(), p, n);
        *(buf as *mut *mut u8) = p;
        return n as u32;
    }
    if n + 1 > size as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_ansi_buf(&s, buf, size as usize);
    n as u32
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------

static NEXT_ATOM: AtomicUsize = AtomicUsize::new(0xC000);

fn add_atom(s: String) -> u16 {
    static ATOMS: OnceLock<std::sync::Mutex<std::collections::HashMap<String, u16>>> = OnceLock::new();
    let mut map = ATOMS.get_or_init(Default::default).lock().unwrap();
    let key = s.to_ascii_lowercase();
    if let Some(&a) = map.get(&key) {
        return a;
    }
    let a = NEXT_ATOM.fetch_add(1, Ordering::Relaxed) as u16;
    map.insert(key, a);
    a
}

pub unsafe extern "win64" fn GlobalAddAtomW(s: LPCWSTR) -> u16 {
    if s.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    add_atom(from_wide(s))
}
pub unsafe extern "win64" fn GlobalAddAtomA(s: LPCSTR) -> u16 {
    if s.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    add_atom(from_ansi(s))
}
pub unsafe extern "win64" fn GlobalDeleteAtom(_a: u16) -> u16 {
    0
}

pub unsafe extern "win64" fn GetProcessMemoryInfo(h: HANDLE, out: *mut u8, cb: u32) -> BOOL {
    K32GetProcessMemoryInfo(h, out, cb)
}

pub unsafe extern "win64" fn IsApiSetImplemented(contract: LPCSTR) -> BOOL {
    let name = from_ansi_opt(contract).unwrap_or_default();
    let c = crate::canonical_dll(&name);
    (c != normalize_dll(&name) || crate::find_builtin(&c).is_some()) as BOOL
}

fn normalize_dll(s: &str) -> String {
    pe_image::normalize_dll_name(s)
}

pub unsafe extern "win64" fn QuirkIsEnabled(_quirk: *const c_void) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn GetSystemTimes(idle: *mut FILETIME, kernel: *mut FILETIME, user: *mut FILETIME) -> BOOL {
    extern "C" {
        fn mach_host_self() -> u32;
        fn host_statistics(host_priv: u32, flavor: i32, host_info_out: *mut i32, host_info_out_cnt: *mut u32) -> i32;
    }
    const HOST_CPU_LOAD_INFO: i32 = 3;
    let mut cpu_ticks = [0u32; 4]; // [USER, SYSTEM, IDLE, NICE]
    let mut count: u32 = 4;
    let kr = host_statistics(
        mach_host_self(),
        HOST_CPU_LOAD_INFO,
        cpu_ticks.as_mut_ptr() as *mut i32,
        &mut count,
    );
    let (idle_100ns, kernel_100ns, user_100ns) = if kr == 0 && count >= 4 {
        let hz = libc::sysconf(libc::_SC_CLK_TCK);
        let tick_100ns = if hz > 0 { 10_000_000u64 / (hz as u64) } else { 100_000u64 };
        let u = (cpu_ticks[0] as u64).saturating_add(cpu_ticks[3] as u64).saturating_mul(tick_100ns);
        let s = (cpu_ticks[1] as u64).saturating_mul(tick_100ns);
        let i = (cpu_ticks[2] as u64).saturating_mul(tick_100ns);
        // Per Win32 MSDN docs, lpKernelTime includes lpIdleTime.
        (i, s.saturating_add(i), u)
    } else {
        let up = crate::kernel32::time::interrupt_time_100ns();
        let n = pr::num_cpus() as u64;
        (up * n / 2, up * n, up * n / 4)
    };
    for (p, v) in [(idle, idle_100ns), (kernel, kernel_100ns), (user, user_100ns)] {
        if !p.is_null() {
            std::ptr::write_unaligned(p, FILETIME::from_u64(v));
        }
    }
    TRUE
}

pub unsafe extern "win64" fn GetCurrentPackageId(_len: *mut u32, _buf: *mut u8) -> i32 {
    15700 // APPMODEL_ERROR_NO_PACKAGE
}
pub unsafe extern "win64" fn GetCurrentPackageFullName(_len: *mut u32, _buf: *mut u16) -> i32 {
    15700
}
pub unsafe extern "win64" fn GetPackageFamilyName(_h: HANDLE, _len: *mut u32, _buf: *mut u16) -> i32 {
    15700
}

pub unsafe extern "win64" fn WerRegisterRuntimeExceptionModule(_p: LPCWSTR, _c: *mut c_void) -> HRESULT {
    S_OK
}
pub unsafe extern "win64" fn WerSetFlags(_f: u32) -> HRESULT {
    S_OK
}
pub unsafe extern "win64" fn WerRegisterCustomMetadata(_key: LPCWSTR, _val: LPCWSTR) -> HRESULT {
    S_OK
}
pub unsafe extern "win64" fn WerUnregisterCustomMetadata(_key: LPCWSTR) -> HRESULT {
    S_OK
}
pub unsafe extern "win64" fn RegisterApplicationRestart(_c: LPCWSTR, _f: u32) -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn SetThreadExecutionState(f: u32) -> u32 {
    static PREV: AtomicU32 = AtomicU32::new(0x8000_0000);
    PREV.swap(f, Ordering::Relaxed)
}

pub unsafe extern "win64" fn GetSystemPowerStatus(out: *mut u8) -> BOOL {
    // AC online, no battery info.
    *out = 1;
    *out.add(1) = 128;
    *out.add(2) = 255;
    *out.add(3) = 0;
    (out.add(4) as *mut u32).write_unaligned(u32::MAX);
    (out.add(8) as *mut u32).write_unaligned(u32::MAX);
    TRUE
}

pub unsafe extern "win64" fn DebugActiveProcess(_pid: u32) -> BOOL {
    set_err(err::ERROR_ACCESS_DENIED);
    FALSE
}

pub unsafe extern "win64" fn DebugActiveProcessStop(_pid: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetBinaryTypeW(_app_name: LPCWSTR, binary_type: *mut u32) -> BOOL {
    if binary_type.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    *binary_type = 6; // SCS_64BIT_BINARY
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pointer_codec() {
        let p = 0x1234_5678_9abc_usize;
        assert_eq!(decode_pointer(encode_pointer(p)), p);
    }
    #[test]
    fn muldiv() {
        unsafe {
            assert_eq!(MulDiv(10, 3, 4), 8); // 7.5 -> 8
            assert_eq!(MulDiv(-10, 3, 4), -8);
            assert_eq!(MulDiv(1, 1, 0), -1);
        }
    }
    #[test]
    fn inserts() {
        unsafe {
            let s = b"world\0";
            let mut get = |i: usize| if i == 1 { s.as_ptr() as u64 } else { 42 };
            assert_eq!(format_inserts("hello %1 %2!d!%n", false, false, &mut get), "hello world 42\r\n");
        }
    }

    #[test]
    fn toolhelp_snapshot_validates_thread_and_module_sizes() {
        unsafe {
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD | TH32CS_SNAPMODULE, 0);
            assert!(!snap.is_null());

            let mut tiny = [0u8; 28];
            (tiny.as_mut_ptr() as *mut u32).write_unaligned(16);
            assert_eq!(Thread32First(snap, tiny.as_mut_ptr()), FALSE);
            assert_eq!(GetLastError(), 24);

            let mut te = [0u8; 28];
            (te.as_mut_ptr() as *mut u32).write_unaligned(28);
            // No Win32 guest threads started in unit test context -> ERROR_NO_MORE_FILES cleanly.
            let got = Thread32First(snap, te.as_mut_ptr());
            if got == FALSE {
                assert_eq!(GetLastError(), err::ERROR_NO_MORE_FILES);
            } else {
                assert_eq!((te.as_ptr().add(0x0C) as *const u32).read_unaligned(), std::process::id());
            }

            let mut small_mod = [0u8; 568];
            (small_mod.as_mut_ptr() as *mut u32).write_unaligned(100);
            assert_eq!(Module32First(snap, small_mod.as_mut_ptr()), FALSE);
            assert_eq!(GetLastError(), 24);
            assert!(handles::close(snap));
        }
    }

    #[test]
    fn get_system_times_reads_host_cpu_ticks() {
        unsafe {
            let mut idle = FILETIME::from_u64(0);
            let mut kernel = FILETIME::from_u64(0);
            let mut user = FILETIME::from_u64(0);
            assert_eq!(GetSystemTimes(&mut idle, &mut kernel, &mut user), TRUE);
            let i = ((idle.dwHighDateTime as u64) << 32) | (idle.dwLowDateTime as u64);
            let k = ((kernel.dwHighDateTime as u64) << 32) | (kernel.dwLowDateTime as u64);
            let u = ((user.dwHighDateTime as u64) << 32) | (user.dwLowDateTime as u64);
            assert!(k >= i, "kernel time ({k}) must include idle time ({i})");
            assert!(k + u > 0, "total CPU ticks must be nonzero");
        }
    }

    #[test]
    fn proc_thread_attribute_list_and_create_process_validation() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, GetHandleInformation, SetHandleInformation};
        unsafe {
            // Query required size for 2 attributes.
            let mut sz = 0usize;
            assert_eq!(InitializeProcThreadAttributeList(std::ptr::null_mut(), 2, 0, &mut sz), FALSE);
            assert_eq!(GetLastError(), err::ERROR_INSUFFICIENT_BUFFER);
            assert!(sz >= std::mem::size_of::<ProcThreadAttrHeader>() + 2 * std::mem::size_of::<ProcThreadAttrEntry>());

            let mut buf = vec![0u8; sz];
            assert_eq!(InitializeProcThreadAttributeList(buf.as_mut_ptr() as *mut c_void, 2, 0, &mut sz), TRUE);

            // Update with HANDLE_LIST succeeds.
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert_eq!(SetHandleInformation(ev, handles::HANDLE_FLAG_INHERIT, handles::HANDLE_FLAG_INHERIT), TRUE);
            let mut flags_out = 0u32;
            assert_eq!(GetHandleInformation(ev, &mut flags_out), TRUE);
            assert_eq!(flags_out, handles::HANDLE_FLAG_INHERIT);

            let mut hlist = [ev];
            assert_eq!(
                UpdateProcThreadAttribute(
                    buf.as_mut_ptr() as *mut c_void,
                    0,
                    PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
                    hlist.as_mut_ptr() as *mut c_void,
                    std::mem::size_of_val(&hlist),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                TRUE
            );

            // Unsupported attribute returns ERROR_NOT_SUPPORTED.
            let mut dummy = 0u64;
            assert_eq!(
                UpdateProcThreadAttribute(
                    buf.as_mut_ptr() as *mut c_void,
                    0,
                    0x0002_0016, // PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE
                    &mut dummy as *mut u64 as *mut c_void,
                    8,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(GetLastError(), err::ERROR_NOT_SUPPORTED);

            // CreateProcessW rejects CREATE_SUSPENDED and invalid SI / handle-list without inherit.
            let app = to_wide_z("C:\\nonexistent.exe");
            let mut si = [0u8; 112];
            (si.as_mut_ptr() as *mut u32).write_unaligned(112);
            (si.as_mut_ptr().add(104) as *mut *mut c_void).write_unaligned(buf.as_mut_ptr() as *mut c_void);
            let mut pi = [0u8; 24];

            assert_eq!(
                CreateProcessW(
                    app.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    TRUE,
                    CREATE_SUSPENDED,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si.as_mut_ptr() as *mut c_void,
                    pi.as_mut_ptr() as *mut c_void,
                ),
                FALSE
            );
            assert_eq!(GetLastError(), err::ERROR_NOT_SUPPORTED);

            #[cfg(not(target_os = "ios"))]
            {
                // HANDLE_LIST with inherit=FALSE -> ERROR_INVALID_PARAMETER.
                assert_eq!(
                    CreateProcessW(
                        app.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        FALSE,
                        EXTENDED_STARTUPINFO_PRESENT,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                        si.as_mut_ptr() as *mut c_void,
                        pi.as_mut_ptr() as *mut c_void,
                    ),
                    FALSE
                );
                assert_eq!(GetLastError(), err::ERROR_INVALID_PARAMETER);

                // HANDLE_LIST with non-stdio handle (Event) or without STARTF_USESTDHANDLES -> ERROR_NOT_SUPPORTED before spawn.
                assert_eq!(
                    CreateProcessW(
                        app.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        TRUE,
                        EXTENDED_STARTUPINFO_PRESENT,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                        si.as_mut_ptr() as *mut c_void,
                        pi.as_mut_ptr() as *mut c_void,
                    ),
                    FALSE
                );
                assert_eq!(GetLastError(), err::ERROR_NOT_SUPPORTED);

                // Even with STARTF_USESTDHANDLES set, a non-stdio handle in HANDLE_LIST -> ERROR_NOT_SUPPORTED.
                let mut si_std = si;
                (si_std.as_mut_ptr().add(60) as *mut u32).write_unaligned(STARTF_USESTDHANDLES);
                (si_std.as_mut_ptr().add(80) as *mut HANDLE).write_unaligned(ev);
                assert_eq!(
                    CreateProcessW(
                        app.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        TRUE,
                        EXTENDED_STARTUPINFO_PRESENT,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                        si_std.as_mut_ptr() as *mut c_void,
                        pi.as_mut_ptr() as *mut c_void,
                    ),
                    FALSE
                );
                assert_eq!(GetLastError(), err::ERROR_NOT_SUPPORTED);

                // Missing exe with standard STARTUPINFOW -> ERROR_FILE_NOT_FOUND.
                let mut si_basic = [0u8; 104];
                (si_basic.as_mut_ptr() as *mut u32).write_unaligned(104);
                assert_eq!(
                    CreateProcessW(
                        app.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        FALSE,
                        0,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                        si_basic.as_mut_ptr() as *mut c_void,
                        pi.as_mut_ptr() as *mut c_void,
                    ),
                    FALSE
                );
                assert_eq!(GetLastError(), err::ERROR_FILE_NOT_FOUND);
            }

            DeleteProcThreadAttributeList(buf.as_mut_ptr() as *mut c_void);
            // Update after delete fails with ERROR_INVALID_PARAMETER.
            assert_eq!(
                UpdateProcThreadAttribute(
                    buf.as_mut_ptr() as *mut c_void,
                    0,
                    PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
                    hlist.as_mut_ptr() as *mut c_void,
                    std::mem::size_of_val(&hlist),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(GetLastError(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(CloseHandle(ev), TRUE);

            // Env codec roundtrip
            let orig = vec![
                ("FOO".to_string(), "bar=baz:1".to_string()),
                ("EMPTY".to_string(), "".to_string()),
            ];
            let enc = pr::encode_env_list(&orig);
            assert_eq!(pr::decode_env_list(&enc).as_deref(), Some(orig.as_slice()));
        }
    }

    #[cfg(not(target_os = "ios"))]
    #[test]
    fn child_runner_command_clears_host_env_and_isolates_protocol_vars() {
        pr::init_trusted_host_env();
        let marker_key = "HOST_ONLY_CHILD_ISOLATION_MARKER";
        let aot_key = "PORT_AOT_DYLIB";
        let prev_marker = std::env::var_os(marker_key);
        let prev_aot = std::env::var_os(aot_key);
        std::env::set_var(marker_key, "host-only-value");
        std::env::set_var(aot_key, "marker-value-must-not-propagate");

        let guest_vars = vec![
            ("GUEST_OVERRIDE".to_string(), "guest-value".to_string()),
            ("PORT_AOT_DYLIB".to_string(), "guest-var-stays-serialized".to_string()),
        ];
        let cmd = build_child_runner_command(
            std::path::Path::new("/usr/bin/true"),
            std::path::Path::new("/tmp/guest.exe"),
            "C:\\guest.exe",
            "\"C:\\guest.exe\"",
            "C:\\",
            &guest_vars,
            true,
        );

        match prev_marker {
            Some(v) => std::env::set_var(marker_key, v),
            None => std::env::remove_var(marker_key),
        }
        match prev_aot {
            Some(v) => std::env::set_var(aot_key, v),
            None => std::env::remove_var(aot_key),
        }

        let env_map: std::collections::HashMap<String, Option<String>> = cmd
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|val| val.to_string_lossy().into_owned()),
                )
            })
            .collect();

        // Host-only / loader override vars must NOT leak into child host environment.
        assert!(!env_map.contains_key(marker_key));
        assert!(!env_map.contains_key(aot_key));
        assert!(!env_map.contains_key("DYLD_INSERT_LIBRARIES"));

        // Runtime-owned protocol controls and serialized guest environment are explicitly present.
        assert_eq!(env_map.get("PORT_CHILD_STDIO").and_then(|v| v.as_deref()), Some("1"));
        assert_eq!(env_map.get("PORT_CHILD_EXE_WIN").and_then(|v| v.as_deref()), Some("C:\\guest.exe"));
        assert_eq!(env_map.get("PORT_CHILD_CWD").and_then(|v| v.as_deref()), Some("C:\\"));
        assert_eq!(env_map.get("PORT_NO_BASE_RETRY").and_then(|v| v.as_deref()), Some("1"));
        assert_eq!(env_map.get("PORT_HEADLESS").and_then(|v| v.as_deref()), Some("1"));
        let decoded = pr::decode_env_list(
            env_map.get("PORT_CHILD_ENV").and_then(|v| v.as_deref()).expect("PORT_CHILD_ENV"),
        )
        .expect("decode PORT_CHILD_ENV");
        assert_eq!(decoded, guest_vars);
    }
}
