//! fnsig — normalized x86-64 function features for fndb library identification.
//!
//!   fnsig exe  <AlanWake2.exe> <funcs.tsv> <out.tsv>
//!       funcs.tsv: one function per line, hex VAs: `va<TAB>end[<TAB>fragstart-fragend ...]`
//!   fnsig icalls <AlanWake2.exe> <funcs.tsv> <out.tsv>
//!       indirect calls/tail jumps per function (for `fndb can-throw`):
//!       `va<TAB>v<disp>,...`: `call/jmp [reg+disp]` (vcall slot = disp/8),
//!       `g<va>` for `[rip+x]` (IAT slot / global fn pointer), `r` for `call reg`.
//!   fnsig coff <lib-label> <out.tsv> <file.obj|file.lib>...
//!       every function (symbol-delimited range of an executable section) in COFF objects
//!       (or archive members) built with clang-cl.
//!
//! Output (TSV, one function per line):
//!   va/id  name  n_insn  n_bytes  h_exact  h_loose  h_mnem  cfg  consts  callees  strings  minhash
//!
//! Normalization (both sides identical so hashes are comparable):
//!   * call targets            -> `C`  (callee recorded separately)
//!   * jumps inside function   -> byte offset from function start; outside -> `X` (tail jump)
//!   * rip-relative operands   -> masked (`R`); xmm constant-pool loads add an `F:` constant
//!   * immediates/displacements covered by a relocation (obj) or that look like image addresses /
//!     RVAs (exe)              -> masked
//!   * h_exact keeps registers and small immediates/displacements; h_loose keeps only operand
//!     classes (register size class, memory size, imm<0x100); h_mnem is mnemonics only.
//!   * minhash: 32 x u32 over 3-gram shingles of the loose token stream.
use iced_x86::{
    Code, ConstantOffsets, Decoder, DecoderOptions, FlowControl, Instruction, MemorySize, Mnemonic,
    OpKind, Register,
};
use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget, SectionKind, SymbolKind};
use rayon::prelude::*;
mod retlive;
use std::collections::{BTreeSet, HashMap};
use std::io::Write;

const NMH: usize = 32;

// ------------------------------------------------------------------------------------------------
// hashing helpers
// ------------------------------------------------------------------------------------------------
#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

#[derive(Clone, Copy)]
struct H(u64);
impl H {
    fn new() -> Self {
        H(0xcbf29ce484222325)
    }
    #[inline]
    fn add(&mut self, v: u64) {
        self.0 = mix64(self.0 ^ v.wrapping_add(0x9e3779b97f4a7c15));
    }
}

// ------------------------------------------------------------------------------------------------
// provider: what each side knows about relocations / data
// ------------------------------------------------------------------------------------------------
enum RipTarget {
    Const(String),
    Str(String),
    Code(u64),
    Sym(String),
    Other,
}

trait Provider: Sync {
    /// relocation covering [addr, addr+len) (obj only): target symbol name
    fn reloc(&self, addr: u64, len: usize) -> Option<&str>;
    /// rip-relative memory operand target
    fn rip_target(&self, insn: &Instruction, disp_addr: u64, xmm: bool) -> RipTarget;
    /// value looks like an absolute image address / RVA (exe only)
    fn looks_like_addr(&self, v: u64) -> bool;
}

// ------------------------------------------------------------------------------------------------
// feature extraction
// ------------------------------------------------------------------------------------------------
struct Feat {
    id: String,
    name: String,
    n_insn: usize,
    n_bytes: usize,
    h_exact: u64,
    h_loose: u64,
    h_mnem: u64,
    cfg: [u32; 7],
    consts: BTreeSet<String>,
    callees: Vec<String>,
    strings: Vec<String>,
    minhash: [u32; NMH],
    /// cross-compiler-tolerant value bag: non-stack field accesses `m<size>@<disp>` and immediates
    /// of compare/test/logic/mov instructions `i:<value>` (bt n -> 1<<n)
    vals: BTreeSet<String>,
    /// mnemonic-class sequence (one char per instruction, nops dropped), functions <= 160 insns
    mseq: String,
}

const MSEQ_MAX: usize = 160;

