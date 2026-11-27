# Command-Line Game Acquisition (`tools/port-acquire`)

`tools/port-acquire` is a game-agnostic, pluggable CLI under `tools/` for acquiring **legally owned**
Windows x86-64 PC game builds from supported storefronts and handing them directly to the Apple
Silicon Port Factory audit, bring-up, and `.app` packaging pipeline.

> [!IMPORTANT]
> **Hard Policy — Zero Bypass, Zero Stored Secrets**
> - **No DRM, anti-cheat, or ownership bypass of any kind.** `port-acquire` only queries and
>   downloads titles owned by the currently authenticated account via established open-source store
>   clients (`legendary` for Epic Games Store; pluggable backends such as Steam via
>   `tools/port_acquire/backends/`).
> - **Never commit game files or credentials.** Authentication tokens remain strictly inside the
>   store client's own config directory (for Epic: `legendary`'s `user.json` under
>   `~/.config/legendary` or `$PORT_LEGENDARY_CONFIG`). `status` reports only login state and the
>   account display name; `launch-args` and exported manifests automatically redact auth/token flags
>   and environment variables.

---

## 1. Installing the Epic Backend (`legendary`)

The `epic` backend wraps Heroic Games Launcher's open-source [`legendary`](https://github.com/derrod/legendary)
CLI (`legendary-gl`). It never reimplements the Epic Games protocol.

Install `legendary` via pipx/pip or reuse Heroic Games Launcher's bundled ARM64 binary:

```sh
# Option A: pipx (recommended on macOS)
brew install pipx
pipx install legendary-gl

# Option B: pip3
pip3 install --user legendary-gl

# Option C: Heroic Games Launcher already installed in /Applications/Heroic.app
# `port-acquire` auto-discovers Heroic's bundled arm64/x64 `legendary` binary.
```

### Environment Knobs (Registered in `crates/port-manifest/src/knobs.rs`)

| Variable | Default | Purpose |
|---|---|---|
| `PORT_ACQUIRE_STORE` | `epic` | Default `--store` backend selected by `tools/port-acquire`. |
| `PORT_ACQUIRE_DEST` | `~/Games/PortFactory/<store>` | Default base directory for `download` when `--dest` is omitted. |
| `PORT_LEGENDARY` | auto-discovered | Explicit path to the `legendary` executable. |
| `PORT_LEGENDARY_CONFIG` | `~/.config/legendary` | Override `LEGENDARY_CONFIG_PATH` (e.g. set to `$HOME/Library/Application Support/heroic/legendaryConfig/legendary` to share an existing Heroic session). |

---

## 2. Interactive Login (Run by the Account Owner)

Authentication uses Epic's official browser or authorization-code flow and **must be run
interactively by the account owner** — never by an agent with scraped credentials:

```sh
# Official browser / webview login
tools/port-acquire login --store epic

# Or open in your default browser and paste the official authorization code
tools/port-acquire login --store epic --no-webview

# Check status afterward (prints only logged_in + display_name; never tokens)
tools/port-acquire status --store epic --json

# Log out (removes legendary's saved session)
tools/port-acquire login --store epic --logout
```

If an agent invokes `tools/port-acquire login` without a TTY, it exits with code `3`
(`auth_required`) and prints the exact interactive command for the user to run.

---

## 3. Agent-Facing Workflow ("I own X on Epic")

When a user says *"I own `<Title>` on Epic — download and port it"*, follow this exact sequence.
Pass `--json` on every step so every decision is driven by structured output.

### Step 1 — Check login status

```sh
tools/port-acquire status --store epic --json
```

- If `backend_available` is `false`: ask the user to install `legendary-gl` (or Heroic) or set `PORT_LEGENDARY`.
- If `logged_in` is `false`: **stop immediately** and tell the user to run:
  ```sh
  tools/port-acquire login --store epic
  ```

### Step 2 — Fuzzy-search the owned library to resolve `app_id`

```sh
tools/port-acquire search "Spider-Man 2" --store epic --platform windows --json
```

- Inspect `matches[0].app_id`, `matches[0].title`, and `matches[0].score`.
- If multiple candidates have close scores or `matches` is empty, run
  `tools/port-acquire list-owned --store epic --platform windows --json` and ask the user to confirm
  which owned title they mean.

### Step 3 — Inspect size, EOS, anti-cheat, DRM, DLCs, and redistributables

```sh
tools/port-acquire info <app_id> --store epic --platform windows --json
```

Review the JSON fields before downloading:
- `download_size_bytes` / `disk_size_bytes`: verify adequate free disk space.
- `anticheat` (`report_only: true`): if `detected == true`, report the exact anti-cheat system(s)
  honestly to the user (kernel anti-cheat is unsupported and never bypassed).
- `drm` (`report_only: true`): report `requires_online_auth`, `requires_ownership_token`, and any
  `third_party_launcher` honestly.
