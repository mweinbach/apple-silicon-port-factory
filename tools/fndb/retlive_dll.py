#!/usr/bin/env python3
"""Return-register liveness rows for lifted DLL entries (the DLL part of `AW2_SHADOW rets=`).

    python3 tools/fndb/retlive_dll.py [--out db/ret_liveness_dll.tsv] [module ...]

`tools/fndb/retlive.py` covers AlanWake2.exe only, so the shadow compares a DLL entry's full
rax and xmm0. This runs the same analysis (`fnsig retlive`, which loads any PE at its header
image base) per DLL: functions = .pdata roots (+ chained fragments) and exports without
.pdata, call sites = direct `call rel32` / tail `jmp rel32` found by disassembly, targets =
the lifted entries (`Lifted::new(<pinned va>` in crates/aw2-game/src/dll_native/lift/<module>).
Callers not all known -> `ind` (row omitted = full compare): exports, targets of a DIR64
relocation (vtables, function-pointer tables) and of a rip-relative `lea`.
Exports instead get a row from their declared return type ([`sig_exports`]: the MSVC-mangled
name's `void` / integer width / pointer / `float`, unioned over the names ICF folded onto
the VA; any unknown name -> no row), valid for every caller whatever the call path.
Rows use the pinned VA; columns as db/ret_liveness.tsv (`sig` is `?` for caller-analysis rows).
"""
import argparse
import os
import re
import struct
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, "tools/fndb"))
sys.path.insert(0, os.path.join(ROOT, "tools/decomp_scale"))
import fndb  # noqa: E402
import dll_lift  # noqa: E402
from retlive import NOFLOAT_DLLS  # noqa: E402

FNSIG_CANDIDATES = [
    os.path.join(ROOT, "tools/fndb/fnsig/target/release/fnsig"),
]
GAME = os.environ.get("PORT_GAME_DIR") or os.environ.get("AW2_GAME_DIR", "")
LIFT = os.path.join(ROOT, "games/alan-wake-2/crates/aw2-game/src/dll_native/lift")
# module dir -> (file name, pinned base): keep in sync with dll_native::PINNED / lift IMAGES.
MODULES = {
    "vcruntime140_1": ("vcruntime140_1.dll", 0x7001EC4000),
    "vcruntime140": ("vcruntime140.dll", 0x7001EA4000),
    "v8_libplatform": ("v8_libplatform.dll", 0x7001ED0000),
    "renoircore_windowsdesktop": ("RenoirCore.WindowsDesktop.dll", 0x7001F04000),
    "cohtml_windowsdesktop": ("cohtml.WindowsDesktop.dll", 0x7000000000),
    "msvcp140": ("msvcp140.dll", 0x7001E14000),
    "v8_libbase": ("v8_libbase.dll", 0x7001DC4000),
    "v8": ("v8.dll", 0x70006EC000),
}
# Exports whose declared return type is floating point (Win64: xmm0; rax undefined for every
# caller, so those rows hold whatever the callee's last integer op left). C exports of the
# MSVC STL (xmath.h / yvals.h: _Stod/_Stof/_Stold & the x/W variants, _Cosh/_Sinh and the F/L
# variants; long double = double on MSVC); C++ exports by their mangled return type
# (`M` float, `N` double, `O` long double). Rows: rax `?`, xmm `?`, sig `float` -> rax mask 0,
# xmm0 compared (RetUse::from_row, RET_SIG), independent of the callers.
FLOAT_C_EXPORTS = {
    "msvcp140": {"_Cosh", "_Sinh", "_FCosh", "_FSinh", "_LCosh", "_LSinh", "_Stod", "_Stodx", "_Stof",
                 "_Stofx", "_Stold", "_Stoldx", "_WStod", "_WStodx", "_WStof", "_WStofx", "_WStold",
                 "_WStoldx"},
}


