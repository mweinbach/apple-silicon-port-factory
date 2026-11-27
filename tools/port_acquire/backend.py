"""Small, stable store-backend interface for `tools/port-acquire`.

How to add a new store backend (for example, `steam`):
1. Create `tools/port_acquire/backends/<store>.py` (e.g. `backends/steam.py`).
2. Subclass :class:`StoreBackend` and implement the abstract methods.
3. Export `BACKEND_CLASS = YourBackend` (or a zero-arg `create_backend()` factory).

`port_acquire.registry` discovers `tools/port_acquire/backends/<store>.py`
automatically, so parallel store backends do not need to edit shared files.

Hard rules for all backends:
- Never bypass DRM, anti-cheat, or store ownership checks.
- Only download titles owned by the currently authenticated account via official
  or established open-source store CLIs (e.g. `legendary`, `DepotDownloader`).
- Never store, return, or log passwords, OAuth tokens, refresh tokens, cookies,
  or exchange codes. `status()` must report only login state and display name.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
from difflib import SequenceMatcher
from pathlib import Path
import re
from typing import Any


# ---------------------------------------------------------------------------
# Errors
# ---------------------------------------------------------------------------


class AcquireError(RuntimeError):
    """Structured error raised by store backends and CLI helpers."""

    code: str = "acquire_error"
    exit_code: int = 1

    def __init__(
        self,
        message: str,
        *,
        code: str | None = None,
        exit_code: int | None = None,
        details: dict[str, Any] | None = None,
    ) -> None:
        super().__init__(message)
        if code is not None:
            self.code = code
        if exit_code is not None:
            self.exit_code = exit_code
        self.details: dict[str, Any] = details or {}

    def to_dict(self) -> dict[str, Any]:
        payload: dict[str, Any] = {
            "ok": False,
            "error": {
                "code": self.code,
                "message": str(self),
            },
        }
        if self.details:
            payload["error"]["details"] = self.details
        return payload


class BackendUnavailableError(AcquireError):
    code = "backend_unavailable"
    exit_code = 2


class AuthRequiredError(AcquireError):
    code = "auth_required"
    exit_code = 3


class AppNotFoundError(AcquireError):
    code = "app_not_found"
    exit_code = 4


class CommandFailedError(AcquireError):
    code = "command_failed"
    exit_code = 5


class UnsupportedOperationError(AcquireError):
    code = "unsupported_operation"
    exit_code = 6


# ---------------------------------------------------------------------------
# Typed Models (JSON-serializable via `.to_dict()`)
# ---------------------------------------------------------------------------


@dataclass
class AuthStatus:
    """Authentication status.

    Security invariant: never include tokens, cookies, session IDs, account
    secrets, or passwords — only login state, display name, and backend tool info.
    """

    store: str
    logged_in: bool
    display_name: str | None = None
    backend_available: bool = True
    backend_version: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class DlcItem:
    """One DLC or add-on item associated with a base game."""

    app_id: str
    title: str
    owned: bool = True
    installed: bool = False
    install_path: str | None = None
    download_size_bytes: int | None = None
    disk_size_bytes: int | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class RedistributableItem:
    """A runtime prerequisite or SDK component (VC++, DirectX, EOS SDK, etc.)."""

    id: str
    name: str
    category: str  # "vcredist" | "directx" | "eos_sdk" | "dotnet" | "physx" | "other"
    source: str  # "prereq_info" | "manifest_files" | "disk_scan" | "eos_overlay"
    relative_path: str | None = None
    args: str = ""
    native_in_win32_rt: bool = False
    present_on_disk: bool | None = None
    notes: str = ""

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class AnticheatReport:
    """Read-only report of detected anti-cheat signals (never bypassed)."""

    detected: bool = False
    systems: list[str] = field(default_factory=list)
    signals: list[str] = field(default_factory=list)
    report_only: bool = True

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class DrmReport:
    """Read-only report of DRM / online-auth / third-party launcher signals."""

    detected: bool = False
    requires_online_auth: bool = False
    requires_ownership_token: bool = False
    third_party_launcher: str | None = None
    signals: list[str] = field(default_factory=list)
    report_only: bool = True

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class OwnedGame:
    """One title in the authenticated user's owned library."""

    store: str
    app_id: str
    title: str
    platforms: list[str] = field(default_factory=list)
    version: str | None = None
    installed: bool = False
    install_path: str | None = None
    dlcs: list[DlcItem] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class SearchMatch:
    """One ranked result from fuzzy title search over the owned library."""

    store: str
    app_id: str
    title: str
    score: float
    installed: bool = False
    platforms: list[str] = field(default_factory=list)
    dlcs_count: int = 0

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class DepotItem:
    """One store content depot (e.g. Steam Windows x64, DLC, or shared redist depot)."""

    depot_id: str
    name: str = ""
    oslist: list[str] = field(default_factory=list)
    osarch: str | None = None
    language: str | None = None
    manifest_gid: str | None = None
    size_bytes: int | None = None
    download_bytes: int | None = None
    dlc_app_id: str | None = None
    shared_install: bool = False
    depot_from_app: str | None = None
    is_common_redist: bool = False

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class GameInfo:
    """Detailed pre-download or post-download inspection for one owned title."""

    store: str
    app_id: str
    title: str
    platform: str = "windows"
    version: str | None = None
    download_size_bytes: int | None = None
    disk_size_bytes: int | None = None
    installed: bool = False
    install_path: str | None = None
    executable: str | None = None
    can_run_offline: bool | None = None
    requires_eos: bool = False
    eos_signals: list[str] = field(default_factory=list)
    requires_steamworks: bool = False
    steamworks_signals: list[str] = field(default_factory=list)
    anticheat: AnticheatReport = field(default_factory=AnticheatReport)
    drm: DrmReport = field(default_factory=DrmReport)
    depots: list[DepotItem] = field(default_factory=list)
    branches: list[dict[str, Any]] = field(default_factory=list)
    dlcs: list[DlcItem] = field(default_factory=list)
    redistributables: list[RedistributableItem] = field(default_factory=list)
    proton_redist_info: dict[str, Any] = field(default_factory=dict)
    manifest_files_checked: bool = False

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class DownloadOptions:
    """Options passed to :meth:`StoreBackend.download`."""

    app_id: str
    dest: Path
    platform: str = "windows"
    with_dlcs: bool = False
    skip_dlcs: bool = True
    dlcs: tuple[str, ...] = ()
    with_eos_overlay: bool = False
    with_redists: bool = False
    branch: str | None = None
    branch_password: str | None = None
    validate: bool = False
    no_register: bool = False
    max_workers: int | None = None
    extra_args: tuple[str, ...] = ()


