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

static PROCESS_AFFINITY_MASK: AtomicUsize = AtomicUsize::new(0);

pub fn system_affinity_mask() -> usize {
    let n = pr::num_cpus().min(64);
    if n == 64 { usize::MAX } else { (1usize << n) - 1 }
}

pub fn current_process_affinity_mask() -> usize {
    let v = PROCESS_AFFINITY_MASK.load(Ordering::Acquire);
    if v == 0 { system_affinity_mask() } else { v }
}

pub unsafe extern "win64" fn GetProcessAffinityMask(h: HANDLE, proc_mask: *mut usize, sys_mask: *mut usize) -> BOOL {
    if !is_self(h) && handles::get_as::<ChildProcessObj>(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let sm = system_affinity_mask();
    let pm = current_process_affinity_mask() & sm;
    if !proc_mask.is_null() {
        std::ptr::write_unaligned(proc_mask, if pm == 0 { sm } else { pm });
    }
    if !sys_mask.is_null() {
        std::ptr::write_unaligned(sys_mask, sm);
    }
    TRUE
}

pub unsafe extern "win64" fn SetProcessAffinityMask(h: HANDLE, mask: usize) -> BOOL {
    if !is_self(h) {
        set_err(if handles::get_as::<ChildProcessObj>(h).is_some() {
            err::ERROR_ACCESS_DENIED
        } else {
            err::ERROR_INVALID_HANDLE
        });
        return FALSE;
    }
    let sm = system_affinity_mask();
    if mask == 0 || (mask & !sm) != 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    PROCESS_AFFINITY_MASK.store(mask, Ordering::Release);
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
pub unsafe extern "win64" fn GetProcessHandleCount(h: HANDLE, n: *mut u32) -> BOOL {
    if n.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if !is_self(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    std::ptr::write_unaligned(n, handles::live_count());
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

pub(crate) fn task_memory() -> (u64, u64) {
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
fn win_parent_dir(win_path: &str) -> Option<String> {
    let norm = crate::path::normalize(win_path.trim());
    let trimmed = norm.trim_end_matches('\\');
    let sep = trimmed.rfind('\\')?;
    let dir = &trimmed[..sep];
    if dir.is_empty() {
        Some("\\".to_string())
    } else if dir.len() == 2 && dir.as_bytes()[1] == b':' {
        Some(format!("{dir}\\"))
    } else {
        Some(dir.to_string())
    }
}

#[cfg(not(target_os = "ios"))]
fn join_win_dir(dir: &str, name: &str) -> String {
    let cleaned = dir.trim().trim_matches('"').trim();
    if cleaned.is_empty() {
        return name.to_string();
    }
    let base = cleaned.trim_end_matches(['\\', '/']);
    if base.is_empty() {
        format!("\\{name}")
    } else {
        format!("{base}\\{name}")
    }
}

#[cfg(not(target_os = "ios"))]
fn resolve_guest_exe_with_dirs(
    raw_exe: &str,
    from_app_name: bool,
    parent_exe: Option<&str>,
    win_dirs: [&str; 3],
) -> Result<(String, std::path::PathBuf), u32> {
    let has_dir = raw_exe.contains('\\') || raw_exe.contains('/') || raw_exe.contains(':');
    if from_app_name || has_dir {
        let win = crate::path::absolute(raw_exe);
        if let Some(host) = crate::path::to_host(&win) {
            if host.is_file() {
                return Ok((crate::path::normalize(&win), host));
            }
        }
        return Err(err::ERROR_FILE_NOT_FOUND);
    }

    let file_name = if raw_exe.contains('.') {
        raw_exe.to_string()
    } else {
        format!("{raw_exe}.exe")
    };

    let mut dirs: Vec<String> = Vec::new();
    if let Some(app_dir) = parent_exe.and_then(win_parent_dir) {
        dirs.push(app_dir);
    }
    dirs.push(crate::path::cwd());
    for d in win_dirs {
        dirs.push(d.to_string());
    }
    if let Some(path_env) = pr::get_env("Path") {
        for raw_dir in path_env.split(';') {
            let dir = raw_dir.trim().trim_matches('"').trim();
            if !dir.is_empty() {
                dirs.push(dir.to_string());
            }
        }
    }

    for dir in dirs {
        let win = crate::path::absolute(&join_win_dir(&dir, &file_name));
        if let Some(host) = crate::path::to_host(&win) {
            if host.is_file() {
                return Ok((crate::path::normalize(&win), host));
            }
        }
    }
    Err(err::ERROR_FILE_NOT_FOUND)
}

#[cfg(not(target_os = "ios"))]
fn resolve_guest_exe(
    raw_exe: &str,
    from_app_name: bool,
    parent_exe: Option<&str>,
) -> Result<(String, std::path::PathBuf), u32> {
    resolve_guest_exe_with_dirs(
        raw_exe,
        from_app_name,
        parent_exe,
        [
            crate::kernel32::sysinfo::SYSTEM_DIR,
            crate::kernel32::sysinfo::SYSTEM16_DIR,
            crate::kernel32::sysinfo::WINDOWS_DIR,
        ],
    )
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
    command.env("PORT_CHILD_PROCESS", "1");
    command.env("PORT_AOT", "0");
    command.env("AW2_DLL_AOT", "0");
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
const POSIX_SPAWN_START_SUSPENDED: libc::c_short = 0x0080;
#[cfg(not(target_os = "ios"))]
const POSIX_SPAWN_CLOEXEC_DEFAULT: libc::c_short = 0x4000;

#[cfg(not(target_os = "ios"))]
extern "C" {
    fn posix_spawn_file_actions_addinherit_np(
        file_actions: *mut libc::posix_spawn_file_actions_t,
        filedes: libc::c_int,
    ) -> libc::c_int;
}

#[cfg(not(target_os = "ios"))]
struct OwnedFd(i32);
#[cfg(not(target_os = "ios"))]
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

#[cfg(not(target_os = "ios"))]
enum ChildStdioTarget {
    Inherit,
    DevNull,
    DupFd(OwnedFd),
}

#[cfg(not(target_os = "ios"))]
struct ChildStdioPlan {
    stdin: ChildStdioTarget,
    stdout: ChildStdioTarget,
    stderr: ChildStdioTarget,
}

#[cfg(not(target_os = "ios"))]
fn dup_handle_to_child_stdio(h: HANDLE, allowed_list: Option<&[HANDLE]>) -> Result<ChildStdioTarget, u32> {
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return Ok(ChildStdioTarget::DevNull);
    }
    if let Some(list) = allowed_list {
        if !list.contains(&h) {
            return Ok(ChildStdioTarget::DevNull);
        }
    } else if !is_inheritable_handle(h) {
        return Ok(ChildStdioTarget::DevNull);
    }
    let Some((raw_fd, _kind)) = crate::kernel32::file::file_fd_and_kind(h) else {
        return Err(err::ERROR_INVALID_HANDLE);
    };
    let dup_fd = unsafe { libc::fcntl(raw_fd, libc::F_DUPFD_CLOEXEC, 3) };
    if dup_fd < 0 {
        return Err(errno_to_win32(last_errno()));
    }
    Ok(ChildStdioTarget::DupFd(OwnedFd(dup_fd)))
}

#[cfg(not(target_os = "ios"))]
fn spawn_and_verify_suspended_child(
    exe_path: &std::path::Path,
    expected_identity: &pr::RunnerCodeIdentity,
    args: &[std::ffi::OsString],
    envs: &[(std::ffi::OsString, std::ffi::OsString)],
    stdio: &ChildStdioPlan,
    inherit_fd: Option<i32>,
) -> Result<libc::pid_t, u32> {
    use std::ffi::CString;

    let c_exe = CString::new(exe_path.as_os_str().as_encoded_bytes())
        .map_err(|_| err::ERROR_INVALID_PARAMETER)?;
    let mut c_args: Vec<CString> = Vec::with_capacity(args.len() + 1);
    c_args.push(c_exe.clone());
    for a in args {
        c_args.push(
            CString::new(a.as_encoded_bytes()).map_err(|_| err::ERROR_INVALID_PARAMETER)?,
        );
    }
    let mut argv_ptrs: Vec<*mut libc::c_char> =
        c_args.iter().map(|s| s.as_ptr() as *mut libc::c_char).collect();
    argv_ptrs.push(std::ptr::null_mut());

    let mut c_envs: Vec<CString> = Vec::with_capacity(envs.len());
    for (k, v) in envs {
        let mut kv = Vec::with_capacity(k.len() + 1 + v.len());
        kv.extend_from_slice(k.as_encoded_bytes());
        kv.push(b'=');
        kv.extend_from_slice(v.as_encoded_bytes());
        c_envs.push(CString::new(kv).map_err(|_| err::ERROR_INVALID_PARAMETER)?);
    }
    let mut envp_ptrs: Vec<*mut libc::c_char> =
        c_envs.iter().map(|s| s.as_ptr() as *mut libc::c_char).collect();
    envp_ptrs.push(std::ptr::null_mut());

    unsafe {
        let mut attr: libc::posix_spawnattr_t = std::ptr::null_mut();
        let rc = libc::posix_spawnattr_init(&mut attr);
        if rc != 0 {
            return Err(errno_to_win32(rc));
        }
        let mut actions: libc::posix_spawn_file_actions_t = std::ptr::null_mut();
        let rc = libc::posix_spawn_file_actions_init(&mut actions);
        if rc != 0 {
            libc::posix_spawnattr_destroy(&mut attr);
            return Err(errno_to_win32(rc));
        }

        let wire_res = (|| -> Result<(), u32> {
            // Start suspended in the kernel BEFORE dyld/initializers execute, and close every
            // parent fd by default except explicitly wired stdio + exit-code channel fds.
            let rc = libc::posix_spawnattr_setflags(
                &mut attr,
                POSIX_SPAWN_START_SUSPENDED | POSIX_SPAWN_CLOEXEC_DEFAULT,
            );
            if rc != 0 {
                return Err(errno_to_win32(rc));
            }
            for (target_fd, target) in [
                (0, &stdio.stdin),
                (1, &stdio.stdout),
                (2, &stdio.stderr),
            ] {
                let open_mode = if target_fd == 0 { libc::O_RDONLY } else { libc::O_WRONLY };
                let rc = match target {
                    ChildStdioTarget::Inherit => {
                        if libc::fcntl(target_fd, libc::F_GETFD) >= 0 {
                            posix_spawn_file_actions_addinherit_np(&mut actions, target_fd)
                        } else {
                            libc::posix_spawn_file_actions_addopen(
                                &mut actions,
                                target_fd,
                                c"/dev/null".as_ptr(),
                                open_mode,
                                0,
                            )
                        }
                    }
                    ChildStdioTarget::DevNull => libc::posix_spawn_file_actions_addopen(
                        &mut actions,
                        target_fd,
                        c"/dev/null".as_ptr(),
                        open_mode,
                        0,
                    ),
                    ChildStdioTarget::DupFd(fd) => {
                        libc::posix_spawn_file_actions_adddup2(&mut actions, fd.0, target_fd)
                    }
                };
                if rc != 0 {
                    return Err(errno_to_win32(rc));
                }
            }
            if let Some(fd) = inherit_fd {
                let rc = posix_spawn_file_actions_addinherit_np(&mut actions, fd);
                if rc != 0 {
                    return Err(errno_to_win32(rc));
                }
            }
            Ok(())
        })();

        if let Err(e) = wire_res {
            libc::posix_spawn_file_actions_destroy(&mut actions);
            libc::posix_spawnattr_destroy(&mut attr);
            return Err(e);
        }

        let mut pid: libc::pid_t = 0;
        let rc = libc::posix_spawn(
            &mut pid,
            c_exe.as_ptr(),
            &actions,
            &attr,
            argv_ptrs.as_ptr(),
            envp_ptrs.as_ptr(),
        );
        libc::posix_spawn_file_actions_destroy(&mut actions);
        libc::posix_spawnattr_destroy(&mut attr);
        if rc != 0 {
            return Err(errno_to_win32(rc));
        }

        // Validate the kernel-backed signing status and CDHash of the exact mapped image
        // while the child is still suspended before dyld/userspace entry.
        if let Err(reason) = pr::verify_suspended_child_identity(pid, expected_identity) {
            log::error!(
                "CreateProcessW: rejecting spawned child pid={pid} before execution: {reason}"
            );
            pr::kill_and_reap_pid(pid);
            return Err(err::ERROR_ACCESS_DENIED);
        }

        if libc::kill(pid, libc::SIGCONT) != 0 {
            let e = last_errno();
            pr::kill_and_reap_pid(pid);
            return Err(errno_to_win32(e));
        }

        Ok(pid)
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
        // Native-unsupported on iOS: Apple's iOS App Sandbox and kernel policy prohibit
        // spawning child processes (`posix_spawn` / `execve` are blocked by entitlement).
        // Returning FALSE with ERROR_NOT_SUPPORTED matches Windows platforms without
        // child-process creation capability.
        let _ = (inherit, flags, env, cwd, si, pi);
        log::debug!(
            "CreateProcessW unavailable on iOS sandbox: app={:?} cmd={:?}",
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

        let (raw_exe, from_app_name) = if let Some(ref a) = app_s {
            (a.clone(), true)
        } else {
            let parts = pr::split_cmdline(cmd_s.as_deref().unwrap_or(""));
            let Some(first) = parts.into_iter().next().filter(|s| !s.is_empty()) else {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            };
            (first, false)
        };

        let (exe_win, exe_host) = match resolve_guest_exe(
            &raw_exe,
            from_app_name,
            pr::try_config().map(|c| c.exe_path.as_str()),
        ) {
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

        let Some(trusted_runner) = pr::trusted_runner() else {
            log::warn!("CreateProcessW: failing closed — no verified trusted host runner pinned");
            set_err(err::ERROR_ACCESS_DENIED);
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
        let command = build_child_runner_command(
            &trusted_runner.exe_path,
            &exe_host,
            &exe_win,
            &effective_cmdline,
            &child_cwd_win,
            &child_env_vars,
            headless,
        );

        let stdio_plan = if (si_flags & STARTF_USESTDHANDLES) != 0 && inherit != FALSE {
            let allowed = handle_list.as_deref();
            let stdin = match dup_handle_to_child_stdio(h_stdin, allowed) {
                Ok(s) => s,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            };
            let stdout = match dup_handle_to_child_stdio(h_stdout, allowed) {
                Ok(s) => s,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            };
            let stderr = match dup_handle_to_child_stdio(h_stderr, allowed) {
                Ok(s) => s,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            };
            ChildStdioPlan { stdin, stdout, stderr }
        } else {
            ChildStdioPlan {
                stdin: ChildStdioTarget::Inherit,
                stdout: ChildStdioTarget::Inherit,
                stderr: ChildStdioTarget::Inherit,
            }
        };

        // Dedicated parent-child pipe to report the full 32-bit guest Windows exit code.
        // Both ends start FD_CLOEXEC and the write end is kept >= 3 so it never collides
        // with child stdio (0/1/2); `POSIX_SPAWN_CLOEXEC_DEFAULT` + `addinherit_np` passes
        // only this write end to the child.
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
        let mut exit_write_fd = OwnedFd(exit_pipe[1]);
        if exit_write_fd.0 < 3 {
            let moved = libc::fcntl(exit_write_fd.0, libc::F_DUPFD_CLOEXEC, 3);
            if moved < 0 {
                set_err(errno_to_win32(last_errno()));
                return FALSE;
            }
            exit_write_fd = OwnedFd(moved);
        }
        let child_exit_wfd = exit_write_fd.0;

        let args: Vec<std::ffi::OsString> =
            command.get_args().map(|a| a.to_os_string()).collect();
        let mut envs: Vec<(std::ffi::OsString, std::ffi::OsString)> = command
            .get_envs()
            .filter_map(|(k, v)| v.map(|val| (k.to_os_string(), val.to_os_string())))
            .collect();
        envs.push((
            std::ffi::OsString::from("PORT_EXIT_CODE_FD"),
            std::ffi::OsString::from(child_exit_wfd.to_string()),
        ));

        let raw_pid = match spawn_and_verify_suspended_child(
            &trusted_runner.exe_path,
            &trusted_runner.identity,
            &args,
            &envs,
            &stdio_plan,
            Some(child_exit_wfd),
        ) {
            Ok(p) => p,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        // Close parent's write end and temporary duped stdio fds immediately so EOF arrives
        // as soon as the child exits.
        drop(exit_write_fd);
        drop(stdio_plan);

        let pid = raw_pid as u32;
        let proc_obj = Arc::new(ChildProcessObj {
            pid,
            exe_win,
            exit_code: handles::Guarded::new(None),
            terminated_code: handles::Guarded::new(None),
            os_pid: std::sync::atomic::AtomicI32::new(raw_pid),
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
                let mut status: libc::c_int = 0;
                let waited = loop {
                    let r = libc::waitpid(raw_pid, &mut status, 0);
                    if r < 0 && last_errno() == libc::EINTR {
                        continue;
                    }
                    break r;
                };
                let fallback_code = if waited == raw_pid {
                    if libc::WIFEXITED(status) {
                        libc::WEXITSTATUS(status) as u32
                    } else if libc::WIFSIGNALED(status) {
                        0xC000_0000u32 | (libc::WTERMSIG(status) as u32)
                    } else {
                        0
                    }
                } else {
                    1
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
// Job objects (limits tracking, process membership, query, kill-on-close)
// ---------------------------------------------------------------------------

const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: u32 = 0x0000_0008;
const JOB_OBJECT_LIMIT_AFFINITY: u32 = 0x0000_0010;
const JOB_OBJECT_LIMIT_PRIORITY_CLASS: u32 = 0x0000_0020;
const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x0000_2000;

enum JobMember {
    SelfProc(u32),
    Child(Arc<ChildProcessObj>),
}

impl JobMember {
    fn pid(&self) -> u32 {
        match self {
            JobMember::SelfProc(pid) => *pid,
            JobMember::Child(c) => c.pid,
        }
    }
    fn is_active(&self) -> bool {
        match self {
            JobMember::SelfProc(_) => true,
            JobMember::Child(c) => c.current_exit_code().is_none(),
        }
    }
}

struct JobState {
    basic_limits: [u8; 64],
    io_counters: [u8; 48],
    process_memory_limit: usize,
    job_memory_limit: usize,
    peak_process_memory: usize,
    peak_job_memory: usize,
    ui_restrictions: u32,
    end_of_job_time_action: u32,
    completion_port: Option<(usize, usize)>,
    cpu_rate_control: [u8; 8],
    notification_limits: [u8; 72],
    group_ids: Vec<u16>,
    group_affinities: Vec<[u8; 16]>,
    members: Vec<JobMember>,
    total_processes: u32,
    total_terminated: u32,
    name: Option<String>,
}

impl JobState {
    fn new(name: Option<String>) -> Self {
        Self {
            basic_limits: [0u8; 64],
            io_counters: [0u8; 48],
            process_memory_limit: 0,
            job_memory_limit: 0,
            peak_process_memory: 0,
            peak_job_memory: 0,
            ui_restrictions: 0,
            end_of_job_time_action: 0,
            completion_port: None,
            cpu_rate_control: [0u8; 8],
            notification_limits: [0u8; 72],
            group_ids: vec![0],
            group_affinities: Vec::new(),
            members: Vec::new(),
            total_processes: 0,
            total_terminated: 0,
            name,
        }
    }

    fn limit_flags(&self) -> u32 {
        u32::from_le_bytes(self.basic_limits[16..20].try_into().unwrap())
    }

    fn active_process_limit(&self) -> u32 {
        u32::from_le_bytes(self.basic_limits[40..44].try_into().unwrap())
    }

    fn active_count(&self) -> u32 {
        self.members.iter().filter(|m| m.is_active()).count() as u32
    }

    fn contains_pid(&self, pid: u32) -> bool {
        self.members.iter().any(|m| m.pid() == pid)
    }
}

pub struct JobObj {
    state: std::sync::Mutex<JobState>,
}

impl JobObj {
    fn new(name: Option<String>) -> Arc<Self> {
        let obj = Arc::new(Self {
            state: std::sync::Mutex::new(JobState::new(name)),
        });
        let mut all = all_jobs().lock().unwrap();
        all.retain(|w| w.strong_count() > 0);
        all.push(Arc::downgrade(&obj));
        obj
    }

    fn terminate_children(&self, exit_code: u32) -> bool {
        let children: Vec<Arc<ChildProcessObj>> = {
            let mut st = self.state.lock().unwrap();
            let mut out = Vec::new();
            for m in &st.members {
                if let JobMember::Child(c) = m {
                    if c.current_exit_code().is_none() {
                        out.push(Arc::clone(c));
                    }
                }
            }
            st.total_terminated = st.total_terminated.saturating_add(out.len() as u32);
            out
        };
        #[cfg(not(target_os = "ios"))]
        for child in children {
            let already_exited = {
                let g = handles::wait_lock();
                if child.exit_code.get(&g).is_some() {
                    true
                } else {
                    let tslot = child.terminated_code.get(&g);
                    if tslot.is_none() {
                        *tslot = Some(exit_code);
                    }
                    false
                }
            };
            if !already_exited {
                let pid = child.os_pid.swap(0, Ordering::AcqRel);
                if pid > 0 {
                    unsafe {
                        let _ = libc::kill(pid, libc::SIGKILL);
                    }
                }
            }
        }
        #[cfg(target_os = "ios")]
        let _ = (children, exit_code);
        let st = self.state.lock().unwrap();
        st.contains_pid(std::process::id())
    }
}

impl Drop for JobObj {
    fn drop(&mut self) {
        let (kill_on_close, name) = {
            let st = self.state.get_mut().unwrap_or_else(|e| e.into_inner());
            (
                (st.limit_flags() & JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE) != 0,
                st.name.take(),
            )
        };
        if let Some(n) = name {
            let mut jobs = named_jobs().lock().unwrap_or_else(|e| e.into_inner());
            if jobs.get(&n).is_some_and(|w| w.strong_count() == 0) {
                jobs.remove(&n);
            }
        }
        if kill_on_close {
            // Terminate all processes associated with this job on last-handle close
            // (both assigned child processes and the current process if assigned).
            if self.terminate_children(1) {
                pr::exit_process(1);
            }
        }
    }
}

impl KObject for JobObj {
    fn type_name(&self) -> &'static str {
        "Job"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn all_jobs() -> &'static std::sync::Mutex<Vec<std::sync::Weak<JobObj>>> {
    static JOBS: OnceLock<std::sync::Mutex<Vec<std::sync::Weak<JobObj>>>> = OnceLock::new();
    JOBS.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// Weak namespace for named job objects so closing the last handle drops `JobObj`
/// (triggering `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` and freeing the name for reuse).
fn named_jobs() -> &'static std::sync::Mutex<std::collections::HashMap<String, std::sync::Weak<JobObj>>> {
    static MAP: OnceLock<std::sync::Mutex<std::collections::HashMap<String, std::sync::Weak<JobObj>>>> =
        OnceLock::new();
    MAP.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn create_job_object_impl(name: Option<String>) -> HANDLE {
    let Some(n) = name.filter(|s| !s.is_empty()) else {
        set_err(0);
        return handles::insert(JobObj::new(None));
    };
    {
        let other = handles::named_objects().lock().unwrap();
        if other.contains_key(&n) {
            set_err(err::ERROR_INVALID_HANDLE);
            return std::ptr::null_mut();
        }
    }
    let mut jobs = named_jobs().lock().unwrap();
    if let Some(existing) = jobs.get(&n).and_then(|w| w.upgrade()) {
        drop(jobs);
        set_err(err::ERROR_ALREADY_EXISTS);
        return handles::insert(existing);
    }
    let obj = JobObj::new(Some(n.clone()));
    jobs.insert(n, Arc::downgrade(&obj));
    drop(jobs);
    set_err(0);
    handles::insert(obj)
}

pub unsafe extern "win64" fn CreateJobObjectW(_sec: *mut c_void, name: LPCWSTR) -> HANDLE {
    create_job_object_impl(from_wide_opt(name))
}
pub unsafe extern "win64" fn CreateJobObjectA(_sec: *mut c_void, name: LPCSTR) -> HANDLE {
    create_job_object_impl(from_ansi_opt(name))
}

pub unsafe extern "win64" fn OpenJobObjectW(_access: u32, inherit: BOOL, name: LPCWSTR) -> HANDLE {
    let Some(n) = from_wide_opt(name).filter(|s| !s.is_empty()) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let flags = if inherit != 0 { handles::HANDLE_FLAG_INHERIT } else { 0 };
    let mut jobs = named_jobs().lock().unwrap();
    if let Some(existing) = jobs.get(&n).and_then(|w| w.upgrade()) {
        return handles::insert_with_flags(existing, flags);
    }
    jobs.remove(&n);
    set_err(err::ERROR_FILE_NOT_FOUND);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn OpenJobObjectA(access: u32, inherit: BOOL, name: LPCSTR) -> HANDLE {
    let Some(s) = from_ansi_opt(name) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let w = to_wide_z(&s);
    OpenJobObjectW(access, inherit, w.as_ptr())
}

pub unsafe extern "win64" fn AssignProcessToJobObject(job: HANDLE, p: HANDLE) -> BOOL {
    let Some(j) = handles::get_as::<JobObj>(job) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let member = if is_self(p) {
        JobMember::SelfProc(std::process::id())
    } else if let Some(child) = handles::get_as::<ChildProcessObj>(p) {
        JobMember::Child(child)
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let pid = member.pid();
    let mut st = j.state.lock().unwrap();
    if st.contains_pid(pid) {
        return TRUE;
    }
    if (st.limit_flags() & JOB_OBJECT_LIMIT_ACTIVE_PROCESS) != 0 {
        let limit = st.active_process_limit();
        if limit > 0 && st.active_count() >= limit {
            set_err(err::ERROR_NOT_ENOUGH_QUOTA);
            return FALSE;
        }
    }
    if (st.limit_flags() & JOB_OBJECT_LIMIT_PRIORITY_CLASS) != 0 && pid == std::process::id() {
        let prio = u32::from_le_bytes(st.basic_limits[56..60].try_into().unwrap());
        if prio != 0 {
            PRIORITY_CLASS.store(prio, Ordering::Relaxed);
        }
    }
    st.members.push(member);
    st.total_processes = st.total_processes.saturating_add(1);
    TRUE
}

pub unsafe extern "win64" fn SetInformationJobObject(job: HANDLE, class: u32, info: *const c_void, len: u32) -> BOOL {
    let Some(j) = handles::get_as::<JobObj>(job) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if info.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let bytes = info as *const u8;
    let mut st = j.state.lock().unwrap();
    match class {
        2 => {
            // JobObjectBasicLimitInformation (64 bytes on x64)
            if len != 64 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let mut buf = [0u8; 64];
            std::ptr::copy_nonoverlapping(bytes, buf.as_mut_ptr(), 64);
            let flags = u32::from_le_bytes(buf[16..20].try_into().unwrap());
            if (flags & JOB_OBJECT_LIMIT_AFFINITY) != 0 {
                let aff = usize::from_le_bytes(buf[48..56].try_into().unwrap());
                let sm = system_affinity_mask();
                if aff == 0 || (aff & !sm) != 0 {
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
            }
            st.basic_limits = buf;
            if (flags & JOB_OBJECT_LIMIT_PRIORITY_CLASS) != 0 && st.contains_pid(std::process::id()) {
                let prio = u32::from_le_bytes(buf[56..60].try_into().unwrap());
                if prio != 0 {
                    PRIORITY_CLASS.store(prio, Ordering::Relaxed);
                }
            }
            TRUE
        }
        4 => {
            // JobObjectBasicUIRestrictions (4 bytes)
            if len != 4 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            st.ui_restrictions = std::ptr::read_unaligned(bytes as *const u32);
            TRUE
        }
        6 => {
            // JobObjectEndOfJobTimeInformation (4 bytes)
            if len != 4 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            st.end_of_job_time_action = std::ptr::read_unaligned(bytes as *const u32);
            TRUE
        }
        7 => {
            // JobObjectAssociateCompletionPortInformation (16 bytes on x64)
            if len != 16 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let key = std::ptr::read_unaligned(bytes as *const usize);
            let port = std::ptr::read_unaligned(bytes.add(8) as *const usize);
            st.completion_port = Some((key, port));
            TRUE
        }
        9 => {
            // JobObjectExtendedLimitInformation (144 bytes on x64)
            if len != 144 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let mut basic = [0u8; 64];
            std::ptr::copy_nonoverlapping(bytes, basic.as_mut_ptr(), 64);
            let flags = u32::from_le_bytes(basic[16..20].try_into().unwrap());
            if (flags & JOB_OBJECT_LIMIT_AFFINITY) != 0 {
                let aff = usize::from_le_bytes(basic[48..56].try_into().unwrap());
                let sm = system_affinity_mask();
                if aff == 0 || (aff & !sm) != 0 {
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
            }
            st.basic_limits = basic;
            st.process_memory_limit = std::ptr::read_unaligned(bytes.add(112) as *const usize);
            st.job_memory_limit = std::ptr::read_unaligned(bytes.add(120) as *const usize);
            if (flags & JOB_OBJECT_LIMIT_PRIORITY_CLASS) != 0 && st.contains_pid(std::process::id()) {
                let prio = u32::from_le_bytes(basic[56..60].try_into().unwrap());
                if prio != 0 {
                    PRIORITY_CLASS.store(prio, Ordering::Relaxed);
                }
            }
            TRUE
        }
        11 => {
            // JobObjectGroupInformation (array of WORD group numbers)
            if len == 0 || (len % 2) != 0 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let count = (len / 2) as usize;
            let mut groups = Vec::with_capacity(count);
            for i in 0..count {
                let g = std::ptr::read_unaligned(bytes.add(i * 2) as *const u16);
                if g != 0 {
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
                groups.push(g);
            }
            st.group_ids = groups;
            TRUE
        }
        12 => {
            // JobObjectNotificationLimitInformation (72 bytes on x64)
            if len != 72 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(bytes, st.notification_limits.as_mut_ptr(), 72);
            TRUE
        }
        14 => {
            // JobObjectGroupInformationEx (array of GROUP_AFFINITY, 16 bytes each)
            if len == 0 || (len % 16) != 0 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let count = (len / 16) as usize;
            let sm = system_affinity_mask();
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let mut ga = [0u8; 16];
                std::ptr::copy_nonoverlapping(bytes.add(i * 16), ga.as_mut_ptr(), 16);
                let mask = usize::from_le_bytes(ga[0..8].try_into().unwrap());
                let group = u16::from_le_bytes(ga[8..10].try_into().unwrap());
                if group != 0 || mask == 0 || (mask & !sm) != 0 {
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
                out.push(ga);
            }
            st.group_affinities = out;
            TRUE
        }
        15 => {
            // JobObjectCpuRateControlInformation (JOBOBJECT_CPU_RATE_CONTROL_INFORMATION: 8 bytes:
            // DWORD ControlFlags + 4-byte union CpuRate/Weight/{MinRate,MaxRate})
            if len != 8 {
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(bytes, st.cpu_rate_control.as_mut_ptr(), 8);
            TRUE
        }
        _ => {
            set_err(err::ERROR_INVALID_PARAMETER);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn QueryInformationJobObject(
    job: HANDLE,
    class: u32,
    info: *mut c_void,
    len: u32,
    ret_len: *mut u32,
) -> BOOL {
    if info.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let j: Arc<JobObj> = if job.is_null() {
        // Per Microsoft Learn: NULL queries the job associated with the current process.
        let my_pid = std::process::id();
        let found = {
            let jobs = all_jobs().lock().unwrap();
            jobs.iter()
                .filter_map(|w| w.upgrade())
                .find(|jo| jo.state.lock().unwrap().contains_pid(my_pid))
        };
        match found {
            Some(jo) => jo,
            None => {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
        }
    } else {
        match handles::get_as::<JobObj>(job) {
            Some(jo) => jo,
            None => {
                set_err(err::ERROR_INVALID_HANDLE);
                return FALSE;
            }
        }
    };

    let put_ret = |n: u32| {
        if !ret_len.is_null() {
            std::ptr::write_unaligned(ret_len, n);
        }
    };
    let out = info as *mut u8;
    let st = j.state.lock().unwrap();
    match class {
        1 => {
            // JobObjectBasicAccountingInformation (48 bytes):
            // +0..32: 4x LARGE_INTEGER times, +32: TotalPageFaultCount (u32),
            // +36: TotalProcesses (u32), +40: ActiveProcesses (u32), +44: TotalTerminatedProcesses (u32).
            if len < 48 {
                put_ret(48);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 48);
            std::ptr::write_unaligned(out.add(36) as *mut u32, st.total_processes);
            std::ptr::write_unaligned(out.add(40) as *mut u32, st.active_count());
            std::ptr::write_unaligned(out.add(44) as *mut u32, st.total_terminated);
            put_ret(48);
            TRUE
        }
        2 => {
            // JobObjectBasicLimitInformation (64 bytes)
            if len < 64 {
                put_ret(64);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(st.basic_limits.as_ptr(), out, 64);
            put_ret(64);
            TRUE
        }
        3 => {
            // JobObjectBasicProcessIdList: u32 assigned, u32 in_list, ULONG_PTR[in_list]
            if len < 8 {
                put_ret(8);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            let pids: Vec<usize> = st.members.iter().filter(|m| m.is_active()).map(|m| m.pid() as usize).collect();
            let cap = ((len as usize) - 8) / std::mem::size_of::<usize>();
            let copy_n = pids.len().min(cap);
            std::ptr::write_unaligned(out as *mut u32, pids.len() as u32);
            std::ptr::write_unaligned(out.add(4) as *mut u32, copy_n as u32);
            for (i, &pid) in pids.iter().take(copy_n).enumerate() {
                std::ptr::write_unaligned(out.add(8 + i * 8) as *mut usize, pid);
            }
            let written = (8 + copy_n * 8) as u32;
            put_ret(written);
            if copy_n < pids.len() {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            TRUE
        }
        4 => {
            // JobObjectBasicUIRestrictions (4 bytes)
            if len < 4 {
                put_ret(4);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::write_unaligned(out as *mut u32, st.ui_restrictions);
            put_ret(4);
            TRUE
        }
        6 => {
            // JobObjectEndOfJobTimeInformation (4 bytes)
            if len < 4 {
                put_ret(4);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::write_unaligned(out as *mut u32, st.end_of_job_time_action);
            put_ret(4);
            TRUE
        }
        8 => {
            // JobObjectBasicAndIoAccountingInformation (96 bytes = 48 basic + 48 io)
            if len < 96 {
                put_ret(96);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 96);
            std::ptr::write_unaligned(out.add(36) as *mut u32, st.total_processes);
            std::ptr::write_unaligned(out.add(40) as *mut u32, st.active_count());
            std::ptr::write_unaligned(out.add(44) as *mut u32, st.total_terminated);
            std::ptr::copy_nonoverlapping(st.io_counters.as_ptr(), out.add(48), 48);
            put_ret(96);
            TRUE
        }
        9 => {
            // JobObjectExtendedLimitInformation (144 bytes on x64)
            if len < 144 {
                put_ret(144);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 144);
            std::ptr::copy_nonoverlapping(st.basic_limits.as_ptr(), out, 64);
            std::ptr::copy_nonoverlapping(st.io_counters.as_ptr(), out.add(64), 48);
            let self_peak = if st.contains_pid(std::process::id()) {
                task_memory().1 as usize
            } else {
                0
            };
            let peak_proc = st.peak_process_memory.max(self_peak);
            let peak_job = st.peak_job_memory.max(peak_proc);
            std::ptr::write_unaligned(out.add(112) as *mut usize, st.process_memory_limit);
            std::ptr::write_unaligned(out.add(120) as *mut usize, st.job_memory_limit);
            std::ptr::write_unaligned(out.add(128) as *mut usize, peak_proc);
            std::ptr::write_unaligned(out.add(136) as *mut usize, peak_job);
            put_ret(144);
            TRUE
        }
        11 => {
            // JobObjectGroupInformation
            let needed = (st.group_ids.len() * 2) as u32;
            put_ret(needed);
            if len < needed {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            for (i, &g) in st.group_ids.iter().enumerate() {
                std::ptr::write_unaligned(out.add(i * 2) as *mut u16, g);
            }
            TRUE
        }
        12 => {
            // JobObjectNotificationLimitInformation (72 bytes)
            if len < 72 {
                put_ret(72);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(st.notification_limits.as_ptr(), out, 72);
            put_ret(72);
            TRUE
        }
        14 => {
            // JobObjectGroupInformationEx
            let entries: Vec<[u8; 16]> = if st.group_affinities.is_empty() {
                let mut def = [0u8; 16];
                def[0..8].copy_from_slice(&current_process_affinity_mask().to_le_bytes());
                vec![def]
            } else {
                st.group_affinities.clone()
            };
            let needed = (entries.len() * 16) as u32;
            put_ret(needed);
            if len < needed {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            for (i, ga) in entries.iter().enumerate() {
                std::ptr::copy_nonoverlapping(ga.as_ptr(), out.add(i * 16), 16);
            }
            TRUE
        }
        15 => {
            // JobObjectCpuRateControlInformation (8 bytes)
            if len < 8 {
                put_ret(8);
                set_err(err::ERROR_BAD_LENGTH);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(st.cpu_rate_control.as_ptr(), out, 8);
            put_ret(8);
            TRUE
        }
        _ => {
            set_err(err::ERROR_INVALID_PARAMETER);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn IsProcessInJob(p: HANDLE, job: HANDLE, out: *mut BOOL) -> BOOL {
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let pid = if is_self(p) {
        std::process::id()
    } else if let Some(child) = handles::get_as::<ChildProcessObj>(p) {
        child.pid
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let in_job = if job.is_null() {
        let jobs = all_jobs().lock().unwrap();
        jobs.iter()
            .filter_map(|w| w.upgrade())
            .any(|jo| jo.state.lock().unwrap().contains_pid(pid))
    } else {
        let Some(jo) = handles::get_as::<JobObj>(job) else {
            set_err(err::ERROR_INVALID_HANDLE);
            return FALSE;
        };
        let found = jo.state.lock().unwrap().contains_pid(pid);
        found
    };
    std::ptr::write_unaligned(out, if in_job { TRUE } else { FALSE });
    TRUE
}

pub unsafe extern "win64" fn TerminateJobObject(job: HANDLE, exit_code: u32) -> BOOL {
    let Some(j) = handles::get_as::<JobObj>(job) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let includes_self = j.terminate_children(exit_code);
    if includes_self {
        pr::exit_process(exit_code);
    }
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

pub(crate) fn process_cookie() -> usize {
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

/// Look up message `id` inside a loaded PE module's `RT_MESSAGETABLE` (resource type 11).
///
/// Parses the Windows `MESSAGE_RESOURCE_DATA` / `MESSAGE_RESOURCE_BLOCK` /
/// `MESSAGE_RESOURCE_ENTRY` binary format (`winnt.h`), supporting both Unicode
/// (`MESSAGE_RESOURCE_UNICODE = 0x0001`) and ANSI entries.
unsafe fn lookup_module_message(source: *const c_void, id: u32, lang: u32) -> Result<String, u32> {
    const RT_MESSAGETABLE: usize = 11;
    const MESSAGE_RESOURCE_UNICODE: u16 = 0x0001;
    let hmod: HMODULE = if source.is_null() {
        match crate::try_loader().and_then(|l| l.get_module_handle(None)) {
            Some(base) if base != 0 => base as HMODULE,
            _ => return Err(err::ERROR_MR_MID_NOT_FOUND),
        }
    } else {
        source as HMODULE
    };
    let hres = crate::kernel32::module::FindResourceExW(
        hmod,
        RT_MESSAGETABLE as LPCWSTR,
        1usize as LPCWSTR,
        lang as u16,
    );
    if hres.is_null() {
        return Err(err::ERROR_MR_MID_NOT_FOUND);
    }
    let data = crate::kernel32::module::LoadResource(hmod, hres) as *const u8;
    let total_len = crate::kernel32::module::SizeofResource(hmod, hres) as usize;
    if data.is_null() || total_len < 4 {
        return Err(err::ERROR_MR_MID_NOT_FOUND);
    }
    let num_blocks = std::ptr::read_unaligned(data as *const u32) as usize;
    let header_end = 4usize.checked_add(num_blocks.checked_mul(12).ok_or(err::ERROR_MR_MID_NOT_FOUND)?).ok_or(err::ERROR_MR_MID_NOT_FOUND)?;
    if header_end > total_len {
        return Err(err::ERROR_MR_MID_NOT_FOUND);
    }
    for b in 0..num_blocks {
        let blk = data.add(4 + b * 12);
        let low_id = std::ptr::read_unaligned(blk as *const u32);
        let high_id = std::ptr::read_unaligned(blk.add(4) as *const u32);
        let mut off = std::ptr::read_unaligned(blk.add(8) as *const u32) as usize;
        if id < low_id || id > high_id {
            continue;
        }
        let target_idx = id - low_id;
        for idx in 0..=target_idx {
            if off.checked_add(4).is_none_or(|e| e > total_len) {
                return Err(err::ERROR_MR_MID_NOT_FOUND);
            }
            let entry_len = std::ptr::read_unaligned(data.add(off) as *const u16) as usize;
            let entry_flags = std::ptr::read_unaligned(data.add(off + 2) as *const u16);
            if entry_len < 4 || off.checked_add(entry_len).is_none_or(|e| e > total_len) {
                return Err(err::ERROR_MR_MID_NOT_FOUND);
            }
            if idx == target_idx {
                let payload = data.add(off + 4);
                let payload_len = entry_len - 4;
                if (entry_flags & MESSAGE_RESOURCE_UNICODE) != 0 {
                    let max_units = payload_len / 2;
                    let mut units = Vec::with_capacity(max_units);
                    for k in 0..max_units {
                        let u = std::ptr::read_unaligned(payload.add(k * 2) as *const u16);
                        if u == 0 {
                            break;
                        }
                        units.push(u);
                    }
                    return Ok(String::from_utf16_lossy(&units));
                } else {
                    let mut bytes = Vec::with_capacity(payload_len);
                    for k in 0..payload_len {
                        let ch = *payload.add(k);
                        if ch == 0 {
                            break;
                        }
                        bytes.push(ch);
                    }
                    return Ok(bytes.into_iter().map(|c| c as char).collect());
                }
            }
            off += entry_len;
        }
    }
    Err(err::ERROR_MR_MID_NOT_FOUND)
}

unsafe fn format_message(flags: u32, source: *const c_void, id: u32, lang: u32, args: *const u64, wide: bool) -> Result<String, u32> {
    let fmt: String = if flags & FORMAT_MESSAGE_FROM_STRING != 0 {
        if source.is_null() {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        if wide {
            from_wide(source as *const u16)
        } else {
            from_ansi(source as *const u8)
        }
    } else if (flags & (FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_FROM_SYSTEM)) != 0 {
        // Per Microsoft Learn (FormatMessage): when both FROM_HMODULE and FROM_SYSTEM
        // are specified, search the module's RT_MESSAGETABLE first and fall back to the
        // system message table if not found.
        let from_mod = if (flags & FORMAT_MESSAGE_FROM_HMODULE) != 0 {
            lookup_module_message(source, id, lang).ok()
        } else {
            None
        };
        match from_mod {
            Some(m) => m,
            None if (flags & FORMAT_MESSAGE_FROM_SYSTEM) != 0 => match system_message(id) {
                Some(m) => format!("{m}\r\n"),
                None => {
                    log::debug!("FormatMessage: no system text for {id:#x}");
                    return Err(err::ERROR_MR_MID_NOT_FOUND);
                }
            },
            None => return Err(err::ERROR_MR_MID_NOT_FOUND),
        }
    } else {
        return Err(err::ERROR_INVALID_PARAMETER);
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

pub unsafe extern "win64" fn FormatMessageW(flags: u32, source: *const c_void, id: u32, lang: u32, buf: LPWSTR, size: u32, args: *const u64) -> u32 {
    let s = match format_message(flags, source, id, lang, args, true) {
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

pub unsafe extern "win64" fn FormatMessageA(flags: u32, source: *const c_void, id: u32, lang: u32, buf: LPSTR, size: u32, args: *const u64) -> u32 {
    let s = match format_message(flags, source, id, lang, args, false) {
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
pub(crate) mod tests {
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
        assert_eq!(env_map.get("PORT_CHILD_PROCESS").and_then(|v| v.as_deref()), Some("1"));
        assert_eq!(env_map.get("PORT_AOT").and_then(|v| v.as_deref()), Some("0"));
        assert_eq!(env_map.get("AW2_DLL_AOT").and_then(|v| v.as_deref()), Some("0"));
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

    #[cfg(not(target_os = "ios"))]
    #[test]
    fn suspended_spawn_rejects_mismatched_cdhash_before_execution_and_accepts_verified_binary() {
        let self_id = pr::query_pid_code_identity(unsafe { libc::getpid() })
            .expect("current macOS binary must have kernel-backed CDHash");
        let echo_path = std::path::Path::new("/bin/echo");
        let echo_trusted = pr::capture_runner_identity(echo_path)
            .expect("/bin/echo must have kernel-backed CDHash");
        assert_ne!(
            self_id.cdhash, echo_trusted.identity.cdhash,
            "test runner and /bin/echo must have distinct kernel CDHashes"
        );

        let make_pipe = || -> (OwnedFd, OwnedFd) {
            let mut fds = [-1i32; 2];
            assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
            for &fd in &fds {
                let fl = unsafe { libc::fcntl(fd, libc::F_GETFD) };
                if fl >= 0 {
                    unsafe {
                        let _ = libc::fcntl(fd, libc::F_SETFD, fl | libc::FD_CLOEXEC);
                    }
                }
            }
            (OwnedFd(fds[0]), OwnedFd(fds[1]))
        };

        let read_all = |rfd: &OwnedFd| -> Vec<u8> {
            let mut out = Vec::new();
            let mut buf = [0u8; 128];
            loop {
                let n = unsafe { libc::read(rfd.0, buf.as_mut_ptr() as *mut c_void, buf.len()) };
                if n <= 0 {
                    break;
                }
                out.extend_from_slice(&buf[..n as usize]);
            }
            out
        };

        // 1. Mismatched expected identity: /bin/echo is spawned suspended, fails CDHash check,
        //    is SIGKILLed and reaped before userspace executes, and produces 0 bytes on stdout.
        let (r1, w1) = make_pipe();
        let stdio1 = ChildStdioPlan {
            stdin: ChildStdioTarget::DevNull,
            stdout: ChildStdioTarget::DupFd(w1),
            stderr: ChildStdioTarget::DevNull,
        };
        let res_mismatch = spawn_and_verify_suspended_child(
            echo_path,
            &self_id,
            &[std::ffi::OsString::from("MUST_NOT_EXECUTE")],
            &[],
            &stdio1,
            None,
        );
        drop(stdio1);
        assert_eq!(
            res_mismatch,
            Err(err::ERROR_ACCESS_DENIED),
            "mismatched CDHash must fail closed with ERROR_ACCESS_DENIED"
        );
        assert!(
            read_all(&r1).is_empty(),
            "rejected child must never execute userspace code or emit output"
        );

        // 2. Matching trusted identity: /bin/echo passes CDHash check, resumes via SIGCONT,
        //    writes expected stdout, and exits cleanly with status 0.
        let (r2, w2) = make_pipe();
        let stdio2 = ChildStdioPlan {
            stdin: ChildStdioTarget::DevNull,
            stdout: ChildStdioTarget::DupFd(w2),
            stderr: ChildStdioTarget::DevNull,
        };
        let pid = spawn_and_verify_suspended_child(
            echo_path,
            &echo_trusted.identity,
            &[std::ffi::OsString::from("verified-ok")],
            &[],
            &stdio2,
            None,
        )
        .expect("matching kernel CDHash must resume and succeed");
        drop(stdio2);
        let out = read_all(&r2);
        let mut status: libc::c_int = 0;
        let waited = unsafe { libc::waitpid(pid, &mut status, 0) };
        assert_eq!(waited, pid);
        assert!(libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0);
        assert_eq!(out, b"verified-ok\n");
    }

    #[cfg(not(target_os = "ios"))]
    #[test]
    fn capture_runner_identity_fails_closed_on_missing_or_inert_non_executable_file() {
        let dir = std::env::temp_dir().join(format!("port_factory_id_fail_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let inert = dir.join("not_a_binary.txt");
        std::fs::write(&inert, b"inert non-executable fixture").unwrap();

        assert!(pr::capture_runner_identity(&inert).is_err());
        assert!(pr::capture_runner_identity(&dir.join("missing_runner")).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(not(target_os = "ios"))]
    #[test]
    fn resolve_guest_exe_follows_create_process_search_order_and_app_name_rules() {
        assert_eq!(win_parent_dir("R:\\app\\parent.exe").as_deref(), Some("R:\\app"));
        assert_eq!(win_parent_dir("R:/app/sub/parent.exe").as_deref(), Some("R:\\app\\sub"));
        assert_eq!(win_parent_dir("R:\\parent.exe").as_deref(), Some("R:\\"));
        assert_eq!(win_parent_dir("R:/parent.exe").as_deref(), Some("R:\\"));
        assert_eq!(win_parent_dir("R:\\"), None);
        assert_eq!(win_parent_dir("bare.exe"), None);

        let dir = std::env::temp_dir().join(format!("port_factory_resolve_exe_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let app_dir = dir.join("app");
        let cwd_dir = dir.join("cwd");
        let cwd_sub = cwd_dir.join("sub");
        let app_sub = app_dir.join("sub");
        let sys32_dir = dir.join("win").join("system32");
        let sys16_dir = dir.join("win").join("System");
        let win_dir = dir.join("win");
        let path_dir = dir.join("pathdir");
        for d in [&app_dir, &cwd_dir, &cwd_sub, &app_sub, &sys32_dir, &sys16_dir, &win_dir, &path_dir] {
            std::fs::create_dir_all(d).unwrap();
        }

        // Seed files across all 6 documented search locations to assert exact precedence.
        for f in [
            app_dir.join("tool.exe"),
            app_dir.join("app_only.exe"),
            app_sub.join("rel.exe"),
            dir.join("root_tool.exe"),
            cwd_dir.join("tool.exe"),
            cwd_dir.join("cwd_tool.exe"),
            cwd_dir.join("root_tool.exe"),
            cwd_dir.join("raw_bin"),
            cwd_sub.join("rel.exe"),
            cwd_sub.join("raw_sub"),
            sys32_dir.join("tool.exe"),
            sys32_dir.join("sys32_tool.exe"),
            sys16_dir.join("sys32_tool.exe"),
            sys16_dir.join("sys16_tool.exe"),
            win_dir.join("sys16_tool.exe"),
            win_dir.join("win_tool.exe"),
            path_dir.join("tool.exe"),
            path_dir.join("win_tool.exe"),
            path_dir.join("path_tool.exe"),
            path_dir.join("script.com"),
        ] {
            std::fs::write(&f, b"MZ").unwrap();
        }

        let prev_cwd = crate::path::cwd();
        let prev_path = pr::get_env("Path");
        crate::path::add_mount("R:\\", &dir);
        crate::path::set_cwd("R:\\cwd");
        pr::set_env("Path", Some("\"R:\\pathdir\""));

        let win_dirs = ["R:\\win\\system32", "R:\\win\\System", "R:\\win"];
        let parent_bs = Some("R:\\app\\parent.exe");
        let parent_fs = Some("R:/app/parent.exe");

        // 1. Bare command-line name precedence: app dir > cwd > system32 > System > Windows > PATH.
        assert_eq!(
            resolve_guest_exe_with_dirs("tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\app\\tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("tool", false, parent_fs, win_dirs).unwrap().0,
            "R:\\app\\tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("root_tool", false, Some("R:/parent.exe"), win_dirs).unwrap().0,
            "R:\\root_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("cwd_tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\cwd_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sys32_tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\win\\system32\\sys32_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sys16_tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\win\\System\\sys16_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("win_tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\win\\win_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("path_tool", false, parent_bs, win_dirs).unwrap().0,
            "R:\\pathdir\\path_tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("script.com", false, parent_bs, win_dirs).unwrap().0,
            "R:\\pathdir\\script.com"
        );

        // 2. App-only helper resolves beside parent guest image even when cwd differs.
        assert_eq!(
            resolve_guest_exe_with_dirs("app_only", false, parent_bs, win_dirs).unwrap().0,
            "R:\\app\\app_only.exe"
        );
        // Trailing dot suppresses .exe inference on bare command-line names.
        assert_eq!(
            resolve_guest_exe_with_dirs("tool.", false, parent_bs, win_dirs),
            Err(err::ERROR_FILE_NOT_FOUND)
        );

        // 3. Explicit lpApplicationName resolves directly against cwd without probing search path or inferring .exe.
        assert_eq!(
            resolve_guest_exe_with_dirs("tool.exe", true, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\tool.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("app_only.exe", true, parent_bs, win_dirs),
            Err(err::ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("path_tool.exe", true, parent_bs, win_dirs),
            Err(err::ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("tool", true, parent_bs, win_dirs),
            Err(err::ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("raw_bin", true, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\raw_bin"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sub\\rel.exe", true, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\sub\\rel.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sub/rel.exe", true, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\sub\\rel.exe"
        );

        // 4. Command-line name containing a directory resolves directly relative to cwd/drive without search or .exe inference.
        assert_eq!(
            resolve_guest_exe_with_dirs("sub\\rel.exe", false, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\sub\\rel.exe"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sub\\rel", false, parent_bs, win_dirs),
            Err(err::ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("sub\\raw_sub", false, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\sub\\raw_sub"
        );
        assert_eq!(
            resolve_guest_exe_with_dirs("R:tool.exe", false, parent_bs, win_dirs).unwrap().0,
            "R:\\cwd\\tool.exe"
        );

        crate::path::set_cwd(&prev_cwd);
        pr::set_env("Path", prev_path.as_deref());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn job_object_limits_membership_query_and_active_limit() {
        unsafe {
            let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
            assert!(!job.is_null());

            // Initially current process is not in this job.
            let mut in_job: BOOL = TRUE;
            assert_eq!(IsProcessInJob(handles::CURRENT_PROCESS, job, &mut in_job), TRUE);
            assert_eq!(in_job, FALSE);

            // Set extended limits (KILL_ON_JOB_CLOSE | ACTIVE_PROCESS=1, ProcessMemoryLimit=64 MiB).
            let mut ext = [0u8; 144];
            let flags = JOB_OBJECT_LIMIT_ACTIVE_PROCESS | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            std::ptr::write_unaligned(ext.as_mut_ptr().add(16) as *mut u32, flags);
            std::ptr::write_unaligned(ext.as_mut_ptr().add(40) as *mut u32, 1u32); // ActiveProcessLimit = 1
            std::ptr::write_unaligned(ext.as_mut_ptr().add(112) as *mut usize, 64 * 1024 * 1024);
            std::ptr::write_unaligned(ext.as_mut_ptr().add(120) as *mut usize, 128 * 1024 * 1024);
            assert_eq!(SetInformationJobObject(job, 9, ext.as_ptr() as *const c_void, 144), TRUE);

            // Assign current process.
            assert_eq!(AssignProcessToJobObject(job, handles::CURRENT_PROCESS), TRUE);
            assert_eq!(IsProcessInJob(handles::CURRENT_PROCESS, job, &mut in_job), TRUE);
            assert_eq!(in_job, TRUE);
            assert_eq!(IsProcessInJob(handles::CURRENT_PROCESS, std::ptr::null_mut(), &mut in_job), TRUE);
            assert_eq!(in_job, TRUE);

            // Query back JobObjectExtendedLimitInformation (class 9).
            let mut out_ext = [0u8; 144];
            let mut ret_len = 0u32;
            assert_eq!(QueryInformationJobObject(job, 9, out_ext.as_mut_ptr() as *mut c_void, 144, &mut ret_len), TRUE);
            assert_eq!(ret_len, 144);
            assert_eq!(std::ptr::read_unaligned(out_ext.as_ptr().add(16) as *const u32), flags);
            assert_eq!(std::ptr::read_unaligned(out_ext.as_ptr().add(40) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(out_ext.as_ptr().add(112) as *const usize), 64 * 1024 * 1024);
            assert_eq!(std::ptr::read_unaligned(out_ext.as_ptr().add(120) as *const usize), 128 * 1024 * 1024);

            // Query JobObjectBasicAccountingInformation (class 1) and BasicAndIoAccounting (class 8)
            // at the exact Windows ABI offsets: +32 TotalPageFaultCount, +36 TotalProcesses,
            // +40 ActiveProcesses, +44 TotalTerminatedProcesses.
            let mut acct = [0u8; 48];
            assert_eq!(QueryInformationJobObject(job, 1, acct.as_mut_ptr() as *mut c_void, 48, &mut ret_len), TRUE);
            assert_eq!(ret_len, 48);
            assert_eq!(std::ptr::read_unaligned(acct.as_ptr().add(32) as *const u32), 0);
            assert_eq!(std::ptr::read_unaligned(acct.as_ptr().add(36) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(acct.as_ptr().add(40) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(acct.as_ptr().add(44) as *const u32), 0);

            let mut acct_io = [0u8; 96];
            assert_eq!(QueryInformationJobObject(job, 8, acct_io.as_mut_ptr() as *mut c_void, 96, &mut ret_len), TRUE);
            assert_eq!(ret_len, 96);
            assert_eq!(std::ptr::read_unaligned(acct_io.as_ptr().add(32) as *const u32), 0);
            assert_eq!(std::ptr::read_unaligned(acct_io.as_ptr().add(36) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(acct_io.as_ptr().add(40) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(acct_io.as_ptr().add(44) as *const u32), 0);

            // Query JobObjectBasicProcessIdList (class 3).
            let mut pid_list = [0u8; 32];
            assert_eq!(QueryInformationJobObject(job, 3, pid_list.as_mut_ptr() as *mut c_void, 32, &mut ret_len), TRUE);
            assert_eq!(std::ptr::read_unaligned(pid_list.as_ptr() as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(pid_list.as_ptr().add(4) as *const u32), 1);
            assert_eq!(std::ptr::read_unaligned(pid_list.as_ptr().add(8) as *const usize), std::process::id() as usize);

            // JobObjectCpuRateControlInformation (class 15) uses the 8-byte Windows ABI.
            let mut cpu_rate = [0u8; 8];
            std::ptr::write_unaligned(cpu_rate.as_mut_ptr() as *mut u32, 0x05u32); // ENABLE | HARD_CAP
            std::ptr::write_unaligned(cpu_rate.as_mut_ptr().add(4) as *mut u32, 5000u32); // 50%
            assert_eq!(SetInformationJobObject(job, 15, cpu_rate.as_ptr() as *const c_void, 12), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_BAD_LENGTH);
            assert_eq!(SetInformationJobObject(job, 15, cpu_rate.as_ptr() as *const c_void, 8), TRUE);
            let mut out_cpu = [0u8; 8];
            assert_eq!(QueryInformationJobObject(job, 15, out_cpu.as_mut_ptr() as *mut c_void, 8, &mut ret_len), TRUE);
            assert_eq!(ret_len, 8);
            assert_eq!(out_cpu, cpu_rate);

            // Wrong struct size -> ERROR_BAD_LENGTH.
            assert_eq!(SetInformationJobObject(job, 2, ext.as_ptr() as *const c_void, 16), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_BAD_LENGTH);

            // Unknown info class -> ERROR_INVALID_PARAMETER.
            assert_eq!(SetInformationJobObject(job, 999, ext.as_ptr() as *const c_void, 4), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Clear KILL_ON_JOB_CLOSE before closing in-process so the test runner continues
            // (self-termination on KILL_ON_JOB_CLOSE is tested in the subprocess test below).
            std::ptr::write_unaligned(ext.as_mut_ptr().add(16) as *mut u32, JOB_OBJECT_LIMIT_ACTIVE_PROCESS);
            assert_eq!(SetInformationJobObject(job, 9, ext.as_ptr() as *const c_void, 144), TRUE);
            assert_eq!(crate::kernel32::threads::CloseHandle(job), TRUE);

            // Named job objects release namespace ownership when the last handle closes.
            let jname = to_wide_z("Local\\KernelCoreNamedJobTest");
            let nj1 = CreateJobObjectW(std::ptr::null_mut(), jname.as_ptr());
            assert!(!nj1.is_null());
            assert_eq!(crate::teb::last_error(), 0);
            assert_eq!(SetInformationJobObject(nj1, 15, cpu_rate.as_ptr() as *const c_void, 8), TRUE);
            let nj2 = CreateJobObjectW(std::ptr::null_mut(), jname.as_ptr());
            assert!(!nj2.is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_ALREADY_EXISTS);
            assert_eq!(crate::kernel32::threads::CloseHandle(nj1), TRUE);
            assert_eq!(crate::kernel32::threads::CloseHandle(nj2), TRUE);
            // OpenJobObject fails once all handles have closed.
            assert!(OpenJobObjectW(0x1F001F, FALSE, jname.as_ptr()).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);
            // Recreating with the same name yields a brand-new job with fresh zeroed state.
            let nj3 = CreateJobObjectW(std::ptr::null_mut(), jname.as_ptr());
            assert!(!nj3.is_null());
            assert_eq!(crate::teb::last_error(), 0);
            let mut fresh_cpu = [0xFFu8; 8];
            assert_eq!(QueryInformationJobObject(nj3, 15, fresh_cpu.as_mut_ptr() as *mut c_void, 8, &mut ret_len), TRUE);
            assert_eq!(fresh_cpu, [0u8; 8]);
            assert_eq!(crate::kernel32::threads::CloseHandle(nj3), TRUE);
        }
    }

    pub(crate) fn make_test_child_handle(pid: u32, exe_win: &str) -> HANDLE {
        let obj = Arc::new(ChildProcessObj {
            pid,
            exe_win: exe_win.to_string(),
            exit_code: handles::Guarded::new(None),
            #[cfg(not(target_os = "ios"))]
            terminated_code: handles::Guarded::new(None),
            #[cfg(not(target_os = "ios"))]
            os_pid: std::sync::atomic::AtomicI32::new(0),
        });
        handles::insert(obj)
    }

    #[test]
    fn job_object_kill_on_close_terminates_current_process_on_last_handle() {
        const ENV_KEY: &str = "WIN32_RT_TEST_JOB_KILL_SELF";
        if std::env::var_os(ENV_KEY).is_some() {
            unsafe {
                let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
                assert!(!job.is_null());
                let mut ext = [0u8; 144];
                std::ptr::write_unaligned(
                    ext.as_mut_ptr().add(16) as *mut u32,
                    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                );
                assert_eq!(SetInformationJobObject(job, 9, ext.as_ptr() as *const c_void, 144), TRUE);
                assert_eq!(AssignProcessToJobObject(job, handles::CURRENT_PROCESS), TRUE);
                // Closing the last handle must terminate this process with code 1 and never return.
                let _ = crate::kernel32::threads::CloseHandle(job);
                std::process::exit(99);
            }
        }
        let exe = std::env::current_exe().expect("current_exe");
        let status = std::process::Command::new(exe)
            .env(ENV_KEY, "1")
            .arg("--exact")
            .arg("kernel32::process::tests::job_object_kill_on_close_terminates_current_process_on_last_handle")
            .arg("--nocapture")
            .status()
            .expect("spawn job kill-on-close child");
        assert_eq!(
            status.code(),
            Some(1),
            "closing last handle of KILL_ON_JOB_CLOSE job containing self must terminate process with code 1"
        );
    }

    #[test]
    fn format_message_from_module_rt_messagetable_unicode_and_ansi() {
        // Build a minimal in-memory PE32+ image containing an RT_MESSAGETABLE (type 11, id 1, lang 0x0409)
        // with two blocks: a Unicode block [0x1000..=0x1001] and an ANSI block [0x2000..=0x2000].
        let mut pe = vec![0u8; 0x2000];
        pe[0] = b'M';
        pe[1] = b'Z';
        pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes()); // e_lfanew = 0x80
        let nt = 0x80usize;
        pe[nt..nt + 4].copy_from_slice(b"PE\0\0");
        pe[nt + 4..nt + 6].copy_from_slice(&0x8664u16.to_le_bytes());
        pe[nt + 20..nt + 22].copy_from_slice(&0xF0u16.to_le_bytes()); // SizeOfOptionalHeader
        let opt = nt + 24;
        pe[opt..opt + 2].copy_from_slice(&0x020Bu16.to_le_bytes()); // PE32+
        pe[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        // DataDirectory[2] (Resource Table) at opt + 112 + 2*8 = opt + 128
        let rsrc_rva = 0x1000u32;
        pe[opt + 128..opt + 132].copy_from_slice(&rsrc_rva.to_le_bytes());
        pe[opt + 132..opt + 136].copy_from_slice(&0x1000u32.to_le_bytes());

        // 3-level resource directory at rsrc_rva:
        // +0x00: root dir (1 ID entry: type 11 -> subdir at +0x20)
        // +0x20: type dir (1 ID entry: id 1 -> subdir at +0x40)
        // +0x40: name dir (1 ID entry: lang 0x0409 -> data entry at +0x60)
        // +0x60: IMAGE_RESOURCE_DATA_ENTRY (rva = 0x1100, size = msg_table.len())
        let r = rsrc_rva as usize;
        // Root dir
        pe[r + 14..r + 16].copy_from_slice(&1u16.to_le_bytes());
        pe[r + 16..r + 20].copy_from_slice(&11u32.to_le_bytes()); // RT_MESSAGETABLE
        pe[r + 20..r + 24].copy_from_slice(&(0x8000_0000u32 | 0x20).to_le_bytes());
        // Type dir
        pe[r + 0x20 + 14..r + 0x20 + 16].copy_from_slice(&1u16.to_le_bytes());
        pe[r + 0x20 + 16..r + 0x20 + 20].copy_from_slice(&1u32.to_le_bytes()); // ID 1
        pe[r + 0x20 + 20..r + 0x20 + 24].copy_from_slice(&(0x8000_0000u32 | 0x40).to_le_bytes());
        // Name dir
        pe[r + 0x40 + 14..r + 0x40 + 16].copy_from_slice(&1u16.to_le_bytes());
        pe[r + 0x40 + 16..r + 0x40 + 20].copy_from_slice(&0x0409u32.to_le_bytes()); // en-US
        pe[r + 0x40 + 20..r + 0x40 + 24].copy_from_slice(&0x60u32.to_le_bytes());

        // Construct MESSAGE_RESOURCE_DATA
        let mut mt = Vec::<u8>::new();
        mt.extend_from_slice(&2u32.to_le_bytes()); // NumberOfBlocks = 2
        // Placeholder for 2 blocks (24 bytes), entries start at offset 28
        let blk0_off = 28u32;
        let msg_u0: Vec<u16> = "Hello %1!\0".encode_utf16().collect();
        let mut e0 = Vec::<u8>::new();
        let e0_len = ((4 + msg_u0.len() * 2 + 3) & !3) as u16;
        e0.extend_from_slice(&e0_len.to_le_bytes());
        e0.extend_from_slice(&1u16.to_le_bytes()); // MESSAGE_RESOURCE_UNICODE
        for u in &msg_u0 {
            e0.extend_from_slice(&u.to_le_bytes());
        }
        e0.resize(e0_len as usize, 0);

        let msg_u1: Vec<u16> = "Second=%1!u!\0".encode_utf16().collect();
        let mut e1 = Vec::<u8>::new();
        let e1_len = ((4 + msg_u1.len() * 2 + 3) & !3) as u16;
        e1.extend_from_slice(&e1_len.to_le_bytes());
        e1.extend_from_slice(&1u16.to_le_bytes());
        for u in &msg_u1 {
            e1.extend_from_slice(&u.to_le_bytes());
        }
        e1.resize(e1_len as usize, 0);

        let blk1_off = blk0_off + (e0.len() + e1.len()) as u32;
        let msg_a = b"Ansi code %1!x!\0";
        let mut e2 = Vec::<u8>::new();
        let e2_len = ((4 + msg_a.len() + 3) & !3) as u16;
        e2.extend_from_slice(&e2_len.to_le_bytes());
        e2.extend_from_slice(&0u16.to_le_bytes()); // ANSI
        e2.extend_from_slice(msg_a);
        e2.resize(e2_len as usize, 0);

        // Block 0: 0x1000..=0x1001
        mt.extend_from_slice(&0x1000u32.to_le_bytes());
        mt.extend_from_slice(&0x1001u32.to_le_bytes());
        mt.extend_from_slice(&blk0_off.to_le_bytes());
        // Block 1: 0x2000..=0x2000
        mt.extend_from_slice(&0x2000u32.to_le_bytes());
        mt.extend_from_slice(&0x2000u32.to_le_bytes());
        mt.extend_from_slice(&blk1_off.to_le_bytes());
        mt.extend_from_slice(&e0);
        mt.extend_from_slice(&e1);
        mt.extend_from_slice(&e2);

        // Data entry at r + 0x60 pointing to RVA 0x1100
        pe[r + 0x60..r + 0x64].copy_from_slice(&0x1100u32.to_le_bytes());
        pe[r + 0x64..r + 0x68].copy_from_slice(&(mt.len() as u32).to_le_bytes());
        pe[0x1100..0x1100 + mt.len()].copy_from_slice(&mt);

        unsafe {
            let hmod = pe.as_ptr() as *const c_void;
            let mut out = [0u16; 128];
            let arg_str = to_wide_z("World");
            let args1 = [arg_str.as_ptr() as u64];
            let n = FormatMessageW(
                FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_ARGUMENT_ARRAY,
                hmod,
                0x1000,
                0,
                out.as_mut_ptr(),
                out.len() as u32,
                args1.as_ptr(),
            );
            assert_eq!(from_wide(out.as_ptr()), "Hello World!");
            assert_eq!(n, 12);

            let args2 = [42u64];
            let n2 = FormatMessageW(
                FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_ARGUMENT_ARRAY,
                hmod,
                0x1001,
                0x0409,
                out.as_mut_ptr(),
                out.len() as u32,
                args2.as_ptr(),
            );
            assert_eq!(from_wide(out.as_ptr()), "Second=42");
            assert_eq!(n2, 9);

            // ANSI entry via FormatMessageA
            let mut out_a = [0u8; 128];
            let args3 = [0xABu64];
            let n3 = FormatMessageA(
                FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_ARGUMENT_ARRAY,
                hmod,
                0x2000,
                0,
                out_a.as_mut_ptr(),
                out_a.len() as u32,
                args3.as_ptr(),
            );
            assert_eq!(from_ansi(out_a.as_ptr()), "Ansi code ab");
            assert_eq!(n3, 12);

            // Fallback FROM_HMODULE | FROM_SYSTEM falls back to system table for ERROR_FILE_NOT_FOUND (2).
            let n_sys = FormatMessageW(
                FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                hmod,
                2,
                0,
                out.as_mut_ptr(),
                out.len() as u32,
                std::ptr::null(),
            );
            assert!(n_sys > 0);
            assert!(from_wide(out.as_ptr()).contains("file specified"));

            // Missing ID -> 0 and ERROR_MR_MID_NOT_FOUND (317).
            assert_eq!(
                FormatMessageW(
                    FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_IGNORE_INSERTS,
                    hmod,
                    0x9999,
                    0,
                    out.as_mut_ptr(),
                    out.len() as u32,
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_MR_MID_NOT_FOUND);
        }
    }
}
