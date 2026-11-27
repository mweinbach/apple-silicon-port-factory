"""Epic Games Store backend for `tools/port-acquire`, powered by `legendary`.

Wraps Heroic Games Launcher's open-source `legendary` CLI (`legendary-gl`) without
reimplementing the Epic Games protocol.

Security & policy invariants:
- Authentication uses `legendary`'s official interactive browser / authorization-code
  flow (`legendary auth`). Credentials stay in `legendary`'s own config directory.
- `status()` returns ONLY login state, account display name, and binary availability.
- `launch_args()` defaults to `--offline` and always redacts auth/token parameters.
- EOS, anti-cheat, and DRM signals are reported honestly (`report_only=True`) and
  are never bypassed or modified.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
from typing import Any

from ..backend import (
    AcquireError,
    AnticheatReport,
    AppNotFoundError,
    AuthRequiredError,
    AuthStatus,
    BackendUnavailableError,
    CommandFailedError,
    DlcItem,
    DownloadOptions,
    DownloadResult,
    DrmReport,
    GameInfo,
    LaunchPlan,
    OwnedGame,
    RedistributableItem,
    StoreBackend,
    UninstallResult,
    UpdateResult,
    VerifyResult,
    classify_redistributable_path,
    scan_dir_redistributables,
)
from ..manifest import redact_env, redact_launch_args

_HEROIC_LEGENDARY_CANDIDATES = (
    Path("/Applications/Heroic.app/Contents/Resources/app.asar.unpacked/build/bin/arm64/darwin/legendary"),
    Path("/Applications/Heroic.app/Contents/Resources/app.asar.unpacked/build/bin/x64/darwin/legendary"),
)

_AUTH_ERROR_PATTERNS = (
    "no saved credentials",
    "not logged in",
    "login failed",
    "authentication required",
    "invalid refresh token",
    "invalid_grant",
    "please authenticate",
    "run \"legendary auth\"",
    "run 'legendary auth'",
)

_NOT_FOUND_PATTERNS = (
    "could not find",
    "not installed",
    "is not installed",
    "no such app",
    "game not found",
    "not in library",
    "not found in",
)

_VERIFY_SUCCESS_PATTERNS = (
    "verification was successful",
    "verification finished successfully",
    "verification succeeded",
    "verification complete",
    "all files verified",
)

_INSTALL_FAILURE_PATTERNS = (
    "installation failed",
    "download failed",
    "failed to install",
    "failed to download",
    "aborting",
    "[cli] error:",
    "[dlm] error:",
    "[core] error:",
)

_INSTALL_SUCCESS_PATTERNS = (
    "finished installation process",
    "finished downloading",
    "download finished",
    "installation finished",
    "already up to date",
    "is up to date",
    "nothing to download",
    "finished installing",
)

_EOS_OVERLAY_SUCCESS_PATTERNS = (
    "overlay has been downloaded",
    "overlay has been installed",
    "overlay is already up to date",
    "finished installation process",
    "finished downloading",
)

_UNINSTALL_FAILURE_PATTERNS = (
    "removing game failed",
    "failed to uninstall",
    "failed to remove",
    "could not acquire lock",
    "failed to acquire lock",
    "unable to acquire lock",
    "lockfile",
    "[cli] error:",
    "[core] error:",
)

_UNINSTALL_SUCCESS_PATTERNS = (
    "game has been uninstalled",
    "has been uninstalled",
    "uninstalled successfully",
    "removed from legendary",
)

_ANTICHEAT_RULES: tuple[tuple[str, tuple[str, ...]], ...] = (
    (
        "Easy Anti-Cheat (EAC)",
        (
            "easyanticheat",
            "eac_launcher",
            "start_protected_game.exe",
            "anticheat_launcher",
            "easyanticheat_eos",
        ),
    ),
    (
        "BattlEye",
        ("battleye", "beservice", "beclient"),
    ),
    (
        "Denuvo Anti-Cheat",
        ("denuvo-anti-cheat", "denuvo_anticheat", "denuvoanticheat"),
    ),
    (
        "Riot Vanguard",
        ("vgc.exe", "vgk.sys", "vanguard"),
    ),
    (
        "RICOCHET Anti-Cheat",
        ("randgrid.sys", "ricochet"),
    ),
    (
        "nProtect GameGuard",
        ("gameguard", "npgg"),
    ),
)

_EOS_FILE_TOKENS = (
    "eossdk",
    "eosoverlay",
    "epic_online_services",
    "onlinesubsystemeos",
    "eossdk-win64-shipping.dll",
)

_THIRD_PARTY_LAUNCHERS: tuple[tuple[str, tuple[str, ...]], ...] = (
    ("Ubisoft Connect", ("uplay", "ubisoft", "upc.exe")),
    ("EA App / Origin", ("origin", "eadm", "eaapp", "link2ea")),
    ("Rockstar Games Launcher", ("rockstar", "socialclub")),
    ("2K Launcher", ("2klauncher", "launcherpatcher")),
)


def _platform_to_legendary(platform: str) -> str | None:
    low = platform.strip().lower()
    if low in ("windows", "win", "win64"):
        return "Windows"
    if low in ("macos", "mac", "darwin", "osx"):
        return "Mac"
    if low == "all":
        return None
    return platform


def _normalize_platform_name(raw: str) -> str:
    low = raw.strip().lower()
    if low in ("windows", "win32", "win64"):
        return "windows"
    if low in ("mac", "macos", "darwin"):
        return "macos"
    return low


def _parse_bool_attr(val: Any) -> bool | None:
    if isinstance(val, bool):
        return val
    if isinstance(val, dict) and "value" in val:
        return _parse_bool_attr(val["value"])
    if isinstance(val, str):
        low = val.strip().lower()
        if low in ("true", "1", "yes"):
            return True
        if low in ("false", "0", "no"):
            return False
    return None


def _extract_release_app_id(entries: Any, *, platform: str = "Windows") -> str:
    """Extract an installable Epic `appId` from Legendary's `installable` or `releaseInfo` array."""
    if not isinstance(entries, list):
        return ""
    want = platform.strip().lower()
    fallback = ""
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        cand = str(entry.get("appId") or entry.get("app_name") or "").strip()
        if not cand:
            continue
        if not fallback:
            fallback = cand
        plats = entry.get("platform")
        if isinstance(plats, list) and any(str(p).strip().lower() == want for p in plats):
            return cand
    return fallback