def mangled_ret(name):
    """Return-type code (1-2 chars) of an MSVC-mangled x64 function name, or None.

    The position is checked against the x64 markers (instance members `<access>E<cv>A`,
    free / static functions `<kind>A`: `E` = __ptr64, `A` = __cdecl), so a template scope
    whose argument list ends in `@@` (first `@@` not the end of the qualified name) gives
    None instead of a misread."""
    if not name.startswith("?"):
        return None
    i = name.find("@@")
    if i < 0 or i + 2 >= len(name):
        return None
    s = name[i + 2:]
    c = s[0]
    if c in "YCKS":  # free / static member: <kind><cc>
        if s[1:2] != "A":
            return None
        k = 2
    elif c in "ABEFIJMNQRUV":  # instance member: <access>E<cv><cc>
        if s[1:2] != "E" or s[2:3] not in ("A", "B", "C", "D") or s[3:4] != "A":
            return None
        k = 4
    else:
        return None
    return s[k:k + 2] or None


# Declared return type -> (rax bits or None, xmm0 live). Win64: integers / pointers /
# references in rax (callers read only the declared width), float / double in xmm0, void in
# neither. Class / struct / union by value (`V`, `U`, `T`, `?A...`: rax may be a hidden
# pointer or a packed small struct) and anything else -> None (full compare).
def ret_kind(code):
    if not code:
        return None
    c, c2 = code[0], code[:2]
    if c == "X":
        return (0, False)
    if c in "MNO":
        return (0, True)
    if c == "_":
        bits = {"_N": 8, "_D": 8, "_E": 8, "_Q": 8, "_F": 16, "_G": 16, "_W": 16, "_S": 16,
                "_H": 32, "_I": 32, "_U": 32, "_J": 64, "_K": 64}.get(c2)
        return (bits, False) if bits else None
    if c in "CDE":
        return (8, False)
    if c in "FG":
        return (16, False)
    if c in "HIJK":
        return (32, False)
    if c in "PQRSAB":  # pointer / reference
        return (64, False)
    if c2 == "W4":  # enum (int)
        return (32, False)
    return None


def sig_exports(mod, pe, fb):
    """{file VA: (rax, xmm, sig) row columns} for exports whose every name at that VA (ICF
    folds functions of different signatures onto one body) has a known return type: rax =
    the widest declared integer width (`?` if none), xmm `1` only if some name returns
    float and some an integer, sig `float` / `void` / `int`."""
    names = FLOAT_C_EXPORTS.get(mod, set())
    per_va = {}
    for _, r, n in pe.exports():
        if not (n and pe.in_text(r)):
            continue
        k = (0, True) if n in names else ret_kind(mangled_ret(n))
        per_va.setdefault(fb + r, []).append(k)
    out = {}
    for va, ks in per_va.items():
        if any(k is None for k in ks):
            continue
        bits = max(k[0] for k in ks)
        fl = any(k[1] for k in ks)
        if bits == 0:
            out[va] = ("?", "?", "float" if fl else "void")
        else:
            out[va] = (str(bits), "1" if fl else "?", "int")
    return out


def _kind_cols(k):
    """ret_kind tuple -> (rax bits bound, xmm 0/1)."""
    return (k[0], 1 if k[1] else 0)


def body_widths(mod, objdump_text=None):
    """Callee-side width bounds ([`retbody`]) for every function of `mod`, keyed by file VA:
    ({va: rax bits}, {va: xmm 0/1}, {va: exits}, file base). Seeds: declared export types
    ([`sig_exports`]), import names (mangled return type; xmm0 dead for no-float DLLs)."""
    import retbody
    fname, _pinned = MODULES[mod]
    path = os.path.join(GAME, fname)
    pe = fndb.PE(path)
    fb = pe.image_base
    img = retbody.Image(path, objdump_text)
    img.base = fb

    def _read(va, n):
        o = pe.off(va - fb)
        return None if o is None else pe.data[o:o + n]
    img.read = _read
    frags = {}
    for b, e, u in sorted(pe.pdata()):
        if not (pe.in_text(b) and b < e):
            continue
        p, _fl, _h = dll_lift._unwind_primary(pe, u)
        frags.setdefault(fb + (p if p is not None else b), []).append((fb + b, fb + e))
    lo, hi = fb + pe.text_rva, fb + pe.text_end
    extra = {fb + r for _, r, _n in pe.exports() if pe.in_text(r)} | retbody.direct_call_targets(img, lo, hi)
    decl = {}
    for va, (rax, xmm, sig) in sig_exports(mod, pe, fb).items():
        decl[va] = (0, 1 if sig == "float" else 0) if rax == "?" else (int(rax), 1 if xmm == "1" else 0)
    imp_rax, imp_xmm = {}, {}
    for s, dll, fn in pe.imports():
        dll = (dll or "").lower()
        k = (0, True) if fn in FLOAT_C_EXPORTS.get(dll[:-4], set()) else ret_kind(mangled_ret(fn or ""))
        if k is not None:
            imp_rax[fb + s], imp_xmm[fb + s] = _kind_cols(k)
        elif dll in NOFLOAT_DLLS:
            imp_xmm[fb + s] = 0
    W, X, funcs = retbody.analyze(img, frags, extra, decl, imp_rax, imp_xmm)
    return W, X, funcs, fb


