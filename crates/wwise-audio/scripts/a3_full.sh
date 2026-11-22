#!/bin/sh
# A3 exit-test driver (full set). Builds patch-2d banks (+ PCM variants), runs the A2 oracle in
# groups, renders natively and null-tests.
#   a3_full.sh <list.tsv: bank<TAB>event...> <out dir> [group size]
# Env: RENDER_ONLY=1 reuses existing banks and oracle captures; MODES="p2d pcm" (default).
# Layout: <out>/p2d (derived banks, Vorbis media), <out>/pcm (same + media as in-bank PCM16 of our
# decode), each with ev/ (oracle captures) and ours/ (native renders), and <out>/{p2d,pcm}.tsv.
set -eu
list=$1; out=$2; group=${3:-16}
root=$(pwd)
mkdir -p "$out/p2d" "$out/pcm"
render_only=${RENDER_ONLY:-0}
banks=$(grep -v '^#' "$list" | cut -f1 | sed 's/\.bnk$//' | sort -u)
if [ "$render_only" != 1 ]; then
args=""
for b in $banks; do args="$args --bank $b"; done
# shellcheck disable=SC2086
./target/release/wwise-audio patch-2d --out-dir "$out/p2d" $args
cp "$out/p2d/Init.bnk" "$out/pcm/Init.bnk"
for b in $banks; do
    [ -f "$out/pcm/$b.bnk" ] || ./target/release/examples/pcm_bank "$out/p2d/$b.bnk" "$out/pcm/$b.bnk" 2>/dev/null
done
fi
for mode in ${MODES:-p2d pcm}; do
    D="$root/$out/$mode"
    mkdir -p "$D/ev"
    if [ "$render_only" != 1 ]; then
    : >"$D/ev/all_results.tsv"
    grep -v '^#' "$list" | split -l "$group" - "$D/ev/grp."
    for g in "$D"/ev/grp.*; do
        B="Init,$(cut -f1 "$g" | sed 's/\.bnk$//' | paste -sd, -)"
        I=$(cut -f2 "$g" | paste -sd, -)
        AW2_AUDIO_HARNESS_DIR="$D" AW2_AUDIO_HARNESS_BYPASS_FX=1 AW2_AK_FORCE_SSE=1 AW2_AUDIO_BACKEND=null \
            out/wt-a3/tools/audio_oracle.sh events "$B" "$I" "$D/ev" >"$g.log" 2>&1 || echo "oracle group $g failed"
        tail -n +2 "$D/ev/results.tsv" >>"$D/ev/all_results.tsv" 2>/dev/null || true
        rm -f "$g"
    done
    fi
    mkdir -p "$D/ours"
    : >"$root/$out/$mode.tsv"
    grep -v '^#' "$list" | while IFS="$(printf '\t')" read -r bank ev rest; do
        bank=${bank%.bnk}
        if ! ./target/release/wwise-audio render --bank-dir "$D" --bank "$bank" --event "$ev" --out "$D/ours/$ev.wav" >"$D/ours/$ev.log" 2>&1; then
            printf '%s\t%s\trender-failed\n' "$bank" "$ev" >>"$root/$out/$mode.tsv"; continue
        fi
        if [ ! -f "$D/ev/$ev.f32.wav" ]; then
            printf '%s\t%s\tno-oracle\n' "$bank" "$ev" >>"$root/$out/$mode.tsv"; continue
        fi
        r=$(./target/release/wwise-audio nulltest "$D/ev/$ev.f32.wav" "$D/ours/$ev.wav" 2>&1 | tail -1)
        printf '%s\t%s\t%s\n' "$bank" "$ev" "$r" >>"$root/$out/$mode.tsv"
    done
done