def _resolve_dlc_entry(d: dict[str, Any], *, platform: str = "Windows") -> tuple[str, bool, str | None]:
    """Return `(app_id, installable, catalog_id)` for a Legendary DLC entry.

    In Legendary `info --json`, entitlement-backed downloadable DLCs may have `app_name: null`
    with a non-empty `installable` release-info list containing `appId`, whereas truly
    non-installable entitlements have `installable: false` (or `[]`) and only a catalog `id`.
    """
    d_meta = d.get("metadata") if isinstance(d.get("metadata"), dict) else {}
    catalog_id = str(d.get("id") or d_meta.get("id") or "").strip() or None
    d_app = str(d.get("app_name") or "").strip()
    raw_installable = d.get("installable")

    if isinstance(raw_installable, list):
        if not d_app:
            d_app = _extract_release_app_id(raw_installable, platform=platform)
        installable = bool(raw_installable) and bool(d_app)
    elif raw_installable is False:
        installable = False
    else:
        if not d_app:
            d_app = _extract_release_app_id(d.get("releaseInfo"), platform=platform)
        installable = bool(d_app)
    return d_app, installable, catalog_id


def _parse_int(val: Any) -> int | None:
    if isinstance(val, bool):
        return None
    if isinstance(val, int):
        return val
    if isinstance(val, str) and val.strip().isdigit():
        return int(val.strip())
    return None


def _extract_build_version(
    *,
    game: dict[str, Any],
    manifest: dict[str, Any],
    install: dict[str, Any] | None = None,
    platform: str = "Windows",
) -> str | None:
    """Return the game build version from Legendary `info --json` data.

    In Legendary's schema, `manifest.version` is the manifest format revision (e.g. `18`),
    whereas `manifest.build_version` is the game's build version string.
    """
    build_ver = manifest.get("build_version")
    if isinstance(build_ver, str) and build_ver.strip():
        return build_ver.strip()

    asset_infos = game.get("asset_infos")
    if isinstance(asset_infos, dict):
        for plat in (platform, "Windows", "Mac", *asset_infos.keys()):
            ai = asset_infos.get(plat)
            if isinstance(ai, dict):
                bv = ai.get("build_version")
                if isinstance(bv, str) and bv.strip():
                    return bv.strip()

    for container, key in (
        (game, "app_version"),
        (game, "version"),
        (install or {}, "version"),
    ):
        val = container.get(key)
        if isinstance(val, str) and val.strip():
            return val.strip()
    return None


def _sanitize_error_text(text: str, *, secrets: tuple[str, ...] = ()) -> str:
    out = text.strip()
    for secret in secrets:
        if secret:
            out = out.replace(secret, "<redacted>")
    out = re.sub(
        r"(?i)(access_token|refresh_token|exchange_code|authorization_code|password|sid)\s*[:=]\s*\S+",
        r"\1=<redacted>",
        out,
    )
    return out


def _extract_json_payload(stdout: str) -> Any:
    text = stdout.strip()
    if not text:
        raise CommandFailedError("legendary returned empty JSON output")
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        pass

    # Fallback: skip leading non-JSON log lines (such as `[Core] WARNING: ...`)
    # by attempting raw_decode from each line that starts with `{` or `[`.
    decoder = json.JSONDecoder()
    pos = 0
    for line in text.splitlines(keepends=True):
        stripped = line.lstrip()
        if stripped.startswith(("{", "[")):
            offset = pos + (len(line) - len(stripped))
            try:
                obj, _ = decoder.raw_decode(text, offset)
                return obj
            except json.JSONDecodeError:
                pass
        pos += len(line)
    raise CommandFailedError(
        "failed to parse JSON output from legendary",
        details={"stdout_preview": _sanitize_error_text(text[:300])},
    )


def find_legendary(explicit: str | Path | None = None) -> Path:
    """Locate the `legendary` CLI binary (env override, PATH, Homebrew/pip, or Heroic)."""
    if explicit is not None:
        p = Path(explicit).expanduser()
        if p.is_file():
            return p
        raise BackendUnavailableError(
            f"configured legendary binary does not exist: {p}",
            details={"path": str(p)},
        )

    env_path = os.environ.get("PORT_LEGENDARY", "").strip()
    if env_path:
        p = Path(env_path).expanduser()
        if p.is_file():
            return p
        raise BackendUnavailableError(
            f"PORT_LEGENDARY points to a missing file: {p}",
            details={"env": "PORT_LEGENDARY", "path": str(p)},
        )

    on_path = shutil.which("legendary")
    if on_path:
        return Path(on_path)

    home = Path.home()
    candidates: list[Path] = [
        home / ".local/bin/legendary",
        Path("/opt/homebrew/bin/legendary"),
        Path("/usr/local/bin/legendary"),
        *_HEROIC_LEGENDARY_CANDIDATES,
        home
        / "Applications/Heroic.app/Contents/Resources/app.asar.unpacked/build/bin/arm64/darwin/legendary",
        home
        / "Applications/Heroic.app/Contents/Resources/app.asar.unpacked/build/bin/x64/darwin/legendary",
    ]
    for cand in candidates:
        if cand.is_file():
            return cand

    raise BackendUnavailableError(
        "legendary CLI not found. Install it via `pipx install legendary-gl` or "
        "`pip3 install legendary-gl` (or Heroic Games Launcher), or set PORT_LEGENDARY=/path/to/legendary."
    )