LINE_RE = re.compile(r"^\s*([0-9a-f]+):\s+(\S+)\s*(.*)$")


def reloc_targets(pe):
    """File VAs stored at DIR64 relocation sites (absolute pointers in the image)."""
    out = set()
    rva, size = pe.dd[5] if len(pe.dd) > 5 else (0, 0)
    if not (rva and size):
        return out
    d = pe.data
    o = pe.off(rva)
    end = o + size
    while o + 8 <= end:
        page, blk = struct.unpack_from("<II", d, o)
        if blk < 8:
            break
        for k in range((blk - 8) // 2):
            e = struct.unpack_from("<H", d, o + 8 + 2 * k)[0]
            if e >> 12 == 10:
                fo = pe.off(page + (e & 0xFFF))
                if fo is not None:
                    out.add(struct.unpack_from("<Q", d, fo)[0])
        o += blk
    return out


def module_rows(mod, fnsig):
    fname, pinned = MODULES[mod]
    path = os.path.join(GAME, fname)
    pe = fndb.PE(path)
    fb = pe.image_base
    import glob as _glob
    lifted = set()
    aliases = {}
    for ef in sorted(_glob.glob(os.path.join(LIFT, mod, "e*.rs"))):
        with open(ef) as fh:
            for ln in fh:
                code = ln.split("//", 1)[0]
                m_new = re.search(r"Lifted::new\(0x([0-9a-f]+)", code)
                if m_new:
                    lifted.add(int(m_new.group(1), 16) - pinned + fb)
                    continue
                m_al = re.search(r"Lifted::alias\(0x([0-9a-f]+),.*,\s*0x([0-9a-f]+)\)", code)
                if m_al:
                    aliases[int(m_al.group(1), 16) - pinned + fb] = fb + int(m_al.group(2), 16)
    frags = {}
    for b, e, u in sorted(pe.pdata()):
        if not (pe.in_text(b) and b < e):
            continue
        p, _fl, _h = dll_lift._unwind_primary(pe, u)
        frags.setdefault(fb + (p if p is not None else b), []).append((fb + b, fb + e))
    exports = {fb + r for _, r, _n in pe.exports() if pe.in_text(r)}
    # Internal leaves (dll_lift: pdata-less direct call targets of lifted roots) are functions
    # too: without a start their call sites were dropped and the row missing (full compare).
    covered = sorted((b, e) for fr in frags.values() for b, e in fr)
    cst = [b for b, _ in covered]

    def in_pdata(va):
        import bisect as _b
        i = _b.bisect_right(cst, va) - 1
        return i >= 0 and covered[i][0] <= va < covered[i][1]
    internal = {va for va in lifted if va not in frags and va not in exports and not in_pdata(va)}
    starts = sorted(set(frags) | exports | internal)
    # Disassembly: direct calls / tails and rip-relative lea targets.
    out = subprocess.check_output(["objdump", "--disassemble", "--x86-asm-syntax=intel", "--no-show-raw-insn", path], text=True)
    insns = []
    lea_t = set()
    for line in out.splitlines():
        m = LINE_RE.match(line)
        if not m:
            continue
        va, mn, rest = int(m.group(1), 16), m.group(2), m.group(3)
        if mn == "lea" and "rip" in rest and "#" in rest:
            t = re.search(r"#\s*(0x[0-9a-f]+)", rest)
            if t:
                lea_t.add(int(t.group(1), 16))
        if mn in ("call", "jmp"):
            t = re.match(r"(0x[0-9a-f]+)", rest.strip())
            if t:
                insns.append((va, mn, int(t.group(1), 16)))
    import bisect

    fstart = set(starts)
    # Leaves without .pdata (exports, internal): extent to the next start; 512 bytes cap for
    # exports as before, 16 KiB for internal leaves (dll_lift's extent rule for them).
    np_ext = {}
    for k, va in enumerate(starts):
        if va in frags:
            continue
        nxt = starts[k + 1] if k + 1 < len(starts) else va + 512
        np_ext[va] = min(nxt, va + (16384 if va in internal else 512))
    own_frags = {**{f: fr for f, fr in frags.items()}, **{va: [(va, e)] for va, e in np_ext.items()}}
    # A leaf's tail jmp is a call site too (its callers' reads reach the target).
    ranges = sorted((b, e, f) for f, fr in own_frags.items() for b, e in fr)
    rstarts = [r[0] for r in ranges]
    # A pure `jmp rel32` thunk (v8's incremental-link stubs) is resolved to its final target,
    # as the lifter does (dll_lift CALL_REDIRECT): its call sites are the target's callers.
    jmp_at = {va: t for va, mn, t in insns if mn == "jmp"}

    def resolve(t):
        for _ in range(8):
            if t in fstart or t not in jmp_at:
                break
            t = jmp_at[t]
        return t
    calls = []
    for va, mn, t in insns:
        i = bisect.bisect_right(rstarts, va) - 1
        if i < 0 or not (ranges[i][0] <= va < ranges[i][1]):
            continue
        caller = ranges[i][2]
        if not any(b <= t < e for b, e in own_frags[caller]):
            t = resolve(t)
        if t not in fstart:
            continue
        if mn == "call":
            calls.append((caller, t, va, "call"))
        elif not any(b <= t < e for b, e in own_frags[caller]):
            calls.append((caller, t, va, "tail"))
    # Pointers may name the thunk instead of the function (v8's vtables point at its jump
    # stubs): an address-taken or exported thunk makes its final target `ind`.
    taken = exports | reloc_targets(pe) | lea_t

    def chain(t):
        out = {t}
        for _ in range(8):
            if t not in jmp_at:
                break
            t = jmp_at[t]
            out.add(t)
        return out
    ind = {x for t in taken for x in chain(t)} & fstart | exports
    slots = [fb + s for s, dll, _fn in pe.imports() if (dll or "").lower() in NOFLOAT_DLLS]
    rows = []
    with tempfile.TemporaryDirectory() as td:
        fp, cp, tp, op, ip = (os.path.join(td, n) for n in ("funcs.tsv", "calls.tsv", "targets.tsv", "out.tsv", "iat.tsv"))
        with open(fp, "w") as f:
            for va in starts:
                if va in frags:
                    own = [r for r in frags[va] if r[0] == va]
                    if not own:
                        continue
                    extra = "".join("\t%x-%x" % r for r in frags[va] if r[0] != va)
                    f.write("%x\t%x%s\n" % (va, own[0][1], extra))
                else:  # leaf without .pdata (export / internal): 'np'
                    f.write("%x\t%x\tnp\n" % (va, np_ext[va]))
        with open(cp, "w") as f:
            for c in calls:
                f.write("%x\t%x\t%x\t%s\n" % c)
        with open(tp, "w") as f:
            for t in sorted(lifted):
                f.write("%x\n" % t)
        with open(ip, "w") as f:
            for s in slots:
                f.write("%x\n" % s)
        subprocess.run([fnsig, "retlive", path, fp, cp, tp, op, ip], check=True, stdout=subprocess.DEVNULL)
        with open(op) as fh:
            next(fh)
            for line in fh:
                va, n, rax, xmm = line.rstrip("\n").split("\t")
                va = int(va, 16)
                rows.append((va - fb + pinned, n, rax, xmm, va in ind, "?"))
    sx_all = sig_exports(mod, pe, fb)
    sx = {va: v for va, v in sx_all.items() if va in lifted}
    rows = [r for r in rows if r[0] - pinned + fb not in sx]
    rows += [(va - fb + pinned, "-", rax, xmm, False, sig) for va, (rax, xmm, sig) in sorted(sx.items())]
    W, X, _funcs, _fb = body_widths(mod, out)
    rows, nb = merge_body(rows, W, X, {va - fb + pinned: va for va in lifted})
    by_pva = {r[0]: r for r in rows}
    alias_rows = []
    for ava, tva in sorted(aliases.items()):
        apva, tpva = ava - fb + pinned, tva - fb + pinned
        if ava in sx_all:
            rax, xmm, sig = sx_all[ava]
            ar = (apva, "-", rax, xmm, False, sig)
            if sig != "float":
                rc, xc = decode_row(ar)
                w, x = min(rc, W.get(tva, 64)), min(xc, X.get(tva, 1))
                if (w, x) != (rc, xc):
                    nb += 1
                    ar = (apva, "-", str(w) if w < 64 else "?", str(x), False, "body")
            alias_rows.append(ar)
        elif ava not in exports and tpva in by_pva:
            tr = by_pva[tpva]
            alias_rows.append((apva, tr[1], tr[2], tr[3], tr[4], tr[5]))
        else:
            w, x = W.get(tva, 64), X.get(tva, 1)
            if (w, x) != (64, 1):
                nb += 1
                alias_rows.append((apva, "-", str(w) if w < 64 else "?", str(x), False, "body"))
    rows.extend(alias_rows)
    return rows, len(lifted) + len(aliases), len(calls), len(ind), nb


def decode_row(r):
    """(rax bits, xmm 0/1) a row decodes to (as `RetUse::from_row`); None row / `ind` = full."""
    if r is None or r[4]:
        return 64, 1
    _va, _n, rax, xmm, _ind, sig = r
    num = rax in ("0", "8", "16", "32", "64")
    rc = int(rax) if num else (0 if sig in ("void", "float") else 64)
    if xmm in ("0", "1"):
        xc = int(xmm)
    elif sig in ("void", "int"):
        xc = 0
    elif sig == "float":
        xc = 1
    else:
        xc = 0 if num and rax != "0" else 1
    return rc, xc


def merge_body(rows, W, X, lifted_file):
    """Tighten rows with the body bounds ({pinned va: file va} of the lifted entries)."""
    by_va = {r[0]: r for r in rows}
    out, nb = [r for r in rows if r[0] not in lifted_file], 0
    for pva, fva in sorted(lifted_file.items()):
        r = by_va.get(pva)
        if r is not None and r[5] == "float":
            out.append(r)
            continue
        rc, xc = decode_row(r)
        w, x = min(rc, W.get(fva, 64)), min(xc, X.get(fva, 1))
        if (w, x) == (rc, xc):
            if r is not None:
                out.append(r)
            continue
        nb += 1
        out.append((pva, r[1] if r is not None else "-", str(w) if w < 64 else "?", str(x), False, "body"))
    return out, nb


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(ROOT, "db/ret_liveness_dll.tsv"))
    ap.add_argument("modules", nargs="*", default=list(MODULES))
    a = ap.parse_args()
    fnsig = next((p for p in FNSIG_CANDIDATES if os.path.exists(p)), None)
    if not fnsig:
        raise SystemExit("fnsig not built (cargo build --release in tools/fndb/fnsig)")
    allrows = []
    from collections import Counter

    for mod in a.modules:
        rows, nl, nc, ni, nb = module_rows(mod, fnsig)
        kept = [r for r in rows if not r[4] and not (r[2] == "?" and r[3] == "?" and r[5] == "?")]
        c = Counter((r[2], r[3]) for r in kept)
        print(f"retlive_dll: {mod}: {nl} lifted, {nc} direct sites, {ni} address-taken/exported; "
              f"{len(kept)} rows ({nb} tightened by the body analysis); rax/xmm {dict(c.most_common(8))}", file=sys.stderr)
        allrows.extend(kept)
    with open(a.out, "w") as f:
        f.write("# generated by tools/fndb/retlive_dll.py (fnsig retlive over the lifted DLLs); see AW2_SHADOW rets=\n")
        f.write("# rows that would decode to a full compare are omitted (missing row = full)\n")
        f.write("va\tsites\trax\txmm\tind\tsig\n")
        for va, n, rax, xmm, _ind, sig in sorted(allrows):
            f.write("0x%x\t%s\t%s\t%s\t0\t%s\n" % (va, n, rax, xmm, sig))
    print(f"retlive_dll: {len(allrows)} rows -> {a.out}", file=sys.stderr)


if __name__ == "__main__":
    main()
