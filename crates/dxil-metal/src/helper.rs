//! Out-of-process conversion: the `dxil-metald` helper and its client.
//!
//! Why: the hybrid loader is an x86_64 process under Rosetta. Running Metal Shader Converter
//! (a large LLVM-based C++ library) there costs ~1.5x CPU (measured on the full game corpus:
//! 15.4 s arm64 vs 23.4 s x86_64), puts LLVM in the game's address space, and, more
//! importantly, any converter crash (4.0.1 segfaults on some inputs) would take the game down.
//! The helper is a native arm64 process; a crash costs one request, not the game.
//!
//! Protocol: the client spawns workers (`dxil-metald`, stdin/stdout pipes) and sends frames
//! `u32 len | payload`. Each worker handles one request at a time; the client keeps a pool of
//! up to N workers for parallel compiles. Requests: `Hello` (protocol check, returns the
//! converter fingerprint), `Compile`, `StageIn`. Replies: `0 | shader` or `1 | error`.
//! The client keeps the cache: only misses reach the helper, results decode in pure Rust,
//! and the client never loads the converter.

use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Once, OnceLock};
use std::time::{Duration, Instant};

use crate::compiler::{compile_uncached, stage_in_shader, CompileOptions, CompileRequest, CompiledShader};
use crate::reflection::{ShaderReflection, ShaderStage, StageInfo};
use crate::root_signature::{RootSignature, RootSignatureDesc};
use crate::wire::{self, Reader, Writer};
use crate::Error;

/// Bumped on any incompatible protocol change.
pub const PROTOCOL_VERSION: u32 = 1;
const MAGIC: u32 = u32::from_le_bytes(*b"AW2D");
const OP_HELLO: u8 = 1;
const OP_COMPILE: u8 = 2;
const OP_STAGE_IN: u8 = 3;
/// Makes the worker abort (exercises crash recovery in tests).
const OP_ABORT: u8 = 0xFF;
/// Makes the worker sleep `u32` ms (exercises the request timeout in tests).
const OP_SLEEP: u8 = 0xFE;
/// Default per-request timeout (`PORT_SHADERD_TIMEOUT_SECS` / `AW2_SHADERD_TIMEOUT_SECS`, 0 = none).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
/// Grace period given to a worker to exit cleanly after closing its stdin pipe before `SIGKILL`.
const GRACEFUL_EXIT_TIMEOUT: Duration = Duration::from_millis(200);
/// Largest frame accepted (DXIL containers and metallibs are far smaller).
const MAX_FRAME: usize = 256 << 20;

/// File name of the helper executable.
pub const HELPER_NAME: &str = "dxil-metald";

fn write_frame(w: &mut impl Write, payload: &[u8]) -> std::io::Result<()> {
    w.write_all(&(payload.len() as u32).to_le_bytes())?;
    w.write_all(payload)?;
    w.flush()
}

/// `Ok(None)` on clean EOF before a frame starts.
fn read_frame(r: &mut impl Read) -> std::io::Result<Option<Vec<u8>>> {
    let mut len = [0u8; 4];
    let mut got = 0;
    while got < 4 {
        match r.read(&mut len[got..])? {
            0 if got == 0 => return Ok(None),
            0 => return Err(std::io::ErrorKind::UnexpectedEof.into()),
            n => got += n,
        }
    }
    let n = u32::from_le_bytes(len) as usize;
    if n > MAX_FRAME {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("frame of {n} bytes")));
    }
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf)?;
    Ok(Some(buf))
}

fn reply(result: Result<Writer, Error>) -> Vec<u8> {
    match result {
        Ok(w) => {
            let mut out = Vec::with_capacity(w.buf.len() + 1);
            out.push(0);
            out.extend_from_slice(&w.buf);
            out
        }
        Err(e) => {
            let mut w = Writer::default();
            w.u8(1);
            wire::put_error(&mut w, &e);
            w.buf
        }
    }
}

// ---- server -----------------------------------------------------------------------------------

