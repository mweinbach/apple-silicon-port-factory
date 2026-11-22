#!/usr/bin/env python3
"""Callee-side return-width inference ("body" rows of db/ret_liveness_dll.tsv).

    python3 tools/fndb/retbody.py <dll> [--why <file va hex> ...]

`fnsig retlive` bounds the live return bits from the *callers*, so it has nothing to say about
an address-taken function (vtable entry, callback): callers unknown -> full compare. This bounds
the *declared* return width from the function's own body instead, which holds for every caller:

At each exit of the function the reaching definitions of rax (resp. xmm0) are collected over
the CFG of its .pdata fragments (backward walk from each `ret`; a direct `jmp`/`jcc` out of the
function is a tail call and an exit whose value is the target's return; a `jmp reg` fed by an
MSVC RVA jump table `[base + 4*idx + rva]` gets the table's targets as successors, read as
gen_subsystems reads them for the lifted code; any other indirect jmp leaves the walk open). Each exit path gives
an upper bound on the declared width:

  * the incoming rax (no write on the path; rax is never an argument register on Win64): a
    non-void function defines rax on every path to a `ret`, so the function returns no
    integer: bound 0. Applied only when (a) the incoming value is never read (`__chkstk`
    takes its size in rax and returns it intact) and no indirect `jmp` is reachable before a
    write, and (b) every other exit's value comes from a call (an own def next to an
    incoming path is what a value-returning function with an unreachable/UB path looks
    like, so then no bound). Not used for xmm0 (the first float argument arrives in xmm0, so
    `double id(double x)` returns its incoming xmm0).
  * the unchanged result of a direct call / tail call T: bound = T's width (no extension
    instruction in between, else it would be an own def). T's width: its MSVC-mangled
    export / import name's declared type, or T's own inferred width.
  * an import from a no-float DLL (`retlive.NOFLOAT_DLLS`): xmm0 bound 0.
  * an own def or an indirect call: no bound.

Width = min over the exits' bounds and the declared export type; iterated to the greatest
fixed point from 64 / live (sound: the true widths satisfy every bound). Values: rax bits
in {0,8,16,32,64}, xmm0 in {0 (dead), 1}.
"""
import bisect
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

PREFIXES = {"lock", "rep", "repe", "repne", "repz", "repnz", "notrack", "bnd", "data16", "cs", "ds", "es", "ss", "fs", "gs", "addr32"}
RAX_REG = re.compile(r"(rax|eax|ax|al|ah)")
RAX_ANY = re.compile(r"\b(rax|eax|ax|al|ah)\b")
X0_REG = re.compile(r"[xyz]mm0")
CMP_ONLY = {"cmp", "test", "push", "bt", "ucomiss", "ucomisd", "comiss", "comisd", "vucomiss", "vucomisd",
            "vcomiss", "vcomisd", "ptest", "vptest", "vtestps", "vtestpd", "out"}
# Implicit rax writers (the explicit-destination rule covers the rest).
IMPL_RAX_W = {"mul", "div", "idiv", "cdqe", "cwde", "cbw", "cmpxchg", "cmpxchg8b", "cmpxchg16b", "lodsb", "lodsw",
              "lodsd", "lodsq", "rdtsc", "rdtscp", "cpuid", "xgetbv", "lahf", "in", "syscall", "rdpid", "rdpkru"}
# Implicit rax readers.
IMPL_RAX_R = {"mul", "div", "idiv", "cdqe", "cwde", "cbw", "cqo", "cdq", "cwd", "cmpxchg", "cmpxchg8b",
              "cmpxchg16b", "stosb", "stosw", "stosd", "stosq", "scasb", "scasw", "scasd", "scasq", "sahf",
              "xsave", "xsave64", "xsaveopt", "xsaveopt64", "xsavec", "xsavec64", "xrstor", "xrstor64",
              "wrpkru", "xsetbv"}
# Pure writers of their first operand (the old value is not read).
PURE_W = {"mov", "movzx", "movsx", "movsxd", "movabs", "lea", "pop", "movd", "movq", "vmovd", "vmovq", "rdrand",
          "rdseed", "lzcnt", "tzcnt", "popcnt", "pext", "pdep", "bzhi", "andn", "shlx", "sarx", "shrx", "rorx",
          "blsr", "blsi", "blsmsk", "cvttsd2si", "cvtsd2si", "cvttss2si", "cvtss2si", "vcvttsd2si", "vcvtsd2si",
          "vcvttss2si", "vcvtss2si", "movmskps", "movmskpd", "pmovmskb", "vmovmskps", "vmovmskpd", "vpmovmskb",
          "pextrb", "pextrw", "pextrd", "pextrq", "vpextrb", "vpextrw", "vpextrd", "vpextrq", "extractps",
          "vextractps"}
