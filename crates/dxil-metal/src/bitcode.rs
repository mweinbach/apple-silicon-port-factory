//! Minimal LLVM bitstream reader: just enough of DXIL's LLVM 3.7 bitcode to find the names of
//! the functions a module defines.
//!
//! Graphics and compute PSOs don't carry an entry-point name. `PSV0` only records it from
//! runtime info v3 (validator 1.8+), so for older shaders the defined function is the entry
//! point: a non-library DXIL module defines exactly one function, and everything else is a
//! `dx.op.*` declaration. This reads the module block's `FUNCTION` records (isproto = 0) and
//! the module-level value symbol table, and skips every other block (function bodies, metadata,
//! constants, types) by its length.

use std::collections::{HashMap, HashSet};

const BLOCKINFO: u64 = 0;
const MODULE_BLOCK: u64 = 8;
const CONSTANTS_BLOCK: u64 = 11;
const FUNCTION_BLOCK: u64 = 12;
const VALUE_SYMTAB_BLOCK: u64 = 14;
const METADATA_BLOCK: u64 = 15;
const TYPE_BLOCK_NEW: u64 = 17;

const METADATA_STRING: u64 = 1;
const METADATA_VALUE: u64 = 2;
const METADATA_NODE: u64 = 3;
const METADATA_NAME: u64 = 4;
const METADATA_DISTINCT_NODE: u64 = 5;
const METADATA_KIND: u64 = 6;
const METADATA_LOCATION: u64 = 7;
const METADATA_OLD_NODE: u64 = 8;
const METADATA_OLD_FN_NODE: u64 = 9;
const METADATA_NAMED_NODE: u64 = 10;
const METADATA_GENERIC_DEBUG: u64 = 12;
const METADATA_MODULE: u64 = 32;

const MODULE_CODE_VERSION: u64 = 1;
const MODULE_CODE_GLOBALVAR: u64 = 7;
const MODULE_CODE_FUNCTION: u64 = 8;
const MODULE_CODE_ALIAS_OLD: u64 = 9;
const MODULE_CODE_ALIAS: u64 = 14;

const TYPE_CODE_NUMENTRY: u64 = 1;
const TYPE_CODE_VOID: u64 = 2;
const TYPE_CODE_FLOAT: u64 = 3;
const TYPE_CODE_DOUBLE: u64 = 4;
const TYPE_CODE_POINTER: u64 = 8;
const TYPE_CODE_FUNCTION_OLD: u64 = 9;
const TYPE_CODE_HALF: u64 = 10;
const TYPE_CODE_ARRAY: u64 = 11;
const TYPE_CODE_VECTOR: u64 = 12;
const TYPE_CODE_X86_FP80: u64 = 13;
const TYPE_CODE_FP128: u64 = 14;
const TYPE_CODE_PPC_FP128: u64 = 15;
const TYPE_CODE_STRUCT_ANON: u64 = 18;
const TYPE_CODE_STRUCT_NAME: u64 = 19;
const TYPE_CODE_STRUCT_NAMED: u64 = 20;
const TYPE_CODE_FUNCTION: u64 = 21;

const CST_CODE_SETTYPE: u64 = 1;
const CST_CODE_NULL: u64 = 2;
const CST_CODE_UNDEF: u64 = 3;
const CST_CODE_INTEGER: u64 = 4;
const CST_CODE_FLOAT: u64 = 6;
const CST_CODE_AGGREGATE: u64 = 7;
const CST_CODE_CE_BINOP: u64 = 10;
const CST_CODE_CE_CAST: u64 = 11;
const CST_CODE_CE_SELECT: u64 = 13;
const CST_CODE_CE_EXTRACTELT: u64 = 14;
const CST_CODE_CE_INSERTELT: u64 = 15;
const CST_CODE_CE_SHUFFLEVEC: u64 = 16;
const CST_CODE_CE_CMP: u64 = 17;
const CST_CODE_CE_SHUFVEC_EX: u64 = 19;
const CST_CODE_DATA: u64 = 22;

const FUNC_CODE_DECLAREBLOCKS: u64 = 1;
const FUNC_CODE_INST_BINOP: u64 = 2;
const FUNC_CODE_INST_CAST: u64 = 3;
const FUNC_CODE_INST_GEP_OLD: u64 = 4;
const FUNC_CODE_INST_SELECT: u64 = 5;
const FUNC_CODE_INST_EXTRACTELT: u64 = 6;
const FUNC_CODE_INST_INSERTELT: u64 = 7;
const FUNC_CODE_INST_SHUFFLEVEC: u64 = 8;
const FUNC_CODE_INST_CMP: u64 = 9;
const FUNC_CODE_INST_RET: u64 = 10;
const FUNC_CODE_INST_BR: u64 = 11;
const FUNC_CODE_INST_SWITCH: u64 = 12;
const FUNC_CODE_INST_UNREACHABLE: u64 = 15;
const FUNC_CODE_INST_PHI: u64 = 16;
const FUNC_CODE_INST_ALLOCA: u64 = 19;
const FUNC_CODE_INST_LOAD: u64 = 20;
const FUNC_CODE_INST_STORE_OLD: u64 = 24;
const FUNC_CODE_INST_EXTRACTVAL: u64 = 26;
const FUNC_CODE_INST_INSERTVAL: u64 = 27;
const FUNC_CODE_INST_CMP2: u64 = 28;
const FUNC_CODE_INST_VSELECT: u64 = 29;
const FUNC_CODE_INST_INBOUNDS_GEP_OLD: u64 = 30;
const FUNC_CODE_INST_INDIRECTBR: u64 = 31;
const FUNC_CODE_DEBUG_LOC_AGAIN: u64 = 33;
const FUNC_CODE_INST_CALL: u64 = 34;
const FUNC_CODE_DEBUG_LOC: u64 = 35;
const FUNC_CODE_INST_FENCE: u64 = 36;
const FUNC_CODE_INST_CMPXCHG_OLD: u64 = 37;
const FUNC_CODE_INST_ATOMICRMW: u64 = 38;
const FUNC_CODE_INST_RESUME: u64 = 39;
const FUNC_CODE_INST_LANDINGPAD_OLD: u64 = 40;
const FUNC_CODE_INST_LOADATOMIC: u64 = 41;
const FUNC_CODE_INST_STOREATOMIC_OLD: u64 = 42;
const FUNC_CODE_INST_GEP: u64 = 43;
const FUNC_CODE_INST_STORE: u64 = 44;
const FUNC_CODE_INST_STOREATOMIC: u64 = 45;
const FUNC_CODE_INST_CMPXCHG: u64 = 46;
const FUNC_CODE_INST_LANDINGPAD: u64 = 47;

const CAST_FPTOUI: u64 = 3;
const CAST_FPTOSI: u64 = 4;
const CAST_UITOFP: u64 = 5;
const CAST_SITOFP: u64 = 6;
const CAST_FPTRUNC: u64 = 7;
const CAST_FPEXT: u64 = 8;
const CAST_BITCAST: u64 = 11;

const SFI0_DOUBLES_MASK: u64 = (1u64 << 0) | (1u64 << 5);

const BINOP_ADD: u64 = 0;
const BINOP_AND: u64 = 10;
const BINOP_OR: u64 = 11;
const ICMP_EQ: u64 = 32;

const VST_CODE_ENTRY: u64 = 1;
const VST_CODE_FNENTRY: u64 = 3;

const BLOCKINFO_CODE_SETBID: u64 = 1;

#[derive(Clone, Debug)]
enum Op {
    Literal(u64),
    Fixed(u32),
    Vbr(u32),
    Array,
    Char6,
    Blob,
}

type Abbrev = Vec<Op>;

struct Bits<'a> {
    data: &'a [u8],
    pos: usize, // bit position
}

impl<'a> Bits<'a> {
    fn read(&mut self, n: u32) -> Option<u64> {
        if n == 0 {
            return Some(0);
        }
        if n > 64 || self.pos + n as usize > self.data.len() * 8 {
            return None;
        }
        let mut v = 0u64;
        let mut got = 0u32;
        while got < n {
            let byte = self.data[self.pos / 8] as u64;
            let bit = (self.pos % 8) as u32;
            let take = (8 - bit).min(n - got);
            v |= ((byte >> bit) & ((1u64 << take) - 1)) << got;
            got += take;
            self.pos += take as usize;
        }
        Some(v)
    }

    fn vbr(&mut self, n: u32) -> Option<u64> {
        if !(2..=32).contains(&n) {
            return None;
        }
        let hi = 1u64 << (n - 1);
        let mut v = 0u64;
        let mut shift = 0u32;
        loop {
            let piece = self.read(n)?;
            v |= (piece & (hi - 1)).checked_shl(shift)?;
            if piece & hi == 0 {
                return Some(v);
            }
            shift += n - 1;
            if shift >= 64 {
                return None;
            }
        }
    }

    fn align32(&mut self) {
        self.pos = self.pos.next_multiple_of(32);
    }

    fn at_end(&self) -> bool {
        self.pos >= self.data.len() * 8
    }
}

fn char6(v: u64) -> u64 {
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._"[v as usize & 63] as u64
}

fn read_scalar(b: &mut Bits, op: &Op) -> Option<u64> {
    match *op {
        Op::Literal(v) => Some(v),
        Op::Fixed(w) => b.read(w),
        Op::Vbr(w) => b.vbr(w),
        Op::Char6 => b.read(6).map(char6),
        Op::Array | Op::Blob => None,
    }
}

/// Reads an abbreviated record: (code, operands). Blob bytes become one operand each.
fn read_abbrev_record(b: &mut Bits, a: &Abbrev) -> Option<(u64, Vec<u64>)> {
    let mut vals = Vec::new();
    let mut i = 0;
    while i < a.len() {
        match &a[i] {
            Op::Array => {
                let elt = a.get(i + 1)?;
                let n = b.vbr(6)?;
                for _ in 0..n {
                    vals.push(read_scalar(b, elt)?);
                }
                i += 2;
                continue;
            }
            Op::Blob => {
                let n = b.vbr(6)? as usize;
                b.align32();
                for _ in 0..n {
                    vals.push(b.read(8)?);
                }
                b.align32();
            }
            op => vals.push(read_scalar(b, op)?),
        }
        i += 1;
    }
    if vals.is_empty() {
        return None;
    }
    let code = vals.remove(0);
    Some((code, vals))
}

fn read_define_abbrev(b: &mut Bits) -> Option<Abbrev> {
    let n = b.vbr(5)?;
    let mut ops = Vec::with_capacity(n as usize);
    for _ in 0..n {
        if b.read(1)? == 1 {
            ops.push(Op::Literal(b.vbr(8)?));
        } else {
            ops.push(match b.read(3)? {
                1 => Op::Fixed(b.vbr(5)? as u32),
                2 => Op::Vbr(b.vbr(5)? as u32),
                3 => Op::Array,
                4 => Op::Char6,
                5 => Op::Blob,
                _ => return None,
            });
        }
    }
    Some(ops)
}

enum Entry {
    End,
    Block(u64),
    Record(u64, Vec<u64>),
}

struct Reader<'a> {
    b: Bits<'a>,
    blockinfo: HashMap<u64, Vec<Abbrev>>,
}

impl<'a> Reader<'a> {
    /// Next entry in a block with abbreviation width `w` and abbreviations `abbrevs`.
    /// DEFINE_ABBREV records are absorbed into `abbrevs`.
    fn next(&mut self, w: u32, abbrevs: &mut Vec<Abbrev>) -> Option<Entry> {
        loop {
            match self.b.read(w)? {
                0 => {
                    self.b.align32();
                    return Some(Entry::End);
                }
                1 => {
                    let id = self.b.vbr(8)?;
                    return Some(Entry::Block(id));
                }
                2 => abbrevs.push(read_define_abbrev(&mut self.b)?),
                3 => {
                    let code = self.b.vbr(6)?;
                    let n = self.b.vbr(6)?;
                    let mut ops = Vec::with_capacity(n.min(4096) as usize);
                    for _ in 0..n {
                        ops.push(self.b.vbr(6)?);
                    }
                    return Some(Entry::Record(code, ops));
                }
                id => {
                    let a = abbrevs.get(id as usize - 4)?.clone();
                    let (code, ops) = read_abbrev_record(&mut self.b, &a)?;
                    return Some(Entry::Record(code, ops));
                }
            }
        }
    }

    /// After ENTER_SUBBLOCK's id: reads the header, returns (abbrev width, end bit position).
    fn block_header(&mut self) -> Option<(u32, usize)> {
        let w = self.b.vbr(4)? as u32;
        self.b.align32();
        let words = self.b.read(32)? as usize;
        let end = self.b.pos + words * 32;
        (end <= self.b.data.len() * 8 && w >= 2).then_some((w, end))
    }

    fn skip_block(&mut self) -> Option<()> {
        let (_, end) = self.block_header()?;
        self.b.pos = end;
        Some(())
    }

