#!/usr/bin/env bash
# Boot timer for the hybrid loader: launch → first log line matching a marker
# (default: the game's first CreateDXGIFactory call), optionally sampling the
# process with /usr/bin/sample while it boots.
#
#   tools/perf/boot_time.sh [options] [-- loader args...]
#     -n N               runs (default 1); prints each time and the minimum
#     -m REGEX           marker regex in the log (default 'CreateDXGIFactory')
#     -t SECONDS         give up after this long (default 180)
#     -s AT[:DUR]        run `sample` AT seconds after launch for DUR s (default 10)
#     -o DIR             output dir for logs/samples (default out/perf)
#     --release          use target/x86_64-apple-darwin/release (build it yourself)
#     --windowed         don't set AW2_HEADLESS=1
#   Analyse a sample with: tools/perf/busy.py out/perf/sample_1.txt
#
# Notes:
# * Build first: `cargo loader-build` (this script doesn't build, so timings
#   never include compilation). The first launch after a rebuild is ~1 s slower
#   (Rosetta translates the new binary once).
# * `/usr/bin/sample` is used explicitly: a Python package may shadow `sample`.
set -uo pipefail
cd "$(dirname "$0")/../.."

runs=1 marker='CreateDXGIFactory' timeout=180 sample_at='' sample_dur=10 outdir=out/perf
profile=debug headless=1
while [[ $# -gt 0 ]]; do
    case "$1" in
        -n) runs=$2; shift 2 ;;
        -m) marker=$2; shift 2 ;;
        -t) timeout=$2; shift 2 ;;
        -s) sample_at=${2%%:*}; [[ "$2" == *:* ]] && sample_dur=${2#*:}; shift 2 ;;
        -o) outdir=$2; shift 2 ;;
        --release) profile=release; shift ;;
        --windowed) headless=0; shift ;;
        --) shift; break ;;
        -h|--help) sed -n '2,22p' "$0"; exit 0 ;;
        *) echo "unknown option $1" >&2; exit 2 ;;
    esac
done
bin=target/x86_64-apple-darwin/$profile/aw2-loader
[[ -x "$bin" ]] || { echo "missing $bin (run cargo loader-build)" >&2; exit 2; }
mkdir -p "$outdir"
[[ $headless == 1 ]] && export AW2_HEADLESS=1

now() { perl -MTime::HiRes=time -e 'printf "%.3f\n", time'; }
best=''
for ((i = 1; i <= runs; i++)); do
    log="$outdir/boot_$i.log"
    t0=$(now)
    "$bin" "$@" >"$log" 2>&1 &
    pid=$!; disown "$pid"
    sampled=0 result=''
    while :; do
        el=$(perl -e "printf '%.3f', $(now) - $t0")
        if grep -qE "$marker" "$log" 2>/dev/null; then result="$el"; break; fi
        if ! kill -0 "$pid" 2>/dev/null; then result="exited@$el"; break; fi
        if [[ -n "$sample_at" && $sampled == 0 ]] && perl -e "exit !($el >= $sample_at)"; then
            sampled=1
            /usr/bin/sample "$pid" "$sample_dur" -file "$outdir/sample_$i.txt" >/dev/null 2>&1 &
        fi
        if perl -e "exit !($el >= $timeout)"; then result="timeout@$el"; break; fi
        sleep 0.1
    done
    kill "$pid" 2>/dev/null; sleep 0.3; kill -9 "$pid" 2>/dev/null
    wait 2>/dev/null  # sample, if any
    printf 'run %d: %s s  (log %s%s)\n' "$i" "$result" "$log" "$([[ $sampled == 1 ]] && echo ", sample $outdir/sample_$i.txt")"
    if [[ "$result" =~ ^[0-9.]+$ ]] && { [[ -z "$best" ]] || perl -e "exit !($result < $best)"; }; then best=$result; fi
done
[[ -n "$best" ]] && echo "best: $best s to /$marker/"