/// Helper main loop: serves requests from `input` until EOF or broken pipe. Used by `dxil-metald`.
pub fn serve(input: impl Read, output: impl Write) -> std::io::Result<()> {
    let mut input = BufReader::new(input);
    let mut output = BufWriter::new(output);
    let mut root_signatures: HashMap<u128, Arc<RootSignature>> = HashMap::new();
    while let Some(frame) = read_frame(&mut input)? {
        let out = reply(handle(&frame, &mut root_signatures));
        match write_frame(&mut output, &out) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => return Ok(()),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Installs signal handlers (`SIGTERM`/`SIGINT` -> clean exit 0) and a parent-death watchdog
/// thread (`kqueue` `EVFILT_PROC` + `NOTE_EXIT` on macOS, `getppid()` fallback elsewhere) so a
/// `dxil-metald` worker never lingers if its parent exits, catches Ctrl-C, or crashes (`SIGKILL` /
/// `SIGSEGV`).
pub fn install_daemon_lifecycle_guards() {
    #[cfg(unix)]
    sys::install_daemon_lifecycle_guards();
}

fn get_rs(cache: &mut HashMap<u128, Arc<RootSignature>>, rts0: Option<&[u8]>) -> Result<Option<Arc<RootSignature>>, Error> {
    let Some(b) = rts0 else { return Ok(None) };
    let desc = RootSignatureDesc::from_rts0(b)?;
    let h = desc.canonical_hash();
    if let Some(rs) = cache.get(&h) {
        return Ok(Some(rs.clone()));
    }
    let rs = RootSignature::new(desc)?;
    cache.insert(h, rs.clone());
    Ok(Some(rs))
}

fn handle(frame: &[u8], rs_cache: &mut HashMap<u128, Arc<RootSignature>>) -> Result<Writer, Error> {
    let mut r = Reader::new(frame);
    let mut w = Writer::default();
    match r.u8()? {
        OP_HELLO => {
            let (magic, version) = (r.u32()?, r.u32()?);
            if magic != MAGIC || version != PROTOCOL_VERSION {
                return Err(Error::Helper(format!("protocol mismatch: client v{version}, helper v{PROTOCOL_VERSION}")));
            }
            let api = crate::ffi::api()?;
            w.u32(PROTOCOL_VERSION).str(&api.fingerprint).u32(std::process::id());
        }
        OP_COMPILE => {
            let dxil = r.bytes()?;
            let entry = r.opt_string()?;
            let stage = r.opt_u32()?.map(ShaderStage::from_raw);
            let rs = get_rs(rs_cache, r.opt_bytes()?)?;
            let lrs = get_rs(rs_cache, r.opt_bytes()?)?;
            let options = wire::get_options(&mut r)?;
            let req = CompileRequest {
                dxil,
                entry_point: entry.as_deref(),
                expected_stage: stage,
                root_signature: rs.as_deref(),
                local_root_signature: lrs.as_deref(),
                options: &options,
            };
            let s = compile_uncached(&req)?;
            wire::put_shader(&mut w, &s);
        }
        OP_STAGE_IN => {
            let stage = ShaderStage::from_raw(r.u32()?);
            let json = r.string()?;
            let layout = wire::get_input_layout(&mut r)?;
            let options = wire::get_options(&mut r)?;
            let vertex = CompiledShader {
                metallib: Vec::new(),
                function_name: String::new(),
                stage,
                reflection: ShaderReflection {
                    stage,
                    function_name: String::new(),
                    resources: Vec::new(),
                    needs_function_constants: false,
                    function_constants: Vec::new(),
                    info: StageInfo::None,
                    json,
                },
                key: 0,
            };
            let s = stage_in_shader(&vertex, &layout, &options, 0)?;
            wire::put_shader(&mut w, &s);
        }
        OP_ABORT => std::process::exit(134),
        OP_SLEEP => std::thread::sleep(Duration::from_millis(r.u32()? as u64)),
        op => return Err(Error::InvalidInput(format!("unknown helper op {op}"))),
    }
    Ok(w)
}

// ---- platform process/watchdog primitives -----------------------------------------------------

#[cfg(unix)]
mod sys {
    use std::ffi::c_int;
    #[cfg(target_os = "macos")]
    use std::ffi::c_void;

    pub const SIGINT: c_int = 2;
    pub const SIGKILL: c_int = 9;
    pub const SIGTERM: c_int = 15;
    pub const ESRCH: i32 = 3;
    pub const EINTR: i32 = 4;
    pub const WNOHANG: c_int = 1;

    #[cfg(target_os = "macos")]
    pub const EVFILT_PROC: i16 = -5;
    #[cfg(target_os = "macos")]
    pub const EV_ADD: u16 = 0x0001;
    #[cfg(target_os = "macos")]
    pub const EV_ENABLE: u16 = 0x0004;
    #[cfg(target_os = "macos")]
    pub const EV_ONESHOT: u16 = 0x0010;
    #[cfg(target_os = "macos")]
    pub const EV_ERROR: u16 = 0x4000;
    #[cfg(target_os = "macos")]
    pub const NOTE_EXIT: u32 = 0x8000_0000;

    #[cfg(target_os = "macos")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Kevent {
        pub ident: usize,
        pub filter: i16,
        pub flags: u16,
        pub fflags: u32,
        pub data: isize,
        pub udata: *mut c_void,
    }

    #[cfg(target_os = "macos")]
    #[repr(C)]
    pub struct Timespec {
        pub tv_sec: isize,
        pub tv_nsec: isize,
    }

    extern "C" {
        pub fn getppid() -> c_int;
        pub fn kill(pid: c_int, sig: c_int) -> c_int;
        pub fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
        pub fn atexit(func: extern "C" fn()) -> c_int;
        pub fn signal(sig: c_int, handler: extern "C" fn(c_int)) -> usize;
        pub fn _exit(status: c_int) -> !;
        #[cfg(target_os = "macos")]
        pub fn close(fd: c_int) -> c_int;
        #[cfg(target_os = "macos")]
        pub fn kqueue() -> c_int;
        #[cfg(target_os = "macos")]
        pub fn kevent(
            kq: c_int,
            changelist: *const Kevent,
            nchanges: c_int,
            eventlist: *mut Kevent,
            nevents: c_int,
            timeout: *const Timespec,
        ) -> c_int;
    }

    extern "C" fn on_term_signal(_sig: c_int) {
        // SAFETY: `_exit` is async-signal-safe and avoids running C++ LLVM destructors in
        // `libmetalirconverter.dylib` concurrently with an in-flight compilation.
        unsafe { _exit(0) }
    }

    pub fn install_daemon_lifecycle_guards() {
        // SAFETY: installing an async-signal-safe handler for SIGTERM/SIGINT.
        unsafe {
            signal(SIGTERM, on_term_signal);
            signal(SIGINT, on_term_signal);
        }

        // SAFETY: `getppid` is always safe.
        let parent_pid = unsafe { getppid() };
        if parent_pid <= 1 {
            // Parent already died and we were reparented to PID 1 before startup completed.
            // SAFETY: immediate clean process exit.
            unsafe { _exit(0) }
        }

        let _ = std::thread::Builder::new()
            .name("dxil-metald-watchdog".into())
            .spawn(move || watch_parent(parent_pid));
    }

    fn watch_parent(parent_pid: c_int) -> ! {
        #[cfg(target_os = "macos")]
        {
            // SAFETY: `kqueue()` creates a new kernel event queue fd.
            let kq = unsafe { kqueue() };
            if kq >= 0 {
                let change = Kevent {
                    ident: parent_pid as usize,
                    filter: EVFILT_PROC,
                    flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
                    fflags: NOTE_EXIT,
                    data: 0,
                    udata: std::ptr::null_mut(),
                };
                // Step 1: Register the EVFILT_PROC knote without blocking.
                let registered = loop {
                    // SAFETY: valid kq fd and single Kevent struct.
                    let rc = unsafe {
                        kevent(kq, &change, 1, std::ptr::null_mut(), 0, std::ptr::null())
                    };
                    if rc == 0 {
                        break true;
                    }
                    match std::io::Error::last_os_error().raw_os_error() {
                        Some(EINTR) => continue,
                        Some(ESRCH) => {
                            // Parent already exited before kevent registration.
                            // SAFETY: immediate clean process exit.
                            unsafe { _exit(0) }
                        }
                        _ => break false,
                    }
                };
                if registered {
                    // Close the race window where the parent died between `getppid()` and
                    // `kevent()` registration (e.g. reparented to PID 1).
                    // SAFETY: `getppid` is always safe.
                    if unsafe { getppid() } != parent_pid {
                        // SAFETY: immediate clean process exit.
                        unsafe { _exit(0) }
                    }
                    let mut ev = Kevent {
                        ident: 0,
                        filter: 0,
                        flags: 0,
                        fflags: 0,
                        data: 0,
                        udata: std::ptr::null_mut(),
                    };
                    loop {
                        // SAFETY: blocks until parent exits (NOTE_EXIT) or interrupted by signal.
                        let n = unsafe {
                            kevent(kq, std::ptr::null(), 0, &mut ev, 1, std::ptr::null())
                        };
                        if n > 0 {
                            let exited = (ev.filter == EVFILT_PROC && (ev.fflags & NOTE_EXIT) != 0)
                                || ((ev.flags & EV_ERROR) != 0 && ev.data as i32 == ESRCH);
                            if exited {
                                // SAFETY: immediate clean process exit without C++ destructor races.
                                unsafe { _exit(0) }
                            }
                        } else if n < 0 {
                            if std::io::Error::last_os_error().raw_os_error() == Some(EINTR) {
                                continue;
                            }
                            break;
                        }
                    }
                }
                // SAFETY: close kq fd before falling back to polling.
                unsafe {
                    close(kq);
                }
            }
        }
        watch_parent_poll(parent_pid)
    }

    fn watch_parent_poll(parent_pid: c_int) -> ! {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            // SAFETY: `getppid` and `kill(pid, 0)` are plain read-only process queries.
            let gone = unsafe {
                getppid() != parent_pid
                    || (kill(parent_pid, 0) != 0
                        && std::io::Error::last_os_error().raw_os_error() == Some(ESRCH))
            };
            if gone {
                // SAFETY: immediate clean process exit.
                unsafe { _exit(0) }
            }
        }
    }

    pub fn register_atexit(func: extern "C" fn()) {
        // SAFETY: `func` is a valid C function pointer with static lifetime.
        unsafe {
            let _ = atexit(func);
        }
    }

    pub fn send_sigterm(pid: u32) {
        if let Ok(pid) = c_int::try_from(pid) {
            if pid > 0 {
                // SAFETY: signaling a tracked child worker PID.
                unsafe {
                    let _ = kill(pid, SIGTERM);
                }
            }
        }
    }

    pub fn try_reap_nonblocking(pid: u32) -> bool {
        let Ok(pid) = c_int::try_from(pid) else { return true };
        if pid <= 0 {
            return true;
        }
        let mut status: c_int = 0;
        // SAFETY: non-blocking waitpid on a tracked child PID.
        let rc = unsafe { waitpid(pid, &mut status, WNOHANG) };
        if rc > 0 {
            return true;
        }
        if rc == 0 {
            return false;
        }
        std::io::Error::last_os_error().raw_os_error() != Some(EINTR)
    }

    pub fn kill_and_reap(pid: u32) {
        let Ok(pid) = c_int::try_from(pid) else { return };
        if pid <= 0 {
            return;
        }
        // SAFETY: SIGKILL + blocking waitpid on an unreaped tracked child worker.
        unsafe {
            let _ = kill(pid, SIGKILL);
            let mut status: c_int = 0;
            while waitpid(pid, &mut status, 0) < 0 {
                if std::io::Error::last_os_error().raw_os_error() != Some(EINTR) {
                    break;
                }
            }
        }
    }
}

