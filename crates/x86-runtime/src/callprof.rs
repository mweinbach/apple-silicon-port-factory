//! Call profilers for the lifted-call bridge (`call_va8` / `call_va8_xmm`), diagnostics only.
//!
//! - `AW2_CALL_PROF=1`: per-thread edges keyed by the host return address of the bridge
//!   (the lifted caller) and the target VA. Every ~10 s each busy thread logs its top edges
//!   as `callprof` lines, so hot edges can be turned into direct calls (`call_native!`).
//! - `AW2_CALLVA_PROF=1`: process-wide histogram. Every 30 s it logs cumulative `callva`
//!   tables (tab-separated):
//!   - the top 200 targets, calling functions and (caller, target) edges;
//!   - the dispatch path mix: `lean` direct replacement call, `plain` builtin, `host` full
//!     bridge, `emu` x86 (JIT/AOT/interpreter), `orig` forced-original;
//!   - the stack-arg window case, where `zero`/`partial` go through the out-of-line
//!     `read_caller_extra_args_partial`;
//!   - per-thread totals.
//!   The caller is the first frame above the bridge that isn't a `call_*` / `tail_stk*`
//!   helper. Both knobs cost one relaxed load per call when off (the existing `on()` check).

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::Instant;

/// 0 = off, 1 = on, 2 = not read yet.
static ON: AtomicU8 = AtomicU8::new(2);
/// `AW2_CALLVA_PROF`: process-wide histogram instead of the per-thread edge dump.
static VA_MODE: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn on() -> bool {
    match ON.load(Ordering::Relaxed) {
        0 => false,
        1 => true,
        _ => init(),
    }
}

#[cold]
fn init() -> bool {
    let legacy = std::env::var_os("AW2_CALL_PROF").is_some_and(|v| v != "0");
    let va = std::env::var_os("AW2_CALLVA_PROF").is_some_and(|v| v != "0");
    VA_MODE.store(va, Ordering::Relaxed);
    let on = legacy || va;
    if ON.swap(on as u8, Ordering::Relaxed) == 2 && va {
        log::info!(
            "callva: AW2_CALLVA_PROF=1: counting every call_va8/call_va8_xmm dispatch (target, caller, path); top {} tables every {} s",
            va_prof::TOP,
            va_prof::dump_secs()
        );
    }
    on
}

/// `hit` entry kinds.
pub const ENTRY_VA8: u8 = 0;
pub const ENTRY_XMM: u8 = 1;

struct Local {
    /// (caller return address, target VA) -> (calls, host fn or 0 = interpreted).
    edges: HashMap<(u64, u64), (u64, usize)>,
    hits: u64,
    last: Instant,
}

thread_local! {
    static L: RefCell<Option<Local>> = const { RefCell::new(None) };
}

#[repr(C)]
struct DlInfo {
    fname: *const i8,
    fbase: *mut u8,
    sname: *const i8,
    saddr: *mut u8,
}

extern "C" {
    fn dladdr(addr: *const u8, info: *mut DlInfo) -> i32;
}

fn sym(addr: usize) -> String {
    if addr == 0 {
        return "<interp>".into();
    }
    let mut i = DlInfo { fname: std::ptr::null(), fbase: std::ptr::null_mut(), sname: std::ptr::null(), saddr: std::ptr::null_mut() };
    // SAFETY: dladdr only reads loader tables; the out struct is valid.
    if unsafe { dladdr(addr as *const u8, &mut i) } != 0 && !i.sname.is_null() {
        let s = unsafe { std::ffi::CStr::from_ptr(i.sname) }.to_string_lossy();
        return format!("{}+{:#x}", short(&s), addr - i.saddr as usize);
    }
    format!("{addr:#x}")
}

