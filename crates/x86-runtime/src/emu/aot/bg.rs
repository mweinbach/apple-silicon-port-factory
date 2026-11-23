//! AOT cache maintenance (crash-loop guard, LRU eviction), background builds (`PORT_AOT=2`),
//! and JIT block-profile recording / merging / TSV dumping.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex};

use super::builder::{
    build_image_with_threads, read_profile, WorkerCaches, PARALLEL_MIN_BLOCKS,
};
use super::cfg_discover::discover;
use super::format::{
    cache_file, encode, image_hash, profile_file, write_atomic, AotKey, BUILD_VBASE,
    FLAG_VERIFY_PENDING,
};
use super::runtime::{install_image, Verify, MAPPED};
use super::{
    aot_closure_depth, bg_threads, bg_wait, build_request, cache_dir, env_u32_any, game_run,
    profile_enabled, set_qos, verify_mode, QOS_CLASS_BACKGROUND, QOS_CLASS_UTILITY,
};

// ---------------------------------------------------------------------------------------------
// Cache maintenance: crash-loop guard, eviction
// ---------------------------------------------------------------------------------------------

/// Consecutive runs that did not exit cleanly after which one run goes without AOT.
const GUARD_STRIKES: u32 = 2;

/// Crash-loop guard verdict for a starting run ([`session_check`]).
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Session {
    /// AOT on (this run's lock created); `strikes` consecutive unclean runs before it,
    /// `stale` locks of dead processes removed.
    Go { strikes: u32, stale: usize },
    /// AOT off for this run (no lock; the strike counter starts over).
    Skip { stale: usize },
}

pub(super) fn session_lock(dir: &Path, pid: u32) -> PathBuf {
    dir.join(format!("session-{pid}.lock"))
}

/// Remove the `session-<pid>.lock` files in `dir` of processes that are gone (`alive`
/// false, or `pid` itself: a reused pid), i.e. runs that did not exit cleanly; count
/// consecutive starts that found one in `session.strikes`; decide. On [`Session::Go`],
/// `pid`'s lock is created ([`session_end`] removes it on a clean exit).
pub(super) fn session_check(dir: &Path, pid: u32, alive: impl Fn(u32) -> bool) -> Session {
    let _ = std::fs::create_dir_all(dir);
    let mut stale = 0;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let f = e.file_name();
            let Some(p) = f
                .to_str()
                .and_then(|s| s.strip_prefix("session-"))
                .and_then(|s| s.strip_suffix(".lock"))
                .and_then(|s| s.parse::<u32>().ok())
            else {
                continue;
            };
            if (p == pid || !alive(p)) && std::fs::remove_file(e.path()).is_ok() {
                stale += 1;
            }
        }
    }
    let sf = dir.join("session.strikes");
    let prev = std::fs::read_to_string(&sf)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);
    let strikes = if stale > 0 { prev + 1 } else { 0 };
    if strikes >= GUARD_STRIKES {
        let _ = std::fs::write(&sf, "0\n");
        return Session::Skip { stale };
    }
    let _ = std::fs::write(&sf, format!("{strikes}\n"));
    let _ = std::fs::write(session_lock(dir, pid), format!("{pid}\n"));
    Session::Go { strikes, stale }
}

/// Crash-loop guard of a game run (`PORT_AOT_GUARD` / `AW2_DLL_AOT_GUARD`, default 1):
/// false = no AOT caches this run. Only runs that die before they are established count:
/// this run's lock is released after `PORT_AOT_GUARD_SECS` / `AW2_DLL_AOT_GUARD_SECS`
/// (default 120) of uptime or at a clean exit, whichever comes first (harness SIGTERM /
/// force quit later is no strike).
pub(super) fn session_begin() -> bool {
    if cfg!(test)
        || !game_run()
        || build_request().is_some()
        || env_u32_any(&["PORT_AOT_GUARD", "AW2_DLL_AOT_GUARD"], 1) == 0
    {
        return true;
    }
    match session_check(&cache_dir(), std::process::id(), pid_alive) {
        Session::Go { strikes, stale } => {
            win32_rt::process::register_exit_hook(session_end);
            let secs = env_u32_any(&["PORT_AOT_GUARD_SECS", "AW2_DLL_AOT_GUARD_SECS"], 120) as u64;
            let _ = std::thread::Builder::new().name("port-aot-guard".into()).spawn(move || {
                set_qos(QOS_CLASS_BACKGROUND);
                std::thread::sleep(std::time::Duration::from_secs(secs));
                session_release(&format!("{secs} s uptime"));
            });
            if stale > 0 {
                log::warn!(
                    "emu aot: the previous run died before it was established ({strikes} in a row; after {GUARD_STRIKES} one run goes without AOT caches)"
                );
            }
            true
        }
        Session::Skip { stale } => {
            log::warn!(
                "emu aot: {GUARD_STRIKES} runs in a row died before they were established ({stale} stale session lock(s)): AOT caches off for this run (crash-loop guard; PORT_AOT_GUARD=0 disables it)"
            );
            false
        }
    }
}