@dataclass
class DownloadResult:
    """Result returned by :meth:`StoreBackend.download`."""

    store: str
    app_id: str
    title: str
    platform: str
    install_path: str
    version: str | None = None
    branch: str | None = None
    downloaded_dlcs: list[str] = field(default_factory=list)
    downloaded_depots: list[str] = field(default_factory=list)
    downloaded_redists: list[str] = field(default_factory=list)
    eos_overlay_path: str | None = None
    manifest_path: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class VerifyResult:
    """Result returned by :meth:`StoreBackend.verify`."""

    store: str
    app_id: str
    ok: bool
    repaired: bool = False
    missing_files: list[str] = field(default_factory=list)
    corrupted_files: list[str] = field(default_factory=list)
    message: str = ""

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class UpdateResult:
    """Result returned by :meth:`StoreBackend.update`."""

    store: str
    app_id: str
    updated: bool
    update_available: bool = False
    old_version: str | None = None
    new_version: str | None = None
    install_path: str | None = None
    message: str = ""

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class UninstallResult:
    """Result returned by :meth:`StoreBackend.uninstall`."""

    store: str
    app_id: str
    uninstalled: bool
    kept_files: bool = False
    install_path: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class LaunchPlan:
    """Sanitized launch configuration for `port-runner` / `game.toml` / `.app` packaging.

    Security invariant: auth tokens, passwords, and exchange codes must always be
    stripped or redacted before returning a LaunchPlan.
    """

    store: str
    app_id: str
    title: str
    install_dir: str
    executable: str
    working_dir: str
    args: list[str] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    offline: bool = True
    redacted_auth_args: bool = False

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


