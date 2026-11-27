//! `AW2_SHADOW`: gameplay shadow oracle. Design: `target/review/exec/shadow_design.md`.
//!
//! Selected lifted replacements are compared against their guest originals on live game
//! state. When a selected function is called (bridge entry, see [`crate::emu::DF_SHADOW`])
//! and a probe may start, the game thread forks once and continues. The forked
//! orchestrator write-protects guest-visible memory and forks three runs from that
//! snapshot: guest original, native replacement, guest original. Each run records the
//! pages it dirties. When the first two differ, guest and native run once more with their
//! scratch stacks poisoned (G3, N3), to separate bytes read from uninitialised stack. The
//! orchestrator then compares rax/xmm0 and the dirtied bytes and reports `equal` /
//! `mismatch` / `abi_narrow` / `nondet` / `alloc_layout` / `ret_dead` / `stack_layout` /
//! `uninit` / `skipped(..)` over a pipe to the collector thread, which writes `shadow.log`,
//! `summary.txt` and the dumps.
//!
//! Spec (`;`-separated): `va=0x..,0x..` `skip=..` `shard=templates_a,r_more`
//! `class=draft|verified|tpl|dll|all` `throws=1` `n=8` `n_va=` `every=1` `rate=100` (ms
//! between probes) `timeout=2000` (ms per run) `stop_equal=4` `stop_dto=2` `jit=0`
//! `maxpages=16384` `dir=target/shadow` `db=db/progress.tsv`
//! `rets=db/ret_liveness.tsv,db/ret_liveness_dll.tsv`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock, RwLock};

mod compare;
mod core_probe;
mod handler;
mod hostalloc;
mod memmap;
mod run;
mod types;
pub use compare::DiffRange;
pub use core_probe::diag;
pub use types::{ChildExit, Outcome, Reason, Report, RetUse};

/// Class bits (`class=`).
pub const C_DRAFT: u32 = 1;
pub const C_VERIFIED: u32 = 2;
pub const C_TPL: u32 = 4;
pub const C_DLL: u32 = 8;

/// Who asked for the probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An exe replacement entered through the `call_va8` bridge.
    Exe,
    /// An exe-zero lifted DLL entry (cov stub).
    Dll,
}

/// Argument image for [`check`]: Win64 GPR args 1..8, XMM0..3, stack args 9..16.
#[derive(Clone, Copy, Debug, Default)]
pub struct ArgRegs {
    pub gpr: [u64; 8],
    pub xmm: [u128; 4],
    pub stack: [u64; 8],
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub vas: Vec<u64>,
    pub skip: Vec<u64>,
    pub shards: Vec<String>,
    pub classes: u32,
    pub throws: bool,
    pub n: u32,
    /// Sample budget for functions named in `va=` (0: same as `n`).
    pub n_va: u32,
    pub every: u32,
    pub rate_ms: u64,
    pub timeout_ms: u64,
    pub stop_equal: u32,
    pub stop_dto: u32,
    pub jit: bool,
    pub maxpages: usize,
    pub dir: String,
    pub db: String,
    /// Return-register liveness tables, comma-separated (`tools/fndb/retlive.py`,
    /// `retlive_dll.py`); a function without a row gets a full compare.
    pub rets: String,
}

impl Default for Spec {
    fn default() -> Self {
        Spec {
            vas: Vec::new(),
            skip: Vec::new(),
            shards: Vec::new(),
            classes: 0,
            throws: false,
            n: 8,
            n_va: 0,
            every: 1,
            rate_ms: 100,
            timeout_ms: 2000,
            stop_equal: 4,
            stop_dto: 2,
            jit: false,
            maxpages: 16384,
            dir: "target/shadow".into(),
            db: "db/progress.tsv".into(),
            rets: "db/ret_liveness.tsv,db/ret_liveness_dll.tsv".into(),
        }
    }
}

fn parse_u64(s: &str) -> Option<u64> {
    let s = s.trim();
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(h) => u64::from_str_radix(h, 16).ok(),
        None => s.parse().ok(),
    }
}