/// Remove this run's session lock (once): the run is established, a later death is no
/// strike.
fn session_release(why: &str) {
    static DONE: AtomicBool = AtomicBool::new(false);
    if DONE.swap(true, Ordering::AcqRel) {
        return;
    }
    let _ = std::fs::remove_file(session_lock(&cache_dir(), std::process::id()));
    log::info!("emu aot: session established ({why}); crash-loop guard lock released");
}

pub(super) fn pid_alive(pid: u32) -> bool {
    extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    const EPERM: i32 = 1;
    if pid == 0 || pid > i32::MAX as u32 {
        return false;
    }
    unsafe {
        kill(pid as i32, 0) == 0
            || std::io::Error::last_os_error().raw_os_error() == Some(EPERM)
    }
}

/// Exit hook: a clean exit removes this run's session lock (crash paths run the exit hooks
/// too, without an exit code or with a failure code).
fn session_end() {
    if let Some(c) = win32_rt::process::exit_code().filter(|&c| clean_exit_code(c)) {
        session_release(&format!("clean exit, code {c}"));
    }
}

/// `ExitProcess` codes that count as a clean exit: not an NTSTATUS error / warning, not
/// `abort()`'s 3 or `terminate()`'s 255.
pub(super) fn clean_exit_code(c: u32) -> bool {
    c < 0x8000_0000 && c != 3 && c != 255
}

/// The sweep removes cache files not modified for this long ...
const EVICT_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(30 * 24 * 3600);
/// ... and then the least recently modified ones until the rest fits this (4 GiB to accommodate
/// multi-bank `.aot` caches for 100+ MiB game executables without immediate eviction).
const EVICT_CAP: u64 = 4 << 30;

/// Eviction candidates in cache dir `dir`: `*.aot`, `profile/*.tsv` and leftover
/// `*.tmp<pid>` files, with size and mtime (loads bump a cache's mtime: [`touch`]).
fn cache_files(dir: &Path) -> Vec<(PathBuf, u64, std::time::SystemTime)> {
    let mut v = Vec::new();
    for d in [dir.to_path_buf(), dir.join("profile"), dir.join("callbacks")] {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("");
            if !(ext == "aot" || ext == "tsv" || ext.starts_with("tmp")) {
                continue;
            }
            let Ok(md) = e.metadata() else { continue };
            if md.is_file() {
                v.push((p, md.len(), md.modified().unwrap_or(std::time::UNIX_EPOCH)));
            }
        }
    }
    v
}

/// Remove the cache files of `dir` not modified for `max_age`, then the least recently
/// modified ones until the rest fits in `cap` bytes. Returns `[removed files, removed
/// bytes, kept files, kept bytes]`.
pub(super) fn evict(
    dir: &Path,
    now: std::time::SystemTime,
    max_age: std::time::Duration,
    cap: u64,
) -> [u64; 4] {
    let mut files = cache_files(dir);
    files.sort_by_key(|f| f.2);
    let mut kept: u64 = files.iter().map(|f| f.1).sum();
    let mut out = [0u64; 4];
    for (p, len, mt) in &files {
        let old = now.duration_since(*mt).is_ok_and(|age| age > max_age);
        if (old || kept > cap) && std::fs::remove_file(p).is_ok() {
            out[0] += 1;
            out[1] += len;
            kept -= len;
        }
    }
    out[2] = files.len() as u64 - out[0];
    out[3] = kept;
    out
}

