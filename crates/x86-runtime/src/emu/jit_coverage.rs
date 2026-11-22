//! Offline translator coverage report (ignored CLI diagnostic + unit tests).
//!
//! ```text
//! AW2_JIT_COV_DLL=/path/cohtml.WindowsDesktop.dll AW2_JIT_COV_RVAS=0x20d10,0x4261c0 \
//!   cargo test -p x86-runtime --release --lib jit_coverage -- --ignored --nocapture
//! ```
//! Walks the control flow (fall-through + direct jcc/jmp targets + direct calls into
//! `.pdata`-less leaf callees + `LEA reg, [rip + disp32]` `.pdata`-less leaf targets, up to
//! `PORT_AOT_CLOSURE` / `AW2_DLL_AOT_CLOSURE` depth, default 128) from each RVA in
//! `AW2_JIT_COV_RVAS` (default: `.pdata` roots + entry point + exports + TLS callbacks +
//! Load Config CFG function tables + PE Base Relocation `IMAGE_REL_BASED_DIR64` targets) and
//! reports the instructions that end a translated block: `K_GENERIC` (no fast op) and fast ops
//! the translator doesn't support. Counts are static (each instruction once).

use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

const DEFAULT_COV_CLOSURE: u32 = 128;
const MAX_TLS_CALLBACKS: usize = 1024;
const MAX_CFG_ENTRIES: usize = 4_000_000;
const MAX_RELOC_ENTRIES: usize = 4_000_000;

fn reloc_roots_enabled() -> bool {
    std::env::var("PORT_AOT_RELOC_ROOTS")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(1)
        != 0
}

fn u16le(b: &[u8], o: usize) -> Option<u16> {
    let s = b.get(o..o.checked_add(2)?)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}
fn u32le(b: &[u8], o: usize) -> Option<u32> {
    let s = b.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}
fn u64le(b: &[u8], o: usize) -> Option<u64> {
    let s = b.get(o..o.checked_add(8)?)?;
    Some(u64::from_le_bytes(s.try_into().ok()?))
}

struct Pe {
    data: Vec<u8>,
    base: u64,
    size_of_image: u32,
    entry_rva: u32,
    /// `(rva, vsize, raw_off, raw_size, characteristics)`
    secs: Vec<(u32, u32, u32, u32, u32)>,
    /// Sorted `(begin_rva, end_rva)` from `.pdata`.
    pdata: Vec<(u32, u32)>,
    data_dirs: [(u32, u32); 16],
}

impl Pe {
    const CNT_CODE: u32 = 0x20;
    const CNT_INIT_DATA: u32 = 0x40;
    const MEM_DISCARDABLE: u32 = 0x0200_0000;
    const MEM_EXECUTE: u32 = 0x2000_0000;

    fn load(path: &str) -> Pe {
        let data = std::fs::read(path).expect("read dll");
        Pe::from_bytes(data)
    }

    fn from_bytes(data: Vec<u8>) -> Pe {
        let nt = u32le(&data, 0x3c).expect("e_lfanew") as usize;
        let opt = nt + 24;
        let base = u64le(&data, opt + 24).expect("ImageBase");
        let entry_rva = u32le(&data, opt + 16).unwrap_or(0);
        let size_of_image = u32le(&data, opt + 56).unwrap_or(0);
        let nsec = u16le(&data, nt + 6).expect("NumberOfSections") as usize;
        let opt_sz = u16le(&data, nt + 20).expect("SizeOfOptionalHeader") as usize;
        let nrva = u32le(&data, opt + 108).unwrap_or(0).min(16) as usize;
        let mut data_dirs = [(0u32, 0u32); 16];
        for (i, dd) in data_dirs.iter_mut().enumerate().take(nrva) {
            if let (Some(rva), Some(sz)) = (u32le(&data, opt + 112 + i * 8), u32le(&data, opt + 112 + i * 8 + 4)) {
                *dd = (rva, sz);
            }
        }
        let sec0 = opt + opt_sz;
        let mut secs = Vec::with_capacity(nsec);
        for i in 0..nsec {
            let s = sec0 + i * 40;
            if let (Some(vs), Some(va), Some(rs), Some(ro), Some(ch)) = (
                u32le(&data, s + 8),
                u32le(&data, s + 12),
                u32le(&data, s + 16),
                u32le(&data, s + 20),
                u32le(&data, s + 36),
            ) {
                secs.push((va, vs, ro, rs, ch));
            }
        }
        let mut pe = Pe { data, base, size_of_image, entry_rva, secs, pdata: Vec::new(), data_dirs };
        let (exc_rva, exc_sz) = pe.data_dirs[3];
        if exc_rva != 0 && exc_sz >= 12 {
            if let Some(o) = pe.off(exc_rva) {
                for i in 0..(exc_sz as usize / 12) {
                    let e = o + i * 12;
                    if let (Some(b), Some(end)) = (u32le(&pe.data, e), u32le(&pe.data, e + 4)) {
                        if b < end {
                            pe.pdata.push((b, end));
                        }
                    }
                }
            }
        }
        pe.pdata.sort_unstable();
        pe
    }

