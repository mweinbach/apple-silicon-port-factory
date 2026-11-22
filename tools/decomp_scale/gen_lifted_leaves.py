#!/usr/bin/env python3
"""
Generate crates/aw2-game/src/shards/lifted_leaves.rs by lifting remaining
small Remedy functions (5..512 bytes, 0..8 callees) from AlanWake2.exe into
deduplicated const-generic Rust functions.
"""

from collections import deque
import bisect
import re
import sqlite3
import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "fndb"))
import canthrow  # noqa: E402
import fndb  # noqa: E402


def fast_eh_info(pe, va: int) -> str:
    rva = va - pe.image_base
    ents = getattr(pe, "_pdata_sorted", None)
    starts = getattr(pe, "_pdata_starts", None)
    if ents is None:
        ents = pe._pdata_sorted = sorted(pe.pdata())
        starts = pe._pdata_starts = [e[0] for e in ents]
    i = bisect.bisect_right(starts, rva) - 1
    if i < 0 or not (ents[i][0] <= rva < ents[i][1]):
        return "none"
    d = pe.data
    u = ents[i][2]
    for _ in range(64):
        if u & 1:
            u = struct.unpack_from("<I", d, pe.off(u & ~1) + 8)[0]
            continue
        uo = pe.off(u)
        fl, cnt = d[uo] >> 3, d[uo + 2]
        tail = uo + 4 + ((cnt + 1) & ~1) * 2
        if fl & 4:
            u = struct.unpack_from("<I", d, tail + 8)[0]
            continue
        break
    if not (fl & 3):
        return "none"
    h = pe.image_base + struct.unpack_from("<I", d, tail)[0]
    data = tail + 4
    if h in (canthrow.H_GS, canthrow.H_GS_FH4, canthrow.H_GS_FH3):
        return "gs"
    if h == canthrow.H_FH4:
        hdr = d[pe.off(struct.unpack_from("<I", d, data)[0])]
        if hdr & 0x10:
            return "catch"
        if hdr & 0x40:
            return "noexcept"
        return "cleanup" if hdr & 0x08 else "none"
    if h == canthrow.H_FH3:
        o = pe.off(struct.unpack_from("<I", d, data)[0])
        ntry = struct.unpack_from("<I", d, o + 12)[0]
        maxstate = struct.unpack_from("<i", d, o + 4)[0]
        return "catch" if ntry else ("cleanup" if maxstate > 0 else "none")
    if h == canthrow.H_SEH:
        n = struct.unpack_from("<I", d, data)[0]
        targets = [
            struct.unpack_from("<I", d, data + 4 + 16 * k + 12)[0]
            for k in range(min(n, 256))
        ]
        return "catch" if any(targets) else ("cleanup" if n else "none")
    return f"unknown:0x{h:x}"


def build_elf64(code_bytes: bytes, base_vaddr: int = 0x140000000) -> bytes:
    """Wrap raw x86_64 machine code bytes in a minimal 64-bit ELF for fast batch objdump."""
    ehdr_size = 64
    phdr_size = 56
    shdr_size = 64
    shstrtab = b"\x00.text\x00.shstrtab\x00"
    code_offset = 0x1000
    shstr_offset = code_offset + len(code_bytes)
    shstr_offset = (shstr_offset + 7) & ~7
    shdr_offset = shstr_offset + len(shstrtab)
    shdr_offset = (shdr_offset + 7) & ~7

    e_ident = b"\x7fELF\x02\x01\x01\x00" + b"\x00" * 8
    ehdr = struct.pack(
        "<16sHHIQQQIHHHHHH",
        e_ident,
        2,  # ET_EXEC
        62,  # EM_X86_64
        1,
        base_vaddr,
        ehdr_size,
        shdr_offset,
        0,
        ehdr_size,
        phdr_size,
        1,
        shdr_size,
        3,
        2,
    )
    phdr = struct.pack(
        "<IIQQQQQQ",
        1,  # PT_LOAD
        5,  # PF_R | PF_X
        code_offset,
        base_vaddr,
        base_vaddr,
        len(code_bytes),
        len(code_bytes),
        0x1000,
    )
    shdr_null = b"\x00" * 64
    shdr_text = struct.pack(
        "<IIQQQQIIQQ",
        1,
        1,  # SHT_PROGBITS
        6,  # SHF_ALLOC | SHF_EXECINSTR
        base_vaddr,
        code_offset,
        len(code_bytes),
        0,
        0,
        16,
        0,
    )
    shdr_str = struct.pack(
        "<IIQQQQIIQQ",
        7,
        3,  # SHT_STRTAB
        0,
        0,
        shstr_offset,
        len(shstrtab),
        0,
        0,
        1,
        0,
    )

    buf = bytearray(shdr_offset + 3 * shdr_size)
    buf[0:ehdr_size] = ehdr
    buf[ehdr_size : ehdr_size + phdr_size] = phdr
    buf[code_offset : code_offset + len(code_bytes)] = code_bytes
    buf[shstr_offset : shstr_offset + len(shstrtab)] = shstrtab
    buf[shdr_offset : shdr_offset + 64] = shdr_null
    buf[shdr_offset + 64 : shdr_offset + 128] = shdr_text
    buf[shdr_offset + 128 : shdr_offset + 192] = shdr_str
    return bytes(buf)


GS_RE = re.compile(rb"\x65[\x48\x4c\x44\x8b]", re.DOTALL)

XMM_ONLY_RE = re.compile(
    rb"\x0f[\x10-\x17\x28-\x2f\x50-\x5f\x6e\x7e\xd6]|[\xf3\xf2\x66]\x0f[\x10-\x17\x28-\x2f\x50-\x5f\x6e\x6f\x70\x7e\x7f\xd6\xef]",
    re.DOTALL,
)


def has_gs(b: bytes) -> bool:
    return GS_RE.search(b) is not None


REG_MAP = {
    "rax": ("rax", 64, "low"),
    "eax": ("rax", 32, "low"),
    "ax": ("rax", 16, "low"),
    "al": ("rax", 8, "low"),
    "ah": ("rax", 8, "high"),
    "rcx": ("rcx", 64, "low"),
    "ecx": ("rcx", 32, "low"),
    "cx": ("rcx", 16, "low"),
    "cl": ("rcx", 8, "low"),
    "ch": ("rcx", 8, "high"),
    "rdx": ("rdx", 64, "low"),
    "edx": ("rdx", 32, "low"),
    "dx": ("rdx", 16, "low"),
    "dl": ("rdx", 8, "low"),
    "dh": ("rdx", 8, "high"),
    "rbx": ("rbx", 64, "low"),
    "ebx": ("rbx", 32, "low"),
    "bx": ("rbx", 16, "low"),
    "bl": ("rbx", 8, "low"),
    "bh": ("rbx", 8, "high"),
    "rsp": ("rsp", 64, "low"),
    "esp": ("rsp", 32, "low"),
    "sp": ("rsp", 16, "low"),
    "spl": ("rsp", 8, "low"),
    "rbp": ("rbp", 64, "low"),
    "ebp": ("rbp", 32, "low"),
    "bp": ("rbp", 16, "low"),
    "bpl": ("rbp", 8, "low"),
    "rsi": ("rsi", 64, "low"),
    "esi": ("rsi", 32, "low"),
    "si": ("rsi", 16, "low"),
    "sil": ("rsi", 8, "low"),
    "rdi": ("rdi", 64, "low"),
    "edi": ("rdi", 32, "low"),
    "di": ("rdi", 16, "low"),
    "dil": ("rdi", 8, "low"),
    "r8": ("r8", 64, "low"),
    "r8d": ("r8", 32, "low"),
    "r8w": ("r8", 16, "low"),
    "r8b": ("r8", 8, "low"),
    "r9": ("r9", 64, "low"),
    "r9d": ("r9", 32, "low"),
    "r9w": ("r9", 16, "low"),
    "r9b": ("r9", 8, "low"),
    "r10": ("r10", 64, "low"),
    "r10d": ("r10", 32, "low"),
    "r10w": ("r10", 16, "low"),
    "r10b": ("r10", 8, "low"),
    "r11": ("r11", 64, "low"),
    "r11d": ("r11", 32, "low"),
    "r11w": ("r11", 16, "low"),
    "r11b": ("r11", 8, "low"),
    "r12": ("r12", 64, "low"),
    "r12d": ("r12", 32, "low"),
    "r12w": ("r12", 16, "low"),
    "r12b": ("r12", 8, "low"),
    "r13": ("r13", 64, "low"),
    "r13d": ("r13", 32, "low"),
    "r13w": ("r13", 16, "low"),
    "r13b": ("r13", 8, "low"),
    "r14": ("r14", 64, "low"),
    "r14d": ("r14", 32, "low"),
    "r14w": ("r14", 16, "low"),
    "r14b": ("r14", 8, "low"),
    "r15": ("r15", 64, "low"),
    "r15d": ("r15", 32, "low"),
    "r15w": ("r15", 16, "low"),
    "r15b": ("r15", 8, "low"),
    "xmm0": ("xmm0", 128, "low"),
    "xmm1": ("xmm1", 128, "low"),
    "xmm2": ("xmm2", 128, "low"),
    "xmm3": ("xmm3", 128, "low"),
    "xmm4": ("xmm4", 128, "low"),
    "xmm5": ("xmm5", 128, "low"),
}