/// After `keep` (`<dir>/<name>-<key>.aot`) was written: remove `name`'s caches for other
/// keys (other loader builds / knobs / image versions). Returns the number removed.
pub(super) fn delete_other_keys(dir: &Path, name: &str, keep: &Path) -> usize {
    let pre = format!("{name}-");
    let Ok(rd) = std::fs::read_dir(dir) else { return 0 };
    let mut n = 0;
    for e in rd.flatten() {
        let f = e.file_name();
        let Some(id) = f
            .to_str()
            .and_then(|s| s.strip_prefix(pre.as_str()))
            .and_then(|s| s.strip_suffix(".aot"))
        else {
            continue;
        };
        if id.len() == 16
            && id.bytes().all(|b| b.is_ascii_hexdigit())
            && e.path() != keep
            && std::fs::remove_file(e.path()).is_ok()
        {
            n += 1;
        }
    }
    n
}

/// Bump `path`'s mtime (a loaded cache is in use: the sweep evicts least recently used).
pub(super) fn touch(path: &Path) {
    if let Ok(f) = std::fs::File::options().write(true).open(path) {
        let _ = f.set_modified(std::time::SystemTime::now());
    }
}

/// Once per process: after [`super::bg_delay`], at background QoS, [`evict`] the cache dir.
pub(super) fn sweep_start() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = std::thread::Builder::new().name("port-aot-sweep".into()).spawn(|| {
            set_qos(QOS_CLASS_BACKGROUND);
            std::thread::sleep(bg_wait());
            let r = evict(&cache_dir(), std::time::SystemTime::now(), EVICT_MAX_AGE, EVICT_CAP);
            log::info!(
                "emu aot: cache sweep: removed {} files ({} KiB; unused for {} days / over the {} MiB cap), kept {} files ({} KiB)",
                r[0],
                r[1] >> 10,
                EVICT_MAX_AGE.as_secs() / 86400,
                EVICT_CAP >> 20,
                r[2],
                r[3] >> 10
            );
        });
    });
}

// ---------------------------------------------------------------------------------------------
// Background builds (`AW2_DLL_AOT=2`)
// ---------------------------------------------------------------------------------------------

/// One background build: module, its activation-time key, and whether the result is
/// installed into this process (no valid cache was loaded) or only written for the next run
/// (the loaded cache's profile grew).
pub(super) struct BgJob {
    name: String,
    base: usize,
    size: usize,
    key: AotKey,
    install: bool,
}

/// Pending jobs and whether the builder thread is running (both under this lock).
static BG: Mutex<(Vec<BgJob>, bool)> = Mutex::new((Vec::new(), false));
/// Wakes [`bg_main`] immediately when new jobs (e.g. late `LoadLibrary` install jobs) arrive.
static BG_CV: Condvar = Condvar::new();

/// A loaded cache is rebuilt for the next run when more than 1/`BG_STALE_DIV` of the
/// module's profiled blocks, or more than `BG_STALE_MAX` of them, are not in it.
const BG_STALE_DIV: usize = 10;
const BG_STALE_MAX: usize = 2000;

/// `AW2_DLL_AOT=2` at activation: the background job for module `name` given the load
/// result (`loaded`: block IPs of the installed cache), or None.
pub(super) fn bg_job(
    name: &str,
    base: usize,
    size: usize,
    key: AotKey,
    loaded: Option<&[u64]>,
) -> Option<BgJob> {
    if crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!(
            "emu aot: {name}: overlaps run-time generated/modified code; skipping background AOT build"
        );
        return None;
    }
    let prof = read_profile(name, &key.image_hash);
    if prof.is_empty() {
        log::info!(
            "emu aot: {name}: no profile yet (recording; AW2_DLL_AOT=2 builds at a later start)"
        );
        return None;
    }
    let install = match loaded {
        None => true,
        Some(ips) => {
            let have: HashSet<u64> = ips.iter().copied().collect();
            let new = prof
                .iter()
                .filter(|&&rva| !have.contains(&(base as u64 + rva as u64)))
                .count();
            if new * BG_STALE_DIV <= prof.len() && new <= BG_STALE_MAX {
                return None;
            }
            log::info!(
                "emu aot: {name}: {new} of {} profiled blocks not in the cache; rebuilding it in the background for the next run",
                prof.len()
            );
            false
        }
    };
    Some(BgJob { name: name.to_string(), base, size, key, install })
}

