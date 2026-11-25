#!/usr/bin/env bash
# Read-only repository hygiene, crate boundary, copyright, privacy, and db/*.tsv storage audit.
# Usage:
#   .agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh [--staged] [--doctor]
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

STAGED=0
RUN_DOCTOR=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --staged) STAGED=1 ;;
        --doctor) RUN_DOCTOR=1 ;;
        -h|--help)
            echo "Usage: $0 [--staged] [--doctor]" >&2
            exit 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 2
            ;;
    esac
    shift
done

ERRORS=0
fail() { echo "  [FAIL] $*"; ERRORS=$((ERRORS + 1)); }
ok()   { echo "  [OK]   $*"; }

echo "=== Apple Silicon Port Factory — Repo Hygiene Audit ==="

# 1. Zero git submodules / gitlinks
echo "[1/7] Checking submodules & gitlinks..."
if [[ -f .gitmodules ]]; then
    fail ".gitmodules exists (repo must stay single-repo with zero submodules)"
else
    ok "No .gitmodules"
fi
GITLINKS="$(git ls-files --stage | awk '$1 == "160000" { print $4 }')"
if [[ -n "$GITLINKS" ]]; then
    fail "Gitlink submodule entries found: $GITLINKS"
else
    ok "Zero gitlink entries"
fi

# 2. db/*.tsv storage: catalogs are plain text (reviewable diffs, no Git LFS needed to build,
#    test or run doctor); only large analysis tables go to Git LFS.
echo "[2/7] Checking db/*.tsv storage (plain-text catalogs, Git LFS for large tables)..."
LFS_BAD=0
LFS_LIMIT=$((1024 * 1024))
PLAIN_CATALOGS=" db/ops_x86_to_arm64.tsv db/isa_coverage.tsv db/win32_apis.tsv db/shader_formats.tsv db/game_profiles.tsv db/stub_inventory.tsv "
while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    ATTR="$(git check-attr filter -- "$f" | awk -F': ' '{print $NF}')"
    if [[ "$ATTR" == "lfs" ]]; then
        if [[ "$PLAIN_CATALOGS" == *" $f "* ]]; then
            fail "$f is in Git LFS; doctor-checked catalogs must stay plain text"
            LFS_BAD=1
        fi
        continue
    fi
    SIZE="$(git cat-file -s ":$f" 2>/dev/null || echo 0)"
    if (( SIZE > LFS_LIMIT )); then
        fail "$f is $SIZE bytes of plain text; add tables over 1 MiB to Git LFS in .gitattributes"
        LFS_BAD=1
    fi
done < <(git ls-files 'db/*.tsv')
[[ $LFS_BAD -eq 0 ]] && ok "db catalogs are plain text; tables over 1 MiB are in Git LFS"

# 3. Hard crate boundaries
echo "[3/7] Checking crate architectural boundaries..."
BOUNDARY_HITS="$(python3 - <<'PY'
import glob, re
hits = []
for f in ["crates/win32-rt/Cargo.toml"] + glob.glob("crates/win32-rt/src/**/*.rs", recursive=True):
    for idx, line in enumerate(open(f, encoding="utf-8", errors="ignore"), 1):
        stripped = line.lstrip()
        if f.endswith(".rs") and stripped.startswith("//"):
            continue
        if f.endswith(".toml") and stripped.startswith("#"):
            continue
        if re.search(r"\b(?:objc2[-_]metal|aw2[-_]data)\b", stripped):
            hits.append(f"{f}:{idx}: {stripped.strip()}")
for h in hits[:15]:
    print(h)
PY
)"
if [[ -n "$BOUNDARY_HITS" ]]; then
    fail "Forbidden objc2-metal or aw2-data dependency/import in crates/win32-rt:"
    echo "$BOUNDARY_HITS" | sed 's/^/         /'
else
    ok "crates/win32-rt has zero objc2-metal and zero aw2-data imports/dependencies"
fi
if grep -qE '^default\s*=.*aw2' crates/port-loader/Cargo.toml; then
    fail "port-loader default features must not enable aw2/aw2-core"
