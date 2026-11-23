#!/usr/bin/env python3
"""Static guardrail checker for crates/win32-rt Rust modules.

Mirrors crates/win32-rt/build.rs export scanning rules and checks:
  1. Zero `objc2` / `objc2_metal` / `aw2_data` references inside `win32-rt`
  2. No top-level Win64 exports (`extern "win64"`, `failing!`, `win64_stubs!`,
     `context_capturing_export!`, `win64_varargs_thunk!`, `// @exports`) placed
     AFTER the first `#[cfg(test)]` (build.rs silently drops them!)
  3. Multiple `#[cfg(test)]` blocks warning (build.rs stops at the FIRST one)
  4. For `src/misc/<stem>.rs`: verifies `EXTRA_EXPORTS`, `ORDINALS`, and
     `pub mod <stem>;` registration in `crates/win32-rt/src/misc/mod.rs`
  5. Flags `static mut` usage and personal `/Users/<name>` paths
  6. Reports exported symbol count and remaining `stub!`/`failing!`/`win64_stubs!` counts

Usage:
  python3 .agents/skills/implement-win32-api/scripts/audit_win32_module.py crates/win32-rt/src/misc/foo.rs [...]
  python3 .agents/skills/implement-win32-api/scripts/audit_win32_module.py --all-misc
"""

import glob
import os
import re
import sys
from pathlib import Path


def scan_exports(src: str) -> list[str]:
    names: list[str] = []
    for line in src.splitlines():
        t = line.lstrip()
        if t.startswith("// @exports "):
            for sym in t[len("// @exports "):].split():
                if sym and sym not in names:
                    names.append(sym)
            continue
        if len(t) != len(line):
            continue
        rest = None
        for pfx in (
            'pub unsafe extern "win64" fn ',
            'pub extern "win64" fn ',
            "context_capturing_export!(",
            "win64_varargs_thunk!(",
        ):
            if t.startswith(pfx):
                rest = t[len(pfx):]
                break
        if rest is None:
            continue
        m = re.match(r"([A-Za-z0-9_]+)", rest)
        if m:
            nm = m.group(1)
            if not nm.startswith(("dispatch_", "aw2_")) and nm not in names:
                names.append(nm)

    for pfx in ("failing!(", "win64_stubs!("):
        rem = src
        while True:
            pos = rem.find(pfx)
            if pos < 0:
                break
            after = rem[pos + len(pfx):]
            end = after.find(");")
            if end < 0:
                break
            block = after[:end]
            list_part = block.split(";", 1)[1] if ";" in block else block
            for raw in list_part.split(","):
                m = re.match(r"([A-Za-z0-9_]+)", raw.strip())
                if m:
                    nm = m.group(1)
                    if not nm.startswith(("dispatch_", "aw2_")) and nm not in names:
                        names.append(nm)
            rem = after[end + 2:]
    return names


def audit_file(path: Path, misc_mod_src: str) -> tuple[int, int]:
    text = path.read_text(encoding="utf-8")
    errors = 0
    warnings = 0

    parts = text.split("#[cfg(test)]")
    before_test = parts[0]
    after_test = "#[cfg(test)]".join(parts[1:]) if len(parts) > 1 else ""

    exports_ok = scan_exports(before_test)
    exports_lost = scan_exports(after_test) if after_test else []

    print(f"=== {path} ===")
    print(f"  Exports scanned by build.rs : {len(exports_ok)}")

    if exports_lost:
        print(
            f"  [ERROR] {len(exports_lost)} export(s) appear AFTER #[cfg(test)] and will be "
            f"SILENTLY DROPPED by build.rs: {', '.join(exports_lost[:10])}"
        )
        errors += 1

    if len(parts) > 2:
        print(
            f"  [WARN]  Found {len(parts) - 1} #[cfg(test)] markers; build.rs stops at the FIRST one"
        )
        warnings += 1

    # Hard crate-boundary rules (check non-comment code lines only)
    code_lines = [
        ln for ln in text.splitlines()
        if not ln.lstrip().startswith("//")
    ]
    code_only = "\n".join(code_lines)
    for pat, label in (
        (r"\bobjc2_metal\b", "objc2_metal"),
        (r"\baw2_data\b", "aw2_data"),
    ):
        if re.search(pat, code_only):
            print(f"  [ERROR] Forbidden crate reference in code inside win32-rt: {label}")
            errors += 1

    # Personal path check (ignore generic doc placeholders and macOS /Users/Shared)
    for m in re.finditer(r"/Users/([A-Za-z0-9_-]+)", text):
        if m.group(1) not in ("name", "user", "username", "Shared"):
            print(f"  [ERROR] Hardcoded personal path detected: {m.group(0)}")
            errors += 1

    # Rust 2024 lint hygiene
    static_mut_hits = len(re.findall(r"\bstatic\s+mut\b", before_test))
    if static_mut_hits:
        print(f"  [WARN]  {static_mut_hits} `static mut` declaration(s) in non-test code (prefer Atomic/Mutex/OnceLock)")
        warnings += 1

    # Misc DLL contract
    rel = str(path).replace("\\", "/")
    if "/src/misc/" in rel and path.parent.name == "misc" and path.name != "mod.rs":
        stem = path.stem
        if not re.search(r"\bpub\s+(?:static|const)\s+EXTRA_EXPORTS\b", before_test):
            print("  [ERROR] Missing `pub static/const EXTRA_EXPORTS` before #[cfg(test)]")
            errors += 1
        if not re.search(r"\bpub\s+(?:static|const)\s+ORDINALS\b", before_test):
            print("  [ERROR] Missing `pub static/const ORDINALS` before #[cfg(test)]")
            errors += 1
        if f"pub mod {stem};" not in misc_mod_src:
            print(f"  [ERROR] `pub mod {stem};` not found in crates/win32-rt/src/misc/mod.rs")
            errors += 1

    stub_count = len(re.findall(r"\bstub!\s*\(", before_test))
    failing_count = len(re.findall(r"\b(?:failing|win64_stubs)!\s*\(", before_test))
    print(f"  Honest stubs in module      : stub!={stub_count}, failing!/win64_stubs!={failing_count}")

    if errors == 0 and warnings == 0:
        print("  [OK]    All guardrails passed.")
    return errors, warnings


def main() -> int:
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print(__doc__)
        return 2

    repo = Path(__file__).resolve().parents[4]
    misc_mod = repo / "crates/win32-rt/src/misc/mod.rs"
    misc_mod_src = misc_mod.read_text(encoding="utf-8") if misc_mod.is_file() else ""

    targets: list[Path] = []
    for arg in sys.argv[1:]:
        if arg == "--all-misc":
            for p in sorted(glob.glob(str(repo / "crates/win32-rt/src/misc/*.rs"))):
                if not p.endswith("/mod.rs"):
                    targets.append(Path(p))
        else:
            p = Path(arg)
            if not p.is_absolute():
                p = (Path.cwd() / p).resolve()
            targets.append(p)

    total_err = 0
    total_warn = 0
    for t in targets:
        if not t.is_file():
            print(f"[ERROR] File not found: {t}")
            total_err += 1
            continue
        e, w = audit_file(t, misc_mod_src)
        total_err += e
        total_warn += w

    print(f"\nSummary: {len(targets)} file(s) audited, {total_err} error(s), {total_warn} warning(s).")
    return 1 if total_err else 0


if __name__ == "__main__":
    sys.exit(main())
