#!/usr/bin/env zsh
# Fetches optional pinned upstream reference repositories into ./third_party/
# using plain shallow `git clone` (no git submodules; ./third_party/ is gitignored).
#
# Usage:
#   ./scripts/third_party.sh                 # minimal set: directx-headers + vgmstream (~35 MB)
#   ./scripts/third_party.sh --minimal       # same as default
#   ./scripts/third_party.sh --all           # all 14 reference repos (~2.7 GB, includes PhysX/NvCloth/FidelityFX)
#   ./scripts/third_party.sh <name> [...]    # fetch specific repos by name (e.g. vgmstream wwiser)
set -euo pipefail
cd "${0:A:h}/.."
export GIT_LFS_SKIP_SMUDGE=1

# name | url | ref-kind (branch|tag) | ref
ALL_DEPS=(
  "directx-headers|https://github.com/microsoft/DirectX-Headers.git|branch|main"
  "vgmstream|https://github.com/vgmstream/vgmstream.git|branch|master"
  "wwiser|https://github.com/bnnm/wwiser.git|branch|master"
  "faudio|https://github.com/FNA-XNA/FAudio.git|branch|master"
  "mimalloc|https://github.com/microsoft/mimalloc.git|tag|v2.1.2"
  "luau|https://github.com/luau-lang/luau.git|branch|master"
  "recastnavigation|https://github.com/recastnavigation/recastnavigation.git|branch|main"
  "nv_cluster_builder|https://github.com/nvpro-samples/nv_cluster_builder.git|branch|main"
  "nri|https://github.com/NVIDIA-RTX/NRI.git|tag|v1.99"
  "nrd|https://github.com/NVIDIA-RTX/NRD.git|tag|v4.3.6"
  "streamline|https://github.com/NVIDIA-RTX/Streamline.git|tag|v2.7.2"
  "fidelityfx-sdk|https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK.git|branch|main"
  "nvcloth|https://github.com/NVIDIAGameWorks/NvCloth.git|branch|1.1.6"
  "physx-3.4|https://github.com/NVIDIAGameWorks/PhysX-3.4.git|branch|master"
)

MINIMAL_NAMES=(directx-headers vgmstream)

typeset -a SELECTED
if [[ $# -eq 0 || "$1" == "--minimal" ]]; then
  SELECTED=("${MINIMAL_NAMES[@]}")
elif [[ "$1" == "--all" ]]; then
  for entry in "${ALL_DEPS[@]}"; do
    IFS='|' read -r name _ _ _ <<< "$entry"
    SELECTED+=("$name")
  done
elif [[ "$1" == "-h" || "$1" == "--help" ]]; then
  sed -n '2,9p' "$0"
  exit 0
else
  SELECTED=("$@")
fi

mkdir -p third_party

for wanted in "${SELECTED[@]}"; do
  found=""
  for entry in "${ALL_DEPS[@]}"; do
    IFS='|' read -r name url kind ref <<< "$entry"
    if [[ "$name" == "$wanted" ]]; then
      found="$entry"
      break
    fi
  done
  if [[ -z "$found" ]]; then
    echo "Unknown third_party entry: $wanted" >&2
    exit 1
  fi
  IFS='|' read -r name url kind ref <<< "$found"
  dest="third_party/$name"
  if [[ -d "$dest" && -n "$(ls -A "$dest" 2>/dev/null)" ]]; then
    echo "== $name: already present at $dest"
    continue
  fi
  echo "== cloning $name ($kind $ref) -> $dest"
  rm -rf "$dest"
  git clone --depth 1 --branch "$ref" "$url" "$dest"
done

echo "third_party ready."
