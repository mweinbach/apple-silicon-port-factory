#!/usr/bin/env python3
"""Export a compact symbol file for AlanWake2.exe from the function DB.

    tools/debug/export_symbols.py [--db db/functions.sqlite] [--progress db/progress.tsv]
                                  [--out out/symbols/AlanWake2.exe.syms]

The loader (crash reports, trap backtraces, every `symbolize()` call) and
tools/debug/lldb_aw2.py read this file. Format (text, one function per line,
sorted by RVA):

    # aw2-syms v1 image=AlanWake2.exe base=0x140000000 count=N
    <rva hex>\t<size hex>\t<owner>\t<status>\t<name>

`owner` comes from fndb (remedy/thirdparty/crt/unknown). `status` is the
db/progress.tsv decomp status (ported functions: draft/verified...), or `-`.
`name` is the fndb name and may be empty (the reader prints `sub_<va>`).
Regenerate after `tools/fndb.sh build` / `sync`. The file is ~10 MB and
lives in out/ (gitignored).
"""
import argparse
import csv
import os
import sqlite3
import sys

BASE = 0x140000000


def main():
    root = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--db', default=os.path.join(root, 'db', 'functions.sqlite'))
    ap.add_argument('--progress', default=os.path.join(root, 'db', 'progress.tsv'))
    ap.add_argument('--out', default=os.path.join(root, 'out', 'symbols', 'AlanWake2.exe.syms'))
    a = ap.parse_args()
    if not os.path.exists(a.db):
        sys.exit(f'{a.db} not found: run tools/fndb.sh build first')

    status = {}
    if os.path.exists(a.progress):
        with open(a.progress, newline='') as f:
            for row in csv.DictReader(f, delimiter='\t'):
                try:
                    status[int(row['va'], 16)] = row['status'] or '-'
                except (KeyError, ValueError):
                    pass

    con = sqlite3.connect(f'file:{a.db}?mode=ro', uri=True)
    rows = con.execute('select va, size, coalesce(owner, ""), coalesce(name, "") from functions order by va').fetchall()
    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    tmp = a.out + '.tmp'
    with open(tmp, 'w') as f:
        f.write(f'# aw2-syms v1 image=AlanWake2.exe base=0x{BASE:x} count={len(rows)}\n')
        for va, size, owner, name in rows:
            name = name.replace('\t', ' ').replace('\n', ' ')
            f.write(f'{va - BASE:x}\t{(size or 0):x}\t{owner or "unknown"}\t{status.get(va, "-")}\t{name}\n')
    os.replace(tmp, a.out)
    print(f'wrote {len(rows)} symbols ({sum(1 for r in rows if r[3])} named, {len(status)} with progress status) to {a.out}')


if __name__ == '__main__':
    main()