fn mnem_class(ins: &Instruction) -> Option<char> {
    let fc = ins.flow_control();
    match fc {
        FlowControl::ConditionalBranch => return Some('j'),
        FlowControl::UnconditionalBranch => return Some('J'),
        FlowControl::Call | FlowControl::IndirectCall => return Some('c'),
        FlowControl::Return => return Some('r'),
        FlowControl::IndirectBranch => return Some('I'),
        _ => {}
    }
    let n = format!("{:?}", ins.mnemonic()).to_ascii_lowercase();
    let n = n.as_str();
    let c = match n {
        "nop" | "int3" => return None,
        "mov" | "movzx" | "movsx" | "movsxd" => 'm',
        "lea" => 'l',
        "cmp" | "test" | "bt" => 't',
        "add" | "sub" | "inc" | "dec" | "neg" | "adc" | "sbb" => 'a',
        "and" | "or" | "xor" | "not" | "andn" => 'b',
        "shl" | "shr" | "sar" | "rol" | "ror" | "shld" | "shrd" | "shlx" | "shrx" | "sarx" => 's',
        "imul" | "mul" => 'x',
        "div" | "idiv" => 'd',
        "push" | "pop" => 'p',
        "xchg" | "cmpxchg" | "xadd" => 'X',
        "cdq" | "cqo" | "cdqe" | "cwde" => 'e',
        _ if n.starts_with("set") => 'S',
        _ if n.starts_with("cmov") => 'C',
        _ if n.starts_with("cvt") || n.starts_with("vcvt") => 'V',
        _ if n.contains("mov") => 'v',
        _ if n.starts_with("rep") || n.starts_with("stos") || n.starts_with("movs") => 'R',
        _ if n.starts_with('v') || n.starts_with('p') || n.ends_with("ps") || n.ends_with("pd") || n.ends_with("ss") || n.ends_with("sd") => 'f',
        _ => 'o',
    };
    Some(c)
}

fn imm_token(v: i64) -> String {
    if v < 0 {
        format!("i:-{:x}", v.unsigned_abs())
    } else {
        format!("i:{:x}", v)
    }
}

fn reg_class(r: Register) -> u64 {
    if r.is_gpr64() {
        1
    } else if r.is_gpr32() {
        2
    } else if r.is_gpr16() {
        3
    } else if r.is_gpr8() {
        4
    } else if r.is_xmm() {
        5
    } else if r.is_ymm() {
        6
    } else if r.is_zmm() {
        7
    } else {
        8 + r as u64
    }
}

fn is_vec(r: Register) -> bool {
    r.is_xmm() || r.is_ymm() || r.is_zmm()
}

