# Steam Game Acquisition (`tools/port-acquire --store steam`)

This guide covers the agent-facing and user-facing workflow for acquiring **legally owned**
Windows x86-64 games from Steam on Apple Silicon macOS (`arm64`) and handing them off to
the Apple Silicon Port Factory pipeline (`audit-game`, `port-runner`, and `.app` packaging).

> **Hard policy:** Never bypass DRM, anti-cheat, or store ownership checks. Only download
> titles owned by the logged-in Steam account through Steam's normal depot protocol. Never
> commit game binaries, assets, passwords, Steam Guard codes, or tokens.

---

## 1. Tool Selection: Native `arm64` `DepotDownloader`

The `steam` backend (`tools/port_acquire/backends/steam.py`) uses **SteamRE/DepotDownloader**
(standalone native Apple Silicon `arm64` macOS binary — zero x86 translation layer, zero
`.NET` runtime dependency):

- **Install (one command via Port Factory):**
  ```sh
  cargo opsdb -- steam-setup
  ```
  This downloads the official standalone `DepotDownloader-macos-arm64.zip` release from
  `SteamRE/DepotDownloader` into `~/.local/opt/depotdownloader/DepotDownloader` and symlinks
  `~/.local/bin/DepotDownloader`. You can also override the binary path with the registered
  `PORT_DEPOT_DOWNLOADER` environment knob.

### Capabilities & Honest Gaps

| Operation | How `port-acquire --store steam` handles it | Notes / Gaps |
|---|---|---|
| `login` | `DepotDownloader -app 228980 -manifest-only -username <user> -remember-password` (interactive TTY) | Account owner enters password and completes Steam Guard directly; refresh token stays in `DepotDownloader`'s own cache. Zero game files downloaded during login. |
| `status` | Non-interactive `-manifest-only` probe on free App `228980` (`--offline` inspects local session metadata only) | Reports login state, account name, and `DepotDownloader` version only — never tokens or secrets. |
| `list-owned` | **1)** Steam Web API `IPlayerService/GetOwnedGames/v1` (when an optional Web API key is stored in macOS Keychain or `PORT_STEAM_API_KEY` and SteamID64 is known); **2)** account-scoped local Steam library caches (`userdata/<account_id32>/config/{localconfig.vdf,librarycache}` + account-owned `steamapps/appmanifest_*.acf` + managed installs) | **Gap:** `DepotDownloader` has no `licenses_print` command. Without a Steam Web API key or a local macOS Steam client cache for the active account, unplayed/uninstalled owned games are not enumerated automatically — pass the numeric AppID directly to `info`/`download`, and `DepotDownloader` still enforces Valve's server-side ownership check. |
| `search <name>` | Fuzzy match over owned library (+ Steam public Store Search intersected strictly with owned AppIDs to resolve unnamed local cache IDs) | Never returns unowned titles. |
| `info <app>` | `DepotDownloader -app <id> -os windows -osarch 64 -manifest-only` + cached Steam Store `appdetails` | Gives exact depot IDs, manifest GIDs, compressed/disk sizes, and **complete file lists before downloading** (detecting `.exe`, Steamworks `steam_api64.dll`, anti-cheat binaries, and `_CommonRedist`), plus cached Store `drm_notice` and DLC IDs checked against account-scoped owned entitlements. **Gap:** `DepotDownloader` does not dump raw `appinfo.vdf` branch catalogs or Steam Deck compatibility blocks; pass explicit `--branch <name>` if targeting a beta branch. |
| `download` | `DepotDownloader -app <id> -os windows -osarch 64 -branch <branch> [-branchpassword <pw>] -dir <dest> -username <user> -remember-password [-validate]` | Also downloads owned DLCs (`--with-dlcs` / `--dlc <id>`) and App `228980` Steamworks Common Redistributables (`--with-redists`), and writes `<dest>/port-acquire-manifest.json`. |
| `verify` | Read-only `DepotDownloader -manifest-only` + local file size/SHA-1 check; `verify --repair` runs `DepotDownloader ... -validate` | Default `verify` never modifies or downloads files on disk; `--repair` re-downloads missing or corrupted files via `DepotDownloader -validate`. |
| `update` | Fresh `DepotDownloader -manifest-only` comparison against stored manifest GIDs, then in-place `download` if changed | `--check-only` fails honestly if remote manifests cannot be retrieved instead of falsely claiming up-to-date. |
| `uninstall` | Removes managed install directory (with exact-title/numeric-AppID safety check) and registry entry | Refuses ambiguous/non-exact titles on destructive uninstall. |

---

## 2. Authentication & Secret Hygiene

1. **Interactive login is always run by the account owner (never by an automated agent):**
   ```sh
   tools/port-acquire --store steam login --username <your_steam_account>
   ```
   `port-acquire` attaches `DepotDownloader` directly to the user's terminal with
   `-username <account> -remember-password` so password and Steam Guard prompts go straight
   to `DepotDownloader`. Refresh tokens stay exclusively in `DepotDownloader`'s own cache.
