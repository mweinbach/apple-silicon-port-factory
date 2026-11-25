//! `AW2_JIT_STATS=<secs>` (diagnostic only, default off): runtime coverage counters for the
//! x86->arm64 block translator, logged every `<secs>` seconds (cumulative, all threads).
//!
//! Counts, weighted by execution:
//! - guest instructions executed in translated code vs. interpreted while the translator is on,
//!   split into generic-path instructions (`K_GENERIC`, which end a translated block) and
//!   fast-path instructions the translator didn't run (cold blocks / kinds `jit::supported`
//!   rejects / side exits);
//! - translated-code exits by reason (`EXIT_RIP` [and how many of those re-entered translated
//!   code immediately, i.e. a wasted exit+enter round trip], host-dispatch `EXIT_CALL`/`JMP`/`JCC`,
//!   `EXIT_SIDE`, zero-progress exits);
//! - the top interpreted mnemonics and guest IPs, side-exit IPs and host-exit targets.
//!
//! Only active with the translator on (it is off when `AW2_EMU_PROF>=2` or `AW2_EMU_VERIFY=1`).
//! Costs a TLS access + hash-map update per interpreted instruction while enabled.

use iced_x86::Mnemonic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const MN: usize = 2048;
const FLUSH_EVERY: u64 = 1 << 18;

fn period_secs() -> u64 {
    static V: OnceLock<u64> = OnceLock::new();
    *V.get_or_init(|| {
        let s = std::env::var("AW2_JIT_STATS").ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(0);
        if s != 0 {
            log::info!("emu jit stats: enabled (AW2_JIT_STATS={s}s)");
            win32_rt::process::register_exit_hook(flush_on_exit);
        }
        s
    })
}

/// Whether `AW2_JIT_STATS` is set (non-zero).
#[inline]
pub(super) fn enabled() -> bool {
    period_secs() != 0
}

#[derive(Default)]
struct Counts {
    jit_insns: u64,
    aot_insns: u64,
    runtime_jit_insns: u64,
    jit_entries: u64,
    aot_entries: u64,
    runtime_jit_entries: u64,
    /// EXIT_RIP, EXIT_CALL, EXIT_JMP, EXIT_JCC, EXIT_SIDE, other, zero-progress.
    exits: [u64; 7],
    rip_rehit: u64,
    interp_generic: u64,
    interp_fast: u64,
    mn_generic: Vec<u64>,
    mn_fast: Vec<u64>,
    /// ip -> (count, mnemonic)
    ip_generic: HashMap<u64, (u64, u16)>,
    ip_fast: HashMap<u64, (u64, u16)>,
    ip_side: HashMap<u64, u64>,
    host_exit: HashMap<u64, u64>,
    /// Host calls / tail jumps made from translated code (`jit_host_xfer`, no exit).
    xfer_call: u64,
    xfer_jmp: u64,
    xfer_target: HashMap<u64, u64>,
    /// target -> [static call, dynamic call, static jmp, dynamic jmp].
    xfer_shape: HashMap<u64, [u64; 4]>,
    /// (target, return address: of the call, or on the stack for tail jumps) -> count.
    xfer_site: HashMap<(u64, u64), u64>,
    /// Transfers served by the `xfer_plain` fast path (no bridge).
    xfer_plain: u64,
    events: u64,
}

impl Counts {
    fn new() -> Self {
        Self { mn_generic: vec![0; MN], mn_fast: vec![0; MN], ..Self::default() }
    }

