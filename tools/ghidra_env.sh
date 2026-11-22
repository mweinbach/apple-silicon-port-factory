# shellcheck shell=bash
# Common environment for the Ghidra helper scripts. Source, don't execute.
#
# Overridable env vars:
#   AW2_EXE                path to AlanWake2.exe (or set AW2_GAME_DIR)
#   GHIDRA_INSTALL_DIR     Ghidra install dir containing support/analyzeHeadless
#   GHIDRA_HEADLESS_MAXMEM JVM heap for headless runs (default 48G)
#   AW2_GHIDRA_CPUS        max CPUs for analysis (default 24)

AW2_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AW2_EXE="${AW2_EXE:-${AW2_GAME_DIR:+${AW2_GAME_DIR%/}/AlanWake2.exe}}"
if [[ -z "${AW2_EXE:-}" ]]; then
  echo "error: set AW2_EXE=/path/to/AlanWake2.exe or AW2_GAME_DIR=/path/to/AlanWake2" >&2
  return 1 2>/dev/null || exit 1
fi
AW2_GHIDRA_PROJ_DIR="$AW2_ROOT/re/ghidra"
AW2_GHIDRA_PROJ_NAME="AW2"
AW2_GHIDRA_PROGRAM="$(basename "$AW2_EXE")"
AW2_GHIDRA_SCRIPTS="$AW2_ROOT/re/ghidra"
AW2_GHIDRA_LOGDIR="$AW2_ROOT/out/ghidra"
mkdir -p "$AW2_GHIDRA_LOGDIR"

if [[ -z "${GHIDRA_INSTALL_DIR:-}" ]]; then
  for c in /opt/homebrew/opt/ghidra/libexec /usr/local/opt/ghidra/libexec \
           /Applications/ghidra* "$HOME"/ghidra*; do
    if [[ -x "$c/support/analyzeHeadless" ]]; then GHIDRA_INSTALL_DIR="$c"; break; fi
  done
fi
if [[ -z "${GHIDRA_INSTALL_DIR:-}" || ! -x "$GHIDRA_INSTALL_DIR/support/analyzeHeadless" ]]; then
  echo "error: Ghidra not found. Install with 'brew install ghidra' or set GHIDRA_INSTALL_DIR." >&2
  return 1 2>/dev/null || exit 1
fi
AW2_HEADLESS="$GHIDRA_INSTALL_DIR/support/analyzeHeadless"
export GHIDRA_HEADLESS_MAXMEM="${GHIDRA_HEADLESS_MAXMEM:-48G}"
AW2_GHIDRA_CPUS="${AW2_GHIDRA_CPUS:-24}"

aw2_project_exists() { [[ -f "$AW2_GHIDRA_PROJ_DIR/$AW2_GHIDRA_PROJ_NAME.gpr" ]]; }

# Fail fast if another headless instance holds the project lock.
aw2_check_lock() {
  if ls "$AW2_GHIDRA_PROJ_DIR/$AW2_GHIDRA_PROJ_NAME".lock* >/dev/null 2>&1; then
    if pgrep -f "AnalyzeHeadless.*$AW2_GHIDRA_PROJ_DIR" >/dev/null 2>&1; then
      echo "error: Ghidra project is locked by a running headless process (analysis still running?)." >&2
      echo "       monitor: tail -f $AW2_GHIDRA_LOGDIR/analyze_latest.log" >&2
      return 1
    fi
  fi
  return 0
}