IMPL_X0_W = {"pcmpestrm", "pcmpistrm", "vpcmpestrm", "vpcmpistrm", "vzeroall"}
LINE_RE = re.compile(r"^\s*([0-9a-f]+):\s+(\S+)\s*(.*)$")
# RVA jump-table operand (as gen_subsystems.JT_DISP_RE): `[base + 4*idx + rva]`.
JT_DISP_RE = re.compile(r"(?:4\s*\*\s*[a-z0-9]+|[a-z0-9]+\s*\*\s*4)\s*\+\s*(0x[0-9a-fA-F]{6,})", re.I)


def split_ops(ops):
    return [p.strip() for p in re.split(r",(?![^\[]*\])", ops)] if ops else []


def classify(mn, parts):
    """(writes rax, reads rax, writes xmm0) of one non-control instruction."""
    if mn.startswith("nop") or mn in ("endbr64", "int3", "vzeroupper", "pause", "prefetcht0", "prefetcht1",
                                      "prefetcht2", "prefetchnta", "prefetchw"):
        return False, False, False
    first = parts[0] if parts else ""
    first_rax = bool(RAX_REG.fullmatch(first))
    w = mn in IMPL_RAX_W or (mn == "imul" and len(parts) == 1) or (first_rax and mn not in CMP_ONLY)
    if mn == "xchg" and any(RAX_REG.fullmatch(p) for p in parts):
        w = True
    zero_idiom = mn in ("xor", "sub") and len(parts) == 2 and first_rax and parts[0] == parts[1]
    r = (mn in IMPL_RAX_R or any(RAX_ANY.search(p) for p in parts[1:])
         or (first and not first_rax and RAX_ANY.search(first) is not None)
         or (first_rax and mn not in PURE_W and not mn.startswith("set") and not zero_idiom))
    if zero_idiom:
        r = False
    x0 = mn in IMPL_X0_W or (bool(X0_REG.fullmatch(first)) and mn not in CMP_ONLY)
    return w, r, x0


class Image:
    """Whole-image disassembly (objdump) with rip-relative comment targets."""

    def __init__(self, path, objdump_text=None):
        out = objdump_text or subprocess.check_output(
            ["objdump", "--disassemble", "--x86-asm-syntax=intel", "--no-show-raw-insn", path], text=True)
        self.va, self.mn, self.parts, self.cmt = [], [], [], []
        for line in out.splitlines():
            m = LINE_RE.match(line)
            if not m:
                continue
            mn, rest = m.group(2), m.group(3)
            cmt = None
            if "#" in rest:
                rest, c = rest.split("#", 1)
                t = re.match(r"\s*(0x[0-9a-f]+)", c)
                cmt = int(t.group(1), 16) if t else None
            rest = re.sub(r"\s*<[^>]*>", "", rest).strip()
            while mn in PREFIXES and rest:
                sp = rest.split(None, 1)
                mn, rest = sp[0], (sp[1] if len(sp) > 1 else "")
            self.va.append(int(m.group(1), 16))
            self.mn.append(mn)
            self.parts.append(split_ops(rest))
            self.cmt.append(cmt)

    def span(self, b, e):
        return range(bisect.bisect_left(self.va, b), bisect.bisect_left(self.va, e))

    # Set by the caller (retlive_dll.body_widths): file image base and `read(va, n) -> bytes |
    # None` over the section data, for jump tables. Without them indirect jmps stay open.
    base = None
    read = None

    def jump_table_targets(self, gs, internal_starts):
        """Targets of the MSVC RVA jump tables (`[reg + 4*reg + rva]`, the base register lea'd
        to the image base) referenced by the instructions `gs`, read as gen_subsystems reads
        them for the lifted code: consecutive u32 RVAs while each lands on an instruction start
        of the function (`internal_starts`). Empty if the image bytes are unavailable."""
        out = set()
        if self.base is None or self.read is None:
            return out
        for g in gs:
            for p in self.parts[g]:
                m = JT_DISP_RE.search(p)
                if not m:
                    continue
                jt = self.base + int(m.group(1), 16)
                data = self.read(jt, 512)
                if not data:
                    continue
                for k in range(0, len(data) - 3, 4):
                    t = self.base + int.from_bytes(data[k:k + 4], "little")
                    if t not in internal_starts:
                        break
                    out.add(t)
        return out