// ---- process-global worker tracking (`atexit` cleanup) ----------------------------------------

struct ActiveWorkerEntry {
    pid: u32,
    stdin: Arc<Mutex<Option<BufWriter<ChildStdin>>>>,
}

static ACTIVE_WORKERS: OnceLock<Mutex<HashMap<u64, ActiveWorkerEntry>>> = OnceLock::new();
static NEXT_WORKER_ID: AtomicU64 = AtomicU64::new(1);
static ATEXIT_INSTALLED: Once = Once::new();

fn active_workers_map() -> &'static Mutex<HashMap<u64, ActiveWorkerEntry>> {
    ACTIVE_WORKERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn register_active_worker(pid: u32, stdin: Arc<Mutex<Option<BufWriter<ChildStdin>>>>) -> u64 {
    #[cfg(unix)]
    ATEXIT_INSTALLED.call_once(|| sys::register_atexit(atexit_cleanup_workers));
    let id = NEXT_WORKER_ID.fetch_add(1, Ordering::Relaxed);
    let mut map = active_workers_map().lock().unwrap_or_else(|p| p.into_inner());
    map.insert(id, ActiveWorkerEntry { pid, stdin });
    id
}

fn unregister_active_worker(id: u64) {
    if let Some(map_mutex) = ACTIVE_WORKERS.get() {
        let mut map = map_mutex.lock().unwrap_or_else(|p| p.into_inner());
        map.remove(&id);
    }
}