    fn blockinfo_block(&mut self) -> Option<()> {
        let (w, _) = self.block_header()?;
        let mut cur: Option<u64> = None;
        loop {
            // DEFINE_ABBREV inside BLOCKINFO applies to the current SETBID block.
            let abbrev_id = self.b.read(w)?;
            match abbrev_id {
                0 => {
                    self.b.align32();
                    return Some(());
                }
                1 => {
                    self.b.vbr(8)?;
                    self.skip_block()?;
                }
                2 => {
                    let a = read_define_abbrev(&mut self.b)?;
                    self.blockinfo.entry(cur?).or_default().push(a);
                }
                3 => {
                    let code = self.b.vbr(6)?;
                    let n = self.b.vbr(6)?;
                    let mut ops = Vec::new();
                    for _ in 0..n {
                        ops.push(self.b.vbr(6)?);
                    }
                    if code == BLOCKINFO_CODE_SETBID {
                        cur = ops.first().copied();
                    }
                }
                _ => return None, // no abbreviations are defined for BLOCKINFO itself
            }
        }
    }

    fn value_symtab(&mut self, names: &mut HashMap<u64, String>) -> Option<()> {
        let (w, _) = self.block_header()?;
        let mut abbrevs = self.blockinfo.get(&VALUE_SYMTAB_BLOCK).cloned().unwrap_or_default();
        loop {
            match self.next(w, &mut abbrevs)? {
                Entry::End => return Some(()),
                Entry::Block(_) => self.skip_block()?,
                Entry::Record(code, ops) => {
                    let skip = match code {
                        VST_CODE_ENTRY => 1,
                        VST_CODE_FNENTRY => 2,
                        _ => continue,
                    };
                    if ops.len() > skip {
                        let name: String = ops[skip..].iter().map(|&c| (c as u8) as char).collect();
                        names.insert(ops[0], name);
                    }
                }
            }
        }
    }

    fn metadata_block(
        &mut self,
        next_md_id: &mut u64,
        md_strings: &mut HashMap<u64, String>,
        md_nodes: &mut HashMap<u64, Vec<u64>>,
        named_nodes: &mut HashMap<String, Vec<u64>>,
    ) -> Option<()> {
        let (w, _) = self.block_header()?;
        let mut abbrevs = self.blockinfo.get(&METADATA_BLOCK).cloned().unwrap_or_default();
        let mut pending_name: Option<String> = None;
        loop {
            match self.next(w, &mut abbrevs)? {
                Entry::End => return Some(()),
                Entry::Block(_) => self.skip_block()?,
                Entry::Record(code, ops) => match code {
                    METADATA_NAME => {
                        let s: String = ops.iter().map(|&c| (c as u8) as char).collect();
                        pending_name = Some(s);
                    }
                    METADATA_NAMED_NODE => {
                        if let Some(name) = pending_name.take() {
                            named_nodes.entry(name).or_default().extend(ops);
                        }
                    }
                    METADATA_STRING => {
                        let id = *next_md_id;
                        *next_md_id += 1;
                        let s: String = ops.iter().map(|&c| (c as u8) as char).collect();
                        md_strings.insert(id, s);
                    }
                    METADATA_NODE | METADATA_DISTINCT_NODE => {
                        let id = *next_md_id;
                        *next_md_id += 1;
                        md_nodes.insert(id, ops);
                    }
                    METADATA_VALUE
                    | METADATA_LOCATION
                    | METADATA_OLD_NODE
                    | METADATA_OLD_FN_NODE
                    | METADATA_GENERIC_DEBUG..=METADATA_MODULE => {
                        *next_md_id += 1;
                    }
                    METADATA_KIND => {}
                    _ => {}
                },
            }
        }
    }

    fn module_entry_points(&mut self) -> Option<Vec<String>> {
        let (w, _) = self.block_header()?;
        let mut abbrevs = self.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
        let mut next_md_id = 0u64;
        let mut md_strings = HashMap::new();
        let mut md_nodes = HashMap::new();
        let mut named_nodes = HashMap::new();
        loop {
            match self.next(w, &mut abbrevs)? {
                Entry::End => break,
                Entry::Block(BLOCKINFO) => self.blockinfo_block()?,
                Entry::Block(METADATA_BLOCK) => {
                    self.metadata_block(&mut next_md_id, &mut md_strings, &mut md_nodes, &mut named_nodes)?;
                }
                Entry::Block(_) => self.skip_block()?,
                Entry::Record(..) => {}
            }
        }
        let ep_ids = named_nodes.get("dx.entryPoints")?;
        let mut out = Vec::new();
        for &ep_id in ep_ids {
            let Some(tuple) = md_nodes.get(&ep_id) else {
                continue;
            };
            // DXIL !dx.entryPoints tuple: !{fn_or_null, !"EntryName", !sigs, !resources, !props}
            // METADATA_NODE operands are 1-based (0 = null, k = md[k - 1]).
            if let Some(&name_ref) = tuple.get(1) {
                if name_ref > 0 {
                    if let Some(name) = md_strings.get(&(name_ref - 1)) {
                        if !name.is_empty() {
                            out.push(name.clone());
                        }
                    }
                }
            }
        }
        (!out.is_empty()).then_some(out)
    }

    fn module(&mut self) -> Option<Vec<String>> {
        let (w, _) = self.block_header()?;
        let mut abbrevs = self.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
        let mut next_value = 0u64;
        let mut defined = Vec::new();
        let mut names = HashMap::new();
        loop {
            match self.next(w, &mut abbrevs)? {
                Entry::End => break,
                Entry::Block(BLOCKINFO) => self.blockinfo_block()?,
                Entry::Block(VALUE_SYMTAB_BLOCK) => self.value_symtab(&mut names)?,
                Entry::Block(_) => self.skip_block()?,
                Entry::Record(code, ops) => match code {
                    MODULE_CODE_GLOBALVAR | MODULE_CODE_ALIAS | MODULE_CODE_ALIAS_OLD => next_value += 1,
                    MODULE_CODE_FUNCTION => {
                        // [type, callingconv, isproto, linkage, ...]
                        if ops.get(2) == Some(&0) {
                            defined.push(next_value);
                        }
                        next_value += 1;
                    }
                    _ => {}
                },
            }
        }
        Some(
            defined
                .iter()
                .filter_map(|id| names.get(id).filter(|n| !n.starts_with("aw2.op.")).cloned())
                .collect(),
        )
    }
}

/// Names of the functions defined (not just declared) in an LLVM bitcode module, in
/// definition order. `None` if the bitcode can't be read.
pub fn defined_functions(bitcode: &[u8]) -> Option<Vec<String>> {
    if bitcode.get(0..4)? != b"BC\xC0\xDE" {
        return None;
    }
    let mut r = Reader { b: Bits { data: bitcode, pos: 32 }, blockinfo: HashMap::new() };
    let mut top = Vec::new();
    while !r.b.at_end() {
        match r.next(2, &mut top)? {
            Entry::Block(BLOCKINFO) => r.blockinfo_block()?,
            Entry::Block(MODULE_BLOCK) => return r.module(),
            Entry::Block(_) => r.skip_block()?,
            Entry::End | Entry::Record(..) => return None,
        }
    }
    None
}

/// Non-empty DXIL entry point names from the LLVM bitcode's `!dx.entryPoints` named metadata.
///
/// Every DXIL shader records its entry point tuple `!{fn_or_null, !"EntryName", ...}` in
/// `!dx.entryPoints` (even pre-v3 `PSV0` containers, pass-through Hull shaders whose control-point
/// function is `null`, and Hull shaders that also define a patch-constant function).
pub fn entry_points(bitcode: &[u8]) -> Option<Vec<String>> {
    if bitcode.get(0..4)? != b"BC\xC0\xDE" {
        return None;
    }
    let mut r = Reader { b: Bits { data: bitcode, pos: 32 }, blockinfo: HashMap::new() };
    let mut top = Vec::new();
    while !r.b.at_end() {
        match r.next(2, &mut top)? {
            Entry::Block(BLOCKINFO) => r.blockinfo_block()?,
            Entry::Block(MODULE_BLOCK) => return r.module_entry_points(),
            Entry::Block(_) => r.skip_block()?,
            Entry::End | Entry::Record(..) => return None,
        }
    }
    None
}

/// The bitcode inside a `DXIL` part (after the program and bitcode headers).
pub fn dxil_part_bitcode(dxil_part: &[u8]) -> Option<&[u8]> {
    let rd = |o: usize| dxil_part.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()) as usize);
    if dxil_part.get(8..12)? != b"DXIL" {
        return None;
    }
    let off = rd(16)?;
    let size = rd(20)?;
    dxil_part.get(8 + off..8 + off + size)
}

/// Writes LSB-first bit fields, like LLVM's BitstreamWriter.
#[derive(Default)]
struct W {
    out: Vec<u8>,
    bit: usize,
}

impl W {
    fn put(&mut self, v: u64, n: u32) {
        for i in 0..n {
            if self.bit.is_multiple_of(8) {
                self.out.push(0);
            }
            if (v >> i) & 1 == 1 {
                *self.out.last_mut().unwrap() |= 1 << (self.bit % 8);
            }
            self.bit += 1;
        }
    }
    fn vbr(&mut self, mut v: u64, n: u32) {
        let hi = 1u64 << (n - 1);
        loop {
            if v < hi {
                self.put(v, n);
                return;
            }
            self.put((v & (hi - 1)) | hi, n);
            v >>= n - 1;
        }
    }
    fn align(&mut self) {
        while !self.bit.is_multiple_of(32) {
            self.put(0, 1);
        }
    }
    /// ENTER_SUBBLOCK with a placeholder length; returns the length word's byte index.
    fn enter(&mut self, w: u32, id: u64, new_w: u32) -> usize {
        self.put(1, w);
        self.vbr(id, 8);
        self.vbr(new_w as u64, 4);
        self.align();
        let at = self.out.len();
        self.put(0, 32);
        at
    }
    fn end(&mut self, w: u32, len_at: usize) {
        self.put(0, w);
        self.align();
        let words = ((self.out.len() - len_at - 4) / 4) as u32;
        self.out[len_at..len_at + 4].copy_from_slice(&words.to_le_bytes());
    }
    fn unabbrev(&mut self, w: u32, code: u64, ops: &[u64]) {
        self.put(3, w);
        self.vbr(code, 6);
        self.vbr(ops.len() as u64, 6);
        for &o in ops {
            self.vbr(o, 6);
        }
    }
    fn copy_subblock(&mut self, w: u32, id: u64, sub_w: u32, raw_bytes: &[u8]) {
        self.put(1, w);
        self.vbr(id, 8);
        self.vbr(sub_w as u64, 4);
        self.align();
        let words = (raw_bytes.len() / 4) as u64;
        self.put(words, 32);
        self.out.extend_from_slice(raw_bytes);
        self.bit = self.out.len() * 8;
    }
}

/// True if `name` is an SM 6.9 `dx.op.*` intrinsic that Metal Shader Converter 4.0.1 rejects
/// with `UnsupportedInstruction` and that [`lower_sm69_bitcode`] lowers into an internal
/// helper function.
fn is_lowered_sm69_dxop(name: &str) -> bool {
    matches!(
        name,
        "dx.op.hitObject_MakeNop"
            | "dx.op.maybeReorderThread"
            | "dx.op.allocateRayQuery2"
            | "dx.op.dot.v2f32"
            | "dx.op.dot.v3f32"
            | "dx.op.dot.v4f32"
            | "dx.op.vectorReduce.v2i1"
            | "dx.op.vectorReduce.v3i1"
            | "dx.op.vectorReduce.v4i1"
            | "dx.op.isSpecialFloat.v2f32"
            | "dx.op.isSpecialFloat.v3f32"
            | "dx.op.isSpecialFloat.v4f32"
    ) || name.strip_prefix("dx.op.rawBufferVectorLoad.v").is_some_and(|s| {
        s.strip_suffix("i32")
            .or_else(|| s.strip_suffix("f32"))
            .and_then(|d| d.parse::<usize>().ok())
            .is_some_and(|d| (1..=16).contains(&d))
    })
}

enum ModuleItem<'a> {
    Record(u64, Vec<u64>),
    Subblock { id: u64, w: u32, bit_start: usize, bit_end: usize, raw: &'a [u8] },
}

fn parse_subblock_records(bitcode: &[u8], bit_start: usize, bit_end: usize, w: u32, init_abbrevs: &[Abbrev]) -> Option<Vec<(u64, Vec<u64>)>> {
    let mut r = Reader { b: Bits { data: bitcode, pos: bit_start }, blockinfo: HashMap::new() };
    let mut abbrevs = init_abbrevs.to_vec();
    let mut out = Vec::new();
    while r.b.pos < bit_end {
        match r.next(w, &mut abbrevs)? {
            Entry::End => break,
            Entry::Block(_) => r.skip_block()?,
            Entry::Record(code, ops) => out.push((code, ops)),
        }
    }
    Some(out)
}

