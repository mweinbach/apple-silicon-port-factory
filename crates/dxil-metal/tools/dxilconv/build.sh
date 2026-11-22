#!/usr/bin/env bash
# Builds libdxilconv.dylib (Microsoft's open-source DXBC -> DXIL converter from DXC's
# projects/dxilconv, NCSA license) for macOS, so dxil-metal can convert SM4/SM5 (FXC)
# shaders: the Metal Shader Converter reads DXIL only.
#
# Upstream builds dxilconv on Windows only; dxilconv-macos.patch (next to this script) makes
# it portable without changing the converter itself.
#
#   crates/dxil-metal/tools/dxilconv/build.sh            # -> ~/.local/lib/libdxilconv.dylib
#   PREFIX=/opt/aw2 SRC=/tmp/dxc build.sh
#
# Takes a few minutes on first build (LLVM support libraries), seconds afterwards.
set -euo pipefail

DXC_REPO=${DXC_REPO:-https://github.com/microsoft/DirectXShaderCompiler.git}
# Pinned: the patch is made against this commit.
DXC_COMMIT=${DXC_COMMIT:-aa001f3f5445ac062e59f209b6f1f8652b8474f3}
SRC=${SRC:-$HOME/.local/src/DirectXShaderCompiler}
PREFIX=${PREFIX:-$HOME/.local}
ARCH=${ARCH:-arm64}
HERE=$(cd "$(dirname "$0")" && pwd)
PATCH=$HERE/dxilconv-macos.patch

for tool in git cmake ninja; do
    command -v "$tool" >/dev/null || { echo "error: $tool not found (brew install cmake ninja)" >&2; exit 1; }
done

if [ ! -d "$SRC/.git" ]; then
    echo "== fetching DXC $DXC_COMMIT into $SRC"
    mkdir -p "$SRC"
    git -C "$SRC" init -q
    git -C "$SRC" remote add origin "$DXC_REPO"
    git -C "$SRC" fetch -q --depth 1 origin "$DXC_COMMIT"
    git -C "$SRC" checkout -q FETCH_HEAD
    git -C "$SRC" submodule update -q --init --recursive --depth 1
fi

if [ "$(git -C "$SRC" rev-parse HEAD)" != "$DXC_COMMIT" ]; then
    echo "warning: $SRC is at $(git -C "$SRC" rev-parse --short HEAD), patch targets ${DXC_COMMIT:0:8}" >&2
fi

# Apply the patch unless it already is (re-runs).
if git -C "$SRC" apply --reverse --check "$PATCH" 2>/dev/null; then
    echo "== patch already applied"
else
    echo "== applying $(basename "$PATCH")"
    git -C "$SRC" apply "$PATCH"
fi

echo "== configuring ($ARCH)"
cmake -S "$SRC" -B "$SRC/build" -G Ninja \
    -C "$SRC/cmake/caches/PredefinedParams.cmake" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_OSX_ARCHITECTURES="$ARCH" \
    -DCMAKE_INSTALL_PREFIX="$PREFIX" \
    -DHLSL_BUILD_DXILCONV=ON \
    -DENABLE_SPIRV_CODEGEN=OFF -DSPIRV_BUILD_TESTS=OFF \
    -DHLSL_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_TESTS=OFF -DCLANG_INCLUDE_TESTS=OFF \
    -DLLVM_ENABLE_ASSERTIONS=OFF -DHLSL_OFFICIAL_BUILD=OFF >/dev/null

echo "== building dxilconv"
ninja -C "$SRC/build" dxilconv

mkdir -p "$PREFIX/lib"
cp "$SRC/build/lib/libdxilconv.dylib" "$PREFIX/lib/libdxilconv.dylib"
echo "== installed $PREFIX/lib/libdxilconv.dylib"
echo "   (dxil-metal also finds it via \$AW2_DXILCONV or next to the executable)"
