"""Steam store backend for `tools/port-acquire` using Valve's `steamcmd`.

Tool selection (macOS Apple Silicon / `arm64`):
- **Primary engine: Valve's `steamcmd`** (`+@sSteamCmdForcePlatformType windows`),
  because it is Valve's official CLI and covers the complete acquisition lifecycle:
  interactive login + Steam Guard token caching, owned license enumeration
  (`+licenses_print`), full depot/branch/DLC/redist metadata (`+app_info_print`),
  Windows depot downloads with branch/beta switching (`+app_update -beta`), file
  checksum verification (`validate`), and targeted DLC / `_CommonRedist` (`228980`)
  depot fetches (`+download_depot`).
- **macOS `arm64` caveat & optional fallback:** Valve ships macOS `steamcmd` as an
  Intel `x86_64` binary, so it executes via Rosetta 2 on Apple Silicon. When
  `steamcmd` or Rosetta 2 is unavailable, download/verify/update operations can fall
  back to native Apple Silicon `arm64` `DepotDownloader` (`PORT_DEPOT_DOWNLOADER` or
  `~/.local/bin/DepotDownloader`, installed by `cargo opsdb -- steam-setup`), while
  noting that `DepotDownloader` cannot run `licenses_print` or `app_info_print`.

Hard rules:
- Zero DRM, anti-cheat, or ownership bypasses of any kind. `info` reports
  Steamworks/DRM/anti-cheat signals read-only.
- Only download titles owned by the logged-in account through Valve's official tools.
- Never store, return, or log passwords, Steam Guard codes, refresh tokens, or API
  keys. Credentials stay in `steamcmd`'s own cache or the macOS Keychain.
"""

from __future__ import annotations

from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform as py_platform
import re
import shutil
import subprocess
import sys
from typing import Any, Callable
import urllib.error
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
    UnsupportedOperationError,
    UpdateResult,
    VerifyResult,
    fuzzy_search_owned,
    normalize_title,
    scan_dir_redistributables,
    score_title_match,
)
from ..manifest import redact_env, redact_launch_args
from ._steam_vdf import (
    COMMON_REDIST_APP_ID,
    COMMON_REDIST_DEPOTS,
    extract_appinfo_vdf,
    extract_proton_redist_info,
    has_cached_connect_token,
    parse_appmanifest_acf,
    parse_licenses_print,
    parse_loginusers_vdf,
    vdf_get,
)

KEYCHAIN_WEB_API_SERVICE = "port-acquire-steam-web-api-key"

_ANTICHEAT_PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    ("Easy Anti-Cheat", re.compile(r"(easyanticheat|eac_launcher|start_protected_game)", re.I)),
    ("BattlEye", re.compile(r"(battleye|beservice|beclient)", re.I)),
    ("Valve Anti-Cheat (VAC)", re.compile(r"(\bvac\b|vacmacmodulecache)", re.I)),
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
    ("Battlenet / Activision", re.compile(r"battle\.net", re.I), "Battle.net"),
    ("SteamStub / CEG", re.compile(r"(steamstub|steam\s+ceg|\b\.bind\b)", re.I), None),
]

_SKIP_EXE_PATTERNS = re.compile(
    r"(unins|vcredist|vc_redist|dxsetup|dxwebsetup|dotnet|ndp4|oalinst|physx|crashreport|ueprereq|redist)",
    re.I,
)

_LOGIN_FAIL_PATTERNS = re.compile(
    r"(FAILED|Invalid\s+Password|Rate\s+Limit|Steam\s+Guard|Two-factor|Cached\s+credentials|"
    r"password:|Logon\s+Failure|Access\s+Denied|No\s+connection)",
    re.I,
)


