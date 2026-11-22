//! `fnsig retlive <exe> <funcs.tsv> <calls.tsv> <targets.tsv> <out.tsv>`
//!
//! Return-register liveness for the shadow oracle: does any caller of a function read its
//! `rax` (and how wide) or its `xmm0` after the call returns?
//!
//! For each direct call site of a target (`calls.tsv`: `caller callee site kind`, kind
//! `call`/`tail`) the caller's code after the call is walked (CFG inside the caller's
//! ranges, bounded):
//!   * a read of al/ah/ax/eax/rax (xmm0/ymm0/zmm0) before a full write → live (max width);
//!   * a full write (eax/rax; a full or low-lane xmm0 write) → dead on that path;
//!   * `call`: rax is clobbered (dead); xmm0 may be the next callee's first float argument,
//!     so a direct callee is checked for reading xmm0 at entry (indirect → unknown);
//!   * `ret` / a tail-site (`jmp target`) → the value is the caller's own return: resolved
//!     through the caller's call sites (depth-limited);
//!   * jump tables, leaving the function's ranges, or the step budget → unknown.
//! Output: `va sites rax xmm` with rax ∈ {0,8,16,32,64,?} and xmm ∈ {0,1,?}.
use crate::Exe;
use iced_x86::{
    Decoder, DecoderOptions, FlowControl, Instruction, InstructionInfoFactory, Mnemonic, OpAccess,
    OpKind, Register,
};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// Unknown-cause counters (diagnostics).
static WHY: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
const WHY_NAMES: [&str; 8] = ["out_of_range", "budget", "indirect_branch", "invalid", "callee_unknown", "indirect_call", "pass_limit", "no_func"];
fn why(i: usize) {
    WHY[i].fetch_add(1, Relaxed);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reg {
    Rax,
    Xmm0,
}

#[derive(Default, Clone, Copy)]
struct Acc {
    /// Max read width in bits (rax) or 1 (xmm0); 0 = no read seen.
    live: u8,
    /// Bits of the value some path returns to the caller's caller (0 = none).
    pass: u64,
    unknown: bool,
}

impl Acc {
    fn merge(&mut self, o: Acc) {
        self.live = self.live.max(o.live);
        self.pass |= o.pass;
        self.unknown |= o.unknown;
    }
}

struct Ctx<'a> {
    exe: &'a Exe,
    funcs: HashMap<u64, Vec<(u64, u64)>>,
    /// Functions without unwind info (gap code: funclet tables, leaf chunks): a value
    /// returned through them is not traced further (unknown).
    nopass: HashSet<u64>,
    /// callee → (caller, site, tail)
    sites: HashMap<u64, Vec<(u64, u64, bool)>>,
    /// IAT slots of imports whose first parameter is never a float (system DLLs): a call
    /// through one clobbers xmm0 without reading it.
    nofloat_iat: HashSet<u64>,
}

const BUDGET: usize = 3000;
const MAX_PASS_DEPTH: u32 = 6;
const MAX_ENTRY_DEPTH: u32 = 2;
/// Callers with more sites than this are not expanded for pass-through (unknown).
const MAX_FANOUT: usize = 4000;

fn in_ranges(r: &[(u64, u64)], ip: u64) -> bool {
    r.iter().any(|&(a, b)| ip >= a && ip < b)
}

/// Read width class (8/16/32/64) of a set of rax bits; 0 for none.
fn width_of(bits: u64) -> u8 {
    match 64 - bits.leading_zeros() {
        0 => 0,
        1..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        _ => 64,
    }
}

/// Bits of rax covered by `r` (0 = not an rax sub-register).
fn rax_mask(r: Register) -> u64 {
    match r {
        Register::AL => 0xff,
        Register::AH => 0xff00,
        Register::AX => 0xffff,
        Register::EAX | Register::RAX => u64::MAX,
        _ => 0,
    }
}

fn is_xmm0(r: Register) -> bool {
    matches!(r, Register::XMM0 | Register::YMM0 | Register::ZMM0)
}