#[cfg(unix)]
extern "C" fn atexit_cleanup_workers() {
    let Some(map_mutex) = ACTIVE_WORKERS.get() else {
        return;
    };
    let entries: Vec<ActiveWorkerEntry> = {
        let mut map = map_mutex.lock().unwrap_or_else(|p| p.into_inner());
        map.drain().map(|(_, v)| v).collect()
    };
    if entries.is_empty() {
        return;
    }

    // Phase 1: Close each worker's stdin pipe so idle workers exit cleanly on EOF.
    // If another thread died or is stuck holding the stdin lock, send SIGTERM instead of blocking.
    let mut pending_pids = Vec::with_capacity(entries.len());
    for entry in entries {
        match entry.stdin.try_lock() {
            Ok(mut guard) => {
                if let Some(mut w) = guard.take() {
                    let _ = w.flush();
                    drop(w);
                }
            }
            Err(_) => {
                sys::send_sigterm(entry.pid);
            }
        }
        pending_pids.push(entry.pid);
    }

    // Phase 2: Wait briefly for clean exit and reap zombies.
    let deadline = Instant::now() + GRACEFUL_EXIT_TIMEOUT;
    let mut poll_delay = Duration::from_millis(1);
    while !pending_pids.is_empty() {
        pending_pids.retain(|&pid| !sys::try_reap_nonblocking(pid));
        if pending_pids.is_empty() || Instant::now() >= deadline {
            break;
        }
        std::thread::sleep(poll_delay);
        poll_delay = Duration::from_millis(5);
    }

    // Phase 3: Force-kill and reap any remaining straggler.
    for pid in pending_pids {
        sys::kill_and_reap(pid);
    }
}

