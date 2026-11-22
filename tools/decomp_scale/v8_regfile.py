"""Structural V8 EmbeddedData discovery and `DF_REGFILE` P1/P2/P3 basic-block Rust lifter.

Used by `dll_lift.py` when an image has a contiguous non-Win64-ABI embedded code range
(`CrashForExceptionInNonABICompliantCodeRange`, `[na_lo, na_hi)` in `v8.dll`).

1. Locates `(DefaultEmbeddedBlobCode, DefaultEmbeddedBlobData)` structurally in `.data` via
   `pe.reloc_sites` (an adjacent relocated 64-bit pointer pair whose first target is `na_lo`
   and whose second target starts a valid `EmbeddedData::LayoutDescription` table covering
   `[0, na_hi - na_lo)`).
2. Classifies every builtin in `LayoutDescription`:
   - `v8_nonabi_trap_stub`: 1-instruction `int3` / `ud2` architecture/simulator trap stub.
   - `v8_nonabi_x87`: builtin using x87 FPU stack instructions (`fld` / `fstp` / `fprem` / ...).
   - `v8_nonabi_atomic`: builtin using `lock`-prefixed / atomic RMW instructions (`cmpxchg` / `xadd` / memory `xchg`).
   - P1/P2/P3 (`1,616 / 1,643` builtins in `v8.dll`): lowered to a single block-indexed Rust
     `RegfileFn` (`unsafe extern "C" fn(*mut u8, u32) -> RegfileExit`) with continuation `(rva_off, blk)`
     entries for every `call` return address, RIP-addressed internal code label, and untargeted sub-entry.
"""

from __future__ import annotations

import bisect
import re
import struct

MASK64 = 0xFFFF_FFFF_FFFF_FFFF
_ST_PAT = re.compile(r"^st(?:\(([0-7])\))?$", re.IGNORECASE)
_RIP_DISP_RE = re.compile(r"\[rip\s*([+-])\s*(0x[0-9a-f]+|\d+)\]")

_GPR_INFO: dict[str, tuple[int, int, bool]] = {}
for _idx, (_r64, _r32, _r16, _r8l) in enumerate(
    [
        ("rax", "eax", "ax", "al"),
        ("rcx", "ecx", "cx", "cl"),
        ("rdx", "edx", "dx", "dl"),
        ("rbx", "ebx", "bx", "bl"),
        ("rsp", "esp", "sp", "spl"),
        ("rbp", "ebp", "bp", "bpl"),
        ("rsi", "esi", "si", "sil"),
        ("rdi", "edi", "di", "dil"),
    ]
):
    _GPR_INFO[_r64] = (_idx, 64, False)
    _GPR_INFO[_r32] = (_idx, 32, False)
    _GPR_INFO[_r16] = (_idx, 16, False)
    _GPR_INFO[_r8l] = (_idx, 8, False)
for _idx, _rh in enumerate(("ah", "ch", "dh", "bh")):
    _GPR_INFO[_rh] = (_idx, 8, True)
for _i in range(8, 16):
    _GPR_INFO[f"r{_i}"] = (_i, 64, False)
    _GPR_INFO[f"r{_i}d"] = (_i, 32, False)
    _GPR_INFO[f"r{_i}w"] = (_i, 16, False)
    _GPR_INFO[f"r{_i}b"] = (_i, 8, False)

_XMM_INFO: dict[str, int] = {f"xmm{i}": i for i in range(16)}

_CC_MAP: dict[str, str] = {
    "o": "o",
    "no": "no",
    "b": "b",
    "c": "b",
    "nae": "b",
    "ae": "ae",
    "nb": "ae",
    "nc": "ae",
    "e": "e",
    "z": "e",
    "ne": "ne",
    "nz": "ne",
    "be": "be",
    "na": "be",
    "a": "a",
    "nbe": "a",
    "s": "s",
    "ns": "ns",
    "p": "p",
    "pe": "p",
    "np": "np",
    "po": "np",
    "l": "l",
    "nge": "l",
    "ge": "ge",
    "nl": "ge",
    "le": "le",
    "ng": "le",
    "g": "g",
    "nle": "g",
}

_PTR_BITS: dict[str, int] = {
    "byte": 8,
    "word": 16,
    "dword": 32,
    "qword": 64,
    "xmmword": 128,
}

_MEM_PAT = re.compile(r"^(?:(byte|word|dword|qword|xmmword)\s+ptr\s+)?\[([^\]]+)\]$")
_SSE_ARITH_SD: dict[str, int] = {
    "addsd": 0,
    "subsd": 1,
    "mulsd": 2,
    "divsd": 3,
    "minsd": 4,
    "maxsd": 5,
}


def _split_ops(ops: str) -> list[str]:
    if not ops:
        return []
    return [p.strip() for p in re.split(r",(?![^\[]*\])", ops)]


def _parse_int(tok: str) -> int:
    tok = tok.strip()
    neg = tok.startswith("-")
    if tok[:1] in "+-":
        tok = tok[1:].strip()
    v = int(tok, 16) if tok.startswith("0x") else int(tok, 10)
    return -v if neg else v


def _parse_branch_rva(tok: str, file_base: int) -> int | None:
    tok = tok.strip()
    if re.fullmatch(r"0x[0-9a-f]+", tok):
        v = int(tok, 16)
        if v >= file_base:
            return v - file_base
    return None