/// Zeroing idioms (`xor eax,eax`, `xorps xmm0,xmm0`, ...): a write, not a read.
fn zero_idiom(ins: &Instruction) -> bool {
    use Mnemonic::*;
    let m = ins.mnemonic();
    let same = |a: u32, b: u32| {
        ins.op_count() > b
            && ins.op_kind(a) == OpKind::Register
            && ins.op_kind(b) == OpKind::Register
            && ins.op_register(a) == ins.op_register(b)
    };
    match m {
        Xor | Sub | Pxor | Xorps | Xorpd | Psubb | Psubw | Psubd | Psubq | Pcmpgtb | Pcmpgtw | Pcmpgtd | Pcmpeqb
        | Pcmpeqw | Pcmpeqd | Pcmpeqq => same(0, 1),
        Vxorps | Vxorpd | Vpxor | Vpxord | Vpxorq | Vpsubb | Vpsubw | Vpsubd | Vpsubq | Vpcmpeqb | Vpcmpeqw
        | Vpcmpeqd | Vpcmpeqq | Vpcmpgtb | Vpcmpgtw | Vpcmpgtd => same(1, 2),
        _ => false,
    }
}

/// Scalar ops that write only the low lane of their destination and do not read it
/// (legacy: dest is op0, sources op1..; VEX: op1 supplies the upper lanes).
fn low_lane_write(ins: &Instruction) -> bool {
    use Mnemonic::*;
    let m = ins.mnemonic();
    let legacy = matches!(m, Cvtsi2ss | Cvtsi2sd | Cvtss2sd | Cvtsd2ss | Sqrtss | Sqrtsd | Rcpss | Rsqrtss | Roundss | Roundsd)
        || (matches!(m, Movss | Movsd) && ins.op_count() == 2 && ins.op_kind(1) == OpKind::Register);
    if legacy {
        return !(ins.op_kind(1) == OpKind::Register && is_xmm0(ins.op_register(1)));
    }
    let vex = matches!(
        m,
        Vcvtsi2ss | Vcvtsi2sd | Vcvtss2sd | Vcvtsd2ss | Vsqrtss | Vsqrtsd | Vrcpss | Vrsqrtss | Vroundss | Vroundsd
    ) || (matches!(m, Vmovss | Vmovsd) && ins.op_count() == 3);
    if vex {
        // op1 (upper-lane source) may be xmm0: it reads only the upper lanes, which no
        // scalar float return uses. op2 being xmm0 is a real read.
        return !(ins.op_count() > 2 && ins.op_kind(2) == OpKind::Register && is_xmm0(ins.op_register(2)));
    }
    false
}

/// (read mask, written mask) of `reg` by `ins` (xmm0: all-or-nothing, `u64::MAX`).
/// A 32-bit write zero-extends, so it covers all of rax.
fn effect(fac: &mut InstructionInfoFactory, ins: &Instruction, reg: Reg) -> (u64, u64) {
    let (r, k) = effect_w(fac, ins, reg);
    (r, k)
}

fn effect_w(fac: &mut InstructionInfoFactory, ins: &Instruction, reg: Reg) -> (u64, u64) {
    match ins.mnemonic() {
        // Upper lanes only: no scalar/xmm return value lives there.
        Mnemonic::Vzeroupper => return (0, 0),
        Mnemonic::Vzeroall => return (0, if reg == Reg::Xmm0 { u64::MAX } else { 0 }),
        _ => {}
    }
    if zero_idiom(ins) {
        let r = ins.op_register(0);
        return match reg {
            Reg::Rax => (0, rax_mask(r)),
            Reg::Xmm0 => (0, if is_xmm0(r) { u64::MAX } else { 0 }),
        };
    }
    if reg == Reg::Xmm0
        && ins.op_count() > 0
        && ins.op_kind(0) == OpKind::Register
        && is_xmm0(ins.op_register(0))
        && low_lane_write(ins)
    {
        return (0, u64::MAX);
    }
    let info = fac.info(ins);
    let (mut rd, mut kill) = (0u64, 0u64);
    for u in info.used_registers() {
        let m = match reg {
            Reg::Rax => rax_mask(u.register()),
            Reg::Xmm0 => {
                if is_xmm0(u.register()) {
                    u64::MAX
                } else {
                    0
                }
            }
        };
        if m == 0 {
            continue;
        }
        match u.access() {
            OpAccess::Read | OpAccess::CondRead | OpAccess::ReadWrite | OpAccess::ReadCondWrite => rd |= m,
            OpAccess::Write => kill |= m,
            _ => {}
        }
    }
    (rd, kill)
}

