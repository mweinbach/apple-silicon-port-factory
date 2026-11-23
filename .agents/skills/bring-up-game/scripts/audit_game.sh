#!/usr/bin/env bash
# Offline bring-up audit battery for a Windows x86-64 game directory.
# Usage:
#   .agents/skills/bring-up-game/scripts/audit_game.sh /path/to/GameDir [MainGame.exe] [--record]
set -euo pipefail

if [[ $# -lt 1 || "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: $0 <GAME_DIR> [MAIN_EXE_NAME_OR_PATH] [--record]" >&2
    exit 2
fi

GAME_DIR="$1"
shift

MAIN_EXE=""
RECORD_FLAG=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --record) RECORD_FLAG="--record" ;;
        *) MAIN_EXE="$1" ;;
    esac
    shift
done

if [[ ! -d "$GAME_DIR" ]]; then
    echo "error: game directory not found: $GAME_DIR" >&2
    exit 1
fi

if [[ -z "$MAIN_EXE" ]]; then
    # Pick the largest top-level .exe (excluding obvious crash/uninstall helpers) as default target.
    MAIN_EXE="$(python3 - "$GAME_DIR" <<'PY'
import os, sys
root = sys.argv[1]
skip = ("unins", "crash", "redist", "setup", "vcredist", "dxsetup")
cands = []
for f in os.listdir(root):
    low = f.lower()
    if low.endswith(".exe") and not any(k in low for k in skip):
        p = os.path.join(root, f)
        if os.path.isfile(p):
            cands.append((os.path.getsize(p), p))
if not cands:
    for dp, _, fns in os.walk(root):
        for f in fns:
            low = f.lower()
            if low.endswith(".exe") and not any(k in low for k in skip):
                p = os.path.join(dp, f)
                cands.append((os.path.getsize(p), p))
if cands:
    cands.sort(reverse=True)
    print(cands[0][1])
PY
)"
elif [[ ! -f "$MAIN_EXE" && -f "$GAME_DIR/$MAIN_EXE" ]]; then
    MAIN_EXE="$GAME_DIR/$MAIN_EXE"
fi

if [[ -z "$MAIN_EXE" || ! -f "$MAIN_EXE" ]]; then
    echo "error: could not locate main .exe in $GAME_DIR (pass it as arg 2)" >&2
    exit 1
fi

echo "=== Port Factory Offline Bring-Up Audit ==="
echo "  Game Dir : $GAME_DIR"
echo "  Main EXE : $MAIN_EXE"
echo

run_step() {
    local title="$1"
    shift
    echo "--------------------------------------------------------------------"
    echo ">>> $title"
    echo "--------------------------------------------------------------------"
    "$@" || echo "[WARN] step returned non-zero: $title"
    echo
}

run_step "1/7 Instruction & ARM64 JIT Coverage (scan)" \
    cargo opsdb -- scan "$GAME_DIR" ${RECORD_FLAG:+$RECORD_FLAG}

run_step "2/7 Win32 / CRT / Middleware Imports (win32-audit)" \
    cargo opsdb -- win32-audit "$GAME_DIR"

run_step "3/7 Anti-Cheat / Direct Syscall / Unpacker Audit (anticheat-audit)" \
    cargo opsdb -- anticheat-audit "$MAIN_EXE"

run_step "4/7 .NET / CLR / Unity Mono / IL2CPP Audit (dotnet-audit)" \
    cargo opsdb -- dotnet-audit "$GAME_DIR"

run_step "5/7 Graphics API & Embedded Shader Audit (app-audit)" \
    cargo opsdb -- app-audit "$MAIN_EXE"

run_step "6/7 Known Engine Profile Detection (game-profiles)" \
    cargo opsdb -- game-profiles "$MAIN_EXE"

run_step "7/7 Static Runtime Hazards: MXCSR/x87/AVX/LOCK/RWX (runtime-audit)" \
    cargo opsdb -- runtime-audit "$GAME_DIR"

echo "=== Audit complete ==="
