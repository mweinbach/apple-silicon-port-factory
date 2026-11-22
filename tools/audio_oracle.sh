#!/bin/sh
# Audio milestone A2: oracle harness around the game's ORIGINAL statically linked
# Wwise (docs/AUDIO.md, "A2 oracle harness"; crates/aw2-win32/src/audio/{akrec,akoracle}.rs).
#
#   tools/audio_oracle.sh determinism [bank] [event]
#       Exit test: record a realtime harness run with AW2_AK_RECORD, replay the trace
#       offline twice (separate processes), require bit-identical WAVs (i16 + f32).
#   tools/audio_oracle.sh record [bank] [event]      -> $DIR/trace.akt (+ trace.akt.banks/)
#   tools/audio_oracle.sh replay [trace] [out.wav]   offline replay -> out.wav, out.f32.wav, out.txt
#   tools/audio_oracle.sh events <bank,bank,...> <id,id,...|@file> [outdir]
#       offline single-post renders (one WAV pair per event ShortID + results.tsv)
#
# DIR = $AW2_AUDIO_HARNESS_DIR (default out/audio_harness); banks are extracted there
# with `cargo data cat data_pc/audio/events/<name>.bnk` when missing.
# Env passed through: AW2_AK_SEED, AW2_AUDIO_HARNESS_BYPASS_FX=1, AW2_AUDIO_HARNESS_MAX_FRAMES,
# AW2_AUDIO_HARNESS_TAIL_FRAMES, AW2_AK_REPLAY_TAIL_FRAMES, AW2_AUDIO_BACKEND, AW2_AUDIO_DEVICE, RUST_LOG.
set -eu
cd "$(dirname "$0")/.."
DIR=${AW2_AUDIO_HARNESS_DIR:-$PWD/out/audio_harness}
mkdir -p "$DIR"
export AW2_AUDIO_HARNESS_DIR="$DIR"
export RUST_LOG="${RUST_LOG:-info,stub=warn}"

extract() {
    for b in "$@"; do
        if [ ! -s "$DIR/$b.bnk" ]; then
            cargo data cat "data_pc/audio/events/$b.bnk" "$DIR/$b.bnk" >/dev/null
        fi
    done
}

loader() { # <log> ; env already set
    log=$1
    set +e
    cargo loader -- --audio-harness >"$log" 2>&1
    code=$?
    set -e
    return $code
}

record() {
    bank=${1:-play_gui_caseboard_hand_up}
    event=${2:-$bank}
    extract Init "$bank"
    rm -rf "$DIR/trace.akt" "$DIR/trace.akt.banks"
    AW2_AK_RECORD="$DIR/trace.akt" AW2_AUDIO_HARNESS_BANK="$bank" AW2_AUDIO_HARNESS_EVENT="$event" \
        AW2_AUDIO_HARNESS_MODE=realtime loader "$DIR/record.log" || {
        echo "record run failed (exit $?), see $DIR/record.log"
        tail -20 "$DIR/record.log"
        return 1
    }
    grep -E "AK recorder" "$DIR/record.log" | tail -3
}

replay() {
    trace=${1:-$DIR/trace.akt}
    out=${2:-$DIR/replay.wav}
    AW2_AUDIO_HARNESS_MODE=replay AW2_AK_TRACE="$trace" AW2_AK_REPLAY_OUT="$out" \
        loader "${out%.wav}.log" || {
        c=$?
        echo "replay failed (exit $c), see ${out%.wav}.log"
        tail -20 "${out%.wav}.log"
        return $c
    }
    cat "${out%.wav}.txt"
}

case "${1:-determinism}" in
record)
    shift
    record "$@"
    ;;
replay)
    shift
    replay "$@"
    ;;
events)
    shift
    banks=${1:?banks}
    ids=${2:?ids}
    out=${3:-$DIR/events}
    # shellcheck disable=SC2046
    extract $(echo "$banks" | tr ',' ' ')
    AW2_AUDIO_HARNESS_MODE=events AW2_AUDIO_HARNESS_BANKS="$banks" AW2_AUDIO_HARNESS_EVENT_IDS="$ids" \
        AW2_AUDIO_HARNESS_OUT="$out" loader "$out.log" || {
        c=$?
        echo "events run exit $c, see $out.log"
    }
    grep -E "events: [0-9]+ events in" "$out.log" || tail -20 "$out.log"
    column -t "$out/results.tsv" 2>/dev/null | cut -c1-160 | head -40
    ;;
determinism)
    shift
    record "$@"
    replay "$DIR/trace.akt" "$DIR/replay1.wav"
    replay "$DIR/trace.akt" "$DIR/replay2.wav"
    ok=0
    for ext in wav f32.wav; do
        if cmp -s "$DIR/replay1.$ext" "$DIR/replay2.$ext"; then
            echo "replay1.$ext == replay2.$ext ($(wc -c <"$DIR/replay1.$ext" | tr -d ' ') bytes, bit-identical)"
        else
            echo "replay1.$ext != replay2.$ext"
            ok=2
        fi
    done
    exit $ok
    ;;
*)
    sed -n '2,17p' "$0"
    exit 1
    ;;
esac