def _parse_mem_ea(inner: str, next_va: int) -> str:
    expr = inner.replace(" ", "")
    if ":" in expr:
        seg, expr = expr.split(":", 1)
        if seg not in ("ds", "ss", "cs", "es"):
            raise ValueError(f"unsupported segment {seg}:[{inner}]")
    terms = re.findall(r"[+-]?[^+-]+", expr)
    rip_rel = False
    regs: list[tuple[int, int]] = []  # (gpr_idx, scale)
    disp = 0
    for t in terms:
        sign = -1 if t.startswith("-") else 1
        body = t.lstrip("+-")
        if body == "rip":
            if sign != 1:
                raise ValueError(f"negative rip in [{inner}]")
            rip_rel = True
        elif "*" in body:
            if sign != 1:
                raise ValueError(f"subtracted index in [{inner}]")
            a, b = body.split("*", 1)
            if a in _GPR_INFO:
                rname, sc_s = a, b
            elif b in _GPR_INFO:
                rname, sc_s = b, a
            else:
                raise ValueError(f"bad scaled index [{inner}]")
            ri, rbits, rhi = _GPR_INFO[rname]
            if rbits != 64 or rhi:
                raise ValueError(f"non-64 index {rname} in [{inner}]")
            sc = _parse_int(sc_s)
            regs.append((ri, sc))
        elif body in _GPR_INFO:
            if sign != 1:
                raise ValueError(f"subtracted base in [{inner}]")
            ri, rbits, rhi = _GPR_INFO[body]
            if rbits != 64 or rhi:
                raise ValueError(f"non-64 base {body} in [{inner}]")
            regs.append((ri, 1))
        else:
            disp += sign * _parse_int(body)
    if rip_rel:
        if regs:
            raise ValueError(f"mixed rip and gpr in [{inner}]")
        return f"0x{(next_va + disp) & MASK64:x}u64"
    parts: list[str] = []
    for ri, sc in regs:
        if sc == 1:
            parts.append(f"c.r64({ri})")
        else:
            parts.append(f"c.r64({ri}).wrapping_mul({sc})")
    if not parts:
        return f"0x{disp & MASK64:x}u64"
    acc = parts[0]
    for p in parts[1:]:
        acc = f"{acc}.wrapping_add({p})"
    if disp > 0:
        acc = f"{acc}.wrapping_add(0x{disp:x})"
    elif disp < 0:
        acc = f"{acc}.wrapping_sub(0x{-disp:x})"
    return acc


def _parse_op(op_s: str, next_va: int):
    s = op_s.strip()
    if s in _GPR_INFO:
        idx, bits, hi8 = _GPR_INFO[s]
        return ("gpr", idx, bits, hi8)
    if s in _XMM_INFO:
        return ("xmm", _XMM_INFO[s])
    m_st = _ST_PAT.match(s)
    if m_st:
        return ("st", int(m_st.group(1) or "0"))
    m = _MEM_PAT.match(s)
    if m:
        mbits = _PTR_BITS[m.group(1)] if m.group(1) else 0
        return ("mem", mbits, _parse_mem_ea(m.group(2), next_va))
    return ("imm", _parse_int(s) & MASK64)


def discover_embedded_builtins(pe, base: int, na_lo: int, na_hi: int) -> list[tuple[int, int, int]]:
    """Return `[(bid, rva, inst_len), ...]` from V8's `EmbeddedData::LayoutDescription` table."""
    if any(na_lo <= s < na_hi for s in pe.reloc_sites):
        raise RuntimeError(f"unexpected base relocation inside non-ABI blob +0x{na_lo:x}..+0x{na_hi:x}")
    relocs = sorted(s for s in pe.reloc_sites if not pe.in_text(s))
    reloc_set = set(relocs)
    blob_span = na_hi - na_lo
    for s in relocs:
        if s + 8 not in reloc_set:
            continue
        v_code = pe.u64_at_va(base + s)
        v_data = pe.u64_at_va(base + s + 8)
        if v_code is None or v_data is None:
            continue
        code_rva = v_code - base
        data_rva = v_data - base
        if code_rva != na_lo:
            continue
        doff = pe.off(data_rva)
        if doff is None or doff + 0x28 > len(pe.data):
            continue
        entries: list[tuple[int, int, int]] = []
        exp_moff = 0
        prev_ioff = -1
        bid = 0
        while True:
            off = doff + 0x18 + 16 * bid
            if off + 16 > len(pe.data):
                break
            ioff, ilen, moff, mlen = struct.unpack_from("<IIII", pe.data, off)
            if bid == 0 and (ioff != 0 or moff != 0 or ilen == 0):
                entries = []
                break
            if moff != exp_moff or ilen == 0 or ioff <= prev_ioff or ioff + ilen > blob_span:
                break
            entries.append((bid, na_lo + ioff, ilen))
            exp_moff = moff + mlen
            prev_ioff = ioff
            bid += 1
        if entries and ((entries[-1][1] - na_lo + entries[-1][2] + 0x1F) & ~0x1F) >= (blob_span - 0x20):
            return entries
    raise RuntimeError(f"failed to locate V8 EmbeddedData LayoutDescription for +0x{na_lo:x}..+0x{na_hi:x}")


def _emit_read_scalar(op, default_bits: int, ins_va: int, lines: list[str], tmp_id: int) -> tuple[str, int]:
    kind = op[0]
    if kind == "gpr":
        _, idx, rbits, hi8 = op
        return (f"c.rh8({idx})" if hi8 else f"c.r{rbits}({idx})"), tmp_id
    if kind == "imm":
        return f"0x{op[1]:x}u64", tmp_id
    if kind == "mem":
        _, mbits, ea = op
        bits = mbits or default_bits
        if bits not in (8, 16, 32, 64):
            raise ValueError(f"bad scalar load width {bits}")
        t = f"t{tmp_id}"
        lines.append(f"let Some({t}) = c.ld{bits}({ea}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};")
        return t, tmp_id + 1
    raise ValueError(f"expected scalar operand, got {op}")