impl Spec {
    /// Parse an `AW2_SHADOW` value. Unknown keys are reported in the error.
    pub fn parse(s: &str) -> Result<Spec, String> {
        let mut sp = Spec::default();
        for part in s.split(';').map(str::trim).filter(|p| !p.is_empty()) {
            let (k, v) = part.split_once('=').ok_or_else(|| format!("shadow: `{part}` is not key=value"))?;
            let list = || v.split(',').map(str::trim).filter(|x| !x.is_empty());
            let num = || parse_u64(v).ok_or_else(|| format!("shadow: bad number in `{part}`"));
            match k.trim() {
                "va" => {
                    for x in list() {
                        sp.vas.push(parse_u64(x).ok_or_else(|| format!("shadow: bad va `{x}`"))?);
                    }
                }
                "skip" => {
                    for x in list() {
                        sp.skip.push(parse_u64(x).ok_or_else(|| format!("shadow: bad va `{x}`"))?);
                    }
                }
                "shard" => sp.shards.extend(list().map(String::from)),
                "class" => {
                    for c in v.split(['|', ',']).map(str::trim).filter(|x| !x.is_empty()) {
                        sp.classes |= match c {
                            "draft" => C_DRAFT,
                            "verified" => C_VERIFIED,
                            "tpl" => C_TPL,
                            "dll" => C_DLL,
                            "all" => C_DRAFT | C_VERIFIED | C_TPL | C_DLL,
                            _ => return Err(format!("shadow: unknown class `{c}`")),
                        };
                    }
                }
                "throws" => sp.throws = num()? != 0,
                "n" => sp.n = num()?.max(1) as u32,
                "n_va" => sp.n_va = num()? as u32,
                "every" => sp.every = num()?.max(1) as u32,
                "rate" => sp.rate_ms = num()?,
                "timeout" => sp.timeout_ms = num()?.max(1),
                "stop_equal" => sp.stop_equal = num()?.max(1) as u32,
                "stop_dto" => sp.stop_dto = num()?.max(1) as u32,
                "jit" => sp.jit = num()? != 0,
                "maxpages" => sp.maxpages = num()?.clamp(16, 1 << 20) as usize,
                "dir" => sp.dir = v.trim().to_string(),
                "db" => sp.db = v.trim().to_string(),
                "rets" => sp.rets = v.trim().to_string(),
                other => return Err(format!("shadow: unknown key `{other}`")),
            }
        }
        Ok(sp)
    }
}

/// A selectable function.
#[derive(Clone, Debug)]
pub struct FnInfo {
    pub va: u64,
    pub name: String,
    pub group: &'static str,
    pub classes: u32,
    pub throws: bool,
}

/// Per-function sampling state and outcome counters.
pub struct Slot {
    pub info: FnInfo,
    /// Calls seen while a probe could start (not all calls: disarmed calls aren't counted).
    calls: AtomicU64,
    /// Next call index that may be sampled (`every` with doubling backoff after `equal`).
    next_at: AtomicU64,
    samples: AtomicU32,
    /// Sample budget (`n`, or `n_va` for functions named in `va=`).
    limit: u32,
    retired: AtomicBool,
    pub counts: [AtomicU32; core_probe::N_OUTCOMES],
    dto: AtomicU32,
    every: AtomicU64,
}

impl Slot {
    fn new(info: FnInfo, every: u32, limit: u32) -> Slot {
        Slot {
            info,
            calls: AtomicU64::new(0),
            next_at: AtomicU64::new(0),
            samples: AtomicU32::new(0),
            limit,
            retired: AtomicBool::new(false),
            counts: Default::default(),
            dto: AtomicU32::new(0),
            every: AtomicU64::new(every as u64),
        }
    }
}

pub(crate) struct State {
    pub spec: Spec,
    /// Append-only (late [`register_dll`] entries): slots are leaked, indices stay valid.
    slots: RwLock<Vec<&'static Slot>>,
    index: RwLock<HashMap<u64, usize>>,
    /// Result pipe: (read end, write end).
    pub pipe: (i32, i32),
    pub pending: Mutex<Option<std::sync::mpsc::Sender<core_probe::Pending>>>,
    pub seq: AtomicU64,
    /// The collector stopped re-arming (every slot retired); a late slot re-arms.
    pub idle: AtomicBool,
    /// Every `rets=` row (init-time and late-registered entries alike).
    rets: HashMap<u64, RetUse>,
}