    fn off(&self, rva: u32) -> Option<usize> {
        for &(va, vs, ro, rs, _) in &self.secs {
            if rva >= va && rva < va.saturating_add(vs.max(rs)) {
                let d = rva - va;
                if d < rs {
                    let pos = (ro as usize).checked_add(d as usize)?;
                    return (pos < self.data.len()).then_some(pos);
                }
                return None;
            }
        }
        None
    }

    fn is_exec(&self, rva: u32) -> bool {
        self.secs.iter().any(|&(va, vs, _, rs, ch)| {
            (ch & (Self::MEM_EXECUTE | Self::CNT_CODE)) != 0
                && rva >= va
                && rva < va.saturating_add(vs.max(rs))
                && (rva - va) < rs
        })
    }

    fn exec_slice(&self, rva: u32) -> Option<&[u8]> {
        for &(va, vs, ro, rs, ch) in &self.secs {
            if (ch & (Self::MEM_EXECUTE | Self::CNT_CODE)) != 0
                && rva >= va
                && rva < va.saturating_add(vs.max(rs))
            {
                let d = rva - va;
                if d < rs {
                    let pos = (ro as usize).checked_add(d as usize)?;
                    let sec_end = (ro as usize).checked_add(rs as usize)?.min(self.data.len());
                    if pos < sec_end {
                        return Some(&self.data[pos..sec_end]);
                    }
                }
                return None;
            }
        }
        None
    }

    fn is_plausible_code_entry(&self, rva: u32) -> bool {
        let Some(slice) = self.exec_slice(rva) else {
            return false;
        };
        if slice.is_empty() || slice[0] == 0xCC {
            return false;
        }
        if slice[0] == 0x00 && slice.len() >= 2 && slice[1] == 0x00 {
            return false;
        }
        let avail = slice.len().min(15);
        let mut dec = iced_x86::Decoder::with_ip(
            64,
            &slice[..avail],
            self.base.wrapping_add(rva as u64),
            0,
        );
        !dec.decode().is_invalid()
    }

    fn is_clean_code_entry(&self, rva: u32) -> bool {
        let Some(&(va, _vs, ro, _rs, _ch)) = self.secs.iter().find(|&&(va, vs, _, rs, ch)| {
            (ch & (Self::MEM_EXECUTE | Self::CNT_CODE)) != 0
                && rva >= va
                && rva < va.saturating_add(vs.max(rs))
                && (rva - va) < rs
        }) else {
            return false;
        };
        let clean_boundary = ((rva - va) & 0xF) == 0
            || (rva > va
                && (ro as usize)
                    .checked_add((rva - va - 1) as usize)
                    .and_then(|pos| self.data.get(pos))
                    .is_some_and(|&b| matches!(b, 0xCC | 0x90 | 0xC3)));
        clean_boundary && self.is_plausible_code_entry(rva)
    }

    fn in_pdata(&self, rva: u32) -> bool {
        let i = self.pdata.partition_point(|&(b, _)| b <= rva);
        i.checked_sub(1).and_then(|j| self.pdata.get(j)).is_some_and(|&(_, e)| rva < e)
    }

    fn va_to_rva(&self, va: u64) -> Option<u32> {
        let rva = va.checked_sub(self.base)?;
        if self.size_of_image != 0 && rva >= self.size_of_image as u64 {
            return None;
        }
        u32::try_from(rva).ok()
    }

    fn export_rvas(&self) -> Vec<u32> {
        let mut out = Vec::new();
        let (dir_rva, dir_sz) = self.data_dirs[0];
        if dir_rva == 0 || dir_sz < 40 {
            return out;
        }
        let Some(doff) = self.off(dir_rva) else { return out };
        let Some(n_funcs) = u32le(&self.data, doff + 20) else { return out };
        let Some(funcs_rva) = u32le(&self.data, doff + 28) else { return out };
        let dir_end = dir_rva.saturating_add(dir_sz);
        for i in 0..n_funcs {
            let Some(foff) = self.off(funcs_rva.saturating_add(i.saturating_mul(4))) else { break };
            let Some(rva) = u32le(&self.data, foff) else { break };
            if rva != 0 && !(rva >= dir_rva && rva < dir_end) {
                out.push(rva);
            }
        }
        out
    }

