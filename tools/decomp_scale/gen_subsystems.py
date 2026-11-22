#!/usr/bin/env python3
"""
Generate crates/aw2-game/src/shards/{ecs_more,r_more,subsystems}.rs by lifting
remaining functions from AlanWake2.exe into deduplicated const-generic Rust
functions partitioned by subsystem (ECS/Component, r:: core library, and general
engine subsystems).
"""

from collections import Counter, deque
from pathlib import Path
import os
import re
import sqlite3
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "fndb"))
sys.path.insert(0, str(ROOT / "tools" / "decomp_scale"))
import fndb  # noqa: E402
import gen_lifted_leaves as g  # noqa: E402


# ---------------------------------------------------------------------------------------------
# x86 `lock` prefix.
#
# llvm-objdump prints an F0 prefix that comes first on its own `lock` line, with the locked
# instruction on the next line at lock_va + 1; when F0 follows another prefix (e.g. `66 F0`) it
# prints `lock add word ptr [..], 0x1` on one line. main() records the VA of every locked
# instruction (both forms) in LOCKED_INSN_VAS, and translate_linear_insns() additionally treats
# the instruction following a bare `lock` pseudo-instruction as locked, so a lock-prefixed
# memory RMW is never lowered to a plain ruN/wuN load+store pair (lost updates -> deadlocks).
# `xchg` with a memory operand is implicitly locked; xchg/xadd/cmpxchg with a memory operand are
# always lowered to crate::simd_ops::atomic_* regardless of the prefix.
#
# LOCK_RMW_MODE (env AW2_LOCK_RMW_MODE) selects how memory-destination RMWs are lowered:
#   "all"    (default) every memory add/sub/and/or/xor/inc/dec goes through
#            crate::simd_ops::lk_* (an atomic RMW is also a valid implementation of a plain x86
#            RMW). This is byte-identical to the shards post-processed by
#            target/lockscan/lock_rmw_fix.py. Forms that script did not rewrite (flag-observed
#            `sub`, `not`) are atomic only when lock-prefixed.
#   "locked" only lock-prefixed RMWs are atomic; unprefixed RMWs stay plain loads/stores.
#   "legacy-plain"  VALIDATION ONLY: reproduces the pre-fix output (prefix dropped, every RMW a
#            plain load+store) so A/B diffs can be made; main() refuses to write the real shards
#            in this mode.
# A lock-prefixed instruction with no atomic lowering (neg/bts/btr/btc/adc/sbb/... on memory)
# makes translate_linear_insns() reject the function, which then stays on the aot_va path.
# ---------------------------------------------------------------------------------------------
LOCKED_INSN_VAS: set[int] = set()
LOCK_RMW_MODE = os.environ.get("AW2_LOCK_RMW_MODE", "all")
if LOCK_RMW_MODE not in ("all", "locked", "legacy-plain"):
    raise SystemExit(f"AW2_LOCK_RMW_MODE must be all|locked|legacy-plain, got {LOCK_RMW_MODE!r}")
# Lock-prefixed mnemonics with an atomic lowering (memory destination required).
LOCK_LOWERED_MNS = frozenset(
    ("add", "sub", "and", "or", "xor", "inc", "dec", "not", "xadd", "cmpxchg", "xchg")
)

# ---------------------------------------------------------------------------------------------
# Target image. Defaults are AlanWake2.exe; `--image <dll> --base <pinned>` (dll_lift.py)
# retargets them to a DLL relocated to the base the loader pins it at.
#   IMAGE_BASE          base of RVA-relative jump tables (`[reg + idx*4 + rva]`).
#   GUARD_DISPATCH_VAS  CFG dispatch functions (`jmp rax`): direct call/jmp -> `rax`.
#   NOP_CALL_VAS        __security_check_cookie / __chkstk variants: direct call -> nop.
#   GUARD_FPTR_LIT      value loaded from __guard_dispatch_icall_fptr (memory-indirect call or
#                       jmp through it -> `rax`), as a Rust literal; None = no such slot.
#   READ_TABLE          (pe, va, size) -> bytes | None reader for jump tables; None = .text only
#                       (`read_frag_va`; the exe's tables follow their functions in .text).
#   EXTRA_BAD_MNEMS     mnemonics that reject a function (DLL mode: `int`, which the exe lift
#                       turns into a plain return; a DLL's `int 0x29` fast-fail must stay guest).
#   EXACT_FLAGS         x86-exact corner cases the old lift approximated:
#                       - `sbb r, r` keeps CF (CF out = CF in) and sets ZF/SF from the result,
#                         instead of "no flags" (which a following `sbb`/`adc`/jcc reads as
#                         false: MSVC memcmp's `sbb eax,eax; sbb eax,-1` returned 0 for "less");
#                       - `bsf`/`bsr` with a zero source leave the destination unchanged (as
#                         the emulator and hardware do) instead of writing 64/-1 (memchr's
#                         not-found path returns that register);
#                       - `add` whose flags reach a CF/OF/signed consumer in the same block
#                         (`add; adc`, `add rax, 8; cmovb`) records ("add", res, src) so every
#                         condition is exact, instead of ZF/SF only.
#   STRICT_FLAGS        a flag read the lowering cannot express exactly (no recorded producer,
#                       a condition the producer's kind does not support, a `multi` join with
#                       such an arm, or a join with a predecessor whose flags are unknown)
#                       rejects the translation (structured -> CFG -> function stays on the
#                       aot_va path) instead of reading `false`. Dataflow pre-passes are exempt.
#   AW2_GEN_LEGACY_FLAGS=1 turns both off (pre-2026-10-25 output, for relift_identity.sh).
# ---------------------------------------------------------------------------------------------
IMAGE_BASE = 0x140000000
GUARD_DISPATCH_VAS = frozenset((0x142547700, 0x142547720, 0x141836100))
NOP_CALL_VAS = frozenset((0x141835480, 0x1418351D0, 0x1418353D0, 0x141835430))
# CALL_REDIRECT: {va: va'} direct call/jmp targets that are a lone `jmp rel32` to va' (DLL
# mode: incremental-link thunks); the call goes to va' (same semantics, no thunk hop).
CALL_REDIRECT = {}
# LEAF_STARTS: pdata-less leaf function starts (DLL mode, dll_lift.load): accepted, like
# `fn_starts`, as conditional tail jmp targets (frontier_plan.md §1).
LEAF_STARTS = frozenset()
# jcc targets dll_lift's leaf rule refused: VA -> "leaf_unaligned" / "leaf_no_pad".
LEAF_REJECTS = {}
# Known noreturn import slots and `.text` `FF 25` stubs (DLL mode, `dll_lift.load`), so FH4
# funclets / helpers whose `.pdata` extent ends right after `call _CxxThrowException` / `abort` /
# `terminate` without trailing `0xCC` padding are recognized as complete extents.
NORETURN_CALL_VAS = frozenset()
_RIP_MEM_RE = re.compile(r"\[rip\s*([+-])\s*(0x[0-9a-fA-F]+|\d+)\]")


def is_noreturn_call_insn(insn) -> bool:
    if not NORETURN_CALL_VAS or insn[1] != "call":
        return False
    ops = insn[2].strip()
    if ops.startswith("0x"):
        try:
            tgt = int(ops.split()[0], 16)
        except ValueError:
            return False
        return tgt in NORETURN_CALL_VAS or CALL_REDIRECT.get(tgt) in NORETURN_CALL_VAS
    m = _RIP_MEM_RE.search(ops)
    if m:
        disp = int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
        return ((insn[3] + disp) & 0xFFFF_FFFF_FFFF_FFFF) in NORETURN_CALL_VAS
    return False


# DLL C++ EH (dll_lift.py, eh_lift_plan.md T1/T2). EH_SITES: DLL mode, the exe's
# `call_va8_caught`/`rethrow` forms are not used. EH_CUR: facts of the function being
# translated (dll_lift `eh` entry) or None: every call in it is preceded by
# `eh.site(<call VA>)` on its `lift::eh::EhFrame` record (pushed at entry), and calls inside
# its try ranges (`try_ranges`, None = all) use `eh.call` whose `Err(Landing)` arm dispatches
# to the catch continuation block (`conts`, extra block starts). Tail jumps unlink the record
# first (`eh_leave`) and stay plain.
EH_SITES = False
EH_CUR = None
# Exe mode (L5, eh_lift_plan.md T4): eh_mode-2 functions (own FH4 FuncInfo, a throwing callee)
# get `dll_lift.exe_eh_facts` as EH_CUR and the EH_SITES lowering, with their `EhMeta` as a
# static in the body (`exe_eh_meta_line`). AW2_GEN_EXE_EH=0: the pre-L5 output
# (`call_va8_caught` + `rethrow`), for drift checks against shards lifted before L5.
EXE_EH = os.environ.get("AW2_GEN_EXE_EH", "1") != "0"
EXE_PATH = Path(
    os.environ.get("AW2_EXE")
    or (
        os.path.join(os.environ["AW2_GAME_DIR"], "AlanWake2.exe")
        if os.environ.get("AW2_GAME_DIR")
        else "AlanWake2.exe"
    )
)


def exe_eh_meta_line(va: int, x: dict) -> str:
    """`static EH_M: EhMeta = ...;` for an exe EH_CUR fn (fields as dll_lift `emit`)."""
    conts = ", ".join(f"0x{c:x}" for c in x["conts"])
    return (
        "static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { "
        f"va: 0x{va:x}, funcinfo_rva: 0x{x['funcinfo_rva']:x}, handler: {x['handler']}, shape: {x['shape']}, "
        f"frame_reg: {x['frame_reg']}, establisher_off: 0x{x['establisher_off']:x}, conts: &[{conts}] }};"
    )
GUARD_FPTR_LIT = "0x1_4254_7700"
READ_TABLE = None
EXTRA_BAD_MNEMS = frozenset()
_LEGACY_FLAGS = os.environ.get("AW2_GEN_LEGACY_FLAGS") == "1"
EXACT_FLAGS = not _LEGACY_FLAGS
STRICT_FLAGS = not _LEGACY_FLAGS
# Flags after an instruction whose effect is not modelled (call, adc/sbb result, imul, ...).
# Distinct from None (= no information yet / bottom in the dataflow join).
UNK_FLAG = ("unk", 0, "", "")

# `AW2_GEN_AUDIT=<file>`: record, per translated function, every site where the lowering
# approximates x86 (unknown flags read as `false`, bsf/bsr zero-source write) as JSON lines
# {fn, addr, mn, kind, detail}; output is unchanged. The last translation of a function wins.
AUDIT = {} if os.environ.get("AW2_GEN_AUDIT") else None
AUDIT_CTX = [0, 0, ""]  # fn VA, insn addr, mnemonic
# True while translate_blocks' flag dataflow pre-passes run (blocks are translated with
# unknown in-flags there by design; only the final emission counts).
IN_DATAFLOW = False


def audit(kind: str, detail: str = ""):
    if AUDIT is not None and not IN_DATAFLOW:
        fn, addr, mn = AUDIT_CTX
        AUDIT.setdefault(fn, set()).add((addr, mn, kind, detail))


def _audit_dump():
    import json

    with open(os.environ["AW2_GEN_AUDIT"], "w") as f:
        for fn in sorted(AUDIT):
            for addr, mn, kind, detail in sorted(AUDIT[fn]):
                f.write(json.dumps({"fn": fn, "addr": addr, "mn": mn, "kind": kind, "detail": detail}) + "\n")


if AUDIT is not None:
    import atexit

    atexit.register(_audit_dump)


def rmw_atomic(dst, locked: bool, plain_too: bool = True) -> bool:
    """Lower this memory-destination RMW through crate::simd_ops::lk_*?

    plain_too: in "all" mode, also for unprefixed instances (False for the forms
    lock_rmw_fix.py never rewrote, so "all" output stays identical to the fixed shards)."""
    if dst[0] != "mem" or LOCK_RMW_MODE == "legacy-plain":
        return False
    return locked or (plain_too and LOCK_RMW_MODE == "all")


def lk_rmw_call(op: str, dst, bits: int, x_expr: str) -> str:
    """crate::simd_ops::lk_<op>_u<bits>(addr, x) -> new value (atomic when naturally aligned)."""
    return f"crate::simd_ops::lk_{op}_u{bits}({dst[2]}, ({x_expr}) as u{bits})"


def lk_fl_l_stmt(call: str, bits: int) -> str:
    """`fl_l = <new value>` in the exact form lock_rmw_fix.py produced."""
    return f"fl_l = ({call}) as u{bits} as u64;" if bits != 64 else f"fl_l = ({call}) as u64;"


def collect_all_existing_vas() -> set[int]:
    existing = set()
    reg_dir = ROOT / "crates" / "aw2-game" / "src" / "registry"
    shards_dir = ROOT / "crates" / "aw2-game" / "src" / "shards"
    files = list(sorted(reg_dir.rglob("*.rs")))
    for p in sorted(shards_dir.rglob("*.rs")):
        rel_parts = p.relative_to(shards_dir).parts
        if rel_parts[0] not in ("ecs_more", "ecs_more.rs", "r_more", "r_more.rs", "subsystems", "subsystems.rs", "mod.rs"):
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


REG_MAP = dict(g.REG_MAP)
for i in range(16):
    REG_MAP[f"xmm{i}"] = (f"xmm{i}", 128, "low")
    REG_MAP[f"ymm{i}"] = (f"xmm{i}", 256, "low")

XMM_PARENTS = tuple(f"xmm{i}" for i in range(16))
YMM_NAMES = {f"ymm{i}": f"xmm{i}" for i in range(16)}
VOLATILE_XMM = {f"xmm{i}" for i in range(6)}
NONVOLATILE_XMM = {f"xmm{i}" for i in range(6, 16)}

PTR_SIZE = {
    "BYTE": 8,
    "WORD": 16,
    "DWORD": 32,
    "QWORD": 64,
    "XMMWORD": 128,
    "YMMWORD": 256,
}
MEM_RE = re.compile(
    r"^(?:(BYTE|WORD|DWORD|QWORD|XMMWORD|YMMWORD)\s+PTR\s+)?(?:[dcefgs]s:)?\[(?:[dcefgs]s:)?([^\]]+)\]$",
    re.IGNORECASE,
)
XYMM_WORD_RE = re.compile(r"\b([xy]mm\d+)\b", re.IGNORECASE)
R11_WORD_RE = re.compile(r"\b(r11|r11d|r11w|r11b)\b", re.IGNORECASE)
R11_BRACKET_RE = re.compile(
    r"\[\s*r11(?:\s*([+-])\s*(0x[0-9a-fA-F]+|\d+))?\s*\]", re.IGNORECASE
)

XMM_ZERO_MNS = {"vxorps", "vxorpd", "vpxor", "xorps", "xorpd", "pxor"}
XMM_MOV128_MNS = {
    "vmovups", "vmovaps", "vmovdqu", "vmovdqa", "vmovupd", "vmovapd",
    "movups", "movaps", "movdqu", "movdqa", "movupd", "movapd",
    "vlddqu", "lddqu", "vmovntdq", "movntdq", "vmovntdqa", "movntdqa", "vmovntps", "movntps", "vmovntpd", "movntpd",
}
XMM_MOV64_MNS = {"vmovsd", "movsd", "vmovq", "movq", "vmovlpd", "movlpd", "vmovlps", "movlps"}
XMM_MOV32_MNS = {"vmovss", "movss", "vmovd", "movd"}
XMM_MOVH_MNS = {"vmovhpd", "movhpd", "vmovhps", "movhps"}
XMM_ARITH_SS_MNS = {
    "vaddss": "+", "addss": "+",
    "vsubss": "-", "subss": "-",
    "vmulss": "*", "mulss": "*",
    "vdivss": "/", "divss": "/",
    "vminss": "min", "minss": "min",
    "vmaxss": "max", "maxss": "max",
}
XMM_ARITH_SD_MNS = {
    "vaddsd": "+", "addsd": "+",
    "vsubsd": "-", "subsd": "-",
    "vmulsd": "*", "mulsd": "*",
    "vdivsd": "/", "divsd": "/",
    "vminsd": "min", "minsd": "min",
    "vmaxsd": "max", "maxsd": "max",
}
XMM_BITWISE_MNS = {
    "vandps": "&", "andps": "&", "vandpd": "&", "andpd": "&", "vpand": "&", "pand": "&",
    "vorps": "|", "orps": "|", "vorpd": "|", "orpd": "|", "vpor": "|", "por": "|",
    "vxorps": "^", "xorps": "^", "vxorpd": "^", "xorpd": "^", "vpxor": "^", "pxor": "^",
    "vandnps": "&!", "andnps": "&!", "vandnpd": "&!", "andnpd": "&!", "vpandn": "&!", "pandn": "&!",
}
XMM_CMP_SS_MNS = {"vcomiss", "comiss", "vucomiss", "ucomiss"}
XMM_CMP_SD_MNS = {"vcomisd", "comisd", "vucomisd", "ucomisd"}
XMM_CVT_MNS = {
    "vcvtsi2ss", "cvtsi2ss", "vcvtsi2sd", "cvtsi2sd",
    "vcvttss2si", "cvttss2si", "vcvttsd2si", "cvttsd2si",
    "vcvtss2si", "cvtss2si", "vcvtsd2si", "cvtsd2si",
    "vcvtss2sd", "cvtss2sd", "vcvtsd2ss", "cvtsd2ss",
}
XMM_EXTR_MNS = {"vpextrq", "pextrq", "vpextrd", "pextrd", "vextractps", "extractps", "vpextrw", "pextrw", "vpextrb", "pextrb"}
XMM_INSR_MNS = {"vpinsrq", "pinsrq", "vpinsrd", "pinsrd", "vpinsrw", "pinsrw", "vpinsrb", "pinsrb"}
XMM_SHUF_MNS = {"vmovlhps", "movlhps", "vmovhlps", "movhlps", "vmovddup", "movddup", "vmovshdup", "movshdup", "vmovsldup", "movsldup"}
YMM_LANE_MNS = {"vextractf128", "vextracti128", "vinsertf128", "vinserti128", "vbroadcastf128"}
XMM_PACKED_BIN_MNS = {
    "vaddps": "vaddps", "addps": "vaddps",
    "vsubps": "vsubps", "subps": "vsubps",
    "vmulps": "vmulps", "mulps": "vmulps",
    "vdivps": "vdivps", "divps": "vdivps",
    "vminps": "vminps", "minps": "vminps",
    "vmaxps": "vmaxps", "maxps": "vmaxps",
    "vhaddps": "vhaddps", "haddps": "vhaddps",
    "vhsubps": "vhsubps", "hsubps": "vhsubps",
    "vaddsubps": "vaddsubps", "addsubps": "vaddsubps",
    "vaddpd": "vaddpd", "addpd": "vaddpd",
    "vsubpd": "vsubpd", "subpd": "vsubpd",
    "vmulpd": "vmulpd", "mulpd": "vmulpd",
    "vdivpd": "vdivpd", "divpd": "vdivpd",
    "vminpd": "vminpd", "minpd": "vminpd",
    "vmaxpd": "vmaxpd", "maxpd": "vmaxpd",
    "vpshufb": "vpshufb", "pshufb": "vpshufb",
    "vunpcklps": "vunpcklps", "unpcklps": "vunpcklps",
    "vunpckhps": "vunpckhps", "unpckhps": "vunpckhps",
    "vunpcklpd": "vunpcklpd", "unpcklpd": "vunpcklpd", "vpunpcklqdq": "vunpcklpd", "punpcklqdq": "vunpcklpd",
    "vunpckhpd": "vunpckhpd", "unpckhpd": "vunpckhpd", "vpunpckhqdq": "vunpckhpd", "punpckhqdq": "vunpckhpd",
    "vpunpckldq": "vunpcklps", "punpckldq": "vunpcklps",
    "vpunpckhdq": "vunpckhps", "punpckhdq": "vunpckhps",
    "vpaddd": "vpaddd", "paddd": "vpaddd",
    "vpsubd": "vpsubd", "psubd": "vpsubd",
    "vpaddq": "vpaddq", "paddq": "vpaddq",
    "vpsubq": "vpsubq", "psubq": "vpsubq",
    "vpmuludq": "vpmuludq", "pmuludq": "vpmuludq",
    "vpcmpeqd": "vpcmpeqd", "pcmpeqd": "vpcmpeqd",
    "vpcmpeqq": "vpcmpeqq", "pcmpeqq": "vpcmpeqq",
    "vpcmpeqb": "vpcmpeqb", "pcmpeqb": "vpcmpeqb",
    "vpcmpgtb": "vpcmpgtb", "pcmpgtb": "vpcmpgtb",
    "vpcmpgtd": "vpcmpgtd", "pcmpgtd": "vpcmpgtd",
    "vpcmpgtw": "vpcmpgtw", "pcmpgtw": "vpcmpgtw",
    "vpcmpgtq": "vpcmpgtq", "pcmpgtq": "vpcmpgtq",
    "vpcmpeqw": "vpcmpeqw", "pcmpeqw": "vpcmpeqw",
    "vpsignb": "vpsignb", "psignb": "vpsignb",
    "vpmullw": "vpmullw", "pmullw": "vpmullw",
    "vpmulld": "vpmulld", "pmulld": "vpmulld",
    "vpmaddwd": "vpmaddwd", "pmaddwd": "vpmaddwd",
    "vpackssdw": "vpackssdw", "packssdw": "vpackssdw",
    "vpaddw": "vpaddw", "paddw": "vpaddw",
    "vpsubw": "vpsubw", "psubw": "vpsubw",
    "vpunpcklbw": "vpunpcklbw", "punpcklbw": "vpunpcklbw",
    "vpunpckhbw": "vpunpckhbw", "punpckhbw": "vpunpckhbw",
    "vpunpcklwd": "vpunpcklwd", "punpcklwd": "vpunpcklwd",
    "vpunpckhwd": "vpunpckhwd", "punpckhwd": "vpunpckhwd",
    "vpmaxsw": "vpmaxsw", "pmaxsw": "vpmaxsw",
    "vpmaxsd": "vpmaxsd", "pmaxsd": "vpmaxsd",
    "vpminsd": "vpminsd", "pminsd": "vpminsd",
    "vpmaxud": "vpmaxud", "pmaxud": "vpmaxud",
    "vpaddb": "vpaddb", "paddb": "vpaddb",
    "vpmaxuw": "vpmaxuw", "pmaxuw": "vpmaxuw",
    "vpminud": "vpminud", "pminud": "vpminud",
    "vphaddd": "vphaddd", "phaddd": "vphaddd",
    "vpmulhw": "vpmulhw", "pmulhw": "vpmulhw",
    "vpackuswb": "vpackuswb", "packuswb": "vpackuswb",
    "vpmullq": "vpmullq", "pmullq": "vpmullq",
}
XMM_PACKED_UNARY_MNS = {
    "vsqrtps": "vsqrtps", "sqrtps": "vsqrtps",
    "vrsqrtps": "vrsqrtps", "rsqrtps": "vrsqrtps",
    "vrcpps": "vrcpps", "rcpps": "vrcpps",
    "vsqrtpd": "vsqrtpd", "sqrtpd": "vsqrtpd",
    "vcvtdq2ps": "vcvtdq2ps", "cvtdq2ps": "vcvtdq2ps",
    "vcvttps2dq": "vcvttps2dq", "cvttps2dq": "vcvttps2dq",
    "vcvtps2dq": "vcvtps2dq", "cvtps2dq": "vcvtps2dq",
    "vcvtdq2pd": "vcvtdq2pd", "cvtdq2pd": "vcvtdq2pd",
    "vcvtpd2ps": "vcvtpd2ps", "cvtpd2ps": "vcvtpd2ps",
    "vcvtps2pd": "vcvtps2pd", "cvtps2pd": "vcvtps2pd",
    "vpabsd": "vpabsd", "pabsd": "vpabsd",
}
XMM_PACKED_IMM_MNS = {
    "vshufps": ("vshufps", True), "shufps": ("vshufps", True),
    "vshufpd": ("vshufpd", True), "shufpd": ("vshufpd", True),
    "vpermilps": ("vpermilps", False),
    "vpermilpd": ("vpermilpd", False),
    "vpshufd": ("vpshufd", False), "pshufd": ("vpshufd", False),
    "vpshuflw": ("vpshuflw", False), "pshuflw": ("vpshuflw", False),
    "vpshufhw": ("vpshufhw", False), "pshufhw": ("vpshufhw", False),
    "vinsertps": ("vinsertps", True), "insertps": ("vinsertps", True),
    "vblendps": ("vblendps", True), "blendps": ("vblendps", True),
    "vblendpd": ("vblendpd", True), "blendpd": ("vblendpd", True),
    "vpblendw": ("vpblendw", True), "pblendw": ("vpblendw", True),
    "vdpps": ("vdpps", True), "dpps": ("vdpps", True),
    "vcmpps": ("vcmpps", True), "cmpps": ("vcmpps", True),
    "vcmppd": ("vcmppd", True), "cmppd": ("vcmppd", True),
    "vroundps": ("vroundps", False), "roundps": ("vroundps", False),
    "vroundpd": ("vroundpd", False), "roundpd": ("vroundpd", False),
    "vpsrldq": ("vpsrldq", False), "psrldq": ("vpsrldq", False),
    "vpslldq": ("vpslldq", False), "pslldq": ("vpslldq", False),
}
XMM_FMA_MNS = {
    f"v{op}{ord_s}{ty}": (op, ord_s, ty)
    for op in ("fmadd", "fnmadd", "fmsub", "fnmsub", "fmaddsub", "fmsubadd")
    for ord_s in ("132", "213", "231")
    for ty in ("ps", "pd", "ss", "sd")
}
XMM_MISC_MNS = {
    "vrsqrtss", "rsqrtss", "vrcpss", "rcpss",
    "vbroadcastss", "vbroadcastsd", "vpbroadcastb", "vpbroadcastw", "vpbroadcastd", "vpbroadcastq",
    "vblendvps", "blendvps", "vblendvpd", "blendvpd", "vpblendvb", "pblendvb",
    "vmovmskps", "movmskps", "vmovmskpd", "movmskpd", "vpmovmskb", "pmovmskb",
    "vroundss", "roundss", "vroundsd", "roundsd",
    "vcmpss", "cmpss", "vcmpsd", "cmpsd",
    "vpslld", "pslld", "vpsrld", "psrld", "vpsrad", "psrad", "vpsllq", "psllq", "vpsrlq", "psrlq",
    "vpsllw", "psllw", "vpsrlw", "psrlw", "vpsraw", "psraw",
    "vcvtps2ph", "vcvtph2ps", "vpermpd", "vpermq", "vpermd", "vpermps", "vstmxcsr", "stmxcsr", "vptest", "ptest",
    "pcmpistri",
    "vpmovzxbw", "pmovzxbw", "vpmovsxbw", "pmovsxbw",
    "vpmovzxbd", "pmovzxbd", "vpmovsxbd", "pmovsxbd",
    "vpmovzxwd", "pmovzxwd", "vpmovsxwd", "pmovsxwd",
    "vpmovzxdq", "pmovzxdq", "vpmovsxdq", "pmovsxdq",
    "vpmovzxwq", "pmovzxwq",
}