// ---- client -----------------------------------------------------------------------------------

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}
const POLLIN: i16 = 0x1;
extern "C" {
    fn poll(fds: *mut PollFd, nfds: u32, timeout: i32) -> i32;
}

/// Child stdout with an optional read deadline (poll(2) before every read).
struct TimedReader {
    inner: ChildStdout,
    deadline: Option<Instant>,
}

impl Read for TimedReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        use std::os::fd::AsRawFd;
        if let Some(deadline) = self.deadline {
            loop {
                let left = deadline.saturating_duration_since(Instant::now());
                if left.is_zero() {
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let mut fd = PollFd { fd: self.inner.as_raw_fd(), events: POLLIN, revents: 0 };
                let ms = left.as_millis().clamp(1, i32::MAX as u128) as i32;
                // SAFETY: one valid pollfd.
                let n = unsafe { poll(&mut fd, 1, ms) };
                if n > 0 {
                    break; // readable, hung up or error: let read() report which
                }
                if n < 0 {
                    let e = std::io::Error::last_os_error();
                    if e.kind() != std::io::ErrorKind::Interrupted {
                        return Err(e);
                    }
                }
            }
        }
        self.inner.read(buf)
    }
}

struct Worker {
    registry_id: u64,
    pid: u32,
    child: Child,
    stdin: Arc<Mutex<Option<BufWriter<ChildStdin>>>>,
    stdout: Option<BufReader<TimedReader>>,
    reaped: bool,
    kill_immediately: bool,
}

impl Worker {
    fn spawn(exe: &Path, timeout: Option<Duration>) -> Result<(Self, String), Error> {
        let mut child = Command::new(exe)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .env_remove("DYLD_INSERT_LIBRARIES")
            .spawn()
            .map_err(|e| Error::Helper(format!("spawn {}: {e}", exe.display())))?;
        let pid = child.id();
        let stdin = Arc::new(Mutex::new(Some(BufWriter::new(child.stdin.take().unwrap()))));
        let stdout = Some(BufReader::new(TimedReader { inner: child.stdout.take().unwrap(), deadline: None }));
        let registry_id = register_active_worker(pid, stdin.clone());
        let mut w = Worker {
            registry_id,
            pid,
            child,
            stdin,
            stdout,
            reaped: false,
            kill_immediately: false,
        };
        let mut req = Writer::default();
        req.u8(OP_HELLO).u32(MAGIC).u32(PROTOCOL_VERSION);
        let reply = w.call(&req.buf, timeout)?;
        let mut r = Reader::new(&reply[1..]);
        let _version = r.u32()?;
        let fingerprint = r.string()?;
        Ok((w, fingerprint))
    }

