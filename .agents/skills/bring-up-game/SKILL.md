---
name: bring-up-game
description: Bring up a new Windows x86-64 game on the universal port-runner using offline PE audits, a declarative games/<id>/game.toml manifest, ARM64 .aot prewarming, and iterative blocker triage with zero game-specific Rust code in crates/. Use when porting or adding a new Windows game, auditing a game install, or triaging why a title fails to boot or render.
---

# Bring Up a New Windows Game (`port-runner` + `game.toml`)

Universal rule: **normal games require zero custom Rust code in `crates/`.** Configure per-game
metadata and override policies declaratively in `games/<id>/game.toml`, and fix genuinely missing
Win32, x86-64, shader, or graphics behavior in the universal crates so every game benefits.

**Hard policy:** Never commit copyrighted game binaries/assets or personal machine paths, and never
bypass kernel anti-cheat or DRM. Flag anti-cheat/DRM honestly.

## Step 1 — Audit the Game Install Offline

Run the full factory audit battery on the user's legally owned Windows game directory before
booting:

```sh
cargo opsdb -- scan /path/to/Game --record
cargo opsdb -- win32-audit /path/to/Game
cargo opsdb -- anticheat-audit /path/to/Game/Game.exe
cargo opsdb -- dotnet-audit /path/to/Game
cargo opsdb -- app-audit /path/to/Game/Game.exe
cargo opsdb -- game-profiles /path/to/Game/Game.exe
cargo opsdb -- runtime-audit /path/to/Game
```

What to look for:
- **Anti-cheat / DRM (`anticheat-audit`):** Kernel drivers (Vanguard, kernel EAC/BattlEye,
  Ricochet) or anti-tamper watchdogs must be reported honestly as blockers/caveats.
- **Missing Win32/CRT/middleware exports (`win32-audit`):** Note unresolved DLLs/exports needed at
  startup. Prefer letting real shipping compression/physics DLLs (`oo2core_*`, `zstd`, `lz4`,
  `PhysX3*`) run on `x86-runtime` rather than stubbing them.
- **Graphics API (`app-audit`):** Identify D3D12, D3D11 (needs `libdxilconv.dylib` for SM 4/5), or
  Vulkan (needs `libMoltenVK.dylib`).
- **Runtime hazards (`runtime-audit`):** Note `ldmxcsr`/FTZ/DAZ, x87, AVX narrowing, RWX/JIT code
  (V8/Cohtml → set `v8_jitless = true`; .NET ReadyToRun → inspect with `dotnet-audit`).
- **Steam installs:** If the game came from Steam, run `cargo opsdb -- steam-postinstall /path/to/Game`
  to extract `installscript.vdf` registry keys and check `_CommonRedist`.

## Step 2 — Create `games/<id>/game.toml`

Copy `games/_template/game.toml` to `games/<id>/game.toml` (also see `games/overwatch/game.toml`
and `games/alan-wake-2/game.toml` for real examples).

All keys parsed by `crates/port-loader/src/target.rs` and `crates/port-factory/src/storage_opt.rs`:

```toml
[app]
id = "my-game"
name = "My Game"
exe = "MyGame.exe"
win_dir = "C:\\Games\\MyGame"
log_name = "MyGame.log"
computer_name = "WIN64PORT"
# steam_app_id = "123456"

[runtime]
preferred_base_retry = false
v8_jitless = false
legacy_remedy_migration = false
# Prefix with `-` to force the game's shipping PE DLL via x86-runtime instead of a builtin:
# builtin_overrides = "-physx3_x64.dll,-bink2w64.dll"
# Or "0" to disable all builtin overrides for game-shipped DLLs.
# args = "-windowed"

# Optional: for `optimize-assets` / `ios-bundle --optimize-storage`
# [storage]
# keep_dlls = ["custom_middleware.dll"]
# archive_repacker = "..."
# archive_dir = "..."

# Optional: first-launch config files copied into drive_c on iOS if absent
# [ios]
# seed_file = "ios-config.ini -> AppData/Local/MyGame/settings.ini"
```

## Step 3 — Dry-Run Imports & Prewarm ARM64 `.aot` Cache

Build the thin universal runner (~15 s, zero AW2 shards) and verify load-time imports + offline AOT:

```sh
cargo runner-build
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --ops-report --no-run
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --prewarm-aot
```

- Override AOT thread count with `PORT_AOT_THREADS=<N>` or cache path with `--aot-dir <DIR>` /
  `PORT_AOT_DIR`.
- Check virtual path resolution without booting via `cargo opsdb -- vfs-resolve "<WIN_PATH>"`.

## Step 4 — Boot & Triage Blockers One at a Time

Start headless first for fast iteration, then windowed:

```sh
PORT_HEADLESS=1 cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
# Windowed interactive run:
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
```

Add `--trace` to log every resolved import call (`RUST_LOG=trace=info`). Check crash reports in
`out/crash/crash-<pid>-<time>.txt` and hitch/stall logs in `out/hitch/hitch-<pid>.log`.

Route each blocker to the right skill and crate:

| Symptom | Next Skill & Action |
|---|---|
| `UNIMPLEMENTED foo.dll!Bar` or broken Win32/COM/CRT call | Use `implement-win32-api` (`crates/win32-rt` + `cargo opsdb -- win32-sync`) |
| Unhandled or hot `GENERIC_INTERP` x86 opcode | Use `implement-x86-op` (`crates/x86-runtime` + `cargo opsdb -- sync`) |
| Wrong math, NaN physics/geometry, AOT/JIT crash, hang | Use `debug-runtime-and-nan` (`verify-x86`, `live-probe`, `bisect-runtime`, `nan-trace`) |
| Shader conversion / PSO compile error or stutter | Use `shader-pipeline` (`crates/dxil-metal`, `dxil-metald`, `pso_manifest`) |
| Metal validation error (`MTL_DEBUG_LAYER=1`), black frame, D3D11/D3D12/Vulkan bug | Use `graphics-metal4` (`crates/d3d12-metal`, `win32-rt/src/{d3d11,d3d12,misc/vulkan_1.rs}`) |
| Window freeze, broken mouse/gamepad, silent/glitchy audio | Use `audio-and-windowing` (`win32-rt/src/{user32,macos,audio}`, `crates/wwise-audio`) |

Once the game renders frames, capture automated smoke verification with `live-probe`:

```sh
cargo opsdb -- live-probe \
  --manifest games/<id>/game.toml \
  --game-dir /path/to/Game \
  --ready-regex "Present #10" \
  --settle-frames 60 \
  --step "shot out/bringup_f0.png" \
  --fail-regex 'UNIMPLEMENTED|\[diag-boundary\]'
```

## Step 5 — Verify & Land Cleanly

```sh
cargo runner-build
cargo factory-test -- --test-threads=1
cargo opsdb -- doctor
```

- Confirm `games/<id>/game.toml` contains zero personal paths or game binaries.
- Package as a portable `.app` / `.ipa` using the `package-macos-and-ios` skill once playable.
- Commit manifest cleanly as `games: add manifest for <title>` (and document honest status/caveats
  in `docs/STATUS.md` / `docs/TODOS.md` if applicable).