- `requires_eos` & `eos_signals`: note whether the title ships `EOSSDK-Win64-Shipping.dll` or
  requests EOS overlay bits.
- `dlcs` (or `tools/port-acquire list-dlcs <app_id> --json`): check owned DLCs/add-ons and confirm
  whether to include `--with-dlcs` or specific `--dlc <dlc_app_id>` flags.
- `redistributables` (or `tools/port-acquire list-redists <app_id> --json`): VC++ and DirectX
  entries are flagged `native_in_win32_rt: true` (no installer run needed); EOS SDK entries are
  flagged `native_in_win32_rt: false` so you know whether `--with-eos-overlay` or `[middleware.eos]`
  is relevant.

### Step 4 — Download the Windows build (and optional DLCs / EOS overlay)

```sh
tools/port-acquire download <app_id> \
  --store epic \
  --platform windows \
  --dest "$HOME/Games/PortFactory/epic/<Title>" \
  --with-dlcs \
  --json
```

- Add `--with-eos-overlay` only when the title requires Epic's EOS overlay redistributable bits.
- Use `--dlc <dlc_app_id>` (repeatable) instead of `--with-dlcs` to download specific owned DLCs.
- By default, `download` writes `<dest>/port-acquire-manifest.json` (schema v1, strictly
  secret-free).

### Step 5 — Verify downloaded files

```sh
tools/port-acquire verify <app_id> --store epic --json
# If any file is missing or corrupt:
tools/port-acquire verify <app_id> --store epic --repair --json
```

### Step 6 — Hand off to the Port Factory bring-up & convert checklist

Once the Windows build is on disk at `<game-dir>`, switch to the standard Port Factory pipeline
(see `.agents/skills/bring-up-game/SKILL.md` and [docs/OPS_DB.md](OPS_DB.md)):

```sh
# 1. Offline single-pass PE, Win32, shader, anti-cheat & instruction readiness audit
cargo opsdb -- audit-game "<game-dir>" --json --init-manifest
cargo opsdb -- readiness "<game-dir>" --json

# 2. (Optional) Export sanitized launch args + starter game.toml
tools/port-acquire launch-args <app_id> --store epic --json \
  --export-manifest "<game-dir>/port-acquire-manifest.json" \
  --export-game-toml "games/<id>/game.toml"

# 3. Build universal runner and prewarm ARM64 .aot cache
cargo runner-build
cargo runner -- --manifest "games/<id>/game.toml" --game-dir "<game-dir>" --ops-report --no-run
cargo runner -- --manifest "games/<id>/game.toml" --game-dir "<game-dir>" --prewarm-aot

# 4. Boot headless first, then windowed
PORT_HEADLESS=1 cargo runner -- --manifest "games/<id>/game.toml" --game-dir "<game-dir>"
cargo runner -- --manifest "games/<id>/game.toml" --game-dir "<game-dir>"

# 5. Once playable, package into a self-contained macOS .app bundle and verify portability
cargo opsdb -- steam-convert "<game-dir>" --format app --max-fns 0 --out "out/apps/<Title>.app"
cargo opsdb -- bundle-verify "out/apps/<Title>.app"
```

---

## 4. CLI Command Reference

All commands accept `--store <name>` and `--json` either before or after the subcommand.

| Command | Description |
|---|---|
| `stores` | List discovered/registered store backends (`epic`, and future plugins like `steam`). |
| `login [--no-webview] [--code CODE] [--import-existing] [--logout]` | Run official interactive authentication (or log out). Never logs codes or tokens. |
| `status [--offline]` | Report `logged_in`, `display_name`, and backend binary availability only. |
| `list-owned [--platform windows\|macos\|all] [--refresh]` | List owned base titles with attached owned DLCs and local install state. |
| `search <query> [--platform ...] [--limit N] [--min-score F]` | Fuzzy-match a human title (including Roman/Arabic numerals) to a store `app_id`. |
| `info <app> [--platform windows\|macos] [--offline]` | Report download/disk size, executable, offline support, EOS/anti-cheat/DRM signals (report-only), DLCs, and redistributables. |
| `list-dlcs <app>` (`dlcs`) | List DLCs and add-ons for `<app>`. |
| `list-redists <app>` (`redists`) | List prerequisites, VC++/DirectX redistributables, and EOS SDK components. |
| `download <app> [--dest DIR] [--platform windows\|macos] [--with-dlcs] [--dlc ID ...] [--with-eos-overlay] [--no-manifest]` | Download an owned build and write `<dest>/port-acquire-manifest.json`. |
| `verify <app> [--repair]` | Verify installed files against the official store manifest (and optionally repair). |
| `update <app> [--check-only] [--with-dlcs]` | Check for or apply updates and refresh any existing `port-acquire-manifest.json`. |
| `uninstall <app> [--keep-files] [--skip-uninstaller]` | Uninstall a title managed by the store backend. |
| `launch-args <app> [--online] [--export-manifest PATH] [--export-game-toml PATH]` | Emit sanitized launch arguments (defaults to `--offline`; always redacts auth/token args and env vars). |

