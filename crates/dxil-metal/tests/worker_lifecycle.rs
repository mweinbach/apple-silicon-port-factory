//! End-to-end lifecycle & crash-resilience tests for the real `dxil-metald` worker binary.
//!
//! Covers every termination mode:
//! - clean stdin EOF (exit 0, empty stderr)
//! - `SIGTERM` and `SIGINT` (exit 0, empty stderr)
//! - `HelperClient` drop (all pooled workers gone within < 1s)
//! - `atexit` teardown when a client is never dropped (simulates `ShaderPipeline::global()` / `exit(0)`)
//! - abrupt parent `SIGKILL` with stdin still held open (proves macOS `kqueue` `EVFILT_PROC` + `NOTE_EXIT`)
//! - abrupt parent `SIGKILL` while worker is mid-request (`OP_SLEEP`, not reading stdin)

use std::ffi::{c_int, OsStr};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use dxil_metal::helper::HelperClient;

const HELPER: &str = env!("CARGO_BIN_EXE_dxil-metald");

#[cfg(unix)]
mod unix {
    use super::*;

    pub const SIGINT: c_int = 2;
    pub const SIGKILL: c_int = 9;
    pub const SIGTERM: c_int = 15;
    pub const ESRCH: i32 = 3;

    extern "C" {
        fn kill(pid: c_int, sig: c_int) -> c_int;
    }

    pub fn send_signal(pid: u32, sig: c_int) {
        unsafe {
            let _ = kill(pid as c_int, sig);
        }
    }

    /// True if the process exists (`kill(pid, 0) == 0` or errno != `ESRCH`).
    pub fn pid_exists(pid: u32) -> bool {
        let rc = unsafe { kill(pid as c_int, 0) };
        if rc == 0 {
            return true;
        }
        std::io::Error::last_os_error().raw_os_error() != Some(ESRCH)
    }

