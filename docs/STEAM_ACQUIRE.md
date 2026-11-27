# Steam Game Acquisition (`tools/port-acquire --store steam`)

This guide covers the agent-facing and user-facing workflow for acquiring **legally owned**
Windows x86-64 games from Steam on Apple Silicon macOS and handing them off to the
Apple Silicon Port Factory pipeline (`audit-game`, `port-runner`, and `.app` packaging).

> **Hard policy:** Never bypass DRM, anti-cheat, or store ownership checks. Only download
> titles owned by the logged-in Steam account through Valve's official tools. Never commit
> game binaries, assets, passwords, Steam Guard codes, or tokens.

---

## 1. Tool Selection: Why `steamcmd` (and when `DepotDownloader` is used)

The `steam` backend (`tools/port_acquire/backends/steam.py`) uses **Valve's official
`steamcmd`** as its primary engine, with native Apple Silicon `DepotDownloader` as an
optional download/verify fallback:

| Capability | Valve `steamcmd` (Primary) | `DepotDownloader` (Fallback) |
|---|---|---|
| Force Windows depots on macOS | `+@sSteamCmdForcePlatformType windows` | `-os windows -osarch 64` |
| Interactive login + Steam Guard token cache | Built-in (`+login <user>`, cached by Steam in `config.vdf`) | `-username` / `-qr` + `-remember-password` |
| Owned library license list (`list-owned`) | Built-in (`+licenses_print`) | Not supported |
| Full depot, branch/beta, DLC & `228980` redist metadata (`info`) | Built-in (`+app_info_update 1 +app_info_request <id> +app_info_print <id>`) | Not supported |
| Branch / beta switching (`--branch`) | `+app_update <id> -beta <branch> [-betapassword <pw>]` | `-branch <branch> [-branchpassword <pw>]` |
| Checksum verification (`verify`) | `+app_update <id> validate` | `-validate` |
| Targeted DLC & `_CommonRedist` (`228980`) depot pulls | `+download_depot <app_id> <depot_id>` | `-app <id> -depot <depot_id>` |
| macOS Apple Silicon (`arm64`) binary | Intel `x86_64` (runs via **Rosetta 2**) | Native `arm64` Mach-O (`cargo opsdb -- steam-setup`) |

**Why `steamcmd` is primary:** `DepotDownloader` only fetches manifests/depots and cannot
query owned licenses (`licenses_print`) or full AppInfo VDF (`app_info_print`) for depots,
branches, DLC lists, Steamworks Common Redistributables (`228980`), or Proton/Steam Deck
compatibility metadata. `steamcmd` provides all of those directly from Valve.

**macOS `arm64` caveat:** Valve distributes macOS `steamcmd` as a 64-bit Intel (`x86_64`)
binary. On Apple Silicon Macs, install Rosetta 2 once and then install `steamcmd`:

```sh
softwareupdate --install-rosetta --agree-to-license
brew install --cask steamcmd
```

If `steamcmd` or Rosetta 2 is absent on a machine, `download`, `verify`, and `update` fall
back automatically to native `arm64` `DepotDownloader` (installed via `cargo opsdb --
steam-setup` or located via `PORT_DEPOT_DOWNLOADER`).

---

## 2. Authentication & Secret Hygiene

1. **Interactive login is always run by the account owner (never by an automated agent):**
   ```sh
   tools/port-acquire --store steam login --username <your_steam_account>
   ```
   `port-acquire` attaches `steamcmd +login <account> +quit` directly to the user's terminal
   so password and Steam Guard prompts go straight to Valve's `steamcmd`. Refresh tokens stay
   exclusively in Steam's own cache (`~/Library/Application Support/Steam/config/config.vdf`).
2. **Never pass or store passwords, Steam Guard codes, or refresh tokens** in git, CI,
   manifests, or chat/thread logs. `port-acquire` stores only `{account_name, steam_id64}` in
   `~/.local/share/port-acquire/steam-session.json`.
3. **`status` reports login state and account name only:**
   ```sh
   tools/port-acquire --store steam status --json
   ```