def _split_windows_args(raw: str) -> list[str]:
    """Parse a Windows-style command-line argument string preserving quoted values.

    Handles double-quoted substrings (e.g. `-config "My Settings.ini"` ->
    `['-config', 'My Settings.ini']`) and escaped quotes (`\\"`), while keeping
    Windows path backslashes (e.g. `C:\\Games\\Foo`) intact.
    """
    args: list[str] = []
    cur: list[str] = []
    in_quotes = False
    has_token = False
    i = 0
    n = len(raw)
    while i < n:
        ch = raw[i]
        if ch == "\\":
            # Count consecutive backslashes before a quote (Windows CommandLineToArgvW rule)
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
    for idx, tok in enumerate(cmd):
        if redact_next:
            out.append("<redacted>")
            redact_next = False
            continue
        low = tok.lower()
        if low in ("-betapassword", "-branchpassword", "-password", "--password"):
            out.append(tok)
            redact_next = True
            continue
        # Handle `+login <user> <password>` (we never pass passwords, but guard defensively)
        if idx >= 2 and cmd[idx - 2].lower() in ("+login", "login") and not tok.startswith("+"):
            out.append("<redacted>")
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
    """Pluggable Steam backend backed primarily by Valve's `steamcmd`."""

    name = "steam"
    display_name = "Steam (steamcmd)"

    def __init__(
        self,
        *,
        steamcmd_bin: str | Path | None = None,
        depot_downloader_bin: str | Path | None = None,
        steam_home: str | Path | None = None,
        state_dir: str | Path | None = None,
        http_get_json: Callable[[str], Any] | None = None,
        allow_network: bool = True,
        rosetta_check: Callable[[], bool] | None = None,
    ) -> None:
        self._explicit_steamcmd = Path(steamcmd_bin) if steamcmd_bin else None
        self._explicit_dd = Path(depot_downloader_bin) if depot_downloader_bin else None
        self._explicit_steam_home = Path(steam_home) if steam_home else None
        self._explicit_state_dir = Path(state_dir) if state_dir else None
        self._http_get_json = http_get_json or _default_http_get_json
        self._allow_network = allow_network
        self._rosetta_check = rosetta_check

    # ------------------------------------------------------------------
    # Paths & Tool Discovery
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

    def _appinfo_cache_file(self, app_id: str) -> Path:
        return self.state_dir() / "appinfo" / f"{app_id}.json"

    def find_steamcmd(self) -> Path | None:
        if self._explicit_steamcmd is not None:
            return self._explicit_steamcmd if self._explicit_steamcmd.is_file() else None
        env_bin = os.environ.get("PORT_STEAMCMD", "").strip()
        if env_bin:
            p = Path(env_bin).expanduser()
            return p if p.is_file() else None
        for name in ("steamcmd", "steamcmd.sh"):
            found = shutil.which(name)
            if found:
                return Path(found)
        candidates = [
            Path("/opt/homebrew/bin/steamcmd"),
            Path("/usr/local/bin/steamcmd"),
            Path.home() / "Steam" / "steamcmd.sh",
            Path.home() / ".local" / "opt" / "steamcmd" / "steamcmd.sh",
            Path.home() / "Library" / "Application Support" / "SteamCMD" / "steamcmd.sh",
        ]
        for cand in candidates:
            if cand.is_file():
                return cand
        return None

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
        ]
        for cand in candidates:
            if cand.is_file():
                return cand
        for name in ("DepotDownloader", "depotdownloader"):
            found = shutil.which(name)
            if found:
                return Path(found)
        return None

    def is_rosetta_available(self) -> bool:
        if self._rosetta_check is not None:
            return self._rosetta_check()
        if sys.platform != "darwin" or py_platform.machine().lower() not in ("arm64", "aarch64"):
            return True
        try:
            proc = subprocess.run(
                ["/usr/bin/arch", "-x86_64", "/usr/bin/true"],
                capture_output=True,
                timeout=5,
                check=False,
            )
            return proc.returncode == 0
        except OSError:
            return False

    def _require_steamcmd(self) -> Path:
        scmd = self.find_steamcmd()
        if scmd is None:
            raise BackendUnavailableError(
                "Valve `steamcmd` not found. Install it with `brew install --cask steamcmd` "
                "(requires Rosetta 2 on Apple Silicon: `softwareupdate --install-rosetta`), "
                "or point `PORT_STEAMCMD` to your `steamcmd` binary."
            )
        return scmd

    def _backend_version_label(self) -> str | None:
        scmd = self.find_steamcmd()
        if scmd is not None:
            if sys.platform == "darwin" and py_platform.machine().lower() in ("arm64", "aarch64"):
                rosetta = "Rosetta 2 OK" if self.is_rosetta_available() else "Rosetta 2 missing"
                return f"steamcmd ({scmd.name}, x86_64 via {rosetta})"
            return f"steamcmd ({scmd.name})"
        dd = self.find_depot_downloader()
        if dd is not None:
            return f"DepotDownloader ({dd.name}, fallback)"
        return None

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
        sa_dir = self.steam_home() / "steamapps"
        if not sa_dir.is_dir():
            return out
        for path in sorted(sa_dir.glob("appmanifest_*.acf")):
            parsed = parse_appmanifest_acf(path)
            if parsed and parsed.get("app_id"):
                installdir = parsed.get("installdir", "")
                if installdir:
                    parsed["install_path"] = str(sa_dir / "common" / installdir)
                out[parsed["app_id"]] = parsed
        return out

    def _get_installed_entry(self, app_id: str) -> dict[str, Any] | None:
        installs = self._load_installs()
        if str(app_id) in installs:
            return installs[str(app_id)]
        acf_map = self._scan_steamapps_manifests()
        return acf_map.get(str(app_id))

    # ------------------------------------------------------------------
    # Subprocess Execution
    # ------------------------------------------------------------------

    def _run_steamcmd(
        self,
        args: list[str],
        *,
        interactive: bool = False,
        timeout: float | None = 300.0,
        check: bool = True,
    ) -> subprocess.CompletedProcess[str]:
        scmd = self._require_steamcmd()
        cmd = [str(scmd), *args]
        env = os.environ.copy()
        if self._explicit_steam_home is not None:
            env["PORT_STEAM_HOME"] = str(self._explicit_steam_home)
        if interactive:
            try:
                proc = subprocess.run(cmd, env=env, check=False)
            except OSError as exc:
                raise BackendUnavailableError(f"failed to execute steamcmd: {exc}") from exc
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
                    "steamcmd timed out (it may be waiting for interactive login or Steam Guard; "
                    "run `tools/port-acquire --store steam login --username <account>` first)",
                    details={"command": _redact_cmd(cmd)},
                ) from exc
            except OSError as exc:
                raise BackendUnavailableError(f"failed to execute steamcmd: {exc}") from exc

        if check and completed.returncode != 0:
            combined = (completed.stdout + "\n" + completed.stderr).strip()
            tail = "\n".join(combined.splitlines()[-12:])
            raise CommandFailedError(
                f"steamcmd exited with status {completed.returncode}: {tail or 'no output'}",
                details={"returncode": completed.returncode, "command": _redact_cmd(cmd)},
            )
        return completed

    def _run_depot_downloader(
        self,
        args: list[str],
        *,
        timeout: float | None = 1800.0,
    ) -> subprocess.CompletedProcess[str]:
        dd = self.find_depot_downloader()
        if dd is None:
            raise BackendUnavailableError(
                "Neither `steamcmd` nor `DepotDownloader` is installed. "
                "Install `steamcmd` (`brew install --cask steamcmd`) or run `cargo opsdb -- steam-setup`."
            )
        cmd = [str(dd), *args]
        try:
            completed = subprocess.run(
                cmd,
                stdin=subprocess.DEVNULL,
                capture_output=True,
                text=True,
                timeout=timeout,
                check=False,
            )
        except OSError as exc:
            raise BackendUnavailableError(f"failed to execute DepotDownloader: {exc}") from exc
        if completed.returncode != 0:
            combined = (completed.stdout + "\n" + completed.stderr).strip()
            tail = "\n".join(combined.splitlines()[-12:])
            raise CommandFailedError(
                f"DepotDownloader exited with status {completed.returncode}: {tail or 'no output'}",
                details={"returncode": completed.returncode, "command": _redact_cmd(cmd)},
            )
        return completed

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
        del no_webview, auth_code  # SteamCMD handles Steam Guard directly in the terminal
        if logout:
            self._clear_session()
            return AuthStatus(
                store=self.name,
                logged_in=False,
                display_name=None,
                backend_available=self.find_steamcmd() is not None or self.find_depot_downloader() is not None,
                backend_version=self._backend_version_label(),
            )

        account_name, steam_id64 = self._resolve_account_info(
            username,
            ignore_logged_out=import_existing,
        )

        if import_existing:
            if not account_name:
                raise AuthRequiredError(
                    "No existing Steam login found in `loginusers.vdf`. Run interactive login: "
                    "`tools/port-acquire --store steam login --username <your_steam_account>`"
                )
            self._save_session(account_name, steam_id64)
            return self.status(offline=self.find_steamcmd() is None)

        if not account_name:
            if sys.stdin.isatty():
                entered = input("Steam account username (not password): ").strip()
                if entered:
                    account_name, steam_id64 = self._resolve_account_info(entered)
            if not account_name:
                raise AuthRequiredError(
                    "Steam username required for interactive login. Run:\n"
                    "  tools/port-acquire --store steam login --username <your_steam_account>"
                )

        # Interactive login attaches directly to the user's TTY so Valve's `steamcmd`
        # prompts for password and Steam Guard directly (credentials never touch our code).
        scmd = self._require_steamcmd()
        if sys.platform == "darwin" and py_platform.machine().lower() in ("arm64", "aarch64"):
            if not self.is_rosetta_available():
                raise BackendUnavailableError(
                    "Rosetta 2 is required to run Valve's x86_64 `steamcmd` on Apple Silicon. "
                    "Install it with `softwareupdate --install-rosetta --agree-to-license`."
                )

        proc = self._run_steamcmd(
            ["+login", account_name, "+quit"],
            interactive=True,
            check=False,
        )
        if proc.returncode != 0:
            raise AuthRequiredError(
                f"SteamCMD login did not complete cleanly (exit {proc.returncode}). "
                f"Re-run interactively:\n"
                f"  tools/port-acquire --store steam login --username {account_name}"
            )

        # Refresh SteamID64 from loginusers.vdf if SteamCMD updated it.
        _, refreshed_id64 = self._resolve_account_info(account_name)
        self._save_session(account_name, refreshed_id64 or steam_id64)

        return AuthStatus(
            store=self.name,
            logged_in=True,
            display_name=account_name,
            backend_available=scmd.is_file(),
            backend_version=self._backend_version_label(),
        )

    def status(self, *, offline: bool = False) -> AuthStatus:
        scmd = self.find_steamcmd()
        dd = self.find_depot_downloader()
        available = scmd is not None or dd is not None
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

        if offline or not self._use_steamcmd_for_download():
            has_token = has_cached_connect_token(
                self.steam_home() / "config" / "config.vdf",
                account_name=account_name,
            )
            sess_present = bool(self._load_session().get("account_name"))
            loginusers = parse_loginusers_vdf(self.steam_home() / "config" / "loginusers.vdf")
            remembered = any(
                u["account_name"].lower() == account_name.lower()
                and (u["remember_password"] or u["allow_auto_login"])
                for u in loginusers
            )
            logged_in = bool(has_token or sess_present or remembered)
            return AuthStatus(
                store=self.name,
                logged_in=logged_in,
                display_name=account_name if logged_in else None,
                backend_available=available,
                backend_version=version,
            )

        # Non-interactive cached-token probe via `steamcmd +login <user> +quit`.
        proc = self._run_steamcmd(
            ["+login", account_name, "+quit"],
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
            # Never expose `url` (contains api_key) in errors; fall back cleanly to steamcmd.
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

    def list_owned(self, *, platform: str = "windows", refresh: bool = False) -> list[OwnedGame]:
        del refresh
        account_name, steam_id64 = self._require_logged_in_account()
        installs = self._load_installs()
        acf_map = self._scan_steamapps_manifests()

        games_by_id: dict[str, OwnedGame] = {}

        # 1. Optional Steam Web API (fast, includes human titles)
        api_key = self._get_steam_web_api_key()
        if api_key and steam_id64:
            web_games = self._list_owned_via_web_api(api_key, steam_id64)
            if web_games is not None:
                for g in web_games:
                    games_by_id[g.app_id] = g

        # 2. Primary zero-API-key source: `steamcmd +login <user> +licenses_print +quit`
        if not games_by_id:
            proc = self._run_steamcmd(
                ["+login", account_name, "+licenses_print", "+quit"],
                interactive=False,
                timeout=120.0,
                check=False,
            )
            combined = proc.stdout + "\n" + proc.stderr
            if proc.returncode != 0 or _LOGIN_FAIL_PATTERNS.search(combined):
                raise AuthRequiredError(
                    f"SteamCMD could not list licenses for '{account_name}' with cached login. "
                    f"Re-authenticate interactively:\n"
                    f"  tools/port-acquire --store steam login --username {account_name}"
                )
            licenses = parse_licenses_print(combined)
            for lic in licenses:
                if not lic.get("active", True):
                    continue
                for aid in lic.get("app_ids", []):
                    if aid == COMMON_REDIST_APP_ID:
                        continue
                    if aid not in games_by_id:
                        cached_vdf = _read_json(self._appinfo_cache_file(aid), {})
                        cached_title = str(vdf_get(cached_vdf, "common", "name", default="") or "")
                        inst_title = (
                            str(installs.get(aid, {}).get("title") or "")
                            or str(acf_map.get(aid, {}).get("name") or "")
                        )
                        title = cached_title or inst_title or f"Steam App {aid}"
                        games_by_id[aid] = OwnedGame(
                            store=self.name,
                            app_id=aid,
                            title=title,
                            platforms=["windows"],
                        )

        # Merge local installed state
        for aid, inst in installs.items():
            if aid in games_by_id:
                games_by_id[aid].installed = True
                games_by_id[aid].install_path = inst.get("install_path")
                games_by_id[aid].version = inst.get("build_id")
                if inst.get("title") and games_by_id[aid].title.startswith("Steam App "):
                    games_by_id[aid].title = str(inst["title"])
        for aid, acf in acf_map.items():
            if aid in games_by_id:
                games_by_id[aid].installed = True
                games_by_id[aid].install_path = acf.get("install_path")
                games_by_id[aid].version = acf.get("build_id")
                if acf.get("name") and games_by_id[aid].title.startswith("Steam App "):
                    games_by_id[aid].title = str(acf["name"])

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

        # If any owned apps came from `licenses_print` without human titles, enrich
        # titles of matching owned apps via Steam's public Store Search API while
        # strictly restricting returned results to apps owned by the logged-in account.
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

    def _fetch_store_appdetails(self, app_id: str) -> dict[str, Any]:
        if not self._allow_network:
            return {}
        url = f"https://store.steampowered.com/api/appdetails?appids={urllib.parse.quote(app_id)}&l=english"
        try:
            payload = self._http_get_json(url)
        except Exception:
            return {}
        entry = payload.get(str(app_id)) if isinstance(payload, dict) else None
        if isinstance(entry, dict) and entry.get("success") and isinstance(entry.get("data"), dict):
            return entry["data"]
        return {}

    def _fetch_appinfo_vdf(
        self,
        app_id: str,
        *,
        platform: str = "windows",
        offline: bool = False,
        allow_cache_fallback: bool = True,
    ) -> dict[str, Any]:
        cache_path = self._appinfo_cache_file(app_id)
        if offline:
            cached = _read_json(cache_path, {})
            return cached if isinstance(cached, dict) else {}

        account_name, _ = self._resolve_account_info()
        login_user = account_name or "anonymous"
        proc = self._run_steamcmd(
            [
                "+@sSteamCmdForcePlatformType",
                platform,
                "+login",
                login_user,
                "+app_info_update",
                "1",
                "+app_info_request",
                app_id,
                "+app_info_print",
                app_id,
                "+quit",
            ],
            interactive=False,
            timeout=120.0,
            check=False,
        )
        vdf_data, _ = extract_appinfo_vdf(proc.stdout + "\n" + proc.stderr, app_id)
        if not vdf_data and proc.returncode != 0:
            if allow_cache_fallback:
                cached = _read_json(cache_path, {})
                if isinstance(cached, dict) and cached:
                    return cached
            raise CommandFailedError(
                f"steamcmd failed to query app_info for AppID {app_id} (exit {proc.returncode})"
            )
        if vdf_data:
            _atomic_write_json(cache_path, vdf_data)
        return vdf_data

    def _detect_signals(
        self,
        appinfo: dict[str, Any],
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

        # 1. Inspect AppInfo VDF
        ufs = vdf_get(appinfo, "ufs")
        if isinstance(ufs, dict) and ufs:
            steamworks_signals.append("appinfo:ufs_steam_cloud")
        if vdf_get(appinfo, "config", "steamcontrollertemplateindex") is not None:
            steamworks_signals.append("appinfo:steam_input")
        depots_node = vdf_get(appinfo, "depots")
        if isinstance(depots_node, dict):
            for did, dval in depots_node.items():
                if not did.isdigit() or not isinstance(dval, dict):
                    continue
                if str(vdf_get(dval, "depotfromapp", default="")) == COMMON_REDIST_APP_ID:
                    if "appinfo:common_redist_228980" not in steamworks_signals:
                        steamworks_signals.append("appinfo:common_redist_228980")

        ac_url = str(vdf_get(appinfo, "extended", "anti_cheat_support_url", default="") or "").strip()
        if ac_url:
            anticheat_signals.append(f"appinfo:anti_cheat_support_url={ac_url}")

        if vdf_get(appinfo, "common", "vacmacmodulecache") is not None:
            if "Valve Anti-Cheat (VAC)" not in anticheat_systems:
                anticheat_systems.append("Valve Anti-Cheat (VAC)")
            anticheat_signals.append("appinfo:vacmacmodulecache")

        if str(vdf_get(appinfo, "extended", "thirdpartycdkey", default="0")) == "1":
            drm_signals.append("appinfo:thirdpartycdkey")

        # Scan serialized launch/common/extended text for known anti-cheat/DRM strings
        metadata_blob = json.dumps(
            {
                "common": vdf_get(appinfo, "common", default={}),
                "extended": vdf_get(appinfo, "extended", default={}),
                "config": vdf_get(appinfo, "config", default={}),
            },
            default=str,
        )
        for sys_name, pattern in _ANTICHEAT_PATTERNS:
            if pattern.search(metadata_blob):
                if sys_name not in anticheat_systems:
                    anticheat_systems.append(sys_name)
                anticheat_signals.append(f"appinfo:{sys_name}")

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

    def _pick_launch_entry(
        self,
        appinfo: dict[str, Any],
        *,
        platform: str = "windows",
    ) -> tuple[str | None, list[str], str | None]:
        """Select the primary Windows launch executable, args, and workingdir from AppInfo."""
        launch_node = vdf_get(appinfo, "config", "launch")
        if not isinstance(launch_node, dict):
            return None, [], None

        candidates: list[tuple[int, str, list[str], str | None]] = []
        for key, entry in launch_node.items():
            if not isinstance(entry, dict):
                continue
            exe = str(vdf_get(entry, "executable", default="") or "").strip().replace("\\", "/")
            if not exe:
                continue
            cfg = vdf_get(entry, "config")
            oslist = str(vdf_get(cfg, "oslist", default="") or "").lower()
            osarch = str(vdf_get(cfg, "osarch", default="") or "").strip()
            launch_type = str(vdf_get(entry, "type", default="default") or "default").lower()
            if oslist and platform not in oslist:
                continue
            if launch_type not in ("default", "none", ""):
                priority = 2
            else:
                priority = 0
            if osarch == "64":
                priority -= 1
            raw_args = str(vdf_get(entry, "arguments", default="") or "").strip()
            args_list = _split_windows_args(raw_args) if raw_args else []
            workdir = str(vdf_get(entry, "workingdir", default="") or "").strip().replace("\\", "/") or None
            sort_idx = int(key) if str(key).isdigit() else 999
            candidates.append((priority * 1000 + sort_idx, exe, args_list, workdir))

        if not candidates:
            return None, [], None
        candidates.sort(key=lambda item: item[0])
        _, exe, args_list, workdir = candidates[0]
        return exe, args_list, workdir

    def info(self, app_id: str, *, platform: str = "windows", offline: bool = False) -> GameInfo:
        resolved_id = self._resolve_app_id(app_id, offline=offline)
        appinfo = self._fetch_appinfo_vdf(resolved_id, platform=platform, offline=offline)
        store_details = {} if offline else self._fetch_store_appdetails(resolved_id)

        installed_entry = self._get_installed_entry(resolved_id)
        install_path_str = (
            str(installed_entry.get("install_path"))
            if installed_entry and installed_entry.get("install_path")
            else None
        )
        install_dir = Path(install_path_str) if install_path_str else None
        is_installed = bool(install_dir and install_dir.is_dir())

        title = (
            str(vdf_get(appinfo, "common", "name", default="") or "")
            or str(store_details.get("name") or "")
            or str((installed_entry or {}).get("title") or (installed_entry or {}).get("name") or "")
            or f"Steam App {resolved_id}"
        )

        # Parse branches & depots
        depots_node = vdf_get(appinfo, "depots")
        depots: list[DepotItem] = []
        branches: list[dict[str, Any]] = []
        public_build_id: str | None = None
        base_download_bytes = 0
        base_disk_bytes = 0
        has_size_info = False
        dlc_map: dict[str, DlcItem] = {}

        if isinstance(depots_node, dict):
            branches_node = vdf_get(depots_node, "branches")
            if isinstance(branches_node, dict):
                for bname, bval in branches_node.items():
                    if not isinstance(bval, dict):
                        continue
                    bid = str(vdf_get(bval, "buildid", default="") or "") or None
                    if bname == "public" and bid:
                        public_build_id = bid
                    branches.append(
                        {
                            "name": str(bname),
                            "build_id": bid,
                            "time_updated": str(vdf_get(bval, "timeupdated", default="") or "") or None,
                            "pwd_required": str(vdf_get(bval, "pwdrequired", default="0")) == "1",
                            "description": str(vdf_get(bval, "description", default="") or ""),
                        }
                    )

            for key, dval in depots_node.items():
                if not str(key).isdigit() or not isinstance(dval, dict):
                    continue
                cfg = vdf_get(dval, "config")
                oslist_raw = str(vdf_get(cfg, "oslist", default="") or "")
                oslist = [s.strip().lower() for s in oslist_raw.split(",") if s.strip()]
                osarch = str(vdf_get(cfg, "osarch", default="") or "").strip() or None
                language = str(vdf_get(cfg, "language", default="") or "").strip() or None
                dlc_app_id = str(vdf_get(dval, "dlcappid", default="") or "").strip() or None
                depot_from_app = str(vdf_get(dval, "depotfromapp", default="") or "").strip() or None
                shared_install = str(vdf_get(dval, "sharedinstall", default="0")) == "1"
                is_common_redist = (
                    depot_from_app == COMMON_REDIST_APP_ID or str(key) in COMMON_REDIST_DEPOTS
                )

                pub_manifest = vdf_get(dval, "manifests", "public")
                manifest_gid: str | None = None
                size_bytes: int | None = None
                download_bytes: int | None = None
                if isinstance(pub_manifest, dict):
                    manifest_gid = str(vdf_get(pub_manifest, "gid", default="") or "") or None
                    sz_s = str(vdf_get(pub_manifest, "size", default="") or "").strip()
                    dl_s = str(vdf_get(pub_manifest, "download", default="") or "").strip()
                    size_bytes = int(sz_s) if sz_s.isdigit() else None
                    download_bytes = int(dl_s) if dl_s.isdigit() else None
                elif isinstance(pub_manifest, str) and pub_manifest.strip():
                    manifest_gid = pub_manifest.strip()

                depot_name = str(vdf_get(dval, "name", default="") or "")
                if not depot_name and is_common_redist:
                    depot_name = COMMON_REDIST_DEPOTS.get(str(key), {}).get(
                        "name", f"Steamworks Common Redistributable ({key})"
                    )

                depots.append(
                    DepotItem(
                        depot_id=str(key),
                        name=depot_name,
                        oslist=oslist,
                        osarch=osarch,
                        language=language,
                        manifest_gid=manifest_gid,
                        size_bytes=size_bytes,
                        download_bytes=download_bytes,
                        dlc_app_id=dlc_app_id,
                        shared_install=shared_install,
                        depot_from_app=depot_from_app,
                        is_common_redist=is_common_redist,
                    )
                )

                matches_platform = not oslist or platform in oslist
                if matches_platform and not is_common_redist:
                    if dlc_app_id:
                        existing_dlc = dlc_map.get(dlc_app_id)
                        if existing_dlc is None:
                            dlc_map[dlc_app_id] = DlcItem(
                                app_id=dlc_app_id,
                                title=depot_name or f"Steam DLC {dlc_app_id}",
                                owned=True,
                                installed=dlc_app_id in ((installed_entry or {}).get("dlcs") or []),
                                download_size_bytes=download_bytes,
                                disk_size_bytes=size_bytes,
                            )
                        else:
                            if download_bytes is not None:
                                existing_dlc.download_size_bytes = (
                                    existing_dlc.download_size_bytes or 0
                                ) + download_bytes
                            if size_bytes is not None:
                                existing_dlc.disk_size_bytes = (
                                    existing_dlc.disk_size_bytes or 0
                                ) + size_bytes
                    else:
                        if size_bytes is not None:
                            base_disk_bytes += size_bytes
                            has_size_info = True
                        if download_bytes is not None:
                            base_download_bytes += download_bytes
                            has_size_info = True

        # Add any DLCs listed in `extended.listofdlc`
        listofdlc = str(vdf_get(appinfo, "extended", "listofdlc", default="") or "")
        for dlc_tok in re.findall(r"\d+", listofdlc):
            if dlc_tok not in dlc_map:
                dlc_map[dlc_tok] = DlcItem(
                    app_id=dlc_tok,
                    title=f"Steam DLC {dlc_tok}",
                    owned=True,
                    installed=dlc_tok in ((installed_entry or {}).get("dlcs") or []),
                )

        # Redistributables + Proton-adjacent info
        proton_info = extract_proton_redist_info(appinfo, install_dir if is_installed else None)
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
            appinfo,
            store_details,
            install_dir if is_installed else None,
        )
        exe, _, _ = self._pick_launch_entry(appinfo, platform=platform)
        version = public_build_id or (installed_entry or {}).get("build_id")

        return GameInfo(
            store=self.name,
            app_id=resolved_id,
            title=title,
            platform=platform,
            version=str(version) if version else None,
            download_size_bytes=base_download_bytes if has_size_info else None,
            disk_size_bytes=(
                base_disk_bytes
                if has_size_info
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
            dlcs=sorted(dlc_map.values(), key=lambda d: int(d.app_id) if d.app_id.isdigit() else 0),
            redistributables=redists,
            proton_redist_info=proton_info,
            manifest_files_checked=bool(appinfo),
        )

    def _use_steamcmd_for_download(self) -> bool:
        scmd = self.find_steamcmd()
        if scmd is None:
            return False
        if sys.platform == "darwin" and py_platform.machine().lower() in ("arm64", "aarch64"):
            return self.is_rosetta_available()
        return True

    def download(self, options: DownloadOptions) -> DownloadResult:
        account_name, _ = self._require_logged_in_account()
        use_steamcmd = self._use_steamcmd_for_download()
        resolved_id = self._resolve_app_id(options.app_id, offline=not use_steamcmd)
        dest = options.dest.expanduser().resolve()
        dest.mkdir(parents=True, exist_ok=True)
        branch = (options.branch or "public").strip()

        # Fetch metadata before download (best-effort) when steamcmd is runnable.
        info: GameInfo | None = None
        if use_steamcmd:
            try:
                info = self.info(resolved_id, platform=options.platform, offline=False)
            except AcquireError:
                info = None

        downloaded_depots: list[str] = []
        downloaded_dlcs: list[str] = []
        downloaded_redists: list[str] = []

        if use_steamcmd:
            args: list[str] = [
                "+@sSteamCmdForcePlatformType",
                options.platform,
                "+force_install_dir",
                str(dest),
                "+login",
                account_name,
                "+app_update",
                resolved_id,
            ]
            if branch and branch != "public":
                args.extend(["-beta", branch])
                if options.branch_password:
                    args.extend(["-betapassword", options.branch_password])
            if options.validate:
                args.append("validate")
            args.extend(options.extra_args)
            args.append("+quit")

            proc = self._run_steamcmd(args, interactive=False, timeout=None, check=False)
            combined = proc.stdout + "\n" + proc.stderr
            if proc.returncode != 0 or _LOGIN_FAIL_PATTERNS.search(combined):
                if _LOGIN_FAIL_PATTERNS.search(combined):
                    raise AuthRequiredError(
                        f"SteamCMD authentication failed for '{account_name}'. "
                        f"Run interactively:\n"
                        f"  tools/port-acquire --store steam login --username {account_name}"
                    )
                raise CommandFailedError(
                    f"steamcmd app_update failed for AppID {resolved_id} (exit {proc.returncode})",
                    details={"returncode": proc.returncode, "command": _redact_cmd(args)},
                )

            if info is not None:
                for dep in info.depots:
                    matches_os = not dep.oslist or options.platform in dep.oslist
                    if matches_os and not dep.is_common_redist and not dep.dlc_app_id:
                        downloaded_depots.append(dep.depot_id)

            # Explicit DLC depot download when requested
            if info is not None and (options.with_dlcs or options.dlcs):
                wanted_dlcs = set(options.dlcs) if options.dlcs else {d.app_id for d in info.dlcs}
                for dep in info.depots:
                    if dep.dlc_app_id and dep.dlc_app_id in wanted_dlcs:
                        if not dep.oslist or options.platform in dep.oslist:
                            dlc_args = [
                                "+@sSteamCmdForcePlatformType",
                                options.platform,
                                "+login",
                                account_name,
                                "+download_depot",
                                resolved_id,
                                dep.depot_id,
                                "+quit",
                            ]
                            self._run_steamcmd(dlc_args, interactive=False, timeout=None, check=True)
                            downloaded_depots.append(dep.depot_id)
                            if dep.dlc_app_id not in downloaded_dlcs:
                                downloaded_dlcs.append(dep.dlc_app_id)

            # Steamworks Common Redistributables (`228980`) depot fetch when requested
            if info is not None and options.with_redists:
                for dep in info.depots:
                    if dep.is_common_redist:
                        redist_args = [
                            "+@sSteamCmdForcePlatformType",
                            options.platform,
                            "+login",
                            account_name,
                            "+download_depot",
                            COMMON_REDIST_APP_ID,
                            dep.depot_id,
                            "+quit",
                        ]
                        self._run_steamcmd(redist_args, interactive=False, timeout=None, check=True)
                        downloaded_redists.append(dep.depot_id)
                        # Stage downloaded depot directory if SteamCMD placed it under steamapps/content
                        staged_src = (
                            self.steam_home()
                            / "steamapps"
                            / "content"
                            / f"app_{COMMON_REDIST_APP_ID}"
                            / f"depot_{dep.depot_id}"
                        )
                        if staged_src.is_dir():
                            redist_dst = dest / "_CommonRedist" / dep.depot_id
                            shutil.copytree(staged_src, redist_dst, dirs_exist_ok=True)
        else:
            # Native arm64 DepotDownloader fallback when steamcmd/Rosetta is unavailable
            dd_args = [
                "-app",
                resolved_id,
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
            if options.branch_password:
                dd_args.extend(["-branchpassword", options.branch_password])
            if options.validate:
                dd_args.append("-validate")
            if options.max_workers:
                dd_args.extend(["-max-downloads", str(options.max_workers)])
            self._run_depot_downloader(dd_args)

        # Ensure standard `steam_appid.txt` exists for downstream packaging/runtime
        appid_txt = dest / "steam_appid.txt"
        if not appid_txt.exists():
            appid_txt.write_text(f"{resolved_id}\n", encoding="utf-8")

        # Determine build_id from downloaded appmanifest or info
        build_id = info.version if info else None
        local_acf = parse_appmanifest_acf(dest / "steamapps" / f"appmanifest_{resolved_id}.acf")
        if local_acf and local_acf.get("build_id"):
            build_id = str(local_acf["build_id"])

        title = info.title if info else (local_acf or {}).get("name") or f"Steam App {resolved_id}"

        if not options.no_register:
            installs = self._load_installs()
            installs[resolved_id] = {
                "app_id": resolved_id,
                "title": title,
                "install_path": str(dest),
                "platform": options.platform,
                "branch": branch,
                "build_id": build_id,
                "depots": downloaded_depots,
                "dlcs": downloaded_dlcs,
                "redists": downloaded_redists,
                "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
            self._save_installs(installs)

        return DownloadResult(
            store=self.name,
            app_id=resolved_id,
            title=title,
            platform=options.platform,
            install_path=str(dest),
            version=build_id,
            branch=branch,
            downloaded_dlcs=downloaded_dlcs,
            downloaded_depots=downloaded_depots,
            downloaded_redists=downloaded_redists,
        )

    def verify(self, app_id: str, *, repair: bool = False) -> VerifyResult:
        account_name, _ = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(app_id, offline=True, strict=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not registered as installed. "
                "Download it first with `tools/port-acquire --store steam download <appid>`."
            )
        install_path = Path(str(entry["install_path"]))
        if not install_path.is_dir():
            return VerifyResult(
                store=self.name,
                app_id=resolved_id,
                ok=False,
                repaired=False,
                missing_files=[str(install_path)],
                message=f"Install directory missing: {install_path}",
            )

        platform = str(entry.get("platform") or "windows")
        branch = str(entry.get("branch") or "public")

        if self._use_steamcmd_for_download():
            args = [
                "+@sSteamCmdForcePlatformType",
                platform,
                "+force_install_dir",
                str(install_path),
                "+login",
                account_name,
                "+app_update",
                resolved_id,
            ]
            if branch and branch != "public":
                args.extend(["-beta", branch])
            args.extend(["validate", "+quit"])
            proc = self._run_steamcmd(args, interactive=False, timeout=None, check=False)
            combined = proc.stdout + "\n" + proc.stderr
            ok = proc.returncode == 0 and "error!" not in combined.lower()
            repaired_flag = bool(
                repair and ok and ("downloading" in combined.lower() or "repaired" in combined.lower())
            )
            msg_lines = [ln.strip() for ln in combined.splitlines() if ln.strip()]
            return VerifyResult(
                store=self.name,
                app_id=resolved_id,
                ok=ok,
                repaired=repaired_flag,
                message=msg_lines[-1] if msg_lines else ("Verified OK" if ok else "Validation failed"),
            )

        # DepotDownloader fallback
        dd_args = [
            "-app",
            resolved_id,
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
        self._run_depot_downloader(dd_args)
        return VerifyResult(
            store=self.name,
            app_id=resolved_id,
            ok=True,
            repaired=repair,
            message="Verified via DepotDownloader -validate",
        )

    def update(
        self,
        app_id: str,
        *,
        check_only: bool = False,
        with_dlcs: bool = False,
    ) -> UpdateResult:
        self._require_logged_in_account()
        use_steamcmd = self._use_steamcmd_for_download()
        resolved_id = self._resolve_app_id(app_id, offline=not use_steamcmd, strict=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not installed; use `download` instead of `update`."
            )
        install_path = Path(str(entry["install_path"]))
        platform = str(entry.get("platform") or "windows")
        branch = str(entry.get("branch") or "public")
        old_version = str(entry.get("build_id") or "") or None

        remote_build: str | None = None
        if use_steamcmd:
            remote_appinfo = self._fetch_appinfo_vdf(
                resolved_id,
                platform=platform,
                offline=False,
                allow_cache_fallback=False,
            )
            branches_node = vdf_get(remote_appinfo, "depots", "branches")
            if isinstance(branches_node, dict):
                branch_entry = vdf_get(branches_node, branch)
                if isinstance(branch_entry, dict):
                    bid = str(vdf_get(branch_entry, "buildid", default="") or "").strip()
                    if bid:
                        remote_build = bid
                if remote_build is None and branch == "public":
                    pub_entry = vdf_get(branches_node, "public")
                    if isinstance(pub_entry, dict):
                        bid = str(vdf_get(pub_entry, "buildid", default="") or "").strip()
                        if bid:
                            remote_build = bid

            if remote_build is None:
                if check_only:
                    raise CommandFailedError(
                        f"Could not obtain remote build ID for Steam AppID {resolved_id} "
                        f"(branch {branch!r}) from steamcmd app_info_print."
                    )
            else:
                update_available = remote_build != old_version
                if check_only:
                    return UpdateResult(
                        store=self.name,
                        app_id=resolved_id,
                        updated=False,
                        update_available=update_available,
                        old_version=old_version,
                        new_version=remote_build,
                        install_path=str(install_path),
                        message=(
                            f"Update available ({old_version or 'unknown'} -> {remote_build})"
                            if update_available
                            else "Already up to date"
                        ),
                    )

                if not update_available and old_version is not None:
                    return UpdateResult(
                        store=self.name,
                        app_id=resolved_id,
                        updated=False,
                        update_available=False,
                        old_version=old_version,
                        new_version=remote_build,
                        install_path=str(install_path),
                        message="Already up to date",
                    )
        elif check_only:
            raise UnsupportedOperationError(
                "Remote update checks (--check-only) require `steamcmd` (and Rosetta 2 on Apple Silicon); "
                "run `update` without `--check-only` to update in-place via the DepotDownloader fallback."
            )

        dl_res = self.download(
            DownloadOptions(
                app_id=resolved_id,
                dest=install_path,
                platform=platform,
                with_dlcs=with_dlcs,
                skip_dlcs=not with_dlcs,
                branch=branch,
            )
        )
        return UpdateResult(
            store=self.name,
            app_id=resolved_id,
            updated=True,
            update_available=True,
            old_version=old_version,
            new_version=dl_res.version or remote_build,
            install_path=dl_res.install_path,
            message=(
                f"Updated {resolved_id} on branch '{branch}'"
                if use_steamcmd
                else f"Updated {resolved_id} on branch '{branch}' via DepotDownloader fallback"
            ),
        )

    def uninstall(
        self,
        app_id: str,
        *,
        keep_files: bool = False,
        skip_uninstaller: bool = False,
    ) -> UninstallResult:
        del skip_uninstaller  # SteamCMD has no interactive Windows uninstaller on macOS
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
            # Safety guard: refuse to delete root, home, or top-level system directories.
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

    def launch_args(self, app_id: str, *, offline: bool = True) -> LaunchPlan:
        resolved_id = self._resolve_app_id(app_id, offline=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not installed; cannot generate launch plan."
            )
        install_dir = Path(str(entry["install_path"]))
        platform = str(entry.get("platform") or "windows")
        appinfo = self._fetch_appinfo_vdf(resolved_id, platform=platform, offline=True)
        if not appinfo and not offline and self.find_steamcmd() is not None:
            try:
                appinfo = self._fetch_appinfo_vdf(resolved_id, platform=platform, offline=False)
            except AcquireError:
                appinfo = {}

        exe, raw_args, workdir = self._pick_launch_entry(appinfo, platform=platform)
        if not exe and install_dir.is_dir():
            # Fallback: scan for the primary Windows .exe inside `install_dir`
            exe_candidates: list[Path] = []
            for p in sorted(install_dir.rglob("*.exe")):
                if p.is_symlink() or _SKIP_EXE_PATTERNS.search(p.name):
                    continue
                exe_candidates.append(p)
            if exe_candidates:
                exe_candidates.sort(key=lambda p: (len(p.relative_to(install_dir).parts), p.name.lower()))
                exe = exe_candidates[0].relative_to(install_dir).as_posix()

        safe_args, args_redacted = redact_launch_args(raw_args)
        safe_env, env_redacted = redact_env({"SteamAppId": resolved_id, "SteamGameId": resolved_id})
        title = (
            str(vdf_get(appinfo, "common", "name", default="") or "")
            or str(entry.get("title") or entry.get("name") or f"Steam App {resolved_id}")
        )
        return LaunchPlan(
            store=self.name,
            app_id=resolved_id,
            title=title,
            install_dir=str(install_dir),
            executable=exe or "",
            working_dir=workdir or (Path(exe).parent.as_posix() if exe else "."),
            args=safe_args,
            env=safe_env,
            offline=offline,
            redacted_auth_args=bool(args_redacted or env_redacted),
        )


BACKEND_CLASS = SteamBackend