# ---------------------------------------------------------------------------
# Shared Fuzzy Search & Redistributable Classification Helpers
# ---------------------------------------------------------------------------

_ROMAN_TO_ARABIC = {
    "ii": "2",
    "iii": "3",
    "iv": "4",
    "v": "5",
    "vi": "6",
    "vii": "7",
    "viii": "8",
    "ix": "9",
    "x": "10",
}

_TRADEMARK_RE = re.compile(r"[™®©]")
_NON_ALNUM_RE = re.compile(r"[^a-z0-9]+")


def normalize_title(text: str) -> str:
    """Normalize a game title for fuzzy matching (strips punctuation, normalizes numerals)."""
    cleaned = _TRADEMARK_RE.sub("", text.lower())
    # Collapse apostrophes inside words ("marvel's" -> "marvels") before splitting
    cleaned = cleaned.replace("'", "").replace("\u2019", "")
    tokens = [t for t in _NON_ALNUM_RE.split(cleaned) if t]
    normalized = [_ROMAN_TO_ARABIC.get(tok, tok) for tok in tokens]
    return " ".join(normalized)


def score_title_match(query: str, title: str, app_id: str = "") -> float:
    """Return a fuzzy relevance score in `[0.0, 1.0]` for `query` against `title`/`app_id`."""
    q_raw = query.strip().lower()
    if not q_raw:
        return 0.0
    if app_id and q_raw == app_id.strip().lower():
        return 1.0

    nq = normalize_title(query)
    nt = normalize_title(title)
    if not nq or not nt:
        return 0.0
    if nq == nt:
        return 1.0

    q_tokens = nq.split()
    t_tokens = nt.split()
    q_set = set(q_tokens)
    t_set = set(t_tokens)

    # Exact substring bonus (e.g. "spider man 2" in "marvels spider man 2")
    substring_score = 0.0
    if nq in nt:
        length_ratio = len(nq) / max(len(nt), 1)
        substring_score = 0.82 + 0.16 * length_ratio
    elif nt in nq:
        length_ratio = len(nt) / max(len(nq), 1)
        substring_score = 0.72 + 0.16 * length_ratio

    # Token coverage: how many query tokens match a title token (exact or prefix)
    matched_tokens = 0.0
    for qt in q_tokens:
        if qt in t_set:
            matched_tokens += 1.0
        elif any(tt.startswith(qt) or qt.startswith(tt) for tt in t_tokens if len(qt) >= 3):
            matched_tokens += 0.75
        else:
            best_tok = max((SequenceMatcher(None, qt, tt).ratio() for tt in t_tokens), default=0.0)
            if best_tok >= 0.78:
                matched_tokens += best_tok * 0.8
    query_coverage = matched_tokens / len(q_tokens)
    jaccard = len(q_set & t_set) / max(len(q_set | t_set), 1)
    seq_ratio = SequenceMatcher(None, nq, nt).ratio()
    compact_ratio = SequenceMatcher(None, nq.replace(" ", ""), nt.replace(" ", "")).ratio()

    token_blend = 0.55 * query_coverage + 0.20 * jaccard + 0.25 * max(seq_ratio, compact_ratio)
    # Penalize queries that specify a trailing numeral not present in the title
    # (e.g. "spider man 2" should strongly prefer "Spider-Man 2" over "Spider-Man Remastered").
    q_nums = {tok for tok in q_tokens if tok.isdigit()}
    t_nums = {tok for tok in t_tokens if tok.isdigit()}
    if q_nums and not (q_nums <= t_nums):
        token_blend *= 0.65
        substring_score *= 0.65

    return round(min(1.0, max(substring_score, token_blend)), 4)