4. **Optional Steam Web API key (for faster `list-owned` with titles):**
   Without an API key, `list-owned` uses `steamcmd +licenses_print` out of the box. If you
   want full title + playtime resolution via Valve's `IPlayerService/GetOwnedGames/v1`, store
   a Steam Web API key in the macOS Keychain (preferred) or set the registered
   `PORT_STEAM_API_KEY` environment variable:
   ```sh
   # Preferred: macOS Keychain (never touches disk in repo or shell history when using -w prompt)
   security add-generic-password -a "$USER" -s port-acquire-steam-web-api-key -w
   ```
5. **Private beta branch passwords:** Never pass branch passwords on the CLI. Use
   `--branch-password-env MY_BETA_PW` so the value is read from the environment and redacted
   from logs and manifests.

### Registered `PORT_*` Environment Knobs (`crates/port-manifest/src/knobs.rs`)

| Knob | Purpose |
|---|---|
| `PORT_STEAMCMD` | Override path to `steamcmd` / `steamcmd.sh` |
| `PORT_STEAM_API_KEY` | Optional Steam Web API key (alternative to macOS Keychain; never logged) |
| `PORT_STEAM_HOME` | Override Steam config/library directory (default `~/Library/Application Support/Steam`) |
| `PORT_STEAM_USER` | Optional default Steam account name (never a password) |
| `PORT_DEPOT_DOWNLOADER` | Override path to native `arm64` `DepotDownloader` fallback |

---

## 3. Agent-Facing Workflow ("I own X on Steam")

When the user says **"I own `<Game Title>` on Steam"**, follow these exact steps:

### Step 1 — Check login status (do NOT run `login` yourself)

```sh
tools/port-acquire --store steam status --json
```

- If `logged_in` is `false`, **stop** and give the user the exact command to run in their own
  terminal:
  ```sh
  tools/port-acquire --store steam login --username <your_steam_account>
  ```
- If `backend_available` is `false`, ask the user to install `steamcmd`:
  ```sh
  softwareupdate --install-rosetta --agree-to-license
  brew install --cask steamcmd
  ```

### Step 2 — Find the owned AppID

```sh
tools/port-acquire --store steam search "<Game Title>" --json
# Or inspect the full owned list:
tools/port-acquire --store steam list-owned --platform windows --json
```

`search` fuzzy-matches against the user's owned library (and when `licenses_print` has
numeric-only entries without a Steam Web API key, intersects Steam's public Store Search
results strictly with the user's owned AppIDs so unowned titles are never returned).

### Step 3 — Inspect depots, size, DLCs, redists, and DRM/anti-cheat (report-only)

```sh
tools/port-acquire --store steam info <appid> --platform windows --json
tools/port-acquire --store steam list-dlcs <appid> --json
tools/port-acquire --store steam list-redists <appid> --json
```

Review the JSON report with the user before downloading:
- `download_size_bytes` / `disk_size_bytes` and `depots` (Windows x64 vs DLC vs `228980`
  shared redist depots)
