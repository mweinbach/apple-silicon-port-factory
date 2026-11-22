#!/usr/bin/env zsh
# One-time / idempotent bootstrap for the Apple Silicon Port Factory workspace.
set -uo pipefail
cd "${0:A:h}/.."

ok()   { print -P "%F{green}✓%f $*"; }
warn() { print -P "%F{yellow}!%f $*"; }
fail() { print -P "%F{red}✗%f $*"; }

# 1. Rust toolchain + native arm64 target (x86_64 optional for legacy Rosetta difftests)
if command -v rustup >/dev/null; then
  rustup target add aarch64-apple-darwin >/dev/null 2>&1 \
    && ok "rust target: aarch64-apple-darwin" \
    || fail "rustup target add aarch64-apple-darwin failed"
else
  fail "rustup not found (https://rustup.rs)"
fi

# 2. Git LFS check for db/*.tsv
if command -v git-lfs >/dev/null 2>&1; then
  if head -n 1 db/win32_apis.tsv 2>/dev/null | grep -q '^version https://git-lfs.github.com/spec/v1'; then
    warn "db/*.tsv are Git LFS pointers — run: git lfs pull"
  else
    ok "Git LFS data files materialized"
  fi
else
  warn "git-lfs not found (install with: brew install git-lfs && git lfs install && git lfs pull)"
fi

# 3. Optional third_party references (no git submodules; fetched via plain git clone)
if [[ -f third_party/vgmstream/src/coding/libs/vorbis_codebooks_wwise.h ]]; then
  ok "third_party/vgmstream (optional Wwise Vorbis codebooks)"
else
  warn "third_party/vgmstream not fetched (optional; run ./scripts/third_party.sh for Wwise Vorbis decoding)"
fi

# 4. Metal Shader Converter
MSC=""
for c in "${METAL_IRCONVERTER_DYLIB:-}" "$HOME/.local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib" \
         /opt/homebrew/lib/libmetalirconverter.dylib \
         /usr/local/lib/libmetalirconverter.dylib \
         /opt/metal-shaderconverter/lib/libmetalirconverter.dylib; do
  [[ -n "$c" && -f "$c" ]] && { MSC="$c"; break; }
done
if [[ -n "$MSC" ]]; then
  ok "Metal Shader Converter: $MSC"
else
  fail "Metal Shader Converter not found: scripts/install_metal_shader_converter.sh <pkg>"
fi

# 5. Optional user-supplied game install check
GAME_DIR="${PORT_GAME_DIR:-${AW2_GAME_DIR:-}}"
if [[ -n "$GAME_DIR" && -d "$GAME_DIR" ]]; then
  ok "Configured game directory: $GAME_DIR"
else
  warn "No PORT_GAME_DIR / AW2_GAME_DIR set (supply your own legally owned Windows game via --game-dir or PORT_GAME_DIR)"
fi

# 6. Optional RE tooling (Python + Ghidra)
command -v python3 >/dev/null && ok "python3 $(python3 -c 'import sys;print(sys.version.split()[0])')"
if command -v ghidraRun >/dev/null || [[ -d /opt/homebrew/opt/ghidra ]]; then
  ok "Ghidra"
else
  warn "Ghidra missing (optional, only for bespoke decompilation): brew install ghidra"
fi

# 7. Universal port-runner build check
cargo runner-build -q 2>/dev/null && ok "cargo runner-build (universal native ARM64 port-runner)" || fail "cargo runner-build failed"
