#!/usr/bin/env bash
# Package a self-contained Alan Wake 2 macOS application bundle (`Alan Wake 2.app`):
# - `Contents/MacOS/AlanWake2`: native Apple Silicon (arm64) AOT runtime by default
#   (all 244,795 AOT-compiled Rust functions, 38 native Win32/middleware builtins,
#   Metal 4 D3D12 backend, and CoreAudio). Set `AW2_APP_ARCH=x86_64` for the Rosetta
#   hybrid build.
# - `Contents/MacOS/dxil-metald`: native Apple Silicon (arm64) multi-process
#   DXIL -> Metal shader compiler worker daemon.
# - `Contents/Frameworks/libmetalirconverter.dylib` + `libdxilconv.dylib`
# - `Contents/Resources/Game`: full `AlanWake2.exe`, DLLs, and `data/` + `data_pack2/`
#   assets (cloned via APFS copy-on-write on local volumes, or copied across volumes;
#   set `AW2_SYMLINK_GAME=1` for a lightweight dev symlink).
# - `Contents/Resources/symbols`: `alanwake2.exe.syms` symbol table
# - `Contents/Resources/ShaderCache`: prewarmed DXIL -> Metal shader cache
#
# Usage:
#   ./scripts/package_app.sh [--copy-to /Volumes/MySSD] [--symlink-game]
set -euo pipefail

COPY_TO="${AW2_COPY_TO:-}"
SYMLINK_GAME="${AW2_SYMLINK_GAME:-0}"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --copy-to)
            COPY_TO="$2"
            shift 2
            ;;
        --symlink-game)
            SYMLINK_GAME=1
            shift
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 1
            ;;
    esac
done

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
APP_BUNDLE="${AW2_APP_OUT:-$ROOT_DIR/Alan Wake 2.app}"
APP_ARCH="${AW2_APP_ARCH:-arm64}"
CONTENTS_DIR="$APP_BUNDLE/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
FRAMEWORKS_DIR="$CONTENTS_DIR/Frameworks"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

cd "$ROOT_DIR"
if [[ "$APP_ARCH" == "x86_64" ]]; then
    echo "[1/6] Building native arm64 shader compiler daemon (dxil-metald, dxil-metal)..."
    cargo build --release -p dxil-metal --features tools --bin dxil-metald --bin dxil-metal
    echo "[2/6] Building release x86_64 hybrid runtime (aw2-loader)..."
    cargo build --release --target x86_64-apple-darwin -p port-loader --features aw2 --bin aw2-loader
    LOADER_BIN="$ROOT_DIR/target/x86_64-apple-darwin/release/aw2-loader"
    FIRST_ARCH="x86_64"
    SECOND_ARCH="arm64"
else
    echo "[1/6] Building native arm64 runtime and shader compiler daemon (aw2-loader, dxil-metald, dxil-metal)..."
    cargo build --release --target aarch64-apple-darwin -p port-loader --features aw2 --bin aw2-loader
    cargo build --release --target aarch64-apple-darwin -p dxil-metal --features tools --bin dxil-metald --bin dxil-metal
    echo "[2/6] Using release native arm64 runtime ($ROOT_DIR/target/aarch64-apple-darwin/release/aw2-loader)..."
    LOADER_BIN="$ROOT_DIR/target/aarch64-apple-darwin/release/aw2-loader"
    FIRST_ARCH="arm64"
    SECOND_ARCH="x86_64"
fi

echo "[3/6] Creating 'Alan Wake 2.app' bundle structure ($APP_ARCH)..."
rm -rf "$CONTENTS_DIR/_CodeSignature" "$MACOS_DIR" "$FRAMEWORKS_DIR"
mkdir -p "$MACOS_DIR" "$FRAMEWORKS_DIR" "$RESOURCES_DIR"

cat > "$CONTENTS_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>Alan Wake 2</string>
    <key>CFBundleExecutable</key>
    <string>AlanWake2</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.remedy.AlanWake2.macos</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Alan Wake 2</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.2.10</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.games</string>
    <key>LSMinimumSystemVersion</key>
    <string>14.0</string>
    <key>LSArchitecturePriority</key>
    <array>
        <string>${FIRST_ARCH}</string>
        <string>${SECOND_ARCH}</string>
    </array>
    <key>LSEnvironment</key>
    <dict>
        <key>ROSETTA_ADVERTISE_AVX</key>
        <string>1</string>
        <key>AW2_HOOK_SHARDS</key>
        <string>1</string>
    </dict>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF

echo "[4/6] Installing executables, boot-logging wrapper, symbols, and shader libraries..."
cp "$LOADER_BIN" "$MACOS_DIR/aw2-loader"
if [[ -f "$ROOT_DIR/target/aarch64-apple-darwin/release/dxil-metald" ]]; then
    cp "$ROOT_DIR/target/aarch64-apple-darwin/release/dxil-metald" "$MACOS_DIR/dxil-metald"
else
    cp "$ROOT_DIR/target/release/dxil-metald" "$MACOS_DIR/dxil-metald"
fi
if [[ -f "$ROOT_DIR/games/alan-wake-2/game.toml" ]]; then
    cp -p "$ROOT_DIR/games/alan-wake-2/game.toml" "$RESOURCES_DIR/game.toml"
fi

cat > "$MACOS_DIR/AlanWake2" << 'WRAPEOF'
#!/bin/sh
# Pre-dyld boot wrapper for Alan Wake 2.app.
# Creates ~/Library/Logs/AW2Port/AlanWake2.log (and /tmp/AlanWake2.log) before
# launching the Mach-O runtime so that dyld errors, OS version mismatches,
# quarantine blocks, and early crashes are always logged and surfaced.

LOG_DIR="${AW2_LOG_DIR:-$HOME/Library/Logs/AW2Port}"
mkdir -p "$LOG_DIR" 2>/dev/null || {
    LOG_DIR="/tmp/AW2Port"
    mkdir -p "$LOG_DIR" 2>/dev/null
}
LOG_FILE="$LOG_DIR/AlanWake2.log"
if [ -s "$LOG_FILE" ]; then
    cp -f "$LOG_FILE" "$LOG_DIR/AlanWake2.prev.log" 2>/dev/null || true
fi
: > "$LOG_FILE"
ln -sf "$LOG_FILE" /tmp/AlanWake2.log 2>/dev/null || true

MACOS_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTENTS_DIR="$(cd "$MACOS_DIR/.." && pwd)"
APP_DIR="$(cd "$CONTENTS_DIR/.." && pwd)"

{
    echo "================================================================================"
    echo "Alan Wake 2.app Wrapper Boot Log — $(date '+%Y-%m-%d %H:%M:%S %z')"
    echo "App Bundle : $APP_DIR"
    echo "User / Host: $(id -un 2>/dev/null) @ $(hostname 2>/dev/null)"
    echo "macOS      : $(sw_vers -productName 2>/dev/null) $(sw_vers -productVersion 2>/dev/null) ($(sw_vers -buildVersion 2>/dev/null))"
    echo "Hardware   : $(sysctl -n hw.model 2>/dev/null) / $(sysctl -n machdep.cpu.brand_string 2>/dev/null) ($(uname -m))"
    echo "Arguments  : $*"
    echo "================================================================================"
} >> "$LOG_FILE" 2>&1

# Strip Gatekeeper quarantine xattrs on executables/dylibs if copied from another Mac/drive
xattr -dr com.apple.quarantine "$MACOS_DIR" "$CONTENTS_DIR/Frameworks" 2>/dev/null || true

export PORT_MANIFEST="${PORT_MANIFEST:-$CONTENTS_DIR/Resources/game.toml}"
export PORT_SHADER_CACHE_FALLBACK="${PORT_SHADER_CACHE_FALLBACK:-$CONTENTS_DIR/Resources/ShaderCache}"
export PORT_SHADER_MANIFEST_FALLBACK="${PORT_SHADER_MANIFEST_FALLBACK:-$CONTENTS_DIR/Resources/ShaderCache/manifest}"
export PORT_PSO_MANIFEST_FALLBACK="${PORT_PSO_MANIFEST_FALLBACK:-$CONTENTS_DIR/Resources/ShaderCache/pso/pso_manifest}"
export PORT_SHADERD="${PORT_SHADERD:-$MACOS_DIR/dxil-metald}"
export AW2_SHADERD="${AW2_SHADERD:-$MACOS_DIR/dxil-metald}"
if [ -f "$CONTENTS_DIR/Frameworks/libmetalirconverter.dylib" ]; then
    export METAL_IRCONVERTER_DYLIB="${METAL_IRCONVERTER_DYLIB:-$CONTENTS_DIR/Frameworks/libmetalirconverter.dylib}"