def _target(parts):
    if len(parts) == 1 and re.fullmatch(r"0x[0-9a-f]+", parts[0]):
        return int(parts[0], 16)
    return None


def _slot(img, g):
    p = img.parts[g]
    return img.cmt[g] if len(p) == 1 and "rip" in p[0] and "[" in p[0] else None


def exits(img, frags, ctx=None):
    """Exit-value sources of the function made of the file-VA ranges `frags` (primary first).

    Returns (rax, xmm, info): rax / xmm = list of sources per exit path, a source being
    ('in',) incoming value, ('fn', va) direct (tail) call result, ('imp', slot) import call,
    ('own',) own def, ('ind',) indirect call; info = {'entry_read', 'open' (a path runs off
    the analysed range or through an indirect jump), 'targets' (direct call / tail targets)}.
    `ctx`: 'noret' (callees that never return: no fallthrough after the call, tails into them
    are not exits), 'trans_rax' / 'trans_xmm' (callees that hand back the value they were
    entered with: walked through), 'reads_rax' (callees that read their incoming rax)."""
    ctx = ctx or {}
    noret = ctx.get("noret", ())
    nodes = [g for b, e in frags for g in img.span(b, e)]
    if not nodes:
        return [], [], {"entry_read": True, "open": True, "targets": set()}
    pos = {g: k for k, g in enumerate(nodes)}

    def internal(t):
        return t is not None and any(b <= t < e for b, e in frags)

    n = len(nodes)
    succ = [[] for _ in range(n)]
    preds = [[] for _ in range(n)]
    terms = []  # (k, kind, payload): 'ret' | 'tail' (source)
    ind_jmp, open_end, targets = set(), set(), set()
    starts = None  # {instruction va: g} of the function, built at the first `jmp reg`
    for k, g in enumerate(nodes):
        mn, parts = img.mn[g], img.parts[g]
        t = _target(parts)
        nxt = pos.get(g + 1)
        if t is not None and (mn == "call" or mn.startswith("j")) and not internal(t):
            targets.add(t)
        if mn.startswith("ret"):
            terms.append((k, "ret", None))
        elif mn in ("int3", "ud2", "hlt") or (mn == "int" and parts == ["0x29"]):
            pass
        elif mn == "jmp":
            if internal(t):
                succ[k].append(pos.get(img.span(t, t + 1).start, None))
            elif t is not None:
                terms.append((k, "tail", ("fn", t)))
            elif _slot(img, g) is not None:
                terms.append((k, "tail", ("imp", _slot(img, g))))
            else:
                jt = set()
                if len(parts) == 1 and "[" not in parts[0]:
                    if starts is None:
                        starts = {img.va[x]: x for x in nodes}
                    jt = img.jump_table_targets([nodes[j] for j in range(max(0, k - 8), k)], starts)
                if jt:
                    succ[k].extend(pos[starts[x]] for x in sorted(jt))
                else:
                    ind_jmp.add(k)
        elif mn.startswith("j"):
            if nxt is not None:
                succ[k].append(nxt)
            else:
                open_end.add(k)
            if internal(t):
                succ[k].append(pos.get(img.span(t, t + 1).start, None))
            elif t is not None:
                terms.append((k, "tail", ("fn", t)))
            else:
                ind_jmp.add(k)
        elif mn == "call" and t is not None and t in noret:
            pass
        elif nxt is not None:
            succ[k].append(nxt)
        else:
            open_end.add(k)
        succ[k] = [s for s in succ[k] if s is not None]
        for s in succ[k]:
            preds[s].append(k)

    reach, work = set(), [0]
    while work:
        j = work.pop()
        if j not in reach:
            reach.add(j)
            work += succ[j]
    terms = [t for t in terms if t[0] in reach]
    preds = [[p for p in ps if p in reach] for ps in preds]
    is_open = bool((ind_jmp | open_end) & reach)

    cls = {}

    def c(k):
        if k not in cls:
            g = nodes[k]
            mn = img.mn[g]
            if mn == "call":
                cls[k] = (True, True, True)
            else:
                cls[k] = classify(mn, img.parts[g])
        return cls[k]

    def call_src(k):
        g = nodes[k]
        t = _target(img.parts[g])
        if t is not None:
            return ("fn", t)
        s = _slot(img, g)
        return ("imp", s) if s is not None else ("ind",)

    trans = (ctx.get("trans_rax", ()), None, ctx.get("trans_xmm", ()))
    reads = ctx.get("reads_rax", ())

    def walk(start_preds, reg, at_entry):  # reg 0 = rax, 2 = xmm0
        srcs, seen, work = set(), set(), list(start_preds)
        if at_entry:
            srcs.add(("in",))
        while work:
            j = work.pop()
            if j in seen:
                continue
            seen.add(j)
            if img.mn[nodes[j]] == "call":
                s = call_src(j)
                if s[0] == "fn" and s[1] in noret:
                    continue  # infeasible: the callee does not return
                if not (s[0] == "fn" and s[1] in trans[reg]):
                    srcs.add(s)
                    continue
            elif c(j)[reg]:
                srcs.add(("own",))
                continue
            if j == 0:
                srcs.add(("in",))
            work += preds[j]
        return sorted(srcs)

    rax, xmm = [], []
    for k, kind, src in terms:
        if kind == "tail" and src[0] == "fn" and src[1] in noret:
            continue
        for reg, out in ((0, rax), (2, xmm)):
            if kind == "tail" and not (src[0] == "fn" and src[1] in trans[reg]):
                out.append([src])
            else:  # `ret`, or a tail call that hands back the value it was entered with
                out.append(walk(preds[k] if k != 0 else [], reg, k == 0))

    # Is the incoming rax read (or can it escape through an indirect jump) before a write?
    entry_read, seen, work = False, set(), [0]
    while work and not entry_read:
        j = work.pop()
        if j in seen:
            continue
        seen.add(j)
        if j in ind_jmp:
            entry_read = True
            break
        if img.mn[nodes[j]] == "call":
            s = call_src(j)
            if s[0] == "fn" and s[1] in trans[0]:
                if s[1] in reads:
                    entry_read = True
                    break
                work += succ[j]
            continue
        w, r, _ = c(j)
        if r:
            entry_read = True
            break
        if w:
            continue
        work += succ[j]
    for k, kind, src in terms:  # a tail call hands over the incoming rax
        if kind == "tail" and k in seen and src[0] == "fn" and src[1] in reads:
            entry_read = True
    return rax, xmm, {"entry_read": entry_read, "open": is_open, "targets": targets}