impl State {
    fn new(spec: Spec, chosen: Vec<FnInfo>, pipe: (i32, i32), pending: Option<std::sync::mpsc::Sender<core_probe::Pending>>) -> State {
        let st = State {
            spec,
            slots: RwLock::new(Vec::with_capacity(chosen.len())),
            index: RwLock::new(HashMap::with_capacity(chosen.len())),
            pipe,
            pending: Mutex::new(pending),
            seq: AtomicU64::new(0),
            idle: AtomicBool::new(false),
            rets: HashMap::new(),
        };
        for f in chosen {
            st.push(f);
        }
        st
    }

    /// Add a slot for `f` unless its va already has one. Returns whether it was added.
    fn push(&self, f: FnInfo) -> bool {
        let (Ok(mut idx), Ok(mut slots)) = (self.index.write(), self.slots.write()) else { return false };
        if idx.contains_key(&f.va) {
            return false;
        }
        idx.insert(f.va, slots.len());
        let limit = if self.spec.n_va > 0 && self.spec.vas.contains(&f.va) { self.spec.n_va } else { self.spec.n };
        slots.push(Box::leak(Box::new(Slot::new(f, self.spec.every, limit))));
        true
    }

    /// Late registration: add `f` if the spec selects it (same rules as at init).
    fn add_late(&self, f: FnInfo) -> bool {
        match choose(&self.spec, vec![f]).pop() {
            Some(f) => self.push(f),
            None => false,
        }
    }

    pub(crate) fn lookup(&self, va: u64) -> Option<usize> {
        self.index.read().ok()?.get(&va).copied()
    }

    pub(crate) fn slot(&self, i: usize) -> &'static Slot {
        self.slots.read().map(|s| s[i]).expect("shadow slot table")
    }

    pub(crate) fn slot_list(&self) -> Vec<&'static Slot> {
        self.slots.read().map(|s| s.clone()).unwrap_or_default()
    }

    pub(crate) fn ret_use(&self, va: u64) -> RetUse {
        self.rets.get(&va).copied().unwrap_or_default()
    }
}

/// `rets=` tables (comma-separated paths; rows `va sites rax xmm ind sig`, see
/// [`RetUse::from_row`]). Every row is kept, so late-registered DLL entries find theirs.
fn load_rets(paths: &str) -> HashMap<u64, RetUse> {
    let mut m = HashMap::new();
    for path in paths.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let Ok(text) = std::fs::read_to_string(path) else {
            log::warn!("shadow: rets table {path} not readable");
            continue;
        };
        for line in text.lines() {
            let c: Vec<&str> = line.split('\t').collect();
            if c.len() < 6 || c[0].starts_with('#') {
                continue;
            }
            let Some(va) = parse_u64(c[0]) else { continue };
            m.insert(va, RetUse::from_row(c[2], c[3], c[4] == "1", c[5]));
        }
    }
    m
}

/// A probe may start now (shadow on, none in flight, `rate` elapsed).
static ARMED: AtomicBool = AtomicBool::new(false);
static STATE: OnceLock<Option<State>> = OnceLock::new();

/// One relaxed load: whether a probe may start now. For hook sites (exe-zero cov stubs):
/// `if shadow::armed() { shadow::check(..) }`.
#[inline(always)]
pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

pub(crate) fn state() -> Option<&'static State> {
    STATE.get_or_init(init).as_ref()
}