ALL_XMM_MNS = (
    XMM_ZERO_MNS
    | XMM_MOV128_MNS
    | XMM_MOV64_MNS
    | XMM_MOV32_MNS
    | XMM_MOVH_MNS
    | set(XMM_ARITH_SS_MNS)
    | set(XMM_ARITH_SD_MNS)
    | set(XMM_BITWISE_MNS)
    | XMM_CMP_SS_MNS
    | XMM_CMP_SD_MNS
    | XMM_CVT_MNS
    | XMM_EXTR_MNS
    | XMM_INSR_MNS
    | XMM_SHUF_MNS
    | YMM_LANE_MNS
    | set(XMM_PACKED_BIN_MNS)
    | set(XMM_PACKED_UNARY_MNS)
    | set(XMM_PACKED_IMM_MNS)
    | set(XMM_FMA_MNS)
    | XMM_MISC_MNS
    | {"vsqrtss", "sqrtss", "vsqrtsd", "sqrtsd"}
)
ALLOWED_V_MNS = ALL_XMM_MNS | {"vzeroupper"}


def norm_xmm_name(s: str) -> str:
    sl = s.strip().lower()
    return YMM_NAMES.get(sl, sl)


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
    s = op_str.strip()
    sl = s.lower()
    if sl in ("qword ptr gs:[0x58]", "gs:[0x58]"):
        return ("gs", 64, "(crate::exe::tls_pointer() as u64)")
    if sl in ("qword ptr gs:[0x30]", "gs:[0x30]", "qword ptr gs:[0x8]", "gs:[0x8]"):
        return ("gs", 64, "(crate::exe::teb_self() as u64)")
    if sl in ("qword ptr gs:[0x10]", "gs:[0x10]"):
        return ("gs", 64, "0x1000u64")
    if sl in REG_MAP:
        parent, bits, hl = REG_MAP[sl]
        return ("reg", parent, bits, hl)
    m = MEM_RE.match(s)
    if m:
        sz_name = m.group(1).upper() if m.group(1) else None
        inner = m.group(2)
        bits = PTR_SIZE.get(sz_name, 0)
        addr_rust = parse_addr_expr(inner)
        if addr_rust is None:
            return None
        return ("mem", bits, addr_rust)
    try:
        val = int(s, 0)
        return ("imm", val)
    except ValueError:
        return None


def op_bits(op):
    if op is None:
        return None
    if op[0] == "gs":
        return 64
    if op[0] == "reg":
        return op[2]
    if op[0] == "mem":
        return op[1] if op[1] != 0 else None
    return None


def read_op(op, expected_bits: int):
    kind = op[0]
    if kind == "gs":
        return op[2] if expected_bits in (0, 64) else None
    if kind == "reg":
        _, parent, bits, hl = op
        if expected_bits and bits != expected_bits:
            return None
        if bits in (64, 128):
            return parent
        return g.read_reg(parent, bits, hl)
    if kind == "mem":
        _, bits, addr = op
        b = bits or expected_bits
        if b not in (8, 16, 32, 64, 128):
            return None
        return f"crate::simd_ops::ru{b}({addr})"
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
        if bits == 128:
            idx = parent[3:]
            return f"{parent} = ({val_expr}) as u128; ymm{idx}_hi = 0;"
        return g.write_reg(parent, bits, hl, val_expr)
    if kind == "mem":
        _, bits, addr = op
        b = bits or expected_bits
        if b not in (8, 16, 32, 64, 128):
            return None
        return f"crate::simd_ops::wu{b}({addr}, ({val_expr}) as u{b});"
    return None


def is_stack_mem_str(s: str) -> bool:
    return bool(g.RSP_BRACKET_RE.search(s) or g.RBP_BRACKET_RE.search(s) or R11_BRACKET_RE.search(s))


# Per-function map from concrete lazy-flag tuple -> `fl_k` id, populated by the block emitter
# while it emits a function that has conflicting flag joins (see translate_blocks dataflow).
FLAG_IDS: dict = {}

# Functions rejected because their recorded extent falls off the end (see main()).
TRUNCATED_VAS: set = set()


def join_flags(flags):
    """Join lazy-flag states arriving at a block. None = no information (ignored); UNK_FLAG
    (a predecessor whose flags are clobbered by something unmodelled) absorbs: any read of
    the joined flags then rejects under STRICT_FLAGS."""
    members = set()
    for f in flags:
        if f is None:
            continue
        if f[0] == "multi":
            members.update(f[2])
        else:
            members.add(f)
    if not members:
        return None
    if UNK_FLAG in members:
        return UNK_FLAG
    if len(members) == 1:
        return next(iter(members))
    return ("multi", 0, tuple(sorted(members, key=repr)), None)


def _strict_reject(kind: str, detail: str):
    """A flag read with no exact lowering: audit it; None (reject) under STRICT_FLAGS outside
    the dataflow pre-passes, else the legacy `false`."""
    audit(kind, detail)
    if STRICT_FLAGS and not IN_DATAFLOW:
        return None
    return "false"


def add_cond(cc: str, bits: int, res: str, src: str):
    """Conditions after `add d, s` from res = d + s and s (both uN expressions)."""
    d = f"{res}.wrapping_sub({src})"
    ssum = f"(({d} as i{bits} as i128) + ({src} as i{bits} as i128))"
    table = {
        "e": f"({res} == 0)", "z": f"({res} == 0)", "ne": f"({res} != 0)", "nz": f"({res} != 0)",
        "s": f"(({res} as i{bits}) < 0)", "ns": f"(({res} as i{bits}) >= 0)",
        "b": f"({res} < {src})", "c": f"({res} < {src})", "nae": f"({res} < {src})",
        "ae": f"({res} >= {src})", "nb": f"({res} >= {src})", "nc": f"({res} >= {src})",
        "be": f"({res} == 0 || {res} < {src})", "na": f"({res} == 0 || {res} < {src})",
        "a": f"({res} != 0 && {res} >= {src})", "nbe": f"({res} != 0 && {res} >= {src})",
        "o": f"({ssum} != ({res} as i{bits} as i128))", "no": f"({ssum} == ({res} as i{bits} as i128))",
        "l": f"({ssum} < 0)", "nge": f"({ssum} < 0)", "ge": f"({ssum} >= 0)", "nl": f"({ssum} >= 0)",
        "le": f"({ssum} <= 0)", "ng": f"({ssum} <= 0)", "g": f"({ssum} > 0)", "nle": f"({ssum} > 0)",
    }
    return table.get(cc)


def cond_expr(cc: str, last_flag):
    if last_flag is None:
        return _strict_reject("none_false", cc)
    fkind, bits, l_expr, r_expr = last_flag
    if fkind == "unk":
        return _strict_reject("unk_false", cc)
    if fkind == "multi":
        arms = []
        for m in l_expr:
            c = cond_expr(cc, m)
            if c is None:
                c = _strict_reject("multi_arm_false", f"{cc}:{m[0]}")
                if c is None:
                    return None
            arms.append(f"{FLAG_IDS.get(m, 0)} => {c},")
        return f"(match fl_k {{ {' '.join(arms)} _ => false }})"
    if fkind == "add":
        return add_cond(cc, bits, l_expr, r_expr)
    if fkind == "inc_cf":
        # After inc/dec: ZF/SF from the result (l_expr), CF kept (r_expr != 0); OF unknown.
        z, cf = f"({l_expr} == 0)", f"({r_expr} != 0)"
        table = {
            "e": z, "z": z, "ne": f"!{z}", "nz": f"!{z}",
            "s": f"(({l_expr} as i{bits}) < 0)", "ns": f"(({l_expr} as i{bits}) >= 0)",
            "b": cf, "c": cf, "nae": cf, "ae": f"!{cf}", "nb": f"!{cf}", "nc": f"!{cf}",
            "be": f"({z} || {cf})", "na": f"({z} || {cf})", "a": f"!({z} || {cf})", "nbe": f"!({z} || {cf})",
        }
        return table.get(cc)
    if fkind == "cf_mask":
        # After `sbb r, r`: l_expr = r = CF ? all-ones : 0; CF = SF = (r != 0), ZF = (r == 0),
        # OF = 0. PF/AF: unknown (reject).
        nz, z = f"({l_expr} != 0)", f"({l_expr} == 0)"
        table = {
            "b": nz, "c": nz, "nae": nz, "ae": z, "nb": z, "nc": z,
            "e": z, "z": z, "ne": nz, "nz": nz, "s": nz, "ns": z,
            "o": "false", "no": "true", "be": "true", "na": "true", "a": "false", "nbe": "false",
            "l": nz, "nge": nz, "ge": z, "nl": z, "le": "true", "ng": "true", "g": "false", "nle": "false",
        }
        return table.get(cc)
    if fkind == "mul_ovf":
        if cc in ("o", "b", "c", "nae"):
            return f"({l_expr} != 0)"
        if cc in ("no", "ae", "nb", "nc"):
            return f"({l_expr} == 0)"
        return _strict_reject("mul_ovf_false", cc)
    if fkind == "pcmpstr":
        # `l_expr` = CF | ZF << 1 | SF << 2 | OF << 3 (simd_ops::pcmpistri); PF = 0.
        c, z, sg, o = (f"(({l_expr} >> {k}) & 1 != 0)" for k in range(4))
        m = {
            ("b", "c", "nae"): c, ("ae", "nb", "nc"): f"!{c}",
            ("e", "z"): z, ("ne", "nz"): f"!{z}",
            ("s",): sg, ("ns",): f"!{sg}",
            ("o",): o, ("no",): f"!{o}",
            ("be", "na"): f"({c} || {z})", ("a", "nbe"): f"(!{c} && !{z})",
            ("l", "nge"): f"({sg} != {o})", ("ge", "nl"): f"({sg} == {o})",
            ("le", "ng"): f"({z} || {sg} != {o})", ("g", "nle"): f"(!{z} && {sg} == {o})",
            ("p", "pe"): "false", ("np", "po"): "true",
        }
        for ks, v in m.items():
            if cc in ks:
                return v
        return None
    if fkind == "ptest":
        if cc in ("e", "z"):
            return f"({l_expr} == 0)"
        if cc in ("ne", "nz"):
            return f"({l_expr} != 0)"
        if cc in ("b", "c", "nae"):
            return f"({r_expr} != 0)"
        if cc in ("ae", "nb", "nc"):
            return f"({r_expr} == 0)"
        if cc in ("be", "na"):
            return f"({l_expr} == 0 || {r_expr} != 0)"
        if cc in ("a", "nbe"):
            return f"({l_expr} != 0 && {r_expr} == 0)"
        return None
    if fkind in ("fcmp32", "fcmp64"):
        ft = "f32" if fkind == "fcmp32" else "f64"
        fa = f"{ft}::from_bits({l_expr})"
        fb = f"{ft}::from_bits({r_expr})"
        if cc in ("b", "c", "nae"):
            return f"!({fa} >= {fb})"
        if cc in ("be", "na"):
            return f"!({fa} > {fb})"
        if cc in ("a", "nbe"):
            return f"({fa} > {fb})"
        if cc in ("ae", "nb", "nc"):
            return f"({fa} >= {fb})"
        if cc in ("e", "z"):
            return f"({fa} == {fb} || {fa}.is_nan() || {fb}.is_nan())"
        if cc in ("ne", "nz"):
            return f"({fa} != {fb} && !{fa}.is_nan() && !{fb}.is_nan())"
        if cc in ("p", "pe"):
            return f"({fa}.is_nan() || {fb}.is_nan())"
        if cc in ("np", "po"):
            return f"(!{fa}.is_nan() && !{fb}.is_nan())"
        return None
    res = g.cond_expr(cc, last_flag)
    if res is None:
        return _strict_reject("unsupported_false", f"{cc}:{fkind}")
    return res


FLAG_CLOBBER_MNS = g.FLAG_CLOBBER_MNS | {
    "xadd", "cmpxchg", "adc", "blsr", "blsi", "blsmsk", "andn", "shrd",
    "tzcnt", "lzcnt", "bsf", "bsr", "popcnt",
    "vcomiss", "comiss", "vucomiss", "ucomiss",
    "vcomisd", "comisd", "vucomisd", "ucomisd",
    "vptest", "ptest", "pcmpistri",
}


def _consumer_cc(mn: str):
    """Condition code a flag consumer reads (adc/sbb read CF -> "b"), or None."""
    if mn.startswith("set"):
        return mn[3:]
    if mn.startswith("cmov"):
        return mn[4:]
    if mn in ("sbb", "adc"):
        return "b"
    if mn.startswith("j") and mn != "jmp":
        return mn[1:]
    return None


def flag_consumers_after(insns, idx: int):
    """Condition codes that read the flags produced at insns[idx], up to the next producer
    (or the end of the block: `None` is appended when the flags may reach a successor)."""
    out = []
    for j in range(idx + 1, len(insns)):
        mn = insns[j][1]
        cc = _consumer_cc(mn)
        if cc is not None:
            out.append(cc)
        if mn in FLAG_CLOBBER_MNS or mn in ("adc", "sbb", "ret"):
            return out
    out.append(None)
    return out


CF_OF_SIGNED_CCS = frozenset((
    "b", "c", "nae", "ae", "nb", "nc", "be", "na", "a", "nbe", "o", "no",
    "l", "nge", "ge", "nl", "le", "ng", "g", "nle",
))
CF_CCS = frozenset(("b", "c", "nae", "ae", "nb", "nc"))


def _cf_only_in_block(insns, idx: int) -> bool:
    """Every reader of insns[idx]'s flags reads CF only, and the flags do not leave the block."""
    return all(c is not None and c in CF_CCS for c in flag_consumers_after(insns, idx))


def _settle_flag(prev, after):
    """Flag state after the instruction `prev = (mnemonic, state before it)`: under STRICT_FLAGS
    a flag-writing instruction that left no state (None), or whose handler records none
    (UNRECORDED_FLAG_MNS), yields UNK_FLAG. inc/dec deliberately keep the prior state for
    CF-only readers (see the inc/dec handler)."""
    if not STRICT_FLAGS:
        return after
    mn, _before = prev
    if mn not in FLAG_CLOBBER_MNS and mn not in ("adc", "sbb", "call"):
        return after
    if after is None or mn in UNRECORDED_FLAG_MNS:
        return UNK_FLAG
    return after


# Handled flag-writing mnemonics whose handler leaves last_flag untouched.
UNRECORDED_FLAG_MNS = frozenset(("shrd",))


def flags_observed_after(insns, idx: int) -> bool:
    for j in range(idx + 1, len(insns)):
        mn = insns[j][1]
        if (
            mn.startswith("set")
            or mn.startswith("cmov")
            or mn in ("sbb", "adc")
            or (mn.startswith("j") and mn != "jmp")
        ):
            return True
        if mn in FLAG_CLOBBER_MNS:
            return False
    return True


def cf_only_observed_after_inc_dec(insns, idx: int) -> bool:
    for j in range(idx + 1, len(insns)):
        mn = insns[j][1]
        if mn in ("sbb", "adc"):
            return True
        if mn.startswith("set") or mn.startswith("cmov") or (mn.startswith("j") and mn != "jmp"):
            cc = mn[3:] if mn.startswith("set") else (mn[4:] if mn.startswith("cmov") else mn[1:])
            return cc in ("b", "c", "nae", "ae", "nb", "nc")
        if mn in FLAG_CLOBBER_MNS:
            return False
    return False


def read_xmm_f32(op):
    if op[0] == "reg" and op[2] == 128:
        return f"f32::from_bits({op[1]} as u32)"
    if op[0] == "mem" and op[1] in (0, 32):
        return f"f32::from_bits(crate::simd_ops::ru32({op[2]}))"
    return None


def read_xmm_f64(op):
    if op[0] == "reg" and op[2] == 128:
        return f"f64::from_bits({op[1]} as u64)"
    if op[0] == "mem" and op[1] in (0, 64):
        return f"f64::from_bits(crate::simd_ops::ru64({op[2]}))"
    return None


def set_xmm(parent: str, expr: str) -> str:
    idx = parent[3:]
    return f"{parent} = {expr}; ymm{idx}_hi = 0;"


def set_ymm(parent: str, lo_expr: str, hi_expr: str) -> str:
    idx = parent[3:]
    return f"let (y_lo, y_hi): (u128, u128) = ({lo_expr}, {hi_expr}); {parent} = y_lo; ymm{idx}_hi = y_hi;"


def read_ymm_halves(op):
    if op[0] == "reg" and op[2] == 256:
        idx = op[1][3:]
        return (op[1], f"ymm{idx}_hi")
    if op[0] == "mem":
        addr = op[2]
        return (
            f"core::ptr::read_unaligned(({addr}) as *const u128)",
            f"core::ptr::read_unaligned(({addr}).wrapping_add(16) as *const u128)",
        )
    return None


# Trapping instructions raise the guest exception the emulator raises for them
# (crates/aw2-game/src/lift_ops.rs `trap`: vectored handlers, then the unhandled path: crash
# report, filter, exit). They used to lower to a plain `return rax`, which let e.g. the int3
# after a noreturn `__scrt_fastfail` / `V8_Fatal` call "return" into a corrupt caller.
TRAP_CODES = {"int3": 0x80000003, "ud2": 0xC000001D}
# AW2_GEN_TRAPS=0: the old lowering (plain `return rax`), for A/B and census diffs.
TRAPS = os.environ.get("AW2_GEN_TRAPS", "1") != "0"
# int3s appended after a function's trailing noreturn call (they were dropped as padding). The
# single-block fast path (translate_block) is skipped for them, so such functions keep the CFG
# form they had and differ from the old lowering only by the trap statement.
TAIL_TRAP_VAS = set()


def int_trap_code(ops_str: str) -> int:
    """NTSTATUS of `int n`: 0x29 __fastfail (STATUS_STACK_BUFFER_OVERRUN), 0x2c assertion
    (STATUS_ASSERTION_FAILURE), 3 breakpoint; any other vector is a #GP in user mode
    (STATUS_ACCESS_VIOLATION)."""
    s = (ops_str.split() or [""])[0].lower()
    try:
        n = int(s[:-1], 16) if s.endswith("h") else int(s, 0)
    except ValueError:
        n = -1
    return {0x29: 0xC0000409, 0x2C: 0xC0000420, 3: 0x80000003}.get(n, 0xC0000005)


TRAP_P_RE = re.compile(r"\bTRAP_P_(\d+)\b")


def trap_stmt(code: int, addr: int, uses_stack: bool, ops_str: str = "") -> str:
    """`crate::lift_ops::trap(code, site, rcx, rsp)`; the site is the function's parameter
    (TRAP_P_n, see translate_function) when there is one, else the literal address."""
    m = TRAP_P_RE.search(ops_str)
    site = f"P{m.group(1)}" if m else f"0x{addr:x}"
    rsp = "rsp" if uses_stack else "0"
    return f"crate::lift_ops::trap(0x{code:x}, {site}, rcx, {rsp});"


def eh_leave() -> str:
    """Before a tail jump of an `EH_CUR` fn: unlink its EH record (the frame is gone, as after
    the x86 epilog; the callee must not see this frame's handler with a stale state)."""
    return "drop(_eh); " if EH_CUR is not None else ""


def eh_landing(addr_to_bb) -> str:
    """`Err(l)` arm of a catching call (`EH_CUR`): resume at the catch continuation block the
    landing names (`l.cont_va`); a continuation that is no block of this body (linear
    translation, or outside the lifted extent) is fatal (`Landing::unknown`). rax at the
    target is RtlUnwindEx's ReturnValue (`landed_rax`: the exception code at an SEH `__except`
    body, `seh_conts`); a setjmp continuation (`sj_conts`) instead takes the longjmp's
    registers (`SJ_LAND`, see sj_fill)."""
    fn = EH_CUR["fn_va"]
    sj = set(EH_CUR.get("sj_conts", ()))
    rv = "rax = crate::dll_native::lift::eh::landed_rax(); "
    arms = [
        f"0x{c:x} => {{ {SJ_LAND if c in sj else rv}bb = {addr_to_bb[c]}; continue; }}"
        for c in EH_CUR["conts"]
        if addr_to_bb and c in addr_to_bb
    ]
    if arms:
        return f"match l.cont_va {{ {' '.join(arms)} _ => l.unknown(0x{fn:x}) }}"
    return f"l.unknown(0x{fn:x})"


