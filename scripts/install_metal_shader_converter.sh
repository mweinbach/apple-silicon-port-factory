#!/usr/bin/env zsh
# Install Apple's Metal Shader Converter from the official .pkg WITHOUT sudo:
# unpacks the (Apple-signed) payload into ~/.local/opt/metal-shaderconverter-<ver>
# and points ~/.local/opt/metal-shaderconverter at it. aw2-shaders searches there first.
#   usage: scripts/install_metal_shader_converter.sh /path/to/Metal_Shader_Converter_X.pkg
set -euo pipefail
PKG="${1:?usage: $0 <Metal_Shader_Converter.pkg>}"
pkgutil --check-signature "$PKG" | grep -q "signed Apple Software" || { echo "not an Apple-signed pkg"; exit 1; }
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
pkgutil --expand-full "$PKG" "$TMP/x" >/dev/null
PAY=$(find "$TMP/x" -type d -name Payload | head -1)
VER=$("$PAY/usr/local/bin/metal-shaderconverter" --version 2>/dev/null | awk '/version/{print $NF}')
DEST="$HOME/.local/opt/metal-shaderconverter-${VER:-unknown}"
rm -rf "$DEST"; mkdir -p "$DEST"
cp -R "$PAY/usr/local/"{bin,include,lib,share} "$DEST/"
ln -sfn "$DEST" "$HOME/.local/opt/metal-shaderconverter"
echo "installed Metal Shader Converter ${VER} -> $DEST"