/// ranges: (start_va, bytes); the first one is the primary range.
fn extract<P: Provider>(p: &P, id: String, name: String, ranges: &[(u64, &[u8])]) -> Feat {
    let lo = ranges[0].0;
    let in_func = |a: u64| ranges.iter().any(|(s, b)| a >= *s && a < *s + b.len() as u64);
    let mut insns: Vec<(Instruction, ConstantOffsets)> = Vec::new();
    let mut n_bytes = 0usize;
    for (start, bytes) in ranges {
        n_bytes += bytes.len();
        let mut dec = Decoder::with_ip(64, bytes, *start, DecoderOptions::NONE);
        let mut ins = Instruction::default();
        while dec.can_decode() {
            dec.decode_out(&mut ins);
            if ins.code() == Code::INVALID {
                break;
            }
            let co = dec.get_constant_offsets(&ins);
            insns.push((ins, co));
        }
    }
    // trim trailing int3 padding
    while let Some((i, _)) = insns.last() {
        if i.mnemonic() == Mnemonic::Int3 && insns.len() > 1 {
            insns.pop();
        } else {
            break;
        }
    }
    let mut he = H::new();
    let mut hl = H::new();
    let mut hm = H::new();
    let mut loose_tokens: Vec<u64> = Vec::with_capacity(insns.len());
    let mut consts = BTreeSet::new();
    let mut callees = Vec::new();
    let mut strings = Vec::new();
    let mut cfg = [0u32; 7]; // nbb, ncond, nback, ncall, nicall, nret, nijmp
    let mut leaders: BTreeSet<u64> = BTreeSet::new();
    leaders.insert(lo);
    for (ins, co) in &insns {
        let ip = ins.ip();
        let mn = ins.mnemonic() as u64;
        let mut te = H::new();
        let mut tl = H::new();
        te.add(ins.code() as u64);
        tl.add(mn);
        let has_vec = (0..ins.op_count()).any(|i| ins.op_kind(i) == OpKind::Register && is_vec(ins.op_register(i)));
        let fc = ins.flow_control();
        match fc {
            FlowControl::ConditionalBranch => cfg[1] += 1,
            FlowControl::Call => cfg[3] += 1,
            FlowControl::IndirectCall => cfg[4] += 1,
            FlowControl::Return => cfg[5] += 1,
            FlowControl::IndirectBranch => cfg[6] += 1,
            _ => {}
        }
        if matches!(
            fc,
            FlowControl::ConditionalBranch
                | FlowControl::UnconditionalBranch
                | FlowControl::Return
                | FlowControl::IndirectBranch
                | FlowControl::Interrupt
        ) {
            leaders.insert(ip + ins.len() as u64);
        }
        for i in 0..ins.op_count() {
            match ins.op_kind(i) {
                OpKind::Register => {
                    let r = ins.op_register(i);
                    te.add(0x100 + r as u64);
                    tl.add(0x200 + reg_class(r));
                }
                OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
                    let t = ins.near_branch_target();
                    // only rel32 forms can carry a relocation (obj): last 4 bytes of the insn
                    let rsym = if ins.len() >= 5 { p.reloc(ip + ins.len() as u64 - 4, 4) } else { None };
                    if fc == FlowControl::Call {
                        te.add(0xC0);
                        tl.add(0xC0);
                        // callee: relocation symbol (obj) or VA (exe)
                        if let Some(s) = rsym {
                            callees.push(s.to_string());
                        } else {
                            callees.push(format!("{:x}", t));
                        }
                    } else if in_func(t) && rsym.is_none() {
                        te.add(0xB0 ^ (t.wrapping_sub(lo)) << 8);
                        tl.add(0xB0);
                        if t <= ip {
                            cfg[2] += 1;
                        }
                        leaders.insert(t);
                    } else {
                        te.add(0xB1);
                        tl.add(0xB1);
                        let disp_at = ip + ins.len() as u64 - 4;
                        if let Some(s) = p.reloc(disp_at, 4) {
                            callees.push(format!("tail:{}", s));
                        } else {
                            callees.push(format!("tail:{:x}", t));
                        }
                    }
                }
                OpKind::Immediate8
                | OpKind::Immediate8_2nd
                | OpKind::Immediate16
                | OpKind::Immediate32
                | OpKind::Immediate64
                | OpKind::Immediate8to16
                | OpKind::Immediate8to32
                | OpKind::Immediate8to64
                | OpKind::Immediate32to64 => {
                    let v = ins.immediate(i);
                    let masked = (co.has_immediate()
                        && p.reloc(ip + co.immediate_offset() as u64, co.immediate_size()).is_some())
                        || p.looks_like_addr(v);
                    if masked {
                        te.add(0xA1);
                        tl.add(0xA1);
                    } else {
                        te.add(0xA0 ^ v.rotate_left(8));
                        let sv = v as i64;
                        if sv.unsigned_abs() < 0x100 {
                            tl.add(0xA2 ^ v.rotate_left(8));
                        } else {
                            tl.add(0xA3);
                        }
                        if sv.unsigned_abs() >= 0x1000 && ins.mnemonic() != Mnemonic::And || sv.unsigned_abs() >= 0x10000 {
                            consts.insert(format!("I:{:x}", v));
                        }
                    }
                }
                OpKind::Memory => {
                    let ms = ins.memory_size() as u64;
                    tl.add(0x300 + ms);
                    te.add(0x300 + ms);
                    if ins.is_ip_rel_memory_operand() {
                        te.add(0x3FF);
                        tl.add(0x3FF);
                        let disp_at = ip + co.displacement_offset() as u64;
                        match p.rip_target(ins, disp_at, has_vec) {
                            RipTarget::Const(c) => {
                                consts.insert(c);
                            }
                            RipTarget::Str(s) => strings.push(s),
                            RipTarget::Code(_) | RipTarget::Sym(_) | RipTarget::Other => {}
                        }
                    } else {
                        let b = ins.memory_base();
                        let x = ins.memory_index();
                        te.add(0x400 + b as u64);
                        te.add(0x500 + x as u64);
                        te.add(0x600 + ins.memory_index_scale() as u64);
                        tl.add(0x700 + (b != Register::None) as u64 * 2 + (x != Register::None) as u64);
                        let d = ins.memory_displacement64();
                        let dmask = (co.has_displacement()
                            && p.reloc(ip + co.displacement_offset() as u64, co.displacement_size()).is_some())
                            || (d as i64).unsigned_abs() >= 0x100000;
                        if dmask {
                            te.add(0x3FE);
                        } else {
                            te.add(0x800 ^ d.rotate_left(12));
                        }
                    }
                }
                k => {
                    te.add(0x900 + k as u64);
                    tl.add(0x900 + k as u64);
                }
            }
        }
        he.add(te.0);
        hl.add(tl.0);
        hm.add(mn);
        loose_tokens.push(tl.0);
    }
    cfg[0] = leaders.iter().filter(|a| in_func(**a)).count() as u32;
    // value bag + mnemonic classes
    let mut vals = BTreeSet::new();
    let mut mseq = String::new();
    for (ins, co) in &insns {
        if insns.len() <= MSEQ_MAX {
            if let Some(c) = mnem_class(ins) {
                mseq.push(c);
            }
        }
        let mn = ins.mnemonic();
        for i in 0..ins.op_count() {
            match ins.op_kind(i) {
                OpKind::Memory if !ins.is_ip_rel_memory_operand() => {
                    let b = ins.memory_base();
                    if matches!(b, Register::RSP | Register::RBP | Register::ESP | Register::EBP) {
                        continue;
                    }
                    if mn == Mnemonic::Lea || mn == Mnemonic::Nop {
                        continue;
                    }
                    let d = ins.memory_displacement64() as i64;
                    let dmask = co.has_displacement()
                        && p.reloc(ins.ip() + co.displacement_offset() as u64, co.displacement_size()).is_some();
                    if !dmask && d.unsigned_abs() < 0x1000 {
                        vals.insert(format!("m{}@{:x}", ins.memory_size().size(), d));
                    }
                }
                OpKind::Immediate8
                | OpKind::Immediate16
                | OpKind::Immediate32
                | OpKind::Immediate64
                | OpKind::Immediate8to16
                | OpKind::Immediate8to32
                | OpKind::Immediate8to64
                | OpKind::Immediate32to64 => {
                    let keep = matches!(
                        mn,
                        Mnemonic::Cmp | Mnemonic::Test | Mnemonic::And | Mnemonic::Or | Mnemonic::Xor
                            | Mnemonic::Mov | Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc
                            | Mnemonic::Imul
                    );
                    if !keep {
                        continue;
                    }
                    if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register
                        && matches!(ins.op_register(0), Register::RSP | Register::RBP)
                    {
                        continue;
                    }
                    let raw = ins.immediate(i);
                    let masked = (co.has_immediate()
                        && p.reloc(ins.ip() + co.immediate_offset() as u64, co.immediate_size()).is_some())
                        || p.looks_like_addr(raw);
                    if masked {
                        continue;
                    }
                    let mut v = raw as i64;
                    // 32-bit operations: interpret as signed 32-bit
                    if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register && ins.op_register(0).is_gpr32() {
                        v = raw as u32 as i32 as i64;
                    }
                    if matches!(mn, Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc) {
                        if (0..63).contains(&v) {
                            v = 1i64 << v;
                        } else {
                            continue;
                        }
                    }
                    if v == 0 || v == 1 || v == -1 {
                        continue;
                    }
                    vals.insert(imm_token(v));
                }
                _ => {}
            }
        }
    }
    // minhash over 3-gram shingles (or the whole stream if shorter)
    let mut mh = [u32::MAX; NMH];
    let mut shingle = |s: u64| {
        for (k, m) in mh.iter_mut().enumerate() {
            let v = (mix64(s ^ (k as u64).wrapping_mul(0x9e3779b97f4a7c15)) >> 32) as u32;
            if v < *m {
                *m = v;
            }
        }
    };
    if loose_tokens.len() < 3 {
        let mut h = H::new();
        for t in &loose_tokens {
            h.add(*t);
        }
        shingle(h.0);
    } else {
        for w in loose_tokens.windows(3) {
            let mut h = H::new();
            h.add(w[0]);
            h.add(w[1]);
            h.add(w[2]);
            shingle(h.0);
        }
    }
    Feat {
        id,
        name,
        n_insn: insns.len(),
        n_bytes,
        h_exact: he.0,
        h_loose: hl.0,
        h_mnem: hm.0,
        cfg,
        consts,
        callees,
        strings,
        minhash: mh,
        vals,
        mseq,
    }
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\t' => o.push_str("\\t"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\\' => o.push_str("\\\\"),
            '\u{1f}' => o.push_str("\\x1f"),
            c => o.push(c),
        }
    }
    o
}