    fn tls_callback_rvas(&self) -> Vec<u32> {
        let mut out = Vec::new();
        let (dir_rva, dir_sz) = self.data_dirs[9];
        if dir_rva == 0 || dir_sz < 32 {
            return out;
        }
        let Some(doff) = self.off(dir_rva) else { return out };
        let Some(cb_va) = u64le(&self.data, doff + 24) else { return out };
        let Some(cb_rva) = self.va_to_rva(cb_va) else { return out };
        for i in 0..MAX_TLS_CALLBACKS {
            let Some(off) = self.off(cb_rva.saturating_add((i * 8) as u32)) else { break };
            let Some(va) = u64le(&self.data, off) else { break };
            if va == 0 {
                break;
            }
            let Some(rva) = self.va_to_rva(va) else { break };
            out.push(rva);
        }
        out
    }

    fn load_config_cfg_rvas(&self) -> Vec<u32> {
        const GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 0x0000_0400;
        const GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 0x0001_0000;
        let mut out = Vec::new();
        let (dir_rva, _) = self.data_dirs[10];
        if dir_rva == 0 {
            return out;
        }
        let Some(doff) = self.off(dir_rva) else { return out };
        let Some(lc_size) = u32le(&self.data, doff) else { return out };
        if lc_size < 0x94 {
            return out;
        }
        let guard_flags = u32le(&self.data, doff + 0x90).unwrap_or(0);
        if (guard_flags & GUARD_CF_FUNCTION_TABLE_PRESENT) != 0 {
            let tbl_va = u64le(&self.data, doff + 0x80).unwrap_or(0);
            let count = u64le(&self.data, doff + 0x88).unwrap_or(0);
            let extra = ((guard_flags & 0xF000_0000) >> 28) as usize;
            let stride = 4 + extra;
            if let Some(tbl_rva) = self.va_to_rva(tbl_va) {
                let n = (count as usize).min(MAX_CFG_ENTRIES);
                for i in 0..n {
                    let Some(delta) = u32::try_from(i.saturating_mul(stride)).ok() else { break };
                    let Some(off) = self.off(tbl_rva.saturating_add(delta)) else { break };
                    let Some(rva) = u32le(&self.data, off) else { break };
                    if rva != 0 {
                        out.push(rva);
                    }
                }
            }
        }
        if lc_size >= 0xC0 && (guard_flags & GUARD_CF_LONGJUMP_TABLE_PRESENT) != 0 {
            let tbl_va = u64le(&self.data, doff + 0xB0).unwrap_or(0);
            let count = u64le(&self.data, doff + 0xB8).unwrap_or(0);
            if let Some(tbl_rva) = self.va_to_rva(tbl_va) {
                let n = (count as usize).min(MAX_CFG_ENTRIES);
                for i in 0..n {
                    let Some(delta) = u32::try_from(i.saturating_mul(4)).ok() else { break };
                    let Some(off) = self.off(tbl_rva.saturating_add(delta)) else { break };
                    let Some(rva) = u32le(&self.data, off) else { break };
                    if rva != 0 {
                        out.push(rva);
                    }
                }
            }
        }
        out
    }