/// [`state`] for the collector thread, which `init` spawns before `STATE` is set.
pub(crate) fn state_wait() -> Option<&'static State> {
    loop {
        if let Some(s) = STATE.get() {
            return s.as_ref();
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// VM page size used by the probe (set by the orchestrator; 16 KiB on arm64 macOS).
pub(crate) fn page_size() -> usize {
    handler::page()
}

/// Whether `va` is selected (cold: called from `resolve_dispatch_slow`; first call reads
/// `AW2_SHADOW`, builds the selection and starts the collector).
pub fn selected(va: u64) -> bool {
    state().is_some_and(|s| s.lookup(va).is_some())
}

/// Make a DLL entry known to the selection (exe-zero). Selected by `va=` or `class=dll`
/// (and not `skip=`). May be called at any time: before the shadow state exists the entry
/// is queued for `init`; afterwards (late-loaded DLLs such as cohtml/renoir) a slot is
/// added to the live selection and an idle collector is re-armed. Hook sites must use
/// [`armed`]/[`check`] directly — a dispatch already resolved via [`selected`] is not
/// revisited.
pub fn register_dll(va: u64, name: &str) {
    let info = FnInfo { va, name: name.to_string(), group: "dll", classes: C_DLL, throws: false };
    if let Ok(mut v) = EXTRA.lock() {
        v.push(info.clone());
    }
    if let Some(Some(st)) = STATE.get() {
        if st.add_late(info) {
            log::info!("shadow: late dll entry {va:#x} {name} selected");
            if !win32_rt::shadow::in_child() && st.idle.swap(false, Ordering::SeqCst) {
                ARMED.store(true, Ordering::Relaxed);
            }
        }
    }
}
static EXTRA: Mutex<Vec<FnInfo>> = Mutex::new(Vec::new());

/// Collector start (after `STATE` is set): pick up entries registered while `init` ran
/// (queued in `EXTRA` after `candidates` read it, but before `STATE` was visible).
pub(crate) fn adopt_extra(st: &State) {
    let extra: Vec<FnInfo> = EXTRA.lock().map(|v| v.clone()).unwrap_or_default();
    for f in extra {
        st.add_late(f);
    }
}

/// All registry replacements as [`FnInfo`] (class from status/group, overridden by the db).
fn candidates(spec: &Spec) -> Vec<FnInfo> {
    use crate::registry_types::{Replacement, Status, Throws};
    let mut v: Vec<FnInfo> = Vec::new();
    let mk = |r: &Replacement, group: &'static str| {
        let mut classes = if r.status == Status::Verified { C_VERIFIED } else { C_DRAFT };
        if group.starts_with("templates_") {
            classes |= C_TPL;
        }
        FnInfo { va: r.va, name: r.name.to_string(), group, classes, throws: r.throws != Throws::No }
    };
    for (g, r) in crate::hooks::collect_shadow_candidates() {
        v.push(mk(r, g));
    }
    // db/progress.tsv: `va name status rust_path notes`. Status and tpl (rust_path tpl_*)
    // override the registry's view.
    if let Ok(text) = std::fs::read_to_string(&spec.db) {
        let mut by_va: HashMap<u64, usize> = HashMap::with_capacity(v.len());
        for (i, f) in v.iter().enumerate() {
            by_va.insert(f.va, i);
        }
        for line in text.lines().skip(1) {
            let mut cols = line.split('\t');
            let (Some(va), Some(_name), Some(status)) = (cols.next(), cols.next(), cols.next()) else { continue };
            let path = cols.next().unwrap_or("");
            let Some(va) = parse_u64(va) else { continue };
            let Some(&i) = by_va.get(&va) else { continue };
            let mut c = match status.trim() {
                "verified" => C_VERIFIED,
                _ => C_DRAFT,
            };
            if path.starts_with("tpl_") {
                c |= C_TPL;
            }
            if path.contains("dll_native") {
                c |= C_DLL;
            }
            v[i].classes = c;
        }
    }
    if let Ok(extra) = EXTRA.lock() {
        v.extend(extra.iter().cloned());
    }
    v
}

fn choose(spec: &Spec, all: Vec<FnInfo>) -> Vec<FnInfo> {
    let mut seen = std::collections::HashSet::new();
    all.into_iter()
        .filter(|f| {
            let want = spec.vas.contains(&f.va)
                || spec.shards.iter().any(|s| s == f.group)
                || (spec.classes & f.classes) != 0;
            want && !spec.skip.contains(&f.va) && (spec.throws || !f.throws || spec.vas.contains(&f.va))
        })
        .filter(|f| seen.insert(f.va))
        .collect()
}

fn init() -> Option<State> {
    let raw = std::env::var("AW2_SHADOW").ok()?;
    if raw.trim().is_empty() || raw.trim() == "0" {
        return None;
    }
    let spec = match Spec::parse(&raw) {
        Ok(s) => s,
        Err(e) => {
            log::error!("{e}; AW2_SHADOW ignored");
            return None;
        }
    };
    let chosen = choose(&spec, candidates(&spec));
    if chosen.is_empty() {
        // Keep going: late `register_dll` entries (cohtml, renoir, ...) may still match.
        log::warn!("shadow: AW2_SHADOW selects no functions (yet)");
    }
    let mut fds = [0i32; 2];
    if unsafe { win32_rt::libc::pipe(fds.as_mut_ptr()) } != 0 {
        log::error!("shadow: pipe failed; AW2_SHADOW ignored");
        return None;
    }
    unsafe {
        win32_rt::libc::fcntl(fds[0], win32_rt::libc::F_SETFD, win32_rt::libc::FD_CLOEXEC);
        win32_rt::libc::fcntl(fds[1], win32_rt::libc::F_SETFD, win32_rt::libc::FD_CLOEXEC);
        let fl = win32_rt::libc::fcntl(fds[0], win32_rt::libc::F_GETFL);
        win32_rt::libc::fcntl(fds[0], win32_rt::libc::F_SETFL, fl | win32_rt::libc::O_NONBLOCK);
    }
    let _ = std::fs::create_dir_all(&spec.dir);
    let (tx, rx) = std::sync::mpsc::channel();
    log::warn!(
        "shadow: AW2_SHADOW on: {} functions selected (n={} n_va={} every={} rate={}ms timeout={}ms jit={}) -> {}/shadow.log",
        chosen.len(),
        spec.n,
        spec.n_va,
        spec.every,
        spec.rate_ms,
        spec.timeout_ms,
        spec.jit as u8,
        spec.dir
    );
    let rets = load_rets(&spec.rets);
    let covered = chosen.iter().filter(|f| rets.contains_key(&f.va)).count();
    log::warn!(
        "shadow: {} of {} selected functions have a return-use row ({} rows from {})",
        covered,
        chosen.len(),
        rets.len(),
        spec.rets
    );
    let mut st = State::new(spec, chosen, (fds[0], fds[1]), Some(tx));
    st.rets = rets;
    std::thread::Builder::new()
        .name("aw2-shadow".into())
        .spawn(move || core_probe::collector(rx))
        .ok()?;
    win32_rt::shadow::install_death_diagnostics();
    ARMED.store(true, Ordering::Relaxed);
    Some(st)
}

/// Sampling decision for `va` (armed slow path). `Some(slot index, sample number)` when this
/// call is to be probed; the caller then owns the probe (ARMED is cleared).
fn take_sample(st: &State, va: u64) -> Option<(usize, u32)> {
    // Never from a forked orchestrator/run (a run that reached probe-able code again).
    if win32_rt::shadow::in_child() {
        return None;
    }
    let i = st.lookup(va)?;
    let s = st.slot(i);
    if s.retired.load(Ordering::Relaxed) {
        return None;
    }
    let k = s.calls.fetch_add(1, Ordering::Relaxed);
    if k < s.next_at.load(Ordering::Relaxed) {
        return None;
    }
    if ARMED.compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed).is_err() {
        return None;
    }
    let n = s.samples.fetch_add(1, Ordering::Relaxed) + 1;
    if n >= s.limit {
        s.retired.store(true, Ordering::Relaxed);
    }
    s.next_at.store(k + s.every.load(Ordering::Relaxed), Ordering::Relaxed);
    Some((i, n))
}