fn write_feats(path: &str, feats: &[Feat]) {
    let f = std::fs::File::create(path).expect("create output");
    let mut w = std::io::BufWriter::new(f);
    writeln!(w, "id\tname\tn_insn\tn_bytes\th_exact\th_loose\th_mnem\tcfg\tconsts\tcallees\tstrings\tminhash\tvals\tmseq").unwrap();
    for f in feats {
        let mh: String = f.minhash.iter().map(|m| format!("{:08x}", m)).collect();
        let cfg: Vec<String> = f.cfg.iter().map(|x| x.to_string()).collect();
        let consts: Vec<&str> = f.consts.iter().map(|s| s.as_str()).collect();
        let strs: Vec<String> = f.strings.iter().map(|s| esc(s)).collect();
        let vals: Vec<&str> = f.vals.iter().map(|s| s.as_str()).collect();
        writeln!(
            w,
            "{}\t{}\t{}\t{}\t{:016x}\t{:016x}\t{:016x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            f.id,
            esc(&f.name),
            f.n_insn,
            f.n_bytes,
            f.h_exact,
            f.h_loose,
            f.h_mnem,
            cfg.join(","),
            consts.join(";"),
            f.callees.iter().map(|s| esc(s)).collect::<Vec<_>>().join("\u{1f}"),
            strs.join("\u{1f}"),
            mh,
            vals.join(";"),
            f.mseq
        )
        .unwrap();
    }
}

