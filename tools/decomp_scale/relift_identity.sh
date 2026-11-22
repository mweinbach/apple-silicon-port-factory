#!/bin/zsh
# Exe-mode identity check for gen_subsystems.py: regenerate the exe shards with the
# generator from <ref> (default: merge-base of HEAD and main) and with the working-tree generator,
# and require byte-identical output. Run after any generator change (DLL-mode hooks such as
# READ_TABLE / EXTRA_BAD_MNEMS must keep their exe defaults neutral).
#
#   tools/decomp_scale/relift_identity.sh [<ref>] [<out dir>]
#
# Exit 0 = identical; 1 = differs (diff in <out dir>/diff.txt); 2 = a generator failed.
set -u
here=${0:A:h}
repo=$(git -C "$here" rev-parse --show-toplevel) || exit 2
ref=${1:-$(git -C "$repo" merge-base HEAD main)} || exit 2
out=${2:-$repo/target/relift_identity}
rm -rf "$out/old" "$out/new"
mkdir -p "$out"
ref_gen="$here/_gen_ref_$$.py"
trap 'rm -f "$ref_gen"' EXIT
git -C "$repo" show "${ref}:tools/decomp_scale/gen_subsystems.py" > "$ref_gen" || exit 2
cd "$repo"
python3 "$ref_gen" --out-dir "$out/old" > "$out/old.log" 2>&1 || { echo "ref generator failed: $out/old.log"; exit 2; }
python3 "$here/gen_subsystems.py" --out-dir "$out/new" > "$out/new.log" 2>&1 || { echo "generator failed: $out/new.log"; exit 2; }
if diff -r "$out/old" "$out/new" > "$out/diff.txt"; then
    echo "identical to $ref ($(tail -1 "$out/new.log"))"
    exit 0
fi
echo "DIFFERS from $ref: $out/diff.txt ($(wc -l < "$out/diff.txt") lines)"
exit 1