def _emit_write_scalar(op, val_expr: str, default_bits: int, ins_va: int, lines: list[str]) -> None:
    kind = op[0]
    if kind == "gpr":
        _, idx, rbits, hi8 = op
        if hi8:
            lines.append(f"c.wh8({idx}, {val_expr});")
        else:
            lines.append(f"c.w{rbits}({idx}, {val_expr});")
        return
    if kind == "mem":
        _, mbits, ea = op
        bits = mbits or default_bits
        if bits not in (8, 16, 32, 64):
            raise ValueError(f"bad scalar store width {bits}")
        lines.append(f"if !c.st{bits}({ea}, {val_expr}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
        return
    raise ValueError(f"expected writable scalar operand, got {op}")


def _op_width(op) -> int:
    if op[0] == "gpr":
        return op[2]
    if op[0] == "mem":
        return op[1]
    return 0


def _lower_builtin(
    base: int, file_base: int, b_rva: int, e_rva: int, raw_insns: list[tuple[int, str, str, int]]
) -> tuple[str, list[tuple[int, int]]]:
    """Lower `[b_rva, e_rva)` instruction stream `(rva, mn, ops, next_rva)` to `(code, conts)`."""
    insns: list[tuple[int, str, str, int, bool]] = []
    i_raw = 0
    while i_raw < len(raw_insns):
        rva, mn, ops, nxt_rva = raw_insns[i_raw]
        if mn == "lock" and not ops:
            if i_raw + 1 >= len(raw_insns):
                raise ValueError(f"+0x{rva:x}: trailing bare lock prefix")
            _, mn2, ops2, nxt2 = raw_insns[i_raw + 1]
            insns.append((rva, mn2, ops2, nxt2, True))
            i_raw += 2
            continue
        if mn == "lock" and ops:
            parts_l = ops.split(None, 1)
            mn2 = parts_l[0]
            ops2 = parts_l[1] if len(parts_l) > 1 else ""
            insns.append((rva, mn2, ops2, nxt_rva, True))
            i_raw += 1
            continue
        insns.append((rva, mn, ops, nxt_rva, False))
        i_raw += 1

    addr_to_idx = {rva: i for i, (rva, _, _, _, _) in enumerate(insns)}
    leaders: set[int] = {b_rva}
    internal_branch_tgts: set[int] = set()
    call_ret_rvas: set[int] = set()
    rip_label_rvas: set[int] = set()
    uncond_after: set[int] = set()

    for idx, (rva, mn, ops, nxt_rva, _locked) in enumerate(insns):
        if mn == "jmp":
            tgt = _parse_branch_rva(ops, file_base)
            if tgt is not None and b_rva <= tgt < e_rva:
                if tgt not in addr_to_idx:
                    raise ValueError(f"+0x{rva:x}: internal jmp target +0x{tgt:x} mid-instruction")
                leaders.add(tgt)
                internal_branch_tgts.add(tgt)
            if idx + 1 < len(insns):
                nxt_ld = insns[idx + 1][0]
                leaders.add(nxt_ld)
                uncond_after.add(nxt_ld)
        elif mn.startswith("j") and mn[1:] in _CC_MAP:
            tgt = _parse_branch_rva(ops, file_base)
            if tgt is None:
                raise ValueError(f"+0x{rva:x}: non-direct jcc {mn} {ops}")
            if b_rva <= tgt < e_rva:
                if tgt not in addr_to_idx:
                    raise ValueError(f"+0x{rva:x}: internal jcc target +0x{tgt:x} mid-instruction")
                leaders.add(tgt)
                internal_branch_tgts.add(tgt)
            if idx + 1 < len(insns):
                leaders.add(insns[idx + 1][0])
        elif mn in ("call", "callq"):
            tgt = _parse_branch_rva(ops, file_base)
            if tgt is not None and b_rva <= tgt < e_rva and tgt in addr_to_idx:
                leaders.add(tgt)
            if idx + 1 < len(insns):
                nxt_ld = insns[idx + 1][0]
                leaders.add(nxt_ld)
                call_ret_rvas.add(nxt_ld)
        elif mn in ("ret", "retq", "int3", "ud2"):
            if idx + 1 < len(insns):
                nxt_ld = insns[idx + 1][0]
                leaders.add(nxt_ld)
                uncond_after.add(nxt_ld)
        elif mn == "lea":
            for m in _RIP_DISP_RE.finditer(ops):
                disp = _parse_int(m.group(2)) * (1 if m.group(1) == "+" else -1)
                tgt = nxt_rva + disp
                if b_rva < tgt < e_rva and tgt in addr_to_idx:
                    leaders.add(tgt)
                    rip_label_rvas.add(tgt)

    sorted_leaders = sorted(leaders)
    blk_of = {ld: bi for bi, ld in enumerate(sorted_leaders)}
    blocks: list[list[tuple[int, str, str, int, bool]]] = []
    for bi, ld in enumerate(sorted_leaders):
        end_ld = sorted_leaders[bi + 1] if bi + 1 < len(sorted_leaders) else e_rva
        s_i = addr_to_idx[ld]
        b_ins: list[tuple[int, str, str, int, bool]] = []
        k = s_i
        while k < len(insns) and insns[k][0] < end_ld:
            b_ins.append(insns[k])
            k += 1
        blocks.append(b_ins)

    conts: list[tuple[int, int]] = []
    for bi, ld in enumerate(sorted_leaders):
        if ld <= b_rva:
            continue
        is_cont = ld in call_ret_rvas or ld in rip_label_rvas
        if not is_cont and ld in uncond_after and ld not in internal_branch_tgts:
            first_mn, first_ops = blocks[bi][0][1], blocks[bi][0][2]
            if first_mn not in ("int3", "ud2", "nop") and not (first_mn == "data16" and first_ops.startswith("nop")):
                is_cont = True
        if is_cont:
            coff = ld - b_rva
            if coff > 0xFFFF or bi > 0xFFFF:
                raise ValueError(f"+0x{b_rva:x}: continuation ({coff:#x}, {bi}) overflows u16")
            conts.append((coff, bi))

    tmp_id = 0
    blk_arms: list[str] = []
    multi = len(blocks) > 1

    def jump_rva_stmt(tgt_rva: int) -> str:
        if tgt_rva in blk_of:
            return f"blk = {blk_of[tgt_rva]}; continue;"
        return f"return RegfileExit::jump(0x{base + tgt_rva:x});"

    for bi, b_ins in enumerate(blocks):
        lines: list[str] = []
        terminated = False
        for rva, mn, ops, nxt_rva, locked in b_ins:
            ins_va = base + rva
            next_va = base + nxt_rva
            raw_ops = _split_ops(ops)
            if locked and mn not in ("cmpxchg", "xchg"):
                raise ValueError(f"+0x{rva:x}: unsupported lock-prefixed {mn} {ops}")

            if mn in ("nop", "endbr64", "pause", "fnop") or mn.startswith("prefetch"):
                continue
            if mn == "data16" and ops.startswith("nop"):
                continue
            if mn in ("int3", "ud2"):
                lines.append(f"return RegfileExit::fault(0x{ins_va:x});")
                terminated = True
                break
            if mn == "leave":
                lines.append(f"if !c.leave() {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                continue
            if mn in ("ret", "retq"):
                imm16 = _parse_int(raw_ops[0]) if raw_ops else 0
                lines.append(f"return c.ret(0x{imm16:x}, 0x{ins_va:x});")
                terminated = True
                break
            if mn in ("call", "callq"):
                tgt_rva = _parse_branch_rva(raw_ops[0], file_base)
                if tgt_rva is not None:
                    callee_e = f"0x{base + tgt_rva:x}"
                else:
                    op0 = _parse_op(raw_ops[0], next_va)
                    callee_e, tmp_id = _emit_read_scalar(op0, 64, ins_va, lines, tmp_id)
                lines.append(f"return c.call({callee_e}, 0x{next_va:x}, 0x{ins_va:x});")
                terminated = True
                break
            if mn == "jmp":
                tgt_rva = _parse_branch_rva(raw_ops[0], file_base)
                if tgt_rva is not None:
                    lines.append(jump_rva_stmt(tgt_rva))
                else:
                    op0 = _parse_op(raw_ops[0], next_va)
                    v0, tmp_id = _emit_read_scalar(op0, 64, ins_va, lines, tmp_id)
                    lines.append(f"return RegfileExit::jump({v0});")
                terminated = True
                break
            if mn.startswith("j") and mn[1:] in _CC_MAP:
                cc = _CC_MAP[mn[1:]]
                tgt_rva = _parse_branch_rva(raw_ops[0], file_base)
                if tgt_rva is None:
                    raise ValueError(f"+0x{rva:x}: indirect jcc {ops}")
                taken = jump_rva_stmt(tgt_rva)
                lines.append(f"if c.cc_{cc}() {{ {taken} }}")
                if nxt_rva in blk_of:
                    lines.append(f"blk = {blk_of[nxt_rva]}; continue;")
                else:
                    lines.append(f"return RegfileExit::jump(0x{base + nxt_rva:x});")
                terminated = True
                break

            parsed = [_parse_op(x, next_va) for x in raw_ops]

            if mn in ("push", "pushq"):
                op0 = parsed[0]
                w = _op_width(op0) or 64
                if w != 64:
                    raise ValueError(f"+0x{rva:x}: non-64 push {ops}")
                v0, tmp_id = _emit_read_scalar(op0, 64, ins_va, lines, tmp_id)
                lines.append(f"if !c.push64({v0}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
            elif mn in ("pop", "popq"):
                op0 = parsed[0]
                if op0[0] == "gpr" and op0[2] == 64:
                    lines.append(f"if !c.pop64_reg({op0[1]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                elif op0[0] == "mem" and op0[1] in (0, 64):
                    sp_o = f"sp{tmp_id}"
                    pv = f"pv{tmp_id + 1}"
                    pea = f"pea{tmp_id + 2}"
                    tmp_id += 3
                    lines.append(f"let {sp_o} = c.r64(4);")
                    lines.append(f"let Some({pv}) = c.ld64({sp_o}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};")
                    lines.append(f"c.set_rsp({sp_o}.wrapping_add(8));")
                    lines.append(f"let {pea} = {op0[2]};")
                    lines.append(
                        f"if !c.st64({pea}, {pv}) {{ c.set_rsp({sp_o}); return RegfileExit::fault(0x{ins_va:x}); }}"
                    )
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported pop {ops}")
            elif mn == "lea":
                op0, op1 = parsed
                if op0[0] != "gpr" or op1[0] != "mem":
                    raise ValueError(f"+0x{rva:x}: bad lea {ops}")
                _emit_write_scalar(op0, op1[2], op0[2], ins_va, lines)
            elif mn in ("mov", "movabs"):
                op0, op1 = parsed
                bits = _op_width(op0) or _op_width(op1)
                if not bits:
                    raise ValueError(f"+0x{rva:x}: unsized mov {ops}")
                v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                _emit_write_scalar(op0, v1, bits, ins_va, lines)
            elif mn == "movzx":
                op0, op1 = parsed
                dst_bits = _op_width(op0)
                src_bits = _op_width(op1)
                if dst_bits not in (16, 32, 64) or src_bits not in (8, 16):
                    raise ValueError(f"+0x{rva:x}: bad movzx {ops}")
                v1, tmp_id = _emit_read_scalar(op1, src_bits, ins_va, lines, tmp_id)
                _emit_write_scalar(op0, v1, dst_bits, ins_va, lines)
            elif mn in ("movsx", "movsxd"):
                op0, op1 = parsed
                dst_bits = _op_width(op0)
                src_bits = _op_width(op1) or (32 if mn == "movsxd" else 0)
                if dst_bits not in (16, 32, 64) or src_bits not in (8, 16, 32):
                    raise ValueError(f"+0x{rva:x}: bad {mn} {ops}")
                v1, tmp_id = _emit_read_scalar(op1, src_bits, ins_va, lines, tmp_id)
                sext = f"({v1} as u{src_bits} as i{src_bits} as i64 as u64)"
                _emit_write_scalar(op0, sext, dst_bits, ins_va, lines)
            elif mn in ("cdqe", "cltq"):
                lines.append("c.cdqe();")
            elif mn in ("cdq", "cltd"):
                lines.append("c.cdq();")
            elif mn in ("cqo", "cqto"):
                lines.append("c.cqo();")
            elif mn == "idiv":
                op0 = parsed[0]
                bits = _op_width(op0)
                if bits not in (32, 64):
                    raise ValueError(f"+0x{rva:x}: unsupported idiv width {ops}")
                v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                lines.append(f"c.idiv({v0}, {bits});")
            elif mn == "bsr":
                op0, op1 = parsed
                if op0[0] != "gpr":
                    raise ValueError(f"+0x{rva:x}: non-gpr bsr {ops}")
                _, dst_i, bits, _ = op0
                v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                lines.append(f"c.bsr({dst_i}, {v1}, {bits});")
            elif mn == "xchg":
                op0, op1 = parsed
                if op0[0] == "gpr" and op1[0] == "gpr":
                    bits = _op_width(op0)
                    v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                    v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                    t0 = f"x{tmp_id}"
                    t1 = f"x{tmp_id + 1}"
                    tmp_id += 2
                    lines.append(f"let ({t0}, {t1}) = ({v0}, {v1});")
                    _emit_write_scalar(op0, t1, bits, ins_va, lines)
                    _emit_write_scalar(op1, t0, bits, ins_va, lines)
                elif (op0[0] == "mem" and op1[0] == "gpr") or (op0[0] == "gpr" and op1[0] == "mem"):
                    mem_op, reg_op = (op0, op1) if op0[0] == "mem" else (op1, op0)
                    bits = _op_width(reg_op)
                    if bits not in (8, 16, 32, 64):
                        raise ValueError(f"+0x{rva:x}: bad mem xchg width {ops}")
                    vr, tmp_id = _emit_read_scalar(reg_op, bits, ins_va, lines, tmp_id)
                    old_t = f"xm{tmp_id}"
                    tmp_id += 1
                    lines.append(
                        f"let Some({old_t}) = c.xchg_mem({mem_op[2]}, {vr}, {bits}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};"
                    )
                    _emit_write_scalar(reg_op, old_t, bits, ins_va, lines)
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported xchg {ops}")
            elif mn == "cmpxchg":
                op0, op1 = parsed
                bits = _op_width(op1) or _op_width(op0)
                if op0[0] != "mem" or op1[0] != "gpr" or bits not in (8, 16, 32, 64):
                    raise ValueError(f"+0x{rva:x}: unsupported cmpxchg {ops}")
                v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                lines.append(
                    f"if !c.lock_cmpxchg({op0[2]}, {v1}, {bits}) {{ return RegfileExit::fault(0x{ins_va:x}); }}"
                )
            elif mn in ("add", "sub", "xor", "or", "and"):
                op0, op1 = parsed
                bits = _op_width(op0) or _op_width(op1)
                if op0 == op1 and op0[0] == "gpr" and mn == "xor":
                    _emit_write_scalar(op0, "0", bits, ins_va, lines)
                    lines.append(f"c.logic(0, {bits});")
                else:
                    v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                    if op0[0] == "mem":
                        ea = f"ea{tmp_id}"
                        va_cur = f"t{tmp_id + 1}"
                        tmp_id += 2
                        lines.append(f"let {ea} = {op0[2]};")
                        lines.append(f"let Some({va_cur}) = c.ld{bits}({ea}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};")
                        res = f"r{tmp_id}"
                        tmp_id += 1
                        if mn in ("add", "sub"):
                            lines.append(f"let {res} = c.{mn}({va_cur}, {v1}, {bits});")
                        else:
                            op_ch = {"xor": "^", "or": "|", "and": "&"}[mn]
                            lines.append(f"let {res} = {va_cur} {op_ch} {v1}; c.logic({res}, {bits});")
                        lines.append(f"if !c.st{bits}({ea}, {res}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                    else:
                        v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                        res = f"r{tmp_id}"
                        tmp_id += 1
                        if mn in ("add", "sub"):
                            lines.append(f"let {res} = c.{mn}({v0}, {v1}, {bits});")
                        else:
                            op_ch = {"xor": "^", "or": "|", "and": "&"}[mn]
                            lines.append(f"let {res} = {v0} {op_ch} {v1};")
                        _emit_write_scalar(op0, res, bits, ins_va, lines)
                        if mn in ("xor", "or", "and"):
                            lines.append(f"c.logic({res}, {bits});")
            elif mn in ("cmp", "test"):
                op0, op1 = parsed
                bits = _op_width(op0) or _op_width(op1)
                v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                if mn == "cmp":
                    lines.append(f"c.cmp({v0}, {v1}, {bits});")
                else:
                    lines.append(f"c.logic({v0} & {v1}, {bits});")
            elif mn in ("inc", "dec", "neg", "not"):
                op0 = parsed[0]
                bits = _op_width(op0)
                if op0[0] == "mem":
                    ea = f"ea{tmp_id}"
                    va_cur = f"t{tmp_id + 1}"
                    res = f"r{tmp_id + 2}"
                    tmp_id += 3
                    lines.append(f"let {ea} = {op0[2]};")
                    lines.append(f"let Some({va_cur}) = c.ld{bits}({ea}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};")
                    if mn == "not":
                        lines.append(f"let {res} = !{va_cur};")
                    else:
                        lines.append(f"let {res} = c.{mn}({va_cur}, {bits});")
                    lines.append(f"if !c.st{bits}({ea}, {res}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                    res = f"r{tmp_id}"
                    tmp_id += 1
                    if mn == "not":
                        lines.append(f"let {res} = !{v0};")
                    else:
                        lines.append(f"let {res} = c.{mn}({v0}, {bits});")
                    _emit_write_scalar(op0, res, bits, ins_va, lines)
            elif mn in ("shl", "sal", "shr", "sar"):
                fn_nm = "shl" if mn == "sal" else mn
                op0 = parsed[0]
                bits = _op_width(op0)
                if len(parsed) == 1:
                    cnt_e = "1u64"
                else:
                    cnt_e, tmp_id = _emit_read_scalar(parsed[1], 8, ins_va, lines, tmp_id)
                if op0[0] == "mem":
                    ea = f"ea{tmp_id}"
                    va_cur = f"t{tmp_id + 1}"
                    tmp_id += 2
                    lines.append(f"let {ea} = {op0[2]};")
                    lines.append(f"let Some({va_cur}) = c.ld{bits}({ea}) else {{ return RegfileExit::fault(0x{ins_va:x}); }};")
                    lines.append(
                        f"if let Some(sr) = c.{fn_nm}({va_cur}, {cnt_e}, {bits}) {{ "
                        f"if !c.st{bits}({ea}, sr) {{ return RegfileExit::fault(0x{ins_va:x}); }} }}"
                    )
                else:
                    v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                    if bits == 32:
                        res = f"r{tmp_id}"
                        tmp_id += 1
                        lines.append(f"let {res} = c.{fn_nm}({v0}, {cnt_e}, 32).unwrap_or({v0});")
                        _emit_write_scalar(op0, res, 32, ins_va, lines)
                    else:
                        sub_l: list[str] = []
                        _emit_write_scalar(op0, "sr", bits, ins_va, sub_l)
                        lines.append(f"if let Some(sr) = c.{fn_nm}({v0}, {cnt_e}, {bits}) {{ {' '.join(sub_l)} }}")
            elif mn == "imul":
                if len(parsed) == 2:
                    op0, op1 = parsed
                    bits = _op_width(op0)
                    v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                    v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                    res = f"r{tmp_id}"
                    tmp_id += 1
                    lines.append(f"let {res} = c.imul2({v0}, {v1}, {bits});")
                    _emit_write_scalar(op0, res, bits, ins_va, lines)
                elif len(parsed) == 3:
                    op0, op1, op2 = parsed
                    bits = _op_width(op0)
                    v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, lines, tmp_id)
                    v2, tmp_id = _emit_read_scalar(op2, bits, ins_va, lines, tmp_id)
                    res = f"r{tmp_id}"
                    tmp_id += 1
                    lines.append(f"let {res} = c.imul2({v1}, {v2}, {bits});")
                    _emit_write_scalar(op0, res, bits, ins_va, lines)
                elif len(parsed) == 1:
                    op0 = parsed[0]
                    bits = _op_width(op0)
                    if bits not in (32, 64):
                        raise ValueError(f"+0x{rva:x}: unsupported 1-op imul width {ops}")
                    v0, tmp_id = _emit_read_scalar(op0, bits, ins_va, lines, tmp_id)
                    lines.append(f"c.imul1({v0}, {bits});")
                else:
                    raise ValueError(f"+0x{rva:x}: bad imul {ops}")
            elif mn.startswith("set") and mn[3:] in _CC_MAP:
                cc = _CC_MAP[mn[3:]]
                op0 = parsed[0]
                _emit_write_scalar(op0, f"(c.cc_{cc}() as u64)", 8, ins_va, lines)
            elif mn.startswith("cmov") and mn[4:] in _CC_MAP:
                cc = _CC_MAP[mn[4:]]
                op0, op1 = parsed
                if op0[0] != "gpr":
                    raise ValueError(f"+0x{rva:x}: non-gpr cmov {ops}")
                _, dst_i, bits, _ = op0
                sub_l = []
                v1, tmp_id = _emit_read_scalar(op1, bits, ins_va, sub_l, tmp_id)
                sub_l.append(f"c.w{bits}({dst_i}, {v1});")
                if bits == 32:
                    lines.append(
                        f"if c.cc_{cc}() {{ {' '.join(sub_l)} }} else {{ let cur = c.r32({dst_i}); c.w32({dst_i}, cur); }}"
                    )
                else:
                    lines.append(f"if c.cc_{cc}() {{ {' '.join(sub_l)} }}")
            elif mn in ("movsd", "movss"):
                op0, op1 = parsed
                suf = "sd" if mn == "movsd" else "ss"
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.mov{suf}_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.mov{suf}_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                elif op0[0] == "mem" and op1[0] == "xmm":
                    lines.append(f"if !c.mov{suf}_mr({op0[2]}, {op1[1]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
            elif mn == "movq":
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.movq_xx({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "gpr":
                    lines.append(f"c.movq_xr({op0[1]}, {op1[1]});")
                elif op0[0] == "gpr" and op1[0] == "xmm":
                    lines.append(f"c.movq_rx({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.movsd_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                elif op0[0] == "mem" and op1[0] == "xmm":
                    lines.append(f"if !c.movsd_mr({op0[2]}, {op1[1]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported movq {ops}")
            elif mn == "movd":
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "gpr":
                    lines.append(f"c.movd_xr({op0[1]}, {op1[1]});")
                elif op0[0] == "gpr" and op1[0] == "xmm":
                    lines.append(f"c.movd_rx({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.movss_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                elif op0[0] == "mem" and op1[0] == "xmm":
                    lines.append(f"if !c.movss_mr({op0[2]}, {op1[1]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported movd {ops}")
            elif mn in ("movaps", "movups", "movdqu", "movapd", "movupd", "movdqa"):
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.mov128_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.mov128_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                elif op0[0] == "mem" and op1[0] == "xmm":
                    lines.append(f"if !c.mov128_mr({op0[2]}, {op1[1]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
            elif mn in ("xorpd", "xorps", "pxor"):
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.xorpd_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.xorpd_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
            elif mn in ("andpd", "andps", "pand"):
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.and128_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.and128_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
            elif mn == "pcmpeqd":
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.pcmpeqd_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.pcmpeqd_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
            elif mn in ("psllq", "psrlq"):
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad {mn} {ops}")
                if op1[0] == "imm":
                    cnt_e = f"0x{op1[1]:x}"
                elif op1[0] == "xmm":
                    cnt_e = f"c.xmm_u64({op1[1]})"
                else:
                    cnt_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                lines.append(f"c.{mn}({op0[1]}, {cnt_e});")
            elif mn == "movmskpd":
                op0, op1 = parsed
                if op0[0] != "gpr" or op1[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad movmskpd {ops}")
                _, dst_i, dbits, _ = op0
                lines.append(f"let m = c.movmskpd({op1[1]}); c.w{dbits}({dst_i}, m);")
            elif mn == "punpckldq":
                op0, op1 = parsed
                if op0[0] == "xmm" and op1[0] == "xmm":
                    lines.append(f"c.punpckldq_rr({op0[1]}, {op1[1]});")
                elif op0[0] == "xmm" and op1[0] == "mem":
                    lines.append(f"if !c.punpckldq_rm({op0[1]}, {op1[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported punpckldq {ops}")
            elif mn in _SSE_ARITH_SD:
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad {mn} {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u64({op1[1]})"
                else:
                    b_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                lines.append(f"c.arith_sd({_SSE_ARITH_SD[mn]}, {op0[1]}, {b_e});")
            elif mn == "sqrtsd":
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad sqrtsd {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u64({op1[1]})"
                else:
                    b_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                lines.append(f"c.sqrtsd({op0[1]}, {b_e});")
            elif mn in ("ucomisd", "comisd"):
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad {mn} {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u64({op1[1]})"
                else:
                    b_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                lines.append(f"c.ucomisd(c.xmm_u64({op0[1]}), {b_e});")
            elif mn in ("ucomiss", "comiss"):
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad {mn} {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u32({op1[1]})"
                else:
                    t_s, tmp_id = _emit_read_scalar(op1, 32, ins_va, lines, tmp_id)
                    b_e = f"({t_s} as u32)"
                lines.append(f"c.ucomiss(c.xmm_u32({op0[1]}), {b_e});")
            elif mn == "cvtsi2sd":
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad cvtsi2sd {ops}")
                sbits = _op_width(op1)
                if sbits not in (32, 64):
                    raise ValueError(f"+0x{rva:x}: unsized cvtsi2sd {ops}")
                v1, tmp_id = _emit_read_scalar(op1, sbits, ins_va, lines, tmp_id)
                iv = f"({v1} as i64)" if sbits == 64 else f"({v1} as u32 as i32 as i64)"
                lines.append(f"c.cvtsi2sd({op0[1]}, {iv});")
            elif mn == "cvtsi2ss":
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad cvtsi2ss {ops}")
                sbits = _op_width(op1)
                if sbits not in (32, 64):
                    raise ValueError(f"+0x{rva:x}: unsized cvtsi2ss {ops}")
                v1, tmp_id = _emit_read_scalar(op1, sbits, ins_va, lines, tmp_id)
                iv = f"({v1} as i64)" if sbits == 64 else f"({v1} as u32 as i32 as i64)"
                lines.append(f"c.cvtsi2ss({op0[1]}, {iv});")
            elif mn == "cvttsd2si":
                op0, op1 = parsed
                if op0[0] != "gpr":
                    raise ValueError(f"+0x{rva:x}: bad cvttsd2si {ops}")
                _, dst_i, dbits, _ = op0
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u64({op1[1]})"
                else:
                    b_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                fn_c = "cvttsd2si64" if dbits == 64 else "cvttsd2si32"
                lines.append(f"let cv = c.{fn_c}({b_e}); c.w{dbits}({dst_i}, cv);")
            elif mn == "cvttss2si":
                op0, op1 = parsed
                if op0[0] != "gpr":
                    raise ValueError(f"+0x{rva:x}: bad cvttss2si {ops}")
                _, dst_i, dbits, _ = op0
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u32({op1[1]})"
                else:
                    t_s, tmp_id = _emit_read_scalar(op1, 32, ins_va, lines, tmp_id)
                    b_e = f"({t_s} as u32)"
                fn_c = "cvttss2si64" if dbits == 64 else "cvttss2si32"
                lines.append(f"let cv = c.{fn_c}({b_e}); c.w{dbits}({dst_i}, cv);")
            elif mn == "cvtsd2ss":
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad cvtsd2ss {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u64({op1[1]})"
                else:
                    b_e, tmp_id = _emit_read_scalar(op1, 64, ins_va, lines, tmp_id)
                lines.append(f"c.cvtsd2ss({op0[1]}, {b_e});")
            elif mn == "cvtss2sd":
                op0, op1 = parsed
                if op0[0] != "xmm":
                    raise ValueError(f"+0x{rva:x}: bad cvtss2sd {ops}")
                if op1[0] == "xmm":
                    b_e = f"c.xmm_u32({op1[1]})"
                else:
                    t_s, tmp_id = _emit_read_scalar(op1, 32, ins_va, lines, tmp_id)
                    b_e = f"({t_s} as u32)"
                lines.append(f"c.cvtss2sd({op0[1]}, {b_e});")
            elif mn in ("popfq", "popf"):
                lines.append(f"if !c.popfq() {{ return RegfileExit::fault(0x{ins_va:x}); }}")
            elif mn in ("fld", "fldl"):
                op0 = parsed[0]
                if op0[0] != "mem" or op0[1] not in (0, 64):
                    raise ValueError(f"+0x{rva:x}: unsupported fld {ops}")
                lines.append(f"if !c.fld_m64({op0[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
            elif mn == "fprem":
                lines.append("c.fprem();")
            elif mn in ("fnstsw", "fstsw"):
                op0 = parsed[0]
                if op0 != ("gpr", 0, 16, False):
                    raise ValueError(f"+0x{rva:x}: unsupported {mn} {ops}")
                lines.append("c.fnstsw_ax();")
            elif mn in ("fstp", "fstpl"):
                op0 = parsed[0]
                if op0[0] == "st":
                    lines.append(f"c.fstp_st({op0[1]});")
                elif op0[0] == "mem" and op0[1] in (0, 64):
                    lines.append(f"if !c.fstp_m64({op0[2]}) {{ return RegfileExit::fault(0x{ins_va:x}); }}")
                else:
                    raise ValueError(f"+0x{rva:x}: unsupported fstp {ops}")
            else:
                raise ValueError(f"+0x{rva:x}: unhandled mnemonic {mn} {ops}")

        if not terminated:
            if bi + 1 < len(blocks):
                lines.append(f"blk = {bi + 1}; continue;")
            else:
                lines.append(f"return RegfileExit::jump(0x{base + e_rva:x});")

        if multi:
            body_s = "\n            ".join(lines)
            blk_arms.append(f"        {bi} => {{\n            {body_s}\n        }}")
        else:
            body_s = "\n    ".join(lines)
            blk_arms.append(f"    {body_s}")

    fn_va = base + b_rva
    if multi:
        arms = "\n".join(blk_arms)
        code = (
            f"pub unsafe extern \"C\" fn rf_{fn_va:09x}(raw: *mut u8, mut blk: u32) -> RegfileExit {{\n"
            f"    let mut c = ctx(raw);\n"
            f"    loop {{\n"
            f"        match blk {{\n"
            f"{arms}\n"
            f"            _ => unreachable!(),\n"
            f"        }}\n"
            f"    }}\n"
            f"}}\n"
        )
        return code, conts
    if "c." not in blk_arms[0]:
        code = (
            f"pub unsafe extern \"C\" fn rf_{fn_va:09x}(_raw: *mut u8, _blk: u32) -> RegfileExit {{\n"
            f"{blk_arms[0]}\n"
            f"}}\n"
        )
        return code, conts
    code = (
        f"pub unsafe extern \"C\" fn rf_{fn_va:09x}(raw: *mut u8, _blk: u32) -> RegfileExit {{\n"
        f"    let mut c = ctx(raw);\n"
        f"{blk_arms[0]}\n"
        f"}}\n"
    )
    return code, conts


def lift_nonabi(pe, dis, base: int, na_lo: int, na_hi: int) -> dict:
    """Discover and lift all V8 embedded builtins inside `[na_lo, na_hi)`.

    Returns `{
      "lifted": [(va, rva, inst_len, bid, code_str, conts), ...],
      "rejected": [(va, rva, bid, reason), ...],
      "total": int,
    }`.
    """
    builtins = discover_embedded_builtins(pe, base, na_lo, na_hi)
    lifted: list[tuple[int, int, int, int, str, list[tuple[int, int]]]] = []
    rejected: list[tuple[int, int, int, str]] = []

    for bid, b_rva, inst_len in builtins:
        e_rva = b_rva + inst_len
        va = base + b_rva
        raw_slice = dis.range(b_rva, e_rva + 16)
        if not raw_slice or raw_slice[0][0] != b_rva:
            if not dis.resync(b_rva, min(e_rva + 32, pe.text_end)):
                rejected.append((va, b_rva, bid, "v8_nonabi_desync: failed to decode builtin start"))
                continue
            raw_slice = dis.range(b_rva, e_rva + 16)

        insns: list[tuple[int, str, str, int]] = []
        for idx, (a, mn, ops, _parts) in enumerate(raw_slice):
            if a >= e_rva:
                break
            if idx + 1 < len(raw_slice):
                nxt_a = raw_slice[idx + 1][0]
            else:
                ni = bisect.bisect_right(dis.addrs, a)
                nxt_a = dis.addrs[ni] if ni < len(dis.addrs) else e_rva
            insns.append((a, mn, ops, nxt_a))

        if not insns:
            rejected.append((va, b_rva, bid, "v8_nonabi_empty: 0 instructions"))
            continue

        code, conts = _lower_builtin(base, pe.file_base, b_rva, e_rva, insns)
        lifted.append((va, b_rva, inst_len, bid, code, conts))

    return {
        "lifted": lifted,
        "rejected": rejected,
        "total": len(builtins),
    }