// ------------------------------------------------------------------------------------------------
// exe side
// ------------------------------------------------------------------------------------------------
struct Exe {
    data: Vec<u8>,
    base: u64,
    size_of_image: u64,
    secs: Vec<(String, u64, u64, usize, usize)>, // name, rva, vsize, raw_off, raw_size
}

impl Exe {
    fn load(path: &str) -> Exe {
        let data = std::fs::read(path).expect("read exe");
        let rd16 = |o: usize| u16::from_le_bytes([data[o], data[o + 1]]) as usize;
        let rd32 = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap()) as usize;
        let pe = rd32(0x3c);
        let nsec = rd16(pe + 6);
        let optsz = rd16(pe + 20);
        let opt = pe + 24;
        let base = u64::from_le_bytes(data[opt + 24..opt + 32].try_into().unwrap());
        let size_of_image = rd32(opt + 56) as u64;
        let mut secs = Vec::new();
        for i in 0..nsec {
            let o = opt + optsz + 40 * i;
            let name = String::from_utf8_lossy(&data[o..o + 8]).trim_end_matches('\0').to_string();
            secs.push((name, rd32(o + 12) as u64, rd32(o + 8) as u64, rd32(o + 20), rd32(o + 16)));
        }
        Exe { data, base, size_of_image, secs }
    }
    fn sec_of(&self, va: u64) -> Option<&(String, u64, u64, usize, usize)> {
        let rva = va.checked_sub(self.base)?;
        self.secs.iter().find(|s| rva >= s.1 && rva < s.1 + s.2.max(s.4 as u64))
    }
    fn bytes(&self, va: u64, len: usize) -> Option<&[u8]> {
        let s = self.sec_of(va)?;
        let off = (va - self.base - s.1) as usize;
        if off + len > s.4 {
            return None;
        }
        Some(&self.data[s.3 + off..s.3 + off + len])
    }
}

