#!/usr/bin/env python3
"""Return-register liveness table for the shadow oracle (`AW2_SHADOW rets=`).

    python3 tools/fndb/retlive.py [--db db/functions.sqlite] [--targets db/progress.tsv]
                                  [--out db/ret_liveness.tsv] [--exe PATH]

For every target function (default: the VAs in db/progress.tsv), `fnsig retlive` walks
the original x86 after each direct call site and reports the max width of rax the
callers read (0 = never read) and whether xmm0 is read. Functions whose callers are not
all known -- address-taken (`lea` / data xrefs), vftable slots or exports -- are written
with `ind=1`; the shadow compares their full rax/xmm0.

The Rust replacement's declared return type is added as `sig` (void|int|float|?), from
the registry entry's function path. The shadow uses it only where the x86 analysis is
inconclusive (see shadow `rets=`).

Output columns: va  sites  rax(0|8|16|32|64|?)  xmm(0|1|?)  ind(0|1)  sig
"""
import argparse
import os
import re
import sqlite3
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
FNSIG = os.path.join(ROOT, 'tools/fndb/fnsig/target/release/fnsig')
DEFAULT_EXE = os.environ.get('AW2_EXE') or (
    os.path.join(os.environ['AW2_GAME_DIR'], 'AlanWake2.exe')
    if os.environ.get('AW2_GAME_DIR')
    else ''
)


REG = re.compile(r'Replacement::(?:verified|verified_throws|draft|draft_throws|draft_diff)\(\s*(0x[0-9a-fA-F_]+)\s*,\s*"(?:[^"\\]|\\.)*"\s*,\s*([\w:]+)(?:::<[^>]*>)?\s+as\s+\*const\s*\(\)')
FN = re.compile(r'\bfn\s+(\w+)\s*(?:<[^{(]*?>)?\s*\(')


def sig_class(ret):
    if ret is None:
        return 'void'
    t = re.sub(r'\s+', '', ret)
    if t in ('()', '!'):
        return 'void'
    if t in ('f32', 'f64', 'u128', '__m128', '__m128d', '__m128i', 'M128') or t.endswith('::__m128'):
        return 'float'
    if t.startswith('('):
        return '?'
    return 'int'


def fn_index(src):
    """name -> {(file, class)} over every `fn` in the crate sources."""
    idx = {}
    for dp, _, fs in os.walk(src):
        for fn in fs:
            if not fn.endswith('.rs'):
                continue
            path = os.path.join(dp, fn)
            try:
                text = open(path, encoding='utf-8', errors='replace').read()
            except OSError:
                continue
            for m in FN.finditer(text):
                i, depth = m.end(), 1
                while i < len(text) and depth:
                    c = text[i]
                    depth += c == '('
                    depth -= c == ')'
                    i += 1
                tail = text[i:i + 400]
                r = re.match(r'\s*->\s*(.+?)\s*(?:\bwhere\b|\{|;)', tail, re.S)
                cls = sig_class(r.group(1) if r else None)
                # Lifted (generated) bodies always return u64 and pass xmm0 through
                # set_xmm_ret: their declared type says nothing.
                if re.search(r'\bmut\s+rcx\s*:\s*u64', text[m.end():i]) or 'get_rax_in()' in text[i:i + 600]:
                    cls = '?'
                idx.setdefault(m.group(1), set()).add((path, cls))
    return idx


def rust_sigs(src):
    idx = fn_index(src)
    out = {}
    for dp, _, fs in os.walk(src):
        for fn in fs:
            if not fn.endswith('.rs'):
                continue
            text = open(os.path.join(dp, fn), encoding='utf-8', errors='replace').read()
            if 'Replacement::' not in text:
                continue
            for m in REG.finditer(text):
                va = int(m.group(1).replace('_', ''), 16)
                path = re.sub(r'::<.*', '', m.group(2).strip())
                segs = [x for x in path.split('::') if x not in ('crate', 'self', 'super')]
                cands = idx.get(segs[-1], set())
                if not cands:
                    continue
                mods = segs[:-1]
                hit = [c for (f, c) in cands
                       if mods and (f.endswith('/'.join(mods) + '.rs') or f.endswith('/'.join(mods) + '/mod.rs'))]
                classes = set(hit) if hit else {c for (_, c) in cands}
                out[va] = classes.pop() if len(classes) == 1 else '?'
    return out


