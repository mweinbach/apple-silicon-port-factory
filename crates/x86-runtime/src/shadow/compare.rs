//! Comparison of the probe runs (orchestrator, after all runs exited).

use super::handler::page;
use super::memmap::{is_malloc_tag, region_of, Region};
use super::run::{Bufs, RunRes, NRUNS, ST_FAULT, ST_FORK, ST_HOST, ST_OK, ST_OVERFLOW, ST_SHARED, ST_TIMEOUT};
use super::types::{Outcome, Reason, Report, RetUse};
use std::collections::{BTreeSet, HashMap};

/// One differing byte range `[lo, hi)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffRange {
    pub lo: usize,
    pub hi: usize,
}

/// Comparison inputs.
pub struct Cmp<'a> {
    pub bufs: &'a Bufs,
    /// G1, N, G2, G3 (poisoned guest), N3 (poisoned native).
    pub res: [RunRes; NRUNS],
    /// G3 and N3 ran.
    pub poisoned: bool,
    pub ret_xmm: bool,
    /// Return-register bits callers use.
    pub ret: RetUse,
    /// Byte ranges the comparison ignores (host TLS, CallCtx, caller-supplied).
    pub soft: Vec<(usize, usize)>,
    /// Scratch stacks (dead frames): address pairs into them are `stack_layout`.
    pub scratch: Vec<(usize, usize)>,
    /// VM map at protect time (region tags for the report).
    pub regions: &'a [Region],
}

/// Comparison result details (for the dump).
#[derive(Default)]
pub struct Detail {
    /// G1 vs N outside the G1/G2 mask.
    pub diffs: Vec<DiffRange>,
    /// G1 vs G2.
    pub noise: Vec<DiffRange>,
    /// G1 vs G3 and N vs N3 (bytes that depend on uninitialised stack memory).
    pub uninit: [Vec<DiffRange>; 2],
    /// Diff bytes the poisoned runs do not explain.
    pub uncovered: Vec<DiffRange>,
    /// Page → (pristine, [G1, N, G2, G3, N3]) content pointers, for the dump.
    pub views: HashMap<u64, (usize, [usize; NRUNS])>,
    /// Ignored byte ranges within 1 MiB of the first diff (dump diagnostics).
    pub soft_near: Vec<(usize, usize)>,
    /// Scratch stacks and poisoning windows of this probe (dump diagnostics).
    pub scratch: Vec<(usize, usize)>,
    pub poison: Vec<(usize, usize)>,
}

fn push_byte(out: &mut Vec<DiffRange>, addr: usize) {
    match out.last_mut() {
        Some(r) if r.hi == addr => r.hi = addr + 1,
        _ => out.push(DiffRange { lo: addr, hi: addr + 1 }),
    }
}

fn in_ranges(list: &[(usize, usize)], addr: usize) -> bool {
    list.iter().any(|&(lo, hi)| addr >= lo && addr < hi)
}

/// Some byte of `r` is in one of the sorted, disjoint `list` ranges.
fn overlaps(list: &[DiffRange], r: &DiffRange) -> bool {
    let i = list.partition_point(|x| x.hi <= r.lo);
    list.get(i).is_some_and(|x| x.lo < r.hi)
}

/// `rax` differs only by the emulator's 8/16-bit result narrowing.
fn abi_narrow(g: u64, n: u64) -> bool {
    g != n && ((g <= 0xff && g == n & 0xff) || (g <= 0xffff && g == n & 0xffff))
}

/// Run statuses → skip reason or one-sided outcome, before any memory comparison.
fn status_outcome(res: &[RunRes; NRUNS], rep: &mut Report) -> Option<(Outcome, Reason)> {
    let [g1, n, g2] = [res[0].status, res[1].status, res[2].status];
    if [g1, n, g2].contains(&ST_FORK) {
        return Some((Outcome::Skipped, Reason::ForkFailed));
    }
    if [g1, n, g2].contains(&ST_SHARED) {
        return Some((Outcome::Skipped, Reason::SharedWrite));
    }
    if let Some(r) = res[..3].iter().find(|r| r.status == ST_HOST) {
        rep.host_api = r.host_api;
        return Some((Outcome::Skipped, Reason::HostUnsafe));
    }
    if [g1, n, g2].contains(&ST_OVERFLOW) {
        return Some((Outcome::Skipped, Reason::TooManyPages));
    }
    if g1 == ST_TIMEOUT || g2 == ST_TIMEOUT {
        let r = if n == ST_TIMEOUT { Reason::DoubleTimeout } else { Reason::GuestTimeout };
        return Some((Outcome::Skipped, r));
    }
    if g1 == ST_FAULT || g2 == ST_FAULT {
        return Some((Outcome::Skipped, Reason::GuestFault));
    }
    if n == ST_TIMEOUT {
        return Some((Outcome::Mismatch, Reason::NativeTimeout));
    }
    if n == ST_FAULT {
        let a = res[1].fault_addr;
        if a != 0 && !(0x10000..(1u64 << 47)).contains(&a) {
            return Some((Outcome::Skipped, Reason::NativeWildFault));
        }
        rep.first_diff = a;
        return Some((Outcome::Mismatch, Reason::NativeFault));
    }
    debug_assert!(g1 == ST_OK && n == ST_OK && g2 == ST_OK);
    None
}

