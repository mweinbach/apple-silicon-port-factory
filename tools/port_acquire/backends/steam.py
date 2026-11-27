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
    parse_appmanifest_acf,
    parse_depot_downloader_stdout,
    parse_depot_manifest_txt,
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

    def _owned_cache_file(self) -> Path:
        return self.state_dir() / "steam-owned-cache.json"

    def _manifest_cache_file(self, app_id: str) -> Path:
        return self.state_dir() / "manifests" / f"{app_id}.json"

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
        platform: str = "windows",
        branch: str = "public",
        account_name: str | None = None,
        offline: bool = False,
        allow_cache_fallback: bool = True,
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

            depots: list[dict[str, Any]] = []
            seen_ids: set[str] = set()
            for txt_file in sorted(Path(tmp_dir).rglob("manifest_*.txt")):
                parsed = parse_depot_manifest_txt(
                    txt_file.read_text(encoding="utf-8", errors="ignore"),
                    filename=txt_file.name,
                )
                did = parsed.get("depot_id")
                if did:
                    seen_ids.add(did)
                    depots.append(parsed)

            # Merge stdout depot names / any depot lines without a separate .txt file
            stdout_by_id = {d["depot_id"]: d for d in stdout_info.get("depots", []) if d.get("depot_id")}
            for dep in depots:
                sd = stdout_by_id.get(dep["depot_id"])
                if sd and sd.get("name"):
                    dep["name"] = sd["name"]
            for did, sd in stdout_by_id.items():
                if did not in seen_ids:
                    depots.append(
                        {
                            "depot_id": did,
                            "name": sd.get("name", ""),
                            "manifest_gid": sd.get("manifest_gid"),
                            "creation_date": sd.get("date"),
                            "file_count": 0,
                            "disk_size_bytes": None,
                            "download_size_bytes": None,
                            "files": [],
                        }
                    )

            if not depots and proc.returncode != 0:
                if allow_cache_fallback:
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
                "fetched_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            }
            if depots:
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

    def _load_owned_id_cache(self) -> set[str]:
        data = _read_json(self._owned_cache_file(), {})
        raw = data.get("app_ids") if isinstance(data, dict) else None
        if isinstance(raw, list):
            return {str(x) for x in raw if str(x).isdigit()}
        return set()

    def _save_owned_id_cache(self, app_ids: set[str]) -> None:
        _atomic_write_json(
            self._owned_cache_file(),
            {
                "app_ids": sorted(app_ids, key=lambda x: int(x) if x.isdigit() else 0),
                "updated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            },
        )

    def _known_owned_app_ids(self, *, offline: bool = False) -> set[str]:
        """Return known owned AppIDs from Web API, local Steam caches, and managed installs."""
        owned_ids = self._load_owned_id_cache()
        owned_ids |= parse_local_owned_app_ids(self.steam_home())
        owned_ids |= set(self._load_installs().keys())
        owned_ids |= set(self._scan_steamapps_manifests().keys())
        for inst in self._load_installs().values():
            for dlc_id in inst.get("dlcs") or []:
                if str(dlc_id).isdigit():
                    owned_ids.add(str(dlc_id))

        if not offline:
            account_name, steam_id64 = self._resolve_account_info()
            api_key = self._get_steam_web_api_key()
            if account_name and api_key and steam_id64:
                web_games = self._list_owned_via_web_api(api_key, steam_id64)
                if web_games is not None:
                    for g in web_games:
                        owned_ids.add(g.app_id)
                    self._save_owned_id_cache(owned_ids)
        return owned_ids

    def list_owned(self, *, platform: str = "windows", refresh: bool = False) -> list[OwnedGame]:
        del refresh
        account_name, steam_id64 = self._require_logged_in_account()
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

        # 2. Local Steam library caches (`userdata/*/config/{localconfig.vdf,librarycache}`),
        #    local `steamapps/appmanifest_*.acf`, and managed `port-acquire` installs
        local_ids = parse_local_owned_app_ids(self.steam_home())
        local_ids |= set(installs.keys())
        local_ids |= set(acf_map.keys())
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

        # Merge installed state
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

        if games_by_id:
            self._save_owned_id_cache(set(games_by_id.keys()))

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

    def info(self, app_id: str, *, platform: str = "windows", offline: bool = False) -> GameInfo:
        resolved_id = self._resolve_app_id(app_id, offline=offline)
        account_name, _ = self._resolve_account_info()
        installed_entry = self._get_installed_entry(resolved_id)
        branch = str((installed_entry or {}).get("branch") or "public")

        manifest_data = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            branch=branch,
            account_name=account_name,
            offline=offline,
        )
        store_details = {} if offline else self._fetch_store_appdetails(resolved_id)
        owned_app_ids = self._known_owned_app_ids(offline=offline)

        install_path_str = (
            str(installed_entry.get("install_path"))
            if installed_entry and installed_entry.get("install_path")
            else None
        )
        install_dir = Path(install_path_str) if install_path_str else None
        is_installed = bool(install_dir and install_dir.is_dir())

        raw_depots = manifest_data.get("depots") or []
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

        active_branch = str(manifest_data.get("branch") or branch)
        manifest_ver = self._version_from_manifests(raw_depots)
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
        account_name, _ = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(options.app_id, offline=False)
        dest = options.dest.expanduser().resolve()
        dest.mkdir(parents=True, exist_ok=True)
        branch = (options.branch or "public").strip()

        # Pre-fetch manifest metadata (best-effort) so we know depots, owned DLCs, and redists.
        info: GameInfo | None = None
        try:
            info = self.info(resolved_id, platform=options.platform, offline=False)
        except AcquireError:
            info = None

        dd_args: list[str] = [
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
        dd_args.extend(options.extra_args)

        proc = self._run_depot_downloader(dd_args, interactive=False, timeout=None, check=True)
        stdout_info = parse_depot_downloader_stdout(proc.stdout + "\n" + proc.stderr)

        downloaded_depots: list[str] = [
            d["depot_id"] for d in stdout_info.get("depots", []) if d.get("depot_id")
        ]
        if not downloaded_depots and info is not None:
            downloaded_depots = [d.depot_id for d in info.depots if not d.is_common_redist]

        # Owned DLC depots when requested
        downloaded_dlcs: list[str] = []
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
                # Only download DLCs confirmed owned by the account
                target_dlcs = [d.app_id for d in info.dlcs if d.owned]
            else:
                target_dlcs = []

            for dlc_id in target_dlcs:
                dlc_args = [
                    "-app",
                    dlc_id,
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
                if options.validate:
                    dlc_args.append("-validate")
                self._run_depot_downloader(dlc_args, interactive=False, timeout=None, check=True)
                if dlc_id not in downloaded_dlcs:
                    downloaded_dlcs.append(dlc_id)

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
                self._run_depot_downloader(redist_args, interactive=False, timeout=None, check=True)
                if did not in downloaded_redists:
                    downloaded_redists.append(did)

        # Ensure standard `steam_appid.txt` exists for downstream packaging/runtime
        appid_txt = dest / "steam_appid.txt"
        if not appid_txt.exists():
            appid_txt.write_text(f"{resolved_id}\n", encoding="utf-8")

        manifest_gids = {
            d["depot_id"]: d["manifest_gid"]
            for d in stdout_info.get("depots", [])
            if d.get("depot_id") and d.get("manifest_gid")
        }
        version = self._version_from_manifests(stdout_info.get("depots", [])) or (
            info.version if info else None
        )
        title = info.title if info else f"Steam App {resolved_id}"

        if not options.no_register:
            installs = self._load_installs()
            installs[resolved_id] = {
                "app_id": resolved_id,
                "title": title,
                "install_path": str(dest),
                "platform": options.platform,
                "branch": branch,
                "build_id": version,
                "manifest_gids": manifest_gids,
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
            version=version,
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
        proc = self._run_depot_downloader(dd_args, interactive=False, timeout=None, check=False)
        combined = proc.stdout + "\n" + proc.stderr
        ok = proc.returncode == 0
        parsed_out = parse_depot_downloader_stdout(combined)
        repaired_flag = bool(
            repair and ok and (parsed_out.get("total_downloaded_bytes") or 0) > 0
        )
        msg_lines = [ln.strip() for ln in combined.splitlines() if ln.strip()]
        return VerifyResult(
            store=self.name,
            app_id=resolved_id,
            ok=ok,
            repaired=repaired_flag,
            message=msg_lines[-1] if msg_lines else ("Verified OK via DepotDownloader -validate" if ok else "Validation failed"),
        )

    def update(
        self,
        app_id: str,
        *,
        check_only: bool = False,
        with_dlcs: bool = False,
    ) -> UpdateResult:
        account_name, _ = self._require_logged_in_account()
        resolved_id = self._resolve_app_id(app_id, offline=False, strict=True)
        entry = self._get_installed_entry(resolved_id)
        if not entry or not entry.get("install_path"):
            raise AppNotFoundError(
                f"Steam AppID {resolved_id} is not installed; use `download` instead of `update`."
            )
        install_path = Path(str(entry["install_path"]))
        platform = str(entry.get("platform") or "windows")
        branch = str(entry.get("branch") or "public")
        old_version = str(entry.get("build_id") or "") or None

        # Always fetch fresh remote manifests (never falling back to local cache or local build_id).
        remote_manifests = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            branch=branch,
            account_name=account_name,
            offline=False,
            allow_cache_fallback=False,
        )
        remote_version = self._version_from_manifests(remote_manifests.get("depots") or [])

        if remote_version is None:
            if check_only:
                raise CommandFailedError(
                    f"Could not obtain remote manifest ID for Steam AppID {resolved_id} "
                    f"(branch {branch!r}) from DepotDownloader -manifest-only."
                )
        else:
            update_available = remote_version != old_version
            if check_only:
                return UpdateResult(
                    store=self.name,
                    app_id=resolved_id,
                    updated=False,
                    update_available=update_available,
                    old_version=old_version,
                    new_version=remote_version,
                    install_path=str(install_path),
                    message=(
                        f"Update available ({old_version or 'unknown'} -> {remote_version})"
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
                    new_version=remote_version,
                    install_path=str(install_path),
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
        manifest_data = self._fetch_depot_manifests(
            resolved_id,
            platform=platform,
            offline=True,
        )
        manifest_paths: list[str] = []
        for d in manifest_data.get("depots") or []:
            if isinstance(d, dict):
                for f in d.get("files") or []:
                    if isinstance(f, dict) and f.get("path"):
                        manifest_paths.append(str(f["path"]))

        exe = self._pick_executable_from_paths(manifest_paths)
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