- `branches` (default `public` plus any available beta branches)
- `requires_steamworks` / `steamworks_signals`
- `drm` (`report_only: true` — flags Denuvo, third-party launchers, online activation)
- `anticheat` (`report_only: true` — flags EAC, BattlEye, VAC, Vanguard, Ricochet)
- `proton_redist_info` (App `228980` Steamworks Common Redistributables, `installscript.vdf`,
  and Valve's `steam_deck_compatibility` / recommended Proton runtime notes)

If kernel anti-cheat or incompatible DRM is reported, inform the user honestly — **never**
attempt any bypass.

### Step 4 — Download Windows depots (+ optional branch, DLCs, and `_CommonRedist`)

```sh
tools/port-acquire --store steam download <appid> \
  --dest ~/Games/PortFactory/steam/<GameName> \
  --platform windows \
  --with-dlcs \
  --with-redists \
  --validate \
  --json
```

For a beta branch:
```sh
tools/port-acquire --store steam download <appid> \
  --dest ~/Games/PortFactory/steam/<GameName> \
  --platform windows \
  --branch <branch_name> \
  --json
```

This writes:
- `<dest>/steam_appid.txt`
- `<dest>/port-acquire-manifest.json` (schema v1, strictly secret-free)
- `<dest>/_CommonRedist/<depot_id>/` (when `--with-redists` fetches App `228980` depots)

### Step 5 — Maintenance (`verify`, `update`, `uninstall`, `launch-args`)

```sh
tools/port-acquire --store steam verify <appid> --repair --json
tools/port-acquire --store steam update <appid> --check-only --json
tools/port-acquire --store steam update <appid> --with-dlcs --json
tools/port-acquire --store steam launch-args <appid> \
  --export-manifest <dest>/port-acquire-manifest.json \
  --export-game-toml games/<id>/game.toml \
  --json
tools/port-acquire --store steam uninstall <appid> --json
```

### Step 6 — Hand off to the Port Factory bring-up & `.app` pipeline

Once downloaded, follow the standard game-agnostic pipeline (see
`.agents/skills/bring-up-game/SKILL.md` and `docs/OPS_DB.md`):

```sh
# 1. Single-pass offline PE, Win32 import, shader, anti-cheat, and .NET audit
cargo opsdb -- audit-game ~/Games/PortFactory/steam/<GameName> --json --init-manifest

# 2. Extract Steam installscript.vdf registry keys & inspect _CommonRedist
cargo opsdb -- steam-postinstall ~/Games/PortFactory/steam/<GameName>

# 3. Build universal runner and prewarm ARM64 .aot block caches
cargo runner-build
cargo runner -- --manifest games/<id>/game.toml \
  --game-dir ~/Games/PortFactory/steam/<GameName> --prewarm-aot

# 4. Boot headless, then windowed
PORT_HEADLESS=1 cargo runner -- --manifest games/<id>/game.toml \
  --game-dir ~/Games/PortFactory/steam/<GameName>

# 5. Package a self-contained macOS .app bundle and verify portability
cargo opsdb -- steam-convert ~/Games/PortFactory/steam/<GameName> \
  --format app --copy-game --max-fns 0 --out out/apps/<GameName>.app
cargo opsdb -- bundle-verify out/apps/<GameName>.app
```

---

## 4. Packaging Hook Point (`port-acquire-manifest.json`)

Every `download` (and `launch-args --export-manifest`) writes a game-agnostic schema-v1
JSON file at `<dest>/port-acquire-manifest.json`. There is **zero game-specific logic** in
the manifest or hook.

Key fields recorded for downstream `.app` packaging
(`crates/port-factory/src/steam_library/macos_bundle.rs` and `vdf.rs`):

- `store`, `app_id`, `title`, `platform`, `version` (`buildid`), `branch`
- `executable`, `working_dir`, `launch_args`, `launch_env` (sanitized; auth tokens redacted)
- `depots` and `downloaded_depots` (depot IDs, OS/arch filters, manifest GIDs, sizes, DLC links)
- `dlcs` (owned/installed DLC AppIDs and sizes)
- `redistributables` and `downloaded_redists` (each tagged with `native_in_win32_rt: true/false`
  so `.app` bundling knows VC++, DirectX, and OpenAL `228980` depots are satisfied natively by
  `crates/win32-rt` while `.NET` and `PhysX` stay flagged for audit)
- `proton_redist_info` (`shared_redist_app_id: "228980"`, `shared_depots`, `common_redist_dirs`,
  `install_scripts`, `steam_deck_compatibility` / `recommended_runtime`, and `proton_notes`)
- `compatibility_signals` (`requires_steamworks`, `steamworks_signals`, `anticheat`, `drm` —
  strictly report-only)
- `packaging_hook`:
  ```json
  {
    "format": "port-acquire-manifest-v1",
    "expected_bundle_resource": "Contents/Resources/port-acquire-manifest.json",
    "notes": "Consumed by .app packaging (`crates/port-factory/src/steam_library/macos_bundle.rs` & `vdf.rs`) ..."
  }
  ```

Future `.app` packaging work in `crates/port-factory/src/steam_library/macos_bundle.rs` can
read `<game_dir>/port-acquire-manifest.json` (when present) to:
1. Select `executable` and seed `steam_app_id` + sanitized `launch_args` into
   `Contents/Resources/game.toml` without heuristic PE guessing.
2. Copy `port-acquire-manifest.json` into `Contents/Resources/port-acquire-manifest.json`.
3. Skip staging redundant `native_in_win32_rt: true` redistributables while preserving DLC
   records and Proton/Steamworks metadata for `bundle-verify`.
