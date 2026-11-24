# port-loader (`port-runner` & `aw2-loader`)

`crates/port-loader` is the executable host for the port factory. It loads Windows PE32+ binaries,
wires up `win32-rt` and `x86-runtime`, applies a declarative `game.toml` manifest if present, and
starts the guest on Apple Silicon (`aarch64-apple-darwin`, with optional legacy Rosetta 2
`x86_64-apple-darwin` support for differential testing).

## Two binaries, one core

| Binary | Cargo feature | Debug size | When to use it |
|---|---|---:|---|
| **`port-runner`** *(default)* | *(none)* | ~17 MB | **Use this for everything except Alan Wake 2.** Runs any Windows `.exe`/`.dll` or `games/<id>/game.toml` via `x86-runtime` (ARM64/NEON block JIT + `.aot` cache), `win32-rt` (D3D12, D3D11, Vulkan→MoltenVK, Win32/CRT/WASAPI/AppKit), `d3d12-metal` (Metal 4 + MetalFX), and `dxil-metal`. Builds in ~15 seconds; contains zero AW2 code or data. |
| **`aw2-loader`** | `--features aw2` | full | Alan Wake 2 only. Links `games/alan-wake-2/crates/aw2-game` (hand-decompiled `ns/` + lifted shards), `win32-rt/wwise-bridge`, AW2 Cohtml/PhysX/AutoPlay hooks, and the `db/throws.tsv` hook linter. |

## Common commands

```sh
# Build the fast universal runner
cargo runner-build

# Prewarm ARM64 .aot blocks offline and run a declarative game target
cargo runner -- --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch --prewarm-aot
cargo runner -- --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch

# Inspect or call exports on any standalone Windows PE
cargo runner -- --pe /path/to/app.exe --ops-report --no-run
cargo runner -- --pe /path/to/zlib1.dll --list-exports
cargo runner -- --pe /path/to/zlib1.dll --call-export zlibVersion

# Alan Wake 2 target (supply your own legally owned install)
cargo loader -- --game-dir /path/to/AlanWake2
cargo loader -- --game-dir /path/to/AlanWake2 --diff-test
```

## `game.toml` manifests

A `game.toml` lets you configure a Windows game without writing Rust. `port-runner` discovers it
from `--manifest`, `$PORT_MANIFEST`, or `../Resources/game.toml` inside a macOS `.app` bundle.

```toml
[app]
id = "overwatch"
name = "Overwatch"
exe = "Overwatch.exe"
win_dir = "C:\\Games\\Overwatch"
log_name = "Overwatch.log"
computer_name = "WIN64PORT"
steam_app_id = "2357570"

[runtime]
# Optional comma-separated builtin DLL override settings or AOT dylibs
# builtin_overrides = "0"
preferred_base_retry = false
v8_jitless = false
legacy_remedy_migration = false
```

## CLI options

| Option | What it does |
|---|---|
| `--manifest FILE` | Load a `game.toml` target manifest. |
| `--game-dir DIR` | Game install directory (defaults to `../Resources/Game`, `$PORT_GAME_DIR`, or `$AW2_GAME_DIR`). |
| `--exe NAME` | Executable or DLL filename inside `--game-dir`. |
| `--pe PATH` | Direct path to any Windows x86-64 `.exe` or `.dll` (implies its parent as game dir). |
| `--prewarm-aot` | Headless offline `.aot` precompilation: maps the PE + non-builtin imports, translates `.pdata` + export + entry + static CFG closure to ARM64 blocks, writes `.aot` files, prints a summary, and exits `0` without running `DllMain` or `WinMain`. |
| `--prewarm-shaders` | Precompiles recorded Tier-1 shaders (`.metallib`) and Tier-2 Metal 4 PSOs (`MTL4Archive`) in parallel for the current GPU + macOS build and exits `0`. |
| `--aot-max-funcs N` | Optional per-module root cap for `--prewarm-aot` (`0` = all roots). |
| `--aot-dir DIR` | Override `.aot` cache directory (default `PORT_AOT_DIR` or `~/Library/Caches/port-factory/aot`). |
| `--drive-c DIR` | Override the virtual Windows `C:\` root (default `$TRANSLATOR_HOME` or `~/Translator`). |
| `--mount WIN=HOST` | Mount a Windows path to a macOS directory (repeatable, or `$PORT_MOUNTS`). |
| `--ops-report` | Disassemble the PE against `port-factory`'s x86-64 → ARM64 ops catalog and print coverage. |
| `--list-exports` | Print the target PE's exported symbols and exit. |
| `--call-export SYM` | Initialize the target PE and call export `SYM` (or `#ORD`) with arguments passed after `--`. |
| `--no-builtin-overrides` | Load shipping PE DLLs from `--game-dir` instead of builtin overrides (`PORT_BUILTIN_OVERRIDES=0`). |
| `--no-aw2-hooks` | Disable Alan Wake 2 VA hooks (already off automatically for non-AW2 binaries). |
| `--trace` | Log every resolved import call (`RUST_LOG=trace=info`). |
| `--no-run` | Map the PE, resolve imports, print any unresolved imports per DLL, and exit. |
| `--fail-on-unimpl-op` | Abort with documented exit code `86` (`EXIT_UNIMPL_OP`) and a crash report if the generic interpreter hits an unimplemented x86-64 instruction (`PORT_FAIL_ON_UNIMPL_OP=1`). Default **on** for CLI bring-up runs (outside a packaged `.app` bundle) and `live-probe`. |
| `--no-fail-on-unimpl-op` | Log unimplemented x86-64 instructions once per IP and continue as a no-op (`PORT_FAIL_ON_UNIMPL_OP=0`). |
| `--diff-test` | (`aw2` feature) Run differential tests comparing original x86-64 code against Rust replacements. |
| `--audio-harness` | (`aw2` feature) Replace `WinMain` with the Wwise audio oracle harness. |

