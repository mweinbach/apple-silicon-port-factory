#!/bin/sh
# Audio milestone A1 check: run the game's own Wwise (via its exported AK API)
# through our WASAPI-on-CoreAudio layer and compare our PCM tap with Wwise's
# AK::SoundEngine::StartOutputCapture. See crates/aw2-win32/src/audio/harness.rs.
#
#   tools/audio_harness.sh [bank] [event]
#
# Env passed through: AW2_AUDIO_DEVICE (e.g. "studio speakers"), AW2_AUDIO_BACKEND=null,
# AW2_AUDIO_HARNESS_SECONDS, AW2_AUDIO_HARNESS_REPEAT_MS, RUST_LOG.
# Output: out/audio_harness/{tap.wav,tap.wav.tsv,capture.wav,report.txt,run.log}.
# Exit code: 0 = bit-exact, 2 = mismatch, other = setup failure.
set -eu
cd "$(dirname "$0")/.."
BANK=${1:-play_gui_caseboard_hand_up}
EVENT=${2:-$BANK}
DIR="$PWD/out/audio_harness"
mkdir -p "$DIR"
for b in Init "$BANK"; do
    if [ ! -s "$DIR/$b.bnk" ]; then
        cargo data cat "data_pc/audio/events/$b.bnk" "$DIR/$b.bnk" >/dev/null
    fi
done
set +e
AW2_AUDIO_HARNESS_DIR="$DIR" AW2_AUDIO_HARNESS_BANK="$BANK" AW2_AUDIO_HARNESS_EVENT="$EVENT" \
    RUST_LOG="${RUST_LOG:-info,stub=warn}" cargo loader -- --audio-harness >"$DIR/run.log" 2>&1
code=$?
set -e
cat "$DIR/report.txt" 2>/dev/null || tail -40 "$DIR/run.log"
exit $code