    fn close_stdin(&self) {
        if let Ok(mut guard) = self.stdin.lock() {
            if let Some(mut w) = guard.take() {
                let _ = w.flush();
                drop(w);
            }
        }
    }

    fn transport_error(&mut self, e: std::io::Error) -> Error {
        let status = match self.child.try_wait() {
            Ok(Some(s)) => {
                // Child has been reaped; unregister immediately so its PID is never signaled later.
                self.reaped = true;
                unregister_active_worker(self.registry_id);
                s.to_string()
            }
            _ => "running".to_string(),
        };
        Error::Helper(format!("{HELPER_NAME} transport error: {e} (helper {status})"))
    }

    /// Sends one request and returns the reply frame (`frame[0] == 0`; payload at `[1..]`).
    /// Helper-side failures come back as their original `Error`; transport failures (the
    /// helper crashed, closed the pipe or exceeded `timeout`) are `Error::Helper`; the caller
    /// must then drop the worker (which kills it).
    fn call(&mut self, payload: &[u8], timeout: Option<Duration>) -> Result<Vec<u8>, Error> {
        let write_res = {
            let mut guard = self.stdin.lock().unwrap_or_else(|p| p.into_inner());
            match guard.as_mut() {
                Some(w) => write_frame(w, payload),
                None => Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "helper stdin closed")),
            }
        };
        if let Err(e) = write_res {
            return Err(self.transport_error(e));
        }
        let Some(stdout) = self.stdout.as_mut() else {
            return Err(self.transport_error(std::io::ErrorKind::BrokenPipe.into()));
        };
        stdout.get_mut().deadline = timeout.map(|t| Instant::now() + t);
        let frame = read_frame(stdout);
        stdout.get_mut().deadline = None;
        let frame = match frame {
            Ok(Some(f)) if !f.is_empty() => f,
            Ok(_) => return Err(self.transport_error(std::io::ErrorKind::UnexpectedEof.into())),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                self.kill_immediately = true;
                return Err(Error::Helper(format!(
                    "{HELPER_NAME} request timed out after {:.1}s; worker killed",
                    timeout.unwrap_or_default().as_secs_f64()
                )));
            }
            Err(e) => return Err(self.transport_error(e)),
        };
        if frame[0] == 0 {
            Ok(frame)
        } else {
            Err(wire::get_error(&mut Reader::new(&frame[1..]))?)
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        unregister_active_worker(self.registry_id);
        // Close stdin/stdout pipes first so an idle worker sees EOF and exits cleanly.
        self.close_stdin();
        drop(self.stdout.take());
        if self.reaped {
            return;
        }
        if !self.kill_immediately {
            let deadline = Instant::now() + GRACEFUL_EXIT_TIMEOUT;
            let mut poll_delay = Duration::from_millis(1);
            loop {
                match self.child.try_wait() {
                    Ok(Some(_)) => {
                        self.reaped = true;
                        return;
                    }
                    Ok(None) => {
                        if Instant::now() >= deadline {
                            break;
                        }
                        std::thread::sleep(poll_delay);
                        poll_delay = Duration::from_millis(5);
                    }
                    Err(_) => break,
                }
            }
        }
        // Still alive after grace period (or timed out): SIGKILL and reap so no zombie remains.
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.reaped = true;
    }
}

struct PoolState {
    idle: Vec<Worker>,
    live: usize,
}

/// Pool of `dxil-metald` worker processes.
pub struct HelperClient {
    exe: PathBuf,
    fingerprint: String,
    max_workers: usize,
    /// Per-request timeout in ms (0 = none).
    timeout_ms: AtomicU64,
    state: Mutex<PoolState>,
    available: Condvar,
}

