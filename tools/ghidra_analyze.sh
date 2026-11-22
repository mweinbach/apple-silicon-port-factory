#!/usr/bin/env bash
# Create / update the headless Ghidra project re/ghidra/AW2 for AlanWake2.exe.
#
#   tools/ghidra_analyze.sh            import exe + full auto-analysis + apply fndb names
#                                      (long: hours on this 40 MB .text binary; run in background)
#   tools/ghidra_analyze.sh --names    (re)apply names from db/ghidra_names.tsv (fast, no analysis)
#   tools/ghidra_analyze.sh --export   export functions + call graph to db/ghidra_functions.json
#   tools/ghidra_analyze.sh --status   show whether analysis is running / finished
#
# db/ghidra_names.tsv is produced by `tools/fndb.sh ghidra-names` (done automatically here).
# Logs: out/ghidra/analyze_<timestamp>.log (+ analyze_latest.log symlink),
#       out/ghidra/ghidra_<timestamp>.log (Ghidra application log), script logs alongside.
set -euo pipefail
source "$(dirname "$0")/ghidra_env.sh"

mode="${1:-full}"
ts="$(date +%Y%m%d_%H%M%S)"
LOG="$AW2_GHIDRA_LOGDIR/analyze_${ts}.log"
GLOG="$AW2_GHIDRA_LOGDIR/ghidra_${ts}.log"
SLOG="$AW2_GHIDRA_LOGDIR/script_${ts}.log"
NAMES_TSV="$AW2_ROOT/db/ghidra_names.tsv"
EXPORT_JSON="$AW2_ROOT/db/ghidra_functions.jsonl"

common=( -scriptPath "$AW2_GHIDRA_SCRIPTS" -log "$GLOG" -scriptlog "$SLOG" )

ensure_names() {
  if [[ ! -f "$AW2_ROOT/db/functions.sqlite" ]]; then
    python3 "$AW2_ROOT/tools/fndb/fndb.py" build --exe "$AW2_EXE"
  fi
  python3 "$AW2_ROOT/tools/fndb/fndb.py" ghidra-names --out "$NAMES_TSV"
}

run() {
  ln -sf "$(basename "$LOG")" "$AW2_GHIDRA_LOGDIR/analyze_latest.log"
  {
    echo "# $(date)  mode=$mode"
    echo "# $AW2_HEADLESS $*"
    echo "# heap=$GHIDRA_HEADLESS_MAXMEM cpus=$AW2_GHIDRA_CPUS"
  } >"$LOG"
  local start=$SECONDS rc=0
  "$AW2_HEADLESS" "$@" >>"$LOG" 2>&1 || rc=$?
  echo "# finished rc=$rc after $((SECONDS - start))s at $(date)" >>"$LOG"
  echo "done (rc=$rc, $((SECONDS - start))s). log: $LOG"
  return $rc
}

case "$mode" in
  full)
    aw2_check_lock
    if aw2_project_exists; then
      echo "error: project $AW2_GHIDRA_PROJ_DIR/$AW2_GHIDRA_PROJ_NAME.gpr already exists." >&2
      echo "       use --names / --export, or delete re/ghidra/AW2.gpr + AW2.rep to re-analyze." >&2
      exit 1
    fi
    [[ -f "$AW2_EXE" ]] || { echo "error: exe not found: $AW2_EXE" >&2; exit 1; }
    # The TSV is only read when the post-script runs (after analysis), so a failure here is
    # non-fatal: regenerate it any time before analysis ends, or apply later with --names.
    ensure_names || echo "warning: could not generate $NAMES_TSV now (fndb not built yet?)" >&2
    post=( -postScript AW2ImportNames.java "$NAMES_TSV" )
    echo "starting import + auto-analysis; log: $LOG"
    run "$AW2_GHIDRA_PROJ_DIR" "$AW2_GHIDRA_PROJ_NAME" -import "$AW2_EXE" -loader PeLoader -loader-loadLibraries false \
        -max-cpu "$AW2_GHIDRA_CPUS" "${common[@]}" \
        -preScript AW2AnalysisOptions.java "${post[@]}"
    ;;
  --names)
    aw2_check_lock
    ensure_names
    run "$AW2_GHIDRA_PROJ_DIR" "$AW2_GHIDRA_PROJ_NAME" -process "$AW2_GHIDRA_PROGRAM" -noanalysis \
        "${common[@]}" -postScript AW2ImportNames.java "$NAMES_TSV"
    ;;
  --export)
    aw2_check_lock
    run "$AW2_GHIDRA_PROJ_DIR" "$AW2_GHIDRA_PROJ_NAME" -process "$AW2_GHIDRA_PROGRAM" -noanalysis -readOnly \
        "${common[@]}" -postScript AW2ExportFunctions.java "$EXPORT_JSON"
    echo "now run: tools/fndb.sh ingest-ghidra $EXPORT_JSON"
    ;;
  --status)
    if pgrep -fl "AnalyzeHeadless" | grep -q "$AW2_GHIDRA_PROJ_DIR"; then
      echo "headless Ghidra RUNNING:"; pgrep -fl AnalyzeHeadless | grep "$AW2_GHIDRA_PROJ_DIR" | cut -c1-200
    else
      echo "no headless Ghidra process for this project"
    fi
    latest="$AW2_GHIDRA_LOGDIR/analyze_latest.log"
    [[ -e "$latest" ]] && { echo "--- tail $(readlink "$latest")"; tail -n 15 "$latest"; }
    aw2_project_exists && echo "project: $AW2_GHIDRA_PROJ_DIR/$AW2_GHIDRA_PROJ_NAME.gpr exists" || echo "project: not created"
    ;;
  *)
    sed -n '2,14p' "$0"; exit 2 ;;
esac
