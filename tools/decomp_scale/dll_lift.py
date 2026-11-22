"""DLL mode of gen_subsystems.py: `--image <dll> --base <pinned VA>`.

Lifts a game-shipped DLL (msvcp140, vcruntime140(_1), v8_libbase, v8_libplatform, ...) with
the exe lifter. The lifted Rust embeds absolute VAs (data / call targets / jump tables), so the
image is relocated to the base the loader pins it at (`aw2_game::dll_native::pinned_base`) and
the runtime installs the result only when the module is mapped exactly there with a
byte-identical `.text` (base + FNV-1a gate in `dll_native::lift`).

Differences from the exe mode:
  * functions = `.pdata` roots (chained unwind fragments merged into their primary) plus
    exports without `.pdata` (leaf functions), instead of db/functions.sqlite;
  * functions with an exception/termination handler are classified by the handler
    (`handler_kind`, target/review/exec/g12/eh_lift_plan.md):
      - `gs_check` (`__GSHandlerCheck`: validates the frame's cookie when an exception passes and
        always returns ExceptionContinueSearch) is lifted like a function without handler: the
        handler never runs on a non-throwing path;
      - `v8_nonabi` (`CrashForExceptionInNonABICompliantCodeRange`: V8 embedded builtins, which
        take inputs in V8's registers, e.g. r13 = roots) is rejected with that reason;
      - C++ EH with `__CxxFrameHandler4` (`fh4`, or `fh4_gs`: `__GSHandlerCheck_EH4`) is lifted
        on the normal path (T1: cleanup-only / empty FuncInfo4, `eh_mode` 1; T2: try/catch,
        `eh_mode` 2, calls in try ranges use the catching form with continuation dispatch).
        Each gets an `EH_META` record (FuncInfo4, frame register / establisher offset,
        continuations) for the T3 runtime, catch funclets (header.isCatch) included;
      - SEH with a C scope table (`seh_scope`: `__C_specific_handler` / `__GSHandlerCheck_SEH`)
        is lifted with an `EH_META` record (handler 3, conts = `__except` JumpTargets, every
        call catching) unless `seh_scopes` names a reason (`seh_body_fault`, ...);
      - setjmp callers (`setjmp_sites`) get an `EH_META` record whose continuations are the
        setjmp return addresses;
      undecodable FuncInfos, FH3 (`fh3`), `_except_handler` (`seh_startup`) and SEH context
      exports stay guest code, counted per kind in `skipped_eh`;
    other lifted entries are `eh_mode` 0;
  * CFG dispatch / security-cookie / __chkstk helpers are found in the image itself
    (load config directory, byte patterns) and retargeted in gen_subsystems' globals;
  * output is `crates/aw2-game/src/dll_native/lift/<module>/` with an `IMAGE` descriptor
    (base, .text range + FNV) and `Lifted` entries only (functions that do not lift keep
    running as guest code; there are no `aot_va` wrappers).
"""

from pathlib import Path
import os
import re
import struct

import fndb  # noqa: E402  (sys.path set up by gen_subsystems)

MASK64 = 0xFFFF_FFFF_FFFF_FFFF

# Exports that capture, unwind to or restore a guest context (setjmp/longjmp, SEH/C++ EH
# dispatch, GS failure reporting). A lifted body runs on a host-side frame with no unwind
# info and cannot move the guest rsp/rip, so these always stay guest code (C++ EH on
# aarch64 is a known gap, see target/review/exec/small_dlls.md).
CONTEXT_EXPORT_RE = re.compile(
    r"setjmp|longjmp|_local_unwind|C_specific_handler|FrameHandler|_CxxThrowException|"
    r"__report_gsfailure|NLG_|CallSettingFrame|CallCatchBlock|GSHandlerCheck|RtlUnwind|"
    r"_except_handler|__DestructExceptionObject|__CxxExceptionFilter|_CxxFrameUnwindFilter"
)
# Imports that capture, walk or restore the *calling* machine context. A function that calls
# one directly (`call/jmp [rip+IAT]` or loads the IAT slot) would, lifted, capture the native
# host frames instead of the guest's: semantically wrong, so it stays guest code
# (e.g. vcruntime's capture_previous_context, found by the dll_lift fuzz).
# The setjmp family (SETJMP_IMPORT_RE) stores the caller's rsp / return address /
# nonvolatiles for a later longjmp: called through the guest helper, the caller's frame is a
# host frame (renoircore +0x16cc00: `mov rdx, rsp; call __intrinsic_setjmp` saved the lifted
# frame's sentinel return slot, found by the dll_lift fuzz). A function whose only context
# imports are plain `call`s of it is lifted with an `EhFrame::setjmp` at each call instead
# (`setjmp_sites`, eh_lift_plan.md T3): the buffer names the lifted frame's record and the
# return address, a continuation the longjmp unwind lands at. `longjmp` is no context import:
# vcruntime's longjmp is an `RtlUnwind(STATUS_LONGJMP)` from guest code, and its caller's frame
# is never resumed.
CONTEXT_IMPORT_RE = re.compile(
    r"^(RtlCaptureContext|RtlLookupFunctionEntry|RtlVirtualUnwind|RtlUnwind|RtlUnwindEx|RtlRestoreContext|"
    r"_local_unwind|__intrinsic_setjmp|__intrinsic_setjmpex|_setjmp|_setjmpex|setjmp)$"
)
SETJMP_IMPORT_RE = re.compile(r"^(__intrinsic_setjmp|__intrinsic_setjmpex|_setjmp|_setjmpex|setjmp)$")
# Imports that walk the caller's stack for a backtrace (no context captured or restored): a
# caller lifts with an EH record (handler 0) so the record-chain walk reports its frame.
BACKTRACE_IMPORT_RE = re.compile(r"^(RtlCaptureStackBackTrace|RtlWalkFrameChain|CaptureStackBackTrace)$")
NORETURN_IMPORT_RE = re.compile(
    r"^(_CxxThrowException|abort|_Exit|_exit|exit|quick_exit|terminate|"
    r"_invalid_parameter_noinfo_noreturn|RaiseFailFastException|ExitProcess|ExitThread)$"
)


def import_ref_sites(pe, ranges, slots):
    """Every reference to an import in `slots` (IAT slot rva -> name) inside the rva `ranges`,
    as (name, kind, insn rva, insn length): kind "call" = `[48] FF 15` call [rip+d] or `E8`
    call of a local `FF 25` import thunk; "other" = jmp [rip+d], mov reg, [rip+d], `E9` to a
    thunk. Same byte scan as `context_import_refs`."""
    out = []
    for b, e in ranges:
        o = pe.off(b)
        if o is None:
            continue
        code = pe.data[o : o + (e - b)]
        for i in range(len(code) - 5):
            op, m = code[i], code[i + 1]
            if (op == 0xFF and m in (0x15, 0x25)) or (op == 0x8B and (m & 0xC7) == 0x05):
                tgt = b + i + 6 + struct.unpack_from("<i", code, i + 2)[0]
                if tgt in slots:
                    if op == 0xFF and m == 0x15:
                        rex = i > 0 and code[i - 1] == 0x48
                        out.append((slots[tgt], "call", b + i - rex, 6 + rex))
                    else:
                        out.append((slots[tgt], "other", b + i, 6))
            elif op in (0xE8, 0xE9):
                th = b + i + 5 + struct.unpack_from("<i", code, i + 1)[0]
                to = pe.off(th) if 0 <= th < (1 << 32) else None
                if to is not None and pe.data[to : to + 2] == b"\xff\x25":
                    tgt = th + 6 + struct.unpack_from("<i", pe.data, to + 2)[0]
                    if tgt in slots:
                        out.append((slots[tgt], "call" if op == 0xE8 else "other", b + i, 5))
    return out


def setjmp_sites(pe, ranges, slots, dis):
    """Call sites [(insn rva, return rva)] of the setjmp family in the function `ranges` when
    its context imports (`slots`: CONTEXT_IMPORT_RE) are exactly such calls, each a decoded
    `call` instruction; None otherwise (no setjmp, or another context import / use)."""
    refs = import_ref_sites(pe, ranges, slots)
    if not refs or any(not SETJMP_IMPORT_RE.match(n) or k != "call" for n, k, _, _ in refs):
        return None
    starts = {}
    for b, e in ranges:
        for a, mn, _ops, _parts in dis.range(b, e):
            starts[a] = mn
    if any(starts.get(a) != "call" for _, _, a, _ in refs):
        return None
    return sorted({(a, a + n) for _, _, a, n in refs})



def context_import_refs(pe, ranges, slots):
    """Name of the first CONTEXT_IMPORT_RE import whose IAT slot (rva -> name in `slots`) is
    referenced rip-relative by `FF 15/25` (call/jmp [rip+d]) or `8B /r` with a rip-relative
    ModRM (mov reg, [rip+d]) inside the rva `ranges`, or reached by an `E8/E9` call/jmp to a
    local `FF 25` import thunk; None if none. A byte scan: conservative (a stray match only
    keeps a function as guest code)."""
    for b, e in ranges:
        o = pe.off(b)
        if o is None:
            continue
        code = pe.data[o : o + (e - b)]
        for i in range(len(code) - 5):
            op, m = code[i], code[i + 1]
            if (op == 0xFF and m in (0x15, 0x25)) or (op == 0x8B and (m & 0xC7) == 0x05):
                tgt = b + i + 6 + struct.unpack_from("<i", code, i + 2)[0]
                if tgt in slots:
                    return slots[tgt]
            elif op in (0xE8, 0xE9):
                th = b + i + 5 + struct.unpack_from("<i", code, i + 1)[0]
                to = pe.off(th) if 0 <= th < (1 << 32) else None
                if to is not None and pe.data[to : to + 2] == b"\xff\x25":
                    tgt = th + 6 + struct.unpack_from("<i", pe.data, to + 2)[0]
                    if tgt in slots:
                        return slots[tgt]
    return None


# Per-function stack-argument windows (target/review/exec/stackargs_design.md). A lifted frame
# holds incoming slots 0..A-1 (entry rsp + 8*slot; slot 5 = arg5): `LiftedFrame<N, F, A>` /
# `LiftedStack<N, A>`, `init_lifted_stack` copies arg9..arg(A-1) from the caller's published
# outgoing area. A = STACK_ARG_SLOT_LIMIT (24, the runtime default) unless the function reads
# slot 24+ (A = max slot + 1, rounded up to even so entry rsp stays 8 mod 16) or, in DLL mode,
# lets the incoming area escape (`lea` of [entry+0x28+], va_start: A = ESCAPE_WINDOW, the
# guest->lifted publication limit). `format_aot_fn` reads the window from a
# `// stackargs: A=<A> max=<max slot>` body line (also the grep audit trail).
# Remaining runtime limits (never-lift):
#   * a function reading slot > GUEST_ARG_SLOT_MAX: guest callers (`host_call` /
#     `host_tail_call`) publish rsp+0x100, i.e. arg5..arg32;
#   * a lifted caller of a function reading slot >= GUEST_FWD_SLOT_LIMIT (`emu_run_full`
#     hi_args stop at arg32);
#   * a big-frame (LiftedStack::<1>) caller of a function reading slot >=
#     BIG_FRAME_FWD_SLOT_LIMIT: its read_s5_call/_tail sites publish rsp+0xc0 (arg5..arg24).
# LiftedFrame callers' read_s5_call/_tail sites publish the frame top, which covers every
# outgoing slot they write, so wide lifted callees need no `call_stk_w` there.
# History: the g2 crash, cohtml +0x490110 reads arg24/arg25 (`[rbp+0xff]`, rbp = entry rsp -
# 0x3f) and got 0 from a 24-slot frame.
STACK_ARG_SLOT_LIMIT = 24
ESCAPE_WINDOW = 32
GUEST_ARG_SLOT_MAX = 32
GUEST_FWD_SLOT_LIMIT = 33
BIG_FRAME_FWD_SLOT_LIMIT = 25
STACKARGS_RE = re.compile(r"^// stackargs: A=(\d+) max=(-?\d+)$")


def stack_window(max_slot, escapes=False):
    """Argument window `A` (qwords above the entry rsp) for a function reading incoming slots
    up to `max_slot`; `escapes`: its incoming area escapes (DLL mode, lea_args / va_start)."""
    a = STACK_ARG_SLOT_LIMIT
    if max_slot >= a:
        a = (max_slot + 2) & ~1
    if escapes:
        a = max(a, ESCAPE_WINDOW)
    return a


def window_escapes(info):
    """Does the incoming argument area escape (address taken / va_start)? `rsp_unknown` (rsp
    lost after alloca / `and rsp`) is not counted; incoming reads through a frame register
    copied before that are still tracked by `stack_arg_info`'s max_slot."""
    return any(kind in ("lea_args", "va_start") for _va, kind in info["escapes"])

_R64 = {}
for _r in ("ax", "bx", "cx", "dx"):
    for _n in (f"r{_r}", f"e{_r}", _r, f"{_r[0]}l", f"{_r[0]}h"):
        _R64[_n] = f"r{_r}"
for _r in ("si", "di", "bp", "sp"):
    for _n in (f"r{_r}", f"e{_r}", _r, f"{_r}l"):
        _R64[_n] = f"r{_r}"
for _i in range(8, 16):
    for _s in ("", "d", "w", "b"):
        _R64[f"r{_i}{_s}"] = f"r{_i}"
_VOLATILE = ("rax", "rcx", "rdx", "r8", "r9", "r10", "r11")
_MEM_RE = re.compile(r"\[([^\]]+)\]")
_OBJ_LINE_RE = re.compile(r"^\s*([0-9a-f]+):\s+(\S+)\s*(.*)$")


def _int(s):
    return int(s, 16) if s.startswith("0x") else int(s)