def src_bound(s, width, imp, allow_in):
    """Upper bound on the declared width given by one reaching source (None = no bound).
    Every CFG path to an exit is taken as feasible, so each source bounds on its own."""
    if s[0] == "in":
        return 0 if allow_in else None
    if s[0] == "fn":
        return width.get(s[1])
    if s[0] == "imp":
        return imp.get(s[1])
    return None


def infer(funcs, decl, imp_rax, imp_xmm, max_iter=64):
    """funcs: {va: (rax exits, xmm exits, info)}; decl: {va: (rax bits, xmm 0/1)} declared
    (export) types; imp_*: {slot: bound}. Returns ({va: rax bits}, {va: xmm 0/1})."""
    W = {va: 64 for va in funcs}
    X = {va: 1 for va in funcs}
    for va, (r, x) in decl.items():
        if va in W:
            W[va] = min(W[va], r)
            X[va] = min(X[va], x)
    allow = {}
    for va, (rax, _xmm, info) in funcs.items():
        own = any(("own",) in p for p in rax)
        # (b): the other exits' values all come from calls (direct, import or indirect).
        allow[va] = not info["entry_read"] and not own
    srcs = {va: ({s for p in rax for s in p}, {s for p in xmm for s in p}) for va, (rax, xmm, _i) in funcs.items()}
    for _ in range(max_iter):
        changed = False
        for va, (rs, xs) in srcs.items():
            w = W[va]
            for s in rs:
                b = src_bound(s, W, imp_rax, allow[va])
                if b is not None and b < w:
                    w = b
            x = X[va]
            for s in xs:
                b = src_bound(s, X, imp_xmm, False)
                if b is not None and b < x:
                    x = b
            if w != W[va] or x != X[va]:
                W[va], X[va] = w, x
                changed = True
        if not changed:
            break
    return W, X


def direct_call_targets(img, lo, hi):
    """Targets of direct `call rel32` / `jmp rel32` inside [lo, hi) (jmp targets inside a
    .pdata function are filtered by the caller)."""
    out = set()
    for g, mn in enumerate(img.mn):
        if mn == "call" or mn == "jmp":
            t = _target(img.parts[g])
            if t is not None and lo <= t < hi:
                out.add((t, mn))
    return out