def fuzzy_search_owned(
    games: list[OwnedGame],
    query: str,
    *,
    limit: int = 10,
    min_score: float = 0.35,
) -> list[SearchMatch]:
    """Rank owned games against `query` using :func:`score_title_match`."""
    matches: list[SearchMatch] = []
    for game in games:
        score = score_title_match(query, game.title, game.app_id)
        if score >= min_score:
            matches.append(
                SearchMatch(
                    store=game.store,
                    app_id=game.app_id,
                    title=game.title,
                    score=score,
                    installed=game.installed,
                    platforms=list(game.platforms),
                    dlcs_count=len(game.dlcs),
                )
            )
    matches.sort(key=lambda m: (-m.score, m.title.lower(), m.app_id.lower()))
    return matches[: max(1, limit)]


def sanitize_dirname(title: str) -> str:
    """Convert a game title into a safe directory name (matches `port-factory`)."""
    cleaned = "".join(
        ch for ch in title if ch.isascii() and (ch.isalnum() or ch in (" ", "-", "_"))
    ).strip()
    cleaned = re.sub(r"\s+", " ", cleaned)
    return cleaned or "AcquiredGame"


def classify_redistributable_path(
    rel_path: str,
    *,
    source: str = "manifest_files",
    present_on_disk: bool | None = None,
) -> RedistributableItem | None:
    """Identify VC++, DirectX, EOS SDK, .NET, or PhysX components from a relative file path."""
    norm = rel_path.replace("\\", "/")
    lower = norm.lower()
    fname = Path(norm).name
    fname_lower = fname.lower()

    if ("vc_redist" in fname_lower or "vcredist" in fname_lower) and fname_lower.endswith(".exe"):
        return RedistributableItem(
            id=f"vcredist:{fname_lower}",
            name=f"Visual C++ Redistributable ({fname})",
            category="vcredist",
            source=source,
            relative_path=norm,
            native_in_win32_rt=True,
            present_on_disk=present_on_disk,
            notes="Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
        )
    if (
        fname_lower in ("dxsetup.exe", "dxwebsetup.exe")
        or ("directx" in lower and fname_lower.endswith(".exe") and "setup" in fname_lower)
    ):
        return RedistributableItem(
            id=f"directx:{fname_lower}",
            name=f"DirectX End-User Runtime ({fname})",
            category="directx",
            source=source,
            relative_path=norm,
            native_in_win32_rt=True,
            present_on_disk=present_on_disk,
            notes="Satisfied natively by crates/win32-rt D3D11/D3D12 + crates/d3d12-metal",
        )
    if (
        "eossdk" in fname_lower
        or "eosoverlay" in fname_lower
        or "epic_online_services" in lower
        or ("eos" in fname_lower and fname_lower.endswith((".dll", ".exe")) and "redist" in lower)
    ):
        return RedistributableItem(
            id=f"eos_sdk:{fname_lower}",
            name=f"Epic Online Services SDK ({fname})",
            category="eos_sdk",
            source=source,
            relative_path=norm,
            native_in_win32_rt=False,
            present_on_disk=present_on_disk,
            notes="Shipped EOS component; keep with game payload and configure [middleware.eos] if needed",
        )
    if ("dotnet" in fname_lower or "ndp4" in fname_lower) and fname_lower.endswith(".exe"):
        return RedistributableItem(
            id=f"dotnet:{fname_lower}",
            name=f".NET Runtime Redistributable ({fname})",
            category="dotnet",
            source=source,
            relative_path=norm,
            native_in_win32_rt=False,
            present_on_disk=present_on_disk,
            notes="Inspect with `cargo opsdb -- dotnet-audit`",
        )
    if "physx" in fname_lower and fname_lower.endswith((".exe", ".msi")):
        return RedistributableItem(
            id=f"physx:{fname_lower}",
            name=f"NVIDIA PhysX System Software ({fname})",
            category="physx",
            source=source,
            relative_path=norm,
            native_in_win32_rt=False,
            present_on_disk=present_on_disk,
            notes="Shipped PhysX DLLs typically run via ARM64 JIT; configure [middleware.physx] if needed",
        )
    return None