#[allow(clippy::too_many_arguments)]
fn emit_helper_function(
    w: &mut W,
    mw: u32,
    name: &str,
    retty: u64,
    params: &[u64],
    types: &[(u64, Vec<u64>)],
    fn_by_name: &HashMap<String, (u64, u64, Vec<u64>)>,
    module_val_size: u64,
) -> Option<()> {
    let f_at = w.enter(mw, FUNCTION_BLOCK, 4);
    w.unabbrev(4, FUNC_CODE_DECLAREBLOCKS, &[1]);
    let mut cur = module_val_size + params.len() as u64;
    let args: Vec<u64> = (0..params.len() as u64).map(|i| module_val_size + i).collect();

    if name == "dx.op.maybeReorderThread" {
        w.unabbrev(4, FUNC_CODE_INST_RET, &[]);
        w.end(4, f_at);
        return Some(());
    }

    if name == "dx.op.hitObject_MakeNop" {
        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[retty]);
        w.unabbrev(4, CST_CODE_NULL, &[]);
        w.end(4, c_at);
        let c_zero = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - c_zero]);
        w.end(4, f_at);
        return Some(());
    }

    if name == "dx.op.allocateRayQuery2" {
        let &(callee_vid, _, _) = fn_by_name.get("dx.op.allocateRayQuery")?;
        let i32_ty = *params.first()?;
        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[i32_ty]);
        w.unabbrev(4, CST_CODE_INTEGER, &[178 << 1]);
        w.end(4, c_at);
        let c_178 = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_CALL, &[0, 0, cur - callee_vid, cur - c_178, cur - *args.get(1)?]);
        let res = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - res]);
        w.end(4, f_at);
        return Some(());
    }

    if let Some(dim_str) = name.strip_prefix("dx.op.dot.v").and_then(|s| s.strip_suffix("f32")) {
        let dim: usize = dim_str.parse().ok()?;
        let (callee_name, opc) = match dim {
            2 => ("dx.op.dot2.f32", 54u64),
            3 => ("dx.op.dot3.f32", 55u64),
            4 => ("dx.op.dot4.f32", 56u64),
            _ => return None,
        };
        let &(callee_vid, _, _) = fn_by_name.get(callee_name)?;
        let i32_ty = *params.first()?;
        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[i32_ty]);
        w.unabbrev(4, CST_CODE_NULL, &[]);
        for i in 1..dim {
            w.unabbrev(4, CST_CODE_INTEGER, &[(i as u64) << 1]);
        }
        w.unabbrev(4, CST_CODE_INTEGER, &[opc << 1]);
        w.end(4, c_at);
        let idx_consts: Vec<u64> = (0..dim as u64).map(|i| cur + i).collect();
        cur += dim as u64;
        let c_opc = cur;
        cur += 1;
        let mut a_elts = Vec::with_capacity(dim);
        let mut b_elts = Vec::with_capacity(dim);
        for &ic in &idx_consts {
            w.unabbrev(4, FUNC_CODE_INST_EXTRACTELT, &[cur - *args.get(1)?, cur - ic]);
            a_elts.push(cur);
            cur += 1;
        }
        for &ic in &idx_consts {
            w.unabbrev(4, FUNC_CODE_INST_EXTRACTELT, &[cur - *args.get(2)?, cur - ic]);
            b_elts.push(cur);
            cur += 1;
        }
        let mut call_ops = vec![0, 0, cur - callee_vid, cur - c_opc];
        for &x in &a_elts {
            call_ops.push(cur - x);
        }
        for &x in &b_elts {
            call_ops.push(cur - x);
        }
        w.unabbrev(4, FUNC_CODE_INST_CALL, &call_ops);
        let res = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - res]);
        w.end(4, f_at);
        return Some(());
    }

    if let Some(dim_str) = name.strip_prefix("dx.op.vectorReduce.v").and_then(|s| s.strip_suffix("i1")) {
        let dim: usize = dim_str.parse().ok()?;
        let i32_ty = *params.first()?;
        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[i32_ty]);
        w.unabbrev(4, CST_CODE_NULL, &[]);
        for i in 1..dim {
            w.unabbrev(4, CST_CODE_INTEGER, &[(i as u64) << 1]);
        }
        w.unabbrev(4, CST_CODE_INTEGER, &[309 << 1]);
        w.end(4, c_at);
        let idx_consts: Vec<u64> = (0..dim as u64).map(|i| cur + i).collect();
        cur += dim as u64;
        let c_309 = cur;
        cur += 1;
        let mut elts = Vec::with_capacity(dim);
        for &ic in &idx_consts {
            w.unabbrev(4, FUNC_CODE_INST_EXTRACTELT, &[cur - *args.get(1)?, cur - ic]);
            elts.push(cur);
            cur += 1;
        }
        let mut and_v = elts[0];
        for &e in &elts[1..] {
            w.unabbrev(4, FUNC_CODE_INST_BINOP, &[cur - and_v, cur - e, BINOP_AND]);
            and_v = cur;
            cur += 1;
        }
        let mut or_v = elts[0];
        for &e in &elts[1..] {
            w.unabbrev(4, FUNC_CODE_INST_BINOP, &[cur - or_v, cur - e, BINOP_OR]);
            or_v = cur;
            cur += 1;
        }
        w.unabbrev(4, FUNC_CODE_INST_CMP2, &[cur - args[0], cur - c_309, ICMP_EQ]);
        let cond = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_VSELECT, &[cur - and_v, cur - or_v, cur - cond]);
        let res = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - res]);
        w.end(4, f_at);
        return Some(());
    }

    if let Some(dim_str) = name.strip_prefix("dx.op.isSpecialFloat.v").and_then(|s| s.strip_suffix("f32")) {
        let dim: usize = dim_str.parse().ok()?;
        let &(callee_vid, _, _) = fn_by_name.get("dx.op.isSpecialFloat.f32")?;
        let i32_ty = *params.first()?;
        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[i32_ty]);
        w.unabbrev(4, CST_CODE_NULL, &[]);
        for i in 1..dim {
            w.unabbrev(4, CST_CODE_INTEGER, &[(i as u64) << 1]);
        }
        w.unabbrev(4, CST_CODE_SETTYPE, &[retty]);
        w.unabbrev(4, CST_CODE_UNDEF, &[]);
        w.end(4, c_at);
        let idx_consts: Vec<u64> = (0..dim as u64).map(|i| cur + i).collect();
        cur += dim as u64;
        let mut vec_cur = cur;
        cur += 1;
        let mut elts = Vec::with_capacity(dim);
        for &ic in &idx_consts {
            w.unabbrev(4, FUNC_CODE_INST_EXTRACTELT, &[cur - *args.get(1)?, cur - ic]);
            let x = cur;
            cur += 1;
            w.unabbrev(4, FUNC_CODE_INST_CALL, &[0, 0, cur - callee_vid, cur - args[0], cur - x]);
            elts.push(cur);
            cur += 1;
        }
        for (&e, &ic) in elts.iter().zip(&idx_consts) {
            w.unabbrev(4, FUNC_CODE_INST_INSERTELT, &[cur - vec_cur, cur - e, cur - ic]);
            vec_cur = cur;
            cur += 1;
        }
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - vec_cur]);
        w.end(4, f_at);
        return Some(());
    }

    if let Some(rest) = name.strip_prefix("dx.op.rawBufferVectorLoad.v") {
        let (dim_str, is_f32) = if let Some(d) = rest.strip_suffix("f32") {
            (d, true)
        } else if let Some(d) = rest.strip_suffix("i32") {
            (d, false)
        } else {
            return None;
        };
        let dim: usize = dim_str.parse().ok()?;
        let callee_name = if is_f32 { "dx.op.rawBufferLoad.f32" } else { "dx.op.rawBufferLoad.i32" };
        let (callee_vid, _, callee_params) = fn_by_name.get(callee_name)?;
        let i32_ty = *params.first()?;
        let i8_ty = *callee_params.get(4)?;
        let vec_ty = *types.get(retty as usize)?.1.get(1)?;
        let n_chunks = dim.div_ceil(4);
        let last_chunk = dim - (n_chunks - 1) * 4;
        let last_mask = (1u64 << last_chunk) - 1;

        let c_at = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[i32_ty]);
        w.unabbrev(4, CST_CODE_NULL, &[]);
        for i in 1..dim {
            w.unabbrev(4, CST_CODE_INTEGER, &[(i as u64) << 1]);
        }
        w.unabbrev(4, CST_CODE_INTEGER, &[139 << 1]);
        for k in 1..n_chunks {
            w.unabbrev(4, CST_CODE_INTEGER, &[((k * 16) as u64) << 1]);
        }
        w.unabbrev(4, CST_CODE_SETTYPE, &[i8_ty]);
        if n_chunks > 1 {
            w.unabbrev(4, CST_CODE_INTEGER, &[15 << 1]);
        }
        if last_mask != 15 || n_chunks == 1 {
            w.unabbrev(4, CST_CODE_INTEGER, &[last_mask << 1]);
        }
        w.unabbrev(4, CST_CODE_SETTYPE, &[vec_ty]);
        w.unabbrev(4, CST_CODE_UNDEF, &[]);
        w.unabbrev(4, CST_CODE_SETTYPE, &[retty]);
        w.unabbrev(4, CST_CODE_UNDEF, &[]);
        w.end(4, c_at);

        let idx_consts: Vec<u64> = (0..dim as u64).map(|i| cur + i).collect();
        cur += dim as u64;
        let c_139 = cur;
        cur += 1;
        let off_consts: Vec<u64> = (0..(n_chunks - 1) as u64).map(|i| cur + i).collect();
        cur += (n_chunks - 1) as u64;
        let (c_mask15, c_mask_last) = if n_chunks > 1 {
            let m15 = cur;
            cur += 1;
            let mlast = if last_mask != 15 {
                let m = cur;
                cur += 1;
                m
            } else {
                m15
            };
            (m15, mlast)
        } else {
            let m = cur;
            cur += 1;
            (m, m)
        };
        let mut vec_cur = cur;
        cur += 1;
        let resret_undef = cur;
        cur += 1;

        let mut lds = Vec::with_capacity(n_chunks);
        for k in 0..n_chunks {
            let off_v = if k == 0 {
                *args.get(3)?
            } else {
                w.unabbrev(4, FUNC_CODE_INST_BINOP, &[cur - *args.get(3)?, cur - off_consts[k - 1], BINOP_ADD]);
                let v = cur;
                cur += 1;
                v
            };
            let mask_v = if k + 1 < n_chunks { c_mask15 } else { c_mask_last };
            w.unabbrev(
                4,
                FUNC_CODE_INST_CALL,
                &[
                    0,
                    0,
                    cur - *callee_vid,
                    cur - c_139,
                    cur - *args.get(1)?,
                    cur - *args.get(2)?,
                    cur - off_v,
                    cur - mask_v,
                    cur - *args.get(4)?,
                ],
            );
            lds.push(cur);
            cur += 1;
        }

        let mut scalars = Vec::with_capacity(dim);
        for (k, &ld) in lds.iter().enumerate() {
            let cnt = if k + 1 < n_chunks { 4 } else { last_chunk };
            for c in 0..cnt as u64 {
                w.unabbrev(4, FUNC_CODE_INST_EXTRACTVAL, &[cur - ld, c]);
                scalars.push(cur);
                cur += 1;
            }
        }
        w.unabbrev(4, FUNC_CODE_INST_EXTRACTVAL, &[cur - lds[0], 4]);
        let status_v = cur;
        cur += 1;

        for (&s, &ic) in scalars.iter().zip(&idx_consts) {
            w.unabbrev(4, FUNC_CODE_INST_INSERTELT, &[cur - vec_cur, cur - s, cur - ic]);
            vec_cur = cur;
            cur += 1;
        }

        w.unabbrev(4, FUNC_CODE_INST_INSERTVAL, &[cur - resret_undef, cur - vec_cur, 0]);
        let r0 = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_INSERTVAL, &[cur - r0, cur - status_v, 1]);
        let r1 = cur;
        cur += 1;
        w.unabbrev(4, FUNC_CODE_INST_RET, &[cur - r1]);
        w.end(4, f_at);
        return Some(());
    }

    None
}

