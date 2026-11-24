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

## Step 1 — Audit the Game Install Offline (Single Pass)

Run the unified single-pass offline audit on the user's legally owned Windows game directory before
booting (parses every PE32+ file once and optionally pre-fills `games/<id>/game.toml`):

```sh
cargo opsdb -- audit-game /path/to/Game --init-manifest
# Machine-readable JSON output + manifest generation:
cargo opsdb -- audit-game /path/to/Game --json --init-manifest
# Or via the convenience wrapper script:
.agents/skills/bring-up-game/scripts/audit_game.sh /path/to/Game [MainGame.exe] [--record] [--json] [--init-manifest]
```

What to inspect in the output:
- **Win32 imports by `impl_kind` (`real` / `partial` / `stub` / `failing` / `unresolved`):**
  `audit-game` (and `win32-audit`) reports resolved counts separately by `impl_kind` from
  `db/win32_apis.tsv` and explicitly lists every `stub` / `failing` import the game uses. Check
  [game_toml_schema.md](./references/game_toml_schema.md) for which DLLs are overridden by native
  ARM64 builtins (`BUILTIN_OVERRIDES`) vs run as retail PEs via ARM64 JIT (`PhysX3*`, `nrd.dll`,
  compression DLLs `oo2core_*`/`zstd`).
- **Graphics API & embedded shaders:** D3D12, D3D11 (needs `libdxilconv.dylib` for SM 4/5), or
  Vulkan (needs `libMoltenVK.dylib`), plus upscalers (DLSS/Streamline, FSR, XeSS) and engine profile.
- **Anti-cheat / anti-tamper:** Kernel drivers (Vanguard, kernel EAC/BattlEye, Ricochet), direct
  syscalls/SSNs, RWX/packed sections, or anti-tamper watchdogs are reported honestly as blockers or
  caveats.
- **ARM64 AOT coverage estimate:** Aggregate instruction count, JIT coverage percentage, and top
  unmapped/generic-interp mnemonics.
- **.NET / Unity & runtime hazards:** Pure-IL/R2R/Mono/IL2CPP counts (`--strip-r2r` via
  `dotnet-audit` if needed), plus `ldmxcsr` FTZ/DAZ, x87, AVX 256→128 narrowing, approx FP, `LOCK`
  atomics, and RWX/V8/Cohtml (`v8_jitless = true` is pre-filled automatically when detected).
- **Steam installs:** Run `cargo opsdb -- steam-postinstall /path/to/Game` to extract
  `installscript.vdf` registry keys and check `_CommonRedist`.

## Step 2 — Review Pre-Filled `games/<id>/game.toml`

`cargo opsdb -- audit-game /path/to/Game --init-manifest` writes a pre-filled `games/<id>/game.toml`
with detected `[app]` metadata, `steam_app_id` (from `steam_appid.txt`), `v8_jitless`, and
`[middleware.*]` entries (or copy `games/_template/game.toml` manually if starting from scratch;
see `games/overwatch/game.toml`, `games/alan-wake-2/game.toml`, and
[game_toml_schema.md](./references/game_toml_schema.md)).

Minimal template:

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
# Negate specific builtins with `-foo.dll` (or `"0"` for all) to run the game's retail PE via JIT:
# builtin_overrides = "-physx3_x64.dll"
# args = "-windowed"
```

## Step 3 — Dry-Run Imports & Prewarm ARM64 `.aot` Cache

Build the thin universal runner (~15 s, zero AW2 shards) and precompile ARM64 `.aot` blocks:

```sh
cargo runner-build
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --ops-report --no-run
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --prewarm-aot
```

- Control thread count with `PORT_AOT_THREADS=<N>` or cache directory with `--aot-dir` /
  `PORT_AOT_DIR`. Test VFS mappings without booting via `cargo opsdb -- vfs-resolve "<WIN_PATH>"`.

## Step 4 — Boot & Triage Blockers One at a Time

Start headless first, then windowed:

```sh
PORT_HEADLESS=1 cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
```

Add `--trace` to log resolved imports (`RUST_LOG=trace=info`). Check `out/crash/crash-*.txt` and
`out/hitch/hitch-*.log`. Route each blocker to the appropriate skill:

| Symptom | Skill & Action |
|---|---|
| `UNIMPLEMENTED foo.dll!Bar`, wrong ABI/COM/middleware behavior | `implement-win32-api` (`crates/win32-rt` + `cargo opsdb -- win32-sync`) |
| Unhandled or hot `GENERIC_INTERP` x86 opcode | `implement-x86-op` (`crates/x86-runtime` + `cargo opsdb -- sync`) |
| Wrong math, NaN physics/geometry, AOT/JIT crash, hang | `debug-runtime-and-nan` (`verify-x86`, `live-probe`, `bisect-runtime`, `nan-trace`) |
| Shader conversion / `D3DCompile` / PSO stutter | `shader-pipeline` (`crates/dxil-metal`, `dxil-metald`, `d3dcompiler_47`) |
| Metal validation error (`MTL_DEBUG_LAYER=1`), black frame, D3D11/D3D12/Vulkan/upscaler bug | `graphics-metal4` (`crates/d3d12-metal`, `win32-rt/src/{d3d11,d3d12,misc/vulkan_1.rs}`) |
| Window freeze, broken mouse/gamepad/IME, silent/glitchy audio | `audio-and-windowing` (`win32-rt/src/{user32,macos,audio,misc/{fmod,xaudio2_9redist}}`) |

Once frames render, capture repeatable smoke verification with `live-probe`:

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
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh --staged --doctor
```

- Package as a self-contained macOS `.app` or iOS `.ipa` using `package-macos-and-ios` once playable.
- Commit cleanly (`games: add manifest for <title>`) and record honest caveats in `docs/STATUS.md`.
