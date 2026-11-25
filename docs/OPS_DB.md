# Factory CLI (`crates/port-factory`), Ops DB, Win32 DB & Virtual Filesystem

`crates/port-factory` is the analysis, auditing, precompilation, scaffolding, and packaging toolkit
for the port factory. Run it via `cargo opsdb -- <command>` or `./tools/opsdb.sh <command>`.

---

## 1. Architecture & Catalogs

`doctor` compares five generated TSVs, checks ops and Win32 source references and verifies the
published catalog counts. `doctor --fix` refreshes generated rows and the marked block in
STATUS. Correct broken source references in the generator.

| Component | Code & Data | What it does |
|---|---|---|
| **x86-64 → ARM64 Ops DB** | `crates/port-factory/src/ops_db/`, `db/ops_x86_to_arm64.tsv` | Groups mnemonics by implementation tier, with source references and vector counts. [Current totals and known gaps](STATUS.md#catalog-totals) |
| **A64 encoding forms and JIT emitter matches** | [A64 catalog](../db/a64_encodings.tsv), [emitter matches](../db/a64_emitter_matches.tsv) | Look up an Arm operation's encodings, operand fields and feature requirements, or find the form used by a JIT test word. [Verification and regeneration](DIAGNOSTICS.md#check-the-a64-words-emitted-by-the-jit) |
| **Win32 API DB** | `crates/port-factory/src/win32_db.rs`, `db/win32_apis.tsv` | Indexes exports by DLL, symbol, ordinal and implementation kind. Supports `api-ms-win-*` normalization and `kernel32 ↔ ntdll` forwarding. |
| **PE Coverage Scanner** | `crates/port-factory/src/scanner.rs`, `db/ops_profiles.tsv` | Disassembles any Windows x86-64 `.exe`/`.dll` and measures ARM64 JIT coverage, instruction mix, and Win32 import readiness. |
| **Offline `.aot` Prewarmer** | `crates/x86-runtime/src/emu/aot.rs`, `port-factory aot-prewarm` | Drives `port-runner --prewarm-aot` to compile PE `.pdata` + exports + entry + static CFG closures into relocatable ARM64 `.aot` block caches ahead of launch. |
| **Auditors** | `anticheat.rs`, `dotnet.rs`, `gpu_shaders.rs`, `game_profiles.rs` | Audits anti-cheat / direct syscalls / VEH traps, .NET CLR / ReadyToRun / Unity Mono / IL2CPP, desktop/GPU capabilities, embedded shaders, and known engine profiles (`db/game_profiles.tsv`, `db/shader_formats.tsv`). |
| **Steam Scanner & `.app` Bundler** | `crates/port-factory/src/steam_library.rs` | Scans macOS/CrossOver/Heroic Steam libraries, runs `installscript.vdf` post-install, and packages self-contained macOS `.app` bundles containing `port-runner`, `dxil-metald`, `game.toml`, and prewarmed `AotCache`. |
| **iOS Bundler & Storage Optimizer** | `crates/port-factory/src/{ios_bundle,storage_opt}.rs`, `crates/bink2-codec/` | Packages any Windows game into a signed ARM64 iOS `.app`/`.ipa` with static `.aot.o` code, trimmed runtime-only `ShaderCache/`, and opt-in storage optimization (`optimize-assets` / `ios-bundle --optimize-storage`: unused PC DLL stripping, Bink 2 → `KB2v` hardware `VideoToolbox` HEVC, lossless DDS mipmap capping, plus declarative `[storage].archive_repacker` delegation). |
| **Scaffolding & Prototype Rust AOT** | `scaffold.rs`, `aot/` | Generates starter code for missing x86 ops (`scaffold-op`) and Win32 DLL modules (`scaffold-dll`), plus an experimental C-leaf PE → Rust transpiler (`aot` / `aot-crate`) for small self-contained DLLs like `zlib1.dll`. |
| **Diagnostic & Verification Toolkit** | `diag_toolkit.rs`, `x86-verify`, `tools/debug/{x86_oracle,bisect_runtime,live_probe}.py` | Static runtime-hazard auditing (`runtime-audit`), offline 4-tier PE/DLL differential verification (`verify-x86 --oracle`), Rosetta 2 x86-64 snapshot replay (`oracle`), knob-matrix + VA bisection (`bisect-runtime`), frame-paced game automation (`live-probe`), and `PORT_NAN_TRACE` runner (`nan-trace`). See [docs/DIAGNOSTICS.md](DIAGNOSTICS.md). |
| **Virtual Windows VFS** | `crates/port-factory/src/vfs.rs` & `crates/win32-rt/src/path.rs` | Maps standard Windows paths and env vars onto `~/Translator`. |

---

## 2. Everyday Commands

### Auditing a new game or binary
```sh
# 1. Measure x86-64 -> ARM64 instruction coverage (add --record to log to db/ops_profiles.tsv)
cargo opsdb -- scan /path/to/Game/ --record

# 2. Check which Win32/CRT/middleware imports are implemented vs missing
cargo opsdb -- win32-audit /path/to/Game/

# 3. Check for anti-cheat drivers, direct syscalls, .NET runtimes, and embedded shaders
cargo opsdb -- anticheat-audit /path/to/Game/Game.exe
cargo opsdb -- dotnet-audit /path/to/Game/
cargo opsdb -- app-audit /path/to/Game/Game.exe
cargo opsdb -- game-profiles /path/to/Game/Game.exe
```

### Offline `.aot` block-cache precompilation
```sh
# Prewarm ARM64 block caches for a PE and its non-builtin DLLs
cargo opsdb -- aot-prewarm /path/to/Game/Game.exe
cargo opsdb -- aot-prewarm /path/to/Game/Game.exe --aot-dir /custom/cache --max-funcs 500

# Equivalent direct port-runner invocation
cargo runner -- --pe /path/to/Game/Game.exe --prewarm-aot
```
Default cache directory: `~/Library/Caches/port-factory/aot` (override with `PORT_AOT_DIR`; reads
legacy `~/Library/Caches/aw2-port/aot` as a fallback when no env override is set).

### Syncing & querying the Ops and Win32 databases
```sh
# Always run win32-sync after adding/removing/moving exports in crates/win32-rt
cargo opsdb -- win32-sync
cargo opsdb -- win32-stats --search VirtualAlloc

# Ops DB queries
cargo opsdb -- sync
cargo opsdb -- stats
cargo opsdb -- list --category SIMD_FP
cargo opsdb -- explain vfmadd213ps

# Verify all catalogs and VFS rules stay consistent
cargo opsdb -- doctor
```

### Diagnostic & verification toolkit (any Windows game / DLL)
```sh
# 1. Static runtime-hazard audit (ldmxcsr/FPCR, x87, AVX narrowing, approx-FP, lock-RMW, WaitOnAddress, RWX)
cargo opsdb -- runtime-audit /path/to/Game/

# 2. Offline 4-tier differential verifier (Generic vs FastOp vs JIT vs AOT + linked CFG + Rosetta 2 oracle)
cargo opsdb -- verify-x86 /path/to/Game/ --oracle

# 3. Frame-paced automated probe & screenshot/log-regex reproducer
cargo opsdb -- live-probe --manifest games/<id>/game.toml --game-dir /path/to/Game --script repro.txt --repeat 5

# 4. Execution-mode knob matrix & O(log N) PORT_AOT_SKIP / PORT_JIT_SKIP bisection
cargo opsdb -- bisect-runtime --cmd "python3 tools/debug/live_probe.py ..." --bisect-range "Game.exe:0x1000..0x2000000"

# 5. Live NaN-birth tracer + Rosetta 2 x86-64 hardware oracle replay (supports --mxcsr override)
cargo opsdb -- nan-trace games/<id>/game.toml --game-dir /path/to/Game --dump-dir out/snaps --run-oracle
cargo opsdb -- oracle out/snaps/ --mxcsr 0x9fc0
```
See [docs/DIAGNOSTICS.md](DIAGNOSTICS.md) for the full 5-stage debugging workflow and env-var reference.

### Scaffolding new ops and Win32 DLLs
```sh
# Find instructions in a PE that lack ARM64 JIT lowering and print ready-to-paste scaffolds
cargo opsdb -- scaffold-op --from-pe /path/to/Game.exe
cargo opsdb -- scaffold-op vgf2p8affineqb --category SIMD_FP_PACKED --arm64 "eor + tbl"

# Generate a new crates/win32-rt/src/misc/<stem>.rs module from a DLL's exports or an EXE's imports
cargo opsdb -- scaffold-dll my_middleware.dll /path/to/my_middleware.dll --write
cargo opsdb -- win32-sync
```

### Shaders & Steam `.app` packaging
```sh
# Inspect, convert, or precompile standalone DXIL / DXBC / SPIR-V / WGSL shaders
cargo opsdb -- shader-formats
cargo opsdb -- shader-inspect path/to/shader.spv
cargo opsdb -- shader-convert path/to/shader.spv --msl-out out.metal --out out.metallib
cargo opsdb -- precompile-shaders /path/to/Game/

### Interactive Launcher, Steam Management & `.app` Packaging
```sh
# Launch the interactive local web UI (download Steam games, convert to .app, verify, copy to SSD)
cargo opsdb -- launcher
cargo opsdb -- launcher-app                          # builds double-clickable out/apps/Port Factory.app

# CLI workflow: install native ARM64 DepotDownloader, download/repair Win64 Steam depot, port to .app
cargo opsdb -- steam-setup
cargo opsdb -- steam-scan
cargo opsdb -- steam-download 2357570 --qr --validate
cargo opsdb -- steam-port 2357570 --qr --copy-to /Volumes/MySSD

# Convert an existing game directory and verify/export the .app bundle
cargo opsdb -- steam-convert /path/to/Game --format app --copy-game --max-fns 0 --out out/apps/Game.app
cargo opsdb -- bundles
cargo opsdb -- bundle-verify out/apps/Overwatch.app
cargo opsdb -- bundle-export out/apps/Overwatch.app /Volumes/MySSD

# Package for iOS with optional storage optimization (lossless | high | compact)
cargo opsdb -- ios-bundle /path/to/Game --out out/ios/Game.ipa --target device --release \
  --bundle-shader-cache --optimize-storage high

# Or run standalone storage optimization (universal + optional [storage].archive_repacker)
cargo opsdb -- optimize-assets /path/to/Game --out /path/to/Game_Optimized \
  --manifest games/alan-wake-2/game.toml --profile compact --dry-run
```
```

### Prototype Static Rust AOT (small C-leaf DLLs only)
For large games, `x86-runtime`'s ARM64 block JIT + `.aot` machine-code cache is the primary engine.
For small self-contained C libraries (such as `zlib1.dll`), `port-factory` also includes an
experimental PE → Rust transpiler (`aot` / `aot-crate`). At runtime, `port-runner` only activates
compiled Rust AOT entries flagged `aw2_aot_entry_fully_lowered` so partially lowered functions fall
back safely to `x86-runtime`.

```sh
cargo opsdb -- aot /path/to/zlib1.dll --out out/zlib1_aot.rs
cargo opsdb -- aot-crate /path/to/zlib1.dll --out out/aot_zlib1
```

---

## 3. Standardized `~/Translator` Virtual Filesystem

All Windows user and system paths resolve under `~/Translator` by default (override with
`TRANSLATOR_HOME`, `AW2_DRIVE_C`, or `--drive-c <DIR>`). Add custom drive/folder mappings with
`--mount "D:\Mods=/path/on/mac"` or `PORT_MOUNTS`.

| Windows Path / Known Folder / Env Var | macOS Host Directory |
|---|---|
| `C:\users\<user>\Documents` (`FOLDERID_Documents`) | `~/Translator/Documents` |
| `C:\users\<user>\Desktop` (`FOLDERID_Desktop`) | `~/Translator/Desktop` |
| `C:\users\<user>\Downloads` (`FOLDERID_Downloads`) | `~/Translator/Downloads` |
| `C:\users\<user>\Saved Games` (`FOLDERID_SavedGames`) | `~/Translator/Saved Games` |
| `C:\users\<user>\Pictures` / `Music` / `Videos` | `~/Translator/{Pictures,Music,Videos}` |
| `C:\users\<user>\AppData\Roaming` (`%APPDATA%`) | `~/Translator/AppData/Roaming` |
| `C:\users\<user>\AppData\Local` (`%LOCALAPPDATA%`) | `~/Translator/AppData/Local` |
| `C:\users\<user>\AppData\LocalLow` | `~/Translator/AppData/LocalLow` |
| `C:\users\<user>\AppData\Local\Temp`, `C:\Windows\Temp` (`%TEMP%`) | `~/Translator/Temp` |
| `C:\ProgramData` (`%ProgramData%`) | `~/Translator/ProgramData` |
| `C:\Program Files`, `C:\Program Files (x86)` | `~/Translator/Program Files`, `~/Translator/Program Files (x86)` |
| `C:\users\Public` (`%PUBLIC%`) | `~/Translator/Public/{Documents,Desktop,Downloads,Pictures,Music,Videos}` |
| `C:\Windows` (`%SystemRoot%`) | `~/Translator/Windows/{System32,SysWOW64,Fonts,Temp}` |
| `C:\users\<user>` (`%USERPROFILE%`) | `~/Translator/Profile` |
| `C:\` (fallback root) | `~/Translator/drive_c` |
| `Z:\` | `/` |

Test path resolution anytime without launching a game:
```sh
cargo opsdb -- vfs-resolve "%USERPROFILE%\Documents\My Games\save.dat" --mount "D:\Mods=/tmp"
```