    fn base_reloc_code_rvas(&self) -> Vec<u32> {
        const IMAGE_REL_BASED_DIR64: u16 = 10;
        let mut out = Vec::new();
        let (dir_rva, dir_len) = self.data_dirs[5];
        if dir_rva != 0 && dir_len != 0 {
            let Some(dir_end) = (dir_rva as u64).checked_add(dir_len as u64) else {
                return out;
            };
            let mut cur_rva = dir_rva as u64;
            while cur_rva.checked_add(8).is_some_and(|e| e <= dir_end) {
                let Ok(cur_rva32) = u32::try_from(cur_rva) else { break };
                let Some(boff) = self.off(cur_rva32) else { break };
                let Some(page_rva) = u32le(&self.data, boff) else { break };
                let Some(block_size) = u32le(&self.data, boff + 4) else { break };
                if block_size < 8 {
                    break;
                }
                let Some(next_rva) = cur_rva.checked_add(block_size as u64) else { break };
                if next_rva > dir_end {
                    break;
                }
                let n_entries = ((block_size - 8) / 2) as usize;
                for i in 0..n_entries {
                    let Some(entry) = u16le(&self.data, boff + 8 + i * 2) else { break };
                    let ty = entry >> 12;
                    if ty != IMAGE_REL_BASED_DIR64 {
                        continue;
                    }
                    let rel_off = (entry & 0x0FFF) as u32;
                    let Some(slot_rva) = page_rva.checked_add(rel_off) else { continue };
                    let Some(slot_off) = self.off(slot_rva) else { continue };
                    if slot_rva.checked_add(7).and_then(|e| self.off(e)).is_none() {
                        continue;
                    }
                    let Some(va) = u64le(&self.data, slot_off) else { continue };
                    let Some(target_rva) = self.va_to_rva(va) else { continue };
                    if self.is_plausible_code_entry(target_rva) {
                        out.push(target_rva);
                        if out.len() >= MAX_RELOC_ENTRIES {
                            return out;
                        }
                    }
                }
                cur_rva = next_rva;
            }
            return out;
        }

        // Fallback when `DataDirectory[5]` is absent/empty (stripped `.reloc`): scan 8-byte-aligned
        // slots in non-executable initialized data sections.
        if self.base == 0 {
            return out;
        }
        for &(va, vs, ro, rs, ch) in &self.secs {
            if (ch & (Self::MEM_EXECUTE | Self::CNT_CODE | Self::MEM_DISCARDABLE)) != 0
                || (ch & Self::CNT_INIT_DATA) == 0
                || vs == 0
                || rs == 0
            {
                continue;
            }
            let sec_len = vs.min(rs);
            let start_rva = va.max(0x400);
            let end_rva = va.saturating_add(sec_len);
            if start_rva >= end_rva {
                continue;
            }
            let Some(mut rva) = start_rva.checked_add(7).map(|x| x & !7) else { continue };
            while rva.checked_add(8).is_some_and(|e| e <= end_rva) {
                let Some(off) = (ro as usize).checked_add((rva - va) as usize) else { break };
                let Some(ptr_va) = u64le(&self.data, off) else { break };
                if let Some(target_rva) = self.va_to_rva(ptr_va) {
                    if self.is_clean_code_entry(target_rva) {
                        out.push(target_rva);
                        if out.len() >= MAX_RELOC_ENTRIES {
                            return out;
                        }
                    }
                }
                let Some(next) = rva.checked_add(8) else { break };
                rva = next;
            }
        }
        out
    }

    fn default_roots(&self) -> Vec<u32> {
        let mut roots = Vec::new();
        for &(b, _) in &self.pdata {
            if self.is_exec(b) {
                roots.push(b);
            }
        }
        if self.entry_rva != 0 && self.is_exec(self.entry_rva) {
            roots.push(self.entry_rva);
        }
        for rva in self
            .export_rvas()
            .into_iter()
            .chain(self.tls_callback_rvas())
            .chain(self.load_config_cfg_rvas())
        {
            if self.is_exec(rva) {
                roots.push(rva);
            }
        }
        if reloc_roots_enabled() {
            for rva in self.base_reloc_code_rvas() {
                if self.is_exec(rva) {
                    roots.push(rva);
                }
            }
        }
        roots.sort_unstable();
        roots.dedup();
        roots
    }
}

struct CoverageReport {
    total_instrs: u64,
    bad_instrs: u64,
    blocks_visited: usize,
    leaf_callees: usize,
    lea_leaves: usize,
    ends: HashMap<(bool, Mnemonic), (u64, u32)>,
    per_root: Vec<(u32, u64, u64)>,
}