impl Provider for Exe {
    fn reloc(&self, _addr: u64, _len: usize) -> Option<&str> {
        None
    }
    fn rip_target(&self, ins: &Instruction, _disp_addr: u64, xmm: bool) -> RipTarget {
        let t = ins.ip_rel_memory_address();
        let Some(s) = self.sec_of(t) else { return RipTarget::Other };
        if s.0 == ".text" {
            return RipTarget::Code(t);
        }
        if xmm && s.0 == ".rdata" {
            let sz = mem_bytes(ins.memory_size());
            if sz > 0 {
                if let Some(b) = self.bytes(t, sz) {
                    return RipTarget::Const(const_name(b));
                }
            }
        }
        RipTarget::Other
    }
    fn looks_like_addr(&self, v: u64) -> bool {
        v >= self.base && v < self.base + self.size_of_image
    }
}

fn mem_bytes(ms: MemorySize) -> usize {
    match ms.size() {
        4 | 8 | 16 => ms.size(),
        _ => 0,
    }
}

/// big-endian hex like MSVC's `__real@` / `__xmm@` constant symbols
fn const_name(b: &[u8]) -> String {
    let mut s = String::from("F:");
    for x in b.iter().rev() {
        s.push_str(&format!("{:02x}", x));
    }
    s
}

fn cmd_exe(args: &[String]) {
    let exe = Exe::load(&args[0]);
    let txt = std::fs::read_to_string(&args[1]).expect("read funcs");
    let mut jobs: Vec<(u64, Vec<(u64, u64)>)> = Vec::new();
    for line in txt.lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 2 || c[0].starts_with('#') || c[0] == "va" {
            continue;
        }
        let p = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
        let va = p(c[0]);
        let mut ranges = vec![(va, p(c[1]))];
        for fr in &c[2..] {
            if let Some((a, b)) = fr.split_once('-') {
                ranges.push((p(a), p(b)));
            }
        }
        jobs.push((va, ranges));
    }
    let feats: Vec<Feat> = jobs
        .par_iter()
        .filter_map(|(va, ranges)| {
            let rs: Vec<(u64, &[u8])> = ranges
                .iter()
                .filter_map(|(a, b)| exe.bytes(*a, (b - a) as usize).map(|x| (*a, x)))
                .collect();
            if rs.is_empty() || rs[0].0 != *va {
                return None;
            }
            Some(extract(&exe, format!("{:x}", va), String::new(), &rs))
        })
        .collect();
    write_feats(&args[2], &feats);
    eprintln!("fnsig exe: {} functions", feats.len());
}

// ------------------------------------------------------------------------------------------------
// COFF side
// ------------------------------------------------------------------------------------------------
struct ObjCtx {
    /// absolute (section-virtual) address -> (len, target symbol name)
    relocs: HashMap<u64, (usize, String)>,
    /// symbol name -> resolved data (for ??_C@ strings and local constants)
    sym_data: HashMap<String, Vec<u8>>,
}