CALLEE_SAVED_PARENTS = ("rbx", "rbp", "rsi", "rdi", "r12", "r13", "r14", "r15")
XMM_PARENTS = ("xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5")

XMM_ZERO_MNS = {"vxorps", "vxorpd", "vpxor", "xorps", "xorpd", "pxor"}
XMM_MOV_MNS = {
    "vmovups",
    "vmovaps",
    "vmovdqu",
    "vmovdqa",
    "movups",
    "movaps",
    "movdqu",
    "movdqa",
}
ALLOWED_V_MNS = XMM_ZERO_MNS | XMM_MOV_MNS | {"vzeroupper"}

PTR_SIZE = {
    "BYTE": 8,
    "WORD": 16,
    "DWORD": 32,
    "QWORD": 64,
    "XMMWORD": 128,
}

MEM_RE = re.compile(
    r"^(?:(BYTE|WORD|DWORD|QWORD|XMMWORD)\s+PTR\s+)?\[([^\]]+)\]$", re.IGNORECASE
)
RIP_RE = re.compile(r"\brip\s*([+-])\s*(0x[0-9a-fA-F]+|\d+)", re.IGNORECASE)
RSP_WORD_RE = re.compile(r"\b(rsp|esp|sp|spl)\b", re.IGNORECASE)
RSP_BRACKET_RE = re.compile(
    r"\[\s*rsp(?:\s*([+-])\s*(0x[0-9a-fA-F]+|\d+))?\s*\]", re.IGNORECASE
)
RBP_WORD_RE = re.compile(r"\b(rbp|ebp|bp|bpl)\b", re.IGNORECASE)
RBP_BRACKET_RE = re.compile(
    r"\[\s*rbp(?:\s*([+-])\s*(0x[0-9a-fA-F]+|\d+))?\s*\]", re.IGNORECASE
)
XMM_WORD_RE = re.compile(r"\bxmm\d+\b", re.IGNORECASE)


def parse_addr_expr(expr: str):
    expr = expr.strip()
    tokens = re.split(r"(\+|\-)", expr)
    terms = []
    sign = "+"
    for tok in tokens:
        tok = tok.strip()
        if not tok:
            continue
        if tok in ("+", "-"):
            sign = tok
            continue
        if "*" in tok:
            parts = [p.strip() for p in tok.split("*")]
            if len(parts) != 2:
                return None
            if parts[0] in REG_MAP:
                r, scale = parts[0], parts[1]
            elif parts[1] in REG_MAP:
                scale, r = parts[0], parts[1]
            else:
                return None
            parent, bits, hl = REG_MAP[r]
            if hl != "low" or bits not in (32, 64):
                return None
            r_expr = parent if bits == 64 else f"({parent} as u32 as u64)"
            try:
                s_val = int(scale, 0)
            except ValueError:
                return None
            term = f"{r_expr}.wrapping_mul({s_val})"
        elif tok.startswith("RIP_P_"):
            p_idx = int(tok.split("_")[2])
            term = f"P{p_idx}"
        elif tok in REG_MAP:
            parent, bits, hl = REG_MAP[tok]
            if hl != "low" or bits not in (32, 64):
                return None
            term = parent if bits == 64 else f"({parent} as u32 as u64)"
        else:
            try:
                val = int(tok, 0)
                term = f"0x{val:x}u64"
            except ValueError:
                return None
        if not terms:
            if sign == "-":
                terms.append(f"0u64.wrapping_sub({term})")
            else:
                terms.append(term)
        else:
            op = "wrapping_add" if sign == "+" else "wrapping_sub"
            terms[-1] = f"{terms[-1]}.{op}({term})"
    if not terms:
        return None
    return terms[0]


def parse_op(op_str: str):
    op_str = op_str.strip()
    if op_str in REG_MAP:
        parent, bits, hl = REG_MAP[op_str]
        return ("reg", parent, bits, hl)
    m = MEM_RE.match(op_str)
    if m:
        sz_name = m.group(1).upper() if m.group(1) else None
        inner = m.group(2)
        bits = PTR_SIZE.get(sz_name, 0)
        addr_rust = parse_addr_expr(inner)
        if addr_rust is None:
            return None
        return ("mem", bits, addr_rust)
    try:
        val = int(op_str, 0)
        return ("imm", val)
    except ValueError:
        return None


def op_bits(op):
    if op is None:
        return None
    if op[0] == "reg":
        return op[2]
    if op[0] == "mem":
        return op[1] if op[1] != 0 else None
    return None


def read_reg(parent: str, bits: int, hl: str) -> str:
    if bits in (64, 128):
        return parent
    if bits == 32:
        return f"({parent} as u32)"
    if bits == 16:
        return f"({parent} as u16)"
    if bits == 8:
        if hl == "high":
            return f"(({parent} >> 8) as u8)"
        return f"({parent} as u8)"
    raise ValueError(bits)


def write_reg(parent: str, bits: int, hl: str, val_expr: str) -> str:
    if bits == 128:
        return f"{parent} = ({val_expr}) as u128;"
    if bits == 64:
        return f"{parent} = ({val_expr}) as u64;"
    if bits == 32:
        return f"{parent} = ({val_expr}) as u32 as u64;"
    if bits == 16:
        return f"{parent} = ({parent} & !0xffffu64) | ((({val_expr}) as u16 as u64));"
    if bits == 8:
        if hl == "high":
            return f"{parent} = ({parent} & !0xff00u64) | ((({val_expr}) as u8 as u64) << 8);"
        return f"{parent} = ({parent} & !0xffu64) | ((({val_expr}) as u8 as u64));"
    raise ValueError(bits)


def read_op(op, expected_bits: int):
    kind = op[0]
    if kind == "reg":
        _, parent, bits, hl = op
        if expected_bits and bits != expected_bits:
            return None
        return read_reg(parent, bits, hl)
    if kind == "mem":
        _, bits, addr = op
        b = bits or expected_bits
        if b not in (8, 16, 32, 64, 128):
            return None
        return f"core::ptr::read_unaligned(({addr}) as *const u{b})"
    if kind == "imm":
        val = op[1]
        b = expected_bits or 64
        mask = (1 << b) - 1
        return f"0x{val & mask:x}u{b}"
    return None


def write_op(op, val_expr: str, expected_bits: int = 0):
    kind = op[0]
    if kind == "reg":
        _, parent, bits, hl = op
        return write_reg(parent, bits, hl, val_expr)
    if kind == "mem":
        _, bits, addr = op
        b = bits or expected_bits
        if b not in (8, 16, 32, 64, 128):
            return None
        return f"core::ptr::write_unaligned(({addr}) as *mut u{b}, ({val_expr}) as u{b});"
    return None


def cond_expr(cc: str, last_flag):
    if last_flag is None:
        return None
    fkind, bits, l_expr, r_expr = last_flag
    st = f"i{bits}"
    if fkind == "test":
        and_expr = f"(({l_expr}) & ({r_expr}))"
        if cc in ("e", "z"):
            return f"({and_expr} == 0)"
        if cc in ("ne", "nz"):
            return f"({and_expr} != 0)"
        if cc in ("s", "l"):
            return f"(({and_expr} as {st}) < 0)"
        if cc in ("ns", "ge"):
            return f"(({and_expr} as {st}) >= 0)"
        if cc == "g":
            return f"(({and_expr} as {st}) > 0)"
        if cc == "le":
            return f"(({and_expr} as {st}) <= 0)"
        if cc in ("a", "nbe"):
            return f"({and_expr} != 0)"
        if cc in ("be", "na"):
            return f"({and_expr} == 0)"
        if cc in ("b", "c", "nae"):
            return "false"
        if cc in ("ae", "nb", "nc"):
            return "true"
        return None
    elif fkind == "cmp":
        if cc in ("e", "z"):
            return f"(({l_expr}) == ({r_expr}))"
        if cc in ("ne", "nz"):
            return f"(({l_expr}) != ({r_expr}))"
        if cc in ("b", "c", "nae"):
            return f"(({l_expr}) < ({r_expr}))"
        if cc in ("ae", "nb", "nc"):
            return f"(({l_expr}) >= ({r_expr}))"
        if cc in ("be", "na"):
            return f"(({l_expr}) <= ({r_expr}))"
        if cc in ("a", "nbe"):
            return f"(({l_expr}) > ({r_expr}))"
        if cc in ("l", "nge"):
            return f"((({l_expr}) as {st}) < (({r_expr}) as {st}))"
        if cc in ("ge", "nl"):
            return f"((({l_expr}) as {st}) >= (({r_expr}) as {st}))"
        if cc in ("le", "ng"):
            return f"((({l_expr}) as {st}) <= (({r_expr}) as {st}))"
        if cc in ("g", "nle"):
            return f"((({l_expr}) as {st}) > (({r_expr}) as {st}))"
        if cc == "s":
            return f"(((({l_expr}).wrapping_sub({r_expr})) as {st}) < 0)"
        if cc == "ns":
            return f"(((({l_expr}).wrapping_sub({r_expr})) as {st}) >= 0)"
        return None
    elif fkind == "bt":
        if cc in ("b", "c", "nae"):
            return f"({l_expr} != 0)"
        if cc in ("ae", "nb", "nc"):
            return f"({l_expr} == 0)"
        return None
    elif fkind == "z_s":
        if cc in ("e", "z"):
            return f"(({l_expr}) == 0)"
        if cc in ("ne", "nz"):
            return f"(({l_expr}) != 0)"
        if cc == "s":
            return f"((({l_expr}) as {st}) < 0)"
        if cc == "ns":
            return f"((({l_expr}) as {st}) >= 0)"
        return None
    return None