def helper_kind(img, va):
    """Compiler helpers with a register contract callers rely on: `__security_check_cookie`
    (rax / xmm0 preserved) and `__chkstk` (size in rax, rax preserved)."""
    g = img.span(va, va + 1).start
    if g + 3 > len(img.va) or img.va[g] != va:
        return None
    m = [(img.mn[g + i], img.parts[g + i]) for i in range(3)]
    if (m[0][0] == "cmp" and m[0][1][:1] == ["rcx"] and "rip" in m[0][1][1] and m[1][0] == "jne"
            and m[2] == ("rol", ["rcx", "0x10"])):
        return "cookie"
    if (m[0] == ("sub", ["rsp", "0x10"]) and m[1] == ("mov", ["qword ptr [rsp]", "r10"])
            and m[2] == ("mov", ["qword ptr [rsp + 0x8]", "r11"])):
        return "chkstk"
    return None


def analyze(img, frags, extra_starts, decl, imp_rax, imp_xmm, max_rounds=8):
    """Body widths for every function: `frags` {start va: [(b, e)]} (.pdata), `extra_starts`
    {(va, 'call'|'jmp')} plus plain VAs (exports; direct call targets; jmp targets outside
    every .pdata range, i.e. leaf code reached by a thunk): extent to the next start, <= 512
    bytes."""
    ranges = sorted(r for fr in frags.values() for r in fr)
    rb = [r[0] for r in ranges]

    def in_pdata(t):
        i = bisect.bisect_right(rb, t) - 1
        return i >= 0 and ranges[i][0] <= t < ranges[i][1]

    extra = set()
    for x in extra_starts:
        if isinstance(x, tuple):
            t, mn = x
            if mn == "call" or not in_pdata(t):
                extra.add(t)
        else:
            extra.add(x)
    starts = sorted(set(frags) | extra)
    fr_of = {}
    for va in starts:
        if va in frags:
            fr_of[va] = sorted(frags[va], key=lambda r: (r[0] != va, r[0]))
        else:
            j = bisect.bisect_right(starts, va)
            nxt = starts[j] if j < len(starts) else va + 512
            fr_of[va] = [(va, min(nxt, va + 512))]
    helpers = {va: k for va in starts for k in [helper_kind(img, va)] if k}
    ctx = {"noret": set(), "trans_rax": {va for va in helpers}, "trans_xmm": {va for va in helpers},
           "reads_rax": {va for va, k in helpers.items() if k == "chkstk"}}
    funcs = {va: exits(img, fr_of[va], ctx) for va in starts}
    callers = {}
    for va, (_r, _x, info) in funcs.items():
        for t in info["targets"]:
            callers.setdefault(t, set()).add(va)
    for _ in range(max_rounds):
        noret, tr, tx, rd = set(ctx["noret"]), set(ctx["trans_rax"]), set(ctx["trans_xmm"]), set(ctx["reads_rax"])
        for va, (rax, xmm, info) in funcs.items():
            if va in helpers:
                continue
            if not rax and not info["open"] and fr_of[va]:
                noret.add(va)
                continue
            if rax and all(p == [("in",)] for p in rax):
                tr.add(va)
                if info["entry_read"]:
                    rd.add(va)
            if xmm and all(p == [("in",)] for p in xmm):
                tx.add(va)
        delta = (noret - ctx["noret"]) | (tr - ctx["trans_rax"]) | (tx - ctx["trans_xmm"]) | (rd - ctx["reads_rax"])
        if not delta:
            break
        ctx = {"noret": noret, "trans_rax": tr, "trans_xmm": tx, "reads_rax": rd}
        redo = set().union(*(callers.get(t, ()) for t in delta))
        for va in redo:
            funcs[va] = exits(img, fr_of[va], ctx)
    W, X = infer(funcs, decl, imp_rax, imp_xmm)
    return W, X, funcs


def main():
    import retlive_dll as rd
    args = sys.argv[1:]
    mod = args[0]
    why = [int(a, 16) for a in args[args.index("--why") + 1:]] if "--why" in args else []
    W, X, funcs, fb = rd.body_widths(mod)
    for rva in why:
        va = fb + rva
        rax, xmm, info = funcs.get(va, ([], [], {}))
        print(f"+0x{rva:x}: rax {W.get(va)} xmm {X.get(va)} {info}")

        def fmt(s, tab):
            if s[0] == "fn":
                return f"fn+0x{s[1] - fb:x}={tab.get(s[1])}"
            if s[0] == "imp":
                return f"imp@0x{s[1] - fb:x}"
            return s[0]
        for p in rax:
            print("  rax exit:", ", ".join(fmt(s, W) for s in p))
        for p in xmm:
            print("  xmm exit:", ", ".join(fmt(s, X) for s in p))
    from collections import Counter
    print("rax", sorted(Counter(W.values()).items()), "xmm", sorted(Counter(X.values()).items()))


if __name__ == "__main__":
    main()