/// Queue `jobs` for the builder thread (started on demand; exits when the queue is empty).
pub(super) fn bg_enqueue(jobs: Vec<BgJob>) {
    if jobs.is_empty() {
        return;
    }
    let mut g = BG.lock().unwrap_or_else(|e| e.into_inner());
    g.0.extend(jobs);
    if g.1 {
        BG_CV.notify_one();
        return;
    }
    g.1 = true;
    let spawned = std::thread::Builder::new()
        .name("port-aot-build".into())
        .stack_size(16 << 20)
        .spawn(bg_main);
    if let Err(e) = spawned {
        log::error!("emu aot: cannot start the background build thread: {e}");
        g.0.clear();
        g.1 = false;
    }
}

/// Builder thread: jobs whose result is installed now first (utility QoS); next-run
/// rebuilds wait until [`super::bg_delay`] after the first activation (woken immediately via
/// [`BG_CV`] if a late-loaded install job arrives) and run at background QoS.
fn bg_main() {
    let mut pool = WorkerCaches::new();
    let mut qos = 0;
    loop {
        let job = {
            let mut g = BG.lock().unwrap_or_else(|e| e.into_inner());
            loop {
                if g.0.is_empty() {
                    g.1 = false;
                    return;
                }
                if let Some(i) = g.0.iter().position(|j| j.install) {
                    break g.0.remove(i);
                }
                let wait = bg_wait();
                if wait.is_zero() {
                    break g.0.remove(0);
                }
                let (next_g, _) = BG_CV.wait_timeout(g, wait).unwrap_or_else(|e| e.into_inner());
                g = next_g;
            }
        };
        let want = if job.install { QOS_CLASS_UTILITY } else { QOS_CLASS_BACKGROUND };
        if want != qos {
            set_qos(want);
            qos = want;
        }
        unsafe { bg_build(&mut pool, job) };
    }
}

