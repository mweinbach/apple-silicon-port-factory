//! Instruction-level lowering (`x86-64 -> Native Rust/ARM64`) for the
//! Static AOT Compiler.

use super::abi::IatCallAbi;
use super::operands::*;
use super::AotImportBinding;
use anyhow::Result;
use iced_x86::{Code, ConditionCode, Instruction, Mnemonic, OpKind, Register};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

pub(super) fn is_valid_rust_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !matches!(
            s,
            "fn" | "let" | "mut" | "pub" | "mod" | "use" | "struct" | "enum" | "impl"
                | "trait" | "type" | "where" | "for" | "loop" | "while" | "if" | "else"
                | "match" | "return" | "break" | "continue" | "unsafe" | "extern" | "crate"
                | "self" | "super" | "in" | "as" | "ref" | "move" | "box" | "const" | "static"
        )
}

pub(super) fn is_block_terminator(ins: &Instruction) -> bool {
    if matches!(
        ins.mnemonic(),
        Mnemonic::Ret | Mnemonic::Jmp | Mnemonic::Int3 | Mnemonic::Ud2
    ) {
        return true;
    }
    ins.condition_code() != ConditionCode::None
        && matches!(
            ins.op0_kind(),
            OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
        )
}

/// Emit a direct Win32 IAT `call` or tail `jmp` using the import's classified Win64 ABI.
///
/// - For known pure-integer/pointer Win64 imports (`IatCallAbi::IntArgs(n)`):
///   - On `call`: forwards `rcx..r9` plus stack arguments `5..=n` from `[rsp + 0x20..]`
///     (e.g. `ReadFileEx`'s 5th `lpCompletionRoutine` pointer at `[rsp + 0x20]`) -> `Ok(true)`.
///   - On tail `jmp`: exact only when `n <= 4` (the standalone 4-GPR function signature did
///     not receive caller stack arguments 5+); for `n > 4` emits a best-effort call and
///     honestly returns `Ok(false)` (`fully_lowered = false`).
/// - For `IatCallAbi::Fallback` (variadic, float/SIMD, hidden `sret`, or unknown signature):
///   emits a best-effort bridge and honestly returns `Ok(false)` so `port-loader` defers
///   the function to the full ARM64 JIT instead of silently dropping arguments.
fn emit_iat_call(
    out: &mut String,
    iat_rva: u32,
    imp: &AotImportBinding,
    is_tail_jmp: bool,
) -> Result<bool> {
    let stack_base = if is_tail_jmp { "0x28" } else { "0x20" };
    let assign_or_ret = if is_tail_jmp { "return " } else { "rax = " };
    match imp.call_abi {
        IatCallAbi::IntArgs(arity) => {
            let exact = !is_tail_jmp || arity <= 4;
            if exact {
                writeln!(
                    out,
                    "                    // Direct Win32 IAT {} (int_args={arity}): {}!{} (win32={:?})",
                    if is_tail_jmp { "tail-jmp" } else { "call" },
                    imp.canonical_dll,
                    imp.symbol,
                    imp.rust_path
                )?;
            } else {
                writeln!(
                    out,
                    "                    // [AOT_FALLBACK] tail-jmp to {arity}-arg IAT {}!{} exceeds 4-GPR standalone entry ABI",
                    imp.canonical_dll, imp.symbol
                )?;
            }
            writeln!(
                out,
                "                    {assign_or_ret}call_iat_args(0x{iat_rva:08X}, rcx, rdx, r8, r9, rsp, {stack_base}, {arity});"
            )?;
            Ok(exact)
        }
        IatCallAbi::Fallback => {
            writeln!(
                out,
                "                    // [AOT_FALLBACK] IAT {} {}!{} has non-4GPR/float/variadic/unknown ABI",
                if is_tail_jmp { "tail-jmp" } else { "call" },
                imp.canonical_dll,
                imp.symbol
            )?;
            writeln!(
                out,
                "                    {assign_or_ret}call_iat_args(0x{iat_rva:08X}, rcx, rdx, r8, r9, rsp, {stack_base}, 8);"
            )?;
            Ok(false)
        }
    }
}

/// Lower one `iced_x86::Instruction` into Rust statements inside the basic block.
/// Returns `Ok(true)` if lowered to exact semantics, or `Ok(false)` if the
/// instruction is unsupported or only approximated (emitted as a no-op comment or
/// best-effort code). `Ok(false)` marks the function as not `fully_lowered`.
#[allow(clippy::too_many_arguments)]
pub(super) fn lower_instruction(
    out: &mut String,
    ins: &Instruction,
    image_base: u64,
    next_blk_rva: Option<u32>,
    local_insn_rvas: &BTreeSet<u32>,
    known_fn_rvas: &BTreeSet<u32>,
    iat_thunks: &HashMap<u32, u32>,
    imports_by_rva: &BTreeMap<u32, AotImportBinding>,
    terminated: &mut bool,
) -> Result<bool> {
    let exact = lower_instruction_inner(
        out,
        ins,
        image_base,
        next_blk_rva,
        local_insn_rvas,
        known_fn_rvas,
        iat_thunks,
        imports_by_rva,
        terminated,
    )?;
    // PF is not tracked (`p`/`np` evaluate to constants), so anything consuming
    // parity (jp/jnp/setp/cmovp/...) is only approximated.
    let uses_parity = matches!(
        ins.condition_code(),
        ConditionCode::p | ConditionCode::np
    );
    Ok(exact && !uses_parity)
}