    fn merge_into(&mut self, g: &mut Counts) {
        g.jit_insns += self.jit_insns;
        g.aot_insns += self.aot_insns;
        g.runtime_jit_insns += self.runtime_jit_insns;
        g.jit_entries += self.jit_entries;
        g.aot_entries += self.aot_entries;
        g.runtime_jit_entries += self.runtime_jit_entries;
        for i in 0..self.exits.len() {
            g.exits[i] += self.exits[i];
        }
        g.rip_rehit += self.rip_rehit;
        g.interp_generic += self.interp_generic;
        g.interp_fast += self.interp_fast;
        for i in 0..MN {
            g.mn_generic[i] += self.mn_generic[i];
            g.mn_fast[i] += self.mn_fast[i];
        }
        for (k, v) in self.ip_generic.drain() {
            let e = g.ip_generic.entry(k).or_insert((0, v.1));
            e.0 += v.0;
        }
        for (k, v) in self.ip_fast.drain() {
            let e = g.ip_fast.entry(k).or_insert((0, v.1));
            e.0 += v.0;
        }
        for (k, v) in self.ip_side.drain() {
            *g.ip_side.entry(k).or_insert(0) += v;
        }
        for (k, v) in self.host_exit.drain() {
            *g.host_exit.entry(k).or_insert(0) += v;
        }
        g.xfer_call += self.xfer_call;
        g.xfer_jmp += self.xfer_jmp;
        for (k, v) in self.xfer_target.drain() {
            *g.xfer_target.entry(k).or_insert(0) += v;
        }
        for (k, v) in self.xfer_shape.drain() {
            let e = g.xfer_shape.entry(k).or_insert([0; 4]);
            for i in 0..4 {
                e[i] += v[i];
            }
        }
        for (k, v) in self.xfer_site.drain() {
            *g.xfer_site.entry(k).or_insert(0) += v;
        }
        g.xfer_plain += self.xfer_plain;
        let (mg, mf) = (core::mem::take(&mut self.mn_generic), core::mem::take(&mut self.mn_fast));
        *self = Counts { mn_generic: mg, mn_fast: mf, ..Counts::default() };
        self.mn_generic.fill(0);
        self.mn_fast.fill(0);
    }
}

struct Global {
    c: Counts,
    start: Instant,
    last: Instant,
}

static GLOBAL: Mutex<Option<Global>> = Mutex::new(None);

struct TlsCounts(RefCell<Option<Counts>>);

impl Drop for TlsCounts {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            if let Some(c) = b.as_mut() {
                if c.events > 0 {
                    if let Ok(mut g) = GLOBAL.lock() {
                        let now = Instant::now();
                        let g = g.get_or_insert_with(|| Global {
                            c: Counts::new(),
                            start: now,
                            last: now,
                        });
                        c.merge_into(&mut g.c);
                    }
                }
            }
        }
    }
}

std::thread_local! {
    static TLS: TlsCounts = const { TlsCounts(RefCell::new(None)) };
}

#[inline]
fn with<F: FnOnce(&mut Counts)>(f: F) {
    TLS.with(|t| {
        let mut b = t.0.borrow_mut();
        let c = b.get_or_insert_with(Counts::new);
        f(c);
        c.events += 1;
        if c.events >= FLUSH_EVERY {
            flush(c);
        }
    });
}

#[cold]
fn flush(c: &mut Counts) {
    let Ok(mut g) = GLOBAL.lock() else { return };
    let now = Instant::now();
    let g = g.get_or_insert_with(|| Global { c: Counts::new(), start: now, last: now });
    c.merge_into(&mut g.c);
    if now.duration_since(g.last).as_secs() >= period_secs() {
        g.last = now;
        log::info!("{}", report(&g.c, now.duration_since(g.start).as_secs_f64()));
    }
}

/// Process exit hook: merge current thread's buffered counters and emit the cumulative summary.
fn flush_on_exit() {
    if !enabled() {
        return;
    }
    let _ = TLS.try_with(|t| {
        if let Ok(mut b) = t.0.try_borrow_mut() {
            if let Some(c) = b.as_mut() {
                if c.events > 0 {
                    if let Ok(mut g) = GLOBAL.try_lock() {
                        let now = Instant::now();
                        let g = g.get_or_insert_with(|| Global {
                            c: Counts::new(),
                            start: now,
                            last: now,
                        });
                        c.merge_into(&mut g.c);
                    }
                }
            }
        }
    });
    if let Ok(g) = GLOBAL.try_lock() {
        if let Some(g) = g.as_ref() {
            log::info!("{}", report(&g.c, Instant::now().duration_since(g.start).as_secs_f64()));
        }
    }
}

/// Translated code ran `executed` guest instructions and exited with `ex`.
#[inline]
pub(super) fn jit_run(ex: u64, executed: i64, target: u64, rip: u64, from_aot: bool) {
    with(|c| {
        let n = executed.max(0) as u64;
        c.jit_entries += 1;
        c.jit_insns += n;
        if from_aot {
            c.aot_entries += 1;
            c.aot_insns += n;
        } else {
            c.runtime_jit_entries += 1;
            c.runtime_jit_insns += n;
        }
        let i = match ex {
            super::jit::EXIT_RIP => 0,
            super::jit::EXIT_CALL => 1,
            super::jit::EXIT_JMP => 2,
            super::jit::EXIT_JCC => 3,
            super::jit::EXIT_SIDE => 4,
            _ => 5,
        };
        c.exits[i] += 1;
        if executed <= 0 {
            c.exits[6] += 1;
        }
        match i {
            1..=3 => *c.host_exit.entry(target).or_insert(0) += 1,
            4 => *c.ip_side.entry(rip).or_insert(0) += 1,
            _ => {}
        }
    });
}