# `seh_guards` of an SEH `EH_CUR` fn (dll_lift `seh_scopes`): instructions inside an
# `__except` scope whose memory operand can fault. Their `simd_ops::ru*/wu*` accesses become
# `EhFrame::ru*/wu*` (lift::eh; probes of lift::guarded): a fault raises
# STATUS_ACCESS_VIOLATION into this frame at the instruction, and a landing resumes at the
# `__except` block (`eh_landing`). Anything else touching memory in such an instruction
# (locked / vector forms, lazy flags that re-read memory later) cannot be guarded: the fn is
# rejected with the named reason in SEH_GUARD_FAIL[0].
SEH_GUARD_FAIL = [None]
# Reason a translate_function() call returned None (frontier_plan.md §4), set at the failure
# site; reported as the fn's reject reason when SEH_GUARD_FAIL[0] is unset.
TRANSLATE_FAIL = [None]


def _tfail(reason: str):
    TRANSLATE_FAIL[0] = reason
    return None


def _insn_fail_reason(insns) -> str:
    """The insn translate_linear_insns() stopped at (AUDIT_CTX), with its operands."""
    addr, mn = AUDIT_CTX[1], AUDIT_CTX[2]
    ops = next((o for a, m, o, _ in insns if a == addr), "")
    return f"untranslatable insn at 0x{addr:x}: {mn} {ops}".rstrip()
_GUARD_ACC_RE = re.compile(r"crate::simd_ops::(ru|wu)(8|16|32|64|128)\(")
_GUARD_BAD = ("simd_ops::lk_", "read_unaligned", "write_unaligned", "simd_ops::ru128", "simd_ops::wu128")