def _mem_base_disp(expr):
    """(base, has_index, disp) of an Intel memory operand body ('rbp + 0xff', 'rsp + 8*rax')."""
    base, index, disp = None, False, 0
    for t in re.findall(r"[+-]?[^+-]+", expr.replace(" ", "").split(":")[-1]):
        sign = -1 if t.startswith("-") else 1
        t = t.lstrip("+-")
        if "*" in t:
            index = True
        elif t in _R64:
            if base is None:
                base = _R64[t]
            else:
                index = True
        else:
            try:
                disp += sign * _int(t)
            except ValueError:
                index = True
    return base, index, disp


class _Disasm:
    """Whole-image llvm-objdump listing indexed by rva (built on first use)."""

    def __init__(self, image, file_base):
        self.image, self.file_base = image, file_base
        out = __import__("subprocess").check_output(
            ["objdump", "--disassemble", "--x86-asm-syntax=intel", "--no-show-raw-insn", str(image)], text=True
        )
        self.addrs, self.recs = self._parse(out)

    def _parse(self, out):
        addrs, recs = [], []
        for line in out.splitlines():
            m = _OBJ_LINE_RE.match(line)
            if m:
                ops = re.sub(r"\s*<[^>]*>", "", m.group(3).split("#")[0]).strip()
                parts = [p.strip() for p in re.split(r",(?![^\[]*\])", ops)] if ops else []
                parts = [_hex_target(p, self.file_base) for p in parts]
                addrs.append(int(m.group(1), 16) - self.file_base)
                recs.append((m.group(2), ops, parts))
        return addrs, recs

    def resync(self, b, e):
        """Re-decode [b, e) starting at the known entry `b` and splice it into the listing
        (the linear sweep can run out of sync on data bytes in .text before an entry: cohtml
        +0x49df00 after a byte table). True when `b` is an instruction start afterwards."""
        import bisect

        out = __import__("subprocess").check_output(
            ["objdump", "--disassemble", "--x86-asm-syntax=intel", "--no-show-raw-insn",
             f"--start-address=0x{self.file_base + b:x}", f"--stop-address=0x{self.file_base + e:x}", str(self.image)],
            text=True,
        )
        na, nr = self._parse(out)
        na_r = [(a, r) for a, r in zip(na, nr) if b <= a < e]
        if not na_r or na_r[0][0] != b:
            return False
        i = bisect.bisect_left(self.addrs, b)
        j = bisect.bisect_left(self.addrs, e)
        self.addrs[i:j] = [a for a, _ in na_r]
        self.recs[i:j] = [r for _, r in na_r]
        return True

    def range(self, b, e):
        import bisect

        i = bisect.bisect_left(self.addrs, b)
        out = []
        while i < len(self.addrs) and self.addrs[i] < e:
            out.append((self.addrs[i],) + self.recs[i])
            i += 1
        return out


def _hex_target(p, file_base):
    """Branch-target operands as rvas (`0x18048201d` -> int rva); other operands unchanged."""
    if re.fullmatch(r"0x[0-9a-f]+", p):
        v = int(p, 16)
        if v >= file_base:
            return v - file_base
    return p