/// Lowers unsupported SM 6.9 `dx.op.*` intrinsics (`dx.op.dot.v*`, `dx.op.vectorReduce.v*`,
/// `dx.op.isSpecialFloat.v*`, `dx.op.rawBufferVectorLoad.v*`, `dx.op.allocateRayQuery2`,
/// `dx.op.hitObject_MakeNop`, `dx.op.maybeReorderThread`) into internal helper functions backed
/// by SM 6.5/6.6 scalar intrinsics so `libmetalirconverter.dylib` compiles them cleanly.
/// Returns `None` if `bitcode` contains no such intrinsics.
pub fn lower_sm69_bitcode(bitcode: &[u8]) -> Option<Vec<u8>> {
    if bitcode.get(0..4)? != b"BC\xC0\xDE" {
        return None;
    }
    let mut r = Reader { b: Bits { data: bitcode, pos: 32 }, blockinfo: HashMap::new() };
    let mut top = Vec::new();
    let mut top_blockinfo: Option<(u32, &[u8])> = None;
    let mut mw = 3u32;
    let mut items = Vec::new();

    while !r.b.at_end() {
        match r.next(2, &mut top)? {
            Entry::Block(BLOCKINFO) => {
                let (bw, bend) = r.block_header()?;
                let bstart = r.b.pos;
                r.b.pos = bstart - 32; // rewind over word length? No, parse via blockinfo_block
                // Re-parse blockinfo properly:
                let raw = &bitcode[bstart / 8..bend / 8];
                r.b.pos = bstart;
                let mut cur: Option<u64> = None;
                while r.b.pos < bend {
                    match r.b.read(bw)? {
                        0 => {
                            r.b.align32();
                            break;
                        }
                        1 => {
                            r.b.vbr(8)?;
                            r.skip_block()?;
                        }
                        2 => {
                            let a = read_define_abbrev(&mut r.b)?;
                            r.blockinfo.entry(cur?).or_default().push(a);
                        }
                        3 => {
                            let code = r.b.vbr(6)?;
                            let n = r.b.vbr(6)?;
                            let mut ops = Vec::new();
                            for _ in 0..n {
                                ops.push(r.b.vbr(6)?);
                            }
                            if code == BLOCKINFO_CODE_SETBID {
                                cur = ops.first().copied();
                            }
                        }
                        _ => return None,
                    }
                }
                top_blockinfo = Some((bw, raw));
            }
            Entry::Block(MODULE_BLOCK) => {
                let (w, mend) = r.block_header()?;
                mw = w;
                let mut mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                while r.b.pos < mend {
                    match r.next(mw, &mut mab)? {
                        Entry::End => break,
                        Entry::Block(sbid) => {
                            let (sw, send) = r.block_header()?;
                            let sstart = r.b.pos;
                            let raw = &bitcode[sstart / 8..send / 8];
                            if sbid == BLOCKINFO {
                                let mut cur: Option<u64> = None;
                                while r.b.pos < send {
                                    match r.b.read(sw)? {
                                        0 => {
                                            r.b.align32();
                                            break;
                                        }
                                        1 => {
                                            r.b.vbr(8)?;
                                            r.skip_block()?;
                                        }
                                        2 => {
                                            let a = read_define_abbrev(&mut r.b)?;
                                            r.blockinfo.entry(cur?).or_default().push(a);
                                        }
                                        3 => {
                                            let code = r.b.vbr(6)?;
                                            let n = r.b.vbr(6)?;
                                            let mut ops = Vec::new();
                                            for _ in 0..n {
                                                ops.push(r.b.vbr(6)?);
                                            }
                                            if code == BLOCKINFO_CODE_SETBID {
                                                cur = ops.first().copied();
                                            }
                                        }
                                        _ => return None,
                                    }
                                }
                                mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                            } else {
                                r.b.pos = send;
                            }
                            items.push(ModuleItem::Subblock { id: sbid, w: sw, bit_start: sstart, bit_end: send, raw });
                        }
                        Entry::Record(code, ops) => items.push(ModuleItem::Record(code, ops)),
                    }
                }
                break;
            }
            Entry::Block(_) => r.skip_block()?,
            Entry::End | Entry::Record(..) => return None,
        }
    }

    let mut types = Vec::new();
    let mut num_module_consts = 0u64;
    let mut vst_records = Vec::new();
    let mut names = HashMap::new();
    let mut orig_fn_blocks = Vec::new();

    for it in &items {
        if let &ModuleItem::Subblock { id, w, bit_start, bit_end, raw } = it {
            match id {
                TYPE_BLOCK_NEW => {
                    let init = r.blockinfo.get(&TYPE_BLOCK_NEW).map_or(&[][..], Vec::as_slice);
                    for (rc, rops) in parse_subblock_records(bitcode, bit_start, bit_end, w, init)? {
                        if rc != TYPE_CODE_NUMENTRY && rc != TYPE_CODE_STRUCT_NAME {
                            types.push((rc, rops));
                        }
                    }
                }
                CONSTANTS_BLOCK => {
                    let init = r.blockinfo.get(&CONSTANTS_BLOCK).map_or(&[][..], Vec::as_slice);
                    for (rc, _) in parse_subblock_records(bitcode, bit_start, bit_end, w, init)? {
                        if rc != CST_CODE_SETTYPE {
                            num_module_consts += 1;
                        }
                    }
                }
                VALUE_SYMTAB_BLOCK => {
                    let init = r.blockinfo.get(&VALUE_SYMTAB_BLOCK).map_or(&[][..], Vec::as_slice);
                    for (rc, rops) in parse_subblock_records(bitcode, bit_start, bit_end, w, init)? {
                        if rc == VST_CODE_ENTRY && rops.len() > 1 {
                            let name: String = rops[1..].iter().map(|&c| (c as u8) as char).collect();
                            names.insert(rops[0], name);
                        }
                        vst_records.push((rc, rops));
                    }
                }
                FUNCTION_BLOCK => orig_fn_blocks.push((w, raw)),
                _ => {}
            }
        }
    }

    if !names.values().any(|n| is_lowered_sm69_dxop(n)) {
        return None;
    }

    let get_fn_sig = |ty_id: u64| -> Option<(u64, Vec<u64>)> {
        let (mut rc, mut rops) = types.get(ty_id as usize)?.clone();
        if rc == TYPE_CODE_POINTER {
            let pointee = *rops.first()?;
            (rc, rops) = types.get(pointee as usize)?.clone();
        }
        if rc != TYPE_CODE_FUNCTION || rops.len() < 2 {
            return None;
        }
        Some((rops[1], rops[2..].to_vec()))
    };

    let mut next_val = 0u64;
    let mut fn_by_name = HashMap::new();
    let mut fn_info = HashMap::new();
    for it in &items {
        if let ModuleItem::Record(rc, rops) = it {
            match *rc {
                MODULE_CODE_GLOBALVAR | MODULE_CODE_ALIAS | MODULE_CODE_ALIAS_OLD => next_val += 1,
                MODULE_CODE_FUNCTION => {
                    let vid = next_val;
                    next_val += 1;
                    let nm = names.get(&vid).cloned().unwrap_or_default();
                    let (retty, params) = get_fn_sig(*rops.first()?)?;
                    if !nm.is_empty() {
                        fn_by_name.insert(nm.clone(), (vid, retty, params.clone()));
                    }
                    fn_info.insert(vid, (nm, retty, params));
                }
                _ => {}
            }
        }
    }

    let module_val_size = next_val + num_module_consts;
    enum PendingFn<'a> {
        Orig(u32, &'a [u8]),
        Synth(String, u64, Vec<u64>),
    }
    let mut fn_bodies_queue = Vec::new();
    let mut orig_fn_idx = 0usize;
    let mut emitted_fn_bodies = false;

    let mut w = W::default();
    w.out.extend_from_slice(b"BC\xC0\xDE");
    w.bit = 32;
    if let Some((bw, raw)) = top_blockinfo {
        w.copy_subblock(2, BLOCKINFO, bw, raw);
    }
    let m_at = w.enter(2, MODULE_BLOCK, mw);

    let mut vid_cursor = 0u64;
    for it in items {
        match it {
            ModuleItem::Record(rc, mut rops) => {
                match rc {
                    MODULE_CODE_GLOBALVAR | MODULE_CODE_ALIAS | MODULE_CODE_ALIAS_OLD => vid_cursor += 1,
                    MODULE_CODE_FUNCTION => {
                        let vid = vid_cursor;
                        vid_cursor += 1;
                        let (nm, retty, params) = fn_info.get(&vid)?.clone();
                        if rops.get(2) == Some(&0) {
                            let &(sw, raw) = orig_fn_blocks.get(orig_fn_idx)?;
                            orig_fn_idx += 1;
                            fn_bodies_queue.push(PendingFn::Orig(sw, raw));
                        } else if is_lowered_sm69_dxop(&nm) {
                            if rops.len() < 4 {
                                return None;
                            }
                            rops[2] = 0; // isproto = 0
                            rops[3] = 3; // linkage = internal
                            fn_bodies_queue.push(PendingFn::Synth(nm, retty, params));
                        }
                    }
                    _ => {}
                }
                w.unabbrev(mw, rc, &rops);
            }
            ModuleItem::Subblock { id, w: sw, raw, .. } => {
                if id == VALUE_SYMTAB_BLOCK {
                    let v_at = w.enter(mw, VALUE_SYMTAB_BLOCK, sw);
                    for (vrc, mut vrops) in vst_records.clone() {
                        if vrc == VST_CODE_ENTRY && vrops.len() > 1 {
                            let nm: String = vrops[1..].iter().map(|&c| (c as u8) as char).collect();
                            if is_lowered_sm69_dxop(&nm) {
                                let new_nm = format!("dxil_lower.op.{}", &nm[6..]);
                                let vid = vrops[0];
                                vrops.clear();
                                vrops.push(vid);
                                vrops.extend(new_nm.bytes().map(|b| b as u64));
                            }
                        }
                        w.unabbrev(sw, vrc, &vrops);
                    }
                    w.end(sw, v_at);
                } else if id == FUNCTION_BLOCK {
                    if !emitted_fn_bodies {
                        emitted_fn_bodies = true;
                        for fb in &fn_bodies_queue {
                            match fb {
                                PendingFn::Orig(fsw, fraw) => w.copy_subblock(mw, FUNCTION_BLOCK, *fsw, fraw),
                                PendingFn::Synth(nm, retty, params) => {
                                    emit_helper_function(&mut w, mw, nm, *retty, params, &types, &fn_by_name, module_val_size)?;
                                }
                            }
                        }
                    }
                } else {
                    w.copy_subblock(mw, id, sw, raw);
                }
            }
        }
    }

    if !emitted_fn_bodies {
        for fb in &fn_bodies_queue {
            match fb {
                PendingFn::Orig(fsw, fraw) => w.copy_subblock(mw, FUNCTION_BLOCK, *fsw, fraw),
                PendingFn::Synth(nm, retty, params) => {
                    emit_helper_function(&mut w, mw, nm, *retty, params, &types, &fn_by_name, module_val_size)?;
                }
            }
        }
    }

    w.end(mw, m_at);
    Some(w.out)
}