def _call_args(s: str, open_i: int):
    """Top-level comma-split arguments of the call whose `(` is at `open_i`, and the index
    after its `)`; None when unbalanced."""
    depth, args, start = 0, [], open_i + 1
    for i in range(open_i, len(s)):
        c = s[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                args.append(s[start:i].strip())
                return args, i + 1
        elif c == "," and depth == 1:
            args.append(s[start:i].strip())
            start = i + 1
    return None


def _guard_expr(s: str, va: int, land: str):
    """`s` with every `simd_ops::ru*/wu*` access (nested ones too: the read of an RMW sits
    in its write's value) guarded at site `va`; (text, accesses). ValueError(why) otherwise."""
    out, pos, n = [], 0, 0
    while True:
        m = _GUARD_ACC_RE.search(s, pos)
        if m is None:
            out.append(s[pos:])
            return "".join(out), n
        got = _call_args(s, m.end() - 1)
        if got is None:
            raise ValueError("unbalanced access")
        args, end = got
        kind, bits = m.group(1), m.group(2)
        if bits == "128" or len(args) != (1 if kind == "ru" else 2):
            raise ValueError("unguardable access form")
        g_args = []
        for a in args:
            t, k = _guard_expr(a, va, land)
            g_args.append(t)
            n += k
        if kind == "ru":
            rep = f"(match eh.ru{bits}(0x{va:x}, {g_args[0]}) {{ Ok(v) => v, Err(l) => {land} }})"
        else:
            if end < len(s) and s[end] == ";":
                end += 1
            rep = f"if let Err(l) = eh.wu{bits}(0x{va:x}, {g_args[0]}, {g_args[1]}) {{ {land} }}"
        out.append(s[pos:m.start()])
        out.append(rep)
        pos = end
        n += 1


def seh_guard_lines(lines: list, va: int, start: int, addr_to_bb, last_flag) -> bool:
    """Guard the accesses of instruction `va` (its lines are `lines[start:]`, in place)."""
    def fail(why):
        SEH_GUARD_FAIL[0] = f"seh_guard_unlowered ({why} at 0x{va:x})"
        return False

    if "simd_ops::" in repr(last_flag):
        return fail("lazy flags re-read memory")
    land = eh_landing(addr_to_bb)
    n, done = 0, []
    for s in lines[start:]:
        if any(b in s for b in _GUARD_BAD):
            return fail("unguardable access form")
        try:
            t, k = _guard_expr(s, va, land)
        except ValueError as e:
            return fail(str(e))
        done.append(t)
        n += k
    if n == 0:
        return fail("no access lowered")
    lines[start:] = done
    return True


# setjmp sites of an `EH_CUR` fn (dll_lift `setjmp_sites`): `call __intrinsic_setjmp` becomes
# `eh.setjmp(site, rcx, rdx, return VA, SJ_ARGS)` and its continuation arm starts with
# `SJ_LAND`; `sj_fill` replaces both markers once the whole body is known, with the
# nonvolatile locals it has (a register the body never names holds no lifted state: 0 in the
# buffer, not restored).
SJ_ARGS = "/*SJ_ARGS*/"
SJ_LAND = "/*SJ_LAND*/ "
SJ_NV = ("rbx", "rbp", "rsi", "rdi", "r12", "r13", "r14", "r15")
SJ_GPR_IDX = {"rbx": 3, "rsp": 4, "rbp": 5, "rsi": 6, "rdi": 7, "r12": 12, "r13": 13, "r14": 14, "r15": 15}


def sj_fill(lines: list[str]) -> list[str]:
    text = "\n".join(lines)
    if SJ_ARGS not in text and SJ_LAND not in text:
        return lines
    words = set(IDENT_RE.findall(text.replace(SJ_ARGS, "").replace(SJ_LAND, "")))
    have = lambda r: r in words  # noqa: E731
    nv = ", ".join(r if have(r) else "0" for r in SJ_NV)
    xmm = ", ".join(f"xmm{k}" if have(f"xmm{k}") else "0" for k in range(6, 16))
    args = f"{'rsp' if have('rsp') else '0'}, [{nv}], [{xmm}]"
    land = ["let j = crate::dll_native::lift::eh::longjmp_regs(); rax = j.gpr[0];"]
    land += [f"{r} = j.gpr[{SJ_GPR_IDX[r]}];" for r in ("rsp",) + SJ_NV if have(r)]
    land += [f"xmm{k} = j.xmm[{k - 6}];" for k in range(6, 16) if have(f"xmm{k}")]
    land_s = " ".join(land) + " "
    return [l.replace(SJ_ARGS, args).replace(SJ_LAND, land_s) for l in lines]


def translate_linear_insns(
    insns,
    initial_flag=None,
    addr_to_bb=None,
    uses_stack: bool = False,
    uses_xmm_out: bool = False,
    uses_xmm_call: bool = False,
    eh_mode: int = 0,
):
    lines = []
    last_flag = initial_flag
    ret_stmt = "crate::simd_ops::set_xmm_ret(xmm0); return rax;" if uses_xmm_out else "return rax;"
    lock_next = False
    prev = None  # (mnemonic, flag state before it) of the previous instruction
    guards = set(EH_CUR.get("seh_guards", ())) if EH_CUR is not None else set()
    guard_at = None  # (VA, first line index) of a seh_guards instruction being translated
    for idx, (addr, mn, ops_str, next_addr) in enumerate(insns):
        if guard_at is not None:
            if not seh_guard_lines(lines, *guard_at, addr_to_bb, last_flag):
                return None, None
            guard_at = None
        if addr in guards:
            guard_at = (addr, len(lines))
        if prev is not None:
            last_flag = _settle_flag(prev, last_flag)
        prev = (mn, last_flag)
        AUDIT_CTX[1], AUDIT_CTX[2] = addr, mn
        ops = g.split_operands(ops_str)
        # `locked`: this instruction carries an F0 prefix (see LOCKED_INSN_VAS).
        locked = (lock_next or addr in LOCKED_INSN_VAS) and LOCK_RMW_MODE != "legacy-plain"
        lock_next = mn == "lock"
        if locked and mn not in LOCK_LOWERED_MNS:
            # No atomic lowering for this lock-prefixed form (or a data misdecode): reject.
            return None, None
        if mn == "mfence":
            # Only StoreLoad barrier besides locked ops: ldapr may pass an earlier stlr.
            if LOCK_RMW_MODE != "legacy-plain":
                lines.append("core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);")
            continue
        if mn in ("nop", "vzeroupper", "pause", "lock", "cld", "lfence", "sfence"):
            continue
        elif mn in ("ret", "int3", "ud2"):
            if idx != len(insns) - 1:
                return None, None
            if mn != "ret" and TRAPS:
                lines.append(trap_stmt(TRAP_CODES[mn], addr, uses_stack, ops_str))
            lines.append(ret_stmt)
        elif mn in XMM_ZERO_MNS and (
            (len(ops) == 3 and ops[1] == ops[2]) or (len(ops) == 2 and ops[0] == ops[1])
        ):
            dst = parse_op(ops[0])
            if not dst or dst[0] != "reg" or dst[2] not in (128, 256):
                return None, None
            lines.append(set_xmm(dst[1], "0u128"))
        elif mn in XMM_MOV128_MNS:
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src:
                return None, None
            dbits = op_bits(dst)
            sbits = op_bits(src)
            if dbits == 256 or sbits == 256:
                if dst[0] == "reg" and dst[2] == 256 and src[0] == "reg" and src[2] == 256:
                    d_i = dst[1][3:]
                    s_i = src[1][3:]
                    lines.append(f"{dst[1]} = {src[1]}; ymm{d_i}_hi = ymm{s_i}_hi;")
                elif dst[0] == "reg" and dst[2] == 256 and src[0] == "mem":
                    d_i = dst[1][3:]
                    lines.append(f"let y_addr: u64 = {src[2]}; {dst[1]} = core::ptr::read_unaligned(y_addr as *const u128); ymm{d_i}_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128);")
                elif dst[0] == "mem" and src[0] == "reg" and src[2] == 256:
                    s_i = src[1][3:]
                    lines.append(f"let y_addr: u64 = {dst[2]}; core::ptr::write_unaligned(y_addr as *mut u128, {src[1]}); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm{s_i}_hi);")
                else:
                    return None, None
                continue
            if dbits != 128 and sbits != 128:
                return None, None
            s_expr = read_op(src, 128)
            if s_expr is None:
                return None, None
            w_stmt = write_op(dst, s_expr, 128)
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
        elif mn in YMM_LANE_MNS:
            if mn in ("vextractf128", "vextracti128"):
                if len(ops) != 3:
                    return None, None
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                imm = parse_op(ops[2])
                if not dst or not src or not imm or src[0] != "reg" or src[2] != 256 or imm[0] != "imm":
                    return None, None
                s_i = src[1][3:]
                lane_expr = f"ymm{s_i}_hi" if (imm[1] & 1) else src[1]
                w_stmt = write_op(dst, lane_expr, 128)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
            elif mn in ("vinsertf128", "vinserti128"):
                if len(ops) != 4:
                    return None, None
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
                imm = parse_op(ops[3])
                if not dst or not src1 or not src2 or not imm or dst[0] != "reg" or dst[2] != 256 or src1[0] != "reg" or src1[2] != 256 or imm[0] != "imm":
                    return None, None
                s2_expr = read_op(src2, 128)
                if s2_expr is None:
                    return None, None
                d_i = dst[1][3:]
                s1_i = src1[1][3:]
                if imm[1] & 1:
                    lines.append(f"let ins_v: u128 = {s2_expr}; {dst[1]} = {src1[1]}; ymm{d_i}_hi = ins_v;")
                else:
                    lines.append(f"let ins_v: u128 = {s2_expr}; ymm{d_i}_hi = ymm{s1_i}_hi; {dst[1]} = ins_v;")
            elif mn == "vbroadcastf128":
                if len(ops) != 2:
                    return None, None
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                if not dst or not src or dst[0] != "reg" or dst[2] != 256 or src[0] != "mem":
                    return None, None
                s_expr = read_op(src, 128)
                if s_expr is None:
                    return None, None
                d_i = dst[1][3:]
                lines.append(f"let bc_v: u128 = {s_expr}; {dst[1]} = bc_v; ymm{d_i}_hi = bc_v;")
            else:
                return None, None
        elif mn in XMM_MOV64_MNS:
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                if not dst or not src:
                    return None, None
                if dst[0] == "reg" and dst[2] == 128 and src[0] == "reg" and src[2] == 128:
                    if mn in ("movq", "vmovq"):
                        lines.append(set_xmm(dst[1], f"({src[1]} as u64) as u128"))
                    else:
                        lines.append(
                            set_xmm(dst[1], f"({dst[1]} & !0xffff_ffff_ffff_ffffu128) | ({src[1]} & 0xffff_ffff_ffff_ffffu128)")
                        )
                elif dst[0] == "reg" and dst[2] == 128:
                    s_expr = read_op(src, 64)
                    if s_expr is None:
                        return None, None
                    if mn in ("movlpd", "movlps"):
                        lines.append(
                            set_xmm(dst[1], f"({dst[1]} & !0xffff_ffff_ffff_ffffu128) | ((({s_expr}) as u64) as u128)")
                        )
                    else:
                        lines.append(set_xmm(dst[1], f"(({s_expr}) as u64) as u128"))
                elif src[0] == "reg" and src[2] == 128:
                    w_stmt = write_op(dst, f"({src[1]} as u64)", 64)
                    if w_stmt is None:
                        return None, None
                    lines.append(w_stmt)
                else:
                    return None, None
            elif len(ops) == 3 and mn in ("vmovsd", "vmovlpd", "vmovlps"):
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
                if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] != 128 or src1[0] != "reg" or src1[2] != 128:
                    return None, None
                if src2[0] == "reg" and src2[2] == 128:
                    low64 = f"({src2[1]} & 0xffff_ffff_ffff_ffffu128)"
                else:
                    s2 = read_op(src2, 64)
                    if s2 is None:
                        return None, None
                    low64 = f"((({s2}) as u64) as u128)"
                lines.append(set_xmm(dst[1], f"({src1[1]} & !0xffff_ffff_ffff_ffffu128) | {low64}"))
            else:
                return None, None
        elif mn in XMM_MOV32_MNS:
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                if not dst or not src:
                    return None, None
                if dst[0] == "reg" and dst[2] == 128 and src[0] == "reg" and src[2] == 128:
                    lines.append(set_xmm(dst[1], f"({dst[1]} & !0xffff_ffffu128) | ({src[1]} & 0xffff_ffffu128)"))
                elif dst[0] == "reg" and dst[2] == 128:
                    s_expr = read_op(src, 32)
                    if s_expr is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"(({s_expr}) as u32) as u128"))
                elif src[0] == "reg" and src[2] == 128:
                    w_stmt = write_op(dst, f"({src[1]} as u32)", 32)
                    if w_stmt is None:
                        return None, None
                    lines.append(w_stmt)
                else:
                    return None, None
            elif len(ops) == 3 and mn == "vmovss":
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
                if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] != 128 or src1[0] != "reg" or src1[2] != 128 or src2[0] != "reg" or src2[2] != 128:
                    return None, None
                lines.append(set_xmm(dst[1], f"({src1[1]} & !0xffff_ffffu128) | ({src2[1]} & 0xffff_ffffu128)"))
            else:
                return None, None
        elif mn in XMM_MOVH_MNS:
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src = parse_op(ops[1])
                if not dst or not src:
                    return None, None
                if dst[0] == "mem" and src[0] == "reg" and src[2] == 128:
                    lines.append(write_op(dst, f"(({src[1]} >> 64) as u64)", 64))
                elif dst[0] == "reg" and dst[2] == 128 and src[0] == "mem":
                    s_expr = read_op(src, 64)
                    if s_expr is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"({dst[1]} & 0xffff_ffff_ffff_ffffu128) | ((({s_expr}) as u128) << 64)"))
                else:
                    return None, None
            elif len(ops) == 3:
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
                if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] != 128 or src1[0] != "reg" or src1[2] != 128 or src2[0] != "mem":
                    return None, None
                s_expr = read_op(src2, 64)
                if s_expr is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({src1[1]} & 0xffff_ffff_ffff_ffffu128) | ((({s_expr}) as u128) << 64)"))
            else:
                return None, None
        elif mn in XMM_SHUF_MNS:
            dst = parse_op(ops[0])
            if not dst or dst[0] != "reg" or dst[2] != 128:
                return None, None
            if mn in ("vmovlhps", "movlhps", "vmovhlps", "movhlps"):
                s1 = parse_op(ops[1])
                s2 = parse_op(ops[2]) if len(ops) == 3 else s1
                if len(ops) == 2:
                    s1, s2 = dst, parse_op(ops[1])
                if not s1 or not s2 or s1[0] != "reg" or s1[2] != 128 or s2[0] != "reg" or s2[2] != 128:
                    return None, None
                if "lh" in mn:
                    lines.append(set_xmm(dst[1], f"({s1[1]} & 0xffff_ffff_ffff_ffffu128) | (({s2[1]} & 0xffff_ffff_ffff_ffffu128) << 64)"))
                else:
                    lines.append(set_xmm(dst[1], f"({s1[1]} & !0xffff_ffff_ffff_ffffu128) | (({s2[1]} >> 64) & 0xffff_ffff_ffff_ffffu128)"))
            elif mn in ("vmovddup", "movddup") and len(ops) == 2:
                src = parse_op(ops[1])
                if not src:
                    return None, None
                s64 = f"({src[1]} as u64)" if (src[0] == "reg" and src[2] == 128) else read_op(src, 64)
                if s64 is None:
                    return None, None
                lines.append(f"let ddup: u128 = ({s64} as u64) as u128; " + set_xmm(dst[1], "ddup | (ddup << 64)"))
            elif mn in ("vmovshdup", "movshdup", "vmovsldup", "movsldup") and len(ops) == 2:
                src = parse_op(ops[1])
                if not src:
                    return None, None
                s128 = read_op(src, 128)
                if s128 is None:
                    return None, None
                if "shdup" in mn:
                    lines.append(f"let sv: u128 = {s128}; let w1 = (sv >> 32) & 0xffff_ffffu128; let w3 = (sv >> 96) & 0xffff_ffffu128; " + set_xmm(dst[1], "w1 | (w1 << 32) | (w3 << 64) | (w3 << 96)"))
                else:
                    lines.append(f"let sv: u128 = {s128}; let w0 = sv & 0xffff_ffffu128; let w2 = (sv >> 64) & 0xffff_ffffu128; " + set_xmm(dst[1], "w0 | (w0 << 32) | (w2 << 64) | (w2 << 96)"))
            else:
                return None, None
        elif mn in XMM_ARITH_SS_MNS:
            op_sym = XMM_ARITH_SS_MNS[mn]
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src1 = dst
                src2 = parse_op(ops[1])
            elif len(ops) == 3:
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
            else:
                return None, None
            if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] != 128 or src1[0] != "reg" or src1[2] != 128:
                return None, None
            f1 = read_xmm_f32(src1)
            f2 = read_xmm_f32(src2)
            if f1 is None or f2 is None:
                return None, None
            if op_sym == "min":
                res_f = f"(if {f1} < {f2} {{ {f1} }} else {{ {f2} }}).to_bits() as u128"
            elif op_sym == "max":
                res_f = f"(if {f1} > {f2} {{ {f1} }} else {{ {f2} }}).to_bits() as u128"
            else:
                res_f = f"({f1} {op_sym} {f2}).to_bits() as u128"
            lines.append(set_xmm(dst[1], f"({src1[1]} & !0xffff_ffffu128) | ({res_f})"))
        elif mn in XMM_ARITH_SD_MNS:
            op_sym = XMM_ARITH_SD_MNS[mn]
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src1 = dst
                src2 = parse_op(ops[1])
            elif len(ops) == 3:
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
            else:
                return None, None
            if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] != 128 or src1[0] != "reg" or src1[2] != 128:
                return None, None
            f1 = read_xmm_f64(src1)
            f2 = read_xmm_f64(src2)
            if f1 is None or f2 is None:
                return None, None
            if op_sym == "min":
                res_f = f"(if {f1} < {f2} {{ {f1} }} else {{ {f2} }}).to_bits() as u128"
            elif op_sym == "max":
                res_f = f"(if {f1} > {f2} {{ {f1} }} else {{ {f2} }}).to_bits() as u128"
            else:
                res_f = f"({f1} {op_sym} {f2}).to_bits() as u128"
            lines.append(set_xmm(dst[1], f"({src1[1]} & !0xffff_ffff_ffff_ffffu128) | ({res_f})"))
        elif mn in ("vsqrtss", "sqrtss", "vsqrtsd", "sqrtsd"):
            dst = parse_op(ops[0])
            src = parse_op(ops[-1])
            base_reg = parse_op(ops[1]) if len(ops) == 3 else dst
            if not dst or not src or not base_reg or dst[0] != "reg" or dst[2] != 128 or base_reg[0] != "reg" or base_reg[2] != 128:
                return None, None
            if mn.endswith("ss"):
                fs = read_xmm_f32(src)
                if fs is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffffu128) | (({fs}).sqrt().to_bits() as u128)"))
            else:
                fs = read_xmm_f64(src)
                if fs is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffff_ffff_ffffu128) | (({fs}).sqrt().to_bits() as u128)"))
        elif mn in XMM_BITWISE_MNS:
            op_sym = XMM_BITWISE_MNS[mn]
            if len(ops) == 2:
                dst = parse_op(ops[0])
                src1 = dst
                src2 = parse_op(ops[1])
            elif len(ops) == 3:
                dst = parse_op(ops[0])
                src1 = parse_op(ops[1])
                src2 = parse_op(ops[2])
            else:
                return None, None
            if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] not in (128, 256) or src1[0] != "reg" or src1[2] not in (128, 256):
                return None, None
            if dst[2] == 256:
                h1 = read_ymm_halves(src1)
                h2 = read_ymm_halves(src2)
                if not h1 or not h2:
                    return None, None
                if op_sym == "&!":
                    lines.append(set_ymm(dst[1], f"(!({h1[0]})) & ({h2[0]})", f"(!({h1[1]})) & ({h2[1]})"))
                else:
                    lines.append(set_ymm(dst[1], f"({h1[0]}) {op_sym} ({h2[0]})", f"({h1[1]}) {op_sym} ({h2[1]})"))
            else:
                e1 = read_op(src1, 128)
                e2 = read_op(src2, 128)
                if e1 is None or e2 is None:
                    return None, None
                if op_sym == "&!":
                    lines.append(set_xmm(dst[1], f"(!({e1})) & ({e2})"))
                else:
                    lines.append(set_xmm(dst[1], f"({e1}) {op_sym} ({e2})"))
        elif mn in XMM_CMP_SS_MNS:
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2 or op1[0] != "reg" or op1[2] != 128:
                return None, None
            e1 = f"({op1[1]} as u32)"
            e2 = f"({op2[1]} as u32)" if (op2[0] == "reg" and op2[2] == 128) else read_op(op2, 32)
            if e2 is None:
                return None, None
            lines.append(f"fl_l = ({e1}) as u32 as u64; fl_r = ({e2}) as u32 as u64;")
            last_flag = ("fcmp32", 32, "(fl_l as u32)", "(fl_r as u32)")
        elif mn in XMM_CMP_SD_MNS:
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2 or op1[0] != "reg" or op1[2] != 128:
                return None, None
            e1 = f"({op1[1]} as u64)"
            e2 = f"({op2[1]} as u64)" if (op2[0] == "reg" and op2[2] == 128) else read_op(op2, 64)
            if e2 is None:
                return None, None
            lines.append(f"fl_l = ({e1}) as u64; fl_r = ({e2}) as u64;")
            last_flag = ("fcmp64", 64, "fl_l", "fl_r")
        elif mn in XMM_CVT_MNS:
            dst = parse_op(ops[0])
            src = parse_op(ops[-1])
            base_reg = parse_op(ops[1]) if len(ops) == 3 else dst
            if not dst or not src:
                return None, None
            if mn in ("vcvtsi2ss", "cvtsi2ss"):
                if dst[0] != "reg" or dst[2] != 128 or not base_reg or base_reg[0] != "reg" or base_reg[2] != 128:
                    return None, None
                sbits = op_bits(src) or 32
                if sbits not in (32, 64):
                    return None, None
                se = read_op(src, sbits)
                if se is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffffu128) | (((({se}) as i{sbits}) as f32).to_bits() as u128)"))
            elif mn in ("vcvtsi2sd", "cvtsi2sd"):
                if dst[0] != "reg" or dst[2] != 128 or not base_reg or base_reg[0] != "reg" or base_reg[2] != 128:
                    return None, None
                sbits = op_bits(src) or 32
                if sbits not in (32, 64):
                    return None, None
                se = read_op(src, sbits)
                if se is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffff_ffff_ffffu128) | (((({se}) as i{sbits}) as f64).to_bits() as u128)"))
            elif mn in ("vcvttss2si", "cvttss2si", "vcvtss2si", "cvtss2si"):
                if dst[0] != "reg" or dst[2] not in (32, 64):
                    return None, None
                dbits = dst[2]
                fs = read_xmm_f32(src)
                if fs is None:
                    return None, None
                fe = f"({fs}).round_ties_even()" if "cvts" in mn else f"({fs})"
                lines.append(write_op(dst, f"crate::simd_ops::cvt_f32_i{dbits}({fe})", dbits))
            elif mn in ("vcvttsd2si", "cvttsd2si", "vcvtsd2si", "cvtsd2si"):
                if dst[0] != "reg" or dst[2] not in (32, 64):
                    return None, None
                dbits = dst[2]
                fs = read_xmm_f64(src)
                if fs is None:
                    return None, None
                fe = f"({fs}).round_ties_even()" if "cvts" in mn else f"({fs})"
                lines.append(write_op(dst, f"crate::simd_ops::cvt_f64_i{dbits}({fe})", dbits))
            elif mn in ("vcvtss2sd", "cvtss2sd"):
                if dst[0] != "reg" or dst[2] != 128 or not base_reg or base_reg[0] != "reg" or base_reg[2] != 128:
                    return None, None
                fs = read_xmm_f32(src)
                if fs is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffff_ffff_ffffu128) | ((({fs}) as f64).to_bits() as u128)"))
            elif mn in ("vcvtsd2ss", "cvtsd2ss"):
                if dst[0] != "reg" or dst[2] != 128 or not base_reg or base_reg[0] != "reg" or base_reg[2] != 128:
                    return None, None
                fs = read_xmm_f64(src)
                if fs is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & !0xffff_ffffu128) | ((({fs}) as f32).to_bits() as u128)"))
            else:
                return None, None
        elif mn in XMM_PACKED_BIN_MNS:
            fn_name = XMM_PACKED_BIN_MNS[mn]
            dst = parse_op(ops[0]) if ops else None
            s1 = parse_op(ops[1]) if len(ops) == 3 else dst
            s2 = parse_op(ops[-1]) if len(ops) in (2, 3) else None
            if not dst or not s1 or not s2 or dst[0] != "reg" or dst[2] not in (128, 256) or s1[0] != "reg" or s1[2] not in (128, 256):
                return None, None
            if dst[2] == 256:
                h1 = read_ymm_halves(s1)
                h2 = read_ymm_halves(s2)
                if not h1 or not h2:
                    return None, None
                lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_name}({h1[0]}, {h2[0]})", f"crate::simd_ops::{fn_name}({h1[1]}, {h2[1]})"))
            else:
                e1 = s1[1]
                e2 = s2[1] if (s2[0] == "reg" and s2[2] in (128, 256)) else read_op(s2, 128)
                if e2 is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_name}({e1}, {e2})"))
        elif mn in XMM_PACKED_UNARY_MNS:
            fn_name = XMM_PACKED_UNARY_MNS[mn]
            if len(ops) not in (2, 3):
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[-1])
            if not dst or not src or dst[0] != "reg" or dst[2] not in (128, 256):
                return None, None
            if dst[2] == 256 and mn in ("vcvtdq2pd", "vcvtps2pd"):
                se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 128)
                if se is None:
                    return None, None
                lines.append(
                    f"let cv_in: u128 = {se}; "
                    + set_ymm(
                        dst[1],
                        f"crate::simd_ops::{fn_name}(cv_in & 0xffff_ffff_ffff_ffffu128)",
                        f"crate::simd_ops::{fn_name}(cv_in >> 64)",
                    )
                )
            elif mn == "vcvtpd2ps" and src[0] == "reg" and src[2] == 256:
                hs = read_ymm_halves(src)
                lines.append(
                    set_xmm(
                        dst[1],
                        f"(crate::simd_ops::vcvtpd2ps({hs[0]}) & 0xffff_ffff_ffff_ffffu128) | (crate::simd_ops::vcvtpd2ps({hs[1]}) << 64)",
                    )
                )
            elif dst[2] == 256:
                hs = read_ymm_halves(src)
                if not hs:
                    return None, None
                lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_name}({hs[0]})", f"crate::simd_ops::{fn_name}({hs[1]})"))
            else:
                if mn in ("vcvtdq2pd", "cvtdq2pd", "vcvtps2pd", "cvtps2pd"):
                    se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else f"(({read_op(src, 64)}) as u128)" if read_op(src, 64) else None
                else:
                    se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 128)
                if se is None:
                    return None, None
                lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_name}({se})"))
        elif mn in XMM_PACKED_IMM_MNS:
            fn_name, is_bin = XMM_PACKED_IMM_MNS[mn]
            if not ops or (len(ops) < 3 if is_bin else len(ops) < 2):
                return None, None
            dst = parse_op(ops[0])
            imm = parse_op(ops[-1])
            if mn == "vpermilps" and len(ops) == 3 and imm and imm[0] != "imm":
                s1 = parse_op(ops[1])
                s2 = imm
                if not dst or not s1 or not s2 or dst[0] != "reg" or s1[0] != "reg":
                    return None, None
                if dst[2] == 256:
                    h1 = read_ymm_halves(s1)
                    h2 = read_ymm_halves(s2)
                    if not h1 or not h2:
                        return None, None
                    lines.append(set_ymm(dst[1], f"crate::simd_ops::vpermilps_var({h1[0]}, {h2[0]})", f"crate::simd_ops::vpermilps_var({h1[1]}, {h2[1]})"))
                else:
                    e2 = s2[1] if (s2[0] == "reg" and s2[2] in (128, 256)) else read_op(s2, 128)
                    if e2 is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::vpermilps_var({s1[1]}, {e2})"))
                continue
            if not dst or not imm or dst[0] != "reg" or dst[2] not in (128, 256) or imm[0] != "imm":
                return None, None
            imm_val = imm[1] & 0xff
            imm_u8 = f"0x{imm_val:x}u8"
            imm_hi_u8 = (
                f"0x{(imm_val >> 2) & 0xff:x}u8"
                if fn_name in ("vshufpd", "vblendpd")
                else (f"0x{(imm_val >> 4) & 0xff:x}u8" if fn_name == "vblendps" else imm_u8)
            )
            if is_bin:
                s1 = parse_op(ops[1]) if len(ops) == 4 else dst
                s2 = parse_op(ops[-2])
                if not s1 or not s2 or s1[0] != "reg" or s1[2] not in (128, 256):
                    return None, None
                if dst[2] == 256:
                    h1 = read_ymm_halves(s1)
                    h2 = read_ymm_halves(s2)
                    if not h1 or not h2:
                        return None, None
                    lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_name}({h1[0]}, {h2[0]}, {imm_u8})", f"crate::simd_ops::{fn_name}({h1[1]}, {h2[1]}, {imm_hi_u8})"))
                else:
                    e1 = s1[1]
                    if fn_name == "vinsertps" and s2[0] == "mem":
                        r32 = read_op(s2, 32)
                        e2 = f"(({r32}) as u128)" if r32 else None
                    else:
                        e2 = s2[1] if (s2[0] == "reg" and s2[2] in (128, 256)) else read_op(s2, 128)
                    if e2 is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_name}({e1}, {e2}, {imm_u8})"))
            else:
                src = parse_op(ops[-2])
                if not src:
                    return None, None
                if dst[2] == 256:
                    hs = read_ymm_halves(src)
                    if not hs:
                        return None, None
                    lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_name}({hs[0]}, {imm_u8})", f"crate::simd_ops::{fn_name}({hs[1]}, {imm_hi_u8})"))
                else:
                    se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 128)
                    if se is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_name}({se}, {imm_u8})"))
        elif mn in XMM_FMA_MNS:
            op_k, ord_s, ty = XMM_FMA_MNS[mn]
            if len(ops) != 3:
                return None, None
            o1, o2, o3 = parse_op(ops[0]), parse_op(ops[1]), parse_op(ops[2])
            if not o1 or not o2 or not o3 or o1[0] != "reg" or o1[2] not in (128, 256) or o2[0] != "reg" or o2[2] not in (128, 256):
                return None, None
            if op_k in ("fmaddsub", "fmsubadd"):
                if ty != "ps":
                    return None, None
                sub_even = "true" if op_k == "fmaddsub" else "false"
                if o1[2] == 256:
                    h1 = read_ymm_halves(o1)
                    h2 = read_ymm_halves(o2)
                    h3 = read_ymm_halves(o3)
                    if not h1 or not h2 or not h3:
                        return None, None
                    la, lb, lc = (h1[0], h3[0], h2[0]) if ord_s == "132" else ((h2[0], h1[0], h3[0]) if ord_s == "213" else (h2[0], h3[0], h1[0]))
                    ha, hb, hc = (h1[1], h3[1], h2[1]) if ord_s == "132" else ((h2[1], h1[1], h3[1]) if ord_s == "213" else (h2[1], h3[1], h1[1]))
                    lines.append(set_ymm(o1[1], f"crate::simd_ops::vfmaddsub_ps({la}, {lb}, {lc}, {sub_even})", f"crate::simd_ops::vfmaddsub_ps({ha}, {hb}, {hc}, {sub_even})"))
                else:
                    e1, e2 = o1[1], o2[1]
                    e3 = o3[1] if (o3[0] == "reg" and o3[2] in (128, 256)) else read_op(o3, 128)
                    if e3 is None:
                        return None, None
                    a, b, c = (e1, e3, e2) if ord_s == "132" else ((e2, e1, e3) if ord_s == "213" else (e2, e3, e1))
                    lines.append(set_xmm(o1[1], f"crate::simd_ops::vfmaddsub_ps({a}, {b}, {c}, {sub_even})"))
                continue
            neg_ab = "true" if "nm" in op_k else "false"
            neg_c = "true" if "sub" in op_k else "false"
            if ty in ("ps", "pd"):
                if o1[2] == 256:
                    h1 = read_ymm_halves(o1)
                    h2 = read_ymm_halves(o2)
                    h3 = read_ymm_halves(o3)
                    if not h1 or not h2 or not h3:
                        return None, None
                    la, lb, lc = (h1[0], h3[0], h2[0]) if ord_s == "132" else ((h2[0], h1[0], h3[0]) if ord_s == "213" else (h2[0], h3[0], h1[0]))
                    ha, hb, hc = (h1[1], h3[1], h2[1]) if ord_s == "132" else ((h2[1], h1[1], h3[1]) if ord_s == "213" else (h2[1], h3[1], h1[1]))
                    lines.append(set_ymm(o1[1], f"crate::simd_ops::vfmadd_{ty}({la}, {lb}, {lc}, {neg_ab}, {neg_c})", f"crate::simd_ops::vfmadd_{ty}({ha}, {hb}, {hc}, {neg_ab}, {neg_c})"))
                else:
                    e1, e2 = o1[1], o2[1]
                    e3 = o3[1] if (o3[0] == "reg" and o3[2] in (128, 256)) else read_op(o3, 128)
                    if e3 is None:
                        return None, None
                    a, b, c = (e1, e3, e2) if ord_s == "132" else ((e2, e1, e3) if ord_s == "213" else (e2, e3, e1))
                    lines.append(set_xmm(o1[1], f"crate::simd_ops::vfmadd_{ty}({a}, {b}, {c}, {neg_ab}, {neg_c})"))
            else:
                nb = 32 if ty == "ss" else 64
                e1 = f"({o1[1]} as u{nb})"
                e2 = f"({o2[1]} as u{nb})"
                e3 = f"({o3[1]} as u{nb})" if (o3[0] == "reg" and o3[2] in (128, 256)) else read_op(o3, nb)
                if e3 is None:
                    return None, None
                a, b, c = (e1, e3, e2) if ord_s == "132" else ((e2, e1, e3) if ord_s == "213" else (e2, e3, e1))
                lines.append(set_xmm(o1[1], f"crate::simd_ops::vfmadd_{ty}({o1[1]}, {a}, {b}, {c}, {neg_ab}, {neg_c})"))
        elif mn in XMM_MISC_MNS:
            if mn in ("vrsqrtss", "rsqrtss", "vrcpss", "rcpss"):
                dst = parse_op(ops[0])
                base_r = parse_op(ops[1]) if len(ops) == 3 else dst
                src = parse_op(ops[-1])
                if not dst or not base_r or not src or dst[0] != "reg" or base_r[0] != "reg":
                    return None, None
                se = f"({src[1]} as u32)" if (src[0] == "reg" and src[2] == 128) else read_op(src, 32)
                if se is None:
                    return None, None
                fn_n = "vrsqrtss" if "rsqrt" in mn else "vrcpss"
                lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({base_r[1]}, {se})"))
            elif mn in ("vbroadcastss", "vbroadcastsd", "vpbroadcastb", "vpbroadcastw", "vpbroadcastd", "vpbroadcastq"):
                dst, src = parse_op(ops[0]), parse_op(ops[1])
                if not dst or not src or dst[0] != "reg":
                    return None, None
                nb = 8 if mn.endswith("b") else (16 if mn.endswith("w") else (64 if mn.endswith(("q", "sd")) else 32))
                se = f"({src[1]} as u{nb})" if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, nb)
                if se is None:
                    return None, None
                if dst[2] == 256:
                    lines.append(f"let bc_v: u128 = crate::simd_ops::{mn}({se}); " + set_ymm(dst[1], "bc_v", "bc_v"))
                else:
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{mn}({se})"))
            elif mn in ("vpermpd", "vpermq"):
                if len(ops) != 3:
                    return None, None
                dst, src, imm = parse_op(ops[0]), parse_op(ops[1]), parse_op(ops[2])
                if not dst or not src or not imm or dst[0] != "reg" or dst[2] != 256 or imm[0] != "imm":
                    return None, None
                hs = read_ymm_halves(src)
                if not hs:
                    return None, None
                d_idx = dst[1][3:]
                lines.append(f"let (p_lo, p_hi) = crate::simd_ops::vpermpd({hs[0]}, {hs[1]}, 0x{imm[1] & 0xff:x}u8); {dst[1]} = p_lo; ymm{d_idx}_hi = p_hi;")
            elif mn in ("vpermd", "vpermps"):
                if len(ops) != 3:
                    return None, None
                dst, idx_op, src = parse_op(ops[0]), parse_op(ops[1]), parse_op(ops[2])
                if (
                    not dst
                    or not idx_op
                    or not src
                    or dst[0] != "reg"
                    or dst[2] != 256
                    or idx_op[0] != "reg"
                    or idx_op[2] != 256
                ):
                    return None, None
                hi_idx = read_ymm_halves(idx_op)
                hs = read_ymm_halves(src)
                if not hi_idx or not hs:
                    return None, None
                d_idx = dst[1][3:]
                lines.append(
                    f"let (p_lo, p_hi) = crate::simd_ops::vpermd({hi_idx[0]}, {hi_idx[1]}, {hs[0]}, {hs[1]}); "
                    f"{dst[1]} = p_lo; ymm{d_idx}_hi = p_hi;"
                )
            elif mn in ("vstmxcsr", "stmxcsr"):
                if len(ops) != 1:
                    return None, None
                dst = parse_op(ops[0])
                if not dst or dst[0] != "mem":
                    return None, None
                lines.append(write_op(dst, "0x1f80u32", 32))
            elif mn in ("vblendvps", "blendvps", "vblendvpd", "blendvpd", "vpblendvb", "pblendvb"):
                dst = parse_op(ops[0])
                if len(ops) == 4:
                    s1, s2, msk = parse_op(ops[1]), parse_op(ops[2]), parse_op(ops[3])
                else:
                    s1, s2 = dst, parse_op(ops[1])
                    msk = parse_op(ops[2]) if len(ops) == 3 else ("reg", "xmm0", 128, "low")
                if not dst or not s1 or not s2 or not msk or dst[0] != "reg" or s1[0] != "reg" or msk[0] != "reg":
                    return None, None
                fn_n = "vblendvps" if mn.endswith("ps") else ("vblendvpd" if mn.endswith("pd") else "vpblendvb")
                if dst[2] == 256:
                    h1 = read_ymm_halves(s1)
                    h2 = read_ymm_halves(s2)
                    hm = read_ymm_halves(msk)
                    if not h1 or not h2 or not hm:
                        return None, None
                    lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_n}({h1[0]}, {h2[0]}, {hm[0]})", f"crate::simd_ops::{fn_n}({h1[1]}, {h2[1]}, {hm[1]})"))
                else:
                    e2 = s2[1] if (s2[0] == "reg" and s2[2] in (128, 256)) else read_op(s2, 128)
                    if e2 is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({s1[1]}, {e2}, {msk[1]})"))
            elif mn in ("vmovmskps", "movmskps", "vmovmskpd", "movmskpd", "vpmovmskb", "pmovmskb"):
                dst, src = parse_op(ops[0]), parse_op(ops[1])
                if not dst or not src or dst[0] != "reg" or src[0] != "reg":
                    return None, None
                fn_n = "vmovmskps" if mn.endswith("ps") else ("vmovmskpd" if mn.endswith("pd") else "vpmovmskb")
                if src[2] == 256:
                    # A ymm source contributes its upper 128-bit lane too (ps: bits 4-7,
                    # pd: bits 2-3, pmovmskb: bits 16-31). Dropping it made SIMD find-index
                    # helpers (0x142489cc0/0x142489d30) miss matches and return -1.
                    hi_shift = {"vmovmskps": 4, "vmovmskpd": 2, "vpmovmskb": 16}[fn_n]
                    idx = src[1][3:]
                    expr = (
                        f"(crate::simd_ops::{fn_n}({src[1]}) | "
                        f"(crate::simd_ops::{fn_n}(ymm{idx}_hi) << {hi_shift}))"
                    )
                else:
                    expr = f"crate::simd_ops::{fn_n}({src[1]})"
                lines.append(write_op(dst, f"{expr} as u{dst[2]}", dst[2]))
            elif mn in ("vroundss", "roundss", "vroundsd", "roundsd"):
                dst = parse_op(ops[0])
                base_r = parse_op(ops[1]) if len(ops) == 4 else dst
                src, imm = parse_op(ops[-2]), parse_op(ops[-1])
                if not dst or not base_r or not src or not imm or dst[0] != "reg" or base_r[0] != "reg" or imm[0] != "imm":
                    return None, None
                nb = 32 if mn.endswith("ss") else 64
                se = f"({src[1]} as u{nb})" if (src[0] == "reg" and src[2] == 128) else read_op(src, nb)
                if se is None:
                    return None, None
                fn_n = "vroundss" if nb == 32 else "vroundsd"
                lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({base_r[1]}, {se}, 0x{imm[1] & 0xff:x}u8)"))
            elif mn in ("vcmpss", "cmpss", "vcmpsd", "cmpsd"):
                dst = parse_op(ops[0])
                s1 = parse_op(ops[1]) if len(ops) == 4 else dst
                s2, imm = parse_op(ops[-2]), parse_op(ops[-1])
                if not dst or not s1 or not s2 or not imm or dst[0] != "reg" or s1[0] != "reg" or imm[0] != "imm":
                    return None, None
                nb = 32 if mn.endswith("ss") else 64
                e1 = f"({s1[1]} as u{nb})"
                e2 = f"({s2[1]} as u{nb})" if (s2[0] == "reg" and s2[2] == 128) else read_op(s2, nb)
                if e2 is None:
                    return None, None
                fn_n = "vcmpss" if nb == 32 else "vcmpsd"
                lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({s1[1]}, {e1}, {e2}, 0x{imm[1] & 0xff:x}u8)"))
            elif mn in (
                "vpslld", "pslld", "vpsrld", "psrld", "vpsrad", "psrad",
                "vpsllq", "psllq", "vpsrlq", "psrlq",
                "vpsllw", "psllw", "vpsrlw", "psrlw", "vpsraw", "psraw",
            ):
                dst = parse_op(ops[0])
                s1 = parse_op(ops[1]) if len(ops) == 3 else dst
                cnt = parse_op(ops[-1])
                if not dst or not s1 or not cnt or dst[0] != "reg" or s1[0] != "reg":
                    return None, None
                ce = f"0x{cnt[1]:x}u64" if cnt[0] == "imm" else (f"({cnt[1]} as u64)" if (cnt[0] == "reg" and cnt[2] in (128, 256)) else read_op(cnt, 64))
                if ce is None:
                    return None, None
                fn_n = mn if mn.startswith("v") else f"v{mn}"
                if dst[2] == 256:
                    h1 = read_ymm_halves(s1)
                    if not h1:
                        return None, None
                    lines.append(set_ymm(dst[1], f"crate::simd_ops::{fn_n}({h1[0]}, {ce})", f"crate::simd_ops::{fn_n}({h1[1]}, {ce})"))
                else:
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({s1[1]}, {ce})"))
            elif mn in (
                "vpmovzxbw", "pmovzxbw", "vpmovsxbw", "pmovsxbw",
                "vpmovzxbd", "pmovzxbd", "vpmovsxbd", "pmovsxbd",
                "vpmovzxwd", "pmovzxwd", "vpmovsxwd", "pmovsxwd",
                "vpmovzxdq", "pmovzxdq", "vpmovsxdq", "pmovsxdq",
                "vpmovzxwq", "pmovzxwq",
            ):
                if len(ops) != 2:
                    return None, None
                dst, src = parse_op(ops[0]), parse_op(ops[1])
                if not dst or not src or dst[0] != "reg":
                    return None, None
                fn_n = mn if mn.startswith("v") else f"v{mn}"
                ratio4 = fn_n.endswith(("bd", "wq"))
                if dst[2] == 256:
                    if ratio4:
                        se = f"({src[1]} as u64)" if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 64)
                        if se is None:
                            return None, None
                        lines.append(
                            f"let pm_in: u64 = {se}; "
                            + set_ymm(
                                dst[1],
                                f"crate::simd_ops::{fn_n}(pm_in as u128)",
                                f"crate::simd_ops::{fn_n}((pm_in >> 32) as u128)",
                            )
                        )
                    else:
                        se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 128)
                        if se is None:
                            return None, None
                        lines.append(
                            f"let pm_in: u128 = {se}; "
                            + set_ymm(
                                dst[1],
                                f"crate::simd_ops::{fn_n}(pm_in)",
                                f"crate::simd_ops::{fn_n}(pm_in >> 64)",
                            )
                        )
                else:
                    nb_m = 32 if ratio4 else 64
                    se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else (f"(({read_op(src, nb_m)}) as u128)" if read_op(src, nb_m) else None)
                    if se is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::{fn_n}({se})"))
            elif mn == "vcvtps2ph":
                dst, src, imm = parse_op(ops[0]), parse_op(ops[1]), parse_op(ops[2])
                if not dst or not src or src[0] != "reg":
                    return None, None
                if src[2] == 256:
                    s_idx = src[1][3:]
                    ph_expr = f"(crate::simd_ops::vcvtps2ph({src[1]}) as u128) | ((crate::simd_ops::vcvtps2ph(ymm{s_idx}_hi) as u128) << 64)"
                    if dst[0] == "reg" and dst[2] == 128:
                        lines.append(set_xmm(dst[1], ph_expr))
                    else:
                        w_stmt = write_op(dst, ph_expr, 128)
                        if w_stmt is None:
                            return None, None
                        lines.append(w_stmt)
                elif dst[0] == "reg" and dst[2] == 128:
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::vcvtps2ph({src[1]}) as u128"))
                else:
                    w_stmt = write_op(dst, f"crate::simd_ops::vcvtps2ph({src[1]})", 64)
                    if w_stmt is None:
                        return None, None
                    lines.append(w_stmt)
            elif mn in ("vptest", "ptest"):
                if len(ops) != 2:
                    return None, None
                o1, o2 = parse_op(ops[0]), parse_op(ops[1])
                if not o1 or not o2 or o1[0] != "reg":
                    return None, None
                if o1[2] == 256:
                    h1 = read_ymm_halves(o1)
                    h2 = read_ymm_halves(o2)
                    if not h1 or not h2:
                        return None, None
                    lines.append(
                        f"let (pt_a0, pt_a1, pt_b0, pt_b1) = ({h1[0]}, {h1[1]}, {h2[0]}, {h2[1]}); "
                        "fl_l = (((pt_a0 & pt_b0) | (pt_a1 & pt_b1)) != 0) as u64; "
                        "fl_r = ((((!pt_a0) & pt_b0) | ((!pt_a1) & pt_b1)) == 0) as u64;"
                    )
                else:
                    e1 = o1[1]
                    e2 = o2[1] if (o2[0] == "reg" and o2[2] in (128, 256)) else read_op(o2, 128)
                    if e2 is None:
                        return None, None
                    lines.append(
                        f"let (pt_a, pt_b) = ({e1}, {e2}); "
                        "fl_l = ((pt_a & pt_b) != 0) as u64; "
                        "fl_r = (((!pt_a) & pt_b) == 0) as u64;"
                    )
                last_flag = ("ptest", 64, "fl_l", "fl_r")
            elif mn == "pcmpistri":
                # SSE4.2 implicit-length string compare (frontier_plan.md §2): ecx = index
                # (zero-extended), CF / ZF / SF / OF from `simd_ops::pcmpistri`, AF = PF = 0.
                if len(ops) != 3:
                    return None, None
                o1, o2 = parse_op(ops[0]), parse_op(ops[1])
                if not o1 or not o2 or o1[0] != "reg" or o1[2] != 128:
                    return None, None
                try:
                    ps_imm = int(ops[2], 0) & 0xFF
                except ValueError:
                    return None, None
                e2 = o2[1] if (o2[0] == "reg" and o2[2] == 128) else read_op(o2, 128)
                if e2 is None:
                    return None, None
                lines.append(
                    f"let (ps_i, ps_f) = crate::simd_ops::pcmpistri({o1[1]}, {e2}, 0x{ps_imm:x}); "
                    "rcx = ps_i as u64; fl_l = ps_f as u64; fl_r = 0;"
                )
                last_flag = ("pcmpstr", 64, "fl_l", "fl_r")
            elif mn == "vcvtph2ps":
                dst, src = parse_op(ops[0]), parse_op(ops[1])
                if not dst or not src or dst[0] != "reg":
                    return None, None
                if dst[2] == 256:
                    se = src[1] if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 128)
                    if se is None:
                        return None, None
                    lines.append(
                        f"let ph_in: u128 = {se}; "
                        + set_ymm(
                            dst[1],
                            "crate::simd_ops::vcvtph2ps(ph_in as u64)",
                            "crate::simd_ops::vcvtph2ps((ph_in >> 64) as u64)",
                        )
                    )
                else:
                    se = f"({src[1]} as u64)" if (src[0] == "reg" and src[2] in (128, 256)) else read_op(src, 64)
                    if se is None:
                        return None, None
                    lines.append(set_xmm(dst[1], f"crate::simd_ops::vcvtph2ps({se})"))
            else:
                return None, None
        elif mn in XMM_EXTR_MNS:
            if len(ops) != 3:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            imm = parse_op(ops[2])
            if not dst or not src or not imm or src[0] != "reg" or src[2] != 128 or imm[0] != "imm":
                return None, None
            if mn in ("vpextrq", "pextrq"):
                sh = (imm[1] & 1) * 64
                w_stmt = write_op(dst, f"(({src[1]} >> {sh}) as u64)", 64)
            elif mn in ("vpextrd", "pextrd", "vextractps", "extractps"):
                sh = (imm[1] & 3) * 32
                dbits = op_bits(dst) or 32
                w_stmt = write_op(dst, f"(({src[1]} >> {sh}) as u32) as u{dbits}", dbits)
            elif mn in ("vpextrw", "pextrw"):
                sh = (imm[1] & 7) * 16
                dbits = op_bits(dst) or 16
                w_stmt = write_op(dst, f"(({src[1]} >> {sh}) as u16) as u{dbits}", dbits)
            elif mn in ("vpextrb", "pextrb"):
                sh = (imm[1] & 15) * 8
                dbits = op_bits(dst) or 8
                w_stmt = write_op(dst, f"(({src[1]} >> {sh}) as u8) as u{dbits}", dbits)
            else:
                return None, None
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
        elif mn in XMM_INSR_MNS:
            dst = parse_op(ops[0])
            base_reg = parse_op(ops[1]) if len(ops) == 4 else dst
            src = parse_op(ops[-2])
            imm = parse_op(ops[-1])
            if not dst or not base_reg or not src or not imm or dst[0] != "reg" or dst[2] != 128 or base_reg[0] != "reg" or base_reg[2] != 128 or imm[0] != "imm":
                return None, None
            if mn in ("vpinsrq", "pinsrq"):
                sh = (imm[1] & 1) * 64
                se = read_op(src, 64)
                if se is None:
                    return None, None
                mask = f"!(0xffff_ffff_ffff_ffffu128 << {sh})"
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & {mask}) | (((({se}) as u64) as u128) << {sh})"))
            elif mn in ("vpinsrd", "pinsrd"):
                sh = (imm[1] & 3) * 32
                se = read_op(src, 32)
                if se is None:
                    return None, None
                mask = f"!(0xffff_ffffu128 << {sh})"
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & {mask}) | (((({se}) as u32) as u128) << {sh})"))
            elif mn in ("vpinsrw", "pinsrw"):
                sh = (imm[1] & 7) * 16
                sbits = op_bits(src) or 16
                se = read_op(src, sbits)
                if se is None:
                    return None, None
                mask = f"!(0xffffu128 << {sh})"
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & {mask}) | (((({se}) as u16) as u128) << {sh})"))
            elif mn in ("vpinsrb", "pinsrb"):
                sh = (imm[1] & 15) * 8
                sbits = op_bits(src) or 8
                se = read_op(src, sbits)
                if se is None:
                    return None, None
                mask = f"!(0xffu128 << {sh})"
                lines.append(set_xmm(dst[1], f"({base_reg[1]} & {mask}) | (((({se}) as u8) as u128) << {sh})"))
            else:
                return None, None
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
            if dbits is None or dbits in (128, 256):
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
            if not sbits or dbits in (128, 256):
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
            if not sbits or dbits in (128, 256):
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
            if not dst or not src or dst[0] != "reg" or src[0] != "mem" or dst[2] in (128, 256):
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
            if mn == "xor" and dst == src and dst[0] == "reg" and dst[2] not in (128, 256):
                lines.append(write_op(dst, f"0u{dst[2]}", dst[2]))
                lines.append("fl_l = 0; fl_r = 0;")
                last_flag = ("test", dst[2], f"(fl_l as u{dst[2]})", f"(fl_r as u{dst[2]})")
                continue
            dbits = op_bits(dst)
            if not dbits or dbits in (128, 256):
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
            if locked and dst[0] != "mem":
                return None, None
            if mn == "sub" and obs:
                if rmw_atomic(dst, locked, plain_too=False):
                    # lk_sub returns the new value; old = new + src gives the cmp-style flags.
                    lines.append(
                        f"let lk_s: u{dbits} = ({s_expr}) as u{dbits}; fl_r = lk_s as u64; "
                        f"fl_l = crate::simd_ops::lk_sub_u{dbits}({dst[2]}, lk_s).wrapping_add(lk_s) as u64;"
                    )
                    last_flag = ("cmp", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
                    continue
                lines.append(f"fl_l = ({d_expr}) as u{dbits} as u64; fl_r = ({s_expr}) as u{dbits} as u64;")
                w_stmt = write_op(dst, f"(fl_l as u{dbits}).wrapping_sub(fl_r as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("cmp", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
                continue
            if rmw_atomic(dst, locked):
                call = lk_rmw_call(mn, dst, dbits, s_expr)
                if obs:
                    lines.append(lk_fl_l_stmt(call, dbits) + " fl_r = fl_l;")
                    if mn in ("and", "or", "xor"):
                        last_flag = ("test", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
                    else:
                        last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
                else:
                    lines.append(call + ";")
                    last_flag = None
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
            if obs and mn == "add" and EXACT_FLAGS and any(
                c in CF_OF_SIGNED_CCS for c in flag_consumers_after(insns, idx) if c is not None
            ):
                lines.append(
                    f"let ad_s: u{dbits} = {s_expr}; fl_r = ad_s as u64; "
                    f"fl_l = ({d_expr}).wrapping_add(ad_s) as u64;"
                )
                w_stmt = write_op(dst, f"(fl_l as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("add", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
            elif obs:
                lines.append(f"fl_l = ({res}) as u{dbits} as u64; fl_r = fl_l;")
                w_stmt = write_op(dst, f"(fl_l as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                if mn in ("and", "or", "xor"):
                    last_flag = ("test", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
                else:
                    last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
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
            if not dbits or dbits in (128, 256):
                return None, None
            d_expr = read_op(dst, dbits)
            if d_expr is None:
                return None, None
            obs = flags_observed_after(insns, idx)
            if locked and (dst[0] != "mem" or mn == "neg"):
                # `lock neg [mem]` needs the old value for CF/OF; not present in the binary.
                return None, None
            if mn in ("inc", "dec") and rmw_atomic(dst, locked):
                call = lk_rmw_call("add" if mn == "inc" else "sub", dst, dbits, "1")
                if obs and not cf_only_observed_after_inc_dec(insns, idx):
                    lines.append(lk_fl_l_stmt(call, dbits))
                    last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
                else:
                    lines.append(call + ";")
                    if obs and EXACT_FLAGS and not _cf_only_in_block(insns, idx):
                        last_flag = UNK_FLAG
                continue
            if mn == "not" and rmw_atomic(dst, locked, plain_too=False):
                # `not` leaves flags untouched.
                lines.append(lk_rmw_call("xor", dst, dbits, f"0x{(1 << dbits) - 1:x}u{dbits}") + ";")
                continue
            if mn == "neg" and obs:
                lines.append(f"fl_l = 0; fl_r = ({d_expr}) as u{dbits} as u64;")
                w_stmt = write_op(dst, f"0u{dbits}.wrapping_sub(fl_r as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("cmp", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
                continue
            if mn == "inc":
                res = f"({d_expr}).wrapping_add(1)"
            elif mn == "dec":
                res = f"({d_expr}).wrapping_sub(1)"
            elif mn == "not":
                res = f"!({d_expr})"
            elif mn == "neg":
                res = f"0u{dbits}.wrapping_sub({d_expr})"
            if mn in ("inc", "dec") and obs and not cf_only_observed_after_inc_dec(insns, idx):
                lines.append(f"fl_l = ({res}) as u{dbits} as u64;")
                w_stmt = write_op(dst, f"(fl_l as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
            elif mn in ("inc", "dec") and obs and EXACT_FLAGS and not _cf_only_in_block(insns, idx):
                # inc/dec keep CF but set ZF/SF/OF: the old state is exact for the first (CF)
                # reader only. Carry CF in fl_r and the result in fl_l.
                cfc = cond_expr("b", last_flag)
                if cfc is None:
                    return None, None
                lines.append(f"let ic_cf: bool = {cfc}; fl_l = ({res}) as u{dbits} as u64; fl_r = ic_cf as u64;")
                w_stmt = write_op(dst, f"(fl_l as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("inc_cf", dbits, f"(fl_l as u{dbits})", "fl_r")
            else:
                w_stmt = write_op(dst, res, dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                if mn == "neg":
                    last_flag = None
        elif mn in ("shl", "shr", "sar", "rol", "ror"):
            if len(ops) not in (1, 2):
                return None, None
            dst = parse_op(ops[0])
            if not dst:
                return None, None
            dbits = op_bits(dst)
            if not dbits or dbits in (128, 256):
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
                # x86 masks the count to 5 bits even for r8/r16, so counts >= the width shift
                # every bit out (sar: sign fill); Rust's narrow wrapping_shl/shr mask to 3/4 bits.
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
                lines.append(f"fl_l = ({res}) as u{dbits} as u64;")
                w_stmt = write_op(dst, f"(fl_l as u{dbits})", dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
                if EXACT_FLAGS and (not sh_expr.endswith("u32") or sh_expr == "0u32"):
                    # count may be 0 (cl) or is 0: x86 leaves the flags unchanged.
                    last_flag = UNK_FLAG
            else:
                w_stmt = write_op(dst, res, dbits)
                if w_stmt is None:
                    return None, None
                lines.append(w_stmt)
                last_flag = None
        elif mn in ("shlx", "shrx", "sarx", "rorx"):
            if len(ops) != 3:
                return None, None
            dst = parse_op(ops[0])
            src1 = parse_op(ops[1])
            src2 = parse_op(ops[2])
            if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] not in (32, 64):
                return None, None
            dbits = dst[2]
            mask = 63 if dbits == 64 else 31
            e1 = read_op(src1, dbits)
            if e1 is None:
                return None, None
            if mn == "rorx":
                if src2[0] != "imm":
                    return None, None
                sh_expr = f"{src2[1] & mask}u32"
                res = f"({e1}).rotate_right({sh_expr})"
            else:
                e2 = read_op(src2, dbits)
                if e2 is None:
                    return None, None
                sh_expr = f"((({e2}) as u32) & {mask})"
                if mn == "shlx":
                    res = f"({e1}).wrapping_shl({sh_expr})"
                elif mn == "shrx":
                    res = f"({e1}).wrapping_shr({sh_expr})"
                else:
                    res = f"((({e1}) as i{dbits}).wrapping_shr({sh_expr}) as u{dbits})"
            lines.append(write_op(dst, res, dbits))
        elif mn == "shrd":
            if len(ops) != 3:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            cnt = parse_op(ops[2])
            if not dst or not src or not cnt:
                return None, None
            dbits = op_bits(dst) or op_bits(src)
            if dbits not in (32, 64):
                return None, None
            mask = 63 if dbits == 64 else 31
            d_e = read_op(dst, dbits)
            s_e = read_op(src, dbits)
            if d_e is None or s_e is None:
                return None, None
            if cnt[0] == "imm":
                sh_e = f"{cnt[1] & mask}u32"
            else:
                sh_e = f"((rcx as u32) & {mask})"
            w_s = write_op(
                dst,
                f"{{ let sh = {sh_e}; if sh == 0 {{ {d_e} }} else {{ (({d_e}) >> sh) | (({s_e}) << ({dbits} - sh)) }} }}",
                dbits,
            )
            if w_s is None:
                return None, None
            lines.append(w_s)
        elif mn in ("blsr", "blsi", "blsmsk"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg" or dst[2] not in (32, 64):
                return None, None
            dbits = dst[2]
            se = read_op(src, dbits)
            if se is None:
                return None, None
            if mn == "blsr":
                res = f"({se}) & (({se}).wrapping_sub(1))"
            elif mn == "blsi":
                res = f"({se}) & (0u{dbits}.wrapping_sub({se}))"
            else:
                res = f"({se}) ^ (({se}).wrapping_sub(1))"
            lines.append(f"fl_l = ({res}) as u{dbits} as u64;")
            lines.append(write_op(dst, f"(fl_l as u{dbits})", dbits))
            last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
        elif mn == "andn":
            if len(ops) != 3:
                return None, None
            dst = parse_op(ops[0])
            src1 = parse_op(ops[1])
            src2 = parse_op(ops[2])
            if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] not in (32, 64):
                return None, None
            dbits = dst[2]
            e1 = read_op(src1, dbits)
            e2 = read_op(src2, dbits)
            if e1 is None or e2 is None:
                return None, None
            res = f"(!({e1})) & ({e2})"
            lines.append(f"fl_l = ({res}) as u{dbits} as u64; fl_r = fl_l;")
            lines.append(write_op(dst, f"(fl_l as u{dbits})", dbits))
            last_flag = ("test", dbits, f"(fl_l as u{dbits})", f"(fl_r as u{dbits})")
        elif mn in ("tzcnt", "lzcnt", "bsf", "bsr", "popcnt"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or dst[0] != "reg" or dst[2] not in (16, 32, 64):
                return None, None
            dbits = dst[2]
            se = read_op(src, dbits)
            if se is None:
                return None, None
            if mn in ("tzcnt", "bsf"):
                res = f"(bit_in.trailing_zeros() as u{dbits})"
            elif mn == "lzcnt":
                res = f"(bit_in.leading_zeros() as u{dbits})"
            elif mn == "bsr":
                res = f"(({dbits - 1}u32.wrapping_sub(bit_in.leading_zeros())) as u{dbits})"
            else:
                res = f"(bit_in.count_ones() as u{dbits})"
            lines.append(f"let bit_in: u{dbits} = {se};")
            if mn in ("bsf", "bsr"):
                nxt = insns[idx + 1][1] if idx + 1 < len(insns) else "-"
                audit("bsf_zero" if not EXACT_FLAGS else "bsf_exact", f"{'same' if src == dst else 'diff'};next={nxt}")
            if EXACT_FLAGS and mn in ("bsf", "bsr"):
                w = write_op(dst, res, dbits)
                lines.append(f"if bit_in != 0 {{ {w} }}")
            else:
                lines.append(write_op(dst, res, dbits))
            if mn in ("bsf", "bsr"):
                lines.append("fl_l = bit_in as u64;")
            else:
                lines.append(f"fl_l = ({dst[1]} as u{dbits}) as u64;")
            last_flag = ("z_s", dbits, f"(fl_l as u{dbits})", "0")
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
            lines.append(f"fl_l = ((({e1}) >> ({b_expr})) & 1) as u64;")
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
            last_flag = ("bt", bits, f"(fl_l as u{bits})", "0")
        elif mn in ("sbb", "adc"):
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src:
                return None, None
            dbits = op_bits(dst)
            if not dbits or dbits in (128, 256):
                return None, None
            c_expr = cond_expr("b", last_flag)
            if c_expr is None:
                return None, None
            if mn == "sbb" and dst == src and dst[0] == "reg":
                mask_hex = f"0x{(1 << dbits) - 1:x}u{dbits}"
                lines.append(write_op(dst, f"if {c_expr} {{ {mask_hex} }} else {{ 0u{dbits} }}", dbits))
                if EXACT_FLAGS and flag_consumers_after(insns, idx):
                    lines.append(f"fl_l = ({read_op(dst, dbits)}) as u64;")
                    last_flag = ("cf_mask", dbits, "fl_l", "0")
                else:
                    last_flag = None
                continue
            d_expr = read_op(dst, dbits)
            s_expr = f"0x{src[1] & ((1 << dbits) - 1):x}u{dbits}" if src[0] == "imm" else read_op(src, dbits)
            if d_expr is None or s_expr is None:
                return None, None
            cf_val = f"(if {c_expr} {{ 1u{dbits} }} else {{ 0u{dbits} }})"
            if mn == "sbb":
                res = f"({d_expr}).wrapping_sub({s_expr}).wrapping_sub({cf_val})"
            else:
                res = f"({d_expr}).wrapping_add({s_expr}).wrapping_add({cf_val})"
            w_stmt = write_op(dst, res, dbits)
            if w_stmt is None:
                return None, None
            lines.append(w_stmt)
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
                if not dst or not src or dst[0] != "reg" or dst[2] in (128, 256):
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
                if not dst or not src1 or not src2 or dst[0] != "reg" or dst[2] in (128, 256):
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
                lines.append("rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64;")
            else:
                lines.append(f"let prod = (rax as u32 as u64).wrapping_mul(({s_expr}) as u32 as u64);")
                lines.append("rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64;")
            last_flag = ("mul_ovf", 64, "fl_l", "0")
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
        elif mn == "cpuid":
            lines.append(
                "let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); "
                "rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d;"
            )
            last_flag = None
        elif mn == "xgetbv":
            lines.append("rax = 7; rdx = 0;")
            last_flag = None
        elif mn in ("rdtsc", "rdtscp"):
            # The emulator's time-stamp counter, the same value on every path (frontier_plan.md
            # §3: `emu::guest_rdtsc`, GUEST_TSC); flags unchanged; rdtscp: ecx = TSC_AUX = 0.
            lines.append(
                "{ let t = crate::emu::guest_rdtsc(); rax = t & 0xffff_ffff; rdx = t >> 32; }"
                + (" rcx = 0;" if mn == "rdtscp" else "")
            )
        elif mn == "int":
            if TRAPS:
                lines.append(trap_stmt(int_trap_code(ops_str), addr, uses_stack, ops_str))
            lines.append(ret_stmt)
        elif mn == "xchg":
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2:
                return None, None
            if locked and op1[0] != "mem" and op2[0] != "mem":
                return None, None
            if op1 == op2:
                continue
            bits = op_bits(op1) or op_bits(op2)
            if bits not in (8, 16, 32, 64):
                return None, None
            e1 = read_op(op1, bits)
            e2 = read_op(op2, bits)
            if e1 is None or e2 is None:
                return None, None
            if op1[0] == "mem":
                lines.append(f"let tmp_xchg: u{bits} = crate::simd_ops::atomic_xchg_u{bits}({op1[2]}, ({e2}) as u{bits});")
                w2 = write_op(op2, "tmp_xchg", bits)
                if w2 is None:
                    return None, None
                lines.append(w2)
            elif op2[0] == "mem":
                lines.append(f"let tmp_xchg: u{bits} = crate::simd_ops::atomic_xchg_u{bits}({op2[2]}, ({e1}) as u{bits});")
                w1 = write_op(op1, "tmp_xchg", bits)
                if w1 is None:
                    return None, None
                lines.append(w1)
            else:
                lines.append(f"let tmp_xchg: u{bits} = {e1};")
                w1 = write_op(op1, e2, bits)
                w2 = write_op(op2, "tmp_xchg", bits)
                if w1 is None or w2 is None:
                    return None, None
                lines.append(w1)
                lines.append(w2)
        elif mn == "xadd":
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or src[0] != "reg":
                return None, None
            if locked and dst[0] != "mem":
                return None, None
            bits = op_bits(dst) or op_bits(src)
            if bits not in (8, 16, 32, 64):
                return None, None
            d_expr = read_op(dst, bits)
            s_expr = read_op(src, bits)
            if d_expr is None or s_expr is None:
                return None, None
            if dst[0] == "mem":
                lines.append(f"let xa_src: u{bits} = {s_expr}; let xa_old: u{bits} = crate::simd_ops::atomic_xadd_u{bits}({dst[2]}, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;")
                w2 = write_op(src, "xa_old", bits)
                if w2 is None:
                    return None, None
                lines.append(w2)
            else:
                lines.append(f"let xa_old: u{bits} = {d_expr}; let xa_src: u{bits} = {s_expr}; fl_l = xa_old.wrapping_add(xa_src) as u64;")
                w1 = write_op(dst, f"(fl_l as u{bits})", bits)
                w2 = write_op(src, "xa_old", bits)
                if w1 is None or w2 is None:
                    return None, None
                lines.append(w1)
                lines.append(w2)
            last_flag = ("z_s", bits, f"(fl_l as u{bits})", "0")
        elif mn == "cmpxchg":
            if len(ops) != 2:
                return None, None
            dst = parse_op(ops[0])
            src = parse_op(ops[1])
            if not dst or not src or src[0] != "reg":
                return None, None
            if locked and dst[0] != "mem":
                return None, None
            bits = src[2]
            if bits not in (8, 16, 32, 64):
                return None, None
            d_expr = read_op(dst, bits)
            s_expr = read_op(src, bits)
            acc_op = ("reg", "rax", bits, "low")
            a_expr = read_op(acc_op, bits)
            if d_expr is None or s_expr is None or a_expr is None:
                return None, None
            if dst[0] == "mem":
                w_acc = write_op(acc_op, f"(fl_r as u{bits})", bits)
                if w_acc is None:
                    return None, None
                lines.append(f"fl_l = ({a_expr}) as u{bits} as u64; let cx_src: u{bits} = {s_expr};")
                lines.append(f"let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u{bits}({dst[2]}, fl_l as u{bits}, cx_src); fl_r = cx_old as u64; if !cx_ok {{ {w_acc} }}")
            else:
                lines.append(f"fl_l = ({a_expr}) as u{bits} as u64; fl_r = ({d_expr}) as u{bits} as u64; let cx_src: u{bits} = {s_expr};")
                w_dst = write_op(dst, "cx_src", bits)
                w_acc = write_op(acc_op, f"(fl_r as u{bits})", bits)
                if w_dst is None or w_acc is None:
                    return None, None
                lines.append(f"if (fl_l as u{bits}) == (fl_r as u{bits}) {{ {w_dst} }} else {{ {w_acc} }}")
            last_flag = ("cmp", bits, f"(fl_l as u{bits})", f"(fl_r as u{bits})")
        elif mn in ("cmp", "test"):
            if len(ops) != 2:
                return None, None
            op1 = parse_op(ops[0])
            op2 = parse_op(ops[1])
            if not op1 or not op2:
                return None, None
            bits = op_bits(op1) or op_bits(op2)
            if not bits or bits in (128, 256):
                return None, None
            e1 = read_op(op1, bits)
            if op2[0] == "imm":
                imm_val = op2[1] & ((1 << bits) - 1)
                e2 = f"0x{imm_val:x}u{bits}"
            else:
                e2 = read_op(op2, bits)
            if e1 is None or e2 is None:
                return None, None
            lines.append(f"fl_l = ({e1}) as u{bits} as u64; fl_r = ({e2}) as u{bits} as u64;")
            last_flag = (mn, bits, f"(fl_l as u{bits})", f"(fl_r as u{bits})")
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
            if not dst or not src or dst[0] != "reg" or dst[2] in (128, 256):
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
            if target == "AW2_SETJMP":
                if EH_CUR is None:
                    return None, None
                lines.append(f"rax = eh.setjmp(0x{addr:x}, rcx, rdx, 0x{next_addr:x}, {SJ_ARGS});")
                last_flag = None
                continue
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
                if op[0] == "mem" and GUARD_FPTR_LIT:
                    fn_ptr = f"{{ let t = ({fn_ptr}) as u64; if t == {GUARD_FPTR_LIT} {{ rax }} else {{ t }} }}"
            s_args = "s5, s6, s7, s8" if uses_stack else "arg5, arg6, arg7, arg8"
            if uses_stack:
                lines.append(
                    "let s5 = crate::simd_ops::read_s5_call(rsp, &stack); "
                    "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); "
                    "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); "
                    "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);"
                )
            else:
                lines.append("crate::simd_ops::forward_incoming_to_outgoing();")
            eh_pre, caught = "", False
            if EH_CUR is not None:
                eh_pre = f"eh.site(0x{addr:x}); "
                tr = EH_CUR["try_ranges"]
                caught = tr is None or any(lo <= addr < hi or lo <= next_addr < hi for lo, hi in tr)
            if caught:
                land = eh_landing(addr_to_bb)
                if uses_xmm_call:
                    lines.append(
                        f"{eh_pre}crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x{addr:x}, ({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}, [xmm0, xmm1, xmm2, xmm3]) {{ Ok(v) => v, Err(l) => {land} }}; rax = c_rax; xmm0 = c_x0;"
                    )
                else:
                    lines.append(
                        f"{eh_pre}crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x{addr:x}, ({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}) {{ Ok(v) => v, Err(l) => {land} }};"
                    )
            elif uses_xmm_call:
                lines.append(
                    f"{eh_pre}crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;"
                )
            elif eh_mode == 2 and not EH_SITES:
                lines.append(
                    f"crate::simd_ops::replace_rax_in(rax); rax = match crate::simd_ops::call_va8_caught(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}) {{ Ok(v) => v, Err(e) => e.rethrow() }};"
                )
            else:
                lines.append(
                    f"{eh_pre}crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args});"
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
                if op[0] == "mem" and GUARD_FPTR_LIT:
                    fn_ptr = f"{{ let t = ({fn_ptr}) as u64; if t == {GUARD_FPTR_LIT} {{ rax }} else {{ t }} }}"
            s_args = "s5, s6, s7, s8" if uses_stack else "arg5, arg6, arg7, arg8"
            if uses_stack:
                prep_s = (
                    "let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); "
                    "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); "
                    "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); "
                    "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);"
                )
            else:
                prep_s = "crate::simd_ops::forward_incoming_to_outgoing();"
            if uses_xmm_call or uses_xmm_out:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;"
            elif eh_mode == 2 and not EH_SITES:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let r = match crate::simd_ops::call_va8_caught(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}) {{ Ok(v) => v, Err(e) => e.rethrow() }}; return r;"
            else:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args});"
            if addr_to_bb is not None and not target.startswith("RIP_P_") and op[0] == "reg" and len(addr_to_bb) > 1:
                lines.append(
                    f"j_tgt = ({fn_ptr}) as u64; bb = 0xffff_fffe; continue;"
                )
            else:
                lines.append(tail_stmt)
        elif mn in (
            "rep_movsq",
            "rep_movsb",
            "rep_stosq",
            "rep_stosd",
            "rep_stosw",
            "rep_stosb",
            "repne_scasw",
            "repne_scasb",
            "rep_cmpsw",
        ):
            if mn == "rep_movsq":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u64, core::ptr::read_unaligned(rsi as *const u64)); rdi = rdi.wrapping_add(8); rsi = rsi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }")
            elif mn == "rep_movsb":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u8, core::ptr::read_unaligned(rsi as *const u8)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }")
            elif mn == "rep_stosq":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u64, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }")
            elif mn == "rep_stosd":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u32, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); }")
            elif mn == "rep_stosw":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u16, rax as u16); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); }")
            elif mn == "rep_stosb":
                lines.append("while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u8, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }")
            elif mn == "repne_scasw":
                lines.append("while rcx != 0 { let v = core::ptr::read_unaligned(rdi as *const u16); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } }")
            elif mn == "repne_scasb":
                lines.append("while rcx != 0 { let v = core::ptr::read_unaligned(rdi as *const u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); if v == (rax as u8) { break; } }")
            elif mn == "rep_cmpsw":
                lines.append("while rcx != 0 { let a = core::ptr::read_unaligned(rsi as *const u16); let b = core::ptr::read_unaligned(rdi as *const u16); rsi = rsi.wrapping_add(2); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); fl_l = a as u64; fl_r = b as u64; if a != b { break; } }")
                last_flag = ("cmp", 16, "(fl_l as u16)", "(fl_r as u16)")
        elif mn.startswith("j") and ops_str.strip().startswith("EXT_P_"):
            cc = mn[1:]
            p_idx = int(ops_str.strip().split("_")[2])
            fn_ptr = f"P{p_idx}"
            c_expr = cond_expr(cc, last_flag)
            if c_expr is None:
                return None, None
            s_args = "s5, s6, s7, s8" if uses_stack else "arg5, arg6, arg7, arg8"
            if uses_stack:
                prep_s = (
                    "let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); "
                    "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); "
                    "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); "
                    "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);"
                )
            else:
                prep_s = "crate::simd_ops::forward_incoming_to_outgoing();"
            if uses_xmm_call or uses_xmm_out:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;"
            elif eh_mode == 2 and not EH_SITES:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let r = match crate::simd_ops::call_va8_caught(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args}) {{ Ok(v) => v, Err(e) => e.rethrow() }}; return r;"
            else:
                tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({fn_ptr}) as u64, rcx, rdx, r8, r9, {s_args});"
            lines.append(f"if {c_expr} {{ {tail_stmt} }}")
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
    if guard_at is not None and not seh_guard_lines(lines, *guard_at, addr_to_bb, last_flag):
        return None, None
    if prev is not None:
        last_flag = _settle_flag(prev, last_flag)
    return lines, last_flag


