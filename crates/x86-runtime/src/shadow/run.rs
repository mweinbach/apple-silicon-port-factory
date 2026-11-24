//! Probe runs: buffers, sandbox and the forked run with dirty-page copy-out.

use super::handler::{self, page, RunHdr, HDR_BYTES};
use win32_rt::libc;
use win32_rt::shadow::HOST_API_LEN;
use std::sync::atomic::{AtomicI32, Ordering};

/// Map `len` bytes of anonymous memory (`shared`: MAP_SHARED, written by runs, read by the
/// orchestrator).
pub unsafe fn map_anon(len: usize, shared: bool) -> usize {
    let flags = libc::MAP_ANON | if shared { libc::MAP_SHARED } else { libc::MAP_PRIVATE };
    let p = libc::mmap(core::ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, flags, -1, 0);
    if p == libc::MAP_FAILED {
        0
    } else {
        p as usize
    }
}

pub fn now_us() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    ts.tv_sec as u64 * 1_000_000 + ts.tv_nsec as u64 / 1000
}

#[cfg(target_os = "macos")]
extern "C" {
    fn proc_pidfdinfo(pid: i32, fd: i32, flavor: i32, buffer: *mut libc::c_void, buffersize: i32) -> i32;
}

/// Close every unguarded file/socket/pipe fd except `keep`; /dev/null on 0..=2. Guarded fds
/// (libdispatch, os_log, ...) and kqueues stay: closing a guarded fd is a fatal EXC_GUARD.
pub unsafe fn sandbox_fds(keep: i32) {
    let null = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR);
    if null >= 0 {
        for fd in 0..3 {
            if fd != null {
                libc::dup2(null, fd);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        const PROX_FDTYPE_VNODE: u32 = 1;
        const PROX_FDTYPE_SOCKET: u32 = 2;
        const PROX_FDTYPE_PIPE: u32 = 6;
        const PROC_PIDFDVNODEINFO: i32 = 1;
        const PROC_PIDFDSOCKETINFO: i32 = 3;
        const PROC_PIDFDPIPEINFO: i32 = 6;
        let rec = core::mem::size_of::<libc::proc_fdinfo>();
        let mut buf = vec![0u8; 16384 * rec];
        let n = libc::proc_pidinfo(
            libc::getpid(),
            libc::PROC_PIDLISTFDS,
            0,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len() as i32,
        );
        if n <= 0 {
            return;
        }
        let mut info = vec![0u64; 2048];
        for i in 0..n as usize / rec {
            let e = &*(buf.as_ptr().add(i * rec) as *const libc::proc_fdinfo);
            let fd = e.proc_fd;
            if fd <= 2 || fd == keep || fd == null {
                continue;
            }
            let flavor = match e.proc_fdtype {
                PROX_FDTYPE_VNODE => PROC_PIDFDVNODEINFO,
                PROX_FDTYPE_SOCKET => PROC_PIDFDSOCKETINFO,
                PROX_FDTYPE_PIPE => PROC_PIDFDPIPEINFO,
                _ => continue,
            };
            let got = proc_pidfdinfo(
                libc::getpid(),
                fd,
                flavor,
                info.as_mut_ptr() as *mut libc::c_void,
                (info.len() * 8) as i32,
            );
            // struct proc_fileinfo: fi_guardflags at +20.
            if got >= 24 && *((info.as_ptr() as *const u8).add(20) as *const u32) == 0 {
                libc::close(fd);
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = keep;
    }
}

pub const HEAP_LEN: usize = 256 << 20;

/// Run buffers: G1, N, G2, then the poisoned G3 and N3 (only when G1 and N differ).
pub const NRUNS: usize = 5;

/// Buffers of one probe (mapped by the orchestrator before it protects memory).
pub struct Bufs {
    pub run: [usize; NRUNS],
    pub run_len: usize,
    pub maxpages: usize,
    /// Guest heap arena of the runs (tracked; pristine = zeros).
    pub heap: usize,
    /// Pristine copies of the union pages (orchestrator private).
    pub pristine: usize,
    pub pristine_len: usize,
    /// Host malloc arena of the runs ([`super::hostalloc`]; never tracked).
    pub host: usize,
}

impl Bufs {
    pub unsafe fn new(maxpages: usize) -> Option<Bufs> {
        let pg = page();
        let run_len = writers_off(maxpages) + maxpages * 16;
        let mut run = [0usize; NRUNS];
        for r in run.iter_mut() {
            *r = map_anon(run_len, true);
            if *r == 0 {
                return None;
            }
        }
        let heap = map_anon(HEAP_LEN, false);
        let host = map_anon(HEAP_LEN, false);
        let pristine_len = NRUNS * maxpages * pg + NRUNS * maxpages * 8;
        let pristine = map_anon(pristine_len, false);
        if heap == 0 || pristine == 0 || host == 0 {
            return None;
        }
        Some(Bufs { run, run_len, maxpages, heap, pristine, pristine_len, host })
    }

    /// Copy the orchestrator's (pristine) content of every page any run dirtied into the
    /// pristine area. Allocation-free: must run before the orchestrator touches its heap.
    /// Duplicates across runs are copied twice (same content).
    pub unsafe fn snapshot_pristine(&self) {
        let pg = page();
        let addrs = (self.pristine + NRUNS * self.maxpages * pg) as *mut u64;
        let mut j = 0usize;
        for r in 0..NRUNS {
            let n = ((*(self.run[r] as *const RunHdr)).npages as usize).min(self.maxpages);
            let list = (self.run[r] + HDR_BYTES) as *const u64;
            for i in 0..n {
                let a = *list.add(i);
                core::ptr::copy_nonoverlapping(a as *const u8, (self.pristine + j * pg) as *mut u8, pg);
                *addrs.add(j) = a;
                j += 1;
            }
        }
        for k in j..NRUNS * self.maxpages {
            *addrs.add(k) = 0;
        }
    }

    /// Whether runs `a` and `b` left any tracked byte outside `soft` different (a page only
    /// one of them dirtied is compared with the orchestrator's own, still pristine, copy).
    /// Allocation-free (before [`Bufs::snapshot_pristine`]).
    pub unsafe fn pages_differ(&self, a: usize, b: usize, soft: &[(usize, usize)]) -> bool {
        let pg = page();
        let page_of = |r: usize, addr: u64| -> *const u8 {
            let n = (self.hdr(r).npages as usize).min(self.maxpages);
            let list = (self.run[r] + HDR_BYTES) as *const u64;
            let data = self.run[r] + data_off(self.maxpages);
            for i in 0..n {
                if *list.add(i) == addr {
                    return (data + i * pg) as *const u8;
                }
            }
            addr as *const u8
        };
        for (x, y) in [(a, b), (b, a)] {
            let n = (self.hdr(x).npages as usize).min(self.maxpages);
            let list = (self.run[x] + HDR_BYTES) as *const u64;
            for i in 0..n {
                let addr = *list.add(i);
                let p = core::slice::from_raw_parts(page_of(x, addr), pg);
                let q = core::slice::from_raw_parts(page_of(y, addr), pg);
                if p == q {
                    continue;
                }
                for k in 0..pg {
                    if p[k] != q[k] {
                        let at = addr as usize + k;
                        if !soft.iter().any(|&(lo, hi)| at >= lo && at < hi) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Ranges never tracked (run buffers, pristine area, host arena; not the guest heap arena).
    pub fn own_ranges(&self) -> [(usize, usize); NRUNS + 2] {
        let mut o = [(0usize, 0usize); NRUNS + 2];
        for (r, b) in self.run.iter().enumerate() {
            o[r] = (*b, *b + self.run_len);
        }
        o[NRUNS] = (self.pristine, self.pristine + self.pristine_len);
        o[NRUNS + 1] = (self.host, self.host + HEAP_LEN);
        o
    }

    pub unsafe fn hdr(&self, r: usize) -> &RunHdr {
        &*(self.run[r] as *const RunHdr)
    }

    /// Run `r`'s dirty pages (page address, data pointer), sorted by address.
    pub unsafe fn pages(&self, r: usize) -> Vec<(u64, *const u8)> {
        let n = (self.hdr(r).npages as usize).min(self.maxpages);
        let list = (self.run[r] + HDR_BYTES) as *const u64;
        let data = self.run[r] + data_off(self.maxpages);
        let mut v: Vec<(u64, *const u8)> = (0..n).map(|i| (*list.add(i), (data + i * page()) as *const u8)).collect();
        v.sort_unstable_by_key(|p| p.0);
        v.dedup_by_key(|p| p.0);
        v
    }
}

/// Offset of the page data in a run buffer.
pub fn data_off(maxpages: usize) -> usize {
    let pg = page();
    HDR_BYTES + ((maxpages * 8 + pg - 1) & !(pg - 1))
}

/// Offset of the baseline copies (pre-write page content, [`handler::RUN_BASE`]).
pub fn base_off(maxpages: usize) -> usize {
    data_off(maxpages) + maxpages * page()
}

/// Offset of the first-writer `(pc, lr)` pairs ([`handler::RUN_WRITERS`]).
pub fn writers_off(maxpages: usize) -> usize {
    base_off(maxpages) + maxpages * page()
}

impl Bufs {
    /// `(pc, lr)` of run `r`'s first write to the page holding `addr` (None: not dirtied).
    pub unsafe fn first_writer(&self, r: usize, addr: usize) -> Option<(u64, u64)> {
        let pa = (addr & !(page() - 1)) as u64;
        let n = (self.hdr(r).npages as usize).min(self.maxpages);
        let list = (self.run[r] + HDR_BYTES) as *const u64;
        let w = (self.run[r] + writers_off(self.maxpages)) as *const u64;
        (0..n).find(|&i| *list.add(i) == pa).map(|i| (*w.add(2 * i), *w.add(2 * i + 1)))
    }
}

pub const ST_OK: u8 = 0;
pub const ST_TIMEOUT: u8 = 1;
pub const ST_FAULT: u8 = 2;
pub const ST_SHARED: u8 = 3;
pub const ST_OVERFLOW: u8 = 4;
pub const ST_FORK: u8 = 5;
/// The run reached a host API that cannot work in a forked child
/// ([`win32_rt::shadow::guard_host`]).
pub const ST_HOST: u8 = 6;

/// Orchestrator `_exit` status when the game/test process is gone.
pub const EXIT_PARENT_GONE: i32 = 93;

/// Pid of the orchestrator's parent (game/test process), recorded at orchestrator start.
pub static PARENT: AtomicI32 = AtomicI32::new(0);

/// The orchestrator's parent exited (it is reparented to launchd): nobody will read the
/// result, and the runs must not outlive the game.
pub fn parent_gone() -> bool {
    let p = PARENT.load(Ordering::Relaxed);
    p != 0 && unsafe { libc::getppid() } != p
}

/// Hard limit for this child: `SIGALRM` (default action: terminate, no crash report) after
/// `ms`, independent of the process that would otherwise kill it. Timers are not inherited
/// across `fork`, so every orchestrator and run arms its own.
pub unsafe fn arm_watchdog(ms: u64) {
    let mut set: libc::sigset_t = core::mem::zeroed();
    libc::sigemptyset(&mut set);
    libc::sigaddset(&mut set, libc::SIGALRM);
    libc::pthread_sigmask(libc::SIG_UNBLOCK, &set, core::ptr::null_mut());
    libc::signal(libc::SIGALRM, libc::SIG_DFL);
    let it = libc::itimerval {
        it_interval: libc::timeval { tv_sec: 0, tv_usec: 0 },
        it_value: libc::timeval { tv_sec: (ms / 1000) as _, tv_usec: ((ms % 1000) * 1000) as _ },
    };
    libc::setitimer(libc::ITIMER_REAL, &it, core::ptr::null_mut());
}

/// Run watchdog: well past the orchestrator's own `timeout_ms` kill (it only fires when the
/// orchestrator is gone or stuck). Extra 5 s slack keeps VM scheduling pauses on hosted CI
/// runners from firing `SIGALRM` ahead of the orchestrator's `timeout_ms` check.
pub fn run_watchdog_ms(timeout_ms: u64) -> u64 {
    timeout_ms.saturating_mul(2).saturating_add(5_000)
}

/// Call `waitpid`, retrying on `EINTR`. Returns `(w, errno_if_neg)`.
pub(crate) unsafe fn waitpid_retry(pid: i32, status: &mut i32, options: i32) -> (i32, i32) {
    loop {
        let w = libc::waitpid(pid, status, options);
        if w < 0 {
            let err = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            if err == libc::EINTR {
                continue;
            }
            return (w, err);
        }
        return (w, 0);
    }
}

/// Result of one run.
#[derive(Clone, Copy, Default, Debug)]
pub struct RunRes {
    pub status: u8,
    pub rax: u64,
    pub xmm0: u128,
    pub t_us: u64,
    pub fault_addr: u64,
    pub fault_pc: u64,
    /// Raw `waitpid` status (or `-errno` when `waitpid` failed).
    pub wait_status: i32,
    /// Signal recorded by `on_fault` (`RunHdr::fault_sig`, 0 if `on_fault` did not exit).
    pub fault_sig: u32,
    /// [`ST_HOST`]: the host API (NUL-terminated).
    pub host_api: [u8; HOST_API_LEN],
}

/// Tests: `fn()` called in each run child after its fork and before tracking starts.
#[cfg(test)]
pub static TEST_PRE_TRACK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Fork one run of `body` with dirty tracking into `bufs.run[r]`; wait up to `timeout_ms`.
pub unsafe fn run_one(bufs: &Bufs, r: usize, timeout_ms: u64, body: &dyn Fn() -> (u64, u128)) -> RunRes {
    let buf = bufs.run[r];
    let h = buf as *mut RunHdr;
    core::ptr::write_bytes(h as *mut u8, 0, core::mem::size_of::<RunHdr>());
    let pid = libc::fork();
    if pid < 0 {
        return RunRes { status: ST_FORK, ..Default::default() };
    }
    if pid == 0 {
        arm_watchdog(run_watchdog_ms(timeout_ms));
        win32_rt::shadow::set_forbid_record(core::ptr::addr_of_mut!((*h).host_api) as usize);
        handler::RUN_BUF.store(buf, Ordering::Relaxed);
        handler::RUN_MAX.store(bufs.maxpages, Ordering::Relaxed);
        handler::RUN_BASE.store(buf + base_off(bufs.maxpages), Ordering::Relaxed);
        handler::RUN_WRITERS.store(buf + writers_off(bufs.maxpages), Ordering::Relaxed);
        super::hostalloc::enable();
        #[cfg(test)]
        {
            let f = TEST_PRE_TRACK.load(Ordering::Relaxed);
            if f != 0 {
                core::mem::transmute::<usize, fn()>(f)();
            }
        }
        handler::MODE.store(handler::MODE_RUN, Ordering::Relaxed);
        // Tracking starts here: every later write to a tracked page is recorded.
        let rp = handler::RANGES_PTR.load(Ordering::Relaxed) as *const [usize; 3];
        let rn = handler::RANGES_LEN.load(Ordering::Relaxed);
        if !rp.is_null() {
            super::memmap::protect(core::slice::from_raw_parts(rp, rn));
        }
        let t0 = now_us();
        let (rax, xmm0) = body();
        let t1 = now_us();
        handler::MODE.store(handler::MODE_OFF, Ordering::Relaxed);
        let n = ((*h).npages as usize).min(bufs.maxpages);
        let list = (buf + HDR_BYTES) as *const u64;
        let data = buf + data_off(bufs.maxpages);
        let pg = page();
        for i in 0..n {
            core::ptr::copy_nonoverlapping(*list.add(i) as *const u8, (data + i * pg) as *mut u8, pg);
        }
        (*h).rax = rax;
        (*h).xmm0 = xmm0;
        (*h).t_us = t1.saturating_sub(t0);
        (*h).done = 1;
        libc::_exit(handler::EXIT_OK);
    }
    let deadline = now_us() + timeout_ms * 1000;
    let mut raw_status = 0i32;
    loop {
        let (w, err) = waitpid_retry(pid, &mut raw_status, libc::WNOHANG);
        if w == pid {
            break;
        }
        if w < 0 {
            raw_status = if err > 0 { -err } else { -1 };
            break;
        }
        if now_us() > deadline {
            libc::kill(pid, libc::SIGKILL);
            let _ = waitpid_retry(pid, &mut raw_status, 0);
            return RunRes { status: ST_TIMEOUT, wait_status: raw_status, ..Default::default() };
        }
        if parent_gone() {
            libc::kill(pid, libc::SIGKILL);
            let _ = waitpid_retry(pid, &mut raw_status, 0);
            libc::_exit(EXIT_PARENT_GONE);
        }
        libc::usleep(100);
    }
    let done = core::ptr::read_volatile(core::ptr::addr_of!((*h).done));
    let overflow = core::ptr::read_volatile(core::ptr::addr_of!((*h).overflow));
    let status = classify_run_exit(raw_status, done, overflow);
    if status == ST_OK {
        rebase(buf, bufs.maxpages);
    }
    RunRes {
        status,
        rax: core::ptr::read_volatile(core::ptr::addr_of!((*h).rax)),
        xmm0: core::ptr::read_volatile(core::ptr::addr_of!((*h).xmm0)),
        t_us: core::ptr::read_volatile(core::ptr::addr_of!((*h).t_us)),
        fault_addr: core::ptr::read_volatile(core::ptr::addr_of!((*h).fault_addr)),
        fault_pc: core::ptr::read_volatile(core::ptr::addr_of!((*h).fault_pc)),
        wait_status: raw_status,
        fault_sig: core::ptr::read_volatile(core::ptr::addr_of!((*h).fault_sig)) as u32,
        host_api: core::ptr::read_volatile(core::ptr::addr_of!((*h).host_api)),
    }
}

/// Classify a finished run child from its raw `waitpid` status and shared header flags.
///
/// A child terminated by its own `arm_watchdog` `SIGALRM` (for example if the orchestrator was
/// preempted past `run_watchdog_ms` on a virtualized runner) is a timeout (`ST_TIMEOUT`), not
/// a guest fault (`ST_FAULT`).
pub(crate) fn classify_run_exit(raw_status: i32, done: u64, overflow: u64) -> u8 {
    let signaled = raw_status >= 0 && libc::WIFSIGNALED(raw_status);
    if signaled && libc::WTERMSIG(raw_status) == libc::SIGALRM {
        return ST_TIMEOUT;
    }
    let exited = raw_status >= 0 && libc::WIFEXITED(raw_status);
    let code = if exited { libc::WEXITSTATUS(raw_status) } else { -1 };
    if code == handler::EXIT_SHARED {
        ST_SHARED
    } else if code == win32_rt::shadow::EXIT_HOST_UNSAFE {
        ST_HOST
    } else if code != handler::EXIT_OK || done != 1 {
        ST_FAULT
    } else if overflow != 0 {
        ST_OVERFLOW
    } else {
        ST_OK
    }
}

/// Orchestrator, after a completed run: keep only the bytes the run itself changed. A byte
/// of a dirtied page equal to the run's baseline (its content before the run's first write)
/// is replaced with the orchestrator's own. Without this, writes made in the run child
/// before tracking began (libSystem fork-child handlers: objc cache fills, xpc state) showed
/// up as diffs whenever the run also dirtied the same page, and runs that did not dirty it
/// read the orchestrator's untouched copy. Allocation-free.
unsafe fn rebase(buf: usize, maxpages: usize) {
    let pg = page();
    let n = ((*(buf as *const RunHdr)).npages as usize).min(maxpages);
    let list = (buf + HDR_BYTES) as *const u64;
    let data = buf + data_off(maxpages);
    let base = buf + base_off(maxpages);
    for i in 0..n {
        let live = *list.add(i) as *const u64;
        let d = (data + i * pg) as *mut u64;
        let b = (base + i * pg) as *const u64;
        for w in 0..pg / 8 {
            let x = *d.add(w) ^ *b.add(w);
            let mut keep = 0u64;
            if x != 0 {
                for k in 0..8 {
                    if (x >> (8 * k)) & 0xff != 0 {
                        keep |= 0xff << (8 * k);
                    }
                }
            }
            *d.add(w) = (*d.add(w) & keep) | (live.add(w).read_volatile() & !keep);
        }
    }
}