/// Bridge hook ([`crate::emu::DF_SHADOW`] lean calls): probe this call if armed and the
/// function's budget allows. Returns immediately otherwise; never changes the caller's
/// state (the probe runs in forked children only).
#[cfg(target_arch = "aarch64")]
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn probe_lean(
    _c: &crate::callctx::CallCtx,
    va: u64,
    host_fn: usize,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    s_extra: &[u64; 8],
    rsp_window: (u64, u64),
    ret_xmm: bool,
) {
    if !armed() {
        return;
    }
    let Some(st) = state() else { return };
    let Some((slot, k)) = take_sample(st, va) else { return };
    let guest = || crate::emu::shadow_run_guest(va, rsp_window, gpr, xmm, s_extra);
    let native = || crate::emu::shadow_run_lean(host_fn, rsp_window, gpr, xmm, s_extra);
    let regs = ArgRegs { gpr: *gpr, xmm: *xmm, stack: *s_extra };
    core_probe::launch(st, slot, k, Kind::Exe, &regs, ret_xmm, &guest, &native);
}

/// Probe API for other hook sites (exe-zero DLL cov stubs): if armed and `va` is selected,
/// fork a probe comparing the guest entry `guest` (run by the emulator: `emu_run_full`, so
/// `addr | ORIG_TRAMPOLINE_TAG` works) against the native entry `native` (lean AAPCS call
/// with the Win64 argument image). Returns whether a probe started. The caller then makes
/// its normal call.
#[cfg(target_arch = "aarch64")]
pub unsafe fn check(kind: Kind, va: u64, guest: u64, native: usize, regs: &ArgRegs, ret_xmm: bool) -> bool {
    let g = || crate::emu::emu_run_full(guest, regs.gpr, regs.xmm, regs.stack);
    // The caller's window (republished before each run): `emu_run_full` reads args 17.. from
    // it, lifted natives args 9...
    let win = crate::emu::peek_incoming_rsp();
    let n = || crate::emu::shadow_run_lean(native, win, &regs.gpr, &regs.xmm, &regs.stack);
    check_with(kind, va, regs, ret_xmm, &g, &n)
}