fi
if [ -f "$CONTENTS_DIR/Frameworks/libdxilconv.dylib" ]; then
    export AW2_DXILCONV="${AW2_DXILCONV:-$CONTENTS_DIR/Frameworks/libdxilconv.dylib}"
fi
export AW2_LOG_APPEND=1
export ROSETTA_ADVERTISE_AVX="${ROSETTA_ADVERTISE_AVX:-1}"
export AW2_HOOK_SHARDS="${AW2_HOOK_SHARDS:-1}"

if [ -t 2 ]; then
    "$MACOS_DIR/aw2-loader" "$@"
    STATUS=$?
else
    "$MACOS_DIR/aw2-loader" "$@" >> "$LOG_FILE" 2>&1
    STATUS=$?
fi

if [ "$STATUS" -ne 0 ]; then
    echo "[wrapper] aw2-loader exited with status $STATUS at $(date '+%Y-%m-%d %H:%M:%S %z')" >> "$LOG_FILE"
    if [ "${AW2_HEADLESS:-0}" != "1" ]; then
        TAIL_MSG="$(tail -n 15 "$LOG_FILE" 2>/dev/null | tr '"' "'" | tr '\\' '/')"
        BUTTON="$(/usr/bin/osascript -e "button returned of (display alert \"Alan Wake 2 Failed to Boot (exit $STATUS)\" message \"Log saved to:\n$LOG_FILE\n\nRecent log output:\n$TAIL_MSG\" as critical buttons {\"Show Log\", \"OK\"} default button \"Show Log\")" 2>/dev/null || true)"
        if [ "$BUTTON" = "Show Log" ]; then
            open -a Console "$LOG_FILE" 2>/dev/null || open -e "$LOG_FILE" 2>/dev/null || true
        fi
    fi
fi
exit "$STATUS"
WRAPEOF

chmod +x "$MACOS_DIR/AlanWake2" "$MACOS_DIR/aw2-loader" "$MACOS_DIR/dxil-metald"

if [[ -d "$ROOT_DIR/out/symbols" ]]; then
    mkdir -p "$RESOURCES_DIR/symbols"
    cp -p "$ROOT_DIR/out/symbols/"*.syms "$RESOURCES_DIR/symbols/" 2>/dev/null || true
    echo "  Bundled symbol tables into Contents/Resources/symbols"
fi

for cand in \
    "${METAL_IRCONVERTER_DYLIB:-}" \
    "$HOME/.local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib" \
    "/usr/local/lib/libmetalirconverter.dylib" \
    "/opt/metal-shaderconverter/lib/libmetalirconverter.dylib" \
    "/Applications/Parallels Desktop.app/Contents/Frameworks/libmetalirconverter.dylib" \
    "/Library/Application Support/Apple/GamePortingToolkit/lib/libmetalirconverter.dylib"
do
    if [[ -n "$cand" && -f "$cand" ]]; then
        cp -p "$cand" "$FRAMEWORKS_DIR/libmetalirconverter.dylib"
        echo "  Bundled libmetalirconverter.dylib from $cand"
        break
    fi
done

for cand in \
    "${AW2_DXILCONV:-}" \
    "$HOME/.local/lib/libdxilconv.dylib" \
    "/opt/homebrew/lib/libdxilconv.dylib" \
    "/usr/local/lib/libdxilconv.dylib"
do
    if [[ -n "$cand" && -f "$cand" ]]; then
        cp -p "$cand" "$FRAMEWORKS_DIR/libdxilconv.dylib"
        chmod 755 "$FRAMEWORKS_DIR/libdxilconv.dylib"
        vtool -set-build-version macos 14.0 27.2 -replace -output "$FRAMEWORKS_DIR/libdxilconv.dylib" "$FRAMEWORKS_DIR/libdxilconv.dylib" 2>/dev/null || true
        echo "  Bundled libdxilconv.dylib from $cand (minos set to 14.0)"
        break
    fi