impl<'a> Ctx<'a> {
    /// `call/jmp [rip+slot]` through a no-float import's IAT slot.
    fn nofloat_import(&self, ins: &Instruction) -> bool {
        ins.is_ip_rel_memory_operand() && self.nofloat_iat.contains(&ins.ip_rel_memory_address())
    }

    fn decode(&self, ip: u64) -> Option<Instruction> {
        let b = self.exe.bytes(ip, 15).or_else(|| self.exe.bytes(ip, 1))?;
        let mut d = Decoder::with_ip(64, b, ip, DecoderOptions::NONE);
        let ins = d.decode();
        if ins.is_invalid() {
            None
        } else {
            Some(ins)
        }
    }

    /// Walk from `start` inside `ranges`. `entry`: checking whether a callee reads xmm0 at
    /// its entry (a `ret` then means "not read").
    fn scan(&self, ranges: &[(u64, u64)], start: u64, reg: Reg, entry: bool, depth: u32) -> Acc {
        let mut acc = Acc::default();
        let mut fac = InstructionInfoFactory::new();
        // Path state: bits of the value already overwritten (partial writes like `setl al`).
        let mut seen: HashSet<(u64, u64)> = HashSet::new();
        let mut wl: Vec<(u64, u64)> = vec![(start, 0)];
        let mut budget = BUDGET;
        'paths: while let Some((mut ip, mut dead)) = wl.pop() {
            loop {
                if !seen.insert((ip, dead)) {
                    continue 'paths;
                }
                if !in_ranges(ranges, ip) {
                    why(0);
                    acc.unknown = true;
                    continue 'paths;
                }
                if budget == 0 {
                    why(1);
                    acc.unknown = true;
                    return acc;
                }
                budget -= 1;
                let Some(ins) = self.decode(ip) else {
                    why(3);
                    acc.unknown = true;
                    continue 'paths;
                };
                let (rmask, wmask) = effect(&mut fac, &ins, reg);
                let live_bits = rmask & !dead;
                let rd: u8 = if live_bits == 0 {
                    0
                } else if reg == Reg::Xmm0 {
                    1
                } else {
                    // Width up to the highest live bit read.
                    width_of(live_bits)
                };
                if rd > 0 {
                    if std::env::var("RETLIVE_TRACE").is_ok() {
                        eprintln!("    read {} at {ip:x} (start {start:x}, entry {entry})", rd);
                    }
                    acc.live = acc.live.max(rd);
                    if (reg == Reg::Rax && acc.live >= 64) || reg == Reg::Xmm0 {
                        return acc;
                    }
                }
                dead |= wmask;
                if dead == u64::MAX {
                    continue 'paths;
                }
                match ins.flow_control() {
                    FlowControl::Next => ip = ins.next_ip(),
                    FlowControl::UnconditionalBranch | FlowControl::ConditionalBranch => {
                        let cond = ins.flow_control() == FlowControl::ConditionalBranch;
                        let t = ins.near_branch_target();
                        if in_ranges(ranges, t) {
                            wl.push((t, dead));
                        } else if t != 0 {
                            // Tail call: the target returns its own rax; it may take xmm0.
                            if reg == Reg::Xmm0 {
                                acc.merge(self.callee_reads_xmm0(t, depth + 1));
                            }
                        } else {
                            acc.unknown = true;
                        }
                        if cond {
                            ip = ins.next_ip();
                        } else {
                            continue 'paths;
                        }
                    }
                    // `jmp [rip+x]` / `jmp [reg+disp]`: a tail call through the IAT or a vtable.
                    FlowControl::IndirectBranch
                        if ins.op0_kind() == OpKind::Memory && ins.memory_index() == Register::None =>
                    {
                        if reg == Reg::Xmm0 && !self.nofloat_import(&ins) {
                            why(5);
                            acc.unknown = true;
                        }
                        continue 'paths;
                    }
                    FlowControl::IndirectBranch | FlowControl::XbeginXabortXend => {
                        why(2);
                        acc.unknown = true;
                        continue 'paths;
                    }
                    FlowControl::Call => {
                        if reg == Reg::Xmm0 {
                            acc.merge(self.callee_reads_xmm0(ins.near_branch_target(), depth + 1));
                        }
                        continue 'paths;
                    }
                    FlowControl::IndirectCall => {
                        if reg == Reg::Xmm0 && !self.nofloat_import(&ins) {
                            why(5);
                            acc.unknown = true;
                        }
                        continue 'paths;
                    }
                    FlowControl::Return => {
                        if !entry {
                            acc.pass |= !dead;
                        }
                        continue 'paths;
                    }
                    FlowControl::Interrupt | FlowControl::Exception => continue 'paths,
                }
            }
        }
        acc
    }

    /// Whether `f` may read xmm0 at entry (as its first float argument).
    fn callee_reads_xmm0(&self, f: u64, depth: u32) -> Acc {
        let mut a = Acc::default();
        let Some(r) = self.funcs.get(&f) else {
            // Import thunks / unknown code: assume it may.
            why(4);
            a.unknown = true;
            return a;
        };
        if depth > MAX_ENTRY_DEPTH {
            a.unknown = true;
            return a;
        }
        let s = self.scan(r, f, Reg::Xmm0, true, depth);
        a.live = s.live;
        a.unknown = s.unknown;
        a
    }

    /// Liveness of `f`'s return register over all its known call sites.
    fn fn_liveness(&self, f: u64, reg: Reg, depth: u32, stack: &mut Vec<u64>) -> (usize, Acc) {
        let mut acc = Acc::default();
        let Some(sites) = self.sites.get(&f) else {
            acc.unknown = true;
            return (0, acc);
        };
        if depth > 0 && sites.len() > MAX_FANOUT {
            acc.unknown = true;
            return (sites.len(), acc);
        }
        stack.push(f);
        let mut pass_callers: Vec<(u64, u64)> = Vec::new();
        for &(caller, site, tail) in sites {
            if tail {
                pass_callers.push((caller, u64::MAX));
                continue;
            }
            let Some(r) = self.funcs.get(&caller) else {
                why(7);
                acc.unknown = true;
                continue;
            };
            let Some(ins) = self.decode(site) else {
                acc.unknown = true;
                continue;
            };
            let s = self.scan(r, ins.next_ip(), reg, false, 0);
            if depth == 0 && std::env::var("RETLIVE_DEBUG").ok().and_then(|v| hex(&v)) == Some(f) {
                eprintln!("{:?} site {site:x} caller {caller:x}: live {} pass {} unknown {}", reg == Reg::Rax, s.live, s.pass, s.unknown);
            }
            acc.live = acc.live.max(s.live);
            acc.unknown |= s.unknown;
            if s.pass != 0 {
                pass_callers.push((caller, s.pass));
            }
            if acc.unknown && acc.live >= if reg == Reg::Rax { 64 } else { 1 } {
                break;
            }
        }
        pass_callers.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::new();
        for (c, m) in pass_callers {
            match merged.last_mut() {
                Some(l) if l.0 == c => l.1 |= m,
                _ => merged.push((c, m)),
            }
        }
        for (c, pmask) in merged {
            if depth + 1 > MAX_PASS_DEPTH || stack.contains(&c) || self.nopass.contains(&c) {
                why(6);
                acc.unknown = true;
                continue;
            }
            let (_, s) = self.fn_liveness(c, reg, depth + 1, stack);
            // Bits the caller's callers read, restricted to the bits still ours at its ret.
            let read = match s.live {
                0 => 0,
                8 => 0xff,
                16 => 0xffff,
                32 => 0xffff_ffff,
                _ => u64::MAX,
            } & pmask;
            let live = if reg == Reg::Xmm0 { (read != 0) as u8 } else { width_of(read) };
            if std::env::var("RETLIVE_TRACE").is_ok() && reg == Reg::Rax && live > 0 {
                eprintln!("  pass {f:x} -> caller {c:x} (depth {depth}): live {live} unknown {}", s.unknown);
            }
            acc.live = acc.live.max(live);
            acc.unknown |= s.unknown;
        }
        stack.pop();
        (sites.len(), acc)
    }
}

