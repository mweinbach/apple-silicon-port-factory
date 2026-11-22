#!/usr/bin/env python3
"""`db/dll_progress.tsv`: per-function status of the whole-DLL lifts (`dll_lift.py`).

Columns: module, rva, name, status, leaf, body_hash, difftest, tags, ran_calls, notes.

- `status`: `draft` (lifted, registered when the image is installed) or `rejected` (stays guest
  code; `notes` has the generator's reason). Lifted DLL code stays `draft` until shadow
  sampling in gameplay promotes it.
- `leaf`: 1 when the body makes no `call_va*` calls.
- `body_hash`: 64-bit BLAKE2b of the emitted body; a relift that changes it resets `difftest`.
- `difftest` / `tags`: the last `dll_lift::<module>` difftest result for the function
  (`verified` / `unverified` / `mismatch` / `not_fuzzed`, `untested` before any run) and its
  listed classes (`abi_narrow`, `noncode`, `rax_stack`, `nan_sign`).
- `ran_calls`: calls counted in the last gameplay run with `AW2_DLL_LIFT_COV=1` (empty = no
  coverage ingested for this body).

`dll_lift.emit` calls `update_lift` on every lift; after a difftest run:

    python3 tools/decomp_scale/dll_progress.py --ingest <AW2_LOG_DIR>/AlanWake2.log

and after a gameplay run with coverage (`cov\t<module>\t+0x<rva>\t<calls>` lines):

    python3 tools/decomp_scale/dll_progress.py --ingest-cov <cov.txt>
"""

from pathlib import Path
import os
import sys

ROOT = Path(__file__).resolve().parents[2]
TSV = ROOT / "db" / "dll_progress.tsv"
COLS = ["module", "rva", "name", "status", "leaf", "body_hash", "difftest", "tags", "ran_calls", "notes"]


def load(path: Path = TSV) -> dict:
    rows = {}
    if not path.exists():
        return rows
    lines = path.read_text().splitlines()
    hdr = lines[0].split("\t") if lines else COLS
    for ln in lines[1:]:
        f = ln.split("\t")
        r = {c: "" for c in COLS}
        r.update(zip(hdr, f))
        rows[(r["module"], int(r["rva"], 16))] = r
    return rows


def save(rows: dict, path: Path = TSV):
    out = ["\t".join(COLS)]
    for (_, _), r in sorted(rows.items(), key=lambda kv: (kv[0][0], kv[0][1])):
        out.append("\t".join(str(r.get(c, "")).replace("\t", " ").replace("\n", " ") for c in COLS))
    path.write_text("\n".join(out) + "\n")


def update_lift(module: str, lifted: list, rejected: list, path: Path = TSV):
    """Replace `module`'s rows. `lifted`: (rva, name, leaf, body_hash); `rejected`: (rva, name,
    reason). The difftest result of a lifted function is kept while its body is unchanged.
    `AW2_DLL_PROGRESS=0` (dry runs into other output dirs): no update."""
    if os.environ.get("AW2_DLL_PROGRESS") == "0":
        return len(lifted), len(rejected)
    old = load(path)
    rows = {k: r for k, r in old.items() if k[0] != module}
    for rva, name, leaf, h in lifted:
        prev = old.get((module, rva))
        keep = prev is not None and prev["status"] == "draft" and prev["body_hash"] == f"{h:016x}"
        rows[(module, rva)] = {
            "module": module,
            "rva": f"0x{rva:x}",
            "name": name,
            "status": "draft",
            "leaf": str(int(leaf)),
            "body_hash": f"{h:016x}",
            "difftest": prev["difftest"] if keep else "untested",
            "tags": prev["tags"] if keep else "",
            "ran_calls": prev.get("ran_calls", "") if keep else "",
            "notes": prev.get("notes", "") if keep else "",
        }
    for rva, name, reason in rejected:
        rows[(module, rva)] = {
            "module": module,
            "rva": f"0x{rva:x}",
            "name": name,
            "status": "rejected",
            "leaf": "",
            "body_hash": "",
            "difftest": "",
            "tags": "",
            "ran_calls": "",
            "notes": reason,
        }
    save(rows, path)
    return len(lifted), len(rejected)


def ingest(log: Path, path: Path = TSV):
    """Apply the `dll_lift difftest fn\\t<module>\\t+0x<rva>\\t<result>\\t<tags>` lines of a
    difftest log (later lines win). Rows not in the TSV are reported and skipped."""
    rows = load(path)
    seen, missing = 0, []
    marker = "dll_lift difftest fn\t"
    for ln in log.read_text(errors="replace").splitlines():
        i = ln.find(marker)
        if i < 0:
            continue
        f = ln[i + len(marker):].split("\t")
        if len(f) < 3:
            continue
        module, rva, res = f[0], int(f[1].lstrip("+"), 16), f[2]
        tags = f[3] if len(f) > 3 else ""
        r = rows.get((module, rva))
        if r is None or r["status"] != "draft":
            missing.append(f"{module}+0x{rva:x}")
            continue
        r["difftest"], r["tags"] = res, tags
        seen += 1
    save(rows, path)
    print(f"{path}: {seen} difftest result(s) applied, {len(missing)} not lifted in the TSV {missing[:10]}")


def ingest_cov(cov: Path, path: Path = TSV):
    """Apply `cov\t<module>\t+0x<rva>\t<calls>` lines (`AW2_DLL_LIFT_COV=1` report)."""
    rows = load(path)
    seen, missing = 0, []
    for ln in cov.read_text(errors="replace").splitlines():
        f = ln.split("\t")
        if len(f) != 4 or f[0] != "cov":
            continue
        r = rows.get((f[1], int(f[2].lstrip("+"), 16)))
        if r is None or r["status"] != "draft":
            missing.append(f"{f[1]}{f[2]}")
            continue
        r["ran_calls"] = f[3]
        seen += 1
    save(rows, path)
    print(f"{path}: {seen} coverage count(s) applied, {len(missing)} not lifted in the TSV {missing[:10]}")


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--ingest":
        ingest(Path(sys.argv[2]))
    elif len(sys.argv) == 3 and sys.argv[1] == "--ingest-cov":
        ingest_cov(Path(sys.argv[2]))
    else:
        sys.exit(__doc__)