done

echo "[5/6] Bundling game assets, extracting icon, and bundling prewarmed Metal shaders..."
GAME_DIR="${PORT_GAME_DIR:-${AW2_GAME_DIR:-}}"
if [[ -n "$GAME_DIR" && -d "$GAME_DIR" ]]; then
    if [[ "$SYMLINK_GAME" == "1" ]]; then
        rm -rf "$RESOURCES_DIR/Game"
        ln -s "$GAME_DIR" "$RESOURCES_DIR/Game"
        echo "  Symlinked Game -> $GAME_DIR"
    elif [[ -f "$RESOURCES_DIR/Game/AlanWake2.exe" && ! -L "$RESOURCES_DIR/Game" ]]; then
        echo "  Contents/Resources/Game already populated (skipping re-clone)"
    else
        rm -rf "$RESOURCES_DIR/Game"
        echo "  Bundling $GAME_DIR into Contents/Resources/Game..."
        if cp -cR "$GAME_DIR" "$RESOURCES_DIR/Game" 2>/dev/null; then
            echo "  Cloned game assets via APFS copy-on-write into Contents/Resources/Game"
        else
            rm -rf "$RESOURCES_DIR/Game"
            cp -R "$GAME_DIR" "$RESOURCES_DIR/Game"
            echo "  Copied game assets into Contents/Resources/Game"
        fi
    fi
    if [[ ! -f "$RESOURCES_DIR/AppIcon.icns" ]]; then
        python3 - "$GAME_DIR" "$RESOURCES_DIR" << 'PYEOF'
import glob, os, subprocess, sys, tempfile