/// Last two path segments of a legacy-mangled Rust symbol (`t38::fn_1422fe4e0`).
fn short(s: &str) -> String {
    let b = s.as_bytes();
    let mut out: Vec<&str> = Vec::new();
    let mut i = s.find("_8aw2_").map_or(0, |p| p + 1);
    while i < b.len() {
        if !b[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let st = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        let n: usize = s[st..i].parse().unwrap_or(0);
        if n == 0 || i + n > b.len() {
            break;
        }
        out.push(&s[i..i + n]);
        i += n;
    }
    if out.len() >= 2 {
        format!("{}::{}", out[out.len() - 2], out[out.len() - 1])
    } else {
        s.to_string()
    }
}

/// Record one bridge dispatch. Must be called directly from the bridge function
/// (reads the frame records above it for the caller).
/// - `va_raw`: the target as passed to the bridge (bit 46 = forced original).
/// - `dispatch`: its `emu::resolve_dispatch` result.
#[cold]
#[inline(never)]
pub fn hit(c: &crate::callctx::CallCtx, va_raw: u64, dispatch: (usize, u32), entry: u8) {
    let fp: usize;
    // SAFETY: reads this function's own frame pointer.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("mov {}, x29", out(reg) fp, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        fp = 0;
    }
    if VA_MODE.load(Ordering::Relaxed) {
        // SAFETY: frame pointers are always kept on aarch64 macOS; the walk is bounded.
        let rets = unsafe { va_prof::walk(fp) };
        va_prof::record(c, rets, va_raw, dispatch, entry);
        return;
    }
    let va = va_raw & !(1u64 << 46);
    let host_fn = dispatch.0;
    let ret: u64;
    // SAFETY: frame pointers are always kept on aarch64 macOS; [x29] is the
    // bridge's frame record and [fp + 8] its saved return address.
    unsafe {
        ret = if fp == 0 {
            0
        } else {
            let bridge_fp = *(fp as *const usize);
            if bridge_fp == 0 { 0 } else { *((bridge_fp + 8) as *const u64) }
        };
    }
    L.with(|l| {
        let mut l = l.borrow_mut();
        let st = l.get_or_insert_with(|| Local { edges: HashMap::new(), hits: 0, last: Instant::now() });
        let e = st.edges.entry((ret, va)).or_insert((0, host_fn));
        e.0 += 1;
        st.hits += 1;
        if st.hits & 0xffff == 0 {
            let dt = st.last.elapsed().as_secs_f64();
            if dt >= 10.0 {
                dump(st, dt);
                st.edges.clear();
                st.hits = 0;
                st.last = Instant::now();
            }
        }
    });
}

fn dump(st: &Local, dt: f64) {
    let total: u64 = st.edges.values().map(|v| v.0).sum();
    let rate = total as f64 / dt;
    if rate < 20_000.0 {
        return;
    }
    let tid = format!("{:?}", std::thread::current().id());
    let mut v: Vec<_> = st.edges.iter().collect();
    v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    // Per-target totals (all callers).
    let mut by_t: HashMap<u64, (u64, usize)> = HashMap::new();
    for ((_, va), (n, h)) in &st.edges {
        let e = by_t.entry(*va).or_insert((0, *h));
        e.0 += n;
    }
    let mut t: Vec<_> = by_t.into_iter().collect();
    t.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    let mut out = format!("callprof {tid}: {rate:.0} calls/s over {dt:.1}s, {} edges\n", st.edges.len());
    for (va, (n, h)) in t.iter().take(25) {
        out += &format!("callprof {tid} target {:9.0}/s {va:#x} -> {}\n", *n as f64 / dt, sym(*h));
    }
    for ((ret, va), (n, h)) in v.iter().take(40) {
        out += &format!(
            "callprof {tid} edge {:9.0}/s {} -> {va:#x} {}\n",
            *n as f64 / dt,
            sym(*ret as usize),
            sym(*h)
        );
    }
    // Interpreted targets are rare but expensive per call; list them on their own.
    for ((ret, va), (n, _)) in v.iter().filter(|e| e.1 .1 == 0).take(25) {
        out += &format!("callprof {tid} interp {:9.0}/s {} -> {va:#x}\n", *n as f64 / dt, sym(*ret as usize));
    }
    log::info!("{out}");
}

/// `AW2_CALLVA_PROF=1`: process-wide `call_va8` histogram (see the module docs).
mod va_prof {
    use super::{short, ENTRY_XMM};
    use crate::callctx::CallCtx;
    use crate::emu::{DF_HOST, DF_PLAIN, DF_VALID, LEAN_FLAGS};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::Write as _;
    use std::sync::{Mutex, OnceLock};
    use std::time::Instant;

