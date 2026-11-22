#!/usr/bin/env python3
"""Executed functions of a game DLL, from its AOT JIT profile, for `gen_subsystems.py --rva-list`.

    python3 tools/decomp_scale/dll_profile_fns.py <dll> <profile.tsv> [-o out.txt]

The profile (`~/Library/Caches/aw2-port/aot/profile/<module>-<hash>.tsv`, `rva<TAB>count`,
written with `AW2_JIT_PROFILE=1`) lists every image block the emulator translated. Each block is
mapped to the `.pdata` primary function containing it (chained fragments resolved); blocks at an
exported start without `.pdata` (leaf functions) are kept as is. Prints the coverage and writes
the function start RVAs (hex, one per line).
"""

import argparse
import bisect
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "fndb"))
sys.path.insert(0, str(ROOT / "tools" / "decomp_scale"))
import fndb  # noqa: E402
import dll_lift  # noqa: E402


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("dll")
    ap.add_argument("profile")
    ap.add_argument("-o", "--out", default=None)
    a = ap.parse_args()
    pe = fndb.PE(a.dll)
    ents = sorted(pe.pdata())
    starts = [b for b, _, _ in ents]
    exports = {frva for _, frva, ename in pe.exports() if ename}
    root, size = {}, {}
    for b, e, u in ents:
        p, _, _ = dll_lift._unwind_primary(pe, u)
        r = p if p is not None else b
        root[b] = r
        size[r] = size.get(r, 0) + (e - b)
    hit, leaves, outside = set(), set(), 0
    for line in Path(a.profile).read_text().splitlines():
        f = line.split()
        if len(f) < 2:
            continue
        try:
            rva = int(f[0], 16)
        except ValueError:
            continue
        i = bisect.bisect_right(starts, rva) - 1
        if i >= 0 and ents[i][0] <= rva < ents[i][1]:
            hit.add(root[ents[i][0]])
        elif rva in exports:
            leaves.add(rva)
        else:
            outside += 1
    tb = sum(size.values()) or 1
    hb = sum(size[r] for r in hit)
    print(
        f"{Path(a.dll).name}: {len(hit)}/{len(size)} .pdata functions executed ({hb}/{tb} bytes, {hb * 100 / tb:.1f}%), "
        f"{len(leaves)} exported leaves, {outside} profiled blocks in other code"
    )
    if a.out:
        Path(a.out).write_text("".join(f"0x{r:x}\n" for r in sorted(hit | leaves)))


if __name__ == "__main__":
    main()