else
    ok "crates/port-loader default features are free of aw2 targets"
fi

# 4. Copyrighted game binaries, assets, bundles & captures
echo "[4/7] Checking for tracked game binaries, assets, bundles, or captures..."
if [[ $STAGED -eq 1 ]]; then
    FILE_LIST_CMD=(git diff --cached --name-only --diff-filter=ACMR)
else
    FILE_LIST_CMD=(git ls-files)
fi
BAD_FILES="$("${FILE_LIST_CMD[@]}" | grep -iE '\.(exe|dll|rmdblob|rmdtoc|binrfx|bnk|wem|bk2|pak|ucas|utoc|ipa|mobileprovision|png|jpg|jpeg|webp|mp4|mov)$|\.app/|steam_library_browser\.html$' || true)"
if [[ -n "$BAD_FILES" ]]; then
    fail "Tracked binary/asset/bundle/image files found:"
    echo "$BAD_FILES" | sed 's/^/         /' | head -n 25
else
    ok "Zero copyrighted binaries, assets, bundles, or screenshots tracked"
fi

# 5. Hardcoded personal paths (excluding doc placeholders, /Users/Shared, and .agents/mcp_config.json)
echo "[5/7] Scanning tracked text files for hardcoded personal paths..."
PATH_HITS="$(python3 - "$STAGED" <<'PY'
import re, subprocess, sys
staged = sys.argv[1] == "1"
cmd = ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"] if staged else ["git", "ls-files"]
files = [f for f in subprocess.check_output(cmd, text=True).splitlines() if f]
skip_files = {".agents/mcp_config.json"}
user_re = re.compile(r"/Users/([A-Za-z0-9_-]+)")
vol_re = re.compile(r"/Volumes/([A-Za-z0-9_-]+)")
ok_users = {"name", "user", "username", "alice", "bob", "Shared"}
ok_vols = {"MySSD", "SSD", "External", "drive", "Volume", "Data"}
hits = []
for f in files:
    if f in skip_files or f.startswith("db/"):
        continue
    try:
        data = open(f, "rb").read()
    except OSError:
        continue
    if b"\x00" in data[:4096]:
        continue
    text = data.decode("utf-8", errors="ignore")
    for idx, line in enumerate(text.splitlines(), 1):
        for m in user_re.finditer(line):
            if m.group(1) not in ok_users:
                hits.append(f"{f}:{idx}: {m.group(0)}")
        for m in vol_re.finditer(line):
            if m.group(1) not in ok_vols:
                hits.append(f"{f}:{idx}: {m.group(0)}")
for h in hits[:30]:
    print(h)
PY
)"
if [[ -n "$PATH_HITS" ]]; then
    fail "Hardcoded personal paths found:"
    echo "$PATH_HITS" | sed 's/^/         /'
else
    ok "Zero hardcoded personal /Users/ or /Volumes/ paths"
fi

# 6. Zero-new-stub regression guard against db/stub_inventory.tsv
echo "[6/7] Checking zero-new-stub policy against db/stub_inventory.tsv..."
if [[ -f db/stub_inventory.tsv ]]; then
    NEW_STUBS="$(python3 - <<'PY'
import glob, re
from collections import Counter

def strip_comment(line):
    in_s, esc = False, False
    for i, ch in enumerate(line):
        if in_s:
            if esc: esc = False
            elif ch == '\\': esc = True
            elif ch == '"': in_s = False
        elif ch == '"':
            in_s = True
        elif ch == '/' and i + 1 < len(line) and line[i+1] == '/':
            return line[:i]
    return line

def find_macro(line, name):
    pat = name + "!("
    in_s, esc = False, False
    for i in range(len(line)):
        ch = line[i]
        if in_s:
            if esc: esc = False
            elif ch == '\\': esc = True
            elif ch == '"': in_s = False
        elif ch == '"':
            in_s = True
        elif line.startswith(pat, i):
            if i == 0 or not (line[i-1].isalnum() or line[i-1] == '_'):
                return i
    return None

