#!/bin/sh
# A3 null test driver: render each (bank, event) of a list with wwise-audio on the patch-2d
# derived banks and null-test against the A2 oracle's float capture.
#   a3_null.sh <list.tsv: bank<TAB>event...> <oracle ev dir> <our out dir> [extra render opts]
set -eu
list=$1; odir=$2; ours=$3; shift 3
mkdir -p "$ours"
while IFS="$(printf '\t')" read -r bank ev rest; do
    case "$bank" in \#*|"") continue ;; esac
    ./target/release/wwise-audio render --patch-2d --bank "$bank" --event "$ev" --out "$ours/$ev.wav" "$@" >"$ours/$ev.log" 2>&1 || { echo "$bank	$ev	render-failed"; continue; }
    r=$(./target/release/wwise-audio nulltest "$odir/$ev.f32.wav" "$ours/$ev.wav" 2>&1)
    echo "$bank	$ev	$r"
done <"$list"