def stack_arg_info(dis, frags):
    """Stack-argument facts of the function made of the rva `frags` (primary first):
      max_slot: highest incoming slot read (entry rsp + 8*slot; leas excluded), -1 if none;
      escapes:  [(va|None, kind)]: `lea_args` (address of [entry+0x28+] taken), `va_start`
                (home-slot lea + r9 spill), `rsp_unknown` (rsp-based access after the entry
                offset was lost: alloca, `and rsp`, disagreeing paths);
      calls:    [(va, "call"|"tail", direct target rva|None, X)]: X = top of the run of 8-byte
                outgoing stores [rsp+0x20], [rsp+0x28], ... [rsp+X] (rsp at the call) in the
                block before the site; None if [rsp+0x20] is not stored there, "rsp_unknown";
      targets:  direct call/jmp target rvas.
    Forward dataflow over the fragments' instructions tracking the
    entry-rsp offset of rsp and of registers copied/lea'd from it (meet = agree or unknown);
    every memory operand on a tracked register yields entry offset -> slot. Conservative only
    in the sense that matters here: a spurious hit keeps a function as guest code."""
    raw_lst = []
    for b, e in frags:
        seg = dis.range(b, e)
        for i, (va, mn, ops, parts) in enumerate(seg):
            raw_lst.append((va, mn, ops, parts, None if i + 1 < len(seg) else e))
    if not raw_lst:
        return {"max_slot": -1, "escapes": [], "calls": [], "targets": set(), "big_frame": False, "stack_pos": []}
    idx = {r[0]: i for i, r in enumerate(raw_lst)}
    lst = [
        (va, mn, ops, parts, (i + 1) if fe is None else idx.get(fe))
        for i, (va, mn, ops, parts, fe) in enumerate(raw_lst)
    ]
    targets = set()

    def succs(i):
        _va, mn, _ops, parts, ft_idx = lst[i]
        if mn in ("ret", "int3", "ud2"):
            return []
        tgt = None
        if mn.startswith("j") and parts and isinstance(parts[0], int):
            tgt = idx.get(parts[0])
        if mn == "jmp":
            return [tgt] if tgt is not None else []
        out = [ft_idx] if ft_idx is not None else []
        if tgt is not None:
            out.append(tgt)
        return out

    def transfer(off, mn, parts):
        dst = _R64.get(parts[0]) if parts and isinstance(parts[0], str) else None
        src = parts[1] if len(parts) == 2 and isinstance(parts[1], str) else None
        # "#raxlea": rax holds an address computed by `lea rax, [<stack>]` (see stack_pos).
        if mn == "call" or (dst == "rax" and mn not in ("cmp", "test", "push") and not mn.startswith("j")):
            off.pop("#raxlea", None)
        if mn == "push":
            if "rsp" in off:
                off["rsp"] -= 8
        elif mn == "pop":
            if "rsp" in off:
                off["rsp"] += 8
            off.pop(dst, None)
        elif mn in ("sub", "add") and dst == "rsp" and src is not None:
            try:
                v = _int(src)
            except ValueError:
                v = off.get("#rax") if src == "rax" else None  # __chkstk: mov eax, N; call; sub rsp, rax
            if v is None or "rsp" not in off:
                off.pop("rsp", None)
            else:
                off["rsp"] += -v if mn == "sub" else v
        elif mn == "mov" and dst and src is not None and _R64.get(src) == src and src in off:
            off[dst] = off[src]
        elif mn == "mov" and dst == "rax" and src is not None and re.fullmatch(r"0x[0-9a-f]+|\d+", src):
            off.pop("rax", None)
            off["#rax"] = _int(src)
        elif mn == "lea" and dst and src is not None:
            base, index, disp = _mem_base_disp(src.split("[")[-1].rstrip("]"))
            if base in off and not index:
                off[dst] = off[base] + disp
                if dst == "rax":
                    off["#raxlea"] = 1
            else:
                off.pop(dst, None)
        elif mn == "call":
            for r in _VOLATILE:
                off.pop(r, None)
        elif dst and mn not in ("cmp", "test") and not mn.startswith("j"):
            off.pop(dst, None)
            if dst == "rax":
                off.pop("#rax", None)

    state = [None] * len(lst)
    state[0] = {"rsp": 0}
    work, inq = [0], {0}
    while work:
        i = work.pop()
        inq.discard(i)
        s = dict(state[i])
        transfer(s, lst[i][1], lst[i][3])
        for j in succs(i):
            old = state[j]
            if old is None:
                new = dict(s)
            else:
                new = {k: v for k, v in old.items() if s.get(k) == v}
                if len(new) == len(old):
                    continue
            state[j] = new
            if j not in inq:
                inq.add(j)
                work.append(j)
    leaders = {0}
    for i in range(len(lst)):
        mn, ft_idx = lst[i][1], lst[i][4]
        for j in succs(i):
            if j != i + 1:
                leaders.add(j)
        if mn.startswith("j") or mn == "ret" or ft_idx != i + 1:
            leaders.add(i + 1)
    max_slot = -1
    escapes, calls = [], []
    stack_pos, has_call = [], False
    home_lea = r9_spill = False
    blk_stores = []  # entry offsets stored since the block start / previous call
    sub_bytes = push_bytes = 0
    dyn_rsp = False
    for i, (va, mn, ops, parts, _ft) in enumerate(lst):
        if i in leaders:
            blk_stores = []
        if mn == "push":
            push_bytes += 8
        elif mn == "sub" and len(parts) == 2 and parts[0] == "rsp":
            try:
                sub_bytes += _int(parts[1]) if isinstance(parts[1], str) else parts[1]
            except ValueError:
                dyn_rsp = True
        tgt = parts[0] if mn in ("call", "jmp") and parts and isinstance(parts[0], int) else None
        if tgt is not None:
            targets.add(tgt)
        off = state[i]
        if off is None:
            continue  # unreachable from the entry
        first = parts[0] if parts and isinstance(parts[0], str) else ""
        if mn == "call":
            has_call = True
        elif mn == "ret" and "#raxlea" in off:
            stack_pos.append((va, "returns its own stack address"))
        elif mn == "cmp" and "rsp" in parts:
            stack_pos.append((va, "compares rsp"))
        elif mn == "mov" and len(parts) == 2 and parts[1] == "rax" and "#raxlea" in off and "[" in first:
            if _mem_base_disp(first.split("[")[-1].rstrip("]"))[0] not in off:
                stack_pos.append((va, "stores its own stack address through a pointer"))
        for m in _MEM_RE.finditer(ops):
            base, _index, disp = _mem_base_disp(m.group(1))
            if base == "rsp" and "rsp" not in off:
                escapes.append((va, "rsp_unknown"))
                continue
            if base not in off:
                continue
            eo = off[base] + disp
            if mn == "lea":
                if eo >= 0x28:
                    escapes.append((va, "lea_args"))
                elif eo >= 8:
                    home_lea = True
                continue
            store = m.group(0) in first and mn not in ("cmp", "test", "push", "bt")
            if store:
                w = 32 if "ymmword" in first else 16 if "xmmword" in first else 8
                blk_stores.extend(range(eo, eo + w, 8))
                if eo == 0x20 and len(parts) == 2 and parts[1] == "r9":
                    r9_spill = True
            if eo >= 0:
                max_slot = max(max_slot, eo // 8)
        tail = mn == "jmp" and (tgt is None or tgt not in idx)
        if mn == "call" or tail:
            if "rsp" in off:
                # Outgoing args are stored as a run from [rsp+0x20] up; X = top of that run
                # (stores of locals above it are not counted unless they extend the run).
                rel = {eo - off["rsp"] for eo in blk_stores}
                x = None
                while (0x20 if x is None else x + 8) in rel:
                    x = 0x20 if x is None else x + 8
                calls.append((va, "tail" if tail else "call", tgt, x))
            else:
                calls.append((va, "tail" if tail else "call", tgt, "rsp_unknown"))
            blk_stores = []
    if home_lea and r9_spill:
        escapes.append((None, "va_start"))
    # gen_subsystems' frame choice (stack_words >= 4096 -> LiftedStack::<1>, whose call sites
    # publish rsp+0xc0 / +0xc8 instead of the frame top as the guest stack-arg window).
    big = dyn_rsp or (sub_bytes + push_bytes) // 8 + 64 > 2048
    # stack_pos: the body hands out its own stack position (GetCurrentStackPosition-style
    # reader: `lea rax, [rsp+x]` returned or stored through a pointer, in a body without
    # calls) or compares rsp itself. Lifted, rsp is an lstack arena address, not the guest
    # stack the V8 stack limits / stack starts are measured on, so such bodies stay guest.
    if has_call:
        stack_pos = [s for s in stack_pos if s[1] == "compares rsp"]
    return {"max_slot": max_slot, "escapes": escapes, "calls": calls, "targets": targets, "big_frame": big, "stack_pos": stack_pos}


def stack_pos_reason(info):
    """Rejection reason for a body with `stack_arg_info()["stack_pos"]` hits (rva in the text)."""
    va, why = info["stack_pos"][0]
    return f"stack-position reader: {why} at +0x{va:x} (lifted rsp is an lstack arena address, not the guest stack V8 limits are measured on)"


_REG_RE = re.compile(r"[re]?(?:[abcd]x|[sd]il?|[sb]pl?|r(?:8|9|1[0-5])[dwb]?|[abcd]l)")


def _reg_setup(mn, parts):
    """A register-only instruction a thunk may run before its tail jmp (argument setup):
    `xor r, r` (same register), `mov r, imm|r`, `lea r, [rip +- d]`. No memory, no stack, no
    flags anyone reads (the target is entered fresh)."""
    if len(parts) != 2 or not isinstance(parts[0], str) or not _REG_RE.fullmatch(parts[0]):
        return False
    src = parts[1]
    if mn == "xor":
        return src == parts[0]
    if mn == "mov":
        return isinstance(src, int) or bool(re.fullmatch(r"0x[0-9a-f]+|\d+", str(src))) or bool(_REG_RE.fullmatch(str(src)))
    if mn == "lea":
        return bool(re.fullmatch(r"\[rip [+-] 0x[0-9a-f]+\]", str(src)))
    return False


def tail_thunk_target(dis, rva):
    """Target rva when the function at `rva` is a tail `jmp rel32` (thunk), optionally after
    up to 4 register-only argument-setup instructions ([`_reg_setup`]), else None."""
    ins = dis.range(rva, rva + 32)
    if not ins or ins[0][0] != rva:
        return None
    for k, (_a, mn, _ops, parts) in enumerate(ins[:5]):
        if mn == "jmp":
            return parts[0] if parts and isinstance(parts[0], int) else None
        if k == 4 or not _reg_setup(mn, parts):
            return None
    return None


def stack_arg_scan(dis, frags):
    """(max incoming slot read, direct call/jmp target rvas); see `stack_arg_info`."""
    r = stack_arg_info(dis, frags)
    return r["max_slot"], r["targets"]


ALLOW_HDR = (
    "#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, "
    "dead_code, clippy::all, unreachable_code, unused_parens)]\n"
)


def fnv1a(b: bytes) -> int:
    h = 0xCBF29CE484222325
    for x in b:
        h = ((h ^ x) * 0x100000001B3) & MASK64
    return h


class DllPE(fndb.PE):
    """fndb.PE with base relocations applied for `base` (image_base = base, .text relocated)."""

    def __init__(self, path, base: int):
        super().__init__(path)
        self.file_base = self.image_base
        delta = (base - self.image_base) & MASK64
        d = bytearray(self.data)
        rva, size = self.dd[5] if len(self.dd) > 5 else (0, 0)
        self.n_relocs = 0
        self.reloc_sites = []  # rva of every applied DIR64 site (data-referenced leaves)
        if rva and size:
            o = self.off(rva)
            end = o + size
            while o + 8 <= end:
                page, blk = struct.unpack_from("<II", d, o)
                if blk < 8:
                    break
                for k in range((blk - 8) // 2):
                    e = struct.unpack_from("<H", d, o + 8 + 2 * k)[0]
                    typ, off = e >> 12, e & 0xFFF
                    if typ == 0:  # IMAGE_REL_BASED_ABSOLUTE (padding)
                        continue
                    if typ != 10:  # IMAGE_REL_BASED_DIR64
                        raise SystemExit(f"{path}: unsupported relocation type {typ} at rva 0x{page + off:x}")
                    fo = self.off(page + off)
                    if fo is None:
                        continue  # uninitialized data (not in the file)
                    v = struct.unpack_from("<Q", d, fo)[0]
                    struct.pack_into("<Q", d, fo, (v + delta) & MASK64)
                    self.n_relocs += 1
                    self.reloc_sites.append(page + off)
                o += blk
        self.data = bytes(d)
        self.image_base = base
        t = self.sec(".text")
        self.text = self.data[t[3] : t[3] + min(t[2], t[4])]
        # Mapped .text as the loader leaves it: virtual size, zero-filled past the raw data.
        self.text_mapped = self.text + b"\0" * max(0, t[2] - len(self.text))

    def u64_at_va(self, va: int):
        o = self.off(va - self.image_base)
        return None if o is None else struct.unpack_from("<Q", self.data, o)[0]

    def read_va(self, _pe, va: int, size: int):
        """`size` bytes at `va` from any section's file data (jump tables), or None.
        (Signature of gen_subsystems.READ_TABLE: called as `READ_TABLE(pe, va, size)`.)"""
        if size <= 0:
            return None
        o = self.off(va - self.image_base)
        if o is None or o + size > len(self.data):
            return None
        return self.data[o : o + size]


def _unwind_primary(pe: DllPE, u: int):
    """(primary begin rva or None, handler flags, handler rva or None) of the unwind info at
    rva `u`."""
    d = pe.data
    begin = None
    for _ in range(64):
        if u & 1:  # indirect: points at another RUNTIME_FUNCTION
            o = pe.off(u & ~1)
            begin, _, u = struct.unpack_from("<III", d, o)
            continue
        uo = pe.off(u)
        fl, cnt = d[uo] >> 3, d[uo + 2]
        tail = uo + 4 + ((cnt + 1) & ~1) * 2
        if fl & 4:  # UNW_FLAG_CHAININFO: RUNTIME_FUNCTION of the parent follows the codes
            begin, _, u = struct.unpack_from("<III", d, tail)
            continue
        h = struct.unpack_from("<I", d, tail)[0] if fl & 3 else None
        return begin, fl & 3, h
    return begin, 0, None


# `__GSHandlerCheck` (static CRT, identical in every game DLL): `sub rsp,28h; mov r8,[r9+38h]
# (DispatcherContext->HandlerData); mov rcx,rdx; mov rdx,r9; call __GSHandlerCheckCommon;
# mov eax,1 (ExceptionContinueSearch); add rsp,28h; ret`. It only validates the frame's
# security cookie while an exception passes and never claims one.
GS_HANDLER_RE = re.compile(rb"\x48\x83\xec\x28\x4d\x8b\x41\x38\x48\x8b\xca\x49\x8b\xd1\xe8....\xb8\x01\x00\x00\x00\x48\x83\xc4\x28\xc3", re.S)
CXX_HANDLERS = ("__CxxFrameHandler4", "__CxxFrameHandler3", "__CxxFrameHandler", "__GSHandlerCheck_EH4", "__GSHandlerCheck_EH")
SEH_HANDLERS = ("__C_specific_handler", "__GSHandlerCheck_SEH", "_except_handler")


def handler_kind(pe: DllPE, h: int, slots: dict, exports: dict) -> str:
    """Class of the language handler at rva `h`: `gs_check`, `v8_nonabi`, `fh4`
    (`__CxxFrameHandler4`), `fh4_gs` (local `__GSHandlerCheck_EH4`: GS check, then FH4; same
    FuncInfo4 handler data), `fh3` (older FuncInfo format), `seh_startup` or `other`. Follows
    `jmp rel32` (incremental-link thunks) to the body; an IAT `jmp [rip]` names an import. A
    local wrapper is classed by the import it calls through an IAT thunk."""
    d = pe.data

    def imp_of(rva):
        o = pe.off(rva)
        if o is not None and d[o] == 0xFF and d[o + 1] == 0x25:
            return slots.get(rva + 6 + struct.unpack_from("<i", d, o + 2)[0])
        return None

    def classify(name):
        if not name:
            return None
        if "CrashForExceptionInNonABICompliantCodeRange" in name:
            return "v8_nonabi"
        if name == "__GSHandlerCheck":
            return "gs_check"
        if name.startswith("__CxxFrameHandler4"):
            return "fh4"
        if name.startswith("__GSHandlerCheck_EH4"):
            return "fh4_gs"
        if any(name.startswith(n) for n in CXX_HANDLERS):
            return "fh3"  # FuncInfo (magic 0x1993052x) format: not decoded here, stays guest
        if name.startswith("__C_specific_handler") or name.startswith("__GSHandlerCheck_SEH"):
            return "seh_scope"  # handler data = C scope table (`seh_scopes`)
        if any(name.startswith(n) for n in SEH_HANDLERS):
            return "seh_startup"
        return None

    for _ in range(8):
        k = classify(exports.get(h)) or classify(imp_of(h))
        if k:
            return k
        o = pe.off(h)
        if o is None:
            return "other"
        if d[o] == 0xE9:
            h = h + 5 + struct.unpack_from("<i", d, o + 1)[0]
            continue
        if GS_HANDLER_RE.match(d, o):
            return "gs_check"
        # Local wrapper: the first frame handler (import thunk or in-image export) reached by a
        # rel32 call/jmp in its body.
        for i in range(o, min(o + 0xA0, len(d) - 5)):
            if d[i] in (0xE8, 0xE9):
                t = h + (i - o) + 5 + struct.unpack_from("<i", d, i + 1)[0]
                k = classify(imp_of(t)) or classify(exports.get(t))
                if k in ("fh4", "fh4_gs"):
                    return "fh4_gs"
                if k in ("fh3", "seh_startup", "seh_scope"):
                    return k
        return "other"
    return "other"


def _unwind_eh(pe: DllPE, u: int):
    """(primary UNWIND_INFO rva, handler-data rva or None) of the unwind info at rva `u`
    (same chain walk as `_unwind_primary`)."""
    d = pe.data
    for _ in range(64):
        if u & 1:
            _, _, u = struct.unpack_from("<III", d, pe.off(u & ~1))
            continue
        uo = pe.off(u)
        fl, cnt = d[uo] >> 3, d[uo + 2]
        tail = u + 4 + ((cnt + 1) & ~1) * 2
        if fl & 4:
            _, _, u = struct.unpack_from("<III", d, pe.off(tail))
            continue
        return u, (tail + 4 if fl & 3 else None)
    return None, None


def unwind_frame(pe: DllPE, u: int):
    """(frame register (x86 number, 0 = none: rsp), establisher offset) of the UNWIND_INFO at
    rva `u`: with a frame register EstablisherFrame = reg - offset (FrameOffset * 16), else it is
    rsp after the prolog = entry rsp - offset (the static rsp delta of the unwind codes)."""
    d = pe.data
    uo = pe.off(u)
    ver, cnt, fr = d[uo] & 7, d[uo + 2], d[uo + 3]
    if fr & 15:
        return fr & 15, (fr >> 4) * 16
    delta, i = 0, 0
    while i < cnt:
        op, info = d[uo + 4 + 2 * i + 1] & 15, d[uo + 4 + 2 * i + 1] >> 4
        slot = lambda k: struct.unpack_from("<H", d, uo + 4 + 2 * (i + k))[0]
        if op == 0:  # PUSH_NONVOL
            delta, n = delta + 8, 1
        elif op == 1:  # ALLOC_LARGE
            if info == 0:
                delta, n = delta + slot(1) * 8, 2
            else:
                delta, n = delta + (slot(1) | slot(2) << 16), 3
        elif op == 2:  # ALLOC_SMALL
            delta, n = delta + info * 8 + 8, 1
        elif op == 3:  # SET_FPREG
            n = 1
        elif op in (4, 8):  # SAVE_NONVOL / SAVE_XMM128
            n = 2
        elif op in (5, 9):  # *_FAR
            n = 3
        elif op == 6 and ver == 2:  # EPILOG (v2)
            n = 1
        elif op == 10:  # PUSH_MACHFRAME
            delta, n = delta + (48 if info else 40), 1
        else:
            return None, None
        i += n
    return 0, delta


_REG64 = ("rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15")
_SEH_MEM_RE = re.compile(r"(?:^|\s|:)\[\s*([a-z0-9]+)")
# Integer instructions whose memory operand gen_subsystems lowers to one `simd_ops::ru*/wu*`
# access each: inside an `__except` scope they become guarded accesses (`seh_guards`,
# `lift::eh::EhFrame::ru*` / `wu*` over `lift::guarded` probes), so a fault lands in the
# `__except` body as in the guest. gen_subsystems re-checks the lowering (named reject
# `seh_guard_unlowered`).
_SEH_GUARD_MNS = frozenset((
    "mov", "movzx", "movsx", "movsxd", "add", "sub", "and", "or", "xor", "cmp", "test",
    "inc", "dec", "neg", "not", "adc", "sbb",
))
_SEH_VEC_RE = re.compile(r"\b[xyz]mm\d+\b|\b(?:xmm|ymm|zmm)word\b|\bst\(\d\)")


def seh_scopes(pe: DllPE, hd: int, frags, dis, frame_reg: int):
    """C scope table (`__C_specific_handler` / `__GSHandlerCheck_SEH` handler data at rva `hd`:
    Count, then {BeginAddress, EndAddress, HandlerAddress, JumpTarget}) of the function made
    of the rva `frags`: (scopes, None, guards) when it lifts, (None, reason, None) otherwise:
      `seh_scope_table`  count outside 1..64 or a scope outside the function;
      `seh_jump_outside` an `__except` JumpTarget outside the function (the landing dispatches
                         on blocks of this body only);
      `seh_body_trap`    an `__except` scope holds div / idiv / int3 / ud2 / int / into (a
                         trap the lifted body does not raise as the guest would);
      `seh_filter_reads_context` an `__except` filter of a scope holding a guarded
                         instruction observes more of the access violation than its code
                         (`seh_filter_observes`: ExceptionAddress / ContextRecord are not the
                         guest's in the lifted raise);
      `seh_body_fault`   an `__except` scope holds a non-call instruction with a memory
                         operand on a register other than rsp / rip / the frame register that
                         is not a plain integer access (`_SEH_GUARD_MNS`; lock-prefixed,
                         vector / x87, string ops): it cannot be guarded.
    `guards`: rvas of the guardable faulting instructions (`_SEH_GUARD_MNS` with such an
    operand): gen_subsystems lowers their accesses to guarded ones that raise
    STATUS_ACCESS_VIOLATION into this frame at the instruction (ControlPc = its VA).
    `__finally` scopes (JumpTarget 0) only run their termination handler during an unwind."""
    d = pe.data
    n = struct.unpack_from("<I", d, pe.off(hd))[0]
    if not 1 <= n <= 64:
        return None, "seh_scope_table", None
    in_fn = lambda x, end=False: any(b <= x < e or (end and x == e) for b, e in frags)  # noqa: E731
    safe = {"rsp", "rip", "esp"} | ({_REG64[frame_reg]} if frame_reg else set())
    scopes, guards = [], set()
    for i in range(n):
        b, e, h, j = struct.unpack_from("<IIII", d, pe.off(hd + 4 + 16 * i))
        if not (in_fn(b) and in_fn(e, True) and b < e):
            return None, "seh_scope_table", None
        if j:
            if not in_fn(j):
                return None, "seh_jump_outside", None
            prev = None
            for a, mn, ops, _parts in dis.range(b, e):
                locked, prev = prev == "lock", mn
                if mn in ("div", "idiv", "int3", "ud2", "int", "into"):
                    return None, f"seh_body_trap ({mn} at +0x{a:x})", None
                if mn == "lea" or mn.startswith("nop") or mn.startswith("prefetch"):
                    continue
                for m in _SEH_MEM_RE.finditer(ops):
                    if "gs:" in ops or m.group(1) in safe:
                        continue
                    if mn in _SEH_GUARD_MNS and not locked and not _SEH_VEC_RE.search(ops):
                        guards.add(a)
                        continue
                    return None, f"seh_body_fault ({mn} {ops} at +0x{a:x})", None
        scopes.append((b, e, h, j))
    # Every `__except` filter that sees a guarded fault (all scopes holding the instruction:
    # __C_specific_handler tries each) may observe only what the lifted raise reproduces.
    for b, e, h, j in scopes:
        if j and h != 1 and any(b <= g < e for g in guards):
            why = seh_filter_observes(dis, h)
            if why:
                return None, f"seh_filter_reads_context (filter +0x{h:x}: {why})", None
    return scopes, None, sorted(guards)


_AV_CODE = 0xC0000005
_GPR_ARGS = ("rcx", "rdx", "r8", "r9")
_SEH_JCC = {"je": True, "jne": False}  # jump taken when the compared values are (un)equal


def seh_filter_observes(dis, h: int, regs=None, depth: int = 0):
    """What the `__except` filter at rva `h` (rcx = EXCEPTION_POINTERS) observes of a
    STATUS_ACCESS_VIOLATION raised by a guarded access beyond its code, or None.

    The lifted raise (lift::eh `EhFrame::access_violation`) reproduces the record's code,
    flags (0), NumberParameters (2) and Information [write, address], but ExceptionAddress is
    the host raise site and ContextRecord is the dispatcher's, not the guest's. Rather than a
    field list, any read through EXCEPTION_POINTERS / the record other than the record's code
    counts, as does the pointer escaping (stored outside the filter's frame, passed to an
    indirect or too-deep call). Walked along the path an access violation takes: `cmp dword
    [rec], imm` + je / jne resolves; other branches explore both sides; direct calls that get
    a tracked pointer are walked too (depth 2)."""
    regs = dict(regs or {"rcx": "ep"})  # 64-bit reg -> "ep" | "rec"
    seen, work = set(), [(h, regs, {}, None)]
    steps = 0
    while work:
        a, regs, slots, flag = work.pop()
        while True:
            steps += 1
            if steps > 4000:
                return "filter too large to walk"
            key = (a, tuple(sorted(regs.items())), tuple(sorted(slots.items())), flag)
            if key in seen:
                break
            seen.add(key)
            ins = dis.range(a, a + 16)
            if not ins or ins[0][0] != a:
                return f"no instruction at +0x{a:x}"
            _, mn, ops, parts = ins[0]
            nxt = dis.range(a + 1, a + 17)
            nxt_a = nxt[0][0] if nxt else None
            mems = [(i, p) for i, p in enumerate(parts) if isinstance(p, str) and "[" in p]
            # memory reads/writes through tracked pointers
            for i, p in mems:
                base, idx, disp = _mem_base_disp(p[p.index("[") + 1 : p.rindex("]")])
                cls = regs.get(base)
                if cls is None:
                    continue
                if idx:
                    return f"indexed access through {cls} at +0x{a:x}"
                if cls == "ep" and disp == 0 and mn == "mov" and i == 1:
                    continue  # rec = ep->ExceptionRecord
                if cls == "rec" and disp == 0 and not (mn == "mov" and i == 0) and mn in ("cmp", "mov", "test"):
                    continue  # the code
                return f"{mn} {ops} at +0x{a:x} reads {'EXCEPTION_POINTERS' if cls == 'ep' else 'ExceptionRecord'}+0x{disp:x}"
            dst = parts[0] if parts else None
            dst_r = _R64.get(dst) if isinstance(dst, str) else None
            src = parts[1] if len(parts) > 1 else None
            src_cls = regs.get(_R64.get(src)) if isinstance(src, str) and src in _R64 else None
            if mn in ("ret",):
                break
            if mn == "mov" and dst_r and isinstance(src, str) and "[" in src:
                base, idx, disp = _mem_base_disp(src[src.index("[") + 1 : src.rindex("]")])
                if regs.get(base) == "ep" and disp == 0:
                    regs[dst_r] = "rec"
                elif base in ("rbp", "rsp") and not idx and (base, disp) in slots:
                    regs[dst_r] = slots[(base, disp)]
                else:
                    regs.pop(dst_r, None)
            elif mn == "mov" and isinstance(dst, str) and "[" in dst and src_cls:
                base, idx, disp = _mem_base_disp(dst[dst.index("[") + 1 : dst.rindex("]")])
                if base not in ("rbp", "rsp") or idx:
                    return f"{src_cls} pointer escapes ({mn} {ops} at +0x{a:x})"
                slots[(base, disp)] = src_cls
            elif mn == "mov" and dst_r and src_cls and dst in _R64 and dst == dst_r:
                regs[dst_r] = src_cls
            elif mn == "cmp" and isinstance(dst, str) and "[" in dst and isinstance(src, str):
                base, _i, disp = _mem_base_disp(dst[dst.index("[") + 1 : dst.rindex("]")])
                if regs.get(base) == "rec" and disp == 0:
                    try:
                        flag = (_int(src) & 0xFFFFFFFF) == _AV_CODE
                    except ValueError:
                        flag = None
                else:
                    flag = None
                a = nxt_a
                continue
            elif mn in _SEH_JCC and isinstance(parts[0], int) and flag is not None:
                a = parts[0] if (flag == _SEH_JCC[mn]) else nxt_a
                flag = None
                continue
            elif mn.startswith("j") and mn != "jmp":
                if isinstance(parts[0], int):
                    work.append((parts[0], dict(regs), dict(slots), None))
                flag = None
                a = nxt_a
                continue
            elif mn == "jmp":
                if not isinstance(parts[0], int):
                    return f"indirect jmp at +0x{a:x}"
                a = parts[0]
                continue
            elif mn == "call":
                passed = {r: regs[r] for r in _GPR_ARGS if r in regs}
                if passed:
                    if not isinstance(parts[0], int) or depth >= 2:
                        return f"{'/'.join(sorted(set(passed.values())))} passed to call {ops} at +0x{a:x}"
                    why = seh_filter_observes(dis, parts[0], passed, depth + 1)
                    if why:
                        return why
                for r in ("rax", "rcx", "rdx", "r8", "r9", "r10", "r11"):
                    regs.pop(r, None)
            elif dst_r:
                regs.pop(dst_r, None)  # any other register write
            if mn not in ("cmp", "test"):
                flag = None
            if nxt_a is None:
                return f"falls off the listing at +0x{a:x}"
            a = nxt_a
    return None


_FH4_NEG = (-1, -2, -1, -3, -1, -2, -1, -4, -1, -2, -1, -3, -1, -2, -1, -5)
_FH4_SH = (25, 18, 25, 11, 25, 18, 25, 4, 25, 18, 25, 11, 25, 18, 25, 0)


def _fh4_ru(d, o):
    """FH4 compressed unsigned (`ReadUnsigned`) at file offset `o`: (value, next offset)."""
    i = d[o] & 15
    o2 = o - _FH4_NEG[i]
    return struct.unpack_from("<I", d, o2 - 4)[0] >> _FH4_SH[i], o2


def fh4_decode(pe: DllPE, fi_rva: int, fn_rva: int, frags, dis):
    """C++ EH facts of the FH4 function at rva `fn_rva` (FuncInfo4 at `fi_rva`, layout of
    vcruntime's ehdata4_export.h; `frags` = the function's rva ranges):
      shape: `catch_funclet` (FuncInfo of a catch funclet, header.isCatch), `try`, `cleanup`
             (unwind map entries, no try map) or `empty`;
      try_ranges: rva [lo, hi) ranges whose IP-to-state state lies in a try block's
             [tryLow, tryHigh] (None: not decodable, every call site counts as in a try);
      conts: catch continuation rvas (from the handler, or the `lea rax, [rip+X]` returns of
             the catch funclet when the handler carries none), [] if none could be found;
      n_unwind, n_try. Raises on malformed data."""
    d = pe.data
    o = pe.off(fi_rva)
    hdr = d[o]
    o += 1
    if hdr & 4:
        _, o = _fh4_ru(d, o)
    uw = tr = None
    if hdr & 8:
        uw = struct.unpack_from("<i", d, o)[0]
        o += 4
    if hdr & 16:
        tr = struct.unpack_from("<i", d, o)[0]
        o += 4
    ip2s = struct.unpack_from("<i", d, o)[0]
    n_uw = _fh4_ru(d, pe.off(uw))[0] if uw else 0
    # header.isCatch: a catch funclet's own FuncInfo (its unwind / try maps: nested tries in the
    # catch block). The trailing dispFrame (parent establisher slot) is read by
    # __CxxFrameHandler4 at run time from the funclet's establisher frame, which the lifted
    # body keeps in the guest layout; nothing here depends on it.
    res = {"shape": "empty", "try_ranges": [], "conts": [], "n_unwind": n_uw, "n_try": 0, "catch": bool(hdr & 1)}
    if not tr:
        res["shape"] = "cleanup" if n_uw else "empty"
        return res
    res["shape"] = "try"
    to = pe.off(tr)
    n_try, to = _fh4_ru(d, to)
    res["n_try"] = n_try
    tries, conts = [], set()
    in_fn = lambda x: any(b <= x < e for b, e in frags)
    for _ in range(n_try):
        lo, to = _fh4_ru(d, to)
        hi, to = _fh4_ru(d, to)
        _, to = _fh4_ru(d, to)  # catchHigh
        dh = struct.unpack_from("<i", d, to)[0]
        to += 4
        tries.append((lo, hi))
        ho = pe.off(dh)
        n_c, ho = _fh4_ru(d, ho)
        for _ in range(n_c):
            f4 = d[ho]
            ho += 1
            if f4 & 1:  # adjectives
                _, ho = _fh4_ru(d, ho)
            if f4 & 2:  # dispType
                ho += 4
            if f4 & 4:  # dispCatchObj
                _, ho = _fh4_ru(d, ho)
            hand = struct.unpack_from("<i", d, ho)[0]
            ho += 4
            n_ca = (f4 >> 4) & 3  # contAddr: 0 = what the funclet returns
            for _ in range(n_ca):
                if f4 & 8:  # contIsRVA
                    conts.add(struct.unpack_from("<i", d, ho)[0])
                    ho += 4
                else:
                    c, ho = _fh4_ru(d, ho)
                    conts.add(fn_rva + c)
            if not n_ca:
                # MSVC catch funclets end `lea rax, [rip+cont]; ...; ret`.
                for fb, fe in pe_frag_ranges(pe, hand) or [(hand, hand + 0x400)]:
                    for a, *_ in dis.range(fb, fe):
                        ao = pe.off(a)
                        if d[ao : ao + 3] == b"\x48\x8d\x05":
                            t = a + 7 + struct.unpack_from("<i", d, ao + 3)[0]
                            if in_fn(t):
                                conts.add(t)
    res["conts"] = sorted(conts)
    # IP-to-state: one table (function-relative IPs) or, separated code, one per segment.
    tables = []
    if hdr & 2:
        so = pe.off(ip2s)
        n_seg, so = _fh4_ru(d, so)
        for _ in range(n_seg):
            seg, disp = struct.unpack_from("<ii", d, so)
            so += 8
            tables.append((seg, disp))
    else:
        tables.append((fn_rva, ip2s))
    ranges = []
    for seg, disp in tables:
        seg_end = next((e for b, e in frags if b <= seg < e), None)
        if seg_end is None:
            res["try_ranges"] = None
            return res
        io = pe.off(disp)
        n, io = _fh4_ru(d, io)
        ents = []
        ip = 0
        for _ in range(n):
            dip, io = _fh4_ru(d, io)
            st, io = _fh4_ru(d, io)
            ip += dip
            ents.append((seg + ip, st - 1))
        for k, (a, st) in enumerate(ents):
            b = ents[k + 1][0] if k + 1 < len(ents) else seg_end
            if any(lo <= st <= hi for lo, hi in tries):
                ranges.append((a, b))
    res["try_ranges"] = ranges
    return res


_PDATA_FRAGS = {}


def pe_frag_ranges(pe: DllPE, r: int):
    """`.pdata` ranges [b, e) of the function whose primary begins at rva `r` (its own entry
    and chained fragments)."""
    key = id(pe)
    if key not in _PDATA_FRAGS:
        m = {}
        for b, e, u in pe.pdata():
            p, _, _ = _unwind_primary(pe, u)
            m.setdefault(p if p is not None else b, []).append((b, e))
        _PDATA_FRAGS[key] = m
    return _PDATA_FRAGS[key].get(r, [])


def special_vas(pe: DllPE):
    """CFG dispatch target(s), cookie-check/__chkstk VAs, and the guard fptr literal."""
    d = pe.data
    guard_dispatch, nop_calls, fptr_lit = set(), set(), None
    cookie_va = None
    rva, size = pe.dd[10] if len(pe.dd) > 10 else (0, 0)
    if rva and size >= 0x60:
        o = pe.off(rva)
        n = struct.unpack_from("<I", d, o)[0]
        cookie_va = struct.unpack_from("<Q", d, o + 0x58)[0] or None
        if n >= 0x80:
            disp_ptr = struct.unpack_from("<Q", d, o + 0x78)[0]
            if disp_ptr:
                tgt = pe.u64_at_va(disp_ptr)
                if tgt:
                    guard_dispatch.add(tgt)
                    fptr_lit = f"0x{tgt:x}"
    text_va = pe.image_base + pe.text_rva
    # __security_check_cookie: cmp rcx,[rip+cookie]; jne; rol rcx,16; test cx,0xffff; jne; ret
    for m in re.finditer(rb"\x48\x3b\x0d(....)\x75.\x48\xc1\xc1\x10\x66\xf7\xc1\xff\xff\x75.\xc3", pe.text, re.S):
        va = text_va + m.start()
        tgt = (va + 7 + struct.unpack("<i", m.group(1))[0]) & MASK64
        if cookie_va is None or tgt == cookie_va:
            nop_calls.add(va)
    # __chkstk / __alloca_probe
    for m in re.finditer(rb"\x48\x83\xec\x10\x4c\x89\x14\x24\x4c\x89\x5c\x24\x08\x4d\x33\xdb", pe.text):
        nop_calls.add(text_va + m.start())
    # Incremental-link thunks to them (`jmp rel32` at a 16-byte boundary, v8.dll): a call
    # through one is the same nop. Lifted as a real call it runs the thunk in the emulator,
    # which starts with rax = 0, and the (rax-preserving) cookie check then returns that 0
    # as the caller's rax (v8 +0x657bc0: `mov rax, rbx; ...; call thunk; ret`).
    if nop_calls:
        t = pe.text
        for i in range(0, len(t) - 5, 16):
            if t[i] == 0xE9:
                va = text_va + i
                if (va + 5 + struct.unpack_from("<i", t, i + 1)[0]) & MASK64 in nop_calls:
                    nop_calls.add(va)
    return frozenset(guard_dispatch), frozenset(nop_calls), fptr_lit


class _ByteScan:
    """`range(b, e)` = every byte offset as a pseudo-instruction start. `fh4_decode` only
    byte-matches `lea rax, [rip+X]` at the offsets it gets (catch funclets without encoded
    continuations); a whole-image listing of the exe would take minutes. A false match must
    also point into the parent function and be an instruction start of its body to become a
    block start, and then only adds a landing arm."""

    def range(self, b, e):
        return [(a, "", "", []) for a in range(b, e)]


def exe_eh_facts(image, base, vas, frag_map):
    """FH4 facts (as `load`'s `eh` entries) of the exe functions at `vas` (eh_mode 2: an own
    `__CxxFrameHandler4` / `__GSHandlerCheck_EH4` FuncInfo and a throwing callee), keyed by VA;
    plus counts per outcome. A function whose handler is another kind, whose FuncInfo4 does
    not decode or whose UNWIND_INFO gives no establisher gets no entry (counted by reason):
    it keeps lifting without a record, as before L5."""
    pe = DllPE(image, base)
    slots = {slot: fn for slot, _dll, fn in pe.imports()}
    by_root = {}
    for b, e, u in pe.pdata():
        p, fl, h = _unwind_primary(pe, u)
        by_root.setdefault(p if p is not None else b, []).append((b, e, u, fl, h))
    dis = _ByteScan()
    out, counts = {}, {}

    def count(k):
        counts[k] = counts.get(k, 0) + 1

    for va in sorted(vas):
        r = va - base
        ents = by_root.get(r)
        own = next((x for x in ents or [] if x[0] == r), None)
        if own is None:
            count("no_pdata_root")
            continue
        kinds = {handler_kind(pe, h, slots, {}) for _b, _e, _u, fl, h in ents if fl} - {"gs_check"}
        if not (len(kinds) == 1 and kinds <= {"fh4", "fh4_gs"}):
            count("handler_" + ("_".join(sorted(kinds)) or "none"))
            continue
        pu, hd = _unwind_eh(pe, own[2])
        if not hd:
            count("no_handler_data")
            continue
        frs = [(fva - base, fva - base + fsz) for fva, fsz in frag_map.get(va, [])] or [(b, e) for b, e, *_ in ents]
        try:
            fi_rva = struct.unpack_from("<I", pe.data, pe.off(hd))[0]
            x = fh4_decode(pe, fi_rva, r, frs, dis)
        except Exception:  # malformed / unexpected encoding
            x = None
        fr, eoff = unwind_frame(pe, pu) if x is not None else (None, None)
        if x is None or fr is None:
            count("fh4_undecoded")
            continue
        out[va] = {
            "mode": 2 if x["shape"] == "try" else 1,
            "handler": 1 if "fh4" in kinds else 2,
            "shape": {"empty": 0, "cleanup": 1, "try": 2}[x["shape"]],
            "funcinfo_rva": fi_rva,
            "frame_reg": fr,
            "establisher_off": eoff,
            "conts": [base + c for c in x["conts"]],
            "try_ranges": None if x["try_ranges"] is None else [(base + a, base + b) for a, b in x["try_ranges"]],
        }
        count("catch_funclet" if x["catch"] else x["shape"])
    return out, counts


def load(args, gs):
    """Configure gen_subsystems (`gs`) for the DLL and return its lifter inputs:
    (pe, all_rows, frag_map, fn_starts, info)."""
    base = int(args.base, 16)
    pe = DllPE(args.image, base)
    name = args.name or Path(args.image).name.lower()
    gd, nops, lit = special_vas(pe)
    gs.IMAGE_BASE = base
    gs.GUARD_DISPATCH_VAS = gd
    gs.NOP_CALL_VAS = nops
    gs.GUARD_FPTR_LIT = lit
    # Jump-table displacements are RVAs; a DLL's tables can sit below 0x100000 (renoircore
    # 0x5d6f4), which the exe pattern (>= 6 hex digits) misses. .text starts at 0x1000.
    gs.JT_DISP_RE = re.compile(gs.JT_DISP_RE.pattern.replace("{6,}", "{4,}"), gs.JT_DISP_RE.flags)
    # ... and live in any section (vcruntime140's memcpy tables are in .rdata); without the
    # table the targets are no block starts and the `jmp reg` falls back to a tail dispatch
    # that loses non-argument registers (rax = memcpy's return value).
    gs.READ_TABLE = pe.read_va
    # `int 0x29` (__fastfail: GS / security failure reporting) must terminate. With the trap
    # lowering (`lift_ops::trap`: STATUS_STACK_BUFFER_OVERRUN, no VEH, never returns) it lifts
    # exactly; the old lowering (AW2_GEN_TRAPS=0) turned `int` into a return, so there such
    # functions stay guest code.
    gs.EXTRA_BAD_MNEMS = frozenset() if gs.TRAPS else frozenset({"int"})
    # x86-exact `sbb r,r` flags and zero-source bsf/bsr (found by the dll_lift fuzz: memcmp
    # returned 0 for "less", memchr's not-found path returned 64).
    gs.EXACT_FLAGS = True
    # C++ EH functions (eh_lift_plan.md T1/T2): an `EhFrame` record with `eh.site` before every call in them, and the
    # catching `eh.call` form (continuation dispatch) for calls inside try ranges.
    gs.EH_SITES = True
    # The exe's per-function caps (550 instructions / 8 KiB) bound compile time over ~100k
    # functions; a DLL has few, and its hot code sits in big ones (cohtml 0x1396e0: 4057,
    # v8 0x1000b00: 5495; all 9 >4000 functions across the 8 DLLs fit under 6000).
    if "AW2_GEN_MAX_INSNS" not in os.environ:
        gs.MAX_INSNS = 25000
    if "AW2_GEN_MAX_BYTES" not in os.environ:
        gs.MAX_FN_BYTES = 131072

    exports = {}
    for _, frva, ename in pe.exports():
        if ename and pe.in_text(frva):
            exports.setdefault(frva, ename)

    ents = sorted(pe.pdata())
    all_slots = {slot: fn for slot, _dll, fn in pe.imports()}
    nr_slots = {slot for slot, fn in all_slots.items() if NORETURN_IMPORT_RE.match(fn)}
    if nr_slots:
        nr_vas = {base + slot for slot in nr_slots}
        t_code = pe.text_mapped
        pos = 0
        while True:
            pos = t_code.find(b"\xff\x25", pos)
            if pos == -1 or pos + 6 > len(t_code):
                break
            slot = pe.text_rva + pos + 6 + struct.unpack_from("<i", t_code, pos + 2)[0]
            if slot in nr_slots:
                nr_vas.add(base + pe.text_rva + pos)
                if pos > 0 and t_code[pos - 1] == 0x48:
                    nr_vas.add(base + pe.text_rva + pos - 1)
            pos += 2
        gs.NORETURN_CALL_VAS = frozenset(nr_vas)
    else:
        gs.NORETURN_CALL_VAS = frozenset()
    root_of, handler = {}, {}  # handler: root rva -> set of handler kinds (handler_kind)
    ehd = {}  # root rva -> (primary UNWIND_INFO rva, handler-data rva)
    unwind_of = {}  # .pdata begin rva -> its UNWIND_INFO rva
    for b, e, u in ents:
        if not (pe.in_text(b) and b < e):
            continue
        unwind_of.setdefault(b, u)
        p, fl, h = _unwind_primary(pe, u)
        r = p if p is not None else b
        root_of[(b, e)] = r
        if fl:
            handler.setdefault(r, set()).add(handler_kind(pe, h, all_slots, exports))
            ehd.setdefault(r, _unwind_eh(pe, u))
    # Structural non-ABI embedded code range (V8 embedded builtins: handler
    # CrashForExceptionInNonABICompliantCodeRange). Frameless builtins inside that contiguous
    # blob lack .pdata entries yet still use V8's non-Win64 register convention (r13 roots,
    # r14 cage); when no regular .pdata span overlaps [min_b, max_e), any pdata-less target
    # inside [min_b, max_e) is structurally non-ABI too.
    nonabi_spans = [
        (b, e)
        for (b, e), r in root_of.items()
        if "v8_nonabi" in (handler.get(r, set()) - {"gs_check"})
    ]
    nonabi_range = None
    if nonabi_spans:
        na_lo = min(b for b, _ in nonabi_spans)
        na_hi = max(e for _, e in nonabi_spans)
        if all(
            "v8_nonabi" in (handler.get(r, set()) - {"gs_check"})
            for (b, e), r in root_of.items()
            if b < na_hi and e > na_lo
        ):
            nonabi_range = (na_lo, na_hi)

    def in_nonabi_range(t):
        return nonabi_range is not None and nonabi_range[0] <= t < nonabi_range[1]

    frags = {}
    for (b, e), r in root_of.items():
        frags.setdefault(r, []).append((b, e))
    roots = {r for r in frags if any(b == r for b, _ in frags[r])}
    starts_sorted = sorted({b for b, _ in root_of} | set(exports))

    fn_starts = {base + r for r in roots} | {base + x for x in exports}
    # Exported leaf functions (no .pdata): extent up to the next known start (the lifter
    # rejects extents that do not end in a terminator followed by padding).
    covered = sorted(root_of)
    pdata_ends = {e for _, e in covered}
    import bisect

    cstarts = [b for b, _ in covered]
    leaf_ext = {}
    for x in sorted(exports):
        i = bisect.bisect_right(cstarts, x) - 1
        if i >= 0 and covered[i][0] <= x < covered[i][1]:
            continue
        if in_nonabi_range(x):
            continue
        j = bisect.bisect_right(starts_sorted, x)
        nxt = starts_sorted[j] if j < len(starts_sorted) else pe.text_end
        ext = min(nxt, x + 512, pe.text_end) - x
        if ext > 0 and not CONTEXT_EXPORT_RE.search(exports[x]):
            leaf_ext[x] = ext
    # Max incoming stack slot of every function (whole-image listing, so functions that never
    # become candidates count too): gen_subsystems' residual caller rules look targets up here.
    dis = _Disasm(args.image, pe.file_base)
    regfile_res = None
    regfile_lifted_rvas = set()
    regfile_rej_rvas = set()
    regfile_spans = []
    if nonabi_range is not None:
        import v8_regfile

        regfile_res = v8_regfile.lift_nonabi(pe, dis, base, nonabi_range[0], nonabi_range[1])
        regfile_lifted_rvas = {rva for _va, rva, _ilen, _bid, _code, _conts in regfile_res["lifted"]}
        regfile_rej_rvas = {rva for _va, rva, _bid, _why in regfile_res["rejected"]}
        regfile_spans = [(rva - 4, rva + ilen) for _va, rva, ilen, *_ in regfile_res["lifted"]]
        n_conts = sum(len(conts) for *_rest, conts in regfile_res["lifted"])
        print(
            f"  v8_regfile: {regfile_res['total']} embedded builtins in +0x{nonabi_range[0]:x}..+0x{nonabi_range[1]:x} -> "
            f"{len(regfile_res['lifted'])} lifted (+{n_conts} conts, DF_REGFILE), {len(regfile_res['rejected'])} kept guest"
        )
    fn_frags = {r: sorted(frags[r], key=lambda f: (f[0] != r, f[0])) for r in roots}
    fn_frags.update({x: [(x, x + ext)] for x, ext in leaf_ext.items()})
    infos = {r: stack_arg_info(dis, fr) for r, fr in fn_frags.items()}
    slot_max = {base + r: i["max_slot"] for r, i in infos.items() if i["max_slot"] >= STACK_ARG_SLOT_LIMIT}
    frag_map, all_rows = {}, []
    skipped_eh = skipped_ctx = 0
    eh_kinds = {}  # kind -> roots kept as guest code because of their handler
    eh = {}  # va -> EH facts of a lifted-candidate FH4 function (EH_META, generator sites)
    eh_shapes = {}  # FH4 shape -> candidates
    ctx_slots = {slot: fn for slot, _dll, fn in pe.imports() if CONTEXT_IMPORT_RE.match(fn)}
    bt_slots = {slot: fn for slot, _dll, fn in pe.imports() if BACKTRACE_IMPORT_RE.match(fn)}
    pre_rejected = {}  # va -> reason (reported as rejected, see emit)
    for r in sorted(roots):
        kinds = handler.get(r, set()) - {"gs_check"}
        if "v8_nonabi" in kinds:
            if (
                r not in regfile_lifted_rvas
                and r not in regfile_rej_rvas
                and not any(lo <= r < hi for lo, hi in regfile_spans)
            ):
                pre_rejected[base + r] = (
                    "v8_nonabi: V8 embedded builtin (handler CrashForExceptionInNonABICompliantCodeRange), "
                    "inputs in V8 registers (r13 roots, rax/rbx/rdi argc/target/new.target) outside the Win64 entry"
                )
            continue
        if kinds and len(kinds) == 1 and kinds <= {"fh4", "fh4_gs"} and ehd.get(r, (None, None))[1]:
            # C++ EH (eh_lift_plan.md T1/T2): the normal path is lifted. A throw crossing the
            # frame is fatal on aarch64 with or without the lift until T3; the EH_META record
            # and the generator's `eh.site` / `eh.call` sites are what T3 runs on.
            pu, hd = ehd[r]
            try:
                fi_rva = struct.unpack_from("<I", pe.data, pe.off(hd))[0]
                x = fh4_decode(pe, fi_rva, r, frags[r], dis)
            except Exception:  # malformed / unexpected encoding
                x = None
            fr, eoff = unwind_frame(pe, pu) if x is not None else (None, None)
            if x is None or fr is None:
                eh_kinds["fh4_undecoded"] = eh_kinds.get("fh4_undecoded", 0) + 1
                skipped_eh += 1
                continue
            shape = x["shape"]
            if x["catch"]:
                eh_shapes["catch_funclet"] = eh_shapes.get("catch_funclet", 0) + 1
            eh[base + r] = {
                "mode": 2 if shape == "try" else 1,
                "handler": 1 if "fh4" in kinds else 2,
                "shape": {"empty": 0, "cleanup": 1, "try": 2}[shape],
                "funcinfo_rva": fi_rva,
                "frame_reg": fr,
                "establisher_off": eoff,
                "conts": [base + c for c in x["conts"]],
                "try_ranges": None if x["try_ranges"] is None else [(base + a, base + b) for a, b in x["try_ranges"]],
            }
            eh_shapes[shape] = eh_shapes.get(shape, 0) + 1
            kinds = set()
        if kinds == {"seh_scope"} and ehd.get(r, (None, None))[1] and not (
            exports.get(r) and CONTEXT_EXPORT_RE.search(exports[r])
        ):
            # SEH (eh_lift_plan.md T3): the guest __C_specific_handler runs on the lifted frame
            # (ControlPc = the in-flight call) and unwinds to an `__except` JumpTarget with
            # rax = the exception code (`landed_rax`); every call catches.
            pu, hd = ehd[r]
            fr, eoff = unwind_frame(pe, pu)
            sc, why, guards = seh_scopes(pe, hd, frags[r], dis, fr or 0) if fr is not None else (None, "seh_unwind_frame", None)
            if sc is None:
                pre_rejected[base + r] = f"{why}: SEH root kept as guest code"
                eh_kinds[why.split(" ")[0]] = eh_kinds.get(why.split(" ")[0], 0) + 1
                skipped_eh += 1
                continue
            sconts = sorted({base + j for _, _, _, j in sc if j})
            eh[base + r] = {
                "mode": 1, "handler": 3, "shape": 0, "funcinfo_rva": hd,
                "frame_reg": fr, "establisher_off": eoff, "conts": sconts, "try_ranges": None,
                "seh_conts": sconts, "seh_guards": [base + a for a in guards],
            }
            k = "seh_except" if sconts else "seh_finally"
            eh_shapes[k] = eh_shapes.get(k, 0) + 1
            if guards:
                eh_shapes["seh_guarded"] = eh_shapes.get("seh_guarded", 0) + 1
            kinds = set()
        if kinds:
            k = (
                "seh_startup" if "seh_startup" in kinds
                else "seh_context_export" if kinds == {"seh_scope"}
                else "seh_mixed" if "seh_scope" in kinds
                else "fh3" if "fh3" in kinds
                else "fh4_mixed" if kinds & {"fh4", "fh4_gs"}
                else "other"
            )
            eh_kinds[k] = eh_kinds.get(k, 0) + 1
            skipped_eh += 1
            continue
        if exports.get(r) and CONTEXT_EXPORT_RE.search(exports[r]):
            skipped_ctx += 1
            continue
        imp = context_import_refs(pe, frags[r], ctx_slots)
        if imp:
            sj = setjmp_sites(pe, frags[r], ctx_slots, dis)
            fr, eoff = unwind_frame(pe, unwind_of[r]) if sj else (None, None)
            if fr is None:
                pre_rejected[base + r] = f"calls {imp} (would capture/unwind native machine state)"
                continue
            # setjmp caller (eh_lift_plan.md T3): an EH record whose continuations are the
            # setjmp return addresses (`EhFrame::setjmp` at each call); every call catches.
            x = eh.setdefault(base + r, {
                "mode": 1, "handler": 0, "shape": 0, "funcinfo_rva": 0,
                "frame_reg": fr, "establisher_off": eoff, "conts": [], "try_ranges": None,
            })
            x["sj_sites"] = [base + a for a, _ in sj]
            x["sj_conts"] = [base + n for _, n in sj]
            x["conts"] = sorted(set(x["conts"]) | set(x["sj_conts"]))
            x["try_ranges"] = None
            eh_shapes["setjmp"] = eh_shapes.get("setjmp", 0) + 1
        if context_import_refs(pe, frags[r], bt_slots):
            # Stack-backtrace caller: RtlCaptureStackBackTrace walks the EH record chain
            # (aw2-win32 `backtrace_chain`); frame 0 is this function, visible only with a
            # record (handler 0, no catching call; the state IP names the in-flight call).
            fr, eoff = unwind_frame(pe, unwind_of[r]) if r in unwind_of else (None, None)
            if fr is None:
                eh_shapes["backtrace_unrecorded"] = eh_shapes.get("backtrace_unrecorded", 0) + 1
            else:
                eh.setdefault(base + r, {
                    "mode": 1, "handler": 0, "shape": 0, "funcinfo_rva": 0,
                    "frame_reg": fr, "establisher_off": eoff, "conts": [], "try_ranges": [],
                })
                eh_shapes["backtrace"] = eh_shapes.get("backtrace", 0) + 1
        if infos[r]["stack_pos"]:
            pre_rejected[base + r] = stack_pos_reason(infos[r])
            continue
        own = next(e for b, e in frags[r] if b == r)
        total = sum(e - b for b, e in frags[r])
        frag_map[base + r] = [(base + b, e - b) for b, e in sorted(frags[r])]
        all_rows.append((base + r, own - r, total, 0, None, "dll", None, exports.get(r)))
    # Internal leaf functions: direct `call rel32` targets of the roots above that lie outside
    # every .pdata range and are not exports. x64 requires unwind info for any function that
    # moves rsp or saves a nonvolatile register, so these are leaves (renoircore +0x14e700, a
    # 10-instruction rounding helper called ~10^8 times from +0x160c60's loop). Kept guest,
    # each call from lifted code is a whole emulator entry. Same extent rule as exported
    # leaves; `callers` restricts --rva-list images to leaves a listed root calls.
    # A pure `jmp rel32` thunk (v8's incremental-link jump stubs: +0x23790 -> +0x4d7c40) is no
    # leaf: lifted as one, its target ran from the thunk's lifted frame at another stack depth
    # than in the guest run (stack-address results differed). `call thunk` == `call target`,
    # so the generator calls the final target directly (`gs.CALL_REDIRECT`, also for tail
    # jmps and conditional tail jmps) and the thunk stays guest code; no emulator entry per
    # call either. A thunk is the `jmp` alone: int3 padding, the next stub's `jmp` or a
    # `jmp [rip+IAT]` import stub follows it (vcruntime140 +0x10c60 `jmp +0` onto the RtlUnwind
    # stub at +0x10c65). A `jmp` followed by code is a function's rotated-loop entry (msvcp140 +0x1702c:
    # `jmp +0x17046` over the loop body), not a thunk: redirecting it made the leaf start at
    # the loop test with the back edge outside its extent.
    def pure_thunk(x):
        ins = dis.range(x, x + 8)
        if ins and ins[0][0] == x and ins[0][1] == "jmp" and ins[0][3] and isinstance(ins[0][3][0], int):
            o = pe.off(x)
            n = {0xE9: 5, 0xEB: 2}.get(pe.data[o]) if o is not None else None
            if n is None or not (pe.data[o + n] in (0xCC, 0xE9) or import_stub(x + n)):
                return None
            return ins[0][3][0]
        return None

    redirect = {}

    def resolve(x):
        seen = [x]
        while x not in frags and x not in exports and len(seen) <= 8:
            y = pure_thunk(x)
            if y is None or y in seen:
                break
            x = y
            seen.append(x)
        for s in seen[:-1]:
            redirect[base + s] = base + x
        return x

    iat_rva, iat_sz = pe.dd[12] if len(pe.dd) > 12 else (0, 0)

    def import_stub(t):
        to = pe.off(t)
        if to is None:
            return False
        if pe.data[to : to + 2] == b"\xff\x25":
            ln = 6
        elif pe.data[to : to + 3] == b"\x48\xff\x25":
            ln = 7
        else:
            return False
        if to + ln > len(pe.data):
            return False
        slot = t + ln + struct.unpack_from("<i", pe.data, to + ln - 4)[0]
        return slot in all_slots or (
            iat_sz >= 8 and iat_rva <= slot and slot + 8 <= iat_rva + iat_sz and (slot & 7) == 0
        )

    def in_pdata(t):
        i = bisect.bisect_right(cstarts, t) - 1
        return i >= 0 and covered[i][0] <= t < covered[i][1]

    # Data-referenced leaves (`--dataref-leaves`, next_frontier step 3): address-taken code that
    # no .pdata range covers and no direct call reaches: vtable slots of leaf methods (cohtml
    # +0x3fa0f0.. `mov rcx,[rcx+8]; mov r8d,0xb; mov rax,[rcx]; jmp [rax+N]` forwarders, 10% of
    # the g21 steady-state guest instructions), this-adjustor thunks packed at 4-byte alignment
    # (`sub rcx,0xd8; jmp X`), ICF'd `mov al,1; ret` stubs. Sources: every DIR64 relocation site
    # outside .text whose value points into .text (vtables / function-pointer tables) and the CFG
    # GuardCFFunctionTable. A data pointer is a function entry by itself, so no alignment /
    # padding rule (the jcc rule's evidence); refused, named: `dataref_import_stub` (a
    # `jmp [rip+IAT]` stub: stays as it is, resolved per call), `dataref_jmp_thunk` (a lone
    # `jmp rel32`: stays guest, its target is lifted on its own), `dataref_in_padding` (an int3
    # byte), `dataref_listing_desync` (no instruction start at the target even after a fresh
    # decode from it). The whole-image listing is a linear sweep that data bytes in .text can
    # desynchronise (cohtml +0x49df00 `or [rcx+0x668],edx; ret` after a byte table): a target
    # the sweep missed is re-decoded from the entry (`_Disasm.resync`) first. Targets inside a .pdata range are
    # blocks of pdata functions (counted, not entries).
    def import_stub_chain(t):
        cur = t
        for _ in range(4):
            y = pure_thunk(cur)
            if y is None or not pe.in_text(y) or y == cur:
                break
            cur = y
        return import_stub(cur)

    dataref = {}  # rva -> reference count
    dataref_bounds = set()  # all valid dataref starts (bounds extents even under --dataref-listed)
    dataref_refused = {}  # rva -> reason
    dataref_interior = 0
    dataref_resynced = []  # targets the linear sweep missed, re-decoded from the entry
    if getattr(args, "dataref_leaves", False):
        refs = {}
        for site in pe.reloc_sites:
            if pe.in_text(site):
                continue
            v = pe.u64_at_va(base + site)
            if v is None:
                continue
            t = v - base
            if 0 <= t < 1 << 32 and pe.in_text(t):
                refs[t] = refs.get(t, 0) + 1
        has_gfids = False
        lc = pe.dd[10] if len(pe.dd) > 10 else (0, 0)
        if lc[0]:
            lo = pe.off(lc[0])
            if struct.unpack_from("<I", pe.data, lo)[0] >= 0x94:
                tab, cnt, gflags = struct.unpack_from("<QQI", pe.data, lo + 0x80)
                if tab and cnt:
                    has_gfids = True
                    stride = 4 + ((gflags >> 28) & 0xF)
                    to = pe.off(tab - base)
                    for k in range(cnt):
                        t = struct.unpack_from("<I", pe.data, to + stride * k)[0]
                        if pe.in_text(t):
                            refs.setdefault(t, 0)
        if not has_gfids:
            # Non-CFG images (renoircore, cohtml) have an empty GuardCFFunctionTable, so callbacks
            # whose address is taken only via RIP-relative `lea reg, [rip +- disp32]` in .text are
            # absent from `reloc_sites` (renoircore +0x384e0, +0x11bf90). Accept 16-aligned .text
            # targets outside .pdata that follow a function terminator (`int3`, `.pdata` end, or
            # preceding `ret`/`jmp`/`ud2`/`int3`).
            lea_re = re.compile(r"\[rip ([+-]) 0x([0-9a-f]+)\]")
            for b, e in covered:
                ins_list = dis.range(b, e)
                for idx_i, (a, mn, ops, _parts) in enumerate(ins_list):
                    if mn != "lea":
                        continue
                    m = lea_re.search(ops)
                    if not m:
                        continue
                    if idx_i + 1 < len(ins_list):
                        ln = ins_list[idx_i + 1][0] - a
                    else:
                        nxt_i = dis.range(a + 1, a + 16)
                        if not nxt_i:
                            continue
                        ln = nxt_i[0][0] - a
                    disp = int(m.group(2), 16) * (1 if m.group(1) == "+" else -1)
                    t = a + ln + disp
                    if not (pe.in_text(t) and t % 16 == 0) or t in frags or t in exports or in_pdata(t):
                        continue
                    to = pe.off(t)
                    if to is None or pe.data[to] == 0xCC:
                        continue
                    prev_ok = pe.data[to - 1] == 0xCC or t in pdata_ends
                    if not prev_ok:
                        pi = bisect.bisect_left(dis.addrs, t) - 1
                        if pi >= 0 and dis.recs[pi][0] in ("ret", "jmp", "ud2", "int3"):
                            prev_ok = True
                    if prev_ok:
                        refs[t] = refs.get(t, 0) + 1
        rva_listed_set = (
            {int(x, 16) for x in Path(args.rva_list).read_text().split() if x.strip()}
            if getattr(args, "rva_list", None) and getattr(args, "dataref_listed", False)
            else None
        )
        for t in sorted(refs):
            if t in frags or t in exports:
                continue
            if in_pdata(t):
                dataref_interior += 1
                continue
            to = pe.off(t)
            if to is None or pe.data[to] == 0xCC:
                dataref_refused[t] = "dataref_in_padding"
                continue
            if in_nonabi_range(t):
                dataref_refused[t] = "dataref_v8_nonabi"
                continue
            if import_stub_chain(t):
                dataref_refused[t] = "dataref_import_stub"
                continue
            dataref_bounds.add(t)
            if rva_listed_set is not None and t not in rva_listed_set:
                continue
            ins = dis.range(t, t + 16)
            if not ins or ins[0][0] != t:
                # Fresh decode from the entry up to the next .pdata start, spliced into the
                # listing (`_Disasm.resync`); the rest of the listing is unchanged.
                k = bisect.bisect_right(cstarts, t)
                nxt = cstarts[k] if k < len(cstarts) else pe.text_end
                if dis.resync(t, min(nxt, t + 16384)):
                    dataref_resynced.append(t)
                    ins = dis.range(t, t + 16)
            if not ins or ins[0][0] != t:
                dataref_refused[t] = "dataref_listing_desync"
            else:
                dataref[t] = refs[t]
        rc = {}
        for why in dataref_refused.values():
            rc[why] = rc.get(why, 0) + 1
        print(
            f"  dataref leaves: {len(refs)} address-taken .text targets -> {len(dataref)} leaf starts "
            f"({sum(1 for t in dataref if t % 16)} unaligned, {len(dataref_resynced)} resynced), "
            f"{dataref_interior} inside .pdata ranges, refused {rc}"
        )
        for t in dataref_resynced:
            print(f"    dataref_resynced +0x{t:x}")
        for t, why in sorted(dataref_refused.items()):
            if why not in ("dataref_import_stub", "dataref_v8_nonabi", "dataref_in_padding"):
                print(f"    {why} +0x{t:x}")

    callers = {}
    row_roots = {va - base for va, *_ in all_rows}
    # jcc targets the leaf rule refused (named; gen_subsystems rejects the jcc's fn with it):
    # `leaf_unaligned` after int3 padding but not 16-aligned, `leaf_no_pad` not after int3
    # (a block of another function or data), `leaf_in_padding` an int3 byte itself,
    # `v8_nonabi_leaf` inside the non-ABI embedded code range.
    # VA -> (reason, owner VAs).
    leaf_rejects = {}
    leaf_call_skips = set()  # call targets in leaves (stay guest calls, not new leaves)

    def reach(x, ext):
        """Instructions of the pdata-less leaf at `x` reachable from its entry within the
        provisional extent `ext` (recursive descent: stops at ret / jmp / int3 / ud2 /
        indirect jmp), so padding, jump tables and the next function are not decoded."""
        out, seen, work = [], set(), [x]
        while work:
            a = work.pop()
            while x <= a < x + ext and a not in seen:
                ins = dis.range(a, min(a + 16, x + ext))
                if not ins or ins[0][0] != a:
                    break
                seen.add(a)
                i0 = ins[0]
                out.append(i0)
                mn, parts = i0[1], i0[3]
                t = parts[0] if parts and isinstance(parts[0], int) else None
                if mn in ("ret", "int3", "ud2") or (mn == "jmp" and t is None):
                    break
                if mn.startswith("j") and t is not None and x <= t < x + ext:
                    work.append(t)
                if mn == "jmp":
                    break
                nxt = ins[1][0] if len(ins) > 1 else None
                if nxt is None:
                    break
                a = nxt
        return out

    def ensure_decoded(t):
        to = pe.off(t)
        ins = dis.range(t, t + 16)
        if to is not None and (not ins or ins[0][0] != t):
            k = bisect.bisect_right(cstarts, t)
            nxt = cstarts[k] if k < len(cstarts) else pe.text_end
            dis.resync(t, min(nxt, t + 16384))

    def scan(owner, frs, roots_of, insns=None):
        """Direct call / jmp / jcc targets of the function `owner` (rva fragments `frs`; a
        leaf passes its reachable `insns` and gets no new call-target leaves): thunk chains
        into `redirect`; new internal leaves into `callers` (returned)."""
        new = []
        owner_is_thunk = pure_thunk(owner) is not None
        it = insns if insns is not None else (i for b, e in frs for i in dis.range(b, e))
        for _a, mn, _ops, parts in it:
            if not mn.startswith("j") and mn != "call":
                continue
            if not parts or not isinstance(parts[0], int):
                continue
            t = parts[0]
            if any(fb <= t < fe for fb, fe in frs):
                continue
            if pe.in_text(t):
                t = resolve(t)
            if t in leaf_ext:
                prev_len = len(callers.get(t, ()))
                callers.setdefault(t, set()).update(roots_of)
                if len(callers[t]) > prev_len:
                    new.append(t)
                continue
            if t in exports or t in frags or not pe.in_text(t):
                continue
            if base + t in gs.NOP_CALL_VAS or base + t in gs.GUARD_DISPATCH_VAS:
                continue
            # `jmp [rip+IAT]` import stubs (and ILT chains to them) stay resolved per call.
            if import_stub_chain(t) or in_pdata(t):
                continue
            if in_nonabi_range(t):
                if mn != "call" and mn != "jmp":
                    leaf_rejects.setdefault(base + t, ("v8_nonabi_leaf", set()))[1].add(base + owner)
                continue
            to = pe.off(t)
            if mn == "jmp":
                # Pure `jmp rel32` ILT stubs (e.g. v8 exported ILT thunks +0x5ae10 -> +0x2e6c40,
                # +0x96570 -> +0x4eed40, +0x3f410 -> +0x1090084) jump straight to a real function
                # entry; other external tail jmps (`+0x4d02a0 -> +0x4dae0 -> +0x55a870`) add a
                # pdata-less leaf when the target lies at a clean function boundary.
                if to is None or pe.data[to] == 0xCC:
                    continue
                if not owner_is_thunk and not ((pe.data[to - 1] == 0xCC and t % 16 == 0) or t in pdata_ends):
                    continue
                ensure_decoded(t)
            elif mn != "call" or insns is not None:
                # Conditional tail jmp (frontier_plan.md §1c): its target is a leaf start only
                # at a function boundary, 16-aligned after int3 padding (renoircore +0x14e720
                # `je` -> +0x14fe40, the unicode charmap finder); else it is a block of another
                # function, or a decode of data (renoircore +0xb6fd0's in-extent jump tables
                # gave +0xb7f3a / +0xb7f3e). A leaf's calls add no leaves (as before: they
                # stay guest calls).
                if mn == "call":
                    leaf_call_skips.add(t)
                    continue
                if to is None or pe.data[to - 1] != 0xCC or t % 16 != 0 or pe.data[to] == 0xCC:
                    why = (
                        "leaf_no_pad" if to is None or pe.data[to - 1] != 0xCC
                        else "leaf_unaligned" if t % 16 != 0
                        else "leaf_in_padding"  # int3 padding itself (v8 +0x6977e0, +0x7156f0)
                    )
                    leaf_rejects.setdefault(base + t, (why, set()))[1].add(base + owner)
                    continue
                ensure_decoded(t)
            else:
                ensure_decoded(t)
            prev_len = len(callers.get(t, ()))
            callers.setdefault(t, set()).update(roots_of)
            if len(callers[t]) > prev_len:
                new.append(t)
        return new

    for r in sorted(row_roots):
        scan(r, frags[r], {r})
    # pdata-less leaves (exported, then internal ones as found) have direct targets too
    # (v8's exported ILT stubs and internal leaves ending in tail/conditional jmps): scan only
    # their reachable instructions (`reach`) within a provisional extent (next known start).
    work = sorted(set(leaf_ext) | set(callers) | set(dataref))
    scanned_roots = {}
    for _round in range(8):
        if not work:
            break
        bounds = sorted(set(starts_sorted) | set(callers) | set(leaf_ext) | set(dataref) | set(dataref_bounds) | set(dataref_refused))
        nxt_work = []
        for x in work:
            r_of = set(callers.get(x, ()))
            if x in exports or x in dataref or not r_of:
                r_of.add(x)
            if scanned_roots.get(x) == r_of:
                continue
            scanned_roots[x] = set(r_of)
            j = bisect.bisect_right(bounds, x)
            nxt = bounds[j] if j < len(bounds) else pe.text_end
            ext = leaf_ext.get(x) or (min(nxt, x + 16384, pe.text_end) - x)
            if ext > 0:
                nxt_work += scan(x, [(x, x + ext)], r_of, reach(x, ext))
        work = sorted(set(nxt_work))
    gs.CALL_REDIRECT = redirect
    if callers or dataref:
        bounds = sorted(set(starts_sorted) | set(callers) | set(dataref) | set(dataref_bounds) | set(dataref_refused))
        for x in sorted(set(callers) | set(dataref)):
            if x in leaf_ext:
                continue
            j = bisect.bisect_right(bounds, x)
            nxt = bounds[j] if j < len(bounds) else pe.text_end
            # Bounded by the next known start; the 4 KiB cap only guards the last function.
            # (512 as for exports cut renoir +0x13c950, whose switch default is at +0x212.)
            ext = min(nxt, x + 16384, pe.text_end) - x
            if ext > 0:
                leaf_ext[x] = ext
                fn_frags[x] = [(x, x + ext)]
                infos[x] = stack_arg_info(dis, fn_frags[x])
                if infos[x]["max_slot"] >= STACK_ARG_SLOT_LIMIT:
                    slot_max[base + x] = infos[x]["max_slot"]
    internal = sum(1 for x in leaf_ext if x not in exports)
    # Conditional tail jmp targets the generator accepts besides pdata roots / exports
    # (`gs.LEAF_STARTS`, gen_subsystems' jcc pre-check): every pdata-less leaf start.
    gs.LEAF_STARTS = frozenset(base + x for x in leaf_ext)
    fn_starts |= gs.LEAF_STARTS
    gs.LEAF_REJECTS = {va: why for va, (why, _) in leaf_rejects.items() if va not in gs.LEAF_STARTS}
    print(f"  leaf rule: {len(gs.LEAF_REJECTS)} jcc target(s) refused, {len(leaf_call_skips)} leaf call target(s) left guest")
    for va, (why, owners) in sorted(leaf_rejects.items()):
        if va in gs.LEAF_REJECTS:
            print(f"    {why} 0x{va:x} (+0x{va - base:x}) from " + " ".join(f"+0x{o - base:x}" for o in sorted(owners)))
    leaves = 0
    for x, ext in sorted(leaf_ext.items()):
        r_ins = reach(x, ext)
        has_ind_jmp = any(
            mn == "jmp" and not (parts and isinstance(parts[0], int))
            for _a, mn, _ops, parts in r_ins
        )
        if r_ins and not has_ind_jmp:
            max_a = max(i[0] for i in r_ins)
            ni = bisect.bisect_right(dis.addrs, max_a)
            chk_end = min(x + ext, dis.addrs[ni] if ni < len(dis.addrs) else x + ext)
        else:
            chk_end = x + ext
        imp = context_import_refs(pe, [(x, chk_end)], ctx_slots)
        if imp:
            pre_rejected[base + x] = f"calls {imp} (would capture/unwind native machine state)"
            continue
        if infos[x]["stack_pos"]:
            pre_rejected[base + x] = stack_pos_reason(infos[x])
            continue
        all_rows.append((base + x, ext, ext, 0, None, "dll", None, exports.get(x)))
        leaves += 1
    all_rows.sort()
    if getattr(args, "rva_list", None):
        subset = {int(x, 16) for x in Path(args.rva_list).read_text().split() if x.strip()}
        if not getattr(args, "dataref_listed", False):
            # Data-referenced leaves have no direct caller to follow: kept (with --dataref-leaves).
            subset |= set(dataref)
        # Internal leaves (and exported/dataref thunks' targets) follow their callers: kept when
        # a listed root/thunk/dataref leaf reaches them.
        subset |= {x for x, rs in callers.items() if rs & subset}
        all_rows = [r for r in all_rows if r[0] - base in subset]
        pre_rejected = {va: why for va, why in pre_rejected.items() if va - base in subset}
        print(f"{name}: --rva-list: {len(all_rows)} of {len(subset)} listed functions are candidates")
    # (No per-VA skip list: a difftest mismatch is fixed in the generator / harness, or the
    # function falls under a detection rule such as the ones below and above.)
    # A tail `jmp` thunk (optionally after register-only argument setup, see
    # `tail_thunk_target`) into a function that stays guest (EH body, rejected, not listed):
    # lifting it gains nothing and runs the target from a lifted frame at another emulator
    # stack depth, where the target's uninitialised locals hold other stale bytes
    # (v8_libplatform +0x2870: a std::string SSO byte; msvcp140 +0x5710 ctype<char>(): the
    # padding of the _Ctypevec temporary). Only known function starts count as targets (pdata
    # roots, discovered leaves): a thunk into an import stub stays lifted. Iterated: a thunk to
    # a dropped thunk is dropped.
    known_starts = set(roots) | set(leaf_ext) | {x for x in exports if not import_stub_chain(x)}
    thunk_targets = {}
    for rva in known_starts:
        t = tail_thunk_target(dis, rva)
        if t is not None:
            t_res = resolve(t) if pe.in_text(t) else t
            if t_res in known_starts or in_nonabi_range(t_res):
                thunk_targets[base + rva] = base + t_res
            elif t in known_starts or in_nonabi_range(t):
                thunk_targets[base + rva] = base + t
    while True:
        lifted_rvas = {r[0] - base for r in all_rows}
        drop = {}
        for r in all_rows:
            t_va = thunk_targets.get(r[0])
            if t_va is not None and (t_va - base) not in lifted_rvas:
                drop[r[0]] = f"thunk to guest-only target +0x{t_va - base:x} (a lone tail jmp: nothing to lift)"
        if not drop:
            break
        pre_rejected.update(drop)
        all_rows = [r for r in all_rows if r[0] not in drop]
        print(f"{name}: {len(drop)} tail thunk(s) to guest-only targets kept as guest code")
    info = {
        "name": name,
        "base": base,
        "text_rva": pe.text_rva,
        "text_len": len(pe.text_mapped),
        "text_fnv": fnv1a(pe.text_mapped),
        "roots": len(roots),
        "skipped_eh": skipped_eh,
        "eh_kinds": eh_kinds,
        "eh": eh,
        "eh_shapes": eh_shapes,
        "skipped_ctx": skipped_ctx,
        "leaves": leaves,
        "internal_leaves": internal,
        "relocs": pe.n_relocs,
        "pe": pe,
        "pre_rejected": pre_rejected,
        "slot_max": slot_max,
        "thunk_targets": thunk_targets,
        "regfile": regfile_res,
    }
    print(
        f"{name}: base 0x{base:x} (file 0x{pe.file_base:x}, {pe.n_relocs} relocs), .text 0x{pe.text_rva:x}+0x{info['text_len']:x} "
        f"fnv 0x{info['text_fnv']:016x}; {len(roots)} pdata roots ({skipped_eh} with handlers {eh_kinds} kept guest, FH4 candidates {eh_shapes}, {skipped_ctx} context exports skipped, "
        f"{sum(w.startswith(('calls ', 'stack-position')) for w in pre_rejected.values())} context-import / stack-position rejections, {len(slot_max)} arg{STACK_ARG_SLOT_LIMIT}+ readers), "
        f"{leaves} leaves ({internal} internal pdata-less call targets); guard dispatch {[hex(v) for v in sorted(gd)]}, nop calls {[hex(v) for v in sorted(nops)]}"
    )
    return pe, all_rows, frag_map, fn_starts, info


def module_ident(name: str) -> str:
    stem = name[:-4] if name.endswith(".dll") else name
    return re.sub(r"[^a-z0-9_]", "_", stem.lower())


def emit(out_root: Path, info: dict, gs, templates: dict, lifted_by_va: dict, all_rows: list, reject_reason: dict, format_aot_fn):
    """Write `<out_root>/<module>/{mod.rs,tNN.rs,eNN.rs}`."""
    from collections import Counter

    ident = module_ident(info["name"])
    out_dir = out_root / ident
    for va, why in info.get("pre_rejected", {}).items():
        reject_reason.setdefault(va, why)
    # Post-lift guard: if a function was rejected inside `gen_subsystems` (bad_mnems, jcc, etc.),
    # any tail-jmp thunk whose target is that now-guest-only function must also stay guest so it
    # never runs the guest target from a lifted frame at a different stack depth.
    thunk_targets = info.get("thunk_targets", {})
    while True:
        drop = {
            va: f"thunk to guest-only target +0x{t - info['base']:x} (a lone tail jmp: nothing to lift)"
            for va in list(lifted_by_va)
            if (t := thunk_targets.get(va)) is not None and t not in lifted_by_va
        }
        if not drop:
            break
        for va, why in drop.items():
            lifted_by_va.pop(va, None)
            reject_reason.setdefault(va, why)
    key_uses = Counter(k for k, _ in lifted_by_va.values())
    # EH functions are never templated: each body names its own `EH_META` entry.
    kept = {k for k in templates if key_uses[k] >= 2 and len(k[0]) <= 64 and not any("EH_META_SELF" in l for l in k[0])}
    names = {va: n for va, _, _, _, _, _, _, n in all_rows}

    tpl_ids = {}
    fn_blocks = []
    for key, (t_idx, n_params) in sorted(templates.items(), key=lambda x: x[1][0]):
        if key not in kept:
            continue
        tpl_ids[key] = len(tpl_ids)
        generics = "<" + ", ".join(f"const P{i}: u64" for i in range(n_params)) + ">" if n_params else ""
        fn_blocks.append(format_aot_fn(f"tpl_{tpl_ids[key]}", generics, list(key[0]), None))

    # Incremental-link thunks (`E9 rel32`, possibly chained) whose final target is lifted:
    # dispatch the thunk VA straight to the target's function.
    pe = info["pe"]

    def e9_final(va):
        seen = 0
        while seen < 6:
            b = gs.read_frag_va(pe, va, 5)
            if not b or b[0] != 0xE9:
                return va if seen else None
            va = (va + 5 + struct.unpack("<i", b[1:5])[0]) & MASK64
            seen += 1
        return None

    alias_of = {}
    for va in lifted_by_va:
        t = e9_final(va)
        if t is not None and t in lifted_by_va and e9_final(t) is None:
            alias_of[va] = t

    def fflags(key):
        body = "\n".join(key[0])
        return (0 if "call_va8" in body or "eh.call" in body or "rethrow" in body else 1) | (2 if "set_xmm_ret" in body else 0)

    # Index of each lifted EH function's `EH_META` entry (same order as the table below).
    eh_idx = {}
    for va in sorted(lifted_by_va):
        if va not in alias_of and info.get("eh", {}).get(va) is not None:
            eh_idx[va] = len(eh_idx)

    fn_expr_of = {}
    for va in sorted(lifted_by_va):
        if va in alias_of:
            continue
        key, params = lifted_by_va[va]
        if key in kept:
            t = tpl_ids[key]
            fn_expr_of[va] = f"tpl_{t}::<{', '.join(f'0x{p:x}' for p in params)}> as *const ()" if params else f"tpl_{t} as *const ()"
        else:
            body = list(key[0])
            if any("EH_META_SELF" in l for l in body):
                assert va in eh_idx, f"0x{va:x}: EH record without an EH_META entry"
                body = [l.replace("EH_META_SELF", f"&super::EH_META[{eh_idx[va]}]") for l in body]
            fn_blocks.append(format_aot_fn(f"fn_{va:09x}", "", body, params))
            fn_expr_of[va] = f"fn_{va:09x} as *const ()"
    ent_lines = []
    for va in sorted(lifted_by_va):
        nm = (names.get(va) or f"sub_{va - info['base']:x}").replace("\n", " ")[:120]
        if va in alias_of:
            t = alias_of[va]
            fl = fflags(lifted_by_va[t][0])
            ent_lines.append(
                f"    Lifted::alias(0x{va:x}, {fn_expr_of[t]}, {fl}, 0x{t - info['base']:x}), // +0x{va - info['base']:x} {nm} (thunk -> +0x{t - info['base']:x})\n"
            )
        else:
            fl = fflags(lifted_by_va[va][0])
            ent_lines.append(f"    Lifted::new(0x{va:x}, {fn_expr_of[va]}, {fl}), // +0x{va - info['base']:x} {nm}\n")

    out_dir.mkdir(parents=True, exist_ok=True)
    for old in out_dir.glob("*.rs"):
        old.unlink()
    tpl_lines_max = 12000 if len(fn_blocks) > 10000 else 4200
    ent_chunk_sz = 8000 if len(ent_lines) > 20000 else 4800
    t_chunks, cur, cur_l = [], [], 0
    for b in fn_blocks:
        bl = b.count("\n") + 1
        if cur and cur_l + bl > tpl_lines_max:
            t_chunks.append(cur)
            cur, cur_l = [], 0
        cur.append(b)
        cur_l += bl
    if cur:
        t_chunks.append(cur)
    e_chunks = [ent_lines[i : i + ent_chunk_sz] for i in range(0, len(ent_lines), ent_chunk_sz)] or [[]]
    regfile = info.get("regfile")
    regfile_lifted = sorted(regfile["lifted"]) if regfile else []
    regfile_rejected = sorted(regfile["rejected"]) if regfile else []
    rf_note = f", {len(regfile_lifted)} DF_REGFILE builtins" if regfile_lifted else ""
    mod = [
        f"//! `{info['name']}` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` "
        f"(generated; do not edit).\n",
        f"//! {len(lifted_by_va)} lifted of {len(all_rows)} candidates{rf_note} ({info['roots']} .pdata roots, "
        f"{info['skipped_eh']} with handlers and {info['skipped_ctx']} context exports skipped, {info['leaves']} leaves of which {info['internal_leaves']} internal).\n",
        ALLOW_HDR,
        "\n",
    ]
    for i, ch in enumerate(t_chunks):
        mod.append(f"pub mod t{i:02d};\npub use t{i:02d}::*;\n")
        (out_dir / f"t{i:02d}.rs").write_text(ALLOW_HDR + "\n" + "\n".join(ch))
    for i, ch in enumerate(e_chunks):
        mod.append(f"pub mod e{i:02d};\n")
        (out_dir / f"e{i:02d}.rs").write_text(
            ALLOW_HDR
            + "use super::super::Lifted;\nuse super::*;\n\n"
            + "pub static ENTRIES: &[Lifted] = &[\n"
            + "".join(ch)
            + "];\n"
        )
    slices = ", ".join(f"e{i:02d}::ENTRIES" for i in range(len(e_chunks)))
    regfile_slices = ""
    if regfile_lifted:
        r_chunks, r_cur, r_cur_l = [], [], 0
        for item in regfile_lifted:
            bl = item[4].count("\n") + 1
            if r_cur and r_cur_l + bl > 20000:
                r_chunks.append(r_cur)
                r_cur, r_cur_l = [], 0
            r_cur.append(item)
            r_cur_l += bl
        if r_cur:
            r_chunks.append(r_cur)
        for i, ch in enumerate(r_chunks):
            mod.append(f"pub mod r{i:02d};\npub use r{i:02d}::*;\n")
            (out_dir / f"r{i:02d}.rs").write_text(
                ALLOW_HDR
                + "use crate::emu::regfile::{ctx, RegfileExit};\n\n"
                + "\n".join(code for _va, _rva, _ilen, _bid, code, _conts in ch)
            )
        re_lines = []
        for va, rva, ilen, bid, _code, conts in regfile_lifted:
            if conts:
                cs = ", ".join(f"(0x{off:x}, {blk})" for off, blk in conts)
                re_lines.append(
                    f"    LiftedRegfile::with_conts(0x{va:x}, rf_{va:09x}, 0x{ilen:x}, &[{cs}]), // +0x{rva:x} v8_builtin_{bid}\n"
                )
            else:
                re_lines.append(
                    f"    LiftedRegfile::new(0x{va:x}, rf_{va:09x}, 0x{ilen:x}), // +0x{rva:x} v8_builtin_{bid}\n"
                )
        re_chunks = [re_lines[i : i + 1024] for i in range(0, len(re_lines), 1024)]
        for i, ch in enumerate(re_chunks):
            mod.append(f"pub mod re{i:02d};\n")
            (out_dir / f"re{i:02d}.rs").write_text(
                ALLOW_HDR
                + "use super::super::LiftedRegfile;\nuse super::*;\n\n"
                + "pub static REGFILE_ENTRIES: &[LiftedRegfile] = &[\n"
                + "".join(ch)
                + "];\n"
            )
        regfile_slices = ", ".join(f"re{i:02d}::REGFILE_ENTRIES" for i in range(len(re_chunks)))
    # C++ EH records of the lifted FH4 functions (eh_lift_plan.md T1/T2; read by T3).
    eh_lines = []
    for va in sorted(lifted_by_va):
        x = info.get("eh", {}).get(va)
        if x is None or va in alias_of:
            continue
        conts = ", ".join(f"0x{c:x}" for c in x["conts"])
        eh_lines.append(
            f"    EhMeta {{ va: 0x{va:x}, funcinfo_rva: 0x{x['funcinfo_rva']:x}, handler: {x['handler']}, shape: {x['shape']}, "
            f"frame_reg: {x['frame_reg']}, establisher_off: 0x{x['establisher_off']:x}, conts: &[{conts}] }},\n"
        )
    mod.append(
        "\nuse super::eh::EhMeta;\n\n"
        f"/// {len(eh_lines)} lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).\n"
        "pub static EH_META: &[EhMeta] = &[\n" + "".join(eh_lines) + "];\n"
    )
    mod.append(
        "\npub static IMAGE: super::LiftedImage = super::LiftedImage {\n"
        f"    name: \"{info['name']}\",\n"
        f"    base: 0x{info['base']:x},\n"
        f"    text_rva: 0x{info['text_rva']:x},\n"
        f"    text_len: 0x{info['text_len']:x},\n"
        f"    text_fnv: 0x{info['text_fnv']:016x},\n"
        f"    slices: &[{slices}],\n"
        f"    regfile_slices: &[{regfile_slices}],\n"
        "    eh_meta: EH_META,\n"
        "};\n"
    )
    (out_dir / "mod.rs").write_text("".join(mod))
    import dll_progress
    import hashlib

    base = info["base"]
    prog_lifted = []
    for va in sorted(lifted_by_va):
        key, params = lifted_by_va[alias_of.get(va, va)]
        body = "\n".join(key[0]) + "\n" + ",".join(f"{p:x}" for p in (params or []))
        h = int.from_bytes(hashlib.blake2b(body.encode(), digest_size=8).digest(), "big")
        prog_lifted.append((va - base, names.get(va) or f"sub_{va - base:x}", fflags(key) & 1, h))
    seen_lifted_rvas = {rva for rva, *_ in prog_lifted}
    for _va, rva, _ilen, bid, code, conts in regfile_lifted:
        if rva not in seen_lifted_rvas:
            c_sig = ",".join(f"{o:x}:{b}" for o, b in conts)
            h = int.from_bytes(hashlib.blake2b(f"{code}\n{c_sig}".encode(), digest_size=8).digest(), "big")
            prog_lifted.append((rva, f"v8_builtin_{bid}", 1, h))
            seen_lifted_rvas.add(rva)
    prog_rejected = [(va - base, names.get(va) or f"sub_{va - base:x}", r) for va, r in sorted(reject_reason.items())]
    seen_rej_rvas = {rva for rva, *_ in prog_rejected} | seen_lifted_rvas
    for _va, rva, bid, why in regfile_rejected:
        if rva not in seen_rej_rvas:
            prog_rejected.append((rva, f"v8_builtin_{bid}", why))
            seen_rej_rvas.add(rva)
    nl, nr = dll_progress.update_lift(info["name"], prog_lifted, prog_rejected)
    print(f"{dll_progress.TSV}: {info['name']}: {nl} draft, {nr} rejected")
    reasons = Counter(r.split(" at ")[0].split(" (")[0] for va, r in reject_reason.items())
    print(
        f"Wrote {out_dir}: {len(lifted_by_va)} lifted ({len(tpl_ids)} templates)"
        + (f", {len(regfile_lifted)} regfile" if regfile_lifted else "")
        + f", {len(reject_reason)} rejected"
    )
    for r, c in reasons.most_common():
        print(f"  rejected {c}: {r}")
    for va in sorted(reject_reason)[:60]:
        print(f"    +0x{va - info['base']:x} {names.get(va) or ''}: {reject_reason[va]}")