RAX_WRITE_RE = re.compile(r"^\s*(rax|eax|ax|al|ah)\b", re.IGNORECASE)


def analyze_xmm_usage(processed):
    if len(processed) == 1 and processed[0][1] == "jmp":
        return True, True, True, True
    has_xmm = any(
        mn in ALLOWED_V_MNS or XYMM_WORD_RE.search(ops)
        for _, mn, ops, _ in processed
    )
    if not has_xmm:
        return True, False, False, False

    for _, mn, ops, _ in processed:
        if mn in ("vzeroupper", "nop", "pause", "lock", "cld", "lfence", "sfence", "mfence"):
            continue
        if XYMM_WORD_RE.search(ops) and mn not in ALL_XMM_MNS:
            return False, False, False, False

    uses_xmm_in = any(
        re.search(r"\b[xy]mm[0-3]\b", ops, re.I)
        or mn in ("blendvps", "blendvpd", "pblendvb")
        for _, mn, ops, _ in processed
    )
    uses_xmm_out = any(
        re.search(r"\b[xy]mm0\b", ops, re.I) for _, _, ops, _ in processed
    )
    uses_xmm_call = uses_xmm_in and any(
        mn == "call"
        or (mn == "jmp" and (ops.startswith("RIP_P_") or not ops.startswith("0x")))
        or (mn.startswith("j") and ops.startswith("EXT_P_"))
        for _, mn, ops, _ in processed
    )
    # A body that tracks xmm0..3 across calls (`call_va8_xmm`, `xmm0 = c_x0`) returns the
    # callee's xmm0 when it does not write xmm0 itself afterwards (`double f() { double r =
    # g(); ...int-only epilogue...; return r; }`): x86 xmm0 at `ret` is its last write, so
    # publish it on every return. `call_va8_xmm` consumes the callee's published value, so
    # without this the caller of such a body reads a stale xmm0.
    if uses_xmm_call and uses_xmm_in:
        uses_xmm_out = True
    return True, uses_xmm_in, uses_xmm_out, uses_xmm_call


