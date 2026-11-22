#!/usr/bin/env bash
# Print Ghidra's decompilation of one or more functions from the re/ghidra/AW2 project.
#
#   tools/ghidra_decompile.sh <va|name> [<va|name> ...]   decompile (cached in out/decomp/<va>.c)
#   tools/ghidra_decompile.sh --fresh <va> ...            ignore cache
#   tools/ghidra_decompile.sh --asm <va>                  objdump disassembly only (no Ghidra)
#   tools/ghidra_decompile.sh --server-start              keep Ghidra open (fast: ~1 s/function,
#                                                         safe for many concurrent callers)
#   tools/ghidra_decompile.sh --server-stop | --server-status
#
# Names are resolved through fndb (tools/fndb.sh show <substring> to search).
# Without a running server each call opens the project headless (slow, tens of seconds) and
# takes a mutex so concurrent callers queue up. If the project is unavailable/locked by another
# job (analysis, --names, --export), falls back to annotated objdump disassembly.
set -euo pipefail
source "$(dirname "$0")/ghidra_env.sh"

OUT="$AW2_ROOT/out/decomp"
Q="$OUT/queue"
MUTEX="$OUT/.oneshot.lock"
mkdir -p "$OUT" "$Q"
FNDB=(python3 "$AW2_ROOT/tools/fndb/fndb.py")

server_alive() {
  [[ -f "$Q/heartbeat" ]] || return 1
  local hb now
  hb=$(cat "$Q/heartbeat" 2>/dev/null || echo 0)
  now=$(python3 -c 'import time;print(int(time.time()*1000))')
  (( now - hb < 10000 ))
}

fresh=0; asm=0; keys=()
while (($#)); do
  case "$1" in
    --fresh) fresh=1 ;;
    --asm) asm=1 ;;
    --server-start)
      if server_alive; then echo "decompile server already running"; exit 0; fi
      aw2_check_lock || exit 1
      rm -f "$Q/stop"
      LOG="$AW2_GHIDRA_LOGDIR/decomp_server_$(date +%Y%m%d_%H%M%S).log"
      nohup "$AW2_HEADLESS" "$AW2_GHIDRA_PROJ_DIR" "$AW2_GHIDRA_PROJ_NAME" -process "$AW2_GHIDRA_PROGRAM" \
        -noanalysis -readOnly -scriptPath "$AW2_GHIDRA_SCRIPTS" \
        -postScript AW2Decompile.java "$OUT" --serve "$Q" "${AW2_DECOMP_IDLE_MIN:-240}" >"$LOG" 2>&1 &
      echo "starting decompile server (pid $!), log: $LOG"
      for _ in $(seq 1 180); do server_alive && { echo "server ready"; exit 0; }; sleep 1; done
      echo "server not ready after 180 s; check $LOG" >&2; exit 1 ;;
    --server-stop)
      if ! server_alive; then echo "decompile server: not running"; rm -f "$Q/heartbeat"; exit 0; fi
      touch "$Q/stop"
      for _ in $(seq 1 100); do [[ -f "$Q/heartbeat" ]] || break; sleep 0.2; done
      # wait for the JVM to release the project lock
      for _ in $(seq 1 100); do pgrep -f "AW2Decompile.java $OUT --serve" >/dev/null || break; sleep 0.2; done
      rm -f "$Q/heartbeat" "$Q/stop"; echo "decompile server stopped"; exit 0 ;;
    --server-status) if server_alive; then echo "decompile server: running"; else echo "decompile server: not running"; fi; exit 0 ;;
    -h|--help) sed -n '2,16p' "$0"; exit 0 ;;
    *) keys+=("$1") ;;
  esac
  shift
done
((${#keys[@]})) || { sed -n '2,16p' "$0"; exit 2; }

vas=()
for k in "${keys[@]}"; do
  v=$("${FNDB[@]}" resolve "$k") || { echo "error: cannot resolve '$k' (try: tools/fndb.sh show '$k')" >&2; exit 1; }
  vas+=("$v")
done

if ((asm)); then
  for v in "${vas[@]}"; do "${FNDB[@]}" disasm "$v"; done; exit 0
fi

need=()
for v in "${vas[@]}"; do
  f="$OUT/${v#0x}.c"
  if ((fresh)) || [[ ! -s "$f" ]]; then rm -f "$f"; need+=("$v"); fi
done

fallback() {
  echo "// ghidra_decompile: $1 -- falling back to disassembly" >&2
  for v in "${vas[@]}"; do [[ -s "$OUT/${v#0x}.c" ]] && cat "$OUT/${v#0x}.c" || "${FNDB[@]}" disasm "$v"; done
  exit 0
}

if ((${#need[@]})); then
  if server_alive; then
    req="$Q/$$_$RANDOM"
    printf '%s\n' "${need[@]}" >"$req.tmp" && mv "$req.tmp" "$req.req"
    for i in $(seq 1 1800); do
      [[ -e "$req.done" ]] && break
      if (( i % 50 == 0 )) && ! server_alive; then echo "// decompile server died" >&2; break; fi
      sleep 0.1
    done
    rm -f "$req.done" "$req.req"
  else
    aw2_project_exists || fallback "no Ghidra project (run tools/ghidra_analyze.sh)"
    # serialize one-shot runs (the Ghidra project allows a single opener)
    for _ in $(seq 1 600); do mkdir "$MUTEX" 2>/dev/null && break; sleep 1; done
    trap 'rmdir "$MUTEX" 2>/dev/null || true' EXIT
    aw2_check_lock 2>/dev/null || fallback "project locked by another Ghidra job"
    LOG="$AW2_GHIDRA_LOGDIR/decomp_oneshot.log"
    "$AW2_HEADLESS" "$AW2_GHIDRA_PROJ_DIR" "$AW2_GHIDRA_PROJ_NAME" -process "$AW2_GHIDRA_PROGRAM" \
      -noanalysis -readOnly -scriptPath "$AW2_GHIDRA_SCRIPTS" \
      -postScript AW2Decompile.java "$OUT" "${need[@]}" >"$LOG" 2>&1 || true
    rmdir "$MUTEX" 2>/dev/null || true
  fi
fi

for v in "${vas[@]}"; do
  f="$OUT/${v#0x}.c"
  if [[ -s "$f" ]]; then cat "$f"; else echo "// no decompilation produced for $v (see out/ghidra/*.log)"; "${FNDB[@]}" disasm "$v"; fi
  echo
done