/// Reads an abbreviated record, returning `None` if the abbreviation uses `Op::Blob`
/// (which cannot be losslessly re-emitted as an unabbreviated record).
fn read_abbrev_record_no_blob(b: &mut Bits, a: &Abbrev) -> Option<(u64, Vec<u64>)> {
    if a.iter().any(|op| matches!(op, Op::Blob)) {
        return None;
    }
    read_abbrev_record(b, a)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FpShape {
    DoubleScalar,
    DoubleVec(u64),
    FloatScalar,
    FloatVec(u64),
    HalfScalar,
    HalfVec(u64),
    Other,
}

impl FpShape {
    fn is_double(self) -> bool {
        matches!(self, FpShape::DoubleScalar | FpShape::DoubleVec(_))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ValTy {
    Known(u64),
    /// Pointee type of an `alloca` whose operand was an explicit element type.
    PtrTo(u64),
    /// Shufflevector result whose exact vector type ID may not exist in `types`.
    Shape(FpShape),
    /// Non-double value whose exact type ID is not needed (only produced by ops that cannot yield
    /// `double` once pointer/aggregate/function types referencing `double` have been rejected).
    Unknown,
}

enum SubblockItem<'a> {
    Record(u64, Vec<u64>),
    Subblock { id: u64, w: u32, bit_start: usize, bit_end: usize, raw: &'a [u8] },
}

fn parse_subblock_items<'a>(
    bitcode: &'a [u8],
    bit_start: usize,
    bit_end: usize,
    w: u32,
    init_abbrevs: &[Abbrev],
) -> Option<Vec<SubblockItem<'a>>> {
    let mut b = Bits { data: bitcode, pos: bit_start };
    let mut abbrevs = init_abbrevs.to_vec();
    let mut out = Vec::new();
    while b.pos < bit_end {
        match b.read(w)? {
            0 => {
                b.align32();
                break;
            }
            1 => {
                let sbid = b.vbr(8)?;
                let sw = b.vbr(4)? as u32;
                b.align32();
                let words = b.read(32)? as usize;
                let sstart = b.pos;
                let send = sstart + words * 32;
                if send > bit_end || sw < 2 {
                    return None;
                }
                let raw = &bitcode[sstart / 8..send / 8];
                b.pos = send;
                out.push(SubblockItem::Subblock { id: sbid, w: sw, bit_start: sstart, bit_end: send, raw });
            }
            2 => abbrevs.push(read_define_abbrev(&mut b)?),
            3 => {
                let code = b.vbr(6)?;
                let n = b.vbr(6)?;
                let mut ops = Vec::with_capacity(n.min(4096) as usize);
                for _ in 0..n {
                    ops.push(b.vbr(6)?);
                }
                out.push(SubblockItem::Record(code, ops));
            }
            id => {
                let a = abbrevs.get(id as usize - 4)?.clone();
                let (code, ops) = read_abbrev_record_no_blob(&mut b, &a)?;
                out.push(SubblockItem::Record(code, ops));
            }
        }
    }
    Some(out)
}

fn parse_subblock_records_no_blob(
    bitcode: &[u8],
    bit_start: usize,
    bit_end: usize,
    w: u32,
    init_abbrevs: &[Abbrev],
) -> Option<Vec<(u64, Vec<u64>)>> {
    let items = parse_subblock_items(bitcode, bit_start, bit_end, w, init_abbrevs)?;
    let mut out = Vec::with_capacity(items.len());
    for it in items {
        match it {
            SubblockItem::Record(code, ops) => out.push((code, ops)),
            SubblockItem::Subblock { .. } => return None,
        }
    }
    Some(out)
}

/// Fast check: returns `true` iff `bitcode` contains at least one `TYPE_CODE_DOUBLE` record in
/// `TYPE_BLOCK_NEW` (skips all function bodies, constants, and metadata in O(1)).
fn bitcode_has_double_type(bitcode: &[u8]) -> bool {
    if bitcode.get(0..4) != Some(b"BC\xC0\xDE") {
        return false;
    }
    let mut r = Reader { b: Bits { data: bitcode, pos: 32 }, blockinfo: HashMap::new() };
    let mut top = Vec::new();
    while !r.b.at_end() {
        let Some(entry) = r.next(2, &mut top) else {
            return false;
        };
        match entry {
            Entry::Block(BLOCKINFO) => {
                if r.blockinfo_block().is_none() {
                    return false;
                }
            }
            Entry::Block(MODULE_BLOCK) => {
                let Some((mw, mend)) = r.block_header() else {
                    return false;
                };
                let mut mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                while r.b.pos < mend {
                    let Some(me) = r.next(mw, &mut mab) else {
                        return false;
                    };
                    match me {
                        Entry::End => return false,
                        Entry::Block(BLOCKINFO) => {
                            if r.blockinfo_block().is_none() {
                                return false;
                            }
                            mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                        }
                        Entry::Block(TYPE_BLOCK_NEW) => {
                            let Some((tw, tend)) = r.block_header() else {
                                return false;
                            };
                            let mut tab = r.blockinfo.get(&TYPE_BLOCK_NEW).cloned().unwrap_or_default();
                            while r.b.pos < tend {
                                let Some(te) = r.next(tw, &mut tab) else {
                                    return false;
                                };
                                match te {
                                    Entry::End => break,
                                    Entry::Block(_) => {
                                        if r.skip_block().is_none() {
                                            return false;
                                        }
                                    }
                                    Entry::Record(TYPE_CODE_DOUBLE, _) => return true,
                                    Entry::Record(..) => {}
                                }
                            }
                            return false;
                        }
                        Entry::Block(_) => {
                            if r.skip_block().is_none() {
                                return false;
                            }
                        }
                        Entry::Record(..) => {}
                    }
                }
                return false;
            }
            Entry::Block(_) => {
                if r.skip_block().is_none() {
                    return false;
                }
            }
            Entry::End | Entry::Record(..) => return false,
        }
    }
    false
}

fn fp_shape_of_tid(types: &[(u64, Vec<u64>)], tid: u64) -> Option<FpShape> {
    let (rc, rops) = types.get(tid as usize)?;
    Some(match *rc {
        TYPE_CODE_DOUBLE => FpShape::DoubleScalar,
        TYPE_CODE_FLOAT => FpShape::FloatScalar,
        TYPE_CODE_HALF => FpShape::HalfScalar,
        TYPE_CODE_VECTOR => {
            let n = *rops.first()?;
            let elt = *rops.get(1)?;
            match types.get(elt as usize)?.0 {
                TYPE_CODE_DOUBLE => FpShape::DoubleVec(n),
                TYPE_CODE_FLOAT => FpShape::FloatVec(n),
                TYPE_CODE_HALF => FpShape::HalfVec(n),
                _ => FpShape::Other,
            }
        }
        _ => FpShape::Other,
    })
}

fn fp_shape_of_val(types: &[(u64, Vec<u64>)], vt: ValTy) -> Option<FpShape> {
    match vt {
        ValTy::Known(tid) => fp_shape_of_tid(types, tid),
        ValTy::PtrTo(_) => Some(FpShape::Other),
        ValTy::Shape(s) => Some(s),
        ValTy::Unknown => None,
    }
}

fn rewrite_fp64_cast_opcode(castopc: u64, src: Option<FpShape>, dst: FpShape) -> Option<u64> {
    let dst_dbl = dst.is_double();
    let src_dbl = src.is_some_and(FpShape::is_double);
    match castopc {
        CAST_FPTRUNC => {
            if src_dbl {
                match (src?, dst) {
                    (FpShape::DoubleScalar, FpShape::FloatScalar) => Some(CAST_BITCAST),
                    (FpShape::DoubleVec(n), FpShape::FloatVec(m)) if n == m => Some(CAST_BITCAST),
                    (FpShape::DoubleScalar, FpShape::HalfScalar) => Some(CAST_FPTRUNC),
                    (FpShape::DoubleVec(n), FpShape::HalfVec(m)) if n == m => Some(CAST_FPTRUNC),
                    _ => None,
                }
            } else if src.is_none() && matches!(dst, FpShape::FloatScalar | FpShape::FloatVec(_)) {
                // Unknown source truncated to float cannot be disambiguated safely.
                None
            } else {
                Some(CAST_FPTRUNC)
            }
        }
        CAST_FPEXT => {
            if dst_dbl {
                match (src?, dst) {
                    (FpShape::FloatScalar, FpShape::DoubleScalar) => Some(CAST_BITCAST),
                    (FpShape::FloatVec(n), FpShape::DoubleVec(m)) if n == m => Some(CAST_BITCAST),
                    (FpShape::HalfScalar, FpShape::DoubleScalar) => Some(CAST_FPEXT),
                    (FpShape::HalfVec(n), FpShape::DoubleVec(m)) if n == m => Some(CAST_FPEXT),
                    _ => None,
                }
            } else {
                Some(CAST_FPEXT)
            }
        }
        CAST_BITCAST => {
            if src_dbl || dst_dbl {
                // Only no-op bitcast between identical double shapes stays valid after lowering.
                let s = src?;
                (s == dst).then_some(CAST_BITCAST)
            } else {
                Some(CAST_BITCAST)
            }
        }
        CAST_FPTOUI | CAST_FPTOSI | CAST_UITOFP | CAST_SITOFP => Some(castopc),
        other => {
            if src_dbl || dst_dbl {
                None
            } else {
                Some(other)
            }
        }
    }
}

fn rewrite_constants_block_fp64(
    bitcode: &[u8],
    bit_start: usize,
    bit_end: usize,
    w_bits: u32,
    init_abbrevs: &[Abbrev],
    types: &[(u64, Vec<u64>)],
    double_scalar_tys: &HashSet<u64>,
    double_vec_tys: &HashSet<u64>,
    out_val_types: &mut Vec<ValTy>,
) -> Option<Vec<(u64, Vec<u64>)>> {
    let records = parse_subblock_records_no_blob(bitcode, bit_start, bit_end, w_bits, init_abbrevs)?;
    let mut cur_ty: Option<u64> = None;
    let mut out = Vec::with_capacity(records.len());
    for (code, mut ops) in records {
        if code == CST_CODE_SETTYPE {
            cur_ty = ops.first().copied();
            out.push((code, ops));
            continue;
        }
        let ty = cur_ty?;
        let is_dbl_scalar = double_scalar_tys.contains(&ty);
        let is_dbl_vec = double_vec_tys.contains(&ty);
        let is_dbl = is_dbl_scalar || is_dbl_vec;
        match code {
            CST_CODE_FLOAT if is_dbl_scalar => {
                if ops.len() != 1 {
                    return None;
                }
                let f = f64::from_bits(ops[0]) as f32;
                ops[0] = f.to_bits() as u64;
            }
            CST_CODE_DATA if is_dbl_vec => {
                for x in &mut ops {
                    let f = f64::from_bits(*x) as f32;
                    *x = f.to_bits() as u64;
                }
            }
            CST_CODE_CE_CAST => {
                // [castopc, opty, opval]
                if ops.len() != 3 {
                    return None;
                }
                let src = fp_shape_of_tid(types, ops[1])?;
                let dst = fp_shape_of_tid(types, ty)?;
                ops[0] = rewrite_fp64_cast_opcode(ops[0], Some(src), dst)?;
            }
            CST_CODE_NULL
            | CST_CODE_UNDEF
            | CST_CODE_AGGREGATE
            | CST_CODE_CE_BINOP
            | CST_CODE_CE_SELECT
            | CST_CODE_CE_EXTRACTELT
            | CST_CODE_CE_INSERTELT
            | CST_CODE_CE_SHUFFLEVEC
            | CST_CODE_CE_CMP
            | CST_CODE_CE_SHUFVEC_EX => {}
            _ if is_dbl => return None,
            _ => {}
        }
        out_val_types.push(ValTy::Known(ty));
        out.push((code, ops));
    }
    Some(out)
}

/// Lowers pure in-register FP64 (`double`) SSA operations in an LLVM 3.7 DXIL bitcode module to
/// FP32 (`float`) so Apple's `AGXCompilerCore` (which has no FP64 GPU hardware and aborts with
/// `report_fatal_error` when Metal Shader Converter leaks raw `double` into AIR) can compile the
/// pipeline cleanly.
///
/// Safety gate: returns `None` without modifying the module if `double` does not appear or if
/// `double` (scalar or vector) is referenced by any memory/ABI type (`pointer`, `struct`, `array`,
/// `function` signature), `alloca`, `bitcast` across differing types (e.g. `double <-> i64`),
/// extended FP types, or unrecognized bitcode records.
pub fn lower_fp64_bitcode(bitcode: &[u8]) -> Option<Vec<u8>> {
    if !bitcode_has_double_type(bitcode) {
        return None;
    }

    let mut r = Reader { b: Bits { data: bitcode, pos: 32 }, blockinfo: HashMap::new() };
    let mut top = Vec::new();
    let mut top_blockinfo: Option<(u32, &[u8])> = None;
    let mut mw = 3u32;
    let mut items = Vec::new();

    while !r.b.at_end() {
        match r.next(2, &mut top)? {
            Entry::Block(BLOCKINFO) => {
                let (bw, bend) = r.block_header()?;
                let bstart = r.b.pos;
                let raw = &bitcode[bstart / 8..bend / 8];
                let mut cur: Option<u64> = None;
                while r.b.pos < bend {
                    match r.b.read(bw)? {
                        0 => {
                            r.b.align32();
                            break;
                        }
                        1 => {
                            r.b.vbr(8)?;
                            r.skip_block()?;
                        }
                        2 => {
                            let a = read_define_abbrev(&mut r.b)?;
                            r.blockinfo.entry(cur?).or_default().push(a);
                        }
                        3 => {
                            let code = r.b.vbr(6)?;
                            let n = r.b.vbr(6)?;
                            let mut ops = Vec::new();
                            for _ in 0..n {
                                ops.push(r.b.vbr(6)?);
                            }
                            if code == BLOCKINFO_CODE_SETBID {
                                cur = ops.first().copied();
                            }
                        }
                        _ => return None,
                    }
                }
                top_blockinfo = Some((bw, raw));
            }
            Entry::Block(MODULE_BLOCK) => {
                let (w, mend) = r.block_header()?;
                mw = w;
                let mut mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                while r.b.pos < mend {
                    match r.next(mw, &mut mab)? {
                        Entry::End => break,
                        Entry::Block(sbid) => {
                            let (sw, send) = r.block_header()?;
                            let sstart = r.b.pos;
                            let raw = &bitcode[sstart / 8..send / 8];
                            if sbid == BLOCKINFO {
                                let mut cur: Option<u64> = None;
                                while r.b.pos < send {
                                    match r.b.read(sw)? {
                                        0 => {
                                            r.b.align32();
                                            break;
                                        }
                                        1 => {
                                            r.b.vbr(8)?;
                                            r.skip_block()?;
                                        }
                                        2 => {
                                            let a = read_define_abbrev(&mut r.b)?;
                                            r.blockinfo.entry(cur?).or_default().push(a);
                                        }
                                        3 => {
                                            let code = r.b.vbr(6)?;
                                            let n = r.b.vbr(6)?;
                                            let mut ops = Vec::new();
                                            for _ in 0..n {
                                                ops.push(r.b.vbr(6)?);
                                            }
                                            if code == BLOCKINFO_CODE_SETBID {
                                                cur = ops.first().copied();
                                            }
                                        }
                                        _ => return None,
                                    }
                                }
                                mab = r.blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
                            } else {
                                r.b.pos = send;
                            }
                            items.push(ModuleItem::Subblock { id: sbid, w: sw, bit_start: sstart, bit_end: send, raw });
                        }
                        Entry::Record(code, ops) => items.push(ModuleItem::Record(code, ops)),
                    }
                }
                break;
            }
            Entry::Block(_) => r.skip_block()?,
            Entry::End | Entry::Record(..) => return None,
        }
    }

    // Require LLVM 3.7 relative value IDs (VERSION = 1).
    let mut version: Option<u64> = None;
    for it in &items {
        if let ModuleItem::Record(MODULE_CODE_VERSION, ops) = it {
            version = ops.first().copied();
        }
    }
    if version.is_some_and(|v| v != 1) {
        return None;
    }

    // Parse TYPE_BLOCK_NEW once and validate safety gates.
    let mut raw_type_records: Option<(u32, Vec<(u64, Vec<u64>)>)> = None;
    let mut types: Vec<(u64, Vec<u64>)> = Vec::new();
    for it in &items {
        if let &ModuleItem::Subblock { id: TYPE_BLOCK_NEW, w, bit_start, bit_end, .. } = it {
            if raw_type_records.is_some() {
                return None;
            }
            let init = r.blockinfo.get(&TYPE_BLOCK_NEW).map_or(&[][..], Vec::as_slice);
            let recs = parse_subblock_records_no_blob(bitcode, bit_start, bit_end, w, init)?;
            for (rc, rops) in &recs {
                if *rc == 0 || *rc > TYPE_CODE_FUNCTION {
                    return None;
                }
                if matches!(*rc, TYPE_CODE_X86_FP80 | TYPE_CODE_FP128 | TYPE_CODE_PPC_FP128) {
                    return None;
                }
                if *rc != TYPE_CODE_NUMENTRY && *rc != TYPE_CODE_STRUCT_NAME {
                    types.push((*rc, rops.clone()));
                }
            }
            raw_type_records = Some((w, recs));
        }
    }
    let (_, type_records) = raw_type_records?;

    let mut double_scalar_tys = HashSet::new();
    for (tid, (rc, _)) in types.iter().enumerate() {
        if *rc == TYPE_CODE_DOUBLE {
            double_scalar_tys.insert(tid as u64);
        }
    }
    if double_scalar_tys.is_empty() {
        return None;
    }
    let mut double_vec_tys = HashSet::new();
    for (tid, (rc, rops)) in types.iter().enumerate() {
        if *rc == TYPE_CODE_VECTOR && rops.get(1).is_some_and(|elt| double_scalar_tys.contains(elt)) {
            double_vec_tys.insert(tid as u64);
        }
    }
    let is_double_tid = |t: u64| double_scalar_tys.contains(&t) || double_vec_tys.contains(&t);

    // Reject if any memory/ABI type references double (scalar or vector).
    for (rc, rops) in &types {
        match *rc {
            TYPE_CODE_POINTER => {
                if rops.first().is_some_and(|&t| is_double_tid(t)) {
                    return None;
                }
            }
            TYPE_CODE_ARRAY => {
                if rops.get(1).is_some_and(|&t| is_double_tid(t)) {
                    return None;
                }
            }
            TYPE_CODE_STRUCT_ANON | TYPE_CODE_STRUCT_NAMED => {
                if rops.iter().skip(1).any(|&t| is_double_tid(t)) {
                    return None;
                }
            }
            TYPE_CODE_FUNCTION => {
                if rops.iter().skip(1).any(|&t| is_double_tid(t)) {
                    return None;
                }
            }
            TYPE_CODE_FUNCTION_OLD => {
                if rops.iter().skip(2).any(|&t| is_double_tid(t)) {
                    return None;
                }
            }
            _ => {}
        }
    }

    let get_fn_sig = |ty_id: u64| -> Option<(u64, Vec<u64>)> {
        let (mut rc, mut rops) = types.get(ty_id as usize)?.clone();
        if rc == TYPE_CODE_POINTER {
            let pointee = *rops.first()?;
            (rc, rops) = types.get(pointee as usize)?.clone();
        }
        match rc {
            TYPE_CODE_FUNCTION if rops.len() >= 2 => Some((rops[1], rops[2..].to_vec())),
            TYPE_CODE_FUNCTION_OLD if rops.len() >= 3 => Some((rops[2], rops[3..].to_vec())),
            _ => None,
        }
    };

    // Walk MODULE_BLOCK in order to build the exact module-level ValueList and rewrite
    // module-level CONSTANTS_BLOCKs + each defined FUNCTION_BLOCK.
    let mut module_val_types: Vec<ValTy> = Vec::new();
    let mut defined_fn_params: Vec<Vec<u64>> = Vec::new();
    let mut rewritten_mod_consts: HashMap<usize, Vec<(u64, Vec<u64>)>> = HashMap::new();

    for (idx, it) in items.iter().enumerate() {
        match it {
            ModuleItem::Record(rc, rops) => match *rc {
                MODULE_CODE_GLOBALVAR => {
                    // ops[0] is the pointee type (or pointer type when explicit-type bit is set).
                    let tid = *rops.first()?;
                    let vt = if types.get(tid as usize).is_some_and(|(c, _)| *c == TYPE_CODE_POINTER) {
                        ValTy::Known(tid)
                    } else {
                        ValTy::PtrTo(tid)
                    };
                    module_val_types.push(vt);
                }
                MODULE_CODE_FUNCTION => {
                    let fty = *rops.first()?;
                    module_val_types.push(ValTy::Known(fty));
                    if rops.get(2) == Some(&0) {
                        let (_, params) = get_fn_sig(fty)?;
                        defined_fn_params.push(params);
                    }
                }
                MODULE_CODE_ALIAS | MODULE_CODE_ALIAS_OLD => {
                    let tid = *rops.first()?;
                    module_val_types.push(ValTy::Known(tid));
                }
                _ => {}
            },
            ModuleItem::Subblock { id: CONSTANTS_BLOCK, w, bit_start, bit_end, .. } => {
                let init = r.blockinfo.get(&CONSTANTS_BLOCK).map_or(&[][..], Vec::as_slice);
                let recs = rewrite_constants_block_fp64(
                    bitcode,
                    *bit_start,
                    *bit_end,
                    *w,
                    init,
                    &types,
                    &double_scalar_tys,
                    &double_vec_tys,
                    &mut module_val_types,
                )?;
                rewritten_mod_consts.insert(idx, recs);
            }
            _ => {}
        }
    }

    // Rewrite every FUNCTION_BLOCK with exact per-function ValueList tracking.
    enum RewrittenFnItem<'a> {
        Record(u64, Vec<u64>),
        Constants(u32, Vec<(u64, Vec<u64>)>),
        RawSubblock { id: u64, w: u32, raw: &'a [u8] },
    }
    let mut rewritten_fns: HashMap<usize, (u32, Vec<RewrittenFnItem>)> = HashMap::new();
    let mut fn_def_idx = 0usize;

    let back_idx = |raw: u64, cur_vid: u64| -> Option<usize> {
        let cur32 = cur_vid as u32;
        let val_no = cur32.wrapping_sub(raw as u32);
        (val_no < cur32).then_some(val_no as usize)
    };
    let resolve_op = |raw: u64, explicit_ty: Option<u64>, cur_vid: u64, val_types: &[ValTy]| -> Option<ValTy> {
        if let Some(i) = back_idx(raw, cur_vid) {
            val_types.get(i).copied()
        } else {
            Some(ValTy::Known(explicit_ty?))
        }
    };

    for (idx, it) in items.iter().enumerate() {
        let &ModuleItem::Subblock { id: FUNCTION_BLOCK, w: fw, bit_start, bit_end, .. } = it else {
            continue;
        };
        let params = defined_fn_params.get(fn_def_idx)?;
        fn_def_idx += 1;

        let mut val_types = module_val_types.clone();
        for &pty in params {
            val_types.push(ValTy::Known(pty));
        }

        let init = r.blockinfo.get(&FUNCTION_BLOCK).map_or(&[][..], Vec::as_slice);
        let fitems = parse_subblock_items(bitcode, bit_start, bit_end, fw, init)?;
        let mut out_fitems = Vec::with_capacity(fitems.len());

        for fi in fitems {
            match fi {
                SubblockItem::Subblock { id: CONSTANTS_BLOCK, w: cw, bit_start: cs, bit_end: ce, .. } => {
                    let cinit = r.blockinfo.get(&CONSTANTS_BLOCK).map_or(&[][..], Vec::as_slice);
                    let recs = rewrite_constants_block_fp64(
                        bitcode,
                        cs,
                        ce,
                        cw,
                        cinit,
                        &types,
                        &double_scalar_tys,
                        &double_vec_tys,
                        &mut val_types,
                    )?;
                    out_fitems.push(RewrittenFnItem::Constants(cw, recs));
                }
                SubblockItem::Subblock { id, w: sw, raw, .. } => {
                    out_fitems.push(RewrittenFnItem::RawSubblock { id, w: sw, raw });
                }
                SubblockItem::Record(code, mut ops) => {
                    let cur_vid = val_types.len() as u64;
                    match code {
                        FUNC_CODE_DECLAREBLOCKS
                        | FUNC_CODE_INST_RET
                        | FUNC_CODE_INST_BR
                        | FUNC_CODE_INST_SWITCH
                        | FUNC_CODE_INST_UNREACHABLE
                        | FUNC_CODE_INST_STORE_OLD
                        | FUNC_CODE_INST_INDIRECTBR
                        | FUNC_CODE_DEBUG_LOC_AGAIN
                        | FUNC_CODE_DEBUG_LOC
                        | FUNC_CODE_INST_FENCE
                        | FUNC_CODE_INST_RESUME
                        | FUNC_CODE_INST_STOREATOMIC_OLD
                        | FUNC_CODE_INST_STORE
                        | FUNC_CODE_INST_STOREATOMIC => {}

                        FUNC_CODE_INST_BINOP | FUNC_CODE_INST_SELECT | FUNC_CODE_INST_VSELECT
                        | FUNC_CODE_INST_INSERTELT | FUNC_CODE_INST_INSERTVAL => {
                            let raw0 = *ops.first()?;
                            let ty0 = resolve_op(raw0, ops.get(1).copied(), cur_vid, &val_types)?;
                            val_types.push(ty0);
                        }

                        FUNC_CODE_INST_CAST => {
                            let raw0 = *ops.first()?;
                            let (src_vt, dst_tid, opc_idx) = if let Some(i) = back_idx(raw0, cur_vid) {
                                if ops.len() != 3 {
                                    return None;
                                }
                                let src = *val_types.get(i)?;
                                (src, ops[1], 2)
                            } else {
                                if ops.len() != 4 {
                                    return None;
                                }
                                (ValTy::Known(ops[1]), ops[2], 3)
                            };
                            let dst_shape = fp_shape_of_tid(&types, dst_tid)?;
                            let src_shape = fp_shape_of_val(&types, src_vt);
                            ops[opc_idx] = rewrite_fp64_cast_opcode(ops[opc_idx], src_shape, dst_shape)?;
                            val_types.push(ValTy::Known(dst_tid));
                        }

                        FUNC_CODE_INST_EXTRACTELT => {
                            let raw0 = *ops.first()?;
                            let vec_vt = resolve_op(raw0, ops.get(1).copied(), cur_vid, &val_types)?;
                            let elt_vt = match vec_vt {
                                ValTy::Known(tid) => match types.get(tid as usize) {
                                    Some((TYPE_CODE_VECTOR, rops)) => {
                                        rops.get(1).map_or(ValTy::Unknown, |&e| ValTy::Known(e))
                                    }
                                    _ => ValTy::Unknown,
                                },
                                ValTy::Shape(FpShape::DoubleVec(_)) => ValTy::Shape(FpShape::DoubleScalar),
                                ValTy::Shape(FpShape::FloatVec(_)) => ValTy::Shape(FpShape::FloatScalar),
                                ValTy::Shape(FpShape::HalfVec(_)) => ValTy::Shape(FpShape::HalfScalar),
                                _ => ValTy::Unknown,
                            };
                            val_types.push(elt_vt);
                        }

                        FUNC_CODE_INST_SHUFFLEVEC => {
                            let raw0 = *ops.first()?;
                            let (vec_vt, mut next) = if let Some(i) = back_idx(raw0, cur_vid) {
                                (*val_types.get(i)?, 1)
                            } else {
                                (ValTy::Known(*ops.get(1)?), 2)
                            };
                            next += 1; // skip op1 (same type as op0)
                            let mask_raw = *ops.get(next)?;
                            let mask_vt = resolve_op(mask_raw, ops.get(next + 1).copied(), cur_vid, &val_types)?;
                            let mask_len = match mask_vt {
                                ValTy::Known(tid) => match types.get(tid as usize) {
                                    Some((TYPE_CODE_VECTOR, rops)) => rops.first().copied(),
                                    _ => None,
                                },
                                _ => None,
                            };
                            let out_vt = match (fp_shape_of_val(&types, vec_vt), mask_len) {
                                (Some(FpShape::DoubleVec(_)), Some(m)) => ValTy::Shape(FpShape::DoubleVec(m)),
                                (Some(FpShape::DoubleVec(_)), None) => return None,
                                (Some(FpShape::FloatVec(_)), Some(m)) => ValTy::Shape(FpShape::FloatVec(m)),
                                (Some(FpShape::HalfVec(_)), Some(m)) => ValTy::Shape(FpShape::HalfVec(m)),
                                _ => ValTy::Unknown,
                            };
                            val_types.push(out_vt);
                        }

                        FUNC_CODE_INST_PHI => {
                            let ty = *ops.first()?;
                            val_types.push(ValTy::Known(ty));
                        }

                        FUNC_CODE_INST_ALLOCA => {
                            if ops.len() != 4 {
                                return None;
                            }
                            let ty = ops[0];
                            let explicit_type = (ops[3] & (1u64 << 6)) != 0;
                            if explicit_type {
                                if is_double_tid(ty) {
                                    return None;
                                }
                                val_types.push(ValTy::PtrTo(ty));
                            } else {
                                if !types.get(ty as usize).is_some_and(|(c, _)| *c == TYPE_CODE_POINTER) {
                                    return None;
                                }
                                val_types.push(ValTy::Known(ty));
                            }
                        }

                        FUNC_CODE_INST_LOAD | FUNC_CODE_INST_LOADATOMIC => {
                            let raw0 = *ops.first()?;
                            let (ptr_vt, next) = if let Some(i) = back_idx(raw0, cur_vid) {
                                (*val_types.get(i)?, 1)
                            } else {
                                (ValTy::Known(*ops.get(1)?), 2)
                            };
                            // Explicit loaded type appears in LLVM 3.7 when `ops.len() == next + 3`
                            // (`[ptr, (ptr_ty), loaded_ty, align, vol]`) for LOAD or `next + 5` for LOADATOMIC.
                            let explicit_loaded = if code == FUNC_CODE_INST_LOAD && ops.len() == next + 3 {
                                Some(ops[next])
                            } else if code == FUNC_CODE_INST_LOADATOMIC && ops.len() == next + 5 {
                                Some(ops[next])
                            } else {
                                None
                            };
                            let loaded_vt = if let Some(lt) = explicit_loaded {
                                ValTy::Known(lt)
                            } else {
                                match ptr_vt {
                                    ValTy::PtrTo(t) => ValTy::Known(t),
                                    ValTy::Known(tid) => match types.get(tid as usize) {
                                        Some((TYPE_CODE_POINTER, rops)) => {
                                            rops.first().map_or(ValTy::Unknown, |&p| ValTy::Known(p))
                                        }
                                        _ => ValTy::Unknown,
                                    },
                                    _ => ValTy::Unknown,
                                }
                            };
                            val_types.push(loaded_vt);
                        }

                        FUNC_CODE_INST_EXTRACTVAL => {
                            let raw0 = *ops.first()?;
                            let (agg_vt, idx_start) = if let Some(i) = back_idx(raw0, cur_vid) {
                                (*val_types.get(i)?, 1)
                            } else {
                                (ValTy::Known(*ops.get(1)?), 2)
                            };
                            let mut cur = match agg_vt {
                                ValTy::Known(tid) => Some(tid),
                                _ => None,
                            };
                            for &field in ops.get(idx_start..).unwrap_or(&[]) {
                                let Some(tid) = cur else {
                                    break;
                                };
                                cur = match types.get(tid as usize) {
                                    Some((TYPE_CODE_STRUCT_ANON | TYPE_CODE_STRUCT_NAMED, rops)) => {
                                        rops.get(1 + field as usize).copied()
                                    }
                                    Some((TYPE_CODE_ARRAY, rops)) => rops.get(1).copied(),
                                    _ => None,
                                };
                            }
                            val_types.push(cur.map_or(ValTy::Unknown, ValTy::Known));
                        }

                        FUNC_CODE_INST_CALL => {
                            let ccinfo = *ops.get(1)?;
                            let fn_ty = if ccinfo & (1 << 15) != 0 {
                                *ops.get(2)?
                            } else {
                                let callee_raw = *ops.get(2)?;
                                match resolve_op(callee_raw, ops.get(3).copied(), cur_vid, &val_types)? {
                                    ValTy::Known(tid) => tid,
                                    _ => return None,
                                }
                            };
                            let (retty, _) = get_fn_sig(fn_ty)?;
                            if types.get(retty as usize)?.0 != TYPE_CODE_VOID {
                                val_types.push(ValTy::Known(retty));
                            }
                        }

                        FUNC_CODE_INST_CMP
                        | FUNC_CODE_INST_CMP2
                        | FUNC_CODE_INST_GEP_OLD
                        | FUNC_CODE_INST_INBOUNDS_GEP_OLD
                        | FUNC_CODE_INST_GEP => {
                            val_types.push(ValTy::Shape(FpShape::Other));
                        }

                        FUNC_CODE_INST_CMPXCHG_OLD
                        | FUNC_CODE_INST_ATOMICRMW
                        | FUNC_CODE_INST_LANDINGPAD_OLD
                        | FUNC_CODE_INST_CMPXCHG
                        | FUNC_CODE_INST_LANDINGPAD => {
                            val_types.push(ValTy::Unknown);
                        }

                        // Unknown or unsupported function record code (e.g. INVOKE, VAARG): bail safely.
                        _ => return None,
                    }
                    out_fitems.push(RewrittenFnItem::Record(code, ops));
                }
            }
        }
        rewritten_fns.insert(idx, (fw, out_fitems));
    }

    // Re-emit the complete module bitstream with TYPE_CODE_DOUBLE -> TYPE_CODE_FLOAT,
    // rewritten double constants, and rewritten double<->float casts.
    let mut w = W::default();
    w.out.extend_from_slice(b"BC\xC0\xDE");
    w.bit = 32;
    if let Some((bw, raw)) = top_blockinfo {
        w.copy_subblock(2, BLOCKINFO, bw, raw);
    }
    let m_at = w.enter(2, MODULE_BLOCK, mw);

    for (idx, it) in items.into_iter().enumerate() {
        match it {
            ModuleItem::Record(rc, rops) => w.unabbrev(mw, rc, &rops),
            ModuleItem::Subblock { id, w: sw, raw, .. } => match id {
                TYPE_BLOCK_NEW => {
                    let t_at = w.enter(mw, TYPE_BLOCK_NEW, sw);
                    for (trc, trops) in &type_records {
                        let out_code = if *trc == TYPE_CODE_DOUBLE { TYPE_CODE_FLOAT } else { *trc };
                        w.unabbrev(sw, out_code, trops);
                    }
                    w.end(sw, t_at);
                }
                CONSTANTS_BLOCK => {
                    let recs = rewritten_mod_consts.get(&idx)?;
                    let c_at = w.enter(mw, CONSTANTS_BLOCK, sw);
                    for (crc, crops) in recs {
                        w.unabbrev(sw, *crc, crops);
                    }
                    w.end(sw, c_at);
                }
                FUNCTION_BLOCK => {
                    let (fw, fitems) = rewritten_fns.remove(&idx)?;
                    let f_at = w.enter(mw, FUNCTION_BLOCK, fw);
                    for fi in fitems {
                        match fi {
                            RewrittenFnItem::Record(frc, fops) => w.unabbrev(fw, frc, &fops),
                            RewrittenFnItem::Constants(cw, crecs) => {
                                let c_at = w.enter(fw, CONSTANTS_BLOCK, cw);
                                for (crc, crops) in &crecs {
                                    w.unabbrev(cw, *crc, crops);
                                }
                                w.end(cw, c_at);
                            }
                            RewrittenFnItem::RawSubblock { id: sid, w: ssw, raw: sraw } => {
                                w.copy_subblock(fw, sid, ssw, sraw);
                            }
                        }
                    }
                    w.end(fw, f_at);
                }
                _ => w.copy_subblock(mw, id, sw, raw),
            },
        }
    }

    w.end(mw, m_at);
    Some(w.out)
}

/// Returns `true` when `dxil_container` is eligible for FP64-to-FP32 bitcode lowering (used to
/// salt the Tier-1 shader cache key only for shaders affected by FP64 lowering, avoiding any
/// cache invalidation for existing non-FP64 shaders).
pub fn needs_fp64_lowering(dxil_container: &[u8]) -> bool {
    let Ok(container) = crate::dxbc::Container::parse(dxil_container) else {
        return false;
    };
    if container.is_sm4_bytecode() {
        return (container.feature_flags() & SFI0_DOUBLES_MASK) != 0;
    }
    let Some(dxil_part) = container.part(crate::dxbc::FOURCC_DXIL) else {
        return false;
    };
    let Some(bc) = dxil_part_bitcode(dxil_part) else {
        return false;
    };
    if !bitcode_has_double_type(bc) {
        return false;
    }
    let sm69 = lower_sm69_bitcode(bc);
    let cur_bc = sm69.as_deref().unwrap_or(bc);
    lower_fp64_bitcode(cur_bc).is_some()
}

/// If `dxil_container` is a DXBC container whose `DXIL` part uses unsupported SM 6.9 `dx.op.*`
/// intrinsics or pure in-register FP64 (`double`) SSA operations, returns a rewritten container
/// with those lowered for `libmetalirconverter.dylib` / Apple Silicon Metal. Returns `None` when
/// no lowering is needed.
pub fn lower_dxil_container(dxil_container: &[u8]) -> Option<Vec<u8>> {
    let container = crate::dxbc::Container::parse(dxil_container).ok()?;
    let dxil_part = container.part(crate::dxbc::FOURCC_DXIL)?;
    let bc = dxil_part_bitcode(dxil_part)?;
    let sm69 = lower_sm69_bitcode(bc);
    let cur_bc = sm69.as_deref().unwrap_or(bc);
    let fp64 = lower_fp64_bitcode(cur_bc);
    let fp64_lowered = fp64.is_some();
    let mut new_bc = fp64.or(sm69)?;
    while !new_bc.len().is_multiple_of(4) {
        new_bc.push(0);
    }
    if dxil_part.len() < 24 {
        return None;
    }
    let mut new_dxil_part = Vec::with_capacity(24 + new_bc.len());
    new_dxil_part.extend_from_slice(&dxil_part[0..4]); // program version
    new_dxil_part.extend_from_slice(&(((24 + new_bc.len()) / 4) as u32).to_le_bytes());
    new_dxil_part.extend_from_slice(b"DXIL");
    new_dxil_part.extend_from_slice(&dxil_part[12..16]); // dxil version
    new_dxil_part.extend_from_slice(&16u32.to_le_bytes()); // bitcode offset from magic
    new_dxil_part.extend_from_slice(&(new_bc.len() as u32).to_le_bytes());
    new_dxil_part.extend_from_slice(&new_bc);

    let mut new_sfi0: Option<Vec<u8>> = None;
    if fp64_lowered {
        if let Some(sfi0) = container.part(crate::dxbc::FOURCC_SFI0) {
            if sfi0.len() >= 8 {
                let mut copy = sfi0.to_vec();
                let flags = u64::from_le_bytes(copy[0..8].try_into().unwrap()) & !SFI0_DOUBLES_MASK;
                copy[0..8].copy_from_slice(&flags.to_le_bytes());
                new_sfi0 = Some(copy);
            }
        }
    }

    let parts: Vec<([u8; 4], &[u8])> = container
        .parts
        .iter()
        .map(|p| {
            if p.fourcc == crate::dxbc::FOURCC_DXIL {
                (p.fourcc, new_dxil_part.as_slice())
            } else if p.fourcc == crate::dxbc::FOURCC_SFI0 {
                (p.fourcc, new_sfi0.as_deref().unwrap_or(p.data))
            } else {
                (p.fourcc, p.data)
            }
        })
        .collect();
    Some(crate::dxbc::build_container(&parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A module with: blockinfo (VST char6 abbrev), a declared function, a defined function,
    /// a skipped function body block, and a VST naming both.
    fn module(defined_name: &str) -> Vec<u8> {
        let mut w = W::default();
        w.out.extend_from_slice(b"BC\xC0\xDE");
        w.bit = 32;
        // BLOCKINFO: SETBID 14, DEFINE_ABBREV [Fixed(3)=1 literal? -> literal 1, vbr8, array char6]
        let bi = w.enter(2, BLOCKINFO, 3);
        w.unabbrev(3, BLOCKINFO_CODE_SETBID, &[VALUE_SYMTAB_BLOCK]);
        w.put(2, 3); // DEFINE_ABBREV
        w.vbr(4, 5);
        w.put(1, 1);
        w.vbr(VST_CODE_ENTRY, 8); // literal code
        w.put(0, 1);
        w.put(2, 3);
        w.vbr(8, 5); // vbr8 value id
        w.put(0, 1);
        w.put(3, 3); // array
        w.put(0, 1);
        w.put(4, 3); // char6
        w.end(3, bi);

        let m = w.enter(2, MODULE_BLOCK, 3);
        w.unabbrev(3, 1, &[1]); // VERSION
        w.unabbrev(3, MODULE_CODE_GLOBALVAR, &[0, 0, 0, 0]); // value 0
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[5, 0, 1, 0, 0, 0]); // value 1: declaration
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[6, 0, 0, 0, 0, 0]); // value 2: definition
        // A function body block full of junk words, skipped by length.
        let f = w.enter(3, 12, 4);
        w.unabbrev(4, 1, &[7, 7, 7]);
        w.end(4, f);
        // Module VST: value 1 unabbreviated (8-bit chars), value 2 with the char6 abbrev (id 4).
        let v = w.enter(3, VALUE_SYMTAB_BLOCK, 4);
        let decl: Vec<u64> = std::iter::once(1).chain(b"dx.op.loadInput.f32".iter().map(|&c| c as u64)).collect();
        w.unabbrev(4, VST_CODE_ENTRY, &decl);
        w.put(4, 4);
        w.vbr(2, 8);
        w.vbr(defined_name.len() as u64, 6);
        for c in defined_name.bytes() {
            let v = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._".iter().position(|&x| x == c).unwrap();
            w.put(v as u64, 6);
        }
        w.end(4, v);
        w.end(3, m);
        w.out
    }

    /// A module containing a `METADATA_BLOCK` with `!dx.entryPoints = !{!2}` where
    /// `!2 = !{null, !"HS_NullCP", null, null, null}` (mirroring a pass-through Hull shader
    /// whose control-point function pointer is null and whose only defined function is `pc_main`).
    fn module_with_dx_entrypoints(entry_name: &str) -> Vec<u8> {
        let mut w = W::default();
        w.out.extend_from_slice(b"BC\xC0\xDE");
        w.bit = 32;
        let m = w.enter(2, MODULE_BLOCK, 3);
        w.unabbrev(3, 1, &[1]); // VERSION
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[6, 0, 0, 0, 0, 0]); // value 0: pc_main
        let md = w.enter(3, METADATA_BLOCK, 4);
        // md 0: METADATA_STRING "other"
        w.unabbrev(4, METADATA_STRING, &b"other".iter().map(|&c| c as u64).collect::<Vec<_>>());
        // METADATA_KIND does NOT allocate a metadata ID.
        let mut kind_ops = vec![0u64];
        kind_ops.extend(b"dbg".iter().map(|&c| c as u64));
        w.unabbrev(4, METADATA_KIND, &kind_ops);
        // md 1: METADATA_STRING entry_name
        w.unabbrev(4, METADATA_STRING, &entry_name.bytes().map(|c| c as u64).collect::<Vec<_>>());
        // md 2: METADATA_NODE [0 (null fn), 2 (1-based -> md 1), 0, 0, 0]
        w.unabbrev(4, METADATA_NODE, &[0, 2, 0, 0, 0]);
        // Named metadata !dx.entryPoints = !{!2} (0-based -> md 2)
        w.unabbrev(4, METADATA_NAME, &b"dx.entryPoints".iter().map(|&c| c as u64).collect::<Vec<_>>());
        w.unabbrev(4, METADATA_NAMED_NODE, &[2]);
        w.end(4, md);
        w.end(3, m);
        w.out
    }

    /// Builds a minimal LLVM 3.7 bitcode module exercising pure in-register FP64 (`double`)
    /// operations (constants, `phi`, `vselect`, `fmul`, `fptrunc double->float`,
    /// `fpext float->double`, `fpext half->double`, `fptrunc double->half`, `extractvalue`
    /// of `float` from a struct, and a `void` call before the casts).
    fn fp64_module(include_illegal_bitcast: bool, include_double_ptr: bool) -> Vec<u8> {
        let mut w = W::default();
        w.out.extend_from_slice(b"BC\xC0\xDE");
        w.bit = 32;
        let m = w.enter(2, MODULE_BLOCK, 3);
        w.unabbrev(3, MODULE_CODE_VERSION, &[1]);

        // TYPE_BLOCK_NEW:
        //   0: void
        //   1: half
        //   2: float
        //   3: double
        //   4: i64 (int 64)
        //   5: i1 (int 1)
        //   6: struct { float, float }
        //   7: fn void ()
        //   8: fn struct { float, float } ()
        //   9: <2 x double>
        //  10: <2 x float>
        //  11 (optional): pointer to double
        let t = w.enter(3, TYPE_BLOCK_NEW, 4);
        w.unabbrev(4, TYPE_CODE_NUMENTRY, &[if include_double_ptr { 12 } else { 11 }]);
        w.unabbrev(4, TYPE_CODE_VOID, &[]); // 0
        w.unabbrev(4, TYPE_CODE_HALF, &[]); // 1
        w.unabbrev(4, TYPE_CODE_FLOAT, &[]); // 2
        w.unabbrev(4, TYPE_CODE_DOUBLE, &[]); // 3
        w.unabbrev(4, 7, &[64]); // 4: i64
        w.unabbrev(4, 7, &[1]); // 5: i1
        w.unabbrev(4, TYPE_CODE_STRUCT_ANON, &[0, 2, 2]); // 6: { float, float }
        w.unabbrev(4, TYPE_CODE_FUNCTION, &[0, 0]); // 7: void ()
        w.unabbrev(4, TYPE_CODE_FUNCTION, &[0, 6]); // 8: { float, float } ()
        w.unabbrev(4, TYPE_CODE_VECTOR, &[2, 3]); // 9: <2 x double>
        w.unabbrev(4, TYPE_CODE_VECTOR, &[2, 2]); // 10: <2 x float>
        if include_double_ptr {
            w.unabbrev(4, TYPE_CODE_POINTER, &[3, 0]); // 11: double*
        }
        w.end(4, t);

        // Module values:
        //   0: declared void fn (ty 7)
        //   1: declared struct-returning fn (ty 8)
        //   2: defined entry fn void () (ty 7)
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[7, 0, 1, 0, 0, 0]);
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[8, 0, 1, 0, 0, 0]);
        w.unabbrev(3, MODULE_CODE_FUNCTION, &[7, 0, 0, 0, 0, 0]);

        // Module-level CONSTANTS_BLOCK:
        //   3: double 38.0
        //   4: double -0.05 (exercises bit 63 set in VBR6)
        //   5: <2 x double> data [1.0, -2.5]
        //   6: i1 true (const int 1)
        //   7: half 0.0 (null)
        let mc = w.enter(3, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[3]);
        w.unabbrev(4, CST_CODE_FLOAT, &[38.0f64.to_bits()]); // 3
        w.unabbrev(4, CST_CODE_FLOAT, &[(-0.05f64).to_bits()]); // 4
        w.unabbrev(4, CST_CODE_SETTYPE, &[9]);
        w.unabbrev(4, CST_CODE_DATA, &[1.0f64.to_bits(), (-2.5f64).to_bits()]); // 5
        w.unabbrev(4, CST_CODE_SETTYPE, &[5]);
        w.unabbrev(4, CST_CODE_INTEGER, &[1 << 1]); // 6
        w.unabbrev(4, CST_CODE_SETTYPE, &[1]);
        w.unabbrev(4, CST_CODE_NULL, &[]); // 7
        w.end(4, mc);

        // FUNCTION_BLOCK for entry fn (val_types starts at 8):
        let f = w.enter(3, FUNCTION_BLOCK, 4);
        w.unabbrev(4, FUNC_CODE_DECLAREBLOCKS, &[1]);
        // Function-level CONSTANTS_BLOCK:
        //   8: double 32.0
        let fc = w.enter(4, CONSTANTS_BLOCK, 4);
        w.unabbrev(4, CST_CODE_SETTYPE, &[3]);
        w.unabbrev(4, CST_CODE_FLOAT, &[32.0f64.to_bits()]); // 8
        w.end(4, fc);
        // void call to fn 0 (must NOT increment cur_vid = 9!)
        w.unabbrev(4, FUNC_CODE_INST_CALL, &[0, 0, 9]);
        // call to fn 1 -> returns { float, float } (vid = 9)
        w.unabbrev(4, FUNC_CODE_INST_CALL, &[0, 0, 8]);
        // extractvalue { float, float } %9, 0 -> float (vid = 10)
        w.unabbrev(4, FUNC_CODE_INST_EXTRACTVAL, &[1, 0]);
        // fpext float %10 to double -> MUST become CAST_BITCAST (vid = 11)
        w.unabbrev(4, FUNC_CODE_INST_CAST, &[1, 3, CAST_FPEXT]);
        // fpext half %7 to double -> MUST stay CAST_FPEXT (vid = 12)
        w.unabbrev(4, FUNC_CODE_INST_CAST, &[5, 3, CAST_FPEXT]);
        // vselect i1 %6, double %3 (38.0), double %8 (32.0) (vid = 13)
        w.unabbrev(4, FUNC_CODE_INST_VSELECT, &[10, 5, 7]);
        // fmul double %13, %11 (vid = 14)
        w.unabbrev(4, FUNC_CODE_INST_BINOP, &[1, 3, 2, 31]);
        // fptrunc double %14 to float -> MUST become CAST_BITCAST (vid = 15)
        w.unabbrev(4, FUNC_CODE_INST_CAST, &[1, 2, CAST_FPTRUNC]);
        // fptrunc double %14 to half -> MUST stay CAST_FPTRUNC (vid = 16)
        w.unabbrev(4, FUNC_CODE_INST_CAST, &[2, 1, CAST_FPTRUNC]);
        // fptrunc <2 x double> %5 to <2 x float> -> MUST become CAST_BITCAST (vid = 17)
        w.unabbrev(4, FUNC_CODE_INST_CAST, &[12, 10, CAST_FPTRUNC]);
        if include_illegal_bitcast {
            // bitcast double %14 to i64 -> MUST reject lowering!
            w.unabbrev(4, FUNC_CODE_INST_CAST, &[4, 4, CAST_BITCAST]);
        }
        w.unabbrev(4, FUNC_CODE_INST_RET, &[]);
        w.end(4, f);

        w.end(3, m);
        w.out
    }

    #[test]
    fn finds_the_defined_function() {
        let bc = module("FullscreenVS_main2");
        assert_eq!(defined_functions(&bc).unwrap(), ["FullscreenVS_main2"]);
        assert_eq!(entry_points(&bc), None);
        assert_eq!(lower_sm69_bitcode(&bc), None);
        assert_eq!(lower_fp64_bitcode(&bc), None);
        assert!(defined_functions(b"BC\xC0\xDE").is_none());
        assert!(defined_functions(&bc[..bc.len() - 9]).is_none());
        assert!(defined_functions(b"nope").is_none());
    }

    #[test]
    fn finds_dx_entrypoints_metadata_with_null_fn_ptr() {
        let bc = module_with_dx_entrypoints("HS_NullCP");
        assert_eq!(entry_points(&bc).unwrap(), ["HS_NullCP"]);
        let empty = module_with_dx_entrypoints("");
        assert_eq!(entry_points(&empty), None);
    }

    #[test]
    fn lowers_in_register_fp64_to_fp32_and_preserves_half_casts() {
        let bc = fp64_module(false, false);
        assert!(bitcode_has_double_type(&bc));
        let lowered = lower_fp64_bitcode(&bc).expect("expected pure in-register fp64 module to lower");
        assert!(!bitcode_has_double_type(&lowered), "lowered bitcode must have zero TYPE_CODE_DOUBLE");
        // Idempotent: lowering again returns None.
        assert_eq!(lower_fp64_bitcode(&lowered), None);

        // Parse the lowered bitcode and verify rewritten float constants + cast opcodes.
        let mut r = Reader { b: Bits { data: &lowered, pos: 32 }, blockinfo: HashMap::new() };
        let mut top = Vec::new();
        assert!(matches!(r.next(2, &mut top), Some(Entry::Block(MODULE_BLOCK))));
        let (mw, mend) = r.block_header().unwrap();
        let mut mab = Vec::new();
        let mut float_consts = Vec::new();
        let mut data_consts = Vec::new();
        let mut cast_opcs = Vec::new();
        while r.b.pos < mend {
            match r.next(mw, &mut mab).unwrap() {
                Entry::End => break,
                Entry::Block(CONSTANTS_BLOCK) => {
                    let (cw, cend) = r.block_header().unwrap();
                    let recs = parse_subblock_records_no_blob(&lowered, r.b.pos, cend, cw, &[]).unwrap();
                    r.b.pos = cend;
                    for (c, ops) in recs {
                        if c == CST_CODE_FLOAT {
                            float_consts.push(ops[0]);
                        } else if c == CST_CODE_DATA {
                            data_consts.extend(ops);
                        }
                    }
                }
                Entry::Block(FUNCTION_BLOCK) => {
                    let (fw, fend) = r.block_header().unwrap();
                    let fitems = parse_subblock_items(&lowered, r.b.pos, fend, fw, &[]).unwrap();
                    r.b.pos = fend;
                    for fi in fitems {
                        match fi {
                            SubblockItem::Subblock { id: CONSTANTS_BLOCK, w: cw, bit_start, bit_end, .. } => {
                                let recs = parse_subblock_records_no_blob(&lowered, bit_start, bit_end, cw, &[]).unwrap();
                                for (c, ops) in recs {
                                    if c == CST_CODE_FLOAT {
                                        float_consts.push(ops[0]);
                                    }
                                }
                            }
                            SubblockItem::Record(FUNC_CODE_INST_CAST, ops) => {
                                cast_opcs.push(*ops.last().unwrap());
                            }
                            _ => {}
                        }
                    }
                }
                Entry::Block(_) => r.skip_block().unwrap(),
                Entry::Record(..) => {}
            }
        }
        assert_eq!(
            float_consts,
            vec![38.0f32.to_bits() as u64, (-0.05f32).to_bits() as u64, 32.0f32.to_bits() as u64]
        );
        assert_eq!(data_consts, vec![1.0f32.to_bits() as u64, (-2.5f32).to_bits() as u64]);
        // 5 casts in order:
        //   fpext float->double   => BITCAST
        //   fpext half->double    => FPEXT
        //   fptrunc double->float => BITCAST
        //   fptrunc double->half  => FPTRUNC
        //   fptrunc <2xdbl>-><2xflt> => BITCAST
        assert_eq!(cast_opcs, vec![CAST_BITCAST, CAST_FPEXT, CAST_BITCAST, CAST_FPTRUNC, CAST_BITCAST]);
    }

    #[test]
    fn rejects_fp64_lowering_for_bitcast_and_memory_abi_types() {
        let with_bitcast = fp64_module(true, false);
        assert_eq!(lower_fp64_bitcode(&with_bitcast), None, "bitcast double->i64 must not be lowered");
        let with_ptr = fp64_module(false, true);
        assert_eq!(lower_fp64_bitcode(&with_ptr), None, "double* pointer must not be lowered");
    }

    #[test]
    fn lower_dxil_container_clears_sfi0_doubles_and_salts_cache_key() {
        let mut bc = fp64_module(false, false);
        while !bc.len().is_multiple_of(4) {
            bc.push(0);
        }
        let mut dxil_part = Vec::with_capacity(24 + bc.len());
        dxil_part.extend_from_slice(&0x0005_0066u32.to_le_bytes()); // cs_6_6
        dxil_part.extend_from_slice(&(((24 + bc.len()) / 4) as u32).to_le_bytes());
        dxil_part.extend_from_slice(b"DXIL");
        dxil_part.extend_from_slice(&0x0001_0006u32.to_le_bytes());
        dxil_part.extend_from_slice(&16u32.to_le_bytes());
        dxil_part.extend_from_slice(&(bc.len() as u32).to_le_bytes());
        dxil_part.extend_from_slice(&bc);
        // SFI0 with Doubles (bit 0) + EarlyDepthStencil (bit 1) + 11_1_DoubleExtensions (bit 5).
        let sfi0 = ((1u64 << 0) | (1u64 << 1) | (1u64 << 5)).to_le_bytes();
        let container = crate::dxbc::build_container(&[
            (crate::dxbc::FOURCC_SFI0, &sfi0),
            (crate::dxbc::FOURCC_DXIL, &dxil_part),
        ]);
        assert!(needs_fp64_lowering(&container));
        let lowered = lower_dxil_container(&container).expect("container should lower");
        assert!(!needs_fp64_lowering(&lowered));
        let parsed = crate::dxbc::Container::parse(&lowered).unwrap();
        assert_eq!(parsed.feature_flags(), 1u64 << 1, "only double feature bits should be cleared");
    }
}