---

## 5. Package Manifest (`port-acquire-manifest.json`) & `.app` Packaging Hook Point

`tools/port-acquire download` and `tools/port-acquire launch-args --export-manifest` write a
schema-v1 JSON file named `port-acquire-manifest.json` in the downloaded game directory:

```json
{
  "schema_version": 1,
  "generated_by": "port-acquire",
  "store": "epic",
  "app_id": "ExampleAppName",
  "title": "Example Game",
  "platform": "windows",
  "version": "1.1.0",
  "install_dir": "/path/to/ExampleGame",
  "executable": "Binaries/Win64/ExampleGame.exe",
  "working_dir": "/path/to/ExampleGame/Binaries/Win64",
  "launch_args": ["-epicapp=ExampleAppName", "-windowed"],
  "launch_env": {},
  "redacted_auth_args": true,
  "dlcs": [],
  "redistributables": [
    {
      "id": "vcredist:vc_redist.x64.exe",
      "name": "Visual C++ 2022 Redistributable",
      "category": "vcredist",
      "source": "prereq_info",
      "relative_path": "Redist/VC_redist.x64.exe",
      "args": "/install /quiet /norestart",
      "native_in_win32_rt": true,
      "present_on_disk": true,
      "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime"
    }
  ],
  "compatibility_signals": {
    "can_run_offline": true,
    "requires_eos": true,
    "eos_signals": ["manifest_file:EOSSDK-Win64-Shipping.dll"],
    "anticheat": { "detected": false, "systems": [], "signals": [], "report_only": true },
    "drm": { "detected": false, "requires_online_auth": false, "requires_ownership_token": false, "third_party_launcher": null, "signals": [], "report_only": true }
  },
  "packaging_hook": {
    "format": "port-acquire-manifest-v1",
    "expected_bundle_resource": "Contents/Resources/port-acquire-manifest.json"
  }
}
```

### How `.app` Packaging Consumes This Manifest (Future Hook Point)

`crates/port-factory/src/bundle_common.rs` (`copy_game_payload`) already preserves
`port-acquire-manifest.json` inside `Contents/Resources/Game/` when converting a downloaded folder.
To wire first-class store-agnostic packaging metadata without adding game-specific Rust code:

1. **`crates/port-factory/src/steam_library/vdf.rs` (`scan_post_install` / `apply_post_install`)**:
   If `<game-dir>/port-acquire-manifest.json` exists (`schema_version == 1`), read its
   `redistributables` array alongside `installscript.vdf` / `_CommonRedist`:
   - Entries with `native_in_win32_rt: true` (`vcredist`, `directx`) are recorded as natively
     satisfied by `crates/win32-rt` (no Windows installer run needed).
   - Entries with `category == "eos_sdk"` record shipped EOS DLLs / overlay paths for middleware
     diagnostics.
2. **`crates/port-factory/src/steam_library/macos_bundle.rs` (`convert_steam_game` / `package_macos_game_bundle`)**:
   - Read `title` and `executable` from `port-acquire-manifest.json` to select the primary Windows
     PE deterministically when no curated `games/<id>/game.toml` exists.
   - Copy `port-acquire-manifest.json` into `Contents/Resources/port-acquire-manifest.json`.
   - Seed non-secret `launch_args` into generated `Contents/Resources/game.toml` `[runtime].args`.
3. **`crates/port-factory/src/steam_library/verify.rs` (`verify_app_bundle`)**:
   - Optionally validate that `Contents/Resources/port-acquire-manifest.json` (when present) has
     `schema_version == 1` and zero unredacted auth flags.

---

## 6. Adding Another Store Backend (e.g. `steam`, `gog`, `amazon`)

The shared interface lives in [`tools/port_acquire/backend.py`](../tools/port_acquire/backend.py)
and backend discovery lives in [`tools/port_acquire/registry.py`](../tools/port_acquire/registry.py).

To plug in a new store without editing any shared files:

1. Create `tools/port_acquire/backends/<store>.py` (for example, `backends/steam.py`).
2. Import `StoreBackend` and the typed result dataclasses from `port_acquire.backend`.
3. Subclass `StoreBackend`, set `name = "<store>"` and `display_name = "..."`, and implement the
   abstract methods (`login`, `status`, `list_owned`, `info`, `download`, `verify`, `update`,
   `uninstall`, `launch_args`). `search()`, `list_dlcs()`, and `list_redists()` have default
   implementations built on `list_owned()` and `info()`.
4. Export `BACKEND_CLASS = YourBackend` (or a zero-argument `create_backend() -> StoreBackend`).
5. Add offline mocked unit tests at `tools/port_acquire/tests/test_<store>_backend.py`.

`tools/port-acquire --store <store> ...` and `tools/port-acquire stores --json` pick up the new
backend automatically.
