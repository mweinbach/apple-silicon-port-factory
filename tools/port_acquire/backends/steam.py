"""Native Apple Silicon (`arm64`) Steam backend for `tools/port-acquire`.

Uses **SteamRE/DepotDownloader** (standalone native `arm64` macOS binary — zero
x86 translation layer) for interactive login, Steam Guard token caching, Windows
depot inspection (`-manifest-only`), Windows x64 depot downloads (`-os windows
-osarch 64`), branch/beta switching (`-branch`), checksum verification
(`-validate`), DLC depot pulls, and Steamworks Common Redistributables
(`AppID 228980`) fetches.

Metadata & owned-library sources (native `arm64` only):
- **`list-owned`:** Optional Steam Web API (`IPlayerService/GetOwnedGames/v1`,
  key read from macOS Keychain `port-acquire-steam-web-api-key` or registered
  `PORT_STEAM_API_KEY`) when configured; otherwise scans local Steam client
  `userdata/*/config/{localconfig.vdf,librarycache}` + `steamapps/appmanifest_*.acf`
  + managed `port-acquire` installs.
- **`info <app>`:** Runs `DepotDownloader -app <id> -os windows -osarch 64
  -manifest-only` to retrieve exact depot IDs, manifest GIDs, compressed/disk
  sizes, and full depot file lists (for executable discovery, Steamworks/anti-cheat/
  `_CommonRedist` detection before downloading), combined with Steam's public
  Store `appdetails` API for title, DLC AppID list, and DRM notices.

Hard rules:
- Zero DRM, anti-cheat, or ownership bypasses of any kind. `info` reports
  Steamworks/DRM/anti-cheat signals read-only.
- Only download titles owned by the logged-in account through DepotDownloader /
  Valve's normal Steam3 depot protocol.
- Never store, return, or log passwords, Steam Guard codes, refresh tokens, or
  Web API keys. Credentials stay in DepotDownloader's own cache or the macOS
  Keychain.
"""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from typing import Any, Callable
import urllib.parse
import urllib.request

from ..backend import (
    AcquireError,
    AnticheatReport,
    AppNotFoundError,
    AuthRequiredError,
    AuthStatus,
    BackendUnavailableError,
    CommandFailedError,
    DepotItem,
    DlcItem,
    DownloadOptions,
    DownloadResult,
    DrmReport,
    GameInfo,
    LaunchPlan,
    OwnedGame,
    RedistributableItem,
    SearchMatch,
    StoreBackend,
    UninstallResult,
    UpdateResult,
    VerifyResult,
    classify_redistributable_path,
    fuzzy_search_owned,
    normalize_title,
    scan_dir_redistributables,
    score_title_match,
)
from ..manifest import redact_env, redact_launch_args
from ._steam_vdf import (
    COMMON_REDIST_APP_ID,
    COMMON_REDIST_DEPOTS,
    extract_proton_redist_info,
    parse_appinfo_vdf_dlc_depots,
    parse_appmanifest_acf,
    parse_depot_downloader_stdout,
    parse_depot_manifest_txt,
    parse_libraryfolders_vdf,
    parse_local_owned_app_ids,
    parse_loginusers_vdf,
)

KEYCHAIN_WEB_API_SERVICE = "port-acquire-steam-web-api-key"

_ANTICHEAT_PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    ("Easy Anti-Cheat", re.compile(r"(easyanticheat|eac_launcher|start_protected_game)", re.I)),
    ("BattlEye", re.compile(r"(battleye|beservice|beclient)", re.I)),
    ("Valve Anti-Cheat (VAC)", re.compile(r"(\bvac\b|valve\s+anti-cheat)", re.I)),
    ("Riot Vanguard", re.compile(r"(vanguard|vgk\.sys)", re.I)),
    ("RICOCHET Anti-Cheat", re.compile(r"(ricochet|randgrid)", re.I)),
    ("nProtect GameGuard", re.compile(r"(gameguard|nprotect)", re.I)),
    ("PunkBuster", re.compile(r"punkbuster", re.I)),
    ("XIGNCODE", re.compile(r"xigncode", re.I)),
]

_DRM_PATTERNS: list[tuple[str, re.Pattern[str], str | None]] = [
    ("Denuvo Anti-Tamper", re.compile(r"denuvo", re.I), None),
    ("EA App / Origin", re.compile(r"(ea\s+app|origin\s+thin|ea\s+online\s+activation)", re.I), "EA App"),
    ("Ubisoft Connect", re.compile(r"(ubisoft\s+connect|uplay)", re.I), "Ubisoft Connect"),
    ("Rockstar Games Launcher", re.compile(r"rockstar\s+games\s+launcher", re.I), "Rockstar Games Launcher"),
    ("2K Launcher", re.compile(r"2k\s+launcher", re.I), "2K Launcher"),
    ("Battle.net", re.compile(r"battle\.net", re.I), "Battle.net"),
    ("SteamStub / CEG", re.compile(r"(steamstub|steam\s+ceg)", re.I), None),
]

_SKIP_EXE_PATTERNS = re.compile(
    r"(unins|vcredist|vc_redist|dxsetup|dxwebsetup|dotnet|ndp4|oalinst|physx|"
    r"crashreport|ueprereq|redist|easyanticheat|battleye|beservice|_setup\.exe$|setup\.exe$)",
    re.I,
)

_LOGIN_FAIL_PATTERNS = re.compile(
    r"(Unable\s+to\s+login|InvalidPassword|Enter\s+account\s+password|Steam\s+Guard|"
    r"Two-factor|AccountLogonDenied|RateLimitExceeded|Login\s+failed|No\s+stored\s+token|"
    r"does\s+not\s+have\s+access|not\s+available\s+from\s+this\s+account)",
    re.I,
)

_SKIPPED_DEPOT_RE = re.compile(
    r"Depot\s+(\d+)\s+is\s+not\s+available\s+from\s+this\s+account",
    re.I,
)

# DepotDownloader 3.4.0 emits a per-depot warning when a private beta branch is
# missing or its password is invalid, then silently falls back to `public`.
_BRANCH_FALLBACK_RE = re.compile(
    r"(does\s+not\s+have\s+branch\s+named|Trying\s+public\s+branch|"
    r"Unable\s+to\s+find\s+branch|Invalid\s+branch\s+password)",
    re.I,
)


def _check_branch_fallback(
    requested_branch: str,
    combined_output: str,
    actual_branch: str | None,
    *,
    app_id: str,
    context: str,
) -> None:
    """Reject DepotDownloader's silent fallback from a non-public branch to `public`."""
    if requested_branch.lower() == "public":
        return
    fell_back_header = bool(actual_branch and actual_branch.lower() != requested_branch.lower())
    fell_back_warning = bool(_BRANCH_FALLBACK_RE.search(combined_output))
    if fell_back_header or fell_back_warning:
        raise CommandFailedError(
            f"DepotDownloader fell back from requested branch {requested_branch!r} "
            f"(to {actual_branch or 'public'!r}) during {context} for Steam AppID {app_id}; "
            "verify the branch name and pass --branch-password-env for private betas.",
            code="branch_fallback",
            details={
                "app_id": str(app_id),
                "requested_branch": requested_branch,
                "actual_branch": actual_branch or "public",
            },
        )


def _split_windows_args(raw: str) -> list[str]:
    """Parse a Windows-style command-line argument string preserving quoted values."""
    args: list[str] = []
    cur: list[str] = []
    in_quotes = False
    has_token = False
    i = 0
    n = len(raw)
    while i < n:
        ch = raw[i]
        if ch == "\\":
            bs = 0
            while i + bs < n and raw[i + bs] == "\\":
                bs += 1
            if i + bs < n and raw[i + bs] == '"':
                cur.append("\\" * (bs // 2))
                if bs % 2 == 1:
                    cur.append('"')
                    has_token = True
                    i += bs + 1
                    continue
                i += bs
                continue
            cur.append("\\" * bs)
            has_token = True
            i += bs
            continue
        if ch == '"':
            in_quotes = not in_quotes
            has_token = True
            i += 1
            continue
        if ch.isspace() and not in_quotes:
            if has_token or cur:
                args.append("".join(cur))
                cur = []
                has_token = False
            i += 1
            continue
        cur.append(ch)
        has_token = True
        i += 1
    if has_token or cur:
        args.append("".join(cur))
    return args


def _redact_cmd(cmd: list[str]) -> list[str]:
    """Return a copy of `cmd` with any sensitive arguments replaced by `<redacted>`."""
    out: list[str] = []
    redact_next = False
    for tok in cmd:
        if redact_next:
            out.append("<redacted>")
            redact_next = False
            continue
        low = tok.lower()
        if low in ("-branchpassword", "-password", "--password"):
            out.append(tok)
            redact_next = True
            continue
        out.append(tok)
    return out


def _atomic_write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(data, indent=2, sort_keys=False) + "\n", encoding="utf-8")
    tmp.replace(path)


def _read_json(path: Path, default: Any) -> Any:
    if not path.is_file():
        return default
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return default


def _default_http_get_json(url: str, *, timeout: float = 15.0) -> Any:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "apple-silicon-port-factory/port-acquire",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read().decode("utf-8", errors="replace"))