impl HelperClient {
    /// Spawns the first worker (verifying the protocol) and returns the client. The request
    /// timeout comes from `PORT_SHADERD_TIMEOUT_SECS` / `AW2_SHADERD_TIMEOUT_SECS`
    /// (default [`DEFAULT_TIMEOUT`], 0 = none).
    pub fn connect(exe: &Path, max_workers: usize) -> Result<Self, Error> {
        let timeout = match std::env::var("PORT_SHADERD_TIMEOUT_SECS")
            .or_else(|_| std::env::var("AW2_SHADERD_TIMEOUT_SECS"))
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
        {
            Some(s) if s > 0.0 => Some(Duration::from_secs_f64(s)),
            Some(_) => None,
            None => Some(DEFAULT_TIMEOUT),
        };
        let (w, fingerprint) = Worker::spawn(exe, timeout)?;
        Ok(Self {
            exe: exe.to_path_buf(),
            fingerprint,
            max_workers: max_workers.max(1),
            timeout_ms: AtomicU64::new(timeout.map_or(0, |t| t.as_millis().max(1) as u64)),
            state: Mutex::new(PoolState { idle: vec![w], live: 1 }),
            available: Condvar::new(),
        })
    }

    /// Cleanly shuts down all currently idle worker processes in parallel (closes their stdin
    /// pipes simultaneously, waits briefly for EOF exit, then force-kills any straggler).
    pub fn shutdown(&self) {
        let drained: Vec<Worker> = {
            let mut st = self.state.lock().unwrap_or_else(|p| p.into_inner());
            let workers: Vec<Worker> = st.idle.drain(..).collect();
            st.live = st.live.saturating_sub(workers.len());
            workers
        };
        // Signal EOF to every worker in parallel before waiting/reaping each one.
        for w in &drained {
            w.close_stdin();
        }
        drop(drained);
        self.available.notify_all();
    }

    /// Converter fingerprint reported by the helper (cache namespace / key component).
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
    pub fn executable(&self) -> &Path {
        &self.exe
    }
    /// Per-request timeout; a worker that exceeds it is killed (the request fails with
    /// `Error::Helper`) and replaced on demand. `None` disables it.
    pub fn timeout(&self) -> Option<Duration> {
        match self.timeout_ms.load(Ordering::Relaxed) {
            0 => None,
            ms => Some(Duration::from_millis(ms)),
        }
    }
    pub fn set_timeout(&self, timeout: Option<Duration>) {
        self.timeout_ms.store(timeout.map_or(0, |t| t.as_millis().max(1) as u64), Ordering::Relaxed);
    }

    fn acquire(&self) -> Result<Worker, Error> {
        let mut st = self.state.lock().unwrap();
        loop {
            if let Some(w) = st.idle.pop() {
                return Ok(w);
            }
            if st.live < self.max_workers {
                st.live += 1;
                drop(st);
                return match Worker::spawn(&self.exe, self.timeout()) {
                    Ok((w, _)) => Ok(w),
                    Err(e) => {
                        self.state.lock().unwrap().live -= 1;
                        self.available.notify_one();
                        Err(e)
                    }
                };
            }
            st = self.available.wait(st).unwrap();
        }
    }

    fn release(&self, w: Option<Worker>) {
        let mut st = self.state.lock().unwrap();
        match w {
            Some(w) => st.idle.push(w),
            None => st.live -= 1,
        }
        drop(st);
        self.available.notify_one();
    }

    fn request(&self, payload: &[u8], key: u128) -> Result<CompiledShader, Error> {
        let mut w = self.acquire()?;
        match w.call(payload, self.timeout()) {
            Ok(frame) => {
                self.release(Some(w));
                wire::get_shader(&mut Reader::new(&frame[1..]), key)
            }
            Err(e @ Error::Helper(_)) => {
                // The worker is dead or desynchronized: drop it (a new one is spawned on demand).
                log::error!(target: "shaders", "{e}");
                drop(w);
                self.release(None);
                Err(e)
            }
            Err(e) => {
                self.release(Some(w));
                Err(e)
            }
        }
    }

    /// Remote [`compile_uncached`]; the result carries `key`.
    pub fn compile(&self, req: &CompileRequest, key: u128) -> Result<CompiledShader, Error> {
        let mut w = Writer::with_capacity(req.dxil.len() + 1024);
        w.u8(OP_COMPILE).bytes(req.dxil).opt_str(req.entry_point).opt_u32(req.expected_stage.map(|s| s.to_raw()));
        let rs = req.root_signature.map(|r| r.desc().to_rts0());
        let lrs = req.local_root_signature.map(|r| r.desc().to_rts0());
        w.opt_bytes(rs.as_deref()).opt_bytes(lrs.as_deref());
        wire::put_options(&mut w, req.options);
        self.request(&w.buf, key)
    }