fn analyze_coverage(pe: &Pe, roots: &[u32], max_depth: u32) -> CoverageReport {
    let mut seen_insns: HashSet<u32> = HashSet::new();
    let mut seen_blocks: HashSet<u32> = HashSet::new();
    let mut seen_leaves: HashSet<u32> = HashSet::new();
    let mut seen_lea_leaves: HashSet<u32> = HashSet::new();
    let mut total = 0u64;
    let mut ends: HashMap<(bool, Mnemonic), (u64, u32)> = HashMap::new();
    let mut per_root: Vec<(u32, u64, u64)> = Vec::new();

    for &root in roots {
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        if pe.is_exec(root) {
            queue.push_back((root, 0));
        }
        let (mut n, mut bad) = (0u64, 0u64);
        while let Some((start_rva, depth)) = queue.pop_front() {
            if !seen_blocks.insert(start_rva) {
                continue;
            }
            let mut rva = start_rva;
            loop {
                if !seen_insns.insert(rva) {
                    break;
                }
                let Some(o) = pe.off(rva) else { break };
                let end = (o + 16).min(pe.data.len());
                let mut dec = iced_x86::Decoder::with_ip(64, &pe.data[o..end], pe.base + rva as u64, 0);
                let ins = dec.decode();
                if ins.is_invalid() {
                    break;
                }
                n += 1;
                let op = fast::lower(&ins);
                let generic = op.kind == fast::K_GENERIC;
                if generic || !jit::supported(&op) {
                    bad += 1;
                    let e = ends.entry((generic, ins.mnemonic())).or_insert((0, rva));
                    e.0 += 1;
                }
                let next = rva + ins.len() as u32;
                let can_expand = depth < max_depth;
                if can_expand && ins.mnemonic() == Mnemonic::Lea && ins.is_ip_rel_memory_operand() {
                    if let Some(target_rva) = pe.va_to_rva(ins.ip_rel_memory_address()) {
                        if pe.is_exec(target_rva)
                            && (pe.pdata.is_empty() || !pe.in_pdata(target_rva))
                            && pe.is_clean_code_entry(target_rva)
                            && !seen_blocks.contains(&target_rva)
                            && seen_lea_leaves.insert(target_rva)
                        {
                            queue.push_back((target_rva, depth + 1));
                        }
                    }
                }
                match ins.flow_control() {
                    FlowControl::Return | FlowControl::IndirectBranch | FlowControl::Interrupt => break,
                    FlowControl::UnconditionalBranch => {
                        if can_expand {
                            if let Some(t) = ins.near_branch_target().checked_sub(pe.base) {
                                let t32 = t as u32;
                                if pe.is_exec(t32) {
                                    if !pe.pdata.is_empty() && pe.in_pdata(start_rva) && !pe.in_pdata(t32) {
                                        seen_leaves.insert(t32);
                                    }
                                    queue.push_back((t32, depth + 1));
                                }
                            }
                        }
                        break;
                    }
                    FlowControl::ConditionalBranch => {
                        if can_expand {
                            if let Some(t) = ins.near_branch_target().checked_sub(pe.base) {
                                let t32 = t as u32;
                                if pe.is_exec(t32) {
                                    queue.push_back((t32, depth + 1));
                                }
                            }
                            if pe.is_exec(next) {
                                queue.push_back((next, depth + 1));
                            }
                        }
                        break;
                    }
                    FlowControl::Call => {
                        if can_expand {
                            if let Some(t) = ins.near_branch_target().checked_sub(pe.base) {
                                let t32 = t as u32;
                                // Follow direct calls to `.pdata`-less leaf callees (or any direct
                                // callee when the image has no `.pdata` directory).
                                if pe.is_exec(t32) && (pe.pdata.is_empty() || !pe.in_pdata(t32)) {
                                    if !pe.pdata.is_empty() {
                                        seen_leaves.insert(t32);
                                    }
                                    queue.push_back((t32, depth + 1));
                                }
                            }
                            if pe.is_exec(next) {
                                queue.push_back((next, depth + 1));
                            }
                        }
                        break;
                    }
                    FlowControl::IndirectCall => {
                        if can_expand && pe.is_exec(next) {
                            queue.push_back((next, depth + 1));
                        }
                        break;
                    }
                    _ => {}
                }
                rva = next;
            }
        }
        total += n;
        per_root.push((root, n, bad));
    }

    let bad_instrs = ends.values().map(|&(c, _)| c).sum();
    CoverageReport {
        total_instrs: total,
        bad_instrs,
        blocks_visited: seen_blocks.len(),
        leaf_callees: seen_leaves.len(),
        lea_leaves: seen_lea_leaves.len(),
        ends,
        per_root,
    }
}

#[test]
#[ignore]
fn jit_coverage() {
    let Ok(path) = std::env::var("AW2_JIT_COV_DLL") else {
        eprintln!("AW2_JIT_COV_DLL not set");
        return;
    };
    let pe = Pe::load(&path);
    let depth: u32 = ["AW2_JIT_COV_DEPTH", "PORT_AOT_CLOSURE", "AW2_DLL_AOT_CLOSURE"]
        .iter()
        .find_map(|k| std::env::var(k).ok().and_then(|s| s.trim().parse().ok()))
        .unwrap_or(DEFAULT_COV_CLOSURE);
    let roots: Vec<u32> = match std::env::var("AW2_JIT_COV_RVAS") {
        Ok(s) if !s.trim().is_empty() => s
            .split(',')
            .map(|t| u32::from_str_radix(t.trim().trim_start_matches("0x"), 16).expect("hex rva"))
            .collect(),
        _ => pe.default_roots(),
    };
    let rep = analyze_coverage(&pe, &roots, depth);
    let mut v: Vec<_> = rep.ends.into_iter().collect();
    v.sort_by_key(|(_, (c, _))| core::cmp::Reverse(*c));
    println!(
        "jit coverage {path}: {} roots, {} blocks, {} leaf callees, {} lea leaves, depth {depth}, {} instrs, {} block-ending ({:.2}%)",
        roots.len(),
        rep.blocks_visited,
        rep.leaf_callees,
        rep.lea_leaves,
        rep.total_instrs,
        rep.bad_instrs,
        rep.bad_instrs as f64 * 100.0 / rep.total_instrs.max(1) as f64
    );
    for ((g, m), (c, ex)) in v.iter().take(80) {
        println!("  {:>8} {:>7} {:?}  e.g. +{:#x}", c, if *g { "generic" } else { "nojit" }, m, ex);
    }
    if roots.len() <= 64 {
        for (r, n, b) in rep.per_root {
            println!("  root +{r:#x}: {n} instrs, {b} block-ending");
        }
    }
}