fn hex(s: &str) -> Option<u64> {
    u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok()
}

pub fn cmd(args: &[String]) {
    let exe = Exe::load(&args[0]);
    let mut funcs: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
    let mut nopass: HashSet<u64> = HashSet::new();
    for line in std::fs::read_to_string(&args[1]).expect("read funcs").lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 2 || c[0].starts_with('#') || c[0] == "va" {
            continue;
        }
        let (Some(va), Some(end)) = (hex(c[0]), hex(c[1])) else { continue };
        let mut r = vec![(va, end)];
        for fr in &c[2..] {
            if *fr == "np" {
                nopass.insert(va);
            } else if let Some((a, b)) = fr.split_once('-') {
                if let (Some(a), Some(b)) = (hex(a), hex(b)) {
                    r.push((a, b));
                }
            }
        }
        funcs.insert(va, r);
    }
    let mut sites: HashMap<u64, Vec<(u64, u64, bool)>> = HashMap::new();
    for line in std::fs::read_to_string(&args[2]).expect("read calls").lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 4 || c[0] == "caller" {
            continue;
        }
        let (Some(caller), Some(callee), Some(site)) = (hex(c[0]), hex(c[1]), hex(c[2])) else { continue };
        sites.entry(callee).or_default().push((caller, site, c[3] == "tail"));
    }
    let targets: Vec<u64> = std::fs::read_to_string(&args[3])
        .expect("read targets")
        .lines()
        .filter_map(|l| hex(l.split('\t').next().unwrap_or("")))
        .collect();
    // Optional 6th file: IAT slot VAs (hex, one per line) of imports that take no float first
    // argument.
    let nofloat_iat: HashSet<u64> = args
        .get(5)
        .map(|p| std::fs::read_to_string(p).expect("read iat").lines().filter_map(hex).collect())
        .unwrap_or_default();
    let ctx = Ctx { exe: &exe, funcs, nopass, sites, nofloat_iat };
    let rows: Vec<(u64, usize, String, String)> = targets
        .par_iter()
        .map(|&va| {
            let (n, r) = ctx.fn_liveness(va, Reg::Rax, 0, &mut Vec::new());
            let (_, x) = ctx.fn_liveness(va, Reg::Xmm0, 0, &mut Vec::new());
            // A definite read wins over unknown paths (it is live either way).
            let rax = if r.live > 0 {
                r.live.to_string()
            } else if r.unknown || n == 0 {
                "?".into()
            } else {
                "0".into()
            };
            let xmm = if x.live > 0 {
                "1".into()
            } else if x.unknown || n == 0 {
                "?".into()
            } else {
                "0".into()
            };
            (va, n, rax, xmm)
        })
        .collect();
    let mut f = std::io::BufWriter::new(std::fs::File::create(&args[4]).expect("create out"));
    writeln!(f, "va\tsites\trax\txmm").unwrap();
    for (va, n, rax, xmm) in &rows {
        writeln!(f, "{va:x}\t{n}\t{rax}\t{xmm}").unwrap();
    }
    for (i, n) in WHY_NAMES.iter().enumerate() {
        eprintln!("  unknown cause {n}: {}", WHY[i].load(Relaxed));
    }
    let cnt = |p: &dyn Fn(&(u64, usize, String, String)) -> bool| rows.iter().filter(|r| p(r)).count();
    eprintln!(
        "fnsig retlive: {} targets: rax dead {} live {} unknown {}; xmm dead {} live {} unknown {}",
        rows.len(),
        cnt(&|r| r.2 == "0"),
        cnt(&|r| r.2 != "0" && r.2 != "?"),
        cnt(&|r| r.2 == "?"),
        cnt(&|r| r.3 == "0"),
        cnt(&|r| r.3 == "1"),
        cnt(&|r| r.3 == "?"),
    );
}