# System DLLs none of whose imports take a float first argument: a call through their IAT
# clobbers xmm0 without reading it. (Not the CRT math/convert sets, MSVCP or third-party DLLs.)
NOFLOAT_DLLS = {
    'kernel32.dll', 'user32.dll', 'advapi32.dll', 'ole32.dll', 'oleaut32.dll', 'ws2_32.dll',
    'gdi32.dll', 'hid.dll', 'setupapi.dll', 'ntdll.dll', 'shell32.dll', 'shlwapi.dll',
    'vcruntime140.dll', 'vcruntime140_1.dll', 'api-ms-win-crt-heap-l1-1-0.dll',
    'api-ms-win-crt-runtime-l1-1-0.dll', 'api-ms-win-crt-string-l1-1-0.dll',
    'api-ms-win-crt-stdio-l1-1-0.dll', 'api-ms-win-crt-time-l1-1-0.dll',
    'api-ms-win-crt-environment-l1-1-0.dll', 'api-ms-win-crt-filesystem-l1-1-0.dll',
    'api-ms-win-crt-locale-l1-1-0.dll', 'crypt32.dll', 'rpcrt4.dll', 'version.dll', 'winmm.dll',
    'xinput1_4.dll', 'dwmapi.dll',
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--db', default=os.path.join(ROOT, 'db/functions.sqlite'))
    ap.add_argument('--targets', default=os.path.join(ROOT, 'db/progress.tsv'))
    ap.add_argument('--out', default=os.path.join(ROOT, 'db/ret_liveness.tsv'))
    ap.add_argument('--exe', default=DEFAULT_EXE)
    ap.add_argument('--src', default=os.path.join(ROOT, 'crates/aw2-game/src'))
    a = ap.parse_args()

    db = sqlite3.connect(a.db)
    targets = set()
    with open(a.targets) as fh:
        for line in fh:
            c = line.split('\t')[0].strip()
            if c.startswith('0x'):
                targets.add(int(c, 16))
            elif c and c[0] in '0123456789abcdef' and c != 'va':
                try:
                    targets.add(int(c, 16))
                except ValueError:
                    pass
    frags = {}
    for va, end, fva in db.execute('SELECT va, "end", func_va FROM fragments'):
        frags.setdefault(fva, []).append((va, end))
    ind = set()
    for (t,) in db.execute("SELECT DISTINCT target FROM xrefs WHERE kind='lea' OR kind LIKE 'data%'"):
        ind.add(t)
    for (t,) in db.execute('SELECT DISTINCT target FROM vftable_slots'):
        ind.add(t)
    for (t,) in db.execute('SELECT DISTINCT va FROM exports'):
        ind.add(t)

    with tempfile.TemporaryDirectory() as td:
        fp, cp, tp, op = (os.path.join(td, n) for n in ('funcs.tsv', 'calls.tsv', 'targets.tsv', 'out.tsv'))
        with open(fp, 'w') as f:
            for va, end, src in db.execute('SELECT va, "end", source FROM functions'):
                extra = ''.join('\t%x-%x' % r for r in frags.get(va, []))
                if src != 'pdata':
                    extra += '\tnp'
                f.write('%x\t%x%s\n' % (va, end, extra))
        with open(cp, 'w') as f:
            for caller, callee, site, kind in db.execute(
                    "SELECT caller, callee, site, kind FROM calls WHERE kind IN ('call','tail')"):
                f.write('%x\t%x\t%x\t%s\n' % (caller, callee, site, kind))
        with open(tp, 'w') as f:
            for t in sorted(targets):
                f.write('%x\n' % t)
        ip = os.path.join(td, 'iat.tsv')
        with open(ip, 'w') as f:
            n_iat = 0
            for iat, dll in db.execute('SELECT iat_va, dll FROM imports'):
                if (dll or '').lower() in NOFLOAT_DLLS:
                    f.write('%x\n' % iat)
                    n_iat += 1
        print('retlive: %d no-float import slots' % n_iat, file=sys.stderr)
        subprocess.run([FNSIG, 'retlive', a.exe, fp, cp, tp, op, ip], check=True)
        rows = []
        with open(op) as fh:
            next(fh)
            for line in fh:
                va, n, rax, xmm = line.rstrip('\n').split('\t')
                rows.append((int(va, 16), n, rax, xmm))
    sigs = rust_sigs(a.src)
    # A missing row means a full compare (shadow RetUse::default), so only rows that can
    # narrow it are written: address-taken (`ind`) functions and functions with nothing
    # known (rax, xmm0 and the Rust signature all '?') are omitted.
    n_out = 0
    with open(a.out, 'w') as f:
        f.write('# generated by tools/fndb/retlive.py (fnsig retlive); see AW2_SHADOW rets=\n')
        f.write('# rows that would decode to a full compare are omitted (missing row = full)\n')
        f.write('va\tsites\trax\txmm\tind\tsig\n')
        for va, n, rax, xmm in sorted(rows):
            sig = sigs.get(va, '?')
            if va in ind or (rax == '?' and xmm == '?' and sig == '?'):
                continue
            f.write('0x%x\t%s\t%s\t%s\t%d\t%s\n' % (va, n, rax, xmm, 0, sig))
            n_out += 1
    from collections import Counter
    print('retlive: rust sigs %s' % dict(Counter(sigs.values())), file=sys.stderr)
    n_ind = sum(1 for r in rows if r[0] in ind)
    print('retlive: %d functions (%d with unknown callers), %d informative rows -> %s'
          % (len(rows), n_ind, n_out, a.out), file=sys.stderr)


if __name__ == '__main__':
    main()