pub unsafe fn compare(c: &Cmp, rep: &mut Report) -> Detail {
    let mut d = Detail::default();
    for r in 0..3 {
        rep.rax[r] = c.res[r].rax;
        rep.xmm0[r] = c.res[r].xmm0;
        rep.t_us[r] = c.res[r].t_us;
        rep.fault_addr[r] = c.res[r].fault_addr;
        rep.fault_pc[r] = c.res[r].fault_pc;
        rep.run_status[r] = c.res[r].wait_status;
        rep.run_exit_kind[r] = c.res[r].exit_kind;
        rep.run_exit_code[r] = c.res[r].exit_code;
        rep.fault_sig[r] = c.res[r].fault_sig;
        rep.pages[r] = c.bufs.hdr(r).npages as u32;
    }
    rep.rax_p = [c.res[3].rax, c.res[4].rax];
    if let Some((o, why)) = status_outcome(&c.res, rep) {
        rep.outcome = o as u32;
        rep.reason = why as u32;
        return d;
    }
    // Poison evidence per side, only from runs that completed.
    let pg_ok = [c.poisoned && c.res[3].status == ST_OK, c.poisoned && c.res[4].status == ST_OK];
    let pg = page();
    let lists: [Vec<(u64, *const u8)>; NRUNS] = core::array::from_fn(|r| c.bufs.pages(r));
    let mut pristine: HashMap<u64, *const u8> = HashMap::new();
    {
        let addrs = (c.bufs.pristine + NRUNS * c.bufs.maxpages * pg) as *const u64;
        for k in 0..NRUNS * c.bufs.maxpages {
            let a = *addrs.add(k);
            if a == 0 {
                break;
            }
            pristine.entry(a).or_insert((c.bufs.pristine + k * pg) as *const u8);
        }
    }
    let union: BTreeSet<u64> = lists.iter().flat_map(|l| l.iter().map(|p| p.0)).collect();
    let view = |r: usize, a: u64| -> *const u8 {
        match lists[r].binary_search_by_key(&a, |p| p.0) {
            Ok(i) => lists[r][i].1,
            Err(_) => pristine.get(&a).copied().unwrap_or(a as *const u8),
        }
    };
    let mut native_g1 = true;
    let mut native_g2 = true;
    for &a in &union {
        let p = pristine.get(&a).copied().unwrap_or(a as *const u8);
        let v: [*const u8; NRUNS] = core::array::from_fn(|r| view(r, a));
        let s: [&[u8]; NRUNS] = core::array::from_fn(|r| core::slice::from_raw_parts(v[r], pg));
        if s.iter().all(|x| *x == s[0]) {
            continue;
        }
        d.views.insert(a, (p as usize, v.map(|x| x as usize)));
        for i in 0..pg {
            let addr = a as usize + i;
            let b: [u8; NRUNS] = core::array::from_fn(|r| s[r][i]);
            if b.iter().all(|&x| x == b[0]) {
                continue;
            }
            if in_ranges(&c.soft, addr) {
                continue;
            }
            let ug = pg_ok[0] && b[0] != b[3];
            let un = pg_ok[1] && b[1] != b[4];
            if ug {
                push_byte(&mut d.uninit[0], addr);
            }
            if un {
                push_byte(&mut d.uninit[1], addr);
            }
            if b[0] != b[2] {
                push_byte(&mut d.noise, addr);
            } else if b[1] != b[0] {
                push_byte(&mut d.diffs, addr);
                if !(ug || un) {
                    push_byte(&mut d.uncovered, addr);
                }
            }
            if b[0] == b[2] {
                continue;
            }
            native_g1 &= b[1] == b[0];
            native_g2 &= b[1] == b[2];
        }
    }
    rep.uninit_ranges = [d.uninit[0].len() as u32, d.uninit[1].len() as u32];
    // Only the return-register bits some caller reads count (`rets=`); differences in the
    // others are reported as `ret_dead`, never as equal.
    let m = c.ret.rax_mask;
    let xl = c.ret_xmm && c.ret.xmm;
    rep.ret_mask = m;
    rep.ret_xmm = xl as u32;
    rep.ret_src = c.ret.src;
    let [g1, n1, g2, g3, n3] = c.res;
    let ret_noise = (g1.rax ^ g2.rax) & m != 0 || (xl && g1.xmm0 != g2.xmm0);
    let rax_diff = (g1.rax ^ n1.rax) & m != 0;
    let xmm_diff = xl && g1.xmm0 != n1.xmm0;
    let dead_diff = g1.rax != n1.rax || (c.ret_xmm && g1.xmm0 != n1.xmm0);
    // rax / xmm0 that move under poisoning: computed from uninitialised stack.
    let rax_u = (pg_ok[0] && (g1.rax ^ g3.rax) & m != 0) || (pg_ok[1] && (n1.rax ^ n3.rax) & m != 0);
    let xmm_u = xl && ((pg_ok[0] && g1.xmm0 != g3.xmm0) || (pg_ok[1] && n1.xmm0 != n3.xmm0));
    // Tags: heap regions, probe-arena pointer pairs and scratch-stack address pairs.
    let arena = c.bufs.heap..c.bufs.heap + super::run::HEAP_LEN;
    let in_scratch = |x: u64| in_ranges(&c.scratch, x as usize);
    let rax_layout = rax_diff && arena.contains(&(g1.rax as usize)) && arena.contains(&(n1.rax as usize));
    let rax_stack = rax_diff && in_scratch(g1.rax) && in_scratch(n1.rax);
    let mut arena_only = 0usize;
    let mut layout_or_covered = 0usize;
    let mut covered = 0usize;
    for r in &d.diffs {
        rep.diff_bytes += (r.hi - r.lo) as u64;
        if region_of(c.regions, r.lo).is_some_and(|g| is_malloc_tag(g.tag)) {
            rep.heap_ranges += 1;
        }
        let q = r.lo & !7;
        let (mut alloc, mut stack) = (false, false);
        if r.hi <= q + 8 {
            let pa = q as u64 & !(pg as u64 - 1);
            if let Some(&(_, v)) = d.views.get(&pa) {
                let off = q - pa as usize;
                let gq = (v[0] as *const u8).add(off).cast::<u64>().read_unaligned();
                let nq = (v[1] as *const u8).add(off).cast::<u64>().read_unaligned();
                alloc = arena.contains(&(gq as usize)) && arena.contains(&(nq as usize));
                stack = !alloc && in_scratch(gq) && in_scratch(nq);
            }
        }
        let cov = !overlaps(&d.uncovered, r);
        if alloc {
            rep.alloc_layout_ranges += 1;
        } else if stack {
            rep.stack_layout_ranges += 1;
        } else if arena.contains(&r.lo) {
            arena_only += 1;
        }
        if cov && !alloc && !stack {
            covered += 1;
        }
        if alloc || stack || cov {
            layout_or_covered += 1;
        }
    }
    rep.diff_ranges = d.diffs.len() as u32;
    rep.first_diff = d.diffs.first().map_or(0, |r| r.lo as u64);
    if let Some(f) = d.diffs.first().or(d.noise.first()) {
        let near = 1usize << 20;
        d.soft_near =
            c.soft.iter().filter(|&&(lo, hi)| hi + near > f.lo && lo < f.lo + near).copied().take(32).collect();
    }
    let layouts = rep.alloc_layout_ranges as usize + rep.stack_layout_ranges as usize;
    let layout_diverged = layouts > 0 || rax_layout || rax_stack;
    let (o, why) = if !d.noise.is_empty() || ret_noise {
        rep.native_consistent = (native_g1 && !rax_diff || native_g2 && n1.rax == g2.rax) as u32;
        (Outcome::Nondet, Reason::None)
    } else if d.diffs.is_empty() && !xmm_diff && !rax_diff {
        (if dead_diff { Outcome::RetDead } else { Outcome::Equal }, Reason::None)
    } else if d.diffs.is_empty() && !xmm_diff && abi_narrow(g1.rax, n1.rax) {
        (Outcome::AbiNarrow, Reason::None)
    } else if !xmm_diff
        && layout_diverged
        && layouts + arena_only == d.diffs.len()
        && (!rax_diff || rax_layout || rax_stack)
    {
        if rep.stack_layout_ranges > 0 || rax_stack {
            (Outcome::StackLayout, Reason::None)
        } else {
            (Outcome::AllocLayout, Reason::None)
        }
    } else if pg_ok[0]
        && pg_ok[1]
        && layout_or_covered == d.diffs.len()
        && (!rax_diff || rax_layout || rax_stack || rax_u)
        && (!xmm_diff || xmm_u)
        && (covered > 0 || rax_u || xmm_u)
    {
        (Outcome::Uninit, Reason::None)
    } else {
        (Outcome::Mismatch, Reason::None)
    };
    rep.outcome = o as u32;
    rep.reason = why as u32;
    d
}