impl Provider for ObjCtx {
    fn reloc(&self, addr: u64, len: usize) -> Option<&str> {
        for a in addr..addr + len as u64 {
            if let Some((_, s)) = self.relocs.get(&a) {
                return Some(s.as_str());
            }
        }
        None
    }
    fn rip_target(&self, ins: &Instruction, disp_addr: u64, xmm: bool) -> RipTarget {
        let Some(sym) = self.reloc(disp_addr, 4) else { return RipTarget::Other };
        if let Some(h) = sym.strip_prefix("__real@").or_else(|| sym.strip_prefix("__xmm@")).or_else(|| sym.strip_prefix("__ymm@")) {
            if xmm {
                let sz = mem_bytes(ins.memory_size());
                // __xmm@ is 128 bits; a scalar load from it reads only the low part
                let h = if sz > 0 && h.len() > sz * 2 { &h[h.len() - sz * 2..] } else { h };
                return RipTarget::Const(format!("F:{}", h.to_ascii_lowercase()));
            }
            return RipTarget::Sym(sym.to_string());
        }
        if sym.starts_with("??_C@") {
            if let Some(d) = self.sym_data.get(sym) {
                let wide = sym.starts_with("??_C@_1");
                let s = if wide {
                    let u: Vec<u16> = d.chunks(2).map(|c| u16::from_le_bytes([c[0], *c.get(1).unwrap_or(&0)])).take_while(|c| *c != 0).collect();
                    String::from_utf16_lossy(&u)
                } else {
                    let e = d.iter().position(|c| *c == 0).unwrap_or(d.len());
                    String::from_utf8_lossy(&d[..e]).into_owned()
                };
                return RipTarget::Str(s);
            }
        }
        if xmm {
            if let Some(d) = self.sym_data.get(sym) {
                let sz = mem_bytes(ins.memory_size());
                if sz > 0 && d.len() >= sz {
                    return RipTarget::Const(const_name(&d[..sz]));
                }
            }
        }
        RipTarget::Sym(sym.to_string())
    }
    fn looks_like_addr(&self, _v: u64) -> bool {
        false
    }
}

fn coff_feats(label: &str, origin: &str, data: &[u8], out: &mut Vec<Feat>) {
    let file = match object::File::parse(data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("skip {}: {}", origin, e);
            return;
        }
    };
    // symbol data for strings / constants (whole section from symbol offset)
    let mut sym_data: HashMap<String, Vec<u8>> = HashMap::new();
    for sym in file.symbols() {
        let Ok(name) = sym.name() else { continue };
        if !(name.starts_with("??_C@") || name.starts_with("__real@") || name.starts_with("__xmm@") || sym.kind() == SymbolKind::Data) {
            continue;
        }
        if let Some(si) = sym.section_index() {
            if let Ok(sec) = file.section_by_index(si) {
                if let Ok(d) = sec.data() {
                    let off = sym.address().saturating_sub(sec.address()) as usize;
                    if off < d.len() {
                        sym_data.insert(name.to_string(), d[off..d.len().min(off + 4096)].to_vec());
                    }
                }
            }
        }
    }
    for sec in file.sections() {
        if sec.kind() != SectionKind::Text {
            continue;
        }
        let Ok(d) = sec.data() else { continue };
        if d.is_empty() {
            continue;
        }
        let saddr = sec.address();
        let mut relocs = HashMap::new();
        for (off, r) in sec.relocations() {
            let tname = match r.target() {
                RelocationTarget::Symbol(si) => file
                    .symbol_by_index(si)
                    .ok()
                    .and_then(|s| s.name().ok().map(|n| n.to_string()))
                    .unwrap_or_default(),
                _ => String::new(),
            };
            let len = (r.size() / 8).max(4) as usize;
            relocs.insert(saddr + off, (len, tname));
        }
        // function symbols in this section
        let mut syms: Vec<(u64, String)> = file
            .symbols()
            .filter(|s| s.section_index() == Some(sec.index()) && s.kind() == SymbolKind::Text && s.is_definition())
            .filter_map(|s| s.name().ok().map(|n| (s.address(), n.to_string())))
            .filter(|(_, n)| !n.starts_with('$') && !n.starts_with(".L"))
            .collect();
        syms.sort();
        syms.dedup_by(|a, b| a.0 == b.0);
        if syms.is_empty() {
            continue;
        }
        let ctx = ObjCtx { relocs, sym_data: sym_data.clone() };
        for (i, (a, n)) in syms.iter().enumerate() {
            let end = syms.get(i + 1).map(|x| x.0).unwrap_or(saddr + d.len() as u64);
            let off = (a - saddr) as usize;
            let len = (end - a) as usize;
            if len == 0 || off + len > d.len() {
                continue;
            }
            let mut f = extract(&ctx, format!("{}:{}", label, origin), n.clone(), &[(*a, &d[off..off + len])]);
            f.id = format!("{}|{}", label, origin);
            out.push(f);
        }
    }
}

