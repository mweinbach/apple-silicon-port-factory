#!/usr/bin/env bash
# Rebuilds our own HLSL test fixtures (dxil-metal and d3d12-metal tests/fixtures/*.hlsl ->
# *.dxil). These are dev tooling only. The generated .dxil files are committed, so tests never need a compiler, and
# nothing at runtime depends on one. See tools/README.md.
#
# Compiler preference:
#   1. $DXC, or `dxc` on PATH: an open-source microsoft/DirectXShaderCompiler build (CLI).
#   2. $DXCOMPILER: a libdxcompiler.dylib you point at (an open-source build), driven by
#      tools/dxc_compile.cpp.
#   3. An open-source libdxcompiler.dylib in a common prefix (Homebrew, /usr/local, ~/.local).
#   4. Fallback, with a warning: the x86_64 libdxcompiler.dylib inside D3DMetal (GPTK /
#      CrossOver), run under Rosetta. This is acceptable for regenerating fixtures. The port
#      must never use D3DMetal components at runtime.
# Output is unsigned DXIL (no validator on macOS). The tests fix up the container checksum.
set -euo pipefail
cd "$(dirname "$0")/.."

# input  output  profile  entry  [extra dxc args...]
FIXTURES=(
    "tests/fixtures/static_samplers.hlsl tests/fixtures/static_samplers.cs.dxil cs_6_0 main"
    # Validator 1.0 output: PSV0 without the entry name, so the entry comes from the bitcode.
    "tests/fixtures/entry_v2.hlsl tests/fixtures/entry_v2.vs.dxil vs_6_0 mainVS -validator-version 1.0"
    # SM 6.6 Hull shaders with PSV0 v2 (validator 1.6): entry point lives in !dx.entryPoints
    # metadata alongside the patch-constant function.
    "tests/fixtures/entry_hull.hlsl tests/fixtures/entry_hull_v2.hs.dxil hs_6_6 HS_Custom -validator-version 1.6"
    "tests/fixtures/entry_hull.hlsl tests/fixtures/entry_hull_passthrough.hs.dxil hs_6_6 HS_PassThrough -validator-version 1.6"
    "../d3d12-metal/tests/fixtures/read_texture.hlsl ../d3d12-metal/tests/fixtures/read_texture.vs.dxil vs_6_0 main"
    # d3d12-metal mesh-shader GPU test (tests/gpu_mesh.rs).
    "../d3d12-metal/tests/fixtures/mesh.hlsl ../d3d12-metal/tests/fixtures/mesh.as.dxil as_6_5 mainAS"
    "../d3d12-metal/tests/fixtures/mesh.hlsl ../d3d12-metal/tests/fixtures/mesh.ms.dxil ms_6_5 mainMS"
    "../d3d12-metal/tests/fixtures/mesh.hlsl ../d3d12-metal/tests/fixtures/mesh_direct.ms.dxil ms_6_5 mainMSDirect"
    "../d3d12-metal/tests/fixtures/mesh.hlsl ../d3d12-metal/tests/fixtures/mesh.ps.dxil ps_6_5 mainPS"
    # d3d12-metal inline ray query GPU test (tests/gpu_rayquery.rs).
    "../d3d12-metal/tests/fixtures/rayquery.hlsl ../d3d12-metal/tests/fixtures/rayquery.cs.dxil cs_6_5 main"
)

DXC_BIN="${DXC:-$(command -v dxc || true)}"
if [[ -n "$DXC_BIN" ]]; then
    echo "using DXC CLI: $DXC_BIN"
    for f in "${FIXTURES[@]}"; do
        read -r in out profile entry extra <<<"$f"
        # shellcheck disable=SC2086 # extra args are word-split on purpose
        "$DXC_BIN" -T "$profile" -E "$entry" ${extra:-} -Fo "$out" "$in"
        echo "$out: $(wc -c <"$out" | tr -d ' ') bytes"
    done
    exit 0
fi

lib="${DXCOMPILER:-}"
if [[ -z "$lib" ]]; then
    for cand in /opt/homebrew/lib/libdxcompiler.dylib /usr/local/lib/libdxcompiler.dylib \
        "$HOME"/.local/lib/libdxcompiler.dylib "$HOME"/.local/opt/*/lib/libdxcompiler.dylib; do
        [[ -f "$cand" ]] && { lib="$cand"; break; }
    done
fi
if [[ -z "$lib" ]]; then
    lib="$(find /Applications "$HOME/Library/Application Support/heroic" -path '*D3DMetal.framework*' \
        -name libdxcompiler.dylib 2>/dev/null | head -1 || true)"
    [[ -n "$lib" ]] || { echo "no DXC found: install/build microsoft/DirectXShaderCompiler (see tools/README.md)" >&2; exit 1; }
    echo "WARNING: no open-source DXC found; falling back to D3DMetal's libdxcompiler.dylib (dev tooling only):" >&2
    echo "WARNING:   $lib" >&2
fi

# Build the driver for an architecture the dylib has; arm64 is preferred, x86_64 runs under Rosetta.
archs="$(lipo -archs "$lib" 2>/dev/null || echo x86_64)"
arch=x86_64
[[ " $archs " == *" arm64 "* ]] && arch=arm64
driver="${TMPDIR:-/tmp}/aw2_dxc_compile_$arch"
echo "using $lib ($arch)"
clang++ -std=c++17 -arch "$arch" -O1 -o "$driver" tools/dxc_compile.cpp
for f in "${FIXTURES[@]}"; do
    read -r in out profile entry extra <<<"$f"
    # shellcheck disable=SC2086
    DXCOMPILER="$lib" "$driver" "$in" "$out" "$profile" "$entry" ${extra:-}
done