/// [`check`] with caller-supplied runs (each runs only inside a forked child).
pub unsafe fn check_with(
    kind: Kind,
    va: u64,
    regs: &ArgRegs,
    ret_xmm: bool,
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
) -> bool {
    if !armed() {
        return false;
    }
    let Some(st) = state() else { return false };
    let Some((slot, k)) = take_sample(st, va) else { return false };
    core_probe::launch(st, slot, k, kind, regs, ret_xmm, guest, native)
}

/// Blocking probe without selection or sampling (unit tests, difftest harnesses): runs the
/// full fork/compare in the calling thread and returns the outcome.
pub unsafe fn probe_blocking(
    cfg: &ProbeCfg,
    regs: &ArgRegs,
    ret_xmm: bool,
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
) -> core_probe::Report {
    core_probe::blocking(cfg, regs, ret_xmm, guest, native)
}

/// Settings for one probe.
#[derive(Clone, Debug)]
pub struct ProbeCfg {
    pub va: u64,
    pub name: String,
    pub timeout_ms: u64,
    pub maxpages: usize,
    pub jit: bool,
    /// Write a dump file for mismatch/nondet into this directory (empty = none).
    pub dump_dir: String,
    /// Extra `[lo, hi)` ranges the comparison ignores.
    pub ignore: Vec<(usize, usize)>,
    /// Image segments (hard) and this thread's TLV blocks (soft), computed in the parent
    /// before the fork: the TLV thunks can take dyld's lock, which another thread may hold
    /// at fork time.
    pub(crate) image: (Vec<(usize, usize)>, Vec<(usize, usize)>),
    /// Return-register bits the comparison uses (default: all).
    pub ret: RetUse,
}

impl ProbeCfg {
    pub fn new(va: u64, name: &str) -> ProbeCfg {
        ProbeCfg {
            va,
            name: name.to_string(),
            timeout_ms: 2000,
            maxpages: 4096,
            jit: false,
            dump_dir: String::new(),
            ignore: Vec::new(),
            image: (Vec::new(), Vec::new()),
            ret: RetUse::default(),
        }
    }
}

#[cfg(test)]
mod tests;