    /// Remote stage-in synthesis.
    pub fn stage_in(&self, vertex: &CompiledShader, layout: &[crate::InputElement], options: &CompileOptions, key: u128) -> Result<CompiledShader, Error> {
        let mut w = Writer::default();
        w.u8(OP_STAGE_IN).u32(vertex.stage.to_raw()).str(&vertex.reflection.json);
        wire::put_input_layout(&mut w, layout);
        wire::put_options(&mut w, options);
        self.request(&w.buf, key)
    }

    /// Number of live worker processes.
    pub fn workers(&self) -> usize {
        self.state.lock().unwrap().live
    }

    /// Test hook: returns OS PIDs of currently idle worker processes in the pool.
    #[doc(hidden)]
    pub fn worker_pids(&self) -> Vec<u32> {
        self.state.lock().unwrap().idle.iter().map(|w| w.pid).collect()
    }

    /// Test hook: makes one worker sleep `ms` milliseconds inside a request.
    #[doc(hidden)]
    pub fn sleep_one_worker(&self, ms: u32) -> Result<(), Error> {
        let mut w = Writer::default();
        w.u8(OP_SLEEP).u32(ms);
        match self.request(&w.buf, 0) {
            // The (empty) reply doesn't decode as a shader; reaching the decoder means success.
            Err(Error::Io(_)) | Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Test hook: makes one worker abort mid-request, as a converter crash would. Returns the
    /// resulting error (always `Error::Helper`).
    #[doc(hidden)]
    pub fn crash_one_worker(&self) -> Error {
        match self.request(&[OP_ABORT], 0) {
            Err(e) => e,
            Ok(_) => Error::Helper("worker did not crash".into()),
        }
    }
}

impl Drop for HelperClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Finds the helper: `PORT_SHADERD`, `AW2_SHADERD`, next to the current executable (or its parent
/// for `target/<triple>/<profile>` layouts), then this workspace's arm64 `target/` dirs.
pub fn find_helper() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_SHADERD")
        .filter(|v| !v.is_empty())
        .or_else(|| std::env::var_os("AW2_SHADERD").filter(|v| !v.is_empty()))
    {
        let p = PathBuf::from(p);
        return p.is_file().then_some(p);
    }
    let mut cands = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            cands.push(dir.join(HELPER_NAME));
            // target/x86_64-apple-darwin/debug/aw2-loader -> target/debug/dxil-metald
            if let (Some(profile), Some(target)) = (dir.file_name(), dir.parent().and_then(Path::parent)) {
                cands.push(target.join(profile).join(HELPER_NAME));
                cands.push(target.join("aarch64-apple-darwin").join(profile).join(HELPER_NAME));
            }
        }
    }
    let ws_target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
    let mut ws_cands = Vec::new();
    for profile in ["release", "debug"] {
        ws_cands.push(ws_target.join(profile).join(HELPER_NAME));
        ws_cands.push(ws_target.join("aarch64-apple-darwin").join(profile).join(HELPER_NAME));
    }
    ws_cands.retain(|p| p.is_file());
    ws_cands.sort_by_key(|p| std::cmp::Reverse(p.metadata().and_then(|m| m.modified()).ok()));
    cands.extend(ws_cands);
    cands.into_iter().find(|p| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_exits_cleanly_on_eof() {
        let mut out = Vec::new();
        serve(std::io::Cursor::new(Vec::<u8>::new()), &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn serve_exits_cleanly_on_broken_pipe() {
        struct BrokenPipeWriter;
        impl Write for BrokenPipeWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
            }
        }

        // Send a minimal valid frame (unknown op -> error reply frame written to BrokenPipeWriter).
        let mut input = Vec::new();
        write_frame(&mut input, &[0x7F]).unwrap();
        serve(std::io::Cursor::new(input), BrokenPipeWriter).unwrap();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn kevent_abi_layout_matches_macos_lp64() {
        assert_eq!(std::mem::size_of::<sys::Kevent>(), 32);
        assert_eq!(std::mem::align_of::<sys::Kevent>(), 8);
    }
}