#[test]
fn jit_coverage_deep_closure_leaf_callees_and_roots() {
    use iced_x86::code_asm::*;

    // Synthetic PE32+ file laid out with file offsets == RVAs:
    //   .text  @ 0x1000..0x2000 (code)
    //   .rdata @ 0x2000..0x3000 (.pdata, TLS dir, LoadConfig CFG table)
    const BASE: u64 = 0x1_4000_0000;
    let mut buf = vec![0u8; 0x3000];
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(&mut buf, 0, 0x5A4D);
    w32(&mut buf, 0x3C, 0x40);
    w32(&mut buf, 0x40, 0x0000_4550);
    w16(&mut buf, 0x44, 0x8664);
    w16(&mut buf, 0x46, 2);
    w16(&mut buf, 0x54, 240);

    let opt = 0x58;
    w16(&mut buf, opt, 0x20B);
    w32(&mut buf, opt + 16, 0x1000); // EntryPoint
    w64(&mut buf, opt + 24, BASE);
    let img_len = buf.len() as u32;
    w32(&mut buf, opt + 56, img_len);
    w32(&mut buf, opt + 108, 16);
    // DataDirectory[3]: .pdata @ 0x2000 (1 entry = 12B)
    w32(&mut buf, opt + 112 + 3 * 8, 0x2000);
    w32(&mut buf, opt + 112 + 3 * 8 + 4, 12);
    // DataDirectory[9]: TLS @ 0x2100 (40B)
    w32(&mut buf, opt + 112 + 9 * 8, 0x2100);
    w32(&mut buf, opt + 112 + 9 * 8 + 4, 40);
    // DataDirectory[10]: LoadConfig @ 0x2200 (0x40 header dir size, 0xC0 struct size)
    w32(&mut buf, opt + 112 + 10 * 8, 0x2200);
    w32(&mut buf, opt + 112 + 10 * 8 + 4, 0x40);

    let sec0 = opt + 240;
    buf[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut buf, sec0 + 8, 0x1000);
    w32(&mut buf, sec0 + 12, 0x1000);
    w32(&mut buf, sec0 + 16, 0x1000);
    w32(&mut buf, sec0 + 20, 0x1000);
    w32(&mut buf, sec0 + 36, 0x6000_0020);

    let sec1 = sec0 + 40;
    buf[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut buf, sec1 + 8, 0x1000);
    w32(&mut buf, sec1 + 12, 0x2000);
    w32(&mut buf, sec1 + 16, 0x1000);
    w32(&mut buf, sec1 + 20, 0x2000);
    w32(&mut buf, sec1 + 36, 0x4000_0040);

    // Function 0 @ 0x1000 (in .pdata): 6-hop chain of conditional branches ending in a direct
    // call to `.pdata`-less leaf callee @ 0x1300!
    let mut a = CodeAssembler::new(64).unwrap();
    let mut labels: Vec<CodeLabel> = (0..6).map(|_| a.create_label()).collect();
    for i in 0..6 {
        a.cmp(ecx, i as i32).unwrap();
        let lbl = labels[i];
        a.je(lbl).unwrap();
    }
    a.call(BASE + 0x1300).unwrap();
    a.ret().unwrap();
    for (i, lbl) in labels.iter_mut().enumerate() {
        a.set_label(lbl).unwrap();
        a.mov(eax, i as u32).unwrap();
        a.ret().unwrap();
    }
    let fn0 = a.assemble(BASE + 0x1000).unwrap();
    buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

    // `.pdata`-less leaf callee @ 0x1300: add eax, 42; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.add(eax, 42).unwrap();
    a.ret().unwrap();
    let leaf = a.assemble(BASE + 0x1300).unwrap();
    buf[0x1300..0x1300 + leaf.len()].copy_from_slice(&leaf);

    // TLS callback @ 0x1340 (NOT in .pdata)
    buf[0x1340..0x1346].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);
    // CFG function table entry @ 0x1380 (NOT in .pdata)
    buf[0x1380..0x1386].copy_from_slice(&[0xB8, 2, 0, 0, 0, 0xC3]);

    // .pdata @ 0x2000: covers ONLY 0x1000..0x1200
    w32(&mut buf, 0x2000, 0x1000);
    w32(&mut buf, 0x2004, 0x1200);
    w32(&mut buf, 0x2008, 0x2080);

    // TLS directory @ 0x2100 -> callback array @ 0x2140
    w64(&mut buf, 0x2100 + 24, BASE + 0x2140);
    w64(&mut buf, 0x2140, BASE + 0x1340);
    w64(&mut buf, 0x2148, 0);

    // Load Config @ 0x2200 -> GuardCFFunctionTable @ 0x2300 with 5-byte entries (extra=1)
    w32(&mut buf, 0x2200, 0xC0); // Size
    w64(&mut buf, 0x2200 + 0x80, BASE + 0x2300); // GuardCFFunctionTable
    w64(&mut buf, 0x2200 + 0x88, 2); // GuardCFFunctionCount
    w32(&mut buf, 0x2200 + 0x90, 0x0000_0400 | (1 << 28)); // PRESENT + extra=1 byte/entry
    w32(&mut buf, 0x2300, 0x1000);
    buf[0x2304] = 0;
    w32(&mut buf, 0x2305, 0x1380);
    buf[0x2309] = 1;

    let pe = Pe::from_bytes(buf);
    let roots = pe.default_roots();
    assert_eq!(roots, vec![0x1000, 0x1340, 0x1380]);

    // Shallow depth 2 misses the leaf callee at depth 6; deep closure 128 discovers it!
    let shallow = analyze_coverage(&pe, &[0x1000], 2);
    let deep = analyze_coverage(&pe, &[0x1000], DEFAULT_COV_CLOSURE);
    assert_eq!(shallow.leaf_callees, 0);
    assert_eq!(deep.leaf_callees, 1);
    assert!(deep.blocks_visited > shallow.blocks_visited + 5);
    assert!(deep.total_instrs > shallow.total_instrs + 5);
}