#[allow(clippy::too_many_arguments)]
fn lower_instruction_inner(
    out: &mut String,
    ins: &Instruction,
    image_base: u64,
    next_blk_rva: Option<u32>,
    local_insn_rvas: &BTreeSet<u32>,
    known_fn_rvas: &BTreeSet<u32>,
    iat_thunks: &HashMap<u32, u32>,
    imports_by_rva: &BTreeMap<u32, AotImportBinding>,
    terminated: &mut bool,
) -> Result<bool> {
    let mn = ins.mnemonic();
    match mn {
        // lfence/sfence are subsumed by the TSO (acquire/release) rd*/wr* helpers.
        Mnemonic::Nop
        | Mnemonic::Endbr64
        | Mnemonic::Endbr32
        | Mnemonic::Prefetcht0
        | Mnemonic::Prefetcht1
        | Mnemonic::Prefetcht2
        | Mnemonic::Prefetchnta
        | Mnemonic::Prefetchw
        | Mnemonic::Pause
        | Mnemonic::Lfence
        | Mnemonic::Sfence => {
            return Ok(true);
        }
        Mnemonic::Mfence => {
            writeln!(
                out,
                "                    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);"
            )?;
            return Ok(true);
        }
        Mnemonic::Int3 | Mnemonic::Ud2 => {
            writeln!(out, "                    return rax;")?;
            *terminated = true;
            return Ok(true);
        }
        Mnemonic::Ret => {
            let imm = if ins.op_count() > 0 && ins.op0_kind() == OpKind::Immediate16 {
                ins.immediate16() as u64
            } else {
                0
            };
            writeln!(
                out,
                "                    let _ret_addr = rd64s(rsp); rsp = rsp.wrapping_add({}); return rax;",
                8 + imm
            )?;
            *terminated = true;
            return Ok(true);
        }
        // Lifter-owned stack slots (push/pop/ret/leave/enter) are thread-private and
        // use the plain rd64s/wr64s accessors instead of the TSO ones.
        Mnemonic::Push => {
            let val = read_op_u64(ins, 0, 64, image_base);
            writeln!(
                out,
                "                    {{ let _v = {val}; rsp = rsp.wrapping_sub(8); wr64s(rsp, _v); }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Pop => {
            let write = write_op_u64(ins, 0, 64, "_v", image_base);
            writeln!(
                out,
                "                    {{ let _v = rd64s(rsp); rsp = rsp.wrapping_add(8); {write} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Leave => {
            writeln!(
                out,
                "                    rsp = rbp; rbp = rd64s(rsp); rsp = rsp.wrapping_add(8);"
            )?;
            return Ok(true);
        }
        Mnemonic::Call => {
            let arg_rcx = "rcx";
            let arg_rdx = "rdx";
            let arg_r8 = "r8";
            let arg_r9 = "r9";
            if matches!(
                ins.op0_kind(),
                OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
            ) {
                let target_va = ins.near_branch_target();
                if let Some(rel) = target_va.checked_sub(image_base) {
                    let target_rva = rel as u32;
                    if let Some(&iat_rva) = iat_thunks.get(&target_rva) {
                        let imp = &imports_by_rva[&iat_rva];
                        return emit_iat_call(out, iat_rva, imp, false);
                    }
                    if known_fn_rvas.contains(&target_rva) {
                        writeln!(
                            out,
                            "                    rax = fn_rva_{target_rva:08x}({arg_rcx}, {arg_rdx}, {arg_r8}, {arg_r9});"
                        )?;
                        return Ok(true);
                    }
                }
                writeln!(
                    out,
                    "                    rax = call_indirect(0x{target_va:016X}, {arg_rcx}, {arg_rdx}, {arg_r8}, {arg_r9});"
                )?;
                return Ok(true);
            }
            if ins.op0_kind() == OpKind::Memory && ins.memory_base() == Register::RIP {
                let target_va = ins.ip_rel_memory_address();
                if let Some(rel) = target_va.checked_sub(image_base) {
                    let iat_rva = rel as u32;
                    if let Some(imp) = imports_by_rva.get(&iat_rva) {
                        return emit_iat_call(out, iat_rva, imp, false);
                    }
                }
            }
            let target_expr = read_op_u64(ins, 0, 64, image_base);
            writeln!(
                out,
                "                    rax = call_indirect({target_expr}, {arg_rcx}, {arg_rdx}, {arg_r8}, {arg_r9});"
            )?;
            return Ok(true);
        }
        Mnemonic::Jmp => {
            if matches!(
                ins.op0_kind(),
                OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
            ) {
                let target_va = ins.near_branch_target();
                if let Some(rel) = target_va.checked_sub(image_base) {
                    let target_rva = rel as u32;
                    if local_insn_rvas.contains(&target_rva) {
                        writeln!(out, "                    pc = 0x{target_rva:08X};")?;
                        *terminated = true;
                        return Ok(true);
                    }
                    if let Some(&iat_rva) = iat_thunks.get(&target_rva) {
                        let imp = &imports_by_rva[&iat_rva];
                        *terminated = true;
                        return emit_iat_call(out, iat_rva, imp, true);
                    }
                    if known_fn_rvas.contains(&target_rva) {
                        writeln!(
                            out,
                            "                    return fn_rva_{target_rva:08x}(rcx, rdx, r8, r9);"
                        )?;
                        *terminated = true;
                        return Ok(true);
                    }
                }
                writeln!(
                    out,
                    "                    return call_indirect(0x{target_va:016X}, rcx, rdx, r8, r9);"
                )?;
                *terminated = true;
                return Ok(true);
            }
            if ins.op0_kind() == OpKind::Memory && ins.memory_base() == Register::RIP {
                let target_va = ins.ip_rel_memory_address();
                if let Some(rel) = target_va.checked_sub(image_base) {
                    let iat_rva = rel as u32;
                    if let Some(imp) = imports_by_rva.get(&iat_rva) {
                        *terminated = true;
                        return emit_iat_call(out, iat_rva, imp, true);
                    }
                }
            }
            let target_expr = read_op_u64(ins, 0, 64, image_base);
            writeln!(
                out,
                "                    {{ let _t = {target_expr}; let _b = image_base(); if _t >= _b && _t < _b + SIZE_OF_IMAGE as u64 {{ pc = (_t - _b) as u32; }} else {{ return call_indirect(_t, rcx, rdx, r8, r9); }} }}"
            )?;
            *terminated = true;
            return Ok(true);
        }
        _ if ins.condition_code() != ConditionCode::None
            && matches!(
                ins.op0_kind(),
                OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
            ) =>
        {
            let cond = cond_expr(ins.condition_code());
            let target_va = ins.near_branch_target();
            let target_rva = target_va.saturating_sub(image_base) as u32;
            let fallthrough = next_blk_rva.unwrap_or(0);
            if local_insn_rvas.contains(&target_rva) {
                writeln!(
                    out,
                    "                    if {cond} {{ pc = 0x{target_rva:08X}; }} else {{ pc = 0x{fallthrough:08X}; }}"
                )?;
            } else if known_fn_rvas.contains(&target_rva) {
                writeln!(
                    out,
                    "                    if {cond} {{ return fn_rva_{target_rva:08x}(rcx, rdx, r8, r9); }} else {{ pc = 0x{fallthrough:08X}; }}"
                )?;
            } else {
                writeln!(
                    out,
                    "                    if {cond} {{ pc = 0x{target_rva:08X}; }} else {{ pc = 0x{fallthrough:08X}; }}"
                )?;
            }
            *terminated = true;
            return Ok(true);
        }
        Mnemonic::Lea => {
            let bits = op_bits(ins, 0);
            let addr = mem_addr_expr(ins, image_base);
            let wr = write_op_u64(ins, 0, bits, "_ea", image_base);
            writeln!(out, "                    {{ let _ea = {addr}; {wr} }}")?;
            return Ok(true);
        }
        Mnemonic::Mov | Mnemonic::Movnti => {
            let bits = op_bits(ins, 0);
            if bits == 128 {
                let src = read_op_u128(ins, 1, image_base);
                let wr = write_op_u128(ins, 0, "_v", image_base);
                writeln!(out, "                    {{ let _v = {src}; {wr} }}")?;
            } else {
                let src = read_op_u64(ins, 1, bits, image_base);
                let wr = write_op_u64(ins, 0, bits, "_v", image_base);
                writeln!(out, "                    {{ let _v = {src}; {wr} }}")?;
            }
            return Ok(true);
        }
        Mnemonic::Movzx => {
            let dst_bits = op_bits(ins, 0);
            let src_bits = op_bits(ins, 1);
            let src = read_op_u64(ins, 1, src_bits, image_base);
            let wr = write_op_u64(ins, 0, dst_bits, "_v", image_base);
            writeln!(out, "                    {{ let _v = {src}; {wr} }}")?;
            return Ok(true);
        }
        Mnemonic::Movsx | Mnemonic::Movsxd => {
            let dst_bits = op_bits(ins, 0);
            let src_bits = op_bits(ins, 1);
            let src = read_op_u64(ins, 1, src_bits, image_base);
            let sext = sign_extend_expr(&src, src_bits);
            let wr = write_op_u64(ins, 0, dst_bits, "_v", image_base);
            writeln!(out, "                    {{ let _v = {sext}; {wr} }}")?;
            return Ok(true);
        }
        Mnemonic::Xchg => {
            let bits = op_bits(ins, 0);
            let mem_op = if is_mem(ins, 0) {
                Some(0)
            } else if is_mem(ins, 1) {
                Some(1)
            } else {
                None
            };
            if let Some(m) = mem_op {
                // `xchg` with a memory operand is implicitly locked (full barrier).
                let r = 1 - m;
                let mask = mask_lit(bits);
                let addr = mem_addr_expr(ins, image_base);
                let v = read_op_u64(ins, r, bits, image_base);
                let wr = write_op_u64(ins, r, bits, "_old", image_base);
                let f = lk("xchg", bits);
                writeln!(
                    out,
                    "                    {{ let _v = ({v}) & {mask}; let _old = {f}({addr}, _v); {wr} }}"
                )?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let b = read_op_u64(ins, 1, bits, image_base);
            let w0 = write_op_u64(ins, 0, bits, "_b", image_base);
            let w1 = write_op_u64(ins, 1, bits, "_a", image_base);
            writeln!(
                out,
                "                    {{ let _a = {a}; let _b = {b}; {w0} {w1} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Add | Mnemonic::Adc | Mnemonic::Sub | Mnemonic::Sbb | Mnemonic::Cmp => {
            let bits = op_bits(ins, 0);
            let is_sub = matches!(mn, Mnemonic::Sub | Mnemonic::Sbb | Mnemonic::Cmp);
            let carry_in = if matches!(mn, Mnemonic::Adc | Mnemonic::Sbb) {
                "if cf { 1u64 } else { 0u64 }"
            } else {
                "0u64"
            };
            let b = read_op_u64(ins, 1, bits, image_base);
            if is_locked_rmw(ins) && mn != Mnemonic::Cmp {
                // Fetch-op returns the OLD value; flags are recomputed from it.
                let mask = mask_lit(bits);
                let addr = mem_addr_expr(ins, image_base);
                let f = lk(if is_sub { "sub" } else { "add" }, bits);
                writeln!(
                    out,
                    "                    {{ let _bv = ({b}) & {mask}; let _cv = {carry_in}; let _old = {f}({addr}, _bv.wrapping_add(_cv) & {mask});"
                )?;
                emit_arith_add_sub(out, bits, "_old", "_bv", "_cv", is_sub, "", false)?;
                writeln!(out, "                    }}")?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let write_back = mn != Mnemonic::Cmp;
            let wr = if write_back {
                write_op_u64(ins, 0, bits, "_r", image_base)
            } else {
                String::new()
            };
            emit_arith_add_sub(out, bits, &a, &b, carry_in, is_sub, &wr, write_back)?;
            return Ok(true);
        }
        Mnemonic::Inc | Mnemonic::Dec => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let sign_shift = bits - 1;
            let (op_fn, of_expr) = if mn == Mnemonic::Inc {
                ("wrapping_add(1)", format!("_r == (1u64 << {sign_shift})"))
            } else {
                ("wrapping_sub(1)", format!("_a == (1u64 << {sign_shift})"))
            };
            let flags = format!(
                "zf = _r == 0; sf = (_r >> {sign_shift}) != 0; of = {of_expr};"
            );
            if is_locked_rmw(ins) {
                let addr = mem_addr_expr(ins, image_base);
                let f = lk(if mn == Mnemonic::Inc { "add" } else { "sub" }, bits);
                writeln!(
                    out,
                    "                    {{ let _a = {f}({addr}, 1) & {mask}; let _r = _a.{op_fn} & {mask}; {flags} }}"
                )?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            writeln!(
                out,
                "                    {{ let _a = ({a}) & {mask}; let _r = _a.{op_fn} & {mask}; {flags} {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Neg => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let sign_shift = bits - 1;
            let flags = format!(
                "cf = _a != 0; zf = _r == 0; sf = (_r >> {sign_shift}) != 0; of = _a == (1u64 << {sign_shift});"
            );
            if is_locked_rmw(ins) {
                let addr = mem_addr_expr(ins, image_base);
                let f = lk("rmw", bits);
                writeln!(
                    out,
                    "                    {{ let _a = {f}({addr}, |_x| _x.wrapping_neg()) & {mask}; let _r = (0u64.wrapping_sub(_a)) & {mask}; {flags} }}"
                )?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            writeln!(
                out,
                "                    {{ let _a = ({a}) & {mask}; let _r = (0u64.wrapping_sub(_a)) & {mask}; {flags} {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Not => {
            let bits = op_bits(ins, 0);
            if is_locked_rmw(ins) {
                let addr = mem_addr_expr(ins, image_base);
                let f = lk("rmw", bits);
                writeln!(out, "                    {{ let _ = {f}({addr}, |_x| !_x); }}")?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            let mask = mask_lit(bits);
            writeln!(
                out,
                "                    {{ let _r = (!({a})) & {mask}; {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::And | Mnemonic::Or | Mnemonic::Xor | Mnemonic::Test => {
            let bits = op_bits(ins, 0);
            if mn == Mnemonic::Xor
                && ins.op0_kind() == OpKind::Register
                && ins.op1_kind() == OpKind::Register
                && ins.op0_register() == ins.op1_register()
            {
                let wr = write_op_u64(ins, 0, bits, "0", image_base);
                writeln!(
                    out,
                    "                    {{ {wr} cf = false; of = false; zf = true; sf = false; }}"
                )?;
                return Ok(true);
            }
            let b = read_op_u64(ins, 1, bits, image_base);
            let mask = mask_lit(bits);
            let sign_shift = bits - 1;
            let op_sym = match mn {
                Mnemonic::And | Mnemonic::Test => "&",
                Mnemonic::Or => "|",
                Mnemonic::Xor => "^",
                _ => unreachable!(),
            };
            let flags = format!(
                "cf = false; of = false; zf = _r == 0; sf = (_r >> {sign_shift}) != 0;"
            );
            if is_locked_rmw(ins) && mn != Mnemonic::Test {
                let addr = mem_addr_expr(ins, image_base);
                let f = lk(
                    match mn {
                        Mnemonic::And => "and",
                        Mnemonic::Or => "or",
                        _ => "xor",
                    },
                    bits,
                );
                writeln!(
                    out,
                    "                    {{ let _bv = ({b}) & {mask}; let _old = {f}({addr}, _bv); let _r = (_old {op_sym} _bv) & {mask}; {flags} }}"
                )?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr = if mn == Mnemonic::Test {
                String::new()
            } else {
                write_op_u64(ins, 0, bits, "_r", image_base)
            };
            writeln!(
                out,
                "                    {{ let _r = (({a}) {op_sym} ({b})) & {mask}; {flags} {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Shl | Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar | Mnemonic::Rol | Mnemonic::Ror => {
            let bits = op_bits(ins, 0);
            let a = read_op_u64(ins, 0, bits, image_base);
            let cnt = if ins.op_count() > 1 {
                read_op_u64(ins, 1, 8, image_base)
            } else {
                "1u64".to_string()
            };
            let shift_mask = if bits == 64 { 63 } else { 31 };
            let mask = mask_lit(bits);
            let s = bits - 1;
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            let sext = sign_extend_expr("_a", bits);
            let msb_r = format!("(((_r >> {s}) & 1) != 0)");
            let (expr, flags) = match mn {
                Mnemonic::Shl | Mnemonic::Sal => (
                    format!("(_a.wrapping_shl(_c)) & {mask}"),
                    format!(
                        "cf = _c <= {bits} && ((_a >> ({bits} - _c)) & 1) != 0; zf = _r == 0; sf = {msb_r}; of = {msb_r} != cf;"
                    ),
                ),
                Mnemonic::Shr => (
                    format!("(_a & {mask}).wrapping_shr(_c)"),
                    format!(
                        "cf = ((_a & {mask}).wrapping_shr(_c - 1) & 1) != 0; zf = _r == 0; sf = {msb_r}; of = ((_a >> {s}) & 1) != 0;"
                    ),
                ),
                Mnemonic::Sar => (
                    format!("(({sext} as i64).wrapping_shr(_c) as u64) & {mask}"),
                    format!(
                        "cf = ((({sext} as i64).wrapping_shr(_c - 1) as u64) & 1) != 0; zf = _r == 0; sf = {msb_r}; of = false;"
                    ),
                ),
                // Rotates only touch CF/OF.
                Mnemonic::Rol => (
                    format!(
                        "(((_a & {mask}).wrapping_shl(_c % {bits})) | ((_a & {mask}).wrapping_shr(({bits} - (_c % {bits})) % {bits}))) & {mask}"
                    ),
                    format!("cf = (_r & 1) != 0; of = {msb_r} != cf;"),
                ),
                Mnemonic::Ror => (
                    format!(
                        "(((_a & {mask}).wrapping_shr(_c % {bits})) | ((_a & {mask}).wrapping_shl(({bits} - (_c % {bits})) % {bits}))) & {mask}"
                    ),
                    format!(
                        "cf = {msb_r}; of = cf != (((_r >> {}) & 1) != 0);",
                        s - 1
                    ),
                ),
                _ => unreachable!(),
            };
            writeln!(
                out,
                "                    {{ let _a = ({a}) & {mask}; let _c = (({cnt}) as u32) & {shift_mask}; let _r = if _c == 0 {{ _a }} else {{ {expr} }}; if _c != 0 {{ {flags} }} {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Shld | Mnemonic::Shrd => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let s = bits - 1;
            let d = read_op_u64(ins, 0, bits, image_base);
            let src = read_op_u64(ins, 1, bits, image_base);
            let cnt = read_op_u64(ins, 2, 8, image_base);
            let shift_mask = if bits == 64 { 63 } else { 31 };
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            let (expr, cf_expr) = if mn == Mnemonic::Shld {
                (
                    format!("(((((_d as u128) << {bits}) | (_s as u128)) << _c) >> {bits}) as u64 & {mask}"),
                    format!("(((_d as u128) << _c) >> {bits}) & 1 != 0"),
                )
            } else {
                (
                    format!("((((_s as u128) << {bits}) | (_d as u128)) >> _c) as u64 & {mask}"),
                    "((_d as u128) >> (_c - 1)) & 1 != 0".to_string(),
                )
            };
            writeln!(
                out,
                "                    {{ let _d = ({d}) & {mask}; let _s = ({src}) & {mask}; let _c = (({cnt}) as u32) & {shift_mask}; if _c != 0 {{ let _r = {expr}; cf = {cf_expr}; zf = _r == 0; sf = ((_r >> {s}) & 1) != 0; of = ((_r ^ _d) >> {s}) & 1 != 0; {wr} }} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Imul | Mnemonic::Mul => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let signed = mn == Mnemonic::Imul;
            match ins.op_count() {
                1 => {
                    let src = read_op_u64(ins, 0, bits, image_base);
                    let prod = if signed {
                        let sa = sign_extend_expr(&format!("rax & {mask}"), bits);
                        let sb = sign_extend_expr(&format!("({src}) & {mask}"), bits);
                        format!("let _p = (({sa}) as i64 as i128) * (({sb}) as i64 as i128); let _lo = (_p as u64) & {mask}; let _hi = ((_p >> {bits}) as u64) & {mask}; cf = _p != (({}) as i64 as i128);", sign_extend_expr("_lo", bits))
                    } else {
                        format!("let _p = ((rax & {mask}) as u128) * ((({src}) & {mask}) as u128); let _lo = (_p as u64) & {mask}; let _hi = ((_p >> {bits}) as u64) & {mask}; cf = _hi != 0;")
                    };
                    let wb = match bits {
                        8 => "rax = (rax & !0xFFFF) | _lo | (_hi << 8);".to_string(),
                        16 => "rax = (rax & !0xFFFF) | _lo; rdx = (rdx & !0xFFFF) | _hi;".to_string(),
                        _ => "rax = _lo; rdx = _hi;".to_string(),
                    };
                    writeln!(out, "                    {{ {prod} of = cf; {wb} }}")?;
                }
                2 | 3 => {
                    let (x, y) = if ins.op_count() == 2 { (0, 1) } else { (1, 2) };
                    let a = read_op_u64(ins, x, bits, image_base);
                    let b = read_op_u64(ins, y, bits, image_base);
                    let sa = sign_extend_expr(&format!("({a}) & {mask}"), bits);
                    let sb = sign_extend_expr(&format!("({b}) & {mask}"), bits);
                    let sr = sign_extend_expr("_r", bits);
                    let wr = write_op_u64(ins, 0, bits, "_r", image_base);
                    writeln!(
                        out,
                        "                    {{ let _p = (({sa}) as i64 as i128) * (({sb}) as i64 as i128); let _r = (_p as u64) & {mask}; cf = _p != (({sr}) as i64 as i128); of = cf; {wr} }}"
                    )?;
                }
                _ => return Ok(false),
            }
            return Ok(true);
        }
        Mnemonic::Div | Mnemonic::Idiv => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let src = read_op_u64(ins, 0, bits, image_base);
            // Dividend is the double-width rdx:rax (ax for 8-bit), taken before any write.
            let n_u = match bits {
                8 => "(rax & 0xFFFF) as u128".to_string(),
                16 => "(((rdx & 0xFFFF) << 16) | (rax & 0xFFFF)) as u128".to_string(),
                32 => "(((rdx & 0xFFFF_FFFF) << 32) | (rax & 0xFFFF_FFFF)) as u128".to_string(),
                _ => "((rdx as u128) << 64) | (rax as u128)".to_string(),
            };
            let n_s = match bits {
                8 => "(rax as u16 as i16) as i128".to_string(),
                16 => "((((rdx & 0xFFFF) << 16) | (rax & 0xFFFF)) as u32 as i32) as i128".to_string(),
                32 => "((((rdx & 0xFFFF_FFFF) << 32) | (rax & 0xFFFF_FFFF)) as i64) as i128".to_string(),
                _ => "(((rdx as u128) << 64) | (rax as u128)) as i128".to_string(),
            };
            let wb = match bits {
                8 => "rax = (rax & !0xFFFF) | (_q & 0xFF) | ((_r & 0xFF) << 8);",
                16 => "rax = (rax & !0xFFFF) | _q; rdx = (rdx & !0xFFFF) | _r;",
                _ => "rax = _q; rdx = _r;",
            };
            // #DE (zero divisor / quotient overflow) is not modeled: registers stay unchanged.
            if mn == Mnemonic::Div {
                writeln!(
                    out,
                    "                    {{ let _dv = (({src}) & {mask}) as u128; let _n: u128 = {n_u}; if _dv != 0 && _n / _dv <= ({mask} as u128) {{ let _q = (_n / _dv) as u64; let _r = (_n % _dv) as u64; {wb} }} }}"
                )?;
            } else {
                let sd = sign_extend_expr(&format!("({src}) & {mask}"), bits);
                let s = bits - 1;
                writeln!(
                    out,
                    "                    {{ let _dv = (({sd}) as i64) as i128; let _n: i128 = {n_s}; if let Some(_qq) = _n.checked_div(_dv) {{ if _qq >= -(1i128 << {s}) && _qq < (1i128 << {s}) {{ let _q = (_qq as u64) & {mask}; let _r = (_n.wrapping_rem(_dv) as u64) & {mask}; {wb} }} }} }}"
                )?;
            }
            return Ok(true);
        }
        Mnemonic::Cbw => {
            writeln!(
                out,
                "                    rax = (rax & !0xFFFF) | ((rax as u8 as i8 as i16 as u16) as u64);"
            )?;
            return Ok(true);
        }
        Mnemonic::Cwde => {
            writeln!(out, "                    rax = (rax as u16 as i16 as i32 as u32) as u64;")?;
            return Ok(true);
        }
        Mnemonic::Cwd => {
            writeln!(
                out,
                "                    rdx = (rdx & !0xFFFF) | (if (rax & 0x8000) != 0 {{ 0xFFFF }} else {{ 0 }});"
            )?;
            return Ok(true);
        }
        Mnemonic::Cdqe => {
            writeln!(out, "                    rax = (rax as u32 as i32 as i64) as u64;")?;
            return Ok(true);
        }
        Mnemonic::Cdq => {
            writeln!(
                out,
                "                    rdx = if (rax & 0x8000_0000) != 0 {{ 0xFFFF_FFFF }} else {{ 0 }};"
            )?;
            return Ok(true);
        }
        Mnemonic::Cqo => {
            writeln!(
                out,
                "                    rdx = if (rax >> 63) != 0 {{ u64::MAX }} else {{ 0 }};"
            )?;
            return Ok(true);
        }
        Mnemonic::Bswap => {
            let bits = op_bits(ins, 0);
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            if bits == 64 {
                writeln!(
                    out,
                    "                    {{ let _r = ({a}).swap_bytes(); {wr} }}"
                )?;
            } else {
                writeln!(
                    out,
                    "                    {{ let _r = (({a}) as u32).swap_bytes() as u64; {wr} }}"
                )?;
            }
            return Ok(true);
        }
        Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let bm = bits - 1;
            let new_expr = match mn {
                Mnemonic::Bts => Some("_a | _m"),
                Mnemonic::Btr => Some("_a & !_m"),
                Mnemonic::Btc => Some("_a ^ _m"),
                _ => None,
            };
            let off = read_op_u64(ins, 1, bits, image_base);
            if is_mem(ins, 0) {
                // Memory bit string: a register offset is signed and may address outside
                // the operand (ea + (off >> log2(bits)) * bytes); an immediate is masked.
                let base = mem_addr_expr(ins, image_base);
                let addr_stmt = if ins.op1_kind() == OpKind::Register {
                    let soff = sign_extend_expr(&off, bits);
                    let shift = bits.trailing_zeros();
                    let bytes = bits / 8;
                    format!(
                        "let _off = {soff}; let _ea = ({base}).wrapping_add((((_off as i64) >> {shift}) as u64).wrapping_mul({bytes})); let _bit = (_off & {bm}) as u32;"
                    )
                } else {
                    format!("let _ea = {base}; let _bit = (({off}) & {bm}) as u32;")
                };
                match new_expr {
                    Some(_) if ins.has_lock_prefix() => {
                        let (f, arg) = match mn {
                            Mnemonic::Bts => ("or", "_m".to_string()),
                            Mnemonic::Btr => ("and", format!("!_m & {mask}")),
                            _ => ("xor", "_m".to_string()),
                        };
                        let f = lk(f, bits);
                        writeln!(
                            out,
                            "                    {{ {addr_stmt} let _m = 1u64 << _bit; let _a = {f}(_ea, {arg}); cf = ((_a >> _bit) & 1) != 0; }}"
                        )?;
                    }
                    Some(e) => {
                        let rd = rd_mem(bits, "_ea");
                        let wr = wr_mem(bits, "_ea", "_r");
                        writeln!(
                            out,
                            "                    {{ {addr_stmt} let _m = 1u64 << _bit; let _a = {rd}; cf = ((_a >> _bit) & 1) != 0; let _r = ({e}) & {mask}; {wr} }}"
                        )?;
                    }
                    None => {
                        let rd = rd_mem(bits, "_ea");
                        writeln!(
                            out,
                            "                    {{ {addr_stmt} let _a = {rd}; cf = ((_a >> _bit) & 1) != 0; }}"
                        )?;
                    }
                }
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let tail = match new_expr {
                Some(e) => {
                    let wr = write_op_u64(ins, 0, bits, "_r", image_base);
                    format!("let _r = ({e}) & {mask}; {wr}")
                }
                None => String::new(),
            };
            writeln!(
                out,
                "                    {{ let _a = ({a}) & {mask}; let _bit = (({off}) & {bm}) as u32; let _m = 1u64 << _bit; cf = ((_a >> _bit) & 1) != 0; {tail} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Tzcnt | Mnemonic::Bsf | Mnemonic::Lzcnt | Mnemonic::Bsr => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let src = read_op_u64(ins, 1, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            let body = match mn {
                Mnemonic::Tzcnt => format!(
                    "let _r = if _s == 0 {{ {bits}u64 }} else {{ _s.trailing_zeros() as u64 }}; cf = _s == 0; zf = _r == 0; {wr}"
                ),
                Mnemonic::Lzcnt => format!(
                    "let _r = if _s == 0 {{ {bits}u64 }} else {{ (_s.leading_zeros() - {}) as u64 }}; cf = _s == 0; zf = _r == 0; {wr}",
                    64 - bits
                ),
                // bsf/bsr leave the destination unchanged for a zero source.
                Mnemonic::Bsf => format!(
                    "zf = _s == 0; if !zf {{ let _r = _s.trailing_zeros() as u64; {wr} }}"
                ),
                _ => format!(
                    "zf = _s == 0; if !zf {{ let _r = (63 - _s.leading_zeros()) as u64; {wr} }}"
                ),
            };
            writeln!(out, "                    {{ let _s = ({src}) & {mask}; {body} }}")?;
            return Ok(true);
        }
        Mnemonic::Popcnt => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let src = read_op_u64(ins, 1, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            writeln!(
                out,
                "                    {{ let _s = ({src}) & {mask}; let _r = _s.count_ones() as u64; zf = _r == 0; cf = false; of = false; sf = false; {wr} }}"
            )?;
            return Ok(true);
        }
        _ if ins.condition_code() != ConditionCode::None && ins.op_count() == 2 => {
            let cond = cond_expr(ins.condition_code());
            let bits = op_bits(ins, 0);
            let src = read_op_u64(ins, 1, bits, image_base);
            let dst = read_op_u64(ins, 0, bits, image_base);
            let wr = write_op_u64(ins, 0, bits, "_v", image_base);
            writeln!(
                out,
                "                    {{ let _v = if {cond} {{ {src} }} else {{ {dst} }}; {wr} }}"
            )?;
            return Ok(true);
        }
        _ if ins.condition_code() != ConditionCode::None && ins.op_count() == 1 => {
            let cond = cond_expr(ins.condition_code());
            let wr = write_op_u64(ins, 0, 8, "_v", image_base);
            writeln!(
                out,
                "                    {{ let _v = if {cond} {{ 1u64 }} else {{ 0u64 }}; {wr} }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Movups
        | Mnemonic::Movaps
        | Mnemonic::Movupd
        | Mnemonic::Movapd
        | Mnemonic::Movdqu
        | Mnemonic::Movdqa
        | Mnemonic::Vmovups
        | Mnemonic::Vmovaps
        | Mnemonic::Vmovdqu
        | Mnemonic::Vmovdqa => {
            let src = read_op_u128(ins, 1, image_base);
            let wr = write_op_u128(ins, 0, "_v", image_base);
            writeln!(out, "                    {{ let _v = {src}; {wr} }}")?;
            // The xmm file is 128-bit; 256-bit forms only move the low lane.
            let is_256 = (0..ins.op_count()).any(|i| {
                (ins.op_kind(i) == OpKind::Register && ins.op_register(i).is_ymm())
                    || (ins.op_kind(i) == OpKind::Memory && ins.memory_size().size() == 32)
            });
            return Ok(!is_256);
        }
        // String instructions (DF is assumed clear; `std` is reported as unsupported).
        Mnemonic::Stosb
        | Mnemonic::Stosw
        | Mnemonic::Stosd
        | Mnemonic::Stosq
        | Mnemonic::Movsb
        | Mnemonic::Movsw
        | Mnemonic::Movsd
        | Mnemonic::Movsq
        | Mnemonic::Lodsb
        | Mnemonic::Lodsw
        | Mnemonic::Lodsd
        | Mnemonic::Lodsq
            if is_string_op(ins) =>
        {
            let n: u32 = match mn {
                Mnemonic::Stosb | Mnemonic::Movsb | Mnemonic::Lodsb => 1,
                Mnemonic::Stosw | Mnemonic::Movsw | Mnemonic::Lodsw => 2,
                Mnemonic::Stosd | Mnemonic::Movsd | Mnemonic::Lodsd => 4,
                _ => 8,
            };
            let bits = n * 8;
            let mask = mask_lit(bits);
            let ty = format!("u{bits}");
            let rep = ins.has_rep_prefix() || ins.has_repne_prefix();
            let stmt = match mn {
                Mnemonic::Stosb | Mnemonic::Stosw | Mnemonic::Stosd | Mnemonic::Stosq => {
                    if !rep {
                        let wr = wr_mem(bits, "rdi", &format!("rax & {mask}"));
                        format!("{{ {wr} rdi = rdi.wrapping_add({n}); }}")
                    } else if n == 1 {
                        "{ core::ptr::write_bytes(rdi as *mut u8, (rax & 0xFF) as u8, rcx as usize); rdi = rdi.wrapping_add(rcx); rcx = 0; }".to_string()
                    } else {
                        format!("{{ let _v = (rax & {mask}) as {ty}; while rcx != 0 {{ core::ptr::write_unaligned(rdi as *mut {ty}, _v); rdi = rdi.wrapping_add({n}); rcx = rcx.wrapping_sub(1); }} }}")
                    }
                }
                Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq => {
                    if !rep {
                        let rd = rd_mem(bits, "rsi");
                        let wr = wr_mem(bits, "rdi", "_v");
                        format!("{{ let _v = {rd}; {wr} rsi = rsi.wrapping_add({n}); rdi = rdi.wrapping_add({n}); }}")
                    } else {
                        // Forward element-wise semantics: memmove is equivalent unless the
                        // destination starts inside the source range (pattern-fill idiom).
                        format!("{{ let _bytes = rcx.wrapping_mul({n}); if rdi.wrapping_sub(rsi) >= _bytes {{ core::ptr::copy(rsi as *const u8, rdi as *mut u8, _bytes as usize); rsi = rsi.wrapping_add(_bytes); rdi = rdi.wrapping_add(_bytes); rcx = 0; }} else {{ while rcx != 0 {{ let _v = core::ptr::read_unaligned(rsi as *const {ty}); core::ptr::write_unaligned(rdi as *mut {ty}, _v); rsi = rsi.wrapping_add({n}); rdi = rdi.wrapping_add({n}); rcx = rcx.wrapping_sub(1); }} }} }}")
                    }
                }
                _ => {
                    let rd = rd_mem(bits, "rsi");
                    let wr = write_reg_u64(acc_reg(bits), "_v");
                    if !rep {
                        format!("{{ let _v = {rd}; {wr} rsi = rsi.wrapping_add({n}); }}")
                    } else {
                        format!("{{ while rcx != 0 {{ let _v = {rd}; {wr} rsi = rsi.wrapping_add({n}); rcx = rcx.wrapping_sub(1); }} }}")
                    }
                }
            };
            writeln!(out, "                    {stmt}")?;
            return Ok(true);
        }
        Mnemonic::Movss | Mnemonic::Movsd | Mnemonic::Movq | Mnemonic::Movd => {
            let w: u32 = if matches!(mn, Mnemonic::Movss | Mnemonic::Movd) { 32 } else { 64 };
            let lowmask = if w == 32 { "0xFFFF_FFFFu128" } else { "0xFFFF_FFFF_FFFF_FFFFu128" };
            let dst_xmm = ins.op0_kind() == OpKind::Register && is_xmm_reg(ins.op0_register());
            let src_xmm = ins.op1_kind() == OpKind::Register && is_xmm_reg(ins.op1_register());
            if dst_xmm {
                let d = xmm_index(ins.op0_register());
                if src_xmm && matches!(mn, Mnemonic::Movss | Mnemonic::Movsd) {
                    // reg-reg movss/movsd merge into the low lane, upper lanes preserved.
                    let s = xmm_index(ins.op1_register());
                    writeln!(
                        out,
                        "                    xmm[{d}] = (xmm[{d}] & !{lowmask}) | (xmm[{s}] & {lowmask});"
                    )?;
                } else {
                    // Loads from memory / GPR (and movq xmm,xmm) zero the upper lanes.
                    let src = if src_xmm {
                        format!("(xmm[{}] as u64)", xmm_index(ins.op1_register()))
                    } else {
                        read_op_u64(ins, 1, w, image_base)
                    };
                    writeln!(
                        out,
                        "                    xmm[{d}] = (({src}) as u128) & {lowmask};"
                    )?;
                }
                return Ok(true);
            }
            if src_xmm {
                let s = xmm_index(ins.op1_register());
                let wr = write_op_u64(ins, 0, w, "_v", image_base);
                let m = mask_lit(w);
                writeln!(
                    out,
                    "                    {{ let _v = (xmm[{s}] as u64) & {m}; {wr} }}"
                )?;
                return Ok(true);
            }
            // MMX forms are not modeled.
            writeln!(out, "                    // [AOT_UNIMPLEMENTED] mnemonic={mn:?} (mmx)")?;
            return Ok(false);
        }
        Mnemonic::Xorps | Mnemonic::Xorpd | Mnemonic::Pxor | Mnemonic::Vpxor | Mnemonic::Vxorps => {
            if ins.op0_kind() == OpKind::Register && is_xmm_reg(ins.op0_register()) && ins.op_count() == 2 {
                let d = xmm_index(ins.op0_register());
                if ins.op1_kind() == OpKind::Register && ins.op0_register() == ins.op1_register() {
                    writeln!(out, "                    xmm[{d}] = 0;")?;
                } else {
                    let s = read_op_u128(ins, 1, image_base);
                    writeln!(out, "                    xmm[{d}] ^= {s};")?;
                }
                return Ok(true);
            }
            writeln!(out, "                    // [AOT_UNIMPLEMENTED] mnemonic={mn:?}")?;
            return Ok(false);
        }
        Mnemonic::Pand | Mnemonic::Por | Mnemonic::Pandn => {
            if ins.op0_kind() == OpKind::Register && is_xmm_reg(ins.op0_register()) {
                let d = xmm_index(ins.op0_register());
                let s = read_op_u128(ins, 1, image_base);
                match mn {
                    Mnemonic::Pand => writeln!(out, "                    xmm[{d}] &= {s};")?,
                    Mnemonic::Por => writeln!(out, "                    xmm[{d}] |= {s};")?,
                    _ => writeln!(out, "                    xmm[{d}] = (!xmm[{d}]) & ({s});")?,
                }
                return Ok(true);
            }
            writeln!(out, "                    // [AOT_UNIMPLEMENTED] mnemonic={mn:?}")?;
            return Ok(false);
        }
        // SSE integer/float arithmetic, shuffles, compares and conversions are NOT
        // implemented: emit nothing and report the instruction as not lowered.
        Mnemonic::Paddb
        | Mnemonic::Paddw
        | Mnemonic::Paddd
        | Mnemonic::Paddq
        | Mnemonic::Psubb
        | Mnemonic::Psubw
        | Mnemonic::Psubd
        | Mnemonic::Psubq
        | Mnemonic::Psubusb
        | Mnemonic::Psubusw
        | Mnemonic::Pcmpeqb
        | Mnemonic::Pcmpeqw
        | Mnemonic::Pcmpeqd
        | Mnemonic::Pmovmskb
        | Mnemonic::Pshufd
        | Mnemonic::Psrldq
        | Mnemonic::Pslldq
        | Mnemonic::Psllw
        | Mnemonic::Pslld
        | Mnemonic::Psllq
        | Mnemonic::Psrlw
        | Mnemonic::Psrld
        | Mnemonic::Psrlq
        | Mnemonic::Psraw
        | Mnemonic::Psrad
        | Mnemonic::Punpcklbw
        | Mnemonic::Punpcklwd
        | Mnemonic::Punpckldq
        | Mnemonic::Punpcklqdq
        | Mnemonic::Punpckhbw
        | Mnemonic::Punpckhwd
        | Mnemonic::Punpckhdq
        | Mnemonic::Punpckhqdq
        | Mnemonic::Addss
        | Mnemonic::Subss
        | Mnemonic::Mulss
        | Mnemonic::Divss
        | Mnemonic::Addsd
        | Mnemonic::Subsd
        | Mnemonic::Mulsd
        | Mnemonic::Divsd
        | Mnemonic::Ucomiss
        | Mnemonic::Ucomisd
        | Mnemonic::Comiss
        | Mnemonic::Comisd
        | Mnemonic::Cvtsi2ss
        | Mnemonic::Cvtsi2sd
        | Mnemonic::Cvtss2sd
        | Mnemonic::Cvtsd2ss
        | Mnemonic::Cvttss2si
        | Mnemonic::Cvttsd2si => {
            writeln!(out, "                    // [AOT_UNIMPLEMENTED] mnemonic={mn:?}")?;
            return Ok(false);
        }
        Mnemonic::Cmpxchg => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let src = read_op_u64(ins, 1, bits, image_base);
            let wr_acc = write_reg_u64(acc_reg(bits), "_old");
            if is_mem(ins, 0) {
                // Memory cmpxchg is always performed atomically (a locked cmpxchg is a valid
                // implementation of an unlocked one). Flags = cmp(acc, old).
                let addr = mem_addr_expr(ins, image_base);
                let f = lk("cas", bits);
                writeln!(
                    out,
                    "                    {{ let _src = ({src}) & {mask}; let _acc = rax & {mask}; let _old = {f}({addr}, _acc, _src);"
                )?;
                emit_arith_add_sub(out, bits, "_acc", "_old", "0u64", true, "", false)?;
                writeln!(out, "                    if !zf {{ {wr_acc} }} }}")?;
                return Ok(true);
            }
            let dst = read_op_u64(ins, 0, bits, image_base);
            let wr_dst = write_op_u64(ins, 0, bits, "_src", image_base);
            writeln!(
                out,
                "                    {{ let _old = ({dst}) & {mask}; let _src = ({src}) & {mask}; let _acc = rax & {mask};"
            )?;
            emit_arith_add_sub(out, bits, "_acc", "_old", "0u64", true, "", false)?;
            writeln!(out, "                    if zf {{ {wr_dst} }} else {{ {wr_acc} }} }}")?;
            return Ok(true);
        }
        Mnemonic::Cmpxchg8b | Mnemonic::Cmpxchg16b => {
            if !is_mem(ins, 0) {
                return Ok(false);
            }
            let addr = mem_addr_expr(ins, image_base);
            if mn == Mnemonic::Cmpxchg16b {
                writeln!(
                    out,
                    "                    {{ let (_ok, _lo, _hi) = aot_lk_cas128({addr}, rax, rdx, rbx, rcx); zf = _ok; if !_ok {{ rax = _lo; rdx = _hi; }} }}"
                )?;
            } else {
                writeln!(
                    out,
                    "                    {{ let _exp = ((rdx & 0xFFFF_FFFF) << 32) | (rax & 0xFFFF_FFFF); let _new = ((rcx & 0xFFFF_FFFF) << 32) | (rbx & 0xFFFF_FFFF); let _old = aot_lk_cas64({addr}, _exp, _new); zf = _old == _exp; if !zf {{ rax = _old & 0xFFFF_FFFF; rdx = _old >> 32; }} }}"
                )?;
            }
            return Ok(true);
        }
        Mnemonic::Xadd => {
            let bits = op_bits(ins, 0);
            let mask = mask_lit(bits);
            let b = read_op_u64(ins, 1, bits, image_base);
            if is_mem(ins, 0) {
                let addr = mem_addr_expr(ins, image_base);
                let f = lk("add", bits);
                let wr_src = write_op_u64(ins, 1, bits, "_old", image_base);
                writeln!(
                    out,
                    "                    {{ let _bv = ({b}) & {mask}; let _old = {f}({addr}, _bv);"
                )?;
                emit_arith_add_sub(out, bits, "_old", "_bv", "0u64", false, "", false)?;
                writeln!(out, "                    {wr_src} }}")?;
                return Ok(true);
            }
            let a = read_op_u64(ins, 0, bits, image_base);
            let wr_src = write_op_u64(ins, 1, bits, "_dv", image_base);
            let wr_dst = write_op_u64(ins, 0, bits, "_sum", image_base);
            writeln!(
                out,
                "                    {{ let _dv = ({a}) & {mask}; let _bv = ({b}) & {mask}; let _sum = _dv.wrapping_add(_bv) & {mask};"
            )?;
            emit_arith_add_sub(out, bits, "_dv", "_bv", "0u64", false, "", false)?;
            // TEMP = SRC + DEST; SRC = DEST; DEST = TEMP (dest written last).
            writeln!(out, "                    {wr_src} {wr_dst} }}")?;
            return Ok(true);
        }
        Mnemonic::Enter => {
            let alloc = if ins.op_count() > 0 && ins.op0_kind() == OpKind::Immediate16 {
                ins.immediate16() as u64
            } else {
                0
            };
            writeln!(
                out,
                "                    rsp = rsp.wrapping_sub(8); wr64s(rsp, rbp); rbp = rsp; rsp = rsp.wrapping_sub({alloc});"
            )?;
            return Ok(true);
        }
        Mnemonic::Xlatb => {
            writeln!(
                out,
                "                    {{ let _idx = rax & 0xFF; let _b = rd8(rbx.wrapping_add(_idx)) as u64; rax = (rax & !0xFF) | _b; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Rdrand | Mnemonic::Rdseed => {
            let bits = op_bits(ins, 0);
            let wr = write_op_u64(ins, 0, bits, "0x9E37_79B9_7F4A_7C15u64", image_base);
            writeln!(
                out,
                "                    {{ {wr} cf = true; zf = false; sf = false; of = false; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Xtest => {
            writeln!(
                out,
                "                    {{ zf = true; cf = false; sf = false; of = false; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Syscall | Mnemonic::Sysenter => {
            let next_va = ins.next_ip();
            writeln!(
                out,
                "                    {{ let _r10 = if r10 != 0 {{ r10 }} else {{ rcx }}; rcx = 0x{next_va:016X}; r11 = 0x202; rax = aot_nt_syscall(rax as u32, _r10, rdx, r8, r9, rsp); }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Cpuid => {
            writeln!(
                out,
                "                    {{ let (_a, _b, _c, _d) = aot_cpuid(rax as u32, rcx as u32); rax = _a; rbx = _b; rcx = _c; rdx = _d; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Xgetbv => {
            writeln!(out, "                    {{ rax = 7; rdx = 0; }}")?;
            return Ok(true);
        }
        Mnemonic::Rdtsc => {
            writeln!(
                out,
                "                    {{ let _t = aot_read_tsc(); rax = _t & 0xFFFF_FFFF; rdx = _t >> 32; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Rdtscp => {
            writeln!(
                out,
                "                    {{ let _t = aot_read_tsc(); rax = _t & 0xFFFF_FFFF; rdx = _t >> 32; rcx = 0; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Clc => {
            writeln!(out, "                    cf = false;")?;
            return Ok(true);
        }
        Mnemonic::Stc => {
            writeln!(out, "                    cf = true;")?;
            return Ok(true);
        }
        Mnemonic::Cmc => {
            writeln!(out, "                    cf = !cf;")?;
            return Ok(true);
        }
        Mnemonic::Sahf => {
            writeln!(
                out,
                "                    {{ let _ah = (rax >> 8) & 0xFF; sf = (_ah & 0x80) != 0; zf = (_ah & 0x40) != 0; cf = (_ah & 1) != 0; }}"
            )?;
            return Ok(true);
        }
        Mnemonic::Lahf => {
            // PF/AF are not tracked (reported as 0), so this is only approximate.
            writeln!(
                out,
                "                    rax = (rax & !0xFF00) | ((sf as u64) << 15) | ((zf as u64) << 14) | (1u64 << 9) | ((cf as u64) << 8);"
            )?;
            return Ok(false);
        }
        Mnemonic::Std => {
            writeln!(out, "                    // [AOT_UNIMPLEMENTED] std (DF=1 not modeled)")?;
            return Ok(false);
        }
        Mnemonic::Stmxcsr => {
            let addr = mem_addr_expr(ins, image_base);
            writeln!(out, "                    wr32({addr}, 0x1F80);")?;
            return Ok(true);
        }
        Mnemonic::Fnstcw => {
            let addr = mem_addr_expr(ins, image_base);
            writeln!(out, "                    wr16({addr}, 0x027F);")?;
            return Ok(true);
        }
        // Treated as no-ops: DF is always clear (cld), the FP environment / MXCSR
        // rounding and exception modes are not modeled, and cache-control hints don't matter.
        Mnemonic::Cld
        | Mnemonic::Fninit
        | Mnemonic::Fnclex
        | Mnemonic::Fldcw
        | Mnemonic::Ldmxcsr
        | Mnemonic::Clflush
        | Mnemonic::Clflushopt
        | Mnemonic::Clwb
        | Mnemonic::Xend => {
            return Ok(true);
        }
        _ if crate::ops_db::classify_instruction(ins).is_some() => {
            if let Some(rec) = crate::ops_db::classify_instruction(ins) {
                if rec.tier == crate::ops_db::TranslationTier::TrapOrBoundary {
                    writeln!(out, "                    return rax;")?;
                    *terminated = true;
                    return Ok(true);
                }
                // Known to the ops catalog, but there is no lowering here: no code is
                // emitted, so it must not count as lowered.
                writeln!(
                    out,
                    "                    // [AOT_MAPPED:{}] mnemonic={mn:?} (not lowered)",
                    rec.id
                )?;
            }
            return Ok(false);
        }
        _ => {
            writeln!(out, "                    // [AOT_FALLBACK] mnemonic={mn:?}")?;
            let _ = Code::Nopd;
            return Ok(false);
        }
    }
}

/// `true` for a `lock`-prefixed instruction whose destination is memory.
fn is_locked_rmw(ins: &Instruction) -> bool {
    ins.has_lock_prefix() && ins.op_count() > 0 && ins.op0_kind() == OpKind::Memory
}

fn is_mem(ins: &Instruction, op: u32) -> bool {
    ins.op_count() > op && ins.op_kind(op) == OpKind::Memory
}

/// `true` for implicit-operand string instructions (`stos`/`movs`/`lods`/...), as
/// opposed to the SSE `movsd` which shares a mnemonic with the string `movsd`.
fn is_string_op(ins: &Instruction) -> bool {
    (0..ins.op_count()).any(|i| {
        matches!(
            ins.op_kind(i),
            OpKind::MemorySegSI
                | OpKind::MemorySegESI
                | OpKind::MemorySegRSI
                | OpKind::MemoryESDI
                | OpKind::MemoryESEDI
                | OpKind::MemoryESRDI
        )
    })
}

/// Name of the prelude's locked-RMW helper for `op` at operand width `bits`.
fn lk(op: &str, bits: u32) -> String {
    let w = match bits {
        8 => 8,
        16 => 16,
        32 => 32,
        _ => 64,
    };
    format!("aot_lk_{op}{w}")
}

fn acc_reg(bits: u32) -> Register {
    match bits {
        8 => Register::AL,
        16 => Register::AX,
        32 => Register::EAX,
        _ => Register::RAX,
    }
}

fn rd_mem(bits: u32, addr: &str) -> String {
    match bits {
        8 => format!("(rd8({addr}) as u64)"),
        16 => format!("(rd16({addr}) as u64)"),
        32 => format!("(rd32({addr}) as u64)"),
        _ => format!("rd64({addr})"),
    }
}

fn wr_mem(bits: u32, addr: &str, val: &str) -> String {
    match bits {
        8 => format!("wr8({addr}, ({val}) as u8);"),
        16 => format!("wr16({addr}, ({val}) as u16);"),
        32 => format!("wr32({addr}, ({val}) as u32);"),
        _ => format!("wr64({addr}, {val});"),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_arith_add_sub(
    out: &mut String,
    bits: u32,
    a: &str,
    b: &str,
    carry_in: &str,
    is_sub: bool,
    wr: &str,
    write_back: bool,
) -> Result<()> {
    let mask = mask_lit(bits);
    let sign_shift = bits - 1;
    let op_call = if is_sub {
        "wrapping_sub(_b).wrapping_sub(_c)"
    } else {
        "wrapping_add(_b).wrapping_add(_c)"
    };
    let cf_expr = if is_sub {
        "(_a as u128) < (_b as u128 + _c as u128)".to_string()
    } else {
        format!("((_a as u128) + (_b as u128) + (_c as u128)) > ({mask} as u128)")
    };
    let of_expr = if is_sub {
        format!("(((_a ^ _b) & (_a ^ _r)) >> {sign_shift}) != 0")
    } else {
        format!("((!(_a ^ _b) & (_a ^ _r)) >> {sign_shift}) != 0")
    };
    let tail = if write_back { wr } else { "" };
    writeln!(
        out,
        "                    {{ let _a = ({a}) & {mask}; let _b = ({b}) & {mask}; let _c = {carry_in}; let _r = (_a.{op_call}) & {mask}; cf = {cf_expr}; zf = _r == 0; sf = (_r >> {sign_shift}) != 0; of = {of_expr}; {tail} }}"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Lowers hand-encoded x86 snippets, compiles the generated Rust (with the real AOT
    //! prelude) with `rustc`, and executes it natively, including multi-threaded
    //! contention tests for the locked/atomic lowerings.
    use super::super::runtime::{
        emit_aot_runtime_prelude, emit_function_directory, emit_import_table,
    };
    use super::*;
    use iced_x86::{Decoder, DecoderOptions, Formatter, IntelFormatter};
    use std::path::Path;
    use std::process::Command;

    const BASE: u64 = 0x1_4000_0000;

    struct Snip {
        name: &'static str,
        code: &'static [u8],
        asm: &'static [&'static str],
        exact: bool,
    }

    const SNIPS: &[Snip] = &[
        Snip { name: "lock_inc32", code: &[0xF0, 0xFF, 0x01], asm: &["lock inc dword ptr [rcx]"], exact: true },
        Snip { name: "lock_xadd32", code: &[0xF0, 0x0F, 0xC1, 0x01], asm: &["lock xadd [rcx],eax"], exact: true },
        Snip { name: "lock_add64", code: &[0xF0, 0x48, 0x01, 0x11], asm: &["lock add [rcx],rdx"], exact: true },
        Snip { name: "lock_cmpxchg32", code: &[0xF0, 0x0F, 0xB1, 0x11], asm: &["lock cmpxchg [rcx],edx"], exact: true },
        Snip { name: "cmpxchg32", code: &[0x0F, 0xB1, 0x11], asm: &["cmpxchg [rcx],edx"], exact: true },
        Snip { name: "xchg64", code: &[0x48, 0x87, 0x01], asm: &["xchg [rcx],rax"], exact: true },
        Snip { name: "inc64_plain", code: &[0x48, 0xFF, 0x02], asm: &["inc qword ptr [rdx]"], exact: true },
        Snip { name: "mov_rel64", code: &[0x48, 0xC7, 0x01, 0, 0, 0, 0], asm: &["mov qword ptr [rcx],0"], exact: true },
        Snip { name: "cas16b", code: &[0xF0, 0x48, 0x0F, 0xC7, 0x0E], asm: &["lock cmpxchg16b [rsi]"], exact: true },
        Snip { name: "cas8b", code: &[0xF0, 0x0F, 0xC7, 0x0E], asm: &["lock cmpxchg8b [rsi]"], exact: true },
        Snip { name: "lock_sub32_5", code: &[0xF0, 0x83, 0x29, 0x05], asm: &["lock sub dword ptr [rcx],5"], exact: true },
        Snip { name: "lock_or8", code: &[0xF0, 0x80, 0x09, 0x80], asm: &["lock or byte ptr [rcx],80h"], exact: true },
        Snip { name: "lock_dec32", code: &[0xF0, 0xFF, 0x09], asm: &["lock dec dword ptr [rcx]"], exact: true },
        Snip { name: "lock_neg32", code: &[0xF0, 0xF7, 0x19], asm: &["lock neg dword ptr [rcx]"], exact: true },
        Snip { name: "lock_not64", code: &[0xF0, 0x48, 0xF7, 0x11], asm: &["lock not qword ptr [rcx]"], exact: true },
        Snip { name: "lock_adc32", code: &[0xF0, 0x11, 0x11], asm: &["lock adc [rcx],edx"], exact: true },
        Snip { name: "lock_bts32_3", code: &[0xF0, 0x0F, 0xBA, 0x29, 0x03], asm: &["lock bts dword ptr [rcx],3"], exact: true },
        Snip { name: "bts64_reg", code: &[0x48, 0x0F, 0xAB, 0x11], asm: &["bts [rcx],rdx"], exact: true },
        Snip { name: "btr_eax_4", code: &[0x0F, 0xBA, 0xF0, 0x04], asm: &["btr eax,4"], exact: true },
        Snip { name: "btc_rax_rdx", code: &[0x48, 0x0F, 0xBB, 0xD0], asm: &["btc rax,rdx"], exact: true },
        Snip { name: "div_rcx", code: &[0x48, 0xF7, 0xF1], asm: &["div rcx"], exact: true },
        Snip { name: "idiv_ecx", code: &[0xF7, 0xF9], asm: &["idiv ecx"], exact: true },
        Snip { name: "idiv_rcx", code: &[0x48, 0xF7, 0xF9], asm: &["idiv rcx"], exact: true },
        Snip { name: "div_cl", code: &[0xF6, 0xF1], asm: &["div cl"], exact: true },
        Snip { name: "idiv_cx", code: &[0x66, 0xF7, 0xF9], asm: &["idiv cx"], exact: true },
        Snip { name: "imul_ecx", code: &[0xF7, 0xE9], asm: &["imul ecx"], exact: true },
        Snip { name: "imul_eax_ecx", code: &[0x0F, 0xAF, 0xC1], asm: &["imul eax,ecx"], exact: true },
        Snip { name: "mul_cl", code: &[0xF6, 0xE1], asm: &["mul cl"], exact: true },
        Snip { name: "shl_eax_1", code: &[0xD1, 0xE0], asm: &["shl eax,1"], exact: true },
        Snip { name: "shr_eax_cl", code: &[0xD3, 0xE8], asm: &["shr eax,cl"], exact: true },
        Snip { name: "movss_load", code: &[0xF3, 0x0F, 0x10, 0x01], asm: &["movss xmm0,[rcx]"], exact: true },
        Snip { name: "movss_store", code: &[0xF3, 0x0F, 0x11, 0x09], asm: &["movss [rcx],xmm1"], exact: true },
        Snip { name: "movss_rr", code: &[0xF3, 0x0F, 0x10, 0xC1], asm: &["movss xmm0,xmm1"], exact: true },
        Snip { name: "bsr_eax_ecx", code: &[0x0F, 0xBD, 0xC1], asm: &["bsr eax,ecx"], exact: true },
        Snip { name: "lzcnt_eax_ecx", code: &[0xF3, 0x0F, 0xBD, 0xC1], asm: &["lzcnt eax,ecx"], exact: true },
        Snip { name: "tzcnt_eax_ecx", code: &[0xF3, 0x0F, 0xBC, 0xC1], asm: &["tzcnt eax,ecx"], exact: true },
        Snip { name: "shld_rax_rdx_4", code: &[0x48, 0x0F, 0xA4, 0xD0, 0x04], asm: &["shld rax,rdx,4"], exact: true },
        Snip { name: "rep_movsb", code: &[0xF3, 0xA4], asm: &["rep movsb"], exact: true },
        Snip { name: "rep_stosd", code: &[0xF3, 0xAB], asm: &["rep stosd"], exact: true },
        Snip { name: "movsd_str", code: &[0xA5], asm: &["movsd"], exact: true },
        Snip { name: "mfence", code: &[0x0F, 0xAE, 0xF0], asm: &["mfence"], exact: true },
        // Approximations must report Ok(false).
        Snip { name: "addss_unimpl", code: &[0xF3, 0x0F, 0x58, 0xC1], asm: &["addss xmm0,xmm1"], exact: false },
        Snip { name: "setp_al", code: &[0x0F, 0x9A, 0xC0], asm: &["setp al"], exact: false },
    ];

    const MAIN: &str = r#"
fn main() {
    unsafe {
        let mut st = St::default();
        let mut m = [0u64; 8];
        let p = m.as_mut_ptr() as u64;

        // ---- single-threaded semantics ----
        m[0] = 3; st.r[RCX] = p; lock_sub32_5(&mut st);
        assert_eq!(m[0], 0xFFFF_FFFE); assert!(st.cf && st.sf && !st.zf && !st.of);
        m[0] = 0x01; lock_or8(&mut st); assert_eq!(m[0], 0x81); assert!(st.sf && !st.cf);
        m[0] = 1; lock_dec32(&mut st); assert_eq!(m[0], 0); assert!(st.zf && !st.of);
        m[0] = 0x8000_0000; lock_dec32(&mut st); assert_eq!(m[0], 0x7FFF_FFFF); assert!(st.of);
        m[0] = 5; lock_neg32(&mut st); assert_eq!(m[0], 0xFFFF_FFFB); assert!(st.cf && st.sf);
        m[0] = 0; lock_not64(&mut st); assert_eq!(m[0], u64::MAX);
        m[0] = 0xFFFF_FFFF; st.r[RDX] = 0; st.cf = true; lock_adc32(&mut st);
        assert_eq!(m[0], 0); assert!(st.cf && st.zf);
        m[0] = 0; lock_bts32_3(&mut st); assert_eq!(m[0], 8); assert!(!st.cf);
        lock_bts32_3(&mut st); assert_eq!(m[0], 8); assert!(st.cf);

        // bts with a negative register bit offset addresses the previous qword.
        m[0] = 0; m[1] = 0; m[2] = 0; st.r[RCX] = p + 8; st.r[RDX] = u64::MAX; bts64_reg(&mut st);
        assert_eq!(m[0], 1u64 << 63); assert_eq!(m[1], 0); assert!(!st.cf);
        st.r[RDX] = 65; bts64_reg(&mut st); assert_eq!(m[2], 2); assert!(!st.cf);
        bts64_reg(&mut st); assert!(st.cf);
        st.r[RAX] = u64::MAX; btr_eax_4(&mut st); assert_eq!(st.r[RAX], 0xFFFF_FFEF); assert!(st.cf);
        st.r[RAX] = 0; st.r[RDX] = 64 + 5; btc_rax_rdx(&mut st); assert_eq!(st.r[RAX], 0x20); assert!(!st.cf);

        // cmpxchg failure: ZF=0, accumulator <- memory, flags of cmp(acc, mem).
        m[0] = 5; st.r[RCX] = p; st.r[RAX] = 0xFFFF_FFFF_0000_0003; st.r[RDX] = 9; cmpxchg32(&mut st);
        assert!(!st.zf && st.cf); assert_eq!(st.r[RAX], 5); assert_eq!(m[0], 5);
        st.r[RAX] = 5; lock_cmpxchg32(&mut st); assert!(st.zf); assert_eq!(m[0], 9);

        // div / idiv
        st.r[RDX] = 1; st.r[RAX] = 0; st.r[RCX] = 2; div_rcx(&mut st);
        assert_eq!((st.r[RAX], st.r[RDX]), (1u64 << 63, 0));
        st.r[RDX] = 0; st.r[RAX] = 7; st.r[RCX] = 2; div_rcx(&mut st); assert_eq!((st.r[RAX], st.r[RDX]), (3, 1));
        st.r[RDX] = 0xFFFF_FFFF; st.r[RAX] = 0xFFFF_FFF9; st.r[RCX] = 2; idiv_ecx(&mut st);
        assert_eq!((st.r[RAX], st.r[RDX]), (0xFFFF_FFFD, 0xFFFF_FFFF));
        st.r[RDX] = u64::MAX; st.r[RAX] = (-100i64) as u64; st.r[RCX] = 7; idiv_rcx(&mut st);
        assert_eq!((st.r[RAX] as i64, st.r[RDX] as i64), (-14, -2));
        st.r[RAX] = 0x1234_0000_0000_0103; st.r[RCX] = 0x10; div_cl(&mut st); assert_eq!(st.r[RAX], 0x1234_0000_0000_0310);
        st.r[RAX] = 0xAAAA_0000_0000_FC18; st.r[RDX] = 0xBBBB_0000_0000_FFFF; st.r[RCX] = 10; idiv_cx(&mut st);
        assert_eq!((st.r[RAX], st.r[RDX]), (0xAAAA_0000_0000_FF9C, 0xBBBB_0000_0000_0000));
        st.r[RAX] = 11; st.r[RDX] = 0; st.r[RCX] = 0; div_rcx(&mut st); assert_eq!((st.r[RAX], st.r[RDX]), (11, 0));

        // mul / imul
        st.r[RAX] = 0x1_0000_0002; st.r[RCX] = 0xFFFF_FFFD; imul_ecx(&mut st);
        assert_eq!((st.r[RAX], st.r[RDX]), (0xFFFF_FFFA, 0xFFFF_FFFF)); assert!(!st.cf && !st.of);
        st.r[RAX] = 0x4000_0000; st.r[RCX] = 4; imul_eax_ecx(&mut st); assert_eq!(st.r[RAX], 0); assert!(st.cf && st.of);
        st.r[RAX] = 0x80; st.r[RCX] = 2; mul_cl(&mut st); assert_eq!(st.r[RAX], 0x100); assert!(st.cf);

        // shifts
        st.r[RAX] = 0x8000_0001; shl_eax_1(&mut st); assert_eq!(st.r[RAX], 2); assert!(st.cf && st.of);
        st.r[RAX] = 0x10; st.r[RCX] = 5; shr_eax_cl(&mut st); assert_eq!(st.r[RAX], 0); assert!(st.cf && st.zf);
        st.r[RAX] = 1; st.r[RDX] = 0xF000_0000_0000_0000; shld_rax_rdx_4(&mut st); assert_eq!(st.r[RAX], 0x1F);

        // bit scans
        st.r[RCX] = 0x10; bsr_eax_ecx(&mut st); assert_eq!(st.r[RAX], 4); assert!(!st.zf);
        lzcnt_eax_ecx(&mut st); assert_eq!(st.r[RAX], 27);
        st.r[RCX] = 0; tzcnt_eax_ecx(&mut st); assert_eq!(st.r[RAX], 32); assert!(st.cf);
        st.r[RAX] = 77; bsr_eax_ecx(&mut st); assert_eq!(st.r[RAX], 77); assert!(st.zf);

        // movss
        m[0] = 0x5566_7788_1122_3344; st.r[RCX] = p; st.xmm[0] = u128::MAX; movss_load(&mut st);
        assert_eq!(st.xmm[0], 0x1122_3344);
        m[0] = 0xAAAA_AAAA_AAAA_AAAA; st.xmm[1] = 0x9999_DEAD_BEEF; movss_store(&mut st);
        assert_eq!(m[0], 0xAAAA_AAAA_DEAD_BEEF);
        st.xmm[0] = 0x1111_1111_1111_1111_1111_1111_1111_1111; st.xmm[1] = 0x2222_2222_2222_2222_2222_2222_2222_2222;
        movss_rr(&mut st); assert_eq!(st.xmm[0], 0x1111_1111_1111_1111_1111_1111_2222_2222);

        // string ops
        let mut buf = [1u8, 2, 3, 0, 0, 0, 0, 0];
        st.r[RSI] = buf.as_ptr() as u64; st.r[RDI] = buf.as_ptr() as u64 + 1; st.r[RCX] = 5; rep_movsb(&mut st);
        assert_eq!(buf, [1, 1, 1, 1, 1, 1, 0, 0]); assert_eq!(st.r[RCX], 0);
        m[0] = 0; m[1] = 0; st.r[RDI] = p; st.r[RCX] = 2; st.r[RAX] = 0xAABB_CCDD; rep_stosd(&mut st);
        assert_eq!(m[0], 0xAABB_CCDD_AABB_CCDD); assert_eq!(m[1], 0); assert_eq!(st.r[RDI], p + 8);
        st.r[RSI] = p; st.r[RDI] = p + 8; movsd_str(&mut st); assert_eq!(m[1], 0xAABB_CCDD);
        assert_eq!((st.r[RSI], st.r[RDI]), (p + 4, p + 12));

        // cmpxchg8b success then failure
        m[0] = 0x0000_0002_0000_0001; st.r[RSI] = p; st.r[RDX] = 2; st.r[RAX] = 1; st.r[RCX] = 4; st.r[RBX] = 3;
        cas8b(&mut st); assert!(st.zf); assert_eq!(m[0], 0x0000_0004_0000_0003);
        st.r[RDX] = 2; st.r[RAX] = 1; cas8b(&mut st); assert!(!st.zf); assert_eq!((st.r[RAX], st.r[RDX]), (3, 4));
        mfence(&mut st);

        // ---- multi-threaded contention ----
        const THREADS: usize = 8;
        const ITERS: u64 = 20_000;
        let ctr: &'static mut [u64; 8] = Box::leak(Box::new([0u64; 8]));
        let base = ctr.as_mut_ptr() as u64;
        let c128: &'static mut A16 = Box::leak(Box::new(A16([u64::MAX - 1000, 7])));
        let c128p = c128 as *mut A16 as u64;
        let mut hs = Vec::new();
        for _ in 0..THREADS {
            hs.push(std::thread::spawn(move || {
                let mut st = St::default();
                for _ in 0..ITERS {
                    st.r[RCX] = base; lock_inc32(&mut st);
                    st.r[RCX] = base + 8; st.r[RAX] = 2; lock_xadd32(&mut st);
                    st.r[RCX] = base + 16; st.r[RDX] = 3; lock_add64(&mut st);
                    // CAS-loop increment (failure reloads the accumulator).
                    st.r[RCX] = base + 24; st.r[RAX] = 0;
                    loop { st.r[RDX] = (st.r[RAX] + 1) & 0xFFFF_FFFF; lock_cmpxchg32(&mut st); if st.zf { break; } }
                    // xchg spinlock protecting a plain (non-locked) increment + plain release store.
                    loop { st.r[RCX] = base + 32; st.r[RAX] = 1; xchg64(&mut st); if st.r[RAX] == 0 { break; } std::hint::spin_loop(); }
                    st.r[RDX] = base + 40; inc64_plain(&mut st);
                    st.r[RCX] = base + 32; mov_rel64(&mut st);
                    // 128-bit CAS-loop increment with carry.
                    st.r[RSI] = c128p; st.r[RAX] = 0; st.r[RDX] = 0;
                    loop {
                        let (lo, c) = st.r[RAX].overflowing_add(1);
                        st.r[RBX] = lo; st.r[RCX] = st.r[RDX] + c as u64;
                        cas16b(&mut st);
                        if st.zf { break; }
                    }
                }
            }));
        }
        for h in hs { h.join().unwrap(); }
        let n = THREADS as u64 * ITERS;
        assert_eq!(ctr[0], n, "lock inc");
        assert_eq!(ctr[1], 2 * n, "lock xadd");
        assert_eq!(ctr[2], 3 * n, "lock add");
        assert_eq!(ctr[3], n, "lock cmpxchg loop");
        assert_eq!(ctr[4], 0, "xchg lock released");
        assert_eq!(ctr[5], n, "xchg spinlock + TSO");
        let v = ((c128.0[1] as u128) << 64) | c128.0[0] as u128;
        assert_eq!(v, ((7u128 << 64) | (u64::MAX - 1000) as u128) + n as u128, "cmpxchg16b loop");
    }
    println!("AOT_SEMANTICS_OK");
}
"#;

    fn lower_snip(s: &Snip) -> (String, bool) {
        let mut dec = Decoder::with_ip(64, s.code, BASE + 0x1000, DecoderOptions::NONE);
        let mut fmt = IntelFormatter::new();
        let mut body = String::new();
        let mut exact = true;
        let mut idx = 0;
        while dec.can_decode() {
            let ins = dec.decode();
            assert!(!ins.is_invalid(), "{}: invalid encoding", s.name);
            let mut text = String::new();
            fmt.format(&ins, &mut text);
            assert!(text.starts_with(s.asm[idx]), "{}: decoded `{text}`, expected `{}`", s.name, s.asm[idx]);
            idx += 1;
            let mut term = false;
            exact &= lower_instruction(
                &mut body,
                &ins,
                BASE,
                None,
                &BTreeSet::new(),
                &BTreeSet::new(),
                &HashMap::new(),
                &BTreeMap::new(),
                &mut term,
            )
            .unwrap();
            assert!(!term, "{}: unexpected terminator", s.name);
        }
        assert_eq!(idx, s.asm.len(), "{}: instruction count", s.name);
        let src = format!(
            "unsafe fn {}(st: &mut St) {{\n    let [mut rax, mut rcx, mut rdx, mut rbx, mut rsp, mut rbp, mut rsi, mut rdi, mut r8, mut r9, mut r10, mut r11, mut r12, mut r13, mut r14, mut r15] = st.r;\n    let mut xmm = st.xmm; let (mut cf, mut zf, mut sf, mut of) = (st.cf, st.zf, st.sf, st.of); let mut pc: u32 = 0;\n{body}    st.r = [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15]; st.xmm = xmm; st.cf = cf; st.zf = zf; st.sf = sf; st.of = of;\n}}\n",
            s.name
        );
        (src, exact)
    }

    fn build_program() -> String {
        let mut src = String::new();
        src.push_str("#![allow(unused_variables, unused_mut, unused_assignments, unused_parens, dead_code, non_snake_case, unreachable_code, unused_unsafe, clippy::all)]\n");
        emit_aot_runtime_prelude(&mut src, BASE, 0x10000).unwrap();
        emit_import_table(&mut src, &BTreeMap::new()).unwrap();
        emit_function_directory(&mut src, &[], &[], &[], &[]).unwrap();
        src.push_str(
            "#[derive(Clone, Copy, Default)]\nstruct St { r: [u64; 16], xmm: [u128; 16], cf: bool, zf: bool, sf: bool, of: bool }\n\
             #[repr(C, align(16))]\nstruct A16([u64; 2]);\n\
             const RAX: usize = 0; const RCX: usize = 1; const RDX: usize = 2; const RBX: usize = 3; const RSI: usize = 6; const RDI: usize = 7;\n",
        );
        for s in SNIPS {
            let (f, exact) = lower_snip(s);
            assert_eq!(exact, s.exact, "{}: exactness flag", s.name);
            src.push_str(&f);
        }
        src.push_str(MAIN);
        src
    }

    fn compile_and_run(src_file: &Path, bin: &Path, extra: &[&str]) {
        let status = Command::new("rustc")
            .args(["--edition=2021"])
            .args(extra)
            .arg(src_file)
            .arg("-o")
            .arg(bin)
            .status()
            .expect("invoke rustc");
        assert!(status.success(), "rustc failed for {extra:?}");
        let out = Command::new(bin).output().expect("run AOT semantics binary");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            out.status.success() && stdout.contains("AOT_SEMANTICS_OK"),
            "AOT semantics run failed ({extra:?}): stdout={stdout} stderr={stderr}"
        );
    }

    #[test]
    fn locked_rmw_lowerings_use_atomic_helpers() {
        let src = build_program();
        for needle in [
            "aot_lk_add32(",
            "aot_lk_sub32(",
            "aot_lk_or8(",
            "aot_lk_xchg64(",
            "aot_lk_cas32(",
            "aot_lk_rmw32(",
            "aot_lk_cas128(",
            "aot_lk_cas64(",
            "core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);",
            "ldapr",
            "stlr",
        ] {
            assert!(src.contains(needle), "generated source is missing `{needle}`");
        }
    }

    #[test]
    fn aot_lowerings_execute_natively_with_correct_semantics() {
        let src = build_program();
        let dir = std::env::temp_dir().join(format!("aw2_x86_aot_lower_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src_file = dir.join("aot_semantics.rs");
        std::fs::write(&src_file, &src).unwrap();
        // Debug build: overflow checks catch shift/arith overflow in the generated code
        // and verify the asm helpers at opt-level 0.
        compile_and_run(
            &src_file,
            &dir.join("aot_semantics_o0"),
            &["-C", "opt-level=0", "-C", "overflow-checks=on", "-C", "debug-assertions=on"],
        );
        // Same opt level as the shipped AOT dylibs.
        compile_and_run(&src_file, &dir.join("aot_semantics_o2"), &["-C", "opt-level=2"]);
        // Portable (non-aarch64) prelude path, if the x86_64 std is installed (runs under Rosetta).
        if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
            let has_x86 = Command::new("rustc")
                .args(["--print", "target-libdir", "--target", "x86_64-apple-darwin"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .is_some_and(|d| {
                    std::fs::read_dir(d.trim()).is_ok_and(|mut r| {
                        r.any(|e| {
                            e.is_ok_and(|e| e.file_name().to_string_lossy().starts_with("libstd"))
                        })
                    })
                });
            if has_x86 {
                compile_and_run(
                    &src_file,
                    &dir.join("aot_semantics_x86_64"),
                    &["--target", "x86_64-apple-darwin", "-C", "opt-level=1"],
                );
            }
        }
    }
}