/// The lookup right after an `EXIT_RIP` found translated code (wasted exit/enter round trip).
#[inline]
pub(super) fn rip_rehit() {
    with(|c| c.rip_rehit += 1);
}

/// A host call (`tail` = false) / tail jump made from translated code without an exit;
/// `dynamic`: register/memory-indirect target (`lookup_call`/`lookup_jmp`); `site`: the
/// call's return address (tail jumps: the return address on the stack).
pub(super) fn host_xfer(target: u64, tail: bool, dynamic: bool, site: u64) {
    with(|c| {
        if tail {
            c.xfer_jmp += 1;
        } else {
            c.xfer_call += 1;
        }
        *c.xfer_target.entry(target).or_insert(0) += 1;
        c.xfer_shape.entry(target).or_insert([0; 4])[tail as usize * 2 + dynamic as usize] += 1;
        *c.xfer_site.entry((target, site)).or_insert(0) += 1;
    });
}

/// A host transfer served by `xfer_plain` (inline-callable target reached indirectly).
pub(super) fn xfer_plain() {
    with(|c| c.xfer_plain += 1);
}

/// Per-thread translation work: thread label -> [`TW_BLOCKS`] .. [`TW_PROBE_HIT`].
static COMPILE: Mutex<Option<HashMap<String, [u64; 7]>>> = Mutex::new(None);
/// [`COMPILE`] columns: blocks translated, translations rejected, total ns, AOT blocks
/// installed into the thread's index, index misses the AOT directory could not serve,
/// direct-link sites left unpatched because their AOT target is out of `b` range, stub
/// lookups served by the shared AOT probe table (`AW2_DLL_AOT_PROBE`).
const TW_BLOCKS: usize = 0;
const TW_REJECTED: usize = 1;
const TW_NS: usize = 2;
pub(super) const TW_AOT_INSTALL: usize = 3;
pub(super) const TW_AOT_MISS: usize = 4;
pub(super) const TW_FAR: usize = 5;
pub(super) const TW_PROBE_HIT: usize = 6;

