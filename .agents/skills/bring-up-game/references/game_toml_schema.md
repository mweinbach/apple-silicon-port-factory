# `game.toml` Manifest Schema & `BUILTIN_OVERRIDES` Reference

Source of truth:
- `crates/port-loader/src/target.rs` (`TargetManifest::parse`)
- `crates/port-loader/src/loader.rs` (`BUILTIN_OVERRIDES`, `is_builtin_override_with`)
- `crates/port-factory/src/storage_opt.rs` (`parse_manifest_storage_config`)

## Supported `game.toml` Keys

| Section & Key | Type | Effect |
|---|---|---|
| `[app] id` | string | Kebab-case game ID (matches `games/<id>/` and default bundle ID suffix) |
| `[app] name` | string | Human-readable display name (AppKit menus, fallback window title, `.app` bundle name) |
| `[app] exe` | string | Main Windows x86-64 executable filename inside `--game-dir` |
| `[app] win_dir` | string | Virtual Windows directory mounted to `--game-dir` (e.g. `"C:\\Games\\MyGame"`) |
| `[app] log_name` | string | Runtime log filename under `~/Library/Logs/Translator/` |
| `[app] computer_name` | string | Value returned by `GetComputerNameA/W` and `COMPUTERNAME` |
| `[app] steam_app_id` | string | Optional Steam AppID (used by `steam_api64.dll` IPC and `steam-port` / `steam-convert`) |
| `[runtime] preferred_base_retry` | bool | Retry mapping at the PE's preferred `ImageBase` (leave `false` for normal games; `true` for AW2 lifted shards) |
| `[runtime] v8_jitless` | bool | Pass `--jitless` for games embedding V8 (e.g. Coherent Gameface) so V8 avoids guest RWX JIT |
| `[runtime] legacy_remedy_migration` | bool | AW2-only legacy save migration (always `false` for new games) |
| `[runtime] builtin_overrides` | string | Comma-separated override controls: `"0"`/`"none"`/`"false"` disables all builtin overrides; `"1"`/`"all"` overrides all; `"-foo.dll"` or `"!foo.dll"` forces loading the game's retail PE DLL via `x86-runtime`; `"bar.dll"` forces the builtin |
| `[runtime] aot_dylibs` | string | Optional prototype static Rust AOT dylib list |
| `[runtime] args` *(alias `game_args`)* | string | Default Windows guest CLI arguments (supports double-quoted tokens) |
| `[storage] keep_dlls` | `[string]` | DLL filenames preserved during `optimize-assets --strip-unused-dlls` (negative `builtin_overrides` and static `.aot.o` modules are preserved automatically) |
| `[storage] archive_repacker` | string | Optional external archive repacker binary name (e.g. `"aw2-data"`) — keeps `crates/` game-agnostic |
| `[storage] archive_dir` | string | Optional relative asset directory delegated to `archive_repacker` (e.g. `"data_pack2"`) |
| `[ios] seed_file` | string (repeatable) | `"<bundle_rel> -> <drive_c_rel>"` copied into `drive_c` on first iOS launch only when destination is absent |

## Discovery Precedence (`TargetManifest::discover`)

1. Explicit `--manifest <FILE>`
2. `$PORT_MANIFEST` environment variable
3. `<exe_dir>/game.toml` (flat iOS `.app` bundle)
4. `<exe_dir>/../Resources/game.toml` (macOS `.app` bundle)

## Default `BUILTIN_OVERRIDES` Policy (`crates/port-loader/src/loader.rs`)

When a DLL exists in the game directory **and** is in `BUILTIN_OVERRIDES`, `port-runner` uses our
native ARM64 Rust implementation instead of JITing the x86-64 PE:

- **Graphics/Upscaler/Vendor bridges:** `sl.interposer.dll`, `nvapi64.dll`, `nvapi.dll`,
  `amd_ags_x64.dll`, `nvngx_dlss.dll`, `_nvngx.dll`, `nvngx_dlssg.dll`, `nvngx_dlssd.dll`,
  `libxess.dll`, `libxess_dx12.dll`, `libxess_dx11.dll`, `amd_fidelityfx_loader_dx12.dll`,
  `dstorage.dll`, `dstoragecore.dll`, `nri.dll`
- **Media, Audio, Input & Cloth:** `bink2w64.dll`, `mediadecoders.windowsdesktop.dll`,
  `xaudio2_9redist.dll`, `gameinput.dll`, `nvcloth_x64.dll`
- **Platform SDKs, Networking & Runtimes:** `steam_api64.dll`, `steam_api.dll`,
  `eossdk-win64-shipping.dll`, `libcurl.dll`, `cohtml_icuuc.dll`, `libprotobuf.dll`,
  `mscoree.dll`, `hostfxr.dll`, `coreclr.dll`, `mono-2.0-bdwgc.dll`,
  `tbb12.dll`, `tbb.dll`, `tbbmalloc.dll`, `zlib1.dll`, `v8_zlib.dll`, `lz4.dll`

**Intentionally NOT overridden by default** (games shipping them run the retail PE via ARM64 JIT):
- **PhysX 3.4** (`physx3_x64.dll`, `physx3common_x64.dll`, `physx3cooking_x64.dll`, `pxfoundation_x64.dll`)
- **NVIDIA NRD** (`nrd.dll` — retail PE has full ARM64 JIT coverage & embedded shaders; builtin acts as fallback when absent)
- **Shipping decompressors** (`oo2core_*`, `zstd.dll`, etc. — never replace real decompressors with stubs)

If a game's shipping DLL works better via ARM64 JIT (or while diagnosing a builtin bug), negate it
in `game.toml` via `builtin_overrides = "-foo.dll"` or test live with
`PORT_BUILTIN_OVERRIDES="-foo.dll"`.