/// Build `job`'s module from its profile on this (background) thread using a conservative
/// thread pool (`bg_threads()`); discard the result if the image's code changed since
/// activation; write the cache (flagged for verification at its first load,
/// [`FLAG_VERIFY_PENDING`]); install it if requested.
unsafe fn bg_build(pool: &mut WorkerCaches, job: BgJob) {
    let BgJob { name, base, size, key, install } = job;
    if crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!(
            "emu aot: {name}: overlaps run-time generated/modified code; skipping background AOT build"
        );
        return;
    }
    let t0 = std::time::Instant::now();
    let mut prof = read_profile(&name, &key.image_hash);
    let replayed = super::callbacks::load_replayed_callback_roots(&name, base, size);
    if !replayed.is_empty() {
        prof.extend(replayed.into_iter().map(|c| c.rva));
        prof.sort_unstable();
        prof.dedup();
    }
    let depth = aot_closure_depth();
    let bcp = pool.get_one();
    let (ips, _) = discover(bcp, base, size, &prof, depth);
    if crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!(
            "emu aot: {name}: became run-time modified during discovery; aborting background AOT build"
        );
        return;
    }
    let build_threads = if ips.len() >= PARALLEL_MIN_BLOCKS { bg_threads() } else { 1 };
    let Some(img) = build_image_with_threads(pool, &ips, BUILD_VBASE, build_threads) else {
        log::error!("emu aot: {name}: shared stubs don't fit; not built");
        return;
    };
    if crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: became run-time modified during build; result discarded");
        return;
    }
    if image_hash(base, size) != key.image_hash {
        log::warn!("emu aot: {name}: code changed during the background build; result discarded");
        return;
    }
    let build_s = t0.elapsed().as_secs_f64();
    let path = cache_file(&name, &key);
    let mut evicted = 0;
    match write_atomic(&path, &encode(&name, &key, &img, FLAG_VERIFY_PENDING)) {
        Ok(()) => evicted = delete_other_keys(&cache_dir(), &name, &path),
        Err(e) => log::error!("emu aot: {name}: writing {}: {e}", path.display()),
    }
    log::info!(
        "emu aot: {name}: background build: {} blocks ({} profiled, {} candidates), {} KiB code in {build_s:.2}s -> {}{}{}",
        img.blocks.len(),
        prof.len(),
        ips.len(),
        img.code_bytes() >> 10,
        path.display(),
        if install { "; installing" } else { " (next run)" },
        if evicted != 0 { format!("; {evicted} cache(s) of other keys removed") } else { String::new() }
    );
    if install {
        install_image(
            &name,
            base,
            size,
            img,
            if verify_mode() != 0 { Verify::All } else { Verify::None },
            false,
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Profile
// ---------------------------------------------------------------------------------------------

static PROFILE: Mutex<Option<HashMap<u64, u32>>> = Mutex::new(None);

/// `JitCache` translated the image block at `ip` (`gen == 0`).
#[inline]
pub(in crate::emu) fn profile_note(ip: u64) {
    if profile_enabled() {
        profile_add(ip);
    }
}

#[cold]
#[inline(never)]
fn profile_add(ip: u64) {
    static START: std::sync::Once = std::sync::Once::new();
    START.call_once(|| {
        win32_rt::process::register_exit_hook(profile_dump);
        let _ = std::thread::Builder::new().name("port-jit-profile".into()).spawn(|| loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
            profile_dump();
        });
    });
    if let Ok(mut p) = PROFILE.lock() {
        *p.get_or_insert_with(HashMap::new).entry(ip).or_insert(0) += 1;
    }
}

/// Profile counts are halved (rounding up: no block drops out) once their total exceeds
/// this, so they stay bounded across runs and recent runs keep their weight.
const PROFILE_HALVE_AT: u64 = 1 << 32;

/// Add `counts` to the merged profile `all`; halve it past [`PROFILE_HALVE_AT`] (true).
pub(super) fn profile_merge(
    all: &mut HashMap<u32, u64>,
    counts: impl IntoIterator<Item = (u32, u64)>,
) -> bool {
    for (rva, n) in counts {
        let e = all.entry(rva).or_insert(0);
        *e = e.saturating_add(n);
    }
    let total = all.values().fold(0u64, |a, &n| a.saturating_add(n));
    if total <= PROFILE_HALVE_AT {
        return false;
    }
    for n in all.values_mut() {
        *n = n.div_ceil(2);
    }
    true
}

pub(super) fn read_tsv(path: &Path) -> HashMap<u32, u64> {
    let mut m = HashMap::new();
    if let Ok(s) = std::fs::read_to_string(path) {
        for line in s.lines() {
            let mut it = line.split('\t');
            let (Some(a), Some(b)) = (it.next(), it.next()) else { continue };
            let (Ok(rva), Ok(n)) = (
                u32::from_str_radix(a.trim().trim_start_matches("0x"), 16),
                b.trim().parse::<u64>(),
            ) else {
                continue;
            };
            *m.entry(rva).or_insert(0) += n;
        }
    }
    m
}

/// Merge the profile recorded since the last dump into
/// `<cache>/profile/<module>-<imagehash>.tsv` (one file per mapped image). Exit hook and
/// periodic; never blocks on a lock held elsewhere (crash paths).
pub fn profile_dump() {
    let taken = match PROFILE.try_lock() {
        Ok(mut g) => g.take(),
        Err(_) => None,
    };
    let Some(p) = taken else { return };
    let mods: Vec<(String, usize, usize, Option<[u64; 2]>)> = match MAPPED.try_lock() {
        Ok(v) => v.iter().map(|m| (m.name.clone(), m.base, m.size, m.phash)).collect(),
        Err(_) => return,
    };
    let mut per: HashMap<usize, HashMap<u32, u64>> = HashMap::new();
    for (ip, n) in p {
        let a = ip as usize;
        if let Some(i) = mods.iter().position(|m| a >= m.1 && a - m.1 < m.2) {
            *per.entry(i).or_default().entry((a - mods[i].1) as u32).or_insert(0) += n as u64;
        }
    }
    for (i, counts) in per {
        let (name, base, size, phash) = &mods[i];
        let path = profile_file(name, &phash.unwrap_or_else(|| unsafe { image_hash(*base, *size) }));
        let mut all = read_tsv(&path);
        profile_merge(&mut all, counts);
        let mut v: Vec<(u32, u64)> = all.into_iter().collect();
        v.sort_unstable_by_key(|&(rva, n)| (core::cmp::Reverse(n), rva));
        let mut s = String::with_capacity(v.len() * 16);
        for (rva, n) in &v {
            s += &format!("{rva:#x}\t{n}\n");
        }
        match write_atomic(&path, s.as_bytes()) {
            Ok(()) => log::info!("emu aot: profile {name}: {} blocks -> {}", v.len(), path.display()),
            Err(e) => log::warn!("emu aot: profile {name}: {}: {e}", path.display()),
        }
    }
}