    pub const TOP: usize = 200;
    /// Dump interval: `AW2_CALLVA_PROF_SECS` (default 30).
    pub fn dump_secs() -> u64 {
        static S: OnceLock<u64> = OnceLock::new();
        *S.get_or_init(|| {
            std::env::var("AW2_CALLVA_PROF_SECS").ok().and_then(|v| v.trim().parse().ok()).filter(|&v| v > 0).unwrap_or(30)
        })
    }
    /// Frames recorded above `hit` (bridge, `call_stk*` wrapper, caller, caller's caller).
    const DEPTH: usize = 4;

    const PATHS: [&str; 5] = ["lean", "plain", "host", "emu", "orig"];
    const P_LEAN: u8 = 0;
    const P_PLAIN: u8 = 1;
    const P_HOST: u8 = 2;
    const P_EMU: u8 = 3;
    const P_ORIG: u8 = 4;
    /// Stack-arg window of the call (`emu::read_caller_extra_args`): `none` = not read
    /// (plain path), `zero` = no caller rsp, `full` = inline 64-byte copy, `partial` =
    /// window past the limit. `zero` and `partial` call `read_caller_extra_args_partial`.
    const XARGS: [&str; 4] = ["none", "zero", "full", "partial"];

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct Key {
        rets: [u64; DEPTH],
        va: u64,
        /// path | xargs << 3 | entry << 5
        meta: u8,
    }

    struct Local {
        map: HashMap<Key, (u64, usize)>,
        hits: u64,
        last_flush: Instant,
        thread: String,
    }

    thread_local! {
        static L: RefCell<Option<Local>> = const { RefCell::new(None) };
    }

    struct Global {
        map: HashMap<Key, (u64, usize)>,
        threads: HashMap<String, u64>,
        start: Instant,
        last_dump: Instant,
        last_total: u64,
        dumps: u32,
        /// addr -> short function name (no offset), `None` = unresolved.
        syms: HashMap<u64, Option<String>>,
    }