def scan_dir_redistributables(install_dir: Path, *, max_depth: int = 4) -> list[RedistributableItem]:
    """Scan a downloaded game directory (without following symlinks) for redistributable files."""
    if not install_dir.is_dir():
        return []
    found: list[RedistributableItem] = []
    seen: set[str] = set()
    stack: list[tuple[Path, int]] = [(install_dir, 0)]
    while stack:
        cur, depth = stack.pop()
        if depth > max_depth:
            continue
        try:
            entries = sorted(cur.iterdir(), key=lambda p: p.name.lower())
        except OSError:
            continue
        for entry in entries:
            if entry.is_symlink():
                continue
            if entry.is_dir():
                if depth < max_depth and not entry.name.startswith("."):
                    stack.append((entry, depth + 1))
                continue
            if not entry.is_file():
                continue
            rel = entry.relative_to(install_dir).as_posix()
            item = classify_redistributable_path(rel, source="disk_scan", present_on_disk=True)
            if item is not None and item.relative_path not in seen:
                seen.add(item.relative_path or item.id)
                found.append(item)
    return found


# ---------------------------------------------------------------------------
# Pluggable Store Backend ABC
# ---------------------------------------------------------------------------


class StoreBackend(ABC):
    """Abstract interface implemented by each store backend (`epic`, `steam`, ...)."""

    name: str = ""
    display_name: str = ""

    @abstractmethod
    def login(
        self,
        *,
        no_webview: bool = False,
        auth_code: str | None = None,
        import_existing: bool = False,
        logout: bool = False,
    ) -> AuthStatus:
        """Run the interactive official authentication flow (never store/log credentials)."""

    @abstractmethod
    def status(self, *, offline: bool = False) -> AuthStatus:
        """Return login state and account display name only (never tokens or secrets)."""

    @abstractmethod
    def list_owned(self, *, platform: str = "windows", refresh: bool = False) -> list[OwnedGame]:
        """List games owned by the authenticated account."""

    def search(
        self,
        query: str,
        *,
        platform: str = "windows",
        limit: int = 10,
        min_score: float = 0.35,
        refresh: bool = False,
    ) -> list[SearchMatch]:
        """Fuzzy-search owned titles and map a human title to an `app_id`."""
        owned = self.list_owned(platform=platform, refresh=refresh)
        return fuzzy_search_owned(owned, query, limit=limit, min_score=min_score)

    @abstractmethod
    def info(self, app_id: str, *, platform: str = "windows", offline: bool = False) -> GameInfo:
        """Inspect size, EOS/anti-cheat/DRM signals (report-only), DLCs, and redistributables."""

    def list_dlcs(
        self, app_id: str, *, platform: str = "windows", offline: bool = False
    ) -> list[DlcItem]:
        """List DLCs / add-ons for `app_id`."""
        return self.info(app_id, platform=platform, offline=offline).dlcs

    def list_redists(
        self, app_id: str, *, platform: str = "windows", offline: bool = False
    ) -> list[RedistributableItem]:
        """List prerequisites, redistributables, and SDK bits for `app_id`."""
        return self.info(app_id, platform=platform, offline=offline).redistributables

    @abstractmethod
    def download(self, options: DownloadOptions) -> DownloadResult:
        """Download an owned title (and optional DLCs/redistributable components) to `options.dest`."""

    @abstractmethod
    def verify(self, app_id: str, *, repair: bool = False) -> VerifyResult:
        """Verify local files against the store manifest (and optionally repair)."""

    @abstractmethod
    def update(
        self, app_id: str, *, check_only: bool = False, with_dlcs: bool = False
    ) -> UpdateResult:
        """Check for or apply an update to an installed title."""

    @abstractmethod
    def uninstall(
        self, app_id: str, *, keep_files: bool = False, skip_uninstaller: bool = False
    ) -> UninstallResult:
        """Uninstall a title managed by this store backend."""

    @abstractmethod
    def launch_args(self, app_id: str, *, offline: bool = True) -> LaunchPlan:
        """Return sanitized executable, working directory, and launch arguments."""