## Environment variables

New flags use the `PORT_*` prefix. Legacy `AW2_*` names still work as fallbacks where older scripts
use them.

| Variable | Fallback | Purpose |
|---|---|---|
| `PORT_MANIFEST` | — | Default `game.toml` path |
| `PORT_GAME_DIR` | `AW2_GAME_DIR` | Default game install directory |
| `PORT_AOT_DIR` | `AW2_DLL_AOT_DIR` | Writable `.aot` block-cache directory (default `~/Library/Caches/port-factory/aot`) |
| `PORT_AOT_FALLBACK_DIR` | — | Read-only seed `.aot` block-cache directory (e.g. bundled `Contents/Resources/AotCache`) |
| `PORT_HEADLESS` | `AW2_HEADLESS` | `1` disables AppKit and uses a synthetic 1920×1080 display |
| `PORT_METALFX` | `AW2_METALFX` | `0` disables `MTL4FXSpatialScaler` and forces bilinear compute fallback |
| `PORT_D3D11` | — | `0` disables `d3d11.dll` (`DXGI_ERROR_UNSUPPORTED`) |
| `PORT_BUILTIN_OVERRIDES` | `AW2_BUILTIN_OVERRIDES` | `0` disables builtin overrides for game-shipped DLLs |
| `PORT_FAIL_ON_UNIMPL_OP` | — | `1` aborts with exit code `86` on unimplemented x86-64 instructions (`0` continues as no-op; defaults on for CLI bring-up runs and `live-probe`) |
| `PORT_SHADER_CACHE` | `AW2_SHADER_CACHE` | Writable on-disk `.metallib` shader cache directory (default `~/Translator/ShaderCache`) |
| `PORT_SHADER_CACHE_FALLBACK` | — | Read-only seed `.metallib` cache directory (e.g. bundled `Contents/Resources/ShaderCache`) |
| `PORT_PSO_CACHE` | `AW2_PSO_CACHE` | Writable Metal 4 `MTL4Archive` PSO cache root (default `<shader cache>/pso`) |
| `PORT_PSO_MANIFEST` | `AW2_PSO_MANIFEST` | Writable GPU/OS-independent PSO recipe manifest dir (default `<pso cache>/pso_manifest`; `0` disables) |
| `PORT_PSO_MANIFEST_FALLBACK` | — | Read-only bundled PSO recipe manifest dir for new-device cold prewarming |
| `PORT_PREWARM_SHADERS` | — | Cold-device launch prewarm policy: `auto` (default, prewarms only when current GPU+OS has 0 archives), `force`/`1`, or `0`/`off` |
| `PORT_SHADERD` | `AW2_SHADERD` | Path to the out-of-process `dxil-metald` worker |
| `PORT_SYMBOLS` | `AW2_SYMBOLS` | Symbol file/dir for crash backtraces (`0` disables) |
| `TRANSLATOR_HOME` | `AW2_DRIVE_C` | Virtual Windows `C:\` root (default `~/Translator`) |
