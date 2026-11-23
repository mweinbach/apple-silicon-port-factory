#!/usr/bin/env bash
# Read-only repository hygiene, crate boundary, copyright, privacy, and Git LFS audit.
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
echo "[1/6] Checking submodules & gitlinks..."
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

# 2. Git LFS check for db/*.tsv
echo "[2/6] Checking Git LFS tracking for db/*.tsv..."
LFS_BAD=0
while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    ATTR="$(git check-attr filter -- "$f" | awk -F': ' '{print $NF}')"
    if [[ "$ATTR" != "lfs" ]]; then
        fail "$f is not tracked by Git LFS (filter=$ATTR)"
        LFS_BAD=1
    fi
done < <(git ls-files 'db/*.tsv')
[[ $LFS_BAD -eq 0 ]] && ok "All tracked db/*.tsv files match .gitattributes filter=lfs"

# 3. Hard crate boundaries
echo "[3/6] Checking crate architectural boundaries..."
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
echo "[4/6] Checking for tracked game binaries, assets, bundles, or captures..."
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
echo "[5/6] Scanning tracked text files for hardcoded personal paths..."
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

# 6. Optional catalog drift check
if [[ $RUN_DOCTOR -eq 1 ]]; then
    echo "[6/6] Running cargo opsdb -- doctor..."
    if cargo opsdb -- doctor; then
        ok "All 4 canonical db/*.tsv catalogs are in sync"
    else
        fail "cargo opsdb -- doctor reported drift (run cargo opsdb -- doctor --fix)"
    fi
else
    echo "[6/6] Skipping cargo opsdb -- doctor (pass --doctor to include)"
fi

echo
if [[ $ERRORS -gt 0 ]]; then
    echo "FAILED: $ERRORS issue(s) detected."
    exit 1
fi
echo "PASSED: Repo hygiene audit clean."