    /// Polls until `kill(pid, 0) == -1 && errno == ESRCH`, or returns `false` on timeout.
    pub fn wait_until_pid_gone(pid: u32, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        loop {
            let rc = unsafe { kill(pid as c_int, 0) };
            if rc != 0 && std::io::Error::last_os_error().raw_os_error() == Some(ESRCH) {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Ensures a stray PID is killed if an assertion fails mid-test.
    pub struct PidGuard(pub Option<u32>);
    impl Drop for PidGuard {
        fn drop(&mut self) {
            if let Some(pid) = self.0.take() {
                if pid_exists(pid) {
                    send_signal(pid, SIGKILL);
                }
            }
        }
    }

    /// Ensures a spawned `Child` is killed and reaped if an assertion fails mid-test.
    pub struct ChildGuard(pub Option<Child>);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            if let Some(mut child) = self.0.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn daemon_exits_cleanly_on_stdin_eof() {
    let mut child = Command::new(HELPER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn dxil-metald");
    let pid = child.id();

    // Give the daemon a brief moment to start its watchdog and block on stdin.
    std::thread::sleep(Duration::from_millis(40));
    assert!(unix::pid_exists(pid), "dxil-metald {pid} exited prematurely");

    // Closing stdin sends EOF; daemon must exit cleanly with code 0 and empty stderr.
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("wait for dxil-metald");
    assert_eq!(output.status.code(), Some(0), "unexpected status: {:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "unexpected stderr on clean EOF: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(unix::wait_until_pid_gone(pid, Duration::from_secs(1)));
}

#[cfg(unix)]
#[test]
fn daemon_exits_cleanly_on_sigterm_and_sigint() {
    for (name, sig) in [("SIGTERM", unix::SIGTERM), ("SIGINT", unix::SIGINT)] {
        let mut guard = unix::ChildGuard(Some(
            Command::new(HELPER)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn dxil-metald"),
        ));
        let pid = guard.0.as_ref().unwrap().id();
        // Hold stdin open so the daemon is blocked in `read_frame`, not exiting via EOF.
        let _stdin_keepalive = guard.0.as_mut().unwrap().stdin.take();

        std::thread::sleep(Duration::from_millis(50));
        assert!(unix::pid_exists(pid), "dxil-metald {pid} exited before {name}");

        unix::send_signal(pid, sig);
        let child = guard.0.take().unwrap();
        let output = child.wait_with_output().expect("wait after signal");
        assert_eq!(
            output.status.code(),
            Some(0),
            "dxil-metald did not exit cleanly with code 0 on {name}: {:?}",
            output.status
        );
        assert!(
            output.stderr.is_empty(),
            "unexpected stderr on {name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(unix::wait_until_pid_gone(pid, Duration::from_secs(1)));
    }
}

#[cfg(unix)]
#[test]
fn client_drop_terminates_workers_cleanly() {
    if !dxil_metal::converter_available() {
        eprintln!("skip: libmetalirconverter.dylib not installed");
        return;
    }

    let client = Arc::new(HelperClient::connect(Path::new(HELPER), 2).expect("connect HelperClient"));
    // Run two concurrent requests so both pooled workers are spawned.
    let c1 = client.clone();
    let c2 = client.clone();
    let t1 = std::thread::spawn(move || c1.sleep_one_worker(80).unwrap());
    let t2 = std::thread::spawn(move || c2.sleep_one_worker(80).unwrap());
    t1.join().unwrap();
    t2.join().unwrap();

    let pids = client.worker_pids();
    assert!(
        !pids.is_empty() && pids.len() <= 2,
        "expected 1..=2 live workers, got {pids:?}"
    );
    let mut guards: Vec<unix::PidGuard> = pids.iter().map(|&p| unix::PidGuard(Some(p))).collect();
    for &pid in &pids {
        assert!(unix::pid_exists(pid), "worker {pid} should be alive while client lives");
    }

    let t0 = Instant::now();
    drop(client);

    for &pid in &pids {
        assert!(
            unix::wait_until_pid_gone(pid, Duration::from_secs(1)),
            "worker {pid} survived HelperClient::drop"
        );
    }
    assert!(
        t0.elapsed() < Duration::from_secs(1),
        "HelperClient::drop took too long: {:?}",
        t0.elapsed()
    );
    for g in &mut guards {
        g.0 = None;
    }
}

#[cfg(unix)]
#[test]
fn atexit_cleans_up_leaked_client_without_drop() {
    if std::env::var_os("DXIL_METALD_TEST_ROLE").as_deref() == Some(OsStr::new("atexit_child")) {
        let client = HelperClient::connect(Path::new(HELPER), 1).expect("connect HelperClient in child");
        let pids = client.worker_pids();
        assert_eq!(pids.len(), 1);
        println!("WORKER_PID={}", pids[0]);
        use std::io::Write;
        let _ = std::io::stdout().flush();
        // Intentionally skip Drop (just like `static OnceLock` in `ShaderPipeline::global()`).
        std::mem::forget(client);
        std::process::exit(0);
    }

    if !dxil_metal::converter_available() {
        eprintln!("skip: libmetalirconverter.dylib not installed");
        return;
    }

    let self_exe = std::env::current_exe().expect("current_exe");
    let output = Command::new(self_exe)
        .args(["--exact", "atexit_cleans_up_leaked_client_without_drop", "--nocapture"])
        .env("DXIL_METALD_TEST_ROLE", "atexit_child")
        .output()
        .expect("spawn atexit child");
    assert!(
        output.status.success(),
        "atexit child failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let worker_pid: u32 = stdout
        .lines()
        .find_map(|l| l.strip_prefix("WORKER_PID="))
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or_else(|| panic!("missing WORKER_PID in child stdout:\n{stdout}"));
    let mut guard = unix::PidGuard(Some(worker_pid));

    assert!(
        unix::wait_until_pid_gone(worker_pid, Duration::from_secs(1)),
        "worker {worker_pid} lingered after process exit with leaked HelperClient"
    );
    guard.0 = None;
}

#[cfg(unix)]
#[test]
fn parent_sigkill_triggers_kqueue_watchdog_even_with_open_stdin() {
    // Spawn a `/bin/sh` parent that launches `dxil-metald` inheriting our open stdin pipe (`<&0`),
    // prints `dxil-metald`'s PID, and waits. Keeping our write end of the pipe open ensures
    // `dxil-metald` does NOT receive stdin EOF when `/bin/sh` is `SIGKILL`ed — ONLY the parent-death
    // watchdog (`kqueue` `EVFILT_PROC` + `NOTE_EXIT` on macOS) can terminate it.
    let mut parent_guard = unix::ChildGuard(Some(
        Command::new("/bin/sh")
            .args(["-c", r#""$1" <&0 >/dev/null 2>&1 & echo $!; wait"#, "_", HELPER])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn parent shell"),
    ));
    let parent_stdin_keepalive = parent_guard.0.as_mut().unwrap().stdin.take().unwrap();
    let parent_stdout = parent_guard.0.as_mut().unwrap().stdout.take().unwrap();
    let parent_pid = parent_guard.0.as_ref().unwrap().id();

    let mut line = String::new();
    BufReader::new(parent_stdout)
        .read_line(&mut line)
        .expect("read worker PID from parent shell");
    let worker_pid: u32 = line
        .trim()
        .parse()
        .unwrap_or_else(|e| panic!("invalid worker PID {line:?}: {e}"));
    let mut worker_guard = unix::PidGuard(Some(worker_pid));

    // Confirm both parent and worker stay alive before the crash.
    std::thread::sleep(Duration::from_millis(80));
    assert!(unix::pid_exists(parent_pid), "parent shell {parent_pid} exited early");
    assert!(unix::pid_exists(worker_pid), "dxil-metald {worker_pid} exited early");

    // Abruptly SIGKILL the parent process while STILL holding `parent_stdin_keepalive` open!
    let t0 = Instant::now();
    unix::send_signal(parent_pid, unix::SIGKILL);
    let _ = parent_guard.0.take().unwrap().wait();

    assert!(
        unix::wait_until_pid_gone(worker_pid, Duration::from_secs(2)),
        "dxil-metald {worker_pid} survived parent SIGKILL (elapsed {:?})",
        t0.elapsed()
    );
    worker_guard.0 = None;
    drop(parent_stdin_keepalive);
}

#[cfg(unix)]
#[test]
fn parent_sigkill_while_worker_busy_sleeping_triggers_watchdog() {
    if std::env::var_os("DXIL_METALD_TEST_ROLE").as_deref() == Some(OsStr::new("busy_sigkill_child")) {
        let client = Arc::new(HelperClient::connect(Path::new(HELPER), 1).expect("connect HelperClient"));
        let pids = client.worker_pids();
        assert_eq!(pids.len(), 1);
        let worker_pid = pids[0];

        // Put the worker into a 30s sleep inside OP_SLEEP so it is NOT blocked reading stdin.
        let c = client.clone();
        std::thread::spawn(move || {
            let _ = c.sleep_one_worker(30_000);
        });
        std::thread::sleep(Duration::from_millis(80));

        println!("WORKER_PID={worker_pid}");
        use std::io::Write;
        let _ = std::io::stdout().flush();

        // Wait until parent SIGKILLs us.
        std::thread::sleep(Duration::from_secs(60));
        return;
    }

    if !dxil_metal::converter_available() {
        eprintln!("skip: libmetalirconverter.dylib not installed");
        return;
    }

    let self_exe = std::env::current_exe().expect("current_exe");
    let mut child_guard = unix::ChildGuard(Some(
        Command::new(self_exe)
            .args([
                "--exact",
                "parent_sigkill_while_worker_busy_sleeping_triggers_watchdog",
                "--nocapture",
            ])
            .env("DXIL_METALD_TEST_ROLE", "busy_sigkill_child")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn busy_sigkill_child"),
    ));
    let child_pid = child_guard.0.as_ref().unwrap().id();
    let child_stdout = child_guard.0.as_mut().unwrap().stdout.take().unwrap();

    let mut worker_pid = None;
    for line in BufReader::new(child_stdout).lines() {
        let line = line.expect("read line from child");
        if let Some(rest) = line.strip_prefix("WORKER_PID=") {
            worker_pid = rest.trim().parse::<u32>().ok();
            break;
        }
    }
    let worker_pid = worker_pid.expect("missing WORKER_PID from busy_sigkill_child");
    let mut worker_guard = unix::PidGuard(Some(worker_pid));

    assert!(unix::pid_exists(worker_pid), "busy worker {worker_pid} exited early");

    // Abruptly SIGKILL the parent client process while worker is mid-OP_SLEEP(30_000).
    let t0 = Instant::now();
    unix::send_signal(child_pid, unix::SIGKILL);
    let _ = child_guard.0.take().unwrap().wait();

    assert!(
        unix::wait_until_pid_gone(worker_pid, Duration::from_secs(2)),
        "busy dxil-metald {worker_pid} survived parent SIGKILL (elapsed {:?})",
        t0.elapsed()
    );
    worker_guard.0 = None;
}