class EpicBackend(StoreBackend):
    """Epic Games Store backend powered by Heroic's `legendary` CLI."""

    name = "epic"
    display_name = "Epic Games Store (legendary)"

    def __init__(
        self,
        *,
        legendary_bin: str | Path | None = None,
        config_dir: str | Path | None = None,
    ) -> None:
        self._explicit_bin = Path(legendary_bin) if legendary_bin is not None else None
        self._explicit_config_dir = Path(config_dir) if config_dir is not None else None

    def _binary(self) -> Path:
        return find_legendary(self._explicit_bin)

    def _env(self) -> dict[str, str]:
        env = dict(os.environ)
        cfg = self._explicit_config_dir
        if cfg is None:
            env_cfg = os.environ.get("PORT_LEGENDARY_CONFIG", "").strip()
            if env_cfg:
                cfg = Path(env_cfg).expanduser()
        if cfg is not None:
            env["LEGENDARY_CONFIG_PATH"] = str(cfg)
        # Prevent PyInstaller/macOS locale ValueError warnings when LANG lacks a territory.
        lang = env.get("LANG", "")
        if not lang or "_" not in lang:
            env["LANG"] = "en_US.UTF-8"
        return env

    def _run(
        self,
        args: list[str],
        *,
        check: bool = True,
        secrets: tuple[str, ...] = (),
    ) -> subprocess.CompletedProcess[str]:
        binary = self._binary()
        cmd = [str(binary), *args]
        try:
            proc = subprocess.run(
                cmd,
                env=self._env(),
                capture_output=True,
                text=True,
                check=False,
            )
        except OSError as exc:
            raise BackendUnavailableError(
                f"failed to execute legendary ({binary}): {exc}"
            ) from exc

        if check and proc.returncode != 0:
            combined = f"{proc.stderr}\n{proc.stdout}".lower()
            sanitized = _sanitize_error_text(
                proc.stderr or proc.stdout or f"exit code {proc.returncode}",
                secrets=secrets,
            )
            if any(pat in combined for pat in _AUTH_ERROR_PATTERNS):
                raise AuthRequiredError(
                    "Epic Games Store authentication required. Run `tools/port-acquire login --store epic` interactively.",
                    details={"backend_message": sanitized},
                )
            if any(pat in combined for pat in _NOT_FOUND_PATTERNS):
                raise AppNotFoundError(
                    f"Epic app not found or not installed: {sanitized}",
                    details={"returncode": proc.returncode},
                )
            raise CommandFailedError(
                f"legendary {' '.join(args[:2])} failed (exit {proc.returncode}): {sanitized}",
                details={"returncode": proc.returncode},
            )
        return proc

    def _run_json(self, args: list[str]) -> Any:
        proc = self._run(args, check=True)
        return _extract_json_payload(proc.stdout)

    def _version(self) -> str | None:
        try:
            proc = self._run(["--version"], check=False)
        except BackendUnavailableError:
            return None
        if proc.returncode != 0:
            return None
        line = (proc.stdout or proc.stderr).strip().splitlines()
        return line[0].strip() if line else None

    def _installed_map(self) -> dict[str, dict[str, Any]]:
        try:
            data = self._run_json(["list-installed", "--json"])
        except AcquireError:
            return {}
        if not isinstance(data, list):
            return {}
        out: dict[str, dict[str, Any]] = {}
        for item in data:
            if isinstance(item, dict) and item.get("app_name"):
                out[str(item["app_name"])] = item
        return out

    # ------------------------------------------------------------------
    # StoreBackend implementation
    # ------------------------------------------------------------------

    def login(
        self,
        *,
        no_webview: bool = False,
        auth_code: str | None = None,
        import_existing: bool = False,
        logout: bool = False,
    ) -> AuthStatus:
        binary = self._binary()
        if logout:
            self._run(["auth", "--delete"], check=True)
            return self.status(offline=True)

        if not auth_code and not import_existing and not sys.stdin.isatty():
            raise AuthRequiredError(
                "Epic login requires an interactive terminal run by the account owner. "
                "Run: tools/port-acquire login --store epic "
                "(or `tools/port-acquire login --store epic --no-webview` to paste the official authorization code)."
            )

        cmd = [str(binary), "auth"]
        if no_webview:
            cmd.append("--disable-webview")
        if import_existing:
            cmd.append("--import")
        if auth_code:
            cmd.extend(["--code", auth_code])

        # Inherit stdio for interactive browser/code flow; never capture or log auth_code.
        try:
            if auth_code:
                proc = subprocess.run(
                    cmd,
                    env=self._env(),
                    capture_output=True,
                    text=True,
                    check=False,
                )
            else:
                proc = subprocess.run(cmd, env=self._env(), check=False)
        except OSError as exc:
            raise BackendUnavailableError(f"failed to execute legendary auth: {exc}") from exc

        if proc.returncode != 0:
            msg = ""
            if auth_code and getattr(proc, "stderr", None):
                msg = ": " + _sanitize_error_text(proc.stderr, secrets=(auth_code,))
            raise CommandFailedError(
                f"legendary auth failed (exit {proc.returncode}){msg}",
                details={"returncode": proc.returncode},
            )
        return self.status(offline=True)

    def status(self, *, offline: bool = False) -> AuthStatus:
        try:
            self._binary()
        except BackendUnavailableError:
            return AuthStatus(
                store=self.name,
                logged_in=False,
                display_name=None,
                backend_available=False,
                backend_version=None,
            )

        version = self._version()
        args = ["status", "--json"]
        if offline:
            args.insert(1, "--offline")
        data = self._run_json(args)
        if not isinstance(data, dict):
            raise CommandFailedError("unexpected non-object JSON from `legendary status --json`")

        raw_account = data.get("account")
        if isinstance(raw_account, str):
            acct = raw_account.strip()
            if not acct or acct.lower() == "<not logged in>":
                logged_in = False
                display_name = None
            else:
                logged_in = True
                display_name = acct
        else:
            logged_in = False
            display_name = None

        # Security invariant: return ONLY login state, display name, and binary metadata.
        return AuthStatus(
            store=self.name,
            logged_in=logged_in,
            display_name=display_name,
            backend_available=True,
            backend_version=version,
        )

    def _fetch_raw_list(self, *, platform: str, refresh: bool) -> list[dict[str, Any]]:
        leg_plat = _platform_to_legendary(platform)
        if leg_plat is None:
            merged: dict[str, dict[str, Any]] = {}
            for plat in ("Windows", "Mac"):
                cmd = ["list", "--platform", plat, "--json"]
                if refresh:
                    cmd.append("--force-refresh")
                try:
                    rows = self._run_json(cmd)
                except AppNotFoundError:
                    rows = []
                if isinstance(rows, list):
                    for row in rows:
                        if isinstance(row, dict) and row.get("app_name"):
                            key = str(row["app_name"])
                            if key not in merged:
                                merged[key] = row
                            else:
                                # Merge asset_infos platforms
                                existing_ai = dict(merged[key].get("asset_infos") or {})
                                existing_ai.update(row.get("asset_infos") or {})
                                merged[key]["asset_infos"] = existing_ai
            return list(merged.values())

        cmd = ["list", "--platform", leg_plat, "--json"]
        if refresh:
            cmd.append("--force-refresh")
        rows = self._run_json(cmd)
        if not isinstance(rows, list):
            raise CommandFailedError("unexpected non-list JSON from `legendary list --json`")
        return [r for r in rows if isinstance(r, dict)]

    def list_owned(self, *, platform: str = "windows", refresh: bool = False) -> list[OwnedGame]:
        rows = self._fetch_raw_list(platform=platform, refresh=refresh)
        installed_map = self._installed_map()

        # First pass: index base games by catalog item id and collect DLC rows.
        by_catalog_id: dict[str, OwnedGame] = {}
        base_games: list[OwnedGame] = []
        dlc_rows: list[tuple[str | None, DlcItem]] = []

        for row in rows:
            app_name = str(row.get("app_name") or "").strip()
            if not app_name:
                continue
            title = str(row.get("app_title") or row.get("title") or app_name).strip()
            metadata = row.get("metadata") if isinstance(row.get("metadata"), dict) else {}
            asset_infos = row.get("asset_infos") if isinstance(row.get("asset_infos"), dict) else {}

            platforms = sorted({_normalize_platform_name(k) for k in asset_infos.keys()})
            if not platforms and platform != "all":
                platforms = [platform.lower()]

            version: str | None = None
            for pref in ("Windows", "Mac"):
                ai = asset_infos.get(pref)
                if isinstance(ai, dict) and ai.get("build_version"):
                    version = str(ai["build_version"])
                    break
            if version is None and row.get("app_version"):
                version = str(row["app_version"])

            inst = installed_map.get(app_name)
            is_installed = inst is not None
            install_path = str(inst["install_path"]) if inst and inst.get("install_path") else None

            main_game_item = metadata.get("mainGameItem")
            if isinstance(main_game_item, dict) and main_game_item.get("id"):
                parent_id = str(main_game_item["id"])
                dlc_rows.append(
                    (
                        parent_id,
                        DlcItem(
                            app_id=app_name,
                            title=title,
                            owned=True,
                            installed=is_installed,
                            install_path=install_path,
                            disk_size_bytes=_parse_int(inst.get("install_size")) if inst else None,
                        ),
                    )
                )
                continue

            inline_dlcs: list[DlcItem] = []
            seen_inline_dlcs: set[tuple[str, str | None]] = set()
            # Real Legendary `list --json` nests add-ons under `dlcs`; also accept
            # legacy/alternate `dlc_list` if present.
            raw_dlcs: list[Any] = []
            for key in ("dlcs", "dlc_list"):
                val = row.get(key)
                if isinstance(val, list):
                    raw_dlcs.extend(val)
            for d in raw_dlcs:
                if not isinstance(d, dict):
                    continue
                d_app, installable, catalog_id = _resolve_dlc_entry(
                    d, platform=_platform_to_legendary(platform) or "Windows"
                )
                if not d_app and not catalog_id:
                    continue
                dedup_key = (d_app, None if d_app else catalog_id)
                if dedup_key in seen_inline_dlcs:
                    continue
                seen_inline_dlcs.add(dedup_key)
                d_inst = installed_map.get(d_app) if d_app else None
                inline_dlcs.append(
                    DlcItem(
                        app_id=d_app,
                        title=str(d.get("app_title") or d.get("title") or d_app or catalog_id).strip(),
                        owned=True,
                        installable=installable,
                        catalog_id=catalog_id,
                        installed=d_inst is not None,
                        install_path=(
                            str(d_inst["install_path"])
                            if d_inst and d_inst.get("install_path")
                            else None
                        ),
                        download_size_bytes=_parse_int(d.get("download_size")),
                        disk_size_bytes=(
                            _parse_int(d.get("disk_size"))
                            or (_parse_int(d_inst.get("install_size")) if d_inst else None)
                        ),
                    )
                )

            owned = OwnedGame(
                store=self.name,
                app_id=app_name,
                title=title,
                platforms=platforms,
                version=version,
                installed=is_installed,
                install_path=install_path,
                dlcs=inline_dlcs,
            )
            base_games.append(owned)
            cat_id = metadata.get("id")
            if cat_id:
                by_catalog_id[str(cat_id)] = owned

        for parent_id, dlc in dlc_rows:
            parent = by_catalog_id.get(parent_id or "")
            if parent is not None:
                if not any(existing.app_id == dlc.app_id for existing in parent.dlcs):
                    parent.dlcs.append(dlc)

        base_games.sort(key=lambda g: (g.title.lower(), g.app_id.lower()))
        return base_games

    def _list_manifest_files(self, app_id: str, *, platform: str) -> list[str] | None:
        leg_plat = _platform_to_legendary(platform) or "Windows"
        try:
            data = self._run_json(["list-files", app_id, "--platform", leg_plat, "--json"])
        except AcquireError:
            return None
        if not isinstance(data, list):
            return None
        paths: list[str] = []
        for entry in data:
            if isinstance(entry, str):
                paths.append(entry)
            elif isinstance(entry, dict):
                fname = entry.get("filename") or entry.get("path") or entry.get("name")
                if isinstance(fname, str):
                    paths.append(fname)
        return paths

    def _analyze_compatibility(
        self,
        *,
        game: dict[str, Any],
        manifest: dict[str, Any],
        install: dict[str, Any],
        manifest_files: list[str] | None,
    ) -> tuple[bool | None, bool, list[str], AnticheatReport, DrmReport, list[RedistributableItem]]:
        metadata = game.get("metadata") if isinstance(game.get("metadata"), dict) else {}
        custom = (
            metadata.get("customAttributes")
            if isinstance(metadata.get("customAttributes"), dict)
            else {}
        )

        # Offline capability & ownership token
        can_run_offline = _parse_bool_attr(game.get("can_run_offline"))
        if can_run_offline is None:
            can_run_offline = _parse_bool_attr(custom.get("CanRunOffline"))
        if can_run_offline is None:
            can_run_offline = _parse_bool_attr(install.get("can_run_offline"))

        requires_ot = bool(game.get("requires_ot") or install.get("requires_ot"))
        if _parse_bool_attr(custom.get("UseAccessControl")) is True:
            requires_ot = True

        eos_signals: list[str] = []
        anticheat_systems: list[str] = []
        anticheat_signals: list[str] = []
        drm_signals: list[str] = []
        third_party_launcher: str | None = None

        if _parse_bool_attr(custom.get("MonitorPresence")) is True or custom.get("PresenceId"):
            eos_signals.append("metadata:PresenceId/MonitorPresence")
        if custom.get("EOSDeploymentId") or custom.get("EosDeploymentId"):
            eos_signals.append("metadata:EOSDeploymentId")
        if requires_ot:
            eos_signals.append("metadata:requires_ownership_token")
            drm_signals.append("requires_ownership_token")
        if can_run_offline is False:
            drm_signals.append("CanRunOffline=false")

        # Real Legendary `info --json` exposes external launchers via `game.external_activation`
        # (and raw library rows may expose `customAttributes.ThirdPartyManagedApp` / `PartnerLinkType`).
        ext_activation = game.get("external_activation")
        if isinstance(ext_activation, str) and ext_activation.strip() and ext_activation.strip().lower() not in ("none", "false"):
            third_party_launcher = ext_activation.strip()
            drm_signals.append(f"external_activation:{third_party_launcher}")
        else:
            third_party_attr = custom.get("ThirdPartyManagedApp") or custom.get("PartnerLinkType")
            if isinstance(third_party_attr, dict):
                third_party_attr = third_party_attr.get("value")
            if isinstance(third_party_attr, str) and third_party_attr.strip():
                third_party_launcher = third_party_attr.strip()
                drm_signals.append(f"third_party_launcher:{third_party_launcher}")

        # Redistributables from Epic prereq_info / manifest.prerequisites
        redists: list[RedistributableItem] = []
        seen_redists: set[tuple[str, str | None]] = set()

        def add_redist(item: RedistributableItem) -> None:
            key = (item.id, item.relative_path)
            if key not in seen_redists:
                seen_redists.add(key)
                redists.append(item)

        prereqs = manifest.get("prerequisites") or install.get("prereq_info")
        if isinstance(prereqs, dict) and (prereqs.get("path") or prereqs.get("name")):
            p_path = str(prereqs.get("path") or "").replace("\\", "/") or None
            p_name = str(prereqs.get("name") or (Path(p_path).name if p_path else "Prerequisite"))
            p_args = str(prereqs.get("args") or "")
            classified = classify_redistributable_path(p_path, source="prereq_info") if p_path else None
            if classified is not None:
                classified.name = p_name or classified.name
                classified.args = p_args
                add_redist(classified)
            else:
                low_name = p_name.lower()
                if "visual c++" in low_name or "vcredist" in low_name or "vc_redist" in low_name:
                    cat, native = "vcredist", True
                    note = "Satisfied natively by crates/win32-rt UCRT/MSVC runtime"
                elif "directx" in low_name:
                    cat, native = "directx", True
                    note = "Satisfied natively by crates/win32-rt D3D11/D3D12 + crates/d3d12-metal"
                elif "epic online services" in low_name or "eos" in low_name:
                    cat, native = "eos_sdk", False
                    note = "Epic Online Services prerequisite declared by manifest"
                    eos_signals.append(f"prereq:{p_name}")
                else:
                    cat, native = "other", False
                    note = "Declared in Epic manifest prerequisites"
                add_redist(
                    RedistributableItem(
                        id=f"prereq:{re.sub(r'[^a-z0-9]+', '_', p_name.lower()).strip('_')}",
                        name=p_name,
                        category=cat,
                        source="prereq_info",
                        relative_path=p_path,
                        args=p_args,
                        native_in_win32_rt=native,
                        notes=note,
                    )
                )

        # Inspect manifest file list (if available) for EOS, anti-cheat, DRM, redistributables
        if manifest_files:
            for raw_f in manifest_files:
                norm = raw_f.replace("\\", "/")
                low = norm.lower()

                if any(tok in low for tok in _EOS_FILE_TOKENS):
                    sig = f"manifest_file:{Path(norm).name}"
                    if sig not in eos_signals:
                        eos_signals.append(sig)

                for ac_name, tokens in _ANTICHEAT_RULES:
                    if any(tok in low for tok in tokens):
                        if ac_name not in anticheat_systems:
                            anticheat_systems.append(ac_name)
                        sig = f"manifest_file:{norm}"
                        if len(anticheat_signals) < 12 and sig not in anticheat_signals:
                            anticheat_signals.append(sig)

                if "denuvo" in low and "anti" not in low:
                    if "manifest_file:denuvo" not in drm_signals:
                        drm_signals.append("manifest_file:denuvo")

                if third_party_launcher is None:
                    for launcher_name, tokens in _THIRD_PARTY_LAUNCHERS:
                        if any(tok in low for tok in tokens):
                            third_party_launcher = launcher_name
                            drm_signals.append(f"manifest_file:{launcher_name}")
                            break

                redist = classify_redistributable_path(norm, source="manifest_files")
                if redist is not None:
                    add_redist(redist)

        anticheat = AnticheatReport(
            detected=bool(anticheat_systems),
            systems=anticheat_systems,
            signals=anticheat_signals,
            report_only=True,
        )
        drm = DrmReport(
            detected=bool(
                (can_run_offline is False)
                or requires_ot
                or third_party_launcher is not None
                or bool(drm_signals)
            ),
            requires_online_auth=(can_run_offline is False),
            requires_ownership_token=requires_ot,
            third_party_launcher=third_party_launcher,
            signals=drm_signals,
            report_only=True,
        )
        requires_eos = bool(eos_signals)
        return can_run_offline, requires_eos, eos_signals, anticheat, drm, redists

    def info(self, app_id: str, *, platform: str = "windows", offline: bool = False) -> GameInfo:
        leg_plat = _platform_to_legendary(platform) or "Windows"
        cmd = ["info", app_id, "--platform", leg_plat, "--json"]
        if offline:
            cmd.append("--offline")
        data = self._run_json(cmd)
        if not isinstance(data, dict):
            raise CommandFailedError("unexpected non-object JSON from `legendary info --json`")

        game = data.get("game") if isinstance(data.get("game"), dict) else data
        manifest = data.get("manifest") if isinstance(data.get("manifest"), dict) else {}
        install = data.get("install") if isinstance(data.get("install"), dict) else {}

        resolved_app_id = str(game.get("app_name") or app_id)
        title = str(game.get("title") or game.get("app_title") or resolved_app_id)
        version = _extract_build_version(
            game=game, manifest=manifest, install=install, platform=leg_plat
        )
        download_size = _parse_int(manifest.get("download_size"))
        disk_size = _parse_int(manifest.get("disk_size")) or _parse_int(install.get("install_size"))

        installed_map = self._installed_map()
        inst_entry = installed_map.get(resolved_app_id) or (install if install.get("install_path") else None)
        installed = inst_entry is not None
        install_path = (
            str(inst_entry["install_path"])
            if inst_entry and inst_entry.get("install_path")
            else None
        )

        executable = manifest.get("launch_exe") or (
            inst_entry.get("executable") if inst_entry else None
        )
        if isinstance(executable, str):
            executable = executable.replace("\\", "/")
        else:
            executable = None

        manifest_files = None if offline else self._list_manifest_files(resolved_app_id, platform=platform)

        (
            can_run_offline,
            requires_eos,
            eos_signals,
            anticheat,
            drm,
            redists,
        ) = self._analyze_compatibility(
            game=game,
            manifest=manifest,
            install=inst_entry or {},
            manifest_files=manifest_files,
        )

        # Merge on-disk redistributable scan if already installed
        if install_path and Path(install_path).is_dir():
            seen = {(r.id, r.relative_path) for r in redists}
            for item in scan_dir_redistributables(Path(install_path)):
                if (item.id, item.relative_path) not in seen:
                    seen.add((item.id, item.relative_path))
                    redists.append(item)

        # Collect DLCs from `game.owned_dlc` / `game.dlcs` / `game.dlc_list` / `metadata.dlcItemList`
        dlcs: list[DlcItem] = []
        seen_dlcs: set[tuple[str, str | None]] = set()
        dlc_sources = []
        for key in ("owned_dlc", "dlcs", "dlc_list"):
            if isinstance(game.get(key), list):
                dlc_sources.extend(game[key])
        metadata = game.get("metadata") if isinstance(game.get("metadata"), dict) else {}
        if isinstance(metadata.get("dlcItemList"), list):
            dlc_sources.extend(metadata["dlcItemList"])

        for d in dlc_sources:
            if not isinstance(d, dict):
                continue
            d_app, installable, catalog_id = _resolve_dlc_entry(d, platform=leg_plat)
            if not d_app and not catalog_id:
                continue
            dedup_key = (d_app, None if d_app else catalog_id)
            if dedup_key in seen_dlcs:
                continue
            seen_dlcs.add(dedup_key)
            d_inst = installed_map.get(d_app) if d_app else None
            dlcs.append(
                DlcItem(
                    app_id=d_app,
                    title=str(d.get("app_title") or d.get("title") or d_app or catalog_id).strip(),
                    owned=bool(d.get("owned", True)),
                    installable=installable,
                    catalog_id=catalog_id,
                    installed=d_inst is not None,
                    install_path=(
                        str(d_inst["install_path"])
                        if d_inst and d_inst.get("install_path")
                        else None
                    ),
                    download_size_bytes=_parse_int(d.get("download_size")),
                    disk_size_bytes=_parse_int(d.get("disk_size"))
                    or (_parse_int(d_inst.get("install_size")) if d_inst else None),
                )
            )

        raw_launch_cmd = manifest.get("launch_command")
        default_launch_args: list[str] = []
        if isinstance(raw_launch_cmd, str) and raw_launch_cmd.strip():
            try:
                parsed_args = shlex.split(raw_launch_cmd)
            except ValueError:
                parsed_args = raw_launch_cmd.split()
            default_launch_args, _ = redact_launch_args(parsed_args)

        return GameInfo(
            store=self.name,
            app_id=resolved_app_id,
            title=title,
            platform=platform.lower(),
            version=version,
            download_size_bytes=download_size,
            disk_size_bytes=disk_size,
            installed=installed,
            install_path=install_path,
            executable=executable,
            can_run_offline=can_run_offline,
            requires_eos=requires_eos,
            eos_signals=eos_signals,
            anticheat=anticheat,
            drm=drm,
            dlcs=dlcs,
            redistributables=redists,
            manifest_files_checked=manifest_files is not None,
            default_launch_args=default_launch_args,
        )

    def _assert_install_completed(
        self,
        proc: subprocess.CompletedProcess[str],
        *,
        app_id: str,
        action: str,
        success_patterns: tuple[str, ...] = _INSTALL_SUCCESS_PATTERNS,
    ) -> None:
        """Confirm Legendary actually completed an install/download/repair/update.

        Real Legendary catches downloader exceptions, logs `Installation failed`, and
        exits 0; treat exit code 0 without positive completion output (or with any error
        banner) as a failure rather than silently returning success.
        """
        combined = f"{proc.stdout}\n{proc.stderr}"
        combined_lower = combined.lower()
        sanitized = _sanitize_error_text(combined)

        if any(pat in combined_lower for pat in _AUTH_ERROR_PATTERNS):
            raise AuthRequiredError(
                "Epic Games Store authentication required. Run `tools/port-acquire login --store epic` interactively.",
                details={"backend_message": sanitized},
            )
        if any(pat in combined_lower for pat in _NOT_FOUND_PATTERNS):
            raise AppNotFoundError(
                f"app {app_id!r} not found or not installed: {sanitized}",
                details={"returncode": proc.returncode},
            )

        has_error_line = any(
            line.lstrip().lower().startswith("error:")
            or any(pat in line.lower() for pat in _INSTALL_FAILURE_PATTERNS)
            for line in combined.splitlines()
        )
        completed_ok = any(pat in combined_lower for pat in success_patterns)

        if proc.returncode != 0 or has_error_line or not completed_ok:
            reason = sanitized or f"exit {proc.returncode} with no completion confirmation"
            raise CommandFailedError(
                f"legendary {action} for {app_id!r} did not complete cleanly: {reason}",
                details={"returncode": proc.returncode},
            )

    def download(self, options: DownloadOptions) -> DownloadResult:
        leg_plat = _platform_to_legendary(options.platform) or "Windows"
        dest = options.dest.expanduser().resolve()

        # Legendary's `--download-only` maps to `no_install=True`, which skips its built-in
        # `--with-dlcs` branch. When both are requested, download the base game with
        # `--skip-dlcs` and then download each owned, installable DLC explicitly into `dest`.
        explicit_dlcs_after_base: list[str] = []
        if options.with_dlcs and options.no_register:
            explicit_dlcs_after_base = [
                d.app_id
                for d in self.list_dlcs(options.app_id, platform=options.platform)
                if d.owned and d.installable and d.app_id
            ]
        elif not options.with_dlcs and options.dlcs:
            explicit_dlcs_after_base = list(options.dlcs)

        # Real Legendary reuses `igame.install_path` before `--game-folder` whenever an app is
        # already registered in `installed.json` (even under `--download-only`). Reject a
        # conflicting `--dest` up front so we never silently modify files outside `--dest`.
        existing_installed = self._installed_map()
        for target_id in (options.app_id, *explicit_dlcs_after_base):
            existing = existing_installed.get(target_id)
            if existing and existing.get("install_path"):
                existing_path = Path(str(existing["install_path"])).expanduser().resolve()
                if existing_path != dest:
                    raise CommandFailedError(
                        f"Epic app {target_id!r} is already registered in Legendary at "
                        f"{existing_path}, which conflicts with requested --dest {dest}. "
                        f"Legendary reuses the registered install path instead of --game-folder. "
                        f"Use --dest {existing_path!s}, run `tools/port-acquire update {target_id}`, "
                        f"uninstall/move the existing registration, or set PORT_LEGENDARY_CONFIG "
                        f"to an isolated config directory for a separate download.",
                        code="destination_conflict",
                        details={
                            "app_id": target_id,
                            "requested_dest": str(dest),
                            "existing_install_path": str(existing_path),
                        },
                    )

        dest.mkdir(parents=True, exist_ok=True)

        cmd = [
            "-y",
            "install",
            options.app_id,
            "--platform",
            leg_plat,
            "--game-folder",
            str(dest),
            "--skip-sdl",
        ]
        if options.with_dlcs and not options.no_register:
            cmd.append("--with-dlcs")
        else:
            cmd.append("--skip-dlcs")
        if options.no_register:
            cmd.append("--download-only")
        if options.max_workers is not None:
            cmd.extend(["--max-workers", str(options.max_workers)])
        cmd.extend(options.extra_args)

        proc = self._run(cmd, check=False)
        self._assert_install_completed(proc, app_id=options.app_id, action="download")

        downloaded_dlcs: list[str] = []
        for dlc_id in explicit_dlcs_after_base:
            dlc_cmd = [
                "-y",
                "install",
                dlc_id,
                "--platform",
                leg_plat,
                "--game-folder",
                str(dest),
                "--skip-dlcs",
                "--skip-sdl",
            ]
            if options.no_register:
                dlc_cmd.append("--download-only")
            if options.max_workers is not None:
                dlc_cmd.extend(["--max-workers", str(options.max_workers)])
            dlc_proc = self._run(dlc_cmd, check=False)
            self._assert_install_completed(dlc_proc, app_id=dlc_id, action="dlc download")
            downloaded_dlcs.append(dlc_id)

        eos_overlay_path: str | None = None
        if options.with_eos_overlay:
            overlay_dir = dest / ".port-acquire" / "eos-overlay"
            overlay_dir.mkdir(parents=True, exist_ok=True)
            ov_proc = self._run(
                ["-y", "eos-overlay", "install", "--path", str(overlay_dir)],
                check=False,
            )
            self._assert_install_completed(
                ov_proc,
                app_id=f"{options.app_id}:eos-overlay",
                action="eos-overlay install",
                success_patterns=_EOS_OVERLAY_SUCCESS_PATTERNS,
            )
            eos_overlay_path = str(overlay_dir)

        installed_map = self._installed_map()
        inst = installed_map.get(options.app_id)
        if not options.no_register and inst is None:
            raise CommandFailedError(
                f"legendary install for {options.app_id!r} did not register an installed entry",
                details={"app_id": options.app_id},
            )
        install_path = str(inst["install_path"]) if inst and inst.get("install_path") else str(dest)
        title = str(inst.get("title") or options.app_id) if inst else options.app_id
        version = str(inst["version"]) if inst and inst.get("version") else None

        if options.with_dlcs and not options.no_register:
            for d_id, d_entry in installed_map.items():
                if d_entry.get("is_dlc") and d_entry.get("install_path") == install_path:
                    downloaded_dlcs.append(d_id)

        return DownloadResult(
            store=self.name,
            app_id=options.app_id,
            title=title,
            platform=options.platform.lower(),
            install_path=install_path,
            version=version,
            downloaded_dlcs=sorted(set(downloaded_dlcs)),
            eos_overlay_path=eos_overlay_path,
        )

    def verify(self, app_id: str, *, repair: bool = False) -> VerifyResult:
        if repair:
            proc = self._run(["-y", "install", app_id, "--repair", "--skip-dlcs"], check=False)
            try:
                self._assert_install_completed(proc, app_id=app_id, action="repair")
            except (AppNotFoundError, AuthRequiredError):
                raise
            except CommandFailedError as exc:
                return VerifyResult(
                    store=self.name,
                    app_id=app_id,
                    ok=False,
                    repaired=False,
                    message=str(exc),
                )
            return VerifyResult(
                store=self.name,
                app_id=app_id,
                ok=True,
                repaired=True,
                message="Verification and repair completed.",
            )

        proc = self._run(["verify", app_id], check=False)
        combined = f"{proc.stdout}\n{proc.stderr}"
        combined_lower = combined.lower()
        # Real Legendary logs `Game "<app>" is not installed` and exits 0; always check
        # not-installed patterns regardless of exit status.
        if any(pat in combined_lower for pat in _NOT_FOUND_PATTERNS):
            raise AppNotFoundError(
                f"app {app_id!r} is not installed: {_sanitize_error_text(combined)}"
            )

        missing: list[str] = []
        corrupted: list[str] = []
        has_error_line = False
        for line in combined.splitlines():
            low = line.lower()
            if "missing" in low and ("file" in low or '"' in line or "'" in line):
                missing.append(line.strip())
            elif ("corrupt" in low or "mismatch" in low or "hash" in low) and "failed" in low:
                corrupted.append(line.strip())
            if "error:" in low:
                has_error_line = True

        completed_ok = any(pat in combined_lower for pat in _VERIFY_SUCCESS_PATTERNS)
        ok = (
            proc.returncode == 0
            and completed_ok
            and not has_error_line
            and not missing
            and not corrupted
        )
        if ok:
            msg = "All files verified cleanly."
        elif proc.returncode == 0 and not completed_ok and not combined.strip():
            msg = "legendary verify exited 0 without reporting verification completion."
        else:
            msg = _sanitize_error_text(
                combined.strip()
                or f"verify did not confirm completion (exit {proc.returncode})"
            )
        return VerifyResult(
            store=self.name,
            app_id=app_id,
            ok=ok,
            repaired=False,
            missing_files=missing,
            corrupted_files=corrupted,
            message=msg,
        )

    def update(
        self, app_id: str, *, check_only: bool = False, with_dlcs: bool = False
    ) -> UpdateResult:
        rows = self._run_json(["list-installed", "--check-updates", "--json"])
        if not isinstance(rows, list):
            raise CommandFailedError("unexpected JSON from `legendary list-installed --check-updates --json`")
        entry = next(
            (r for r in rows if isinstance(r, dict) and str(r.get("app_name")) == app_id),
            None,
        )
        if entry is None:
            raise AppNotFoundError(f"Epic app {app_id!r} is not installed")

        old_version = str(entry.get("version") or "") or None
        remote_version = str(entry.get("remote_version") or "") or None
        install_path = str(entry.get("install_path") or "") or None
        inst_platform = str(entry.get("platform") or "Windows")
        update_available = bool(
            entry.get("update_available")
            or (remote_version and old_version and remote_version != old_version)
        )
        # Some legendary builds omit `remote_version` in `list-installed --check-updates --json`;
        # cross-check against `info --json` `manifest.build_version` when available.
        if remote_version is None and not update_available:
            try:
                latest_info = self._run_json(
                    ["info", app_id, "--platform", inst_platform, "--json"]
                )
                if isinstance(latest_info, dict):
                    manifest = (
                        latest_info.get("manifest")
                        if isinstance(latest_info.get("manifest"), dict)
                        else {}
                    )
                    game_meta = (
                        latest_info.get("game")
                        if isinstance(latest_info.get("game"), dict)
                        else {}
                    )
                    cand_ver = _extract_build_version(
                        game=game_meta,
                        manifest=manifest,
                        install=None,
                        platform=inst_platform,
                    )
                    if cand_ver:
                        remote_version = cand_ver
                        if old_version and remote_version != old_version:
                            update_available = True
            except AcquireError:
                pass

        if check_only:
            return UpdateResult(
                store=self.name,
                app_id=app_id,
                updated=False,
                update_available=update_available,
                old_version=old_version,
                new_version=remote_version or old_version,
                install_path=install_path,
                message="Update available." if update_available else "Already up to date.",
            )

        cmd = ["-y", "install", app_id, "--update-only", "--skip-sdl"]
        cmd.append("--with-dlcs" if with_dlcs else "--skip-dlcs")
        upd_proc = self._run(cmd, check=False)
        self._assert_install_completed(upd_proc, app_id=app_id, action="update")

        after = self._installed_map().get(app_id) or {}
        new_version = str(after.get("version") or remote_version or old_version or "") or None
        changed = bool(old_version and new_version and new_version != old_version) or update_available
        return UpdateResult(
            store=self.name,
            app_id=app_id,
            updated=changed,
            update_available=False,
            old_version=old_version,
            new_version=new_version,
            install_path=str(after.get("install_path") or install_path or "") or None,
            message="Update applied." if changed else "Already up to date.",
        )

    def uninstall(
        self, app_id: str, *, keep_files: bool = False, skip_uninstaller: bool = False
    ) -> UninstallResult:
        existing = self._installed_map().get(app_id)
        install_path = (
            str(existing["install_path"]) if existing and existing.get("install_path") else None
        )
        cmd = ["-y", "uninstall", app_id]
        if keep_files:
            cmd.append("--keep-files")
        if skip_uninstaller:
            cmd.append("--skip-uninstaller")
        proc = self._run(cmd, check=False)
        combined = f"{proc.stdout}\n{proc.stderr}"
        combined_lower = combined.lower()
        sanitized = _sanitize_error_text(combined)
        if any(pat in combined_lower for pat in _NOT_FOUND_PATTERNS):
            raise AppNotFoundError(f"app {app_id!r} is not installed: {sanitized}")

        has_error_line = any(
            line.lstrip().lower().startswith("error:")
            or any(pat in line.lower() for pat in _UNINSTALL_FAILURE_PATTERNS)
            for line in combined.splitlines()
        )
        completed_ok = any(pat in combined_lower for pat in _UNINSTALL_SUCCESS_PATTERNS)
        still_installed = app_id in self._installed_map()
        if proc.returncode != 0 or has_error_line or not completed_ok or still_installed:
            reason = sanitized or f"exit {proc.returncode} with no uninstall completion confirmation"
            raise CommandFailedError(
                f"legendary uninstall for {app_id!r} did not complete cleanly: {reason}",
                details={"returncode": proc.returncode, "still_installed": still_installed},
            )
        return UninstallResult(
            store=self.name,
            app_id=app_id,
            uninstalled=True,
            kept_files=keep_files,
            install_path=install_path,
        )

    def launch_args(self, app_id: str, *, offline: bool = True) -> LaunchPlan:
        cmd = ["launch", app_id, "--json", "--no-wine"]
        if offline:
            cmd.append("--offline")
        data = self._run_json(cmd)
        if not isinstance(data, dict):
            raise CommandFailedError("unexpected non-object JSON from `legendary launch --json`")

        installed = self._installed_map().get(app_id) or {}
        game_dir = str(data.get("game_directory") or installed.get("install_path") or "")
        game_exe = str(data.get("game_executable") or installed.get("executable") or "").replace("\\", "/")
        work_dir = str(data.get("working_directory") or game_dir or ".")
        title = str(installed.get("title") or app_id)

        raw_args: list[str] = []
        for field_name in ("game_parameters", "egl_parameters", "user_parameters"):
            vals = data.get(field_name)
            if isinstance(vals, list):
                raw_args.extend(str(v) for v in vals)

        safe_args, args_redacted = redact_launch_args(raw_args)
        raw_env = data.get("environment") if isinstance(data.get("environment"), dict) else {}
        safe_env, env_redacted = redact_env({str(k): str(v) for k, v in raw_env.items()})

        return LaunchPlan(
            store=self.name,
            app_id=app_id,
            title=title,
            install_dir=game_dir,
            executable=game_exe,
            working_dir=work_dir,
            args=safe_args,
            env=safe_env,
            offline=offline,
            redacted_auth_args=bool(args_redacted or env_redacted),
        )


BACKEND_CLASS = EpicBackend


def create_backend() -> StoreBackend:
    return EpicBackend()