def split_operands(ops_str: str):
    if not ops_str.strip():
        return []
    return [p.strip() for p in ops_str.split(",")]


FLAG_CLOBBER_MNS = {
    "cmp",
    "test",
    "add",
    "sub",
    "and",
    "or",
    "xor",
    "inc",
    "dec",
    "neg",
    "shl",
    "shr",
    "sar",
    "rol",
    "ror",
    "imul",
    "mul",
    "div",
    "idiv",
    "bt",
    "bts",
    "btr",
    "btc",
    "call",
    "sbb",
}


def flags_observed_after(insns, idx: int) -> bool:
    for j in range(idx + 1, len(insns)):
        mn = insns[j][1]
        if (
            mn.startswith("set")
            or mn.startswith("cmov")
            or mn == "sbb"
            or (mn.startswith("j") and mn != "jmp")
        ):
            return True
        if mn in FLAG_CLOBBER_MNS:
            return False
    return True


def translate_linear_insns(
    insns, initial_flag=None, addr_to_bb=None, uses_stack: bool = False
):
    lines = []
    last_flag = initial_flag
    for idx, (addr, mn, ops_str, next_addr) in enumerate(insns):
        ops = split_operands(ops_str)
        if mn in ("nop", "vzeroupper"):
            continue
        elif mn == "ret":
            if idx != len(insns) - 1:
                return None, None
            lines.append("return rax;")
        elif mn in XMM_ZERO_MNS:
            if len(ops) == 3 and ops[0] == ops[1] == ops[2]:
                dst = parse_op(ops[0])
            elif len(ops) == 2 and ops[0] == ops[1]:
                dst = parse_op(ops[0])
            else:
                return None, None
            if not dst or dst[0] != "reg" or dst[2] != 128:
                return None, None
            lines.append(write_op(dst, "0u128", 128))
        elif mn in XMM_MOV_MNS:
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src:
                return None, None
            if op_bits(dst) != 128 and op_bits(src) != 128:
                return None, None
            s_expr = read_op(src, 128)
            if s_expr is None:
                return None, None
            w_stmt = write_op(dst, s_expr, 128)
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
        elif mn == "push":
            if len(ops) != 1:
                return None, None
            src = parse_op(ops[0])
            if not src:
                return None, None
            if src[0] == "reg" and (src[2] != 64 or src[1] == "rsp"):
                return None, None
            s_expr = read_op(src, 64)
            if s_expr is None:
                return None, None
            lines.append("rsp = rsp.wrapping_sub(8);")
            lines.append(f"core::ptr::write_unaligned(rsp as *mut u64, ({s_expr}) as u64);")
        elif mn == "pop":
            if len(ops) != 1:
                return None, None
            dst = parse_op(ops[0])
            if not dst or dst[0] != "reg" or dst[2] != 64 or dst[1] == "rsp":
                return None, None
            lines.append(f"{dst[1]} = core::ptr::read_unaligned(rsp as *const u64);")
            lines.append("rsp = rsp.wrapping_add(8);")
        elif mn in ("mov", "movabs"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src:
                return None, None
            dbits = op_bits(dst)
            if dbits is None or dbits == 128:
                return None, None
            s_expr = read_op(src, dbits)
            if s_expr is None:
                return None, None
            w_stmt = write_op(dst, s_expr, dbits)
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
        elif mn == "movzx":
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg":
                return None, None
            dbits = dst[2]
            sbits = op_bits(src)
            if not sbits or dbits == 128:
                return None, None
            s_expr = read_op(src, sbits)
            if s_expr is None:
                return None, None
            lines.append(write_op(dst, f"({s_expr}) as u{dbits}", dbits))
        elif mn in ("movsx", "movsxd"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg":
                return None, None
            dbits = dst[2]
            sbits = op_bits(src) or (32 if mn == "movsxd" else None)
            if not sbits or dbits == 128:
                return None, None
            s_expr = read_op(src, sbits)
            if s_expr is None:
                return None, None
            lines.append(write_op(dst, f"(({s_expr}) as i{sbits} as i{dbits}) as u{dbits}", dbits))
        elif mn == "lea":
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg" or src[0] != "mem" or dst[2] == 128:
                return None, None
            dbits = dst[2]
            addr_expr = src[2]
            lines.append(write_op(dst, f"({addr_expr}) as u{dbits}", dbits))
        elif mn in ("add", "sub", "and", "or", "xor"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src:
                return None, None
            if mn == "xor" and dst == src and dst[0] == "reg" and dst[2] != 128:
                lines.append(write_op(dst, f"0u{dst[2]}", dst[2]))
                last_flag = ("test", dst[2], f"0u{dst[2]}", f"0u{dst[2]}")
                continue
            dbits = op_bits(dst)
            if not dbits or dbits == 128:
                return None, None
            d_expr = read_op(dst, dbits)
            if src[0] == "imm":
                imm_val = src[1] & ((1 << dbits) - 1)
                s_expr = f"0x{imm_val:x}u{dbits}"
            else:
                s_expr = read_op(src, dbits)
            if d_expr is None or s_expr is None:
                return None, None
            is_rsp = dst[0] == "reg" and dst[1] == "rsp"
            obs = (not is_rsp) and flags_observed_after(insns, idx)
            if mn == "sub" and obs:
                lines.append(f"let fl_l: u{dbits} = {d_expr}; let fl_r: u{dbits} = {s_expr};")
                w_stmt = write_op(dst, "fl_l.wrapping_sub(fl_r)", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("cmp", dbits, "fl_l", "fl_r")
                continue
            if mn == "add":
                res = f"({d_expr}).wrapping_add({s_expr})"
            elif mn == "sub":
                res = f"({d_expr}).wrapping_sub({s_expr})"
            elif mn == "and":
                res = f"(({d_expr}) & ({s_expr}))"
            elif mn == "or":
                res = f"(({d_expr}) | ({s_expr}))"
            elif mn == "xor":
                res = f"(({d_expr}) ^ ({s_expr}))"
            if obs:
                lines.append(f"let fl_res: u{dbits} = {res};")
                w_stmt = write_op(dst, "fl_res", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                if mn in ("and", "or", "xor"):
                    last_flag = ("test", dbits, "fl_res", "fl_res")
                else:
                    last_flag = ("z_s", dbits, "fl_res", "0")
            else:
                w_stmt = write_op(dst, res, dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = None
        elif mn in ("inc", "dec", "not", "neg"):
            if len(ops) != 1:
                return None, None
            dst = parse_op(ops[0])
            if not dst:
                return None, None
            dbits = op_bits(dst)
            if not dbits or dbits == 128:
                return None, None
            d_expr = read_op(dst, dbits)
            if d_expr is None:
                return None, None
            obs = flags_observed_after(insns, idx)
            if mn == "neg" and obs:
                lines.append(f"let fl_neg: u{dbits} = {d_expr};")
                w_stmt = write_op(dst, f"0u{dbits}.wrapping_sub(fl_neg)", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("cmp", dbits, f"0u{dbits}", "fl_neg")
                continue
            if mn == "inc":
                res = f"({d_expr}).wrapping_add(1)"
            elif mn == "dec":
                res = f"({d_expr}).wrapping_sub(1)"
            elif mn == "not":
                res = f"!({d_expr})"
            elif mn == "neg":
                res = f"0u{dbits}.wrapping_sub({d_expr})"
            if mn in ("inc", "dec") and obs:
                lines.append(f"let fl_res: u{dbits} = {res};")
                w_stmt = write_op(dst, "fl_res", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("z_s", dbits, "fl_res", "0")
            else:
                w_stmt = write_op(dst, res, dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                if mn != "not":
                    last_flag = None
        elif mn in ("shl", "shr", "sar", "rol", "ror"):
            if len(ops) not in (1, 2):
                return None, None
            dst = parse_op(ops[0])
            if not dst:
                return None, None
            dbits = op_bits(dst)
            if not dbits or dbits == 128:
                return None, None
            d_expr = read_op(dst, dbits)
            if d_expr is None:
                return None, None
            mask = 63 if dbits == 64 else 31
            if len(ops) == 1:
                sh_expr = "1u32"
            else:
                src = parse_op(ops[1])
                if not src:
                    return None, None
                if src[0] == "imm":
                    sh = src[1] & mask
                    sh_expr = f"{sh}u32"
                elif src == ("reg", "rcx", 8, "low"):
                    sh_expr = f"((rcx as u32) & {mask})"
                else:
                    return None, None
            narrow_var = dbits in (8, 16) and not (sh_expr.endswith("u32") and int(sh_expr[:-3]) < dbits)
            if mn in ("shl", "shr", "sar") and narrow_var:
                # x86 masks the count to 5 bits even for r8/r16 (see crate::lift_ops).
                res = f"crate::lift_ops::{mn}{dbits}({d_expr}, {sh_expr})"
            elif mn == "shl":
                res = f"({d_expr}).wrapping_shl({sh_expr})"
            elif mn == "shr":
                res = f"({d_expr}).wrapping_shr({sh_expr})"
            elif mn == "sar":
                res = f"((({d_expr}) as i{dbits}).wrapping_shr({sh_expr}) as u{dbits})"
            elif mn == "rol":
                res = f"({d_expr}).rotate_left({sh_expr})"
            elif mn == "ror":
                res = f"({d_expr}).rotate_right({sh_expr})"
            obs = mn in ("shl", "shr", "sar") and flags_observed_after(insns, idx)
            if obs:
                lines.append(f"let fl_res: u{dbits} = {res};")
                w_stmt = write_op(dst, "fl_res", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("z_s", dbits, "fl_res", "0")
            else:
                w_stmt = write_op(dst, res, dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = None
        elif mn == "bswap":
            if len(ops) != 1:
                return None, None
            dst = parse_op(ops[0])
            if not dst or dst[0] != "reg" or dst[2] not in (32, 64):
                return None, None
            dbits = dst[2]
            d_expr = read_op(dst, dbits)
            lines.append(write_op(dst, f"({d_expr}).swap_bytes()", dbits))
        elif mn in ("bt", "bts", "btr", "btc"):
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2:
                return None, None
            bits = op_bits(op1) or op_bits(op2)
            if bits not in (16, 32, 64):
                return None, None
            mask = bits - 1
            e1 = read_op(op1, bits)
            if e1 is None:
                return None, None
            if op2[0] == "imm":
                b_expr = f"{op2[1] & mask}u32"
            else:
                e2 = read_op(op2, bits)
                if e2 is None:
                    return None, None
                b_expr = f"(({e2}) as u32 & {mask})"
            lines.append(f"let fl_bt: u{bits} = (({e1}) >> ({b_expr})) & 1;")
            if mn != "bt":
                if mn == "bts":
                    mod_expr = f"({e1}) | ((1u{bits}) << ({b_expr}))"
                elif mn == "btr":
                    mod_expr = f"({e1}) & !((1u{bits}) << ({b_expr}))"
                elif mn == "btc":
                    mod_expr = f"({e1}) ^ ((1u{bits}) << ({b_expr}))"
                w_stmt = write_op(op1, mod_expr, bits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
            last_flag = ("bt", bits, "fl_bt", "0")
        elif mn == "sbb":
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst != src or dst[0] != "reg" or dst[2] == 128:
                return None, None
            dbits = dst[2]
            c_expr = cond_expr("b", last_flag)
            if c_expr is None:
                return None, None
            mask_hex = f"0x{(1 << dbits) - 1:x}u{dbits}"
            lines.append(write_op(dst, f"if {c_expr} {{ {mask_hex} }} else {{ 0u{dbits} }}", dbits))
            last_flag = None
        elif mn == "imul":
            if len(ops) == 1:
                op = parse_op(ops[0])
                if not op:
                    return None, None
                bits = op_bits(op)
                if bits not in (32, 64):
                    return None, None
                s_expr = read_op(op, bits)
                if s_expr is None:
                    return None, None
                if bits == 64:
                    lines.append(
                        f"let prod = (rax as i64 as i128).wrapping_mul(({s_expr}) as i64 as i128) as u128;"
                    )
                    lines.append("rax = prod as u64; rdx = (prod >> 64) as u64;")
                else:
                    lines.append(
                        f"let prod = (rax as u32 as i32 as i64).wrapping_mul(({s_expr}) as i32 as i64) as u64;"
                    )
                    lines.append("rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64;")
            elif len(ops) == 2:
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                if not dst or not src or dst[0] != "reg" or dst[2] == 128:
                    return None, None
                dbits = dst[2]
                d_expr = read_op(dst, dbits)
                s_expr = read_op(src, dbits)
                if d_expr is None or s_expr is None:
                    return None, None
                lines.append(
                    write_op(
                        dst,
                        f"(({d_expr}) as i{dbits}).wrapping_mul(({s_expr}) as i{dbits}) as u{dbits}",
                        dbits,
                    )
                )
            elif len(ops) == 3:
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
                if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] == 128:
                    return None, None
                dbits = dst[2]
                s1_expr = read_op(src1, dbits)
                s2_expr = read_op(src2, dbits)
                if s1_expr is None or s2_expr is None:
                    return None, None
                lines.append(
                    write_op(
                        dst,
                        f"(({s1_expr}) as i{dbits}).wrapping_mul(({s2_expr}) as i{dbits}) as u{dbits}",
                        dbits,
                    )
                )
            else:
                return None, None
            last_flag = None
        elif mn == "mul":
            if len(ops) != 1:
                return None, None
            op = parse_op(ops[0])
            if not op:
                return None, None
            bits = op_bits(op)
            if bits not in (32, 64):
                return None, None
            s_expr = read_op(op, bits)
            if s_expr is None:
                return None, None
            if bits == 64:
                lines.append(f"let prod = (rax as u128).wrapping_mul(({s_expr}) as u128);")
                lines.append("rax = prod as u64; rdx = (prod >> 64) as u64;")
            else:
                lines.append(f"let prod = (rax as u32 as u64).wrapping_mul(({s_expr}) as u32 as u64);")
                lines.append("rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64;")
            last_flag = None
        elif mn == "div":
            if len(ops) != 1:
                return None, None
            op = parse_op(ops[0])
            if not op:
                return None, None
            bits = op_bits(op)
            if bits not in (32, 64):
                return None, None
            s_expr = read_op(op, bits)
            if s_expr is None:
                return None, None
            if bits == 64:
                lines.append(
                    f"let d = ({s_expr}) as u128; let n = ((rdx as u128) << 64) | (rax as u128);"
                )
                lines.append("if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }")
            else:
                lines.append(
                    f"let d = ({s_expr}) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64);"
                )
                lines.append("if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; }")
            last_flag = None
        elif mn == "idiv":
            if len(ops) != 1:
                return None, None
            op = parse_op(ops[0])
            if not op:
                return None, None
            bits = op_bits(op)
            if bits not in (32, 64):
                return None, None
            s_expr = read_op(op, bits)
            if s_expr is None:
                return None, None
            if bits == 64:
                lines.append(
                    f"let d = ({s_expr}) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;"
                )
                lines.append(
                    "if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }"
                )
            else:
                lines.append(
                    f"let d = ({s_expr}) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64;"
                )
                lines.append(
                    "if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; }"
                )
            last_flag = None
        elif mn == "cbw":
            lines.append("rax = (rax & !0xffffu64) | ((rax as u8 as i8 as i16 as u16) as u64);")
        elif mn == "cwde":
            lines.append("rax = (rax as u16 as i16 as i32 as u32) as u64;")
        elif mn == "cdqe":
            lines.append("rax = (rax as u32 as i32 as i64) as u64;")
        elif mn == "cdq":
            lines.append("rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64;")
        elif mn == "cqo":
            lines.append("rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };")
        elif mn == "xchg":
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2:
                return None, None
            if op1 == op2:
                continue
            if op1[0] != "reg" or op2[0] != "reg" or op1[2] != op2[2] or op1[2] == 128:
                return None, None
            bits = op1[2]
            e1 = read_op(op1, bits)
            e2 = read_op(op2, bits)
            lines.append(f"let tmp_xchg = {e1};")
            lines.append(write_op(op1, e2, bits))
            lines.append(write_op(op2, "tmp_xchg", bits))
        elif mn in ("cmp", "test"):
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2:
                return None, None
            bits = op_bits(op1) or op_bits(op2)
            if not bits or bits == 128:
                return None, None
            e1 = read_op(op1, bits)
            if op2[0] == "imm":
                imm_val = op2[1] & ((1 << bits) - 1)
                e2 = f"0x{imm_val:x}u{bits}"
            else:
                e2 = read_op(op2, bits)
            if e1 is None or e2 is None:
                return None, None
            lines.append(f"let fl_l: u{bits} = {e1}; let fl_r: u{bits} = {e2};")
            last_flag = (mn, bits, "fl_l", "fl_r")
        elif mn.startswith("set"):
            cc = mn[3:]
            if len(ops) != 1:
                return None, None
            dst = parse_op(ops[0])
            if not dst or op_bits(dst) != 8:
                return None, None
            c_expr = cond_expr(cc, last_flag)
            if c_expr is None:
                return None, None
            w_stmt = write_op(dst, f"if {c_expr} {{ 1u8 }} else {{ 0u8 }}", 8)
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
        elif mn.startswith("cmov"):
            cc = mn[4:]
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg" or dst[2] == 128:
                return None, None
            dbits = dst[2]
            c_expr = cond_expr(cc, last_flag)
            d_expr = read_op(dst, dbits)
            s_expr = read_op(src, dbits)
            if c_expr is None or d_expr is None or s_expr is None:
                return None, None
            lines.append(
                write_op(dst, f"if {c_expr} {{ {s_expr} }} else {{ {d_expr} }}", dbits)
            )
        elif mn == "call":
            if len(ops) != 1:
                return None, None
            target = ops[0].strip()
            if target.startswith("RIP_P_"):
                p_idx = int(target.split("_")[2])
                fn_ptr = f"P{p_idx}"
            elif target.startswith("0x") or target.isdigit():
                return None, None
            else:
                op = parse_op(target)
                if not op:
                    return None, None
                fn_ptr = read_op(op, 64)
                if fn_ptr is None:
                    return None, None
            if uses_stack:
                lines.append(
                    "let s5 = crate::simd_ops::read_s5_call(rsp, &stack);"
                )
                lines.append(
                    "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);"
                )
                lines.append(
                    "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);"
                )
                lines.append(
                    "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);"
                )
                lines.append(
                    f"rax = crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);"
                )
            else:
                lines.append("crate::simd_ops::forward_incoming_to_outgoing();")
                lines.append(
                    f"rax = crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);"
                )
            last_flag = None
        elif mn == "jmp":
            if idx != len(insns) - 1 or len(ops) != 1:
                return None, None
            target = ops[0].strip()
            if target.startswith("RIP_P_"):
                p_idx = int(target.split("_")[2])
                fn_ptr = f"P{p_idx}"
            elif target.startswith("0x"):
                if addr_to_bb is None:
                    return None, None
                try:
                    t_va = int(target.split()[0], 16)
                except ValueError:
                    return None, None
                if t_va not in addr_to_bb:
                    return None, None
                lines.append(f"bb = {addr_to_bb[t_va]}; continue;")
                continue
            elif target.isdigit():
                return None, None
            else:
                op = parse_op(target)
                if not op:
                    return None, None
                fn_ptr = read_op(op, 64)
                if fn_ptr is None:
                    return None, None
            if uses_stack:
                lines.append(
                    "let s5 = crate::simd_ops::read_s5_tail(rsp, &stack);"
                )
                lines.append(
                    "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);"
                )
                lines.append(
                    "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);"
                )
                lines.append(
                    "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);"
                )
                lines.append(
                    f"return crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);"
                )
            else:
                lines.append("crate::simd_ops::forward_incoming_to_outgoing();")
                lines.append(
                    f"return crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);"
                )
        elif mn.startswith("j") and addr_to_bb is not None:
            cc = mn[1:]
            t_str = ops_str.strip().split()[0]
            try:
                t_va = int(t_str, 16)
            except ValueError:
                return None, None
            if t_va not in addr_to_bb:
                return None, None
            c_expr = cond_expr(cc, last_flag)
            if c_expr is None:
                return None, None
            lines.append(f"if {c_expr} {{ bb = {addr_to_bb[t_va]}; continue; }}")
        else:
            return None, None
    return lines, last_flag


def validate_xmm_usage(processed) -> bool:
    has_xmm = any(
        mn in ALLOWED_V_MNS or XMM_WORD_RE.search(ops)
        for _, mn, ops, _ in processed
    )
    if not has_xmm:
        return True

    defined = set()
    stored = {}
    for _, mn, ops, _ in processed:
        if mn == "vzeroupper":
            continue
        if mn in XMM_ZERO_MNS:
            parts = split_operands(ops)
            if len(parts) == 3 and parts[0] == parts[1] == parts[2]:
                r = parts[0]
            elif len(parts) == 2 and parts[0] == parts[1]:
                r = parts[0]
            else:
                return False
            if r not in XMM_PARENTS:
                return False
            defined.add(r)
            stored[r] = False
        elif mn in XMM_MOV_MNS:
            parts = split_operands(ops)
            if len(parts) != 2:
                return False
            d_str, s_str = parts[0], parts[1]
            if d_str in XMM_PARENTS and s_str in XMM_PARENTS:
                if s_str not in defined:
                    return False
                defined.add(d_str)
                stored[d_str] = False
            elif d_str in XMM_PARENTS:
                if XMM_WORD_RE.search(s_str):
                    return False
                defined.add(d_str)
                stored[d_str] = False
            elif s_str in XMM_PARENTS:
                if XMM_WORD_RE.search(d_str) or s_str not in defined:
                    return False
                stored[s_str] = True
            else:
                return False
        elif mn in ("call", "ret") or (
            mn == "jmp" and (ops.startswith("RIP_P_") or not ops.startswith("0x"))
        ):
            if XMM_WORD_RE.search(ops):
                return False
            if not all(stored.get(r, False) for r in defined):
                return False
            if mn == "call":
                defined.clear()
                stored.clear()
        elif mn.startswith("j"):
            if not all(stored.get(r, False) for r in defined):
                return False
        elif XMM_WORD_RE.search(ops):
            return False
    return True


def _check_frame_eff(eff: int, mn: str, ops: str) -> bool:
    if mn == "lea":
        return (-800 <= eff < 0) or (8 <= eff < 72)
    parts = split_operands(ops)
    if len(parts) == 2:
        op0 = parse_op(parts[0])
        op1 = parse_op(parts[1])
        bits = op_bits(op0) or op_bits(op1) or 64
    elif len(parts) == 1:
        op0 = parse_op(parts[0])
        bits = op_bits(op0) or 64
    else:
        return False
    sz_bytes = max(bits // 8, 1)
    return (-800 <= eff and eff + sz_bytes <= 0) or (8 <= eff and eff + sz_bytes <= 72)


def validate_stack_usage(processed):
    uses_stack = any(
        mn in ("push", "pop") or RSP_WORD_RE.search(ops)
        for _, mn, ops, _ in processed
    )
    if not uses_stack:
        return True, False

    addr_to_idx = {addr: i for i, (addr, _, _, _) in enumerate(processed)}
    state_at_idx = {}
    queue = deque([(0, 0, None)])

    while queue:
        idx, sp, bp = queue.popleft()
        if idx < 0 or idx >= len(processed):
            return False, False
        if idx in state_at_idx:
            if state_at_idx[idx] != (sp, bp):
                return False, False
            continue
        state_at_idx[idx] = (sp, bp)

        addr, mn, ops, next_addr = processed[idx]
        next_sp, next_bp = sp, bp
        ops_compact = ops.lower().replace(" ", "")

        if mn == "push":
            if RSP_WORD_RE.search(ops) or (bp is not None and RBP_WORD_RE.search(ops)):
                return False, False
            next_sp = sp - 8
            if next_sp < -800:
                return False, False
        elif mn == "pop":
            if RSP_WORD_RE.search(ops):
                return False, False
            if sp >= 0 or sp < -800:
                return False, False
            next_sp = sp + 8
            if ops_compact == "rbp":
                next_bp = None
            elif bp is not None and RBP_WORD_RE.search(ops):
                return False, False
        elif mn in ("sub", "add") and ops.lower().startswith("rsp,"):
            parts = split_operands(ops)
            if (
                len(parts) != 2
                or RSP_WORD_RE.search(parts[1])
                or (bp is not None and RBP_WORD_RE.search(parts[1]))
            ):
                return False, False
            try:
                imm = int(parts[1], 0)
            except ValueError:
                return False, False
            next_sp = (sp - imm) if mn == "sub" else (sp + imm)
            if next_sp > 0 or next_sp < -800:
                return False, False
        elif mn == "mov" and ops_compact == "rbp,rsp":
            next_bp = sp
        elif mn == "mov" and ops_compact == "rsp,rbp":
            if bp is None or bp > 0 or bp < -800:
                return False, False
            next_sp = bp
        elif mn == "lea" and ops.lower().startswith("rbp,") and RSP_BRACKET_RE.search(ops):
            m = RSP_BRACKET_RE.search(ops)
            outside = ops[: m.start()] + ops[m.end() :]
            if RSP_WORD_RE.search(outside):
                return False, False
            disp = (
                int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
                if m.group(1)
                else 0
            )
            next_bp = sp + disp
            if next_bp > 72 or next_bp < -800:
                return False, False
        elif mn == "lea" and ops.lower().startswith("rsp,") and RBP_BRACKET_RE.search(ops):
            if bp is None:
                return False, False
            m = RBP_BRACKET_RE.search(ops)
            outside = ops[: m.start()] + ops[m.end() :]
            if RBP_WORD_RE.search(outside) or RSP_WORD_RE.search(ops[m.start() : m.end()]):
                return False, False
            disp = (
                int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
                if m.group(1)
                else 0
            )
            next_sp = bp + disp
            if next_sp > 0 or next_sp < -800:
                return False, False
        elif mn == "mov" and ops.lower().startswith("rbp,") and RSP_BRACKET_RE.search(ops):
            next_bp = None
        elif mn == "ret" or (
            mn == "jmp" and (ops.startswith("RIP_P_") or not ops.startswith("0x"))
        ):
            if RSP_WORD_RE.search(ops) or sp != 0:
                return False, False
            if bp is not None and RBP_WORD_RE.search(ops):
                return False, False
            continue
        elif mn.startswith("j"):
            if RSP_WORD_RE.search(ops) or (bp is not None and RBP_WORD_RE.search(ops)):
                return False, False
            t_str = ops.strip().split()[0]
            try:
                t_va = int(t_str, 16)
            except ValueError:
                return False, False
            if t_va not in addr_to_idx:
                return False, False
            queue.append((addr_to_idx[t_va], sp, bp))
            if mn != "jmp":
                queue.append((idx + 1, sp, bp))
            continue

        queue.append((idx + 1, next_sp, next_bp))

    if len(state_at_idx) != len(processed):
        return False, False

    for idx, (addr, mn, ops, next_addr) in enumerate(processed):
        sp, bp = state_at_idx[idx]
        ops_compact = ops.lower().replace(" ", "")
        if mn in ("push", "pop") or (
            mn in ("sub", "add") and ops.lower().startswith("rsp,")
        ):
            continue
        if mn == "mov" and ops_compact in ("rbp,rsp", "rsp,rbp"):
            continue
        if mn == "lea" and (
            (ops.lower().startswith("rbp,") and RSP_BRACKET_RE.search(ops))
            or (ops.lower().startswith("rsp,") and RBP_BRACKET_RE.search(ops))
        ):
            continue

        if RSP_WORD_RE.search(ops):
            m = RSP_BRACKET_RE.search(ops)
            if not m:
                return False, False
            outside = ops[: m.start()] + ops[m.end() :]
            if RSP_WORD_RE.search(outside):
                return False, False
            disp = (
                int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
                if m.group(1)
                else 0
            )
            if not _check_frame_eff(sp + disp, mn, ops):
                return False, False

        if bp is not None and RBP_WORD_RE.search(ops):
            if mn == "mov" and ops.lower().startswith("rbp,") and RSP_BRACKET_RE.search(ops):
                continue
            m = RBP_BRACKET_RE.search(ops)
            if not m:
                return False, False
            outside = ops[: m.start()] + ops[m.end() :]
            if RBP_WORD_RE.search(outside):
                return False, False
            disp = (
                int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
                if m.group(1)
                else 0
            )
            if not _check_frame_eff(bp + disp, mn, ops):
                return False, False

    return True, True


def is_internal_branch(mn: str, ops: str) -> bool:
    if mn.startswith("j") and mn != "jmp":
        return True
    if mn == "jmp" and ops.strip().startswith("0x"):
        return True
    return False


def translate_block(insns, initial_flag=None, depth: int = 0, uses_stack: bool = False):
    if depth > 6 or not insns:
        return None

    branches = [
        (i, item)
        for i, item in enumerate(insns)
        if is_internal_branch(item[1], item[2])
    ]
    if not branches:
        lines, _ = translate_linear_insns(insns, initial_flag, None, uses_stack)
        return lines

    j_idx, (j_addr, j_mn, j_ops, _) = branches[0]
    if j_mn == "jmp":
        return None

    cc = j_mn[1:]
    t_str = j_ops.strip().split()[0]
    try:
        t_va = int(t_str, 16)
    except ValueError:
        return None

    target_indices = [i for i, item in enumerate(insns) if item[0] == t_va]
    if not target_indices:
        return None
    t_idx = target_indices[0]

    # Backward branch: simple `do { ... } while (cond);` loop when `t_idx <= j_idx`
    if t_idx <= j_idx:
        before_insns = insns[:t_idx]
        loop_insns = insns[t_idx:j_idx]
        after_insns = insns[j_idx + 1 :]
        if not after_insns:
            return None
        loop_addrs = {x[0] for x in loop_insns}
        if any(
            is_internal_branch(m, o) and int(o.strip().split()[0], 16) in loop_addrs
            for _, m, o, _ in after_insns
        ):
            return None
        before_lines, _ = translate_linear_insns(
            before_insns, initial_flag, None, uses_stack
        )
        if before_lines is None:
            return None
        loop_lines, flag_loop = translate_linear_insns(loop_insns, None, None, uses_stack)
        if loop_lines is None or flag_loop is None:
            return None
        c_expr = cond_expr(cc, flag_loop)
        if c_expr is None:
            return None
        after_lines = translate_block(after_insns, flag_loop, depth + 1, uses_stack)
        if (
            after_lines is None
            or not after_lines
            or not after_lines[-1].startswith("return ")
        ):
            return None
        combined = list(before_lines)
        combined.append("loop {")
        for l in loop_lines:
            combined.append("    " + l)
        combined.append(f"    if !({c_expr}) {{ break; }}")
        combined.append("}")
        combined.extend(after_lines)
        return combined

    before_insns = insns[:j_idx]
    before_lines, flag_after_before = translate_linear_insns(
        before_insns, initial_flag, None, uses_stack
    )
    if before_lines is None or flag_after_before is None:
        return None
    c_expr = cond_expr(cc, flag_after_before)
    if c_expr is None:
        return None

    # Ensure no branch after j_idx jumps backwards to or before j_idx
    min_after_addr = insns[j_idx + 1][0] if j_idx + 1 < len(insns) else t_va
    for i in range(j_idx + 1, len(insns)):
        _, mn2, ops2, _ = insns[i]
        if is_internal_branch(mn2, ops2):
            try:
                bt = int(ops2.strip().split()[0], 16)
                if bt < min_after_addr:
                    return None
            except ValueError:
                return None

    # Pattern A: if / else diamond (insns[t_idx - 1] is `jmp join_va` with join_idx > t_idx)
    if (
        t_idx >= j_idx + 2
        and insns[t_idx - 1][1] == "jmp"
        and insns[t_idx - 1][2].strip().startswith("0x")
    ):
        try:
            join_va = int(insns[t_idx - 1][2].strip().split()[0], 16)
        except ValueError:
            join_va = -1
        join_indices = [i for i, item in enumerate(insns) if item[0] == join_va]
        if join_indices and join_indices[0] > t_idx:
            join_idx = join_indices[0]
            then_insns = insns[j_idx + 1 : t_idx - 1]
            else_insns = insns[t_idx:join_idx]
            after_insns = insns[join_idx:]

            then_addrs = {x[0] for x in then_insns}
            else_addrs = {x[0] for x in else_insns}
            cross_bad = False
            for _, mn2, ops2, _ in then_insns:
                if is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) not in then_addrs:
                        cross_bad = True
            for _, mn2, ops2, _ in else_insns:
                if is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) not in else_addrs:
                        cross_bad = True
            for _, mn2, ops2, _ in after_insns:
                if is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) in (then_addrs | else_addrs):
                        cross_bad = True
            if not cross_bad:
                then_lines = translate_block(
                    then_insns, flag_after_before, depth + 1, uses_stack
                )
                else_lines = translate_block(
                    else_insns, flag_after_before, depth + 1, uses_stack
                )
                after_lines = translate_block(after_insns, None, depth + 1, uses_stack)
                if (
                    then_lines is not None
                    and else_lines is not None
                    and after_lines is not None
                    and after_lines
                    and after_lines[-1].startswith("return ")
                ):
                    combined = list(before_lines)
                    combined.append(f"if !({c_expr}) {{")
                    for l in then_lines:
                        combined.append("    " + l)
                    combined.append("} else {")
                    for l in else_lines:
                        combined.append("    " + l)
                    combined.append("}")
                    combined.extend(after_lines)
                    return combined

    # Pattern B: Isolated `if !(c_expr) { fall_lines } after_lines`
    fall_insns = insns[j_idx + 1 : t_idx]
    after_insns = insns[t_idx:]
    fall_addrs = {x[0] for x in fall_insns}
    after_addrs = {x[0] for x in after_insns}
    isolated = True
    for _, mn2, ops2, _ in fall_insns:
        if is_internal_branch(mn2, ops2):
            if int(ops2.strip().split()[0], 16) not in fall_addrs:
                isolated = False
    for _, mn2, ops2, _ in after_insns:
        if is_internal_branch(mn2, ops2):
            if int(ops2.strip().split()[0], 16) not in after_addrs:
                isolated = False
    if isolated:
        fall_lines = translate_block(fall_insns, flag_after_before, depth + 1, uses_stack)
        after_flag = (
            flag_after_before
            if not any(m in FLAG_CLOBBER_MNS for _, m, _, _ in fall_insns)
            else None
        )
        after_lines = translate_block(after_insns, after_flag, depth + 1, uses_stack)
        if (
            fall_lines is not None
            and after_lines is not None
            and after_lines
            and after_lines[-1].startswith("return ")
        ):
            combined = list(before_lines)
            combined.append(f"if !({c_expr}) {{")
            for l in fall_lines:
                combined.append("    " + l)
            combined.append("}")
            combined.extend(after_lines)
            return combined

    # Pattern C: Guard branch `jcc t_va` to a branchless shared return/epilogue tail at `insns[t_idx:]`
    if not any(is_internal_branch(m, o) for _, m, o, _ in after_insns):
        strict_tail_addrs = {x[0] for x in after_insns[1:]}
        if not any(
            is_internal_branch(m, o)
            and int(o.strip().split()[0], 16) in strict_tail_addrs
            for _, m, o, _ in insns[j_idx + 1 : t_idx]
        ):
            tail_lines, _ = translate_linear_insns(
                after_insns, flag_after_before, None, uses_stack
            )
            if (
                tail_lines is not None
                and tail_lines
                and tail_lines[-1].startswith("return ")
            ):
                rest_lines = translate_block(
                    insns[j_idx + 1 :], flag_after_before, depth + 1, uses_stack
                )
                if (
                    rest_lines is not None
                    and rest_lines
                    and rest_lines[-1].startswith("return ")
                ):
                    combined = list(before_lines)
                    combined.append(f"if {c_expr} {{")
                    for l in tail_lines:
                        combined.append("    " + l)
                    combined.append("}")
                    combined.extend(rest_lines)
                    return combined

    return None


def translate_cfg(processed, uses_stack: bool = False):
    if not processed:
        return None
    all_addrs = {addr for addr, _, _, _ in processed}
    block_starts = {processed[0][0]}
    for addr, mn, ops, next_addr in processed:
        if is_internal_branch(mn, ops):
            try:
                t_va = int(ops.strip().split()[0], 16)
            except ValueError:
                return None
            if t_va not in all_addrs:
                return None
            block_starts.add(t_va)
            if mn == "jmp" and next_addr in all_addrs:
                block_starts.add(next_addr)
        elif mn in ("ret", "jmp"):
            if next_addr in all_addrs:
                block_starts.add(next_addr)

    sorted_starts = sorted(block_starts)
    if len(sorted_starts) > 64:
        return None
    addr_to_bb = {va: i for i, va in enumerate(sorted_starts)}

    blocks = []
    cur = []
    for item in processed:
        if item[0] in addr_to_bb and cur:
            blocks.append(cur)
            cur = [item]
        else:
            cur.append(item)
    if cur:
        blocks.append(cur)

    lines = ["let mut bb: u32 = 0;", "loop {", "    match bb {"]
    for i, blk in enumerate(blocks):
        b_lines, _ = translate_linear_insns(blk, None, addr_to_bb, uses_stack)
        if b_lines is None:
            return None
        last_mn = blk[-1][1]
        if last_mn not in ("ret", "jmp"):
            if i + 1 >= len(blocks):
                return None
            b_lines.append(f"bb = {i + 1}; continue;")
        lines.append(f"        {i} => {{")
        for bl in b_lines:
            lines.append(f"            {bl}")
        lines.append("        }")
    lines.append("        _ => return rax,")
    lines.append("    }")
    lines.append("}")
    return lines


def translate_function(raw_insns, fn_va: int, fn_size: int, int_safe_targets: set):
    params = []
    insns = []
    for addr, mn, ops in raw_insns:
        if mn == "nop":
            insns.append((addr, mn, "", addr))
        else:
            insns.append((addr, mn, ops, 0))

    processed = []
    for i, (addr, mn, ops, _) in enumerate(insns):
        next_addr = insns[i + 1][0] if i + 1 < len(insns) else (fn_va + fn_size)
        if "rip" in ops:
            m = RIP_RE.search(ops)
            if not m:
                return None
            sign = 1 if m.group(1) == "+" else -1
            disp = int(m.group(2), 0) * sign
            target_va = (next_addr + disp) & 0xFFFFFFFFFFFFFFFF
            p_idx = len(params)
            params.append(target_va)
            ops = RIP_RE.sub(f"RIP_P_{p_idx}", ops)
        elif mn == "jmp":
            t_str = ops.strip().split()[0]
            try:
                t_va = int(t_str, 16)
                if not (fn_va <= t_va < fn_va + fn_size):
                    if t_va not in int_safe_targets:
                        return None
                    p_idx = len(params)
                    params.append(t_va)
                    ops = f"RIP_P_{p_idx}"
            except ValueError:
                pass
        elif mn == "call":
            t_str = ops.strip().split()[0]
            try:
                t_va = int(t_str, 16)
                if t_va not in int_safe_targets:
                    return None
                p_idx = len(params)
                params.append(t_va)
                ops = f"RIP_P_{p_idx}"
            except ValueError:
                pass
        processed.append((addr, mn, ops, next_addr))

    if not validate_xmm_usage(processed):
        return None
    stack_ok, uses_stack = validate_stack_usage(processed)
    if not stack_ok:
        return None

    lines = translate_block(processed, None, 0, uses_stack)
    if lines is None or not lines or not lines[-1].startswith("return "):
        lines = translate_cfg(processed, uses_stack)
        if lines is None:
            return None
    return (lines, params)


def collect_existing_vas() -> set[int]:
    existing = set()
    reg_dir = ROOT / "crates" / "aw2-game" / "src" / "registry"
    shards_dir = ROOT / "crates" / "aw2-game" / "src" / "shards"
    files = list(sorted(reg_dir.rglob("*.rs")))
    for p in sorted(shards_dir.rglob("*.rs")):
        rel_parts = p.relative_to(shards_dir).parts
        if rel_parts[0] not in ("lifted_leaves", "lifted_leaves.rs", "ecs_more.rs", "r_more", "r_more.rs", "subsystems", "subsystems.rs", "mod.rs"):
            files.append(p)
    ctor_re = re.compile(
        r"Replacement::(?:draft(?:_diff|_throws|_throws_diff)?|verified(?:_throws)?)\s*\(\s*(0x[0-9a-fA-F_]+)"
    )
    va_re = re.compile(r"\bva\s*:\s*(0x[0-9a-fA-F_]+)")
    for path in files:
        if not path.exists():
            continue
        for line in path.read_text().splitlines():
            s = line.lstrip()
            if s.startswith("//"):
                continue
            m = ctor_re.search(s) or va_re.search(s)
            if m:
                existing.add(int(m.group(1).replace("_", ""), 16))
    return existing


def read_text_va(pe, va: int, size: int):
    rva = va - pe.image_base
    off = rva - pe.text_rva
    if off < 0 or off + max(size, 5) > len(pe.text):
        return None
    if size < 5 and pe.text[off + size : off + 5] != b"\xcc" * (5 - size):
        return None
    return pe.text[off : off + size]


def main():
    exe_path = os.environ.get("AW2_EXE") or (
        os.path.join(os.environ["AW2_GAME_DIR"], "AlanWake2.exe")
        if os.environ.get("AW2_GAME_DIR")
        else ""
    )
    if not exe_path or not os.path.isfile(exe_path):
        raise SystemExit("set AW2_EXE or AW2_GAME_DIR to your local Alan Wake 2 install")
    pe = fndb.PE(exe_path)
    db = sqlite3.connect(str(ROOT / "db" / "functions.sqlite"))

    can_throw_map = dict(db.execute("SELECT va, mask_direct FROM can_throw").fetchall())
    existing_vas = collect_existing_vas()

    all_fns = db.execute("SELECT va, size FROM functions").fetchall()
    bad_callees = {0x1418351D0}  # __security_check_cookie
    int_safe_targets = set()
    for va, sz in all_fns:
        if va in bad_callees:
            continue
        b = read_text_va(pe, va, sz)
        if b and not XMM_ONLY_RE.search(b):
            int_safe_targets.add(va)

    rows = db.execute(
        """
        SELECT va, size, n_callees, name
        FROM functions
        WHERE owner = 'remedy'
          AND n_frags = 0
          AND size <= 1024
          AND n_callees <= 16
        ORDER BY va
        """
    ).fetchall()

    blob = bytearray()
    offset_map = []
    base_vaddr = 0x140000000

    for va, size, n_callees, name in rows:
        if va in existing_vas:
            continue
        eh = fast_eh_info(pe, va)
        mask_direct = can_throw_map.get(va, 0)
        if not (
            eh in ("none", "noexcept")
            or (eh == "cleanup" and (mask_direct & 21) == 0)
        ):
            continue
        b = read_text_va(pe, va, size)
        if not b:
            continue
        if has_gs(b):
            continue
        start_off = len(blob)
        blob.extend(b)
        while len(blob) % 16 != 0:
            blob.append(0xCC)
        offset_map.append((start_off, len(b), va, n_callees, name))

    elf_bytes = build_elf64(bytes(blob), base_vaddr=base_vaddr)
    tmp_dir = ROOT / "target"
    tmp_dir.mkdir(parents=True, exist_ok=True)
    elf_path = tmp_dir / ".lifted_leaves_batch.elf"
    elf_path.write_bytes(elf_bytes)

    try:
        out = subprocess.check_output(
            [
                "objdump",
                "--disassemble",
                "--x86-asm-syntax=intel",
                "--no-show-raw-insn",
                str(elf_path),
            ],
            text=True,
        )
    finally:
        if elf_path.exists():
            elf_path.unlink()

    insn_by_off = {}
    line_re = re.compile(r"^\s*([0-9a-fA-F]+):\s+([a-z0-9]+)\s*(.*)$")
    for line in out.splitlines():
        m = line_re.match(line)
        if m:
            vaddr = int(m.group(1), 16)
            off = vaddr - base_vaddr
            mn = m.group(2)
            ops = m.group(3).split("#")[0].strip()
            insn_by_off[off] = (mn, ops)

    bad_regs_re = re.compile(
        r"\b(xmm(?:[6-9]|1\d)|ymm\d+|zmm\d+|mm\d+|gs|fs|cs|ds|es|ss)\b"
    )
    bad_mnems = {
        "int3",
        "ud2",
        "lock",
        "rep",
        "repz",
        "repnz",
        "repe",
        "repne",
        "leave",
        "enter",
    }

    templates = {}  # (tuple(lines), num_params) -> tpl_name
    lifted_entries = []  # (va, name_str, tpl_name, params, throws)

    for start_off, sz, va, n_callees, fn_name in offset_map:
        insns = []
        for off in range(start_off, start_off + sz):
            if off in insn_by_off:
                mn, ops = insn_by_off[off]
                real_va = va + (off - start_off)
                if mn == "nop":
                    ops = ""
                elif (mn.startswith("j") or mn == "call") and ops:
                    first_tok = ops.strip().split()[0]
                    if first_tok.startswith("0x"):
                        try:
                            elf_t_va = int(first_tok, 16)
                            real_t_va = (
                                va + (elf_t_va - (base_vaddr + start_off))
                            ) & 0xFFFFFFFFFFFFFFFF
                            ops = f"0x{real_t_va:x}"
                        except ValueError:
                            pass
                insns.append((real_va, mn, ops))

        if not insns:
            continue

        # Trim trailing int3/nop padding after terminal ret/jmp first
        trimmed = []
        for addr, mn, ops in insns:
            trimmed.append((addr, mn, ops))
            if mn in ("ret", "jmp"):
                later_addrs = {a for a, _, _ in insns if a > addr}
                has_jump_to_later = False
                for _, m2, o2 in trimmed:
                    if m2.startswith("j"):
                        try:
                            if int(o2.strip().split()[0], 16) in later_addrs:
                                has_jump_to_later = True
                                break
                        except ValueError:
                            pass
                if not has_jump_to_later:
                    break

        if len(trimmed) > 200:
            continue

        # Must be safe for 5-byte hook patch at entry: no interior branch target in (va, va + 5)
        interior_in_first_5 = False
        bad = False
        for addr, mn, ops in trimmed:
            if (
                mn in bad_mnems
                or (mn.startswith("v") and mn not in ALLOWED_V_MNS)
                or bad_regs_re.search(ops)
            ):
                bad = True
                break
            if mn.startswith("j"):
                t_str = ops.strip().split()[0]
                try:
                    t_va = int(t_str, 16)
                    if va < t_va < va + 5:
                        interior_in_first_5 = True
                        break
                    if mn != "jmp" and not (va <= t_va < va + sz):
                        bad = True
                        break
                except ValueError:
                    pass
        if bad or interior_in_first_5:
            continue

        res = translate_function(trimmed, va, sz, int_safe_targets)
        if res is None:
            continue

        lines, params = res
        if any("(0x0u64) as *" in l for l in lines):
            continue
        key = (tuple(lines), len(params))
        if key not in templates:
            tpl_id = len(templates)
            templates[key] = f"tpl_leaf_{tpl_id:04d}"
        tpl_name = templates[key]

        mask_direct = can_throw_map.get(va, 0)
        throws = (mask_direct & 23) != 0
        sym_name = f"lifted_leaf_{va:x}"
        lifted_entries.append((va, sym_name, tpl_name, params, throws))

    # Re-read existing_vas right before writing output to guarantee 0 collisions with concurrent shards
    existing_vas = collect_existing_vas()
    lifted_entries = [e for e in lifted_entries if e[0] not in existing_vas]

    from collections import Counter as _Counter
    tpl_by_name = {v: (lines_tup, np) for (lines_tup, np), v in templates.items()}
    tpl_counts = _Counter(e[2] for e in lifted_entries)
    kept_tpls = {
        tname
        for tname, (lines_tup, np) in tpl_by_name.items()
        if len(lines_tup) <= 16 and "match bb {" not in "\n".join(lines_tup) and tpl_counts[tname] >= 3
    }
    lifted_entries = [e for e in lifted_entries if e[2] in kept_tpls]

    out_lines = [
        "// Auto-generated by tools/decomp_scale/gen_lifted_leaves.py — DO NOT EDIT BY HAND.",
        "#![allow(unused_variables, unused_mut, unused_assignments, unused_parens, dead_code, unreachable_code, clippy::all)]",
        "",
        "use crate::registry::{Replacement, Throws};",
        "",
    ]

    used_tpls = {e[2] for e in lifted_entries}
    for (body_lines, n_params), tpl_name in templates.items():
        if tpl_name not in used_tpls:
            continue
        if n_params == 0:
            generics = ""
        else:
            generics = "<" + ", ".join(f"const P{i}: u64" for i in range(n_params)) + ">"
        body_text = "\n".join(body_lines)
        has_rsp = re.search(r"\brsp\b", body_text) is not None
        has_arg5 = re.search(r"\barg5\b", body_text) is not None
        out_lines.append("#[inline(never)]")
        if has_rsp or has_arg5:
            out_lines.append(
                f'pub unsafe extern "win64" fn {tpl_name}{generics}(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {{'
            )
        else:
            out_lines.append(
                f'pub unsafe extern "win64" fn {tpl_name}{generics}(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {{'
            )
        out_lines.append("    let mut rax: u64 = 0;")
        out_lines.append("    let mut r10: u64 = 0;")
        out_lines.append("    let mut r11: u64 = 0;")
        for cs_reg in CALLEE_SAVED_PARENTS:
            if re.search(rf"\b{cs_reg}\b", body_text):
                out_lines.append(f"    let mut {cs_reg}: u64 = 0;")
        for xm_reg in XMM_PARENTS:
            if re.search(rf"\b{xm_reg}\b", body_text):
                out_lines.append(f"    let mut {xm_reg}: u128 = 0;")
        if has_rsp:
            out_lines.append("    let mut stack = crate::simd_ops::LiftedStack::<128>::new();")
            out_lines.append("    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);")
        for bl in body_lines:
            out_lines.append(f"    {bl}")
        out_lines.append("}")
        out_lines.append("")

    out_lines.append("pub static ENTRIES: &[Replacement] = &[")
    for va, sym_name, tpl_name, params, throws in lifted_entries:
        if params:
            turbo = "::<" + ", ".join(f"0x{p:x}" for p in params) + ">"
        else:
            turbo = ""
        fn_expr = f"{tpl_name}{turbo} as *const ()"
        if throws:
            out_lines.append(
                f'    Replacement::draft_throws(0x{va:x}, "{sym_name}", {fn_expr}, Throws::Discard),'
            )
        else:
            out_lines.append(
                f'    Replacement::draft(0x{va:x}, "{sym_name}", {fn_expr}),'
            )
    out_lines.append("];")
    out_lines.append("")

    out_path = ROOT / "crates" / "aw2-game" / "src" / "shards" / "lifted_leaves.rs"
    out_path.write_text("\n".join(out_lines))
    print(
        f"Wrote {len(lifted_entries)} replacements ({len(used_tpls)} unique templates) to {out_path}"
    )


if __name__ == "__main__":
    main()
