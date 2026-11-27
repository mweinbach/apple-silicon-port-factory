//! Static CFG successor & RIP-relative switch / jump-table discovery during offline AOT
//! closure analysis.

use super::cfg_discover::FuncMap;
use crate::emu::BlockCache;

/// Safety cap on jump-table entries scanned per indirect branch.
const MAX_JT_ENTRIES: usize = 4096;
/// Maximum number of hint-upgrade re-analyses per already-seen block during CFG closure.
pub(super) const MAX_JT_REANALYZE: u8 = 2;

/// Abstract GPR value tracked across basic blocks for RIP-relative switch / jump-table
/// discovery during offline static CFG closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum JtVal {
    Unknown,
    /// Known constant address (`lea reg, [rip + disp]`, `mov reg, imm64`, const arithmetic).
    Const(u64),
    /// Switch index with optional exclusive upper bound `0..bound` (from `cmp`/`ja` etc. or
    /// bitmask `and reg, mask`).
    Index { bound: Option<usize> },
    /// Pre-scaled switch index (`shl reg, 2|3` or `lea reg, [idx*4|8]`): `idx * scale`.
    ScaledIndex { scale: u8, indir: Option<(u64, u8)>, bound: Option<usize> },
    /// Secondary index loaded from an 8-bit or 16-bit indirection table:
    /// `idx2 = *(u8|u16)(tbl_va + idx * elem_bytes)`.
    /// For `elem_bytes == 2`, may also be added directly to a base register as a 16-bit
    /// unsigned offset jump table (`Target16`).
    IndirIndex {
        tbl_va: u64,
        elem_bytes: u8,
        assumed_base_reg: Option<u8>,
        bound: Option<usize>,
    },
    /// Address of a jump-table slot (`lea rPtr, [rTbl + idx*stride]` or `add rTbl, scaled_idx`),
    /// ready for `mov reg, [rPtr]` / `movsxd reg, [rPtr]` / `movsx/movzx reg, word ptr [rPtr]` /
    /// `jmp qword ptr [rPtr]`.
    TablePtr { tbl_va: u64, stride: u8, indir: Option<(u64, u8)>, bound: Option<usize> },
    /// 16-bit offset loaded from a jump table: `off = *(u16|i16)(tbl_va + idx * 2)`.
    TableOff16 {
        tbl_va: u64,
        signed: bool,
        assumed_base_reg: Option<u8>,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// 32-bit offset loaded from a jump table: `off = *(u32|i32)(tbl_va + idx * 4)`.
    /// `assumed_base_reg`: `Some(r)` when `r` was unknown (e.g. MSVC hoisted `__ImageBase`
    /// in `[rBase + idx*4 + rva_tbl]`) and `tbl_va` was formed as `image_base + disp`.
    TableOff32 {
        tbl_va: u64,
        signed: bool,
        assumed_base_reg: Option<u8>,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// Resolved 16-bit-offset jump target (`base_va ± entry`), ready for `jmp reg`.
    Target16 {
        base_va: u64,
        tbl_va: u64,
        signed: bool,
        sub_from_base: bool,
        assumed_base: bool,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// Resolved 32-bit-offset jump target (`base_va ± entry`), ready for `jmp reg`.
    Target32 {
        base_va: u64,
        tbl_va: u64,
        signed: bool,
        sub_from_base: bool,
        assumed_base: bool,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// Resolved 64-bit-pointer jump target (`*(u64)(tbl_va + idx * 8)`), ready for `jmp reg`.
    Target64 {
        tbl_va: u64,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
}

pub(super) type JtRegs = [JtVal; 16];

fn gpr_idx(reg: iced_x86::Register) -> Option<usize> {
    if reg == iced_x86::Register::None {
        return None;
    }
    let full = reg.full_register();
    full.is_gpr64().then(|| full.number())
}

fn op_imm(ins: &iced_x86::Instruction, op: u32) -> Option<u64> {
    use iced_x86::OpKind as K;
    match ins.op_kind(op) {
        K::Immediate8 => Some(ins.immediate8() as u64),
        K::Immediate16 => Some(ins.immediate16() as u64),
        K::Immediate32 => Some(ins.immediate32() as u64),
        K::Immediate64 => Some(ins.immediate64()),
        K::Immediate8to16 => Some(ins.immediate8to16() as u16 as u64),
        K::Immediate8to32 => Some(ins.immediate8to32() as u32 as u64),
        K::Immediate8to64 => Some(ins.immediate8to64() as u64),
        K::Immediate32to64 => Some(ins.immediate32to64() as u64),
        _ => None,
    }
}

fn extract_idx(v: JtVal) -> (Option<(u64, u8)>, Option<usize>) {
    match v {
        JtVal::Index { bound } => (None, bound),
        JtVal::IndirIndex { tbl_va, elem_bytes, bound, .. } => (Some((tbl_va, elem_bytes)), bound),
        _ => (None, None),
    }
}

fn pack_hint(regs: &JtRegs) -> Option<Box<JtRegs>> {
    if regs.iter().all(|v| matches!(v, JtVal::Unknown)) {
        None
    } else {
        Some(Box::new(*regs))
    }
}

fn jt_val_rank(v: JtVal) -> u8 {
    match v {
        JtVal::Unknown => 0,
        JtVal::Index { bound: None } => 1,
        JtVal::Index { bound: Some(_) } => 2,
        JtVal::IndirIndex { bound: None, .. } | JtVal::ScaledIndex { bound: None, .. } => 3,
        JtVal::IndirIndex { bound: Some(_), .. } | JtVal::ScaledIndex { bound: Some(_), .. } => 4,
        JtVal::Const(_) => 5,
        JtVal::TablePtr { .. } => 6,
        JtVal::TableOff16 { .. } | JtVal::TableOff32 { .. } => 7,
        JtVal::Target16 { .. } | JtVal::Target32 { .. } | JtVal::Target64 { .. } => 8,
    }
}

fn jt_upper_bound(v: JtVal) -> Option<usize> {
    match v {
        JtVal::Const(c) if c < MAX_JT_ENTRIES as u64 => Some(c as usize + 1),
        JtVal::Index { bound } => bound,
        _ => None,
    }
}

fn flip_cmp_cc(cc: iced_x86::ConditionCode) -> iced_x86::ConditionCode {
    use iced_x86::ConditionCode as CC;
    match cc {
        CC::a => CC::b,
        CC::ae => CC::be,
        CC::b => CC::a,
        CC::be => CC::ae,
        CC::g => CC::l,
        CC::ge => CC::le,
        CC::l => CC::g,
        CC::le => CC::ge,
        other => other,
    }
}

fn cmov_cc(mn: iced_x86::Mnemonic) -> Option<iced_x86::ConditionCode> {
    use iced_x86::{ConditionCode as CC, Mnemonic as M};
    match mn {
        M::Cmova => Some(CC::a),
        M::Cmovae => Some(CC::ae),
        M::Cmovb => Some(CC::b),
        M::Cmovbe => Some(CC::be),
        M::Cmovg => Some(CC::g),
        M::Cmovge => Some(CC::ge),
        M::Cmovl => Some(CC::l),
        M::Cmovle => Some(CC::le),
        M::Cmove => Some(CC::e),
        M::Cmovne => Some(CC::ne),
        _ => None,
    }
}

/// Merge incoming hint `new` into `dst`, keeping richer register knowledge from both
/// predecessors. Returns `true` if any register strictly gained information.
pub(super) fn merge_jt_regs(dst: &mut JtRegs, new: &JtRegs) -> bool {
    let mut improved = false;
    for (d, &n) in dst.iter_mut().zip(new.iter()) {
        if *d == n || matches!(n, JtVal::Unknown) {
            continue;
        }
        let merged = match (*d, n) {
            (JtVal::Unknown, v) => v,
            (JtVal::Index { bound: b1 }, JtVal::Index { bound: b2 }) => {
                let b = match (b1, b2) {
                    (Some(x), Some(y)) => Some(x.max(y)),
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (None, None) => None,
                };
                JtVal::Index { bound: b }
            }
            (old, cand) if jt_val_rank(cand) > jt_val_rank(old) => cand,
            (old, _) => old,
        };
        if merged != *d {
            *d = merged;
            improved = true;
        }
    }
    improved
}

fn adjust_index_bound(bound: Option<usize>, signed_delta: i64) -> JtVal {
    if signed_delta == 0 {
        return JtVal::Index { bound };
    }
    if signed_delta < 0 {
        let sub = (-signed_delta) as u64;
        if sub < MAX_JT_ENTRIES as u64 {
            return JtVal::Index {
                bound: bound.and_then(|b| b.checked_sub(sub as usize)).filter(|&b| b > 0),
            };
        }
    } else if (signed_delta as u64) < MAX_JT_ENTRIES as u64 {
        return JtVal::Index {
            bound: bound.map(|b| b.saturating_add(signed_delta as usize).min(MAX_JT_ENTRIES)),
        };
    }
    JtVal::Unknown
}

fn combine_add_target(lhs: JtVal, lhs_reg: usize, rhs: JtVal, rhs_reg: usize, img_base: u64) -> JtVal {
    let pair = |off: JtVal, base_val: JtVal, base_reg: usize| -> Option<JtVal> {
        match off {
            JtVal::TableOff32 { tbl_va, signed, assumed_base_reg, indir, bound } => {
                match (assumed_base_reg, base_val) {
                    (None, JtVal::Const(base_va)) => Some(JtVal::Target32 {
                        base_va,
                        tbl_va,
                        signed,
                        sub_from_base: false,
                        assumed_base: false,
                        indir,
                        bound,
                    }),
                    (Some(breg), _) if breg as usize == base_reg => Some(JtVal::Target32 {
                        base_va: img_base,
                        tbl_va,
                        signed,
                        sub_from_base: false,
                        assumed_base: true,
                        indir,
                        bound,
                    }),
                    _ => None,
                }
            }
            JtVal::TableOff16 { tbl_va, signed, assumed_base_reg, indir, bound } => {
                match (assumed_base_reg, base_val) {
                    (None, JtVal::Const(base_va)) => Some(JtVal::Target16 {
                        base_va,
                        tbl_va,
                        signed,
                        sub_from_base: false,
                        assumed_base: false,
                        indir,
                        bound,
                    }),
                    (Some(breg), _) if breg as usize == base_reg => Some(JtVal::Target16 {
                        base_va: img_base,
                        tbl_va,
                        signed,
                        sub_from_base: false,
                        assumed_base: true,
                        indir,
                        bound,
                    }),
                    _ => None,
                }
            }
            JtVal::IndirIndex { tbl_va, elem_bytes: 2, assumed_base_reg, bound } => {
                match (assumed_base_reg, base_val) {
                    (None, JtVal::Const(base_va)) => Some(JtVal::Target16 {
                        base_va,
                        tbl_va,
                        signed: false,
                        sub_from_base: false,
                        assumed_base: false,
                        indir: None,
                        bound,
                    }),
                    (Some(breg), _) if breg as usize == base_reg => Some(JtVal::Target16 {
                        base_va: img_base,
                        tbl_va,
                        signed: false,
                        sub_from_base: false,
                        assumed_base: true,
                        indir: None,
                        bound,
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    };
    if let Some(t) = pair(lhs, rhs, rhs_reg) {
        return t;
    }
    if let Some(t) = pair(rhs, lhs, lhs_reg) {
        return t;
    }
    match (lhs, rhs) {
        (JtVal::Const(a), JtVal::Const(b)) => JtVal::Const(a.wrapping_add(b)),
        (JtVal::Const(tbl_va), JtVal::ScaledIndex { scale, indir, bound })
        | (JtVal::ScaledIndex { scale, indir, bound }, JtVal::Const(tbl_va)) => {
            JtVal::TablePtr { tbl_va, stride: scale, indir, bound }
        }
        _ => JtVal::Unknown,
    }
}

fn eval_mem_load(
    ins: &iced_x86::Instruction,
    regs: &JtRegs,
    signed: bool,
    img_base: u64,
    img_size: u64,
) -> JtVal {
    if ins.is_ip_rel_memory_operand() {
        return JtVal::Unknown;
    }
    let msize = ins.memory_size().size();
    let scale = ins.memory_index_scale();
    let mbase = gpr_idx(ins.memory_base());
    let midx = gpr_idx(ins.memory_index());
    let disp = ins.memory_displacement64();

    // Pre-computed table slot pointer: `mov eax, [rPtr]` / `movsxd rax, [rPtr]` /
    // `movsx rax, word ptr [rPtr]` / `jmp [rPtr]`.
    if midx.is_none() && disp == 0 {
        if let Some(b) = mbase {
            if let JtVal::TablePtr { tbl_va, stride, indir, bound } = regs[b] {
                if msize == 2 && stride == 2 {
                    return JtVal::TableOff16 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if msize == 4 && stride == 4 {
                    return JtVal::TableOff32 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if !signed && msize == 8 && stride == 8 {
                    return JtVal::Target64 { tbl_va, indir, bound };
                }
            }
        }
        return JtVal::Unknown;
    }

    let Some(ix) = midx else {
        return JtVal::Unknown;
    };

    // Pre-scaled index in `[rTbl + rScaledIdx]` (scale == 1, e.g. after `shl rIdx, 1|2|3`).
    if scale == 1 {
        if let Some(b) = mbase {
            let scaled_pair = match (regs[b], regs[ix]) {
                (JtVal::Const(c), JtVal::ScaledIndex { scale: s, indir, bound }) => {
                    Some((c.wrapping_add(disp), s, indir, bound))
                }
                (JtVal::ScaledIndex { scale: s, indir, bound }, JtVal::Const(c)) => {
                    Some((c.wrapping_add(disp), s, indir, bound))
                }
                _ => None,
            };
            if let Some((tbl_va, s, indir, bound)) = scaled_pair {
                if msize == 2 && s == 2 {
                    return JtVal::TableOff16 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if msize == 4 && s == 4 {
                    return JtVal::TableOff32 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if !signed && msize == 8 && s == 8 {
                    return JtVal::Target64 { tbl_va, indir, bound };
                }
            }
        }
    }

    let (indir, bound) = extract_idx(regs[ix]);

    if msize == 2 && scale == 2 {
        match mbase {
            Some(b) => match regs[b] {
                JtVal::Const(c) => JtVal::TableOff16 {
                    tbl_va: c.wrapping_add(disp),
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                },
                _ if disp > 0 && disp < img_size => JtVal::TableOff16 {
                    tbl_va: img_base.wrapping_add(disp),
                    signed,
                    assumed_base_reg: Some(b as u8),
                    indir,
                    bound,
                },
                _ => JtVal::Unknown,
            },
            None if disp >= img_base && disp < img_base.saturating_add(img_size) => {
                JtVal::TableOff16 {
                    tbl_va: disp,
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                }
            }
            None => JtVal::Unknown,
        }
    } else if msize == 4 && scale == 4 {
        match mbase {
            Some(b) => match regs[b] {
                JtVal::Const(c) => JtVal::TableOff32 {
                    tbl_va: c.wrapping_add(disp),
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                },
                _ if disp > 0 && disp < img_size => JtVal::TableOff32 {
                    tbl_va: img_base.wrapping_add(disp),
                    signed,
                    assumed_base_reg: Some(b as u8),
                    indir,
                    bound,
                },
                _ => JtVal::Unknown,
            },
            None if disp >= img_base && disp < img_base.saturating_add(img_size) => {
                JtVal::TableOff32 {
                    tbl_va: disp,
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                }
            }
            None => JtVal::Unknown,
        }
    } else if !signed && msize == 8 && scale == 8 {
        match mbase {
            Some(b) => match regs[b] {
                JtVal::Const(c) => JtVal::Target64 { tbl_va: c.wrapping_add(disp), indir, bound },
                _ => JtVal::Unknown,
            },
            None if disp >= img_base && disp < img_base.saturating_add(img_size) => {
                JtVal::Target64 { tbl_va: disp, indir, bound }
            }
            None => JtVal::Unknown,
        }
    } else {
        JtVal::Unknown
    }
}

fn eval_mem_movzx(
    ins: &iced_x86::Instruction,
    regs: &JtRegs,
    img_base: u64,
    img_size: u64,
) -> JtVal {
    let msize = ins.memory_size().size();
    if msize != 1 && msize != 2 {
        return JtVal::Unknown;
    }
    if ins.is_ip_rel_memory_operand() {
        return if msize == 1 {
            JtVal::Index { bound: Some(256) }
        } else {
            JtVal::Unknown
        };
    }
    let scale = ins.memory_index_scale();
    let mbase = gpr_idx(ins.memory_base());
    let midx = gpr_idx(ins.memory_index());
    let disp = ins.memory_displacement64();

    // Pre-computed 16-bit table slot pointer (`movzx reg, word ptr [rPtr]`) or pre-scaled index.
    if msize == 2 && midx.is_none() && disp == 0 {
        if let Some(b) = mbase {
            if let JtVal::TablePtr { tbl_va, stride: 2, indir, bound } = regs[b] {
                return JtVal::TableOff16 {
                    tbl_va,
                    signed: false,
                    assumed_base_reg: None,
                    indir,
                    bound,
                };
            }
        }
    }
    if msize == 2 && scale == 1 {
        if let (Some(b), Some(ix)) = (mbase, midx) {
            let scaled_pair = match (regs[b], regs[ix]) {
                (JtVal::Const(c), JtVal::ScaledIndex { scale: 2, indir, bound }) => {
                    Some((c.wrapping_add(disp), indir, bound))
                }
                (JtVal::ScaledIndex { scale: 2, indir, bound }, JtVal::Const(c)) => {
                    Some((c.wrapping_add(disp), indir, bound))
                }
                _ => None,
            };
            if let Some((tbl_va, indir, bound)) = scaled_pair {
                return JtVal::TableOff16 {
                    tbl_va,
                    signed: false,
                    assumed_base_reg: None,
                    indir,
                    bound,
                };
            }
        }
    }
    if scale != msize as u32 {
        return JtVal::Unknown;
    }

    match (mbase, midx) {
        (Some(b), Some(ix)) => {
            if let JtVal::Const(c) = regs[b] {
                let (indir, bound) = extract_idx(regs[ix]);
                if msize == 2 && indir.is_some() {
                    return JtVal::TableOff16 {
                        tbl_va: c.wrapping_add(disp),
                        signed: false,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                return JtVal::IndirIndex {
                    tbl_va: c.wrapping_add(disp),
                    elem_bytes: msize as u8,
                    assumed_base_reg: None,
                    bound,
                };
            }
            if scale == 1 {
                if let JtVal::Const(c) = regs[ix] {
                    let (_, bound) = extract_idx(regs[b]);
                    return JtVal::IndirIndex {
                        tbl_va: c.wrapping_add(disp),
                        elem_bytes: 1,
                        assumed_base_reg: None,
                        bound,
                    };
                }
            }
            // Hoisted __ImageBase heuristic: `movzx rIdx2, byte/word ptr [rBase + rIdx*s + rva]`.
            if disp > 0 && disp < img_size {
                let (base_reg, idx_reg) = if scale == 1
                    && matches!(regs[b], JtVal::Index { bound: Some(_) })
                    && !matches!(regs[ix], JtVal::Index { bound: Some(_) })
                {
                    (ix, b)
                } else {
                    (b, ix)
                };
                let (indir, bound) = extract_idx(regs[idx_reg]);
                if msize == 2 && indir.is_some() {
                    return JtVal::TableOff16 {
                        tbl_va: img_base.wrapping_add(disp),
                        signed: false,
                        assumed_base_reg: Some(base_reg as u8),
                        indir,
                        bound,
                    };
                }
                return JtVal::IndirIndex {
                    tbl_va: img_base.wrapping_add(disp),
                    elem_bytes: msize as u8,
                    assumed_base_reg: Some(base_reg as u8),
                    bound,
                };
            }
            JtVal::Unknown
        }
        (None, Some(ix)) if disp >= img_base && disp < img_base.saturating_add(img_size) => {
            let (indir, bound) = extract_idx(regs[ix]);
            if msize == 2 && indir.is_some() {
                return JtVal::TableOff16 {
                    tbl_va: disp,
                    signed: false,
                    assumed_base_reg: None,
                    indir,
                    bound,
                };
            }
            JtVal::IndirIndex {
                tbl_va: disp,
                elem_bytes: msize as u8,
                assumed_base_reg: None,
                bound,
            }
        }
        // Byte-loaded switch/bytecode index (`movzx reg, byte ptr [base + disp]`): naturally
        // bounded to `0..256` without requiring an explicit prior `cmp reg, imm` check.
        (Some(_), None) if msize == 1 => JtVal::Index { bound: Some(256) },
        _ => JtVal::Unknown,
    }
}

/// Validate that `va` lies inside an executable section of the module and begins with a
/// valid decoded x86-64 instruction.
pub(super) unsafe fn valid_code_target(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(_s, e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let avail = (e as u64 - va).min(15) as usize;
    if avail == 0 {
        return false;
    }
    let bytes = core::slice::from_raw_parts(va as usize as *const u8, avail);
    let mut dec = iced_x86::Decoder::with_ip(64, bytes, va, iced_x86::DecoderOptions::NONE);
    !dec.decode().is_invalid()
}

/// Validate that `va` lies inside an executable section, does not start with `0xCC` (`int3`)
/// or `[0x00, 0x00]` (`add [rax], al`) padding, and decodes cleanly via [`valid_code_target`].
pub(super) unsafe fn is_plausible_code_entry(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(_s, e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let avail = (e as u64 - va).min(15) as usize;
    if avail == 0 {
        return false;
    }
    let b0 = core::ptr::read_unaligned(va as usize as *const u8);
    if b0 == 0xCC {
        return false;
    }
    if b0 == 0x00
        && avail >= 2
        && core::ptr::read_unaligned((va as usize + 1) as *const u8) == 0x00
    {
        return false;
    }
    valid_code_target(va, text)
}

/// True if `va` lies at a clean function entry boundary inside an executable section:
/// 16-byte aligned relative to the executable section start (`((va - s) & 0xF) == 0`, matching
/// `(va & 0xF) == 0` for page-aligned PEs) OR immediately preceded within the same executable
/// section by `0xCC` (`int3`), `0x90` (`nop`), or `0xC3` (`ret`), AND passes
/// [`is_plausible_code_entry`].
pub(super) unsafe fn is_clean_code_entry(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(s, _e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let clean_boundary = ((va - s as u64) & 0xF) == 0
        || (va > s as u64
            && matches!(
                core::ptr::read_unaligned((va as usize - 1) as *const u8),
                0xCC | 0x90 | 0xC3
            ));
    clean_boundary && is_plausible_code_entry(va, text)
}

/// Read and validate switch / jump-table entries from a mapped module `[base, base + size)`.
/// Stops immediately on the first out-of-range, non-code, self-table, or cross-function
/// entry. Deduplicates targets deterministically.
unsafe fn resolve_jt_targets(
    kind: JtVal,
    base: usize,
    size: usize,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    cur_func: Option<u32>,
) -> Vec<u64> {
    let img_lo = (base as u64).saturating_add(0x100);
    let img_hi = (base as u64).saturating_add(size as u64);
    let in_img = |addr: u64, len: u64| -> bool {
        addr >= img_lo && addr.checked_add(len).is_some_and(|end| end <= img_hi)
    };

    let (base_va, tbl_va, stride, signed, sub_from_base, assumed_base, indir, bound) = match kind {
        JtVal::Target16 { base_va, tbl_va, signed, sub_from_base, assumed_base, indir, bound } => {
            (base_va, tbl_va, 2u64, signed, sub_from_base, assumed_base, indir, bound)
        }
        JtVal::Target32 { base_va, tbl_va, signed, sub_from_base, assumed_base, indir, bound } => {
            (base_va, tbl_va, 4u64, signed, sub_from_base, assumed_base, indir, bound)
        }
        JtVal::Target64 { tbl_va, indir, bound } => {
            (0u64, tbl_va, 8u64, false, false, false, indir, bound)
        }
        _ => return Vec::new(),
    };

    let limit = match bound {
        Some(0) => return Vec::new(),
        Some(n) => n.min(MAX_JT_ENTRIES),
        None => MAX_JT_ENTRIES,
    };

    let mut targets = Vec::new();
    let mut raw_count = 0usize;
    let mut max_jt_idx = 0usize;

    for i in 0..limit {
        let jt_idx: usize = if let Some((idx_tbl, elem_bytes)) = indir {
            let Some(off) = (i as u64).checked_mul(elem_bytes as u64) else { break };
            let Some(addr) = idx_tbl.checked_add(off) else { break };
            if !in_img(addr, elem_bytes as u64) {
                break;
            }
            let v = if elem_bytes == 1 {
                core::ptr::read_unaligned(addr as usize as *const u8) as usize
            } else {
                core::ptr::read_unaligned(addr as usize as *const u16) as usize
            };
            if v >= MAX_JT_ENTRIES {
                break;
            }
            v
        } else {
            i
        };

        let Some(off) = (jt_idx as u64).checked_mul(stride) else { break };
        let Some(entry_addr) = tbl_va.checked_add(off) else { break };
        if !in_img(entry_addr, stride) {
            break;
        }
        let target = if stride == 2 {
            let raw = core::ptr::read_unaligned(entry_addr as usize as *const u16);
            let disp = if signed { raw as i16 as i64 as u64 } else { raw as u64 };
            if sub_from_base {
                base_va.wrapping_sub(disp)
            } else {
                base_va.wrapping_add(disp)
            }
        } else if stride == 4 {
            let raw = core::ptr::read_unaligned(entry_addr as usize as *const u32);
            let disp = if signed { raw as i32 as i64 as u64 } else { raw as u64 };
            if sub_from_base {
                base_va.wrapping_sub(disp)
            } else {
                base_va.wrapping_add(disp)
            }
        } else {
            core::ptr::read_unaligned(entry_addr as usize as *const u64)
        };

        max_jt_idx = max_jt_idx.max(jt_idx);
        let tbl_end = tbl_va.saturating_add((max_jt_idx as u64 + 1).saturating_mul(stride));
        if target >= tbl_va && target < tbl_end {
            // A switch handler cannot jump inside its own jump-table data (guards against
            // zero-padded tables placed inside `.text`).
            break;
        }
        if !fmap.is_empty() {
            let rva = target.wrapping_sub(base as u64);
            if rva >= size as u64 || fmap.root_of(rva as u32) != cur_func {
                break;
            }
        }
        if !valid_code_target(target, text) {
            break;
        }
        raw_count += 1;
        if !targets.contains(&target) {
            targets.push(target);
        }
    }

    // Hoisted __ImageBase heuristic requires at least 2 valid entries (or an explicit bound of 1)
    // to avoid false positives on unrelated indexed dword loads.
    if assumed_base && raw_count < 2 && bound != Some(1) {
        return Vec::new();
    }
    targets
}

/// Kind of static CFG edge produced by [`analyze_block`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum EdgeKind {
    /// Normal intra-function edge (fallthrough or conditional branch).
    Intra,
    /// Unconditional direct jump (intra-function, or tail-call into a `.pdata`-less leaf).
    TailBranch,
    /// Direct `call rel32` target (followed for `.pdata`-less leaf callees / intra-function calls).
    DirectCall,
    /// Switch / jump-table case target (strictly within the same `.pdata` function).
    JumpTable,
    /// `.pdata`-less leaf function entry loaded via `LEA reg, [rip + disp32]`.
    LeaLeaf,
}

/// Emit [`EdgeKind::LeaLeaf`] edges for `LEA reg, [rip + disp32]` instructions targeting
/// `.pdata`-less clean leaf function entries inside `text`.
#[inline]
unsafe fn push_lea_leaf_edges(
    insns: &[iced_x86::Instruction],
    img_base: u64,
    img_size: u64,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    edges: &mut Vec<(u64, EdgeKind, Option<Box<JtRegs>>)>,
) {
    for ins in insns {
        if ins.mnemonic() == iced_x86::Mnemonic::Lea && ins.is_ip_rel_memory_operand() {
            let ea = ins.ip_rel_memory_address();
            if !text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ea)) {
                continue;
            }
            let rva = ea.wrapping_sub(img_base);
            if rva < img_size
                && (fmap.is_empty() || fmap.root_of(rva as u32).is_none())
                && is_clean_code_entry(ea, text)
            {
                edges.push((ea, EdgeKind::LeaLeaf, None));
            }
        }
    }
}

/// Result of analysing one basic block during static CFG discovery.
#[derive(Default)]
pub(super) struct BlockStep {
    /// `(successor_ip, edge_kind, outgoing_register_hint)`.
    pub(super) edges: Vec<(u64, EdgeKind, Option<Box<JtRegs>>)>,
    /// Number of unique targets discovered via a switch / jump table at this block's terminator.
    pub(super) jt_targets: usize,
    /// True if this block ends in an indirect jump (`jmp reg` / `jmp [mem]`), so arrival of
    /// richer register hints later in BFS can re-analyse the dispatch.
    pub(super) ends_indirect_jmp: bool,
}

/// Static successors of the block at `ip`, including RIP-relative switch / jump-table targets
/// when `jt_on` is true, direct call targets, `LEA reg, [rip + disp32]` `.pdata`-less leaf
/// targets, and outgoing register/bound hints.
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn analyze_block(
    bc: *mut BlockCache,
    ip: u64,
    in_regs: Option<&JtRegs>,
    base: usize,
    size: usize,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    jt_on: bool,
) -> BlockStep {
    use iced_x86::{ConditionCode as CC, FlowControl as F, Mnemonic as M, OpKind as K};
    let mut dummy = 0u64;
    let Ok((off, n)) = BlockCache::get(bc, ip, &mut dummy) else {
        return BlockStep::default();
    };
    let insns = &(&(*bc).arena)[off..off + n];
    let ops = &(&(*bc).ops)[off..off + n];
    let last = &insns[n - 1];
    let (tgt, next) = (last.near_branch_target(), last.next_ip());
    let fc = last.flow_control();
    let img_base = base as u64;
    let img_size = size as u64;

    if !jt_on {
        let mut step = BlockStep::default();
        // Whenever an instruction before the block's branch terminator is not supported by the
        // ARM64 JIT (`!jit::supported`, e.g. `xgetbv`, `ldmxcsr`, `cpuid`), `prep_block_with`
        // truncates the AOT block right before it and execution resumes at `insns[i].next_ip()`.
        // Emit that resume IP as an intra-function edge so the tail is precompiled into `.aot`.
        for i in 0..n.saturating_sub(1) {
            if !crate::emu::jit::supported(&ops[i]) {
                step.edges.push((insns[i].next_ip(), EdgeKind::Intra, None));
            }
        }
        match fc {
            F::Next | F::IndirectCall => step.edges.push((next, EdgeKind::Intra, None)),
            F::Call => {
                step.edges.push((next, EdgeKind::Intra, None));
                if tgt != 0 {
                    step.edges.push((tgt, EdgeKind::DirectCall, None));
                }
            }
            F::ConditionalBranch => {
                step.edges.push((tgt, EdgeKind::Intra, None));
                step.edges.push((next, EdgeKind::Intra, None));
            }
            F::UnconditionalBranch if tgt != 0 => {
                step.edges.push((tgt, EdgeKind::TailBranch, None));
            }
            _ => {}
        }
        push_lea_leaf_edges(insns, img_base, img_size, text, fmap, &mut step.edges);
        return step;
    }

    let mut regs: JtRegs = in_regs.copied().unwrap_or([JtVal::Unknown; 16]);
    let mut alias: [u16; 16] = core::array::from_fn(|i| 1u16 << i);
    // `(alias_mask, const_bound, reversed)`: `reversed = false` means `cmp reg, const`;
    // `reversed = true` means `cmp const, reg`.
    let mut last_cmp: Option<(u16, u64, bool)> = None;

    let clobber_alias = |alias: &mut [u16; 16], r: usize| {
        let bit = !(1u16 << r);
        for m in alias.iter_mut() {
            *m &= bit;
        }
        alias[r] = 1u16 << r;
    };
    let link_alias = |alias: &mut [u16; 16], dst: usize, src: usize| {
        if dst != src {
            let bit = !(1u16 << dst);
            for m in alias.iter_mut() {
                *m &= bit;
            }
            let cls = alias[src] | (1u16 << dst);
            for (k, m) in alias.iter_mut().enumerate() {
                if cls & (1u16 << k) != 0 {
                    *m = cls;
                }
            }
        }
    };
    let clear_cmp_reg = |last_cmp: &mut Option<(u16, u64, bool)>, r: usize| {
        if let Some((mut mask, imm, rev)) = *last_cmp {
            mask &= !(1u16 << r);
            *last_cmp = (mask != 0).then_some((mask, imm, rev));
        }
    };
    let copy_cmp_alias = |last_cmp: &mut Option<(u16, u64, bool)>, dst: usize, src: usize| {
        if let Some((mut mask, imm, rev)) = *last_cmp {
            if mask & (1u16 << src) != 0 {
                mask |= 1u16 << dst;
            } else {
                mask &= !(1u16 << dst);
            }
            *last_cmp = (mask != 0).then_some((mask, imm, rev));
        }
    };

    let mut step = BlockStep::default();

    for (ins_idx, ins) in insns.iter().enumerate() {
        let mn = ins.mnemonic();
        let mid_resume = ins_idx + 1 < n && !crate::emu::jit::supported(&ops[ins_idx]);
        if mn == M::Cmp {
            last_cmp = if ins.op_count() >= 2 && ins.op_kind(0) == K::Register {
                match (gpr_idx(ins.op0_register()), op_imm(ins, 1)) {
                    (Some(r), Some(imm)) => Some((alias[r], imm, false)),
                    (Some(r0), None) if ins.op_kind(1) == K::Register => {
                        match gpr_idx(ins.op1_register()) {
                            Some(r1) => match (regs[r0], regs[r1]) {
                                (_, JtVal::Const(imm)) => Some((alias[r0], imm, false)),
                                (JtVal::Const(imm), _) => Some((alias[r1], imm, true)),
                                _ => None,
                            },
                            None => None,
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };
            if mid_resume {
                step.edges.push((ins.next_ip(), EdgeKind::Intra, pack_hint(&regs)));
            }
            continue;
        }

        let modifies_flags = (ins.rflags_written()
            | ins.rflags_cleared()
            | ins.rflags_set()
            | ins.rflags_undefined())
            != 0;
        if modifies_flags {
            last_cmp = None;
        }

        match mn {
            M::Lea => {
                if let Some(dst) = gpr_idx(ins.op0_register()) {
                    let new_val = if ins.is_ip_rel_memory_operand() {
                        clobber_alias(&mut alias, dst);
                        JtVal::Const(ins.ip_rel_memory_address())
                    } else {
                        let mbase = gpr_idx(ins.memory_base());
                        let midx = gpr_idx(ins.memory_index());
                        let scale = ins.memory_index_scale();
                        let disp = ins.memory_displacement64();
                        match (mbase, midx, scale, disp) {
                            (Some(b), None, _, 0) => {
                                let v = regs[b];
                                link_alias(&mut alias, dst, b);
                                copy_cmp_alias(&mut last_cmp, dst, b);
                                regs[dst] = v;
                                continue;
                            }
                            (Some(b), None, _, d) => {
                                clobber_alias(&mut alias, dst);
                                match regs[b] {
                                    JtVal::Const(c) => JtVal::Const(c.wrapping_add(d)),
                                    JtVal::Index { bound } => {
                                        let sd = if ins.op0_register().size() == 4 {
                                            d as i32 as i64
                                        } else {
                                            d as i64
                                        };
                                        adjust_index_bound(bound, sd)
                                    }
                                    _ => JtVal::Unknown,
                                }
                            }
                            (None, Some(ix), s @ (2 | 4 | 8), 0) => {
                                clobber_alias(&mut alias, dst);
                                let (indir, bound) = extract_idx(regs[ix]);
                                JtVal::ScaledIndex { scale: s as u8, indir, bound }
                            }
                            (Some(b), Some(ix), s @ (2 | 4 | 8), d) => {
                                clobber_alias(&mut alias, dst);
                                if let JtVal::Const(c) = regs[b] {
                                    let (indir, bound) = extract_idx(regs[ix]);
                                    JtVal::TablePtr {
                                        tbl_va: c.wrapping_add(d),
                                        stride: s as u8,
                                        indir,
                                        bound,
                                    }
                                } else {
                                    JtVal::Unknown
                                }
                            }
                            (Some(b), Some(ix), 1, 0) => {
                                clobber_alias(&mut alias, dst);
                                combine_add_target(regs[b], b, regs[ix], ix, img_base)
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                JtVal::Unknown
                            }
                        }
                    };
                    regs[dst] = new_val;
                    clear_cmp_reg(&mut last_cmp, dst);
                }
            }
            M::Mov => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_load(ins, &regs, false, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = match op_imm(ins, 1) {
                                    Some(imm) => JtVal::Const(imm),
                                    None => JtVal::Unknown,
                                };
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Movsxd | M::Movsx => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_load(ins, &regs, true, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = JtVal::Unknown;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Movzx => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_movzx(ins, &regs, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = JtVal::Unknown;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Cdqe => {
                // `movsxd rax, eax`: preserves regs[0], alias[0], and last_cmp.
            }
            M::Add => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = if ins.op_kind(1) == K::Register {
                            match gpr_idx(ins.op1_register()) {
                                Some(src) => {
                                    combine_add_target(regs[dst], dst, regs[src], src, img_base)
                                }
                                None => JtVal::Unknown,
                            }
                        } else if let Some(imm) = op_imm(ins, 1) {
                            let is32 = ins.op0_register().size() == 4;
                            match regs[dst] {
                                JtVal::Const(c) => {
                                    if is32 {
                                        JtVal::Const((c as u32).wrapping_add(imm as u32) as u64)
                                    } else {
                                        JtVal::Const(c.wrapping_add(imm))
                                    }
                                }
                                JtVal::Index { bound } => {
                                    let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                    adjust_index_bound(bound, sd)
                                }
                                _ => JtVal::Unknown,
                            }
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Sub => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = if ins.op_kind(1) == K::Register {
                            match gpr_idx(ins.op1_register()) {
                                Some(src) if src == dst => JtVal::Const(0),
                                Some(src) => match (regs[dst], regs[src]) {
                                    (JtVal::Const(a), JtVal::Const(b)) => {
                                        if is32 {
                                            JtVal::Const((a as u32).wrapping_sub(b as u32) as u64)
                                        } else {
                                            JtVal::Const(a.wrapping_sub(b))
                                        }
                                    }
                                    (
                                        JtVal::Const(base_va),
                                        JtVal::TableOff16 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: None,
                                            indir,
                                            bound,
                                        },
                                    ) => JtVal::Target16 {
                                        base_va,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: false,
                                        indir,
                                        bound,
                                    },
                                    (
                                        _,
                                        JtVal::TableOff16 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: Some(breg),
                                            indir,
                                            bound,
                                        },
                                    ) if breg as usize == dst => JtVal::Target16 {
                                        base_va: img_base,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: true,
                                        indir,
                                        bound,
                                    },
                                    (
                                        JtVal::Const(base_va),
                                        JtVal::IndirIndex {
                                            tbl_va,
                                            elem_bytes: 2,
                                            assumed_base_reg: None,
                                            bound,
                                        },
                                    ) => JtVal::Target16 {
                                        base_va,
                                        tbl_va,
                                        signed: false,
                                        sub_from_base: true,
                                        assumed_base: false,
                                        indir: None,
                                        bound,
                                    },
                                    (
                                        _,
                                        JtVal::IndirIndex {
                                            tbl_va,
                                            elem_bytes: 2,
                                            assumed_base_reg: Some(breg),
                                            bound,
                                        },
                                    ) if breg as usize == dst => JtVal::Target16 {
                                        base_va: img_base,
                                        tbl_va,
                                        signed: false,
                                        sub_from_base: true,
                                        assumed_base: true,
                                        indir: None,
                                        bound,
                                    },
                                    (
                                        JtVal::Const(base_va),
                                        JtVal::TableOff32 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: None,
                                            indir,
                                            bound,
                                        },
                                    ) => JtVal::Target32 {
                                        base_va,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: false,
                                        indir,
                                        bound,
                                    },
                                    (
                                        _,
                                        JtVal::TableOff32 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: Some(breg),
                                            indir,
                                            bound,
                                        },
                                    ) if breg as usize == dst => JtVal::Target32 {
                                        base_va: img_base,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: true,
                                        indir,
                                        bound,
                                    },
                                    (JtVal::Index { bound }, JtVal::Const(imm)) => {
                                        let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                        adjust_index_bound(bound, sd.saturating_neg())
                                    }
                                    _ => JtVal::Unknown,
                                },
                                None => JtVal::Unknown,
                            }
                        } else if let Some(imm) = op_imm(ins, 1) {
                            match regs[dst] {
                                JtVal::Const(c) => {
                                    if is32 {
                                        JtVal::Const((c as u32).wrapping_sub(imm as u32) as u64)
                                    } else {
                                        JtVal::Const(c.wrapping_sub(imm))
                                    }
                                }
                                JtVal::Index { bound } => {
                                    let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                    adjust_index_bound(bound, sd.saturating_neg())
                                }
                                _ => JtVal::Unknown,
                            }
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Dec => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = match regs[dst] {
                            JtVal::Const(c) => {
                                if is32 {
                                    JtVal::Const((c as u32).wrapping_sub(1) as u64)
                                } else {
                                    JtVal::Const(c.wrapping_sub(1))
                                }
                            }
                            JtVal::Index { bound } => adjust_index_bound(bound, -1),
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Inc => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = match regs[dst] {
                            JtVal::Const(c) => {
                                if is32 {
                                    JtVal::Const((c as u32).wrapping_add(1) as u64)
                                } else {
                                    JtVal::Const(c.wrapping_add(1))
                                }
                            }
                            JtVal::Index { bound } => adjust_index_bound(bound, 1),
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Xor => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = if ins.op_kind(1) == K::Register
                            && gpr_idx(ins.op1_register()) == Some(dst)
                        {
                            JtVal::Const(0)
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Shl => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = match op_imm(ins, 1) {
                            Some(1) => {
                                let (indir, bound) = extract_idx(regs[dst]);
                                JtVal::ScaledIndex { scale: 2, indir, bound }
                            }
                            Some(2) => {
                                let (indir, bound) = extract_idx(regs[dst]);
                                JtVal::ScaledIndex { scale: 4, indir, bound }
                            }
                            Some(3) => {
                                let (indir, bound) = extract_idx(regs[dst]);
                                JtVal::ScaledIndex { scale: 8, indir, bound }
                            }
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::And => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        clobber_alias(&mut alias, dst);
                        regs[dst] = match op_imm(ins, 1) {
                            Some(imm) if imm > 0 && imm < MAX_JT_ENTRIES as u64 => {
                                JtVal::Index { bound: Some(imm as usize + 1) }
                            }
                            _ => JtVal::Unknown,
                        };
                    }
                }
            }
            M::Test
            | M::Bt
            | M::Nop
            | M::Push
            | M::Jmp
            | M::Ret
            | M::Int3
            | M::Ud2
            | M::Pause
            | M::Mfence
            | M::Lfence
            | M::Sfence => {}
            M::Call => {
                // Win64 ABI volatile registers are clobbered; callee-saved regs stay intact.
                for r in [0usize, 1, 2, 8, 9, 10, 11] {
                    clobber_alias(&mut alias, r);
                    regs[r] = JtVal::Unknown;
                    clear_cmp_reg(&mut last_cmp, r);
                }
            }
            M::Xchg => {
                for op in [0u32, 1] {
                    if ins.op_kind(op) == K::Register {
                        if let Some(r) = gpr_idx(ins.op_register(op)) {
                            clobber_alias(&mut alias, r);
                            regs[r] = JtVal::Unknown;
                            clear_cmp_reg(&mut last_cmp, r);
                        }
                    }
                }
            }
            M::Mul | M::Div | M::Idiv | M::Cdq | M::Cqo => {
                for r in [0usize, 2] {
                    clobber_alias(&mut alias, r);
                    regs[r] = JtVal::Unknown;
                    clear_cmp_reg(&mut last_cmp, r);
                }
            }
            _ if cmov_cc(mn).is_some() => {
                // Branchless switch-index clamping (Spectre-safe / MSVC / Clang):
                // e.g. `cmp eax, N; mov ecx, def; cmova eax, ecx` or `mov ecx, def; cmp eax, N; cmovb ecx, eax`.
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let raw_cc = cmov_cc(mn).unwrap();
                        let (src_reg, src_val) = if ins.op_kind(1) == K::Register {
                            match gpr_idx(ins.op1_register()) {
                                Some(sr) => (Some(sr), regs[sr]),
                                None => (None, JtVal::Unknown),
                            }
                        } else {
                            (None, JtVal::Unknown)
                        };
                        let mut clamped: Option<usize> = None;
                        if let Some((mask, imm, rev)) = last_cmp {
                            let cc = if rev { flip_cmp_cc(raw_cc) } else { raw_cc };
                            let bound_le =
                                (imm < MAX_JT_ENTRIES as u64).then(|| imm as usize + 1);
                            let bound_lt = (imm > 0 && imm <= MAX_JT_ENTRIES as u64)
                                .then(|| imm as usize);
                            if (mask & (1u16 << dst)) != 0 {
                                let keep_b = match cc {
                                    CC::a | CC::g => bound_le,
                                    CC::ae | CC::ge => bound_lt,
                                    _ => None,
                                };
                                if let (Some(kb), Some(sb)) = (keep_b, jt_upper_bound(src_val)) {
                                    clamped = Some(kb.max(sb));
                                }
                            } else if let Some(sr) = src_reg {
                                if (mask & (1u16 << sr)) != 0 {
                                    let move_b = match cc {
                                        CC::b | CC::l => bound_lt,
                                        CC::be | CC::le => bound_le,
                                        _ => None,
                                    };
                                    if let (Some(mb), Some(db)) = (move_b, jt_upper_bound(regs[dst]))
                                    {
                                        clamped = Some(mb.max(db));
                                    }
                                }
                            }
                        }
                        if clamped.is_none() {
                            if let (Some(db), Some(sb)) =
                                (jt_upper_bound(regs[dst]), jt_upper_bound(src_val))
                            {
                                clamped = Some(db.max(sb));
                            }
                        }
                        clobber_alias(&mut alias, dst);
                        regs[dst] = match clamped {
                            Some(b) => JtVal::Index { bound: Some(b.min(MAX_JT_ENTRIES)) },
                            None => JtVal::Unknown,
                        };
                        clear_cmp_reg(&mut last_cmp, dst);
                    }
                }
            }
            _ => {
                if ins.flow_control() != F::ConditionalBranch
                    && ins.op_count() > 0
                    && ins.op_kind(0) == K::Register
                {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        clobber_alias(&mut alias, dst);
                        regs[dst] = JtVal::Unknown;
                        clear_cmp_reg(&mut last_cmp, dst);
                    }
                }
            }
        }
        if mid_resume {
            // Mid-block `!jit::supported` op (e.g. `ldmxcsr`, `vldmxcsr`, `xgetbv`, `cpuid`):
            // `prep_block_with` splits the AOT block right before it and execution resumes at
            // `ins.next_ip()` after the interpreter steps that single instruction.
            step.edges.push((ins.next_ip(), EdgeKind::Intra, pack_hint(&regs)));
        }
    }

    match fc {
        F::Next | F::IndirectCall => {
            step.edges.push((next, EdgeKind::Intra, pack_hint(&regs)));
        }
        F::Call => {
            step.edges.push((next, EdgeKind::Intra, pack_hint(&regs)));
            if tgt != 0 {
                step.edges.push((tgt, EdgeKind::DirectCall, None));
            }
        }
        F::ConditionalBranch => {
            let mut taken_regs = regs;
            let mut fall_regs = regs;
            if let Some((mask, imm, rev)) = last_cmp {
                let bound_le = (imm < MAX_JT_ENTRIES as u64).then(|| imm as usize + 1);
                let bound_lt = (imm > 0 && imm <= MAX_JT_ENTRIES as u64).then(|| imm as usize);
                let cc = if !rev {
                    last.condition_code()
                } else {
                    // `cmp const, reg` swaps the direction of ordered comparisons.
                    flip_cmp_cc(last.condition_code())
                };
                let (taken_b, fall_b) = match cc {
                    CC::a | CC::g => (None, bound_le),
                    CC::ae | CC::ge => (None, bound_lt),
                    CC::be | CC::le => (bound_le, None),
                    CC::b | CC::l => (bound_lt, None),
                    _ => (None, None),
                };
                if let Some(b) = taken_b {
                    for (r, v) in taken_regs.iter_mut().enumerate() {
                        if mask & (1u16 << r) != 0 {
                            *v = JtVal::Index { bound: Some(b) };
                        }
                    }
                }
                if let Some(b) = fall_b {
                    for (r, v) in fall_regs.iter_mut().enumerate() {
                        if mask & (1u16 << r) != 0 {
                            *v = JtVal::Index { bound: Some(b) };
                        }
                    }
                }
            }
            step.edges.push((tgt, EdgeKind::Intra, pack_hint(&taken_regs)));
            step.edges.push((next, EdgeKind::Intra, pack_hint(&fall_regs)));
        }
        F::UnconditionalBranch if tgt != 0 => {
            step.edges.push((tgt, EdgeKind::TailBranch, pack_hint(&regs)));
        }
        F::IndirectBranch if last.mnemonic() == M::Jmp => {
            step.ends_indirect_jmp = true;
            let jt_kind = match last.op_kind(0) {
                K::Register => {
                    gpr_idx(last.op0_register()).map(|r| regs[r]).unwrap_or(JtVal::Unknown)
                }
                K::Memory => eval_mem_load(last, &regs, false, img_base, img_size),
                _ => JtVal::Unknown,
            };
            if matches!(
                jt_kind,
                JtVal::Target16 { .. } | JtVal::Target32 { .. } | JtVal::Target64 { .. }
            ) {
                let cur_func = fmap.root_of(ip.wrapping_sub(img_base) as u32);
                let targets = resolve_jt_targets(jt_kind, base, size, text, fmap, cur_func);
                step.jt_targets = targets.len();
                for t in targets {
                    step.edges.push((t, EdgeKind::JumpTable, None));
                }
            }
        }
        _ => {}
    }
    push_lea_leaf_edges(insns, img_base, img_size, text, fmap, &mut step.edges);
    step
}