#[test]
fn jit_coverage_base_reloc_roots_and_lea_leaf_discovery() {
    struct EnvRestore(Option<std::ffi::OsString>);
    impl Drop for EnvRestore {
        fn drop(&mut self) {
            match &self.0 {
                Some(v) => std::env::set_var("PORT_AOT_RELOC_ROOTS", v),
                None => std::env::remove_var("PORT_AOT_RELOC_ROOTS"),
            }
        }
    }
    let _restore = EnvRestore(std::env::var_os("PORT_AOT_RELOC_ROOTS"));
    std::env::remove_var("PORT_AOT_RELOC_ROOTS");

    const BASE: u64 = 0x1_4000_0000;
    let mut buf = vec![0u8; 0x3000];
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());
    let emit_lea_rip = |code: &mut Vec<u8>, code_base_va: u64, reg: u8, target_va: u64| {
        let rex = 0x48 | ((reg >> 3) << 2);
        let modrm = ((reg & 7) << 3) | 0x05;
        let next_ip = code_base_va + code.len() as u64 + 7;
        let disp = (target_va as i64 - next_ip as i64) as i32;
        code.push(rex);
        code.push(0x8D);
        code.push(modrm);
        code.extend_from_slice(&disp.to_le_bytes());
    };

    w16(&mut buf, 0, 0x5A4D);
    w32(&mut buf, 0x3C, 0x40);
    w32(&mut buf, 0x40, 0x0000_4550);
    w16(&mut buf, 0x44, 0x8664);
    w16(&mut buf, 0x46, 2);
    w16(&mut buf, 0x54, 240);

    let opt = 0x58;
    w16(&mut buf, opt, 0x20B);
    w32(&mut buf, opt + 16, 0x1000);
    w64(&mut buf, opt + 24, BASE);
    let img_len = buf.len() as u32;
    w32(&mut buf, opt + 56, img_len);
    w32(&mut buf, opt + 108, 16);
    // DataDirectory[3]: .pdata @ 0x2000 (2 entries = 24B)
    w32(&mut buf, opt + 112 + 3 * 8, 0x2000);
    w32(&mut buf, opt + 112 + 3 * 8 + 4, 24);
    // DataDirectory[5]: Base Relocation Directory @ 0x2400 (24B)
    w32(&mut buf, opt + 112 + 5 * 8, 0x2400);
    w32(&mut buf, opt + 112 + 5 * 8 + 4, 24);

    let sec0 = opt + 240;
    buf[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut buf, sec0 + 8, 0x1000);
    w32(&mut buf, sec0 + 12, 0x1000);
    w32(&mut buf, sec0 + 16, 0x1000);
    w32(&mut buf, sec0 + 20, 0x1000);
    w32(&mut buf, sec0 + 36, 0x6000_0020);

    let sec1 = sec0 + 40;
    buf[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut buf, sec1 + 8, 0x1000);
    w32(&mut buf, sec1 + 12, 0x2000);
    w32(&mut buf, sec1 + 16, 0x1000);
    w32(&mut buf, sec1 + 20, 0x2000);
    w32(&mut buf, sec1 + 36, 0x4000_0040);

    // .pdata: 0x1000..0x1100 and 0x1100..0x1140
    w32(&mut buf, 0x2000, 0x1000);
    w32(&mut buf, 0x2004, 0x1100);
    w32(&mut buf, 0x2008, 0x2080);
    w32(&mut buf, 0x200C, 0x1100);
    w32(&mut buf, 0x2010, 0x1140);
    w32(&mut buf, 0x2014, 0x2080);

    // .pdata function @ 0x1100
    buf[0x1100..0x1106].copy_from_slice(&[0xB8, 7, 0, 0, 0, 0xC3]);
    // `.pdata`-less vtable leaf @ 0x1200: mov al, 1; ret
    buf[0x1200..0x1203].copy_from_slice(&[0xB0, 0x01, 0xC3]);
    // Padding / invalid targets for reloc rejection
    buf[0x1240..0x1244].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]);
    // 0x1260 left zeroed (`[0x00, 0x00]`)

    // LEA targets in `.text`:
    buf[0x1300..0x1303].copy_from_slice(&[0xB0, 0x01, 0xC3]); // valid 16B-aligned leaf
    buf[0x1344] = 0xCC;
    buf[0x1345..0x1348].copy_from_slice(&[0x31, 0xC0, 0xC3]); // valid post-0xCC leaf
    buf[0x1380..0x1384].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]); // 0xCC pad
    // 0x13A0 left zeroed (`[0x00, 0x00]`)
    buf[0x13C0..0x13C4].copy_from_slice(&[0x06, 0x07, 0x0E, 0x16]); // invalid 64-bit opcode
    buf[0x13E2] = 0x55;
    buf[0x13E3..0x13E6].copy_from_slice(&[0xB0, 0x01, 0xC3]); // unaligned non-boundary data

    // Caller @ 0x1000 loads addresses via `LEA reg, [rip + disp32]` without calling them.
    let mut c = Vec::new();
    emit_lea_rip(&mut c, BASE + 0x1000, 0, BASE + 0x1300);
    emit_lea_rip(&mut c, BASE + 0x1000, 1, BASE + 0x1345);
    emit_lea_rip(&mut c, BASE + 0x1000, 2, BASE + 0x1100);
    emit_lea_rip(&mut c, BASE + 0x1000, 8, BASE + 0x1380);
    emit_lea_rip(&mut c, BASE + 0x1000, 9, BASE + 0x13A0);
    emit_lea_rip(&mut c, BASE + 0x1000, 10, BASE + 0x13C0);
    emit_lea_rip(&mut c, BASE + 0x1000, 11, BASE + 0x13E3);
    c.push(0xC3);
    buf[0x1000..0x1000 + c.len()].copy_from_slice(&c);

    // Vtable @ 0x2100 in `.rdata` + DIR64 reloc block @ 0x2400
    w64(&mut buf, 0x2100, BASE + 0x1200);
    w64(&mut buf, 0x2108, BASE + 0x2800);
    w64(&mut buf, 0x2110, BASE + 0x1240);
    w64(&mut buf, 0x2118, BASE + 0x1260);
    w32(&mut buf, 0x2400, 0x2000); // page_rva
    w32(&mut buf, 0x2404, 16); // block_size (8 header + 4 * 2B entries)
    w16(&mut buf, 0x2408, (10 << 12) | 0x100);
    w16(&mut buf, 0x240A, (10 << 12) | 0x108);
    w16(&mut buf, 0x240C, (10 << 12) | 0x110);
    w16(&mut buf, 0x240E, (10 << 12) | 0x118);

    let pe = Pe::from_bytes(buf.clone());
    assert_eq!(pe.default_roots(), vec![0x1000, 0x1100, 0x1200]);

    let rep = analyze_coverage(&pe, &[0x1000], DEFAULT_COV_CLOSURE);
    assert_eq!(rep.lea_leaves, 2);
    assert_eq!(rep.blocks_visited, 3);

    // Stripped `.reloc` fallback (zero DataDirectory[5], add unaligned non-boundary pointer).
    w32(&mut buf, opt + 112 + 5 * 8, 0);
    w32(&mut buf, opt + 112 + 5 * 8 + 4, 0);
    w64(&mut buf, 0x2120, BASE + 0x13E3);
    let pe_stripped = Pe::from_bytes(buf);
    assert_eq!(pe_stripped.default_roots(), vec![0x1000, 0x1100, 0x1200]);
}
