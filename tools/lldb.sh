#!/usr/bin/env bash
# Debug the hybrid loader with lldb by ATTACHING to it (Rosetta 2 x86_64 process).
#
#   tools/lldb.sh [options] [-- loader args...]   start the loader, attach when it is mapped
#   tools/lldb.sh -p <pid> [options]              attach to a running aw2-loader
#
#     -b <va|rva|name>   breakpoint in AlanWake2.exe (repeatable): VA, RVA or fndb name
#     -x <file>          extra lldb commands (sourced after attaching)
#     -w                 windowed (default: AW2_HEADLESS=1)
#     -t <secs>          watchdog: kill lldb + loader after this long (default 1800, 0 = off)
#     --batch            lldb -b: run -x commands, then `process kill` + quit (scripted use)
#     --keep             leave the loader running when lldb exits (default: kill it)
#     --release          use target/x86_64-apple-darwin/release/aw2-loader
#     --lldb <path>      lldb binary (default: Xcode's via xcrun, else /usr/bin/lldb)
#
# Flow: the loader is started normally with AW2_WAIT_DEBUGGER, maps the exe,
# installs hooks and waits in aw2_debug_image_mapped(); lldb attaches by pid,
# sets the breakpoints (rebased on AW2_EXE_BASE) and releases it by writing
# AW2_DEBUGGER_GO. Loader output goes to out/debug/loader-<pid>.log.
#
# NEVER launch the loader under lldb (`process launch`, especially
# --stop-at-entry): on 2026-10-05 that wedged Rosetta's oahd machine-wide (every
# new x86_64 binary hung in state U until `sudo killall -9 oahd`). This script
# only attaches, and on exit kills lldb's debugserver and the loader.
#
# Inside lldb: aw2sym, aw2b, aw2bt, aw2report, aw2info (tools/debug/lldb_aw2.py).
# SIGSEGV/SIGBUS/SIGILL/SIGFPE/SIGTRAP (-> SEH) and SIGUSR2 (crash-report
# snapshots) are passed to the loader without stopping.
# Does not build: run `cargo loader-build` first.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROFILE=debug
PID=""
HEADLESS=1
EXTRA=""
LLDB=""
BATCH=0
KEEP=0
TIMEOUT=1800
BPS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        -b) BPS+=("$2"); shift 2 ;;
        -p) PID="$2"; shift 2 ;;
        -w|--windowed) HEADLESS=0; shift ;;
        -x) EXTRA="$2"; shift 2 ;;
        -t) TIMEOUT="$2"; shift 2 ;;
        --batch) BATCH=1; shift ;;
        --keep) KEEP=1; shift ;;
        --release) PROFILE=release; shift ;;
        --lldb) LLDB="$2"; shift 2 ;;
        -h|--help) sed -n '2,30p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        --) shift; break ;;
        *) break ;;
    esac
done
BIN="$ROOT/target/x86_64-apple-darwin/$PROFILE/aw2-loader"
[[ -n "$PID" || -x "$BIN" ]] || { echo "missing $BIN (cargo loader-build)" >&2; exit 1; }
if [[ -z "$LLDB" ]]; then
    LLDB="$(xcrun -f lldb 2>/dev/null || echo /usr/bin/lldb)"
fi
SYMS="${AW2_SYMBOLS:-$ROOT/out/symbols/AlanWake2.exe.syms}"
if [[ ! -f "$SYMS" ]]; then
    echo "note: $SYMS missing, generating (tools/debug/export_symbols.py)" >&2
    "$ROOT/tools/debug/export_symbols.py" >&2 || echo "warning: no fndb symbols" >&2
fi

# Kill a process and all its descendants (debugserver is lldb's child).
kill_tree() {
    local p="$1" c
    for c in $(pgrep -P "$p" 2>/dev/null); do kill_tree "$c"; done
    kill -9 "$p" 2>/dev/null || true
}

STARTED=0
LOG=""
if [[ -z "$PID" ]]; then
    mkdir -p "$ROOT/out/debug"
    LOG="$ROOT/out/debug/loader-$$.log"
    env ROSETTA_ADVERTISE_AVX=1 AW2_WAIT_DEBUGGER=120 \
        $([[ $HEADLESS == 1 ]] && echo AW2_HEADLESS=1) \
        "$BIN" "$@" > "$LOG" 2>&1 &
    PID=$!
    STARTED=1
    echo "aw2-loader pid $PID, log $LOG" >&2
    for _ in $(seq 1 600); do
        grep -q "waiting for debugger" "$LOG" 2>/dev/null && break
        kill -0 "$PID" 2>/dev/null || { echo "loader exited before mapping the exe:" >&2; tail -20 "$LOG" >&2; exit 1; }
        sleep 0.1
    done
    grep -q "waiting for debugger" "$LOG" || { echo "loader did not reach aw2_debug_image_mapped in 60 s" >&2; kill_tree "$PID"; exit 1; }
fi

CMDS="$(mktemp -t aw2lldb)"
LLDB_PID=""
WATCHDOG=""
cleanup() {
    rm -f "$CMDS"
    [[ -n "$WATCHDOG" ]] && kill "$WATCHDOG" 2>/dev/null || true
    if [[ -n "$LLDB_PID" ]]; then
        for c in $(pgrep -P "$LLDB_PID" 2>/dev/null); do kill_tree "$c"; done   # debugserver
        kill -9 "$LLDB_PID" 2>/dev/null || true
    fi
    # Orphaned Rosetta debugservers (parent gone -> ppid 1) are what wedged oahd.
    for d in $(pgrep -f "oah/debugserver" 2>/dev/null); do
        [[ "$(ps -o ppid= -p "$d" 2>/dev/null | tr -d ' ')" == 1 ]] && kill -9 "$d" 2>/dev/null || true
    done
    if [[ $STARTED == 1 && $KEEP == 0 ]]; then
        kill_tree "$PID"
    fi
}
trap cleanup EXIT INT TERM HUP

{
    echo "settings set target.x86-disassembly-flavor intel"
    echo "command script import '$ROOT/tools/debug/lldb_aw2.py'"
    echo "process handle SIGSEGV SIGBUS SIGILL SIGFPE SIGTRAP SIGUSR2 -n false -p true -s false"
    echo "process attach --pid $PID"
    echo "process handle SIGSEGV SIGBUS SIGILL SIGFPE SIGTRAP SIGUSR2 -n false -p true -s false"
    echo "aw2attach"
    for b in "${BPS[@]+"${BPS[@]}"}"; do
        echo "aw2b $b"
    done
    echo "aw2go"
    [[ -n "$EXTRA" ]] && echo "command source '$EXTRA'"
    if [[ $BATCH == 1 ]]; then
        echo "process kill"
        echo "quit"
    fi
} > "$CMDS"

export PYTHONPATH="$ROOT/tools/debug${PYTHONPATH:+:$PYTHONPATH}"
ARGS=(-s "$CMDS")
[[ $BATCH == 1 ]] && ARGS=(-b "${ARGS[@]}")
# Explicit stdin: background jobs of a non-interactive shell get /dev/null.
"$LLDB" "${ARGS[@]}" <&0 &
LLDB_PID=$!
if [[ "$TIMEOUT" != 0 ]]; then
    ( sleep "$TIMEOUT"; echo "tools/lldb.sh: watchdog ($TIMEOUT s) - killing lldb and the loader" >&2; kill -TERM $$ ) &
    WATCHDOG=$!
fi
# lldb needs the terminal: wait for it in the foreground.
wait "$LLDB_PID" || true
[[ -n "$LOG" ]] && echo "loader log: $LOG" >&2
exit 0