fn cmd_coff(args: &[String]) {
    let label = &args[0];
    let outp = &args[1];
    let files = &args[2..];
    let per: Vec<Vec<Feat>> = files
        .par_iter()
        .map(|path| {
            let mut out = Vec::new();
            let data = match std::fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("skip {}: {}", path, e);
                    return out;
                }
            };
            let base = std::path::Path::new(path).file_name().unwrap().to_string_lossy().to_string();
            if data.starts_with(b"!<arch>\n") {
                if let Ok(ar) = object::read::archive::ArchiveFile::parse(&*data) {
                    for m in ar.members().flatten() {
                        let mname = String::from_utf8_lossy(m.name()).to_string();
                        if let Ok(md) = m.data(&*data) {
                            coff_feats(label, &format!("{}({})", base, mname), md, &mut out);
                        }
                    }
                }
            } else {
                coff_feats(label, &base, &data, &mut out);
            }
            out
        })
        .collect();
    let feats: Vec<Feat> = per.into_iter().flatten().collect();
    write_feats(outp, &feats);
    eprintln!("fnsig coff [{}]: {} functions from {} files", label, feats.len(), files.len());
}

/// `fnsig icalls`: indirect call/jump operands per function (see the header).
fn cmd_icalls(args: &[String]) {
    let exe = Exe::load(&args[0]);
    let txt = std::fs::read_to_string(&args[1]).expect("read funcs");
    let mut jobs: Vec<(u64, Vec<(u64, u64)>)> = Vec::new();
    for line in txt.lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 2 || c[0].starts_with('#') || c[0] == "va" {
            continue;
        }
        let p = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
        let va = p(c[0]);
        let mut ranges = vec![(va, p(c[1]))];
        for fr in &c[2..] {
            if let Some((a, b)) = fr.split_once('-') {
                ranges.push((p(a), p(b)));
            }
        }
        jobs.push((va, ranges));
    }
    let rows: Vec<(u64, String)> = jobs
        .par_iter()
        .filter_map(|(va, ranges)| {
            let mut set: BTreeSet<String> = BTreeSet::new();
            for (a, b) in ranges {
                let Some(bytes) = exe.bytes(*a, (b - a) as usize) else { continue };
                let mut dec = Decoder::with_ip(64, bytes, *a, DecoderOptions::NONE);
                let mut ins = Instruction::default();
                while dec.can_decode() {
                    dec.decode_out(&mut ins);
                    match ins.flow_control() {
                        FlowControl::IndirectCall | FlowControl::IndirectBranch => {}
                        _ => continue,
                    }
                    match ins.op0_kind() {
                        OpKind::Register => {
                            set.insert("r".into());
                        }
                        OpKind::Memory if ins.is_ip_rel_memory_operand() => {
                            set.insert(format!("g{:x}", ins.ip_rel_memory_address()));
                        }
                        OpKind::Memory if ins.memory_index() == Register::None => {
                            set.insert(format!("v{:x}", ins.memory_displacement64()));
                        }
                        // Jump tables (`jmp [base+idx*8]`) and other indexed forms.
                        OpKind::Memory if ins.flow_control() == FlowControl::IndirectCall => {
                            set.insert("r".into());
                        }
                        _ => {}
                    }
                }
            }
            if set.is_empty() {
                None
            } else {
                Some((*va, set.into_iter().collect::<Vec<_>>().join(",")))
            }
        })
        .collect();
    let mut f = std::io::BufWriter::new(std::fs::File::create(&args[2]).expect("create out"));
    writeln!(f, "va\ticalls").unwrap();
    for (va, s) in &rows {
        writeln!(f, "{va:x}\t{s}").unwrap();
    }
    eprintln!("fnsig icalls: {} functions with indirect calls", rows.len());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("icalls") if args.len() >= 5 => cmd_icalls(&args[2..]),
        Some("retlive") if args.len() >= 7 => retlive::cmd(&args[2..]),
        Some("exe") if args.len() >= 5 => cmd_exe(&args[2..]),
        Some("coff") if args.len() >= 5 => cmd_coff(&args[2..]),
        _ => {
            eprintln!("usage: fnsig exe <exe> <funcs.tsv> <out.tsv> | fnsig coff <label> <out.tsv> <obj/lib>...");
            std::process::exit(2);
        }
    }
}