fn_re = re.compile(r'^\s*(?:(?:pub(?:\([^)]*\))?|unsafe|const|async|extern(?:\s+"[^"]+")?)\s+)*fn\s+([A-Za-z0-9_]+)')

base = Counter()
with open("db/stub_inventory.tsv", encoding="utf-8") as fh:
    for idx, line in enumerate(fh):
        if idx == 0 and line.startswith("dll\t"): continue
        cols = line.rstrip("\n").split("\t")
        if len(cols) >= 5:
            base[(cols[1], cols[2], cols[3])] += 1

cur = Counter()
violations = []
for f in sorted(glob.glob("crates/*/src/**/*.rs", recursive=True)):
    rel = f.replace("\\", "/")
    if rel == "crates/win32-rt/src/macros.rs":
        continue
    raw = open(f, encoding="utf-8", errors="ignore").read().split("#[cfg(test)]")[0]
    lines = raw.splitlines()
    cur_fn = "<module>"
    for i, raw_line in enumerate(lines):
        cl = strip_comment(raw_line)
        m = fn_re.match(cl)
        if m:
            cur_fn = m.group(1)
        if find_macro(cl, "stub") is not None:
            # match either stub or middleware_stub in baseline
            k_stub = (cur_fn, "stub", rel)
            k_mw = (cur_fn, "middleware_stub", rel)
            key = k_mw if base[k_mw] > 0 else k_stub
            cur[key] += 1
            if cur[key] > base[key]:
                violations.append(f"{rel}:{i+1}: new stub! in {cur_fn}")
        for mname in ("failing", "win64_stubs"):
            col = find_macro(cl, mname)
            if col is not None:
                # collect balanced parens
                buf, depth, in_s, esc = [], 0, False, False
                for off, rl in enumerate(lines[i:]):
                    sl = strip_comment(rl)
                    sl = sl[col + len(mname) + 1:] if off == 0 else sl
                    done = False
                    for ch in sl:
                        if in_s:
                            buf.append(ch)
                            if esc: esc = False
                            elif ch == '\\': esc = True
                            elif ch == '"': in_s = False
                            continue
                        if ch == '"': in_s = True; buf.append(ch); continue
                        if ch == '(': depth += 1; (buf.append(ch) if depth > 1 else None)
                        elif ch == ')':
                            depth -= 1
                            if depth == 0: done = True; break
                            buf.append(ch)
                        elif depth >= 1: buf.append(ch)
                    if done: break
                    buf.append(' ')
                args = "".join(buf)
                list_part = args.split(";", 1)[1] if ";" in args else args
                for tok in list_part.split(","):
                    ident = "".join(c for c in tok.strip() if c.isalnum() or c == "_")
                    if not ident: continue
                    key = (ident, "failing", rel)
                    cur[key] += 1
                    if cur[key] > base[key]:
                        violations.append(f"{rel}:{i+1}: new {mname}! export {ident}")

for v in violations[:25]:
    print(v)
PY
)"
    if [[ -n "$NEW_STUBS" ]]; then
        fail "New stub!/failing!/win64_stubs! site(s) detected beyond db/stub_inventory.tsv:"
        echo "$NEW_STUBS" | sed 's/^/         /'
    else
        ok "Zero new stub!/failing! sites beyond db/stub_inventory.tsv"
    fi
else
    fail "db/stub_inventory.tsv is missing (run cargo opsdb -- stub-sync)"
fi

# 7. Optional catalog drift check
if [[ $RUN_DOCTOR -eq 1 ]]; then
    echo "[7/7] Running cargo opsdb -- doctor..."
    if cargo opsdb -- doctor; then
        ok "All 5 canonical db/*.tsv catalogs are in sync"
    else
        fail "cargo opsdb -- doctor reported drift (run cargo opsdb -- doctor --fix)"
    fi
else
    echo "[7/7] Skipping cargo opsdb -- doctor (pass --doctor to include)"
fi

echo
if [[ $ERRORS -gt 0 ]]; then
    echo "FAILED: $ERRORS issue(s) detected."
    exit 1
fi
echo "PASSED: Repo hygiene audit clean."