def analyze_stack_usage(processed):
    uses_stack = any(
        mn in ("push", "pop") or g.RSP_WORD_RE.search(ops)
        for _, mn, ops, _ in processed
    )
    if not uses_stack:
        return True, False
    total_sub = 0
    for _, mn, ops, _ in processed:
        if mn == "push":
            total_sub += 8
        elif mn == "sub" and ops.lower().startswith("rsp,"):
            parts = g.split_operands(ops)
            if len(parts) == 2:
                try:
                    total_sub += max(0, int(parts[1], 0))
                except ValueError:
                    total_sub += 256
    if total_sub > 524288:
        return False, False
    return True, True


def translate_block(
    insns,
    initial_flag=None,
    depth: int = 0,
    uses_stack: bool = False,
    uses_xmm_out: bool = False,
    uses_xmm_call: bool = False,
    eh_mode: int = 0,
):
    if depth > 6 or not insns:
        return None

    branches = [
        (i, item)
        for i, item in enumerate(insns)
        if g.is_internal_branch(item[1], item[2])
    ]
    if not branches:
        lines, _ = translate_linear_insns(
            insns, initial_flag, None, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
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

    if t_idx <= j_idx:
        before_insns = insns[:t_idx]
        loop_insns = insns[t_idx:j_idx]
        after_insns = insns[j_idx + 1 :]
        if not after_insns:
            return None
        loop_addrs = {x[0] for x in loop_insns}
        if any(
            g.is_internal_branch(m, o) and int(o.strip().split()[0], 16) in loop_addrs
            for _, m, o, _ in after_insns
        ):
            return None
        before_lines, _ = translate_linear_insns(
            before_insns, initial_flag, None, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        if before_lines is None:
            return None
        loop_lines, flag_loop = translate_linear_insns(
            loop_insns, None, None, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        if loop_lines is None or flag_loop is None:
            return None
        c_expr = cond_expr(cc, flag_loop)
        if c_expr is None:
            return None
        after_lines = translate_block(
            after_insns, flag_loop, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        if (
            after_lines is None
            or not after_lines
            or ("return " not in after_lines[-1])
        ):
            return None
        combined = list(before_lines)
        combined.append(f"loop {{ {' '.join(loop_lines)} if !({c_expr}) {{ break; }} }}")
        combined.extend(after_lines)
        return combined

    before_insns = insns[:j_idx]
    before_lines, flag_after_before = translate_linear_insns(
        before_insns, initial_flag, None, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
    )
    if before_lines is None or flag_after_before is None:
        return None
    c_expr = cond_expr(cc, flag_after_before)
    if c_expr is None:
        return None

    min_after_addr = insns[j_idx + 1][0] if j_idx + 1 < len(insns) else t_va
    for i in range(j_idx + 1, len(insns)):
        _, mn2, ops2, _ = insns[i]
        if g.is_internal_branch(mn2, ops2):
            try:
                bt = int(ops2.strip().split()[0], 16)
                if bt < min_after_addr:
                    return None
            except ValueError:
                return None

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
                if g.is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) not in then_addrs:
                        cross_bad = True
            for _, mn2, ops2, _ in else_insns:
                if g.is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) not in else_addrs:
                        cross_bad = True
            for _, mn2, ops2, _ in after_insns:
                if g.is_internal_branch(mn2, ops2):
                    if int(ops2.strip().split()[0], 16) in (then_addrs | else_addrs):
                        cross_bad = True
            if not cross_bad:
                then_lines = translate_block(
                    then_insns, flag_after_before, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
                )
                else_lines = translate_block(
                    else_insns, flag_after_before, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
                )
                after_lines = translate_block(
                    after_insns, None, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
                )
                if (
                    then_lines is not None
                    and else_lines is not None
                    and after_lines is not None
                    and after_lines
                    and ("return " in after_lines[-1])
                ):
                    combined = list(before_lines)
                    combined.append(f"if !({c_expr}) {{ {' '.join(then_lines)} }} else {{ {' '.join(else_lines)} }}")
                    combined.extend(after_lines)
                    return combined

    fall_insns = insns[j_idx + 1 : t_idx]
    after_insns = insns[t_idx:]
    fall_addrs = {x[0] for x in fall_insns}
    after_addrs = {x[0] for x in after_insns}
    isolated = True
    for _, mn2, ops2, _ in fall_insns:
        if g.is_internal_branch(mn2, ops2):
            if int(ops2.strip().split()[0], 16) not in fall_addrs:
                isolated = False
    for _, mn2, ops2, _ in after_insns:
        if g.is_internal_branch(mn2, ops2):
            if int(ops2.strip().split()[0], 16) not in after_addrs:
                isolated = False
    if isolated:
        fall_lines = translate_block(
            fall_insns, flag_after_before, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        after_flag = (
            flag_after_before
            if not any(m in FLAG_CLOBBER_MNS for _, m, _, _ in fall_insns)
            else None
        )
        after_lines = translate_block(
            after_insns, after_flag, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        if (
            fall_lines is not None
            and after_lines is not None
            and after_lines
            and ("return " in after_lines[-1])
        ):
            combined = list(before_lines)
            combined.append(f"if !({c_expr}) {{ {' '.join(fall_lines)} }}")
            combined.extend(after_lines)
            return combined

    if not any(g.is_internal_branch(m, o) for _, m, o, _ in after_insns):
        strict_tail_addrs = {x[0] for x in after_insns[1:]}
        if not any(
            g.is_internal_branch(m, o)
            and int(o.strip().split()[0], 16) in strict_tail_addrs
            for _, m, o, _ in insns[j_idx + 1 : t_idx]
        ):
            tail_lines, _ = translate_linear_insns(
                after_insns, flag_after_before, None, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
            )
            if (
                tail_lines is not None
                and tail_lines
                and ("return " in tail_lines[-1])
            ):
                rest_lines = translate_block(
                    insns[j_idx + 1 :], flag_after_before, depth + 1, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
                )
                if (
                    rest_lines is not None
                    and rest_lines
                    and ("return " in rest_lines[-1])
                ):
                    combined = list(before_lines)
                    combined.append(f"if {c_expr} {{ {' '.join(tail_lines)} }}")
                    combined.extend(rest_lines)
                    return combined

    return None


def translate_cfg(
    processed,
    uses_stack: bool = False,
    uses_xmm_out: bool = False,
    uses_xmm_call: bool = False,
    eh_mode: int = 0,
    extra_block_starts=None,
):
    if not processed:
        return None
    all_addrs = {addr for addr, _, _, _ in processed}
    block_starts = {processed[0][0]}
    if extra_block_starts:
        for bva in extra_block_starts:
            if bva in all_addrs:
                block_starts.add(bva)
    for addr, mn, ops, next_addr in processed:
        if g.is_internal_branch(mn, ops) and not ops.strip().startswith("EXT_P_"):
            try:
                t_va = int(ops.strip().split()[0], 16)
            except ValueError:
                return _tfail(f"cfg: non-literal branch target at 0x{addr:x}: {mn} {ops}")
            if t_va not in all_addrs:
                return _tfail(f"cfg: branch target outside fn at 0x{addr:x}: {mn} 0x{t_va:x}")
            block_starts.add(t_va)
            if next_addr in all_addrs:
                block_starts.add(next_addr)
        elif mn in ("ret", "jmp", "int3", "ud2"):
            if next_addr in all_addrs:
                block_starts.add(next_addr)

    sorted_starts = sorted(block_starts)
    if len(sorted_starts) > 2048:
        return _tfail(f"cfg: {len(sorted_starts)} blocks > 2048")
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

    # Lazy-flag dataflow. A block's in-flag is the join of its predecessors' out-flags. When
    # predecessors disagree (e.g. a loop head reached from `test rdx, rdx; jmp` and from the
    # latch's `sub rdx, 1`), the join is a ("multi", 0, members, None) flag: every edge into such
    # a block records which concrete producer is live in `fl_k`, and conditions dispatch on it.
    # (Previously the first predecessor seen won, so the latch's `jne` was evaluated with TEST
    # semantics on SUB operands and loops exited early.)
    succs_of = []
    for i, blk in enumerate(blocks):
        last_mn = blk[-1][1]
        last_ops = blk[-1][2]
        succs = []
        if g.is_internal_branch(last_mn, last_ops) and not last_ops.strip().startswith("EXT_P_"):
            t_va = int(last_ops.strip().split()[0], 16)
            if t_va in addr_to_bb:
                succs.append(addr_to_bb[t_va])
        if last_mn not in ("ret", "jmp", "int3", "ud2") and i + 1 < len(blocks):
            succs.append(i + 1)
        succs_of.append(succs)

    in_flags = [None] * len(blocks)
    out_flags = [None] * len(blocks)
    global IN_DATAFLOW
    IN_DATAFLOW = True
    for _ in range(64):
        changed = False
        for i, blk in enumerate(blocks):
            _, out_f = translate_linear_insns(
                blk, in_flags[i], addr_to_bb, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
            )
            out_flags[i] = out_f
        contrib = [[f] for f in in_flags]
        for i in range(len(blocks)):
            if out_flags[i] is not None:
                for s in succs_of[i]:
                    contrib[s].append(out_flags[i])
        for s in range(len(blocks)):
            nf = join_flags(contrib[s])
            if nf != in_flags[s]:
                in_flags[s] = nf
                changed = True
        if not changed:
            break
    IN_DATAFLOW = False

    multi_members = set()
    for f in in_flags:
        if f is not None and f[0] == "multi":
            multi_members.update(f[2])
    FLAG_IDS.clear()
    for k, m in enumerate(sorted(multi_members, key=repr)):
        FLAG_IDS[m] = k

    ret_stmt = "crate::simd_ops::set_xmm_ret(xmm0); return rax;" if uses_xmm_out else "return rax;"
    lines = ["let mut bb: u32 = 0;", "loop {", "    match bb {"]
    if FLAG_IDS:
        lines.insert(0, "let mut fl_k: u32 = 0;")
    for i, blk in enumerate(blocks):
        b_lines, _ = translate_linear_insns(
            blk, in_flags[i], addr_to_bb, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
        if b_lines is None:
            FLAG_IDS.clear()
            return _tfail(_insn_fail_reason(blk))
        last_mn = blk[-1][1]
        if last_mn not in ("ret", "jmp", "int3", "ud2"):
            if i + 1 < len(blocks):
                b_lines.append(f"bb = {i + 1}; continue;")
            else:
                b_lines.append(ret_stmt)
        out_f = out_flags[i]
        if FLAG_IDS and out_f is not None and out_f[0] != "multi" and out_f in FLAG_IDS:
            for s in set(succs_of[i]):
                sf = in_flags[s]
                if sf is not None and sf[0] == "multi":
                    b_lines = [
                        l.replace(f"bb = {s}; continue;", f"fl_k = {FLAG_IDS[out_f]}; bb = {s}; continue;")
                        for l in b_lines
                    ]
        lines.append(f"        {i} => {{ {' '.join(b_lines)} }}")
    if FLAG_IDS and not any("(match fl_k" in l for l in lines):
        # Conflicting joins whose flags are never consumed: drop the fl_k bookkeeping.
        lines = [re.sub(r"fl_k = \d+; ", "", l) for l in lines if l != "let mut fl_k: u32 = 0;"]
    FLAG_IDS.clear()
    if any("0xffff_fffe" in l for l in lines):
        base_va = min(addr_to_bb.keys())
        arms = " ".join(
            f"0x{b_va - base_va:x} => {{ bb = {b_idx}; continue; }}"
            for b_va, b_idx in sorted(addr_to_bb.items())
            if b_idx > 0 and b_va >= base_va
        )
        if uses_stack:
            prep_s = (
                "let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); "
                "let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); "
                "let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); "
                "let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);"
            )
            s_args = "s5, s6, s7, s8"
        else:
            prep_s = "crate::simd_ops::forward_incoming_to_outgoing();"
            s_args = "arg5, arg6, arg7, arg8"
        if uses_xmm_call or uses_xmm_out:
            tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, {s_args}, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;"
        elif eh_mode == 2 and not EH_SITES:
            tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); let r = match crate::simd_ops::call_va8_caught(j_tgt, rcx, rdx, r8, r9, {s_args}) {{ Ok(v) => v, Err(e) => e.rethrow() }}; return r;"
        else:
            tail_stmt = f"{eh_leave()}{prep_s} crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, {s_args});"
        lines.append(
            f"        0xffff_fffe => {{ match j_tgt.wrapping_sub(0x{base_va:x}u64) {{ {arms} _ => {{ {tail_stmt} }} }} }}"
        )
    lines.append(f"        _ => {{ {ret_stmt} }}")
    lines.append("    }")
    lines.append("}")
    return lines


def strip_unused_ymm_hi(lines: list[str]) -> list[str]:
    joined = "\n".join(lines)
    unused_hi = []
    for k in range(16):
        hi_var = f"ymm{k}_hi"
        if hi_var in joined:
            cleaned = joined.replace(f" {hi_var} = 0;", "")
            if hi_var not in cleaned:
                unused_hi.append(f" {hi_var} = 0;")
    if not unused_hi:
        return lines
    out = []
    for l in lines:
        for u in unused_hi:
            l = l.replace(u, "")
        out.append(l)
    return out


JT_DISP_RE = re.compile(r"(?:4\s*\*\s*[a-z0-9]+|[a-z0-9]+\s*\*\s*4)\s*\+\s*(0x[0-9a-fA-F]{6,})", re.I)
NONSTACK_DISP_RE = re.compile(
    r"\[(?![^\]]*\b(?:rsp|rbp|r11|rip)\b)([^\]]+?)\s*([+-])\s*(0x[0-9a-fA-F]+|\d+)\s*\]",
    re.I,
)
CMP_PREDS = {
    "eq": 0, "lt": 1, "le": 2, "unord": 3, "neq": 4, "nlt": 5, "nle": 6, "ord": 7,
    "eq_uq": 8, "nge": 9, "ngt": 10, "false": 11, "neq_oq": 12, "ge": 13, "gt": 14, "true": 15,
    "eq_os": 16, "lt_oq": 17, "le_oq": 18, "unord_s": 19, "neq_us": 20, "nlt_uq": 21, "nle_uq": 22, "ord_s": 23,
    "eq_us": 24, "nge_uq": 25, "ngt_uq": 26, "false_os": 27, "neq_os": 28, "ge_oq": 29, "gt_oq": 30, "true_us": 31,
}
CMP_PSEUDO_RE = re.compile(r"^(v?)cmp([a-z_]+?)(ps|pd|ss|sd)$")


def norm_objdump_mn_ops(mn: str, ops: str):
    m = CMP_PSEUDO_RE.match(mn)
    if m:
        v_pfx, pred, suf = m.group(1), m.group(2), m.group(3)
        if pred in CMP_PREDS:
            return f"{v_pfx}cmp{suf}", f"{ops}, 0x{CMP_PREDS[pred]:x}"
    if mn == "vcmplt" and ops.startswith("_oqps"):
        return "vcmpps", f"{ops[5:].strip()}, 0x11"
    if (
        mn.startswith("prefetch")
        or mn in (
            "vldmxcsr", "ldmxcsr", "clc", "stc", "cmc",
            "std", "sti", "cli", "pushfq", "popfq",
            "in", "out", "fdiv", "xlatb", "lodsb", "<unknown>",
        )
    ):
        return "nop", ""
    return mn, ops


# Per-function stack-argument windows: `// stackargs: A=<A> max=<max slot>` body line, written by
# the main loop (dll_lift.stack_window) and read by format_aot_fn. The window rules and the
# residual never-lift limits are described at dll_lift.STACK_ARG_SLOT_LIMIT.
STACKARGS_RE = re.compile(r"^// stackargs: A=(\d+) max=(-?\d+)$")


class _SegDis:
    """`dll_lift.stack_arg_info` view of one function's decoded segments (VAs)."""

    def __init__(self, seg_raws):
        import dll_lift

        self.recs = []
        for _s_va, _s_len, s_raw in seg_raws:
            for r_va, r_mn, r_ops in s_raw:
                ops = re.sub(r"\s*<[^>]*>", "", r_ops).strip()
                parts = [q.strip() for q in re.split(r",(?![^\[]*\])", ops)] if ops else []
                parts = [dll_lift._hex_target(q, 0) if re.fullmatch(r"0x[0-9a-f]+", q) and int(q, 16) >= IMAGE_BASE else q for q in parts]
                self.recs.append((r_va, r_mn, ops, parts))
        self.recs.sort()

    def range(self, b, e):
        return [r for r in self.recs if b <= r[0] < e]


def stack_arg_info_segs(seg_raws, fn_va):
    import dll_lift

    frags = sorted(((s_va, s_va + s_len) for s_va, s_len, _ in seg_raws), key=lambda f: (f[0] != fn_va, f[0]))
    return dll_lift.stack_arg_info(_SegDis(seg_raws), frags)


def translate_function(raw_insns, fn_va: int, fn_size: int, eh_mode: int = 0, pe=None):
    params = []
    param_idx_map = {}

    def add_param(val: int) -> int:
        if val in param_idx_map:
            return param_idx_map[val]
        idx = len(params)
        params.append(val)
        param_idx_map[val] = idx
        return idx

    all_addrs = {addr for addr, _, _, _ in raw_insns}
    AUDIT_CTX[0] = fn_va
    if AUDIT is not None:
        AUDIT[fn_va] = set()
    insns = []
    for addr, mn, ops, next_va in raw_insns:
        if mn == "nop":
            insns.append((addr, mn, "", next_va))
        else:
            insns.append((addr, mn, ops, next_va))

    extra_block_starts = set()
    has_ind_jmp = False
    if pe is not None:
        import struct as _st
        for _, mn, ops, _ in insns:
            if mn == "jmp" and ops.strip() in REG_MAP:
                has_ind_jmp = True
            for m_jt in JT_DISP_RE.finditer(ops):
                disp = int(m_jt.group(1), 16)
                jt_va = IMAGE_BASE + disp
                jt_pre = (READ_TABLE or read_frag_va)(pe, jt_va - 32, 32)
                if jt_pre:
                    for k in range(0, 32, 4):
                        rva = _st.unpack("<I", jt_pre[k : k + 4])[0]
                        t_va = IMAGE_BASE + rva
                        if t_va in all_addrs:
                            extra_block_starts.add(t_va)
                jt_bytes = (READ_TABLE or read_frag_va)(pe, jt_va, 512)
                if jt_bytes:
                    for k in range(0, len(jt_bytes) - 3, 4):
                        rva = _st.unpack("<I", jt_bytes[k : k + 4])[0]
                        t_va = IMAGE_BASE + rva
                        if t_va in all_addrs:
                            extra_block_starts.add(t_va)
                        elif k > 0:
                            break

    if EH_CUR is not None:
        # Catch continuations (eh_landing): resumption points of a landing, so block starts.
        extra_block_starts.update(c for c in EH_CUR["conts"] if c in all_addrs)

    param_disps = len(insns) <= 64 and not extra_block_starts and not has_ind_jmp
    n_trap_params = 0  # trap-site params do not count against the displacement budget
    processed = []
    sj_sites = set(EH_CUR.get("sj_sites", ())) if EH_CUR is not None else set()
    for i, (addr, mn, ops, next_addr) in enumerate(insns):
        if addr in sj_sites and mn == "call":
            processed.append((addr, mn, "AW2_SETJMP", next_addr))
            continue
        if "rip" in ops:
            m = g.RIP_RE.search(ops)
            if not m:
                return _tfail(f"unparsed rip operand at 0x{addr:x}: {mn} {ops}")
            sign = 1 if m.group(1) == "+" else -1
            disp = int(m.group(2), 0) * sign
            target_va = (next_addr + disp) & 0xFFFFFFFFFFFFFFFF
            p_idx = add_param(target_va)
            ops = g.RIP_RE.sub(f"RIP_P_{p_idx}", ops)
        elif param_disps and "[" in ops and len(params) - n_trap_params < 12:
            m_d = NONSTACK_DISP_RE.search(ops)
            if m_d:
                base_part, sign_s, disp_s = m_d.group(1), m_d.group(2), m_d.group(3)
                d_val = int(disp_s, 0)
                if d_val != 0:
                    signed_val = (d_val if sign_s == "+" else -d_val) & 0xFFFFFFFFFFFFFFFF
                    p_idx = add_param(signed_val)
                    ops = (
                        ops[: m_d.start()]
                        + f"[{base_part} + RIP_P_{p_idx}]"
                        + ops[m_d.end() :]
                    )
        if mn == "jmp":
            t_str = ops.strip().split()[0] if ops.strip() else ""
            try:
                t_va = int(t_str, 16)
                if t_va not in all_addrs:
                    t_va = CALL_REDIRECT.get(t_va, t_va)
                    if t_va in GUARD_DISPATCH_VAS:
                        ops = "rax"
                    else:
                        p_idx = add_param(t_va)
                        ops = f"RIP_P_{p_idx}"
            except ValueError:
                pass
        elif mn.startswith("j") and mn != "jmp":
            t_str = ops.strip().split()[0] if ops.strip() else ""
            try:
                t_va = int(t_str, 16)
                if t_va not in all_addrs:
                    p_idx = add_param(CALL_REDIRECT.get(t_va, t_va))
                    ops = f"EXT_P_{p_idx}"
            except ValueError:
                pass
        elif mn == "call":
            t_str = ops.strip().split()[0] if ops.strip() else ""
            try:
                t_va = CALL_REDIRECT.get(int(t_str, 16), int(t_str, 16))
                if t_va in NOP_CALL_VAS:
                    mn, ops = "nop", ""
                elif t_va in GUARD_DISPATCH_VAS:
                    ops = "rax"
                else:
                    p_idx = add_param(t_va)
                    ops = f"RIP_P_{p_idx}"
            except ValueError:
                pass
        elif mn in ("int3", "ud2", "int") and TRAPS:
            # Trap site (see trap_stmt): a parameter, so template instances still share bodies.
            p_idx = add_param(addr)
            n_trap_params += 1
            ops = f"{ops} TRAP_P_{p_idx}".strip()
        processed.append((addr, mn, ops, next_addr))

    xmm_ok, uses_xmm_in, uses_xmm_out, uses_xmm_call = analyze_xmm_usage(processed)
    if not xmm_ok:
        return _tfail("xmm usage analysis failed")
    stack_ok, uses_stack = analyze_stack_usage(processed)
    if not stack_ok:
        return _tfail("stack usage analysis failed")

    lines = None
    tail_trap = bool(processed) and processed[-1][1] == "int3" and processed[-1][0] in TAIL_TRAP_VAS
    if len(processed) <= 32 and not extra_block_starts and not has_ind_jmp and not tail_trap:
        lines = translate_block(
            processed, None, 0, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode
        )
    if lines is None or not lines or ("return " not in lines[-1]):
        lines = translate_cfg(
            processed, uses_stack, uses_xmm_out, uses_xmm_call, eh_mode, extra_block_starts
        )
    if lines is None:
        return _tfail(TRANSLATE_FAIL[0] or "cfg translation failed")
    if EH_CUR is not None:
        # The frame's EH_LIFTED record (T3), linked at entry rsp until the fn returns;
        # `EH_META_SELF` is this fn's `EH_META` entry (dll_lift `emit`).
        lines = ["let mut eh = crate::dll_native::lift::eh::EhFrame::new(EH_META_SELF, rsp); let _eh = eh.push();"] + lines
        lines = sj_fill(lines)
    if uses_xmm_in:
        lines = ["let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();"] + lines
    lines = strip_unused_ymm_hi(lines)
    return (lines, params)


def fast_eh_info_exact(pe, va: int) -> str:
    return g.canthrow.eh_info(pe, va)


def read_frag_va(pe, va: int, size: int):
    rva = va - pe.image_base
    off = rva - pe.text_rva
    if off < 0 or size <= 0 or off + size > len(pe.text):
        return None
    return pe.text[off : off + size]


def extract_cfg_meta(va: int, sz: int, b: bytes, eh: str, md: int):
    catchable = (md & 21) != 0
    contract = (md & 2) != 0
    if eh in ("catch", "cleanup") and catchable:
        eh_mode = 2
    elif catchable or contract:
        eh_mode = 1
    else:
        eh_mode = 0
    return (va, sz, 0x28, eh_mode, 0, 0, 0, 0)


IDENT_RE = re.compile(r"[a-zA-Z_][a-zA-Z0-9_]*")


def format_aot_fn(
    fn_name: str,
    generics: str,
    body_lines: list[str],
    inline_params: list[int] | None = None,
) -> str:
    body_text = "\n".join(body_lines)
    words = set(IDENT_RE.findall(body_text))
    has_rsp = "rsp" in words
    has_arg5 = "arg5" in words
    has_take_xmm = "take_xmm_args" in words
    sig_args = (
        "mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64"
        if (has_rsp or has_arg5)
        else "mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64"
    )
    out = [
        "#[inline(never)]",
        f'pub unsafe extern "win64" fn {fn_name}{generics}({sig_args}) -> u64 {{',
    ]
    if inline_params:
        consts_str = " ".join(
            f"const P{i}: u64 = 0x{v:x};" for i, v in enumerate(inline_params)
        )
        out.append(f"    {consts_str}")

    gpr_vars = ["rax"]
    for extra_r in ("r10", "r11", "fl_l", "fl_r", "j_tgt") + tuple(g.CALLEE_SAVED_PARENTS):
        if extra_r in words:
            gpr_vars.append(extra_r)
    if len(gpr_vars) == 1:
        out.append("    let mut rax: u64 = crate::simd_ops::get_rax_in();")
    else:
        lhs = ", ".join(f"mut {r}" for r in gpr_vars)
        rhs = ", ".join(["crate::simd_ops::get_rax_in()"] + ["0u64" for _ in gpr_vars[1:]])
        out.append(f"    let ({lhs}) = ({rhs});")

    xmm_decls = []
    for k in range(16):
        xm_reg = f"xmm{k}"
        ym_hi = f"ymm{k}_hi"
        if not (has_take_xmm and k < 4) and xm_reg in words:
            xmm_decls.append(f"let mut {xm_reg}: u128 = 0;")
        if ym_hi in words:
            xmm_decls.append(f"let mut {ym_hi}: u128 = 0;")
    if xmm_decls:
        out.append("    " + " ".join(xmm_decls))

    if has_rsp:
        sub_bytes = sum(
            int(m_s.group(1), 16)
            for m_s in re.finditer(r"\(rsp\)\.wrapping_sub\(0x([0-9a-fA-F]+)u64\)", body_text)
        )
        push_bytes = body_text.count("rsp = rsp.wrapping_sub(8);") * 8
        need_words = ((sub_bytes + push_bytes) // 8) + 64
        if re.search(r"\(rsp\)\.wrapping_sub\(r[a-z0-9]+\)", body_text):
            need_words = max(need_words, 4096)
        stack_words = 256
        while stack_words < need_words:
            stack_words *= 2
        if stack_words >= 4096:
            # Big frames live in the TLS arena; the local is only a host-address marker.
            stack_words = 1
        # Argument window (see STACKARGS_RE); the default 24 is the runtime's defaulted `A`.
        win = ""
        for bl in body_lines[:4]:
            m_w = STACKARGS_RE.match(bl)
            if m_w:
                a_w, max_w = int(m_w.group(1)), int(m_w.group(2))
                assert a_w % 2 == 0 and 24 < a_w and max_w < a_w, f"{fn_name}: bad stack-arg window {bl}"
                win = f", {a_w}"
                break
        if stack_words == 1:
            decl = f"crate::simd_ops::LiftedStack::<{stack_words}{win}>::new();"
        else:
            # Clean per-thread arena frame (crates/aw2-game/src/lstack.rs): the second
            # parameter is the static frame in qwords (zeroed window = frame + slack).
            frame_words = (sub_bytes + push_bytes) // 8
            decl = f"crate::simd_ops::LiftedFrame::<{stack_words}, {frame_words}{win}>::new();"
        out.append(
            f"    let mut stack = {decl} let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);"
        )
    for bl in body_lines:
        out.append(f"    {bl}")
    out.append("}\n")
    return "\n".join(out)


def emit_shard_file(
    out_path: Path,
    header_comment: str,
    templates: dict,
    entries: list,
):
    allow_hdr = "#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]\n"
    entries_sorted = sorted(entries, key=lambda e: e[0])

    fn_blocks = []
    used_keys = {e[3] for e in entries_sorted if e[3] is not None}
    for key, (t_idx, n_params) in sorted(templates.items(), key=lambda x: x[1][0]):
        if key not in used_keys:
            continue
        body_lines, _, _ = key
        const_params = ", ".join(f"const P{i}: u64" for i in range(n_params))
        generics = f"<{const_params}>" if n_params > 0 else ""
        fn_blocks.append(format_aot_fn(f"tpl_{t_idx}", generics, list(body_lines), None))

    for va, sym, name_comment, key, params, single_body, cfg_meta in entries_sorted:
        if key is not None:
            continue
        if single_body is not None:
            fn_blocks.append(
                format_aot_fn(f"fn_{va:09x}", "", list(single_body), params)
            )

    ent_lines = []
    for va, sym, name_comment, key, params, single_body, cfg_meta in entries_sorted:
        _, _, _, eh_mode, _, _, _, _ = cfg_meta
        if key is not None:
            t_idx, n_params = templates[key]
            if n_params > 0:
                p_args = ", ".join(f"0x{p:x}" for p in params)
                fn_expr = f"tpl_{t_idx}::<{p_args}> as *const ()"
            else:
                fn_expr = f"tpl_{t_idx} as *const ()"
        elif single_body is not None:
            fn_expr = f"fn_{va:09x} as *const ()"
        else:
            fn_expr = f"crate::simd_ops::aot_va::<0x{va:09x}, {eh_mode}> as *const ()"
        if eh_mode == 2:
            ent_lines.append(
                f'    Replacement::draft_throws(0x{va:09x}, "{sym}", {fn_expr}, Throws::Caught), // {name_comment}\n'
            )
        elif eh_mode == 1:
            ent_lines.append(
                f'    Replacement::draft_throws(0x{va:09x}, "{sym}", {fn_expr}, Throws::Discard), // {name_comment}\n'
            )
        else:
            ent_lines.append(
                f'    Replacement::draft(0x{va:09x}, "{sym}", {fn_expr}), // {name_comment}\n'
            )

    out_dir = out_path.with_suffix("")
    if out_path.exists():
        out_path.unlink()
    emit_modular_shard_dir(out_dir, out_path.stem, allow_hdr, fn_blocks, ent_lines)
    print(f"Wrote {out_path.stem}/: {len(entries_sorted)} replacements ({len(fn_blocks)} AOT functions)")


def emit_modular_shard_dir(
    out_dir: Path,
    name: str,
    allow_hdr: str,
    fn_blocks: list[str],
    ent_lines: list[str],
):
    tpl_lines_max = 4200
    ent_chunk_sz = 4800

    t_chunks = []
    cur, cur_l = [], 0
    for b in fn_blocks:
        bl = b.count("\n") + 1
        if cur and cur_l + bl > tpl_lines_max:
            t_chunks.append(cur)
            cur, cur_l = [], 0
        cur.append(b)
        cur_l += bl
    if cur:
        t_chunks.append(cur)

    e_chunks = [ent_lines[i : i + ent_chunk_sz] for i in range(0, len(ent_lines), ent_chunk_sz)]

    if out_dir.exists():
        for old_f in out_dir.glob("*.rs"):
            old_f.unlink()
    out_dir.mkdir(exist_ok=True)

    mod_lines = [
        f"//! Sharded `{name}` AOT-compiled functions and templates.\n",
        allow_hdr,
        "use crate::registry::Replacement;\n\n",
    ]
    for idx, ch in enumerate(t_chunks):
        mname = f"t{idx:02d}"
        mod_lines.append(f"pub mod {mname};\npub use {mname}::*;\n")
        (out_dir / f"{mname}.rs").write_text(allow_hdr + "\n" + "\n".join(ch))

    ent_slice_refs = []
    for idx, ch in enumerate(e_chunks):
        mname = f"e{idx:02d}"
        mod_lines.append(f"pub mod {mname};\n")
        ent_slice_refs.append(f"    {mname}::ENTRIES,\n")
        (out_dir / f"{mname}.rs").write_text(
            allow_hdr
            + "use crate::registry::{Replacement, Throws};\nuse super::*;\n\n"
            + "pub static ENTRIES: &[Replacement] = &[\n"
            + "".join(ch)
            + "];\n"
        )
    mod_lines.append("\npub static SLICES: &[&[Replacement]] = &[\n" + "".join(ent_slice_refs) + "];\n")
    (out_dir / "mod.rs").write_text("".join(mod_lines))


# Per-function instruction cap. Larger functions lift correctly but cost compile time;
# raise it (e.g. AW2_GEN_MAX_INSNS=4000 with --only-va) to re-lift hot large functions.
MAX_INSNS = int(os.environ.get("AW2_GEN_MAX_INSNS", "550"))
MAX_FN_BYTES = int(os.environ.get("AW2_GEN_MAX_BYTES", "8192"))
DEBUG_RIPNEXT = bool(os.environ.get("AW2_GEN_DEBUG_RIPNEXT"))


def parse_args(argv=None):
    import argparse

    # `@file`: arguments one per line (e.g. a 100k-VA --only-va list beyond ARG_MAX).
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter, fromfile_prefix_chars="@")
    ap.add_argument(
        "--only-va",
        action="append",
        default=[],
        metavar="VA[,VA...]",
        help="review mode: lift only these function VAs (hex) and write each as fn_<va> into "
        "--out-dir (default target/review/gen); the real shard files are never touched",
    )
    ap.add_argument(
        "--out-dir",
        default=None,
        help="output directory; without --only-va, the full ecs_more/r_more/subsystems shard "
        "dirs are emitted here instead of crates/aw2-game/src/shards",
    )
    ap.add_argument(
        "--image",
        default=None,
        help="DLL mode (see dll_lift.py): lift this DLL instead of AlanWake2.exe; output goes to "
        "crates/aw2-game/src/dll_native/lift/<module>/ (or --out-dir/<module>/)",
    )
    ap.add_argument("--base", default=None, help="DLL mode: pinned base VA (hex) the image is relocated to")
    ap.add_argument("--name", default=None, help="DLL mode: loader module name (default: lower-case file name)")
    ap.add_argument(
        "--rva-list",
        default=None,
        help="DLL mode: lift only the functions whose start RVA (hex, one per line) is listed, e.g. the "
        "executed ones from the AOT JIT profile (tools/decomp_scale/dll_profile_fns.py)",
    )
    ap.add_argument(
        "--dataref-leaves",
        action="store_true",
        help="DLL mode: also lift pdata-less leaves reached only through data (relocated pointers into "
        ".text from vtables / function tables, CFG GuardCFFunctionTable, RIP-relative leas; dll_lift.load)",
    )
    ap.add_argument(
        "--dataref-listed",
        action="store_true",
        help="DLL mode (with --rva-list and --dataref-leaves): keep only dataref leaves listed in --rva-list "
        "(or reached from listed roots/thunks), instead of every GuardCFFunctionTable leaf in the image",
    )
    args = ap.parse_args(argv)
    if bool(args.image) != bool(args.base):
        ap.error("--image and --base go together")
    return args


def main(argv=None):
    global EH_CUR, EH_SITES
    args = parse_args(argv)
    only_vas = {int(x, 16) for s in args.only_va for x in s.split(",") if x.strip()}
    review = bool(only_vas)
    if LOCK_RMW_MODE == "legacy-plain" and not (review or args.out_dir):
        raise SystemExit("AW2_LOCK_RMW_MODE=legacy-plain is validation-only; pass --only-va or --out-dir")
    LOCKED_INSN_VAS.clear()
    print(f"lock RMW mode: {LOCK_RMW_MODE}")

    dll_info = None
    if args.image:
        import dll_lift

        pe, all_rows, frag_map, fn_starts, dll_info = dll_lift.load(args, sys.modules[__name__])
        existing = set()
        can_throw_map = {}
    else:
        existing = set() if review else collect_all_existing_vas()
        print(f"Existing VAs outside target shards: {len(existing)}")

        pe = fndb.PE(str(EXE_PATH))
        # L5: the exe's eh_mode-2 functions use the T3 record form (EhFrame, `eh.site` /
        # `eh.call`), never `call_va8_caught` + rethrow.
        if EXE_EH:
            EH_SITES = True
        db = sqlite3.connect(str(ROOT / "db" / "functions.sqlite"))
        can_throw_map = dict(db.execute("SELECT va, mask_direct FROM can_throw").fetchall())
        fn_starts = {r[0] for r in db.execute("SELECT va FROM functions").fetchall()}

        frag_map = {}
        for fva, va, fsz in db.execute(
            "SELECT func_va, va, end - va FROM fragments ORDER BY func_va, va"
        ).fetchall():
            if fsz > 0:
                frag_map.setdefault(fva, []).append((va, fsz))

        all_rows = db.execute(
            """
            SELECT va, end - va, size, n_callees, owner, top_ns, namespace, name
            FROM functions
            ORDER BY va
            """
        ).fetchall()
    if review:
        all_rows = [r for r in all_rows if r[0] in only_vas]
        missing = only_vas - {r[0] for r in all_rows}
        if missing:
            print("not in functions table: " + ", ".join(f"0x{v:x}" for v in sorted(missing)))


    blob = bytearray()
    offset_map = []
    base_vaddr = 0x140000000
    all_target_rows = []
    reject_reason = {}
    for va, p_sz, sz, nc, owner, top_ns, ns, name in all_rows:
        if va in existing:
            continue
        b = read_frag_va(pe, va, p_sz) or b""
        eh_exact = fast_eh_info_exact(pe, va)
        md = can_throw_map.get(va, 0)
        cfg_meta = extract_cfg_meta(va, sz, b, eh_exact, md)
        if dll_info is not None:
            # DLL C++ EH (dll_lift `eh`): 1 = cleanup-only / empty FuncInfo4, 2 = try map.
            ehx = dll_info["eh"].get(va)
            cfg_meta = (va, sz, 0x28, ehx["mode"] if ehx else 0, 0, 0, 0, 0)
        all_target_rows.append((va, sz, nc, owner, top_ns, ns, name, cfg_meta))

        if not b:
            reject_reason[va] = "empty_or_unreadable_primary"
            continue
        if sz > MAX_FN_BYTES:
            reject_reason[va] = f"exceeds MAX_FN_BYTES ({sz} > {MAX_FN_BYTES})"
            continue
        raw_segs = [(va, b)]
        for fva, fsz in frag_map.get(va, []):
            if fva == va:
                continue
            fb = read_frag_va(pe, fva, fsz)
            if fb:
                raw_segs.append((fva, fb))
        raw_segs.sort(key=lambda x: x[0])
        merged_segs = []
        for s_va, s_b in raw_segs:
            if merged_segs and merged_segs[-1][0] + len(merged_segs[-1][1]) == s_va:
                merged_segs[-1] = (merged_segs[-1][0], merged_segs[-1][1] + s_b)
            else:
                merged_segs.append((s_va, s_b))
        raw_segs = merged_segs
        tot_seg_sz = sum(len(x[1]) for x in raw_segs)
        if tot_seg_sz > MAX_FN_BYTES:
            reject_reason[va] = f"exceeds MAX_FN_BYTES ({tot_seg_sz} > {MAX_FN_BYTES})"
            continue

        segs_info = []
        for s_va, s_bytes in raw_segs:
            start_off = len(blob)
            blob.extend(s_bytes)
            blob.extend(b"\xcc" * 16)
            while len(blob) % 16 != 0:
                blob.append(0xCC)
            segs_info.append((start_off, len(s_bytes), s_va))
        offset_map.append((segs_info, va, sz, nc, owner, top_ns, ns, name, cfg_meta))

    print(f"Disassembling {len(offset_map)} candidate functions for AOT lifting...")
    elf_path = ROOT / "target" / ".gen_subsystems_batch.elf"
    elf_path.parent.mkdir(parents=True, exist_ok=True)
    elf_path.write_bytes(g.build_elf64(bytes(blob), base_vaddr=base_vaddr))
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
    elf_path.unlink(missing_ok=True)

    insn_by_off = {}
    locked_offs = set()
    lock_pending = False
    line_re = re.compile(r"^\s*([0-9a-fA-F]+):\s+([a-z0-9_<>]+)\s*(.*)$")
    for line in out.splitlines():
        m = line_re.match(line)
        if m:
            vaddr = int(m.group(1), 16)
            off = vaddr - base_vaddr
            mn = m.group(2)
            ops = m.group(3).split("#")[0].strip()
            if lock_pending:
                # Instruction following a bare `lock` line carries the F0 prefix.
                locked_offs.add(off)
                lock_pending = False
            if mn == "lock" and not ops:
                lock_pending = True
            elif mn == "lock" and ops:
                parts_l = ops.split(None, 1)
                mn = parts_l[0]
                ops = parts_l[1] if len(parts_l) > 1 else ""
                locked_offs.add(off)
            elif mn in ("rep", "repz", "repe") and ops:
                sub_mn = ops.split()[0]
                if sub_mn == "ret":
                    mn, ops = "ret", ""
                elif sub_mn in ("movsq", "movsb"):
                    mn = f"rep_{sub_mn}"
                    ops = "rdi, rsi, rcx"
                elif sub_mn in ("stosq", "stosd", "stosw", "stosb"):
                    mn = f"rep_{sub_mn}"
                    ops = "rdi, rax, rcx"
                elif sub_mn == "cmpsw":
                    mn = "rep_cmpsw"
                    ops = "rdi, rsi, rcx"
            elif mn in ("repnz", "repne") and ops:
                sub_mn = ops.split()[0]
                if sub_mn in ("scasw", "scasb"):
                    mn = f"repne_{sub_mn}"
                    ops = "rdi, rax, rcx"
            mn, ops = norm_objdump_mn_ops(mn, ops)
            insn_by_off[off] = (mn, ops)

    bad_regs_re = re.compile(r"\b(zmm\d+|mm\d+|fs|cs|ds|es|ss)\b")
    bad_mnems = {"rep", "repz", "repnz", "repe", "repne", "leave", "enter"}

    templates = {}
    lifted_by_va = {}
    stack_infos = {}  # va -> dll_lift.stack_arg_info of each candidate
    wide_readers = {}  # va -> max incoming slot (>= 24) of candidates (lifted or not)
    stack_windows = {}  # va -> argument window A (> 24 only) of lifted wide functions
    import dll_lift as _dl

    exe_eh = {}  # exe (L5): va -> FH4 facts of an eh_mode-2 candidate (EH_CUR)
    if dll_info is None and EXE_EH:
        eh_vas = [o[1] for o in offset_map if o[8][3] == 2]
        exe_eh, eh_counts = _dl.exe_eh_facts(str(EXE_PATH), IMAGE_BASE, eh_vas, frag_map)
        print(
            f"exe C++ EH (L5): {len(eh_vas)} eh_mode-2 candidates, {len(exe_eh)} with an EhFrame record; "
            f"by outcome {dict(sorted(eh_counts.items(), key=lambda kv: -kv[1]))} (no record: lifted with plain calls, as before L5)"
        )

    for segs_info, va, sz, nc, owner, top_ns, ns, fn_name, cfg_meta in offset_map:
        seg_raws = []
        seg_bad = False
        for start_off, seg_len, seg_va in segs_info:
            if start_off not in insn_by_off:
                seg_bad = True
                break
            seg_raw = []
            for off in range(start_off, start_off + seg_len):
                if off in insn_by_off:
                    mn, ops = insn_by_off[off]
                    real_va = seg_va + (off - start_off)
                    if off in locked_offs:
                        LOCKED_INSN_VAS.add(real_va)
                    if mn in ("nop", "lock", "pause"):
                        ops = ""
                    elif (mn.startswith("j") or mn == "call") and ops:
                        first_tok = ops.strip().split()[0]
                        if first_tok.startswith("0x"):
                            try:
                                elf_t_va = int(first_tok, 16)
                                real_t_va = (
                                    seg_va + (elf_t_va - (base_vaddr + start_off))
                                ) & 0xFFFFFFFFFFFFFFFF
                                ops = f"0x{real_t_va:x}"
                            except ValueError:
                                pass
                    seg_raw.append((real_va, mn, ops))
            # Verify the last instruction in the segment did not spill into the 0xCC padding.
            # A trailing MSVC jump table (data after the last `ret`/`jmp`) decodes as garbage
            # that can run past the end; that region is cut at the table start below, so a
            # spill is fine when an in-segment table displacement explains it.
            if (start_off + seg_len) not in insn_by_off:
                jt_in_seg = any(
                    seg_va < IMAGE_BASE + int(m.group(1), 16) < seg_va + seg_len
                    for _, _, rops in seg_raw
                    for m in JT_DISP_RE.finditer(rops)
                )
                if not jt_in_seg:
                    seg_bad = True
                    break
            seg_raws.append((seg_va, seg_len, seg_raw))
        if seg_bad or not seg_raws:
            reject_reason[va] = "segment decode spilled into padding"
            continue
        # Stack-argument window (dll_lift.STACK_ARG_SLOT_LIMIT): A > 24 for functions reading
        # slot 24+ (exe 0x14155d810, cohtml +0x490110) and, in DLL mode, for escaping incoming
        # areas. Reads beyond arg32 (what guest callers publish) stay guest code.
        s_info = stack_arg_info_segs(seg_raws, va)
        s_max = s_info["max_slot"]
        if s_max >= _dl.STACK_ARG_SLOT_LIMIT:
            wide_readers[va] = s_max
        if s_max > _dl.GUEST_ARG_SLOT_MAX:
            reject_reason[va] = f"reads incoming stack arg{s_max} (guest callers publish arg5..arg{_dl.GUEST_ARG_SLOT_MAX})"
            continue
        stack_infos[va] = s_info
        s_win = _dl.stack_window(s_max, dll_info is not None and _dl.window_escapes(s_info))
        all_jt_vas = [
            IMAGE_BASE + int(m.group(1), 16)
            for _, _, s_r in seg_raws
            for _, _, rops in s_r
            for m in JT_DISP_RE.finditer(rops)
        ]
        all_seg_addrs = {a for _, _, s_r in seg_raws for a, _, _ in s_r}
        insns = []
        import struct as _st
        for seg_va, seg_len, seg_raw in seg_raws:
            # True fall-through address of each decoded instruction, taken before the jump-table
            # cut / padding pop below. RIP-relative operands are relative to this; after a cut the
            # list successor can be past the table (0x14205b0a0: `jmp [rip+d]` resolved 0x19 high).
            true_next = {
                seg_raw[k][0]: (seg_raw[k + 1][0] if k + 1 < len(seg_raw) else seg_va + seg_len)
                for k in range(len(seg_raw))
            }
            jt_cuts = [jva for jva in all_jt_vas if seg_va <= jva < seg_va + seg_len]
            if not jt_cuts and seg_raw and seg_raw[-1][1] not in ("ret", "jmp", "int3", "ud2", "nop"):
                end_aligned = (seg_va + seg_len) & ~3
                t_cur = end_aligned
                n_rvas = 0
                while t_cur - 4 >= seg_va:
                    prev_b = read_frag_va(pe, t_cur - 4, 4)
                    if not prev_b:
                        break
                    rva = _st.unpack("<I", prev_b)[0]
                    t_va = IMAGE_BASE + rva
                    if t_va in all_seg_addrs or (n_rvas >= 2 and seg_va - 64 <= t_va < seg_va):
                        t_cur -= 4
                        n_rvas += 1
                    else:
                        break
                if n_rvas >= 4:
                    jt_cuts.append(t_cur)
            if jt_cuts:
                cut_va = min(jt_cuts)
                while cut_va - 4 > seg_va:
                    prev_b = read_frag_va(pe, cut_va - 4, 4)
                    if not prev_b:
                        break
                    rva = _st.unpack("<I", prev_b)[0]
                    if (IMAGE_BASE + rva) in all_seg_addrs:
                        cut_va -= 4
                    else:
                        break
                seg_raw = [x for x in seg_raw if x[0] < cut_va]
                while len(seg_raw) > 1 and seg_raw[-1][1] in ("int3", "nop"):
                    seg_raw.pop()
            for idx_s, (r_va, r_mn, r_ops) in enumerate(seg_raw):
                if DEBUG_RIPNEXT and "rip" in r_ops:
                    old_nxt = seg_raw[idx_s + 1][0] if idx_s + 1 < len(seg_raw) else (seg_va + seg_len)
                    if old_nxt != true_next[r_va]:
                        print(f"RIPNEXT fn=0x{va:x} insn=0x{r_va:x} old=0x{old_nxt:x} new=0x{true_next[r_va]:x}")
                insns.append((r_va, r_mn, r_ops, true_next[r_va]))
        if not insns:
            reject_reason[va] = "no instructions"
            continue

        while len(insns) > 1 and insns[-1][1] in ("int3", "nop"):
            insns.pop()
        trimmed = insns
        if len(trimmed) > MAX_INSNS:
            reject_reason[va] = f"too many instructions ({len(trimmed)} > {MAX_INSNS})"
            continue
        # A function must end in a terminator, or (e.g. a trailing noreturn call) be followed by
        # int3 padding. Otherwise the recorded extent is truncated and the lifter would silently
        # append `return rax`, dropping the real tail (0x141be4380: placement-ctor returned 0).
        seg_end_va = seg_raws[-1][0] + seg_raws[-1][1]
        tail_after_call_ok = False
        if trimmed[-1][1] == "call":
            end_b = read_frag_va(pe, seg_end_va, 1)
            tail_after_call_ok = (
                seg_end_va in fn_starts
                or bool(end_b and end_b[0] == 0xCC)
                or is_noreturn_call_insn(trimmed[-1])
            )
            nxt = trimmed[-1][3]
            nxt_b = read_frag_va(pe, nxt, 1)
            if TRAPS and nxt_b and nxt_b[0] == 0xCC:
                # The int3 right after a trailing (noreturn) call is what runs if the call
                # returns: keep it (popped above as padding), so it lowers to a trap instead of
                # the appended `return rax`.
                trimmed = trimmed + [(nxt, "int3", "", nxt + 1)]
                TAIL_TRAP_VAS.add(nxt)
        elif (
            trimmed[-1][1] not in ("ret", "jmp", "int3", "ud2")
            and trimmed[-1][3] == seg_end_va
            and seg_end_va in fn_starts
            and trimmed[-1][0] + 1 < seg_end_va
        ):
            # Falls through into the next function: equivalent to `jmp next` (same frame).
            last = trimmed[-1]
            trimmed = trimmed[:-1] + [(last[0], last[1], last[2], last[0] + 1),
                                      (last[0] + 1, "jmp", f"0x{seg_end_va:x}", seg_end_va)]
        if trimmed[-1][1] not in ("ret", "jmp", "int3", "ud2") and not tail_after_call_ok:
            tail_b = read_frag_va(pe, trimmed[-1][3], 1)
            if not tail_b or tail_b[0] != 0xCC:
                TRUNCATED_VAS.add(va)
                reject_reason[va] = "truncated extent (no terminator)"
                if os.environ.get("AW2_GEN_DEBUG_TRUNC"):
                    print(f"  trunc 0x{va:x} tail: {[(hex(a), m, o) for a, m, o, _ in trimmed[-4:]]} segs={[(hex(s), l) for s, l, _ in seg_raws]}")
                continue

        all_fn_addrs = {a for a, _, _, _ in trimmed}
        bad = None
        for addr, mn, ops, _ in trimmed:
            if (
                mn in bad_mnems
                or mn in EXTRA_BAD_MNEMS
                or (mn.startswith("v") and mn not in ALLOWED_V_MNS)
                or bad_regs_re.search(ops)
            ):
                bad = f"unsupported instruction at 0x{addr:x}: {mn} {ops}"
                break
            if mn.startswith("j") and mn != "jmp":
                t_str = ops.strip().split()[0]
                try:
                    t_va = int(t_str, 16)
                except ValueError:
                    bad = f"unsupported instruction at 0x{addr:x}: {mn} {ops}"
                    break
                if t_va in all_fn_addrs:
                    continue
                # Conditional tail jmp (frontier_plan.md §1): lowered as `if cond { tail call }`
                # (EXT_P_n) when the target, through incremental-link thunks (CALL_REDIRECT), is
                # a function entry: pdata root / export, pdata-less leaf, or a `jmp [rip+IAT]`
                # import stub (resolved per call, as for an unconditional tail jmp).
                t2 = CALL_REDIRECT.get(t_va, t_va)
                tb = (read_frag_va(pe, t2, 3) or b"") if dll_info is not None else b""
                if not (t2 in fn_starts or t2 in LEAF_STARTS or tb[:2] == b"\xff\x25" or tb == b"\x48\xff\x25"):
                    why = LEAF_REJECTS.get(t2, "jcc to non-entry code")
                    bad = f"{why} at 0x{addr:x}: {mn} 0x{t_va:x}" + (f" (-> 0x{t2:x})" if t2 != t_va else "")
                    break
        if bad:
            reject_reason[va] = bad
            continue

        _, _, _, eh_mode, _, _, _, _ = cfg_meta
        ehx = dll_info["eh"].get(va) if dll_info is not None else exe_eh.get(va)
        EH_CUR = dict(ehx, fn_va=va) if ehx else None
        SEH_GUARD_FAIL[0] = None
        TRANSLATE_FAIL[0] = None
        res = translate_function(trimmed, va, sz, eh_mode, pe)
        EH_CUR = None
        if res is None:
            reject_reason[va] = SEH_GUARD_FAIL[0] or TRANSLATE_FAIL[0] or "translate_function failed"
            continue
        lines, params = res
        if dll_info is None and ehx:
            # Exe (L5): the record's `EhMeta` is a static of the fn itself (no EH_META table;
            # it names the fn's VA, so the body is never shared as a template).
            lines = [exe_eh_meta_line(va, ehx)] + [l.replace("EH_META_SELF", "&EH_M") for l in lines]
        if any("(0x0u64) as *" in l for l in lines):
            reject_reason[va] = "null-address memory access in output"
            continue
        if s_win != _dl.STACK_ARG_SLOT_LIMIT:
            # Before the key: templates are shared only between equal windows.
            lines = [f"// stackargs: A={s_win} max={s_max}"] + list(lines)
            stack_windows[va] = s_win

        key = (tuple(lines), len(params), eh_mode)
        if key not in templates:
            templates[key] = (len(templates), len(params))
        lifted_by_va[va] = (key, params)

    # Residual caller rules (dll_lift.STACK_ARG_SLOT_LIMIT): a big-frame (LiftedStack::<1>)
    # caller publishes rsp+0xc0 (arg5..arg24) at its read_s5_call/_tail sites, and nothing
    # forwards past arg32 to a guest callee. LiftedFrame callers publish their frame top, which
    # covers what they write, so their wide callees (lifted with A > 24, or guest) are fine.
    slot_max = dict(dll_info.get("slot_max", {})) if dll_info is not None else {}
    slot_max.update(wide_readers)
    for c_va in sorted(lifted_by_va):
        c_inf = stack_infos.get(c_va)
        if not c_inf:
            continue
        lim = _dl.BIG_FRAME_FWD_SLOT_LIMIT if c_inf["big_frame"] else _dl.GUEST_FWD_SLOT_LIMIT
        hit = sorted(t for t in c_inf["targets"] if t != c_va and slot_max.get(t, -1) >= lim)
        if hit:
            del lifted_by_va[c_va]
            stack_windows.pop(c_va, None)
            reject_reason[c_va] = (
                f"calls 0x{hit[0]:x}, which reads incoming stack arg{slot_max[hit[0]]} "
                + ("(a big-frame caller publishes arg5..arg24)" if c_inf["big_frame"] else "(beyond arg32)")
            )
    if wide_readers or stack_windows:
        print(
            f"stack-arg windows: {len(stack_windows)} lifted with A > {_dl.STACK_ARG_SLOT_LIMIT}: "
            + ", ".join(f"0x{v:x} (A={a}, max={wide_readers.get(v, '-')})" for v, a in sorted(stack_windows.items()))
        )
        guest_wide = sorted(v for v in wide_readers if v not in lifted_by_va)
        if guest_wide:
            print(
                f"stack-arg windows: {len(guest_wide)} arg{_dl.STACK_ARG_SLOT_LIMIT}+ readers not lifted: "
                + ", ".join(f"0x{v:x} (arg{wide_readers[v]}: {reject_reason.get(v, '?')})" for v in guest_wide)
            )

    if review:
        out_dir = Path(args.out_dir) if args.out_dir else ROOT / "target" / "review" / "gen"
        out_dir.mkdir(parents=True, exist_ok=True)
        for va in sorted(only_vas):
            p = out_dir / f"fn_{va:09x}.rs"
            if va in lifted_by_va:
                (k_lines, _, _), params = lifted_by_va[va]
                p.write_text(format_aot_fn(f"fn_{va:09x}", "", list(k_lines), params))
                print(f"lifted 0x{va:x} -> {p}")
            else:
                p.unlink(missing_ok=True)
                print(f"NOT lifted 0x{va:x} ({reject_reason.get(va, 'filtered before disassembly: size/extent')})")
        return

    if dll_info is not None:
        out_root = Path(args.out_dir) if args.out_dir else ROOT / "crates" / "aw2-game" / "src" / "dll_native" / "lift"
        dll_lift.emit(out_root, dll_info, sys.modules[__name__], templates, lifted_by_va, all_rows, reject_reason, format_aot_fn)
        return

    key_uses = Counter(k for k, _ in lifted_by_va.values())
    kept_keys = {
        key
        for key in templates
        if (key_uses[key] >= 2 and len(key[0]) <= 36) or (key_uses[key] >= 4 and len(key[0]) <= 64)
    }

    ecs_entries = []
    r_entries = []
    sub_entries = []

    for va, sz, nc, owner, top_ns, ns, fn_name, cfg_meta in all_target_rows:
        prefix = top_ns if top_ns else (owner if owner else "sub")
        clean_prefix = re.sub(r"[^a-zA-Z0-9_]", "_", prefix).lower()
        sym = f"{clean_prefix}_{va:09x}"
        name_comment = f"{ns}::{fn_name}" if ns and fn_name else (fn_name or f"sub_{va:x}")
        name_comment = name_comment.replace("\n", " ").replace("\r", "")[:120]

        if va in lifted_by_va:
            k, params = lifted_by_va[va]
            if k in kept_keys:
                key, single_body = k, None
            elif top_ns in ("ecs", "dp") or (top_ns == "r" and len(k[0]) <= 12):
                key, single_body = None, k[0]
            else:
                key, single_body = None, None
        else:
            key, params, single_body = None, [], None

        entry = (va, sym, name_comment, key, params, single_body, cfg_meta)
        if top_ns in ("ecs", "dp"):
            ecs_entries.append(entry)
        elif top_ns == "r":
            r_entries.append(entry)
        else:
            sub_entries.append(entry)

    shards_dir = Path(args.out_dir) if args.out_dir else ROOT / "crates" / "aw2-game" / "src" / "shards"
    shards_dir.mkdir(parents=True, exist_ok=True)
    emit_shard_file(
        shards_dir / "ecs_more.rs",
        "Additional ECS and DP subsystem functions.",
        templates,
        ecs_entries,
    )
    emit_shard_file(
        shards_dir / "r_more.rs",
        "Additional R:: core/renderer/resource subsystem functions.",
        templates,
        r_entries,
    )
    emit_shard_file(
        shards_dir / "subsystems.rs",
        "Engine and game subsystem functions across remaining namespaces.",
        templates,
        sub_entries,
    )
    total = len(ecs_entries) + len(r_entries) + len(sub_entries)
    n_multi = sum(1 for v in lifted_by_va if lifted_by_va[v][0] in kept_keys)
    n_single = len(lifted_by_va) - n_multi
    print(
        f"Total generated across 3 shards: {total} (lifted inline: {len(lifted_by_va)} = {n_multi} via {len(kept_keys)} shared templates + {n_single} single-use fn_<va>, guest-va wrapper: {total - len(lifted_by_va)})"
    )


if __name__ == "__main__":
    main()