game_dir, res_dir = sys.argv[1], sys.argv[2]
exe_path = os.path.join(game_dir, "AlanWake2.exe")
if os.path.isfile(exe_path):
    with open(exe_path, "rb") as f:
        data = f.read()
    png_start = data.find(b"\x89PNG\r\n\x1a\n")
    if png_start != -1:
        iend = data.find(b"IEND", png_start)
        if iend != -1:
            png_bytes = data[png_start:iend + 8]
            with tempfile.TemporaryDirectory() as tmp:
                iconset = os.path.join(tmp, "AppIcon.iconset")
                os.makedirs(iconset, exist_ok=True)
                base_png = os.path.join(tmp, "base.png")
                with open(base_png, "wb") as pf:
                    pf.write(png_bytes)
                for sz in [16, 32, 128, 256, 512]:
                    subprocess.run(["sips", "-z", str(sz), str(sz), base_png, "--out", os.path.join(iconset, f"icon_{sz}x{sz}.png")], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                    subprocess.run(["sips", "-z", str(sz*2), str(sz*2), base_png, "--out", os.path.join(iconset, f"icon_{sz}x{sz}@2x.png")], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                subprocess.run(["iconutil", "-c", "icns", iconset, "-o", os.path.join(res_dir, "AppIcon.icns")], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
PYEOF
    fi
fi

SHADER_SRC="${PORT_SHADER_CACHE:-${AW2_SHADER_CACHE:-$HOME/Translator/ShaderCache}}"
if [[ ! -d "$SHADER_SRC" && -d "$HOME/Library/Caches/port-factory/shaders" ]]; then
    SHADER_SRC="$HOME/Library/Caches/port-factory/shaders"
fi

DXIL_METAL_BIN="$ROOT_DIR/target/aarch64-apple-darwin/release/dxil-metal"
[[ ! -x "$DXIL_METAL_BIN" ]] && DXIL_METAL_BIN="$ROOT_DIR/target/release/dxil-metal"
if [[ -x "$DXIL_METAL_BIN" && -f "$SHADER_SRC/manifest/records.aw2m" ]]; then
    "$DXIL_METAL_BIN" prewarm --manifest --backend helper || true
fi

if [[ -d "$SHADER_SRC" ]]; then
    mkdir -p "$RESOURCES_DIR/ShaderCache"
    for subdir in "$SHADER_SRC"/ir* "$SHADER_SRC/manifest"; do
        if [[ -d "$subdir" ]]; then
            bname="$(basename "$subdir")"
            rm -rf "$RESOURCES_DIR/ShaderCache/$bname"
            cp -cR "$subdir" "$RESOURCES_DIR/ShaderCache/$bname" 2>/dev/null || cp -R "$subdir" "$RESOURCES_DIR/ShaderCache/$bname"
        fi
    done
    # Bundle portable GPU/OS-independent PSO recipes (never device-specific *.mtl4a files)
    if [[ -d "$SHADER_SRC/pso/pso_manifest" ]]; then
        mkdir -p "$RESOURCES_DIR/ShaderCache/pso"
        rm -rf "$RESOURCES_DIR/ShaderCache/pso/pso_manifest"
        cp -cR "$SHADER_SRC/pso/pso_manifest" "$RESOURCES_DIR/ShaderCache/pso/pso_manifest" 2>/dev/null || cp -R "$SHADER_SRC/pso/pso_manifest" "$RESOURCES_DIR/ShaderCache/pso/pso_manifest"
    fi
    echo "  Bundled portable shader cache & manifests into Contents/Resources/ShaderCache"
fi

echo "[6/6] Codesigning bundle Mach-O binaries..."
for dylib in "$FRAMEWORKS_DIR"/*.dylib; do
    [[ -f "$dylib" ]] && codesign --force --sign - "$dylib"
done
codesign --force --sign - "$MACOS_DIR/dxil-metald"
codesign --force --sign - "$MACOS_DIR/aw2-loader"
codesign --force --sign - "$APP_BUNDLE"
echo "Built self-contained macOS app bundle: $APP_BUNDLE"

if [[ -n "$COPY_TO" ]]; then
    if [[ "$COPY_TO" == *.app ]]; then
        DEST_APP="$COPY_TO"
    else
        DEST_APP="${COPY_TO%/}/Alan Wake 2.app"
    fi
    echo "[Transfer] Syncing '$APP_BUNDLE' -> '$DEST_APP'..."
    mkdir -p "$DEST_APP/Contents/MacOS" "$DEST_APP/Contents/Frameworks" "$DEST_APP/Contents/Resources"
    rm -rf "$DEST_APP/Contents/_CodeSignature"
    if [[ -f "$DEST_APP/Contents/Resources/Game/AlanWake2.exe" && -d "$DEST_APP/Contents/Resources/ShaderCache" ]]; then
        echo "  Destination Game/ and ShaderCache/ already populated; syncing MacOS/, Frameworks/, Info.plist, game.toml, and symbols/..."
        COPYFILE_DISABLE=1 cp -f "$APP_BUNDLE/Contents/Info.plist" "$DEST_APP/Contents/Info.plist"
        [[ -f "$APP_BUNDLE/Contents/Resources/game.toml" ]] && COPYFILE_DISABLE=1 cp -f "$APP_BUNDLE/Contents/Resources/game.toml" "$DEST_APP/Contents/Resources/game.toml"
        [[ -f "$APP_BUNDLE/Contents/Resources/AppIcon.icns" ]] && COPYFILE_DISABLE=1 cp -f "$APP_BUNDLE/Contents/Resources/AppIcon.icns" "$DEST_APP/Contents/Resources/AppIcon.icns"
        COPYFILE_DISABLE=1 rsync -rlptDW --inplace --modify-window=2 "$APP_BUNDLE/Contents/MacOS/" "$DEST_APP/Contents/MacOS/"
        COPYFILE_DISABLE=1 rsync -rlptDW --inplace --modify-window=2 "$APP_BUNDLE/Contents/Frameworks/" "$DEST_APP/Contents/Frameworks/"
        if [[ -d "$APP_BUNDLE/Contents/Resources/symbols" ]]; then
            mkdir -p "$DEST_APP/Contents/Resources/symbols"
            COPYFILE_DISABLE=1 rsync -rlptDW --inplace --modify-window=2 "$APP_BUNDLE/Contents/Resources/symbols/" "$DEST_APP/Contents/Resources/symbols/"
        fi
        codesign --force --sign - "$DEST_APP"
    else
        COPYFILE_DISABLE=1 rsync -rlptDW --inplace --modify-window=2 "$APP_BUNDLE/" "$DEST_APP/"
    fi
    echo "Transferred complete bundle to: $DEST_APP"
fi