std::thread_local! {
    static THREAD_LABEL: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// `JitCache::translate` took `ns` on this thread (`ok`: a block was translated).
#[cold]
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn compile(ns: u64, ok: bool) {
    thread_work(|e| {
        e[if ok { TW_BLOCKS } else { TW_REJECTED }] += 1;
        e[TW_NS] += ns;
    });
}

/// Per-thread AOT event `col` ([`TW_AOT_INSTALL`] / [`TW_AOT_MISS`] / [`TW_FAR`] /
/// [`TW_PROBE_HIT`]), `n` times.
#[cold]
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn aot_event(col: usize, n: u64) {
    thread_work(|e| e[col] += n);
}

fn thread_work(f: impl FnOnce(&mut [u64; 7])) {
    let label = THREAD_LABEL.with(|l| {
        l.borrow_mut()
            .get_or_insert_with(|| {
                let t = std::thread::current();
                match t.name() {
                    Some(n) => format!("{n} ({:?})", t.id()),
                    None => format!("{:?}", t.id()),
                }
            })
            .clone()
    });
    if let Ok(mut g) = COMPILE.lock() {
        f(g.get_or_insert_with(HashMap::new).entry(label).or_insert([0; 7]));
    }
}

/// The call instruction(s) that could end at `ret` (a guest return address in the exe or a
/// mapped DLL image), disassembled (every length that decodes to a call, ` | `-separated:
/// decoding backwards is ambiguous), or "" when there is none.
fn call_before(ret: u64) -> String {
    use iced_x86::{Decoder, DecoderOptions, FlowControl, Formatter, IntelFormatter};
    let (b, pref, sz) = (crate::exe::base() as u64, crate::exe::preferred_base(), crate::exe::image_size());
    let in_image = (b + 16..b.wrapping_add(sz)).contains(&ret)
        || (pref + 16..pref.wrapping_add(sz)).contains(&ret)
        || (crate::dll_native::in_mapped_image(ret) && crate::dll_native::in_mapped_image(ret - 8));
    if !in_image {
        return String::new();
    }
    let mut out = Vec::new();
    for len in [2usize, 3, 4, 5, 6, 7] {
        let start = ret - len as u64;
        let bytes = unsafe { std::slice::from_raw_parts(super::to_mapped_addr(start) as *const u8, len) };
        let mut d = Decoder::with_ip(64, bytes, start, DecoderOptions::NONE);
        let ins = d.decode();
        if !ins.is_invalid()
            && ins.len() == len
            && matches!(ins.flow_control(), FlowControl::Call | FlowControl::IndirectCall)
        {
            let mut s = String::new();
            IntelFormatter::new().format(&ins, &mut s);
            out.push(s);
        }
    }
    out.join(" | ")
}

/// One instruction (the head of a fast chain) interpreted while the translator is on.
#[inline]
pub(super) fn interp(ip: u64, mn: Mnemonic, generic: bool) {
    with(|c| {
        let m = (mn as usize).min(MN - 1);
        if generic {
            c.interp_generic += 1;
            c.mn_generic[m] += 1;
            c.ip_generic.entry(ip).or_insert((0, m as u16)).0 += 1;
        } else {
            c.interp_fast += 1;
            c.mn_fast[m] += 1;
            c.ip_fast.entry(ip).or_insert((0, m as u16)).0 += 1;
        }
    });
}

/// `n` further fast-path instructions interpreted as part of a chain.
#[inline]
pub(super) fn interp_chain(n: u64) {
    with(|c| c.interp_fast += n);
}

fn mname(i: usize) -> String {
    Mnemonic::try_from(i).map(|m| format!("{m:?}")).unwrap_or_else(|_| format!("#{i}"))
}

fn pct(a: u64, t: u64) -> f64 {
    if t == 0 { 0.0 } else { a as f64 * 100.0 / t as f64 }
}

fn report(c: &Counts, secs: f64) -> String {
    use std::fmt::Write;
    let total = c.jit_insns + c.interp_generic + c.interp_fast;
    let mut o = String::new();
    let _ = writeln!(o, "emu jit stats (cumulative, all threads, {secs:.0}s):");
    let _ = writeln!(
        o,
        "  guest instrs {total}: jit {} ({:.2}% [aot {} ({:.2}%), runtime_jit {} ({:.2}%)]), interp generic {} ({:.2}%), interp fast {} ({:.2}%)",
        c.jit_insns,
        pct(c.jit_insns, total),
        c.aot_insns,
        pct(c.aot_insns, total),
        c.runtime_jit_insns,
        pct(c.runtime_jit_insns, total),
        c.interp_generic,
        pct(c.interp_generic, total),
        c.interp_fast,
        pct(c.interp_fast, total)
    );
    let _ = writeln!(
        o,
        "  jit entries {} (aot {}, runtime_jit {}, {:.1} instrs/entry); exits: rip {} (re-entered at once {}), host call {}, host jmp {}, host jcc {}, side {}, other {}, zero-progress {}",
        c.jit_entries,
        c.aot_entries,
        c.runtime_jit_entries,
        c.jit_insns as f64 / c.jit_entries.max(1) as f64,
        c.exits[0],
        c.rip_rehit,
        c.exits[1],
        c.exits[2],
        c.exits[3],
        c.exits[4],
        c.exits[5],
        c.exits[6]
    );
    let _ = writeln!(o, "  in-translation host calls {} / tail jumps {} (no exit)", c.xfer_call, c.xfer_jmp);
    // Per-thread translation time (all translations since start, not reset by flushes).
    if let Ok(g) = COMPILE.try_lock() {
        if let Some(m) = g.as_ref() {
            let mut v: Vec<(&String, &[u64; 7])> = m.iter().collect();
            v.sort_by_key(|(_, e)| core::cmp::Reverse((e[TW_NS], e[TW_AOT_INSTALL] + e[TW_PROBE_HIT])));
            let mut t = [0u64; 7];
            for (_, e) in &v {
                for (a, b) in t.iter_mut().zip(e.iter()) {
                    *a += b;
                }
            }
            let _ = writeln!(
                o,
                "  jit compile time: {} us total, {} blocks translated, {} rejected ({:.1} us/translation), {} threads; aot: {} installs, {} misses, {} far JIT->AOT link sites, {} probe-table hits:",
                t[TW_NS] / 1000,
                t[TW_BLOCKS],
                t[TW_REJECTED],
                t[TW_NS] as f64 / 1000.0 / (t[TW_BLOCKS] + t[TW_REJECTED]).max(1) as f64,
                v.len(),
                t[TW_AOT_INSTALL],
                t[TW_AOT_MISS],
                t[TW_FAR],
                t[TW_PROBE_HIT]
            );
            for (label, e) in v.iter().take(24) {
                let _ = writeln!(
                    o,
                    "    {:>12} us  {:>8} blocks  {:>6} rejected  {:6.1} us/block  aot {:>8} installs {:>8} misses {:>6} far {:>10} probe  {}",
                    e[TW_NS] / 1000,
                    e[TW_BLOCKS],
                    e[TW_REJECTED],
                    e[TW_NS] as f64 / 1000.0 / (e[TW_BLOCKS] + e[TW_REJECTED]).max(1) as f64,
                    e[TW_AOT_INSTALL],
                    e[TW_AOT_MISS],
                    e[TW_FAR],
                    e[TW_PROBE_HIT],
                    label
                );
            }
        }
    }
    #[cfg(target_arch = "aarch64")]
    if super::aot::active() {
        let a = super::aot::counts();
        let _ = writeln!(
            o,
            "  aot invalidation: {} blocks killed, {} revived, {} would revive (AW2_DLL_AOT_REVIVE=0)",
            a.kills, a.revived, a.would_revive
        );
    }
    let top_mn = |o: &mut String, v: &[u64], what: &str, sum: u64| {
        let mut idx: Vec<usize> = (0..MN).filter(|&i| v[i] != 0).collect();
        idx.sort_by_key(|&i| core::cmp::Reverse(v[i]));
        let _ = writeln!(o, "  top {what} mnemonics:");
        for &i in idx.iter().take(40) {
            let _ = writeln!(o, "    {:>12} {:6.2}%  {}", v[i], pct(v[i], sum), mname(i));
        }
    };
    top_mn(&mut o, &c.mn_generic, "interpreted generic", c.interp_generic);
    top_mn(&mut o, &c.mn_fast, "interpreted fast (chain heads)", c.interp_fast);
    let top_ip = |o: &mut String, m: &HashMap<u64, (u64, u16)>, what: &str| {
        let mut v: Vec<(&u64, &(u64, u16))> = m.iter().collect();
        v.sort_by_key(|(_, &(n, _))| core::cmp::Reverse(n));
        let _ = writeln!(o, "  top {what} ips:");
        for (ip, (n, mn)) in v.into_iter().take(60) {
            let _ = writeln!(o, "    {:>12}  {:#x}  {}", n, ip, mname(*mn as usize));
        }
    };
    top_ip(&mut o, &c.ip_generic, "interpreted generic");
    top_ip(&mut o, &c.ip_fast, "interpreted fast");
    let top_plain = |o: &mut String, m: &HashMap<u64, u64>, what: &str| {
        let mut v: Vec<(&u64, &u64)> = m.iter().collect();
        v.sort_by_key(|(_, &n)| core::cmp::Reverse(n));
        let _ = writeln!(o, "  top {what}:");
        for (ip, n) in v.into_iter().take(30) {
            let host = super::resolve_host_callable(*ip).is_some();
            let _ = writeln!(
                o,
                "    {:>12}  {:#x} {} {}",
                n,
                ip,
                if host { "[host]" } else { "[guest]" },
                win32_rt::symbolize(*ip as usize)
            );
        }
    };
    top_plain(&mut o, &c.ip_side, "side-exit ips");
    top_plain(&mut o, &c.host_exit, "EXIT_CALL/JMP/JCC targets (host dispatch or lookup miss)");
    let _ = writeln!(
        o,
        "  top in-translation host call/jmp targets ([call static/dynamic, jmp static/dynamic]; {} served by the indirect plain fast path):",
        c.xfer_plain
    );
    let mut v: Vec<(&u64, &u64)> = c.xfer_target.iter().collect();
    v.sort_by_key(|(_, &n)| core::cmp::Reverse(n));
    for (&t, &n) in v.into_iter().take(30) {
        let host = super::resolve_host_callable(t).is_some();
        let sh = c.xfer_shape.get(&t).copied().unwrap_or_default();
        let _ = writeln!(
            o,
            "    {:>12}  {:#x} {} {} [call {}/{}, jmp {}/{}]",
            n,
            t,
            if host { "[host]" } else { "[guest]" },
            win32_rt::symbolize(t as usize),
            sh[0],
            sh[1],
            sh[2],
            sh[3]
        );
        let mut sites: Vec<(u64, u64)> = c.xfer_site.iter().filter(|((tt, _), _)| *tt == t).map(|(&(_, s), &k)| (s, k)).collect();
        sites.sort_by_key(|&(_, k)| core::cmp::Reverse(k));
        for (s, k) in sites.into_iter().take(3) {
            let _ = writeln!(o, "          {:>10}  ret {:#x} {}  {}", k, s, win32_rt::symbolize(s as usize), call_before(s));
        }
    }
    o
}
