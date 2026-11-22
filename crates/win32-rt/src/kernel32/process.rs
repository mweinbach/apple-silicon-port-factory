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

pub unsafe extern "win64" fn OpenProcess(_access: u32, _inherit: BOOL, pid: u32) -> HANDLE {
    if pid == std::process::id() {
        handles::insert(Arc::new(ProcessObj))
    } else {
        log::debug!("OpenProcess({pid}): not our process");
        set_err(if pid == 0 { err::ERROR_INVALID_PARAMETER } else { err::ERROR_ACCESS_DENIED });
        std::ptr::null_mut()
    }
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
    set_err(err::ERROR_ACCESS_DENIED);
    FALSE
}

pub unsafe extern "win64" fn GetExitCodeProcess(h: HANDLE, code: *mut u32) -> BOOL {
    if is_self(h) {
        *code = crate::kernel32::threads::STILL_ACTIVE;
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
    if !is_self(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let p = &pr::config().exe_path;
    let n = p.encode_utf16().count() as u32;
    if n + 1 > *size {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_wide_buf(p, buf, *size as usize);
    *size = n;
    TRUE
}

pub unsafe extern "win64" fn QueryFullProcessImageNameA(h: HANDLE, _flags: u32, buf: LPSTR, size: *mut u32) -> BOOL {
    if !is_self(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let p = &pr::config().exe_path;
    if p.len() as u32 + 1 > *size {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_ansi_buf(p, buf, *size as usize);
    *size = p.len() as u32;
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
// CreateProcess (not supported)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CreateProcessW(
    app: LPCWSTR,
    cmd: LPWSTR,
    _pa: *mut c_void,
    _ta: *mut c_void,
    _inherit: BOOL,
    _flags: u32,
    _env: *mut c_void,
    _cwd: LPCWSTR,
    _si: *mut c_void,
    _pi: *mut c_void,
) -> BOOL {
    stub!("CreateProcessW refused: app={:?} cmd={:?}", from_wide_opt(app), from_wide_opt(cmd));
    set_err(err::ERROR_NOT_SUPPORTED);
    FALSE
}

pub unsafe extern "win64" fn CreateProcessA(
    app: LPCSTR,
    cmd: LPSTR,
    _pa: *mut c_void,
    _ta: *mut c_void,
    _inherit: BOOL,
    _flags: u32,
    _env: *mut c_void,
    _cwd: LPCSTR,
    _si: *mut c_void,
    _pi: *mut c_void,
) -> BOOL {
    stub!("CreateProcessA refused: app={:?} cmd={:?}", from_ansi_opt(app), from_ansi_opt(cmd));
    set_err(err::ERROR_NOT_SUPPORTED);
    FALSE
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
}