    fn global() -> &'static Mutex<Global> {
        static G: OnceLock<Mutex<Global>> = OnceLock::new();
        G.get_or_init(|| {
            let now = Instant::now();
            Mutex::new(Global {
                map: HashMap::new(),
                threads: HashMap::new(),
                start: now,
                last_dump: now,
                last_total: 0,
                dumps: 0,
                syms: HashMap::new(),
            })
        })
    }

    /// Return addresses of the `DEPTH` frames above `hit` (`fp` = `hit`'s frame record).
    pub unsafe fn walk(fp: usize) -> [u64; DEPTH] {
        let mut r = [0u64; DEPTH];
        let mut f = fp;
        for slot in r.iter_mut() {
            if f == 0 || f & 7 != 0 {
                break;
            }
            *slot = *((f + 8) as *const u64);
            let next = *(f as *const usize);
            if next <= f {
                break;
            }
            f = next;
        }
        r
    }

    pub fn record(c: &CallCtx, rets: [u64; DEPTH], va_raw: u64, dispatch: (usize, u32), entry: u8) {
        let (host_fn, flags) = dispatch;
        let xmm = entry == ENTRY_XMM;
        let path = if va_raw & (1u64 << 46) != 0 {
            P_ORIG
        } else if flags == LEAN_FLAGS {
            P_LEAN
        } else if !xmm && flags == DF_VALID | DF_HOST | DF_PLAIN && c.outgoing_rsp.get().0 == 0 && !c.xmm_in_set.get() {
            P_PLAIN
        } else if flags & DF_HOST != 0 {
            P_HOST
        } else {
            P_EMU
        };
        let (rsp, limit) = if xmm {
            c.outgoing_rsp.get()
        } else {
            let o = c.outgoing_rsp.get();
            if o.0 != 0 { o } else { c.incoming_rsp.get() }
        };
        let xargs: u8 = if path == P_PLAIN {
            0
        } else if rsp == 0 {
            1
        } else {
            let base = rsp.wrapping_add(0x40);
            if base <= base.wrapping_add(0x40) && base.wrapping_add(0x40) <= limit { 2 } else { 3 }
        };
        let key = Key { rets, va: va_raw & !(1u64 << 46), meta: path | xargs << 3 | entry << 5 };
        L.with(|l| {
            let mut l = l.borrow_mut();
            let st = l.get_or_insert_with(|| Local {
                map: HashMap::new(),
                hits: 0,
                last_flush: Instant::now(),
                thread: std::thread::current()
                    .name()
                    .map(str::to_owned)
                    .unwrap_or_else(|| format!("{:?}", std::thread::current().id())),
            });
            st.map.entry(key).or_insert((0, host_fn)).0 += 1;
            st.hits += 1;
            if st.hits & 0xfff == 0 && st.last_flush.elapsed().as_millis() >= 1000 {
                flush(st);
            }
        });
    }

    fn flush(st: &mut Local) {
        let mut g = global().lock().unwrap_or_else(|e| e.into_inner());
        for (k, (n, h)) in st.map.drain() {
            g.map.entry(k).or_insert((0, h)).0 += n;
        }
        *g.threads.entry(st.thread.clone()).or_insert(0) += st.hits;
        st.hits = 0;
        st.last_flush = Instant::now();
        if g.last_dump.elapsed().as_secs() >= dump_secs() {
            dump(&mut g);
        }
    }

    fn sym_name(syms: &mut HashMap<u64, Option<String>>, addr: u64) -> Option<String> {
        if addr == 0 {
            return None;
        }
        syms.entry(addr)
            .or_insert_with(|| {
                let mut i = super::DlInfo {
                    fname: std::ptr::null(),
                    fbase: std::ptr::null_mut(),
                    sname: std::ptr::null(),
                    saddr: std::ptr::null_mut(),
                };
                // SAFETY: dladdr only reads loader tables; the out struct is valid.
                if unsafe { super::dladdr(addr as *const u8, &mut i) } != 0 && !i.sname.is_null() {
                    let s = unsafe { std::ffi::CStr::from_ptr(i.sname) }.to_string_lossy();
                    Some(short(&s))
                } else {
                    None
                }
            })
            .clone()
    }

    /// Bridge plumbing between `hit` and the lifted caller.
    fn is_bridge(name: &str) -> bool {
        let last = name.rsplit("::").next().unwrap_or(name);
        last.starts_with("call_va")
            || last.starts_with("call_stk")
            || last.starts_with("tail_stk")
            || last.starts_with("call_lean")
            || last.starts_with("call_host")
            || last == "hit"
    }

    fn pct(n: u64, total: u64) -> f64 {
        if total == 0 { 0.0 } else { n as f64 * 100.0 / total as f64 }
    }

    /// Dominant path of a per-path count array, with its share when mixed.
    fn path_mix(p: &[u64; 5]) -> String {
        let total: u64 = p.iter().sum();
        let (i, n) = p.iter().enumerate().max_by_key(|(_, n)| **n).map(|(i, n)| (i, *n)).unwrap_or((0, 0));
        if n == total {
            PATHS[i].to_string()
        } else {
            format!("{}({:.0}%)+", PATHS[i], pct(n, total))
        }
    }

    fn top_of<K: Clone>(m: &HashMap<K, u64>) -> Option<(K, u64)> {
        m.iter().max_by_key(|(_, n)| **n).map(|(k, n)| (k.clone(), *n))
    }

    #[derive(Default)]
    struct Agg {
        n: u64,
        paths: [u64; 5],
        host_fn: usize,
        by: HashMap<String, u64>,
    }

    fn dump(g: &mut Global) {
        let out = render(g);
        log::info!("callva dump {} ({} lines follow)\n{out}", g.dumps, out.lines().count());
    }

    /// The cumulative tables (advances the dump counter and interval baseline).
    fn render(g: &mut Global) -> String {
        let now = Instant::now();
        g.dumps += 1;
        let total: u64 = g.map.values().map(|v| v.0).sum();
        let elapsed = now.duration_since(g.start).as_secs_f64();
        let interval = now.duration_since(g.last_dump).as_secs_f64();
        let irate = (total - g.last_total) as f64 / interval.max(1e-3);
        g.last_dump = now;
        g.last_total = total;

        let mut paths = [0u64; 5];
        let mut xargs = [[0u64; 4]; 5];
        let mut entries = [0u64; 2];
        let mut targets: HashMap<u64, Agg> = HashMap::new();
        let mut callers: HashMap<String, Agg> = HashMap::new();
        let mut edges: HashMap<(String, u64), Agg> = HashMap::new();
        let entries_v: Vec<(Key, (u64, usize))> = g.map.iter().map(|(k, v)| (*k, *v)).collect();
        for (k, (n, h)) in entries_v {
            let path = (k.meta & 7) as usize;
            let xa = ((k.meta >> 3) & 3) as usize;
            let entry = ((k.meta >> 5) & 1) as usize;
            paths[path] += n;
            xargs[path][xa] += n;
            entries[entry] += n;
            let mut caller = None;
            for r in k.rets {
                match sym_name(&mut g.syms, r) {
                    Some(s) if is_bridge(&s) => continue,
                    Some(s) => {
                        caller = Some(s);
                        break;
                    }
                    None if r != 0 => {
                        caller = Some(format!("{r:#x}"));
                        break;
                    }
                    None => break,
                }
            }
            let caller = caller.unwrap_or_else(|| "<unknown>".into());
            let t = targets.entry(k.va).or_default();
            t.n += n;
            t.paths[path] += n;
            t.host_fn = h;
            *t.by.entry(caller.clone()).or_insert(0) += n;
            let cl = callers.entry(caller.clone()).or_default();
            cl.n += n;
            cl.paths[path] += n;
            *cl.by.entry(format!("{:#x}", k.va)).or_insert(0) += n;
            let e = edges.entry((caller, k.va)).or_default();
            e.n += n;
            e.paths[path] += n;
            e.host_fn = h;
        }

        let mut out = String::new();
        let _ = writeln!(
            out,
            "callva\tsummary\tdump {}\t{elapsed:.0}s\ttotal {total}\t{:.0}/s avg\t{irate:.0}/s last {interval:.0}s\ttargets {}\tcallers {}\tedges {}",
            g.dumps,
            total as f64 / elapsed.max(1e-3),
            targets.len(),
            callers.len(),
            edges.len()
        );
        let mut line = String::from("callva\tpaths");
        for (i, p) in PATHS.iter().enumerate() {
            let _ = write!(line, "\t{p} {} ({:.1}%)", paths[i], pct(paths[i], total));
        }
        let _ = writeln!(out, "{line}");
        let mut line = String::from("callva\txargs");
        for (xi, x) in XARGS.iter().enumerate() {
            let n: u64 = xargs.iter().map(|a| a[xi]).sum();
            let _ = write!(line, "\t{x} {n} ({:.1}%)", pct(n, total));
        }
        let _ = writeln!(out, "{line}");
        for (pi, p) in PATHS.iter().enumerate() {
            if paths[pi] == 0 {
                continue;
            }
            let mut line = format!("callva\txargs.{p}");
            for (xi, x) in XARGS.iter().enumerate() {
                let _ = write!(line, "\t{x} {:.1}%", pct(xargs[pi][xi], paths[pi]));
            }
            let _ = writeln!(out, "{line}");
        }
        let _ = writeln!(
            out,
            "callva\tentry\tcall_va8 {} ({:.1}%)\tcall_va8_xmm {} ({:.1}%)",
            entries[0],
            pct(entries[0], total),
            entries[1],
            pct(entries[1], total)
        );
        let mut th: Vec<_> = g.threads.iter().map(|(k, v)| (k.clone(), *v)).collect();
        th.sort_by(|a, b| b.1.cmp(&a.1));
        for (name, n) in th.iter().take(24) {
            let _ = writeln!(out, "callva\tthread\t{n}\t{:.1}%\t{name}", pct(*n, total));
        }

        let _ = writeln!(out, "callva\t#target\trank\tcalls\t%\tva\tpath\thost_fn\tcallers\ttop_caller\ttop_caller%");
        let mut tv: Vec<_> = targets.into_iter().collect();
        tv.sort_by(|a, b| b.1.n.cmp(&a.1.n));
        for (rank, (va, a)) in tv.iter().take(TOP).enumerate() {
            let hs = if a.host_fn == 0 {
                "<x86>".to_string()
            } else {
                sym_name(&mut g.syms, a.host_fn as u64).unwrap_or_else(|| format!("{:#x}", a.host_fn))
            };
            let (tc, tn) = top_of(&a.by).unwrap_or_default();
            let _ = writeln!(
                out,
                "callva\ttarget\t{}\t{}\t{:.2}\t{va:#x}\t{}\t{hs}\t{}\t{tc}\t{:.0}",
                rank + 1,
                a.n,
                pct(a.n, total),
                path_mix(&a.paths),
                a.by.len(),
                pct(tn, a.n)
            );
        }

        let _ = writeln!(out, "callva\t#caller\trank\tcalls\t%\tcaller\tpath\ttargets\ttop_target\ttop_target%");
        let mut cv: Vec<_> = callers.into_iter().collect();
        cv.sort_by(|a, b| b.1.n.cmp(&a.1.n));
        for (rank, (name, a)) in cv.iter().take(TOP).enumerate() {
            let (tt, tn) = top_of(&a.by).unwrap_or_default();
            let _ = writeln!(
                out,
                "callva\tcaller\t{}\t{}\t{:.2}\t{name}\t{}\t{}\t{tt}\t{:.0}",
                rank + 1,
                a.n,
                pct(a.n, total),
                path_mix(&a.paths),
                a.by.len(),
                pct(tn, a.n)
            );
        }

        let _ = writeln!(out, "callva\t#edge\trank\tcalls\t%\tcaller\tva\tpath\thost_fn");
        let mut ev: Vec<_> = edges.into_iter().collect();
        ev.sort_by(|a, b| b.1.n.cmp(&a.1.n));
        for (rank, ((caller, va), a)) in ev.iter().take(TOP).enumerate() {
            let hs = if a.host_fn == 0 {
                "<x86>".to_string()
            } else {
                sym_name(&mut g.syms, a.host_fn as u64).unwrap_or_else(|| format!("{:#x}", a.host_fn))
            };
            let _ = writeln!(
                out,
                "callva\tedge\t{}\t{}\t{:.2}\t{caller}\t{va:#x}\t{}\t{hs}",
                rank + 1,
                a.n,
                pct(a.n, total),
                path_mix(&a.paths)
            );
        }
        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::callctx::ctx;

        #[cfg(target_arch = "aarch64")]
        #[inline(never)]
        fn fake_bridge(va: u64, d: (usize, u32), entry: u8) {
            let fp: usize;
            unsafe { core::arch::asm!("mov {}, x29", out(reg) fp, options(nomem, nostack, preserves_flags)) };
            let rets = unsafe { walk(fp) };
            record(ctx(), rets, va, d, entry);
        }

        #[cfg(target_arch = "aarch64")]
        #[no_mangle]
        #[inline(never)]
        extern "C" fn callprof_test_histogram_driver() {
            let c = ctx();
            c.outgoing_rsp.set((0, 0));
            c.incoming_rsp.set((0, 0));
            for _ in 0..5000 {
                fake_bridge(0x1_4123_4560, (0x1000, LEAN_FLAGS), super::super::ENTRY_VA8);
            }
            let buf = [0u64; 32];
            let rsp = buf.as_ptr() as u64;
            c.outgoing_rsp.set((rsp, rsp + 0xc0));
            for _ in 0..3000 {
                fake_bridge(0x1_4200_0000 | (1u64 << 46), (0, 0), super::super::ENTRY_XMM);
            }
            c.outgoing_rsp.set((0, 0));
        }

        #[test]
        #[cfg(target_arch = "aarch64")]
        fn histogram_counts_paths_targets_and_callers() {
            callprof_test_histogram_driver();
            L.with(|l| flush(l.borrow_mut().as_mut().unwrap()));
            let out = render(&mut global().lock().unwrap());
            let line = |tag: &str| out.lines().find(|l| l.starts_with(tag)).unwrap_or_else(|| panic!("no {tag} in\n{out}")).to_string();
            assert!(line("callva\tsummary").contains("total 8000"), "{out}");
            assert!(line("callva\tpaths").contains("lean 5000"), "{out}");
            assert!(line("callva\tpaths").contains("orig 3000"), "{out}");
            assert!(line("callva\txargs\t").contains("zero 5000") && line("callva\txargs\t").contains("full 3000"), "{out}");
            let t1 = line("callva\ttarget\t1\t");
            assert!(t1.contains("\t5000\t") && t1.contains("0x141234560") && t1.contains("\tlean\t"), "{t1}");
            // `fake_bridge` stands in for `hit`: the first recorded frame is its caller.
            let c1 = line("callva\tcaller\t1\t");
            assert!(c1.contains("callprof_test_histogram_driver"), "{c1}");
            assert!(out.contains("0x142000000\torig\t<x86>"), "{out}");
            assert!(is_bridge("emu::call_va8") && is_bridge("simd_ops::call_stk_xmm") && !is_bridge("t09::fn_1424f5230"));
        }
    }
}