2. **Never pass or store passwords, Steam Guard codes, or refresh tokens** in git, CI,
   manifests, or chat/thread logs. `port-acquire` stores only `{logged_out, account_name,
   steam_id64}` in `~/.local/share/port-acquire/steam-session.json`.
3. **`status` reports login state and account name only:**
   ```sh
   tools/port-acquire --store steam status --json
   ```
4. **Optional Steam Web API key (for full `list-owned` with titles):**
   Store a Steam Web API key in the macOS Keychain (preferred) or set the registered
   `PORT_STEAM_API_KEY` environment variable (automatically redacted to `<redacted>` in
   `port-runner --knobs` and `--knobs --json`):
   ```sh
   # Preferred: macOS Keychain (prompts securely without placing the key in shell history)
   security add-generic-password -a "$USER" -s port-acquire-steam-web-api-key -w
   ```
5. **Private beta branch passwords:** Never pass branch passwords on the CLI. Use
   `--branch-password-env MY_BETA_PW` so the value is read from the environment and redacted
   from error logs and manifests.

### Registered `PORT_*` Environment Knobs (`crates/port-manifest/src/knobs.rs`)

| Knob | Purpose |
|---|---|
| `PORT_DEPOT_DOWNLOADER` | Override path to native `arm64` `DepotDownloader` binary |
| `PORT_STEAM_API_KEY` | Optional Steam Web API key (alternative to macOS Keychain; redacted in `--knobs` diagnostics) |
| `PORT_STEAM_HOME` | Override local Steam config/library directory (default `~/Library/Application Support/Steam`) |
| `PORT_STEAM_USER` | Optional default Steam account name (never a password) |

---

## 3. Agent-Facing Workflow ("I own X on Steam")

When the user says **"I own `<Game Title>` on Steam"**, follow these exact steps:

### Step 1 — Check login status (do NOT run `login` yourself)

```sh
tools/port-acquire --store steam status --json
```

- If `backend_available` is `false`, install native `arm64` `DepotDownloader`:
  ```sh
  cargo opsdb -- steam-setup
  ```
- If `logged_in` is `false`, **stop** and give the user the exact native login command to
  run in their own terminal:
  ```sh
  tools/port-acquire --store steam login --username <your_steam_account>
  ```

### Step 2 — Find the owned AppID

```sh
tools/port-acquire --store steam search "<Game Title>" --json
# Or inspect the owned list:
tools/port-acquire --store steam list-owned --platform windows --json
```

If the user has neither a Steam Web API key nor a local Steam cache entry for the game, ask
for or confirm the numeric Steam AppID and proceed to Step 3 (`DepotDownloader` enforces
ownership directly against Valve's servers).

### Step 3 — Inspect depots, size, DLCs, redists, and DRM/anti-cheat (report-only)

```sh
tools/port-acquire --store steam info <appid> --platform windows --json
tools/port-acquire --store steam list-dlcs <appid> --json
tools/port-acquire --store steam list-redists <appid> --json
```

Review the JSON report with the user before downloading:
- `download_size_bytes` / `disk_size_bytes` and `depots` (Windows x64 manifest GIDs and sizes)
- `dlcs` (each tagged honestly with `owned: true/false` against known account entitlements)
- `requires_steamworks` / `steamworks_signals`
- `drm` (`report_only: true` — flags Denuvo, third-party launchers, online activation)
- `anticheat` (`report_only: true` — flags EAC, BattlEye, VAC, Vanguard, Ricochet)
- `proton_redist_info` (inferred App `228980` Steamworks Common Redistributables,
  `_CommonRedist` paths, `installscript.vdf`, and runtime notes)

If kernel anti-cheat or incompatible DRM is reported, inform the user honestly — **never**
attempt any bypass.

### Step 4 — Download Windows depots (+ optional branch, owned DLCs, and `_CommonRedist`)

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

Every `download` (including `--no-register`) and `launch-args --export-manifest` writes a
game-agnostic schema-v1 JSON file at `<dest>/port-acquire-manifest.json`. There is **zero
game-specific logic** in the manifest or hook.

Key fields recorded for downstream `.app` packaging
(`crates/port-factory/src/steam_library/macos_bundle.rs` and `vdf.rs`):

- `store`, `app_id`, `title`, `platform`, `version` (manifest GID fingerprint), `branch`
- `executable`, `working_dir`, `launch_args`, `launch_env` (sanitized; auth tokens redacted)
- `depots` and `downloaded_depots` (depot IDs, OS/arch filters, manifest GIDs, sizes)
- `dlcs` (AppIDs with honest `owned` and `installed` flags)
- `redistributables` and `downloaded_redists` (each tagged with `native_in_win32_rt: true/false`
  so `.app` bundling knows VC++, DirectX, and OpenAL `228980` depots are satisfied natively by
  `crates/win32-rt` while `.NET` and `PhysX` stay flagged for audit)
- `proton_redist_info` (`shared_redist_app_id: "228980"`, `shared_depots`, `common_redist_dirs`,
  `install_scripts`, and `proton_notes`)
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
   records and Steamworks/Proton-adjacent redist metadata for `bundle-verify`.