class SteamBackend(StoreBackend):
    """Pluggable Steam backend powered natively on Apple Silicon by `DepotDownloader`."""

    name = "steam"
    display_name = "Steam (DepotDownloader native arm64)"

    def __init__(
        self,
        *,
        depot_downloader_bin: str | Path | None = None,
        steam_home: str | Path | None = None,
        state_dir: str | Path | None = None,
        http_get_json: Callable[[str], Any] | None = None,
        allow_network: bool = True,
    ) -> None:
        self._explicit_dd = Path(depot_downloader_bin) if depot_downloader_bin else None
        self._explicit_steam_home = Path(steam_home) if steam_home else None
        self._explicit_state_dir = Path(state_dir) if state_dir else None
        self._http_get_json = http_get_json or _default_http_get_json
        self._allow_network = allow_network

    # ------------------------------------------------------------------
    # Paths & Native Binary Discovery
    # ------------------------------------------------------------------

    def steam_home(self) -> Path:
        if self._explicit_steam_home is not None:
            return self._explicit_steam_home
        env_home = os.environ.get("PORT_STEAM_HOME", "").strip()
        if env_home:
            return Path(env_home).expanduser()
        if sys.platform == "darwin":
            return Path.home() / "Library" / "Application Support" / "Steam"
        return Path.home() / ".local" / "share" / "Steam"

    def state_dir(self) -> Path:
        if self._explicit_state_dir is not None:
            return self._explicit_state_dir
        env_home = os.environ.get("PORT_STEAM_HOME", "").strip()
        if env_home:
            return Path(env_home).expanduser() / ".port-acquire"
        return Path.home() / ".local" / "share" / "port-acquire"

    def _session_file(self) -> Path:
        return self.state_dir() / "steam-session.json"

    def _installs_file(self) -> Path:
        return self.state_dir() / "steam-installs.json"

    def _owned_cache_file(self, account_name: str) -> Path:
        safe_key = re.sub(r"[^a-z0-9_-]+", "_", account_name.strip().lower()).strip("_") or "default"
        return self.state_dir() / f"steam-owned-cache-{safe_key}.json"

    def _manifest_cache_file(self, app_id: str) -> Path:
        return self.state_dir() / "manifests" / f"{app_id}.json"

    def _store_details_cache_file(self, app_id: str) -> Path:
        return self.state_dir() / "store_details" / f"{app_id}.json"

    def find_depot_downloader(self) -> Path | None:
        if self._explicit_dd is not None:
            return self._explicit_dd if self._explicit_dd.is_file() else None
        env_bin = os.environ.get("PORT_DEPOT_DOWNLOADER", "").strip()
        if env_bin:
            p = Path(env_bin).expanduser()
            return p if p.is_file() else None
        candidates = [
            Path.home() / ".local" / "opt" / "depotdownloader" / "DepotDownloader",
            Path.home() / ".local" / "bin" / "DepotDownloader",
            Path.home() / ".local" / "bin" / "depotdownloader",
            Path("/opt/homebrew/bin/DepotDownloader"),
            Path("/opt/homebrew/bin/depotdownloader"),
            Path("/usr/local/bin/DepotDownloader"),
        ]
        for cand in candidates:
            if cand.is_file():
                return cand
        for name in ("DepotDownloader", "depotdownloader"):
            found = shutil.which(name)
            if found:
                return Path(found)
        return None

    def _require_depot_downloader(self) -> Path:
        dd = self.find_depot_downloader()
        if dd is None:
            raise BackendUnavailableError(
                "Native arm64 `DepotDownloader` not found. Install it with "
                "`cargo opsdb -- steam-setup` (downloads the standalone native macOS arm64 "
                "SteamRE/DepotDownloader release into ~/.local/bin/DepotDownloader), "
                "or point `PORT_DEPOT_DOWNLOADER` to your DepotDownloader binary."
            )
        return dd

    def _backend_version_label(self) -> str | None:
        dd = self.find_depot_downloader()
        if dd is None:
            return None
        try:
            proc = subprocess.run(
                [str(dd), "--version"],
                stdin=subprocess.DEVNULL,
                capture_output=True,
                text=True,
                timeout=5,
                check=False,
            )
            first = (proc.stdout or proc.stderr).strip().splitlines()
            if proc.returncode == 0 and first:
                return first[0].strip()
        except OSError:
            pass
        return f"DepotDownloader ({dd.name})"

    def _has_depot_downloader_config(self) -> bool:
        """Boolean check for DepotDownloader's own cached token store (never reads token bytes)."""
        dd = self.find_depot_downloader()
        candidates: list[Path] = [
            self.state_dir() / ".DepotDownloader" / "DepotDownloader.config",
            Path.home() / ".local" / "share" / "DepotDownloader" / "DepotDownloader.config",
            Path.home() / ".config" / "DepotDownloader" / "DepotDownloader.config",
        ]
        if dd is not None:
            resolved_dir = dd.resolve().parent
            candidates.append(resolved_dir / "DepotDownloader.config")
            candidates.append(resolved_dir / ".DepotDownloader" / "DepotDownloader.config")
        return any(c.is_file() for c in candidates)

    # ------------------------------------------------------------------
    # Keychain & Session Metadata (Strictly Secret-Free)
    # ------------------------------------------------------------------

    def _get_steam_web_api_key(self) -> str | None:
        """Read optional Steam Web API key from registered `PORT_STEAM_API_KEY` or macOS Keychain.

        Never returns or logs the key in user-facing output or manifests.
        """
        env_key = os.environ.get("PORT_STEAM_API_KEY", "").strip()
        if env_key:
            return env_key
        if sys.platform != "darwin":
            return None
        try:
            proc = subprocess.run(
                [
                    "/usr/bin/security",
                    "find-generic-password",
                    "-s",
                    KEYCHAIN_WEB_API_SERVICE,
                    "-w",
                ],
                capture_output=True,
                text=True,
                timeout=5,
                check=False,
            )
            if proc.returncode == 0:
                val = proc.stdout.strip()
                return val or None
        except OSError:
            pass
        return None

    def _load_session(self) -> dict[str, Any]:
        data = _read_json(self._session_file(), {})
        return data if isinstance(data, dict) else {}

    def _save_session(self, account_name: str, steam_id64: str | None = None) -> None:
        # Security invariant: ONLY account name and public SteamID64 are ever saved.
        payload: dict[str, Any] = {
            "logged_out": False,
            "account_name": account_name,
            "steam_id64": steam_id64,
            "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        }
        _atomic_write_json(self._session_file(), payload)

    def _clear_session(self) -> None:
        # Persist an explicit logged-out sentinel so subsequent `status`, `list-owned`,
        # and `download` calls do not silently rediscover an account from `loginusers.vdf`.
        payload: dict[str, Any] = {
            "logged_out": True,
            "account_name": None,
            "steam_id64": None,
            "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        }
        _atomic_write_json(self._session_file(), payload)

    def _resolve_account_info(
        self,
        explicit_username: str | None = None,
        *,
        ignore_logged_out: bool = False,
    ) -> tuple[str | None, str | None]:
        """Return `(account_name, steam_id64)` without ever touching passwords or tokens."""
        loginusers = parse_loginusers_vdf(self.steam_home() / "config" / "loginusers.vdf")

        def _find_id64(acct: str) -> str | None:
            for u in loginusers:
                if u["account_name"].lower() == acct.lower():
                    return u["steam_id64"]
            return None

        if explicit_username and explicit_username.strip():
            acct = explicit_username.strip()
            return acct, _find_id64(acct)

        sess = self._load_session()
        if not ignore_logged_out and bool(sess.get("logged_out")):
            return None, None

        env_user = os.environ.get("PORT_STEAM_USER", "").strip()
        if env_user:
            return env_user, _find_id64(env_user)

        sess_acct = str(sess.get("account_name") or "").strip()
        if sess_acct:
            sess_id = str(sess.get("steam_id64") or "").strip() or _find_id64(sess_acct)
            return sess_acct, sess_id

        if loginusers:
            primary = loginusers[0]
            return primary["account_name"] or None, primary["steam_id64"] or None

        return None, None

    def _load_installs(self) -> dict[str, dict[str, Any]]:
        data = _read_json(self._installs_file(), {})
        if not isinstance(data, dict):
            return {}
        return {str(k): v for k, v in data.items() if isinstance(v, dict)}

    def _save_installs(self, installs: dict[str, dict[str, Any]]) -> None:
        _atomic_write_json(self._installs_file(), installs)

    def _scan_steamapps_manifests(self) -> dict[str, dict[str, Any]]:
        out: dict[str, dict[str, Any]] = {}
        for sa_dir in parse_libraryfolders_vdf(self.steam_home()):
            for path in sorted(sa_dir.glob("appmanifest_*.acf")):
                parsed = parse_appmanifest_acf(path)
                if not parsed or not parsed.get("app_id"):
                    continue
                installdir = parsed.get("installdir", "")
                if installdir:
                    parsed["install_path"] = str(sa_dir / "common" / installdir)
                aid = parsed["app_id"]
                existing = out.get(aid)
                if existing is None:
                    out[aid] = parsed
                else:
                    # Prefer an entry whose install directory actually exists on disk.
                    old_exists = bool(
                        existing.get("install_path") and Path(str(existing["install_path"])).is_dir()
                    )
                    new_exists = bool(
                        parsed.get("install_path") and Path(str(parsed["install_path"])).is_dir()
                    )
                    if new_exists and not old_exists:
                        out[aid] = parsed
        return out

    def _get_installed_entry(self, app_id: str) -> dict[str, Any] | None:
        installs = self._load_installs()
        if str(app_id) in installs:
            return installs[str(app_id)]
        acf_map = self._scan_steamapps_manifests()
        return acf_map.get(str(app_id))

    def existing_managed_install_path(
        self,
        app_id: str,
        *,
        platform: str = "windows",
    ) -> Path | None:
        """Return the registered `port-acquire` install path for `app_id` matching `platform`.

        Intentionally ignores unmanaged Steam-client installations under `steamapps/common/`
        so downloading Windows depots without `--dest` never overwrites a native macOS Steam
        client installation.
        """
        resolved_id = self._resolve_app_id(app_id, offline=True)
        entry = self._load_installs().get(resolved_id)
        if not isinstance(entry, dict) or not entry.get("install_path"):
            return None
        entry_plat = str(entry.get("platform") or "windows").strip().lower()
        if entry_plat != platform.strip().lower():
            return None
        return Path(str(entry["install_path"])).expanduser().resolve()

    def _is_unmanaged_steam_client_install(
        self,
        resolved_id: str,
        entry: dict[str, Any],
    ) -> bool:
        """Return True when `entry` came from a Steam-client `.acf` rather than `port-acquire`."""
        return bool(entry.get("manifest_path")) and resolved_id not in self._load_installs()

    def _detect_acf_install_platform(self, install_path: Path) -> str | None:
        """Best-effort shallow detection of whether an unmanaged Steam-client folder is macOS or Windows."""
        if not install_path.is_dir():
            return None
        has_app = False
        has_exe = False
        stack: list[tuple[Path, int]] = [(install_path, 0)]
        while stack:
            cur, depth = stack.pop()
            if depth > 2:
                continue
            try:
                children = list(cur.iterdir())
            except OSError:
                continue
            for child in children:
                if child.is_symlink():
                    continue
                low = child.name.lower()
                if child.is_dir() and low.endswith(".app"):
                    has_app = True
                elif child.is_file() and low.endswith(".exe"):
                    has_exe = True
                elif child.is_dir() and depth < 2 and not child.name.startswith("."):
                    stack.append((child, depth + 1))
        if has_app and not has_exe:
            return "macos"
        if has_exe and not has_app:
            return "windows"
        return None

    # ------------------------------------------------------------------
    # DepotDownloader Execution
    # ------------------------------------------------------------------

    def _run_depot_downloader(
        self,
        args: list[str],
        *,
        interactive: bool = False,
        timeout: float | None = 1800.0,
        check: bool = True,
    ) -> subprocess.CompletedProcess[str]:
        dd = self._require_depot_downloader()
        cmd = [str(dd), *args]
        env = os.environ.copy()
        if self._explicit_steam_home is not None:
            env["PORT_STEAM_HOME"] = str(self._explicit_steam_home)

        if interactive:
            try:
                proc = subprocess.run(cmd, env=env, check=False)
            except OSError as exc:
                raise BackendUnavailableError(f"failed to execute DepotDownloader: {exc}") from exc
            completed = subprocess.CompletedProcess(cmd, proc.returncode, "", "")
        else:
            try:
                completed = subprocess.run(
                    cmd,
                    env=env,
                    stdin=subprocess.DEVNULL,
                    capture_output=True,
                    text=True,
                    timeout=timeout,
                    check=False,
                )
            except subprocess.TimeoutExpired as exc:
                raise CommandFailedError(
                    "DepotDownloader timed out (it may be waiting for interactive login or Steam Guard; "
                    "run `tools/port-acquire --store steam login --username <your_steam_account>` first)",
                    details={"command": _redact_cmd(cmd)},
                ) from exc
            except OSError as exc:
                raise BackendUnavailableError(f"failed to execute DepotDownloader: {exc}") from exc

        if check and completed.returncode != 0:
            combined = (completed.stdout + "\n" + completed.stderr).strip()
            tail = "\n".join(combined.splitlines()[-12:])
            if _LOGIN_FAIL_PATTERNS.search(combined):
                raise AuthRequiredError(
                    f"DepotDownloader authentication/entitlement check failed: {tail or 'login required'}",
                    details={"returncode": completed.returncode, "command": _redact_cmd(cmd)},
                )
            raise CommandFailedError(
                f"DepotDownloader exited with status {completed.returncode}: {tail or 'no output'}",
                details={"returncode": completed.returncode, "command": _redact_cmd(cmd)},
            )
        return completed

    def _fetch_depot_manifests(
        self,
        app_id: str,
        *,
        depot_id: str | None = None,
        platform: str = "windows",
        branch: str = "public",
        branch_password: str | None = None,
        account_name: str | None = None,
        offline: bool = False,
        allow_cache_fallback: bool = True,
        update_cache: bool = True,
    ) -> dict[str, Any]:
        """Run `DepotDownloader -manifest-only` (or read cached manifests when offline)."""
        cache_path = self._manifest_cache_file(app_id)
        if offline:
            cached = _read_json(cache_path, {})
            return cached if isinstance(cached, dict) else {}

        with tempfile.TemporaryDirectory(prefix=f"port-acquire-dd-{app_id}-") as tmp_dir:
            dd_args: list[str] = [
                "-app",
                str(app_id),
            ]
            if depot_id:
                dd_args.extend(["-depot", str(depot_id)])
            dd_args.extend(
                [
                    "-os",
                    platform,
                    "-osarch",
                    "64",
                    "-branch",
                    branch,
                    "-manifest-only",
                    "-dir",
                    tmp_dir,
                ]
            )
            if branch_password:
                dd_args.extend(["-branchpassword", branch_password])
            if account_name:
                dd_args.extend(["-username", account_name, "-remember-password"])

            proc = self._run_depot_downloader(
                dd_args,
                interactive=False,
                timeout=120.0,
                check=False,
            )
            combined = proc.stdout + "\n" + proc.stderr
            stdout_info = parse_depot_downloader_stdout(combined)
            skipped_depots = sorted(set(_SKIPPED_DEPOT_RE.findall(combined)))
            _check_branch_fallback(
                branch,
                combined,
                stdout_info.get("branch"),
                app_id=str(app_id),
                context="manifest query",
            )

            parsed_by_id: dict[str, dict[str, Any]] = {}
            for txt_file in sorted(Path(tmp_dir).rglob("manifest_*.txt")):
                parsed = parse_depot_manifest_txt(
                    txt_file.read_text(encoding="utf-8", errors="ignore"),
                    filename=txt_file.name,
                )
                did = parsed.get("depot_id")
                if did:
                    parsed_by_id[did] = parsed

            # Preserve DepotDownloader's exact depot processing order (from stdout) so
            # overlapping files across depots follow DepotDownloader's last-depot-wins order.
            depots: list[dict[str, Any]] = []
            seen_ids: set[str] = set()
            for sd in stdout_info.get("depots", []):
                did = sd.get("depot_id")
                if not did or did in seen_ids:
                    continue
                seen_ids.add(did)
                if did in parsed_by_id:
                    dep = parsed_by_id[did]
                    if sd.get("name"):
                        dep["name"] = sd["name"]
                    depots.append(dep)
                else:
                    depots.append(
                        {
                            "depot_id": did,
                            "name": sd.get("name", ""),
                            "manifest_gid": sd.get("manifest_gid"),
                            "creation_date": sd.get("date"),
                            "file_count": 0,
                            "disk_size_bytes": None,
                            "download_size_bytes": None,
                            "has_file_manifest": False,
                            "files": [],
                        }
                    )
            for did in sorted(
                (k for k in parsed_by_id if k not in seen_ids),
                key=lambda x: int(x) if x.isdigit() else 0,
            ):
                seen_ids.add(did)
                depots.append(parsed_by_id[did])

            if proc.returncode != 0:
                if allow_cache_fallback:
                    if depots:
                        return {
                            "app_id": str(app_id),
                            "platform": platform,
                            "branch": stdout_info.get("branch") or branch,
                            "depots": depots,
                            "partial": True,
                            "fetched_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
                        }
                    cached = _read_json(cache_path, {})
                    if isinstance(cached, dict) and cached.get("depots"):
                        return cached
                if _LOGIN_FAIL_PATTERNS.search(combined):
                    raise AuthRequiredError(
                        f"DepotDownloader could not query manifests for AppID {app_id}; "
                        f"run `tools/port-acquire --store steam login --username {account_name or '<your_steam_account>'}`."
                    )
                raise CommandFailedError(
                    f"DepotDownloader -manifest-only failed for AppID {app_id} (exit {proc.returncode})"
                )

            payload = {
                "app_id": str(app_id),
                "platform": platform,
                "branch": stdout_info.get("branch") or branch,
                "depots": depots,
                "skipped_depots": skipped_depots,
                "fetched_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
            if update_cache and depots and depot_id is None:
                _atomic_write_json(cache_path, payload)
            return payload

    # ------------------------------------------------------------------
    # StoreBackend Methods
    # ------------------------------------------------------------------

    def login(
        self,
        *,
        no_webview: bool = False,
        auth_code: str | None = None,
        import_existing: bool = False,
        logout: bool = False,
        username: str | None = None,
    ) -> AuthStatus:
        del no_webview, auth_code  # DepotDownloader handles Steam Guard directly in the terminal
        if logout:
            self._clear_session()
            return AuthStatus(
                store=self.name,
                logged_in=False,
                display_name=None,
                backend_available=self.find_depot_downloader() is not None,
                backend_version=self._backend_version_label(),
            )

        account_name, steam_id64 = self._resolve_account_info(
            username,
            ignore_logged_out=import_existing,
        )

        if import_existing:
            if not account_name:
                raise AuthRequiredError(
                    "No existing Steam account found in `loginusers.vdf`. Run interactive login:\n"
                    "  tools/port-acquire --store steam login --username <your_steam_account>"
                )
            self._save_session(account_name, steam_id64)
            return self.status(offline=self.find_depot_downloader() is None)

        if not account_name:
            if sys.stdin.isatty():
                entered = input("Steam account username (not password): ").strip()
                if entered:
                    account_name, steam_id64 = self._resolve_account_info(entered, ignore_logged_out=True)
            if not account_name:
                raise AuthRequiredError(
                    "Steam username required for interactive login. Run:\n"
                    "  tools/port-acquire --store steam login --username <your_steam_account>"
                )

        # Interactive login attaches directly to the user's TTY so DepotDownloader
        # prompts for password and Steam Guard directly (credentials stay in
        # DepotDownloader's own cache and never touch our code or logs).
        # Uses AppID 228980 (free Steamworks Common Redistributables) with -manifest-only
        # so zero game payloads are downloaded during authentication.
        dd = self._require_depot_downloader()
        with tempfile.TemporaryDirectory(prefix="port-acquire-dd-login-") as tmp_dir:
            proc = self._run_depot_downloader(
                [
                    "-app",
                    COMMON_REDIST_APP_ID,
                    "-os",
                    "windows",
                    "-osarch",
                    "64",
                    "-manifest-only",
                    "-dir",
                    tmp_dir,
                    "-username",
                    account_name,
                    "-remember-password",
                ],
                interactive=True,
                check=False,
            )

        if proc.returncode != 0:
            raise AuthRequiredError(
                f"DepotDownloader login did not complete cleanly (exit {proc.returncode}). "
                f"Re-run interactively:\n"
                f"  tools/port-acquire --store steam login --username {account_name}"
            )

        _, refreshed_id64 = self._resolve_account_info(account_name, ignore_logged_out=True)
        self._save_session(account_name, refreshed_id64 or steam_id64)

        return AuthStatus(
            store=self.name,
            logged_in=True,
            display_name=account_name,
            backend_available=dd.is_file(),
            backend_version=self._backend_version_label(),
        )

    def status(self, *, offline: bool = False) -> AuthStatus:
        dd = self.find_depot_downloader()
        available = dd is not None
        version = self._backend_version_label()
        account_name, _ = self._resolve_account_info()

        if not account_name:
            return AuthStatus(
                store=self.name,
                logged_in=False,
                display_name=None,
                backend_available=available,
                backend_version=version,
            )

        if offline or dd is None:
            sess = self._load_session()
            sess_active = bool(sess.get("account_name")) and not bool(sess.get("logged_out"))
            has_cache = self._has_depot_downloader_config()
            loginusers = parse_loginusers_vdf(self.steam_home() / "config" / "loginusers.vdf")
            remembered = any(
                u["account_name"].lower() == account_name.lower()
                and (u["remember_password"] or u["allow_auto_login"])
                for u in loginusers
            )
            logged_in = bool(sess_active or has_cache or remembered)
            return AuthStatus(
                store=self.name,
                logged_in=logged_in,
                display_name=account_name if logged_in else None,
                backend_available=available,
                backend_version=version,
            )

        # Non-interactive cached-token probe via `DepotDownloader -manifest-only` on free App 228980.
        with tempfile.TemporaryDirectory(prefix="port-acquire-dd-status-") as tmp_dir:
            proc = self._run_depot_downloader(
                [
                    "-app",
                    COMMON_REDIST_APP_ID,
                    "-os",
                    "windows",
                    "-osarch",
                    "64",
                    "-manifest-only",
                    "-dir",
                    tmp_dir,
                    "-username",
                    account_name,
                    "-remember-password",
                ],
                interactive=False,
                timeout=30.0,
                check=False,
            )
        combined = proc.stdout + "\n" + proc.stderr
        logged_in = proc.returncode == 0 and not _LOGIN_FAIL_PATTERNS.search(combined)
        return AuthStatus(
            store=self.name,
            logged_in=logged_in,
            display_name=account_name if logged_in else None,
            backend_available=available,
            backend_version=version,
        )

    def _require_logged_in_account(self) -> tuple[str, str | None]:
        account_name, steam_id64 = self._resolve_account_info()
        if not account_name:
            raise AuthRequiredError(
                "Not logged in to Steam. Have the account owner run:\n"
                "  tools/port-acquire --store steam login --username <your_steam_account>"
            )
        return account_name, steam_id64

    def _list_owned_via_web_api(self, api_key: str, steam_id64: str) -> list[OwnedGame] | None:
        if not self._allow_network:
            return None
        query = urllib.parse.urlencode(
            {
                "key": api_key,
                "steamid": steam_id64,
                "include_appinfo": "1",
                "include_played_free_games": "1",
                "format": "json",
            }
        )
        url = f"https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?{query}"
        try:
            payload = self._http_get_json(url)
        except Exception:
            # Never expose `url` (contains api_key) in errors.
            return None
        resp = payload.get("response") if isinstance(payload, dict) else None
        raw_games = resp.get("games") if isinstance(resp, dict) else None
        if not isinstance(raw_games, list):
            return None
        owned: list[OwnedGame] = []
        for entry in raw_games:
            if not isinstance(entry, dict) or "appid" not in entry:
                continue
            aid = str(entry["appid"])
            title = str(entry.get("name") or f"Steam App {aid}")
            owned.append(
                OwnedGame(
                    store=self.name,
                    app_id=aid,
                    title=title,
                    platforms=["windows"],
                )
            )
        return owned

    def _load_owned_id_cache(self, account_name: str | None) -> set[str]:
        if not account_name:
            return set()
        data = _read_json(self._owned_cache_file(account_name), {})
        raw = data.get("app_ids") if isinstance(data, dict) else None
        if isinstance(raw, list):
            return {str(x) for x in raw if str(x).isdigit()}
        return set()

    def _save_owned_id_cache(self, app_ids: set[str], account_name: str | None) -> None:
        if not account_name:
            return
        _atomic_write_json(
            self._owned_cache_file(account_name),
            {
                "account_name": account_name,
                "app_ids": sorted(app_ids, key=lambda x: int(x) if x.isdigit() else 0),
                "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            },
        )

    def _is_multi_account_environment(self, account_name: str | None = None) -> bool:
        loginusers = parse_loginusers_vdf(self.steam_home() / "config" / "loginusers.vdf")
        if len(loginusers) > 1:
            return True
        if account_name and loginusers:
            known_accts = {u["account_name"].lower() for u in loginusers if u.get("account_name")}
            if known_accts and account_name.lower() not in known_accts:
                return True
        userdata = self.steam_home() / "userdata"
        if userdata.is_dir():
            numeric_dirs = [p for p in userdata.iterdir() if p.is_dir() and p.name.isdigit()]
            if len(numeric_dirs) > 1:
                return True
        return False

    def _local_userdata_owned_ids(
        self,
        account_name: str | None,
        steam_id64: str | None,
    ) -> set[str]:
        if steam_id64 is not None:
            return parse_local_owned_app_ids(self.steam_home(), steam_id64=steam_id64)
        loginusers = parse_loginusers_vdf(self.steam_home() / "config" / "loginusers.vdf")
        if loginusers:
            # `loginusers.vdf` exists for other account(s), none matching `account_name`;
            # never fall back to reading another account's `userdata` directory.
            return set()
        return parse_local_owned_app_ids(self.steam_home(), steam_id64=None)

    def _install_belongs_to_account(
        self,
        entry: dict[str, Any],
        account_name: str | None,
        steam_id64: str | None,
        multi_account: bool,
    ) -> bool:
        entry_acct = str(entry.get("account_name") or "").strip()
        entry_id64 = str(entry.get("steam_id64") or "").strip()
        if entry_acct:
            return bool(account_name and entry_acct.lower() == account_name.lower())
        if entry_id64:
            return bool(steam_id64 and entry_id64 == str(steam_id64))
        return not multi_account

    def _acf_belongs_to_account(
        self,
        acf: dict[str, Any],
        steam_id64: str | None,
        multi_account: bool,
    ) -> bool:
        last_owner = str(acf.get("last_owner") or "").strip()
        if last_owner and last_owner != "0":
            return bool(steam_id64 and last_owner == str(steam_id64))
        return not multi_account

    def _known_owned_app_ids(self, *, offline: bool = False) -> set[str]:
        """Return known owned AppIDs scoped strictly to the active Steam account."""
        account_name, steam_id64 = self._resolve_account_info()
        if not account_name:
            return set()

        multi_account = self._is_multi_account_environment(account_name)
        owned_ids = self._load_owned_id_cache(account_name)
        owned_ids |= self._local_userdata_owned_ids(account_name, steam_id64)

        for aid, inst in self._load_installs().items():
            if self._install_belongs_to_account(inst, account_name, steam_id64, multi_account):
                owned_ids.add(aid)
                for dlc_id in inst.get("dlcs") or []:
                    if str(dlc_id).isdigit():
                        owned_ids.add(str(dlc_id))

        for aid, acf in self._scan_steamapps_manifests().items():
            if self._acf_belongs_to_account(acf, steam_id64, multi_account):
                owned_ids.add(aid)

        if not offline:
            api_key = self._get_steam_web_api_key()
            if api_key and steam_id64:
                web_games = self._list_owned_via_web_api(api_key, steam_id64)
                if web_games is not None:
                    for g in web_games:
                        owned_ids.add(g.app_id)
                    self._save_owned_id_cache(owned_ids, account_name)
        return owned_ids

    def list_owned(self, *, platform: str = "windows", refresh: bool = False) -> list[OwnedGame]:
        del refresh
        account_name, steam_id64 = self._require_logged_in_account()
        multi_account = self._is_multi_account_environment(account_name)
        installs = self._load_installs()
        acf_map = self._scan_steamapps_manifests()

        games_by_id: dict[str, OwnedGame] = {}

        # 1. Steam Web API (`IPlayerService/GetOwnedGames/v1`) when key + SteamID64 are available
        api_key = self._get_steam_web_api_key()
        if api_key and steam_id64:
            web_games = self._list_owned_via_web_api(api_key, steam_id64)
            if web_games is not None:
                for g in web_games:
                    games_by_id[g.app_id] = g

        # 2. Local Steam library caches scoped to the active Steam account:
        #    `userdata/<account_id32>/config/{localconfig.vdf,librarycache}`,
        #    account-owned `steamapps/appmanifest_*.acf`, and managed installs.
        local_ids = self._local_userdata_owned_ids(account_name, steam_id64)
        for aid, inst in installs.items():
            if self._install_belongs_to_account(inst, account_name, steam_id64, multi_account):
                local_ids.add(aid)
        for aid, acf in acf_map.items():
            if self._acf_belongs_to_account(acf, steam_id64, multi_account):
                local_ids.add(aid)

        for aid in sorted(local_ids, key=lambda x: int(x) if x.isdigit() else 0):
            if aid == COMMON_REDIST_APP_ID:
                continue
            if aid not in games_by_id:
                inst_title = (
                    str(installs.get(aid, {}).get("title") or "")
                    or str(acf_map.get(aid, {}).get("name") or "")
                )
                games_by_id[aid] = OwnedGame(
                    store=self.name,
                    app_id=aid,
                    title=inst_title or f"Steam App {aid}",
                    platforms=["windows"],
                )

        # Merge installed state onto games owned by this account.
        # Managed `port-acquire` installs take precedence over unmanaged Steam-client
        # `.acf` installs (matching `_get_installed_entry()`).
        managed_installed_ids: set[str] = set()
        for aid, inst in installs.items():
            if aid in games_by_id:
                games_by_id[aid].installed = True
                games_by_id[aid].install_path = inst.get("install_path")
                games_by_id[aid].version = inst.get("build_id")
                managed_installed_ids.add(aid)
                if inst.get("title") and games_by_id[aid].title.startswith("Steam App "):
                    games_by_id[aid].title = str(inst["title"])
        for aid, acf in acf_map.items():
            if aid in games_by_id:
                if aid not in managed_installed_ids:
                    games_by_id[aid].installed = True
                    games_by_id[aid].install_path = acf.get("install_path")
                    games_by_id[aid].version = acf.get("build_id")
                if acf.get("name") and games_by_id[aid].title.startswith("Steam App "):
                    games_by_id[aid].title = str(acf["name"])

        if games_by_id:
            self._save_owned_id_cache(set(games_by_id.keys()), account_name)

        result = sorted(
            games_by_id.values(),
            key=lambda g: (not g.installed, g.title.lower(), int(g.app_id) if g.app_id.isdigit() else 0),
        )
        if platform != "all":
            result = [g for g in result if not g.platforms or platform in g.platforms]
        return result

    def _store_search_titles(self, query: str) -> list[tuple[str, str]]:
        """Query Steam's public Store Search API (no credentials) to map titles to AppIDs."""
        if not self._allow_network:
            return []
        qs = urllib.parse.urlencode({"term": query, "l": "english", "cc": "US"})
        url = f"https://store.steampowered.com/api/storesearch/?{qs}"
        try:
            payload = self._http_get_json(url)
        except Exception:
            return []
        items = payload.get("items") if isinstance(payload, dict) else None
        if not isinstance(items, list):
            return []
        out: list[tuple[str, str]] = []
        for item in items:
            if not isinstance(item, dict) or "id" not in item:
                continue
            out.append((str(item["id"]), str(item.get("name") or "")))
        return out

    def search(
        self,
        query: str,
        *,
        platform: str = "windows",
        limit: int = 10,
        min_score: float = 0.35,
        refresh: bool = False,
    ) -> list[SearchMatch]:
        owned = self.list_owned(platform=platform, refresh=refresh)

        # Enrich unnamed local-cache AppIDs via Steam's public Store Search API while
        # strictly restricting returned matches to titles owned by the account.
        has_unnamed = any(g.title.startswith("Steam App ") for g in owned)
        if has_unnamed and self._allow_network and not query.strip().isdigit():
            owned_by_id = {g.app_id: g for g in owned}
            for store_aid, store_title in self._store_search_titles(query):
                if store_aid in owned_by_id and store_title:
                    owned_by_id[store_aid].title = store_title

        return fuzzy_search_owned(owned, query, limit=limit, min_score=min_score)

    def _resolve_app_id(
        self,
        app_ref: str,
        *,
        offline: bool = False,
        strict: bool = False,
        require_exact: bool = False,
    ) -> str:
        raw = app_ref.strip()
        if raw.isdigit():
            return raw

        norm_query = normalize_title(raw)

        # Collect all local installed candidates across both managed installs and steamapps ACFs.
        local_candidates: dict[str, str] = {}
        for aid, acf in self._scan_steamapps_manifests().items():
            local_candidates[str(aid)] = str(acf.get("name") or "")
        for aid, inst in self._load_installs().items():
            local_candidates[str(aid)] = str(inst.get("title") or local_candidates.get(str(aid), ""))

        # 1. Exact normalized title match always wins over fuzzy matches.
        exact_local = [
            (aid, title)
            for aid, title in local_candidates.items()
            if norm_query and normalize_title(title) == norm_query
        ]
        if len(exact_local) == 1:
            return exact_local[0][0]
        if len(exact_local) > 1:
            choices = ", ".join(f"{aid} ({title})" for aid, title in sorted(exact_local))
            raise AcquireError(
                f"Ambiguous Steam app name {app_ref!r}; multiple exact matches: {choices}. "
                "Specify the numeric AppID explicitly.",
                code="ambiguous_app",
                details={"query": raw, "candidates": [{"app_id": a, "title": t} for a, t in exact_local]},
            )

        # 2. Rank fuzzy local matches; destructive operations require exact title or numeric AppID.
        scored_local: list[tuple[float, str, str]] = []
        for aid, title in local_candidates.items():
            sc = score_title_match(raw, title, aid)
            if sc >= 0.60:
                scored_local.append((sc, aid, title))
        scored_local.sort(key=lambda item: (-item[0], item[2].lower(), item[1]))
        if require_exact:
            if scored_local:
                all_cands = [(aid, title) for _, aid, title in scored_local[:5]]
                choices = ", ".join(f"{aid} ({title})" for aid, title in all_cands)
                raise AcquireError(
                    f"Ambiguous or non-exact Steam app name {app_ref!r} for destructive operation; "
                    f"closest installed titles: {choices}. Specify the exact title or numeric AppID.",
                    code="ambiguous_app",
                    details={"query": raw, "candidates": [{"app_id": a, "title": t} for a, t in all_cands]},
                )
            raise AppNotFoundError(
                f"Could not find installed Steam app {app_ref!r}. Specify the exact title or numeric AppID."
            )
        if scored_local and scored_local[0][0] >= 0.75:
            top_score, top_aid, _ = scored_local[0]
            competing = [
                (aid, title)
                for sc, aid, title in scored_local[1:]
                if sc >= 0.75 or (strict and (top_score - sc) <= 0.15)
            ]
            if competing:
                all_cands = [(top_aid, scored_local[0][2]), *competing]
                choices = ", ".join(f"{aid} ({title})" for aid, title in all_cands)
                raise AcquireError(
                    f"Ambiguous Steam app name {app_ref!r}; matching installed titles: {choices}. "
                    "Specify the exact title or numeric AppID.",
                    code="ambiguous_app",
                    details={"query": raw, "candidates": [{"app_id": a, "title": t} for a, t in all_cands]},
                )
            return top_aid

        if not offline:
            matches = self.search(raw, limit=5, min_score=0.5)
            if matches:
                exact_remote = [m for m in matches if norm_query and normalize_title(m.title) == norm_query]
                if len(exact_remote) == 1:
                    return exact_remote[0].app_id
                if len(exact_remote) > 1:
                    choices = ", ".join(f"{m.app_id} ({m.title})" for m in exact_remote)
                    raise AcquireError(
                        f"Ambiguous Steam app name {app_ref!r}; multiple exact matches: {choices}. "
                        "Specify the numeric AppID explicitly.",
                        code="ambiguous_app",
                    )
                if len(matches) > 1 and (matches[0].score - matches[1].score) <= 0.05:
                    choices = ", ".join(f"{m.app_id} ({m.title})" for m in matches[:3])
                    raise AcquireError(
                        f"Ambiguous Steam app name {app_ref!r}; top matches: {choices}. "
                        "Specify the exact title or numeric AppID.",
                        code="ambiguous_app",
                    )
                return matches[0].app_id

        raise AppNotFoundError(
            f"Could not resolve Steam app {app_ref!r} to a numeric AppID. "
            "Run `tools/port-acquire --store steam search \"<title>\" --json` to find its AppID."
        )

    def _fetch_store_appdetails(self, app_id: str, *, offline: bool = False) -> dict[str, Any]:
        cache_path = self._store_details_cache_file(app_id)
        if offline or not self._allow_network:
            cached = _read_json(cache_path, {})
            return cached if isinstance(cached, dict) else {}
        url = f"https://store.steampowered.com/api/appdetails?appids={urllib.parse.quote(app_id)}&l=english"
        try:
            payload = self._http_get_json(url)
        except Exception:
            cached = _read_json(cache_path, {})
            return cached if isinstance(cached, dict) else {}
        entry = payload.get(str(app_id)) if isinstance(payload, dict) else None
        if isinstance(entry, dict) and entry.get("success") and isinstance(entry.get("data"), dict):
            data = entry["data"]
            _atomic_write_json(cache_path, data)
            return data
        cached = _read_json(cache_path, {})
        return cached if isinstance(cached, dict) else {}

    def _detect_signals(
        self,
        manifest_paths: list[str],
        store_details: dict[str, Any],
        install_dir: Path | None,
    ) -> tuple[bool, list[str], AnticheatReport, DrmReport]:
        """Collect read-only Steamworks, anti-cheat, and DRM signals (never bypass)."""
        steamworks_signals: list[str] = []
        anticheat_systems: list[str] = []
        anticheat_signals: list[str] = []
        drm_signals: list[str] = []
        requires_online_auth = False
        third_party_launcher: str | None = None

        # 1. Inspect file paths from DepotDownloader `-manifest-only`
        for rel in manifest_paths:
            low_name = Path(rel).name.lower()
            if low_name in ("steam_api64.dll", "steam_api.dll", "steam_appid.txt"):
                sig = f"manifest:{rel}"
                if sig not in steamworks_signals:
                    steamworks_signals.append(sig)
            for sys_name, pattern in _ANTICHEAT_PATTERNS:
                if pattern.search(rel):
                    if sys_name not in anticheat_systems:
                        anticheat_systems.append(sys_name)
                    sig = f"manifest:{rel}"
                    if sig not in anticheat_signals:
                        anticheat_signals.append(sig)

        # 2. Inspect Steam Store `appdetails` (drm_notice, ext_user_account_notice, categories)
        drm_notice = str(store_details.get("drm_notice") or "").strip()
        if drm_notice:
            clean_notice = re.sub(r"<[^>]+>", " ", drm_notice).strip()
            drm_signals.append(f"store:drm_notice={clean_notice}")
            for drm_name, pattern, launcher in _DRM_PATTERNS:
                if pattern.search(clean_notice):
                    if launcher and third_party_launcher is None:
                        third_party_launcher = launcher
                    if drm_name == "Denuvo Anti-Tamper":
                        requires_online_auth = True
                    if f"drm:{drm_name}" not in drm_signals:
                        drm_signals.append(f"drm:{drm_name}")

        ext_account = str(store_details.get("ext_user_account_notice") or "").strip()
        if ext_account:
            clean_ext = re.sub(r"<[^>]+>", " ", ext_account).strip()
            drm_signals.append(f"store:ext_user_account={clean_ext}")
            requires_online_auth = True

        categories = store_details.get("categories")
        if isinstance(categories, list):
            for cat in categories:
                desc = str(cat.get("description") or "") if isinstance(cat, dict) else ""
                low = desc.lower()
                if "steam achievements" in low or "steam cloud" in low or "steam workshop" in low:
                    sig = f"store:category={desc}"
                    if sig not in steamworks_signals:
                        steamworks_signals.append(sig)
                if "valve anti-cheat" in low:
                    if "Valve Anti-Cheat (VAC)" not in anticheat_systems:
                        anticheat_systems.append("Valve Anti-Cheat (VAC)")
                    anticheat_signals.append("store:category=VAC")

        # 3. Inspect downloaded files on disk (without following symlinks)
        if install_dir is not None and install_dir.is_dir():
            stack: list[tuple[Path, int]] = [(install_dir, 0)]
            while stack:
                cur, depth = stack.pop()
                if depth > 4:
                    continue
                try:
                    entries = sorted(cur.iterdir(), key=lambda p: p.name.lower())
                except OSError:
                    continue
                for entry in entries:
                    if entry.is_symlink():
                        continue
                    rel = entry.relative_to(install_dir).as_posix()
                    low_name = entry.name.lower()
                    if low_name in ("steam_api64.dll", "steam_api.dll", "steam_appid.txt"):
                        sig = f"disk:{rel}"
                        if sig not in steamworks_signals:
                            steamworks_signals.append(sig)
                    for sys_name, pattern in _ANTICHEAT_PATTERNS:
                        if pattern.search(rel):
                            if sys_name not in anticheat_systems:
                                anticheat_systems.append(sys_name)
                            sig = f"disk:{rel}"
                            if sig not in anticheat_signals:
                                anticheat_signals.append(sig)
                    if entry.is_dir() and depth < 4 and not entry.name.startswith("."):
                        stack.append((entry, depth + 1))

        ac_report = AnticheatReport(
            detected=bool(anticheat_systems or anticheat_signals),
            systems=anticheat_systems,
            signals=anticheat_signals,
            report_only=True,
        )
        drm_report = DrmReport(
            detected=bool(drm_signals or requires_online_auth or third_party_launcher),
            requires_online_auth=requires_online_auth,
            requires_ownership_token=False,
            third_party_launcher=third_party_launcher,
            signals=drm_signals,
            report_only=True,
        )
        return bool(steamworks_signals), steamworks_signals, ac_report, drm_report

    def _pick_executable_from_paths(self, paths: list[str]) -> str | None:
        """Choose the most likely primary Windows `.exe` from a list of relative file paths."""
        candidates: list[str] = []
        for raw in paths:
            norm = raw.replace("\\", "/")
            low = norm.lower()
            if not low.endswith(".exe"):
                continue
            fname = Path(norm).name
            if _SKIP_EXE_PATTERNS.search(fname) or "_commonredist/" in low:
                continue
            candidates.append(norm)
        if not candidates:
            return None
        candidates.sort(key=lambda p: (len(Path(p).parts), Path(p).name.lower()))
        return candidates[0]

    def _version_from_manifests(self, depots: list[dict[str, Any]]) -> str | None:
        """Derive a deterministic version fingerprint from depot manifest GIDs."""
        pairs = sorted(
            (str(d["depot_id"]), str(d["manifest_gid"]))
            for d in depots
            if d.get("depot_id") and d.get("manifest_gid")
        )
        if not pairs:
            return None
        if len(pairs) == 1:
            return pairs[0][1]
        return ",".join(f"{did}:{gid}" for did, gid in pairs)

    @staticmethod
    def _extract_dd_filter_options(extra_args: tuple[str, ...] | list[str]) -> tuple[str, bool]:
        """Extract `-language <lang>` (default `'english'`) and `-lowviolence` from extra args."""
        language = "english"
        lowviolence = False
        args = list(extra_args)
        for i, tok in enumerate(args):
            low = str(tok).strip().lower()
            if low == "-language" and i + 1 < len(args) and str(args[i + 1]).strip():
                language = str(args[i + 1]).strip().lower()
            elif low == "-lowviolence":
                lowviolence = True
        return language, lowviolence

    def _resolve_dlc_depot_map(
        self,
        app_id: str,
        depots: list[dict[str, Any]],
        store_details: dict[str, Any],
        *,
        saved_dlc_depots: dict[str, Any] | None = None,
        platform: str | None = None,
        osarch: str | None = None,
        language: str | None = None,
        lowviolence: bool = False,
    ) -> tuple[dict[str, list[str]], dict[str, str]]:
        """Map `dlc_app_id -> [depot_ids]` and `depot_id -> dlc_app_id` under base `app_id`.

        In Steamworks, all DLC depots reside inside the base game's depot list (never under
        the DLC's standalone AppID). Resolution order:
        1. Local Steam binary `appcache/appinfo.vdf` (`depots.<id>.dlcappid`), filtered by
           `config.{oslist,osarch,language,lowviolence}` when `platform` is supplied.
        2. Previously recorded `dlc_depots` in `steam-installs.json` (excluding depots
           explicitly ruled out by `appinfo.vdf` config filters).
        3. Explicit `dlc_app_id` field on depot dicts.
        4. Steam Store `appdetails.dlc` IDs correlated with depots returned under `app_id`
           (exact depot ID == DLC AppID or standard adjacent depot ID == DLC AppID + 1).
        """
        dlc_to_depots: dict[str, list[str]] = {}
        depot_to_dlc: dict[str, str] = {}

        def _link(dlc_id: str, depot_id: str) -> None:
            if (
                not dlc_id.isdigit()
                or not depot_id.isdigit()
                or depot_id == str(app_id)
                or depot_id in COMMON_REDIST_DEPOTS
                or depot_id in depot_to_dlc
            ):
                return
            depot_to_dlc[depot_id] = dlc_id
            lst = dlc_to_depots.setdefault(dlc_id, [])
            if depot_id not in lst:
                lst.append(depot_id)

        # 1. Local binary appinfo.vdf (authoritative Steamworks `dlcappid` metadata).
        # When target filters are supplied, compute both unfiltered and filtered sets so we
        # never select incompatible platform/arch/language depots into `-depot <ids>`, while
        # still recognizing all DLC depots in `depot_to_dlc` if they appear in raw metadata.
        unfiltered_appinfo = parse_appinfo_vdf_dlc_depots(self.steam_home(), app_id)
        if platform is not None or osarch is not None or language is not None:
            filtered_appinfo = parse_appinfo_vdf_dlc_depots(
                self.steam_home(),
                app_id,
                platform=platform,
                osarch=osarch or "64",
                language=language or "english",
                lowviolence=lowviolence,
            )
            filtered_depot_set = {
                str(did) for dep_ids in filtered_appinfo.values() for did in dep_ids
            }
            excluded_by_config = {
                str(did)
                for dep_ids in unfiltered_appinfo.values()
                for did in dep_ids
                if str(did) not in filtered_depot_set
            }
            appinfo_map = filtered_appinfo
        else:
            excluded_by_config = set()
            appinfo_map = unfiltered_appinfo

        for dlc_id, dep_ids in appinfo_map.items():
            for did in dep_ids:
                _link(str(dlc_id), str(did))

        # 2. Previously saved dlc_depots from managed installs
        if isinstance(saved_dlc_depots, dict):
            for dlc_id, dep_ids in saved_dlc_depots.items():
                if isinstance(dep_ids, list):
                    for did in dep_ids:
                        if str(did) not in excluded_by_config:
                            _link(str(dlc_id), str(did))

        # 3. Explicit dlc_app_id already on depot dicts
        for d in depots:
            if not isinstance(d, dict):
                continue
            did = str(d.get("depot_id") or "")
            explicit_dlc = str(d.get("dlc_app_id") or "")
            if did and explicit_dlc and did not in excluded_by_config:
                _link(explicit_dlc, did)

        # 4. Correlate Store `appdetails.dlc` IDs with depots enumerated under base `app_id`
        store_dlc_ids = {
            str(raw_dlc)
            for raw_dlc in (store_details.get("dlc") or [])
            if str(raw_dlc).isdigit()
        }
        base_num = int(app_id) if str(app_id).isdigit() else -1
        if store_dlc_ids:
            for d in depots:
                if not isinstance(d, dict):
                    continue
                did = str(d.get("depot_id") or "")
                if (
                    not did.isdigit()
                    or did in depot_to_dlc
                    or did in COMMON_REDIST_DEPOTS
                    or did in excluded_by_config
                ):
                    continue
                if did in store_dlc_ids:
                    _link(did, did)
                    continue
                dnum = int(did)
                prev_id = str(dnum - 1)
                if prev_id in store_dlc_ids and not (base_num >= 0 and base_num <= dnum <= base_num + 9):
                    _link(prev_id, did)
                    continue
                name = str(d.get("name") or "")
                m = re.search(r"\bdlc\s*(?:app\s*id\s*[=:]?\s*)?(\d+)\b", name, re.I)
                if m and m.group(1) in store_dlc_ids:
                    _link(m.group(1), did)

        return dlc_to_depots, depot_to_dlc

    def info(
        self,
        app_id: str,
        *,
        platform: str = "windows",
        offline: bool = False,
        branch: str | None = None,
        branch_password: str | None = None,
    ) -> GameInfo:
        resolved_id = self._resolve_app_id(app_id, offline=offline)
        account_name, _ = self._resolve_account_info()
        installed_entry = self._get_installed_entry(resolved_id)
        effective_branch = (
            branch.strip()
            if branch and branch.strip()
            else str((installed_entry or {}).get("branch") or "public")
        )

        manifest_data = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            branch=effective_branch,
            branch_password=branch_password,
            account_name=account_name,
            offline=offline,
        )
        store_details = self._fetch_store_appdetails(resolved_id, offline=offline)
        owned_app_ids = self._known_owned_app_ids(offline=offline)

        install_path_str = (
            str(installed_entry.get("install_path"))
            if installed_entry and installed_entry.get("install_path")
            else None
        )
        install_dir = Path(install_path_str) if install_path_str else None
        is_installed = bool(install_dir and install_dir.is_dir())

        raw_depots = manifest_data.get("depots") or []
        _, depot_to_dlc = self._resolve_dlc_depot_map(
            resolved_id,
            raw_depots,
            store_details,
            saved_dlc_depots=(installed_entry or {}).get("dlc_depots"),
        )
        depots: list[DepotItem] = []
        manifest_paths: list[str] = []
        total_disk = 0
        total_download = 0
        has_size = False

        for d in raw_depots:
            if not isinstance(d, dict) or not d.get("depot_id"):
                continue
            did = str(d["depot_id"])
            is_common_redist = did in COMMON_REDIST_DEPOTS
            disk_sz = d.get("disk_size_bytes")
            dl_sz = d.get("download_size_bytes")
            depots.append(
                DepotItem(
                    depot_id=did,
                    name=str(
                        d.get("name")
                        or (COMMON_REDIST_DEPOTS.get(did, {}).get("name") if is_common_redist else "")
                    ),
                    oslist=[platform],
                    osarch="64",
                    manifest_gid=str(d["manifest_gid"]) if d.get("manifest_gid") else None,
                    size_bytes=int(disk_sz) if isinstance(disk_sz, int) else None,
                    download_bytes=int(dl_sz) if isinstance(dl_sz, int) else None,
                    dlc_app_id=depot_to_dlc.get(did),
                    shared_install=is_common_redist,
                    depot_from_app=COMMON_REDIST_APP_ID if is_common_redist else None,
                    is_common_redist=is_common_redist,
                )
            )
            if not is_common_redist:
                if isinstance(disk_sz, int):
                    total_disk += disk_sz
                    has_size = True
                if isinstance(dl_sz, int):
                    total_download += dl_sz
                    has_size = True
            for f in d.get("files") or []:
                if isinstance(f, dict) and f.get("path"):
                    manifest_paths.append(str(f["path"]))

        # DLCs from Steam Store `appdetails.dlc` with honest entitlement resolution
        # against the user's owned AppIDs (never blindly marking catalog DLCs as owned).
        installed_dlcs = set((installed_entry or {}).get("dlcs") or [])
        dlcs: list[DlcItem] = []
        raw_dlc_ids = store_details.get("dlc")
        if isinstance(raw_dlc_ids, list):
            for raw_dlc in raw_dlc_ids:
                dlc_id = str(raw_dlc)
                if not dlc_id.isdigit():
                    continue
                is_owned = dlc_id in owned_app_ids or dlc_id in installed_dlcs
                dlcs.append(
                    DlcItem(
                        app_id=dlc_id,
                        title=f"Steam DLC {dlc_id}",
                        owned=is_owned,
                        installed=dlc_id in installed_dlcs,
                    )
                )

        # Redistributables + Proton-adjacent redist info
        proton_info = extract_proton_redist_info(
            depot_ids=[d.depot_id for d in depots],
            manifest_paths=manifest_paths,
            install_dir=install_dir if is_installed else None,
        )
        redists: list[RedistributableItem] = []
        seen_redists: set[tuple[str, str | None]] = set()

        for sd in proton_info.get("shared_depots", []):
            item = RedistributableItem(
                id=f"steam_redist:{sd['depot_id']}",
                name=str(sd["name"]),
                category=str(sd["category"]),
                source="common_redist_228980",
                relative_path=f"_CommonRedist/{sd['depot_id']}",
                native_in_win32_rt=bool(sd["native_in_win32_rt"]),
                present_on_disk=(
                    (install_dir / "_CommonRedist" / sd["depot_id"]).exists()
                    if is_installed and install_dir is not None
                    else None
                ),
                notes=str(sd.get("notes") or ""),
            )
            key = (item.id, item.relative_path)
            if key not in seen_redists:
                seen_redists.add(key)
                redists.append(item)

        for rel in manifest_paths:
            item = classify_redistributable_path(rel, source="manifest_files", present_on_disk=None)
            if item is not None:
                key = (item.id, item.relative_path)
                if key not in seen_redists:
                    seen_redists.add(key)
                    redists.append(item)

        for script in proton_info.get("install_scripts", []):
            for proc_entry in script.get("run_processes", []):
                proc_path = str(proc_entry.get("process") or "").replace("\\", "/")
                item = RedistributableItem(
                    id=f"installscript:{proc_entry.get('name', 'step')}",
                    name=str(proc_entry.get("name") or Path(proc_path).name or "InstallScript Step"),
                    category="other",
                    source="installscript_vdf",
                    relative_path=proc_path or script.get("relative_path"),
                    args=str(proc_entry.get("args") or ""),
                    native_in_win32_rt=bool(
                        re.search(r"(vcredist|vc_redist|dxsetup|oalinst)", proc_path, re.I)
                    ),
                    present_on_disk=True,
                    notes=f"From {script.get('relative_path')}",
                )
                key = (item.id, item.relative_path)
                if key not in seen_redists:
                    seen_redists.add(key)
                    redists.append(item)

        if is_installed and install_dir is not None:
            for item in scan_dir_redistributables(install_dir):
                key = (item.id, item.relative_path)
                if key not in seen_redists:
                    seen_redists.add(key)
                    redists.append(item)

        req_steamworks, steamworks_signals, ac_report, drm_report = self._detect_signals(
            manifest_paths,
            store_details,
            install_dir if is_installed else None,
        )

        exe = self._pick_executable_from_paths(manifest_paths)
        if not exe and is_installed and install_dir is not None:
            disk_exes = [
                p.relative_to(install_dir).as_posix()
                for p in sorted(install_dir.rglob("*.exe"))
                if not p.is_symlink()
            ]
            exe = self._pick_executable_from_paths(disk_exes)

        active_branch = str(manifest_data.get("branch") or effective_branch)
        base_raw_depots = [
            d
            for d in raw_depots
            if isinstance(d, dict)
            and str(d.get("depot_id") or "") not in depot_to_dlc
            and str(d.get("depot_id") or "") not in COMMON_REDIST_DEPOTS
        ]
        manifest_ver = self._version_from_manifests(base_raw_depots or raw_depots)
        version = manifest_ver or (installed_entry or {}).get("build_id")

        title = (
            str(store_details.get("name") or "")
            or str((installed_entry or {}).get("title") or (installed_entry or {}).get("name") or "")
            or f"Steam App {resolved_id}"
        )

        branches = [
            {
                "name": active_branch,
                "build_id": manifest_ver,
                "description": "Active queried branch (DepotDownloader)",
            }
        ]

        return GameInfo(
            store=self.name,
            app_id=resolved_id,
            title=title,
            platform=platform,
            version=str(version) if version else None,
            download_size_bytes=total_download if has_size else None,
            disk_size_bytes=(
                total_disk
                if has_size
                else (installed_entry or {}).get("size_on_disk_bytes")
            ),
            installed=is_installed,
            install_path=install_path_str if is_installed else None,
            executable=exe,
            can_run_offline=not drm_report.requires_online_auth,
            requires_eos=False,
            eos_signals=[],
            requires_steamworks=req_steamworks,
            steamworks_signals=steamworks_signals,
            anticheat=ac_report,
            drm=drm_report,
            depots=depots,
            branches=branches,
            dlcs=sorted(dlcs, key=lambda d: int(d.app_id) if d.app_id.isdigit() else 0),
            redistributables=redists,
            proton_redist_info=proton_info,
            manifest_files_checked=bool(manifest_paths),
        )

    def download(self, options: DownloadOptions) -> DownloadResult:
        account_name, steam_id64 = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(options.app_id, offline=False)
        dest = options.dest.expanduser().resolve()
        dest.mkdir(parents=True, exist_ok=True)
        branch = (options.branch or "public").strip()

        # Pre-fetch manifest metadata for the requested branch (including branch password when set)
        # so depot caches, offline `info()`, `launch_args()`, and package manifests reflect this branch.
        # Propagate branch-fallback and auth failures immediately before transferring any files.
        info: GameInfo | None = None
        try:
            info = self.info(
                resolved_id,
                platform=options.platform,
                offline=False,
                branch=branch,
                branch_password=options.branch_password,
            )
        except AuthRequiredError:
            raise
        except AcquireError as exc:
            if exc.code == "branch_fallback":
                raise
            info = None

        store_details = self._fetch_store_appdetails(resolved_id, offline=True)
        existing_entry = self._get_installed_entry(resolved_id)
        info_depot_dicts = (
            [
                {
                    "depot_id": d.depot_id,
                    "name": d.name,
                    "manifest_gid": d.manifest_gid,
                    "dlc_app_id": d.dlc_app_id,
                }
                for d in info.depots
            ]
            if info is not None
            else []
        )
        dd_language, dd_lowviolence = self._extract_dd_filter_options(options.extra_args)
        dlc_to_depots, depot_to_dlc = self._resolve_dlc_depot_map(
            resolved_id,
            info_depot_dicts,
            store_details,
            saved_dlc_depots=(existing_entry or {}).get("dlc_depots"),
            platform=options.platform,
            osarch="64",
            language=dd_language,
            lowviolence=dd_lowviolence,
        )

        # Validate requested DLC ownership upfront before transferring payloads.
        target_dlcs: list[str] = []
        if options.with_dlcs or options.dlcs:
            owned_ids = self._known_owned_app_ids(offline=False)
            if options.dlcs:
                target_dlcs = list(options.dlcs)
                for dlc_id in target_dlcs:
                    if owned_ids and dlc_id not in owned_ids:
                        raise AcquireError(
                            f"DLC AppID {dlc_id} is not owned by the logged-in Steam account.",
                            code="dlc_not_owned",
                            details={"dlc_app_id": dlc_id},
                        )
            elif info is not None:
                # Only select DLCs confirmed owned by the account
                target_dlcs = [d.app_id for d in info.dlcs if d.owned]

        def _build_dd_args(depot_ids: list[str] | None = None) -> list[str]:
            args: list[str] = ["-app", resolved_id]
            if depot_ids:
                args.append("-depot")
                args.extend(depot_ids)
            args.extend(
                [
                    "-os",
                    options.platform,
                    "-osarch",
                    "64",
                    "-branch",
                    branch,
                    "-dir",
                    str(dest),
                    "-username",
                    account_name,
                    "-remember-password",
                ]
            )
            if options.branch_password:
                args.extend(["-branchpassword", options.branch_password])
            if options.validate:
                args.append("-validate")
            if options.max_workers:
                args.extend(["-max-downloads", str(options.max_workers)])
            args.extend(options.extra_args)
            return args

        base_depot_ids = [
            d.depot_id
            for d in (info.depots if info is not None else [])
            if not d.is_common_redist and d.depot_id not in depot_to_dlc
        ]
        known_dlc_depot_ids = set(depot_to_dlc.keys())

        info_depot_ids = {d.depot_id for d in (info.depots if info is not None else [])}
        requested_dlc_depot_ids: list[str] = []
        dlc_depots_by_dlc: dict[str, list[str]] = {}
        for dlc_id in target_dlcs:
            dep_ids = list(dlc_to_depots.get(dlc_id) or [])
            if not dep_ids and info is not None and any(d.depot_id == dlc_id for d in info.depots):
                dep_ids = [dlc_id]
            # When the platform-filtered `-manifest-only` query returned depots for this DLC,
            # intersect against `info_depot_ids` so explicit `-depot` never selects a depot
            # excluded by DepotDownloader's own platform/arch/language filters.
            if info_depot_ids and dep_ids:
                confirmed = [did for did in dep_ids if did in info_depot_ids]
                if confirmed:
                    dep_ids = confirmed
            dlc_depots_by_dlc[dlc_id] = dep_ids
            for did in dep_ids:
                if did not in requested_dlc_depot_ids:
                    requested_dlc_depot_ids.append(did)

        all_requested_dlcs_mapped = all(bool(dlc_depots_by_dlc.get(d)) for d in options.dlcs)

        # Pass all selected depots together in ONE DepotDownloader invocation
        # (`-app <base_id> -depot <id1> <id2> ...`) so DepotDownloader's cross-depot
        # cleanup (`allFileNamesAllDepots`) and last-depot-wins precedence stay intact.
        selected_depot_ids: list[str] | None = None
        if base_depot_ids and known_dlc_depot_ids:
            if not options.with_dlcs and not options.dlcs:
                selected_depot_ids = list(base_depot_ids)
            elif options.dlcs and all_requested_dlcs_mapped:
                selected_depot_ids = list(base_depot_ids) + [
                    did for did in requested_dlc_depot_ids if did not in base_depot_ids
                ]
            elif options.with_dlcs and requested_dlc_depot_ids:
                selected_depot_ids = list(base_depot_ids) + [
                    did for did in requested_dlc_depot_ids if did not in base_depot_ids
                ]

        proc = self._run_depot_downloader(
            _build_dd_args(depot_ids=selected_depot_ids),
            interactive=False,
            timeout=None,
            check=True,
        )
        combined = proc.stdout + "\n" + proc.stderr
        out = parse_depot_downloader_stdout(combined)
        _check_branch_fallback(
            branch,
            combined,
            out.get("branch"),
            app_id=resolved_id,
            context="download",
        )
        skipped_depots = set(_SKIPPED_DEPOT_RE.findall(combined))

        all_stdout_depots: list[dict[str, Any]] = []
        downloaded_depots_by_id: dict[str, dict[str, Any]] = {}
        for dep in out.get("depots", []):
            did = str(dep.get("depot_id") or "")
            if did:
                downloaded_depots_by_id[did] = dep
                all_stdout_depots.append(dep)

        # Verify DepotDownloader actually processed the required depots; DepotDownloader can
        # exit 0 after skipping inaccessible depots ("Depot X is not available from this account.").
        required_depots = (
            list(selected_depot_ids)
            if selected_depot_ids is not None
            else [did for did in requested_dlc_depot_ids if did not in downloaded_depots_by_id]
        )
        missing_depots = [did for did in required_depots if did not in downloaded_depots_by_id]
        if selected_depot_ids is None and not downloaded_depots_by_id:
            if skipped_depots or _LOGIN_FAIL_PATTERNS.search(combined):
                raise AuthRequiredError(
                    f"DepotDownloader had no accessible depots for Steam AppID {resolved_id} "
                    f"(skipped: {', '.join(sorted(skipped_depots)) or 'unknown'}).",
                    details={"skipped_depots": sorted(skipped_depots)},
                )
            raise CommandFailedError(
                f"DepotDownloader exited 0 without processing any depots for Steam AppID {resolved_id}."
            )
        if missing_depots:
            if any(did in skipped_depots for did in missing_depots) or _LOGIN_FAIL_PATTERNS.search(combined):
                raise AuthRequiredError(
                    f"DepotDownloader skipped required depot(s) {', '.join(missing_depots)} for "
                    f"Steam AppID {resolved_id} (account lacks entitlement or depot key).",
                    details={"missing_depots": missing_depots, "skipped_depots": sorted(skipped_depots)},
                )
            raise CommandFailedError(
                f"DepotDownloader did not process required depot(s) {', '.join(missing_depots)} "
                f"for Steam AppID {resolved_id}.",
                details={"missing_depots": missing_depots},
            )
        skipped_base = [did for did in base_depot_ids if did in skipped_depots and did not in downloaded_depots_by_id]
        if skipped_base:
            raise AuthRequiredError(
                f"DepotDownloader skipped base depot(s) {', '.join(skipped_base)} for "
                f"Steam AppID {resolved_id}.",
                details={"skipped_depots": skipped_base},
            )

        # Record downloaded DLC depots & manifest versions ONLY when depot transfer evidence exists.
        downloaded_dlcs: list[str] = []
        dlc_versions: dict[str, str] = {}
        dlc_depots_recorded: dict[str, list[str]] = {}
        explicit_dlc_set = {str(d) for d in options.dlcs}
        if target_dlcs:
            dlc_to_depots, depot_to_dlc = self._resolve_dlc_depot_map(
                resolved_id,
                info_depot_dicts + all_stdout_depots,
                store_details,
                saved_dlc_depots=(existing_entry or {}).get("dlc_depots"),
                platform=options.platform,
                osarch="64",
                language=dd_language,
                lowviolence=dd_lowviolence,
            )
            for dlc_id in target_dlcs:
                dep_ids = list(dlc_to_depots.get(dlc_id) or dlc_depots_by_dlc.get(dlc_id) or [])
                this_dlc_depots = [
                    downloaded_depots_by_id[did]
                    for did in dep_ids
                    if did in downloaded_depots_by_id
                ]
                if not this_dlc_depots:
                    if dlc_id in explicit_dlc_set:
                        raise AcquireError(
                            f"Could not resolve or confirm depot transfer for requested DLC AppID "
                            f"{dlc_id} under base Steam AppID {resolved_id} (ensure local Steam "
                            "appcache/appinfo.vdf is present or that the DLC has an accessible "
                            "depot under the base app).",
                            code="dlc_depot_not_found",
                            details={"app_id": resolved_id, "dlc_app_id": dlc_id},
                        )
                    # Do not falsely record an unverified/unmapped DLC as installed.
                    continue
                dlc_depots_recorded[dlc_id] = [
                    str(d["depot_id"]) for d in this_dlc_depots if d.get("depot_id")
                ]
                dlc_ver = self._version_from_manifests(this_dlc_depots)
                if dlc_ver:
                    dlc_versions[dlc_id] = dlc_ver
                if dlc_id not in downloaded_dlcs:
                    downloaded_dlcs.append(dlc_id)

        downloaded_depots: list[str] = list(downloaded_depots_by_id.keys())

        # Steamworks Common Redistributables (`AppID 228980`) depots when requested
        downloaded_redists: list[str] = []
        if options.with_redists and info is not None:
            shared_depots = info.proton_redist_info.get("shared_depots") or []
            for sd in shared_depots:
                did = str(sd.get("depot_id") or "")
                if not did.isdigit():
                    continue
                redist_dst = dest / "_CommonRedist" / did
                redist_dst.mkdir(parents=True, exist_ok=True)
                redist_args = [
                    "-app",
                    COMMON_REDIST_APP_ID,
                    "-depot",
                    did,
                    "-os",
                    options.platform,
                    "-osarch",
                    "64",
                    "-dir",
                    str(redist_dst),
                    "-username",
                    account_name,
                    "-remember-password",
                ]
                redist_proc = self._run_depot_downloader(
                    redist_args, interactive=False, timeout=None, check=True
                )
                redist_combined = redist_proc.stdout + "\n" + redist_proc.stderr
                redist_out = parse_depot_downloader_stdout(redist_combined)
                redist_processed = {
                    str(d.get("depot_id") or "") for d in redist_out.get("depots", []) if d.get("depot_id")
                }
                if did not in redist_processed:
                    if _SKIPPED_DEPOT_RE.search(redist_combined) or _LOGIN_FAIL_PATTERNS.search(redist_combined):
                        raise AuthRequiredError(
                            f"DepotDownloader skipped Common Redistributable depot {did} (App {COMMON_REDIST_APP_ID})."
                        )
                    raise CommandFailedError(
                        f"DepotDownloader did not process Common Redistributable depot {did} (App {COMMON_REDIST_APP_ID})."
                    )
                if did not in downloaded_redists:
                    downloaded_redists.append(did)

        # Ensure standard `steam_appid.txt` exists for downstream packaging/runtime
        appid_txt = dest / "steam_appid.txt"
        if not appid_txt.exists():
            appid_txt.write_text(f"{resolved_id}\n", encoding="utf-8")

        manifest_gids = {
            did: str(d["manifest_gid"])
            for did, d in downloaded_depots_by_id.items()
            if d.get("manifest_gid")
        }
        base_stdout_depots = [
            d
            for did, d in downloaded_depots_by_id.items()
            if did not in depot_to_dlc and did not in COMMON_REDIST_DEPOTS
        ]
        version = self._version_from_manifests(
            base_stdout_depots or list(downloaded_depots_by_id.values())
        ) or (info.version if info else None)
        title = info.title if info else f"Steam App {resolved_id}"

        if not options.no_register:
            installs = self._load_installs()
            existing = installs.get(resolved_id)
            same_install = False
            if isinstance(existing, dict) and existing.get("install_path"):
                try:
                    same_install = Path(str(existing["install_path"])).expanduser().resolve() == dest
                except OSError:
                    same_install = str(existing["install_path"]) == str(dest)

            merged_dlcs: list[str] = []
            merged_dlc_versions: dict[str, str] = {}
            merged_dlc_depots: dict[str, list[str]] = {}
            merged_redists: list[str] = []
            merged_depots: list[str] = list(downloaded_depots)
            merged_manifest_gids: dict[str, str] = dict(manifest_gids)
            if same_install and isinstance(existing, dict):
                for prev_dlc in existing.get("dlcs") or []:
                    s_dlc = str(prev_dlc)
                    if s_dlc and s_dlc not in merged_dlcs:
                        merged_dlcs.append(s_dlc)
                prev_vers = existing.get("dlc_versions")
                if isinstance(prev_vers, dict):
                    for k, v in prev_vers.items():
                        if v:
                            merged_dlc_versions[str(k)] = str(v)
                prev_ddeps = existing.get("dlc_depots")
                if isinstance(prev_ddeps, dict):
                    for k, v in prev_ddeps.items():
                        if isinstance(v, list):
                            merged_dlc_depots[str(k)] = [str(x) for x in v if str(x).isdigit()]
                for prev_redist in existing.get("redists") or []:
                    s_red = str(prev_redist)
                    if s_red and s_red not in merged_redists:
                        merged_redists.append(s_red)

            for dlc_id in downloaded_dlcs:
                if dlc_id not in merged_dlcs:
                    merged_dlcs.append(dlc_id)
            merged_dlc_versions.update(dlc_versions)
            merged_dlc_depots.update(dlc_depots_recorded)
            for redist_id in downloaded_redists:
                if redist_id not in merged_redists:
                    merged_redists.append(redist_id)

            # Preserve untouched installed DLC depots and their manifest GIDs during base-only
            # updates so subsequent `verify --repair` keeps repairing installed DLCs too.
            if same_install and isinstance(existing, dict):
                prev_gids = existing.get("manifest_gids")
                prev_gids_map = prev_gids if isinstance(prev_gids, dict) else {}
                for dep_list in merged_dlc_depots.values():
                    for did in dep_list:
                        if did not in merged_depots:
                            merged_depots.append(did)
                        if did not in merged_manifest_gids and prev_gids_map.get(did):
                            merged_manifest_gids[did] = str(prev_gids_map[did])

            branch_requires_password = bool(options.branch_password)
            if (
                not branch_requires_password
                and same_install
                and isinstance(existing, dict)
                and str(existing.get("branch") or "public").lower() == branch.lower()
            ):
                branch_requires_password = bool(existing.get("branch_requires_password"))

            new_entry: dict[str, Any] = {
                "app_id": resolved_id,
                "title": title,
                "account_name": account_name,
                "steam_id64": steam_id64,
                "install_path": str(dest),
                "platform": options.platform,
                "branch": branch,
                "branch_requires_password": branch_requires_password,
                "build_id": version,
                "manifest_gids": merged_manifest_gids,
                "depots": merged_depots,
                "dlcs": merged_dlcs,
                "dlc_versions": merged_dlc_versions,
                "dlc_depots": merged_dlc_depots,
                "redists": merged_redists,
                "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
            if same_install and isinstance(existing, dict) and existing.get("launch_args"):
                new_entry["launch_args"] = existing["launch_args"]

            installs[resolved_id] = new_entry
            self._save_installs(installs)

        return DownloadResult(
            store=self.name,
            app_id=resolved_id,
            title=title,
            platform=options.platform,
            install_path=str(dest),
            version=version,
            branch=branch,
            downloaded_dlcs=downloaded_dlcs,
            downloaded_depots=downloaded_depots,
            downloaded_redists=downloaded_redists,
        )

    def _verify_files_readonly(
        self,
        install_path: Path,
        depots: list[dict[str, Any]],
    ) -> tuple[int, list[str], list[str]]:
        """Check local files against depot manifest sizes and SHA-1 digests without writing to disk.

        Overlapping relative paths across multiple depots follow DepotDownloader's
        last-depot-wins precedence.
        """
        install_root = install_path.resolve()
        winning_files: dict[str, tuple[str, dict[str, Any]]] = {}

        for depot in depots:
            if not isinstance(depot, dict):
                continue
            for entry in depot.get("files") or []:
                if not isinstance(entry, dict):
                    continue
                flags = int(entry.get("flags") or 0)
                # EDepotFileFlag.Directory (0x40)
                if flags & 0x40:
                    continue
                rel = str(entry.get("path") or "").replace("\\", "/").strip().lstrip("/")
                if not rel or rel.endswith("/"):
                    continue
                rel_path = Path(rel)
                if rel_path.is_absolute() or ".." in rel_path.parts:
                    continue
                # Last-depot-wins (case-insensitive for Windows depot paths)
                winning_files[rel.lower()] = (rel, entry)

        checked = 0
        missing_files: list[str] = []
        corrupted_files: list[str] = []

        for rel, entry in winning_files.values():
            target = install_path / Path(rel)
            try:
                target.resolve().relative_to(install_root)
            except (OSError, ValueError):
                checked += 1
                corrupted_files.append(rel)
                continue

            checked += 1
            if not target.is_file():
                missing_files.append(rel)
                continue

            expected_size = entry.get("size_bytes")
            try:
                actual_size = target.stat().st_size
            except OSError:
                corrupted_files.append(rel)
                continue

            if isinstance(expected_size, int) and actual_size != expected_size:
                corrupted_files.append(rel)
                continue

            expected_sha1 = str(entry.get("sha1") or "").lower()
            if len(expected_sha1) == 40 and expected_sha1 != "0" * 40:
                digest = hashlib.sha1()
                try:
                    with target.open("rb") as fh:
                        while True:
                            chunk = fh.read(1024 * 1024)
                            if not chunk:
                                break
                            digest.update(chunk)
                except OSError:
                    corrupted_files.append(rel)
                    continue
                if digest.hexdigest() != expected_sha1:
                    corrupted_files.append(rel)

        return checked, missing_files, corrupted_files

    def verify(
        self,
        app_id: str,
        *,
        repair: bool = False,
        branch_password: str | None = None,
    ) -> VerifyResult:
        account_name, _ = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(app_id, offline=True, strict=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not registered as installed. "
                "Download it first with `tools/port-acquire --store steam download <appid>`."
            )
        install_path = Path(str(entry["install_path"]))
        unmanaged_acf = self._is_unmanaged_steam_client_install(resolved_id, entry)
        if unmanaged_acf:
            if repair:
                raise AcquireError(
                    f"Steam AppID {resolved_id} is an unmanaged Steam-client installation "
                    f"({install_path}); refusing to repair it in-place with DepotDownloader. "
                    "Verify it via the Steam client or download a separate managed Windows copy "
                    "with `tools/port-acquire --store steam download <appid> --platform windows`.",
                    code="unmanaged_steam_install",
                    details={"app_id": resolved_id, "install_path": str(install_path)},
                )
            detected_plat = self._detect_acf_install_platform(install_path)
            if detected_plat == "macos":
                raise AcquireError(
                    f"Steam AppID {resolved_id} at {install_path} is a native macOS Steam-client "
                    "installation; refusing to verify it against Windows manifests.",
                    code="unmanaged_steam_install",
                    details={"app_id": resolved_id, "install_path": str(install_path), "platform": "macos"},
                )
        if not install_path.is_dir():
            if not repair:
                return VerifyResult(
                    store=self.name,
                    app_id=resolved_id,
                    ok=False,
                    repaired=False,
                    missing_files=[str(install_path)],
                    message=f"Install directory missing: {install_path}",
                )
            install_path.mkdir(parents=True, exist_ok=True)

        platform = str(entry.get("platform") or "windows")
        branch = str(entry.get("branch") or "public")
        if entry.get("branch_requires_password") and not branch_password:
            raise AcquireError(
                f"Steam AppID {resolved_id} is installed on password-protected branch {branch!r}; "
                "pass `--branch-password-env <VAR>` to verify or repair without falling back to public.",
                code="missing_branch_password",
                details={"app_id": resolved_id, "branch": branch},
            )

        recorded_depots: list[str] = []
        raw_rec = entry.get("depots")
        if isinstance(raw_rec, list):
            recorded_depots = [
                str(d) for d in raw_rec if str(d).isdigit() and str(d) not in COMMON_REDIST_DEPOTS
            ]
        elif isinstance(entry.get("mounted_depots"), dict):
            recorded_depots = [
                str(d)
                for d in entry["mounted_depots"].keys()
                if str(d).isdigit() and str(d) not in COMMON_REDIST_DEPOTS
            ]
        raw_dlc_depots = entry.get("dlc_depots")
        if isinstance(raw_dlc_depots, dict):
            for dep_list in raw_dlc_depots.values():
                if isinstance(dep_list, list):
                    for d in dep_list:
                        sd = str(d)
                        if sd.isdigit() and sd not in COMMON_REDIST_DEPOTS and sd not in recorded_depots:
                            recorded_depots.append(sd)

        if not repair:
            # Read-only verification: query fresh depot manifests via `-manifest-only`
            # and verify file presence, size, and SHA-1 locally without downloading or
            # overwriting any files in `install_path`.
            manifest_data = self._fetch_depot_manifests(
                resolved_id,
                platform=platform,
                branch=branch,
                branch_password=branch_password,
                account_name=account_name,
                offline=False,
                allow_cache_fallback=False,
                update_cache=False,
            )
            all_depots = manifest_data.get("depots") or []
            skipped_depots = {
                did
                for did in (manifest_data.get("skipped_depots") or [])
                if str(did) not in COMMON_REDIST_DEPOTS
            }
            # Steamworks Common Redistributables (App 228980) are managed separately under
            # `_CommonRedist/<depot_id>` only when `--with-redists` is requested; exclude
            # them from the base game-root verification set.
            installed_redists = {str(x) for x in (entry.get("redists") or []) if str(x).isdigit()}
            redist_depots = [
                d
                for d in all_depots
                if isinstance(d, dict) and str(d.get("depot_id") or "") in COMMON_REDIST_DEPOTS
            ]
            depots = [
                d
                for d in all_depots
                if isinstance(d, dict) and str(d.get("depot_id") or "") not in COMMON_REDIST_DEPOTS
            ]
            missing_file_manifests = [
                str(d.get("depot_id"))
                for d in depots
                if not d.get("has_file_manifest", bool(d.get("files")))
            ]
            if not depots or missing_file_manifests:
                raise CommandFailedError(
                    f"Incomplete depot file manifests for Steam AppID {resolved_id} "
                    f"(branch {branch!r}); missing file lists for depots: "
                    f"{', '.join(missing_file_manifests) or 'none'}."
                )
            # Exclude depots belonging to DLCs that were not installed in this installation
            if entry.get("depots"):
                store_details = self._fetch_store_appdetails(resolved_id, offline=True)
                _, depot_to_dlc = self._resolve_dlc_depot_map(
                    resolved_id,
                    depots,
                    store_details,
                    saved_dlc_depots=entry.get("dlc_depots"),
                )
                installed_dlcs = {str(x) for x in (entry.get("dlcs") or [])}
                depots = [
                    d
                    for d in depots
                    if not (
                        str(d.get("depot_id") or "") in depot_to_dlc
                        and depot_to_dlc[str(d.get("depot_id") or "")] not in installed_dlcs
                    )
                ]

            # Require complete coverage of the installation's recorded depots before reporting OK.
            returned_depot_ids = {
                str(d.get("depot_id")) for d in depots if d.get("depot_id")
            }
            missing_required = [did for did in recorded_depots if did not in returned_depot_ids]
            if missing_required:
                if any(did in skipped_depots for did in missing_required):
                    raise AuthRequiredError(
                        f"Cannot verify Steam AppID {resolved_id}: current account lacks access to "
                        f"installed depot(s) {', '.join(missing_required)}.",
                        details={
                            "missing_depots": missing_required,
                            "skipped_depots": sorted(skipped_depots),
                        },
                    )
                raise CommandFailedError(
                    f"Incomplete depot coverage for Steam AppID {resolved_id}: installed depot(s) "
                    f"{', '.join(missing_required)} were omitted from manifest query.",
                    details={"missing_depots": missing_required},
                )
            if not recorded_depots and skipped_depots:
                raise AuthRequiredError(
                    f"Cannot verify Steam AppID {resolved_id}: DepotDownloader skipped depot(s) "
                    f"{', '.join(sorted(skipped_depots))} (not available from this account).",
                    details={"skipped_depots": sorted(skipped_depots)},
                )

            checked, missing_files, corrupted_files = self._verify_files_readonly(
                install_path,
                depots,
            )
            # Verify every separately downloaded Common Redistributable (`entry["redists"]`)
            # against `_CommonRedist/<depot_id>`. Query App 228980 explicitly when a recorded
            # redistributable is absent from the base app's manifest response or lacks a file
            # manifest, and fail honestly instead of silently skipping incomplete coverage.
            redist_depots_by_id = {
                str(rdep.get("depot_id") or ""): rdep
                for rdep in redist_depots
                if rdep.get("depot_id")
            }
            for rdid in sorted(installed_redists, key=lambda x: int(x) if x.isdigit() else 0):
                rdep = redist_depots_by_id.get(rdid)
                if rdep is None or not rdep.get("has_file_manifest", bool(rdep.get("files"))):
                    r_manifest = self._fetch_depot_manifests(
                        COMMON_REDIST_APP_ID,
                        depot_id=rdid,
                        platform=platform,
                        branch="public",
                        account_name=account_name,
                        offline=False,
                        allow_cache_fallback=False,
                        update_cache=False,
                    )
                    r_skipped = set(r_manifest.get("skipped_depots") or [])
                    r_found = [
                        d
                        for d in (r_manifest.get("depots") or [])
                        if isinstance(d, dict) and str(d.get("depot_id") or "") == rdid
                    ]
                    rdep = r_found[0] if r_found else None
                    if rdep is None or not rdep.get("has_file_manifest", bool(rdep.get("files"))):
                        if rdid in r_skipped:
                            raise AuthRequiredError(
                                f"Cannot verify Steam AppID {resolved_id}: current account lacks access to "
                                f"installed redistributable depot {rdid} (App {COMMON_REDIST_APP_ID}).",
                                details={"missing_depots": [rdid], "skipped_depots": sorted(r_skipped)},
                            )
                        raise CommandFailedError(
                            f"Incomplete redistributable manifest coverage for Steam AppID {resolved_id}: "
                            f"could not obtain file manifest for installed redistributable depot {rdid}.",
                            details={"missing_depots": [rdid]},
                        )
                r_root = install_path / "_CommonRedist" / rdid
                if not r_root.is_dir():
                    missing_files.append(f"_CommonRedist/{rdid}")
                    continue
                r_chk, r_miss, r_corr = self._verify_files_readonly(r_root, [rdep])
                checked += r_chk
                missing_files.extend(f"_CommonRedist/{rdid}/{p}" for p in r_miss)
                corrupted_files.extend(f"_CommonRedist/{rdid}/{p}" for p in r_corr)
            if checked == 0:
                raise CommandFailedError(
                    f"Could not obtain depot file manifest entries for Steam AppID {resolved_id} "
                    f"(branch {branch!r}) to verify."
                )
            ok = not missing_files and not corrupted_files
            return VerifyResult(
                store=self.name,
                app_id=resolved_id,
                ok=ok,
                repaired=False,
                missing_files=missing_files,
                corrupted_files=corrupted_files,
                message=(
                    f"Verified {checked} files OK (read-only)"
                    if ok
                    else (
                        f"Verification failed (read-only): {len(missing_files)} missing, "
                        f"{len(corrupted_files)} corrupted (re-run with --repair to fix)"
                    )
                ),
            )

        dd_args = [
            "-app",
            resolved_id,
        ]
        if recorded_depots:
            dd_args.append("-depot")
            dd_args.extend(recorded_depots)
        dd_args.extend(
            [
                "-os",
                platform,
                "-osarch",
                "64",
                "-branch",
                branch,
                "-dir",
                str(install_path),
                "-username",
                account_name,
                "-remember-password",
                "-validate",
            ]
        )
        if branch_password:
            dd_args.extend(["-branchpassword", branch_password])
        proc = self._run_depot_downloader(dd_args, interactive=False, timeout=None, check=False)
        combined = proc.stdout + "\n" + proc.stderr
        parsed_out = parse_depot_downloader_stdout(combined)
        _check_branch_fallback(
            branch,
            combined,
            parsed_out.get("branch"),
            app_id=resolved_id,
            context="repair",
        )
        skipped_repair = set(_SKIPPED_DEPOT_RE.findall(combined))
        processed_repair_list = [
            d for d in parsed_out.get("depots", []) if isinstance(d, dict) and d.get("depot_id")
        ]
        processed_repair = {str(d["depot_id"]): d for d in processed_repair_list}
        missing_repair = [did for did in recorded_depots if did not in processed_repair]
        if proc.returncode == 0 and missing_repair:
            if any(did in skipped_repair for did in missing_repair):
                raise AuthRequiredError(
                    f"Repair skipped installed depot(s) {', '.join(missing_repair)} for Steam AppID {resolved_id}.",
                    details={"missing_depots": missing_repair, "skipped_depots": sorted(skipped_repair)},
                )
            raise CommandFailedError(
                f"Repair did not process installed depot(s) {', '.join(missing_repair)} for Steam AppID {resolved_id}.",
                details={"missing_depots": missing_repair},
            )
        ok = proc.returncode == 0
        redist_repaired_bytes = 0
        installed_redists_list = list(
            dict.fromkeys(str(x) for x in (entry.get("redists") or []) if str(x).isdigit())
        )
        if ok and installed_redists_list:
            # Repair separately managed Common Redistributables (`AppID 228980`) at their
            # actual `_CommonRedist/<depot_id>` destinations before reporting overall success.
            for rdid in installed_redists_list:
                redist_dst = install_path / "_CommonRedist" / rdid
                redist_dst.mkdir(parents=True, exist_ok=True)
                redist_args = [
                    "-app",
                    COMMON_REDIST_APP_ID,
                    "-depot",
                    rdid,
                    "-os",
                    platform,
                    "-osarch",
                    "64",
                    "-dir",
                    str(redist_dst),
                    "-username",
                    account_name,
                    "-remember-password",
                    "-validate",
                ]
                r_proc = self._run_depot_downloader(
                    redist_args,
                    interactive=False,
                    timeout=None,
                    check=False,
                )
                r_combined = r_proc.stdout + "\n" + r_proc.stderr
                r_parsed = parse_depot_downloader_stdout(r_combined)
                r_skipped = set(_SKIPPED_DEPOT_RE.findall(r_combined))
                r_processed = {
                    str(d["depot_id"])
                    for d in r_parsed.get("depots", [])
                    if isinstance(d, dict) and d.get("depot_id")
                }
                if r_proc.returncode != 0:
                    ok = False
                    combined = combined + "\n" + r_combined
                    break
                if rdid not in r_processed:
                    if rdid in r_skipped:
                        raise AuthRequiredError(
                            f"Repair skipped installed redistributable depot {rdid} for Steam AppID {resolved_id}.",
                            details={"missing_depots": [rdid], "skipped_depots": sorted(r_skipped)},
                        )
                    raise CommandFailedError(
                        f"Repair did not process installed redistributable depot {rdid} for Steam AppID {resolved_id}.",
                        details={"missing_depots": [rdid]},
                    )
                redist_repaired_bytes += int(r_parsed.get("total_downloaded_bytes") or 0)

        if ok:
            # DepotDownloader `-validate` downloads the current manifest for `branch` when no
            # manifest GID is pinned; synchronize the installation registry and cached manifest
            # file list so subsequent `info`, `launch-args`, and `update` reflect repaired files.
            try:
                self._fetch_depot_manifests(
                    resolved_id,
                    platform=platform,
                    branch=branch,
                    branch_password=branch_password,
                    account_name=account_name,
                    offline=False,
                    allow_cache_fallback=True,
                    update_cache=True,
                )
            except AcquireError:
                pass

            installs = self._load_installs()
            managed = installs.get(resolved_id)
            if isinstance(managed, dict) and processed_repair:
                store_details = self._fetch_store_appdetails(resolved_id, offline=True)
                dlc_to_depots, depot_to_dlc = self._resolve_dlc_depot_map(
                    resolved_id,
                    processed_repair_list,
                    store_details,
                    saved_dlc_depots=managed.get("dlc_depots"),
                )
                gids = (
                    dict(managed["manifest_gids"])
                    if isinstance(managed.get("manifest_gids"), dict)
                    else {}
                )
                for did, dep in processed_repair.items():
                    if dep.get("manifest_gid"):
                        gids[did] = str(dep["manifest_gid"])
                managed["manifest_gids"] = gids

                base_repaired = [
                    dep
                    for did, dep in processed_repair.items()
                    if did not in depot_to_dlc and did not in COMMON_REDIST_DEPOTS
                ]
                new_ver = self._version_from_manifests(base_repaired or processed_repair_list)
                if new_ver:
                    managed["build_id"] = new_ver

                dlc_vers = (
                    dict(managed["dlc_versions"])
                    if isinstance(managed.get("dlc_versions"), dict)
                    else {}
                )
                for dlc_id, dep_ids in dlc_to_depots.items():
                    dlc_deps = [processed_repair[did] for did in dep_ids if did in processed_repair]
                    dver = self._version_from_manifests(dlc_deps)
                    if dver:
                        dlc_vers[dlc_id] = dver
                managed["dlc_versions"] = dlc_vers
                managed["updated_at_utc"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
                installs[resolved_id] = managed
                self._save_installs(installs)

        total_repaired_bytes = int(parsed_out.get("total_downloaded_bytes") or 0) + redist_repaired_bytes
        repaired_flag = bool(repair and ok and total_repaired_bytes > 0)
        msg_lines = [ln.strip() for ln in combined.splitlines() if ln.strip()]
        return VerifyResult(
            store=self.name,
            app_id=resolved_id,
            ok=ok,
            repaired=repaired_flag,
            message=msg_lines[-1] if msg_lines else ("Repaired OK via DepotDownloader -validate" if ok else "Repair failed"),
        )

    def update(
        self,
        app_id: str,
        *,
        check_only: bool = False,
        with_dlcs: bool = False,
        branch_password: str | None = None,
    ) -> UpdateResult:
        account_name, _ = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(app_id, offline=False, strict=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not installed; use `download` instead of `update`."
            )
        install_path = Path(str(entry["install_path"]))
        unmanaged_acf = self._is_unmanaged_steam_client_install(resolved_id, entry)
        if unmanaged_acf:
            if not check_only:
                raise AcquireError(
                    f"Steam AppID {resolved_id} is an unmanaged Steam-client installation "
                    f"({install_path}); refusing to overwrite it in-place with DepotDownloader. "
                    "Download a separate managed Windows copy with "
                    "`tools/port-acquire --store steam download <appid> --platform windows`.",
                    code="unmanaged_steam_install",
                    details={"app_id": resolved_id, "install_path": str(install_path)},
                )
            detected_plat = self._detect_acf_install_platform(install_path)
            if detected_plat == "macos":
                raise AcquireError(
                    f"Steam AppID {resolved_id} at {install_path} is a native macOS Steam-client "
                    "installation; refusing to check Windows updates against it.",
                    code="unmanaged_steam_install",
                    details={"app_id": resolved_id, "install_path": str(install_path), "platform": "macos"},
                )
        platform = str(entry.get("platform") or "windows")
        branch = str(entry.get("branch") or "public")
        old_version = str(entry.get("build_id") or "") or None
        if entry.get("branch_requires_password") and not branch_password:
            raise AcquireError(
                f"Steam AppID {resolved_id} is installed on password-protected branch {branch!r}; "
                "pass `--branch-password-env <VAR>` to update without falling back to public.",
                code="missing_branch_password",
                details={"app_id": resolved_id, "branch": branch},
            )

        # Always fetch fresh remote manifests under base `resolved_id` without overwriting the
        # installed manifest cache so `--check-only` cannot clobber installed launch metadata.
        remote_manifests = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            branch=branch,
            branch_password=branch_password,
            account_name=account_name,
            offline=False,
            allow_cache_fallback=False,
            update_cache=False,
        )
        remote_depots = remote_manifests.get("depots") or []
        store_details = self._fetch_store_appdetails(resolved_id, offline=not with_dlcs)
        dlc_to_depots, depot_to_dlc = self._resolve_dlc_depot_map(
            resolved_id,
            remote_depots,
            store_details,
            saved_dlc_depots=entry.get("dlc_depots"),
            platform=platform,
            osarch="64",
            language="english",
        )
        remote_base_depots = [
            d
            for d in remote_depots
            if isinstance(d, dict)
            and str(d.get("depot_id") or "") not in depot_to_dlc
            and str(d.get("depot_id") or "") not in COMMON_REDIST_DEPOTS
        ]
        remote_version = self._version_from_manifests(remote_base_depots or remote_depots)

        # For Steam-client installs discovered from `appmanifest_*.acf`, `entry["build_id"]`
        # is Steam's app buildid rather than a depot manifest GID fingerprint. Derive `old_version`
        # from `mounted_depots` so both sides compare matching depot manifest GIDs.
        mounted_depots = entry.get("mounted_depots")
        if isinstance(mounted_depots, dict) and mounted_depots and not entry.get("manifest_gids"):
            acf_all = [
                {"depot_id": str(did), "manifest_gid": str(gid)}
                for did, gid in mounted_depots.items()
                if str(did).isdigit() and str(gid) and str(did) not in COMMON_REDIST_DEPOTS
            ]
            acf_base = [d for d in acf_all if d["depot_id"] not in depot_to_dlc]
            acf_ver = self._version_from_manifests(acf_base or acf_all)
            if acf_ver:
                old_version = acf_ver

        # When `--with-dlcs` is requested, also check owned DLC depots under base `resolved_id`
        # (never querying `-app <dlc_id>`) for newly owned/uninstalled DLCs or updated manifests.
        dlc_updates_needed: list[str] = []
        if with_dlcs:
            installed_dlcs = {str(x) for x in (entry.get("dlcs") or []) if str(x).isdigit()}
            raw_dlc_vers = entry.get("dlc_versions")
            installed_dlc_versions: dict[str, str] = (
                {str(k): str(v) for k, v in raw_dlc_vers.items() if v}
                if isinstance(raw_dlc_vers, dict)
                else {}
            )
            if isinstance(mounted_depots, dict) and mounted_depots:
                for dlc_id, dep_ids in dlc_to_depots.items():
                    mounted_for_dlc = [
                        {"depot_id": did, "manifest_gid": str(mounted_depots[did])}
                        for did in dep_ids
                        if did in mounted_depots and mounted_depots[did]
                    ]
                    if mounted_for_dlc:
                        installed_dlcs.add(dlc_id)
                        ver_str = self._version_from_manifests(mounted_for_dlc)
                        if ver_str and dlc_id not in installed_dlc_versions:
                            installed_dlc_versions[dlc_id] = ver_str
            owned_ids = self._known_owned_app_ids(offline=False)
            target_dlcs: list[str] = []
            seen_dlcs: set[str] = set()
            for raw_dlc in store_details.get("dlc") or []:
                did = str(raw_dlc)
                if did.isdigit() and did not in seen_dlcs and (did in owned_ids or did in installed_dlcs):
                    seen_dlcs.add(did)
                    target_dlcs.append(did)
            for did in sorted(installed_dlcs, key=lambda x: int(x)):
                if did not in seen_dlcs:
                    seen_dlcs.add(did)
                    target_dlcs.append(did)

            remote_by_id = {
                str(d["depot_id"]): d
                for d in remote_depots
                if isinstance(d, dict) and d.get("depot_id")
            }

            for dlc_id in target_dlcs:
                if dlc_id not in installed_dlcs:
                    dlc_updates_needed.append(dlc_id)
                    continue
                installed_dlc_ver = installed_dlc_versions.get(dlc_id)
                dep_ids = list(dlc_to_depots.get(dlc_id) or [])
                dlc_remote_depots: list[dict[str, Any]] = []
                for dep_id in dep_ids:
                    if dep_id in remote_by_id:
                        dlc_remote_depots.append(remote_by_id[dep_id])
                        continue
                    try:
                        single = self._fetch_depot_manifests(
                            resolved_id,
                            depot_id=dep_id,
                            platform=platform,
                            branch=branch,
                            branch_password=branch_password,
                            account_name=account_name,
                            offline=False,
                            allow_cache_fallback=False,
                            update_cache=False,
                        )
                        for dep in single.get("depots") or []:
                            if isinstance(dep, dict) and str(dep.get("depot_id") or "") == dep_id:
                                dlc_remote_depots.append(dep)
                    except CommandFailedError:
                        pass

                dlc_remote_ver = self._version_from_manifests(dlc_remote_depots)
                if dlc_remote_ver is None:
                    if dep_ids:
                        if not check_only:
                            dlc_updates_needed.append(dlc_id)
                        elif installed_dlc_ver is not None:
                            raise CommandFailedError(
                                f"Could not obtain remote manifest ID for Steam DLC AppID {dlc_id} "
                                f"depots ({', '.join(dep_ids)}) under base AppID {resolved_id}."
                            )
                elif dlc_remote_ver != installed_dlc_ver:
                    dlc_updates_needed.append(dlc_id)

        if remote_version is None:
            if check_only:
                raise CommandFailedError(
                    f"Could not obtain remote manifest ID for Steam AppID {resolved_id} "
                    f"(branch {branch!r}) from DepotDownloader -manifest-only."
                )
        else:
            base_update_available = remote_version != old_version
            update_available = base_update_available or bool(dlc_updates_needed)
            if check_only:
                if base_update_available and dlc_updates_needed:
                    msg = (
                        f"Update available ({old_version or 'unknown'} -> {remote_version}) "
                        f"plus DLC updates: {', '.join(dlc_updates_needed)}"
                    )
                elif base_update_available:
                    msg = f"Update available ({old_version or 'unknown'} -> {remote_version})"
                elif dlc_updates_needed:
                    msg = f"DLC update/install available: {', '.join(dlc_updates_needed)}"
                else:
                    msg = "Already up to date"
                return UpdateResult(
                    store=self.name,
                    app_id=resolved_id,
                    updated=False,
                    update_available=update_available,
                    old_version=old_version,
                    new_version=remote_version,
                    install_path=str(install_path),
                    platform=platform,
                    message=msg,
                )

            if not update_available and old_version is not None:
                return UpdateResult(
                    store=self.name,
                    app_id=resolved_id,
                    updated=False,
                    update_available=False,
                    old_version=old_version,
                    new_version=remote_version,
                    install_path=str(install_path),
                    platform=platform,
                    message="Already up to date",
                )

        dl_res = self.download(
            DownloadOptions(
                app_id=resolved_id,
                dest=install_path,
                platform=platform,
                with_dlcs=with_dlcs,
                skip_dlcs=not with_dlcs,
                branch=branch,
                branch_password=branch_password,
            )
        )
        return UpdateResult(
            store=self.name,
            app_id=resolved_id,
            updated=True,
            update_available=True,
            old_version=old_version,
            new_version=dl_res.version or remote_version,
            install_path=dl_res.install_path,
            platform=dl_res.platform,
            updated_dlcs=list(dl_res.downloaded_dlcs),
            message=f"Updated {resolved_id} on branch '{branch}' via DepotDownloader",
        )

    def uninstall(
        self,
        app_id: str,
        *,
        keep_files: bool = False,
        skip_uninstaller: bool = False,
    ) -> UninstallResult:
        del skip_uninstaller
        resolved_id = self._resolve_app_id(app_id, offline=True, strict=True, require_exact=True)
        installs = self._load_installs()
        entry = installs.get(resolved_id)
        if entry is None:
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not managed by `port-acquire`."
            )

        install_path_str = str(entry.get("install_path") or "")
        install_path = Path(install_path_str).resolve() if install_path_str else None

        if not keep_files and install_path is not None:
            home_resolved = Path.home().resolve()
            if (
                install_path == Path("/")
                or install_path == home_resolved
                or len(install_path.parts) <= 2
            ):
                raise AcquireError(
                    f"Refusing to delete unsafe path during uninstall: {install_path}",
                    code="unsafe_uninstall_path",
                )
            if install_path.exists():
                shutil.rmtree(install_path)

        installs.pop(resolved_id, None)
        self._save_installs(installs)

        return UninstallResult(
            store=self.name,
            app_id=resolved_id,
            uninstalled=True,
            kept_files=keep_files,
            install_path=install_path_str or None,
        )

    def launch_args(
        self,
        app_id: str,
        *,
        offline: bool = True,
        install_dir: Path | None = None,
    ) -> LaunchPlan:
        resolved_id = self._resolve_app_id(app_id, offline=True)
        entry = self._get_installed_entry(resolved_id)

        resolved_dir: Path | None = None
        if install_dir is not None:
            resolved_dir = Path(install_dir)
        elif entry and entry.get("install_path"):
            resolved_dir = Path(str(entry["install_path"]))

        if resolved_dir is None:
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not installed; cannot generate launch plan."
            )

        platform = str((entry or {}).get("platform") or "windows")
        expected_branch = str((entry or {}).get("branch") or "").strip()
        manifest_data = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            offline=True,
        )
        manifest_branch = str(manifest_data.get("branch") or "").strip()
        manifest_paths: list[str] = []
        if not expected_branch or not manifest_branch or manifest_branch == expected_branch:
            for d in manifest_data.get("depots") or []:
                if isinstance(d, dict):
                    for f in d.get("files") or []:
                        if isinstance(f, dict) and f.get("path"):
                            manifest_paths.append(str(f["path"]))

        exe = self._pick_executable_from_paths(manifest_paths)
        if exe and resolved_dir.is_dir() and not (resolved_dir / Path(exe)).is_file():
            existing_manifest_paths = [
                p for p in manifest_paths if (resolved_dir / Path(p.replace("\\", "/"))).is_file()
            ]
            exe = self._pick_executable_from_paths(existing_manifest_paths)
        if not exe and resolved_dir.is_dir():
            disk_exes = [
                p.relative_to(resolved_dir).as_posix()
                for p in sorted(resolved_dir.rglob("*.exe"))
                if not p.is_symlink()
            ]
            exe = self._pick_executable_from_paths(disk_exes)

        raw_args_str = str((entry or {}).get("launch_args") or "")
        raw_args = _split_windows_args(raw_args_str) if raw_args_str else []
        safe_args, args_redacted = redact_launch_args(raw_args)
        safe_env, env_redacted = redact_env({"SteamAppId": resolved_id, "SteamGameId": resolved_id})
        title = str(
            (entry or {}).get("title")
            or (entry or {}).get("name")
            or f"Steam App {resolved_id}"
        )
        return LaunchPlan(
            store=self.name,
            app_id=resolved_id,
            title=title,
            install_dir=str(resolved_dir),
            executable=exe or "",
            working_dir=Path(exe).parent.as_posix() if exe else ".",
            args=safe_args,
            env=safe_env,
            offline=offline,
            redacted_auth_args=bool(args_redacted or env_redacted),
        )


BACKEND_CLASS = SteamBackend
